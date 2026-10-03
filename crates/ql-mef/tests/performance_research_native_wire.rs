//! Source-driven bounded mechanism measurements; vendor implementations are
//! not executed or inferred. The driver returns exact production consequences
//! and explicit unimplemented joins rather than equating unit labels with R.
#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::{coupled::HarmonicSource, performance::PerformanceOwner};
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference, bind_performance_return};
use ql_mef::nara_performance_receiving::{
    ContextKind, ReceivingContext, ReceivingPreparation, Reference, prepare_native_receiving,
};
use ql_mef::performance_source_context::{
    NativePublicSourceOwnership, prepare_native_source_context,
};
use serde_json::{Value, json};
fn source_packet(changed: bool) -> Value {
    let (mut basis, mut config) = source::config(true);
    config.use_native_m1_harmonic_ratio = false;
    if changed {
        let mut original = basis.input.clone();
        original.harmonic_source = HarmonicSource::CanonicalBasis { index: 0 };
        basis = original.compose().unwrap();
    }
    let mut owner =
        PerformanceOwner::prepare(&basis, "expression:retained-research/current", config).unwrap();
    let r = |value: &str| ReturnReference {
        reference: value.into(),
        revision: "1".into(),
    };
    let context = ReturnContext {
        context: r("research:neutral-world/context"),
        receiver: r("research:neutral-world/receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        kind: "world".into(),
        private: false,
        required_assets: vec![],
    };
    let c = ReceivingContext {
        kind: ContextKind::World,
        context: Reference {
            reference: context.context.reference.clone(),
            revision: context.context.revision.clone(),
        },
        receiver: Reference {
            reference: context.receiver.reference.clone(),
            revision: context.receiver.revision.clone(),
        },
        protected_state: None,
        consent: None,
        original_occasion: None,
        private: false,
    };
    let neutral = |c| ReceivingPreparation {
        prepared: owner.binding(),
        context: c,
        identity: None,
        current: None,
        original_occasion: None,
        calibration: None,
    };
    let definition = prepare_native_receiving(neutral(c.clone())).unwrap();
    let ownership = NativePublicSourceOwnership::reference_source(&basis).unwrap();
    let witness = prepare_native_source_context(
        &owner.source_context_basis(&basis).unwrap(),
        &definition,
        neutral(c),
        context.clone(),
        Some(&ownership),
    )
    .unwrap();
    owner.admit_source_context(&basis, &witness).unwrap();
    let returned = bind_performance_return(owner.binding(), None, context, 0).unwrap();
    json!({"native_preparation":owner.native_packet().unwrap(),"native_basis":owner.binding().native_basis(),"source_assets":owner.source_assets(),"native_catalog":owner.native_catalog(),"basis":returned.expression_basis().unwrap(),"pitches":returned.expression_pitches(0).unwrap()})
}
#[test]
fn research_pair_is_native_source_form_and_fixed_keys_body_policy() {
    let a = source_packet(false);
    let b = source_packet(true);
    assert_ne!(
        a["native_preparation"]["determination"]["audio_octet_hz"],
        b["native_preparation"]["determination"]["audio_octet_hz"]
    );
    assert_eq!(
        a["native_preparation"]["notes"],
        b["native_preparation"]["notes"]
    );
    assert!(a["native_preparation"]["physical_body"].is_object());
    assert!(b["native_preparation"]["physical_body"].is_object());
    assert_eq!(
        a["native_preparation"]["physical_body"],
        b["native_preparation"]["physical_body"]
    );
    assert_eq!(
        a["native_preparation"]["determination"]["excitation"],
        b["native_preparation"]["determination"]["excitation"]
    );
}
#[test]
#[ignore = "requires normal-floor qualified performance_research_wire binary"]
fn actual_source_to_force_body_pickup_receiving_and_continuation_measurements() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let executable = std::env::var("QL_NATIVE_WIRE_TEST").expect("normal native research driver");
    let packet = json!({"schema":"ql.native-research-mechanisms-fixture/v1","baseline":source_packet(false),"changed":source_packet(true)});
    let bytes = serde_json::to_vec(&packet).unwrap();
    assert!(bytes.len() < 16 * 1024 * 1024);
    let mut child = Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let measured: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(
        measured["schema"],
        "ql.native-research-mechanisms-receipt/v1"
    );
    let causal = &measured["R_D1_D4_same_cause_and_bus"];
    assert_eq!(causal["physical_end_cursor"], "2048");
    for field in [
        "preparation_ref",
        "state_ref",
        "body_revision",
        "source_coordinate",
        "source_revision",
        "source_generation",
        "eigenbasis_identity",
        "node_ids",
        "pratibimba",
    ] {
        assert_eq!(causal["physical"][field], causal["changed_physical"][field]);
    }
    assert_ne!(
        causal["physical"]["positions_metres"],
        causal["changed_physical"]["positions_metres"]
    );
    assert_eq!(causal["physical"]["samples_elapsed"], "2048");
    assert_eq!(causal["changed_physical"]["samples_elapsed"], "2048");
    assert_eq!(
        causal["baseline_body_checkpoint"]["identity"],
        causal["changed_body_checkpoint"]["identity"]
    );
    assert_eq!(
        causal["baseline_body_checkpoint"]["basis"],
        causal["changed_body_checkpoint"]["basis"]
    );
    assert_eq!(
        causal["baseline_body_checkpoint"]["state"]["samples_elapsed"],
        "2048"
    );
    assert_eq!(
        causal["changed_body_checkpoint"]["state"]["samples_elapsed"],
        "2048"
    );
    for field in [
        "displacement_modal_metres",
        "velocity_modal_metres_per_second",
    ] {
        assert_ne!(
            causal["baseline_body_checkpoint"]["state"][field],
            causal["changed_body_checkpoint"]["state"][field]
        );
    }
    for key in [
        "max_modal_q_difference_metres",
        "max_modal_v_difference_metres_per_second",
        "max_visible_position_difference_metres",
    ] {
        let delta = causal[key]
            .as_f64()
            .expect("actual same-cursor physical measurement");
        assert!(delta.is_finite() && delta > 0.0);
    }
    let retained = &measured["R_D5_retained_controls"];
    assert_eq!(retained["master_baseline_linear"], 0.25);
    assert_eq!(retained["master_target_linear"], 0.73);
    let effective = retained["master_actual_effective_at_384"].as_f64().unwrap();
    assert!(effective > 0.25 && effective < 0.73);
    assert_ne!(retained["baseline_pcm"], retained["automated_pcm"]);
    assert_eq!(retained["original_application_count"], "2");
    assert_eq!(retained["original_input_history_count"], "2");
    let applications = retained["applied_applications"].as_array().unwrap();
    let journal = retained["original_input_history"].as_array().unwrap();
    assert_eq!(applications.len(), 2);
    assert_eq!(journal.len(), 2);
    for (index, sample) in ["0", "64"].iter().enumerate() {
        let applied = &applications[index];
        assert_eq!(applied["schema"], "ql.performance-applied-event/v2");
        assert_eq!(applied["applied"], true);
        assert_eq!(applied["sequence"], (index + 1).to_string());
        assert_eq!(
            applied["applied_application_ordinal"],
            (index + 1).to_string()
        );
        for field in ["requested_sample", "admitted_sample", "applied_sample"] {
            assert_eq!(applied[field], *sample);
        }
        assert_eq!(applied["committed_cursor"], "128");
        assert_eq!(
            applied["identity"],
            packet["baseline"]["native_preparation"]["determination"]["identity"]
        );
    }
    assert_eq!(applications[0]["operation"], "note_on");
    assert_eq!(applications[0]["value"], 0.8);
    assert_eq!(
        applications[0]["note"],
        packet["baseline"]["native_preparation"]["notes"][0]
    );
    assert_eq!(applications[1]["operation"], "parameter");
    assert_eq!(applications[1]["value"], 0.73);
    assert_eq!(applications[1]["note"], Value::Null);
    for (index, history) in journal.iter().enumerate() {
        assert_eq!(history["ordinal"], (index + 1).to_string());
        assert_eq!(history["native_sequence"], "1");
        assert_eq!(history["input_ref"], "research:retained/original-touch");
        assert_eq!(history["target"], applications[0]["note"]);
    }
    assert_eq!(journal[0]["change"], 0);
    assert_eq!(journal[1]["change"], 2);
    if let Some(path) = std::env::var_os("QL_RESEARCH_RECEIPT_PATH") {
        std::fs::write(path, &result.stdout).unwrap();
    }
    println!("{}", String::from_utf8(result.stdout).unwrap());
}
