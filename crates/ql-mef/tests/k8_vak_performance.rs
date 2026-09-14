#![recursion_limit = "256"]
//! K8 consumes QL's accepted performance event, not Factory's wire directly.
//! Factory owns execution actuality; `vak_performance` owns C′/musical meaning;
//! this test proves the continuous owner only admits that qualified projection.
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
        source_receipts: vec![json!({"standing":"controlled non-performance source receipt"})],
    }
}

fn factory_performance(subject: &str, frame: &str, thread: &str, musical_role: &str) -> Value {
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

fn performance_event(subject: &str, frame: &str, thread: &str, musical_role: &str) -> Value {
    let (mode_index, mode, voice) = match frame {
        "CF5" => (4, "mixolydian", "Anima"),
        "CF7" => (6, "locrian", "Sophia"),
        _ => panic!("controlled fixture only uses CF5/CF7"),
    };
    let factory = factory_performance(subject, frame, thread, musical_role);
    json!({
        "contract":"ql.vak-performance-event/v1",
        "performanceRef":factory["performanceRef"],
        "observation":{"observationRef":"observation:k8-live","mode":"live"},
        "factoryContract":"factory.vak-orchestration/v1",
        "factoryReceiptRefs":["factory-receipt:attempt","factory-receipt:return"],
        "qlBindingRef":"ql:whole:vak-performance",
        "qlBindingRevision":"ql-revision-1",
        "qlBasisRefs":["docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md","evidence:ql-performance"],
        "semantics":{
            "profileContract":"ql.vak-composition.profile/v1",
            "participation":"authorised-undertaking",
            "content":"CT2",
            "position":"4.2",
            "contextFrame":frame,
            "constitutionalVoice":voice,
            "thread":thread,
            "sequence":"CS2",
            "direction":"forward",
            "musicalRole":musical_role,
            "musicalMode":mode,
            "musicalModeIndex":mode_index,
            "lens":"L0",
            "musicalBasis":"chromatic",
            "framePitch":0
        },
        "settled":true,
        "hasFailure":false,
        "hasInterruption":false,
        "hasLateReturn":false,
        "factory":factory,
        "standing":"controlled QL semantic projection of retained Factory execution"
    })
}

#[test]
fn ql_performance_event_selects_context_frame_mode_without_rewriting_m1() {
    let mut request = input();
    request.schema = REQUEST_V3.into();
    let event = performance_event(&request.m3.subject_ref, "CF5", "CFP1", "chord");
    request.source_receipts.push(event.clone());
    let original = serde_json::to_value(&request).unwrap();
    let basis = request.compose().unwrap();

    assert_eq!(basis.m2_input.vimarsha.as_ref().unwrap().musical_mode, 4);
    assert_eq!(basis.derivation["context_frame"], "CF5");
    assert_eq!(basis.derivation["m1_context_frame"], "CF1");
    assert_eq!(
        basis.derivation["vak_performance_event"]["semantics"]["thread"],
        "CFP1"
    );
    assert_eq!(
        basis.derivation["vak_performance_event"]["semantics"]["musicalRole"],
        "chord"
    );
    assert_eq!(
        basis.derivation["vak_performance_event"]["source_receipt_index"],
        1
    );
    assert_eq!(serde_json::to_value(&basis.input).unwrap(), original);
    assert_eq!(basis.input.source_receipts[1], event);
    assert_eq!(
        basis.m2_input.resonator.as_ref().unwrap().modes[0].frequency_hz,
        basis.m2["vimarsha"]["reading"]["audio_octet_hz"][0]
            .as_f64()
            .unwrap()
    );
    assert_eq!(
        serde_json::to_value(request.compose().unwrap()).unwrap(),
        serde_json::to_value(basis).unwrap(),
        "same immutable performance event must replay exactly"
    );
}

#[test]
fn every_cfp_form_retains_factory_role_while_ql_event_owns_pitch_semantics() {
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
        request.source_receipts.push(performance_event(
            &request.m3.subject_ref,
            "CF7",
            thread,
            role,
        ));
        let basis = request.compose().unwrap();
        assert_eq!(basis.m2_input.vimarsha.as_ref().unwrap().musical_mode, 6);
        assert_eq!(
            basis.derivation["vak_performance_event"]["semantics"]["thread"],
            thread
        );
        assert_eq!(
            basis.derivation["vak_performance_event"]["factory"]["attempts"],
            1
        );
        assert_eq!(basis.derivation["context_frame"], "CF7");
    }
}

#[test]
fn performance_event_is_versioned_fail_closed_and_subject_bound() {
    let base = input();

    let mut missing = base.clone();
    missing.schema = REQUEST_V3.into();
    assert!(
        missing
            .compose()
            .unwrap_err()
            .contains("requires one QL Vāk performance")
    );

    let mut raw_factory = base.clone();
    raw_factory.schema = REQUEST_V3.into();
    raw_factory.source_receipts.push(factory_performance(
        &raw_factory.m3.subject_ref,
        "CF5",
        "CFP0",
        "single-voice",
    ));
    assert!(
        raw_factory
            .compose()
            .unwrap_err()
            .contains("must be projected by the QL Vāk performance owner")
    );

    let mut implicit_upgrade = base.clone();
    implicit_upgrade.source_receipts.push(performance_event(
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
    wrong_subject.source_receipts.push(performance_event(
        "subject:other",
        "CF5",
        "CFP0",
        "single-voice",
    ));
    assert!(
        wrong_subject
            .compose()
            .unwrap_err()
            .contains("same subject")
    );

    let mut semantic_drift = base.clone();
    semantic_drift.schema = REQUEST_V3.into();
    let mut event = performance_event(&semantic_drift.m3.subject_ref, "CF5", "CFP2", "melody");
    event["semantics"]["musicalRole"] = json!("chord");
    semantic_drift.source_receipts.push(event);
    assert!(
        semantic_drift
            .compose()
            .unwrap_err()
            .contains("semantics disagree")
    );

    let mut duplicate = base;
    duplicate.schema = REQUEST_V3.into();
    let event = performance_event(&duplicate.m3.subject_ref, "CF5", "CFP0", "single-voice");
    duplicate.source_receipts.extend([event.clone(), event]);
    assert!(
        duplicate
            .compose()
            .unwrap_err()
            .contains("multiple QL Vāk performance")
    );
}
