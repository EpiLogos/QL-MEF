# M3 coin-value discrepancy — ripple analysis and owner ratification ask

status: "analysis of record; canonical table landed in Rust with parity-consistent default; C dataset regeneration pending owner ratification"
created: 2026-09-06
task: "TASK N4 stage 0, discrepancy deliverable"
authorities:
  - "vendor/epi-kernel/reference/include/m3.h (FR 2.3.12 array, FR 2.3.1 bit semantics)"
  - "vendor/epi-kernel/reference/src/m4.c m4_cast_iching (the generating coin law)"
  - "vendor/epi-kernel/reference/src/m3.c M3_PAIR_MATRIX, m3_verify_integral_invariant"
  - "docs/geometry/FOLD-AND-RULING-GRAMMAR.md s12 (crease-bits, ratified elemental throughline)"
  - "docs/origami work/INTEGRATED-1-2-3-PHYSICAL-POLE-OBJECT.md s5.1"

---

## 1. The discrepancy, stated once

`vendor/epi-kernel/reference/include/m3.h:35-44` (FR 2.3.12) declares:

```c
A=6(Yin,Moving), T=9(Yang,Moving), C=7(yin,Resting), G=8(yang,Resting)
static const uint8_t NUCLEOTIDE_ICHING_VALUE[4] = {6, 9, 7, 8};
```

The same header's FR 2.3.1 block fixes the two-bit semantics (bit 0 polarity:
0=yin; bit 1 mobility: 0=moving) and the labels:

```c
A=0b00 Yin/Moving   — Old Yin
T=0b01 Yang/Moving  — Old Yang
C=0b10 Yin/Resting  — Young Yin
G=0b11 Yang/Resting — Young Yang
```

The **labels are right and human-ratified** (C=Earth/yin/resting, G=Air/yang/resting;
elemental throughline `A=Water, T=Fire, C=Earth, G=Air` per M4 ratification 5,
carried in FOLD-AND-RULING-GRAMMAR s12). The **numbers violate both** the labels'
own coin arithmetic and the generator's parity law.

The generating law is `m4_cast_iching` (`m4.c:~403-419`): three coins, heads=3
tails=2, sum in {6,7,8,9};

```c
/* Yang line (7 or 9) = bit set; Yin (6 or 8) = bit clear */
if (sum & 1) ...
/* Changing lines: 6 (old yin) or 9 (old yang) */
if (sum == 6 || sum == 9) ...
```

Polarity is the **parity of the coin sum** (odd = yang: 7, 9; even = yin: 6, 8);
mobility is **extremity** (all-same triples 6=2+2+2 and 9=3+3+3 move; mixed
triples 7=2+2+3 and 8=2+3+3 rest). This is also the classical coin arithmetic:
old yin 6, **young yang 7**, **young yin 8**, old yang 9.

Therefore in the current array **both C and G carry the wrong value for their
ratified parity**: C (yin/resting) must be 8, not 7; G (yang/resting) must be 7,
not 8. The parity-consistent array is **{A=6, T=9, C=8, G=7}** — element labels
untouched, mobility untouched, complementary-pair sums untouched (A+T = C+G = 15;
total 30). Only the C/G value assignment flips.

## 2. What depends on the current array (ripple inventory)

Everything below consumes `NUCLEOTIDE_ICHING_VALUE` directly or was **generated**
from it.

### 2.1 Value-preserving under the flip (no ripple)

- Complementary-pair sums (A+T, C+G = 15): the flip swaps 7/8 inside the pair.
- Total over all values (30), and **any codon-sum aggregate that is symmetric in
  C/G** — in particular the 360 integral **total**: raw pp sum over all 64 codons
  is 1440 = 4×360 under either assignment, because the flip permutes the value
  multiset {6,9,7,8} (identical multiset).
- `M3_RES_MATRIX` 8 resonance gaps: structural (trigram-row positions
  Kun/Li, Kan/Li, …), not value-derived — unaffected.
- `M3_CODON_TO_AA`, hexagram/trigram LUTs, codon classification (40 non-dual /
  24 dual → 472 rotational states), all bit-level line-change machinery
  (`m3_line_change`, complement, 384 adjacency): structural, value-independent.
- Existing Rust conformance fixtures: **none exist** — there is no prior
  nucleotide/codon code in the Rust workspace. The Rust pole contract is
  greenfield and takes the parity-consistent table as its default.

### 2.2 Changed by the flip (dataset-locked ripples)

