//! Conformance for `ql.pole.transcription/v1` — the N4 transcription
//! layer (DET masks, wave superposition, amino record, RNA phase law,
//! Parashakti ascent).
//!
//! Zero-transcription checks: every numeric table is re-derived from the
//! vendor C sources by an independent runtime parser (comment-stripping,
//! not the module's const scanner) and pinned byte-for-byte; the laws are
//! re-derived from first principles and cross-checked against
//! `ql_core::det_shadow` — the flat DET law's single home.

// The transcription module is compiled here through a `#[path]` shim
// because the pole wiring (`pub mod transcription` + re-exports) lands
// with the integrating agent: the shim points at the real source file,
// and the stand-in modules below re-export the same `ql_core` items
// under the `super::` / `crate::pole::` paths the module uses
// (`ql_core::pole` and its submodules are private until wired). The test
// exercises the exact bytes that will ship; nothing is duplicated.
pub use ql_core::QlError;
mod codon {
    pub use ql_core::{Codon64, MatrixFamily, PairIndex16};
}
mod coin {
    pub use ql_core::monoid;
}
mod nucleotide {
    pub use ql_core::Nucleotide;
}
mod pole {
    pub mod basis {
        pub use ql_core::det_shadow;
    }
}

#[path = "../src/pole/transcription.rs"]
pub mod transcription;

use ql_core::{Codon64, MatrixFamily, Nucleotide, PairIndex16, det_shadow};
use transcription::{
    AA_STOP_INDEX, AMINO_ACID_VOCABULARY, DNA_RNA_UNIQUE_FORMS, M2_TO_M3_CYMATIC_PROJECTION,
    M3_CODON_TO_AA, M3_MATRIX_PAIR, M3_PAIR_DIFFERENCE_SIGN, M3_RNA_DARK_MASK,
    M3_RNA_FUNCTIONAL_MASK, PARASHAKTI_SHADOW_OFFSET, POLE_TRANSCRIPTION_REF,
    RNA_T_CONTAINING_CODONS, RNA_T_FREE_CODONS, amino_acid_name, apply_epogdoon_compression,
    codon_parashakti_frequency, is_evolutionary_gap, is_stop_codon, m3_codon_amino_index,
    m3_codon_is_rna_capable, matrix_pair_nucleotides, matrix_partner, pair_difference, pair_sum,
    parashakti_frequency, rotational_total_sum_value, transduce_vibration_to_symbol,
};

const M2_C: &str = include_str!("../../../vendor/epi-kernel/reference/src/m2.c");
const M3_C: &str = include_str!("../../../vendor/epi-kernel/reference/src/m3.c");

