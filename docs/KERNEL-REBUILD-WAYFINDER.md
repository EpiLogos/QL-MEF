# QL-MEF Kernel Rebuild Wayfinder

Status: **current development Wayfinder**  
Scope: **one QL-MEF kernel, rebuilt from the existing C/Rust/Bimba bodies and extended through C++ embodiment**  
Primary repository: `EpiLogos/QL-MEF`  
Related source field: `EpiLogos/Epi-Logos-C-Experiments/Idea/Bimba/Map/**` and the live Neo4j Bimba Map  
Existing programme relations: #51, #78, #69, #73 and current successors  

## 0. Purpose

This Wayfinder fixes the development centre for the next QL-MEF programme.

The work is **not** a fresh theoretical reconstruction and not a language rewrite. We are rebuilding one already-developed holographic kernel into a clear long-lived architecture while preserving the depth of the existing Bimba, C and Rust bodies.

The governing development unit is the **coordinate and its relations**.

No single pass over M1, M2, M3 or another M root can be expected to hold the full subsystem in view. The deep capability matrices and the Bimba Map exist precisely to make that unnecessary. The programme therefore proceeds by coordinate coverage, source/implementation reconciliation and parity rather than by asking an Agent to recover an entire subsystem from whichever files happen to be visible in one session.

The kernel build has four computational strata:

```text
C
    lowest executable Bimba / structural kernel lock

Rust
    typed operational and computational kernel

C++
    continuous geometric / material / acoustic / spatial computation

Neo4j + other projections
    rich active semantic Bimba and transformable relation field
```

These are not separate ontologies. They are different computational holdings of the same kernel field.

### 0.1 Accepted producer floor — 2026-09-10

Reconciled against main `374cf8db5569401980a0f88ae38da1e2d11129b1`: **#138 is complete through merged PR #142. `ql.vak-composition/v1` is the QL-owned producer contract.** Consume the existing `ql_mef::vak_composition::VakComposition`, `ql_cli::vak_composition::execute_request`, `ql vak compose <request.json> --json`, and `fixtures/kernel/vak-composition-v1.json` surfaces. Preserve their whole/identity/frame/provenance/Return behavior while rebuilding the structural strata.

#123 remains architectural provenance, not an instruction to restart Vāk reconciliation. This Wayfinder does not import or re-author the parallel Vāk articulation theory. Downstream owner adoption is separate from producer completion and is not a prerequisite for K0.

This accepted Rust operational floor does **not** claim native-C execution of the higher-order composition operations, a complete recursive M registry, continuous C++ embodiment, or whole-M closure. K0 pins the actual floor and discrepancies; #125–#134 retain their separate contracts. The seven canonical Context Frames are settled; historical `(4/5/0)` is superseded/noncanonical, not an eighth-frame/alias/blocker question.

## 1. The kernel object

The foundational QL field remains:

```text
# / 0/1 <-> 1/0
        ↓
#0 #1 #2 #3 #4 #5
        ↓
C / P / L / S / T / M
        ↓
direct / prime-conjugate
        ↓
canonical relation/operator field
        ↓
recursive coordinate structures and their computations
```

The current native C work already established much of the family/position/face foundation on the closed #76 line. The next rebuild must restore that foundation onto current `main` and extend it into the recursive M structure rather than leaving the M body implicit behind root positions.

### 1.1 M is a first-class Bimba tree

The M family is a kernel-resident semantic world structure:

```text
M
├── M0  Anuttara
├── M1  Paramaśiva
├── M2  Paraśakti
├── M3  Mahāmāyā
├── M4  Nara
└── M5  Epii
```

Each root is both:

- the aggregate/index for that M world; and
- the parent of its actual asymmetric recursive coordinate tree.

Examples:

```text
M1
└── M1-2
    └── M1-2-5

M2
└── M2-4
    ├── M2-4.0
    ├── M2-4.1
    ├── M2-4.2
    ├── M2-4.3
    └── M2-4.5
```

The registry must preserve real source structure. It must not manufacture missing siblings to regularise a tree.

### 1.2 Structural existence is not implementation readiness

A coordinate exists because it belongs to the canonical Bimba/source field. A current implementation may be complete, partial or absent.

The kernel must preserve this distinction mechanically:

```text
coordinate existence
    !=
implementation binding
    !=
readiness
    !=
observed experiential acceptance
```

