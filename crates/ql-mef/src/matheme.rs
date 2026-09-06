use ql_core::{
    CanonicalCrossPass, QlAddress, QlFace, QlFamily, QlPosition, RelationalSixfold, SixBySixField,
    WHOLE_ANCHOR_SYMBOL, canonical_cross_pass_d1,
};

use crate::{CANONICAL_RATIOS, HarmonicRatio, SECOND_SPANDA_VERTICAL};

/// Version of `fixtures/kernel/matheme-derivation-contract-v1.tsv`, the
/// machine-readable conformance boundary for this module.
pub const MATHEME_DERIVATION_CONTRACT_VERSION: &str = "1.0.0";

/// The matheme derivation is the definitional 0-layer; the kernel
/// coordinates remain the governing 1.
pub const MATHEME_DERIVATION_LAYER: u8 = 0;

/// The holographic kernel contract's own composition top line, unchanged.
pub const TOP_LINE: &str = "# / 0/1 <-> 1/0";

/// One same-position circuit `n <-> n'` carries two coordinates (D1).
pub const CIRCUIT_COORDINATES: usize = 2;

/// One circuit is one 360-degree traversal of the double cover.
pub const CIRCUIT_DEGREES: u32 = 360;

/// The identity `n -> n' -> n` is the double beat: two turns.
pub const DOUBLE_BEAT_TURNS: u32 = 2;

/// Recognition is structural, not prose: identity restored only after the
/// double beat, never after one.
pub const RECOGNITION_DEGREES: u32 = CIRCUIT_DEGREES * DOUBLE_BEAT_TURNS;

/// The copula direction is the relational sixfold's whole/return anchor.
pub const COPULA_SYMBOL: &str = WHOLE_ANCHOR_SYMBOL;

/// The retained One of `1 + 64 + 72`: the standing whole `1/1` counted once.
pub const RETAINED_ONE: u32 = 1;

/// The top line bound to existing kernel elements.
///
/// `#` is the hash bedrock the kernel already names `NONE`; the two
/// slash-readings are the two faces of one position. No new primitive is
/// introduced by the binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MathemeTopLine {
    pub hash: QlFamily,
    pub copula: QlFace,
    pub return_switch: QlFace,
}

impl MathemeTopLine {
    pub const fn canonical() -> Self {
        Self {
            hash: QlFamily::None,
            copula: QlFace::Direct,
            return_switch: QlFace::Conjugate,
        }
    }
}

/// Eq1 — the anchor traversal law.
///
/// One circuit is the existing same-position conjugation operator; the
/// double beat is that circuit composed with itself, which restores the
/// address exactly when taken twice.
pub mod eq1 {
    use super::*;

    /// One circuit `n <-> n'` under the existing cross.same-position
    /// relation. This is `canonical_cross_pass_d1`, not a new relation.
    pub fn one_circuit(position: QlPosition) -> CanonicalCrossPass {
        canonical_cross_pass_d1(position)
    }

    /// One beat of the double cover: the face flips, the position is
    /// preserved (same-position conjugation).
    pub fn beat(address: QlAddress) -> QlAddress {
        address.with_face(address.face().conjugate())
    }

    /// The double beat `n -> n' -> n`: the address restored to itself after
    /// both turns — 720 degrees as recognition.
    pub fn double_beat(address: QlAddress) -> QlAddress {
        beat(beat(address))
    }

    /// The standing whole `(0/1)+(1/0) = 1/1` from the canonical ratios.
    pub fn standing_whole() -> HarmonicRatio {
        CANONICAL_RATIOS[0]
    }
}

/// Eq2 — the position field as the decomposed 100%.
pub mod eq2 {
    use super::*;

    /// The position hexad: four twos + two threes (the Second-Spanda 4+2)
    /// and the registry's local-positions-per-lens are one count.
    pub fn position_hexad() -> u32 {
        u32::from(SECOND_SPANDA_VERTICAL.0 + SECOND_SPANDA_VERTICAL.1)
    }

    /// `2^6`: the hexad read in the binary register.
    pub fn binary_register() -> u32 {
        2u32.pow(position_hexad())
    }

    /// `6^2`: the hexad read against itself — computed as the six-by-six
    /// field's own address cardinality.
    pub fn self_register() -> u32 {
        SixBySixField::canonical().addresses.len() as u32
    }

    /// The decomposed totality `2^6 + 6^2 = 100`.
    pub fn decomposed_totality() -> u32 {
        binary_register() + self_register()
    }

    /// The totality ratio `64/36 = 16/9` in lowest terms.
    pub fn totality_ratio() -> HarmonicRatio {
        HarmonicRatio::new(binary_register(), self_register()).expect("both registers are non-zero")
    }

