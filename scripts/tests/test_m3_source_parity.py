"""Current M3 source fidelity against the admitted complete Bimba read.

Expectations enumerate original coordinates, UUIDs, full properties and typed
qualified edges directly from bimba-content-v1, independently of the numerical
projection. No external checkout, skipped obsolete API or live graph claim.
"""
import copy
import hashlib
import importlib.util
import json
from collections import Counter
from pathlib import Path
import re
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


m3 = module("m3_source_parity", ROOT / "scripts/m3-source-parity.py")
ledger = module("m3_shared_ledger", ROOT / "scripts/m-ledger.py")


class M3SourceParityTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.original = json.loads((ROOT / "fixtures/kernel/bimba-content-v1.json").read_text())
        cls.registry = json.loads((ROOT / m3.REGISTRY).read_text())
        cls.content = cls.original["content"]
        cls.raw_nodes = {key: value for key, value in cls.content["nodes"].items()
                         if re.fullmatch(r"M3(?:-.*)?", key)}
        cls.raw_edges = [edge for edge in cls.content["relations"] if edge[0] in cls.raw_nodes]
        cls.read = {"schema": "ql.bimba-map-read/v1",
                    "content_sha256": cls.original["source_revision"], **cls.content}
        cls.projection = m3.project(cls.registry, cls.read)
        cls.audit = m3.Audit(cls.projection).run()
        cls.lock = m3.load_json(ROOT / m3.LOCK)

    @staticmethod
    def native_ref(coordinate):
        # Native registry spelling contract, independently of the projector:
        # numeric M coordinates retain fractions/dots, dropping only brackets.
        if re.fullmatch(r"M[0-5](?:[-./()0-9]*[0-9)])?", coordinate):
            return "#" + coordinate[1:].replace("(", "").replace(")", "")
        return "bimba:" + coordinate

    def changed(self):
        return copy.deepcopy(self.projection)

    def assert_original_correspondence(self, projection):
        expected = {self.native_ref(ref): value["properties"] for ref, value in self.raw_nodes.items()}
        self.assertEqual(len(projection["nodes"]), len(expected), "required source node absent or duplicated")
        self.assertEqual({node["ref"] for node in projection["nodes"]}, set(expected))
        for node in projection["nodes"]:
            self.assertEqual(node["properties"], expected[node["ref"]], "full property/UUID payload changed")
            ident = hashlib.sha256(("ql.m-node/v1\0" + node["ref"]).encode()).hexdigest()[:16]
            self.assertEqual(node["id"], ident, "exact native coordinate identity changed")
        wanted = Counter(json.dumps([self.native_ref(a), kind, self.native_ref(b), properties],
                                   sort_keys=True, ensure_ascii=False) for a, kind, b, properties in self.raw_edges)
        observed = Counter(json.dumps([edge["from_ref"], edge["kind"], edge["to_ref"], edge["properties"]],
                                     sort_keys=True, ensure_ascii=False) for edge in projection["relations"])
        self.assertEqual(observed, wanted, "full typed edge direction/qualification/multiplicity changed")

    def test_exact_original_hash_full_field_and_m3_source_lock(self):
        self.assertEqual(self.original["source_revision"], self.registry["source_revision"])
        encoded = json.dumps(self.content, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
        self.assertEqual(hashlib.sha256(encoded).hexdigest(), self.original["source_revision"])
        self.assertEqual((len(self.content["nodes"]), len(self.content["relations"])), (2141, 11810))
        self.assertEqual((len(self.raw_nodes), len(self.raw_edges)), (996, 4952))
        self.assertEqual(m3.lock_for(self.projection, self.audit), self.lock)
        self.assertEqual(m3.project(self.registry, self.read), self.projection)
        self.assert_original_correspondence(self.projection)
        repeated = m3.Audit(self.projection)
        self.assertEqual(repeated.run(), repeated.run())

    def test_full_uuid_properties_and_qualified_relations_match_originals(self):
        self.assertTrue(all(value["properties"].get("c_2_uuid") for value in self.raw_nodes.values()))
        self.assert_original_correspondence(self.projection)
        for edge in self.projection["relations"]:
            expected = hashlib.sha256(("ql.m-relation/v1\0" + edge["ref"]).encode()).hexdigest()[:16]
            self.assertEqual(edge["id"], expected)
        refs = {node["ref"] for node in self.projection["nodes"]}
        self.assertIn("#3-5-5/0-0/360", refs)
        self.assertIn("#3-3-3-0/1-0", refs)
        self.assertIn("#3-4.0-1-0", refs)
        self.assertNotIn("#3-5-5-0-0-360", refs)
        self.assertNotIn("#3-4-0-1-0", refs)

    def test_original_cross_branch_inputs_remain_directional_and_qualified(self):
        required = {
            ("M3-0", "INHERITS_QUATERNION_FROM", "M1-5"),
            ("M3-0", "RECEIVES_VIBRATIONAL_MATRIX_FROM", "M2"),
            ("M3-0", "TRANSFORMS_72_TO_64_VIA", "M2-5"),
            ("M3", "PROVIDES_SYMBOLS_TO", "M4.2-0"),
            ("M3-1", "OPERATES_THROUGH", "M4.4.3-5-0"),
            ("M3-5", "INTEGRATES_WITH", "M4.0"),
        }
        self.assertLessEqual(required, {(a, kind, b) for a, kind, b, _ in self.raw_edges})
        self.assert_original_correspondence(self.projection)
        for a, kind, b in required:
            changed = self.changed()
            index = next(i for i, edge in enumerate(changed["relations"])
                         if (edge["from_ref"], edge["kind"], edge["to_ref"])
                         == (self.native_ref(a), kind, self.native_ref(b)))
            changed["relations"].pop(index)
            with self.subTest(subject=(a, kind, b)), self.assertRaisesRegex(AssertionError, "typed edge"):
                self.assert_original_correspondence(changed)

    def test_registry_identity_tampering_is_not_source_parity(self):
        changed = copy.deepcopy(self.registry)
        changed["nodes"][0]["id"] = "0000000000000001"
        with self.assertRaisesRegex(ValueError, "registry revision/content disagreement"):
            m3.project(changed, self.read)

    def test_missing_duplicate_or_wrong_coordinate_source_node_is_detected(self):
        for mutation in (lambda nodes: nodes.pop(), lambda nodes: nodes.append(copy.deepcopy(nodes[0])),
                         lambda nodes: nodes[0].update(ref="#4.4.4.4")):
            changed = self.changed()
            mutation(changed["nodes"])
            with self.subTest(mutation=mutation), self.assertRaises(AssertionError):
                self.assert_original_correspondence(changed)

    def test_full_property_and_uuid_removal_is_detected_with_metadata_retained(self):
        for key in ("c_2_uuid", "c_3_integral_pp"):
            changed = self.changed()
            node = next(node for node in changed["nodes"] if node["ref"] == "#3-2-1")
            node["properties"].pop(key)
            with self.subTest(key=key), self.assertRaisesRegex(AssertionError, "payload changed"):
                self.assert_original_correspondence(changed)

    def test_edge_removal_wrong_direction_and_qualification_are_detected(self):
        for mutation in (lambda edges: edges.pop(),
                         lambda edges: edges[0].update(to_ref=edges[0]["from_ref"]),
                         lambda edges: edges[0]["properties"].update(c_2_relation_type="wrong qualification")):
            changed = self.changed()
            mutation(changed["relations"])
            with self.subTest(mutation=mutation), self.assertRaisesRegex(AssertionError, "typed edge"):
                self.assert_original_correspondence(changed)

    def test_actual_map_reader_refuses_changed_original_even_with_restamped_hash(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "map.json"
            path.write_text(json.dumps(self.read, ensure_ascii=False))
            registry, read = m3.read_source(path)
            self.assertEqual(registry, self.registry)
            self.assertEqual(read, self.read)
            changed = copy.deepcopy(self.read)
            changed["nodes"]["M3-2-1"]["properties"]["c_3_integral_pp"] = -1
            path.write_text(json.dumps(changed))
            with self.assertRaisesRegex(SystemExit, "content hash"):
                m3.read_source(path)
            content = {key: changed[key] for key in ("nodes", "relations")}
            changed["content_sha256"] = hashlib.sha256(m3.canonical(content)).hexdigest()
            path.write_text(json.dumps(changed))
            with self.assertRaisesRegex(ValueError, "map the registry was built from"):
                m3.read_source(path)

    def test_hexagram_addresses_and_source_line_edges_remain_distinct_from_native_law(self):
        rows = self.audit["details"]["hexagrams"]
        self.assertEqual({r["trigram_derived_address"] for r in rows}, set(range(64)))
        # Current source has no asserted binary; preserve exact trigram-derived
        # addresses rather than inventing the removed historical field.
        self.assertTrue(all("source_binary_code" not in row for row in rows))
        self.assertEqual(self.lock["finding_counts"]["line-change"], 75)
        self.assertEqual(self.lock["finding_counts"]["nuclear-register"], 60)
        source_lines = sum(kind == "LINE_CHANGE" for _, kind, _, _ in self.raw_edges)
        self.assertEqual(source_lines, 383)  # Native XOR law has 384 transitions; source is retained.
        self.assertEqual(sum(e["kind"] == "LINE_CHANGE" for e in self.projection["relations"]), source_lines)

    def test_deleted_trigram_or_duplicate_line_relation_fails(self):
        for kind in ("HAS_UPPER_TRIGRAM", "LINE_CHANGE"):
            changed = self.changed()
            index = next(i for i, e in enumerate(changed["relations"]) if e["kind"] == kind)
            changed["relations"].pop(index)
            with self.subTest(kind=kind), self.assertRaisesRegex(AssertionError, "typed edge"):
                self.assert_original_correspondence(changed)

    def test_matrix_field_retains_original_qualified_cardinality_and_distinct_admissibility(self):
        rows = self.audit["details"]["matrices"]
        self.assertEqual([sum(r["family"] == f for r in rows) for f in range(3)], [64, 64, 56])
        self.assertNotIn("matrix-unresolved-endpoint", self.lock["finding_counts"])
        self.assertNotIn("matrix-missing-pair-role", self.lock["finding_counts"])
        self.assertEqual(self.lock["finding_counts"]["resonance-admissibility"], 1)
        self.assertEqual(sorted(set(range(64)) - {r["address"] for r in rows if r["family"] == 2}),
                         [6, 14, 22, 30, 38, 46, 54, 62])
        cell = next(r for r in rows if r["ref"] == "#3-3-2-2-24")
        self.assertEqual(len(cell["pair_relations"]), 1)
        source_yields = sum(kind == "YIELDS_CODON" and a.startswith("M3-3-2-")
                            for a, kind, _, _ in self.raw_edges)
        self.assertEqual(sum(len(r["codon_relations"]) for r in rows), source_yields)

    def test_transcription_is_37_exact_t_to_u_with_27_shared_forms(self):
        rows = self.audit["details"]["genetics"]["rna"]
        self.assertEqual(len(rows), 37)
        self.assertEqual(len({r["dna"] for r in rows}), 37)
        for row in rows:
            self.assertEqual(row["rna"], row["dna"].replace("T", "U"))
        auc = next(r for r in rows if r["dna"] == "ATC")
        self.assertEqual(auc["rna_ref"], "#3-3-3-2-3")
        self.assertEqual(64 + len(rows), 101)
        self.assertEqual(64 - len(rows), 27)

    def test_changed_transcription_endpoint_fails_semantic_check(self):
        changed = self.changed()
        edge = next(e for e in changed["relations"] if e["kind"] == "TRANSCRIBES_TO" and e["from_ref"] == "#3-2-1-2-3")
        edge["to_ref"] = "#3-3-3-0/1-0"
        with self.assertRaisesRegex(ValueError, "not exact T-to-U"):
            m3.Audit(changed).run()

    def test_source_473_is_not_falsely_reported_as_native_472(self):
        rows = self.audit["details"]["genetics"]["phase"]
        self.assertEqual(sum(r["state_count"] for r in rows), 473)
        self.assertEqual(sum(r["native_state_count"] for r in rows), 472)
        mismatch = [r for r in rows if r["state_count"] != r["native_state_count"]]
        self.assertEqual(len(mismatch), 1)
        self.assertEqual((mismatch[0]["ref"], mismatch[0]["sequence"]), ("#3-4.0-2-8", "TCT"))
        self.assertEqual(self.lock["finding_counts"]["orientation-count"], 1)

    def test_translation_is_multi_valued_and_conditional_not_an_index_alias(self):
        rows = self.audit["details"]["genetics"]["phase"]
        edges = {e["id"]: e for e in self.projection["relations"]}
        atg = next(r for r in rows if r["sequence"] == "ATG")
        self.assertEqual({edges[e]["to_ref"] for e in atg["translation_relations"]}, {"#3-3-4-5", "#3-3-4-22"})
        self.assertEqual(sum(len(r["translation_relations"]) for r in rows), 65)
        self.assertEqual(sum(len(r["conditional_translation_relations"]) for r in rows), 2)
        self.assertEqual(len({r["tarot_ref"] for r in rows}), 56)
        self.assertEqual(sum(len(r["court_relations"]) for r in rows), 16)
        associations = self.audit["details"]["genetics"]["major_associations"]
        fool = next(r for r in associations if r["ref"] == "#3-4-5/0-0")
        self.assertEqual(len(fool["relations"]["PROVIDES_VESSEL_FOR"]), 3)
        source_cascade = sum(kind == "ARCHETYPAL_CASCADE" and self.native_ref(a) == fool["ref"]
                             for a, kind, _, _ in self.raw_edges)
        self.assertEqual(len(fool["relations"]["ARCHETYPAL_CASCADE"]), source_cascade)

    def test_native_pair_reference_matches_actual_retained_c_table(self):
        text = (ROOT / "vendor/epi-kernel/reference/src/m3.c").read_text()
        body = text.split("const M3_SD_Value M3_PAIR_MATRIX[16] = {", 1)[1].split("};", 1)[0]
        table = {int(i): [int(s), int(d)] for i, s, d in re.findall(r"\[(\d+)\]\s*=\s*\{\s*(-?\d+)\s*,\s*(-?\d+)\s*\}", body)}
        self.assertEqual(len(table), 16)
        for row in self.audit["details"]["genetics"]["pairs"]:
            sequence = row["sequence"]
            index = "ATCG".index(sequence[0])*4 + "ATCG".index(sequence[1])
            self.assertEqual(row["native"], table[index])
        self.assertNotIn("pair-descriptor", self.lock["finding_counts"])
        self.assertNotIn("codon-charge", self.lock["finding_counts"])

    def test_full_clock_backbone_inverse_flow_and_opposition(self):
        rows = self.audit["details"]["clock"]
        self.assertEqual(len(rows), 360)
        self.assertEqual(rows[0]["ref"], "#3-5-5/0-0/360")
        self.assertEqual(len({r["backbone_id"] for r in rows}), 24)
        self.assertFalse({r["id"] for r in rows} & {r["backbone_id"] for r in rows})
        for degree, row in enumerate(rows):
            self.assertEqual(row["clockwise_ref"], rows[(degree+1) % 360]["ref"])
            self.assertEqual(row["opposite_ref"], rows[(degree+180) % 360]["ref"])
            self.assertEqual(sum(r["backbone_id"] == row["backbone_id"] for r in rows), 15)
        self.assertEqual(self.lock["finding_counts"]["legacy-clock-placeholder"], 1)

    def test_clock_governor_deletion_and_retargeting_fail(self):
        for kind in ("GOVERNS_DEGREE_ARC", "ANCHORED_BY", "FLOWS_CLOCKWISE", "POLAR_OPPOSITE"):
            changed = self.changed()
            i = next(i for i, e in enumerate(changed["relations"]) if e["kind"] == kind)
            changed["relations"].pop(i)
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                m3.Audit(changed).run()
        changed = self.changed()
        edge = next(e for e in changed["relations"] if e["kind"] == "FLOWS_CLOCKWISE")
        edge["to_ref"] = edge["from_ref"]
        with self.assertRaisesRegex(ValueError, "clock flow drift"):
            m3.Audit(changed).run()

    def test_all_findings_fit_existing_ledger_without_readiness_promotion(self):
        original = ledger.read(ROOT / ledger.LEDGER)
        updated = copy.deepcopy(original)
        # K7 has now admitted these findings. Re-running the source audit must
        # retain a reviewed lifecycle, not append duplicate IDs or overwrite it.
        existing = {d["id"]: d for d in updated["discrepancies"]}
        for finding in self.audit["discrepancies"]:
            if finding["id"] in existing:
                for key in ("axis", "from_peer", "to_peer", "subjects", "detail"):
                    self.assertEqual(existing[finding["id"]][key], finding[key])
            else:
                updated["discrepancies"].append(copy.deepcopy(finding))
        updated["ledger_revision"] = ledger.digest(ledger.canonical({k: v for k, v in updated.items() if k != "ledger_revision"}))
        ledger.verify(ROOT, updated)
        row_ids = {r["id"] for r in updated["rows"]}
        finding_ids = [d["id"] for d in updated["discrepancies"]]
        self.assertEqual(len(finding_ids), len(set(finding_ids)))
        for finding in self.audit["discrepancies"]:
            self.assertLessEqual(set(finding["subjects"]), row_ids)
            self.assertEqual(finding["state"], "open")
            self.assertIsNone(finding["promotion"])
            self.assertIsNone(finding["decision"])
            self.assertNotEqual(finding["from_peer"], finding["to_peer"])
        for field in ("rows", "assessments", "evidence", "implementations"):
            self.assertEqual(updated[field], original[field])
        self.assertEqual(ledger.read(ROOT / ledger.LEDGER), original, "canonical ledger mutated by the audit")

    def test_no_execution_acceptance_or_live_graph_claim_can_hide_in_source_success(self):
        self.assertEqual(self.audit["standing"]["c_rust_execution"], "not-claimed-by-this-source-audit")
        self.assertEqual(self.audit["standing"]["K4_acceptance"], "not-claimed")
        self.assertEqual(self.audit["standing"]["neo4j_live"], "read-only map read; content hash = registry source_revision")
        self.assertEqual(self.audit["standing"]["cpp_embodiment"], "not-executed")
        self.assertEqual(self.audit["standing"]["experiential"], "not-claimed")
        self.assertEqual(len(self.audit["discrepancies"]), 183)


if __name__ == "__main__":
    unittest.main()
