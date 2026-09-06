//! The M3 64: codon address, six-bit fold motif, 384 adjacency, four charges.
//!
//! Three articulated sites × two binary properties (polarity, mobility) = six
//! bits = 4³ = 2⁶ = 64 canonical form motifs. The I-Ching change graph and the
//! fold-change graph are the same state adjacency: one line-change is one
//! coin-flip 2<->3 at one site, so every one-bit fold mutation corresponds to
//! exactly one I-Ching line change and vice versa (64 × 6 = 384).
//!
//! Bit layout of the C reference kernel is preserved: the address is
//! `(X << 4) | (Y << 2) | Z` over nucleotide two-bit values, so site X (outer)
//! owns bits 4-5, the hinge site Y (middle) owns bits 2-3, and site Z (inner)
//! owns bits 0-1; within a site, polarity is the low bit and mobility the high
//! bit. Reading the same six bits as `upper<<3 | lower` gives the I-Ching
//! hexagram id; line ℓ is bit ℓ.
//!
//! The three M3 matrix families act as spatial transforms on the quaternion
//! axes already bound in the kernel:
//!
//! ```text
//! Complementary  -> i (complementation = polarity-conjugating fold)
//! Moving/Resting -> j (mobility half-exchange)
//! Same-Quality   -> k (diagonal resonance action)
//! ```
//!
//! One operation, four registers: symbol / matrix / quaternion-axis / visible
//! fold. The fold-level solver that renders these transforms is the M3
//! unresolved item 11 and stays open; the binding itself is architecture.

use super::coin::{CoinSum, Mobility, Polarity};
use super::nucleotide::Nucleotide;
use crate::QlError;
use core::fmt;

/// A codon: three nucleotides, the 6-bit M3 address (0-63).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Codon64(u8);

impl Codon64 {
    pub const COUNT: usize = 64;
    pub const LINES: usize = 6;

    pub const fn new(address: u8) -> Self {
        Self(address & 0x3F)
    }

    pub fn from_nucleotides(outer: Nucleotide, middle: Nucleotide, inner: Nucleotide) -> Self {
        Self((outer.bits() << 4) | (middle.bits() << 2) | inner.bits())
    }

    pub const fn address(self) -> u8 {
        self.0
    }

    /// The same six bits read as an I-Ching hexagram id.
    pub const fn hexagram_id(self) -> u8 {
        self.0
    }

    pub const fn outer(self) -> Nucleotide {
        const_restore((self.0 >> 4) & 0x03)
    }

    /// The middle nucleotide — the shared hinge of the XY/YZ pair pair.
    pub const fn middle(self) -> Nucleotide {
        const_restore((self.0 >> 2) & 0x03)
    }

    pub const fn inner(self) -> Nucleotide {
        const_restore(self.0 & 0x03)
    }

    /// The sites in body order: [X outer, Y hinge, Z inner].
    pub const fn nucleotides(self) -> [Nucleotide; 3] {
        [self.outer(), self.middle(), self.inner()]
    }

    /// The three site values (X, Y, Z) — the coin sums of outer, hinge, inner.
    pub const fn site_values(self) -> [CoinSum; 3] {
        [
            self.outer().coin_value(),
            self.middle().coin_value(),
            self.inner().coin_value(),
        ]
    }

    /// The codon sum X+Y+Z — the (+,+) positive-resonance integral pp.
    pub const fn codon_sum(self) -> u16 {
        let sites = self.site_values();
        sites[0].value() as u16 + sites[1].value() as u16 + sites[2].value() as u16
    }

    /// The six-bit fold motif: the address read through the fold body.
    pub const fn fold_motif(self) -> FoldMotif {
        FoldMotif(self.0)
    }

    /// One I-Ching line change = one property flip at one site.
    ///
    /// In the coin register the flipped site's value parity flips: a polarity
    /// line (even bit) moves the site value by an odd coin amount (the
    /// canonical resolutions 6→7 and 9→8 are its single-flip realizations),
    /// and a mobility line (odd bit) re-casts the extremity, moving the value
    /// by exactly two coins (6↔8, 9↔7). See the coin module for the
    /// generating cast law.
    pub fn line_change(self, line: u8) -> Result<Self, QlError> {
        if line < Self::LINES as u8 {
            Ok(Self(self.0 ^ (1u8 << line)))
        } else {
            Err(QlError::InvalidPoleValue {
                field: "line",
                value: line as u32,
            })
        }
    }

