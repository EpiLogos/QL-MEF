"""Real process failures exercise the research gate's cleanup and evidence."""
import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("native_research_gate", ROOT / "scripts/test-ql-agent-research.py")
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


class ActualResearchProcessChecks(unittest.TestCase):
    def test_actual_timeout_allows_owner_to_reap_separately_grouped_child_and_retains_attempt(self):
        with tempfile.TemporaryDirectory() as name:
            root = Path(name)
            pid = root / "child.pid"
            source = root / "native-process-owner.py"
            source.write_text("import os,signal,subprocess,sys,time\n"
                "p=subprocess.Popen([sys.executable,'-c','import time;time.sleep(30)'],start_new_session=True)\n"
                "def stop(sig,frame):\n"
                " os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=1)\n"
                " print('owned descendant reaped',flush=True);raise SystemExit(143)\n"
                "signal.signal(signal.SIGTERM,stop)\n"
                "open(sys.argv[1],'w').write(str(p.pid))\n"
                "print('actual owner started',flush=True)\n"
                "time.sleep(30)\n")
            receipt = {"commands": []}
            try:
                with self.assertRaises(subprocess.TimeoutExpired):
                    gate.run_program("timeout", [sys.executable, str(source), str(pid)],
                                     ROOT, os.environ.copy(), root, receipt, .5)
                child = int(pid.read_text())
                with self.assertRaises(ProcessLookupError):
                    os.kill(child, 0)
                self.assertIn(b"owned descendant reaped", (root / "timeout.stdout").read_bytes())
                self.assertEqual(receipt["commands"][0]["outcome"], "timed-out")
                self.assertEqual(receipt["commands"][0]["exit_code"], 143)
                self.assertEqual(receipt["commands"][0]["stdout_sha256"], gate.digest(root / "timeout.stdout"))
            finally:
                if pid.exists():
                    try:
                        os.killpg(int(pid.read_text()), 15)
                    except ProcessLookupError:
                        pass

    def test_actual_launch_failure_retains_metadata_and_both_output_hashes(self):
        with tempfile.TemporaryDirectory() as name:
            root = Path(name)
            receipt = {"commands": []}
            with self.assertRaises(FileNotFoundError):
                gate.run_program("absent", [str(root / "absent")], ROOT,
                                 os.environ.copy(), root, receipt, 1)
            attempt = receipt["commands"][0]
            self.assertIsNone(attempt["exit_code"])
            self.assertEqual(attempt["outcome"], "failed")
            self.assertEqual(attempt["stderr_sha256"], gate.digest(root / "absent.stderr"))


if __name__ == "__main__":
    unittest.main()
