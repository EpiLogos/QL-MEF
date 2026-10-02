#!/usr/bin/env python3
"""Actual Bimba READ/native-registry compiler tests (no network or build)."""
import argparse
import copy
import importlib.util
import json
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("m2_relation_source", Path(__file__).with_name("m2-relation-source.py"))
compiler = importlib.util.module_from_spec(spec)
spec.loader.exec_module(compiler)
READ = REGISTRY = PROJECTION = None

class RelationSource(unittest.TestCase):
    def test_exact_projection_replays_from_complete_read(self):
        field = compiler.compile_field(READ, REGISTRY)
        self.assertEqual(compiler.render(field), PROJECTION.read_text())
        self.assertEqual(len(field["nodes"]), 598)
        self.assertEqual(len(field["relations"]), 4490)
        self.assertEqual([n["map_coordinate"] for n in field["prime_source_nodes"]], ["M2'"])

    def test_all_native_leaf_properties_and_typed_edge_payloads_are_preserved(self):
        field = compiler.compile_field(READ, REGISTRY)
        for node in field["nodes"]:
            self.assertEqual(json.loads(node["canonical_properties"]), READ["nodes"][node["map_coordinate"]]["properties"])
        native_edges = {r["relation_ref"]: r for r in REGISTRY["relations"]}
        for edge in field["relations"]:
            native = native_edges[edge["relation_ref"]]
            self.assertEqual(edge["id"], native["id"])
            self.assertEqual(edge["kind"], native["source_kind"])
            self.assertEqual(edge["from_coordinate"], native["from_ref"])
            self.assertEqual(edge["to_coordinate"], native["to_ref"])
            self.assertEqual(edge["payload_sha256"], REGISTRY["records"][native["record"]]["payload_sha256"])

    def test_rast_spelling_and_ficino_planet_sources_are_not_replaced_by_tuning(self):
        field = compiler.compile_field(READ, REGISTRY)
        nodes = {n["coordinate"]: json.loads(n["canonical_properties"]) for n in field["nodes"]}
        self.assertEqual(nodes["#2-4.3-1-0"]["c_2_tonic_note"], "C")
        self.assertEqual(nodes["#2-4.3-1-0"]["c_2_dominant_note"], "G")
        self.assertEqual(nodes["#2-4.3-1-0"]["c_2_ajnas"], "Primary: Rast pentachord on C, Secondary: Rast tetrachord on G")
        self.assertEqual(nodes["#2-5-5"]["m_2_5_interval_from_root"], "Perfect Fifth (3:2)")
        self.assertIn("Ficino", nodes["#2-5-5"]["c_2_medieval_sublimation"])
        self.assertNotIn("spelled_steps24", nodes["#2-4.3-1-0"])

    def test_unqualified_read_mutation_is_refused(self):
        changed = copy.deepcopy(READ)
        changed["nodes"]["M2-4.3-1-0"]["properties"]["c_2_tonic_note"] = "D"
        with self.assertRaisesRegex(ValueError, "content hash mismatch"):
            compiler.compile_field(changed, REGISTRY)
        changed["content_sha256"] = compiler.sha({"nodes": changed["nodes"], "relations": changed["relations"]})
        with self.assertRaisesRegex(ValueError, "not the registry source"):
            compiler.compile_field(changed, REGISTRY)

    def test_lost_edge_and_wrong_registered_payload_are_refused(self):
        changed = copy.deepcopy(READ)
        changed["relations"] = [r for r in changed["relations"] if not (r[0] == "M2-4.3-1-0" and r[1] == "TONIC_PLANETARY_RESONANCE")]
        with self.assertRaisesRegex(ValueError, "content hash mismatch"):
            compiler.compile_field(changed, REGISTRY)
        registry = copy.deepcopy(REGISTRY)
        native = next(n for n in registry["nodes"] if n["source_ref"] == "#2-4.3-1-0")
        registry["records"][native["records"][0]]["payload_sha256"] = "0" * 64
        with self.assertRaisesRegex(ValueError, "node source record mismatch"):
            compiler.compile_field(READ, registry)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--map", type=Path, required=True)
    parser.add_argument("--registry", type=Path, required=True)
    parser.add_argument("--projection", type=Path, required=True)
    args = parser.parse_args()
    READ = json.loads(args.map.read_text())
    REGISTRY = json.loads(args.registry.read_text())
    PROJECTION = args.projection
    unittest.main(argv=["test_m2_relation_source"], verbosity=2)

