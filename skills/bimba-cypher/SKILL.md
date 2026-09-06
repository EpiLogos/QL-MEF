---
name: bimba-cypher
description: Develop and apply Cypher updates to the live Bimba graph safely — connection, conventions, idempotency discipline, the granular log law, retrieval patterns, and known quirks. Use when writing or reviewing any Cypher that modifies the Bimba Neo4j database, or when querying it deeply (coordinate walks, branch-downs, cluster retrieval).
---

# Bimba Cypher development

The Bimba graph is the living semantic authority the Map reflects. Cypher written against it is **authored intervention**: every statement must be idempotent, logged granularly, and verified by read-back. This skill is the operating discipline.

## 1. Connection

```bash
# interactive
docker exec -it epi-neo4j cypher-shell -a bolt://localhost:7687
# script (preferred for writes)
docker exec -i epi-neo4j cypher-shell -a bolt://localhost:7687 < cypher/<date>-<slug>.cypher
```

Auth is disabled (`NEO4J_AUTH=none`). If the container is down, start Docker Desktop (`open -a Docker`) and check `docker ps` for `epi-neo4j` — **do not** run the repo's compose files to revive it: an older compose created the running data-bearing container, and re-composing risks recreating/downgrading it.

## 2. The log law (non-negotiable)

Every applied write statement is recorded in [`cypher/CYPHER-LOG.md`](../../cypher/CYPHER-LOG.md) — one dated entry per batch, **one row per statement**, with: target (node coordinate or relation), idempotency mechanism, and observed result (created / pre-existing-verified / applied). Scripts live as dated `.cypher` files in `cypher/`; the log records what actually ran. No silent writes; no "quick fixes" outside the log.

## 3. Idempotency discipline

- **Node properties**: `SET` (naturally idempotent). Provenance fields use `coalesce()`: `SET n.c_1_former_name = coalesce(n.c_1_former_name, '<old value>')` so re-runs never overwrite provenance.
- **Relationships**: `MERGE` on the full (source, type, target) triple with `ON CREATE SET` (full provenance props) and `ON MATCH SET r.c_4_last_verified = '<date>'`. A pre-existing edge is a **find**, not a failure — stamp verification, never rewrite its original provenance.
- **Never**: `CREATE` for coordinate-bearing nodes or relations (bypasses MERGE dedup); `DETACH DELETE` / `DELETE` without an explicit owner decision recorded in the log; `SET` on properties you haven't read first.

## 4. Conventions

- **Addressing**: nodes by `coordinate` property (`MATCH (n {coordinate:'M2-4'})`). Branch-down: `n.coordinate STARTS WITH 'M2-4.'`. Regex: `n.coordinate =~ 'M2-4\\.[0-9]$'`.
- **Property prefixes** (observed in-graph): `c_0` essence/nature, `c_1` name/designation/description, `c_2` mathematical/architectural facts, `c_3` provenance (dataset branch, sources, `c_3_updated_at`), `c_4` access/QL category, `c_5` resonances/integrations; `q_0`–`q_5` quintessential readings; also `m_`, `t_`, `s_`, `l_`, `p_` family prefixes on some nodes. New properties follow the nearest existing prefix semantics. Renames keep `c_1_former_name`.
- **Relationship properties**: `c_0_source_coordinate`, `c_1_relation_description`, `c_2_relation_type` (+ `c_2_relation_ratio` where a ratio is the law), `c_3_created_at`, `c_3_dataset_branch` (e.g. `fold-in-2026-09-06`); verification stamp `c_4_last_verified`.
- **Timestamps**: ISO date strings (`'2026-09-06'`), matching existing `c_3_created_at` style.

## 5. Retrieval patterns (read-first, always)

1. **By coordinate**: `MATCH (n {coordinate:'M2-0'}) RETURN keys(n)` — read the property map before writing.
2. **Branch-down**: `MATCH (n) WHERE n.coordinate STARTS WITH 'M2-4.' RETURN n.coordinate, n.c_1_name ORDER BY n.coordinate`.
3. **Cluster retrieval**: relationship-type neighborhoods — `MATCH (a)-[r:CAUSAL_RESONANCE]-(b) WHERE a.coordinate = 'M2-0' RETURN …` — the graph's native cluster mode (M2-x nodes cluster to their L-family resonances this way).
4. **Relations of a seat**: `MATCH (a)-[r]->(b) WHERE a.coordinate IN […] OR b.coordinate IN […] RETURN …, keys(r)`.
5. **Text search**: only over known string fields — `coalesce(n.c_1_name,'') + coalesce(n.c_1_primary_designation,'') + coalesce(n.c_0_essence,'') + coalesce(n.c_1_description,'') =~ '(?i).*<term>.*'`.

## 6. Known quirks (learned the hard way)

- **Apostrophes**: Cypher escapes with **backslash** (`\'`), not SQL doubling (`''` — parse error). Prime coordinates are the common trap: `'L2-5\''`. File-based execution avoids shell mangling entirely.
- **Array-valued properties** break `toString(n[k])` in `any(k IN keys(n) …)` scans — search explicit string fields instead (pattern 5).
- **M2-4.4 does not exist — deliberately.** Branch asymmetry is law ("do not invent a generic M2-4.4 to complete a visual pattern"). Do not "fix" gaps without checking the matrices' asymmetry rulings.
- **L2-5′ is `Salt`** (renamed from `Mineral` 2026-09-06; register erratum). `c_1_former_name` holds the old value.
- Node `name` is `NULL` on coordinate nodes — names live in `c_1_name` / `c_1_primary_designation`.

## 7. Write protocol

1. Read-first: patterns 1 and 4 on every target.
2. Draft the script as `cypher/<date>-<slug>.cypher` — idempotent per §3, conventions per §4.
3. Execute file-based (§1).
4. Read-back verification of every node and relation touched.
5. Append the granular dated entry to `CYPHER-LOG.md` with actual results (created vs verified-stamped).
6. If the change should reach Map reflections or suite docs, flag it — the graph is authority; reflections follow through their own owners.
