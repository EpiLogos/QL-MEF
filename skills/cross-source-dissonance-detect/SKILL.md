---
name: cross-source-dissonance-detect
description: "METHOD: Flag contradictions and tensions across the sources tied to one coordinate or subject — where two sources, a document and the code, or a claim and its evidence disagree — with both sides quoted, their standing, and which one currently governs. Use before operating on a subject whose sources may have drifted, and whenever a relational scour returns dissonance."
---

# Cross-source dissonance detect

## Contract metadata

- Semantic ref: `ql:skill:cross-source-dissonance-detect` (`skill/ql/cross-source-dissonance-detect`)
- Native owners: AIKit `aikit --json knowledge search|read|sources|explain` and `aikit knowledge code context`; section reads through `skill/ql/darshana`; the installed products' `capabilities --json` for implementation facts.
- Source: named and defined in one line by `Body/S/S4/ta-onta/S4-4p-anima/S4'/agents/eros.md` §5 (HEAD blob `455b64566aa637a093e7f0390c7cf1c0f3e37bbb`): "flags contradictions or tensions across sources tied to a target coord". No original SKILL.md body existed.
- Used by: `agent/anima-eros` (owner); results feed `skill/ql/aletheia-module-audit`. Frame: CF3, CT2/CT3, CP 4.2.

## Procedure

1. **Gather the sources** tied to the target: authored ground, specifications and locks, code at its revision, capability-matrix rows, wiki nodes, recorded evidence.
2. **Read the claims**, not summaries — the exact section or line.
3. **Compare** pairwise. A dissonance is a real incompatibility (the document says JSON or YAML, the parser accepts only JSON; a lock forbids what a skill instructs), not a difference in wording.
4. **Attach standing** to each side: authored position, design commitment, architecture contract, implementation fact, observed evidence, agent inference. Say which one currently governs and why.
5. **Route**: a stale document or a claim that outran its implementation goes to its owner (`skill/ql/aletheia-module-audit`); a genuine paradox the owner must hold stays open as a `T2` Challenge thought.

## Output

```text
DISSONANCE: <target>
<source A ref> says … (standing) | <source B ref> says … (standing)
Governs: A|B|undecided — because …   Route: owner <repo> | open question
```

Read-only.
