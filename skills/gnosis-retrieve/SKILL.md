---
name: gnosis-retrieve
description: "METHOD: Gate knowledge retrieval for a workflow — scope first, retrieve through AIKit's provider-neutral knowledge field (search, resolve, read, relations, graph) and the Central wikis, keep every claim tied to its source, degrade honestly when a source is unavailable, and hand the grounded context to the caller without concluding for it. Use whenever Nous, Anansi, Moirai or any member needs recall that must not be invented."
---

# Gnosis retrieve

## Contract metadata

- Semantic ref: `ql:skill:gnosis-retrieve` (`skill/ql/gnosis-retrieve`)
- Native owners: AIKit `aikit knowledge search|resolve|open|read|relations|graph|sources|explain|status` and `aikit knowledge code search|context|impact|trace`; Central `ctrl --json action run central.wiki.read '{}'` (root wiki) and `projectcentral.wiki.read` (project wiki); `aikit wiki query` for a wiki file's semantic index.
- Replaces the Pi tools `aletheia_gnosis_query`, `aletheia_gnosis_status`, `hen_hybrid_retrieve` and `graph_query`. The Gnosis vector/graph store (3072-dim embeddings in a Neo4j `:Gnosis` namespace) is not carried; AIKit knowledge is the retrieval owner now.
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/gnosis-retrieve/SKILL.md`, blob `a8db8d49dd11440c4b542e4e77ea734865992ad7` (pinned and HEAD agree).
- Used by: `agent/anima-nous`, `agent/aletheia-anansi`, `agent/aletheia-moirai` (Lachesis mode), `agent/aletheia`, `agent/aletheia-agora`, `agent/aletheia-mercurius`.

## Procedure

1. **Scope before querying.** Name the question and the field: this repository, a Project, the root register, one wiki, a set of named documents. Narrow by coordinate when the question is coordinate-bound.
2. **Retrieve natively.**

```bash
aikit --json knowledge search "<query>" --limit 20
aikit --json knowledge resolve "<query>"          # rows with owner, provenance, available Actions
aikit --json knowledge read '<address from search>'
aikit --json knowledge relations '<address>' --depth 2
aikit --json knowledge code search "<symbol or path>"   # code lens with provenance
ctrl --json action run central.wiki.read '{}'
ctrl --json action run projectcentral.wiki.read '{"project":"<Name>"}'
```

   For a long document, read only the needed section (`skill/ql/darshana`). For the Bimba graph, use `skill/ql/bimba-cypher` under its own authority rules.
3. **Provenance is mandatory.** Every retrieved claim carries its source ref (address, path and revision, wiki node ref). A result with no provenance is discarded or re-queried — never paraphrased forward.
4. **Degrade honestly.** If a provider or index is unavailable (`aikit knowledge status` says so), say which and continue with what is grounded. Missing retrieval never blocks ordinary work and never becomes invented recall.
5. **Hand off, don't conclude.** Retrieval feeds the caller's reasoning. Deep recomposition over what was found belongs to Epii's owner-side Contemplate (`aikit flow contemplate`, owner-invoked), not to this step.

## Output

`grounded` (every claim cited) · `partial` (name the unbacked parts) · `empty` (nothing in this scope). Never `grounded` without provenance.

## Limits

Read-only. Wikis are agent-maintained knowledge, not source; never edit `wiki.json`. New knowledge travels back through `skill/ql/thought-distil` and NOW promotion.