/// Strip `/* … */` comments so the independent parsers see only code.
fn c_code(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i += 2;
            out.push(' ');
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

/// Independent decimal-array parser: entries between `anchor … }`.
fn parse_decimal_array(code: &str, anchor: &str, expected: usize) -> Vec<u16> {
    let start = code
        .find(anchor)
        .unwrap_or_else(|| panic!("anchor {anchor}"))
        + anchor.len();
    let end = code[start..]
        .find('}')
        .unwrap_or_else(|| panic!("close of {anchor}"));
    let values: Vec<u16> = code[start..start + end]
        .split(',')
        .filter_map(|field| field.trim().parse::<u16>().ok())
        .collect();
    assert_eq!(
        values.len(),
        expected,
        "vendor array {anchor} must carry {expected} entries"
    );
    values
}

/// Independent hex-array parser: the first `expected` `0x…` literals
/// after `anchor`.
fn parse_hex_array(code: &str, anchor: &str, expected: usize) -> Vec<u64> {
    let start = code
        .find(anchor)
        .unwrap_or_else(|| panic!("anchor {anchor}"))
        + anchor.len();
    let mut values = Vec::new();
    let mut rest = &code[start..];
    while values.len() < expected {
        let begin = rest
            .find("0x")
            .unwrap_or_else(|| panic!("hex literal for {anchor}"))
            + 2;
        rest = &rest[begin..];
        let digits = rest
            .find(|c: char| !c.is_ascii_hexdigit())
            .unwrap_or_else(|| panic!("hex digits for {anchor}"));
        values.push(u64::from_str_radix(&rest[..digits], 16).expect("hex value"));
        rest = &rest[digits..];
    }
    values
}

/// Independent single-hex parser: the first `0x…` literal after `anchor`.
fn parse_hex_value(code: &str, anchor: &str) -> u64 {
    let values = parse_hex_array(code, anchor, 1);
    values[0]
}

/// The `M3_PAIR_MATRIX` sum/difference columns, by pair index.
fn parse_pair_matrix(code: &str) -> [(i16, i16); 16] {
    let anchor = "M3_PAIR_MATRIX[16] = {";
    let start = code.find(anchor).expect("pair matrix anchor") + anchor.len();
    let end = code[start..].find("};").expect("pair matrix close");
    let mut table = [(0i16, 0i16); 16];
    let mut found = 0;
    for line in code[start..start + end].lines() {
        let line = line.trim();
        if !(line.starts_with('[') && line.contains('{') && line.contains('}')) {
            continue;
        }
        let close_bracket = line.find(']').expect("index close");
        let index: usize = line[1..close_bracket].trim().parse().expect("pair index");
        let open_brace = line.find('{').expect("entry open");
        let close_brace = line.find('}').expect("entry close");
        let numbers: Vec<i16> = line[open_brace + 1..close_brace]
            .split(',')
            .filter_map(|piece| piece.trim().parse().ok())
            .collect();
        assert_eq!(numbers.len(), 2, "two values per entry: {line}");
        table[index] = (numbers[0], numbers[1]);
        found += 1;
    }
    assert_eq!(found, 16, "the pair matrix must carry all 16 entries");
    table
}

/// The 12 `M3_NUC_` letters of `M3_MATRIX_PAIR`, row-major.
fn parse_matrix_pair_letters(code: &str) -> Vec<Nucleotide> {
    let anchor = "M3_MATRIX_PAIR[M3_MATRIX_COUNT][4] = {";
    let start = code.find(anchor).expect("matrix pair anchor") + anchor.len();
    let end = code[start..].find("};").expect("matrix pair close");
    let mut letters = Vec::new();
    let mut rest = &code[start..start + end];
    while let Some(marker) = rest.find("M3_NUC_") {
        let after = &rest[marker + "M3_NUC_".len()..];
        let letter = after.chars().next().expect("nucleotide letter");
        letters.push(
            Nucleotide::try_from(match letter {
                'A' => 0,
                'T' => 1,
                'C' => 2,
                'G' => 3,
                other => panic!("unexpected M3_NUC_ letter {other}"),
            })
            .expect("nucleotide bits"),
        );
        rest = after;
    }
    assert_eq!(letters.len(), 12, "three families × four nucleotides");
    letters
}

fn codon_symbols(codon: Codon64) -> String {
    codon.nucleotides().iter().map(|n| n.symbol()).collect()
}

#[test]
fn contract_identity_is_semantically_versioned() {
    assert_eq!(POLE_TRANSCRIPTION_REF, "ql.pole.transcription/v1");
}

// ===================================================================
// FR 2.3.0 / M3-C02 — DET projection masks
// ===================================================================

#[test]
fn det_masks_are_the_vendor_table_byte_for_byte() {
    let parsed = parse_hex_array(&c_code(M2_C), "M2_TO_M3_CYMATIC_PROJECTION[72] = {", 72);
    assert_eq!(parsed, M2_TO_M3_CYMATIC_PROJECTION.to_vec(), "m2.c:821-895");
}

#[test]
fn det_masks_cross_check_against_ql_core_det_shadow() {
    // The flat DET law's single home is pole::basis::det_shadow; the mask
    // table is its wave projection and must agree address for address:
    // states 0-63 identity, states 64-71 folded onto bits 0, 8, …, 56.
    for state in 0u8..72 {
        assert_eq!(
            M2_TO_M3_CYMATIC_PROJECTION[state as usize],
            1u64 << det_shadow(state).unwrap(),
            "state {state}"
        );
    }
    let mut union = 0u64;
    for mask in M2_TO_M3_CYMATIC_PROJECTION {
        assert_eq!(mask.count_ones(), 1, "one-hot mask");
        union |= mask;
    }
    assert_eq!(union.count_ones(), 64, "72 × 8/9 = 64");
}

#[test]
fn wave_superposition_law_holds() {
    assert_eq!(transduce_vibration_to_symbol(&[]).unwrap(), 0);
    assert_eq!(transduce_vibration_to_symbol(&[0, 1]).unwrap(), 0b11);
    // Folded states collapse onto their targets: 64 and 0 share bit 0.
    assert_eq!(transduce_vibration_to_symbol(&[0, 64]).unwrap(), 1);
    assert!(transduce_vibration_to_symbol(&[72]).is_err());
    assert!(transduce_vibration_to_symbol(&[0, 255]).is_err());
}

// ===================================================================
// FR 2.3.6 — epogdoon arithmetic + Parashakti ascent
// ===================================================================

#[test]
fn epogdoon_laws_hold_on_the_72_cycle() {
    for index in 0u8..72 {
        assert_eq!(
            apply_epogdoon_compression(index).unwrap(),
            (u32::from(index) * 8 / 9) as u8,
        );
    }
    // The lossy round trip keeps exactly the multiples of 9.
    let survivors: Vec<u8> = (0u8..72)
        .filter(|index| !is_evolutionary_gap(*index).unwrap())
        .collect();
    assert_eq!(survivors, vec![0, 9, 18, 27, 36, 45, 54, 63]);
    assert!(apply_epogdoon_compression(72).is_err());
    assert!(is_evolutionary_gap(72).is_err());
}

#[test]
fn pair_matrix_columns_agree_with_the_vendor_table() {
    let vendor = parse_pair_matrix(&c_code(M3_C));
    for first in Nucleotide::ALL {
        for second in Nucleotide::ALL {
            let index = ((first.bits() << 2) | second.bits()) as usize;
            let (sum, difference) = vendor[index];
            assert_eq!(pair_sum(first, second), sum, "sum for {first}{second}");
            assert_eq!(
                pair_difference(first, second),
                difference,
                "recorded difference for {first}{second} (sign provenance, M3 unresolved item 2)"
            );
            // The restated provenance table is the signum of the recorded
            // column (0 = homogeneous).
            let expected_sign = if difference == 0 {
                0i8
            } else if difference > 0 {
                1
            } else {
                -1
            };
            assert_eq!(M3_PAIR_DIFFERENCE_SIGN[index], expected_sign);
        }
    }
}

#[test]
fn parashakti_ascent_law_holds() {
    assert_eq!(PARASHAKTI_SHADOW_OFFSET, 36);
    // Every pair combination: base 24-36, shadow +36 → 60-72.
    for first in 0u8..16 {
        for second in 0u8..16 {
            let pair_xy = PairIndex16::from_index(first).unwrap();
            let pair_yz = PairIndex16::from_index(second).unwrap();
            let base = parashakti_frequency(pair_xy, pair_yz, false);
            let shadow = parashakti_frequency(pair_xy, pair_yz, true);
            assert_eq!(shadow, base + 36);
            assert!((24..=36).contains(&base));
            assert!((60..=72).contains(&shadow));
            assert_eq!(
                rotational_total_sum_value(pair_xy, pair_yz),
                i16::from(base)
            );
        }
    }
    // Codon-level: the codon reading composes its own two hinge pairs.
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        let base = parashakti_frequency(codon.pair_xy(), codon.pair_yz(), false);
        let shadow = parashakti_frequency(codon.pair_xy(), codon.pair_yz(), true);
        assert_eq!(codon_parashakti_frequency(codon, false), base);
        assert_eq!(codon_parashakti_frequency(codon, true), shadow);
        assert_eq!(codon_parashakti_frequency(codon, true), base + 36);
    }
}

