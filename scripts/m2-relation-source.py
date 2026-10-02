#!/usr/bin/env python3
"""Compile a read-only M2 source projection for the native relation-plan producer.

Every literal and typed edge is retained at the existing M-tree ID/record.
This command never queries or mutates the graph. It requires an already
qualified Bimba READ and registry; no invented tuning or prime descendants.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import re
from pathlib import Path

SCHEMA = "ql.m2-relation-source/v1"
M = re.compile(r"^M[0-5](?:[-./()0-9]*[0-9)])?$")

def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))

def sha(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()

def native_ref(coordinate):
    return "#" + coordinate[1:].replace("(", "").replace(")", "")

def compile_field(read, registry):
    if read.get("schema") != "ql.bimba-map-read/v1":
        raise ValueError("unsupported Bimba READ")
    content = {"nodes": read["nodes"], "relations": read["relations"]}
    if sha(content) != read["content_sha256"]:
        raise ValueError("READ content hash mismatch")
    if registry["source_repository"] != "bimba-map" or registry["source_revision"] != read["content_sha256"]:
        raise ValueError("READ is not the registry source")
    native = {n["source_ref"]: n for n in registry["nodes"]}
    edges = {r["relation_ref"]: r for r in registry["relations"]}
    nodes = []
    relations = []
    for index, (coordinate, node) in enumerate(sorted(read["nodes"].items())):
        if not M.fullmatch(coordinate) or not coordinate.startswith("M2"):
            continue
        ref = native_ref(coordinate)
        n = native[ref]
        props = node["properties"]
        records = [registry["records"][r] for r in n["records"]]
        record = next((r for r in records if r["record_index"] == index and r["payload_sha256"] == sha(props)), None)
        if record is None:
            raise ValueError("node source record mismatch: " + ref)
        nodes.append({"coordinate": ref, "map_coordinate": coordinate, "id": n["id"],
                      "record_index": index, "payload_sha256": sha(props),
                      "canonical_properties": canonical(props)})
    for index, (a, kind, b, props) in enumerate(read["relations"]):
        ref = "bimba:relation:" + sha([a, kind, b])[:24]
        r = edges.get(ref)
        if r is None or not any(str(r.get(k, "")).startswith("#2") for k in ["from_ref", "to_ref"]):
            continue
        record = registry["records"][r["record"]]
        if record["record_index"] != len(read["nodes"]) + index or record["payload_sha256"] != sha(props):
            raise ValueError("edge source record mismatch: " + ref)
        relations.append({"id": r["id"], "relation_ref": ref, "kind": kind,
                          "from_coordinate": r["from_ref"], "to_coordinate": r["to_ref"],
                          "record_index": record["record_index"], "payload_sha256": sha(props),
                          "canonical_properties": canonical(props)})
    # Prime source nodes are outside the native M tree. Their exact full source
    # is retained without assigning a fictional native ID or child family.
    primes = [{"map_coordinate": c, "canonical_properties": canonical(n["properties"])}
              for c, n in sorted(read["nodes"].items()) if c == "M2'" or c.startswith("M2'-")]
    return {"schema": SCHEMA, "registry_revision": registry["registry_revision"],
            "source_revision": registry["source_revision"],
            "source_repository": registry["source_repository"],
            "source_path": registry["files"][0]["path"],
            "source_sha256": registry["files"][0]["sha256"],
            "standing": "native-source-projection; no-physical-or-authentic-tuning-claim",
            "nodes": nodes, "relations": sorted(relations, key=lambda r: r["relation_ref"]),
            "prime_source_nodes": primes}

def render(field):
    lines = []
    for key, value in field.items():
        body = "[\n" + ",\n".join("    " + canonical(row) for row in value) + "\n  ]" if isinstance(value, list) else canonical(value)
        lines.append("  " + json.dumps(key) + ": " + body)
    return "{\n" + ",\n".join(lines) + "\n}\n"

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["refresh", "check"])
    parser.add_argument("--map", type=Path, required=True)
    parser.add_argument("--registry", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    field = compile_field(json.loads(args.map.read_text()), json.loads(args.registry.read_text()))
    text = render(field)
    if args.command == "refresh":
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(text)
    elif not args.output.is_file() or args.output.read_text() != text:
        raise SystemExit("stale M2 relation source projection")
    print(f"M2 relation source: {len(field['nodes'])} native nodes, {len(field['relations'])} typed edges, {len(field['prime_source_nodes'])} exact prime sources")
if __name__ == "__main__":
    main()

