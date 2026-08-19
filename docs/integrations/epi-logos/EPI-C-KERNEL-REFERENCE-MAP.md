# Epi C Kernel Reference Map — R1/R2

**Status:** observed migration evidence for QL-MEF #53/#54  
**Frozen source authority:** `EpiLogos/Epi-Logos-C-Experiments@daa660cbc1b8c5da83828698665a753852cb0287`  
**Frozen path:** `vendor/epi-kernel/reference/`  
**R0 merge:** QL-MEF PR #58, `d030823584d35eca4e27df60e6385d67a531026b`  
**R1/R2 work:** QL-MEF PR #59  
**First complete evidence run:** GitHub Actions `32198410273`

This document records **implementation fact and observed build/parity result**. It does not elevate every historical C representation into QL canon, and it does not move Epi semantic ownership merely because a lower computation is reusable.

The governing ownership test remains:

> Can the operation be stated and used coherently without knowing that Anuttara, Paramasiva, Parashakti, Mahamaya, Nara or Epii exist?

Only operations that survive that test are candidates for the generalized QL-MEF C floor.

## 1. What R1 actually proved about the frozen oracle

The R0 smoke was a real but narrow `kernel.c` execution path. R1 now characterizes every frozen source translation unit independently under:

```text
cc -std=c11 -Wall -Wextra -Ivendor/epi-kernel/reference/include
```

The characterization is intentionally observational. A historical source failure does not fail the characterization job and is never repaired under `vendor/epi-kernel/reference/`. Per-file compiler logs, dependency files, symbols, cross-translation-unit edges, generated/provisional markers and the aggregate-link disposition are uploaded as CI evidence.

At the first complete run, **11 of 15** translation units compiled independently and **4 of 15** did not. Because not every translation unit compiles, the aggregate historical executable link is honestly recorded as `not-attempted` rather than approximated from a subset.

### 1.1 Translation-unit build map

| Translation unit | Strict C11 compile | Direct historical dependencies / returned reality | Migration reading |
|---|---:|---|---|
| `arena.c` | yes | libc allocation/memory only | buildable infrastructure witness; generalized ownership still requires lifetime/API review |
| `engine.c` | yes | `Psychoid_0..5` from `psychoid_numbers.c` | Epi orchestration; not a primitive floor |
| `families.c` | yes | `arena_alloc`; `Psychoid_0..5` | mixed structural/semantic family wiring |
| `kernel.c` | yes | libm `logf`, `sqrtf`; no project TU dependency | strongest independent historical computational seam |
| `m0.c` | yes | `CF_TABLE`; `vak_register_handler`; libc | Epi M0 body; includes explicit Siva operator stubs |
| `m1.c` | yes | `CF_TABLE`, `Psychoid_0..5`; libc | Epi M1 body containing reusable low-level ring/QL mechanics plus Epi matrices/data |
| `m2.c` | yes | `CF_TABLE`; libc | Epi M2 body containing 36/64/72 structures and extensive authored correspondences |
| `m3.c` | **no** | four non-constant strict-C11 `_Static_assert` expressions | source-language/build defect; preserve reference and use external compatibility only for isolated inline parity |
| `m3_clock_lut.c` | **no** | transitively blocked by the two `m3.h` static assertions | generated LUT; representation must not be normalized before generator provenance is recovered |
| `m4.c` | **no** | `blake3.h` unavailable from frozen tree/build | missing external dependency plus extensive explicit stubs/provisional data |
| `m5.c` | yes | `CF_TABLE`, `Psychoid_0/4/5`; libc | Epi M5 body; semantic return remains Epi-owned |
| `main.c` | **no** | transitively blocked by `m3.h`; includes all M headers | historical command-reach witness, not target CLI architecture |
| `pointer_web.c` | yes | `CF_TABLE`, `Psychoid_0..5`, `Psychoid_Hash` | promising structural web, but still coupled to Epi source identities |
| `psychoid_numbers.c` | yes | libc | central provider of Psychoid/CF globals; mixed authored data and provisional execution stubs |
| `qv_data.c` | yes | none beyond included Epi declarations | managed/generated-style Epi data projection; values stay Epi-owned pending #55 source-authority audit |

