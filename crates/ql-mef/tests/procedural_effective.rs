//! Tests the real native effective-operand compiler, never a receiver/clock.
use ql_mef::procedural_composition::*;
use ql_mef::procedural_effective::*;
use ql_mef::procedural_manifestation::*;
use ql_mef::procedural_retention::*;
use serde_json::json;
use std::collections::BTreeMap;

fn inputs() -> (GeneratedContribution, TargetReading, NativeMaterialization) {
    let expression = "expression:acceptance";
    let scene = "expression:acceptance:scene:canonical";
    let entity = "expression:acceptance:entity:force";
    let manifest = ql_mef::m_tree::native_current_m_registry().manifest();
    let subject = NativeSubject {
        subject_ref: "ql:m-coordinate:bimba:M3".into(),
        native_owner: "ql-mef".into(),
        presentation_role: SubjectRole::Thing,
        sources: vec![NativeReading {
            reference: format!(
                "{}:{}",
                manifest.source_repository, manifest.source_dataset_tree
            ),
            revision: manifest.source_snapshot_sha256.clone(),
            availability: ReadingAvailability::Available,
        }],
        readings: vec![],
        actions: vec![],
    };
    let recipe_revision = fingerprint(&include_str!(
        "../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md"
    ))
    .unwrap();
    let address = native_parameter_address(expression, scene, entity, "force_radius").unwrap();
    let subjects = vec![subject.subject_ref.clone()];
    let identity = contribution_identity("procedure:force", "radius", &subjects, entity).unwrap();
    let generated = GeneratedContribution {
        contribution_ref: identity.clone(),
        procedure_ref: "procedure:force".into(),
        procedure_revision: "1".into(),
        recipe: SourceBasis {
            source_ref:
                "docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3".into(),
            revision: recipe_revision.clone(),
        },
        output_slot: "radius".into(),
        subjects,
        occurrence_ref: entity.into(),
        owned_addresses: vec![address.clone()],
        native_changes: vec![NativeChange::ParameterSet {
            entity_ref: entity.into(),
            parameter: "force_radius".into(),
            value: json!(200.0),
        }],
        generated_basis: json!({"parameter":"force_radius","value":200.0}),
    };
    let reading = TargetReading {
        occurrence_ref: entity.into(),
        address: address.clone(),
        subject: subject.clone(),
        revision: 9,
        tags: vec![],
        properties: BTreeMap::from([("force_radius".into(), json!(640.0))]),
    };
    let retained = json!({"schema":RETENTION_SCHEMA,"bindings":[],"procedures":[],"contributions":[{"contribution_ref":identity,"procedure_ref":"procedure:force","status":"active","output_slot":"radius","subject_refs":[subject.subject_ref],"occurrence_ref":entity,"recipe_revision":recipe_revision,"owned_addresses":[address],"generated_basis":{"parameter":"force_radius","value":200.0},"authored_overrides":[{"address":address,"value":0.4,"actor":"human:owner","operation_ref":"oi.expression:accepted-manual-radius","revision":8,"persistent":true}]}],"controls":[],"operations":[],"scene_flow":[],"time_mappings":[],"source_basis":[]});
    let context = NativeMaterialization {
        schema: MATERIALIZATION_CONTRACT.into(),
        lifecycles: vec![],
        document_revision: 9,
        rule_cursor: 2,
        state: "held".into(),
        scenes: vec![NativeRetentionScene {
            scene_ref: scene.into(),
            document_revision: 9,
            existing_retention: Some(retained),
            current_presentation: None,
            principal: subject,
            contributors: vec![],
            locus: NativeReading {
                reference: "ql:m-coordinate:bimba:M3".into(),
                revision: manifest.registry_revision.clone(),
                availability: ReadingAvailability::Available,
            },
        }],
    };
    (generated, reading, context)
}

#[test]
fn native_radius_intervention_uses_current_parameter_units_and_keeps_recipe_basis() {
    let (original, mut reading, context) = inputs();
    reading.revision = 8;
    let mut c = vec![original.clone()];
    let observed = apply_retained_force_interventions(&mut c, &[reading], &context).unwrap();
    assert_eq!(c[0].generated_basis, original.generated_basis);
    assert_eq!(c[0].owned_addresses, original.owned_addresses);
    let NativeChange::ParameterSet { value, .. } = &c[0].native_changes[0] else {
        panic!("actual native operation")
    };
    assert_eq!(value, &json!(640.0));
    assert_ne!(value, &json!(0.4));
    assert_eq!(observed[0].target_revision, 8);
    assert_eq!(observed[0].document_revision, 9);
    assert_eq!(observed[0].actor_ref, "human:owner");
    assert_eq!(
        observed[0].operation_ref.as_deref(),
        Some("oi.expression:accepted-manual-radius")
    );
    assert_eq!(observed[0].intervention_revision, Some(8));
    let mut repeat = vec![original];
    apply_retained_force_interventions(&mut repeat, &[inputs().1], &context).unwrap();
    assert_eq!(repeat, c);
}

