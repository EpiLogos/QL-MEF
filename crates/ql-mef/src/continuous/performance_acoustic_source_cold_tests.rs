//! Actual source/worker replay tests. Positives never invent a native pulse or
//! private selected-Act lease; the native component test uses the real worker.
use super::*;
use crate::continuous::performance::native_source_support;
use crate::musical_performance_return::{ReturnContext, ReturnReference};
fn world_source() -> (
    crate::scene::WorldRequest,
    NativePerformanceReceivingSource,
    PerformanceConfig,
) {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let request: crate::scene::WorldRequest = serde_json::from_value(json!({
        "schema":crate::scene::WORLD_REQUEST,"instance_ref":"expression:physical-cold/world",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:physical-cold/current",
        "texture":[64,64],"units_per_metre":1.,"sky":sky,"start":{"tick12":3,"cycle":7,"aperture":9}
    }))
    .unwrap();
    let produced = crate::scene::world(request.clone()).unwrap();
    let input: crate::continuous::coupled::CoupledInput =
        serde_json::from_value(produced["event"].clone()).unwrap();
    let current = input.compose().unwrap();
    let r = |reference: &str| ReturnReference {
        reference: reference.into(),
        revision: "1".into(),
    };
    let source = NativePerformanceReceivingSource::world_source(
        request.clone(),
        ReturnContext {
            kind: "world".into(),
            context: r("native:physical-cold/world"),
            receiver: r("native:physical-cold/receiver"),
            source_occasion: None,
            protected_state: None,
            consent: None,
            private: false,
            required_assets: vec![],
        },
    )
    .unwrap();
    let (_, mut config) = native_source_support::config(false);
    config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    (request, source, config)
}
fn original_source() -> (
    PerformanceOwner,
    CoupledBasis,
    NativePerformanceReceivingSource,
) {
    let (request, source, config) = world_source();
    let produced = crate::scene::world(request).unwrap();
    let input: crate::continuous::coupled::CoupledInput =
        serde_json::from_value(produced["event"].clone()).unwrap();
    let current = input.compose().unwrap();
    let mut owner =
        PerformanceOwner::prepare(&current, "expression:physical-cold/world", config).unwrap();
    let receiving = source.admit_current(&mut owner, &current, 0).unwrap();
    owner.source_assets["receiving_source_inputs"] = receiving.source_inputs().clone();
    owner.source_assets["receiving_definition"] = receiving.definition().snapshot().unwrap();
    owner.source_assets["current_receiving"] = receiving.snapshot().unwrap();
    (owner, current, source)
}
fn authored(current: &CoupledBasis) -> AuthoredNativePhysicalEdit {
    AuthoredNativePhysicalEdit::Form {
        actor_ref: "native:physical-cold/author".into(),
        cause_ref: "native:physical-cold/actual-form".into(),
        occurrence_unix_ms: current.input.m3.occurrence_unix_ms + 1,
        receipt_unix_ms: current.input.m3.receipt_unix_ms + 1,
        operations: vec![
            M3Operation::ChangeLine { line: 0 },
            M3Operation::SetPose { pose: 2 },
        ],
    }
}
fn native_request(
    owner: &PerformanceOwner,
    descendant: &PreparedPhysicalSourceDescendant,
    request_id: u64,
    edit: &AuthoredNativePhysicalEdit,
) -> Value {
    let cursor = decimal(&descendant.source_record["native_sample"]).unwrap();
    let mut request = owner.raw("source-body-transition").unwrap();
    request["original_request_id"] = json!(request_id.to_string());
    request["expected_sample"] = json!(cursor.to_string());
    request["kind"] = json!(edit.kind());
    request["cause_ref"] = json!(edit.cause());
    request["before_packet"] = owner.packet().unwrap();
    request["before_native_basis"] = serde_json::to_value(owner.binding().native_basis()).unwrap();
    request["after_packet"] = descendant.after_owner.packet().unwrap();
    request["actual_after_packet"] = descendant.after_owner.packet().unwrap();
    request["actual_after_native_basis"] =
        serde_json::to_value(descendant.after_owner.binding().native_basis()).unwrap();
    request["receiving_admission"] = descendant.current_receiving.admission().snapshot().unwrap();
    request["current_receiving_admission"] =
        descendant.current_receiving.admission().snapshot().unwrap();
    request["native_catalog"] = json!(descendant.after_owner.cells);
    request["body_source"] = json!({"kind":"sourceForm","recipe_ref":descendant.after_owner.config.recipe.provenance.reference,
        "validated_m3_generation":descendant.after_owner.config.controls.expected_m3_generation.to_string()});
    request["before_acoustic"] = json!(descendant.before_acoustic);
    request["prepared_acoustic"] = json!(descendant.after_acoustic);
    request["current_acoustic"] = json!(descendant.after_acoustic);
    request
}
fn acoustic_source(source: NativePerformanceReceivingSource) -> NativePerformanceReceivingSource {
    source
        .with_acoustic_configuration(AcousticConfiguration {
            schema: "ql.native-acoustic-receiving-configuration/v1".into(),
            source_ref: "native:physical-cold/pickup".into(),
            source_motion_ref: "native:physical-cold/emitter".into(),
            receiver_motion_ref: "native:physical-cold/receiver-motion".into(),
            policy_ref: "native:physical-cold/metric".into(),
            policy_revision: "1".into(),
            standing: "architecture-model".into(),
            revision: 1,
            source_translation_metres: [0.; 3],
            receiver_position_metres: [0., 0., 1.],
            receiver_forward: [0., 0., -1.],
            source_velocity_metres_per_second: [0.1, 0., 0.],
            receiver_velocity_metres_per_second: [0.; 3],
            speed_metres_per_second: 340.,
            minimum_distance_metres: 1.,
            directivity: AcousticDirectivity::Omnidirectional,
            propagation_delay: true,
            span_samples: 480_000,
        })
        .unwrap()
}
fn actual_component_render(
    owner: &mut PerformanceOwner,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
) -> Value {
    use sha2::{Digest, Sha256};
    let reading = owner.reading().unwrap();
    let full_source = serde_json::to_vec(owner.source_assets()).unwrap();
    let digest = format!("sha256:{:x}", Sha256::digest(&full_source));
    let mut request = owner.raw("offline-render").unwrap();
    request["frames"] = json!(512);
    request["scope"] = json!({"schema":"ql.native-offline-render-scope/v1",
        "session_ref":owner.config.session_ref,"scene_ref":"native:physical-cold/actual-component-scene",
        "performance_revision":"1","performance_digest":digest,
        "basis_seal":format!("native:physical-cold/source-body/{}",owner.config.controls.body_revision),
        "event_prefix_seal":format!("native:physical-cold/admitted/{}",decimal(&reading["accepted_sequence"]).unwrap()),
        "checkpoint_ref":format!("native:physical-cold/before-render/{}",decimal(&reading["samples_elapsed"]).unwrap()),
        "expected_source":owner.binding().determination()["identity"],"expected_body_revision":reading["scope"]["body_revision"],
        "expected_cursor":reading["samples_elapsed"],"expected_accepted_sequence":reading["accepted_sequence"]});
    let pulse = scene
        .session_mut()
        .performance_exchange_retained(&request)
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
    assert_eq!(pulse["accepted"], true, "{pulse}");
    owner.validate_reply(&pulse).unwrap();
    owner.last = Some(pulse.clone());
    pulse
}
fn actual_component_checkpoint(
    owner: &mut PerformanceOwner,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
) -> Value {
    let pulse = scene
        .session_mut()
        .performance_exchange_retained(&owner.raw("checkpoint").unwrap())
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
    assert_eq!(pulse["accepted"], true, "{pulse}");
    owner.validate_reply(&pulse).unwrap();
    owner.last = Some(pulse.clone());
    pulse
}

