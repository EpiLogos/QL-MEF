# Epi Capability Matrix Field Index

**Standing:** architecture-contract  
**Register:** episteme  
**Provenance:** QL-MEF PR #93 merge `b6c7443`; human-authored R3 ratification  

**Status:** active R3 field index  
**Date:** 2026-09-03

## Read the field in this order

1. `ProjectCentral/user/capability-matrix.{json,csv,md}` — the suite-facing product profile consumed by O:I. It indexes concrete QL product powers; it does not replace the native Epi matrices below.
2. `EPI-DEEP-SUBSYSTEM-CAPABILITY-MATRIX-PROTOCOL.md` — method, provenance and standing discipline.
3. `EPI-TA-ONTA-AGENT-WORLD-CAPABILITY-MATRIX.md` — the 6 × 6 M × S′ inhabitation field.
4. `EPI-M-CAPABILITY-FIELD.md` — the 36 recovered real capabilities, now read through M′ operation, S′ composition, native O:I embodiment, Epii-on-X development and Return.
5. `epi-m-capability-field.json` + `epi-m-capability-field-m0.json` … `epi-m-capability-field-m5.json` — normalized machine carrier/evidence registry.
6. `docs/origami work/M1` … `M4/*-DEEP-CAPABILITY-COORDINATE-MATRIX.{json,md}` — the four current subsystem-specific deep coordinate matrices. These retain their own richer v2 schema and are not partitions of the normalized 36-capability field.
7. `epi-relational-field`, `epi-ssprime-relational-field` and `epi-ta-onta-m-relational-field` — the M↔M′, S↔S′ and Ta-Onta↔M relational carriers. Each CSV is paired with its own `.matrix.json` `ql-capability-matrix/1` declaration.
8. `epi-epii-operational-capacities.json` — exact six source-backed M5′ Epii-on-X developmental relations.
9. `EPI-CAPABILITY-MATRIX-SOURCE-TRACE.md` — current locks, exact source blobs, live implementation deltas and historical lineage.

The machine-readable inventory of these matrix families is
`.oi/product.json#capability_matrices`. Exactly one entry has
`suite_catalogue: true`; O:I may collate that interoperable profile while
retaining the identities and carrier paths of every native matrix. A repository
therefore owns one or more matrices, rather than being forced into one flattened
matrix. CI verifies the inventory and every declared carrier with
`scripts/check-capability-matrix-registry.py`.

Completeness is filename-audited under `ProjectCentral/user` and `docs`: a file
whose name contains `capability-matrix` or `relational-field` must be owned by a
registered family or appear in the registry's explicit exclusion map with a
reason. The field index, source trace and protocol are excluded because they
govern or describe matrix instances; they are not themselves matrix carriers.

## One composed field

```text
M
canonical/domain determination
        ↓
M′
lived / playable / reflected instrument
        ↓
S′ = Ta-Onta
S0′ Khora · S1′ Hen · S2′ Pleroma
S3′ Chronos · S4′ Anima · S5′ Aletheia
        ↓
S = native O:I mechanics
Central · Actuation · AIKit · Factory · Workcell · QL-MEF
        ↓
actual activity / evidence / reading / trace
        ↓
M5′ Epii-on-X
subsystem-specific developmental interpretation
        ↓
Return / Recognition
back to the authority owning target ground
```

These are not competing decompositions. The S′ matrix describes **how an Epi Agent-world is constituted**. The 36-capability field describes **what real capacities are being constituted and operated**. The Epii-on-X family describes **how each M domain may deliberately develop from returned evidence**.

## Standing boundary

A live native mechanic can be an `IMPLEMENTATION-FACT` while the specifically Epi composition using it remains `DERIVED-CURRENT-ARCHITECTURE`. An Epii-on-X file can be a `CURRENT-DOMAIN-SPEC / RESEARCH-PROPOSITION` while its proposed training/construction pipeline remains unimplemented. Returned model outputs, embeddings, pathways, WikiReadings and developmental evidence remain proposals or derived state until the authority owning target ground recognises them.

## Current notation

The S′ coordinates are exactly:

```text
S0′ Khora
S1′ Hen
S2′ Pleroma
S3′ Chronos
S4′ Anima
S5′ Aletheia
```

Ta-Onta is the S′ whole. Same-index S↔S′ relations remain structural affinities, not exclusive repository ownership.
