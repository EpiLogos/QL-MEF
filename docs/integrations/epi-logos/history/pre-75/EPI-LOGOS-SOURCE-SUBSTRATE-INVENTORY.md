# Epi-Logos Source / Substrate Inventory

**Status:** working source-of-truth inventory  
**Parent:** QL-MEF #38  
**Wayfinder:** #30  
**Architecture:** #25 / PR #27

This document inventories the sources and executable bodies that matter to Epi-Logos reconstitution. Its purpose is not to preserve repository layout. Its purpose is to preserve **authority, semantic content, implemented behaviour, data provenance and migration evidence** while allowing obsolete shells and transport architecture to be rebuilt.

A path listed as exact has been directly verified in the current research pass. A source family listed without an exact path is intentionally marked for verification in #38 rather than guessed.

---

## 1. Inventory record schema

Every item that becomes implementation work should eventually have this record:

```text
id
human meaning / why it exists
source class
source-theoretic provenance
exact source path(s)
current implementation path(s)
semantic owner
current implementation status
inputs / outputs
invariants / cardinalities
tests / fixtures
downstream consumers
native O:I dependencies
migration disposition
parity evidence required
open authority questions
```

Source classes:

- `AUTHORED-SOURCE` — theory/canon/vision/source position;
- `ACCEPTED-ARCHITECTURE` — reviewed system/design commitment;
- `IMPLEMENTATION-FACT` — actual code/tests that run now;
- `HISTORICAL-SHELL` — application/runtime specimen whose framework identity is not authoritative;
- `RESEARCH-PROPOSITION` — proposition not established merely by code/spec;
- `MIGRATION-INFERENCE` — current architectural judgment about where responsibility should live.

---

# PART I — FORMAL / AUTHORED SOURCES

## 2. M / M′ parent system

### Exact verified source

`EpiLogos/Epi-Logos-C-Experiments`

```text
Idea/Bimba/Seeds/M/M'-SYSTEM-SPEC.md
```

### Authority
`AUTHORED-SOURCE / ACCEPTED-ARCHITECTURE` for the relation:

```text
M  = Bimba / canonical coordinate image
M′ = Pratibimba / lived-reflected operation of that image
```

It establishes:

- M′ as lived/visual/sonic/operational Pratibimba, not a floating UI family;
- S/S′ as bodies/protocols rather than the semantic definition of M′;
- the non-numbered 0/1 parent field;
- Cosmic composition M1′+M2′+M3′;
- Personal composition M4′+M5′+M0′;
- six full-depth M′ workspaces in the 4+2 relation;
- the cross-cutting command membrane.

### Migration law
Never infer that replacing a UI shell replaces M′ semantics.

---

## 3. Bimba / coordinate authority

### Source family
Epi Bimba / Idea / Seeds corpus, plus the accepted R1 source snapshot under Epi-Logos-C-Experiments #3.

### Exact implementation-related sources already verified

```text
Body/S/S0/epi-lib/src/psychoid_numbers.c
Body/S/S0/epi-lib/src/pointer_web.c
Body/S/S0/epi-lib/src/qv_data.c
Body/S/S0/epi-lib/src/main.c
```

### Authority split

- Bimba source/canonical topology remains Epi-owned.
- Interactive graph representations are Pratibimba affordances, not topology authority.
- QL-MEF Meta-Knowledge Graph is not Bimba.
- AIKit Knowledge graphs/indexes are not Bimba.
- O:I Projection is not Bimba.

### Migration disposition
`retain-domain` + explicit `bridge`/provenance to generic graph/index capabilities.

---

## 4. Retained coordinate / lens / harmonic sources

### Retained source artifacts consumed in this programme

The current research pass has retained and cross-read:

```text
epi_logos_coordinate_system.md
epi_logos_cheat_sheet.md
mef-12-lenses-sublens-reference.md
ql-musical-derivation-v3.md
```

These are source/research artifacts used to reconstruct the formal genealogy. Their eventual canonical repository locations and supersession relationships must be verified/ratified in #38/#31 rather than inferred from upload names.

### Principal structures carried by these sources

