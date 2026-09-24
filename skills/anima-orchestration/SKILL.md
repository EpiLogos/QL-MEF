---
name: anima-orchestration
description: "METHOD: Turn an evaluated Vāk block into an actual dispatch — which member takes each act, in which thread form, through which native route (a Claude Code subagent, a background task, a Factory workflow unit, or a Communique to another Position) — including Night′ Moirai routing and the Möbius return. Use after vak-evaluate whenever Anima composes work across its members or calls on the Aletheia team."
---

# Anima orchestration

## Contract metadata

- Semantic ref: `ql:skill:anima-orchestration` (`skill/ql/anima-orchestration`)
- Native owners of the routes: Claude Code `Agent`/`SendMessage` (in-session members), AIKit `aikit task spawn|list|close` (separate agent processes) and `aikit gateway send|delegate` (Positions), Software Factory `factory workflow check|compile|commission|inspect` and `factory development custody …` (commissioned work), QL-MEF `ql vak compose` (formal C′ determination).
- Replaces the Pi tools `anima_orchestrate`, `dispatch_agent`, `dispatch_parallel_agents`, `dispatch_fusion_agents`, `dispatch_moirai_night_pass`, `run_chain`, `subagent_create|continue|list|remove`, `anima_self_invoke`, `tilldone`.
- Source: `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/anima-orchestration/SKILL.md` (pinned blob `918aabec…`/`feac1be6…`; read at HEAD blob `84397e44…`), `S4-4p-anima/CONTRACT.md`, `S4-4p-anima/extension/dispatch-tools.ts`.
- Used by: `agent/anima` (owner), `agent/anima-psyche` (coordinator plans).

## Inputs

A `skill/ql/vak-evaluate` block with CPF `authorised-undertaking` and its authority; the concern, required difference and Return address; the subject (repo, Project, NOW clearing, Expression, constellation); the members actually available in this body.

## Frame → member

| Frame | Member | Takes |
|---|---|---|
| CF1 `(00/00)` | `agent/anima-nous` | clearing only; never the task executor. Returns questions and sources; re-evaluate after. |
| CF2 `(0/1)` | `agent/anima-logos` | scope, specification, plan |
| CF3 `(0/1/2)` | `agent/anima-eros` | relational scour, operation, tests, verification |
| CF4 `(0/1/2/3)` | `agent/anima-mythos` | pattern, debugging, archetypal reading |
| CF5 `(4.0/1-4.4/5)` | `agent/anima` | the dispatch itself — not dispatched to |
| CF6 `(4.5/0)` | `agent/anima-psyche` | continuity, NOW, coordination plans, Aletheia calls |
| CF7 `(5/0)` | `agent/anima-sophia` | synthesis, Night′ lead, Möbius return, branch finishing |
| — | `agent/anima-techne-helper` | worktrees and terminal workspaces, on explicit instruction only |

The Aletheia team (`agent/aletheia` and its specialists Anansi, Janus, Moirai, Mercurius, Agora, Zeithoven) is called through Psyche's and Sophia's acts, not routed by frame. Techne on a constellation is Aletheia_i in M_i′ (see the team's `TEAM.md`); Anima does not dispatch a "Techne agent".

## Thread form → native route

| CFP | In one session (lead body) | Across sessions or commissioned |
|---|---|---|
| CFP0 base | one `Agent` call with `subagent_type` = the member slug | one Factory workflow unit |
| CFP1 parallel | several `Agent` calls in one message, each with its own bounded brief | units with `independenceFrom` and no shared dependency |
| CFP2 chain | sequential `Agent` calls; pass only the selected result forward, never the transcript | units with `dependencies` and a verification obligation between phases |
| CFP3 fusion | the same brief to several members, then `agent/aletheia-agora` fuses | parallel units plus a barrier and an Agora unit |
| CFP4 sustained | a task list that closes only when every task is done; running out of cycles is reported as exhaustion, not completion | a unit whose `verificationObligations` gate its close; `factory workflow inspect` |
| CFP5 nested | Claude Code subagents cannot spawn subagents: a member returns a sub-plan and the lead dispatches it | Factory `nesting` |
| Z | compose → perform → record → rehear → recompose under an authorised undertaking | Factory commission; the formal Z stages through `ql vak compose` (`z-begin`, `z-advance`) |

A member that must run as its own process (long, isolated, other model): `aikit task spawn <name> --agent claude --worktree`, then `aikit task list` / `aikit task close`. A task for another Position (another Anima_i, an Aletheia_i, a guardian): `aikit gateway send` to its `central:position:…` ref; `aikit gateway delegate` when it must become custody-bearing work.

## Factory workflow units

When the work is commissioned (development, Expression production, Technè on a constellation), write it as a Factory TypeScript workflow (`factory workflow sdk <dir>` gives the SDK; `factory workflow check|compile <source.workflow.ts>`; `factory workflow commission <state> <request.json> <source.workflow.ts>`). Each unit names its participants and practice:

```json
"agentRequirements": {
  "agentSetRefs": ["central:pasu:agent-set:anima"],
  "agentRefs": ["agent/anima-logos"]
},
"praxisRefs": ["skill/ql/vak-evaluate", "skill/personal/writing-plans"]
```

The Vāk block is the unit's C′ reading; Factory owns the run, attempts, custody and Return. Generic Factory workflows need none of these fields.

## Night′ Moirai routing

| Night′ position | Moirai mode | Frame | Act |
|---|---|---|---|
| P1′ traces | Klotho | CF3 | assert and validate evidence |
| P4′ discovery | Lachesis | CF5 | retrieve and triangulate sources |
| P5′ insight | Atropos | CF7 | cut to essential synthesis, seed forward |

The mode activates when the returning passage reaches its position; it is one actor (`agent/aletheia-moirai`) in three modes, not three agents. A full Night′ fusion (CFP3) runs all three and returns to Psyche and Sophia for aggregation.

## Möbius return

Sophia closes the Night′ pass with the P5′ insight and the P0′ questions it opens. Record them in the active NOW as thought fixtures (`ctrl --json action run central.now.thoughts.append` with `reading` `T5` and `T0`), and carry open questions as a NOW return (`projectcentral.now.return`, `kind` `question`). The next `vak-evaluate` reads them.

## Human checkpoints

A dialogical checkpoint exists only because it was authored onto a step for one of three reasons: implied by the task, requested at origination, or learned from review. Only a human answers it; an agent answering its own checkpoint is not an answer. In Factory this is a unit's `escalationConditions`/`stopConditions`.

## Output

```text
ANIMA-ORCHESTRATION: <task-short-name>
Members: <agent refs>  Frames: <CF codes>  Form: CFP<n>|Z
Route: agent-tool | aikit-task | factory-unit | gateway-communique
Moirai: klotho|lachesis|atropos|none   Return: <address>
```

## Authority and limits

Orchestration chooses who acts and how; it does not create authority. Factory commission, custody, Actuation occupancy and any Workcell effect keep their own authority. Never report simulated personas as independent subagents: if a voice was not actually dispatched, say it was the lead's own reading.
