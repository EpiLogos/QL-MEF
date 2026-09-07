//! The 18-lens clock: sixteen static divisor lenses, one Fibonacci/Pisano
//! ground, one Anuttara void ring.
//!
//! Canon (owner ruling 2026-09-07, unifying the deep CLOCK-AND-NARA specs
//! with the M0 void-ring register):
//!
//! ```text
//! 16  static divisor lenses — the authored reciprocal factor-pair matrix
//!     (deep clock_system.lenses; M3 capability M3-C27): 1°×360, 2°×180,
//!     4°×90, 8°×45, 9°×40, 10°×36, 12°×30, 15°×24, and reciprocals —
//!     arcs 1, 2, 4, 8, 9, 10, 12, 15, 24, 30, 36, 40, 45, 90, 180, 360.
//!     Eight complementary pairs (arc ↔ 360°/arc); reciprocity is divisor
//!     complementation, index law p ↔ 15−p. The four divisor pairs held
//!     by the system's own registers are not lenses: 3/120 (triad),
//!     5/72 (pentad/vortex), 6/60 (the Fibonacci ground's own pair),
//!     18/20 (the elemental fibre grid).
//! +1  Fibonacci/Pisano ground — 60 × 6° = 360° (the Pisano period of the
//!     Fibonacci sequence in base 10; LCM(6, 5, 12) = 60). Level-0,
//!     pre-lensic: never a static lens.
//! +1  Anuttara void ring — 16 arcs × 22.5° = 360°, the M0 register the
//!     sixteen lenses are rendered upon. Its 22.5° quantum is the outlier
//!     division: the only division of 360° that is not an even (integer
//!     degree) division. Its pairing is antipodal, arc p ↔ p+8 — a
//!     different reciprocity from the lens mirror.
//!     = 18 lenses.
//! ```
//!
//! The three physical-pole angular grids synchronize:
//!
//! ```text
//! 6°    Fibonacci/Pisano ground
//! 20°   M2 elemental-vibrational fibre
//! 22.5° Anuttara void ring (the M3 form/fold render register)
//!
//! lcm(6°,20°)    = 60°  -> 6 closures
//! lcm(6°,22.5°)  = 90°  -> 4 closures
//! lcm(20°,22.5°) = 180° -> 2 closures
//! ```
//!
//! The half-turn is the epogdoon seam: 180° = 9 × 20° = 8 × 22.5°.

use super::codon::AngleDeg10;
use crate::QlError;

/// The authored sixteen lens arcs, ascending, in tenths of a degree:
/// 1°, 2°, 4°, 8°, 9°, 10°, 12°, 15°, 24°, 30°, 36°, 40°, 45°, 90°, 180°, 360°
/// — the reciprocal factor-pair matrix of the deep clock
/// (`clock_system.lenses`, M3 capability M3-C27: 1°×360 … 15°×24 and
/// reciprocals). Complement partners sit at `15 − index` exactly: index 0
/// (1°) ↔ index 15 (360°), …, index 7 (15°) ↔ index 8 (24°). The four
/// divisor pairs NOT lenses are the registers the system holds as its own:
/// 3°/120° (triad), 5°/72° (pentad/vortex endpoints), 6°/60° (the
/// Fibonacci/Pisano ground pair — the ground's own complement), 18°/20°
/// (the elemental fibre pair).
const STATIC_DIVISION_DEG10: [i32; 16] = [
    10, 20, 40, 80, 90, 100, 120, 150, 240, 300, 360, 400, 450, 900, 1800, 3600,
];

/// One of the sixteen static divisor lenses, p ∈ 0..16.
///
/// A lens is one of the authored reciprocal factor-pair divisions of 360° —
/// "divide the circle into `360° / division()` parts of `division()` degrees"
/// (e.g. lens 7 = 15°×24, its reciprocal lens 8 = 24°×15). The lens index
/// orders the sixteen arcs ascending, so the reciprocal partner (the
/// complementary divisor) is always at `15 − p`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApertureIndex(u8);

impl ApertureIndex {
    pub const COUNT: usize = 16;
    /// The Anuttara void-ring arc quantum: 360°/16 = 22.5°, in tenths of a
    /// degree. This is the render ring's spacing (see [`AnuttaraVoidRing`]),
    /// not the lens's own division quantum.
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

    /// This lens's own division: the circle divided into parts of this many
    /// degrees (tenths of a degree).
    pub const fn division_deg10(self) -> i32 {
        STATIC_DIVISION_DEG10[self.0 as usize]
    }

