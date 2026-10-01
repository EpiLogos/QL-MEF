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

The current native inventory sets scope: the fidelity coverage retains 2,141 exact source records and 11,810 qualified relations. The earlier 1,875 figure is historical. Reusable profile/material grammar and purposeful compositions provide routes through the full field without requiring one hand-authored scene file per source record.

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

## 30 September 2026 whole-world fidelity correction

Continue the existing #135/#201/#258 and O:I #335/#375/#65/#220 programme through the [whole-world audit](../../kernel-rebuild/EPI-WORLD-FIDELITY-AUDIT-2026-09-30.md), [dependency-ordered recovery packets](../../kernel-rebuild/EPI-WORLD-RECOVERY-MAP-2026-09-30.json) and [complete coordinate disposition](../../kernel-rebuild/EPI-WORLD-COORDINATE-COVERAGE-2026-09-30.json). These are source-backed corrective delivery carriers within this Wayfinder, not another architecture programme. Current Bimba properties/typed relationships and explicit owner rulings govern their meanings; source, owner decision, derivation, authored material and live adjustment remain separately qualified.

The reported sliders-without-world failure is confirmed in the installed pre-correction cut. Atlas coordinate/profile adoption with empty material and a torus-only native projection did not construct the promised geocentric and personal subjects. Adoption/readback/transport tests retain their bounded value; they do not establish visible composition. The first delivery must join the native Earth/ten observed bodies/nine voices, played M1 torus, corrected inscription/lens circles, full source registers and current symbolic form to the same particular person at canonical M4.4.4.4. A saved identity, generic M4 tab or arbitrary node-count cloud does not satisfy this obligation.

The current audit accounts for all 2141 source records, 11810 qualified relations, 42 immediate/legacy entrances and 185 complete existing capability records. Original texts and source properties remain at exact native pointers; broad capability family matches are contextual obligations, not exact-coordinate proof. The registered matrix carriers now retain original standing alongside actual installed component observations. UX01–UX12 remain their original stories; an installed observation, a code pass and H Recognition remain distinct.

Claim R0→R1→R2→R3→R4→R5→R6 in dependency order from the recovery carrier. R7 reconciles publication/acceptance; A0–A5 carry the full outstanding deeper field, personal journey and agency beyond the first cosmic/personal repair. Each packet identifies native owners, mutable files, original source inputs, operation/receiving route and separate semantic/expressive/causal tests. Actual native CAS, current person/occasion reception, durable save/reopen, original entrance replay and managed installed visual/audio evidence are required before first-delivery completion. The audit publication alone changes no installed acceptance or H standing.

## 1 October 2026 — native material construction and receiving entrance

Use the [fidelity audit](../../kernel-rebuild/EPI-WORLD-FIDELITY-AUDIT-2026-09-30.md), [recovery map](../../kernel-rebuild/EPI-WORLD-RECOVERY-MAP-2026-09-30.json) and [complete coordinate coverage](../../kernel-rebuild/EPI-WORLD-COORDINATE-COVERAGE-2026-09-30.json) to execute TA0–TA2/EA3 and the existing deeper TA/EA packets. The actual producer chain is full current Bimba content/relations plus one admitted native occasion → native profile defaults → reusable family/branch/purpose material → native Expression CAS → ordinary scene loader → actual selected engine partitions/personal context. Coordinate adoption supplies grammar/context; each required scene/entity subject still needs its proper material, source and receiver.

The bounded producer consumes actual `profile_define`/`profile_inspect.resolved_defaults`. Existing inheritance resolves parents in declared order with first-parent precedence for a key and whole-key child replacement; `material` is not deep-merged. Retain authored definitions and exact current source/coordinate and numerical-registry readings separately. The native profile catalogue now admits 4,096 refs and 16 MiB aggregate compact typed content, preserving the existing 2 MiB per-profile, 768 KiB material, four document adoptions and lineage-eight bounds. It is a catalogue with revision/dependency guards and no automatic eviction. More complex secondary-parent/rich-key and full ordinary reopen proof remain executable obligations.

