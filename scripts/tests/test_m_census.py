"""K4 census regressions: coordinate identity, anchor joins, classification."""
import importlib.util
import unittest
import copy
import json
import contextlib
import io
from unittest import mock

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("m_census_tool", ROOT / "scripts/m_census.py")
census = importlib.util.module_from_spec(spec)
spec.loader.exec_module(census)


class CoordinateIdentityTests(unittest.TestCase):
    def test_numeric_path_is_separator_blind_but_order_sensitive(self):
        self.assertEqual(census.numeric_path("#0-1-0/1"), ("0", (0, 1, 0, 1)))
        self.assertEqual(census.numeric_path("M0-1-(0/1)"), ("0", (0, 1, 0, 1)))
        self.assertEqual(census.numeric_path("M3-2-1-1-1"), ("3", (3, 2, 1, 1, 1)))
        self.assertNotEqual(census.numeric_path("#0-1-0/1"), census.numeric_path("#0-1-1/0"))
        self.assertIsNone(census.numeric_path("#"))
        self.assertIsNone(census.numeric_path("S1"))

    def test_prime_spellings_carry_no_numeric_path_identity(self):
        # M1' is the Paramasiva deep-instrument coordinate, not the #1 root;
        # it must stay a live-only delta, never join the M tree.
        self.assertIsNone(census.numeric_path("M1'"))
        self.assertFalse(census.in_census_scope("M1'"))

    def test_normalize_spelling_applies_only_the_sanctioned_root_alias(self):
        self.assertEqual(census.normalize_spelling("M1-2-0"), "#1-2-0")
        self.assertEqual(census.normalize_spelling("#1-2-0"), "#1-2-0")
        self.assertEqual(census.normalize_spelling("M2-3-(5/0)"), "#2-3-5/0")
        self.assertIsNone(census.normalize_spelling("M1-"))
        self.assertIsNone(census.normalize_spelling("CF_BINARY"))

    def test_spelling_key_is_word_safe_for_symbol_presence_checks(self):
        self.assertEqual(census.spelling_key("M2-3-(5/0)"), "M2-3-5/0")
        self.assertEqual(census.spelling_key("M1-2"), "M1-2")

    def test_census_scope_covers_M1_M2_M3_only(self):
        self.assertTrue(census.in_census_scope("#1-2-0"))
        self.assertTrue(census.in_census_scope("M3-5"))
        self.assertFalse(census.in_census_scope("#0"))
        self.assertFalse(census.in_census_scope("#4-1"))
        self.assertFalse(census.in_census_scope("M1'"))


class TokenTests(unittest.TestCase):
    def test_incidental_name_words_cannot_anchor_code(self):
        # "arena" appears in "#2-4 Vibrational Arena of Archetypal Powers"
        # but memory-arena machinery must not bind through it.
        self.assertNotIn("arena", census.symbol_tokens("arena_init"))
        self.assertNotIn("arena", census.content_tokens("Vibrational Arena of Archetypal Powers"))
        self.assertEqual(census.symbol_tokens("arena_init"), set())

    def test_symbol_tokens_stem_and_split_identifiers(self):
        self.assertEqual(census.symbol_tokens("ANANDA_BIMBA"), {"ananda", "bimba"})
        self.assertEqual(census.symbol_tokens("m3_print_codon"), {"codon"})
        self.assertIn("tattva", census.symbol_tokens("m2_print_tattvas"))
        self.assertIn("quintessence", census.symbol_tokens("ANANDA_QUINTESSENCE"))

    def test_content_tokens_stem_names_and_labels(self):
        self.assertEqual(census.content_tokens("Matrix 0: The Original (Bimba)"), {"original", "bimba"})
        self.assertIn("tattva", census.content_tokens("Shiva Tattva"))
        self.assertIn("codon", census.content_tokens("Codon_AAA"))


