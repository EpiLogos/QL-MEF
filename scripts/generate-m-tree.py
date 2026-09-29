#!/usr/bin/env python3
"""Generate the native M registry from the Bimba map.

The map (the owner's Neo4j graph) is the authority. `--from-map` turns a
read-only map read (scripts/bimba_map.py) into the committed source snapshot
c/registry/m-tree-source-v1.json; every other run, including CI, regenerates
the registry, C tables and Rust manifest from that snapshot with no network.
The snapshot is the M-tree extract of the map: coordinates, names, relations,
and per-record property keys with payload hashes, not the property values.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SNAPSHOT = Path("c/registry/m-tree-source-v1.json")
MANIFEST = Path("fixtures/kernel/m-tree-v1.json")
TABLES = Path("c/src/m_tree_data.inc")
SCHEMA = "ql.m-tree/v1"
SOURCE_SCHEMA = "ql.m-tree-source/v1"
COORD = re.compile(r"#[0-5](?:[-./][0-9]+)*\Z")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def canonical(value: object) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":")).encode("utf-8")


def digest(value: object) -> str:
    return hashlib.sha256(canonical(value)).hexdigest()


def stable_id(domain: str, spelling: str) -> str:
    # Persist the full spelling too. A collision is an error, never an alias.
    result = hashlib.sha256((domain + "\0" + spelling).encode()).hexdigest()[:16]
    require(result != "0000000000000000", "reserved zero ID collision")
    return result


def read(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def render(value: dict) -> str:
    """One record per line: deterministic and reviewable without 200k-line fixtures."""
    parts = []
    for key, item in value.items():
        if isinstance(item, list):
            text = "[\n" + ",\n".join("    " + canonical(row).decode() for row in item) + "\n  ]"
        else:
            text = canonical(item).decode()
        parts.append("  " + json.dumps(key) + ": " + text)
    return "{\n" + ",\n".join(parts) + "\n}\n"


def from_map(value: dict, root: Path) -> dict:
    """The M-tree extract of one map read, in the source-snapshot shape.

    Coordinates are the map's M0..M5 nodes in QL's # notation (M -> #, context
    frame brackets dropped); the exact map spelling is kept as an alias. Names
    are c_1_name. HAS_INTERNAL_COMPONENT gives the source parent assertions.
    Relations are every map relation touching an M-tree node; an endpoint outside
    the M tree is spelled "bimba:<map coordinate>".
    """
    sys.path.insert(0, str(root / "scripts"))
    import bimba_map
    spell = {c: bimba_map.ql_spelling(c) for c in value["nodes"] if bimba_map.is_m_coordinate(c)}
    require(len(set(spell.values())) == len(spell), "map coordinates collide in # notation")
    content = {"nodes": value["nodes"], "relations": value["relations"]}
    body = canonical(content)
    require(hashlib.sha256(body).hexdigest() == value["content_sha256"], "map read content hash")
    git_blob = hashlib.sha1(b"blob %d\0" % len(body) + body).hexdigest()
    receipt = {k: value[k] for k in ("endpoint", "read_at", "bookmarks", "content_sha256")}
    files = [{"path": "bimba-map:" + value["endpoint"], "git_blob": git_blob,
              "sha256": value["content_sha256"], "bytes": len(body), "record_class": "bimba-map-read"}]
    records = []

    def record(index: int, payload: dict) -> int:
        records.append({"file": 0, "record_index": index, "payload_sha256": digest(payload),
                        "property_keys": sorted(payload)})
        return len(records) - 1

    parents: dict[str, set[str]] = {}
    for a, kind, b, _ in value["relations"]:
        if kind == "HAS_INTERNAL_COMPONENT" and a in spell and b in spell:
            parents.setdefault(spell[b], set()).add(spell[a])
    nodes = []
    for index, (coordinate, node) in enumerate(sorted(value["nodes"].items())):
        if coordinate not in spell:
            continue
        ref, props = spell[coordinate], node["properties"]
        lexical = re.fullmatch(r"(#[0-5](?:[-./][0-9]+)*)[-./][0-9]+", ref)
        name = props.get("c_1_name")
        nodes.append({"source_ref": ref,
                      "names": [name.strip()] if isinstance(name, str) and name.strip() else [],
                      "aliases": [coordinate] if "(" in coordinate else [],
                      "source_parent_refs": sorted(parents.get(ref, ())),
                      "lexical_parent_source_ref": lexical.group(1) if lexical else "#",
                      "records": [record(index, props)]})
    relations = []
    for index, (a, kind, b, props) in enumerate(value["relations"]):
        if a not in spell and b not in spell:
            continue
        # Endpoints outside the M tree (S/L/C lattices, primes, the map's own
        # "#0".."#5" meta nodes) keep their map spelling under "bimba:", so they
        # can never be read as QL's #N coordinates.
        from_ref, to_ref = spell.get(a, "bimba:" + a), spell.get(b, "bimba:" + b)
        relations.append({"relation_ref": "bimba:relation:" + digest([a, kind, b])[:24],
                          "source_kind": kind, "from_ref": from_ref, "to_ref": to_ref,
                          "orientation": "directed",
                          "cross_m": a in spell and b in spell and from_ref[1] != to_ref[1],
                          "record": record(len(value["nodes"]) + index, props)})
    return {"schema": SOURCE_SCHEMA, "repository": "bimba-map",
            "revision": value["content_sha256"], "dataset_tree": ",".join(value["bookmarks"]),
            "source_lock_sha256": digest(receipt),
            "compiler_sha256": {p: hashlib.sha256((root / p).read_bytes()).hexdigest()
                                for p in ("scripts/bimba_map.py", "scripts/generate-m-tree.py")},
            "files": files, "records": records, "nodes": nodes,
            "relations": sorted(relations, key=lambda r: r["relation_ref"]),
            "alternate_notation_groups": [], "meta_source_records": []}


def prefix_of(parent: str, child: str) -> bool:
    return child.startswith(parent) and len(child) > len(parent) and child[len(parent)] in "-./"


def model(source: dict) -> dict:
    require(source["schema"] == SOURCE_SCHEMA, "unsupported source schema")
    sources = {n["source_ref"]: n for n in source["nodes"]}
    require(len(sources) == len(source["nodes"]), "duplicate coordinate")
    require(all(COORD.fullmatch(ref) for ref in sources), "invalid source coordinate")
    require(all(f"#{i}" in sources for i in range(6)), "missing aggregate source root")
    ids = {ref: stable_id("ql.m-node/v1", ref) for ref in ["M", *sorted(sources)]}
    require(len(set(ids.values())) == len(ids), "coordinate ID collision")
    nodes = [{"id": ids["M"], "source_ref": "M", "parent_id": None, "root_id": ids["M"],
              "root_position": None, "local_segment": "M", "separator": "", "depth": 0,
              "lexical_depth": 0, "lexical_parent_source_ref": None, "parent_basis": "master",
              "source_parent_refs": [], "structural_status": "master-index", "aggregate": True,
              "names": ["M"], "aliases": [], "records": []}]
    discrepancies = []
    for ref, n in sorted(sources.items()):
        root = int(ref[1])
        explicit = n["source_parent_refs"]
        # Only one prefix ancestor can supply tree containment. Other source
        # parent assertions remain explicit relational evidence, never discarded.
        candidates = [p for p in explicit if p in sources and prefix_of(p, ref)]
        require(len(candidates) <= 1, "ambiguous source tree parent: " + ref)
        if len(ref) == 2:
            parent, basis, segment, separator = "M", "aggregate-root", ref[1], ""
        else:
            if candidates:
                parent, basis = candidates[0], "source-parent"
            else:
                ancestors = [p for p in sources if prefix_of(p, ref)]
                require(bool(ancestors), "source node without root: " + ref)
                parent = max(ancestors, key=len)
                basis = "existing-source-prefix"
            separator, segment = ref[len(parent)], ref[len(parent) + 1:]
        if any(p != parent for p in explicit):
            discrepancies.append({"coordinate": ref, "tree_parent": parent,
                                  "source_parent_refs": explicit,
                                  "standing": "non-tree-parent-assertion-retained-as-source-relation"})
        nodes.append({"id": ids[ref], "source_ref": ref, "parent_id": ids[parent],
                      "root_id": ids[f"#{root}"], "root_position": root,
                      "local_segment": segment, "separator": separator, "depth": 0,
                      "lexical_depth": len(re.findall(r"[-./][0-9]+", ref)),
                      "lexical_parent_source_ref": n["lexical_parent_source_ref"],
                      "parent_basis": basis, "source_parent_refs": explicit,
                      "structural_status": "source-declared", "aggregate": len(ref) == 2,
                      "names": n["names"], "aliases": n["aliases"], "records": n["records"]})
    by_id = {n["id"]: n for n in nodes}
    children = {i: [] for i in by_id}
    for n in nodes[1:]:
        children[n["parent_id"]].append(n["id"])
    # Iterative traversal: source recursion is not bounded by six levels or a
    # recursive host call stack. No missing sibling/prefix is materialised.
    order = [ids["M"]]
    for identity in order:
        for child in children[identity]:
            by_id[child]["depth"] = by_id[identity]["depth"] + 1
            order.append(child)
    require(len(order) == len(nodes), "disconnected/cyclic M tree")
    for identity in reversed(order):
        n = by_id[identity]
        n["children"] = children[identity]
        n["subtree_count"] = 1 + sum(by_id[c]["subtree_count"] for c in children[identity])
    relations = []
    relation_ids = set()
    for r in sorted(source["relations"], key=lambda r: r["relation_ref"]):
        relation_id = stable_id("ql.m-relation/v1", r["relation_ref"])
        require(relation_id not in relation_ids, "duplicate/colliding relation ID")
        relation_ids.add(relation_id)
        for endpoint in (r["from_ref"], r["to_ref"]):
            require(endpoint is None or not COORD.fullmatch(endpoint) or endpoint in ids,
                    "relation refers to missing source coordinate: " + str(endpoint))
        relations.append({**r, "id": relation_id, "class": "bimba-source",
                          "from_id": ids.get(r["from_ref"]), "to_id": ids.get(r["to_ref"])})
    for f in source["files"]:
        require(re.fullmatch(r"[0-9a-f]{40}", f["git_blob"]) is not None, "invalid source blob")
        require(re.fullmatch(r"[0-9a-f]{64}", f["sha256"]) is not None, "invalid file digest")
    for r in source["records"]:
        require(0 <= r["file"] < len(source["files"]) and r["record_index"] >= 0,
                "invalid record provenance")
        require(re.fullmatch(r"[0-9a-f]{64}", r["payload_sha256"]) is not None, "invalid payload digest")
    require(all(0 <= i < len(source["records"]) for n in nodes for i in n["records"]),
            "invalid node record")
    require(all(0 <= r["record"] < len(source["records"]) for r in relations), "invalid relation record")
    bindings = [{"identity": "ql.m-index:" + ("M" if i is None else f"M{i}"),
                 "module": "c/src/m_tree.c", "disposition": "COORDINATE-BOUND",
                 "coordinate_id": ids["M" if i is None else f"#{i}"], "relation_id": None,
                 "readiness": "structural-index-only", "evidence": "tests/native-m-tree"}
                for i in [None, *range(6)]]
    result = {"schema": SCHEMA, "id_scheme": "first-64-bits-sha256-domain-NUL-exact-source-spelling",
              "source_repository": source["repository"], "source_revision": source["revision"],
              "source_dataset_tree": source["dataset_tree"], "source_snapshot_sha256": digest(source),
              "source_lock_sha256": source["source_lock_sha256"], "compiler_sha256": source["compiler_sha256"],
              "master_id": ids["M"], "roots": [ids[f"#{i}"] for i in range(6)],
              "files": source["files"], "records": source["records"], "nodes": nodes,
              "relations": relations, "bindings": bindings,
              "alternate_notation_groups": source["alternate_notation_groups"],
              "meta_source_records": source["meta_source_records"],
              "parent_discrepancies": discrepancies}
    result["registry_revision"] = digest(result)
    return result


def cstr(value: str | None) -> str:
    if value is None:
        return "NULL"
    require("\0" not in value, "embedded NUL is not representable in a native registry string")
    if len(value.encode()) > 4095:
        return "((const char[]){" + ",".join(f"(char){b}" for b in value.encode()) + ",0})"
    # UTF-8 octal bytes avoid C universal-character restrictions and hex-run-on.
    return '"' + ''.join(chr(b) if 32 <= b <= 126 and b not in (34, 92, 63)
                         else f"\\{b:03o}" for b in value.encode()) + '"'


def cid(value: str | None) -> str:
    return "UINT64_C(0x" + (value or "0000000000000000") + ")"


def tables(m: dict) -> str:
    out = ["/* Generated by scripts/generate-m-tree.py; do not hand-author another tree. */",
           f"static const char m_revision[] = {cstr(m['registry_revision'])};",
           f"static const char m_source_revision[] = {cstr(m['source_revision'])};",
           f"static const char m_source_repository[] = {cstr(m['source_repository'])};"]
    nodes = m["nodes"]
    node_indexes = {n["id"]: i for i, n in enumerate(nodes)}
    flat_children, flat_records, rows = [], [], []
    bases = {"master": "QL_M_PARENT_MASTER", "aggregate-root": "QL_M_PARENT_AGGREGATE",
             "source-parent": "QL_M_PARENT_SOURCE", "existing-source-prefix": "QL_M_PARENT_PREFIX"}
    for n in nodes:
        rows.append("    {" + ", ".join([cid(n['id']), cid(n['parent_id']), cid(n['root_id']),
            cstr(n['source_ref']), cstr(n['local_segment']), cstr(n['separator']),
            cstr(n['lexical_parent_source_ref']), cstr(canonical(n['source_parent_refs']).decode()),
            cstr(canonical(n['names']).decode()), cstr(canonical(n['aliases']).decode()),
            str(n['root_position'] if n['root_position'] is not None else 255), str(n['depth']),
            str(n['lexical_depth']), bases[n['parent_basis']],
            "QL_M_SOURCE_DECLARED" if n['structural_status'] == 'source-declared' else "QL_M_MASTER_INDEX",
            str(int(n['aggregate'])), str(len(flat_children)), str(len(n['children'])),
            str(len(flat_records)), str(len(n['records'])), str(n['subtree_count'])]) + "},")
        flat_children.extend(node_indexes[c] for c in n["children"])
        flat_records.extend(n["records"])
    out += ["static const QL_M_Node m_nodes[] = {", *rows, "};"]
    for name, values in (("m_children", flat_children), ("m_node_records", flat_records),
                         ("m_nodes_by_id", sorted(range(len(nodes)), key=lambda i: nodes[i]["id"]))):
        out += [f"static const uint32_t {name}[] = {{", ",".join(map(str, values)) or "0", "};"]
    out += ["static const QL_M_SourceFile m_files[] = {"]
    out += ["    {" + ", ".join(cstr(f[k]) for k in ("path", "git_blob", "sha256", "record_class")) + ", UINT64_C(" + str(f["bytes"]) + ")},"
            for f in m["files"]]
    out += ["};", "static const QL_M_SourceRecord m_records[] = {"]
    out += [f"    {{{r['file']}, {r['record_index']}, {cstr(r['payload_sha256'])}, "
            + cstr(canonical(r['property_keys']).decode()) + "}," for r in m["records"]]
    out += ["};", "static const QL_M_Relation m_relations[] = {"]
    orientations = {"directed": "QL_M_DIRECTED", "undirected": "QL_M_UNDIRECTED", "unspecified": "QL_M_UNSPECIFIED"}
    for r in m["relations"]:
        out += ["    {" + ", ".join([cid(r['id']), cid(r['from_id']), cid(r['to_id']),
                cstr(r['relation_ref']), cstr(r['source_kind']), cstr(r['from_ref']), cstr(r['to_ref']),
                orientations[r['orientation']], str(int(r['cross_m'])), str(r['record'])]) + "},"]
    out += ["};", "static const uint32_t m_relations_by_id[] = {",
            ",".join(map(str, sorted(range(len(m['relations'])), key=lambda i: m['relations'][i]['id']))) or "0", "};",
            "static const QL_M_Binding m_bindings[] = {"]
    out += ["    {" + ", ".join([cstr(b['identity']), cstr(b['module']), "QL_M_COORDINATE_BOUND",
            cid(b['coordinate_id']), cid(b['relation_id']), "QL_M_STRUCTURAL_INDEX_ONLY", cstr(b['evidence'])])
            + "}," for b in m["bindings"]]
    out += ["};", "static const QL_M_NodeId m_roots[] = {", ",".join(cid(r) for r in m["roots"]), "};"]
    return "\n".join(out) + "\n"


def outputs(root: Path, source: dict) -> dict[Path, str]:
    m = model(source)
    return {root / MANIFEST: render(m), root / TABLES: tables(m)}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--from-map", type=Path, nargs="?", const=ROOT / "target/bimba-map/map.json",
                        help="rebuild the source snapshot from a map read (default target/bimba-map/map.json)")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        source = from_map(read(args.from_map), args.root) if args.from_map else read(args.root / SNAPSHOT)
        generated = outputs(args.root, source)
        if args.from_map:
            generated[args.root / SNAPSHOT] = render(source)
        if args.check:
            drift = [str(p.relative_to(args.root)) for p, text in generated.items()
                     if not p.is_file() or p.read_bytes() != text.encode()]
            require(not drift, "generated registry drift: " + ", ".join(drift))
        else:
            for p, text in generated.items():
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text(text, encoding="utf-8")
        m = model(source)
        print(f"{SCHEMA}: {len(m['nodes'])} nodes, {len(m['relations'])} relations, {m['registry_revision']}")
    except (ValueError, KeyError, OSError) as error:
        sys.exit(f"M registry: {error}")


if __name__ == "__main__":
    main()