#[test]
fn actual_acoustic_source_producer_preserves_world_body_origin_and_separate_epochs() {
    let (mut owner, current, before_source) = original_source();
    let original_assets = owner.source_assets().clone();
    let after_source = acoustic_source(before_source.clone());
    let install = owner
        .prepare_acoustic_installation_descendant_at(&current, &after_source, 9, 512)
        .unwrap();
    assert_eq!(install.record["before_acoustic"], Value::Null);
    assert_eq!(install.record["kind"], "install");
    assert_eq!(
        install.record["after_current_receiving"]["native_admission"]["operation"]["native_sample"],
        "512"
    );
    assert_eq!(install.after.packet()["history_origin_sample"], "512");
    assert_eq!(owner.source_assets(), &original_assets);
    owner.source_assets = install.after_assets;
    let installed = owner.source_assets().clone();
    let mut config = after_source.acoustic_configuration().unwrap().clone();
    config.revision = 2;
    config.source_motion_ref = "native:physical-cold/second-source-motion".into();
    config.source_translation_metres = [0.2, 0., 0.];
    config.source_velocity_metres_per_second = [-0.1, 0., 0.];
    config.receiver_position_metres = [0.5, 0., 1.];
    config.receiver_velocity_metres_per_second = [0.05, 0., 0.];
    let after = after_source
        .clone()
        .with_acoustic_configuration(config)
        .unwrap();
    let changed = owner
        .prepare_acoustic_source_descendant_at(&current, &after, 13, 1024)
        .unwrap();
    assert_eq!(changed.record["kind"], "replace");
    assert_eq!(changed.original.packet()["origin_sample"], "512");
    assert_eq!(changed.after.packet()["origin_sample"], "1024");
    assert_eq!(changed.after.packet()["history_origin_sample"], "512");
    assert_eq!(
        changed.after.packet()["source_body"],
        changed.original.packet()["source_body"]
    );
    assert_eq!(
        changed.record["native_preparation"],
        owner.packet().unwrap()
    );
    assert_eq!(
        changed.after_assets["original_native_input"],
        original_assets["original_native_input"]
    );
    assert_eq!(
        changed.after_assets["acoustic_transition_history"][0],
        install.record
    );
    assert_eq!(
        changed.after_assets["acoustic_transition_history"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(owner.source_assets(), &installed);
    assert!(owner.reading().is_none());
    for (request, sample) in [(0, 1024), (9, 1024), (13, 511)] {
        assert!(
            owner
                .prepare_acoustic_source_descendant_at(&current, &after, request, sample)
                .is_err()
        );
    }
    assert!(
        PerformanceOwner::replay_cold_native_acoustic_source(
            &current,
            "expression:physical-cold/world",
            &after,
            &changed.after_assets,
            &[],
            &[]
        )
        .is_err(),
        "source records cannot invent native applications"
    );
}

fn bit_exact(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(key, value)| b.get(key).is_some_and(|other| bit_exact(value, other)))
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| bit_exact(a, b))
        }
        (Value::Number(a), Value::Number(b)) if a.is_f64() || b.is_f64() => a
            .as_f64()
            .zip(b.as_f64())
            .is_some_and(|(a, b)| a.to_bits() == b.to_bits()),
        _ => left == right,
    }
}
fn component_exchange(
    owner: &mut PerformanceOwner,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
    request: &Value,
) -> Value {
    let pulse = scene
        .session_mut()
        .performance_exchange_retained(request)
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
    owner.validate_reply(&pulse).unwrap();
    owner.last = Some(pulse.clone());
    pulse
}
fn component_acoustic(
    owner: &mut PerformanceOwner,
    current: &CoupledBasis,
    before_source: &NativePerformanceReceivingSource,
    after_source: &NativePerformanceReceivingSource,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
    ordinal: u64,
) -> Value {
    let cursor = decimal(&owner.reading().unwrap()["samples_elapsed"]).unwrap();
    let (record, assets, packet) = if before_source.acoustic_configuration().is_none() {
        let candidate = owner
            .prepare_stopped_acoustic_installation_assets_at_request(
                current,
                before_source,
                after_source,
                ordinal,
            )
            .unwrap();
        (
            candidate.source_transition_record().unwrap().clone(),
            candidate.source_assets().clone(),
            candidate.preparation().packet().clone(),
        )
    } else {
        let candidate = owner
            .prepare_stopped_acoustic_receiver_assets_at_request(current, after_source, ordinal)
            .unwrap();
        owner
            .validate_stopped_acoustic_receiver_candidate(current, after_source, &candidate)
            .unwrap();
        (
            candidate.source_transition_record().unwrap().clone(),
            candidate.source_assets().clone(),
            candidate.preparation().packet().clone(),
        )
    };
    let operation = if record["kind"] == "install" {
        "receiving-transport-install"
    } else {
        "receiving-transport-replace"
    };
    let mut request = owner.raw(operation).unwrap();
    request["prepared_acoustic"] = packet.clone();
    request["current_acoustic"] = packet;
    request["expected_sample"] = json!(cursor.to_string());
    if operation == "receiving-transport-replace" {
        request["before_acoustic"] = record["before_acoustic"].clone();
    }
    // Numerical native worker component: no private Scene/Act lease is minted.
    // Production reaches the same exchange only through Root/C's opaque CAS.
    let before_cut = actual_component_checkpoint(owner, scene);
    for (path, wrong) in [
        ("/expected_sample", json!((cursor + 1).to_string())),
        (
            "/current_acoustic/source_body/source_coordinate/face",
            json!("wrong-face"),
        ),
        (
            "/prepared_acoustic/context/context/reference",
            json!("native:context-leak"),
        ),
    ] {
        let mut invalid = request.clone();
        *invalid.pointer_mut(path).unwrap() = wrong;
        let refusal = component_exchange(owner, scene, &invalid);
        assert_eq!(refusal["accepted"], false, "{path}: {refusal}");
        let unchanged = actual_component_checkpoint(owner, scene);
        assert_eq!(
            unchanged["payload"]["checkpoint"], before_cut["payload"]["checkpoint"],
            "{path}"
        );
    }
    let pulse = component_exchange(owner, scene, &request);
    assert_eq!(pulse["accepted"], true, "{pulse}");
    validate_manifest(
        &pulse["reading"]["receiving_transport"]["manifest"],
        &record["after_acoustic"],
        owner,
    )
    .unwrap();
    assert_eq!(pulse["reading"]["samples_elapsed"], record["native_sample"]);
    owner
        .acoustic_source_history
        .push(json!({"source":record,"native_application":pulse}));
    owner.source_assets = assets;
    pulse
}
fn component_physical(
    owner: &mut PerformanceOwner,
    current: &mut CoupledBasis,
    source: &NativePerformanceReceivingSource,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
    ordinal: u64,
    edit: &AuthoredNativePhysicalEdit,
) -> Value {
    let cursor = decimal(&owner.reading().unwrap()["samples_elapsed"]).unwrap();
    let mut next = owner
        .prepare_physical_source_descendant_at(current, source, ordinal, edit, cursor)
        .unwrap();
    let request = native_request(owner, &next, ordinal, edit);
    let pulse = scene
        .session_mut()
        .performance_exchange_retained(&request)
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
    assert_eq!(pulse["accepted"], true, "{pulse}");
    next.after_owner.validate_reply(&pulse).unwrap();
    next.after_owner.physical_source_history = owner
        .physical_source_history
        .iter()
        .map(|row| NativePhysicalTransitionRecord {
            source: row.source.clone(),
            application: row.application.clone(),
        })
        .collect();
    next.after_owner
        .physical_source_history
        .push(NativePhysicalTransitionRecord {
            source: next.source_record,
            application: pulse.clone(),
        });
    next.after_owner.acoustic_source_history = owner.acoustic_source_history.clone();
    next.after_owner.last = Some(pulse.clone());
    *owner = next.after_owner;
    *current = next.after_current;
    pulse
}
#[test]
#[ignore = "requires actual matching normal-floor native worker; no C source/Act lease or native pulse is fabricated"]
fn actual_m4_install_move_body_material_cold_replay_preserves_pcm_ring_and_all_source_epochs() {
    use std::{path::PathBuf, time::Duration};
    let worker = PathBuf::from(
        std::env::var("QL_NATIVE_FIELD_WORKER").expect("actual matching native worker"),
    );
    let (request, mut source, config) = world_source();
    let produced = crate::scene::world(request).unwrap();
    let scene_config: crate::continuous::scene_field::SceneConfig =
        serde_json::from_value(produced["binding"]["host"].clone()).unwrap();
    let mut scene = crate::continuous::scene_field::SceneInstrument::open(
        &worker,
        scene_config.clone(),
        Duration::from_secs(20),
    )
    .unwrap();
    let original = scene.session().current_basis().clone();
    let mut current = original.clone();
    let mut owner =
        PerformanceOwner::prepare(&original, "expression:physical-cold/world", config).unwrap();
    owner
        .activate_with_current_receiving(&original, scene.session_mut(), &source)
        .unwrap();
    let born = owner.source_assets().clone();
    let cell = owner.cells.iter().find(|c| c["available"] == true).unwrap();
    let note = owner
        .native_note_target(
            &original,
            KeyTouch {
                key: cell["key"].as_u64().unwrap() as u8,
                register: cell["register_octave"].as_i64().unwrap() as i8,
                member: 1,
                touch: 1,
                touch_ref: "native:physical-cold/m4-score-touch".into(),
            },
        )
        .unwrap();
    for event in [
        json!({"identity":owner.binding().determination()["identity"],"kind":0,"sequence":"1","sample":"0","touch":"1","value":0.8,"pitch_hz":note["hertz"],"parameter":0,"late_admitted":false,"has_note":true,"has_determination":false,"note":note}),
        json!({"identity":owner.binding().determination()["identity"],"kind":5,"sequence":"2","sample":"768","touch":"0","value":2.,"pitch_hz":0.,"parameter":0,"late_admitted":false,"has_note":false,"has_determination":false}),
        json!({"identity":owner.binding().determination()["identity"],"kind":1,"sequence":"3","sample":"2048","touch":"1","value":0.,"pitch_hz":0.,"parameter":0,"late_admitted":false,"has_note":false,"has_determination":false}),
    ] {
        let mut request = owner.raw("score").unwrap();
        request["event"] = event;
        request["input_ref"] = Value::Null;
        let pulse = component_exchange(&mut owner, &mut scene, &request);
        assert_eq!(pulse["accepted"], true, "{pulse}");
    }
    let mut chunks = vec![actual_component_render(&mut owner, &mut scene)];
    let edit = authored(&current);
    let form = component_physical(&mut owner, &mut current, &source, &mut scene, 9, &edit);
    chunks.push(actual_component_render(&mut owner, &mut scene));
    let mut cuts = vec![actual_component_checkpoint(&mut owner, &mut scene)];
    assert_eq!(
        cuts[0]["payload"]["checkpoint"]["native_pair"]["audio"]["has_receiving"],
        false
    );
    let installed_source = acoustic_source(source.clone());
    let install = component_acoustic(
        &mut owner,
        &current,
        &source,
        &installed_source,
        &mut scene,
        13,
    );
    source = installed_source;
    chunks.push(actual_component_render(&mut owner, &mut scene));
    cuts.push(actual_component_checkpoint(&mut owner, &mut scene));
    let mut moved = source.acoustic_configuration().unwrap().clone();
    moved.revision = 2;
    moved.source_motion_ref = "native:physical-cold/m4-emitter-2".into();
    moved.source_translation_metres = [0.2, 0., 0.];
    moved.source_velocity_metres_per_second = [-0.1, 0., 0.];
    moved.receiver_motion_ref = "native:physical-cold/m4-receiver-2".into();
    moved.receiver_position_metres = [0.5, 0., 1.];
    moved.receiver_velocity_metres_per_second = [0.05, 0., 0.];
    let moved_source = source.clone().with_acoustic_configuration(moved).unwrap();
    let movement = component_acoustic(&mut owner, &current, &source, &moved_source, &mut scene, 17);
    source = moved_source;
    let edit = AuthoredNativePhysicalEdit::Material {
        cause_ref: "native:physical-cold/m4-material".into(),
        material: {
            let mut material = owner.config.controls.material.clone();
            material.young_modulus_pa *= 2.;
            material
        },
    };
    let material = component_physical(&mut owner, &mut current, &source, &mut scene, 19, &edit);
    chunks.push(actual_component_render(&mut owner, &mut scene));
    cuts.push(actual_component_checkpoint(&mut owner, &mut scene));
    let mut moved = source.acoustic_configuration().unwrap().clone();
    moved.revision = 3;
    moved.source_motion_ref = "native:physical-cold/m4-emitter-3".into();
    moved.source_translation_metres = [-0.1, 0.1, 0.];
    moved.source_velocity_metres_per_second = [0.2, 0., 0.];
    moved.receiver_motion_ref = "native:physical-cold/m4-receiver-3".into();
    moved.receiver_position_metres = [0.25, 0.1, 1.];
    moved.receiver_forward = [0., 0., -1.];
    let moved_source = source.clone().with_acoustic_configuration(moved).unwrap();
    let geometry = component_acoustic(&mut owner, &current, &source, &moved_source, &mut scene, 23);
    source = moved_source;
    let saved = actual_component_checkpoint(&mut owner, &mut scene);
    cuts.push(saved.clone());
    let expected = owner.source_assets().clone();
    let physical = owner.native_physical_source_history();
    let acoustic = owner.native_acoustic_source_history();
    let replay = || {
        PerformanceOwner::replay_cold_native_acoustic_source(
            &original,
            "expression:physical-cold/world",
            &source,
            &expected,
            &physical,
            &acoustic,
        )
        .unwrap()
    };
    let replayed = replay();
    assert_eq!(replayed.frames().len(), 6);
    assert_eq!(replayed.frames()[0].owner().source_assets(), &born);
    assert_eq!(replayed.final_frame().owner().source_assets(), &expected);
    assert_eq!(
        replayed
            .final_frame()
            .owner()
            .native_acoustic_source_history(),
        acoustic
    );
    assert_eq!(
        replayed
            .final_frame()
            .owner()
            .native_physical_source_history(),
        physical
    );
    let wire = serde_json::to_string(&saved["payload"]["checkpoint"]).unwrap();
    let verify = |candidate: &str| {
        crate::continuous::performance::acoustic_history::verify_dated_source_history(
            &replayed,
            &source,
            "expression:physical-cold/world",
            candidate,
        )
    };
    let history = verify(&wire).unwrap();
    assert_eq!(history["segments"].as_array().unwrap().len(), 3);
    assert_eq!(history["segments"][0]["effective_sample"], "1024");
    assert_eq!(history["segments"][1]["effective_sample"], "1536");
    assert_eq!(history["segments"][2]["effective_sample"], "2048");
    assert_eq!(history["segments"][1]["body_revision"], "3");
    let ring =
        &saved["payload"]["checkpoint"]["native_pair"]["audio"]["receiving"]["history_linear"];
    assert!(
        ring.as_array()
            .unwrap()
            .iter()
            .any(|n| n.as_f64().unwrap() != 0.),
        "real held note must excite the actual retained native ring"
    );
    let mut frames = Vec::new();
    for frame in replayed.frames() {
        let historical_source = frame.receiving_source(&source).unwrap();
        let returned = frame.prepare_return(&historical_source, 1).unwrap();
        frames.push(json!({"source_sample":frame.source_sample().to_string(),"return":returned.snapshot().unwrap(),"basis":returned.expression_basis().unwrap(),"pitches":returned.expression_pitches(0).unwrap(),"source_assets":frame.owner().source_assets(),"native_physical_source_history":frame.owner().native_physical_source_history(),"native_acoustic_source_history":frame.owner().native_acoustic_source_history()}));
    }
    // The same body/M3 basis can have distinct acoustic epochs. Require full
    // source bundles and both native sidecars rather than a duplicated basis.
    assert_eq!(frames[1]["basis"], frames[2]["basis"]);
    assert_ne!(frames[1]["source_assets"], frames[2]["source_assets"]);
    for (index, field, wrong) in [
        (0, "effective_sample", json!("0")),
        (0, "trajectory_origin_sample", json!("0")),
        (1, "eigenbasis", json!("native:stale/eigenbasis")),
        (1, "pratibimba", json!(true)),
        (2, "source_motion", json!("native:lost/source")),
        (2, "anchor_metres", json!([0., 0., 0.])),
        (2, "velocity_metres_per_second", json!([0., 0., 0.])),
    ] {
        let mut invalid = saved["payload"]["checkpoint"].clone();
        invalid["native_pair"]["audio"]["receiving"]["source_history"]["segments"][index][field] =
            wrong;
        assert!(
            verify(&serde_json::to_string(&invalid).unwrap()).is_err(),
            "segment {index} {field}"
        );
        assert_eq!(verify(&wire).unwrap(), history);
    }
    for path in [
        "/acoustic_transition_history/0/before_acoustic",
        "/acoustic_transition_history/1/after_source_context",
        "/acoustic_transition_history/2/native_sample",
        "/current_receiving",
        "/original_native_input",
    ] {
        let mut invalid = expected.clone();
        *invalid.pointer_mut(path).unwrap() = json!("lost");
        assert!(
            PerformanceOwner::replay_cold_native_acoustic_source(
                &original,
                "expression:physical-cold/world",
                &source,
                &invalid,
                &physical,
                &acoustic
            )
            .is_err(),
            "{path}"
        );
        assert_eq!(replay().final_frame().owner().source_assets(), &expected);
    }
    for path in [
        "/native_application/reading/receiving_transport/manifest/context",
        "/native_application/payload/receiving_replacement/before_manifest/eigenbasis",
        "/native_application/reading/samples_elapsed",
    ] {
        let mut invalid = acoustic.clone();
        *invalid[1].pointer_mut(path).unwrap() = json!("wrong-owner");
        assert!(
            PerformanceOwner::replay_cold_native_acoustic_source(
                &original,
                "expression:physical-cold/world",
                &source,
                &expected,
                &physical,
                &invalid
            )
            .is_err(),
            "{path}"
        );
        assert_eq!(replay().final_frame().owner().source_assets(), &expected);
    }
    assert!(
        PerformanceOwner::replay_cold_native_acoustic_source(
            &original,
            "expression:physical-cold/world",
            &source,
            &expected,
            &physical,
            &acoustic[1..]
        )
        .is_err()
    );
    let mut reversed = acoustic.clone();
    reversed.reverse();
    assert!(
        PerformanceOwner::replay_cold_native_acoustic_source(
            &original,
            "expression:physical-cold/world",
            &source,
            &expected,
            &physical,
            &reversed
        )
        .is_err()
    );
    assert!(
        PerformanceOwner::replay_cold_native_acoustic_source(
            &current,
            "expression:physical-cold/world",
            &source,
            &expected,
            &physical,
            &acoustic
        )
        .is_err()
    );
    // Actual cold native numerical reinstallation/restoration: this component
    // owns no C permission, while C54 separately repeats the private Act path.
    let mut fresh_scene = crate::continuous::scene_field::SceneInstrument::open(
        &worker,
        scene_config,
        Duration::from_secs(20),
    )
    .unwrap();
    let (mut fresh, fresh_current) = replay().into_final();
    let saved_assets = fresh.source_assets().clone();
    fresh
        .activate_with_current_receiving(&fresh_current, fresh_scene.session_mut(), &source)
        .unwrap();
    let admitted = source
        .prepare_current(&fresh, &fresh_current, 2048)
        .unwrap();
    let mut restore = fresh.raw("restore-current-receiving").unwrap();
    restore["original_checkpoint_wire"] = json!(wire);
    restore["expected_cursor"] = json!("0");
    restore["transaction_ref"] = json!("native:physical-cold/m4-cold-restore");
    restore["checkpoint_ref"] = json!("native:physical-cold/m4-saved");
    restore["current_source_packet"] = fresh.packet().unwrap();
    restore["actual_native_basis"] = serde_json::to_value(fresh.binding().native_basis()).unwrap();
    restore["receiving_admission"] = admitted.admission().snapshot().unwrap();
    restore["current_receiving_admission"] = admitted.admission().snapshot().unwrap();
    restore["current_receiving"] = admitted.snapshot().unwrap();
    restore["native_catalog"] = json!(fresh.cells);
    restore["original_saved_acoustic"] = saved_assets["acoustic_receiving"]["packet"].clone();
    let restoration = component_exchange(&mut fresh, &mut fresh_scene, &restore);
    assert_eq!(restoration["accepted"], true, "{restoration}");
    fresh.source_assets = saved_assets;
    let restored = actual_component_checkpoint(&mut fresh, &mut fresh_scene);
    assert!(
        bit_exact(
            &restored["payload"]["checkpoint"]["native_pair"]["physical"],
            &saved["payload"]["checkpoint"]["native_pair"]["physical"]
        ),
        "actual cold physical float bits differ"
    );
    assert!(
        bit_exact(
            &restored["payload"]["checkpoint"]["native_pair"]["audio"]["receiving"],
            &saved["payload"]["checkpoint"]["native_pair"]["audio"]["receiving"]
        ),
        "actual cold receiving float bits differ"
    );
    let live_chunk = actual_component_render(&mut owner, &mut scene);
    let reopened_chunk = actual_component_render(&mut fresh, &mut fresh_scene);
    assert!(
        bit_exact(
            &live_chunk["payload"]["chunk"]["interleaved_f32"],
            &reopened_chunk["payload"]["chunk"]["interleaved_f32"]
        ),
        "actual cold native PCM bits differ"
    );
    let live_after = actual_component_checkpoint(&mut owner, &mut scene);
    let reopened_after = actual_component_checkpoint(&mut fresh, &mut fresh_scene);
    assert!(
        bit_exact(
            &live_after["payload"]["checkpoint"]["native_pair"]["physical"],
            &reopened_after["payload"]["checkpoint"]["native_pair"]["physical"]
        ),
        "actual cold physical float bits differ"
    );
    assert!(
        bit_exact(
            &live_after["payload"]["checkpoint"]["native_pair"]["audio"]["receiving"],
            &reopened_after["payload"]["checkpoint"]["native_pair"]["audio"]["receiving"]
        ),
        "actual cold receiving float bits differ"
    );
    if let Some(path) = std::env::var_os("QL_NATIVE_ACOUSTIC_SOURCE_REPLAY_ARTIFACT") {
        std::fs::write(path,serde_json::to_vec_pretty(&json!({"schema":"ql.actual-native-acoustic-source-replay-component/v1","origin":born,"current":expected,"native_physical_source_history":physical,"native_acoustic_source_history":acoustic,"frames":frames,"native_checkpoints":cuts,"native_pcm_chunks":chunks,"actual_applications":[form,install,movement,material,geometry],"qualified_numerical_source_history":history,"cold_trial":{"restore":restoration,"before":restored,"live_chunk":live_chunk,"reopened_chunk":reopened_chunk,"live_after":live_after,"reopened_after":reopened_after},"scope":"actual retained P/M4 install, source/receiver edits, Form/material and cold bitexact native component; no Scene/Act permission or completed app claim"})).unwrap()).unwrap();
    }
}
