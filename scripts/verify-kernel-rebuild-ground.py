#!/usr/bin/env python3
"""Verify K0 input identity, not capability completeness or live Neo4j parity.

Reads immutable Git objects in explicitly supplied checkouts. Never fetches,
changes source files, connects to a database, or promotes a source assertion.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys

SCHEMA = "ql.kernel-rebuild-source-canon-ledger/v1"
LEDGER = "docs/kernel-rebuild/source-canon-ledger-v1.json"
CLASSES = {
    "governing_architecture", "canonical_structural_source",
    "canonical_deep_coordinate_source", "current_c_implementation",
    "current_rust_implementation", "imported_c_implementation_body",
    "imported_rust_implementation_body", "returned_correction",
    "research_proposition", "implementation_evidence", "stale_representation",
    "mixed_source_field",
}


class GroundError(ValueError):
    """An input cannot support the ledger's exact identity claim."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GroundError(message)


def sha40(value: object) -> bool:
    return isinstance(value, str) and re.fullmatch(r"[0-9a-f]{40}", value) is not None


def validate_ledger(data: dict) -> None:
    require(data.get("schema") == SCHEMA, "unsupported K0 ledger schema")
    require(bool(data.get("authority_rule")), "missing scoped authority rule")
    repos = data["repositories"]
    require({"ql", "recovery", "epi", "nara"} <= set(repos), "missing input repository")
    for key, repo in repos.items():
        require(sha40(repo.get("revision")), f"{key}: moving or invalid revision")
    ids: set[str] = set()
    for item in data["inputs"]:
        key = item["id"]
        require(key not in ids, f"duplicate input: {key}")
        ids.add(key)
        require(item["repository"] in repos, f"{key}: unknown repository")
        path = item["path"]
        require(isinstance(path, str) and bool(path), f"{key}: empty path")
        require(not PurePosixPath(path).is_absolute() and ".." not in PurePosixPath(path).parts
                and "\\" not in path, f"{key}: unsafe path")
        require(item["kind"] in {"blob", "tree"}, f"{key}: invalid object kind")
        require("oid" not in item or sha40(item["oid"]), f"{key}: invalid object id")
        require(bool(item["classes"]) and set(item["classes"]) <= CLASSES,
                f"{key}: unknown source class")
        require(bool(item["standing"]), f"{key}: missing standing")
    indexed = {item["id"]: item for item in data["inputs"]}
    for key in ("imported-c", "imported-rust", "current-rust"):
        require(indexed[key]["standing"] == "KERNEL_REBUILD_BODY",
                f"{key}: rebuild body demoted to disposable evidence")
    live = data["live_graph"]
    require(live["observation_status"] == "not_observed"
            and live["database_revision"] is None and live["receipt"] is None,
            "K0 must not manufacture a live database observation")
    frames = data["kernel"]["canonical_context_frames"]
    require(len(frames) == len(set(frames)) == 7
            and data["kernel"]["historical_noncanonical_context_frame"] not in frames,
            "seven settled Context Frames must not be reopened")
    require(sha40(data["kernel"]["producer_merge_revision"]), "unpinned producer")
    for entry in data["discrepancies"]:
        require(set(entry["sources"]) <= ids, f"{entry['id']}: unresolved source link")
        require(bool(entry["rule"]) and bool(entry["owner"]), "unowned discrepancy")
    for handoff in data["handoff"].values():
        require(set(handoff["inputs"]) <= ids, "unresolved handoff input")


def git_bytes(root: Path, *args: str) -> bytes:
    result = subprocess.run(["git", "-C", str(root), *args], capture_output=True, check=False)
    if result.returncode:
        raise GroundError(f"git {' '.join(args)} failed in {root}: "
                          + result.stderr.decode("utf-8", errors="replace").strip())
    return result.stdout


def git_text(root: Path, *args: str) -> str:
    return git_bytes(root, *args).decode("utf-8").strip()


def read_object(root: Path, revision: str, path: str) -> bytes:
    return git_bytes(root, "show", f"{revision}:{path}")


