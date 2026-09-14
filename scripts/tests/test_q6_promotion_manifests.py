import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class Q6PromotionManifestTests(unittest.TestCase):
    def test_current_q6_promotions_hold(self):
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "check-q6-promotion-manifests.py")],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("Q6 promotion manifests: PASS", result.stdout)


if __name__ == "__main__":
    unittest.main()
