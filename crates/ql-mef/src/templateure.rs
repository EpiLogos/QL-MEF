//! Vibrational templateure — the M2 modal form-potential field (T4).
//!
//! Paraśakti supplies a field of lawful resonant/form possibility before
//! Mahāmāyā resolves a single determinate address. The elemental-fibre state
//! space is the direct sum
//!
//! ```text
//! V72 = V18(Earth) ⊕ V18(Fire) ⊕ V18(Water) ⊕ V18(Air)
//! ```
//!
//! and the form-facing transform is
//!
//! ```text
//! T72→64 = I4 ⊗ T18→16
//! ```
//!
//! into `R64 = R16(Earth) ⊕ R16(Fire) ⊕ R16(Water) ⊕ R16(Air)`. The field
//! carries **amplitudes**, not a winning index: the determinate 64-form is a
//! downstream resolution (the discrete-address shadow), never the store.
//!
//! The analytic-signal (quadrature) operator advances every amplitude's phase
//! by +90° — the recognition operator of the M2-4/M2-5 sonic layer. Over
//! integer complex amplitudes it is exact: multiplication by i, with power
//! conserved and order-4. One resonator state produces both the sounded field
//! and the deformation potential — acceptance criterion 9 at the state level:
//! the quadrature operator commutes with the transduction, so the sounded and
//! form-facing readings stay the same field.

use ql_core::{Element, ElementalQuaternionBasis, Transduction18to16, carrier};

/// Semantic identity of the templateure field contract.
pub const TEMPLATEURE_FIELD_VERSION: &str = "1.0.0";
pub const TEMPLATEURE_FIELD_CONTRACT_REF: &str = "ql.mef.templateure-field/v1";

/// An exact integer complex amplitude — one modal coefficient.
///
/// Generic kernel units; the phase is carried implicitly by (re, im), which
/// keeps the quadrature operator exact (no floating point in the field law).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Amplitude {
    pub re: i64,
    pub im: i64,
}

impl Amplitude {
    pub const ZERO: Self = Self { re: 0, im: 0 };

    pub const fn new(re: i64, im: i64) -> Self {
        Self { re, im }
    }

    pub const fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    /// Power |z|² = re² + im² — the intensity of the mode.
    pub const fn power(self) -> u128 {
        let re = self.re as i128;
        let im = self.im as i128;
        (re * re + im * im) as u128
    }

    /// The analytic-signal step on one amplitude: rotate +90° (multiply by i).
    /// Exact: (re, im) → (−im, re). Order 4; power conserved.
    pub const fn quadrature(self) -> Self {
        Self {
            re: -self.im,
            im: self.re,
        }
    }
}

/// One elemental fibre of the templateure: 18 modal amplitudes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FibreField {
    element: Element,
    amplitudes: [Amplitude; carrier::FIBRE_STATES],
}

impl FibreField {
    pub fn new(element: Element, amplitudes: [Amplitude; carrier::FIBRE_STATES]) -> Self {
        Self {
            element,
            amplitudes,
        }
    }

    pub const fn element(&self) -> Element {
        self.element
    }

    pub const fn amplitude(&self, state: u8) -> Amplitude {
        self.amplitudes[state as usize]
    }

    /// The fibre's quadrature image.
    pub fn quadrature(&self) -> Self {
        Self {
            element: self.element,
            amplitudes: self.amplitudes.map(|a| a.quadrature()),
        }
    }

    pub const fn total_power(&self) -> u128 {
        let mut total = 0u128;
        let mut i = 0;
        while i < self.amplitudes.len() {
            total += self.amplitudes[i].power();
            i += 1;
        }
        total
    }
}

/// The vibrational templateure field: V72 as the direct sum of four V18.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateureField {
    fibres: [FibreField; carrier::ELEMENTS],
}

impl TemplateureField {
    /// Assemble the field in canonical elemental fibre order.
    pub fn new(fibres: [FibreField; carrier::ELEMENTS]) -> Result<Self, ql_core::QlError> {
        let basis = ElementalQuaternionBasis::canonical();
        for (index, fibre) in fibres.iter().enumerate() {
            if basis.fibre_index_of(fibre.element()) != index {
                return Err(ql_core::QlError::InvalidPoleValue {
                    field: "fibre-order",
                    value: index as u32,
                });
            }
        }
        Ok(Self { fibres })
    }

