#!/usr/bin/env python3
"""Structural divergence between the K2 registry's seed pin and the live Bimba graph.

The K2 registry (fixtures/kernel/m-tree-v1.json) is compiled from the Bimba
datasets at the revision in data/epi-bimba-map/source-lock.json. Upstream has
since declared those datasets a deprecated seed archive and the live Neo4j
graph the authority. This tool measures the difference; it decides nothing.

Laws:

  * Live access is read-only and enforced by the server: every statement runs
    through the Query API with accessMode READ, so a write is refused by Neo4j,
    not merely avoided here. No endpoint or credential is embedded.
  * The capture and the report live under target/ and are never committed. They
    are evidence, not an index: the registry and the M ledger stay the only
    committed coordinate tree and discrepancy record.
  * Seed values are read by git blob id from the source lock, and every parsed
    record is checked against the registry's payload_sha256 before use.
  * Coordinate and property-key translation is upstream's own
    (generate-deep-regional-cypher.mjs, blob-pinned below), executed with node,
    and cross-checked against the census numeric-path join. No third mapping.
  * `ledger` appends discrepancies with the registry as current executable
    authority and no decision. An existing id with different detail is an error:
    a changed divergence is a new lifecycle entry, never a silent rewrite.
"""
from __future__ import annotations

import argparse
import base64
import datetime
import hashlib
import importlib.util
import json
import os
import re
import subprocess
import sys
import tempfile
import urllib.request
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "fixtures/kernel/m-tree-v1.json"
LEDGER = ROOT / "fixtures/kernel/m-ledger-v1.json"
SOURCE_LOCK = ROOT / "data/epi-bimba-map/source-lock.json"
WORKDIR = ROOT / "target/bimba-live"
DOC = "docs/kernel-rebuild/BIMBA-LIVE-DIVERGENCE.md"

# Upstream artifacts that interpret the live graph. Pinned by blob so a later
# upstream edit cannot silently change what "equal" means here.
UPSTREAM_REVISION = "c7872e96"
UPSTREAM = {
    "mapping": ("Idea/Bimba/Map/datasets/scripts/generate-deep-regional-cypher.mjs",
                "b0be97b6fd3a732bfeb5912ca615eefdd59221d0"),
    "recovered": ("Idea/Bimba/Map/datasets/recovery/recovered-bimba-properties.audit.json",
                  "cfa6a5fe444e976de108b8a0eef135d8de47282b"),
    "empty_repair": ("Idea/Bimba/Map/datasets/recovery/recovered-empty-property-values.audit.json",
                     "765bf3a88e44fc18862f55a5804112586585a9b1"),
}

STATEMENTS = {
    "identity": "CALL dbms.components() YIELD name, versions, edition RETURN name, versions, edition",
    "counts": "MATCH (n) WITH count(n) AS all_nodes "
              "MATCH (b:Bimba) WITH all_nodes, count(b) AS bimba_nodes "
              "MATCH ()-[r]->() WITH all_nodes, bimba_nodes, count(r) AS all_relations "
              "MATCH (:Bimba)-[s]->(:Bimba) "
              "RETURN all_nodes, bimba_nodes, all_relations, count(s) AS bimba_relations",
    # Embedding vectors are presence-only: they are derived, large, and not map content.
    "nodes": "MATCH (n:Bimba) RETURN n.coordinate AS coordinate, labels(n) AS labels, "
             "[k IN keys(n) WHERE NOT k STARTS WITH 'embedding' | [k, n[k]]] AS properties, "
             "[k IN keys(n) WHERE k STARTS WITH 'embedding'] AS derived_keys ORDER BY coordinate",
    "relations": "MATCH (a:Bimba)-[r]->(b:Bimba) RETURN a.coordinate AS source, type(r) AS type, "
                 "b.coordinate AS target, properties(r) AS properties "
                 "ORDER BY source, type, target",
}


def canonical(value) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def digest(value) -> str:
    return hashlib.sha256(canonical(value)).hexdigest()


