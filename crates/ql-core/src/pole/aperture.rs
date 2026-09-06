//! The 16+1 form-aperture clock.
//!
//! The M3 clock retains the Fibonacci/Pisano base (60 × 6° = 360°) as the
//! pre-lensic ground and the sixteen static divisor apertures at 22.5°·p. The
//! ground is never a seventeenth static aperture — it is the field the sixteen
//! share (typed here as a distinct marker type). The eight reciprocal aperture
//! pairs occupy opposed orientations (0↔15 … 7↔8) and enact reciprocity as the
//! antipodal fold across the same body.
//!
//! The three physical-pole angular grids synchronize:
//!
//! ```text
//! 6°    Fibonacci/Pisano base
//! 20°   M2 elemental-vibrational fibre
//! 22.5° M3 form/fold aperture
//!
//! lcm(6°,20°)    = 60°  -> 6 closures
//! lcm(6°,22.5°)  = 90°  -> 4 closures
//! lcm(20°,22.5°) = 180° -> 2 closures
//! ```
//!
//! hence the shared object exposes the 6/4/2 synchronization field.

use super::codon::AngleDeg10;
use crate::QlError;

/// One of the sixteen static form apertures, p ∈ 0..16.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApertureIndex(u8);

impl ApertureIndex {
    pub const COUNT: usize = 16;
    /// The aperture quantum: 360°/16 = 22.5°, in tenths of a degree.
    pub const QUANTUM_DEG10: i32 = AngleDeg10::FULL_TURN_DEG10 / 16;

    pub fn new(index: u8) -> Result<Self, QlError> {
        if index < Self::COUNT as u8 {
            Ok(Self(index))
        } else {
            Err(QlError::InvalidPoleValue {
                field: "aperture-index",
                value: index as u32,
            })
        }
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    /// Orientation of aperture p on the form ring: 22.5°·p.
    pub const fn orientation(self) -> AngleDeg10 {
        AngleDeg10(Self::QUANTUM_DEG10 * self.0 as i32)
    }

    /// The reciprocal partner: p ↔ 15−p.
    ///
    /// Reciprocity is the mirror relation across the half-quantum axis
    /// between apertures 7 and 8 (the axis at 7.5 quanta): the paired
    /// orientations sum to 15 quanta (337.5°). Folding the body across that
    /// diameter enacts the reciprocity — the antipodal fold of the grammar —
    /// while the +1 Fibonacci ground remains the shared field beneath both
    /// partners.
    pub const fn reciprocal(self) -> Self {
        Self(15 - self.0)
    }

    /// The reciprocity mirror axis, at 7.5 apertures (168.75°).
    pub const RECIPROCITY_AXIS_QUANTA: i32 = 15; // paired orientations sum to this, in quanta

    /// The eight reciprocal pairs (0↔15 … 7↔8).
    pub const RECIPROCAL_PAIRS: [(u8, u8); 8] = [
        (0, 15),
        (1, 14),
        (2, 13),
        (3, 12),
        (4, 11),
        (5, 10),
        (6, 9),
        (7, 8),
    ];

    /// There are exactly eight reciprocal pairs among sixteen apertures.
    pub const RECIPROCAL_PAIR_COUNT: usize = Self::RECIPROCAL_PAIRS.len();
}

/// The Fibonacci/Pisano ground of the M3 clock: 60 divisions × 6° = 360°.
///
/// The ground is pre-lensic/base relative to the sixteen static apertures. It
/// is deliberately a distinct type: no operation may promote it to a
/// seventeenth static aperture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FibonacciGround;

impl FibonacciGround {
    pub const DIVISIONS: u16 = 60;
    /// The base quantum 6°, in tenths of a degree.
    pub const QUANTUM_DEG10: i32 = 60;
    pub const FULL_TURN_DEG10: i32 = 3600;

    /// Orientation of Fibonacci phase φ ∈ 0..60: 6°·φ.
    pub fn phase_orientation(phase: u16) -> Result<AngleDeg10, QlError> {
        if phase < Self::DIVISIONS {
            Ok(AngleDeg10(Self::QUANTUM_DEG10 * phase as i32))
        } else {
            Err(QlError::InvalidPoleValue {
                field: "fibonacci-phase",
                value: phase as u32,
            })
        }
    }
}

/// The 16+1 composition: one shared Fibonacci/Pisano ground, sixteen static
/// reciprocal apertures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApertureClock {
    pub ground: FibonacciGround,
}