    /// Build a field from per-element amplitude arrays in canonical order.
    pub fn from_amplitudes(
        earth: [Amplitude; carrier::FIBRE_STATES],
        fire: [Amplitude; carrier::FIBRE_STATES],
        water: [Amplitude; carrier::FIBRE_STATES],
        air: [Amplitude; carrier::FIBRE_STATES],
    ) -> Self {
        Self {
            fibres: [
                FibreField::new(Element::Earth, earth),
                FibreField::new(Element::Fire, fire),
                FibreField::new(Element::Water, water),
                FibreField::new(Element::Air, air),
            ],
        }
    }

    pub const fn fibre(&self, element: Element) -> &FibreField {
        &self.fibres[element.fibre_index()]
    }

    /// The analytic-signal (quadrature) operator: every amplitude advances
    /// +90°. Power is conserved; applied four times it is the identity.
    pub fn quadrature(&self) -> Self {
        let fibres = self.fibres;
        Self {
            fibres: fibres.map(|f| f.quadrature()),
        }
    }

    /// Total field power — invariant under the quadrature operator.
    pub const fn total_power(&self) -> u128 {
        let mut total = 0u128;
        let mut i = 0;
        while i < self.fibres.len() {
            total += self.fibres[i].total_power();
            i += 1;
        }
        total
    }

    /// The form-facing transform `T72→64 = I4 ⊗ T18→16` with amplitudes:
    /// identity sources carry their amplitude; the two folded tax states add
    /// onto their fold targets. The element is preserved (I4).
    pub fn transduce(&self) -> FormPotential {
        let mut fibres = [[Amplitude::ZERO; carrier::FORM_STATES]; carrier::ELEMENTS];
        for (row, fibre) in fibres.iter_mut().zip(self.fibres.iter()) {
            for s in 0..carrier::FIBRE_STATES {
                let target = Transduction18to16::new(s as u8)
                    .expect("fibre state in range")
                    .target() as usize;
                row[target] = row[target].add(fibre.amplitude(s as u8));
            }
        }
        FormPotential { fibres }
    }
}

/// The form potential R64: 4 × 16 amplitudes — the modal storehouse the
/// determinate 64 resolves from, carried as amplitudes, not a winning index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormPotential {
    fibres: [[Amplitude; carrier::FORM_STATES]; carrier::ELEMENTS],
}

impl FormPotential {
    pub const fn amplitude(&self, element: Element, state: u8) -> Amplitude {
        self.fibres[element.fibre_index()][state as usize]
    }

    /// Total power of the potential.
    pub const fn total_power(&self) -> u128 {
        let mut total = 0u128;
        let mut f = 0;
        while f < self.fibres.len() {
            let mut s = 0;
            while s < carrier::FORM_STATES {
                total += self.fibres[f][s].power();
                s += 1;
            }
            f += 1;
        }
        total
    }

    /// The quadrature operator on the form potential — the same analytic
    /// signal law on the R64 side of the seam.
    pub fn quadrature(&self) -> Self {
        let mut fibres = self.fibres;
        for row in &mut fibres {
            *row = row.map(Amplitude::quadrature);
        }
        Self { fibres }
    }

