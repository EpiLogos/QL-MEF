"""Real specimen construction and filesystem publication; no substitute CLI."""
import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "scripts"))
import ql_agent_contracts as c
import ql_agent_corpus as corpus


class CorpusChecks(unittest.TestCase):
    def test_only_exact_native_exit_and_diagnostic_can_be_retained_as_refusal(self):
        executable = Path("/tmp/contradict-in-path/ql")
        reason = "observed lens state contradicts kernel derivation"
        valid = RuntimeError(f"{executable} exited 2: ql: {reason}\n")
        self.assertEqual(corpus.refusal_reason(valid, executable, reason), reason)
        for message in (f"{executable} exited 2: ql: unknown command `agent-event`\n",
                        f"{executable} exited -15: ql: {reason}\n",
                        f"{executable} exited 2: transport: {reason}\n",
                        f"{executable} exited 2: ql: {reason}\nextra diagnostic\n"):
            with self.assertRaises(c.ContractError):
                corpus.refusal_reason(RuntimeError(message), executable, reason)

    def test_specimen_retains_exact_material_and_observed_source_basis(self):
        request = corpus.specimen("coordinate-local", "one", {"lens": "L2'", "local-position": 3})
        event = request["event"]
        c.validate(event)
        material = event["material"]
        self.assertEqual(json.loads(material["text"]), {"lens": "L2'", "local-position": 3})
        self.assertEqual(material["revision"], "sha256:" + hashlib.sha256(material["text"].encode()).hexdigest())
        self.assertEqual(event["source_basis"], [{"ref": material["ref"], "revision": material["revision"]}])
        self.assertTrue(all(fact["origin"] == "observed" and fact["basis_refs"] == [material["ref"]]
                            for fact in event["observed"]))
        self.assertEqual(request["requested_heads"], [])
        self.assertNotIn("expected", request)
        self.assertNotIn("bindings", event)

    def test_template_families_have_one_fixed_split(self):
        self.assertEqual(corpus.SPLITS["coordinate-local"], "train")
        self.assertEqual(corpus.SPLITS["coordinate-absolute"], "validation")
        for family in ("relation-reversed", "relation-overlap", "completion-missing-side",
                       "context-inverse", "missing-semantic-evidence"):
            self.assertEqual(corpus.SPLITS[family], "test")
        for family, split in corpus.SPLITS.items():
            if family.endswith(("-overlap", "-reversed")):
                self.assertEqual(split, "test", "held-out traversal leaked through its completion template")
        with self.assertRaises(c.ContractError):
            corpus.specimen("random-row-split", "one", {})

    def test_output_is_exclusive_and_existing_material_is_preserved(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "manifest.json"
            corpus.publish(path, {"original": "owned bytes"})
            original = path.read_bytes()
            with self.assertRaises(FileExistsError):
                corpus.publish(path, {"replacement": True})
            self.assertEqual(path.read_bytes(), original)
            self.assertEqual(corpus.file_digest(path), "sha256:" + hashlib.sha256(original).hexdigest())

    def test_counterfactual_keeps_source_identity_but_updates_its_revision(self):
        original = corpus.specimen("coordinate-local", "one", {"lens": "L2'"})
        changed = corpus.counterfactual(original, "lens-face", "day")
        c.validate(changed["event"])
        self.assertEqual(changed["event"]["event_ref"], original["event"]["event_ref"])
        self.assertEqual(changed["event"]["material"]["ref"], original["event"]["material"]["ref"])
        self.assertNotEqual(changed["event"]["material"]["revision"], original["event"]["material"]["revision"])
        self.assertEqual(len(changed["event"]["observed"]), 2)
        self.assertEqual(len(original["event"]["observed"]), 1)
        self.assertEqual(json.loads(changed["event"]["material"]["text"])["lens-face"], "day")


if __name__ == "__main__":
    unittest.main()
