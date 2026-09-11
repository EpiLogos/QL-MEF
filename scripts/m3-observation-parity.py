#!/usr/bin/env python3
"""Check executed K7 native observations against independent locked Bimba bindings.

This is the missing third side of Bimba/C/Rust parity: two implementations can
agree while both attach an operation to the wrong source coordinate. This reader
uses the lossless source audit, not generate-m3.py or a copy of the C address map.
It checks native group bindings, all DNA/RNA outputs and two full clock covers.
Other native finite-domain computations remain the existing engine suite's work.
"""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
_spec = importlib.util.spec_from_file_location("m3_locked_source", ROOT / "scripts/m3-source-parity.py")
source = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(source)

ALPHABET = "ATCG"
GROUP_COUNTS = (1, 4, 16, 64, 8, 64, 3, 56, 22, 360, 24, 1)
SELECTED = {"node", "transcription", "clock"}
OTHER_KINDS = {"pair", "trigram", "codon", "line", "matrix", "profile", "pose",
               "rotation", "quaternion", "partner", "minor", "major", "transduce", "record"}


def sequence(address: int) -> str:
    return "".join(ALPHABET[(address >> shift) & 3] for shift in (4, 2, 0))


def expected_rows(projection: dict, audit: dict) -> list:
    """Independent source-derived expectations, not an execution receipt."""
    nodes = projection["nodes"]
    by_ref = {n["ref"]: n for n in nodes}

    def named(name: str, role: str | None = None) -> dict:
        found = [n for n in nodes if n["properties"].get("name") == name and
                 (role is None or n["role"] == role)]
        source.require(len(found) == 1, "ambiguous source name: " + name)
        return found[0]

    def role_nodes(role: str) -> list:
        return [n for n in nodes if n["role"] == role]

    nuc = {n["properties"]["symbol"]: n for n in role_nodes("nucleotide")}
    pair = {n["properties"]["sequence"]: n for n in role_nodes("dinucleotide")}
    codon = {n["properties"]["sequence"]: n for n in role_nodes("dna-codon")}
    minor = []
    for suit in ("Cups", "Wands", "Pentacles", "Swords"):
        parent = named(suit)
        children = [n for n in role_nodes("minor-arcana") if n["parent_id"] == parent["id"]]
        source.require({n["properties"]["qlPosition"] for n in children} == set(range(14)),
                       "incomplete source suit rank field: " + suit)
        minor.extend(sorted(children, key=lambda n: n["properties"]["qlPosition"]))
    groups = [
        [by_ref["#3"]],
        [nuc[c] for c in ALPHABET],
        [pair[a+b] for a in ALPHABET for b in ALPHABET],
        [codon[sequence(i)] for i in range(64)],
        [named(name, "trigram") for name in ("Qian", "Kun", "Zhen", "Xun", "Kan", "Li", "Gen", "Dui")],
        [by_ref[h["ref"]] for h in sorted(audit["details"]["hexagrams"], key=lambda h: h["trigram_derived_address"])],
        [named(f"Matrix {i}") for i in range(1, 4)],
        minor,
        sorted(role_nodes("major-arcana"), key=lambda n: n["properties"]["number"]),
        sorted(role_nodes("clock-degree"), key=lambda n: n["properties"]["degree"]),
        sorted(role_nodes("clock-backbone"), key=lambda n: (n["properties"]["quadrant"], n["properties"]["position"])),
        [by_ref["#3-4.0"]],
    ]
    source.require(tuple(map(len, groups)) == GROUP_COUNTS, "native/source group cardinality drift")
    rows = [["node", kind, i, node["id"], node["ref"]]
            for kind, group in enumerate(groups) for i, node in enumerate(group)]
    rna = {r["dna"]: r["rna"] for r in audit["details"]["genetics"]["rna"]}
    for address in range(64):
        dna = codon[sequence(address)]["properties"]["sequence"]
        rows.append(["transcription", address, 0, dna])
        # Non-T forms are shared, not invented extra RNA coordinates.
        rows.append(["transcription", address, 1, rna.get(dna, dna)])
    clocks = audit["details"]["clock"]
    for step in range(1441):
        d, layer = step % 360, (step % 720) // 360
        c = clocks[d]
        # Only the clock arithmetic is derived here; source coordinate bindings
        # are taken independently from the complete checked relation projection.
        rows.append(["clock", step, step % 720, d, layer, d // 30,
                     d // 10 + layer * 36, layer * 360 + (d + 180) % 360,
                     d * 64 // 360, step // 720, c["id"], c["backbone_id"],
                     by_ref[c["clockwise_ref"]]["id"], by_ref[c["opposite_ref"]]["id"]])
    return rows


