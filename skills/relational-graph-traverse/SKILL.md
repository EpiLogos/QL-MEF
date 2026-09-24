---
name: relational-graph-traverse
description: "METHOD: Eros's relational scour over the graph — from a seed coordinate or knowledge address, walk the actual relations (knowledge relations, Vāk context, Bimba graph reads) and sort what is found into resonance, opposition and dissonance partners, each with the edge that shows it. Use when a task needs to know what a coordinate or subject is actually connected to before operating on it."
---

# Relational graph traverse

## Contract metadata

- Semantic ref: `ql:skill:relational-graph-traverse` (`skill/ql/relational-graph-traverse`)
- Native owners: AIKit `aikit --json knowledge relations <address> --depth <n>` and `aikit --json knowledge graph "<query>"`; QL-MEF `ql vak context <vak-ref> [0..2] --json` (source-locked, depth at most 2); the Bimba graph through `skill/ql/bimba-cypher` for read-only inspection under its own rules.
- Source: named and defined in one line by `Body/S/S4/ta-onta/S4-4p-anima/S4'/agents/eros.md` §5 (HEAD blob `455b64566aa637a093e7f0390c7cf1c0f3e37bbb`): "walks the bimba graph from a seed coord finding resonance / opposition / dissonance partners". No original SKILL.md body existed; this Method is the first body, bounded to that definition.
- Used by: `agent/anima-eros` (owner). Frame: CF3 `(0/1/2)`, CT2, CP 4.2.

## Procedure

1. **Seed.** Resolve the seed to a typed address (`aikit --json knowledge resolve "<seed>"`) or a Vāk ref (`ql vak locate <ref>`). Keep its revision.
2. **Walk** outward to depth 2 unless the task asks otherwise:

```bash
aikit --json knowledge relations '<address>' --depth 2 --max-nodes 256
ql vak context <vak-ref> 2 --json
```

3. **Sort each partner** by what the edge actually says:
   - **resonance** — same pattern, affirming relation, shared source;
   - **opposition** — complementary pair or declared contrast (for QL positions, the pair summing to 5);
   - **dissonance** — a relation whose two ends make incompatible claims (hand these to `skill/ql/cross-source-dissonance-detect` for the source-level check).
4. **Report** with the edge refs; do not add relations the graph does not hold.

## Output

```text
RELATIONAL-SCOUR: <seed ref>@<rev>
Resonance: <partner ref> via <edge/relation ref> …
Opposition: <partner ref> via … 
Dissonance: <partner ref> via … (→ cross-source-dissonance-detect)
Limits: depth <n>, nodes <n>, providers unavailable: <which>
```

Read-only. Graph writes are `skill/ql/bimba-cypher` under explicit authority.
