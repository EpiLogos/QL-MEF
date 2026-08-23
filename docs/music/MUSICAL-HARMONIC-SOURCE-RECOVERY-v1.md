# Musical / Harmonic Source Recovery v1

Status: **operational recovery for QL-MEF #31**  
Executable module: `crates/ql-mef/src/music.rs`  
Machine-readable fixtures: `fixtures/music/*`  
Kernel authority: `ql.holographic-kernel-contract/v1` / `1.1.0`

## Current relation

The musical field is one continuation of the accepted holographic kernel:

```text
#0..#5
  -> family + direct/prime face
  -> A/B/C + D square/cross grammar
  -> VAK
  -> 12 L/L' lenses x six local positions = 72 MEF coordinates
  -> seven canonical Context Frames
  -> M1 formal harmonic relations
  -> M2 active 72-fold refraction/transduction
  -> M3 determinate 64-fold transcription and clock apertures
```

M1/Paramaśiva supplies the formal harmonic possibility-space. M2/Paraśakti actively differentiates and qualitatively applies it. M3/Mahāmāyā inscribes/transcribes determinate form. M4/Nara consumes the resulting field in situated lived composition and is presently `research-human-ratification-required`. M5/Epii supplies recognition/governed return; it does not mint new harmonic constants.

The exact source revisions used by this recovery are frozen in `fixtures/music/source-provenance-v1.tsv`.

## Recovered foundational musical determinations

The current human-reviewed M1 matrix gives the finite ratio vocabulary:

```text
1/1  4/3  3/4  3/2  2/3  16/9  9/8  2/1
```

and the explicit relations:

```text
(4/3)(3/2) = 2/1
(3/2)/(4/3) = 9/8
(2/1)/(16/9) = 9/8
72/64 = 9/8
(16/9)(9/8) = 2/1
(4/3)(9/8)(4/3) = 2/1
```

It also names two traversal bases, `epogdoon/chromatic` and `fifths`; retains A/B/C plus D1/D2/D3 as the already-reconciled kernel relation families; identifies `6+6`, `8-fold`, and the `7-fold CF cut` as relevant grains; and states the mode/tonic landscape as:

```text
12 lens tonics x 7 Context Frame modes = 84
```

The executable cut therefore reuses `LensId` and `ContextFrameId` directly. It does not create a second pitch, lens, or modal coordinate system.

## M2 -> M3 epogdoon / DET handoff

M2 identifies `9:8` as the epogdoon/DET handoff from its 72-condition field into M3. The current human-reviewed M3 matrix makes the finite operation exact:

```text
target64 = floor(source72 * 8 / 9)
source72 in 0..71
```

This map has:

```text
72 source states
64 target states
8 target collisions
```

The implementation and `fixtures/music/epogdoon-72-to-64-v1.tsv` freeze all 72 rows. Tests prove all 64 target addresses are reached and exactly eight targets have a two-element preimage.

M3 also explicitly records `M3-C02` as `implemented-fold-semantics-open`. The integer operation is therefore executable while the richer meaning of each fold/collision remains an open semantic edge.

## Two distinct lens/aperture systems

The accepted MEF registry contains **12 canonical L/L' lenses**. Those are the lens identities used by the 72-coordinate MEF and the 84 tonic x Context-Frame landscape.

The M3 world-clock separately contains **16 static clock apertures** over 360 degrees:

```text
1x360   2x180   4x90    8x45
9x40    10x36   12x30   15x24
24x15   30x12   36x10   40x9
45x8    90x4    180x2   360x1
```

with reciprocal index pairs:

```text
0<->15  1<->14  2<->13  3<->12
4<->11  5<->10  6<->9   7<->8
```

The M3 matrix's 2026-08-23 human-ratification record states that these sixteen static apertures are relatively settled and that Fibonacci/Pisano is pre-lensic/base rather than a seventeenth static lens. QL-MEF therefore exposes them as `M3ClockAperture`, not `LensId`, preserving the distinction in the type system.

## Complete recovery of the retained M3 harmonic-mathematics v2 source

`m3-prime-ql-harmonic-mathematics-v2.md` is presently marked `seed`; it is rich research/source ground rather than automatic executable authority. Its complete argumentative structure is:

1. `100% -> 16:9 -> 2^4:3^2 -> 5^2` genesis and Bimba/Pratibimba ground.
2. 4/6 static/processual breathing, pentadic discovery, and 80-fold space.
3. 2^6 = 64 base frame, frame modulation, quadrants, quaternion core, complement operations, trigram/QL partitions.
4. +/-16 phase space, 4.5 clock multiplier, 72/216/288/360 projection, 1:3:4:5 relation, 72 derivations.
5. +/-16 Tarot rotational dynamics.
6. Sixteen 360-degree division grammars and reciprocal pairs.
7. 144 = 12^2 mixed-modular space and mod9/mod12 phase-lock at 36/72.
8. 0/1, 2/3, 4/6 trika of frame-breathings and shifted-depth isomorphisms.
9. Frame-size x modular-tier master table and cross-frame isomorphisms.
10. Five-level harmonic hierarchy: 64, 144, 256, 729, 1296.
11. Three-matrix / Trika processing modality and ternary processing states.
12. Tarot integration: 80-fold repertoire, 24-fold Major/governance reading, 56-fold Minor field, 40+40 traversal.
13. Quaternion signature system and candidate layered-weight approaches.
14. Higher mod6/mod12 tiers and Ananda phase-lock.
15. Key-number structural ledger.
16. Self-sealing return from the generated number field to the originating 16:9 relation.

This recovery preserves those propositions as source-ground. Only relations independently ratified strongly enough by the current kernel/matrices are promoted by `music.rs` in this cut.

## Standing by distinction

### Authored / accepted for this executable cut

- accepted kernel relation/operator identities from completed #39 and kernel contract 1.1.0;
- M1 eight-ratio vocabulary and its stated exact product/quotient relations;
- the existing 12 L/L' lens identities and seven Context Frames;
- the stated 12 x 7 = 84 tonic/Context-Frame address landscape;
- M2's `9:8` 72->64 handoff relation;
- M3's exact `floor(index72*8/9)` operation and finite counts;
- M3's sixteen static clock-aperture table and reciprocal pairs.

### Formally derived and test-proved here

- rational reduction and reciprocal equality for the promoted ratio vocabulary;
- the exact product/quotient closures listed above;
- 72 epogdoon input rows reach all 64 targets;
- exactly eight M3 targets have two 72-space preimages;
- the tonic/Context-Frame product contains exactly 84 unique native addresses;
- every M3 clock aperture satisfies `sectors * arc_degrees = 360` and reciprocal pairing is involutive.

### Research/source propositions retained without promotion

- the v2 source's wider mixed-modular hierarchy and metaphysical interpretations;
- 4.5 as a general phase-space-to-clock scaling operator;
- Tarot, biochemical, sacred-number, cymatic and higher-modular correspondences beyond presently ratified executable relations;
- candidate quaternion weighting approaches in the v2 source.

### Explicit open edges

- semantic meaning of the eight 72->64 collision/fold points (`M3-C02` is explicitly open);
- canonical coordinate <-> pitch-class assignment beyond the already-authored ratio/traversal ground;
- the exact executable `epogdoon/chromatic` and `fifths` traversal operator tables beyond their present authored names;
- mode/scale content at each of the 84 addresses beyond the address landscape itself;
- chord, voicing, voice-leading, progression and cadence operators: the recovered v2 source does not provide a deterministic table for them;
- rhythmic semantics beyond already-existing 12-fold/epogdoon structural periods;
- M3 Spanda tick/pulse/two-phase-superposition semantics, explicitly still open in the current M3 matrix;
- M4 personal/lived quaternion composition decisions, whose current matrix still requires human ratification.

These are development edges, not invitations to fill the gaps from generic Western music theory or symmetry alone.

## Executable surface in this cut

`crates/ql-mef/src/music.rs` now provides:

- `HarmonicRatio` and the eight-entry `HARMONIC_RELATIONS` table;
- `MusicalEvidenceClass` for authored/accepted, formally derivable, research-proposed and open-edge standing;
- `epogdoon_72_to_64` and target preimage-width inspection;
- `tonic_context_frame_landscape` over native `LensId x ContextFrameId`;
- `M3ClockAperture`, the exact sixteen-entry clock table and reciprocal indices;
- explicit constants retaining the open status of epogdoon-fold semantics.

The accompanying TSV fixtures make the ratio table, all 72 epogdoon rows, the sixteen M3 clock apertures, and exact source revisions independently inspectable by agents and conformance tests.
