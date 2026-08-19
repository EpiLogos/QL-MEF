#!/usr/bin/env python3
"""Static conformance for QL-MEF #42 canonical Epi guardian materialisation."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EPI = ROOT / "docs" / "integrations" / "epi-logos"

EXPECTED = [
    (0, "Anuttara", "Central", "EpiLogos/Central", "Khora", "epi.deep.m0", "epi:agent:anuttara"),
    (1, "Paramaśiva", "Actuation", "EpiLogos/Actuation", "Hen", "epi.deep.m1", "epi:agent:paramasiva"),
    (2, "Paraśakti", "AIKit", "EpiLogos/ai-kit", "Pleroma", "epi.deep.m2", "epi:agent:parasakti"),
    (3, "Mahāmāyā", "Software Factory", "EpiLogos/agent-system-design", "Chronos", "epi.deep.m3", "epi:agent:mahamaya"),
    (4, "Nara", "Workcell", "EpiLogos/Workcell", "Anima", "epi.deep.m4", "epi:agent:nara"),
    (5, "Epii", "QL-MEF", "EpiLogos/QL-MEF", "Aletheia", "epi.deep.m5", "epi:agent:epii"),
]
EXPECTED_ROUTES = {
    "source/provenance/world-ground": "epi:agent:anuttara",
    "agency/authority/delegation": "epi:agent:paramasiva",
    "capability/context/provider": "epi:agent:parasakti",
    "development/praxis/source-code": "epi:agent:mahamaya",
    "material/provider/persistence": "epi:agent:nara",
    "formal/refractive/whole-system": "epi:agent:epii",
}


def load(name: str) -> dict:
    return json.loads((EPI / name).read_text(encoding="utf-8"))


def fail(message: str) -> None:
    raise AssertionError(message)


fourfold = load("epi-ms-fourfold.json")
guardians = load("epi-guardians.json")
material = load("epi-guardian-materialisation.json")

if guardians.get("schema_version") != "epi.guardians/v1":
    fail("guardian registry schema version drift")
if material.get("schema_version") != "epi.guardian-materialisation/v1":
    fail("guardian materialisation schema version drift")

fourfold_rows = sorted(fourfold["rows"], key=lambda row: row["index"])
guardian_rows = sorted(guardians["guardians"], key=lambda row: row["index"])

if len(fourfold_rows) != 6 or len(guardian_rows) != 6:
    fail("exactly six constitutional guardians are required")

for expected, source, guardian in zip(EXPECTED, fourfold_rows, guardian_rows, strict=True):
    idx, name, product, repo, ta_onta, deep, agent_ref = expected
    if source["index"] != idx or guardian["index"] != idx:
        fail(f"coordinate index drift at {idx}")
    if source["canonical_guardian_agent"] != name or source["m_identity"] != name:
        fail(f"fourfold M/guardian drift at {idx}")
    if source["s_product"] != product or source["s_product_repo"] != repo:
        fail(f"fourfold S drift at {idx}")
    if source["s_prime_ta_onta"] != ta_onta or source["m_prime_product"] != deep:
        fail(f"fourfold S-prime/M-prime drift at {idx}")
    if guardian["name"] != name or guardian["m_identity"] != name:
        fail(f"guardian identity drift at {idx}")
    if guardian["s_product"] != product or guardian["s_repo"] != repo:
        fail(f"guardian standing-product stewardship drift at {idx}")
    if guardian["s_prime"] != ta_onta or guardian["m_prime"] != deep:
        fail(f"guardian fourfold stewardship drift at {idx}")
    if guardian["agent_ref"] != agent_ref:
        fail(f"guardian durable ref drift at {idx}")

identity_law = guardians["identity_law"]
if identity_law.get("factory_stage_identity") is not False:
    fail("guardians must not be interpreted as historical Factory stages")
if identity_law.get("model_harness_provider_independent") is not True:
    fail("guardian identity must remain model/harness/provider independent")

materialisations = material["materialisations"]
if len(materialisations) != 6:
    fail("all six guardians require a native materialisation")
by_ref = {row["guardian_ref"]: row for row in materialisations}
if set(by_ref) != {row[-1] for row in EXPECTED}:
    fail("native materialisation guardian set drift")

for expected in EXPECTED:
    idx, name, product, repo, ta_onta, deep, agent_ref = expected
    row = by_ref[agent_ref]
    if row["native_product_repo"] != repo:
        fail(f"{name}: native product repo drift")
    actuation = row["actuation"]
    binding = actuation["world_binding"]
    root_scope = actuation["root_scope"]
    if binding["schema"] != "actuation.agency/v1":
        fail(f"{name}: not using native Actuation agency-v1")
    if binding["agent_ref"] != agent_ref:
        fail(f"{name}: WorldBinding must bind canonical Agent ref")
    if binding["agency_ref"] == agent_ref:
        fail(f"{name}: Guardian identity collapsed into situated Agency")
    if binding["world_ref"] != f"github:{repo}":
        fail(f"{name}: WorldBinding must target standing native product")
    if root_scope["schema"] != "actuation.agency/v1":
        fail(f"{name}: RootScope not using native Actuation agency-v1")
    if root_scope["scope_ref"] != binding["scope_ref"]:
        fail(f"{name}: RootScope and WorldBinding scope mismatch")

    aikit = row["aikit"]
    required = {
        "profile_ref", "skill_set_ref", "context_ref",
        "context_world_binding_ref", "method_resolution",
        "session_space_resolution", "model_harness_provider_selection",
    }
    missing = sorted(required - set(aikit))
    if missing:
        fail(f"{name}: AIKit composition missing {missing}")
    if aikit["context_world_binding_ref"] != binding["binding_ref"]:
        fail(f"{name}: AIKit Context must preserve WorldBinding provenance")
    if aikit["model_harness_provider_selection"] != "resolved-at-execution":
        fail(f"{name}: model/harness/provider must remain execution-time selection")
    if any(aikit[key] is not None for key in ("fixed_model", "fixed_harness", "fixed_provider")):
        fail(f"{name}: guardian illegally fixed to model/harness/provider")
    if row["material_body"]["owner"] != "EpiLogos/Workcell":
        fail(f"{name}: material body must remain Workcell-owned")
    if row["developmental_run"]["owner"] != "EpiLogos/agent-system-design":
        fail(f"{name}: developmental Run/Evidence must remain Factory-owned")

specimen = material["labouring_agency_specimen"]
if specimen["guardian_ref"] != "epi:agent:epii":
    fail("native product operating specimen must use Epii")
determination = specimen["determination"]
returned = specimen["return"]
if determination["schema"] != "actuation.agency/v1" or returned["schema"] != "actuation.agency/v1":
    fail("labouring Agency and Return must use native Actuation agency-v1")
if determination["differentiated_agency_ref"] == specimen["guardian_ref"]:
    fail("labouring Agency must remain distinct from guardian")
if returned["from_agency_ref"] != determination["differentiated_agency_ref"]:
    fail("Return must be attributable to the labouring Agency")
if returned["to_agency_ref"] != determination["determining_agency_ref"]:
    fail("Return must reintegrate to the determining stewardship Agency")
if returned["determination_ref"] != determination["determination_ref"]:
    fail("Return must cite its determination")
if not returned["evidence_refs"]:
    fail("Return requires evidence provenance")

operation = specimen["native_product_operation"]
if operation["product_repo"] != "EpiLogos/QL-MEF":
    fail("Epii operating proof must target Epii's standing QL-MEF product")
reading_refs = {(r["ref"], r["operation"], r["source"]) for r in operation["readings"]}
expected_readings = {
    ("ql-service:locate", "Operation::Locate", "crates/ql-service/src/read_ops.rs"),
    ("ql-service:refract", "Operation::Refract", "crates/ql-service/src/read_ops.rs"),
}
if reading_refs != expected_readings:
    fail("Epii operating proof must use the canonical QL-MEF locate/refract readings")

whole = material["whole_field_guardian"]
if whole.get("numbered_coordinate") is not False or whole.get("coordinate") is not None:
    fail("0/1 whole-field Guardian must not become a numbered coordinate")
if whole["ref"] in {row[-1] for row in EXPECTED}:
    fail("0/1 whole-field Guardian must not alias a canonical guardian identity")
if whole["composes_guardian_refs"] != [row[-1] for row in EXPECTED]:
    fail("0/1 whole-field Guardian must compose the six guardians exactly")
grant = whole["metagency_grant"]
if grant["schema"] != "actuation.agency/v1":
    fail("whole-field routing must use native Actuation metagency semantics")
if set(grant["operations"]) != {"determine-agency", "reintegrate-return"}:
    fail("whole-field metagency must be routing/reintegration only")
routes = {row["discrepancy"]: row["guardian_ref"] for row in whole["routes"]}
if routes != EXPECTED_ROUTES:
    fail("whole-field discrepancy routing drift")

encoded = json.dumps({"guardians": guardians, "material": material}, ensure_ascii=False)
for forbidden in ('"index": 6', '"coordinate": 6', '"P6"', '"M6"', '"M7"'):
    if forbidden in encoded:
        fail(f"forbidden seventh-coordinate encoding found: {forbidden}")

print("Epi guardian conformance: 6/6 canonical identities and native materialisations OK")
print("Epii native operation: locate + refract with attributable labouring Agency/Return OK")
print("0/1 whole-field metagency: non-numbered six-route composition OK")
