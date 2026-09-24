---
name: aletheia-module-audit
description: "METHOD: Attribute a failure or misalignment to the layer, module and owner that actually own it — which contract was violated, whether it is architecture, implementation or integration debt, and which gate should review the correction — so the fix lands at its native owner instead of being patched where it surfaced. Use after a stack traversal, an incident, a failing gate or a red test whose cause is not yet placed."
---

# Aletheia module audit

## Contract metadata

- Semantic ref: `ql:skill:aletheia-module-audit` (`skill/ql/aletheia-module-audit`)
- Native evidence: the failing command's exact output; `git log`/`git blame` on the owning repository; `gh issue list|view` and `gh pr list|view` on the owner; `aikit knowledge code impact|trace`; `aikit failures`; `factory workflow inspect` for commissioned runs; `actuation stream replay` for recorded agent activity.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/aletheia-module-audit/SKILL.md`, blob `fe27e7af5181a008954a2ea1b98b4d013b5bd6a1` (pinned and HEAD agree).
- Used by: `agent/aletheia` (owner). Frame: CT4, CP 4.4.

## Procedure

1. **Preserve the failure.** Keep the exact command, input, output and revision before touching anything.
2. **Locate the origin.** Follow the call from where it surfaced to where the wrong decision was made (`aikit knowledge code trace <from> <to>`). The owner is the repository whose contract the wrong behaviour violates, not the one where it was noticed.
3. **Name the contract.** Quote the violated line (README, CONTRACT, schema, capability-matrix row, test) with its path and revision.
4. **Classify.** Architecture debt (the contract itself is wrong or missing), implementation debt (code departs from a sound contract), integration debt (two sound sides disagree at a seam).
5. **Choose the gate** that must review the correction: coordinates → `aletheia-ql-gate`; meaning → `aletheia-m-gate`; layer placement → `aletheia-s-gate`; instrument surface → `aletheia-m-prime-gate`; archetypal form → `aletheia-rupa-gate`; anything permanent or paradigmatic → `aletheia-collab-gate`.
6. **Close or carry.** A small fix inside the current commission is repaired at its owner with a regression test and the original activity replayed. A fix that needs another owner's hand goes to that owner as an issue or PR with the proof. A decision only the person can make goes to Recognition.

## Output

```text
MODULE-AUDIT: <symptom>
Origin: <repo>@<rev> <path:line>   Contract: <quoted rule + ref>
Class: architecture | implementation | integration   Gate: <gate skill>
Action: repaired (<PR>) | handed to owner (<issue/PR>) | for Recognition (<question>)
```
