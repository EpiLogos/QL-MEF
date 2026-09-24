---
name: vak-coordinate-frame
description: Reference grammar for the six C′ Vāk layers an Anima or Aletheia act is written in — CPF participation, CT content type, CP position, CF context frame, CFP thread form and CS sequence — with the current QL codes, the constitutional voice of each frame, the Day/Night′ question pairs and the thought meanings they return into. Use whenever a coordinate, frame, thread form or passage must be named or checked; it is a lookup, not a procedure.
---

# Vāk coordinate frame

## Contract metadata

- Semantic ref: `ql:skill:vak-coordinate-frame` (catalogued by AIKit as `skill/ql/vak-coordinate-frame`)
- Native owner: `EpiLogos/QL-MEF`; executable vocabulary: `ql context-frame list`, `ql vak compose` (`ql.vak-composition/v1`, `crates/ql-cli/src/vak_composition.rs`), `constitutional_voice` in `crates/ql-mef/src/vak_profile.rs`
- Source: `EpiLogos/Epi-Logos-C-Experiments` `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/vak-coordinate-frame/SKILL.md` (pinned blob `b12f3f47…` in `docs/integrations/epi-logos/aw0-sources-original-skills.json`; read at HEAD blob `adb34b1b…`)
- Teams: Anima (all members), Aletheia (gates and traversal). Risk class: reference only — no authority, no mutation.

## The six layers

| Layer | Question | Current values |
|---|---|---|
| CPF — participation | Dialogue with the person, or an admitted undertaking? | `dialogical` `(00/00)` · `authorised-undertaking` `(4.0/1-4.4/5)` with its authority ref |
| CT — content type | What kind of content does the act make or use? | CT0 relational · CT1 definitional · CT2 operational · CT3 pattern · CT4 contextual · CT4b′ contextual artifact through Day/NOW · CT5 integrative |
| CP — position | Where on the 4.x lattice does the act sit? | 4.0 ground · 4.1 definition · 4.2 operation · 4.3 pattern · 4.4 context · 4.5 integration |
| CF — context frame | Which frame (and so which constitutional voice) holds it? | see the frame table |
| CFP — thread form | How is execution shaped? | CFP0 base · CFP1 parallel · CFP2 chain · CFP3 fusion · CFP4 sustained · CFP5 nested · Z compose→perform→record→rehear→recompose |
| CS — sequence | Which paired passage, in which direction? | CS0 full-traverse · CS1 quick-grounding-context · CS2 ground-through-operation · CS3 through-pattern · CS4 context-focused · CS5 direct-synthesis; direction `forward-synthesis` (Day) or `returning-inquiry` (Night′) |

A CP is also the nesting operator: a position either holds one leaf act or opens a whole nested frame. Frames never act by themselves; only leaves are dispatched.

## Frames and their constitutional voices

`ql context-frame list` is the authority for codes and expressions. The voice column is `constitutional_voice` in `vak_profile.rs`.

| CF | Expression | Frame | Voice (agent) |
|---|---|---|---|
| CF1 | `(00/00)` | fourfold zero, undifferentiated ground | Nous (`agent/anima-nous`) |
| CF2 | `(0/1)` | non-dual anchor | Logos (`agent/anima-logos`) |
| CF3 | `(0/1/2)` | dual-non-dual, triadic circulation | Eros (`agent/anima-eros`) |
| CF4 | `(0/1/2/3)` | trinitarian, tetradic closure | Mythos (`agent/anima-mythos`) |
| CF5 | `(4.0/1-4.4/5)` | fractal-doubling executive | Anima (`agent/anima`) — the dispatch itself |
| CF6 | `(4.5/0)` | .5 bridge | Psyche (`agent/anima-psyche`) |
| CF7 | `(5/0)` | total synthesis, Möbius closure and reopening | Sophia (`agent/anima-sophia`) |

There is no eighth frame. Aletheia is not a constitutional voice. Aletheia members work inside these frames (for example Moirai in CF3, Zeithoven in CF7) without becoming the voice of that frame. CF1 does not execute a task: Nous clears the ground, then the act is re-evaluated.

