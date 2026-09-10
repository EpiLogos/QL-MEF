use ql_core::{
    AnchorReturn, CallerProvenance, CarrierError, ConstellationGrain, GroundKind, QlFace,
    QlPosition, RELATION_FIELD_COMPOSITION_OPERATOR_REF, RelationFieldComposition,
    SIX_BY_SIX_SHAPE_REF, STRUCTURAL_CARRIER_CONTRACT_VERSION, ShapeBinding, ShapeRelationBinding,
    StructuralConstellation, StructuralParticipation, WHOLE_ANCHOR_SYMBOL,
};

fn p(value: u8) -> QlPosition {
    QlPosition::new(value).expect("fixture positions are canonical 0..5")
}

fn member(prefix: &str, position: u8, face: QlFace) -> StructuralParticipation {
    StructuralParticipation::new(
        format!("{prefix}:{position}:{}", face.as_str()),
        p(position),
        face,
    )
    .expect("fixture member has a stable ref")
}

fn whole(
    anchor_ref: &str,
    prefix: &str,
    direct: &[u8],
    conjugate: &[u8],
) -> StructuralConstellation {
    let mut members = direct
        .iter()
        .map(|value| member(prefix, *value, QlFace::Direct))
        .collect::<Vec<_>>();
    members.extend(
        conjugate
            .iter()
            .map(|value| member(prefix, *value, QlFace::Conjugate)),
    );
    StructuralConstellation::new(anchor_ref, members, vec![]).expect("valid disclosed whole")
}

#[test]
fn relation_fields_derive_cardinality_from_disclosed_wholes() {
    let two = whole("external:whole:two", "external:two", &[0, 1], &[]);
    let three = whole("external:whole:three", "external:three", &[1, 2, 3], &[]);
    let four = whole("external:whole:four", "external:four", &[1, 2, 3, 4], &[]);
    let six = whole(
        "external:whole:six",
        "external:six",
        &[0, 1, 2, 3, 4, 5],
        &[],
    );
    let twelve = whole(
        "external:whole:twelve",
        "external:twelve",
        &[0, 1, 2, 3, 4, 5],
        &[0, 1, 2, 3, 4, 5],
    );

    assert_eq!(
        RelationFieldComposition::compose(&two, &two)
            .unwrap()
            .cardinality(),
        (2, 2, 4)
    );
    assert_eq!(
        RelationFieldComposition::compose(&three, &four)
            .unwrap()
            .cardinality(),
        (3, 4, 12)
    );
    assert_eq!(
        RelationFieldComposition::compose(&four, &four)
            .unwrap()
            .cardinality(),
        (4, 4, 16)
    );
    assert_eq!(
        RelationFieldComposition::compose(&six, &six)
            .unwrap()
            .cardinality(),
        (6, 6, 36)
    );
    assert_eq!(
        RelationFieldComposition::compose(&six, &twelve)
            .unwrap()
            .cardinality(),
        (6, 12, 72)
    );
    assert_eq!(
        RelationFieldComposition::compose(&twelve, &twelve)
            .unwrap()
            .cardinality(),
        (12, 12, 144)
    );
}

#[test]
fn undisclosed_other_grain_is_not_promoted_by_dimension() {
    let arbitrary_three = whole(
        "external:whole:arbitrary-three",
        "external:arbitrary-three",
        &[0, 2, 5],
        &[],
    );
    let four = whole("external:whole:four", "external:four", &[1, 2, 3, 4], &[]);

    assert_eq!(
        arbitrary_three.grain(),
        ConstellationGrain::Other {
            direct: 3,
            conjugate: 0
        }
    );
    assert!(matches!(
        RelationFieldComposition::compose(&arbitrary_three, &four),
        Err(CarrierError::UndisclosedAxisShape {
            axis: "row",
            grain: ConstellationGrain::Other {
                direct: 3,
                conjugate: 0
            }
        })
    ));
}