1. **`M3_PAIR_MATRIX[16]` (dataset-backed sumValue/differenceValue).** The
   recorded values are exactly the current array's arithmetic:
   `CC={14,0}` (7+7), `GG={16,0}` (8+8), `AG={14,±2}` (6+8), `TC={16,∓2}` (9+7),
   `AC={13,±1}` (6+7), `TG={17,±1}` (9+8). Under the parity-consistent array
   these become CC=16, GG=14, AG=13, TC=17, AC=14, TG=16, and the mixed-pair
   difference magnitudes change (e.g. TC: |9−7|=2 → |9−8|=1). **8 of 16 entries
   change.** Consumers: `compute_rotational_state` (total_sum/difference),
   `m3_generate_rotational_states` (`rotational_value` = pair sums/differences),
   the epogdoon ascent `get_parashakti_frequency` (sumValue → Parashakti
   frequency index), Matrix-2 class-stability bookkeeping.

2. **Per-suit 360 integral constants (FR 2.3.15).** `m3_verify_integral_invariant`
   classifies by outer nucleotide: raw suit sums 336/384/352/368 → constants
   84/96/88/92 (Cups/Wands/Pentacles/Swords). Under the flip, outer-A and outer-T
   sums are unchanged (336, 384) but **outer-C and outer-G swap**: C→368 (=92×4),
   G→352 (=88×4). The C `_Static_assert` on the *sum* still holds, but
   `m3_verify()` (runtime, suit-by-suit) **fails** unless the constants are
   regenerated together with the array. The Tarot codon map's per-card `pp=`
   annotations (m3.c comments) shift likewise for C/G-bearing codons.

3. **`m3_quat_from_codon`** (w = codon sum, x = v_outer − v_inner, z = sum mod 6):
   every codon whose outer/inner C/G content is unbalanced gets a different
   quaternion seed, hence different `m3_quat_active_state` selections in the
   DET overlay path.

4. **Any inherited literature mapping read *from* the current numbers.** The
   datasets' sumValue/differenceValue columns were generated under the legacy
   assignment; any external or essay-side table derived by reading those columns
   inherits the legacy values and must be regenerated, not reinterpreted.

### 2.3 The discrepancy is proven, not assumed

The dataset matrix is *internally consistent with the legacy array* — which is
exactly why it is generation-locked. It is *inconsistent with the parity law its
own generator enforces*: under the coin law, yin/resting pairs must sum even and
yang/resting pairs odd — the recorded CC=14 (even, fine) is consistent at sum
level, but the value-level parity identities (C's value must be even; G's odd)
fail. Both facts are encoded as executable conformance in the Rust contract:

- `crates/ql-core/tests/pole_coin_contract.rs` asserts the canonical table
  `{6,9,8,7}` satisfies parity+extremity+complement laws.
- The same test file contains a passing test proving the legacy array
  `{6,9,7,8}` is parity-violating at the C/G positions.
- A `#[ignore]`-marked known-open test documents that the C dataset
  (`M3_PAIR_MATRIX`, per-suit integrals) is *not yet* consistent with the
  canonical table; it becomes runnable the day the dataset is regenerated.

## 3. Resolution carried by this series (one table, one law)

The Rust pole contract (this series) lands **one named canonical table**:

```text
NUCLEOTIDE_COIN_VALUE — parity-consistent default
A=6 (even, yin, moving — old yin)
T=9 (odd,  yang, moving — old yang)
C=8 (even, yin, resting — young yin)
G=7 (odd,  yang, resting — young yang)
```

conformance-tested against the coin law (parity, extremity, complement sums,
codon-sum invariants, four-charge 4X invariant, 360 total). The C reference
kernel and its datasets are **not modified here**. Never mix mappings silently:
until the owner ratifies the flip, the C dataset stays legacy and the Rust
contract's known-open test marks the seam.

## 4. Owner ratification ask

**Ask:** ratify the parity-consistent nucleotide coin values
`{A=6, T=9, C=8, G=7}` and authorise a follow-up kernel-change series that:

1. flips `NUCLEOTIDE_ICHING_VALUE` to `{6, 9, 8, 7}` in
   `vendor/epi-kernel/reference/include/m3.h` (keeping FR 2.3.12's sum
   static-asserts, which already hold);
2. regenerates the dataset-backed `M3_PAIR_MATRIX` sumValue/differenceValue
   columns (8 entries change) from the ratified table;
3. swaps the per-suit integral constants to `Pentacles=92, Swords=88` (total
   84+96+92+88 = 360 unchanged) so `m3_verify()` passes;
4. regenerates any dataset columns derived from pair sums (rotational values,
   Parashakti frequency indices) and re-runs the C/Rust parity migrations.

**Evidence for the ask:** s1 (the generator's own parity law), s2.2 (the exact
dataset entries that move), s2.3 (executable proof in the Rust conformance
tests). The elemental labels (C=Earth/yin, G=Air/yang) are **not** in question —
ratifying this ask changes numbers to match the ratified labels, never the other
way around.
