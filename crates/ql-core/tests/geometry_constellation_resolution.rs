use ql_core::{
    COMPRESS_TO_WHOLE_OPERATOR_REF, CallerProvenance, ConstellationGrain,
    EIGHTEEN_TO_THREE_OPERATOR_REF, EighteenFoldGeometry, FourByFourByFourField, QlFace,
    QlPosition, QlShape, QlShapeCompression, Recognition, RecognitionError, RecognitionRoute,
    RelationFamily, SecondSpandaGeometry, ShapeBinding, SixfoldRoute, StructuralConstellation,
    StructuralParticipation, THREE_TO_ONE_OPERATOR_REF,
};

#[test]
fn geometry_reads_the_existing_anchor_as_onefold_and_completes_the_sevenfold_gap() {
    assert_eq!(QlShape::onefold().fold_count(), Some(1));
    assert_eq!(QlShape::canonical_sevenfold().fold_count(), Some(7));
    assert!(
        QlShape::canonical_sevenfold()
            .shape_ref()
            .contains("partial-conjugate-7")
    );
}

#[test]
fn three_to_one_compression_preserves_the_disclosed_basis_in_the_existing_binding_contract() {
    let compression =
        QlShapeCompression::to_onefold(QlShape::Constellation(ConstellationGrain::ThreeFold123))
            .expect("canonical threefold compresses to 0/1");
    assert_eq!(compression.presented, QlShape::onefold());
    assert_eq!(compression.disclosed_fold, 3);
    assert_eq!(compression.operator_ref, THREE_TO_ONE_OPERATOR_REF);
    assert_eq!(
        compression.recognition_superset,
        Some(QlShape::Constellation(ConstellationGrain::FourFold1234))
    );

    let derivation_ref = compression.derivation_ref();
    let binding = ShapeBinding::new(
        "wiki:node:recognised-whole",
        compression.presented.shape_ref(),
        "wiki:anchor:recognised-whole",
        vec![
            "wiki:node:a".into(),
            "wiki:node:b".into(),
            "wiki:node:c".into(),
        ],
        Vec::new(),
        Vec::new(),
        Some(derivation_ref.clone()),
        Some(compression.operator_ref.into()),
        Vec::new(),
        CallerProvenance::new("wiki:test", "wiki:source", "authored").unwrap(),
    )
    .unwrap();

    assert_eq!(binding.shape_ref, QlShape::onefold().shape_ref());
    assert_eq!(
        binding.derivation_ref.as_deref(),
        Some(derivation_ref.as_str())
    );
    assert_eq!(
        binding.operator_ref.as_deref(),
        Some(THREE_TO_ONE_OPERATOR_REF)
    );
    assert_eq!(binding.basis_refs.len(), 3);
}

#[test]
fn twofold_can_also_be_carried_as_the_basis_of_a_presented_onefold() {
    let compression =
        QlShapeCompression::to_onefold(QlShape::Constellation(ConstellationGrain::TwoFold))
            .expect("canonical twofold compresses to 0/1");
    assert_eq!(compression.disclosed_fold, 2);
    assert_eq!(compression.operator_ref, COMPRESS_TO_WHOLE_OPERATOR_REF);
}

#[test]
fn m3_four_cubed_is_the_existing_sixty_four_state_fold_body() {
    let m3 = FourByFourByFourField::canonical();
    assert_eq!(m3.site_cardinality, 3);
    assert_eq!(m3.states_per_site, 4);
    assert_eq!(m3.binary_properties_per_site, 2);
    assert_eq!(m3.address_cardinality, 64);
}

#[test]
fn second_spanda_and_its_decadic_projection_preserve_the_same_sixty_four_thirty_six_partition() {
    let geometry = SecondSpandaGeometry::canonical();
    assert_eq!(geometry.m3_quaternary_cubic.address_cardinality, 64);
    assert_eq!(geometry.m2_senary_square.addresses.len(), 36);
    assert_eq!(geometry.totality(), 100);
    assert_eq!(
        geometry.decadic_projection.block_cardinalities,
        [16, 24, 24, 36]
    );
    assert_eq!(geometry.decadic_projection.m3_partition_cardinality, 64);
    assert_eq!(geometry.decadic_projection.m2_partition_cardinality, 36);
    assert_eq!(geometry.decadic_projection.address_cardinality, 100);
}

#[test]
fn eighteenfold_is_the_three_sixfold_geometry_and_compresses_by_six_to_three() {
    let geometry = EighteenFoldGeometry::canonical();
    assert_eq!(geometry.sixfold_cardinality, 6);
    assert_eq!(geometry.fold_cardinality, 18);
    assert_eq!(geometry.compressed_fold_cardinality, 3);
    assert_eq!(
        geometry.compression_operator_ref(),
        EIGHTEEN_TO_THREE_OPERATOR_REF
    );
}

fn constellation(direct: &[u8], conjugate: &[u8]) -> StructuralConstellation {
    let member = |p: u8, face| {
        StructuralParticipation::new(format!("t:{p}:{face:?}"), QlPosition::new(p).unwrap(), face)
            .unwrap()
    };
    let members = direct
        .iter()
        .map(|p| member(*p, QlFace::Direct))
        .chain(conjugate.iter().map(|p| member(*p, QlFace::Conjugate)))
        .collect();
    StructuralConstellation::new("anchor:whole", members, Vec::new()).unwrap()
}

