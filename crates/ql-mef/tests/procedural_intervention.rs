//! Actual native manual attribution over the production authoring material.
use ql_mef::procedural_composition::{OverlayOperation, OwnedAddress};
use ql_mef::procedural_intervention::*;
use serde_json::{Value, json};
use std::collections::BTreeMap;
fn edit() -> ManualEdit {
    let mut material: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap();
    material["scene"]["id"] = json!("expression:acceptance:scene:canonical");
    let entities = material["scene"]["entities"].as_array().unwrap();
    let refs = entities
        .iter()
        .enumerate()
        .map(|(n, e)| {
            (
                e["id"].as_str().unwrap().into(),
                format!("expression:acceptance:entity:{n}"),
            )
        })
        .collect::<BTreeMap<_, _>>();
    // Each ordinary radius edit below writes 0.3. Start from a distinct real
    // authored baseline so the extractor exercises an actual delta.
    material["scene"]["entities"][0]["force"]["radius"] = json!(0.2);
    ManualEdit {
        expression_ref: "expression:acceptance".into(),
        scene_ref: "expression:acceptance:scene:canonical".into(),
        contribution_ref: "contribution:native".into(),
        owned_addresses: vec![OwnedAddress {
            expression_ref: "expression:acceptance".into(),
            scene_ref: Some("expression:acceptance:scene:canonical".into()),
            entity_ref: None,
            component: "scene".into(),
            constituent_ref: None,
            parent_ref: None,
            property: None,
        }],
        entity_refs: refs,
        before: material.clone(),
        after: material,
        actor_ref: "human:owner".into(),
        operation_ref: "ordinary-edit:actual".into(),
        document_revision: 9,
        retained_overlays: vec![],
        retained_native_records: vec![],
    }
}
#[test]
fn actual_human_edit_roundtrips_exact_native_rows_and_latest_overlay() {
    let mut edit = edit();
    edit.after["scene"]["entities"][0]["force"]["strength"] = json!(0.73);
    let source = extract_manual_interventions(&edit).unwrap();
    assert_eq!(source.native_records.len(), 1);
    let row = &source.native_records[0];
    assert_eq!(row.actor, "human:owner");
    assert_eq!(row.revision, 9);
    assert_eq!(row.operation_ref.as_deref(), Some("ordinary-edit:actual"));
    assert!(row.path.contains("/@"));
    assert_eq!(row.address.component, "force");
    assert_eq!(
        project_native_authored_interventions(&edit, &source.native_records).unwrap(),
        source.overlays
    );
    let mut old = row.clone();
    old.actor = "human:earlier".into();
    old.revision = 4;
    old.value = json!(0.1);
    let overlays =
        project_native_authored_interventions(&edit, &[row.clone(), old.clone()]).unwrap();
    assert_eq!(overlays.len(), 1);
    assert_eq!(overlays[0].value, json!(0.73));
    assert_eq!(overlays[0].actor_ref, "human:owner");
    let mut conflict = row.clone();
    conflict.value = json!(0.2);
    assert!(project_native_authored_interventions(&edit, &[row.clone(), conflict]).is_err());
    old.revision = 10;
    assert!(project_native_authored_interventions(&edit, &[old]).is_err());
}
#[test]
fn native_rows_refuse_forged_coordinate_index_journal_and_foreign_scope() {
    let mut edit = edit();
    edit.after["scene"]["entities"][0]["force"]["strength"] = json!(0.62);
    let row = extract_manual_interventions(&edit)
        .unwrap()
        .native_records
        .remove(0);
    let mut forged = row.clone();
    forged.address.property = Some("radius".into());
    assert!(project_native_authored_interventions(&edit, &[forged]).is_err());
    let mut forged = row.clone();
    forged.path = "/scene/entities/0/force/strength".into();
    assert!(project_native_authored_interventions(&edit, &[forged]).is_err());
    let mut forged = row.clone();
    forged.path = "/scene/procedural/operations".into();
    assert!(project_native_authored_interventions(&edit, &[forged]).is_err());
    let mut forged = row.clone();
    forged.address.expression_ref = "expression:foreign".into();
    assert!(project_native_authored_interventions(&edit, &[forged]).is_err());
    let mut forged = row;
    forged.actor.clear();
    assert!(project_native_authored_interventions(&edit, &[forged]).is_err());
}
#[test]
fn same_layer_id_parent_deletion_and_order_keep_exact_native_coordinate() {
    let mut edit = edit();
    edit.before["scene"]["entities"][0]["layers"] =
        json!([{"id":"shared","z":8},{"id":"other","z":4}]);
    edit.before["scene"]["entities"][0]["sequence"]["steps"][0]["layers"] =
        json!([{"id":"shared","z":9}]);
    edit.after = edit.before.clone();
    edit.after["scene"]["entities"][0]["layers"] =
        json!([{"id":"other","z":4},{"id":"shared","z":8}]);
    edit.after["scene"]["entities"][0]["sequence"]["steps"][0]["layers"] = json!([]);
    let source = extract_manual_interventions(&edit).unwrap();
    let state_id = edit.before["scene"]["entities"][0]["sequence"]["steps"][0]["id"]
        .as_str()
        .unwrap();
    let deletion = source
        .native_records
        .iter()
        .find(|r| r.kind == InterventionKind::Delete)
        .unwrap();
    assert_eq!(deletion.address.parent_ref, Some(Some(state_id.into())));
    assert_eq!(deletion.address.constituent_ref.as_deref(), Some("shared"));
    let overlays = project_native_authored_interventions(&edit, &source.native_records).unwrap();
    assert!(
        overlays
            .iter()
            .any(|r| r.operation == OverlayOperation::Delete)
    );
    assert!(
        overlays
            .iter()
            .any(|r| r.operation == OverlayOperation::Reorder)
    );
    let mut forged = deletion.clone();
    forged.address.parent_ref = Some(None);
    assert!(project_native_authored_interventions(&edit, &[forged]).is_err());
}

