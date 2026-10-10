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

// ---- C′ CLI end-to-end coverage: interpret, ct, cf, thread-binding cfp and
// the Z cycle over a non-base CFP form. Every negative is modelled on the
// kernel's actual fail-closed refusal paths, not on guessed messages.

/// The single R# step of the Freedom source R-path is the Techne coordinate.
const TECHNE_R_FACTOR_REF: &str = "M0-3-10-(0/1)";

static COVERAGE_REQUEST_FILE: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

fn basis(source: &str, standing: &str) -> Value {
    json!({
        "caller": "agent:zcode-cprime-coverage",
        "source": source,
        "revision": "synthetic-v1",
        "standing": standing,
        "evidence": ["specimen:caller-input"]
    })
}

/// Full-profile reading envelope in the exact shape `parse_language` accepts,
/// mirroring the library tests' synthetic interpretation language.
fn language(subject: &str, operator: &str, field: &str) -> Value {
    json!({
        "general": {
            "syntaxVersion": "aikit.operative-resolve/v1",
            "ownerRevision": "synthetic-owner-revision",
            "resolvePathIdentity": "test:resolve-path",
            "rendered": "cli specimen supplied AST rendering",
            "fullVakRendering": "cli specimen supplied full rendering",
            "evidence": ["test:not-an-AIKit-execution"]
        },
        "reading": {
            "contract": "vak-expression-reading-v1",
            "operator": operator,
            "horizon": "@5",
            "subjects": [{"native": subject}],
            "relationRefs": [],
            "complementRefs": [],
            "worldRef": "test:world",
            "standing": "DERIVED",
            "evidence": ["test:interpretation"]
        },
        "acceptedSyntaxRevision": "synthetic-owner-revision",
        "nativeNodeRef": format!("test:node:{subject}"),
        "selfOther": "!?",
        "field": field,
        "interpreter": "test:interpreter"
    })
}

fn entry(id: &str, whole: &str) -> Value {
    json!({"op":"enter","id":id,"useRef":whole,"categoryGround":{"position":4,"face":"direct"}})
}

