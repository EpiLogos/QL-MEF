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
STANDING = ("current source/application revalidation; numerical readiness and parity are claimed only where an "
            "engine's published successor proof at the current registry backs them (successor_verticals)")
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


LINEAGE = Path("fixtures/kernel/k8-build-lineage-v1.json")


def lineage_chain(lineage: dict, engine: str, head: str, bases: set[str]) -> str:
    """Walk an engine's published successors back from ``head`` to one of ``bases``."""
    if lineage["schema"] != "ql.k8-build-lineage/v1" or lineage["proofs"][engine] != head:
        raise ValueError("active historical proof is not the published lineage proof: " + engine)
    current, seen = head, set()
    while current not in bases:
        entries = [e for e in lineage["successors"] if e["engine"] == engine and e["sha256"] == current]
        if (len(entries) != 1 or current in seen or not entries[0].get("reason", "").strip()
                or not re.fullmatch(r"[0-9a-f]{40}", entries[0].get("accepted_at_revision", ""))):
            raise ValueError("native historical lineage missing or forged: successor chain of " + engine)
        seen.add(current)
        current = entries[0]["previous_sha256"]
    return current


ENGINE_OWNERS = {"m2": ("scripts/m2-ledger.py", "k6-m2:"), "m3": ("scripts/m3-ledger.py", "k7-m3:")}
UNASSESSED = ("unassessed", "unimplemented")


def load_owner(root: Path, engine: str):
    """The engine's own ledger owner, loaded the way ``m-ledger.py`` is."""
    owner_spec = importlib.util.spec_from_file_location(f"{engine}_ledger_owner", root / ENGINE_OWNERS[engine][0])
    module = importlib.util.module_from_spec(owner_spec)
    owner_spec.loader.exec_module(module)
    return module


def historical_bases(root: Path, engine: str, digest: str) -> set[str]:
    """Proofs already qualified here: the historical original and, for M3, the published alias."""
    bases = {digest}
    if engine == "m3":
        record = json.loads((root / "fixtures/kernel/m3-journey-source-integration-v1.json").read_text())
        bases.add(record["native_history"]["successor"]["sha256"])
    return bases


def successor_engines(root: Path, proof_locks: dict[str, str]) -> dict[str, dict]:
    """Engines whose active proof is a published successor at the current registry.

    Three conditions, each refused separately: the active proof is the one the
    build lineage publishes and differs from the historical original; the
    lineage walks back from it to a proof qualified here; and the proof's own
    input locks are the current files, binding the current registry (directly
    when it locks the tree, otherwise through a registry-stamped input).
    An engine failing any of them is not composed and stays unassessed.
    """
    registry = m.read(root / m.REGISTRY)
    tree_sha = sha((root / m.REGISTRY).read_bytes())
    lineage = json.loads((root / LINEAGE).read_bytes())
    found = {}
    for engine in ENGINE_OWNERS:
        path = f"fixtures/kernel/{engine}-finite-proof-v1.json"
        original = proof_locks[path]
        active = sha((root / path).read_bytes())
        if active == original:
            continue
        try:
            base = lineage_chain(lineage, engine, active, historical_bases(root, engine, original))
        except ValueError:
            continue
        proof = json.loads((root / path).read_text())
        inputs = proof.get("inputs", {})
        if not inputs or proof.get("result") != "passed" \
                or any(sha((root / name).read_bytes()) != digest for name, digest in inputs.items()):
            continue
        if m.REGISTRY.as_posix() in inputs:
            bound = inputs[m.REGISTRY.as_posix()] == tree_sha
        else:
            bound = any(name.endswith(".json") and json.loads((root / name).read_text()).get("registry_revision")
                        == registry["registry_revision"] for name in inputs)
        if not bound:
            continue
        accepted = next(e["accepted_at_revision"] for e in lineage["successors"]
                        if e["engine"] == engine and e["sha256"] == active)
        found[engine] = {"proof": path, "proof_sha256": active, "historical_base_sha256": base,
                         "accepted_at_revision": accepted}
    return found


