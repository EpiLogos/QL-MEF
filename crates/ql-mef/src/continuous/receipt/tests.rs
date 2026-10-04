use super::*;
use crate::continuous::{FieldSession, LiftInput};
use crate::m2_engine::M2Request;
use std::time::Duration;

fn fixture() -> (M2Request, FieldInput, ReceiptGuard, Value) {
    let mut input: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/kernel/m2-engine-request-v1.json"
    )))
    .unwrap();
    input["resonator"] = json!({"stamp":input["stamp"], "provider_ref":"controlled:receipt-test",
        "geometry_ref":"controlled:one-sample", "material_ref":"controlled:linear",
        "material_model_ref":"controlled:modal-v1", "material_parameters":{},
        "modes":[{"mode_ref":"controlled:mode/0", "source_coordinate":"#2-1", "material_fibre":"earth",
            "carrier_weights":[{"carrier":0,"weight":1.0}], "frequency_hz":123.0,
            "amplitude":[0.005,0.0], "excitation":[0.001,0.0], "damping_per_second":0.25,
            "nodal_state_ref":"controlled:node", "antinodal_state_ref":"controlled:antinode"}]});
    let m2: M2Request = serde_json::from_value(input).unwrap();
    let field: FieldInput = serde_json::from_value(json!({"subject_ref":"controlled:subject/one",
        "sample_rate":48000, "driver_numerator":720, "driver_denominator":1,
        "clock":{"generation":"4", "inscription":{"turns":"-1","half_degrees":719},
            "lensing":{"turns":"2","half_degrees":1}, "grid_origins":[3,9,21],
            "rate_numerators":["9","8"], "rate_denominator":8, "rate_remainders":["0","0"]},
        "units":{"amplitude":"m","excitation":"m/s","shape":"dimensionless","position":"m","audio":"linear"},
        "audio_gains":[1.0], "samples":[{"identity":0,"constituent":"#3-0","attachment":0,
            "rest_metres":[0.0,0.0,0.0],"mode_shapes":[[1.0,0.0,0.0]]}]})).unwrap();
    let frame = serde_json::to_value(m2.execute().unwrap()).unwrap();
    let guard = ReceiptGuard::new(&frame, &field).unwrap();
    let mut receipt = guard.fixed.clone();
    receipt.as_object_mut().unwrap().extend(
        json!({"generation":"1", "samples_elapsed":"0",
        "clock":guard.initial_clock, "amplitudes_metres":[[0.005,0.0]], "audio":[],
        "targets":[{"identity":0,"constituent":"#3-0","position":[0.005,0.0,0.0]}],
        "presentation_units_per_metre":1.0, "m2_identity":frame["identity"]})
        .as_object()
        .unwrap()
        .clone(),
    );
    (m2, field, guard, receipt)
}
fn read() -> Value {
    json!({"schema":"ql.field-control/v1","operation":"read"})
}
fn initialize() -> Value {
    json!({"operation":"initialize"})
}

#[test]
fn initial_receipt_binds_every_identity_and_source_not_just_schema() {
    let (_, _, guard, receipt) = fixture();
    guard.validate(&initialize(), None, &receipt).unwrap();
    for (path, invalid) in [
        ("/subject_ref", json!("controlled:subject/two")),
        ("/event_ref", json!("another-event")),
        ("/registry_revision", json!("another-registry")),
        ("/geometry_ref", json!("another-geometry")),
        ("/material_ref", json!("another-material")),
        ("/model_ref", json!("another-model")),
        ("/sample_rate", json!(44100)),
        ("/standing", json!("live-empirical-proof")),
        ("/m2_identity/profile_generation", json!(2)),
        ("/generation", json!("01")),
        ("/samples_elapsed", json!("1")),
        ("/targets/0/identity", json!(1)),
        ("/targets/0/constituent", json!("#3-5")),
        ("/targets/0/position/0", json!(1e39)),
        ("/amplitudes_metres/0/0", json!(1e13)),
        ("/presentation_units_per_metre", json!(1000)),
        ("/clock/centre_ref", json!("#3-0")),
        ("/clock/inscription/double_cover_half_degrees", json!(719)),
        ("/clock/generation", json!("5")),
        ("/clock/rate_remainders/0", json!("8")),
    ] {
        let mut bad = receipt.clone();
        *bad.pointer_mut(path).unwrap() = invalid;
        assert!(
            guard.validate(&initialize(), None, &bad).is_err(),
            "accepted {path}"
        );
    }
    let mut extra = receipt.clone();
    extra["private_subject"] = json!("unrequested");
    assert!(guard.validate(&initialize(), None, &extra).is_err());
    let mut short = receipt.clone();
    short["targets"] = json!([]);
    assert!(guard.validate(&initialize(), None, &short).is_err());
}

