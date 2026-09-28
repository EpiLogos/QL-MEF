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
    @classmethod
    def setUpClass(cls):
        cls.field = compiler.load(ROOT / compiler.OUTPUT)
        cls.registry = compiler.load(ROOT / "fixtures/kernel/m-tree-v1.json")

    def test_no_bimba_spelled_tuning_is_claimed_without_a_map_spelling(self):
        # The map states tonic, dominant and ajnas, not a full scale spelling.
        for rule in self.field["rules"]:
            self.assertIsNone(rule["spelled_steps24"])
            self.assertEqual((rule["interval_literal"], rule["planetary_mode_literal"], rule["element_literal"]), ("", "", ""))

    def test_claims_point_at_properties_the_map_record_holds(self):
        nodes = {n["source_ref"]: n for n in self.registry["nodes"]}
        self.assertEqual(len(self.field["rules"]), 127)
        self.assertEqual(len(self.field["gaps"]), 17)
        self.assertEqual({r["colour_name"] for r in self.field["rules"]}, {None, "yellow", "silver", "red"})
        for rule in self.field["rules"]:
            for claim in rule["claims"]:
                record = self.registry["records"][nodes[claim["coordinate"]]["records"][0]]
                self.assertIn(claim["property"], record["property_keys"])
                self.assertEqual(claim["pointer"], f"/{record['record_index']}/{claim['property']}")

    def test_recomputes_exactly_from_the_registry_map(self):
        read = compiler.bimba_map.CACHE
        if not read.is_file() or compiler.bimba_map.load(read)["content_sha256"] != self.registry["source_revision"]:
            self.skipTest("no read of the registry's map; the committed field is checked above")
        field = compiler.compile_field(compiler.bimba_map.load(read))
        self.assertEqual(field, self.field)
        self.assertEqual(compiler.native(field), (ROOT / compiler.NATIVE).read_text())

    def test_a_read_of_another_map_is_refused(self):
        with self.assertRaisesRegex(ValueError, "not the map the registry was built from"):
            compiler.compile_field({"content_sha256": "0" * 64, "nodes": {}, "relations": []})


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