#[test]
fn later_child_manual_edit_refreshes_retained_parent_from_actual_material() {
    let mut edit = edit();
    let id = edit.before["scene"]["entities"][0]["id"].as_str().unwrap();
    let pointer = format!("/scene/entities/@{id}/force");
    edit.retained_overlays
        .push(ql_mef::procedural_composition::AuthoredOverlay {
            contribution_ref: edit.contribution_ref.clone(),
            pointer: pointer.clone(),
            value: json!({"strength":0.1,"radius":0.4}),
            actor_ref: "human:earlier".into(),
            persistent: true,
            operation: OverlayOperation::Set,
        });
    edit.after["scene"]["entities"][0]["force"]["strength"] = json!(0.83);
    let result = extract_manual_interventions(&edit).unwrap();
    let parent = result
        .native_records
        .iter()
        .find(|r| r.path == pointer)
        .unwrap();
    assert_eq!(parent.value, edit.after["scene"]["entities"][0]["force"]);
    assert_eq!(parent.actor, "human:owner");
    assert_eq!(parent.revision, 9);
    let projected = project_native_authored_interventions(&edit, &result.native_records).unwrap();
    assert_eq!(projected, result.overlays);
}

#[test]
fn actual_native_container_and_authored_scene_properties_have_no_synthetic_order() {
    let mut edit = edit();
    edit.before["scene"]["entities"][0]["layers"] = json!([{"id":"a","z":4},{"id":"b","z":8}]);
    edit.after = edit.before.clone();
    edit.after["scene"]["duration"] = json!(27);
    edit.after["scene"]["entities"][0]["layers"] = json!([{"id":"b","z":8},{"id":"a","z":4}]);
    let result = extract_manual_interventions(&edit).unwrap();
    let order = result
        .native_records
        .iter()
        .find(|r| r.kind == InterventionKind::Reorder)
        .unwrap();
    assert_eq!(order.address.component, "entity");
    assert_eq!(order.address.property.as_deref(), Some("layers"));
    let duration = result
        .native_records
        .iter()
        .find(|r| r.path == "/scene/duration")
        .unwrap();
    assert_eq!(duration.address.component, "property");
    assert_eq!(duration.address.property.as_deref(), Some("duration"));
    let projected = project_native_authored_interventions(&edit, &result.native_records).unwrap();
    assert_eq!(projected, result.overlays);
}