    /// The complementary division: parts of `360° / division()` degrees.
    /// Exact for every static lens — complementation is the reciprocity law.
    pub const fn complement_deg10(self) -> i32 {
        AngleDeg10::FULL_TURN_DEG10 * 10 / self.division_deg10()
    }

    /// Render position of lens p on the Anuttara void ring: 22.5°·p.
    pub const fn orientation(self) -> AngleDeg10 {
        AngleDeg10(Self::QUANTUM_DEG10 * self.0 as i32)
    }

    /// The reciprocal partner: p ↔ 15−p, which is the complementary divisor
    /// `360° / division()` by the ascending-order index law.
    ///
    /// On the render ring the partners mirror across the half-quantum axis
    /// between arcs 7 and 8 (paired orientations sum to 15 quanta = 337.5°).
    /// Folding the ring across that diameter enacts lens reciprocity; the
    /// +1 Fibonacci ground remains the shared field beneath both partners.
    pub const fn reciprocal(self) -> Self {
        Self(15 - self.0)
    }

    /// The reciprocity mirror axis, at 7.5 arcs (168.75°).
    pub const RECIPROCITY_AXIS_QUANTA: i32 = 15; // paired orientations sum to this, in quanta

    /// The eight reciprocal (complementary-divisor) pairs (0↔15 … 7↔8).
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

    /// There are exactly eight reciprocal pairs among sixteen lenses.
    pub const RECIPROCAL_PAIR_COUNT: usize = Self::RECIPROCAL_PAIRS.len();
}

/// The Fibonacci/Pisano ground of the clock: 60 divisions × 6° = 360°.
///
/// The 60 is the Pisano period of the Fibonacci sequence in base 10 (its
/// final digit repeats every 60 terms) and the LCM of the system's three
/// fundamental counts: LCM(6, 5, 12) = 60. The ground is Level-0,
/// pre-lensic/base relative to the sixteen static lenses: no operation may
/// promote it to a static lens. It is one of the eighteen lenses.
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

/// The Anuttara void ring: 16 arcs × 22.5° = 360°.
///
/// The M0 register the sixteen static lenses are rendered upon. Its 22.5°
/// quantum is the outlier division of the canon — the only division of the
/// 360° circle that is not an even (integer-degree) division — so it is
/// neither one of the sixteen static lenses nor the Fibonacci ground, but
/// the third member of the 18-lens count. Its own pairing is antipodal:
/// arc p ↔ p+8 across the diameter, a different reciprocity from the lens
/// mirror p ↔ 15−p.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnuttaraVoidRing;

impl AnuttaraVoidRing {
    pub const ARC_COUNT: usize = 16;
    /// 22.5°, in tenths of a degree.
    pub const ARC_QUANTUM_DEG10: i32 = AngleDeg10::FULL_TURN_DEG10 / 16;

    /// The antipodal partner of arc p: the arc one half-turn away.
    pub const fn antipodal(arc: u8) -> u8 {
        (arc + 8) % 16
    }

    /// The eight antipodal arc pairs (0/8, 1/9, …, 7/15).
    pub const ANTIPODAL_PAIRS: [(u8, u8); 8] = [
        (0, 8),
        (1, 9),
        (2, 10),
        (3, 11),
        (4, 12),
        (5, 13),
        (6, 14),
        (7, 15),
    ];
}

/// The 18-lens composition: one shared Fibonacci/Pisano ground, sixteen
/// static reciprocal divisor lenses rendered on the Anuttara void ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApertureClock {
    pub ground: FibonacciGround,
    pub void_ring: AnuttaraVoidRing,
}

impl ApertureClock {
    pub const STATIC_LENSES: usize = ApertureIndex::COUNT;
    pub const GROUNDS: usize = 1;
    pub const VOID_RINGS: usize = 1;
    /// 16 static lenses + 1 Fibonacci ground + 1 Anuttara void ring.
    pub const TOTAL_LENSES: usize = Self::STATIC_LENSES + Self::GROUNDS + Self::VOID_RINGS;

