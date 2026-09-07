//! The M3 Tarot exact-cover bridge: 56 Minor Arcana over the 64 codons, 22
//! Major Arcana over the chromosome/amino output registers, and the 22+2
//! boundary governance — the typed, headless port of the C reference
//! kernel's Tarot tables (branch M3-4, "Tarot / Phase / Expressional Field").
//!
//! # Coordinates
//!
//! - Kernel FR 2.3.16 — the canonical LUT contract (`M3_TarotCodonEntry`,
//!   the pip/court constants, the deck counts):
//!   `vendor/epi-kernel/reference/include/m3.h:610-644`; the Major entry
//!   struct `M3_Major_Arcana_Entry` at `m3.h:646-652`.
//! - Kernel FR 2.3.19 — the dataset-backed codon LUT comment:
//!   `vendor/epi-kernel/reference/src/m3.c:250-271`; the codon-encoding
//!   macro `COD(a,b,c)` at `m3.c:298-300`.
//! - Kernel FR 2.3.4 — the suit enum and the kernel-side card register
//!   (`Minor_Arcana_Card`, `SUIT_NAMES`):
//!   `vendor/epi-kernel/reference/include/m3.h:303-334`.
//! - Ported tables (PARSED from the vendored source, never transcribed —
//!   zero transcription, zero drift):
//!   - `M3_TAROT_CODON_MAP[4][16]` at
//!     `vendor/epi-kernel/reference/src/m3.c:302-380` (the suit-integral
//!     comments sit at `m3.c:303-304`, `m3.c:322-323`, `m3.c:341-342`,
//!     `m3.c:360-361`);
//!   - `M3_MAJOR_ARCANA[22]` at
//!     `vendor/epi-kernel/reference/src/m3.c:273-296`.
//! - Suit identity constants — the per-suit integrals and the 360
//!   invariant: `m3.h:596-600` (`M3_SUIT_*_INTEGRAL`,
//!   `M3_INTEGRAL_INVARIANT`), pinned by the conformance suite.
//! - Capability hooks (`docs/origami work/M3/
//!   M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md`): `M3-C19` (Minor
//!   Arcana exact cover — 56 cards, 48 single + 8 dual-codon courts),
//!   `M3-C20` (Major Arcana / governance layer — the +2 semantics
//!   alignment is typed here as [`TranscendentOperator`]), `M3-C21`
//!   (narrative whole transcription — this module provides the typed
//!   surface only; the session trajectory stays open, `M3-C22`).
//! - Branch `M3-4` (matrix row "Tarot / Phase / Expressional Field";
//!   matrix §10 for the 22+2 governance reading).
//!
//! # The exact cover (M3-C19)
//!
//! The 64 codons are covered exactly once by 56 Minor Arcana cards:
//! 48 single-codon cards plus 8 dual-codon court cards carrying two codons
//! each (48 + 8×2 = 64). Every card's codons keep the suit nucleotide in
//! the outer site (FR 2.3.4: suit index = nucleotide two-bit value):
//!
//! ```text
//! Cups       = A family — Water — yin moving    — suit integral 84
//! Wands      = T family — Fire  — yang moving   — suit integral 96
//! Pentacles  = C family — Earth — yin resting   — suit integral 92
//! Swords     = G family — Air   — yang resting  — suit integral 88
//! ```
//!
//! The court gendering law (FR 2.3.16 header, `m3.h:610-614`): the yin
//! suits (Cups, Pentacles) carry their dual codons at Prince(Knight)+King;
//! the yang suits (Wands, Swords) at Princess(Page)+Queen. Every dual pair
//! swaps the middle and inner nucleotides while keeping the outer — the
//! two codons of a court are the two reflections of one relation across
//! the hinge.
//!
//! # Nomenclature duality: one deck, two registers
//!
//! The two junior courts carry Thoth and Rider–Waite–Smith (RWS) names for
//! the same slots: pip 10 is Princess (the `M3_TAROT_PIP_PRINCESS`
//! constant register) and Page (the kernel `TAROT_RANK_NAMES` table,
//! `m3.h:635-639`); pip 11 is Prince and Knight. This is nomenclature
//! only — one deck, two naming registers, no second deck. The kernel
//! itself mixes the registers in its comments (the FR 2.3.16 header spells
//! the yang courts "Page+Queen" and the yin courts "Knight+King"); the
//! typed surface exposes both names and keeps the slot numbers as the law.
//!
//! # Major Arcana and the 22+2 governance (M3-C20)
//!
//! `M3_MAJOR_ARCANA[22]` maps the 22 Major Arcana to chromosome pairs
//! 1-22 and amino-acid output indices 0-21. Beyond the 22-fold autosomal
//! cycle, the matrix §10 reading (ratified; M3-C20 alignment note) types
//! the boundary cards as the 0/1 start-finish relation of an expressional
//! sequence: The Fool = start / opening / 0 and The Universe (Aeon/World
//! register) = stop / completion / 1. These are two additional functional
//! readings of the boundary Major Arcana — NOT two extra physical cards
//! beyond the 78-card deck. [`TranscendentOperator`] carries the reading.
//!
//! Codon-adjacent semantics: the majors carry no codon (`primary_codon`
//! is 0xFF for major/transcendent entries, `m3.h:654-660`), but the kernel
//! still seats them in the 80-entry quaternion layout
//! (`M3_TAROT_QUATERNION_COUNT = 56 + 22 + 2`, `m3.h:643`): the two
//! transcendent operators close the layout at ids 78 and 79 with antipodal
//! rotations on the quaternion z axis (`m3_tarot_rotation`,
//! `m3.h:807-835`) — +z for The Fool (opening) and −z for The Universe
//! (completion). In the ratified basis z carries Air, so the boundary
//! pair acts antipodally through the Air component.
//!
//! # Integral laws
//!
//! The suit integral is the outer-nucleotide pp family charge per
//! quarter: for suit nucleotide v the 16 family codons sum to 16v + 240,
//! giving the integrals 84/96/92/88 (4v + 60) whose total is the 360
//! invariant. The total pp over all 64 codons is 1440 = 4 × 360 — proven
//! independently in `crates/ql-core/tests/pole_coin_contract.rs` (the
//! coin-value ground); this module does not duplicate that computation.
//!
//! The machine-readable deck contract derived 1:1 from the same vendor
//! tables lives at `fixtures/pole/tarot-bridge-v1.tsv` and is cross-pinned
//! against the parsed bridge by the conformance suite
//! (`crates/ql-core/tests/pole_tarot.rs`).

