//! Conformance for `ql.pole.tarot-bridge/v1` — the Tarot exact-cover bridge
//! (branch M3-4; capability hooks M3-C19, M3-C20, M3-C21).
//!
//! The vendor C source is the data authority: `pole::tarot` parses
//! `M3_TAROT_CODON_MAP[4][16]` (`vendor/epi-kernel/reference/src/m3.c:302-380`)
//! and `M3_MAJOR_ARCANA[22]` (`m3.c:273-296`) directly, so this suite pins
//! the laws — exact cover, court gendering, hinge swap, outer law, suit
//! integrals, 22+2 governance — against the same bytes the kernel ships,
//! together with the FR 2.3.16 constants of
//! `vendor/epi-kernel/reference/include/m3.h:596-652` and the
//! machine-readable fixture `fixtures/pole/tarot-bridge-v1.tsv`.
//!
//! Compilation note: the module is included here via `#[path]` with
//! sibling-module shims over `ql_core`'s public surface, so the suite runs
//! before AND after the integrating agent wires `pole::tarot` into the
//! crate and re-exports its symbols — no test change needed either way.

// Sibling-module shims over ql_core's public surface: `pole::tarot` refers
// to its pole siblings through `super::`, so the `#[path]` include below
// needs those names at the test-crate root.
mod basis {
    pub use ql_core::{Element, ElementalQuaternionBasis};
}
mod codon {
    pub use ql_core::Codon64;
}
mod coin {
    pub use ql_core::Polarity;
}
mod nucleotide {
    pub use ql_core::Nucleotide;
}

use ql_core::QlError;

#[path = "../src/pole/tarot.rs"]
mod tarot;

use ql_core::{Codon64, Element, ElementalQuaternionBasis, Nucleotide};
use tarot::{
    ENTRIES_PER_SUIT, MAJOR_ARCANA_COUNT, MINOR_ARCANA_COUNT, POLE_TAROT_BRIDGE_REF,
    TAROT_QUATERNION_COUNT, TRANSCENDENT_TAROT_COUNT, TarotBridge, TarotPip, TarotSuit,
    TranscendentOperator, parse_kernel_source,
};

const VENDOR_M3_C: &str = include_str!("../../../vendor/epi-kernel/reference/src/m3.c");
const VENDOR_M3_H: &str = include_str!("../../../vendor/epi-kernel/reference/include/m3.h");
const FIXTURE: &str = include_str!("../../../fixtures/pole/tarot-bridge-v1.tsv");

/// The value of a simple `#define NAME <number>[Uu]` line of m3.h —
/// decimal or 0x-hex. The name must match whole-word, so e.g.
/// `M3_TAROT_PIP_PRINCE` does not match the `M3_TAROT_PIP_PRINCESS` line.
fn define_value(name: &str) -> u32 {
    let line = VENDOR_M3_H
        .lines()
        .find(|line| {
            let mut words = line.split_whitespace();
            words.next() == Some("#define") && words.next() == Some(name)
        })
        .unwrap_or_else(|| panic!("m3.h is missing {name}"));
    let token = line
        .split_whitespace()
        .nth(2)
        .unwrap_or_else(|| panic!("m3.h define {name} carries no value"));
    let number = token.trim_end_matches(['U', 'u']);
    if let Some(hex) = number.strip_prefix("0x") {
        u32::from_str_radix(hex, 16)
    } else {
        number.parse()
    }
    .unwrap_or_else(|_| panic!("m3.h define {name} is not numeric: {token}"))
}

/// The quoted strings of a C `static const char* const NAME[..] = {...}`
/// initializer in m3.h, located by its declaration anchor.
fn header_string_array(declaration: &str) -> Vec<String> {
    let at = VENDOR_M3_H
        .find(declaration)
        .unwrap_or_else(|| panic!("m3.h is missing {declaration}"));
    let open = at + VENDOR_M3_H[at..].find("= {").expect("initializer open") + 3;
    let end = open + VENDOR_M3_H[open..].find("};").expect("array close");
    VENDOR_M3_H[open..end]
        .split(',')
        .filter_map(|piece| {
            let trimmed = piece.trim();
            trimmed
                .strip_prefix('"')
                .and_then(|rest| rest.strip_suffix('"'))
                .map(str::to_string)
        })
        .collect()
}

