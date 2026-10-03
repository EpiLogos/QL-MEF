//! Actual FieldHost/worker producer for pure initial candidate/currentness.
//! The closed Scene CAS/private lease installation is tested by its C owner;
//! no imported lease or fabricated pulse supplies a positive here.
use super::*;
use crate::continuous::performance::native_source_support as native_source;
use crate::continuous::performance::{AcousticConfiguration, AcousticDirectivity};
use crate::musical_performance_return::{ReturnContext, ReturnReference};
fn configuration() -> AcousticConfiguration {
    AcousticConfiguration {
        schema: "ql.native-acoustic-receiving-configuration/v1".into(),
        source_ref: "controlled:initial-acoustic/actual-pickup".into(),
        source_motion_ref: "controlled:initial-acoustic/emitter".into(),
        receiver_motion_ref: "controlled:initial-acoustic/receiver".into(),
        policy_ref: "controlled:initial-acoustic/metric-model".into(),
        policy_revision: "1".into(),
        standing: "architecture-model".into(),
        revision: 1,
        source_translation_metres: [0., 0., 0.],
        receiver_position_metres: [0., 0., 1.],
        receiver_forward: [0., 0., -1.],
        source_velocity_metres_per_second: [0.; 3],
        receiver_velocity_metres_per_second: [0.; 3],
        speed_metres_per_second: 340.,
        minimum_distance_metres: 1.,
        directivity: AcousticDirectivity::Omnidirectional,
        propagation_delay: true,
        span_samples: 480_000,
    }
}
fn request(host: &FieldHost, ordinal: &str, command: HostOperation) -> HostRequest {
    let ready = host.ready();
    HostRequest {
        schema: HOST_REQUEST.into(),
        instance_ref: host.instance_ref.clone(),
        event_ref: ready["field"]["event_ref"].as_str().unwrap().into(),
        subject_ref: ready["field"]["subject_ref"].as_str().unwrap().into(),
        request_id: ordinal.into(),
        expected_generation: ready["field"]["generation"].as_str().unwrap().into(),
        expected_samples_elapsed: ready["field"]["samples_elapsed"].as_str().unwrap().into(),
        command,
    }
}
#[test]
#[ignore = "requires the exact normal-floor native worker; no private lease is manufactured"]
fn actual_native_acoustic_initial_candidate_is_pure_and_detects_stale_boundary() {
    let worker =
        std::path::PathBuf::from(std::env::var("QL_NATIVE_FIELD_WORKER").expect("actual worker"));
    let sky: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let world_request: crate::scene::WorldRequest = serde_json::from_value(json!({
        "schema":crate::scene::WORLD_REQUEST,"instance_ref":"expression:initial-acoustic/world",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-current-receiving",
        "texture":[64,64],"units_per_metre":1.,"sky":sky,"start":{"tick12":3,"cycle":7,"aperture":9}
    }))
    .unwrap();
    let actual = world(world_request.clone()).unwrap();
    let scene: SceneConfig = serde_json::from_value(actual["binding"]["host"].clone()).unwrap();
    let r = |reference: &str| ReturnReference {
        reference: reference.into(),
        revision: "1".into(),
    };
    let context = ReturnContext {
        kind: "world".into(),
        context: r("controlled:initial-acoustic/world"),
        receiver: r("controlled:initial-acoustic/receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        private: false,
        required_assets: vec![],
    };
    let source = NativePerformanceReceivingSource::world_source(world_request, context).unwrap();
    let mut host = FieldHost::open_scene(&worker, scene, Duration::from_secs(20)).unwrap();
    host.bind_performance_receiving_source(source.clone())
        .unwrap();
    let current = host.session.session().current_basis().clone();
    let (_, mut config) = native_source::config(true);
    config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    let prepare = request(
        &host,
        "1",
        HostOperation::PerformancePrepare {
            config: Box::new(config),
        },
    );
    let reply = host.execute(prepare);
    assert_eq!(reply["status"], "ok", "{reply}");
    assert_eq!(reply["performance"]["accepted"], true, "{reply}");
    let original_assets = host.performance.as_ref().unwrap().source_assets().clone();
    let original_reading = host
        .performance
        .as_ref()
        .unwrap()
        .reading()
        .unwrap()
        .clone();
    let candidate = host
        .prepare_performance_acoustic_installation(configuration())
        .unwrap();
    assert_eq!(candidate.before_source_assets(), &original_assets);
    assert_eq!(candidate.native_boundary(), &original_reading);
    assert_eq!(candidate.configuration(), &configuration());
    assert_eq!(candidate.preparation().packet()["origin_sample"], "0");
    assert_eq!(
        candidate.preparation().packet()["history_origin_sample"],
        "0"
    );
    assert_eq!(
        candidate.source_assets()["acoustic_receiving"],
        candidate.preparation().snapshot()
    );
    assert_eq!(
        candidate.source_assets()["current_receiving"],
        candidate.preparation().current_receiving().clone()
    );
    assert_eq!(
        candidate.source_assets()["receiving_source_inputs"]["acoustic_receiving"],
        serde_json::to_value(configuration()).unwrap()
    );
    assert_eq!(
        host.performance.as_ref().unwrap().source_assets(),
        &original_assets
    );
    assert_eq!(
        host.performance.as_ref().unwrap().reading().unwrap(),
        &original_reading
    );
    // Dropping a candidate after a denied Scene CAS cannot publish its assets.
    let abandoned = host
        .prepare_performance_acoustic_installation(configuration())
        .unwrap();
    drop(abandoned);
    assert_eq!(
        host.performance.as_ref().unwrap().source_assets(),
        &original_assets
    );
    assert_eq!(
        host.performance.as_ref().unwrap().reading().unwrap(),
        &original_reading
    );
    let after = source
        .clone()
        .with_acoustic_configuration(configuration())
        .unwrap();
    host.performance
        .as_ref()
        .unwrap()
        .validate_stopped_acoustic_installation_candidate(&current, &source, &after, &candidate)
        .unwrap();
    // A different valid native receiver is not an initial numerical addition.
    let mut other_context = source.return_context().clone();
    other_context.receiver.reference = "controlled:initial-acoustic/valid-other-receiver".into();
    let world_input = source.source_inputs().unwrap()["world_request"].clone();
    let other = NativePerformanceReceivingSource::world_source(
        serde_json::from_value(world_input).unwrap(),
        other_context,
    )
    .unwrap();
    other
        .prepare_current(host.performance.as_ref().unwrap(), &current, 0)
        .unwrap();
    assert!(
        host.performance
            .as_ref()
            .unwrap()
            .prepare_stopped_acoustic_installation_assets(
                &current,
                &other,
                &other
                    .clone()
                    .with_acoustic_configuration(configuration())
                    .unwrap()
            )
            .is_err()
    );
    // The actual native owner accepts a parameter into its queue without
    // advancing P. Same cursor does not make the old complete boundary current.
    let event = json!({"identity":host.performance.as_ref().unwrap().binding().determination()["identity"],
        "kind":5,"sequence":"1","sample":"48000","touch":"0",
        "value":0.6,"pitch_hz":0.0,"parameter":4,"late_admitted":false,"has_note":false,"has_determination":false});
    // Authored score provides its deadline. Only native admission may stamp
    // requested_sample or a device clock: retained application fields cannot
    // be resubmitted as if the caller owned that timing provenance.
    for native_timing in [
        json!({"requested_sample":"48000"}),
        json!({"native_clock":{
            "epoch":"1","anchor_ordinal":"1","trigger_host_ticks":"1",
            "admitted_host_ticks":"1","mapping_uncertainty_samples":0.0,
            "input_transit_unknown":true
        }}),
    ] {
        let mut wrong = event.clone();
        wrong
            .as_object_mut()
            .unwrap()
            .extend(native_timing.as_object().unwrap().clone());
        let refused = host
            .performance
            .as_mut()
            .unwrap()
            .owner_stopped_exchange(
                &current,
                &source,
                host.session.session_mut(),
                "score",
                &json!({"event":wrong,"input_ref":null}),
            )
            .unwrap_or_else(|failure| {
                panic!(
                    "spoofed stopped score postvalidation failed: {}; native receipts: {:?}",
                    failure.reason, failure.native_receipts
                )
            });
        assert_eq!(refused["accepted"], false, "{refused}");
        assert_eq!(refused["reason"], "invalid", "{refused}");
        assert_eq!(refused["reading"], original_reading, "{refused}");
        assert_eq!(refused["applications"], json!([]));
        assert_eq!(refused["input_history"], json!([]));
        assert_eq!(
            host.performance.as_ref().unwrap().source_assets(),
            &original_assets
        );
    }
    let reply = host
        .performance
        .as_mut()
        .unwrap()
        .owner_stopped_exchange(
            &current,
            &source,
            host.session.session_mut(),
            "score",
            &json!({"event":event,"input_ref":null}),
        )
        .unwrap_or_else(|failure| {
            panic!(
                "actual stopped score exchange refused: {}; native receipts: {:?}",
                failure.reason, failure.native_receipts
            )
        });
    assert_eq!(reply["accepted"], true, "{reply}");
    assert_eq!(
        reply["payload"]["score_admission"]["queued"], true,
        "{reply}"
    );
    assert_eq!(
        reply["payload"]["score_admission"]["event"]["sample"],
        "48000"
    );
    assert_eq!(
        reply["payload"]["score_admission"]["event"]["requested_sample"],
        "48000"
    );
    assert_eq!(reply["payload"]["score_admission"]["queue_cursor"], "0");
    assert_eq!(
        host.performance.as_ref().unwrap().reading().unwrap()["samples_elapsed"],
        "0"
    );
    assert_ne!(
        host.performance.as_ref().unwrap().reading().unwrap()["accepted_sequence"],
        original_reading["accepted_sequence"]
    );
    assert!(
        host.performance
            .as_ref()
            .unwrap()
            .validate_stopped_acoustic_installation_candidate(&current, &source, &after, &candidate)
            .is_err()
    );
    assert_eq!(
        host.performance.as_ref().unwrap().source_assets(),
        &original_assets
    );
    let fresh = host
        .prepare_performance_acoustic_installation(configuration())
        .unwrap();
    assert_eq!(fresh.before_source_assets(), &original_assets);
    assert_ne!(fresh.native_boundary(), candidate.native_boundary());
    assert_eq!(
        fresh.preparation().snapshot(),
        candidate.preparation().snapshot()
    );
}