### 1.2 Exact root build barriers

The four failed translation units reduce to two root barrier families.

**Strict-C11 non-constant assertions.** `m3.h` indexes the file-scope `static const NUCLEOTIDE_ICHING_VALUE[]` array in two `_Static_assert`s. Strict C11 does not treat those array reads as integer constant expressions. `m3.c` additionally performs two `_Static_assert`s over `M3_RNA_FUNCTIONAL_MASK` / `M3_RNA_DARK_MASK` objects that are likewise not constant expressions. This directly blocks `m3.c`, and the header assertions transitively block `m3_clock_lut.c` and `main.c`.

R1 does **not** alter these assertions. For the narrow parity of the historical inline `m3_complement()` and `m3_line_change()` operations, `migration/epi-kernel/compat/m3-reference.h` suppresses `_Static_assert` only while the frozen M3 header is parsed. That compatibility file is migration-test scaffolding and is explicitly forbidden as a production QL-MEF include.

**Unavailable BLAKE3 header.** `m4.c` unconditionally includes `blake3.h`, but the locked source tree/build surface does not provide it. That is presently an unavailable dependency/build-glue fact. It is not evidence that M4 should move, and it is not repaired in the oracle.

## 2. Actual cross-translation-unit dependency shape

Object-symbol analysis of the 11 compiling units gives the following project-internal edges:

```text
arena.c
  ↑
  └── families.c

psychoid_numbers.c
  ↑
  ├── engine.c              [Psychoid_0..5]
  ├── families.c            [Psychoid_0..5]
  ├── m0.c                  [CF_TABLE]
  ├── m1.c                  [CF_TABLE, Psychoid_0..5]
  ├── m2.c                  [CF_TABLE]
  ├── m5.c                  [CF_TABLE, Psychoid_0/4/5]
  └── pointer_web.c         [CF_TABLE, Psychoid_0..5, Psychoid_Hash]

engine.c
  ↑
  └── m0.c                  [vak_register_handler]

kernel.c
  └── libm only             [logf, sqrtf]
```

This returned graph matters for build order. `kernel.c` is not merely early because its filename says “kernel”; it is in fact the least semantically coupled executable computational seam in the frozen body. Conversely, `psychoid_numbers.c` is a real dependency hub and cannot be casually promoted merely because many modules depend upon it: the globals it supplies carry Epi semantic identity and mixed data/state concerns.

The characterization deliberately records libc/compiler symbols separately from project-internal edges. Their presence is not interpreted as a domain dependency.

## 3. Global/static state and data classification

R1 records exported and file-local data symbols from successfully compiled objects. Linker section class is **evidence about representation**, not a semantic classification by itself: read-only `.rodata` does not prove canonical authority, and writable `.data` does not prove runtime mutability.

The first #55-oriented classification is therefore intentionally conservative.

### A — canonical authored/source data

Material whose values are meaningful in Epi's authored semantic world remains Epi-owned even when stored as plain C tables. Current candidates include M0 semantic LUTs, M1 Ananda/DR structures, M2 archetype/MEF/tattva/decan/planet/mantra/maqam correspondences, Psychoid/CF definitions, and Quintessential View material.

**R1 disposition:** preserve values exactly; do not move these tables merely because their consumers may later call generalized primitives. #55 must follow their actual source authority before deciding canonical storage form.

### B — deterministic derived data / law

The clearest first examples are small finite mechanics which can be recomputed from a lower law rather than treated as authored correspondence:

- six-position inversion `p -> 5 - p`;
- twelve-ring wrapping and two explicit six-position projections;
- `6 x 6 = 36` relation addressing;
- six-bit complement and one-line change over the `2^6 = 64` state field;
- `6 x 2 x 6 = 72` resonance addressing;
- tritone-square pairing over the six lens indices;
- the already-proven harmonic ratios and twelve-tick ratio projection.

These are the contents of the first native QL-MEF C tranche. Their promotion does **not** promote M1/M2/M3 semantic datasets with them.

### C — generated optimization / LUT

`m3_clock_lut.c` identifies itself as:

```text
AUTO-GENERATED by tools/build_clock_degree_lut.py
Do not edit by hand
```

