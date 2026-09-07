//! The M3 rotational machinery (FR 2.3.14) — the dataset-backed reflection
//! profile of the 64 codons, the pair S/D law of the generation register, the
//! two-family 8-fold generation law, and the Watson-Crick anticodon.
//!
//! Kernel coordinates: FR 2.3.14 (the 8-fold rotational composition law,
//! `vendor/epi-kernel/reference/include/m3.h:517-582`), FR 2.3.17b (the
//! three-tier 40 non-dual / 24 dual class split,
//! `include/m3.h:672-748`), capability `M3-C13` (the 472-state rotational
//! surface), `M3-C12` (class / non-duality), `M3-C15` (orientation
//! refinement), branch `M3-3`. The C evidence is
//! `vendor/epi-kernel/reference/src/m3.c`: the profile table
//! `M3_ROTATIONAL_PROFILE[64]` (`m3.c:396-431`, built by the `R7`/`R8`/`R8P`
//! macros), the generation law `m3_generate_rotational_states`
//! (`m3.c:513-588`), the profile getter (`m3.c:590-592`) and the anticodon
//! `wc_anticodon` (`m3.c:601-607`); the boot-time integrity laws of
//! `m3_verify` (`m3.c:758-791`: 40 seven-state, 24 eight-state, 40 anchored,
//! 16 pair links).
//!
//! Two registers meet here. The dataset-backed PROFILE (state counts 7/8,
//! state types, anchor pairs, court-reflection links — source
//! `nodes-full-detail.json` codon reflections) is the lawful SURFACE: 40
//! non-dual codons × 7 + 24 dual codons × 8 = 472 rotational states, agreeing
//! entry-for-entry with the algorithmic classifier
//! ([`super::codon::Codon64::rotational_state_count`]). The GENERATION law is
//! the 8-fold candidate sweep that serves the surface: every codon generates
//! exactly eight ranked orientations — four of each valence, the middle site
//! sweeping the whole alphabet — from which the profile count admits the
//! lawful poses ([`super::pose::RotationalPose`]).
//!
//! The port truth (verified exhaustively, see `tests/pole_rotational.rs`):
//! both valences produce the codon `(X, swept, Z)`, so the eight candidates
//! span exactly FOUR distinct resulting codons × two valences for every
//! input codon — the 7/8 profile split is dataset provenance, NOT a
//! distinctness count of the candidate set. The one literal collapse is the
//! perfect-palindrome bipolar state: there both valences emit the identical
//! candidate (same pairs, codon, value), leaving 7 distinct candidate
//! records. The vendor ranking assigns slots ASCENDING by rotational value
//! (slot 0 = the lowest value), negative valence before positive within
//! equal values, stable within a (value, valence) class — this contradicts
//! the "descending" reading of the FR prose and follows the C implementation
//! verbatim (`m3.c:565-585`).

use super::codon::{Codon64, PairIndex16};
use super::nucleotide::Nucleotide;
use crate::QlError;

/// Candidates per generation sweep (`M3_ROTATIONAL_TABLE_ENTRIES`,
/// `m3.h:525`): four negative + four positive orientations.
pub const ROTATIONAL_TABLE_ENTRIES: usize = 8;

/// Polarized (non-bipolar) entries of the 8-fold sweep
/// (`M3_ROTATIONAL_POLARIZED_ENTRIES`, `m3.h:526`): the 7 lawful states of
/// the non-dual surface.
pub const POLARIZED_ENTRIES: usize = 7;

/// The one collapsed non-dual orientation (`M3_ROTATIONAL_NONDUAL_ENTRIES`,
/// `m3.h:527`): 8 − 1 = 7 lawful states on the non-dual surface.
pub const NONDUAL_ENTRIES: usize = 1;

/// The vendor "no anchor pair" sentinel (`M3_ROTATIONAL_NO_PAIR`,
/// `m3.h:528`).
pub const NO_PAIR: u8 = 0xFF;

/// The vendor "no court reflection" sentinel (`M3_ROTATIONAL_NO_PAIRING`,
/// `m3.h:529`).
pub const NO_PAIRING: u8 = 0xFF;

/// Degrees per ranked rotation slot: the eighth of the quarter turn.
pub const ROTATION_SLOT_DEGREES: u16 = 45;

/// Semantic identity of the dataset-backed rotational profile contract.
pub const POLE_ROTATIONAL_PROFILE_REF: &str = "ql.pole.rotational-profile/v1";

