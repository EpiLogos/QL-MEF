use serde_json::{Value, json};
use std::path::PathBuf;

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/kernel/techne-reading-target-v1.json")
}

#[test]
fn techne_reading_returns_the_validated_production_reading_and_selections() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["techne", "reading"])
        .arg(fixture())
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(v["contract"], "ql.techne/v1");
    let reading = &v["reading"];
    assert_eq!(reading["subject"]["subject_ref"], "example:frame");
    assert_eq!(reading["whole"]["whole_ref"], "example:whole-anchor");
    assert_eq!(
        reading["whole"]["member_refs"],
        json!(["example:a", "example:b", "example:c"])
    );
    let available: Vec<&str> = reading["disclosure"]["instruments"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|i| i["available"].as_bool().unwrap_or(false))
        .map(|i| i["instrument"].as_str().unwrap())
        .collect();
    assert_eq!(available, vec!["project", "canvas"]);
    let selections = v["selections"].as_array().unwrap();
    assert_eq!(selections.len(), 2);
    assert_eq!(selections[0]["instrument"], "project");
    assert_eq!(selections[1]["instrument"], "canvas");
    assert_eq!(
        selections[0]["subject_ref"],
        reading["subject"]["subject_ref"]
    );
    // Plain mode reports the same reading legibly.
    let plain = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["techne", "reading"])
        .arg(fixture())
        .output()
        .unwrap();
    assert!(plain.status.success());
    let text = String::from_utf8_lossy(&plain.stdout);
    assert!(text.contains("example:frame"));
    assert!(text.contains("unavailable"));
}

#[test]
fn techne_reading_refuses_an_unbound_subject_instead_of_inventing_one() {
    let mut target: Value = serde_json::from_slice(&std::fs::read(fixture()).unwrap()).unwrap();
    target["target_ref"] = json!("example:absent");
    let path = std::env::temp_dir().join(format!("ql-techne-absent-{}.json", std::process::id()));
    std::fs::write(&path, serde_json::to_vec(&target).unwrap()).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["techne", "reading"])
        .arg(&path)
        .arg("--json")
        .output()
        .unwrap();
    let _ = std::fs::remove_file(path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("shape-binding subject must equal"));
}

#[test]
fn capabilities_disclose_the_techne_reading_command() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["capabilities", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let v: Value = serde_json::from_slice(&output.stdout).unwrap();
    let commands: Vec<&str> = v["commands"]
        .as_array()
        .expect("capabilities commands array")
        .iter()
        .map(|c| c.as_str().expect("command names are strings"))
        .collect();
    assert!(
        commands.contains(&"techne.reading"),
        "a dispatched command must be disclosed by `ql capabilities`: {commands:?}"
    );
}
