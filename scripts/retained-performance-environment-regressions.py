#!/usr/bin/env python3
"""Counterprove publication with the actual ready native producer corpus.

Requires real preparer output: never synthesises a native fixture or reruns a
producer. Every case privately relocates path metadata and hardlinks actual
bytes; byte corruption replaces only its private link. Original receipts and
native files are left unchanged. The same public publisher CLI is exercised.
"""
from __future__ import annotations
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile
import unittest
import uuid

sys.dont_write_bytecode = True
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--result", required=True, type=Path)
parser.add_argument("--expected-ql-head", required=True)
parser.add_argument("--output-root", required=True, type=Path)
args = parser.parse_args()
pointer = json.loads(args.result.read_bytes())
original = json.loads(Path(pointer["receipt"]).read_bytes())
original_run = Path(original["run"]).resolve(strict=True)
assert original["status"] == "ready", "Counterproofs need actual successful producer output"
assert original["actual_ql_head"] == original["expected_ql_head"] == args.expected_ql_head
publisher = Path(__file__).with_name("retained-performance-ci-environment.py")
sentinel = b"EXISTING_RECEIVER=preserved\n"


class ActualPublication(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="publisher-counterproof-", dir=args.output_root)
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.run = self.root / ("native-performance-" + uuid.uuid4().hex)
        self.run.mkdir()
        self.receipt = copy.deepcopy(original)

        def relocate(raw: str) -> Path:
            return self.run / Path(raw).relative_to(original_run)

        for row in self.receipt["fixtures"]:
            source = Path(row["path"])
            target = relocate(row["path"])
            target.parent.mkdir(parents=True, exist_ok=True)
            os.link(source, target)
            row["path"] = str(target)
        for command in self.receipt["commands"]:
            for key in ("log","stdout"):
                source = Path(command[key])
                target = relocate(command[key])
                if not target.exists():
                    target.parent.mkdir(parents=True, exist_ok=True)
                    os.link(source, target)
                command[key] = str(target)
            for row in command["original_output"]:
                row["path"] = str(relocate(row["path"]))
            # Only path metadata is relocated. All original native output
            # remains byte-identical; no successful native receipt is invented.
            preexecution = json.loads(Path(command["preexecution_receipt"]).read_bytes())
            command["preexecution_receipt"] = str(relocate(command["preexecution_receipt"]))
            for key in ("log","stdout","preexecution_receipt"):
                preexecution[key] = command[key]
            Path(command["preexecution_receipt"]).write_text(json.dumps(preexecution))
        environment = {}
        for line in Path(original["environment_file"]).read_text().splitlines():
            words = shlex.split(line, comments=False)
            assert len(words) == 2 and words[0] == "export"
            key, raw = words[1].split("=", 1)
            target = relocate(raw)
            if Path(raw).is_dir() or key.endswith("DIRECTORY") or key.endswith("HOME"):
                target.mkdir(parents=True, exist_ok=True)
            environment[key] = str(target)
        self.environment = environment
        self.environment_file = self.run / "environment.sh"
        self.environment_file.write_text("".join("export " + key + "=" + shlex.quote(value) + "\n" for key, value in environment.items()))
        self.receipt.update(run=str(self.run), environment_file=str(self.environment_file))
        self.receipt_file = self.run / "receipt.json"
        self.result = self.root / "result.json"
        self.result.write_text(json.dumps({"receipt": str(self.receipt_file), "environment_file": str(self.environment_file)}))
        self.github_env = self.root / "github-env"
        self.github_env.write_bytes(sentinel)

    def invoke(self, expected_head: str | None = None) -> subprocess.CompletedProcess:
        self.receipt_file.write_text(json.dumps(self.receipt))
        return subprocess.run([sys.executable, "-B", str(publisher), "--result", str(self.result),
            "--expected-ql-head", expected_head or args.expected_ql_head,
            "--output-root", str(self.root), "--github-env", str(self.github_env)], capture_output=True, text=True, check=False)

    def refused(self, message: str, expected_head: str | None = None) -> None:
        result = self.invoke(expected_head)
        self.assertNotEqual(result.returncode, 0, "Invalid actual corpus must be refused before publication")
        self.assertIn(message, result.stderr)
        self.assertEqual(self.github_env.read_bytes(), sentinel, "Refusal must preserve the existing receiving environment")

    def test_complete_actual_corpus_publishes_all_original_receiving_paths(self) -> None:
        result = self.invoke()
        self.assertEqual(result.returncode, 0, result.stderr)
        expected = sentinel + "".join(key + "=" + self.environment[key] + "\n" for key in sorted(self.environment)).encode()
        self.assertEqual(self.github_env.read_bytes(), expected)
        self.assertEqual(json.loads(result.stdout)["variables"], sorted(self.environment))

    def test_omitted_sourceform_body_and_opening_checkpoint_are_refused(self) -> None:
        missing = {"native-retained-source-workload/source-performance.json", "native-retained-source-workload/initial.checkpoint.json"}
        self.receipt["fixtures"] = [row for row in self.receipt["fixtures"] if str(Path(row["path"]).relative_to(self.run)) not in missing]
        self.refused("actual native fixture inventory membership differs")

    def test_missing_last_native_edition_is_refused(self) -> None:
        missing = "native-retained-workload/edition-180.history.json"
        self.receipt["fixtures"] = [row for row in self.receipt["fixtures"] if str(Path(row["path"]).relative_to(self.run)) != missing]
        self.refused("actual native fixture inventory membership differs")

    def test_missing_actual_postpulse_native_cuts_refuse_publication(self) -> None:
        original_fixtures = copy.deepcopy(self.receipt["fixtures"])
        for kind in ("release", "panic"):
            for suffix in ("pulse-checkpoint", "pending-pulse-checkpoint", "pending-pulse-history"):
                missing = "native-managed-order/" + kind + "." + suffix + ".json"
                with self.subTest(actual_native_member=missing):
                    self.assertEqual(sum(str(Path(row["path"]).relative_to(self.run)) == missing
                        for row in original_fixtures), 1, "Counterproof requires this actual native-produced cut")
                    self.receipt["fixtures"] = [row for row in original_fixtures
                        if str(Path(row["path"]).relative_to(self.run)) != missing]
                    self.refused("actual native fixture inventory membership differs")
        self.receipt["fixtures"] = original_fixtures

    def test_missing_same_source_prearm_member_refuses_publication(self) -> None:
        original_fixtures = copy.deepcopy(self.receipt["fixtures"])
        names=("stopped-force.source-return.json", "stopped-force-born0-management.json",
            "stopped-force-admission.json", "stopped-force-pending-management.json",
            "stopped-force-applied.json", "stopped-force-after-management.json",
            "stopped-force-restore-before-management.json", "stopped-force-restore-ack.json",
            "stopped-force-restored-pending-management.json", "stopped-force-restored-applied.json",
            "stopped-force-restored-after-management.json")
        for name in names:
            missing="native-prearm-force/"+name
            with self.subTest(actual_native_member=missing):
                self.assertEqual(sum(str(Path(row["path"]).relative_to(self.run))==missing
                    for row in original_fixtures),1,"Counterproof requires the original native producer bytes")
                self.receipt["fixtures"]=[row for row in original_fixtures
                    if str(Path(row["path"]).relative_to(self.run))!=missing]
                self.refused("actual native fixture inventory membership differs")
        self.receipt["fixtures"]=original_fixtures

    def test_missing_same_source_prearm_receiving_path_refuses_publication(self) -> None:
        self.environment_file.write_text("".join("export "+key+"="+shlex.quote(value)+"\n"
            for key,value in self.environment.items() if key!="QL_RETAINED_PERFORMANCE_PREARM_DIRECTORY"))
        self.refused("native kernel fixture environment is incomplete")

    def test_changed_same_source_prearm_original_bytes_refuse_publication(self) -> None:
        for name in ("stopped-force.source-return.json", "stopped-force-pending-management.json",
                     "stopped-force-restored-applied.json"):
            with self.subTest(actual_native_member=name):
                path=self.run/"native-prearm-force"/name
                body=path.read_bytes()
                path.unlink()
                path.write_bytes(body+b" ")
                self.refused("native fixture byte/hash declaration differs")
                path.unlink()
                source=original_run/"native-prearm-force"/name
                os.link(source,path)

    def test_missing_actual_journal_tail_cuts_refuse_publication(self) -> None:
        original_fixtures = copy.deepcopy(self.receipt["fixtures"])
        for suffix in ("origin-checkpoint", "complete", "empty", "checkpoint"):
            missing = "native-managed-order/journal-tail." + suffix + ".json"
            with self.subTest(actual_native_member=missing):
                self.assertEqual(sum(str(Path(row["path"]).relative_to(self.run)) == missing
                    for row in original_fixtures), 1, "Counterproof requires this actual native-produced cut")
                self.receipt["fixtures"] = [row for row in original_fixtures
                    if str(Path(row["path"]).relative_to(self.run)) != missing]
                self.refused("actual native fixture inventory membership differs")
        self.receipt["fixtures"] = original_fixtures

    def test_missing_actual_reservation_origin_refuses_publication(self) -> None:
        original_fixtures = copy.deepcopy(self.receipt["fixtures"])
        for kind in ("cancel", "lost"):
            missing = "native-score-reservations/" + kind + ".origin.json"
            with self.subTest(actual_native_member=missing):
                self.assertEqual(sum(str(Path(row["path"]).relative_to(self.run)) == missing
                    for row in original_fixtures), 1,
                    "Counterproof requires the same actual trial's prequeue native checkpoint")
                self.receipt["fixtures"] = [row for row in original_fixtures
                    if str(Path(row["path"]).relative_to(self.run)) != missing]
                self.refused("actual native fixture inventory membership differs")
        self.receipt["fixtures"] = original_fixtures

    def test_missing_dense_or_control_original_member_refuses_publication(self) -> None:
        original_fixtures = copy.deepcopy(self.receipt["fixtures"])
        for missing in ("original-current-native-source.json", "native-radius-control.json", "native-strength-control.json"):
            with self.subTest(actual_native_member=missing):
                self.assertEqual(sum(str(Path(row["path"]).relative_to(self.run)) == missing
                    for row in original_fixtures), 1, "Counterproof requires the genuine emitted native source")
                self.receipt["fixtures"] = [row for row in original_fixtures
                    if str(Path(row["path"]).relative_to(self.run)) != missing]
                self.refused("actual native fixture inventory membership differs")
        self.receipt["fixtures"] = original_fixtures

    def test_missing_dense_or_control_receiving_path_refuses_publication(self) -> None:
        for key in ("OI_NATIVE_DENSE_FIELD_SOURCE_ARTIFACT", "QL_PROCEDURAL_CONTROL_CONSUMER_ARTIFACT", "QL_PROCEDURAL_CONTROL_STRENGTH_ARTIFACT"):
            with self.subTest(actual_receiving_path=key):
                body = self.environment_file.read_bytes()
                self.environment_file.write_text("".join("export " + name + "=" + shlex.quote(value) + "\n"
                    for name,value in self.environment.items() if name != key))
                self.refused("native kernel fixture environment is incomplete")
                self.environment_file.write_bytes(body)

    def test_changed_dense_or_control_original_bytes_refuse_publication(self) -> None:
        for name in ("original-current-native-source.json", "native-radius-control.json", "native-strength-control.json"):
            with self.subTest(actual_native_member=name):
                path=self.run/name
                body=path.read_bytes()
                path.unlink()
                path.write_bytes(bytes([body[0]^1])+body[1:])
                self.refused("actual native fixture changed after producer qualification")
                path.write_bytes(body)

    def test_duplicate_actual_native_body_is_refused(self) -> None:
        self.receipt["fixtures"].append(copy.deepcopy(self.receipt["fixtures"][0]))
        self.refused("actual native fixture inventory membership differs")

    def test_changed_actual_native_bytes_are_refused(self) -> None:
        path = Path(self.receipt["fixtures"][0]["path"])
        body = path.read_bytes()
        path.unlink()
        path.write_bytes(bytes([body[0] ^ 1]) + body[1:])
        self.refused("actual native fixture changed after producer qualification")

    def test_missing_actual_opposite_zero_native_member_refuses_publication(self) -> None:
        original_fixtures = copy.deepcopy(self.receipt["fixtures"])
        names = ("native-opposite-zero-fixture.json", "native-opposite-zero-management-fixture.json",
            "native-opposite-zero-packets/baseline.packet.json", "native-opposite-zero-packets/baseline.basis.json",
            *("native-opposite-zero-management/" + name for name in (
                "baseline.pending.management.json", "baseline.basis.json", "baseline.applied-events.json",
                "baseline.input-journal.json", "baseline.current.management.json", "manifest.json")))
        for missing in names:
            with self.subTest(actual_native_member=missing):
                self.assertEqual(sum(str(Path(row["path"]).relative_to(self.run)) == missing
                    for row in original_fixtures), 1, "Counterproof needs the genuine native opposite-phase bytes")
                self.receipt["fixtures"] = [row for row in original_fixtures
                    if str(Path(row["path"]).relative_to(self.run)) != missing]
                self.refused("actual native fixture inventory membership differs")
        self.receipt["fixtures"] = original_fixtures

    def test_missing_actual_opposite_zero_receiving_path_refuses_publication(self) -> None:
        for key in ("QL_RETAINED_NATIVE_OPPOSITE_ZERO_FIXTURE",
                    "QL_RETAINED_NATIVE_OPPOSITE_ZERO_MANAGEMENT_FIXTURE",
                    "QL_RETAINED_NATIVE_OPPOSITE_ZERO_MANAGEMENT_DIRECTORY"):
            with self.subTest(actual_receiving_path=key):
                body = self.environment_file.read_bytes()
                self.environment_file.write_text("".join("export " + name + "=" + shlex.quote(value) + "\n"
                    for name, value in self.environment.items() if name != key))
                self.refused("native kernel fixture environment is incomplete")
                self.environment_file.write_bytes(body)

    def test_changed_actual_opposite_zero_original_bytes_refuse_publication(self) -> None:
        for relative in ("native-opposite-zero-fixture.json",
                         "native-opposite-zero-management/baseline.applied-events.json",
                         "native-opposite-zero-management/baseline.input-journal.json"):
            with self.subTest(actual_native_member=relative):
                path = self.run / relative
                body = path.read_bytes()
                path.unlink()
                path.write_bytes(bytes([body[0] ^ 1]) + body[1:])
                self.refused("actual native fixture changed after producer qualification")
                path.unlink()
                os.link(original_run / relative, path)

    def test_missing_actual_native_readmission_member_refuses_publication(self) -> None:
        original_fixtures = copy.deepcopy(self.receipt["fixtures"])
        for kind in ("world", "personal", "shared"):
            for name in ("producer-input.json", "native-preexecution.json", "native-child-started.json",
                         "native-stdout.json", "native-stderr.txt", "native-exit.json"):
                missing = "native-receiving-readmission/" + kind + "/" + name
                with self.subTest(actual_native_member=missing):
                    self.assertEqual(sum(str(Path(row["path"]).relative_to(self.run)) == missing
                        for row in original_fixtures), 1, "Counterproof needs the actual original native context activity")
                    self.receipt["fixtures"] = [row for row in original_fixtures
                        if str(Path(row["path"]).relative_to(self.run)) != missing]
                    self.refused("actual native fixture inventory membership differs")
        self.receipt["fixtures"] = original_fixtures

    def test_missing_actual_native_readmission_receiving_path_refuses_publication(self) -> None:
        self.environment_file.write_text("".join("export " + key + "=" + shlex.quote(value) + "\n"
            for key,value in self.environment.items() if key!="QL_RETAINED_RECEIVING_READMISSION_DIRECTORY"))
        self.refused("native kernel fixture environment is incomplete")

    def test_changed_actual_native_readmission_original_bytes_refuse_publication(self) -> None:
        for kind in ("world", "personal", "shared"):
            for name in ("producer-input.json", "native-preexecution.json", "native-child-started.json",
                         "native-stdout.json", "native-stderr.txt", "native-exit.json"):
                relative = "native-receiving-readmission/" + kind + "/" + name
                with self.subTest(actual_native_member=relative):
                    path=self.run/relative
                    body=path.read_bytes()
                    path.unlink()
                    path.write_bytes(bytes([body[0]^1])+body[1:] if body else b"lost original empty stderr")
                    # Changed size or same-length changed bytes must both fail
                    # before publishing even one receiving environment entry.
                    self.refused("native fixture byte/hash declaration differs" if not body
                                 else "actual native fixture changed after producer qualification")
                    path.unlink()
                    os.link(original_run/relative,path)

    def test_failed_actual_command_receipt_is_refused(self) -> None:
        self.receipt["commands"][0]["exit_code"] = 1
        self.refused("actual native producer failed or timed out")

    def test_lost_original_native_preexecution_custody_refuses_publication(self) -> None:
        self.receipt["commands"][0]["source_head"] = "0"*40
        self.refused("actual producer lost its durable original preexecution custody")

    def test_lost_owned_native_child_group_refuses_publication(self) -> None:
        self.receipt["commands"][0]["owned_process_group"] = {}
        self.refused("actual producer lost its owned child/session readback")

    def test_missing_original_native_environment_override_custody_refuses_publication(self) -> None:
        del self.receipt["commands"][0]["explicit_environment"]
        self.refused("actual producer lost its durable original preexecution custody")

    def test_changed_actual_native_environment_override_refuses_publication(self) -> None:
        command=next(row for row in self.receipt["commands"]
                     if "QL_NATIVE_WIRE_TEST" in row["explicit_environment"])
        command["explicit_environment"]["QL_NATIVE_WIRE_TEST"] += ".disconnected-producer"
        self.refused("actual producer lost its durable original preexecution custody")

    def test_lost_original_native_output_readback_refuses_publication(self) -> None:
        self.receipt["commands"][0]["original_output_complete"] = False
        self.refused("actual producer lost its complete original output custody")

    def test_changed_original_native_stderr_refuses_publication(self) -> None:
        path=Path(self.receipt["commands"][0]["log"])
        body=path.read_bytes()
        path.unlink()
        path.write_bytes(body+b"lost original stderr")
        self.refused("actual producer original output byte/hash declaration differs")

    def test_wrong_exact_source_is_refused(self) -> None:
        wrong = ("0" if args.expected_ql_head[0] != "0" else "1") + args.expected_ql_head[1:]
        self.refused("actual native fixture source/result/unique custody differs", wrong)


suite = unittest.defaultTestLoader.loadTestsFromTestCase(ActualPublication)
result = unittest.TextTestRunner(verbosity=2).run(suite)
print(json.dumps({"schema": "oi.actual-native-publication-counterproof/v1", "tests": result.testsRun,
    "failures": len(result.failures), "errors": len(result.errors), "actual_ql_head": args.expected_ql_head,
    "actual_fixture_records": len(original["fixtures"]), "original_receipt_sha256": hashlib.sha256(Path(pointer["receipt"]).read_bytes()).hexdigest(),
    "scope": "Actual prepared native bytes through the public receiving CLI; no native reexecution, full kernel or installed verdict"}, sort_keys=True))
sys.exit(0 if result.wasSuccessful() else 1)
