//! The M3 generation law: how a matrix turns a hexagram into two codons.
//!
//! The map holds one `GenerationEvent` cell for every matrix and hexagram:
//! 3 × 64 = 192 cells, at `M3-3-2-0-*`, `M3-3-2-1-*` and `M3-3-2-2-*`
//! (Matrix 1 Complementary, Matrix 2 Moving/Resting, Matrix 3 Same-Quality).
//! HMS FR 2.Q.8 states the law; the migration
//! `migration/bimba-map/generate_m3_generation_cells.py` proves it against the
//! live map and `fixtures/kernel/m3-generation-field-v1.json` carries the 192
//! cells it found. This module is the same law as kernel code.
//!
//! # The law
//!
//! A matrix sends each of the eight trigrams to a nucleotide pair. The four
//! palindromic trigrams (Qian, Kun, Kan, Li) go to TT, AA, CC, GG in every
//! matrix. The other four (Zhen, Gen, Dui, Xun) go to the two mixed pairs of
//! the matrix's base partition, in both orders:
//!
//! ```text
//! Matrix 1 Complementary  (A↔T, C↔G): Zhen GC  Gen TA  Dui AT  Xun CG
//! Matrix 2 Moving/Resting (A↔G, T↔C): Zhen CT  Dui TC  Gen AG  Xun GA
//! Matrix 3 Same-Quality   (A↔C, T↔G): Zhen AC  Gen CA  Dui GT  Xun TG
//! ```
//!
//! For a hexagram with lower trigram L and upper trigram U, let `ab = π(L)`
//! and `cd = π(U)`. Then
//!
//! ```text
//! positive codon = a·c·d   (the last pair is the upper pair)
//! negative codon = a·b·d   (the first pair is the lower pair)
//! ```
//!
//! The cell is non-dual exactly when `b = c`: the two codons coincide and the
//! cell has one fixed point. Per matrix 16 cells are non-dual (4 perfect
//! palindromes, 4 imperfect palindromes, 8 non-palindromic non-dual) and 48 are
//! dual. The field has 192 cells, 384 polarity slots, 336 distinct yields and
//! 96 non-dual slots. The fixed points of the cells are the 40 non-dual codons;
//! the 24 dual codons (three distinct bases) are never a fixed point.
//!
//! The rotational generation law (`rotational.rs`: four candidates per
//! valence, the middle site swept) is this law at the pair register: the
//! positive valence takes the middle from the upper pair, the negative from the
//! lower.
//!
//! # The matrix actions
//!
//! Each matrix acts on the field in two scales and carries every cell to a
//! cell (equivariance):
//!
//! - codon scale: the matrix's base partner applied to every site
//!   ([`MatrixFamily::apply_to_codon`]); the masks Complementary `⊕0x15`,
//!   Moving/Resting `⊕0x3F`, Same-Quality `⊕0x2A` are derived from the partner
//!   table ([`MatrixFamily::codon_mask`]), not stored;
//! - hexagram scale: each trigram goes to the trigram whose pair is the partner
//!   of its pair ([`MatrixFamily::apply_to_hexagram`]). In lines: Matrix 1 flips
//!   all three lines of every trigram; Matrix 2 flips the outer lines of the
//!   palindromic trigrams and the middle line of the other four; Matrix 3 flips
//!   the middle line of the palindromic trigrams and the outer lines of the
//!   other four.
//!
//! Matrix 1 ∘ Matrix 2 = Matrix 3 at both scales (the Klein group; the
//! quaternion lift supplies the sign of `ij = k`). There are no gaps.
//!
//! The I Ching's own hexagram complement (all six lines,
//! [`super::iching::complement`]) and trigram exchange are separate named I
//! Ching relations. They are not the matrices, although Matrix 1's hexagram
//! action coincides numerically with the complement.
//!
//! # Trigram identity
//!
//! Trigrams are keyed by their three-bit line pattern as the kernel reads a
//! hexagram id: bit 0 is the bottom line, yang = 1, lower trigram in bits 0-2,
//! upper trigram in bits 3-5 (`Trigram::binary`, [`super::iching::lower_trigram_id`]).

use super::codon::{Codon64, CodonClass, MatrixFamily, PairIndex16};
use super::iching::{Trigram, lower_trigram_id, upper_trigram_id};
use super::nucleotide::{Nucleotide, Nucleotide as N};
use super::transcription::matrix_partner;

/// Semantic identity of the generation law.
pub const POLE_GENERATION_LAW_REF: &str = "ql.pole.m3-generation-law/v1";

/// Matrices × hexagrams: the cells of the generation field.
pub const GENERATION_CELL_COUNT: usize = 192;

