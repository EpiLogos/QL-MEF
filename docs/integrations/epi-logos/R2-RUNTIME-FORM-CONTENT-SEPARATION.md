# R2 — Runtime / Form / Psychoid Computation Separation

**Status:** R2 working architecture for human review  
**Tracking:** QL-MEF #25 · Epi-Logos-C-Experiments #4  
**Do not treat as implementation backlog or closure evidence.**

This pass answers a narrower and more load-bearing question than the earlier cross-map:

> Given the actual Epi-Logos architecture and the actual current O:I suite, what is generic technological runtime, what is Epi-Logos constitutional form, and what is Epi-Logos psychoid computation/content?

The purpose of the distinction is not organisational neatness. Without it, two opposite errors become likely:

1. treating mature Epi software as legacy infrastructure to be replaced merely because O:I now supplies more general machinery;
2. re-generalising Epi-specific constitutional or psychoid meaning into O:I and thereby making the Cradle secretly depend on one application of its possibility space.

The intended relation is instead:

```text
O:I Cradle / generic technological agency
                    ↓ makes available
       Epi constitutional inhabitation form
                    ↓ operates
       Epi psychoid computational domain
                    ↓ becomes lived as
                  M′
                    ↓ encounter / return
              renewed Epi ground
```

The general substrate may improve because Epi is a demanding inhabitant. Epi may improve because O:I supplies increasingly capable generic bodies. Neither relation requires either system to lose its native semantic ownership.

---

## 1. Provenance and present-state discipline

### 1.1 Epi source authority used here

Accepted R1 source merge:

`be54a505728eaa06ddcc268fa53df5dd756bfb5e`

R1 source-reading head:

`8608648f33e697dd5a8c5f499492619a02259af5`

This pass re-opened active M/M′ specs and implementation rather than relying only on the R1 inventory. In particular it traced:

- `Idea/Bimba/Seeds/M/M'-SYSTEM-SPEC.md`;
- the active `M0′`–`M5′` specs;
- `Body/S/S4/ta-onta/` and the Khora / Hen / Pleroma / Chronos / Anima / Aletheia carriers;
- `Body/S/S0/epi-lib/` — the C kernel and M0–M5 roots;
- `Body/S/S0/epi-cli/` — the Rust `epi` command surface and C FFI;
- `Body/S/S0/portal-core/` — Rust Pratibimba/harmonic projection logic;
- `Body/S/S3/gateway/` — Rust session/runtime/subscription body;
- `Body/S/S5/epii-autoresearch-core/` — Rust Epii return/recomposition body;
- `Body/M/epi-theia/` — the implemented M′ application/extension family.

### 1.2 O:I target state re-read for this pass

The previous R2 cross-map had already become stale because the suite continued to move. The following heads were re-inspected during this pass on 2026-08-18:

```text
O:I                 3ef417a88b84fa9684a58caf968e863bf7671cd7
Central             d09864643f804ee169bbced36d5a0e09bc311c08
Actuation           0c6a9147a780329007733df643eb07108f589ac6
AIKit               c9919ac2b2bb775a19180300f2f4e14e3d34a4b1
Software Factory    4767d2873ce0c309816c1b333575323254eacd13
Workcell            8a744b1ddbdc694ad71b7aea72064f7def6620d2
Quaternal Logic     d0e012b9a2080b75b9583d5fcc672775cce3a7ca
```

Current product meaning was read from the merged/current authored vision/constitution material, not inferred backwards from code alone. O:I remains the sparse whole-field composition/disclosure layer; the native products keep their semantics.

---

## 2. Purpose before decomposition

The human correction guiding this pass is:

> Epi-Logos is intended to genuinely compute psychoid reality in the service of turning technology directly, and with quality, skill and precision, toward human flourishing across the entire psychoid level of reality.

This sentence must not be flattened into either a marketing claim or a mere software architecture claim.

### 2.1 What is implementation fact

There is already substantial executable formal/symbolic psychoid machinery.

The C `epi-lib` boot path verifies canonical Bimba entities, the `#5 → #0` Möbius return, the `#4` lemniscate anchor and the seven CF roots, instantiates six mutable mirrors, cross-links six families, then initializes and verifies M0 Anuttara, M1 Paramasiva, M2 Parashakti, M3 Mahamaya, M4 Nara and M5 Epii as linked computational roots.

The C sources contain, among other things:

- M0 immutable Bimba/Anuttara tables, Vimarśa operators, virtues, Vāk/zodiacal operators, archetypal-number language and divine-act structures;
- M1 Ananda Bimba/Pratibimba/synthesis matrices, Spanda structure, dual digital-root rings and QL flowering;
- M2 the 72-position MEF/vibrational architecture, 12 lenses, Tattva structures and cross-correspondential tables;
- M3 the symbolic transcription engine: dinucleotide operators, 8 trigrams, 64 hexagrams/codons, complement/movement/resonance operators and related mappings;
- the bioquaternionic kernel, including Bimba↔Pratibimba displacement, 72-dimensional resonance energy and the twelve-tick descent/ascent cycle;
- M5 the Logos FSM, M/L/P/S/T/C self-views and guarded Möbius return.

