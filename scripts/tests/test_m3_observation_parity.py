"""Independent native/source observation validation and mutation regressions.

The positive fixtures are explicitly synthetic, not native execution receipts.
Real producer output is checked by the CLI with exact producer-revision binding.
"""
import copy
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location("m3_observation", ROOT / "scripts/m3-observation-parity.py")
observer = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(observer)


class M3ObservationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        root = os.environ.get("M3_SOURCE_ROOT")
        if not root:
            raise unittest.SkipTest("M3_SOURCE_ROOT absent; independent source observation tests not run")
        registry, nodes, edges = observer.source.read_source(Path(root))
        cls.projection = observer.source.project(registry, nodes, edges)
        cls.audit = observer.source.Audit(cls.projection).run()
        cls.synthetic = observer.expected_rows(cls.projection, cls.audit)

    def rows(self):
        return copy.deepcopy(self.synthetic)

    def check(self, rows):
        return observer.check(self.projection, self.audit, rows)

    def test_synthetic_valid_stream_is_complete_but_not_an_execution_receipt(self):
        result = self.check(self.synthetic)
        self.assertEqual(result["status"], "passed")
        self.assertEqual(result["checked"], {"coordinate_bindings": 623, "transcriptions": 128, "clock_frames": 1441})
        self.assertEqual(result["differences"], [])
        self.assertNotIn("producer_revision", result)
        self.assertNotIn("input_sha256", result)
        self.assertEqual(result["standing"]["rust"], "requires-separate-executed-C-Rust-comparison")
        self.assertEqual(result["standing"]["symbolic_clock_placeholders"], "not-promoted")

    def test_shared_upper_lower_transposition_cannot_pass(self):
        rows = self.rows()
        expected = {r[2]: r for r in self.synthetic if r[:2] == ["node", 5]}
        for row in rows:
            if row[:2] == ["node", 5]:
                address = row[2]
                other = expected[((address & 7) << 3) | (address >> 3)]
                row[3:] = other[3:]
        result = self.check(rows)
        self.assertEqual(result["status"], "failed")
        self.assertEqual(len(result["differences"]), 56)
        self.assertEqual(result["differences"][0]["key"], ["node", 5, 1])
        self.assertEqual(result["differences"][0]["expected"][-1], "#3-1-2-1")
        self.assertEqual(result["differences"][0]["observed"][-1], "#3-1-1-2")

    def test_correct_reference_with_wrong_stable_id_fails(self):
        rows = self.rows()
        row = next(r for r in rows if r[:3] == ["node", 5, 1])
        row[3] = "0000000000000001"
        result = self.check(rows)
        self.assertEqual(result["status"], "failed")
        self.assertEqual(len(result["differences"]), 1)

    def test_missing_native_source_binding_cannot_be_covered_by_other_records(self):
        rows = self.rows()
        rows.pop(0)
        rows.extend([["codon", 0], ["pair", 0]])
        with self.assertRaisesRegex(ValueError, "missing native observations"):
            self.check(rows)

    def test_duplicate_and_out_of_range_indices_are_rejected(self):
        rows = self.rows()
        rows.append(copy.deepcopy(rows[0]))
        with self.assertRaisesRegex(ValueError, "duplicate native observation"):
            self.check(rows)
        rows = self.rows()
        rows[0][2] = 1
        with self.assertRaisesRegex(ValueError, "out-of-domain"):
            self.check(rows)

    def test_clock_boundary_omission_cannot_pass_two_cover_evidence(self):
        for step in [0, 359, 360, 719, 720, 1079, 1080, 1439, 1440]:
            rows = [r for r in self.synthetic if r[:2] != ["clock", step]]
            with self.subTest(step=step), self.assertRaisesRegex(ValueError, "missing native observations"):
                self.check(rows)

    def test_layer_and_double_cover_lineage_are_not_modulo360_aliases(self):
        for step, field in [(360, 4), (719, 2), (720, 9), (1440, 9)]:
            rows = self.rows()
            row = next(r for r in rows if r[:2] == ["clock", step])
            row[field] = 0
            result = self.check(rows)
            with self.subTest(step=step, field=field):
                self.assertEqual(result["status"], "failed")
                self.assertEqual(len(result["differences"]), 1)

    def test_clock_backbone_cannot_be_replaced_with_fifteenth_degree_flag_identity(self):
        rows = self.rows()
        row = next(r for r in rows if r[:2] == ["clock", 15])
        row[11] = row[10]
        result = self.check(rows)
        self.assertEqual(result["status"], "failed")
        self.assertEqual(len(result["differences"]), 1)

    def test_clockwise_and_polar_ids_checked_against_source_edges(self):
        rows = self.rows()
        row = next(r for r in rows if r[:2] == ["clock", 359])
        row[12], row[13] = row[13], row[12]
        self.assertEqual(self.check(rows)["status"], "failed")

    def test_rna_is_t_to_u_not_polarity_flip_or_a_dna_alias(self):
        for address in [1, 6, 21, 63]:
            rows = self.rows()
            row = next(r for r in rows if r[:3] == ["transcription", address, 1])
            row[3] = "XXX"
            result = self.check(rows)
            with self.subTest(address=address):
                self.assertEqual(result["status"], "failed")
                self.assertEqual(len(result["differences"]), 1)

    def test_unknown_shapes_boolean_keys_and_silent_type_coercion_fail(self):
        for row in [["unknown", 0], ["clock", 0], ["node", True, 0, "a", "#3"]]:
            with self.subTest(row=row), self.assertRaises(ValueError):
                self.check([row] + self.rows())
        rows = self.rows()
        row = next(r for r in rows if r[:2] == ["clock", 0])
        row[4] = False
        self.assertEqual(self.check(rows)["status"], "failed")

    def test_other_engine_operations_are_neither_recertified_nor_mistaken_for_source_bindings(self):
        rows = self.rows() + [[kind, "opaque"] for kind in sorted(observer.OTHER_KINDS)]
        result = self.check(rows)
        self.assertEqual(result["status"], "passed")
        self.assertEqual(result["standing"]["other_native_operations"], "not-recertified-by-this-check")
        self.assertEqual(result["input_categories"]["quaternion"], 1)

    def test_non_finite_or_malformed_json_cannot_be_native_evidence(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "observed.jsonl"
            for text in ['["clock",NaN]\n', '["clock",Infinity]\n', '["clock",1e999]\n', '{broken']:
                path.write_text(text)
                with self.subTest(text=text), self.assertRaises(ValueError):
                    observer.read_observations(path)

    def test_cli_rejects_producer_revision_mismatch_before_reading_source(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            revision = root / "revision.txt"
            revision.write_text("a" * 40)
            result = subprocess.run([
                "python3", str(ROOT / "scripts/m3-observation-parity.py"),
                "--source-root", str(root / "absent-source"), "--input", str(root / "absent-input"),
                "--producer-revision-file", str(revision), "--expected-revision", "b" * 40,
            ], capture_output=True, text=True, check=False)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("producer/expected revision mismatch", result.stderr)
            self.assertNotIn("No such file", result.stderr)


if __name__ == "__main__":
    if not os.environ.get("M3_SOURCE_ROOT"):
        raise SystemExit("M3_SOURCE_ROOT is required for the explicit observation test run")
    unittest.main()
