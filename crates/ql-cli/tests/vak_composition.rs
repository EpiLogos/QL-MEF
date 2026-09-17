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

#[test]
fn native_cli_lineage_retains_return_snapshots_and_replays_identically() {
    let mut request: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    request["steps"]
        .as_array_mut()
        .unwrap()
        .push(json!({"op":"lineage","reference":"offered"}));
    let file = std::env::temp_dir().join(format!("ql-lineage-{}.json", std::process::id()));
    std::fs::write(&file, serde_json::to_vec(&request).unwrap()).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["vak", "compose"])
        .arg(&file)
        .arg("--json")
        .output()
        .unwrap();
    let _ = std::fs::remove_file(file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        value,
        ql_cli::vak_composition::execute_request(&request).unwrap()
    );
    let lineage = &value["results"].as_array().unwrap().last().unwrap()["result"];
    assert!(
        lineage["determinations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["reference"] == "d")
    );
    assert_eq!(lineage["returns"][0]["reference"], "r");
    assert_eq!(lineage["returns"][0]["producing"]["standing"], "DERIVED");
    assert_eq!(lineage["returns"][0]["route"]["anchorRef"], "anchor:outer");
}

fn oikonomia_specimen() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/kernel/vak-cprime-oikonomia-v1.json")
}
#[test]
fn full_cs0_walk_survives_cfp_form_z_cycle_and_cs_profile_into_operative_return() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["vak", "compose"])
        .arg(oikonomia_specimen())
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let v: Value = serde_json::from_slice(&output.stdout).unwrap();
    let results = v["results"].as_array().unwrap();
    let selected = &results[3]["result"];
    assert_eq!(
        selected["csWalk"]["marker"],
        "ql.cprime-oikonomia/v1:cs0:forward-synthesis:extent:6"
    );
    let inspected = &results[results.len() - 2]["result"];
    assert!(inspected["zCycle"]["complete"].as_bool().unwrap());
    assert!(inspected["csComplete"].as_bool().unwrap());
    let operative = &results[results.len() - 1]["result"];
    assert_eq!(
        operative["cfpForm"],
        "ql.cprime-oikonomia/v1:cfp0:base-one-voice"
    );
    assert_eq!(
        operative["csWalk"]["marker"],
        "ql.cprime-oikonomia/v1:cs0:forward-synthesis:extent:6"
    );
    let executed: Vec<(u64, u64)> = operative["csWalk"]["executed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| (h["from"].as_u64().unwrap(), h["to"].as_u64().unwrap()))
        .collect();
    assert_eq!(
        executed,
        vec![(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)]
    );
    assert_eq!(operative["return"]["standing"], "DERIVED");
}
#[test]
fn cs_hop_rejects_a_return_outside_the_selected_pair() {
    let mut v: Value =
        serde_json::from_slice(&std::fs::read(oikonomia_specimen()).unwrap()).unwrap();
    v["steps"][7]["target"] = json!("src0");
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(err.to_string().contains("CS pair/direction"));
}
#[test]
fn z_cycle_refuses_to_skip_stages_through_the_cli() {
    let mut v: Value =
        serde_json::from_slice(&std::fs::read(oikonomia_specimen()).unwrap()).unwrap();
    let steps = v["steps"].as_array().unwrap().clone();
    let reduced: Vec<Value> = steps
        .into_iter()
        .filter(|s| s["op"] != "z-advance" || s["stage"] != json!("compose"))
        .collect();
    v["steps"] = json!(reduced);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(err.to_string().contains("out of source-defined order"));
}