A recursive inspection of the locked Epi revision does **not** contain `tools/build_clock_degree_lut.py`. The LUT is therefore presently **not reproducible from the locked source revision as preserved**. Its values remain reference evidence, but representation normalization is blocked until the generator relation is recovered or a new explicit source-of-truth decision is made.

`qv_data.c` also says “Do not edit manually” and routes updates through an `epi core knowing ... --update` command. That is evidence of a managed/generated projection relation, but not enough by itself to classify the underlying QV text as derived rather than authored. #55 must recover that authority chain.

### D — mutable runtime / process state

The object map exposes writable/global state such as `QL_STACK`, `M1_M0_CROSSLINK`, `QL_FLOWERING`, `SPANDA_CF_SUBSTAGE_LUT`, `SPANDA_COMPILER_PASSES`, `CF_TABLE`, Psychoid objects and Weave objects. Some are initialized tables and some are genuinely mutable process state; linker writability alone cannot decide which.

**R1 disposition:** keep them out of the first native primitive ABI. #55 must distinguish immutable source data accidentally emitted writable from state whose mutation is semantically required.

### E — research / provisional material

Explicit source markers include:

- M0 Siva operator stubs;
- M3 `56 valid + 8 evolutionary gaps`, including `STATUS_PROVISIONAL` handling;
- M4 six lens vtable stubs;
- M4 36 protocol cards explicitly described as stub values whose real values should come from a dataset;
- M4 seven alchemical operation stubs;
- M4 non-macOS sacred-random fallback which zero-fills and explicitly says it is not cryptographic;
- Psychoid context-frame root execute stubs.

These are implementation facts about the historical corpus. They must not become generalized truth merely because the C migration gives them a new home.

## 4. Platform/compiler/build assumptions discovered

The first characterized environment is GitHub Actions Ubuntu 24.04 with the system C compiler under strict C11.

Observed assumptions include:

- C11 allocation (`aligned_alloc`) in arena code;
- libm for the independent kernel (`logf`, `sqrtf`);
- strict-C11-invalid static assertions in M3 as described above;
- unavailable `blake3.h` for M4;
- an explicit `__APPLE__` branch in M4 using `arc4random_buf`, with a non-macOS zero-fill stub fallback;
- historical CLI code in module bodies and `main.c`, which proves command reach but is not inherited as the final `epi` UX.

No source file is changed merely to erase these assumptions from the reference record.

## 5. Historical tests brought forward deliberately

The workflow checks out the **locked historical Epi revision** separately and compiles its unchanged `test/engine/test_kernel.c` against the frozen QL-MEF `kernel.c`.

On Actions run `32198410273`, all historical kernel tests passed:

```text
Kernel Math: 8/8 passed
```

The covered historical surface includes epogdoon constants, bioquaternion normalization, tiny axes, slash flip, quaternion distance, 72-fold resonance/tritone emphasis, energy decomposition, and twelve tick / eight-element projection.

R1 does not copy stale historical binaries as evidence. Tests are addressed from the locked source revision and rebuilt in CI.

## 6. First native QL-MEF C primitive tranche

The first native implementation lives outside the vendor tree:

```text
c/include/ql/primitive.h
c/src/primitive.c
```

API identity:

```text
ql-c/primitive 0.1.0
```

The implementation reports explicit source provenance back to the locked Epi revision. It contains no Epi Agent names, M-domain dispatch, VAK/Nara semantics, authored correspondence datasets, CLI logic, Rust FFI or product/session machinery.

Promoted generalized operations are:

```text
six-position inversion
12-ring wrap / half / base-position / traversal-stage
6x6 relation index
64-state complement
64-state line change
72-space resonance index
tritone square identity
five harmonic ratios
epogdoon tick projection
```

The native tick type is deliberately named `QL_Primitive_Tick`; the historical Epi headers already own `QL_Tick` as a `uint8_t` ring type. R1 discovered that collision in CI and changed the native ABI rather than overwriting Epi vocabulary.

## 7. Exhaustive parity evidence

`migration/epi-kernel/parity-first-tranche.c` dual-runs the frozen source and native implementation wherever a callable historical operation exists. `scripts/test-epi-c-parity.sh` emits `ql-mef.epi-c-parity/v1` JSON with source authority, revision, operation, inputs/domain, comparison rule, result, readiness and observation time.

