---
name: aletheia-expressive-return
description: "METHOD: Carry the knowledge and Return side of an Expression act — develop the explanatory material the act shows, conduct Technè work on the same act (act_continue into techne and back), evaluate what was performed against the intent, complete the act with its Return (act_complete), and curate the forms that worked into reusable material with associations to their workflow, task type, SkillSet or skill and their revision and event basis. Use when Aletheia is called on an act that Anima conducts, when an explanation needs its content developed, or when a performed form should become repertoire."
---

# Aletheia expressive Return

## Contract metadata

- Semantic ref: `ql:skill:aletheia-expressive-return` (`skill/ql/aletheia-expressive-return`)
- Native owners: the O:I desktop kernel through `oi desktop expression [SOCKET] REQUEST_JSON` (world requests carry `"schema":"oi.expression-world/v1"`: `act_inspect`, `act_text`, `act_operate`, `act_continue`, `act_complete`, `material_list`; ordinary Expression requests: `open_file`, `fork`, `edit` with `reuse_set`, `save_as`, `save`); Technè instruments through `ql techne reading <target.json> --json` and the instrument's published Actions; knowledge through `aikit --json knowledge search|resolve|read`; the Return door through `ctrl --json action run projectcentral.now.return` (Project register) or the root NOW Actions; the material register through `central.files.*` at `Control/agents/expressive-material/<kind>/<slug>.expression.json`.
- Contract: O:I `docs/contracts/EXPRESSION-ACT-MATERIAL-V1.md` §1, §4, §5.
- Source: O:I `docs/cradle/handovers/factory-expressions-2026-09-26/EXPRESSION-DEVELOPMENT-SPEC.md` §5 (repertoire association, timeline), §6, §7 (Aletheia row), §9 items 6 and 8. New practice for the S5′ Aletheia organ; the gate vocabulary is `skill/ql/aletheia-m-prime-gate` and `skill/ql/aletheia-rupa-gate`.
- Used by: `agent/aletheia` (owner: Return and curation), `agent/aletheia-moirai` (distils the explanatory content), `agent/aletheia-agora` (fuses plural readings of a performance), `agent/aletheia-zeithoven` (hands a successful form forward as repertoire). Anima's side of the act is `skill/ql/anima-expressive-composition`; reading and replaying passages is `skill/ql/chronos-act-continuity`.

## Inputs

The `act_ref` and its target Expression; the intent the act was opened for (the unit's `requiredDifference` and `returnContract`, or the explanation's question); the Return address (a NOW field); for Technè, the constellation ref and the target file `ql techne reading` accepts.

## 1. Read the act before speaking about it

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_inspect","act_ref":"<act_ref>"}'
```

Take the cast, subject, instrument, material (`file_ref`, `revision`, `scene_ref`), bindings and every passage with its `event_basis`. Disclosure begins from this record, not from memory of what was intended.

## 2. Develop the explanatory material

An explanation's text roles (`caption`, `progressText`, `resultText`) carry knowledge, not decoration. Retrieve and distil the content with its sources (`skill/ql/gnosis-retrieve`, `skill/ql/thought-distil`), then fill the roles on the running act:

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_text",
  "act_ref":"<act_ref>","role":"caption","text":"<distilled sentence>","actor":"agent:aletheia"}'
```

Each filled text traces to a cited source or to the act's own recorded evidence.

## 3. Conduct Technè work on the same act

Technè is Aletheia_i situated in M_i′, operating the instrument; it continues the act rather than starting another. Carry the act into Technè with its cast, subject and selection:

```sh
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_continue",
  "act_ref":"<act_ref>","to":"techne","instrument_ref":"<constellation ref>","actor":"agent:aletheia"}'
ql techne reading <target.json> --json
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_operate",
  "act_ref":"<act_ref>","mode":"techne","native_ref":"<reading or instrument Action ref>","summary":"<what the instrument did>","actor":"agent:aletheia"}'
```

Record every instrument operation with `act_operate` and its native ref. When the Technè work has a result, continue back to the mode the act came from (`"to":"factory"` for a Run, `"to":"expressions"` for composition work) so the result arrives in the same act and the working view and selection are restored there. Canonical changes to the constellation still pass `skill/ql/aletheia-collab-gate`.

## 4. Evaluate

Compare intent against performance: which passages were performed, whether the bound roles showed the actual participants and objects, whether the text is true to the evidence, whether the form served what the work was doing. Use `skill/ql/aletheia-m-prime-gate` for the Expression artifact and `skill/ql/aletheia-rupa-gate` for the rendered form; results use their `aligned | annotated | hold | redirect` vocabulary. Observed presentation is evidence only when it was seen in the running desktop.

## 5. Return and complete

Write the Return through the register's door first, then complete the act with its ref:

```sh
ctrl --json action run projectcentral.now.return '{"project":"<Name>","actor":"agent:aletheia","kind":"handoff","subject":"<act_ref>","result":"<intent vs performance, evidence refs, material file_ref@revision>","status":"active"}'
oi desktop expression '{"schema":"oi.expression-world/v1","operation":"act_complete",
  "act_ref":"<act_ref>","return_ref":"<ref returned by the NOW Action>","result_text":"<one-paragraph result>","actor":"agent:aletheia"}'
```

`act_complete` appends the `return` passage. Completion with unfinished work is not a completion: continue the act instead (`skill/ql/chronos-act-continuity`, incomplete branch).

## 6. Curate reusable material

A form that worked becomes repertoire. For each such form — a Scene the act composed, a character state, a gesture, or the act's whole sequence as an Expression:

1. Check it is not already in the register: `material_list` by kind and association. Improve an existing form with `save` (its `expected_file_revision`) rather than adding a near-duplicate.
2. Otherwise `edit` the draft with `{"change":"reuse_set","reuse":{…}}`: `kind`; `roles` matching the role slots; `associations` naming the actual `workflow_keys`, `task_types`, `skill_set_refs`, `skill_refs` and `event_families` it served; `variation_of` with the exact source `file_ref` and `revision` when it was adapted; `authored_by` the agent or person whose act made it. For a whole Expression, set `playback` to the Scene order as performed and `entry_scene_ref`.
3. `save_as` into `Control/agents/expressive-material/<kind>/<slug>.expression.json` (parent location from `ctrl --json action run central.files.list '{"path":"Control/agents/expressive-material/<kind>"}'`).
4. Put the saved `file_ref@revision`, the originating `act_ref` and the passages' `event_basis` (family, source, event ref, occurrence) into the Return, so the Run's history can reopen the sequence exactly.

A form that failed is not saved; its gap goes into the Return with the evidence.

## Output

```text
EXPRESSIVE-RETURN: <act_ref>
Knowledge: <roles filled → sources>   Technè: <continuations and act_operate refs | none>
Evaluation: M′ <aligned|annotated|hold|redirect>  Rupa <…>  <one line>
Return: <return_ref>   Completed: yes | continued (reason)
Curated: <file_ref@revision kind associations variation_of> | none (why)
```

## Authority and limits

Aletheia returns and curates; it does not recognise. The Return is a proposal to the owner; promotion to wiki or human ground is `skill/personal/central-knowledge-promotion`, never this Method. Material is saved only through the Expression `save_as`/`save` path into the register; no file is copied around it. An original edition is never overwritten.
