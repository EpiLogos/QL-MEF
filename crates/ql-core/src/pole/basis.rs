//! The ratified elemental carrier basis — one material-element identity from
//! M2 vibration through M3 form into M4 composition.
//!
//! Semantics are RATIFIED (M4 ratification 5; fold-grammar s12 hardening):
//!
//! ```text
//! A = Water, T = Fire, C = Earth, G = Air
//! [w, x, y, z] = [Earth, Fire, Water, Air]
//! q = w_E + x_F·i + y_W·j + z_A·k
//! ```
//!
//! Contrary orderings are drift, not open design alternatives: the type is
//! sealed so no permuted basis can be constructed; conformance tests pin the
//! nucleotide, fibre and quaternion-component relations.
//!
//! The carrier counts and the epogdoon continuity are the M2→M3 handoff:
//!
//! ```text
//! 72 = 4 × 18   --8/9-->   64 = 4 × 16
//! 18 × 20° = 16 × 22.5° = 360°,   9 × 20° = 8 × 22.5° = 180°
//! 20° × 9/8 = 22.5°
//! ```
//!
//! The same epogdoon that operates as the musical interval operates as the
//! conjugate count/aperture transform through which M2 modal differentiation
//! becomes M3 determinate form (integrated-object s1, acceptance criteria 2-3).

use super::nucleotide::Nucleotide;

/// The four differentiated material elements of the physical pole.
///
/// Canonical component order is [Earth, Fire, Water, Air]. The fifth element
/// (Ākāśa/quintessence) belongs to the potentiating whole and is deliberately
/// absent here: the 72→64 material transduction is carried by the four
/// differentiated material elements only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Element {
    Earth,
    Fire,
    Water,
    Air,
}

impl Element {
    /// Ratified canonical component order — [w, x, y, z].
    pub const CANONICAL_ORDER: [Element; 4] =
        [Element::Earth, Element::Fire, Element::Water, Element::Air];

    pub const ALL: [Element; 4] = Self::CANONICAL_ORDER;

    /// Quaternion component slot: w=Earth, x=Fire, y=Water, z=Air.
    pub const fn component_index(self) -> usize {
        match self {
            Element::Earth => 0,
            Element::Fire => 1,
            Element::Water => 2,
            Element::Air => 3,
        }
    }

    /// The M3 nucleotide carrying this element (C/T/A/G in suit order).
    pub const fn nucleotide(self) -> Nucleotide {
        match self {
            Element::Earth => Nucleotide::C,
            Element::Fire => Nucleotide::T,
            Element::Water => Nucleotide::A,
            Element::Air => Nucleotide::G,
        }
    }

    /// The M2 elemental fibre carrying this element, in canonical fibre order.
    pub const fn fibre_index(self) -> usize {
        self.component_index()
    }

    pub const fn name(self) -> &'static str {
        match self {
            Element::Earth => "Earth",
            Element::Fire => "Fire",
            Element::Water => "Water",
            Element::Air => "Air",
        }
    }
}

/// Quaternion components aligned to the ratified basis.
///
/// `q = w_E + x_F·i + y_W·j + z_A·k` — w carries Earth, x Fire, y Water,
/// z Air. Values are generic kernel-quantities (charges, indices, weights);
/// interpretation belongs to the consuming layer (M4 composition).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct QuaternionComponents {
    /// Earth component.
    pub w: i32,
    /// Fire component (i axis).
    pub x: i32,
    /// Water component (j axis).
    pub y: i32,
    /// Air component (k axis).
    pub z: i32,
}

impl QuaternionComponents {
    /// Read a component by element.
    pub const fn by_element(self, element: Element) -> i32 {
        match element {
            Element::Earth => self.w,
            Element::Fire => self.x,
            Element::Water => self.y,
            Element::Air => self.z,
        }
    }
}

/// The sealed ratified basis.
///
/// There is exactly one basis. `canonical()` is the only constructor, so a
/// permuted elemental order (drift) cannot be expressed as a basis value —
/// enforcing M4 ratification 5 by construction, with conformance tests pinning
/// the relations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementalQuaternionBasis {
    _sealed: (),
}

impl ElementalQuaternionBasis {
    /// The one ratified basis: A=Water, T=Fire, C=Earth, G=Air;
    /// [w,x,y,z]=[Earth,Fire,Water,Air].
    pub const fn canonical() -> Self {
        Self { _sealed: () }
    }

    /// Canonical component order of the quaternion composition.
    pub const fn component_order(&self) -> [Element; 4] {
        Element::CANONICAL_ORDER
    }

    /// M3 nucleotide relation: which nucleotide carries an element.
    pub const fn nucleotide_of(&self, element: Element) -> Nucleotide {
        element.nucleotide()
    }

