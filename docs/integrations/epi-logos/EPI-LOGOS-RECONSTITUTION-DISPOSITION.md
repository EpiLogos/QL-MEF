# Epi-Logos Reconstitution / Migration Disposition

**Status:** working migration law  
**Parent:** QL-MEF #40 / #45  
**Wayfinder:** #30  
**Source inventory:** #38

This document defines what it means to preserve Epi-Logos while rebuilding its software architecture.

The continuity target is **not** Theia, a CLI executable name, a Rust gateway, a static C FFI boundary or the current Pratibimba shell.

The continuity target is the system's formal/computational substrate, source authority, domain semantics, lived instrument functions, agent constitution, personal authorship law and return relations.

---

## 1. Default stance

> **Rebuild the thing in the right native architecture; preserve/port functions and semantic/computational substance; retire historical shells when parity is demonstrated.**

Historical architecture often contains responsibilities that were necessary before O:I, Central, Actuation, AIKit, Factory and Workcell existed in their present forms.

That historical fact is valuable evidence. It is not a command to carry every old boundary forward.

---

## 2. Disposition vocabulary

### `retain-domain`
The semantic/computational content remains authoritative to Epi/QL-MEF.

Examples:
- Bimba source meaning;
- QL/MEF/harmonic derivations;
- M0–M5 operators/datasets;
- M′ instrument semantics;
- VAK/CF/CFP;
- Nara/Epii domain law.

Implementation may still be rewritten.

### `operate-through`
An Epi concept remains meaningful but uses a native O:I/suite primitive for generic mechanics.

Examples:
- Epi Agent through Actuation Agent/Agency;
- M′ instrument through AIKit Components/Surfaces;
- Pleroma through AIKit capabilities;
- Aletheia through Factory Evidence/Actuation Return.

### `delegate-generic`
Historical Epi code owns generic mechanics that now have a native owner and should converge there.

Examples:
- generic session/harness/provider selection;
- generic process/service lifecycle;
- generic Run/evidence/autoresearch orchestration;
- generic component/surface composition.

### `adapt-transport`
The transport/provider/store/FFI/host may change while the semantic contract remains.

Examples:
- SpaceTimeDB bridge;
- graph store;
- static C FFI;
- audio provider;
- local process protocol.

### `rebuild-shell`
The current application/UI/CLI shell is an implementation specimen, not a preservation target.

Examples:
- Theia;
- current Pratibimba app shell where native composition is better;
- dashboard/pane taxonomy;
- broad `epi` CLI wrapper.

### `candidate-contract`
A recurring relation may be reusable, but must be proved as extension/provider/integration before root ontology.

Examples:
- `TemporalCondition`;
- four-seam inhabitation contribution.

### `unresolved-authority`
Do not migrate/canonicalise yet because two source/package lineages overlap and provenance must decide the boundary.

Examples:
- exact M1/M2 ↔ QL-MEF operator ownership;
- structural A/B/C/D relation names versus retained musical derivation.

---

## 3. Preservation hierarchy

When architectural pressures conflict, preserve in this order unless an explicit human/source decision says otherwise:

1. **source authority and provenance**;
2. **formal/computational meaning and invariants**;
3. **data/dataset authority and derivation**;
4. **human authorship/privacy/return law**;
5. **domain instrument functions and cross-domain relations**;
6. **canonical Agent identity/constitution**;
7. **observable feature/function behaviour**;
8. **native O:I integration contract**;
9. **provider/transport compatibility**;
10. **historical package/framework/layout/executable identity**.

This is why a shell may be replaced while a mathematically small lookup table may be highly protected: the latter may carry the actual formal source.

---

## 4. Shell dispositions

## 4.1 Theia

**Disposition:** `rebuild-shell`.

The Theia application is frozen historical evidence.

Preserve by extraction:

- M0′–M5′ instrument functionality;
- integrated 1-2-3 / 4-5-0 compositions;
- command/action vocabulary;
- useful layout/interaction affordances;
- kernel/bridge/readiness behaviour;
- agent-control use cases;
- acceptance fixtures.

Do not preserve by default:

- Theia extension/plugin identity;
- pane IDs merely because they existed;
- framework-specific lifecycle/state mechanisms;
- bespoke component protocol superseded by AIKit native composition.

Retire when the migration receipt proves the required functions exist in the new application.

---

## 4.2 Current Pratibimba application

**Disposition:** `rebuild-shell` with higher evidential weight than Theia because it is the real current app form.

It must be inspected for current behaviour before replacement.

A rewrite may change:

- framework;
- page/component structure;
- layout/navigation;
- local state plumbing;
- hosting arrangement.

A rewrite must not silently lose:

- instrument affordances;
- cross-M state flow;
- authoring/interaction capability;
- source/provenance visibility;
- agent-addressable operation;
- privacy/return behaviour;
- computational access.

---

## 4.3 `epi` CLI

**Disposition:** `rebuild-shell` / `delegate-generic` / optional thin adapter.

No requirement exists to retain a standalone `epi` executable.

Preferred end states, in order of semantic need:

1. native operation exists through common Action and `oi`/AIKit surface — no `epi` command needed;
2. `epi` is a thin ergonomic alias/passthrough into those native Actions;
3. a small domain CLI remains for genuinely Epi/QL-MEF computation, diagnostics or offline use.

Forbidden end state:

> a second generic session/capability/provider/runtime stack retained solely to make the old CLI continue working internally.

CLI parity is about **operations available**, not binary-name continuity.

---

## 4.4 S3 gateway / session store

**Disposition:** mixed `delegate-generic` + `adapt-transport` + `retain-domain` payload.

### Generic responsibilities to converge

- session/run registry;
- model/provider overrides;
- process/chat lifecycle;
- generic subscription/provider plumbing;
- cmux/session-space binding;
- generic readiness/state transport.

### Likely native owners

- Actuation — Agent/Agency/world determination;
- AIKit — SessionSpace, Context, models/harnesses/providers/surfaces;
- Workcell — material processes/services;
- O:I — shared-field/projection transport where relevant.

### Epi state/meaning to preserve

- Day/NOW;
- VAK address/condition;
- Kairos/Chronos state;
- Epi coordinate/Bimba bindings;
- Graphiti/Epi episodic semantic links where still part of domain architecture;
- Epi privacy/projection metadata.

The old gateway may disappear once native owners can carry those payloads with parity.

---

## 4.5 `epi-lib`

**Disposition:** `retain-domain`; package/language adaptable.

The C body contains high-value formal computation and source data.

May change:

- C → Rust or other implementation;
- static globals → generated/versioned registries;
- monolith → multiple libraries;
- CLI harness;
- FFI boundary.

Must preserve or explicitly revise through source governance:

- formal operators;
- dimensions/cardinalities;
- Bimba/coordinate invariants;
- M1/M2/M3 transforms;
- Nara/Epii implemented domain state;
- kernel ratios/energy/tick semantics;
- data provenance.

---

## 4.6 `portal-core`

**Disposition:** `retain-domain` computational asset; package boundary adaptable.

Reconcile it with C and canonical QL-MEF rather than maintaining duplicated drifting formal algorithms.

Modules such as harmonic profile, Hopf, codon/rotation, Mahamaya and kernel projections should become explicit owners/consumers after #31/#33 decide authority.

---

## 4.7 Epii autoresearch/coding bodies

**Disposition:** split.

`delegate-generic`:
- generic coding-agent orchestration;
- generic experiment/run lifecycle;
- generic evidence/artifact management.

`retain-domain`:
- six-Epi-domain evaluation;
- Logos pedagogy;
- source/canon interpretation;
- Epi return/rehearing/crystallisation semantics.

Factory is the generic development substrate. Epii remains the Epi recursive/pedagogical instrument.

---

## 5. Computational migration receipt

Every operator/table/kernel moved or rewritten must carry:

```text
id / semantic name
old exact source path + symbol
source/derivation ref
old implementation status
new package/path + symbol/API
inputs / outputs
invariants/cardinalities
old fixtures/tests
new fixtures/tests
numeric/operator parity result
provenance parity result
known intentional changes
consumers migrated
research status preserved
retirement status
```