def verify_added_discrepancies(root: Path, prior: dict, ledger: dict, composed) -> list[str]:
    """Records a composed engine's owner adds beside the historical decisions.

    The historical decisions are compared by id elsewhere. Anything else must
    be open and undecided, and must be that engine's own: its prefix, or, for
    M3, a finding template of its source catalogue. Returns the added ids.
    """
    known = {d["id"] for d in prior["discrepancies"]}
    allowed_prefixes = tuple(ENGINE_OWNERS[e][1] for e in composed)
    templates = set()
    if "m3" in composed:
        templates = {d["id"] for d in json.loads((root / "fixtures/kernel/m3-domain-v1.json").read_text())["discrepancies"]}
    added = [d for d in ledger["discrepancies"] if d["id"] not in known]
    for d in added:
        if not (d["id"].startswith(allowed_prefixes) or d["id"] in templates):
            raise ValueError("composition added a discrepancy no composed engine owns: " + d["id"])
        if d["state"] != "open" or d["decision"] is not None or d["proposal"] is not None or d.get("promotion") is not None:
            raise ValueError("composition added a decided discrepancy: " + d["id"])
    if len({d["id"] for d in ledger["discrepancies"]}) != len(ledger["discrepancies"]):
        raise ValueError("duplicate discrepancy ids after composition")
    return [d["id"] for d in added]


def verify_readiness(root: Path, receipt: dict, ledger: dict) -> None:
    """Readiness stands only where a composed engine's successor evidence backs it.

    Every assessment outside a composed engine's own vertical is unassessed or
    unimplemented with no evidence and no parity. Inside a composed vertical,
    each claim cites only that vertical's evidence, whose artifacts lock the
    current files; a tested claim cites the successor test-receipt that locks
    the current proof file named by the receipt.
    """
    composed = receipt.get("successor_verticals")
    if composed is None or set(composed) - set(ENGINE_OWNERS):
        raise ValueError("receipt does not record its successor verticals")
    if composed != successor_engines(root, receipt["historical_numerical_proofs_unchanged"]):
        raise ValueError("composed successor vertical is not a published successor at the current registry")
    evidence = {e["id"]: e for e in ledger["evidence"]}
    prefixes = {ENGINE_OWNERS[e][1]: e for e in composed}
    for ident, assessment in ledger["assessments"].items():
        if ident == "k2-index":
            continue
        engine = next((prefixes[p] for p in prefixes if ident.startswith(p)), None)
        claims = list(assessment["readiness"].values())
        pairs = [pair for group in assessment["parity"].values() for pair in group]
        if engine is None:
            if any(c["status"] not in UNASSESSED or c["evidence"] for c in claims) or pairs:
                raise ValueError("readiness without successor evidence: " + ident)
            continue
        proof_lock = {"path": composed[engine]["proof"], "sha256": composed[engine]["proof_sha256"]}
        for claim in claims + pairs:
            if claim["status"] in UNASSESSED and not claim["evidence"]:
                continue
            if not claim["evidence"]:
                raise ValueError("readiness without evidence: " + ident)
            for ref in claim["evidence"]:
                item = evidence.get(ref)
                if item is None or not ref.startswith(ENGINE_OWNERS[engine][1]) \
                        or m.lock(root, item["artifact"]["path"]) != item["artifact"]:
                    raise ValueError("readiness cites evidence outside its successor vertical: " + ident)
                if claim["warrant"] == "tested" and (
                        item["kind"] != "test-receipt" or item["artifact"] != proof_lock or item["result"] != "passed"):
                    raise ValueError("tested readiness is not backed by the successor proof: " + ident)
    for engine, record in composed.items():
        if not any(e["kind"] == "test-receipt" and e["id"].startswith(ENGINE_OWNERS[engine][1])
                   and e["artifact"] == {"path": record["proof"], "sha256": record["proof_sha256"]}
                   for e in ledger["evidence"]):
            raise ValueError("composed vertical has no successor test-receipt: " + engine)
    for e in ledger["evidence"]:
        for engine, (_, prefix) in ENGINE_OWNERS.items():
            if e["id"].startswith(prefix) and engine not in composed:
                raise ValueError("successor evidence for an engine that is not composed: " + e["id"])


