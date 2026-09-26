---
name: chronos-act-continuity
description: "METHOD: Relate an Expression act to its occasion, order, completion and continuation — read an act's passages (act_list, act_inspect), re-enter a timeline position (act_seek), play an Expression's saved playback order with its recorded bindings and text, resume an ongoing act when its content changes, and route a finished or unfinished piece of work to a completion or continuation passage. Consumes Central Day/NOW and AIKit Routines for the actual time and continuation state. Use when resuming, replaying or continuing an act, when work returns to a parent Run, or when choosing between forward and returning passages."
---

# Chronos act continuity

## Contract metadata

- Semantic ref: `ql:skill:chronos-act-continuity` (`skill/ql/chronos-act-continuity`)
- Native owners: the O:I desktop kernel through `oi desktop expression [SOCKET] REQUEST_JSON` (world requests with `"schema":"oi.expression-world/v1"`: `act_list`, `act_inspect`, `act_seek`, `act_open`, `act_select`, `act_text`, `act_continue`, `act_complete`, and the existing `act_checkpoint`/`act_restore`); acts persist under `$OI_HOME/desktop/expression-acts/` and survive restart. Time and continuation state: Central `central.day.read`, `central.now.list`, `central.now.read`, `projectcentral.now.inspect`, `central.time.policy`; scheduled re-entry: `aikit routine list|show|create` (a created Routine stays Draft until the owner enables it).
- Contract: O:I `docs/contracts/EXPRESSION-ACT-MATERIAL-V1.md` §4 (act, `sequence`, `continuations`, `position`).
- Source: O:I `docs/cradle/handovers/factory-expressions-2026-09-26/EXPRESSION-DEVELOPMENT-SPEC.md` §5 (timeline and playback), §6, §7 (Chronos row and source mechanics), §9 item 10. Behavioural provenance in `EpiLogos/Epi-Logos-C-Experiments` at `c7872e96`: `Body/S/S4/ta-onta/S4-3p-chronos/modules/parent-slice-bifurcation.ts` (blob `481441b9`), `temporal-control-plane.ts` (blob `d62113dd`), `temporal-frame.ts` (blob `e800c1ab`). Those modules are provenance for the meanings below, not code to port; the current suite implements the time.
- Used by: `agent/anima-psyche` (continuity of the act across occasions), `agent/aletheia-janus` (the before/after threshold, session and day seams), `agent/anima-sophia` (the returning passage that closes a pass). Composition of new passages is `skill/ql/anima-expressive-composition`; Return is `skill/ql/aletheia-expressive-return`.

## Inputs

An `act_ref`, or the target Expression whose acts are to be found; the occasion (the current Day and the NOW clearing the work belongs to); for playback, the Expression material's `file_ref` or the completed act; for continuation, the result of the finished or unfinished work (its evidence and whether it completed).

## 1. Read the timeline

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_list"}'
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_inspect","act_ref":"<act_ref>"}'
```

The act's `sequence[]` is its timeline: each passage has `index`, `kind` (`scene | state | gesture | text | operate | continue | return`), material `file_ref@revision` and `scene_ref`, bindings, captions, transition, `event_basis` and `mode`. `position` is the current passage; `continuations[]` records mode changes; `phase` is `running | held | completed | cancelled`. Read the occasion alongside it: `ctrl --json action run central.day.read '{}'` and `ctrl --json action run projectcentral.now.inspect '{"project":"<Name>"}'` (or `central.now.list` at the root).

## 2. Re-enter a position

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_seek","act_ref":"<act_ref>","position":3,"actor":"agent:anima-psyche"}'
```

`act_seek` re-performs passage N with its recorded material, bindings, text and transition; selecting a point on the timeline selects that state. Take an `act_checkpoint` before seeking inside a running act whose later passages must be kept.

## 3. Play an Expression

Playback is the Expression's Scene sequence, performed through with its recorded bindings and text — nothing else:

- a **completed act**: `act_seek` each passage index in order, from 0 to the last;
- a **saved Expression** (for example `expression/expression-development.expression.json`, playback arrival → work-passage → skill-invocation → handoff → review → completion): read its `reuse.playback` from `material_list`, open an act on the target Expression, and `act_select` each `scene_ref` in `playback` order with the bindings recorded in the Run's act (or the cast now present when there is no recorded act).

Keep each passage's authored `duration`, `transition`, easing, morph, camera and sound; do not re-time them in the Method.

## 4. Completion or continuation (parent-slice bifurcation)

When a piece of work inside an act returns, route it by whether it completed:

| Work state | Source meaning | Act operation |
|---|---|---|
| completed (`c=1`) | the result folds into the parent's continuation | the child act ends with `act_complete` (`return_ref`, result text) — a `return` passage; the parent act then performs its completion or review passage (`act_select` of the completion or review Scene, `act_text` with `resultText`) and continues from there |
| incomplete (`c=0`) | the same agent continues on its own history | no completion; the same act, same cast and same lead continue: `act_select` the work passage again with unchanged bindings, `act_text` with the new progress, a `continue` passage when the mode changes (`act_continue`) |

Never complete an act to hide unfinished work, and never hand an unfinished piece to a different agent through this route.

## 5. Re-entry through changed content (temporal control plane)

An ongoing act is reopened, not replaced: `act_open` with the same `act_ref` resumes its cast, material and selection. When its content has changed since the last passage (a new attempt reading, a changed artifact, a new message), update the objects and text in place — `act_text` for text and value roles, `act_select` with the same material and updated bindings for objects — and let the next passage carry the change. A response due later is a NOW obligation or an AIKit Routine the owner enables (`aikit routine create` from a proven basis); this Method does not keep its own schedule or ledger.

## 6. Direction (temporal frame)

The occasion's direction selects the passage family:

| Direction | Occasion | Passages |
|---|---|---|
| outward (Day; `direction: "forward"` in a workflow unit's C′) | work going out: arrival, task start, work, invocation, handoff | `arrival`, `work-passage`, `skill-invocation`, `handoff` |
| returning (Night′; `direction: "returning"`) | work coming back: review, integration, Return | `review`, `completion`, an explanation of what was learned |

Choose material by the direction the act is actually in; a returning occasion does not replay the outward passages as if new.

## Output

```text
CHRONOS-CONTINUITY: <act_ref> phase=<running|held|completed|cancelled> position=<n>/<len>
Occasion: day=<day> now=<clearing ref>  Direction: outward|returning
Action: read | seek <n> | playback <file_ref@revision> | fold (return_ref) | continue (same lead) | re-enter (changed: …)
```

## Authority and limits

Chronos relates time; it does not own it. Day and NOW state are Central's, schedules are AIKit Routines under owner authority, and passages are the kernel's. The Method never rewrites a passage (a restore is a new revision), never closes a Day or NOW clearing (that is `skill/personal/central-day-close` and the NOW Actions), and never invents a timeline the act record does not hold.