    /// M3 form-family relation: which element a nucleotide carries.
    pub const fn element_of(&self, nucleotide: Nucleotide) -> Element {
        match nucleotide {
            Nucleotide::A => Element::Water,
            Nucleotide::T => Element::Fire,
            Nucleotide::C => Element::Earth,
            Nucleotide::G => Element::Air,
        }
    }

    /// M2 fibre relation: the elemental fibre index of an element
    /// (`4×18` state: `fibre_index × 18 + state`).
    pub const fn fibre_index_of(&self, element: Element) -> usize {
        element.fibre_index()
    }

    /// M4 composition: align four element-ordered quantities into the
    /// ratified quaternion components (w=Earth, x=Fire, y=Water, z=Air).
    pub const fn components(
        &self,
        earth: i32,
        fire: i32,
        water: i32,
        air: i32,
    ) -> QuaternionComponents {
        QuaternionComponents {
            w: earth,
            x: fire,
            y: water,
            z: air,
        }
    }
}

/// Per-element fibre/state counts of the M2→M3 handoff.
pub mod carrier {
    use super::super::codon::AngleDeg10;

    /// M2 vibrational templateure states per elemental fibre (4×18 = 72).
    pub const FIBRE_STATES: usize = 18;
    /// M3 determinate form states per elemental fibre (4×16 = 64).
    pub const FORM_STATES: usize = 16;
    /// Total material carriers.
    pub const ELEMENTS: usize = 4;

    /// The Second Spanda totality: 100 = 36 + 64 = 4(3²+4²) = 4×5², with the
    /// pentadic quotient 100/5 = 20 — the M2 fibre quantum in degrees
    /// (`M2-C23` pentadic elemental aperture).
    pub const SECOND_SPANDA_TOTAL: u32 = 100;
    pub const PENTADIC_QUOTIENT: u32 = 20;

    /// The M2 elemental-vibrational fibre quantum: 360°/18 = 20°.
    pub const FIBRE_QUANTUM_DEG10: i32 = AngleDeg10::FULL_TURN_DEG10 / 18;

    /// The M3 form/fold aperture quantum: 360°/16 = 22.5°.
    pub const FORM_QUANTUM_DEG10: i32 = AngleDeg10::FULL_TURN_DEG10 / 16;
}

/// The element-preserving epogdoon transduction `T18→16` (`M2-C24`).
///
/// Fibrewise law following the kernel's discrete fold-back pattern at the
/// epogdoon tax (2 folded states per fibre): source states 0..16 map 1:1; the
/// two tax states 16 and 17 fold onto targets 0 and 8 (stride 16/2). The
/// total `T72→64 = I4 ⊗ T18→16` preserves the element through the transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transduction18to16(u8);

impl Transduction18to16 {
    /// The two folded tax states per fibre.
    pub const FOLDED_SOURCES: [u8; 2] = [16, 17];
    /// Fold stride: 16 / 2.
    pub const FOLD_STRIDE: u8 = 8;

    pub fn new(source: u8) -> Result<Self, crate::QlError> {
        if source < carrier::FIBRE_STATES as u8 {
            Ok(Self(source))
        } else {
            Err(crate::QlError::InvalidPoleValue {
                field: "f18-source",
                value: source as u32,
            })
        }
    }

    pub const fn source(self) -> u8 {
        self.0
    }

    /// Target form state under the fibrewise epogdoon.
    pub const fn target(self) -> u8 {
        if self.0 < carrier::FORM_STATES as u8 {
            self.0
        } else {
            (self.0 - carrier::FORM_STATES as u8) * Self::FOLD_STRIDE
        }
    }

    /// True when the source is one of the two folded tax states.
    pub const fn is_folded(self) -> bool {
        self.0 >= carrier::FORM_STATES as u8
    }
}