The Rust `epi` CLI statically exposes this C body through typed FFI rather than treating it as an external historical program. Rust `portal-core` independently contains harmonic, Hopf, codon/rotation, Mahamaya and kernel projection modules. Rust S5 consumes the Aletheia return wire format and evaluates the six Epi subsystems.

### 2.2 What is not thereby proven

The implementation does **not** prove that the formal system exhaustively or veridically computes psychoid reality, nor that use of the system reliably produces human flourishing.

Those are research propositions to be encountered experimentally.

The present implementation is also uneven. For example, M4 already has a real Nara root, safety state, lens registry, voice/container structures and consent-gated randomness, while its lens translation callbacks, alchemical operations and protocol-card contents are explicitly stubs/placeholders. The architecture must therefore preserve both substantial reality and honest incompleteness.

### 2.3 Why the distinction matters for O:I

O:I should make this research/programme increasingly executable, inspectable and safely inhabitable. It should not silently become the authority for Epi's psychoid truth claims.

The desired dependency direction is:

```text
Epi authored/formal source
        ↓
Epi executable psychoid computation
        ↓ exposed through explicit contracts
O:I-native Agency / Context / body / development / materialisation
        ↓
Epi M′ lived instruments and parent field
        ↓
encounter + evidence + human recognition
        ↓
reviewed return
```

---

## 3. First correction: “runtime” names two different things

The word `runtime` has been obscuring the architecture.

### 3.1 Epi formal execution runtime

This is the executable body of the Epi domain itself:

```text
C epi-lib
Rust Epi projections / bridges
Bimba-specific graph/read models
M0–M5 computation
harmonic / symbolic / temporal / personal transforms
Epi-specific return/recomposition
```

This is **not** generic runtime scaffolding merely because it executes.

It is analogous to a physics engine, compiler, game simulation or domain scientific kernel: O:I can host it, expose it and give Agents powers over it without acquiring its semantic ownership.

### 3.2 Generic technological-agency runtime

This is the general machinery around an acting model:

```text
world binding
Agent / Agency
context resolution
capabilities / Actions
models / harnesses
sessions / panes / surfaces
knowledge providers
processes / services / stores / networks
Runs / evidence / candidates
shared projection / contribution
```

O:I's native products increasingly own these concerns.

### 3.3 Architectural law

Therefore:

> **Generalising the technological-agency runtime must not be mistaken for replacing the Epi formal execution runtime.**

When Epi code mixes both concerns in one historical package, the modernisation operation is **separation and explicit delegation**, not wholesale migration.

---

## 4. The three layers

### 4.1 Layer A — O:I Cradle / generic runtime

Current native owners:

| Product | Generic responsibility used by Epi |
|---|---|
| **Central** | authored continuity, Control/Work, durable personal/project ground, stable Actions and Connector boundary |
| **Actuation** | Agent identity, situated Agency, WorldBinding, agentic composition, authority/bounds, metagency, attributable return |
| **AIKit** | Context, Resource/Knowledge, Capability/SkillSet, Profile, model/harness, SessionSpace, Surface, component/body composition, trust, immutable Generation, reversible Procedure, Explain/History |
| **Software Factory** | Project, Commission/intent, Run/Run Map, developmental transformation, Artifact, Claim/Evidence, Candidate and Recognition |
| **Workcell** | material demand, process/service/storage/network/provider bodies, workspaces, material world and lifecycle |
| **Quaternal Logic** | generic executable QL/MEF references/operators, lens/refraction contracts, locate/relate/refract/synthesise and readings provenance |
| **O:I parent** | whole-level installation/disclosure and Participant/Projection/SharedField/Contribution/Encounter between independently grounded worlds |

These products provide powers and relations. They do not define Epi's psychoid subject matter.

### 4.2 Layer B — Epi constitutional / inhabitation form

This is the form by which generic powers become Epi powers **in this encounter**:

```text
Bimba identity / authority / provenance
M / M′ distinction
canonical six Epi domain Agents
S4 functional constellation
VAK: CPF · CT · CP · CF · CFP · CS
dialogical ↔ determinate/mechanistic polarity
Day / Night′ and Möbius return
Kairos interpretation
Hen coordinate/form/residency law
Psyche / Nara lived-context law
Aletheia disclosure / rehearsal / translation / staging law
Epi review and promotion criteria
0/1 parent field
1-2-3 cosmic composition
4-5-0 return/recognition composition
```

This is more than configuration but less than the complete psychoid content. It is **constitutional form**: it determines how capabilities, agents, artifacts, temporal conditions and returned differences are interpreted and related.

### 4.3 Layer C — Epi psychoid computation/content

