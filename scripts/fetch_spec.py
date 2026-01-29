#!/usr/bin/env python3
"""
Fetch the latest Torn OpenAPI spec.

Downloads the OpenAPI v2 spec from https://www.torn.com/swagger/openapi.json
and writes it to openapi/latest.json in the workspace root.

Uses a custom User-Agent to avoid Cloudflare blocks.
"""

import json
import os
import sys
import urllib.request

SPEC_URL = "https://www.torn.com/swagger/openapi.json"
USER_AGENT = "rs-torn-client/0.1.0 (spec-fetcher; +https://github.com/YOUR_ORG/rs-torn-client)"

# Resolve workspace root (parent of scripts/)
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
WORKSPACE_ROOT = os.path.dirname(SCRIPT_DIR)
OUTPUT_DIR = os.path.join(WORKSPACE_ROOT, "openapi")
OUTPUT_PATH = os.path.join(OUTPUT_DIR, "latest.json")


def fetch_spec() -> dict:
    """Fetch the OpenAPI spec and return parsed JSON."""
    print(f"Fetching spec from {SPEC_URL} ...")
    req = urllib.request.Request(SPEC_URL, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=30) as resp:
        raw = resp.read()
    spec = json.loads(raw)
    print(f"  Fetched spec: {spec.get('info', {}).get('title', '?')} "
          f"v{spec.get('info', {}).get('version', '?')}")
    return spec


def write_spec(spec: dict) -> None:
    """Write spec JSON to the output path (pretty-printed, deterministic order)."""
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    with open(OUTPUT_PATH, "w", encoding="utf-8", newline="\n") as f:
        json.dump(spec, f, indent=2, sort_keys=True, ensure_ascii=False)
        f.write("\n")
    print(f"  Wrote {OUTPUT_PATH}")


def main() -> int:
    try:
        spec = fetch_spec()
        write_spec(spec)
        return 0
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