#[test]
fn generic_six_by_six_cannot_impersonate_canonical_direct_conjugate_field() {
    let six_a = whole("external:whole:a", "external:a", &[0, 1, 2, 3, 4, 5], &[]);
    let six_b = whole("external:whole:b", "external:b", &[0, 1, 2, 3, 4, 5], &[]);
    let generic = RelationFieldComposition::compose(&six_a, &six_b).unwrap();

    assert_eq!(generic.cardinality(), (6, 6, 36));
    assert_ne!(generic.shape_ref(), SIX_BY_SIX_SHAPE_REF);
    assert_eq!(
        generic.operator_ref(),
        RELATION_FIELD_COMPOSITION_OPERATOR_REF
    );
}

#[test]
fn addresses_are_deterministic_under_external_member_rename_and_reordering() {
    let row_a = StructuralConstellation::new(
        "external:whole:row",
        vec![
            member("external:old", 2, QlFace::Direct),
            member("external:old", 0, QlFace::Direct),
            member("external:old", 1, QlFace::Direct),
        ],
        vec![],
    )
    .unwrap();
    let row_b = StructuralConstellation::new(
        "external:whole:row",
        vec![
            member("external:renamed", 1, QlFace::Direct),
            member("external:renamed", 2, QlFace::Direct),
            member("external:renamed", 0, QlFace::Direct),
        ],
        vec![],
    )
    .unwrap();
    let column = whole(
        "external:whole:column",
        "external:column",
        &[1, 2, 3, 4],
        &[],
    );

    let before = RelationFieldComposition::compose(&row_a, &column).unwrap();
    let after = RelationFieldComposition::compose(&row_b, &column).unwrap();

    assert_eq!(before.shape_ref(), after.shape_ref());
    assert_eq!(before.addresses, after.addresses);
    assert_ne!(before.row_axis.basis_refs(), after.row_axis.basis_refs());
}

#[test]
fn semantic_relations_are_sparse_plural_asymmetric_and_caller_attributable() {
    let row = whole("external:whole:row", "external:row", &[0, 1], &[]);
    let column = whole("external:whole:column", "external:column", &[0, 1], &[]);
    let field = RelationFieldComposition::compose(&row, &column).unwrap();
    let address = field.addresses[1];
    let first = ShapeRelationBinding::new(
        address,
        "external:relation:first",
        vec!["external:evidence:one".into()],
    )
    .unwrap();
    let second = ShapeRelationBinding::new(
        address,
        "external:relation:second",
        vec!["external:evidence:two".into()],
    )
    .unwrap();
    let provenance = CallerProvenance::new(
        "external:caller:agent",
        "external:source:run",
        "external:standing:observed",
    )
    .unwrap();
    let binding = ShapeBinding::new(
        "external:subject:item",
        field.shape_ref(),
        "external:whole:reading",
        [row.members.as_slice(), column.members.as_slice()]
            .concat()
            .into_iter()
            .map(|value| value.subject_ref)
            .collect(),
        vec![],
        vec![first, second],
        Some("external:derivation:reading".into()),
        Some(field.operator_ref().into()),
        vec![],
        provenance,
    )
    .unwrap();

    binding.validate_relation_field(&field).unwrap();
    assert_eq!(field.addresses.len(), 4);
    assert_eq!(binding.relation_bindings.len(), 2);
    assert_eq!(
        binding.relation_bindings[0].address,
        binding.relation_bindings[1].address
    );
    assert_eq!(binding.member_bindings().len(), 0);
    assert_eq!(
        binding.caller_provenance().standing_ref,
        "external:standing:observed"
    );
    assert_ne!(address.row, address.column);
}