#[test]
fn read_and_zero_advance_cannot_reset_or_advance_the_resident_field() {
    let (_, _, guard, receipt) = fixture();
    for operation in [
        read(),
        json!({"operation":"advance","frames":0,"muted":false}),
    ] {
        guard
            .validate(&operation, Some(&receipt), &receipt)
            .unwrap();
        for (path, invalid) in [
            ("/generation", json!("2")),
            ("/samples_elapsed", json!("1")),
            ("/targets/0/position/0", json!(0.006)),
            ("/amplitudes_metres/0/0", json!(0.0)),
            ("/clock/lensing/half_degrees", json!(2)),
        ] {
            let mut bad = receipt.clone();
            *bad.pointer_mut(path).unwrap() = invalid;
            assert!(
                guard.validate(&operation, Some(&receipt), &bad).is_err(),
                "accepted {path}"
            );
        }
    }
}

#[test]
fn each_acknowledgement_has_its_exact_operation_cursor_and_audio_extent() {
    let (_, _, guard, receipt) = fixture();
    let advance = json!({"operation":"advance","frames":32,"muted":true});
    let mut next = receipt.clone();
    next["samples_elapsed"] = json!("32");
    next["audio"] = json!(vec![0.0; 32]);
    guard.validate(&advance, Some(&receipt), &next).unwrap();
    for (path, invalid) in [
        ("/samples_elapsed", json!("31")),
        ("/samples_elapsed", json!("33")),
        ("/generation", json!("2")),
        ("/audio/0", json!(0.001)),
        ("/clock/generation", json!("3")),
        ("/clock/generation", json!("6")),
        ("/clock/rate_numerators/0", json!("8")),
    ] {
        let mut bad = next.clone();
        *bad.pointer_mut(path).unwrap() = invalid;
        assert!(
            guard.validate(&advance, Some(&receipt), &bad).is_err(),
            "accepted {path}"
        );
    }
    next["audio"] = json!([]);
    assert!(guard.validate(&advance, Some(&receipt), &next).is_err());
    let mut boundary = receipt.clone();
    boundary["samples_elapsed"] = json!(u64::MAX.to_string());
    assert!(guard.validate(&advance, Some(&boundary), &next).is_err());
}

#[test]
fn independent_phase_ack_preserves_the_other_axis_residue_and_amplitudes() {
    let (_, _, guard, receipt) = fixture();
    let request = json!({"operation":"set-axis","axis":1,"phase":{"turns":"-2","half_degrees":37}});
    let mut next = receipt.clone();
    next["generation"] = json!("2");
    next["clock"]["generation"] = json!("5");
    next["clock"]["lensing"] =
        json!({"turns":"-2","half_degrees":37,"double_cover_half_degrees":37});
    guard.validate(&request, Some(&receipt), &next).unwrap();
    let mut bad = next.clone();
    bad["clock"]["inscription"] = next["clock"]["lensing"].clone();
    assert!(guard.validate(&request, Some(&receipt), &bad).is_err());
    bad = next.clone();
    bad["clock"]["rate_remainders"][1] = json!("1");
    assert!(guard.validate(&request, Some(&receipt), &bad).is_err());
}

