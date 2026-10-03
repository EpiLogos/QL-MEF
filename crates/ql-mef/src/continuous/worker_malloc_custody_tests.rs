//! Actual installed-worker diagnostic custody. No substitute process or SDK stats.
use super::worker_malloc_custody::DiagnosticConfig;
use super::{FIELD_CONTRACT, FieldInput, Worker};
use crate::m2_engine::M2Request;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn m2() -> M2Request {
    let mut input: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/m2-engine-request-v1.json"
    ))
    .unwrap();
    let mode = |i: u32, re: f64| {
        json!({"mode_ref":format!("controlled:reshape-mode/{i}"), "source_coordinate":"#2-1",
            "material_fibre":"earth", "carrier_weights":[{"carrier":i,"weight":1.0}],
            "frequency_hz":123.0 + f64::from(i) * 61.0, "amplitude":[re,0.001],
            "excitation":[0.001,0.0], "damping_per_second":0.25,
            "nodal_state_ref":"controlled:node", "antinodal_state_ref":"controlled:antinode"})
    };
    input["resonator"] = json!({"stamp":input["stamp"], "provider_ref":"controlled:reshape-test",
        "geometry_ref":"controlled:three-samples", "material_ref":"controlled:linear",
        "material_model_ref":"controlled:modal-v1", "material_parameters":{},
        "modes":[mode(0, 0.005), mode(1, 0.003)]});
    serde_json::from_value(input).unwrap()
}

fn field(subject: &str) -> FieldInput {
    serde_json::from_value(json!({"subject_ref":subject,
        "sample_rate":48000, "driver_numerator":720, "driver_denominator":1,
        "clock":{"generation":"4", "inscription":{"turns":"-1","half_degrees":719},
            "lensing":{"turns":"2","half_degrees":1}, "grid_origins":[3,9,21],
            "rate_numerators":["9","8"], "rate_denominator":8, "rate_remainders":["0","0"]},
        "units":{"amplitude":"m","excitation":"m/s","shape":"dimensionless","position":"m","audio":"linear"},
        "audio_gains":[1.0, 0.5], "samples":[
            {"identity":1,"constituent":"#3-5-5/0","attachment":0,"rest_metres":[0.0,0.0,0.0],
             "mode_shapes":[[0.0,0.0,1.0],[0.0,0.0,1.0]]},
            {"identity":2,"constituent":"#3-0","attachment":1,"rest_metres":[1.0,0.0,0.0],
             "mode_shapes":[[0.0,0.0,1.0],[0.0,0.0,0.0]]},
            {"identity":3,"constituent":"#2-0-0","attachment":2,"rest_metres":[0.0,1.0,0.0],
             "mode_shapes":[[0.0,0.0,0.0],[0.0,0.0,1.0]]}]}))
    .unwrap()
}