#[test]
fn invalid_revision_foreign_duplicate_and_missing_native_parameter_readings_refuse() {
    let (original, reading, context) = inputs();
    let mut stale = reading.clone();
    stale.revision = 10;
    assert!(
        apply_retained_force_interventions(&mut [original.clone()], &[stale], &context).is_err()
    );
    let mut foreign = reading.clone();
    foreign.address.scene_ref = Some("expression:acceptance:scene:foreign".into());
    assert!(
        apply_retained_force_interventions(&mut [original.clone()], &[foreign], &context).is_err()
    );
    assert!(
        apply_retained_force_interventions(
            &mut [original.clone()],
            &[reading.clone(), reading.clone()],
            &context
        )
        .is_err()
    );
    let mut absent = reading;
    absent.properties.clear();
    assert!(apply_retained_force_interventions(&mut [original], &[absent], &context).is_err());
}

#[test]
fn newest_native_intervention_wins_and_release_restores_recipe_without_fabricating_legacy_origin() {
    let (original, reading, mut context) = inputs();
    let old = context.scenes[0].existing_retention.as_mut().unwrap();
    let mut earlier = old["contributions"][0]["authored_overrides"][0].clone();
    earlier["actor"] = json!("human:earlier");
    earlier["operation_ref"] = json!("oi.expression:earlier");
    earlier["revision"] = json!(7);
    old["contributions"][0]["authored_overrides"]
        .as_array_mut()
        .unwrap()
        .push(earlier);
    let mut c = vec![original.clone()];
    let proof = apply_retained_force_interventions(&mut c, &[reading.clone()], &context).unwrap();
    assert_eq!(proof[0].actor_ref, "human:owner");
    let overlay = &mut context.scenes[0].existing_retention.as_mut().unwrap()["contributions"][0]["authored_overrides"]
        [0];
    overlay.as_object_mut().unwrap().remove("operation_ref");
    overlay.as_object_mut().unwrap().remove("revision");
    let legacy = overlay.clone();
    context.scenes[0].existing_retention.as_mut().unwrap()["contributions"][0]["authored_overrides"] =
        json!([legacy]);
    assert!(apply_retained_force_interventions(&mut c, &[reading.clone()], &context).is_err());
    // Legacy attribution without its actual revision remains unavailable rather
    // than gaining a fabricated accepted CAS origin.
    context.scenes[0].existing_retention.as_mut().unwrap()["contributions"][0]["authored_overrides"]
        [0]["revision"] = json!(8);
    let proof = apply_retained_force_interventions(&mut c, &[reading.clone()], &context).unwrap();
    assert!(proof[0].operation_ref.is_none());
    assert_eq!(proof[0].intervention_revision, Some(8));
    context.scenes[0].existing_retention.as_mut().unwrap()["contributions"][0]["authored_overrides"] =
        json!([]);
    let mut released = vec![original.clone()];
    assert!(
        apply_retained_force_interventions(&mut released, &[reading], &context)
            .unwrap()
            .is_empty()
    );
    assert_eq!(released, vec![original]);
}

#[test]
fn authored_radius_point_three_preserves_actual_native_parameter_one_twenty() {
    let (generated, mut reading, mut context) = inputs();
    reading.properties.insert("force_radius".into(), json!(120));
    context.scenes[0].existing_retention.as_mut().unwrap()["contributions"][0]["authored_overrides"]
        [0]["value"] = json!(0.3);
    let basis = generated.generated_basis.clone();
    let mut contributions = vec![generated];
    let effective =
        apply_retained_force_interventions(&mut contributions, &[reading], &context).unwrap();
    assert_eq!(effective[0].value, json!(120));
    assert_eq!(contributions[0].generated_basis, basis);
    assert!(
        matches!(&contributions[0].native_changes[0],NativeChange::ParameterSet{value,..} if value==&json!(120))
    );
}

