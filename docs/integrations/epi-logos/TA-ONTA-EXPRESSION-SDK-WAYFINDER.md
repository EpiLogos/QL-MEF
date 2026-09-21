# Ta-Onta → Expression SDK/API Wayfinder

**Standing:** owner-directed successor clarification, 15 September 2026.  
**Parent programme:** #135 Kernel Rebuild / lived acceptance.  
**Consumes:** completed #94 Ta-Onta/AW field, accepted K8/K9/K10, current M ledger/capability matrices, O:I Expression Field and `oi.expression/v1`.  
**Does not reopen:** K0–K10, AW0–AW3, the Bimba registry, the O:I renderer, O:I Surface/window semantics, Central source, AIKit generic capability resolution, Actuation generic authority, Factory generic development, or SharedField transport.

---

## 0. Clarification

Ta-Onta is the **Epi-Logos agent-world SDK/API into the general O:I Expression-world substrate**.

The substrate is deliberately paradigm-neutral. O:I can present and operate files, Wiki subjects, pages, Agents, images, glyphs and other native objects in an Expression without requiring QL-MEF.

QL-MEF supplies the Epi paradigm over that substrate:

```text
Bimba / M / M′ / Vāk / current source standing
                         ↓
                     TA-ONTA S′
     Khora · Hen · Pleroma · Chronos · Anima · Aletheia
                         ↓
               Epi domain SDK / profiles
                         ↓
                O:I Expression substrate
                         ↓
          Nara / person / SharedField / pages
```

The point is not to “render QL”. The point is to make the already-ratified agent world **operable through the same live expressive world the person inhabits**.

The generic and paradigm layers must remain distinguishable:

```text
oi.expression/v1          generic application contract
ExpressionProfile         generic material/presentation grammar
SurfacePortal             generic source/page/Surface placement
ExpressionEdition         generic portable two-sided edition

Ta-Onta SDK               Epi semantics over those contracts
Bimba Expression Atlas    Epi profile resolver/address space
M/M′ grammars             Epi dynamic determinations
Nara                       foreground dialogical/voice Agent
Epii                       deeper M5 enrichment/development Agent
```

---

# 1. S′ becomes the concrete runtime/API composition

The existing Ta-Onta functions remain exact. Their relation to the Expression substrate is now explicit.

## S0′ Khora — entry / ground / continuation

Khora resolves the actual Epi world to inhabit:

```text
WorldRef
subject_ref
Bimba coordinate / branch
source revisions
occasion/currentness
eligible Personal relation
ExpressionProfile basis
current Expression or new instantiation
Agent/Nara continuation
material/renderer readiness
```

Khora does not create those owners. It returns one attributable `EpiWorldEntry`/equivalent application reading sufficient for Nara or another Epi Agent to enter the same world coherently.

Required operations:

```text
epi.world.enter
epi.world.restore
epi.world.explain-entry
```

Entry refusal retains the actual unavailable owner/revision/authority reason.

## S1′ Hen — form / profile / artifact

Hen binds semantic/artifact form to the generic Expression profile and verso/page system.

It resolves:

- coordinate/branch profile lineage;
- source standing;
- typed Bimba relation/property form;
- glyph/form bindings;
- page/verso forms;
- authored profile variants;
- form/template birth and source Return;
- Expression Edition/Card specialisations.

Hen does not create another renderer/template engine or page store.

Required operations:

```text
epi.expression.profile.resolve
epi.expression.profile.explain
epi.expression.variant.propose
epi.expression.form.open
epi.expression.form.return
```

## S2′ Pleroma — available body / potency

Pleroma resolves what can actually participate in the present act:

- M′ instruments;
- Expression operations;
- source/Wiki faculties;
- model/harness bodies;
- Nara realtime voice provider/body;
- Epii;
- specialist Agents;
- Skills/Methods;
- native Actions;
- Workcell/material providers;
- relevant Surfaces.

It preserves the distinction:

```text
available != selected != disclosed != permitted != invoked
```

The Ta-Onta layer consumes AIKit/native capability resolution; it does not build another capability registry.

## S3′ Chronos — occasion / currentness / temporal continuity

Chronos binds the several times relevant to an Expression encounter:

