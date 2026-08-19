# Epi C kernel R4 — holographic reconstitution receipt

Status: implementation/evidence receipt for QL-MEF issue #56, under programme #51.

This tranche migrates the reusable computational substrate of the frozen Epi C kernel into QL-MEF-owned C. It does **not** move Epi's semantic ownership of M identities, named Bimba worlds, guardian identities, M′ products, or authored domain interpretation.

Source authority is the frozen `Body/S/S0/epi-lib` corpus at Epi revision `daa660cbc1b8c5da83828698665a753852cb0287`. The pre-R4 QL base is `3e9fb929f0c34e8b4474eef59ce512b53082fba1`. The accepted scalar floor remains `ql-c/primitive 0.1.0` and is reused rather than copied or reimplemented.

## Old → native movement

| Historical substrate | Native QL-MEF C | Status / reason |
|---|---|---|
| `ontology.h::Coordinate_Family` | `ql/holographic.h::QL_Coordinate_Family` | Migrated with exact `C/P/L/S/T/M/NONE` values. `NONE` remains pre-categorical. |
| `ontology.h::Holographic_Coordinate` | `QL_Holographic_Coordinate` | Reconstituted as the 128-byte seed, retaining family + six-position identity, inversion state, weave state, semantic/source anchor, 6 base links, 6 reflective slots, execution hook, payload. |
| tagged `#`, `.`, `-`, `()` pointer relations | `ql_relation_tag/target/flags/family/position` | Reconstituted with the historical upper-bit contract and source-determined nesting/branching rule. |
| `families.c::FAMILIES[6][6]` and cross-links | `QL_Holographic_Field` | Native 6×6 field; all six parent families remain addressable and cross-linked. |
| historical `cf`/`cs` wiring | native `cf`/`cs` links | Preserved where the frozen initializer actually wires them. `cpf/ct/cp/cfp` remain explicit slots but are not populated here: R4 does not create a second Context Frame system. |
| `BIMBA` / `PRATIBIMBA` | `QL_Bimba` / `QL_Pratibimba` + materialize/source/bedrock | Source ↔ manifestation remains structurally recoverable. Native generic psychoid Bimba carries no Epi-owned semantic payload. |
| P/P′ topology and inversion state | `ql_coordinate_toggle_cover`, `ql_coordinate_conjugate` | Conjugacy preserves the parent family and delegates positional inversion to accepted `ql_position_invert`. P′ is represented as the inverted/Klein cover; L/L′ uses the same family-preserving conjugacy without deriving it from M1. |
| M-family coordinate identity | same universal field/conjugacy | `M1` is `family=M, position=1`; conjugation produces `family=M, position=4`, never a flattened generic index. |
| `kernel.c` harmonic/resonance helpers | accepted `ql-c/primitive` calls | Ratios, 72-fold resonance indexing and tritone mapping reuse the accepted scalar floor. |
| `Kernel_Bioquaternion`, slash flip, resonance emphasis, energy | `ql/kernel.h` + `c/src/kernel.c` | Migrated into QL-owned C with frozen-kernel parity tests. |
| 12-tick kernel mechanics | `QL_Kernel_Tick` | Preserves historical `position6 = tick % 6` while also carrying accepted `base_position` and distinct `traversal_stage`; issue #60 is not resolved by this migration. |
| no independent QL native-C distribution seam | `c/Makefile` static archive/install/package | Adds a deterministic `libql-mef-c.a`, headers, API version and exact build revision metadata so an independent Epi C consumer can pin and verify QL-owned C rather than copy source or rely on a sibling checkout. |

## Structural invariants proved in QL-MEF

`migration/epi-kernel/r4-holographic-kernel-parity.c` compares the promoted kernel against the frozen executable source where direct parity is meaningful and proves structural invariants where the frozen implementation is initializer-based:

- all `C/P/L/S/T/M` family values and `FAMILY_NONE` match;
- the 128-byte coordinate seed and critical offsets match;
- raw psychoids remain `FAMILY_NONE` Bimba;
- the complete 6×6 family field exists with family + position identity;
- base family links retain tagged family and position identity;
- nesting/branching follows the historical source-coordinate rule;
- `cf` and `cs` links retain the frozen initializer behavior without introducing new CF semantics;
- P, L and M conjugation preserves family identity and applies the accepted positional inversion law;
- Bimba → Pratibimba materialisation retains a recoverable source link;
- execution hooks remain callable;
- harmonic ratio/log, 72-fold resonance, tritone squares, bioquaternion normalization, slash-flip conjugation, resonance emphasis, energy decomposition and the complete 256-byte tick input domain compare against the frozen kernel;
- historical kernel `position6/base_position` and M1-style `traversal_stage` stay explicit and non-equated on the inverted half.

`scripts/test-epi-c-r4.sh` also builds and installs only the QL-owned static library, then links an external smoke consumer against the installed prefix. This is the dependency seam the historical Epi C build can pin; source-relative includes or copied QL source are not part of the contract.