#[test]
fn native_flow_regeneration_uses_current_selection_and_order_without_changing_recipe_basis() {
    let (mut contribution, mut reading, mut context) = inputs();
    let scene = reading.address.scene_ref.clone().unwrap();
    let passage = "expression:acceptance:scene:passage";
    let whole = OwnedAddress {
        expression_ref: "expression:acceptance".into(),
        scene_ref: None,
        entity_ref: None,
        component: "expression".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    contribution.occurrence_ref = whole.expression_ref.clone();
    contribution.owned_addresses = vec![whole.clone()];
    contribution.native_changes = vec![
        NativeChange::Focus {
            scene_ref: scene.clone(),
            entity_ref: None,
        },
        NativeChange::SceneReorder {
            scene_refs: vec![scene.clone(), passage.into()],
        },
    ];
    contribution.generated_basis = json!({"native_flow":contribution.native_changes});
    let generated = contribution.generated_basis.clone();
    let actual = json!({"schema":"ql.native-atlas-state/v1","expression_ref":whole.expression_ref,"focus":{"scene_ref":passage,"entity_ref":null,"relation_ref":"binding:actual-human"},"scene_order":[passage,scene]});
    reading.address = whole.clone();
    reading.occurrence_ref = whole.expression_ref.clone();
    reading.properties = BTreeMap::from([("native_atlas_state".into(), actual.clone())]);
    context.scenes[0].existing_retention.as_mut().unwrap()["contributions"] = json!([{"contribution_ref":contribution.contribution_ref,"procedure_ref":contribution.procedure_ref,"status":"active","authored_overrides":[
        {"address":whole,"actor":"human:owner","persistent":true,"revision":8,"path":"/focus","value":{"scene_ref":scene,"entity_ref":null},"kind":"set","operation_ref":"operation:prior","before":null},
        {"address":whole,"actor":"human:owner","persistent":true,"revision":8,"path":"/scene_order","value":[scene,passage],"kind":"reorder","operation_ref":"operation:prior","before":null}]}]);
    let mut contributions = vec![contribution];
    apply_retained_flow_interventions(&mut contributions, &[reading], &context).unwrap();
    assert!(
        matches!(&contributions[0].native_changes[0],NativeChange::RelationFocus{scene_ref,binding_ref} if scene_ref==passage && binding_ref=="binding:actual-human")
    );
    assert!(
        matches!(&contributions[0].native_changes[1],NativeChange::SceneReorder{scene_refs} if scene_refs==&vec![passage.to_owned(),scene])
    );
    assert_eq!(contributions[0].generated_basis, generated);
}

#[test]
fn shared_global_parameter_uses_one_native_value_for_all_owned_scene_locations() {
    let (mut c, first, mut context) = inputs();
    let mut second = first.clone();
    second.address.scene_ref = Some("expression:acceptance:scene:second".into());
    c.owned_addresses.push(second.address.clone());
    let mut scene = context.scenes[0].clone();
    scene.scene_ref = second.address.scene_ref.clone().unwrap();
    context.scenes.push(scene);
    let mut contributions = vec![c.clone()];
    let projection = apply_retained_force_interventions(
        &mut contributions,
        &[first.clone(), second.clone()],
        &context,
    )
    .unwrap();
    assert_eq!(projection.len(), 1);
    assert_eq!(projection[0].addresses, c.owned_addresses);
    assert_eq!(contributions[0].generated_basis, c.generated_basis);
    assert!(
        matches!(&contributions[0].native_changes[0],NativeChange::ParameterSet{value,..} if value==&json!(640.0))
    );
    second
        .properties
        .insert("force_radius".into(), json!(641.0));
    assert!(
        apply_retained_force_interventions(
            &mut vec![c.clone()],
            &[first.clone(), second],
            &context
        )
        .is_err()
    );
    assert!(apply_retained_force_interventions(&mut vec![c], &[first], &context).is_err());
}

#[test]
fn active_gesture_takeover_preserves_actual_native_value_until_source_release() {
    let (original, reading, mut context) = inputs();
    let retention = context.scenes[0].existing_retention.as_mut().unwrap();
    retention["contributions"][0]["authored_overrides"] = json!([]);
    retention["controls"] = json!([{"address":original.owned_addresses[0],"takeover":{"value":0.3,"native_value":120,"lifetime":"gesture","actor":"human:gesture","operation_ref":"ordinary:gesture","revision":9}}]);
    let mut generated = vec![original.clone()];
    let preserved =
        apply_retained_force_interventions(&mut generated, &[reading.clone()], &context).unwrap();
    assert_eq!(preserved[0].value, json!(640.0));
    assert_eq!(preserved[0].actor_ref, "human:gesture");
    assert_eq!(generated[0].generated_basis, original.generated_basis);
    let NativeChange::ParameterSet { value, .. } = &generated[0].native_changes[0] else {
        unreachable!()
    };
    assert_eq!(value, &json!(640.0));
    context.scenes[0].existing_retention.as_mut().unwrap()["controls"] = json!([]);
    let mut released = vec![original.clone()];
    assert!(
        apply_retained_force_interventions(&mut released, &[reading], &context)
            .unwrap()
            .is_empty()
    );
    assert_eq!(released, vec![original]);
}
