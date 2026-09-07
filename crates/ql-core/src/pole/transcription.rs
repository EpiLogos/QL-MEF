//! The M3 transcription layer (N4 port): the DET projection masks, the
//! wave superposition, the amino record, the RNA phase law and the
//! Parashakti ascent — the M2→M3 bridge by which vibration becomes
//! addressable symbol.
//!
//! Coordinates: FR 2.3.0 (projection masks — M2-registered data; the C
//! reference files the DET table under FR 2.2.7), FR 2.3.6 (epogdoon
//! bridge / Parashakti ascent), FR 2.3.9 (codon → amino record), FR 2.3.20
//! (DNA/RNA superposition); capabilities `M3-C02` (72→64 DET / epogdoon
//! transduction), `M3-C09` (three-matrix / quaternion-axis field),
//! `M3-C17` (DNA→RNA transcription), `M3-C18` (amino/protein/backbone
//! projection); branches M3-0 and M3-3.
//!
//! Zero-transcription contract: every numeric table below is *parsed* from
//! the C reference kernel at compile time (`include_str!` plus const
//! scanning, the `pole_coin_contract.rs` pattern). If the vendor table
//! moves, renames or loses an entry, the build breaks — nothing is
//! hand-copied. The one written table is the amino-acid vocabulary
//! *naming* (24 slot names, vendor m3.c:216-219); the recorded
//! codon→slot data itself is parsed.
//!
//! Two recorded descriptions of the epogdoon tax live here and must not be
//! conflated (both are ported verbatim, neither derived from the other):
//!
//! 1. the flat DET address fold — 72 sources → 64 targets, states 64-71
//!    folding onto bits `(i−64)×8` — whose single typed home is
//!    [`super::basis::det_shadow`]; this module carries only the mask
//!    projection of that fold and cross-checks against it, and
//! 2. the arithmetic FR 2.3.6 laws — the `i×8/9` compression and the
//!    lossy round-trip gap detection — whose survivors on the 72-cycle
//!    are exactly the multiples of 9.
//!
//! Honesty notes carried with the data:
//!
//! - [`M3_CODON_TO_AA`] is the kernel's simplified/non-standard record of
//!   the genetic code (deep matrix `M3-C18`: "mapping rigor needs audit";
//!   e.g. ATA is recorded `Met` with the vendor's own "mapping simplified"
//!   note at m3.c:237). This module ports the recorded data; it does not
//!   endorse the biology. The recorded STOP slot is vocabulary index 10 —
//!   the vendor header's "0xFF = STOP" sentinel (m3.c:207) is not what
//!   the table carries.
//! - The ratified RNA direction (deep matrix §9 / `M3-C17`) is T→U on the
//!   T-containing codons: 37 transformed, 27 shared, 101 DNA+RNA unique
//!   forms. The polarity-wide RNA flip of the older portal implementation
//!   is the recorded CONTRADICTION (deep matrix §9 "Current implementation
//!   contradiction", unresolved item 4) and is deliberately not ported.

use super::codon::{Codon64, MatrixFamily, PairIndex16};
use super::coin::monoid;
use super::nucleotide::Nucleotide;
use crate::QlError;

/// Semantic identity of the transcription layer.
pub const POLE_TRANSCRIPTION_REF: &str = "ql.pole.transcription/v1";

/// The M2 vibration cycle: 72 = 2³·3² = 8·9 (yin³·yang²); 64 = 2⁶ its
/// epogdoon image (`monoid::SEVENTY_TWO`, `monoid::SIXTY_FOUR`).
pub const M2_VIBRATION_CYCLE: usize = 72;

// ===================================================================
// Zero-transcription sources
// ===================================================================

const M2_SOURCE: &str = include_str!("../../../../vendor/epi-kernel/reference/src/m2.c");
const M3_SOURCE: &str = include_str!("../../../../vendor/epi-kernel/reference/src/m3.c");

/// Index of the first occurrence of `anchor` at or after `from`; a missing
/// vendor table is a compile-time failure, not a runtime one.
const fn find_anchor_from(source: &[u8], anchor: &[u8], from: usize) -> usize {
    let mut start = from;
    while start + anchor.len() <= source.len() {
        let mut offset = 0;
        while offset < anchor.len() && source[start + offset] == anchor[offset] {
            offset += 1;
        }
        if offset == anchor.len() {
            return start;
        }
        start += 1;
    }
    panic!("transcription: vendor table anchor not found");
}