## Thread forms

| CFP | Form | Shape |
|---|---|---|
| CFP0 | base | one voice |
| CFP1 | parallel | different tasks, independent voices, composed as a chord |
| CFP2 | chain | a returned result becomes the next voice's material; validate between phases |
| CFP3 | fusion | the same task to several readers, fused by an independent examination (Agora) |
| CFP4 | sustained | long work with continuation and an explicit stop; closes only when its task list is done |
| CFP5 | nested | a composition inside another composition |
| Z | cycle | compose → perform → record → rehear → recompose, under an authorised undertaking |

## Passages

| CS | Pairs (forward) | Steps |
|---|---|---|
| CS0 | 4.0↔4.5 · 4.1↔4.4 · 4.2↔4.3 · 4.3↔4.2 · 4.4↔4.1 · 4.5↔4.0 | 6 |
| CS1 | 4.0↔4.5 · 4.1↔4.4 | 2 |
| CS2 | 4.0↔4.5 · 4.1↔4.4 · 4.2↔4.3 | 3 |
| CS3 | 4.0↔4.5 · 4.1↔4.4 · 4.2↔4.3 · 4.3↔4.2 | 4 |
| CS4 | 4.0↔4.5 · 4.4↔4.1 · 4.5↔4.0 | 3 |
| CS5 | 4.0↔4.5 · 4.5↔4.0 | 2 |

Day and Night′ are CS directions — logical forward synthesis and returning inquiry. They are not civil time. The civil day belongs to Central (`ctrl --json action run central.time.policy '{}'`).

## Day questions, Night′ questions, thought meanings

| CP | Day question | Night′ question | Thought meaning (Central NOW `T/`) |
|---|---|---|---|
| 4.0 | What do we have? | What don't we know? | T0 Question |
| 4.1 | What must be true? | What evidence exists? | T1 Trace |
| 4.2 | What is being done? | What blocks us? | T2 Challenge |
| 4.3 | What shape does it take? | What repeats? | T3 Pattern |
| 4.4 | Where and when in the larger frame? | What sources inform? | T4 Discovery |
| 4.5 | What was produced? | What crystallises? | T5 Insight |

The prime readings are T0-prime Assumption, T1-prime Lacuna, T2-prime Affordance, T3-prime Anomaly, T4-prime Concealment, T5-prime Integration. Night′ runs 4.5 → 4.0; the Möbius return is P5′ insight generating P0′ questions for the next forward pass.

## Standard coordinate block

```text
VAK: <task-short-name>
CPF: dialogical|authorised-undertaking  CT: CT<n>[,CT<n>]  CP: 4.<n>
CF: CF<n> (<expression>) -> <voice>  CFP: CFP<n>|Z  CS: CS<n> / forward-synthesis|returning-inquiry
```

A block that differs only on CF is one axis with five decorations. Count the layers on which the steps of a composition genuinely differ; do not count fields present.

## Practice skills and their frames

| Practice skill | Frame alignment |
|---|---|
| `skill/personal/brainstorming` | CPF dialogical; CF2 or CF1 when the ground is unclear |
| `skill/personal/writing-plans` | CF2 Logos |
| `skill/personal/test-driven-development` | CF3 Eros, CP 4.2 |
| `skill/personal/systematic-debugging` | CF4 Mythos, CP 4.3 |
| `skill/personal/subagent-driven-development` | CFP2 chain |
| `skill/personal/dispatching-parallel-agents` | CFP1 parallel and CFP3 fusion |
| `skill/personal/executing-plans` | CFP4 sustained |
| `skill/personal/verification-before-completion` | CF7 and partial Night′ checks (P5′/P2′/P1′) |
| `skill/personal/finishing-a-development-branch` | CF7 Sophia, Möbius return |

## Provenance and limits

The source's "40 days / 40 nights" sequence count rests on the 20-frame/40-direction claim, which the AW0 census keeps as an unproved source discrepancy; do not repeat it as fact. Older numbering that started Aletheia's own frames at CF0 is historical: use the `ql context-frame list` codes.