JOURNEY_RECORD = Path("fixtures/kernel/m3-journey-source-integration-v1.json")
JOURNEY_LEDGER_STANDING = ("byte-identical current131 decisions retained; broad readiness unassessed except "
                          "engines with a published successor proof at the current registry")


def journey_source_standing(root: Path) -> dict:
    """The current admitted source, registries and ledger a published journey record stands on."""
    registry = m.read(root / m.REGISTRY)
    coordinates = m.read(root / "fixtures/kernel/k8-structure-receipt-v1.json")
    structure_spec = importlib.util.spec_from_file_location("historical_alias_current_structure", root / "scripts/k8-structure.py")
    structure = importlib.util.module_from_spec(structure_spec)
    structure_spec.loader.exec_module(structure)
    projected, _ = structure.project(root)
    if coordinates["registry_revision"] != projected["registry_revision"]:
        raise ValueError("historical integration record has stale current source standing")
    ledger_bytes = (root / m.LEDGER).read_bytes()
    return {"source_revision": registry["source_revision"],
            "numerical_registry_revision": registry["registry_revision"],
            "coordinate_registry_revision": coordinates["registry_revision"],
            "ledger": {"path": m.LEDGER.as_posix(), "sha256": sha(ledger_bytes),
                       "revision": json.loads(ledger_bytes)["ledger_revision"], "standing": JOURNEY_LEDGER_STANDING}}


def advance_journey_source_standing() -> None:
    """Stand the published journey record on the requalified owners; its prior standing stays dated."""
    path = ROOT / JOURNEY_RECORD
    raw = path.read_bytes()
    record = json.loads(raw)
    current = journey_source_standing(ROOT)
    if record["current_source"] == current:
        return
    published = subprocess.check_output(["git", "log", "-1", "--format=%H", "HEAD", "--", JOURNEY_RECORD.as_posix()],
                                        cwd=ROOT, text=True).strip()
    if git_bytes(published, JOURNEY_RECORD.as_posix()) != raw:
        raise ValueError("journey integration record differs from its published Git source")
    record["current_source_history"].append({
        "source": record["current_source"], "git_ref": f"git:{published}:{JOURNEY_RECORD.as_posix()}",
        "standing": "Dated published source standing; preserved, not retargeted numerical evidence"})
    record["current_source"] = current
    path.write_text(json.dumps(record, indent=2, ensure_ascii=False))


