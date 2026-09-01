# Canonical pre-M musical derivation — executable contract

Status: implementation companion to issue #31.  
Canonical authored source: `docs/sources/ql-musical-derivation-v3.md` blob `6414c56c6241c3da46e1ea6fdcd7a09b6b66c5aa`.  
Kernel ground: #39 / PR #19 and #56 / PR #76.  
Deep-system return Wayfinder: `EpiLogos/Epi-Logos-C-Experiments#32`.

## Kernel relation carried into music

The musical system is a developed reading of the existing holographic kernel field.

```text
# / 0/1 <-> 1/0
        ↓
#0 #1 #2 #3 #4 #5
        ↓
C / P / L / S / T / M
        ↓
direct / prime
```

C/C′, P/P′ and L/L′ share the same position-and-face address and differ by family manifestation. In the current Rust MEF registry, Day is direct and Night is prime. In the canonical musical derivation, P supplies Name content (Truth, Mind, Word, Logos, Son, Image), P′ supplies Power content (Play, Need, Sacrifice, Decision, Love, Work), and L/L′ turns those same twelve kernel addresses into the twelve tonic / epistemic anchorings. `KernelRelationId::FamilySamePosition` is the executable relation tying the family views together.

The existing Q6 Context-Frame implementation supplies the canonical seven selections directly: `#0, #1, #2, #2′, #3′, #4′, #5′`. Music consumes that grammar.

## First Spanda — bi-phasal genesis of the standing whole

The full structural equation retained by the theorem source is:

```text
0 = (0/0)
  → ((0/1)/(1/0))
  → T0 || T1
  → (1/0 + 0/1)
  → 1/1 = 1 = 100%
```

This is structural rather than ordinary arithmetic. `T0` and `T1` articulate the two directional faces latent in one self-differentiation; direct/bimba and prime/pratibimba are therefore co-arising rather than a primary phase with a later inverse attached.

The musical/runtime consequence is two simultaneous descriptions of one state:

```text
synchronic
  six positions × direct/prime phase

and

diachronic
  one 12-state traversal of that doubled sixfold
```

The `3:3` reading is processual: three moments of articulation and three recognitional/complementary moments. Its deeper `3:1` reading is that `4→5→0` is one recognitive act articulated as enfolding/context, recognition, and superposition/regeneration.

## Second Spanda — ratios, Pythagorean closure and the 6/12 topology

The generated whole accounts for itself as:

```text
100% = 2^6 + 6^2
     = 64 + 36

64/36
= 16/9
= 4^2/3^2
= (4/3)^2
```

The exact directional ratio field follows:

```text
4/3   manifestation / going-forth
3/4   recognitional inverse
3/2   complementary fifth
2/3   grounding inverse
```

and `9/8` appears twice from the same field:

```text
2 / (16/9) = 9/8
(3/2) / (4/3) = 9/8
```

The later M2/M3 seam carries the same proportion after Paraśakti's `36` field is doubled:

```text
36 × 2 = 72
72 / 64 = 9/8
```

The reduced `4^2/3^2` also discloses the primitive Pythagorean completion:

```text
3^2 + 4^2 = 5^2
area      = (3×4)/2 = 6
perimeter = 3+4+5   = 12
```

So one sixfold pass and its twelvefold doubled traversal are recovered geometrically from the same Second-Spanda reduction.

## Executable chain

`crates/ql-mef/src/music.rs` carries the canonical pre-M chain as generators and transforms:

```text
First Spanda 3:3 + Second Spanda 4:2
  -> exact ratio field
  -> chromatic basis (9/8) + fifths basis (3/2)
  -> the same twelve pitch classes in two traversal orders
  -> P/P′ Name-Power content on the shared six positions / two faces
  -> A/B/C primary relation families
  -> D1→D2→D3 conjugate completion
  -> semantic cross operators over the same kernel field
  -> all twelve L/L′ tonic anchors derived from their kernel coordinates
  -> 8+4 explicate/implicate partition
  -> 3x3 square apparatus generated from A/B/C pairs on both faces
  -> existing seven-CF cut producing the reference diatonic
  -> seven modal rotations plus authored Name/Power form-selection patterns
  -> 12 lens anchors x 7 CF modes = 84 inspectable structural instances
  -> enriched return remains the next-cycle operation already carried by the kernel
```

`derive_pre_m_music()` returns the whole finite chain for either co-foundational basis. Lens anchoring is a transform from the lens's existing kernel coordinate; the 84 field is produced by those derived anchors and the seven existing Context Frames.