    /// The discrete-address shadow: the most intense (element, state) pair.
    /// The winning form is a projection of the field, never the store
    /// (the DET's discrete-address projection of the same seam).
    pub fn winning(&self) -> (Element, u8) {
        let mut best = (Element::Earth, 0u8);
        let mut best_power = 0u128;
        for element in Element::ALL {
            for state in 0u8..carrier::FORM_STATES as u8 {
                let power = self.amplitude(element, state).power();
                if power > best_power {
                    best_power = power;
                    best = (element, state);
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_fibre(element: Element, lit: u8) -> FibreField {
        let mut amplitudes = [Amplitude::ZERO; carrier::FIBRE_STATES];
        amplitudes[lit as usize] = Amplitude::new(3, -4); // power 25
        FibreField::new(element, amplitudes)
    }

    #[test]
    fn quadrature_is_exact_order_four_and_power_conserving() {
        let a = Amplitude::new(3, -4);
        assert_eq!(a.power(), 25);
        let q1 = a.quadrature();
        assert_eq!((q1.re, q1.im), (4, 3));
        let q2 = q1.quadrature();
        assert_eq!((q2.re, q2.im), (-3, 4), "two steps = the antipode");
        let q4 = q2.quadrature().quadrature();
        assert_eq!(q4, a, "four steps return the identity");
        assert_eq!(q1.power(), a.power());
    }

    #[test]
    fn field_assembles_in_canonical_fibre_order() {
        let field = TemplateureField::from_amplitudes(
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
        );
        assert_eq!(field.fibre(Element::Earth).element(), Element::Earth);
        assert_eq!(field.fibre(Element::Air).element(), Element::Air);
        // A mis-ordered fibre assembly is rejected.
        let mut fibres = [
            unit_fibre(Element::Earth, 0),
            unit_fibre(Element::Fire, 0),
            unit_fibre(Element::Water, 0),
            unit_fibre(Element::Air, 0),
        ];
        fibres.swap(0, 1);
        assert!(TemplateureField::new(fibres).is_err());
    }

    #[test]
    fn transduction_preserves_element_and_folds_amplitudes() {
        // Earth fibre lit at states 0 and 16 → both fold onto target 0.
        let mut earth = [Amplitude::ZERO; 18];
        earth[0] = Amplitude::new(1, 0);
        earth[16] = Amplitude::new(0, 2);
        let field = TemplateureField::from_amplitudes(
            earth,
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
        );
        let potential = field.transduce();
        // Folded addition: target 0 carries 1 + 2i (power 5, not 1+4).
        let fused = potential.amplitude(Element::Earth, 0);
        assert_eq!((fused.re, fused.im), (1, 2));
        assert_eq!(fused.power(), 5);
        // Every other Earth target stays dark except the untouched ones.
        assert_eq!(potential.amplitude(Element::Earth, 5), Amplitude::ZERO);
        // The other fibres are untouched.
        assert_eq!(potential.amplitude(Element::Fire, 0), Amplitude::ZERO);
    }

    #[test]
    fn quadrature_commutes_with_the_transduction() {
        // Acceptance criterion 9 (state level): the sounded field and the
        // deformation potential are the same field — the recognition
        // operator commutes with the form-facing transform.
        let mut fire = [Amplitude::ZERO; 18];
        fire[2] = Amplitude::new(5, 1);
        fire[17] = Amplitude::new(-2, 3);
        let field = TemplateureField::from_amplitudes(
            [Amplitude::ZERO; 18],
            fire,
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
        );
        let left = field.quadrature().transduce();
        let right = field.transduce();
        let right = right.quadrature();
        for element in Element::ALL {
            for state in 0u8..16 {
                assert_eq!(
                    left.amplitude(element, state),
                    right.amplitude(element, state),
                    "commutation at {element:?}/{state}"
                );
            }
        }
    }

    #[test]
    fn winning_index_is_a_projection_not_the_store() {
        let mut water = [Amplitude::ZERO; 18];
        water[9] = Amplitude::new(100, 0);
        let field = TemplateureField::from_amplitudes(
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
            water,
            [Amplitude::ZERO; 18],
        );
        let potential = field.transduce();
        let (element, state) = potential.winning();
        assert_eq!(element, Element::Water);
        assert_eq!(state, 9);
        assert!(potential.total_power() > 0);
    }

    #[test]
    fn total_power_is_invariant_under_quadrature() {
        let mut air = [Amplitude::ZERO; 18];
        air[0] = Amplitude::new(1, 2);
        air[7] = Amplitude::new(-3, 4);
        let field = TemplateureField::from_amplitudes(
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
            air,
        );
        assert_eq!(field.quadrature().total_power(), field.total_power());
    }
}