## First historical M1 edge — executed

The source identity was already resolved by the existing holographic provenance account:

```text
M1 / #1-4.2 Principle of Inversion
    → formal law p ↦ 5-p
    → ql-c/primitive::ql_position_invert
```

QL-MEF PR #76 publishes the native C seam at immutable commit `a3c33a2944fb2d90111afdf18f2afd6e871043e0`. The paired Epi PR #31 (`agent/ql-mef-r4-m1-inversion-consumer`, current head `da4ac68fe333aef755e976ea7c65f550d9a999b2`) pins that exact revision and switches one real historical runtime read only: `m1.c::m1_cli_dispatch(..., "ql", ...)` now obtains each inverse stage from `ql_position_invert(QL_FLOWERING[i].stage)`.

The Epi-owned `QL_INVERT[6]` and `QL_FLOWERING[].inverse` are deliberately retained as source/parity evidence. The Epi workflow materialises QL into an installed prefix, deletes the QL source checkout, then proves:

```text
QL native C dependency verified: ql-c/primitive 0.1.0
  @ a3c33a2944fb2d90111afdf18f2afd6e871043e0

M1 #1-4.2 -> ql_position_invert parity: PASS
  coverage: all six + involution + invalid boundaries

existing focused M1 regression: 190 passed, 0 failed

nm m1-r4.o:
  U ql_position_invert

nm libql-mef-c.a:
  T ql_position_invert
```

This is the required physical consumer proof: the Epi M1 translation unit imports the native operation and the pinned QL archive supplies it. No copied helper, sibling include path, or Rust wrapper is the implementation edge.

## R5 consumer-edge ledger

R4 intentionally stops after the first M1 specimen. The following are the remaining **evidenced** R5 dependency edges; this list does not promote authored domain data merely because its cardinality resembles a generalized QL structure.

| Epi consumer | Remaining edge into native QL C | R5 constraint |
|---|---|---|
| **M0** | `m0.h` still consumes the historical `ontology.h` coordinate/link/execution contract directly. Rebase the structural HC/link/access edge onto `ql/holographic` (or a parity-preserving Epi boundary adapter) before changing any M0 VM semantics. | `VIMARSA_TABLE`, authored archetype/divine-act/Vāk data, Siva/Shakti semantics remain Epi-owned; no invented scalar replacement. |
| **M1** | After the inversion read, historical ring helpers `RING_WRAP`, `IS_SHADOW_PHASE`, `GET_BASE_QL_POS`, and `ql_get_stage` still sit in M1. Rebase them incrementally to `ql_ring_wrap`, `ql_ring_half`, `ql_ring_base_position`, and `ql_ring_traversal_stage` with dual parity. Quaternion consumers are candidates for `ql/kernel` only where the signatures/parity line up. | Keep `base_position` and `traversal_stage` distinct; do not resolve #60 by choosing one. `QL_FLOWERING`, Spanda, Ananda, and M1 semantic tables stay Epi-owned. |
| **M2** | The M2 body still owns its 72-condition semantic union and inline 12×6 lens routing (`get_mef_condition`, L-family linkage). Rebase only the generalized 6×2×6 address/index mechanics onto `ql_resonance_index` / native family-bearing L coordinates where a concrete call site is switched. | Do not move the M2 72-space semantic payload, Tattvas, decans, planets, elements, or authored lens names into QL just because the native index is 72-fold. |
| **M3** | `m3_line_change(hex,line)` and `m3_complement(hex)` remain direct duplicate 6-bit operations; these are the clean next finite-domain edges to `ql_state6_line_change` and `ql_state6_complement`. | Preserve invalid-line behavior explicitly at the boundary. Do not pull `m3_clock_lut.c` / #55 data authority into this rebase. The existing strict-C11 M3 barrier is a separate source/build fact, not a reason to rewrite M3 here. |
| **M4** | No additional generalized R4 primitive consumer is yet proven safe to flip. M4 still depends transitively on the historical coordinate/M1–M3 body and its own BLAKE3/oracle/lens semantics. | First resolve the concrete structural/kernel call edge encountered in R5; do not treat current stubs or deep Nara/M′ semantics as QL kernel mechanics. |
| **M5** | No additional generalized scalar/kernel duplicate has been proven from the accepted R1/R2 floor. M5 still consumes historical HC/M-family integration paths. | Rebase structural HC/kernel calls only when encountered; preserve Epi Logos/synthesis semantics and do not execute deep M′/D work. |

The R5 order implied by current evidence is therefore: **M1 remaining ring/tick edges and M3 finite 6-bit transforms first; M0/M2 structural/index edges where a real consumer is encountered; M4/M5 only after a concrete reusable kernel dependency is demonstrated.** This keeps R5 incremental rather than turning it into an M0–M5 rewrite.

M3 LUT authority (#55), the tick-projection question (#60), fourfold M→S/S′→M′ work (#74/#75), #73, D, and deep M′ products remain outside R4/R5 primitive rebase unless separately authorized.
