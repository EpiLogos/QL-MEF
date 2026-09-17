# AW / S′ continuity ledger

**Standing:** continuity ledger, 2026-09-17. Owner #201 under #135. Backcheck of the current Ta-Onta implementation against the accepted AW architecture (#94/AW closed, `TA-ONTA-FULL-FIELD-LOCK.md`, `VAK-OIKONOMIA-KNOWLEDGE-RETURN.md`, `PRE-K8-AGENT-WORLD-LOCK.md`, `OPTIMISATION-AND-LEARNING.md`, `K-AW-EXPRESSION-PRODUCTION-ALIGNMENT.md` §3–4).
**Produced on:** branch `session/k-aw-expression-production-2026-09-17` at HEAD `e55bf0224ea6113efd042c8f3146536cef31f147`.
**Scope law:** no new Agent ontology, no new runtime roles, no reopened kernel architecture. This ledger audits what runs, names real gaps, and returns them to their native owners.

## 0. What was audited, and how

- Documentary ground: the six locks listed above plus `EPI-TA-ONTA-AGENT-WORLD-CAPABILITY-MATRIX.md`, `AW0-SOURCE-NATIVE-FIELD.md`, and the AW0 census projection (`python3 scripts/aw0.py --json`, 6,588 records; dispositions 258 ACCEPTED-NATIVE / 535 READY-TO-COMPOSE / 92 EXTERNAL-OWNER / 5,702 RESEARCH-ONLY / 1 SOURCE-DISCREPANCY / 0 EPI-GAP).
- GitHub evidence: #94 CLOSED 2026-09-14 (final return PR #191, squash `7a78dd6`); #201 comments (Wiki→Expression seam via O:I #366; 18→3→1 exemplar instruction); merged PRs #196 (Nara activity consumes AW3 reception), #199 (K10 Nara reading to Expression host), #202 (Ta-Onta as SDK/API of the Expression world), #210 (parallel-executable Ta-Onta Expression work).
- Code: `crates/ql-mef/src/vak.rs`, `vak_composition.rs` (+`native_path.rs`, `production.rs`), `context_frame.rs`, `cprime_oikonomia.rs`, `vak_profile.rs`, `vak_performance.rs`, `vak_thought_consumption.rs`, `vak_scope.rs`, `vak_oi.rs`, `focused_instrument_aw2.rs`, `crates/ql-cli/src/vak_composition.rs`, `crates/ql-adapters/src/techne.rs`, `skills/`, `data/`, `schemas/`, `fixtures/`.
- Source recovery (read-only provenance): C-Experiments `Body/S/S4/ta-onta/` — `composite-entry.ts`, the six organ `CONTRACT.md` files, `S4-4p-anima/modules/dispatch-validate.ts` and `moirai-dispatch.ts`, `S4-5p-aletheia/modules/{anansi-lineage,janus-doorway,mercurius-translation,agora-staging,zeithoven-autoresearch}.ts`, and `S4-5p-aletheia/S5'/agents/{anansi,janus,moirai,mercurius,agora,zeithoven}.md`.
- Tests executed: see §8.

**Headline.** The accepted architecture is intact and nothing in the tree contradicts it: exactly six S′ organs (none instantiated as a second organ tree), the exact C′ six-office grammar with real admission/passage consequence for CPF/CT/CP/CF and library-proven semantics for CFP forms/Z/CS walks, the constitutional seven carried as CF1–CF7 identities (no eighth peer — Aletheia is not a CF), Techne under Pleroma (never an Aletheia subagent), all twelve T/T′ meanings consuming real Factory receipts, and the six Epii-on-X laws present in `epi-epii-operational-capacities.json`. The real gaps are: (1) the CFP-form/Z/CS-passage layer has no surface consumer (CLI, fixtures, native dispatch); (2) the six specialists exist only as census rows — no agent definitions or dispatch surfaces anywhere in the suite; (3) the named AW1/AW2/AW3 gaps of the accepted census remain open at their owners.

## 1. The six current organs (S0′–S5′)

