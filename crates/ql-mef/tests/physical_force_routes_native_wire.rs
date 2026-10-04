//! This paired test sends a complete actual native N/A/P producer result to
//! the current C++ owner. It runs under the existing discovered native floor.
#[path = "support/physical_force_routes_fixture.rs"]
mod producer;
use std::io::Write;
use std::process::{Command, Stdio};
#[test]
#[ignore = "requires the matching native floor's physical_force_routes_wire binary"]
fn actual_nine_source_operation_qualifies_distinct_projections_and_same_body_state() {
    let binary = std::env::var("QL_NATIVE_WIRE_TEST").expect("native floor binary");
    let packet = producer::packet().expect("actual native N/A/P producer");
    let bytes = serde_json::to_vec(&packet).unwrap();
    assert!(bytes.len() < 8 * 1024 * 1024);
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("current native transport test");
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "native consumer failed: {}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}
