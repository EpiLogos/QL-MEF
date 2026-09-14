# K10 Nara / M4 contract and acceptance map

Status: implementation contract for `EpiLogos/QL-MEF#134`, K10.0–K10.4.

This document records what the K10 implementation means on the current kernel rebuild. It is not a new product vision and does not supersede the kernel rebuild Wayfinder, the pre-K8 agent-world lock, the living-instrument architecture, the M4 deep capability matrix, or the Ta-Onta full-field lock.

## 1. Current floor and ownership

K10 starts above the accepted K8 personal receiver. The pre-existing floor remains:

- `ql.nara-personal-field/v1` in `crates/ql-mef/src/nara.rs`;
- `ql.personal-coupled-session/v1` in `crates/ql-mef/src/continuous/personal.rs`;
- the accepted M1/M2/M3 `CoupledBasis` and continuous-field owner;
- seven explicitly supplied receiver constitutions, one subject identity, one distinct EarthBody constitution, exact event/source identity, stale/conflicting replay rejection and exact same-input replay.

K10 does **not** restart those systems. It builds the complete M4 personal domain above that accepted event.

The cross-owner boundary is equally strict:

- K8/#132 remains the completed shared structural/C/registry writer;
- #94 owns M0/M5/Ta-Onta and operative Recognition. K10 consumes its public identities and Returns; it does not implement another Bimba graph, Epii loop, T/T' store or Recognition owner;
- Central owns NOW, DAY, Flow/Dialogue source files, retention/rollover and human/Agent authorship. K10 consumes source-qualified temporal handles and never creates a second journal;
- O:I owns the reusable Expression Stage, targets, renderer, library and portable Expression artifact. K10 publishes a protected domain projection and never creates a second expression surface;
- local desktop/human harmonisation remains later work.

The branch is based on current main after the accepted AW1 merge (`05b6d4ae29c20774f6afe69bffe3df9cd1346a90`). That means the K10 Bimba seam consumes the landed `ql.aw1-rooted-m-world/v1` implementation rather than a remembered future contract.

## 2. The complete M4 field

`crates/ql-mef/src/nara/domain.rs` publishes `ql.nara-m4-domain/v1` and keeps all six branches present as distinct offices.

### M4.0 — Mahamaya identity matrix

The identity field contains six explicit slots:

1. birthdate/name;
2. natal chart;
3. Jungian assessment;
4. Gene Keys;
5. Human Design;
6. archetypal quintessence.

Optional evidence is genuinely optional: a slot is either backed by an attributable protected value or carries an explicit absence reason. Missing Gene Keys, Human Design or another layer is not silently synthesized. M3 form address, identity hash and identity quaternion remain distinguishable references rather than one overloaded identity.

### M4.1 — sympathetic / embodied field

The embodied field retains the canonical EFWA component order `Earth, Fire, Water, Air` and requires exactly seven independently sourced centre states. Each centre may carry its own amplitude, phase, modal content, pairwise couplings, body-zone/sense/action and feedback references. No centre is derived from another merely to fill the vector.

EarthBody is a separate anchor with its own source, frame and relation state. It is not an eighth chakra and is not folded into a seven-centre average.

Nadi/sushumna, temporal-astrology, materia and operation references remain explicit parts of the branch. Numerical state is a compositional reading, not a clinical/therapeutic claim.

### M4.2 — divinatory frameworks

An oracle event retains:

- explicit granted consent;
- oracle system/tradition/method;
- protected query reference;
- an attributable entropy receipt rather than an implicit RNG;
- the exact cast/draw tokens and positions;
- cast time/degree and source revisions;
- hygiene standing;
- the protected original payload.

Interpretation is a different object. A later model can add a new interpretation revision, but cannot alter the original cast packet. Historical non-Apple zero-fill and other old C stubs are therefore not promoted into a valid current entropy provider.

### M4.3 — mediating transformation

Transformation phase history retains source-defined structural bounds:

- twelve storeys;
- three decans per storey;
- twenty-four strokes;
- operation references;
- protocol and dialogical-container identity;
- opening/closing occurrence times;
- safety state and feedback.

