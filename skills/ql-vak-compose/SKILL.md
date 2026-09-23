---
name: ql-vak-compose
description: "METHOD: Compose the bounded formal C′ musical/Vāk relation (whole/compose/reframe/read/lineage/interpret/determine/return/CPF/CT/CP/CF/CFP/CS steps) for a selected subject through the real `ql vak compose <request.json>` engine, preserving use-ref/frame/basis identity and provenance. Use when a Vāk performance must be formally determined and read back before any downstream Factory/Actuation dispatch; this Method is the composition step only, not the enacted/dispatched performance."
---

# Compose the formal C′ relation

## Contract metadata

- Native source owner: EpiLogos/QL-MEF; executable support: `ql vak compose <request.json> [--json]` (contract `ql.vak-composition/v1`), dispatched from `crates/ql-cli/src/vak_composition.rs` into `ql_mef::vak_composition::VakComposition` / `crates/ql-mef/src/vak_composition/production.rs`.
- Governing source: `docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md`, `docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md` §AW2, `docs/kernel-rebuild/AGENT-PRACTICE-AND-BOOTSTRAP.md` (practice XP04).
- Practice: XP04 — "Compose and enact the original musical C′ relation" in `ux-spine-trace.json`. This Method covers **compose**, not **enact**: the engine runs a supplied batch of steps against a private in-memory graph and returns the resulting whole/determination/Return records with no external side effects. It never reaches Factory orchestration, Actuation authority or Workcell material execution.
- This Method does not supply a live scene, a real dated occasion or a musical/audio rendering. Those remain #132 K8.2/K8.3 runtime and desktop consumer work.

## When

Use when an authorised actor must formally determine (or read back) a C′ relation — `whole`, `compose`, `reframe`, `read`, `lineage`, `interpret`, `position`, `determine`, `return`, `offer`, `enter`, or a `cpf`/`ct`/`cp`/`cf`/`cfp` step — for a real subject before any consequential undertaking is authorised. Do not use it to claim that a musical/visual performance actually happened, was heard, was dispatched to Factory/Actuation, or affected any live instrument; the engine's own result carries no such claim.

## Inputs

The selected subject/use-ref and its exact revision; the requested step sequence as `ql.vak-composition/v1` JSON (`{"contract":"ql.vak-composition/v1","steps":[...]}`, written to a local file); the active `ActiveFrame`/`ContextFrameId`, ground reference/face and basis for each step; the caller's actual authority to author this composition (composition alone grants none). Read the current registry via `ql vak capabilities` first when the supported step/operator vocabulary is uncertain.

## Authority

Composition is read/compute-only against a fresh, private, in-process graph; each invocation starts empty and nothing persists between runs. It grants no Action authority, no Factory Commission, no Actuation Agency and no Workcell material effect. A successful batch is a formal reading, not permission to perform, publish or dispatch it. Land `EpiLogos/ai-kit#267` (Resolve/Path/ContextResolution binding) and `EpiLogos/Factory#217` (orchestration binding) — both closed as required native joins, but full live desktop/runtime harmonisation for actual C′ performance is separately evidenced per `UX-SPINE-RECONCILIATION.md` §4 — before treating any composed result as an enacted performance.

## Procedure

1. Resolve the real subject, exact source/registry revision, and the caller's actual authority for the intended undertaking (do not compose a relation the caller cannot subsequently take further).
2. Run `ql vak capabilities [--json]` to confirm the supported operators/forms at the installed revision.
3. Write the step batch as `ql.vak-composition/v1` JSON: bind a `whole`, `compose` its category/frame/basis, and `read`/`lineage`/`determine`/`return` as required by the actual undertaking. Preserve the subject's `useRef` and frame/basis identity across steps; do not silently substitute a different ground or category to make a step pass.
4. Invoke `ql vak compose <request.json> --json`. A failed step returns no successful receipt for that batch — do not retry the same batch expecting a different private-graph state, and do not hand-construct a result the engine refused to produce.
5. Read the returned whole/determination/Return view exactly as given; it is the composition, not the Return of an actual dispatched act. Carry the exact contract/schema, `useRef`, frame/basis and result into any downstream Factory/Actuation request that will actually enact it, and name that downstream step as still-required work.
6. If the requested step is unsupported, or the caller's authority does not extend to enactment, stop at composition and return the exact unsupported/unauthorised boundary rather than inventing a dispatch route.

## Outputs

The exact subject/`useRef`, source/registry revision, step sequence, and the engine's returned whole/determination/Return view with contract/schema. State plainly whether any downstream enactment (Factory Commission, Actuation Agency, Workcell material execution, desktop/audio rendering) was actually invoked, and if not, name it as the next dependency — never imply enactment from a successful composition alone.

## Verification

Run the existing Rust workspace tests covering `vak_composition`/`production` (`cargo test -p ql-mef --test vak_composition`, `cargo test -p ql-mef --test vak_composition_native_path`, `cargo test -p ql-cli --test vak_composition`; a bare `cargo test -p ql-mef vak_composition` does not select these integration-test binaries and silently reports zero matching tests — always name the exact `--test`/`--lib` target). A green composition test proves the formal engine's behaviour on its fixtures; it proves nothing about a live desktop/audio/dispatch chain. Use `ql-evidence-report` to return the exact composed/enacted distinction to the person.

## Failure and recovery

An unknown step, invalid coordinate/frame/basis, or unsupported operator is rejected by the engine itself; preserve the exact rejection and the caller's original request. A missing installed `ql` binary or stale revision blocks only this Method's own execution, not the source-level composition record. Never simulate the engine's JSON output by hand to make a downstream consumer proceed.

## Continuity

Compose within `ql-experience-prepare`'s established subject/World/NOW basis; report through `ql-evidence-report`. Preserve the composed result's exact `useRef`/revision so a later session, or the actual Factory/Actuation enactment step, can resume from the same formal determination rather than recomposing it from a paraphrase.
