#!/usr/bin/env python3
"""Validate that the native Epi/QL C floor is not structurally orphaned.

This validator deliberately does not invent unresolved Bimba semantics. It checks
that every public native ql_* export has exactly one explicit manifestation
account, that the frozen corpus remains exhaustively classified, that the locked
product/archetype orientation has not been silently flattened, and that returned
cross-repository evidence is represented at its actual acceptance standing.
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
        "unresolved_relations_explicit",
    ):
        if acceptance.get(required) is not True:
            fail(f"first specimen acceptance lost required evidence: {required}")
    if acceptance.get("aikit_acceptance_standing") != "branch-level-exact-coordinate-conformance-not-main":
        fail("AIKit evidence must remain branch-level rather than being promoted to accepted main")
    if acceptance.get("factory_acceptance_standing") != "generic-current-main-plus-exact-coordinate-draft-pr":
        fail("Factory evidence must distinguish accepted generic ground from open exact-coordinate strengthening")
    if acceptance.get("bimba_graph_live_verified") is not False:
        fail("repository source recovery must not be promoted to live Bimba graph verification")

    returned = specimen.get("returned_evidence", {})
    if returned.get("bimba_map", {}).get("coordinate") != EXPECTED_FIRST_BIMBA_COORDINATE:
        fail("returned Bimba evidence must point to the exact source coordinate")
    if returned.get("aikit", {}).get("accepted_main") is not False:
        fail("AIKit returned evidence must not claim accepted-main standing")
    if returned.get("factory", {}).get("generic_merge") != "dde3ddc7c666f740c022ab347100369563cce90b":
        fail("Factory returned evidence must retain the accepted StructuralGround merge")

    gaps = {gap.get("id"): gap for gap in data.get("known_gaps", [])}
    required_gaps = {
        "bimba-live",
        "m1-ring-discrepancy",
        "m3-clock-generator",
        "qv-source-authority",
        "m4-blake3",
        "provisional-regions",
        "aikit-project-reflection",
        "factory-structural-ground",
    }
    if not required_gaps.issubset(gaps):
        fail(f"required structural gaps were lost: {sorted(required_gaps - set(gaps))}")
    if gaps["bimba-live"].get("status") != "repository-source-identity-recovered-live-graph-outstanding":
        fail("Bimba gap must distinguish repository identity recovery from live graph verification")
    if gaps["aikit-project-reflection"].get("status") != "branch-exact-coordinate-conformance-running-not-main":
        fail("AIKit gap must record exact branch-level returned state")
    if gaps["factory-structural-ground"].get("status") != "generic-resolved-exact-coordinate-strengthening-open":
        fail("Factory gap must record generic acceptance without closing wider or exact-binding work")

    print(
        "holographic-manifest: PASS "
        f"exports={len(header_symbols)} reference_tus={len(corpus_set)} "
        f"gaps={len(gaps)} specimen={specimen.get('id')} "
        f"bimba={semantic.get('bimba_coordinate')}"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AssertionError, KeyError, TypeError, json.JSONDecodeError) as exc:
        print(f"holographic-manifest: FAIL: {exc}", file=sys.stderr)
        raise SystemExit(1)