/// The recorded dataset sign of each pair difference (`M3_PAIR_MATRIX`
/// difference values, `m3.c:32-56`; M3 unresolved item 2 — the class-stable
/// semantics of `differenceValue` stays open, the signs are provenance
/// preserved verbatim). Indexed by [`PairIndex16`]; 0 marks the homogeneous
/// pairs. Magnitudes follow the coin law
/// |v(first) − v(second)| (0 for homogeneous), sums are always
/// v(first) + v(second) under the canonical table {A=6, T=9, C=8, G=7}
/// ([`Nucleotide::NUCLEOTIDE_COIN_VALUE`]). The same signs stand in
/// `tests/pole_coin_contract.rs` (`recorded_signs`) and are pinned against
/// the vendor table byte-for-byte by `tests/pole_rotational.rs`.
pub const RECORDED_PAIR_DIFF_SIGNS: [i8; 16] = [
    0,  // AA (K'un)
    -1, // AT (Tui)
    -1, // AC
    1,  // AG
    1,  // TA (Ken)
    0,  // TT (Ch'ien — max sum 18)
    -1, // TC
    1,  // TG
    1,  // CA
    -1, // CT
    0,  // CC (K'an)
    1,  // CG
    1,  // GA
    -1, // GT
    -1, // GC (Sun)
    0,  // GG (Li)
];

/// The pair sum law of the generation register: `sum = v(first) + v(second)`
/// for every pair (`m3.c:19-21`, regenerated under the corrected value
/// table). The S of the pair S/D accessor.
pub const fn pair_sum(pair: PairIndex16) -> i16 {
    pair.first().coin_value().value() as i16 + pair.second().coin_value().value() as i16
}

/// The pair difference law of the generation register:
/// `|diff| = |v(first) − v(second)|` for mixed pairs, `0` for homogeneous,
/// with the sign of [`RECORDED_PAIR_DIFF_SIGNS`] (recorded dataset
/// provenance). The D of the pair S/D accessor.
pub const fn pair_difference(pair: PairIndex16) -> i16 {
    RECORDED_PAIR_DIFF_SIGNS[pair.index() as usize] as i16
        * (pair.first().coin_value().value() as i16 - pair.second().coin_value().value() as i16)
            .abs()
}

/// The vendor polarity of the rotational composition
/// (`M3_Rotational_Polarity`, `m3.h:531-534`; `M3_ROTATIONAL_NEGATIVE = 0`
/// ranks before positive in the slot ranking).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RotationalPolarity {
    /// Negative valence: `Xy + Za → XZa` (`m3.h:519`).
    Negative,
    /// Positive valence: `Xy + Za → Xya` (`m3.h:518`).
    Positive,
}

impl RotationalPolarity {
    /// Ranking rank: negative (0) precedes positive (1) within equal
    /// rotational values (`m3.c:571-572`).
    const fn rank(self) -> u8 {
        match self {
            Self::Negative => 0,
            Self::Positive => 1,
        }
    }
}

/// The dataset-backed state type of a codon's reflection
/// (`M3_Rotational_State_Type`, `m3.h:536-539`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RotationalStateType {
    /// 7 states, anchored at the palindrome's two pair orientations
    /// (`M3_ROTATIONAL_NON_DUAL_INITIATED`).
    NonDualInitiated,
    /// 8 states, the full rotational sweep (`M3_ROTATIONAL_FULL_ROTATIONAL`).
    FullRotational,
}

/// The dataset-backed rotational reflection profile of one codon
/// (`M3_Rotational_Profile`, `m3.h:551-558`; table `m3.c:405-425`).
///
/// - `state_count` — the dataset-backed 7 or 8 (`nodes-full-detail.json`
///   `stateCount`),
/// - `state_type` — non-dual-initiated or full-rotational (`stateType`),
/// - `anchor_pair_a`/`anchor_pair_b` — the 7-state anchor pair indices
///   (`nonDualPair`; present exactly on [`RotationalStateType::NonDualInitiated`]),
/// - `paired_codon` — the dual-codon court reflection (`pairedWith`;
///   present on 16 of the 24 dual codons).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotationalProfile {
    state_count: u8,
    state_type: RotationalStateType,
    anchor_pair_a: Option<PairIndex16>,
    anchor_pair_b: Option<PairIndex16>,
    paired_codon: Option<Codon64>,
}

