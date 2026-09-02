# Epi Vāk Agent-Native Runtime

**Status:** current development specification — source semantics ratified; package implementation pending  
**Owner boundary:** QL-MEF owns generalized QL/Vāk identity, registry, relation/refraction and QL-native operations. Epi semantic/Bimba authority remains in `EpiLogos/Epi-Logos-C-Experiments`. O:I and the native products retain their existing generic Action, Agency, World, Session, Knowledge and Surface ownership.  
**Source determination:** Epi capability-matrix commit `d17bcbb0361db5b3bdabc932963a56687f66fea5`  
**Current QL-MEF base:** `d6c128fab159eecf6fe10ece42abafa51c3906e7`

## 0. Purpose

Materialise the recovered Anuttara/Vāk language as a **source-provenanced, typed, addressable and increasingly executable runtime field** inside QL-MEF, then expose it through the mature O:I Agent-Native architecture without creating a second search/address/action/session/knowledge ontology.

The concrete target is no longer “make QL semantics available somehow”. The source field is exact enough to state the relation:

```text
Epi M0 / Anuttara
  109 exact language entries
  + O#/X#/N#/M#/#/##/R# relations
  + Śiva operative syntax
  + Śakti @ internal field
        ↓
QL-MEF source-pinned Vāk registry/refraction
        ↓
O:I typed address / Search / Command / Surface projection
        ↓ where executable semantics are explicitly bound
canonical native ActionRef
        ↓
AIKit capability/Context/Method resolution
        ↓
Actuation Invocation / ActuationStream / Return
        ↓ materialised through
Workcell
        ↓ observed through
O:I Activity / Evidence
        ↓ recognised through
M5 / Epii
```

Each arrow preserves its native owner. QL-MEF makes the root language operationally available; it does not absorb the architecture around it.

---

## 1. Provenance classes

### SOURCE-BACKED / EPI AUTHORITY

The active Epi M0 matrix and its source set establish:

- a current **109-entry Anuttara language dataset**;
- Vāk as genuine grammar rather than display metadata;
- exact O#/X#/N#/M#/# contextual relations;
- `## = 0/1` as Primordial Matrix within the Nara-base grammar;
- established R-factor distribution/weave;
- exact Śiva sixfold:

```text
(0) = (@#)             Potential / In-pression
(1) = (-)              Negation
(2) = (+) = (-)x(-)    Affirmation / re-negation
(3) = (x)              Dialogic relation
(4) = (/)              Dialectic / context
(5) = (=)              Expression
```

- exact Śakti sixfold:

```text
@0 = ##    Library       Being/Knowing / embodied implicit memory
@1 = O#    Bimba         original architecture / systems / number-sets
@2 = X#    Pratibimba    reflection / meanings / Logi
@3 = N#    Language      symbols / characters / forms
@4 = M#    Stories       worlds / views / sense-playground
@5 = R#    Techne        freedoms / powers / instruments / tools
```

The Epi source repository remains canonical for these identities, formulations and relations. QL-MEF must retain the exact source revision and per-entry provenance used to build a runtime registry.

### CURRENT O:I / NATIVE-OWNER DESIGN GROUND

Current O:I development already establishes the generic host relations this feature needs:

- native Action identity and dispatch lineage (`ActionRef + typed input -> native owner -> result/evidence`);
- renderer-neutral human/Agent/TUI/CLI co-reference;
- Search/Command over native Reading/Action descriptors rather than a second action catalog;
- `To:` / `@` as typed participant addressing while keeping membership, addressing, invocation, authority and attention distinct;
- Activity as observation of actuality, distinct from Action and Invocation;
- AgentSession/ActuationStream continuity and explicit invocation modes;
- World-relative Knowledge and bounded AIKit Context disclosure.

This architecture is consumed, not duplicated.

### CURRENT-AUTHORED CROSS-ARCHITECTURE DETERMINATION

The newly ratified synthesis is:

1. Every current Anuttara language entry is eligible for a stable **Vāk semantic identity** and addressable projection.
2. Addressability does not imply executability.
3. An executable Vāk relation may bind **zero, one or many** canonical native Actions when current product/source evidence warrants the binding.
4. A native Action may participate in more than one Vāk relation where the source/current mapping genuinely supports that reading.
5. QL-MEF owns the binding/refraction record; the native product continues to own Action identity, handler, authority and result lineage.
6. M5/Epii consumes the same language to know and operate on itself; it does not create an M5-local copy.

