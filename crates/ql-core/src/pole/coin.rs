//! The M3 coin law — the native ground of the physical-pole form-field.
//!
//! Values are generated, never primitive. Each site casts three coins, heads=3
//! and tails=2, and the site value is the coin sum in {6,7,8,9}. The counting
//! starts at 2 and 3 (yin, yang) so that 0/1 remains the non-dual binary — the
//! kernel anchor `# / 0/1 <-> 1/0`. No M3 value type admits 0 or 1.
//!
//! Two bits derive from every coin sum:
//!
//! ```text
//! polarity = parity of the sum      (odd = yang: 7, 9; even = yin: 6, 8)
//! mobility = extremity of the sum   (6 and 9 move — the all-same triples;
//!                                    7 and 8 rest — the mixed triples)
//! ```
//!
//! This is the generating law of `m4_cast_iching` in the C reference kernel
//! (`vendor/epi-kernel/reference/src/m4.c`, lines ~403-419) and of the
//! classical coin arithmetic (old yin 6, young yang 7, young yin 8, old
//! yang 9). One line-change is one coin flip 2<->3: moving lines resolve to
//! their resting partners (6→7, 9→8).

use crate::QlError;
use core::fmt;

/// One coin face: tails carries 2 (yin), heads carries 3 (yang).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoinFace {
    /// tails — 2
    Yin = 2,
    /// heads — 3
    Yang = 3,
}

impl CoinFace {
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// One line-change is one coin flip 2<->3.
    pub const fn flip(self) -> Self {
        match self {
            Self::Yin => Self::Yang,
            Self::Yang => Self::Yin,
        }
    }

    pub fn from_value(value: u8) -> Result<Self, QlError> {
        match value {
            2 => Ok(Self::Yin),
            3 => Ok(Self::Yang),
            other => Err(QlError::InvalidPoleValue {
                field: "coin-face",
                value: other as u32,
            }),
        }
    }
}

/// Three coins cast at one fold site.
///
/// The triple is the generator of the site value: exactly one triple sums to 6
/// ({2,2,2}), three sum to 7, three sum to 8, and exactly one sums to 9
/// ({3,3,3}) — the 1:3:3:1 multiplicity of the coin law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoinTriple([CoinFace; 3]);

impl CoinTriple {
    /// The all-yin cast — the only triple summing to 6.
    pub const OLD_YIN: Self = Self([CoinFace::Yin, CoinFace::Yin, CoinFace::Yin]);
    /// The all-yang cast — the only triple summing to 9.
    pub const OLD_YANG: Self = Self([CoinFace::Yang, CoinFace::Yang, CoinFace::Yang]);

    pub const fn new(coins: [CoinFace; 3]) -> Self {
        Self(coins)
    }

    pub const fn coins(self) -> [CoinFace; 3] {
        self.0
    }

    pub const fn sum(self) -> CoinSum {
        let mut total = 0u8;
        let mut i = 0;
        while i < 3 {
            total += self.0[i].value();
            i += 1;
        }
        CoinSum::const_new(total)
    }

    /// Every one-coin flip of this triple — the elementary change of the site.
    /// Three flips exist (one per coin); the resulting sums are the
    /// one-coin-flip neighbours of the site value.
    pub fn one_coin_flips(self) -> [CoinSum; 3] {
        let mut out = [CoinSum::const_new(6); 3];
        for (slot, out_sum) in out.iter_mut().enumerate() {
            let mut coins = self.0;
            coins[slot] = coins[slot].flip();
            *out_sum = Self::new(coins).sum();
        }
        out
    }
}

/// The generated M3 site value: a coin sum in 6..=9.
///
/// The type is the 0/1-exclusion law: it cannot be constructed from 0, 1 or
/// any value outside the coin range, because M3's value space never uses 0/1 —
/// those remain the non-dual binary beneath the counting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CoinSum(u8);

impl CoinSum {
    pub const MIN: u8 = 6; // 2+2+2 — old yin
    pub const MAX: u8 = 9; // 3+3+3 — old yang

    pub const fn const_new(value: u8) -> Self {
        Self(value)
    }

