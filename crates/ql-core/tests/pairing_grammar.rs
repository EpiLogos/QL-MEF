use std::collections::HashSet;

use ql_core::{
    CanonicalCrossPass, ConjugationDegree, D2CrossPassKind, ExpansionSide,
    PAIRING_GRAMMAR_VERSION, PairingError, QlFace, QlPosition, RelationFamily, all_d3_fields,
    build_d_modulation_frame, canonical_cross_pass_d1, canonical_cross_pass_d2,
    canonical_cross_pass_d3,
};

fn p(value: u8) -> QlPosition {
    QlPosition::new(value).expect("test positions are canonical")
}

#[test]
fn promoted_pairing_grammar_has_its_own_version_boundary() {
    assert_eq!(PAIRING_GRAMMAR_VERSION, "1.0.0");
    assert_eq!(RelationFamily::A.pairs(), [(0, 1), (2, 3), (4, 5)]);
    assert_eq!(RelationFamily::B.pairs(), [(1, 2), (3, 4), (5, 0)]);
    assert_eq!(RelationFamily::C.pairs(), [(0, 5), (1, 4), (2, 3)]);
}

#[test]
fn software_d1_d2_d3_modulation_is_exactly_two_three_four_coordinates() {
    let d1 = build_d_modulation_frame(
        RelationFamily::B,
        1,
        ConjugationDegree::D1,
        None,
    )
    .unwrap();
    let d2_left = build_d_modulation_frame(
        RelationFamily::B,
        1,
        ConjugationDegree::D2,
        Some(ExpansionSide::Left),
    )
    .unwrap();
    let d2_right = build_d_modulation_frame(
        RelationFamily::B,
        1,
        ConjugationDegree::D2,
        Some(ExpansionSide::Right),
    )
    .unwrap();
    let d3 = build_d_modulation_frame(
        RelationFamily::B,
        1,
        ConjugationDegree::D3,
        None,
    )
    .unwrap();

    assert_eq!(d1.coordinates.len(), 2);
    assert_eq!(d1.coordinates[0].position, p(3));
    assert_eq!(d1.coordinates[1].position, p(4));
    assert!(d1.coordinates.iter().all(|c| c.face == QlFace::Direct));

    assert_eq!(d2_left.coordinates.len(), 3);
    assert_eq!(d2_right.coordinates.len(), 3);
    assert!(
        d2_left
            .coordinates
            .iter()
            .any(|c| c.position == p(3) && c.face == QlFace::Conjugate)
    );
    assert!(
        d2_right
            .coordinates
            .iter()
            .any(|c| c.position == p(4) && c.face == QlFace::Conjugate)
    );

    assert_eq!(d3.coordinates.len(), 4);
    assert_eq!(d3.vertex_key(), RelationFamily::B.pair(1).unwrap().d3().vertex_key());
}

#[test]
fn square_apparatus_preserves_nine_entries_eight_orientations_and_seven_tetrads() {
    let fields = all_d3_fields().unwrap();
    assert_eq!(fields.len(), 9);

    let oriented = fields
        .iter()
        .map(|field| {
            let (left, right) = field.pair.positions();
            format!("{}>{}", left.value(), right.value())
        })
        .collect::<HashSet<_>>();
    let unordered = fields
        .iter()
        .map(|field| {
            let mut coordinates = field
                .coordinates
                .iter()
                .map(|coordinate| {
                    format!("{}:{}", coordinate.position.value(), coordinate.face.as_str())
                })
                .collect::<Vec<_>>();
            coordinates.sort();
            coordinates.join("|")
        })
        .collect::<HashSet<_>>();

    assert_eq!(oriented.len(), 8);
    assert_eq!(unordered.len(), 7);

    let a2 = RelationFamily::A.pair(1).unwrap().d3();
    let c3 = RelationFamily::C.pair(2).unwrap().d3();
    assert_eq!(a2.vertex_key(), c3.vertex_key());
    assert_ne!(a2.operator_ref(), c3.operator_ref());

    let b3 = RelationFamily::B.pair(2).unwrap().d3();
    let c1 = RelationFamily::C.pair(0).unwrap().d3();
    assert_eq!(b3.vertex_key(), c1.vertex_key());
    assert_ne!(b3.pair.positions(), c1.pair.positions());
}

