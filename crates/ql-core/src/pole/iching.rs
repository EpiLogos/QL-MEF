//! The I-Ching transformation grammar at full depth (T-I).
//!
//! Ports the structural hexagram layer of the C reference kernel:
//!
//! ```text
//! FR 2.3.1  trigram LUT with earlier/later-heaven, element, family role,
//!           degree anchor            (vendor m3.c M3_TRIGRAM_LUT)
//! FR 2.3.1  hexagram decomposition:  upper/lower trigram, complement,
//!           nuclear trigrams         (vendor m3.c M3_HEXAGRAM_LUT HEX macro)
//! FR 2.3.7  SU(2)-preserving polar opposite over the 720° double cover
//! FR 2.3.11 integral symmetry on the 64-bit field (byte swap)
//! FR 2.3.17 the 16 palindromic (XyX) non-dual anchors
//! ```
//!
//! Coordinates: capability M3-C06 (384-edge transformation graph — the edge
//! law itself lives on [`crate::pole::codon::Codon64`]), M3-C07 (trigram /
//! complement / nuclear operations), M3-C12 (non-dual anchors); branch M3-1.
//! Everything here is structural: nucleotide values never enter.
//!
//! The world-clock LUT (`CLOCK_DEGREE_LUT`, M3-C24/C26) is deliberately NOT
//! ported as data: its dataset-backed columns (hexagram/tarot/ananda/archetype)
//! live in the external deep specimen and were never filled in the vendored
//! file; the computable column laws (zodiac, decan, backbone, shadow, polar
//! opposite) are the arithmetic in this module and in [`crate::pole::aperture`].

use crate::QlError;
use core::fmt;

/// Semantic identity of the I-Ching grammar port.
pub const ICHING_GRAMMAR_REF: &str = "ql.pole.iching-grammar/v1";

/// One of the eight trigrams, with its full attributions.
///
/// Field order and values are the kernel's (`M3_TRIGRAM_LUT`, vendor m3.c).
/// `element` uses the M2 five-element id spine (0 Water, 2 Fire, 3 Wood,
/// 4 Earth as recorded); `degree_anchor` is the trigram's anchor on the
/// #3-5 wheel in whole degrees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Trigram {
    /// Kernel trigram id 0..8 (Qian 0, Kun 1, Zhen 2, Xun 3, Kan 4, Li 5,
    /// Gen 6, Dui 7).
    pub id: u8,
    /// Three-bit line pattern (yang = 1).
    pub binary: u8,
    /// Position in the Earlier Heaven arrangement.
    pub earlier_heaven: u8,
    /// Position in the Later Heaven arrangement.
    pub later_heaven: u8,
    /// M2 five-element id spine (as recorded in the kernel LUT).
    pub element: u8,
    /// Family role (Father/Mother/Son/Daughter register, kernel encoding).
    pub family_role: u8,
    /// Degree anchor on the #3-5 wheel.
    pub degree_anchor: u16,
}

/// Line-pattern → trigram id (the LUT is id-indexed, not pattern-indexed).
const MATCH_BINARY_TO_ID: [usize; 8] = [1, 2, 4, 7, 6, 5, 3, 0];

impl Trigram {
    /// The kernel trigram LUT, verbatim.
    pub const LUT: [Trigram; 8] = [
        Trigram {
            id: 0,
            binary: 0x07,
            earlier_heaven: 0,
            later_heaven: 5,
            element: 4,
            family_role: 0,
            degree_anchor: 0,
        },
        Trigram {
            id: 1,
            binary: 0x00,
            earlier_heaven: 7,
            later_heaven: 0,
            element: 4,
            family_role: 1,
            degree_anchor: 180,
        },
        Trigram {
            id: 2,
            binary: 0x01,
            earlier_heaven: 4,
            later_heaven: 1,
            element: 3,
            family_role: 2,
            degree_anchor: 90,
        },
        Trigram {
            id: 3,
            binary: 0x06,
            earlier_heaven: 3,
            later_heaven: 6,
            element: 3,
            family_role: 3,
            degree_anchor: 315,
        },
        Trigram {
            id: 4,
            binary: 0x02,
            earlier_heaven: 6,
            later_heaven: 7,
            element: 0,
            family_role: 4,
            degree_anchor: 270,
        },
        Trigram {
            id: 5,
            binary: 0x05,
            earlier_heaven: 1,
            later_heaven: 2,
            element: 2,
            family_role: 5,
            degree_anchor: 135,
        },
        Trigram {
            id: 6,
            binary: 0x04,
            earlier_heaven: 2,
            later_heaven: 3,
            element: 4,
            family_role: 6,
            degree_anchor: 45,
        },
        Trigram {
            id: 7,
            binary: 0x03,
            earlier_heaven: 5,
            later_heaven: 4,
            element: 4,
            family_role: 7,
            degree_anchor: 225,
        },
    ];

    pub const fn from_id(id: u8) -> Result<Self, QlError> {
        if id < 8 {
            Ok(Self::LUT[id as usize])
        } else {
            Err(QlError::InvalidPoleValue {
                field: "trigram-id",
                value: id as u32,
            })
        }
    }