- source revision/freshness;
- Bimba/registry revision;
- M1 harmonic/phase time;
- M2 cosmic/Kerykeion condition;
- M3 world-clock/transcription time;
- M4 Day/NOW/personal occasion;
- AgentSession/voice connection lineage;
- Expression scene/timeline/checkpoint;
- replay/original occasion;
- Factory developmental history where relevant.

It must make stale/current/missing explicit. Reopening an Expression does not upgrade old native state into current truth.

## S4′ Anima — situated Agency / dialogical act

Anima composes the act which actually occurs in the Expression-space.

For M4/Nara this is the crucial foreground relation:

```text
person
  ⇄ Nara
      + current Expression-space
      + current coordinate / M focus
      + attention / deixis
      + voice body
      + consent / privacy
      + available faculties
      + native authority
      + intended Return
```

Nara is therefore the foreground dialogical/voice Agent.

Anima may also compose Epii or specialists into the act, but those contributors do not become the foreground voice unless the experience explicitly changes participants.

The expressive action sequence is attributable and interruptible.

## S5′ Aletheia — disclosure / metabolism / Return

Aletheia receives what actually happened:

- Expression operation receipts;
- native Action/Activity evidence;
- source/result differences;
- user interruption/correction;
- Nara dialogue outcome;
- Epii enrichment;
- T/T′ readings;
- Method/praxis evidence;
- Factory evidence;
- human response/Recognition.

It prepares Return without auto-promoting any of these into source/canon.

Epii is the principal M5 agent for deeper evaluation, pedagogy and development, but the owning human/native system still decides acceptance.

---

# 2. Bimba Expression Atlas

## 2.1 Goal

Every eligible Bimba coordinate and meaningful branch should have an **addressable expressive place**.

This does not mean 1,875 hand-authored scene files.

The Atlas is a deterministic profile resolver over existing Bimba identity and source.

```text
coordinate / branch
  + current Bimba registry/source revision
  + inherited Epi profile grammar
  + typed properties / relations
  → CoordinateExpressionBinding
  → derived ExpressionProfile
```

Selected coordinates can later acquire authored variants.

## 2.2 Inheritance

```text
Epi global profile
    ↓
M-family profile
    ↓
branch profile
    ↓
coordinate-derived profile
    ↓
optional authored variant
    ↓
current encounter overlay
```

The resolver must expose the full inherited chain and exact revisions.

An authored variant cannot silently alter the underlying coordinate or typed relation meaning.

## 2.3 Coordinate binding

Representative API/read model:

```text
CoordinateExpressionBinding {
    coordinate_ref
    bimba_registry_revision
    source_refs[]
    branch_path
    family / face / direct-conjugate identity

    inherited_profile_refs[]
    resolved_profile_ref
    property_readings[]
    relation_grammar_refs[]
    glyph/form bindings[]

    permitted_m_focus[]
    available_mprime_instruments[]
    available_ta_onta_faculties[]

    page/verso refs[]
    native action refs[]
    authored_variant_refs[]
}
```

No second graph store, source store or coordinate registry is introduced.

## 2.4 Branches as inhabitable places

Branch-level profiles are important because the person/Nara should be able to inhabit an area before selecting one leaf.

The first complete Nara set should cover:

- M4.0 identity;
- M4.1 embodied / seven-centre / EarthBody;
- M4.2 oracle;
- M4.3 transformation;
- M4.4 contextual/Jungian/phenomenological/Trika;
- M4.5 pedagogy/integration.

These branches keep the K10 domain contracts; the Atlas supplies the expressive geography in which they can be encountered.

---

# 3. M/M′ expressive grammars

An Atlas coordinate remains one whole while different M determinations are foregrounded.

```text
M0  source / coordinate / knowledge / Bimba neighbourhood
M1  topology / harmonic carrier / geometry / phase
M2  resonance / material / colour / music / correspondences / cymatics
M3  glyph / form / transcription / clock / oracle-form packets
M4  personal reception / centres / identity / oracle / transformation / depth
M5  Epii investigation / pedagogy / enrichment / development / Return
```

The Epi SDK needs a profile/refraction operation such as:

```text
epi.expression.refraction.set(coordinate_ref, m_focus, instrument_ref?)
```

This changes how the same world is disclosed. It does not create a new coordinate/application identity.

