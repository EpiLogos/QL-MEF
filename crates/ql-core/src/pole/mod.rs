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
pub mod codon;
pub mod coin;
pub mod nucleotide;
pub mod pose;

pub use aperture::{AngularGrid, ApertureClock, ApertureIndex, FibonacciGround};
pub use codon::{
    AngleDeg10, Codon64, CodonClass, FoldMotif, FourCharge, MatrixAxis, MatrixFamily, PairIndex16,
    SiteProperty, SiteState,
};
pub use coin::{CoinFace, CoinSum, CoinTriple, Mobility, Polarity, monoid};
pub use nucleotide::Nucleotide;
pub use pose::{ROTATIONAL_STATE_TOTAL, RotationalPose, all_poses};

/// Version of the physical-pole form contract this ground serves.
pub const PHYSICAL_POLE_FORM_CONTRACT_VERSION: &str = "1.0.0";

/// Semantic identity of the M3 coin-value ground.
pub const POLE_COIN_CONTRACT_REF: &str = "ql.pole.coin-value-ground/v1";