    /// The trigram whose line pattern matches (pattern, not id): binary
    /// 000→Kun, 001→Zhen, 010→Kan, 011→Dui, 100→Gen, 101→Li, 110→Xun,
    /// 111→Qian.
    pub const fn from_binary(binary: u8) -> Self {
        Self::LUT[MATCH_BINARY_TO_ID[(binary & 0x07) as usize]]
    }
}

impl fmt::Display for Trigram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = ["Qian", "Kun", "Zhen", "Xun", "Kan", "Li", "Gen", "Dui"];
        write!(f, "{}", names[self.id as usize])
    }
}

/// Upper trigram of a hexagram id: bits 3-5.
pub const fn upper_trigram_id(hex: u8) -> u8 {
    (hex >> 3) & 0x07
}

/// Lower trigram of a hexagram id: bits 0-2.
pub const fn lower_trigram_id(hex: u8) -> u8 {
    hex & 0x07
}

/// Compose a hexagram id from trigram ids: `(upper << 3) | lower`.
pub const fn compose_hexagram(upper: u8, lower: u8) -> Result<u8, QlError> {
    if upper < 8 && lower < 8 {
        Ok((upper << 3) | lower)
    } else {
        Err(QlError::InvalidPoleValue {
            field: "trigram-id",
            value: ((upper as u32) << 8) | lower as u32,
        })
    }
}

/// Complement: all six lines inverted (`id ^ 0x3F`).
///
/// The same law the Complementary matrix family applies as a fold
/// (see [`crate::pole::fold`]); exposed here at the hexagram register.
pub const fn complement(hex: u8) -> u8 {
    hex ^ 0x3F
}

/// Nuclear upper trigram id: lines 2-4 → `(hex >> 2) & 0x07`.
pub const fn nuclear_upper(hex: u8) -> u8 {
    (hex >> 2) & 0x07
}

/// Nuclear lower trigram id: lines 1-3 → `(hex >> 1) & 0x07`.
pub const fn nuclear_lower(hex: u8) -> u8 {
    (hex >> 1) & 0x07
}

/// The nuclear hexagram: the inner repetition a form carries at the deeper
/// scale — `upper<<3 | lower` of its nuclear trigrams.
pub const fn nuclear_hexagram(hex: u8) -> u8 {
    (nuclear_upper(hex) << 3) | nuclear_lower(hex)
}

/// Integral symmetry on the 64-bit M3 word: byte reversal
/// (vendor m3.h `integral_symmetry_field`, `__builtin_bswap64`).
pub const fn integral_symmetry_field(word: u64) -> u64 {
    word.swap_bytes()
}

/// SU(2)-preserving polar opposite over the 720° double cover (FR 2.3.7).
///
/// Opposing a shadow-layer degree stays in the shadow layer: the layer
/// offset is preserved and the base degree advances by 180°. The
/// bimba/pratibimba double cover is first-class state — the antipode never
/// collapses the two passes into one 360° wheel.
pub const fn polar_opposite_su2(degree720: u16) -> u16 {
    let layer_offset = if degree720 >= 360 { 360u16 } else { 0 };
    let base = degree720 % 360;
    layer_offset + (base + 180) % 360
}

/// Simple wheel operations on the single 360° layer.
pub const fn flow_clockwise(degree: u16) -> u16 {
    (degree + 1) % 360
}

/// Simple (layer-blind) polar opposite.
pub const fn polar_opposite_simple(degree: u16) -> u16 {
    (degree + 180) % 360
}

/// The quadrant of a primary-layer degree (0-3, N/E/S/W register).
pub const fn quadrant(degree: u16) -> u8 {
    (degree / 90) as u8
}