- generative `#` parent and 0/1 relation;
- P/P′ / Day-Night′ conjugate structure;
- 12-lens MEF field, six base + six conjugate/inverted positions;
- complementary lens axes 0↔5, 1↔4, 2↔3;
- three V4/lens-square structures;
- seven Context Frames;
- 12 × 7 = 84 lens-mode / mode-tonic field;
- formal 4+2 / return relations;
- foundational musical ratios including 16/9, 9/8, 4/3, 3/2 and octave closure;
- two whole-tone/generator bases;
- harmonic pairing families;
- 3×3 square structures;
- diatonic / modal / major-minor derivations;
- coordinate-to-pitch recoverability;
- voice-leading/rhythm/process relations;
- later microtonal/cymatic propositions.

### Status discipline
Not every retained derivation is an accepted executable QL-MEF fact. #31/#39/#49 must promote structures through provenance + tests.

---

## 5. QL-MEF accepted current canon

### Repository
`EpiLogos/QL-MEF`

### Key accepted sources

```text
docs/Q1-DETERMINISTIC-KERNEL.md
docs/Q2-MEF-REGISTRY.md
docs/VISUAL-PRODUCT-UNDERSTANDING.md
README.md
```

### Authority
Current accepted implementation/design floor for deterministic QL relation operations, MEF registry/refraction, product understanding and package boundary.

### Important current limitation
`Q2-MEF-REGISTRY.md` deliberately excludes musical assignments/deeper harmonic structures from **current accepted Q2 scope**. This is not the final architectural boundary of QL-MEF.

### Related ticket authority

- #7 — accepted relation/conjugation/MEF ownership and Bimba distinction;
- #31 — develop fuller formal/harmonic/music substrate;
- #39 — reconcile structural vs retained musical relation-family semantics;
- #49 — promote proven harmonic structures.

---

# PART II — EXECUTABLE EPI COMPUTATION

## 6. Common C substrate — `epi-lib`

### Exact root

```text
Body/S/S0/epi-lib/
```

### Verified implementation files

```text
Body/S/S0/epi-lib/src/arena.c
Body/S/S0/epi-lib/src/engine.c
Body/S/S0/epi-lib/src/families.c
Body/S/S0/epi-lib/src/kernel.c
Body/S/S0/epi-lib/src/m0.c
Body/S/S0/epi-lib/src/m1.c
Body/S/S0/epi-lib/src/m2.c
Body/S/S0/epi-lib/src/m3.c
Body/S/S0/epi-lib/src/m3_clock_lut.c
Body/S/S0/epi-lib/src/m4.c
Body/S/S0/epi-lib/src/m5.c
Body/S/S0/epi-lib/src/main.c
Body/S/S0/epi-lib/src/pointer_web.c
Body/S/S0/epi-lib/src/psychoid_numbers.c
Body/S/S0/epi-lib/src/qv_data.c
```

Includes/docs/tests also exist beneath `include/`, `docs/`, `test/` and must be enumerated at symbol/test level by #38/#33.

### Current implementation fact
`main.c` verifies the static Bimba/coordinate web, canonical flags, #5→#0 Möbius return, #4 lemniscate anchor and CF roots; creates mutable mirrors; instantiates six families; initializes and verifies M0 through M5.

### Disposition
`retain-domain`. C as language and static FFI are not sacred; behaviour, source data and formal invariants are.

---

## 7. Cross-cutting bioquaternionic kernel

### Exact source

```text
Body/S/S0/epi-lib/src/kernel.c
```

### Current implementation facts

- bioquaternion state `Qb / Qp`;
- epogdoon ratio/log;
- ascending/descending fourth and fifth ratios;
- conjugate/slash-flip relation;
- 72-dimensional resonance indexing over lens × helix × position;
- tritone-square emphasis;
- energy evaluation across Bimba-Pratibimba, lens and R-energy terms;
- 12-stage descent/ascent tick with harmonic ratio and QL position.

### Owner
Epi/QL-MEF formal computational substrate. Exact future package boundary between generic QL-MEF operators and Epi-specific bioquaternion state remains #31/#33 work.

### Required parity
Numeric fixtures, dimension/cardinality checks, round-trip/provenance and downstream M′ recovery.

---

## 8. M0 — Anuttara computation

### Exact source

```text
Body/S/S0/epi-lib/src/m0.c
```

### Current implementation facts

Contains source-backed/static structures for:

- Vimarśā operator table;
- virtue table;
- zodiacal/Vāk operators;
- divine-act table;
- 12-fold archetype/number structures;
- Bimba/registry/cross-branch relations.

### Important implementation-state caveat
The file itself describes some intended VM/runtime material as future. Do not convert comments/placeholder structure into an implementation claim.

