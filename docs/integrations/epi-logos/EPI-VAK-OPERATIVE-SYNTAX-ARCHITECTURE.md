# Epi Vāk Operative Syntax Architecture

**Status:** current design determination for QL-MEF #83 / PR #84 and AIKit #142  
**Source authority:** `EpiLogos/Epi-Logos-C-Experiments/Idea/Bimba/Map/datasets/anuttara-deep/anuttara-language-map.md`  
**General O:I / Agent-operative owner:** AIKit over native O:I refs and product-owned Actions  
**Full Vāk semantic owner:** QL-MEF over the source-grounded Anuttara/Bimba language  
**Related kernel owner:** QL-MEF #78 native holographic kernel

## 0. Determination

The current programme has two profiles over one foundation.

```text
PROFILE A — GENERAL O:I

stable typed refs + native product primitives
        ↓
address horizons
        +
operative relation/punctuation syntax
        ↓
Resolve / Search / Context / Method / Action
        ↓
Invocation / Activity / Return / familiarity

PROFILE B — FULL QL-MEF / VĀK

same general O:I syntax and refs
        +
exact Anuttara/Bimba Vāk identities
        +
M0-3 parse/generative self↔other language
        +
M0-4 holographic relation field
        +
M0-5 exact Śiva/Śakti semantics
        +
full 109-node traversable language
        ↓
Vāk refraction / Action profiles / Vāk paths
        ↓
M5 Recognition / return
```

The general profile is the minimal O:I form. The full QL-MEF profile is the maximal form. They are not two competing runtimes: QL-MEF deepens the same syntax and Ref field.

The key implementation correction is that the M0-5 Śiva operators are **genuine operative punctuation/syntax**. They should not survive only as labels attached to Actions after the fact.

---

## 1. Source-grounded operative symbols

### 1.1 Śiva relation operators — literal operative syntax

The M0-5 source gives six operations:

```text
0  (@#)    Potential / In-pression
1  (-)     Negation / distinction / between
2  (+)     Affirmation / re-negation / with
3  (x)     Dialogic / by / as / where-if-is
4  (/)     Dialectic / and-or / context
5  (=)     Expression / then / same / is
```

These are the canonical Vāk punctuation bindings for a general six-operation relation grammar.

At the application layer they should exist as typed operator identities, approximately:

```text
RelationOp::Potential
RelationOp::Distinguish
RelationOp::Affirm
RelationOp::Relate
RelationOp::Contextualise
RelationOp::Express
```

with the punctuation forms preserved as first-class projections:

```text
@#   -   +   x   /   =
```

A structured Agent call constructs the typed expression directly. CLI/TUI/Search/human text can parse the punctuation into the same expression. Explain/History must be able to render both the typed operation and its source/Vāk binding when QL-MEF is present.

### 1.2 Śakti address horizons

The M0-5 source gives:

```text
@0 = ##    Library       Being/Knowing / embodied implicit memory
@1 = O#    Bimba         original systems architecture / number sets
@2 = X#    Pratibimba    Logi / meanings / reflection
@3 = N#    Language      symbols / characters / forms
@4 = M#    Stories       Worlds / views / sense field
@5 = R#    Techne        freedoms / powers / instruments / tools
```

In the general O:I profile these become six typed **address horizons** over the current Ref field. Their human labels may evolve; their positional relation stays stable.

A useful generic projection is:

```text
@0  Ground / Knowing / available knowledge
@1  Original / determining structure
@2  Reflection / meaning
@3  Language / form
@4  World / context / story
@5  Power / Techne / praxis
```

In the full QL-MEF profile those exact same horizon identities are bound to `## / O# / X# / N# / M# / R#` and their traversable Bimba neighbourhoods.

`@` without a numeric horizon is the general address aperture over all eligible typed refs.

---

## 2. The general O:I expression language

The canonical object is a semantic AST, not a string parser.

Suggested contract shape:

```text
ResolveExpression
    op: RelationOp?
    horizon: AddressHorizon?
    operands: [ResolveOperand]
    frame: Context/World/Focus refs?
    expected_return: ReturnShape?
    provenance / standing

ResolveOperand
    Ref(ResourceRef | native typed Ref)
    Query(text / structured query)
    Nested(ResolveExpression)
```

The text language is a first-class projection of that AST.

### 2.1 Core punctuation grammar