## A/B/C relation families and D completion

The canonical distinction is now explicit in both kernel and musical APIs:

```text
A / B / C
  three primary positional relation families

D1
  selected direct relation
  2 coordinates

D2
  one-sided conjugate expansion
  3 coordinates

D3
  complete conjugate square
  4 coordinates
```

`ql_core::build_d_modulation_frame()` owns this structural law. `ql_mef::musical_completion_frame()` now renders that same D1→D3 completion state into basis/lens-specific pitches without creating another relation taxonomy.

Historical cross-pass derivation labels remain available in the kernel for provenance, while caller-facing semantic identities are `CrossSamePosition`, `CrossTransform`, `CrossRequire`, `CrossComplete`, and the conjugate-invariance relations. Those semantic cross operators are **not** additional D completion degrees and do not make D a fourth relation family beside A/B/C.

## Exact basis mappings

At L0, the generators produce the V3 mappings:

- chromatic direct: C D E F# G# A#; prime: C# D# F G A B; cross-face axis = 1 semitone;
- fifths direct: C G D A E B; prime: F# C# G# D# A# F; cross-face axis = 6 semitones.

Both unions are the same complete `Z12` pitch-class substrate.

## Pairing and interval evidence

The executable interval functions calculate directed pitch deltas from the exact V3 pitch mappings while reusing the accepted kernel operators. `AUTHORED_INTERVAL_REFERENCES` separately preserves Reference Table 9's authored interval-language in machine-addressable form. Keeping these representations distinct preserves both the source table and the directly calculable pitch consequence of the exact coordinate mappings.

The 3x3 musical squares are generated from `RelationFamily::{A,B,C}` plus D3 completion. Family and pair-index provenance therefore remains addressable when distinct derivational squares share the same four vertices.

## Jankó instrument Figure

`docs/music/JANKO-QL-INSTRUMENT-FIGURE.md` records the controller projection separately from the derivation authority.

The historical Jankó surface gives:

```text
six rows
= two interleaved whole-tone row families
× three repeated touch-points
```

with traditional alternating row-family colour counts `4 white : 2 black` and `3 white : 3 black`. The project reading places those physical partitions alongside Second-Spanda `4:2` and First-Spanda `3:3`, while the two interleaved whole-tone row families give a directly playable projection of the chromatic direct/prime helices and diagonal semitone relation.

This is an instrument Figure over the accepted QL state, never historical proof or derivational authority.

## CF and modal derivation

`cf_diatonic_cut()` consumes `ContextFrameCut::canonical()` and renders its existing Name/Power selections through the chromatic pitch substrate, transposed to the selected lens anchor. At L0 chromatic this yields C D E F G A B with the canonical form sequence Name-Name-Name-Power-Power-Power-Power.

The seven `ModeKind` values rotate that same CF-cut scale and retain the V3 form-selection patterns for Ionian, Dorian, Phrygian, Lydian, Mixolydian, Aeolian and Locrian. The major/minor character degrees remain the three authored conjugate choices at degrees 3, 6 and 7.

The complete landscape is generated as twelve derived L/L′ scale-beneath anchors crossed through the seven CF groundings, producing 84 structural mode-tonic instances while preserving lens identity, CF identity, pitch content and form-selection provenance.

## M1-2 Ananda handoff

Ananda is a richer M1 consumer/source relation over this accepted musical object. Its executable substrate must keep distinct:

```text
raw 12×12 source arithmetic
true DR 12×12 recursive residue
explicit 10×10/mod-10 decimal aperture
```

The open bridge is therefore not “assign a pitch to each Ananda cell”. It is the derivation/test of:

```text
Ananda family + raw relation + DR residue + decimal aperture where relevant
+ Spanda phase + direct/prime state
  -> A/B/C relation family
  -> D completion degree
  -> exact ratio
  -> basis-specific interval
  -> L/L′ tonic-relative realization
```

The deep-system contract for that bridge is tracked by Epi Wayfinder #32 and `M1-2-ANANDA-EXECUTABLE-SUBSTRATE-CONTRACT.md` in the deep-matrix branch.

## Provenance boundary

The executable core is determined by the vendored V3 plus the accepted Q6/C holographic kernel. Deeper subsystem matrices remain downstream semantic consumers of this musical object. The pre-M derivation reaches the complete authored musical object before any M-family generative dependency is introduced.
