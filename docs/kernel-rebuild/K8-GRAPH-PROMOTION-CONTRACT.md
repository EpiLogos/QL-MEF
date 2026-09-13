# K8 reviewed graph promotion

Owner: #132 K8.0. Source authority remains the existing
[aperture/centre promotion](K8-STRUCTURAL-CONTRACT.md),
[aperture specification](APERTURES-AND-CLOCK-CENTRE.md) and
[full pre-K8 lock](PRE-K8-AGENT-WORLD-LOCK.md).
This implements their graph application/rollback requirement. It does not add
another source registry, edit the frozen C corpus, implement AW's rich property
vocabulary, or claim production Bimba application merely from controlled tests.

## One projection, actual graph identities

`scripts/k8-graph.py` calls `scripts/k8-structure.py::project`. It derives the
**complete 27-node / 78-relation amendment** by comparing current structure with
the accepted v1 source field. The plan is regenerated and compared before every
operation. It cannot acquire hand-edited allocations, aliases or relations just
because someone recomputes its hash.

The graph's existing `coordinate` property is retained. Existing node spellings
come from the captured K4 joins, not a new string normaliser. In particular:

```text
canonical kernel/source  #3-5-5/0
captured graph coordinate M3-5-(5/0)
same existing centre, different retained representations
```

The operator refuses a missing centre, duplicate coordinate, parallel alias,
foreign pre-existing allocation or partial/conflicting promotion. It does not
repair these by overwriting a node, stripping its slash, creating a second
centre or moving any of the 360 degree children.

New nodes carry existing `Bimba`/`Coordinate` labels and the bounded migration
ownership label `QLK8PromotionNode`. Their `ql_k8_*` properties record canonical
IDs, source refs, parent IDs, promotion and registry revision, plus the exact
source assertions. This metadata is transport/application provenance; it is not
a substitute for AW1's definition/assertion/compact/quintessential/derived
property grammar. New relationships use the actual compiler-produced types.
Their IDs, direction, orientation, source record and original qualified payload
are retained independently. Native IDs stay hexadecimal strings, never lossy
JSON numbers. Undirected source standing remains explicit on the one stored
assertion; it does not silently become two directed claims.

A pre-existing relation with the same endpoints and type is **not overwritten**.
It remains an independent assertion beside the source-qualified promotion.
Only exact identity/property equality counts as an already applied amendment.

## Target review and atomic application

Offline use needs only Python's standard library:

```sh
python3 scripts/k8-graph.py plan --out target/k8-graph/plan.json
python3 scripts/k8-graph.py schema --out target/k8-graph/schema.json
```

The schema output is an inspectable provisioning request, not an executed change.
The graph owner provisions the three uniqueness constraints and the single shared
structural-integration lock explicitly. Apply never creates a missing constraint,
relabels existing nodes to satisfy one, or weakens schema checks. Existing target
anchors must already be uniquely identified `Bimba` nodes. All participating
structural writers use the same lock; arbitrary external schema/identity edits
still require native graph-owner coordination.

The operator supplies a target explicitly. There is no default database server,
ambient credential discovery or historical password reuse. Credentials are read
from `QL_K8_GRAPH_PASSWORD` (or the named environment variable), never retained in
plans, reports or command arguments. Plain HTTP is limited to loopback; other
origins require HTTPS. Redirects and URL credentials are refused.

```sh
python3 scripts/k8-graph.py review \
  --endpoint "$GRAPH_ORIGIN" --database "$GRAPH_DATABASE" --user "$GRAPH_USER" \
  --out target/k8-graph/review.json

# PLAN_SHA is the exact plan_sha256 returned by the offline projection.
python3 scripts/k8-graph.py apply \
  --endpoint "$GRAPH_ORIGIN" --database "$GRAPH_DATABASE" --user "$GRAPH_USER" \
  --review target/k8-graph/review.json --confirm-plan "$PLAN_SHA" \
  --out target/k8-graph/apply.json
```

