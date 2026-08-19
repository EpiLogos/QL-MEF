# Epi Bimba ↔ Pratibimba coordinate-parity protocol

**Status:** canonical development protocol / Wayfinder guard for the current QL-MEF product programme  
**Domain source owner:** Epi-Logos Bimba corpus  
**Programme / protocol home:** QL-MEF  
**Primary Bimba relational source pool:** `EpiLogos/Epi-Logos-C-Experiments/Idea/Bimba/Map/`  
**Implementation bodies:** Epi `portal-core` / `epi-lib` and current O:I-hosted Pratibimba consumers

## Why this is a programme-level guard

The first lived M4′ Nara vertical exposed an architectural distinction which applies to all subsequent Epi instrument work.

A useful vertical can be semantically too narrow even when its feature works. Prompt B correctly built the M4′ daily journal, DAY/NOW, protected persistence, profile context and selected-context sendoff, but the actual Bimba Map is more articulated and relational than that first operational slice.

The correction is not to render the full map in the UI. It is to make the map addressable and testable underneath the instruments so later capabilities have somewhere truthful to attach.

```text
Bimba Map relational pool
        ↓ exact source identities + relations + provenance
QL-MEF source reader / normalization / index
        ↓
QL-MEF granular coordinate + relation core
        ↓ same-path M ↔ M′ reflection
Epi executable operational bindings where required
        ↓
objects / events / Actions carry coordinate lineage
        ↓
O:I Surface / instrument
```

A feature list is not an acceptable substitute for the Bimba relational shape.

## Source organisation — corrected and compiled 2026-08-19

The primary Bimba relational data is present under:

```text
Idea/Bimba/Map/
└─ datasets/
   ├─ hashtag_node_data.md
   ├─ deep-property-map.md
   ├─ low-detail/
   │  ├─ nodes_anuttara.json
   │  ├─ nodes_paramasiva.json
   │  ├─ nodes_parashakti.json
   │  ├─ nodes_mahamaya.json
   │  ├─ nodes_nara.json
   │  ├─ nodes_epii.json
   │  ├─ corresponding relation pools
   │  ├─ foundation/hash pools
   │  └─ Parashakti straggler/migration residues
   ├─ anuttara-deep/
   ├─ paramasiva-deep/
   ├─ parashakti-deep/
   ├─ mahamaya-deep/
   ├─ nara-deep/
   │  ├─ nodes-full-detail.json
   │  └─ relations.json
   └─ epii-deep/
```

The `Seeds/M/**` specifications remain important authored interpretation/design sources. They are not a replacement for reading `Idea/Bimba/Map/**` where the Map has granular coordinates, properties and relations.

The earlier programme note saying the Nara deep machine corpus was absent was wrong: the files exist under `Idea/Bimba/Map/datasets/nara-deep/`. PRE-D was therefore source-pool reading, normalization, provenance classification, parity compilation and executable/conformance binding, not source recovery.

The exact PRE-D source lock is:

```text
repository: EpiLogos/Epi-Logos-C-Experiments
revision:   daa660cbc1b8c5da83828698665a753852cb0287
Map root:   Idea/Bimba/Map
Map tree:   cd4f4f77c13f27e2563c5a6753d2f8bf2b605f15
```

`data/epi-bimba-map/source-lock.json` pins the declared source bodies. `data/epi-bimba-map/returned-reality.json` records the accepted PRE-D result. The compiler/conformance artifacts retain per-file Git blob identity, SHA-256 and per-record payload digests.

## Returned source reality

The source reader must follow the corpus rather than force the corpus into the first reader's assumptions.

Final PRE-D compilation returned:

```text
57 recursively inspected source files
3,748 coordinate-bearing source records
1,875 distinct M0–M5 source coordinates
21,083 source relation records
1,381 source relation kinds
2,676 cross-M source relations
```

Whole-M coordinate counts:

| Root | Coordinates |
| --- | ---: |
| M0 Anuttara | 108 |
| M1 Paramasiva | 43 |
| M2 Parashakti | 597 |
| M3 Mahamaya | 996 |
| M4 Nara | 100 |
| M5 Epii | 31 |

Three representation facts were returned by the source itself:

1. M-coordinate spelling uses `.`, `-` **and** `/`; exact spelling is retained.
2. `#` and `#-0 … #-5` are rootless meta-field records. They are explicit external/meta refs, not malformed M0–M5 coordinates.
3. eight equal numeric paths have multiple source spellings. They remain explicit alternate-notation groups; QL-MEF does not silently choose one canonical spelling.

The historical exports also contain raw control characters in some committed JSON bodies. The live reader therefore uses a tolerant representation parser while retaining exact file/provenance hashes. Final accepted JSON loss is zero and unclassified coordinate parse failure count is zero.

