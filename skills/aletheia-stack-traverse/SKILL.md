---
name: aletheia-stack-traverse
description: "METHOD: Walk the actual stack a subject depends on — the six native products at S0–S5 (Central, Actuation, AIKit, Software Factory, Workcell, QL-MEF), the S′ organ that composes them, and the contracts and live code between — comparing intended role, current state and missing seams, and return a layer-by-layer health report with evidence. Use before an audit, a gate, a placement or an improvement proposal, and whenever a claim about what runs needs checking."
---

# Aletheia stack traverse

## Contract metadata

- Semantic ref: `ql:skill:aletheia-stack-traverse` (`skill/ql/aletheia-stack-traverse`)
- Native evidence: `ctrl capabilities --json`, `aikit capabilities`, `factory capabilities --json`, `actuation capabilities --json`, `ql capabilities --json` and each product's `system --json`; `aikit knowledge code search|context|impact|trace`; each repository's README, AGENTS, CONTRACT and capability matrix (`ProjectCentral/user/capability-matrix.json`); `skill/oi/capability-matrix` for reconciling matrix claims.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/aletheia-stack-traverse/SKILL.md`, blob `0cac8902ee7133d5079bafe6756ef904596dd968` (pinned and HEAD agree).
- Used by: `agent/aletheia` (owner), `agent/aletheia-janus`, `agent/aletheia-anansi`. Frame: CT4, CP 4.4.

## The stack

S0–S5 are Central, Actuation, AIKit, Software Factory, Workcell and QL-MEF. The S′ organs (Khora, Hen, Pleroma, Chronos, Anima, Aletheia) specialise their open capabilities for Epi; they do not reproduce their storage, parser, workflow engine, harness or desktop (`docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md` §1). Same-index S↔S′ is affinity, not exclusive ownership.

## Procedure

1. **Query the target first.** Resolve the subject (coordinate, module, Action, skill, Position) with `aikit --json knowledge resolve "<subject>"` and name the owning repository and revision.
2. **Read the contract and the live surface.** The owning repository's README/AGENTS/CONTRACT, its capability-matrix row, and the code it names (`aikit knowledge code context <symbol>`). Check the installed binary's own disclosure (`<product> capabilities --json`) — a documented command that the binary does not expose is a claim that outran its implementation.
3. **Compare** intended role, current state and missing seams per layer. Separate: authored intent, architecture contract, implementation fact, observed evidence, agent inference.
4. **Return** the report; route findings to `skill/ql/aletheia-module-audit` (attribution) or `skill/ql/aletheia-improvement-propose` (next move).

## Output

```text
STACK-TRAVERSE: <subject>
S<n> <product> @<revision>: intended … | current … | seam … | evidence <refs>
S′ <organ>: composes … | gap …
Verdict: healthy | drift | gap | unverified   (per layer)
```

## Limits

Read-only. Never infer a runtime capability from a document alone; say which layer was only read and which was executed.
