"""K7 locked-source and semantic mutation regressions; no live graph claims.

M3_SOURCE_ROOT is a read-only checkout of the K2-pinned source repository. The
M3 workflow always supplies it; generic offline discovery reports an explicit
skip rather than inventing that external source or a passing source observation.
"""
import copy
import hashlib
import importlib.util
import json
import os
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
        source = os.environ.get("M3_SOURCE_ROOT")
        if not source:
            raise unittest.SkipTest("M3_SOURCE_ROOT not supplied; source observation not performed")
        cls.source = Path(source)
        cls.registry, cls.raw_nodes, cls.raw_edges = m3.read_source(cls.source)
        cls.projection = m3.project(cls.registry, cls.raw_nodes, cls.raw_edges)
        cls.audit = m3.Audit(cls.projection).run()
        cls.lock = m3.load_json(ROOT / m3.LOCK)

    def changed(self):
        return copy.deepcopy(self.projection)

    def test_exact_source_lock_and_deterministic_full_projection(self):
        self.assertEqual(m3.lock_for(self.projection, self.audit), self.lock)
        self.assertEqual(m3.project(self.registry, self.raw_nodes, self.raw_edges), self.projection)
        repeated = m3.Audit(self.projection)
        self.assertEqual(repeated.run(), repeated.run())
        self.assertEqual(len(self.projection["nodes"]), 996)
        self.assertEqual(len(self.projection["relations"]), 4891)

    def test_lossless_node_and_qualified_duplicate_relation_roundtrip(self):
        nodes = [{"coordinate": n["ref"], "filteredProps": n["properties"]} for n in self.projection["nodes"]]
        edges = [{"source": e["from_ref"], "target": e["to_ref"], "relType": e["kind"],
                  "relProperties": e["properties"]} for e in self.projection["relations"]]
        self.assertEqual(nodes, self.raw_nodes)
        self.assertEqual(edges, self.raw_edges)
        self.assertTrue(any(e["target"] is None for e in edges))
        signatures = [m3.digest(e) for e in edges]
        self.assertLess(len(set(signatures)), len(signatures), "source duplicate edges were lost")

    def test_exact_shared_id_scheme_and_compound_coordinates(self):
        for node in self.projection["nodes"]:
            expected = hashlib.sha256(("ql.m-node/v1\0" + node["ref"]).encode()).hexdigest()[:16]
            self.assertEqual(node["id"], expected)
        for edge in self.projection["relations"]:
            expected = hashlib.sha256(("ql.m-relation/v1\0" + edge["ref"]).encode()).hexdigest()[:16]
            self.assertEqual(edge["id"], expected)
        refs = {n["ref"] for n in self.projection["nodes"]}
        self.assertIn("#3-5-5/0-0/360", refs)
        self.assertIn("#3-3-3-0/1-0", refs)
        self.assertIn("#3-4.0-1-0", refs)
        self.assertNotIn("#3-5-5-0-0-360", refs)
        self.assertNotIn("#3-4-0-1-0", refs)

    def test_registry_identity_tampering_is_not_source_parity(self):
        changed = copy.deepcopy(self.registry)
        changed["nodes"][0]["id"] = "0000000000000001"
        with self.assertRaisesRegex(ValueError, "registry revision/content disagreement"):
            m3.project(changed, self.raw_nodes, self.raw_edges)

    def test_missing_or_duplicate_node_cannot_pass_a_source_join(self):
        for changed in (self.raw_nodes[:-1], self.raw_nodes + [self.raw_nodes[0]]):
            with self.subTest(count=len(changed)), self.assertRaises(ValueError):
                m3.project(self.registry, changed, self.raw_edges)

    def test_mutated_source_property_and_unclassified_wrapper_fail(self):
        changed = copy.deepcopy(self.raw_nodes)
        changed[0]["filteredProps"]["name"] = "unreviewed replacement"
        with self.assertRaisesRegex(ValueError, "node payload drift"):
            m3.project(self.registry, changed, self.raw_edges)
        changed = copy.deepcopy(self.raw_nodes)
        changed[0]["new_metadata"] = "must not silently disappear"
        with self.assertRaisesRegex(ValueError, "unclassified source node wrapper"):
            m3.project(self.registry, changed, self.raw_edges)

    def test_missing_retargeted_or_mutated_relation_fails(self):
        with self.assertRaisesRegex(ValueError, "relation count drift"):
            m3.project(self.registry, self.raw_nodes, self.raw_edges[:-1])
        for field, value in (("target", "#3"), ("relProperties", {"forged": True})):
            changed = copy.deepcopy(self.raw_edges)
            changed[0][field] = value
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, "relation .*drift"):
                m3.project(self.registry, self.raw_nodes, changed)

    def test_file_bytes_are_locked_not_just_selected_fields(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            relative = m3.DATA + "nodes-full-detail.json"
            path = root / relative
            path.parent.mkdir(parents=True)
            raw = (self.source / relative).read_bytes()
            path.write_bytes(raw.replace(b"Mahamaya", b"Mahamayb", 1))
            with self.assertRaisesRegex(ValueError, "source SHA-256 drift"):
                m3.read_source(root)

    def test_hexagram_composition_keeps_both_source_and_derived_codes(self):
        rows = self.audit["details"]["hexagrams"]
        self.assertEqual({r["trigram_derived_address"] for r in rows}, set(range(64)))
        self.assertEqual(len({r["source_binary_code"] for r in rows}), 61)
        wrong = {r["ref"] for r in rows if r["source_binary_code"] != r["trigram_derived_address"]}
        self.assertEqual(wrong, {"#3-1-5-7", "#3-1-6-5", "#3-1-7-2"})
        self.assertEqual(self.lock["finding_counts"]["line-change"], 258)
        self.assertEqual(self.lock["finding_counts"]["nuclear-register"], 60)
        self.assertEqual(sum(e["kind"] == "LINE_CHANGE" for e in self.projection["relations"]), 384)

    def test_deleted_trigram_or_duplicate_line_relation_fails(self):
        for kind in ("HAS_UPPER_Trigram", "LINE_CHANGE"):
            changed = self.changed()
            index = next(i for i, e in enumerate(changed["relations"]) if e["kind"] == kind)
            changed["relations"].pop(index)
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                m3.Audit(changed).run()

    def test_matrix_field_retains_nulls_missing_roles_and_distinct_admissibility(self):
        rows = self.audit["details"]["matrices"]
        self.assertEqual([sum(r["family"] == f for r in rows) for f in range(3)], [64, 64, 56])
        self.assertEqual(self.lock["finding_counts"]["matrix-unresolved-endpoint"], 114)
        self.assertEqual(self.lock["finding_counts"]["matrix-missing-pair-role"], 1)
        self.assertEqual(self.lock["finding_counts"]["resonance-admissibility"], 1)
        self.assertEqual(sorted(set(range(64)) - {r["address"] for r in rows if r["family"] == 2}),
                         [6, 14, 22, 30, 38, 46, 54, 62])
        cell = next(r for r in rows if r["ref"] == "#3-3-2-2-24")
        self.assertEqual(len(cell["pair_relations"]), 1)
        self.assertEqual(sum(len(r["codon_relations"]) for r in rows), 368)

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
        self.assertEqual(len(fool["relations"]["PROVIDES_VESSEL_FOR"]), 4)
        self.assertEqual(len(fool["relations"]["ARCHETYPAL_CASCADE"]), 6)

    def test_native_pair_reference_matches_actual_retained_c_table(self):
        text = (ROOT / "vendor/epi-kernel/reference/src/m3.c").read_text()
        body = text.split("const M3_SD_Value M3_PAIR_MATRIX[16] = {", 1)[1].split("};", 1)[0]
        table = {int(i): [int(s), int(d)] for i, s, d in re.findall(r"\[(\d+)\]\s*=\s*\{\s*(-?\d+)\s*,\s*(-?\d+)\s*\}", body)}
        self.assertEqual(len(table), 16)
        for row in self.audit["details"]["genetics"]["pairs"]:
            sequence = row["sequence"]
            index = "ATCG".index(sequence[0])*4 + "ATCG".index(sequence[1])
            self.assertEqual(row["native"], table[index])
        self.assertEqual(self.lock["finding_counts"]["pair-descriptor"], 12)
        self.assertEqual(self.lock["finding_counts"]["codon-charge"], 112)

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
        self.assertEqual(self.audit["standing"]["neo4j_live"], "not-observed")
        self.assertEqual(self.audit["standing"]["cpp_embodiment"], "not-executed")
        self.assertEqual(self.audit["standing"]["experiential"], "not-claimed")
        self.assertEqual(len(self.audit["discrepancies"]), 563)


if __name__ == "__main__":
    if not os.environ.get("M3_SOURCE_ROOT"):
        raise SystemExit("M3_SOURCE_ROOT is required for the explicit source-parity test run")
    unittest.main()