use super::basis::{Element, ElementalQuaternionBasis};
use super::codon::Codon64;
use super::coin::Polarity;
use super::nucleotide::Nucleotide;
use crate::QlError;
use core::fmt;
use std::sync::OnceLock;

/// Semantic identity of the Tarot exact-cover bridge contract.
pub const POLE_TAROT_BRIDGE_REF: &str = "ql.pole.tarot-bridge/v1";

/// Minor Arcana card count (kernel `M3_MINOR_ARCANA_COUNT`, `m3.h:640`).
pub const MINOR_ARCANA_COUNT: usize = 56;

/// Major Arcana card count (kernel `M3_MAJOR_ARCANA_COUNT`, `m3.h:641`).
pub const MAJOR_ARCANA_COUNT: usize = 22;

/// Transcendent operator count — the +2 of the 22+2 governance law
/// (kernel `M3_TRANSCENDENT_TAROT_COUNT`, `m3.h:642`).
pub const TRANSCENDENT_TAROT_COUNT: usize = 2;

/// Full typed deck in the kernel quaternion layout: minors + majors + the
/// two boundary governance readings (kernel `M3_TAROT_QUATERNION_COUNT`,
/// `m3.h:643`).
pub const TAROT_QUATERNION_COUNT: usize =
    MINOR_ARCANA_COUNT + MAJOR_ARCANA_COUNT + TRANSCENDENT_TAROT_COUNT;

/// LUT slots per suit in the vendor table — 14 cards + 2 padding entries
/// (kernel `M3_TAROT_ENTRIES_PER_SUIT`, `m3.h:630`).
pub const ENTRIES_PER_SUIT: usize = 16;