The Map also contains 450 source relations with partial/meta endpoints: 400 null endpoints and 50 rootless meta endpoints. Their stable relation identity, source kind, orientation/provenance and known endpoint are retained. QL-MEF does not invent the missing endpoint to make a graph look complete.

These are **source representation facts**, not universal QL semantics.

## Knowledge ownership

```text
Epi-Logos-C-Experiments
  source corpus + historical/domain implementation evidence
  Idea/Bimba/Map relational pool
  Epi-owned executable/domain implementation where still current

QL-MEF
  current product programme knowledge
  source normalization / granular coordinate+relation representation
  Bimba↔Pratibimba parity / reflection
  conformance / returned-reality artifacts
  Wayfinder / build order / continuation ground

O:I
  current application host / consumer contracts

Central
  human Project source + NOW/DAY + explicit accepted return where crossed
```

This does not make QL-MEF the semantic author of Bimba. QL-MEF owns the current formal/parity machinery which can carry source-owned Bimba meaning without silently universalising it.

## The parity vocabulary

Future tickets and PRs must distinguish:

| Level | Meaning |
| --- | --- |
| **source parity** | accepted Bimba source pool and revision are identified and actually read |
| **coordinate parity** | every in-scope coordinate has a stable Ref preserving source identity, root, recursive path, parent/alternate notation and face |
| **relation parity** | source relation kind, endpoints, direction/orientation and provenance remain resolvable, including explicit partial records |
| **operational parity** | implemented behavior actually enacts the coordinate/relation it claims |
| **experiential parity** | the human/agent instrument expresses the right consequence without requiring architectural chrome |

Do not write simply “parity complete” where only one level is demonstrated.

## Structural existence is not implementation

This distinction is executable in the QL-MEF `MMapIndex` / `ImplementationBinding` floor:

```text
coordinate exists
    != capability implemented
    != provider available
    != currently rendered Surface
    != currently disclosed to an Agent
```

A source coordinate remains structurally addressable when no capability binds it. A provider binding cannot create Bimba source identity by being operational.

## Relation classes must not collapse

PRE-D returned a second critical distinction:

```text
Bimba source relation
!= QL-derived/formal relation
!= M↔M′ reflection relation
!= implementation dependency/flow
!= runtime/event relation
!= research candidate
```

The hand-authored Nara floor uses useful operational/formal relation kinds such as `Contains`, `ConjugateReflects`, `AnchorsAt`, `SuppliesEvidenceTo` and `GovernedReturnTo`. These do **not** become Bimba source relation kinds merely because the implementation is valid.

Likewise, a Prompt-C packet path such as `selection → review → ground → proposal/return` is implementation choreography. It must not be written back as a Bimba source relation unless the Map relation index separately supplies that relation.

## Required build order for an M′ domain

Before substantial feature work in Mx′:

1. read the relevant `Idea/Bimba/Map/**` pool, not only Seed prose;
2. inventory the source coordinates and relation records actually present;
3. retain aliases/alternate notation/properties without changing source identity;
4. materialise/update the QL-MEF source index and parity/conformance representation;
5. bind/update the Epi executable coordinate/relation representation where an operational M′ body requires it;
6. prove source-count/content/relation conformance for the declared scope;
7. map current native substrate to those coordinates without treating implementation resemblance as source authority;
8. bind the bounded M′ capability to its exact coordinate lineage;
9. build the Surface/instrument;
10. test operational behavior and experienced use separately.

A Factory/coding agent must not jump from prose spec to feature implementation and create a local semantic enum as the de facto subsystem architecture.

## Structural invariants

The following are acceptance blockers:

- an accepted Bimba coordinate silently disappears because no provider exists;
- a reflected M′ coordinate changes the Bimba path instead of changing only face;
- recursive depth is flattened into a convenient shallower service/UI coordinate;
- a UI/component route becomes canonical semantic identity;
- an Action or mutation bypasses an authored review/return relation;
- an M′ module invents an incompatible coordinate namespace;
- coordinate existence is reported as implemented capability;
- provider availability is reported as semantic authority;
- Seed prose substitutes for the Map pool where Map records exist;
- generated summaries replace source relation records without retained provenance;
- alternate source notation is silently collapsed;
- a missing source relation endpoint is filled by inference;
- implementation relation kinds are reported as Bimba source relations.

## Nara proving case — returned reality

Epi PR #14 established a hand-authored 44-coordinate operational floor in `portal-core`. PRE-D compared it exhaustively against the actual live M4 source pool.

Returned result:

```text
M4 Map coordinates:                         100
current executable Nara floor:              44
exact source ↔ implementation matches:      44
source coordinates absent executable floor: 56
implementation coordinates without source:  0
implemented notation/alias mismatches:        0
implemented parentage mismatches:             0
Map max recursive depth:                      4
implementation max recursive depth:           3
```

