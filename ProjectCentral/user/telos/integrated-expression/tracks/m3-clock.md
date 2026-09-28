# Track: the M3-5 clock and its dynamic inscription

Why: the clock is a geometric codon–hexagram construction:
- the charge integral of the 64 codons closes to 360;
- 720 is the double cover;
- 24 backbone governors sit alongside the 360 degrees (384).

The zodiac is one of the 16+2 lenses (lens 9), and decans are read through those lenses.

The inscription is dynamic. A codon's expression at a place on the clock is
chosen by its rotational state (40 non-dual × 7 + 24 dual × 8 = 472 poses),
composed through the three matrices as quaternion axes. So the inscription is
computed in the kernel. It is never flattened into direct governor→codon or
degree→codon edges, which would drop exactly that information.

Already in the map:
- decans → pip card → codon: 36 `EXPRESSES_AS_TAROT_PIP` edges, owner-approved 28 Sep, joined on the decan's `m_2_3_tarot_card`;
- the 4 aces as the quadrant prototypes;
- 24 governors carrying their season's palindrome;
- `#3-5-*` nodes with `hexagramNumber`, `codonSequence` and `halfMonthNumber`.

Courts keep the 64 → 56 compression role (8 dual-codon courts); they are not on the backbone.

Work:
- port `spanda_codon_advance`, `m3_quat_active_state` and `m3_det_with_quaternion` natively;
- regenerate the missing `YIELDS_CODON` edges (322 of 384 present) from the matrix law with provenance;
- fix the LUT family order and trigram anchors to the graph (#254 §5).

Depends: map-and-kernel. Done-when: the native clock yields the same codon,
hexagram and rotational state as the C kernel for all 384 positions and every
matrix environment, and a test pins that parity against the live map.
