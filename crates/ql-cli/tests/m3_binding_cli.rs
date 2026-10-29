//! Binding's engine through the actual executable: `ql kernel m3-binding -`
//! reads its request on stdin and answers `ql.m3-binding/v1`; a request it
//! cannot read is refused, and nothing is answered in its place.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn ql(args: &[&str], input: &Value) -> std::process::Output {
    let mut p = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = p.stdin.take().unwrap();
    stdin
        .write_all(&serde_json::to_vec(input).unwrap())
        .unwrap();
    drop(stdin);
    p.wait_with_output().unwrap()
}

#[test]
fn the_executable_answers_binding_for_a_hexagram_under_a_matrix() {
    let request = json!({"hexagram": 17, "matrix": "moving-resting", "register": "pp",
        "tension_gev_per_fm": 0.9, "coupling_scale_gev": 1.0});
    let out = ql(&["kernel", "m3-binding", "-", "--json"], &request);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let reading: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(reading["schema"], "ql.m3-binding/v1");
    assert_eq!(reading["cell"]["positive"]["sequence"], "CCC");
    assert_eq!(reading["cell"]["negative"]["sequence"], "CTC");
    assert_eq!(reading["string"]["stretch_fm"], 0.8);
}

#[test]
fn the_executable_refuses_a_request_outside_the_manuals_ranges() {
    let request = json!({"hexagram": 17, "matrix": "moving-resting", "register": "pp",
        "tension_gev_per_fm": 9.0, "coupling_scale_gev": 1.0});
    let out = ql(&["kernel", "m3-binding", "-", "--json"], &request);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Tension"));
}

#[test]
fn the_capabilities_name_the_binding_engine() {
    let out = ql(&["capabilities", "--json"], &json!({}));
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("kernel.m3-binding"));
}