#[test]
fn canonical_cross_pass_d1_is_same_position_conjugation() {
    let cross = canonical_cross_pass_d1(p(4));
    match &cross {
        CanonicalCrossPass::D1 {
            position,
            coordinates,
        } => {
            assert_eq!(*position, p(4));
            assert_eq!(coordinates[0].position, p(4));
            assert_eq!(coordinates[0].face, QlFace::Direct);
            assert_eq!(coordinates[1].position, p(4));
            assert_eq!(coordinates[1].face, QlFace::Conjugate);
        }
        other => panic!("expected D1, got {other:?}"),
    }
    assert_eq!(
        cross.operator_ref(),
        "ql:pairing:1.0.0:cross:D1:position-4"
    );
}

#[test]
fn canonical_cross_pass_d2_transform_require_complete_are_exact() {
    for (kind, position, expected_conjugate) in [
        (D2CrossPassKind::Transform, 5, 0),
        (D2CrossPassKind::Require, 0, 5),
        (D2CrossPassKind::Complete, 2, 3),
    ] {
        let cross = canonical_cross_pass_d2(kind, p(position));
        match cross {
            CanonicalCrossPass::D2 {
                kind: actual_kind,
                position: actual_position,
                coordinates,
            } => {
                assert_eq!(actual_kind, kind);
                assert_eq!(actual_position, p(position));
                assert_eq!(coordinates[0].position, p(position));
                assert_eq!(coordinates[0].face, QlFace::Direct);
                assert_eq!(coordinates[1].position, p(expected_conjugate));
                assert_eq!(coordinates[1].face, QlFace::Conjugate);
            }
            other => panic!("expected D2, got {other:?}"),
        }
    }
}

#[test]
fn canonical_cross_pass_d3_reproduces_each_family_on_conjugate_face() {
    let cross = canonical_cross_pass_d3(RelationFamily::A);
    match &cross {
        CanonicalCrossPass::D3 { family, pairs } => {
            assert_eq!(*family, RelationFamily::A);
            assert_eq!(pairs.len(), 3);
            for (index, coordinates) in pairs.iter().enumerate() {
                let (left, right) = RelationFamily::A.pairs()[index];
                assert_eq!(coordinates[0].position, p(left));
                assert_eq!(coordinates[1].position, p(right));
                assert_eq!(coordinates[0].face, QlFace::Conjugate);
                assert_eq!(coordinates[1].face, QlFace::Conjugate);
            }
        }
        other => panic!("expected D3, got {other:?}"),
    }
    assert_eq!(cross.operator_ref(), "ql:pairing:1.0.0:cross:D3:A");
}

#[test]
fn ambiguous_modulation_requests_fail_instead_of_inventing_semantics() {
    assert!(matches!(
        build_d_modulation_frame(
            RelationFamily::A,
            0,
            ConjugationDegree::D2,
            None,
        ),
        Err(PairingError::MissingD2ProjectionSide)
    ));
    assert!(matches!(
        build_d_modulation_frame(
            RelationFamily::A,
            0,
            ConjugationDegree::D1,
            Some(ExpansionSide::Left),
        ),
        Err(PairingError::UnexpectedProjectionSide(ConjugationDegree::D1))
    ));
    assert!(matches!(
        build_d_modulation_frame(
            RelationFamily::A,
            0,
            ConjugationDegree::D3,
            Some(ExpansionSide::Right),
        ),
        Err(PairingError::UnexpectedProjectionSide(ConjugationDegree::D3))
    ));
}
