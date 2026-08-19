#!/usr/bin/env python3
"""Compile the live Idea/Bimba/Map source pool into deterministic QL-MEF indexes.

This compiler intentionally keeps three things apart:

1. source structural/semantic records from Epi-Logos-C-Experiments;
2. QL-MEF's normalized coordinate/relation representation;
3. current implementation bindings/evidence such as the hand-authored Nara floor.

It does not rewrite Bimba source properties into QL canon and it does not infer Bimba
relations from module calls.  The source repository must be a real Git checkout so
repository revision and Git blob identity remain part of every generated record.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import re
import subprocess
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

SCHEMA = "ql.epi-bimba-map-compilation/v1"
SOURCE_REPOSITORY = "EpiLogos/Epi-Logos-C-Experiments"
MAP_ROOT = Path("Idea/Bimba/Map")
COORD_RE = re.compile(r"^#([0-5])((?:[.-][0-9]+)*)$")
SEGMENT_RE = re.compile(r"([.-])([0-9]+)")
COORD_ANY_RE = re.compile(r"#[0-5](?:[.-][0-9]+)*")
COSMIC_ROOTS = {1, 2, 3}
COSMIC_RELATION_WORDS = (
    "HARMON",
    "MATHEME",
    "SPANDA",
    "RESON",
    "VIBR",
    "CORRESP",
    "TEMPOR",
    "TIME",
    "CLOCK",
    "TRANSCRI",
    "SYMBOL",
    "RHYTHM",
    "CYCLE",
)


@dataclass(frozen=True)
class ParsedCoordinate:
    source_ref: str
    root: int
    path: tuple[int, ...]
    separators: tuple[str, ...]

    @property
    def parent(self) -> str:
        if not self.path:
            return "#"
        value = f"#{self.root}"
        for sep, segment in zip(self.separators[:-1], self.path[:-1]):
            value += f"{sep}{segment}"
        return value

    @property
    def path_key(self) -> tuple[int, tuple[int, ...]]:
        return self.root, self.path

    @property
    def depth(self) -> int:
        return len(self.path)


def parse_coordinate(value: Any) -> ParsedCoordinate | None:
    if not isinstance(value, str):
        return None
    value = value.strip()
    match = COORD_RE.fullmatch(value)
    if not match:
        return None
    root = int(match.group(1))
    segments = SEGMENT_RE.findall(match.group(2))
    return ParsedCoordinate(
        source_ref=value,
        root=root,
        path=tuple(int(segment) for _, segment in segments),
        separators=tuple(separator for separator, _ in segments),
    )


def run_git(repo: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", "-C", str(repo), *args],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return result.stdout.strip()


def sha256_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def canonical_json(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def payload_digest(value: Any) -> str:
    return sha256_bytes(canonical_json(value).encode("utf-8"))


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8-sig"))


def git_blob_map(repo: Path, revision: str, prefix: Path) -> dict[str, str]:
    output = run_git(repo, "ls-tree", "-r", revision, "--", prefix.as_posix())
    result: dict[str, str] = {}
    for line in output.splitlines():
        if not line.strip():
            continue
        meta, path = line.split("\t", 1)
        _mode, kind, blob = meta.split(" ", 2)
        if kind == "blob":
            result[path] = blob
    return result


def classify_source(path: Path) -> str:
    name = path.name.lower()
    parts = {part.lower() for part in path.parts}
    if path.suffix.lower() == ".md":
        return "structural-markdown"
    if path.suffix.lower() == ".cypher":
        return "generated-or-migration-cypher"
    if path.suffix.lower() in {".py", ".mjs", ".js", ".sh"}:
        return "helper-or-migration-code"
    if path.suffix.lower() == ".json":
        if "relation" in name or "relations" in parts:
            return "source-relation-json"
        if "node" in name or "nodes" in parts:
            return "source-node-json"
        return "machine-json"
    if path.suffix.lower() in {".csv", ".tsv"}:
        return "machine-tabular-export"
    if path.suffix.lower() in {".xlsx", ".xls"}:
        return "generated-tabular-export"
    if path.name == ".gitkeep":
        return "placeholder"
    return "other-source-record"


def iter_records(value: Any, preferred_key: str | None = None) -> Iterable[tuple[int, Any]]:
    if isinstance(value, list):
        yield from enumerate(value)
        return
    if isinstance(value, dict):
        if preferred_key and isinstance(value.get(preferred_key), list):
            yield from enumerate(value[preferred_key])
            return
        for key in ("records", "nodes", "relations", "data"):
            if isinstance(value.get(key), list):
                yield from enumerate(value[key])
                return
        yield 0, value


def record_coordinate(record: Any) -> str | None:
    if not isinstance(record, dict):
        return None
    direct = record.get("coordinate")
    if isinstance(direct, str):
        return direct.strip()
    filtered = record.get("filteredProps")
    if isinstance(filtered, dict):
        for key in ("coordinate", "bimbaCoordinate"):
            value = filtered.get(key)
            if isinstance(value, str):
                return value.strip()
    return None


def record_name(record: Any) -> str | None:
    if not isinstance(record, dict):
        return None
    for container in (record, record.get("filteredProps") if isinstance(record.get("filteredProps"), dict) else {}):
        for key in ("name", "title", "label", "primaryDesignation"):
            value = container.get(key)
            if isinstance(value, str) and value.strip():
                return value.strip()
    return None


def explicit_aliases(record: Any, source_ref: str) -> set[str]:
    aliases: set[str] = set()
    if not isinstance(record, dict):
        return aliases
    containers = [record]
    if isinstance(record.get("filteredProps"), dict):
        containers.append(record["filteredProps"])
    for container in containers:
        for key in (
            "bimbaCoordinate",
            "bimba_coordinate",
            "sourceCoordinate",
            "source_coordinate",
            "coordinateAlias",
            "coordinate_alias",
        ):
            value = container.get(key)
            if isinstance(value, str) and value.strip() != source_ref and parse_coordinate(value.strip()):
                aliases.add(value.strip())
        for key in ("aliases", "coordinateAliases", "alternateCoordinates"):
            values = container.get(key)
            if isinstance(values, list):
                for value in values:
                    if isinstance(value, str) and value.strip() != source_ref and parse_coordinate(value.strip()):
                        aliases.add(value.strip())
    return aliases


def property_keys(record: Any) -> list[str]:
    if not isinstance(record, dict):
        return []
    keys = set(record.keys())
    filtered = record.get("filteredProps")
    if isinstance(filtered, dict):
        keys.update(f"filteredProps.{key}" for key in filtered)
    return sorted(keys)


def endpoint_root(value: str | None) -> int | None:
    parsed = parse_coordinate(value)
    return parsed.root if parsed else None


def stable_relation_id(path: str, index: int, record: dict[str, Any]) -> str:
    seed = f"{path}\0{index}\0{canonical_json(record)}".encode("utf-8")
    return f"bimba:relation:{hashlib.sha256(seed).hexdigest()[:24]}"


def load_lock(lock_path: Path) -> dict[str, Any]:
    return json.loads(lock_path.read_text(encoding="utf-8"))


def verify_source_lock(repo: Path, revision: str, blobs: dict[str, str], lock: dict[str, Any]) -> None:
    expected_revision = lock["revision"]
    if revision != expected_revision:
        raise SystemExit(f"source revision mismatch: expected {expected_revision}, got {revision}")
    for item in lock.get("required_sources", []):
        path = item["path"]
        expected_blob = item["git_blob"]
        actual_blob = blobs.get(path)
        if actual_blob != expected_blob:
            raise SystemExit(
                f"source blob mismatch for {path}: expected {expected_blob}, got {actual_blob}"
            )


def compile_map(source_repo: Path, out: Path, lock: dict[str, Any]) -> dict[str, Any]:
    revision = run_git(source_repo, "rev-parse", "HEAD")
    map_path = source_repo / MAP_ROOT
    if not map_path.is_dir():
        raise SystemExit(f"missing live Bimba Map at {map_path}")

    blobs = git_blob_map(source_repo, revision, MAP_ROOT)
    verify_source_lock(source_repo, revision, blobs, lock)

    source_inventory: list[dict[str, Any]] = []
    coordinate_records: list[dict[str, Any]] = []
    relation_records: list[dict[str, Any]] = []
    parse_failures: list[dict[str, Any]] = []
    json_failures: list[dict[str, Any]] = []

    for absolute in sorted(path for path in map_path.rglob("*") if path.is_file()):
        relative = absolute.relative_to(source_repo).as_posix()
        content = absolute.read_bytes()
        source_class = classify_source(absolute.relative_to(map_path))
        inventory_record = {
            "path": relative,
            "git_blob": blobs.get(relative, ""),
            "sha256": sha256_bytes(content),
            "bytes": len(content),
            "record_class": source_class,
        }
        source_inventory.append(inventory_record)

        if absolute.suffix.lower() != ".json":
            continue
        try:
            parsed_json = read_json(absolute)
        except Exception as error:  # inventory must retain malformed/generated residues
            json_failures.append({"path": relative, "error": str(error)})
            continue

        if source_class == "source-node-json":
            for record_index, record in iter_records(parsed_json, "nodes"):
                coordinate_text = record_coordinate(record)
                if coordinate_text is None:
                    continue
                coordinate = parse_coordinate(coordinate_text)
                if coordinate is None:
                    parse_failures.append(
                        {
                            "path": relative,
                            "record_index": record_index,
                            "source_ref": coordinate_text,
                            "class": "node-coordinate",
                        }
                    )
                    continue
                coordinate_records.append(
                    {
                        "source_ref": coordinate.source_ref,
                        "root": coordinate.root,
                        "path": list(coordinate.path),
                        "separators": list(coordinate.separators),
                        "lexical_parent_source_ref": coordinate.parent,
                        "aliases": sorted(explicit_aliases(record, coordinate.source_ref)),
                        "name": record_name(record),
                        "source_path": relative,
                        "source_git_blob": blobs.get(relative, ""),
                        "source_sha256": inventory_record["sha256"],
                        "record_index": record_index,
                        "payload_sha256": payload_digest(record),
                        "property_keys": property_keys(record),
                    }
                )

        if source_class == "source-relation-json":
            for record_index, record in iter_records(parsed_json, "relations"):
                if not isinstance(record, dict):
                    continue
                from_ref = record.get("source")
                to_ref = record.get("target")
                kind = record.get("relType") or record.get("type") or record.get("kind")
                if not isinstance(kind, str):
                    continue
                if from_ref is not None and not isinstance(from_ref, str):
                    from_ref = str(from_ref)
                if to_ref is not None and not isinstance(to_ref, str):
                    to_ref = str(to_ref)
                from_root = endpoint_root(from_ref)
                to_root = endpoint_root(to_ref)
                properties = record.get("relProperties")
                if properties is None:
                    properties = {}
                relation_records.append(
                    {
                        "relation_ref": stable_relation_id(relative, record_index, record),
                        "source_kind": kind,
                        "from_ref": from_ref,
                        "to_ref": to_ref,
                        "orientation": "directed",
                        "cross_m": from_root is not None and to_root is not None and from_root != to_root,
                        "from_root": from_root,
                        "to_root": to_root,
                        "source_path": relative,
                        "source_git_blob": blobs.get(relative, ""),
                        "source_sha256": inventory_record["sha256"],
                        "record_index": record_index,
                        "payload_sha256": payload_digest(properties),
                        "property_keys": sorted(properties.keys()) if isinstance(properties, dict) else [],
                    }
                )

    by_source: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for record in coordinate_records:
        by_source[record["source_ref"]].append(record)

    source_parent_relations: dict[str, set[str]] = defaultdict(set)
    for relation in relation_records:
        if relation["source_kind"] == "HAS_INTERNAL_COMPONENT" and relation["from_ref"] and relation["to_ref"]:
            source_parent_relations[relation["to_ref"]].add(relation["from_ref"])

    coordinate_index: list[dict[str, Any]] = []
    path_spellings: dict[tuple[int, tuple[int, ...]], set[str]] = defaultdict(set)
    for source_ref, records in sorted(by_source.items()):
        first = records[0]
        path_spellings[(first["root"], tuple(first["path"]))].add(source_ref)
        aliases = sorted({alias for record in records for alias in record["aliases"]})
        source_parents = sorted(source_parent_relations.get(source_ref, set()))
        coordinate_index.append(
            {
                "source_ref": source_ref,
                "root": first["root"],
                "path": first["path"],
                "separators": first["separators"],
                "lexical_parent_source_ref": first["lexical_parent_source_ref"],
                "source_parent_refs": source_parents,
                "aliases": aliases,
                "names": sorted({record["name"] for record in records if record["name"]}),
                "record_count": len(records),
                "records": [
                    {
                        "source_path": record["source_path"],
                        "source_git_blob": record["source_git_blob"],
                        "source_sha256": record["source_sha256"],
                        "record_index": record["record_index"],
                        "payload_sha256": record["payload_sha256"],
                        "property_keys": record["property_keys"],
                    }
                    for record in records
                ],
            }
        )

    path_collisions = [
        {
            "root": key[0],
            "path": list(key[1]),
            "source_refs": sorted(values),
            "status": "unresolved-alternate-notation" if len(values) > 1 else "unique",
        }
        for key, values in sorted(path_spellings.items())
        if len(values) > 1
    ]

    coordinate_set = set(by_source)
    relation_kinds = Counter(record["source_kind"] for record in relation_records)
    root_coordinate_counts = Counter(record["root"] for record in coordinate_index)
    root_record_counts = Counter(record["root"] for record in coordinate_records)
    root_relation_counts = Counter(
        relation["from_root"] for relation in relation_records if relation["from_root"] is not None
    )

    unresolved_relation_endpoints = []
    for relation in relation_records:
        missing = []
        for key in ("from_ref", "to_ref"):
            endpoint = relation[key]
            if endpoint is None:
                missing.append({"endpoint": key, "reason": "null"})
            elif parse_coordinate(endpoint) and endpoint not in coordinate_set:
                missing.append({"endpoint": key, "reason": "coordinate-not-in-index", "ref": endpoint})
            elif endpoint == "#":
                missing.append({"endpoint": key, "reason": "meta-source-ref", "ref": endpoint})
        if missing:
            unresolved_relation_endpoints.append(
                {"relation_ref": relation["relation_ref"], "source_kind": relation["source_kind"], "endpoints": missing}
            )

    inventory = {
        "schema": SCHEMA,
        "repository": SOURCE_REPOSITORY,
        "revision": revision,
        "map_root": MAP_ROOT.as_posix(),
        "dataset_tree": lock.get("dataset_tree"),
        "files": source_inventory,
        "record_classes": dict(Counter(item["record_class"] for item in source_inventory)),
        "json_failures": json_failures,
        "coordinate_parse_failures": parse_failures,
    }

    summary = {
        "schema": SCHEMA,
        "repository": SOURCE_REPOSITORY,
        "revision": revision,
        "source_file_count": len(source_inventory),
        "source_coordinate_record_count": len(coordinate_records),
        "source_coordinate_count": len(coordinate_index),
        "source_relation_count": len(relation_records),
        "source_relation_kind_count": len(relation_kinds),
        "cross_m_relation_count": sum(1 for relation in relation_records if relation["cross_m"]),
        "null_relation_endpoint_count": sum(
            1 for relation in relation_records for key in ("from_ref", "to_ref") if relation[key] is None
        ),
        "unresolved_relation_record_count": len(unresolved_relation_endpoints),
        "root_coordinate_counts": {str(root): root_coordinate_counts.get(root, 0) for root in range(6)},
        "root_source_record_counts": {str(root): root_record_counts.get(root, 0) for root in range(6)},
        "root_outgoing_relation_counts": {str(root): root_relation_counts.get(root, 0) for root in range(6)},
        "path_collision_count": len(path_collisions),
        "coordinate_parse_failure_count": len(parse_failures),
        "json_failure_count": len(json_failures),
        "source_relation_kinds": dict(sorted(relation_kinds.items())),
    }

    out.mkdir(parents=True, exist_ok=True)
    write_json(out / "source-inventory.json", inventory)
    write_json(out / "coordinates.json", coordinate_index)
    write_json(out / "relations.json", relation_records)
    write_json(out / "path-collisions.json", path_collisions)
    write_json(out / "unresolved-relation-endpoints.json", unresolved_relation_endpoints)
    write_json(out / "summary.json", summary)
    write_coordinate_tsv(out / "coordinates.tsv", coordinate_index)
    write_relation_tsv(out / "relations.tsv", relation_records)
    return {
        "revision": revision,
        "coordinate_records": coordinate_records,
        "coordinate_index": coordinate_index,
        "relation_records": relation_records,
        "summary": summary,
        "source_parent_relations": source_parent_relations,
        "path_collisions": path_collisions,
    }


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def record_token(record: dict[str, Any]) -> str:
    # @ is excluded from repository paths and SHA values in this corpus.
    return "@".join(
        [
            record["source_path"],
            record["source_git_blob"],
            record["source_sha256"],
            str(record["record_index"]),
            record["payload_sha256"],
        ]
    )


def write_coordinate_tsv(path: Path, coordinates: list[dict[str, Any]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle, delimiter="\t", lineterminator="\n")
        writer.writerow(
            [
                "source_ref",
                "root",
                "path",
                "separators",
                "lexical_parent_source_ref",
                "source_parent_refs",
                "aliases",
                "names",
                "record_count",
                "record_refs",
            ]
        )
        for item in coordinates:
            writer.writerow(
                [
                    item["source_ref"],
                    item["root"],
                    ",".join(map(str, item["path"])),
                    "".join(item["separators"]),
                    item["lexical_parent_source_ref"],
                    ";".join(item["source_parent_refs"]),
                    ";".join(item["aliases"]),
                    ";".join(name.replace("\t", " ").replace("\n", " ") for name in item["names"]),
                    item["record_count"],
                    ";".join(record_token(record) for record in item["records"]),
                ]
            )


def write_relation_tsv(path: Path, relations: list[dict[str, Any]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle, delimiter="\t", lineterminator="\n")
        writer.writerow(
            [
                "relation_ref",
                "source_kind",
                "from_ref",
                "to_ref",
                "orientation",
                "cross_m",
                "source_path",
                "source_git_blob",
                "source_sha256",
                "record_index",
                "payload_sha256",
                "property_keys",
            ]
        )
        for item in relations:
            writer.writerow(
                [
                    item["relation_ref"],
                    item["source_kind"],
                    item["from_ref"] or "",
                    item["to_ref"] or "",
                    item["orientation"],
                    "true" if item["cross_m"] else "false",
                    item["source_path"],
                    item["source_git_blob"],
                    item["source_sha256"],
                    item["record_index"],
                    item["payload_sha256"],
                    ";".join(item["property_keys"]),
                ]
            )


def implementation_coordinate_refs(implementation_repo: Path) -> tuple[set[str], Path]:
    path = implementation_repo / "Body/S/S0/portal-core/src/m_coordinate.rs"
    if not path.is_file():
        raise SystemExit(f"missing current Nara implementation coordinate floor at {path}")
    refs = {
        match.group(0)
        for match in COORD_ANY_RE.finditer(path.read_text(encoding="utf-8"))
        if parse_coordinate(match.group(0)) and parse_coordinate(match.group(0)).root == 4
    }
    return refs, path


def implementation_relation_kinds(implementation_repo: Path) -> tuple[set[str], Path]:
    path = implementation_repo / "Body/S/S0/portal-core/src/m_relation.rs"
    if not path.is_file():
        raise SystemExit(f"missing current Nara implementation relation floor at {path}")
    content = path.read_text(encoding="utf-8")
    kinds = set(re.findall(r"MRelationKind::([A-Za-z0-9_]+)", content))
    return kinds, path


def reconcile_nara(compiled: dict[str, Any], implementation_repo: Path, out: Path) -> dict[str, Any]:
    source_coordinates = {
        item["source_ref"]: item for item in compiled["coordinate_index"] if item["root"] == 4
    }
    implementation_refs, coordinate_file = implementation_coordinate_refs(implementation_repo)
    implementation_kinds, relation_file = implementation_relation_kinds(implementation_repo)

    exact = sorted(set(source_coordinates) & implementation_refs)
    source_absent = sorted(set(source_coordinates) - implementation_refs)
    implementation_no_basis = sorted(implementation_refs - set(source_coordinates))

    by_numeric_path: dict[tuple[int, tuple[int, ...]], set[str]] = defaultdict(set)
    for source_ref in source_coordinates:
        parsed = parse_coordinate(source_ref)
        assert parsed is not None
        by_numeric_path[parsed.path_key].add(source_ref)

    alias_mismatches = []
    for implementation_ref in implementation_no_basis:
        parsed = parse_coordinate(implementation_ref)
        if not parsed:
            continue
        alternatives = sorted(by_numeric_path.get(parsed.path_key, set()))
        if alternatives:
            alias_mismatches.append(
                {
                    "implementation_ref": implementation_ref,
                    "source_refs_same_numeric_path": alternatives,
                    "status": "requires-explicit-alias-resolution",
                }
            )

    parentage_mismatches = []
    source_parent_relations: dict[str, set[str]] = compiled["source_parent_relations"]
    for source_ref in sorted(implementation_refs & set(source_coordinates)):
        parsed = parse_coordinate(source_ref)
        assert parsed is not None
        map_parents = sorted(source_parent_relations.get(source_ref, set()))
        if map_parents and parsed.parent not in map_parents:
            parentage_mismatches.append(
                {
                    "source_ref": source_ref,
                    "implementation_lexical_parent": parsed.parent,
                    "map_parent_relations": map_parents,
                }
            )

    source_depths = [parse_coordinate(ref).depth for ref in source_coordinates if parse_coordinate(ref)]
    implementation_depths = [parse_coordinate(ref).depth for ref in implementation_refs if parse_coordinate(ref)]
    source_relation_kinds = sorted(
        {
            relation["source_kind"]
            for relation in compiled["relation_records"]
            if relation["from_root"] == 4 or relation["to_root"] == 4
        }
    )

    report = {
        "schema": "ql.epi-bimba-nara-conformance/v1",
        "source_revision": compiled["revision"],
        "implementation_revision": run_git(implementation_repo, "rev-parse", "HEAD"),
        "implementation_coordinate_file": coordinate_file.relative_to(implementation_repo).as_posix(),
        "implementation_relation_file": relation_file.relative_to(implementation_repo).as_posix(),
        "source_coordinate_count": len(source_coordinates),
        "implementation_coordinate_ref_count": len(implementation_refs),
        "exact_source_implementation_matches": exact,
        "exact_source_implementation_match_count": len(exact),
        "source_coordinates_absent_from_executable_floor": source_absent,
        "source_coordinates_absent_from_executable_floor_count": len(source_absent),
        "implementation_coordinates_without_direct_source_basis": implementation_no_basis,
        "implementation_coordinates_without_direct_source_basis_count": len(implementation_no_basis),
        "alias_or_notation_mismatches": alias_mismatches,
        "parentage_mismatches": parentage_mismatches,
        "recursive_depth": {
            "source_max_depth": max(source_depths, default=0),
            "implementation_max_depth": max(implementation_depths, default=0),
        },
        "relation_classes": {
            "bimba_source_relation_kinds": source_relation_kinds,
            "implementation_relation_kinds": sorted(implementation_kinds),
            "standing": "distinct-classes; implementation kinds are not source relations unless separately source-qualified",
        },
        "endpoint_direction_comparison": {
            "standing": "source relations preserve source->target direction; current implementation relations are derived/operational and are not rewritten as source endpoint parity",
        },
        "source_property_comparison": {
            "source_payload_record_count": sum(item["record_count"] for item in source_coordinates.values()),
            "implementation_carries_full_source_semantic_payload": False,
            "standing": "source payload remains source-owned and is indexed by provenance/digest; the 44-node implementation floor does not become Bimba semantic canon",
        },
        "legitimate_implementation_only_bindings": sorted(implementation_kinds),
    }
    write_json(out / "nara-conformance.json", report)
    return report


def cosmic_readiness(compiled: dict[str, Any], out: Path) -> dict[str, Any]:
    coordinates = [item for item in compiled["coordinate_index"] if item["root"] in COSMIC_ROOTS]
    relations = compiled["relation_records"]
    internal = [
        relation
        for relation in relations
        if relation["from_root"] in COSMIC_ROOTS and relation["to_root"] in COSMIC_ROOTS
    ]
    outward = [
        relation
        for relation in relations
        if relation["from_root"] in COSMIC_ROOTS
        and relation["to_root"] is not None
        and relation["to_root"] not in COSMIC_ROOTS
    ]
    incoming = [
        relation
        for relation in relations
        if relation["to_root"] in COSMIC_ROOTS
        and relation["from_root"] is not None
        and relation["from_root"] not in COSMIC_ROOTS
    ]
    relevant = [
        relation
        for relation in relations
        if (relation["from_root"] in COSMIC_ROOTS or relation["to_root"] in COSMIC_ROOTS)
        and any(word in relation["source_kind"].upper() for word in COSMIC_RELATION_WORDS)
    ]

    root_rows = {}
    for root in sorted(COSMIC_ROOTS):
        root_coord = next((item for item in coordinates if item["source_ref"] == f"#{root}"), None)
        root_rows[str(root)] = {
            "source_ref": f"#{root}",
            "names": root_coord["names"] if root_coord else [],
            "coordinate_count": sum(1 for item in coordinates if item["root"] == root),
            "max_depth": max((len(item["path"]) for item in coordinates if item["root"] == root), default=0),
            "outgoing_relation_count": sum(1 for relation in relations if relation["from_root"] == root),
        }

    report = {
        "schema": "ql.epi-bimba-cosmic-readiness/v1",
        "source_revision": compiled["revision"],
        "roots": root_rows,
        "m1_m2_m3_coordinate_count": len(coordinates),
        "relations_within_m1_m2_m3_count": len(internal),
        "cross_from_cosmic_to_other_m_count": len(outward),
        "cross_into_cosmic_from_other_m_count": len(incoming),
        "harmonic_temporal_transcription_relation_count": len(relevant),
        "harmonic_temporal_transcription_relation_kinds": dict(
            sorted(Counter(item["source_kind"] for item in relevant).items())
        ),
        "sample_internal_relation_refs": [item["relation_ref"] for item in internal[:25]],
        "sample_cross_relation_refs": [item["relation_ref"] for item in (outward + incoming)[:25]],
        "standing": "source-ground-ready; application/readiness remains separate and Prompt D must build one integrated instrument rather than one workspace per root",
    }
    write_json(out / "cosmic-readiness.json", report)
    return report


def print_returned_reality(summary: dict[str, Any], nara: dict[str, Any] | None, cosmic: dict[str, Any]) -> None:
    print("=== BIMBA_MAP_RETURNED_REALITY ===")
    print(json.dumps({"summary": summary, "nara": nara, "cosmic": cosmic}, ensure_ascii=False, sort_keys=True))
    print("=== END_BIMBA_MAP_RETURNED_REALITY ===")


def main() -> None:
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
    lock = load_lock(args.source_lock)
    compiled = compile_map(source_repo, out, lock)
    nara = None
    if args.implementation_repo:
        nara = reconcile_nara(compiled, args.implementation_repo.resolve(), out)
    cosmic = cosmic_readiness(compiled, out)
    print_returned_reality(compiled["summary"], nara, cosmic)


if __name__ == "__main__":
    main()