/// The trigram → pair table of each matrix, indexed by matrix then by trigram
/// line pattern (`Trigram::binary`: 0 Kun, 1 Zhen, 2 Kan, 3 Dui, 4 Gen, 5 Li,
/// 6 Xun, 7 Qian). Map coordinates `M3-3-2-0`, `M3-3-2-1`, `M3-3-2-2`.
pub const TRIGRAM_PAIRS: [[[Nucleotide; 2]; 8]; 3] = [
    // Matrix 1 Complementary
    [
        [N::A, N::A], // 0 Kun
        [N::G, N::C], // 1 Zhen
        [N::C, N::C], // 2 Kan
        [N::A, N::T], // 3 Dui
        [N::T, N::A], // 4 Gen
        [N::G, N::G], // 5 Li
        [N::C, N::G], // 6 Xun
        [N::T, N::T], // 7 Qian
    ],
    // Matrix 2 Moving/Resting
    [
        [N::A, N::A], // 0 Kun
        [N::C, N::T], // 1 Zhen
        [N::C, N::C], // 2 Kan
        [N::T, N::C], // 3 Dui
        [N::A, N::G], // 4 Gen
        [N::G, N::G], // 5 Li
        [N::G, N::A], // 6 Xun
        [N::T, N::T], // 7 Qian
    ],
    // Matrix 3 Same-Quality
    [
        [N::A, N::A], // 0 Kun
        [N::A, N::C], // 1 Zhen
        [N::C, N::C], // 2 Kan
        [N::G, N::T], // 3 Dui
        [N::C, N::A], // 4 Gen
        [N::G, N::G], // 5 Li
        [N::T, N::G], // 6 Xun
        [N::T, N::T], // 7 Qian
    ],
];

const fn family_row(family: MatrixFamily) -> usize {
    match family {
        MatrixFamily::Complementary => 0,
        MatrixFamily::MovingResting => 1,
        MatrixFamily::SameQuality => 2,
    }
}

/// The pair a matrix gives a trigram (by line pattern, 0..8).
pub const fn trigram_pair(family: MatrixFamily, pattern: u8) -> PairIndex16 {
    let pair = TRIGRAM_PAIRS[family_row(family)][(pattern & 0x07) as usize];
    PairIndex16::from_nucleotides(pair[0], pair[1])
}

/// One cell of the generation field: a matrix applied to a hexagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationCell {
    pub family: MatrixFamily,
    /// Hexagram id as the kernel reads it (`upper << 3 | lower`, line patterns).
    pub hexagram: u8,
    /// The pair of the lower trigram: `ab`.
    pub lower_pair: PairIndex16,
    /// The pair of the upper trigram: `cd`.
    pub upper_pair: PairIndex16,
    /// `a·c·d`: the first base of the lower pair, then the upper pair.
    pub positive: Codon64,
    /// `a·b·d`: the lower pair, then the last base of the upper pair.
    pub negative: Codon64,
    /// Perfect / imperfect palindromic / non-palindromic non-dual when the
    /// cell is non-dual (the class of its fixed point); `Dual` otherwise.
    pub class: CodonClass,
}

impl GenerationCell {
    /// Non-dual exactly when `b = c`: the lower pair's second base is the
    /// upper pair's first.
    pub const fn is_non_dual(&self) -> bool {
        self.lower_pair.second().bits() == self.upper_pair.first().bits()
    }

    /// The single codon both slots yield, when the cell is non-dual.
    pub fn fixed_point(&self) -> Option<Codon64> {
        self.is_non_dual().then_some(self.positive)
    }

    /// The two polarity slots, positive first.
    pub const fn yields(&self) -> [Codon64; 2] {
        [self.positive, self.negative]
    }

    /// Lower trigram line pattern.
    pub const fn lower_trigram(&self) -> Trigram {
        Trigram::from_binary(lower_trigram_id(self.hexagram))
    }

    /// Upper trigram line pattern.
    pub const fn upper_trigram(&self) -> Trigram {
        Trigram::from_binary(upper_trigram_id(self.hexagram))
    }
}

/// The generation law: the cell of a matrix at a hexagram (low six bits used).
pub fn generate(family: MatrixFamily, hexagram: u8) -> GenerationCell {
    let hexagram = hexagram & 0x3F;
    let lower_pair = trigram_pair(family, lower_trigram_id(hexagram));
    let upper_pair = trigram_pair(family, upper_trigram_id(hexagram));
    let (a, b) = (lower_pair.first(), lower_pair.second());
    let (c, d) = (upper_pair.first(), upper_pair.second());
    let positive = Codon64::from_nucleotides(a, c, d);
    let negative = Codon64::from_nucleotides(a, b, d);
    let class = if b == c {
        positive.classify()
    } else {
        CodonClass::Dual
    };
    GenerationCell {
        family,
        hexagram,
        lower_pair,
        upper_pair,
        positive,
        negative,
        class,
    }
}