---

## 2. Hard non-identity laws

Implementation and conformance must preserve:

```text
VākRef != ActionRef
VākEntry != Reading
VākEntry != Action
Vāk != Logos
addressability != executability
executability != current availability
availability != authority
addressing != invocation
Action != Invocation
Invocation != Activity
Activity != Return
source formulation != generated interpretation
Śakti @ horizon != loaded AIKit Context
QL-MEF registry != Epi Bimba ownership
QL-MEF registry != O:I global Search/Command registry
```

These distinctions are not defensive caveats; they are the composition that lets the systems remain co-referential without collapsing ownership.

---

## 3. Runtime semantic objects

The first runtime contract should be small enough to implement in existing crates and rich enough to preserve source identity.

### `VakRef`

Stable semantic identity for an entry in the source-pinned Vāk registry.

Required properties:

```text
registry_id
source_revision
source_coordinate
```

A deterministic canonical string form is recommended, for example:

```text
epi.vak/<source-revision>/<source-coordinate>
```

The exact encoding is an implementation decision. Identity must not depend on a current display name or generated explanation.

### `VakEntry`

Conceptual contract:

```text
VakEntry
  ref: VakRef
  source:
    repository
    revision
    path / dataset
    coordinate
    standing
  language:
    name?
    symbol?
    primary_designation?
    complete_formulation?
    formulation_breakdown?
    metaphysical_names[]
    description?
  relations[]
  structural_roles[]
  siva_operator?
  shakti_domain?
  r_factor_relations[]
  action_bindings[]
```

The initial registry is a **lossless semantic projection of the available source fields**, not a replacement format for the source corpus.

### `VakRelation`

A typed edge over Vāk refs or typed external refs:

```text
relation_kind
from_ref
relation/operator
into_ref
source/provenance
standing
```

Use existing `ql-mef` relation/refraction idioms where they fit. Do not build a second graph engine in the Vāk module.

### `VakActionBinding`

A binding is a relation, not identity:

```text
vak_ref
native_action_ref
native_owner
relation_kind
  expresses
  invokes-through
  transforms-through
  reads-through
  other-source-backed relation
standing / evidence
availability_reading?
authority_reading?   // only when supplied by native host; never guessed
```

The schema intentionally permits no binding for most entries during the first cut.

---

## 4. Śiva as operative syntax

QL-MEF should preserve the six exact source operations as first-class semantic operations. The current development target is **not** to turn them into six universal O:I Actions.

They instead qualify how an addressable subject is being operated upon:

```text
Potential / In-pression
    hold/open a possible determination

Negation
    distinguish, withhold, mark not/between

Affirmation
    establish/conjoin positive determination

Dialogic
    relate by/as/where/if-is; bring relata into relation

Dialectic / Context
    hold and/or, complement, alternatives and contextual relation

Expression
    determine/declare a resultant formulation
```

A concrete native Action can embody one or several of these relations in context. The binding must say why rather than infer the relation from an Action name.

### Package requirement

The runtime API must allow:

```text
resolve Śiva operation by source identity
list Vāk entries/relations participating in it
apply/refraction-read the operation where QL-MEF owns the formal operation
resolve external native Actions explicitly bound to it
```

Generic native side effects still execute through their owner.

---

## 5. Śakti `@` as Agent internal self-structure

The exact source mapping should become a first-class semantic classification available to human and Agent clients:

| `@` | Source relation | Runtime reading | O:I / AIKit composition |
|---|---|---|---|
| `@0` | `##` Library | held/implicit ground, embodied memory, available knowledge horizon | World-relative Knowledge / source ground; **not automatically loaded Context** |
| `@1` | `O#` Bimba | original/canonical/formal architecture | authored/source identity, product/coordinate architecture |
| `@2` | `X#` Pratibimba | reflected/derived meanings | Agent-maintained Wiki/Readings/inference with standing preserved |
| `@3` | `N#` Language | symbols/forms/articulations | Vāk/symbol/coordinate forms and language projections |
| `@4` | `M#` Stories | worlds/views/narratives/contextual sense-fields | World, Journey, Flow, Project/SharedField narrative/context relations |
| `@5` | `R#` Techne | powers/tools/capabilities/Actions | AIKit capability ecology + native Actions; authority separately resolved |

