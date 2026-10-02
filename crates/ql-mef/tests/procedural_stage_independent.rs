//! Independent detecting cases over real production compiler functions.
//! Adopt as crates/ql-mef/tests/procedural_stage_independent.rs for hosted CI.
//! No stage ACK, body observation, rendering or sound is simulated here.
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::procedural_composition::*;
use ql_mef::procedural_manifestation::*;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

fn subject() -> NativeSubject {
    NativeSubject {
        subject_ref: "ql:m-coordinate:bimba:M3".into(),
        native_owner: "ql-mef".into(),
        presentation_role: SubjectRole::Thing,
        sources: vec![NativeReading {
            reference: format!(
                "{}:{}",
                native_current_m_registry().manifest().source_repository,
                native_current_m_registry().manifest().source_dataset_tree
            ),
            revision: native_current_m_registry()
                .manifest()
                .source_snapshot_sha256
                .clone(),
            availability: ReadingAvailability::Available,
        }],
        readings: vec![],
        actions: vec![],
    }
}
fn procedure() -> Procedure {
    Procedure {schema:PROCEDURE_CONTRACT.into(),procedure_ref:"procedure:independent".into(),revision:"1".into(),
        recipe:SourceBasis {source_ref:"recipe:independent".into(),revision:"1".into()},
        profile:SourceBasis {source_ref:"profile:independent".into(),revision:"1".into()},
        registry_revision:native_current_m_registry().manifest().registry_revision.clone(),
        principal_subject_ref:subject().subject_ref,locus_ref:"ql:m-coordinate:bimba:M3".into(),
        occurrence_ref:"expression:independent".into(),
        composition:serde_json::from_value(json!({"CPF":"dialogical","CT":"CT2","CP":"4.2","CF":"CF3","CFP":"CFP1","CS":"CS0",
            "direction":"forward","actor":"agent:independent","interpretation":{"ref":"ql:interpretation:c-prime","revision":"1"},
            "whole":"expression:independent","resolvePath":"aikit:resolve:independent","contextResolution":"aikit:context:independent",
            "sources":["source:accepted-procedural-stage"]})).unwrap(),
        trigger:Trigger {kind:TriggerKind::Explicit,mode:TriggerMode::Level},selector:Selector::All,
        conditions:vec![],recipe_parameters:BTreeMap::new(),membership_change_policy:MembershipChangePolicy::AdmitAndRecord,
        failure_policy:FailurePolicy::StopAffectedAndCheckpoint,continuation_policy:ProcedureContinuation::Continue,
        membership_mode:MembershipMode::Frozen,
        timing:TimingBinding {owner_ref:"native:expression".into(),domain:"simulation".into(),epoch_ref:"epoch:1".into(),requested_cursor:42,time_mapping_ref:None},
        budgets:ExecutionBudget {max_evaluations:8,max_operations:64,max_expansion_depth:8,max_active_instances:8,max_queue:8},
        seed:"independent-source-qualified".into(),seed_algorithm:SEED_ALGORITHM.into(),
        removal_policy:RemovalPolicy::RetireUneditedDetachEdited,
        admitted_changes:["scene_create","entity_add","subject_bind","scene_compose","scene_material_set","parameter_set"].into_iter().map(str::to_owned).collect()}
}
fn presentation() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap()
}
fn generated(p: &Procedure) -> GeneratedContribution {
    instantiate_scene(
        p,
        "expression:independent",
        "main",
        "expression:independent:scene:main",
        &[subject()],
        &presentation(),
    )
    .unwrap()
}