fn read(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
fn exact_pair(directory: &Path, ordinal: u64, request: &Value, reply: &Value) -> Value {
    let request_bytes = fs::read(directory.join(format!("request-{ordinal}.json"))).unwrap();
    assert_eq!(request_bytes, serde_json::to_vec(request).unwrap());
    let reply_bytes = fs::read(directory.join(format!("reply-{ordinal}.jsonl"))).unwrap();
    assert_eq!(reply_bytes.last(), Some(&b'\n'));
    assert_eq!(
        serde_json::from_slice::<Value>(&reply_bytes).unwrap(),
        *reply
    );
    let metadata = read(&directory.join(format!("exchange-{ordinal}.json")));
    assert_eq!(metadata["request_ordinal"], ordinal.to_string());
    assert_eq!(metadata["request_bytes"], request_bytes.len());
    assert_eq!(metadata["reply_bytes"], reply_bytes.len());
    assert_eq!(metadata["request_sha256"], digest(&request_bytes));
    assert_eq!(metadata["reply_sha256"], digest(&reply_bytes));
    assert_eq!(metadata["request_fully_written"], true);
    assert_eq!(metadata["response_framed_complete"], true);
    assert_eq!(metadata["response_parsed"], true);
    for (key, actual, limit) in [
        (
            "request_capacity_bytes",
            request_bytes.len(),
            32 * 1024 * 1024,
        ),
        (
            "reply_capacity_bytes",
            reply_bytes.len(),
            32 * 1024 * 1024 + 1,
        ),
    ] {
        let capacity = metadata[key].as_u64().unwrap() as usize;
        assert!(actual <= capacity && capacity <= limit);
    }
    metadata
}

#[test]
#[ignore = "requires genuine ql-field-worker and private fresh evidence destination"]
fn actual_worker_retains_only_selected_bytes_and_native_stderr_across_close() {
    let executable =
        PathBuf::from(std::env::var_os("QL_NATIVE_FIELD_WORKER").expect("actual compiled worker"));
    assert!(executable.is_file());
    let output = PathBuf::from(
        std::env::var_os("QL_WORKER_MALLOC_CUSTODY_TEST_OUTPUT").expect("fresh private output"),
    );
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(&output).unwrap();
    let output = output.canonicalize().unwrap();
    let config = DiagnosticConfig::for_test("1,3,5", &output).unwrap();
    let mut worker =
        Worker::open_with_diagnostic(&executable, Duration::from_secs(20), Some(config)).unwrap();
    let pid = worker.child.id();
    let directory = worker.diagnostic.as_ref().unwrap().directory().to_owned();
    let request1 = json!({"schema":"ql.field-control/v1","operation":"read"});
    let (error, reply1) = worker
        .exchange_contract_retained(&request1, FIELD_CONTRACT)
        .unwrap_err();
    assert!(!error.is_empty());
    let reply1 = reply1.expect("actual parsed recoverable native refusal");
    assert_eq!(reply1["state_committed"], false);
    assert!(!worker.poisoned);
    let request2 = json!({"schema":"ql.field-control/v1","operation":"initialize",
        "m2":m2().execute().unwrap(),"field":field("controlled:malloc-custody")});
    let initial = worker.exchange(&request2).unwrap();
    let request3 = json!({"schema":"ql.field-control/v1","operation":"advance",
        "expected_generation":initial["generation"],"expected_samples_elapsed":initial["samples_elapsed"],
        "frames":64,"muted":false});
    let reply3 = worker.exchange(&request3).unwrap();
    assert_eq!(reply3["samples_elapsed"], "64");
    assert_eq!(reply3["audio"].as_array().unwrap().len(), 64);
    assert!(
        reply3["audio"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_f64() != Some(0.0))
    );
    let read_request = json!({"schema":"ql.field-control/v1","operation":"read"});
    let before_refusal = worker.exchange(&read_request).unwrap();
    assert_eq!(before_refusal["samples_elapsed"], "64");
    let request5 = json!({"schema":"ql.field-control/v1","operation":"advance",
        "expected_generation":"0","expected_samples_elapsed":"64","frames":1,"muted":false});
    let (error, reply5) = worker
        .exchange_contract_retained(&request5, FIELD_CONTRACT)
        .unwrap_err();
    assert!(!error.is_empty());
    let reply5 = reply5.expect("actual parsed stale native refusal");
    assert_eq!(reply5["state_committed"], false);
    assert!(!worker.poisoned);
    assert_eq!(worker.exchange(&read_request).unwrap(), before_refusal);
    drop(worker);

    let opened = read(&directory.join("opened.json"));
    assert_eq!(opened["worker_pid"], pid);
    assert_eq!(opened["selected"], json!(["1", "3", "5"]));
    let one = exact_pair(&directory, 1, &request1, &reply1);
    assert_eq!(one["frontier"]["field"], Value::Null);
    assert_eq!(one["frontier"]["rendered_field_frames_total"], "0");
    let three = exact_pair(&directory, 3, &request3, &reply3);
    assert_eq!(three["frontier"]["rendered_field_frames"], 64);
    assert_eq!(three["frontier"]["rendered_field_frames_total"], "64");
    let five = exact_pair(&directory, 5, &request5, &reply5);
    assert_eq!(five["facts"]["original_requested_frames"], 1);
    assert_eq!(five["frontier"]["rendered_field_frames"], 0);
    assert_eq!(five["frontier"]["rendered_field_frames_total"], "64");
    for meta in [&three, &five] {
        let f = &meta["frontier"]["field"];
        assert_eq!(f["field_sample_count"], 3);
        assert_eq!(f["field_mode_count"], 2);
        assert_eq!(f["field_shape_vector_count"], 6);
        assert_eq!(f["field_samples_elapsed"], "64");
        assert_eq!(f["field_generation"], before_refusal["generation"]);
        assert_eq!(meta["frontier"]["frame_counter_overflow"], false);
    }
    for n in [2, 4, 6] {
        for name in [
            format!("request-{n}.json"),
            format!("reply-{n}.jsonl"),
            format!("exchange-{n}.json"),
        ] {
            assert!(!directory.join(name).exists());
        }
    }
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 13);
    let status = read(&directory.join("stderr-status.json"));
    assert_eq!(status["worker_pid"], pid);
    assert_eq!(status["eof_observed"], true);
    assert_eq!(status["truncated"], false);
    assert_eq!(status["counter_overflow"], false);
    assert_eq!(status["file_write_failed"], false);
    let stderr = fs::read(directory.join("worker-stderr.bin")).unwrap();
    assert!(stderr.len() <= 8192);
    assert_eq!(status["retained_bytes"], stderr.len());
    assert_eq!(status["observed_bytes"], stderr.len().to_string());
    let closed = read(&directory.join("closed.json"));
    assert_eq!(closed["worker_pid"], pid);
    assert_eq!(closed["phase"], "normal-owner-drop");
    assert_eq!(closed["owned_child_waited"], true);
    assert_eq!(closed["evidence_write_failed"], false);
    assert_eq!(closed["lost_custody"], false);
    assert_eq!(closed["writer_completed"], true);
    assert_eq!(closed["selected_pairs_queued"], json!([true, true, true]));
    assert_eq!(closed["submitted"], json!([true, true, true]));
    if cfg!(target_os = "macos") {
        assert_eq!(closed["close_wait_exhausted"], false);
        let rows = stderr
            .split(|b| *b == b'\n')
            .filter(|r| !r.is_empty())
            .map(|r| serde_json::from_slice::<Value>(r).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            rows.len(),
            3,
            "real Apple SDK rows required; absent is failure"
        );
        for (row, n) in rows.iter().zip([1_u64, 3, 5]) {
            assert_eq!(row["schema"], "ql.native-worker-malloc-observation/v1");
            assert_eq!(row["pid"], pid);
            assert_eq!(row["completed_request_ordinal"], n.to_string());
            assert_eq!(row["phase"], "after-request-destruction");
            assert_eq!(row["malloc_zone_scope"], "all");
            for f in [
                "blocks_in_use",
                "size_in_use",
                "max_size_in_use",
                "size_allocated",
            ] {
                assert!(row[f].as_u64().is_some(), "actual SDK counter {f}");
            }
        }
        assert_eq!(closed["observed"], json!([true, true, true]));
    } else {
        assert_eq!(opened["counter_platform"], "unavailable-non-Apple");
        assert!(
            stderr.is_empty(),
            "non-Apple has no substituted SDK statistics"
        );
        assert_eq!(closed["observed"], json!([false, false, false]));
    }
    // Genuine default transport, same source/state/PCM with no diagnostic record.
    let mut plain =
        Worker::open_with_diagnostic(&executable, Duration::from_secs(20), None).unwrap();
    assert!(plain.diagnostic.is_none());
    assert!(plain.stderr_reader.is_none());
    assert!(plain.diagnostic_writer.is_none());
    assert_eq!(plain.exchange(&request2).unwrap(), initial);
    assert_eq!(plain.exchange(&request3).unwrap(), reply3);
    drop(plain);
    assert_eq!(fs::read_dir(&output).unwrap().count(), 1);
    let qualified = qualify(&directory);
    assert!(
        qualified.status.success(),
        "genuine byte qualifier refused: {}",
        String::from_utf8_lossy(&qualified.stderr)
    );
    let summary: Value = serde_json::from_slice(&qualified.stdout).unwrap();
    assert_eq!(summary["worker_pid"], pid);
    assert_eq!(summary["pairs"].as_array().unwrap().len(), 3);
    assert_eq!(
        summary["observations"].as_array().unwrap().len(),
        if cfg!(target_os = "macos") { 3 } else { 0 }
    );
    let summary_path = output.join("qualification.json");
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(summary_path)
        .unwrap();
    file.write_all(&qualified.stdout).unwrap();
    file.sync_all().unwrap();
    // Negative evidence is copied FROM these actual bytes. No synthetic SDK positive.
    let altered = output.join("changed-request-digest");
    copy_original(&directory, &altered);
    let path = altered.join("exchange-3.json");
    let mut changed = read(&path);
    changed["request_sha256"] =
        json!("sha256:0000000000000000000000000000000000000000000000000000000000000000");
    fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
    assert!(!qualify(&altered).status.success());
    let interrupted = output.join("incomplete-stderr");
    copy_original(&directory, &interrupted);
    let path = interrupted.join("stderr-status.json");
    let mut changed = read(&path);
    changed["eof_observed"] = json!(false);
    fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
    assert!(!qualify(&interrupted).status.success());
    if cfg!(target_os = "macos") {
        let absent = output.join("fabricated-field-ownership");
        copy_original(&directory, &absent);
        let mut rows = stderr
            .split(|b| *b == b'\n')
            .filter(|r| !r.is_empty())
            .map(|r| serde_json::from_slice::<Value>(r).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(rows[0]["field_sample_count"], Value::Null);
        rows[0]["field_sample_count"] = json!(1);
        let bytes = rows
            .iter()
            .flat_map(|v| {
                let mut b = serde_json::to_vec(v).unwrap();
                b.push(b'\n');
                b
            })
            .collect::<Vec<_>>();
        fs::write(absent.join("worker-stderr.bin"), &bytes).unwrap();
        let path = absent.join("stderr-status.json");
        let mut changed = read(&path);
        changed["retained_bytes"] = json!(bytes.len());
        changed["observed_bytes"] = json!(bytes.len().to_string());
        fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
        assert!(!qualify(&absent).status.success());
    }
    // Schema recognition is independent of native JSON key order/whitespace.
    // These are copies of genuine SDK rows on Apple; no fabricated SDK positive.
    if cfg!(target_os = "macos") {
        let lexical = output.join("sdk-row-json-key-order");
        copy_original(&directory, &lexical);
        let rows = stderr
            .split(|b| *b == b'\n')
            .filter(|r| !r.is_empty())
            .map(|r| serde_json::from_slice::<Value>(r).unwrap())
            .collect::<Vec<_>>();
        let bytes = rows
            .iter()
            .flat_map(|row| {
                let mut bytes = b"  ".to_vec();
                bytes.extend(serde_json::to_vec(row).unwrap());
                bytes.push(b'\n');
                bytes
            })
            .collect::<Vec<_>>();
        replace_stderr_prefix(&lexical, &bytes);
        assert!(qualify(&lexical).status.success());
        // The same genuine SDK values cannot qualify a non-Apple scope merely
        // because a changed JSON order defeats a raw-prefix search.
        let changed_platform = output.join("sdk-row-substituted-platform");
        copy_original(&lexical, &changed_platform);
        let path = changed_platform.join("opened.json");
        let mut changed = read(&path);
        changed["counter_platform"] = json!("unavailable-non-Apple");
        fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
        let path = changed_platform.join("closed.json");
        let mut changed = read(&path);
        changed["observed"] = json!([false, false, false]);
        fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
        assert!(
            !qualify_with_requirement(&changed_platform, false)
                .status
                .success()
        );
    } else {
        let substituted = output.join("non-apple-substituted-sdk-schema");
        copy_original(&directory, &substituted);
        // Negative corruption only: original opening fields cannot become SDK
        // evidence through a changed schema. No counter values are invented.
        let mut changed = opened.clone();
        changed["schema"] = json!("ql.native-worker-malloc-observation/v1");
        let mut bytes = b"  ".to_vec();
        bytes.extend(serde_json::to_vec(&changed).unwrap());
        bytes.push(b'\n');
        replace_stderr_prefix(&substituted, &bytes);
        assert!(!qualify(&substituted).status.success());
    }
    // Real exclusive-file collision exercises diagnostic disk failure while
    // actual native refusal, following unselected initialization and PCM remain
    // available. No substitute worker, stats or diagnostic ACK is used.
    let failure_root = output.join("actual-diagnostic-file-refusal");
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(&failure_root).unwrap();
    let config = DiagnosticConfig::for_test("1", &failure_root).unwrap();
    let mut failed_writer =
        Worker::open_with_diagnostic(&executable, Duration::from_secs(20), Some(config)).unwrap();
    let failed_directory = failed_writer
        .diagnostic
        .as_ref()
        .unwrap()
        .directory()
        .to_owned();
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(failed_directory.join("request-1.json"))
        .unwrap();
    let (_, actual_refusal) = failed_writer
        .exchange_contract_retained(&request1, FIELD_CONTRACT)
        .unwrap_err();
    assert_eq!(actual_refusal.as_ref(), Some(&reply1));
    assert!(!failed_writer.poisoned);
    assert_eq!(failed_writer.exchange(&request2).unwrap(), initial);
    assert_eq!(failed_writer.exchange(&request3).unwrap(), reply3);
    assert!(!failed_writer.poisoned);
    drop(failed_writer);
    let failed_close = read(&failed_directory.join("closed.json"));
    assert_eq!(failed_close["evidence_write_failed"], true);
    assert_eq!(failed_close["writer_completed"], true);
    assert_eq!(failed_close["lost_custody"], false);
    assert!(!qualify(&failed_directory).status.success());
}

fn copy_original(directory: &Path, target: &Path) {
    fs::create_dir(target).unwrap();
    for item in fs::read_dir(directory).unwrap() {
        let item = item.unwrap();
        fs::copy(item.path(), target.join(item.file_name())).unwrap();
    }
}
fn replace_stderr_prefix(directory: &Path, bytes: &[u8]) {
    assert!(bytes.len() <= 8192);
    fs::write(directory.join("worker-stderr.bin"), bytes).unwrap();
    let path = directory.join("stderr-status.json");
    let mut changed = read(&path);
    changed["retained_bytes"] = json!(bytes.len());
    changed["observed_bytes"] = json!(bytes.len().to_string());
    fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
}
fn qualify(directory: &Path) -> std::process::Output {
    qualify_with_requirement(directory, cfg!(target_os = "macos"))
}
fn qualify_with_requirement(directory: &Path, require_apple: bool) -> std::process::Output {
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scripts/qualify-worker-malloc-custody.py");
    let mut command = std::process::Command::new("python3");
    command.arg(script).arg(directory);
    if require_apple {
        command.arg("--require-apple");
    }
    let actual = command.output().unwrap();
    assert!(actual.stdout.len() <= 8192 && actual.stderr.len() <= 8192);
    actual
}