    /// The six one-bit neighbours — the 384 adjacency as seen from this codon.
    pub fn neighbours(self) -> [Codon64; 6] {
        let mut out = [Self::new(0); 6];
        for (line, neighbour) in out.iter_mut().enumerate() {
            *neighbour = self
                .line_change(line as u8)
                .expect("neighbour lines are 0..6");
        }
        out
    }

    /// The site owning I-Ching line ℓ: lines 0-1 belong to the inner site Z,
    /// lines 2-3 to the hinge Y, lines 4-5 to the outer site X.
    pub const fn site_of_line(line: u8) -> usize {
        2 - (line as usize / 2)
    }

    /// Pair index of the outer dinucleotide (X,Y).
    pub const fn pair_xy(self) -> PairIndex16 {
        PairIndex16::from_nucleotides(self.outer(), self.middle())
    }

    /// Pair index of the inner dinucleotide (Y,Z) — shares the hinge Y.
    pub const fn pair_yz(self) -> PairIndex16 {
        PairIndex16::from_nucleotides(self.middle(), self.inner())
    }

    /// The four-charge evaluation on the value triplet (X, Y, Z).
    pub const fn four_charge(self) -> FourCharge {
        let sites = self.site_values();
        let x = sites[0].value() as i16;
        let y = sites[1].value() as i16;
        let z = sites[2].value() as i16;
        FourCharge {
            pp: x + y + z,
            mm: x - y - z,
            mp: x - y + z,
            pm: x + y - z,
            x_outer: x,
        }
    }

    /// The three-tier codon classification (40 non-dual / 24 dual).
    pub const fn classify(self) -> CodonClass {
        let n1 = self.outer().bits();
        let n2 = self.middle().bits();
        let n3 = self.inner().bits();
        if n1 == n3 {
            if n1 == n2 {
                CodonClass::PerfectPalindromic
            } else {
                CodonClass::ImperfectPalindromic
            }
        } else if n1 == n2 || n2 == n3 {
            CodonClass::NonPalindromicNonDual
        } else {
            CodonClass::Dual
        }
    }

    /// 7 rotational states for non-dual codons, 8 for dual codons.
    pub const fn rotational_state_count(self) -> u8 {
        self.classify().rotational_state_count()
    }

    /// Palindromic anchor: outer == inner (the 16 XyX codons).
    pub const fn is_palindromic(self) -> bool {
        self.outer().bits() == self.inner().bits()
    }
}

impl fmt::Display for Codon64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c] = self.nucleotides();
        write!(f, "{a}{b}{c}")
    }
}

/// Const two-bit restore used by the address accessors.
const fn const_restore(bits: u8) -> Nucleotide {
    match bits {
        1 => Nucleotide::T,
        2 => Nucleotide::C,
        3 => Nucleotide::G,
        _ => Nucleotide::A,
    }
}

/// The six-bit fold motif — the codon address read through the fold body.
///
/// Site order is body order: site 0 = X (outer, bits 4-5), site 1 = Y (the
/// shared hinge, bits 2-3), site 2 = Z (inner, bits 0-1). Within a site the
/// low bit is polarity (1 = yang) and the high bit mobility (1 = resting).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FoldMotif(u8);

impl FoldMotif {
    pub const SITES: usize = 3;

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub fn from_sites(sites: [SiteState; 3]) -> Self {
        let mut bits = 0u8;
        for (index, site) in sites.iter().enumerate() {
            let high = 4 - index as u8 * 2;
            if site.polarity == Polarity::Yang {
                bits |= 1 << high;
            }
            if site.mobility == Mobility::Resting {
                bits |= 1 << (high + 1);
            }
        }
        Self(bits)
    }