#[test]
fn replacement_admits_new_basis_only_with_an_acknowledged_resident_transition() {
    let (m2, _, mut guard, receipt) = fixture();
    let mut identity = receipt["m2_identity"].clone();
    identity["profile_generation"] = json!(27);
    let mut frame = serde_json::to_value(m2.execute().unwrap()).unwrap();
    frame["identity"] = identity.clone();
    let request = json!({"operation":"replace-modes","replace_state":false,"m2":frame});
    let mut next = receipt.clone();
    next["generation"] = json!("2");
    next["m2_identity"] = identity.clone();
    guard.validate(&request, Some(&receipt), &next).unwrap();
    for path in [
        "/m2/resonator/geometry_ref",
        "/m2/resonator/material_ref",
        "/m2/resonator/material_model_ref",
        "/m2/resonator/modes/0/mode_ref",
    ] {
        let mut wrong = request.clone();
        *wrong.pointer_mut(path).unwrap() = json!("changed");
        assert!(guard.validate(&wrong, Some(&receipt), &next).is_err());
    }
    let mut reset = next.clone();
    reset["amplitudes_metres"][0][0] = json!(0.0);
    assert!(guard.validate(&request, Some(&receipt), &reset).is_err());
    assert_eq!(guard.m2_identity, receipt["m2_identity"]);
    guard.adopted(&next);
    guard.validate(&read(), Some(&next), &next).unwrap();
    assert_eq!(guard.m2_identity, identity);
}

#[test]
fn exact_cursor_strings_reject_aliases_and_overflow_without_panicking() {
    for text in [
        "",
        "00",
        "+1",
        " 1",
        "1 ",
        "1e3",
        "-0",
        "18446744073709551616",
    ] {
        assert!(unsigned(&json!(text)).is_err(), "accepted {text}");
    }
    for text in [
        "",
        "00",
        "+1",
        " 1",
        "-0",
        "9223372036854775808",
        "-9223372036854775809",
    ] {
        assert!(signed(&json!(text)).is_err(), "accepted {text}");
    }
    assert_eq!(unsigned(&json!(u64::MAX.to_string())).unwrap(), u64::MAX);
    assert_eq!(signed(&json!(i64::MIN.to_string())).unwrap(), i64::MIN);
}