/// Position just past the next `0x` hex prefix at or after `pos`.
const fn hex_digits_after_prefix(source: &[u8], mut pos: usize) -> usize {
    while pos + 1 < source.len() {
        if source[pos] == b'0' && (source[pos + 1] == b'x' || source[pos + 1] == b'X') {
            return pos + 2;
        }
        pos += 1;
    }
    panic!("transcription: hex literal not found in vendor source");
}

/// Parse a hex `u64` literal starting at `pos`; returns `(value, next)`.
const fn parse_hex_u64(source: &[u8], mut pos: usize) -> (u64, usize) {
    let mut value: u64 = 0;
    let mut digits = 0;
    while pos < source.len() {
        let digit = match source[pos] {
            b'0'..=b'9' => (source[pos] - b'0') as u64,
            b'a'..=b'f' => (source[pos] - b'a') as u64 + 10,
            b'A'..=b'F' => (source[pos] - b'A') as u64 + 10,
            _ => break,
        };
        value = value * 16 + digit;
        digits += 1;
        pos += 1;
    }
    if digits == 0 || digits > 16 {
        panic!("transcription: malformed hex literal in vendor source");
    }
    (value, pos)
}

/// Skip whitespace, commas and `/* */` comments; returns the position of
/// the next code byte.
const fn skip_table_noise(source: &[u8], mut pos: usize) -> usize {
    loop {
        if pos >= source.len() {
            panic!("transcription: vendor table ended unexpectedly");
        }
        let byte = source[pos];
        if byte == b'/' && pos + 1 < source.len() && source[pos + 1] == b'*' {
            pos += 2;
            while pos + 1 < source.len() && !(source[pos] == b'*' && source[pos + 1] == b'/') {
                pos += 1;
            }
            if pos + 1 >= source.len() {
                panic!("transcription: unterminated comment in vendor source");
            }
            pos += 2;
        } else if byte == b' ' || byte == b'\t' || byte == b'\r' || byte == b'\n' || byte == b',' {
            pos += 1;
        } else {
            return pos;
        }
    }
}

/// Parse a decimal `u8` literal (skipping table noise); returns
/// `(value, next)`. A `}` before the table is filled means the vendor
/// source no longer carries every entry — a compile-time failure.
const fn parse_dec_u8(source: &[u8], mut pos: usize) -> (u8, usize) {
    pos = skip_table_noise(source, pos);
    if source[pos] == b'}' {
        panic!("transcription: vendor table shorter than expected");
    }
    let mut value: u16 = 0;
    let mut digits = 0;
    while pos < source.len() && source[pos].is_ascii_digit() {
        value = value * 10 + (source[pos] - b'0') as u16;
        digits += 1;
        pos += 1;
    }
    if digits == 0 || digits > 3 || value > 255 {
        panic!("transcription: malformed decimal literal in vendor source");
    }
    (value as u8, pos)
}

/// Parse the nucleotide letter following the next `M3_NUC_` marker;
/// returns `(letter, next)`.
const fn next_m3_nuc_letter(source: &[u8], pos: usize) -> (u8, usize) {
    let marker = find_anchor_from(source, b"M3_NUC_", pos);
    let letter_pos = marker + b"M3_NUC_".len();
    if letter_pos >= source.len() {
        panic!("transcription: truncated M3_NUC_ marker in vendor source");
    }
    let letter = source[letter_pos];
    if letter != b'A' && letter != b'T' && letter != b'C' && letter != b'G' {
        panic!("transcription: unexpected M3_NUC_ nucleotide letter");
    }
    (letter, letter_pos + 1)
}

const fn nucleotide_from_letter(letter: u8) -> Nucleotide {
    match letter {
        b'A' => Nucleotide::A,
        b'T' => Nucleotide::T,
        b'C' => Nucleotide::C,
        _ => Nucleotide::G,
    }
}

// ===================================================================
// FR 2.3.0 / M3-C02 — DET projection masks (M2-registered data)
// vendor m2.c:821-895 (`FR 2.2.7` in the C reference)
// ===================================================================

