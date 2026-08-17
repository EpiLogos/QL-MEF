use ql_mef::{
    CONTRACT_SCHEMA_VERSION, ClientRef, InputRefRevision, LensId, LensRef, MEF_REGISTRY_VERSION,
    QlProvenance, QlProviderRef, QlReading, QlRelationReading, QlSynthesis, QlTarget,
    RefractionContract, ResultClass, SublensRef,
};

fn provider() -> QlProviderRef {
    QlProviderRef::new("ql-mef-reference", "0.1.0").expect("provider")
}

#[test]
fn one_opaque_subject_ref_survives_all_twelve_refractions_unchanged() {
    let subject = ClientRef::new("factory:Claim/claim-42").expect("subject");
    let target = QlTarget::new(subject.clone());

    for lens in LensId::ALL {
        let lens_ref = LensRef::canonical(lens);
        let sublens = SublensRef::canonical(lens, 3).expect("sublens");
        let contract = RefractionContract::new(target.clone(), lens_ref, Some(sublens))
            .expect("matching refraction");
        assert_eq!(contract.subject_ref(), &subject);

        let provenance = QlProvenance::new(
            provider(),
            "refract",
            vec![InputRefRevision::new(subject.clone(), Some("rev-7".into()))],
            ResultClass::SemanticStochastic,
        );
        let reading_id = ClientRef::new(format!("reading:{}", lens.code())).expect("reading id");
        let reading = QlReading::new(
            reading_id,
            target.clone(),
            Some(lens_ref),
            "derived",
            provenance,
        );
        assert_eq!(reading.target.subject, subject);
    }
}

#[test]
fn mismatched_sublens_is_rejected_instead_of_coerced() {
    let target = QlTarget::new(ClientRef::new("client:subject/1").expect("subject"));
    let error = RefractionContract::new(
        target,
        LensRef::canonical(LensId::L1),
        Some(SublensRef::canonical(LensId::L4, 0).expect("sublens")),
    )
    .expect_err("mismatch must fail");
    assert_eq!(error.code(), "SUBLENS_LENS_MISMATCH");
}

#[test]
fn provenance_records_version_provider_operation_input_and_result_class() {
    let subject = ClientRef::new("client:artifact/a").expect("subject");
    let provenance = QlProvenance::new(
        provider(),
        "locate",
        vec![InputRefRevision::new(
            subject.clone(),
            Some("sha256:abc".into()),
        )],
        ResultClass::Deterministic,
    );
    assert_eq!(provenance.schema_version, CONTRACT_SCHEMA_VERSION);
    assert_eq!(provenance.mef_registry_version, MEF_REGISTRY_VERSION);
    assert_eq!(provenance.provider.provider, "ql-mef-reference");
    assert_eq!(provenance.provider.version, "0.1.0");
    assert_eq!(provenance.operation, "locate");
    assert_eq!(provenance.input_refs[0].reference, subject);
    assert_eq!(provenance.result_class, ResultClass::Deterministic);
}

#[test]
fn result_classes_are_explicit_and_round_trip() {
    for class in [
        ResultClass::Canonical,
        ResultClass::Deterministic,
        ResultClass::SemanticStochastic,
        ResultClass::Research,
    ] {
        assert_eq!(
            class
                .to_string()
                .parse::<ResultClass>()
                .expect("round trip"),
            class
        );
    }
    assert!("semantic".parse::<ResultClass>().is_err());
}

#[test]
fn relation_and_synthesis_preserve_source_identity_and_unresolved_difference() {
    let left = ClientRef::new("client:left").expect("left");
    let right = ClientRef::new("client:right").expect("right");
    let provenance = QlProvenance::new(
        provider(),
        "relate",
        vec![
            InputRefRevision::new(left.clone(), None),
            InputRefRevision::new(right.clone(), None),
        ],
        ResultClass::SemanticStochastic,
    );
    let relation = QlRelationReading {
        id: ClientRef::new("reading:relation/1").expect("id"),
        subjects: vec![left.clone(), right.clone()],
        frame: None,
        relation: "tension retained",
        addresses: vec![],
        lenses: vec![LensRef::canonical(LensId::L2)],
        evidence_refs: vec![],
        provenance: provenance.clone(),
    };
    assert_eq!(relation.subjects, vec![left, right]);

    let synthesis = QlSynthesis {
        id: ClientRef::new("reading:synthesis/1").expect("id"),
        input_readings: vec![relation.id.clone()],
        synthesis: "partial integration",
        retained_differences: vec!["different causal account".into()],
        unresolved: vec!["evidence conflict".into()],
        provenance,
    };
    assert_eq!(synthesis.retained_differences.len(), 1);
    assert_eq!(synthesis.unresolved.len(), 1);
}
