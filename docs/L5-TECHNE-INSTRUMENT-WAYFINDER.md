# L5 Technē Instrument Constellation Wayfinder

**Status:** owner-authorised architecture and parallel-development lock, 2026-09-16.  
**Implementation standing:** this document defines the product/dependency shape and acceptance law. It is not evidence that the cross-product integration is complete. The owner's local worktrees are ahead of some hosted branches; each implementation lane MUST reconcile the current local code before editing and record any material drift here rather than overwriting newer reality with this hosted snapshot.  
**Programme relation:** extends the canonical L5 / Para Vāk projection and its `L5-5 Techne` office; joins the completed K9 focused-instrument/shared-selection work to the current O:I Expression-world substrate, AIKit SemanticWiki/ProjectMap, Central DAY/NOW, Factory activity, and the mature disclosure surfaces in `EpiLogos/research-canvas`. It is a convergence programme, not another product, graph store, scene ontology or desktop shell.

---

## Local drift reconciliation (execution pass, 2026-09-16)

Reconciled against the current local suite before implementation, per the standing rule above. Material findings; the plan text above stands unless explicitly corrected here.

- **Suite locations.** Research Canvas lives locally at `~/Central/Work/projects/Antichrist Project` (its own Tauri app; `WorkspaceTransport` in `packages/desktop-api/src/index.ts` with three working implementations — Tauri IPC, browser bridge, and a read-only static-bundle transport that already proves surfaces run over an injected read model). The Expressions physics reference lives at `~/Documents/fluid-dynamic-typographic-point-cloud-engine` ("O:I — Expressions", vanilla TS + three.js); O-I already vendors it into `packages/oi-design-system/expressions-engine` via `scripts/vendor-expressions-engine.mjs`.
- **Nothing named `TechneAdapter` / `ql.techne/v1` / `DisclosureSession` exists in any local repo yet.** T0 creates it. The nearest existing seams, which T0 extends rather than replaces: QL-MEF's `QlProvider` + `RegistryDisclosureProvider` + Q4 `ClientSubject` adapters (crates `ql-semantic`, `ql-adapters`); O-I's K9 `ql.focused-instrument/v1` `SourceQualifiedSelection` (`desktop/cradle/src/instrument/source.ts`) — the direct predecessor of the DisclosureSession selection, currently instrument-local and registered only in the dev-walk harness.
- **O-I has no Timeline, Map/Street/Globe, Story or Palace surfaces.** These instruments are greenfield inside the Cradle; Research Canvas is their interaction source, reached through the compatibility-transport direction of §12 (a Techne transport implemented in the Research Canvas repo), not by copying substrate records. Research Canvas is privately licensed; same-owner porting stays inside its repo boundary as a transport implementation.
- **O-I transport law is stricter than "language bindings may differ": the desktop reaches native owners only through subprocess CLIs and registered renderer-side source adapters — no crate, HTTP or WASM dependency on QL-MEF or AIKit.** The `TechneAdapter` in O-I is therefore a renderer-side composition over those seams (pattern: `FocusedInstrumentSource`), with native Actions crossing the existing `KernelOp {op:"invoke_action"}` authority seam.
- **Warranted QL readings today are `QlReading` + `QlProvenance` + `ResultClass` (`canonical | deterministic | semantic-stochastic | research`) with evidence refs — there is no separate warrant object.** The `ql?` facet of `TechneReading` maps onto exactly that; its schema requires warrant provenance so unwarranted QL metadata cannot be expressed.
- **AIKit's knowledge model has no temporal, spatial, or exact-selector facets yet.** Its sanctioned ride-along is versioned declared extensions on the open `extensions` maps (precedent `aikit.ql-stance/v1`), and `crates/aikit-adapters/src/central_temporal.rs` already reads Central DAY/NOW through `ctrl` while refusing to mint a parallel temporal model. T1 extends those; it does not add a second read model. Note: ai-kit's working tree carries unrelated uncommitted work in `aikit-cli` — facet work must stay in `aikit-core`/`aikit-adapters` modules that do not collide.
- **Occurrence vs receipt time is implicit in Central:** author-declared `created_at`/`recorded_at` versus reconciler-declared `observed_at` on `central.source-change-horizon/v1` changes; there is no explicit `occurred_at` field. Timeline preserves the distinction by labelling which native field each temporal facet carries; it does not wait on a Central schema change.
- **Factory Runs carry no wall-clock timestamps at all** — ordering is revision/cursor based. Time-indexed Factory activity for the Timeline comes from ProjectCentral `now/agents` handoffs (`recorded_at_unix_seconds`) joined by opaque run/session refs.
- **"M1′–M4′" has no module-level referent in the physics engine code** — its referents there are the GPGPU point-field engine, the `FieldEngineAdapter` render/needsRender/command/capture/dispose lifecycle (the already-proven "one renderer per active surface" law), and the cymatic resonator. M′ embodiment integrates through those, not through invented engine modules.

## Execution status (T0–T9 first pass, 2026-09-16)

Executed the same day, on the local suite, per §19 (one integration owner, lane branches, contract pinned first). State of the world:

- **T0 pinned.** QL-MEF branch `techne/t0-adapter`: `schemas/techne/ql-techne-reading-v1.schema.json` + `ql-techne-session-v1.schema.json`, conformance fixtures `fixtures/techne/` (representative subject, absent-facets, a real-refs development day), Rust contract types + `TechneAdapter` + `DisclosureSelection` seam in `crates/ql-adapters/src/techne.rs`, and 10 contract tests (`crates/ql-adapters/tests/techne_contract.rs`). O-I branch `techne/convergence`: the TS mirror + validators (`desktop/cradle/src/techne/contract.ts`), the source registry, the DisclosureSession store, surface kind `"techne"`.
- **Lanes landed.** T1 ai-kit `techne/t1-facets` (`aikit.techne-facet/v1` declared extensions + Central temporal mapping); T2-transport Research Canvas `techne/t2-transport` (read-only `createTechneTransport` over techne field bundles); T8 O-I `packages/oi-design-system` techne.css + theme derivation + surface lifecycle; T2–T7 instrument lanes as six O-I worktree branches, merged through the integration owner into `techne/convergence` with the composition root wired (`techne/bootstrap.ts`, `techne/open.ts`, `workspace/store.ts` consumer).
- **Gates.** `python3 scripts/techne-gates.py` proves G0–G2 and G4–G7 (G2 against LIVE Central ctrl data), reports G3's second real spatial case open (no local PlaceFacet producer yet), and hands G8 to the owner. Full O-I integrated suite: 170/170; build green.
- **Open items.** Production QL `FocusedInstrumentSource` registration (today only the K9 dev-walk harness registers one) — the m1234 embodiment path needs it live; real T1 provider feeds replace conformance fixtures in the instruments; the tiled basemap and a second real spatial producer for G3; Research Canvas lane branches are not merged to its main line.


---

## 0. The decision this Wayfinder locks

The five mature Research Canvas disclosure families and the Expression system are no longer to be developed as isolated product modes.

They are a **constellation of Technē instruments over the same selected knowledge field**:

```text
Canvas / Constellation
Timeline
Place / Map / Street / Globe
Story
Palace / mnemonic composition
Expressions
```

The first five contribute mature ways of seeing and manipulating a subject. Expressions supplies the living authored/experiential form through which a subject can become scene, motion, material, pedagogy and M1′–M4′ embodiment.

These six instruments are **not** assigned one-to-one to `L5-0 … L5-5` merely because their count happens to be six. The canonical L5 offices remain:

```text
L5-0 Syntax
L5-1 Root
L5-2 Harmonics
L5-3 Geometry
L5-4 Meta Epistemic Framework
L5-5 Techne
```

The instrument constellation is generative use through that articulated stack, principally owned as L5-5 Technē. Harmonics, Geometry, MEF, Context Frames, Vāk and Return can participate in an instrument reading without the UI surface becoming the owner of those semantics.

The desktop consequence is equally explicit:

> **The Cradle/knowledge graph is the working field. Technē instruments are loadable apertures on the currently selected subject, not separate applications a person must leave the field to enter.**

Changing from Graph to Timeline, Globe, Story or Expression changes how the current subject is disclosed and operated. It does not silently mint a second subject, graph, session or authority domain.

---

# I — OWNERSHIP AND THE SINGLE ADAPTER

## 1. One QL-grounded business/read-model layer; many independently refinable instruments

The Epi experience requires one shared adapter between the instrument UIs and the underlying field. That adapter is the boundary which allows Canvas, Timeline, Place, Story, Palace and Expressions to be developed in parallel without each surface inventing its own semantic model.

Working public identity for this contract:

```text
ql.techne/v1
runtime interface: TechneAdapter
```

The exact language bindings may differ by repository, but every first-party instrument must consume the same semantic contract and conformance fixtures.

The relation is:

```text
                    NATIVE OWNERS

 Central       AIKit/Wiki       Factory       O:I Expression       other sources
 DAY/NOW       SemanticWiki     Run/evidence  scenes/profiles      files/media/etc.
 sources       ProjectMap       activity      presentation         native identity
      \            |              |               |                   /
       \-----------+--------------+---------------+------------------/
                               |
                     native refs / Readings / Actions
                               |
                               v
                 +-------------------------------+
                 |       TechneAdapter           |
                 |                               |
                 | QL Kernel / Vāk / CF / MEF    |
                 | constellation + harmonics     |
                 | geometry + refraction         |
                 | provenance + Return           |
                 |                               |
                 | selection / capability        |
                 | temporal / spatial facets     |
                 | source / scene bindings       |
                 | native Action routing         |
                 +-------------------------------+
                               |
                       ql.techne/v1 readings
                               |
       +-----------+-----------+----------+-----------+-----------+
       |           |           |          |           |           |
     Canvas     Timeline      Place      Story       Palace    Expressions
       |           |           |          |           |           |
       +-----------+-----------+----------+-----------+-----------+
                               |
                    same selected subject/session
                               |
                         Epii / human work
                               |
                       attributable Return
                               |
                          native owners
```

### 1.1 What “shared business logic” means

The shared layer owns the logic which must not drift between surfaces:

