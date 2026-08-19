# Epi Deep Subsystem Capability / Coordinate Matrix Protocol

Status: active research / conformance protocol  
Tracks: #25, #30, #38, #73, #74, #75  
Epi source owner: `EpiLogos/Epi-Logos-C-Experiments`  
QL-MEF role: living relation / refraction / conformance map, not semantic ownership

## Purpose

This protocol exists to stop subsystem reconstitution from collapsing into whichever recent plan, implementation body, renderer, provider, or compressed product summary happens to be easiest to read.

The task for each `Mx` is first to recover the **subsystem itself** as a deep capability / coordinate field:

- what human or agent concern the subsystem exists to answer;
- what its recursive coordinate tree contains;
- what each branch does inside the subsystem;
- what each branch contributes to the Epi whole;
- what a person can experience, inspect, manipulate, enact, or learn there;
- what formal/computational laws are genuinely load-bearing;
- what evidence or state enters and leaves the branch;
- what privacy, authority, safety, provenance and review constraints belong to it;
- which technical choices are settled, which are only historical proposals, and which remain open.

Only after that field is intelligible do we map its capabilities into the same-coordinate fourfold ratified by #74:

```text
M_i   semantic / ontological subsystem + guardian identity
  ↓
S_i   standing technological reflection / O:I product
  ↕
S_i′  operative technological reflection / Ta-Onta module
  ↓
M_i′  deep compositional Pratibimba
```

The fourfold is therefore a **projection target and constitutional relation**, not a shortcut for discovering what the subsystem means.

## 1. Source law — chronology is not semantic precedence

No generic rule of the form `newer document > older document` is permitted.

A later plan may be more explicit because it was generated from an older specification. It may also have lost features, collapsed distinctions, inherited an implementation convenience, or converted an open design question into an accidental assumption. Conversely, an older specification may preserve the most complete feature signal while containing obsolete shell or technology choices.

Every source claim is therefore classified by **role**, not by age alone.

### Source-role classes

- `FOUNDATIONAL-SIGNAL` — original/canonical authored subsystem purpose, feature field, coordinate structure, philosophical or experiential intent.
- `CANONICAL-FEATURE-MATRIX` — previously accepted capability/feature inventory whose purpose is loss detection and coverage.
- `CURRENT-AUTHORED-POSITION` — current explicit authored position or accepted correction.
- `CURRENT-DOMAIN-SPEC` — current specification of the subsystem/domain; may contain unresolved or inherited tech assumptions.
- `DESIGN-COMMITMENT` — explicitly ratified product/UX/architecture choice.
- `RESEARCH-PROPOSITION` — promising but unratified technical/formal/experiential proposal.
- `IMPLEMENTATION-FACT` — code/data/tests that exist now.
- `OBSERVED-RESULT` — behaviour/evidence actually demonstrated.
- `HISTORICAL-IMPLEMENTATION-EVIDENCE` — old body/shell/provider useful for recovering behaviour or feature richness.
- `SUPERSEDED-TECH-ASSUMPTION` — implementation choice no longer authoritative.
- `UNRESOLVED-CONTRADICTION` — competing source claims that have not been ratified away.

A single document may contribute claims in several classes.

## 2. Recovery order

For each subsystem, perform these passes before cross-stack embodiment design.

### Pass A — authored meaning and complete feature signal

Read, in parallel rather than by a one-dimensional precedence ladder:

1. original/canonical subsystem specification(s);
2. canonical feature/capability matrices and source-derived datasets;
3. current active domain spec and architecture/UX documents;
4. current Cycle-3/deep research and development material;
5. old implementation plans where they preserve feature or interaction detail;
6. current code/tests only to establish what is real now.

The output of Pass A is a **union-with-provenance**, not a winner-takes-all summary.

A capability found only in an old canonical feature source is not dropped because a later plan omitted it. It is retained as `FOUNDATIONAL-SIGNAL` until deliberately superseded with a reason.

### Pass B — recursive coordinate recovery

Recover the complete known tree for the subsystem:

```text
Mx
  Mx.0
    Mx.0.0 ... Mx.0.5
    deeper descendants where source-backed
  Mx.1
  ...
  Mx.5
```

For every node record:

- local role inside its parent branch;
- relation to sibling branches;
- relation to the subsystem's `.0/.5` boundary pair;
- relation to the subsystem's 1–4 explicate/internal activity;
- contribution to 3:3 parent composition (`1/2/3` or `4/5/0`) where relevant;
- whole-system relation to other M domains;
- whether the node is semantic, computational, experiential, procedural, representational, or some combination.

Do not infer missing sixfold children merely for symmetry. Mark absent or unknown children explicitly.

### Pass C — experience / UX / agent-use recovery

For every material capability, record what changes for a human or situated agent:

- what can be perceived;
- what can be manipulated or enacted;
- what can be authored or changed;
- what can be explained or inspected;
- what can be replayed/revisited;
- what can be delegated to an agent;
- what is protected or unavailable;
- what deeper context can be summoned from it.

A capability is not considered understood merely because its data structure or computation is known.

### Pass D — formal / computational / data law

Record genuine invariants separately from proposed implementations:

- equations/operators/coordinate relations;
- state lifecycle;
- composition law;
- input/output/effect contract;
- authority and source-of-truth;
- provider/observation distinction;
- deterministic vs inferential steps;
- privacy/safety/consent/review requirements;
- readiness/degradation semantics.