const fn parse_projection_table(source: &[u8], start: usize) -> [u64; 72] {
    let mut masks = [0u64; 72];
    let mut pos = start;
    let mut index = 0;
    while index < 72 {
        pos = hex_digits_after_prefix(source, pos);
        let (value, next) = parse_hex_u64(source, pos);
        masks[index] = value;
        pos = next;
        index += 1;
    }
    masks
}

/// The Discrete Epistemic Transform: 72 one-hot uint64 masks, one per M2
/// vibration state (vendor m2.c:821-895).
///
/// States 0-63 project onto their own bit; the 8 tax states 64-71 fold
/// back onto bits `(i−64)×8` — bits 0, 8, …, 56 — the epogdoon tax
/// 72 × 8/9 = 64. The union of all 72 masks covers the 64-address board
/// exactly. The flat address fold itself lives in
/// [`super::basis::det_shadow`]; these masks are its wave projection.
pub const M2_TO_M3_CYMATIC_PROJECTION: [u64; 72] = parse_projection_table(
    M2_SOURCE.as_bytes(),
    find_anchor(M2_SOURCE.as_bytes(), b"M2_TO_M3_CYMATIC_PROJECTION[72] = {"),
);

const fn find_anchor(source: &[u8], anchor: &[u8]) -> usize {
    find_anchor_from(source, anchor, 0)
}

/// Wave superposition (vendor m2.h:533 `transduce_vibration_to_symbol`):
/// the OR of the DET projection masks of the active M2 state indices —
/// many vibrations collapse into one symbolic bitboard. Indices outside
/// the 72-cycle are rejected.
///
/// Empty input is the silent board: 0.
pub fn transduce_vibration_to_symbol(m2_active_indices: &[u8]) -> Result<u64, QlError> {
    let mut bitboard = 0u64;
    for index in m2_active_indices {
        if usize::from(*index) >= M2_VIBRATION_CYCLE {
            return Err(QlError::InvalidPoleValue {
                field: "m2-vibration-index",
                value: u32::from(*index),
            });
        }
        bitboard |= M2_TO_M3_CYMATIC_PROJECTION[*index as usize];
    }
    Ok(bitboard)
}

// ===================================================================
// FR 2.3.6 — epogdoon arithmetic + Parashakti ascent
// vendor m3.h:344-368 (`get_parashakti_frequency`:352,
// `apply_epogdoon_compression`:359, `is_evolutionary_gap`:364)
// ===================================================================

/// The recorded pair-difference sign provenance (M3 unresolved item 2).
///
/// Restates `recorded_signs` in `tests/pole_coin_contract.rs` and is
/// pinned byte-for-byte against the regenerated `M3_PAIR_MATRIX`
/// difference column (vendor m3.c:29-56) by test. Indexing:
/// `(first.bits() << 2) | second.bits()`. The class-stable semantics of
/// `differenceValue` stays open; these signs are provenance, not law.
pub const M3_PAIR_DIFFERENCE_SIGN: [i8; 16] = [
    0,  // AA
    -1, // AT
    -1, // AC
    1,  // AG
    1,  // TA
    0,  // TT
    -1, // TC
    1,  // TG
    1,  // CA
    -1, // CT
    0,  // CC
    1,  // GC
    1,  // GA
    -1, // GT
    -1, // CG
    0,  // GG
];

/// The `M3_PAIR_MATRIX` sum law (regenerated, owner ratification
/// 2026-09-07): the pair sum is the plain coin-value sum `v1 + v2`
/// (vendor m3.c:29-56, magnitude law pinned by test in
/// `tests/pole_coin_contract.rs`).
pub const fn pair_sum(first: Nucleotide, second: Nucleotide) -> i16 {
    first.coin_value().value() as i16 + second.coin_value().value() as i16
}

/// The recorded pair difference: `sign × |v1 − v2|` with the sign taken
/// from [`M3_PAIR_DIFFERENCE_SIGN`] (dataset provenance, M3 unresolved
/// item 2 — preserved verbatim, not derived).
pub const fn pair_difference(first: Nucleotide, second: Nucleotide) -> i16 {
    let first_value = first.coin_value().value() as i16;
    let second_value = second.coin_value().value() as i16;
    let magnitude = if first_value >= second_value {
        first_value - second_value
    } else {
        second_value - first_value
    };
    M3_PAIR_DIFFERENCE_SIGN[((first.bits() << 2) | second.bits()) as usize] as i16 * magnitude
}

