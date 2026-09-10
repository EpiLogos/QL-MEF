#!/usr/bin/env python3
"""K3 matrix lock, deterministic seed import and evidence/observation verification.

Semantic validation and coverage live in ql_mef::m_ledger (also exposed by ql).
This tool owns checkout-dependent hashes and the lossless matrix import, not a
second coordinate tree. K4 edits assessments/bindings, not the import algorithm.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LEDGER = Path("fixtures/kernel/m-ledger-v1.json")
REGISTRY = Path("fixtures/kernel/m-tree-v1.json")
AXES = ("source", "coordinate", "relation", "operational", "experiential")
STRATA = ("source", "c", "rust", "cpp", "neo4j", "application", "instrument")
BINDING_STRATA = ("c", "rust", "cpp", "neo4j")
# A slash between digits is part of one source spelling. /M or <-> delimits
# authored cross-coordinate expressions. Ranges and primes are NOT expanded.
COORDINATE = re.compile(r"(?<![\w#])(?:M|#)[0-5](?:[-./][0-9]+)*(?![\w′'])")


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def digest(value):
    return hashlib.sha256(value).hexdigest()


def read(path):
    return json.loads(path.read_text(encoding="utf-8"))


def safe_path(root, raw):
    path = (root / raw).resolve()
    if not path.is_relative_to(root.resolve()) or not path.is_file():
        raise ValueError(f"missing or unsafe repository path: {raw}")
    return path


def lock(root, path):
    path = Path(path)
    return {"path": path.as_posix(), "sha256": digest(safe_path(root, path).read_bytes())}


def pointer(document, ref):
    if not ref.startswith("/"):
        raise ValueError(f"invalid JSON pointer: {ref}")
    for part in ref[1:].split("/"):
        key = part.replace("~1", "/").replace("~0", "~")
        document = document[int(key)] if isinstance(document, list) else document[key]
    return document


def matrices(root):
    """Import both native families, preserving the existing independent carriers."""
    result = []
    for path in sorted((root / "docs/origami work").glob("M*/*-DEEP-CAPABILITY-COORDINATE-MATRIX.json")):
        d = read(path)
        if d["schema"] != "epi.deep-subsystem-capability-matrix.v2":
            raise ValueError(f"unsupported deep matrix: {path}")
        scope = f"M{d['subsystem']['index']}"
        result.append((f"deep-{scope}", scope, path.relative_to(root), path.with_suffix(".md").relative_to(root)))
    for i in range(6):
        path = Path(f"docs/integrations/epi-logos/epi-m-capability-field-m{i}.json")
        if read(root / path)["schema_version"] != "epi-m-capability-domain/0.2":
            raise ValueError(f"unsupported normalized matrix: {path}")
        result.append((f"field-M{i}", f"M{i}", path, Path("docs/integrations/epi-logos/EPI-M-CAPABILITY-FIELD.md")))
    return result


def source_rows(root):
    locks, rows = [], []
    for mid, scope, data, rationale in matrices(root):
        locks.append({"id": mid, "data": lock(root, data), "rationale": lock(root, rationale)})
        for index, source in enumerate(read(root / data)["capabilities"]):
            cid = source.get("id", source.get("capability_ref"))
            if not isinstance(cid, str) or not cid:
                raise ValueError(f"missing capability id in {data}:{index}")
            expression = source.get("coordinate", "")
            refs = [expression, *source.get("relations", [])]
            coordinates = []
            for text in refs:
                # A relation range is a source expression, not a claim to its
                # first member. Keep it in the source, pending K4 interpretation.
                if "–" in text or "…" in text:
                    continue
                for match in COORDINATE.finditer(text):
                    ref = match.group()
                    if ref not in coordinates:
                        coordinates.append(ref)
            rows.append({
                "id": f"{mid}:{cid}", "role": source.get("name", source.get("for_what")),
                "scope": scope, "coordinates": coordinates,
                "source": {"matrix": mid, "pointer": f"/capabilities/{index}"},
                "assessment": "unassessed", "bindings": [], "dispositions": {},
                "invariants": [], "relations": [], "dependencies": [],
            })
    return locks, rows


def assessment(index=False):
    readiness = {s: {"status": "unassessed", "warrant": "unassessed", "evidence": []} for s in STRATA}
    if index:
        for s in ("c", "rust"):
            readiness[s] = {"status": "structural-index-only", "warrant": "source-declared", "evidence": ["k2-index-declaration"]}
    return {"readiness": readiness, "parity": {a: [] for a in AXES}}


def refresh(root, previous=None):
    """Refresh immutable imports; preserve reviewed K4 assessments and new rows.