class FakeField(census.BimbaField):
    """A minimal field with hand-set anchors, bypassing registry/living data."""

    def __init__(self):
        self.self_tokens = {
            "#1-2": frozenset({"ananda"}),
            "#1-2-0": frozenset({"original", "bimba"}),
            "#1-2-1": frozenset({"pratibimba"}),
            "#1-3": frozenset({"spanda"}),
        }
        self.ancestor_tokens = {
            "#1-2": frozenset({"ananda"}),
            "#1-2-0": frozenset({"ananda", "original", "bimba"}),
            "#1-2-1": frozenset({"ananda", "pratibimba"}),
            "#1-3": frozenset({"spanda"}),
        }
        self.content = {}

    def parent_of(self, ref):
        return {"#1-2-0": "#1-2", "#1-2-1": "#1-2"}.get(ref)

    def is_ancestor(self, ancestor, descendant):
        return (ancestor, descendant) in {("#1-2", "#1-2-0"), ("#1-2", "#1-2-1")}

    def summary(self, ref):
        return {"names": [], "labels": [], "architectural_function": "", "live_spelling": None}


def bind_one(symbol, field):
    bound, orphans = census.bind_constructs(
        [{"symbol": symbol, "path": "x.c", "kind": "function", "line": 1, "context": ""}],
        field, "c")
    return bound, orphans


class BindingRuleTests(unittest.TestCase):
    def setUp(self):
        self.field = FakeField()

    def test_self_and_ancestor_names_bind_the_deepest_coordinate(self):
        bound, orphans = bind_one("ANANDA_BIMBA", self.field)
        self.assertEqual(len(bound), 1)
        self.assertEqual(bound[0]["coordinates"], ["#1-2-0"])
        self.assertEqual(bound[0]["trail"][0]["how"], "bimba-name-anchor")

    def test_ancestor_only_tokens_bind_the_named_aggregate(self):
        bound, _ = bind_one("ananda_matrix", self.field)
        self.assertEqual(bound[0]["coordinates"], ["#1-2"])

    def test_partially_explained_tokens_are_orphaned_not_forced(self):
        bound, orphans = bind_one("ananda_gadget", self.field)
        self.assertEqual(bound, [])
        self.assertEqual(len(orphans), 1)
        self.assertIn("anchor", orphans[0]["reason"])

    def test_spelling_evidence_binds_exactly(self):
        construct = {"symbol": "handle", "path": "x.c", "kind": "function", "line": 1,
                     "context": "implements #1-2-1 reflection"}
        with mock.patch.object(census, "spelling_hits", return_value={"#1-2-1"}):
            bound, _ = census.bind_constructs([construct], self.field, "c")
        self.assertEqual(bound[0]["coordinates"], ["#1-2-1"])
        self.assertEqual(bound[0]["trail"][0]["how"], "coordinate-spelling")


class ClassificationTests(unittest.TestCase):
    def test_implemented_partial_and_unimplemented_states(self):
        field = FakeField()
        bound = [
            {"symbol": "leaf_a", "path": "x.c", "kind": "function", "line": 1, "context": "",
             "coordinates": ["#1-2-0"], "trail": []},
        ]
        result = census.classify_coordinates(field, bound)
        self.assertEqual(result["#1-2-0"]["state"], "implemented")
        self.assertEqual(result["#1-2"]["state"], "partial")
        self.assertIn("#1-2-0", result["#1-2"]["covered_by_descendants"])
        self.assertEqual(result["#1-3"]["state"], "unimplemented")
        # The aggregate record must carry the parent coordinate so the
        # partial row can hold an intersecting binding.
        self.assertIn("#1-2", bound[0]["extended_coordinates"])

    def test_dispositions_follow_states(self):
        self.assertEqual(census.disposition_for("implemented"), "bound")
        self.assertEqual(census.disposition_for("partial"), "bound")
        self.assertEqual(census.disposition_for("unimplemented"), "unimplemented")


class AssessmentProfileTests(unittest.TestCase):
    def test_profile_names_are_deterministic_and_complete(self):
        name, profile = census.assessment_profile("implemented", "unimplemented", True)
        self.assertEqual(name, "k4:c=implemented,rust=unimplemented,neo4j=structural-index-only")
        self.assertEqual(set(profile["readiness"]),
                         {"source", "c", "rust", "cpp", "neo4j", "application", "instrument"})
        self.assertEqual(profile["readiness"]["c"]["evidence"], ["k4-c-discovery"])
        self.assertEqual(profile["readiness"]["neo4j"]["warrant"], "observed")
        self.assertEqual(set(profile["parity"]), {"source", "coordinate", "relation",
                                                  "operational", "experiential"})

    def test_unimplemented_claims_carry_no_evidence(self):
        _, profile = census.assessment_profile("unimplemented", "unimplemented", False)
        self.assertEqual(profile["readiness"]["c"]["status"], "unimplemented")
        self.assertEqual(profile["readiness"]["c"]["evidence"], [])
        self.assertEqual(profile["readiness"]["neo4j"]["status"], "unassessed")


