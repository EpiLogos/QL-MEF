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
fn prepare_evidence(kind: &str, input: &[u8]) -> Option<std::path::PathBuf> {
    use std::io::Write;
    let Some(path) = std::env::var_os("QL_RECEIVING_PORT_EVIDENCE_DIR") else {
        eprintln!(
            "actual_receiving_port_preexecution context={kind} input_bytes={} input_limit={}",
            input.len(),
            16 * 1024 * 1024
        );
        return None;
    };
    let root = std::path::Path::new(&path);
    std::fs::create_dir_all(root).expect("native receiving port evidence root");
    let context = root.join(kind);
    std::fs::create_dir(&context).expect("fresh named native context evidence destination");
    let limit = 16 * 1024 * 1024;
    let mut source = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(context.join("producer-input.json"))
        .unwrap();
    source.write_all(&input[..input.len().min(limit)]).unwrap();
    source.sync_all().unwrap();
    let mut metadata = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(context.join("native-preexecution.json"))
        .unwrap();
    write!(
        metadata,
        "{}",
        json!({"schema":"ql.native-receiving-port-preexecution/v1",
        "context_kind":kind,"actual_input_bytes":input.len(),"stdin_limit_bytes":limit,
        "retained_input_bytes":input.len().min(limit),"complete_input":input.len() <= limit,
        "child_spawned":false})
    )
    .unwrap();
    metadata.sync_all().unwrap();
    if input.len() > limit {
        let mut marker = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(context.join("producer-input.json.truncated.json"))
            .unwrap();
        write!(
            marker,
            "{}",
            json!({"complete":false,"actual_bytes":input.len(),"retained_prefix_bytes":limit})
        )
        .unwrap();
        marker.sync_all().unwrap();
    }
    Some(context)
}
fn preserve_evidence(root: Option<&std::path::Path>, result: &std::process::Output) {
    use std::io::Write;
    let Some(root) = root else {
        return;
    };
    for (name, bytes, limit) in [
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
            marker.sync_all().unwrap();
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
        result.stdout.len() <= 32 * 1024 * 1024 && result.stderr.len() <= 4 * 1024 * 1024,
        "native receiving port output exceeds bounded custody; incomplete prefix marked explicitly"
    );
}
#[test]
fn actual_source_form_retains_sparse_note_and_public_context_for_receiving() {
    let packet = fixture();
    assert_eq!(
        packet["native_preparation"]["notes"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert_eq!(
        packet["basis"]["context"]["context"]["ref"],
        "research:moving/world"
    );
    assert_eq!(
        packet["basis"]["context"]["receiver"]["ref"],
        "research:moving/receiver"
    );
    assert_eq!(packet["basis"]["context"]["private"], false);
    assert_eq!(
        packet["native_preparation"]["determination"]["excitation"]["root_linear"],
        1.0
    );
    assert_eq!(
        packet["native_preparation"]["determination"]["excitation"]["octet_linear"],
        0.0
    );
}
#[test]
#[ignore = "requires normal-floor actual native performance_receiving_port_wire binary"]
fn actual_installed_receiving_changes_final_native_output_and_restores_full_history() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let packet = fixture();
    let input = serde_json::to_vec(&packet).unwrap();
    let evidence = prepare_evidence("world", &input);
    assert!(input.len() < 16 * 1024 * 1024);
    let mut child = Command::new(std::env::var("QL_NATIVE_WIRE_TEST").unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(root) = evidence.as_deref() {
        let mut started = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join("native-child-started.json"))
            .unwrap();
        write!(
            started,
            "{}",
            json!({"actual_native_child_pid":child.id(),"child_spawned":true})
        )
        .unwrap();
        started.sync_all().unwrap();
    }
    child.stdin.take().unwrap().write_all(&input).unwrap();
    let result = child.wait_with_output().unwrap();
    preserve_evidence(evidence.as_deref(), &result);
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let measured: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(measured["schema"], "ql.native-receiving-port-receipt/v1");
    qualify_native_note_wire(
        &packet["native_preparation"]["notes"][0],
        &measured["native_note"],
    )
    .unwrap();
    assert_eq!(measured["end_cursor"], "96000");
    assert_eq!(
        measured["physical_checkpoint"]["state"]["samples_elapsed"],
        "96000"
    );
    assert_eq!(measured["exact_restore_start"], "64000");
    assert_eq!(measured["exact_restore_frames"], "1024");
    assert_eq!(measured["callback_allocations"], "0");
    assert_eq!(measured["callback_releases"], "0");
    let replaced = &measured["replacement"];
    assert_eq!(
        replaced["schema"],
        "ql.native-receiving-replacement-component/v1"
    );
    assert_eq!(replaced["replacement_sample"], "4096");
    assert_eq!(replaced["end_cursor"], "13000");
    let old_rx = &replaced["before_receiving"];
    let new_rx = &replaced["after_receiving"];
    assert_eq!(old_rx["samples_elapsed"], "4096");
    assert_eq!(new_rx["samples_elapsed"], "4096");
    assert_eq!(old_rx["history_start_sample"], "0");
    assert_eq!(new_rx["history_start_sample"], "0");
    assert_eq!(old_rx["history_linear"], new_rx["history_linear"]);
    assert_eq!(new_rx["manifest"]["origin_sample"], "4096");
    assert_eq!(new_rx["manifest"]["history_origin_sample"], "0");
    assert_eq!(replaced["physical_before"], replaced["physical_after"]);
    let replaced_apps = replaced["applications"].as_array().unwrap();
    assert_eq!(replaced_apps.len(), 3);
    for (app, sequence, date) in [
        (&replaced_apps[0], "1", "0"),
        (&replaced_apps[1], "2", "9000"),
        (&replaced_apps[2], "3", "10000"),
    ] {
        assert_eq!(app["sequence"], sequence);
        assert_eq!(app["applied_application_ordinal"], sequence);
        assert_eq!(app["requested_sample"], date);
        assert_eq!(app["admitted_sample"], date);
        assert_eq!(app["applied_sample"], date);
        assert_eq!(app["applied"], true);
    }
    qualify_native_note_wire(
        &packet["native_preparation"]["notes"][0],
        &replaced_apps[0]["note"],
    )
    .unwrap();
    assert_eq!(
        replaced["continued_pcm"].as_array().unwrap().len(),
        13000 - 4096
    );

    let pickup = measured["pickup"].as_array().unwrap();
    assert_eq!(pickup.len(), 96000);
    assert!(pickup.iter().any(|x| x.as_f64().unwrap() != 0.0));
    let runs = measured["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 3);
    let native_hz = measured["native_note"]["hertz"].as_f64().unwrap();
    for (i, run) in runs.iter().enumerate() {
        assert_eq!(run["physical"]["samples_elapsed"], "96000");
        assert_eq!(run["checkpoint_receiving"]["samples_elapsed"], "64000");
        assert_eq!(
            run["checkpoint_receiving"]["history_linear"]
                .as_array()
                .unwrap()
                .len(),
            16384
        );
        assert_eq!(
            run["checkpoint_receiving"]["manifest"]["context"],
            packet["basis"]["context"]["context"]["ref"]
        );
        assert_eq!(
            run["checkpoint_receiving"]["manifest"]["receiver"],
            packet["basis"]["context"]["receiver"]["ref"]
        );
        let predicted = [
            native_hz,
            native_hz * 340.0 / 335.0,
            native_hz * 343.0 / 340.0,
        ][i];
        let measured = run["measured_hertz"].as_f64().unwrap();
        assert!((1200.0 * (measured / predicted).log2()).abs() <= 1.0);
        let received = run["received"].as_array().unwrap();
        let output = run["output"].as_array().unwrap();
        assert_eq!(received.len(), 96000);
        assert_eq!(output.len(), 96000);
        for (rx, pcm) in received.iter().zip(output) {
            let expected = (0.25 * rx.as_f64().unwrap()).clamp(-0.98, 0.98) as f32;
            assert_eq!((pcm.as_f64().unwrap() as f32).to_bits(), expected.to_bits());
        }
        let apps = run["applications"].as_array().unwrap();
        let journal = run["input_history"].as_array().unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(journal.len(), 2);
        for field in ["requested_sample", "admitted_sample", "applied_sample"] {
            assert_eq!(apps[0][field], "0");
        }
        assert_eq!(apps[0]["sequence"], "1");
        assert_eq!(apps[0]["applied_application_ordinal"], "1");
        qualify_native_note_wire(&packet["native_preparation"]["notes"][0], &apps[0]["note"])
            .unwrap();
        for entry in journal {
            assert_eq!(entry["input_ref"], "research:port/original-input");
            qualify_native_note_wire(&apps[0]["note"], &entry["target"]).unwrap();
        }
    }
    assert_ne!(runs[0]["output"], runs[1]["output"]);
    assert_ne!(runs[0]["output"], runs[2]["output"]);
}