def read(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def load_module(name: str, filename: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / "scripts" / filename)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


# ---------------------------------------------------------------------------
# Capture (read-only)
# ---------------------------------------------------------------------------


def query(endpoint: str, database: str, statement: str, auth: str | None):
    request = urllib.request.Request(
        endpoint.rstrip("/") + f"/db/{database}/query/v2",
        data=json.dumps({"statement": statement, "accessMode": "READ"}).encode(),
        headers={"Content-Type": "application/json", "Accept": "application/json",
                 **({"Authorization": f"Basic {auth}"} if auth else {})})
    with urllib.request.urlopen(request, timeout=300) as response:
        body = json.loads(response.read().decode())
    if body.get("errors"):
        raise SystemExit(f"live query refused: {body['errors']}")
    fields = body["data"]["fields"]
    return [dict(zip(fields, row)) for row in body["data"]["values"]], body.get("bookmarks", [])


def capture(args) -> int:
    password = os.environ.get(args.password_env, "") if args.password_env else ""
    auth = base64.b64encode(f"{args.user}:{password}".encode()).decode() if password else None
    started = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    rows, bookmarks = {}, []
    for name, statement in STATEMENTS.items():
        rows[name], bookmarks = query(args.endpoint, args.database, statement, auth)
    finished = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    nodes = [{"coordinate": r["coordinate"], "labels": sorted(r["labels"]),
              "properties": dict(sorted((k, v) for k, v in r["properties"])),
              "derived_keys": sorted(r["derived_keys"])} for r in rows["nodes"]]
    relations = rows["relations"]
    content = {"nodes": nodes, "relations": relations}
    receipt = {
        "schema": "ql.bimba-live-capture/v1",
        "standing": "read-only observation evidence; not a registry, not committed",
        "endpoint": args.endpoint, "database": args.database,
        "server": rows["identity"][0], "counts": rows["counts"][0],
        "window_utc": {"started": started, "finished": finished},
        "bookmarks": bookmarks,
        "content_sha256": digest(content),
        "captured": {"nodes": len(nodes), "relations": len(relations)},
    }
    write(args.workdir / "capture.json", content)
    write(args.workdir / "receipt.json", receipt)
    print(json.dumps(receipt, indent=2))
    return 0


# ---------------------------------------------------------------------------
# Seed side
# ---------------------------------------------------------------------------


def git(repo: Path, *argv: str) -> bytes:
    return subprocess.run(["git", "-C", str(repo), *argv], check=True, capture_output=True).stdout


def blob(repo: Path, revision: str, path: str, expected: str) -> bytes:
    actual = git(repo, "rev-parse", f"{revision}:{path}").decode().strip()
    if actual != expected:
        raise SystemExit(f"blob drift for {path} at {revision}: expected {expected}, got {actual}")
    return git(repo, "cat-file", "blob", expected)


def seed_records(repo: Path, registry: dict, lock: dict) -> dict[int, dict]:
    """Every registry record's exact seed payload, blob- and sha-verified against the registry."""
    compiler = load_module("epi_bimba_map_compiler", "compile-epi-bimba-map.py")
    by_file = defaultdict(list)
    for index, record in enumerate(registry["records"]):
        by_file[record["file"]].append(index)
    payloads = {}
    for file_index, indices in by_file.items():
        entry = registry["files"][file_index]
        text = blob(repo, lock["revision"], entry["path"], entry["git_blob"]).decode("utf-8-sig")
        parsed = json.loads(text, strict=False)
        kind = "nodes" if entry["record_class"] == "source-node-json" else "relations"
        records = dict(compiler.iter_records(parsed, kind))
        for index in indices:
            record = registry["records"][index]
            raw = records[record["record_index"]]
            # Relation records hash their relProperties; node records the whole record.
            hashed = (raw.get("relProperties") or {}) if kind == "relations" else raw
            if compiler.payload_digest(hashed) != record["payload_sha256"]:
                raise SystemExit(f"seed payload drift: {entry['path']}#{record['record_index']}")
            payloads[index] = {"path": entry["path"], "kind": kind, "record": raw}
    return payloads


def straggler_relations(repo: Path, lock: dict) -> list[dict]:
    """Straggler relations: per-node outgoing/incoming lists the K2 compiler does not read."""
    item = next(s for s in lock["required_sources"] if s["path"].endswith("parashakti-stragglers-relations.json"))
    parsed = json.loads(blob(repo, lock["revision"], item["path"], item["git_blob"]).decode("utf-8-sig"), strict=False)
    found = set()
    for node in parsed:
        for rel in node.get("outgoing") or []:
            found.add((node["coordinate"], rel["type"], rel["target_coord"]))
        for rel in node.get("incoming") or []:
            found.add((rel["source_coord"], rel["type"], node["coordinate"]))
    return [{"from_ref": a, "source_kind": k, "to_ref": b} for a, k, b in sorted(found)]


UPSTREAM_HELPER = r"""
import fs from "node:fs";
const [generator, refsPath] = process.argv.slice(2);
const m = await import(generator);
const refs = JSON.parse(fs.readFileSync(refsPath, "utf8"));
const coordinates = Object.fromEntries(refs.map((r) => [r, m.wrapContextFrames(m.mCoordinate(r))]));
process.stdout.write(JSON.stringify({
  mappings: m.mappings,
  registered: [...m.registeredTargets],
  string_lists: [...m.stringListTargets],
  coordinates,
}));
"""


def upstream_translation(repo: Path, refs: list[str]) -> dict:
    path, expected = UPSTREAM["mapping"]
    source = blob(repo, UPSTREAM_REVISION, path, expected)
    with tempfile.TemporaryDirectory() as tmp:
        generator = Path(tmp) / "generator.mjs"
        generator.write_bytes(source)
        helper = Path(tmp) / "helper.mjs"
        helper.write_text(UPSTREAM_HELPER, encoding="utf-8")
        refs_path = Path(tmp) / "refs.json"
        refs_path.write_text(json.dumps(refs), encoding="utf-8")
        out = subprocess.run(["node", str(helper), generator.as_uri(), str(refs_path)],
                             check=True, capture_output=True).stdout
    return json.loads(out)


def literal(value, target: str, string_lists: set[str]):
    """Python mirror of upstream cypherLiteral: the value the generator would write."""
    if value is None:
        return None
    if target in string_lists:
        items = value if isinstance(value, list) else [x.strip() for x in str(value).split(",")]
        return [str(x) for x in items if str(x).strip() != ""]
    if isinstance(value, bool) or isinstance(value, (int, float)):
        return value
    if isinstance(value, list):
        return [x if isinstance(x, (int, float)) else str(x) for x in value if x is not None and x != ""]
    if isinstance(value, dict):
        return json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    text = str(value)
    return None if text.strip() == "" else text


def expected_properties(props: dict, mappings: dict, string_lists: set[str]) -> dict:
    """Upstream assignmentsForRegion over every region: first present source key wins."""
    result = {}
    for region in mappings.values():
        for source_key, target in region.items():
            if source_key not in props or target in result:
                continue
            value = literal(props[source_key], target, string_lists)
            if value is not None and value != []:
                result[target] = {"value": value, "source_key": source_key}
    return result


def same(seed, live) -> bool:
    if isinstance(seed, float) or isinstance(live, float):
        try:
            return float(seed) == float(live)
        except (TypeError, ValueError):
            return False
    return seed == live


def snake(key: str) -> str:
    return re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", key).lower()


def key_variants(live_props: dict, family: str, suffix: str) -> list[str]:
    """Live keys of the same register family and suffix under another digit path.

    Upstream names this drift itself (m_prime loose form vs the sub-coordinate
    form, e.g. m_3_degree vs m_3_5_degree). A variant is reported as such; it
    is never treated as the mapped key.
    """
    pattern = re.compile(rf"^{re.escape(family)}_(?:[0-9]+_)+{re.escape(suffix)}$")
    return sorted(k for k in live_props if pattern.match(k))


# ---------------------------------------------------------------------------
# Compare
# ---------------------------------------------------------------------------


def compare(args) -> int:
    repo = args.epi_repo.resolve()
    registry, lock = read(REGISTRY), read(SOURCE_LOCK)
    if lock["revision"] != registry["source_revision"]:
        raise SystemExit("source lock and registry disagree on the pinned revision")
    live_doc, receipt = read(args.workdir / "capture.json"), read(args.workdir / "receipt.json")
    if digest(live_doc) != receipt["content_sha256"]:
        raise SystemExit("capture content does not match its receipt")
    live_nodes = {n["coordinate"]: n for n in live_doc["nodes"]}
    census = load_module("m_census", "m_census.py")

    refs = [n["source_ref"] for n in registry["nodes"] if n["source_ref"] != "M"]
    upstream = upstream_translation(repo, refs)
    to_live = upstream["coordinates"]
    string_lists = set(upstream["string_lists"])
    registered = set(upstream["registered"])

    # --- node set -------------------------------------------------------------
    join = census.join_live_to_registry({"nodes": live_nodes}, registry)
    census_pairs = {node["source_ref"]: live for live, node in join["joined"].items()}
    agree = sum(1 for ref, live in census_pairs.items() if to_live.get(ref) == live)
    paired = {ref: to_live[ref] for ref in refs if to_live[ref] in live_nodes}
    seed_only = sorted(ref for ref in refs if ref not in paired)
    live_m = [c for c in live_nodes if re.match(r"^M[0-5]", c)]
    claimed = set(paired.values())
    live_only = sorted(c for c in live_m if c not in claimed)
    outside = Counter(re.match(r"^[A-Za-z]*", c).group() or c[:1] for c in live_nodes if not re.match(r"^M[0-5]", c))

    def ancestor(live_ref: str) -> str:
        """Nearest registry coordinate that is a spelling-prefix of a live-only coordinate."""
        best = None
        for ref, live in paired.items():
            if live_ref.startswith(live) and live_ref[len(live):len(live) + 1] in "-./(" and (
                    best is None or len(live) > len(paired[best])):
                best = ref
        return best or "#" + live_ref[1]

    # --- relations ------------------------------------------------------------
    def live_ref(ref):
        return to_live.get(ref) if ref in to_live else None

    seed_triples = Counter()
    seed_refs = defaultdict(list)
    unresolved_seed = Counter()
    for rel in registry["relations"]:
        a, b = live_ref(rel["from_ref"]), live_ref(rel["to_ref"])
        if a is None or b is None:
            unresolved_seed[rel["source_kind"]] += 1
            continue
        key = (a, rel["source_kind"], b)
        seed_triples[key] += 1
        seed_refs[key].append(rel["relation_ref"])
    live_triples = Counter((r["source"], r["type"], r["target"]) for r in live_doc["relations"])
    kinds = sorted({k[1] for k in seed_triples} | {k[1] for k in live_triples})
    by_kind = {}
    for kind in kinds:
        s = {k for k in seed_triples if k[1] == kind}
        v = {k for k in live_triples if k[1] == kind}
        by_kind[kind] = {
            "seed_records": sum(seed_triples[k] for k in s), "seed_unique": len(s),
            "live": sum(live_triples[k] for k in v), "live_unique": len(v),
            "shared": len(s & v), "seed_only": len(s - v), "live_only": len(v - s),
        }
    seed_only_edges = sorted(k for k in seed_triples if k not in live_triples)
    live_only_edges = sorted(k for k in live_triples if k not in seed_triples)
    # A kind whose seed-only endpoint pairs reappear exactly under one live-only
    # kind is a rename of the relation type, reported as such.
    pairs = lambda edges, kind: {(a, b) for a, k, b in edges if k == kind}
    renames = {}
    for kind in sorted({k for _, k, _ in seed_only_edges}):
        mine = pairs(seed_only_edges, kind)
        for other in sorted({k for _, k, _ in live_only_edges} - set(renames.values())):
            if mine and mine == pairs(live_only_edges, other):
                renames[kind] = other
                break
    m_grammar = lambda ref: bool(re.match(r"^M[0-5]", ref))
    scope = Counter("m" if m_grammar(a) and m_grammar(b) else "mixed" if m_grammar(a) or m_grammar(b)
                    else "outside" for a, _, b in live_only_edges)
    stragglers = straggler_relations(repo, lock)
    straggler_live = [s for s in stragglers
                      if (to_live.get(s["from_ref"], s["from_ref"]), s["source_kind"],
                          to_live.get(s["to_ref"], s["to_ref"])) in live_triples]

    # --- property values --------------------------------------------------------
    payloads = seed_records(repo, registry, lock)
    mappings = upstream["mappings"]
    node_by_ref = {n["source_ref"]: n for n in registry["nodes"]}
    recovered_rows = json.loads(blob(repo, UPSTREAM_REVISION, *UPSTREAM["recovered"]))
    recovered = {(r["coordinate"], r["property"]) for r in recovered_rows}
    empty_audit = json.loads(blob(repo, UPSTREAM_REVISION, *UPSTREAM["empty_repair"]))
    residual = {(r["coordinate"], r["property"]) for r in empty_audit["residual"]}

    values = {"equal": [], "different": [], "variant_equal": [], "variant_different": [], "seed_only": [],
              "live_empty_seed_present": [], "seed_internal_conflict": [],
              "unmapped_absent": [], "unmapped_different": []}
    mapped_sources = {k for region in upstream["mappings"].values() for k in region}
    structural = {"coordinate", "bimbaCoordinate", "filteredProps", "id", "notionPageId", "uuid"}
    unmapped = defaultdict(Counter)
    per_target = defaultdict(Counter)
    for ref, live in sorted(paired.items()):
        expected = {}
        for index in node_by_ref[ref]["records"]:
            payload = payloads[index]
            record = payload["record"]
            deep = isinstance(record.get("filteredProps"), dict)
            props = record["filteredProps"] if deep else record
            live_props = live_nodes[live]["properties"]
            for key, value in props.items():
                if key in mapped_sources or key in structural or value in (None, "", [], {}):
                    continue
                candidates = [k for k in live_props if re.fullmatch(rf"[a-z]+_(?:[0-9]+_)*{re.escape(snake(key))}", k)]
                hit = next((k for k in candidates if same(literal(value, k, string_lists), live_props[k])), None)
                outcome = "live_suffix_equal" if hit else "live_suffix_different" if candidates else "live_absent"
                unmapped[key]["seed_values"] += 1
                unmapped[key][outcome] += 1
                if not hit:
                    values["unmapped_" + outcome.split("_")[-1]].append(
                        {"coordinate": ref, "live": live, "source_key": key, "file": payload["path"],
                         "live_candidates": candidates})
            for target, item in expected_properties(props, mappings, string_lists).items():
                item = {**item, "file": payload["path"], "deep": deep}
                prior = expected.get(target)
                if prior is None or (deep and not prior["deep"]):
                    if prior is not None and not same(prior["value"], item["value"]):
                        values["seed_internal_conflict"].append({"coordinate": ref, "target": target})
                    expected[target] = item
        props = live_nodes[live]["properties"]
        for target, item in expected.items():
            row = {"coordinate": ref, "live": live, "target": target, "source_key": item["source_key"],
                   "registered": target in registered, "recovered": (live, target) in recovered}
            if target not in props:
                form = re.match(r"^([a-z]+)_[0-9]+_(.+)$", target)
                variants = key_variants(props, *form.groups()) if form else []
                row["live_key_variants"] = variants
                if len(variants) == 1:
                    bucket = ("variant_equal" if same(item["value"], props[variants[0]])
                              else "variant_different")
                else:
                    bucket = "seed_only"
            elif props[target] == "" and item["value"] != "":
                bucket = "live_empty_seed_present"
            elif same(item["value"], props[target]):
                bucket = "equal"
            else:
                bucket = "different"
            per_target[target][bucket] += 1
            values[bucket].append(row)

    live_empty = sorted((n["coordinate"], k) for n in live_doc["nodes"]
                        for k, v in n["properties"].items() if v == "")
    empty_family = Counter(k.split("_", 1)[0] + "_*" for _, k in live_empty)
    live_keys = Counter(k for n in live_doc["nodes"] for k in n["properties"])
    mapped_targets = {t for region in mappings.values() for t in region.values()}
    live_prop_total = sum(live_keys.values())
    recovered_live = sum(1 for n in live_doc["nodes"] for k in n["properties"] if (n["coordinate"], k) in recovered)

    report = {
        "schema": "ql.bimba-live-divergence/v1",
        "standing": "measurement only; no side is chosen. Not committed; re-derive with this tool.",
        "seed": {"repository": lock["repository"], "revision": lock["revision"],
                 "dataset_tree": lock["dataset_tree"], "registry_revision": registry["registry_revision"]},
        "live": {k: receipt[k] for k in ("endpoint", "server", "counts", "window_utc", "bookmarks", "content_sha256")},
        "upstream_interpretation": {"revision": UPSTREAM_REVISION,
                                    **{k: {"path": p, "git_blob": b} for k, (p, b) in UPSTREAM.items()}},
        "nodes": {
            "registry": len(refs) + 1, "registry_coordinates": len(refs),
            "live_bimba": len(live_nodes), "live_m_grammar": len(live_m),
            "paired": paired, "seed_only": seed_only, "live_only": live_only,
            "live_outside_m_grammar": dict(outside),
            "join_cross_check": {"census_numeric_path_pairs": len(census_pairs),
                                 "agree_with_upstream_spelling": agree,
                                 "census_ambiguous": len(join["ambiguous"])},
            "live_only_by_registry_anchor": {k: sorted(v) for k, v in _group(live_only, ancestor).items()},
        },
        "relations": {
            "registry_records": len(registry["relations"]),
            "registry_unique_triples": len(seed_triples),
            "registry_unresolved_endpoints": dict(unresolved_seed),
            "live_bimba": sum(live_triples.values()), "live_unique_triples": len(live_triples),
            "shared_triples": len(set(seed_triples) & set(live_triples)),
            "kind_renames": {k: {"live_kind": v, "triples": len(pairs(seed_only_edges, k))} for k, v in renames.items()},
            "live_only_scope": dict(scope),
            "by_kind": by_kind,
            "seed_only": [list(k) for k in seed_only_edges],
            "live_only": [list(k) for k in live_only_edges],
            "seed_only_relation_refs": {"|".join(k): seed_refs[k] for k in seed_only_edges},
            "stragglers_skipped_by_k2_compiler": {"count": len(stragglers), "present_live": len(straggler_live),
                                                  "edges": stragglers},
        },
        "properties": {
            "live_total": live_prop_total,
            "live_keys": len(live_keys),
            "live_keys_without_seed_mapping": sorted(k for k in live_keys if k not in mapped_targets and k != "coordinate"),
            "recovered_from_txlog_present_live": recovered_live,
            "recovered_audit_rows": len(recovered_rows),
            "live_empty": {"count": len(live_empty), "by_family": dict(empty_family),
                           "matches_upstream_residual": len(set(live_empty) & residual),
                           "upstream_residual": len(residual)},
            "compared": {k: len(v) for k, v in values.items()},
            "seed_keys_without_mapping": {k: dict(v) for k, v in sorted(unmapped.items(),
                                                                        key=lambda kv: -kv[1]["seed_values"])},
            "per_target": {t: dict(c) for t, c in sorted(per_target.items())},
            "rows": {k: v for k, v in values.items() if k != "equal"},
        },
    }
    write(args.workdir / "divergence-report.json", report)
    summary = {"nodes": {k: (len(v) if isinstance(v, (list, dict)) and k != "join_cross_check" and k != "live_outside_m_grammar" else v)
                         for k, v in report["nodes"].items()
                         if k != "live_only_by_registry_anchor"},
               "relations": {k: v for k, v in report["relations"].items()
                             if k not in ("by_kind", "seed_only", "live_only", "seed_only_relation_refs",
                                          "stragglers_skipped_by_k2_compiler")},
               "properties": {k: v for k, v in report["properties"].items()
                              if k not in ("rows", "per_target", "live_keys_without_seed_mapping", "seed_keys_without_mapping")}}
    print(json.dumps(summary, indent=2, ensure_ascii=False))
    return 0


def _group(items, key):
    groups = defaultdict(list)
    for item in items:
        groups[key(item)].append(item)
    return groups


# ---------------------------------------------------------------------------
# Ledger
# ---------------------------------------------------------------------------


AUTHORITY = {
    "peer": "c",
    "reference": DOC,
    "reason": "The K2 registry stays the executable coordinate tree at its seed pin until the owner "
              "decides how QL follows the live authority; the live difference is retained, not merged.",
}


def discrepancy(ident, subjects, axis, detail):
    return {"id": ident, "subjects": sorted(set(subjects)), "axis": axis, "from_peer": "bimba", "to_peer": "c",
            "state": "open", "detail": json.dumps(detail, ensure_ascii=False, sort_keys=True),
            "current_authority": AUTHORITY, "proposal": None, "decision": None, "promotion": None,
            "history": [{"state": "open", "reference": DOC}]}


def branch(ref: str) -> str:
    """Owning branch for grouping: root plus first segment (#2-3, #4.4, #3-5)."""
    match = re.match(r"^(#[0-5](?:[-.][0-9]+)?)", ref)
    return match.group(1)


def ledger(args) -> int:
    report = read(args.workdir / "divergence-report.json")
    refs = {n["source_ref"] for n in read(REGISTRY)["nodes"]}
    roots = ["#0", "#1", "#2", "#3", "#4", "#5"]
    basis = {"live_content_sha256": report["live"]["content_sha256"], "live_window_utc": report["live"]["window_utc"],
             "seed_revision": report["seed"]["revision"], "registry_revision": report["seed"]["registry_revision"],
             "upstream_interpretation": report["upstream_interpretation"]["revision"],
             "report": "scripts/bimba-live-divergence.py compare (target/bimba-live/divergence-report.json)"}
    nodes, rels, props = report["nodes"], report["relations"], report["properties"]
    to_ref = {live: ref for ref, live in nodes["paired"].items()}
    for anchor, lives in nodes["live_only_by_registry_anchor"].items():
        to_ref.update({live: anchor for live in lives})
    out = []

    # --- nodes ----------------------------------------------------------------
    for ref in nodes["seed_only"]:
        out.append(discrepancy(f"bimba-live-node-seed-only:{ref}", [ref], "coordinate",
                               {**basis, "kind": "seed-only-coordinate"}))
    for anchor, lives in sorted(nodes["live_only_by_registry_anchor"].items()):
        out.append(discrepancy(f"bimba-live-node-live-only:{anchor}", [anchor], "coordinate",
                               {**basis, "kind": "live-only-coordinates", "live_coordinates": lives,
                                "note": "live :Bimba coordinates with no registry node; the subject is the nearest registry ancestor"}))
    if nodes["live_outside_m_grammar"]:
        out.append(discrepancy("bimba-live-node-outside-m-grammar", roots, "source",
                               {**basis, "kind": "live-coordinates-outside-m-grammar",
                                "by_prefix": nodes["live_outside_m_grammar"],
                                "note": "S/L/C/P/T and other lattices carry :Bimba live; the K2 registry grammar is #0..#5"}))

    # --- relations --------------------------------------------------------------
    renames = rels["kind_renames"]
    renamed_live = {v["live_kind"] for v in renames.values()}
    seed_refs = rels["seed_only_relation_refs"]
    for kind, rename in sorted(renames.items()):
        subjects = [r for key, ids in seed_refs.items() if key.split("|")[1] == kind for r in ids]
        out.append(discrepancy(f"bimba-live-relation-kind-renamed:{kind}", subjects, "relation",
                               {**basis, "kind": "relation-type-renamed", "seed_kind": kind, **rename,
                                "note": "identical endpoint pairs under a renamed type; the registry keeps the seed spelling"}))
    leftover = [t for t in rels["seed_only"] if t[1] not in renames]
    for a, kind, b in leftover:
        out.append(discrepancy(f"bimba-live-relation-seed-only:{a}|{kind}|{b}", seed_refs["|".join([a, kind, b])],
                               "relation", {**basis, "kind": "seed-only-relation", "triple": [a, kind, b]}))
    m_scoped, outside = defaultdict(list), Counter()
    for a, kind, b in rels["live_only"]:
        if kind in renamed_live:
            continue
        if a in to_ref or b in to_ref:
            m_scoped[kind].append([a, kind, b])
        else:
            outside[kind] += 1
    for kind, triples in sorted(m_scoped.items()):
        subjects = [to_ref[end] for t in triples for end in (t[0], t[2]) if end in to_ref]
        stats = rels["by_kind"][kind]
        out.append(discrepancy(f"bimba-live-relation-live-only:{kind}", subjects, "relation",
                               {**basis, "kind": "live-only-relations", "relation_type": kind,
                                "seed_unique": stats["seed_unique"], "live": stats["live"], "triples": triples}))
    if outside:
        out.append(discrepancy("bimba-live-relation-outside-m-grammar", roots, "relation",
                               {**basis, "kind": "live-only-relations-outside-m-grammar", "by_type": dict(sorted(outside.items())),
                                "note": "relations among S/L/C/prime lattices restored 2026-07-29; no registry endpoint"}))
    stragglers = rels["stragglers_skipped_by_k2_compiler"]
    if stragglers["count"]:
        out.append(discrepancy("bimba-live-relation-stragglers-uncompiled", ["#2-5-8", "#2-5-9"], "source",
                               {**basis, "kind": "seed-relations-skipped-by-k2-compiler", **stragglers,
                                "note": "parashakti-stragglers-relations.json holds per-node outgoing/incoming lists; "
                                        "scripts/compile-epi-bimba-map.py iter_records reads 0 relations from it, so these "
                                        "seed edges (present live) never reached the registry"}))

    # --- property key forms: equal values under a different live key ----------
    forms = defaultdict(list)
    for row in props["rows"]["variant_equal"]:
        forms[(row["target"], row["live_key_variants"][0])].append(row["coordinate"])
    for (target, variant), subjects in sorted(forms.items()):
        out.append(discrepancy(f"bimba-live-key-form:{target}->{variant}", subjects, "source",
                               {**basis, "kind": "property-key-form", "mapped_key": target, "live_key": variant,
                                "equal_values": len(subjects),
                                "note": "upstream's mapping writes the first key; live holds the same value under the second"}))

    # --- property values: per owning branch ----------------------------------
    per_branch = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    keep = {"different": ("target", "source_key", "registered", "recovered"),
            "variant_different": ("target", "source_key", "live_key_variants", "recovered"),
            "seed_only": ("target", "source_key", "registered"),
            "live_empty_seed_present": ("target", "source_key"),
            "unmapped_absent": ("source_key",),
            "unmapped_different": ("source_key", "live_candidates")}
    for bucket, fields in keep.items():
        for row in props["rows"][bucket]:
            per_branch[branch(row["coordinate"])][row["coordinate"]][bucket].append({k: row[k] for k in fields})
    for name, coordinates in sorted(per_branch.items()):
        counts = Counter(bucket for c in coordinates.values() for bucket, rows in c.items() for _ in rows)
        detail = {**basis, "kind": "property-values", "counts": dict(sorted(counts.items())),
                  "coordinates": {ref: {b: sorted(r, key=lambda x: json.dumps(x, sort_keys=True)) for b, r in sorted(c.items())}
                                  for ref, c in sorted(coordinates.items())},
                  "note": "different/variant_different: mapped seed value differs live; seed_only: mapped value absent live; "
                          "unmapped_absent: seed property no upstream mapping carries and no live key holds; "
                          "unmapped_different: a same-suffix live key holds a different value (suffix match is not a mapping)"}
        out.append(discrepancy(f"bimba-live-values:{name}", list(coordinates), "source", detail))
    out.append(discrepancy("bimba-live-empty-values", roots, "source",
                           {**basis, "kind": "live-empty-string-properties", **props["live_empty"],
                            "note": "empty strings on live :Bimba nodes; upstream names 828 as unrecoverable from transcripts"}))

    doc = read(LEDGER)
    existing = {d["id"]: d for d in doc["discrepancies"]}
    added = 0
    for item in out:
        bad = [s for s in item["subjects"] if s not in refs and not s.startswith("bimba:relation:")]
        if bad:
            raise SystemExit(f"{item['id']}: subjects outside the registry: {bad[:5]}")
        prior = existing.get(item["id"])
        if prior is None:
            doc["discrepancies"].append(item)
            added += 1
        elif prior["detail"] != item["detail"]:
            raise SystemExit(f"{item['id']} exists with different detail; record a transition, do not rewrite")
    doc["ledger_revision"] = digest({k: v for k, v in doc.items() if k != "ledger_revision"})
    LEDGER.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"{added} discrepancies added ({len(out)} derived); ledger {doc['ledger_revision']}")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="command", required=True)
    cap = sub.add_parser("capture", help="read-only live capture into target/bimba-live")
    cap.add_argument("--endpoint", required=True, help="Neo4j HTTP endpoint, e.g. http://host:7474")
    cap.add_argument("--database", default="neo4j")
    cap.add_argument("--user", default="neo4j")
    cap.add_argument("--password-env", help="environment variable holding the password (omit for auth none)")
    cmp_ = sub.add_parser("compare", help="compare the capture with the registry's seed pin")
    cmp_.add_argument("--epi-repo", type=Path, required=True, help="Epi-Logos-C-Experiments checkout")
    sub.add_parser("ledger", help="append the report's differences to the M ledger as open discrepancies")
    for p in sub.choices.values():
        p.add_argument("--workdir", type=Path, default=WORKDIR)
    args = parser.parse_args(argv)
    return {"capture": capture, "compare": compare, "ledger": ledger}[args.command](args)


if __name__ == "__main__":
    sys.exit(main())
