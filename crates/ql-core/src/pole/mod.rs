//! The M3 coin-value ground of the physical-pole form object.
//!
//! This module is the typed, headless contract of the M3 value space:
//! coin-triple ↔ value {6..9} ↔ (polarity, mobility) ↔ nucleotide ↔
//! address64 ↔ six-bit fold motif, with the 384 one-coin-flip adjacency, the
//! i/j/k matrix-axis bindings, the 472 rotational poses and the 16+1 aperture
//! clock. It is the kernel-side ground of `PhysicalPoleFormState`
//! (integrated-object s8); the composed state itself lives at the profile
//! boundary above it.
//!
//! Authorities: `docs/origami work/INTEGRATED-1-2-3-PHYSICAL-POLE-OBJECT.md`
//! (s5, s6, s8, s13, s14), `docs/geometry/FOLD-AND-RULING-GRAMMAR.md` s12,
//! `docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md` (no second substrate), and
//! the C reference kernel (`vendor/epi-kernel/reference/`) as implementation
//! evidence — notably `m4_cast_iching` for the generating coin law.
//!
//! The 0/1-exclusion law: the counting starts at 2 and 3 (yin, yang) so that
//! 0/1 remains the non-dual binary — the kernel anchor `# / 0/1 <-> 1/0`.
//! M3's value space never uses 0 or 1; [`coin::CoinSum`] cannot be constructed
//! from either.

pub mod aperture;
pub mod basis;
pub mod codon;
pub mod coin;
pub mod fold;
pub mod iching;
pub mod inverse;
pub mod nucleotide;
pub mod pose;
pub mod quaternion;

pub use aperture::{AngularGrid, ApertureClock, ApertureIndex, FibonacciGround};
pub use basis::{
    Element, ElementalQuaternionBasis, QuaternionComponents, Transduction18to16, carrier,
    det_shadow,
};
pub use codon::{
    AngleDeg10, Codon64, CodonClass, FoldMotif, FourCharge, MatrixAxis, MatrixFamily, PairIndex16,
    SiteProperty, SiteState,
};
pub use coin::{CoinFace, CoinSum, CoinTriple, Mobility, Polarity, monoid};
pub use fold::{
    ApplyOutcome, FoldGeometry, FoldState, M3_RES_MATRIX, RES_ADMITTED_COUNT, RES_GAP_ADDRESSES,
    RESONANCE_GAP, SiteReading, is_resonance_gap, resonance_entry,
};
pub use iching::{
    ICHING_GRAMMAR_REF, Trigram, complement, compose_hexagram, flow_clockwise,
    integral_symmetry_field, lower_trigram_id, nuclear_hexagram, nuclear_lower, nuclear_upper,
    palindromic_anchors, polar_opposite_simple, polar_opposite_su2, quadrant, upper_trigram_id,
};
pub use inverse::{
    CanonicalAddress, INVERSE_SEAM_CONTRACT_REF, RetrievalEvidence, SelectionContext, SelectionLaw,
};
pub use nucleotide::Nucleotide;
pub use pose::{ROTATIONAL_STATE_TOTAL, RotationalPose, all_poses};
pub use quaternion::{
    DetOverlay, M2_ELEMENT_RING_POSITIONS, ORIENTATION_CHAIN_REF, Quat, RING_QUATERNION_LUT,
    det_overlay, element_quaternion, matrix_axis_quaternion, quat_active_state, quat_codon_state,
    quat_from_codon, quat_from_ring_pos,
};

/// Version of the physical-pole form contract this ground serves.
pub const PHYSICAL_POLE_FORM_CONTRACT_VERSION: &str = "1.0.0";

/// Semantic identity of the M3 coin-value ground.
pub const POLE_COIN_CONTRACT_REF: &str = "ql.pole.coin-value-ground/v1";

/// Semantic identity of the ratified elemental-carrier contract (T2).
pub const POLE_ELEMENTAL_CARRIER_REF: &str = "ql.pole.elemental-carrier/v1";

/// Semantic identity of the M3 fold/rūpa state projection (T5).
pub const POLE_FOLD_STATE_REF: &str = "ql.pole.fold-state/v1";

/// Semantic identity of the I-Ching grammar port (T-I).
pub const POLE_ICHING_GRAMMAR_CONTRACT_REF: &str = ICHING_GRAMMAR_REF;