impl RotationalProfile {
    /// Typed decode of the raw vendor record: state count, state type, the
    /// two anchor pair indices ([`NO_PAIR`] = none) and the paired-codon
    /// address ([`NO_PAIRING`] = none). Validates exactly the laws
    /// `m3_verify` enforces (`m3.c:758-791`): count 7 or 8, count agreeing
    /// with the state type, anchors present iff non-dual-initiated.
    pub fn try_from_raw(
        state_count: u8,
        state_type: RotationalStateType,
        anchor_pair_a: u8,
        anchor_pair_b: u8,
        paired_codon: u8,
    ) -> Result<Self, QlError> {
        let seven = POLARIZED_ENTRIES as u8;
        let eight = (POLARIZED_ENTRIES + NONDUAL_ENTRIES) as u8;
        if state_count != seven && state_count != eight {
            return Err(QlError::InvalidPoleValue {
                field: "rotational-state-count",
                value: state_count as u32,
            });
        }
        let expected = match state_type {
            RotationalStateType::NonDualInitiated => seven,
            RotationalStateType::FullRotational => eight,
        };
        if state_count != expected {
            return Err(QlError::InvalidPoleValue {
                field: "rotational-state-type",
                value: state_count as u32,
            });
        }
        let anchor_a = if anchor_pair_a == NO_PAIR {
            None
        } else {
            Some(PairIndex16::from_index(anchor_pair_a)?)
        };
        let anchor_b = if anchor_pair_b == NO_PAIR {
            None
        } else {
            Some(PairIndex16::from_index(anchor_pair_b)?)
        };
        match (state_type, anchor_a, anchor_b) {
            (RotationalStateType::NonDualInitiated, Some(_), Some(_))
            | (RotationalStateType::FullRotational, None, None) => {}
            _ => {
                return Err(QlError::InvalidPoleValue {
                    field: "rotational-anchor-pair",
                    value: u32::from(anchor_pair_a),
                });
            }
        }
        let paired_codon = if paired_codon == NO_PAIRING {
            None
        } else {
            if paired_codon >= Codon64::COUNT as u8 {
                return Err(QlError::InvalidPoleValue {
                    field: "paired-codon",
                    value: paired_codon as u32,
                });
            }
            Some(Codon64::new(paired_codon))
        };
        Ok(Self {
            state_count,
            state_type,
            anchor_pair_a: anchor_a,
            anchor_pair_b: anchor_b,
            paired_codon,
        })
    }

    /// The dataset-backed state count: 7 or 8.
    pub const fn state_count(self) -> u8 {
        self.state_count
    }

    /// The dataset-backed state type.
    pub const fn state_type(self) -> RotationalStateType {
        self.state_type
    }

    /// The first anchor pair of a 7-state reflection.
    pub const fn anchor_pair_a(self) -> Option<PairIndex16> {
        self.anchor_pair_a
    }

    /// The second anchor pair of a 7-state reflection.
    pub const fn anchor_pair_b(self) -> Option<PairIndex16> {
        self.anchor_pair_b
    }

    /// The court-reflection partner of a dual codon, when the dataset links
    /// one (16 of the 24 dual codons).
    pub const fn paired_codon(self) -> Option<Codon64> {
        self.paired_codon
    }
}

/// `R7(a,b,c, p1a,p1b, p2a,p2b)` — a 7-state non-dual-initiated entry
/// anchored at the two pair orientations (`m3.c:398-399`).
const fn r7(
    _outer: Nucleotide,
    _middle: Nucleotide,
    _inner: Nucleotide,
    p1a: Nucleotide,
    p1b: Nucleotide,
    p2a: Nucleotide,
    p2b: Nucleotide,
) -> RotationalProfile {
    RotationalProfile {
        state_count: POLARIZED_ENTRIES as u8,
        state_type: RotationalStateType::NonDualInitiated,
        anchor_pair_a: Some(PairIndex16::from_nucleotides(p1a, p1b)),
        anchor_pair_b: Some(PairIndex16::from_nucleotides(p2a, p2b)),
        paired_codon: None,
    }
}

/// `R8(a,b,c)` — an 8-state full-rotational entry with no links
/// (`m3.c:400-401`).
const fn r8(_outer: Nucleotide, _middle: Nucleotide, _inner: Nucleotide) -> RotationalProfile {
    RotationalProfile {
        state_count: (POLARIZED_ENTRIES + NONDUAL_ENTRIES) as u8,
        state_type: RotationalStateType::FullRotational,
        anchor_pair_a: None,
        anchor_pair_b: None,
        paired_codon: None,
    }
}

