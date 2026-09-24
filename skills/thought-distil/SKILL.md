---
name: thought-distil
description: "METHOD: Distil the thought stream of a NOW clearing (T0–T5 and their prime readings) into a reviewable learning during the Night′ rehear — only after traces and sources are present — carrying its full envelope, and hand anything that needs review to Epii's owner-side path instead of promoting it. Use for the Atropos cut, for Sophia's crystallisation, and whenever a learning is ready to leave NOW."
---

# Thought distil

## Contract metadata

- Semantic ref: `ql:skill:thought-distil` (`skill/ql/thought-distil`)
- Native owners: Central `central.now.thoughts.read`, `central.now.learnings.distill`, `central.now.learnings.read`, `projectcentral.now.promote` (the only door from NOW into a wiki); QL-MEF `ql epi-agent invoke` faculty #5 `logos.return` (`skill/ql/ql-logos-return`) for the typed T/C/T′/C′ envelope; AIKit `aikit gateway send` to reach the Epii-side Position.
- Replaces the Pi tools `aletheia_crystallise`, `aletheia_session_promote`, `aletheia_ingest`, `aletheia_episodic_ingest_thoughts`.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/thought-distil/SKILL.md`, blob `6893c1b34b351966748b3e1256da6d35ca0f1650` (pinned and HEAD agree).
- Used by: `agent/aletheia-moirai` (Atropos mode), `agent/anima-sophia` (review pressure), `agent/aletheia`.

## Procedure

1. **Gather the full descent first.** Read the clearing's thoughts:

```bash
ctrl --json action run central.now.thoughts.read '{"now_ref":"<ref>","include_content":true}'
```

   Do not cut before P1′ traces (`T1`) and P4′ sources (`T4`) are present. A crystallisation over a partial rehear is premature — return `deferred` and name what is missing.
2. **Carry the envelope.** Source fixtures, the Vāk block the work ran under, session/Day/NOW lineage, privacy and review class, and whether it is a trigger, a draft or a candidate for promotion.
3. **Distil, do not promote.** Write the learning from its named fixtures:

```bash
ctrl --json action run central.now.learnings.distill '{"now_ref":"<ref>","slug":"<slug>","day":"<YYYY-MM-DD>","actor":"agent/aletheia-moirai","actor_kind":"agent","reading":"T5","source_fixtures":["<T fixture>", "…"],"content":"<what the stream means, in plain statements>"}'
```

   When the T/T′ cycle must be recorded formally (actual output against corrected output), run `skill/ql/ql-logos-return` and keep its envelope ref with the learning.
4. **Route.** A project learning that deserves wiki standing goes through `projectcentral.now.promote` with `target` `agent-wiki` and `acceptance` `agent-return`; the wiki owner's maintenance incorporates it. Material that needs Epii's review, canon or pedagogy is handed to the Aletheia/Epii Position as a NOW handoff (`projectcentral.now.return`, `kind` `handoff`) and, if a body occupies it, a Communique (`aikit gateway send`). Human-ground change goes to the owner through Recognition, never by this Method.
5. **Open, not close.** A distillation that ends inquiry is Sophia's error. Name the P0′ questions it opens (`T0` thoughts) for the next cycle.

## Output

`crystallised → epii` (with envelope and refs) · `deferred` (rehear incomplete; what is missing) · `unavailable` (a native door refused; the exact refusal). Never a promotion decision.

## Limits

Never write `wiki.json`; never mark anything recognised; never promote human ground. Root-register learnings stay in the root NOW's learnings until the root wiki owner's procedure takes them up.