/// The four Tarot suits as the four nucleotide/element families (FR 2.3.4,
/// `m3.h:311-317`): the suit index IS the nucleotide two-bit value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TarotSuit {
    /// Cups — A family, yin moving, Water.
    Cups = 0,
    /// Wands — T family, yang moving, Fire.
    Wands = 1,
    /// Pentacles — C family, yin resting, Earth.
    Pentacles = 2,
    /// Swords — G family, yang resting, Air.
    Swords = 3,
}

impl TarotSuit {
    /// The four suits in kernel index order.
    pub const ALL: [TarotSuit; 4] = [Self::Cups, Self::Wands, Self::Pentacles, Self::Swords];

    /// Raw two-bit suit index (== the carrying nucleotide's bits).
    pub const fn bits(self) -> u8 {
        self as u8
    }

    /// Suit from the kernel two-bit suit index.
    pub const fn from_bits(bits: u8) -> Result<Self, QlError> {
        match bits {
            0 => Ok(Self::Cups),
            1 => Ok(Self::Wands),
            2 => Ok(Self::Pentacles),
            3 => Ok(Self::Swords),
            other => Err(QlError::InvalidPoleValue {
                field: "tarot-suit",
                value: other as u32,
            }),
        }
    }

    /// The suit nucleotide — FR 2.3.4: suit index = nucleotide two-bit
    /// value, so Cups carries A, Wands T, Pentacles C, Swords G.
    pub const fn nucleotide(self) -> Nucleotide {
        Nucleotide::ALL[self as usize]
    }

    /// The suit carrying a nucleotide — the inverse of [`Self::nucleotide`].
    pub const fn from_nucleotide(nucleotide: Nucleotide) -> Self {
        Self::ALL[nucleotide.bits() as usize]
    }

    /// The suit element through the ratified elemental basis.
    pub const fn element(self) -> Element {
        ElementalQuaternionBasis::canonical().element_of(self.nucleotide())
    }

    /// Suit gender: Cups and Pentacles are yin, Wands and Swords yang —
    /// the polarity of the carrying nucleotide (FR 2.3.4 suit comments).
    pub const fn is_yin(self) -> bool {
        matches!(self.nucleotide().polarity(), Polarity::Yin)
    }

    /// The dual-codon court slots of this suit — the court gendering law
    /// (FR 2.3.16 header): yin suits dual at Prince(Knight)+King, yang
    /// suits dual at Princess(Page)+Queen.
    pub const fn dual_court_pips(self) -> [TarotPip; 2] {
        match self {
            Self::Cups | Self::Pentacles => [TarotPip::PRINCE, TarotPip::KING],
            Self::Wands | Self::Swords => [TarotPip::PRINCESS, TarotPip::QUEEN],
        }
    }

    /// The suit integral — the outer-nucleotide pp family charge per
    /// quarter. For suit value v the 16 family codons sum to 16v + 240 pp,
    /// so the integral is 4v + 60: Cups 84, Wands 96, Pentacles 92,
    /// Swords 88 (`m3.h:597-600`); total 360. Computed from the canonical
    /// coin table ([`Nucleotide::coin_value`]), never stored — the kernel
    /// constants are pinned against this by the conformance suite.
    pub const fn integral(self) -> u16 {
        let own = self.nucleotide().coin_value().value() as u16;
        let mut sum = 0u16;
        let mut middle = 0;
        while middle < 4 {
            let middle_value = Nucleotide::ALL[middle].coin_value().value() as u16;
            let mut inner = 0;
            while inner < 4 {
                let inner_value = Nucleotide::ALL[inner].coin_value().value() as u16;
                sum += own + middle_value + inner_value;
                inner += 1;
            }
            middle += 1;
        }
        sum / 4
    }

    /// Kernel suit name (`SUIT_NAMES`, `m3.h:322`).
    pub const fn name(self) -> &'static str {
        match self {
            Self::Cups => "Cups",
            Self::Wands => "Wands",
            Self::Pentacles => "Pentacles",
            Self::Swords => "Swords",
        }
    }
}

