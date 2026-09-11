"""Source spelling/provenance and K6 refresh regressions, without live services."""
import copy
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, ROOT / path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


compiler = module("m2_correspondences", "scripts/m2-correspondences.py")
ledger = module("m2_ledger", "scripts/m2-ledger.py")


class CorrespondenceTests(unittest.TestCase):
    def test_explicit_note_spelling_and_unsupported_notation(self):
        self.assertEqual(compiler.spelled_steps("C - D - E - F - G - A - B - C"),
                         [0, 4, 8, 10, 14, 18, 22, 24])
        self.assertEqual(compiler.spelled_steps("C - D - E♭+ - F - G - A - B♭ - C"),
                         [0, 4, 7, 10, 14, 18, 20, 24])
        for value in ["C - D - E↓ - F - G - A - B - C", "modal variant",
                      "C - D - E - F - G - A - B - D", "C - D - E - E - G - A - B - C"]:
            self.assertIsNone(compiler.spelled_steps(value), value)

    def test_compiled_field_preserves_source_and_separate_readings(self):
        source = ROOT / "target/m2-bimba-source"
        if not source.is_dir():
            self.skipTest("source-byte regeneration is exercised by mandatory M2 acceptance")
        before = {p: compiler.sha(source / p) for p in [compiler.NODE_PATH]}
        field = compiler.compile_field(source)
        self.assertEqual(field, compiler.load(ROOT / compiler.OUTPUT))
        self.assertEqual(compiler.native(field), (ROOT / compiler.NATIVE).read_text())
        self.assertEqual(len(field["rules"]), 127)
        self.assertEqual(len(field["gaps"]), 17)
        self.assertEqual({r["colour_name"] for r in field["rules"]}, {None, "yellow", "silver", "red"})
        self.assertTrue(all(len(r["claims"]) == 4 for r in field["rules"]))
        self.assertEqual(before, {p: compiler.sha(source / p) for p in before})

    def test_changed_source_bytes_cannot_be_promoted_by_refresh(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory)
            path = source / compiler.NODE_PATH
            path.parent.mkdir(parents=True)
            path.write_text("[]\n")
            with self.assertRaisesRegex(ValueError, "pinned Bimba node source"):
                compiler.compile_field(source)


class K6RefreshTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.base = ledger.load(ledger.m.LEDGER)

    def test_refresh_is_identity_stable_and_preserves_census(self):
        original = copy.deepcopy(self.base)
        refreshed = ledger.materialize(self.base)
        self.assertEqual(original, self.base, "refresh must not mutate its caller")
        self.assertEqual(refreshed, ledger.materialize(refreshed))
        for kind in ("rows", "implementations", "evidence", "discrepancies"):
            before = {r["id"]: r for r in self.base[kind] if r["id"].startswith(("census:", "k4:", "k4-"))}
            after = {r["id"]: r for r in refreshed[kind] if r["id"] in before}
            self.assertEqual(before, after)
        source = next(r for r in refreshed["rows"] if r["id"] == "deep-M2:M2-C10")
        self.assertEqual(source["dispositions"]["c"], "bound")
        self.assertTrue(any(b.startswith("k6-m2:") for b in source["bindings"]), "M2 aliases must join #2 identities")

    def test_external_assessment_and_owned_decision_are_not_rewritten(self):
        base = copy.deepcopy(self.base)
        row = next(r for r in base["rows"] if r["id"] == "deep-M2:M2-C10")
        row["assessment"] = "parallel:reviewed-M2"
        base["assessments"][row["assessment"]] = copy.deepcopy(base["assessments"]["unassessed"])
        d = next(d for d in base["discrepancies"] if d["id"] == "k6-m2:difference-asma")
        d["state"] = "proposed"
        d["proposal"] = {"fixture": "retained reviewed proposal"}
        original = copy.deepcopy(base)
        new = ledger.materialize(base)
        self.assertEqual(next(r for r in new["rows"] if r["id"] == row["id"]), row)
        self.assertEqual(next(x for x in new["discrepancies"] if x["id"] == d["id"]), d)
        self.assertEqual(base, original)
