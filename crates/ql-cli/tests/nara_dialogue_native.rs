//! Exercise the public CLI and embedded current registry, without provider or
//! registry doubles. Host refs are controlled inputs, not installed O:I proof.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn invoke(operation: &str, input: &Value) -> Result<Value, String> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["nara", operation, "-", "--json"])
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
    let output = child.wait_with_output().unwrap();
    if !output.status.success() {
        return Err(format!(
            "{}{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    Ok(serde_json::from_slice(&output.stdout).unwrap())
}

fn host_context(coordinate: &str) -> Value {
    json!({"schema":"ql.nara-dialogue-context/v1","context_ref":"context:controlled-native",
        "nara_ref":"nara:controlled","subject_ref":"person:controlled",
        "agent_session_ref":"agent-session/controlled-nara","m4_branch":null,
        "coordinate_ref":coordinate,"bimba":null,"expression_ref":"expression:controlled",
        "expression_revision":"7","profile_ref":"profile:controlled-host","profile_revision":"3",
        "scene_ref":null,"active_m_focus":"m0","pointed_ref":null,"hovered_ref":null,
        "pinned_refs":[],"occasion":null,"disclosed":[],"available_action_refs":[],
        "c_prime":null,"shared_field":null,"shared_reading":null,"expressive_act":null})
}

fn context(coordinate: &str) -> Value {
    invoke(
        "context",
        &json!({"schema":"ql.nara-dialogue-context-request/v1",
        "coordinate_ref":coordinate,"context":host_context(coordinate)}),
    )
    .unwrap()
}

fn delegate(context: &Value) -> Value {
    json!({"context":context,"delegation_ref":"delegation:controlled-native",
        "epii_session_ref":"agent-session/controlled-epii","brief":"Explain the selected native coordinate.",
        "scope_refs":[context["coordinate_ref"],context["expression_ref"]],"delegated_at_unix_ms":1})
}

fn enrichment(delegation: &Value) -> Value {
    json!({"schema":"ql.epii-enrichment/v1","enrichment_ref":"enrichment:controlled-native",
        "delegation_ref":delegation["delegation_ref"],"basis_context_ref":delegation["basis"]["context_ref"],
        "basis_expression_revision":delegation["basis"]["expression_revision"],
        "coordinate_refs":[delegation["basis"]["coordinate_ref"]],
        "source_refs":[],"method_refs":[],"evidence_refs":[],"standing":"proposed",
        "synthesis":"Retain this bounded proposal for the host to review.",
        "proposed_focus_refs":[delegation["basis"]["coordinate_ref"]],
        "proposed_scene_change_refs":[],"proposed_profile_variant_ref":null,
        "proposed_expressive_act_refs":[],"proposed_native_action_refs":[],
        "continuing_questions":[],"factory_commission_proposal":null,"returned_at_unix_ms":2})
}

#[test]
fn native_coordinate_profile_is_repeatable_source_bound_and_face_specific() {
    let request = json!({"coordinate_ref":"#2-1"});
    let first = invoke("coordinate", &request).unwrap();
    assert_eq!(first, invoke("coordinate", &request).unwrap());
    assert_eq!(first["schema"], "ql.coordinate-expression-binding/v1");
    assert_eq!(first["coordinate_ref"], "#2-1");
    assert_eq!(first["face"], "bimba");
    assert_eq!(first["profile_revision"], 1);
    assert!(
        first["resolved_profile_ref"]
            .as_str()
            .unwrap()
            .starts_with("profile:epi-coordinate-")
    );
    assert!(!first["grammar_sources"].as_array().unwrap().is_empty());
    let reflected = invoke(
        "coordinate",
        &json!({"coordinate_ref":"ql:m-coordinate:pratibimba:M2-1"}),
    )
    .unwrap();
    assert_eq!(reflected["coordinate_id"], first["coordinate_id"]);
    assert_eq!(reflected["face"], "pratibimba");
    assert_ne!(
        reflected["resolved_profile_ref"],
        first["resolved_profile_ref"]
    );
    assert!(
        invoke(
            "coordinate",
            &json!({"coordinate_ref":"ql:m-coordinate:pratibimba:M2-1","face":"bimba"})
        )
        .is_err()
    );
}

#[test]
fn native_context_resolves_registry_and_focus_without_inventing_a_profile_or_m4_branch() {
    let resolved = context("M2");
    let expected = ql_mef::aw1_world::resolve_rooted_m_world(
        ql_mef::m_tree::native_current_m_registry(),
        "M2",
    )
    .unwrap();
    assert_eq!(
        resolved["world"]["registry_revision"],
        expected.registry_revision
    );
    assert_eq!(
        resolved["context"]["bimba"]["registry_revision"],
        resolved["world"]["registry_revision"]
    );
    assert_eq!(resolved["context"]["active_m_focus"], "m2");
    assert!(resolved["context"]["m4_branch"].is_null());
    assert_eq!(
        resolved["context"]["profile_ref"],
        "profile:controlled-host"
    );
    assert_eq!(resolved["context"]["profile_revision"], "3");
    assert_eq!(resolved["profile_resolution"], "host-owned");
    assert_eq!(
        resolved["coordinate_binding"]["schema"],
        "ql.coordinate-expression-binding/v1"
    );
    assert_eq!(
        resolved["coordinate_binding"]["rooted_world"]["selected_id"],
        resolved["world"]["selected_id"]
    );
    assert_ne!(
        resolved["coordinate_binding"]["resolved_profile_ref"],
        resolved["context"]["profile_ref"]
    );
    let source_spelling = context("#4.1");
    assert_eq!(source_spelling["context"], context("M4.1")["context"]);
    assert_eq!(source_spelling["context"]["coordinate_ref"], "M4.1");
    assert_eq!(
        source_spelling["context"]["bimba"]["selected_source_ref"],
        "#4.1"
    );
    assert_eq!(
        source_spelling["context"],
        context("ql:m-coordinate:bimba:M4.1")["context"]
    );
    let mixed = context("#2-1");
    assert_eq!(mixed["context"]["coordinate_ref"], "M2-1");
    assert_eq!(mixed["context"]["bimba"]["selected_source_ref"], "#2-1");
    assert_eq!(mixed["context"]["active_m_focus"], "m2");
    for (coordinate, branch) in [
        ("M4.0", "identity"),
        ("M4.1", "embodied"),
        ("M4.2", "oracle"),
        ("M4.3", "transformation"),
        ("M4.4", "context"),
        ("M4.5", "integration"),
    ] {
        let resolved = context(coordinate);
        assert_eq!(resolved["context"]["active_m_focus"], "m4");
        assert_eq!(resolved["context"]["m4_branch"], branch);
    }
}

#[test]
fn native_context_refuses_unknown_coordinate_and_claimed_registry() {
    let unknown = json!({"schema":"ql.nara-dialogue-context-request/v1",
        "coordinate_ref":"M9.9","context":host_context("M9.9")});
    assert!(
        invoke("context", &unknown)
            .unwrap_err()
            .contains("unknown rooted M coordinate")
    );
    let resolved = context("M2");
    let forged = json!({"schema":"ql.nara-dialogue-context-request/v1",
        "coordinate_ref":"M2","context":resolved["context"]});
    assert!(
        invoke("context", &forged)
            .unwrap_err()
            .contains("no claimed Bimba registry")
    );
}

#[test]
fn native_delegation_refuses_forged_registry_and_undisclosed_scope() {
    let resolved = context("M2");
    let mut request = delegate(&resolved["context"]);
    request["context"]["bimba"]["registry_revision"] = json!("forged");
    assert!(
        invoke("delegate", &request)
            .unwrap_err()
            .contains("current native coordinate")
    );
    let mut request = delegate(&resolved["context"]);
    request["scope_refs"] = json!(["private:undisclosed"]);
    assert!(invoke("delegate", &request).is_err());
    let admitted = invoke("delegate", &delegate(&resolved["context"])).unwrap();
    assert_eq!(
        admitted["basis"]["bimba_registry_revision"],
        resolved["world"]["registry_revision"]
    );
    assert_eq!(admitted["state"]["state"], "delegated");
}

#[test]
fn native_enrichment_retains_stale_return_but_refuses_wrong_delegation() {
    let current = context("M2")["context"].clone();
    let delegation = invoke("delegate", &delegate(&current)).unwrap();
    let returned = enrichment(&delegation);
    let request = json!({"delegation":delegation,"enrichment":returned,"current":current});
    let valid = invoke("enrichment", &request).unwrap();
    assert_eq!(valid["apply_allowed"], true);
    assert_eq!(valid["applied"], false);
    assert_eq!(valid["delegation"]["state"]["state"], "returned");
    for (field, next) in [("expression_revision", "8"), ("profile_revision", "4")] {
        let mut stale = request.clone();
        stale["current"][field] = json!(next);
        let retained = invoke("enrichment", &stale).unwrap();
        assert_eq!(retained["apply_allowed"], false);
        assert_eq!(retained["applied"], false);
        assert_eq!(retained["enrichment"], request["enrichment"]);
        assert!(
            retained["reason"]
                .as_str()
                .is_some_and(|value| !value.is_empty())
        );
    }
    let mut wrong = request.clone();
    wrong["enrichment"]["delegation_ref"] = json!("delegation:another");
    assert!(
        invoke("enrichment", &wrong)
            .unwrap_err()
            .contains("another delegation")
    );
    let mut wrong = request;
    wrong["enrichment"]["basis_context_ref"] = json!("context:another");
    assert!(
        invoke("enrichment", &wrong)
            .unwrap_err()
            .contains("delegation's basis")
    );
}

#[test]
fn native_conjugate_context_keeps_face_and_delegates_on_the_same_source() {
    let direct = context("#4.1");
    let conjugate = context("ql:m-coordinate:pratibimba:M4.1");
    assert_eq!(conjugate["context"], direct["context"]);
    assert_eq!(conjugate["coordinate_binding"]["face"], "pratibimba");
    assert_eq!(
        conjugate["coordinate_binding"]["coordinate_id"],
        direct["coordinate_binding"]["coordinate_id"]
    );
    assert_ne!(
        conjugate["coordinate_binding"]["resolved_profile_ref"],
        direct["coordinate_binding"]["resolved_profile_ref"]
    );
    assert!(invoke("delegate", &delegate(&conjugate["context"])).is_ok());
}

#[test]
fn receive_preserves_valid_return_without_claiming_current_context_or_apply_authority() {
    let resolved = context("M2");
    let delegation = invoke("delegate", &delegate(&resolved["context"])).unwrap();
    let returned = enrichment(&delegation);
    let receipt = invoke(
        "receive",
        &json!({"delegation":delegation,"enrichment":returned}),
    )
    .unwrap();
    assert_eq!(receipt["schema"], "ql.nara-enrichment-receipt/v1");
    assert_eq!(receipt["enrichment"], returned);
    assert_eq!(receipt["apply_allowed"], false);
    assert_eq!(receipt["applied"], false);
    let mut wrong = returned.clone();
    wrong["delegation_ref"] = json!("another-delegation");
    assert!(
        invoke(
            "receive",
            &json!({"delegation":delegation,"enrichment":wrong})
        )
        .unwrap_err()
        .contains("another delegation")
    );
    let mut malformed = returned.clone();
    malformed["standing"] = json!("source");
    assert!(
        invoke(
            "receive",
            &json!({"delegation":delegation,"enrichment":malformed})
        )
        .unwrap_err()
        .contains("source standing")
    );
}
