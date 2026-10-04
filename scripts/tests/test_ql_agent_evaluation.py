"""Metric/freeze/leakage laws over real retained contract material.

Controlled proposals exercise arithmetic and disposition handling. They are
not inference or end-to-end model benchmark receipts. No service is mocked.
"""
import copy
import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import ql_agent_contracts as c
import ql_agent_evaluation as e

BASE = ROOT / "fixtures/agent-decision/v1"


class EvaluationChecks(unittest.TestCase):
    def setUp(self):
        self.event = json.loads((BASE / "event-v1.json").read_text())
        self.frame = json.loads((BASE / "decision-frame-v1.json").read_text())
        self.determination = json.loads((BASE / "unavailable-determination-v1.json").read_text())
        c.validate_determination(self.event, self.frame, self.determination)
        self.case = {"schema": "ql.agent-evaluation-case/v1", "id": "controlled:missing-evidence",
                     "family": "controlled:abstention", "split": "test",
                     "projection": {"event": self.event, "requested_heads": ["lens"]},
                     "expected": {"fields": {}, "heads": {"semantic-lens": []},
                                  "status": "unavailable", "basis_refs": ["input:controlled-evaluation-law"]}}

    def record(self, gold=(), predicted=None):
        result = copy.deepcopy(self.determination)
        if predicted is not None:
            proposal = {"head_id": "semantic-lens", "label_ids": list(predicted), "spans": []}
            result["provider"] = {"provider_ref": "input:metric-arithmetic", "model_ref": "input:controlled",
                                  "model_revision": "1", "runtime_revision": "1"}
            result["learned"] = [proposal]
            if predicted:
                result["validated"] = [{"field": "lens", "value": list(predicted), "origin": "learned",
                                        "proposal_head": "semantic-lens", "basis_digest": c.digest(self.frame),
                                        "kernel_rule_refs": ["input:controlled-disposition"]}]
                result["unresolved"] = []
                result["status"] = "determined"
            else:
                result["status"] = "unresolved"
            c.validate_determination(self.event, self.frame, result)
        projection = {"schema": "ql.agent-projection/v1", "event": self.event, "frame": self.frame,
                      "determination": result, "decision_head_ids": ["semantic-lens"]}
        return {"expected": {"fields": {}, "heads": {"semantic-lens": list(gold)},
                             "status": result["status"], "basis_refs": ["input:metric-law"]},
                "projection": projection, "provider_calls": int(predicted is not None),
                "provider_elapsed_seconds": .25 if predicted is not None else None, "input_bytes": 100}

    def frozen(self, cases):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        path = Path(directory.name) / "suite.jsonl"
        path.write_bytes(b"".join(c.canonical(case) + b"\n" for case in cases))
        return path, "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

    def test_frozen_suite_verifies_exact_bytes_and_source_basis(self):
        path, digest = self.frozen([self.case])
        cases, observed = e.load_suite(path, digest)
        self.assertEqual(cases, [self.case])
        self.assertEqual(observed, digest)
        with self.assertRaisesRegex(c.ContractError, "digest mismatch"):
            e.load_suite(path, "sha256:" + "0" * 64)
        bad = copy.deepcopy(self.case)
        bad["expected"]["basis_refs"] = []
        with self.assertRaisesRegex(c.ContractError, "independent"):
            e.load_suite(*self.frozen([bad]))

    def test_random_rows_cannot_leak_a_template_family_between_splits(self):
        other = copy.deepcopy(self.case)
        other.update(id="controlled:second", split="train")
        with self.assertRaisesRegex(c.ContractError, "leaks across splits"):
            e.load_suite(*self.frozen([self.case, other]))

    def test_duplicate_ids_and_json_keys_are_refused(self):
        with self.assertRaisesRegex(c.ContractError, "duplicate case"):
            e.load_suite(*self.frozen([self.case, self.case]))
        with self.assertRaisesRegex(c.ContractError, "duplicate JSON"):
            e.decode('{"x":1,"x":2}')
        with self.assertRaisesRegex(c.ContractError, "non-finite"):
            e.decode('{"x":NaN}')

    def test_provider_state_never_contains_verifier_answers(self):
        record = self.record(["L0"])
        poisoned = dict(record["projection"], expected=self.case["expected"], verifier_secret="answer")
        observed = e.provider_input(poisoned)
        self.assertEqual(set(observed), {"schema", "event", "frame", "decision_head_ids"})
        self.assertNotIn("expected", observed)
        self.assertNotIn("verifier_secret", observed)

    def test_unavailable_probability_and_latency_are_absence_not_zero(self):
        observed = e.metrics([self.record()])
        self.assertEqual(observed["exact_candidate_set_accuracy"], 1)
        self.assertEqual(observed["missing_evidence_abstention_rate"], 1)
        self.assertEqual(observed["provider_calls"], 0)
        self.assertIsNone(observed["micro_f1"])
        self.assertIsNone(observed["provider_elapsed_p50_seconds"])
        self.assertIn("top_k_coverage", observed["not_measured"])

    def test_exact_sets_and_f1_preserve_all_ambiguity(self):
        observed = e.metrics([self.record(["L0", "L1"], ["L0"])])
        self.assertEqual(observed["exact_determination_accuracy"], 0)
        self.assertEqual(observed["exact_candidate_set_accuracy"], 0)
        self.assertEqual(observed["ambiguity_preservation"], 0)
        self.assertAlmostEqual(observed["micro_f1"], 2 / 3)
        self.assertEqual(observed["macro_f1"], .5)
        self.assertEqual(e.metrics([self.record(["L0", "L1"], ["L1", "L0"])])["exact_determination_accuracy"], 1)

    def test_missing_evidence_certainty_is_counted(self):
        observed = e.metrics([self.record([], ["L0"]), self.record([], [])])
        self.assertEqual(observed["unsupported_certainty_rate"], .5)
        self.assertEqual(observed["missing_evidence_abstention_rate"], .5)
        self.assertEqual(observed["provider_calls"], 2)

    def test_refused_proposal_does_not_count_as_operative_prediction(self):
        record = self.record(["L0"], ["L0"])
        result = record["projection"]["determination"]
        result["validated"] = []
        result["status"] = "refused"
        result["refused_candidates"] = [{"head_id": "semantic-lens", "label_ids": ["L0"],
                                         "origin": "learned", "reason": "controlled refusal",
                                         "rule_refs": ["input:controlled-refusal"]}]
        c.validate_determination(self.event, self.frame, result)
        observed = e.metrics([record])
        self.assertEqual(observed["exact_candidate_set_accuracy"], 0)
        self.assertEqual(observed["kernel_refusal_rate"], 1)
        self.assertEqual(observed["refused_result_operative_rate"], 0)

    def test_omitting_a_native_head_cannot_improve_accuracy(self):
        record = self.record(["L0"])
        record["expected"]["heads"] = {}
        with self.assertRaisesRegex(c.ContractError, "exact native eligible"):
            e.metrics([record])

    def test_early_admission_refusal_is_counted_and_cannot_be_scored_as_abstention(self):
        record = self.record()
        result = record["projection"]["determination"]
        result["status"] = "unresolved"
        record["expected"]["status"] = "unresolved"
        record["provider_calls"] = 1
        response = {"schema": "ql.agent-decision-response/v1", "outcome": "answered",
                    "proposals": [{"head_id": "semantic-lens", "label_ids": ["L0"],
                                   "spans": [{"text": "fabricated"}]}]}
        record["response"] = response
        record["admission"] = {"schema": "ql.agent-decision-admission/v1", "response": response,
                               "admission_status": "refused", "projection": record["projection"]}
        c.validate_determination(self.event, self.frame, result)
        observed = e.metrics([record])
        self.assertEqual(observed["admission_refusal_rate"], 1)
        self.assertIsNone(observed["kernel_refusal_rate"])
        self.assertEqual(observed["unsupported_certainty_rate"], 1)
        self.assertEqual(observed["operative_unsupported_certainty_rate"], 0)
        self.assertEqual(observed["missing_evidence_abstention_rate"], 0)
        self.assertEqual(observed["exact_determination_accuracy"], 0)

    def test_gold_cannot_introduce_a_shadow_label(self):
        with self.assertRaisesRegex(c.ContractError, "native legal field"):
            e.metrics([self.record(["L99"])])

    def test_explicitly_expected_kernel_refusal_can_be_exact(self):
        record = self.record([], ["L0"])
        result = record["projection"]["determination"]
        result["validated"] = []
        result["status"] = "refused"
        result["refused_candidates"] = [{"head_id": "semantic-lens", "label_ids": ["L0"],
                                         "origin": "learned", "reason": "controlled negative expectation",
                                         "rule_refs": ["input:controlled-refusal"]}]
        record["expected"]["status"] = "refused"
        record["admission"] = {"admission_status": "refused"}
        c.validate_determination(self.event, self.frame, result)
        self.assertEqual(e.metrics([record])["exact_determination_accuracy"], 1)

    def test_nearest_rank_quantiles_use_actual_samples(self):
        self.assertIsNone(e.quantile([], .95))
        self.assertEqual(e.quantile([4, 1, 3, 2], .5), 2)
        self.assertEqual(e.quantile([4, 1, 3, 2], .95), 4)


if __name__ == "__main__":
    unittest.main()
