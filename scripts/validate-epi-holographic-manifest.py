#!/usr/bin/env python3
"""Validate that the native Epi/QL C floor is not structurally orphaned.

This validator deliberately does not invent unresolved Bimba semantics. It checks
that every public native ql_* export has exactly one explicit manifestation
account, that the frozen corpus remains exhaustively classified, that the locked
product/archetype orientation has not been silently flattened, that the accepted
PRE-D Map receipt remains explicitly M-family scoped rather than being promoted
to whole-coordinate-system parity, and that returned cross-repository evidence
is represented at its actual acceptance standing.
"""

from __future__ import annotations

import json
import pathlib
import re
import sys
from collections import Counter

ROOT = pathlib.Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/integrations/epi-logos/EPI-HOLOGRAPHIC-KERNEL-MANIFEST.json"
HEADER = ROOT / "c/include/ql/primitive.h"

EXPECTED_S_ROOTS = [
    ("S0", "Central"),
    ("S1", "Actuation"),
    ("S2", "AIKit"),
    ("S3", "Software Factory"),
    ("S4", "Workcell"),
    ("S5", "Quaternal Logic"),
]

EXPECTED_ARCHETYPES = {"Khora", "Hen", "Pleroma", "Chronos", "Anima", "Aletheia"}
EXPECTED_FIRST_BIMBA_COORDINATE = "#1-4.2"
EXPECTED_FIRST_BIMBA_PARENT = "#1-4"
EXPECTED_FIRST_BIMBA_PREDECESSOR = "#1-4.1"
EXPECTED_PRE_D_MAIN = "d418abfff6f9e001c8c5ff083206329b298eddcf"
EXPECTED_AIKIT_MAIN = "5308405e447b4a48e57fa2cfb2c5e6ef276ae343"
EXPECTED_AIKIT_HEAD = "b0516fd566173af0e8be81cc9ae46f694df6c49c"
EXPECTED_FACTORY_MAIN = "06579aada01a77bd719c0c010a10f91084b4326f"
EXPECTED_FACTORY_HEAD = "b93c59b4209505468f73d183f2b265f1b765b2e6"
EXPECTED_FACTORY_GENERIC_MERGE = "dde3ddc7c666f740c022ab347100369563cce90b"
EXPECTED_FAMILY_MANIFESTATIONS = {
    ("C", "FAMILY_C", "Category"),
    ("P", "FAMILY_P", "Position"),
    ("L", "FAMILY_L", "Lens"),
    ("S", "FAMILY_S", "Stack"),
    ("T", "FAMILY_T", "Thought"),
    ("M", "FAMILY_M", "Map (Bimba)"),
}

EXPECTED_REFERENCE_TUS = {
    "src/arena.c",
    "src/engine.c",
    "src/families.c",
    "src/kernel.c",
    "src/m0.c",
    "src/m1.c",
    "src/m2.c",
    "src/m3.c",
    "src/m3_clock_lut.c",
    "src/m4.c",
    "src/m5.c",
    "src/main.c",
    "src/pointer_web.c",
    "src/psychoid_numbers.c",
    "src/qv_data.c",
}


def fail(message: str) -> None:
    raise AssertionError(message)


def public_ql_symbols(header: str) -> set[str]:
    # The primitive header's public API consists of ordinary function
    # prototypes. Match names rather than return types so structs/enums do not
    # become accidental manifest subjects.
    return set(re.findall(r"\b(ql_[A-Za-z0-9_]+)\s*\([^;{}]*\)\s*;", header, flags=re.MULTILINE))


def require_nonempty(mapping: dict, key: str, symbol: str) -> None:
    if key not in mapping:
        fail(f"{symbol}: missing required field {key!r}")
    value = mapping[key]
    if value is None or value == "" or value == [] or value == {}:
        fail(f"{symbol}: required field {key!r} is empty")