This distinction is already present in the Rust `MMapIndex` direction and becomes a programme-wide law.

## 2. Bimba authority and the bidirectional reconciliation field

The **live Neo4j Bimba Map is the canonical home of deep coordinate specifics and the large relational field** used from K4 onward.

The `Epi-Logos-C-Experiments/Idea/Bimba/Map/**` corpus also contains a full or near-full serialized M relational body, deep datasets and export/fetch tooling. It is therefore a foundational web-accessible source for the initial pre-vertical phases, especially while this programme is being driven primarily through ChatGPT web sessions.

QL-MEF already carries selected Bimba exports under `data/epi-bimba-map/**`; those are useful local/project fixtures, not a reason to ignore the richer live/serialized source field.

### 2.1 Reconciliation is not one-way

The programme must not encode a simplistic:

```text
Bimba → C → Rust
```

pipeline.

The correct relation is bidirectional and evidence-bearing:

```text
                 M LEDGER
                    │
      ┌─────────────┼─────────────┐
      │             │             │
    Bimba           C            Rust
      │             │             │
      └─────────────┼─────────────┘
                    │
                   C++
```

Examples of legitimate return:

- Bimba reveals a coordinate/relation missing from C;
- C exposes a structural assumption that a serialized or live Bimba projection needs correcting;
- Rust tests expose a wrong or incomplete C table, relation or source interpretation;
- current ratification changes a structural relation and therefore requires Bimba + C + Rust parity work;
- C++ embodiment exposes a contradiction or missing formal relation which returns to the ledger rather than being patched locally;
- a rich Neo4j relation may remain semantic/research material until it is deliberately promoted into structural kernel canon.

The ledger exists to make these movements accountable.

## 3. C — structural Bimba lock

C is the lowest executable layer of the rebuilt kernel.

It owns the stable structural statement of:

```text
kernel positions
coordinate families
direct / prime structure
recursive M-coordinate existence
canonical structural relations
finite-domain canonical tables
structural invariants
coordinate/module bindings
bare-metal deterministic operations where appropriate
```

### 3.1 Current returned reality

On current `main`, native `c/src` contains only `primitive.c`.

The closed #76 head contains the useful native C foundation (`primitive.c`, `holographic.c`, `kernel.c` and related headers), but that line was not merged. The full M0–M5 computational body remains available under the imported/vendored C corpus.

The rebuild therefore begins by reconciling the useful #76 foundation against current main and then promoting the actual M body coordinate-by-coordinate into the native C kernel.

### 3.2 Recursive M registry

The native kernel needs a compact recursive registry with stable IDs. Exact implementation is a K2 design task, but the contract must support:

```text
MNode
    stable node id
    parent id
    root M0..M5
    local segment
    source separator form (- / . / / where meaningful)
    flags/status

MRelation
    stable relation id
    from/to coordinate
    relation kind
    orientation/status

MCapabilityBinding
    exact coordinate
    capability/module identity
    structural status
```

The hot-path C address does not need to carry an arbitrary dynamic path. Recursive coordinates can be interned/compiled into stable IDs with a generated machine-readable manifest.

### 3.3 Every semantic C construct receives a disposition

Every substantial C construct must be one of:

```text
COORDINATE-BOUND
    belongs to an exact kernel/M coordinate

CROSS-COORDINATE
    implements an explicit relation over named coordinates

INFRASTRUCTURAL
    memory/numeric/runtime machinery without semantic coordinate identity
```

No semantically meaningful table or algorithm remains permanently as an unseated helper merely because its coordinate has not yet been inspected.

### 3.4 Structural-change parity law

Once a relation is kernel-structural, structural change requires C reconciliation/parity.

This does not mean every semantic graph edit changes C. It means promotion into the structural Bimba cannot bypass the lowest executable kernel layer.

## 4. Rust — operational/computational kernel

Rust operationalises the same coordinate field.

It owns the safe typed layer for:

```text
coordinate/ref resolution
validated domain types
operators and derivations
state machines
current-state composition
providers
provenance/readiness
service APIs
Actions and application-facing computation
```

Rust should consume/generated-resolve the canonical registry rather than maintaining an independently authored M tree.

