//! Conformance for `ql.pole.fold-state/v1` — the T5 fold/rūpa state
//! projection: cast law, hinge geometry, matrix-axis transformations,
//! reciprocal-aperture fold, and the open inverse seam.

use ql_core::{
    ApertureIndex, Codon64, MatrixFamily, Mobility, POLE_FOLD_STATE_REF, Polarity,
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

/// The fixture rows are derived independently of the kernel from the three
/// base-partner maps; the kernel's `apply_matrix` must reproduce every row.
fn matrix_rows_reproduced(tag: &str, family: MatrixFamily) {
    let rows = rows(tag);
    assert_eq!(rows.len(), 64);
    for fields in rows {
        let address: u8 = fields[1].parse().unwrap();
        let expected: u8 = fields[2].parse().unwrap();
        let applied = FoldStateForTest::new(address).apply(family);
        assert_eq!(applied, expected, "{tag} of {address}");
    }
}

#[test]
fn fixture_matrix_i_rows_are_the_complementary_partner() {
    matrix_rows_reproduced("matrix-i", MatrixFamily::Complementary);
}

#[test]
fn fixture_matrix_j_rows_are_the_moving_resting_partner() {
    matrix_rows_reproduced("matrix-j", MatrixFamily::MovingResting);
}

#[test]
fn fixture_matrix_k_rows_are_the_same_quality_partner() {
    matrix_rows_reproduced("matrix-k", MatrixFamily::SameQuality);
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
        self.0.apply_matrix(family).codon().address()
    }
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