    /// The two-bit state of one site in body order (0 = X outer … 2 = Z inner).
    pub const fn site(self, index: usize) -> SiteState {
        let high = 4 - index as u8 * 2;
        let bits = self.0 >> high;
        SiteState {
            polarity: if bits & 1 != 0 {
                Polarity::Yang
            } else {
                Polarity::Yin
            },
            mobility: if bits & 2 != 0 {
                Mobility::Resting
            } else {
                Mobility::Moving
            },
        }
    }

    pub const fn sites(self) -> [SiteState; 3] {
        [self.site(0), self.site(1), self.site(2)]
    }

    /// The codon whose address carries exactly these six bits.
    pub const fn to_codon(self) -> Codon64 {
        Codon64::new(self.0)
    }

    /// The bit index of one property of one site.
    const fn property_bit(index: usize, property: SiteProperty) -> u8 {
        let high = 4 - index as u8 * 2;
        match property {
            SiteProperty::Polarity => high,
            SiteProperty::Mobility => high + 1,
        }
    }

    /// Flip one property of one site — the elementary fold mutation.
    pub fn mutate_site(self, index: usize, property: SiteProperty) -> Self {
        Self(self.0 ^ (1 << Self::property_bit(index, property)))
    }

    /// Every one-property/one-site mutation, in line order: the six
    /// elementary fold mutations are exactly the six I-Ching line changes.
    pub fn elementary_mutations(self) -> [Codon64; 6] {
        let mut out = [Codon64::new(0); 6];
        for line in 0u8..6 {
            let site = Codon64::site_of_line(line);
            let property = if line % 2 == 0 {
                SiteProperty::Polarity
            } else {
                SiteProperty::Mobility
            };
            out[line as usize] = self.mutate_site(site, property).to_codon();
        }
        out
    }
}

/// Which of a site's two binary properties a fold mutation acts on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SiteProperty {
    Polarity,
    Mobility,
}

/// The two-bit state of one articulated fold site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SiteState {
    pub polarity: Polarity,
    pub mobility: Mobility,
}

/// A dinucleotide pair index in the 4×4 = 16 pair field.
///
/// The orientation quantum of pair p is 22.5°·p — the M3 form aperture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PairIndex16(u8);

impl PairIndex16 {
    pub const COUNT: usize = 16;
    /// The pair-field aperture quantum: 360°/16, in tenths of a degree.
    pub const QUANTUM_DEG10: i32 = 225;

    pub const fn from_nucleotides(first: Nucleotide, second: Nucleotide) -> Self {
        Self((first.bits() << 2) | second.bits())
    }

    pub fn from_index(index: u8) -> Result<Self, QlError> {
        if index < Self::COUNT as u8 {
            Ok(Self(index))
        } else {
            Err(QlError::InvalidPoleValue {
                field: "pair16-index",
                value: index as u32,
            })
        }
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    pub const fn first(self) -> Nucleotide {
        const_restore((self.0 >> 2) & 0x03)
    }

    pub const fn second(self) -> Nucleotide {
        const_restore(self.0 & 0x03)
    }

    /// The orientation quantum θ_pair = 22.5°·p, in tenths of a degree.
    pub const fn orientation_quantum_deg10(self) -> i32 {
        Self::QUANTUM_DEG10 * self.0 as i32
    }
}

/// Fixed-point angle in tenths of a degree — the headless angle register of
/// the fold and aperture laws (no floating point in the kernel ground).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AngleDeg10(pub i32);

impl AngleDeg10 {
    pub const FULL_TURN_DEG10: i32 = 3600;
    pub const HALF_TURN_DEG10: i32 = 1800;
    pub const QUARTER_TURN_DEG10: i32 = 900;

    pub const fn reduced(self) -> Self {
        let mut value = self.0 % Self::FULL_TURN_DEG10;
        if value < 0 {
            value += Self::FULL_TURN_DEG10;
        }
        Self(value)
    }