- stable selected-subject and bounded-whole identity;
- source/native-owner/ref/revision preservation;
- relation-family identity and provenance;
- QL shape/constellation/Context-Frame/refraction operations where warranted;
- capability discovery: which disclosures the current subject actually supports and why;
- temporal and spatial normalization without fabricating missing facts;
- passage/source selectors and standing;
- Expression / Scene / composition bindings;
- shared snapshot/revision semantics for a coherent cross-view reading;
- governed Action discovery/routing and Return receipts;
- Epii co-reference to the same subject/session/readings;
- cross-view selection and navigation identity.

It does **not** own every native domain object merely because those objects are visible together.

### 1.2 The adapter is not a store

Lock this boundary:

```text
TechneAdapter != Wiki database
TechneAdapter != QL semantic database
TechneAdapter != Factory Run store
TechneAdapter != DAY/NOW store
TechneAdapter != Expression persistence
TechneAdapter != UI view-state store
```

QL-MEF owns canonical QL structure, operations, refraction and warrants. AIKit's SemanticWiki/ProjectMap owns the generic knowledge-navigation/application seam over independently owned resources. Central owns temporal/source ground. Factory owns developmental Runs/Decisions/evidence. O:I owns desktop Surface/Expression presentation. Each source keeps its native identity and mutation authority.

The adapter composes attributable readings over them.

### 1.3 Full QL-MEF without forced semanticisation

The Epi adapter is allowed to use the **full QL-MEF Kernel**: QL shape and relation operations, Vāk, Context Frames, Harmonics, Geometry, MEF lens/refraction and Return. This is the shared Epi business layer the instruments sit over.

That does not licence fabricated QL meaning.

A Factory Run, historical event or ordinary file with no warranted QL coordinate/lens relation remains a valid subject. It can be shown in Timeline, Place or Canvas through its real data. QL shape addresses and derived readings appear only when their basis is present. No UI may assign a QL coordinate because it needs a colour, lane, glyph or layout.

This preserves the existing O:I law that ordinary worlds remain valid without QL while making the Epi profile genuinely QL-native when QL relations are present.

---

## 2. The portable Technē reading

The language-neutral contract must expose one sufficiently rich subject reading without flattening native objects. Exact field names are implementation work, but the v1 semantics are locked here.

```text
TechneReading
  reading_ref
  snapshot / revision basis

  subject
    subject_ref
    native_owner
    native_revision/readings
    kind / standing where supplied

  whole
    whole_ref
    member refs
    typed relations
    selection/focus

  ql?
    shape / constellation refs
    coordinate refs where actually warranted
    Context Frame / lens / refraction readings
    harmonic / geometric readings
    Vāk/source refs
    derivation / Return refs

  temporal[]
    occurrence time / interval
    receipt time where distinct
    validity interval
    local Day relation / timezone policy ref where supplied
    NOW / Flow / Session / Run / generation refs where supplied
    uncertainty / precision / source

  spatial[]
    PlaceRef independent of coordinates
    geometry / coordinates
    precision / uncertainty
    hierarchy / containment
    validity interval
    observer/reference-frame metadata where needed
    source

  provenance[]
    source_ref + revision
    standing / native owner
    text span | timestamp range | image region | other selector
    evidence / derivation refs

  expressions[]
    ExpressionRef + revision
    SceneRef / composition relation where present
    profile / subject-binding refs

  actions[]
    native ActionRefs / authority / expected effects

  disclosure
    available instruments
    degraded/unavailable reasons
    suggested related disclosures where policy permits
```

### 2.1 Optional facets are first-class

Temporal, spatial, QL, Expression and source facets are optional. Their absence is data, not an error to “complete”.

The UI can therefore truthfully say:

```text
Timeline available
Place unavailable — no disclosed spatial reading
Expression available
MEF reading unavailable — no warranted lens binding
```

rather than either hiding the whole subject or fabricating metadata.

### 2.2 Time is not one timestamp

The shared temporal model must preserve at least these distinctions where the native producer exposes them:

```text
occurrence time
receipt time
valid-from / valid-to
source creation/modification time
DAY boundary
NOW continuity
Session continuity
Run continuity
generation / simulation / presentation time where relevant
```

Central remains owner of the human civil-day/timezone and DAY/NOW source policy. Factory Runs can span Days; several Agent NOWs can coexist; late Returns retain original occurrence and current receipt. Timeline renders these relations rather than collapsing them to `created_at`.

### 2.3 Place is a temporally valid identity, not only lat/lon

The Place projection must preserve the stronger Research Canvas concept:

```text
PlaceRef
  identity / name
  geometry / coordinates
  precision / uncertainty
  hierarchy
  valid interval
  source/provenance
```

A historical institution can therefore occupy one site for one interval and another later without becoming two unrelated semantic objects merely to satisfy a map component.

### 2.4 Passage provenance remains exact

Where the Wiki/source system can supply it, instruments preserve exact source selectors:

- text spans/headings;
- audio/video timestamp ranges;
- image regions;
- source revision;
- native artifact/evidence refs.

A visual edge, timeline event, map point, Story beat or Expression entity can therefore return to the same source unit.

---

# II — SHARED EXPERIENCE STATE

## 3. One subject; shared disclosure state; local instrument state

The O:I Cradle owns an ephemeral **DisclosureSession** / equivalent application state over the adapter. This is not a new canonical domain object.

Shared state includes equivalents of:

```text
selected subject / bounded whole
selected member(s)
reading snapshot/revision
active Project/World/Context Frame
agent session
optional time window
optional spatial focus
optional Expression / Scene focus
navigation history / Return target
```

