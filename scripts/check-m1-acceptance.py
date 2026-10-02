#!/usr/bin/env python3
"""Check K5 source locks and scoped reviewed dispositions, not a success stamp.

Execution belongs to the native tests. This verifier rejects stale receipts,
missing rows, rewritten source assertions and accidental promotion of providers.
"""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RECEIPT = "docs/kernel-rebuild/m1-engine-acceptance-v1.json"
CURRENT = "docs/kernel-rebuild/m1-engine-revalidation-v1.json"
REGISTRY_REVISION = "2264f5686abd1eb3192ecabd74457ca87086be8cea5f1f29de8b48d02151ef77"


def verify_same_registry(root: Path = ROOT) -> dict[str, int]:
    def load(path: str):
        return json.loads((root / path).read_text())

    receipt = load(RECEIPT)
    current = load(CURRENT)
    assert current["schema"] == "ql.m1.kernel-revalidation/v1"
    assert current["historical_acceptance"] == {
        "path": RECEIPT,
        "sha256": hashlib.sha256((root / RECEIPT).read_bytes()).hexdigest(),
    }, "current execution does not bind the unchanged historical acceptance"
    registry = load("fixtures/kernel/m-tree-v1.json")
    ledger = load("fixtures/kernel/m-ledger-v1.json")
    assert ledger["registry"]["revision"] == registry["registry_revision"], "ledger and accepted registry differ"
    assert receipt["registry_revision"] == registry["registry_revision"] == REGISTRY_REVISION
    assert receipt["k4_revision"] == "5b24b95d17234ab5d23d84e658c0cc06434b41a3"
    assert receipt["source_return_revision"] == "bb47ab9730f0ddadd4891666fb6f3e0a6d457330"
    paths: set[str] = set()
    for lock in current["input_locks"]:
        path = lock["path"]
        assert path not in paths, f"duplicate input lock: {path}"
        paths.add(path)
        assert not Path(path).is_absolute() and ".." not in Path(path).parts
        assert hashlib.sha256((root / path).read_bytes()).hexdigest() == lock["sha256"], f"stale executed input: {path}"
    assert {"c/src/m1.c", "c/src/m1_state.c", "crates/ql-mef/src/m1.rs", "crates/ql-mef/src/m1_engine.rs", "migration/epi-kernel/m1-state-probe.c", "scripts/m_census.py"} <= paths
    assert {lock["path"] for lock in receipt["input_locks"]} <= paths, "current execution omitted historical inputs"
    assert "scripts/refresh-m1-acceptance.py" in paths
    assert len(current["revalidation"]["commands"]) == len(current["revalidation"]["logs"]) == 4
    assert {item["local_path"] for item in current["revalidation"]["observations"]} == {item["local_path"] for item in receipt["observations"]}
    rows = {r["id"]: r for r in ledger["rows"]}
    impls = {i["id"]: i for i in ledger["implementations"]}
    m1 = [n for n in registry["nodes"] if n["root_position"] == 1]
    assert len(m1) == 43
    for node in m1:
        row = rows["census:" + node["source_ref"]]
        profile = ledger["assessments"][row["assessment"]]
        for peer, prefix in [("c", "c/src/"), ("rust", "crates/ql-mef/src/")]:
            assert profile["readiness"][peer]["status"] == "verified", row["id"]
            assert any(impls[b]["stratum"] == peer and impls[b]["path"].startswith(prefix) and node["source_ref"] in impls[b]["coordinates"] for b in row["bindings"]), row["id"]
        assert profile["readiness"]["instrument"]["status"] != "verified"
        assert profile["parity"]["experiential"] == []
    caps = [r for r in rows.values() if r["source"] and r["scope"] == "M1"]
    dispositions = {r["row"] for r in receipt["capability_dispositions"]}
    assert len(caps) == 26 and dispositions == {r["id"] for r in caps}
    for row in caps:
        assert row["assessment"].startswith("k5-m1:"), row["id"]
        assert all(row["dispositions"][p] in ["bound", "unimplemented"] for p in ["c", "rust"])
        # Each source assertion stays at its original matrix pointer. The shared
        # m-ledger check independently proves byte/current-import equality.
        assert row["invariants"] and row["source"]["pointer"]
    for cap in ["M1-C15", "M1-C17", "M1-C18", "M1-C19"]:
        row = rows["deep-M1:" + cap]
        assert not row["bindings"] and row["dispositions"]["rust"] == "unimplemented"
    discrepancies = [d for d in ledger["discrepancies"] if d["id"].startswith("k5-m1:")]
    assert len(discrepancies) == 4 and all(d["state"] == "applied" for d in discrepancies)
    assert all(d["decision"] for d in discrepancies)
    aliases = [d for d in ledger["discrepancies"] if d["id"].startswith("k4-live-spelling-composite-m1")]
    assert aliases and all(d["state"] == "rejected" for d in aliases)
    witness_symbols = {i["symbol"] for i in impls.values() if i["id"].startswith("k5-m1:source-witness:")}
    # _ananda_core/_ananda_initialized retired upstream (10.T10.10; ledger
    # k-move-m1:retired-lazy-ananda-core); the 12×12 tables are their successors.
    assert {"M1_Root", "Quaternion", "m1_ananda_get", "m1_ananda_dr_get", "quat_slerp", "get_quint_diff", "M1_M0_CROSSLINK"} <= witness_symbols
    assert "all" not in witness_symbols and "diff" not in witness_symbols
    return {"coordinates": len(m1), "source_capabilities": len(caps), "input_locks": len(paths), "retained_source_constructs": len(witness_symbols)}