// Controlled transport faults, not numerical field substitutes. Real native
// positive/replay/GPU cases run separately through k8_field and k8_coupled.
#[cfg(unix)]
mod transport {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Stub {
        root: PathBuf,
        executable: PathBuf,
    }
    impl Stub {
        fn new(replies: Vec<Value>, ending: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "ql-field-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&root).unwrap();
            let executable = root.join("worker");
            let program = format!(
                "#!/usr/bin/env python3\nimport json,sys,time\nreplies=json.loads({})\nending={}\nfor i,line in enumerate(sys.stdin):\n with open({},'a') as log: log.write(line)\n if i<len(replies): print(json.dumps(replies[i]),flush=True)\n elif ending=='timeout': time.sleep(60)\n elif ending=='malformed': print('{{',flush=True)\n elif ending=='oversized': print('x'*(32*1024*1024+1),flush=True)\n elif ending=='partial': sys.stdout.write('{{');sys.stdout.flush();break\n else: break\n",
                serde_json::to_string(&serde_json::to_string(&replies).unwrap()).unwrap(),
                serde_json::to_string(ending).unwrap(),
                serde_json::to_string(&root.join("requests")).unwrap()
            );
            std::fs::write(&executable, program).unwrap();
            std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
            Self { root, executable }
        }
        fn count(&self) -> usize {
            std::fs::read_to_string(self.root.join("requests"))
                .unwrap()
                .lines()
                .count()
        }
    }
    impl Drop for Stub {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn valid_refusal_keeps_the_same_session_readable() {
        let (m2, field, _, receipt) = fixture();
        let stub = Stub::new(
            vec![
                receipt.clone(),
                json!({"schema":"ql.field-error/v1","state_committed":false,"error":"bounded refusal"}),
                receipt.clone(),
            ],
            "eof",
        );
        let mut session =
            FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5)).unwrap();
        assert!(session.advance(9000, false).is_err());
        assert!(session.available());
        assert_eq!(session.read().unwrap(), receipt);
        assert_eq!(stub.count(), 3);
    }

    #[test]
    fn wrong_subject_and_cursor_poison_before_any_basis_or_receipt_adoption() {
        for (path, value) in [
            ("/subject_ref", json!("another-subject")),
            ("/samples_elapsed", json!("64")),
        ] {
            let (m2, field, _, receipt) = fixture();
            let original = serde_json::to_value(&m2).unwrap();
            let mut bad = receipt.clone();
            *bad.pointer_mut(path).unwrap() = value;
            let stub = Stub::new(vec![receipt.clone(), bad], "eof");
            let mut session =
                FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5)).unwrap();
            assert!(session.read().unwrap_err().contains("standing unknown"));
            assert!(!session.available());
            assert_eq!(session.last_receipt(), &receipt);
            assert_eq!(
                serde_json::to_value(session.current_basis()).unwrap(),
                original
            );
            assert!(session.advance(32, false).is_err());
            assert_eq!(
                stub.count(),
                2,
                "must not automatically retry an uncertain operation"
            );
        }
    }

    #[test]
    fn lost_malformed_and_postcommit_replies_never_become_clean_refusals() {
        for ending in [
            "eof",
            "partial",
            "malformed",
            "oversized",
            "timeout",
            "postcommit",
            "unqualified",
            "unknown",
        ] {
            let (m2, field, _, receipt) = fixture();
            let mut replies = vec![receipt.clone()];
            match ending {
                "postcommit" => replies.push(json!({"schema":"ql.field-error/v1","state_committed":true,"error":"lost publication"})),
                "unqualified" => replies.push(json!({"schema":"ql.field-error/v1","state_committed":false})),
                "unknown" => replies.push(json!({"schema":"unknown/v1"})),
                _ => {},
            }
            let stub = Stub::new(replies, ending);
            // Match the other transport cases: this deadline includes Python
            // startup and loading the complete native receipt before the fault.
            let mut session =
                FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5))
                    .unwrap_or_else(|error| panic!("{ending} worker startup failed: {error}"));
            assert!(
                session
                    .advance(32, false)
                    .unwrap_err()
                    .contains("standing unknown"),
                "{ending}"
            );
            assert!(!session.available());
            assert_eq!(session.last_receipt(), &receipt);
            assert!(session.read().is_err());
            assert_eq!(stub.count(), 2);
        }
    }

    #[test]
    fn rejected_initial_receipt_never_creates_a_live_session() {
        let (m2, field, _, mut receipt) = fixture();
        receipt["subject_ref"] = json!("wrong-subject");
        let stub = Stub::new(vec![receipt], "eof");
        assert!(FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5)).is_err());
        assert_eq!(stub.count(), 1);
    }

    #[test]
    fn explicit_new_session_after_failure_starts_from_retained_original_inputs() {
        let (m2, field, _, receipt) = fixture();
        let broken = Stub::new(vec![receipt.clone()], "eof");
        let mut session = FieldSession::open(
            &broken.executable,
            m2.clone(),
            field.clone(),
            Duration::from_secs(5),
        )
        .unwrap();
        assert!(session.advance(32, false).is_err());
        let fresh = Stub::new(vec![receipt.clone(), receipt.clone()], "eof");
        let mut recovered =
            FieldSession::open(&fresh.executable, m2, field, Duration::from_secs(5)).unwrap();
        assert_eq!(recovered.read().unwrap(), receipt);
        assert!(recovered.available() && !session.available());
        assert_eq!(broken.count(), 2);
        assert_eq!(fresh.count(), 2);
    }

    #[test]
    fn invalid_axis_success_ack_is_not_an_authorised_clock_change() {
        let (m2, field, _, receipt) = fixture();
        let stub = Stub::new(vec![receipt.clone(), receipt.clone()], "eof");
        let mut session =
            FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5)).unwrap();
        assert!(
            session
                .set_axis(
                    2,
                    LiftInput {
                        turns: "0".into(),
                        half_degrees: 0
                    }
                )
                .is_err()
        );
        assert!(!session.available());
        assert_eq!(session.last_receipt(), &receipt);
    }
}