### Disposition
`retain-domain`; reconcile formal operators with QL-MEF where genuinely generalisable.

---

## 9. M1 — Paramasiva computation

### Exact source

```text
Body/S/S0/epi-lib/src/m1.c
```

### Current implementation facts

- Ananda Bimba 12×12 matrix;
- Ananda Pratibimba matrix;
- synthesis/sum matrix;
- quintessence structures;
- dual digital-root ring structures;
- QL branch-category table;
- M0 crosslink table;
- Spanda contextual substage structures.

### Genealogical significance
`M1-4` is the Quaternal Logic flowering in the Epi source lineage. Generalising formal operators into QL-MEF must preserve this provenance while avoiding duplicate canonical implementations.

### Disposition
`retain-domain` + `unresolved-authority` at exact QL-MEF extraction boundary until #31/#39 resolves it.

---

## 10. M2 — Parashakti computation

### Exact source

```text
Body/S/S0/epi-lib/src/m2.c
```

### Current implementation facts

- 72-byte/72-position vibrational space;
- 12 MEF lens identities and descriptors;
- base and inverted/conjugate lens indexing;
- tattva descriptors;
- additional correspondential tables/structures.

### Fuller specified destination
The active M2/M2′ source additionally carries the fuller Vimarśā audio-genesis / lens-resonance / nodal-cymatic instrument. #38/#33 must mark each item as implemented, partial or specified rather than treating the current C file as the whole M2 destination.

### Genealogical significance
`M2-1` carries MEF/Vimarśā genealogy. QL-MEF package authority should absorb/generalise the formal substrate where appropriate while Epi-specific correspondence/cymatic semantics remain Epi-owned.

### Disposition
`retain-domain` + selective QL-MEF canonicalisation.

---

## 11. M3 — Mahamaya computation

### Exact sources

```text
Body/S/S0/epi-lib/src/m3.c
Body/S/S0/epi-lib/src/m3_clock_lut.c
```

### Current implementation facts

- dinucleotide/pair matrix;
- trigrams;
- 64 hexagram/codon structures;
- non-dual codon sets;
- complement/movement/resonance operators;
- codon/amino-acid and symbolic mappings;
- rotation/clock lookup material.

### Fuller domain relation
M3 receives/transcribes upstream M2/DET/harmonic evidence into symbolic/time structures. It must not independently become an alternate M2 audio/cymatic authority.

### Disposition
`retain-domain`; provider/render shells may change.

---

## 12. M4 — Nara computation

### Exact source

```text
Body/S/S0/epi-lib/src/m4.c
```

### Current implementation facts

- real M4 root and HC link;
- lens registry shape;
- protocol-library shape;
- alchemical operation table shape;
- MEF thresholds;
- voice configuration;
- container structures;
- safety state;
- consent-gated randomness.

### Critical completeness caveat
Several lens translation/activation/annotation functions, protocol values and alchemical operations are explicit stubs/placeholders.

Do not migrate those placeholders as if they were canonical domain content.

### Fuller architecture/source family
The richer M4/Nara architecture contains stable identity, baseline, current event/transit, activity/pattern, composed lived context and integration/return. Exact canonical source path(s) for those architecture documents are to be verified and pinned in #38 rather than guessed here.

### Disposition
`retain-domain`, with Central/AIKit/Workcell integration around it.

---

## 13. M5 — Epii computation

### Exact source

```text
Body/S/S0/epi-lib/src/m5.c
```

### Current implementation facts

- six Logos stage names;
- M/L/P/S/T/C Quintessential View lookup;
- formal pipeline tick;
- guarded Möbius return into M0 model state;
- Epi agent/stack/theory state.

### Authority caveat
Formal model mutation is not authority to mutate Central Control, Nara identity source or Epi/QL-MEF canon.

### Disposition
`retain-domain`; generic development execution delegates to Factory.

---

## 14. Rust computational substrate — `portal-core`

### Exact root

```text
Body/S/S0/portal-core/
```

### Verified source modules

```text
Body/S/S0/portal-core/src/aspect.rs
Body/S/S0/portal-core/src/codon.rs
Body/S/S0/portal-core/src/codon_rotation_projection.rs
Body/S/S0/portal-core/src/events.rs
Body/S/S0/portal-core/src/harmonic_profile.rs
Body/S/S0/portal-core/src/hopf.rs
Body/S/S0/portal-core/src/kernel.rs
Body/S/S0/portal-core/src/lib.rs
Body/S/S0/portal-core/src/mahamaya.rs
```

