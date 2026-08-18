# Epi-Logos / QL-MEF — Grounding Account and System Index

**Status:** working architecture / human review  
**Primary development Wayfinder:** QL-MEF #30  
**R2 relation research:** QL-MEF #25 / PR #27  
**Source coordination:** Epi-Logos-C-Experiments #2, #3, #4  
**M/M′ field:** `epi-relational-field.csv`  
**S/S′ field:** `epi-ssprime-relational-field.csv`  
**O:I field:** O:I #29 / `data/ql-relational-field.csv`

This is the entrypoint for understanding and developing Epi-Logos through QL-MEF and the native O:I product field.

The integration area is **one grounded relational development system**. Its prose establishes meaning and authority; its three QL matrices keep the domain, technical stack and O:I suite perceptible together; its source/substrate inventory ties those relations to real code and data; its Wayfinder describes how development proceeds through the same system.

---

## 0. How this integration area works

### Grounding documents

| Artifact | Job |
|---|---|
| **this file** | whole-system account, index and relation between the three fields |
| `EPI-LOGOS-QL-MEF-FOUNDATION.md` | QL / MEF / harmonic / musical substrate and Epi genealogy |
| `EPI-OI-PRIMITIVE-OWNERSHIP-MAP.md` | semantic dictionary for Epi/S concerns relative to current O:I primitives and owners |
| `EPI-LOGOS-SOURCE-SUBSTRATE-INVENTORY.md` | exact source/code/data bodies, implementation status, provenance and parity evidence |
| `EPI-LOGOS-RECONSTITUTION-DISPOSITION.md` | what must survive, what may be rebuilt, and what evidence permits retirement |
| `EPI-LOGOS-DEVELOPMENT-WAYFINDER.md` | development practice through the three relational fields |

### Three interoperating 12×12 relation fields

```text
M/M′ — EPI DOMAIN / PRODUCT FIELD
M0..M5 / M0′..M5′
        ↓ capabilities are embodied by
S/S′ — EPI TECHNICAL / INHABITATION FIELD
S0..S5 / S0′..S5′
        ↓ generic mechanics operate through / conform to
O:I — TECHNOLOGICAL PRODUCT FIELD
H0..H5 / A0..A5
```

The canonical manipulable fields are:

- `epi-relational-field.csv` — M/M′;
- `epi-ssprime-relational-field.csv` — S/S′;
- O:I `data/ql-relational-field.csv` tracked by O:I #29 — H/A.

All three share the same core schema:

```text
id
src_product
dst_product
ql
coverage
cf_view
seam
defined_in
tracked_by
```

and the same native QL harmonic grammar:

- A/B/C relation families;
- D1/D2/D3 conjugate relations;
- CF1–CF7 contextual readings where germane;
- `H/S/L/W/I` developmental coverage.

Their shared form is what permits cross-field contemplation. It does **not** make same-numbered terms identical.

```text
M/M′ != S/S′ != O:I H/A
```

### The bridges between the fields

`epi-ssprime-embodiment.json` is the capability bridge from M/M′ into S/S′. For each M′ capability it records:

- Epi domain owner and capability ref;
- S/S′ strata required to embody it;
- authority that must remain native;
- likely modern generic O:I homes;
- current migration/reuse reading.

It is not a fourth ontology. It answers **which technical strata embody this domain capability?**

`EPI-OI-PRIMITIVE-OWNERSHIP-MAP.md` then answers **which O:I primitive/product should own the generic mechanics exposed by those S/S′ concerns, and what Epi meaning remains distinct?**

So normal traversal is:

```text
M/M′ relation + capability
        ↓
epi-ssprime-embodiment.json
        ↓
S/S′ relation neighbourhood
        ↓
O:I relation neighbourhood + primitive ownership
        ↓
actual source / code / data / provider
        ↓
vertical implementation + evidence
        ↓
returned remapping
```

### Research/provenance companions

`EPI-OI-CROSS-DOCUMENT-MATRIX.md`, `epi-mmprime-relations.json`, the R2 first/second/final passes and the other JSON maps explain how current relation readings were reached. They are **provenance and deeper analysis**, not parallel systems that a normal development session must read end-to-end.

