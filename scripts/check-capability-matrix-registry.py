#!/usr/bin/env python3
"""Validate every capability-matrix carrier declared by this product."""

from __future__ import annotations

import csv
import json
import sys
from pathlib import Path, PurePosixPath


ROOT = Path(__file__).resolve().parents[1]
PRODUCT_MANIFEST = ROOT / ".oi/product.json"
REQUIRED_PROFILE_COLUMNS = {"id", "record_type", "capability_refs", "extensions", "relation"}


class RegistryError(ValueError):
    pass


def load_json(path: Path) -> dict:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RegistryError(f"{path.relative_to(ROOT)}: cannot read JSON: {error}") from error
    if not isinstance(value, dict):
        raise RegistryError(f"{path.relative_to(ROOT)}: expected a JSON object")
    return value


def repository_path(raw: object, context: str) -> Path:
    if not isinstance(raw, str) or not raw:
        raise RegistryError(f"{context}: carrier path must be a non-empty string")
    logical = PurePosixPath(raw)
    if logical.is_absolute() or ".." in logical.parts:
        raise RegistryError(f"{context}: carrier path must stay within the repository: {raw!r}")
    path = ROOT.joinpath(*logical.parts)
    if not path.is_file():
        raise RegistryError(f"{context}: carrier does not exist: {raw}")
    return path


def validate_product_profile(entry: dict, carriers: dict) -> None:
    manifest_path = repository_path(carriers.get("manifest"), f"{entry['id']}.manifest")
    records_path = repository_path(carriers.get("records"), f"{entry['id']}.records")
    repository_path(carriers.get("rendering"), f"{entry['id']}.rendering")
    repository_path(carriers.get("account"), f"{entry['id']}.account")
    manifest = load_json(manifest_path)
    if manifest.get("protocol") != entry["protocol"]:
        raise RegistryError(f"{entry['id']}: declared protocol disagrees with its manifest")
    if not isinstance(manifest.get("matrix_id"), str) or not manifest["matrix_id"]:
        raise RegistryError(f"{entry['id']}: profile manifest needs matrix_id")

    with records_path.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        headers = set(reader.fieldnames or [])
        missing = REQUIRED_PROFILE_COLUMNS - headers
        if missing:
            raise RegistryError(f"{entry['id']}: records lack columns: {', '.join(sorted(missing))}")
        rows = list(reader)
    ids = [row["id"] for row in rows]
    if not ids or any(not value for value in ids) or len(ids) != len(set(ids)):
        raise RegistryError(f"{entry['id']}: record ids must be non-empty and unique")
    capabilities = {row["id"] for row in rows if row["record_type"] == "capability"}
    for number, row in enumerate(rows, start=2):
        try:
            refs = json.loads(row["capability_refs"] or "[]")
            extensions = json.loads(row["extensions"] or "{}")
        except json.JSONDecodeError as error:
            raise RegistryError(f"{records_path.relative_to(ROOT)}:{number}: malformed JSON cell") from error
        if not isinstance(refs, list) or not all(isinstance(ref, str) and ref for ref in refs):
            raise RegistryError(f"{records_path.relative_to(ROOT)}:{number}: invalid capability_refs")
        if not isinstance(extensions, dict):
            raise RegistryError(f"{records_path.relative_to(ROOT)}:{number}: extensions must be an object")
        missing_refs = sorted(set(refs) - capabilities)
        if missing_refs:
            raise RegistryError(
                f"{records_path.relative_to(ROOT)}:{number}: missing capability refs: {', '.join(missing_refs)}"
            )


def validate_native_matrix(entry: dict, carriers: dict) -> None:
    data_path = repository_path(carriers.get("data"), f"{entry['id']}.data")
    repository_path(carriers.get("rendering"), f"{entry['id']}.rendering")
    if "index" in carriers:
        repository_path(carriers["index"], f"{entry['id']}.index")
    data = load_json(data_path)
    if data.get("schema_version") != entry["protocol"]:
        raise RegistryError(f"{entry['id']}: declared protocol disagrees with data.schema_version")
    partitions = entry.get("partitions", [])
    if not isinstance(partitions, list):
        raise RegistryError(f"{entry['id']}: partitions must be an array")
    expected_partition_protocol = entry.get("partition_protocol")
    for number, raw_path in enumerate(partitions):
        path = repository_path(raw_path, f"{entry['id']}.partitions[{number}]")
        if load_json(path).get("schema_version") != expected_partition_protocol:
            raise RegistryError(f"{path.relative_to(ROOT)}: unexpected partition schema_version")