#[test]
fn contract_identity_and_deck_counts() {
    assert_eq!(POLE_TAROT_BRIDGE_REF, "ql.pole.tarot-bridge/v1");
    assert_eq!(MINOR_ARCANA_COUNT, 56);
    assert_eq!(MAJOR_ARCANA_COUNT, 22);
    assert_eq!(TRANSCENDENT_TAROT_COUNT, 2);
    assert_eq!(TAROT_QUATERNION_COUNT, 80);
    assert_eq!(
        TAROT_QUATERNION_COUNT,
        MINOR_ARCANA_COUNT + MAJOR_ARCANA_COUNT + TRANSCENDENT_TAROT_COUNT
    );

    let bridge = TarotBridge::kernel();
    assert_eq!(bridge.minor().len(), MINOR_ARCANA_COUNT);
    assert_eq!(bridge.major().len(), MAJOR_ARCANA_COUNT);
    assert_eq!(ENTRIES_PER_SUIT, 16, "14 cards + 2 padding LUT slots");
}

/// The FR 2.3.16 constants of m3.h must carry the same numbers as the typed
/// surface: one law, two languages, zero drift. This also pins the `COD`
/// macro's `M3_NUC_*` bit values (`m3.h:84-87`) to [`Nucleotide`], which is
/// what makes the parsed `COD(a,b,c)` triples equal [`Codon64`] addresses.
#[test]
fn m3_h_constants_pin_the_typed_surface() {
    // The nucleotide two-bit alphabet of the codon encoding.
    for (symbol, bits) in [("A", 0u32), ("T", 1), ("C", 2), ("G", 3)] {
        assert_eq!(
            define_value(&format!("M3_NUC_{symbol}")),
            bits,
            "M3_NUC_{symbol}"
        );
        assert_eq!(
            Nucleotide::from_bits_checked(bits as u8)
                .expect("alphabet bits")
                .symbol()
                .to_string(),
            symbol
        );
    }

    // The pip/court register (m3.h:624-628).
    for (name, pip) in [
        ("M3_TAROT_PIP_ACE", TarotPip::ACE),
        ("M3_TAROT_PIP_PRINCESS", TarotPip::PRINCESS),
        ("M3_TAROT_PIP_PRINCE", TarotPip::PRINCE),
        ("M3_TAROT_PIP_QUEEN", TarotPip::QUEEN),
        ("M3_TAROT_PIP_KING", TarotPip::KING),
    ] {
        assert_eq!(define_value(name), pip.value() as u32, "{name}");
    }

    // The deck counts (m3.h:630-643).
    assert_eq!(
        define_value("M3_TAROT_ENTRIES_PER_SUIT"),
        ENTRIES_PER_SUIT as u32
    );
    assert_eq!(
        define_value("M3_MINOR_ARCANA_COUNT"),
        MINOR_ARCANA_COUNT as u32
    );
    assert_eq!(
        define_value("M3_MAJOR_ARCANA_COUNT"),
        MAJOR_ARCANA_COUNT as u32
    );
    assert_eq!(
        define_value("M3_TRANSCENDENT_TAROT_COUNT"),
        TRANSCENDENT_TAROT_COUNT as u32
    );
    assert_eq!(
        define_value("M3_TAROT_QUATERNION_COUNT"),
        TAROT_QUATERNION_COUNT as u32
    );

    // The suit integrals and the 360 invariant (m3.h:596-600).
    assert_eq!(define_value("M3_INTEGRAL_INVARIANT"), 360);
    for (name, suit) in [
        ("M3_SUIT_A_INTEGRAL", TarotSuit::Cups),
        ("M3_SUIT_T_INTEGRAL", TarotSuit::Wands),
        ("M3_SUIT_C_INTEGRAL", TarotSuit::Pentacles),
        ("M3_SUIT_G_INTEGRAL", TarotSuit::Swords),
    ] {
        assert_eq!(define_value(name), suit.integral() as u32, "{name}");
    }
}