impl ApertureClock {
    pub const STATIC_APERTURES: usize = ApertureIndex::COUNT;
    pub const GROUNDS: usize = 1;

    pub const fn canonical() -> Self {
        Self {
            ground: FibonacciGround,
        }
    }
}

/// One angular grid of the physical pole, at its own quantum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AngularGrid {
    /// Fibonacci/Pisano base — 6°.
    FibonacciBase,
    /// M2 elemental-vibrational fibre — 20°.
    ElementalFibre,
    /// M3 form/fold aperture — 22.5°.
    FormAperture,
}

impl AngularGrid {
    pub const fn quantum_deg10(self) -> i32 {
        match self {
            AngularGrid::FibonacciBase => FibonacciGround::QUANTUM_DEG10,
            AngularGrid::ElementalFibre => 200,
            AngularGrid::FormAperture => ApertureIndex::QUANTUM_DEG10,
        }
    }

    /// How many closures the pair of grids shares in one 360° turn:
    /// 360° / lcm(quantum_a, quantum_b).
    pub const fn closures_with(self, other: AngularGrid) -> i32 {
        let a = self.quantum_deg10();
        let b = other.quantum_deg10();
        let g = gcd(a, b);
        AngleDeg10::FULL_TURN_DEG10 / (a / g * b)
    }
}
const fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apertures_cover_the_full_ring() {
        assert_eq!(ApertureIndex::COUNT, 16);
        assert_eq!(ApertureIndex::QUANTUM_DEG10, 225);
        let total = ApertureIndex::COUNT as i32 * ApertureIndex::QUANTUM_DEG10;
        assert_eq!(total, AngleDeg10::FULL_TURN_DEG10);
    }

    #[test]
    fn reciprocal_pairs_mirror_across_the_reciprocity_axis() {
        for (a, b) in ApertureIndex::RECIPROCAL_PAIRS {
            let aperture_a = ApertureIndex::new(a).expect("pair index");
            let aperture_b = ApertureIndex::new(b).expect("pair index");
            assert_eq!(aperture_a.reciprocal(), aperture_b);
            assert_eq!(aperture_b.reciprocal(), aperture_a);
            // Mirror law: paired orientations sum to 15 quanta (337.5°) —
            // reflection across the axis between apertures 7 and 8.
            assert_eq!(
                aperture_a.orientation().reduced().0 + aperture_b.orientation().reduced().0,
                ApertureIndex::QUANTUM_DEG10 * ApertureIndex::RECIPROCITY_AXIS_QUANTA,
                "reciprocal pair ({a},{b}) must mirror across the 7.5-quantum axis"
            );
        }
    }

    #[test]
    fn aperture_rejects_seventeenth_static_index() {
        assert!(ApertureIndex::new(16).is_err());
    }

    #[test]
    fn fibonacci_ground_is_sixty_by_six_degrees() {
        assert_eq!(
            FibonacciGround::DIVISIONS as i32 * FibonacciGround::QUANTUM_DEG10,
            FibonacciGround::FULL_TURN_DEG10
        );
        assert_eq!(
            FibonacciGround::phase_orientation(59)
                .expect("phase")
                .reduced(),
            AngleDeg10(3540)
        );
        assert!(FibonacciGround::phase_orientation(60).is_err());
    }

    #[test]
    fn synchronization_field_is_six_four_two() {
        let base = AngularGrid::FibonacciBase;
        let fibre = AngularGrid::ElementalFibre;
        let aperture = AngularGrid::FormAperture;
        assert_eq!(base.closures_with(fibre), 6);
        assert_eq!(base.closures_with(aperture), 4);
        assert_eq!(fibre.closures_with(aperture), 2);
    }

    #[test]
    fn clock_is_sixteen_plus_one() {
        let clock = ApertureClock::canonical();
        assert_eq!(clock.ground, FibonacciGround);
        assert_eq!(ApertureClock::STATIC_APERTURES + ApertureClock::GROUNDS, 17);
    }
}
