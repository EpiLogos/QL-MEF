use serde_json::{Value, json};
use std::path::PathBuf;

fn specimen() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/kernel/vak-composition-v1.json")
}
#[test]
fn actual_cli_executes_nested_framing_generation_return_and_reentry() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["vak", "compose"])
        .arg(specimen())
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(v["contract"], "ql.vak-composition/v1");
    let results = v["results"].as_array().unwrap();
    let original = &results[5]["result"];
    let changed = &results[7]["result"];
    assert_ne!(original["childIntervals"], changed["childIntervals"]);
    assert_ne!(
        original["geometry"]["childPhaseDegrees"],
        changed["geometry"]["childPhaseDegrees"]
    );
    assert_eq!(
        original["children"][0]["frame"],
        changed["children"][0]["frame"]
    );
    assert_eq!(changed["transitions"].as_array().unwrap().len(), 1);
    let returned = &results[9]["result"];
    assert_eq!(returned["route"]["anchorRef"], "anchor:outer");
    assert_eq!(returned["route"]["groundRef"], "ground:outer");
    assert_eq!(returned["producing"]["standing"], "DERIVED");
    assert_eq!(
        returned["producing"]["reading"]["sourceReturns"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    let reopened = &results[12]["result"];
    assert_eq!(reopened["children"][0]["binding"]["subjectRef"], "d");
    assert_eq!(
        reopened["children"][0]["binding"]["provenance"]["standing"],
        "DERIVED"
    );
    assert_eq!(
        reopened["children"][0]["children"][0]["children"][0]["binding"]["subjectRef"],
        "subject:a"
    );
}
#[test]
fn cli_rejects_invalid_frames_before_returning_a_success_receipt() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    v["steps"][0]["frame"]["id"] = json!("CF8");
    assert!(ql_cli::vak_composition::execute_request(&v).is_err());
    v["steps"][0]["frame"]["id"] = json!("CF2");
    v["steps"][4]["row"] = json!("unresolved:whole");
    assert!(ql_cli::vak_composition::execute_request(&v).is_err());
}
#[test]
fn explicit_cp_member_choice_survives_changed_frame_and_mef_view() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = vec![v["steps"][0].clone()];
    let basis = v["steps"][0]["basis"].clone();
    steps.extend([
        json!({"op":"enter","id":"ctx","useRef":"a","categoryGround":{"position":4,"face":"direct"}}),
        json!({"op":"cp","context":"ctx","into":"cp-a","coordinate":{"position":4,"face":"direct"},"positions":"local","basis":basis}),
        json!({"op":"read","useRef":"cp-a","lens":"L0"}),
        json!({"op":"cf","context":"ctx","into":"cf-a","frame":{"id":"CF3","lens":"L0","basis":"chromatic","face":"direct","positions":"local"},"basis":basis}),
        json!({"op":"read","useRef":"cf-a","lens":"L5'"}),
    ]);
    v["steps"] = json!(steps);
    let r = ql_cli::vak_composition::execute_request(&v).unwrap();
    let before = &r["results"][3]["result"];
    let after = &r["results"][5]["result"];
    assert_eq!(before["address"]["member"]["coordinate"]["position"], 4);
    assert_eq!(before["address"], after["address"]);
    assert_eq!(before["harmonicPitch"], after["harmonicPitch"]);
    assert_ne!(before["focusInterval"], after["focusInterval"]);
    assert_eq!(after["geometry"]["viewingAbsolutePosition"], 4);
}