    /// The exclusion law: only 6..=9 construct.
    pub fn new(value: u8) -> Result<Self, QlError> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(QlError::InvalidPoleValue {
                field: "coin-sum",
                value: value as u32,
            })
        }
    }

    pub const fn value(self) -> u8 {
        self.0
    }

    /// Polarity is the parity of the coin sum: odd = yang, even = yin.
    pub const fn polarity(self) -> Polarity {
        if self.0 % 2 == 1 {
            Polarity::Yang
        } else {
            Polarity::Yin
        }
    }

    /// Mobility is extremity: the all-same triples 6 and 9 move, the mixed
    /// triples 7 and 8 rest.
    pub const fn mobility(self) -> Mobility {
        match self.0 {
            6 | 9 => Mobility::Moving,
            _ => Mobility::Resting,
        }
    }

    /// The resolution law: a moving line resolves to its resting partner
    /// (6→7, 9→8); a resting line is already resolved.
    pub const fn resolve_moving(self) -> Option<CoinSum> {
        match self.0 {
            6 => Some(CoinSum::const_new(7)),
            9 => Some(CoinSum::const_new(8)),
            _ => None,
        }
    }

    /// The site's coin triples — 1 for the extremities, 3 for the mixed sums
    /// (the 1:3:3:1 multiplicity of the coin law).
    pub fn coin_triples(self) -> Vec<CoinTriple> {
        let mut triples = Vec::with_capacity(3);
        for a in [CoinFace::Yin, CoinFace::Yang] {
            for b in [CoinFace::Yin, CoinFace::Yang] {
                for c in [CoinFace::Yin, CoinFace::Yang] {
                    let triple = CoinTriple::new([a, b, c]);
                    if triple.sum() == self {
                        triples.push(triple);
                    }
                }
            }
        }
        triples
    }

    /// The one-coin-flip neighbours of this value: {6}->{7}, {9}->{8},
    /// {7}->{6,8}, {8}->{7,9}. Every neighbour differs by exactly one coin
    /// 2<->3 and flips polarity.
    pub fn one_coin_flip_neighbours(self) -> Vec<CoinSum> {
        let mut sums = Vec::with_capacity(2);
        for triple in self.coin_triples() {
            for neighbour in triple.one_coin_flips() {
                if !sums.contains(&neighbour) {
                    sums.push(neighbour);
                }
            }
        }
        sums.sort_unstable();
        sums
    }
}

impl TryFrom<u8> for CoinSum {
    type Error = QlError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for CoinSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The polarity bit: yin (even coin sum) or yang (odd coin sum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    Yin,
    Yang,
}

impl Polarity {
    /// Sign convention of the fold register: yin folds valley (+), yang folds
    /// mountain (−).
    pub const fn valley(self) -> bool {
        matches!(self, Self::Yin)
    }

    pub const fn flips(self) -> Self {
        match self {
            Self::Yin => Self::Yang,
            Self::Yang => Self::Yin,
        }
    }
}

/// The mobility bit: moving (extreme coin sum 6/9) or resting (mixed 7/8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mobility {
    Moving,
    Resting,
}

impl Mobility {
    pub const fn flips(self) -> Self {
        match self {
            Self::Moving => Self::Resting,
            Self::Resting => Self::Moving,
        }
    }
}

/// The yin-yang monoid identities over the coin base {2,3}.
///
/// These are the arithmetic ground the round-trips stand on:
///
/// ```text
/// 9/8  = 3^2/2^3   (yang^2/yin^3 — the epogdoon)
/// 72   = 2^3 * 3^2 = 8 * 9            (yin^3 * yang^2)
/// 64   = 2^6                          (yin^6)
/// 72 * 8/9 = 64                       (the yang^2 -> yin^3 DET trade)
/// 2+3  = 5                            (the pentadic aperture)
/// 2*3  = 6                            (old yin — the first moving value)
/// ```
pub mod monoid {
    /// 9/8 reduced is already canonical: yang² / yin³.
    pub const EPOGDOON_NUMERATOR: u32 = 9; // 3^2
    pub const EPOGDOON_DENOMINATOR: u32 = 8; // 2^3

