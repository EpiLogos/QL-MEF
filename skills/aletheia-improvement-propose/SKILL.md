---
name: aletheia-improvement-propose
description: "METHOD: Turn traversal, audit, Klein telemetry or gate findings into one concrete, bounded next move — an implementation proposal, a plan or specification addendum, an owner-ready issue, or an explicit no-change recommendation when the gap is not yet justified — each carrying its evidence, owner and review gate. Use when a finding must become an actionable proposal rather than a note."
---

# Aletheia improvement propose

## Contract metadata

- Semantic ref: `ql:skill:aletheia-improvement-propose` (`skill/ql/aletheia-improvement-propose`)
- Native doors: `gh issue create` / `gh pr create` on the owning repository (the old "Linear-ready" statement is now an owner issue); `projectcentral.source.return` for a proposed change to a Project's source; `projectcentral.now.return` (`kind` `note` or `question`) to keep the proposal in the working field; `skill/ql/aletheia-collab-gate` for anything permanent.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/aletheia-improvement-propose/SKILL.md`, blob `21acaf98380077e59506342d305886885ea01385` (pinned and HEAD agree).
- Used by: `agent/aletheia`, `agent/aletheia-mercurius` (gate enrichment), `agent/aletheia-zeithoven`. Frame: CT4, CP 4.5.

## Procedure

1. Start from evidence: the traversal, audit, telemetry or gate result, with refs.
2. Pick exactly one form:
   - **implementation proposal** — the change, the files, the tests that prove it, the owner;
   - **plan or specification addendum** — the section it amends and why the current text no longer holds;
   - **owner issue** — problem statement, reproduction, expected versus actual, evidence, suggested owner;
   - **no change** — why the gap is not yet justified and what evidence would change that.
3. Name the review gate and whether the change is reversible.
4. File it at the owner (issue, PR or source return) or hold it in NOW; never apply a permanent change from this Method.

## Output

```text
IMPROVEMENT: <title>
Form: implementation | addendum | owner-issue | no-change
Owner: <repo or source>  Evidence: <refs>  Gate: <gate skill>  Reversible: yes|no
Filed: <issue/PR/source-return ref> | held in NOW (<return id>)
```
