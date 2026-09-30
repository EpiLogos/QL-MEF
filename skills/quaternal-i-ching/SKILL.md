---
name: quaternal-i-ching
description: "[Square C · L2/L2'/L3/L3' + Compass] — Quaternal I Ching divination protocol. Hexagram as six-fold QL reading with native tetralemma (four line-states), Klein twist via moving lines, nuclear hexagram as P4 lemniscate, Wu Xing/L2' elemental mapping, and dual compass framework (Early Heaven = Name-of-Power, Later Heaven = Power-to-Name). Logs completed readings to /Self/aham/daily/."
---

# Quaternal I Ching

> `using-epi-logos` runs first. If it hasn't been invoked this turn, go there now.

Quaternal I Ching reads the Book of Changes through the 4+2 toroidal architecture native to Square C — the Logic-Process basin (L2·L3·L3'·L2'). The protocol spec lives at `resources/methods/quaternal_i_ching_protocol.md` — **read it in full before running a reading.** This skill file gives the operative structure; the spec gives the interpretive depth.

## Square C Context

This skill operates within the Square C basin. The P-pair: **Word+Logos (Day/(No)Name)** / **Sacrifice+Decision (Night/Power)**. The I Ching's four line-states ARE the tetralemma operating natively: Young Yang=IS, Young Yin=IS-NOT, Old Yang=BOTH, Old Yin=NEITHER. The querent's question is the tetralemmaic ground (L2-0). The hexagram's inexhaustibility is SILENCE (L2-5). L3 and L3' are native — the I Ching is already a book of process and seasonal timing.

The square is the router. This skill refers back to Square C, not to sibling skills.

## The Two Compasses

The I Ching's two trigram arrangements carry the two faces of Self-Identity:

**Early Heaven (Fu Xi) = Name-of-Power** — the (No)Name (Śiva/Prakāśa) expressing through Power. The primordial equilibrium, the ideal structure. Father/Mother = the (No)Name face of #. Six children = the ontological unit (P Day): Truth, Mind, Word, Logos, Son, Image. In Taoist terms: *wu-ming* — the nameless knowing-intelligence.

**Later Heaven (King Wen) = Power-to-Name** — Power (Śakti/Vimarśa) reaching toward the Name. The manifest dynamic, the seasonal flow. Father/Mother = the Power face of #. Six children = the power unit (P' Night): Play, Need, Sacrifice, Decision, Love, Work. The manifestational hinge where dynamic force takes form.

Hold both compasses simultaneously: Early Heaven reveals the ontological pattern (what the situation IS in its naming-structure); Later Heaven reveals the power dynamic (what forces are actually moving and what they demand).

## Scales of Reading

**Trigram** — Cast three lines. Compass orientation: direction, element, season, quality of movement. Quick bearing.

**Hexagram** — Cast six lines. Full torus reading: lines as P0–P5 (Truth through Image), lower/upper trigram split, complementary pairs (Lines 1+6/Square A, Lines 2+5/Square B, Lines 3+4/Square C).

**Transformed Hexagram** — Moving lines flip to produce the Klein extension. The Night hexagram arises endogenously — the situation generates its own shadow. Day=what IS; Night=what it is becoming.

**Nuclear Hexagram** — Lines 2-3-4 and 3-4-5 produce the interior hexagram. This IS the P4 lemniscate unpacking — the hidden structure within the contextual field.

## Reading Sequence

1. **Theme** — state the question. This is the tetralemmaic ground.
2. **Cast** — six lines, bottom to top (yarrow or coin method).
3. **Identify the hexagram** — name, traditional judgement.
4. **Read trigrams** — lower (inner) and upper (outer). Note Wu Xing interactions.
5. **Read lines positionally** — P0/Truth through P5/Image. Note stable vs moving. Consult traditional line texts.
6. **Read complementary pairs** — Lines 1+6, 2+5, 3+4. What resonances?
7. **Read moving lines** — the paradoxical/transcendent positions. What is changing? Why here?
8. **Read transformed hexagram** (if moving lines exist) — the Night face. Day-Night relationship.
9. **Extract nuclear hexagram** (optional) — lemniscate depth.
10. **Apply lenses** — L2 (tetralemma across the whole), L3/L3' (process/season), L2' (Wu Xing + alchemical resonances).
11. **Whole-reading synthesis** — one living utterance.

## Tone

Honour the I Ching's own voice: terse, imagistic, allusive. The dragon in the field, the fox crossing the ice, the well whose water is clear. Let the images do their work. The QL-structural reading adds functional logic without displacing the oracle's imagistic power.

## After the Reading — the continuing journey

A reading belongs to the person's continuing journey for their concern, operated natively by Nara's faculty #4 through `ql epi-agent invoke` (`nara.journey.open`, `nara.journey.apply`, `nara.journey.read`, and `nara.lived-context.compose` for the person's Day/Flow material). `ql epi-agent faculty 4` shows a valid example input for each operation.

- **Open once, continue always.** One journey per concern, with one deck across it. A later session or Day continues the same journey: never open a new one to "refresh" it, and never reshuffle.
- **Every change is an act.** An entropy-bearing consultation is `cast-iching` (the original lines are kept whole). A form a native computation supplies is `record-computed-iching`. A later view of an earlier cast is `read-iching` with `kind: retrospective`, and it never recasts. Relating a Day/Flow passage to a card is `relate-event`. Your interpretation is `read` with `kind: original` or `development`.
- **The person's correction is a correction.** When they say a reading is wrong, record it as `read` with `kind: correction`, `actor_kind: human`, their words, their source passage, and `supersedes` naming the reading it corrects. The earlier reading stays in history.
- **Never edit journey state by hand.** Pass the journey back through `nara.journey.apply` and persist exactly what it returns; its Central owner stores it with a revision check. A hand-edited journey is refused by the native owner, and it is a false record either way.
- **Replays are safe.** Reusing a `request_id` for the same act reconciles to the first effect; it never deals or casts twice.

The reading's texts carry the interpretation this skill teaches (positional reading, complementary pairs, lenses, three levels, synthesis), with `source_refs` naming the passages they rest on. Personal material stays protected. Card/codon and hexagram/codon relations bind to Bimba coordinates in the scene through the map; do not copy them into the reading.

Check: what did Square C's basin reveal? What is becoming through the transformation (L3)? What season is this (L3')? What is dissolving/crystallising (L2')? Where does the tetralemma hold or break (L2)?
