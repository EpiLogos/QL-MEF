---
name: darshana
description: The structural gaze — scout a large Markdown source's topology (frontmatter, header tree, P0–P5 and P0′–P5′ markers, dialogue turns), read one section by header or QL marker, or list its wikilinks, without loading the whole file into context. Use before retrieving, citing or placing a long document, and for surgical section reads during a task.
---

# Darshana — the structural gaze

## Contract metadata

- Semantic ref: `ql:skill:darshana` (`skill/ql/darshana`)
- Executable: `scripts/darshana.py` in this skill (Python 3 standard library, read-only). Carried verbatim from the source (`darshana.py`, blob `f8e10acd705bf1597b7ce2e2671d25c13196653d`, identical in the Anima and Aletheia copies).
- Source: `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/darshana/SKILL.md`, blob `682995f568a4b131f87ec3bf0ca76d922ff94ec1` (pinned and HEAD agree). Aletheia uses the same gaze under the name `skill/ql/repl`.
- Used by: `agent/anima-nous`, `agent/anima-mythos` (affinity), `agent/anima`, and through `repl` `agent/aletheia-anansi` and `agent/aletheia`.
- Risk class: read-only.

## Commands

Run from this skill's directory (or give the script's absolute path). Paths are absolute.

```bash
# The map: frontmatter, header tree, QL markers, dialogue turns
python3 scripts/darshana.py scout "/abs/path/file.md"

# The lens: one section by header substring, or by QL marker in a header
python3 scripts/darshana.py read "/abs/path/file.md" --section "2.1 The Concept"
python3 scripts/darshana.py read "/abs/path/file.md" --ql "P1"

# The weave: every [[wikilink]] in the file
python3 scripts/darshana.py threads "/abs/path/file.md"
```

`read` without `--section`/`--ql` returns the first 50 lines. A section ends at the next header of the same or higher level.

## Where it fits

- **Before knowledge work:** scout a document, then read only the sections a question needs; cite the section and line.
- **Before placing or returning material:** use the skeleton's QL markers and wikilinks to decide where a learning belongs (`skill/ql/anansi`), instead of paraphrasing the whole file.
- **Whole-document and cross-source reads** belong to AIKit: `aikit knowledge search "<query>" --json`, `aikit knowledge read <address>`, `aikit knowledge relations <address>`. Use darshana for the inside of one file, AIKit knowledge for the field.

## QL detection

`P0`–`P5` and `P0′`–`P5′` (written `P0'`) in frontmatter values, headers, and list lines of the form `- P2: …` or `(#3) …`.

## Limits

Naive YAML parsing (key: value lines only). Markdown only. It never writes.