/// `R8P(a,b,c, x,y,z)` — an 8-state full-rotational entry court-linked to
/// the codon `(x,y,z)` (`m3.c:402-403`).
const fn r8p(
    _outer: Nucleotide,
    _middle: Nucleotide,
    _inner: Nucleotide,
    x: Nucleotide,
    y: Nucleotide,
    z: Nucleotide,
) -> RotationalProfile {
    RotationalProfile {
        state_count: (POLARIZED_ENTRIES + NONDUAL_ENTRIES) as u8,
        state_type: RotationalStateType::FullRotational,
        anchor_pair_a: None,
        anchor_pair_b: None,
        paired_codon: Some(Codon64::new((x.bits() << 4) | (y.bits() << 2) | z.bits())),
    }
}

/// The dataset-backed rotational reflection profile of all 64 codons —
/// `M3_ROTATIONAL_PROFILE[64]` (`m3.c:405-425`), transcribed row-for-row in
/// the vendor macro spelling (R7/R8/R8P carry the exact `m3.c:406-424`
/// arguments) and pinned entry-for-entry against the vendor source by
/// `tests/pole_rotational.rs`. Layout: 4 suit rows of 16 in address order,
/// `(outer << 4) | (middle << 2) | inner`, A = 0b00, T = 0b01, C = 0b10,
/// G = 0b11.
pub const ROTATIONAL_PROFILE: [RotationalProfile; 64] = [
    // A-outer (Cups, yin moving) — m3.c vendor row; the C table is designated-initialized
    // (visual order is not memory order), so this layout is address order
    r7(
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
    ), // A A A A A A A -> 0x00
    r7(
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
    ), // A A T A A A T -> 0x01
    r7(
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::C,
    ), // A A C A A A C -> 0x02
    r7(
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::G,
    ), // A A G A A A G -> 0x03
    r7(
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
    ), // A T A A T T A -> 0x04
    r7(
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
    ), // A T T A T T T -> 0x05
    r8p(
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::T,
    ), // A T C A C T -> 0x06
    r8(Nucleotide::A, Nucleotide::T, Nucleotide::G), // A T G -> 0x07
    r7(
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::A,
    ), // A C A A C C A -> 0x08
    r8p(
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::C,
    ), // A C T A T C -> 0x09
    r7(
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
    ), // A C C A C C C -> 0x0A
    r8p(
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::C,
    ), // A C G A G C -> 0x0B
    r7(
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::A,
    ), // A G A A G G A -> 0x0C
    r8(Nucleotide::A, Nucleotide::G, Nucleotide::T), // A G T -> 0x0D
    r8p(
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::G,
    ), // A G C A C G -> 0x0E
    r7(
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
    ), // A G G A G G G -> 0x0F
    // T-outer (Wands, yang moving) — m3.c vendor row; the C table is designated-initialized
    // (visual order is not memory order), so this layout is address order
    r7(
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
    ), // T A A T A A A -> 0x10
    r7(
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::T,
    ), // T A T T A A T -> 0x11
    r8(Nucleotide::T, Nucleotide::A, Nucleotide::C), // T A C -> 0x12
    r8p(
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::A,
    ), // T A G T G A -> 0x13
    r7(
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::A,
    ), // T T A T T T A -> 0x14
    r7(
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
    ), // T T T T T T T -> 0x15
    r7(
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
    ), // T T C T T T C -> 0x16
    r7(
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::G,
    ), // T T G T T T G -> 0x17
    r8(Nucleotide::T, Nucleotide::C, Nucleotide::A), // T C A -> 0x18
    r7(
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::T,
    ), // T C T T C C T -> 0x19
    r7(
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
    ), // T C C T C C C -> 0x1A
    r8p(
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::C,
    ), // T C G T G C -> 0x1B
    r8p(
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::G,
    ), // T G A T A G -> 0x1C
    r7(
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::T,
    ), // T G T T G G T -> 0x1D
    r8p(
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::G,
    ), // T G C T C G -> 0x1E
    r7(
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
    ), // T G G T G G G -> 0x1F
    // C-outer (Pentacles, yin resting) — m3.c vendor row; the C table is designated-initialized
    // (visual order is not memory order), so this layout is address order
    r7(
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
    ), // C A A C A A A -> 0x20
    r8p(
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::A,
    ), // C A T C T A -> 0x21
    r7(
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::C,
    ), // C A C C A A C -> 0x22
    r8p(
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::A,
    ), // C A G C G A -> 0x23
    r8p(
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::T,
    ), // C T A C A T -> 0x24
    r7(
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
    ), // C T T C T T T -> 0x25
    r7(
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::C,
    ), // C T C C T T C -> 0x26
    r8(Nucleotide::C, Nucleotide::T, Nucleotide::G), // C T G -> 0x27
    r7(
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::A,
    ), // C C A C C C A -> 0x28
    r7(
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::T,
    ), // C C T C C C T -> 0x29
    r7(
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
    ), // C C C C C C C -> 0x2A
    r7(
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::G,
    ), // C C G C C C G -> 0x2B
    r8p(
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::C,
        Nucleotide::A,
        Nucleotide::G,
    ), // C G A C A G -> 0x2C
    r8(Nucleotide::C, Nucleotide::G, Nucleotide::T), // C G T -> 0x2D
    r7(
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::C,
    ), // C G C C G G C -> 0x2E
    r7(
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
    ), // C G G C G G G -> 0x2F
    // G-outer (Swords, yang resting) — m3.c vendor row; the C table is designated-initialized
    // (visual order is not memory order), so this layout is address order
    r7(
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::A,
    ), // G A A G A A A -> 0x30
    r8p(
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::A,
    ), // G A T G T A -> 0x31
    r8(Nucleotide::G, Nucleotide::A, Nucleotide::C), // G A C -> 0x32
    r7(
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::A,
        Nucleotide::G,
    ), // G A G G A A G -> 0x33
    r8p(
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::T,
    ), // G T A G A T -> 0x34
    r7(
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::T,
    ), // G T T G T T T -> 0x35
    r8p(
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::T,
    ), // G T C G C T -> 0x36
    r7(
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::T,
        Nucleotide::G,
    ), // G T G G T T G -> 0x37
    r8(Nucleotide::G, Nucleotide::C, Nucleotide::A), // G C A -> 0x38
    r8p(
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::C,
    ), // G C T G T C -> 0x39
    r7(
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::C,
    ), // G C C G C C C -> 0x3A
    r7(
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::C,
        Nucleotide::G,
    ), // G C G G C C G -> 0x3B
    r7(
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::A,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::A,
    ), // G G A G G G A -> 0x3C
    r7(
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::T,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::T,
    ), // G G T G G G T -> 0x3D
    r7(
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::C,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::C,
    ), // G G C G G G C -> 0x3E
    r7(
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
        Nucleotide::G,
    ), // G G G G G G G -> 0x3F
];

