use ql_core::{
    AnchorReturn, ConstellationGrain, ExpansionSide, GroundKind, QlFace, QlPosition,
    RelationFamily, STRUCTURAL_CONTRACT_VERSION, StructuralConstellation,
    StructuralParticipation, WHOLE_ANCHOR_SYMBOL, all_d3_fields,
};

fn p(value: u8) -> QlPosition {
    QlPosition::new(value).expect("fixture positions are 0..5")
}

fn member(position: u8, face: QlFace) -> StructuralParticipation {
    StructuralParticipation::new(format!("wiki:node:{position}:{}", face.as_str()), p(position), face)
        .expect("valid fixture member")
}

#[test]
fn relation_families_are_exact_and_pair_index_is_identity() {
    assert_eq!(RelationFamily::A.pairs(), [(0, 1), (2, 3), (4, 5)]);
    assert_eq!(RelationFamily::B.pairs(), [(1, 2), (3, 4), (5, 0)]);
    assert_eq!(RelationFamily::C.pairs(), [(0, 5), (1, 4), (2, 3)]);

    let a_mid = RelationFamily::A.pair(1).unwrap();
    let c_mid = RelationFamily::C.pair(2).unwrap();
    assert_eq!(a_mid.positions(), c_mid.positions());
    assert_ne!(a_mid.operator_ref(), c_mid.operator_ref());
}

#[test]
fn d1_is_independent_same_position_conjugation_axis() {
    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        for index in 0..3 {
            let pair = family.pair(index).unwrap();
            let oppositions = pair.d1_oppositions();
            assert_eq!(oppositions[0].coordinates()[0].position, pair.left);
            assert_eq!(oppositions[0].coordinates()[1].position, pair.left);
            assert_eq!(oppositions[0].coordinates()[0].face, QlFace::Direct);
            assert_eq!(oppositions[0].coordinates()[1].face, QlFace::Conjugate);
            assert_eq!(oppositions[1].coordinates()[0].position, pair.right);
            assert!(oppositions[0].operator_ref().contains("conjugation:D1"));
        }
    }
}

#[test]
fn d2_has_two_explicit_one_sided_variants_and_d3_expands_both() {
    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        for index in 0..3 {
            let pair = family.pair(index).unwrap();
            let left = pair.d2(ExpansionSide::Left);
            let right = pair.d2(ExpansionSide::Right);
            let d3 = pair.d3();
            assert_eq!(left.coordinates.len(), 3);
            assert_eq!(right.coordinates.len(), 3);
            assert_eq!(d3.coordinates.len(), 4);
            assert_ne!(left.operator_ref(), right.operator_ref());
            assert!(left
                .coordinates
                .iter()
                .any(|c| c.position == pair.left && c.face == QlFace::Conjugate));
            assert!(!left
                .coordinates
                .iter()
                .any(|c| c.position == pair.right && c.face == QlFace::Conjugate));
            assert!(right
                .coordinates
                .iter()
                .any(|c| c.position == pair.right && c.face == QlFace::Conjugate));
            assert!(d3
                .coordinates
                .iter()
                .any(|c| c.position == pair.left && c.face == QlFace::Conjugate));
            assert!(d3
                .coordinates
                .iter()
                .any(|c| c.position == pair.right && c.face == QlFace::Conjugate));
        }
    }
}

#[test]
fn all_nine_d3_fields_are_addressable_and_same_vertices_do_not_collapse_family() {
    let fields = all_d3_fields().unwrap();
    assert_eq!(fields.len(), 9);
    let refs = fields.iter().map(|field| field.operator_ref()).collect::<std::collections::HashSet<_>>();
    assert_eq!(refs.len(), 9);

    let a2 = RelationFamily::A.pair(1).unwrap().d3();
    let c3 = RelationFamily::C.pair(2).unwrap().d3();
    assert_eq!(a2.vertex_key(), c3.vertex_key());
    assert_ne!(a2.structural_key(), c3.structural_key());
    assert_ne!(a2.operator_ref(), c3.operator_ref());
}

