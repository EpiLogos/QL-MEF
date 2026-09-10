# K2 — native recursive M registry

This is the #126 implementation handoff on accepted K1 main
`c3f49a7e099b2af908bac11ec2ea4ce7f3e351e9`. The K1 library and contract remain
available; its historical 36-node substrate is not the recursive M field.

## One source field, two executable consumers

`c/registry/m-tree-source-v1.json` freezes the existing Bimba compiler output
from `EpiLogos/Epi-Logos-C-Experiments` at
`daa660cbc1b8c5da83828698665a753852cb0287`, dataset tree
`cd4f4f77c13f27e2563c5a6753d2f8bf2b605f15`. Its lock and both existing compiler
hashes are retained. No second hand-authored coordinate tree is introduced.
The source pool is the accepted serialized corpus, not a new live Neo4j read.

`scripts/generate-m-tree.py` produces the immutable native descriptors in
`c/src/m_tree_data.inc` and `fixtures/kernel/m-tree-v1.json` (`ql.m-tree/v1`).
`fixtures/kernel/m-tree-v1.schema.json` defines the public document shape.
The native `c/src/m_tree.c` library resolves the former; Rust's
`ql_mef::m_tree::MRegistry` consumes and validates the latter. The Bimba
conformance workflow recompiles the exact historical corpus and demands
byte equality with all three checked-in products. Offline generation and
normal C/Rust builds require no historical checkout or graph connection.

The field contains **M plus 1,875 source coordinates**, including its six
aggregate roots. Source-coordinate subtree counts for M0–M5 are
**108, 43, 597, 996, 100, 31**. It retains **21,083 source relations**, including
2,676 cross-M relations and 450 relations with at least one partial/meta
endpoint, **24,831 payload record references from 57 files**, eight groups of
alternate spellings, and 14 rootless meta records. Records are references to
payload hashes and source objects, not replacements for the original payloads.

## Identity, containment and source spelling

Native IDs are nonzero `uint64_t`; JSON serializes them as sixteen lowercase
hexadecimal digits, avoiding JavaScript number precision loss. A node ID is
the first 64 bits of SHA-256 over `ql.m-node/v1`, a NUL byte and the exact source
spelling. Relations use the `ql.m-relation/v1` domain and their exact relation
reference. The generator rejects collisions instead of silently aliasing.
An unrelated insertion does not renumber existing identities. Array offsets
are only local compact indexes, never exported as durable coordinate IDs.

`M` is the master index. `M0` and `#0` resolve the same root; this namespace
substitution is the only implicit spelling alias and applies to descendants.
`#` and `#-0` remain external meta references, not aliases for M. There is no
whitespace trimming, numeric normalization or separator substitution.

Each node records its parent, root, local segment, separator, lexical depth,
tree depth, source parent assertions, structural standing and provenance.
A unique source-asserted proper-prefix parent supplies containment. Otherwise
the longest *existing* proper boundary prefix supplies an explicitly labelled
`existing-source-prefix` parent. Missing prefixes and siblings are not created.
Multiple eligible source parents fail generation rather than choosing silently.
Non-tree parent assertions and original source relations are retained.

In particular, `#0-4.0/1/2` is directly under `#0-4`, with separator `.` and
local segment `0/1/2`. Its lexical depth and containment depth differ; an
invented `#0-4.0` node would be wrong. `#0-4.0/1-2` remains a distinct identity,
as do `#4.5-0` and `#4.5.0`. The source's `#2-4` assertions name both `#2` and
`#2-4.5`; containment uses `#2`, while the backward assertion stays visible in
`source_parent_refs`, `parent_discrepancies` and the source relation corpus.
Nothing is silently repaired into a regular six-way tree. Traversal and
subtree construction are iterative; the generator regression adds a 19-level
asymmetric path without changing old IDs.

## Public native and Rust operations

Include `ql/m_tree.h` and link `libql-mef-c.a` with `-lm`. M registry API 1.0.0
is additive to the K1 library API 0.1.0 / semantic contract 1.1.0.

- `ql_m_master`, `ql_m_root`, `ql_m_resolve`, `ql_m_node_by_id`, `ql_m_node_at`:
  master, aggregate and exact coordinate resolution.