/// The rotational reflection profile of a codon — the typed form of
/// `m3_get_rotational_profile` (`m3.c:590-592`), in value form: the profile
/// is a small `Copy` record.
pub const fn rotational_profile(codon: Codon64) -> RotationalProfile {
    ROTATIONAL_PROFILE[codon.address() as usize]
}

/// The composition law `Xy + Za → codon` (`compose_rotational_state`,
/// `m3.h:572-582`). The positive valence keeps the FIRST pair's second
/// nucleotide: `Xy + Za → (X, y, a) = Xya`; the negative valence keeps the
/// SECOND pair's first: `Xy + Za → (X, Z, a) = XZa`.
pub const fn compose_rotational_state(xy: PairIndex16, za: PairIndex16, positive: bool) -> Codon64 {
    let x = xy.first().bits();
    let y = xy.second().bits();
    let z = za.first().bits();
    let a = za.second().bits();
    if positive {
        Codon64::new((x << 4) | (y << 2) | a)
    } else {
        Codon64::new((x << 4) | (z << 2) | a)
    }
}

/// The Watson-Crick anticodon (`wc_anticodon`, `m3.c:601-607`): complement
/// every nucleotide (the XOR-0x01 polarity flip,
/// [`Nucleotide::base_pair`]) and reverse outer/inner.
pub const fn wc_anticodon(codon: Codon64) -> Codon64 {
    Codon64::new(
        (codon.inner().base_pair().bits() << 4)
            | (codon.middle().base_pair().bits() << 2)
            | codon.outer().base_pair().bits(),
    )
}

/// One generated rotational orientation of one codon
/// (`M3_Rotational_Generation`, `m3.h:541-550`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotationalCandidate {
    /// The first pair of the composition (`pair1_idx`).
    pub pair1: PairIndex16,
    /// The second pair of the composition (`pair2_idx`).
    pub pair2: PairIndex16,
    /// The codon the composition lands on (`resulting_codon`).
    pub resulting_codon: Codon64,
    /// The valence of the composition (`polarity`).
    pub polarity: RotationalPolarity,
    /// The final rank in 0..8 (`rotation_slot`).
    pub rotation_slot: u8,
    /// The slot angle: [`ROTATION_SLOT_DEGREES`] × slot (`rotation_degrees`).
    pub rotation_degrees: u16,
    /// The S/D composition value (`rotational_value`).
    pub rotational_value: i16,
    /// The bipolar flag: both composition pairs coincide (`is_non_dual`).
    pub is_non_dual: bool,
}