#[test]
fn real_whole_scene_deletion_is_a_stable_tombstone_and_requires_whole_scope() {
    let mut edit = edit();
    edit.after = Value::Null;
    let result = extract_manual_interventions(&edit).unwrap();
    assert_eq!(result.native_records.len(), 1);
    let row = &result.native_records[0];
    assert_eq!(row.kind, InterventionKind::Delete);
    assert_eq!(row.path, "/scene");
    assert_eq!(row.address.component, "scene");
    assert_eq!(row.address.property, None);
    assert_eq!(row.value, Value::Null);
    assert_eq!(result.overlays[0].operation, OverlayOperation::Delete);
    assert_eq!(
        project_native_authored_interventions(&edit, &result.native_records).unwrap(),
        result.overlays
    );
    edit.owned_addresses[0].component = "field".into();
    assert!(extract_manual_interventions(&edit).is_err());
}
#[test]
fn protected_native_rows_are_projected_before_edit_and_caller_overlay_forgery_refuses() {
    let mut edit = edit();
    edit.after["scene"]["entities"][0]["force"]["strength"] = json!(0.41);
    let previous = extract_manual_interventions(&edit).unwrap();
    edit.before = edit.after.clone();
    edit.after["scene"]["entities"][0]["force"]["strength"] = json!(0.52);
    edit.retained_native_records = previous.native_records;
    edit.document_revision = 10;
    let current = extract_manual_interventions(&edit).unwrap();
    assert_eq!(current.overlays[0].value, json!(0.52));
    edit.retained_overlays = current.overlays.clone();
    edit.retained_overlays[0].value = json!(999);
    assert!(
        extract_manual_interventions(&edit)
            .unwrap_err()
            .contains("caller overlays")
    );
}