Instrument-local state stays local unless explicitly saved as a presentation artifact:

```text
Canvas pan/zoom/manual layout
Timeline lane sizing/aggregation/zoom
Globe camera and map layer visibility
Story editor panel state
Palace camera/layout
Expression tool selection / viewport presentation controls
```

A camera move is not a semantic mutation. A timeline lane regroup is not a new ontology. A graph layout is not a source relation. A map filter is not deletion. Persist such state only through an explicit view/presentation owner.

## 4. Cross-instrument selection law

Every instrument must support the same core transition:

```text
open subject A in instrument X
  -> select member/relation B
  -> open B or the bounded whole containing B in instrument Y
  -> return to X
```

The transition preserves stable refs, source basis, AgentSession and selected snapshot unless the person deliberately asks for live refresh/reinterpretation.

Where a view cannot represent the exact selected object directly, it preserves the subject and explains the nearest valid projection rather than silently substituting a different object.

## 5. Gentle disclosure suggestions

The desktop may suggest neighbouring instruments based on the current reading:

```text
View in Timeline
Open places
Tell as Story
Express this
Open constellation
```

These are affordances, not a compulsory wizard and not a linear workflow. Availability is driven by `TechneReading.disclosure`, not hard-coded per route.

Epii may make the same suggestions conversationally because it co-references the same reading. Suggestion does not equal Action execution.

---

# III — THE SIX TECHNĒ INSTRUMENTS

## 6. Canvas / Constellation

**Source technology:** mature Research Canvas canvas/graph interaction plus AIKit SemanticWiki/ProjectMap and QL constellation structure.  
**Destination:** one focused bounded-whole knowledge instrument inside O:I Cradle.

Required behavior:

- consume adapter nodes/members/typed relations instead of owning a second graph substrate;
- preserve leaf -> local whole / bounded constellation navigation;
- support QL-native layouts where a warranted shape/constellation reading exists;
- retain non-QL generic Wiki relations and provider-specific relation types without relabelling them;
- allow manual visual arrangement as presentation state without asserting semantic relations;
- keep provenance/source/standing one panel away;
- selection is the shared DisclosureSession selection;
- allow open-in Timeline/Place/Story/Expression from the same selection.

The Wiki is the general knowledge substrate. Bimba is a deep QL/Epi case within that wider field, not the definition of Wiki.

## 7. Timeline

**Source technology:** Research Canvas timeline + Central DAY/NOW/Flow temporal ground + Factory Run/Decision/evidence + AgentSession/activity refs.  
**Destination:** a general temporal aperture over any subject whose readings carry time.

The Timeline must stop assuming that “time” means historical-biographical content.

Representative lanes can include:

```text
human DAY / authored temporal source
one or several Agent NOW clearings
AgentSessions
Factory Runs / attempts / Returns
commits / source changes / artifacts where admitted
historical events / people / works
Expression scenes / recorded encounters where relevant
```

### 7.1 Multi-lane local scale

At broad scale, the surface may aggregate/compress events. At local scale it can expand a selected interval into multiple concurrent lanes. This is a **view projection over native temporal refs**, not a new multi-timeline domain model.

A single Run may cross DAY boundaries. Several NOWs may overlap. Receipt and occurrence may differ. Those facts stay visible.

### 7.2 DAY/NOW relation

Central owns DAY/NOW lifecycle. Timeline consumes stable refs and readings. It must not close a DAY, carry a NOW, mark a Run complete or promote Wiki knowledge simply because a graphical item was moved.

Any domain mutation is an explicit native Action with its actual owner and expected revision.

## 8. Place / Map / Street / Globe

**Source technology:** Research Canvas map/street/globe components and Temporal Place model.  
**Destination:** a general spatial aperture over any spatially attributable Wiki/field data.

This instrument is not limited to historical geography. It may disclose:

- people/institutions/events through time;
- journeys and routes;
- source production/provenance places;
- shared-field participants or World locations where disclosure permits;
- project/field-study locations;
- astronomical observer/Earth locations where the actual model supplies them;
- any other native subject carrying valid Place readings.

Required behavior:

- same selected subject/relations as Graph/Timeline;
- time filter can constrain spatial readings without changing their identity;
- hierarchy and precision remain inspectable;
- uncertainty is visually distinguishable from exact location;
- Globe/Street/Map are presentations of the same Place readings, not separate spatial stores;
- camera state remains local;
- selected place/entity can open its constellation, temporal history, sources or Expression.

## 9. Story

**Source technology:** Research Canvas Story UX.  
**Canonical sequencing destination:** **Expression Scenes**.

Lock the persistence rule:

```text
Research Canvas Journey / Story
      -> ordered source-backed Expression Scenes
      -> optional Scene frame: time + place + subjects + media/source refs
      -> presentation / pedagogy / traversal
```

Do not retain a second Journey persistence ontology merely to preserve the old surface.

The local implementation must inspect the current Expression Scene contract and add only the smallest framing relation needed for richer historical/pedagogical scenes. The frame belongs with or references the actual Scene; it does not become a duplicate Story graph.

Story UI can remain a specialised editor/reader over those Scenes: sequence, pacing, source cards, place/time context and transitions.

## 10. Palace