Removed source rows are not silently deleted. They remain as stale-source errors
until the owning authority makes an explicit reviewed disposition.
"""
    registry = read(root / REGISTRY)
    matrix_locks, rows = source_rows(root)
    implementations = []
    for binding in registry["bindings"]:
        coordinate = next(n["source_ref"] for n in registry["nodes"] if n["id"] == binding["coordinate_id"])
        row_id = binding["identity"]
        refs = []
        for stratum, path, symbol in (("c", "c/src/m_tree.c", "ql_m_resolve"), ("rust", "crates/ql-mef/src/m_tree.rs", "resolve")):
            ident = f"{stratum}:{row_id}"
            refs.append(ident)
            implementations.append({"id": ident, "stratum": stratum, "kind": "structural-index",
                "path": path, "symbol": symbol, "disposition": "coordinate-bound",
                "coordinates": [coordinate], "relations": [], "rationale": "K2 aggregate index only; no computational-body claim.", "structure": []})
        rows.append({"id": row_id, "role": "recursive M structural index", "scope": coordinate,
            "coordinates": [coordinate], "source": None, "assessment": "k2-index", "bindings": refs,
            "dispositions": {"c": "bound", "rust": "bound"}, "invariants": ["index-is-not-computational-readiness"],
            "relations": [], "dependencies": []})
    registry_lock = lock(root, REGISTRY)
    registry_lock["revision"] = registry["registry_revision"]
    index_subjects = [r["id"] for r in rows if r["source"] is None]
    result = {"schema": "ql.m-ledger/v1", "ledger_revision": "", "registry": registry_lock,
        "matrices": matrix_locks, "assessments": {"unassessed": assessment(), "k2-index": assessment(True)},
        "rows": rows, "implementations": implementations,
        "evidence": [{"id": "k2-index-declaration", "kind": "source", "artifact": lock(root, REGISTRY),
            "registry_revision": registry["registry_revision"], "subjects": index_subjects,
            "strata": ["c", "rust"], "axes": ["coordinate", "relation"], "result": "present"}],
        "discrepancies": []}
    for number, source in enumerate(registry["parent_discrepancies"]):
        result["discrepancies"].append({"id": f"k2-parent-{number}", "subjects": [source["coordinate"]],
            "axis": "coordinate", "from_peer": "bimba", "to_peer": "c", "state": "open",
            "detail": json.dumps(source, ensure_ascii=False, sort_keys=True),
            "current_authority": {"peer": "c", "reference": "docs/KERNEL-RECURSIVE-M-REGISTRY.md", "reason": "K2 containment rule governs the executable tree; all Bimba parent assertions remain retained, not erased or promoted."},
            "proposal": None, "decision": None, "promotion": None,
            "history": [{"state": "open", "reference": "docs/KERNEL-RECURSIVE-M-REGISTRY.md"}]})
    if previous is not None:
        old_rows = {r["id"]: r for r in previous["rows"]}
        for row in rows:
            old = old_rows.pop(row["id"], None)
            if old is not None:
                for key in ("assessment", "bindings", "dispositions", "invariants", "relations", "dependencies"):
                    row[key] = old[key]
        rows.extend(old_rows.values())
        for key in ("assessments", "implementations", "evidence", "discrepancies"):
            result[key] = previous[key]
        # Never silently move evidence to a newer registry. The verifier exposes
        # stale evidence and requires a reviewed Return before promotion.
    result["ledger_revision"] = digest(canonical({k: v for k, v in result.items() if k != "ledger_revision"}))
    return result


def encode(document):
    # One row per line is diff-friendly without repeating deeply nested pretty
    # indentation thousands of times when K4 extends the census.
    return json.dumps(document, ensure_ascii=False, indent=2) + "\n"


def verify(root, ledger):
    """Checkout validation: schema, exact content locks, source inventory, evidence."""
    import jsonschema
    schema = read(root / "fixtures/kernel/m-ledger-v1.schema.json")
    jsonschema.Draft202012Validator.check_schema(schema)
    errors = list(jsonschema.Draft202012Validator(schema).iter_errors(ledger))
    if errors:
        raise ValueError(f"ledger schema: {list(errors[0].absolute_path)}: {errors[0].message}")
    expected = digest(canonical({k: v for k, v in ledger.items() if k != "ledger_revision"}))
    if ledger["ledger_revision"] != expected:
        raise ValueError("stale ledger_revision")
    for item in [ledger["registry"], *[lock for m in ledger["matrices"] for lock in (m["data"], m["rationale"])], *[e["artifact"] for e in ledger["evidence"]]]:
        if digest(safe_path(root, item["path"]).read_bytes()) != item["sha256"]:
            raise ValueError(f"stale content/evidence lock: {item['path']}")
    registry = read(root / REGISTRY)
    if ledger["registry"]["revision"] != registry["registry_revision"]:
        raise ValueError("stale registry revision")
    matrix_locks, sources = source_rows(root)
    if ledger["matrices"] != matrix_locks:
        raise ValueError("matrix inventory is incomplete or stale")
    actual = {r["id"]: r for r in ledger["rows"]}
    if len(actual) != len(ledger["rows"]):
        raise ValueError("duplicate ledger row")
    source_ids = set()
    for expected in sources:
        source_ids.add(expected["id"])
        row = actual.get(expected["id"])
        if row is None:
            raise ValueError(f"source capability missing from ledger: {expected['id']}")
        for field in ("role", "scope", "coordinates", "source"):
            if row[field] != expected[field]:
                raise ValueError(f"source import disagreement: {row['id']}.{field}; use a discrepancy, not a silent rewrite")
    for row in ledger["rows"]:
        if row["source"] is not None and row["id"] not in source_ids:
            raise ValueError(f"orphan/stale source capability: {row['id']}")
    for impl in ledger["implementations"]:
        text = safe_path(root, impl["path"]).read_text(encoding="utf-8")
        if not re.search(r"\b" + re.escape(impl["symbol"]) + r"\b", text):
            raise ValueError(f"missing implementation symbol: {impl['id']}")


def observations(registry, path, peer):
    """Compare actual K2 descriptor JSONL to the registry, never to a second tree.

