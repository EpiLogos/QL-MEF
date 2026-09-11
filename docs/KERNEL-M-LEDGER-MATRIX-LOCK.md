# K3 — M capability ledger, matrix lock and parity protocol

K3/#127 consumes the accepted K2 registry at
`e3e0bc12a632b8f4cb1960387b4920c794393d52`. Its public companion is
`fixtures/kernel/m-ledger-v1.json`, governed by
`fixtures/kernel/m-ledger-v1.schema.json` (`ql.m-ledger/v1`). The native Rust
consumer is `ql_mef::m_ledger`; `ql kernel` exposes it without Python, Neo4j or a
working checkout. The checkout-dependent source/evidence verifier and matrix
importer are `scripts/m-ledger.py`.

## One coordinate field; distinct capability records

The ledger references the exact K2 registry path, byte digest and content
revision. K2 alone supplies coordinate identity, parent/children, names,
source-record and payload provenance, source relations and alternate spellings.
Every one of its 1,876 nodes remains structurally present whether or not a
capability or computational implementation has been discovered. The ledger does
not create missing prefixes, regularise the tree or equate `-`, `.` and `/`.

The initial import is automatic: 149 records from the four existing deep M1–M4
matrices and 36 records from the six normalized capability-field partitions.
These **185 source capabilities** retain distinct family-qualified identities,
roles, original carrier paths and JSON pointers. Seven separate K2 aggregate
index rows bring the seed to **192 rows**. No deep M1/M2/M3 census was manually
populated, and no source capability was pronounced implemented by this import.

The ten matrix locks hash both their JSON and their Markdown rationale. The
original rationale and richer domain fields remain in those carriers, not
rewritten into a flattened description. The ledger extracts coordinate
candidates from `coordinate` and `relations` fields; their exact spellings remain
source assertions. Composite expressions are retained at the source pointer;
prime expressions and ranges are not expanded into invented nodes. Unresolved
source-coordinate candidates are reported as gaps, not silently "corrected" to
a neighbour. A matrix's scope provides a query anchor, not a computational
binding or a claim that the whole scope is covered.

`ql kernel ledger <coordinate> --json` returns a joined view containing the exact
K2 node (including parent and children), source files/records, cross-coordinate
relations, applicable capability rows, full assessments and implementation
bindings. `ql kernel ledger --json` exports the complete normalized ledger.

## Independent readiness and parity

Each row names an assessment profile. Profiles factor repeated state; they do
not merge capability identities. Each profile must explicitly contain readiness,
warrant and evidence for **source, C, Rust, C++, Neo4j, application and instrument**.
The allowed readiness states are `unassessed`, `unimplemented`,
`structural-index-only`, `partial`, `implemented` and `verified`. There is no
scalar `COMPLETE` flag, and the parser rejects unknown fields and unsupported
states. An omitted implementation disposition means `unassessed`, never
not-applicable or complete. A bound disposition requires a real binding.

The five separate parity axes are **source, coordinate, relation, operational
and experiential**. Each contains independently warranted peer comparisons.
An empty axis means no assessed pair, not equivalence. Comparisons name both
peers in Bimba/C/Rust/C++; equivalent parity requires scoped executed evidence
for both peers and that axis. Reverse entries cannot contradict an existing
comparison of the same pair. None of the five axes implies any of the others.

K2's seven index rows expose C and Rust structural-index bindings. Their evidence
is explicitly the K2 declaration, not a newly invented run receipt. It cannot
support computational verification. All imported capability readiness and all
initial peer parity assessments remain unassessed. Current native C/Rust registry
parity is exercised by executable observation tests, separately from a capability's
computational or experiential readiness.

Evidence records carry a repository-relative artifact path and digest, registry
revision, exact subjects, strata, axes, kind and result. Wildcard subjects are
forbidden. Source declaration cannot establish implemented computation. Verified
Rust/C++/application readiness needs operational evidence; C/Neo4j verification
needs coordinate evidence; instrument verification needs experiential evidence.
Live Neo4j verification additionally requires an observation, not a serialized
source file. A passing coordinate test cannot become operational or experiential
proof. The native constructor validates evidence structure and scope; the
checkout verifier checks bytes. Neither claims to authenticate an arbitrary
external receipt or to prove its assertions without trusted review/execution.

## Implementation dispositions and returned disagreement

Implementations are an independently enumerable inventory, not merely fields
nested in rows. Each record names its stratum, source path, symbol, structural
index versus computational kind, and coordinate-bound, cross-coordinate or
infrastructural disposition. A construct without a row/coordinate disposition
is an orphan. Explicit infrastructure must have no invented coordinate and must
state why it is infrastructural. Source path/symbol presence is a conservative
checkout check, not a whole-program symbol census or proof of execution. K4 can
add compiled/scanned discoveries to this inventory; K3 does not claim all existing
C/Rust/C++ constructs have already been inventoried.

An implementation may carry explicit parent/child/relation assertions. They are
compared with the shared registry, including actual relation identities rather
than inferred lexical neighbourhood. Orphan implementations, dangling bindings,
unknown coordinates and a Rust binding attached to a different capability
coordinate are validation errors. Registry structure is not rewritten to make
them pass. A semantic/research relation can remain unpromoted without a C port.

Discrepancies support **every directed pair** among Bimba, C, Rust and C++.
They retain subjects, parity axis, both peers, their assertion/difference, current
authority and rationale, the proposed target/change, decision evidence and a
referenced transition history:

```text
open -> proposed -> accepted -> applied
  \         \           (application needs corrected-peer evidence)
   +---------+-> rejected
```

