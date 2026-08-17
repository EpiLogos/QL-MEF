# Epi-Logos living relation-map home

Tracking issue: #25. Related QL relation/meta-wiki programme: #7.

This directory is the QL-MEF home for the **living relation and cross-field maps** used to integrate Epi-Logos into the newer O:I-era architecture.

It is deliberately not the canonical source of Epi-Logos domain truth. Epi source authority remains in `EpiLogos/Epi-Logos-C-Experiments`; this directory holds version-aware **mappings over that source**.

## Coordinate model

```text
M / M' = Epi-Logos subsystem/domain field

M0 Anuttara
M1 Paramasiva
M2 Parashakti
M3 Mahamaya
M4 Nara
M5 Epii

S / S' = Epi technical/runtime strata and their conjugate/augmentation laws
```

Do not alias Anuttara→Epii to S0→S5.

## Current accepted Epi source input

R1 was merged in `EpiLogos/Epi-Logos-C-Experiments` by PR #7 / merge commit:

`be54a505728eaa06ddcc268fa53df5dd756bfb5e`

The R1 source reading itself is pinned to Epi source head:

`8608648f33e697dd5a8c5f499492619a02259af5`

Canonical R1 inputs:

- `Idea/Bimba/Seeds/Reconstitution/R1/AUTHORITATIVE-SOURCE-MANIFEST.md`
- `Idea/Bimba/Seeds/Reconstitution/R1/CYCLE3-M-MPRIME-CAPABILITY-MATRIX.md`
- `Idea/Bimba/Seeds/Reconstitution/R1/S-SPRIME-TECHNICAL-CAPABILITY-MATRIX.md`
- `Idea/Bimba/Seeds/Reconstitution/R1/BIMBA-CANONICAL-INVENTORY.md`
- `Idea/Bimba/Seeds/Reconstitution/R1/LEGACY-TECHNOLOGY-LEDGER.md`
- `Idea/Bimba/Seeds/Reconstitution/R1/CAPABILITY-INVENTORY.json`
- `Idea/Bimba/Seeds/Reconstitution/R1/README.md`

The local `r1-source-snapshot.json` records this linkage machine-readably.

## Maps developed here

The living R2 field should resolve into four linked maps rather than one overloaded matrix:

```text
Epi M/M' internal relations
        │
        ├── M/M' ↔ S/S' embodiment/authority
        │
        ├── O:I ↔ Epi ownership/parity
        │
        └── Bimba bridge/integration relations
```

Expected machine-readable surfaces, when actually derived from evidence:

- `epi-mmprime-relations.json`
- `epi-ssprime-embodiment.json`
- `oi-epi-cross-map.json`
- `bimba-bridge-map.json`

Human-readable views can be generated alongside them. These names are working homes, not an instruction to invent empty symmetry or pre-fill relation cells.

## Mapping law

Preserve:

```text
Epi domain identity != O:I product identity
M/M' domain identity != S/S' technical stratum identity
provider/body != semantic owner
projection != ownership
availability != authority
shared relation != canonical merge
formal QL reading != application truth
Bimba != Neo4j != MCP
```

A relation cell is evidence-bearing mapping information, not a backlog item.

## Bimba / QL-MEF graph boundary

QL-MEF #7 already fixes the key graph distinction:

```text
QL-MEF OKF meta-wiki / Meta-Knowledge Graph Projection
    !=
Epi-Logos canonical Bimba Graph
```

Any relation between them is an explicit bridge/mapping with source ref, revision, relation/operator, scope, authority and provenance.

Shared coordinates or MEF refraction do not transfer Bimba ownership into QL-MEF and do not make an external WikiFrame part of Bimba.

## Snapshot versus living map

The R1 Epi capability map is intentionally versioned. It answers what the Epi source field said at one exact revision.

The maps in this directory are intentionally **living**. A later Epi source snapshot may update implementation state, evidence or capability details while preserving stable relation identities and their history.

This distinction lets Epi-Logos, QL/MEF and the O:I products continue developing without either losing reproducibility or treating a map as a project freeze.

## Next work

Execute #25 from current live target-platform state:

1. consume the accepted R1 machine inventory;
2. derive the Epi M/M' internal relation dataset;
3. derive M/M'↔S/S' embodiment/authority relations;
4. verify live O:I/native-product contracts and derive the cross-map;
5. derive the explicit Bimba bridge map;
6. only then derive bounded migration/reuse/supersession pressures.