This is the actual domain Epi is computing and making interactable:

```text
M0 — Anuttara formal/Bimba ground, number-language, virtues, Vimarśa/Vāk structures
M1 — Ananda, Spanda, quaternion/Hopf relations, QL genealogy/flowering
M2 — 72-space, MEF/Vimarśa, correspondence, 8+4 harmonic/audio/cymatic field
M3 — 64-fold symbolic/genetic transcription, world-clock/codon/hexagram relations
M4 — personal/lived psychoid composition, Q_identity/Q_transit/Q_activity, oracle/protocol field
M5 — Epii Logos/return, pedagogy, self-articulation, Epi-specific review/autoresearch
Bimba corpus / graph authority
OracleFrame / symbolic packet and protein chains
Epi-specific datasets and correspondential source
```

O:I can materialise these computations and make their outputs available. It must not reinterpret availability as authority.

---

## 5. Ta-Onta recovered as the Epi inhabitation membrane

The deep Ta-Onta implementation is not best understood as six legacy plugins. Its common spine establishes a repeated contract across Khora, Hen, Pleroma, Chronos, Anima and Aletheia.

Each carrier can contribute four things:

```text
1. DISCLOSE / INJECT
   What this part of the world makes operative for the actor now.

2. RECEIVE / EXTRACT
   What attributable residue of the encounter returns to this part of the world.

3. COMPILE / RECOMPOSE
   How accumulated residue is transformed between encounters.

4. QUERY
   How this part of the world remains addressable without being indiscriminately loaded.
```

The current compositor additionally orders disclosure by `hot / warm / cold` cost under a bounded context budget.

This yields a stronger definition of inhabitation:

> A world is not inhabited merely because information from it was inserted into a prompt. A world is inhabited when it has lawful means to become operative within an actor **and** to receive attributable consequences of the encounter back.

The six Ta-Onta carriers then describe six aspects of that relation.

---

## 6. Khora — grounded encounter

### Purpose

Khora establishes *where this act is actually happening* as an inhabitable world.

It is not fundamentally a filesystem or session database.

### Existing implementation

Current Khora code already:

- establishes Epi vault/world roots;
- creates/enters Day and NOW structures;
- initializes session identity;
- establishes the compose-phase VAK condition;
- writes continuation before compaction;
- exposes canonical Epi write/sync behaviour;
- produces exactly one Sophia disclosure on shutdown;
- distinguishes deliberate `rehear` closure from `force_closed` interruption.

### Generic mechanics now naturally owned by O:I

- durable human/project world → **Central**;
- Agent situated in a world → **Actuation WorldBinding / Agency**;
- current project/host/session/task/body → **AIKit Context / SessionSpace**;
- material process and storage body → **Workcell** where required.

### Epi residue

Khora still supplies Epi-specific ground law:

```text
Day
NOW
coordinate path
initial VAK condition
Epi source visibility
Möbius closure/rehearing relation
```

### Operating-through relation

```text
Central authored/world ground
        +
Actuation WorldBinding
        +
AIKit Context / SessionSpace
        ↓ interpreted by
Epi Khora ground law
        ↓
this Epi encounter
```

**Judgment:** do not port Khora as a second generic session/world manager. Preserve it as Epi ground constitution and adapt its session/store mechanics to native O:I owners.

---

## 7. Hen — lawful form and residency

### Purpose

Hen makes expression **lawfully resident and related** in the Epi world.

It is not fundamentally Markdown storage or Obsidian automation.

### Existing implementation

Hen already contains:

- VAK-aware template rendering;
- coordinate-bearing frontmatter;
- schema validation and property mutation;
- task/search/backlink operations;
- semantic link-candidate suggestions;
- hybrid vault + graph retrieval;
- explicit knowledge/residency patterns used by the broader Epi source world.

Some current paths still call `obsidian-cli` directly. Those calls are implementation transport, not Hen's semantic identity.

### Generic mechanics now naturally owned by O:I

- human-authored durable source → **Central**;
- retrieval / Knowledge / Resource projection → **AIKit**;
- developmental Artifact/evidence lifecycle → **Factory** where the object is produced in a development Run;
- graph/store process material → **Workcell** where material hosting is required.

### Epi residue

```text
CT and coordinate grammar
Bimba relation/provenance
Epi template/form grammar
Seeds / Present / World residency
thought → disclosure → crystallisation
promotion conditions
```

### Operating-through relation

```text
native source / Knowledge / Artifact
        ↓
Hen Epi form + coordinate + residency law
        ↓
Epi-addressable form
        ↓
M′ / Agent / human encounter
```

**Judgment:** retire or adapt transport-specific duplication, not Hen's form law.

---

## 8. Pleroma — affordance constitution

### Purpose

Pleroma makes technological potential available as **bounded Epi affordance**.

The generic half of old Pleroma has now been substantially generalised by O:I. This is the strongest absorption case.

