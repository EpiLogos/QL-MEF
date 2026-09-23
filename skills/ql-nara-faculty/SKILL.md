---
name: ql-nara-faculty
description: "METHOD: Validate an authored Nara activity log (occurrences, thought-consumption refs, identity-proposal refs) and receive a typed personal event into a Nara's constitution over a caller-supplied M1/M2/M3 basis, through the real `ql epi-agent invoke` faculty #4 operations `nara.activity.validate` and `nara.personal-receive`. Use to admit an authored activity/identity-proposal record, or to compose a bounded reception of a supplied coherent event into embodied centres; this Method does not itself fetch a live dated occasion or invoke an oracle."
---

# Nara faculty #4 — activity and reception

## Contract metadata

- Native source owner: EpiLogos/QL-MEF; executable support: `ql epi-agent invoke <request.json> [--json]` (schema `ql.epi-logos-agent-invocation/v1`, position `#4`), dispatched from `crates/ql-cli/src/epi_agent_command.rs` into `ql_mef::epi_agent`.
- Governing source: `docs/kernel-rebuild/AGENT-PRACTICE-AND-BOOTSTRAP.md` (practices XP05, XP07), `docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md`, K10 (`nara.rs`, `nara/activity.rs`).
- Practices: XP05 — "Operate and observe the coherent real-data/material event" — via `nara.personal-receive`; XP07 — "Receive authored activity, oracle and identity proposals" — via `nara.activity.validate`. Both operations are typed, deterministic, side-effect-free (`identity_mutation:false`, `effect_authority_granted:false`): they validate/compose caller-supplied JSON, they do not read a live database, fetch a real ephemeris, mutate Central identity, or invoke an oracle.
- Named gap, not fabricated: no live dated-sky/ephemeris producer (K8.1 Kerykeion/ephemeris integration) exists in this repository or any repository read for this practice; `nara.personal-receive` therefore composes only over a `CoupledInput` the caller already holds. Oracle invocation (M4.2 Tarot/I-Ching casting) is not implemented anywhere reachable by this CLI; `nara.activity.validate` only validates that an activity log's `identity_proposal_refs` are well-formed strings, not that any identity proposal was admitted or an oracle was cast.

## When

Use `nara.activity.validate` when an authored Nara activity log — occurrences, thought-consumption refs and identity-proposal refs, each carrying real Central Day/NOW temporal refs — needs to be checked for schema/source-span/protection conformance before it is recorded or acted on. Use `nara.personal-receive` when a caller already holds a real, accepted M1/M2/M3 coupled basis (from an actual K8/K9 producer, not invented) and a seven-receiver `PersonalEventInput`, and needs the deterministic composition of that event into a named Nara constitution. Do not use either operation to claim a live sky was read, an oracle was cast, or an identity proposal was admitted.

## Inputs

For `nara.activity.validate`: a `NaraActivityLog` (`schema`, `subject_id`, `temporal` Central Day/NOW refs, `occurrences`, `thought_consumptions`, `identity_proposal_refs`) as the `input` object at faculty `#4`. For `nara.personal-receive`: a `PersonalConstitution`, a `CoupledInput` (`world`) the caller can actually compose (real M1/M2/M3 basis, not synthesised for the occasion), and a `PersonalEventInput` with exactly seven `ReceiverEventInput` entries matching the constitution's centres. Both requests use envelope `{"schema":"ql.epi-logos-agent-invocation/v1","position":"#4","operation":"<op>","input":{...}}`.

## Authority

Both operations are read/compute-only: they validate or compose supplied data and return a typed result with `identity_mutation:false` and `effect_authority_granted:false`. Neither operation writes to Central NOW/identity, admits an oracle reading, or authorises any downstream act. Recording an activity log into Central's actual Day/NOW, or admitting an identity proposal, remains its own recognised Central owner operation — this Method validates the shape, it does not perform that recording.

## Procedure

1. Establish the real subject (`subject_id`), Central Day/NOW temporal refs, and — for reception — the actual accepted M1/M2/M3 basis the caller holds. Do not invent a basis to make the request well-formed.
2. Write the exact `ql.epi-logos-agent-invocation/v1` request JSON for the chosen operation, preserving the caller's real refs.
3. Invoke `ql epi-agent invoke <request.json> --json` (or `ql epi-agent faculty '#4' --json` first, to confirm the exact admitted operation names at the installed revision).
4. Read the returned `ql.nara-activity-validation/v1` or `ql.nara-personal-reception-result/v1` document exactly as given. A validation failure is a real rejection (stale/cross-event reference, wrong receiver count, duplicate activity ref, mismatched Day/NOW) — preserve it rather than retrying with adjusted values to force a pass.
5. State plainly what the result does and does not establish: a validated log is not yet recorded in Central NOW; a composed reception is not a live dated occasion and not an oracle reading. Name the actual next native owner (Central receiving, or the K8/K9 live producer) for whichever further step the person actually needs.

## Outputs

The exact subject/temporal refs, the chosen operation and its full request, and the returned typed result (`ql.nara-activity-validation/v1` or `ql.nara-personal-reception-result/v1`) with its `standing` field intact. State explicitly which of "recorded in Central NOW", "live dated occasion", "oracle invoked", "identity proposal admitted" remain unestablished by this Method alone.

## Verification

Run the existing Rust workspace tests covering `ql-mef::epi_agent`/`nara`/`nara::activity` (`cargo test -p ql-mef --lib epi_agent`, `cargo test -p ql-mef --lib nara`; a bare `cargo test -p ql-mef epi_agent` without `--lib` does not select the lib target and silently reports zero matching tests). `epi_agent::tests` now includes a direct positive/negative case for `validate_nara_activity` alongside the pre-existing `personal_receive_refuses_an_unaccepted_or_cross_event_basis` negative case for `nara.personal-receive`. A green test proves the typed validation/composition logic on its fixtures only; it proves no live reception, recording or oracle act.

## Failure and recovery

An invalid schema, wrong faculty position, unadmitted operation name, or a validation error from the underlying `NaraActivityLog`/`PersonalEventInput` types is a real rejection — preserve it exactly and the caller's original request. A missing live M1/M2/M3 basis is a named upstream gap (K8/K9 producer), not something this Method can synthesise.

## Continuity

Use within `ql-experience-prepare`'s established subject/World/NOW basis; report through `ql-evidence-report`, naming the exact validated/composed result and the unresolved recording/live-basis/oracle dependency for the next participant.
