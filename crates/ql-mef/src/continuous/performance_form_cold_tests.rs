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
#[test]
fn historical_source_compiler_is_actual_form_and_material_with_no_runtime_readback() {
    let (owner, original, source) = original_source();
    let before = owner.source_assets().clone();
    let first = owner
        .prepare_physical_source_descendant_at(&original, &source, 9, &authored(&original), 512)
        .unwrap();
    assert_eq!(
        first.source_record["native_m3_receipt"]["status"],
        "applied"
    );
    assert_eq!(
        first.source_record["before_current_input"],
        serde_json::to_value(&original.input).unwrap()
    );
    assert_eq!(first.source_record["after_m3"], first.after_current.m3);
    first
        .current_receiving
        .validate_current(&source, &first.after_owner, &first.after_current, 512)
        .unwrap();
    assert_eq!(
        first.after_owner.source_assets()["original_native_input"],
        before["original_native_input"]
    );
    assert_eq!(
        first.after_owner.source_assets()["physical_consumer_projection"],
        before["physical_consumer_projection"]
    );
    let mut material = first.after_owner.config.controls.material.clone();
    material.young_modulus_pa *= 4.;
    let second = first
        .after_owner
        .prepare_physical_source_descendant_at(
            &first.after_current,
            &source,
            13,
            &AuthoredNativePhysicalEdit::Material {
                cause_ref: "native:physical-cold/actual-material".into(),
                material,
            },
            1024,
        )
        .unwrap();
    assert_eq!(second.after_owner.config.controls.body_revision, 3);
    assert_eq!(
        second.after_owner.source_assets()["physical_transition_history"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        serde_json::to_value(&second.after_current).unwrap(),
        serde_json::to_value(&first.after_current).unwrap()
    );
    assert_eq!(
        second
            .after_owner
            .binding()
            .physical_body()
            .request()
            .geometry,
        first
            .after_owner
            .binding()
            .physical_body()
            .request()
            .geometry
    );
    assert_eq!(
        second.after_owner.binding().notes(),
        owner.binding().notes()
    );
    assert!(owner.reading().is_none());
    assert!(first.after_owner.reading().is_none());
    assert!(second.after_owner.reading().is_none());
    assert_eq!(owner.source_assets(), &before);
    assert!(
        PerformanceOwner::replay_cold_native_physical_source(
            &original,
            "expression:physical-cold/world",
            &source,
            second.after_owner.source_assets(),
            &[]
        )
        .is_err(),
        "source records alone cannot imply native applications"
    );
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
#[ignore = "requires actual same normal-floor native worker; no lease or native pulse is fabricated"]
fn actual_stopped_form_material_applications_cold_replay_full_original_owner_and_detect_loss() {
    use std::{path::PathBuf, time::Duration};
    let worker = PathBuf::from(
        std::env::var("QL_NATIVE_FIELD_WORKER").expect("actual matching native worker"),
    );
    let (request, source, config) = world_source();
    let source = acoustic_source(source);
    let produced = crate::scene::world(request).unwrap();
    let scene_config: crate::continuous::scene_field::SceneConfig =
        serde_json::from_value(produced["binding"]["host"].clone()).unwrap();
    let mut scene = crate::continuous::scene_field::SceneInstrument::open(
        &worker,
        scene_config,
        Duration::from_secs(20),
    )
    .unwrap();
    let original = scene.session().current_basis().clone();
    let mut owner =
        PerformanceOwner::prepare(&original, "expression:physical-cold/world", config).unwrap();
    owner
        .activate_with_current_receiving(&original, scene.session_mut(), &source)
        .unwrap();
    // Use the same native acoustic producer and actual stopped worker owner.
    // This component test has no C/Act grant; production install remains private.
    let acoustic = owner
        .prepare_acoustic_receiving_segment(&original, &source, 0, 0)
        .unwrap();
    let mut install = owner.raw("receiving-transport-install").unwrap();
    install["prepared_acoustic"] = acoustic.packet().clone();
    install["current_acoustic"] = acoustic.packet().clone();
    install["expected_sample"] = json!("0");
    let installed = scene
        .session_mut()
        .performance_exchange_retained(&install)
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
    assert_eq!(installed["accepted"], true, "{installed}");
    owner.validate_reply(&installed).unwrap();
    owner.last = Some(installed);
    owner.source_assets["acoustic_receiving"] = acoustic.snapshot();
    let born = owner.source_assets().clone();
    let origin_context = born["source_context"].clone();
    // Resolve the real source key, then enqueue authored native score inputs.
    let cell = owner.cells.iter().find(|c| c["available"] == true).unwrap();
    let note = owner
        .native_note_target(
            &original,
            KeyTouch {
                key: cell["key"].as_u64().unwrap() as u8,
                register: cell["register_octave"].as_i64().unwrap() as i8,
                member: 1,
                touch: 1,
                touch_ref: "native:physical-cold/actual-score-touch".into(),
            },
        )
        .unwrap();
    for event in [
        json!({"identity":owner.binding().determination()["identity"],"kind":0,"sequence":"1","sample":"0","touch":"1", "value":0.8,"pitch_hz":note["hertz"],"parameter":0,"late_admitted":false,"has_note":true,"has_determination":false,"note":note}),
        json!({"identity":owner.binding().determination()["identity"],"kind":5,"sequence":"2","sample":"768","touch":"0", "value":2.0,"pitch_hz":0.0,"parameter":0,"late_admitted":false,"has_note":false,"has_determination":false}),
        json!({"identity":owner.binding().determination()["identity"],"kind":1,"sequence":"3","sample":"1280","touch":"1", "value":0.0,"pitch_hz":0.0,"parameter":0,"late_admitted":false,"has_note":false,"has_determination":false}),
    ] {
        let mut queued = owner.raw("score").unwrap();
        queued["event"] = event;
        queued["input_ref"] = Value::Null;
        let pulse = scene
            .session_mut()
            .performance_exchange_retained(&queued)
            .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
        assert_eq!(pulse["accepted"], true, "{pulse}");
        owner.validate_reply(&pulse).unwrap();
        owner.last = Some(pulse);
    }
    let mut native_chunks = vec![actual_component_render(&mut owner, &mut scene)];
    let mut checkpoints = vec![actual_component_checkpoint(&mut owner, &mut scene)];
    let mut current = original.clone();
    let mut actual_history = Vec::new();
    for (request_id, edit) in [
        (9, authored(&original)),
        (
            13,
            AuthoredNativePhysicalEdit::Material {
                cause_ref: "native:physical-cold/actual-material".into(),
                material: {
                    let mut material = owner.config.controls.material.clone();
                    material.young_modulus_pa *= 4.;
                    material
                },
            },
        ),
    ] {
        let mut next = owner
            .prepare_physical_source_descendant_at(
                &current,
                &source,
                request_id,
                &edit,
                decimal(&owner.reading().unwrap()["samples_elapsed"]).unwrap(),
            )
            .unwrap();
        let request = native_request(&owner, &next, request_id, &edit);
        let pulse = scene
            .session_mut()
            .performance_exchange_retained(&request)
            .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
        assert_eq!(pulse["accepted"], true, "{pulse}");
        next.after_owner.validate_reply(&pulse).unwrap();
        actual_history.push(json!({"source":next.source_record,"native_application":pulse}));
        next.after_owner.last = Some(pulse);
        owner = next.after_owner;
        current = next.after_current;
        checkpoints.push(actual_component_checkpoint(&mut owner, &mut scene));
        native_chunks.push(actual_component_render(&mut owner, &mut scene));
        checkpoints.push(actual_component_checkpoint(&mut owner, &mut scene));
    }
    let expected = owner.source_assets().clone();
    let replayed = PerformanceOwner::replay_cold_native_physical_source(
        &original,
        "expression:physical-cold/world",
        &source,
        &expected,
        &actual_history,
    )
    .unwrap();
    assert_eq!(replayed.frames().len(), 3);
    assert_eq!(replayed.frames()[0].owner().source_assets(), &born);
    assert_eq!(replayed.final_frame().owner().source_assets(), &expected);
    assert_eq!(
        replayed
            .final_frame()
            .owner()
            .native_physical_source_history(),
        actual_history
    );
    assert_eq!(
        serde_json::to_value(replayed.final_frame().current()).unwrap(),
        serde_json::to_value(&current).unwrap()
    );
    for (index, frame) in replayed.frames().iter().enumerate() {
        assert_eq!(
            frame.owner().config.controls.body_revision,
            index as u64 + 1
        );
        assert!(frame.owner().reading().is_none());
        source
            .prepare_current(frame.owner(), frame.current(), frame.source_sample())
            .unwrap();
    }
    for path in [
        "/physical_transition_history/0/native_m3_receipt/after",
        "/physical_transition_history/0/after_native_preparation",
        "/physical_transition_history/1/before_configuration",
        "/operative_native_input",
        "/original_native_input",
        "/receiving_source_inputs",
        "/physical_consumer_projection",
    ] {
        let mut lost = expected.clone();
        *lost.pointer_mut(path).unwrap() = Value::Null;
        assert!(
            PerformanceOwner::replay_cold_native_physical_source(
                &original,
                "expression:physical-cold/world",
                &source,
                &lost,
                &actual_history
            )
            .is_err(),
            "{path}"
        );
    }
    for path in [
        "/native_application/payload/physical_transition/original_native_request_id",
        "/native_application/payload/physical_transition/before_native_preparation",
        "/native_application/reading/physical/source_coordinate",
        "/native_application/payload/body_descriptor/physical_preparation",
    ] {
        let mut lost = actual_history.clone();
        *lost[0].pointer_mut(path).unwrap() = Value::Null;
        assert!(
            PerformanceOwner::replay_cold_native_physical_source(
                &original,
                "expression:physical-cold/world",
                &source,
                &expected,
                &lost
            )
            .is_err(),
            "{path}"
        );
    }
    let mut reversed = actual_history.clone();
    reversed.reverse();
    assert!(
        PerformanceOwner::replay_cold_native_physical_source(
            &original,
            "expression:physical-cold/world",
            &source,
            &expected,
            &reversed
        )
        .is_err()
    );
    assert!(
        PerformanceOwner::replay_cold_native_physical_source(
            &current,
            "expression:physical-cold/world",
            &source,
            &expected,
            &actual_history
        )
        .is_err(),
        "final current cannot become origin"
    );
    // Original classification and protected occasion stay bound to the origin;
    // only the actual preparation hash/operative body source changes.
    use sha2::{Digest, Sha256};
    let expected_original_hash = format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(&born["original_native_input"]).unwrap())
    );
    assert_eq!(
        origin_context["classified_original_input_sha256"],
        expected_original_hash
    );
    for (index, frame) in replayed.frames().iter().enumerate() {
        if index > 0 {
            assert_ne!(
                frame.owner().source_assets()["source_context"]["native_preparation_sha256"],
                origin_context["native_preparation_sha256"]
            );
        }
        for field in [
            "classified_original_input_sha256",
            "classifications",
            "original_occasion",
        ] {
            assert_eq!(
                frame.owner().source_assets()["source_context"][field],
                origin_context[field],
                "{field}"
            );
        }
    }
    let wire =
        serde_json::to_string(&checkpoints.last().unwrap()["payload"]["checkpoint"]).unwrap();
    let verify = |text: &str| {
        crate::continuous::performance::acoustic_history::verify_dated_source_history(
            &replayed,
            &source,
            "expression:physical-cold/world",
            text,
        )
    };
    let genuine = verify(&wire).unwrap();
    assert_eq!(genuine["segments"].as_array().unwrap().len(), 3);
    assert_eq!(genuine["segments"][0]["effective_sample"], "0");
    assert_eq!(genuine["segments"][1]["effective_sample"], "512");
    assert_eq!(genuine["segments"][2]["effective_sample"], "1024");
    let saved: Value = serde_json::from_str(&wire).unwrap();
    let ring = &saved["native_pair"]["audio"]["receiving"]["history_linear"];
    assert!(
        ring.as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_f64().unwrap() != 0.),
        "actual held note must produce a nonzero native P ring"
    );
    for index in 0..3 {
        for (field, wrong) in [
            ("effective_sample", json!("1")),
            ("trajectory_origin_sample", json!("1")),
            ("body_revision", json!("999")),
            ("source_generation", json!("999")),
            (
                "pratibimba",
                json!(!genuine["segments"][index]["pratibimba"].as_bool().unwrap()),
            ),
            ("source_coordinate", json!("native:wrong-face/source")),
            ("source_revision", json!("lost-revision")),
            ("preparation", json!("native:lost-descendant/preparation")),
            ("state", json!("native:stale/state")),
            ("eigenbasis", json!("native:wrong/eigenbasis")),
            ("source_motion", json!("native:context-leak/motion")),
            ("anchor_metres", json!([1., 2., 3.])),
            ("velocity_metres_per_second", json!([1., 2., 3.])),
        ] {
            let mut wrong_saved = saved.clone();
            wrong_saved["native_pair"]["audio"]["receiving"]["source_history"]["segments"][index]
                [field] = wrong;
            assert!(
                verify(&serde_json::to_string(&wrong_saved).unwrap()).is_err(),
                "segment {index} {field}"
            );
        }
    }
    for path in [
        "/native_pair/audio/receiving/source_history/segments",
        "/native_pair/audio/receiving/history_linear",
        "/native_pair/physical/identity/source_coordinate",
        "/native_pair/physical/basis/eigenbasis_identity",
    ] {
        let mut lost = saved.clone();
        *lost.pointer_mut(path).unwrap() = Value::Null;
        assert!(
            verify(&serde_json::to_string(&lost).unwrap()).is_err(),
            "{path}"
        );
    }
    let mut lossy = saved.clone();
    lossy["native_pair"]["audio"]["receiving"]["history_linear"][0] = json!(0.1);
    assert!(verify(&serde_json::to_string(&lossy).unwrap()).is_err());
    for (field, value) in [
        ("first_retained", json!(255)),
        ("date_units", json!("host-ticks")),
        ("anchor_units", json!("cm")),
        ("velocity_units", json!("km/s")),
    ] {
        let mut wrong = saved.clone();
        wrong["native_pair"]["audio"]["receiving"]["source_history"][field] = value;
        assert!(
            verify(&serde_json::to_string(&wrong).unwrap()).is_err(),
            "{field}"
        );
    }
    let mut oversized_ref = saved.clone();
    oversized_ref["native_pair"]["audio"]["receiving"]["source_history"]["segments"][0]["source_motion"] =
        json!("x".repeat(2049));
    assert!(verify(&serde_json::to_string(&oversized_ref).unwrap()).is_err());
    let mut extra = saved.clone();
    extra["native_pair"]["audio"]["receiving"]["source_history"]["segments"][0]["grant"] =
        json!(true);
    assert!(verify(&serde_json::to_string(&extra).unwrap()).is_err());
    let mut stale = saved.clone();
    stale["native_pair"]["audio"]["receiving"]["manifest"]["body_revision"] = json!("1");
    assert!(verify(&serde_json::to_string(&stale).unwrap()).is_err());
    // Repeat the complete untouched numerical/source compiler after mutations;
    // no failed case may poison the original replay or ring.
    assert_eq!(verify(&wire).unwrap(), genuine);
    let mut frame_assets = Vec::new();
    for frame in replayed.frames() {
        let returned = frame.prepare_return(&source, 1).unwrap();
        frame_assets.push(json!({"source_sample":frame.source_sample().to_string(),"return":returned.snapshot().unwrap(),
            "basis":returned.expression_basis().unwrap(),"pitches":returned.expression_pitches(0).unwrap(),
            "source_assets":frame.owner().source_assets(),"native_physical_source_history":frame.owner().native_physical_source_history()}));
    }
    // The callback moves only first_retained; physical compaction occurs only
    // at the next actual source transaction. Exercise both on the real owner.
    let mut late_chunks = Vec::new();
    while decimal(&owner.reading().unwrap()["samples_elapsed"]).unwrap() < 18_432 {
        late_chunks.push(actual_component_render(&mut owner, &mut scene));
    }
    let retained_cut = actual_component_checkpoint(&mut owner, &mut scene);
    let retained_wire = serde_json::to_string(&retained_cut["payload"]["checkpoint"]).unwrap();
    let retained_history = verify(&retained_wire).unwrap();
    assert_eq!(retained_history["segments"].as_array().unwrap().len(), 3);
    assert_eq!(retained_history["first_retained"], 2);
    // Two real body operations at the SAME actual stopped native sample.
    // Original ordered source receipts survive even though the first new
    // emitter row has no interval and is replaced by the second operation.
    let mut late_history = actual_history.clone();
    let mut late_edits = Vec::new();
    let late_form = authored(&current);
    let mut changed_material = owner.config.controls.material.clone();
    changed_material.young_modulus_pa *= 2.;
    for (ordinal, edit) in [
        (17, late_form),
        (
            19,
            AuthoredNativePhysicalEdit::Material {
                cause_ref: "native:physical-cold/same-sample-material".into(),
                material: changed_material,
            },
        ),
    ] {
        let mut next = owner
            .prepare_physical_source_descendant_at(&current, &source, ordinal, &edit, 18_432)
            .unwrap();
        let pulse = scene
            .session_mut()
            .performance_exchange_retained(&native_request(&owner, &next, ordinal, &edit))
            .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
        assert_eq!(pulse["accepted"], true, "{pulse}");
        next.after_owner.validate_reply(&pulse).unwrap();
        late_history.push(json!({"source":next.source_record,"native_application":pulse}));
        late_edits.push(pulse.clone());
        next.after_owner.last = Some(pulse);
        owner = next.after_owner;
        current = next.after_current;
    }
    let late_replayed = PerformanceOwner::replay_cold_native_physical_source(
        &original,
        "expression:physical-cold/world",
        &source,
        owner.source_assets(),
        &late_history,
    )
    .unwrap();
    let replaced_cut = actual_component_checkpoint(&mut owner, &mut scene);
    let replaced_wire = serde_json::to_string(&replaced_cut["payload"]["checkpoint"]).unwrap();
    let replaced_history =
        crate::continuous::performance::acoustic_history::verify_dated_source_history(
            &late_replayed,
            &source,
            "expression:physical-cold/world",
            &replaced_wire,
        )
        .unwrap();
    assert_eq!(late_replayed.frames().len(), 5);
    assert_eq!(replaced_history["segments"].as_array().unwrap().len(), 2);
    assert_eq!(replaced_history["segments"][0]["effective_sample"], "1024");
    assert_eq!(replaced_history["segments"][1]["effective_sample"], "18432");
    assert_eq!(replaced_history["segments"][1]["body_revision"], "5");
    assert_eq!(replaced_history["first_retained"], 0);
    // Whole original wire stays intact; valid f32 ring mutations require C's
    // exact selected-wire lease check, independently exercised by C52.
    assert_eq!(verify(&wire).unwrap(), genuine);
    if let Some(path) = std::env::var_os("QL_NATIVE_PHYSICAL_SOURCE_REPLAY_ARTIFACT") {
        std::fs::write(path,serde_json::to_vec_pretty(&json!({"schema":"ql.actual-native-physical-source-replay-component/v1","origin":born,"current":expected,"native_physical_source_history":actual_history,"frames":frame_assets,"native_checkpoints":checkpoints,"native_pcm_chunks":native_chunks,"qualified_numerical_source_history":genuine,"retention_trial":{"native_chunks":late_chunks,"before_compaction":retained_cut,"actual_same_sample_applications":late_edits,"full_native_source_history":late_history,"after_compaction":replaced_cut,"qualified_numerical_source_history":replaced_history},"scope":"actual source/worker Form512/material1024 and original-owner replay with native M4 ring; no private Scene/Act lease or installed application acceptance"})).unwrap()).unwrap();
    }
}

