#!/usr/bin/env python3
"""Small mutation-oriented smoke for the CI topology contract."""
from __future__ import annotations

from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def run() -> int:
    result = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "check-ci-topology.py")],
        cwd=ROOT,
        text=True,
        capture_output=True,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stdout)
        sys.stderr.write(result.stderr)
        return result.returncode
    if "CI topology: PASS" not in result.stdout:
        raise SystemExit("topology checker did not return its PASS receipt")
    print(result.stdout, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(run())