    /// The antipode — a half turn.
    pub const fn antipode(self) -> Self {
        Self(self.0 + Self::HALF_TURN_DEG10).reduced()
    }
}

impl core::ops::Add for AngleDeg10 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

/// The four-charge evaluation of a codon on its value triplet (X, Y, Z).
///
/// ```text
/// pp = X+Y+Z,  mm = X-Y-Z,  mp = X-Y+Z,  pm = X+Y-Z
/// ```
///
/// Invariant: pp + mm + mp + pm = 4X — the 4X invariant of the outer value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FourCharge {
    pub pp: i16,
    pub mm: i16,
    pub mp: i16,
    pub pm: i16,
    /// The outer value X the 4X invariant resolves to.
    pub x_outer: i16,
}

impl FourCharge {
    /// The 4X invariant: the four charges sum to four times the outer value.
    pub const fn four_x_sum(self) -> i16 {
        self.pp + self.mm + self.mp + self.pm
    }

    pub const fn invariant_holds(self) -> bool {
        self.four_x_sum() == 4 * self.x_outer
    }
}

/// The three M3 matrix families and their bound quaternion axes.
///
/// The kernel binds `Complementary → i`, `Moving/Resting → j`,
/// `Same-Quality/RES → k`; one operation, four registers: symbol /
/// matrix / quaternion-axis / visible fold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatrixFamily {
    Complementary,
    MovingResting,
    SameQuality,
}

impl MatrixFamily {
    pub const ALL: [MatrixFamily; 3] = [
        MatrixFamily::Complementary,
        MatrixFamily::MovingResting,
        MatrixFamily::SameQuality,
    ];

    /// The quaternion axis bound to this matrix family.
    pub const fn axis(self) -> MatrixAxis {
        match self {
            MatrixFamily::Complementary => MatrixAxis::I,
            MatrixFamily::MovingResting => MatrixAxis::J,
            MatrixFamily::SameQuality => MatrixAxis::K,
        }
    }
}

/// The quaternion axes i/j/k the M3 matrices act through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatrixAxis {
    I,
    J,
    K,
}

impl MatrixAxis {
    /// The matrix family acting through this axis.
    pub const fn matrix_family(self) -> MatrixFamily {
        match self {
            MatrixAxis::I => MatrixFamily::Complementary,
            MatrixAxis::J => MatrixFamily::MovingResting,
            MatrixAxis::K => MatrixFamily::SameQuality,
        }
    }
}

/// Codon classification: 4 perfect + 12 imperfect palindromic + 24
/// non-palindromic non-dual = 40 non-dual; 24 dual. 40×7 + 24×8 = 472.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodonClass {
    PerfectPalindromic,
    ImperfectPalindromic,
    NonPalindromicNonDual,
    Dual,
}

impl CodonClass {
    pub const fn rotational_state_count(self) -> u8 {
        match self {
            Self::Dual => 8,
            _ => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codon_layout_round_trips_through_nucleotides() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let [n1, n2, n3] = codon.nucleotides();
            assert_eq!(Codon64::from_nucleotides(n1, n2, n3), codon);
        }
    }

