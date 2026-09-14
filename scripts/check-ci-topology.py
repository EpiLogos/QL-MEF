#!/usr/bin/env python3
"""Guard Quaternal Logic's CI topology against workflow drift.

The suite law is deliberately structural:

* ql-mef-rust.yml is the only pull-request workflow;
* product-local acceptance runs from accepted main (and may be dispatched);
* workflows that read sibling repositories are reusable workers and are
  commissioned by the weekly/on-demand cross-product workflow;
* maintenance/release workflows remain outside pull-request verification.

This checker owns routing only. Individual workflows retain ownership of what
their evidence means.
"""
from __future__ import annotations

from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
WORKFLOWS = ROOT / ".github" / "workflows"
CANONICAL_PR_GATE = "ql-mef-rust.yml"
CROSS_PRODUCT = "cross-product.yml"
CROSS_PRODUCT_WORKERS = {
    "aw-field.yml",
    "epi-bimba-map-conformance.yml",
    "epi-c-r1-r2.yml",
    "epi-c-reference.yml",
    "epi-vak-source-parity.yml",
    "kernel-k8-continuous.yml",
    "kernel-m2.yml",
    "kernel-m3-engine.yml",
    "kernel-rebuild-source-ground.yml",
    "m3-engine.yml",
    "ux-intent.yml",
}


def has_event(text: str, event: str) -> bool:
    return re.search(rf"(?m)^  {re.escape(event)}:\s*$", text) is not None


def fail(message: str) -> None:
    raise SystemExit(f"CI topology violation: {message}")


def main() -> int:
    files = {path.name: path for path in WORKFLOWS.glob("*.yml")}
    if CANONICAL_PR_GATE not in files:
        fail(f"missing canonical PR gate {CANONICAL_PR_GATE}")
    if CROSS_PRODUCT not in files:
        fail(f"missing cross-product coordinator {CROSS_PRODUCT}")

    pr_workflows = []
    for name, path in sorted(files.items()):
        text = path.read_text(encoding="utf-8")
        if has_event(text, "pull_request"):
            pr_workflows.append(name)

        reads_sibling = re.search(r"(?m)^\s+repository:\s*EpiLogos/(?!QL-MEF\s*$)", text) is not None
        if reads_sibling and name not in CROSS_PRODUCT_WORKERS and name != CROSS_PRODUCT:
            fail(f"{name} reads a sibling repository outside the cross-product office")

    if pr_workflows != [CANONICAL_PR_GATE]:
        fail(
            "pull-request execution must be owned only by "
            f"{CANONICAL_PR_GATE}; observed {pr_workflows}"
        )

    gate = files[CANONICAL_PR_GATE].read_text(encoding="utf-8")
    if not has_event(gate, "pull_request"):
        fail("canonical PR gate no longer listens to pull_request")
    if "cancel-in-progress: ${{ github.event_name == 'pull_request' }}" not in gate:
        fail("canonical PR gate must cancel superseded PR heads")

    coordinator = files[CROSS_PRODUCT].read_text(encoding="utf-8")
    if not has_event(coordinator, "schedule") or not has_event(coordinator, "workflow_dispatch"):
        fail("cross-product coordinator must be weekly and manually dispatchable")
    if has_event(coordinator, "pull_request") or has_event(coordinator, "push"):
        fail("cross-product coordinator must not run per push or pull request")

    for worker in sorted(CROSS_PRODUCT_WORKERS):
        path = files.get(worker)
        if path is None:
            fail(f"missing cross-product worker {worker}")
        text = path.read_text(encoding="utf-8")
        if not has_event(text, "workflow_call") or not has_event(text, "workflow_dispatch"):
            fail(f"{worker} must expose workflow_call + workflow_dispatch")
        for forbidden in ("pull_request", "push", "schedule"):
            if has_event(text, forbidden):
                fail(f"{worker} must not own {forbidden}; cross-product.yml commissions it")
        call = f"uses: ./.github/workflows/{worker}"
        if call not in coordinator:
            fail(f"cross-product.yml does not commission {worker}")

    print("CI topology: PASS")
    print(f"pull-request gate: {CANONICAL_PR_GATE}")
    print(f"cross-product workers: {len(CROSS_PRODUCT_WORKERS)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