The organs are not QL runtime types and must not become them: Ta-Onta is the S′ whole composed over the native products (Central, Actuation, AIKit, Factory, Workcell, QL-MEF at S0–S5). QL owns the semantics of composition (Anima's C′) and Return/disclosure (Aletheia's membrane) plus the Vāk language and property law the organs consume. Per-organ state below cites: accepted purpose (lock §1), QL implementation, native participation actually consumed, SP coverage (from the current census disposition — every row states its own class), and the real remaining gap.

### S0′ Khora — entry/ground/continuity — verdict: partial

- **Accepted purpose:** enter, situate and continue an actual agent world: ground, Project, subject, source, session, resources, material condition.
- **QL implementation:** no Khora runtime object (correct — none is authorised). Ground/identity semantics QL does own: source-provenanced entry addresses and ref standing in `vak.rs` (`VakRef`, `VakSourceProvenance`, `VAK_SOURCE_REVISION` pin `daa660c…`); kernel ground/face identity via `ql-core` (`QlCoordinate`, `GroundKind`, caller provenance validated in every `vak_composition::Basis`). CLI entry is the JSON command adapter `ql-cli/src/vak_composition.rs` (`execute_request`, contract `ql.vak-composition/v1`).
- **Native participation consumed:** Central root meta-project/source, AIKit Context/SessionSpace, Actuation Agency, Workcell material facts — all via the registered binding groups (`aw0-native-bindings.json` groups: `resolve`, `session`, `context`, `source`), not re-implemented in QL.
- **SP coverage:** SP00–SP05 all open — SP00/01/04/05 READY-TO-COMPOSE (gaps AW2-SCOPE ×2, AW1-ROOT, AW3-CONSUME), SP02 READY-TO-COMPOSE (AW2-CONDUCT), SP03 EXTERNAL-OWNER (SURFACE).
- **Real gap:** none in QL semantics; the rows await row-specific whole-composition receipts and the AIKit #267 invocation join (G3).

### S1′ Hen — semantic and artifact form — verdict: partial

- **Accepted purpose:** keep semantic and artifact form intelligible: source standing, coordinate/property identity, Wiki constellations, typed relations, templates, derived readings, reviewed source change.
- **QL implementation:** CT content typing is operable (`vak_profile.rs` `ContentType` CT0–CT5 + CT4b′ DayNow, with `accepts()` specialisation); content fields gate composition through `CPrimeContext::ct`/`admit` (`vak_composition.rs:1170-1176, 1257-1266`); the property sixfold and typed property vocabulary live under `crates/ql-mef/src/property*`; Wiki constellations and refraction in `ql-wiki` (incl. `tests/aw1_participation.rs`).
- **Native participation consumed:** Central source/document authority and AIKit Wiki/OKF/Knowledge via binding groups `source`, `wiki`, `knowledge`.
- **SP coverage:** SP10–SP15 all READY-TO-COMPOSE — SP10/11/12 gap AW1-PROPERTY, SP13 AW2-SCOPE, SP14 AW3-CONSUME, SP15 AW1-ROOT.
- **Real gap:** AW1-PROPERTY (G7) — source-defined property assertions, compact expansion, named synthesis/graph provenance and native graph persistence.

### S2′ Pleroma — world of available powers — verdict: partial

- **Accepted purpose:** disclose and compose knowledge faculties, models, harnesses, Skills, Methods, tools, M′ instruments, Surfaces and contributed capacities.
- **QL implementation:** the graph-law sixfold as one faculty body — `vak.rs` `VakRegistry` (all 109 source entries, provenance-pinned), the 36-cell `VakSivaSaktiField` and the relational-sixfold `/` join to kernel generation sites; action profiles pinned to two real native owners (`vak_oi.rs`: `VAK_ACTION_PROFILE_CONTRACT`, Factory/Central action refs); Technē instrument surface `ql.techne/v1` (`ql-adapters/src/techne.rs`). No second capability registry exists — availability is disclosed, never self-authorised (`tests/vak_oi_runtime.rs` proves an implementation binding cannot masquerade as an observed path).
- **Native participation consumed:** AIKit resource/capability/praxis resolution and native product Actions (binding groups `registry`, `provider`, `methods`); the model roster is AIKit's, not QL's.
- **SP coverage:** SP20–SP25 — SP20 gap AW1-ROOT; SP21/22 AW1-PROPERTY; SP23/25 AW2-SCOPE; SP24 EXTERNAL-OWNER (NARA).
- **Real gap:** AW2-SCOPE (G3) for the invocation join; AW1 rows per G7. **Techne remains Pleroma's atomic-skill substrate** — see §5.

