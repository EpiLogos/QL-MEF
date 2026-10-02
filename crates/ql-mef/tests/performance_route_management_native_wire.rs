//! Complete actual Rust N/A/P admissions drive the one current native callback
//! and paired checkpoint. Discovery uses the existing bounded native floor.
#[path = "support/performance_route_management_fixture.rs"]
mod producer;
use std::io::Write;
use std::process::{Command, Stdio};
#[test]
#[ignore = "requires the matching native floor's performance_route_management_wire binary"]
fn actual_current_source_nine_programmes_janko_allroute_hold_checkpoint() {
    let binary = std::env::var("QL_NATIVE_WIRE_TEST").expect("native floor binary");
    let packet = producer::packet().expect("actual native N/A/P producer");
    let bytes = serde_json::to_vec(&packet).unwrap();
    assert!(bytes.len() < 8 * 1024 * 1024);
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("current native nine-program callback test");
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "native consumer failed: {}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}
