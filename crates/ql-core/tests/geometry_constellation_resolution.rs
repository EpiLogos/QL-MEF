use ql_core::{
    COMPRESS_TO_WHOLE_OPERATOR_REF, CallerProvenance, ConstellationGrain,
    EIGHTEEN_TO_THREE_OPERATOR_REF, EighteenFoldGeometry, FourByFourByFourField, QlShape,
    QlShapeCompression, SecondSpandaGeometry, ShapeBinding, THREE_TO_ONE_OPERATOR_REF,
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