    /// 72 = 2^3 · 3^2 = 8 · 9 — yin³ · yang².
    pub const SEVENTY_TWO: u32 = 72;

    /// 64 = 2^6 — yin⁶.
    pub const SIXTY_FOUR: u32 = 64;

    /// 5 = 2 + 3 — the pentadic aperture.
    pub const PENTAD: u32 = 5;

    /// 6 = 2 · 3 — old yin.
    pub const SIX: u32 = 6;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coin_sums_cover_six_through_nine() {
        for value in 6..=9 {
            assert!(CoinSum::new(value).is_ok());
        }
        for value in [0, 1, 2, 3, 4, 5, 10, 255] {
            assert!(
                CoinSum::new(value).is_err(),
                "coin sum must exclude {value}"
            );
        }
    }

    #[test]
    fn parity_law_odd_yang_even_yin() {
        assert_eq!(CoinSum::const_new(6).polarity(), Polarity::Yin);
        assert_eq!(CoinSum::const_new(7).polarity(), Polarity::Yang);
        assert_eq!(CoinSum::const_new(8).polarity(), Polarity::Yin);
        assert_eq!(CoinSum::const_new(9).polarity(), Polarity::Yang);
    }

    #[test]
    fn mobility_law_extremes_move() {
        assert_eq!(CoinSum::const_new(6).mobility(), Mobility::Moving);
        assert_eq!(CoinSum::const_new(7).mobility(), Mobility::Resting);
        assert_eq!(CoinSum::const_new(8).mobility(), Mobility::Resting);
        assert_eq!(CoinSum::const_new(9).mobility(), Mobility::Moving);
    }

    #[test]
    fn moving_lines_resolve_to_resting_partners() {
        assert_eq!(
            CoinSum::const_new(6).resolve_moving(),
            Some(CoinSum::const_new(7))
        );
        assert_eq!(
            CoinSum::const_new(9).resolve_moving(),
            Some(CoinSum::const_new(8))
        );
        for resting in [7, 8] {
            assert_eq!(CoinSum::const_new(resting).resolve_moving(), None);
        }
    }

    #[test]
    fn triple_multiplicity_is_one_three_three_one() {
        assert_eq!(CoinSum::const_new(6).coin_triples().len(), 1);
        assert_eq!(CoinSum::const_new(7).coin_triples().len(), 3);
        assert_eq!(CoinSum::const_new(8).coin_triples().len(), 3);
        assert_eq!(CoinSum::const_new(9).coin_triples().len(), 1);
        assert_eq!(CoinSum::const_new(6).coin_triples()[0], CoinTriple::OLD_YIN);
        assert_eq!(
            CoinSum::const_new(9).coin_triples()[0],
            CoinTriple::OLD_YANG
        );
    }

    #[test]
    fn one_coin_flips_match_resolution_law() {
        assert_eq!(
            CoinSum::const_new(6).one_coin_flip_neighbours(),
            vec![CoinSum::const_new(7)]
        );
        assert_eq!(
            CoinSum::const_new(9).one_coin_flip_neighbours(),
            vec![CoinSum::const_new(8)]
        );
        assert_eq!(
            CoinSum::const_new(7).one_coin_flip_neighbours(),
            vec![CoinSum::const_new(6), CoinSum::const_new(8)]
        );
        assert_eq!(
            CoinSum::const_new(8).one_coin_flip_neighbours(),
            vec![CoinSum::const_new(7), CoinSum::const_new(9)]
        );
    }

    #[test]
    fn monoid_identities_hold() {
        assert_eq!(monoid::EPOGDOON_NUMERATOR, 3 * 3);
        assert_eq!(monoid::EPOGDOON_DENOMINATOR, 2 * 2 * 2);
        assert_eq!(monoid::SEVENTY_TWO, 8 * 9);
        assert_eq!(monoid::SEVENTY_TWO, (1 << 3) * (3 * 3));
        assert_eq!(monoid::SIXTY_FOUR, 1 << 6);
        assert_eq!(monoid::SEVENTY_TWO * 8 / 9, monoid::SIXTY_FOUR);
        assert_eq!(2 + 3, monoid::PENTAD);
        assert_eq!(2 * 3, monoid::SIX);
    }
}