def identity(row: list) -> tuple:
    source.require(isinstance(row, list) and row and isinstance(row[0], str), "malformed observation record")
    kind = row[0]
    source.require(kind in SELECTED | OTHER_KINDS, "unknown native observation kind: " + kind)
    if kind in {"node", "transcription"}:
        source.require(len(row) == (5 if kind == "node" else 4), "wrong native observation width: " + kind)
        source.require(all(type(x) is int for x in row[1:3]), "non-integer native observation key")
        return tuple(row[:3])
    if kind == "clock":
        source.require(len(row) == 14, "wrong native observation width: clock")
        source.require(type(row[1]) is int, "non-integer clock observation key")
        return tuple(row[:2])
    # Other native outputs are retained in the input digest, but deliberately
    # not re-certified by a source-binding comparison.
    return (kind,)


def check(projection: dict, audit: dict, observations: list) -> dict:
    expected = {identity(row): row for row in expected_rows(projection, audit)}
    seen = {}
    categories = Counter()
    for row in observations:
        key = identity(row)
        categories[row[0]] += 1
        if row[0] not in SELECTED:
            continue
        source.require(key not in seen, "duplicate native observation: " + str(key))
        source.require(key in expected, "out-of-domain native observation: " + str(key))
        seen[key] = row
    missing = sorted(set(expected) - set(seen))
    source.require(not missing, "missing native observations: " + repr(missing[:8]))
    # Match types as well as values: bools and numeric strings are not IDs/indices.
    differences = [{"key": list(key), "expected": value, "observed": seen[key]}
                   for key, value in expected.items() if source.canonical(value) != source.canonical(seen[key])]
    return {
        "schema": "ql.m3-native-source-observation/v1",
        "status": "failed" if differences else "passed",
        "registry_revision": projection["registry_revision"],
        "bimba_source_revision": projection["source_revision"],
        "source_projection_sha256": source.digest(projection),
        "checked": {"coordinate_bindings": 623, "transcriptions": 128, "clock_frames": 1441},
        "input_categories": dict(sorted(categories.items())),
        "differences": differences,
        "standing": {
            "stratum": "c", "source_coordinate_transcription_clock": "observed",
            "historic_source_discrepancies": "retained-in-canonical-source-audit",
            "other_native_operations": "not-recertified-by-this-check",
            "rust": "requires-separate-executed-C-Rust-comparison",
            "symbolic_clock_placeholders": "not-promoted",
            "K4_acceptance": "not-claimed", "neo4j_live": "not-observed",
            "cpp_embodiment": "not-executed", "experiential": "not-claimed",
        },
    }


def read_observations(path: Path) -> tuple[list, str]:
    raw = path.read_bytes()
    source.require(len(raw) <= 8 * 1024 * 1024, "native observation exceeds bounded finite-domain size")
    def invalid(value):
        raise ValueError("non-finite JSON value: " + value)
    def finite_float(value):
        number = float(value)
        source.require(math.isfinite(number), "non-finite JSON float: " + value)
        return number
    rows = [json.loads(line, parse_constant=invalid, parse_float=finite_float) for line in raw.decode("utf-8").splitlines() if line.strip()]
    return rows, hashlib.sha256(raw).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-root", required=True, type=Path)
    parser.add_argument("--input", required=True, type=Path)
    parser.add_argument("--producer-revision-file", required=True, type=Path)
    parser.add_argument("--expected-revision", required=True)
    parser.add_argument("--output", type=Path, default=ROOT / "target/m3-source/native-parity.json")
    args = parser.parse_args()
    try:
        revision = args.producer_revision_file.read_text().strip()
        source.require(re.fullmatch(r"[0-9a-f]{40}", revision) is not None and revision == args.expected_revision,
                       "native producer/expected revision mismatch")
        registry, nodes, edges = source.read_source(args.source_root)
        projection = source.project(registry, nodes, edges)
        audit = source.Audit(projection).run()
        source.require(source.lock_for(projection, audit) == source.load_json(ROOT / source.LOCK),
                       "source audit lock drift")
        observations, sha = read_observations(args.input)
        result = check(projection, audit, observations)
        result.update({"producer_revision": revision, "input_sha256": sha})
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(source.render(result))
        print(source.render(result), end="")
        return 0 if result["status"] == "passed" else 1
    except (ValueError, KeyError, TypeError, OSError) as exc:
        print("M3 native/source observation failed: " + str(exc), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
