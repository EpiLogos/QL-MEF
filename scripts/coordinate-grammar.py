#!/usr/bin/env python3
"""The nesting threshold, enforced over the M registry and the Bimba map.

Law: after a position-4 segment the separator is "." — never "-". Position 4 is
the only position that nests (4.0 … 4.5), so every 4 that has a child is
followed by a dot: M4.0, M3-4.0-1, M2-4.3, M1-4.5-0.

  Sources: C-Experiments Idea/Bimba/Map/datasets/paramasiva-deep/
  Quaternal_Logic_Lived_Topology.md §IV ("The dot after 4 is constitutional
  law—only position 4 permits this notation") and Idea/Bimba/Map/M1/M1-4/M1-4.md
  (q_1_fractal_coordinate_system, q_3_nested_quaternity_with_dot). Summary:
  skills/ql-law/SKILL.md.

A dash after a 4 is non-canonical. The registry spells what the map spells (the
map is authority, ruling D10), so a non-canonical spelling in the registry is a
map defect. It is carried as a ledger discrepancy until the map is corrected,
never silently respelled here.

    coordinate-grammar.py check        every registry violation is recorded; no record is stale
    coordinate-grammar.py record       merge missing k2-grammar discrepancies into the ledger
    coordinate-grammar.py map [--cypher PATH]
                                       report the live map read (target/bimba-map/map.json);
                                       optionally write the correcting migration
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import re
import sys
from collections import OrderedDict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = Path("fixtures/kernel/m-tree-v1.json")
LEDGER = Path("fixtures/kernel/m-ledger-v1.json")
MAP_READ = Path("target/bimba-map/map.json")
MIGRATION = Path("migration/bimba-map/2026-09-29-nesting-threshold.cypher")
PREFIX = "k2-grammar-nesting-threshold:"
LAW = "skills/ql-law/SKILL.md"
SOURCE = ("Epi-Logos-C-Experiments Idea/Bimba/Map/datasets/paramasiva-deep/"
          "Quaternal_Logic_Lived_Topology.md §IV")

# A segment is a whole digit run: it starts after the prefix (M or #) or after a
# separator or an opening bracket. A segment that is exactly "4" and is followed
# by "-" breaks the threshold.
FOUR_DASH = re.compile(r"(?:(?<=^M)|(?<=^#)|(?<=[-./(]))4-")


def violations(coordinate: str) -> list[int]:
    """Offsets of every dash that follows a position-4 segment."""
    return [m.end() - 1 for m in FOUR_DASH.finditer(coordinate)]


def canonical(coordinate: str) -> str:
    """The spelling the law requires: each dash after a 4 becomes a dot."""
    return FOUR_DASH.sub("4.", coordinate)


def threshold_parent(coordinate: str) -> str:
    """The coordinate up to and including the first offending 4."""
    return coordinate[: violations(coordinate)[0]]


def map_spelling(node: dict) -> str:
    """A registry node's exact map spelling: the bracketed alias, else M for #."""
    bracketed = [a for a in node["aliases"] if "(" in a]
    return bracketed[0] if bracketed else "M" + node["source_ref"][1:]


def registry_findings(registry: dict) -> "OrderedDict[str, list[dict]]":
    """Non-canonical registry nodes grouped by the parent 4 that breaks the law."""
    spellings = {map_spelling(n) for n in registry["nodes"]}
    groups: "OrderedDict[str, list[dict]]" = OrderedDict()
    for node in sorted(registry["nodes"], key=lambda n: n["source_ref"]):
        if not violations(node["source_ref"]):
            continue
        spelled = map_spelling(node)
        target = canonical(spelled)
        groups.setdefault(threshold_parent(node["source_ref"]), []).append({
            "registry": node["source_ref"], "map": spelled, "canonical": target,
            "kind": "duplicate" if target in spellings else "respell"})
    return groups