Existing Rust work is retained and re-seated rather than replaced. Current code already includes strong QL/MEF/Context-Frame, matheme/music, M2 templateure, physical-pole composition and substantial M3 codon/I-Ching/Tarot/rotation/fold/quaternion computation.

The question for K4 onward is not whether these files are useful. It is **which exact coordinate capabilities they implement, what C/Bimba laws they correspond to, what remains unported, and what requires correction**.

C/Rust parity can be exact table/address equality, exhaustive finite-domain operator parity, or tolerance-bearing numerical parity as appropriate. Rust-only operational state remains valid when its structural inputs resolve to canonical coordinates and its outputs preserve provenance.

## 5. C++ — continuous computational embodiment

C++ extends the same kernel into continuous computation:

```text
geometry
material dynamics
waves
sound
physical/modal simulation
folding
spatial orientation
GPU representation
realtime interaction
```

C++ does not define symbolic identities locally. Every embodied capability remains keyed to stable kernel/M refs and revisions.

The C++ programme should use one shared runtime with domain-complete M engines rather than treating the integrated M1/2/3 object as the only implementation surface:

```text
PoleRuntime
├── M1Engine
├── M2Engine
├── M3Engine
└── M123Composition
```

M1′, M2′ and M3′ can therefore be built against their own full domain engines. The singular M1/2/3 composition then binds those same engine instances around one current event and shared carrier rather than reconstructing reduced copies of them.

The Fourth-Spanda/integrated-physical-pole work already gives important typed embodiment registers — boundary, pulse, carrying topology, wave/quadrature, M2 figure/templateure, M3 crease/form and clock — but this Wayfinder does not attempt to re-author their theory. It treats those current documents and their warrant grades as inputs to the corresponding coordinate rows.

## 6. Neo4j / Bimba projection layer

Neo4j is the rich active semantic field of the same Bimba.

It can safely carry much more than belongs in bare-metal C:

```text
deep coordinate detail
names and meanings
source text
tradition-specific correspondences
large relation networks
research propositions
semantic embeddings
entity/event projections
trajectory/session relations
current semantic developments
```

Canonical graph nodes/relations should resolve to stable kernel coordinate/ref identity and a known registry revision where appropriate.

Research/provisional graph structure may exist without being promoted into structural C canon. Promotion is explicit and then flows through the M ledger.

## 7. The M ledger — central development object

The M ledger is the joining surface between source/canon and all implementation strata.

It should become machine-readable while retaining human-readable matrix explanations.

For every meaningful capability the ledger records at least:

| Field | Meaning |
|---|---|
| coordinate | exact M/kernel coordinate |
| parent/children | recursive structural context |
| canonical role | what this capability is/does |
| source/Bimba refs | authored/deep graph provenance |
| structural invariants | relations that must not drift silently |
| cross-relations | dependencies/transforms/outputs |
| C binding | native C symbol/module/table/operator |
| Rust binding | native typed computation/service |
| C++ binding | continuous embodiment, if relevant |
| Neo4j binding | canonical graph nodes/relations |
| fixture/evidence refs | reproducible evidence |
| warrant | source-recorded / derived / argued / offered / etc. |
| readiness by stratum | absent / mapped / partial / operative / accepted |
| parity state | source/coordinate/relation/operational/experiential |
| discrepancy state | unresolved mismatch and current authority |

One scalar `COMPLETE` is insufficient.

A coordinate may legitimately be:

```text
source-known       YES
C-locked           YES
Rust-operational   PARTIAL
C++-embodied       NO
graph-projected    YES
relation-parity    PARTIAL
experiential       NO
```

The ledger must preserve that reality rather than allowing missing implementation to disappear from scope.

## 8. Matrix-lock protocol

The existing deep capability matrices become the first major ledger inputs.

The Markdown form continues to carry rationale, provenance and human-readable meaning. A machine-readable companion carries exact identities, bindings, statuses and evidence suitable for CI and Agent use.

The matrix system should eventually be able to answer automatically:

```text
Does every canonical matrix coordinate exist in the kernel registry?
Does every C semantic binding resolve to a known coordinate?
Does every claimed Rust implementation resolve to the same coordinate?
Which source capabilities have no implementation disposition?
Which implementation modules are orphaned/unseated?
Which graph coordinates or relations disagree with C/Rust?
Did a structural relation change without C parity?
Is an OPERATIVE claim backed by evidence?
Which rows block a requested vertical slice?
```