One presentation may contain many coordinates and one subject may have multiple purposeful presentations. The current-form glyph and hinge are separately selectable occurrences of one actual native process identity with changing qualified codon/hexagram readings. Personal Pratibimba is the shared M4.4.4.4 locus in its actual ancestry; each saved person/private current/instance receives the same cosmic occasion through the protected native owner. Neither focused adoption nor an authored variant may relabel unrelated resident entities.

`R3-PAUSED-SCENE-ADMISSION` and the existing sparse native target consumer preserve every authored body/connection/density while applying native torus samples only to the bound partition, sampled across the complete native grid. Source-only prime/non-M subjects use native content reads rather than fabricated M-profile bindings. Complete source registers remain focused/selectable material, with full 360/24/36/64/72/18 native member routes. The file-only `oi.expression-storage/v1` representation reuses exact embedded PNGs under unchanged 4 MiB file/8 MiB expanded bounds; the native Document/material/subject semantics stay complete.

At the earlier dated 1 October 2026 cutoff, ordinary24 fixes the normalized-camera receiving break and passes real degree-body pointer selection, then fails Source/Return. Ordinary25 preserves that selection but fails the member-disclosure wait. Ordinary26 passes actual selection and qualified disclosure of all six native registers (360 degrees, 24 backbone governors, 36 decans, 64 codons, 72 skins and 18 apertures), then fails keyboard selection/source opening for the unvoiced sky body. These are actual Chromium/WebGL2 ordinary-app executions through a controlled native host; all three whole receipts are failures. At that earlier cutoff, independent recognition, completed Source/Return, continuing scene lifecycle, two-person live receiving, fresh Nara/Epii, managed installed before/after and physical audio remain pending.

Use `R2-RETAINED-OCCASION`, `R2-FILE-CODEC`, `R2-PROFILE-CAPACITY` and `R5-WORLD-SELECTION` for continuation and disclosure repairs. Their source/commands/tests are in the map. Actual native construction/storage and source inquiry preparation do not replace the required ordinary/managed composition, causal receiver, fresh Nara/Epii or personal Recognition proofs.

### Later 1 October replay — FULL28 native admission boundary

FULL27 is invalid as repair proof: typecheck/build exited 1 and its replay used the preceding bundle. The original rejection is retained; corrected typecheck and build exited 0 before FULL28. Actual FULL28 passes 37 bounded checks in the ordinary Chromium/WebGL2 application through the controlled native bridge. These include all six rendered register selections, complete qualified member disclosure and Source/Return; keyboard Uranus selection and Return without inventing a Bimba coordinate; the protected same-occasion personal seven-centre/nine-driver projection; exact M4.4.4.4 full-source opening and personal Return; and branch-scene entry. The saved person is controlled person A. These results do not claim the owner's autobiographical identity or H Recognition.

FULL28 remains a whole-experience failure. Its first native one-tick operation is refused before host/worker spawn: O:I's closed `Binding` parser admits only `schema`, `host` and `presentation`, while the qualified QL binding also carries `native_basis`, `scene` and `native_readback`. The bounded repair belongs to O:I `kernel/src/native_expression.rs`: admit the exact qualified binding against its retained world first, retain all original qualification and `PrepareWorld` output, then project the unchanged three-field worker envelope for that closed consumer. Arbitrary unknown-field acceptance, qualifier deletion or a numerical remapping would not repair this boundary. Actual worker launch, predicted native determinant effect/invariance and receiving-scene consumption must pass the original ordinary operation and meaningful missing/mismatched-qualifier negatives before this packet closes. Managed installed replay, physical audio, continuing file/scene lifecycle, two-person live causal reception, fresh Nara/Epii and H remain pending.

The actual FULL28 receipt is `51de1a0689167a3283d9e96d1461ac63656300e40d8828ea92d5d9070a3bb732` (`passed:false`, 37 bounded checks); the independent admission/source review is `4124c7763f2c68e5b8fdcf69328e24a0a408748e372d38c505219f1405abe96f`, and the original native refusal is `e2f25e3f3ecb9b24aa7e4ff44c4e1e001c9cb20e8754ec192867c219334467d1`. R7 must carry these exact source/operation references and later receiving evidence; no causal, installed, numerical-readiness or first-delivery completion claim follows from the preceding 37 checks.