#[test]
fn a_semantic_binding_requires_evidence_and_must_land_inside_the_field() {
    let row = whole("external:whole:row", "external:row", &[0, 1], &[]);
    let column = whole("external:whole:column", "external:column", &[0, 1], &[]);
    let field = RelationFieldComposition::compose(&row, &column).unwrap();

    let no_evidence =
        ShapeRelationBinding::new(field.addresses[0], "external:relation:unsupported", vec![]);
    assert_eq!(no_evidence.unwrap_err(), CarrierError::MissingEvidence);

    let outside = ql_core::QlShapeAddress {
        row: ql_core::QlCoordinate::new(p(5), QlFace::Direct),
        column: ql_core::QlCoordinate::new(p(5), QlFace::Direct),
    };
    let relation = ShapeRelationBinding::new(
        outside,
        "external:relation:outside",
        vec!["external:evidence:outside".into()],
    )
    .unwrap();
    let binding = ShapeBinding::new(
        "external:subject:item",
        field.shape_ref(),
        "external:whole:reading",
        vec![],
        vec![],
        vec![relation],
        None,
        Some(field.operator_ref().into()),
        vec![],
        CallerProvenance::new(
            "external:caller:agent",
            "external:source:run",
            "external:standing:proposed",
        )
        .unwrap(),
    )
    .unwrap();

    assert!(matches!(
        binding.validate_relation_field(&field),
        Err(CarrierError::AddressOutsideField(_))
    ));
}

#[test]
fn relation_derivation_keeps_source_wholes_grains_and_full_return_routes() {
    let route = AnchorReturn::new(
        "external:row:5:direct",
        "external:whole:row",
        "external:ground:row",
        QlFace::Direct,
        GroundKind::Own,
    )
    .unwrap();
    let row = StructuralConstellation::new(
        "external:whole:row",
        (0..6)
            .map(|value| member("external:row", value, QlFace::Direct))
            .collect(),
        vec![route],
    )
    .unwrap();
    let column = whole(
        "external:whole:column",
        "external:column",
        &[1, 2, 3, 4],
        &[],
    );
    let field = RelationFieldComposition::compose(&row, &column).unwrap();
    let derivation = field.derivation();

    assert_eq!(
        derivation.source_whole_refs,
        ["external:whole:row", "external:whole:column"]
    );
    assert_eq!(derivation.source_shape_refs[0], field.row_axis.shape_ref());
    assert_eq!(
        derivation.source_shape_refs[1],
        field.column_axis.shape_ref()
    );
    assert_eq!(
        derivation.operator_ref,
        RELATION_FIELD_COMPOSITION_OPERATOR_REF
    );
    assert_eq!(derivation.generated_shape_ref, field.shape_ref());
    assert_eq!(derivation.return_basis, WHOLE_ANCHOR_SYMBOL);
    assert_eq!(derivation.source_return_refs.len(), 1);
    assert_eq!(derivation.source_returns.len(), 1);
    assert_eq!(
        derivation.source_returns[0].through_anchor_ref,
        "external:whole:row"
    );
    assert_eq!(
        derivation.source_returns[0].target_ground_ref,
        "external:ground:row"
    );
}

#[test]
fn anchor_only_whole_is_valid_but_not_fabricated_into_a_positional_axis() {
    let anchor = StructuralConstellation::new("external:whole:anchor", vec![], vec![]).unwrap();
    let pair = whole("external:whole:pair", "external:pair", &[0, 1], &[]);

    assert!(matches!(
        RelationFieldComposition::compose(&anchor, &pair),
        Err(CarrierError::EmptyAxis("row"))
    ));
}

#[test]
fn portable_fixture_freezes_only_structural_law() {
    let fixture = include_str!("../../../fixtures/kernel/ql-structural-carrier-contract-v1.json");

    assert!(fixture.contains("\"contract_id\": \"ql.structural-carrier\""));
    assert!(fixture.contains("\"version\": \"1.0.0\""));
    assert!(fixture.contains("\"dimensions_are_shape_authority\": false"));
    assert!(fixture.contains("\"undisclosed_other_grains_are_carrier_axes\": false"));
    assert!(fixture.contains("\"semantic_cells_generated\": false"));
    assert!(fixture.contains("\"missing_cells_valid\": true"));
    assert!(fixture.contains("\"plural_bindings_per_address_valid\": true"));
    assert!(fixture.contains("\"retains_source_return_routes\": true"));
    assert!(fixture.contains("\"return_basis\": \"0/1\""));
    assert!(fixture.contains("\"external_label_rename_changes_structure\": false"));
    assert_eq!(STRUCTURAL_CARRIER_CONTRACT_VERSION, "1.0.0");
}
