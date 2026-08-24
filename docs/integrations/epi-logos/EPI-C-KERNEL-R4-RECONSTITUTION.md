# Epi C kernel R4 — holographic reconstitution receipt

Status: implementation/evidence receipt for QL-MEF issue #56, under programme #51.

This tranche reconstitutes the reusable historical Epi C coordinate/kernel substrate in QL-MEF-owned C. It does **not** reinterpret the mature inversion/conjugation dynamics as a positional complement rule, and it does not move Epi semantic ownership of M identities, named Bimba worlds, guardian identities, M′ products, or authored domain interpretation.

Source authority is the frozen `Body/S/S0/epi-lib` corpus at Epi revision `daa660cbc1b8c5da83828698665a753852cb0287`. The accepted scalar floor remains `ql-c/primitive 0.1.0` and is reused where it is genuinely scalar law.

## Core correction: coordinate mapping is not conjugation

The R4 kernel now keeps three things distinct:

1. **coordinate identity / labelling** — family + six-position + Bimba/Pratibimba face;
2. **kernel dynamics** — bioquaternion state, slash flip, 12-tick phasing, resonance, harmonic ratios and energy;
3. **historical positional complement** — the separate M1-style `0↔5, 1↔4, 2↔3` relation exposed by the existing scalar helper.

The positional complement is not used to manufacture P/P′ or L/L′.

The mature conjugate-reflection mechanism is the kernel's bioquaternionic operation:

```text
bimba / pratibimba faces
        ↓
q_b / q_p
        ↓
slash flip
        ↓
q -> q*  (scalar preserved, vector sign reversed)
```

That is the operative `0/1 -> 1/0`-type conjugation relation. The coordinate layer labels where a state/aspect sits in the field; it does not replace this dynamics with `p -> 5-p`.

## Old → native movement

| Historical substrate | Native QL-MEF C | R4 disposition |
|---|---|---|
| `ontology.h::Coordinate_Family` | `QL_Coordinate_Family` | Exact `C/P/L/S/T/M/NONE` family identity retained. |
| `ontology.h::Holographic_Coordinate` | `QL_Holographic_Coordinate` | 128-byte historical seed retained as migration-compatible structure: family, position, historical inversion-state storage, weave, semantic/source anchor, six base links, six reflective slots, execution hook and payload. |
| raw psychoids / Hash | `ql_default_psychoid_bimba`, `ql_default_hash_bimba` | Raw psychoids remain pre-categorical `FAMILY_NONE`; Hash remains generative/non-positional rather than being forced into the sixfold. |
| tagged `#`, `.`, `-`, `()` relations | relation tagging/accessors | Historical relation metadata is retained for parity and addressability. The `#` tag is not reduced to positional complement. |
| family field | `QL_Holographic_Field` | Native 6×6 family-bearing coordinate field with M retained as a full parent family. |
| Bimba / Pratibimba source-manifestation | `QL_Bimba`, `QL_Pratibimba`, materialize/source/bedrock | Structural source ↔ manifestation remains recoverable. |
| P/P′, L/L′ coordinate notation | `QL_Coordinate_Label` + face mapping | Prime/unprime is represented as a **face over the same positional index**. `P2′` remains position 2; `L1′` remains lens 1. No `5-i` remap is performed. |
| `Kernel_Bioquaternion`, quaternion conjugation, slash flip | `ql/kernel.h`, `c/src/kernel.c` | Actual conjugate dynamics migrated with frozen-kernel parity. |
| 12-tick kernel | `QL_Kernel_Tick` + `ql_kernel_tick_position_label` | Historical dynamics remain unchanged. Coordinate mapping reads ticks `0..5` as `P0..P5` Bimba-face and `6..11` as `P0′..P5′` Pratibimba-face using the kernel's own `position6 = tick % 6`. |
| 72-fold resonance | `ql_kernel_resonance_map` | Existing `6 lens × 2 face × 6 inner-position` address is labelled as `L_i/L_i′ × inner-position`; no new conjugation math is introduced. |
| harmonic/tritone/resonance scalar helpers | accepted `ql-c/primitive` calls | Reused beneath the kernel where they are genuinely the same finite law. |
| energy evaluation | `ql_kernel_energy_evaluate` | Migrated with frozen-kernel parity. |
| no independent native-C distribution seam | `c/Makefile` static archive/install/package | Deterministic QL-owned native-C artifact with API/revision metadata. Packaging supports consumption but is not itself the R4 product meaning. |

## Coordinate mapping invariants

R4 now proves the mapping layer without claiming it is the generative mechanism:

- `C/P/L/S/T/M` and raw `FAMILY_NONE` remain addressable;
- coordinate family and six-position identity remain separate;
- P/P′ face change preserves the P index;
- L/L′ face change preserves the L index;
- P′ is labelled as the Klein/non-orientable face where the current coordinate account supports that topology;
- M remains `family=M, position=i`; generic coordinate face metadata is not equated with the deeper `M_i′` compositional product relation;
- Bimba → Pratibimba materialisation preserves recoverable source identity;
- the 12-tick dynamics map onto P/P′ labels without injecting M1 traversal/complement semantics;
- the 72-fold resonance dynamics map onto L/L′ labels plus the existing six inner positions;
- actual slash-flip parity remains quaternion conjugation of the bioquaternionic state.

## Kernel parity retained

`migration/epi-kernel/r4-holographic-kernel-parity.c` still compares the promoted kernel to the frozen executable source for:

- harmonic ratios and epogdoon log;
- 72-fold resonance indices and tritone squares;
- bioquaternion initialization;
- slash-flip conjugation;
- resonance-square emphasis;
- energy decomposition;
- the complete `uint8_t` tick-input domain against the historical 12-tick behavior.

The new assertions are mapping assertions: the same returned kernel states receive the expected coordinate labels. They do not alter those states to fit a coordinate theory.

## What R4 no longer does

R4 does **not** create or require an Epi M1 consumer switch through `ql_position_invert`.

The earlier `#1-4.2 Principle of Inversion -> p -> 5-p -> M1 consumer` path conflated a semantic/source coordinate description and a finite positional complement with the mature conjugate-reflection mechanism. That path is withdrawn from R4. The separate Epi PR created for it is superseded/closed rather than treated as migration evidence.

The scalar positional-complement helper can remain as historical/generalized finite law until its naming/source articulation is revisited, but it is not the P/P′ or L/L′ conjugation engine.

## R5 handoff

Per parent programme #51, R5 is the later incremental rebase of M0–M5 consumers onto the now-real shared kernel. It should consume the kernel's actual coordinate maps and dynamics where concrete duplicate dependencies exist. It must not use M1's positional complement table to define the universal conjugate system.

#55 data authority, any remaining historical tick-description discrepancy, #74/#75 fourfold mapping, #73, D, and deep M′ products remain independent work unless separately authorized.
