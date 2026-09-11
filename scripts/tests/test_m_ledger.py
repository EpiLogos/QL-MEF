"""K3 source/matrix-lock regressions, including actual native-C observations."""
import copy
import importlib.util
import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("m_ledger_tool", ROOT / "scripts/m-ledger.py")
ledger = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ledger)


class MLedgerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.base = ledger.read(ROOT / ledger.LEDGER)
        cls.registry = ledger.read(ROOT / ledger.REGISTRY)
        cls.temp = tempfile.TemporaryDirectory()
        cls.world = Path(cls.temp.name)
        paths = {m[k]["path"] for m in cls.base["matrices"] for k in ("data", "rationale")}
        paths.update(i["path"] for i in cls.base["implementations"])
        paths.update(e["artifact"]["path"] for e in cls.base["evidence"])
        paths.update([str(ledger.REGISTRY), "fixtures/kernel/m-ledger-v1.schema.json"])
        for p in paths:
            target = cls.world / p; target.parent.mkdir(parents=True, exist_ok=True); shutil.copyfile(ROOT / p, target)
        probe = cls.world / "probe"
        subprocess.run(["cc", "-std=c11", "-O1", "-Wall", "-Wextra", "-Werror", "-pedantic", "-I" + str(ROOT / "c/include"), str(ROOT / "migration/epi-kernel/k2-m-tree-probe.c"), str(ROOT / "c/src/m_tree.c"), "-o", str(probe)], check=True)
        cls.native = cls.world / "native.jsonl"
        with cls.native.open("w") as out:
            result = subprocess.run([str(probe)], stdout=out, stderr=subprocess.PIPE, check=True, text=True)
        if "126098 checks passed" not in result.stderr:
            raise AssertionError(result.stderr)
        cls.lines = cls.native.read_text().splitlines()

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def resign(self, document):
        document["ledger_revision"] = ledger.digest(ledger.canonical({k: v for k, v in document.items() if k != "ledger_revision"}))
        return document

    def test_seed_determinism_and_source_inventory(self):
        # The deterministic K3 seed is unchanged; the K4 census (see
        # fixtures/kernel/census/) extends it with one row per M1/M2/M3
        # coordinate, and refresh preserves those census additions exactly.
        seed = ledger.refresh(ROOT)
        self.assertEqual(len(seed["rows"]), 192)
        self.assertEqual(len(seed["matrices"]), 10)
        by_id = {r["id"]: r for r in self.base["rows"]}
        for row in seed["rows"]:
            self.assertIn(row["id"], by_id)
            for field in ("role", "scope", "coordinates", "source"):
                self.assertEqual(row[field], by_id[row["id"]][field])
        self.assertEqual(ledger.refresh(ROOT, copy.deepcopy(self.base)), self.base)
        ledger.verify(ROOT, self.base)

    def test_refresh_preserves_assessments_bindings_discrepancies_and_new_rows(self):
        previous = copy.deepcopy(self.base)
        previous["rows"][0]["dispositions"] = {"c": "planned"}
        previous["rows"][0]["invariants"] = ["reviewed invariant"]
        previous["discrepancies"][0]["detail"] = "reviewed description"
        result = ledger.refresh(ROOT, previous)
        self.assertEqual(result["rows"][0]["dispositions"], {"c": "planned"})
        self.assertEqual(result["rows"][0]["invariants"], ["reviewed invariant"])
        self.assertEqual(result["discrepancies"][0]["detail"], "reviewed description")

    def test_missing_capability_cannot_hide_behind_a_recomputed_digest(self):
        changed = copy.deepcopy(self.base); changed["rows"].pop(0)
        with self.assertRaisesRegex(ValueError, "source capability missing"):
            ledger.verify(ROOT, self.resign(changed))

    def test_forged_source_coordinate_is_not_a_resolution(self):
        changed = copy.deepcopy(self.base); changed["rows"][0]["coordinates"] = ["M5"]
        with self.assertRaisesRegex(ValueError, "source import disagreement"):
            ledger.verify(ROOT, self.resign(changed))

    def test_source_pointer_changes_cannot_silently_retarget_a_capability(self):
        changed = copy.deepcopy(self.base); changed["rows"][0]["source"]["pointer"] = "/capabilities/1"
        with self.assertRaisesRegex(ValueError, "source import disagreement"):
            ledger.verify(ROOT, self.resign(changed))

    def test_markdown_rationale_is_locked_without_replacing_it(self):
        path = self.world / self.base["matrices"][0]["rationale"]["path"]
        original = path.read_bytes()
        try:
            path.write_bytes(original + b"\nchanged rationale\n")
            with self.assertRaisesRegex(ValueError, "stale content/evidence lock"):
                ledger.verify(self.world, self.base)
        finally:
            path.write_bytes(original)

    def test_source_inventory_expansion_is_automatic_and_deletion_explicit(self):
        path = self.world / self.base["matrices"][0]["data"]["path"]
        original = path.read_bytes()
        try:
            document = json.loads(original); document["capabilities"].append({"id": "K4-AUTO-TEST", "name": "new source capability", "coordinate": "M1-0"})
            path.write_text(json.dumps(document))
            expanded = ledger.refresh(self.world, copy.deepcopy(self.base))
            self.assertEqual(len(expanded["rows"]), len(self.base["rows"]) + 1)
            ledger.verify(self.world, expanded)
            document["capabilities"].pop(); path.write_bytes(original)
            shrunk = ledger.refresh(self.world, expanded)
            self.assertEqual(len(shrunk["rows"]), len(self.base["rows"]) + 1, "removed source must not be silently forgotten")
            with self.assertRaisesRegex(ValueError, "orphan/stale source capability"):
                ledger.verify(self.world, shrunk)
        finally:
            path.write_bytes(original)

    def test_invalid_schema_complete_and_unsafe_evidence_paths_fail(self):
        changed = copy.deepcopy(self.base); changed["assessments"]["unassessed"]["readiness"]["rust"]["status"] = "COMPLETE"
        with self.assertRaisesRegex(ValueError, "ledger schema"):
            ledger.verify(ROOT, self.resign(changed))
        changed = copy.deepcopy(self.base); changed["evidence"][0]["artifact"]["path"] = "../../outside"
        with self.assertRaisesRegex(ValueError, "unsafe repository path"):
            ledger.verify(ROOT, self.resign(changed))

    def test_deleted_symbol_and_stale_ledger_digest_fail(self):
        changed = copy.deepcopy(self.base); changed["implementations"][0]["symbol"] = "does_not_exist_k3"
        with self.assertRaisesRegex(ValueError, "missing implementation symbol"):
            ledger.verify(ROOT, self.resign(changed))
        changed = copy.deepcopy(self.base); changed["ledger_revision"] = "0" * 64
        with self.assertRaisesRegex(ValueError, "stale ledger_revision"):
            ledger.verify(ROOT, changed)

    def test_actual_native_c_full_descriptor_observation_matches(self):
        self.assertEqual(ledger.observations(self.registry, self.native, "c"), [])

    def changed_observation(self, rows):
        path = self.world / "mutated.jsonl"; path.write_text("\n".join(rows) + "\n"); return path

    def test_missing_coordinate_in_c_or_rust_observation_is_visible(self):
        lines = list(self.lines)
        at = next(i for i, line in enumerate(lines) if json.loads(line).get("source_ref") == "#1")
        lines.pop(at)
        for peer in ("c", "rust"):
            findings = ledger.observations(self.registry, self.changed_observation(lines), peer)
            self.assertTrue(any(f["code"] == "missing-coordinate" and f["peer"] == peer for f in findings))

    def test_structural_disagreement_duplicate_and_orphan_observations(self):
        lines = list(self.lines); node = json.loads(lines[1]); node["parent_id"] = "bad-parent"; lines[1] = json.dumps(node)
        self.assertTrue(any(f["code"] == "structural-disagreement" for f in ledger.observations(self.registry, self.changed_observation(lines), "c")))
        lines.append(lines[1]); node["id"] = "not-in-registry"; lines.append(json.dumps(node))
        codes = {f["code"] for f in ledger.observations(self.registry, self.changed_observation(lines), "rust")}
        self.assertIn("duplicate-observation", codes); self.assertIn("orphan-observation", codes)

    def test_empty_observer_never_defaults_to_parity(self):
        findings = ledger.observations(self.registry, self.changed_observation([]), "cpp")
        self.assertTrue(any(f["code"] == "observation-revision" for f in findings))
        self.assertEqual(sum(f["code"] == "missing-coordinate" for f in findings), 1876)

    def test_candidate_extraction_does_not_rewrite_slashes_or_expand_ranges(self):
        refs = [m.group() for m in ledger.COORDINATE.finditer("M0-4.0/1/2 / M0-4.0/1-2 -> M4.5-0 / M4.5.0")]
        self.assertEqual(refs, ["M0-4.0/1/2", "M0-4.0/1-2", "M4.5-0", "M4.5.0"])

    def test_primed_or_suffixed_expressions_cannot_collapse_to_unprimed_prefixes(self):
        for expression in ("M1-4′", "M1-4'", "M1-4x", "#0-4.0/1/2′"):
            self.assertEqual(list(ledger.COORDINATE.finditer(expression)), [], expression)
        self.assertEqual([m.group() for m in ledger.COORDINATE.finditer("M1/M2")], ["M1", "M2"])


if __name__ == "__main__":
    unittest.main()