Compilation is not parity.

---

## 6. Data migration receipt

Every authoritative data table/registry must be classified:

```text
AUTHORED-CANONICAL-DATA
DETERMINISTICALLY-DERIVABLE
IMPLEMENTATION-FIXTURE/CACHE
PLACEHOLDER/STUB
RESEARCH-DATA/PROPOSITION
```

Then decide:

- versioned source data;
- deterministic generator + fixtures;
- discard/recreate cache;
- refuse promotion of stub;
- preserve research status.

A constant living in `.rodata` does not automatically make it canonical source data.

---

## 7. Feature/function migration receipt

For each historical application feature:

```text
feature
human value / why it exists
historical source/surface
underlying domain computation
required Context/Knowledge
required Actions/Capabilities
native Component/Surface destination
agent operation path
privacy/authority
return/proposal path
parity test or acceptance scenario
intentional UX difference
```

This prevents two failure modes:

1. preserving obsolete UI architecture because nobody separated function from shell;
2. producing a cleaner new UI that quietly omits the system's unusual capabilities.

---

## 8. M′ instrument migration law

The six M′ domains are rebuilt as **DomainInstruments** over native capabilities.

The invariant is not a file tree. It is the domain relation.

### M0′
Bimba becomes playable/addressable while source authority remains external to the surface.

### M1′
Mathematical relation becomes audible/playable with coordinate/source recoverability.

### M2′
MEF/harmonic/cymatic field becomes an actual instrument and transduction surface.

### M3′
Upstream harmonic/formal state becomes symbolic/time transcription.

### M4′
Personal source, condition, activity, episode, reflection and return become one protected lived field.

### M5′
The system becomes teachable/inspectable/recursive without turning generic development into a private parallel Factory.

Native Components/Surfaces/Actions can all change while these invariants remain.

---

## 9. O:I integration migration law

When a historical Epi structure overlaps an O:I primitive:

```text
old mixed Epi structure
        ↓ classify
[Epi semantic payload] + [generic mechanism]
        ↓
Epi payload operates through native mechanism
```

Example:

```text
historical gateway session record
  day_id
  vak_address
  active_agent
  model_override
  cmux_pane
  process child

becomes approximately

Epi Temporal/VAK contribution
  + Actuation Agent/Agency refs
  + AIKit SessionSpace/model/provider refs
  + Workcell material process refs
```

Do not reproduce the old struct one-for-one in a new package if its fields now belong to independent native owners.

---

## 10. QL-MEF canonicalisation law

A formal operator/table may become canonical QL-MEF package authority when it is sufficiently generic and proved.

That move must preserve genealogy:

```text
Epi source / M1 or M2 origin
      ↓ provenance
QL-MEF promoted operator/registry
      ↓ consumed back by
Epi M/M′ + other O:I applications
```

Canonicalisation does not mean:

- deleting its Epi source meaning;
- moving all associated correspondences into QL-MEF;
- changing Epi instrument semantics;
- declaring the original source obsolete merely because code was generalised.

See #31/#39/#49.

---

## 11. Readiness and degradation

A rebuilt capability must not report one undifferentiated green status.

Minimum readiness dimensions:

```text
source/version
operator/computation
data/provenance
native dependency
material provider
instrument/render
privacy/disclosure
return/promotion path
historical parity
```

### Failure law

- required semantic dependency absent → hard/not-ready;
- optional instrument/provider absent → explicit degraded state;
- fallback changes semantic meaning → visible and provenance-bearing;
- silent semantic fallback → forbidden.

Historical S3's explicit fallback discipline is worth preserving as behaviour even if the gateway disappears.

---

## 12. Human-authority migration law

No technical convergence may collapse source classes.

Particularly for Nara:

```text
computed state
    != authored identity source
observation
    != inference
inference
    != adopted belief/source
proposal
    != mutation
formal return
    != human acceptance
```

A migration that makes a write technically easier while weakening this law is a regression even if all tests compile.

---

## 13. Projection migration law

The new application can expose Epi worlds through O:I Projection/shared field.

But migration must preserve:

```text
native source object
local Epi Pratibimba state
projection revision
WorldPresentation/render state
```

