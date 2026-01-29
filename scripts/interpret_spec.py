#!/usr/bin/env python3
"""
Agent C — OpenAPI Interpreter Agent

Parses the raw Torn OpenAPI spec and produces structured intermediate
representations that downstream agents consume.

Outputs:
  - spec_map.json — comprehensive tag→endpoint mapping
  - pagination_map.json — paginated endpoint metadata
  - schema_map.json — schema name→definition mapping for Agent D
"""

import json
import os
from typing import Any, Dict, List, Optional, Set


def resolve_ref(spec: dict, ref: str) -> Any:
    """
    Resolve a $ref pointer in the spec.

    Args:
        spec: The full OpenAPI spec
        ref: A JSON pointer like "#/components/schemas/FactionId"

    Returns:
        The resolved schema, or None if not found
    """
    if not ref.startswith("#/"):
        return None

    parts = ref[2:].split("/")
    current = spec

    for part in parts:
        if isinstance(current, dict):
            current = current.get(part)
            if current is None:
                return None
        else:
            return None

    return current


def resolve_parameter(spec: dict, param: dict) -> dict:
    """
    Resolve a parameter, handling $ref if present.

    Args:
        spec: The full OpenAPI spec
        param: Parameter object (may contain $ref)

    Returns:
        Resolved parameter dict
    """
    if "$ref" in param:
        resolved = resolve_ref(spec, param["$ref"])
        if resolved:
            return resolved
    return param


def extract_response_schema_ref(spec: dict, operation: dict) -> Optional[str]:
    """
    Extract the response schema $ref from a 200 response.

    Handles:
      - Single $ref
      - anyOf/oneOf with multiple refs (returns first one)
      - allOf compositions (returns first one)

    Args:
        spec: The full OpenAPI spec
        operation: Operation object from the spec

    Returns:
        Schema reference string, or None if not found
    """
    responses = operation.get("responses", {})
    resp_200 = responses.get("200", {})
    content = resp_200.get("content", {})
    json_content = content.get("application/json", {})
    schema = json_content.get("schema", {})

    # Direct $ref
    if "$ref" in schema:
        return schema["$ref"]

    # anyOf/oneOf - return first ref
    for key in ["anyOf", "oneOf"]:
        if key in schema and isinstance(schema[key], list) and len(schema[key]) > 0:
            first_item = schema[key][0]
            if "$ref" in first_item:
                return first_item["$ref"]

    # allOf - return first ref
    if "allOf" in schema and isinstance(schema["allOf"], list) and len(schema["allOf"]) > 0:
        first_item = schema["allOf"][0]
        if "$ref" in first_item:
            return first_item["$ref"]

    return None


def check_pagination_in_schema(spec: dict, schema_ref: Optional[str]) -> bool:
    """
    Check if a response schema contains pagination metadata (_metadata.links).

    Args:
        spec: The full OpenAPI spec
        schema_ref: Schema reference like "#/components/schemas/AttacksResponse"

    Returns:
        True if schema has _metadata with links structure
    """
    if not schema_ref:
        return False

    schema = resolve_ref(spec, schema_ref)
    if not schema or not isinstance(schema, dict):
        return False

    properties = schema.get("properties", {})
    if "_metadata" not in properties:
        return False

    metadata = properties["_metadata"]

    # Handle direct schema or $ref
    if "$ref" in metadata:
        metadata = resolve_ref(spec, metadata["$ref"])
        if not metadata:
            return False

    # Check for links property
    metadata_props = metadata.get("properties", {})
    if "links" not in metadata_props:
        return False

    links = metadata_props["links"]
    if "$ref" in links:
        links = resolve_ref(spec, links["$ref"])
        if not links:
            return False

    # Check for next/prev in links
    links_props = links.get("properties", {})
    return "next" in links_props and "prev" in links_props