/// The 16 palindromic non-dual anchors: codons XyX (outer == inner).
///
/// Derived by the kernel's own construction (`M3_NONDUAL_CODONS` is exactly
/// `(X<<4)|(y<<2)|X` over all X, y); verified against the vendor list by
/// test.
pub const fn palindromic_anchors() -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut i = 0;
    let mut x = 0;
    while x < 4 {
        let mut y = 0;
        while y < 4 {
            out[i] = (x << 4) | (y << 2) | x;
            i += 1;
            y += 1;
        }
        x += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigram_lut_matches_the_kernel_law() {
        // Line patterns: Qian 111, Kun 000, and the id ordering of the LUT.
        assert_eq!(Trigram::from_binary(0x07).id, 0);
        assert_eq!(Trigram::from_binary(0x00).id, 1);
        assert_eq!(Trigram::from_binary(0x02).id, 4, "Kan carries pattern 010");
        assert_eq!(Trigram::from_binary(0x05).id, 5, "Li carries pattern 101");
        // Water sits at Kan, Fire at Li (M2 element ids 0 and 2).
        assert_eq!(Trigram::from_id(4).unwrap().element, 0);
        assert_eq!(Trigram::from_id(5).unwrap().element, 2);
        assert!(Trigram::from_id(8).is_err());
        // Degree anchors are the recorded wheel anchors.
        assert_eq!(Trigram::from_id(1).unwrap().degree_anchor, 180);
        assert_eq!(Trigram::from_id(6).unwrap().degree_anchor, 45);
    }

    #[test]
    fn trigram_decomposition_round_trips() {
        for hex in 0u8..64 {
            let upper = upper_trigram_id(hex);
            let lower = lower_trigram_id(hex);
            assert_eq!(compose_hexagram(upper, lower).unwrap(), hex);
            // Pattern lookup: the lower three bits name the lower trigram by
            // line pattern; bits 3-5 the upper.
            assert_eq!(Trigram::from_binary(hex & 0x07).binary, hex & 0x07);
            assert_eq!(
                Trigram::from_binary((hex >> 3) & 0x07).binary,
                (hex >> 3) & 0x07
            );
        }
        // Hexagram 63 (Qian/Qian) decomposes to Qian on both positions.
        assert_eq!(Trigram::from_binary(upper_trigram_id(63)).id, 0);
        assert_eq!(Trigram::from_binary(lower_trigram_id(63)).id, 0);
        // Hexagram 0 decomposes to Kun on both positions.
        assert_eq!(Trigram::from_binary(upper_trigram_id(0)).id, 1);
        assert_eq!(Trigram::from_binary(lower_trigram_id(0)).id, 1);
    }

    #[test]
    fn complement_is_involutive_and_agrees_with_the_matrix_fold() {
        for hex in 0u8..64 {
            assert_eq!(complement(complement(hex)), hex);
            // The hexagram complement and the complementary matrix family
            // act identically on the address (the fold law of pole::fold).
            use crate::pole::{FoldState, MatrixFamily};
            let state = FoldState::from_codon(
                crate::pole::Codon64::new(hex),
                crate::pole::ApertureIndex::new(0).expect("aperture"),
                0,
            );
            let complemented = state
                .apply_matrix(MatrixFamily::Complementary)
                .expect("complement applies")
                .applied()
                .expect("complement is always admitted")
                .codon()
                .address();
            assert_eq!(complemented, complement(hex));
        }
    }

    #[test]
    fn nuclear_trigrams_extract_the_inner_repetition() {
        // HEX macro law: nuclear_upper = (i>>2)&7, nuclear_lower = (i>>1)&7.
        for hex in 0u8..64 {
            assert_eq!(nuclear_upper(hex), (hex >> 2) & 0x07);
            assert_eq!(nuclear_lower(hex), (hex >> 1) & 0x07);
            // The nuclear hexagram is again a lawful hexagram id.
            let nuclear = nuclear_hexagram(hex);
            assert!(nuclear < 64);
        }
        // Hexagram 0 (Kun/Kun) is its own nuclear repetition.
        assert_eq!(nuclear_hexagram(0), 0);
        assert_eq!(nuclear_hexagram(63), 63);
    }

    #[test]
    fn integral_symmetry_is_a_byte_reversal() {
        let word: u64 = 0x0102030405060708;
        assert_eq!(integral_symmetry_field(word), 0x0807060504030201);
        assert_eq!(integral_symmetry_field(integral_symmetry_field(word)), word);
    }

    #[test]
    fn su2_polar_opposite_preserves_the_layer() {
        // Primary stays primary, shadow stays shadow.
        assert_eq!(polar_opposite_su2(0), 180);
        assert_eq!(polar_opposite_su2(180), 0);
        assert_eq!(polar_opposite_su2(359), 179);
        assert_eq!(polar_opposite_su2(360), 540, "shadow layer preserved");
        assert_eq!(polar_opposite_su2(540), 360);
        assert_eq!(polar_opposite_su2(719), 539);
        // The layer-blind simple wheel would collapse the cover — the typed
        // difference between the two operations is the point.
        assert_eq!(polar_opposite_simple(360), 180);
        assert_ne!(polar_opposite_su2(360), polar_opposite_simple(360));
    }

    #[test]
    fn wheel_operations_close() {
        assert_eq!(flow_clockwise(359), 0);
        assert_eq!(quadrant(0), 0);
        assert_eq!(quadrant(90), 1);
        assert_eq!(quadrant(359), 3);
    }

    #[test]
    fn palindromic_anchors_match_the_kernel_list() {
        // The kernel's M3_NONDUAL_CODONS[16] (vendor m3.c) is exactly the
        // XyX construction; pinned here against the recorded literals.
        let kernel_list: [u8; 16] = [
            0x00, 0x04, 0x08, 0x0C, 0x11, 0x15, 0x19, 0x1D, 0x22, 0x26, 0x2A, 0x2E, 0x33, 0x37,
            0x3B, 0x3F,
        ];
        assert_eq!(palindromic_anchors(), kernel_list);
        // Every anchor is outer == inner; every outer==inner codon appears.
        let mut count = 0;
        for address in 0u8..64 {
            let codon = crate::pole::Codon64::new(address);
            if codon.is_palindromic() {
                count += 1;
                assert!(kernel_list.contains(&address));
            }
        }
        assert_eq!(count, 16);
    }
}