// ===================================================================
// FR 2.3.9 / M3-C18 — the amino record
// ===================================================================

#[test]
fn amino_record_is_the_vendor_table_byte_for_byte() {
    let parsed = parse_decimal_array(&c_code(M3_C), "M3_CODON_TO_AA[64] = {", 64);
    let recorded: Vec<u16> = M3_CODON_TO_AA.iter().map(|slot| u16::from(*slot)).collect();
    assert_eq!(parsed, recorded, "m3.c:228-253");
}

#[test]
fn amino_record_is_the_kernel_simplified_record() {
    // Every codon lands in the 24-slot vocabulary (or its STOP slot).
    for address in 0u8..64 {
        assert!(
            u32::from(m3_codon_amino_index(Codon64::new(address))) < 24,
            "codon {address} outside the 24-slot vocabulary"
        );
    }
    assert_eq!(AMINO_ACID_VOCABULARY.len(), 24);
    assert_eq!(amino_acid_name(0), Some("Phe"));
    assert_eq!(amino_acid_name(3), Some("Met"));
    assert_eq!(amino_acid_name(AA_STOP_INDEX), Some("Stop"));
    assert_eq!(amino_acid_name(24), None);
    // The recorded stops: TAA, TAG, TGA — slot 10, not the 0xFF sentinel
    // of the vendor header comment (m3.c:207).
    let stops: Vec<String> = (0u8..64)
        .map(Codon64::new)
        .filter(|codon| is_stop_codon(*codon))
        .map(codon_symbols)
        .collect();
    assert_eq!(stops, vec!["TAA", "TAG", "TGA"]);
    // The deep matrix M3-C18 audit stays open: the vendor's own
    // simplifications remain recorded verbatim (m3.c:237 "mapping
    // simplified") — ATA is recorded Met, not the standard Ile.
    let ata = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::T, Nucleotide::A);
    assert_eq!(m3_codon_amino_index(ata), 3);
    assert_eq!(amino_acid_name(3), Some("Met"));
}

// ===================================================================
// FR 2.3.20 / M3-C17 — the RNA phase law
// ===================================================================

