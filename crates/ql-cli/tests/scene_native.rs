//! `ql scene compose` through the actual executable and the qualified sky provider.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn a_dated_sky_composes_through_the_installed_command() {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v4.json"
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
    // Historic installed-source snapshots stay historic. Each must refuse
    // after an accepted map correction rather than be stamped current.
    for historic in [
        &include_bytes!("../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json")[..],
        &include_bytes!("../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v2.json")[..],
        &include_bytes!("../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v3.json")[..],
    ] {
        let old_sky: Value = serde_json::from_slice(historic).unwrap();
        let stale = native_scene(
            "compose",
            &json!({"schema":"ql.scene-request/v1","sky_snapshot":old_sky,"tick12":3,"cycle":7}),
        );
        assert!(!stale.status.success());
        assert!(String::from_utf8_lossy(&stale.stderr).contains("native source binding is stale"));
    }
    let refused = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["scene", "compose", "/nonexistent.json"])
        .output()
        .unwrap();
    assert!(!refused.status.success());
}

fn native_scene(operation: &str, request: &Value) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["scene", operation, "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(request).unwrap())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn a_retained_occasion_builds_a_world_without_rewriting_the_sky() {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v4.json"
    ))
    .unwrap();
    let request = json!({"schema":"ql.scene-world-request/v1",
        "instance_ref":"test:retained-occasion-world", "subject_ref":"test:controlled-person",
        "event_ref":sky["snapshot_ref"], "sky":sky, "texture":[8,8], "units_per_metre":120,
        "start":{"tick12":3,"cycle":7,"aperture":9}, "snapshot_purpose":"retained-occasion"});
    let out = native_scene("world", &request);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let world: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(world["sky"], sky);
    assert_eq!(world["event_ref"], sky["snapshot_ref"]);
    assert_eq!(world["event"]["source_receipts"], json!([sky]));
    assert_eq!(world["sky_admission"]["fresh_current_attested"], false);
    let q = &world["sky_admission"]["source_binding_qualification"];
    assert_eq!(q["legacy_descriptor_admitted"], false);
    assert_eq!(q["native_sun_route"]["chakra_coordinate"], "#2-5-0/1-7");
    assert_eq!(world["scene"]["bodies"].as_array().unwrap().len(), 10);
    assert_eq!(world["scene"]["centres"].as_array().unwrap().len(), 7);
    assert_eq!(
        world["basis"]["derivation"]["sky_voices"]
            .as_array()
            .unwrap()
            .len(),
        9
    );
    assert_eq!(world["binding"]["native_basis"], world["basis"]);
}

#[test]
fn the_known_legacy_sky_is_retained_and_refused_after_its_registry_moved() {
    // The e6d exception held on its own registry only; the exact snapshot is
    // kept and refused for a retained occasion and for a fresh request.
    let legacy: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-known-e6d-2026-09-15-v1.json"
    ))
    .unwrap();
    // A different old adapter is provenance, not the approved e6d exception.
    let other: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    for (sky, purpose) in [
        (&legacy, "retained-occasion"),
        (&legacy, "requested"),
        (&other, "retained-occasion"),
    ] {
        let request = json!({"schema":"ql.scene-world-request/v1",
            "instance_ref":"test:exact-legacy-sun-world", "subject_ref":"test:controlled-person",
            "event_ref":sky["snapshot_ref"], "sky":sky, "texture":[8,8], "units_per_metre":120,
            "start":{"tick12":3,"cycle":7,"aperture":9}, "snapshot_purpose":purpose});
        let refused = native_scene("world", &request);
        assert!(!refused.status.success());
        assert!(
            String::from_utf8_lossy(&refused.stderr).contains("native source binding is stale")
        );
    }
}