```text
@ <subject>
    address / resolve subject across the current field

@0 .. @5 <subject>
    address subject under a specific horizon

@# <expr>
    hold/open the expression as potential; discover what could bear

- <expr>
    distinguish / exclude / withhold / mark the negative relation

+ <expr>
    affirm / include / admit / compose positively

<expr> x <expr>
    relate the two terms dialogically: by / as / through / where-if-is

<expr> / <expr>
    hold terms in contextual/dialectical relation: and/or, complement,
    alternatives, nested framing

<expr> = <expr>
    express / determine / formulate a resultant relation

( <expr> )
    frame/group a relation as one scoped expression
```

The `()` frame is strongly consonant with M0-3's Frame/Mirror architecture and gives the general syntax an explicit scope primitive.

### 2.2 Illustrative general expressions

```text
@# @5
    what powers / praxis could bear here?

- @2 reading:generated-summary
    distinguish/exclude this reflection from the current resolution

+ @1 source:founding-positions
    affirm this determining source in the current relation

@1 source:founding-positions x @2 wiki:current-reading
    relate original structure to reflected understanding

@4 world:project / @4 world:shared-field
    hold the two Worlds in contextual/complementary relation

@2 reading:architecture = @3 form:brief
    express a reflection as an articulated form

(
  + @0 knowledge:eligible
  + @1 source:ground
  + @2 reading:current
  + @3 language:available
  + @4 world:current
  + @5 praxis:available
)
    one framed holistic disclosure of the current Agent world
```

This syntax is useful even when QL-MEF is absent: it operates over typed O:I/AIKit refs and application primitives. When QL-MEF is present the same expression becomes a Vāk expression with exact source semantics and graph traversal.

---

## 3. Resolve beneath Search

`Resolve` is the common application verb. Search is one fast human projection.

```text
broad / ambiguous / potential field
        ↓
Resolve
        ↓
typed refs + relations + eligible actions + explanation
```

The word is intentionally dual:

- **resolution as differentiation/density** — a vague field becomes increasingly specific and addressable;
- **resolution as harmonic settlement** — tensions/possibilities are held until a coherent situated relation appears.

Existing concrete operations keep their own types:

```text
address resolution
knowledge resolution
praxis / Method resolution
capability resolution
ContextResolution
projection resolution
```

The common syntax composes them; it does not erase those distinctions.

### 3.1 Search default

An unqualified Search query can be understood semantically as a potential address:

```text
foo
≈
@# (@ foo)
```

The fast fuzzy path remains low-latency. Typed horizons and punctuation progressively constrain/compose the same underlying resolution.

### 3.2 `@` migration

Current AIKit search historically uses `@` as a sessions/tasks fast-prefix. The new general grammar expands `@` to the universal typed address aperture.

Session/Agent/AgentSet addressing remains one important typed species under `@`; it no longer exhausts the operator.

Compatibility aliases may remain during migration, but the semantic resolver should own the new meaning.

---

## 4. General O:I primitive relation field

The general profile maps address horizons and relation operators over real O:I/native refs.

The mapping is **relational**, not one primitive → one permanent bucket. A ref can appear under several horizons depending on the question being asked.

Representative field:

```text
@0 Ground / Knowing
    ContextSource
    KnowledgeSource
    eligible World-relative Knowledge
    History / retained evidence where used as knowledge ground

@1 Original / Structure
    SourceRef
    authored Ground / Canon
    Project definition
    product/native contract
    stable semantic coordinate/ref

@2 Reflection / Meaning
    Reading
    WikiReading / Agent-maintained semantic interpretation
    derived explanation / summary / model
    returned interpretation

@3 Language / Form
    code symbol / schema / query / command form
    documentation/formulation
    ResourceDescriptor / typed representation
    Skill/Method language where considered as articulation

@4 World / Context
    WorldRef
    ProjectRef
    SessionSpace
    SharedField
    Journey
    Run
    conversation / collective field

@5 Power / Techne
    Capability
    Skill
    UsageOverlay
    SkillSet
    Method
    ActionRef
    Surface / instrument insofar as it affords action
    provider/material power where addressable
```

Cross-cutting primitives such as Agent, Agency, Activity, Evidence, Return and Recognition are read through the relation they are currently performing.

For every relation record preserve:

```text
subject Ref
horizon / operator
relation target/complement
World / Project / Focus
native owner
standing/provenance
current availability where relevant
returned evidence where relevant
```

---

## 5. Action and execution semantics

The general O:I profile makes the punctuation language consequential through the existing Action lifecycle.