A review binds the plan, explicit origin, actual database ID, exact target
snapshot and source-anchor digest. Its existence is not a permission grant.
The native graph owner authorises the operation and target. Review files expose
only scoped fingerprints and target identity, not complete private properties.
Receipt paths cannot overwrite existing evidence.

Application uses one explicit, non-retrying transaction: verify the schema;
lock the shared integration record; lock affected existing nodes/owned relations;
read and compare the exact target; create only the full amendment; verify the
complete result and original anchors; then commit. Same-value dependent property
writes obtain locks without borrowing/removing a temporary property from someone
else's node. A concurrent duplicate application returns an exact no-op after the
first transaction, not a second set of coordinates or assertions.

The native transport extends the repository's existing census HTTP transaction
route for the captured Neo4j 5.26 installation. That HTTP API is deprecated in
5.26 and unavailable on Aura; this version does not claim a generic Aura or
future-version adapter. Query semantics are covered by the pinned disposable
Neo4j workflow, not inferred from a Python in-memory specimen. No source/model
or graph work runs in the continuous field's real-time callbacks.

## Rollback without erasing subsequent work

Take a new review of the applied state, then call `rollback` with that review
and the same explicit plan confirmation. Application and rollback are separate
operations and need the corresponding reviewed target state.

Rollback requires exact promoted node labels/properties, complete exact relation
identities/qualifications and the application marker. Added node properties,
changed source assertions, altered labels, missing/duplicate edges, external
links (even to nodes without a coordinate), or an amended marker cause refusal.
No automatic repair or broad detach-delete is available. Remove only the checked
78 owned assertions, 27 owned nodes and exact application marker; leave existing
source nodes, degree edges, independent assertions and the provisioned schema/
shared lock intact. A repeat rollback is an exact no-op.

An interrupted request before commit is rolled back by the explicit transaction
or server expiry. A lost/malformed commit acknowledgement is **unknown**, not a
claimed rollback. The operation writes an unknown-standing receipt, does not
retry, and requires a fresh target review to discover what actually committed.
Do not convert an earlier acknowledged snapshot into current live standing.

## Executable evidence

`python3 -m unittest discover -s scripts/tests -p test_k8_graph.py -v` exercises
the full source-derived plan, identity/provenance/count laws, idempotence,
source/target/alias/partial-state conflicts, rollback protection and transport
refusal. These are offline source/admission tests, not graph execution evidence.

`.github/workflows/kernel-k8-graph.yml` executes
`scripts/test-k8-graph-live.py` against a disposable Neo4j 5.26.20 Community
service. It refuses any initially nonempty target and requires an explicit test
opt-in. Its controlled baseline includes every one of the 360 native centre
children and independent parallel assertions. It tests concurrent apply,
repeated apply/rollback, byte-equivalent original graph restoration, amended
sources, foreign allocations/aliases, external knowledge, a missing lock
constraint and a real post-write transaction failure. Exact version, image
digest, source/tree, plan/reviews, operation receipts and original/restored
digests are retained in its artifact.

The current-source census records the Python projection/transaction adapter as
infrastructure and preserves the historical ledger and all inherited unresolved
constructs. Neither the source receipt nor controlled graph execution changes
`pending-live-application` for the owner's Bimba graph. AW1 root/property joins,
AW2 actual performance, #134 Nara and the remaining K8 material/UX requirements
retain their own implementation and acceptance standing.

## Transport references

The implementation follows Neo4j's explicit transaction, uniqueness-constraint
and dependent-property locking contracts:

- https://neo4j.com/docs/http-api/current/transactions/
- https://neo4j.com/docs/cypher-manual/5/constraints/syntax/
- https://neo4j.com/docs/operations-manual/current/database-internals/concurrent-data-access/

These are database mechanism references, not authority for the K8 source graph.
