//! Candidate instrument calibration on the actual native World/source-form
//! owner. Baseline remains qualified; this does not adopt a default or prove
//! actual device loudness, installed interaction or the complete 900 s Act.
#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::performance_receiving::NativePerformanceReceivingSource;
use ql_mef::continuous::{
    host::{FieldHost, HOST_REQUEST, HostOperation, HostRequest},
    scene_field::SceneConfig,
};
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference};
use serde_json::{Value, json};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};
const BASELINE_PICKUP: f64 = 1000.;
// Original63 measured full stress raw_peak2.206540584564209 at1e6.
// Quarter sensitivity predicts0.5516351461410522 (about5dB headroom),
// with the SAME2N operation/24voices/16tails/P; actual trial must verify.
const CANDIDATE_PICKUP: f64 = 250_000.;
fn reference(s: &str) -> ReturnReference {
    ReturnReference {
        reference: s.into(),
        revision: "1".into(),
    }
}
fn actual_source(worker: &Path, pickup: f64) -> Value {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let request:ql_mef::scene::WorldRequest=serde_json::from_value(json!({"schema":ql_mef::scene::WORLD_REQUEST,"instance_ref":"expression:calibration/world","event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-current-receiving","texture":[64,64],"units_per_metre":1.,"sky":sky,"start":{"tick12":3,"cycle":7,"aperture":9}})).unwrap();
    let actual = ql_mef::scene::world(request.clone()).unwrap();
    let scene: SceneConfig = serde_json::from_value(actual["binding"]["host"].clone()).unwrap();
    let context = ReturnContext {
        kind: "world".into(),
        context: reference("controlled:calibration/world"),
        receiver: reference("controlled:calibration/receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        private: false,
        required_assets: vec![],
    };
    let receiving = NativePerformanceReceivingSource::world_source(request, context).unwrap();
    let mut host = FieldHost::open_scene(worker, scene.clone(), Duration::from_secs(20)).unwrap();
    host.bind_performance_receiving_source(receiving).unwrap();
    let (_, mut config) = source::config(true);
    config.controls.pickup_linear_per_metre = pickup;
    config.controls.expected_m3_generation =
        actual["native"]["m3"]["identity"]["profile_generation"]
            .as_u64()
            .unwrap_or_else(|| {
                let input: ql_mef::continuous::coupled::CoupledInput =
                    serde_json::from_value(actual["event"].clone()).unwrap();
                input.compose().unwrap().m3["identity"]["profile_generation"]
                    .as_u64()
                    .unwrap()
            });
    // This explicit AgentProposed instrument configuration remains within the
    // existing native controls. No force/material/displacement cap is changed.
    let ready = host.ready();
    let reply = host.execute(HostRequest {
        schema: HOST_REQUEST.into(),
        instance_ref: scene.instance_ref,
        event_ref: ready["field"]["event_ref"].as_str().unwrap().into(),
        subject_ref: ready["field"]["subject_ref"].as_str().unwrap().into(),
        request_id: "1".into(),
        expected_generation: ready["field"]["generation"].as_str().unwrap().into(),
        expected_samples_elapsed: ready["field"]["samples_elapsed"].as_str().unwrap().into(),
        command: HostOperation::PerformancePrepare {
            config: Box::new(config),
        },
    });
    assert_eq!(reply["status"], "ok", "{reply}");
    assert_eq!(reply["performance"]["accepted"], true, "{reply}");
    let artifact = host.retained_performance_source_artifact(0).unwrap();
    assert_eq!(artifact["native_reading"]["samples_elapsed"], "0");
    assert_eq!(
        artifact["source_assets"]["source_context"]["availability"],
        "available"
    );
    assert_eq!(
        artifact["source_assets"]["source_context"]["context"]["private"],
        false
    );
    assert_eq!(
        artifact["source_assets"]["configuration"]["controls"]["pickup_linear_per_metre"],
        pickup
    );
    assert_eq!(
        artifact["source_assets"]["configuration"]["controls"]["max_force_newtons"],
        10.
    );
    assert_eq!(
        artifact["source_assets"]["configuration"]["controls"]["max_displacement_metres"],
        0.01
    );
    assert_eq!(
        artifact["source_assets"]["current_receiving"]["source_inputs"],
        artifact["source_assets"]["receiving_source_inputs"]
    );
    artifact
}
fn write_new(path: &Path, data: &[u8]) {
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .unwrap();
    f.write_all(data).unwrap();
    f.sync_all().unwrap();
}
// The exact newly built native child is still the only executable admitted by
// this floor. Concurrent pipe drains retain bounded prefixes; a real overflow
// or deadline kills that child and preserves its actual exit/status and prefix.
fn bounded_native_output(mut child: std::process::Child) -> (std::process::Output, u64, u64, bool) {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    let exceeded = Arc::new(AtomicBool::new(false));
    let read_pipe = |mut input: Box<dyn Read + Send>, limit: usize, signal: Arc<AtomicBool>| {
        std::thread::spawn(move || -> std::io::Result<(Vec<u8>, u64)> {
            let mut retained = Vec::new();
            let mut count = 0u64;
            let mut chunk = [0u8; 8192];
            loop {
                let n = input.read(&mut chunk)?;
                if n == 0 {
                    break;
                }
                count = count
                    .checked_add(n as u64)
                    .ok_or_else(|| std::io::Error::other("native output counter overflow"))?;
                let accepted = n.min(limit - retained.len());
                retained.extend_from_slice(&chunk[..accepted]);
                if accepted != n {
                    signal.store(true, Ordering::Release);
                }
            }
            Ok((retained, count))
        })
    };
    let stdout = read_pipe(
        Box::new(child.stdout.take().unwrap()),
        32 * 1024 * 1024,
        exceeded.clone(),
    );
    let stderr = read_pipe(
        Box::new(child.stderr.take().unwrap()),
        4 * 1024 * 1024,
        exceeded.clone(),
    );
    let started = std::time::Instant::now();
    let mut interrupted = false;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if exceeded.load(Ordering::Acquire) || started.elapsed() > Duration::from_secs(120) {
            interrupted = true;
            let _ = child.kill();
            break child.wait().unwrap();
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let (out, out_count) = stdout.join().unwrap().unwrap();
    let (err, err_count) = stderr.join().unwrap().unwrap();
    let complete = !interrupted && !exceeded.load(Ordering::Acquire);
    (
        std::process::Output {
            status,
            stdout: out,
            stderr: err,
        },
        out_count,
        err_count,
        complete,
    )
}
#[test]
fn candidate_sensitivity_is_an_existing_bounded_source_control() {
    let (basis, mut config) = source::config(true);
    config.controls.pickup_linear_per_metre = CANDIDATE_PICKUP;
    let owner = ql_mef::continuous::performance::PerformanceOwner::prepare(
        &basis,
        "expression:calibration/source-bound",
        config.clone(),
    )
    .unwrap();
    assert_eq!(
        owner
            .binding()
            .physical_body()
            .request()
            .pickup_linear_per_metre,
        CANDIDATE_PICKUP
    );
    assert_eq!(
        owner.source_assets()["configuration"]["controls"]["pickup_linear_per_metre"],
        CANDIDATE_PICKUP
    );
    config.controls.pickup_linear_per_metre = 1e9 + 1.;
    assert!(
        ql_mef::continuous::performance::PerformanceOwner::prepare(
            &basis,
            "expression:calibration/source-bound",
            config
        )
        .is_err()
    );
}
#[test]
#[ignore = "requires the exact normal-floor native worker and calibration driver"]
fn actual_source_calibration_native_force_pcm_and_exact_continuation() {
    let worker = PathBuf::from(std::env::var("QL_NATIVE_FIELD_WORKER").unwrap());
    let driver = std::env::var("QL_NATIVE_WIRE_TEST").unwrap();
    let root = PathBuf::from(std::env::var("QL_CALIBRATION_EVIDENCE_DIR").unwrap());
    std::fs::create_dir(&root).unwrap();
    let baseline = actual_source(&worker, BASELINE_PICKUP);
    let candidate = actual_source(&worker, CANDIDATE_PICKUP);
    assert_eq!(baseline["native_basis"], candidate["native_basis"]);
    assert_eq!(
        baseline["native_preparation"]["notes"],
        candidate["native_preparation"]["notes"]
    );
    let mut a = baseline["source_assets"]["configuration"].clone();
    a["controls"]["pickup_linear_per_metre"] = json!(CANDIDATE_PICKUP);
    assert_eq!(a, candidate["source_assets"]["configuration"]);
    write_new(
        &root.join("baseline.source-performance.json"),
        &serde_json::to_vec(&baseline).unwrap(),
    );
    write_new(
        &root.join("candidate.source-performance.json"),
        &serde_json::to_vec(&candidate).unwrap(),
    );
    let configuration = json!({"schema":"ql.native-instrument-calibration-candidate/v1","standing":"agent-proposed-candidate; default-adoption-and-hardware-audibility-unexecuted","pickup_units":"linear/metre","baseline_pickup_linear_per_metre":BASELINE_PICKUP,"candidate_pickup_linear_per_metre":CANDIDATE_PICKUP,"baseline_force_newtons":0.01,"candidate_force_newtons":2.,"force_admission":"actual native authored Parameter operation at sample0; source/current applied receipt retained","native_parameter_target":"ql:performance/parameter/force-newtons","source_assets":"complete existing configuration/native source bundle, no second store","limits_changed":false});
    write_new(
        &root.join("configuration.json"),
        &serde_json::to_vec(&configuration).unwrap(),
    );
    let mut readings = Vec::new();
    for (name, artifact, force, stress) in [
        ("baseline", &baseline, 0.01, false),
        ("sensor_only", &candidate, 0.01, false),
        ("candidate", &candidate, 2., true),
    ] {
        let directory = root.join(name);
        std::fs::create_dir(&directory).unwrap();
        let bytes=serde_json::to_vec(&json!({"schema":"ql.native-calibration-fixture/v1","source":artifact,"force_newtons":force,"stress":stress})).unwrap();
        write_new(&directory.join("producer-input.json"), &bytes);
        assert!(bytes.len() < 16 * 1024 * 1024);
        let mut child = Command::new(&driver)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(&bytes).unwrap();
        let (output, stdout_observed, stderr_observed, complete) = bounded_native_output(child);
        // Original output is retained BEFORE status/numeric postvalidation.
        write_new(
            &directory.join("native-stdout.json"),
            &output.stdout[..output.stdout.len().min(32 * 1024 * 1024)],
        );
        write_new(
            &directory.join("native-stderr.txt"),
            &output.stderr[..output.stderr.len().min(4 * 1024 * 1024)],
        );
        write_new(&directory.join("native-exit.json"),json!({"success":output.status.success(),"code":output.status.code(),"stdout_bytes":output.stdout.len(),"stderr_bytes":output.stderr.len(),"stdout_observed_bytes":stdout_observed,"stderr_observed_bytes":stderr_observed,"complete_output":complete}).to_string().as_bytes());
        assert!(output.stdout.len() <= 32 * 1024 * 1024 && output.stderr.len() <= 4 * 1024 * 1024);
        assert!(
            complete,
            "actual native child output overflow or deadline; original bounded prefixes and actual exit retained"
        );
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let measured: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(measured["schema"], "ql.native-calibration-receipt/v1");
        assert_eq!(
            measured["pickup_linear_per_metre"],
            artifact["source_assets"]["configuration"]["controls"]["pickup_linear_per_metre"]
        );
        assert_eq!(measured["applied_force_newtons"], force);
        assert_eq!(measured["trial"]["samples"], "96000");
        assert_eq!(measured["trial"]["clipping_samples"], "0");
        assert_eq!(measured["callback_allocations"], "0");
        assert_eq!(measured["callback_frees"], "0");
        assert_eq!(measured["continued_f32_bit_exact"], true);
        assert_eq!(measured["queued_release_f32_bit_exact"], true);
        assert_eq!(measured["release_continuation_samples"], "5024");
        let released = measured["release_continuation"]["applications"]
            .as_array()
            .unwrap();
        assert_eq!(released.len(), 2);
        assert_eq!(released[0]["operation"], "parameter");
        assert_eq!(released[0]["sequence"], "3");
        assert_eq!(released[0]["applied_application_ordinal"], "3");
        assert_eq!(released[0]["requested_sample"], "97000");
        assert_eq!(released[0]["admitted_sample"], "97000");
        assert_eq!(released[0]["applied_sample"], "97000");
        assert_eq!(released[1]["operation"], "note_off");
        assert_eq!(released[1]["sequence"], "4");
        assert_eq!(released[1]["applied_application_ordinal"], "4");
        assert_eq!(released[1]["requested_sample"], "100000");
        assert_eq!(released[1]["admitted_sample"], "100000");
        assert_eq!(released[1]["applied_sample"], "100000");
        for field in [
            "operations",
            "releases",
            "pending_operations",
            "pending_releases",
            "cursor",
            "accepted_sequence",
            "accepted_sample",
            "panic_fence",
        ] {
            assert_eq!(
                measured["saved_management_checkpoint"]["native_pair"]["audio"][field],
                measured["restored_pending_management_checkpoint"]["native_pair"]["audio"][field]
            );
        }
        assert_eq!(measured["device_executed"], false);
        if stress {
            assert_eq!(measured["stress"]["maximum_active_voices"], "24");
            assert_eq!(measured["stress"]["maximum_active_tails"], "16");
            assert_eq!(
                measured["stress"]["available_native_keys"],
                json!([0, 2, 4, 5, 7, 9, 11])
            );
            assert_eq!(measured["stress"]["clipping_samples"], "0");
        }
        readings.push(measured);
    }
    assert_eq!(
        readings[0]["eigenfrequencies_hz"],
        readings[1]["eigenfrequencies_hz"]
    );
    assert_eq!(
        readings[0]["eigenfrequencies_hz"],
        readings[2]["eigenfrequencies_hz"]
    );
    assert_eq!(
        readings[0]["physical_q_metres"],
        readings[1]["physical_q_metres"]
    );
    assert_eq!(
        readings[0]["physical_v_metres_per_second"],
        readings[1]["physical_v_metres_per_second"]
    );
    assert_eq!(
        readings[0]["physical_positions_metres"],
        readings[1]["physical_positions_metres"]
    );
    let actual_sensor_ratio = readings[1]["trial"]["rms"].as_f64().unwrap()
        / readings[0]["trial"]["rms"].as_f64().unwrap();
    let declared_sensor_ratio = CANDIDATE_PICKUP / BASELINE_PICKUP;
    assert!((actual_sensor_ratio / declared_sensor_ratio - 1.0).abs() < 1e-5);
    assert!(
        readings[2]["trial"]["rms"].as_f64().unwrap()
            > readings[1]["trial"]["rms"].as_f64().unwrap()
    );
    assert_ne!(
        readings[0]["physical_q_metres"],
        readings[2]["physical_q_metres"]
    );
}
