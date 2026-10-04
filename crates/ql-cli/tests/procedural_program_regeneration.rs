//! Actual ql executable parsing/native compiler/C-prime qualification. Configured
//! source and journals are operands; these tests claim no private/live admission.
#[path = "../../ql-mef/tests/support/procedural_program.rs"]
mod support;
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Output, Stdio};
use support::*;
fn request(f: &Fixture) -> Value {
    let mut input = serde_json::to_value(&f.input).unwrap();
    input["schema"] = json!("ql.scene-procedural-request/v1");
    input["next"] = json!([]);
    input["native_context"] = f.native_context.clone();
    input
}
fn native(input: &Value) -> Output {
    let mut process = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["scene", "procedural", "regenerate", "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    process
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(input).unwrap())
        .unwrap();
    process.wait_with_output().unwrap()
}
fn compiled(input: &Value) -> Value {
    let output = native(input);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reply: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(reply["schema"], "ql.scene-procedural-response/v1");
    assert_eq!(reply["operation"], "regenerate");
    reply["result"].clone()
}
fn refused(input: &Value) {
    let original = input.clone();
    let output = native(input);
    assert!(!output.status.success());
    assert_eq!(input, &original);
    assert!(!output.stderr.is_empty());
}
#[test]
fn actual_program_regenerate_transport_uses_native_scene_force_atlas_and_final_cprime() {
    for f in [
        scene_human_fixture(),
        force_human_fixture(),
        atlas_human_fixture(),
    ] {
        let input = request(&f);
        let result = compiled(&input);
        let prepared = &result["prepared"];
        assert!(prepared["native_cprime"].is_object());
        assert_eq!(prepared["original_procedure"], input["procedure"]);
        assert_eq!(prepared["membership"], json!(f.first.membership));
        assert_eq!(prepared["native_edit"]["expected_revision"], 6);
        assert_eq!(prepared["output_readings"], input["output_readings"]);
        assert_eq!(prepared["required_consumers"], json!(["scene"]));
        let mut fingerprint_basis: ql_mef::procedural_composition::PreparedProcedure =
            serde_json::from_value(prepared.clone()).unwrap();
        let digest = fingerprint_basis.fingerprint.clone();
        fingerprint_basis.fingerprint.clear();
        assert_eq!(
            digest,
            ql_mef::procedural_manifestation::fingerprint(&fingerprint_basis).unwrap()
        );
        let changes = prepared["native_edit"]["changes"].as_array().unwrap();
        match &f.input.program {
            ql_mef::procedural_conduct::NativeRecipeProgram::SceneMaterial { .. } => {
                let scene = changes
                    .iter()
                    .find(|c| c["change"] == "scene_material_set" && c["scene_ref"] == OUTPUT)
                    .unwrap();
                assert_eq!(
                    scene["presentation"]["scene"]["entities"][0]["force"]["strength"],
                    0.875
                );
                assert_eq!(
                    scene["presentation"]["scene"]["entities"][0]["sequence"]["steps"][1]["hold"],
                    7.25
                );
                assert_eq!(
                    scene["presentation"]["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"],
                    true
                );
                assert!(!changes.iter().any(|c| c["change"] == "scene_create"));
            }
            ql_mef::procedural_conduct::NativeRecipeProgram::ForceParameters { .. } => {
                assert_eq!(
                    changes
                        .iter()
                        .filter(|c| c["change"] == "parameter_set")
                        .count(),
                    2
                );
                assert_eq!(changes.iter().find(|c|c["change"]=="parameter_set"&&c["parameter"]=="force_radius").unwrap()["value"],320);
                assert_eq!(changes[0]["change"], "scene_material_set");
            }
            ql_mef::procedural_conduct::NativeRecipeProgram::AtlasPassage { .. } => {
                assert_eq!(
                    changes.iter().find(|c| c["change"] == "focus").unwrap()["scene_ref"],
                    SOURCE
                );
                assert_eq!(
                    changes
                        .iter()
                        .find(|c| c["change"] == "scene_reorder")
                        .unwrap()["scene_refs"],
                    json!([SOURCE, SHARED])
                );
            }
        }
        let mut wrong = input.clone();
        wrong["next"] = json!(f.first.contributions);
        refused(&wrong);
        let mut wrong = input.clone();
        wrong["procedure"]["recipe_parameters"]["native_program"] = json!({});
        refused(&wrong);
        let mut wrong = input.clone();
        wrong["output_readings"][0]["document_revision"] = json!(5);
        refused(&wrong);
        let mut wrong = input.clone();
        wrong["native_context"]["thread_plan"]["legs"][0]["scope_ref"] = json!("foreign:scope");
        refused(&wrong);
        let mut wrong = input;
        wrong["current"][0]["overlays"] = json!([{"contribution_ref":wrong["previous"][0]["contribution_ref"],"pointer":"/value",
            "value":999,"actor_ref":"caller","persistent":true,"operation":"set"}]);
        refused(&wrong);
    }
}

