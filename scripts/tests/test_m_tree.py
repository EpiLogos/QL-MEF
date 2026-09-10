#!/usr/bin/env python3
"""Source-specific and mutation acceptance for the native M tree generator."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("m_tree", ROOT / "scripts/generate-m-tree.py")
g = importlib.util.module_from_spec(spec)
spec.loader.exec_module(g)


class MTreeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = g.read(ROOT / g.SNAPSHOT)
        cls.manifest = g.model(cls.source)

    def test_regeneration_and_full_content_digest(self):
        self.assertEqual(g.render(self.manifest), (ROOT / g.MANIFEST).read_text())
        self.assertEqual(g.tables(self.manifest), (ROOT / g.TABLES).read_text())
        body = dict(self.manifest)
        revision = body.pop("registry_revision")
        self.assertEqual(g.digest(body), revision)
        self.assertEqual(g.digest(self.source), body["source_snapshot_sha256"])

    def test_source_coverage_and_asymmetry(self):
        m = self.manifest
        self.assertEqual(len(m["nodes"]), 1876)
        self.assertEqual(len(m["relations"]), 21083)
        self.assertEqual(len(m["records"]), 24831)
        self.assertEqual(len(m["alternate_notation_groups"]), 8)
        by_ref = {n["source_ref"]: n for n in m["nodes"]}
        self.assertEqual([by_ref[f"#{i}"]["subtree_count"] for i in range(6)], [108, 43, 597, 996, 100, 31])
        self.assertNotIn("#0-4.0", by_ref)
        self.assertNotIn("#3-5-5", by_ref)
        self.assertEqual(by_ref["#0-4.0/1/2"]["parent_id"], by_ref["#0-4"]["id"])
        self.assertEqual(by_ref["#0-4.0/1/2"]["local_segment"], "0/1/2")
        self.assertNotEqual(by_ref["#0-4.0/1/2"]["id"], by_ref["#0-4.0/1-2"]["id"])
        self.assertEqual(by_ref["#2-4"]["source_parent_refs"], ["#2", "#2-4.5"])
        self.assertEqual(len(m["parent_discrepancies"]), 1)
        self.assertEqual(sum(r["cross_m"] for r in m["relations"]), 2676)
        self.assertEqual(sum(r["from_id"] is None or r["to_id"] is None for r in m["relations"]), 450)

    def test_deeper_asymmetric_extension_preserves_existing_ids(self):
        source = copy.deepcopy(self.source)
        original = {n["source_ref"]: n["id"] for n in self.manifest["nodes"]}
        previous = "#5"
        # Well beyond the six aggregate roots and seven-deep historical sample.
        for depth in range(18):
            ref = previous + ("." if depth % 2 else "-") + "17"
            source["nodes"].append({"source_ref": ref, "names": [], "aliases": [],
                "source_parent_refs": [previous], "lexical_parent_source_ref": previous, "records": [0]})
            previous = ref
        grown = g.model(source)
        nodes = {n["source_ref"]: n for n in grown["nodes"]}
        self.assertEqual(nodes[previous]["depth"], 19)
        self.assertEqual(len(nodes), len(original) + 18)
        self.assertTrue(all(nodes[ref]["id"] == identity for ref, identity in original.items()))
        self.assertNotEqual(grown["registry_revision"], self.manifest["registry_revision"])

    def test_collision_or_invalid_source_cannot_silently_normalize(self):
        duplicate = copy.deepcopy(self.source)
        duplicate["nodes"].append(duplicate["nodes"][0])
        with self.assertRaisesRegex(ValueError, "duplicate coordinate"):
            g.model(duplicate)
        dangling = copy.deepcopy(self.source)
        dangling["relations"][0]["from_ref"] = "#2-999999"
        with self.assertRaisesRegex(ValueError, "missing source coordinate"):
            g.model(dangling)
        ambiguous = copy.deepcopy(self.source)
        node = next(n for n in ambiguous["nodes"] if n["source_ref"] == "#0-4.0/1/2")
        node["source_parent_refs"] = ["#0-4", "#0-4.0/1"]
        with self.assertRaisesRegex(ValueError, "ambiguous source tree parent"):
            g.model(ambiguous)

    def test_real_native_probe(self):
        executable = ROOT / "target/m-tree/probe"
        if not executable.exists():
            self.skipTest("native probe exercised by scripts/test-native-m-tree.sh and Rust parity")
        result = subprocess.run([str(executable)], capture_output=True, text=True, check=True)
        verify_probe(self.manifest, result.stdout)


def verify_probe(manifest, text):
    plural = {"node": "nodes", "file": "files", "record": "records", "relation": "relations", "binding": "bindings"}
    offsets = dict.fromkeys(plural, 0)
    metadata = 0
    for line in text.splitlines():
        row = json.loads(line)
        kind = row.pop("kind")
        if kind == "registry":
            expected = {k: manifest[k] for k in ("schema", "registry_revision", "source_revision", "source_repository")}
            expected.update({k: len(manifest[k]) for k in plural.values()})
            if row != expected:
                raise AssertionError("native registry metadata mismatch")
            metadata += 1
        else:
            index = offsets[kind]
            if row != manifest[plural[kind]][index]:
                raise AssertionError(f"native {kind}[{index}] differs")
            offsets[kind] += 1
    if metadata != 1 or any(offsets[k] != len(manifest[v]) for k, v in plural.items()):
        raise AssertionError("missing or duplicate native descriptor records")


if __name__ == "__main__":
    unittest.main()