/// The 64 cells of one matrix, in hexagram-id order.
pub fn cells(family: MatrixFamily) -> impl Iterator<Item = GenerationCell> {
    (0u8..64).map(move |hexagram| generate(family, hexagram))
}

/// All 192 cells: Matrix 1, then 2, then 3, each in hexagram-id order.
pub fn all_cells() -> impl Iterator<Item = GenerationCell> {
    MatrixFamily::ALL.into_iter().flat_map(cells)
}

impl MatrixFamily {
    /// The Matrix number as the map names it: 1 Complementary, 2
    /// Moving/Resting, 3 Same-Quality.
    pub const fn matrix_number(self) -> u8 {
        family_row(self) as u8 + 1
    }

    /// The base partner of a nucleotide under this matrix.
    pub const fn apply_to_nucleotide(self, nucleotide: Nucleotide) -> Nucleotide {
        matrix_partner(self, nucleotide)
    }

    /// The per-site XOR mask of the partner action on a codon address,
    /// derived from the partner table: the partner of A, repeated for the
    /// three sites. Complementary `0x15`, Moving/Resting `0x3F`,
    /// Same-Quality `0x2A`. Every matrix partner is `n ⊕ partner(A)`.
    pub const fn codon_mask(self) -> u8 {
        let bits = matrix_partner(self, Nucleotide::A).bits();
        (bits << 4) | (bits << 2) | bits
    }

    /// The codon scale: the matrix partner applied to every site.
    /// An involution that carries non-dual codons to non-dual and dual to dual.
    pub const fn apply_to_codon(self, codon: Codon64) -> Codon64 {
        Codon64::new(codon.address() ^ self.codon_mask())
    }

    /// The trigram this matrix carries a trigram (by line pattern) to: the one
    /// whose pair is the partner of the trigram's pair.
    pub const fn apply_to_trigram(self, pattern: u8) -> u8 {
        let pair = TRIGRAM_PAIRS[family_row(self)][(pattern & 0x07) as usize];
        let want = [
            matrix_partner(self, pair[0]).bits(),
            matrix_partner(self, pair[1]).bits(),
        ];
        let row = &TRIGRAM_PAIRS[family_row(self)];
        let mut target = 0;
        while target < 8 {
            if row[target][0].bits() == want[0] && row[target][1].bits() == want[1] {
                return target as u8;
            }
            target += 1;
        }
        // The partner of a pair of the row is always a pair of the row
        // (pinned by test); the identity is unreachable.
        pattern & 0x07
    }

    /// The lines this matrix flips in a trigram (by line pattern; bit 0 is the
    /// bottom line): `pattern ⊕ apply_to_trigram(pattern)`.
    pub const fn trigram_line_flips(self, pattern: u8) -> u8 {
        (pattern & 0x07) ^ self.apply_to_trigram(pattern)
    }

    /// The hexagram scale: each trigram goes to its matrix image. Carries the
    /// cell at `hexagram` to the cell at the result.
    pub const fn apply_to_hexagram(self, hexagram: u8) -> u8 {
        let lower = self.apply_to_trigram(lower_trigram_id(hexagram));
        let upper = self.apply_to_trigram(upper_trigram_id(hexagram));
        (upper << 3) | lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_field_has_192_cells() {
        assert_eq!(all_cells().count(), GENERATION_CELL_COUNT);
    }

    #[test]
    fn codon_masks_come_from_the_partner_table() {
        assert_eq!(MatrixFamily::Complementary.codon_mask(), 0x15);
        assert_eq!(MatrixFamily::MovingResting.codon_mask(), 0x3F);
        assert_eq!(MatrixFamily::SameQuality.codon_mask(), 0x2A);
        for family in MatrixFamily::ALL {
            for n in Nucleotide::ALL {
                let mask = family.codon_mask() & 0x03;
                assert_eq!(family.apply_to_nucleotide(n).bits(), n.bits() ^ mask);
            }
        }
    }

    #[test]
    fn matrix_one_then_two_is_matrix_three_at_both_scales() {
        for hexagram in 0u8..64 {
            let m1 = MatrixFamily::Complementary;
            let m2 = MatrixFamily::MovingResting;
            let m3 = MatrixFamily::SameQuality;
            assert_eq!(
                m2.apply_to_hexagram(m1.apply_to_hexagram(hexagram)),
                m3.apply_to_hexagram(hexagram)
            );
            let codon = Codon64::new(hexagram);
            assert_eq!(
                m2.apply_to_codon(m1.apply_to_codon(codon)),
                m3.apply_to_codon(codon)
            );
        }
    }
}
