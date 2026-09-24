---
name: anansi
description: "METHOD: Orient between blueprint and manifestation — hold what was intended (authored ground, locks, specifications) and what actually runs (code, installed commands, recorded evidence) together without confusing them, name the gap, say what comes next for a coordinate, and place a new learning in the right register with strict provenance. Use before structural assessment, after a crystallisation, and whenever a learning or gap needs a place on the coordinate map."
---

# Anansi

## Contract metadata

- Semantic ref: `ql:skill:anansi` (`skill/ql/anansi`)
- Native evidence: authored ground through `ctrl --json action run control.search '{"query":"<term>"}'`, `ctrl --json action run work.search '{"query":"<term>"}'`, `ctrl control open <target>` and ordinary reads of the repositories' locks and wayfinders; manifestation through `aikit --json knowledge search|resolve|relations`, `aikit knowledge code …`, each product's `capabilities --json`, capability matrices; the file map through `ctrl --json action run central.file-map.search '{"query":"<term>"}'` (read-only); section reads through `skill/ql/repl`.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/anansi/SKILL.md`, blob `913b82c307688c68bbc14eb46b149dfae0c5b8f5` (pinned and HEAD agree); agent `S5'/agents/anansi.md`; lineage law in `S4-5p-aletheia/modules/anansi-lineage.ts` (a trace without provenance is refused).
- Used by: `agent/aletheia-anansi` (owner), `agent/aletheia`; affinity Nous and Mythos. Frame: CF1 `(00/00)` orientation; CT0, CT3 when the gap itself is patterned; CP 4.0 with reach to 4.3 and 4.5.

## The two poles

| Pole | Then (source) | Now |
|---|---|---|
| Blueprint — intended structure | `/Idea/Empty/` | authored ground: Central `Control/user`, `ProjectCentral/user`, repository locks, wayfinders, specifications, capability-matrix intent |
| Present — manifested surface | `/Idea/Empty/Present/` | what runs: code at a revision, installed commands and Actions, recorded runs and evidence, capability-matrix implementation facts |

Hold both; do not let one stand in for the other.

## Invocations

- **`--orient "<learning>"`** — where does this belong? Name the coordinate, the register (root or Project), the owner and the standing.
- **`--gap "<S-layer or coordinate>"`** — contrast blueprint and present for that layer; list each gap with both refs.
- **`--next "<S-layer or coordinate>"`** — the smallest next move that closes the most load-bearing gap, and its owner.
- **`--place "<insight>"`** — after a crystallisation: does it enrich the current coordinate, or imply a promotion or a gap change? Hand promotions to `skill/ql/aletheia-self-extend` (coordinate mode) and `skill/ql/aletheia-collab-gate`.

## Provenance law

Every placement, gap and contrast carries resolvable refs to both poles (path and revision, Action id, wiki node, commit). A trace without provenance is refused, not softened into narrative.

## Output

```text
ANANSI --<mode>: <subject>
Blueprint: <refs>   Present: <refs>
Gap: <what differs>   Register: root|project:<Name>   Owner: <repo/source>
Next: <move> | Place: enrich <coordinate> | propose promotion (gate 6)
```

Anansi orients and names; it does not build or judge first, and it never writes to the map itself.