---

## 1. What Epi-Logos is for

Epi-Logos attempts to make the psychoid/formal field computationally and experientially available so that human and technological agency can encounter, inspect, question, compose and return through it with increasing quality, skill and precision in the service of human flourishing.

There is already substantial executable formal/symbolic computation toward that end. That is an implementation fact.

The stronger proposition — that these computations veridically track a psychoid level of reality and that acting through them improves human flourishing — remains a research proposition to be tested through returned reality.

Keep distinct:

- authored theoretical/source position;
- accepted architecture/design commitment;
- implementation fact;
- observed result;
- research proposition;
- migration/development inference.

Returned reality may revise authored understanding, but only explicitly.

---

## 2. Whole relation with O:I

Begin product-meaning work from current O:I `docs/positions/FOUNDING-POSITIONS.md` and QL-MEF #7.

O:I supplies the general technological field of Objective Internality: authored ground, agency, operative composition, development, material execution, formal/reflexive intelligence and shared relation.

QL-MEF is the independently developed package locus for executable Quaternal Logic, MEF, relation/conjugation, refraction, harmonic/music theoretics and related formal intelligence. Its Bimba placement is genealogical/ontological placement, not a demotion of engineering authority.

Epi-Logos is the fuller psychoid/formal constitution and lived field operating through that technological ground:

```text
O:I technological-agency Cradle
Central · Actuation · AIKit · Factory · Workcell · shared field
                         +
QL-MEF executable QL / MEF / harmonic / musical substrate
                         ↓
Epi M/M′ domain constitution
Bimba · six domains · six canonical Agents
                         ↓ embodied through
Epi S/S′ technical / inhabitation constitution
execution · residency · relation · time · agency · return
                         ↓
M0′–M5′ Pratibimba instruments + parent 0/1 field
                         ↓
encounter / action / evidence / explicit return
```

Do **not** collapse two senses of runtime:

- generic technological runtime — sessions, harnesses, capabilities, providers, processes, execution — increasingly belongs to native O:I products;
- Epi formal execution — Bimba/QL/MEF/M0–M5 computation in C/Rust or successor bodies — is domain computation. O:I may host/materialise it without owning its semantics.

---

## 3. QL-MEF and the common harmonic grammar

The same QL grammar structures all three relation fields.

At minimum the living foundation includes:

```text
# / 0-1 generative parent
QL positions and 4+2 / 5→0 return
A/B/C relation families
D1/D2/D3 conjugation-degree semantics
12 MEF lenses = 6 + 6′
three complementary V4 / lens-square structures
seven Context Frames
12 × 7 = 84 lens-mode / mode-tonic field
foundational harmonic ratios
coordinate ↔ pitch relations
voice-leading / traversal / rhythm
8+4 audio/nodal and cymatic relations
later microtonal/cymatic operators where evidence supports them
```

The matrices use those harmonics as a development grammar, not as decorative labels.

The formal system having a Bimba location does not reduce it to a correspondence entry or UI page. The coordinate carries genealogy; QL-MEF develops the executable power.

Current Q1/Q2 boundaries are accepted implementation **now**, not the complete QL-MEF destination. See `EPI-LOGOS-QL-MEF-FOUNDATION.md`, #31, #39 and #49.

---

## 4. M / M′ — domain and lived product field

Canonical relation:

```text
M  = Bimba / canonical coordinate image
M′ = Pratibimba / lived-reflected operation of that image
```

The six domains are:

```text
M0 / M0′  Anuttara
M1 / M1′  Paramasiva
M2 / M2′  Parashakti
M3 / M3′  Mahamaya
M4 / M4′  Nara
M5 / M5′  Epii
```

M′ is not a generic view layer. It is the experiential, visual, sonic and operational face through which the corresponding domain is lived.

The parent Epi application is the **non-numbered 0/1 field**, not a seventh stage:

```text
                       EPI 0/1
                   whole lived field
                         /   \
                        /     \
                 COSMIC       PERSONAL
              M1′ M2′ M3′   M4′ M5′ M0′
                        \     /
                         \   /
                    return / renewal
```

The six M′ workspaces remain full-depth instruments in the wider 4+2 relation.