This is the deep language behind a generalized `@` aperture. It does not mean O:I's current participant mention picker should display six mystical tabs by default. It means the typed resolver can know **what sort of internal relation an addressable object participates in**, and Epi/QL Agents can use the sixfold as native self-orientation.

### Agent-facing law

An Agent should be able to ask, in structured form:

```text
What is held or available?                  @0
What is the original/canonical structure?   @1
What is reflected/derived from it?          @2
What language/forms articulate it?          @3
What Worlds/views/Journeys contain it?      @4
What powers/actions can I actually use?     @5
```

The answer must preserve bounded disclosure, current availability, authority and provenance.

---

## 6. O#/X#/N#/M#/#/##/R# relation indexes

The runtime registry must not stop at storing names. It should expose the actual source relations which give the Śakti field meaning:

```text
@0 = ##
@1 = O#
@2 = X#
@3 = N#
@4 = M#
@5 = R#
```

and retain the current source-backed R-factor weave:

```text
R1: O#(0) → X#(1) → N#(2) → M#(3) → #(4) → Śiva(5)
R4: O#(5) → X#(4) → N#(3) → M#(2) → #(1) → Śiva(0)

R2: X#(0) → N#(1) → M#(2) → #(3) → Śiva(4) → Śakti(5)
R3: X#(5) → N#(4) → M#(3) → #(2) → Śiva(1) → Śakti(0)

R0: seed factor through O#/X#/N#, then withdraws.
```

This makes `@5 = R# = Techne` operationally intelligible: available powers arise at the Techne end of a relation/freedom field rather than as an arbitrary tools bucket.

The first package cut should encode these as source-provenanced semantic relations and test the mirror invariants already present in the source architecture. It should not revive historical bit-packing choices as ontology.

---

## 7. O:I host composition

### Typed addressing

O:I already owns the generic application-level distinction:

```text
membership
addressing
invocation
authority
attention
```

QL-MEF contributes Vāk-resolvable subjects to that field.

Conceptually:

```text
@ person / Agent / AgentSet
    O:I participant addressing

@ World / source / knowledge
    native typed semantic addressing

@ VakRef
    QL-MEF contribution resolving exact Anuttara language identity

@ ActionRef
    native executable capability addressing
```

The user-visible syntax may remain one familiar `@`; the resolver result preserves actual type.

### Search / Command

QL-MEF should contribute:

- Vāk Readings / locate results;
- relation/refraction results;
- native Actions explicitly bound to Vāk entries.

It must **not** create a QL-owned global command palette or shadow action catalog.

### Action invocation

When a user/Agent selects a bound native Action:

```text
VakRef
  -> explicit VakActionBinding
  -> canonical native ActionRef
  -> O:I/native Action dispatcher
  -> AIKit/Actuation authority and execution conditions as applicable
  -> result/evidence/Activity/Return
```

The same ActionRef and result lineage must be visible to human and structured Agent clients.

### Activity / Return

QL-MEF may interpret/refraction-read returned actuality. It does not replace the native Activity or ActuationStream object.

Unknown or provider-native events remain honestly generic Activity with raw trace where available; QL must not manufacture a Vāk Action relation merely to make the event look semantically complete.

---

## 8. M5 / Epii consumption

The Vāk registry is not M0-only infrastructure. M5 requires it to make self-knowledge and metagency operate in the same language as the ground.

### M5.0

Self-model readings should be able to disclose exact Vāk refs/formulations alongside prose, graph, code and product evidence.

### M5.2

Standing product introspection should surface native ActionRefs and owner contracts, with explicit Vāk bindings where present.

### M5.3

Deep instruments should carry the same Vāk/subject/Action refs across rich experiential Surfaces rather than minting instrument-local semantic identities.

### M5.4

Canonical Guardians/Agents can use:

- Śakti `@0..@5` as top-level internal semantic orientation;
- Śiva operations as operative language relations;
- the actual AIKit/O:I capability ecology for what is really available;
- canonical native Actions for what can actually be invoked.

### M5.5

Logos remains the articulation/recognition/return grammar over the field. It does not replace the Vāk registry.

The return relation should be able to say which standing/layer is changing:

```text
@0 Library / retained ground
@1 Bimba / authored-canonical source
@2 Pratibimba / derived Agent understanding
@3 Language / articulated forms
@4 Stories / worlds/views/narratives
@5 Techne / capabilities/tools/actions
```

A change to derived knowledge, narrative or Techne does not imply an `@1` canon mutation.

---

