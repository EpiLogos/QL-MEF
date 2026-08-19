#!/usr/bin/env python3
"""Static conformance for QL-MEF #75 M/S fourfold R0/R1.

No network/runtime dependencies: this verifies only checked-in relation artifacts.
"""

from __future__ import annotations

import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EPI = ROOT / "docs" / "integrations" / "epi-logos"

EXPECTED_M = ["Anuttara", "Paramaśiva", "Paraśakti", "Mahāmāyā", "Nara", "Epii"]
EXPECTED_S = ["Central", "Actuation", "AIKit", "Software Factory", "Workcell", "QL-MEF"]
EXPECTED_SP = ["Khora", "Hen", "Pleroma", "Chronos", "Anima", "Aletheia"]
EXPECTED_MP = [f"epi.deep.m{i}" for i in range(6)]
OLD_TOP_LEVEL = {
    "CommandRuntimeGround",
    "VaultResidency",
    "GraphBody",
    "GatewayControl",
    "AgentRuntime",
    "WorldBoundary",
    "ReflectiveCLILaw",
    "HenCompilerLaw",
    "CoordinateGraphLaw",
    "TemporalStateLaw",
    "TaOntaInhabitation",
    "EpiiReturnLaw",
}


def load_json(name: str):
    with (EPI / name).open(encoding="utf-8") as fh:
        return json.load(fh)


def fail(message: str) -> None:
    raise AssertionError(message)


def main() -> None:
    fourfold = load_json("epi-ms-fourfold.json")
    embodiment = load_json("epi-ssprime-embodiment.json")
    mmprime = load_json("epi-mmprime-relations.json")

    rows = fourfold.get("rows", [])
    if len(rows) != 6:
        fail(f"expected exactly six fourfold rows, got {len(rows)}")
    if [row.get("index") for row in rows] != list(range(6)):
        fail("fourfold indices must be exactly 0..5 in order")

    for i, row in enumerate(rows):
        expected = (EXPECTED_M[i], EXPECTED_S[i], EXPECTED_SP[i], EXPECTED_MP[i])
        actual = (
            row.get("m_identity"),
            row.get("s_product"),
            row.get("s_prime_ta_onta"),
            row.get("m_prime_product"),
        )
        if actual != expected:
            fail(f"coordinate {i} mismatch: expected {expected}, got {actual}")
        if row.get("canonical_guardian_agent") != EXPECTED_M[i]:
            fail(f"coordinate {i} guardian must equal M identity")
        if row.get("m_prime_identity") != EXPECTED_M[i]:
            fail(f"coordinate {i} M′ identity must preserve M identity")
        if not row.get("source_refs"):
            fail(f"coordinate {i} has no source/provenance refs")
        if len(row.get("reflection_relations", [])) < 5:
            fail(f"coordinate {i} is missing constitutional reflection/stewardship relations")

    if rows[5]["s_prime_ta_onta"] == "Epii":
        fail("Epii must not be canonical S5′")
    if rows[5]["s_prime_ta_onta"] != "Aletheia":
        fail("Aletheia must be canonical S5′")

    canonical_names = set(EXPECTED_S + EXPECTED_SP)
    if canonical_names & OLD_TOP_LEVEL:
        fail("historical technical strata leaked into canonical top-level faces")

    # Historical nested aliases are allowed only inside explicit supersession/provenance data.
    for i, row in enumerate(rows):
        aliases = row.get("superseded_aliases_or_shapes", {})
        nested = aliases.get("nested_ta_onta_alias", "")
        if nested != f"S4.{i}′ {EXPECTED_SP[i]}":
            fail(f"coordinate {i} must retain its nested Ta-Onta alias as provenance")
        disposition = aliases.get("disposition", {}).get("nested_ta_onta_alias")
        if disposition != "IMPLEMENTATION-RESIDENCY-ONLY":
            fail(f"coordinate {i} nested alias must be residency-only")

    with (EPI / "epi-ssprime-relational-field.csv").open(encoding="utf-8", newline="") as fh:
        relation_rows = list(csv.DictReader(fh))
    if not relation_rows:
        fail("S/S′ relation field is empty")

    declared_faces = {f"S{i}" for i in range(6)} | {f"S{i}′" for i in range(6)}
    ql_tokens: set[str] = set()
    for relation in relation_rows:
        if relation["src_face"] not in declared_faces or relation["dst_face"] not in declared_faces:
            fail(f"undeclared relation face: {relation['id']}")
        if relation["src_product"] not in canonical_names or relation["dst_product"] not in canonical_names:
            fail(f"non-canonical current relation product: {relation['id']}")
        if any(old in (relation["src_product"], relation["dst_product"]) for old in OLD_TOP_LEVEL):
            fail(f"historical alias used as a current relation face: {relation['id']}")
        ql_tokens.update(filter(None, relation.get("ql", "").replace(":", "-").split("|")))

    # Preserve the full grammar without requiring a 144-row implementation backlog.
    ql_text = "\n".join(row.get("ql", "") for row in relation_rows)
    for required in ("A1", "A2", "A3", "B1", "B2", "B3", "C1", "C2", "C3", "D1", "D2-transform", "D2-require", "D2-complete", "D3:"):
        if required not in ql_text:
            fail(f"S/S′ relation field lost {required} grammar provenance")

    spine = embodiment.get("constitutional_spine", [])
    if len(spine) != 6:
        fail("embodiment map must contain six constitutional spine rows")
    for i, row in enumerate(spine):
        if row.get("index") != i:
            fail("embodiment spine indices must be 0..5")
        if row.get("s_product") != EXPECTED_S[i] or row.get("s_prime") != EXPECTED_SP[i]:
            fail(f"embodiment spine coordinate {i} disagrees with canonical fourfold")
        if row.get("m_prime_product") != EXPECTED_MP[i]:
            fail(f"embodiment spine coordinate {i} M′ product mismatch")

    capability_rows = embodiment.get("capability_embodiment", [])
    if len(capability_rows) != 6:
        fail("expected one capability-embodiment summary per M coordinate")
    if not any(len(row.get("cross_coordinate_products", [])) > 2 for row in capability_rows):
        fail("cross-coordinate product embodiment was accidentally made exclusive")
    if not any(len(row.get("cross_coordinate_ta_onta", [])) > 1 for row in capability_rows):
        fail("cross-coordinate Ta-Onta embodiment was accidentally made exclusive")
    legacy = embodiment.get("legacy_capability_map", {})
    if legacy.get("disposition") != "PRESERVE-AS-HISTORY" or not legacy.get("blob_sha"):
        fail("pre-migration detailed capability map must remain inspectable provenance")

    # Existing M/M′ relation fixture must remain valid and remain in M0..M5 namespace.
    mm_relations = mmprime.get("relations", [])
    if not mm_relations:
        fail("existing M/M′ relation fixture is empty")
    valid_m = {f"M{i}" for i in range(6)}
    for relation in mm_relations:
        if relation.get("source") not in valid_m or relation.get("target") not in valid_m:
            fail(f"existing M/M′ fixture contains invalid coordinate: {relation.get('id')}")

    print(
        "epi-ms-fourfold conformance: PASS "
        f"({len(rows)} fourfold rows, {len(relation_rows)} S/S′ relation rows, "
        f"{len(capability_rows)} capability summaries, {len(mm_relations)} M/M′ relations)"
    )


if __name__ == "__main__":
    main()
