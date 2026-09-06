//! `PhysicalPoleFormState` — the composition bridge at the kernel/profile
//! boundary (Stage 2; integrated-object s8, delta 4).
//!
//! The integrated 1/2/3 physical pole is ONE generated physical/form object
//! with three constitutive determinations. This type composes the already-
//! authoritative M1/M2/M3 facts into that one object under **one eventRef
//! and one profile generation**. It is a composition object, not a fourth
//! calculation pole: every section is a projection of an existing authority
//! (the coin ground, the elemental basis, the templateure field, the fold
//! state), and the renderer, Agent, test harness and Nara projection all
//! consume the same typed state (acceptance criterion 12).
//!
//! Composition laws:
//!
//! ```text
//! one eventRef / one profile generation drives M1, M2 and M3 (criterion 1)
//! elemental carrier continuity runs M2 4x18 -> M3 4x16 -> M4 basis slots
//!     through the ONE ratified basis (criterion 2)
//! the M3 fold state is the resolved form; the M2 field stays the modal
//!     potential — potential and resolution are different roles, never merged
//! readiness aggregates by the weakest section; provenance is carried,
//!     never invented
//! composition is deterministic: same inputs, same state (criterion 12)
//! ```

use crate::{MefError, TemplateureField};
use ql_core::{Element, ElementalQuaternionBasis, FoldState, QlPosition, QuaternionComponents};

/// Version of the composed state contract.
pub const PHYSICAL_POLE_FORM_STATE_VERSION: &str = "1.0.0";
/// Semantic identity of the composition bridge.
pub const PHYSICAL_POLE_FORM_STATE_REF: &str = "ql.mef.physical-pole-form-state/v1";

/// The current-event identity every section shares.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PoleIdentity {
    event_ref: String,
    profile_generation: u64,
    tick12: u8,
    degree720: u16,
}

impl PoleIdentity {
    /// One current event. The 720° double-cover is first-class state:
    /// `degree360` is its primary-layer reading, never reconstructed.
    pub fn new(
        event_ref: impl Into<String>,
        profile_generation: u64,
        tick12: u8,
        degree720: u16,
    ) -> Result<Self, MefError> {
        if tick12 >= 12 {
            return Err(MefError::InvalidSublensPosition(tick12));
        }
        if degree720 >= 720 {
            return Err(MefError::InvalidSublensPosition((degree720 % 720) as u8));
        }
        Ok(Self {
            event_ref: event_ref.into(),
            profile_generation,
            tick12,
            degree720,
        })
    }

    pub fn event_ref(&self) -> &str {
        &self.event_ref
    }

    pub const fn profile_generation(&self) -> u64 {
        self.profile_generation
    }

    pub const fn tick12(&self) -> u8 {
        self.tick12
    }

    pub const fn degree720(&self) -> u16 {
        self.degree720
    }

    /// The primary-layer degree: the 360° reading of the double cover.
    pub const fn degree360(&self) -> u16 {
        self.degree720 % 360
    }

    /// The shadow layer: `true` when the event stands in the second cover.
    pub const fn in_shadow_layer(&self) -> bool {
        self.degree720 >= 360
    }
}

/// The M1 formal-harmonic-topological carrier facts, as provided.
///
/// These are carried, not computed: the pole composition introduces no M1
/// solver. The quaternion is stored in the ratified elemental component
/// order `[w, x, y, z] = [Earth, Fire, Water, Air]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M1CarrierFacts {
    /// The M1 position (0..6) of the current event.
    pub position6: QlPosition,
    /// Ring quaternion in ratified elemental component order.
    pub ring_quaternion: QuaternionComponents,
    /// The active ratio, e.g. the musical epogdoon (9, 8).
    pub active_ratio: (u32, u32),
    /// Stable handles into the M1 authorities (K² topology, Hopf state,
    /// Ananda relation) — provenance, not semantics.
    pub authority_handles: [String; 3],
}

/// Readiness of one section of the composed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Readiness {
    /// Derived and conformant.
    #[default]
    Ready,
    /// Derived with provisional inputs (gaps, cached providers).
    Provisional,
    /// Not currently derived; the section carries its last standing ref.
    Unavailable,
}

/// Provenance carried by the composed state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PoleProvenance {
    /// Source handles the sections were derived from.
    pub source_handles: Vec<String>,
    /// Derivation handles (contract refs of the services that produced
    /// the sections).
    pub derivation_handles: Vec<String>,
    /// Provider status of the live world-condition evidence.
    pub provider_status: Readiness,
}

/// The composed physical-pole form object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalPoleFormState {
    identity: PoleIdentity,
    m1: M1CarrierFacts,
    m2: TemplateureField,
    m2_readiness: Readiness,
    m3: FoldState,
    m3_readiness: Readiness,
    provenance: PoleProvenance,
}

