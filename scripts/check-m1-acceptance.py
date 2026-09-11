#!/usr/bin/env python3
"""Check K5 source locks and scoped reviewed dispositions, not a success stamp.

Execution belongs to the native tests. This verifier rejects stale receipts,
missing rows, rewritten source assertions and accidental promotion of providers.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RECEIPT = "docs/kernel-rebuild/m1-engine-acceptance-v1.json"
REGISTRY_REVISION = "259a2f496c5f3a76d31e5c480dc9afdb45ad1282a7034cc28c528c39a71442e4"


def verify(root: Path = ROOT) -> dict[str, int]:
    def load(path: str):
        return json.loads((root / path).read_text())

    receipt = load(RECEIPT)
    registry = load("fixtures/kernel/m-tree-v1.json")
    ledger = load("fixtures/kernel/m-ledger-v1.json")
    assert receipt["registry_revision"] == registry["registry_revision"] == REGISTRY_REVISION
    assert receipt["k4_revision"] == "5b24b95d17234ab5d23d84e658c0cc06434b41a3"
    assert receipt["source_return_revision"] == "bb47ab9730f0ddadd4891666fb6f3e0a6d457330"
    paths: set[str] = set()
    for lock in receipt["input_locks"]:
        path = lock["path"]
        assert path not in paths, f"duplicate input lock: {path}"
        paths.add(path)
        assert not Path(path).is_absolute() and ".." not in Path(path).parts
        assert hashlib.sha256((root / path).read_bytes()).hexdigest() == lock["sha256"], f"stale executed input: {path}"
    assert {"c/src/m1.c", "c/src/m1_state.c", "crates/ql-mef/src/m1.rs", "crates/ql-mef/src/m1_engine.rs", "migration/epi-kernel/m1-state-probe.c", "scripts/m_census.py"} <= paths
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
    assert {"M1_Root", "Quaternion", "_ananda_core", "_ananda_initialized", "quat_slerp", "get_quint_diff", "M1_M0_CROSSLINK"} <= witness_symbols
    assert "all" not in witness_symbols and "diff" not in witness_symbols
    return {"coordinates": len(m1), "source_capabilities": len(caps), "input_locks": len(paths), "retained_source_constructs": len(witness_symbols)}


if __name__ == "__main__":
    try:
        print(json.dumps(verify(), sort_keys=True))
    except (AssertionError, KeyError, OSError, ValueError) as e:
        raise SystemExit(f"M1 acceptance consistency: {e}") from e
