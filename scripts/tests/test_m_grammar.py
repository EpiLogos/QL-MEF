"""The nesting threshold: after a position-4 segment the separator is '.'."""
import copy
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("coordinate_grammar", ROOT / "scripts/coordinate-grammar.py")
g = importlib.util.module_from_spec(spec)
spec.loader.exec_module(g)


class NestingThresholdTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.registry = g.read(g.REGISTRY)
        cls.ledger = g.read(g.LEDGER)

    def test_rule(self):
        cases = {
            "M4-0": "M4.0", "#4-5": "#4.5", "M3-4-4-4": "M3-4.4.4", "M3-4-(5/0)-1": "M3-4.(5/0)-1",
            "#2-3-4-0-0": "#2-3-4.0-0", "M4.4-1": "M4.4.1",
        }
        for spelled, canonical in cases.items():
            self.assertTrue(g.violations(spelled), spelled)
            self.assertEqual(g.canonical(spelled), canonical)
            self.assertFalse(g.violations(canonical), canonical)
        # Canonical: dots after 4; dashes after any other position; 14 is not 4; frames untouched.
        for ok in ["M4.0", "M3-4.0-14", "M1-4.5-0", "M3-14-2", "M2-5-(0/1)-1", "M0-4.(4.0/1-4.4/5)", "#4", "M", "#2-4.3"]:
            self.assertEqual(g.violations(ok), [], ok)
            self.assertEqual(g.canonical(ok), ok)

    def test_threshold_parent(self):
        self.assertEqual(g.threshold_parent("#3-4-4-4"), "#3-4")
        self.assertEqual(g.threshold_parent("#3-4.0-4-1"), "#3-4.0-4")
        self.assertEqual(g.threshold_parent("#4-2"), "#4")

    def test_checkout_registry_is_fully_recorded(self):
        self.assertEqual(g.check(self.registry, self.ledger), [])
        groups = g.registry_findings(self.registry)
        recorded = {d["id"] for d in self.ledger["discrepancies"] if d["id"].startswith(g.PREFIX)}
        self.assertEqual(recorded, {g.PREFIX + parent for parent in groups})
        # The root-4 duplicates of the live map: M4-0..5 beside M4.0..5.
        self.assertEqual([i["kind"] for i in groups["#4"]], ["duplicate"] * 6)

    def test_new_violation_fails_until_recorded(self):
        registry = copy.deepcopy(self.registry)
        node = copy.deepcopy(next(n for n in registry["nodes"] if n["source_ref"] == "#2-4.3"))
        node.update(source_ref="#1-3-4-9", aliases=[])
        registry["nodes"].append(node)
        errors = g.check(registry, self.ledger)
        self.assertEqual(len(errors), 1)
        self.assertIn("#1-3-4-9", errors[0])
        ledger, added = g.record(registry, copy.deepcopy(self.ledger))
        self.assertEqual(added, 1)
        self.assertEqual(g.check(registry, ledger), [])

    def test_record_never_replaces_a_reviewed_record(self):
        ledger = copy.deepcopy(self.ledger)
        entry = next(d for d in ledger["discrepancies"] if d["id"] == g.PREFIX + "#4")
        entry["state"] = "accepted"
        ledger, added = g.record(self.registry, ledger)
        self.assertEqual(added, 0)
        self.assertEqual(next(d for d in ledger["discrepancies"] if d["id"] == g.PREFIX + "#4")["state"], "accepted")

    def test_live_record_whose_subjects_left_the_registry_is_stale(self):
        registry = copy.deepcopy(self.registry)
        registry["nodes"] = [n for n in registry["nodes"] if n["source_ref"] != "#4-0"]
        self.assertTrue(any("left the registry" in e for e in g.check(registry, self.ledger)))


if __name__ == "__main__":
    unittest.main()