Selection alone remains inspection unless a native operation is explicitly invoked.

## 3.1 M0

Use the current Bimba/Wiki/property owners and local-whole adapter. M0 profile grammar determines how the source/relational field is presented, not which edges exist.

## 3.2 M1

Expose topology/harmonic/toroidal/phase and formal geometry through current K8/M1 owner readings and the generic Expression physics/form vocabulary.

## 3.3 M2

Bind current M2 resonance/material/music/colour/correspondential readings into the semantic field engine. Preserve the law:

```text
M2 resonant condition != M4 chakra-centre identity
```

## 3.4 M3

Bind stable form/glyph/transcription/clock identities and scene sequencing to the same world occasion.

## 3.5 M4

Bind K10 protected Nara domain, seven independent centres and EarthBody as session-local live inputs with safe portable cues only.

## 3.6 M5

Expose Epii pedagogy/enrichment/development as source-bearing Expression proposals and native developmental Actions, never as unexplained visual mutation.

---

# 4. Nara foreground dialogue and voice

## 4.1 Identity

Nara is the foreground dialogical Agent in the inhabited Epi Expression-space.

```text
NaraRef != AgentSessionRef != voice-provider session != realtime transport
```

Voice is a body/capability of Nara.

A provider reconnect does not create another Nara.

## 4.2 Dialogue context

The Ta-Onta adapter publishes a bounded context derived from actual current state:

```text
NaraDialogueContext {
    nara_ref
    subject_ref
    expression_ref + revision
    profile_ref + revision
    scene_ref
    coordinate_ref / branch_path
    active_m_focus

    selected entity / subject / relation refs
    pointed_ref
    pinned_refs

    disclosed source/readings
    available actions
    occasion/currentness
    shared-field relation if any
    expressive_act_ref if active
}
```

No DOM/WebGL scraping is required to understand “this” or “that”.

## 4.3 Deixis

Human pointer/selection maps to exact semantic refs before entering the dialogue turn.

Nara spoken references may carry exact refs which the O:I host uses to focus/highlight/open the real target.

This is the Epi-specific use of the generic O:I joint-focus/deictic substrate.

## 4.4 Realtime voice provider contract

QL-MEF should define the semantic requirements, not hard-code an OpenAI client.

Required provider capabilities/dispositions:

- full-duplex or streamed speech interaction;
- interruption/barge-in status;
- manual interrupt;
- structured event/tool request channel;
- connection/reconnect/degraded status;
- voice/body provenance;
- bounded context refresh or tool access;
- latency/material observations for later fitness evidence.

AIKit resolves the provider/body. Actuation owns the canonical Agent/authority relation. O:I owns the audio/UI adapter and Expression act coupling.

OpenAI Realtime is the first provider target, not the semantic identity.

## 4.5 ExpressiveAct

A Nara turn may act through voice and the Expression at the same time.

Model this as an attributable application act:

```text
ExpressiveAct {
    ref
    actor_ref = Nara
    basis_expression_revision
    basis_scene_ref
    speech_turn_ref?
    operations[]
    before_checkpoint
    current_checkpoint?
    interruptibility / atomic boundaries
    activity_refs[]
}
```

Interruption stops both the voice response and pending expression choreography. Safe atomic presentation operations may finish; stale queued choreography is cancelled/held.

`go back` restores an authored/meaningful checkpoint, not a false bit-exact GPU rewind.

---

# 5. Epii behind Nara

Epii remains M5′ and the deeper systemic/developmental Agent.

Nara requests Epii when the act requires:

- deeper source/Bimba traversal;
- cross-coordinate synthesis;
- formal/correspondential/personal refraction comparison;
- richer pedagogical sequences;
- alternative Expression/profile composition;
- Method/evidence analysis;
- prior Return comparison;
- discrepancy diagnosis;
- Factory development.

The returned contract should be structured and basis-bound:

```text
EpiiEnrichment {
    enrichment_ref
    basis_expression_ref + revision
    coordinate_refs[]
    source_refs[]
    method_refs[]
    evidence_refs[]
    standing

    synthesis
    proposed_focus_refs[]
    proposed_scene_changes[]
    proposed_profile_variant?
    proposed_expressive_actions[]
    proposed_native_actions[]
    continuing_questions[]
}
```

