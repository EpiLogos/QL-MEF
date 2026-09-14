import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class CiTopologyTests(unittest.TestCase):
    def test_current_repository_topology_is_valid(self):
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "check-ci-topology.py")],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("CI topology: PASS", result.stdout)


if __name__ == "__main__":
    unittest.main()
