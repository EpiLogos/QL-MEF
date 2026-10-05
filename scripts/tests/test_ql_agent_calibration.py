"""Calibration over real retained validation inference and native QL/M5.

The machine evidence directory is supplied explicitly. No inference service or
native owner is mocked. Adverse checks mutate copies of that actual evidence.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import ql_agent_calibration as calibration
import ql_agent_contracts as c
import ql_agent_receipt_metrics as evidence


class NativeCalibrationChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        root = os.environ.get("QL_AGENT_EVIDENCE_ROOT")
        if not root:
            raise unittest.SkipTest("explicit actual validation inference evidence required")
        cls.evidence = Path(root)
        if not (cls.evidence / "G5-validation-reviewed-v1/metrics.json").exists():
            raise unittest.SkipTest("actual completed validation inference unavailable")
        if shutil.which("ql-agent") is None:
            raise unittest.SkipTest("actual evaluated native QL executable required")

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.args = argparse.Namespace(metrics=self.evidence / "G5-validation-reviewed-v1/metrics.json",
            receipts=self.evidence / "G5-validation-inference-v1",
            suite=self.evidence / "reviewed-faculty-training-v4/native/suite.jsonl",
            suite_digest="sha256:ed71162682f1a054809dd6917b69f5898b917c2a98258e8d51a75132f2775579",
            regression=self.evidence / "G4-deterministic-regression-v2/metrics.json",
            output=Path(self.temp.name) / "calibration", thresholds=[.4, .5, .6],
            primary="exact_determination_accuracy", ql="ql-agent")

    def test_actual_validation_replays_owner_admission_and_native_m5(self):
        result = calibration.run(self.args)
        self.assertEqual(result["classifier_calls"], 0)
        self.assertFalse(result["held_out_semantic_test_used"])
        self.assertEqual(result["deterministic_regression_split"], "test")
        self.assertEqual(result["default_election"], "not attempted")
        baseline = evidence.read(self.args.metrics)
        for point in result["points"]:
            compared = evidence.read(Path(point["native_m5_return"]))
            self.assertEqual(compared["native_return"]["operation"], "logos.return")
            self.assertEqual(compared["kernel_basis"], result["kernel_basis"])
            self.assertEqual(compared["default_election"], "not attempted")
        middle = evidence.read(Path(result["points"][1]["metrics"]))
        self.assertEqual(middle["metrics"], baseline["metrics"])
        if any(value is None for point in result["points"] for value in point["gates"].values()):
            self.assertIsNone(result["recommended_validation_threshold"])

    def test_held_out_actual_test_inference_cannot_be_calibration_material(self):
        self.args.metrics = self.evidence / "G5-packed-reviewed/metrics.json"
        with self.assertRaisesRegex(c.ContractError, "never held-out"):
            calibration.run(self.args)
        self.assertFalse(self.args.output.exists())

    def test_changed_frozen_gold_is_refused_before_native_replay(self):
        path = Path(self.temp.name) / "suite.jsonl"
        rows = self.args.suite.read_text().splitlines()
        selected = next(i for i, line in enumerate(rows) if json.loads(line)["split"] == "validation")
        value = json.loads(rows[selected])
        value["expected"]["status"] = "refused"
        rows[selected] = c.canonical(value).decode()
        path.write_text("\n".join(rows) + "\n")
        self.args.suite = path
        self.args.suite_digest = evidence.file_digest(path)
        with self.assertRaisesRegex(c.ContractError, "differs from evaluated suite"):
            calibration.run(self.args)
        self.assertFalse(self.args.output.exists())

    def test_actual_semantic_test_cannot_enter_through_regression(self):
        self.args.regression = self.evidence / "G5-packed-reviewed/metrics.json"
        with self.assertRaisesRegex(c.ContractError, "purely deterministic"):
            calibration.run(self.args)
        self.assertFalse(self.args.output.exists())

    def test_changed_actual_provider_snapshot_is_refused(self):
        directory = Path(self.temp.name) / "inference"
        shutil.copytree(self.args.receipts, directory)
        path = next(directory.glob("*.provider.json"))
        provider = evidence.read(path)
        provider["label"] = "changed actual execution snapshot"
        path.write_bytes(c.canonical(provider))
        self.args.receipts = directory
        with self.assertRaisesRegex(c.ContractError, "configuration digest mismatch"):
            calibration.run(self.args)
        self.assertFalse(self.args.output.exists())

    def test_thresholds_are_bounded_distinct_and_not_booleans(self):
        for thresholds in ([.5], [.5, .5], [False, .5], [.5, float("nan")], [.5, 1.01]):
            with self.subTest(thresholds=thresholds):
                self.args.thresholds = thresholds
                with self.assertRaisesRegex(c.ContractError, "distinct finite thresholds"):
                    calibration.run(self.args)
                self.assertFalse(self.args.output.exists())

    def test_different_real_executable_cannot_replace_evaluated_owner(self):
        self.args.ql = sys.executable
        with self.assertRaisesRegex(c.ContractError, "evaluated native QL executable"):
            calibration.run(self.args)
        self.assertFalse(self.args.output.exists())

    def test_existing_output_is_preserved(self):
        self.args.output.mkdir()
        value = self.args.output / "keep.txt"
        value.write_text("unique existing evidence")
        with self.assertRaises(FileExistsError):
            calibration.run(self.args)
        self.assertEqual(value.read_text(), "unique existing evidence")


if __name__ == "__main__":
    unittest.main()