A result produced against revision N cannot auto-apply to revision N+k after the live conversation has moved on.

Nara mediates the enrichment to the person and may apply accepted presentation changes through current application operations.

Epii is not silently given the foreground voice merely because it performed the deeper work.

---

# 6. SharedField / multi-Nara

The existing K10 protected multi-Nara and SharedPresenceConsent contract remains authoritative.

The Atlas/SDK extends this into coordinate Expression-spaces:

```text
shared coordinate Expression-space
    ├─ Person A / Nara A / private M4 A
    └─ Person B / Nara B / private M4 B
```

The shared world may carry safe projected coordinate/profile/scene/entity refs and explicit shared focus/Contributions.

Private Nara context, voice history, raw centre state and private Epii enrichment remain private unless an explicit outward projection separately admits them.

Epii is a shared participant only when explicitly invited/admitted.

---

# 7. Native owner map

| Concern | Owner |
|---|---|
| Bimba/M/Vāk/Ta-Onta semantics | QL-MEF |
| Bimba registry/properties/current accepted engines | QL-MEF existing owners |
| generic Expression/Profile/Edition/portal | O:I |
| source bytes / authored human ground | Central/native source owner |
| generic Wiki/knowledge resolution | AIKit + native Wiki owners |
| Nara/Epii Agent/authority/Activity | Actuation + QL semantic bindings |
| voice provider/body resolution | AIKit |
| OpenAI realtime client/audio/UI adapter | O:I application/provider adapter |
| material service placement | Workcell where relevant |
| developmental implementation changes | Factory |
| shared projection/encounter | O:I SharedField |
| source acceptance / Recognition | owning human/native owner |

No native owner is moved into QL merely because Ta-Onta coordinates it.

---

# 8. Execution units

## TA0 — SDK/API binding lock

Map the six S′ organs and existing M×S′ cells onto the current O:I Expression/Profile/Portal/Edition contracts. Preserve stable capability IDs and extend the existing matrix/evidence carriers; do not create a second registry.

Deliver executable/readable request/result schemas where the cross-repo seam requires them.

## TA1 — Bimba Expression Atlas

Implement derived coordinate/branch profile resolution and inherited profile lineage. First acceptance spans representative M0/M1/M2/M3, all six M4 branches and M5.

## TA2 — M/M′ expressive grammar

Bind accepted M/M′ producer readings to generic Expression profile/refraction hooks with exact source/event/currentness.

## TA3 — Nara dialogue / voice / deixis

Implement the semantic Nara dialogue context, provider-neutral voice requirement contract, joint attention/deixis, interruption and ExpressiveAct semantics. Coordinate native Actuation/AIKit/O:I tickets rather than embedding transport code into QL.

## TA4 — Epii enrichment

Implement Nara→Epii structured delegation/results, basis-revision law, M5 pedagogy/variation/development and Factory escalation.

## TA5 — shared coordinate-space proof

Join Atlas/profile/Nara/Epii with the completed SharedField carrier and protected multi-Nara contract.

## TA6 — self-inhabiting Factory / lived programme

Use the actual Epi/O:I world as a developmental subject through Factory and O:I #65. QL supplies domain explanation/evidence; Factory owns the developmental work.

## TA7 — returned praxis

Feed accepted lived evidence into the existing T/T′ / `= name` / AIKit Method/praxis systems. No aesthetic or conversational score becomes semantic truth by itself.

---

# 9. Parallelism

```text
O:I substrate
ES1 content/portals ───┐
ES3 profiles/edition ──┤
                       ↓
ES2 front/verso/reciprocal embedding
                       ↓
ES4 agent/world operations

QL-MEF
TA0 SDK lock ────────────────────── can begin immediately
        ↓
TA1 Atlas ─────┬──── TA2 M/M′ grammar
               │
ES4 ready ─────┼──── TA3 Nara realtime/deixis
               └──── TA4 Epii enrichment
                         ↓
                    TA5 shared
                         ↓
                    TA6 Factory/#65
                         ↓
                    TA7 praxis
```

The Atlas can develop its source/profile resolver before the entire voice path exists. The realtime provider adapter can prototype transport independently, but final Nara acceptance consumes TA0 + O:I ES4 so voice/deixis operate the canonical Expression state.

