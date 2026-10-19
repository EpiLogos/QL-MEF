//! Conformance for the M3 generation law (`ql.pole.m3-generation-law/v1`):
//! cell counts, fixed points, matrix equivariance and the Klein composition.
//! The 192 map cells themselves are reproduced in
//! `crates/ql-mef/tests/m3_generation_field.rs`.

use std::collections::HashSet;

use ql_core::{
    Codon64, CodonClass, FoldState, GENERATION_CELL_COUNT, MatrixFamily, Nucleotide,
    POLE_GENERATION_LAW_REF, all_cells, generate, generation_cells, matrix_partner,
};

const PALINDROMIC_TRIGRAMS: [u8; 4] = [7, 0, 2, 5]; // Qian, Kun, Kan, Li

#[test]
fn contract_identity_is_versioned() {
    assert_eq!(POLE_GENERATION_LAW_REF, "ql.pole.m3-generation-law/v1");
    assert_eq!(GENERATION_CELL_COUNT, 192);
}

#[test]
fn each_matrix_has_16_non_dual_cells_and_48_dual() {
    for family in MatrixFamily::ALL {
        let cells: Vec<_> = generation_cells(family).collect();
        assert_eq!(cells.len(), 64);
        let non_dual: Vec<_> = cells.iter().filter(|c| c.is_non_dual()).collect();
        assert_eq!(non_dual.len(), 16, "{family:?}");
        let count = |class: CodonClass| non_dual.iter().filter(|c| c.class == class).count();
        assert_eq!(count(CodonClass::PerfectPalindromic), 4, "{family:?}");
        assert_eq!(count(CodonClass::ImperfectPalindromic), 4, "{family:?}");
        assert_eq!(count(CodonClass::NonPalindromicNonDual), 8, "{family:?}");
        assert_eq!(
            cells.iter().filter(|c| c.class == CodonClass::Dual).count(),
            48
        );
    }
}

#[test]
fn the_field_yields_384_slots_336_distinct_yields_and_96_non_dual_slots() {
    let cells: Vec<_> = all_cells().collect();
    assert_eq!(cells.len(), 192);
    assert_eq!(cells.len() * 2, 384, "two polarity slots per cell");
    // A non-dual cell yields one codon in both slots; a dual cell two.
    let distinct: usize = cells
        .iter()
        .map(|c| if c.is_non_dual() { 1 } else { 2 })
        .sum();
    assert_eq!(distinct, 336);
    let non_dual_slots = cells.iter().filter(|c| c.is_non_dual()).count() * 2;
    assert_eq!(non_dual_slots, 96);
}

#[test]
fn palindromic_trigrams_take_the_same_pair_in_every_matrix() {
    for pattern in PALINDROMIC_TRIGRAMS {
        let pairs: HashSet<u8> = MatrixFamily::ALL
            .into_iter()
            .map(|f| ql_core::trigram_pair(f, pattern).index())
            .collect();
        assert_eq!(pairs.len(), 1, "trigram {pattern:03b}");
        let pair = ql_core::trigram_pair(MatrixFamily::Complementary, pattern);
        assert_eq!(pair.first(), pair.second());
    }
}

#[test]
fn every_mixed_trigram_gets_a_mixed_pair_of_the_matrixs_base_partition() {
    for family in MatrixFamily::ALL {
        let mut mixed = HashSet::new();
        for pattern in 0u8..8 {
            if PALINDROMIC_TRIGRAMS.contains(&pattern) {
                continue;
            }
            let pair = ql_core::trigram_pair(family, pattern);
            // The two bases are partners under the matrix.
            assert_eq!(matrix_partner(family, pair.first()), pair.second());
            assert!(mixed.insert(pair.index()), "mixed pairs are distinct");
        }
        assert_eq!(mixed.len(), 4);
    }
}

#[test]
fn the_fixed_points_of_the_cells_are_exactly_the_40_non_dual_codons() {
    let fixed: HashSet<u8> = all_cells()
        .filter_map(|c| c.fixed_point())
        .map(|c| c.address())
        .collect();
    let non_dual: HashSet<u8> = (0u8..64)
        .filter(|a| Codon64::new(*a).classify() != CodonClass::Dual)
        .collect();
    assert_eq!(non_dual.len(), 40);
    assert_eq!(fixed, non_dual);
    // The 24 dual codons are the codons of three distinct bases.
    let dual: Vec<u8> = (0u8..64)
        .filter(|a| Codon64::new(*a).classify() == CodonClass::Dual)
        .collect();
    assert_eq!(dual.len(), 24);
    for a in dual {
        let [x, y, z] = Codon64::new(a).nucleotides();
        assert!(x != y && y != z && x != z);
    }
}

#[test]
fn a_cell_is_non_dual_exactly_when_its_two_codons_coincide() {
    for cell in all_cells() {
        assert_eq!(cell.is_non_dual(), cell.positive == cell.negative);
        assert_eq!(cell.fixed_point().is_some(), cell.is_non_dual());
        // positive = a·c·d, negative = a·b·d
        let (a, b) = (cell.lower_pair.first(), cell.lower_pair.second());
        let (c, d) = (cell.upper_pair.first(), cell.upper_pair.second());
        assert_eq!(cell.positive.nucleotides(), [a, c, d]);
        assert_eq!(cell.negative.nucleotides(), [a, b, d]);
    }
}

