use ql_mef::{
    CENTRAL_ACTION_OWNER_REVISION, CENTRAL_WORK_LIST_ACTION_REF, FACTORY_ACTION_OWNER_REVISION,
    FACTORY_REQUEST_EVIDENCE_ACTION_REF, SelfOtherForm, VAK_ACTION_PROFILE_CONTRACT,
    VAK_OI_PRIMITIVE_MATRIX_CONTRACT, VAK_PATH_CONTRACT, VAK_RECOGNITION_CONTRACT,
    VakAddressHorizon, VakContextField, VakExecutionObservationV1, VakExpressionSubject,
    VakExpressionV1, VakOiPrimitiveKind, VakPathStepV1, VakRegistry, VakRelationOp, VakStanding,
    central_work_list_profile, factory_request_evidence_profile, oi_reference_primitive_matrix,
    recognise_vak_return, reconstruct_observed_vak_path,
};

#[test]
fn broad_oi_matrix_covers_every_required_primitive_without_collapsing_native_ownership() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let matrix = oi_reference_primitive_matrix(&registry).unwrap();
    assert_eq!(matrix.contract, VAK_OI_PRIMITIVE_MATRIX_CONTRACT);
    assert_eq!(matrix.standing, VakStanding::AuthoredArchitecture);

    for primitive in VakOiPrimitiveKind::ALL {
        assert!(
            matrix
                .relations
                .iter()
                .any(|relation| relation.primitive == primitive)
        );
    }

    let action_owners = matrix
        .relations
        .iter()
        .filter(|relation| relation.primitive == VakOiPrimitiveKind::ActionRef)
        .map(|relation| relation.native_owner.as_str())
        .collect::<Vec<_>>();
    assert!(action_owners.contains(&"factory"));
    assert!(action_owners.contains(&"central"));
    assert!(matrix.relations.iter().any(|relation| {
        relation.native_ref == FACTORY_REQUEST_EVIDENCE_ACTION_REF
            && relation.standing == VakStanding::Implementation
    }));
    assert!(matrix.relations.iter().any(|relation| {
        relation.native_ref == CENTRAL_WORK_LIST_ACTION_REF
            && relation.standing == VakStanding::Implementation
    }));
}

#[test]
fn canonical_action_profiles_are_pinned_to_two_real_native_owners() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let factory = factory_request_evidence_profile(&registry).unwrap();
    assert_eq!(factory.contract, VAK_ACTION_PROFILE_CONTRACT);
    assert_eq!(factory.action_ref, FACTORY_REQUEST_EVIDENCE_ACTION_REF);
    assert_eq!(factory.native_owner, "factory");
    assert_eq!(factory.binding_revision, FACTORY_ACTION_OWNER_REVISION);
    assert_eq!(
        factory.primary_vak_ref,
        SelfOtherForm::QueryOfOther.source_ref()
    );
    assert!(!factory.affordances.is_empty());

    let central = central_work_list_profile(&registry).unwrap();
    assert_eq!(central.contract, VAK_ACTION_PROFILE_CONTRACT);
    assert_eq!(central.action_ref, CENTRAL_WORK_LIST_ACTION_REF);
    assert_eq!(central.native_owner, "central");
    assert_eq!(central.binding_revision, CENTRAL_ACTION_OWNER_REVISION);
    assert_eq!(central.primary_vak_ref, VakContextField::World.source_ref());
    assert!(!central.affordances.is_empty());
}

#[test]
fn an_implementation_binding_cannot_masquerade_as_an_observed_vak_path() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let observation = VakExecutionObservationV1 {
        observation_ref: "fixture:not-observed".into(),
        owner_revision: FACTORY_ACTION_OWNER_REVISION.into(),
        evidence_run_ref: "fixture:no-owner-runtime".into(),
        method_ref: "method:ql83/conformance".into(),
        resolve_expression: "@2 candidate / @5 request-evidence".into(),
        world_ref: None,
        project_ref: None,
        focus_ref: None,
        actor_ref: None,
        agency_ref: None,
        action_profile: factory_request_evidence_profile(&registry).unwrap(),
        steps: Vec::new(),
        evidence_refs: vec!["source code exists".into()],
        standing: VakStanding::Implementation,
    };
    let error = reconstruct_observed_vak_path(&registry, observation).unwrap_err();
    assert!(error.to_string().contains("OBSERVED"));
}

