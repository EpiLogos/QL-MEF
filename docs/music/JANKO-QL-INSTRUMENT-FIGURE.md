# Jankó Keyboard as a QL Instrument Figure

Status: historical instrument Figure / controller projection reference  
Musical authority: `docs/sources/ql-musical-derivation-v3.md` + executable #31 derivation  
Kernel authority: #78 / accepted QL-MEF relation grammar  
Purpose: record a concrete playable geometry in which the already-derived QL `3:3`, `4:2`, whole-tone and semitone relations can be expressed without making the historical instrument a source of QL theory.

## 0. Provenance boundary

Two statements are intentionally distinct.

### Historical instrument fact

The Jankó keyboard, developed by Paul von Jankó in the nineteenth century, uses a six-row isomorphic surface made from two interleaved whole-tone row families repeated as three touch-points for each sounding note. Along a row the next key is a whole tone away; the neighbouring interleaved row provides the semitone-shifted complementary whole-tone collection.

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

## 5. Acceptance for a future Jankó mapping

A controller implementation is faithful when:

1. the physical six-row layout preserves two interleaved whole-tone families and three repeated touch-points per sounding note;
2. semitone adjacency is recovered through the cross-row/diagonal relation rather than by inventing a second pitch map;
3. all twelve pitch classes agree with the accepted #31 chromatic substrate;
4. direct/prime and A/B/C+D state are projected from canonical kernel IDs;
5. transposition preserves fingering shape as expected of the isomorphic surface;
6. the `3:3` and `4:2` reading is represented explicitly as a project-level Figure with historical provenance kept separate from QL derivational provenance;
7. no controller-specific representation becomes a new authority for the musical system.

## 6. Historical reference note

The historical facts above should be cited in publication-facing material from primary or specialist keyboard-history sources. The working project reference used in the current review includes descriptions of the original six-row Jankó construction as two interleaved whole-tone manuals with three touch-points per note, and specialist Jankó-layout documentation recording the traditional alternating `4 white : 2 black` and `3 white : 3 black` row colour distributions.