def verify_requalified(root: Path) -> dict[str, int | str]:
    """Check current source standing, without borrowing historical readiness.

    The new source has its own exact tree locks and actual native replay. Those
    observations preserve applied decisions and executable bindings; they do
    not silently promote the old whole numerical or experiential assessments.
    """
    def load(path: str):
        return json.loads((root / path).read_text())

    receipt = load(RECEIPT)
    registry = load("fixtures/kernel/m-tree-v1.json")
    ledger = load("fixtures/kernel/m-ledger-v1.json")
    qualification = load("fixtures/kernel/m-ledger-source-requalification-v1.json")
    assert receipt["registry_revision"] == REGISTRY_REVISION, "historical acceptance source changed"
    assert registry["registry_revision"] != REGISTRY_REVISION, "a source qualification needs a distinct registry"
    assert qualification["schema"] == "ql.m-ledger-source-requalification/v1"
    assert qualification["source_revision"] == registry["source_revision"], "source qualification is for a different source"
    assert qualification["registry_revision"] == ledger["registry"]["revision"] == registry["registry_revision"], "source qualification is for a different registry"
    assert qualification["ledger_revision"] == ledger["ledger_revision"], "source qualification is for a different ledger"
    assert qualification["preserved_applied_decisions"] == 131
    assert qualification["standing"] == "current source/application revalidation; old numerical readiness/parity unassessed until full successor execution"
    assert "current whole M1/M2/M3 numerical readiness from this bootstrap" in qualification["not_claimed"]

    spec = importlib.util.spec_from_file_location("m1_source_qualification", root / "scripts/requalify-m-ledger.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    prior, historical = module.history(qualification["historical"]["git_commit"])
    assert historical == qualification["historical"], "historical evidence was rewritten"
    assert len(prior["discrepancies"]) == len(ledger["discrepancies"]), "historical decisions were omitted or added without review"
    for before, after in zip(prior["discrepancies"], ledger["discrepancies"]):
        restored = copy.deepcopy(after)
        for part in ["decision", "proposal"]:
            original = before.get(part)
            if original and (part == "decision" or before["state"] == "applied"):
                expected = [qualification["evidence_mapping"][ref] for ref in original["evidence"]]
                assert after[part]["evidence"] == expected, "current decision has unqualified evidence: " + before["id"]
                restored[part]["evidence"] = original["evidence"]
        assert restored == before, "historical decision/lifecycle changed: " + before["id"]
    historical_registry = json.loads(subprocess.check_output(
        ["git", "show", historical["git_commit"] + ":fixtures/kernel/m-tree-v1.json"], cwd=root))
    assert historical_registry["registry_revision"] == receipt["registry_revision"]
    for path in [RECEIPT, CURRENT]:
        original = subprocess.check_output(["git", "show", historical["git_commit"] + ":" + path], cwd=root)
        assert (root / path).read_bytes() == original, "historical acceptance/revalidation changed: " + path
    accepted = next(e for e in historical["evidence"] if e["id"] == "k5-m1:accepted-execution")
    assert accepted["artifact"] == {"path": RECEIPT, "sha256": hashlib.sha256((root / RECEIPT).read_bytes()).hexdigest()}
    assert accepted["registry_revision"] == receipt["registry_revision"]

    required = {
        "fixtures/kernel/m-tree-v1.json", "c/registry/m-tree-source-v1.json", "c/src/m_tree_data.inc",
        "crates/ql-mef/src/m_tree.rs", "crates/ql-mef/tests/native_m_tree.rs", "c/src/m_tree.c",
        "c/src/m1.c", "c/src/m1_state.c", "crates/ql-mef/src/m1.rs", "crates/ql-mef/src/m1_engine.rs",
        "crates/ql-mef/tests/m1_engine.rs", "crates/ql-mef/tests/m1_state.rs",
    }
    locks = qualification["current_input_locks"]
    assert required <= locks.keys(), "current qualification omitted registry/native inputs"
    for path, digest in locks.items():
        assert not Path(path).is_absolute() and ".." not in Path(path).parts
        assert hashlib.sha256((root / path).read_bytes()).hexdigest() == digest, "stale current source input: " + path
    replay_paths = {
        "fixtures/kernel/source-requalification/m1-native-c-replay.log",
        "fixtures/kernel/source-requalification/native-rust-c-replay.log",
        "fixtures/kernel/source-requalification/m3-source-audit.json",
    }
    assert {item["path"] for item in qualification["replays"]} == replay_paths
    for item in qualification["replays"]:
        assert hashlib.sha256((root / item["path"]).read_bytes()).hexdigest() == item["sha256"], "current replay changed: " + item["path"]
    module.verify_replays(
        (root / "fixtures/kernel/source-requalification/m1-native-c-replay.log").read_bytes(),
        (root / "fixtures/kernel/source-requalification/native-rust-c-replay.log").read_bytes(),
        load("fixtures/kernel/source-requalification/m3-source-audit.json"), registry)
    replay = (root / "fixtures/kernel/source-requalification/native-rust-c-replay.log").read_text()
    expected_tests = set()
    for path in ["crates/ql-mef/tests/m1_engine.rs", "crates/ql-mef/tests/m1_state.rs",
                 "crates/ql-mef/tests/native_m_tree.rs"]:
        expected_tests.update(re.findall(r"#\[test\]\s*fn\s+(\w+)\s*\(", (root / path).read_text()))
    passed_tests = re.findall(r"^test (\w+) \.\.\. ok$", replay, re.MULTILINE)
    assert len(expected_tests) == 21 and expected_tests <= set(passed_tests), "native replay omitted locked M1/tree tests"
    assert len(passed_tests) == len(set(passed_tests)) == 23, "native replay test inventory differs"
    assert re.findall(r"^test result: ok\. (\d+) passed; 0 failed;", replay, re.MULTILINE) == ["2", "8", "8", "5"]
    for path, digest in qualification["historical_numerical_proofs_unchanged"].items():
        module.verify_historical_proof(root, path, digest, historical["git_commit"])
    rows = {r["id"]: r for r in ledger["rows"]}
    impls = {i["id"]: i for i in ledger["implementations"]}
    m1 = [n for n in registry["nodes"] if n["root_position"] == 1]
    assert len(m1) == 43
    for node in m1:
        row = rows["census:" + node["source_ref"]]
        profile = ledger["assessments"][row["assessment"]]
        for peer, prefix in [("c", "c/src/"), ("rust", "crates/ql-mef/src/")]:
            assert profile["readiness"][peer] == {"status": "unassessed", "warrant": "unassessed", "evidence": []}, "historical readiness promoted: " + row["id"]
            assert any(impls[b]["kind"] == "computational" and impls[b]["stratum"] == peer
                       and impls[b]["path"].startswith(prefix) and node["source_ref"] in impls[b]["coordinates"]
                       for b in row["bindings"]), "current native binding disappeared: " + row["id"]
        assert all(not pairs for pairs in profile["parity"].values()), "historical parity promoted: " + row["id"]
        assert profile["readiness"]["instrument"]["status"] != "verified"
    caps = [r for r in rows.values() if r["source"] and r["scope"] == "M1"]
    assert len(caps) == 26
    assert {r["row"] for r in receipt["capability_dispositions"]} == {r["id"] for r in caps}
    for row in caps:
        assert row["assessment"].startswith("k5-m1:") and row["invariants"] and row["source"]["pointer"]
        assert all(row["dispositions"][p] in ["bound", "unimplemented"] for p in ["c", "rust"])
        profile = ledger["assessments"][row["assessment"]]
        assert all(c["status"] in ["unassessed", "unimplemented"] and not c["evidence"] for c in profile["readiness"].values())
        assert all(not pairs for pairs in profile["parity"].values())
    for cap in ["M1-C15", "M1-C17", "M1-C18", "M1-C19"]:
        row = rows["deep-M1:" + cap]
        assert not row["bindings"] and row["dispositions"]["rust"] == "unimplemented"
    discrepancies = [d for d in ledger["discrepancies"] if d["id"].startswith("k5-m1:")]
    assert len(discrepancies) == 4 and all(d["state"] == "applied" and d["decision"] for d in discrepancies)
    aliases = [d for d in ledger["discrepancies"] if d["id"].startswith("k4-live-spelling-composite-m1")]
    assert aliases and all(d["state"] == "rejected" for d in aliases)
    assert len([d for d in prior["discrepancies"] if d["state"] == "applied"]) == 131
    module.m.verify(root, ledger)
    return {"standing": "current-source-qualified; historical numerical readiness unassessed",
            "coordinates": len(m1), "source_capabilities": len(caps), "input_locks": len(locks),
            "preserved_applied_decisions": 131, "current_native_replay_tests": 23}


def verify(root: Path = ROOT) -> dict[str, int | str]:
    registry = json.loads((root / "fixtures/kernel/m-tree-v1.json").read_text())
    if registry["registry_revision"] == REGISTRY_REVISION:
        return verify_same_registry(root)
    return verify_requalified(root)


if __name__ == "__main__":
    try:
        print(json.dumps(verify(), sort_keys=True))
    except (AssertionError, KeyError, OSError, ValueError) as e:
        raise SystemExit(f"M1 acceptance consistency: {e}") from e
