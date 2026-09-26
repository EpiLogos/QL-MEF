---
name: anima-expressive-composition
description: "METHOD: Choose, adapt and perform the Expression material for an act — find an existing Scene or character through material_list, bind its roles to the actual cast, subject and text, perform it through the O:I act operations (act_open, act_select, act_text, act_gesture), and when no suitable form exists create or fork one, set its reuse block and save it into the root material register so the next invocation finds it. Use whenever Anima (or a member it dispatches) must show, explain or conduct work in the O:I Expressions medium: a Factory Run's live view, an explanation, a skill invocation, a handoff."
---

# Anima expressive composition

## Contract metadata

- Semantic ref: `ql:skill:anima-expressive-composition` (`skill/ql/anima-expressive-composition`)
- Native owner of every operation: the O:I desktop kernel, through the existing `oi desktop expression [SOCKET] REQUEST_JSON` seam. A body whose `schema` is `oi.expression-world/v1` is an Expression-world request (`act_*`, `material_list`); a body without it is an ordinary Expression request (`open_file`, `fork`, `edit`, `save_as`, `save`, `inspect`). Discovery of the implemented operations: `oi desktop expression capabilities` and `oi desktop expression '{"schema":"oi.expression-world/v1","operation":"capabilities"}'`.
- Material register: `Work/O-I/desktop/cradle/material/expressive-material/<kind>/<slug>.expression.json` (O:I `MATERIAL_REGISTER`). `material_list` answers `{materials, folders, unreadable, truncated}`; `folders[kind]` is the owner-disclosed `parent` for `save_as` into the register.
- Contract: O:I `docs/contracts/EXPRESSION-ACT-MATERIAL-V1.md` §1 (reuse block), §3 (bindings), §4 (act and operations). The kernel types in `desktop/cradle/kernel/src/expression_world.rs` are the contract of record; requests refuse unknown fields.
- Source: O:I `docs/cradle/handovers/factory-expressions-2026-09-26/EXPRESSION-DEVELOPMENT-SPEC.md` §1, §4, §6, §7 (Anima row), §9 items 3, 4 and 7. No C-Experiments skill body preceded this one; it is new practice for the S4′ Anima organ.
- Used by: `agent/anima` (owner, CF5 conduct), `agent/anima-eros` (CF3 exchange with the Expression powers), `agent/anima-mythos` (CF4 organising image); dispatch per `skill/ql/anima-orchestration`. Continuation and replay are `skill/ql/chronos-act-continuity`; Return and curation are `skill/ql/aletheia-expressive-return`.

## The law of reuse

1. **Choose premade material first.** Always run `material_list` before composing anything.
2. **Create only when missing.** A new or adapted form is justified only when no listed material carries the roles and meaning the act needs; say which listed candidates were rejected and why.
3. **Always make it reusable.** Anything created or adapted is saved with a `reuse` block (kind, roles, associations, `variation_of`, `authored_by`) into the root register, so `material_list` finds it next time. An unsaved one-off composition is unfinished work.

A Scene change is a state change; an Expression is a set of possible state changes. The constellation is who and what is present (the cast and objects); the composition is how they are organised in the Scene and why. This is art: choose the form that says what the work is doing, not the nearest file.

## Inputs

The act's mode (`factory`, `expressions` or `techne`); the target Expression ref (live, open in the desktop); the cast — each participant's `agent:` ref, `agent-profile:` ref and profile `expressive_character_ref`; the subject or goal ref; the instrument ref (Factory run/attempt, Technè constellation) where there is one; the explanation or event to be shown; the associations that describe this work (workflow key, task type, SkillSet ref, skill ref, event family).

## 1. Open the act

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_open",
  "act_ref":"act:explain-handoff-01","mode":"expressions","expression_ref":"expression:live-run",
  "cast":[{"role":"lead","participant_ref":"agent:anima","profile_ref":"agent-profile:anima","character_ref":"central:…/character/anima.expression.json"},
          {"role":"goal","participant_ref":"goal:…"}],
  "subject_ref":"<goal or subject ref>","actor":"agent:anima"}'
```

`act_open` on an existing `act_ref` resumes that act with its cast, material and selection; open a new act only for a new undertaking.

## 2. Choose existing material (association first, then kind)

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"material_list",
  "kind":"scene","association":{"workflow_keys":["expression-development"]}}'
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"material_list","kind":"scene"}'
```

Each entry is `{file_ref, revision, title, kind, roles, states, gestures, associations}`. Resolution order (contract §5): an explicitly named form → the workflow-associated Expression → task-type/SkillSet/skill-associated material → any material of the right kind whose `roles` fit. The curated starter set in the register:

| Register path | Use |
|---|---|
| `character/anima.expression.json`, `character/aletheia.expression.json` | team characters: states `idle`, `working`, `speaking`; gesture `invoke-skill` |
| `scene/arrival.expression.json` | an agent enters and takes its active state |
| `scene/work-passage.expression.json` | work moves an object from A to B |
| `gesture/skill-invocation.expression.json` | a skill is invoked while the Scene continues |
| `scene/handoff.expression.json` | roles `sender`, `recipient`, `artifact`, `caption` |
| `scene/review.expression.json`, `scene/completion.expression.json` | review outcome; completion with result objects |
| `scene/explanation.expression.json` | roles `lead`, `goal`, `caption`, `progressText` |
| `expression/expression-development.expression.json` | workflow Expression for `expression-development`; playback arrival → work-passage → skill-invocation → handoff → review → completion |

