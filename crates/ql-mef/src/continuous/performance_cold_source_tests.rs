//! Real World/FieldHost/native worker source replay. These component tests do
//! not manufacture the private C Act channel or installed transport authority.
use super::*;
use crate::continuous::host::{FieldHost, HOST_REQUEST, HostOperation, HostRequest};
use crate::continuous::performance::{PerformanceConfig, prepare_current_configuration};
use crate::continuous::scene_field::SceneConfig;
use crate::musical_performance_return::{ReturnContext, ReturnReference};
use std::{path::PathBuf, time::Duration};

fn actual_world() -> (crate::scene::WorldRequest, ReturnContext, PerformanceConfig) {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let request: crate::scene::WorldRequest = serde_json::from_value(json!({
        "schema":crate::scene::WORLD_REQUEST,"instance_ref":"expression:cold-native/world",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-current-receiving",
        "texture":[64,64],"units_per_metre":1.0,"sky":sky,
        "start":{"tick12":3,"cycle":7,"aperture":9}}))
    .unwrap();
    let reference = |name: &str| ReturnReference {
        reference: name.into(),
        revision: "1".into(),
    };
    let context = ReturnContext {
        kind: "world".into(),
        context: reference("controlled:cold/world"),
        receiver: reference("controlled:cold/receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        private: false,
        required_assets: vec![],
    };
    let world = crate::scene::world(request.clone()).unwrap();
    let current: crate::continuous::coupled::CoupledInput =
        serde_json::from_value(world["event"].clone()).unwrap();
    let current = current.compose().unwrap();
    let authored = serde_json::from_value(json!({
        "schema":"ql.current-source-performance-preparation/v1",
        "policy_ref":"policy:cold-actual-current/cf-degree-order",
        "session_ref":"performance:cold-actual-current/session",
        "receipt_ref":"receipt:cold-actual-current/preparation",
        "projection_ref":"policy:cold-actual-current/metric-consumer",
        "relation":{"family":"A","pair_index":0,"degree":1,"expansion_side":null},
        "source_face":1,"physical_face":1,
        "mechanical_policy":{"policy":"declared-instrument-default"},
        "source_choice":"retained-current-condition-cf-degree-order",
        "columns":12,"base_register":0,"transpose":0
    }))
    .unwrap();
    let config = prepare_current_configuration(&current, &request.instance_ref, authored).unwrap();
    (request, context, config)
}

#[test]
#[ignore = "requires the actual matching normal-floor field worker; no fallback"]
fn actual_worker_source_cold_replays_and_rejects_valid_other_context_or_lost_original_assets() {
    let worker =
        PathBuf::from(std::env::var("QL_NATIVE_FIELD_WORKER").expect("actual built field worker"));
    let (request, context, config) = actual_world();
    let world = crate::scene::world(request.clone()).unwrap();
    let scene: SceneConfig = serde_json::from_value(world["binding"]["host"].clone()).unwrap();
    let source =
        NativePerformanceReceivingSource::world_source(request.clone(), context.clone()).unwrap();
    let mut host = FieldHost::open_scene(&worker, scene.clone(), Duration::from_secs(20)).unwrap();
    host.bind_performance_receiving_source(source.clone())
        .unwrap();
    let ready = host.ready();
    let reply = host.execute(HostRequest {
        schema: HOST_REQUEST.into(),
        instance_ref: scene.instance_ref.clone(),
        event_ref: ready["field"]["event_ref"].as_str().unwrap().into(),
        subject_ref: ready["field"]["subject_ref"].as_str().unwrap().into(),
        request_id: "1".into(),
        expected_generation: ready["field"]["generation"].as_str().unwrap().into(),
        expected_samples_elapsed: ready["field"]["samples_elapsed"].as_str().unwrap().into(),
        command: HostOperation::PerformancePrepare {
            config: Box::new(config),
        },
    });
    assert_eq!(reply["status"], "ok", "{reply}");
    assert_eq!(reply["performance"]["accepted"], true, "{reply}");
    let actual = host.retained_performance_source_artifact(73).unwrap();
    let expected = &actual["source_assets"];
    let input: crate::continuous::coupled::CoupledInput =
        serde_json::from_value(expected["original_native_input"].clone()).unwrap();
    let current = input.compose().unwrap();
    let mut cold =
        PerformanceOwner::prepare_cold_act_source(&current, &scene.instance_ref, &source, expected)
            .unwrap();
    assert_eq!(cold.source_assets(), expected);
    assert_eq!(cold.native_packet().unwrap(), actual["native_preparation"]);
    assert_eq!(
        serde_json::to_value(cold.binding().native_basis()).unwrap(),
        actual["native_basis"]
    );
    assert!(
        cold.reading().is_none(),
        "pure source preparation must not claim native readback"
    );
    let mut other = context.clone();
    other.context.reference = "controlled:cold/valid-other-world".into();
    let other = NativePerformanceReceivingSource::world_source(request.clone(), other).unwrap();
    other.prepare_current(&cold, &current, 0).unwrap();
    assert!(
        PerformanceOwner::prepare_cold_act_source(&current, &scene.instance_ref, &other, expected)
            .is_err()
    );
    for path in [
        "/current_receiving/source_payload_context",
        "/receiving_source_inputs/world_request",
        "/source_geometry_reading",
    ] {
        let mut lost = expected.clone();
        let (parent, key) = path.rsplit_once('/').unwrap();
        lost.pointer_mut(parent)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(key)
            .expect("the genuine original source witness must exist");
        assert!(
            PerformanceOwner::prepare_cold_act_source(
                &current,
                &scene.instance_ref,
                &source,
                &lost
            )
            .is_err(),
            "{path}"
        );
    }
    let mut extra = expected.clone();
    extra["receiving_source_inputs"]["identity_profile"] =
        json!({"private":"another native source"});
    assert!(
        PerformanceOwner::prepare_cold_act_source(&current, &scene.instance_ref, &source, &extra)
            .is_err()
    );
    // The fresh real worker must independently confirm the saved role set;
    // the retained copy did not grant numerical residency during preparation.
    let mut fresh = FieldHost::open_scene(&worker, scene, Duration::from_secs(20)).unwrap();
    fresh
        .bind_performance_receiving_source(source.clone())
        .unwrap();
    // This test is a numerical/source component, not a C authority factory.
    // The production call additionally requires the held selected Act lease.
    let receipts = cold
        .activate_cold_act_source(&current, &source, fresh.session.session_mut())
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.reason, failure.native_receipts));
    assert_eq!(receipts.len(), 2);
    for receipt in &receipts {
        assert_eq!(receipt["accepted"], true, "{receipt}");
    }
    assert_eq!(receipts[1]["reading"]["samples_elapsed"], "0");
    assert_eq!(
        receipts[1]["reading"]["consumer_roles"],
        expected["consumer_roles"]
    );
    assert_eq!(
        cold.source_assets(),
        expected,
        "cold activation preserves complete original source history"
    );
}