The historical 12×3 recipe-card layout and seven operation names remain source identities where qualified. The old placeholder LUT contents do not become present implementation truth merely because they compile in the reference C.

### M4.4 — context and depth lenses

The field keeps six distinct contextual branches:

- Gebser;
- ontological;
- epistemological;
- Jungian depth;
- phenomenological;
- Trika/Kashmir.

A branch contains attributable readings or explicit absence. A reading retains source, model when applicable, protected content, confidence when applicable, evidence and standing. Personal Pratibimba and Recognition refs remain explicit instead of being inferred from visual symmetry or model fluency.

### M4.5 — Epii integration

All six source-defined internal offices remain addressable:

- Curriculum Map;
- Core Epi-Logos Voice;
- Method Transparency Lab;
- Integration Lab;
- Pedagogy Lab;
- Logos Cycle Engine.

A Return records inputs, sources, method, protected output, evidence, optional human response, retention policy and standing. #94/Factory/AIKit operative Returns are held as external refs: M4.5 can integrate them personally without becoming their execution or Recognition owner.

## 3. Central DAY / NOW activity

`crates/ql-mef/src/nara/activity.rs` publishes `ql.nara-activity/v1`.

The contract follows Central's landed ownership law:

`NOW != DAY != Session != Run != Wiki != authored canon`.

K10 therefore stores only owner-qualified handles to Central temporal/source objects plus a Nara reception/interpretation receipt. It preserves:

- root/meta-project vs Project temporal scope;
- Day ref/revision and NOW ref/revision;
- Flow/Dialogue refs and source-history ref;
- occurrence time separately from receipt time, so late Returns remain late Returns;
- original protected bytes by reference rather than copied text;
- exact byte spans, source length, parse model/revision, confidence and protection class;
- parser abstention as a valid result;
- proposed identity updates as proposals, not automatic identity mutation;
- human, Agent and provider actors as distinct standings.

A hard validation rule prevents Agent/provider output from being relabelled `human_authored=true`.

The #94/Factory T/T' path is consumed by `ThoughtConsumptionRef`: it retains the producer contract, thought identity, run/result/source interpretation, evidence, optional human response, retention policy and Recognition/continuing-question ref. The thought itself stays with its owner.

## 4. Replay

`crates/ql-mef/src/nara/replay.rs` publishes `ql.nara-replay/v1`.

A `NaraOccasion` records the original accepted world event, personal reception generation, identity revision, Day/NOW refs, occurrence/receipt times, protected complete-state ref and exact constituent activity/oracle/transformation/context/integration/expression refs.

`replay_original()` returns that retained original. Later reinterpretations are append-only receipts linked to the occasion. They cannot claim `Source` or `Observed` standing and cannot mutate the stored original. This is the required distinction between replaying a dated oracle/personal moment and asking a newer model to interpret that moment again.

## 5. Expressions: use the existing system, do not build another one

`crates/ql-mef/src/nara/expression.rs` publishes `ql.nara-expression-projection/v1`.

The projection deliberately contains **references**, not raw personal bodies. In particular it does not serialize the raw journal, identity layers, bioquaternion or seven-centre scalar field into the portable expression envelope.

The projection carries:

- stable expression and occasion refs;
- subject/event/profile/personal reception identity;
- the intended M4 branch or whole-personal/replay intent;
- O:I-owned target/host contract refs;
- an opaque protected state ref;
- source/selection/constituent refs;
- optional host artifact refs returned after O:I saves an Expression;
- optional semantic stage cues;
- private-local or explicitly consented shared-presence disclosure.

This means the current O:I Expression Stage/Library and compatible `oi.journey/1` authoring envelope can persist and present a Nara expression without QL-MEF introducing its own renderer, stage, library or file format. A saved Expression is not claimed to be an exact GPU/particle checkpoint unless the O:I host separately provides such a contract.

K10.3 also consumes the now-landed #94/AW1 `RootedMWorld` directly. `BimbaSelectionBinding::from_rooted_world()` carries the exact selected source coordinate, registry revision and distinct direct/conjugate canonical refs into the Expression. No second Bimba selection store is built.

