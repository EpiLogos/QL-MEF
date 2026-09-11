# M1 / Paramaśiva engine candidate — K5 (#129)

Status: **candidate 0.1.0; not whole-M1 acceptance**. This work was opened on
`e753efc91f62b5b2af09e0a852c5063e366eccbe`. At inspection on 2026-09-11,
#128 contained the accepted K3 handoff, not an accepted K4 M1 census. Therefore
this return must not close #129 or be described as executing an accepted K4
vertical. It supplies a tested, bounded native implementation for that review.

## Authority and recovered depth

K2's `fixtures/kernel/m-tree-v1.json` remains the only coordinate/relation
registry, unchanged at revision
`259a2f496c5f3a76d31e5c480dc9afdb45ad1282a7034cc28c528c39a71442e4`.
K3's `fixtures/kernel/m-ledger-v1.json` remains the only readiness/discrepancy
ledger. Original imported matrix rows, Markdown rationales, source spellings,
parent discrepancies and seven-stratum/five-axis assessments are retained.
New bounded rows disclose what this candidate actually computes; they do not
upgrade inherited M1 capability rows or the aggregate index to complete.

The original M1 C body remains untouched under `vendor/epi-kernel/reference`.
The later accepted upstream return was recovered in full from
`EpiLogos/Epi-Logos-C-Experiments@bb47ab9730f0ddadd4891666fb6f3e0a6d457330`
(PR #33, `research/deep-subsystem-capability-matrices`, not upstream main).
Its exact C projection source/header and complete Vortex Modulae CSV are retained
under `migration/epi-kernel/m1-return`, with blob/SHA-256 locks and original paths.
The recovery also read the full Ananda/Spanda architecture and executable
contract, musical derivation lock, `portal-core/src/ananda_vortex.rs`,
`ananda_music_bridge.rs`, `ananda_ratio_basis.rs`, `ananda_ql_bridge.rs`,
`ananda_traversal.rs`, `hopf.rs` and their tests. Those remain upstream sources,
not a claim that every instrument/application wrapper has been transplanted.

The new Rust code composes existing `ql-mef` matheme/music/completion and
`ql-core` relation/quaternion operators. It does not introduce a new A/B/C
classifier, D grammar, context-frame count, quaternion algebra or M hierarchy.

## Native C boundary

Include `<ql/m1.h>` and link the existing `libql-mef-c.a` plus the platform math
library. The header is C11/C++17 compatible. Version: `QL_M1_ENGINE_VERSION`.

| Operation | Exact native source seats | Contract |
| --- | --- | --- |
| `ql_m1_cell` | `#1-2-0` through `#1-2-5`, each exact `-0` reflection | Five signed raw contributors, true DR contributors, optional decimal-10 aperture, optional scalar and clock |
| `ql_m1_clock` | `#1-3`, `#1-5-2` | Cycle/tick, direct/prime, same-position conjugate, independent Hopf fibre and degrees |
| `ql_m1_spanda` | `#1-3-0` through `#1-3-5`, plus the six exact compound Flowering coordinates | Source compiler-pass weave/inversion result, fold sieve and dual-track substage |
| `ql_m1_formal` | `#1-4.0` through `#1-4.5` | Source stage/next/inverse, six trigonometric operand slots and Cl(4,2) signature |
| `ql_m1_topology` | `#1-5` | Retained element-count and quaternion LUT, explicitly named historical return-stage convention |
| `ql_m1_torus` | `#1-5-1` | Source torus embedding with R=16/9 and r=1; finite angles only |
| `ql_m1_node` | Only existing M1 nodes | Registry-owned immutable descriptor, including original separator/alias semantics |

Every checked C computation returns 1 on success or 0 on invalid input/missing
seat; a null output is invalid and an error leaves the caller's output unchanged.
No allocation, mutable global initialization, private filesystem or shared graph
mutation occurs. Input indices are checked at full `uint32_t` width; 256 cannot
truncate to family 0. Cycle parity is computed without multiplying an absolute
counter, so `UINT64_MAX` remains valid. All data pointers returned by the registry
are registry-owned and process-lifetime. Output structs belong to the caller.
C layout/padding is a native ABI, not a network serialization. Do not memcpy it
to a file/socket or infer byte stability across architectures.

Ananda family order is Bimba, Pratibimba, Sum, Difference A, Difference B,
Quintessence. Contributor order is the first five families. Raw values are
`b=row*col`, `b+1`, `2*b+1`, `-1`, `1`. The accepted runtime's Difference A
complement residue is 9. Decimal-10 is available only for row/column below 10.
A non-scalar Quintessence still carries all five raw/DR/decimal contributors;
its source-rule projection is `{-1, b, 2*b+1}`. Zero in an unused C scalar field
is not a scalar value: inspect `scalar_valid`. Inspect `decimal10_valid` before
reading decimal fields. The legacy XOR decimal operation remains in the frozen
source, not silently relabelled as this tuple.

The exact Flowering coordinates are `#1-3-4.0000`, `#1-3-4.0/1`,
`#1-3-4.0/1/2`, `#1-3-4.0/1/2/3`, `#1-3-4.4.0-4.4/5`, `#1-3-4.5/0`.
They are not expanded into invented slash-separated nodes. Fold counts are
4,6,8,10,12,0; dual track is the fourth substage. A nonzero substage is accepted
only for Flowering. The source mutators are executed in the test oracle; their
weave/inversion writes are checked against the native result. This is not an
unclaimed replacement for an application's mutable HC/arena lifecycle.

Two clock conventions are deliberately separate. The returned canonical clock
has `position6=tick12%6`, `phase=tick12/6`, `hopf_fiber=cycle%2`,
`degree360=30*tick12`, `degree720=degree360+360*hopf_fiber`. Direct/prime and
fibre have four combinations. The historical role-return sequence is
0,1,2,3,4,5,5,4,3,2,1,0. Its retained ring quaternion LUT is not silently
replaced by a uniform angle orbit. Rust `Clock::spinor` names the generated
clock orbit separately; it changes sign after 12 absolute ticks and returns
after 24. Neither arithmetic convention proves the corpus's broader
experiential/topological necessity claims.

## Rust and C++/M1′ composition boundary

Rust exposes `ql_mef::m1::{cell, Clock, spanda, formal, topology, torus,
source_ratio, ratio_basis, traverse, traverse_json}`. `traverse` accepts the
actual source/target QL coordinates, caller pointer evidence, chosen Ananda
cell, explicit conjugate participation, basis and lens. It never substitutes
a clock-derived cell for the walker-selected cell. All pointer roles and
zero-to-many A/B/C candidates survive, including overlapping pairs, reversed
walks and the canonical side of D2 expansion. Cross-face walks are not invented
as primary A/B/C relations. Existing QL operators own musical completion.

Source-row exact ratios are only the explicitly evidenced rows: P0=1/1,
P3=4/3, B6=2/3, P7=16/9, B8=8/9, B9=1/1. Reciprocals and composition retain
a derivation tree, including both operands. Checked widened products reduce
before conversion back to the existing positive `HarmonicRatio` register.
They recover the existing eight canonical ratios, not a new tuning grammar.

The candidate wire envelope is `ql.m1.traversal/v1`. `traverse_json` rejects
unknown fields/schema, invalid indices/faces/lenses and malformed counters.
`cycle` is an unsigned decimal **string** to preserve 64-bit identity in clients.
Source/target have `position6:0..5` and `phase:0|1`; `lens12` uses the existing
L0..L5,L0′..L5′ order. Basis is `chromatic|fifths`; participation is
`none|source-only|target-only|both`. Every pointer field is required; roles may
be empty. Example:

```json
{"schema":"ql.m1.traversal/v1","source":{"position6":2,"phase":0},"target":{"position6":3,"phase":0},"pointer":{"source_ref":"S2:2","target_ref":"S2:3","relation_ref":"walk:2-3","relation_roles":["A","C"]},"family":5,"row12":3,"col12":4,"cycle":"1","tick12":7,"participation":"both","basis":"fifths","lens12":11}
```

Output carries registry/source-return/music versions, exact M1 IDs, all cell
contributors, source-ratio evidence, original pointer evidence, observed
traversal orientation, interval, every candidate's family/pair/D degree/side,
operator reference, completed coordinates and pitches. Pointer evidence is
labelled `caller-supplied`, not authenticated Neo4j evidence. Acceptance is
explicitly candidate and experiential parity remains unassessed. No Rust ABI,
C++ runtime, audio driver, renderer or transport is implied by this JSON method.
The installed C++ consumer proves linkage/data consumption only, not M1′
instrument behaviour. K8 can consume this candidate but must not advertise a
stable accepted whole-M1 engine until K4/K5 decisions are returned.

## Literal source pressure retained through the ledger

`docs/kernel-rebuild/m1-source-comparison-v1.json` compares all 1,728 raw/DR
literal readings of the complete 6×12×12 CSV with **actual native C output**.
There are 1,352 exact textual matches and 376 differences: Sum DR has two
`0/1` literals where the accepted typed result is 1; Difference A DR has 144
source 1 / runtime 9 readings; Quintessence has 120 raw and 110 DR textual
differences. The report records every source row/column, literal and projected
value. Some may express different symbolic/register meanings rather than
incorrect arithmetic. This work does not infer that interpretation, normalise
`0/1`, erase a component, rewrite the CSV, or claim full Bimba/source parity.
The four corresponding existing-ledger discrepancies remain open for K4/K5
resolution. The runtime authority is the accepted typed return; the CSV remains
an independently addressable source witness. Passing runtime parity cannot
close those source/coordinate interpretation questions.

## Executable evidence and remaining acceptance

`cargo test -p ql-mef --test m1_engine --locked` executes 20,736 cells across all
families, rows, columns, ticks and both fibres against the retained C return,
then compares the actual C descriptors with independent Rust execution. It
also exercises 6,912 actual-walk/basis/lens/conjugate combinations, raw/DR/
decimal separation, exact ratio provenance/overflow, D2 reversal, JSON rejection
and full-width counters. `scripts/check-m1-source.py` validates witness hashes
and detects any changed literal/runtime pressure rather than hiding it.

`bash scripts/test-m1-engine.sh` runs ASan/UBSan, actual source/typed observations,
and an installed-library C++17 consumer. It is added to the existing native C
job, not a new CI matrix. Existing whole-registry and retained kernel tests remain
in place. C/Rust source/coordinate/operational evidence here is scoped to these
operations; it is not blanket relation, live Neo4j, application, instrument or
experiential parity. The original inherited source rows remain unassessed.

Still required for #129: accepted K4 M1 inventory/work graph; full per-coordinate
M1 census and dispositions (including existing HC/arena/CLI and other wrappers),
review of the four literal-source pressures and two clock conventions through
that authority, the remaining coordinate/relation/operational obligations,
and accepted stable M1′ handoff. No K6/K7 source, core schema, canonical agent
name, registry node or relation is changed here.