#[test]
fn actual_form_n9_admission_and_retained_acoustic_origin_are_independent_complete_sources() {
    let (request, source, config) = world_source();
    let produced = crate::scene::world(request).unwrap();
    let input: crate::continuous::coupled::CoupledInput =
        serde_json::from_value(produced["event"].clone()).unwrap();
    let original = input.compose().unwrap();
    let source = source
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
        .unwrap();
    let mut owner =
        PerformanceOwner::prepare(&original, "expression:physical-cold/world", config).unwrap();
    let admitted = source.admit_current(&mut owner, &original, 0).unwrap();
    owner.source_assets["receiving_source_inputs"] = admitted.source_inputs().clone();
    owner.source_assets["receiving_definition"] = admitted.definition().snapshot().unwrap();
    owner.source_assets["current_receiving"] = admitted.snapshot().unwrap();
    let acoustic = owner
        .prepare_acoustic_receiving_segment(&original, &source, 0, 0)
        .unwrap();
    owner.source_assets["acoustic_receiving"] = acoustic.snapshot();
    let mut after = owner
        .prepare_physical_source_descendant_at(&original, &source, 9, &authored(&original), 512)
        .unwrap();
    assert_eq!(
        after.after_owner.source_assets()["current_receiving"]["native_admission"]["operation"]["native_sample"],
        "512"
    );
    assert_eq!(
        after.after_owner.source_assets()["acoustic_receiving"]["packet"]["origin_sample"],
        "0"
    );
    assert_eq!(
        after.after_owner.source_assets()["acoustic_receiving"]["packet"]["history_origin_sample"],
        "0"
    );
    let assets = after.after_owner.source_assets().clone();
    let prepared = after
        .after_owner
        .prepare_retained_acoustic_sources(&after.after_current, &source, 768)
        .unwrap();
    assert_eq!(prepared.snapshot(), assets["acoustic_receiving"]);
    assert_eq!(
        after.after_owner.source_assets(),
        &assets,
        "source preparation cannot rewrite admission or receiver dates"
    );
    assert!(after.after_owner.reading().is_none());
    assert!(
        after
            .after_owner
            .prepare_retained_acoustic_sources(&after.after_current, &source, 511)
            .is_err()
    );
    // An independently valid N9 preparation at0 is still the wrong epoch for
    // a body whose actual complete typed Form producer is effective at512.
    let wrong = source
        .prepare_current(&after.after_owner, &after.after_current, 0)
        .unwrap();
    after.after_owner.source_assets["current_receiving"] = wrong.snapshot().unwrap();
    assert!(
        after
            .after_owner
            .prepare_retained_acoustic_sources(&after.after_current, &source, 768)
            .is_err()
    );
    after.after_owner.source_assets = assets.clone();
    for path in [
        "/physical_transition_history/0/after_native_preparation",
        "/physical_transition_history/0/after_current_input",
        "/source_context",
        "/receiving_source_inputs",
        "/receiving_definition",
        "/acoustic_receiving/current_receiving",
    ] {
        *after.after_owner.source_assets.pointer_mut(path).unwrap() = Value::Null;
        assert!(
            after
                .after_owner
                .prepare_retained_acoustic_sources(&after.after_current, &source, 768)
                .is_err(),
            "{path}"
        );
        after.after_owner.source_assets = assets.clone();
    }
    // A real later source admission remains separate from the original
    // retained receiver motion; it neither backdates nor restarts the path.
    let later = source
        .prepare_current(&after.after_owner, &after.after_current, 1024)
        .unwrap();
    after.after_owner.source_assets["current_receiving"] = later.snapshot().unwrap();
    let later_prepared = after
        .after_owner
        .prepare_retained_acoustic_sources(&after.after_current, &source, 1280)
        .unwrap();
    assert_eq!(later_prepared.snapshot(), assets["acoustic_receiving"]);
    assert_eq!(
        after.after_owner.source_assets()["current_receiving"]["native_admission"]["operation"]["native_sample"],
        "1024"
    );
    assert_eq!(later_prepared.packet()["origin_sample"], "0");
    assert_eq!(later_prepared.packet()["history_origin_sample"], "0");
}