The output should support CLI/Agent queries such as:

```text
ql kernel coverage M1
ql kernel coverage M2
ql kernel coverage M3
ql kernel coverage M2-4.5
```

## 9. Pre-vertical development phase

Before broad M1/M2/M3 vertical implementation begins, the programme establishes enough structure that later work is mostly reconciliation rather than rediscovery.

### K0 — current ground and authority reconciliation

Purpose:

- freeze the current development boundary;
- identify current main, current matrices, live Bimba/Neo4j access and the serialized `Idea/Bimba/Map/**` corpus;
- distinguish current canon from stale matrix/export/material without deleting useful source depth;
- record that imported C and Rust are migration bodies to be rebuilt into one kernel, not disposable historical artefacts.

Deliverable: source/canon/implementation input ledger and exact revisions.

### K1 — restore the native C kernel centre

Purpose:

- recover/reconcile useful #76 native C `holographic`/`kernel` work onto current main;
- preserve the accepted family/position/face foundation;
- establish current native C build/test parity before recursive M promotion.

Deliverable: current native C foundational kernel on an active branch/PR.

### K2 — recursive M registry and master M index

Purpose:

- make `M` a first-class whole/index;
- make M0–M5 first-class aggregate roots;
- support arbitrary source-faithful recursive M paths;
- provide stable C-level node/relation/capability identities;
- generate the shared registry manifest consumed by Rust/testing/tooling.

Deliverable: `ql.m-tree/v1` or named successor plus C/Rust resolver conformance.

### K3 — capability ledger + parity protocol

Purpose:

- define machine-readable matrix/ledger schema;
- preserve Markdown rationale alongside machine records;
- define source/coordinate/relation/operational/experiential parity separately;
- define discrepancy/promotion workflow;
- add initial CI and CLI/Agent coverage queries.

Deliverable: reusable matrix-lock and parity machinery.

### K4 — automated pre-vertical M1/M2/M3 census

Purpose:

Use the live Neo4j Bimba Map, `Epi-Logos-C-Experiments/Idea/Bimba/Map/**`, vendored/imported C, current Rust, existing deep matrices and current QL-MEF docs to build the initial row-level coverage ledger.

For every M1/M2/M3 coordinate/capability:

```text
locate Bimba identity
locate parent/children and relations
locate C representation
locate Rust representation
classify missing/partial/misplaced implementations
identify stale or conflicting material
record current authority rather than silently choosing
identify required C++ embodiment where applicable
```

This phase should be heavily automatable. The Bimba relational body is large enough that agents should query and join it rather than reconstructing subsystem topology from prose.

Deliverable: canonical initial M1/M2/M3 coverage ledgers and generated implementation work graph.

## 10. Vertical implementation phase

After K0–K4, development proceeds by coherent coordinate verticals.

The default vertical loop is:

```text
select coordinate / coherent branch
        ↓
resolve complete Bimba/source field
        ↓
lock/reconcile C structure + relations
        ↓
seat/port/align Rust computation
        ↓
prove parity
        ↓
where relevant, embody in C++
        ↓
reconcile Neo4j projection
        ↓
exercise through deep/current-event surfaces
        ↓
return evidence/discrepancy to M ledger
```

This is not a rigid one-direction build order. Returned evidence may move backward through the loop.

### K5 — M1 vertical rebuild

Initial top branch map:

```text
M1-0  Bimba / Original
M1-1  Pratibimba / Reflection
M1-2  Ananda
M1-3  Spanda
M1-4  QL flowering
M1-5  Toroidal recognition
```

The C source already contains substantial Ananda/Spanda/topological machinery; Rust already contains substantial matheme/music/QL computation. K5 seats and completes these against exact Bimba coordinates rather than redesigning Paramaśiva from scratch.

C++ target: continuous formal/harmonic/topological embodiment sufficient for the M1 engine and M1′ instrument.

### K6 — M2 vertical rebuild

Initial top branch map:

```text
M2-0  numerical/potentiation ground
M2-1  MEF / Vimarśā
M2-2  36 Tattvas
M2-3  36 Decans
M2-4  symbolic-musical Power
M2-5  planetary/chakral synthesis + M3 threshold
```

Preserve actual asymmetric deeper branches.

