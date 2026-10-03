//! Real sourceForm preparation and native note feed the same-body receiving
//! experiment. The declared model is ours; no vendor runtime is represented.
#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::performance::PerformanceOwner;
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference, bind_performance_return};
use ql_mef::nara_performance_receiving::{
    ContextKind, ReceivingContext, ReceivingPreparation, Reference, prepare_native_receiving,
};
use ql_mef::performance_management::qualify_native_note_wire;
use ql_mef::performance_source_context::{
    NativePublicSourceOwnership, prepare_native_source_context,
};
use serde_json::{Value, json};

fn fixture() -> Value {
    let (basis, mut config) = source::config(true);
    config.use_native_m1_harmonic_ratio = false;
    // Explicit root-only forcing isolates propagation from the independently
    // tested M2 octet mechanism. It does not declare an inactive populated bus
    // to be operative. Geometry/eigenmodes are not retuned to the native note.
    config.excitation.root_linear = 1.0;
    config.excitation.octet_linear = 0.0;
    config.controls.material.damping_alpha_per_second = 40.0;
    let mut owner =
        PerformanceOwner::prepare(&basis, "research:moving/native-source", config).unwrap();
    let r = |name: &str| ReturnReference {
        reference: name.into(),
        revision: "1".into(),
    };
    let context = ReturnContext {
        context: r("research:moving/world"),
        receiver: r("research:moving/receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        kind: "world".into(),
        private: false,
        required_assets: vec![],
    };
    let receiving = ReceivingContext {
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
    let neutral = |context| ReceivingPreparation {
        prepared: owner.binding(),
        context,
        identity: None,
        current: None,
        original_occasion: None,
        calibration: None,
    };
    let definition = prepare_native_receiving(neutral(receiving.clone())).unwrap();
    let ownership = NativePublicSourceOwnership::reference_source(&basis).unwrap();
    let witness = prepare_native_source_context(
        &owner.source_context_basis(&basis).unwrap(),
        &definition,
        neutral(receiving),
        context.clone(),
        Some(&ownership),
    )
    .unwrap();
    owner.admit_source_context(&basis, &witness).unwrap();
    let returned = bind_performance_return(owner.binding(), None, context, 0).unwrap();
    json!({"schema":"ql.native-moving-receiving-fixture/v1",
        "native_preparation":owner.native_packet().unwrap(),
        "native_basis":owner.binding().native_basis(),
        "source_assets":owner.source_assets(), "native_catalog":owner.native_catalog(),
        "basis":returned.expression_basis().unwrap(),"pitches":returned.expression_pitches(0).unwrap()})
}
fn preserve(input: &[u8], result: &std::process::Output) {
    use std::io::Write;
    let Some(root) = std::env::var_os("QL_MOVING_RECEIVING_EVIDENCE_DIR") else {
        return;
    };
    let root = std::path::Path::new(&root);
    std::fs::create_dir(root).expect("fresh moving-receiving evidence destination");
    for (name, bytes, limit) in [
        ("producer-input.json", input, 16 * 1024 * 1024),
        (
            "native-stdout.json",
            result.stdout.as_slice(),
            32 * 1024 * 1024,
        ),
        (
            "native-stderr.txt",
            result.stderr.as_slice(),
            4 * 1024 * 1024,
        ),
    ] {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(name))
            .unwrap();
        file.write_all(&bytes[..bytes.len().min(limit)]).unwrap();
        file.sync_all().unwrap();
        if bytes.len() > limit {
            let mut marker = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(root.join(format!("{name}.truncated.json")))
                .unwrap();
            write!(
                marker,
                "{}",
                json!({"complete":false,"actual_bytes":bytes.len(),"retained_prefix_bytes":limit})
            )
            .unwrap();
        }
    }
    let mut exit = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join("native-exit.json"))
        .unwrap();
    write!(
        exit,
        "{}",
        json!({"success":result.status.success(),"code":result.status.code()})
    )
    .unwrap();
    exit.sync_all().unwrap();
    assert!(
        input.len() <= 16 * 1024 * 1024
            && result.stdout.len() <= 32 * 1024 * 1024
            && result.stderr.len() <= 4 * 1024 * 1024,
        "native moving receiving exceeded bounded custody; incomplete prefix marked explicitly"
    );
}
#[test]
fn original_native_source_prepares_the_declared_tone_and_body_policy() {
    let input = fixture();
    assert_eq!(
        input["native_preparation"]["notes"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert_eq!(
        input["native_preparation"]["determination"]["excitation"]["root_linear"],
        1.0
    );
    assert_eq!(
        input["native_preparation"]["determination"]["excitation"]["octet_linear"],
        0.0
    );
    assert!(input["native_preparation"]["physical_body"].is_object());
}
#[test]
#[ignore = "requires normal-floor qualified same-source performance_moving_receiving_wire binary"]
fn actual_native_pickup_has_measured_doppler_without_body_retuning_and_restores_exactly() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let packet = fixture();
    let input = serde_json::to_vec(&packet).unwrap();
    assert!(input.len() < 16 * 1024 * 1024);
    let mut child =
        Command::new(std::env::var("QL_NATIVE_WIRE_TEST").expect("normal native moving driver"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
    child.stdin.take().unwrap().write_all(&input).unwrap();
    let result = child.wait_with_output().unwrap();
    preserve(&input, &result);
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let measured: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(measured["schema"], "ql.native-moving-receiving-receipt/v1");
    qualify_native_note_wire(
        &packet["native_preparation"]["notes"][0],
        &measured["native_note"],
    )
    .unwrap();
    assert_eq!(measured["end_cursor"], "96000");
    assert_eq!(measured["physical"]["samples_elapsed"], "96000");
    assert_eq!(
        measured["physical_checkpoint"]["state"]["samples_elapsed"],
        "96000"
    );
    assert_eq!(measured["exact_restore_start"], "64000");
    assert_eq!(measured["exact_restore_frames"], "1024");
    assert_eq!(measured["callback_allocations"], "0");
    assert_eq!(measured["callback_releases"], "0");
    let hertz = measured["native_note"]["hertz"].as_f64().unwrap();
    let predictions = [hertz, hertz * 340.0 / 335.0, hertz * 343.0 / 340.0];
    for (key, predicted) in [
        "stationary_hertz",
        "moving_emitter_hertz",
        "moving_receiver_hertz",
    ]
    .into_iter()
    .zip(predictions)
    {
        let actual = measured[key].as_f64().unwrap();
        assert!(actual.is_finite() && (1200.0 * (actual / predicted).log2()).abs() <= 1.0);
    }
    for key in [
        "pickup",
        "stationary_received",
        "moving_emitter_received",
        "moving_receiver_received",
    ] {
        let pcm = measured[key].as_array().unwrap();
        assert_eq!(pcm.len(), 96000);
        assert!(pcm.iter().all(|x| x.as_f64().is_some_and(f64::is_finite)));
        assert!(pcm.iter().any(|x| x.as_f64().unwrap() != 0.0));
    }
    assert_ne!(
        measured["stationary_received"],
        measured["moving_emitter_received"]
    );
    assert_ne!(
        measured["stationary_received"],
        measured["moving_receiver_received"]
    );
}