#[test]
fn actual_cli_no_change_qualifies_current_native_graph_without_preparation_or_edit() {
    let f = unchanged_scene_fixture();
    let input = request(&f);
    let before = input.clone();
    let result = compiled(&input);
    assert_eq!(result["outcome"], "no_change");
    assert!(result["prepared"].is_null());
    assert_eq!(result["membership"], json!(f.first.membership));
    let basis = &result["no_change_basis"];
    assert_eq!(basis["schema"], "ql.native-procedural-no-change/v1");
    assert_eq!(basis["original_procedure"], input["procedure"]);
    assert_eq!(basis["expression_ref"], input["expression_ref"]);
    assert_eq!(basis["document_revision"], input["document_revision"]);
    assert_eq!(basis["operation_ref"], input["operation_ref"]);
    assert_eq!(basis["required_consumers"], input["required_consumers"]);
    assert_eq!(
        basis["current_inputs_fingerprint"],
        json!(ql_mef::procedural_manifestation::fingerprint(&f.input).unwrap())
    );
    let qualified = &result["source_qualification"];
    assert_eq!(qualified["schema"], "ql.procedural-cprime-preparation/v1");
    assert_eq!(
        qualified["procedure_ref"],
        input["procedure"]["procedure_ref"]
    );
    assert_eq!(
        qualified["procedure_revision"],
        input["procedure"]["revision"]
    );
    assert_eq!(qualified["definition"], input["procedure"]["composition"]);
    assert_eq!(
        qualified["thread_plan"],
        input["native_context"]["thread_plan"]
    );
    assert!(!qualified["fingerprint"].as_str().unwrap().is_empty());
    assert_eq!(input, before);
    let mut configuration_only = input.clone();
    configuration_only["native_context"] = Value::Null;
    let unqualified = compiled(&configuration_only);
    assert_eq!(unqualified["outcome"], "no_change");
    assert!(unqualified["prepared"].is_null());
    assert!(unqualified["source_qualification"].is_null());
    for mutation in 0..3 {
        let mut wrong = input.clone();
        match mutation {
            0 => {
                wrong["native_context"]["thread_plan"]["legs"][0]["scope_ref"] =
                    json!("foreign:scope")
            }
            1 => {
                wrong["native_context"]["currentness"]["expected"]["binding_revision"] =
                    json!("stale")
            }
            _ => wrong["output_readings"][0]["document_revision"] = json!(5),
        }
        refused(&wrong);
    }
    // Same generated body with authored metadata changes is a real native Edit.
    for mutation in 0..3 {
        let mut changed = input.clone();
        match mutation {
            0 => changed["procedure"]["revision"] = json!("2"),
            1 => {
                changed["procedure"]["conditions"] =
                    json!([{"condition":"equals","property":"authored_condition","value":true}])
            }
            _ => changed["materialization"]["rule_cursor"] = json!(1),
        }
        let result = compiled(&changed);
        assert_eq!(result["outcome"], "prepared");
        assert!(result["no_change_basis"].is_null());
        assert!(result["source_qualification"].is_null());
        let prepared = &result["prepared"];
        assert_eq!(prepared["original_procedure"], changed["procedure"]);
        assert!(prepared["native_cprime"].is_object());
        let changes = prepared["native_edit"]["changes"].as_array().unwrap();
        assert!(!changes.is_empty());
        assert!(changes.iter().all(|c| c["change"] == "scene_material_set"));
        let row = &changes.iter().find(|c| c["scene_ref"] == OUTPUT).unwrap()["presentation"]["scene"]
            ["procedural"]["procedures"][0];
        assert_eq!(row["definition"], changed["procedure"]);
        assert_eq!(row["cursor"], changed["materialization"]["rule_cursor"]);
    }
}

#[test]
fn actual_cli_cross_row_operand_preserves_separate_original_scalar_authority() {
    let f = cross_row_force_fixture();
    let input = request(&f);
    assert!(
        input["current_readings"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["properties"].as_object().unwrap().len() == 1)
    );
    let result = compiled(&input);
    assert_eq!(result["outcome"], "prepared");
    let prepared = &result["prepared"];
    assert_eq!(prepared["membership"], json!(f.first.membership));
    assert_eq!(prepared["original_procedure"], input["procedure"]);
    let changes = prepared["native_edit"]["changes"].as_array().unwrap();
    let writes = changes
        .iter()
        .filter(|c| c["change"] == "parameter_set")
        .collect::<Vec<_>>();
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0]["parameter"], "force_strength");
    assert_eq!(writes[0]["value"], 8);
    assert!(prepared["native_cprime"].is_object());
    for mutation in 0..3 {
        let mut wrong = input.clone();
        let rows = wrong["current_readings"].as_array_mut().unwrap();
        let radius = rows
            .iter()
            .position(|r| r["address"]["property"] == "radius")
            .unwrap();
        match mutation {
            0 => {
                rows.remove(radius);
            }
            1 => rows[radius]["properties"]["force_radius"] = json!(9),
            _ => rows[radius]["revision"] = json!(6),
        }
        refused(&wrong);
    }
    let mut wrong = input.clone();
    wrong["procedure"]["selector"]["refs"] = json!(
        f.input
            .current_readings
            .iter()
            .filter(|r| r.address.property.as_deref() == Some("strength"))
            .map(|r| r.occurrence_ref.clone())
            .collect::<Vec<_>>()
    );
    refused(&wrong);
    assert_eq!(input, request(&f));
}

#[test]
fn actual_cli_selected_original_membership_no_change_retains_native_join_history() {
    let f = selected_unchanged_scene_fixture();
    let input = request(&f);
    let result = compiled(&input);
    assert_eq!(result["outcome"], "no_change");
    assert!(result["prepared"].is_null());
    assert_eq!(result["membership"], json!(f.first.membership));
    assert!(result["source_qualification"].is_object());
    assert_eq!(
        result["no_change_basis"]["original_procedure"],
        input["procedure"]
    );
    assert_eq!(input, request(&f));
}