def discrepancy(parent: str, items: list[dict]) -> dict:
    duplicates = sum(1 for i in items if i["kind"] == "duplicate")
    detail = {"code": "nesting-threshold", "law": "after a position-4 segment the separator is '.'",
              "source": SOURCE, "parent": parent, "duplicates": duplicates,
              "respellings": len(items) - duplicates, "coordinates": items}
    change = (f"Correct the map: {len(items) - duplicates} coordinate(s) respelled to the dotted form"
              + (f", {duplicates} dash-spelled duplicate(s) of an existing dotted node removed" if duplicates else "")
              + ". Prior spelling kept on the node as c_2_prior_coordinate; relation coordinate properties follow.")
    return {"id": PREFIX + parent, "subjects": [i["registry"] for i in items], "axis": "coordinate",
            "from_peer": "bimba", "to_peer": "c", "state": "proposed",
            "detail": json.dumps(detail, ensure_ascii=False, sort_keys=True),
            "current_authority": {"peer": "bimba", "reference": "docs/kernel-rebuild/BIMBA-MAP-SOURCE.md",
                                  "reason": "The map is authority (ruling D10); the registry keeps the map's spelling until the owner applies the map correction."},
            "proposal": {"target_peer": "bimba", "change": change, "evidence": [str(MIGRATION)]},
            "decision": None, "promotion": None,
            "history": [{"state": "open", "reference": LAW}, {"state": "proposed", "reference": str(MIGRATION)}]}