#[test]
fn every_whole_of_ql_is_recognised_by_its_own_route() {
    let all: Vec<u8> = (0..6).collect();
    let cases: [(&[u8], &[u8], RecognitionRoute); 9] = [
        (&[2, 5], &[], RecognitionRoute::Direct),
        (&[1, 2, 3], &[], RecognitionRoute::Direct),
        (&[4, 5, 0], &[], RecognitionRoute::Direct),
        (&[1, 2, 3, 4], &[], RecognitionRoute::ThreePlusOne),
        (
            &[0, 1, 2, 3, 4],
            &[],
            RecognitionRoute::FourPlusOne { with: 0 },
        ),
        (
            &[1, 2, 3, 4, 5],
            &[],
            RecognitionRoute::FourPlusOne { with: 5 },
        ),
        (
            &all,
            &[],
            RecognitionRoute::Sixfold(SixfoldRoute::FourPlusTwo),
        ),
        (
            &all,
            &[1, 3],
            RecognitionRoute::SixfoldOpen {
                route: SixfoldRoute::FourPlusTwo,
                open: vec![1, 3],
            },
        ),
        (
            &all,
            &all,
            RecognitionRoute::Eighteen(SixfoldRoute::FourPlusTwo),
        ),
    ];
    for (direct, conjugate, route) in cases {
        let form = constellation(direct, conjugate);
        let recognition = Recognition::of(&form, None).unwrap();
        assert_eq!(recognition.route, route, "{direct:?} {conjugate:?}");
        assert_eq!(recognition.presented(), QlShape::onefold());
        // A presented binding's operator names exactly this recognition again.
        assert_eq!(
            Recognition::from_operator(&form, &recognition.operator_ref()),
            Some(recognition.clone())
        );
    }
    // The direct routes keep the existing 2 -> 1 / 3 -> 1 identity Wiki uses.
    let triad = Recognition::of(&constellation(&[1, 2, 3], &[]), None).unwrap();
    let existing =
        QlShapeCompression::to_onefold(QlShape::Constellation(ConstellationGrain::ThreeFold123))
            .unwrap();
    assert_eq!(triad.operator_ref(), THREE_TO_ONE_OPERATOR_REF);
    assert_eq!(triad.derivation_ref(), existing.derivation_ref());
}

#[test]
fn the_sixfold_is_four_plus_two_by_the_kernels_own_operator_and_its_readings_stay_distinct() {
    assert_eq!(
        SixfoldRoute::FourPlusTwo.parts(),
        vec![vec![1, 2, 3, 4], vec![5, 0]]
    );
    assert_eq!(
        SixfoldRoute::ThreePlusThree.parts(),
        vec![vec![1, 2, 3], vec![4, 5, 0]]
    );
    assert_eq!(
        SixfoldRoute::Pairs(RelationFamily::B).parts(),
        vec![vec![1, 2], vec![3, 4], vec![5, 0]]
    );
    let six = constellation(&[0, 1, 2, 3, 4, 5], &[]);
    let operators: std::collections::BTreeSet<String> = SixfoldRoute::ALL
        .into_iter()
        .map(|route| Recognition::of(&six, Some(route)).unwrap().operator_ref())
        .collect();
    assert_eq!(operators.len(), 5);
    let backbone = Recognition::of(&six, None).unwrap();
    assert!(
        backbone
            .derivation_ref()
            .ends_with(":parts:[1,2,3,4]+[5,0]")
    );
    for route in SixfoldRoute::ALL {
        assert_eq!(SixfoldRoute::parse(route.code()), Some(route));
    }
}

#[test]
fn all_twelve_recognise_through_their_three_sixfolds_and_partial_nights_stay_open() {
    let all = [0, 1, 2, 3, 4, 5];
    let twelve = Recognition::of(
        &constellation(&all, &all),
        Some(SixfoldRoute::ThreePlusThree),
    )
    .unwrap();
    assert_eq!(twelve.disclosed, QlShape::EighteenFold);
    let derivation = twelve.derivation_ref();
    assert!(
        derivation.contains(EIGHTEEN_TO_THREE_OPERATOR_REF),
        "{derivation}"
    );
    assert!(derivation.contains("direct+conjugate+relational"));
    assert!(derivation.ends_with("each:[1,2,3]+[4,5,0]"));
    let nine = Recognition::of(&constellation(&all, &[0, 2, 4]), None).unwrap();
    assert!(nine.derivation_ref().ends_with(":open-D1:[0,2,4]"));
}

#[test]
fn what_is_not_a_whole_is_told_to_recognise_its_parts() {
    let partial = constellation(&[1, 2, 5], &[]);
    let error = Recognition::of(&partial, None).unwrap_err();
    assert!(matches!(error, RecognitionError::NotAWhole(_)));
    assert!(error.to_string().contains("recognise its parts"));
    assert_eq!(
        Recognition::of(&constellation(&[], &[]), None),
        Err(RecognitionError::AlreadyWhole)
    );
    assert!(matches!(
        Recognition::of(
            &constellation(&[1, 2, 3], &[]),
            Some(SixfoldRoute::FourPlusTwo)
        ),
        Err(RecognitionError::RouteWithoutSixfold(_))
    ));
}
