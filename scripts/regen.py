#!/usr/bin/env python3
"""
Regeneration pipeline for rs-torn-client.

Steps:
  1. Fetch latest OpenAPI spec (delegates to fetch_spec.py)
  2. Parse spec and produce spec_map.json, pagination_map.json, schema_map.json
     (delegates to interpret_spec.py — Agent C)
  3. Generate Rust code into crates/torn_models/src/generated/
     (placeholder today — full codegen in future sessions)

This script is designed to be fully deterministic: given the same spec input,
it always produces the same output. CI runs this and asserts git diff is empty.
"""

import importlib.util
import json
import os
import sys

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
WORKSPACE_ROOT = os.path.dirname(SCRIPT_DIR)
SPEC_PATH = os.path.join(WORKSPACE_ROOT, "openapi", "latest.json")
OPENAPI_DIR = os.path.join(WORKSPACE_ROOT, "openapi")
GENERATED_DIR = os.path.join(WORKSPACE_ROOT, "crates", "torn_models", "src", "generated")


def load_module(module_name: str, file_name: str):
    """Dynamically import a Python module."""
    module_path = os.path.join(SCRIPT_DIR, file_name)
    spec = importlib.util.spec_from_file_location(module_name, module_path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def load_fetch_spec():
    """Dynamically import fetch_spec.py so we can call its functions."""
    return load_module("fetch_spec", "fetch_spec.py")


def load_interpret_spec():
    """Dynamically import interpret_spec.py so we can call its functions."""
    return load_module("interpret_spec", "interpret_spec.py")


def load_spec() -> dict:
    """Load spec from disk."""
    with open(SPEC_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


def load_generate_models():
    """Dynamically import generate_models.py so we can call its functions."""
    return load_module("generate_models", "generate_models.py")


def generate_models_impl() -> None:
    """
    Generate Rust models from OpenAPI schemas.

    Delegates to generate_models.py (Agent D).
    """
    schema_map_path = os.path.join(OPENAPI_DIR, "schema_map.json")
    spec_map_path = os.path.join(OPENAPI_DIR, "spec_map.json")

    generate_mod = load_generate_models()
    generate_mod.generate_models(schema_map_path, spec_map_path, GENERATED_DIR)


def main() -> int:
    print("=" * 60)
    print("rs-torn-client: Regeneration Pipeline")
    print("=" * 60)

    # Step 1: Fetch spec
    print("\n[1/3] Fetching latest OpenAPI spec...")
    fetch_mod = load_fetch_spec()
    try:
        spec = fetch_mod.fetch_spec()
        fetch_mod.write_spec(spec)
    except Exception as e:
        print(f"ERROR fetching spec: {e}", file=sys.stderr)
        return 1

    # Step 2: Interpret spec (Agent C)
    print("\n[2/3] Interpreting OpenAPI spec (Agent C)...")
    interpret_mod = load_interpret_spec()
    spec = load_spec()
    try:
        interpret_mod.interpret_spec(spec, OPENAPI_DIR)
    except Exception as e:
        print(f"ERROR interpreting spec: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1

    # Step 3: Generate Rust code (Agent D)
    print("\n[3/3] Generating Rust models (Agent D)...")
    generate_models_impl()

    print("\n" + "=" * 60)
    print("Regeneration complete.")
    print("=" * 60)
    return 0


if __name__ == "__main__":
    sys.exit(main())