The Epii side is represented as the same external AgentSession plus operation-Return refs. Until #94 lands the final M5/Ta-Onta producer interface on main, K10 does not invent a substitute local M5 type. When that producer moves, the binding remains an adapter seam rather than an owner transfer.

## 6. Protected multi-Nara state

`crates/ql-mef/src/nara/multi.rs` publishes `ql.nara-multi-field/v1`.

`ProtectedNaraFieldSet` owns independent `PersonalFieldInstance`s keyed by subject. It can accept subject-specific M3 identity while requiring that participants inhabit the same world occasion: event ref, profile generation, registry revision and M1/M2/M3 source/contract identities must agree.

There is intentionally no bulk API returning every participant's raw `PersonalFieldState`. Raw personal state is read per subject. A shared encounter is made only from already-safe Nara Expression projections plus an explicit `SharedPresenceConsent` allow-list that names:

- at least two participant subjects;
- allowed expression refs;
- allowed O:I target refs;
- grant time and optional expiry.

Thus two Naras may experience one shared world differently without making one another's identity, journal, bioquaternion or sensory derivatives public.

## 7. Acceptance map

The executable tests introduced with K10 cover the following invariants:

| Requirement | Executable evidence |
| --- | --- |
| Six M4.0 offices, optional absence retained | `domain::tests::identity_retains_all_six_offices_without_fabricating_optional_evidence` |
| Seven independent centres + distinct EarthBody | `domain::tests::seven_centres_are_independent_and_earth_body_is_not_an_eighth_peer` |
| EFWA order | `domain::tests::canonical_element_array_is_earth_fire_water_air` |
| Oracle original vs reinterpretation | `domain::tests::oracle_keeps_original_packet_when_interpretation_changes` |
| 12×3/24 transformation bounds | `domain::tests::transformation_preserves_source_defined_phase_ranges` |
| Occurrence != receipt | `activity::tests::occurrence_and_receipt_remain_distinct_for_late_return` |
| Agent output is not human-authored activity | `activity::tests::agent_output_cannot_become_human_activity_by_label` |
| Parser abstention / exact source spans | `activity::tests::parser_may_abstain_without_fabricating_spans`, `overlapping_inferred_spans_are_refused` |
| Exact original replay | `replay::tests::later_interpretation_does_not_mutate_original_replay` |
| Reinterpretation standing remains derived | `replay::tests::reinterpretation_cannot_claim_original_source_standing` |
| Expression has refs, not raw protected state | `expression::tests::expression_projection_contains_refs_not_raw_personal_state` |
| Real #94/AW1 selection is consumed | `expression::tests::accepted_aw1_bimba_selection_is_consumed_without_a_second_graph_store` |
| O:I remains Expression host owner | `expression::tests::ql_does_not_claim_a_non_oi_expression_host` |
| Shared presence requires explicit allow-list | `expression::tests::shared_presence_is_explicitly_allow_listed` |
| Two Naras may share one world without sharing subject identity | `multi::tests::distinct_naras_can_share_world_key_without_sharing_subject_identity` |
| Personal constitutions remain independent | `multi::tests::registry_keeps_constitutions_independent` |
| One shared field cannot mix world occasions | `multi::tests::one_registry_refuses_mixed_world_occasions` |

The pre-existing K8 Personal tests remain part of acceptance: exact same-input replay, stale/conflicting-generation refusal, subject/event matching and field-currentness are not replaced by the new domain tests.

## 8. What K10 explicitly does not claim

K10 does not claim that historical divination, alchemy, astrology, Human Design, Gene Keys, Jungian or chakral source structures are empirically validated medical or physical models. It preserves their source-defined place in the Nara research/instrument domain and keeps source, implementation, observation, model interpretation, user report and proposal distinguishable.

K10 also does not claim desktop parity. The reusable Expression surface belongs to O:I and the later Epi/Nara desktop composition remains the place to harmonise controls, visual language and human interaction around these now-addressable domain states.

## 9. Merge receipt

The PR must not be merged on prose alone. Record the exact accepted head and GitHub workflow conclusions in the PR/issue after the Rust and existing kernel gates complete. If any gate exposes an integration defect, repair it on this same branch and record the superseding SHA rather than accepting a stale green run.