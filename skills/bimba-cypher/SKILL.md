---
name: bimba-cypher
description: "METHOD: Inspect or apply an explicitly authorised Bimba graph change after verifying the actual World, database, source revision and writer contract. Use for compound-coordinate traversal, typed properties and qualified relation evidence; preserve idempotence, granular logs and read-back."
---

# Bimba source and Cypher practice

## Contract metadata

- Native source owner: EpiLogos/QL-MEF; actual graph/provider and authority must be discovered for this World.
- Governing source: `docs/kernel-rebuild/PRE-K8-AGENT-WORLD-LOCK.md`, `VAK-OIKONOMIA-KNOWLEDGE-RETURN.md` §5, and current M ledger/matrix rules.
- Existing change record: `cypher/CYPHER-LOG.md` with dated scripts in `cypher/`.
- This Method does not supply a database, credentials, permission or a missing typed assertion writer.

## Inputs and activation

Use for a scoped source/graph investigation or a reviewed maintenance change. Establish the native World/Project, actual database/provider identity and version, exact subject/coordinates, source/registry revision, desired effect, applicable property/relation schema, expected current values and explicit authority. Read the targets and relevant relations before writing.

## Authority and material preflight

Discover the actual current graph connection through the native owner. A historical `epi-neo4j` Docker container and disabled-auth setting are not universal installation facts. Never disable authentication, start/recreate a data-bearing stack, guess credentials or run an old compose file to satisfy this Method. An unavailable connection blocks the live graph operation, not source-only investigation.

For local maintenance explicitly authorised by the owner, use that verified installation's file-based Cypher route. Record database identity and the approved execution boundary privately; do not publish credentials. For normal Epii/application operations use the supported native graph/Action interface. A raw Cypher route is not a substitute for missing application authority.

Structural identity/property/coordinate changes require the existing ledger decision and K8.0 shared promotion path. Domain assertions, private episodes and structural canon retain their different standing. Human source changes use the relevant Recognition; this Skill cannot enforce it by wording alone.

## Procedure

1. Resolve the actual coordinates through the current registry/source map, preserving compound `-`, `.`, `/` and prime identities. For a recursive branch use actual parent/child relations or an enumerated registry subtree. A dot-prefix query is valid only for a source branch whose grammar proves it; do not invent absent children for symmetry.
2. Read the complete node and relationship properties, current assertions, source and revision for every proposed target. Exact-coordinate reads may use the verified graph's `MATCH (n {coordinate: $coordinate}) RETURN n` with bound parameters. Text search uses known typed string fields; do not coerce every property indiscriminately.
3. Obtain the actual property definitions, value/domain/cardinality/applicability and assertion contract. The approved local form is #0 coordinate, #1 definition, #2 rich assertion, #3 compact KV, #4 named quintessential reading, #5 graph-derived result. Retain producer and graph/source revision; never derive a canonical q_* meaning from its prefix alone.
4. Identify each qualified relation assertion by the current writer's real identity and support. Endpoint/type equality alone is not permission to collapse independent source evidence. A simple `MERGE (source)-[:TYPE]->(target)` is only suitable where the native relation contract explicitly defines one such edge. For plural assertions preserve each evidence/qualification and its revision. If the writer cannot do so, stop the write and return the AW1 dependency; do not invent an `assertion_id` property and call it supported.
5. Draft the exact idempotent transaction in `cypher/<date>-<slug>.cypher`, with read/expected-revision preconditions, target list, proposed changes, provenance and reversible disposition. Validate through the actual owner and applicable native checks. A reviewed structural patch stays with K8.0 until the coordinated projection is ready.
6. Apply only the authorised transaction. Preserve old source/provenance; qualify a pre-existing assertion as verified rather than overwrite its original evidence. Deletion or destructive migration needs its own explicit owner decision and rollback basis.
7. Read back every target and independently qualified relation through the owner. Check exact changed/unchanged state, revision and source. On uncertain execution inspect before any retry. Idempotence is tested against the real selected writer, not assumed from the word MERGE.
8. Append one dated batch to `CYPHER-LOG.md`, one row per executed statement: target/assertion, expected basis, idempotency mechanism, actual result (created, pre-existing-verified, applied, failed/uncertain), read-back evidence and rollback/continuation. A planned statement is not logged as applied.

## Outputs

Return the selected World/database/registry/source basis, precise target identities, schema/authority decision, script and observed application/read-back, remaining discrepancies and next owner. Regenerate downstream projections only through their existing owners. Use existing M-ledger/matrix checks for structural implications; passing them is not live database application or human acceptance.

## Verification

Verify every changed subject by native read-back and the recorded source/revision precondition. Run existing scoped registry/ledger checks where structural changes are proposed. A source-only draft has no live application receipt.

## Failure and recovery

Wrong World, unreadable/private target, stale revision, unsupported property/qualified assertion, absent authority or unknown connection stops the corresponding write. Preserve the draft and evidence. Do not recreate containers, overwrite another lane, merge duplicate qualifications, delete source to make the script idempotent or assert a successful rollback without observing it.

## Continuity and native delivery

Use `ql-evidence-report` for the concise returned difference and retain the exact script/log/source refs in the authorised work record. AIKit owns discovery, review and projection of this canonical Skill. Repository-relative paths identify canonical QL source; they need not exist beneath a generated Skill payload. The full UX proof is the corresponding source/graph branch of UX05/UX10, not a SQL-like statement succeeding alone.
