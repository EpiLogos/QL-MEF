//! `ql scene compose` through the actual executable and the qualified sky provider.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn a_dated_sky_composes_through_the_installed_command() {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
    ))
    .unwrap();
    let request = json!({"schema":"ql.scene-request/v1","sky_snapshot":sky,"tick12":3,"cycle":7});
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["scene", "compose", "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&request).unwrap())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let scene: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(scene["schema"], "ql.scene/v1");
    assert_eq!(scene["bodies"].as_array().unwrap().len(), 10);
    assert_eq!(scene["centres"].as_array().unwrap().len(), 7);
    let refused = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["scene", "compose", "/nonexistent.json"])
        .output()
        .unwrap();
    assert!(!refused.status.success());
}