Additional modules exist and must be enumerated by #38.

### Interpretation
Computational/domain asset, not evidence that the historical portal shell/package boundary must survive.

### Required work
Reconcile overlap with C and QL-MEF, choose canonical generated/data/operator owners, and eliminate drift through shared fixtures/APIs.

---

# PART III — HISTORICAL / CURRENT RUNTIME BODIES

## 15. Rust S3 gateway

### Exact root

```text
Body/S/S3/gateway/
```

### Verified modules

```text
Body/S/S3/gateway/src/bootstrap.rs
Body/S/S3/gateway/src/chat.rs
Body/S/S3/gateway/src/dispatch.rs
Body/S/S3/gateway/src/lib.rs
Body/S/S3/gateway/src/protocol.rs
Body/S/S3/gateway/src/runtime.rs
Body/S/S3/gateway/src/session_store.rs
Body/S/S3/gateway/src/sessions.rs
Body/S/S3/gateway/src/spacetime.rs
```

### Other verified S3 bodies

```text
Body/S/S3/epi-app/
Body/S/S3/epi-spacetime-module/
Body/S/S3/gateway-contract/
Body/S/S3/graphiti-runtime/
Body/S/S3/redis-context/
```

### Current implementation facts
`runtime.rs` owns run/snapshot/listener/chat/subscription machinery. Subscription records carry Epi and generic state such as session, agent, scope, Day, NOW path, Graphiti refs, privacy and projection source.

`session_store.rs` carries session identity, Day/NOW, runtime cwd/vault root, active agent, subagent lineage, model/provider overrides, delivery/channel/team/orchestration/cmux state and VAK address.

`spacetime.rs` owns native WS/fallback/readiness/resync/subscription behaviour and explicitly refuses silent semantic fallback.

### Migration interpretation
This is the clearest mixed-responsibility specimen.

Generic mechanics likely converge onto:

- Actuation — Agent/Agency/session-world determination where applicable;
- AIKit — SessionSpace, context, model/harness/provider/surface composition;
- Workcell — processes/services/material providers;
- O:I — cross-world projection/shared-field transport.

Epi-specific state such as Day/NOW/VAK/Kairos/Bimba/Graphiti semantic bindings remains Epi provider/binding payload.

### Disposition
`delegate-generic` + `adapt-transport`; retirement only after #40/#45 parity receipts.

---

## 16. Epii autoresearch / return body

### Verified root family

```text
Body/S/S5/epii-autoresearch-core/
```

Exact subfile inventory remains #38 work.

### Current research finding
The existing body consumes the Aletheia return wire and evaluates the six Epi subsystems; this is Epi-specific evaluation/return meaning, not merely generic coding automation.

### Migration interpretation

- generic coding/autoresearch/Run/Evidence orchestration → Factory/AIKit;
- six-domain Epi evaluation, Logos pedagogy and canon return → Epii/Epi domain.

### Disposition
split `delegate-generic` / `retain-domain`.

---

# PART IV — APPLICATION / INSTRUMENT EVIDENCE

## 17. Frozen Theia application

### Exact root

```text
Body/M/epi-theia/extensions/
```

### Verified extension directories

```text
acceptance-harness/
agentic-control-room/
body-lite-surface/
contracts/
ide-shell-m0-m5/
integrated-composition/
kernel-bridge-readiness/
kernel-bridge/
m-extension-runtime/
m0-anuttara/
m1-paramasiva/
m1-paramasiva-played-torus/
m2-parashakti/
m3-mahamaya/
m4-nara/
m5-epii/
omnipanel-shell/
plugin-integrated-1-2-3/
plugin-integrated-4-5-0/
pratibimba-layouts/
```

### Status
`HISTORICAL-SHELL`. Frozen. Important for feature/function and interaction archaeology, not a preservation target.

### What to extract

- instrument command/action vocabulary;
- pane/surface relationships;
- integrated Cosmic 1-2-3 composition;
- integrated Personal 4-5-0 composition;
- kernel readiness/bridge assumptions;
- agent control functions;
- historical layout/interaction affordances worth preserving;
- acceptance fixtures.

### Disposition
`rebuild-shell`.

---

## 18. Current Pratibimba application

### Status
Current real application form according to the programme's authored/current state, but framework/package details were not re-enumerated in this inventory pass.

