#!/usr/bin/env python3
"""Compare literal source registers to real C observations, without normalising discrepancies.

This is a witness/report generator, NOT a second ledger or coordinate tree.
The shared M ledger owns decisions. --write records a reviewed source return;
normal invocation compares byte-for-byte to the committed report.
"""
import argparse
import csv
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WITNESS = ROOT / "migration/epi-kernel/m1-return"
REPORT = ROOT / "docs/kernel-rebuild/m1-source-comparison-v1.json"


def compare(observations):
    lock = json.loads((WITNESS / "source-lock.json").read_text())
    for item in lock["files"]:
        data = (ROOT / item["local_path"]).read_bytes()
        assert hashlib.sha256(data).hexdigest() == item["sha256"], item["local_path"]
        assert hashlib.sha1(f"blob {len(data)}\0".encode() + data).hexdigest() == item["git_blob"], item["local_path"]
    with (WITNESS / "vortex-modulae.csv").open(encoding="utf-8-sig", newline="") as stream:
        source = list(csv.reader(stream))
    cells = {}
    with observations.open() as stream:
        for line in stream:
            cell = json.loads(line)
            if "family" in cell and cell["clock"]["cycle"] == "0" and cell["clock"]["tick12"] == 0:
                key = cell["family"], cell["row12"], cell["col12"]
                assert key not in cells, f"duplicate observation: {key}"
                cells[key] = cell
    assert len(cells) == 864, f"expected 864 actually executed source cells, found {len(cells)}"
    groups = {}
    for (family, row, col), cell in sorted(cells.items()):
        y = 4 + row + (row >= 10)
        x = [22, 3, 42, 64, 87, 108][family] + col + (col >= 10)
        for register, offset in [("raw", 0), ("digit-root", 34)]:
            literal = source[y + offset][x]
            if family < 5:
                runtime = str(cell["raw" if register == "raw" else "digit_root"])
            elif register == "raw":
                runtime = f"-1/{cell['raw_terms'][0]}/{cell['raw_terms'][2]}"
            else:
                runtime = f"{cell['dr_terms'][0]}/{cell['dr_terms'][2]}"
            key = f"family-{family}:{register}"
            group = groups.setdefault(key, {"coordinate": f"#1-2-{family}" + ("-0" if offset else ""),
                "family": family, "register": register, "equal": 0, "different": 0, "differences": []})
            if literal == runtime:
                group["equal"] += 1
            else:
                group["different"] += 1
                group["differences"].append([row, col, y + offset + 1, x + 1, literal, runtime])
    return {"schema": "ql.m1.literal-source-comparison/v1", "source_revision": lock["revision"],
        "comparison": "Exact literal comparison, not an inferred equivalence of symbolic/register meanings.",
        "decision_owner": "fixtures/kernel/m-ledger-v1.json; K4/#128 and K5/#129",
        "observed_cells": len(cells), "literal_readings": 1728,
        "equal": sum(g["equal"] for g in groups.values()), "different": sum(g["different"] for g in groups.values()),
        "difference_columns": ["row12", "col12", "csv_row_1based", "csv_column_1based", "literal", "runtime_projection"],
        "groups": list(groups.values())}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--observations", type=Path, default=ROOT / "target/m1-engine/c-rust.jsonl")
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    text = json.dumps(compare(args.observations), ensure_ascii=False, indent=2) + "\n"
    import re
    text = re.sub(r"\[\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\"[^\"]*\"),\s*(\"[^\"]*\")\s*\]", r"[\1, \2, \3, \4, \5, \6]", text)
    if args.write:
        REPORT.write_text(text)
    else:
        assert REPORT.read_text() == text, "source/runtime differences changed; review and return them through the shared ledger"
    result = json.loads(text)
    print(f"M1 source comparison: {result['equal']} equal literal readings, {result['different']} retained differences; no normalisation")


if __name__ == "__main__":
    main()
