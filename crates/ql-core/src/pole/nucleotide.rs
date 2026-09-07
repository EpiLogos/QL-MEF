//! The M3 material alphabet: nucleotides as coin casts.
//!
//! One site carries the same two-bit distinction as a nucleotide:
//! polarity × mobility. The canonical coin-value table below is the ONE named
//! mapping for the whole physical pole, shared with the C reference kernel
//! (FR 2.3.12, corrected 2026-09-07 by owner ratification): the conformance
//! test pins the C array byte-for-byte so the two languages cannot drift.

use super::coin::{CoinSum, Mobility, Polarity};
use crate::QlError;
use core::fmt;

/// The M3 material alphabet.
///
/// Two-bit layout of the C reference kernel (FR 2.3.1): bit 0 polarity
/// (0 = yin), bit 1 mobility (0 = moving).
///
/// ```text
/// A = 0b00 yin/moving   — old yin   (6) — Water
/// T = 0b01 yang/moving  — old yang  (9) — Fire
/// C = 0b10 yin/resting  — young yin (8) — Earth
/// G = 0b11 yang/resting — young yang(7) — Air
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Nucleotide {
    A = 0,
    T = 1,
    C = 2,
    G = 3,
}

impl Nucleotide {
    pub const ALL: [Nucleotide; 4] = [Self::A, Self::T, Self::C, Self::G];

    /// THE canonical nucleotide → coin-value table of the physical pole.
    ///
    /// Parity law: odd sums are yang, even sums are yin; the extreme sums
    /// move, the mixed sums rest. Classical grades: old yin 6 (A, dominant
    /// yin), young yang 7 (G, subdominant yang), young yin 8 (C, subdominant
    /// yin), old yang 9 (T, dominant yang). Complementary pairs keep
    /// A+T = C+G = 15 and the total 30. The same table stands in the C
    /// reference kernel (FR 2.3.12) — pinned by test.
    pub const NUCLEOTIDE_COIN_VALUE: [u8; 4] = [6, 9, 8, 7];

    pub fn from_bits(bits: u8) -> Result<Self, QlError> {
        match bits & 0x03 {
            0 => Ok(Self::A),
            1 => Ok(Self::T),
            2 => Ok(Self::C),
            _ => Ok(Self::G),
        }
    }

    /// Raw two-bit index; a masked u8 must use [`Self::from_bits_checked`] to
    /// reject out-of-alphabet input.
    pub const fn bits(self) -> u8 {
        self as u8
    }

    pub fn from_bits_checked(bits: u8) -> Result<Self, QlError> {
        match bits {
            0 => Ok(Self::A),
            1 => Ok(Self::T),
            2 => Ok(Self::C),
            3 => Ok(Self::G),
            other => Err(QlError::InvalidPoleValue {
                field: "nucleotide-bits",
                value: other as u32,
            }),
        }
    }

    pub const fn coin_value(self) -> CoinSum {
        CoinSum::const_new(Self::NUCLEOTIDE_COIN_VALUE[self as usize])
    }

    pub const fn polarity(self) -> Polarity {
        self.coin_value().polarity()
    }

    pub const fn mobility(self) -> Mobility {
        self.coin_value().mobility()
    }

    /// Base pairing = XOR 0x01: flips polarity, preserves mobility.
    pub const fn base_pair(self) -> Self {
        match self {
            Self::A => Self::T,
            Self::T => Self::A,
            Self::C => Self::G,
            Self::G => Self::C,
        }
    }

    pub const fn symbol(self) -> char {
        match self {
            Self::A => 'A',
            Self::T => 'T',
            Self::C => 'C',
            Self::G => 'G',
        }
    }
}

impl TryFrom<u8> for Nucleotide {
    type Error = QlError;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        Self::from_bits_checked(bits)
    }
}

impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_table_satisfies_coin_law() {
        for nucleotide in Nucleotide::ALL {
            let value = nucleotide.coin_value();
            assert!(CoinSum::new(value.value()).is_ok());
            // The table agrees with the generated two bits.
            assert_eq!(
                value.polarity(),
                nucleotide.polarity(),
                "parity must match the two-bit polarity of {nucleotide}"
            );
            assert_eq!(
                value.mobility(),
                nucleotide.mobility(),
                "extremity must match the two-bit mobility of {nucleotide}"
            );
        }
    }

    #[test]
    fn canonical_table_matches_two_bit_semantics() {
        // bit 0 = polarity (0 yin), bit 1 = mobility (0 moving)
        assert_eq!(Nucleotide::A.coin_value().value(), 6);
        assert_eq!(Nucleotide::T.coin_value().value(), 9);
        assert_eq!(Nucleotide::C.coin_value().value(), 8);
        assert_eq!(Nucleotide::G.coin_value().value(), 7);
    }

    #[test]
    fn complementary_pairs_sum_to_fifteen() {
        assert_eq!(
            Nucleotide::A.coin_value().value() + Nucleotide::T.coin_value().value(),
            15
        );
        assert_eq!(
            Nucleotide::C.coin_value().value() + Nucleotide::G.coin_value().value(),
            15
        );
        let total: u8 = Nucleotide::ALL.iter().map(|n| n.coin_value().value()).sum();
        assert_eq!(total, 30);
    }

    #[test]
    fn base_pairing_flips_polarity_keeps_mobility() {
        for nucleotide in Nucleotide::ALL {
            let pair = nucleotide.base_pair();
            assert_eq!(pair.polarity(), nucleotide.polarity().flips());
            assert_eq!(pair.mobility(), nucleotide.mobility());
        }
    }

    #[test]
    fn c_kernel_array_is_pinned_to_the_canonical_table() {
        // The C reference kernel (FR 2.3.12, corrected 2026-09-07 by owner
        // ratification) must carry the exact same table as the Rust contract:
        // one law, two languages, zero drift.
        let header = include_str!("../../../../vendor/epi-kernel/reference/include/m3.h");
        let anchor = "NUCLEOTIDE_ICHING_VALUE[4] = {";
        let start = header.find(anchor).expect("C array anchor") + anchor.len();
        let end = header[start..].find('}').expect("array close");
        let parsed: Vec<u8> = header[start..start + end]
            .split(',')
            .map(|piece| {
                piece
                    .trim()
                    .parse()
                    .expect("C array entries are byte literals")
            })
            .collect();
        assert_eq!(parsed, Nucleotide::NUCLEOTIDE_COIN_VALUE.to_vec());
    }
}