    pub const fn canonical() -> Self {
        Self {
            ground: FibonacciGround,
            void_ring: AnuttaraVoidRing,
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
    /// Anuttara void ring / M3 form-fold render register — 22.5°.
    FormAperture,
}

impl AngularGrid {
    pub const fn quantum_deg10(self) -> i32 {
        match self {
            AngularGrid::FibonacciBase => FibonacciGround::QUANTUM_DEG10,
            AngularGrid::ElementalFibre => 200,
            AngularGrid::FormAperture => AnuttaraVoidRing::ARC_QUANTUM_DEG10,
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
    fn static_lenses_are_the_authored_factor_pair_sixteen() {
        assert_eq!(
            STATIC_DIVISION_DEG10,
            [
                10, 20, 40, 80, 90, 100, 120, 150, 240, 300, 360, 400, 450, 900, 1800, 3600
            ]
        );
        // Every lens arc divides the full turn evenly; the four register
        // pairs (3/120, 5/72, 6/60, 18/20) are deliberately absent.
        for (index, division) in STATIC_DIVISION_DEG10.into_iter().enumerate() {
            assert_eq!(
                AngleDeg10::FULL_TURN_DEG10 * 10 % division,
                0,
                "division {division} must divide the full turn evenly (index {index})"
            );
        }
        for register in [30, 1200, 50, 720, 60, 600, 180, 200] {
            assert!(
                !STATIC_DIVISION_DEG10.contains(&register),
                "register arc {register} must not be a static lens"
            );
        }
    }

    #[test]
    fn reciprocity_is_divisor_complementation() {
        for p in 0_u8..16 {
            let lens = ApertureIndex::new(p).expect("static lens");
            let partner = lens.reciprocal();
            assert_eq!(partner.index(), 15 - p);
            assert_eq!(partner.division_deg10(), lens.complement_deg10());
            // Complement of the complement is the lens itself.
            assert_eq!(partner.complement_deg10(), lens.division_deg10());
        }
        // The authored pairs: 1↔360, 2↔180, 4↔90, 8↔45, 9↔40, 10↔36,
        // 12↔30, 15↔24.
        assert_eq!(ApertureIndex::new(0).unwrap().complement_deg10(), 3600);
        assert_eq!(ApertureIndex::new(3).unwrap().complement_deg10(), 450);
        assert_eq!(ApertureIndex::new(7).unwrap().complement_deg10(), 240);
    }

    #[test]
    fn reciprocal_pairs_mirror_across_the_reciprocity_axis() {
        for (a, b) in ApertureIndex::RECIPROCAL_PAIRS {
            let aperture_a = ApertureIndex::new(a).expect("pair index");
            let aperture_b = ApertureIndex::new(b).expect("pair index");
            assert_eq!(aperture_a.reciprocal(), aperture_b);
            assert_eq!(aperture_b.reciprocal(), aperture_a);
            // Mirror law: paired render orientations sum to 15 quanta
            // (337.5°) — reflection across the axis between arcs 7 and 8.
            assert_eq!(
                aperture_a.orientation().reduced().0 + aperture_b.orientation().reduced().0,
                ApertureIndex::QUANTUM_DEG10 * ApertureIndex::RECIPROCITY_AXIS_QUANTA,
                "reciprocal pair ({a},{b}) must mirror across the 7.5-quantum axis"
            );
        }
    }

    #[test]
    fn anuttara_ring_pairs_are_antipodal() {
        assert_eq!(AnuttaraVoidRing::ARC_COUNT, 16);
        assert_eq!(AnuttaraVoidRing::ARC_QUANTUM_DEG10, 225);
        for (a, b) in AnuttaraVoidRing::ANTIPODAL_PAIRS {
            assert_eq!(AnuttaraVoidRing::antipodal(a), b);
            assert_eq!(AnuttaraVoidRing::antipodal(b), a);
        }
        // Antipodal arcs sit one half-turn apart: orientations differ by 180°.
        let a = AnuttaraVoidRing::ARC_QUANTUM_DEG10 * 3;
        let b = AnuttaraVoidRing::ARC_QUANTUM_DEG10 * i32::from(AnuttaraVoidRing::antipodal(3));
        assert_eq!(b - a, AngleDeg10::HALF_TURN_DEG10);
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
    fn half_turn_is_the_epogdoon_seam() {
        // 180° = 9 × 20° = 8 × 22.5° — the fibre and ring grids close the
        // half-turn in the 9:8 relation.
        assert_eq!(
            9 * AngularGrid::ElementalFibre.quantum_deg10(),
            AngleDeg10::HALF_TURN_DEG10
        );
        assert_eq!(
            8 * AngularGrid::FormAperture.quantum_deg10(),
            AngleDeg10::HALF_TURN_DEG10
        );
    }

    #[test]
    fn clock_is_eighteen_lenses() {
        let clock = ApertureClock::canonical();
        assert_eq!(clock.ground, FibonacciGround);
        assert_eq!(clock.void_ring, AnuttaraVoidRing);
        assert_eq!(ApertureClock::TOTAL_LENSES, 18);
        assert_eq!(
            ApertureClock::STATIC_LENSES + ApertureClock::GROUNDS + ApertureClock::VOID_RINGS,
            18
        );
    }
}