### S3′ Chronos — the several times co-present — verdict: partial (two rows external)

- **Accepted purpose:** make work/session history, source freshness, actual sky, harmonic/clock phase, personal occasion and developmental return co-present.
- **QL implementation:** the harmonic clock is real — CF canonical selections carry positions/faces/grains through the MEF rotation (`context_frame.rs`), frame pitch derives from the retained M1/M2 musical owner (`vak_profile.rs` `frame_pitch()` via `ActiveFrame::pitch()` → `cf_diatonic_cut`), and `music.rs` carries the M1/M2 musical derivation. QL deliberately owns no civil clock: Day/NOW remain Central's.
- **Native participation consumed:** Central Day/NOW, Factory Run history, M1/M2/Kerykeion/M3/M4 domain times — binding groups `clock`, `workflow`.
- **SP coverage:** SP30/31/32 READY-TO-COMPOSE (AW2-SCOPE, AW3-NAME, AW2-SCOPE); SP33/SP34 EXTERNAL-OWNER (SURFACE, NARA); SP35 READY-TO-COMPOSE (AW3-CONSUME).
- **Real gap:** none QL-owned; the Kairos/Kerykeion qualification rows ride their external lanes (#133/#134 surfaces, Factory history).

### S4′ Anima — composition of the situated act — verdict: real (QL semantics), native dispatch open

- **Accepted purpose:** compose a situated act through full musical C′ Vāk: participation regime, content, position, frame, thread, sequence, actors, authority and intended Return.
- **QL implementation (the accepted C′ engine):** `CPrimeContext` in `vak_composition.rs` with the six reflective offices as receipted operations whose choices have consequence:
  - **CPF** (`cpf`, line 1121): selects direct/conjugate leading face and the eligible operator set; the set actually gates later language admission (`admit`, line 1257: "CPF excludes this language operation").
  - **CT** (`ct`, line 1170): selects source-qualified content fields; gated by `admit` ("CT excludes this contextual content").
  - **CP** (`cp`, line 1179; `cp_at`, line 1481): resolves the actual positioned member of the whole — wrong member selection fails ("CP selects a member not disclosed by this whole").
  - **CF** (`cf`, line 1193): reframes through `ActiveFrame` (CF1–CF7, lens, musical basis, face, position basis) with real harmonic consequence (pitch, phase degrees, child intervals in `FramedReading`).
  - **CFP** (`cfp`, line 1208): binds an explicit whole path to an exact source R-path step sequence (divine-act paths); mismatched step identity/order refuses; every step must declare attributable language.
  - **CS** (`cs`, line 1337): closes the declared thread through attributable Returns with ground-kind checking (`return_result`, line 863) and expected-ground contradiction checks.
  - All receipts are copied into `ReflectiveDerivation` and survive through Return; `determine` requires the focus and thread to match and the Agent contribution to name its CFP inputs.
  - **CFP thread forms + Z + CS walks** (`cprime_oikonomia.rs`): `CfpThreadForm` all six (base/P/C/F/L/B, source markers `ql.cprime-oikonomia/v1:cfp0..5`), `CfpZCycle` (explicit authorisation, ordered compose→perform→record→rehear→recompose, reopen only via attributable reevaluation), `CsProfile` all six with exact paired hops and `CsDirection` forward/returning; `perform_cs_hop` validates that the actual Return's source/target CP positions equal the selected profile/direction pair ("this is what prevents CS from degrading to a label on an otherwise generic Return"); `returned()` refuses to package an OperativeReturn whose CFP form or CS profile was selected after the determination. Additionally `vak_profile.rs` `ThreadForm::validate_plan` structurally validates native `ThreadPlan` legs per form (single/parallel/chain-consumes-predecessor/fusion-requires-aggregation/sustained-resume-and-stop/nested-root).
- **Native participation consumed:** Factory `factory.vak-orchestration/v1` performance snapshots are consumed and projected into one attributable QL event (`vak_performance.rs`, contract `ql.vak-performance-event/v1`, `constitutional_voice` carried, all six CFP factory roles retained — `tests/k8_vak_performance.rs`); AIKit operative syntax `aikit.operative-resolve/v1` is the accepted language revision for `FullVakBinding`; native-path correlation (`vak_composition/native_path.rs`) keeps actor/step/return correlation without granting authority.
- **SP coverage:** SP40–SP45 all READY-TO-COMPOSE — SP40/41 AW2-SCOPE; SP42/43/44 AW2-CONDUCT; SP45 AW3-CONSUME.
- **Real gap:** AW2-CONDUCT (G4) — the six forms and Z through real native Units/Attempts/scoped children and Return (Factory #217). Plus G1 below: the form/Z/CS layer has no consumer surface in this repo.

### S5′ Aletheia — disclosure, knowledge metabolism, Return — verdict: partial

- **Accepted purpose:** disclose what actually happened, compare with intent, consume thought/activity into useful knowledge and learning, prepare Recognition or changed ground. **Not an eighth constitutional peer** — verified: `ContextFrameId` has exactly seven variants; Aletheia appears in QL only as the organ/Return relation, never as a CF or agent peer.
- **QL implementation:** Return semantics are the strongest QL-owned Aletheia surface — `return_result`/`offer_as_whole` with ground-kind law (own/parent/child/other/conjugate; returned content stays DERIVED from its Return, `vak_composition.rs:863-983`); the twelve T/T′ meanings with same-run evidence binding and recognition-candidate emission (`vak_thought_consumption.rs`, `ThoughtMeaning::ALL`, contract `ql.vak-thought-consumption/v1`, recognition contract `vak-recognition-v1`); `VakPath.recognition` in `vak_oi.rs`; M5 recognition return proven in `tests/vak_oi_runtime.rs::native_owner_conformance_can_return_through_vak_path_and_m5_recognition`.
- **Native participation consumed:** native Activity/Evidence/Return, Central receiving, AIKit Knowledge/History/praxis, Factory development evidence, canonical M5′ Epii evaluation/pedagogy — all external; `vak_thought_consumption.rs` consumes the Factory receipt `factory.run-thought-consumption/v1` and stores nothing.
- **SP coverage:** SP50–SP55 all READY-TO-COMPOSE — SP50/52 AW2-SCOPE; SP51/53/54 AW3-CONSUME; SP55 AW3-NAME.
- **Real gap:** AW3-CONSUME (G5) and AW3-NAME (G6) at their owners; Epii-on-X laws are present as data but are authored design standing, not implementation evidence.

## 2. C′ grammar implementation state (CPF / CT / CP / CF / CFP / CS)

| Office | State | Evidence of consequence |
|---|---|---|
| CPF | **Operable** (composition + CLI `cpf` op) | Selects face + eligible operators; `admit()` refuses excluded operations at determine/CFP/CS time; test `cpf_ct_and_source_path_refusals_are_atomic_and_consequential`. |
| CT | **Operable** (composition + CLI `ct` op) | Content fields gate admission; CT4b′ accepts-as-specialisation of CT4; seven content types in `vak_profile.rs`. |
| CP | **Operable** (composition + CLI `cp` op, path and member forms) | Focus/member resolution into immutable wholes; undisclosed member refuses; `cp_at` keeps CF active. |
| CF | **Operable** (composition + CLI `cf`/`reframe`) | Reframe creates a new immutable use; harmonic pitch/geometry/child intervals recomputed from the retained M1/M2 owner; CF identities pinned by fixture (`fixtures/q6/context-frame-promotion-v1.json`, `tests/context_frame_promotion.rs`). |
| CFP (R-path thread) | **Operable** (composition + CLI `cfp` op) | Binds every source R-path step in order to an explicit whole path; each step must carry declared language; receipt survives into Return. |
| CFP (six thread forms + Z) | **Library-proven, no surface consumer** (G1) | `CfpThreadForm`, `CfpZCycle`, `ThreadForm::validate_plan` semantics are consequential (form must precede determination; plan topology validated per form; Z needs authorisation and ordered stages) — but `CPrimeOikonomia` has no caller outside its module, no CLI op, no fixture. |
| CS (six profiles + direction) | **Library-proven hop validation; base `cs` op operable** | `CPrimeContext::cs` operable via CLI `return` with context; `select_cs`/`perform_cs_hop` profile/pair validation library-only (G1). Test `all_six_cs_profiles_preserve_exact_source_extent` and `returning_inquiry_changes_pair_direction_without_reversing_profile_order`. |

The audit test "a carrier containing a CF ref is not acceptance" passes for CPF/CT/CP/CF and the R-path CFP (admission, addressing, pitch and passage actually change); it **fails as a surface claim** for the CFP-form/Z/CS-walk layer — the semantics are real and tested but nothing in this repository drives them through an undertaking yet (this is consistent with the census: the finite grammar is ACCEPTED-NATIVE, the SP rows are READY-TO-COMPOSE, and AW2-CONDUCT names the native-conduct gap at Factory #217).

## 3. Constitutional field (the seven, hosted through Anima)

- **Carried as CF identities, exactly:** `constitutional_voice(ContextFrameId)` in `vak_profile.rs:433-443` maps CF1 `(00/00)`→Nous, CF2 `(0/1)`→Logos, CF3 `(0/1/2)`→Eros, CF4 `(0/1/2/3)`→Mythos, CF5 `(4.0/1-4.4/5)`→Anima, CF6 `(4.5/0)`→Psyche, CF7 `(5/0)`→Sophia. CF5 is Anima's executive whole — not flattened into an array peer; there is no eighth CF, so **Aletheia is structurally not a constitutional peer** (alignment §4 preserved).
- **Musical consequence is real:** each CF has a canonical MEF selection (position/face/grain), a diatonic cut pitch per basis/lens (`cf_diatonic_cut`), and the seven-fold cut partitions all twelve form addresses per lens (`tests/context_frame_promotion.rs`), with the fixture blocking unsupported CF semantics.
- **The source agrees on the binding mechanism:** C-Experiments `S4-4p-anima/modules/dispatch-validate.ts` `AGENT_CF` uses the same seven-name roster with the same CF literals and routes CF→agent for dispatch; Moirai's Klotho/Lachesis/Atropos ride as an escaped subagent roster, not constitutional members.
- **Office semantics (Nous opens ground; Logos distributes usable form; Eros performs according to need; Mythos recognises organising pattern; Psyche keeps contextual continuity; Sophia integrates toward renewed ground)** are authored law in `VAK-OIKONOMIA-KNOWLEDGE-RETURN.md` §2.1. QL carries the identities and their musical/frame consequence; it intentionally does not encode the offices as behaviour branches — the offices govern composition choices, which the composer (Anima-side agency) makes through the C′ offices above. No gap in QL; the office-carrying agent definitions are an AIKit/Actuation concern when a runtime roster is warranted.

## 4. Specialists (Anansi · Janus · Moirai · Mercurius · Agora · Zeithoven)

**Recovered responsibilities (from source, verified against the alignment's shorthand — they agree):**

| Specialist | Source-verified responsibility (S5′ agent definitions + pure modules) | Alignment shorthand | In this repo |
|---|---|---|---|
| Anansi | Orientation and paradigmatic gap analysis; holds /Empty and /Present; disclosure-lineage with strict wikilink provenance — a trace without provenance is refused | coordinate/blueprint | **Census row only** (`scripts/aw0.py` specialist-mode) |
| Janus | Temporal threshold analyst; builds the temporal-context envelope binding archive (before) to present sessions (after) | temporal/threshold | Census row only |
| Moirai | GraphRAG distillation specialist; one actor, three CS-state modes — Klotho (traces/evidence), Lachesis (sources/query), Atropos (crystallisation/reflection) — active in the Night′ CFP3 pass | GraphRAG distillation | Census row only (`Moirai_is_one_actor_with_three_modes: true` retained) |
| Mercurius | Cross-domain translation and kairos signal transport; carries qualitative signal across family boundaries preserving origin charge | Kairos/qualitative temporal pattern | Census row only |
| Agora | Parallel aggregation and multi-channel gathering into one legible field without erasing voice distinction; challenger evaluation staging | aggregation/skill/plugin absorption | Census row only |
| Zeithoven | Temporal creativity and structural manifestation; improvement-context with explicit attribution; next score handed to next compose; deep autoresearch delegates to Epii | creative advance/skill-agent creation | Census row only |

- **Runtime presence: none in this repo.** `rg -i "anansi|janus|moirai|mercurius|agora|zeithoven"` over crates/adapters/skills/data/schemas/fixtures returns only the AW0 census (`scripts/aw0.py`). No QL types, no profiles, no skills.
- **This is a gap, not a violation.** The alignment (§4) requires they be dispatched "through the existing CF/Anima/Aletheia composition only when the undertaking warrants it" — they are not a mandatory production team, and nothing here mints them as QL ontology. But the dispatch route itself (agent definitions carrying Rupa/Ontology/Frame Contract/Temporal/Capability/Sattva, with CF identity + skills) does not yet exist anywhere in the suite that this repo can call. That is named gap **G2**; native owner AIKit/Actuation agent definitions, with QL supplying the CF/C′ dispatch grammar that already exists.
- **Historical interface evidence retained (provenance, not runtime):** the S4 pure modules (`anansi-lineage.ts`, `janus-doorway.ts`, `moirai-dispatch.ts`, `mercurius-translation.ts`, `agora-staging.ts`, `zeithoven-autoresearch.ts`) show the mechanisms (lineage-provenance refusal, temporal envelope, CFP3 Night′ pass with host-CF inheritance, origin-charge translation, plurality-preserving staging, attributed next-form) that any new definitions must preserve.

## 5. Techne under Pleroma

- **Preserved:** Techne is nowhere an Aletheia subagent. In QL it appears (a) as the `ql.techne/v1` language binding of the L5 Technē instrument contract (`ql-adapters/src/techne.rs`: six 4:2 deep instruments M0′–M5′ + `Expressions` as the conjugate 3:3 reading — instrument identity is disclosure vocabulary, not an application boundary), and (b) as the R# praxis horizon inside the Vāk field (`VakContextField::Techne`, `VakAddressHorizon::H5`, `VakPraxisAspect`).
- **Atomic-skill substrate lives natively:** Skills/Methods are AIKit-owned; `skills/README.md` and the five `METHOD:`-classified project skills (`ql-operation`, `ql-experience-walk`, `ql-experience-prepare`, `refraction-adapter-authoring`, `bimba-cypher`) follow the "a Method is a Skill whose description starts `METHOD:`" law with no second skill registry. The 65 historical source skills are retained as AW0 census rows (`source-skill: 65`), not revived as runtime files.
- **No gap in the relation itself**; the skill-entitlement recovery rows remain READY-TO-COMPOSE under the census (AW2-SCOPE/AW3-NAME joins).

## 6. T/T′ → Aletheia/Epii → Recognition / Method / Skill Return path

**What exists:**

1. **Carrier (Central, external):** every NOW clearing carries `T/` raw thought fixtures and `T-prime/` distilled learnings via `central.now.thoughts.append` / `central.now.learnings.distill`; machine spellings `T0`–`T5`, `T0-prime`–`T5-prime`; the contemplate contract is ai-kit now-contemplation (ai-kit #304, O-I #276). Recorded here as a **pointer**, per the implemented-carrier note in `VAK-OIKONOMIA-KNOWLEDGE-RETURN.md` §3 — not verified from this repo.
2. **QL semantics (this repo, real):** all twelve `ThoughtMeaning` values (`vak_thought_consumption.rs:47-77`); consumption validates one immutable Factory receipt (`factory.run-thought-consumption/v1`), binds each reading to the recovered `thought:T0…T5′` inventory identity, and optionally proves the receipt consumed evidence from the same performed Vāk occasion (`performance_relation_requires_shared_actual_evidence_and_same_run`); output is a source-qualified reading under `ql.vak-thought-consumption/v1` that can be offered to Recognition (`VAK_RECOGNITION_CONTRACT = "vak-recognition-v1"`), emitting recognition candidates without stealing AIKit's naming authority. Anti-silo: the twelve are types on one carrier — the module introduces no per-meaning stores.
3. **Recognition → name (AIKit, external):** AIKit #308 accepted source-qualified native Action invocation and Recognition → `= name` → Skill/Method proof/reuse. QL's `VakPath.recognition` and `vak-recognition-v1` are the QL-side candidate seam.
4. **Consumption into improved practice:** owned by Aletheia/Epii per `OPTIMISATION-AND-LEARNING.md` §3 AW3 (including the optimise-the-optimiser subcase); Factory RunThoughtField for commissioned work.

**What is missing (named, not anonymous):**

- **G5 / AW3-CONSUME:** actual direct (non-Factory) consumption across all twelve meanings — the QL module consumes the Factory receipt only; there is no QL surface that consumes a Central `T/`+`T-prime/` clearing directly. That join is a pointer, not a claim; making it real needs either a Central-receipt input path in `vak_thought_consumption.rs` or the Central→Factory→QL chain exercised end to end by its owners.
- **G6 / AW3-NAME:** the census retains "Recognition-to-native Method Skill registration and parameterised reuse, not parser-only naming" as the open named gap (SP31/SP55 rows); fresh-agent uptake episodes (`OPTIMISATION-AND-LEARNING.md` §5 B08/B09/B11) are the acceptance target.
- **G8 / SURFACE+NARA:** same-subject surface use (#133) and protected Nara reception (#134) remain external owners.

**Six Epii-on-X laws:** present and intact at `docs/integrations/epi-logos/epi-epii-operational-capacities.json` (`epi-epii-operational-capacities/0.1`, `source-backed-research-matrix`): M0 Anuttara language-by-construction; M1 Paramaśiva CPT+RAG with matheme authority; M2 Paraśakti rebuildable graph geometry; M3 Mahāmāyā governed RL/federated/genetic pathway learning; M4 Nara Anima-led consent-gated voice refinement; M5 Epii recursive pedagogy/review/canon/autoresearch through explicit Recognition. Standing `active-operational-capacity-spec` — authored law, not implementation evidence.

## 7. Exact gap list

| ID | Organ / relation | What is missing | Native owner | Smallest concrete next action |
|---|---|---|---|---|
| G1 | S4′ CFP-form / Z / CS-walk surface | `CPrimeOikonomia` (six thread forms, Z cycle, `select_cs`/`perform_cs_hop` profile-pair validation) is library-tested but unreachable: no `ql-cli` ops, no fixture, no consumer binds it into a real undertaking | QL-MEF (CLI/facet surface), with Factory #217 for native conduct | Add `cfp-form` / `z-begin` / `z-advance` / `cs-select` / `cs-hop` ops to `ql-cli/src/vak_composition.rs` dispatch and one fixture composition that walks a full CS0 profile through positioned Returns |
| G2 | S4′/S5′ specialists | Anansi, Janus, Moirai (one actor, three modes), Mercurius, Agora, Zeithoven have no agent definitions or dispatch surfaces anywhere in the suite — census rows only | AIKit / Actuation (agent definitions); QL supplies the existing CF/C′ dispatch grammar | When the first production undertaking warrants one, author that specialist as an AIKit/Actuation AgentProfile with its CF identity, tools and skills per the recovered S5′ definitions; no QL ontology change |
| G3 | S0′–S5′ AW2-SCOPE | Production QL provider and source-qualified native invocation beyond the accepted Contemplate foundation (SP01, SP04, SP13, SP23, SP25, SP30, SP32, SP40, SP41, SP50, SP52) | ai-kit #267 | Land the remaining #267 join and bind one row-specific whole-composition receipt per addressed SP row |
| G4 | S4′ AW2-CONDUCT | All six CFP forms and Z through real native Units/Attempts, scoped children and Return (SP02, SP42, SP43, SP44) | Factory #217 | Exercise one real Chain and one real Fusion undertaking through Factory orchestration into `ql.vak-performance-event/v1` and Return |
| G5 | S5′ AW3-CONSUME | Actual direct/commissioned T/T′ consumption and subsequent agency across all twelve meanings; no QL-side consumer of the Central `T/`+`T-prime/` carrier (pointer only) (SP05, SP14, SP35, SP45, SP51, SP53, SP54) | Aletheia/Epii with Central + AIKit owners | Route one real Central NOW thought cycle through the contemplate contract into a Factory receipt and prove the QL reading + recognition candidate end to end |
| G6 | S5′ AW3-NAME | Recognition→native Method Skill registration and parameterised reuse with fresh-agent uptake (SP31, SP55) | AIKit (base #308 accepted) | Prove one `= name` registration from a QL recognition candidate and one fresh-agent reuse on new input |
| G7 | S1′/S2′ AW1-ROOT + AW1-PROPERTY | Rooted family/face/S5/property structural joins through K8.0; source-defined property assertions, compact expansion, named synthesis, graph persistence (SP04, SP10–SP12, SP15, SP20–SP22) | #94 AW1 through K8.0 (#132) | K8.0-shared property-assertion tranche per the AW1 plan |
| G8 | S0′/S3′ SURFACE + NARA (external) | Same-subject field/Bimba/Epii surface use; protected Nara reception and M4.5 (SP03, SP24, SP33, SP34) | #133 (surface), #134 (Nara) | None here — external lanes; do not count as QL gaps |

No anonymous EPI-GAP exists anywhere in the audited field (census: 0). The 5,702 RESEARCH-ONLY rows (deep property-key occurrences etc.) and the single SOURCE-DISCREPANCY (the legacy 20-frame/40-direction claim) retain their standing.

## 8. Evidence: commands and results

```text
git rev-parse HEAD               -> e55bf0224ea6113efd042c8f3146536cef31f147 (branch session/k-aw-expression-production-2026-09-17)

cargo test -p ql-mef --test vak_composition --test vak_composition_native_path --test vak_language \
  --test vak_oi_runtime --test k8_vak_performance --test context_frame_promotion --test context_frame_external_reading
  -> 11 passed (vak_composition), 5 passed (native_path), 14 passed (vak_language),
     4 passed (vak_oi_runtime), 3 passed (k8_vak_performance), 6 passed (context_frame_promotion),
     2 passed (context_frame_external_reading); 0 failed

cargo test -p ql-mef --lib cprime   -> 4 passed (CFP forms distinct; Z order+authorisation; CS extents; returning-inquiry direction)
cargo test -p ql-mef --lib vak_profile -> 6 passed (thread forms have distinct executable obligations; fail-closed malformed topology)
cargo test -p ql-mef --lib vak_performance -> 7 passed (chain performance keeps Factory evidence, derives QL music)
cargo test -p ql-mef --lib vak_thought -> 7 passed (same-run evidence required for performance relation)

python3 scripts/aw0.py --json     -> 6,588 records; disposition counts as quoted in §0; all 36 SP rows inspected individually
```

GitHub: #94 CLOSED (2026-09-14, PR #191 / `7a78dd6`); #201 OPEN with the O:I #366 Wiki→Expression seam dependency and the 18→3→1 exemplar instruction; PRs #196/#199/#202/#210 all MERGED.

## 9. Continuity statement

Nothing audited here authorises a new Ta-Onta tree, a new organ instantiation, a new agent roster or a new registry. The accepted architecture survives the backcheck; what remains open is exactly what the accepted census already names, plus the two locally actionable surfaces (G1 consumer ops, G2 specialist definitions at their native owner) this ledger adds with evidence. Expression production (wayfinder Q3) should consume these contracts as-is: compose through `ql.vak-composition/v1`, return through the C′/Return law, and leave the specialist and consumption joins to their named owners.
