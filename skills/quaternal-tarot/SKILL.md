---
name: quaternal-tarot
description: "METHOD: Read a Quaternal Tarot spread for a person's continuing concern — Sphere, Torus Day or Klein Day+Night positions, complementary pairs, the P4 lemniscate and the Square C lenses — and record it as acts on their one continuing journey through Nara's native faculty. Use when a concern calls for a Tarot reading or when a journey already open needs its next bearing, a relation to the person's writing, or their correction."
---

# Quaternal Tarot

Use when a person's concern calls for a Tarot reading, or when their open journey needs its next bearing, a relation to something they wrote, or their correction recorded.

## Contract metadata

- Semantic ref: `ql:skill:quaternal-tarot` (`skill/ql/quaternal-tarot`)
- Native owner: `EpiLogos/QL-MEF`, M4.2 Divinatory Frameworks. The journey is operated through faculty #4 (`ql epi-agent invoke`: `nara.journey.open`, `nara.journey.apply`, `nara.journey.read`, `nara.lived-context.compose`); `ql epi-agent faculty 4` discloses a valid example input per operation. Card and hexagram relations to codons are the Bimba map's (`REFLECTS_DNA_FORM`, `YIELDS_CODON`), bound in the scene.
- Source: `Body/S/S5/plugins/epi-logos/skills/quaternal-tarot/SKILL.md` (in Epi-Logos-C-Experiments), blob `a4276170c83a474be063b8d20dbcf50657b6ecb8`, with the protocol `Body/S/S5/plugins/epi-logos/resources/methods/quaternal_tarot_protocol.md` (in Epi-Logos-C-Experiments), blob `9929c5048cf4e7c268d550ef0a63cc3f1f280668`, carried here as `skills/quaternal-tarot/resources/methods/quaternal_tarot_protocol.md`. The source's "After the Reading" section (hand-written oracle artifacts in the retired vault) is replaced by the native journey route.
- Used by: `agent/nara` (owner of the reading); `agent/mahamaya` for the symbolic relations; `agent/epii` when teaching the method.

Quaternal Tarot reads any spread through the 4+2 toroidal architecture native to Square C — the Logic-Process basin (L2·L3·L3'·L2'). The protocol spec lives at `skills/quaternal-tarot/resources/methods/quaternal_tarot_protocol.md` — **read it in full before running a reading.** This skill file gives the operative structure; the spec gives the interpretive depth.

## Square C Context

This skill operates within the Square C basin. The P-pair: **Word+Logos (Day/(No)Name)** / **Sacrifice+Decision (Night/Power)**. The tetralemma (L2) is always co-present — every card position can be read through IS/IS-NOT/BOTH/NEITHER/SILENCE. The processual lens (L3) tracks what is becoming; the chronological lens (L3') reads the season; the alchemical lens (L2') reads what dissolves and what crystallises.

The square is the router. This skill refers back to Square C, not to sibling skills.

## Scales of Reading

**Sphere** — Single card. Compass orientation: what quality of energy characterises the moment? Quick bearing.

**Torus** — Six-card Day spread. One card per P-position (P0–P5). Full six-fold QL reading in its explicate form. The core reading.

**Klein** — Six-card Day + six-card Night spread. The Night arc inverts the Day: not contradiction but the analytic face, the shadow, what the Day reading rests on without naming. Together they complete the Klein double-cover.

## Reading Sequence

1. **Theme** — state the question in its deeper form. This is the tetralemmaic ground (L2-0).
2. **Draw and place** — cards into positions P0–P5 (and P0'–P5' if running Klein).
3. **Read positionally** — each card at its P-position. Use the position's question (Why/What/How/Who/Where/Why-for) as the interpretive key.
4. **Read complementary pairs** — P0+P5, P1+P4, P2+P3. What resonances and contradictions emerge?
5. **P4 lemniscate sub-reading** (optional) — draw an additional card or use the P4 card's internal symbolism to unpack the contextual frame's recursive depth.
6. **Night arc** (if Klein scale) — read the six Night cards as the analytic interior of the Day spread.
7. **Apply lenses to the whole** — L2 (tetralemma across the full reading), L3/L3' (growth phase, season), L2' (elemental transformation).
8. **Three-level interpretation** — each card on at least three levels: concrete (life situation), psychological (consciousness), spiritual/archetypal (deep pattern).
9. **Whole-reading synthesis** — weave into a single living utterance, not a sequence of technical observations.

## Tone

The Tarot's own voice is imagistic, symbolic, and allusive. Honour this: let the images speak before the structural analysis arrives. The QL-structural reading adds functional logic, dialectical relationship, and transformation structure — these should be offered clearly but without displacing the cards' own imagistic power.

## After the Reading — the continuing journey

A reading belongs to the person's continuing journey for their concern, operated natively by Nara's faculty #4 through `ql epi-agent invoke` (`nara.journey.open`, `nara.journey.apply`, `nara.journey.read`, and `nara.lived-context.compose` for the person's Day/Flow material). `ql epi-agent faculty 4` shows a valid example input for each operation.

- **Open once, continue always.** One journey per concern, with one deck across it. A later session or Day continues the same journey: never open a new one to "refresh" it, and never reshuffle.
- **Every change is an act.** A spread is `draw` (`sphere`, `torus-day`, `klein-night`, `lemniscate`, `single`, `closing`). The deck is shuffled once when the journey opens, and cards come off it in sequence. A card chosen for its meaning rather than dealt is `assign-symbol`, which leaves the deck untouched. Relating a Day/Flow passage to a card is `relate-event`. Your interpretation is `read` with `kind: original` or `development`.
- **The person's correction is a correction.** When they say a reading is wrong, record it as `read` with `kind: correction`, `actor_kind: human`, their words, their source passage, and `supersedes` naming the reading it corrects. The earlier reading stays in history.
- **Never edit journey state by hand.** Pass the journey back through `nara.journey.apply` and persist exactly what it returns; its Central owner stores it with a revision check. A hand-edited journey is refused by the native owner, and it is a false record either way.
- **Replays are safe.** Reusing a `request_id` for the same act reconciles to the first effect; it never deals or casts twice.

The reading's texts carry the interpretation this skill teaches (positional reading, complementary pairs, lenses, three levels, synthesis), with `source_refs` naming the passages they rest on. Personal material stays protected. Card/codon and hexagram/codon relations bind to Bimba coordinates in the scene through the map; do not copy them into the reading.

## Verify

A reading is verified when `nara.journey.read` returns the journey with the new act in its reading: the placements or hexagram, the reading entries with their authors and source passages, and the deck's dealt count unchanged except by a draw. The state passed to the next act must be exactly what the previous act returned.

If an act is refused, nothing changed: correct the request and send it again with the same `request_id`. If a journey file was edited by hand or is refused as invalid, recover by restoring the last state an act returned; do not repair it by editing.

Check: what did Square C's basin reveal as the primary transformation? What is dissolving (L2'), what is becoming (L3), what season is this (L3'), and where does the logic hold or break (L2)?
