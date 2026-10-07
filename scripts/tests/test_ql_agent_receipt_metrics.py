"""Supplemental metrics over completed, real native inference receipts.

QL_AGENT_EVIDENCE_ROOT selects retained research evidence. No model or service
is replaced. Rejection tests alter copies of those actual receipts.
"""
import json
import os
from pathlib import Path
import shutil
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import ql_agent_contracts as c
import ql_agent_receipt_metrics as metrics


class RetainedInferenceChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        evidence = os.environ.get("QL_AGENT_EVIDENCE_ROOT")
        if not evidence:
            raise unittest.SkipTest("completed native research receipts must be explicitly supplied")
        cls.evidence = Path(evidence)
        cls.evaluation = cls.evidence / "G2-criteria-packed-reviewed/metrics.json"
        cls.receipts = cls.evidence / "G2-criteria-packed-provider-receipts"
        cls.card = cls.evidence / "G2-criteria-threads2-model-card.json"
        cls.world = cls.evidence / "G2-criteria-packed-world.json"

    def copied(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        destination = Path(temporary.name) / "receipts"
        shutil.copytree(self.receipts, destination)
        return destination

    def test_real_ranked_evidence_and_native_token_usage(self):
        result = metrics.collect(self.evaluation, self.receipts, self.card, self.world)
        self.assertEqual(result["answered_cases"], 9)
        self.assertEqual(result["ranked_heads"], 10)
        self.assertEqual(result["top_k_candidate_set_coverage"], {"1": .3, "2": .4, "3": .6})
        self.assertEqual(result["input_tokens"], 12739)
        self.assertEqual(result["output_tokens"], 0)
        self.assertEqual(result["token_usage_cases"], 9)
        self.assertGreater(result["reported_inference_p95_seconds"], 100)
        self.assertGreater(result["cold_model_load_seconds"], 0)
        self.assertFalse(result["operative_mutation"])

    def test_missing_model_card_remains_unmeasured(self):
        self.assertIsNone(metrics.collect(self.evaluation, self.receipts)["cold_model_load_seconds"])

    def test_real_timeout_does_not_impersonate_a_returned_model_revision(self):
        result = metrics.collect(self.evidence / "G6-packed-reviewed/metrics.json",
            self.evidence / "G6-packed-inference-receipts",
            self.evidence / "G6-packed-model-card.json", self.evidence / "G6-packed-world.json")
        self.assertEqual(result["answered_cases"], 8)
        self.assertEqual(result["unanswered_cases"], 1)
        self.assertEqual(result["model_revisions"],
            ["gliner2.5-decide-ql-hard-negative-lora@88ab99909a75ed5f6aa5ddd8ac43e2462fde1582d6276cc6db211fbb74e7087d"])
        self.assertGreater(result["cold_model_load_seconds"], 0)

    def test_actual_kev_noul_bindings_survive_json_tuple_to_array_encoding(self):
        cases = [json.loads(line) for line in (self.evidence / "K0-reviewed-semantic/cases.jsonl").read_text().splitlines()]
        directory = self.evidence / "K0-reviewed-semantic-aikit"
        path = next(directory.glob("*.receipt.json"))
        receipt = json.loads(path.read_text())
        case = next(row for row in cases if row["projection"]["frame"]["event_basis_digest"] == receipt["event_basis_digest"])
        request = json.loads(path.with_name(path.name.replace(".receipt.json", ".request.json")).read_text())
        provider = json.loads(path.with_name(path.name.replace(".receipt.json", ".provider.json")).read_text())
        self.assertTrue(all(isinstance(binding, list) for binding in receipt["bindings"].values()))
        self.assertEqual(metrics.qualify_request(case["projection"], receipt, request, provider), request["model"])

    def test_changed_actual_request_snapshot_is_refused(self):
        directory = self.copied()
        path = next(directory.glob("*.request.json"))
        document = json.loads(path.read_text())
        document["state"]["material"] += " altered source"
        path.write_text(json.dumps(document))
        with self.assertRaisesRegex(c.ContractError, "request snapshot digest"):
            metrics.collect(self.evaluation, directory)

    def test_receipt_cannot_join_a_different_event_basis(self):
        directory = self.copied()
        path = next(directory.glob("*.receipt.json"))
        document = json.loads(path.read_text())
        document["event_basis_digest"] = "sha256:" + "0" * 64
        path.write_text(json.dumps(document))
        with self.assertRaisesRegex(c.ContractError, "exact event/frame"):
            metrics.collect(self.evaluation, directory)

    def test_rehashed_foreign_request_still_cannot_join_native_source(self):
        directory = self.copied()
        path = next(directory.glob("*.receipt.json"))
        document = json.loads(path.read_text())
        request_path = path.with_name(path.name.replace(".receipt.json", ".request.json"))
        request = json.loads(request_path.read_text())
        request["state"]["material"] = "Unrelated source material"
        document["request_digest"] = c.digest(request)
        request_path.write_text(json.dumps(request))
        path.write_text(json.dumps(document))
        with self.assertRaisesRegex(c.ContractError, "joined native semantic basis"):
            metrics.collect(self.evaluation, directory)

    def test_altered_native_probabilities_cannot_reproduce_accepted_response(self):
        directory = self.copied()
        path = next(directory.glob("*.receipt.json"))
        document = json.loads(path.read_text())
        answer = next(iter(document["native_envelope"]["data"]["answer"]["answers"].values()))
        answer["probabilities"][answer["choice"]] = 0
        path.write_text(json.dumps(document))
        with self.assertRaisesRegex(c.ContractError, "probabilities|selection"):
            metrics.collect(self.evaluation, directory)

    def test_foreign_model_card_is_refused(self):
        directory = self.copied()
        path = directory / "foreign-card.json"
        document = json.loads(self.card.read_text())
        document["models"][0]["id"] = "foreign-model"
        path.write_text(json.dumps(document))
        with self.assertRaisesRegex(c.ContractError, "model card identity"):
            metrics.collect(self.evaluation, directory, path, self.world)

    def test_same_id_foreign_source_card_is_refused(self):
        directory = self.copied()
        path = directory / "foreign-source-card.json"
        document = json.loads(self.card.read_text())
        document["models"][0].update(artifact_revision="0" * 40, sdk_revision="0" * 40,
                                    adapter_sha256="0" * 64, load_seconds=9999)
        path.write_text(json.dumps(document))
        with self.assertRaisesRegex(c.ContractError, "source metadata"):
            metrics.collect(self.evaluation, directory, path, self.world)


if __name__ == "__main__":
    unittest.main()
