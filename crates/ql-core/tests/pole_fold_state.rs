//! Conformance for `ql.pole.fold-state/v1` — the T5 fold/rūpa state
//! projection: cast law, hinge geometry, matrix-axis transformations,
//! reciprocal-aperture fold, and the open inverse seam.

use ql_core::{
    ApertureIndex, ApplyOutcome, Codon64, MatrixFamily, Mobility, POLE_FOLD_STATE_REF, Polarity,
    QuaternionComponents, RetrievalEvidence, SelectionContext, SiteReading,
};

const FIXTURE: &str = include_str!("../../../fixtures/pole/fold-state-v1.tsv");

fn rows(tag: &str) -> Vec<Vec<String>> {
    FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').map(|f| f.to_string()).collect::<Vec<_>>())
        .filter(|fields| fields[0] == tag)
        .collect()
}

#[test]
fn contract_identity_is_versioned() {
    assert_eq!(POLE_FOLD_STATE_REF, "ql.pole.fold-state/v1");
}

#[test]
fn fixture_site_conventions_match_the_cast_law() {
    let rows = rows("site-convention");
    assert_eq!(rows.len(), 4, "the four site states");
    for fields in rows {
        let polarity = match fields[1].as_str() {
            "yin" => Polarity::Yin,
            _ => Polarity::Yang,
        };
        let mobility = match fields[2].as_str() {
            "moving" => Mobility::Moving,
            _ => Mobility::Resting,
        };
        let reading = SiteReading {
            signed_angle: fields[5].parse().unwrap(),
            angular_velocity: fields[6].parse().unwrap(),
        };
        let cast = reading.cast();
        assert_eq!(cast.polarity, polarity, "cast polarity for {fields:?}");
        assert_eq!(cast.mobility, mobility, "cast mobility for {fields:?}");
    }
}

#[test]
fn fixture_matrix_complement_rows_flip_all_six_bits() {
    let rows = rows("matrix-complement");
    assert_eq!(rows.len(), 64);
    for fields in rows {
        let address: u8 = fields[1].parse().unwrap();
        let expected: u8 = fields[2].parse().unwrap();
        let state = FoldStateForTest::new(address);
        let complemented = state.apply(MatrixFamily::Complementary);
        assert_eq!(complemented, expected, "complement of {address}");
    }
}

#[test]
fn fixture_matrix_move_rows_exchange_trigrams() {
    let rows = rows("matrix-move");
    assert_eq!(rows.len(), 64);
    for fields in rows {
        let address: u8 = fields[1].parse().unwrap();
        let expected: u8 = fields[2].parse().unwrap();
        let state = FoldStateForTest::new(address);
        let moved = state.apply(MatrixFamily::MovingResting);
        assert_eq!(moved, expected, "trigram exchange of {address}");
    }
}

/// Thin helper exercising FoldState's matrix application.
struct FoldStateForTest(ql_core::FoldState);

impl FoldStateForTest {
    fn new(address: u8) -> Self {
        Self(ql_core::FoldState::from_codon(
            Codon64::new(address),
            ApertureIndex::new(0).expect("aperture"),
            0,
        ))
    }

    fn apply(&self, family: MatrixFamily) -> u8 {
        self.0
            .apply_matrix(family)
            .expect("matrix law applies")
            .applied()
            .expect("determinate transform")
            .codon()
            .address()
    }
}

#[test]
fn fixture_resonance_rows_are_the_ported_dataset_table() {
    let rows = rows("matrix-resonance");
    assert_eq!(rows.len(), 1);
    // The k axis is no longer typed-unresolved: the RES matrix is ported
    // verbatim from the C kernel — identity partners on the 56 admitted
    // entries, the 8 evolutionary gaps typed as Provisional.
    assert_eq!(rows[0][1], "identity-on-56-admitted");
    assert_eq!(rows[0][2], "dataset-structural");
    assert_eq!(rows[0][3], "8-gaps");
    // Executable half: admitted codons apply through the k axis; gap codons
    // yield the typed provisional outcome, never an error.
    let applied = FoldStateForTest::new(0)
        .0
        .apply_matrix(MatrixFamily::SameQuality)
        .expect("k axis executes");
    assert!(matches!(applied, ApplyOutcome::Applied(_)));
    assert!(matches!(
        FoldStateForTest::new(0x05)
            .0
            .apply_matrix(MatrixFamily::SameQuality)
            .expect("k axis executes"),
        ApplyOutcome::Provisional
    ));
}

#[test]
fn fixture_reciprocal_rows_enact_the_antipodal_fold() {
    let rows = rows("reciprocal");
    assert_eq!(rows.len(), 8);
    for fields in rows {
        let a: u8 = fields[1].parse().unwrap();
        let b: u8 = fields[2].parse().unwrap();
        let mut state = ql_core::FoldState::from_codon(
            Codon64::new(0),
            ApertureIndex::new(a).expect("aperture"),
            0,
        );
        state.fold_to_reciprocal_aperture();
        assert_eq!(state.aperture16().index(), b, "reciprocal of {a}");
        state.fold_to_reciprocal_aperture();
        assert_eq!(
            state.aperture16().index(),
            a,
            "reciprocity is an involution"
        );
    }
}

#[test]
fn inverse_seam_stays_open_with_the_roles_typed() {
    let rows = rows("inverse-seam");
    assert_eq!(rows[0][1], "M3-C31");
    assert_eq!(rows[0][2], "open");
    assert_eq!(rows[0][3], "retrieval-vs-canonical-split");

    // Retrieval is alias-tolerant evidence; canonical selection needs a law.
    // CanonicalAddress has no public constructor: laws live kernel-side, so
    // no consumer can collapse retrieval into a canonical verdict.
    let evidence = RetrievalEvidence::from_scores(vec![(Codon64::new(21), 90)]);
    assert_eq!(evidence.best().map(|(c, _)| c.address()), Some(21));
    // The context quaternions are composed in the ratified elemental basis
    // (w=Earth, x=Fire, y=Water, z=Air) — the kernel-side audit found no
    // deterministic, grounded selection law, and the typed split is the
    // strengthened ground any future law reads.
    let context = SelectionContext {
        q_identity: QuaternionComponents {
            w: 1,
            x: 0,
            y: 0,
            z: 0,
        },
        q_composed: QuaternionComponents {
            w: 0,
            x: 1,
            y: 0,
            z: 0,
        },
    };
    // The elemental reads route through the basis, not raw slots.
    use ql_core::Element;
    assert_eq!(
        context.by_element(true, Element::Fire),
        1,
        "composed quaternion's Fire component"
    );
    // The context types keep Q_identity and Q_composed separately typed
    // (acceptance criterion 11); evidence never substitutes for them.
    let _ = (context.q_identity, context.q_composed);
}

#[test]
fn projection_cast_round_trip_over_all_64_addresses() {
    for address in 0u8..64 {
        let state = ql_core::FoldState::from_codon(
            Codon64::new(address),
            ApertureIndex::new(5).expect("aperture"),
            42,
        );
        let recast = ql_core::FoldState::from_cast(
            *state.sites(),
            ApertureIndex::new(5).expect("aperture"),
            42,
        );
        assert_eq!(recast.codon().address(), address);
        assert_eq!(recast.motif().bits(), state.motif().bits());
    }
}
