//! Real process transport to the shared QL owner. Controlled proposals prove
//! admission laws; they do not claim classifier or installed-body performance.
use ql_mef::agent_event::{
    DecisionResponse, ProjectionRequest, admit_decision, project_event, value_digest,
};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn invoke(operation: &str, input: &Value) -> Result<Value, String> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["agent-event", operation, "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(input).unwrap())
        .unwrap();
    let result = child.wait_with_output().unwrap();
    if !result.status.success() {
        return Err(String::from_utf8_lossy(&result.stderr).into_owned());
    }
    serde_json::from_slice(&result.stdout).map_err(|e| e.to_string())
}

fn request() -> Value {
    let mut event: Value = serde_json::from_str(include_str!(
        "../../../fixtures/agent-decision/v1/event-v1.json"
    ))
    .unwrap();
    event["observed"] = json!([
        {"field":"lens","value":"L2'","origin":"observed","basis_refs":[event["material"]["ref"]]},
        {"field":"local-position","value":3,"origin":"observed","basis_refs":[event["material"]["ref"]]},
        {"field":"coordinate-face","value":"direct","origin":"observed","basis_refs":[event["material"]["ref"]]},
        {"field":"musical-basis","value":"chromatic","origin":"observed","basis_refs":[event["material"]["ref"]]}]);
    json!({"event":event,"requested_heads":["lens"]})
}

#[test]
fn public_projection_frame_and_harmonics_match_the_real_owner_exactly() {
    let input = request();
    let owner = project_event(serde_json::from_value(input.clone()).unwrap()).unwrap();
    assert_eq!(
        invoke("project", &input).unwrap(),
        serde_json::to_value(&owner).unwrap()
    );
    assert_eq!(invoke("frame", &input).unwrap(), owner.frame);
    assert_eq!(invoke("harmonic", &input).unwrap(), owner.harmonic);
    assert!(owner.decision_head_ids.is_empty());
    assert!(owner.determination.get("provider").is_none());
}

#[test]
fn public_admission_preserves_provider_and_refuses_impossible_labels() {
    let mut input = request();
    input["event"]["observed"] = json!([]);
    let projection: ProjectionRequest = serde_json::from_value(input.clone()).unwrap();
    let owner = project_event(projection.clone()).unwrap();
    let response = json!({"schema":"ql.agent-decision-response/v1","event_basis_digest":owner.frame["event_basis_digest"],
        "frame_digest":value_digest(&owner.frame).unwrap(),"kernel_basis":owner.frame["kernel_basis"],"outcome":"answered",
        "provider":{"provider_ref":"test:adversarial-input","model_ref":"test:selections","model_revision":"1","runtime_revision":"1"},
        "proposals":[{"head_id":"semantic-lens","label_ids":["L99"],"spans":[]}]});
    let expected = admit_decision(
        projection,
        serde_json::from_value::<DecisionResponse>(response.clone()).unwrap(),
    )
    .unwrap();
    let actual = invoke("validate", &json!({"projection":input,"response":response})).unwrap();
    assert_eq!(actual, serde_json::to_value(expected).unwrap());
    assert_eq!(
        actual["projection"]["determination"]["validated"],
        json!([])
    );
    assert_eq!(
        actual["response"]["proposals"][0]["label_ids"],
        json!(["L99"])
    );
}

#[test]
fn public_commands_refuse_invalid_or_extra_input() {
    assert!(
        invoke("predict", &request())
            .unwrap_err()
            .contains("unknown agent-event")
    );
    let mut invalid = request();
    invalid["model_protocol"] = json!("provider-shadow-ontology");
    assert!(
        invoke("project", &invalid)
            .unwrap_err()
            .contains("unknown field")
    );
    let mut invalid = request();
    invalid["event"]["observed"][0]["value"] = json!("L9");
    assert!(invoke("project", &invalid).is_err());
}