#[test]
fn rna_masks_are_the_vendor_values() {
    // First textual occurrence of each name is its definition (m3.c:133,
    // m3.c:134); the kernel's later uses are asserts and calls.
    assert_eq!(
        parse_hex_value(&c_code(M3_C), "M3_RNA_FUNCTIONAL_MASK"),
        M3_RNA_FUNCTIONAL_MASK,
        "m3.c:133"
    );
    assert_eq!(
        parse_hex_value(&c_code(M3_C), "M3_RNA_DARK_MASK"),
        M3_RNA_DARK_MASK,
        "m3.c:134"
    );
}

#[test]
fn rna_masks_are_disjoint_codon_addressed_bitboards() {
    // The kernel's own static asserts (m3.c:136-139), restated.
    assert_eq!(M3_RNA_FUNCTIONAL_MASK & M3_RNA_DARK_MASK, 0);
    assert_eq!(M3_RNA_FUNCTIONAL_MASK | M3_RNA_DARK_MASK, u64::MAX);
    // Bit i addresses codon i.
    for address in 0u8..64 {
        assert_eq!(
            m3_codon_is_rna_capable(Codon64::new(address)),
            (M3_RNA_FUNCTIONAL_MASK >> address) & 1 == 1,
            "codon {address}"
        );
    }
}

#[test]
fn ratified_transcription_model_counts_hold() {
    // Deep matrix §9 / M3-C17, from first principles over the alphabet:
    // T-free = 3³ = 27, T-containing = 64 − 27 = 37, unique forms 101.
    let mut t_free: usize = 0;
    for outer in Nucleotide::ALL {
        for middle in Nucleotide::ALL {
            for inner in Nucleotide::ALL {
                if !m3_codon_is_rna_capable(Codon64::from_nucleotides(outer, middle, inner)) {
                    t_free += 1;
                }
            }
        }
    }
    assert_eq!(t_free, 27);
    assert_eq!(t_free, RNA_T_FREE_CODONS);
    assert_eq!(RNA_T_CONTAINING_CODONS, 37);
    assert_eq!(RNA_T_FREE_CODONS + RNA_T_CONTAINING_CODONS, 64);
    assert_eq!(64 + RNA_T_CONTAINING_CODONS, DNA_RNA_UNIQUE_FORMS);
    assert_eq!(DNA_RNA_UNIQUE_FORMS, 101);
}

// ===================================================================
// M3-C09 — matrix family → nucleotide binding
// ===================================================================

#[test]
fn matrix_pair_binding_is_the_vendor_table() {
    let parsed = parse_matrix_pair_letters(&c_code(M3_C));
    for family in MatrixFamily::ALL {
        let row = family as usize; // declared order: 0, 1, 2
        assert_eq!(
            parsed[row * 4..(row + 1) * 4].to_vec(),
            matrix_pair_nucleotides(family).to_vec(),
            "family {family:?}"
        );
        assert_eq!(M3_MATRIX_PAIR[row], matrix_pair_nucleotides(family));
    }
}

#[test]
fn matrix_pair_maps_are_involutions_without_fixed_points() {
    for family in MatrixFamily::ALL {
        for nucleotide in Nucleotide::ALL {
            let partner = matrix_partner(family, nucleotide);
            assert_ne!(partner, nucleotide, "no fixed points in {family:?}");
            assert_eq!(
                matrix_partner(family, partner),
                nucleotide,
                "involution in {family:?}"
            );
        }
    }
}

#[test]
fn matrix_families_bind_their_unique_pairs_over_the_shared_ground() {
    // Vendor m3.h:143-156: the 4 homogeneous pairs are shared across all
    // three matrices; each row's involution generates the family's four
    // unique pairs — Complementary AT/CG, MovingResting AG/CT,
    // SameQuality AC/TG.
    let unique_pairs = |family| {
        let mut pairs = Vec::new();
        for nucleotide in Nucleotide::ALL {
            let partner = matrix_partner(family, nucleotide);
            if nucleotide.bits() < partner.bits() {
                pairs.push((nucleotide, partner));
            }
        }
        pairs
    };
    assert_eq!(
        unique_pairs(MatrixFamily::Complementary),
        vec![
            (Nucleotide::A, Nucleotide::T),
            (Nucleotide::C, Nucleotide::G)
        ]
    );
    assert_eq!(
        unique_pairs(MatrixFamily::MovingResting),
        // collected in nucleotide-bit order: T(1) before C(2)
        vec![
            (Nucleotide::A, Nucleotide::G),
            (Nucleotide::T, Nucleotide::C)
        ]
    );
    assert_eq!(
        unique_pairs(MatrixFamily::SameQuality),
        vec![
            (Nucleotide::A, Nucleotide::C),
            (Nucleotide::T, Nucleotide::G)
        ]
    );
}