#[test]
fn suits_identify_through_the_ratified_basis() {
    let basis = ElementalQuaternionBasis::canonical();
    for suit in TarotSuit::ALL {
        // Suit index IS the nucleotide two-bit value (FR 2.3.4).
        assert_eq!(suit.bits(), suit.nucleotide().bits());
        assert_eq!(TarotSuit::from_nucleotide(suit.nucleotide()), suit);
        assert_eq!(TarotSuit::from_bits(suit.bits()).expect("suit bits"), suit);
        // Element relation through the sealed basis.
        assert_eq!(suit.element(), basis.element_of(suit.nucleotide()));
        // Gender = nucleotide polarity.
        assert_eq!(
            suit.nucleotide().polarity() == ql_core::Polarity::Yin,
            suit.is_yin()
        );
    }
    assert_eq!(TarotSuit::Cups.element(), Element::Water);
    assert_eq!(TarotSuit::Wands.element(), Element::Fire);
    assert_eq!(TarotSuit::Pentacles.element(), Element::Earth);
    assert_eq!(TarotSuit::Swords.element(), Element::Air);
    assert_eq!(TarotSuit::Cups.to_string(), "Cups");
    assert!(TarotSuit::Cups.is_yin());
    assert!(TarotSuit::Pentacles.is_yin());
    assert!(!TarotSuit::Wands.is_yin());
    assert!(!TarotSuit::Swords.is_yin());
    assert!(TarotSuit::from_bits(4).is_err());
}

/// Deliverable law (a): the 64 codons are covered exactly once — dual
/// courts contribute both codons, so 48 single + 8 dual = 64 slots. Every
/// codon resolves to exactly one card through the cover index, the
/// resolving card lists the codon among its own, and the 56 cards list
/// exactly 64 codon slots in total: together these exclude both under-
/// and double-coverage.
#[test]
fn the_64_codons_are_covered_exactly_once() {
    let bridge = TarotBridge::kernel();
    for address in 0u8..64 {
        let card = bridge
            .card_of_codon(Codon64::new(address))
            .unwrap_or_else(|| panic!("codon {address} must be covered"));
        assert!(
            card.codons().any(|codon| codon.address() == address),
            "{card} resolves codon {address} but does not list it"
        );
    }

    let duals = bridge
        .minor()
        .iter()
        .filter(|card| card.is_dual_court())
        .count();
    assert_eq!(duals, 8, "eight dual-codon courts");
    assert_eq!(
        bridge.minor().len() - duals,
        48,
        "forty-eight single-codon cards"
    );
    let slots: usize = bridge
        .minor()
        .iter()
        .map(|card| card.codons().count())
        .sum();
    assert_eq!(slots, 64, "48 + 8 x 2 = 64 codon slots — the exact cover");
}

/// Deliverable law (b): the court gendering law — yin suits (Cups,
/// Pentacles) carry their dual codons at Prince(Knight)+King; yang suits
/// (Wands, Swords) at Princess(Page)+Queen (FR 2.3.16 header).
#[test]
fn court_gendering_law_holds() {
    let bridge = TarotBridge::kernel();
    for suit in TarotSuit::ALL {
        let dual_pips: Vec<TarotPip> = bridge
            .suit_cards(suit)
            .iter()
            .filter(|card| card.is_dual_court())
            .map(|card| card.pip())
            .collect();
        assert_eq!(
            dual_pips,
            suit.dual_court_pips().to_vec(),
            "dual courts of {suit}"
        );
        if suit.is_yin() {
            assert_eq!(
                dual_pips,
                vec![TarotPip::PRINCE, TarotPip::KING],
                "yin suit {suit} must dual at Prince(Knight)+King"
            );
        } else {
            assert_eq!(
                dual_pips,
                vec![TarotPip::PRINCESS, TarotPip::QUEEN],
                "yang suit {suit} must dual at Princess(Page)+Queen"
            );
        }
        // The opposite register's court pair stays single-codon.
        let singles: Vec<TarotPip> = bridge
            .suit_cards(suit)
            .iter()
            .filter(|card| !card.is_dual_court() && card.pip().is_court())
            .map(|card| card.pip())
            .collect();
        assert_eq!(singles.len(), 2, "two single-codon courts in {suit}");
    }
}