### Existing implementation

The current Pleroma/Techne body already includes:

- a capability matrix;
- per-Agent tool/skill restrictions;
- VAK profiles on capabilities;
- permission/gate relations;
- bounded primitives;
- provider profile concerns;
- gateway/session tooling;
- cmux workspace/surface/pane control;
- TillDone execution discipline;
- mechanical/craft-level Techne operations.

### Generic mechanics now naturally owned by O:I

- capability/resource discovery + scoped activation → **AIKit**;
- Profiles, SkillSets, overlays, trust, immutable generations → **AIKit**;
- Agent/Agency authority and composition → **Actuation**;
- process/service/runtime embodiment → **Workcell**;
- developmental use of powers → **Factory**.

### Epi residue

Pleroma's remaining constitutional question is not *what capabilities exist?* It is:

```text
Why is this power germane now?
What Epi function does it serve?
At what VAK address?
For which Agent/functional locus?
Under what gate?
In what thread/composition form?
What evidence/return obligation follows its use?
```

### Techne

Techne is best understood as a craft/executable capability register or mode, not as a new generic Agent species. Native AIKit/Workcell capabilities can be presented through an Epi Techne classification without relocating their execution semantics into Epi.

### Operating-through relation

```text
AIKit Capability / Action / model / harness / Surface
        + Actuation bounds
        + Workcell body
        ↓
Pleroma VAK / constitutional affordance reading
        ↓
usable Epi power in this encounter
```

**Judgment:** Pleroma should cease being a parallel generic capability/session manager as native seams become sufficient. Its Epi affordance constitution remains load-bearing.

---

## 9. Chronos — operative temporal condition

### Purpose

Chronos makes time part of the operative world rather than metadata attached to activity.

It is not fundamentally cron.

### Existing implementation

Current Chronos already handles:

- Day and NOW initiation;
- archive/return of a Day;
- morning SEED pickup;
- evening/Möbius recurrence;
- cron registration;
- Kairos computation/status;
- Graphiti day arcs;
- decan-boundary checking;
- temporal context envelopes;
- threshold modification based on temporal/Kairos condition.

### Generic mechanics now naturally owned by O:I

- scheduling mechanics → native capability/provider layer, not Epi ontology;
- session lifecycle → **AIKit**;
- durable world/time-authored source → **Central** where human-authored;
- material scheduler/service → **Workcell** where required;
- events/history → existing native history/evidence surfaces.

### Epi residue

```text
Day / Night′ direction
Day and NOW as lived temporal coordinates
Kairos interpretation
threshold/cadence semantics
Möbius return timing
Epi world-clock relation
```

### Generic pressure exposed by Epi

The current O:I products have time, events, histories, schedules and sessions, but this pass did not find a stable whole-suite abstraction equivalent to:

> an addressable description of the **operative temporal condition** of a world which can become part of Context/Agency independent of one session lifetime.

This is a real pressure, but **not yet evidence for a new root primitive**.

First test:

```text
TemporalCondition provider
        → AIKit ContextSource / Resource
        → ContextResolution / Agency
```

If this is sufficient, no new ontology is needed. Epi Kairos becomes one rich provider; other applications may provide release phase, biological cycle, market session, experiment phase, observatory cycle, etc.

**Judgment:** Chronos may teach O:I a reusable temporal-context contract while remaining the owner of Epi's particular temporal meaning.

---

## 10. Anima — differentiated agency through VAK

### Purpose

Anima turns encountered circumstance into appropriately differentiated agency.

It is not merely an agent router.

### Existing implementation

The implemented VAK address carries:

```text
CPF  polarity / dialogical register
CT   content registers
CP   active positional context
CF   constitutional function
CFP  thread/composition form
CS   sequence + Day/Night′ direction
```

The runtime distinguishes two important modes:

- `(00/00)` dialogical/Ouroboros: exploratory conversation and simple invocation do not require full VAK scaffolding;
- mechanistic/determinate operation: full canonical VAK and Agent/CF/thread constraints apply.

Parallel and fusion forms are separately validated. Guardrails can signal or block transitions. The implementation also supports bounded child-agent propagation and Night′ rehearsal forms.

### Generic mechanics now naturally owned by O:I

- enduring Agent + situated Agency + composition + bounds + return → **Actuation**;
- body/model/harness/session/capabilities → **AIKit**;
- material execution → **Workcell**;
- developmental topology when the act is development → **Factory**.

### Epi residue

```text
VAK determination
CF constitutional-function binding
CPF dialogical ↔ mechanistic relation
CFP Epi thread grammar
Day/Night′ sequence reading
Epi gate law
functional constellation
```

### Two rosters must remain distinct

Canonical M-domain Agents:

```text
Anuttara · Paramasiva · Parashakti · Mahamaya · Nara · Epii
```

S4 functional constellation:

```text
Nous · Logos · Eros · Mythos · Psyche · Sophia · Anima
```

