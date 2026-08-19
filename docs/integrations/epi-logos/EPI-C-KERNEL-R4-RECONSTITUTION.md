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

## First M1 edge

The source identity is already resolved by the existing holographic provenance account:

```text
M1 / #1-4.2 Principle of Inversion
    → formal law p ↦ 5-p
    → ql-c/primitive::ql_position_invert
```

R4's QL side deliberately does not redefine that semantic identity. Once this branch has an immutable QL commit, the paired Epi consumer change pins that revision and switches one historical runtime edge only: the M1 `ql` CLI's inverse-stage read. The frozen `QL_INVERT[6]` / `QL_FLOWERING[].inverse` remains reference evidence for before/after all-six parity rather than being deleted or made into a second implementation.

## R5 handoff

R4 does not wholesale rebase M0–M5. After the first M1 inversion specimen, R5 remains the incremental consumer migration of the remaining M0–M5 call edges onto this shared coordinate/kernel body. Those consumers retain Epi semantic ownership; only reusable generalized computation moves behind the QL C API. M3 LUT authority (#55), the tick-projection question (#60), fourfold M→S/S′→M′ work (#74/#75), #73, D, and deep M′ products remain outside this tranche.
