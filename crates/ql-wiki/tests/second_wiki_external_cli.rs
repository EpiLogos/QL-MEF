use std::io::Write;
use std::process::{Command, Stdio};

use ql_wiki::{RefractionStatus, WikiRefractionResponse};

#[test]
fn research_canvas_d3_fixture_round_trips_through_real_refraction_executable() {
    let fixture = include_str!("../../../fixtures/qw4/research-refraction-request.json");
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql-wiki-refraction"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn ql-wiki-refraction");
    child
        .stdin
        .as_mut()
        .expect("child stdin")
        .write_all(fixture.as_bytes())
        .expect("write Research Canvas request");
    let output = child
        .wait_with_output()
        .expect("wait for refraction executable");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let response: WikiRefractionResponse =
        serde_json::from_slice(&output.stdout).expect("parse WikiRefractionResponse");
    assert_eq!(response.status, RefractionStatus::Complete);
    assert_eq!(response.target_ref, "research:frame:transition-anomaly");
    assert_eq!(
        response.target_snapshot_hash,
        "sha256:research-transition-anomaly-v5"
    );
    assert_eq!(response.readings.len(), 3);
    let disclosures = response
        .readings
        .iter()
        .map(|reading| reading.disclosure.as_str())
        .collect::<Vec<_>>();
    assert!(disclosures.iter().any(|value| value.contains("Processual")));
    assert!(
        disclosures
            .iter()
            .any(|value| value.contains("Archetypal-Numerical"))
    );
    assert!(
        disclosures
            .iter()
            .any(|value| value.contains("Divine Logos"))
    );
    assert!(response.readings.iter().all(|reading| {
        reading.harmonic_field_ref.as_deref() == Some("ql:structural:2.0.0:field:A:1:D3")
            && reading.target_revision.as_ref().unwrap().to_string() == "5"
            && reading.relation_candidates.is_empty()
    }));
}