def verify_historical_proof(root: Path, path: str, digest: str, commit: str, *, standing: bool = True) -> None:
    """Retain the qualified original while admitting a published history alias.

    This checks source lineage only. It does not requalify current numerical
    inputs, publish a new finite proof, or confer numerical readiness. Without
    ``standing`` the alias lineage is checked but not the record's current
    source standing, which a requalification advances after it writes the ledger.
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
    engine = re.fullmatch(r"fixtures/kernel/(m[23])-finite-proof-v1\.json", path)
    if not engine:
        raise ValueError("unqualified historical proof alias: " + path)
    engine = engine[1]

    record = alias = None
    if engine == "m3":
        record = json.loads(checked_path("fixtures/kernel/m3-journey-source-integration-v1.json").read_text())
        if record["schema"] != "ql.published-journey-source-integration/v1":
            raise ValueError("unqualified historical proof integration record")
        alias = record["native_history"]["successor"]["sha256"]
    if sha(active) != alias:
        # A later successor stands on the lineage its owner published; walk it
        # back to a proof already qualified here, the original or the alias.
        lineage = json.loads(checked_path(LINEAGE.as_posix()).read_bytes())
        published = json.loads(subprocess.check_output(["git", "show", f"{commit}:{LINEAGE.as_posix()}"], cwd=root))
        if lineage["successors"][:len(published["successors"])] != published["successors"]:
            raise ValueError("native historical lineage missing or forged")
        reached = lineage_chain(lineage, engine, sha(active), {digest} | ({alias} if alias else set()))
        if reached == digest:
            return
        alias_bytes = git_source(record["native_history"]["successor"]["git_ref"])
    else:
        alias_bytes = active

    if standing and journey_source_standing(root) != record["current_source"]:
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
            or successor["sha256"] != sha(alias_bytes)
            or git_source(expected_ref) != alias_bytes):
        raise ValueError("active historical successor differs from published Git source")

    # The alias is qualified by the lineage as it was published with it; the
    # current lineage may only extend that.
    lock = history["native_lineage"]
    lineage_bytes = git_source(f"git:{record['parents']['incoming']}:{lock['path']}")
    current_lineage = json.loads(checked_path(lock["path"]).read_bytes())
    if (lock["path"] != LINEAGE.as_posix()
            or sha(lineage_bytes) != lock["sha256"]
            or current_lineage["successors"][:len(json.loads(lineage_bytes)["successors"])]
            != json.loads(lineage_bytes)["successors"]):
        raise ValueError("native historical lineage missing or forged")
    lineage = json.loads(lineage_bytes)
    pointer = re.fullmatch(r"/successors/(\d+)", lock["pointer"])
    if not pointer or int(pointer[1]) >= len(lineage["successors"]):
        raise ValueError("unqualified native historical lineage pointer")
    entry = lineage["successors"][int(pointer[1])]
    if (lineage["schema"] != "ql.k8-build-lineage/v1"
            or lineage["proofs"]["m3"] != sha(alias_bytes)
            or entry["engine"] != "m3" or entry["previous_sha256"] != digest
            or entry["sha256"] != sha(alias_bytes)
            or entry["accepted_at_revision"] != history["published_ci"]["revision"]):
        raise ValueError("native historical successor is not qualified by its original")

    before, after = json.loads(original), json.loads(alias_bytes)
    changed_input = "crates/ql-core/src/pole/tarot.rs"
    if history["only_proof_input_change"] != "/inputs/" + changed_input:
        raise ValueError("unqualified historical successor input change")
    restored = copy.deepcopy(after)
    restored["inputs"][changed_input] = before["inputs"][changed_input]
    if restored != before or after["inputs"][changed_input] == before["inputs"][changed_input]:
        raise ValueError("historical successor changes more than its naming input")
    if alias_bytes == active and sha(checked_path(changed_input).read_bytes()) != after["inputs"][changed_input]:
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


def build(args, base: dict | None = None, write_replays: bool = True) -> tuple[dict, dict]:
    """Compute the requalified ledger and its receipt, writing nothing but the replay copies.

    ``base`` is the ledger to start from (default: the current one); a test
    passes a stripped one and ``write_replays=False`` to leave the tree alone.
    """
    prior, old = history(args.previous_git_ref)
    registry = m.read(ROOT / m.REGISTRY)
    if prior["registry"]["revision"] == registry["registry_revision"]:
        raise ValueError("source requalification requires a distinct admitted registry")
    c_log, rust_log = args.m1_c_log.read_bytes(), args.rust_log.read_bytes()
    audit = m.read(args.m3_audit)
    verify_replays(c_log, rust_log, audit, registry)
    # Locks are the historical originals; an active file may differ only by its
    # published, lineage-qualified successor.
    proof_locks = {p.as_posix(): sha(git_bytes(old["git_commit"], p.as_posix())) for p in PROOF_FILES}
    for path, digest in proof_locks.items():
        verify_historical_proof(ROOT, path, digest, old["git_commit"], standing=False)
    applied = [d for d in prior["discrepancies"] if d["state"] == "applied"]
    if len(applied) != 131:
        raise ValueError("inspect the changed decision inventory before requalification")
    if write_replays:
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
    # The current ledger carries the rows, implementations and matrices
    # reconciled since the historical base; its decisions are the historical
    # ones, re-evidenced below and compared with that base.
    base = base if base is not None else m.read(ROOT / m.LEDGER)
    # The base may already carry a composed engine's own open records (re-running
    # over this tool's output); they are validated against the composed engines
    # below. Every historical decision must lead, in order.
    if [d["id"] for d in base["discrepancies"][:len(prior["discrepancies"])]] != [d["id"] for d in prior["discrepancies"]]:
        raise ValueError("inspect the changed decision inventory before requalification")
    ledger = m.refresh(ROOT, copy.deepcopy(base))
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
    for before, d in zip(prior["discrepancies"], ledger["discrepancies"]):
        for part in ("decision", "proposal"):
            if before.get(part) and (part == "decision" or before["state"] == "applied"):
                d[part]["evidence"] = [mapping[ref] for ref in before[part]["evidence"]]
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
    # Full successor execution re-enters through the engines' own owners: an
    # engine whose active proof is a published successor at the current
    # registry has its vertical materialised; any other stays stripped.
    composed = successor_engines(ROOT, proof_locks)
    for engine in composed:
        ledger = load_owner(ROOT, engine).materialize(ledger)
    current = {d["id"]: d for d in ledger["discrepancies"]}
    for before in prior["discrepancies"]:
        if before["id"] not in current:
            raise ValueError("successor composition dropped a prior decision: " + before["id"])
        restored = copy.deepcopy(current[before["id"]])
        for part in ("decision", "proposal"):
            if before.get(part):
                restored[part]["evidence"] = before[part]["evidence"]
        if restored != before:
            raise ValueError("successor composition rewrote a prior decision: " + before["id"])
    verify_added_discrepancies(ROOT, prior, ledger, composed)
    ledger["ledger_revision"] = m.digest(m.canonical({k: v for k, v in ledger.items() if k != "ledger_revision"}))
    m.verify(ROOT, ledger)
    receipt = {"schema": "ql.m-ledger-source-requalification/v1", "source_revision": registry["source_revision"],
               "registry_revision": registry["registry_revision"], "ledger_revision": ledger["ledger_revision"],
               "historical": old, "current_input_locks": {p: m.lock(ROOT, p)["sha256"] for p in INPUTS},
               "replays": [m.lock(ROOT, REPLAYS / p) for p in ("m1-native-c-replay.log", "native-rust-c-replay.log", "m3-source-audit.json")],
               "evidence_mapping": mapping, "preserved_applied_decisions": len(applied),
               "historical_numerical_proofs_unchanged": proof_locks,
               "successor_verticals": composed,
               "standing": STANDING,
               "not_claimed": ["current whole M1/M2/M3 numerical readiness from this bootstrap", "installed visual/audio acceptance",
                               "readiness for any engine without a published successor proof at the current registry"]}
    verify_readiness(ROOT, receipt, ledger)
    return ledger, receipt


def apply(args) -> None:
    ledger, receipt = build(args)
    proof_locks, old = receipt["historical_numerical_proofs_unchanged"], receipt["historical"]
    registry, applied = receipt, receipt["preserved_applied_decisions"]
    (ROOT / RECEIPT).write_text(m.encode(receipt))
    (ROOT / m.LEDGER).write_text(m.encode(ledger))
    advance_journey_source_standing()
    for path, digest in proof_locks.items():
        verify_historical_proof(ROOT, path, digest, old["git_commit"])
    print(json.dumps({"result": "source-requalified", "ledger_revision": ledger["ledger_revision"],
                      "registry_revision": registry["registry_revision"], "preserved_applied_decisions": applied}))


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
    verify_source_comparator_reconciliation(ROOT, receipt)
    m.verify(ROOT, ledger)
    if receipt["standing"] != STANDING:
        raise ValueError("source requalification standing is not the current claim")
    verify_readiness(ROOT, receipt, ledger)
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