def resolve_pin(root: Path, revision: str, item: dict) -> dict:
    oid = git_text(root, "rev-parse", "--verify", f"{revision}:{item['path']}")
    kind = git_text(root, "cat-file", "-t", oid)
    require(kind == item["kind"], f"{item['id']}: expected {item['kind']}, found {kind}")
    require("oid" not in item or oid == item["oid"], f"{item['id']}: Git object mismatch")
    result = {"id": item["id"], "path": item["path"], "revision": revision,
              "kind": kind, "oid": oid}
    if kind == "blob":
        result["sha256"] = hashlib.sha256(git_bytes(root, "cat-file", "blob", oid)).hexdigest()
    return result


def carrier_paths(value: object) -> set[str]:
    if isinstance(value, str):
        return {value} if value.startswith(("docs/", "ProjectCentral/")) else set()
    if isinstance(value, list):
        return set().union(*(carrier_paths(item) for item in value)) if value else set()
    if isinstance(value, dict):
        return set().union(*(carrier_paths(item) for item in value.values())) if value else set()
    return set()


def verify(data: dict, roots: dict[str, Path], out: Path) -> dict:
    validate_ledger(data)
    for key, repo in data["repositories"].items():
        revision = repo["revision"]
        require(git_text(roots[key], "rev-parse", "--verify", f"{revision}^{{commit}}") == revision,
                f"{key}: missing exact commit")
    resolved: list[dict] = []
    files: dict[tuple[str, str, str], dict] = {}
    for item in data["inputs"]:
        key = item["repository"]
        repo = data["repositories"][key]
        root, revision = roots[key], repo["revision"]
        pin = resolve_pin(root, revision, item)
        pin.update(repository=repo["repository"], classes=item["classes"], standing=item["standing"])
        resolved.append(pin)
        for raw in git_bytes(root, "ls-tree", "-r", "-z", revision, "--", item["path"]).split(b"\0"):
            if not raw:
                continue
            header, encoded_path = raw.split(b"\t", 1)
            mode, kind, oid = header.decode().split()
            path = encoded_path.decode("utf-8")
            identity = (repo["repository"], revision, path)
            if identity not in files:
                files[identity] = {"repository": repo["repository"], "revision": revision,
                                   "path": path, "mode": mode, "kind": kind, "oid": oid,
                                   "sha256": hashlib.sha256(git_bytes(root, "cat-file", "blob", oid)).hexdigest()
                                   if kind == "blob" else None, "input_ids": []}
            files[identity]["input_ids"].append(item["id"])

    ql = roots["ql"]
    revision = data["repositories"]["ql"]["revision"]
    epi, epi_revision = roots["epi"], data["repositories"]["epi"]["revision"]
    registry = json.loads(read_object(ql, revision, ".oi/product.json"))["capability_matrices"]["matrices"]
    expected = data["matrix_inventory"]
    require({item["id"] for item in registry} == set(expected["expected_family_ids"]),
            "matrix family inventory drift")
    matrices: list[dict] = []
    unique_paths: set[str] = set()
    for family in registry:
        for path in sorted(carrier_paths(family)):
            pin = resolve_pin(ql, revision, {"id": family["id"], "path": path, "kind": "blob"})
            pin.update(repository=data["repositories"]["ql"]["repository"], protocol=family["protocol"])
            matrices.append(pin)
            unique_paths.add(path)
    require(len(unique_paths) == expected["expected_carrier_count"], "matrix carrier inventory drift")

    source_lock = json.loads(read_object(ql, revision, "migration/epi-kernel/source-lock.json"))
    require(source_lock["source"]["revision"] == epi_revision, "C source revision mismatch")
    corrections: dict[str, str] = {}
    for correction in source_lock["ratified_corrections"]:
        corrections.update(correction["content_sha256"])
    c_files = 0
    for directory, entries in source_lock["reference_inventory"].items():
        for entry in entries:
            path = f"{directory}/{entry['path']}"
            original = f"{source_lock['source']['root']}/{path}"
            resolve_pin(epi, epi_revision, {"id": original, "path": original, "kind": "blob", "oid": entry["blob"]})
            vendored = f"vendor/epi-kernel/reference/{path}"
            actual = resolve_pin(ql, revision, {"id": vendored, "path": vendored, "kind": "blob"})
            require(actual["sha256"] == corrections[path] if path in corrections else actual["oid"] == entry["blob"],
                    f"{vendored}: unratified difference from locked source")
            c_files += 1
    for path, digest in corrections.items():
        require(hashlib.sha256(read_object(ql, revision, f"vendor/epi-kernel/reference/{path}")).hexdigest() == digest,
                f"{path}: ratified correction digest mismatch")

    bimba_lock = json.loads(read_object(ql, revision, "data/epi-bimba-map/source-lock.json"))
    require(bimba_lock["revision"] == epi_revision, "Bimba source revision mismatch")
    resolve_pin(epi, epi_revision, {"id": "locked-dataset-tree", "path": f"{bimba_lock['map_root']}/datasets",
                                    "kind": "tree", "oid": bimba_lock["dataset_tree"]})
    for item in bimba_lock["required_sources"]:
        resolve_pin(epi, epi_revision, {"id": item["path"], "path": item["path"], "kind": "blob", "oid": item["git_blob"]})

    producer = data["kernel"]["producer_merge_revision"]
    producer_pins = []
    for path in data["kernel"]["producer_paths"]:
        accepted = git_text(ql, "rev-parse", "--verify", f"{producer}:{path}")
        current = git_text(ql, "rev-parse", "--verify", f"{revision}:{path}")
        require(accepted == current, f"producer floor changed at {path}")
        producer_pins.append({"path": path, "oid": current, "accepted_revision": producer})
    recovery = data["repositories"]["recovery"]["revision"]
    recovery_pins = [resolve_pin(roots["recovery"], recovery, {"id": path, "path": path, "kind": "blob"})
                     for path in data["handoff"]["K1"]["recovery_candidates"]]
    native_sources = git_text(ql, "ls-tree", "--name-only", "-r", revision, "--", "c/src").splitlines()
    require(native_sources == ["c/src/primitive.c"], "pinned native C floor differs from the K0 observation")

    summary = {"schema": "ql.kernel-rebuild-ground-verification/v1",
               "ledger_sha256": hashlib.sha256(json.dumps(data, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
               "repositories": data["repositories"], "input_count": len(resolved),
               "source_file_count": len(files), "matrix_family_count": len(registry),
               "matrix_carrier_count": len(unique_paths), "c_source_files_checked": c_files,
               "ratified_correction_files_checked": len(corrections),
               "bimba_required_sources_checked": len(bimba_lock["required_sources"]),
               "native_c_sources": native_sources, "producer_pins": producer_pins,
               "k1_recovery_inputs": recovery_pins,
               "source_pin_verdict": "PASS", "live_graph_observation": "NOT_PERFORMED",
               "capability_completeness_verdict": "NOT_ASSESSED", "k1_implementation_verdict": "NOT_STARTED"}
    out.mkdir(parents=True, exist_ok=True)
    for name, value in (("summary.json", summary), ("resolved-inputs.json", resolved),
                        ("source-file-inventory.json", [files[key] for key in sorted(files)]),
                        ("matrix-carrier-inventory.json", matrices),
                        ("live-read-request.json", {"statements": [{"statement": s} for s in data["live_graph"]["read_statements"]]})):
        (out / name).write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return summary


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ql-repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--source-repo", type=Path, required=True)
    parser.add_argument("--recovery-repo", type=Path, required=True)
    parser.add_argument("--implementation-repo", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=Path("target/kernel-rebuild-k0"))
    args = parser.parse_args()
    try:
        data = json.loads((args.ql_repo / LEDGER).read_text(encoding="utf-8"))
        result = verify(data, {"ql": args.ql_repo, "epi": args.source_repo,
                               "recovery": args.recovery_repo, "nara": args.implementation_repo}, args.out)
        print(json.dumps(result, indent=2))
        return 0
    except (GroundError, KeyError, TypeError, OSError, json.JSONDecodeError) as error:
        print(f"K0 source ground FAIL: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