    #[test]
    fn fold_motif_is_exact_address() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            assert_eq!(codon.fold_motif().to_codon(), codon);
            let sites = codon.fold_motif().sites();
            assert_eq!(FoldMotif::from_sites(sites).to_codon(), codon);
            // Site order is body order: [X outer, Y hinge, Z inner].
            for (index, nucleotide) in codon.nucleotides().iter().enumerate() {
                let site = codon.fold_motif().site(index);
                assert_eq!(site.polarity, nucleotide.polarity());
                assert_eq!(site.mobility, nucleotide.mobility());
            }
        }
    }

    #[test]
    fn elementary_mutations_are_exactly_the_six_line_changes() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            assert_eq!(
                codon.fold_motif().elementary_mutations(),
                codon.neighbours()
            );
            let mutations = codon.neighbours();
            assert!(mutations.iter().all(|m| *m != codon));
            assert_eq!(
                mutations
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len(),
                6,
                "six distinct neighbours at {address}"
            );
        }
    }

    #[test]
    fn adjacency_is_exactly_384_directed_edges() {
        let mut edges = std::collections::HashSet::new();
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            for neighbour in codon.neighbours() {
                edges.insert((codon.address(), neighbour.address()));
            }
        }
        assert_eq!(edges.len(), 384);
    }

    #[test]
    fn one_line_change_moves_the_owning_site_by_the_coin_law() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let sites = codon.site_values();
            for line in 0u8..6 {
                let changed = codon.line_change(line).expect("line in range");
                let changed_sites = changed.site_values();
                let site_index = Codon64::site_of_line(line);
                let delta =
                    changed_sites[site_index].value() as i16 - sites[site_index].value() as i16;
                if line % 2 == 0 {
                    // A polarity line flips the site value's parity: an odd
                    // number of coins 2<->3 (one flip realises it; the
                    // canonical resolutions 6->7, 9->8 are single flips).
                    assert_eq!(
                        delta.rem_euclid(2),
                        1,
                        "polarity line {line} of {codon} must flip value parity"
                    );
                    assert!(
                        delta.abs() == 1 || delta.abs() == 3,
                        "polarity line {line} of {codon} moves 1 or 3 coins, got {delta}"
                    );
                } else {
                    // A mobility line re-casts the extremity: exactly two
                    // coins, parity preserved (6<->8, 9<->7).
                    assert_eq!(
                        delta.abs(),
                        2,
                        "mobility line {line} of {codon} must move exactly two coins"
                    );
                }
                // The other two sites are untouched.
                for other in 0..3 {
                    if other != site_index {
                        assert_eq!(changed_sites[other], sites[other]);
                    }
                }
            }
        }
    }

    #[test]
    fn four_charge_invariant_holds_over_all_codons() {
        let mut total_pp = 0i64;
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let charge = codon.four_charge();
            assert!(charge.invariant_holds(), "4X invariant at {codon}");
            assert_eq!(charge.pp, codon.codon_sum() as i16);
            total_pp += charge.pp as i64;
        }
        // 1440 = 4 × 360 — the integral invariant total.
        assert_eq!(total_pp, 1440);
    }

    #[test]
    fn classification_counts_40_24_and_472_states() {
        let mut counts = [0usize; 4];
        let mut states = 0usize;
        for address in 0u8..64 {
            let class = Codon64::new(address).classify();
            counts[class as usize] += 1;
            states += class.rotational_state_count() as usize;
        }
        assert_eq!(counts[0], 4, "perfect palindromic");
        assert_eq!(counts[1], 12, "imperfect palindromic");
        assert_eq!(counts[2], 24, "non-palindromic non-dual");
        assert_eq!(counts[3], 24, "dual");
        assert_eq!(counts[0] + counts[1] + counts[2], 40, "non-dual total");
        assert_eq!(states, 472);
    }

    #[test]
    fn pair16_orientation_quantum_is_22_5_degrees() {
        for pair in 0u8..16 {
            let pair_index = PairIndex16::from_index(pair).expect("pair in range");
            assert_eq!(pair_index.orientation_quantum_deg10(), 225 * pair as i32);
        }
        assert!(PairIndex16::from_index(16).is_err());
        assert_eq!(
            PairIndex16::from_nucleotides(Nucleotide::A, Nucleotide::A).index(),
            0
        );
        assert_eq!(
            PairIndex16::from_nucleotides(Nucleotide::G, Nucleotide::G).index(),
            15
        );
    }

    #[test]
    fn matrix_axes_bind_to_the_three_families() {
        assert_eq!(MatrixFamily::Complementary.axis(), MatrixAxis::I);
        assert_eq!(MatrixFamily::MovingResting.axis(), MatrixAxis::J);
        assert_eq!(MatrixFamily::SameQuality.axis(), MatrixAxis::K);
        for family in MatrixFamily::ALL {
            assert_eq!(family.axis().matrix_family(), family);
        }
    }

    #[test]
    fn angle_arithmetic_reduces_and_antipodes() {
        assert_eq!(AngleDeg10(3600).reduced(), AngleDeg10(0));
        assert_eq!(AngleDeg10(225 * 15).reduced().antipode(), AngleDeg10(1575));
        assert_eq!(AngleDeg10(0).antipode(), AngleDeg10(1800));
        assert_eq!(AngleDeg10(-450).reduced(), AngleDeg10(3150));
    }
}
