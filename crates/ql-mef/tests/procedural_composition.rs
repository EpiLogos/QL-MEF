//! Procedure compiler tests execute real production functions and native wire.
//! Installed stage/body/audio application is verified by the paired consumers.
use ql_mef::{
    MFace, aw1_world, coordinate_expression, m_tree, m3_state, vak_profile, vak_workflow_types,
};
use std::collections::{BTreeMap, BTreeSet};
#[path = "../src/procedural_composition.rs"]
pub mod procedural_composition;
#[path = "../src/procedural_manifestation.rs"]
pub mod procedural_manifestation;
use procedural_composition::*;
use procedural_manifestation::*;
use serde_json::{Value, json};

fn subject() -> NativeSubject {
    let manifest = m_tree::native_current_m_registry().manifest();
    NativeSubject {
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
    }
}

fn procedure() -> Procedure {
    let registry = m_tree::native_current_m_registry();
    Procedure { schema: PROCEDURE_CONTRACT.into(), procedure_ref: "procedure:source-native-passage".into(), revision: "1".into(),
        recipe: SourceBasis { source_ref: "docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3".into(),
            revision:fingerprint(&include_str!("../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md")).unwrap() },
        profile: SourceBasis { source_ref: "profile:source-native-passage".into(), revision: "1".into() },
        registry_revision: registry.manifest().registry_revision.clone(), principal_subject_ref: subject().subject_ref,
        locus_ref: "ql:m-coordinate:bimba:M3".into(), occurrence_ref: "expression:acceptance".into(),
        composition: serde_json::from_value(json!({"CPF":"dialogical","CT":"CT2","CP":"4.2","CF":"CF3","CFP":"CFP1","CS":"CS0",
            "direction":"forward","actor":"agent:anima","interpretation":{"ref":"ql:interpretation:c-prime","revision":"1"},
            "whole":"expression:acceptance","resolvePath":"aikit:resolve:procedural-stage","contextResolution":"aikit:context:procedural-stage",
            "sources":["source:accepted-procedural-stage"]})).unwrap(),
        trigger: Trigger { kind: TriggerKind::Explicit, mode: TriggerMode::Edge }, selector: Selector::All,
        conditions: vec![], recipe_parameters: BTreeMap::from([("source_treatment".into(),json!("glyph_mask"))]),
        membership_mode: MembershipMode::Frozen, membership_change_policy: MembershipChangePolicy::AdmitAndRecord,
        timing: TimingBinding { owner_ref: "native:expression".into(), domain: "simulation".into(), epoch_ref: "epoch:native-1".into(),
            requested_cursor: 42, time_mapping_ref: None }, budgets: ExecutionBudget { max_evaluations: 32, max_operations: 256,
            max_expansion_depth: 8, max_active_instances: 16, max_queue: 16 }, seed: "source-passage-20261002".into(),
        seed_algorithm: SEED_ALGORITHM.into(), removal_policy: RemovalPolicy::RetireUneditedDetachEdited,
        failure_policy: FailurePolicy::StopAffectedAndCheckpoint, continuation_policy: ProcedureContinuation::Continue,
        admitted_changes: ["scene_create","scene_remove","entity_add","entity_remove","subject_bind","scene_compose","scene_material_set","parameter_set","focus"].into_iter().map(str::to_owned).collect() }
}

/// A complete ordinary scene in the existing authoring vocabulary. The same
/// fixture is emitted to the actual O:I Application consumer, not a reducer.
fn presentation() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap()
}

fn contribution(procedure: &Procedure, slot: &str) -> GeneratedContribution {
    instantiate_scene(
        procedure,
        "expression:acceptance",
        slot,
        &format!("expression:acceptance:scene:{slot}"),
        &[subject()],
        &presentation(),
    )
    .unwrap()
}

fn membership() -> ResolvedMembership {
    ResolvedMembership {
        selector: Selector::All,
        mode: MembershipMode::Frozen,
        containing_expression_ref: "expression:acceptance".into(),
        targets: BTreeMap::new(),
        addresses: BTreeMap::new(),
        joined: vec![],
        left: vec![],
    }
}