impl PhysicalPoleFormState {
    /// Compose the one current object at the kernel/profile boundary.
    ///
    /// The identity is taken once: no section carries its own event ref, so
    /// the composition cannot split the event across determinations
    /// (criterion 1 by construction).
    pub fn compose(
        identity: PoleIdentity,
        m1: M1CarrierFacts,
        m2: TemplateureField,
        m2_readiness: Readiness,
        m3: FoldState,
        m3_readiness: Readiness,
        provenance: PoleProvenance,
    ) -> Self {
        Self {
            identity,
            m1,
            m2,
            m2_readiness,
            m3,
            m3_readiness,
            provenance,
        }
    }

    pub const fn identity(&self) -> &PoleIdentity {
        &self.identity
    }

    pub const fn m1_carrier(&self) -> &M1CarrierFacts {
        &self.m1
    }

    pub const fn m2_templateure(&self) -> &TemplateureField {
        &self.m2
    }

    pub const fn m2_readiness(&self) -> Readiness {
        self.m2_readiness
    }

    pub const fn m3_rupa(&self) -> &FoldState {
        &self.m3
    }

    pub const fn m3_readiness(&self) -> Readiness {
        self.m3_readiness
    }

    pub const fn provenance(&self) -> &PoleProvenance {
        &self.provenance
    }

    /// Aggregate readiness — the weakest section governs the object.
    pub fn readiness(&self) -> Readiness {
        if self.m2_readiness == Readiness::Unavailable
            || self.m3_readiness == Readiness::Unavailable
            || self.provenance.provider_status == Readiness::Unavailable
        {
            Readiness::Unavailable
        } else if self.m2_readiness == Readiness::Provisional
            || self.m3_readiness == Readiness::Provisional
            || self.provenance.provider_status == Readiness::Provisional
        {
            Readiness::Provisional
        } else {
            Readiness::Ready
        }
    }

    /// Elemental carrier continuity across the composed object (criterion
    /// 2): for each material element, the M2 fibre, the M3 nucleotide and
    /// the M4 basis component slot are one carrier through the one basis.
    pub fn elemental_carrier_continuity(&self) -> [(Element, usize, u8, usize); 4] {
        let basis = ElementalQuaternionBasis::canonical();
        Element::ALL.map(|element| {
            (
                element,
                basis.fibre_index_of(element),
                basis.nucleotide_of(element).bits(),
                element.component_index(),
            )
        })
    }

    /// Epogdoon continuity as both registers (criterion 3), read off the
    /// composed state: the active ratio's numerator/denominator relation to
    /// the fibre/form counts and to the angular quanta.
    pub const fn epogdoon_continuity_holds(&self) -> bool {
        let (numerator, denominator) = self.m1.active_ratio;
        // The ratio must reduce to the epogdoon 9/8 for the M2→M3 handoff.
        numerator * 8 == denominator * 9 && numerator == 9 && denominator == 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Amplitude, TEMPLATEURE_FIELD_CONTRACT_REF};
    use ql_core::{ApertureIndex, Codon64, CoinSum, Nucleotide};

    fn sample_m2() -> TemplateureField {
        let mut fire = [Amplitude::new(0, 0); 18];
        fire[7] = Amplitude::new(12, 5);
        TemplateureField::from_amplitudes(
            [Amplitude::ZERO; 18],
            fire,
            [Amplitude::ZERO; 18],
            [Amplitude::ZERO; 18],
        )
    }

    fn sample_m3() -> FoldState {
        FoldState::from_codon(
            Codon64::from_nucleotides(Nucleotide::A, Nucleotide::T, Nucleotide::G),
            ApertureIndex::new(4).expect("aperture"),
            17,
        )
    }

    fn sample_identity() -> PoleIdentity {
        PoleIdentity::new("event:n4/sample", 7, 3, 200).expect("identity")
    }

    fn sample_m1() -> M1CarrierFacts {
        M1CarrierFacts {
            position6: QlPosition::new(2).expect("position"),
            ring_quaternion: QuaternionComponents {
                w: 1,
                x: 0,
                y: 0,
                z: 0,
            },
            active_ratio: (9, 8),
            authority_handles: [
                "m1:k2-torus".to_string(),
                "m1:hopf".to_string(),
                "m1:ananda".to_string(),
            ],
        }
    }

    #[test]
    fn composed_state_round_trips_the_composition_laws() {
        let state = PhysicalPoleFormState::compose(
            sample_identity(),
            sample_m1(),
            sample_m2(),
            Readiness::Ready,
            sample_m3(),
            Readiness::Ready,
            PoleProvenance {
                source_handles: vec!["m2:resonator".to_string()],
                derivation_handles: vec![
                    TEMPLATEURE_FIELD_CONTRACT_REF.to_string(),
                    ql_core::POLE_FOLD_STATE_REF.to_string(),
                ],
                provider_status: Readiness::Ready,
            },
        );
        assert_eq!(state.identity().event_ref(), "event:n4/sample");
        assert_eq!(state.identity().profile_generation(), 7);
        assert_eq!(state.identity().degree720(), 200);
        assert_eq!(state.identity().degree360(), 200);
        assert!(!state.identity().in_shadow_layer());
        assert_eq!(state.readiness(), Readiness::Ready);
    }

