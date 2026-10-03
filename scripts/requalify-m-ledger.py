#!/usr/bin/env python3
"""Requalify the ONE ledger after an admitted source change, without moving old proofs.

Historical decisions stay decisions. New scoped source/native observations prove
their continuing application; numerical readiness awaits full successor proofs.
The prior ledger and every old evidence artifact remain exact Git objects, with
their content hashes recorded in the new receipt. No runtime guard is weakened.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("m_ledger", ROOT / "scripts/m-ledger.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
RECEIPT = Path("fixtures/kernel/m-ledger-source-requalification-v1.json")
REPLAYS = Path("fixtures/kernel/source-requalification")
PROOF_FILES = [Path(f"fixtures/kernel/{name}-finite-proof-v1.json") for name in ("m2", "m3")]
INPUTS = [
    "fixtures/kernel/m-tree-v1.json", "c/registry/m-tree-source-v1.json", "c/src/m_tree_data.inc",
    "crates/ql-mef/src/m_tree.rs", "crates/ql-mef/tests/native_m_tree.rs",
    "c/src/m_tree.c", "c/src/m1.c", "c/src/m1_state.c",
    "crates/ql-mef/src/m1.rs", "crates/ql-mef/src/m1_engine.rs",
    "crates/ql-mef/tests/m1_engine.rs", "crates/ql-mef/tests/m1_state.rs",
    "migration/epi-kernel/m1-engine-probe.c", "migration/epi-kernel/m1-state-probe.c",
    "vendor/epi-kernel/reference/src/m1.c", "vendor/epi-kernel/reference/src/psychoid_numbers.c",
    "docs/KERNEL-M1-ENGINE-CONTRACT.md", "migration/epi-kernel/source-lock.json",
    "scripts/check-m1-source.py", "scripts/check-m1-literals.py", "scripts/test-m1-engine.sh",
    "scripts/m3-source-parity.py", "fixtures/kernel/m3-source-bindings-v1.json",
    "vendor/epi-kernel/corrections/M3-COIN-1.patch",
]


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git_bytes(commit: str, path: str) -> bytes:
    return subprocess.check_output(["git", "show", f"{commit}:{path}"], cwd=ROOT)


def verify_historical_proof(root: Path, path: str, digest: str, commit: str) -> None:
    """Retain the qualified original while admitting a published history alias.

    This checks source lineage only. It does not requalify current numerical
    inputs, publish a new finite proof, or confer numerical readiness.
    """
    def checked_path(value: str) -> Path:
        candidate = Path(value)
        if candidate.is_absolute() or ".." in candidate.parts:
            raise ValueError("unsafe historical proof path: " + value)
        return root / candidate

    def git_source(ref: str) -> bytes:
        match = re.fullmatch(r"git:([0-9a-f]{40}):(.+)", ref)
        if not match:
            raise ValueError("unqualified historical proof Git source")
        checked_path(match[2])
        return subprocess.check_output(["git", "show", f"{match[1]}:{match[2]}"], cwd=root)

    original = subprocess.check_output(["git", "show", f"{commit}:{path}"], cwd=root)
    if sha(original) != digest:
        raise ValueError("historical numerical proof was restamped: " + path)
    active = checked_path(path).read_bytes()
    if active == original:
        return
    if path != "fixtures/kernel/m3-finite-proof-v1.json":
        raise ValueError("unqualified historical proof alias: " + path)

    record = json.loads(checked_path("fixtures/kernel/m3-journey-source-integration-v1.json").read_text())
    if record["schema"] != "ql.published-journey-source-integration/v1":
        raise ValueError("unqualified historical proof integration record")
    source = record["current_source"]
    registry = json.loads(checked_path("fixtures/kernel/m-tree-v1.json").read_text())
    coordinates = json.loads(checked_path("fixtures/kernel/k8-structure-receipt-v1.json").read_text())
    structure_spec = importlib.util.spec_from_file_location("historical_alias_current_structure", checked_path("scripts/k8-structure.py"))
    structure = importlib.util.module_from_spec(structure_spec)
    structure_spec.loader.exec_module(structure)
    projected, _ = structure.project(root)
    ledger_bytes = checked_path(source["ledger"]["path"]).read_bytes()
    ledger = json.loads(ledger_bytes)
    if (source["source_revision"] != registry["source_revision"]
            or source["numerical_registry_revision"] != registry["registry_revision"]
            or source["coordinate_registry_revision"] != coordinates["registry_revision"]
            or source["coordinate_registry_revision"] != projected["registry_revision"]
            or source["ledger"]["path"] != "fixtures/kernel/m-ledger-v1.json"
            or source["ledger"]["sha256"] != sha(ledger_bytes)
            or source["ledger"]["revision"] != ledger["ledger_revision"]):
        raise ValueError("historical integration record has stale current source standing")

    history = record["native_history"]
    retained = history["original"]
    if (retained["sha256"] != digest
            or checked_path(retained["path"]).read_bytes() != original
            or git_source(retained["original_git_ref"]) != original):
        raise ValueError("retained historical numerical original changed")
    successor = history["successor"]
    expected_ref = f"git:{record['parents']['incoming']}:{path}"
    if (successor["path"] != path or successor["git_ref"] != expected_ref
            or successor["sha256"] != sha(active)
            or git_source(expected_ref) != active):
        raise ValueError("active historical successor differs from published Git source")

    lock = history["native_lineage"]
    lineage_bytes = checked_path(lock["path"]).read_bytes()
    if (lock["path"] != "fixtures/kernel/k8-build-lineage-v1.json"
            or sha(lineage_bytes) != lock["sha256"]
            or git_source(f"git:{record['parents']['incoming']}:{lock['path']}") != lineage_bytes):
        raise ValueError("native historical lineage missing or forged")
    lineage = json.loads(lineage_bytes)
    pointer = re.fullmatch(r"/successors/(\d+)", lock["pointer"])
    if not pointer or int(pointer[1]) >= len(lineage["successors"]):
        raise ValueError("unqualified native historical lineage pointer")
    entry = lineage["successors"][int(pointer[1])]
    if (lineage["schema"] != "ql.k8-build-lineage/v1"
            or lineage["proofs"]["m3"] != sha(active)
            or entry["engine"] != "m3" or entry["previous_sha256"] != digest
            or entry["sha256"] != sha(active)
            or entry["accepted_at_revision"] != history["published_ci"]["revision"]):
        raise ValueError("native historical successor is not qualified by its original")

    before, after = json.loads(original), json.loads(active)
    changed_input = "crates/ql-core/src/pole/tarot.rs"
    if history["only_proof_input_change"] != "/inputs/" + changed_input:
        raise ValueError("unqualified historical successor input change")
    restored = copy.deepcopy(after)
    restored["inputs"][changed_input] = before["inputs"][changed_input]
    if restored != before or after["inputs"][changed_input] == before["inputs"][changed_input]:
        raise ValueError("historical successor changes more than its naming input")
    if sha(checked_path(changed_input).read_bytes()) != after["inputs"][changed_input]:
        raise ValueError("published historical naming input differs from current source")


def history(commit: str) -> tuple[dict, dict]:
    commit = subprocess.check_output(["git", "rev-parse", f"{commit}^{{commit}}"], cwd=ROOT, text=True).strip()
    raw = git_bytes(commit, m.LEDGER.as_posix())
    prior = json.loads(raw)
    artifacts = []
    for item in prior["evidence"]:
        lock = item["artifact"]
        body = git_bytes(commit, lock["path"])
        if sha(body) != lock["sha256"]:
            raise ValueError(f"historical Git artifact does not match the accepted evidence: {item['id']}")
        artifacts.append({"id": item["id"], "registry_revision": item["registry_revision"],
                          "artifact": lock, "git_ref": f"git:{commit}:{lock['path']}",
                          "standing": "historical evidence; not requalified or claimed as current"})
    return prior, {"ledger_revision": prior["ledger_revision"], "ledger_sha256": sha(raw),
                   "git_ref": f"git:{commit}:{m.LEDGER.as_posix()}", "git_commit": commit,
                   "evidence": artifacts}


def verify_replays(c_log: bytes, rust_log: bytes, audit: dict, registry: dict) -> None:
    c_text, rust_text = c_log.decode(), rust_log.decode()
    for expected in ["1352 equal literal readings, 376 retained differences", "1728 exact raw/DR strings",
                     "M1 native sanitizers and installed C++ consumer passed"]:
        if expected not in c_text:
            raise ValueError("actual current M1 C replay is missing: " + expected)
    for expected in ["actual_native_c_matches_rust_and_retained_c_return",
                     "actual_c_source_relation_and_state_match_independent_rust",
                     "actual_c_descriptors_equal_every_rust_manifest_record"]:
        if not re.search(r"test " + expected + r" \.\.\. ok", rust_text):
            raise ValueError("actual current Rust/C replay is missing: " + expected)
    if "FAILED" in rust_text or "test result: FAILED" in rust_text:
        raise ValueError("the supplied native replay contains a failure")
    if audit["schema"] != "ql.m3-source-audit/v1" or audit["source_revision"] != registry["source_revision"] \
            or audit["registry_revision"] != registry["registry_revision"]:
        raise ValueError("M3 source audit is not for the current admitted source/registry")
    findings = [json.loads(item["detail"]) for item in audit["discrepancies"]]
    if any(item["code"] in ("codon-charge", "pair-descriptor") for item in findings):
        raise ValueError("an accepted coin correction is not applied in the current source")
    # Inspect the actual source and the independently ratified derivation, not
    # merely the absence of a summary finding.
    genetics = audit["details"]["genetics"]
    if not genetics["charges"] or not genetics["pairs"] \
            or any(item["source"] != item["ratified_derivation"] for item in genetics["charges"]) \
            or any(item["source"] != item["native"] for item in genetics["pairs"]):
        raise ValueError("current coin source readings disagree with the ratified law")


def apply(args) -> None:
    prior, old = history(args.previous_git_ref)
    registry = m.read(ROOT / m.REGISTRY)
    if prior["registry"]["revision"] == registry["registry_revision"]:
        raise ValueError("source requalification requires a distinct admitted registry")
    c_log, rust_log = args.m1_c_log.read_bytes(), args.rust_log.read_bytes()
    audit = m.read(args.m3_audit)
    verify_replays(c_log, rust_log, audit, registry)
    proof_locks = {p.as_posix(): sha((ROOT / p).read_bytes()) for p in PROOF_FILES}
    for path, digest in proof_locks.items():
        if sha(git_bytes(old["git_commit"], path)) != digest:
            raise ValueError("historical numerical proof changed before successor execution: " + path)
    applied = [d for d in prior["discrepancies"] if d["state"] == "applied"]
    if len(applied) != 131:
        raise ValueError("inspect the changed decision inventory before requalification")
    REPLAYS_ABS = ROOT / REPLAYS
    REPLAYS_ABS.mkdir(parents=True, exist_ok=True)
    for name, data in [("m1-native-c-replay.log", c_log), ("native-rust-c-replay.log", rust_log),
                       ("m3-source-audit.json", args.m3_audit.read_bytes())]:
        (REPLAYS_ABS / name).write_bytes(data)
    mapping = {
        "coin-1:map-corrected-audit": "source-requalification:coin-current-source",
        "k5-m1:reviewed-dispositions": "source-requalification:m1-current-source",
        "k5-m1:accepted-execution": "source-requalification:m1-current-native",
        "bimba-map:registry-source": "source-requalification:current-registry",
        "bimba-map:native-registry": "source-requalification:current-tree-native",
        "k-move:reference-m1-12x12-ananda": "source-requalification:m1-current-reference",
        "k-move:reference-execution": "source-requalification:m1-current-native",
    }
    # Import immutable source locks through the existing owner, preserving the
    # reviewed rows, bindings and all discrepancy lifecycle/decision content.
    ledger = m.refresh(ROOT, copy.deepcopy(prior))
    for assessment in ledger["assessments"].values():
        for claim in assessment["readiness"].values():
            if claim["status"] != "unimplemented":
                claim.update(status="unassessed", warrant="unassessed", evidence=[])
            else:
                claim.update(warrant="unassessed", evidence=[])
        for axis in assessment["parity"]:
            assessment["parity"][axis] = []
    # Existing source index witnesses are new observations; none of the old17
    # numerical/source evidence records are simply stamped with the new source.
    seed = m.refresh(ROOT)
    ledger["evidence"] = seed["evidence"]
    ledger["assessments"]["k2-index"] = seed["assessments"]["k2-index"]
    for d in ledger["discrepancies"]:
        for part in ("decision", "proposal"):
            if d.get(part) and (part == "decision" or d["state"] == "applied"):
                d[part]["evidence"] = [mapping[ref] for ref in d[part]["evidence"]]
    new = []
    for original, ident in mapping.items():
        subjects = [d["id"] for d in prior["discrepancies"]
                    if any(original in (d.get(part) or {}).get("evidence", []) for part in ("decision", "proposal"))]
        if not subjects:
            raise ValueError("requalification evidence has no exact decision subjects: " + original)
        if ident in {e["id"] for e in new}:
            next(e for e in new if e["id"] == ident)["subjects"] = sorted(set(
                next(e for e in new if e["id"] == ident)["subjects"] + subjects))
            continue
        if "coin" in ident:
            artifact, strata, axes = REPLAYS / "m3-source-audit.json", ["source"], ["operational"]
        elif "tree-native" in ident:
            artifact, strata, axes = REPLAYS / "native-rust-c-replay.log", ["c", "rust"], ["coordinate", "relation"]
        elif "m1-current-native" in ident:
            artifact, strata, axes = REPLAYS / "native-rust-c-replay.log", ["source", "c", "rust"], ["source"]
        elif "current-registry" in ident:
            artifact, strata, axes = m.REGISTRY, ["source", "c", "rust"], ["coordinate"]
        elif "current-reference" in ident:
            artifact, strata, axes = Path("vendor/epi-kernel/reference/src/m1.c"), ["source", "c"], ["source"]
        else:
            artifact, strata, axes = Path("docs/KERNEL-M1-ENGINE-CONTRACT.md"), ["source"], ["source"]
        executable = ident.endswith(("current-native", "tree-native")) or "coin" in ident
        new.append({"id": ident, "kind": "observation" if executable else "source",
                    "artifact": m.lock(ROOT, artifact), "registry_revision": registry["registry_revision"],
                    "subjects": subjects, "strata": strata, "axes": axes,
                    "result": "passed" if executable else "present"})
    ledger["evidence"].extend(new)
    # Content/authority/lifecycle is identical; only operative evidence refers
    # to the newly executed source. Historical references remain in old Git.
    for before, after in zip(prior["discrepancies"], ledger["discrepancies"]):
        restored = copy.deepcopy(after)
        for part in ("decision", "proposal"):
            if before.get(part):
                restored[part]["evidence"] = before[part]["evidence"]
        if restored != before:
            raise ValueError("a prior decision/lifecycle would be rewritten: " + before["id"])
    ledger["ledger_revision"] = m.digest(m.canonical({k: v for k, v in ledger.items() if k != "ledger_revision"}))
    m.verify(ROOT, ledger)
    receipt = {"schema": "ql.m-ledger-source-requalification/v1", "source_revision": registry["source_revision"],
               "registry_revision": registry["registry_revision"], "ledger_revision": ledger["ledger_revision"],
               "historical": old, "current_input_locks": {p: m.lock(ROOT, p)["sha256"] for p in INPUTS},
               "replays": [m.lock(ROOT, REPLAYS / p) for p in ("m1-native-c-replay.log", "native-rust-c-replay.log", "m3-source-audit.json")],
               "evidence_mapping": mapping, "preserved_applied_decisions": len(applied),
               "historical_numerical_proofs_unchanged": proof_locks,
               "standing": "current source/application revalidation; old numerical readiness/parity unassessed until full successor execution",
               "not_claimed": ["current whole M1/M2/M3 numerical readiness from this bootstrap", "installed visual/audio acceptance"]}
    (ROOT / RECEIPT).write_text(m.encode(receipt))
    (ROOT / m.LEDGER).write_text(m.encode(ledger))
    print(json.dumps({"result": "source-requalified", "ledger_revision": ledger["ledger_revision"],
                      "registry_revision": registry["registry_revision"], "preserved_applied_decisions": len(applied)}))


def verify_implementation_relocation(root: Path, receipt: dict) -> dict:
    """Verify a concrete owner move while retaining every prior semantic record.

    Implementation IDs are stable opaque references in the native ledger.
    The explicit path is the current implementation location. This admission
    neither changes historical evidence nor qualifies runtime behavior.
    """
    record = receipt.get("implementation_relocation")
    if record is None:
        return receipt
    if not isinstance(record, dict) or record.get("schema") != "ql.native-implementation-relocation/v1":
        raise ValueError("unqualified implementation relocation")
    commit = record.get("previous_git_commit", "")
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise ValueError("implementation relocation needs an exact prior Git commit")
    def previous(path: str) -> bytes:
        return subprocess.check_output(["git", "show", commit + ":" + path], cwd=root)
    old_raw = previous(m.LEDGER.as_posix())
    old = json.loads(old_raw)
    current_raw = (root / m.LEDGER).read_bytes()
    current = json.loads(current_raw)
    if sha(old_raw) != record["previous_ledger_sha256"] or sha(current_raw) != record["current_ledger_sha256"]:
        raise ValueError("implementation relocation ledger bytes changed")
    restored = copy.deepcopy(current)
    implementations = {row["id"]: row for row in restored["implementations"]}
    moves = record["moves"]
    expected_symbols = {"face", "ground", "parse_language", "frame_view", "return_view"}
    if len(moves) != 5 or {row["symbol"] for row in moves} != expected_symbols or len({row["id"] for row in moves}) != 5:
        raise ValueError("implementation relocation exceeded the five canonical helpers")
    for move in moves:
        if move["before_path"] != "crates/ql-cli/src/vak_composition.rs" or move["after_path"] != "crates/ql-mef/src/vak_composition_wire.rs":
            raise ValueError("implementation relocation changed its native owner")
        row = implementations.get(move["id"])
        if row is None or row["symbol"] != move["symbol"] or row["path"] != move["after_path"]:
            raise ValueError("implementation relocation lost an original binding")
        row["path"] = move["before_path"]
    restored["ledger_revision"] = old["ledger_revision"]
    if restored != old:
        raise ValueError("implementation relocation changed retained semantic ledger data")
    prior_receipt = json.loads(previous(RECEIPT.as_posix()))
    restored_receipt = copy.deepcopy(receipt)
    restored_receipt.pop("implementation_relocation")
    restored_receipt["ledger_revision"] = prior_receipt["ledger_revision"]
    if restored_receipt != prior_receipt or receipt["ledger_revision"] != current["ledger_revision"]:
        raise ValueError("implementation relocation rewrote retained qualification")
    if set(record["current_source_locks"]) != {"crates/ql-cli/src/vak_composition.rs", "crates/ql-mef/src/vak_composition_wire.rs"}:
        raise ValueError("implementation relocation lost its actual canonical source locks")
    for path, digest in record["current_source_locks"].items():
        if sha((root / path).read_bytes()) != digest or sha(previous(path)) != digest:
            raise ValueError("implementation relocation canonical source changed")
    integration_path = "fixtures/kernel/m3-journey-source-integration-v1.json"
    prior_integration = json.loads(previous(integration_path))
    integration = json.loads((root / integration_path).read_bytes())
    restored_integration = copy.deepcopy(integration)
    restored_integration["current_source"]["ledger"] = prior_integration["current_source"]["ledger"]
    if restored_integration != prior_integration:
        raise ValueError("implementation relocation rewrote historical native integration")
    active_lock = integration["current_source"]["ledger"]
    if active_lock["path"] != m.LEDGER.as_posix() or active_lock["sha256"] != sha(current_raw) or active_lock["revision"] != current["ledger_revision"]:
        raise ValueError("implementation relocation current integration lock is stale")
    expected_lock = copy.deepcopy(prior_integration["current_source"]["ledger"])
    expected_lock.update(sha256=sha(current_raw), revision=current["ledger_revision"])
    if active_lock != expected_lock:
        raise ValueError("implementation relocation changed integration standing")
    return restored_receipt


def verify_source_comparator_reconciliation(root: Path, receipt: dict) -> None:
    """Admit an executed source-comparator successor without restamping old replays."""
    retained = m.read(root / "fixtures/kernel/source-requalification/m3-source-audit.json")
    lock = m.read(root / "fixtures/kernel/m3-source-bindings-v1.json")
    if m.digest(m.canonical(retained)) == lock["audit_sha256"]:
        return
    record = receipt.get("source_comparator_reconciliation")
    if not isinstance(record, dict) or record.get("schema") != "ql.source-comparator-reconciliation/v1":
        raise ValueError("Changed source comparator requires its own executed audit reconciliation")
    prior = json.loads(subprocess.check_output(["git", "show", record["prior_receipt_git_commit"] + ":" + str(RECEIPT)], cwd=root))
    restored = copy.deepcopy(receipt)
    restored.pop("source_comparator_reconciliation")
    inputs = record["inputs"]
    if {row["path"] for row in inputs} != {"scripts/m3-source-parity.py", "fixtures/kernel/m3-source-bindings-v1.json"} or len(inputs) != 2:
        raise ValueError("Comparator reconciliation exceeded its two source inputs")
    for row in inputs:
        path = row["path"]
        old = subprocess.check_output(["git", "show", record["prior_input_git_commit"] + ":" + path], cwd=root)
        if sha(old) != row["before_sha256"] or prior["current_input_locks"][path] != row["before_sha256"]:
            raise ValueError("Comparator reconciliation lost its prior source input: " + path)
        if sha((root / path).read_bytes()) != row["after_sha256"] or receipt["current_input_locks"][path] != row["after_sha256"]:
            raise ValueError("Comparator reconciliation is stale: " + path)
        restored["current_input_locks"][path] = row["before_sha256"]
    if restored != prior:
        raise ValueError("Comparator reconciliation rewrote retained native/decision/proof qualification")
    for key in ("current_audit", "actual_source_tests"):
        if m.lock(root, record[key]["path"]) != {k:record[key][k] for k in ("path", "sha256")}:
            raise ValueError("Comparator execution artifact changed: " + key)
    current = m.read(root / record["current_audit"]["path"])
    if m.digest(m.canonical(current)) != lock["audit_sha256"] or current["projection_sha256"] != retained["projection_sha256"] \
            or current["source_revision"] != retained["source_revision"] or current["registry_revision"] != retained["registry_revision"] \
            or current["details"]["genetics"] != retained["details"]["genetics"]:
        raise ValueError("Comparator audit changed original source/native coin basis or differs from its current lock")
    old_ids = {row["id"] for row in retained["discrepancies"]}
    preserved = [row for row in current["discrepancies"] if row["id"] in old_ids]
    additional = [row for row in current["discrepancies"] if row["id"] not in old_ids]
    if preserved != retained["discrepancies"] or len(preserved) != record["preserved_legacy_findings"] or len(additional) != record["additional_open_qualified_edge_records"]:
        raise ValueError("Comparator audit changed or omitted retained source findings")
    if any(row["state"] != "open" or row["decision"] is not None or row["proposal"] is not None \
           or json.loads(row["detail"])["code"] != "line-change-qualification" for row in additional):
        raise ValueError("Comparator audit silently decided a new source discrepancy")
    if m.lock(root, record["actual_source_tests"]["source"]["path"]) != record["actual_source_tests"]["source"]:
        raise ValueError("Comparator test source changed after the recorded source suite")
    log = (root / record["actual_source_tests"]["path"]).read_text()
    if not re.search(r"Ran " + str(record["actual_source_tests"]["tests"]) + r" tests in [0-9.]+s\s+OK\s*$", log):
        raise ValueError("Comparator tests did not actually pass the recorded full source suite")


def check() -> None:
    receipt = m.read(ROOT / RECEIPT)
    ledger = m.read(ROOT / m.LEDGER)
    registry = m.read(ROOT / m.REGISTRY)
    if receipt["ledger_revision"] != ledger["ledger_revision"] or receipt["registry_revision"] != registry["registry_revision"]:
        raise ValueError("source requalification receipt is stale")
    for path, digest in receipt["current_input_locks"].items():
        if m.lock(ROOT, path)["sha256"] != digest:
            raise ValueError("source requalification input changed: " + path)
    for lock in receipt["replays"]:
        if m.lock(ROOT, lock["path"]) != lock:
            raise ValueError("requalification replay changed: " + lock["path"])
    for path, digest in receipt["historical_numerical_proofs_unchanged"].items():
        verify_historical_proof(ROOT, path, digest, receipt["historical"]["git_commit"])
    history(receipt["historical"]["git_commit"])
    prior_qualification = verify_implementation_relocation(ROOT, receipt)
    verify_source_comparator_reconciliation(ROOT, prior_qualification)
    m.verify(ROOT, ledger)
    print("Current source requalification, preserved historical proofs and131 decisions verified")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("apply", "check"))
    parser.add_argument("--previous-git-ref")
    parser.add_argument("--m1-c-log", type=Path)
    parser.add_argument("--rust-log", type=Path)
    parser.add_argument("--m3-audit", type=Path)
    args = parser.parse_args()
    if args.command == "check":
        check()
    else:
        if not all((args.previous_git_ref, args.m1_c_log, args.rust_log, args.m3_audit)):
            parser.error("apply needs the exact prior Git ref and three actual replay artifacts")
        apply(args)


if __name__ == "__main__":
    main()