There is no fixed one-way source hierarchy. A C++ result can propose a correction
to Bimba; that proposal does not itself change canon. Accepted decisions name the
authority and scoped evidence. Applied resolutions require executed evidence from
the corrected peer. Proposed structural promotion can remain open; accepted or
applied promotion to structural kernel canon requires C binding and appropriate
C parity. Promoted structural relations also need the explicit promotion decision.
A semantic graph relation cannot bypass this law by changing a label.

The known K2 `#2-4` / `#2-4.5` parent assertion remains an open, explained
source-versus-containment discrepancy. It is neither erased nor automatically
interpreted as a production defect or a mandate to change the source tree.

## Coverage and verification

```sh
# Checkout-dependent schema, matrix inventory, Markdown and evidence locks.
python3 scripts/m-ledger.py check

# Native semantic validation (gaps are reported; false assertions fail).
cargo run -p ql-cli --locked -- kernel validate-ledger --json

# Exact scope, requested stratum and parity axis; no silent fallback to whole M.
cargo run -p ql-cli --locked -- kernel coverage M1 --json
cargo run -p ql-cli --locked -- kernel coverage M2 --stratum c --axis coordinate --json
cargo run -p ql-cli --locked -- kernel coverage M3 --stratum rust --axis operational --json
cargo run -p ql-cli --locked -- kernel coverage '#0-4.0/1/2' --json
cargo run -p ql-cli --locked -- kernel ledger '#0-4.0/1/2' --json

# All source and executable regression tests.
python3 -m unittest discover -s scripts/tests -p test_m_ledger.py -v
cargo test -p ql-mef --test m_ledger --locked
cargo test -p ql-cli --test m_ledger_cli --locked
cargo test -p ql-mef --test native_m_tree --locked
cargo test --workspace --all-targets --locked

# Compare every actual K2 C/Rust descriptor, not just a hand-authored list.
bash scripts/test-native-m-tree.sh
python3 scripts/m-ledger.py observe --peer c --input target/m-tree/native.jsonl
python3 scripts/m-ledger.py observe --peer rust --input target/m-tree/c-rust-parity.jsonl
```

Coverage returns `ql.m-coverage/v1`: scope and revisions, structural coordinate
count, structural-index bindings, exact coordinates lacking a computational
binding in the requested stratum, coordinates lacking a capability row, source
capabilities without an implementation disposition, blocking rows and integrity
findings. Explicit dependencies expand the requested vertical; missing/cyclic
dependencies fail validation. Global integrity errors and unscoped orphans remain
visible in narrow queries rather than disappearing under a scope filter.

The default request is verified Rust/operational coverage. `--stratum`, `--axis`
and `--require implemented|verified` select a different request. Index-only
existence cannot satisfy either computational requirement. A successful query
means the report was produced, not that its vertical is ready. The JSON arrays of
blockers/missing coordinates and findings are the machine-consumable answer.

Coverage must match the requested stratum to a participant in its parity claim.
`source` corresponds to the Bimba peer and consumes source-scoped evidence; it
cannot borrow a C/Rust-only comparison. Live Neo4j, application and instrument
readiness remain separately expressible, but the v1 four-peer parity protocol
does not give them an implicit peer alias. Their parity-dependent coverage stays
blocked rather than clearing a row from an unrelated native comparison. This
restriction does not erase their readiness evidence or establish a source/live
graph equivalence.

The observation verifier consumes K2 descriptor JSONL produced by the actual C
probe and Rust parity acceptance. All nodes, relations, files, records and bindings
are compared. Missing/duplicate/extra nodes or descriptors, changed parentage or
relations, malformed observations and an absent/stale registry header are
reported. No observer supplied means no observed parity, not an assumed match.
The `--peer` label identifies supplied evidence; it does not turn a replayed C
file into a newly executed C++ or live Neo4j observation.

## K4 handoff and refresh discipline

`docs/kernel-rebuild/m-ledger-v1.json` is the K3 -> K4 handoff. K4/#128 adds the
actual discovered M1/M2/M3 implementation inventory, scoped assessments, evidence,
discrepancies and dependency/work graph to the same ledger. It must not start
another coordinate tree or infer readiness from source matrix words such as
"implemented" or "canonical" without examining the evidence per stratum.

`python3 scripts/m-ledger.py refresh` deterministically refreshes matrix imports
and content locks while preserving row assessments, bindings, dispositions,
invariants, relations, dependencies, evidence and discrepancy decisions. New
source capabilities are added unassessed. Removed source capabilities remain as
explicit stale-source errors until reviewed, not silently dropped. Changed source
imports must be reconciled in the same reviewed change as their rationale;
refresh alone is not an authority decision. Evidence tied to an old registry
revision is not silently retargeted. Run checkout and native validation after
refresh; both are necessary.

The `M ledger / matrix lock` workflow executes these validators, mutation tests,
actual C/Rust descriptor checks and native CLI queries. Other existing native,
source-ground and Rust workflows remain active. CI receipts identify actual
commits; this document is not a substitute for those receipts. No full M body,
full implementation census, C++ embodiment or authenticated live graph observation
is claimed by K3.

K4 handoff update (#128): the automated M1/M2/M3 census has since been executed
onto this same ledger — discovered implementations, scoped readiness, evidence
and discrepancies. See `docs/kernel-rebuild/K4-M123-CENSUS.md` and
`fixtures/kernel/census/`; the import, refresh and verification laws above are
unchanged and continue to govern it.
