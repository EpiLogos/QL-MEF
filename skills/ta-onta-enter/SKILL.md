---
name: ta-onta-enter
description: "METHOD: Enter (Khora, S0′) — enter the exact world an act stands in and pick up where it left off: the ground, the project, the subject, the source basis, the session and the machine, read from their owners and never assumed. Routes AIKit's world reading and inhabitation, Central's Day and NOW, and QL's system reading, and hands Form a grounded context with its continuation. Use when an act opens or resumes — the Day, a session, a subject, a scene or a set — or when the world it stands in is not yet known. Part of Ta-Onta (ta-onta-methodology)."
---

# Enter

## Contract metadata

- Semantic ref: `ql:skill:ta-onta-enter` (`skill/ql/ta-onta-enter`), carried in the set `ql:ta-onta-enter` under `ql:ta-onta`.
- Native owners: AIKit (`aikit world`, `aikit work session`, `aikit history`); Central (`central.now.list`, `projectcentral.now.inspect`, `central.day.read`); Quaternal Logic (`ql system`, `ql capabilities`); Workcell for the material condition.
- Source: `docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md` §1 S0′ Khora, SP00–SP05; `docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md` §8 (the six S′ organs as the life of one act); O-I manual `ProjectCentral/docs/1-vision/dlw/DLW.md` §25.3.
- Used by: any agent acting in the agent world, through `skill/ql/ta-onta-methodology`.

Completion: the act knows, from the owners' own readings, which world it
stands in, which project and subject, which sources at which revision, which
session and machine, and where it left off. Nothing in that context is
assumed, and anything an owner could not say is named as unknown.

## The route

1. **Where am I.** `aikit world whoami` and `aikit world status` (add `--json`
   to keep the reading): the World, the Position, the Project binding and the
   effective view. `aikit world refocus` traces the act back to its
   ProjectCentral ground when the cwd and the ground disagree.
2. **What the register holds.** `ctrl action run central.now.list` (root) or
   `projectcentral.now.inspect` (a project), and `central.day.read` for the Day
   the act stands in: what was returned, what is open, what waits.
3. **Take the seat.** Where the act is an agent's turn in a world, occupy its
   Position: `aikit world inhabit --position <ref>`. A session that already
   stands is resumed, never opened twice: `aikit work session`.
4. **The participant NOW.** `aikit world now-context prepare` binds this
   participant's NOW for the act; `aikit world now-context inspect` reads it
   back. Each conversation is its own NOW.
5. **The machine and the law.** `ql system` and `ql capabilities` for the
   instruments that answer here; the material condition (the seat, the
   checkout, the branch) from Workcell and the World reading, never from a
   guess.
6. **Pick up the thread.** Read the continuation from what actually happened
   last: the last result, failure or clean-up state (`aikit history recent`,
   the NOW's returns). For an act inside an Expression, the scene it stands at
   is opened through the desktop's own expression owner.

## Hands on

A grounded context — World, Focus, subject, source revision, session and
material condition — with the C′ task frame of the act kept separate from it,
and the continuation it resumes. Form takes it from here.

## Route conditions

- No silent substitution: a world, a project or a session the owners do not
  name is reported as missing, never stood in for.
- Resolving a tool or a Position grants nothing. A refused inhabitation spawns
  nothing and is passed on in the owner's words.
- Enter makes no new Agent, Agency or identity. It seats the one that exists.
- Service health and usable capability are reported separately: a service
  that answers is not yet a capability that may be used.

## Source basis

- `docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md` §1 (S0′ Khora) and SP00–SP05.
- `docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md` §8 (enter exact World
  / subject / source basis / continuation).
- `docs/integrations/epi-logos/ANIMA-ALETHEIA-TEAMS.md` §4 (the crosswalk
  to native commands).
- O-I manual §25.3 (Enter).
