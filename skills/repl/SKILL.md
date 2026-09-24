---
name: repl
description: Aletheia's name for the Darshana structural gaze — scout a source's topology, read one section, or list its wikilinks before retrieval, orientation or crystallisation, so disclosure works from the actual structure of a document rather than a paraphrase of it. Use inside Anansi orientation, gnosis retrieval and gate checks when a long source must be read surgically.
---

# REPL (Darshana) for Aletheia

## Contract metadata

- Semantic ref: `ql:skill:repl` (`skill/ql/repl`)
- Executable: the `scripts/darshana.py` bundled with `skill/ql/darshana` (the source kept one identical script, blob `f8e10acd705bf1597b7ce2e2671d25c13196653d`, in both skills). Load `skill/ql/darshana` with this skill; when both are projected together the script is at `../darshana/scripts/darshana.py`.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/repl/SKILL.md`, blob `29a53035f433da6b01d2acb0f9b1ce56f9634d70` (pinned and HEAD agree).
- Used by: `agent/aletheia-anansi`, `agent/aletheia`.
- Risk class: read-only.

## Use in Aletheia's work

| Act | How the gaze serves it |
|---|---|
| Orientation (`skill/ql/anansi`) | `scout` the blueprint and the manifested surface; compare their header trees and QL markers before naming a gap |
| Retrieval (`skill/ql/gnosis-retrieve`) | after `aikit knowledge search` returns a document, `read --section` only the part that answers the question; cite section and line |
| Gates | `threads` to check that every coordinate reference is a resolvable wikilink or source ref |
| Crystallisation (`skill/ql/thought-distil`) | read the exact source passages a learning rests on before distilling it |

## Commands

```bash
python3 ../darshana/scripts/darshana.py scout "/abs/path/file.md"
python3 ../darshana/scripts/darshana.py read "/abs/path/file.md" --section "<header substring>"
python3 ../darshana/scripts/darshana.py read "/abs/path/file.md" --ql "P4'"
python3 ../darshana/scripts/darshana.py threads "/abs/path/file.md"
```

If `skill/ql/darshana` is not projected in this body, say so and fall back to `aikit knowledge read <address>` plus ordinary file reads; do not paraphrase a section you did not read.