## 3. Bind and perform

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_select",
  "act_ref":"act:explain-handoff-01","material":{"file_ref":"<file_ref from material_list>","scene_ref":"<scene ref>"},
  "bindings":{"lead":{"kind":"agent","agent_ref":"agent:anima","profile_ref":"agent-profile:anima","character_ref":"central:…","state":"speaking","label":"Anima"},
              "goal":{"kind":"object","subject_ref":"<goal ref>","label":"Goal"},
              "caption":{"kind":"text","text":"Why the draft goes to Logos next"}},
  "transition":{"duration":1.2,"easing":"easeInOut"},"actor":"agent:anima"}'
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_text",
  "act_ref":"act:explain-handoff-01","role":"progressText","value":0.4,"text":"Specification 2 of 5","actor":"agent:anima"}'
```

For a character, change its state object-locally instead of changing the Scene: `"role":"lead","state":"working","material":{"file_ref":"<the character's file_ref>"}` and no `scene_ref` — the role's occupant takes that state's `self` material while the current Scene continues. A `material.scene_ref` is a Scene change. Bind every role the material exposes; an unbound role keeps its authored placeholder, which is acceptable only when that is the intent. `act_select` appends a passage to the act's sequence and performs through the kernel's `scene_material_set`; the engine carries the physics and transition.

## 4. Gestures for skill invocations

When a member invokes a skill, tool or capability, perform the object-local gesture while the current Scene continues:

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_gesture",
  "act_ref":"act:explain-handoff-01","role":"lead","gesture":"invoke-skill",
  "event_basis":{"family":"skill-invocation","source":"aikit-encounter","event_ref":"<encounter event>","occurrence":"2"},
  "actor":"agent:anima"}'
```

Use the gesture named in the bound character's `gestures`, or the skill's associated gesture material (`material_list` with `association.skill_refs`). Each separate invocation is its own occurrence. Record the functional operation itself with `act_operate` (`mode` + its native ref: the Factory task, skill or message ref), so the expressive passage and the operation stay tied.

## 5. Create or adapt when nothing fits

Adapt (a variation) when a listed form is close; create only when none is. For a variation, open the source through its Central location and fork it so the original edition is untouched:

```sh
ctrl --json action run central.files.resolve '{"ref":"<file_ref>"}'      # the location of the source form
oi desktop expression '{"operation":"open_file","location":<location>,"actor":"agent:anima"}'
oi desktop expression '{"operation":"fork","expression_ref":"<opened ref>","expected_revision":<n>,"new_expression_ref":"expression:explanation-two-voices","actor":"agent:anima"}'
oi desktop expression '{"operation":"edit","expression_ref":"expression:explanation-two-voices","expected_revision":<n>,"actor":"agent:anima",
  "changes":[ …Scene/entity/text changes through the ordinary change vocabulary…,
    {"change":"reuse_set","reuse":{"schema":"oi.expression-reuse/v1","kind":"scene","title":"Explanation, two voices",
      "roles":[{"role":"lead","accepts":"agent","entity_ref":"entity:…"},{"role":"participants.0","accepts":"agent","entity_ref":"entity:…"},
               {"role":"goal","accepts":"object","entity_ref":"entity:…"},{"role":"caption","accepts":"text","text_id":"t-caption"}],
      "entry_scene_ref":"scene:…",
      "associations":{"workflow_keys":["expression-development"],"task_types":["explanation"],"skill_set_refs":[],"skill_refs":["skill/ql/anima-expressive-composition"],"event_families":[]},
      "variation_of":{"file_ref":"<source file_ref>","revision":"<source revision>"},
      "authored_by":"agent:anima"}}]}'
oi desktop expression '{"operation":"save_as","expression_ref":"expression:explanation-two-voices","expected_revision":<n>,
  "parent":<folders.scene from material_list>,"name":"explanation-two-voices.expression.json",
  "operation_ref":"op:…","actor":"agent:anima","actor_kind":"agent"}'
```

A role slot is an entity in `presentation.scene.entities[]` or a text layer in `presentation.scene.text[]` carrying `"role"`; `roles[]` indexes those slots and must match them. `variation_of` names the exact source revision; a new form omits it. `authored_by` names the agent (or person) whose act made it. Later changes to a saved form use `save` with its `expected_file_revision`, never a second copy.

Then perform the new form in this act (`act_select` with the new `file_ref`). **On a later invocation, begin again at step 2**: `material_list` must now return the saved variation, and it is selected like any other form. If it does not appear, the save or its associations are wrong; repair them before composing anything new.

## Output

```text
EXPRESSIVE-COMPOSITION: <act_ref> (<mode>)
Target: <expression_ref>  Cast: <role=agent …>  Subject: <ref>
Chosen: <file_ref>@<revision> <scene_ref|state>  Rejected: <file_ref: reason …>
Created: none | <file_ref>@<revision> variation_of <file_ref>@<revision>  Associations: <…>
Passages: <index kind …>  Gestures: <role gesture occurrence …>
```

## Authority and limits

The Method acts only on a draft or live target Expression the act was opened on, and saves only into the material register. It never replaces or publishes an original edition, and never writes material around `save_as`/`save` (no file copies). A kernel refusal (`deny_unknown_fields`, revision conflict, missing operation) is reported with its typed error and the operation is corrected against `capabilities`; no side route is invented. Performed presentation is observed in the running desktop, not inferred from the returned request.