/// The rotational `total_sum_value` of `compute_rotational_state`
/// (vendor m3.h:198-202): the sum of the two pair sums. For a codon these
/// are its two hinge pairs, `pair_xy` and `pair_yz` — the same anchors
/// (`first_pair`, `last_pair`) the kernel's rotational generation fixes
/// per codon (vendor m3.c:520, 544).
pub const fn rotational_total_sum_value(pair_xy: PairIndex16, pair_yz: PairIndex16) -> i16 {
    pair_sum(pair_xy.first(), pair_xy.second()) + pair_sum(pair_yz.first(), pair_yz.second())
}

/// The shadow-phase ascent offset: +36, half the 72-cycle
/// (vendor m3.h:354).
pub const PARASHAKTI_SHADOW_OFFSET: i16 = 36;

/// Parashakti ascent (FR 2.3.6, vendor m3.h:352 `get_parashakti_frequency`):
/// the M3→M2 harmonic translation. The base frequency is the rotational
/// `total_sum_value`; the shadow phase adds [`PARASHAKTI_SHADOW_OFFSET`].
///
/// Ranges: base 24-36, shadow 60-72 — the shadow summit touches the top
/// of the M2 vibration cycle.
pub const fn parashakti_frequency(
    pair_xy: PairIndex16,
    pair_yz: PairIndex16,
    is_shadow_phase: bool,
) -> u8 {
    let base = rotational_total_sum_value(pair_xy, pair_yz);
    let frequency = if is_shadow_phase {
        base + PARASHAKTI_SHADOW_OFFSET
    } else {
        base
    };
    frequency as u8
}

/// Codon-level Parashakti ascent over the codon's own two hinge pairs
/// ([`Codon64::pair_xy`], [`Codon64::pair_yz`]).
pub const fn codon_parashakti_frequency(codon: Codon64, is_shadow_phase: bool) -> u8 {
    parashakti_frequency(codon.pair_xy(), codon.pair_yz(), is_shadow_phase)
}

/// The raw epogdoon compression of an in-cycle index: `i × 8 / 9`
/// (vendor m3.h:359) — the single home of the arithmetic law.
const fn epogdoon_compress_raw(m2_vibration_index: u8) -> u8 {
    ((m2_vibration_index as u32) * monoid::EPOGDOON_DENOMINATOR / monoid::EPOGDOON_NUMERATOR) as u8
}

/// DESCENDING: M2 → M3 epogdoon compression `i × 8 / 9` on the 72-cycle
/// (vendor m3.h:359). Indices outside the cycle are rejected.
pub const fn apply_epogdoon_compression(m2_vibration_index: u8) -> Result<u8, QlError> {
    if (m2_vibration_index as usize) < M2_VIBRATION_CYCLE {
        Ok(epogdoon_compress_raw(m2_vibration_index))
    } else {
        Err(QlError::InvalidPoleValue {
            field: "m2-vibration-index",
            value: m2_vibration_index as u32,
        })
    }
}

/// Evolutionary gap detection (vendor m3.h:364): the 9:8 round trip is
/// lossy, and an index that does not survive it marks the gap — the
/// "missing states drive evolutionary spiral" of FR 2.3.6. On the
/// 72-cycle the survivors are exactly the multiples of 9.
pub const fn is_evolutionary_gap(m2_vibration_index: u8) -> Result<bool, QlError> {
    if (m2_vibration_index as usize) < M2_VIBRATION_CYCLE {
        let compressed = epogdoon_compress_raw(m2_vibration_index);
        let expanded =
            ((compressed as u32) * monoid::EPOGDOON_NUMERATOR / monoid::EPOGDOON_DENOMINATOR) as u8;
        Ok(expanded != m2_vibration_index)
    } else {
        Err(QlError::InvalidPoleValue {
            field: "m2-vibration-index",
            value: m2_vibration_index as u32,
        })
    }
}

// ===================================================================
// FR 2.3.9 / M3-C18 — the amino record
// vendor m3.c:204-253 (`M3_CODON_TO_AA[64]`:228, vocabulary:216-219)
// ===================================================================

const fn parse_amino_table(source: &[u8], start: usize) -> [u8; 64] {
    let mut table = [0u8; 64];
    let mut pos = start;
    let mut index = 0;
    while index < 64 {
        let (value, next) = parse_dec_u8(source, pos);
        table[index] = value;
        pos = next;
        index += 1;
    }
    table
}