#[test]
fn each_matrix_carries_every_cell_of_its_own_field_to_the_partner_cell() {
    // The hexagram action carries the cell at h to the cell whose positive and
    // negative codons are the partner codons of the cell's own, in both
    // polarity slots.
    for family in MatrixFamily::ALL {
        for hexagram in 0u8..64 {
            let cell = generate(family, hexagram);
            let image = generate(family, family.apply_to_hexagram(hexagram));
            assert_eq!(
                image.positive,
                family.apply_to_codon(cell.positive),
                "{family:?} {hexagram}"
            );
            assert_eq!(
                image.negative,
                family.apply_to_codon(cell.negative),
                "{family:?} {hexagram}"
            );
            assert_eq!(image.is_non_dual(), cell.is_non_dual());
        }
    }
}

#[test]
fn the_codon_action_permutes_the_fixed_points_of_its_field() {
    for family in MatrixFamily::ALL {
        let fixed: HashSet<u8> = generation_cells(family)
            .filter_map(|c| c.fixed_point())
            .map(|c| c.address())
            .collect();
        assert_eq!(fixed.len(), 16);
        for address in &fixed {
            let image = family.apply_to_codon(Codon64::new(*address));
            assert!(fixed.contains(&image.address()), "{family:?} {address}");
        }
    }
}

#[test]
fn trigram_actions_flip_the_lines_the_law_states() {
    // Bottom line is bit 0. Matrix 1 flips all three lines; Matrix 2 flips the
    // outer lines of the palindromic trigrams and the middle line of the other
    // four; Matrix 3 the middle line of the palindromic and the outer of the
    // other four.
    const OUTER: u8 = 0b101;
    const MIDDLE: u8 = 0b010;
    for pattern in 0u8..8 {
        let palindromic = PALINDROMIC_TRIGRAMS.contains(&pattern);
        assert_eq!(
            MatrixFamily::Complementary.trigram_line_flips(pattern),
            0b111
        );
        assert_eq!(
            MatrixFamily::MovingResting.trigram_line_flips(pattern),
            if palindromic { OUTER } else { MIDDLE },
            "matrix 2 trigram {pattern:03b}"
        );
        assert_eq!(
            MatrixFamily::SameQuality.trigram_line_flips(pattern),
            if palindromic { MIDDLE } else { OUTER },
            "matrix 3 trigram {pattern:03b}"
        );
    }
}

#[test]
fn every_matrix_action_is_a_fixed_point_free_involution_at_both_scales() {
    for family in MatrixFamily::ALL {
        for hexagram in 0u8..64 {
            let image = family.apply_to_hexagram(hexagram);
            assert_ne!(image, hexagram);
            assert_eq!(family.apply_to_hexagram(image), hexagram);
        }
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let image = family.apply_to_codon(codon);
            assert_ne!(image, codon);
            assert_eq!(family.apply_to_codon(image), codon);
        }
    }
}

#[test]
fn matrix_one_then_two_is_matrix_three_and_the_group_is_klein() {
    let (m1, m2, m3) = (
        MatrixFamily::Complementary,
        MatrixFamily::MovingResting,
        MatrixFamily::SameQuality,
    );
    for hexagram in 0u8..64 {
        assert_eq!(
            m2.apply_to_hexagram(m1.apply_to_hexagram(hexagram)),
            m3.apply_to_hexagram(hexagram)
        );
        assert_eq!(
            m1.apply_to_hexagram(m2.apply_to_hexagram(hexagram)),
            m3.apply_to_hexagram(hexagram)
        );
        assert_eq!(
            m3.apply_to_hexagram(m1.apply_to_hexagram(hexagram)),
            m2.apply_to_hexagram(hexagram)
        );
    }
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        assert_eq!(
            m2.apply_to_codon(m1.apply_to_codon(codon)),
            m3.apply_to_codon(codon)
        );
        assert_eq!(
            m3.apply_to_codon(m1.apply_to_codon(codon)),
            m2.apply_to_codon(codon)
        );
    }
}

#[test]
fn codon_masks_are_derived_from_the_partner_table() {
    let masks: Vec<u8> = MatrixFamily::ALL
        .into_iter()
        .map(|f| f.codon_mask())
        .collect();
    assert_eq!(masks, [0x15, 0x3F, 0x2A]);
    for family in MatrixFamily::ALL {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let by_site: Vec<Nucleotide> = codon
                .nucleotides()
                .into_iter()
                .map(|n| matrix_partner(family, n))
                .collect();
            assert_eq!(family.apply_to_codon(codon).nucleotides().to_vec(), by_site);
        }
    }
}

#[test]
fn the_fold_state_applies_the_same_codon_action() {
    let aperture = ql_core::ApertureIndex::new(5).unwrap();
    for family in MatrixFamily::ALL {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let state = FoldState::from_codon(codon, aperture, 9);
            assert_eq!(
                state.apply_matrix(family).codon(),
                family.apply_to_codon(codon)
            );
        }
    }
}
