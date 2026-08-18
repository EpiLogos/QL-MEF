# Epi-Logos living relational development system

Tracking: QL-MEF #25 / #30 · O:I #29 · Epi source #2/#3/#4

This directory is the QL-MEF home for the **living relational development system** used to understand and reconstitute Epi-Logos through the current O:I suite.

The normal entrypoint is:

`EPI-LOGOS-SOURCE-OF-TRUTH.md`

The normal development procedure is:

`EPI-LOGOS-DEVELOPMENT-WAYFINDER.md`

## The three interoperating QL fields

The system is organised around three 12×12 relation fields using the same native QL grammar:

```text
M/M′ — Epi product/domain field
M0..M5 / M0′..M5′
        ↓ capability embodiment
S/S′ — Epi technical/inhabitation field
S0..S5 / S0′..S5′
        ↓ native ownership / conformance
O:I — technological product field
H0..H5 / A0..A5
```

Canonical manipulable files:

- `epi-relational-field.csv` — M/M′ 12×12 domain/lived field;
- `epi-ssprime-relational-field.csv` — S/S′ 12×12 stack/prime field;
- O:I `data/ql-relational-field.csv` tracked by O:I #29 — H/A 12×12 suite field.

All three use the same core schema:

```text
id, src_product, dst_product, ql, coverage,
cf_view, seam, defined_in, tracked_by
```

and the same underlying QL relation grammar:

- A/B/C families;
- D1/D2/D3 conjugate relations;
- CF1–CF7 contextual readings where germane;
- `H/S/L/W/I` developmental coverage.

The shared QL form permits relation, complement and refraction across the three scales. It does **not** assert semantic identity between same-numbered faces.

## How the fields connect

`epi-ssprime-embodiment.json` is the capability bridge between M/M′ and S/S′. It records which S strata embody each M′ capability, which authority remains Epi-native, and which modern generic O:I homes are implicated.

`EPI-OI-PRIMITIVE-OWNERSHIP-MAP.md` is the semantic ownership bridge from those stack/capability concerns into current native O:I primitives and product responsibilities.

So a real development concern can be followed as:

```text
M/M′ meaning + relation
        ↓
M capability refs
        ↓
S/S′ embodiment + relation
        ↓
O:I product/capability relation
        ↓
actual source / code / data / provider
        ↓
vertical implementation + evidence
        ↓
returned remapping
```

## S/S′ source model

The current Seed index defines the sixfold stack as a return circuit:

```text
S0  execution            S0′ Khora
S1  residency/form       S1′ Hen
S2  graph/retrieval      S2′ Pleroma
S3  time/routing         S3′ Chronos
S4  agent runtime        S4′ Anima
S5  world/return         S5′ Epii return law
```

S0 makes the system executable; S1 resident and typed; S2 graph/vector/cache real; S3 temporal and routed; S4 agentically inhabited; S5 world-facing and reflective; S5 returns to S0.

Aletheia is a crystallisation/disclosure/return membrane in the S4.5′→S5′ region, not a seventh numbered S face.

## Grounding and evidence

Use these files only when the current concern needs their depth:

- `EPI-LOGOS-QL-MEF-FOUNDATION.md` — formal/harmonic/music substrate;
- `EPI-OI-PRIMITIVE-OWNERSHIP-MAP.md` — primitive and native-owner semantics;
- `EPI-LOGOS-SOURCE-SUBSTRATE-INVENTORY.md` — exact historical/current code, data and source bodies;
- `EPI-LOGOS-RECONSTITUTION-DISPOSITION.md` — preservation/rebuild/parity law.

The older R2 first/second/final passes, cross-document matrix and JSON relation maps are provenance/research companions. They explain how current readings were reached; they are not additional systems a normal implementation session must traverse.

## Development law

Begin from an actual capability, discrepancy or desired experience. Locate it in M/M′, follow its embodiment into S/S′, inspect the relevant O:I relation neighbourhood, then reach the real code/data and build the smallest coherent whole.

**QL generates questions, not obligations.** Complete 12×12 fields make relations perceptible; they do not imply 144 integrations.

Preserve throughout:

```text
M/M′ domain identity != S/S′ technical identity
S/S′ technical identity != O:I product identity
M/M′ != O:I H/A
provider/body != semantic owner
projection != ownership
availability != authority
formal relation != application truth
Bimba != Neo4j != QL-MEF Meta-Knowledge Graph
```
