"""Run the installed owner CLI; these checks never simulate QL or a model."""
import json
import os
import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/ql_agent_encounter.py"


class NativeEncounterChecks(unittest.TestCase):
    def run_encounter(self, text, *, body="pi", kind="input", ql="ql-agent", generation=1):
        occasion = {"schema": "ql.agent-native-occasion/v1", "kind": kind, "generation": generation,
                    "text": text, "body_ref": "ql:body:" + body, "session_ref": "session:" + body,
                    "native_event_ref": "occasion:" + body}
        result = subprocess.run([sys.executable, str(SCRIPT), "--ql", ql],
                                input=json.dumps(occasion).encode(), capture_output=True, timeout=8,
                                env={**os.environ, "QL_AGENT_DECISION_CONFIG": "/does/not/exist"})
        return result

    def test_ordinary_input_and_tool_material_bypass_owner_and_provider(self):
        for text, kind in [("Repair this ordinary file", "input"),
                           ('QL state: {"lens":"L2"}', "tool-result")]:
            result = self.run_encounter(text, kind=kind, ql="/does/not/exist")
            self.assertEqual(result.returncode, 0, result.stderr)
            receipt = json.loads(result.stdout)
            self.assertEqual(receipt["disposition"], "ineligible")
            self.assertEqual(receipt["provider_calls"], 0)
            self.assertNotIn("projection", receipt)

    def test_native_formal_harmonic_reading_and_body_parity(self):
        text = 'QL state: {"lens":"L2\'","local-position":3,"coordinate-face":"direct","musical-basis":"chromatic"}'
        readings = []
        for body, generation in (("prime", 2), ("pi", 17)):
            result = self.run_encounter(text, body=body, generation=generation)
            self.assertEqual(result.returncode, 0, result.stderr)
            receipt = json.loads(result.stdout)
            self.assertEqual(receipt["generation"], generation)
            self.assertEqual(receipt["provider_calls"], 0)
            projection = receipt["projection"]
            self.assertEqual(projection["event"]["generation"], 0)
            self.assertEqual(projection["decision_head_ids"], [])
            pitch = next(f["value"] for f in projection["harmonic"]["harmonic"] if f["field"] == "pitch-class")
            self.assertEqual(pitch, 11)
            readings.append(projection)
        self.assertEqual(readings[0]["frame"], readings[1]["frame"])
        self.assertEqual(readings[0]["determination"], readings[1]["determination"])
        self.assertEqual(readings[0]["harmonic"], readings[1]["harmonic"])
        self.assertNotEqual(readings[0]["event"]["bindings"], readings[1]["event"]["bindings"])

    def test_semantic_reading_retains_native_candidates_without_inference(self):
        result = self.run_encounter("QL read: Study the connectivity implied by these distances.")
        self.assertEqual(result.returncode, 0, result.stderr)
        receipt = json.loads(result.stdout)
        self.assertEqual(receipt["provider_calls"], 0)
        self.assertEqual(set(receipt["projection"]["decision_head_ids"]), {"semantic-faculty", "semantic-operation"})
        self.assertEqual(receipt["projection"]["determination"]["learned"], [])

    def test_native_impossible_and_malformed_state_is_refused(self):
        for text in ('QL state: {"lens":"L99"}', 'QL state: {"lens":"L2","lens":"L3"}',
                     'QL state: {"lens":"L2","lens-face":"night"}', 'QL read: ', 'QL state: null'):
            result = self.run_encounter(text)
            self.assertEqual(result.returncode, 2, result.stdout)
            self.assertIn(b"QL encounter refused:", result.stderr)


if __name__ == "__main__":
    unittest.main()
