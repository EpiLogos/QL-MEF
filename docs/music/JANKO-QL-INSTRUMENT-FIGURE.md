# Jankó Keyboard as a QL Instrument Figure

Status: historical instrument Figure / controller projection reference  
Musical authority: `docs/sources/ql-musical-derivation-v3.md` + executable #31 derivation  
Kernel authority: #78 / accepted QL-MEF relation grammar  
Purpose: record a concrete playable geometry in which the already-derived QL `3:3`, `4:2`, whole-tone and semitone relations can be expressed without making the historical instrument a source of QL theory.

## 0. Provenance boundary

Two statements are intentionally distinct.

### Historical instrument fact

The Jankó keyboard, developed by Paul von Jankó in 1882, uses a six-row isomorphic surface made from two interleaved whole-tone row families repeated as three touch-points for each sounding note. Along a row the next key is a whole tone away; the neighbouring interleaved row provides the semitone-shifted complementary whole-tone collection.

Traditional piano-derived colouring makes the two alternating six-note row families visibly different:

```text
row family A: 4 white : 2 black
row family B: 3 white : 3 black
```

The six-row/two-whole-tone construction and colour distribution are historical/instrumental facts.

### QL instrumental Figure

QL independently derives:

```text
First Spanda   = 3:3
Second Spanda  = 4:2
```

and #31 already derives the chromatic substrate as two six-position whole-tone helices whose union is `Z12`.

The project-level Figure is therefore:

```text
QL 3:3  ↔ Jankó 3:white / 3:black row-family distribution
QL 4:2  ↔ Jankó 4:white / 2:black row-family distribution

QL two whole-tone helices
        ↕
Jankó two interleaved whole-tone row families

QL semitone cross-face axis
        ↕
Jankó diagonal/cross-row semitone movement
```

This is a structural/playable correspondence, not a claim that Jankó historically encoded QL.

---

## 1. Why this Figure belongs after the executable derivation

The accepted #31 object already determines the music:

```text
3:3 × 4:2
  → exact ratios
  → chromatic / fifths bases
  → direct / prime sixfolds
  → A / B / C + D completion
  → L/L′ tonic anchor
  → 8+4 / CF / mode field
```

The Jankó surface becomes a **projection/controller** of that object. It does not alter the ratios, position identities, relation operators, tonic anchors or Context Frames.

Its value is that the physical surface has the same kind of isomorphic regularity as the QL musical object:

- repeated fingering under transposition;
- two interleaved whole-tone rows generating chromatic adjacency;
- three repeated touch-points producing a six-row playable field;
- the `3:3` and `4:2` colour counts co-present on alternate rows of the same instrument.

The standard piano remains a legal rendering. Jankó is especially informative because its geometry makes the sixfold/two-face substrate more directly visible and playable.

---

## 2. Controller-neutral projection contract

A Jankó-aware controller should consume the accepted musical state rather than introduce its own music semantics.

A useful mapping frame is:

```text
JankoSurfaceProjection {
  sounding_pitch_class
  whole_tone_row_family
  repeated_touch_point

  kernel_position
  direct_prime_face

  musical_basis
  relation_family
  completion_degree

  lens_anchor
  context_frame

  spanda_phase?
  ananda_projection_ref?
}
```

The exact ABI belongs to the eventual controller/input layer. The semantic boundary is stable:

```text
QL-MEF derives musical meaning
        ↓
controller projection assigns that meaning to a Jankó key/touch-point
        ↓
input/output events return through the same canonical kernel addresses
```

---

## 3. Initial playable overlays

The same physical key field can expose several orthogonal relations without changing note assignment:

### Direct / prime

Use the two interleaved whole-tone row families to make the currently active direct/prime chromatic sixfold visible.

### 3:3 / 4:2

Retain the traditional black/white colouring as a reference layer when available. It already exposes the two sixfold cardinal partitions that ground the Spanda musical derivation.

### A / B / C

Highlight the active positional pairing relation over the six QL positions.

### D1 → D3

Expose conjugate-completion degree independently from A/B/C. D remains the degree of completion, not a fourth coequal relation family.

### Chromatic / fifths

The physical Jankó rows naturally express the chromatic/whole-tone basis. The fifths basis is a second traversal/address overlay over the same twelve pitch classes, matching the #31 rule that both bases exhaust one `Z12` substrate.

### L/L′ + CF

Lens/tonic and Context-Frame state should alter anchoring/functional display without changing the controller's fundamental isomorphism.

---

## 4. Relation to M1′ / Ananda

Ananda is downstream input to richer M1′ performance state, not a key-numbering scheme.

A Jankó surface can eventually display or modulate:

- current `tick12` / Spanda phase;
- direct/prime face and conjugate co-state;
- Ananda family and raw/DR projection;
- exact harmonic ratio;
- Hopf/SU(2) phase;
- M2/M3 performance/transcription state when the surface is used inside the wider Epi instrument.

The numerical Ananda substrate, pitch class and physical key remain distinct typed relations.

---

## 4a. The surface as the torus unwound (owner ruling, 13 October 2026)

The Figure is no longer only a controller projection. In the DLW's one instrument (Wayfinder §0.4) the Jankó surface is the M1 torus unwound:

```text
bimba helix (orientable torus arc)          ↔ WT₀ ↔ Jankó row family A/B (as #31 anchors it)
pratibimba helix (Klein-extension arc)      ↔ WT₁ ↔ the interleaved row family
key                                         ↔ excitation point on its helix of the M1 torus
semitone (cross-row / diagonal)             ↔ cross-helix step (conjugate axis)
transposition (same fingering)              ↔ motion along the helix (same surface figure)
chord                                       ↔ simultaneous excitation set → its figure on the surface
```

Playing a key excites the torus at that point; the visible pattern and the sound are observations of the same body. The figure a chord draws is the figure shown for the symbol that harmony belongs to. The historical-instrument provenance boundary of §0 still holds: Jankó did not encode QL. What is new is that the playable surface and the sounding body are one object read flat and read wound. The exact angular placement on the arcs is a kernel derivation (Wayfinder §0.4 item 4), not a layout choice.

### Standing of the placement, 20 October 2026

The striking half is built; the placing half is not, because the sources do not yet determine it.

- **Built.** The scene body takes a strike at a point: `strike-at {points: [{theta, phi, amplitude}]}` (host operation; stage change `strike-at`; `InstrumentSession.strikeAt`). Each standing voice is excited by `amplitude × χ(voice, φ, θ)`, the voice's own shape term with the weight the body's `mode_shapes` carry, summed over the points of one act (`continuous/scene_field.rs`, `ShapeBasis::strike_at`). It names no key: the caller names the point.
- **Not determined: which point a key is.** Per key the Figure needs `{theta, phi, arc, cover_turn}`. What the sources fix and leave open:
  - V3 (`docs/sources/ql-musical-derivation-v3.md` lines 21–24, §II-1.1, §II-1.5) fixes which pitch classes lie on which helix (WT₀ = bimba arc = the orientable torus, WT₁ = pratibimba arc = the Klein extension, "one semitone above") and the order along each. It states no angle.
  - Compiled graph, `M1-5` `c_1_winding` and `M1-5-2` `c_1_connection_to_1_4`: positions 1–2 on the meridian θ₁, positions 3–4 on the longitude θ₂, position 5 at (2π, 2π) ~ (0, 0); `M1-5-3`: the negation points θ₁ = π (between positions 1 and 2) and θ₂ = π (between 3 and 4), and the semitone is the π-phase shift made audible. It gives intervals of the circuit, not the angle of a position inside one, and it speaks of six positions, not twelve keys.
  - The seed `M1'-SPEC` §10 (not compiled into the graph) puts the chromatic circle on the longitude and the circle of fifths on the meridian, with the two helices as two longitudes offset by a semitone; the graph says nothing of chromatic or fifths, and `docs/kernel-rebuild/m123-scene-map/m1-body.md` records the pair as the open `gate0-m1:chromatic-fifths-axes-unseated`.
  - Rulings D5 (one tick = 30°) and D6 (the large circle is the ecliptic clock) fix the tick ring and the large circle's use. D17, which sheet carries the anti-periodic content (recommended: the Hopf fibre, `degree720 ≥ 360`), sits against V3, which gives the Klein arc to the pratibimba face. The Klein identification has no QL owner (`m1-body.md` §1b, "K² (what it is here)").
  - Open for the owner: the circle the helices wind on (meridian or longitude), the angle of each position inside its half-arc, whether the pratibimba arc is laid half a turn on, by the semitone, or on the second sheet of the 720° cover. Until then no key carries a point, and the surface disclosure publishes none.

## 5. Acceptance for a future Jankó mapping

A controller implementation is faithful when:

1. the physical six-row layout preserves two interleaved whole-tone families and three repeated touch-points per sounding note;
2. semitone adjacency is recovered through the cross-row/diagonal relation rather than by inventing a second pitch map;
3. all twelve pitch classes agree with the accepted #31 chromatic substrate;
4. direct/prime and A/B/C+D state are projected from canonical kernel IDs;
5. transposition preserves fingering shape as expected of the isomorphic surface;
6. the `3:3` and `4:2` reading is represented explicitly as a project-level Figure with historical provenance kept separate from QL derivational provenance;
7. no controller-specific representation becomes a new authority for the musical system;
8. each key excites the M1 torus at its derived helix point, and the surface figure and the sound of a chord come from the same state (§4a).

## 6. Historical references

The historical facts above are grounded independently from the QL interpretation.

- **Smithsonian Institution — historical instrument/keyboard account.** Its Jankó description records six ranks of keys, each row tuned as a whole-tone scale, with rows 1/3/5 beginning on one whole-tone collection and the alternate rows on the semitone-shifted collection. This establishes the six-row / two-interleaved-whole-tone construction used by the Figure.  
  https://repository.si.edu/bitstreams/76ab52f7-b896-4298-b070-7b7589bf32bc/download
- **Daskin Uniform Keyboard Systems — specialist Jankó construction description.** It records the traditional natural/accidental colouring on the whole-tone rows: the first/third row family carries four white and two black keys per six-note cycle, while the alternating rows carry three of each colour.  
  https://daskin.com/page5/page5.html
- **General construction cross-check.** The standard Jankó description gives two interleaved manuals with three touch-points for each key lever, producing six rows; adjacent columns/alternate rows differ by semitone while movement within a row is by whole tone.  
  https://en.wikipedia.org/wiki/Jank%C3%B3_keyboard

These sources establish only the historical/instrumental side of the Figure. The correspondences `3:3 ↔ 3 white : 3 black`, `4:2 ↔ 4 white : 2 black`, and the relation of the Jankó geometry to QL direct/prime, Spanda and the accepted #31 musical object remain current Epi/QL interpretive and product-design propositions.