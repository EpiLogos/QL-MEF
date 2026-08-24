# Canonical pre-M musical derivation — executable contract

Status: implementation companion to issue #31.  
Canonical authored source: `docs/sources/ql-musical-derivation-v3.md` blob `6414c56c6241c3da46e1ea6fdcd7a09b6b66c5aa`.  
Kernel ground: #39 / PR #19 and #56 / PR #76.

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

## Executable chain

`crates/ql-mef/src/music.rs` carries the canonical pre-M chain as generators and transforms:

```text
First Spanda 3:3 + Second Spanda 4:2
  -> (4:2)/(3:3)
  -> 1/1, 4/3, 3/4, 3/2, 2/3, 16/9, 9/8, 2/1
  -> chromatic basis (9/8) + fifths basis (3/2)
  -> the same twelve pitch classes in two traversal orders
  -> P/P′ Name-Power content on the shared six positions / two faces
  -> kernel A/B/C and canonical cross D1/D2 transform/require/complete; D3 is primed A/B/C invariance
  -> all twelve L/L′ tonic anchors derived from their kernel coordinates
  -> 8+4 explicate/implicate partition
  -> 3x3 square apparatus generated from kernel A/B/C pairs on both faces
  -> existing seven-CF cut producing the reference diatonic
  -> seven modal rotations plus the authored Name/Power form-selection patterns
  -> 12 lens anchors x 7 CF modes = 84 inspectable structural instances
  -> enriched return remains the next-cycle operation already carried by the kernel
```

`derive_pre_m_music()` returns the whole finite chain for either co-foundational basis. Lens anchoring is a transform from the lens's existing kernel coordinate; the 84 field is produced by those derived anchors and the seven existing Context Frames.

## Exact basis mappings

At L0, the generators produce the V3 mappings:

- chromatic direct: C D E F# G# A#; prime: C# D# F G A B; cross-face axis = 1 semitone;
- fifths direct: C G D A E B; prime: F# C# G# D# A# F; cross-face axis = 6 semitones.

Both unions are the same complete `Z12` pitch-class substrate.

## Pairing and interval evidence

The executable interval functions calculate directed pitch deltas from the exact V3 pitch mappings while reusing the accepted kernel operators. `AUTHORED_INTERVAL_REFERENCES` separately preserves Reference Table 9's authored interval-language in machine-addressable form. Keeping these two representations distinct preserves both the source table and the directly calculable pitch consequence of the exact coordinate mappings.

The 3x3 musical squares are generated from `RelationFamily::{A,B,C}` plus direct/prime completion. Family and pair-index provenance therefore remains addressable when A-square-2 and C-square-3 share the same four vertices.

## CF and modal derivation

`cf_diatonic_cut()` consumes `ContextFrameCut::canonical()` and renders its existing Name/Power selections through the chromatic pitch substrate, transposed to the selected lens anchor. At L0 chromatic this yields C D E F G A B with the canonical form sequence Name-Name-Name-Power-Power-Power-Power.

The seven `ModeKind` values rotate that same CF-cut scale and retain the V3 form-selection patterns for Ionian, Dorian, Phrygian, Lydian, Mixolydian, Aeolian and Locrian. The major/minor character degrees remain the three authored conjugate choices at degrees 3, 6 and 7.

The complete landscape is generated as twelve derived L/L′ scale-beneath anchors crossed through the seven CF groundings, producing 84 structural mode-tonic instances while preserving lens identity, CF identity, pitch content and form-selection provenance.

## Provenance boundary

The executable core is determined by the vendored V3 plus the accepted Q6/C holographic kernel. Deeper subsystem matrices remain downstream semantic consumers of this musical object. The pre-M derivation reaches the complete authored musical object before any M-family generative dependency is introduced.
