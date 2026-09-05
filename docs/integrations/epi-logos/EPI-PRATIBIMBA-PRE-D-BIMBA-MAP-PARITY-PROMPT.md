# EPI / PRATIBIMBA — PRE-D BIMBA MAP PARITY COMPILATION

This is a **gating interlude in the original A → B → C → D Pratibimba implementation chain**.

It does not replace Prompt D. It exists so Prompt D begins from the actual granular Bimba relational structure rather than another hand-authored feature approximation.

Primary ticket: QL-MEF #63  
Protocol: `EPI-BIMBA-PRATIBIMBA-COORDINATE-PARITY.md`  
Ticket ground template: `EPI-M-PRIME-TICKET-GROUND-TEMPLATE.md`

## Work directly in the actual current live state

**PRIMARY PRODUCT / PROGRAMME OWNER**  
https://github.com/EpiLogos/QL-MEF

**BIMBA SOURCE CORPUS + CURRENT EPI DOMAIN IMPLEMENTATION BODY**  
https://github.com/EpiLogos/Epi-Logos-C-Experiments

**CURRENT APPLICATION HOST / CONSUMER**  
https://github.com/EpiLogos/O-I

**CURRENT HUMAN-SOURCE / TEMPORAL RETURN OWNER — ONLY WHERE EXISTING PERSONAL RETURN CROSSES IT**  
https://github.com/EpiLogos/Central

**AGENCY / DEVELOPMENT SUPPORT — READ OR MODIFY ONLY WHERE A REAL OWNER CONTRACT REQUIRES IT**  
https://github.com/EpiLogos/ai-kit  
https://github.com/EpiLogos/Actuation  
https://github.com/EpiLogos/Workcell  
https://github.com/EpiLogos/agent-system-design

## Before editing

Reinspect live state. Do not trust prompt-time SHAs, mergeability, CI state or branch ancestry.

Read in QL-MEF:

- #30 — living Epi Wayfinder;
- #62 — Bimba↔Pratibimba coordinate/relation parity gate;
- #63 — this PRE-D source-compilation ticket;
- #64 — ticket-ground retrofit;
- current PR #65 and its actual branch;
- current Prompt-C returned-reality PR #61;
- `docs/integrations/epi-logos/EPI-LOGOS-DEVELOPMENT-WAYFINDER.md`;
- `docs/integrations/epi-logos/EPI-BIMBA-PRATIBIMBA-COORDINATE-PARITY.md`;
- `docs/integrations/epi-logos/EPI-M-PRIME-TICKET-GROUND-TEMPLATE.md`;
- current source/substrate inventory and relation-field artifacts;
- current QL-MEF core/library code relevant to coordinate, relation, pairing, Wiki/refraction and MEF identity.

Read the live implementation PRs before changing their assumptions:

- Epi Prompt-B / Nara lineage;
- Epi Prompt-C Personal 4/5/0 provider;
- O:I Prompt-B Nara lived vertical;
- O:I Prompt-C Personal return;
- O:I coordinate-parity consumer;
- Central Personal/NOW return only where crossed.

## Source law — the Bimba Map is already there

The granular Bimba source pool is:

`EpiLogos/Epi-Logos-C-Experiments/Idea/Bimba/Map/`

Do not repeat the false claim that the Nara deep corpus is missing.

Recursively inspect the actual live tree. At minimum include:

```text
Idea/Bimba/Map/datasets/hashtag_node_data.md
Idea/Bimba/Map/datasets/deep-property-map.md
Idea/Bimba/Map/datasets/low-detail/**
Idea/Bimba/Map/datasets/nara-deep/nodes-full-detail.json
Idea/Bimba/Map/datasets/nara-deep/relations.json
Idea/Bimba/Map/datasets/anuttara-deep/**
Idea/Bimba/Map/datasets/mahamaya-deep/**
Idea/Bimba/Map/datasets/epii-deep/**
all actual live Paramasiva / Parashakti / cross-domain node/relation pools
```