impl fmt::Display for TarotSuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The pip/court register of a Minor Arcana card (FR 2.3.16,
/// `m3.h:624-628`): 0 = Ace, 1-9 = Two..Ten, 10 = Princess(Page),
/// 11 = Prince(Knight), 12 = Queen, 13 = King.
///
/// The two junior courts carry the Thoth names (Princess/Prince) in the
/// `M3_TAROT_PIP_*` constant register and the RWS names (Page/Knight) in
/// the kernel `TAROT_RANK_NAMES` register — nomenclature only; the slot
/// numbers are the law (see the module header).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TarotPip(u8);

impl TarotPip {
    pub const ACE: Self = Self(0);
    pub const TWO: Self = Self(1);
    pub const THREE: Self = Self(2);
    pub const FOUR: Self = Self(3);
    pub const FIVE: Self = Self(4);
    pub const SIX: Self = Self(5);
    pub const SEVEN: Self = Self(6);
    pub const EIGHT: Self = Self(7);
    pub const NINE: Self = Self(8);
    pub const TEN: Self = Self(9);
    /// Thoth register of pip 10; RWS register: Page.
    pub const PRINCESS: Self = Self(10);
    /// Thoth register of pip 11; RWS register: Knight.
    pub const PRINCE: Self = Self(11);
    pub const QUEEN: Self = Self(12);
    pub const KING: Self = Self(13);

    /// The fourteen pips in register order.
    pub const ALL: [TarotPip; 14] = [
        Self::ACE,
        Self::TWO,
        Self::THREE,
        Self::FOUR,
        Self::FIVE,
        Self::SIX,
        Self::SEVEN,
        Self::EIGHT,
        Self::NINE,
        Self::TEN,
        Self::PRINCESS,
        Self::PRINCE,
        Self::QUEEN,
        Self::KING,
    ];

    /// The raw pip slot number (0-13).
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Pip from the raw slot number.
    pub const fn new(value: u8) -> Result<Self, QlError> {
        if value < 14 {
            Ok(Self(value))
        } else {
            Err(QlError::InvalidPoleValue {
                field: "tarot-pip",
                value: value as u32,
            })
        }
    }

    /// Court cards are pips 10-13 (Princess/Page, Prince/Knight, Queen,
    /// King); pips 0-9 are the number cards.
    pub const fn is_court(self) -> bool {
        self.0 >= 10
    }

    /// The Thoth register name — the register of the `M3_TAROT_PIP_*`
    /// constants the vendor LUT is written in.
    pub const fn thoth_name(self) -> &'static str {
        match self.0 {
            0 => "Ace",
            1 => "Two",
            2 => "Three",
            3 => "Four",
            4 => "Five",
            5 => "Six",
            6 => "Seven",
            7 => "Eight",
            8 => "Nine",
            9 => "Ten",
            10 => "Princess",
            11 => "Prince",
            12 => "Queen",
            _ => "King",
        }
    }

    /// The RWS register name — the register of the kernel rank-name table
    /// (`TAROT_RANK_NAMES`, `m3.h:635-639`).
    pub const fn rws_name(self) -> &'static str {
        match self.0 {
            0 => "Ace",
            1 => "Two",
            2 => "Three",
            3 => "Four",
            4 => "Five",
            5 => "Six",
            6 => "Seven",
            7 => "Eight",
            8 => "Nine",
            9 => "Ten",
            10 => "Page",
            11 => "Knight",
            12 => "Queen",
            _ => "King",
        }
    }
}

/// One Minor Arcana card of the exact cover: its suit, its pip slot, and
/// one or two codons (FR 2.3.16 `M3_TarotCodonEntry`, `m3.h:618-623`).
///
/// The 48 single-codon cards carry one codon; the 8 dual-codon courts
/// carry the pair that swaps middle/inner nucleotides across the hinge
/// while keeping the suit nucleotide in the outer site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinorArcanaCard {
    suit: TarotSuit,
    pip: TarotPip,
    codon_a: Codon64,
    codon_b: Option<Codon64>,
}

impl MinorArcanaCard {
    /// The card's suit.
    pub const fn suit(&self) -> TarotSuit {
        self.suit
    }

    /// The card's pip slot.
    pub const fn pip(&self) -> TarotPip {
        self.pip
    }

    /// The primary codon (kernel `codon_a`).
    pub const fn codon_a(&self) -> Codon64 {
        self.codon_a
    }

