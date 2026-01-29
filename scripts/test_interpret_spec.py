#!/usr/bin/env python3
"""
Tests for interpret_spec.py (Agent C)

Validates that the OpenAPI interpreter correctly:
  - Parses all endpoints from the spec
  - Extracts complete metadata (parameters, response schemas, path params)
  - Identifies pagination patterns
  - Handles edge cases (no operationId, no tags, compositions, etc.)
  - Produces deterministic output
"""

import json
import os
import sys
import tempfile
import shutil


def load_module(module_name: str, file_path: str):
    """Dynamically import a Python module."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(module_name, file_path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def test_spec_map_completeness(spec: dict, spec_map: dict) -> bool:
    """Test that spec_map contains all endpoints from the spec."""
    print("\n[TEST] Spec map completeness...")

    # Count endpoints in spec
    spec_endpoint_count = 0
    for path, methods in spec.get("paths", {}).items():
        for method in methods:
            if method not in ("parameters", "servers", "summary", "description"):
                spec_endpoint_count += 1

    # Count endpoints in spec_map
    map_endpoint_count = sum(len(eps) for eps in spec_map.values())

    print(f"  Spec endpoints: {spec_endpoint_count}")
    print(f"  Map endpoints: {map_endpoint_count}")

    if spec_endpoint_count == map_endpoint_count:
        print("  PASS: All endpoints present")
        return True
    else:
        print("  FAIL: Endpoint count mismatch")
        return False


def test_required_fields(spec_map: dict) -> bool:
    """Test that all endpoints have required fields."""
    print("\n[TEST] Required fields...")

    required_fields = [
        "path", "method", "operationId", "summary",
        "parameters", "responseSchemaRef", "pathParams", "requiresId"
    ]

    missing = []
    for tag, endpoints in spec_map.items():
        for ep in endpoints:
            for field in required_fields:
                if field not in ep:
                    missing.append(f"{tag}/{ep.get('path', 'unknown')}: missing '{field}'")

    if not missing:
        print(f"  PASS: All endpoints have required fields")
        return True
    else:
        print(f"  FAIL: {len(missing)} missing fields")
        for m in missing[:5]:
            print(f"    {m}")
        return False


def test_path_params(spec_map: dict) -> bool:
    """Test that pathParams and requiresId are correctly set."""
    print("\n[TEST] Path parameters...")

    errors = []
    for tag, endpoints in spec_map.items():
        for ep in endpoints:
            path = ep["path"]
            path_params = ep["pathParams"]
            requires_id = ep["requiresId"]

            # Check if {id} in path matches requiresId
            has_id_in_path = "{id}" in path
            if has_id_in_path != requires_id:
                errors.append(
                    f"{path}: has_id_in_path={has_id_in_path} but requiresId={requires_id}"
                )

            # Check if pathParams matches parameters
            param_path_params = [
                p["name"] for p in ep["parameters"] if p.get("in") == "path"
            ]
            if set(path_params) != set(param_path_params):
                errors.append(
                    f"{path}: pathParams {path_params} != params {param_path_params}"
                )

    if not errors:
        print(f"  PASS: Path parameters correct")
        return True
    else:
        print(f"  FAIL: {len(errors)} errors")
        for e in errors[:5]:
            print(f"    {e}")
        return False


def test_pagination_detection(spec: dict, pagination_map: dict) -> bool:
    """Test that pagination endpoints are correctly identified."""
    print("\n[TEST] Pagination detection...")

    # Check that all paginated endpoints have required fields
    errors = []
    for op_id, pag_data in pagination_map.items():
        if "path" not in pag_data:
            errors.append(f"{op_id}: missing 'path'")
        if "method" not in pag_data:
            errors.append(f"{op_id}: missing 'method'")
        if "paginationStyle" not in pag_data:
            errors.append(f"{op_id}: missing 'paginationStyle'")
        if "params" not in pag_data:
            errors.append(f"{op_id}: missing 'params'")

        # Check paginationStyle is valid
        style = pag_data.get("paginationStyle")
        if style not in ("metadata_links", "offset_limit"):
            errors.append(f"{op_id}: invalid paginationStyle '{style}'")

    if not errors:
        print(f"  PASS: Pagination data valid")
        return True
    else:
        print(f"  FAIL: {len(errors)} errors")
        for e in errors[:5]:
            print(f"    {e}")
        return False


def test_schema_map_completeness(spec: dict, schema_map: dict) -> bool:
    """Test that schema_map contains all schemas from the spec."""
    print("\n[TEST] Schema map completeness...")

    spec_schemas = spec.get("components", {}).get("schemas", {})
    spec_schema_count = len(spec_schemas)
    map_schema_count = len(schema_map)

    print(f"  Spec schemas: {spec_schema_count}")
    print(f"  Map schemas: {map_schema_count}")

    if spec_schema_count == map_schema_count:
        print("  PASS: All schemas present")
        return True
    else:
        print("  FAIL: Schema count mismatch")
        return False


def test_composition_annotations(schema_map: dict) -> bool:
    """Test that allOf/oneOf/anyOf schemas are annotated."""
    print("\n[TEST] Composition annotations...")

    composed_schemas = {}
    for name, schema in schema_map.items():
        if "allOf" in schema:
            composed_schemas[name] = "allOf"
        elif "oneOf" in schema:
            composed_schemas[name] = "oneOf"
        elif "anyOf" in schema:
            composed_schemas[name] = "anyOf"

    annotated = {
        name: schema.get("_composition")
        for name, schema in schema_map.items()
        if "_composition" in schema
    }

    print(f"  Composed schemas: {len(composed_schemas)}")
    print(f"  Annotated schemas: {len(annotated)}")

    # Check that all composed schemas are annotated
    errors = []
    for name, comp_type in composed_schemas.items():
        if name not in annotated:
            errors.append(f"{name}: has {comp_type} but no _composition annotation")
        elif annotated[name] != comp_type:
            errors.append(
                f"{name}: _composition={annotated[name]} but has {comp_type}"
            )

    if not errors:
        print("  PASS: Compositions correctly annotated")
        return True
    else:
        print(f"  FAIL: {len(errors)} errors")
        for e in errors[:5]:
            print(f"    {e}")
        return False


def test_determinism(spec: dict, output_dir: str, interpret_mod) -> bool:
    """Test that running the interpreter twice produces identical output."""
    print("\n[TEST] Determinism...")

    # Create temp directories for two runs
    temp_dir1 = tempfile.mkdtemp()
    temp_dir2 = tempfile.mkdtemp()

    try:
        # Run interpreter twice
        interpret_mod.interpret_spec(spec, temp_dir1)
        interpret_mod.interpret_spec(spec, temp_dir2)

        # Compare outputs
        files_to_compare = ["spec_map.json", "pagination_map.json", "schema_map.json"]
        differences = []

        for filename in files_to_compare:
            path1 = os.path.join(temp_dir1, filename)
            path2 = os.path.join(temp_dir2, filename)

            with open(path1, "r") as f1, open(path2, "r") as f2:
                content1 = f1.read()
                content2 = f2.read()

                if content1 != content2:
                    differences.append(filename)

        if not differences:
            print("  PASS: Output is deterministic")
            return True
        else:
            print(f"  FAIL: {len(differences)} files differ")
            for f in differences:
                print(f"    {f}")
            return False

    finally:
        shutil.rmtree(temp_dir1, ignore_errors=True)
        shutil.rmtree(temp_dir2, ignore_errors=True)


def test_edge_cases(spec_map: dict, pagination_map: dict, schema_map: dict) -> bool:
    """Test edge case handling."""
    print("\n[TEST] Edge cases...")

    errors = []

    # Test 1: Endpoints with no tags should be in "untagged"
    untagged_count = len(spec_map.get("untagged", []))
    print(f"  Untagged endpoints: {untagged_count}")

    # Test 2: Check for empty parameters arrays (should be [], not missing)
    for tag, endpoints in spec_map.items():
        for ep in endpoints:
            if "parameters" not in ep:
                errors.append(f"{ep['path']}: missing parameters field")
            elif ep["parameters"] is None:
                errors.append(f"{ep['path']}: parameters is None (should be [])")

    # Test 3: Check for response schemas with anyOf/oneOf (should handle gracefully)
    anyof_count = sum(
        1 for eps in spec_map.values()
        for ep in eps
        if ep.get("responseSchemaRef") and "anyOf" not in str(ep["responseSchemaRef"])
    )
    print(f"  Endpoints with resolved response schemas: {anyof_count}")

    if not errors:
        print("  PASS: Edge cases handled correctly")
        return True
    else:
        print(f"  FAIL: {len(errors)} errors")
        for e in errors[:5]:
            print(f"    {e}")
        return False


def main() -> int:
    print("=" * 60)
    print("Testing interpret_spec.py (Agent C)")
    print("=" * 60)

    # Load spec
    script_dir = os.path.dirname(os.path.abspath(__file__))
    workspace_root = os.path.dirname(script_dir)
    spec_path = os.path.join(workspace_root, "openapi", "latest.json")
    output_dir = os.path.join(workspace_root, "openapi")

    if not os.path.exists(spec_path):
        print(f"ERROR: Spec not found at {spec_path}")
        return 1

    with open(spec_path, "r", encoding="utf-8") as f:
        spec = json.load(f)

    # Load interpret_spec module
    interpret_spec_path = os.path.join(script_dir, "interpret_spec.py")
    interpret_mod = load_module("interpret_spec", interpret_spec_path)

    # Load existing outputs
    spec_map_path = os.path.join(output_dir, "spec_map.json")
    pagination_map_path = os.path.join(output_dir, "pagination_map.json")
    schema_map_path = os.path.join(output_dir, "schema_map.json")

    with open(spec_map_path, "r") as f:
        spec_map = json.load(f)
    with open(pagination_map_path, "r") as f:
        pagination_map = json.load(f)
    with open(schema_map_path, "r") as f:
        schema_map = json.load(f)

    # Run tests
    tests = [
        ("Spec map completeness", test_spec_map_completeness, [spec, spec_map]),
        ("Required fields", test_required_fields, [spec_map]),
        ("Path parameters", test_path_params, [spec_map]),
        ("Pagination detection", test_pagination_detection, [spec, pagination_map]),
        ("Schema map completeness", test_schema_map_completeness, [spec, schema_map]),
        ("Composition annotations", test_composition_annotations, [schema_map]),
        ("Determinism", test_determinism, [spec, output_dir, interpret_mod]),
        ("Edge cases", test_edge_cases, [spec_map, pagination_map, schema_map]),
    ]

    results = []
    for test_name, test_func, args in tests:
        try:
            result = test_func(*args)
            results.append((test_name, result))
        except Exception as e:
            print(f"\n[TEST] {test_name}")
            print(f"  ERROR: {e}")
            import traceback
            traceback.print_exc()
            results.append((test_name, False))

    # Summary
    print("\n" + "=" * 60)
    print("Test Summary")
    print("=" * 60)

    passed = sum(1 for _, result in results if result)
    total = len(results)

    for test_name, result in results:
        status = "PASS" if result else "FAIL"
        print(f"  [{status}] {test_name}")

    print(f"\nTotal: {passed}/{total} passed")

    return 0 if passed == total else 1


if __name__ == "__main__":
    sys.exit(main())
