//! Original retained source producer and native SAME callback observation.
//! This component creates no private C lease or attached-device acceptance.
#[path = "support/native_fixture_carrier.rs"]
mod carrier;
#[path = "support/performance_route_management_fixture.rs"]
mod producer;
include!("support/native_fixture_payload_census.rs");
use std::io::Write;
use std::process::{Command, Stdio};
#[test]
#[ignore = "requires the matching normal-floor native performance_capture_wire binary"]
fn original_native_capture_keeps_pcm_force_source_and_reopened_callback_exact() {
    let native = std::env::var("QL_NATIVE_WIRE_TEST").expect("actual native floor binary");
    let (packet, retained) =
        producer::packet_with_return().expect("actual retained N9/body/Return producer");
    assert_eq!(
        packet,
        producer::packet().expect("independent original producer replay")
    );
    report_original_native_fixture_sizes(&packet);
    let encoded = carrier::encode(&packet);
    carrier::detecting_trials(&packet, &encoded);
    let bytes = serde_json::to_vec(&encoded).unwrap();
    assert!(bytes.len() < 8 * 1024 * 1024);
    let mut child = Command::new(native)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    if let Some(root) = std::env::var_os("QL_NATIVE_CAPTURE_RECEIPT_DIR") {
        let root = std::path::PathBuf::from(root);
        assert!(
            root.is_dir(),
            "existing actual original artifact custody required"
        );
        for (name, original) in [
            ("original-native-capture.stdout", result.stdout.as_slice()),
            ("original-native-capture.stderr", result.stderr.as_slice()),
        ] {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(root.join(name))
                .unwrap();
            file.write_all(original).unwrap();
            file.sync_all().unwrap();
        }
        let bytes = serde_json::to_vec(&retained).unwrap();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join("original-source-return.json"))
            .unwrap();
        file.write_all(&bytes).unwrap();
        file.sync_all().unwrap();
    }
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let receipt: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(receipt["schema"], "ql.actual-native-capture-component/v1");
    assert_eq!(receipt["callback_allocations"], "0");
    assert_eq!(receipt["callback_releases"], "0");
    assert_eq!(receipt["batches"].as_array().unwrap().len(), 4);
    assert_eq!(receipt["continued_batches"].as_array().unwrap().len(), 12);
    assert_eq!(
        receipt["overflow_batch"]["audio_blocks"]
            .as_array()
            .unwrap()
            .len(),
        16
    );
    assert_eq!(
        receipt["gap_batch"]["counters"]["dropped_audio_blocks_at_readback"],
        "2"
    );
}