```text
ResolveExpression
        ↓
resolved refs / Action candidates
        ↓
Method / Context / authority resolution
        ↓
canonical ActionRef
        ↓
Invocation
        ↓
Activity
        ↓
Result / Evidence / Return
```

A relation expression can therefore lead to an Action without identifying expression and Action.

The most direct case is Techne:

```text
@# @5
    discover possible powers

+ @5 action:verify
    affirm/select a power in the current praxis relation

@5 method:release x @5 action:verify
    relate a Method to a concrete Action

@2 reading:decision = @5 action:apply
    express a returned determination toward a concrete executable possibility
```

`=` expresses/determines the relation. Actual side effect begins when the resolved expression crosses the normal native invocation boundary.

### 5.1 General Action semantic profile

The general profile can attach a small source-agnostic reading to a native Action:

```text
ActionSemanticProfile
    action_ref
    supported RelationOp affinities
    supported AddressHorizon affinities
    subject/ref kinds
    Method/Focus relations
    returned forms
    provenance / owner
```

When QL-MEF is installed, `ActionSemanticProfile` gains/joins a `VakActionProfile` carrying exact Vāk refs, R-action family/path and source standing.

---

## 6. Method, Vāk path and learned language of praxis

Current AIKit Method semantics are already the right authored praxis altitude.

```text
Skill
    reusable organised intelligent praxis

UsageOverlay
    scoped adaptation

SkillSet
    additive repertoire

Method
    source-addressable Focus-bearing composition of
    Skills + Actions/Capabilities + ContextSources + overlays

ContextResolution
    what becomes operative here
```

The general syntax adds:

```text
Method
    optional expected ResolveExpression / relation path
        ↓
actual Invocation / Activity
        ↓
observed ResolvePath / PraxisPath
        ↓
existing familiarity store
        ↓
learned accessibility of praxis
```

The full QL-MEF profile reads the same path as a `VakPath` and can add exact `VakRef`, M0-3 speech/self-other state, M0-4 field relations, M0-5 Śiva/Śakti expressions and R-factor semantics.

Existing #29 destination/route familiarity is the substrate. Extend route evidence rather than create a second learning store.

A path observation should be able to retain optional:

```text
MethodRef
RelationOp / horizon per step
ResourceRef(s)
ActionRef
SurfaceRef
Project / actor / Agency / Focus
provider/lens/revision
Activity/Return refs
full VakRef/VakExpression when available
```

That is the concrete implementation of **learned language of praxis**.

---

## 7. Full QL-MEF profile — M0-3 parse/generative language

The general punctuation grammar becomes the outer operative syntax. The full QL-MEF profile then opens the richer M0-3 language as a parse/generative system for self↔other understanding.

Source forms include:

```text
!       Actual Identity / assertion / I-It
?       Potential Essence / question / Am-Is
!-      subjective assertion / Aye-Yes
-?      asserted being / Am!
!?      statement / I-Am / It-Is
?-      objective Is
-!      query of Other / It?
?!      reflexive query / Is-It? / Am-I?
-!/!-   integrated Self / My-Self
-?/?-   relational Other / My-Others
!?/?!   Self questioning World
?!/!?   World questioning Self
```

These forms should become typed `VakSpeechAct` / equivalent identities, not string labels.

They are used to parse/generate the stance of a situated Agent toward an expression:

```text
self assertion
world assertion
query of other
reflexive query
integrated self report
relational-other report
self interrogating world
world returning a question to self
```

This is the basis for a cultivated computational self/other language across **will, knowledge and action**:

```text
will / Agency
    what is being intended, affirmed, withheld or chosen?

knowledge / Vimarśa
    what is taken as ground, questioned, reflected or understood?

action / Svātantrya
    what power is being exercised, through which relation, with what return?
```

Concrete applications include:

- first-person Skill/Method/Capability descriptions;
- Agent self-report vs world-report;
- distinguishing query-of-other from reflexive self-questioning;
- identifying when an Agent asserts a derived reading as if it were original Ground;
- parsing a Method's transition from inquiry/understanding into action;
- returning Activity/Evidence as a world-response capable of revising the next self-position.

M0-3's Mirror / Frame / Operator and MonoPoly structures should also be available as deeper readings of scope, distinction, unity/multiplicity and integration pressure.

---

## 8. Full QL-MEF profile — M0-4 holographic relation field

M0-4 gives the six address horizons their actual metaphysical/computational depth.