    /// The secondary codon of a dual-codon court, if any (kernel
    /// `codon_b`; 0xFF = none in the vendor register).
    pub const fn codon_b(&self) -> Option<Codon64> {
        self.codon_b
    }

    /// True for the 8 dual-codon courts.
    pub const fn is_dual_court(&self) -> bool {
        self.codon_b.is_some()
    }

    /// The card's codons in kernel order: primary, then secondary.
    pub fn codons(&self) -> impl Iterator<Item = Codon64> {
        core::iter::once(self.codon_a).chain(self.codon_b)
    }

    /// The kernel deck id: suit × 14 + pip (the addressing of
    /// `m3_tarot_rotation`, `m3.h:807-810`; ids 0-55).
    pub const fn card_id(&self) -> u8 {
        self.suit.bits() * 14 + self.pip.value()
    }

    /// The card's rank name in the kernel print register (RWS).
    pub const fn rank_name(&self) -> &'static str {
        self.pip.rws_name()
    }
}

impl fmt::Display for MinorArcanaCard {
    /// The kernel print register: "<Rank> of <Suit>" (`m3.c:1006-1007`).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.pip.rws_name(), self.suit.name())
    }
}

/// One Major Arcana card: name, chromosome pair, amino-acid output index
/// (`M3_Major_Arcana_Entry`, `m3.h:646-652`; table `m3.c:273-296`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MajorArcanaCard {
    card_id: u8,
    name: &'static str,
    chromosome_pair: u8,
    amino_acid_index: u8,
}

impl MajorArcanaCard {
    /// The card id within the majors (0-21, == the table row index).
    pub const fn card_id(&self) -> u8 {
        self.card_id
    }

    /// The card name as recorded in the vendor table.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// The mapped autosomal chromosome pair (1-22).
    pub const fn chromosome_pair(&self) -> u8 {
        self.chromosome_pair
    }

    /// The mapped amino-acid output index (0-21).
    pub const fn amino_acid_index(&self) -> u8 {
        self.amino_acid_index
    }
}

/// The 22+2 boundary governance law (matrix §10; M3-C20 alignment):
/// two additional functional readings of the boundary Major Arcana — NOT
/// two extra physical cards beyond the 78-card deck.
///
/// ```text
/// The Fool     ↔ start / opening / 0
/// The Universe ↔ stop / completion / 1
/// ```
///
/// Codon-adjacent semantics: the majors carry no codon (0xFF register,
/// `m3.h:654-660`), but the two operators close the kernel's 80-entry
/// quaternion layout at ids 78/79 with antipodal z-axis rotations
/// (`m3_tarot_rotation`, `m3.h:807-835`): The Fool at +z (opening), The
/// Universe at −z (completion) — the 0/1 start-finish relation of an
/// expressional sequence (kernel anchor `# / 0/1 <-> 1/0`), acting
/// antipodally through the Air component of the ratified basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranscendentOperator {
    /// The Fool — start / opening / 0 (reads `M3_MAJOR_ARCANA[0]`).
    Fool,
    /// The Universe (Aeon/World register) — stop / completion / 1
    /// (reads `M3_MAJOR_ARCANA[21]`).
    Universe,
}

impl TranscendentOperator {
    /// The two operators in boundary order (0 then 1).
    pub const ALL: [TranscendentOperator; 2] = [Self::Fool, Self::Universe];

    /// The boundary Major Arcana row this operator reads.
    pub const fn major_index(self) -> u8 {
        match self {
            Self::Fool => 0,
            Self::Universe => MAJOR_ARCANA_COUNT as u8 - 1,
        }
    }

    /// The boundary value in the 0/1 start-finish relation.
    pub const fn boundary_value(self) -> u8 {
        match self {
            Self::Fool => 0,
            Self::Universe => 1,
        }
    }

    /// The governance role, in the matrix §10 register.
    pub const fn role(self) -> &'static str {
        match self {
            Self::Fool => "start/opening",
            Self::Universe => "stop/completion",
        }
    }

    /// The quaternion entry id: the majors end at 77; the operators close
    /// the layout at 78/79 (`m3_tarot_rotation`, `m3.h:832-834`).
    pub const fn quaternion_id(self) -> u8 {
        (MINOR_ARCANA_COUNT + MAJOR_ARCANA_COUNT) as u8 + self.boundary_value()
    }
}