### Required #38 action
Locate and pin exact current application paths/head, then inventory features against #34/#36.

### Authority
Important implementation/evidence source, **not** a preservation mandate.

### Disposition
`rebuild-shell` unless a native component is already correct and reusable.

---

## 19. `epi` CLI

### Status
Existing Rust native Epi command surface with typed access to Epi computational bodies.

### Migration law
No preservation requirement.

Possible final states:

1. removed where native `oi`/AIKit/Actions already cover the operation;
2. thin passthrough/alias to native Actions;
3. small Epi-domain CLI for genuine QL/Epi computation and diagnostics.

It must not preserve duplicate generic session/capability/provider/runtime authority.

### Required #38/#40 action
Pin exact current CLI root/files and produce a command-by-command migration receipt.

---

# PART V — M′ INSTRUMENT SOURCE/FUNCTION INVENTORY

## 20. M0′ Anuttara

### Historical implementation evidence

```text
Body/M/epi-theia/extensions/m0-anuttara/
```

### Functions to preserve/rebuild

- playable Bimba graph/field;
- Epi coordinate navigation;
- language / QL / relation / time-community / personal-route / pedagogy-route layers;
- source/spec/code/test anchors;
- provenance/readiness disclosure;
- routing to Nara/Epii;
- no graph-edit authority over canonical Bimba absent explicit source transaction.

---

## 21. M1′ Paramasiva

### Historical evidence

```text
Body/M/epi-theia/extensions/m1-paramasiva/
Body/M/epi-theia/extensions/m1-paramasiva-played-torus/
```

### Functions to preserve/rebuild

- M1 as playable mathematical-musical instrument;
- relation=interval, position=pitch, traversal=phrase;
- canonical / instance / Ananda / Spanda / QL flowering / toroidal strata;
- 84 lens-mode landscape;
- source/derivation inspection;
- canonical QL-MEF operator consumption;
- M2 audio/nodal bus consumption;
- torus/Hopf/SU(2)/4π expression where supported.

---

## 22. M2′ Parashakti

### Historical evidence

```text
Body/M/epi-theia/extensions/m2-parashakti/
```

### Functions to preserve/rebuild

- full 72-fold harmonic/MEF/correspondential instrument;
- Vimarśā audio genesis / resonance;
- `audio_octet[8] + nodal_quartet[4]` or proven successor;
- standing-wave / Chladni / cymatic rendering;
- source-authoritative correspondential domains;
- 8+4 architecture;
- 72→64 bridge into M3;
- distinction between generic QL-MEF harmonic operators and Epi semantic/correspondential payload.

---

## 23. M3′ Mahamaya

### Historical evidence

```text
Body/M/epi-theia/extensions/m3-mahamaya/
```

### Functions to preserve/rebuild

- Paramasiva tick / harmonic-state reception;
- M2 72→64 transduction reception;
- 64 symbolic/codon/I-Ching/Tarot field;
- 384 line-change and 472 rotation account where source-canonical;
- double-torus/world-clock experience;
- 84 lens-mode projection;
- traceability from visible symbol to computation/source.

---

## 24. M4′ Nara

### Historical evidence

```text
Body/M/epi-theia/extensions/m4-nara/
```

### Functions to preserve/rebuild

- protected personal world;
- stable identity vs baseline/current/activity/pattern distinction;
- journal / DAY / NOW / dream / oracle / highlight / episode continuity;
- M1/M2/M3 integration at personal point;
- composed lived state;
- explicit proposal → review → accept/reject → apply gate;
- protected episodic memory;
- strict disclosure/privacy boundaries;
- selected shared projection without source-authority transfer.

---

## 25. M5′ Epii

### Historical evidence

```text
Body/M/epi-theia/extensions/m5-epii/
```

### Functions to preserve/rebuild

- Epi teaching / source explanation;
- canon/source recognition;
- Logos return inspection;
- six-subsystem evaluation;
- domain computation inspection/test/run;
- Logos Atelier / system self-articulation;
- Epi proposal/return review;
- generic dev delegated through Factory where possible.

---

## 26. Parent compositions

### Historical evidence

```text
Body/M/epi-theia/extensions/plugin-integrated-1-2-3/
Body/M/epi-theia/extensions/plugin-integrated-4-5-0/
Body/M/epi-theia/extensions/integrated-composition/
Body/M/epi-theia/extensions/ide-shell-m0-m5/
Body/M/epi-theia/extensions/pratibimba-layouts/
```