#[test]
fn first_connected_batch_creates_real_scenes_constituents_force_and_sequence() {
    let procedure = procedure();
    let contributions = (0..3)
        .map(|i| contribution(&procedure, &format!("passage-{i}")))
        .collect::<Vec<_>>();
    let prepared = compile_native_batch(
        m_tree::native_current_m_registry(),
        &procedure,
        "operation:first-passage",
        "expression:acceptance",
        1,
        membership(),
        contributions.clone(),
        BTreeSet::from(["scene".into(), "nativeBody".into(), "audio".into()]),
    )
    .unwrap();
    assert_eq!(prepared.native_edit["operation"], "edit");
    assert_eq!(prepared.native_edit["expected_revision"], 1);
    let changes = prepared.native_edit["changes"].as_array().unwrap();
    assert_eq!(
        changes
            .iter()
            .filter(|c| c["change"] == "scene_create")
            .count(),
        3
    );
    assert_eq!(
        changes
            .iter()
            .filter(|c| c["change"] == "entity_add")
            .count(),
        6
    );
    assert_eq!(
        changes
            .iter()
            .filter(|c| c["change"] == "subject_bind")
            .count(),
        6
    );
    assert_eq!(
        contributions[0].generated_basis["scene"]["entities"][1]["kind"],
        "pin"
    );
    assert_eq!(
        contributions[0].generated_basis["scene"]["entities"][0]["sequence"]["steps"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        contributions[0].generated_basis["scene"]["entities"][0]["source"],
        presentation()["scene"]["entities"][0]["source"]
    );
    // sourcesVersion=1 admits each state's own sampler source explicitly;
    // a missing state source deliberately does not inherit the entity source.
    assert_eq!(
        contributions[0].generated_basis["scene"]["entities"][0]["sequence"]["steps"][0]["source"],
        presentation()["scene"]["entities"][0]["source"]
    );
    assert_eq!(
        contributions[0].generated_basis["scene"]["entities"][0]["layers"],
        presentation()["scene"]["entities"][0]["layers"]
    );
    assert_eq!(
        contributions[0].generated_basis["scene"]["entities"][0]["force"]["radius"],
        0.3
    );
    for entity in contributions[0].generated_basis["scene"]["entities"]
        .as_array()
        .unwrap()
    {
        assert!(
            entity["id"]
                .as_str()
                .unwrap()
                .starts_with("expression:acceptance:entity:p-")
        );
    }
    if let Ok(path) = std::env::var("TA_ONTA_FIXTURE_OUTPUT") {
        let old = contributions[0].clone();
        let entity = old.generated_basis["scene"]["entities"][0]["id"]
            .as_str()
            .unwrap();
        let mut actual = old.generated_basis.clone();
        actual["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
        let mut next = old.clone();
        next.procedure_revision = "2".into();
        next.generated_basis["scene"]["entities"][0]["force"]["strength"] = json!(0.5);
        next.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["hold"] = json!(3.5);
        next.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"] =
            json!(true);
        let current = CurrentContribution {
            contribution_ref: old.contribution_ref.clone(),
            material: actual,
            overlays: vec![],
        };
        let regeneration = regenerate(
            &[old.clone()],
            &[current.clone()],
            &[next],
            RemovalPolicy::RetireUneditedDetachEdited,
        )
        .unwrap();
        let native_regeneration =
            regeneration_native_changes(&regeneration, &[old.clone()], &[current]).unwrap();
        std::fs::write(path, serde_json::to_vec_pretty(&json!({"schema":"ql.procedural-stage-fixture/v1", "procedure":procedure,
            "prepared":prepared,"contributions":contributions,"create":{"operation":"create","expression_ref":"expression:acceptance",
            "title":"Ta-Onta Procedural Stage Acceptance","actor":"agent:anima"},
            "human_override":{"operation":"edit","expression_ref":"expression:acceptance","expected_revision":2,"actor":"human:owner",
                "changes":[{"change":"parameter_set","entity_ref":entity,"parameter":"force_strength","value":0.875}]},
            "regeneration":regeneration,"native_regeneration":{"operation":"edit","expression_ref":"expression:acceptance","expected_revision":3,
                "actor":"agent:anima","changes":native_regeneration}})).unwrap()).unwrap();
    }
}

#[test]
fn complete_occurrence_adapter_preserves_native_references_and_opaque_source_content() {
    let p = procedure();
    let mut source = presentation();
    source["scene"]["automation"] = json!([
        {"id":"driver:a","entityId":"source-form","target":"entity:source-form:forces.strength","clockId":"pin:source-form"},
        {"id":"driver:b","entityId":"source-form","target":"link:source-form:source-2:scale"}
    ]);
    source["scene"]["propertyTracks"] = json!([{"id":"track:a","entityId":"source-form","bind":"entity.force.strength","points":[]}]);
    source["scene"]["toolbelt"] = json!([{"id":"tool:a","entityId":"source-form","sceneId":"expression:source:scene:material","journeyId":"source:authored-journey"}]);
    source["scene"]["semanticField"] = json!({"bindings":[{"id":"semantic:a","semanticNodeId":"canonical:source-form","carriers":[
        {"kind":"entity","id":"source-form"},{"kind":"forceEmitter","id":"entity:source-force"},
        {"kind":"forceEmitter","id":"relational:source-form"}]}]});
    source["scene"]["composition"]["blueprint"] = json!({"members":[{"entity_ref":"source-form","subject_ref":"subject:source-form","role_ref":"role:source-form","position":1}],"basis_refs":["source-form"]});
    source["scene"]["entities"][0]["native"] =
        json!({"id":"source-form","shape":{"text":"source-form"}});
    let config = json!({"entities":[{"id":"source-form"},{"id":"source-force"}],"automations":[{"path":"entities.0.forces.strength"}],
        "semanticField":{"bindings":[{"carriers":[{"kind":"entity","id":"source-form"}]}]}});
    source["scene"]["native"] = json!({"config":config,"projection":config,"original":{"id":"source-form","source":"source-form"}});
    source["scene"]["research"] = json!({"cards":{"source-form":{"caption":"source-form is authored text"}},"timeline":{"source-form":{"offsetY":2}},
        "frames":{"frame:a":{"memberRefs":["source-form","source-force"]}},"namedViews":{"view:a":{"selectedRefs":["source-form"],"frameOrder":["frame:a"]}}});
    let generated = instantiate_scene(
        &p,
        "expression:acceptance",
        "main",
        "expression:acceptance:scene:main",
        &[subject()],
        &source,
    )
    .unwrap();
    let material = &generated.generated_basis["scene"];
    let form = material["entities"][0]["id"].as_str().unwrap();
    let force = material["entities"][1]["id"].as_str().unwrap();
    assert_eq!(material["automation"][0]["entityId"], form);
    assert_eq!(
        material["automation"][0]["target"],
        format!("entity:{}:forces.strength", form.replace(':', "%3A"))
    );
    assert_eq!(
        material["automation"][1]["target"],
        format!("link:{}:source-2:scale", form.replace(':', "%3A"))
    );
    assert_eq!(material["automation"][0]["clockId"], format!("pin:{form}"));
    assert_eq!(material["propertyTracks"][0]["entityId"], form);
    assert_eq!(material["toolbelt"][0]["entityId"], form);
    assert_eq!(
        material["toolbelt"][0]["sceneId"],
        "expression:acceptance:scene:main"
    );
    assert_eq!(
        material["toolbelt"][0]["journeyId"],
        "source:authored-journey"
    );
    assert_eq!(material["entities"][0]["native"]["id"], form);
    assert_eq!(
        material["entities"][0]["native"]["shape"]["text"],
        "source-form"
    );
    assert_eq!(
        material["semanticField"]["bindings"][0]["carriers"][0]["id"],
        form
    );
    assert_eq!(
        material["semanticField"]["bindings"][0]["carriers"][1]["id"],
        format!("entity:{force}")
    );
    assert_eq!(
        material["semanticField"]["bindings"][0]["carriers"][2]["id"],
        "relational:source-form"
    );
    assert_eq!(
        material["composition"]["blueprint"]["members"][0]["entity_ref"],
        form
    );
    assert_eq!(
        material["composition"]["blueprint"]["members"][0]["subject_ref"],
        "subject:source-form"
    );
    assert_eq!(
        material["composition"]["blueprint"]["basis_refs"],
        json!(["source-form"])
    );
    for key in ["config", "projection"] {
        assert_eq!(material["native"][key]["entities"][0]["id"], form);
        assert_eq!(
            material["native"][key]["semanticField"]["bindings"][0]["carriers"][0]["id"],
            form
        );
        assert_eq!(
            material["native"][key]["automations"][0]["path"],
            "entities.0.forces.strength"
        );
    }
    assert_eq!(
        material["native"]["original"],
        source["scene"]["native"]["original"]
    );
    assert_eq!(
        material["research"]["cards"][form]["caption"],
        "source-form is authored text"
    );
    assert_eq!(material["research"]["timeline"][form]["offsetY"], 2);
    assert_eq!(
        material["research"]["frames"]["frame:a"]["memberRefs"],
        json!([form, force])
    );
    assert_eq!(
        material["research"]["namedViews"]["view:a"]["selectedRefs"],
        json!([form])
    );

    let mut missing = source.clone();
    missing["scene"]["automation"][0]["target"] = json!("entity:missing:forces.strength");
    assert!(
        instantiate_scene(
            &p,
            "expression:acceptance",
            "main",
            "expression:acceptance:scene:main",
            &[subject()],
            &missing
        )
        .is_err()
    );
    source["scene"]["procedural"] = json!({"procedures":[{"state":"running"}]});
    assert!(
        instantiate_scene(
            &p,
            "expression:acceptance",
            "main",
            "expression:acceptance:scene:main",
            &[subject()],
            &source
        )
        .unwrap_err()
        .contains("clone/rebind")
    );
}

#[test]
fn rule_revision_changes_payload_but_preserves_generated_native_identities() {
    let a = procedure();
    let mut b = a.clone();
    b.revision = "2".into();
    b.recipe.revision = "2".into();
    let a = contribution(&a, "scene");
    let b = contribution(&b, "scene");
    assert_eq!(a.contribution_ref, b.contribution_ref);
    assert_eq!(
        a.generated_basis["scene"]["entities"][0]["id"],
        b.generated_basis["scene"]["entities"][0]["id"]
    );
    assert_ne!(a.procedure_revision, b.procedure_revision);
    assert_ne!(
        a.contribution_ref,
        contribution_identity(
            &a.procedure_ref,
            &a.output_slot,
            &a.subjects,
            "expression:acceptance:scene:clone"
        )
        .unwrap()
    );
}

#[test]
fn three_way_diff_preserves_human_force_override_and_accepts_new_generated_sequence() {
    let procedure = procedure();
    let old = contribution(&procedure, "scene");
    let mut actual = old.generated_basis.clone();
    actual["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
    let mut next = old.clone();
    next.procedure_revision = "2".into();
    next.generated_basis["scene"]["entities"][0]["force"]["strength"] = json!(0.5);
    next.generated_basis["scene"]["entities"][0]["sequence"]["hold"] = json!(3.5);
    let current = CurrentContribution {
        contribution_ref: old.contribution_ref.clone(),
        material: actual,
        overlays: vec![],
    };
    let delta = regenerate(
        &[old.clone()],
        &[current.clone()],
        &[next],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    let effective = &delta.effective_basis[&old.contribution_ref];
    assert_eq!(
        effective["scene"]["entities"][0]["force"]["strength"],
        0.875
    );
    assert_eq!(effective["scene"]["entities"][0]["sequence"]["hold"], 3.5);
    let changes = regeneration_native_changes(&delta, &[old], &[current]).unwrap();
    assert!(!changes.iter().any(|c| matches!(
        c,
        NativeChange::SceneCreate { .. } | NativeChange::EntityAdd { .. }
    )));
    assert!(
        changes
            .iter()
            .any(|c| matches!(c, NativeChange::SceneMaterialSet { .. }))
    );
}

#[test]
fn explicit_persistent_overlay_survives_reordering_regeneration_and_serialization() {
    let old = contribution(&procedure(), "scene");
    let mut next = old.clone();
    next.generated_basis["scene"]["field"]["params"]["speed"] = json!(0.1);
    next.generated_basis["scene"]["entities"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let overlay = AuthoredOverlay {
        contribution_ref: old.contribution_ref.clone(),
        pointer: "/scene/field/params/speed".into(),
        value: json!(0.8),
        actor_ref: "human:owner".into(),
        persistent: true,
    };
    let current = CurrentContribution {
        contribution_ref: old.contribution_ref.clone(),
        material: old.generated_basis.clone(),
        overlays: vec![overlay.clone()],
    };
    let delta = regenerate(
        &[old.clone()],
        &[current],
        &[next],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    assert_eq!(
        delta.effective_basis[&old.contribution_ref]["scene"]["field"]["params"]["speed"],
        0.8
    );
    assert_eq!(delta.retained_overlays, vec![overlay]);
    let reopened: Regeneration =
        serde_json::from_value(serde_json::to_value(&delta).unwrap()).unwrap();
    assert_eq!(delta, reopened);
}

#[test]
fn stable_constituent_override_cannot_move_to_another_id_or_silently_disappear() {
    let old = contribution(&procedure(), "stable-overlay");
    let mut next = old.clone();
    let id = old.generated_basis["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    next.generated_basis["scene"]["entities"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let overlay = AuthoredOverlay {
        contribution_ref: old.contribution_ref.clone(),
        pointer: format!("/scene/entities/@{id}/force/strength"),
        value: json!(0.875),
        actor_ref: "human:owner".into(),
        persistent: true,
    };
    let current = CurrentContribution {
        contribution_ref: old.contribution_ref.clone(),
        material: old.generated_basis.clone(),
        overlays: vec![overlay.clone()],
    };
    let delta = regenerate(
        &[old.clone()],
        &[current.clone()],
        &[next.clone()],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    let entities = delta.effective_basis[&old.contribution_ref]["scene"]["entities"]
        .as_array()
        .unwrap();
    assert_eq!(
        entities.iter().find(|entity| entity["id"] == id).unwrap()["force"]["strength"],
        0.875
    );
    assert_eq!(
        entities.iter().find(|entity| entity["id"] != id).unwrap()["force"]["strength"],
        0.125
    );
    let mut positional = current.clone();
    positional.overlays[0].pointer = "/scene/entities/0/force/strength".into();
    assert!(
        regenerate(
            &[old.clone()],
            &[positional],
            &[next.clone()],
            RemovalPolicy::RetireUneditedDetachEdited
        )
        .unwrap_err()
        .contains("positional")
    );
    next.generated_basis["scene"]["entities"]
        .as_array_mut()
        .unwrap()
        .retain(|entity| entity["id"] != id);
    assert!(
        regenerate(
            &[old],
            &[current],
            &[next],
            RemovalPolicy::RetireUneditedDetachEdited
        )
        .unwrap_err()
        .contains("disappeared")
    );
}

#[test]
fn removal_detaches_edited_contribution_and_retires_only_unedited_native_scene() {
    let a = contribution(&procedure(), "a");
    let b = contribution(&procedure(), "b");
    let mut edited = a.generated_basis.clone();
    edited["scene"]["entities"][0]["name"] = json!("Human interpretation");
    let actual = vec![
        CurrentContribution {
            contribution_ref: a.contribution_ref.clone(),
            material: edited,
            overlays: vec![],
        },
        CurrentContribution {
            contribution_ref: b.contribution_ref.clone(),
            material: b.generated_basis.clone(),
            overlays: vec![],
        },
    ];
    let delta = regenerate(
        &[a.clone(), b.clone()],
        &actual,
        &[],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    assert_eq!(delta.detached, vec![a.contribution_ref.clone()]);
    assert_eq!(delta.retired, vec![b.contribution_ref.clone()]);
    assert_eq!(
        regeneration_native_changes(&delta, &[a.clone(), b.clone()], &actual).unwrap(),
        vec![NativeChange::SceneRemove {
            scene_ref: b.occurrence_ref.clone()
        }]
    );
    assert!(regenerate(&[a, b], &actual, &[], RemovalPolicy::ConflictOnEdited).is_err());
}

#[test]
fn stable_id_merge_preserves_edited_retired_layer_and_unrelated_authored_material() {
    let old = json!({"entities":[{"id":"a","scale":1},{"id":"b","scale":1}],"sequence":{"hold":1}});
    let current = json!({"entities":[{"id":"b","scale":9},{"id":"a","scale":1},{"id":"human","scale":3}],"sequence":{"hold":1}});
    let generated = json!({"entities":[{"id":"a","scale":2}],"sequence":{"hold":2}});
    let effective = reconcile_material(&old, &current, &generated).unwrap();
    assert_eq!(
        effective["entities"],
        json!([{"id":"a","scale":2},{"id":"b","scale":9},{"id":"human","scale":3}])
    );
    assert_eq!(effective["sequence"]["hold"], 2);
    assert!(reconcile_material(&json!({}), &json!({"created":1}), &json!({"created":2})).is_err());
}

fn reading(reference: &str, revision: u64) -> TargetReading {
    TargetReading {
        occurrence_ref: reference.into(),
        address: OwnedAddress {
            expression_ref: "expression:acceptance".into(),
            scene_ref: Some("expression:acceptance:scene:main".into()),
            entity_ref: Some(format!("expression:acceptance:entity:{reference}")),
            component: "entity".into(),
            constituent_ref: None,
            property: None,
        },
        subject: subject(),
        revision,
        tags: vec![QualifiedTag {
            value: "fold".into(),
            origin: TagOrigin::Authored,
            scope_ref: "expression:acceptance".into(),
            basis: SourceBasis {
                source_ref: "annotation:owner".into(),
                revision: "1".into(),
            },
        }],
        properties: BTreeMap::from([("scale".into(), json!(1.0))]),
    }
}

#[test]
fn frozen_membership_does_not_absorb_later_tags_and_sustained_changes_are_recorded() {
    let selector = Selector::Tag {
        value: "fold".into(),
        origin: TagOrigin::Authored,
        scope_ref: "expression:acceptance".into(),
    };
    let a = reading("a", 1);
    let b = reading("b", 1);
    let frozen = resolve_membership(
        &selector,
        "expression:acceptance",
        &[a.clone()],
        MembershipMode::Frozen,
        None,
        MembershipChangePolicy::AdmitAndRecord,
    )
    .unwrap();
    let again = resolve_membership(
        &selector,
        "expression:acceptance",
        &[a.clone(), b.clone()],
        MembershipMode::Frozen,
        Some(&frozen),
        MembershipChangePolicy::AdmitAndRecord,
    )
    .unwrap();
    assert_eq!(again.targets, frozen.targets);
    assert!(
        resolve_membership(
            &Selector::All,
            "expression:acceptance",
            &[a.clone()],
            MembershipMode::Frozen,
            Some(&frozen),
            MembershipChangePolicy::AdmitAndRecord
        )
        .unwrap_err()
        .contains("selector definition")
    );
    let sustained = resolve_membership(
        &selector,
        "expression:acceptance",
        &[a.clone()],
        MembershipMode::Sustained,
        None,
        MembershipChangePolicy::AdmitAndRecord,
    )
    .unwrap();
    let updated = resolve_membership(
        &selector,
        "expression:acceptance",
        &[b.clone()],
        MembershipMode::Sustained,
        Some(&sustained),
        MembershipChangePolicy::AdmitAndRecord,
    )
    .unwrap();
    assert_eq!(updated.joined, vec!["b"]);
    assert_eq!(updated.left, vec!["a"]);
    assert!(
        resolve_membership(
            &selector,
            "expression:acceptance",
            &[a, b],
            MembershipMode::Sustained,
            Some(&sustained),
            MembershipChangePolicy::RejectChange
        )
        .is_err()
    );
}

#[test]
fn explicit_unknown_target_differs_from_intentionally_empty_selector() {
    assert!(
        resolve_membership(
            &Selector::Occurrences {
                refs: vec!["missing".into()]
            },
            "expression:acceptance",
            &[],
            MembershipMode::Frozen,
            None,
            MembershipChangePolicy::AdmitAndRecord
        )
        .is_err()
    );
    assert!(
        resolve_membership(
            &Selector::All,
            "expression:acceptance",
            &[],
            MembershipMode::Frozen,
            None,
            MembershipChangePolicy::AdmitAndRecord
        )
        .unwrap()
        .targets
        .is_empty()
    );
}

#[test]
fn typed_conditions_never_infer_a_native_relation_from_a_tag() {
    let registry = m_tree::native_current_m_registry();
    let target = reading("a", 1);
    assert!(
        Condition::ScalarRange {
            property: "scale".into(),
            minimum: 0.5,
            maximum: 2.0
        }
        .accepts(registry, &target)
        .unwrap()
    );
    assert!(
        !Condition::Equals {
            property: "scale".into(),
            value: json!("1")
        }
        .accepts(registry, &target)
        .unwrap()
    );
    assert!(
        Condition::ScalarRange {
            property: "scale".into(),
            minimum: 2.0,
            maximum: 1.0
        }
        .accepts(registry, &target)
        .is_err()
    );
}

#[test]
fn source_qualified_conditions_are_resolved_before_frozen_native_scope() {
    let registry = m_tree::native_current_m_registry();
    let mut p = procedure();
    p.conditions = vec![Condition::ScalarRange {
        property: "scale".into(),
        minimum: 0.5,
        maximum: 2.0,
    }];
    let a = reading("a", 7);
    let mut b = reading("b", 8);
    b.properties.insert("scale".into(), json!(3.0));
    let selected =
        resolve_procedure_membership(registry, &p, "expression:acceptance", &[b, a.clone()], None)
            .unwrap();
    assert_eq!(selected.targets, BTreeMap::from([("a".into(), 7)]));
    assert_eq!(selected.addresses["a"], a.address);
    let native = registry
        .manifest()
        .relations
        .iter()
        .find(|relation| relation.from_ref.is_some() && relation.to_ref.is_some())
        .unwrap();
    let relation = RequiredRelation {
        relation_id: native.id,
        source_kind: native.source_kind.clone(),
        from_ref: native.from_ref.clone().unwrap(),
        to_ref: native.to_ref.clone().unwrap(),
        registry_revision: registry.manifest().registry_revision.clone(),
    };
    let mut related = a;
    related.subject.subject_ref = relation.from_ref.clone();
    p.conditions = vec![Condition::NativeRelation {
        relation: relation.clone(),
    }];
    assert_eq!(
        resolve_procedure_membership(
            registry,
            &p,
            "expression:acceptance",
            &[related.clone()],
            None
        )
        .unwrap()
        .targets
        .len(),
        1
    );
    let mut stale = relation;
    stale.registry_revision = "stale".into();
    p.conditions = vec![Condition::NativeRelation { relation: stale }];
    assert!(
        resolve_procedure_membership(registry, &p, "expression:acceptance", &[related], None)
            .is_err()
    );
}

#[test]
fn existing_force_write_must_match_actual_resolved_entity_and_writer_scope() {
    let registry = m_tree::native_current_m_registry();
    let p = procedure();
    let a = reading("a", 9);
    let selected =
        resolve_procedure_membership(registry, &p, "expression:acceptance", &[a.clone()], None)
            .unwrap();
    let mut c = contribution(&p, "existing-force");
    let mut owned = a.address.clone();
    owned.component = "property".into();
    owned.property = Some("force_strength".into());
    c.owned_addresses = vec![owned.clone()];
    c.native_changes = vec![NativeChange::ParameterSet {
        entity_ref: a.address.entity_ref.clone().unwrap(),
        parameter: "force_strength".into(),
        value: json!(0.875),
    }];
    let compile = |membership, c| {
        compile_native_batch(
            registry,
            &p,
            "operation:force",
            "expression:acceptance",
            9,
            membership,
            vec![c],
            BTreeSet::from(["scene".into()]),
        )
    };
    assert_eq!(
        compile(selected.clone(), c.clone()).unwrap().native_edit["changes"][0]["parameter"],
        "force_strength"
    );
    let mut wrong = c.clone();
    wrong.owned_addresses[0].entity_ref = Some("expression:acceptance:entity:b".into());
    wrong.native_changes = vec![NativeChange::ParameterSet {
        entity_ref: "expression:acceptance:entity:b".into(),
        parameter: "force_strength".into(),
        value: json!(1),
    }];
    assert!(
        compile(selected.clone(), wrong)
            .unwrap_err()
            .contains("membership")
    );
    let mut lost = selected;
    lost.addresses.clear();
    assert!(
        compile(lost, c)
            .unwrap_err()
            .contains("exact native address")
    );
    let broad = OwnedAddress {
        expression_ref: owned.expression_ref.clone(),
        scene_ref: owned.scene_ref.clone(),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        property: None,
    };
    assert!(broad.covers(&owned));
    assert!(owned.covers(&owned));
}

#[test]
fn whole_scene_and_contained_property_writers_require_explicit_native_composition() {
    let p = procedure();
    let scene = contribution(&p, "scene");
    let mut property = scene.clone();
    property.output_slot = "property".into();
    property.contribution_ref = contribution_identity(
        &p.procedure_ref,
        &property.output_slot,
        &property.subjects,
        &property.occurrence_ref,
    )
    .unwrap();
    let entity = scene.generated_basis["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    property.owned_addresses = vec![OwnedAddress {
        expression_ref: "expression:acceptance".into(),
        scene_ref: Some(scene.occurrence_ref.clone()),
        entity_ref: Some(entity.clone()),
        component: "property".into(),
        constituent_ref: None,
        property: Some("force_strength".into()),
    }];
    property.native_changes = vec![NativeChange::ParameterSet {
        entity_ref: entity,
        parameter: "force_strength".into(),
        value: json!(2),
    }];
    assert!(
        compile_native_batch(
            m_tree::native_current_m_registry(),
            &p,
            "operation:conflict",
            "expression:acceptance",
            1,
            membership(),
            vec![scene, property],
            BTreeSet::from(["scene".into()])
        )
        .unwrap_err()
        .contains("conflicting")
    );
}

#[test]
fn retained_native_output_continuation_keeps_original_selector_and_rejects_forged_scope() {
    let registry = m_tree::native_current_m_registry();
    let p = procedure();
    let old = contribution(&p, "owned");
    let mut actual = old.generated_basis.clone();
    actual["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
    let current = CurrentContribution {
        contribution_ref: old.contribution_ref.clone(),
        material: actual.clone(),
        overlays: vec![],
    };
    let mut next = old.clone();
    next.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["hold"] = json!(3.5);
    next.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"] =
        json!(true);
    let delta = regenerate(
        &[old.clone()],
        &[current.clone()],
        &[next.clone()],
        p.removal_policy,
    )
    .unwrap();
    next.native_changes =
        regeneration_native_changes(&delta, &[old.clone()], &[current.clone()]).unwrap();
    // This is pure producer input validation. Only native S can attest the
    // opaque typed journal digest and actual consumer observations at apply.
    let envelope = json!({"operation_ref":"operation:original","expression_ref":"expression:acceptance","expected_revision":1,
        "actor":"agent:anima","scope":{"kind":"expression"},"changes":old.native_changes,
        "sources":[{"ref":p.profile.source_ref,"revision":p.profile.revision,"availability":"available"},
            {"ref":old.recipe.source_ref,"revision":old.recipe.revision,"availability":"available"}],
        "participants":[],"timing":{"kind":"immediate"},"cause_ref":null});
    let output = RetainedOutputReading {
        schema: RETAINED_OUTPUT_READING.into(),
        native_owner: "oi.expression".into(),
        expression_ref: "expression:acceptance".into(),
        document_revision: 4,
        procedure_ref: p.procedure_ref.clone(),
        source_basis: vec![p.profile.clone(), old.recipe.clone()],
        contribution_ref: old.contribution_ref.clone(),
        output_slot: old.output_slot.clone(),
        subject_refs: old.subjects.clone(),
        occurrence_ref: old.occurrence_ref.clone(),
        recipe_revision: old.recipe.revision.clone(),
        owned_addresses: old.owned_addresses.clone(),
        generated_basis: old.generated_basis.clone(),
        current_basis: actual,
        status: "active".into(),
        applied_operation: json!({"fingerprint":fingerprint(&envelope).unwrap(),"envelope":envelope,"targets":old.owned_addresses,
            "status":"applied","accepted_revision":2,"applied_revision":3,"observations":[],"failure":null}),
    };
    let compile = |generated: GeneratedContribution, readings: Vec<RetainedOutputReading>| {
        compile_native_regeneration_batch(
            registry,
            &p,
            "operation:continuation",
            "expression:acceptance",
            4,
            membership(),
            vec![generated],
            BTreeSet::from(["scene".into()]),
            &[old.clone()],
            &[current.clone()],
            readings,
        )
    };
    assert!(
        compile(next.clone(), vec![])
            .unwrap_err()
            .contains("membership")
    );
    let prepared = compile(next.clone(), vec![output.clone()]).unwrap();
    assert_eq!(prepared.membership, membership());
    assert!(prepared.membership.targets.is_empty());
    assert_eq!(prepared.output_readings, vec![output.clone()]);
    assert_eq!(prepared.native_edit["expected_revision"], 4);
    let material = prepared.native_edit["changes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["change"] == "scene_material_set")
        .unwrap();
    assert_eq!(
        material["presentation"]["scene"]["entities"][0]["force"]["strength"],
        0.875
    );
    assert_eq!(
        material["presentation"]["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"],
        true
    );
    for mutate in 0..9 {
        let mut wrong = output.clone();
        match mutate {
            0 => wrong.document_revision = 3,
            1 => wrong.status = "detached".into(),
            2 => wrong.source_basis.clear(),
            3 => wrong.current_basis["scene"]["entities"][0]["force"]["strength"] = json!(0.1),
            4 => wrong.applied_operation["status"] = json!("prepared"),
            5 => {
                wrong.applied_operation["envelope"]["expression_ref"] = json!("expression:foreign")
            }
            6 => wrong.applied_operation["envelope"]["changes"] = json!([]),
            7 => wrong.current_basis["scene"]["procedural"] = json!({"operations":[]}),
            _ => {
                wrong.applied_operation["envelope"]["output_readings"] = json!([{"recursive":true}])
            }
        }
        assert!(compile(next.clone(), vec![wrong]).is_err());
    }
    let static_creation = output.applied_operation.clone();
    let mut previous_generation = old.clone();
    let mut current_generation = current.clone();
    let initial_size = serde_json::to_vec(&output).unwrap().len();
    for generation in 1..17 {
        let mut authored = p.clone();
        authored.revision = format!("revision:{generation}");
        authored.recipe.revision = format!("recipe:{generation}");
        let mut revised = previous_generation.clone();
        revised.procedure_revision = authored.revision.clone();
        revised.recipe = authored.recipe.clone();
        revised.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["hold"] =
            json!(3.5 + f64::from(generation));
        revised.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"] =
            json!(true);
        let delta = regenerate(
            &[previous_generation.clone()],
            &[current_generation.clone()],
            &[revised.clone()],
            p.removal_policy,
        )
        .unwrap();
        revised.native_changes = regeneration_native_changes(
            &delta,
            &[previous_generation.clone()],
            &[current_generation.clone()],
        )
        .unwrap();
        let mut reading = output.clone();
        reading.document_revision = 4 + generation as u64;
        reading.recipe_revision = previous_generation.recipe.revision.clone();
        reading.source_basis = vec![authored.profile.clone(), previous_generation.recipe.clone()];
        reading.generated_basis = previous_generation.generated_basis.clone();
        reading.current_basis = current_generation.material.clone();
        let continued = compile_native_regeneration_batch(
            registry,
            &authored,
            "operation:continued-static-origin",
            "expression:acceptance",
            reading.document_revision,
            membership(),
            vec![revised.clone()],
            BTreeSet::from(["scene".into()]),
            &[previous_generation],
            &[current_generation],
            vec![reading.clone()],
        )
        .unwrap();
        assert_eq!(continued.membership, membership());
        assert_eq!(
            continued.output_readings[0].applied_operation,
            static_creation
        );
        assert!(
            serde_json::to_vec(&reading).unwrap().len() <= initial_size + 256,
            "continuation recursively embeds earlier journals"
        );
        let effective = delta.effective_basis[&old.contribution_ref].clone();
        assert_eq!(
            effective["scene"]["entities"][0]["force"]["strength"],
            0.875
        );
        previous_generation = revised;
        current_generation = CurrentContribution {
            contribution_ref: old.contribution_ref.clone(),
            material: effective,
            overlays: vec![],
        };
    }
    let mut outside = next.clone();
    outside.owned_addresses.push(OwnedAddress {
        expression_ref: "expression:acceptance".into(),
        scene_ref: Some("expression:acceptance:scene:foreign".into()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        property: None,
    });
    outside.native_changes = vec![NativeChange::SceneMaterialSet {
        scene_ref: "expression:acceptance:scene:foreign".into(),
        presentation: outside.generated_basis.clone(),
    }];
    assert!(
        outside.native_changes[0]
            .validate()
            .unwrap_err()
            .contains("addressed native scene presentation")
    );
    let mut foreign_presentation = outside.generated_basis.clone();
    foreign_presentation["scene"]["id"] = json!("expression:acceptance:scene:foreign");
    outside.native_changes = vec![NativeChange::SceneMaterialSet {
        scene_ref: "expression:acceptance:scene:foreign".into(),
        presentation: foreign_presentation,
    }];
    outside.native_changes[0].validate().unwrap();
    assert!(
        compile(outside, vec![output])
            .unwrap_err()
            .contains("membership")
    );
}

#[test]
fn native_material_and_intent_projections_preserve_authorship_without_recursive_journals() {
    let mut material = presentation();
    material["saved"] = json!({"ref":"saved:source","revision":"original"});
    material["scene"]["procedural"] = json!({"procedures":[{"procedure_ref":"procedure:authored"}],"contributions":[{"contribution_ref":"contribution:retained"}],
        "controls":[{"takeover":{"value":0.875}}],"operations":[{"envelope":{"output_readings":[{"previous_journal":[]}]}}]});
    let projected = procedural_material_projection(&material).unwrap();
    assert!(projected["scene"].get("procedural").is_none());
    assert_eq!(projected["saved"], material["saved"]);
    assert_eq!(
        projected["scene"]["entities"],
        material["scene"]["entities"]
    );
    let intent = procedural_intent_projection(&material).unwrap();
    assert_eq!(intent["scene"]["procedural"]["operations"], json!([]));
    for key in ["procedures", "contributions", "controls"] {
        assert_eq!(
            intent["scene"]["procedural"][key],
            material["scene"]["procedural"][key]
        );
    }
    assert_eq!(intent["saved"], material["saved"]);
    let scene_ref = material["scene"]["id"].as_str().unwrap();
    assert!(
        NativeChange::SceneMaterialSet {
            scene_ref: scene_ref.into(),
            presentation: material.clone()
        }
        .validate()
        .unwrap_err()
        .contains("journal")
    );
    assert!(
        NativeChange::SceneMaterialSet {
            scene_ref: scene_ref.into(),
            presentation: intent
        }
        .validate()
        .is_ok()
    );
    assert_eq!(
        material["scene"]["procedural"]["operations"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn fingerprinted_retry_is_same_payload_and_capability_or_ownership_forgery_is_refused() {
    let registry = m_tree::native_current_m_registry();
    let p = procedure();
    let c = contribution(&p, "scene");
    let compile = |p: &Procedure, c: GeneratedContribution| {
        compile_native_batch(
            registry,
            p,
            "operation:same",
            "expression:acceptance",
            1,
            membership(),
            vec![c],
            BTreeSet::from(["scene".into()]),
        )
    };
    let a = compile(&p, c.clone()).unwrap();
    let b = compile(&p, c.clone()).unwrap();
    assert_eq!(a, b);
    let mut wrong = c.clone();
    wrong.subjects = vec!["native:wrong-subject".into()];
    assert!(compile(&p, wrong).is_err());
    let mut wrong = c.clone();
    wrong.native_changes.push(NativeChange::ParameterSet {
        entity_ref: "expression:other:entity:foreign".into(),
        parameter: "scale".into(),
        value: json!(2),
    });
    assert!(compile(&p, wrong).unwrap_err().contains("ownership"));
    let mut p = p.clone();
    p.admitted_changes.remove("subject_bind");
    assert!(compile(&p, c).is_err());
}

fn event(index: usize, active: bool, ancestors: Vec<String>) -> RuleEvent {
    RuleEvent {
        kind: TriggerKind::Explicit,
        active,
        cause: Cause {
            event_ref: format!("event:{index}"),
            ancestors,
            progress_revision: "progress:same".into(),
            depth: 1,
        },
    }
}

#[test]
fn edge_level_pause_seek_cancel_and_checkpoint_preserve_owner_cursor() {
    let mut p = procedure();
    let mut state = RuleExecution::new(&p, "interval:1").unwrap();
    state.enqueue(&p, event(1, true, vec![])).unwrap();
    assert!(state.next(&p, 1).unwrap().is_some());
    state.enqueue(&p, event(2, true, vec![])).unwrap();
    assert!(state.next(&p, 1).unwrap().is_none());
    state.pause(99);
    state.enqueue(&p, event(3, false, vec![])).unwrap();
    assert!(state.next(&p, 1).unwrap().is_none());
    let serialized = serde_json::to_value(&state).unwrap();
    let mut restored: RuleExecution = serde_json::from_value(serialized).unwrap();
    assert_eq!(restored.cursor, 99);
    assert!(restored.resume("wrong:epoch").is_err());
    restored.resume(&p.timing.epoch_ref).unwrap();
    restored.seek(120, &p.timing.epoch_ref).unwrap();
    assert_eq!(restored.cursor, 120);
    p.trigger.mode = TriggerMode::Level;
    assert!(
        restored
            .enqueue(&p, event(4, true, vec![]))
            .unwrap_err()
            .contains("original procedure")
    );
    // Changing an operative trigger requires explicit new authoring, while
    // the previous source-qualified checkpoint stays available for inspection.
    p.revision = "2".into();
    p.timing.requested_cursor = 120;
    let mut restored = RuleExecution::new(&p, "interval:authored-level").unwrap();
    restored.enqueue(&p, event(4, true, vec![])).unwrap();
    assert!(restored.next(&p, 1).unwrap().is_some());
    restored.enqueue(&p, event(5, true, vec![])).unwrap();
    assert!(restored.next(&p, 1).unwrap().is_some());
    restored.cancel(130);
    assert_eq!(restored.position, RulePosition::Cancelled);
    assert!(restored.enqueue(&p, event(6, true, vec![])).is_err());
}

#[test]
fn retained_rule_checkpoint_refuses_valid_source_recipe_profile_locus_or_scope_retargeting() {
    let p = procedure();
    let original = RuleExecution::new(&p, "interval:source-qualified").unwrap();
    let serialized = serde_json::to_value(&original).unwrap();
    assert_eq!(
        serialized["original_procedure"],
        serde_json::to_value(&p).unwrap()
    );
    for mutation in 0..8 {
        let mut changed = p.clone();
        match mutation {
            0 => changed.recipe.revision = "2".into(),
            1 => changed.profile.revision = "2".into(),
            2 => changed.locus_ref = "ql:m-coordinate:bimba:M2".into(),
            3 => changed.registry_revision = "different:registry".into(),
            4 => changed.principal_subject_ref = "subject:other".into(),
            5 => {
                changed
                    .recipe_parameters
                    .insert("source_treatment".into(), json!("different_material"));
            }
            6 => {
                changed.seed = "new-seed".into();
            }
            _ => {
                changed.selector = Selector::Occurrences {
                    refs: vec!["occurrence:other".into()],
                };
            }
        };
        let mut restored: RuleExecution = serde_json::from_value(serialized.clone()).unwrap();
        assert!(restored.verify_procedure(&changed).is_err());
        assert!(restored.enqueue(&changed, event(1, true, vec![])).is_err());
        assert!(restored.next(&changed, 1).is_err());
        assert_eq!(serde_json::to_value(&restored).unwrap(), serialized);
    }
}

#[test]
fn non_progress_cycle_queue_and_operation_budgets_stop_only_the_procedure() {
    let mut p = procedure();
    p.trigger.mode = TriggerMode::Level;
    p.budgets.max_queue = 1;
    p.budgets.max_operations = 2;
    let mut state = RuleExecution::new(&p, "interval:1").unwrap();
    state.enqueue(&p, event(1, true, vec![])).unwrap();
    assert!(state.enqueue(&p, event(2, true, vec![])).is_err());
    assert!(state.next(&p, 1).unwrap().is_some());
    state
        .enqueue(&p, event(3, true, vec![p.procedure_ref.clone()]))
        .unwrap();
    assert!(state.next(&p, 1).unwrap_err().contains("non-progress"));
    state.enqueue(&p, event(4, true, vec![])).unwrap();
    assert!(state.next(&p, 3).unwrap_err().contains("operations"));
    assert!(state.begin_interval("interval:1", 5).is_err());
    state.begin_interval("interval:2", 6).unwrap();
    assert!(state.begin_interval("interval:1", 7).is_err());
    let mut restored: RuleExecution =
        serde_json::from_value(serde_json::to_value(&state).unwrap()).unwrap();
    assert!(restored.begin_interval("interval:1", 8).is_err());
}

#[test]
fn seeded_replay_and_cprime_dependencies_use_actual_native_return_material() {
    let p = procedure();
    let a = (0..128)
        .map(|draw| seeded_index(&p.seed, &p.occurrence_ref, draw, 8).unwrap())
        .collect::<Vec<_>>();
    let b = (0..128)
        .map(|draw| seeded_index(&p.seed, &p.occurrence_ref, draw, 8).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(a, b);
    assert!(a.iter().collect::<BTreeSet<_>>().len() > 1);
    assert!(seeded_index(&p.seed, &p.occurrence_ref, 0, 0).is_err());
    let slots = vec!["a".into(), "b".into(), "c".into()];
    let mut p = p;
    p.composition.thread = vak_profile::ThreadForm::Chain;
    assert!(cprime_dependencies(&p, &slots, &BTreeMap::new()).is_err());
    let returns = BTreeMap::from([
        ("a".into(), "native:return:a".into()),
        ("b".into(), "native:return:b".into()),
    ]);
    let chain = cprime_dependencies(&p, &slots, &returns).unwrap();
    assert_eq!(chain["b"], vec!["native:return:a"]);
    p.composition.thread = vak_profile::ThreadForm::Fusion;
    assert_eq!(
        cprime_dependencies(&p, &slots, &returns).unwrap()["c"],
        vec!["native:return:a", "native:return:b"]
    );
}
