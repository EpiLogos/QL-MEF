---
name: aletheia-plugin-integrate
description: "METHOD: The integration membrane for a new capability, plugin, provider, skill pack or external surface entering the stack — first gather what already exists, then scope the candidate's owner and S-layer home, judge redundancy, gate coverage and architectural fit, and route any permanent adoption through the collaboration gate. Use before installing, vendoring or wiring anything new into the suite."
---

# Aletheia plugin integrate

## Contract metadata

- Semantic ref: `ql:skill:aletheia-plugin-integrate` (`skill/ql/aletheia-plugin-integrate`)
- Native evidence: `aikit --json knowledge resolve "<capability>"`, `aikit method list`, `aikit source show <id>` (read-only catalogue state), `aikit capabilities`; each product's `capabilities --json`; `actuation harness catalog|detect` for harnesses; `skill/personal/find-skills` for community skills.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/aletheia-plugin-integrate/SKILL.md`, blob `759b02c5b8e8fbbb9373ba5804ff05db12506b35` (pinned and HEAD agree).
- Used by: `agent/aletheia-agora` (owner of the gathering), `agent/aletheia`. Frame: CT4, CP 4.4.

## Procedure

1. **Gather first (Agora, CF6 retrieval).** What already exists natively for this need? Search the catalogue and the product capabilities before looking outward. Discovery precedes integration.
2. **Scope the home.** Which owner and S layer would carry it (Central, Actuation, AIKit, Software Factory, Workcell, QL-MEF)? Which S′ organ composes it? A candidate with no native owner is a proposal to an owner, not an install.
3. **Judge fit.** Redundancy with an existing capability; gate coverage (which Aletheia gate reviews its outputs); architectural fit (does it add a second registry, store, runtime or identity?); licence, provenance and privacy.
4. **Route adoption.** Installation into AIKit, a product dependency, or a vendored copy is a permanent change: `skill/ql/aletheia-collab-gate` first, then the owner's own release path. Reference-only patterns stay reference.

Historical candidate classes from the source (`mgrep`, `claude-mem`, `context7`, devops automation packs, marketplace patterns) are examples, not a queue.

## Output

```text
PLUGIN-INTEGRATE: <candidate>
Existing native coverage: <refs or none>
Home: S<n> <owner> / <organ>   Fit: redundant | complementary | conflicting
Gate: <gate>   Recommendation: adopt via <owner path> | reference only | decline
```