**Source technology:** Research Canvas Mind Palace UX where it survives actual use testing.  
**Canonical semantic destination:** **mnemonic/artistic/pedagogical composition of Expressions**.

Lock these laws:

- Palace does not own a second knowledge graph;
- Palace does not own a second Scene type;
- Palace elements reference actual Expressions / subjects / sources;
- arrangement and navigation are composition/presentation state;
- the existing Palace UI is ported only if real interaction testing says it adds value.

First inspect current O:I Expression composition/Expression-of-Expression capability. Prefer using/extending that actual composition relation. Introduce no `Palace` persistence primitive merely because the old application had one.

## 11. Expressions

**Source technology:** current O:I Expression-world substrate + installed/current physics field and M1′–M4′ instrument.  
**Office:** the most directly generative Technē in this constellation: disclosure becomes an authored living experience.

Expressions already provide revisioned scenes, entities, subject bindings, representations, provenance, selection, parameter state and structured human/Agent operations. The current Expression-world Wayfinder further establishes profiles, reciprocal front/verso presentation, native content bindings and Ta-Onta/Epi participation.

Do not move Expressions “down a layer” or invent another Expression kernel.

The Technē integration adds:

- `TechneReading` as a first-class source for Expression instantiation/focus;
- graph/timeline/place/story selections can open or create an Expression over the exact same subject refs;
- Story edits sequence real Scenes;
- Palace composes real Expression refs;
- QL Harmonics/Geometry/MEF/Vāk can drive lawful profiles/layout/transition when actually warranted;
- M1′–M4′ can embody the same selected field through the production physics/sensory runtime;
- source/verso remains reachable without leaving the Expression identity;
- Epii can inspect/propose/operate through native Actions while remaining in the same AgentSession.

Expressions are consequently the strongest bridge between the abstract knowledge instruments and the M1′–M4′ material/physics instrument.

---

# IV — PORTING STRATEGY: REUSE THE WORK ALREADY DONE

## 12. Research Canvas is UI/interaction source, not the new substrate owner

Research Canvas already states “one product, many surfaces over one graph” and isolates view components behind `WorkspaceTransport`.

Exploit that seam.

Create a temporary/compatibility transport equivalent to:

```text
TechneWorkspaceTransport
    implements the Research Canvas WorkspaceTransport-facing needs
    using TechneAdapter readings + native Actions
```

This lets mature Canvas/Timeline/Place/Story/Palace components move with minimal initial rewriting.

Rules:

- the bridge is for migration/adaptation, not a second business layer;
- do not add new domain semantics to the compatibility transport;
- no Research Canvas substrate record becomes canonical merely because an old component expects it;
- as a surface is integrated, move reusable query/controller logic toward `ql.techne/v1` read models rather than maintaining two permanent abstractions;
- preserve genuinely useful interaction code, camera logic, timeline layout, map/globe work and authoring affordances.

## 13. Data migration is selective and attributable

The Antichrist/bootstrap profile and other Research Canvas datasets remain legitimate source worlds. They can be imported/adapted into the Wiki through source-preserving migration where desired.

Do not conflate UI porting with content migration.

The surface work must function against adapter fixtures and current Wiki data before any large profile migration is required.

---

# V — DESKTOP UX AND EXPRESSIONS PARITY

## 14. Cradle remains the host

Do not create a Technē super-app.

The existing O:I Cradle/Workbench/Surface system remains the desktop host. The graph/knowledge field is the ordinary navigational context. Instruments open as native surfaces:

```text
inline / replace current focus
split beside
maximise / focus
pop-out / detach
return / re-dock
```

K9's shared-selection law remains authoritative: same subject, same running instance where appropriate, same Epii AgentSession, stable source relation.

## 15. UI parity target: the current Expressions physics experience

The current installed/local Expressions physics UI is the visual/interaction reference for this convergence pass.

**Do not treat the older hosted `ExpressionView.tsx` / minimal `expression.css` editor styling as the desired design if the local Expressions experience has moved ahead.** Local implementation reality wins for component extraction; this Wayfinder supplies the product relation.

Parity means a coherent family, not identical controls.

Extract/reuse the current Expressions design language into shared Cradle primitives/tokens where it is actually general:

- canvas-first visual hierarchy;
- restrained chrome and contextual controls;
- coherent dark/light themes rather than product-specific default colour cast;
- common typography, spacing, border/surface/transparency roles;
- icon/tool rail grammar where appropriate;
- contextual inspectors/panels rather than permanently exposed forms;
- shared hover/focus/selected/disabled/revision-conflict states;
- consistent split/focus/detach/full-screen behavior;
- legible source/provenance/depth disclosure;
- reduced-motion/accessibility paths;
- one renderer/resource lifecycle per active visual surface rather than accidental duplicate background engines.

Native instrument interactions remain native:

- Timeline keeps timeline/lane controls;
- Globe keeps camera/geographic controls;
- Canvas keeps graph/layout tools;
- Story keeps sequencing/editor controls;
- Expression keeps physics/material tools.

Do not force all of them into one generic toolbar merely to look consistent.

## 16. Surface lifecycle and performance are UX requirements

Parallel UI convergence must not leave every expensive renderer resident.

Required law:

```text
active/focused visual surface -> allocated resources
hidden/suspended surface      -> paused/released according to owner lifecycle
same live object in two placements -> shared/subscribed where possible, not blindly cloned
```

Measure the point-field, graph and globe cases. Switching instruments must not multiply permanent animation loops, WebGL contexts or subscriptions.

---

# VI — EPII AS CO-WORKER ACROSS THE APERTURES

## 17. One AgentSession, not six assistants

Epii remains the same canonical AgentSession while the person changes disclosure instruments.

The session can co-reference:

```text
current subject/whole
active instrument
selected member/relation
snapshot/revision
visible time/place/scene focus
available native Actions
sources/provenance
```

This supports interactions such as:

```text
"show me yesterday's development around this"
    -> Timeline over the same field

"why did these runs diverge?"
    -> related constellation + evidence

"where did these sources originate?"
    -> Place/Globe if spatial readings exist

"teach me the path that led here"
    -> Story / Expression Scenes

"make this explorable"
    -> Expression over the same subject
```

The agent may gently propose a disclosure switch. It must not simulate the result in chat when a native instrument reading/action exists and is available.

Agent proposals to change source, Wiki, Expression or developmental state retain their actual authority/review path. Visual co-presence never expands authority.

---

# VII — PARALLEL DEVELOPMENT GRAPH

## 18. The lanes

The development phase is deliberately structured so one small shared-contract owner removes the main coupling and the instrument lanes can then move independently.

```text
T0  ql.techne/v1 contract + adapter floor + fixtures
 |\
 | +--> T1 temporal/spatial/provenance provider enrichment
 | +--> T2 Canvas / Constellation
 | +--> T3 Timeline / DAY-NOW / Factory activity
 | +--> T4 Place / Map / Street / Globe
 | +--> T5 Story -> Expression Scenes
 | +--> T6 Palace -> Expression composition
 | +--> T7 Expressions + M1′–M4′ / graph-projection bridge
 | +--> T8 Cradle UX / Epii / shared visual system
 |
 +--------------------------------------------------+
                         |
                         v
                 T9 joined convergence
```

T1–T8 may run in parallel as soon as T0 publishes the minimal fixture/read-model contract. They must not wait for every real provider: use the shared conformance fixtures and then replace fixture inputs with native providers as those lanes land.

### T0 — Shared Technē contract and adapter floor

**Primary owners:** QL-MEF contract semantics; O:I runtime composition; AIKit knowledge/provider consumption.  
**Deliver:**

- `ql.techne/v1` language-neutral schema/fixture;
- `TechneAdapter` interface in the actual desktop/application boundary;
- one representative selected subject containing graph + temporal + spatial + source + Expression bindings;
- capability/unavailability reasoning;
- native Action routing contract;
- shared DisclosureSession selection bridge;
- Research Canvas `WorkspaceTransport` compatibility adapter;
- contract tests proving no native identity is rewritten.

**Do not:** create a database or move SemanticWiki into QL-MEF.

### T1 — Temporal, spatial and provenance richness

**Primary owners:** AIKit/Wiki adapters with Central/Factory/native producer contracts; O:I read-model consumption.  
**Deliver:**

- Temporal facet with occurrence/receipt/validity distinctions;
- Place/geometry/precision/hierarchy/validity facet;
- exact source selectors;
- adapters for real Wiki/historical data;
- DAY/NOW/Flow refs;
- Factory Run/activity/evidence refs;
- degraded/unknown states rather than guessed values.

This lane supplies richer real data but does not block T2–T8 fixture-based UI work.

### T2 — Canvas / Constellation port

**Primary source:** Research Canvas Canvas + AIKit SemanticWiki/ProjectMap + existing Cradle graph.  
**Deliver:** mature interaction over `TechneAdapter`, shared selection, QL layout where warranted, provider relation/provenance inspector, cross-open actions, Expressions-aligned visual system.

Preserve bounded local-whole behavior. Do not create a global graph hairball.

### T3 — Timeline

**Primary source:** Research Canvas Timeline + Central temporal contracts + Factory/Agent activity.  
**Deliver:** general temporal disclosure, local-scale multiple lanes, zoom/aggregation, DAY/NOW/Run/Session distinction, occurrence-vs-receipt display, linked source/graph selection, Expressions-aligned shell.

Prove at least one real development-day case, not only historical fixtures.

### T4 — Place / Map / Street / Globe

**Primary source:** Research Canvas spatial components.  
**Deliver:** generic Place reading, temporal validity/filtering, precision/uncertainty, linked graph/timeline selection, retained camera, Expressions-aligned shell.

Prove both a historical dataset and a non-historical/project/field case where real spatial metadata exists.

### T5 — Story / Scenes

**Primary source:** Research Canvas Story + O:I Expression scenes/pedagogy.  
**Deliver:** Story UX operating real Expression Scene refs, scene-frame temporal/spatial/source context, sequencing/pacing, source inspection, no Journey shadow persistence.

### T6 — Palace / compositions

**Primary source:** Research Canvas Palace + current O:I Expression composition capabilities.  
**First action:** test the existing Palace UX before porting.

If retained, deliver a composition surface over real Expression refs with mnemonic/spatial navigation. If rejected, record the useful interaction elements and implement them through the current composition owner instead of preserving a product noun for its own sake.

### T7 — Expressions and M1′–M4′

