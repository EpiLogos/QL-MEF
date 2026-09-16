use ql_core::{
    CallerProvenance, ConstellationGrain, QlCoordinate, QlFace, QlFamily, QlPosition, QlShape,
    QlShapeCompression, ShapeBinding, StructuralConstellation, StructuralParticipation,
};
use ql_mef::vak_composition::{
    ActiveFrame, Basis, MemberFocus, PositionBasis, SelectedAddress, VakComposition, WholeInput,
};
use ql_mef::{ContextFrameId, LensId, MusicalBasis, VakRegistry};

fn provenance() -> CallerProvenance {
    CallerProvenance::new("wiki:test", "wiki:source", "AUTHORED-ARCHITECTURE").unwrap()
}

fn basis() -> Basis {
    Basis {
        provenance: provenance(),
        revision: "test-revision".into(),
        evidence: vec!["test:evidence".into()],
    }
}

fn member(reference: &str, position: u8) -> StructuralParticipation {
    StructuralParticipation::new(
        reference,
        QlPosition::new(position).unwrap(),
        QlFace::Direct,
    )
    .unwrap()
}

#[test]
fn compressed_threefold_presents_as_onefold_and_reopens_through_member_focus() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let members = vec![
        member("wiki:a", 1),
        member("wiki:b", 2),
        member("wiki:c", 3),
    ];
    let form = StructuralConstellation::new("wiki:whole", members.clone(), Vec::new()).unwrap();
    assert_eq!(form.grain(), ConstellationGrain::ThreeFold123);

    let compression = QlShapeCompression::to_onefold(QlShape::Constellation(form.grain())).unwrap();
    let binding = ShapeBinding::new(
        "wiki:subject",
        compression.presented.shape_ref(),
        "wiki:whole",
        members
            .iter()
            .map(|member| member.subject_ref.clone())
            .collect(),
        members,
        Vec::new(),
        Some(compression.derivation_ref()),
        Some(compression.operator_ref.into()),
        Vec::new(),
        provenance(),
    )
    .unwrap();

    let frame = ActiveFrame {
        id: ContextFrameId::ALL[1],
        lens: LensId::L0,
        basis: MusicalBasis::Chromatic,
        face: QlFace::Direct,
        positions: PositionBasis::Local,
    };
    let mut composition = VakComposition::default();
    composition
        .bind_whole(
            &registry,
            WholeInput {
                use_ref: "use:compressed".into(),
                form,
                binding,
                category: QlFamily::C,
                ground_ref: "wiki:ground".into(),
                ground_face: QlFace::Direct,
                frame,
                basis: basis(),
                language: None,
            },
        )
        .unwrap();

    let collapsed = composition.read("use:compressed", LensId::L0).unwrap();
    assert!(matches!(collapsed.address, SelectedAddress::Anchor(_)));
    assert_eq!(collapsed.geometry.shape_ref, QlShape::onefold().shape_ref());
    assert_eq!(collapsed.binding.basis_refs.len(), 3);
    assert_eq!(
        collapsed.binding.operator_ref.as_deref(),
        Some(compression.operator_ref)
    );

    composition
        .position_member(
            "use:compressed",
            "use:opened",
            MemberFocus {
                coordinate: QlCoordinate::new(QlPosition::new(1).unwrap(), QlFace::Direct),
                positions: PositionBasis::Local,
                basis: basis(),
            },
        )
        .unwrap();
    let opened = composition.read("use:opened", LensId::L0).unwrap();
    assert!(matches!(opened.address, SelectedAddress::Member { .. }));
    assert_eq!(opened.geometry.shape_ref, QlShape::onefold().shape_ref());
}
