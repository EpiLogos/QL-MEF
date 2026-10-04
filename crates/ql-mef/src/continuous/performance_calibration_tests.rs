//! Actual default twelve-node instrument calibration. The original native
//! source, fixed headroom, sole P, exact controls and M4 path remain operative.
use super::super::*;
use super::*;
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;
use crate::musical_performance_return::{ReturnContext, ReturnReference};
use serde_json::{Value, json};

fn intent() -> AuthoredCurrentPerformancePreparation {
    AuthoredCurrentPerformancePreparation {
        schema: CURRENT_PREPARATION.into(),
        policy_ref: "policy:expressions/current-condition-cf-degree-order".into(),
        session_ref: "expression:calibration/current-instrument".into(),
        receipt_ref: "receipt:calibration/native-preparation".into(),
        projection_ref: "policy:expressions/current-metric-consumer".into(),
        relation: RelationConfig {
            family: "C".into(),
            pair_index: 1,
            degree: 3,
            expansion_side: None,
        },
        source_face: 1,
        physical_face: 1,
        mechanical_policy: CurrentMechanicalPolicy::DeclaredInstrumentDefault,
        source_choice: CurrentSourceChoice::RetainedCurrentConditionCfDegreeOrder,
        columns: 6,
        base_register: 0,
        transpose: 0,
    }
}
#[test]
fn declared_excitation_and_pickup_remain_unconstrained_across_all_native_forms_and_poses() {
    let original = native_source_support::config(false).0;
    let policy = instrument_default(original.m2_input.condition.as_ref().unwrap().tonic_hz);
    let mut constrained_legacy = 0;
    let mut cases = 0;
    for address in 0..64 {
        let mut request = original.input.m3.clone();
        request.address = address;
        request.pose = 0;
        let count = M3State::new(request.clone()).unwrap().fold().state_count();
        for pose in 0..count {
            request.pose = pose;
            let state = M3State::new(request.clone()).unwrap();
            let geometry =
                crate::source_form_body::source_form_geometry(&state, &policy.recipe).unwrap();
            for projection in [&policy.exciter, &policy.pickup] {
                let free = geometry
                    .nodes
                    .iter()
                    .zip(&projection.node_weights)
                    .map(|(node, weight)| {
                        (0..3)
                            .filter(|axis| !node.fixed[*axis])
                            .map(|axis| (weight * projection.axis[axis]).abs())
                            .sum::<f64>()
                    })
                    .sum::<f64>();
                assert!(
                    free > 0.,
                    "native form {address} pose {pose} disconnects default projection"
                );
            }
            assert_eq!(geometry.nodes[10].identity, 11);
            assert!(!geometry.nodes[10].constituent.is_empty());
            constrained_legacy += usize::from(geometry.nodes[11].fixed[2]);
            cases += 1;
        }
    }
    assert!(
        constrained_legacy > 0,
        "original node12 Z defect must be exercised"
    );
    assert!(cases >= 64);
}
#[test]
fn current_calibration_command_has_no_caller_policy_value_source_or_ordinal() {
    let command: PerformanceCommand =
        serde_json::from_value(json!({"operation":"calibrate-current"})).unwrap();
    assert!(matches!(command, PerformanceCommand::CalibrateCurrent {}));
    for field in [
        "target_ref",
        "value",
        "policy",
        "source",
        "ordinal",
        "sample",
    ] {
        let mut supplied = json!({"operation":"calibrate-current"});
        supplied[field] = json!("caller-supplied");
        assert!(
            serde_json::from_value::<PerformanceCommand>(supplied).is_err(),
            "calibration accepted caller {field}"
        );
    }
    assert_eq!(intent().output_calibration_declaration()["required"], true);
    let mut explicit = intent();
    explicit.mechanical_policy = CurrentMechanicalPolicy::Explicit {
        mechanical: Box::new(instrument_default(440.)),
    };
    assert_eq!(explicit.output_calibration_declaration()["required"], false);
}