The C source already contains the 72 carrier through MEF/Tattva/Decan/Shem and other M2 structures; Rust already has strong MEF and exact TemplateureField work. K6 establishes coordinate-complete Paraśakti coverage and fills actual C/Rust gaps.

C++ target: physical/modal resonator, wave, acoustic and cymatic embodiment keyed to exact M2 state rather than a parallel audiovisual model.

### K7 — M3 vertical rebuild

K7 must treat the 64 as one address register inside the much richer Mahāmāyā machine.

Known current structural field includes:

```text
4-state nucleotide/element alphabet
16 dinucleotide relations
three matrix families
64 codon/hexagram addresses
384 line-change/change relations
472 lawful orientations
DNA/RNA/transcription relations
Tarot exact-cover/expression relations
360 dynamic degree carriers
24 structural backbone carriers
720° double-cover relation
```

The C body already contains a deep transcription/clock implementation. Rust already carries substantial codon/I-Ching/Tarot/rotation/fold/quaternion computation. K7 establishes exact coordinate coverage and restores missing clock/transcription parity without reducing M3 to the presently easiest subset to inspect.

C++ target: full clock/form/transcription embodiment, including the spatial/physical consequences of matrix, fold, orientation and clock state.

## 11. C++ and deep-instrument programme

### K8 — shared C++ computational substrate

Build the common continuous runtime once:

```text
realtime state handoff
geometry/material-coordinate infrastructure
modal/physics substrate
audio engine
GPU/render substrate
interaction/picking
instrument diagnostics
kernel ref/provenance transport
```

Then implement M1Engine, M2Engine and M3Engine as coordinate-backed domain engines on that substrate.

Do not require all six M roots to be fully embodied before useful C++ work begins. C++ work begins when a requested coordinate vertical has its structural/operational contracts locked enough to consume safely.

### K9 — deep M′ instruments and M1/2/3 composition

Use the same domain engines for:

```text
M1′
M2′
M3′
```

and compose them into the singular M1/2/3 physical/cosmic object around the same current event.

Prompt-D/Prompt-E event identity, deep-open/return, readiness/provenance and human/Agent co-reference laws remain valuable conformance requirements. Their older implementation-history framings do not override this kernel architecture.

A deep instrument resolves through at least:

```text
eventRef
+ exact MCoordinateRef
+ kernel/ledger revision
```

and returns to the exact parent relation without coordinate or event drift.

## 12. Wider M closure

### K10 — extend the same method through M0/M4/M5 and whole-M closure

M1/M2/M3 are the immediate physical/deep proving ground, not a special architecture.

The same registry, ledger, C/Rust parity and projection machinery must extend across M0, M4 and M5. K10 closes the whole-M coverage field and lets Personal 4/5/0, current-event and wider Epi/O:I work consume the same stable kernel rather than carrying separate subsystem registries.

## 13. Parallel workstreams after K3

After the foundational contract exists, work can proceed concurrently:

```text
A  kernel form / canonical relations
B  native C Bimba port + structural parity
C  Rust operational completion + provider/service work
D  C++ embodiment + deep engines
E  Neo4j/current-event/application projection
```

The M ledger is the joining surface.

No stream is allowed to repair another stream's missing semantic law by silently inventing a local substitute.

## 14. Known / partial / unresolved boundary

### Firm enough to build against

- one QL kernel;
- coordinate-based migration;
- M as first-class whole with M0–M5 recursive roots;
- C as structural Bimba lock;
- Rust as operational/computational layer;
- C++ as continuous computational embodiment;
- Neo4j as canonical deep-coordinate semantic/relational home;
- matrix/ledger locking and multi-axis parity;
- M1/M2/M3 capability matrices as current starting maps;
- the integrated M1→M2→M3 physical-pole direction;
- structural changes requiring C reconciliation once promoted to kernel canon.

### Known current implementation facts

- native C current-main state is materially thinner than intended;
- useful native C foundation exists on the closed #76 line;
- imported/vendored C contains rich M0–M5 computation;
- current Rust contains strong but differently-organised portions of M1/M2/M3 and generalized QL/MEF;
- Bimba Map data exists both as live Neo4j structure and substantial serialized/web-accessible datasets.

### K4 census must establish rather than assume