These are not rival versions of one roster.

A useful distinction is:

- **M Agent:** which Epi subsystem/pole of the psychoid whole is being embodied;
- **S4 functional locus:** what function of discrimination, transformation, orchestration or return is needed in an act.

The Factory may validly use the six M identities as its Epi developmental profile, but `Ground / Intent / Design / Development / Application / Recursion` must not become their exhaustive identity.

### Operating-through relation

```text
Actuation Agent / Agency composition
        ↓ embodied by
AIKit body / context / powers
        ↓ materially supported by
Workcell
        ↓ if developmental, situated in
Factory Run topology
        ↓ constitutionally read by
Anima / VAK
```

**Judgment:** do not universalise VAK as the generic Actuation or Factory ontology. Make Epi VAK a first-class profile/constitutional grammar operating through generic agency.

---

## 11. Aletheia — disciplined returned disclosure

### Purpose

Aletheia turns what became disclosed through action into attributable, reviewable difference without prematurely making that difference canonical truth.

It is not generic memory.

### Existing implementation

The scattered Aletheia modules reveal a coherent discipline:

```text
Anansi     intended ↔ became, with attributable traces
Janus      before ↔ after across a threshold
Moirai     trace + source/context + cut insight
Mercurius  translation that preserves origin charge
Agora      plurality held without forced consensus
Zeithoven  next-form anchored to source and attribution
```

The Sophia/Aletheia→Epii handoff is also concrete across TypeScript and Rust. The wire carries session/day identity, final VAK, improvement vectors, Moirai summaries, artifacts and closure quality into the Rust Epii inbox/recomposition body.

### Generic mechanics now naturally owned by O:I

- developmental Evidence / Candidate / Recognition → **Factory**;
- Knowledge / Explain / History / provider-bearing evidence → **AIKit**;
- Agent return and attribution → **Actuation**;
- durable human recognition/promotion → **Central** where authorship is involved;
- cross-world returned difference → **O:I Contribution** when the return enters a SharedField.

### Epi residue

```text
Aletheia's disclosure thresholds
Moirai Night′ rehearing
origin-preserving translation
plurality-before-synthesis discipline
Epi crystallisation criteria
Sophia→Epii return grammar
```

### Operating-through relation

```text
native Evidence / History / return / Contribution
        ↓
Aletheia Epi disclosure + rehearsal discipline
        ↓
Epii / human review
        ↓
accepted return or retained evidence
```

**Judgment:** Aletheia should operate over native return/evidence substrates. It should not become another suite-wide History or Recognition authority.

---

## 12. S / S′ after the Ta-Onta recovery

The earlier working hypothesis survives, but is now sharper.

### 12.1 S as predecessor technical substrate

Base S historically carried both domain computation and generic infrastructure because no O:I Cradle yet existed.

Modern decomposition:

| Historical stratum | Generic mechanics increasingly native to O:I | Epi-semantic residue |
|---|---|---|
| **S0** | command integration, execution/material invocation | C/Rust Epi kernel, formal coordinate bodies, `epi core/profile/portal` |
| **S1** | authored source, indexing, retrieval, ordinary work operations | Hen CT/form/residency and Epi authored-world grammar |
| **S2** | graph/retrieval providers | Bimba identity, authority, Epi graph relations/correspondence |
| **S3** | session store, gateway transport, process/run registry, subscriptions | Day/NOW/Kairos live condition, Epi projection/event semantics |
| **S4** | generic Agent/Agency/harness/session/capability/process composition | canonical Epi Agents, functional constellation, VAK/CF/CFP law |
| **S5** | generic review/evidence/history/autoresearch machinery | Epii pedagogy, Bimba evaluation, Logos/Möbius return, Epi recognition |

### 12.2 S′ as application/refraction/inhabitation law

The strongest current reading is:

```text
O:I-native capability/body/relation
        ↓
S′-like Epi augmentation / refraction / inhabitation law
        ↓
M′ lived domain / Agent experience
```

S′ therefore remains distinct from M′:

- **S′** = how generic technical possibility is made Epi-aware and Epi-lawful;
- **M′** = the lived/interactive Pratibimba of the actual Epi domain.

S′ may distribute across Profiles, SkillSets, providers, adapters, Factory grammars, Actuation bindings and application services. Its architectural function is now clearer than its eventual package topology.

---

## 13. The existing S3 gateway is a separation specimen

The Rust S3 gateway proves why this pass must classify rather than discard old code.

It already implements substantial generic runtime concerns:

- run contexts/snapshots and event broadcast;
- chat process registry and abort state;
- session records and aliases;
- active Agent and subagent lineage;
- model/provider overrides;
- runtime working directory;
- cmux workspace/surface/pane placement;
- subscription registry;
- native SpaceTimeDB WebSocket subscription plus explicit fallback policy.

Those are real implementations, not design placeholders.