`epi-relational-field.csv` keeps the twelve M/M′ faces available as one QL field rather than six isolated product specs.

### Lived instruments

**M0′ — Anuttara:** playable Bimba/coordinate field, source/provenance navigation, relation visibility, personal/pedagogical routing.

**M1′ — Paramasiva:** mathematical-musical engine as instrument: relation→interval, position→pitch, traversal→phrase; Ananda, Spanda, QL flowering, lens/mode and toroidal/Hopf structures where source-supported.

**M2′ — Parashakti:** Vimarśā/MEF/cymatic instrument: 72-fold field, audio genesis, resonance, nodal/cymatic standing-wave expression and source-backed correspondences.

**M3′ — Mahamaya:** symbolic/time/codon/world-clock transcription instrument expressing 64-fold/rotation/time structures over shared upstream state.

**M4′ — Nara:** protected personal-Pratibimba field: identity, current condition, activity/pattern, composed lived context, journal/dream/oracle/highlight/episode continuity, explicit review gates and privacy.

**M5′ — Epii:** recursive pedagogical/developer instrument: Epi-specific teaching, source/canon recognition, Logos return, subsystem evaluation and self-articulation.

---

## 5. S / S′ — technical embodiment and inhabitation field

The current S Seed index defines:

```text
S0 makes the system executable
S1 makes it resident and typed
S2 makes it graph/vector/cache real
S3 makes it temporal and routed
S4 makes it agentically inhabited
S5 makes it world-facing and reflective
S5 returns to S0
```

The S-family is therefore a **return circuit**, not a list of services.

The twelve S/S′ faces are:

| Base face | Base function | Prime face | Prime augmentation |
|---|---|---|---|
| S0 | execution / command / process / kernel materialisation | S0′ | **Khora** — grounded runtime/tool/world surface |
| S1 | files / vault / artifact residency | S1′ | **Hen** — compiler, CT, frontmatter, form/residency law |
| S2 | graph / vector / cache / retrieval substrate | S2′ | **Pleroma** — coordinate-aware relation/retrieval law |
| S3 | gateway / session / channel / temporal routing | S3′ | **Chronos** — Day/NOW/Kairos/presence/context law |
| S4 | agent runtime / providers / skills / permissions / teams | S4′ | **Anima** — VAK/CF/CFP routing and inhabited agency law |
| S5 | world-boundary / knowledge-return / review services | S5′ | **Epii return law** — review, pedagogy, improvement, promotion and Möbius return |

Aletheia remains load-bearing but is **not a seventh S face**. It is the crystallisation/disclosure/rehearing membrane around the S4.5′→S5′ return seam by which attributable encounter residue becomes reviewable rather than silently canonical.

`epi-ssprime-relational-field.csv` makes these twelve faces available as the same complete 12×12 QL field used at the M and O:I scales.

The common four-seam inhabitation pattern remains:

```text
DISCLOSE / INJECT
RECEIVE / EXTRACT
COMPILE / RECOMPOSE
QUERY
```

It is a recurring relation across S′, not yet a reason to invent a universal O:I root primitive.

---

## 6. M/M′ ↔ S/S′ — capability embodiment

The bridge is not numeric identity.

```text
M0 does not equal S0
M1 does not equal S1
...
```

An M/M′ capability may require one, several or all S strata.

`epi-ssprime-embodiment.json` carries this relation explicitly. Examples already grounded there include:

- M0 relation inspection → primarily S2/S2′;
- M0 time/community → S2/S2′ + S3/S3′;
- M1 canonical harmonic engine → S0/S0′;
- M1 Spanda → S0/S0′ + S3/S3′;
- M2 Vimarśā audio → S0/S0′;
- M2 source/correspondence → S0/S0′ + S2/S2′;
- M3 clock/world-clock → S0/S0′ + S3/S3′;
- M4 identity → S0/S0′ + S1/S1′ + S3/S3′;
- M4 day/episode → S1/S1′ + S3/S3′;
- M4 promotion seam → S4/S4′ + S5/S5′;
- M5 canon → S1/S1′ + S5/S5′;
- M5 backend/reflected-app concerns → all six S strata.

