#!/usr/bin/env python3
"""Live-corpus entry point for the deterministic Bimba Map compiler.

The underlying compiler owns normalization, provenance, relation preservation,
Nara reconciliation, and Cosmic-readiness output. This entry point carries
representation facts returned by the actual historical Map corpus:

* M0-M5 coordinates use `.`, `-`, and `/` as meaningful source separators;
* the rootless meta-field uses `#` and `#-0..#-5`, which are external/meta refs,
  not malformed M coordinates;
* several committed `*.json` exports contain raw control characters and require
  Python's non-strict JSON reader;
* some equal numeric paths have multiple source spellings. Those spellings are
  retained as an explicit alternate-notation group and are never silently
  canonicalised into one another.

None of those representation facts promotes Epi semantic payload into QL canon.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import re
import sys
from collections import Counter
from pathlib import Path

META_SOURCE_RE = re.compile(r"^#(?:-[0-5])?$")


def load_compiler():
    compiler_path = Path(__file__).with_name("compile-epi-bimba-map.py")
    spec = importlib.util.spec_from_file_location("epi_bimba_map_compiler", compiler_path)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load compiler engine from {compiler_path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def alternate_group_ref(source_refs: list[str]) -> str:
    payload = "\0".join(sorted(source_refs)).encode("utf-8")
    return f"bimba:alternate-notation:{hashlib.sha256(payload).hexdigest()[:20]}"


def classify_returned_source_reality(compiler, compiled: dict, out: Path) -> None:
    inventory_path = out / "source-inventory.json"
    inventory = json.loads(inventory_path.read_text(encoding="utf-8"))

    meta_records = []
    remaining_failures = []
    for failure in inventory.get("coordinate_parse_failures", []):
        source_ref = failure.get("source_ref")
        if isinstance(source_ref, str) and META_SOURCE_RE.fullmatch(source_ref):
            meta_records.append(
                {
                    **failure,
                    "classification": "rootless-meta-field-coordinate",
                    "standing": "external-meta-ref-not-m-coordinate",
                }
            )
        else:
            remaining_failures.append(failure)

    inventory["coordinate_parse_failures"] = remaining_failures
    inventory["meta_source_coordinate_records"] = meta_records
    inventory["source_representation_standing"] = {
        "m_coordinate_grammar": "#0..#5 plus recursive numeric segments separated by '.', '-', or '/'",
        "meta_coordinate_grammar": "# and #-0..#-5 are rootless meta-field refs outside M0-M5",
        "json_reader": "UTF-8/BOM tolerant and non-strict for committed historical raw-control-character exports",
        "alternate_notation": "equal numeric paths with different source spellings remain separately traceable; no implicit canonicalisation",
    }
    compiler.write_json(inventory_path, inventory)
    compiler.write_json(out / "meta-source-records.json", meta_records)

    alternate_groups = []
    for collision in compiled["path_collisions"]:
        source_refs = collision["source_refs"]
        collision["status"] = "alternate-source-spellings-retained"
        collision["alternate_notation_ref"] = alternate_group_ref(source_refs)
        collision["resolution_policy"] = (
            "preserve every source spelling; do not choose a canonical alias unless an explicit source/design binding does so"
        )
        alternate_groups.append(collision)
    compiler.write_json(out / "path-collisions.json", compiled["path_collisions"])
    compiler.write_json(out / "alternate-notation-groups.json", alternate_groups)

    unresolved_path = out / "unresolved-relation-endpoints.json"
    partial_relations = json.loads(unresolved_path.read_text(encoding="utf-8"))
    reason_counts = Counter(
        endpoint["reason"]
        for relation in partial_relations
        for endpoint in relation.get("endpoints", [])
    )
    partial_report = {
        "schema": "ql.epi-bimba-partial-source-relations/v1",
        "source_revision": compiled["revision"],
        "standing": (
            "source-preserved partial relation records; stable relation refs and source kinds remain resolvable, "
            "but missing/meta endpoints are not invented by QL-MEF"
        ),
        "relation_record_count": len(partial_relations),
        "endpoint_reason_counts": dict(sorted(reason_counts.items())),
        "relations": partial_relations,
    }
    compiler.write_json(out / "partial-source-relations.json", partial_report)

    summary = compiled["summary"]
    summary["coordinate_parse_failure_count"] = len(remaining_failures)
    summary["meta_source_coordinate_record_count"] = len(meta_records)
    summary["alternate_notation_group_count"] = len(alternate_groups)
    summary["partial_source_relation_count"] = len(partial_relations)
    summary["partial_source_relation_endpoint_reason_counts"] = dict(sorted(reason_counts.items()))
    compiler.write_json(out / "summary.json", summary)


def main() -> None:
    compiler = load_compiler()

    compiler.COORD_RE = re.compile(r"^#([0-5])((?:[./-][0-9]+)*)$")
    compiler.SEGMENT_RE = re.compile(r"([./-])([0-9]+)")
    compiler.COORD_ANY_RE = re.compile(r"#[0-5](?:[./-][0-9]+)*")

    def read_live_json(path: Path):
        return json.loads(path.read_text(encoding="utf-8-sig"), strict=False)

    compiler.read_json = read_live_json

    parser = argparse.ArgumentParser()
    parser.add_argument("--source-repo", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument(
        "--source-lock",
        type=Path,
        default=Path("data/epi-bimba-map/source-lock.json"),
    )
    parser.add_argument("--implementation-repo", type=Path)
    args = parser.parse_args()

    source_repo = args.source_repo.resolve()
    out = args.out.resolve()
    lock = compiler.load_lock(args.source_lock)
    compiled = compiler.compile_map(source_repo, out, lock)
    classify_returned_source_reality(compiler, compiled, out)

    nara = None
    if args.implementation_repo:
        nara = compiler.reconcile_nara(compiled, args.implementation_repo.resolve(), out)
    cosmic = compiler.cosmic_readiness(compiled, out)
    compiler.print_returned_reality(compiled["summary"], nara, cosmic)


if __name__ == "__main__":
    main()