## Epi-world corrective receiving return through FULL34 — 1 October 2026

Continue the existing [owner commission QL#135](https://github.com/EpiLogos/QL-MEF/issues/135), [#201](https://github.com/EpiLogos/QL-MEF/issues/201), [#258](https://github.com/EpiLogos/QL-MEF/issues/258) and O:I [#335](https://github.com/EpiLogos/O-I/issues/335)/[#375](https://github.com/EpiLogos/O-I/issues/375)/[#65](https://github.com/EpiLogos/O-I/issues/65)/[#220](https://github.com/EpiLogos/O-I/issues/220) from the current three source carriers: `docs/kernel-rebuild/EPI-WORLD-FIDELITY-AUDIT-2026-09-30.md`, `EPI-WORLD-RECOVERY-MAP-2026-09-30.json#/execution_return/ordinary31_34_native_rest_cursor_and_held_reception_return` and `EPI-WORLD-COORDINATE-COVERAGE-2026-09-30.json`. These are QL-MEF repository paths; the existing programme retains2141 identities/11810 relations/42 entrances/185 full capabilities and original14 parent DAG. No duplicate registry, numerical law or H promotion is introduced.

FULL31's older-file/openNativeDoc41 conflict is excluded from correction acceptance. FULL32 actually acknowledges share4→12/current/reset native CAS and guarded same-file save for the same original24 instance/person/sky/source, but FAILED14 because84,000 resident formation points retain old targets at time/steps0. App-owned production initial zero-time allocation admission is repaired; FULL33 genuinely matches all32 resident bodies to decoded targets, with10980 torus points covering4096 native sample identities and six register masks.37 earlier source/personal/branch checks pass, then nativeM1generation1→4 is falsely rejected by old adapter max2.

Canonical QL InstrumentSession commit `a0ca5590bf5557931f461cb614ca7c4ef22bfbf5` admits source-owned M1delta[2,3] and replace-eventdelta[1,2], retaining other strict cursor/sample/audio/refusal guards. Canonical760cb equals the O:I vendor; actualFULL34 bundlea6688d is frozen.18 controlled protocol tests are not native proof. Separate real16f host/C++worker21 cases demonstrate both M1branches and replacementbranches and all4 owned processes close0; this is not browser/GPU/installed proof.

FULL34 accepts the actual native+3/30° action after the same original24 cold-file/current reception, then FAILS37: ClockA's1281 actual resident points move0 while CPU targets differ275.000015, at time/steps0 before manual reseed. A target/metadata ACK is insufficient. Claim `R3-SCENE-EVENT-CURSOR` for actual browser AudioContext/queued PCM/native target-boundary and meaningful forged-copy regressions; claim `R3-ACKNOWLEDGED-POSE-RECEPTION` for native acknowledged clock/form/torus consequences in actual held body points/pixels, unrelated-input invariance and disconnected consumer failure. Existing `R3-PAUSED-SCENE-ADMISSION` and `R3-TORUS-MATERIAL-CAPACITY` retain the originalRED and32→33 bounded proof. The real saved-reset4-check native guard/restore proof is separate from a pure eighth mutation.

Execute current native owner tests from their packet source receipts: `node --test adapters/retained-field/instrument-session.test.mjs` is controlled transport; the independent actual stdio native suite is `T/independent-review/native-scene-operation-real-cases/replay.py` with actual receipt17bf0c. For ordinary receiving use the existing `desktop/cradle/tests/epi-world-production-native.mjs` and frozen34 configuration as preservedRED; a root-owned successor must retain exact original24 world/file/person/source/occasion while pinning its rebuilt entry/vendor/companion cut. Never mark a rerun of old bundle/config as repaired acceptance.

Sun PhaseA9 provider/native route/knownlegacy/contract/generator source files are integrated, with actual provider21/source-route2 and datedscene-native2 checks. Known e6d qualifiers preserve historical source/sky; new descriptor solar-parent does not authorize fresh-current attestation. Actual stage-A6e4e nativegeneration and PhaseC3source embedding/sourcechecker0 now supply v2 fixture0df; originalv1 31727/original24Sky untouched. Seven compiled default source tests pass at the sourceC cut; full successor companions, actual O:I generic-default host/browser reception and old24 replay with new source remain pending at this sourcecut. Kirdan spelled-TET material stays explicitly unavailable where source notes are absent. Source-only/current protocol proof does not promote broad numerical readiness/parity.

Every next repair carries failure→native owner→exact tests→original encounter→integrate→managed replay. First coherent corrective delivery remains pending until the real rendered native effect/invariance/consumer-cut, source/personal/correction/save&next/restart, two-person differential, fresh Nara/Epii and independent managed semantic/expressive/causal proof pass. Physical audio and the owner's private-person choice/Recognition/H remain their proper distinct matters. Authored references, headless software-GPU production and installed views stay separately labelled.

The separate output5 isolated browser successor now passes 13 cases executing 21 actual native operations; all 13 owned hosts close with exit0. Chrome154.0.8037.59, actual browser AudioContext/nativePCM scheduling and WebGL2/ANGLE SwiftShader target readback with subsequent shader integration are named exactly. Seven negative deliveries mutate or delay copies only after actual native replies; strict native owner refusals are separate. This component proof preserves actual FULL34's zero-step stationary ClockA failure; it does not claim ordinary world, installed software, physical speaker output or H. The exact canonical regression handoff remains to be included through its current owner.

The native O:I Project NOW record `fourfile-technical-release-for-held-native-2026-10-01` grants technical custody to the lead for BRIEF SHA52d57. Its four source/runtime fences cover only PointCloudField/GPGPUSimulator selective stationary position admission after an acknowledged qualified scene revision. All velocities, unselected positions, torus, seed/time/steps/native clocks/cursors/samples and unrelated renderer hunks remain invariant. Canonical source candidate and API-only runtime projection are prepared. Wholefile pre-existing runtimeMAX10/absent geometry-localized-volume versus canonicalTSMAX32 remains; the actual ordinary field imports canonicalTS. Only new API deltas are projected with inverse unchanged bytes. Real owner/whole-consumer tests, independent diff review, source/refreeze Return and broader fullrendererconvergence remain pending; this mismatch does not replace the product receiving audit. This is not Git custody, install authority, broader renderer permission or a repairpass.

The remaining-delivery acceptance map attached to the original R3 execution return keeps the first corrective delivery distinct from the full commission. Beyond any repaired clockhand, active personal two-person sourcepredicted consumer proof, complete samefile Save&next/Restore/Library/recompose/restart and immutableprofile rehydration, current selectedcontext/fresh Nara/Epii basis recovery, real managed computer-use semantic/expressive/causal replay, truthful hardware audio and actual repository/ticket/Human Return remain. A3/A4 continuing DayFlow/selecteddeck symbolic material/person-occasion fences and complete deepbranch coverage remain explicit next packets; their native journey positives do not close the retained crossperson admission gap. All original parent dependencies, source coverage, capability obligations, historical evidence and person-owned H are retained.

## Epi-world receiving successor: independent GPU API and genuine FULL35 failure


The released stationary-position API now has independent actual software-GPU proof at both owners:21 canonical field cases and21 older packaged field cases, plus6 direct GPU guards each. Selected actualRGBA equals transformed targets, all unselected positions and both velocity buffers remain unchanged, renderer bindings are restored, and admission adds zero time/steps/seeds to the prior moving scene.19 field API refusal cases and6 direct range/target guard cases per owner are atomic. Controlled authored targets and opaque admission tags test this primitive; they are not an invented native clock source. CanonicalMAX32 and older packagedMAX10 remain distinct; API-only inverse-byte preservation is not whole-file renderer parity or new localized/volume support.

Genuine FULL35 still FAILED37. The same original24 instance, controlled personA, exact source/sky/occasion and native file cold-open after actualDoc72 preservation/leaseclose. New entry703a9693 imports the qualified canonical primitive; actual native remains7dd/16f. Source inscription now reaches60° at tick2 and the native formAAT/Fu is correct. Actual ClockA1281 resident points nevertheless move0, with currenttargetgap201.313995 at steps/time0. There are no page errors. The app's persisted acknowledged-reading versus temporarily cleared controller influence/needsFrame is an investigation hypothesis, not an established cause. The next repair must make qualified acknowledged state reach the actual quiet body, with exactinstance/person/event/partition fences, invariant unrelated state and a disconnected-consumer negative. FULL34's original failure is retained; primitive green tests do not repair either ordinary encounter by implication.

QL source68d0401709811fba65a63ce79a754db11f02fd60 commits16 Sun/default/census source files and builds all5 qualified native binaries successfully. Currentlinked inventory2870, four census guards and two native-default source guards pass; historical ledger0d85/131decisions and broad numerical UNASSESSED standing remain. This cut is separate from FULL35's still-running16f and managed installed171. The first original24/newsource StageD wrapper raised KeyError texture before native operations; it is preserved as a harness precondition, with corrected native replay and genericnone/now worker proof pending. No fresh-current assertion is inferred from a historical source qualifier.

All6 first corrective delivery gates remain operative: live cosmic/body effect and invariance/consumer cut; actual protected personal/two-person receiving; same-file Save&next/Restore/Library/recompose/restart/profile rehydration; selected source and fresh Nara/Epii recovery; managed independent semantic/expressive/causal computer interaction with actual audio/render environment and chosen private identity; repository/ticket/curated Human Return. The complete S0–S5/deepbranch/DayFlow/selected-deck/cross-person journey/source-numerical recovery remains explicit beyond that first delivery. A clock-hand pass cannot close this whole scope or person-owned Recognition/H.

## Compiled default and retained-occasion replay: bounded successor to FULL35

The committed68d CLI, 7778 host and 937e C++ worker now pass33 native default cases. The actual omitted-event request preserves the declared authored determinants, ten source bodies and nine voices; an explicitly stale original-v1 request refuses. Provider admission, modal/PCM receiving and owned child shutdown are executed. The three-field worker envelope is extracted without changing it. This is not an invocation of the O:I projection: the ordinary generic none/now launch remains an explicit next test.

The corrected root original24 replay also has an independent14-case READ-review. Its full sky, entered controlled identity, event and instance remain exact; the old Sun descriptor is retained as qualified history while the current native Sun-to-centre7 relation governs reception. The actual worker emits512 finite nonzero PCM frames and is reaped. This review inspects the root's real requests and outputs rather than repeating its12 personal operations. Retained occasion is not relabelled fresh. Interpreted activity and composed current remain unavailable until an actual source-qualified M3 activity episode exists.

The three genuine GPU owner tests are now placed at the permanent O:I test paths, and the QL browser runner has its startup-only successor04fa8f33. Exact source and destination bytes match the retained placement receipt. Placement supplies future executable regression owners; it is not an additional run or whole-system success.

FULL35 remains a genuine37-check failure: native inscription60 degrees, actual held ClockA resident displacement0, target gap201.313995. The root has prepared an exact canonical-M3/mirrored-ref guard and an independent render invalidation on NativeField change; source typecheck/build passes were reported, but independent guard and FULL36 acceptance remain pending. Actual pre36 nativeDoc85 guarded save/readback preserves the same full typed file and closes the held lease. It supplements the dated pre35Doc72 transition and does not replace the complete Save&next/Restore/Library/restart gate. All six first-delivery gates, the full remaining S0–S5/branch/journey programme and person-owned H stay open.

## Actual held ClockA receiving and the next discriminating rule — FULL36

FULL36 executes41 preceding checks on the same original24 instance, controlled personA, sky/occasion and native file. It demonstrates the previously missing app effect: before any simulation or manual reseed,1281 actual ClockA resident points move275.000015 to the native target, with target gap0. Both velocity buffers, every unselected body/connection/torus particle, seed, time and steps remain unchanged. This is a real headless application receiving positive, beyond the isolated GPU API. FULL34 and35 remain genuine failed encounters.

The run then fails the assertion that the entire selected-aperture structure stays equal. Independent current/original source review distinguishes seven static aperture fields from the dynamic Fibonacci ground position: native degree60→90 preserves selected index9, reciprocal6,16 static/18 total lenses, divisions300/120 and Void orientation2025, while the60-position six-degree ground index lawfully advances10→15. Full sky, event and person remain exact. The root changes only the verifier from executed4e7d5c78 to1dd76da0; source/native/product numerical laws and the four renderer files are unchanged by that correction. The verifier now requires +30degrees/+5ground positions and must still refuse altered static fields, frozen/wrong ground phases or changed sky/person/event. The failed36 receipt is retained; FULL37 is pending. NativeDoc98 full same-file equality and lease close supplement the earlier72/85 controlled transitions without closing complete continuation.

A separate shared Factory source-body failure remains in the same renderer chain. The retained original enabled native7 ASCII carrier/publication7/projection6 is absent after5seconds, with the peer's measured7 pixels below100. The supplied glyph-only variation is native3→4, not7→8: it removes the ASCII source while keeping the◉ glyph. Its existing body scores8 versus fresh998; a later image has a cyan ring, which is not canonical ASCII acceptance or proof that every glyph view is absent. These exact11 source/image artifacts are read and hashed, with three retained images visually inspected. Canonical ASCII source is not replaced or declared fixed. A bounded R3 source attachment/lifecycle packet at the existing engineProjection/presentation owners must test original source marks and native revisions on the same body across remove/restore/enable/reopen; fresh mount is a comparator, not the repair. This is not a second architecture or a completed Factory/two-person delivery.

All six first-delivery gates remain open. Compiled68d native default33 and original24 READ-review14 remain bounded native positives; permanent browser startup source commit e0e9ca is separate from the five compiled68d companions and the actual36 native16f cut. No managed installed after-view, hardware/heard audio, full personal activity, fresh Nara/Epii or H claim follows from this clock effect.


## Fresh execution source reconciliation — 1 October 2026

The current human Return in [the fidelity audit](../../kernel-rebuild/EPI-WORLD-FIDELITY-AUDIT-2026-09-30.md) leads this commission. The existing [recovery map](../../kernel-rebuild/EPI-WORLD-RECOVERY-MAP-2026-09-30.json) retains 14 packets, 26 ordered subpackets and 185 capability obligations; its `fresh_execution` and evidence register record the independent original-source challenge and current live Bimba READ (907c…, 2,141 nodes / 11,810 directed qualified relations). All five scene-map appendices now have exact source custody and historical scope. This qualifies the recovered whole, not complete branch or installed acceptance.

R0 source reconciliation distinguishes current geometric inscription/lensing from predecessor Clock labels, M3 aperture invariance from M2 modulation, and current 183 M3 discrepancies/383 source LINE_CHANGE edges from the historical 563-record observation/native 384-transition proof. The absent native M4-0-2/-3/-4 identity bindings require canonical M4.0-2/-3/-4 repair with real Reading/full-source UUID receiving checks. Draft personal formulas, trace-only imports and null activity remain qualified; none gains canonical numerical authority from the repair.

R2–R6 execute the first coherent stored cosmic/personal world through complete Return and post-play departure, actual saved-person hydration, same-nine-driver positive/live-zero modal consumption, Library/save/restore/restart, fresh selected-basis Nara/Epii and managed installed demonstration. Original whole-replay failures stay addressable until replayed in the successor environment. Empty material, removed required bodies, wrong personal locus, disconnected consumers and store-only reception fail acceptance. A0–A5 retain branch-specific material, full personal ancestry/lived history and continuing Day/Flow/selected-deck/correction scope. R7 publishes exact source, PR, artifact and operation references. Source/build/transport greens and H/Recognition retain distinct standing; no new release-permission ceremony applies.