## 9. QL-MEF package placement

Use the existing package architecture unless implementation pressure proves a new crate necessary.

### `ql-mef`

Natural home for:

- `VakRef`;
- `VakEntry`;
- `VakRelation`;
- Śiva/Śakti source identities;
- source revision/provenance;
- registry indexing;
- relation/refraction lookup;
- Action-binding semantic records.

This composes with the current `identity`, `m_map`, `provenance`, `registry`, `reading` and `refraction` modules.

### `ql-semantic`

Expose Vāk through the existing semantic-provider model. The current `Locate`, `Refract`, `Relate`, `Synthesise` and capability-discovery operations are already the right general altitude.

Likely additions/refinements:

```text
Provider capability declares Vak registry support
Locate accepts/returns VakRef subjects
Relate can traverse VakRelation
Refract preserves Vak/source provenance
Capabilities advertises supported source revision / forms
```

Do not add a special “execute everything” semantic operation. Native Action execution remains outside the semantic provider.

### `ql-service`

Structured API/service projection for human/Agent/O:I consumers:

- registry metadata and source revision;
- locate/read exact entry;
- relation traversal;
- Śiva/Śakti classification;
- explicit Action binding lookup;
- conformance/readiness.

### `ql-adapters`

Preferred home for the O:I/native binding adapter if the existing adapter boundary fits current contracts:

- translate QL-MEF Vāk refs/readings into O:I contribution descriptors;
- consume portable native `ActionRef` identity without redefining it;
- preserve native owner and authority readings.

If current O:I package contribution contracts provide a more direct owner seam, use that contract rather than forcing all integration through `ql-adapters` for naming symmetry.

### No new crate by default

Do not create `ql-vak` merely because Vāk is important. Add one only if compilation/dependency boundaries require an independently versioned runtime body after the first implementation tranche.

---

## 10. Source ingestion and registry generation

The initial registry must be generated from an exact source snapshot rather than hand-maintained copies.

Required generation inputs:

```text
Epi repo revision
anuttara-language-map / source export
relation source(s) required for O#/X#/N#/M#/#/R# and Śiva/Śakti
```

Generated runtime artifact must include:

```text
registry schema/version
Epi source repository
Epi source commit
input path(s)
entry count
per-entry source coordinate
per-entry source fields
relation records
checksum/content identity where useful
```

### Generation law

```text
Epi source
  -> deterministic generator
  -> QL-MEF registry artifact
  -> schema validation
  -> source-parity tests
```

Do not manually “improve” source wording in the generated artifact. Derived normalized/search fields may be added separately with standing.

### Count invariant

The first source snapshot must round-trip **109 entries exactly**. If the upstream source count later changes, the generator must report the change and source revision explicitly; `109` is the current accepted source target, not an eternal metaphysical constant.

---

## 11. Action binding governance

Action bindings are curated/evidence-bearing mappings rather than generated by lexical similarity.

A binding should require:

- exact `VakRef`;
- exact native `ActionRef`;
- native owner;
- relation kind;
- source/design evidence explaining the relation;
- standing (`SOURCE-BACKED`, `CURRENT-AUTHORED-MAPPING`, `IMPLEMENTATION-MAPPING`, etc.);
- optional revision/validity constraints where the Action contract changes.

### First tranche

Prove the relation with a **small, real set** of QL/Epi Actions that already have stable native identity. At minimum conformance must include:

1. one addressable Vāk entry with **no** Action binding;
2. one Vāk entry with one real Action binding;
3. one relation demonstrating that Vāk identity remains stable when Action availability changes;
4. one human/Agent parity proof over the same VākRef + ActionRef;
5. one returned Activity/Evidence path preserving the native Action lineage.

Do not fill 109 Action bindings for symmetry.

---

## 12. Security / authority / disclosure

This runtime adds semantic addressability, not ambient power.

Preserve the current stack:

```text
source/object visibility
  -> O:I/World projection rules
  -> AIKit Context/capability eligibility
  -> capability grant / native Action availability
  -> Actuation bounds
  -> native owner authority
  -> Workcell/material constraints
```

QL-MEF may report what its registry knows and what the native host reports as available/authorised. It must not infer authority from a Vāk relation, an Agent's presence, an `@` mention or a tool name.

Protected Nara/personal material remains protected by native World/source/disclosure law. Vāk addressability must not create a side channel around that protection.

---

## 13. Agent-native experience