### Pass E — technical decision register

Every technology-shaped statement receives a decision status:

- `RATIFIED` — explicit current design commitment.
- `IMPLEMENTED-CURRENTLY` — real current body, without implying future authority.
- `CANDIDATE` — plausible body/provider/algorithm.
- `HISTORICAL-CANDIDATE` — old design worth preserving as evidence.
- `REPLACED` — explicitly superseded.
- `OPEN` — no decision yet.

Examples that MUST NOT be silently promoted by source archaeology include:

```text
BLAKE3 as semantic identity
BLAKE3 as privacy/address compression
which Nara identity layers contribute to Q_identity
whether q_personal and Q_identity are the same object
quaternion axis order / normalization law
Graphiti as canonical episodic store
Neo4j as Bimba identity
Theia/Tauri/O:I Canvas as subsystem identity
specific GPU/audio/rendering libraries
specific model/training methods
```

If the source corpus disagrees, the matrix records the disagreement and the decision remains `OPEN` until ratified.

### Pass F — current reality and gap

Only after the semantic/experiential field is recovered, compare against current implementation:

```text
RECOVERED CAPABILITY
CURRENT BODY / TEST / PROVIDER
STATUS: absent | partial | operative | experimentally evidenced
LOSS / DRIFT / EXTRA IMPLEMENTATION
NEXT DECISION OR PARITY PROOF
```

Current code tells us what is real now; it does not retroactively define why the subsystem exists.

### Pass G — M′ / S / S′ projection

Only after Passes A–F are materially stable do we populate cross-register embodiment.

For each capability record:

```text
M semantic home
M′ deep instrument expression
S standing technological reflection / native product capabilities
S′ operative Ta-Onta contribution
cross-coordinate S/S′ dependencies
native owner / write authority
provider/body candidates
```

The same-index `M/S/S′/M′` relation from #74 is the constitutional spine. Cross-coordinate capability embodiment remains legal and expected.

Do not use S/S′ mapping to redefine the subsystem feature set.

## 3. Canonical per-capability row

Each deep subsystem matrix uses at least these fields:

```yaml
capability_ref: stable id
coordinate: Mx[.y...]
parent_coordinate: optional
name: human-readable name

meaning:
  purpose: why it exists
  subsystem_role: what it changes inside Mx
  whole_system_role: what it changes in Epi as a whole

experience:
  human_affordances: []
  agent_affordances: []
  summonable_depth: []
  default_surface_character: optional

functional_contract:
  inputs: []
  outputs: []
  state_effects: []
  invariants: []
  deterministic_steps: []
  inferential_steps: []

relations:
  upstream: []
  downstream: []
  sibling_relations: []
  parent_3x3_relation: optional
  boundary_relation_0_5: optional

protection:
  privacy: optional
  authority: []
  consent: []
  review_promotion: []

source_evidence:
  - ref: path/revision
    role: SOURCE-ROLE
    claim: concise claim

implementation:
  current_bodies: []
  observed_evidence: []
  readiness: unknown | absent | partial | operative | accepted

technical_decisions:
  - question: ...
    status: RATIFIED | IMPLEMENTED-CURRENTLY | CANDIDATE | HISTORICAL-CANDIDATE | REPLACED | OPEN
    options_or_evidence: []

cross_register_projection:
  status: NOT-YET-MAPPED | PARTIAL | RATIFIED
  m_prime: optional
  s_product: optional
  s_prime: optional
  cross_coordinate_dependencies: []
```

## 4. Per-coordinate summary matrix

The human-readable document should also maintain a compact recursive table:

| Coordinate | Meaning / role | Human/agent experience | Inputs → effects | Relations | Decision state | Evidence |
|---|---|---|---|---|---|---|

This is the loss-detection view. It should make a missing branch or collapsed capability obvious without reading every implementation plan.

## 5. Contradiction / drift register

Each subsystem document has a dedicated register with entries like:

```text
DRIFT-ID
claim A + source role
claim B + source role
what is actually implemented
what has explicitly been ratified
current interpretation
decision needed
```

No contradiction is silently harmonised because a later document happens to be more detailed.

## 6. Verification gate before implementation design

A subsystem is ready for body/provider/UX implementation decisions only when a human reviewer can answer, from the matrix:

1. what the subsystem is for;
2. what its complete known recursive coordinate tree is;
3. what each major branch does locally and systemically;
4. what a human actually experiences or can do;
5. what an agent can address or act on;
6. which features come from foundational/canonical signal versus later proposals;
7. which technical decisions are ratified versus still open;
8. what currently exists in code and what does not;
9. what privacy/authority/review laws constrain it;
10. what must remain true when it is later projected into M′/S/S′.

If these cannot be answered, the next action is more subsystem research, not renderer/provider selection.

## 7. Operating loop across M0–M5

The research/development cadence is:

```text
recover one subsystem deeply
→ produce compact verification packet for human ratification
→ incorporate corrections into its deep capability/coordinate matrix
→ only then update affected current specs/conformance language
→ immediately begin research on the next subsystem
```

The first instantiation is M4/Nara. M5/Epii follows using the same protocol.

This protocol extends the R1 capability-matrix method: the R1 snapshot remains valuable as a six-domain loss-detection inventory, while these per-subsystem matrices descend recursively enough to guide real deep-product refactoring without turning implementation history into semantic authority.