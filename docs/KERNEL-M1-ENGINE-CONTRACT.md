# M1 / Paramaśiva engine contract — K5 (#129)

Status: **stable kernel contract 1.0.0**. Consumes accepted K4/#153 at
`5b24b95d17234ab5d23d84e658c0cc06434b41a3`. All 43 M1 coordinate
rows are reconciled to native operations; all 26 inherited source capabilities
have explicit implementation/consumer/research dispositions in the same M ledger.
K8 continuous embodiment and K9 played instruments consume this kernel contract.

## Authority and recovered depth

K2's `fixtures/kernel/m-tree-v1.json` remains the only coordinate/relation
registry, unchanged at revision
`259a2f496c5f3a76d31e5c480dc9afdb45ad1282a7034cc28c528c39a71442e4`.
K3's `fixtures/kernel/m-ledger-v1.json` remains the only readiness/discrepancy
ledger. Original imported matrix rows, Markdown rationales, source spellings,
parent discrepancies and seven-stratum/five-axis assessments are retained.
Reviewed census rows now disclose native computation; inherited source capabilities
retain wider provider/research standing. The aggregate index remains an index.

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
as primary A/B/C relations. Existing QL operators own musical completion. The positional classifier's
canonical direct-base frame is projected to the actual walked face using the
existing kernel face involution: prime D1 retains both prime endpoints, and
prime D2 adds the requested endpoint on the direct face. Pitches are recomputed
with the same basis/lens. `completion_base_phase` discloses this projection;
`operator_ref` remains the canonical operator's provenance, not a claim that
the returned oriented coordinates are its unprojected direct-base frame.

Source-row exact ratios are only the explicitly evidenced rows: P0=1/1,
P3=4/3, B6=2/3, P7=16/9, B8=8/9, B9=1/1. Reciprocals and composition retain
a derivation tree, including both operands. Checked widened products reduce
before conversion back to the existing positive `HarmonicRatio` register.
They recover the existing eight canonical ratios, not a new tuning grammar.

The traversal wire envelope is `ql.m1.traversal/v1`. `traverse_json` rejects
unknown fields/schema, invalid indices/faces/lenses and malformed counters.
`cycle` is an unsigned decimal **string** to preserve 64-bit identity in clients.
Source/target have `position6:0..5` and `phase:0|1`; `lens12` uses the existing
interleaved L0,L0′,L1,L1′,…,L5,L5′ (`LensId::ALL`) order. Basis is `chromatic|fifths`; participation is
`none|source-only|target-only|both`. Every pointer field is required; roles may
be empty. Example:

```json
{"schema":"ql.m1.traversal/v1","source":{"position6":2,"phase":0},"target":{"position6":3,"phase":0},"pointer":{"source_ref":"S2:2","target_ref":"S2:3","relation_ref":"walk:2-3","relation_roles":["A","C"]},"family":5,"row12":3,"col12":4,"cycle":"1","tick12":7,"participation":"both","basis":"fifths","lens12":11}
```

Output carries registry/source-return/music versions, exact M1 IDs, all cell
contributors, source-ratio evidence, original pointer evidence, observed
traversal orientation, interval, every candidate's family/pair/D degree/side,
operator reference, completed coordinates and pitches. Pointer evidence is
labelled `caller-supplied`, not authenticated Neo4j evidence. Experiential parity remains unassessed. No Rust ABI,
C++ runtime, audio driver, renderer or transport is implied by this JSON method.
The installed C++ consumer proves linkage/data consumption only, not M1′
instrument behaviour. K8 consumes this stable discrete/finite boundary; its continuous implementation
and the played M1′ experience remain separate acceptance objects.

## Source-register reconciliation

The original 1,728-reading comparison remains unchanged: 1,352 textual matches
and 376 differences. K5 resolves **loss/conflation at the implementation boundary**,
not by pretending those unlike readings are equal. Both are now operational:

`ql_m1_source_cell` / `m1_engine::source_cell` exposes **all 1,728 literal raw/DR
expressions**, their exact CSV row and column, source blob and SHA-256. The
generator reads the complete locked source, and independent tests compare actual
compiled C output directly to the CSV (not to the generated Rust fixture).
`ql_m1_cell` / `m1::cell` continues the accepted, independently tested numeric
projection. A caller can inspect the expression and compute with the named
numeric register without losing either representation.

The four ledger decisions are therefore applied as follows:

| Source reading | Numeric projection | Resolution |
|---|---|---|
| Sum DR has two `0/1` literals | scalar 1 | Return both; no automatic normalization of the expression |
| Difference A DR is 1 in the CSV | complement residue 9 for signed -1 | Keep literal and complement policy separately named; never call `%10` digit root |
| Quintessence raw expressions | full five contributors plus `{-1,bimba,sum}` rule | Return the expression and contributors, never replace the sixth family by a scalar/XOR |
| Quintessence DR expressions | numeric contributor residues | Return both the source expression and computed tuple |

This is an interface/representation decision warranted by the original CSV and
the accepted bb47ab9 typed return. It does not decide that every symbolic
expression has a unique scalar meaning. Original bytes, previous comparison,
and each discrepancy's proposal/decision/application history remain available.

The K4 live composite spelling is also retained. K5 rejects promoting guessed
aliases: `#1-3-4.4.0-4.4/5` remains the executable K2 coordinate, while the
observed live `M1-3-4.(4.0/1-4.4/5)` remains source observation, not an accepted
native alias. No parentheses/slashes are silently normalized. The old duplicate
K4 discrepancy is retained with the same no-promotion disposition. This does
not change or judge the live graph's spelling.

## Complete coordinate operations and state

The added native C `m1_state.c` and Rust `m1_engine` module complete the K4
coordinate assignments without a second registry. The original 43 coordinates,
source parents/children/records and 1,072 incident relation observations come
from K2; relation parity compares the complete retained relation descriptors,
including external endpoints, orientation, source record and cross-M flags.
The normalized K4 live capture establishes observed coordinate presence, not a
new execution or a fresh full live-relation observation in this session.

| Coordinate body | Executable state / reading |
|---|---|
| `#1`, `#1-0`, `#1-1` | Bounded source/instance identity, event, revision, selection and reflection; immutable registry source remains separate from mutable state |
| `#1-2`, six family nodes and their six `-0` DR nodes | Full Ananda numeric/source projections and explicit register selection |
| `#1-3`, six stages and all six compound Flowering nodes | Clock, checked advance, source mutator results, fold sieve and selected substage |
| `#1-4`, six dotted stages | Ratio/cardinality derivation, 4+6 frame, inversion, 12-ring, nesting; formal/trig source slots; finite 36/64/128/4096 state/relation field |
| `#1-5`, six topological children | Two-circle quaternion rotor, torus embedding, independent cover/spinor, opposition, quadrature/72 carrier and genus-one structural invariants |

`ql_m1_source_traits` preserves the source branch category table, doubling and
tripling rings and unary/binary/relational masks, verified directly against the
frozen C tables. These M1-generative source tracks do not replace M2/M3 engines.
The source header/source construct inventory includes the constants, header-only
helpers and historical lifecycle/CLI/cache wrappers omitted by name-only discovery.
Each retained construct names its coordinate and native successor family.
Historical allocation/tagged-pointer/CLI residency does not need a duplicate host:
the process-local Rust engine supplies the bounded mutable state, and static C
projection removes lazy-cache mutation. Source is not deleted.

`ql_m1_finite_field` and `m1_engine::finite_field` use independently supplied
direct and prime six-bit words. They expose the 36 relation index, 64 word states,
128 face-indexed states and 4,096 ordered pairs. Pair indices are storage
ordinals, not new M nodes or M3 codon identities. Bitwise complement is separate
from choosing the prime word. C reuses the accepted primitive operations; Rust
uses the existing sixfold relation order and face involution.

