"""Native replay and adverse copies of real completed Prime/Pi traces."""
import copy
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
import ql_agent_body_evidence as b


class FreshBodyEvidenceChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        directory = os.environ.get("QL_AGENT_EVIDENCE_ROOT")
        if not directory:
            raise unittest.SkipTest("actual fresh trained body receipts must be supplied explicitly")
        cls.evidence = Path(directory)
        cls.plan = b.read(cls.evidence / "g5-fresh-body-learned-plan-v2.json")

    def qualified(self, body, trace=None):
        return b.qualify(trace or self.evidence / (body + "-trained-g5-source-uptake-v2.json"),
            self.evidence / ("g5-fresh-" + body + "-v2/execution.json"), body, self.plan, "ql-agent")

    def changed_trace(self, mutate):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        trace = b.read(self.evidence / "pi-trained-g5-source-uptake-v2.json")
        mutate(trace)
        path = Path(temporary.name) / "actual-trace-adverse-copy.json"
        path.write_text(json.dumps(trace))
        return path

    def test_real_bodies_replay_identical_native_values_with_distinct_invocations(self):
        result = b.compare(self.qualified("prime"), self.qualified("pi"))
        self.assertTrue(result["parity"])
        self.assertNotEqual(result["bodies"][0]["provider_invocation"], result["bodies"][1]["provider_invocation"])
        self.assertTrue(all(body["native_calls"] == 2 and body["provider_attempts"] == 1 for body in result["bodies"]))

    def test_json_transport_numbers_do_not_collapse_booleans_or_round_large_integers(self):
        self.assertTrue(b.equal({"value": [1.0]}, {"value": [1]}))
        self.assertFalse(b.equal({"value": [True]}, {"value": [1]}))
        self.assertFalse(b.equal(9007199254740993, float(9007199254740993)))

    def test_a_duplicate_execution_result_is_not_one_native_call(self):
        path = self.changed_trace(lambda d: d["trace"].append(copy.deepcopy(next(
            row for row in d["trace"] if row["type"] == "tool_execution_end"))))
        with self.assertRaisesRegex(c.ContractError, "exactly two native calls"):
            self.qualified("pi", path)

    def test_tampered_actual_source_result_is_refused_by_native_replay(self):
        def change(document):
            row = next(row for row in document["trace"] if row["type"] == "tool_execution_end" and row["toolName"] == "ql_invoke")
            row["result"]["details"]["native_receipt"]["result"]["source"]["revision"] = "foreign-revision"
        with self.assertRaisesRegex(c.ContractError, "actual native owner"):
            self.qualified("pi", self.changed_trace(change))

    def test_ordinary_reply_cannot_be_empty(self):
        def change(document):
            end = [row for row in document["trace"] if row["type"] == "agent_end"][-1]
            for message in end["messages"]:
                if message["role"] == "assistant":
                    message["content"] = []
        with self.assertRaisesRegex(c.ContractError, "mode-off continuation missing"):
            self.qualified("pi", self.changed_trace(change))

    def test_a_failed_actor_after_a_successful_tool_is_not_successful_uptake(self):
        def change(document):
            end = next(row for row in document["trace"] if row["type"] == "agent_end")
            next(message for message in end["messages"] if message["role"] == "assistant")["stopReason"] = "error"
        with self.assertRaisesRegex(c.ContractError, "actor turn failed"):
            self.qualified("pi", self.changed_trace(change))

    def test_real_provider_snapshot_cannot_change_address_under_an_old_digest(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        directory = Path(temporary.name)
        source_config = self.evidence / "g5-fresh-pi-v2/execution.json"
        configuration = b.read(source_config)
        copied = directory / "inference"
        shutil.copytree(configuration["receipt_dir"], copied)
        snapshot = next(copied.glob("*.provider.json"))
        provider = b.read(snapshot)
        provider["address"] = "127.0.0.1:1"
        snapshot.write_text(json.dumps(provider))
        configuration["receipt_dir"] = str(copied)
        config_path = directory / "execution.json"
        config_path.write_text(json.dumps(configuration))
        with self.assertRaisesRegex(c.ContractError, "provider configuration digest mismatch"):
            b.qualify(self.evidence / "pi-trained-g5-source-uptake-v2.json", config_path,
                      "pi", self.plan, "ql-agent")


if __name__ == "__main__":
    unittest.main()