The first successful run proves:

| Operation | Coverage | Rule | Result |
|---|---:|---|---|
| position inversion | all 6 + invalid boundary | exact + involution | pass |
| ring wrap/half/base/stage | all 256 `uint8_t` inputs | exact vs M1 helpers/macros | pass |
| relation index | all `6 x 6 = 36` | bijection/range invariant; no frozen generic callable index | pass, structural-native only |
| state complement | all 64 | exact vs `m3_complement` + involution | pass |
| line change | all `64 x 6 = 384` | exact vs `m3_line_change` + involution | pass |
| resonance index | all `6 x 2 x 6 = 72` + invalid boundaries | exact vs frozen kernel | pass |
| tritone square | all 6 + invalid | exact vs frozen kernel | pass |
| harmonic ratios | 5 | exact IEEE-754 equality for identical rational expressions | pass |
| epogdoon tick | all 256 `uint8_t` sub-ticks at fixed cycle 17 | exact cycle/wrap/base-position/ratio vs frozen kernel | pass |

No broad epsilon is used. This tranche's promoted floating values are identical rational expressions and therefore compared exactly. Quaternion/topology calculations outside this tranche remain reference-only and require their own tolerance plus structural-invariant design before promotion.

## 8. Discrepancies preserved rather than corrected

R1 exposed two linked historical ring semantics that must not be silently unified.

### 8.1 M1 traversal stage vs kernel position projection

Historical M1 `ql_get_stage()` maps inverted-half ticks:

```text
6  7  8  9  10 11
5  4  3  2   1  0
```

Historical `kernel_tick_from_epogdoon().position6` maps the same ticks by modulo six:

```text
6  7  8  9  10 11
0  1  2  3   4  5
```

The native type therefore carries both `traversal_stage` and `base_position`. Neither is declared the correction of the other.

### 8.2 Ascent/descent naming conflict

Historical M1 names ticks `0..5` ascending and `6..11` descending. Historical kernel `Kernel_Phase` labels the first half `DESCENT` and second half `ASCENT`.

The native primitive therefore uses the neutral names `QL_RING_DIRECT_HALF` and `QL_RING_INVERTED_HALF`. Historical phase labels appear only in parity mapping until their intended relation is explicitly decided.

QL-MEF #60 owns this research/formal decision. It is not a reason to stall preservation or the separate Nara product vertical.

## 9. What has and has not moved

**Moved into generalized QL-MEF C:** only the first finite low-level law listed in section 6, with parity/provenance.

**Not moved:** Epi Agents; M0-M5 semantic identity; Nara; Mahamaya; Epii; VAK grammar; Psychoid/CF authored meanings; Ananda/DR semantic datasets; M2 authored correspondences; codon/I-Ching interpretation; M4 protocol/lens/alchemy semantics; Quintessential View content; historical CLI dispatch.

The migration result is therefore a dependency-floor extraction, not an ontology migration.

## 10. Returned build order into #55/#56

The evidence revises the naive idea that the next work should be “port M1/M2”. The next dependency order is:

1. **#55 source/data normalization:** recover source authority and reproducibility before changing M3 clock, QV, Ananda/DR, Psychoid/CF or M2 table representation; explicitly recover or replace the missing `build_clock_degree_lut.py` relation.
2. **Reference build closure where useful:** decide how the historical M3 static-assert defects and M4 BLAKE3 dependency are represented in compatibility/build scaffolding, without mutating R0.
3. **#56 one-edge rebases:** consume the promoted primitive floor from individual Epi C call sites only where the new owner contract is stable; leave surrounding M modules materially unchanged.
4. **Further primitive tranches:** only after parity design exists for pointer/family topology, quaternion/energy laws or other candidates. Epi semantic datasets do not move with them.
5. **Epi #10 / #57:** expose stable computational calls through the thin Epi-owned `epi` CLI and flip consumers only after the lower owner contract is accepted.

The first tranche is ready to be reviewed as a generalized C primitive floor. It is **not** evidence that the complete historical corpus is yet a clean standalone library, and it is **not** permission to delete or rewrite the frozen source.
