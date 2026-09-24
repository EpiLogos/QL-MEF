---
name: wikilink-resonance-scan
description: "METHOD: Scan the links between sources — wiki neighbours and backlinks, wikilinks inside Markdown sources, knowledge relations — for clusters where several sources point at the same subject, and return those resonance clusters with their members and links. Use when Eros needs to find which sources already speak to one another about a subject before relating or verifying it."
---

# Wikilink resonance scan

## Contract metadata

- Semantic ref: `ql:skill:wikilink-resonance-scan` (`skill/ql/wikilink-resonance-scan`)
- Native owners: AIKit `aikit --json wiki query neighbours|backlinks --file <wiki.json> <ref>` and `aikit --json wiki query search --file <wiki.json> "<text>"` (read-only over a wiki file); `aikit --json knowledge relations <address>`; `skill/ql/darshana` `threads` for the wikilinks inside one Markdown file.
- Source: named and defined in one line by `Body/S/S4/ta-onta/S4-4p-anima/S4'/agents/eros.md` §5 (HEAD blob `455b64566aa637a093e7f0390c7cf1c0f3e37bbb`): "scans vault wikilinks for cross-source resonance clusters". No original SKILL.md body existed; the vault is now the Central/Project wikis and the repositories' Markdown sources.
- Used by: `agent/anima-eros` (owner). Frame: CF3, CT0/CT2, CP 4.2.

## Procedure

1. **Choose the field.** Root wiki (`Control/agents/wiki/wiki.json`), a Project wiki (`Work/<Name>/ProjectCentral/agents/wiki/wiki.json`), or a named set of Markdown sources.
2. **Collect links.** For wiki refs: `neighbours` and `backlinks`. For Markdown: `python3 <darshana>/scripts/darshana.py threads <file>` per file. For knowledge addresses: `relations`.
3. **Cluster.** Group sources that link to the same target or to each other; a cluster needs at least two independent sources.
4. **Report** each cluster with its members and the links that bind them; mark one-way links.

## Output

```text
RESONANCE-SCAN: <field>
Cluster <n>: <target> ← <source refs> (links: …)
Isolated: <sources that link nowhere relevant>
```

Read-only. Wikis are agent-maintained knowledge; a new link travels back through NOW promotion, never by editing `wiki.json`.