- complete M0–M5 coordinate inventory;
- exact C coordinate assignment of every imported semantic table/algorithm;
- exact Rust assignment of every current module;
- complete C↔Rust parity per branch;
- which imported C laws remain correct as-is;
- which require current canonical correction;
- every cross-coordinate relation currently implicit in implementation;
- exact completeness of Rust M1 Ananda/Spanda, M2 Tattva/Decan/Power/synthesis and M3 full clock/transcription ports.

### Still implementation-design work

- final C registry struct layout and generated-definition format;
- exact C ABI/versioning details;
- exact Rust↔C calling/generated-fixture arrangement;
- C++ libraries, render/audio/physics backend choices;
- in-process versus IPC boundaries for continuous runtime components;
- final interaction design of each M′ instrument.

Do not resolve these prematurely where the ledger/vertical evidence can decide them more cleanly.

## 15. Whole-programme acceptance laws

The programme is healthy when all of the following are mechanically defensible:

1. every canonical semantic capability has an exact coordinate or explicit cross-coordinate disposition;
2. every canonical coordinate has a stable identity shared across implementations;
3. M is a first-class master tree with M0–M5 aggregate roots and source-faithful recursion;
4. every substantial semantic C construct is coordinate-bound or explicitly cross-coordinate;
5. C is the lowest executable structural Bimba lock;
6. promoted structural change cannot bypass C reconciliation/parity;
7. Rust operationalises the same field rather than authoring a parallel M ontology;
8. C++ consumes exact kernel refs and does not mint symbolic identity locally;
9. Neo4j canonical deep coordinates resolve to kernel coordinate identity and known revisions;
10. provisional/research graph relations remain possible without being misreported as bare-metal canon;
11. the capability matrices retain known source capabilities even where implementation is missing;
12. missing implementation remains visible in the ledger;
13. source, coordinate, relation, operational and experiential parity remain separate evidence classes;
14. warrant and readiness remain separate;
15. deep M′ instruments preserve event + coordinate identity through open/operate/return;
16. M1/M2/M3 composition consumes the full coordinate-backed engines rather than a reduced integration vocabulary;
17. human and Agent access resolves to the same refs/actions/state where implemented;
18. current-event state remains distinct from kernel possibility and presentation state;
19. returned implementation or graph reality may revise architecture only through an explicit recorded reconciliation;
20. no future Agent needs to reconstruct the kernel's M topology from scattered code or tickets before doing ordinary coordinate work.

## 16. Ticket graph

This Wayfinder owns the current programme ladder:

```text
K0  current ground / source authority reconciliation
 ↓
K1  restore native C kernel centre
 ↓
K2  recursive M registry + master M index
 ↓
K3  capability ledger / matrix lock / parity protocol
 ↓
K4  automated M1/M2/M3 pre-vertical census
 ├──────────────┬──────────────┐
 ↓              ↓              ↓
K5 M1          K6 M2          K7 M3
 └──────────────┴──────────────┘
                ↓
K8  shared C++ substrate + domain engines
                ↓
K9  M′ instruments + M1/2/3 composition
                ↓
K10 whole-M extension / M0-M4-M5 closure
```

After K3, later work may overlap whenever ledger dependencies permit. The arrows express dependency/closure logic, not a demand for strictly serial implementation.

## 17. Relation to existing programme material

Retain useful technical work, data, parity evidence and conformance laws from #51, #78, #69, #73 and related PRs/docs.

Where those materials describe the imported C/Rust bodies primarily as historical specimens, prototype evidence or a one-way migration into Rust, this Wayfinder records the clarified current development intention:

**both C and Rust were brought forward as implementation bodies of the kernel rebuild; C is to become the structural Bimba lock, Rust the operational kernel, and the existing bodies are to be moved, aligned, corrected and completed rather than treated as disposable source archaeology.**

No prior issue history needs to be rewritten to pretend this clarification was always explicit. New implementation work should simply follow this Wayfinder.

## 18. Development posture

The practical method is deliberately conservative about loss and aggressive about automation:

```text
query the coordinate field
      ↓
join existing Bimba + C + Rust evidence
      ↓
record discrepancy instead of guessing
      ↓
resolve the smallest coherent coordinate vertical
      ↓
prove parity
      ↓
return the result to the ledger
```

The depth already exists. The task is to make it structurally impossible for that depth to disappear merely because a later Agent saw only one projection of it.