def build_spec_map(spec: dict) -> dict:
    """
    Build comprehensive tag→endpoint mapping.

    Returns:
        {
          "tag_name": [
            {
              "path": "/user/basic",
              "method": "GET",
              "operationId": "getUserBasic",
              "summary": "Get user basic info",
              "parameters": [...],
              "responseSchemaRef": "#/components/schemas/UserBasicResponse",
              "pathParams": [],
              "requiresId": false
            },
            ...
          ]
        }
    """
    paths = spec.get("paths", {})
    tag_map: Dict[str, List[Dict]] = {}

    for path, methods in sorted(paths.items()):
        for method, operation in sorted(methods.items()):
            # Skip non-operation keys
            if method in ("parameters", "servers", "summary", "description"):
                continue

            # Extract basic metadata
            tags = operation.get("tags", ["untagged"])
            op_id = operation.get("operationId")
            if not op_id:
                # Generate operationId from method + path
                op_id = f"{method}_{path}".replace("/", "_").replace("{", "").replace("}", "")

            summary = operation.get("summary", "")

            # Extract and resolve parameters
            raw_params = operation.get("parameters", [])
            parameters = []
            path_params = []

            for param in raw_params:
                resolved = resolve_parameter(spec, param)
                param_name = resolved.get("name", "")
                param_in = resolved.get("in", "")

                parameters.append({
                    "name": param_name,
                    "in": param_in,
                    "required": resolved.get("required", False),
                    "schema": resolved.get("schema", {}),
                    "description": resolved.get("description", "")
                })

                if param_in == "path":
                    path_params.append(param_name)

            # Extract response schema ref
            response_schema_ref = extract_response_schema_ref(spec, operation)

            # Determine if endpoint requires ID
            requires_id = "{id}" in path or any(p == "id" for p in path_params)

            # Build entry
            entry = {
                "path": path,
                "method": method.upper(),
                "operationId": op_id,
                "summary": summary,
                "parameters": parameters,
                "responseSchemaRef": response_schema_ref,
                "pathParams": path_params,
                "requiresId": requires_id
            }

            # Add to all tags
            for tag in tags:
                tag_lower = tag.lower().strip()
                tag_map.setdefault(tag_lower, []).append(entry)

    return tag_map


def build_pagination_map(spec: dict, spec_map: dict) -> dict:
    """
    Build mapping of paginated endpoints.

    Detection strategy:
      1. Check response schema for _metadata.links.next/prev (primary)
      2. Check for pagination parameters (limit, offset, sort, from, to)

    Returns:
        {
          "operationId": {
            "path": "/market",
            "method": "GET",
            "paginationStyle": "metadata_links" | "offset_limit",
            "params": [...]
          }
        }
    """
    pagination_param_names = {"limit", "offset", "from", "to", "sort", "timestamp"}
    pagination_map: Dict[str, Dict] = {}

    # Flatten spec_map to get all endpoints
    all_endpoints = []
    for endpoints in spec_map.values():
        all_endpoints.extend(endpoints)

    for endpoint in all_endpoints:
        op_id = endpoint["operationId"]
        path = endpoint["path"]
        method = endpoint["method"]
        parameters = endpoint["parameters"]
        response_schema_ref = endpoint.get("responseSchemaRef")

        # Check for pagination metadata in response
        has_metadata_links = check_pagination_in_schema(spec, response_schema_ref)

        # Check for pagination parameters
        pag_params = []
        for param in parameters:
            param_name = param.get("name", "")
            if param_name.lower() in pagination_param_names:
                pag_params.append({
                    "name": param_name,
                    "in": param.get("in", "query"),
                    "required": param.get("required", False),
                    "schema": param.get("schema", {})
                })

        # Determine pagination style
        pagination_style = None
        if has_metadata_links:
            pagination_style = "metadata_links"
        elif pag_params:
            # Heuristic: if has limit/offset, it's offset_limit style
            param_names_lower = {p["name"].lower() for p in pag_params}
            if "limit" in param_names_lower or "offset" in param_names_lower:
                pagination_style = "offset_limit"
            else:
                # Has other pagination params (from/to/sort), still consider paginated
                pagination_style = "metadata_links"

        if pagination_style:
            pagination_map[op_id] = {
                "path": path,
                "method": method,
                "paginationStyle": pagination_style,
                "params": pag_params
            }

    return pagination_map


