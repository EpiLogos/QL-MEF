#!/usr/bin/env python3
"""Retain complete source material from the existing admitted Bimba READ.

This is a deterministic, read-only content projection, not another graph or
an authored interpretation. CI checks the committed capsule without a network;
refresh uses the same full map read as the native registry compiler.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import bimba_map

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "fixtures/kernel/bimba-content-v1.json"


def validate(value: dict) -> None:
    if value.get("schema") != "ql.bimba-content/v1":
        raise ValueError("unsupported Bimba content schema")
    content = value["content"]
    if hashlib.sha256(bimba_map.canonical(content)).hexdigest() != value["source_revision"]:
        raise ValueError("Bimba content hash differs from its admitted full READ")
    if not isinstance(content["nodes"], dict) or not isinstance(content["relations"], list):
        raise ValueError("Bimba content must preserve nodes and directed qualified relations")
    for coordinate, node in content["nodes"].items():
        if not coordinate or not isinstance(node.get("properties"), dict):
            raise ValueError("Bimba source identity/properties absent")
        if node["properties"].get("coordinate") != coordinate:
            raise ValueError("Bimba coordinate/property identity mismatch")
    for edge in content["relations"]:
        if len(edge) != 4 or edge[0] not in content["nodes"] or edge[2] not in content["nodes"] or not isinstance(edge[3], dict):
            raise ValueError("Bimba qualified relation is incomplete")


def project(value: dict) -> dict:
    capsule = {"schema": "ql.bimba-content/v1", "source_revision": value["content_sha256"],
               "source_ref": "bimba-map:" + value["endpoint"],
               "content": {"nodes": value["nodes"], "relations": value["relations"]}}
    validate(capsule)
    return capsule


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    registry = json.loads((ROOT / "c/registry/m-tree-source-v1.json").read_text())
    if args.check:
        value = json.loads(OUTPUT.read_text())
        validate(value)
    else:
        value = project(bimba_map.load())
    if value["source_revision"] != registry["revision"]:
        raise ValueError("Bimba material source and compiled registry have different source revisions")
    if not args.check:
        OUTPUT.write_bytes(bimba_map.canonical(value) + b"\n")
    print(json.dumps({"schema": "ql.bimba-content-receipt/v1", "source_revision": value["source_revision"],
                      "nodes": len(value["content"]["nodes"]), "relations": len(value["content"]["relations"]),
                      "file": str(OUTPUT.relative_to(ROOT)), "standing": "complete source content; no numerical interpretation"}))


if __name__ == "__main__":
    main()
