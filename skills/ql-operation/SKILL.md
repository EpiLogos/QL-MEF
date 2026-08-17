---
name: ql-mef-operation
description: Operate the standalone QL/MEF provider through its versioned capabilities, locate, refract, relate and synthesise contracts while preserving provenance and optional/no-QL behaviour.
---

# QL/MEF operation

Use this Skill when an authorised actor needs a bounded formal QL operation or MEF reading from the standalone QL/MEF product.

## Contract metadata

- Semantic ref: `ql-mef:operator`
- Native owner: `EpiLogos/QL-MEF`
- Public service: `ql_service::QlService`
- Stable operation family: `capabilities`, `locate`, `refract`, `relate`, `synthesise`
- Client adapters: `ql-adapters`
- Verification: `cargo test --workspace --all-targets` plus `bash scripts/verify-native-skills.sh`
- Risk class: formal/epistemic; a reading is not authority or Recognition

## Product boundary

QL/MEF owns executable QL references/forms/addresses, the twelve-lens MEF registry, provider capability negotiation, formal/semantic operations and their provenance. It does not own Factory Projects/Runs/Actions, AIKit ContextResolution, Workcell materialisation, Actuation authority, Central Control or the QL Loop Runtime.

Keep these distinctions explicit:

```text
QL alignment != renaming another product's object
MEF reading != replacement identity
provider advertised != operation supported
semantic stochastic reading != deterministic theorem
QL optional != hidden prerequisite
no-QL != client-data loss
successful experiment != canon promotion
Skill available != Action authorised
```

## Inputs

Obtain the client subject/ref and exact revision where available; requested QL/MEF operation; target/lens/sublens/frame/context references required by that operation; the caller's QL mode (`Disabled`, `Optional`, or `Required`) when using an adapter; and the provider/service capabilities and health.

## Procedure

1. **Negotiate before operating.** Call `capabilities` or inspect `ServiceCapabilities`. Record provider health and advertised provider capabilities. For a specific operation, preserve whether it is supported and deterministic rather than inferring support from provider presence.
2. **Preserve the subject identity.** Carry the client's original reference and revision into the QL target/provenance. Refraction reveals a relation of the object; it does not rename the object into a QL noun.
3. **Use the canonical service operation.** Dispatch `Locate`, `Refract`, `Relate` or `Synthesise` through `QlService`/`ServiceRequest`; do not call private provider implementation state as the normal procedure.
4. **Validate coordinates before provider execution.** Lens and sublens relations must be structurally valid. A mismatched sublens must fail rather than being coerced, including in optional mode.
5. **Treat provider health honestly.** Available, degraded, absent and incompatible states remain distinct. A degraded advertised provider may return a usable reading with degraded health; do not silently upgrade its standing.
6. **Preserve provenance.** For semantic readings, retain provider identity/version, result class, model/config references where present, and input references/revisions. Distinguish deterministic formal results from semantic/stochastic readings.
7. **Respect no-QL behaviour.** In `Disabled` mode, preserve client data exactly and attach no QL reading. In `Optional` mode, absent/incompatible/unadvertised provider state is non-fatal and client identity/data remain intact. In `Required` mode, absence/provider failure is a hard visible error.
8. **Synthesis does not grant authority.** Return the result to the calling product as a reading/evidence relation. QL/MEF does not recognise a Factory Candidate, authorise an Action, mutate Central, materialise Workcell state or promote its own canon from one result.

## Outputs

Return the original subject/ref/revision, operation, provider health/capability decision, formal or semantic result, exact QL/MEF coordinates, and complete available provenance. For optional unavailability, return the unchanged client material plus explicit unavailable/degraded state rather than inventing a reading.

## Verification

Run the Rust workspace tests. The acceptance matrix must retain the `ql-adapters/tests/noql_matrix.rs` cases for disabled, optional, required, degraded/incompatible and invalid-coordinate behaviour, together with service/core/MEF contract tests.
