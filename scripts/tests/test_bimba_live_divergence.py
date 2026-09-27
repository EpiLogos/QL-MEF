"""Seed-vs-live divergence: comparison laws that must not drift silently."""
import copy
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("bimba_live_divergence", ROOT / "scripts/bimba-live-divergence.py")
tool = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tool)


class BimbaLiveDivergenceTests(unittest.TestCase):
    def test_literal_mirrors_upstream_generator(self):
        lists = {"c_5_resonances"}
        self.assertEqual(tool.literal("a, b,,", "c_5_resonances", lists), ["a", "b"])
        self.assertIsNone(tool.literal("   ", "c_1_name", lists))
        self.assertEqual(tool.literal({"b": 1, "a": 2}, "c_1_structure", lists), '{"b":1,"a":2}')
        self.assertEqual(tool.literal(7, "m_3_degree", lists), 7)

    def test_first_present_source_key_wins_per_target(self):
        mappings = {"c": {"lastUpdated": "c_3_updated_at", "updatedAt": "c_3_updated_at", "name": "c_1_name"}}
        got = tool.expected_properties({"updatedAt": "x", "lastUpdated": "y", "name": ""}, mappings, set())
        self.assertEqual(got, {"c_3_updated_at": {"value": "y", "source_key": "lastUpdated"}})

    def test_key_variant_is_reported_not_treated_as_the_mapped_key(self):
        live = {"m_3_5_degree": 12, "m_3_degreex": 1, "c_3_degree": 2}
        self.assertEqual(tool.key_variants(live, "m", "degree"), ["m_3_5_degree"])
        self.assertEqual(tool.key_variants(live, "c", "architectural_function"), [])

    def test_branch_grouping(self):
        self.assertEqual(tool.branch("#2-3-4-0-2"), "#2-3")
        self.assertEqual(tool.branch("#4.4.3-1"), "#4.4")
        self.assertEqual(tool.branch("#5"), "#5")

    def test_ledger_refuses_to_rewrite_an_existing_discrepancy(self):
        ledger = json.loads((ROOT / "fixtures/kernel/m-ledger-v1.json").read_text(encoding="utf-8"))
        existing = next((d for d in ledger["discrepancies"] if d["id"].startswith("bimba-live-")), None)
        if existing is None:
            self.skipTest("no bimba-live discrepancies recorded")
        changed = copy.deepcopy(ledger)
        target = next(d for d in changed["discrepancies"] if d["id"] == existing["id"])
        target["detail"] = json.dumps({"tampered": True})
        with tempfile.TemporaryDirectory() as tmp:
            work = Path(tmp)
            path = work / "m-ledger-v1.json"
            path.write_text(json.dumps(changed), encoding="utf-8")
            report = (ROOT / "target/bimba-live/divergence-report.json")
            if not report.exists():
                self.skipTest("run `bimba-live-divergence.py compare` first")
            (work / "divergence-report.json").write_bytes(report.read_bytes())
            saved = tool.LEDGER
            tool.LEDGER = path
            try:
                with self.assertRaises(SystemExit) as raised:
                    tool.ledger(type("Args", (), {"workdir": work})())
                self.assertIn("do not rewrite", str(raised.exception))
            finally:
                tool.LEDGER = saved


if __name__ == "__main__":
    unittest.main()