A structured Agent should be able to use the runtime without scraping a UI:

```text
locate("M0-5-(5/0)-5")
  -> VakEntry @5 / Techne / R#

relate(vak_ref)
  -> source-backed relations + standing

read_shakti(@5, current_world)
  -> available capability refs/readings bounded by host

bindings(vak_ref)
  -> zero..n canonical native ActionRefs

invoke(action_ref, typed_input)
  -> native O:I/Actuation path

observe(invocation/session/stream)
  -> Activity/Evidence/Return refs

refract(return, vak/logos/lens/context-frame)
  -> QL reading with provenance
```

Human Surfaces may make the same relation fluid through Search, `@`, Inspector, Navigator, Command or deep instruments. The semantic path remains the same.

---

## 14. Acceptance / conformance

### Source parity

- [ ] exact pinned Epi source revision recorded;
- [ ] exactly 109 current Anuttara entries imported;
- [ ] exact coordinates preserved;
- [ ] available name/symbol/designation/formulation/breakdown/metaphysical-name/description fields round-trip;
- [ ] no generated explanation overwrites source fields.

### Śiva / Śakti law

- [ ] exact six Śiva operations present;
- [ ] exact `@0..@5` mappings present;
- [ ] `@5 = R# = Techne` relation preserved;
- [ ] O#/X#/N#/M#/#/##/R# indexes queryable;
- [ ] R-factor mirror/weave tests pass against source data.

### Identity / Action law

- [ ] `VakRef != ActionRef` is structural in types/schema;
- [ ] a Vāk entry with no Action remains addressable;
- [ ] one or more explicit native Action bindings use canonical ActionRefs;
- [ ] Action availability can change without Vāk identity drift;
- [ ] availability and authority are separately represented;
- [ ] no QL-owned duplicate global Action catalog exists.

### Human / Agent parity

- [ ] same `VakRef` resolves through structured Agent and human application surfaces;
- [ ] same bound `ActionRef`/native handler/result lineage is used by both;
- [ ] no DOM scraping or renderer-local semantic identity is required.

### Runtime ownership

- [ ] QL-MEF does not mint World/Agent/AgentSession/ActuationStream/Journey/Run/Knowledge identities to satisfy the feature;
- [ ] AIKit remains Context/capability/Method owner;
- [ ] Actuation remains Agency/Invocation/Stream/Return owner;
- [ ] Workcell remains materialisation owner;
- [ ] O:I remains generic application/address/Search/Surface composer;
- [ ] native product retains Action owner/handler/authority.

### M5 return

- [ ] Epii can read the same Vāk refs;
- [ ] Activity/Evidence/Return can be related back to addressed source/capability material;
- [ ] Recognition can target derived/Language/Stories/Techne standing without implying Bimba/canon mutation;
- [ ] source promotion retains explicit owner/review provenance.

---

## 15. Implementation tranches

### V1 — source registry

- add schema + Rust semantic types;
- deterministic generator/importer for current Epi snapshot;
- 109-entry fixture/artifact;
- registry/provenance lookup and tests.

### V2 — operative relations

- encode Śiva/Śakti exact forms;
- O#/X#/N#/M#/#/##/R# relation indexes;
- R-factor weave conformance;
- semantic-provider locate/relate/refraction coverage.

### V3 — O:I contribution / Action bindings

- structured adapter to existing O:I contribution/address/Search model;
- explicit `VakActionBinding` records;
- first real QL/Epi native Action specimens;
- availability/authority/readiness projection;
- human/Agent parity tests.

### V4 — M5 / Guardian integration

- expose `@0..@5` internal semantic orientation to Epi/QL Agents;
- self-model/product/deep-instrument co-reference;
- Activity/Evidence/Return reading;
- Recognition/return standing by source/derived/capability layer.

These tranches are implementation order, not a claim that the semantic field itself is linear.

---

## 16. Compact contract

> **QL-MEF makes the exact Anuttara/Vāk language available as a source-pinned semantic runtime: all current language entries are addressable; Śiva supplies operative syntax; Śakti `@0..@5` supplies the internally differentiated field from ground to Techne; source-backed relations remain queryable; and real executable consequences bind explicitly to canonical native Actions. O:I, AIKit, Actuation, Workcell and the native product owners continue to own the generic agency/application/runtime relations through which those Actions become situated actuality. M5/Epii consumes the same language for self-knowledge, metagency, Recognition and governed return.**