/// The typed, parsed Tarot bridge: the 56-card Minor exact cover, the 22
/// Major Arcana, and the O(1) codon → card index of the cover.
///
/// Obtain the kernel instance through [`TarotBridge::kernel`]; the tables
/// are parsed from the vendored C source (zero transcription).
#[derive(Debug, Clone)]
pub struct TarotBridge {
    minor: [MinorArcanaCard; MINOR_ARCANA_COUNT],
    major: [MajorArcanaCard; MAJOR_ARCANA_COUNT],
    codon_index: [u8; Codon64::COUNT],
}

impl TarotBridge {
    /// The parsed kernel bridge, built once from the vendored C source.
    ///
    /// Panics only if the vendored kernel source were malformed — it is a
    /// frozen, content-pinned file and the conformance suite
    /// (`crates/ql-core/tests/pole_tarot.rs`) pins its well-formedness.
    pub fn kernel() -> &'static Self {
        static KERNEL: OnceLock<TarotBridge> = OnceLock::new();
        KERNEL.get_or_init(|| {
            parse_kernel_source(include_str!(
                "../../../../vendor/epi-kernel/reference/src/m3.c"
            ))
            .expect("vendored M3 kernel source is well-formed")
        })
    }

    /// The 56 Minor Arcana cards in deck order: suit-major, pip-ascending,
    /// so `minor()[i].card_id() == i`.
    pub const fn minor(&self) -> &[MinorArcanaCard; MINOR_ARCANA_COUNT] {
        &self.minor
    }

    /// The 22 Major Arcana cards in kernel table order.
    pub const fn major(&self) -> &[MajorArcanaCard; MAJOR_ARCANA_COUNT] {
        &self.major
    }

    /// The 14 cards of one suit in pip order.
    pub fn suit_cards(&self, suit: TarotSuit) -> &[MinorArcanaCard] {
        let base = suit.bits() as usize * 14;
        &self.minor[base..base + 14]
    }

    /// Exact-cover lookup: the one Minor Arcana card carrying this codon.
    /// The majors carry no codon, so the cover is total over `Codon64`.
    pub fn card_of_codon(&self, codon: Codon64) -> Option<&MinorArcanaCard> {
        let row = self.codon_index[codon.address() as usize];
        // 0xFF is the kernel "single-codon/none" register (`m3.h:629`)
        // reused here as the "uncovered" sentinel — the cover itself is
        // total, so the sentinel never survives parsing.
        if row == u8::MAX {
            None
        } else {
            self.minor.get(row as usize)
        }
    }
}

/// Parse the Tarot bridge out of the vendored C kernel source.
///
/// Pure and re-runnable: the kernel instance is the parsed vendor source
/// itself, so Rust and C cannot drift. Malformed input is rejected with
/// [`QlError::InvalidPoleValue`], the field naming the failing coordinate.
pub fn parse_kernel_source(source: &'static str) -> Result<TarotBridge, QlError> {
    let minor = parse_codon_map(source)?;
    let major = parse_major_arcana(source)?;

    // Build the codon → deck-row index while enforcing the exact cover:
    // every codon at most once across all cards (dual courts contribute
    // both codons).
    let mut codon_index = [0xFFu8; Codon64::COUNT];
    for (row, card) in minor.iter().enumerate() {
        for codon in card.codons() {
            let slot = &mut codon_index[codon.address() as usize];
            if *slot != 0xFF {
                return Err(QlError::InvalidPoleValue {
                    field: "tarot-codon-duplicate",
                    value: codon.address() as u32,
                });
            }
            *slot = row as u8;
        }
    }

    Ok(TarotBridge {
        minor,
        major,
        codon_index,
    })
}

