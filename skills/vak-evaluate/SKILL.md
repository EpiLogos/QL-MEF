---
name: vak-evaluate
description: "METHOD: Assign the six C′ Vāk coordinates (CPF, CT, CP, CF, CFP, CS) to an incoming task before anyone acts on it — silently for a clear task, explicitly with the person for an ambiguous one — and, when the act must be formally determined, check the block through the real `ql vak compose` engine. Use at the start of every Anima or Aletheia act; a dialogical CPF hands the task to brainstorming instead of dispatch."
---

# Vāk evaluate

## Contract metadata

- Semantic ref: `ql:skill:vak-evaluate` (`skill/ql/vak-evaluate`)
- Native owner: `EpiLogos/QL-MEF`. Executable support: `ql context-frame list`, `ql vak compose <request.json> --json` (contract `ql.vak-composition/v1`; steps `whole`, `enter`, `cpf`, `ct`, `cp`, `cf`, `cfp`, `cfp-form`, `cs-select`, `inspect-oikonomia`).
- Replaces the Pi tool `vak_evaluate` (`S4-4p-anima/extension/tools.ts`).
- Source: `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/vak-evaluate/SKILL.md`, blob `d582a158bdcaabe5342977110d99ad2a6bb0755e` (pinned and HEAD agree).
- Used by: `agent/anima` (owner), `agent/anima-psyche`, `agent/anima-logos`, `agent/anima-eros`, `agent/anima-mythos`. Reference grammar: `skill/ql/vak-coordinate-frame`.

## When

Before any dispatch, plan, test run, synthesis or Return is chosen. Evaluate only the missing layers when some are already set by the owner, the Factory workflow unit or the parent composition.

- Clear, bounded task: infer the block silently and state it in one line.
- Ambiguous scope, new domain, conflicting sources, or anything that changes the person's ground: work through the layers explicitly with the person.

## Procedure

1. **CPF — participation.** Is this dialogue or an admitted undertaking? If it needs the person to decide what it is for, it is `dialogical` `(00/00)`: stop here and hand it to `skill/personal/brainstorming`; that dialogue completes the remaining layers. If the owner has already stated or commissioned the act, it is `authorised-undertaking` `(4.0/1-4.4/5)` — name the authority (the owner's instruction, a Factory commission ref, a custody ref).
2. **CT — content type(s).** Which of CT0–CT5 (and CT4b′ for Day/NOW artifacts) the act makes or consumes. Several are normal.
3. **CP — position.** Where the act sits on 4.0–4.5. If a position must open its own frame, say so: that is a nested frame, not a bigger task.
4. **CF — frame and voice.** Choose the frame from `ql context-frame list`; the voice follows (`skill/ql/vak-coordinate-frame`). If the frame is CF1 `(00/00)`, do not dispatch an executor: send `agent/anima-nous` with minimal context, receive its questions and sources, then re-run this step.
5. **CFP — thread form.** CFP0 one voice; CFP1 different tasks in parallel; CFP2 chained phases with validation between them; CFP3 the same task to several readers fused by Agora; CFP4 sustained till its task list is done; CFP5 nested; Z for an authorised compose→perform→record→rehear→recompose cycle.
6. **CS — sequence and direction.** Pick CS0–CS5 for how much of the paired passage the act needs, and `forward-synthesis` (Day), `returning-inquiry` (Night′) or both.

When the act must be formally determined (an Expression, a composed performance, a Factory undertaking that will carry the block as its source stamp), write the steps as `ql.vak-composition/v1` JSON — `fixtures/kernel/vak-cprime-oikonomia-v1.json` is a complete executable template of `whole` → `enter` → `cfp-form` → `cs-select` → positioned hops — and run:

```bash
ql context-frame list --json
ql vak compose request.json --json
```

A refused step is the engine's answer. Keep the refusal; never hand-edit a result the engine did not produce.

## Outputs

```text
VAK: <task-short-name>
CPF: dialogical|authorised-undertaking (<authority ref>)  CT: CT<n>[,…]  CP: 4.<n>
CF: CF<n> (<expression>) -> <voice agent ref>  CFP: CFP<n>|Z  CS: CS<n> / forward-synthesis|returning-inquiry
```

plus, when composed, the engine's `results` for the named steps.

## Next

- CPF dialogical → `skill/personal/brainstorming`.
- CPF authorised-undertaking → `skill/ql/anima-orchestration` with the block.
- CS with both directions → `skill/ql/day-night-pass` after the forward pass.

## Authority and limits

Evaluation assigns coordinates; it grants no Action, Factory, Actuation or Workcell authority. A composed block is a formal reading, not permission to perform it. Do not claim a frame, thread form or passage the engine refused.