class CompositeDivergenceTests(unittest.TestCase):
    def test_serialized_and_live_spellings_pair_by_longest_prefix(self):
        live_join = {
            "live_only": ["M0-4.(4.0/1-4.4/5)", "M0-4.(4.0/1-4.4/5)-0", "M1-3-4.(4.0/1-4.4/5)"],
            "serialized_only": ["#0-4.4.0-4.4/5", "#0-4.4.0-4.4/5-0", "#1-3-4.4.0-4.4/5"],
        }
        groups = census.composite_divergence_groups(live_join)
        self.assertEqual(len(groups), 2)
        ids = {g["discrepancy_id"] for g in groups}
        self.assertEqual(len(ids), 2, "group ids must not collide")
        m0 = next(g for g in groups if "#0-4.4.0-4.4/5" in g["registry_coordinates"])
        self.assertEqual(m0["registry_coordinates"], ["#0-4.4.0-4.4/5", "#0-4.4.0-4.4/5-0"])
        self.assertIn("M0-4.(4.0/1-4.4/5)", m0["live_spellings"])


class InfrastructuralTests(unittest.TestCase):
    def test_arena_machinery_is_infrastructural_not_coordinate_bound(self):
        self.assertIn("vendor/epi-kernel/reference/src/arena.c", census.INFRASTRUCTURAL_C_FILES)



class ReviewedVerticalTests(unittest.TestCase):
    def test_census_replay_preserves_reviewed_row_and_matrix_assessments(self):
        original = json.loads((ROOT / census.LEDGER).read_text())
        captured = {}
        reviewed_ids = ["census:#1-2-0", "deep-M1:M1-C04", "census:#2-1", "deep-M2:M2-C02"]
        for row in original["rows"]:
            if row["id"] in reviewed_ids:
                row["assessment"] = "k6-test:executed" if "#2" in row["id"] or "deep-M2:" in row["id"] else "k5-test:executed"
                row["bindings"] = ["native-reviewed-binding"]
                row["dependencies"] = ["k5-test:operation"]
                row["invariants"] = ["reviewed-source-register-decision"]
        before = {row["id"]: copy.deepcopy(row) for row in original["rows"] if row["id"] in reviewed_ids}
        real_read = census.read_json
        def read(path):
            return copy.deepcopy(original) if Path(path) == ROOT/census.LEDGER else real_read(path)
        def write(path, value):
            captured[str(path)] = copy.deepcopy(value)
        with mock.patch.object(census, "read_json", side_effect=read), mock.patch.object(census, "write_json", side_effect=write), contextlib.redirect_stdout(io.StringIO()):
            census.build_census(None)
        after = {row["id"]: row for row in captured[str(ROOT/census.LEDGER)]["rows"]}
        for key in reviewed_ids:
            self.assertEqual(before[key], after[key])
        self.assertIn(str(ROOT/census.CENSUS_DIR/"census-m1.json"), captured)
        self.assertFalse(census.reviewed_row({"assessment":"unassessed"}))
        self.assertFalse(census.reviewed_row({"assessment":"k4:c=implemented"}))
        self.assertTrue(census.reviewed_row({"assessment":"k5:partial"}))

class VerticalBindingTests(unittest.TestCase):
    def test_namespaced_binding_stratum_comes_from_inventory_not_id_prefix(self):
        implementations = {
            "k6-m2:mantra:c": {"stratum": "c"},
            "k7-m3:clock:rust": {"stratum": "rust"},
            "c:misleading-prefix": {"stratum": "rust"},
        }
        bindings = [*implementations, "c:unknown"]
        self.assertEqual(census.bindings_for_stratum(bindings, implementations, "c"),
                         {"k6-m2:mantra:c"})
        self.assertEqual(census.bindings_for_stratum(bindings, implementations, "rust"),
                         {"k7-m3:clock:rust", "c:misleading-prefix"})
        self.assertEqual(census.bindings_for_stratum(bindings, implementations, "cpp"), set())


if __name__ == "__main__":
    unittest.main()