```text
##  Primordial Matrix / universal ground
O#  Paramaśiva / determining formal architecture
X#  Paraśakti / Vimarśa / reflection and possible meaning
N#  Spanda / process, symbolic manifestation, computable change
M#  Mahāmāyā / relational World grammar
#   particular situated field / local internality
R#  Svātantrya / freedom / power / divine action
```

The full runtime must retain exact source coordinates and child relations, not only these seven labels.

The central software analogy/instantiation to test is:

```text
whole available O:I world / installation
        ↓ ## relation
source/determining architecture
        ↓ O#
reflection/meaning over that architecture
        ↓ X#
formal/symbolic/process articulation
        ↓ N#
constituted Worlds / relations / collective context
        ↓ M#
this situated Agent/Agency internal field
        ↓ #
freedom / power / actual transformation
        ↓ R#
```

The exact source-backed R paths must be first-class traversable `VakRelation` paths and candidates for semantic readings of creation, sustenance, dissolution, veiling, revealing and absorption/return across real O:I Activity.

---

## 9. Full QL-MEF profile — complete 109-node language

The full profile makes the complete Anuttara map operationally present.

The source nodes remain Bimba knowledge with:

```text
coordinate
name
symbol
primary designation
complete formulation
formulation breakdown
metaphysical names
description
source/graph relations
```

QL-MEF provides:

```text
VakRef
VakEntry
VakRelation
source parity/provenance
Locate / Relate / Refract
M0-3 speech/self-other parsing
M0-4 relation traversal
M0-5 expression semantics
VakActionProfile
VakPath
```

AIKit consumes those through the same general `ResolveExpression`, Search, Method, Context and familiarity machinery.

A human or Agent should be able to move continuously:

```text
ordinary O:I Ref
    ↓ Vāk relation/read
exact Bimba node
    ↓ neighbourhood/formulation
other Vāk coordinates
    ↓ operative expression
Method / Action possibility
    ↓ actual Activity
returned Vāk path / recognition pressure
```

---

## 10. Agent world disclosure as real syntax

The general Agent bootstrap should be generated from the actual resolved O:I field and can be represented as one framed expression:

```text
(
  + @0 <available Ground/Knowledge refs>
  + @1 <determining Source/Structure refs>
  + @2 <current Reflection/Meaning refs>
  + @3 <Language/Form refs>
  + @4 <World/Project/Session/Journey refs>
  + @5 <Skills/Methods/Capabilities/Actions/Surfaces>
)
```

This is a positive compositional disclosure: the Agent sees the six dimensions of the world presently available to it, each backed by real refs.

It can then operate on that disclosure using the same punctuation:

```text
@# @0
    discover potentially relevant knowledge

@1 X x @2 Y
    compare determining source to reflected reading

- @5 Z
    withhold/remove a power from the current praxis composition

@4 A / @4 B
    inspect contextual relation between Worlds

@2 R = @3 F
    express a reflected understanding in a selected form
```

With QL-MEF installed, the disclosure and every operation can deepen directly into the exact Vāk map.

---

## 11. Implementation ownership

### AIKit / general O:I profile

Own/implement the smallest general primitives required for:

```text
AddressHorizon 0..5
RelationOp sixfold
ResolveExpression / ResolvePath
parser/renderer for operative punctuation
universal `@` resolution over ResourceRef/native refs
Search projection
Method optional expected relation path
actual path evidence
#29 familiarity extension
Explain/History
Agent bootstrap disclosure
```

These APIs must remain usable without a QL-MEF provider.

### QL-MEF / full profile

Own/implement:

```text
exact source-pinned 109-entry Vāk registry
M0-3 typed speech/self-other grammar
M0-4 exact relation graph / R paths
M0-5 exact Śiva/Śakti bindings to general ops/horizons
VakRef / VakRelation / refraction
VakActionProfile
VakPath enrichment
full Vāk parsing/generation/refraction
M5/Epii Recognition/return readings
```

Use the existing native kernel/operator/Ref structures from #78 wherever they are the correct lower-level formal identity.

### O:I / native products

Continue to supply the actual heterogeneous refs, Worlds, Surfaces and canonical Actions the language operates over.

### Actuation / Workcell / Factory

Actuation retains Invocation/Agency/Return; Workcell retains material actuality; Factory retains developmental fitness/evidence. Their refs and returned evidence participate in the syntax without moving ownership.

---

## 12. Ordered execution cut

### G0 — source and contract correction