    /// The 12-ring: the doubled hexad `6+6` (direct + prime faces of the
    /// relational sixfold), which the MEF registry counts as its lenses.
    pub fn twelve_ring() -> u32 {
        RelationalSixfold::canonical().sites.len() as u32 * 2
    }

    /// The ring's own aspect `12:6 = 2:1` — the octave of the position field.
    pub fn ring_octave() -> HarmonicRatio {
        HarmonicRatio::new(twelve_ring(), position_hexad()).expect("hexad and ring are non-zero")
    }

    /// The field cardinality `12 x 6 = 72`, computed — the kernel registry's
    /// `mef.address-count`, never restated.
    pub fn field_cardinality() -> u32 {
        twelve_ring() * position_hexad()
    }
}

/// Eq3 — the cardinalities with the retained One, and the two-way door.
pub mod eq3 {
    use super::*;

    /// The two-way door's traversal step: the epogdoon `9/8` (ascent) from
    /// the canonical ratio field; the descent `8/9` is its reciprocal.
    pub fn epogdoon() -> HarmonicRatio {
        CANONICAL_RATIOS[6]
    }

    /// Descent through the door: `72 x 8/9 = 64`, exact rational arithmetic.
    pub fn door_descent() -> HarmonicRatio {
        HarmonicRatio::new(super::eq2::field_cardinality(), 1)
            .expect("field cardinality is non-zero")
            .multiply(epogdoon().reciprocal())
    }

    /// Ascent through the door: `64 x 9/8 = 72`, exact.
    pub fn door_ascent() -> HarmonicRatio {
        HarmonicRatio::new(super::eq2::binary_register(), 1)
            .expect("binary register is non-zero")
            .multiply(epogdoon())
    }

    /// The octave returns through, not by eliminating, its remainder:
    /// `16/9 x 9/8 = 2/1`.
    pub fn octave_through_door() -> HarmonicRatio {
        super::eq2::totality_ratio().multiply(epogdoon())
    }

    /// `1 + 64 + 72 = 137`: the retained One, the binary register, and the
    /// field cardinality.
    pub fn cardinality_sum() -> u32 {
        RETAINED_ONE + super::eq2::binary_register() + super::eq2::field_cardinality()
    }
}

/// The complete 0-layer reading of one derivation, assembled from the
/// equation modules above. `circuits` carries the six D1 passes so callers
/// can inspect the traversal without re-deriving it.
#[derive(Debug, Clone, PartialEq)]
pub struct MathemeDerivation {
    pub contract_version: &'static str,
    pub layer: u8,
    pub top_line: MathemeTopLine,
    pub circuits: Vec<CanonicalCrossPass>,
    pub standing_whole: HarmonicRatio,
    pub position_hexad: u32,
    pub binary_register: u32,
    pub self_register: u32,
    pub decomposed_totality: u32,
    pub totality_ratio: HarmonicRatio,
    pub twelve_ring: u32,
    pub ring_octave: HarmonicRatio,
    pub field_cardinality: u32,
    pub retained_one: u32,
    pub cardinality_sum: u32,
    pub door_descent: HarmonicRatio,
    pub door_ascent: HarmonicRatio,
    pub octave_through_door: HarmonicRatio,
}

pub use eq1::{beat, double_beat, one_circuit, standing_whole};
pub use eq2::{
    binary_register, decomposed_totality, field_cardinality, position_hexad, ring_octave,
    self_register, totality_ratio, twelve_ring,
};
pub use eq3::{cardinality_sum, door_ascent, door_descent, epogdoon, octave_through_door};

/// Derive the complete matheme 0-layer from the existing kernel elements.
pub fn derive_matheme() -> MathemeDerivation {
    MathemeDerivation {
        contract_version: MATHEME_DERIVATION_CONTRACT_VERSION,
        layer: MATHEME_DERIVATION_LAYER,
        top_line: MathemeTopLine::canonical(),
        circuits: (0_u8..6)
            .map(|value| {
                eq1::one_circuit(QlPosition::new(value).expect("positions are modulo six"))
            })
            .collect(),
        standing_whole: eq1::standing_whole(),
        position_hexad: eq2::position_hexad(),
        binary_register: eq2::binary_register(),
        self_register: eq2::self_register(),
        decomposed_totality: eq2::decomposed_totality(),
        totality_ratio: eq2::totality_ratio(),
        twelve_ring: eq2::twelve_ring(),
        ring_octave: eq2::ring_octave(),
        field_cardinality: eq2::field_cardinality(),
        retained_one: RETAINED_ONE,
        cardinality_sum: eq3::cardinality_sum(),
        door_descent: eq3::door_descent(),
        door_ascent: eq3::door_ascent(),
        octave_through_door: eq3::octave_through_door(),
    }
}