/// Deliverable law (c): every dual pair swaps middle/inner nucleotides
/// keeping the outer — the two codons of a court are one relation
/// reflected across the hinge.
#[test]
fn dual_pairs_swap_middle_and_inner_keeping_the_outer() {
    let bridge = TarotBridge::kernel();
    for suit in TarotSuit::ALL {
        for card in bridge.suit_cards(suit) {
            let Some(codon_b) = card.codon_b() else {
                continue;
            };
            let codon_a = card.codon_a();
            assert_eq!(codon_b.outer(), codon_a.outer(), "{card}: shared outer");
            assert_eq!(codon_b.middle(), codon_a.inner(), "{card}: hinge swap");
            assert_eq!(codon_b.inner(), codon_a.middle(), "{card}: inner swap");
            assert_ne!(codon_a, codon_b, "{card}: a genuine pair");
        }
    }
}

/// Deliverable law (d): every card's codons have outer nucleotide = suit
/// nucleotide — the four suits partition the codon field by outer site.
#[test]
fn every_codon_keeps_the_suit_nucleotide_in_the_outer_site() {
    let bridge = TarotBridge::kernel();
    for card in bridge.minor().iter() {
        for codon in card.codons() {
            assert_eq!(
                codon.outer(),
                card.suit().nucleotide(),
                "{card}: outer of {codon}"
            );
        }
    }
    // The pip register is complete: every suit carries all fourteen slots.
    for suit in TarotSuit::ALL {
        let pips: Vec<u8> = bridge
            .suit_cards(suit)
            .iter()
            .map(|c| c.pip().value())
            .collect();
        assert_eq!(pips, (0u8..14).collect::<Vec<_>>(), "{suit} pip register");
    }
}

/// Suit integrals: per suit the pp total over its 16 family codons is
/// 4 × the integral, with integrals 84/96/92/88 summing to the 360
/// invariant (m3.h:596-600, pinned against the parsed kernel constants in
/// `m3_h_constants_pin_the_typed_surface`). The total pp over all 64
/// codons is 1440 = 4 × 360 — proven independently in
/// `pole_coin_contract.rs`; only the quarter-arithmetic is repeated here.
#[test]
fn suit_integrals_follow_the_outer_family_charge() {
    let bridge = TarotBridge::kernel();
    let mut total_pp = 0u32;
    for suit in TarotSuit::ALL {
        let family_pp: u32 = bridge
            .suit_cards(suit)
            .iter()
            .flat_map(|card| card.codons())
            .map(|codon| codon.codon_sum() as u32)
            .sum();
        assert_eq!(
            family_pp,
            4 * u32::from(suit.integral()),
            "{suit}: family pp per quarter"
        );
        total_pp += family_pp;
    }
    assert_eq!(
        [
            TarotSuit::Cups.integral(),
            TarotSuit::Wands.integral(),
            TarotSuit::Pentacles.integral(),
            TarotSuit::Swords.integral()
        ],
        [84, 96, 92, 88]
    );
    assert_eq!(
        TarotSuit::ALL
            .iter()
            .map(|s| u32::from(s.integral()))
            .sum::<u32>(),
        360,
        "the integral invariant"
    );
    assert_eq!(total_pp, 1440, "4 x 360 — the coin-contract closure");
}

