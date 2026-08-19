#!/usr/bin/env python3
"""Live-corpus entry point for the deterministic Bimba Map compiler.

The underlying compiler owns normalization, provenance, relation preservation,
Nara reconciliation, and Cosmic-readiness output.  This entry point carries two
representation facts returned by the actual historical Map corpus:

* coordinates use `.`, `-`, and `/` as meaningful source separators;
* several committed `*.json` exports contain raw control characters and require
  Python's non-strict JSON reader.

Neither fact is semantic canon.  Both are source-representation facts which must
be preserved/accepted so the source body is not silently narrowed to the first
reader implementation.
"""

from __future__ import annotations

import importlib.util
import json
import re
import sys
from pathlib import Path


def load_compiler():
    compiler_path = Path(__file__).with_name("compile-epi-bimba-map.py")
    spec = importlib.util.spec_from_file_location("epi_bimba_map_compiler", compiler_path)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load compiler engine from {compiler_path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def main() -> None:
    compiler = load_compiler()

    compiler.COORD_RE = re.compile(r"^#([0-5])((?:[./-][0-9]+)*)$")
    compiler.SEGMENT_RE = re.compile(r"([./-])([0-9]+)")
    compiler.COORD_ANY_RE = re.compile(r"#[0-5](?:[./-][0-9]+)*")

    def read_live_json(path: Path):
        return json.loads(path.read_text(encoding="utf-8-sig"), strict=False)

    compiler.read_json = read_live_json
    compiler.main()


if __name__ == "__main__":
    main()
