#![recursion_limit = "256"]
//! K8 consumes the accepted Factory performance contract as execution evidence.
//! The controlled JSON below exercises that public wire shape; Factory remains
//! the producer/attempt owner and QL remains the Context-Frame/musical owner.
use ql_mef::continuous::coupled::{
    CoupledInput, FrequencyBinding, HarmonicSource, REQUEST_V2, REQUEST_V3,
};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_engine::M2Request;
use ql_mef::m3_state::M3Request;
use serde_json::{Value, json};

fn input() -> CoupledInput {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(fixture["config"].clone()).unwrap();
    let mut m2: M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    let m3_fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_fixture["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m1.context_frame = 1;
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    m2.resonator = Some(
        serde_json::from_value(json!({
            "stamp":m2.stamp,
            "provider_ref":"controlled:vak-performance-material-provider",
            "geometry_ref":"controlled:vak-performance-geometry",
            "material_ref":"controlled:vak-performance-material",
            "material_model_ref":"ql.continuous-linear-mode/v1",
            "material_parameters":{},
            "modes":[{
                "mode_ref":"controlled:vak-performance-mode/0",
                "source_coordinate":"#2-2-2-5-5",
                "material_fibre":"earth",
                "carrier_weights":[{"carrier":0,"weight":1.0}],
                "frequency_hz":137.0,
                "amplitude":[0.002,0.001],
                "excitation":[0.001,0.0],
                "damping_per_second":0.25,
                "nodal_state_ref":"controlled:vak-nodes",
                "antinodal_state_ref":"controlled:vak-antinodes"
            }]
        }))
        .unwrap(),
    );
    CoupledInput {
        schema: REQUEST_V2.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: vec![FrequencyBinding {
            mode_ref: "controlled:vak-performance-mode/0".into(),
            octet_index: 0,
        }],
        condition_frequency_bindings: vec![],
        source_receipts: vec![json!({"standing":"controlled non-Factory source receipt"})],
    }
}

fn performance(subject: &str, frame: &str, thread: &str, musical_role: &str) -> Value {
    json!({
        "contract":"factory.vak-orchestration/v1",
        "performanceRef":format!("performance:{thread}"),
        "runRef":"run:01ARZ3NDEKTSV4RRFFQ69G5FBD",
        "runRevision":7,
        "workflowSourceRef":"source:factory-workflow",
        "workflowSourceRevision":"source-revision-7",
        "workflowSourceDigest":"sha256:controlled-factory-workflow",
        "actorRef":"agent:epii",
        "subjectRef":subject,
        "wholeRef":"whole:factory-vak-performance",
        "qlBindingRef":"ql:whole:vak-performance",
        "qlBindingRevision":"ql-revision-1",
        "aiKitResolvePathRef":"resolve-scoped-path:vak-performance",
        "contextResolutionRef":"context-resolution:vak-performance",
        "sourceRefs":["source:factory-workflow","source:ql-c-prime","source:aikit-scope"],
        "frame":frame,
        "thread":thread,
        "sequence":"CS2",
        "direction":"forward",
        "musicalRole":musical_role,
        "attempts":[{
            "unitRef":"workflow-unit:inspect-source",
            "attemptIndex":0,
            "current":true,
            "executionRef":"execution:vak-performance",
            "actorRef":"agent:epii",
            "wholeRef":"whole:inspect-source",
            "subjectRef":subject,
            "qlBindingRef":"ql:whole:vak-performance",
            "qlBindingRevision":"ql-revision-1",
            "aiKitResolvePathRef":"resolve-scoped-path:inspect-source",
            "contextResolutionRef":"context-resolution:inspect-source",
            "sourceRefs":["source:factory-workflow"],
            "modelRef":"model:controlled",
            "providerRef":"provider:controlled",
            "status":"returned",
            "statusHistory":["active","returned"],
            "artifactRefs":["artifact:vak-performance"],
            "lateArtifactRefs":[],
            "evidenceRefs":["evidence:vak-performance"],
            "failureReason":null
        }],
        "chainInputs":[],
        "sustainedStop":null
    })
}

#[test]
fn accepted_factory_performance_selects_ql_context_frame_mode_without_rewriting_m1() {
    let mut request = input();
    request.schema = REQUEST_V3.into();
    let receipt = performance(&request.m3.subject_ref, "CF5", "CFP1", "chord");
    request.source_receipts.push(receipt.clone());
    let original = serde_json::to_value(&request).unwrap();
    let basis = request.compose().unwrap();

    assert_eq!(basis.m2_input.vimarsha.as_ref().unwrap().musical_mode, 4);
    assert_eq!(basis.derivation["context_frame"], "CF5");
    assert_eq!(basis.derivation["m1_context_frame"], "CF1");
    assert_eq!(
        basis.derivation["factory_vak_performance"]["thread"],
        "CFP1"
    );
    assert_eq!(
        basis.derivation["factory_vak_performance"]["musical_role"],
        "chord"
    );
    assert_eq!(
        basis.derivation["factory_vak_performance"]["source_receipt_index"],
        1
    );
    assert_eq!(serde_json::to_value(&basis.input).unwrap(), original);
    assert_eq!(basis.input.source_receipts[1], receipt);
    assert_eq!(
        basis.m2_input.resonator.as_ref().unwrap().modes[0].frequency_hz,
        basis.m2["vimarsha"]["reading"]["audio_octet_hz"][0]
            .as_f64()
            .unwrap()
    );
    assert_eq!(
        serde_json::to_value(request.compose().unwrap()).unwrap(),
        serde_json::to_value(basis).unwrap(),
        "same immutable performance must replay exactly"
    );
}

#[test]
fn every_factory_cfp_form_retains_its_source_role_while_ql_owns_the_pitch_derivation() {
    for (thread, role) in [
        ("CFP0", "single-voice"),
        ("CFP1", "chord"),
        ("CFP2", "melody"),
        ("CFP3", "fusion"),
        ("CFP4", "drone"),
        ("CFP5", "canon"),
    ] {
        let mut request = input();
        request.schema = REQUEST_V3.into();
        request
            .source_receipts
            .push(performance(&request.m3.subject_ref, "CF7", thread, role));
        let basis = request.compose().unwrap();
        assert_eq!(basis.m2_input.vimarsha.as_ref().unwrap().musical_mode, 6);
        assert_eq!(
            basis.derivation["factory_vak_performance"]["thread"],
            thread
        );
        assert_eq!(
            basis.derivation["factory_vak_performance"]["musical_role"],
            role
        );
        assert_eq!(basis.derivation["context_frame"], "CF7");
    }
}

#[test]
fn factory_performance_is_versioned_fail_closed_and_identity_bound() {
    let base = input();

    let mut missing = base.clone();
    missing.schema = REQUEST_V3.into();
    assert!(
        missing
            .compose()
            .unwrap_err()
            .contains("requires one actual Factory")
    );

    let mut implicit_upgrade = base.clone();
    implicit_upgrade.source_receipts.push(performance(
        &implicit_upgrade.m3.subject_ref,
        "CF5",
        "CFP0",
        "single-voice",
    ));
    assert!(
        implicit_upgrade
            .compose()
            .unwrap_err()
            .contains("explicit v3")
    );

    let mut wrong_subject = base.clone();
    wrong_subject.schema = REQUEST_V3.into();
    wrong_subject
        .source_receipts
        .push(performance("subject:other", "CF5", "CFP0", "single-voice"));
    assert!(
        wrong_subject
            .compose()
            .unwrap_err()
            .contains("same subject")
    );

    let mut wrong_role = base.clone();
    wrong_role.schema = REQUEST_V3.into();
    wrong_role.source_receipts.push(performance(
        &wrong_role.m3.subject_ref,
        "CF5",
        "CFP2",
        "chord",
    ));
    assert!(
        wrong_role
            .compose()
            .unwrap_err()
            .contains("thread/musical role")
    );

    let mut duplicate = base.clone();
    duplicate.schema = REQUEST_V3.into();
    let event = performance(&duplicate.m3.subject_ref, "CF5", "CFP0", "single-voice");
    duplicate.source_receipts.extend([event.clone(), event]);
    assert!(
        duplicate
            .compose()
            .unwrap_err()
            .contains("multiple Factory")
    );

    let mut no_attempt = base.clone();
    no_attempt.schema = REQUEST_V3.into();
    let mut event = performance(&no_attempt.m3.subject_ref, "CF5", "CFP0", "single-voice");
    event["attempts"] = json!([]);
    no_attempt.source_receipts.push(event);
    assert!(no_attempt.compose().unwrap_err().contains("actual attempt"));

    let mut changed_actor = base;
    changed_actor.schema = REQUEST_V3.into();
    let mut event = performance(&changed_actor.m3.subject_ref, "CF5", "CFP0", "single-voice");
    event["attempts"][0]["actorRef"] = json!("agent:other");
    changed_actor.source_receipts.push(event);
    assert!(
        changed_actor
            .compose()
            .unwrap_err()
            .contains("changed actor/subject/QL identity")
    );
}