/// The kernel deck layout: `minor()[i].card_id() == i` with
/// `card_id = suit x 14 + pip` (the addressing of `m3_tarot_rotation`,
/// `m3.h:806-810`).
#[test]
fn deck_order_matches_the_kernel_card_id_layout() {
    let bridge = TarotBridge::kernel();
    for (index, card) in bridge.minor().iter().enumerate() {
        assert_eq!(card.card_id() as usize, index, "deck order at {card}");
        assert_eq!(card.card_id(), card.suit().bits() * 14 + card.pip().value());
    }
    for suit in TarotSuit::ALL {
        for (pip_index, card) in bridge.suit_cards(suit).iter().enumerate() {
            assert_eq!(card.pip().value(), pip_index as u8, "{suit} pip order");
        }
    }
}

/// The Major Arcana table: 22 rows over chromosome pairs 1-22 and
/// amino-acid indices 0-21 (m3.c:273-296 — the recorded regularity
/// chromosome = id+1, amino = id is pinned verbatim).
#[test]
fn major_arcana_table_is_the_recorded_22() {
    let bridge = TarotBridge::kernel();
    for major in bridge.major().iter() {
        assert_eq!(major.amino_acid_index() as usize, major.card_id() as usize);
        assert_eq!(major.chromosome_pair(), major.card_id() + 1);
        assert!(!major.name().is_empty());
    }
    assert_eq!(bridge.major()[0].name(), "The Fool");
    assert_eq!(bridge.major()[8].name(), "Adjustment");
    assert_eq!(bridge.major()[11].name(), "Lust");
    assert_eq!(bridge.major()[20].name(), "Aeon");
    assert_eq!(bridge.major()[21].name(), "The Universe");
}

/// The 22+2 governance law (matrix §10, M3-C20): The Fool =
/// start/opening/0 and The Universe = stop/completion/1 as two additional
/// functional readings of the boundary Majors — quaternion ids 78/79,
/// closing the 80-entry layout.
#[test]
fn the_22_plus_2_governance_law_is_typed() {
    assert_eq!(TranscendentOperator::ALL.len(), TRANSCENDENT_TAROT_COUNT);
    let bridge = TarotBridge::kernel();
    for (expected_index, expected_boundary, expected_name, expected_role, operator) in [
        (
            0u8,
            0u8,
            "The Fool",
            "start/opening",
            TranscendentOperator::Fool,
        ),
        (
            21,
            1,
            "The Universe",
            "stop/completion",
            TranscendentOperator::Universe,
        ),
    ] {
        assert_eq!(operator.major_index(), expected_index);
        assert_eq!(operator.boundary_value(), expected_boundary);
        assert_eq!(operator.role(), expected_role);
        assert_eq!(
            bridge.major()[operator.major_index() as usize].name(),
            expected_name,
            "{operator:?} reads the boundary Major"
        );
        assert_eq!(
            operator.quaternion_id(),
            (MINOR_ARCANA_COUNT + MAJOR_ARCANA_COUNT) as u8 + operator.boundary_value()
        );
    }
    assert_eq!(TranscendentOperator::Fool.boundary_value(), 0);
    assert_eq!(TranscendentOperator::Universe.boundary_value(), 1);
    assert_eq!(TranscendentOperator::Fool.quaternion_id(), 78);
    assert_eq!(TranscendentOperator::Universe.quaternion_id(), 79);
}

