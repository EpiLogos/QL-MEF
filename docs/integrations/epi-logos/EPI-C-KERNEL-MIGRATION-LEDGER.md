# Epi C Kernel Migration Ledger

This ledger records **movement of implementation and evidence**. It does not silently reassign semantic authority.

Statuses used here:

- `reference-frozen` — exact historical source identity is locked;
- `reference-characterized` — the frozen translation unit has an observed strict-C11 build/dependency/state record;
- `reference-build-blocked` — a specific source/dependency assumption prevents the untouched translation unit from compiling in the characterized environment;
- `parity-pending` — native/promoted implementation must not replace the reference path yet;
- `parity-pass` — the declared finite/input domain and comparison rule passed against the locked reference;
- `native-first-tranche` — generalized C law now exists outside the frozen vendor tree, without implying a consumer flip;
- `classification-pending` — source contains mixed data/law/semantics and must be separated before promotion;
- `stays-epi-semantic` — semantic composition remains Epi-owned even if lower primitives move;
- `secondary-witness` — useful later implementation evidence, not target authority.

| Source / concern | What it is now | Intended disposition | Current state | Required evidence before replacement |
|---|---|---|---|---|
| `include/ontology.h` | shared Epi coordinate / ontology structures | classify structural mechanics vs Epi semantic identity; extract only generalized mechanics | reference-frozen / classification-pending | structural identity fixtures + ownership review |
| `src/arena.c`, `include/arena.h` | coordinate/tensor arena materialization | preserve C; decide whether generic allocation mechanics belong below Epi semantics | reference-frozen / reference-characterized | allocation/lifetime tests + ownership case |
| `src/families.c` | family/mirror instantiation | preserve first; separate generalized coordinate wiring from Epi family meaning | reference-frozen / reference-characterized / classification-pending | structural parity over canonical family graph |
| `src/pointer_web.c`, `include/pointer_web.h` | pointer/family/holographic coordinate web | likely shared structural candidate after provenance classification | reference-frozen / reference-characterized / parity-pending | exhaustive topology/crosslink fixtures where finite |
| `src/engine.c`, `include/engine.h` | orchestration over existing C domains | keep computation in C; reduce hidden orchestration as stable module APIs emerge | reference-frozen / reference-characterized / classification-pending | command/operation equivalence |
| `src/kernel.c`, `include/kernel.h` | bioquaternionic / epogdoon tick kernel | source oracle for first generalized harmonic/tick/resonance mechanics; retain deeper quaternion/energy reference until separately promoted | reference-frozen / reference-characterized / historical kernel tests 8/8 / first-tranche parity-pass | separate tolerance + invariant design before deeper quaternion/energy promotion |
| `src/psychoid_numbers.c`, `include/psychoid_numbers.h` | Psychoid/number computation and CF data hub | preserve; classify general numeric law vs Epi interpretation and state | reference-frozen / reference-characterized / classification-pending | exhaustive finite numeric fixtures + source/state authority |
| `src/qv_data.c` | Quintessential View managed data projection | preserve values/provenance; determine authored source vs generated projection | reference-frozen / reference-characterized / classification-pending | value/hash parity + source authority chain |
| `include/vak.h` | existing VĀK C contract | preserve as Epi/formal source evidence; align with accepted VĀK grammar without inventing replacement | reference-frozen / classification-pending | field/grammar roundtrips + provenance |
| `src/m0.c`, `include/m0.h` | Anuttara/M0 deep computation and data | rebase onto promoted primitives one edge at a time; M0 semantics remain Epi-owned | reference-frozen / reference-characterized / stays-epi-semantic; explicit Siva stubs observed | operation/data parity by declared capability |
| `src/m1.c`, `include/m1.h` | Paramasiva/M1 matrices, quaternion/Spanda/QL-related computation | consume generalized ring/position primitives where a stable owner edge is justified; do not translate wholesale | reference-frozen / reference-characterized / stays-epi-semantic; ring helpers covered by first-tranche parity-pass | consumer-edge parity + resolution of #60 before unifying ring naming/projections |
| `src/m2.c`, `include/m2.h` | Parashakti/M2 72-fold, lens/tattva/correspondence body | split generalized finite mechanics from Epi interpretation | reference-frozen / reference-characterized / classification-pending / stays-epi-semantic; 36 relation addressing is native structural law only | full relevant 72-fold/lens fixtures; authored-data authority |
| `src/m3.c`, `include/m3.h` | Mahamaya/M3 pair, trigram/hexagram/codon computation | rebase shared six-bit state mechanics; semantics stay Epi | reference-frozen / reference-build-blocked by non-constant C11 static assertions / stays-epi-semantic; complement + line-change first-tranche parity-pass | source-build disposition + exhaustive parity for each further M3 capability |
| `src/m3_clock_lut.c` | generated clock/rotation lookup body | preserve exact values; do not change representation until generator relation is reproducible | reference-frozen / reference-build-blocked transitively by `m3.h` / classification-pending; declared generator absent from locked tree | recover `tools/build_clock_degree_lut.py` or make explicit new authority decision; exact value/hash parity |
| `src/m4.c`, `include/m4.h` | Nara/M4 computation with incomplete areas | preserve implemented computation; do not fossilize stubs as canon | reference-frozen / reference-build-blocked by unavailable `blake3.h` / stays-epi-semantic; lens/protocol/alchemy/random fallback stubs observed | dependency/build disposition + implemented-vs-stub inventory + targeted parity |
| `src/m5.c`, `include/m5.h` | Epii/M5 Logos/QV guarded return | preserve formal computation; human canon mutation authority remains distinct | reference-frozen / reference-characterized / stays-epi-semantic | operation parity + authority boundary tests |
| `src/main.c` | historical executable entry | source evidence for command reach, not the desired final CLI UX | reference-frozen / reference-build-blocked transitively by `m3.h` / classification-pending | command inventory after reference build compatibility is explicit; no UI inheritance assumption |
| `c/include/ql/primitive.h`, `c/src/primitive.c` | first QL-MEF-native generalized C floor, API `ql-c/primitive 0.1.0` | stable C owner for promoted finite position/ring/relation/state/resonance/harmonic law | native-first-tranche / parity-pass on declared domains | owner review + real Epi consumer-edge proof before declaring duplicate historical helper replaceable |
| `epi` CLI | desired human/machine computational front door | thin Epi-owned command/query/serialization/provenance layer over stable C APIs | design commitment / implementation pending (#10) | CLI calls accepted library paths; no duplicate math/domain law |
| `portal-core` | later Rust projections/types/consumer code | mine selectively as secondary witness; do not promote wholesale | secondary-witness | per-capability provenance and parity only |
| QL-MEF Rust crates | current accepted QL/MEF product surface | may wrap or interoperate with stable C computational APIs where justified; no default C→Rust rewrite | implemented current product; `cargo test --workspace --locked` remains green alongside C tranche | explicit boundary if/when cross-language consumption is actually required |

## First R1/R2 evidence tranche

The detailed executable map is `EPI-C-KERNEL-REFERENCE-MAP.md`. Machine-readable evidence is regenerated by `.github/workflows/epi-c-r1-r2.yml`.

The first successful complete run (`32198410273`) established:

- 11/15 frozen source translation units compile independently under strict C11;
- `m3.c`, `m3_clock_lut.c`, `m4.c`, and `main.c` have explicit preserved build barriers rather than an invented aggregate library success;
- the unchanged locked historical kernel test rebuilds against the frozen QL-MEF source and passes 8/8;
- native `ql-c/primitive 0.1.0` passes declared parity for 6-position inversion, the complete `uint8_t` ring input space, all 64 six-bit states, all `64 x 6` line changes, all 72 resonance addresses, all six tritone-square inputs, five exact harmonic ratios, and the complete `uint8_t` epogdoon sub-tick input space;
- `6 x 6 = 36` relation addressing is proven as a native bijective structural law, but is **not** called reference replacement because the frozen source has no single generic callable relation-index oracle;
- the ring stage/projection and ascent/descent disagreement is preserved and tracked in #60 rather than normalized;
- existing QL-MEF Rust workspace tests remain green without introducing unsafe C FFI into `ql-core`.

This advances R1/R2 for the **first promoted primitive tranche**. It does not claim that R1 is complete for the full Epi corpus or that any M module has been migrated wholesale.

## Programme gates

### R0 — closed by PR #58

- exact C reference revision and tree identities locked;
- headers and source present unchanged under the reference path;
- historical test tree addressable by locked revision;
- repeatable kernel source-identity and smoke path exists;
- no native implementation was claimed at the R0 boundary.

### R1 — partially closed for first primitive tranche

For the promoted scope:

- reference operations are deterministically invocable;
- comparison evidence is versioned (`ql-mef.epi-c-parity/v1`);
- finite domains are exhausted where applicable;
- comparison modes are explicit;
- provenance identifies reference and native sides;
- historical source tests are rebuilt, not copied as stale binaries.

For the **full corpus**, R1 remains open because the untouched aggregate executable is blocked by characterized M3/M4 build facts and many deeper operations do not yet have exhaustive parity contracts.

### R2 — first native tranche implemented, consumer proof still pending

Promoted generalized law exists under `c/` and is not a wrapper around the vendor functions. The next proof is one safe Epi C consumer edge after the QL-MEF owner contract is accepted. Until then the reference duplicate remains evidence, not dead code.

### R3/#55 handoff

Do not normalize source/data representations merely because the first primitive floor is stable. Recover authored authority, derived/generator relations, mutable state and provisional/research status first. In particular, the M3 clock LUT's declared generator is absent from the locked source tree and the QV projection's update path must be traced before representation changes.

### R4/#56 handoff

Rebase one low-level consumer edge at a time in dependency order. Do not turn a successful helper migration into a flag-day M0→M5 rewrite.
