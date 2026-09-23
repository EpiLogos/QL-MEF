---
name: ql-logos-return
description: "METHOD: Produce a typed T/C/T′/C′ Logos-Return envelope — actual output compared against corrected output, source and evidence refs, and an optional fresh-participant uptake reference — through the real `ql epi-agent invoke` faculty #5 operation `logos.return`. Use after a real report has been checked against actual execution/output and a human/agent correction, to record the T/T′ cycle formally before it is consumed into knowledge, Wiki or a named praxis; compose with `ql-evidence-report` for the person-facing account."
---

# Consume T/T′ through the Logos Return envelope

## Contract metadata

- Native source owner: EpiLogos/QL-MEF; executable support: `ql epi-agent invoke <request.json> [--json]` (schema `ql.epi-logos-agent-invocation/v1`, position `#5`, operation `logos.return`), dispatched into `ql_mef::epi_agent::logos_return`.
- Governing source: `docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md` §3–4 (twelve T/T′ meanings), `docs/kernel-rebuild/AGENT-PRACTICE-AND-BOOTSTRAP.md` (practice XP08), `docs/kernel-rebuild/OPTIMISATION-AND-LEARNING.md` §B08/B09/B11.
- Practice: XP08 — "Consume T/T′ with actual output and human correction". `logos_return` is a typed, deterministic, side-effect-free envelope producer (`learning_demonstrated:false`, `promotion_authority_granted:false`): it records the cycle and whether a fresh-participant uptake reference was supplied, it does not itself consume the material into Wiki/context/praxis, admit a named `= name` pattern, or promote canon. Actual consumption remains Central NOW / AIKit Wiki / Factory `RunThought` and, for named-praxis reuse, `aikit method`/`aikit routine` (see the AIKit-owned continuity Method).

## When

Use after a real report exists (via `ql-evidence-report`) and has been checked against actual retrieval/output/execution and an actual human or fresh-agent response — never from a paraphrase of a prior agent's summary. Use it to record the formal T (original)/C (comparison)/T′ (revised)/C′ (revised comparison) cycle with its source and evidence refs, and, where a fresh participant actually demonstrated later uptake with new input, to carry that reference forward. Do not use it to claim consumption, learning or named-praxis promotion happened merely because the envelope was produced.

## Inputs

`inquiry_ref` (the real question/subject this cycle answers); `T`/`C` (the original output and its comparison against expectation/human response); `T_prime`/`C_prime` (the corrected output and its comparison, after the actual retrieval/execution/human-response check); `source_refs` (non-empty, the actual sources consulted); `evidence_refs` (non-empty, the actual execution/output/Return evidence); optionally `practice_ref` (an existing named praxis this cycle bears on) and `fresh_participant_ref` (a real, observed fresh agent/participant who actually exercised the revised practice on new input — never invented to make `fresh_uptake_reference_present` read true).

## Authority

The envelope is read/compute-only: it validates and records a formal cycle with no mutation authority. It grants no Wiki write, no Factory Candidate Recognition, no canonical promotion, and no automatic `= name` registration. Actual consumption into useful context, Wiki knowledge, an improved Skill/Method, or a named reusable praxis remains its own native owner operation (Central receiving/NOW, AIKit Wiki, or the AIKit named-praxis continuity Method) and its own Recognition.

## Procedure

1. Produce the report through `ql-evidence-report`: the real difference, basis, and any open decision.
2. Compare that report against actual retrieval/output/execution and the person's (or a fresh agent's) actual response — not a restated summary of an earlier agent's confidence.
3. Write the `ql.epi-logos-agent-invocation/v1` request at faculty `#5`, operation `logos.return`, with the real `T`/`C`/`T_prime`/`C_prime`, `inquiry_ref`, non-empty `source_refs` and `evidence_refs`. Include `practice_ref`/`fresh_participant_ref` only when both are real and observed.
4. Invoke `ql epi-agent invoke <request.json> --json`. Read the returned `ql.epii-logos-return/v1` envelope exactly, including its `fresh_uptake_reference_present`, `learning_demonstrated:false` and `promotion_authority_granted:false` fields.
5. Route the recorded envelope to its actual native consumer (Central NOW/receiving for personal Return, AIKit Wiki for durable knowledge, the AIKit named-praxis continuity Method for `= name` registration) rather than treating the envelope itself as consumption. Name whichever consumption step has not yet happened.

## Outputs

The exact `inquiry_ref`, the T/C/T′/C′ cycle, source/evidence refs, and the returned envelope with its `standing` field ("performed-return-envelope; later-held-out-uptake-must-be-observed-separately"). State explicitly whether consumption (Wiki/context/praxis) and any named-praxis registration have actually happened yet, or remain the next dependency.

## Verification

Run the existing Rust workspace tests covering `ql-mef::epi_agent::logos_return` (`cargo test -p ql-mef --lib epi_agent`; a bare `cargo test -p ql-mef epi_agent` without `--lib` does not select the lib target and silently reports zero matching tests). `epi_agent::tests::logos_return_records_the_full_cycle_and_rejects_empty_fields` covers the positive cycle and the empty-field/missing-refs rejections directly. A green test proves the envelope's own validation logic; it proves no Wiki write, no Recognition and no fresh-agent uptake.

## Failure and recovery

Empty `T`/`C`/`T_prime`/`C_prime`, or missing `source_refs`/`evidence_refs`, is rejected by the operation itself — preserve the exact rejection and the original material rather than inventing refs to pass validation. A missing real fresh-participant observation means `fresh_participant_ref` stays absent; do not fabricate one to claim demonstrated learning.

## Continuity

Report through `ql-evidence-report`; record the envelope's exact refs in the authorised NOW/issue record so the next participant, or the actual consuming operation, can pick up from the same cycle rather than reconstructing it from prose.
