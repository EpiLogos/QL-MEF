//! `ql shape bind`: the shape binding of a constellation a caller drafts (O-I #742, the Canvas's Form page). QL names the canonical
//! shape the members disclose, the binding a Wiki keeps for it (`ql-mef/shape-binding/v1`), and the identity of each same-position
//! generated relation the 6 / 6′ -> 6+6′ operation supplies, so a Wiki owner can fold the whole. It creates no member and asserts no
//! relation: the generated relation's semantic content stays an attributable return.
use serde_json::{Value, json};
use std::io::Write as _;
use std::process::{Command, Stdio};

fn bind(request: &Value) -> (bool, Value, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["shape", "bind", "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(request.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let value = serde_json::from_str(&stdout).unwrap_or(Value::Null);
    (
        output.status.success(),
        value,
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn member(subject: &str, position: u8, face: &str) -> Value {
    json!({"subject_ref": subject, "position": position, "face": face})
}

fn request(members: Vec<Value>) -> Value {
    json!({
        "subject_ref": "wiki:frame:form:wiki:space:inquiry",
        "whole_ref": "wiki:space:inquiry",
        "members": members,
        "provenance": {"caller_ref": "oi:techne:canvas:form", "source_ref": "wiki:space:inquiry", "standing_ref": "proposed"}
    })
}

fn twelve() -> Vec<Value> {
    let mut members = Vec::new();
    for position in 0..6u8 {
        members.push(member(
            &format!("wiki:node:d{position}"),
            position,
            "direct",
        ));
        members.push(member(
            &format!("wiki:node:c{position}"),
            position,
            "conjugate",
        ));
    }
    members
}

#[test]
fn a_twelvefold_draft_binds_to_its_shape_and_its_six_generation_sites() {
    let (ok, reading, stderr) = bind(&request(twelve()));
    assert!(ok, "{stderr}");
    assert_eq!(reading["schema"], "ql.shape-bind/v1");
    assert_eq!(reading["grain"], "twelvefold");
    assert_eq!(
        reading["shape_ref"],
        "ql:shape:1.0.0:constellation:twelvefold"
    );
    assert_eq!(reading["fold_count"], 12);
    // The binding is the Wiki's `ql-mef/shape-binding/v1` view of the same members, every member retained in its basis.
    assert_eq!(reading["binding_extension"], "ql-mef/shape-binding/v1");
    let binding = &reading["binding"];
    assert_eq!(binding["subject_ref"], "wiki:frame:form:wiki:space:inquiry");
    assert_eq!(binding["whole_ref"], "wiki:space:inquiry");
    assert_eq!(binding["shape_ref"], reading["shape_ref"]);
    assert_eq!(binding["members"].as_array().unwrap().len(), 12);
    assert_eq!(binding["basis_refs"].as_array().unwrap().len(), 12);
    // Six generation sites, each the kernel's same-position operator over the direct and conjugate member it names.
    let sites = reading["generation_sites"].as_array().unwrap();
    assert_eq!(sites.len(), 6);
    assert_eq!(sites[3]["position"], 3);
    assert_eq!(sites[3]["direct_ref"], "wiki:node:d3");
    assert_eq!(sites[3]["conjugate_ref"], "wiki:node:c3");
    assert_eq!(
        sites[3]["operator_ref"],
        "ql:shape:1.0.0:generation:same-position-direct-conjugate:position-3"
    );
    assert_eq!(
        reading["generated"]["3"],
        "ql:shape:1.0.0:generation:same-position-direct-conjugate:position-3:of:wiki:space:inquiry"
    );
    assert_eq!(reading["generated"].as_object().unwrap().len(), 6);
    assert_eq!(reading["constraints"]["creates_members"], false);
    assert_eq!(reading["constraints"]["asserts_relations"], false);
    assert_eq!(
        reading["constraints"]["semantic_generation_requires_attributable_return"],
        true
    );
}

#[test]
fn a_partial_form_binds_with_generation_only_where_both_faces_stand() {
    let (ok, reading, stderr) = bind(&request(vec![
        member("wiki:node:a", 1, "direct"),
        member("wiki:node:b", 2, "direct"),
        member("wiki:node:c", 3, "direct"),
    ]));
    assert!(ok, "{stderr}");
    assert_eq!(reading["grain"], "threefold-123");
    assert_eq!(
        reading["shape_ref"],
        "ql:shape:1.0.0:constellation:threefold-123"
    );
    assert_eq!(reading["generation_sites"], json!([]));
    assert_eq!(reading["generated"], json!({}));
}

#[test]
fn a_draft_the_structural_contract_refuses_is_refused_in_its_words() {
    // A conjugate with no direct on its position.
    let (ok, _, stderr) = bind(&request(vec![
        member("wiki:node:a", 1, "direct"),
        member("wiki:node:b", 4, "conjugate"),
    ]));
    assert!(!ok);
    assert!(
        stderr.contains("conjugate position 4 requires its direct position"),
        "{stderr}"
    );
    // Two members on one role.
    let (ok, _, stderr) = bind(&request(vec![
        member("wiki:node:a", 1, "direct"),
        member("wiki:node:b", 1, "direct"),
    ]));
    assert!(!ok);
    assert!(
        stderr.contains("duplicate structural coordinate"),
        "{stderr}"
    );
    // The whole is the 0/1 anchor, never a member of its own constellation.
    let (ok, _, stderr) = bind(&request(vec![member("wiki:space:inquiry", 1, "direct")]));
    assert!(!ok);
    assert!(stderr.contains("whole"), "{stderr}");
    // Members that disclose no canonical constellation have no shape to bind.
    let (ok, _, stderr) = bind(&request(vec![
        member("wiki:node:a", 1, "direct"),
        member("wiki:node:b", 3, "direct"),
        member("wiki:node:c", 5, "direct"),
    ]));
    assert!(!ok);
    assert!(stderr.contains("no canonical QL constellation"), "{stderr}");
}

#[test]
fn capabilities_disclose_the_shape_bind_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["capabilities", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let v: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        v["commands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|command| command == "shape.bind")
    );
}