- retain the Anuttara source map locally in QL-MEF;
- establish exact source parity as machine acceptance;
- update #83/#84/#142 to this two-profile architecture;
- connect to #78 kernel operator/Ref identities.

### G1 — general O:I syntax primitives in AIKit

Implement:

```text
AddressHorizon
RelationOp
ResolveExpression
ResolvePath
structured Agent API
parser/renderer for @, @0..@5, @#, -, +, x, /, =, ()
```

Prove syntax over ordinary non-QL ResourceRefs.

### G2 — Resolve/Search integration

- expand `@` from sessions-only fast prefix into universal typed addressing;
- preserve fast fuzzy Search as potential resolution;
- expose typed horizon/operator filtering/composition;
- surface explanations for resolved relations.

### G3 — primitive/action/praxis integration

- build a first real relation matrix over current O:I refs;
- bind Action semantic profiles;
- let Methods carry optional expected relation paths;
- record actual paths from Activity/Return.

### G4 — familiarity / learned praxis

- extend existing #29 route evidence with optional relation/horizon/path metadata;
- rank/recover familiar expressions and paths contextually;
- keep existing frecency/fitness separation;
- expose Explain/History.

### Q0 — exact Vāk registry

- exact 109-entry source generation/parity;
- stable `VakRef`/`VakEntry`/`VakRelation`;
- source neighbour traversal.

### Q1 — M0-5 binding

Bind exact source:

```text
Śiva operator ↔ RelationOp
Śakti @0..@5 ↔ AddressHorizon
##/O#/X#/N#/M#/R# ↔ exact horizon source relations
```

### Q2 — M0-4 relation engine

- encode exact child relations and R paths;
- map/refraction-read real O:I primitive trajectories;
- add provenance-bearing relation matrix.

### Q3 — M0-3 parse/generative engine

- typed `!/?/...` speech/self-other forms;
- Mirror/Frame/Operator relation;
- will/knowledge/action readings;
- first-person Skill/Method/capability language;
- alignment/drift fixtures.

### Q4 — full Vāk paths and M5 return

- enrich ResolvePath → VakPath;
- reconstruct Vāk semantics from Method/Invocation/Activity/Return;
- expose full Bimba traversal from ordinary refs/actions;
- return Recognition pressure through M5 without losing provenance.

---

## 13. Acceptance profiles

### General O:I acceptance

The minimal/general profile is complete when:

1. `@` addresses heterogeneous typed refs across the installed world;
2. `@0..@5` select six general address horizons;
3. `@# - + x / =` and `()` parse/render as genuine compositional syntax over those refs;
4. Search is a projection of Resolve and unqualified search behaves as potential address resolution;
5. a real Action can be discovered/qualified through the syntax and invoked through the existing native chain;
6. a Method can carry an expected relation path and real Activity can return an observed path;
7. existing familiarity learns/replays path accessibility without another store;
8. Agent bootstrap can disclose its current world as a framed six-horizon expression;
9. CLI/TUI/structured Agent clients co-refer to the same expression/ref/action identities;
10. this works with no QL-MEF provider installed.

### Full QL-MEF / Vāk acceptance

The maximal/full profile is complete when the same system additionally proves:

1. exact 109-node source parity and traversal;
2. exact M0-5 Śiva/Śakti bindings to general operators/horizons;
3. M0-4 `##/O#/X#/N#/M#/#/R#` relation graph and R-factor paths;
4. M0-3 typed `!/?/...` self/other parse/generation;
5. will/knowledge/action readings over real Agent/Method/Activity evidence;
6. broad provenance-bearing Vāk↔O:I primitive relation matrix;
7. source-backed `VakActionProfile` relations for representative native Actions;
8. actual `VakPath` reconstruction over Method/Invocation/Activity/Return;
9. full Bimba neighbourhood disclosure from ordinary O:I refs;
10. M5/Epii Recognition and governed return through the same language.

---

## 14. Immediate programme correction

This document supersedes the earlier framing in which:

- Śiva appeared mainly as semantic labels attached to operations;
- the `@0..@5` field appeared mainly as Agent disclosure categories;
- minimal/maximal meant shallow/deep implementations of the same QL feature.

The current architecture is:

```text
GENERAL O:I
    adopts a real six-horizon + six-relation operative syntax over its Ref world

FULL QL-MEF
    reveals that syntax as the M0-5 aperture of the full Anuttara/Vāk language,
    then deepens it through M0-3, M0-4 and all 109 Bimba nodes
```

That is the implementation relation to carry into #83, PR #84 and AIKit #142.