- `ql_m_parent`, `ql_m_child_count`, `ql_m_child_at`, `ql_m_subtree_count`:
  deterministic tree navigation, distinct from arbitrary source relations.
- `ql_m_node_record_at`, `ql_m_source_record_at`, `ql_m_source_file_at`:
  source/payload provenance; the associated count and record-index operations
  support exhaustive enumeration.
- `ql_m_relation_resolve`, `ql_m_relation_by_id`, `ql_m_relation_at`:
  explicit source relations, including unresolved external/meta endpoints.
- `ql_m_binding_at`, `ql_m_binding_valid`: coordinate-bound, cross-coordinate
  and infrastructural module identities; validity never creates a coordinate.
- `ql_m_registry_revision` and source revision/repository functions: exact
  data identity independent of the native build's Git revision.

All returned descriptors/strings are immutable, process-lifetime borrowed
values. Lookup allocates nothing; invalid IDs, indexes and pointers fail
closed. Tables hold no implementation function pointers or mutable readiness.
C numeric node/record offsets are 32-bit internal indexes; IDs are 64-bit.

Rust exposes `native_m_registry()`, `MRegistry::from_json`, exact lookup,
parent/children/root/master traversal, relation lookup and binding validation.
`coordinate(reference, face)` and `to_m_map_index()` project onto the existing
`MCoordinate` / `MMapIndex` API while retaining source record/payload provenance,
relations and Bimba/Pratibimba reflection. Legacy roots keep their `#` parent
convention; M itself is represented by the new index, not forced into that
legacy coordinate grammar. The legacy lexical-parent helper now slices original
spelling so leading-zero segments are not rewritten.

## Structural existence is not implementation readiness

All 1,875 source coordinates exist even without a computational binding.
The seven built-in bindings cover M and M0–M5 **structural indexes only**.
Their `structural-index-only` readiness does not assert a completed M kernel,
full source capability coverage or any operational/experiential parity.
An absent binding, an invalid binding or removal of every binding cannot erase
or invent source coordinates. `COORDINATE-BOUND`, `CROSS-COORDINATE` and
`INFRASTRUCTURAL` identities are available for the later census; a full ledger,
capability dispositions and computational ports remain K3/K4 and later work.

## Reproduction and evidence

```sh
python3 scripts/generate-m-tree.py --check
bash scripts/test-native-m-tree.sh
cargo test -p ql-mef --test native_m_tree --locked
cargo test --workspace --all-targets --locked
cmp target/m-tree/native.jsonl target/m-tree/c-rust-parity.jsonl
```

To refresh from a separately compiled, locked Bimba source pool:

```sh
python3 scripts/generate-m-tree.py --import-compilation target/epi-bimba-map
python3 scripts/generate-m-tree.py --import-compilation target/epi-bimba-map --check
```

The native probe makes 126,098 assertions and emits 47,855 JSONL records
(including metadata). Native, sanitized Clang, installed-library and Rust
acceptance compare every node, relation, source-file, source-record and binding
descriptor, not merely a shared authored list of expected spellings. Python
regressions exercise full regeneration, exact source shape, deep extension,
duplicate/dangling/ambiguous-source rejection and actual compiled C output.
Rust also exercises source-parent/reflection compatibility, unbound existence
and malformed-registry rejection. Existing K1/C/Rust regressions remain active.

`registry_revision` is SHA-256 over the canonical JSON manifest with that field
removed (UTF-8, sorted object keys, compact separators, preserved array order).
It is currently
`259a2f496c5f3a76d31e5c480dc9afdb45ad1282a7034cc28c528c39a71442e4`.
The checked-in snapshot has its own canonical-content SHA-256. Regeneration
verifies digests and exact bytes. Rust's external `from_json` validates structure,
not cryptographic authenticity of an untrusted external registry; signed trust
and live graph reconciliation are not claimed by this constructor.

The native workflow publishes the exact source archive and full test receipts.
Run status belongs to the actual commit/PR evidence, not a static prose claim.
The K3/#127 entry point is `docs/kernel-rebuild/recursive-m-registry-v1.json`.
The vendored full M body, source locks and accepted Vāk composition are unchanged;
no full M computational body, C++ embodiment or live Neo4j access is ported here.