At the same time, the same records carry Epi-specific material such as Day, `vault_now_path`, VAK address, Graphiti arc references and temporal/spacetime projection semantics.

Therefore S3 should be mined along the fault line:

```text
GENERIC
session topology / projection transport / provider overrides / process lifecycle
    → AIKit / Workcell / Actuation / O:I-native seams

EPI
Day / NOW / VAK / Kairos / Epi temporal projection / Bimba-episodic references
    → Epi provider/profile/application semantics
```

Migration should preserve behaviour and evidence while replacing duplicate ownership with delegation. “O:I now owns sessions” is not grounds for throwing away working session semantics before conformance is demonstrated.

---

## 14. C/Rust/CLI ownership rules

### 14.1 Keep Epi domain computation Epi-native

The following are not migration debris merely because they live below `Body/S/S0`:

- `epi-lib` M0–M5 computation;
- psychoid-number / pointer-web / family / bioquaternionic kernel structures;
- Epi-specific correspondence and transcription tables;
- Epi-specific Logos/Möbius machinery;
- `portal-core` Epi harmonic/codon/Hopf/projection logic;
- Epii's six-subsystem recomposition/evaluation semantics.

Workcell may host their processes. AIKit may expose them as Resources, ContextSources, Capabilities and Surfaces. Actuation may give Agents lawful access. Factory may invoke them inside research/development Runs. None of those relations transfer semantic ownership.

### 14.2 Generic extraction must be provenance-preserving

Some Epi code is genealogically upstream of a now-generic O:I product. Quaternal Logic/MEF is the clearest case.

When a QL operator, MEF registry, body-composition law or temporal-context mechanism becomes genuinely generic, the right movement is:

```text
Epi source / implemented specimen
        ↓ explicit generalisation
native O:I product contract
        ↓ consumed back by
Epi profile/application
```

This is **extraction and return**, not retroactive reassignment of the original system's meaning.

### 14.3 The `epi` CLI remains a native product doorway

The Rust CLI already separates Epi semantic surfaces reasonably well. It exposes, among other things:

```text
core
vault
graph
gate
agent
techne
nara
profile
portal
```

The modern rule should be:

> Keep `epi` as the stable native Epi doorway. Where a subcommand currently owns generic mechanics now canonically owned by an O:I product, make the command delegate/adapt to that owner rather than implementing a second authority.

Examples of likely enduring Epi-native entry points:

```text
epi core ...       Epi formal/kernel inspection and domain computation
epi nara ...       Nara semantic application
epi profile ...    Epi harmonic/psychoid profile
epi portal ...     M′/Pratibimba encounter
epi <M-domain> ... Epi-specific instruments where surfaced
```

Examples whose **mechanics** may increasingly delegate:

```text
epi gate ...       service/session/transport operations
epi agent ...      generic Agency/harness/session operations
epi techne ...     generic capability/material operations
epi vault ...      generic authored-world mechanics where Central/AIKit own them
```

The exact delegation surface remains implementation work for later review. This R2 pass only fixes the ownership law.

---

## 15. M / M′ software after the separation

### M is not “content files”

M is the Bimba/domain image and its executable formal body. The current C kernel is direct implementation evidence of that relation.

### M′ is not “generic UI”

M′ is Pratibimba: the living experiential, visual, sonic and operational face of M.

The existing `Body/M/epi-theia` implementation already has separately owned extensions for:

```text
M0′ Anuttara
M1′ Paramasiva
M2′ Parashakti
M3′ Mahamaya
M4′ Nara
M5′ Epii
```

and integrated `1-2-3` / `4-5-0` compositions that explicitly do not replace those coordinate-owned instruments.

This is implementation evidence for the source-level conclusion that Epi consists of six full-depth lived instruments within a non-numbered parent 0/1 whole.

### O:I desktop does not settle Epi application packaging

The fact that O:I has a desktop/surface host does not imply that Epi's M′ application should be dissolved into it.

Possible future embodiments remain open:

- Epi native application using O:I runtime contracts;
- O:I desktop hosting the parent Epi Surface and deep Epi workspaces;
- separately packageable M′ instruments sharing lower computation;
- hybrid composition.

The now-stable point is the dependency contract underneath those packaging choices.

---

## 16. A finer M0–M5 separation