/// Genuine current floor worker, existing M2 compiler and FieldSession only.
/// Neither the fault Stub above nor serialized timing/source grants are used.
#[test]
#[ignore = "requires normal floor's actual QL_NATIVE_FIELD_WORKER"]
fn actual_native_field_clock_constructor_continues_across_owned_operations() {
    let worker = std::path::PathBuf::from(
        std::env::var_os("QL_NATIVE_FIELD_WORKER").expect("actual floor worker required"),
    );
    let (m2, field, _, _) = fixture();
    let mut owner =
        FieldSession::open(&worker, m2.clone(), field.clone(), Duration::from_secs(20)).unwrap();
    let other =
        FieldSession::open(&worker, m2.clone(), field.clone(), Duration::from_secs(20)).unwrap();
    let initial = owner.last_receipt().clone();
    let different_constructor = other.last_receipt()["timing_owner"].clone();
    assert_ne!(
        initial["timing_owner"]["instance_ref"],
        different_constructor["instance_ref"]
    );
    assert_eq!(initial["clock"], other.last_receipt()["clock"]);
    assert_eq!(initial["samples_elapsed"], "0");
    assert_eq!(initial["timing_owner"]["initial_clock_generation"], "4");
    let stable = |actual: &Value| {
        for key in [
            "schema",
            "instance_ref",
            "construction_ordinal",
            "generation",
            "generation_domain",
            "initial_clock_generation",
        ] {
            assert_eq!(actual["timing_owner"][key], initial["timing_owner"][key]);
        }
        assert_eq!(
            actual["timing_owner"]["samples_elapsed"],
            actual["samples_elapsed"]
        );
        assert_eq!(
            actual["timing_owner"]["clock_generation"],
            actual["clock"]["generation"]
        );
        assert_eq!(actual["timing_owner"]["event_ref"], actual["event_ref"]);
        assert_eq!(actual["timing_owner"]["subject_ref"], actual["subject_ref"]);
        assert_eq!(actual["timing_owner"]["sample_rate"], actual["sample_rate"]);
    };
    stable(&initial);
    let readback = owner.read().unwrap();
    assert_eq!(readback, initial);
    let running = owner.advance(512, false).unwrap();
    stable(&running);
    assert_eq!(running["samples_elapsed"], "512");
    assert_ne!(
        running["clock"]["generation"],
        initial["clock"]["generation"]
    );
    assert_eq!(
        running["timing_owner"]["generation"],
        initial["timing_owner"]["generation"]
    );
    assert!(
        running["audio"]
            .as_array()
            .unwrap()
            .iter()
            .any(|x| x.as_f64().unwrap() != 0.)
    );
    let replacement = |generation: u64, amplitude: f64| -> M2Request {
        let mut actual = serde_json::to_value(&m2).unwrap();
        for owner in ["m1_excitation", "vimarsha", "resonator"] {
            actual[owner]["stamp"]["identity"]["profile_generation"] = json!(generation);
        }
        actual["stamp"]["identity"]["profile_generation"] = json!(generation);
        actual["resonator"]["modes"][0]["frequency_hz"] = json!(246.);
        actual["resonator"]["modes"][0]["amplitude"][0] = json!(amplitude);
        serde_json::from_value(actual).unwrap()
    };
    // Native mode replacement and deliberate modal reseeding both continue
    // the SAME embedded clock constructor, rather than minting source aliases.
    let changed = owner.replace_modes(replacement(2, 0.02), false).unwrap();
    stable(&changed);
    assert_eq!(changed["clock"], running["clock"]);
    assert_eq!(changed["amplitudes_metres"], running["amplitudes_metres"]);
    let reseeded = owner.replace_modes(replacement(3, 0.03), true).unwrap();
    stable(&reseeded);
    assert_eq!(reseeded["clock"], running["clock"]);
    assert_ne!(reseeded["amplitudes_metres"], changed["amplitudes_metres"]);
    let shaped = owner
        .replace_shapes("controlled:field-clock/nodal-2", vec![vec![[0., 2., 0.]]])
        .unwrap();
    stable(&shaped);
    assert_eq!(shaped["clock"], reseeded["clock"]);
    assert_eq!(shaped["amplitudes_metres"], reseeded["amplitudes_metres"]);
    assert_ne!(shaped["targets"], reseeded["targets"]);
    let phase = owner
        .set_axis(
            1,
            LiftInput {
                turns: "-2".into(),
                half_degrees: 37,
            },
        )
        .unwrap();
    stable(&phase);
    assert_eq!(phase["samples_elapsed"], "512");
    assert_eq!(phase["amplitudes_metres"], shaped["amplitudes_metres"]);
    assert_ne!(phase["clock"]["generation"], shaped["clock"]["generation"]);
    assert_eq!(
        phase["clock"]["inscription"],
        shaped["clock"]["inscription"]
    );
    let before_refusal = owner.last_receipt().clone();
    assert!(owner.advance(8193, false).is_err());
    assert!(owner.available());
    assert_eq!(owner.read().unwrap(), before_refusal);
    let final_receipt = owner.advance(128, true).unwrap();
    stable(&final_receipt);
    assert_eq!(final_receipt["samples_elapsed"], "640");
    assert!(
        final_receipt["audio"]
            .as_array()
            .unwrap()
            .iter()
            .all(|x| x.as_f64().unwrap() == 0.)
    );
    let current = owner.read().unwrap();
    owner
        .guard
        .validate(&read(), Some(&current), &current)
        .unwrap();
    // Mutate ONLY genuine native bytes for detecting parser refusals. This
    // does not send them to a worker or construct a private source/clock grant.
    for (name, value) in [
        (
            "instance_ref",
            different_constructor["instance_ref"].clone(),
        ),
        ("construction_ordinal", json!("01")),
        ("generation", current["clock"]["generation"].clone()),
        ("generation_domain", json!("m2_generation")),
        ("initial_clock_generation", json!("5")),
        ("clock_generation", json!("0")),
        ("samples_elapsed", json!("641")),
        ("event_ref", json!("different:valid-event")),
        ("subject_ref", json!("different:valid-subject")),
        ("sample_rate", json!(44100)),
    ] {
        let mut wrong = current.clone();
        wrong["timing_owner"][name] = value;
        assert!(
            owner
                .guard
                .validate(&read(), Some(&current), &wrong)
                .is_err(),
            "accepted detached native {name}"
        );
    }
    for shape in ["missing", "null", "unknown", "zero-token"] {
        let mut wrong = current.clone();
        match shape {
            "missing" => {
                wrong.as_object_mut().unwrap().remove("timing_owner");
            }
            "null" => wrong["timing_owner"] = Value::Null,
            "unknown" => wrong["timing_owner"]["caller_generation"] = json!(1),
            "zero-token" => {
                wrong["timing_owner"]["instance_ref"] =
                    json!("native-resident:v1:00000000000000000000000000000000:1")
            }
            _ => unreachable!(),
        }
        assert!(
            owner
                .guard
                .validate(&read(), Some(&current), &wrong)
                .is_err(),
            "accepted {shape}"
        );
    }
    // Original clock declaration may be intentionally re-used, but new native
    // construction after genuine owner Drop is a different resident lifetime.
    let retired_token = current["timing_owner"]["instance_ref"].clone();
    drop(owner);
    drop(other);
    let fresh = FieldSession::open(&worker, m2, field, Duration::from_secs(20)).unwrap();
    assert_ne!(
        fresh.last_receipt()["timing_owner"]["instance_ref"],
        retired_token
    );
    assert_eq!(fresh.last_receipt()["clock"], initial["clock"]);
    assert_eq!(
        fresh.last_receipt()["amplitudes_metres"],
        initial["amplitudes_metres"]
    );
    assert_eq!(fresh.last_receipt()["samples_elapsed"], "0");
    // Existing transport/source parser coverage can read historical literal
    // no-constructor receipts; they have no native clock participant standing.
    let mut historical = initial.clone();
    historical.as_object_mut().unwrap().remove("timing_owner");
    clock_constructor(&historical, None).unwrap();
    assert!(clock_constructor(&initial, Some(&historical)).is_err());
}
