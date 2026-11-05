---
name: ta-onta-time
description: "METHOD: Time (Chronos, S3′) — hold the act's times together without owning any of them: the Day and NOW (Central's), the session and its history, each source's freshness, the run (Factory's), and the sky and the clock (Quaternal Logic's), each read with its epoch and generation so live, historical and retrospective readings never merge. Routes Central's Day, NOW and civil time, AIKit's history and continuity, Factory's run and journey, and QL's scene world, and hands Compose and Return a dated basis. Use when an act must know whether a reading is current, which run or occasion it belongs to, or what the clock and the sky say now. Part of Ta-Onta (ta-onta-methodology)."
---

# Time

## Contract metadata

- Semantic ref: `ql:skill:ta-onta-time` (`skill/ql/ta-onta-time`), carried in the set `ql:ta-onta-time` under `ql:ta-onta`.
- Native owners: Central (`central.day.read`, `central.now.*`, `central.time.policy`, `central.time.occurrences`); AIKit (`aikit history`, `aikit work continuity`, `aikit routine`); Software Factory (`factory development run`, `journey`, `observations`); Quaternal Logic (`ql scene world`).
- Source: `docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md` §1 S3′ Chronos, SP30–SP35; `docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md` §8 (the six S′ organs as the life of one act); O-I manual `ProjectCentral/docs/1-vision/dlw/DLW.md` §25.3.
- Used by: any agent acting in the agent world, through `skill/ql/ta-onta-methodology`.

Completion: every reading the act relies on carries its epoch, generation and
freshness, and the act knows which Day, NOW, session, run and occasion it
stands in. Nothing is dated by guess, and nothing is closed here.

## The route

1. **The Day and NOW.** `ctrl action run central.day.read`, `central.now.list`
   or `central.now.read` (root), `projectcentral.now.inspect` (a project). The
   Day and its close are Central's; Time reads them.
2. **Civil time.** `ctrl action run central.time.policy` for the person's
   civil-time policy and `central.time.occurrences` for when a schedule falls.
   A schedule is Central's grammar; pass its refusal on in its words.
3. **The session's history.** `aikit history recent` (and `aikit history
   failures`) for what happened in this work, and `aikit work continuity` for
   the continuity engine's own reading of a close-out.
4. **The run.** `factory development run <state> <run-ref>` and `factory
   development journey <state> <journey-ref>` for a commissioned run's history
   and where its needle stands; `factory development observations` for what was
   observed on it.
5. **Repeatable practice.** `aikit routine list` and `aikit routine show` for
   what runs on a schedule; `aikit routine options` and `prepare-schedule` read
   what a new one could be. Creating or enabling one is the person's act.
6. **The sky and the clock.** `ql scene world` for the actual sky and the
   harmonic phase where the act reads them. `chronos-act-continuity` carries
   the act's own continuity steps.
7. **Date each reading.** Keep each reading's epoch and generation with it, and
   keep live, historical and retrospective readings apart.

## Hands on

A dated basis: the Day, NOW, session, run and occasion the act stands in, each
reading with its epoch, generation and freshness. Compose builds on it; Return
reads the act against it.

## Route conditions

- Chronos relates time; it does not own it. It never closes a Day or a NOW and
  never moves a schedule.
- The forward and returning directions of a pass are a logical sequence, not
  clock time.
- A raw intention does not create autonomous work: nothing is scheduled to run
  by itself without the person's act.
- Stored history is not yet interpreted; reading it is Return's.
- The person's own occasion (Kairos) has no owner reading yet; where an act
  needs it, say so rather than standing one in.

## Source basis

- `docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md` §1 (S3′ Chronos: "Central owns
  Day/NOW") and SP30–SP35.
- `docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md` §8 (bind currentness).
- `skills/chronos-act-continuity/SKILL.md` (authority and limits).
- O-I manual §25.3 (Time).