/// The Thoth/RWS naming duality: pip 10 = Princess(Page), pip
/// 11 = Prince(Knight) — one deck, two registers (m3.h `M3_TAROT_PIP_*`
/// constants vs the `TAROT_RANK_NAMES` table, `m3.h:635-639`).
#[test]
fn naming_duality_is_one_deck_two_registers() {
    let rank_names = header_string_array("TAROT_RANK_NAMES[14]");
    assert_eq!(rank_names.len(), 14, "the kernel rank-name table");
    for pip in TarotPip::ALL {
        assert_eq!(
            pip.rws_name(),
            rank_names[pip.value() as usize],
            "RWS register of pip {}",
            pip.value()
        );
        match pip.value() {
            10 => {
                assert_eq!(pip.thoth_name(), "Princess");
                assert_eq!(pip.rws_name(), "Page");
            }
            11 => {
                assert_eq!(pip.thoth_name(), "Prince");
                assert_eq!(pip.rws_name(), "Knight");
            }
            _ => assert_eq!(pip.thoth_name(), pip.rws_name()),
        }
    }
    assert_eq!(TarotPip::ALL.len(), 14);
    for (value, pip) in TarotPip::ALL.iter().enumerate() {
        assert_eq!(pip.value(), value as u8);
    }
    // The court register: pips 10-13 are courts, 0-9 are number cards.
    for pip in TarotPip::ALL {
        assert_eq!(pip.is_court(), pip.value() >= 10, "{}", pip.thoth_name());
    }
    assert!(TarotPip::new(13).is_ok());
    assert!(TarotPip::new(14).is_err());
}

/// The kernel print register: "<RWS rank> of <suit>" (m3.c:1006-1007).
#[test]
fn cards_display_in_the_kernel_print_register() {
    let bridge = TarotBridge::kernel();
    let page_of_cups = &bridge.minor()[10];
    assert_eq!(page_of_cups.pip(), TarotPip::PRINCESS);
    assert_eq!(page_of_cups.rank_name(), "Page");
    assert_eq!(page_of_cups.to_string(), "Page of Cups");
    let knight_of_wands = &bridge.minor()[TarotSuit::Wands.bits() as usize * 14 + 11];
    assert_eq!(knight_of_wands.pip(), TarotPip::PRINCE);
    assert_eq!(knight_of_wands.rank_name(), "Knight");
    assert_eq!(knight_of_wands.to_string(), "Knight of Wands");
}

