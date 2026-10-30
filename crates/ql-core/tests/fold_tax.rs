//! Conformance for `ql.qlc0.fold-tax/v1`: every kernel fold's tax, its exact
//! lossless code, and the receipt `fixtures/kernel/qlc0-fold-tax-v1.tsv`.
//!
//! Regenerate the receipt after a deliberate kernel change with
//! `QLC0_WRITE_FIXTURE=1 cargo test -p ql-core --test fold_tax`.

use std::collections::BTreeSet;

use ql_core::{
    FoldCode, FoldTax, all_cells, d1_conjugacy_pairs, epogdoon_arithmetic_pairs,
    epogdoon_det_pairs, epogdoon_fibre_pairs, fold_tax_table, fold_tax_tsv,
    generation_yield_weights,
};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/kernel/qlc0-fold-tax-v1.tsv"
);

fn row(map_ref: &str, distribution_ref: &str) -> FoldTax {
    fold_tax_table()
        .into_iter()
        .find(|row| row.map_ref == map_ref && row.distribution_ref == distribution_ref)
        .unwrap_or_else(|| panic!("no row {map_ref} / {distribution_ref}"))
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

#[test]
fn receipt_matches_the_kernel() {
    let rendered = fold_tax_tsv();
    if std::env::var_os("QLC0_WRITE_FIXTURE").is_some() {
        std::fs::write(FIXTURE_PATH, &rendered).unwrap();
    }
    let fixture = std::fs::read_to_string(FIXTURE_PATH).expect("QLC-0 receipt exists");
    assert_eq!(rendered, fixture, "QLC-0 receipt drifted from the kernel");
}

#[test]
fn every_row_decomposes_source_into_image_plus_tax() {
    for row in fold_tax_table() {
        assert!(
            close(row.source_bits, row.image_bits + row.tax_bits),
            "{} / {}: H(S) = H(I) + H(S|I)",
            row.map_ref,
            row.distribution_ref
        );
        let reached: usize = row.preimage_histogram.values().sum();
        assert_eq!(
            reached, row.image,
            "{}: histogram covers images",
            row.map_ref
        );
        let covered: usize = row.preimage_histogram.iter().map(|(s, n)| s * n).sum();
        assert_eq!(
            covered, row.support,
            "{}: histogram covers sources",
            row.map_ref
        );
    }
}

#[test]
fn same_position_conjugacy_is_free_at_the_address() {
    let d1 = row("d1-same-position-conjugacy", "uniform-12");
    assert_eq!((d1.support, d1.image, d1.max_preimage), (12, 12, 1));
    assert!(close(d1.tax_bits, 0.0));
    assert!(close(d1.source_bits, 12f64.log2()));
}

#[test]
fn the_three_epogdoon_descriptions_pay_one_tax_on_different_collisions() {
    let tax = 16.0 / 72.0;
    for map_ref in [
        "epogdoon-det-flat-72-64",
        "epogdoon-fibre-t18-16",
        "epogdoon-arithmetic-8-9",
    ] {
        let fold = row(map_ref, "uniform-72");
        assert_eq!((fold.support, fold.image, fold.max_preimage), (72, 64, 2));
        assert_eq!(fold.preimage_histogram.get(&2), Some(&8));
        assert!(
            close(fold.tax_bits, tax),
            "{map_ref}: tax {}",
            fold.tax_bits
        );
    }
    // Same tax, three codes: the sources that collide are not the same set.
    let colliding = |pairs: Vec<(u32, u32)>| -> BTreeSet<u32> {
        let code = FoldCode::new(pairs.iter().copied());
        pairs
            .iter()
            .filter(|(_, image)| code.residual_alphabet(*image) == 2)
            .map(|(source, _)| *source)
            .collect()
    };
    let flat = colliding(
        epogdoon_det_pairs()
            .into_iter()
            .map(|(s, i)| (u32::from(s), u32::from(i)))
            .collect(),
    );
    let fibre = colliding(
        epogdoon_fibre_pairs()
            .into_iter()
            .map(|((e, s), (_, t))| {
                (
                    u32::from(e) * 18 + u32::from(s),
                    u32::from(e) * 16 + u32::from(t),
                )
            })
            .collect(),
    );
    let arithmetic = colliding(
        epogdoon_arithmetic_pairs()
            .into_iter()
            .map(|(s, i)| (u32::from(s), u32::from(i)))
            .collect(),
    );
    assert_ne!(flat, arithmetic);
    assert_ne!(flat, fibre);
    assert_ne!(fibre, arithmetic);
}

#[test]
fn every_fold_code_round_trips_every_source() {
    fn round_trip<S: Ord + Copy + std::fmt::Debug, I: Ord + Copy>(pairs: Vec<(S, I)>) {
        let code = FoldCode::new(pairs.iter().copied());
        for (source, image) in pairs {
            let (split_image, rank) = code.split(source).unwrap();
            assert!(split_image == image);
            assert!(rank < code.residual_alphabet(image));
            assert_eq!(code.join(split_image, rank), Some(source));
        }
    }
    round_trip(d1_conjugacy_pairs());
    round_trip(epogdoon_det_pairs());
    round_trip(epogdoon_fibre_pairs());
    round_trip(epogdoon_arithmetic_pairs());
    round_trip(
        all_cells()
            .map(|c| {
                (
                    (
                        c.family.matrix_number(),
                        c.positive.address(),
                        c.negative.address(),
                    ),
                    (c.family.matrix_number(), c.positive.address()),
                )
            })
            .collect(),
    );
}

#[test]
fn the_quaternion_reading_keeps_46_of_64_codons() {
    let quat = row("quat-from-codon", "uniform-64");
    assert_eq!((quat.support, quat.image), (64, 46));
    assert!(quat.tax_bits > 0.5 && quat.tax_bits < 0.6);
}

#[test]
fn a_known_matrix_leaves_the_conjugate_codon_one_bit() {
    let given = row("generation-conjugate-given-matrix", "generation-cells-192");
    assert!(close(given.tax_bits, 1.0), "tax {}", given.tax_bits);
    // Every (matrix, positive) is shared by exactly two cells, so a
    // non-dual cell is never recognisable from its positive codon alone.
    assert_eq!(
        given.preimage_histogram.into_iter().collect::<Vec<_>>(),
        vec![(2, 96)]
    );
    let alone = row("generation-negative-to-anchor", "generation-cells-192");
    assert_eq!(alone.image, 1);
    assert!(
        alone.tax_bits > 5.0,
        "the negative codon alone costs {}",
        alone.tax_bits
    );
    let unknown = row(
        "generation-conjugate-matrix-unknown",
        "generation-cells-192",
    );
    assert!(unknown.tax_bits >= given.tax_bits);
    assert!(unknown.tax_bits < alone.tax_bits);
}

#[test]
fn generation_yields_weigh_all_384_slots() {
    let weights = generation_yield_weights();
    assert_eq!(weights.values().sum::<u64>(), 384);
    assert_eq!(weights.len(), 64);
}
