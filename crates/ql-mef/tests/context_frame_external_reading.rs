use ql_mef::{
    read_external_context_frame, ContextFrameId, ContextFrameReadingOrigin,
    ContextFrameReadingStatus, ContextFrameStructuralProbe, ExternalSixfoldMapping, MefGrain,
    MefUnitFace,
};

fn generic_mapping(names: [&str; 6]) -> ExternalSixfoldMapping {
    ExternalSixfoldMapping::new(
        "target/external-sixfold",
        "source/external-sixfold/v1",
        names.map(str::to_string),
    )
    .unwrap()
}

#[test]
fn exact_structural_evidence_yields_an_exact_named_frame_without_product_semantics() {
    let mapping = generic_mapping(["a", "b", "c", "d", "e", "f"]);
    let probe = ContextFrameStructuralProbe::new(
        3,
        Some(MefUnitFace::Power),
        Some(MefGrain::InnerFour),
    )
    .unwrap();
    let reading = read_external_context_frame(
        &mapping,
        probe,
        "provider/ql-mef",
        ContextFrameReadingOrigin::Derived,
        vec!["evidence/external-structure/1".to_string()],
    )
    .unwrap();

    assert_eq!(reading.status(), &ContextFrameReadingStatus::Exact(ContextFrameId::Cf5));
    assert!(!reading.is_runtime_authority());
}

#[test]
fn renaming_external_members_does_not_change_the_structural_reading() {
    let first = generic_mapping(["alpha", "beta", "gamma", "delta", "epsilon", "zeta"]);
    let second = ExternalSixfoldMapping::new(
        "target/external-sixfold",
        "source/external-sixfold/v2",
        ["one", "two", "three", "four", "five", "six"].map(str::to_string),
    )
    .unwrap();
    let probe = ContextFrameStructuralProbe::new(
        5,
        Some(MefUnitFace::Power),
        Some(MefGrain::OuterTwo),
    )
    .unwrap();

    let a = read_external_context_frame(
        &first,
        probe,
        "provider/ql-mef",
        ContextFrameReadingOrigin::Derived,
        vec![],
    )
    .unwrap();
    let b = read_external_context_frame(
        &second,
        probe,
        "provider/ql-mef",
        ContextFrameReadingOrigin::Derived,
        vec![],
    )
    .unwrap();

    assert_eq!(a.status(), b.status());
    assert_ne!(first.mapping_digest(), second.mapping_digest());
}

#[test]
fn incomplete_evidence_remains_partial_or_ambiguous_instead_of_being_forced() {
    let mapping = generic_mapping(["p0", "p1", "p2", "p3", "p4", "p5"]);

    let partial = read_external_context_frame(
        &mapping,
        ContextFrameStructuralProbe::new(4, None, None).unwrap(),
        "provider/ql-mef",
        ContextFrameReadingOrigin::Proposed,
        vec![],
    )
    .unwrap();
    assert_eq!(
        partial.status(),
        &ContextFrameReadingStatus::Partial {
            frame: ContextFrameId::Cf6,
            missing_face: true,
            missing_grain: true,
        }
    );

    let ambiguous = read_external_context_frame(
        &mapping,
        ContextFrameStructuralProbe::new(2, None, Some(MefGrain::InnerFour)).unwrap(),
        "provider/ql-mef",
        ContextFrameReadingOrigin::Proposed,
        vec![],
    )
    .unwrap();
    assert_eq!(
        ambiguous.status(),
        &ContextFrameReadingStatus::Ambiguous(vec![ContextFrameId::Cf3, ContextFrameId::Cf4])
    );
}

#[test]
fn incompatible_structural_evidence_returns_no_reading() {
    let mapping = generic_mapping(["p0", "p1", "p2", "p3", "p4", "p5"]);
    let reading = read_external_context_frame(
        &mapping,
        ContextFrameStructuralProbe::new(
            0,
            Some(MefUnitFace::Power),
            Some(MefGrain::InnerFour),
        )
        .unwrap(),
        "provider/ql-mef",
        ContextFrameReadingOrigin::Derived,
        vec!["evidence/no-match".to_string()],
    )
    .unwrap();
    assert_eq!(reading.status(), &ContextFrameReadingStatus::NoReading);
}

#[test]
fn an_oi_named_fixture_is_only_external_mapping_data() {
    let mapping = generic_mapping([
        "Central",
        "Actuation",
        "AIKit",
        "SoftwareFactory",
        "Workcell",
        "O:I",
    ]);
    let probe = ContextFrameStructuralProbe::new(
        1,
        Some(MefUnitFace::Name),
        Some(MefGrain::InnerFour),
    )
    .unwrap();
    let reading = read_external_context_frame(
        &mapping,
        probe,
        "provider/ql-mef",
        ContextFrameReadingOrigin::Recognised,
        vec!["evidence/oi-explicit-mapping".to_string()],
    )
    .unwrap();

    assert_eq!(reading.status(), &ContextFrameReadingStatus::Exact(ContextFrameId::Cf2));
    assert_eq!(mapping.external_position_refs()[0], "Central");
    assert!(!reading.is_runtime_authority());
}

#[test]
fn malformed_or_collapsed_sixfold_mappings_are_rejected() {
    assert!(ExternalSixfoldMapping::new(
        "target",
        "source",
        ["a", "a", "c", "d", "e", "f"].map(str::to_string),
    )
    .is_err());
    assert!(ExternalSixfoldMapping::new(
        "target",
        "source",
        ["a", "b", "", "d", "e", "f"].map(str::to_string),
    )
    .is_err());
}