`ql_m1_rotor(a,b)` uses `exp(i*a) exp(j*b)` and the existing quaternion multiply;
angles are finite radians. `ql_m1_carrier` emits the generated clock spinor,
quadrature and its opposite, with exact source coordinate handles and 72-carrier
cardinality. Floats describe a numerical realization, never replacement symbolic
identities. The historical nonuniform quaternion LUT and return-stage sequence
remain explicitly available, distinct from the generated 24-tick orbit.

### Operational owner and Actions

`ql_mef::m1_engine::M1Engine` holds private, validated state. Construction requires
an event ref, source subject, selected exact coordinate, cycle/tick, revision and
harmonic configuration. `select`, `advance` and `configure_harmonics` check the
expected revision and validate the entire change before committing. Invalid,
stale and overflowing changes leave the state untouched; no-op changes keep the
revision. Event and subject identity survive every action. `pole_identity()`
returns the existing cross-domain event identity with the actual degree720.

Every selected branch returns **its own computation**, not the currently active
cell relabelled with a selected coordinate. For example selecting `#1-2-3` while
family 0 is active returns Difference A for the chosen cell without changing the
active family. The selected reading and active configuration are separate.
All 43 selected readings are tested. Every basis/lens/CF selection consumes the
existing musical derivation and returns the actual tonic, mode, seven pitches
and Name/Power pattern. All 168 combinations (two bases × 12 lenses × seven
CFs) are checked against the existing operator.

Storage, authorization, session lifecycle, physical scheduling and realtime
interpolation remain with native consumers. This owner is not a second generic
Actuation/AIKit runtime and does not mutate Bimba or authored source.

### JSON and CLI boundary

The strict request contract is `ql.m1.engine/v1`; its schema and complete example
are `fixtures/kernel/m1-engine-v1.schema.json` and
`fixtures/kernel/m1-engine-v1.request.json`. Invoke:

```sh
ql kernel m1 fixtures/kernel/m1-engine-v1.request.json --json
```

`config` is required. The optional `action` is `advance`, `select` or `harmonics`,
each with `expected_revision`; `null` is a read. Cycle/revision/tick-delta counters
are unsigned decimal strings of at most 20 digits, additionally checked for u64
overflow. Invalid coordinate spellings, out-of-range values, unknown fields and
stale revisions fail. Event refs are non-blank and bounded to 4096 UTF-8 bytes.
The returned configuration can re-enter the request transport without precision
loss. A transported configuration is caller-supplied data, not authenticated
remote mutation authority; persistent consumers keep the actual engine instance.

The separate `ql.m1.traversal/v1` boundary remains available with source/target
pointer evidence, A/B/C participation and D completion. Public C and Rust APIs
also expose numeric geometry and finite-state operations for in-process clients.

## Remaining source-capability and downstream standing

The shared ledger accounts all 20 deep and six normalized M1 capabilities.
A broad authored capability may remain **partial** while its scoped numeric
operations are verified. In particular, a kernel test does not complete a
played instrument or a general activity/psychophysical interpretation.

The explicit residuals are: general activity-based excess/deficiency and
eight-determination inference (C10); representation-trajectory TDA (C15);
continuous audio/material/rendering and the played/Jankó instrument (C16 and
render-facing field rows); and Agency, persistence, source Recognition and
whole-product Return in their existing native owners (C17–C19). Arbitrary scalar
Ananda-cell-to-musical-relation inference is not substituted for the implemented
actual-walk bridge. These are preserved capabilities, with native-operation
dependencies and existing owner/programme dispositions, not undiscovered holes
or extra completion claims hidden behind a single green flag.

K8/#132 consumes the source-backed carrier and stable C/Rust boundary. K9/#133
composes the full deep instrument and M1/M2/M3 shared event. This K5 acceptance
completes the coordinate-backed **kernel contract**, not those downstream
embodiment or experiential acceptances. No M2/M3 runtime, registry/schema,
canonical Agent name or source-coordinate hierarchy is changed.