const AA_ANCHOR: &[u8] = b"M3_CODON_TO_AA[64] = {";

/// The recorded codon → amino-acid-slot table (vendor m3.c:228-253,
/// FR 2.3.9): 64 entries in the kernel's re-indexed scheme
/// `A=0, T=1, C=2, G=3` with codon = `(outer<<4) | (middle<<2) | inner` —
/// exactly [`Codon64`]'s address layout.
///
/// This is the kernel's simplified/non-standard record (deep matrix
/// `M3-C18`: the mapping rigor audit stays open): ported data, not
/// endorsed biology. The STOP slot is [`AA_STOP_INDEX`] (vocabulary
/// index 10), not the 0xFF sentinel of the vendor header comment.
pub const M3_CODON_TO_AA: [u8; 64] = parse_amino_table(
    M3_SOURCE.as_bytes(),
    find_anchor(M3_SOURCE.as_bytes(), AA_ANCHOR) + AA_ANCHOR.len(),
);

/// The 24-slot amino/backbone vocabulary, in slot order
/// (vendor m3.c:216-219; the vendor spells slot 3 "Met(START)" and
/// slot 10 "STOP").
pub const AMINO_ACID_VOCABULARY: [&str; 24] = [
    "Phe", "Leu", "Ile", "Met", "Val", "Ser", "Pro", "Thr", "Ala", "Tyr", "Stop", "His", "Gln",
    "Asn", "Lys", "Asp", "Glu", "Cys", "Trp", "Arg", "Gly", "Ser2", "Arg2", "Thr2",
];

/// The recorded STOP slot of [`AMINO_ACID_VOCABULARY`]
/// (vendor m3.c:217).
pub const AA_STOP_INDEX: u8 = 10;

/// The recorded amino slot of a codon (parsed [`M3_CODON_TO_AA`]).
pub const fn m3_codon_amino_index(codon: Codon64) -> u8 {
    M3_CODON_TO_AA[codon.address() as usize]
}

/// True when the codon is recorded as a STOP (slot [`AA_STOP_INDEX`]):
/// TAA, TAG, TGA.
pub const fn is_stop_codon(codon: Codon64) -> bool {
    m3_codon_amino_index(codon) == AA_STOP_INDEX
}

/// The vocabulary name of a recorded amino slot, or `None` outside the
/// 24-slot backbone.
pub const fn amino_acid_name(index: u8) -> Option<&'static str> {
    if (index as usize) < AMINO_ACID_VOCABULARY.len() {
        Some(AMINO_ACID_VOCABULARY[index as usize])
    } else {
        None
    }
}

// ===================================================================
// FR 2.3.20 / M3-C17 — the RNA phase law
// vendor m3.c:133-134 (masks), m3.h:793-800 (`m3_codon_is_rna_capable`)
// ===================================================================

const fn parse_rna_mask(source: &[u8], anchor: &[u8]) -> u64 {
    let start = find_anchor(source, anchor);
    let digits = hex_digits_after_prefix(source, start);
    let (value, _) = parse_hex_u64(source, digits);
    value
}

/// The RNA-functional codon bitboard (vendor m3.c:133, FR 2.3.20): bit
/// `i` addresses codon `i`. Ratified reading (deep matrix §9 /
/// `M3-C17`): exactly the 37 T-containing codons — the T→U expressive
/// descendants. Pinned to the T-set by test.
pub const M3_RNA_FUNCTIONAL_MASK: u64 =
    parse_rna_mask(M3_SOURCE.as_bytes(), b"M3_RNA_FUNCTIONAL_MASK = ");

/// The RNA-dark codon bitboard (vendor m3.c:134): the complement — the
/// 27 T-free codons, which no transcription can reach. Pinned to the
/// T-free set by test.
pub const M3_RNA_DARK_MASK: u64 = parse_rna_mask(M3_SOURCE.as_bytes(), b"M3_RNA_DARK_MASK");

/// The ratified transcription model (deep matrix §9 / `M3-C17`):
/// 64 − 3³ = 37 codons carry a T and transcribe to a U-form.
pub const RNA_T_CONTAINING_CODONS: usize = 37;

/// The ratified transcription model (deep matrix §9 / `M3-C17`):
/// 3³ = 27 codons are T-free — the shared RNA/DNA forms.
pub const RNA_T_FREE_CODONS: usize = 27;