def read(path: Path) -> dict:
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def ledger_tool():
    spec = importlib.util.spec_from_file_location("m_ledger_tool", ROOT / "scripts/m-ledger.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def check(registry: dict, ledger: dict) -> list[str]:
    """Errors: an unrecorded violation, or a live record whose subject is gone."""
    live = {d["id"]: d for d in ledger["discrepancies"] if d["id"].startswith(PREFIX)}
    covered = {s for d in live.values() if d["state"] != "rejected" for s in d["subjects"]}
    refs = {n["source_ref"] for n in registry["nodes"]}
    errors = []
    for parent, items in registry_findings(registry).items():
        for item in items:
            if item["registry"] not in covered:
                errors.append(f"unrecorded nesting-threshold violation {item['registry']} ({item['map']} -> "
                              f"{item['canonical']}); record it: python3 scripts/coordinate-grammar.py record")
    for ident, d in live.items():
        if d["state"] in ("open", "proposed", "accepted"):
            gone = [s for s in d["subjects"] if s not in refs]
            if gone:
                errors.append(f"{ident} is {d['state']} but {len(gone)} subject(s) left the registry "
                              f"(e.g. {gone[0]}); review it to applied with its evidence")
    return errors


def record(registry: dict, ledger: dict) -> tuple[dict, int]:
    """Add each missing group record; a reviewed record is never replaced."""
    present = {d["id"] for d in ledger["discrepancies"]}
    added = 0
    for parent, items in registry_findings(registry).items():
        entry = discrepancy(parent, items)
        if entry["id"] not in present:
            ledger["discrepancies"].append(entry)
            added += 1
    return ledger, added


# ---- the live map ---------------------------------------------------------

M_COORDINATE = re.compile(r"^M[0-5](?:[-./()0-9]*[0-9)])?$")
TOKEN = re.compile(r"(?<![\w'′])M[0-5](?:[-./][0-9]+|[-.]\([0-9/.\-]+\))*(?![\w'′])")


def map_findings(value: dict) -> dict:
    nodes = value["nodes"]
    coords = sorted(c for c in nodes if M_COORDINATE.fullmatch(c))
    bad = [c for c in coords if violations(c)]
    out = {"read": value["content_sha256"], "read_at": value["read_at"], "m_coordinates": len(coords),
           "duplicates": [], "respellings": [], "collisions": []}
    for c in bad:
        target = canonical(c)
        if target in nodes:
            out["duplicates"].append({"map": c, "canonical": target,
                                      "labels": nodes[c]["labels"], "canonical_labels": nodes[target]["labels"],
                                      "relations": sum(1 for s, _, t, _ in value["relations"] if c in (s, t))})
        else:
            out["respellings"].append({"map": c, "canonical": target})
    targets = [r["canonical"] for r in out["respellings"]]
    out["collisions"] = sorted({t for t in targets if targets.count(t) > 1})
    return out


def prose_rewrites(value: dict, renames: dict[str, str]) -> tuple[list, list]:
    """Exact coordinate tokens in node and relation string properties that change."""
    skip = {"coordinate", "m_2_1_prior_coordinate", "c_2_prior_coordinate"}
    node_edits, rel_edits = [], []

    def fix(text):
        return TOKEN.sub(lambda m: renames.get(m.group(0), m.group(0)), text)

    for c, v in value["nodes"].items():
        for k, val in v["properties"].items():
            if k in skip or not isinstance(val, str):
                continue
            new = fix(val)
            if new != val:
                node_edits.append((c, k))
    for s, t, d, props in value["relations"]:
        for k, val in (props or {}).items():
            if isinstance(val, str) and fix(val) != val:
                rel_edits.append((s, t, d, k))
    return node_edits, rel_edits


def cypher_string(text: str) -> str:
    return "'" + text.replace("\\", "\\\\").replace("'", "\\'") + "'"


def migration(value: dict, found: dict) -> str:
    renames = {r["map"]: r["canonical"] for r in found["respellings"]}
    renames.update({d["map"]: d["canonical"] for d in found["duplicates"]})
    node_edits, rel_edits = prose_rewrites(value, renames)
    removable = [d for d in found["duplicates"] if d["relations"] == 0]
    blocked = [d for d in found["duplicates"] if d["relations"] != 0]
    if blocked or found["collisions"]:
        raise SystemExit(f"refusing to write a migration: duplicates with relations {blocked} "
                         f"or respelling collisions {found['collisions']} need an explicit merge design")
    lines = [
        "// The nesting threshold: after a position-4 segment the separator is '.', never '-'.",
        "// Law: C-Experiments Idea/Bimba/Map/datasets/paramasiva-deep/Quaternal_Logic_Lived_Topology.md §IV;",
        "// Idea/Bimba/Map/M1/M1-4/M1-4.md. Summary and ruling trail: skills/ql-law/SKILL.md.",
        f"// Generated by scripts/coordinate-grammar.py map --cypher from map read {found['read']}",
        f"// ({found['read_at']}). {len(found['respellings'])} respellings, {len(removable)} relationless",
        "// dash-spelled duplicates of existing dotted nodes, "
        f"{len(node_edits)} node and {len(rel_edits)} relation string properties",
        "// whose exact coordinate tokens follow the respelling. The prior spelling is kept as",
        "// c_2_prior_coordinate. NOT APPLIED: map writes need the owner's explicit OK.",
        "//   dry run:  python3 migration/bimba-map/run.py " + str(MIGRATION),
        "//   apply:    python3 migration/bimba-map/run.py " + str(MIGRATION) + " --commit",
        "// After applying: scripts/refresh-from-bimba-map.sh, then review the k2-grammar ledger records to applied.",
        "",
        "// @probe dash-after-4 M coordinates (expect "
        f"{len(found['respellings']) + len(found['duplicates'])} before apply, 0 after)",
        "MATCH (n:Bimba) WHERE n.coordinate =~ '^M(4|.*[-.(/]4)-.*' RETURN count(n) AS n;",
        "",
        "// @probe dotted twins of the duplicates exist (expect "
        f"{len(removable)})",
        "MATCH (n:Bimba) WHERE n.coordinate IN [" + ", ".join(cypher_string(d["canonical"]) for d in removable)
        + "] RETURN count(n) AS n;",
        "",
    ]
    if removable:
        lines += [f"// @apply remove {len(removable)} relationless dash-spelled duplicates (their dotted twins carry the content and relations)"]
        for d in removable:
            lines.append(f"MATCH (n:Bimba {{coordinate:{cypher_string(d['map'])}}}) WHERE NOT (n)--() DELETE n;")
        lines.append("")
    lines.append(f"// @apply respell {len(found['respellings'])} coordinates to the dotted form")
    for r in found["respellings"]:
        lines.append(f"MATCH (n:Bimba {{coordinate:{cypher_string(r['map'])}}}) "
                     f"SET n.c_2_prior_coordinate = n.coordinate, n.coordinate = {cypher_string(r['canonical'])};")
    lines.append("")
    # Property rewrites are generated as exact whole-value replacements computed here,
    # so the migration is reviewable text rather than a regex run inside the database.
    by_node: dict = {}
    for c, k in node_edits:
        by_node.setdefault(c, []).append(k)
    if by_node:
        lines.append(f"// @apply follow the respelling in {len(node_edits)} node string properties")
        for c, keys in sorted(by_node.items()):
            props = value["nodes"][c]["properties"]
            sets = ", ".join(f"n.{k} = {cypher_string(TOKEN.sub(lambda m: renames.get(m.group(0), m.group(0)), props[k]))}"
                             for k in sorted(keys))
            lines.append(f"MATCH (n:Bimba {{coordinate:{cypher_string(renames.get(c, c))}}}) SET {sets};")
        lines.append("")
    if rel_edits:
        lines.append(f"// @apply follow the respelling in {len(rel_edits)} relation string properties")
        for s, t, d, k in rel_edits:
            props = next(p for (a, b, e, p) in value["relations"] if (a, b, e) == (s, t, d))
            new = TOKEN.sub(lambda m: renames.get(m.group(0), m.group(0)), props[k])
            lines.append(f"MATCH (a:Bimba {{coordinate:{cypher_string(renames.get(s, s))}}})-[r:`{t}`]->"
                         f"(b:Bimba {{coordinate:{cypher_string(renames.get(d, d))}}}) "
                         f"WHERE r.{k} = {cypher_string(props[k])} SET r.{k} = {cypher_string(new)};")
        lines.append("")
    lines += ["// @report dash-after-4 M coordinates remaining",
              "MATCH (n:Bimba) WHERE n.coordinate =~ '^M(4|.*[-.(/]4)-.*' RETURN count(n) AS n;",
              "// @report nodes carrying c_2_prior_coordinate",
              "MATCH (n:Bimba) WHERE n.c_2_prior_coordinate IS NOT NULL RETURN count(n) AS n;", ""]
    return "\n".join(lines)


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("command", choices=["check", "record", "map"])
    parser.add_argument("--cypher", type=Path, help="with map: write the correcting migration here")
    args = parser.parse_args(argv)
    if args.command == "map":
        value = read(MAP_READ)
        found = map_findings(value)
        print(json.dumps({k: (v if not isinstance(v, list) else len(v)) for k, v in found.items()}, indent=2))
        for d in found["duplicates"]:
            print(f"duplicate  {d['map']} -> {d['canonical']} ({d['relations']} relations; twin labels {d['canonical_labels']})")
        if args.cypher:
            (ROOT / args.cypher).write_text(migration(value, found), encoding="utf-8")
            print(f"wrote {args.cypher}")
        return 0
    registry, ledger = read(REGISTRY), read(LEDGER)
    if args.command == "record":
        ledger, added = record(registry, ledger)
        tool = ledger_tool()
        result = tool.refresh(ROOT, ledger)
        (ROOT / LEDGER).write_text(tool.encode(result), encoding="utf-8")
        print(f"recorded {added} nesting-threshold discrepancy group(s); ledger {result['ledger_revision']}")
        return 0
    errors = check(registry, ledger)
    for e in errors:
        print("error:", e, file=sys.stderr)
    groups = registry_findings(registry)
    count = sum(len(v) for v in groups.values())
    if not errors:
        print(f"nesting threshold: {count} non-canonical registry coordinate(s) in {len(groups)} group(s), all recorded: OK")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
