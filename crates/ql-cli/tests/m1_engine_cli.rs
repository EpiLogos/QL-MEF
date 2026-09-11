use serde_json::Value;
use std::path::Path;
use std::process::Command;
#[test]
fn native_cli_consumes_the_published_engine_request_and_rejects_bad_arity() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/kernel/m1-engine-v1.request.json");
    let run = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["kernel", "m1"])
        .arg(fixture)
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let v: Value = serde_json::from_slice(&run.stdout).unwrap();
    assert_eq!(v["schema"], "ql.m1.engine/v1");
    assert_eq!(v["config"]["event_ref"], "example:cosmic-event");
    assert_eq!(v["selected_reading"]["content"]["projection"]["family"], 5);
    assert_eq!(v["music"]["context_frame"], "CF7");
    assert!(
        !Command::new(env!("CARGO_BIN_EXE_ql"))
            .args(["kernel", "m1"])
            .output()
            .unwrap()
            .status
            .success()
    );
}