/// The ratified transcription model (deep matrix §9 / `M3-C17`):
/// 64 + 37 = 101 unique DNA+RNA forms.
pub const DNA_RNA_UNIQUE_FORMS: usize = 101;

/// RNA capability (vendor m3.h:797 `m3_codon_is_rna_capable`): the codon
/// contains at least one T — the T→U substitution site. The ratified
/// direction is T→U on these codons only; the polarity-wide RNA flip of
/// the older portal implementation (deep matrix §9 "Current
/// implementation contradiction", unresolved item 4) is not ported.
pub const fn m3_codon_is_rna_capable(codon: Codon64) -> bool {
    matches!(codon.outer(), Nucleotide::T)
        || matches!(codon.middle(), Nucleotide::T)
        || matches!(codon.inner(), Nucleotide::T)
}

// ===================================================================
// M3-C09 — matrix family → nucleotide binding
// vendor m3.c:58-62 (`M3_MATRIX_PAIR[3][4]`)
// ===================================================================

const fn parse_matrix_pair_table(source: &[u8], start: usize) -> [[Nucleotide; 4]; 3] {
    let mut table = [[Nucleotide::A; 4]; 3];
    let mut pos = start;
    let mut row = 0;
    while row < 3 {
        let mut column = 0;
        while column < 4 {
            let (letter, next) = next_m3_nuc_letter(source, pos);
            table[row][column] = nucleotide_from_letter(letter);
            pos = next;
            column += 1;
        }
        row += 1;
    }
    table
}

/// The matrix-family pairing table (vendor m3.c:58-62), typed over
/// [`MatrixFamily`].
///
/// Row order is the kernel's family order (Complementary, MovingResting,
/// SameQuality); within a row, index = nucleotide bits (A, T, C, G).
/// Each row is the family's *partner map*: `row[n]` is the nucleotide
/// that nucleotide `n` pairs with under that family — an involution
/// without fixed points whose pairs, over the shared homogeneous ground
/// (AA, TT, CC, GG), generate the family's four unique pairs (vendor
/// m3.h:143-156): Complementary AT/CG, MovingResting AG/CT, SameQuality
/// AC/TG. The kernel declares the table but never dereferences it; the
/// partner-map reading is the one consistent with that comment and is
/// pinned by the involution and unique-pair tests.
pub const M3_MATRIX_PAIR: [[Nucleotide; 4]; 3] = parse_matrix_pair_table(
    M3_SOURCE.as_bytes(),
    find_anchor(
        M3_SOURCE.as_bytes(),
        b"M3_MATRIX_PAIR[M3_MATRIX_COUNT][4] = {",
    ),
);

/// The family's partner map, indexed by nucleotide bits.
pub const fn matrix_pair_nucleotides(family: MatrixFamily) -> [Nucleotide; 4] {
    match family {
        MatrixFamily::Complementary => M3_MATRIX_PAIR[0],
        MatrixFamily::MovingResting => M3_MATRIX_PAIR[1],
        MatrixFamily::SameQuality => M3_MATRIX_PAIR[2],
    }
}