/// This becomes a real observed cross-product path only in the dedicated owner-conformance job.
/// Ordinary repository CI intentionally returns early rather than fabricating runtime evidence.
#[test]
fn native_owner_conformance_can_return_through_vak_path_and_m5_recognition() {
    let Ok(run_ref) = std::env::var("VAK_OWNER_CONFORMANCE_RUN_REF") else {
        return;
    };
    assert!(!run_ref.trim().is_empty());
    let registry = VakRegistry::from_authoritative_source().unwrap();

    let factory_profile = factory_request_evidence_profile(&registry).unwrap();
    let factory_return = "human-request/request-evidence/candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC";
    let factory_path = reconstruct_observed_vak_path(
        &registry,
        VakExecutionObservationV1 {
            observation_ref: "factory:build-file-provider/request-evidence".into(),
            owner_revision: FACTORY_ACTION_OWNER_REVISION.into(),
            evidence_run_ref: run_ref.clone(),
            method_ref: "method:ql83/factory-request-evidence-conformance".into(),
            resolve_expression: "@2 candidate / @5 request-evidence".into(),
            world_ref: Some("factory:world/build".into()),
            project_ref: Some("project:01ARZ3NDEKTSV4RRFFQ69G5FCA".into()),
            focus_ref: Some("candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC".into()),
            actor_ref: Some("authority/provider-test".into()),
            agency_ref: None,
            action_profile: factory_profile,
            steps: vec![
                VakPathStepV1 {
                    step_id: "factory-invocation".into(),
                    expression: VakExpressionV1 {
                        contract: ql_mef::VAK_EXPRESSION_CONTRACT,
                        operator: VakRelationOp::Potential,
                        horizon: VakAddressHorizon::H2,
                        subjects: vec![
                            VakExpressionSubject::Native(
                                "candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC".into(),
                            ),
                            VakExpressionSubject::Vak(SelfOtherForm::QueryOfOther.source_ref()),
                        ],
                        relation_refs: vec![SelfOtherForm::QueryOfOther.source_ref()],
                        complement_refs: Vec::new(),
                        world_ref: Some("factory:world/build".into()),
                        project_ref: Some("project:01ARZ3NDEKTSV4RRFFQ69G5FCA".into()),
                        focus_ref: Some("candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC".into()),
                        expected_return: Some("additional-evidence human request".into()),
                        standing: VakStanding::Observed,
                        evidence: vec![run_ref.clone()],
                    },
                    native_subject_refs: vec!["candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC".into()],
                    method_ref: Some("method:ql83/factory-request-evidence-conformance".into()),
                    action_ref: Some(FACTORY_REQUEST_EVIDENCE_ACTION_REF.into()),
                    invocation_ref: Some(
                        "factory/tests/build_file_provider.rs::FactoryActionInvocation".into(),
                    ),
                    activity_ref: None,
                    result_refs: Vec::new(),
                    return_ref: None,
                    source_surface: Some("factory-build-provider-test".into()),
                    evidence_refs: vec![run_ref.clone()],
                    standing: VakStanding::Observed,
                },
                VakPathStepV1 {
                    step_id: "factory-activity-return".into(),
                    expression: VakExpressionV1 {
                        contract: ql_mef::VAK_EXPRESSION_CONTRACT,
                        operator: VakRelationOp::Express,
                        horizon: VakAddressHorizon::H5,
                        subjects: vec![VakExpressionSubject::Native(
                            FACTORY_REQUEST_EVIDENCE_ACTION_REF.into(),
                        )],
                        relation_refs: vec![VakContextField::Techne.source_ref()],
                        complement_refs: Vec::new(),
                        world_ref: Some("factory:world/build".into()),
                        project_ref: Some("project:01ARZ3NDEKTSV4RRFFQ69G5FCA".into()),
                        focus_ref: Some("candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC".into()),
                        expected_return: Some(factory_return.into()),
                        standing: VakStanding::Observed,
                        evidence: vec![run_ref.clone()],
                    },
                    native_subject_refs: vec![FACTORY_REQUEST_EVIDENCE_ACTION_REF.into()],
                    method_ref: Some("method:ql83/factory-request-evidence-conformance".into()),
                    action_ref: Some(FACTORY_REQUEST_EVIDENCE_ACTION_REF.into()),
                    invocation_ref: None,
                    activity_ref: Some("factory:FactoryActionExecutor.execute".into()),
                    result_refs: vec![factory_return.into()],
                    return_ref: Some(factory_return.into()),
                    source_surface: Some("factory-build-provider-test".into()),
                    evidence_refs: vec![run_ref.clone()],
                    standing: VakStanding::Observed,
                },
            ],
            evidence_refs: vec![
                format!("EpiLogos/agent-system-design@{FACTORY_ACTION_OWNER_REVISION}"),
                run_ref.clone(),
            ],
            standing: VakStanding::Observed,
        },
    )
    .unwrap();
    assert_eq!(factory_path.contract, VAK_PATH_CONTRACT);
    assert_eq!(factory_path.standing, VakStanding::Observed);
    assert!(
        factory_path
            .steps
            .iter()
            .any(|step| step.return_ref.is_some())
    );
    let recognition = recognise_vak_return(
        &registry,
        &factory_path,
        "recognition:ql83/factory-request-evidence",
        vec![run_ref.clone()],
    )
    .unwrap();
    assert_eq!(recognition.contract, VAK_RECOGNITION_CONTRACT);
    assert_eq!(recognition.standing, VakStanding::Derived);
    assert!(
        recognition
            .returned_refs
            .iter()
            .any(|value| value == factory_return)
    );
    assert!(recognition.proposals.is_empty());

    let central_profile = central_work_list_profile(&registry).unwrap();
    let central_path = reconstruct_observed_vak_path(
        &registry,
        VakExecutionObservationV1 {
            observation_ref: "central:port-connector/work-list".into(),
            owner_revision: CENTRAL_ACTION_OWNER_REVISION.into(),
            evidence_run_ref: run_ref.clone(),
            method_ref: "method:ql83/central-work-list-conformance".into(),
            resolve_expression: "@4 Central/Work x @3 work.list".into(),
            world_ref: Some("central:world".into()),
            project_ref: None,
            focus_ref: Some("Central/Work".into()),
            actor_ref: Some("central-ctrl:test".into()),
            agency_ref: None,
            action_profile: central_profile,
            steps: vec![
                VakPathStepV1 {
                    step_id: "central-invocation".into(),
                    expression: VakExpressionV1 {
                        contract: ql_mef::VAK_EXPRESSION_CONTRACT,
                        operator: VakRelationOp::Relate,
                        horizon: VakAddressHorizon::H4,
                        subjects: vec![VakExpressionSubject::Native("Central/Work".into())],
                        relation_refs: vec![VakContextField::World.source_ref()],
                        complement_refs: Vec::new(),
                        world_ref: Some("central:world".into()),
                        project_ref: None,
                        focus_ref: Some("Central/Work".into()),
                        expected_return: Some("Work item list".into()),
                        standing: VakStanding::Observed,
                        evidence: vec![run_ref.clone()],
                    },
                    native_subject_refs: vec!["Central/Work".into()],
                    method_ref: Some("method:ql83/central-work-list-conformance".into()),
                    action_ref: Some(CENTRAL_WORK_LIST_ACTION_REF.into()),
                    invocation_ref: Some("central:ActionRegistry.execute/work.list".into()),
                    activity_ref: None,
                    result_refs: Vec::new(),
                    return_ref: None,
                    source_surface: Some("central-ctrl-port-connector-test".into()),
                    evidence_refs: vec![run_ref.clone()],
                    standing: VakStanding::Observed,
                },
                VakPathStepV1 {
                    step_id: "central-provider-return".into(),
                    expression: VakExpressionV1 {
                        contract: ql_mef::VAK_EXPRESSION_CONTRACT,
                        operator: VakRelationOp::Express,
                        horizon: VakAddressHorizon::H3,
                        subjects: vec![VakExpressionSubject::Native(
                            "reference.work-filesystem".into(),
                        )],
                        relation_refs: vec![VakContextField::Language.source_ref()],
                        complement_refs: Vec::new(),
                        world_ref: Some("central:world".into()),
                        project_ref: None,
                        focus_ref: Some("Central/Work".into()),
                        expected_return: Some("ActionResult::Success(work.list)".into()),
                        standing: VakStanding::Observed,
                        evidence: vec![run_ref.clone()],
                    },
                    native_subject_refs: vec!["reference.work-filesystem".into()],
                    method_ref: Some("method:ql83/central-work-list-conformance".into()),
                    action_ref: Some(CENTRAL_WORK_LIST_ACTION_REF.into()),
                    invocation_ref: None,
                    activity_ref: Some("central:WorkDiscovery.list".into()),
                    result_refs: vec!["central:ActionResult/work.list/success".into()],
                    return_ref: Some("central:ActionResult/work.list/success".into()),
                    source_surface: Some("central-ctrl-port-connector-test".into()),
                    evidence_refs: vec![run_ref.clone()],
                    standing: VakStanding::Observed,
                },
            ],
            evidence_refs: vec![
                format!("EpiLogos/Central@{CENTRAL_ACTION_OWNER_REVISION}"),
                run_ref.clone(),
            ],
            standing: VakStanding::Observed,
        },
    )
    .unwrap();
    assert_eq!(central_path.standing, VakStanding::Observed);
    let central_recognition = recognise_vak_return(
        &registry,
        &central_path,
        "recognition:ql83/central-work-list",
        vec![run_ref],
    )
    .unwrap();
    assert_eq!(central_recognition.standing, VakStanding::Derived);
    assert!(
        central_recognition
            .changed_fields
            .contains(&VakContextField::World)
    );
    assert!(
        central_recognition
            .changed_fields
            .contains(&VakContextField::Language)
    );
}