def main() -> int:
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    header = HEADER.read_text(encoding="utf-8")

    if data.get("schema") != "ql-mef.epi-holographic-kernel-manifest/v1":
        fail("unexpected manifest schema/version")

    map_state = data.get("sources", {}).get("epi", {}).get("bimba_map_returned_state", {})
    if map_state.get("status") != "accepted-current-main-m-family-map-substrate":
        fail("PRE-D Map standing must be accepted-current-main and explicitly M-family scoped")
    if map_state.get("accepted_ql_main") != EXPECTED_PRE_D_MAIN:
        fail("PRE-D Map receipt must retain the exact accepted QL main")
    if map_state.get("historical_producer_pr") != 67:
        fail("PRE-D Map receipt must retain PR #67 as historical producer provenance")
    if map_state.get("map_root") != "Idea/Bimba/Map":
        fail("PRE-D Map source root must remain Idea/Bimba/Map")
    if map_state.get("coordinate_family_scope") != ["M"]:
        fail("PRE-D Map receipt must not expand beyond the M coordinate family")
    scope_law = map_state.get("scope_law", "")
    if "not the complete Epi coordinate system" not in scope_law:
        fail("PRE-D Map scope law must explicitly reject whole-coordinate-system promotion")

    coordinate_ground = data.get("orientation", {}).get("coordinate_system_ground", {})
    raw_root = coordinate_ground.get("raw_psychoid_root", {})
    if raw_root.get("family") != "FAMILY_NONE":
        fail("raw psychoid root must remain pre-categorical FAMILY_NONE")
    if raw_root.get("source") != "Body/S/S0/epi-lib/include/psychoid_numbers.h":
        fail("raw psychoid root source must remain exact")
    expected_psychoids = {
        "Psychoid_0", "Psychoid_1", "Psychoid_2", "Psychoid_3",
        "Psychoid_4", "Psychoid_5", "Psychoid_Hash",
    }
    if set(raw_root.get("coordinates", [])) != expected_psychoids:
        fail("raw psychoid coordinate set changed")

    family_manifestations = {
        (entry.get("family"), entry.get("implementation_name"), entry.get("role"))
        for entry in coordinate_ground.get("family_manifestations", [])
    }
    if family_manifestations != EXPECTED_FAMILY_MANIFESTATIONS:
        fail(f"coordinate-family manifestation field changed: {sorted(family_manifestations)!r}")
    if coordinate_ground.get("implementation_source") != "Body/S/S0/epi-lib/include/ontology.h":
        fail("coordinate-family implementation source must remain exact")
    kernel_relation = coordinate_ground.get("ql_mef_kernel_relation", {})
    if kernel_relation.get("source") != "Body/S/S0/epi-lib/include/kernel.h":
        fail("QL/MEF kernel relation must retain frozen kernel.h source")
    if "M-family" not in kernel_relation.get("law", ""):
        fail("QL/MEF kernel relation must preserve the M-family substrate distinction")
    m1_relation = coordinate_ground.get("m1_inner_logic_relation", {})
    if m1_relation.get("source") != "Body/S/S0/epi-lib/include/m1.h":
        fail("M1 inner-logic relation must retain frozen m1.h source")
    m1_law = m1_relation.get("law", "")
    if "whole coordinate ontology" not in m1_law or "without making" not in m1_law:
        fail("M1 must not be promoted into the whole coordinate ontology")

    header_symbols = public_ql_symbols(header)
    exports = data.get("native_exports", [])
    manifest_symbols = [entry.get("symbol") for entry in exports]
    counts = Counter(manifest_symbols)
    duplicates = sorted(symbol for symbol, count in counts.items() if count != 1)
    if duplicates:
        fail(f"native export symbols must occur exactly once: {duplicates}")

    manifest_symbol_set = set(manifest_symbols)
    missing = sorted(header_symbols - manifest_symbol_set)
    extra = sorted(manifest_symbol_set - header_symbols)
    if missing or extra:
        fail(f"native export coverage mismatch: missing={missing}, extra={extra}")

    for entry in exports:
        symbol = entry["symbol"]
        for key in ("implementation", "artifact_class", "formal_relation", "bimba", "technology", "authority", "evidence", "epistemic_standing"):
            require_nonempty(entry, key, symbol)

        implementation = entry["implementation"]
        if implementation.get("path") != "c/src/primitive.c":
            fail(f"{symbol}: native implementation path must remain exact")
        if implementation.get("header") != "c/include/ql/primitive.h":
            fail(f"{symbol}: native header path must remain exact")

        technology = entry["technology"]
        if "S5" not in technology.get("s_products", []):
            fail(f"{symbol}: native QL-MEF C manifestation must retain S5 embodiment")
        if technology.get("branch") != "#5-2":
            fail(f"{symbol}: native technological manifestation must retain #5-2 branch reading")

        authority = entry["authority"]
        for key in ("semantic", "computational", "provider", "standing"):
            require_nonempty(authority, key, f"{symbol}.authority")

        bimba = entry["bimba"]
        if "standing" not in bimba or not bimba["standing"]:
            fail(f"{symbol}: Bimba standing must be explicit even when unresolved")
        if "c_categories" not in bimba:
            fail(f"{symbol}: C-category account must be explicit")

        if symbol == "ql_position_invert":
            if bimba.get("coordinates") != [EXPECTED_FIRST_BIMBA_COORDINATE]:
                fail("ql_position_invert must retain exact recovered Bimba coordinate #1-4.2")
            if bimba.get("parent") != EXPECTED_FIRST_BIMBA_PARENT:
                fail("ql_position_invert must retain exact recovered Bimba parent #1-4")
            relation_text = " ".join(
                str(item.get("relation", "")) for item in bimba.get("other_relations", [])
            )
            if "INVERTS_INTO" not in relation_text or "CONTAINS_LOGIC_STAGE" not in relation_text:
                fail("ql_position_invert must retain recovered source relation and parentage evidence")

    roots = [(entry.get("coordinate"), entry.get("product")) for entry in data["orientation"]["s_roots"]]
    if roots != EXPECTED_S_ROOTS:
        fail(f"S root mapping changed: {roots!r}")

    archetypes = set(data["orientation"].get("ta_onta_archetypes", []))
    missing_archetypes = sorted(EXPECTED_ARCHETYPES - archetypes)
    if missing_archetypes:
        fail(f"Ta-Onta archetypes were dropped: {missing_archetypes}")

    corpus_paths = [entry.get("path") for entry in data.get("reference_corpus", [])]
    corpus_counts = Counter(corpus_paths)
    duplicated_tus = sorted(path for path, count in corpus_counts.items() if count != 1)
    if duplicated_tus:
        fail(f"reference translation units must occur exactly once: {duplicated_tus}")
    corpus_set = set(corpus_paths)
    missing_tus = sorted(EXPECTED_REFERENCE_TUS - corpus_set)
    extra_tus = sorted(corpus_set - EXPECTED_REFERENCE_TUS)
    if missing_tus or extra_tus:
        fail(f"frozen reference corpus coverage mismatch: missing={missing_tus}, extra={extra_tus}")

    specimen = data.get("first_holographic_specimen", {})
    code = specimen.get("code_manifestation", {})
    if code.get("symbol") != "ql_position_invert" or code.get("path") != "c/src/primitive.c":
        fail("first holographic specimen must retain exact ql_position_invert code identity")
    semantic = specimen.get("semantic_subject", {})
    if "5-p" not in semantic.get("description", ""):
        fail("first specimen must name the six-position complement relation")
    if semantic.get("bimba_coordinate") != EXPECTED_FIRST_BIMBA_COORDINATE:
        fail("first specimen must retain exact frozen-Map Bimba coordinate #1-4.2")
    if semantic.get("bimba_parent") != EXPECTED_FIRST_BIMBA_PARENT:
        fail("first specimen must retain exact frozen-Map Bimba parent #1-4")
    if semantic.get("bimba_predecessor") != EXPECTED_FIRST_BIMBA_PREDECESSOR:
        fail("first specimen must retain exact inversion predecessor #1-4.1")
    if "INVERTS_INTO" not in semantic.get("source_transition_relation", ""):
        fail("first specimen must retain exact source INVERTS_INTO relation")
    if "CONTAINS_LOGIC_STAGE" not in semantic.get("source_parent_relation", ""):
        fail("first specimen must retain exact source parentage relation")
    if "whole coordinate-system parity" not in semantic.get("ql_map_projection", {}).get("standing", ""):
        fail("first specimen projection must retain the M-family/whole-system scope firewall")

    parity = specimen.get("parity", {})
    if parity.get("result") != "pass" or "6 valid positions" not in parity.get("domain", ""):
        fail("first specimen must retain finite-domain parity evidence")

    acceptance = specimen.get("acceptance", {})
    for required in (
        "semantic_identity_survives",
        "source_coordinate_exact",
        "source_parentage_exact",
        "source_relation_exact",
        "source_revision_known",
        "code_symbol_exact",
        "numeric_parity",
        "relation_preserved",
        "aikit_bidirectional_traversal",
        "factory_structural_evidence",
        "pre_d_map_scope_is_m_family_only",
        "whole_coordinate_system_not_claimed_by_pre_d",
        "unresolved_relations_explicit",
    ):
        if acceptance.get(required) is not True:
            fail(f"first specimen acceptance lost required evidence: {required}")
    if acceptance.get("aikit_acceptance_standing") != "accepted-current-main-exact-coordinate-conformance":
        fail("AIKit evidence must record accepted current-main exact-coordinate conformance")
    if acceptance.get("factory_acceptance_standing") != "accepted-current-main-exact-coordinate-conformance-with-wider-156-open":
        fail("Factory evidence must record exact acceptance while preserving wider #156")
    if acceptance.get("bimba_graph_live_verified") is not False:
        fail("repository source recovery must not be promoted to live Bimba graph verification")

    returned = specimen.get("returned_evidence", {})
    if returned.get("bimba_map", {}).get("coordinate") != EXPECTED_FIRST_BIMBA_COORDINATE:
        fail("returned Bimba evidence must point to the exact source coordinate")
    ql_map = returned.get("ql_map_projection", {})
    if ql_map.get("accepted_main") != EXPECTED_PRE_D_MAIN or ql_map.get("scope") != "M-family":
        fail("returned QL Map evidence must point to accepted PRE-D main and retain M-family scope")
    aikit = returned.get("aikit", {})
    if aikit.get("accepted_main") != EXPECTED_AIKIT_MAIN or aikit.get("candidate_head") != EXPECTED_AIKIT_HEAD:
        fail("AIKit returned evidence must retain exact accepted main and candidate head")
    if sorted(aikit.get("workflow_runs", [])) != sorted([32287246271, 32287246211, 32287246142, 32287246138, 32287246189]):
        fail("AIKit returned evidence must retain all five exact-coordinate green workflow receipts")
    factory = returned.get("factory", {})
    if factory.get("generic_merge") != EXPECTED_FACTORY_GENERIC_MERGE:
        fail("Factory returned evidence must retain the accepted StructuralGround merge")
    if factory.get("accepted_main") != EXPECTED_FACTORY_MAIN or factory.get("exact_binding_head") != EXPECTED_FACTORY_HEAD:
        fail("Factory returned evidence must retain exact accepted main and candidate head")
    if sorted(factory.get("workflow_runs", [])) != sorted([32287295358, 32287295425, 32287295364, 32287295491, 32287295518, 32287295580]):
        fail("Factory returned evidence must retain all six exact-coordinate green workflow receipts")

    gaps = {gap.get("id"): gap for gap in data.get("known_gaps", [])}
    required_gaps = {
        "bimba-live",
        "coordinate-family-ground-beyond-m-map",
        "m1-ring-discrepancy",
        "m3-clock-generator",
        "qv-source-authority",
        "m4-blake3",
        "provisional-regions",
        "factory-structural-fidelity-programme",
    }
    if not required_gaps.issubset(gaps):
        fail(f"required structural gaps were lost: {sorted(required_gaps - set(gaps))}")
    if gaps["bimba-live"].get("status") != "repository-source-identity-recovered-live-graph-outstanding":
        fail("Bimba gap must distinguish repository identity recovery from live graph verification")
    if gaps["coordinate-family-ground-beyond-m-map"].get("status") != "scope-explicit-broader-family-reconciliation-open":
        fail("coordinate-system gap must preserve PRE-D M-family scope without claiming wider parity")
    if gaps["m3-clock-generator"].get("status") != "lineage-recovered-exact-generator-input-authority-open":
        fail("M3 generator gap must retain recovered lineage without overstating exact authority")
    if gaps["factory-structural-fidelity-programme"].get("status") != "exact-conformance-accepted-wider-programme-open":
        fail("Factory gap must preserve wider #156 after exact conformance acceptance")

    print(
        "holographic-manifest: PASS "
        f"exports={len(header_symbols)} reference_tus={len(corpus_set)} "
        f"gaps={len(gaps)} specimen={specimen.get('id')} "
        f"bimba={semantic.get('bimba_coordinate')} map_scope=M"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AssertionError, KeyError, TypeError, json.JSONDecodeError) as exc:
        print(f"holographic-manifest: FAIL: {exc}", file=sys.stderr)
        raise SystemExit(1)