| Epi domain | Domain computation/content to retain | O:I operating-through | Constitutional/form relation |
|---|---|---|---|
| **M0 Anuttara** | Bimba ground, number/Vāk/Vimarśa/archetype/virtue structures, canonical coordinate identity | QL generic operators where actually promoted; AIKit read/query surfaces; Workcell body | ground/authority/provenance and M0′ structural instrument |
| **M1 Paramasiva** | Ananda/Spanda, harmonic/mathematical engine, M1-specific QL genealogy | QL generic formal ops; Workcell audio/math body; AIKit capability/surface | M1′ playable movement/instrument law |
| **M2 Parashakti** | 72-space, Epi MEF/correspondential field, Vimarśa audio genesis, cymatic/harmonic mappings | QL generic lens registry where compatible; Workcell audio/render; AIKit Surface | M2′ correspondence/cymatic instrument law |
| **M3 Mahamaya** | symbolic transcription, 64/472 structures, Epi world-clock/codon relations | Workcell compute/render; AIKit Surface; QL relation/refraction | M3′ symbolic clock/cosmos instrument law |
| **M4 Nara** | personal psychoid composition, oracle/protocol/lived interpretive semantics | Central residence; Actuation/AIKit context; Workcell compute | privacy, consent, Day/NOW, lived-context and Nara instrument law |
| **M5 Epii** | Logos/Möbius integration, Epi pedagogy, canon recognition, Epi autoresearch semantics | Factory development/evidence; AIKit Knowledge/History; Actuation return; Central reviewed return | Epii recursive/pedagogical instrument law |

Two tensions remain especially important:

1. **M1/M2 ↔ Quaternal Logic/MEF.** There is real genealogical and implementation overlap. The eventual dependency direction must avoid two drifting “canonical” MEF/QL registries while preserving Epi-specific meaning and source provenance.
2. **M5 internal return ↔ human-authored ground.** The C M5 kernel has a guarded internal `m5_execute_mobius_return` which can modify M0 ground state at the formal cycle boundary. That internal computational operation must never be conflated with authority to mutate human-authored Central Control or Epi canon. Formal return, software state promotion and human authorship are different gates.

---

## 17. Candidate reusable contract: Inhabitation Contribution

The Ta-Onta spine exposes a reusable relation that appears more important than any one carrier implementation.

A component of an actor's operative world may need to state:

```text
what I disclose into this encounter
what I can receive back from this encounter
how I recompose accumulated residue
how I can be queried without always being loaded
```

Provisional contract shape:

```text
InhabitationContribution
  disclose(context) -> bounded contribution
  receive(encounter_return) -> attributable residue
  recompose(residue_set) -> derived/compiled state
  query(question, scope) -> evidence-bearing result
```

This resembles the current Ta-Onta `SpineContribution` without assuming its TypeScript packaging, vendor lifecycle hooks or storage layout.

### Why it exists

AIKit already resolves what is available and what is loaded. Actuation already describes world-bound agency and return. The missing relation may simply be an **integration contract between those two facts**: how a world component participates bidirectionally in an encounter.

### Why it is not yet a root primitive

It may be adequately expressible as:

- an AIKit Component/ContextSource contribution interface;
- an Actuation WorldBinding/return adapter;
- a small shared extension protocol between them.

Promote it to generic ontology only if several non-Epi applications require stable identity/history for the contribution itself.

---

## 18. Candidate reusable contract: Temporal Condition

A second pressure is exposed by Chronos.

Provisional meaning:

> `TemporalCondition` is the current operative temporal situation of a world as something an Agent/Context may reason and act relative to. It is not merely wall-clock time or a schedule.

Possible provider output:

```text
TemporalCondition
  observed_at
  scope/world_ref
  phase
  cadence/transition state
  relevant boundaries
  source/provenance
  application-specific payload
```

Epi could supply:

```text
Day
NOW
Day/Night′ direction
Kairos
world-clock state
return threshold/cadence
```

Another application could supply something entirely different.

Again, first test whether this is simply an AIKit `ContextSource`/Resource class with no new root primitive.

---

## 19. Updated primitive judgment

This deep pass produces **fewer new primitives, not more**.

Most Ta-Onta concepts are better understood as relations or application-specific disciplines over existing O:I objects:

| Candidate noun | Current judgment |
|---|---|
| Gate | policy/authority over Action, Agency, Run or promotion; not new root noun |
| Lineage | existing provenance/determination/evidence relation |
| NextForm | usually Candidate / proposed Artifact / intended transformation |
| Staging | Surface/projection/composition relation |
| Grounding | WorldBinding + Context + source authority |
| Translation | operation preserving provenance/charge; application method unless generic evidence demands more |
| Rehearing | Epi recognition method over returned evidence |
| Residency | source/artifact placement + promotion relation, with Hen providing Epi law |
| Closure quality | evidence field on Session/Run/return (`rehear`, `force_closed`, etc.) |
| InhabitationContribution | promising extension contract; not yet root primitive |
| TemporalCondition | promising ContextSource/provider contract; not yet root primitive |

The ontology should grow only where stable identity, lifecycle or cross-product relation genuinely requires it.

---

## 20. Human flourishing, authority and safety consequences

The purpose of computing the psychoid domain gives the architecture a stricter human-authority obligation, not a licence to bypass it.

### 20.1 Computed relevance is not human authorship

Epi may compute that some pattern, correspondence, temporal condition or developmental pressure is salient. That can alter what is disclosed or proposed. It does not automatically author the person's durable world.