as independently addressable/provenanced things.

Do not convert “the app is now projection-native” into “shared projection is the only native source representation.”

---

## 14. Agent migration law

The six canonical Epi Agents survive as identities/constitutions, not as bespoke runtime classes.

Migrate:

```text
historical agent config/runtime
        ↓
Actuation Agent identity
+ Epi-authored Agent constitution
+ AIKit Profile/SkillSet/Context
+ native model/harness/SessionSpace
+ Workcell body
+ Factory Run/Evidence when needed
```

The names and Epi domain poles are semantically load-bearing. The old session wrapper is not.

---

## 15. Historical-behaviour acceptance scenarios

Before deleting a major old shell, prove scenarios spanning the unusual parts of Epi, not only boot/navigation.

At minimum include:

1. inspect Bimba source and traverse M0′ without mutating source;
2. traverse a QL/MEF relation and recover its source/operator;
3. produce/play an M1 harmonic trajectory with formal coordinates;
4. pass M1/M2 state through the M2′ harmonic/cymatic bus;
5. transcribe relevant M2 state into M3 symbolic/time state;
6. enter protected Nara state, generate an inference/proposal and refuse silent source mutation;
7. operate a canonical Epi Agent through Actuation/AIKit;
8. run a developmental Epi task through Factory and inspect returned evidence in Epii;
9. expose a selected Epi world as O:I Projection without confusing it with local Pratibimba;
10. degrade a provider and show accurate readiness/fallback provenance.

These are cross-layer continuity tests for the **system**, not the shell.

---

## 16. Retirement decision table

| Historical body | Default | Can retire when |
|---|---|---|
| Theia | rebuild-shell | instrument/function receipts complete |
| current Pratibimba shell | rebuild-shell | current behaviour inventoried + new parent/instrument parity |
| `epi` CLI | optional adapter | all needed Actions/domain commands reachable elsewhere |
| S3 gateway | delegate/adapt | native session/provider/process/projection parity proven |
| bespoke capability registries | delegate-generic | AIKit composition covers semantics and gates |
| bespoke agent runtime | delegate-generic | six Agents run natively through Actuation/AIKit |
| C static FFI | adapt-transport | rewritten kernel parity/provenance proven |
| C/Rust domain operators | retain-domain | only replaced by proved canonical implementations |
| historical QL/MEF tables | unresolved/retain | provenance reconciliation + canonical registry promotion |
| Epii generic autoresearch | delegate-generic | Factory supplies equivalent generic mechanics |
| Epii pedagogy/return | retain-domain | never retired without explicit source redesign |

---

## 17. Anti-patterns

### “Port the repo”
Wrong because historical repository topology is not the product architecture.

### “Delete old stuff because O:I has equivalents”
Wrong because the old body may mix generic mechanics with unique Epi semantics/data.

### “Keep the CLI/app because it already works”
Wrong when this creates a second authority and blocks native composition.

### “Rewrite everything from the specs”
Wrong because current code contains implementation facts, fixtures, invariants and edge behaviour not recoverable from vision alone.

### “Treat all C tables as canon”
Wrong because some are derived, fixtures or explicit placeholders/stubs.

### “Move every M1/M2 thing into QL-MEF”
Wrong because formal generalisation does not transfer all Epi semantic/correspondential ownership.

### “Make public projection the new source”
Wrong because Projection is selective representation, not source authority.

---

## 18. Definition of successful reconstitution

A successful migration leaves us able to say:

- the historical shell can disappear without losing an Epi capability;
- the new application is natively composed through current O:I products;
- QL-MEF is stronger and more independently executable than before;
- M0–M5 computation has explicit provenance and parity;
- M′ instruments are more coherent and agent/human addressable;
- Nara's human-source boundary is stricter, not weaker;
- Epii uses Factory rather than duplicating it while retaining Epi recursion/pedagogy;
- shared Epi worlds project cleanly into O:I without source confusion;
- every retired body has a receipt;
- every intentionally changed behaviour has a reason tied to product meaning.

The continuity is the **relation and capability of the living Epi world**, not the accidental architecture of its previous containers.
