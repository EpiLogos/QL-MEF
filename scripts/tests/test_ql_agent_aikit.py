"""Actual frame translation/receipt laws; no inference service is mocked.

Controlled Noul values test the public codec, not model accuracy. Zero-head and
disabled-provider cases execute the real adapter with no available executable.
"""
import copy
import json
import os
import subprocess
import sys
import tempfile
import time
import unittest
from argparse import Namespace
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import ql_agent_contracts as c
import ql_agent_aikit as a
import ql_agent_evaluation as e

BASE = ROOT / "fixtures/agent-decision/v1"


class AiKitTranslationChecks(unittest.TestCase):
    def setUp(self):
        self.projection = {"schema": "ql.agent-projection/v1",
                           "event": json.loads((BASE / "event-v1.json").read_text()),
                           "frame": json.loads((BASE / "decision-frame-v1.json").read_text()),
                           "decision_head_ids": ["semantic-lens"]}
        self.request, self.bindings = a.translate_request(self.projection, "input:controlled-codec")

    def receipt(self, scores):
        return {"outcome": "completed", "invocation_ref": "input:controlled-codec-receipt",
                "answer": {"model": "input:returned-model", "answers": {
                    question: {"type": "noul", "noul": scores.get(label, .1)}
                    for question, (_, label) in self.bindings.items()}}}

    def answer(self, receipt, threshold=.5):
        return a.translate_answer(self.projection, self.bindings, receipt, threshold,
                                  "input:controlled-material", "input:controlled-runtime")

    def test_exact_native_labels_descriptions_and_owner_refs_are_preserved(self):
        native = self.projection["frame"]["unresolved"][0]["labels"]
        translated = [question["criteria"]["true"]["candidate"]
                      for question in self.request["questions"].values()]
        self.assertEqual(translated, native)
        self.assertEqual(self.request["state"]["determined"], self.projection["frame"]["determined"])
        self.assertEqual(self.request["state"]["constraints"], self.projection["frame"]["constraints"])

    def test_gold_or_extra_fields_cannot_enter_provider_input(self):
        self.projection["expected"] = {"lens": "L2"}
        with self.assertRaisesRegex(c.ContractError, "allow-list"):
            a.translate_request(self.projection, "input:model")

    def test_explicit_basis_is_retained_and_not_relabelled(self):
        answer = self.answer(self.receipt({"L2": .8}))
        self.assertEqual(answer["event_basis_digest"], self.projection["frame"]["event_basis_digest"])
        self.assertEqual(answer["frame_digest"], c.digest(self.projection["frame"]))
        self.assertEqual(answer["kernel_basis"], self.projection["frame"]["kernel_basis"])
        self.assertEqual(answer["provider"]["model_ref"], "input:returned-model")
        self.assertEqual(answer["proposals"], [{"head_id": "semantic-lens", "label_ids": ["L2"],
                                               "confidence": .8, "spans": []}])

    def test_ambiguity_retains_all_threshold_candidates_without_joint_probability(self):
        answer = self.answer(self.receipt({"L2": .8, "L2'": .7}))
        self.assertEqual(answer["proposals"][0]["label_ids"], ["L2", "L2'"])
        self.assertNotIn("confidence", answer["proposals"][0])

    def test_insufficient_scores_abstain_instead_of_selecting_largest(self):
        self.assertEqual(self.answer(self.receipt({}))['proposals'][0]['label_ids'], [])

    def test_threshold_is_explicit_finite_and_not_an_implicit_default(self):
        for threshold in (0, -1, 1.1, float("nan"), True):
            with self.assertRaises(c.ContractError):
                self.answer(self.receipt({}), threshold)

    def test_wrong_or_missing_actual_answer_is_refused_without_probability_repair(self):
        for score in (-.1, 1.1, float("inf"), True, "0.8"):
            with self.assertRaises(c.ContractError):
                self.answer(self.receipt({"L2": score}))
        receipt = self.receipt({})
        receipt["answer"]["answers"].pop(next(iter(self.bindings)))
        with self.assertRaisesRegex(c.ContractError, "exactly"):
            self.answer(receipt)

    def test_failed_native_execution_remains_unavailable(self):
        answer = self.answer({"outcome": "failed", "failure": {"message": "model unavailable"}})
        self.assertEqual(answer["outcome"], "unavailable")
        self.assertEqual(answer["proposals"], [])
        self.assertEqual(answer["reason"], "model unavailable")

    def test_zero_heads_bypass_real_execution_and_even_config_access(self):
        self.projection["decision_head_ids"] = []
        with tempfile.TemporaryDirectory() as directory:
            args = Namespace(provider_file=Path(directory) / "absent.json",
                             receipt_dir=Path(directory) / "must-not-exist",
                             aikit="/unavailable/aikit")
            answer = a.execute(args, self.projection)
            self.assertEqual(answer["outcome"], "unavailable")
            self.assertEqual(answer["proposals"], [])
            self.assertFalse(args.receipt_dir.exists())

    def test_disabled_provider_never_resolves_or_launches_executable(self):
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / "none.json"
            config.write_text(json.dumps({"schema": "aikit.decision-provider/v1", "mode": "none"}))
            args = Namespace(provider_file=config, receipt_dir=Path(directory) / "must-not-exist",
                             aikit="/unavailable/aikit")
            answer = a.execute(args, self.projection)
            self.assertEqual(answer["outcome"], "unavailable")
            self.assertEqual(answer["reason"], "AIKit decision provider disabled")
            self.assertFalse(args.receipt_dir.exists())

    def test_invalid_or_duplicate_head_never_enters_packed_questions(self):
        for ids in (["absent-head"], ["semantic-lens", "semantic-lens"]):
            projection = copy.deepcopy(self.projection)
            projection["decision_head_ids"] = ids
            with self.assertRaises(c.ContractError):
                a.translate_request(projection, "input:model")

    def test_input_required_disposition_cannot_be_promoted_to_semantic_head(self):
        self.projection["frame"]["unresolved"][0]["id"] = "input-lens"
        self.projection["decision_head_ids"] = ["input-lens"]
        with self.assertRaisesRegex(c.ContractError, "input-required"):
            a.translate_request(self.projection, "input:model")

    def test_owned_native_process_timeout_reaps_its_actual_child(self):
        with tempfile.TemporaryDirectory() as directory:
            pid_path = Path(directory) / "child.pid"
            program = ("import subprocess,time,pathlib; "
                       "p=subprocess.Popen(['" + sys.executable + "','-c','import time; time.sleep(30)']); "
                       "pathlib.Path(" + repr(str(pid_path)) + ").write_text(str(p.pid)); time.sleep(30)")
            with self.assertRaises(subprocess.TimeoutExpired):
                a.run_owned([sys.executable, "-c", program], 1)
            pid = int(pid_path.read_text())
            for _ in range(20):
                try:
                    os.kill(pid, 0)
                except ProcessLookupError:
                    break
                time.sleep(.05)
            else:
                self.fail("owned child survived adapter timeout")

    def test_actual_outer_evaluator_timeout_reaps_adapter_and_nested_group(self):
        with tempfile.TemporaryDirectory() as directory:
            adapter_pid = Path(directory) / "adapter.pid"
            child_pid = Path(directory) / "child.pid"
            child_program = ("import os,time,pathlib; pathlib.Path(" + repr(str(child_pid)) +
                             ").write_text(str(os.getpid())); time.sleep(30)")
            adapter_program = ("import os,sys,pathlib; sys.path.insert(0," +
                               repr(str(ROOT / 'scripts')) + "); import ql_agent_aikit as a; " +
                               "pathlib.Path(" + repr(str(adapter_pid)) + ").write_text(str(os.getpid())); " +
                               "a.run_owned([sys.executable,'-c'," + repr(child_program) + "],30)")
            with self.assertRaises(subprocess.TimeoutExpired):
                e.invoke([sys.executable, "-c", adapter_program], {}, 1)
            for path in (adapter_pid, child_pid):
                pid = int(path.read_text())
                for _ in range(20):
                    try:
                        os.kill(pid, 0)
                    except ProcessLookupError:
                        break
                    time.sleep(.05)
                else:
                    self.fail("owned process survived outer evaluation timeout: " + str(path))

    def test_actual_evaluator_termination_reaps_all_owned_nested_groups(self):
        with tempfile.TemporaryDirectory() as directory:
            adapter_pid = Path(directory) / "adapter.pid"
            child_pid = Path(directory) / "child.pid"
            child_program = ("import os,time,pathlib; pathlib.Path(" + repr(str(child_pid)) +
                             ").write_text(str(os.getpid())); time.sleep(30)")
            adapter_program = ("import os,sys,pathlib; sys.path.insert(0," +
                               repr(str(ROOT / 'scripts')) + "); import ql_agent_aikit as a; " +
                               "pathlib.Path(" + repr(str(adapter_pid)) + ").write_text(str(os.getpid())); " +
                               "a.run_owned([sys.executable,'-c'," + repr(child_program) + "],30)")
            evaluator_program = ("import sys; sys.path.insert(0," + repr(str(ROOT / 'scripts')) +
                                 "); import ql_agent_evaluation as e; " +
                                 "e.invoke([sys.executable,'-c'," + repr(adapter_program) + "],{},30)")
            process = subprocess.Popen([sys.executable, "-c", evaluator_program],
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                       start_new_session=True)
            try:
                for _ in range(200):
                    if child_pid.exists():
                        break
                    time.sleep(.05)
                else:
                    self.fail("real descendant did not enter the invocation")
                process.terminate()
                process.communicate(timeout=5)
                self.assertEqual(process.returncode, 143)
                for path in (adapter_pid, child_pid):
                    pid = int(path.read_text())
                    with self.assertRaises(ProcessLookupError):
                        os.kill(pid, 0)
            finally:
                if process.poll() is None:
                    process.terminate()
                    process.communicate(timeout=5)

    def test_codec_rejects_duplicate_json_keys_and_nonfinite_numbers(self):
        for data in (b'{"a":1,"a":2}', b'{"a":NaN}'):
            with self.assertRaises(c.ContractError):
                a.decode(data)

    def test_malformed_receipt_containers_are_explicit_contract_refusals(self):
        for receipt in ([], None, 3, "receipt", {"outcome": "unrecognised"},
                        {"outcome": "failed", "failure": []},
                        {"outcome": "failed", "failure": {"message": ["error"]}},
                        {"outcome": "completed", "answer": []},
                        {"outcome": "completed", "answer": {"answers": []}}):
            with self.assertRaises(c.ContractError):
                self.answer(receipt)

    def test_malformed_configurations_remain_unavailable_without_execution(self):
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / "invalid.json"
            args = Namespace(provider_file=config, receipt_dir=Path(directory) / "must-not-exist",
                             aikit="/unavailable/aikit")
            for document in ([], None, "config", {"schema": "aikit.decision-provider/v1", "mode": "endpoint",
                                                  "limits": []}):
                config.write_text(json.dumps(document))
                answer = a.execute(args, self.projection)
                self.assertEqual(answer["outcome"], "unavailable")
                self.assertEqual(answer["proposals"], [])
                self.assertFalse(args.receipt_dir.exists())


if __name__ == "__main__":
    unittest.main()