/// The nucleotide's partner under the family's pairing.
pub const fn matrix_partner(family: MatrixFamily, nucleotide: Nucleotide) -> Nucleotide {
    matrix_pair_nucleotides(family)[nucleotide.bits() as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pole::basis::det_shadow;

    #[test]
    fn projection_masks_are_one_hot_and_cover_the_address_space() {
        let mut union = 0u64;
        for (state, mask) in M2_TO_M3_CYMATIC_PROJECTION.iter().enumerate() {
            assert_eq!(mask.count_ones(), 1, "state {state} must project one-hot");
            union |= mask;
        }
        assert_eq!(
            union.count_ones(),
            64,
            "72 states × 8/9 = 64: the union covers the address space exactly"
        );
    }

    #[test]
    fn fold_back_states_land_on_the_det_collision_addresses() {
        // The mask projection of the flat DET fold (pole::basis::det_shadow,
        // the law's single home): states 0-63 identity, states 64-71 onto
        // (i−64)×8 — bits 0, 8, …, 56.
        for state in 0u8..72 {
            let target = det_shadow(state).unwrap();
            assert_eq!(
                M2_TO_M3_CYMATIC_PROJECTION[state as usize],
                1u64 << target,
                "state {state} must project onto its DET address"
            );
        }
        for fold in 0u8..8 {
            assert_eq!(det_shadow(64 + fold).unwrap(), fold * 8);
        }
    }

    #[test]
    fn wave_superposition_ors_the_active_masks() {
        assert_eq!(transduce_vibration_to_symbol(&[]).unwrap(), 0);
        assert_eq!(transduce_vibration_to_symbol(&[0]).unwrap(), 0b1);
        assert_eq!(transduce_vibration_to_symbol(&[0, 1]).unwrap(), 0b11);
        assert_eq!(transduce_vibration_to_symbol(&[63]).unwrap(), 1 << 63);
        // Superposition collapses: folded state 64 lands on bit 0.
        assert_eq!(transduce_vibration_to_symbol(&[64]).unwrap(), 1);
        assert_eq!(transduce_vibration_to_symbol(&[0, 64]).unwrap(), 1);
        assert_eq!(transduce_vibration_to_symbol(&[64, 65]).unwrap(), 0x101);
        // Two active states is the kernel's own DET call shape (m3.c:503).
        assert_eq!(
            transduce_vibration_to_symbol(&[31, 32]).unwrap(),
            (1 << 31) | (1 << 32)
        );
    }

    #[test]
    fn out_of_cycle_vibration_indices_are_rejected() {
        assert!(transduce_vibration_to_symbol(&[72]).is_err());
        assert!(transduce_vibration_to_symbol(&[0, 255]).is_err());
        assert!(apply_epogdoon_compression(72).is_err());
        assert!(apply_epogdoon_compression(255).is_err());
        assert!(is_evolutionary_gap(72).is_err());
    }

    #[test]
    fn epogdoon_arithmetic_matches_the_vendor_laws() {
        for index in 0u8..72 {
            let expected = (u32::from(index) * 8) / 9;
            assert_eq!(
                apply_epogdoon_compression(index).unwrap(),
                expected as u8,
                "compression of {index}"
            );
        }
        // The lossy round trip keeps exactly the multiples of 9 — eight
        // surviving states on the 72-cycle (vendor m3.h:364).
        let survivors: Vec<u8> = (0u8..72)
            .filter(|index| !is_evolutionary_gap(*index).unwrap())
            .collect();
        assert_eq!(survivors, vec![0, 9, 18, 27, 36, 45, 54, 63]);
    }

    #[test]
    fn pair_sums_and_differences_follow_the_regenerated_law() {
        for first in Nucleotide::ALL {
            for second in Nucleotide::ALL {
                let sum = first.coin_value().value() as i16 + second.coin_value().value() as i16;
                assert_eq!(pair_sum(first, second), sum, "{first}{second}");
                let magnitude =
                    (first.coin_value().value() as i16 - second.coin_value().value() as i16).abs();
                let sign = M3_PAIR_DIFFERENCE_SIGN[((first.bits() << 2) | second.bits()) as usize];
                assert_eq!(pair_difference(first, second), i16::from(sign) * magnitude);
            }
        }
        // Complementary ground: A+T = C+G = 15.
        assert_eq!(pair_sum(Nucleotide::A, Nucleotide::T), 15);
        assert_eq!(pair_sum(Nucleotide::C, Nucleotide::G), 15);
    }

    #[test]
    fn parashakti_ascent_is_base_plus_shadow_thirtysix() {
        for first in 0u8..16 {
            for second in 0u8..16 {
                let pair_xy = PairIndex16::from_index(first).unwrap();
                let pair_yz = PairIndex16::from_index(second).unwrap();
                let base = parashakti_frequency(pair_xy, pair_yz, false);
                let shadow = parashakti_frequency(pair_xy, pair_yz, true);
                assert_eq!(shadow, base + 36, "shadow phase adds +36");
                assert!((24..=36).contains(&base), "base range at {first},{second}");
                assert!(
                    (60..=72).contains(&shadow),
                    "shadow range at {first},{second}"
                );
            }
        }
        // The shadow summit touches the top of the M2 cycle.
        assert_eq!(monoid::SEVENTY_TWO, 72);
        assert_eq!(monoid::SIXTY_FOUR, 64);
    }

    #[test]
    fn amino_record_stays_in_the_24_slot_vocabulary() {
        assert_eq!(AMINO_ACID_VOCABULARY.len(), 24);
        assert_eq!(amino_acid_name(AA_STOP_INDEX), Some("Stop"));
        assert_eq!(amino_acid_name(24), None);
        let mut stops = Vec::new();
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let index = m3_codon_amino_index(codon);
            assert!(u32::from(index) < 24, "codon {address} outside vocabulary");
            if is_stop_codon(codon) {
                stops.push(codon);
            }
        }
        let symbols: Vec<String> = stops
            .iter()
            .map(|codon| codon.nucleotides().iter().map(|n| n.symbol()).collect())
            .collect();
        assert_eq!(symbols, vec!["TAA", "TAG", "TGA"], "the recorded stops");
    }

    #[test]
    fn rna_masks_are_disjoint_t_addressed_codon_bitboards() {
        assert_eq!(M3_RNA_FUNCTIONAL_MASK & M3_RNA_DARK_MASK, 0);
        assert_eq!(M3_RNA_FUNCTIONAL_MASK | M3_RNA_DARK_MASK, u64::MAX);
        let mut functional = 0u64;
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let capable = m3_codon_is_rna_capable(codon);
            assert_eq!(
                capable,
                (M3_RNA_FUNCTIONAL_MASK >> address) & 1 == 1,
                "functional bit must address the T-containing codon {address}"
            );
            if capable {
                functional |= 1u64 << address;
            }
        }
        assert_eq!(functional, M3_RNA_FUNCTIONAL_MASK);
        assert_eq!(functional.count_ones(), RNA_T_CONTAINING_CODONS as u32);
        assert_eq!(
            (u64::MAX ^ functional).count_ones(),
            RNA_T_FREE_CODONS as u32
        );
    }

    #[test]
    fn ratified_transcription_counts_hold() {
        let mut t_free = 0;
        let mut t_containing = 0;
        for address in 0u8..64 {
            if m3_codon_is_rna_capable(Codon64::new(address)) {
                t_containing += 1;
            } else {
                t_free += 1;
            }
        }
        // 3³ = 27 T-free; 64 − 27 = 37 transformed; 64 + 37 = 101 forms.
        assert_eq!(t_free, 27);
        assert_eq!(t_free, RNA_T_FREE_CODONS);
        assert_eq!(t_containing, 37);
        assert_eq!(t_containing, RNA_T_CONTAINING_CODONS);
        assert_eq!(DNA_RNA_UNIQUE_FORMS, 101);
        assert_eq!(RNA_T_FREE_CODONS + RNA_T_CONTAINING_CODONS, 64);
        assert_eq!(64 + RNA_T_CONTAINING_CODONS, DNA_RNA_UNIQUE_FORMS);
    }

    #[test]
    fn matrix_pair_is_a_partner_map_per_family() {
        for family in MatrixFamily::ALL {
            let partners = matrix_pair_nucleotides(family);
            for nucleotide in Nucleotide::ALL {
                let partner = matrix_partner(family, nucleotide);
                assert_eq!(partners[nucleotide.bits() as usize], partner);
                assert_ne!(partner, nucleotide, "no fixed points in {family:?}");
                assert_eq!(
                    matrix_partner(family, partner),
                    nucleotide,
                    "the pairing is an involution in {family:?}"
                );
            }
        }
        // The family unique-pair sets (vendor m3.h:143-156): Watson-Crick
        // ground for Complementary, cross-complementary AG/CT for
        // MovingResting, cross-diagonal AC/TG for SameQuality.
        assert_eq!(
            matrix_partner(MatrixFamily::Complementary, Nucleotide::A),
            Nucleotide::T
        );
        assert_eq!(
            matrix_partner(MatrixFamily::Complementary, Nucleotide::G),
            Nucleotide::C
        );
        assert_eq!(
            matrix_partner(MatrixFamily::MovingResting, Nucleotide::A),
            Nucleotide::G
        );
        assert_eq!(
            matrix_partner(MatrixFamily::MovingResting, Nucleotide::C),
            Nucleotide::T
        );
        assert_eq!(
            matrix_partner(MatrixFamily::SameQuality, Nucleotide::A),
            Nucleotide::C
        );
        assert_eq!(
            matrix_partner(MatrixFamily::SameQuality, Nucleotide::T),
            Nucleotide::G
        );
    }
}