**Primary owners:** O:I Expression-world substrate + current physics renderer + QL-MEF M′ producers.  
**Deliver:** open/create/focus Expression from shared Technē selection; QL-backed profile/layout/transition inputs; same-subject graph companion; M1′–M4′ embodied manipulation; stable source/verso; scenes/compositions consumed by T5/T6; no duplicate point-field host.

This is the deepest technology layer, not a separate data world.

### T8 — Cradle, Epii and UI convergence

**Primary owner:** O:I desktop.  
**Deliver:** shared DisclosureSession, instrument switch/open/split/detach grammar, capability-driven affordances, same Epii AgentSession, design-system extraction from current local Expressions UI, dark/light baseline, contextual panel conventions, resource lifecycle and accessibility.

This lane can begin immediately against fixtures and the current local Expressions visual implementation.

### T9 — Joined convergence and human acceptance

T9 is not another implementation rewrite. It integrates the accepted lane outputs and proves the whole relation.

---

## 19. Worktree / branch discipline for local multi-agent development

Use one integration owner and separate worktrees/branches for disjoint lanes.

At start:

1. inspect current local branches/worktrees and the exact installed/current desktop/Expressions implementation;
2. read repository-local `AGENTS.md` and current Wayfinders before editing;
3. reconcile this hosted plan against local current code; add a concise drift note here for any material difference;
4. land/pin T0 contract fixtures first;
5. give every lane the exact T0 contract revision;
6. launch T1–T8 in parallel where paths are safely separable;
7. rebase/merge through the integration owner, not by letting each lane redefine the shared contract;
8. contract changes after T0 require explicit compatibility/migration notes and fixture update.

A lane may refine internal UI implementation freely as long as it satisfies the common contract and does not take semantic ownership from another product.

Prefer branch names equivalent to:

```text
techne/t0-adapter
techne/t1-data-facets
techne/t2-canvas
techne/t3-timeline
techne/t4-place
techne/t5-story
techne/t6-palace
techne/t7-expression-m1234
techne/t8-cradle-ux
techne/t9-convergence
```

Exact names may follow the local repository convention.

---

# VIII — INTEGRATION GATES

## 20. G0 — contract truth

Before broad convergence:

- every instrument fixture validates against one `ql.techne/v1` revision;
- stable subject/native-owner/source refs round-trip;
- missing QL/temporal/spatial facets remain valid;
- no adapter write bypasses native Action authority;
- view state does not become semantic data by accident.

## 21. G1 — cross-view identity

One real Wiki subject must travel:

```text
Graph -> Timeline -> Place -> Graph
```

where the facets are available, retaining the same subject/source basis and selection history.

Unavailable facets report why rather than substitute a different subject.

## 22. G2 — temporal development case

Prove a real Project interval containing at minimum:

```text
human DAY material
multiple Agent NOW/session/run relations where present
Factory developmental activity/evidence
late/returned material where available
```

At local zoom, concurrent lanes are distinguishable. No graphical operation changes DAY/NOW/Run lifecycle without an explicit native Action.

## 23. G3 — spatial case

Prove one temporally valid historical Place case and one other real spatial case. Precision, hierarchy, source and validity survive Map <-> Timeline <-> Graph transitions.

## 24. G4 — Story / Expression identity

Create/open a source-backed Story over real Expression Scenes. Time/place/subject/source framing remains attributable. Re-open the same Scene in the Expression editor/runtime without conversion to a duplicate Journey record.

## 25. G5 — Palace / composition

If Palace UX is retained, prove a composition of several real Expressions, each independently openable with identity/revision/source intact. Reordering/spatial arrangement changes presentation/composition only.

## 26. G6 — Epii co-reference

The same AgentSession must be able to discuss and operate on the selected subject while the person changes at least Graph -> Timeline -> Expression. Agent context follows refs/readings, not DOM scraping or UI text reconstruction.

A proposed mutation returns through the native owner and its receipt is visible in the relevant instrument.

## 27. G7 — M1′–M4′ embodiment

Take one subject with warranted QL/M′ relations from graph/knowledge selection into the production Expression/M1′–M4′ instrument. Demonstrate that the deeper harmonic/geometric/material manipulation remains connected to the same subject/source/scene and can Return to the knowledge field without creating a shadow object.

## 28. G8 — UX/performance acceptance

Human acceptance on the installed desktop must verify:

- coherent Expressions-family design across instruments;
- graph remains a comfortable home context rather than a launch menu;
- view switching feels like disclosure of the same thing;
- split/detach/focus preserves selection and Epii session;
- dark/light and accessibility paths work;
- no unintended pinned/persistent chrome blocks the canvas-first experience;
- no hidden duplicate point-field/globe/graph render loops survive switching;
- large real datasets remain navigable through bounded/aggregated views;
- the richer system remains understandable without exposing all QL terminology by default.

---

# IX — AMBIGUITIES CLOSED BY THIS PLAN

## 29. Do not reopen these distinctions during lane implementation