A missing observer is not inferred to agree. This routine requires every public
node/relation/file/record/binding descriptor and rejects duplicate/extra records.
"""
    expected = {}
    for kind, collection, key in (("node", "nodes", "id"), ("relation", "relations", "id"), ("file", "files", None), ("record", "records", None), ("binding", "bindings", None)):
        for index, record in enumerate(registry[collection]):
            expected[(kind, record[key] if key else index)] = record
    actual, metadata, errors = {}, [], []
    counters = {k: 0 for k in ("file", "record", "binding")}
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        try:
            obj = json.loads(line)
            if not isinstance(obj, dict) or not isinstance(obj.get("kind"), str):
                raise ValueError("descriptor must be an object with a kind")
            kind = obj.pop("kind")
        except ValueError as error:
            errors.append({"code": "malformed-observation", "peer": peer, "detail": str(error)})
            continue
        if kind == "registry":
            metadata.append(obj)
            continue
        if kind in ("node", "relation"):
            ident = obj.get("id")
            if not isinstance(ident, str):
                errors.append({"code": "malformed-observation", "peer": peer, "detail": "node/relation id must be a string"})
                continue
        elif kind in counters:
            ident = counters[kind]
            counters[kind] += 1
        else:
            errors.append({"code": "unknown-descriptor-kind", "peer": peer, "subject": kind})
            continue
        key = (kind, ident)
        if key in actual:
            errors.append({"code": "duplicate-observation", "peer": peer, "subject": str(key)})
        actual[key] = obj
    if len(metadata) != 1 or metadata[0].get("registry_revision") != registry["registry_revision"]:
        errors.append({"code": "observation-revision", "peer": peer})
    for key, value in expected.items():
        if key not in actual:
            errors.append({"code": "missing-coordinate" if key[0] == "node" else "missing-descriptor", "peer": peer, "subject": str(key)})
        elif actual[key] != value:
            errors.append({"code": "structural-disagreement", "peer": peer, "subject": str(key)})
    for key in actual.keys() - expected.keys():
        errors.append({"code": "orphan-observation", "peer": peer, "subject": str(key)})
    return errors


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("refresh", "check", "observe"))
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--ledger", type=Path, default=LEDGER)
    parser.add_argument("--peer", choices=("bimba", "c", "rust", "cpp"))
    parser.add_argument("--input", type=Path)
    args = parser.parse_args(argv)
    root = args.root.resolve()
    path = args.ledger if args.ledger.is_absolute() else root / args.ledger
    try:
        if args.command == "refresh":
            doc = refresh(root, read(path) if path.exists() else None)
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(encode(doc), encoding="utf-8")
            print(f"{len(doc['rows'])} rows; {doc['ledger_revision']}")
        elif args.command == "check":
            verify(root, read(path))
            print("M ledger schema, matrix/source inventory and evidence locks: OK")
        else:
            if args.peer is None or args.input is None:
                parser.error("observe requires --peer and --input")
            errors = observations(read(root / REGISTRY), args.input, args.peer)
            print(json.dumps({"schema": "ql.m-observation-report/v1", "peer": args.peer, "findings": errors}, indent=2))
            return bool(errors)
        return 0
    except (ValueError, OSError, KeyError, IndexError) as error:
        print(f"m-ledger: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
