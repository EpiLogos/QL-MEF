---
name: day-night-pass
description: "METHOD: Run the full torus over an undertaking — a Day pass forward through 4.0→4.5 asking what we have, what must be true, what is being done, what shape, where, what was produced; then a Night′ pass returning 4.5→4.0 with the orthogonal questions (what crystallises, what sources, what repeats, what blocks, what evidence, what we don't know) — and close with the Möbius return that seeds the next cycle. Use when vak-evaluate selects both directions, CS0, or a complex CS2/CS3 undertaking."
---

# Day / Night′ pass

## Contract metadata

- Semantic ref: `ql:skill:day-night-pass` (`skill/ql/day-night-pass`)
- Native owners: QL-MEF (`ql vak compose` steps `cs-select` with `forward-synthesis`/`returning-inquiry` and `cs-hop`); Central NOW (`central.now.thoughts.append`, `central.now.learnings.distill`, `projectcentral.now.return`).
- Source: `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/day-night-pass/SKILL.md` (pinned `7a089284…`/`9f705445…`; HEAD `7f8d8d42…`).
- Used by: `agent/anima-psyche` and `agent/anima-sophia` (shared owners), `agent/anima`; calls `agent/aletheia-moirai`.

Day and Night′ are the two directions of the CS passage — logical synthesis and returning inquiry — not the civil day. The civil Day and its close belong to Central.

## When

- The evaluated block has CS with both directions, or CS0.
- An authorised undertaking with CS2/CS3 is complex enough that its result must be interrogated before it is called done.
- In dialogue (CPF dialogical) the person may take the Night′ pass themselves; offer the questions, do not answer for them.

Not for simple forward execution.

## Day pass — forward

| CP | Question | Usual voice |
|---|---|---|
| 4.0 ground | What do we have? | the frame assigned by orchestration |
| 4.1 definition | What must be true? | Logos (CF2) |
| 4.2 operation | What is being done? | Eros (CF3) — red, green, refactor happen here |
| 4.3 pattern | What shape does it take? | Mythos (CF4) |
| 4.4 context | Where and when in the larger frame? | Psyche (CF6) |
| 4.5 integration | What was produced? | Sophia (CF7) |

## Night′ pass — returning

Night′ asks different questions; it is an inversion, not a re-read of the Day.

| From | Night′ position | Question | Who |
|---|---|---|---|
| 4.5 | P5′ insight | What crystallises? | Moirai as Atropos, with Sophia (CF7) |
| 4.4 | P4′ discovery | What sources inform? | Moirai as Lachesis (CF5) |
| 4.3 | P3′ patterns | What repeats? | Mythos (CF4) |
| 4.2 | P2′ challenges | What blocks us? | Eros (CF3) |
| 4.1 | P1′ traces | What evidence exists? | Moirai as Klotho (CF3); the source also names Logos here — use whichever frame actually holds the evidence |
| 4.0 | P0′ questions | What don't we know? | Nous (CF1), fresh context |

**Integration gate at P2′.** If challenges remain, return them to Psyche; they may open a new Day cycle. Do not crystallise over unresolved blocks.

A full Night′ fusion (CFP3) sends Klotho, Lachesis and Atropos in parallel; Psyche aggregates their returns into one Night′ account (use `agent/aletheia-agora` when the returns disagree).

## Recording

Each Night′ answer is a thought in the active NOW, attributed to the member who produced it:

```bash
ctrl --json action run central.now.thoughts.append '{"now_ref":"<now ref>","slug":"<short-slug>","day":"<YYYY-MM-DD from central.time.policy>","actor":"agent/aletheia-moirai","actor_kind":"agent","reading":"T1","content":"<plain statement with its source refs>"}'
```

(Add `"project":"<Name>"` for a project NOW.) Readings: P0′→`T0`, P1′→`T1`, P2′→`T2`, P3′→`T3`, P4′→`T4`, P5′→`T5`.

When the formal passage must be proven, run it through the engine — `cs-select` with `returning-inquiry`, then positioned `cs-hop` steps (template: `fixtures/kernel/vak-cprime-oikonomia-v1.json`).

## Möbius return

1. P5′ insight crystallises (Atropos with Sophia).
2. It generates P0′ questions — new unknowns.
3. Record both (`T5` and `T0`), distil the learning if one is ready (`skill/ql/thought-distil`), and carry open questions as a NOW return (`projectcentral.now.return`, `kind` `question`).
4. The next `skill/ql/vak-evaluate` reads those questions.

```text
MOBIUS_RETURN: <P5′ insight> | <P0′ questions>
```

## Authority and limits

The pass records and interrogates; it promotes nothing. Durable learning leaves NOW only through `skill/ql/thought-distil` and the promotion door. Closing the civil day is Central's `central.day.lifecycle`, which needs the owner's native token; a member does not close the day.