| Question | Locked determination |
|---|---|
| Is the Wiki Bimba? | No. SemanticWiki/Wiki is the general knowledge substrate; Bimba is an Epi/QL knowledge world projected through it. |
| Are the six instruments six new apps? | No. They are L5 Technē apertures within the existing O:I/Cradle experience. |
| Do the six instruments map to L5-0…5 by count? | No. They are principally L5-5 generative instruments using the whole prior L5 articulation. |
| Does QL-MEF become the universal semantic database? | No. It owns QL structural/refraction semantics; native content owners retain their data and authority. |
| Is `TechneAdapter` a store? | No. It is a composed read/action boundary over native refs plus QL operations. |
| Must every subject have QL metadata? | No. QL relations are exposed when warranted; none are fabricated for UI convenience. |
| Must every subject have time/place? | No. Temporal/spatial facets are optional capabilities with explicit absence. |
| Is multi-lane Timeline a new temporal ontology? | No. It is a projection over native temporal objects. |
| Does Story keep Journey persistence? | No. Story sequences actual Expression Scenes. |
| Is Palace a knowledge substrate? | No. It is a composition of Expressions; existing UI survives only if useful. |
| Are Expressions being moved down/rebuilt? | No. The existing Expression-world substrate is retained and integrated. |
| Is M1′–M4′ another data model? | No. It is a deeper computational/sensory engagement with the same selected subject/readings. |
| Does a visual drag/move change domain truth? | Only when an explicit native Action says so. Otherwise it is presentation state. |
| Does Epii get a separate session per instrument? | No. One AgentSession co-references the changing disclosure of the same field. |
| Should porting Research Canvas require migrating all Antichrist data first? | No. Port surfaces against adapter fixtures/current Wiki; content migration is separate. |
| Which UI is the style authority? | The actual current local Expressions physics UI, reconciled into shared Cradle primitives; older hosted editor CSS is implementation evidence, not a design target. |

---

# X — DEFINITION OF THE COMPLETED PHASE

This programme is complete when the person can remain in the O:I desktop with Epii, select a real subject in the Wiki/graph, and disclose that same subject through the instruments appropriate to its actual data:

```text
constellation / graph
        <-> timeline
        <-> place / globe
        <-> story / scenes
        <-> Expression
        <-> mnemonic composition
```

while:

- all surfaces consume one shared Technē business/read-model contract;
- native owners retain source/data/mutation authority;
- QL-MEF supplies the common structural/harmonic/geometric/refraction depth without fabricated semanticisation;
- DAY/NOW and Factory activity become temporally visible without ontology duplication;
- spatial data becomes a general disclosure axis rather than a historical special case;
- Story and Palace converge onto the existing Expression scene/composition world;
- the M1′–M4′ physics/sensory system receives the same selected field rather than a copied version of it;
- Epii understands and operates the subject across views through native refs/Actions;
- the desktop presents these capabilities as one coherent Expressions-family UX;
- work done through an instrument can return as an attributable artifact, relation, source proposal, Expression or other native determination.

The result is not “Research Canvas inside O:I”.

It is the intended Epi recursion made operational:

```text
native worlds / sources
        -> Wiki / selected whole
        -> QL articulation / Context Frame
        -> Technē disclosure or M1′–M4′ embodiment
        -> human + Epii refinement / creation
        -> attributable Return
        -> changed native world / Wiki / Expression
        -> new ground for the next encounter
```

---

# XI — SOURCE / PROVENANCE TRAIL

Reconcile against current local successors before implementation. At authoring time the principal carriers are:

### QL-MEF
- `docs/KERNEL-REBUILD-WAYFINDER.md`
- `docs/kernel-rebuild/LIVING-INSTRUMENT-ARCHITECTURE.md`
- `docs/kernel-rebuild/APERTURES-AND-CLOCK-CENTRE.md`
- `docs/kernel-rebuild/PARENT-SURFACES-INTEGRATION.md`
- `docs/QL-VAK-KERNEL-RECONCILIATION.md`
- issue #123 — canonical L5 / Vāk / Technē reconciliation
- issue #133 — K9 shared selection/focused desktop instrument (completed; architectural provenance)

### O:I
- `docs/positions/FOUNDING-POSITIONS.md`
- `docs/EXPRESSION-WORLD-SUBSTRATE-WAYFINDER.md`
- current `docs/cradle/EXPRESSION-FIELD.md` / `docs/experience/EXPRESSION-FIELD.md` successors
- `desktop/cradle/src/expression/**`
- current Cradle/Workbench/Surface/Knowledge/AgentSession implementations
- actual current local Expressions physics/UI implementation

### AIKit
- V2 Knowledge Navigation / SemanticWiki / ProjectMap programme (#23, #34–#38 and current successors)
- V2 shared TUI/application relation model where its provider-neutral selection/read-model contracts are reusable
- current QL/MEF provider interoperability contracts

### Central / Factory
- Central #74 and current DAY/NOW/Flow successors
- Central source/change-horizon contracts
- current Factory Run/Decision/Claim/Evidence/Return contracts
- current AgentSession / activity relations consumed by O:I

### Research Canvas
- `CONTEXT.md`
- `docs/architecture.md`
- `docs/data-model.md`
- `docs/surfaces.md`
- current `WorkspaceTransport`
- Canvas / Timeline / Map-Street-Globe / Story / Palace surface implementations

This source trail is a provenance map, not a command to preserve obsolete implementations. Local current code determines what exists now; this Wayfinder determines the intended convergence relation unless a later explicit owner decision supersedes it.