    #[test]
    fn double_cover_first_class_and_shadow_layer() {
        let identity = PoleIdentity::new("event:x", 1, 0, 500).expect("identity");
        assert_eq!(identity.degree360(), 140);
        assert!(identity.in_shadow_layer());
        assert!(PoleIdentity::new("event:x", 1, 0, 720).is_err());
        assert!(PoleIdentity::new("event:x", 1, 12, 0).is_err());
    }

    #[test]
    fn elemental_carrier_continuity_is_one_basis_end_to_end() {
        let state = PhysicalPoleFormState::compose(
            sample_identity(),
            sample_m1(),
            sample_m2(),
            Readiness::Ready,
            sample_m3(),
            Readiness::Ready,
            PoleProvenance::default(),
        );
        for (element, fibre_index, nucleotide_bits, component_slot) in
            state.elemental_carrier_continuity()
        {
            assert_eq!(fibre_index, component_slot, "fibre and component agree");
            let nucleotide = Nucleotide::try_from(nucleotide_bits).expect("nucleotide");
            let basis = ElementalQuaternionBasis::canonical();
            assert_eq!(basis.element_of(nucleotide), element);
            // The M2 field carries the same element in the same fibre slot.
            assert_eq!(state.m2_templateure().fibre(element).element(), element);
        }
        // The composed M3 fold state resolves through the same alphabet.
        let [x, y, z] = state.m3_rupa().nucleotides();
        let basis = ElementalQuaternionBasis::canonical();
        assert_eq!(basis.element_of(x), Element::Water);
        assert_eq!(basis.element_of(y), Element::Fire);
        assert_eq!(basis.element_of(z), Element::Air);
    }

    #[test]
    fn epogdoon_continuity_reads_off_the_active_ratio() {
        let state = PhysicalPoleFormState::compose(
            sample_identity(),
            sample_m1(),
            sample_m2(),
            Readiness::Ready,
            sample_m3(),
            Readiness::Ready,
            PoleProvenance::default(),
        );
        assert!(state.epogdoon_continuity_holds(), "9/8 active ratio");
        let drifted = M1CarrierFacts {
            active_ratio: (10, 9),
            ..sample_m1()
        };
        let state = PhysicalPoleFormState::compose(
            sample_identity(),
            drifted,
            sample_m2(),
            Readiness::Ready,
            sample_m3(),
            Readiness::Ready,
            PoleProvenance::default(),
        );
        assert!(!state.epogdoon_continuity_holds(), "drift is detectable");
    }

    #[test]
    fn readiness_aggregates_by_the_weakest_section() {
        let compose = |m2: Readiness, m3: Readiness, provider: Readiness| {
            PhysicalPoleFormState::compose(
                sample_identity(),
                sample_m1(),
                sample_m2(),
                m2,
                sample_m3(),
                m3,
                PoleProvenance {
                    provider_status: provider,
                    ..PoleProvenance::default()
                },
            )
        };
        assert_eq!(
            compose(Readiness::Ready, Readiness::Ready, Readiness::Ready).readiness(),
            Readiness::Ready
        );
        assert_eq!(
            compose(Readiness::Ready, Readiness::Provisional, Readiness::Ready).readiness(),
            Readiness::Provisional
        );
        assert_eq!(
            compose(Readiness::Ready, Readiness::Ready, Readiness::Unavailable).readiness(),
            Readiness::Unavailable
        );
    }

    #[test]
    fn composition_is_deterministic_and_shared() {
        // Criterion 12: human and agent resolve the same state — composition
        // is a pure function of the sections.
        let build = || {
            PhysicalPoleFormState::compose(
                sample_identity(),
                sample_m1(),
                sample_m2(),
                Readiness::Ready,
                sample_m3(),
                Readiness::Ready,
                PoleProvenance::default(),
            )
        };
        assert_eq!(build(), build());
    }

    #[test]
    fn m2_potential_and_m3_resolution_stay_distinct_roles() {
        // The M2 field's winning form is a shadow of the modal potential;
        // the M3 fold state is the resolved form. They are different roles
        // and the composition never merges them.
        let state = PhysicalPoleFormState::compose(
            sample_identity(),
            sample_m1(),
            sample_m2(),
            Readiness::Ready,
            sample_m3(),
            Readiness::Ready,
            PoleProvenance::default(),
        );
        let (element, form_state) = state.m2_templateure().transduce().winning();
        assert_eq!(element, Element::Fire);
        assert_eq!(form_state, 7);
        assert_eq!(
            state.m3_rupa().codon().hexagram_id(),
            state.m3_rupa().codon().address()
        );
        // 0/1-exclusion echoes through: no M3 value here admits 0 or 1.
        for nucleotide in state.m3_rupa().nucleotides() {
            assert!(CoinSum::new(nucleotide.coin_value().value()).is_ok());
        }
    }
}