Treat migration scripts/export helpers as representation provenance, not semantic authority by themselves.

Then read the relevant `Idea/Bimba/Seeds/M/**` specs as interpretive/design context. **The Seed specs do not replace the granular Map pool.**

## Goal

Make the **whole granular M/Bimba relational field legible to and reflectable by the QL-MEF core**, so every subsequent M′ instrument is rooted in the actual source coordinates/relations rather than in whatever feature-local ontology a coding session happens to invent.

The intended relation is:

```text
BIMBA MAP SOURCE POOL
coordinates + properties + relations + provenance
        ↓
QL-MEF SOURCE NORMALISATION / INDEX
        ↓
QL-MEF GRANULAR COORDINATE + RELATION CORE
        ↓
exact M ↔ M′ coordinate reflection / parity
        ↓
EPI OPERATIONAL BINDINGS
        ↓
O:I-HOSTED PRATIBIMBA INSTRUMENTS
```

QL-MEF is the primary product destination for the general coordinate/relation/reflection/parity machinery and the durable programme knowledge.

Epi remains source owner for Bimba semantic material and an implementation body where Epi-specific execution remains native.

O:I remains host/consumer.

## Do the source work first

Build a deterministic source inventory before changing the core.

For every relevant source pool/file, record enough provenance to answer:

```text
repository revision
source path
content hash / stable source revision
record class
coordinate identity
parent / recursive path
semantic/property payload class
relation kind
relation endpoints
relation direction/orientation
cross-M status
aliases / alternate notation
source confidence/status if present
```

Do not flatten all properties into QL canon. Preserve source-owned payload and provenance.

Distinguish at least:

```text
source structural fact
source semantic/property payload
authored interpretive/design statement
implementation binding
research/inference
migration/export representation
```

## Build the QL-MEF granular coordinate substrate

Implement the smallest general substrate that can represent the actual Map rather than the existing 44-node Nara hand model.

It must support arbitrary recursive coordinate depth actually present in source.

It must preserve the distinction:

```text
coordinate exists
!= capability implemented
!= provider available
!= visible UI
```

The core should support stable identities equivalent in function to:

```text
MCoordinate
  canonical source identity
  M root
  full granular path
  face = Bimba | Pratibimba
  parent
  source provenance

MRelation
  stable relation identity
  typed/source relation kind
  from / to
  direction/orientation
  source provenance

ImplementationBinding
  coordinate/ref
  owner/provider
  implementation/readiness state
  evidence/provenance
```

Do not hard-code the Map as hundreds of hand-written Rust enum variants if a source-driven indexed/generated representation is the truthful design.

## Make M ↔ M′ reflection exact

For every source coordinate included in the declared parity scope:

```text
root(Mx...) == root(Mx...′)
path(Mx...) == path(Mx...′)
face differs
source identity remains traceable
```

Reflection must not alter parentage, recursive depth or relation provenance.

If source notation and current implementation notation differ, maintain explicit aliases/resolution rather than silently choosing one and losing the other.

## Nara is the first exact conformance proof

Compare the actual M4/Nara Map against the current Epi coordinate-parity implementation.

The existing Epi PR #14 44-node structural floor is **implementation evidence**, not source authority.

Prove and report:

- which of its coordinates are exact source matches;
- source coordinates it omitted;
- implementation coordinates lacking source basis, if any;
- alias/notation discrepancies;
- parentage/recursive discrepancies;
- relation-kind/end-point discrepancies;
- properties that belong to source payload rather than QL core;
- implementation-only relations which are legitimate operational bindings rather than Bimba source relations.

Then revise/migrate the Nara operational bindings so the already-working daily episode and selection packets retain **source-derived/source-conformant coordinate lineage**.

Do not redesign the quiet Nara UX merely because the substrate becomes more complete.

## Reconcile Personal 4/5/0

Prompt C has already produced useful lived-return behavior.

Preserve it while rooting it more exactly:

```text
M4′ Nara source selection
        ↓
M5′ Epii review/proposal
        ↓
M0′ Anuttara/Bimba ground
        ↓
reviewed/owner-governed return
```

The Epii and Anuttara/Bimba packets/actions must use the same source-derived coordinate/relation substrate rather than named local packets standing in for their M5/M0 relational ground.

Do not change Central's authored/observed/inferred/derived/proposal/accepted-mutation law.

Do not create another session/runtime.

## Prepare the whole M field for Prompt D

This tranche is **not** deep M0′–M5′ implementation.

But the whole Map should be indexed deeply enough that the next Cosmic work cannot make the same mistake.

Before closing PRE-D, ensure the QL-MEF substrate can identify the actual granular source ground for:

```text
M0 / Anuttara
M1 / Paramasiva
M2 / Parashakti
M3 / Mahamaya
M4 / Nara
M5 / Epii
```

and relation records crossing those domains.

M1/M2/M3 need enough source-ground readiness that Prompt D can compose **one Cosmic instrument** from rooted domains instead of inventing three local dashboards.

## Knowledge organisation

Keep durable new programme knowledge in QL-MEF.

Prefer updating the existing:

```text
EPI-BIMBA-PRATIBIMBA-COORDINATE-PARITY.md
EPI-M-PRIME-TICKET-GROUND-TEMPLATE.md
EPI-LOGOS-DEVELOPMENT-WAYFINDER.md
EPI-LOGOS-SOURCE-SUBSTRATE-INVENTORY.md
existing relation-field / machine guard artifacts
implementation prompt chain
```

rather than creating duplicate Wayfinders or another Epi-only protocol document.

Epi source files should be changed only where the source itself or implementation-specific provenance requires it.

## Acceptance

The tranche is complete only when:

1. the actual `Idea/Bimba/Map/**` pool has been recursively inventoried on a recorded live revision;
2. QL-MEF has a data-driven granular coordinate/relation representation adequate to the declared whole-M source scope;
3. source coordinate and relation records can be resolved with provenance;
4. exact source counts/content/relation conformance can be run for declared pools;
5. recursive paths and cross-M relations survive normalization;
6. M↔M′ reflection preserves exact coordinate identity;
7. implementation/readiness is separate from coordinate existence;
8. Nara's existing operational coordinate floor has been compared to source and reconciled;
9. Prompt-C Personal 4/5/0 retains working privacy/authority/UX while consuming source-conformant lineage;
10. M1/M2/M3 source ground is ready for one integrated Cosmic instrument;
11. QL-MEF Wayfinder/tickets/protocols reflect returned reality;
12. no new programme knowledge exists only in the historical Epi repo;
13. CI/conformance proves the real source→QL-MEF→Epi/O:I path touched by the work.

## Mandatory final deliverable — continue the original prompt chain

This session MUST end by outputting a complete ready-to-run prompt titled:

# EPI / PRATIBIMBA D — M1′ + M2′ + M3′ INTEGRATED COSMIC INSTRUMENT

Generate it **after** the PRE-D work so it reflects returned reality.

The generated Prompt D must:

- begin from actual live state and current PR/issue topology;
- name QL-MEF as the structural/formal product owner for the Map-derived coordinate/relation core;
- use Epi `Idea/Bimba/Map/**` as the source ground for M1/M2/M3;
- consume the exact PRE-D manifests/APIs/refs that now exist;
- build M1′+M2′+M3′ as **one integrated Cosmic instrument**;
- feed the same Nara/Personal world and shared Matheme/harmonic/temporal state;
- preserve O:I host, AIKit/Actuation agency, Central source-authority and Nara privacy boundaries;
- prohibit three dashboard-style local ontologies;
- require human/agent co-reference to exact source-rooted Cosmic objects;
- include acceptance and returned-reality requirements;
- specify the next continuation prompt/handoff the D session itself must generate based on what D reveals.

Do not conclude with “next, do Prompt D.” The full Prompt D text is an acceptance artifact.