```text
computed / inferred / refracted finding
        ↓ attributable proposal or evidence
human encounter / review
        ↓ if accepted as authored
Central / Epi canon return
```

### 20.2 Precision must remain optional where determination is not yet warranted

The implemented `(00/00)` dialogical mode is therefore not an exception to the architecture; it is an important part of it.

Open conversation, imagination and exploratory encounter must remain possible without forcing full VAK determination. VAK becomes valuable when increased precision actually serves the act.

### 20.3 Interpretation must preserve provenance

Aletheia's repeated invariants — preserve trace, preserve source, preserve origin charge, preserve plural distinction — should remain load-bearing as the system becomes more powerful.

A psychoid interpretation is not made more trustworthy by hiding the chain through which it was produced.

### 20.4 Epi may improve the Cradle

Epi is not merely a consumer of O:I. It is a demanding whole-system proving environment.

Its needs can expose reusable improvements in:

- temporal Context;
- bidirectional world inhabitation;
- attribution of returned difference;
- agent/world continuity;
- human authorship gates;
- interpretive provenance;
- composite body disclosure.

The rule is that any such improvement must be generalised from evidence rather than making Epi terminology mandatory in generic products.

---

## 21. Resulting architecture

The current best whole can be written as:

```text
                    EPI-LOGOS

        Bimba + C/Rust psychoid computation
                        │
                        ↓
             Epi live/domain state
 harmonic · symbolic · temporal · personal · graph
                        │
            ┌───────────┴───────────┐
            │  INHABITATION FORM    │
            │                       │
          Khora     ground encounter
          Hen       lawful form
          Pleroma   bounded affordance
          Chronos   temporal condition
          Anima     differentiated agency
          Aletheia  disclosed return
            │                       │
            └───────────┬───────────┘
                        ↓
                 M0′ … M5′
          parent 0/1 lived Epi field
                        ↓
              encounter / activity
                        ↓
          Aletheia / Epii / human review
                        ↓
                 reviewed return
                        ↺

              OPERATING THROUGH O:I

Central       authored continuity / world ground
Actuation     Agent / Agency / bounds / return
AIKit         Context / powers / body / knowledge / Surface
Factory       development / evidence / Candidate / Recognition
Workcell      material bodies and lifecycle
QL            generic formal/refraction faculties
O:I parent    external/shared Projection / Contribution / Encounter
```

The relation is deliberately **many-to-many**. Khora is not “Central”; Anima is not “Actuation”; Pleroma is not “AIKit”. Each Epi carrier cross-cuts the native products needed to enact its Epi function.

---

## 22. R2 judgments now stable enough for focused review

1. **O:I is the Cradle, not the semantic replacement for Epi.**
2. **Ta-Onta is best retained as Epi inhabitation/return architecture, while its generic mechanics increasingly operate through native O:I products.**
3. **The C/Rust Epi kernel is domain computation and remains Epi-owned unless a capability is deliberately generalised with provenance into another product.**
4. **`epi` should remain a native Epi doorway; generic-mechanics subcommands should delegate to native O:I owners instead of becoming duplicate authorities.**
5. **S′ is now most intelligible as distributed Epi augmentation/refraction/inhabitation law, not necessarily one package and not the same thing as M′.**
6. **M′ remains six full lived Epi instruments inside the parent 0/1 field; O:I Surface hosting does not decide their package topology.**
7. **Temporal Condition and the four-seam Inhabitation Contribution are the two strongest generic lessons currently exposed by the deep Epi architecture, but both should first be tested as provider/extension contracts rather than new root primitives.**
8. **Human-authored return remains a separate authority boundary from formal/computational Möbius return.**

---

## 23. Remaining human-review questions

These are deliberately architecture questions, not implementation tickets.

1. **QL/MEF authority:** which exact M1/M2 tables/operators should eventually consume the standalone Quaternal Logic package, which remain Epi-specialised, and how is genealogy/provenance preserved without dual canonical registries?
2. **Temporal contract:** can current AIKit ContextSource/Resource and Actuation WorldBinding express `TemporalCondition` cleanly, or is one small shared contract missing?
3. **Inhabitation contract:** should the four-seam Ta-Onta pattern be an AIKit Component contribution, an Actuation world-return adapter, or a deliberately small cross-product protocol?
4. **S3 convergence:** which existing Rust gateway behaviours should be preserved behind AIKit/Workcell/Actuation adapters and which transport/state paths become unnecessary once native O:I conformance is proven?
5. **Epi application body:** how should the already-real M′ Theia extensions and integrated 1-2-3 / 4-5-0 compositions relate to the O:I desktop Surface host without prematurely deciding one monolithic packaging model?
6. **Return authority:** what exact software boundary separates Epi formal recurrence, Epi canon promotion, Nara personal continuity and human-authored Central Control?

Until those questions have focused evidence and human review, this pass should **not** be translated into a migration backlog or used to close R2.