/// Execute one request through the installed `ql` binary. The request file is
/// removed before this returns.
fn run_actual_cli(request: &Value) -> Value {
    use std::sync::atomic::Ordering;
    let file = std::env::temp_dir().join(format!(
        "ql-cprime-coverage-{}-{}.json",
        std::process::id(),
        COVERAGE_REQUEST_FILE.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&file, serde_json::to_vec(request).unwrap()).unwrap();
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
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn returned_whole_reenters_through_interpret_and_joins_cfp_again() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    // steps[9] returns and steps[10] offers `offered` standing DERIVED; the
    // returned field then re-enters composition as an interpreted whole and
    // participates in CFP and Return again.
    steps.extend([
        json!({"op":"interpret","from":"offered","into":"interpreted","language":language("d","+","R#"),"basis":basis("specimen:interpret","PROPOSED")}),
        entry("ctxr", "interpreted"),
        json!({"op":"cfp","context":"ctxr","act":"Freedom","bindings":[{"sourceStep":TECHNE_R_FACTOR_REF,"path":[]}],"basis":basis("specimen:reentry-cfp","PROPOSED")}),
        json!({"op":"determine","id":"d2","useRef":"interpreted","lens":"L0","context":"ctxr","basis":basis("specimen:determination","PROPOSED")}),
        json!({"op":"return","id":"r2","determination":"d2","target":"interpreted","kind":"own","context":"ctxr","basis":basis("specimen:Return","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let r = run_actual_cli(&v);
    let results = r["results"].as_array().unwrap();
    assert_eq!(results.len(), 18);
    let interpreted = &results[13]["result"];
    assert_eq!(interpreted["useRef"], "interpreted");
    assert_eq!(interpreted["binding"]["subjectRef"], "d");
    assert_eq!(interpreted["binding"]["provenance"]["standing"], "DERIVED");
    assert_eq!(interpreted["language"]["reading"]["standing"], "DERIVED");
    assert_eq!(interpreted["language"]["field"], "R#");
    assert!(
        interpreted["producingRefs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p == "offered")
    );
    let reentered = &results[15]["result"];
    assert_eq!(reentered["thread"], json!(["interpreted"]));
    assert_eq!(
        reentered["threadBindings"][0]["sourceStep"],
        TECHNE_R_FACTOR_REF
    );
    assert_eq!(reentered["threadBindings"][0]["path"], json!([]));
    assert_eq!(reentered["rPath"]["actRef"], TECHNE_R_FACTOR_REF);
    assert_eq!(
        reentered["rPath"]["steps"][0]["vakRef"],
        TECHNE_R_FACTOR_REF
    );
    assert_eq!(reentered["rPath"]["standing"], "SOURCE");
    let determined = &results[16]["result"];
    assert_eq!(determined["standing"], "DERIVED");
    assert_eq!(
        determined["context"]["readings"][0]["binding"]["subjectRef"],
        "d"
    );
    assert_eq!(
        determined["context"]["rPath"]["actRef"],
        TECHNE_R_FACTOR_REF
    );
    let returned = &results[17]["result"];
    assert_eq!(returned["standing"], "DERIVED");
    assert_eq!(returned["route"]["anchorRef"], "anchor:offered");
    assert_eq!(returned["route"]["fromRef"], "d2");
}

#[test]
fn interpret_refuses_non_derived_standing_and_missing_language() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    let mut forged = json!({"op":"interpret","from":"offered","into":"interpreted","language":language("d","+","R#"),"basis":basis("specimen:interpret","PROPOSED")});
    forged["language"]["reading"]["standing"] = json!("SOURCE");
    steps.push(forged);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string()
            .contains("new interpretation remains DERIVED or PROPOSED"),
        "{err}"
    );
    // The CLI envelope itself refuses an interpret without a full-profile reading.
    v["steps"][13].as_object_mut().unwrap().remove("language");
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string()
            .contains("interpret requires a full-profile reading"),
        "{err}"
    );
}

#[test]
fn ct_selects_source_qualified_fields_and_admission_follows_the_selection() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctx", "a"),
        json!({"op":"ct","context":"ctx","fields":["R#"],"basis":basis("specimen:ct","PROPOSED")}),
        json!({"op":"determine","id":"dct","useRef":"a","lens":"L0","context":"ctx","language":language("subject:a","=","R#"),"basis":basis("specimen:determination","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let r = ql_cli::vak_composition::execute_request(&v).unwrap();
    let results = r["results"].as_array().unwrap();
    assert_eq!(results.len(), 16);
    let selection = &results[14]["result"];
    assert_eq!(selection["contentFields"], json!(["R#"]));
    let receipt = selection["operations"].as_array().unwrap().last().unwrap();
    assert_eq!(receipt["family"], "CT");
    assert_eq!(receipt["outputRefs"], json!([TECHNE_R_FACTOR_REF]));
    let determined = &results[15]["result"];
    assert_eq!(determined["context"]["contentFields"], json!(["R#"]));
    assert_eq!(determined["standing"], "DERIVED");
}

#[test]
fn ct_refuses_unqualified_fields_and_excludes_unselected_content() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctx", "a"),
        json!({"op":"ct","context":"ctx","fields":["q#"],"basis":basis("specimen:ct","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(err.to_string().contains("unknown M0-4 field"), "{err}");

    // Narrowing CT to Bimba makes the Techne interpretation inadmissible.
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctx", "a"),
        json!({"op":"ct","context":"ctx","fields":["O#"],"basis":basis("specimen:ct","PROPOSED")}),
        json!({"op":"determine","id":"dct","useRef":"a","lens":"L0","context":"ctx","language":language("subject:a","=","R#"),"basis":basis("specimen:determination","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string()
            .contains("CT excludes this contextual content"),
        "{err}"
    );
}

#[test]
fn cf_reframes_into_an_explicit_active_frame_through_the_context() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctx", "a"),
        json!({"op":"read","useRef":"a","lens":"L0"}),
        json!({"op":"cf","context":"ctx","into":"a-cf5","frame":{"id":"CF5","lens":"L0","basis":"chromatic","face":"direct","positions":"local"},"basis":basis("specimen:cf","PROPOSED")}),
        json!({"op":"read","useRef":"a-cf5","lens":"L0"}),
    ]);
    v["steps"] = json!(steps);
    let r = ql_cli::vak_composition::execute_request(&v).unwrap();
    let results = r["results"].as_array().unwrap();
    assert_eq!(results.len(), 17);
    let before = &results[14]["result"];
    assert_eq!(before["frame"]["id"], "CF2");
    let reframed = &results[15]["result"];
    assert_eq!(reframed["focusUse"], "a-cf5");
    let receipt = reframed["operations"].as_array().unwrap().last().unwrap();
    assert_eq!(receipt["family"], "CF");
    assert_eq!(receipt["inputRefs"], json!(["a"]));
    assert_eq!(receipt["outputRefs"], json!(["a-cf5"]));
    let after = &results[16]["result"];
    assert_eq!(after["frame"]["id"], "CF5");
    assert_ne!(before["address"], after["address"]);
    let transitions = after["transitions"].as_array().unwrap();
    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0]["fromRef"], "a");
    assert_eq!(transitions[0]["from"]["id"], "CF2");
    assert_eq!(transitions[0]["into"]["id"], "CF5");
}

#[test]
fn cf_refuses_invalid_frames() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctx", "a"),
        json!({"op":"cf","context":"ctx","into":"bad","frame":{"id":"CF8","lens":"L0","basis":"chromatic","face":"direct","positions":"local"},"basis":basis("specimen:cf","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(err.to_string().contains("unknown canonical CF"), "{err}");

    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctx", "a"),
        json!({"op":"cf","context":"ctx","into":"bad","frame":{"id":"CF3","lens":"L0","basis":"quartertones","face":"direct","positions":"local"},"basis":basis("specimen:cf","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(err.to_string().contains("unknown harmonic basis"), "{err}");
}

#[test]
fn cfp_binds_a_recursive_whole_path_to_the_source_r_factor_path() {
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    v["steps"][0]["language"] = language("subject:a", "+", "R#");
    let mut steps = v["steps"].as_array().unwrap().clone();
    // The Freedom R-path step is bound to an explicit two-axis recursive path
    // through the composed field: outer → ab → a.
    steps.extend([
        entry("ctxo", "outer"),
        json!({"op":"cfp","context":"ctxo","act":"Freedom","bindings":[{"sourceStep":TECHNE_R_FACTOR_REF,"path":["row","row"]}],"basis":basis("specimen:cfp","PROPOSED")}),
        json!({"op":"determine","id":"dthread","useRef":"outer","lens":"L0","context":"ctxo","basis":basis("specimen:determination","PROPOSED")}),
        json!({"op":"return","id":"rthread","determination":"dthread","target":"outer","kind":"own","context":"ctxo","basis":basis("specimen:Return","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let r = ql_cli::vak_composition::execute_request(&v).unwrap();
    let results = r["results"].as_array().unwrap();
    assert_eq!(results.len(), 17);
    let bound = &results[14]["result"];
    assert_eq!(bound["focusUse"], "outer");
    assert_eq!(bound["thread"], json!(["a"]));
    assert_eq!(
        bound["threadBindings"][0]["sourceStep"],
        TECHNE_R_FACTOR_REF
    );
    assert_eq!(bound["threadBindings"][0]["path"], json!(["row", "row"]));
    assert_eq!(bound["rPath"]["steps"][0]["vakRef"], TECHNE_R_FACTOR_REF);
    let determined = &results[15]["result"];
    assert_eq!(determined["context"]["readings"][0]["useRef"], "a");
    assert_eq!(
        determined["context"]["threadBindings"][0]["path"],
        json!(["row", "row"])
    );
    let returned = &results[16]["result"];
    assert_eq!(returned["standing"], "DERIVED");
    assert_eq!(returned["route"]["anchorRef"], "anchor:outer");
}

#[test]
fn cfp_refuses_steps_without_attributable_whole_interpretation() {
    // The bound path step resolves to whole `a`, which carries no language.
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctxo", "outer"),
        json!({"op":"cfp","context":"ctxo","act":"Freedom","bindings":[{"sourceStep":TECHNE_R_FACTOR_REF,"path":["row","row"]}],"basis":basis("specimen:cfp","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string()
            .contains("CFP source step needs an attributable whole interpretation"),
        "{err}"
    );

    // Source-step identity is checked against the actual R-path order.
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    v["steps"][0]["language"] = language("subject:a", "+", "R#");
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.extend([
        entry("ctxo", "outer"),
        json!({"op":"cfp","context":"ctxo","act":"Freedom","bindings":[{"sourceStep":"M0-2-9","path":["row","row"]}],"basis":basis("specimen:cfp","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string()
            .contains("CFP source step order/identity mismatch"),
        "{err}"
    );

    // Every actual source R-path step must be bound exactly once.
    let mut v: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    v["steps"][0]["language"] = language("subject:a", "+", "R#");
    let mut steps = v["steps"].as_array().unwrap().clone();
    let duplicate = json!({"sourceStep":TECHNE_R_FACTOR_REF,"path":["row","row"]});
    steps.extend([
        entry("ctxo", "outer"),
        json!({"op":"cfp","context":"ctxo","act":"Freedom","bindings":[duplicate.clone(),duplicate],"basis":basis("specimen:cfp","PROPOSED")}),
    ]);
    v["steps"] = json!(steps);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string()
            .contains("CFP must bind every actual source R-path step"),
        "{err}"
    );
}

#[test]
fn z_cycle_runs_over_the_chain_form_and_reopens_into_iteration_two() {
    let mut v: Value =
        serde_json::from_slice(&std::fs::read(oikonomia_specimen()).unwrap()).unwrap();
    v["steps"][2]["form"] = json!("chain");
    let mut steps = v["steps"].as_array().unwrap().clone();
    steps.push(json!({
        "op":"z-reopen","oikonomia":"oik","context":"ctx",
        "reevaluationEvidence":["reevaluation:cs0-human"],
        "basis":basis("specimen:z-reopen","PROPOSED")
    }));
    v["steps"] = json!(steps);
    let r = run_actual_cli(&v);
    let results = r["results"].as_array().unwrap();
    assert_eq!(results.len(), 37);
    assert_eq!(
        results[2]["result"]["cfpForm"],
        "ql.cprime-oikonomia/v1:cfp2:chained-c-thread"
    );
    let operative = &results[35]["result"];
    assert_eq!(
        operative["cfpForm"],
        "ql.cprime-oikonomia/v1:cfp2:chained-c-thread"
    );
    assert_eq!(operative["zCycle"]["complete"], json!(true));
    assert_eq!(operative["zCycle"]["iteration"], 1);
    let z = &results[36]["result"]["zCycle"];
    assert_eq!(z["iteration"], 2);
    assert_eq!(z["expectedStage"], "ql.cprime-oikonomia/v1:z:compose");
    assert_eq!(z["complete"], json!(false));
    assert_eq!(z["stages"], json!([]));
    assert_eq!(z["reevaluationEvidence"], json!(["reevaluation:cs0-human"]));
}

#[test]
fn chain_form_z_cycle_still_refuses_stage_skips_and_premature_reopens() {
    // Skipping the compose stage remains out of source-defined order.
    let mut v: Value =
        serde_json::from_slice(&std::fs::read(oikonomia_specimen()).unwrap()).unwrap();
    v["steps"][2]["form"] = json!("chain");
    let reduced: Vec<Value> = v["steps"]
        .as_array()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|s| s["op"] != "z-advance" || s["stage"] != json!("compose"))
        .collect();
    v["steps"] = json!(reduced);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string().contains("out of source-defined order"),
        "{err}"
    );

    // Z cannot reopen before recompose.
    let mut v: Value =
        serde_json::from_slice(&std::fs::read(oikonomia_specimen()).unwrap()).unwrap();
    v["steps"][2]["form"] = json!("chain");
    let truncated: Vec<Value> = v["steps"]
        .as_array()
        .unwrap()
        .clone()
        .into_iter()
        .take(32)
        .collect();
    let mut truncated = truncated;
    truncated.push(json!({
        "op":"z-reopen","oikonomia":"oik","context":"ctx",
        "reevaluationEvidence":["reevaluation:too-early"],
        "basis":basis("specimen:z-reopen","PROPOSED")
    }));
    v["steps"] = json!(truncated);
    let err = ql_cli::vak_composition::execute_request(&v).unwrap_err();
    assert!(
        err.to_string().contains("Z cannot reopen before recompose"),
        "{err}"
    );
}