/// The fixture is the machine-readable deck contract: every row is checked
/// against the parsed kernel bridge, and the bridge totals are checked
/// against the fixture counts.
#[test]
fn fixture_cross_pins_the_parsed_bridge() {
    let bridge = TarotBridge::kernel();

    let mut suit_rows = 0;
    let mut card_rows = 0;
    let mut major_rows = 0;
    let mut governance_rows = 0;
    let mut counts: Vec<(String, usize)> = Vec::new();

    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<&str> = row.split('\t').collect();
        match fields[0] {
            "meta" => {}
            "suit" => {
                suit_rows += 1;
                let suit = TarotSuit::from_bits(fields[1].parse().expect("suit bits"))
                    .unwrap_or_else(|_| panic!("suit row: {row}"));
                assert_eq!(suit.name(), fields[2], "{row}");
                assert_eq!(suit.nucleotide().symbol().to_string(), fields[3], "{row}");
                assert_eq!(suit.element().name(), fields[4], "{row}");
                assert_eq!(
                    if suit.is_yin() { "yin" } else { "yang" },
                    fields[5],
                    "{row}"
                );
                assert_eq!(suit.integral().to_string(), fields[6], "{row}");
            }
            "card" => {
                card_rows += 1;
                let card_id: usize = fields[1].parse().expect("card id");
                let card = &bridge.minor()[card_id];
                assert_eq!(card.suit().bits().to_string(), fields[2], "{row}");
                assert_eq!(card.pip().value().to_string(), fields[3], "{row}");
                assert_eq!(card.pip().thoth_name(), fields[4], "{row}");
                assert_eq!(card.pip().rws_name(), fields[5], "{row}");
                assert_eq!(card.codon_a().to_string(), fields[6], "{row}");
                assert_eq!(
                    card.codon_b()
                        .map(|codon| codon.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                    fields[7],
                    "{row}"
                );
                assert_eq!(card.is_dual_court(), fields[7] != "-", "{row}");
            }
            "major" => {
                major_rows += 1;
                let major = &bridge.major()[fields[1].parse::<usize>().expect("major id")];
                assert_eq!(major.name(), fields[2], "{row}");
                assert_eq!(major.chromosome_pair().to_string(), fields[3], "{row}");
                assert_eq!(major.amino_acid_index().to_string(), fields[4], "{row}");
            }
            "governance" => {
                governance_rows += 1;
                let operator = match fields[1] {
                    "Fool" => TranscendentOperator::Fool,
                    "Universe" => TranscendentOperator::Universe,
                    other => panic!("unknown governance operator: {other}"),
                };
                assert_eq!(operator.major_index().to_string(), fields[2], "{row}");
                assert_eq!(operator.boundary_value().to_string(), fields[3], "{row}");
                assert_eq!(operator.quaternion_id().to_string(), fields[4], "{row}");
                assert_eq!(operator.role(), fields[5], "{row}");
            }
            "counts" => counts.push((fields[1].to_string(), fields[2].parse().expect("count"))),
            other => panic!("unknown fixture row kind: {other}"),
        }
    }

    assert_eq!(suit_rows, 4);
    assert_eq!(card_rows, MINOR_ARCANA_COUNT);
    assert_eq!(major_rows, MAJOR_ARCANA_COUNT);
    assert_eq!(governance_rows, TRANSCENDENT_TAROT_COUNT);

    let covered = (0u8..64)
        .filter(|address| bridge.card_of_codon(Codon64::new(*address)).is_some())
        .count();
    let integral_quarter: u32 = TarotSuit::ALL.iter().map(|s| u32::from(s.integral())).sum();
    let expected = vec![
        ("minor".to_string(), MINOR_ARCANA_COUNT),
        ("major".to_string(), MAJOR_ARCANA_COUNT),
        ("transcendent".to_string(), TRANSCENDENT_TAROT_COUNT),
        ("quaternion-total".to_string(), TAROT_QUATERNION_COUNT),
        ("covered-codons".to_string(), covered),
        (
            "integral-total".to_string(),
            (4 * integral_quarter) as usize,
        ),
    ];
    assert_eq!(counts, expected);
    assert_eq!(covered, 64);
    assert_eq!(integral_quarter, 360, "the 360 invariant over the quarters");
    assert_eq!(4 * integral_quarter, 1440, "total pp over all 64 codons");
}

/// The parser keeps the [`QlError::InvalidPoleValue`] convention: a
/// malformed source names its failing coordinate instead of panicking.
#[test]
fn parser_rejects_malformed_source_with_invalid_pole_value() {
    // Missing table anchor.
    let error = parse_kernel_source("").expect_err("empty source must not parse");
    match error {
        QlError::InvalidPoleValue { field, .. } => assert_eq!(field, "tarot-table-anchor"),
        other => panic!("unexpected error: {other:?}"),
    }

    // An impossible nucleotide symbol inside a COD(..) codon.
    let corrupted: &'static str = Box::leak(
        VENDOR_M3_C
            .replacen("COD(A,A,A)", "COD(A,A,Z)", 1)
            .into_boxed_str(),
    );
    let error = parse_kernel_source(corrupted).expect_err("Z is not in the alphabet");
    match error {
        QlError::InvalidPoleValue { field, .. } => assert_eq!(field, "tarot-codon-symbol"),
        other => panic!("unexpected error: {other:?}"),
    }

    // A duplicated codon breaks the exact cover.
    let duplicated: &'static str = Box::leak(
        VENDOR_M3_C
            .replacen("COD(A,A,G)", "COD(A,A,A)", 1)
            .into_boxed_str(),
    );
    let error = parse_kernel_source(duplicated).expect_err("AAA covered twice");
    match error {
        QlError::InvalidPoleValue { field, value } => {
            assert_eq!(field, "tarot-codon-duplicate");
            assert_eq!(value, 0, "AAA is codon address 0");
        }
        other => panic!("unexpected error: {other:?}"),
    }

    // The pristine vendor source parses (the same bytes kernel() uses).
    assert!(parse_kernel_source(VENDOR_M3_C).is_ok());
}