## Reproduction and evidence

`cargo test -p ql-mef --test m1_engine --test m1_state --locked` executes the
retained-source comparisons, actual C/Rust observations and state/music tests.
`bash scripts/test-m1-engine.sh` executes native ASan/UBSan and an installed
C++17 consumer. `scripts/generate-m1-source.py` checks exact generated literals;
`scripts/check-m1-literals.py` compares actual C source readings to the CSV.
The old numeric/literal comparison is retained by `scripts/check-m1-source.py`.

The ledger evidence receipt is `docs/kernel-rebuild/m1-engine-acceptance-v1.json`.
Its exact input locks and observation scope are checked by the M1 acceptance
regressions; CI independently executes the actual operations. The earlier
candidate receipt remains historical rather than being silently relabelled.
The K4 census now preserves reviewed vertical rows during refresh while still
regenerating its independent discovery reports. A replay regression protects
both reviewed coordinate rows and source-matrix assessments. No shared schema
change or competing ledger is needed.

## Parent/deep runtime handoff

This is the M1 producer consumed by the existing
`kernel-rebuild/PARENT-SURFACES-INTEGRATION.md` plan, not an additional surface
or renderer. `config.revision` is the canonical state generation; `clock.cycle`
and `clock.tick12` are its exact logical effective time. One immutable snapshot
carries the event, source subject, selected coordinate, harmonic configuration,
source-return and registry revisions together. K8 relates this logical time to
its integration/audio/render clock explicitly; K5 does not invent a UTC
observation or audio-device time. C++ interpolates that generation while domain
changes return through the revision-checked Rust owner. Personal/Cosmic and
deep M1′ consumers keep the same event and subject rather than starting another
clock or generating a renderer-local ontology.


### M1 → M2 reception: lens encoding, source subject and time

`crates/ql-mef/tests/m1_m2_parent_join.rs` executes the accepted M1 engine and
M2 joint-condition producer together for Cosmic, Personal and deep M1 callers.
It uses both musical bases and all twelve native lens identities. The actual
Ananda traversal supplies its source-provenanced ratio, and the same M1 owner
supplies event/generation, effective tick/degree and active lens/local position.
M2 continues to own Vimarśā and the correspondence source paths. The test does
not manufacture colour from pitch, simulate a physical resonator, or treat
fixture amplitudes as observed excitation. Stale incoming M1 stamps fail.

Native M1 `lens12` is the **interleaved `LensId::ALL` slot**. M2's retained C
MEF table uses **grouped** direct/conjugate rows. Compose through the existing
`Reading72::from_sublens` / `mef_sublens` bridge; do not copy a numeric M1 slot
into the C table index. The earlier grouped-order wording in this document was
incorrect. Native code and lens identities are unchanged; the joined regression
checks every forward/inverse mapping as well as actual M2 execution.

`config.subject_coordinate` remains M1's source/instance coordinate, not a
Nara identity or a human Day. The Personal host keeps its receiving subject,
identity revision and authorized Day/NOW/Flow source handles separately and
relates them to the same event/generation. Passing those refs does not authorize
protected-source disclosure or turn the kernel into their source owner. K10
supplies Nara's actual constitution; K8 applies it to the existing field.

M1 cycles/revisions are full-width unsigned 64-bit decimal strings. The current
M2 JSON `EventIdentity.profile_generation` is a numeric value restricted to
`2^53-1`. A consumer must reject unsupported generations, not truncate or reset
them. The joined fixture keeps a cycle above that threshold exact in M1 and
separately tests rejection of an unsafe M2 generation even when both input
stamps agree. Logical kernel time remains distinct from UTC/provider time and
from an owner's Day/NOW identity. These are headless producer tests, not a new
Agent, map, parent surface or installed experiential acceptance claim.