### Functions to recover

- shared state/coordinate propagation;
- Cosmic M1′→M2′→M3′ relation;
- Personal M4′↔M5′↔M0′ relation;
- command/agent membrane;
- full-depth navigation;
- readiness/bridge handling;
- provenance and source inspection.

Framework/layout identity is not authoritative.

---

# PART VI — O:I / SUITE TARGET SOURCES

## 27. O:I whole / positions

### Exact current authored entrypoint

```text
EpiLogos/O-I/docs/positions/FOUNDING-POSITIONS.md
```

### Use for
Whole/product meanings, source/projection/participant/field distinctions, objective internality and product boundary reasoning.

---

## 28. Central

### Current semantic owner
Human-owned durable source/operative ground, source classes and accepted durable mutation.

### Key source families already used in R2

```text
CENTRAL-SYSTEM-SPEC.md
CONTROL-CONTENT-PROTOCOL.md
```

Exact current paths/head should be re-pinned by implementation sessions because the repository is moving rapidly.

### Epi relation
Central does not become Nara; it supplies/receives human-authorised ground.

---

## 29. Actuation

### Current semantic owner
Agent identity, Agency, WorldBinding, situated determination/bounds, Return, metagency/composition.

### Epi relation
Six canonical Epi Agents use these generic semantics; VAK/CF remain Epi constitution, not generic Actuation ontology.

---

## 30. AIKit

### Current semantic owner
Context, Knowledge, Profile, SkillSet, Capability, model/harness, SessionSpace, Component/Contribution/Surface and operative composition.

### Epi relation
Primary native host/composition layer for rebuilt Epi application/instruments and optional CLI passthrough.

`AIKit Profile` remains distinct from Epi MathemeProfile/Nara identity models.

---

## 31. Software Factory

### Current canonical document family
The accepted/current Factory docs include:

```text
docs/canon/QL-SOFTWARE-FACTORY-ARCHITECTURE-SPEC.md
docs/canon/QL-SOFTWARE-FACTORY-PRIMITIVE-RELATIONS.md
docs/canon/QL-SOFTWARE-FACTORY-DEEP-QL-INTEGRATION-FOUNDATIONS.md
docs/canon/QL-SOFTWARE-FACTORY-WORKCELL-MODULE-SPEC.md
```

### Epi relation
Generic Project/Commission/Run/Artifact/Evidence/Candidate/Recognition and development traversal. Epii retains Epi-specific pedagogy/evaluation.

---

## 32. Workcell

### Current semantic owner
Material provider/process/service/store/execution body and observed material state.

### Epi relation
Materialises graph/audio/render/process/model/service needs without becoming semantic owner of Epi computation or full Context.

---

## 33. O:I shared field / Explore

### Current developmental relation
O:I Projection / Participant / Contribution / Encounter and `WorldPresentation` work provide the external/shared representation path.

### Epi law

```text
native Epi source/world
   ≠ local Pratibimba instrument
   ↓ explicit projection transaction
O:I Projection revision
   ↓
WorldPresentation / shared rendering
```

Never use public/shared representation as source authority.

---

# PART VII — MIGRATION PRIORITIES

## 34. High-value assets to protect first

1. formal and harmonic derivations/provenance;
2. Bimba/coordinate authority;
3. C/Rust M0–M5 operators/tables/invariants;
4. M1/M2/M3 cross-domain buses/transductions;
5. Nara source/privacy/promotion law;
6. Epii Logos/return/evaluation semantics;
7. six canonical Agent constitutions;
8. M′ instrument functions;
9. Ta-Onta inhabitation semantics;
10. parent Cosmic/Personal composition.

## 35. Low-preservation-value identities

These are useful only insofar as they carry functions not yet recovered elsewhere:

- Theia framework/plugin identity;
- old dashboard/layout structures;
- bespoke generic session/capability/provider managers;
- gateway transport implementation;
- static FFI shape;
- `epi` executable name;
- duplicated command surfaces;
- historical package boundaries that only reflect missing O:I infrastructure.

## 36. Retirement gate

A historical body may be retired only when a receipt names:

```text
source/body
functions/features extracted
semantic owner for each
successor Action/Component/provider/operator
state/provenance migration
structural/operator/instrument parity
intentional differences
remaining shim or rollback path
```

See #40 and #45.
