use super::*;
use crate::continuous::{FieldSession, LiftInput};
use crate::m2_engine::M2Request;
use std::time::Duration;

fn fixture() -> (M2Request, FieldInput, ReceiptGuard, Value) {
    let mut input: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/kernel/m2-engine-request-v1.json"))).unwrap();
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
    receipt.as_object_mut().unwrap().extend(json!({"generation":"1", "samples_elapsed":"0",
        "clock":guard.initial_clock, "amplitudes_metres":[[0.005,0.0]], "audio":[],
        "targets":[{"identity":0,"constituent":"#3-0","position":[0.005,0.0,0.0]}],
        "presentation_units_per_metre":1.0, "m2_identity":frame["identity"]}).as_object().unwrap().clone());
    (m2, field, guard, receipt)
}
fn read() -> Value { json!({"schema":"ql.field-control/v1","operation":"read"}) }
fn initialize() -> Value { json!({"operation":"initialize"}) }

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
        assert!(guard.validate(&initialize(), None, &bad).is_err(), "accepted {path}");
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
    for operation in [read(), json!({"operation":"advance","frames":0,"muted":false})] {
        guard.validate(&operation, Some(&receipt), &receipt).unwrap();
        for (path, invalid) in [
            ("/generation", json!("2")), ("/samples_elapsed", json!("1")),
            ("/targets/0/position/0", json!(0.006)),
            ("/amplitudes_metres/0/0", json!(0.0)),
            ("/clock/lensing/half_degrees", json!(2)),
        ] {
            let mut bad = receipt.clone();
            *bad.pointer_mut(path).unwrap() = invalid;
            assert!(guard.validate(&operation, Some(&receipt), &bad).is_err(), "accepted {path}");
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
        ("/samples_elapsed", json!("31")), ("/samples_elapsed", json!("33")),
        ("/generation", json!("2")), ("/audio/0", json!(0.001)),
        ("/clock/generation", json!("3")), ("/clock/generation", json!("6")),
        ("/clock/rate_numerators/0", json!("8")),
    ] {
        let mut bad = next.clone();
        *bad.pointer_mut(path).unwrap() = invalid;
        assert!(guard.validate(&advance, Some(&receipt), &bad).is_err(), "accepted {path}");
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
    next["clock"]["lensing"] = json!({"turns":"-2","half_degrees":37,"double_cover_half_degrees":37});
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
    for path in ["/m2/resonator/geometry_ref", "/m2/resonator/material_ref", "/m2/resonator/material_model_ref", "/m2/resonator/modes/0/mode_ref"] {
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
    for text in ["", "00", "+1", " 1", "1 ", "1e3", "-0", "18446744073709551616"] {
        assert!(unsigned(&json!(text)).is_err(), "accepted {text}");
    }
    for text in ["", "00", "+1", " 1", "-0", "9223372036854775808", "-9223372036854775809"] {
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
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::os::unix::fs::PermissionsExt;
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Stub { root: PathBuf, executable: PathBuf }
    impl Stub {
        fn new(replies: Vec<Value>, ending: &str) -> Self {
            let root = std::env::temp_dir().join(format!("ql-field-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            std::fs::create_dir(&root).unwrap();
            let executable = root.join("worker");
            let program = format!("#!/usr/bin/env python3\nimport json,sys,time\nreplies=json.loads({})\nending={}\nfor i,line in enumerate(sys.stdin):\n with open({},'a') as log: log.write(line)\n if i<len(replies): print(json.dumps(replies[i]),flush=True)\n elif ending=='timeout': time.sleep(60)\n elif ending=='malformed': print('{{',flush=True)\n elif ending=='oversized': print('x'*(32*1024*1024+1),flush=True)\n elif ending=='partial': sys.stdout.write('{{');sys.stdout.flush();break\n else: break\n",
                serde_json::to_string(&serde_json::to_string(&replies).unwrap()).unwrap(),
                serde_json::to_string(ending).unwrap(), serde_json::to_string(&root.join("requests")).unwrap());
            std::fs::write(&executable, program).unwrap();
            std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
            Self { root, executable }
        }
        fn count(&self) -> usize { std::fs::read_to_string(self.root.join("requests")).unwrap().lines().count() }
    }
    impl Drop for Stub { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.root); } }

    #[test]
    fn valid_refusal_keeps_the_same_session_readable() {
        let (m2, field, _, receipt) = fixture();
        let stub = Stub::new(vec![receipt.clone(), json!({"schema":"ql.field-error/v1","state_committed":false,"error":"bounded refusal"}), receipt.clone()], "eof");
        let mut session = FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5)).unwrap();
        assert!(session.advance(9000, false).is_err());
        assert!(session.available());
        assert_eq!(session.read().unwrap(), receipt);
        assert_eq!(stub.count(), 3);
    }

    #[test]
    fn wrong_subject_and_cursor_poison_before_any_basis_or_receipt_adoption() {
        for (path, value) in [("/subject_ref", json!("another-subject")), ("/samples_elapsed", json!("64"))] {
            let (m2, field, _, receipt) = fixture();
            let original = serde_json::to_value(&m2).unwrap();
            let mut bad = receipt.clone();
            *bad.pointer_mut(path).unwrap() = value;
            let stub = Stub::new(vec![receipt.clone(), bad], "eof");
            let mut session = FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5)).unwrap();
            assert!(session.read().unwrap_err().contains("standing unknown"));
            assert!(!session.available());
            assert_eq!(session.last_receipt(), &receipt);
            assert_eq!(serde_json::to_value(session.current_basis()).unwrap(), original);
            assert!(session.advance(32, false).is_err());
            assert_eq!(stub.count(), 2, "must not automatically retry an uncertain operation");
        }
    }

    #[test]
    fn lost_malformed_and_postcommit_replies_never_become_clean_refusals() {
        for ending in ["eof", "partial", "malformed", "oversized", "timeout", "postcommit", "unqualified", "unknown"] {
            let (m2, field, _, receipt) = fixture();
            let mut replies = vec![receipt.clone()];
            match ending {
                "postcommit" => replies.push(json!({"schema":"ql.field-error/v1","state_committed":true,"error":"lost publication"})),
                "unqualified" => replies.push(json!({"schema":"ql.field-error/v1","state_committed":false})),
                "unknown" => replies.push(json!({"schema":"unknown/v1"})),
                _ => {},
            }
            let stub = Stub::new(replies, ending);
            let mut session = FieldSession::open(&stub.executable, m2, field, Duration::from_secs(2)).unwrap();
            assert!(session.advance(32, false).unwrap_err().contains("standing unknown"), "{ending}");
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
        let mut session = FieldSession::open(&broken.executable, m2.clone(), field.clone(), Duration::from_secs(5)).unwrap();
        assert!(session.advance(32, false).is_err());
        let fresh = Stub::new(vec![receipt.clone(), receipt.clone()], "eof");
        let mut recovered = FieldSession::open(&fresh.executable, m2, field, Duration::from_secs(5)).unwrap();
        assert_eq!(recovered.read().unwrap(), receipt);
        assert!(recovered.available() && !session.available());
        assert_eq!(broken.count(), 2);
        assert_eq!(fresh.count(), 2);
    }

    #[test]
    fn invalid_axis_success_ack_is_not_an_authorised_clock_change() {
        let (m2, field, _, receipt) = fixture();
        let stub = Stub::new(vec![receipt.clone(), receipt.clone()], "eof");
        let mut session = FieldSession::open(&stub.executable, m2, field, Duration::from_secs(5)).unwrap();
        assert!(session.set_axis(2, LiftInput { turns:"0".into(), half_degrees:0 }).is_err());
        assert!(!session.available());
        assert_eq!(session.last_receipt(), &receipt);
    }
}