/// Parse `M3_TAROT_CODON_MAP[4][16]` (`m3.c:302-380`) into the 56-card
/// deck. The 32 padding entries (two per suit, codon-less) are skipped;
/// exactly 14 real entries per suit must remain.
fn parse_codon_map(source: &'static str) -> Result<[MinorArcanaCard; MINOR_ARCANA_COUNT], QlError> {
    const ANCHOR: &str = "const M3_TarotCodonEntry M3_TAROT_CODON_MAP[4][16] = {";
    let body = table_body(source, ANCHOR)?;

    let mut slots: [Option<MinorArcanaCard>; MINOR_ARCANA_COUNT] = std::array::from_fn(|_| None);
    let mut found = 0usize;

    for line in body.lines() {
        let trimmed = line.trim();
        // Real entries carry a COD(..) codon; comments, brace rows and the
        // codon-less padding pairs do not.
        if !trimmed.starts_with('{') || !trimmed.contains("COD(") {
            continue;
        }
        let close = trimmed.find('}').ok_or(invalid("tarot-codon-entry", 0))?;
        let fields = split_top_commas(&trimmed[1..close]);
        if fields.len() != 4 {
            return Err(invalid("tarot-codon-entry", fields.len() as u32));
        }

        let suit_bits: u8 = fields[0]
            .trim()
            .parse()
            .map_err(|_| invalid("tarot-codon-entry-suit", 0))?;
        let suit = TarotSuit::from_bits(suit_bits)?;
        let pip = parse_pip_field(fields[1])?;
        let codon_a =
            parse_codon_field(fields[2])?.ok_or(invalid("tarot-codon-entry-primary", 0))?;
        let codon_b = parse_codon_field(fields[3])?;

        let index = suit.bits() as usize * 14 + pip.value() as usize;
        if slots[index].is_some() {
            return Err(QlError::InvalidPoleValue {
                field: "tarot-card-duplicate",
                value: index as u32,
            });
        }
        slots[index] = Some(MinorArcanaCard {
            suit,
            pip,
            codon_a,
            codon_b,
        });
        found += 1;
    }

    if found != MINOR_ARCANA_COUNT {
        return Err(invalid("tarot-codon-entry-count", found as u32));
    }
    Ok(std::array::from_fn(|index| {
        slots[index].take().expect("all slots validated filled")
    }))
}

/// Parse `M3_MAJOR_ARCANA[M3_MAJOR_ARCANA_COUNT]` (`m3.c:273-296`) into
/// the 22 Major Arcana entries.
fn parse_major_arcana(
    source: &'static str,
) -> Result<[MajorArcanaCard; MAJOR_ARCANA_COUNT], QlError> {
    const ANCHOR: &str = "const M3_Major_Arcana_Entry M3_MAJOR_ARCANA[M3_MAJOR_ARCANA_COUNT] = {";
    let body = table_body(source, ANCHOR)?;

    let mut slots: [Option<MajorArcanaCard>; MAJOR_ARCANA_COUNT] = std::array::from_fn(|_| None);
    let mut found = 0usize;

    for line in body.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('{') {
            continue;
        }
        let close = trimmed.find('}').ok_or(invalid("tarot-major-entry", 0))?;
        let fields = split_top_commas(&trimmed[1..close]);
        if fields.len() != 4 {
            return Err(invalid("tarot-major-entry", fields.len() as u32));
        }

        let card_id: u8 = fields[0]
            .trim()
            .parse()
            .map_err(|_| invalid("tarot-major-entry-id", 0))?;
        if card_id as usize >= MAJOR_ARCANA_COUNT || slots[card_id as usize].is_some() {
            return Err(QlError::InvalidPoleValue {
                field: "tarot-major-entry-id",
                value: card_id as u32,
            });
        }
        let name = strip_quotes(fields[1].trim()).ok_or(invalid("tarot-major-entry-name", 0))?;
        let chromosome_pair: u8 = fields[2]
            .trim()
            .parse()
            .map_err(|_| invalid("tarot-major-entry-chromosome", 0))?;
        if !(1..=22).contains(&chromosome_pair) {
            return Err(QlError::InvalidPoleValue {
                field: "tarot-major-entry-chromosome",
                value: chromosome_pair as u32,
            });
        }
        let amino_acid_index: u8 = fields[3]
            .trim()
            .parse()
            .map_err(|_| invalid("tarot-major-entry-amino", 0))?;
        if amino_acid_index > 21 {
            return Err(QlError::InvalidPoleValue {
                field: "tarot-major-entry-amino",
                value: amino_acid_index as u32,
            });
        }

        slots[card_id as usize] = Some(MajorArcanaCard {
            card_id,
            name,
            chromosome_pair,
            amino_acid_index,
        });
        found += 1;
    }

    if found != MAJOR_ARCANA_COUNT {
        return Err(invalid("tarot-major-entry-count", found as u32));
    }
    Ok(std::array::from_fn(|index| {
        slots[index].take().expect("all slots validated filled")
    }))
}

