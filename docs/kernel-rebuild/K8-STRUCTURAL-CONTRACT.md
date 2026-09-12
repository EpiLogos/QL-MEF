# K8 current structure and coupled-clock native contract

Status: repository implementation, first K8.0 increment. This is not closure of
K8.0–K8.3 or a live graph/planetary/retained-renderer/installed-desktop receipt.
Authority: [apertures and centre](APERTURES-AND-CLOCK-CENTRE.md),
[full pre-K8 lock](PRE-K8-AGENT-WORLD-LOCK.md), issue #132. Parallel #94,
#134 and #133 consume these identities rather than generating another core.

## One source, explicitly versioned projections

`c/registry/promotions/k8-apertures-v1.json` holds the reviewed source amendment,
its exact authorial document blob and revision, and all adopted allocations.
`scripts/k8-structure.py` first reproduces the accepted M compiler floor and then
applies that amendment through the same compiler. No existing source ID,
parentage, record, relation or binding is rewritten. Only ancestor child/subtree
summaries change. The old 1,876-node / 21,083-relation snapshot and its accepted
M1–M3 proofs remain available at their original registry revision.

The current `ql.m-tree/v2` projection has **1,903 nodes, 21,161 relations**: 27
new descendants and 78 typed relationships. Its reviewed lineage is explicit.
The promotion's source records point to the QL-MEF authorial document, not to
the inherited C-Experiments commit. Generated tables/manifests are build outputs;
Git retains the amendment and a deterministic summary receipt. Install includes
both the accepted v1 and current v2 manifestations. This is version compatibility
over one source/compiler, not two independently editable structural registries.

C consumers include `<ql/m_tree_live.h>` and link `libql-mef-c.a`. The
`ql_m_live_*` calls have the same immutable, process-lifetime ownership as the
accepted `ql_m_*` API. Use `ql_m_live_source_origin_at(file_index)` for each
record's actual repository/revision. The corpus-level source revision is not a
substitute for that per-file origin. `ql_m_live_accepts_base` admits only the
accepted floor or this generated current revision; it is not authorization to
apply an Action, or a claim that arbitrary incoming source data is trusted.

## M2 aperture identities

`<ql/m2_aperture.h>` supplies 18 descriptors. Ordinals 0–15 retain the existing
native ascending divisor order, followed by void (16) and Fibonacci (17). Each
static descriptor identifies its exact M2 coordinate, pair container and
reciprocal coordinate, division and segment count, and native ring orientation.
Eight containers are not extra apertures. Ground/void phase positions are not
extra apertures either. The additional descriptors carry no static native index.

`ql_m2_void_antipode` is the sixteen-position ring's `p+8` operation. Static
reciprocity is separately represented by `p <-> 15-p`. Neither operation rewrites
MEF lens identities, elemental positions or the retained M2 coefficient arrays.
No proposed display arrangement changes native ordering.

## Two independent lifted phases

`<ql/coupled_clock.h>`, contract `ql.coupled-clock/v1`, holds inscription and
lensing as integer turn winding plus 0–719 half-degree position. The 720-degree
double-cover projection is explicit; a static aperture selection is not a phase.
A fractional-rate trajectory additionally retains an exact Euclidean remainder
for each axis. This lets partitioned advances agree with one combined advance,
including reversal, without discarding sub-half-degree state.

`ql_clock_set_axis`, `ql_clock_set_trajectory` and `ql_clock_advance` require the
expected clock generation. They are transactional, including aliased input and
output; stale input, range errors and overflow leave output untouched. A changed
trajectory starts at the same lifted axes and deliberately clears the preceding
trajectory's fractional residues. Rates are signed integers bounded by 1,000,000;
the common denominator is 1–1,000,000, and each driver step is at most 1,000,000,000
half-degree units in either direction. Logical driver increments are not UTC,
audio-device time or an ephemeris observation time.

Fibonacci, elemental and void origins remain separate, nonzero state. The exact
quanta are 12, 40 and 45 half-degrees. Pairwise closures are 120, 180 and 360
half-degrees (6/4/2 per turn); void/elemental is 9/8. Alignment with current
origins can be absent. `ql_m2_grid_alignment` returns `NO_ALIGNMENT` rather than
snapping origins into an invented coincidence. `ql_clock_field` resolves M3-0;
`ql_clock_centre` resolves the existing **M3-5-5/0** and retains all 360 degree
children. No degree/return marker becomes an alias for this centre.

These laws sit in C. The runtime composition boundary must still retain complete
accepted M1/M2/M3 frames, original bases, receiving subject and separate temporal
handles; a clock-only value is not the complete continuous event. In particular,
M1's subject coordinate is not Nara identity, and M2's distributed modal/form
potential must not be reduced to its compact condition.

## Reproduction and integration boundary

```sh
python3 scripts/k8-structure.py --check --out target/k8-structure
bash scripts/test-k8-native.sh
```

The native suite checks every old node/relation, all new aperture identities and
source origins, exact centre continuity, rational phase partition/reversal,
overflow/stale/invalid transactions and nonzero grid alignment. It runs ASan and
UBSan and compiles a C++17 consumer using **only installed headers/library**.
It also checks the original M ledger without retargeting historical proofs.

The local graph's availability/application and rollback, shared rooted/property
bindings, full current ledger/matrix reconciliation, joined Rust producer,
planetary observation, continuous physical runtime and retained particle receiver
have separate acceptance. They are not implied by successful structural lookup
or the installed clock consumer. K8 remains the shared integration writer;
#94 owns its full source/property/Vāk work, #134 Personal inputs, and #133 the
same-event interface. Owner-machine desktop harmonisation is later, not a reason
to defer these repository implementations.

## Rust and current-source evidence

`ql_mef::m_tree::native_current_m_registry()` consumes the very same generated
v2 manifest; `native_m_registry()` remains the accepted v1 engine source view.
The current parser retains source lineage and per-file repository/revision.
`k8_structure` executes the actual native C descriptor probe and compares every
node, record, relation, file and binding with Rust, including promotion origins.
The ratified source is retained under `c/registry/promotions/sources/` by its Git
blob identity, so later documentation edits cannot rewrite historical authority.

`scripts/k8-census.py --check` scans the complete current C/Rust source field,
joins accepted bindings, retains old unresolved constructs, rejects new unowned
constructs and verifies the new public C APIs against the built static library.
Reviewed K8 module dispositions are in `c/registry/promotions/k8-bindings-v1.json`.
The full inventory is emitted under `target/m-ledger/current/` with a checked-in
digest receipt. It is current liveness/binding evidence over the same ledger,
not a second readiness ledger and not an automatic promotion of old orphans.
K4 workbooks and their reviewed assessments are not regenerated or restamped.

The original K6/K7 proof artifacts remain byte-identical. Their original build
Makefile is retained in `fixtures/kernel/history/pre-k8-Makefile`. Each current
engine acceptance must execute its full native tests first. The ledger checks
then require the same proof-input set, the same checks, this exact Git head and
fresh current input hashes. `scripts/k8-preservation.py` admits only the reviewed
packaging change named by `k8-build-lineage-v1.json`; any changed numerical/source
input, historical proof, unreviewed build edit, partial/stale execution or changed
post-execution input fails. The new `k8-preservation.json` stays beside that run's
native receipt. It does not pretend the old proof ran on the new Makefile.