---

# 10. Acceptance

### Atlas

Bimba coordinate → inherited profile chain → live Expression → source/verso → M1/M2/M3 refractions → same coordinate identity.

### Nara

Enter M4 branch coordinate → Nara speaks in that exact space → pointer reference resolves exact subject → Nara speech focuses exact relation → perform expression transition → interrupt → both speech/choreography stop → restore checkpoint → continue same Nara after reconnect.

### Epii

Ask deep question → Nara delegates → Epii uses exact sources/coordinate → enrichment returns with evidence/profile/scene proposals → human rejects one → no mutation → accept another → exact Expression revision advances.

### Shared

Two independently grounded worlds enter same projected coordinate-space → separate Naras/private M4 states → shared safe semantic refs → explicit Epii invitation → Contribution/Return → private sentinels absent.

### Self-development

Person/Nara identifies a real defect in an Epi coordinate-space → Epii diagnoses source/intent/implementation → Factory changes native owner → rebuilt same space demonstrates returned difference → independent verifier + human Recognition.

---

## 11. Closure

This successor tranche is complete when Ta-Onta is no longer only a capability description around Epi operations but is also the **actual domain SDK/API through which Epi Agents enter and operate the general Expression-world substrate**; Bimba provides addressable derived places; M/M′ determine those places dynamically; Nara provides the foreground dialogical/voice relation; Epii supplies deeper M5 enrichment/development; and all of this preserves the accepted native product owners beneath it.


## W6 native identity material return — 21 September 2026

This is a dependent native increment over #235 at
`0807b827b63732a38c58660a36ca7e5988e09164`, not closure of TA3/TA4,
the entire M4 matrix, or physical/human acceptance. Ownership remains
QL #201; the receiving personal/identity UI is the dependent O:I
`agent/nara-personal-completion-20260921` above #460. Coordination:
QL #201 comment5759425569 and O:I #220 comment5759371739.

The exact typed accepted six-office source/provenance basis now has a
native material contract, `ql.nara-identity-material/v1`, with
`sha256-native-identity-basis/v1`. Native ordinal ordering and sorted
evidence/reference sets are part of v1. Nara and personal subject bind
the basis; record/receipt/review labels, current encounter, provider,
M3 address and quaternion do not become hash bits. Missing layers stay
absent; an empty stack cannot produce a completed identity material.
This implements M4-C64's stable exact provenance/variation role. It
does not invent the open archetypal-quintessence compression law,
semantic similarity, a natal chart, a medical assessment or a person.

`ql nara --request-file - --json` accepts the read-only operation
`identity_material` with existing target, explicit human consent and
expected personal revision. It returns the digest, protected hash ref,
supplied-office/source basis, acceptance standing and separately typed
native form and current orientation. `apply` with mutation
`identity_seal` and `expected_value` is the ordinary revision-checked,
receipt-bearing acceptance path. Rejection/preview changes nothing;
stale expected values refuse. An accepted identity replacement clears
old hash, M3-form and identity-quaternion derivation refs. Central
continues to own source prose and its current retrieval policy.

Production implementation was published at
`cce5d34b555640043f88c2334df0ab5d27c4e447` only after run
**35592841086** passed all **70 ql-cli all-target tests** and clippy
with warnings denied. The earlier run **35592309909** passed those
tests but failed three borrowed-path lint checks; it remains original
evidence. No tests or lint rules were suppressed. New tests exercise
empty/mismatched/stale/private inputs, deterministic native basis,
read-only preview, exact acceptance, idempotent replay and derivative
invalidation; inherited #235 practice/privacy regressions remain.

Reproduce from the PR head:

```sh
cargo test --locked -p ql-cli --all-targets
cargo clippy --locked -p ql-cli --all-targets -- -D warnings
```

Controlled sources and temporary private Worlds are not installed
personal material, microphone/audio, a paid provider or human evidence.
The desktop must recheck Central source revisions, require explicit
acceptance and pass only digest/orientation/form into a private
Expressions frame. Native records, notes, audio and dialogue are not
public editions, shared search inputs or generic diagnostics. Both
installable/self-inhabiting technology and complete published corpus
remain programme outcomes; this private material is not corpus input.