#[test]
fn anchor_is_not_a_seventh_member_and_full_six_and_twelvefold_are_exact() {
    assert_eq!(WHOLE_ANCHOR_SYMBOL, "0/1");
    let six = StructuralConstellation::new(
        "wiki:anchor:whole",
        (0..6).map(|n| member(n, QlFace::Direct)).collect(),
        vec![],
    )
    .unwrap();
    assert_eq!(six.members.len(), 6);
    assert_eq!(six.grain(), ConstellationGrain::SixFold);

    let twelve = StructuralConstellation::new(
        "wiki:anchor:whole",
        (0..6)
            .flat_map(|n| [member(n, QlFace::Direct), member(n, QlFace::Conjugate)])
            .collect(),
        vec![],
    )
    .unwrap();
    assert_eq!(twelve.members.len(), 12);
    assert_eq!(twelve.grain(), ConstellationGrain::TwelveFold);
}

#[test]
fn canonical_grains_and_partial_conjugate_8_to_11_are_formal_not_guessed_semantics() {
    let anchor = StructuralConstellation::new("wiki:anchor:only", vec![], vec![]).unwrap();
    assert_eq!(anchor.grain(), ConstellationGrain::AnchorOnly);

    let triad_123 = StructuralConstellation::new(
        "wiki:anchor:t",
        [1, 2, 3].into_iter().map(|n| member(n, QlFace::Direct)).collect(),
        vec![],
    )
    .unwrap();
    assert_eq!(triad_123.grain(), ConstellationGrain::ThreeFold123);

    let triad_450 = StructuralConstellation::new(
        "wiki:anchor:t",
        [4, 5, 0].into_iter().map(|n| member(n, QlFace::Direct)).collect(),
        vec![],
    )
    .unwrap();
    assert_eq!(triad_450.grain(), ConstellationGrain::ThreeFold450);

    for (conjugates, expected) in [
        (2, ConstellationGrain::PartialConjugate8),
        (3, ConstellationGrain::PartialConjugate9),
        (4, ConstellationGrain::PartialConjugate10),
        (5, ConstellationGrain::PartialConjugate11),
    ] {
        let mut members = (0..6).map(|n| member(n, QlFace::Direct)).collect::<Vec<_>>();
        members.extend((0..conjugates).map(|n| member(n, QlFace::Conjugate)));
        let constellation = StructuralConstellation::new("wiki:anchor:p", members, vec![]).unwrap();
        assert_eq!(constellation.grain(), expected);
        assert_eq!(constellation.direct_positions(), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(constellation.conjugate_positions().len(), conjugates as usize);
    }
}

#[test]
fn return_is_a_two_hop_path_through_anchor_to_explicit_zero_ground() {
    for (kind, target, face) in [
        (GroundKind::Own, "wiki:ground:own", QlFace::Direct),
        (GroundKind::Parent, "wiki:ground:parent", QlFace::Direct),
        (GroundKind::Child, "wiki:ground:child", QlFace::Direct),
        (GroundKind::Other, "wiki:ground:other", QlFace::Direct),
        (GroundKind::Conjugate, "wiki:ground:prime", QlFace::Conjugate),
    ] {
        let route = AnchorReturn::new("wiki:node:p5", "wiki:anchor:whole", target, face, kind).unwrap();
        assert_eq!(route.through_anchor_ref, "wiki:anchor:whole");
        assert_eq!(route.target_ground_position.value(), 0);
        assert_eq!(route.target_face, face);
        assert!(route.operator_ref().contains("return:through-anchor"));
        let constellation = StructuralConstellation::new(
            "wiki:anchor:whole",
            vec![member(5, QlFace::Direct)],
            vec![route],
        )
        .unwrap();
        assert_eq!(constellation.returns.len(), 1);
    }
}

#[test]
fn structural_operator_refs_are_independently_versioned() {
    assert_eq!(STRUCTURAL_CONTRACT_VERSION, "2.0.0");
    let d3 = RelationFamily::B.pair(2).unwrap().d3();
    assert!(d3.operator_ref().starts_with("ql:structural:2.0.0:"));
    assert!(!d3.operator_ref().contains("transform"));
    assert!(!d3.operator_ref().contains("require"));
    assert!(!d3.operator_ref().contains("complete"));
}