/// The two-family generation law (`m3_generate_rotational_states`,
/// `m3.c:513-588`) — the full 8-fold candidate sweep of one codon, ranked.
///
/// For codon `XYZ` with `first_pair = encode(X, Y)`,
/// `last_pair = encode(Y, Z)`:
///
/// - NEGATIVE family (sweep `first` over the alphabet):
///   `pair2 = encode(first, Z)`, the candidate codon is
///   `compose(first_pair, pair2, negative) = (X, first, Z)`, the value is
///   `pair1.S + pair2.D`, and the bipolar flag fires when
///   `first_pair == pair2` (i.e. `first == X` and `Y == Z`);
/// - POSITIVE family (sweep `second`): `pair1 = encode(X, second)`, the
///   candidate codon is `compose(pair1, last_pair, positive) = (X, second, Z)`,
///   the value is `pair1.D + pair2.S`, and the flag fires when
///   `pair1 == last_pair` (i.e. `X == Y` and `second == Z`).
///
/// Both families land on `(X, swept, Z)` — the middle site sweeps the whole
/// alphabet, so the sweep spans exactly four distinct codons × two valences.
/// The two families meet at the hinge: at `swept == Y` both emit the input
/// codon itself (the bipolar candidate of a perfect palindrome is emitted
/// identically by both valences).
///
/// Ranking (`m3.c:560-585`, the vendor insertion sort applied verbatim):
/// ASCENDING by rotational value — slot 0 is the LOWEST value; within equal
/// values the negative valence precedes the positive (polarity rank 0 < 1);
/// stable within a (value, valence) class. Slots then carry
/// [`ROTATION_SLOT_DEGREES`] × rank. (This direction contradicts the
/// "descending" prose reading of FR 2.3.14; the C implementation is the
/// authority and the port follows it exactly.)
pub const fn generate_rotational_states(
    codon: Codon64,
) -> [RotationalCandidate; ROTATIONAL_TABLE_ENTRIES] {
    let n1 = codon.outer();
    let n2 = codon.middle();
    let n3 = codon.inner();
    let first_pair = PairIndex16::from_nucleotides(n1, n2);
    let last_pair = PairIndex16::from_nucleotides(n2, n3);

    let mut out = [RotationalCandidate {
        pair1: first_pair,
        pair2: last_pair,
        resulting_codon: codon,
        polarity: RotationalPolarity::Negative,
        rotation_slot: 0,
        rotation_degrees: 0,
        rotational_value: 0,
        is_non_dual: false,
    }; ROTATIONAL_TABLE_ENTRIES];

    // Negative family — m3.c:528-542.
    let mut idx = 0usize;
    while idx < 4 {
        let swept = Nucleotide::ALL[idx];
        let pair2 = PairIndex16::from_nucleotides(swept, n3);
        out[idx] = RotationalCandidate {
            pair1: first_pair,
            pair2,
            resulting_codon: compose_rotational_state(first_pair, pair2, false),
            polarity: RotationalPolarity::Negative,
            rotation_slot: 0,
            rotation_degrees: 0,
            rotational_value: pair_sum(first_pair) + pair_difference(pair2),
            is_non_dual: first_pair.index() == pair2.index(),
        };
        idx += 1;
    }

    // Positive family — m3.c:544-558.
    while idx < ROTATIONAL_TABLE_ENTRIES {
        let swept = Nucleotide::ALL[idx - 4];
        let pair1 = PairIndex16::from_nucleotides(n1, swept);
        out[idx] = RotationalCandidate {
            pair1,
            pair2: last_pair,
            resulting_codon: compose_rotational_state(pair1, last_pair, true),
            polarity: RotationalPolarity::Positive,
            rotation_slot: 0,
            rotation_degrees: 0,
            rotational_value: pair_difference(pair1) + pair_sum(last_pair),
            is_non_dual: pair1.index() == last_pair.index(),
        };
        idx += 1;
    }

    // Vendor ranking — the m3.c:560-585 insertion sort, verbatim: the order
    // array starts as the identity permutation (m3.c:562-564), then the sort
    // shifts the key left while the previous element is strictly greater, or
    // equal with a strictly greater polarity rank. Net law: ascending by
    // value, NEG before POS within equal values, stable within
    // (value, valence).
    let mut order = [0u8; ROTATIONAL_TABLE_ENTRIES];
    let mut seed = 0usize;
    while seed < ROTATIONAL_TABLE_ENTRIES {
        order[seed] = seed as u8;
        seed += 1;
    }
    let mut i = 1usize;
    while i < ROTATIONAL_TABLE_ENTRIES {
        let key = order[i];
        let mut j = i;
        while j > 0 {
            let prev = order[j - 1];
            let before = out[prev as usize].rotational_value < out[key as usize].rotational_value;
            let tie_break = out[prev as usize].rotational_value
                == out[key as usize].rotational_value
                && out[prev as usize].polarity.rank() <= out[key as usize].polarity.rank();
            if before || tie_break {
                break;
            }
            order[j] = prev;
            j -= 1;
        }
        order[j] = key;
        i += 1;
    }

    let mut ranked = out;
    let mut rank = 0usize;
    while rank < ROTATIONAL_TABLE_ENTRIES {
        let mut candidate = out[order[rank] as usize];
        candidate.rotation_slot = rank as u8;
        candidate.rotation_degrees = (rank as u16) * ROTATION_SLOT_DEGREES;
        ranked[rank] = candidate;
        rank += 1;
    }
    ranked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_sd_law_holds_over_the_sixteen_pairs() {
        let mut wc_sums = 0;
        for a in Nucleotide::ALL {
            for b in Nucleotide::ALL {
                let pair = PairIndex16::from_nucleotides(a, b);
                let v1 = a.coin_value().value() as i16;
                let v2 = b.coin_value().value() as i16;
                assert_eq!(pair_sum(pair), v1 + v2, "sum of {a}{b}");
                let magnitude = if a == b { 0 } else { (v1 - v2).abs() };
                assert_eq!(
                    pair_difference(pair).abs(),
                    magnitude,
                    "|difference| of {a}{b}"
                );
                assert_eq!(
                    RECORDED_PAIR_DIFF_SIGNS[pair.index() as usize] as i16,
                    if magnitude == 0 {
                        0
                    } else {
                        pair_difference(pair) / magnitude
                    },
                    "recorded sign of {a}{b}"
                );
                if matches!(
                    (a, b),
                    (Nucleotide::A, Nucleotide::T)
                        | (Nucleotide::T, Nucleotide::A)
                        | (Nucleotide::C, Nucleotide::G)
                        | (Nucleotide::G, Nucleotide::C)
                ) {
                    assert_eq!(pair_sum(pair), 15, "Watson-Crick sum of {a}{b}");
                    wc_sums += 1;
                }
            }
        }
        assert_eq!(wc_sums, 4);
        assert_eq!(
            pair_sum(PairIndex16::from_nucleotides(Nucleotide::A, Nucleotide::A)),
            12
        );
        assert_eq!(
            pair_sum(PairIndex16::from_nucleotides(Nucleotide::T, Nucleotide::T)),
            18
        );
    }

    #[test]
    fn composition_law_pos_keeps_y_neg_keeps_z() {
        for xy in 0u8..16 {
            for za in 0u8..16 {
                let xy = PairIndex16::from_index(xy).expect("xy in range");
                let za = PairIndex16::from_index(za).expect("za in range");
                let positive = compose_rotational_state(xy, za, true);
                assert_eq!(
                    (positive.outer(), positive.middle(), positive.inner()),
                    (xy.first(), xy.second(), za.second()),
                    "Xy + Za -> Xya"
                );
                let negative = compose_rotational_state(xy, za, false);
                assert_eq!(
                    (negative.outer(), negative.middle(), negative.inner()),
                    (xy.first(), za.first(), za.second()),
                    "Xy + Za -> XZa"
                );
            }
        }
    }

    #[test]
    fn anticodon_complements_and_reverses() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let anti = wc_anticodon(codon);
            // Reverse + complement each site.
            assert_eq!(anti.outer(), codon.inner().base_pair());
            assert_eq!(anti.middle(), codon.middle().base_pair());
            assert_eq!(anti.inner(), codon.outer().base_pair());
            // The law is an involution.
            assert_eq!(wc_anticodon(anti), codon, "involution at {codon}");
        }
        assert_eq!(
            wc_anticodon(Codon64::from_nucleotides(
                Nucleotide::A,
                Nucleotide::T,
                Nucleotide::G
            ))
            .to_string(),
            "CAT"
        );
    }

    #[test]
    fn aaa_sweep_ranks_ascending_by_value() {
        let ranked = generate_rotational_states(Codon64::new(0x00));
        let expected: [(&str, bool, i16, &str); 8] = [
            ("pos", false, 9, "ATA"),
            ("pos", false, 10, "ACA"),
            ("neg", true, 12, "AAA"),
            ("pos", true, 12, "AAA"),
            ("neg", false, 13, "AGA"),
            ("pos", false, 13, "AGA"),
            ("neg", false, 14, "ACA"),
            ("neg", false, 15, "ATA"),
        ];
        for (slot, (valence, nd, value, codon)) in expected.iter().enumerate() {
            let candidate = &ranked[slot];
            assert_eq!(
                candidate.polarity,
                if *valence == "neg" {
                    RotationalPolarity::Negative
                } else {
                    RotationalPolarity::Positive
                }
            );
            assert_eq!(candidate.is_non_dual, *nd, "slot {slot} flag");
            assert_eq!(candidate.rotational_value, *value, "slot {slot} value");
            assert_eq!(candidate.resulting_codon.to_string(), *codon);
            assert_eq!(candidate.rotation_slot, slot as u8);
            assert_eq!(candidate.rotation_degrees, (slot * 45) as u16);
        }
        // The perfect-palindrome bipolar state: both valences emit the
        // identical candidate record.
        assert_eq!(ranked[2].pair1, ranked[3].pair1);
        assert_eq!(ranked[2].pair2, ranked[3].pair2);
        assert!(ranked[2].is_non_dual && ranked[3].is_non_dual);
    }

    #[test]
    fn profile_matches_classifier_and_kernel_counts() {
        let mut seven = 0;
        let mut eight = 0;
        let mut anchored = 0;
        let mut paired = 0;
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let profile = rotational_profile(codon);
            assert_eq!(profile.state_count(), codon.rotational_state_count());
            match profile.state_type() {
                RotationalStateType::NonDualInitiated => {
                    seven += 1;
                    anchored += 1;
                    let (a, b) = (
                        profile.anchor_pair_a().expect("anchored"),
                        profile.anchor_pair_b().expect("anchored"),
                    );
                    assert_eq!(a, codon.pair_xy(), "anchor a at {codon}");
                    assert_eq!(b, codon.pair_yz(), "anchor b at {codon}");
                }
                RotationalStateType::FullRotational => {
                    eight += 1;
                    assert!(profile.anchor_pair_a().is_none());
                    assert!(profile.anchor_pair_b().is_none());
                    if profile.paired_codon().is_some() {
                        paired += 1;
                    }
                }
            }
        }
        // The m3_verify counts (m3.c:785-790).
        assert_eq!(seven, 40);
        assert_eq!(eight, 24);
        assert_eq!(anchored, 40);
        assert_eq!(paired, 16);
        let mut states = 0;
        for address in 0u8..64 {
            states += rotational_profile(Codon64::new(address)).state_count() as usize;
        }
        assert_eq!(states, 472);
    }

    #[test]
    fn profile_decoder_validates_the_kernel_laws() {
        let ok = RotationalProfile::try_from_raw(
            7,
            RotationalStateType::NonDualInitiated,
            0,
            5,
            NO_PAIRING,
        )
        .expect("lawful 7-state record");
        assert_eq!(ok.state_count(), 7);
        assert_eq!(ok.anchor_pair_b().map(PairIndex16::index), Some(5));
        assert!(ok.paired_codon().is_none());
        let paired = RotationalProfile::try_from_raw(
            8,
            RotationalStateType::FullRotational,
            NO_PAIR,
            NO_PAIR,
            0x06,
        )
        .expect("lawful 8-state record");
        assert_eq!(paired.paired_codon().map(Codon64::address), Some(0x06));
        for (count, kind, a, b, p, field) in [
            (
                6u8,
                RotationalStateType::NonDualInitiated,
                0u8,
                0u8,
                NO_PAIRING,
                "rotational-state-count",
            ),
            (
                9,
                RotationalStateType::FullRotational,
                NO_PAIR,
                NO_PAIR,
                NO_PAIRING,
                "rotational-state-count",
            ),
            (
                7,
                RotationalStateType::FullRotational,
                NO_PAIR,
                NO_PAIR,
                NO_PAIRING,
                "rotational-state-type",
            ),
            (
                7,
                RotationalStateType::NonDualInitiated,
                NO_PAIR,
                0,
                NO_PAIRING,
                "rotational-anchor-pair",
            ),
            (
                8,
                RotationalStateType::FullRotational,
                0,
                NO_PAIR,
                NO_PAIRING,
                "rotational-anchor-pair",
            ),
            (
                8,
                RotationalStateType::FullRotational,
                NO_PAIR,
                NO_PAIR,
                64,
                "paired-codon",
            ),
        ] {
            let err = RotationalProfile::try_from_raw(count, kind, a, b, p)
                .expect_err("unlawful record must be rejected");
            assert!(
                matches!(err, QlError::InvalidPoleValue { .. }),
                "{field}: {err:?}"
            );
        }
    }
}