/// The flat discrete-address shadow of the transduction (the kernel DET):
/// 72 sources → 64 targets, sources 64..72 folding back onto every 8th
/// target ((i−64)×8).
///
/// The two descriptions — fibrewise tensor `I4⊗T18→16` and flat DET — are
/// the two descriptions of the many-to-one map and must not be collapsed
/// into one gap count (fold-grammar s12 two-descriptions law).
pub const fn det_shadow(source: u8) -> Result<u8, crate::QlError> {
    if source < 72 {
        Ok(if source < 64 {
            source
        } else {
            (source - 64) * 8
        })
    } else {
        Err(crate::QlError::InvalidPoleValue {
            field: "det-source",
            value: source as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pole::codon::AngleDeg10;

    #[test]
    fn ratified_nucleotide_element_law() {
        let basis = ElementalQuaternionBasis::canonical();
        assert_eq!(basis.element_of(Nucleotide::A), Element::Water);
        assert_eq!(basis.element_of(Nucleotide::T), Element::Fire);
        assert_eq!(basis.element_of(Nucleotide::C), Element::Earth);
        assert_eq!(basis.element_of(Nucleotide::G), Element::Air);
        for element in Element::ALL {
            assert_eq!(basis.element_of(basis.nucleotide_of(element)), element);
        }
    }

    #[test]
    fn canonical_component_order_is_ratified() {
        let basis = ElementalQuaternionBasis::canonical();
        assert_eq!(
            basis.component_order(),
            [Element::Earth, Element::Fire, Element::Water, Element::Air]
        );
        // q = w_E + x_F·i + y_W·j + z_A·k
        let components = basis.components(1, 2, 3, 4);
        assert_eq!(components.by_element(Element::Earth), 1);
        assert_eq!(components.by_element(Element::Fire), 2);
        assert_eq!(components.by_element(Element::Water), 3);
        assert_eq!(components.by_element(Element::Air), 4);
    }

    #[test]
    fn carrier_counts_are_four_eighteen_to_four_sixteen() {
        use carrier::*;
        assert_eq!(ELEMENTS * FIBRE_STATES, 72);
        assert_eq!(ELEMENTS * FORM_STATES, 64);
        // Second Spanda: 100 = 4(3²+4²) = 4×5²; 100/5 = 20 = the fibre quantum.
        assert_eq!(SECOND_SPANDA_TOTAL, 4 * (3 * 3 + 4 * 4));
        assert_eq!(SECOND_SPANDA_TOTAL / 5, PENTADIC_QUOTIENT);
        assert_eq!(FIBRE_QUANTUM_DEG10, PENTADIC_QUOTIENT as i32 * 10);
    }

    #[test]
    fn epogdoon_continuity_as_counts_and_angles() {
        use carrier::*;
        // Count register: 16/18 = 8/9.
        assert_eq!(FORM_STATES * 9, FIBRE_STATES * 8);
        // Angle register: 20° × 9/8 = 22.5°.
        assert_eq!(FIBRE_QUANTUM_DEG10 * 9, FORM_QUANTUM_DEG10 * 8);
        assert_eq!(FIBRE_QUANTUM_DEG10 * 9 / 8, 225, "20° × 9/8 = 22.5°");
        // Shared closures: 9 × 20° = 8 × 22.5° = 180°.
        assert_eq!(9 * FIBRE_QUANTUM_DEG10, AngleDeg10::HALF_TURN_DEG10);
        assert_eq!(8 * FORM_QUANTUM_DEG10, AngleDeg10::HALF_TURN_DEG10);
        // Full turns: 18 × 20° = 16 × 22.5° = 360°.
        assert_eq!(18 * FIBRE_QUANTUM_DEG10, AngleDeg10::FULL_TURN_DEG10);
        assert_eq!(16 * FORM_QUANTUM_DEG10, AngleDeg10::FULL_TURN_DEG10);
    }

    #[test]
    fn fibrewise_transduction_preserves_element_and_folds_the_tax() {
        for source in 0u8..18 {
            let t = Transduction18to16::new(source).expect("fibre source");
            if source < 16 {
                assert_eq!(t.target(), source);
                assert!(!t.is_folded());
            } else {
                assert!(t.is_folded());
            }
        }
        assert_eq!(Transduction18to16::new(16).unwrap().target(), 0);
        assert_eq!(Transduction18to16::new(17).unwrap().target(), 8);
        assert!(Transduction18to16::new(18).is_err());
    }

    #[test]
    fn det_shadow_mirrors_the_kernel_fold_back() {
        for source in 0u8..64 {
            assert_eq!(det_shadow(source).unwrap(), source);
        }
        for (source, target) in (64u8..72).enumerate() {
            assert_eq!(det_shadow(target).unwrap(), (source as u8) * 8);
        }
        assert!(det_shadow(72).is_err());
    }

    #[test]
    fn tensor_and_det_stay_two_descriptions() {
        // The element-preserving tensor map: (element, state) → 16·e + T18→16(s).
        let mut tensor_targets = std::collections::HashSet::new();
        let mut tensor_collisions = 0usize;
        for element in 0usize..4 {
            for source in 0u8..18 {
                let target =
                    element * 16 + Transduction18to16::new(source).unwrap().target() as usize;
                if !tensor_targets.insert(target) {
                    tensor_collisions += 1;
                }
            }
        }
        assert_eq!(tensor_targets.len(), 64);
        assert_eq!(
            tensor_collisions, 8,
            "the epogdoon tax: 8 folded collisions"
        );
        // The flat DET shadow folds its own 8 sources — the same tax, a
        // different description; both are kept, neither collapsed.
        let mut det = std::collections::HashSet::new();
        for source in 0u8..72 {
            det.insert(det_shadow(source).unwrap());
        }
        assert_eq!(det.len(), 64, "72 sources resolve onto 64 targets");
    }
}
