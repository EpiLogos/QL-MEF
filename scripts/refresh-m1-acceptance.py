#!/usr/bin/env python3
"""Revalidate M1 natively before deriving current acceptance input locks.

The historical source, capability dispositions and observation hashes are not
rewritten. Fresh C/Rust observations retain their own compiler-qualified hashes;
the independent native tests enforce the contract's exact and numeric tolerances.
Run with the track's persistent CARGO_TARGET_DIR, never a new agent build tree.
"""
from __future__ import annotations

import hashlib
import json
import os
import platform
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HISTORICAL = ROOT / "docs/kernel-rebuild/m1-engine-acceptance-v1.json"
RECEIPT = ROOT / "docs/kernel-rebuild/m1-engine-revalidation-v1.json"
SELF = "scripts/refresh-m1-acceptance.py"
COMMANDS = [
    ["bash", "scripts/test-m1-engine.sh", "--execute-for-receipt-refresh"],
    ["cargo", "test", "--locked", "-p", "ql-mef", "--test", "m1_engine", "--test", "m1_state"],
    ["cargo", "test", "--locked", "-p", "ql-cli", "--test", "m1_engine_cli"],
    ["python3", "-m", "unittest", "discover", "-s", "scripts/tests", "-p", "test_m_census.py", "-v"],
]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    if not os.environ.get("CARGO_TARGET_DIR"):
        raise SystemExit("Set CARGO_TARGET_DIR to the track's persistent Rust cache before revalidation.")
    historical_bytes = HISTORICAL.read_bytes()
    historical = json.loads(historical_bytes)
    original = RECEIPT.read_bytes() if RECEIPT.exists() else None
    paths = [lock["path"] for lock in historical["input_locks"]]
    for path in [SELF, "c/Makefile"]:
        if path not in paths:
            paths.append(path)
    before = {path: sha(ROOT / path) for path in paths}
    out = ROOT / "target/m1-engine/revalidation" / str(time.time_ns())
    out.mkdir(parents=True, exist_ok=True)
    for index, command in enumerate(COMMANDS, 1):
        log = out / f"{index}.log"
        print("Executing " + " ".join(command), flush=True)
        with log.open("w") as stream:
            result = subprocess.run(command, cwd=ROOT, stdout=stream, stderr=subprocess.STDOUT)
        if result.returncode:
            raise SystemExit(f"Native revalidation failed ({result.returncode}); receipt unchanged. Read {log}")
    observations = []
    for observation in historical["observations"]:
        path = ROOT / observation["local_path"]
        retained = out / (path.parent.name + "-" + path.name)
        retained.write_bytes(path.read_bytes())
        observations.append({"local_path": observation["local_path"], "sha256": sha(path),
                             "retained_path": str(retained.relative_to(ROOT)),
                             "matches_historical_bytes": sha(path) == observation["sha256"]})
    after = {path: sha(ROOT / path) for path in paths}
    current_bytes = RECEIPT.read_bytes() if RECEIPT.exists() else None
    if before != after or current_bytes != original or HISTORICAL.read_bytes() != historical_bytes:
        raise SystemExit("Acceptance source changed during execution; receipt unchanged.")
    receipt = {"schema": "ql.m1.kernel-revalidation/v1",
               "historical_acceptance": {"path": str(HISTORICAL.relative_to(ROOT)), "sha256": sha(HISTORICAL)}}
    receipt["input_locks"] = [{"path": path, "sha256": digest} for path, digest in before.items()]
    receipt["revalidation"] = {
        "standing": "fresh native execution under existing exact/literal and floating tolerances; historical observations retained separately",
        "platform": platform.system() + " " + platform.machine(),
        "c_compiler": subprocess.check_output([os.environ.get("CC", "clang"), "--version"], text=True).splitlines()[0],
        "rust_compiler": subprocess.check_output(["rustc", "--version"], text=True).strip(),
        "commands": COMMANDS,
        "observations": observations,
        "logs": [{"local_path": str((out / f"{index}.log").relative_to(ROOT)), "sha256": sha(out / f"{index}.log")}
                 for index in range(1, len(COMMANDS) + 1)],
    }
    candidate = json.dumps(receipt, ensure_ascii=False, indent=2) + "\n"
    RECEIPT.write_text(candidate)
    checked = subprocess.run(["python3", "scripts/check-m1-acceptance.py"], cwd=ROOT)
    if checked.returncode:
        if original is None:
            RECEIPT.unlink()
        else:
            RECEIPT.write_bytes(original)
        raise SystemExit("Derived acceptance failed consistency; original receipt restored.")
    print("M1 receipt revalidated by native execution; historical observations and dispositions preserved.")


if __name__ == "__main__":
    main()
