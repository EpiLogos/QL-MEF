---
name: aletheia-self-extend
description: "METHOD: Propose a bounded extension of the system itself — in tool and skill space (a new skill, Method, command or membrane, with owner, layer, validation gate and rollback) or in coordinate space (an enrichment or promotion of the coordinate map after the insight has been placed) — always ending at the human collaboration gate. Use when Night′ work or an audit shows the system needs a new capacity or a changed map, never to apply one."
---

# Aletheia self-extend

## Contract metadata

- Semantic ref: `ql:skill:aletheia-self-extend` (`skill/ql/aletheia-self-extend`)
- Native doors: skills and Methods are authored in their owning repository and released through AIKit (`skill/personal/writing-skills`, `skill/personal/central-skill-release`; AIKit catalogues a source only after `aikit source sync` and `promote` by its owner); commands and Actions are proposed to their product repository (`gh issue create`/`gh pr create`); coordinate-map changes are proposed to their canonical owner (QL-MEF canon, the Bimba source) as a reviewed PR.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/aletheia-self-extend/SKILL.md`, blob `448199c70da2d558383dadcddf6ae203ee275ddd` (pinned and HEAD agree).
- Used by: `agent/aletheia`, `agent/aletheia-zeithoven` (owner of manifestation), `agent/aletheia-anansi` (placement support). Frame: CT5, CP 4.5.

## Mode: tools

Propose a new tool, skill, Method or tool–skill membrane. Include:

- owner repository and the native capability it would extend (never a second registry);
- affected S layer and S′ organ;
- the validation gate and the test or check that proves it;
- rollback — how it is disabled or reverted;
- whether an existing native skill or command already covers it (`aikit method list`, `aikit --json knowledge resolve "<capability>"`).

## Mode: coordinate

Propose a coordinate-map change only after:

1. the Night′ insight is placed by `skill/ql/anansi` (`--place`);
2. current versus gap register is identified;
3. the exact coordinate to enrich or promote is named, with its source and revision;
4. `skill/ql/aletheia-collab-gate` is requested before any write.

## Output

```text
SELF-EXTEND (<tools|coordinate>): <title>
Owner: <repo>  Layer: S<n>/<organ>  Evidence: <refs>
Proposal: <PR/issue ref or draft>  Gate: aletheia-collab-gate (pending)  Rollback: <how>
```

Both modes end at the collaboration gate. Nothing here is applied, merged or promoted by the Method itself.
