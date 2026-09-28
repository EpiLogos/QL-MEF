#!/usr/bin/env python3
"""Read the Bimba map, the owner's Neo4j graph, which is the authority QL builds from.

Every statement runs through the Query API with accessMode READ, so the server
itself refuses writes. The read lands in target/bimba-map/map.json (not
committed); generators record its content hash. There is no default endpoint:
pass --endpoint or set QL_BIMBA_MAP (e.g. http://100.92.62.101:7474 on the
owner's tailnet). QL_BIMBA_MAP_PASSWORD is used when the graph has auth.

    python3 scripts/bimba_map.py read [--endpoint URL]
"""
from __future__ import annotations

import argparse
import base64
import datetime
import hashlib
import json
import os
import re
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CACHE = ROOT / "target/bimba-map/map.json"
M_COORDINATE = re.compile(r"^M[0-5](?:[-./()0-9]*[0-9)])?$")

STATEMENTS = {
    # Embedding vectors are derived search data, not map content.
    "nodes": "MATCH (n:Bimba) WHERE n.coordinate IS NOT NULL "
             "RETURN n.coordinate AS coordinate, labels(n) AS labels, "
             "[k IN keys(n) WHERE NOT k STARTS WITH 'embedding' | [k, n[k]]] AS properties "
             "ORDER BY coordinate",
    "relations": "MATCH (a:Bimba)-[r]->(b:Bimba) "
                 "WHERE a.coordinate IS NOT NULL AND b.coordinate IS NOT NULL "
                 "RETURN a.coordinate AS source, type(r) AS type, b.coordinate AS target, "
                 "properties(r) AS properties ORDER BY source, type, target",
}


def canonical(value) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def query(endpoint: str, statement: str) -> tuple[list[dict], list[str]]:
    headers = {"Content-Type": "application/json", "Accept": "application/json"}
    password = os.environ.get("QL_BIMBA_MAP_PASSWORD")
    if password:
        user = os.environ.get("QL_BIMBA_MAP_USER", "neo4j")
        headers["Authorization"] = "Basic " + base64.b64encode(f"{user}:{password}".encode()).decode()
    request = urllib.request.Request(endpoint.rstrip("/") + "/db/neo4j/query/v2", headers=headers,
                                     data=json.dumps({"statement": statement, "accessMode": "READ"}).encode())
    with urllib.request.urlopen(request, timeout=300) as response:
        body = json.loads(response.read().decode())
    if body.get("errors"):
        raise SystemExit(f"Bimba map refused the read: {body['errors']}")
    fields = body["data"]["fields"]
    return [dict(zip(fields, row)) for row in body["data"]["values"]], body.get("bookmarks", [])


def read(endpoint: str) -> dict:
    rows, bookmarks = {}, []
    for name, statement in STATEMENTS.items():
        rows[name], bookmarks = query(endpoint, statement)
    content = {
        "nodes": {r["coordinate"]: {"labels": sorted(r["labels"]),
                                    "properties": dict(sorted((k, v) for k, v in r["properties"]))}
                  for r in rows["nodes"]},
        "relations": [[r["source"], r["type"], r["target"], r["properties"]] for r in rows["relations"]],
    }
    return {"schema": "ql.bimba-map-read/v1", "endpoint": endpoint,
            "read_at": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "bookmarks": bookmarks, "content_sha256": hashlib.sha256(canonical(content)).hexdigest(),
            **content}


def load(path: Path = CACHE) -> dict:
    if not path.is_file():
        raise SystemExit(f"no Bimba map read at {path.relative_to(ROOT)}; "
                         "run: python3 scripts/bimba_map.py read --endpoint URL")
    value = json.loads(path.read_text(encoding="utf-8"))
    content = {"nodes": value["nodes"], "relations": value["relations"]}
    if hashlib.sha256(canonical(content)).hexdigest() != value["content_sha256"]:
        raise SystemExit("Bimba map read does not match its content hash")
    return value


def is_m_coordinate(coordinate: str) -> bool:
    """M0..M5 tree coordinates. Primes (M0') and other lattices are not M-tree nodes."""
    return bool(M_COORDINATE.fullmatch(coordinate))


def ql_spelling(coordinate: str) -> str:
    """QL's # notation: the map spelling with M -> # and context-frame brackets dropped."""
    return "#" + coordinate[1:].replace("(", "").replace(")", "")


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("command", choices=["read"])
    parser.add_argument("--endpoint", default=os.environ.get("QL_BIMBA_MAP"))
    parser.add_argument("--out", type=Path, default=CACHE)
    args = parser.parse_args(argv)
    if not args.endpoint:
        parser.error("no endpoint: pass --endpoint or set QL_BIMBA_MAP")
    value = read(args.endpoint)
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(value, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Bimba map: {len(value['nodes'])} nodes, {len(value['relations'])} relations, "
          f"{value['content_sha256']} ({value['read_at']})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