/// Slice out a C initializer body between an anchor and its closing `};`.
fn table_body<'a>(source: &'a str, anchor: &str) -> Result<&'a str, QlError> {
    let start = source
        .find(anchor)
        .ok_or(invalid("tarot-table-anchor", 0))?
        + anchor.len();
    let end = source[start..]
        .find("};")
        .ok_or(invalid("tarot-table-close", 0))?;
    Ok(&source[start..start + end])
}

/// Split on top-level commas — commas not inside `COD(..)` parentheses.
fn split_top_commas(inner: &str) -> Vec<&str> {
    let mut fields = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (index, character) in inner.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                fields.push(inner[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    let last = inner[start..].trim();
    if !last.is_empty() {
        fields.push(last);
    }
    fields
}

/// Parse the pip field: a kernel `M3_TAROT_PIP_*` constant or a decimal
/// slot number (the LUT uses both registers).
fn parse_pip_field(field: &str) -> Result<TarotPip, QlError> {
    let field = field.trim();
    let value = match field {
        "M3_TAROT_PIP_ACE" => TarotPip::ACE,
        "M3_TAROT_PIP_PRINCESS" => TarotPip::PRINCESS,
        "M3_TAROT_PIP_PRINCE" => TarotPip::PRINCE,
        "M3_TAROT_PIP_QUEEN" => TarotPip::QUEEN,
        "M3_TAROT_PIP_KING" => TarotPip::KING,
        decimal => TarotPip::new(
            decimal
                .parse()
                .map_err(|_| invalid("tarot-pip", first_byte(decimal)))?,
        )?,
    };
    Ok(value)
}

/// Parse a codon field: `COD(X,Y,Z)` (the kernel macro at `m3.c:298`,
/// `(outer << 4) | (middle << 2) | inner` over `M3_NUC_*` bits — exactly
/// the [`Codon64`] address layout) or `NONE` (the 0xFF register).
fn parse_codon_field(field: &str) -> Result<Option<Codon64>, QlError> {
    let field = field.trim();
    if field == "NONE" {
        return Ok(None);
    }
    let inner = field
        .strip_prefix("COD(")
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or(invalid("tarot-codon-field", first_byte(field)))?;
    let mut nucleotides = [Nucleotide::A; 3];
    for (slot, symbol) in inner.split(',').enumerate() {
        if slot >= 3 {
            return Err(invalid("tarot-codon-field", first_byte(field)));
        }
        nucleotides[slot] = nucleotide_from_symbol(symbol.trim())?;
    }
    Ok(Some(Codon64::from_nucleotides(
        nucleotides[0],
        nucleotides[1],
        nucleotides[2],
    )))
}

/// The `M3_NUC_*` two-bit values (FR 2.3.16, `m3.h:84-87`): A=0, T=1,
/// C=2, G=3 — the canonical [`Nucleotide`] bits.
fn nucleotide_from_symbol(symbol: &str) -> Result<Nucleotide, QlError> {
    match symbol {
        "A" => Ok(Nucleotide::A),
        "T" => Ok(Nucleotide::T),
        "C" => Ok(Nucleotide::C),
        "G" => Ok(Nucleotide::G),
        other => Err(QlError::InvalidPoleValue {
            field: "tarot-codon-symbol",
            value: first_byte(other),
        }),
    }
}

fn strip_quotes(field: &str) -> Option<&str> {
    field
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
}

const fn invalid(field: &'static str, value: u32) -> QlError {
    QlError::InvalidPoleValue { field, value }
}

const fn first_byte(field: &str) -> u32 {
    match field.as_bytes().first() {
        Some(&byte) => byte as u32,
        None => 0,
    }
}