def resolve_schema_recursive(
    spec: dict,
    schema: dict,
    visited: Optional[Set[str]] = None
) -> dict:
    """
    Resolve a schema recursively, handling $ref pointers.

    Avoids infinite loops by tracking visited refs.

    Args:
        spec: The full OpenAPI spec
        schema: Schema object to resolve
        visited: Set of already visited $ref strings

    Returns:
        Resolved schema dict
    """
    if visited is None:
        visited = set()

    if not isinstance(schema, dict):
        return schema

    # Handle $ref
    if "$ref" in schema:
        ref_str = schema["$ref"]

        # Avoid circular references
        if ref_str in visited:
            return {"$ref": ref_str, "_circular": True}

        visited.add(ref_str)
        resolved = resolve_ref(spec, ref_str)

        if resolved:
            return resolve_schema_recursive(spec, resolved, visited)
        else:
            return schema

    # Recursively resolve nested schemas
    result = {}
    for key, value in schema.items():
        if isinstance(value, dict):
            result[key] = resolve_schema_recursive(spec, value, visited.copy())
        elif isinstance(value, list):
            result[key] = [
                resolve_schema_recursive(spec, item, visited.copy())
                if isinstance(item, dict) else item
                for item in value
            ]
        else:
            result[key] = value

    return result


def build_schema_map(spec: dict) -> dict:
    """
    Build schema name→definition mapping.

    Extracts all schemas from components.schemas with:
      - Resolved $ref references (top-level only, to avoid infinite loops)
      - Enum definitions
      - allOf/oneOf/anyOf compositions noted explicitly

    Returns:
        {
          "UserBasicResponse": {
            "type": "object",
            "properties": {...},
            "required": [...]
          },
          ...
        }
    """
    schemas = spec.get("components", {}).get("schemas", {})
    schema_map: Dict[str, Dict] = {}

    for schema_name, schema_def in sorted(schemas.items()):
        # We don't recursively resolve all $refs to avoid infinite loops
        # and excessive expansion. We just note compositions.

        resolved = schema_def.copy()

        # If top-level is a $ref, resolve one level
        if "$ref" in resolved:
            ref_resolved = resolve_ref(spec, resolved["$ref"])
            if ref_resolved:
                resolved = ref_resolved.copy()

        # Annotate compositions
        if "allOf" in resolved:
            resolved["_composition"] = "allOf"
        elif "oneOf" in resolved:
            resolved["_composition"] = "oneOf"
        elif "anyOf" in resolved:
            resolved["_composition"] = "anyOf"

        schema_map[schema_name] = resolved

    return schema_map


def write_json(data: dict, path: str, label: str) -> None:
    """Write JSON with deterministic formatting."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(data, f, indent=2, sort_keys=True, ensure_ascii=False)
        f.write("\n")
    print(f"  Wrote {label}: {path}")


def interpret_spec(spec: dict, output_dir: str) -> None:
    """
    Main interpreter function.

    Args:
        spec: Loaded OpenAPI spec dict
        output_dir: Directory to write output files (e.g., "openapi/")
    """
    # Build spec_map
    print("  Building spec_map.json...")
    spec_map = build_spec_map(spec)
    spec_map_path = os.path.join(output_dir, "spec_map.json")
    write_json(spec_map, spec_map_path, "spec_map")

    tag_count = len(spec_map)
    endpoint_count = sum(len(v) for v in spec_map.values())
    print(f"    Tags: {tag_count}, Endpoints: {endpoint_count}")

    # Build pagination_map
    print("  Building pagination_map.json...")
    pagination_map = build_pagination_map(spec, spec_map)
    pagination_map_path = os.path.join(output_dir, "pagination_map.json")
    write_json(pagination_map, pagination_map_path, "pagination_map")
    print(f"    Paginated endpoints: {len(pagination_map)}")

    # Build schema_map
    print("  Building schema_map.json...")
    schema_map = build_schema_map(spec)
    schema_map_path = os.path.join(output_dir, "schema_map.json")
    write_json(schema_map, schema_map_path, "schema_map")
    print(f"    Schemas: {len(schema_map)}")


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: interpret_spec.py <spec.json>")
        sys.exit(1)

    spec_path = sys.argv[1]
    with open(spec_path, "r", encoding="utf-8") as f:
        spec = json.load(f)

    output_dir = os.path.dirname(spec_path)
    interpret_spec(spec, output_dir)