def validate_member_matrix(entry: dict) -> set[str]:
    members = entry.get("members")
    if not isinstance(members, list) or not members:
        raise RegistryError(f"{entry['id']}: members must be a non-empty array")
    member_ids: set[str] = set()
    paths: set[str] = set()
    for number, member in enumerate(members):
        if not isinstance(member, dict) or not isinstance(member.get("id"), str) or not member["id"]:
            raise RegistryError(f"{entry['id']}.members[{number}]: non-empty id is required")
        if member["id"] in member_ids:
            raise RegistryError(f"{entry['id']}: duplicate member id {member['id']!r}")
        member_ids.add(member["id"])
        if "data" in member:
            data_path = repository_path(member["data"], f"{entry['id']}.{member['id']}.data")
            rendering = repository_path(member.get("rendering"), f"{entry['id']}.{member['id']}.rendering")
            if load_json(data_path).get("schema") != entry["protocol"]:
                raise RegistryError(f"{data_path.relative_to(ROOT)}: unexpected schema")
            paths.update((member["data"], str(rendering.relative_to(ROOT))))
        elif "records" in member:
            manifest_path = repository_path(member.get("manifest"), f"{entry['id']}.{member['id']}.manifest")
            records_path = repository_path(member["records"], f"{entry['id']}.{member['id']}.records")
            manifest = load_json(manifest_path)
            if manifest.get("protocol") != entry["protocol"] or not manifest.get("matrix_id"):
                raise RegistryError(f"{manifest_path.relative_to(ROOT)}: protocol or matrix_id disagrees with registry")
            required = member.get("required_columns")
            if not isinstance(required, list) or not required or not all(isinstance(column, str) and column for column in required):
                raise RegistryError(f"{entry['id']}.{member['id']}: required_columns must be non-empty strings")
            with records_path.open(encoding="utf-8", newline="") as stream:
                reader = csv.DictReader(stream)
                if reader.fieldnames != required:
                    raise RegistryError(f"{records_path.relative_to(ROOT)}: header disagrees with required_columns")
                rows = list(reader)
            if not rows:
                raise RegistryError(f"{records_path.relative_to(ROOT)}: relational field must not be empty")
            paths.update((member["manifest"], member["records"]))
        else:
            raise RegistryError(f"{entry['id']}.{member['id']}: data or records carrier is required")
    return paths


def validate_discovery(registry: dict, claimed_paths: set[str]) -> None:
    discovery = registry.get("discovery")
    if not isinstance(discovery, dict):
        raise RegistryError(".oi/product.json: capability matrix discovery policy is required")
    roots = discovery.get("roots")
    signatures = discovery.get("filename_signatures")
    excluded = discovery.get("excluded")
    valid_signatures = (
        isinstance(signatures, list)
        and signatures
        and all(
            isinstance(signature, list)
            and signature
            and all(isinstance(term, str) and term for term in signature)
            for signature in signatures
        )
    )
    if not isinstance(roots, list) or not valid_signatures or not isinstance(excluded, dict):
        raise RegistryError(".oi/product.json: discovery roots, filename_signatures and excluded are required")
    candidates: set[str] = set()
    for raw_root in roots:
        root = ROOT / raw_root
        if not root.is_dir():
            raise RegistryError(f"discovery root does not exist: {raw_root}")
        for path in root.rglob("*"):
            relative = path.relative_to(ROOT).as_posix()
            lowered = path.name.lower()
            if path.is_file() and any(all(term.lower() in lowered for term in signature) for signature in signatures):
                candidates.add(relative)
    for path, reason in excluded.items():
        repository_path(path, "capability_matrices.discovery.excluded")
        if path not in candidates or not isinstance(reason, str) or not reason.strip():
            raise RegistryError(f"invalid discovery exclusion: {path}")
    unclassified = candidates - claimed_paths - set(excluded)
    if unclassified:
        raise RegistryError(f"unclassified capability matrix paths: {', '.join(sorted(unclassified))}")
    stale_exclusions = set(excluded) & claimed_paths
    if stale_exclusions:
        raise RegistryError(f"registered carriers must not also be excluded: {', '.join(sorted(stale_exclusions))}")


def validate() -> int:
    product = load_json(PRODUCT_MANIFEST)
    registry = product.get("capability_matrices")
    if not isinstance(registry, dict) or registry.get("schema") != "oi.product-capability-sources/v1":
        raise RegistryError(".oi/product.json: missing oi.product-capability-sources/v1 registry")
    matrices = registry.get("matrices")
    if not isinstance(matrices, list) or not matrices:
        raise RegistryError(".oi/product.json: capability_matrices.matrices must be non-empty")
    ids = [entry.get("id") for entry in matrices if isinstance(entry, dict)]
    if len(ids) != len(matrices) or any(not isinstance(value, str) or not value for value in ids):
        raise RegistryError(".oi/product.json: every matrix needs a non-empty id")
    if len(ids) != len(set(ids)):
        raise RegistryError(".oi/product.json: matrix ids must be unique")
    primary = registry.get("primary_id")
    suite_entries = [entry for entry in matrices if entry.get("suite_catalogue") is True]
    if len(suite_entries) != 1 or suite_entries[0]["id"] != primary:
        raise RegistryError(".oi/product.json: primary_id must name the sole suite-catalogued matrix")

    claimed_paths: set[str] = set()
    for entry in matrices:
        protocol = entry.get("protocol")
        carriers = entry.get("carriers")
        members = entry.get("members")
        if not isinstance(protocol, str) or not protocol:
            raise RegistryError(f"{entry['id']}: protocol is required")
        if not isinstance(carriers, dict) or not carriers:
            if not isinstance(members, list) or not members:
                raise RegistryError(f"{entry['id']}: carriers or members are required")
            carriers = {}
        paths = list(carriers.values()) + list(entry.get("partitions", []))
        duplicates = claimed_paths.intersection(paths)
        if duplicates:
            raise RegistryError(f"{entry['id']}: carriers already owned by another matrix: {', '.join(sorted(duplicates))}")
        claimed_paths.update(paths)
        if entry.get("role") == "suite-profile":
            validate_product_profile(entry, carriers)
        elif carriers:
            validate_native_matrix(entry, carriers)
        if "members" in entry:
            member_paths = validate_member_matrix(entry)
            duplicates = claimed_paths.intersection(member_paths)
            if duplicates:
                raise RegistryError(f"{entry['id']}: carriers already owned by another matrix: {', '.join(sorted(duplicates))}")
            claimed_paths.update(member_paths)
    validate_discovery(registry, claimed_paths)
    print(f"Capability matrix registry: {len(matrices)} matrix families and {len(claimed_paths)} carriers verified.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(validate())
    except RegistryError as error:
        print(f"capability matrix registry failed: {error}", file=sys.stderr)
        raise SystemExit(2)
