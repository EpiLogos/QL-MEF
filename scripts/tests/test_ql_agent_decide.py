"""Shared carrier bounds and pre-execution admission; no model is simulated."""
import subprocess
import sys
import unittest
from argparse import Namespace
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import ql_agent_contracts as c
import ql_agent_decide as d


class DecisionCarrierChecks(unittest.TestCase):
    def test_invalid_calibration_or_revision_is_refused_before_execution(self):
        for threshold in (0, -1, 1.01, float("nan"), float("inf"), True, "0.5"):
            with self.assertRaises(c.ContractError):
                d.validate_execution(Namespace(threshold=threshold, model_revision="exact:revision"))
        for revision in (None, "", " ", 3):
            with self.assertRaises(c.ContractError):
                d.validate_execution(Namespace(threshold=.5, model_revision=revision))
        d.validate_execution(Namespace(threshold=.5, model_revision="exact:revision"))

    def test_raw_oversize_whitespace_is_not_silently_truncated(self):
        result = subprocess.run([sys.executable, str(ROOT / "scripts/ql_agent_decide.py")],
                                input=b"{}" + b" " * (1024 * 1024), capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 2)
        self.assertIn(b"input exceeds native byte limit", result.stderr)


if __name__ == "__main__":
    unittest.main()