#[test]
fn a06_a07_numeric_overlay_cannot_transfer_human_override_after_reordering() {
    let p = procedure();
    let old = generated(&p);
    let mut next = old.clone();
    next.generated_basis["scene"]["entities"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let current = CurrentContribution {
        contribution_ref: old.contribution_ref.clone(),
        material: old.generated_basis.clone(),
        overlays: vec![AuthoredOverlay {
            contribution_ref: old.contribution_ref.clone(),
            pointer: "/scene/entities/0/force/strength".into(),
            value: json!(0.9),
            actor_ref: "human:owner".into(),
            persistent: true,
        }],
    };
    // Until overlays have a stable constituent-address form, an index pointer
    // must be rejected. Applying it to the new first entity silently moves it.
    assert!(
        regenerate(
            &[old],
            &[current],
            &[next],
            RemovalPolicy::RetireUneditedDetachEdited
        )
        .is_err(),
        "numeric constituent override was applied to a different stable entity"
    );
}

#[test]
fn a05_a08_frozen_membership_cannot_write_a_different_existing_entity() {
    let mut p = procedure();
    p.selector = Selector::Occurrences {
        refs: vec!["occurrence:a".into()],
    };
    let mut c = generated(&p);
    let b = "expression:independent:entity:existing-b".to_owned();
    c.owned_addresses = vec![OwnedAddress {
        expression_ref: "expression:independent".into(),
        scene_ref: Some(c.occurrence_ref.clone()),
        entity_ref: Some(b.clone()),
        component: "property".into(),
        constituent_ref: None,
        property: Some("scale".into()),
    }];
    c.native_changes = vec![NativeChange::ParameterSet {
        entity_ref: b,
        parameter: "scale".into(),
        value: json!(2.0),
    }];
    let membership = ResolvedMembership {
        selector: p.selector.clone(),
        mode: MembershipMode::Frozen,
        containing_expression_ref: "expression:independent".into(),
        targets: BTreeMap::from([("occurrence:a".into(), 1)]),
        addresses: BTreeMap::from([(
            "occurrence:a".into(),
            OwnedAddress {
                expression_ref: "expression:independent".into(),
                scene_ref: Some("expression:independent:scene:main".into()),
                entity_ref: Some("expression:independent:entity:existing-a".into()),
                component: "entity".into(),
                constituent_ref: None,
                property: None,
            },
        )]),
        joined: vec!["occurrence:a".into()],
        left: vec![],
    };
    assert!(
        compile_native_batch(
            native_current_m_registry(),
            &p,
            "operation:outside-selection",
            "expression:independent",
            1,
            membership,
            vec![c],
            BTreeSet::from(["scene".into()])
        )
        .is_err(),
        "declared ownership bypassed frozen membership"
    );
}

#[test]
fn a07_a08_scene_and_contained_property_writers_are_overlapping() {
    let p = procedure();
    let scene = generated(&p);
    let mut contained = scene.clone();
    contained.output_slot = "property".into();
    contained.contribution_ref = contribution_identity(
        &p.procedure_ref,
        &contained.output_slot,
        &contained.subjects,
        &contained.occurrence_ref,
    )
    .unwrap();
    let r = scene.generated_basis["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    contained.owned_addresses = vec![OwnedAddress {
        expression_ref: "expression:independent".into(),
        scene_ref: Some(scene.occurrence_ref.clone()),
        entity_ref: Some(r.clone()),
        component: "property".into(),
        constituent_ref: None,
        property: Some("scale".into()),
    }];
    contained.native_changes = vec![NativeChange::ParameterSet {
        entity_ref: r,
        parameter: "scale".into(),
        value: json!(2.0),
    }];
    let membership = ResolvedMembership {
        selector: p.selector.clone(),
        mode: MembershipMode::Frozen,
        containing_expression_ref: "expression:independent".into(),
        targets: BTreeMap::new(),
        addresses: BTreeMap::new(),
        joined: vec![],
        left: vec![],
    };
    assert!(
        compile_native_batch(
            native_current_m_registry(),
            &p,
            "operation:overlap",
            "expression:independent",
            1,
            membership,
            vec![scene, contained],
            BTreeSet::from(["scene".into()])
        )
        .is_err(),
        "whole-scene and contained-property writes conflict"
    );
}

#[test]
fn a07_reordered_generated_material_preserves_human_edit_on_the_same_id() {
    let p = procedure();
    let old = generated(&p);
    let mut actual = old.generated_basis.clone();
    let retained_id = actual["scene"]["entities"][0]["id"].clone();
    actual["scene"]["entities"][0]["force"]["strength"] = json!(0.9);
    let mut next = old.clone();
    next.generated_basis["scene"]["entities"]
        .as_array_mut()
        .unwrap()
        .reverse();
    for e in next.generated_basis["scene"]["entities"]
        .as_array_mut()
        .unwrap()
    {
        e["force"]["strength"] = json!(0.4);
    }
    let current = CurrentContribution {
        contribution_ref: old.contribution_ref.clone(),
        material: actual,
        overlays: vec![],
    };
    let delta = regenerate(
        &[old.clone()],
        &[current],
        &[next],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    let values = delta.effective_basis[&old.contribution_ref]["scene"]["entities"]
        .as_array()
        .unwrap();
    let retained = values.iter().find(|e| e["id"] == retained_id).unwrap();
    assert_eq!(retained["force"]["strength"], 0.9);
    assert_eq!(
        values.iter().find(|e| e["id"] != retained_id).unwrap()["force"]["strength"],
        0.4
    );
}

#[test]
fn a10_same_owner_interval_cannot_reset_budget_after_an_intervening_interval() {
    let p = procedure();
    let mut execution = RuleExecution::new(&p, "interval:a").unwrap();
    execution.begin_interval("interval:b", 128).unwrap();
    assert!(
        execution.begin_interval("interval:a", 128).is_err(),
        "revisiting the same owner interval reset its execution budget"
    );
}

#[test]
fn a02_a06_a07_generated_native_scene_preserves_exact_constituent_bound_drivers_and_tracks() {
    let p = procedure();
    let mut source = presentation();
    source["scene"]["automation"] = json!([{"id":"driver:source","entityId":"source-form",
        "target":"entity:source-form:forces.strength","enabled":true,"type":"lfo","wave":"sine",
        "min":0.1,"max":0.7,"rate":0.25,"phase":0.0,"blend":"replace","duration":5.0,
        "delay":0.0,"loop":"loop","firedAt":0}]);
    source["scene"]["propertyTracks"] = json!([{"id":"track:source","entityId":"source-form",
        "bind":"entity.force.strength","points":[{"time":0.0,"value":0.2},{"time":10.0,"value":0.4}]}]);
    // Captions/source provenance are content, and must not be string-rewritten.
    source["scene"]["character"] = json!("source-form is the retained authored source slot");
    let generated = instantiate_scene(
        &p,
        "expression:independent",
        "main",
        "expression:independent:scene:main",
        &[subject()],
        &source,
    )
    .unwrap();
    let material = &generated.generated_basis["scene"];
    let native_ref = material["entities"][0]["id"].as_str().unwrap();
    assert_ne!(native_ref, "source-form");
    assert_eq!(
        material["automation"][0]["entityId"], native_ref,
        "generated native driver still addresses the template's old occurrence slot"
    );
    assert_eq!(
        material["automation"][0]["target"],
        format!("entity:{}:forces.strength", native_ref.replace(':', "%3A")),
        "actual native registry target is disconnected after generation"
    );
    assert_eq!(material["propertyTracks"][0]["entityId"], native_ref);
    assert_eq!(material["character"], source["scene"]["character"]);
}