Therefore the 44-node floor is neither Bimba authority nor an error. It is an **exact source-backed operational subset**. The additional 56 M4 coordinates remain structurally present/queryable underneath the lived product without forcing expansion of the quiet Nara Surface.

The current lived path remains:

```text
#4.4 / M4-4′
  → protected DAY episode / exact selection
  → #4.4.4.4 personal carrier
  → #4.5 review seam
```

Prompt-C reconciliation then preserves the same exact `NaraCoordinateBinding` through Epii review, Anuttara ground and proposal. Epii review/proposal carry source-derived M5/M5′ root ground; Anuttara ground carries source-derived M0/M0′ root ground. Packet choreography is explicitly marked implementation-flow rather than Bimba-source relation. Central source authority and Nara privacy are unchanged.

## Whole-M / Cosmic readiness returned by PRE-D

Prompt D must begin from the Map-derived source field, not from three local application ontologies.

The compiled M1/M2/M3 ground is:

| Root | Coordinates | Max depth | Outgoing source relations |
| --- | ---: | ---: | ---: |
| M1 Paramasiva | 43 | 7 | 469 |
| M2 Parashakti | 597 | 7 | 8,038 |
| M3 Mahamaya | 996 | 5 | 9,784 |

Across the three roots:

```text
M1/M2/M3 coordinates:                     1,636
relations wholly inside M1/M2/M3:       16,408
cross Cosmic → other M:                   1,505
cross other M → Cosmic:                   1,011
harmonic/temporal/transcription relations 3,665
```

This proves **source-ground readiness**, not application readiness. Prompt D must inspect current provider/implementation readiness separately.

The authored integrated Cosmic architecture still gives the intended composition relation: one shared K² surface, one shared `MathemeHarmonicProfile` tick, and M1/M2/M3 as superposed constitutive poles (`137 = 64 + 72 + 1`) rather than three panes. PRE-D supplies the real coordinate/relation ground underneath that design; it does not prove every historical Cosmic implementation still exists or works.

## QL-MEF's role

QL-MEF owns the organised parity/conformance machinery:

- index the Bimba relational pool;
- provide stable source references and revisions;
- normalize source records without erasing provenance;
- distinguish structural fact, authored interpretation, implementation binding and research inference;
- define/test M↔M′ coordinate and relation parity;
- expose the smallest general operators that have genuinely become QL/MEF concerns;
- keep Epi-specific semantic payload visibly source-owned.

The current general substrate lives in `crates/ql-mef/src/m_map.rs`; the deterministic compiler/conformance path lives under `scripts/compile-epi-bimba-map*.py` and `data/epi-bimba-map/**`.

O:I remains the host/consumer. Epi code remains an implementation body where domain-specific execution belongs.

## Required ticket shape

Every substantial M0′–M5′ ticket must include **Coordinate / Relation Ground**:

```text
Bimba Map source scope:
Source revision(s):
Coordinate pool/ref:
Relation pool/ref:
QL-MEF parity/conformance ref:
Epi executable binding/ref where required:
Current substrate bindings:
Implementation/readiness state:
Unbound/research coordinates:
Vertical origin coordinate:
Carrier coordinate(s):
Review/return coordinate(s):
Parity acceptance:
```

For integrated instruments such as Personal 4/5/0 or Cosmic 1/2/3, each participant domain must arrive already rooted. The integrated instrument composes coordinates; it does not retrofit semantic ownership after feature code exists.

## Acceptance evidence

The PRE-D QL-MEF head `46f83ed10866ab4391ec25c70c9c16c78281158e` was green on:

- QL-MEF Rust run `32274940738`;
- QL-MEF Rust Verify run `32274940731`;
- Pre-local release run `32274940702`;
- Epi Bimba Map conformance run `32274940726`.

The conformance artifact was `9373903781`, digest `sha256:8c5cd719aeec52c913f5d3591284d0dc96d70328a48e6eed552283fb2ad7697d`.

Subsequent documentation/returned-reality commits must rerun the same branch gates before PRE-D is treated as final live state.

## Continuation law

The parity gate is an interlude in the original implementation chain, not a new competing chain.

```text
Prompt A — primitive bridge
Prompt B — M4′ Nara lived vertical
Prompt C — Personal 4/5/0 lived return
PRE-D — compile the actual Bimba Map pool and reconcile parity
Prompt D — M1′ + M2′ + M3′ as ONE integrated Cosmic instrument
```

PRE-D must finish by emitting a complete ready-to-run **Prompt D** generated from the actual returned live state. Prompt D must not say “build M1 next”, must not create three dashboard/local ontologies, and must consume the same Map-derived relational field and the same lived Nara/Personal world.