This bridge is how product meaning becomes technical architecture without pretending the technical layer owns the domain.

---

## 7. S/S′ ↔ O:I — native ownership and conformance

The S field is the intermediary that makes migration/reconstitution clean.

It lets us ask first **what technical function Epi actually needs**, then **which current O:I product owns the generic mechanics**, rather than mapping M domains straight onto O:I product names.

Current generic ownership remains:

| Concern | Native owner | Epi/S relation |
|---|---|---|
| authored human/operative ground | Central | source ground; S1 residency may bind to it without taking authority |
| Agent / Agency / WorldBinding / Return | Actuation | S4/S4′ agency and return operate through these semantics |
| Context / Knowledge / Profile / SkillSet / Capability / model / harness / SessionSpace / Component / Surface | AIKit | S2/S3/S4 generic operative composition and instrument exposure |
| Project / Run / Artifact / Evidence / Candidate / Recognition | Factory | developmental execution/evidence across S0–S5 |
| provider / process / service / store / audio / graph / render body | Workcell | material body beneath S0/S2/S3/S4 concerns |
| relation / conjugation / MEF / harmonic formalism | QL-MEF | formal substrate used across M, S and O:I readings |
| Projection / Participant / SharedField / Contribution / Encounter | O:I parent | world-facing/shared S5 relations and external encounter |

`EPI-OI-PRIMITIVE-OWNERSHIP-MAP.md` carries the detailed semantic non-identities and specialisation/bridge decisions.

---

## 8. Ta-Onta and the agentic inhabitation seam

The older Ta-Onta bodies are not a second generic runtime that must survive literally.

Their enduring contribution is the Epi constitution of the S′ face:

- Khora — grounded execution/world establishment;
- Hen — lawful form/residency;
- Pleroma — relational/capability affordance substrate;
- Chronos — temporal/Kairos condition;
- Anima — VAK/CF/CFP differentiation and dispatch;
- Epii-return — review, pedagogy, improvement and return;
- Aletheia — crystallisation/disclosure membrane inside the return path.

Generic session, capability, provider, process and orchestration mechanics should operate through current O:I owners. Epi retains the reasons those powers are selected, related, timed, interpreted and returned.

`TemporalCondition`, disclose/receive/recompose/query and `Uptake` remain research pressures unless broader evidence justifies promotion.

---

## 9. Central ↔ Nara

Central is the human-owned sparse source root. Nara is a structured protected personal-internality model.

Preserve:

```text
Authored source ≠ Observation ≠ Inference ≠ Derived state ≠ Proposal
```

Working relation:

```text
Central human-owned ground
        ↓ PersonalGroundBinding
Nara identity + lived state
        ↓ observation / inference / proposal
human recognition
        ↓
renewed durable ground where appropriate
```

Activity, inference, model return and agent confidence cannot silently become durable identity/source.

The M4 capability bridge shows why this relation crosses S0/S1/S3 for identity and lived state, then S4/S5 for proposal/review.

---

## 10. Six canonical Epi Agents

Anuttara, Paramasiva, Parashakti, Mahamaya, Nara and Epii remain canonical persistent **M-domain Agent identities**.

They are not aliases for the six O:I products and are not the same thing as the six S′ prime functions.

Their generic body is composed through:

```text
Epi Agent constitution
    ↓
Actuation Agent / Agency / WorldBinding
    ↓
AIKit Profile / SkillSet / Context / model / harness / SessionSpace
    ↓
Workcell body where material execution is required
    ↓
Factory Run/Evidence when activity is developmental
    ↓
Epi/Aletheia/owning-source return
```

S4′ Anima names the inhabitation/dispatch function through which such agents may be differentiated in an act; it does not replace their M-domain identities.

---

## 11. Historical shells and reconstitution

The preservation target is the **system**, not historical package identity.

- frozen Theia — historical function/interaction specimen;
- current Pratibimba app — real current evidence, but rebuildable;
- broad `epi` CLI — may disappear, become a thin passthrough/alias, or retain only genuine domain execution;
- mixed S3 gateway/session/provider bodies — split Epi temporal/routing meaning from generic mechanics and converge the latter after parity;
- C `epi-lib`, Rust `portal-core`, formal operators, datasets and invariants — computational/domain assets; language/package may change, meaning/provenance may not silently change.