fn world() -> (
    Value,
    NativePerformanceReceivingSource,
    crate::continuous::scene_field::SceneConfig,
) {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    // Keep the exact raw native constructor input before typed decoding.
    let original_request = json!({"schema":crate::scene::WORLD_REQUEST,"instance_ref":"expression:calibration/world","event_ref":sky["snapshot_ref"],"subject_ref":"person:calibration/current","texture":[64,64],"units_per_metre":1.,"sky":sky,"start":{"tick12":3,"cycle":7,"aperture":9}});
    let request: crate::scene::WorldRequest =
        serde_json::from_value(original_request.clone()).unwrap();
    let r = |reference: &str| ReturnReference {
        reference: reference.into(),
        revision: "1".into(),
    };
    let source = NativePerformanceReceivingSource::world_source(
        request.clone(),
        ReturnContext {
            kind: "world".into(),
            context: r("native:calibration/world"),
            receiver: r("native:calibration/receiver"),
            source_occasion: None,
            protected_state: None,
            consent: None,
            private: false,
            required_assets: vec![],
        },
    )
    .unwrap()
    .with_acoustic_configuration(AcousticConfiguration {
        schema: "ql.native-acoustic-receiving-configuration/v1".into(),
        source_ref: "native:calibration/pickup".into(),
        source_motion_ref: "native:calibration/emitter".into(),
        receiver_motion_ref: "native:calibration/receiver-motion".into(),
        policy_ref: "native:calibration/metric".into(),
        policy_revision: "1".into(),
        standing: "architecture-model".into(),
        revision: 1,
        source_translation_metres: [0.; 3],
        receiver_position_metres: [0., 0., 1.],
        receiver_forward: [0., 0., -1.],
        source_velocity_metres_per_second: [0.; 3],
        receiver_velocity_metres_per_second: [0.; 3],
        speed_metres_per_second: 340.,
        minimum_distance_metres: 1.,
        directivity: AcousticDirectivity::Omnidirectional,
        propagation_delay: true,
        span_samples: 480_000,
    })
    .unwrap();
    let produced = crate::scene::world(request.clone()).unwrap();
    let config = serde_json::from_value(produced["binding"]["host"].clone()).unwrap();
    (original_request, source, config)
}
fn exchange(
    owner: &mut PerformanceOwner,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
    request: Value,
) -> Value {
    let pulse = scene
        .session_mut()
        .performance_exchange_retained(&request)
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.0, failure.1));
    assert_eq!(pulse["accepted"], true, "{pulse}");
    owner.validate_reply(&pulse).unwrap();
    owner.last = Some(pulse.clone());
    pulse
}
fn checkpoint(
    owner: &mut PerformanceOwner,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
) -> Value {
    let request = owner.raw("checkpoint").unwrap();
    exchange(owner, scene, request)
}
fn render(
    owner: &mut PerformanceOwner,
    scene: &mut crate::continuous::scene_field::SceneInstrument,
) -> Value {
    use sha2::{Digest, Sha256};
    let reading = owner.reading().unwrap();
    let digest = format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(owner.source_assets()).unwrap())
    );
    let mut request = owner.raw("offline-render").unwrap();
    request["frames"] = json!(512);
    request["scope"] = json!({"schema":"ql.native-offline-render-scope/v1","session_ref":owner.config.session_ref,"scene_ref":"native:calibration/source-world-scene","performance_revision":"1","performance_digest":digest,"basis_seal":"native:calibration/full-source-body","event_prefix_seal":format!("native:calibration/admitted/{}",reading["accepted_sequence"].as_str().unwrap()),"checkpoint_ref":format!("native:calibration/before-render/{}",reading["samples_elapsed"].as_str().unwrap()),"expected_source":owner.binding().determination()["identity"],"expected_body_revision":reading["scope"]["body_revision"],"expected_cursor":reading["samples_elapsed"],"expected_accepted_sequence":reading["accepted_sequence"]});
    exchange(owner, scene, request)
}
fn run_case(worker: &std::path::Path, legacy: bool) -> Value {
    use std::{collections::BTreeMap, time::Duration};
    let (world_request, source, scene_config) = world();
    let mut scene = crate::continuous::scene_field::SceneInstrument::open(
        worker,
        scene_config,
        Duration::from_secs(20),
    )
    .unwrap();
    let current = scene.session().current_basis().clone();
    let mut authored = intent();
    if legacy {
        let mut before = instrument_default(current.m2_input.condition.as_ref().unwrap().tonic_hz);
        before.exciter.node_weights.fill(0.);
        before.pickup.node_weights.fill(0.);
        before.exciter.node_weights[11] = 1.;
        before.pickup.node_weights[11] = 1.;
        authored.mechanical_policy = CurrentMechanicalPolicy::Explicit {
            mechanical: Box::new(before),
        };
    }
    let configuration =
        prepare_current_configuration(&current, "expression:calibration/world", authored.clone())
            .unwrap();
    let mut owner =
        PerformanceOwner::prepare(&current, "expression:calibration/world", configuration).unwrap();
    owner
        .activate_with_current_receiving(&current, scene.session_mut(), &source)
        .unwrap();
    let acoustic = owner
        .prepare_acoustic_receiving_source(&current, &source, 0)
        .unwrap();
    let mut request = owner.raw("receiving-transport-install").unwrap();
    request["prepared_acoustic"] = acoustic.packet().clone();
    request["current_acoustic"] = acoustic.packet().clone();
    request["expected_sample"] = json!("0");
    let installation = exchange(&mut owner, &mut scene, request);
    owner.source_assets["acoustic_receiving"] = acoustic.snapshot();
    // The real recording origin is kept before calibration; queued controls
    // belong to a separate pending cut, never a fabricated performed prefix.
    let born = checkpoint(&mut owner, &mut scene);
    let born_audio = &born["payload"]["checkpoint"]["native_pair"]["audio"];
    assert_eq!(born_audio["accepted_sequence"], "0");
    assert_eq!(born_audio["applied_application_ordinal"], "0");
    assert!(born["applications"].as_array().unwrap().is_empty());
    let direct_refusal = match owner.execute_with_original_pulse(
        &current,
        scene.session_mut(),
        PerformanceCommand::CalibrateCurrent {},
    ) {
        Ok(_) => panic!("PerformanceOwner cannot grant original Host preparation custody"),
        Err(refusal) => refusal,
    };
    assert!(direct_refusal.native_pulse.is_none());
    let no_origin =
        match crate::continuous::host::performance_calibration::from_retained_preparation(
            None,
            &mut owner,
            &current,
            scene.session_mut(),
        ) {
            Ok(_) => panic!("no original fresh authored preparation means no calibration grant"),
            Err(refusal) => refusal,
        };
    assert!(no_origin.native_pulse.is_none());
    let unchanged = checkpoint(&mut owner, &mut scene);
    assert_eq!(
        unchanged["payload"]["checkpoint"],
        born["payload"]["checkpoint"]
    );
    if !legacy {
        let mut wrong_authored = authored.clone();
        wrong_authored.session_ref.push_str("/wrong-source-intent");
        let failure = match wrong_authored.admit_declared_output_calibration(
            &mut owner,
            &current,
            scene.session_mut(),
        ) {
            Ok(_) => panic!("different full authored preparation must be refused"),
            Err(refusal) => refusal,
        };
        assert!(failure.native_pulse.is_none());
        let after_refusal = checkpoint(&mut owner, &mut scene);
        assert_eq!(
            after_refusal["payload"]["checkpoint"],
            born["payload"]["checkpoint"]
        );
    }
    let force_control = if legacy {
        let not_applicable =
            match crate::continuous::host::performance_calibration::from_retained_preparation(
                Some(&authored),
                &mut owner,
                &current,
                scene.session_mut(),
            ) {
                Ok(_) => panic!("actual explicit policy does not admit declared calibration"),
                Err(refusal) => refusal,
            };
        assert!(not_applicable.native_pulse.is_none());
        owner
            .execute_with_original_pulse(
                &current,
                scene.session_mut(),
                PerformanceCommand::Parameter {
                    target_ref: "ql:performance/parameter/force-newtons".into(),
                    action: ParameterAction::Set,
                    value: Some(0.01),
                },
            )
            .unwrap_or_else(|failure| panic!("{}: {:?}", failure.reason, failure.native_pulse))
            .1
    } else {
        crate::continuous::host::performance_calibration::from_retained_preparation(
            Some(&authored),
            &mut owner,
            &current,
            scene.session_mut(),
        )
        .unwrap_or_else(|failure| panic!("{}: {:?}", failure.reason, failure.native_pulse))
        .1
    };
    assert_eq!(force_control["accepted"], true);
    let pending = checkpoint(&mut owner, &mut scene);
    assert_eq!(
        pending["payload"]["checkpoint"]["native_pair"]["audio"]["cursor"],
        "0"
    );
    let pending_audio = &pending["payload"]["checkpoint"]["native_pair"]["audio"];
    assert_eq!(pending_audio["accepted_sequence"], "1");
    assert_eq!(pending_audio["applied_application_ordinal"], "0");
    assert_eq!(
        pending_audio["source"]["force_newtons"],
        born_audio["source"]["force_newtons"]
    );
    assert!(pending["applications"].as_array().unwrap().is_empty());
    let admission = &force_control["payload"]["score_admission"];
    assert_eq!(admission["queued"], true);
    assert_eq!(admission["input_ref"], Value::Null);
    assert_eq!(admission["event"]["kind"], 5);
    assert_eq!(admission["event"]["parameter"], 0);
    assert_eq!(admission["event"]["sequence"], "1");
    assert_eq!(admission["event"]["sample"], "0");
    assert_eq!(admission["event"]["requested_sample"], "0");
    if !legacy {
        let refusal = match authored.admit_declared_output_calibration(
            &mut owner,
            &current,
            scene.session_mut(),
        ) {
            Ok(_) => panic!("fresh calibration cannot admit twice or during saved continuation"),
            Err(refusal) => refusal,
        };
        assert!(refusal.native_pulse.is_none());
        let after_refusal = checkpoint(&mut owner, &mut scene);
        assert_eq!(
            after_refusal["payload"]["checkpoint"],
            pending["payload"]["checkpoint"]
        );
    }
    let mut keys = BTreeMap::new();
    for cell in &owner.cells {
        if cell["available"] == true {
            keys.entry(cell["key"].as_u64().unwrap())
                .or_insert_with(|| cell.clone());
        }
    }
    assert_eq!(
        keys.len(),
        7,
        "actual current source reduction must supply its seven degrees"
    );
    let mut events = Vec::new();
    let mut pitches = Vec::new();
    for (index, cell) in keys.values().enumerate() {
        let touch = (index + 1) as u64;
        let target = owner
            .native_note_target(
                &current,
                KeyTouch {
                    key: cell["key"].as_u64().unwrap() as u8,
                    register: cell["register_octave"].as_i64().unwrap() as i8,
                    member: touch,
                    touch,
                    touch_ref: format!("native:calibration/source-degree/{index}"),
                },
            )
            .unwrap();
        pitches.push(target.clone());
        let at = index as u64 * 4096;
        events.push((at,json!({"identity":owner.binding().determination()["identity"],"kind":0,"sequence":"0","sample":at.to_string(),"touch":touch.to_string(),"value":0.8,"pitch_hz":target["hertz"],"parameter":0,"late_admitted":false,"has_note":true,"has_determination":false,"note":target})));
        let at = at + 3072;
        events.push((at,json!({"identity":owner.binding().determination()["identity"],"kind":1,"sequence":"0","sample":at.to_string(),"touch":touch.to_string(),"value":0.,"pitch_hz":0.,"parameter":0,"late_admitted":false,"has_note":false,"has_determination":false})));
    }
    for (index, cell) in keys.values().take(3).enumerate() {
        let touch = 100 + index as u64;
        let target = owner
            .native_note_target(
                &current,
                KeyTouch {
                    key: cell["key"].as_u64().unwrap() as u8,
                    register: cell["register_octave"].as_i64().unwrap() as i8,
                    member: touch,
                    touch,
                    touch_ref: format!("native:calibration/chord/{index}"),
                },
            )
            .unwrap();
        events.push((28672,json!({"identity":owner.binding().determination()["identity"],"kind":0,"sequence":"0","sample":"28672","touch":touch.to_string(),"value":0.65,"pitch_hz":target["hertz"],"parameter":0,"late_admitted":false,"has_note":true,"has_determination":false,"note":target})));
        events.push((40960,json!({"identity":owner.binding().determination()["identity"],"kind":1,"sequence":"0","sample":"40960","touch":touch.to_string(),"value":0.,"pitch_hz":0.,"parameter":0,"late_admitted":false,"has_note":false,"has_determination":false})));
    }
    events.sort_by_key(|(sample, _)| *sample);
    let mut original_admissions = Vec::new();
    let initial_sequence = owner.reading().unwrap()["accepted_sequence"]
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    for (index, (_, mut event)) in events.into_iter().enumerate() {
        event["sequence"] = json!((initial_sequence + index as u64 + 1).to_string());
        let mut request = owner.raw("score").unwrap();
        request["event"] = event;
        // Native C admits and journals the same authored input for each
        // NoteOn/NoteOff pair; parameter automation carries no input binding.
        request["input_ref"] = if matches!(request["event"]["kind"].as_u64(), Some(0 | 1 | 3)) {
            json!(format!(
                "native:calibration/input/{}",
                request["event"]["touch"].as_str().unwrap()
            ))
        } else {
            Value::Null
        };
        original_admissions.push(exchange(&mut owner, &mut scene, request));
    }
    let saved_pending = checkpoint(&mut owner, &mut scene);
    let body = owner.binding().physical_body().request().geometry.clone();
    let mut chunks = Vec::new();
    let mut peak = 0_f64;
    let mut power = 0_f64;
    let mut samples = 0;
    let mut energy = 0_f64;
    let mut displacement = 0_f64;
    let mut pickup = 0_f64;
    let mut force_applications = Vec::new();
    for _ in 0..96 {
        let pulse = render(&mut owner, &mut scene);
        for application in pulse["applications"].as_array().unwrap() {
            if application["kind"] == 5 && application["parameter"] == 0 {
                assert_eq!(application["status"], "applied");
                assert_eq!(application["sequence"], "1");
                assert_eq!(application["requested_sample"], "0");
                assert_eq!(application["applied_sample"], "0");
                assert_eq!(
                    application["value"],
                    if legacy {
                        0.01
                    } else {
                        DECLARED_INSTRUMENT_FORCE_NEWTONS
                    }
                );
                force_applications.push(application.clone());
            }
        }
        let pcm = pulse["payload"]["chunk"]["interleaved_f32"]
            .as_array()
            .unwrap();
        for number in pcm {
            let x = number.as_f64().unwrap();
            assert!(x.is_finite());
            peak = peak.max(x.abs());
            power += x * x;
            samples += 1;
        }
        let observation = &pulse["reading"]["physical"];
        assert_eq!(observation["node_ids"].as_array().unwrap().len(), 12);
        energy = energy.max(observation["energy_joules"].as_f64().unwrap());
        pickup = pickup.max(observation["pickup_linear"].as_f64().unwrap().abs());
        for (index, node) in body.nodes.iter().enumerate() {
            assert_eq!(observation["node_ids"][index], node.identity.to_string());
            let xyz = &observation["positions_metres"][index];
            for (axis, rest) in node.rest_metres.iter().enumerate() {
                let delta = xyz[axis].as_f64().unwrap() - rest;
                assert!(delta.is_finite());
                displacement = displacement.max(delta.abs());
                if node.fixed[axis] {
                    assert_eq!(delta, 0., "native constrained source axis moved");
                }
            }
        }
        let projection = &owner.config.controls.pickup;
        let expected_pickup = body
            .nodes
            .iter()
            .zip(&projection.node_weights)
            .enumerate()
            .map(|(index, (node, weight))| {
                (0..3)
                    .map(|axis| {
                        weight
                            * projection.axis[axis]
                            * (observation["positions_metres"][index][axis]
                                .as_f64()
                                .unwrap()
                                - node.rest_metres[axis])
                    })
                    .sum::<f64>()
            })
            .sum::<f64>()
            * owner.config.controls.pickup_linear_per_metre;
        assert!(
            (expected_pickup - observation["pickup_linear"].as_f64().unwrap()).abs()
                <= 1e-7 + expected_pickup.abs() * 1e-6,
            "pickup disconnected from same native visible q/state"
        );
        chunks.push(pulse);
    }
    let after = checkpoint(&mut owner, &mut scene);
    assert_eq!(
        force_applications.len(),
        1,
        "actual queued calibration must apply exactly once"
    );
    assert_eq!(
        after["payload"]["checkpoint"]["native_pair"]["audio"]["source"]["force_newtons"],
        if legacy {
            0.01
        } else {
            DECLARED_INSTRUMENT_FORCE_NEWTONS
        }
    );
    let rms = (power / samples as f64).sqrt();
    let dbfs = |linear: f64| {
        if linear > 0. {
            Some(20. * linear.log10())
        } else {
            None
        }
    };
    json!({"schema":"ql.actual-native-twelve-node-calibration-case/v1","legacy_projection":legacy,"original_world_request":world_request,"authored_preparation":authored,"native_configuration":owner.config,"native_source_assets":owner.source_assets(),"native_preparation":owner.packet().unwrap(),"actual_installation":installation,"original_born0_checkpoint":born,"actual_force_control":force_control,"actual_force_applications":force_applications,"pending_force_checkpoint":pending,"original_admissions":original_admissions,"saved_pending_checkpoint":saved_pending,"original_source_pitches":pitches,"native_chunks":chunks,"after_checkpoint":after,"measurements":{"sample_rate":48000,"frames":samples,"peak_linear":peak,"rms_linear":rms,"peak_dbfs":dbfs(peak),"rms_dbfs":dbfs(rms),"maximum_displacement_metres":displacement,"maximum_energy_joules":energy,"maximum_pickup_linear":pickup,"clipping_samples":owner.reading().unwrap()["clipping_samples"],"force_limited_samples":owner.reading().unwrap()["force_limited_samples"]},"scope":"actual immutable-source twelve-node instrument/native M1 source-degree note admission/physical q/M4/PCM with full saved native Force control; no device or app acceptance"})
}
#[test]
#[ignore = "requires exact normal hosted native worker; never a local substitute oscillator or scaled WAV"]
fn actual_default_twelve_node_instrument_calibration_measures_native_force_body_receiving_and_pcm()
{
    let worker = std::path::PathBuf::from(
        std::env::var("QL_NATIVE_FIELD_WORKER").expect("actual matching native worker"),
    );
    let before = run_case(&worker, true);
    let after = run_case(&worker, false);
    // The actual producer is emitted before acceptance asserts. A failure keeps
    // its unmodified native PCM/controls/source instead of a fabricated pass.
    if let Some(path) = std::env::var_os("QL_NATIVE_INSTRUMENT_CALIBRATION_ARTIFACT") {
        std::fs::write(path,serde_json::to_vec_pretty(&json!({"schema":"ql.actual-native-twelve-node-calibration/v1","before":before,"after":after,"acceptance":{"standing":"agent-proposed native output calibration acceptance; owner musical feel remains live review","force_newtons":2.,"master_linear":0.25,"pickup_linear_per_metre":250000.,"peak_minimum_linear":0.03,"rms_minimum_linear":0.005,"unintended_clipping_samples":0,"physical_displacement_bound_metres":0.01,"fixed_headroom_voices":24,"fixed_headroom_tails":16},"scope":"same actual twelve-node commissioned source path; complete original900s24voice/45k/180 and actual Mac device/app/owner delivery remain required"})).unwrap()).unwrap();
    }
    let measured = &after["measurements"];
    assert!(
        measured["peak_linear"].as_f64().unwrap() >= 0.03,
        "actual twelve-node passage is still too quiet: {measured}"
    );
    assert!(
        measured["rms_linear"].as_f64().unwrap() >= 0.005,
        "actual twelve-node RMS still too quiet: {measured}"
    );
    assert!(measured["maximum_displacement_metres"].as_f64().unwrap() > 0.);
    assert!(measured["maximum_displacement_metres"].as_f64().unwrap() <= 0.01);
    assert!(measured["maximum_energy_joules"].as_f64().unwrap() > 0.);
    assert_eq!(measured["clipping_samples"], "0");
    assert_eq!(measured["force_limited_samples"], "0");
}
