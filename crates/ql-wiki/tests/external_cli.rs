use std::io::Write;
use std::process::{Command, Stdio};

use ql_wiki::{RefractionStatus, WIKI_REFRACTION_CONTRACT, WikiRefractionResponse};

#[test]
fn language_neutral_fixture_round_trips_through_real_refraction_executable() {
    let fixture = include_str!("../../../fixtures/qw2/wiki-refraction-request.json");
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
        .expect("write request fixture");
    let output = child
        .wait_with_output()
        .expect("wait for refraction executable");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let response: WikiRefractionResponse =
        serde_json::from_slice(&output.stdout).expect("parse executable response");
    assert_eq!(response.contract, WIKI_REFRACTION_CONTRACT);
    assert_eq!(response.status, RefractionStatus::Complete);
    assert_eq!(response.target_ref, "example:wiki:frame:decision-17");
    assert_eq!(
        response.target_snapshot_hash,
        "sha256:example-frame-snapshot"
    );
    assert_eq!(response.readings.len(), 3);
    assert_eq!(
        response.readings[0]
            .target_revision
            .as_ref()
            .unwrap()
            .to_string(),
        "7"
    );
    assert!(response.readings.iter().all(|reading| {
        reading.provider.provider_ref == "ql-mef:provider:registry-disclosure"
            && reading.provider.provider_version == "1.0.0"
    }));
    assert!(response.readings.iter().all(|reading| {
        reading.harmonic_field_ref.as_deref() == Some("ql:structural:2.0.0:field:A:1:D3")
    }));
    assert!(response.readings.iter().all(|reading| {
        reading.reading_type == "MEF-derived" && reading.relation_candidates.is_empty()
    }));
}
