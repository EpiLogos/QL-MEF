# K1 — native C foundation for K2

K1 (#125, PR #144) restores the native C centre on the accepted K0 main
`f97745b4715311e97e44eda5c22488de5eb2455c`. It selectively recovers closed,
unmerged PR #76 at `e5f612653c71c8a9ae74199ef2e8b326e3e9deba`; it does not
merge that old branch or replace current Rust. The machine-readable recovery
and handoff inventory is [native-c-foundation-v1.json](kernel-rebuild/native-c-foundation-v1.json).
The merged PR and #126 handoff comment identify the exact accepted commit;
each native CI artifact also records its actual checkout SHA and source objects.

## What is executable now

`c/include/ql/primitive.h`, `holographic.h`, and `kernel.h` are the public C11
headers. `c/src/primitive.c`, `holographic.c`, and `kernel.c` build
`c/build/libql-mef-c.a`, API **0.1.0**. The shared semantic contract remains
**ql.holographic-kernel-contract/v1, 1.1.0**. Its existing TSV fixture is
unchanged, Git blob `5112349d363a5c01e63418c91880af67c5e60ed1`.

`QL_Kernel_Address` carries family, position and direct/prime face. It admits
`#`, twelve raw position/face addresses, and 72 C/P/L/S/T/M position/face
addresses: 85 distinct semantic addresses. `#` is NONE/255/direct, not an
invented seventh position or a Rust sixfold address. Changing face preserves
position; complement and cross-completion are separate operations.

`ql_kernel_*_address`, `ql_kernel_address_valid` and
`ql_kernel_address_format` construct, validate and disclose those identities.
`ql_holographic_field_init/get/get_const/resolve` expose the 36-node historical
family/position substrate, plus raw/hash bedrock. Direct and prime semantic
addresses can resolve to the same substrate pointer: pointer equality is not
semantic address equality. Source anchoring, materialisation, execution hooks,
P/P-prime topology distinction, and the historical cf/cs pointer web remain.

`ql_kernel_relation_id/resolve` expose the 25 current stable relation identities,
including family same-position, pairing A/B/C, face-crossing, complement,
successor and Mobius Return. `ql_kernel_mef_address` supplies the 72 MEF
lens/local positions. `ql_kernel_context_frame_address` supplies the seven
current Context Frames at all twelve lenses (84 combinations). CF5 remains
`(4.0/1-4.4/5)` and CF6 `(4.5/0)`; no additional Context Frame is introduced.

The six VAK descriptors and five-byte instruction preserve family, index,
branch, position and prime suffix. VAK CF retains the one Context-Frame relation
identity. Parameterised instructions are not reduced to unary relations.
The identity-only CF/nesting/branching/source-provenance relation references
are structural seams, not claims of fully executed recursive operations.
Recovered quaternion, resonance, energy and tick helpers remain covered by
historical parity; they do not constitute an M-coordinate migration.

## Reconciliation and two bounded repairs

Current Rust production sources, the accepted `ql.vak-composition/v1` producer,
its fixtures, and the corrected full `vendor/epi-kernel` body are unchanged.
The full M0–M5 source remains available under `vendor/epi-kernel/reference`,
with source locks and ratified corrections, including M3-COIN-1. It is retained
for actual coordinate porting, not reduced to a stub or replaced with a witness.
The historical source revision remains `daa660cbc1b8c5da83828698665a753852cb0287`.

The recovery's compact address constructor previously narrowed enums before
validation: family or face 256 could become valid zero. The K1 constructor
validates enum-width values first. The new edge regression reproduced 14
failures before the repair, then passes all 97 checks (98 with a requested
compiled-revision check). No valid canonical address or relation changes.

The recovered Makefile could reuse a library compiled with an old source SHA
while installing metadata for a new SHA. A change-sensitive revision stamp is
now an object dependency and the installed metadata is copied from that stamp.
The test deliberately rebuilds in one directory without cleaning between
revision changes, checks compiled and installed revision agreement, checks a
no-op archive, then checks byte-identical repeated packages. The R4 script also
separates clean and build and accepts an explicit source revision for archives.

Several blob IDs advertised in the K0/#125 prose recovery table differ from
the actual objects returned by GitHub at the pinned #76 commit. The machine
inventory records both advertised and observed values. Recovery follows the
observed commit/path objects, independently checked against checkout bytes;
the historical K0 ledger is not rewritten to conceal the discrepancy.

## Build, package and verify

From a clean checkout:

```sh
make -C c all
make -C c install DESTDIR="$PWD/target/c-install" PREFIX=/ql-mef-c
make -C c package
bash scripts/test-epi-c-parity.sh
bash scripts/test-epi-c-r4.sh
bash scripts/test-native-c-foundation.sh
cargo test -p ql-mef --test native_c_foundation -- --nocapture
cargo test --workspace --all-targets --locked
```

Link clients against `libql-mef-c.a` and `-lm`. Install includes all three
headers, the shared contract TSV and API/contract/source-revision text files.
Package output is `c/dist/ql-mef-c-0.1.0-<source-revision>.tar.gz` with its SHA-256
sidecar. Exported source without Git must supply an honest `SOURCE_REVISION`;
a working-tree build is not accepted-commit evidence.

The native workflow tests the exact PR head (and exact main SHA after merge),
not an unnamed historical run. It retains source archive/object inventory,
R4 logs, installed-package smoke, GCC and Clang/ASan/UBSan evidence, package
checks, and `c-rust-parity.tsv`. The current Rust test compiles the actual C
probe and validates **3,343 unique outcomes** against existing Rust APIs and
the explicit count fixture; missing or duplicate outcomes fail. These cover
all 85 addresses, all 25 relation IDs, 2,604 family/position/face relation queries,
25 hash relation queries, 72 MEF addresses, 84 Context-Frame selections and 432
VAK operand combinations, plus family/face/version/descriptor rows. Clang's
sanitised probe must produce the same stream. Existing VAK/MEF/Rust tests run
alongside this, rather than being replaced by historical C parity.

The observed build target is Linux x86-64, C11, GCC/Clang and GNU make/ar/tar.
The historical 128-byte pointer layout assumes 64-bit pointers; tagged links
retain only the low 48 address bits and must be untagged before dereference.
Neither arbitrary-address portability, a C++ embodiment, nor live Neo4j
acceptance is claimed here.

## Exact seam consumed by #126

K2 starts from this merged C library, public headers, contract fixture and
executable parity floor. It should extend native identity with the source-faithful
recursive M registry required by #126, preserving M-whole/M0–M5 roots, asymmetric
paths, `-`, `.`, `/`, provenance and implementation-binding distinctions.
`QL_Kernel_Address` supplies the existing family/position/face identity; the
128-byte pointer substrate is not the place to squeeze recursive path strings
or fabricate a full M tree from 36 slots. Rust must consume the same native
registry rather than create a second authoritative M hierarchy.

K1 does **not** supply `ql.m-tree/v1`, a recursive M registry, full M0–M5 ports,
new M readiness claims, C++/Neo4j embodiment or a replacement VAK composition
producer. Those remain their existing subsequent work. The preserved full M
body is the porting material; the restored library is its current native centre.
