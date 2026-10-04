//! Genuine prepared worker/Host: imported numerical Contact cannot become
//! admission through the existing generic performance transport.
use super::*;
use crate::continuous::performance::native_source_support;
use crate::musical_performance_return::{ReturnContext, ReturnReference};

#[test]
#[ignore = "requires the exact normal-floor native worker; no Scene lease is manufactured"]
fn actual_prepared_worker_denies_generic_contact_without_changing_full_checkpoint() {
    let worker = std::path::PathBuf::from(
        std::env::var("QL_NATIVE_FIELD_WORKER").expect("actual matching worker"),
    );
    let sky: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let input: crate::scene::WorldRequest = serde_json::from_value(json!({
        "schema":crate::scene::WORLD_REQUEST,"instance_ref":"expression:contact-ingress/world",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-current-receiving",
        "texture":[64,64],"units_per_metre":1.,"sky":sky,
        "start":{"tick12":3,"cycle":7,"aperture":9}
    }))
    .unwrap();
    let actual = world(input.clone()).unwrap();
    let scene: SceneConfig = serde_json::from_value(actual["binding"]["host"].clone()).unwrap();
    let reference = |name: &str| ReturnReference {
        reference: name.into(),
        revision: "1".into(),
    };
    let context = ReturnContext {
        kind: "world".into(),
        context: reference("controlled:contact-ingress/world"),
        receiver: reference("controlled:contact-ingress/receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        private: false,
        required_assets: vec![],
    };
    let source = NativePerformanceReceivingSource::world_source(input, context).unwrap();
    let mut host = FieldHost::open_scene(&worker, scene, Duration::from_secs(20)).unwrap();
    host.bind_performance_receiving_source(source.clone())
        .unwrap();
    let current = host.session.session().current_basis().clone();
    let (_, mut config) = native_source_support::config(true);
    config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    let ready = host.ready();
    let request = HostRequest {
        schema: HOST_REQUEST.into(),
        instance_ref: host.instance_ref.clone(),
        event_ref: ready["field"]["event_ref"].as_str().unwrap().into(),
        subject_ref: ready["field"]["subject_ref"].as_str().unwrap().into(),
        request_id: "1".into(),
        expected_generation: ready["field"]["generation"].as_str().unwrap().into(),
        expected_samples_elapsed: ready["field"]["samples_elapsed"].as_str().unwrap().into(),
        command: HostOperation::PerformancePrepare {
            config: Box::new(config),
        },
    };
    let prepared = host.execute(request);
    assert_eq!(prepared["status"], "ok", "{prepared}");
    assert_eq!(prepared["performance"]["accepted"], true, "{prepared}");
    let source_before = host.performance.as_ref().unwrap().source_assets().clone();
    let before = host
        .performance
        .as_mut()
        .unwrap()
        .owner_stopped_exchange(
            &current,
            &source,
            host.session.session_mut(),
            "checkpoint",
            &json!({}),
        )
        .unwrap_or_else(|failure| {
            panic!(
                "actual stopped native checkpoint refused: {}",
                failure.reason
            )
        });
    assert_eq!(before["accepted"], true, "{before}");
    for phase in [
        "contact-prepare",
        "contact-apply",
        "contact-trigger",
        "contact-history-verify",
    ] {
        let imported = json!({"schema": if phase == "contact-history-verify" {
                "ql.native-scene-contact-replay-request/v1"
            } else { "ql.native-scene-contact-owner-request/v1" },
            "operation":phase,"original_native_request_id":"1",
            "scene_constructor":{"schema":"oi.native-document-scene-constructor/v1"},
            "contact_source":{"schema":"ql.native-scene-contact-source/v1"},
            "contact":{"generation":"1","slot":0}});
        let refusal = host
            .session
            .session_mut()
            .performance_exchange_retained(&imported);
        match refusal {
            Ok(_) => panic!("generic {phase} minted a private native Contact operation"),
            Err((reason, original)) => {
                assert_eq!(reason, "unknown native performance request contract");
                assert!(
                    original.is_none(),
                    "pre-delivery rejection fabricated a native pulse"
                );
            }
        }
        assert!(host.available());
        assert_eq!(
            host.performance.as_ref().unwrap().source_assets(),
            &source_before
        );
    }
    let after = host
        .performance
        .as_mut()
        .unwrap()
        .owner_stopped_exchange(
            &current,
            &source,
            host.session.session_mut(),
            "checkpoint",
            &json!({}),
        )
        .unwrap_or_else(|failure| {
            panic!(
                "actual stopped native checkpoint refused: {}",
                failure.reason
            )
        });
    assert_eq!(after["accepted"], true, "{after}");
    assert_eq!(
        before["payload"]["checkpoint"],
        after["payload"]["checkpoint"]
    );
    assert!(
        host.performance
            .as_ref()
            .unwrap()
            .native_contact_admission_history()
            .is_empty()
    );
}