The S matrix makes shell retirement safer because each old body's behaviour can be relocated into the technical relation that actually justified it, then mapped to its native O:I owner.

Use `EPI-LOGOS-RECONSTITUTION-DISPOSITION.md` and the source inventory to make those decisions.

---

## 12. High-risk non-identities

```text
M/M′ domain identity ≠ S/S′ technical stratum identity
S/S′ technical identity ≠ O:I product identity
M/M′ ≠ O:I H/A same-product face conjugacy
Pratibimba ≠ O:I Projection ≠ AIKit Projection ≠ WorldPresentation
AIKit Profile ≠ MathemeProfile ≠ Nara PersonalIdentityProfile
AIKit Context ≠ Nara PersonalField ≠ O:I SharedField ≠ Workcell world
HumanIdentity ≠ Central Control prose ≠ Nara IdentityModel ≠ Participant
Bimba Graph ≠ QL-MEF Meta-Knowledge Graph ≠ AIKit Knowledge graph
Encounter ≠ understanding / memory / belief / phenomenality
Agent ≠ Agency ≠ AgentSession ≠ model/harness/body
M-domain Agent ≠ S′ constitutional function
Bimba coordinate placement ≠ package/runtime ownership
formal model return ≠ human-authorised source mutation
```

These distinctions protect semantic authority while the common QL fields let us explore genuine complements.

---

## 13. Development through the three fields

Do not leave the grounding account and invent a separate development ontology.

```text
actual concern / capability / discrepancy
        ↓
GROUND in relevant M/M′ + S/S′ source
        ↓
LOCATE focal M/M′ relation + harmonic neighbours
        ↓
EMBODY via capability refs in epi-ssprime-embodiment.json
        ↓
LOCATE relevant S/S′ relation + complements/return
        ↓
REFRACT through corresponding O:I #29 neighbourhood
        ↓
OWNERSHIP via primitive map
        ↓
REALITY via source/substrate inventory and current code/data
        ↓
RECONSTITUTE smallest coherent vertical slice
        ↓
PROVE operator/data/agent/instrument/authority parity as germane
        ↓
RETURN / REMAP only what reality changed
```

QL relations **generate questions, not obligations**.

A complete field does not mean every cell becomes an integration. The matrices exist so a real concern can be held against its conjugates, complements, requirements and returns before code is changed.

See `EPI-LOGOS-DEVELOPMENT-WAYFINDER.md` for the working procedure.

---

## 14. Current development facets

#30 coordinates facets of the same field:

- #31 / #39 / #49 — QL-MEF formal, harmonic and musical substrate;
- #32 / #46 / #47 / #48 — S/S′↔O:I inhabitation, primitive and candidate-contract research;
- #33 / #38 / #44 / #45 — computation, source/data authority and parity;
- #34 / #40 — M′ instrument reconstruction and shell retirement;
- #35 — Central↔Nara personal internality;
- #36 — non-numbered parent 0/1 experience;
- #37 — Epii/evidence/return/human authority;
- #41 — Pratibimba↔Projection/shared-field boundary;
- #42 — six canonical M-domain Epi Agent materialisation.

These are not stages. A single real capability may traverse several at once.

---

## 15. Reading discipline

For any development task, take the smallest sufficient route through the same system:

1. current O:I founding positions where whole/product meaning is involved;
2. this grounding account;
3. relevant M/M′ Seed/source;
4. focal `epi-relational-field.csv` neighbourhood;
5. `epi-ssprime-embodiment.json` capability relation;
6. focal `epi-ssprime-relational-field.csv` neighbourhood and relevant S/S′ Seed;
7. corresponding O:I #29 matrix neighbourhood where native ownership matters;
8. primitive ownership map;
9. exact source/substrate inventory and live implementation;
10. relevant Wayfinder ticket only once the concrete pressure is clear.

Stop when enough context has been recovered to make the change correctly.

Current code tells us what is real now. It does not retroactively define why Epi exists. Authored source tells us what is meant. It does not prove what works. The three relation fields let meaning, embodiment and current technological reality be considered together without collapsing them.