#[test]
fn actual_selection_relation_and_scene_order_are_attributed_without_scene_material_delta() {
    let expression = "expression:acceptance";
    let a = "expression:acceptance:scene:canonical";
    let b = "expression:acceptance:scene:passage";
    let before = json!({"schema":"ql.native-atlas-state/v1","expression_ref":expression,"focus":{"scene_ref":a,"entity_ref":null,"relation_ref":null},"scene_order":[a,b]});
    let after = json!({"schema":"ql.native-atlas-state/v1","expression_ref":expression,"focus":{"scene_ref":b,"entity_ref":null,"relation_ref":"binding:actual"},"scene_order":[b,a]});
    let whole = OwnedAddress {
        expression_ref: expression.into(),
        scene_ref: None,
        entity_ref: None,
        component: "expression".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    let mut edit = FlowManualEdit {
        expression_ref: expression.into(),
        contribution_ref: "contribution:atlas".into(),
        owned_addresses: vec![whole.clone()],
        before: before.clone(),
        after: after.clone(),
        actor_ref: "human:owner".into(),
        operation_ref: "operation:native-relation-focus-reorder".into(),
        document_revision: 9,
        retained_native_records: vec![],
    };
    let actual = extract_flow_interventions(&edit).unwrap();
    assert_eq!(actual.native_records.len(), 2);
    assert!(
        actual
            .native_records
            .iter()
            .all(|r| r.address == whole && r.actor == "human:owner" && r.revision == 9)
    );
    assert_eq!(actual.native_records[0].path, "/focus");
    assert_eq!(actual.native_records[0].value, after["focus"]);
    assert_eq!(actual.native_records[1].kind, InterventionKind::Reorder);
    assert_eq!(actual.native_records[1].value, after["scene_order"]);
    edit.retained_native_records = actual.native_records.clone();
    assert_eq!(
        project_native_flow_interventions(&edit).unwrap(),
        actual.overlays
    );
    let mut forged = edit.clone();
    forged.after["focus"]["entity_ref"] = json!("expression:acceptance:entity:both");
    assert!(extract_flow_interventions(&forged).is_err());
    let mut forged = edit.clone();
    forged.owned_addresses = vec![OwnedAddress {
        expression_ref: expression.into(),
        scene_ref: Some(a.into()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    }];
    assert!(extract_flow_interventions(&forged).is_err());
    let mut forged = edit.clone();
    forged.retained_native_records[0].path = "/scene/procedural".into();
    assert!(project_native_flow_interventions(&forged).is_err());
    let mut forged = edit;
    forged.after["scene_order"] = json!([a, a]);
    assert!(extract_flow_interventions(&forged).is_err());
}

#[test]
fn owned_extractor_attributes_radius_while_actual_caption_stays_authored() {
    let mut input = edit();
    let id = input.before["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let entity = input.entity_refs[&id].clone();
    input.owned_addresses = vec![OwnedAddress {
        expression_ref: input.expression_ref.clone(),
        scene_ref: Some(input.scene_ref.clone()),
        entity_ref: Some(entity),
        component: "force".into(),
        constituent_ref: None,
        parent_ref: None,
        property: Some("radius".into()),
    }];
    input.after["scene"]["entities"][0]["force"]["radius"] = json!(0.3);
    input.after["scene"]["caption"] = json!("Actual authored caption edited in same native batch");
    assert!(extract_manual_interventions(&input).is_err());
    let actual = input.after.clone();
    let attribution = extract_owned_manual_interventions(&input).unwrap();
    assert_eq!(attribution.native_records.len(), 1);
    assert_eq!(
        attribution.native_records[0].address,
        input.owned_addresses[0]
    );
    assert_eq!(attribution.native_records[0].value, json!(0.3));
    assert!(
        attribution.native_records[0]
            .path
            .ends_with("/force/radius")
    );
    assert_eq!(input.after, actual);
    assert_eq!(
        input.after["scene"]["caption"],
        "Actual authored caption edited in same native batch"
    );
}
#[test]
fn native_intervention_batch_keeps_entry_order_same_cas_and_refuses_control_injection() {
    let mut first = edit();
    first.after["scene"]["entities"][0]["force"]["radius"] = json!(0.3);
    let mut second = edit();
    second.contribution_ref = "contribution:second".into();
    second.after["scene"]["caption"] = json!("authored");
    let entries = vec![
        InterventionPreflight::InterventionsOwned {
            edit: Box::new(first.clone()),
        },
        InterventionPreflight::InterventionsOwned {
            edit: Box::new(second.clone()),
        },
    ];
    let actual = intervention_batch(&entries).unwrap();
    assert_eq!(
        actual["results"][0]["interventions"][0]["contribution_ref"],
        first.contribution_ref
    );
    assert_eq!(
        actual["results"][1]["interventions"][0]["contribution_ref"],
        second.contribution_ref
    );
    second.operation_ref = "operation:foreign".into();
    assert!(
        intervention_batch(&[
            entries[0].clone(),
            InterventionPreflight::InterventionsOwned {
                edit: Box::new(second)
            }
        ])
        .is_err()
    );
    assert!(
        serde_json::from_value::<InterventionPreflight>(
            json!({"action":"material_readback","input":{}})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<InterventionPreflight>(
            json!({"action":"intervention_batch","entries":[]})
        )
        .is_err()
    );
    assert!(intervention_batch(&[]).is_err());
}
#[test]
fn actual_selection_optional_relation_wire_and_simultaneous_scene_remove_remain_coherent() {
    let expression = "expression:acceptance";
    let a = "expression:acceptance:scene:canonical";
    let b = "expression:acceptance:scene:passage";
    // Actual Selection serde omits relation_ref when None.
    let before = json!({"schema":"ql.native-atlas-state/v1","expression_ref":expression,"focus":{"scene_ref":a,"entity_ref":null},"scene_order":[a,b]});
    let after = json!({"schema":"ql.native-atlas-state/v1","expression_ref":expression,"focus":{"scene_ref":b,"entity_ref":null},"scene_order":[b]});
    let whole = OwnedAddress {
        expression_ref: expression.into(),
        scene_ref: None,
        entity_ref: None,
        component: "expression".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    let mut input = FlowManualEdit {
        expression_ref: expression.into(),
        contribution_ref: "contribution:flow".into(),
        owned_addresses: vec![whole],
        before,
        after: after.clone(),
        actor_ref: "human:owner".into(),
        operation_ref: "operation:actual-remove-fallback".into(),
        document_revision: 9,
        retained_native_records: vec![],
    };
    let result = extract_flow_interventions(&input).unwrap();
    assert_eq!(result.native_records.len(), 2);
    assert!(result.native_records[0].value.get("relation_ref").is_none());
    input.retained_native_records = result.native_records;
    assert_eq!(project_native_flow_interventions(&input).unwrap().len(), 2);
    input.after["focus"]["extra"] = json!("foreign");
    assert!(extract_flow_interventions(&input).is_err());
}

#[test]
fn shared_global_parameter_owned_preflight_preserves_all_locations_and_foreign_coordinate_refusal()
{
    let mut edit = edit();
    // Use exact first material member, not the map's lexical iteration order.
    let material_id = edit.before["scene"]["entities"][0]["id"].as_str().unwrap();
    let entity = edit.entity_refs[material_id].clone();
    let primary = ql_mef::procedural_composition::native_parameter_address(
        &edit.expression_ref,
        &edit.scene_ref,
        &entity,
        "force_radius",
    )
    .unwrap();
    let mut second = primary.clone();
    second.scene_ref = Some("expression:acceptance:scene:second".into());
    edit.owned_addresses = vec![primary.clone(), second.clone()];
    edit.after["scene"]["entities"][0]["force"]["radius"] = json!(0.3);
    edit.after["scene"]["caption"] = json!("also authored");
    let result = extract_owned_manual_interventions(&edit).unwrap();
    assert_eq!(result.native_records.len(), 1);
    assert_eq!(result.native_records[0].address, primary);
    assert_eq!(result.native_records[0].value, json!(0.3));
    assert_eq!(edit.after["scene"]["caption"], "also authored");
    assert!(extract_manual_interventions(&edit).is_err());
    let mut foreign = edit.clone();
    foreign.owned_addresses[1].entity_ref = Some("expression:acceptance:entity:foreign".into());
    assert!(extract_owned_manual_interventions(&foreign).is_err());
    let mut foreign = edit;
    foreign.owned_addresses[1].property = Some("strength".into());
    assert!(extract_owned_manual_interventions(&foreign).is_err());
}

#[test]
fn scene_deletion_keeps_prior_human_rows_and_release_restores_child_overlay() {
    let mut edit = edit();
    edit.after["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
    let child = extract_manual_interventions(&edit)
        .unwrap()
        .native_records
        .remove(0);
    edit.before = edit.after.clone();
    edit.after = Value::Null;
    edit.document_revision = 10;
    edit.retained_native_records = vec![child.clone()];
    let deleted = extract_manual_interventions(&edit).unwrap();
    assert!(deleted.native_records.contains(&child));
    assert_eq!(deleted.native_records.len(), 2);
    let mut released = deleted.native_records.clone();
    let tombstone = released.iter_mut().find(|r| r.path == "/scene").unwrap();
    tombstone.persistent = false;
    tombstone.revision = 11;
    tombstone.actor = "human:release".into();
    tombstone.operation_ref = Some("operation:release".into());
    edit.document_revision = 11;
    let overlays = project_native_authored_interventions(&edit, &released).unwrap();
    assert_eq!(overlays.len(), 1);
    assert_eq!(overlays[0].value, json!(0.875));
    assert_eq!(overlays[0].actor_ref, child.actor);
    assert_ne!(overlays[0].pointer, "/scene");
}
