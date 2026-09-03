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

S  = generic O:I powers (Central, Actuation, AIKit, Factory, Workcell, QL-MEF)
S′ = Ta-Onta Agent-world composition of those powers
     S0′ Khora · S1′ Hen · S2′ Pleroma · S3′ Chronos · S4′ Anima · S5′ Aletheia
```

Do not alias Anuttara→Epii to S0→S5. Same-index S↔S′ is affinity, not exclusive ownership. Ta-Onta is the S′ whole, not a nested S4′ package.

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

Later capability-matrix research may supersede or deepen particular relation readings without rewriting the historical R1 snapshot. Current living maps must record the exact newer source revision they consume.

## Maps developed here

The **current living R3 field** is entered through `EPI-CAPABILITY-MATRIX-FIELD-INDEX.md`.

Ratified carriers:

- `EPI-DEEP-SUBSYSTEM-CAPABILITY-MATRIX-PROTOCOL.md`
- `EPI-TA-ONTA-AGENT-WORLD-CAPABILITY-MATRIX.md`
- `EPI-M-CAPABILITY-FIELD.md`
- `EPI-CAPABILITY-MATRIX-FIELD-INDEX.md`
- `EPI-CAPABILITY-READINESS-LEDGER.md`
- `EPI-CAPABILITY-MATRIX-SOURCE-TRACE.md`

Machine files:

- `epi-m-capability-field.json`
- `epi-ta-onta-m-relational-field.csv`
- `epi-capability-readiness.json`

The R2 four linked maps remain historical/lineage work under #25. They are still-open derivation work, not the present field home:

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
VākRef != ActionRef
addressability != executability != availability != authority
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

## Current Vāk / Agent-Native runtime line

The current deep M0/M5 capability-matrix pass in `EpiLogos/Epi-Logos-C-Experiments` at commit `d17bcbb0361db5b3bdabc932963a56687f66fea5` determines a concrete QL-MEF development line over the mature O:I architecture.

The governing specification is:

- `EPI-VAK-AGENT-NATIVE-RUNTIME.md`
- machine contract: `../../../schemas/epi-vak-registry-v1.schema.json`

The recovered relation is:

```text
M0 Anuttara source language
  109 current source entries
  + Śiva operative syntax
  + Śakti @0..@5 internal field
  + O#/X#/N#/M#/#/##/R# relations
        ↓
QL-MEF source-pinned Vāk registry / relation / refraction
        ↓
existing O:I typed address + Search/Command + canonical Action field
        ↓ where an explicit binding is warranted
native ActionRef / native owner / native authority
        ↓
Invocation / Activity / Evidence / Return
        ↓
M5 Epii Recognition / governed return
```

QL-MEF owns the generalized Vāk registry/refraction and QL-native formal operations. It does **not** own Epi source/Bimba truth, O:I's global application registry, AIKit Context/Method resolution, Actuation Agency/Stream/Return, Workcell materialisation, or another product's Action handler/authority.

The first runtime must therefore make all current source entries addressable while allowing zero native Action bindings. Real Action bindings are explicit provenance-bearing relations rather than generated name matches.

This line also clarifies C′/VĀK in `EPI-HOLOGRAPHIC-KERNEL-ORIENTATION.md`: low-level callable primitives do not become Vāk merely by being functions, while source-level Vāk can now receive a genuine runtime identity and can bind an executable manifestation where the semantic relation is established.

## Next work

1. continue execution in QL-MEF #94 against the ratified R3 field (`EPI-CAPABILITY-READINESS-LEDGER.md`), without reopening the 36 capability identities or the Ta-Onta = S′ lock.

Still-open R2 derivation under #25, from current live target-platform state while consuming the newer Vāk runtime determination where relevant:

2. consume the accepted R1 machine inventory and any explicitly newer source-pinned capability matrix used by a relation;
3. derive the Epi M/M' internal relation dataset;
4. derive M/M'↔S/S' embodiment/authority relations;
5. verify live O:I/native-product contracts and derive the cross-map;
6. derive the explicit Bimba bridge map;
7. materialise the source-pinned Vāk registry/action-binding line according to `EPI-VAK-AGENT-NATIVE-RUNTIME.md` rather than creating a parallel runtime ontology;
8. only then derive bounded migration/reuse/supersession pressures.
