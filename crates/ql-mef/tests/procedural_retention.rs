//! Real producer retirement and metadata compiler functions; native receiving
//! effects/clock/physical ACK are independently exercised by actual owner gates.
use ql_mef::{
    m_tree, procedural_composition::*, procedural_manifestation::*, procedural_retention::*,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
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
fn reading(p: &Procedure, c: &GeneratedContribution, actual: Value) -> RetainedOutputReading {
    // Intake fixture only: native S must attest its real protected operation,
    // original source producer and current CAS independently before applying.
    let envelope = json!({"operation_ref":"operation:first","expression_ref":"expression:acceptance","expected_revision":1,"actor":"agent:anima","scope":{"kind":"expression"},
        "changes":c.native_changes,"sources":[{"ref":p.profile.source_ref,"revision":p.profile.revision,"availability":"available"},{"ref":p.recipe.source_ref,"revision":p.recipe.revision,"availability":"available"}],
        "participants":[],"timing":{"kind":"immediate"},"cause_ref":null});
    RetainedOutputReading {
        schema: RETAINED_OUTPUT_READING.into(),
        native_owner: "oi.expression".into(),
        expression_ref: "expression:acceptance".into(),
        document_revision: 4,
        procedure_ref: p.procedure_ref.clone(),
        source_basis: vec![p.recipe.clone(), p.profile.clone()],
        origin_source_basis: None,
        contribution_ref: c.contribution_ref.clone(),
        output_slot: c.output_slot.clone(),
        subject_refs: c.subjects.clone(),
        occurrence_ref: c.occurrence_ref.clone(),
        recipe_revision: c.recipe.revision.clone(),
        owned_addresses: c.owned_addresses.clone(),
        generated_basis: c.generated_basis.clone(),
        current_basis: actual,
        status: "active".into(),
        applied_operation: json!({"fingerprint":fingerprint(&envelope).unwrap(),"envelope":envelope,"targets":c.owned_addresses,"status":"applied","accepted_revision":2,"applied_revision":3,"observations":[],"failure":null}),
    }
}
fn materialization() -> NativeMaterialization {
    let mut presentation: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap();
    presentation["scene"]["id"] = json!("expression:acceptance:scene:canonical");
    NativeMaterialization {
        schema: MATERIALIZATION_CONTRACT.into(),
        lifecycles: vec![],
        document_revision: 4,
        rule_cursor: 1,
        state: "retired".into(),
        scenes: vec![NativeRetentionScene {
            scene_ref: "expression:acceptance:scene:canonical".into(),
            document_revision: 4,
            current_presentation: Some(presentation),
            existing_retention: Some(
                json!({"schema":RETENTION_SCHEMA,"bindings":[],"procedures":[],"contributions":[],"controls":[],"operations":[],"scene_flow":[],"time_mappings":[],"source_basis":[]}),
            ),
            principal: subject(),
            contributors: vec![],
            locus: NativeReading {
                reference: "ql:m-coordinate:bimba:M3".into(),
                revision: "current-native-profile".into(),
                availability: ReadingAvailability::Available,
            },
        }],
    }
}
fn force(p: &Procedure) -> GeneratedContribution {
    let address = native_parameter_address(
        "expression:acceptance",
        "expression:acceptance:scene:canonical",
        "expression:acceptance:entity:existing",
        "force_radius",
    )
    .unwrap();
    let subjects = vec![p.principal_subject_ref.clone()];
    let slot = "radius";
    let occurrence = "expression:acceptance:entity:existing";
    GeneratedContribution {
        contribution_ref: contribution_identity(&p.procedure_ref, slot, &subjects, occurrence)
            .unwrap(),
        procedure_ref: p.procedure_ref.clone(),
        procedure_revision: p.revision.clone(),
        recipe: p.recipe.clone(),
        output_slot: slot.into(),
        subjects,
        occurrence_ref: occurrence.into(),
        owned_addresses: vec![address.clone()],
        native_changes: vec![NativeChange::ParameterSet {
            entity_ref: occurrence.into(),
            parameter: "force_radius".into(),
            value: json!(200),
        }],
        generated_basis: json!({"parameter":"force_radius","value":200,"authored_basis":{"schema":"ql.native-parameter-state/v1","address":address,"parameter":"force_radius","value":120,"target_revision":1}}),
    }
}
fn retire(
    p: &Procedure,
    c: &GeneratedContribution,
    actual: Value,
    context: &NativeMaterialization,
) -> std::result::Result<PreparedProcedure, String> {
    let r = reading(p, c, actual.clone());
    prepare_native_retirement(
        m_tree::native_current_m_registry(),
        p,
        "operation:retire",
        "expression:acceptance",
        4,
        membership(),
        std::slice::from_ref(c),
        &[CurrentContribution {
            contribution_ref: c.contribution_ref.clone(),
            material: actual,
            overlays: vec![],
        }],
        vec![r],
        BTreeSet::from(["scene".into()]),
        context,
    )
}
#[test]
fn unedited_force_restores_original_native_units_and_edited_force_detaches_without_reset() {
    let p = procedure();
    let c = force(&p);
    let mut actual = c.generated_basis["authored_basis"].clone();
    actual["value"] = json!(200);
    actual["target_revision"] = json!(3);
    let prepared = retire(&p, &c, actual.clone(), &materialization()).unwrap();
    assert_eq!(
        prepared.native_edit["changes"][0]["change"],
        "scene_material_set"
    );
    assert_eq!(
        prepared.native_edit["changes"][1]["change"],
        "parameter_set"
    );
    assert_eq!(prepared.native_edit["changes"][1]["value"], 120);
    assert_eq!(
        prepared.native_edit["changes"][0]["presentation"]["scene"]["procedural"]["contributions"]
            [0]["status"],
        "retired"
    );
    assert_eq!(prepared.membership, membership());
    actual["value"] = json!(320);
    let detached = retire(&p, &c, actual.clone(), &materialization()).unwrap();
    let changes = detached.native_edit["changes"].as_array().unwrap();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0]["change"], "scene_material_set");
    assert_eq!(
        changes[0]["presentation"]["scene"]["procedural"]["contributions"][0]["status"],
        "detached"
    );
    let mut conflict = p.clone();
    conflict.removal_policy = RemovalPolicy::ConflictOnEdited;
    assert!(retire(&conflict, &c, actual, &materialization()).is_err());
}
#[test]
fn atlas_retirement_restores_actual_native_relation_selection_and_keeps_world() {
    let mut p = procedure();
    p.admitted_changes.insert("relation_focus".into());
    let subjects = vec![p.principal_subject_ref.clone()];
    let occurrence = "expression:acceptance";
    let flow = vec![NativeChange::Focus {
        scene_ref: "expression:acceptance:scene:canonical".into(),
        entity_ref: None,
    }];
    let baseline = json!({"schema":"ql.native-atlas-state/v1","expression_ref":occurrence,"focus":{"scene_ref":"expression:acceptance:scene:canonical","entity_ref":null,"relation_ref":"binding:original"},"scene_order":["expression:acceptance:scene:canonical"]});
    let c = GeneratedContribution {
        contribution_ref: contribution_identity(
            &p.procedure_ref,
            "native-flow",
            &subjects,
            occurrence,
        )
        .unwrap(),
        procedure_ref: p.procedure_ref.clone(),
        procedure_revision: p.revision.clone(),
        recipe: p.recipe.clone(),
        output_slot: "native-flow".into(),
        subjects,
        occurrence_ref: occurrence.into(),
        owned_addresses: vec![OwnedAddress {
            expression_ref: occurrence.into(),
            scene_ref: None,
            entity_ref: None,
            component: "expression".into(),
            constituent_ref: None,
            parent_ref: None,
            property: None,
        }],
        native_changes: flow.clone(),
        generated_basis: json!({"native_flow":flow,"authored_basis":baseline}),
    };
    let mut actual = baseline;
    actual["focus"]["relation_ref"] = Value::Null;
    let prepared = retire(&p, &c, actual, &materialization()).unwrap();
    assert_eq!(
        prepared.native_edit["changes"][1]["change"],
        "relation_focus"
    );
    assert_eq!(
        prepared.native_edit["changes"][1]["binding_ref"],
        "binding:original"
    );
    assert_eq!(
        prepared.native_edit["changes"][0]["presentation"]["scene"]["procedural"]["contributions"]
            [0]["owned_addresses"][0]["component"],
        "expression"
    );
}
#[test]
fn human_edited_atlas_detaches_without_requiring_absent_authored_focus_reset() {
    let p = procedure();
    let subjects = vec![p.principal_subject_ref.clone()];
    let occurrence = "expression:acceptance";
    let flow = vec![NativeChange::Focus {
        scene_ref: "expression:acceptance:scene:canonical".into(),
        entity_ref: None,
    }];
    let baseline = json!({"schema":"ql.native-atlas-state/v1","expression_ref":occurrence,"focus":null,"scene_order":["expression:acceptance:scene:canonical"]});
    let c = GeneratedContribution {
        contribution_ref: contribution_identity(
            &p.procedure_ref,
            "native-flow",
            &subjects,
            occurrence,
        )
        .unwrap(),
        procedure_ref: p.procedure_ref.clone(),
        procedure_revision: p.revision.clone(),
        recipe: p.recipe.clone(),
        output_slot: "native-flow".into(),
        subjects,
        occurrence_ref: occurrence.into(),
        owned_addresses: vec![OwnedAddress {
            expression_ref: occurrence.into(),
            scene_ref: None,
            entity_ref: None,
            component: "expression".into(),
            constituent_ref: None,
            parent_ref: None,
            property: None,
        }],
        native_changes: flow.clone(),
        generated_basis: json!({"native_flow":flow,"authored_basis":baseline}),
    };
    // Actual native Document has no current Selection after a human clear.
    // This is edited material and detaches; it does not need a reset operation.
    let prepared = retire(&p, &c, baseline.clone(), &materialization()).unwrap();
    assert_eq!(prepared.native_edit["changes"].as_array().unwrap().len(), 1);
    assert_eq!(
        prepared.native_edit["changes"][0]["change"],
        "scene_material_set"
    );
    assert_eq!(
        prepared.native_edit["changes"][0]["presentation"]["scene"]["procedural"]["contributions"]
            [0]["status"],
        "detached"
    );
    let mut original_effective = baseline;
    original_effective["focus"] =
        json!({"scene_ref":"expression:acceptance:scene:canonical","entity_ref":null});
    assert!(
        retire(&p, &c, original_effective, &materialization())
            .unwrap_err()
            .contains("clear-selection")
    );
}
#[test]
fn original_authored_parameter_basis_is_immutable_under_retained_regeneration() {
    let p = procedure();
    let c = force(&p);
    let mut actual = c.generated_basis["authored_basis"].clone();
    actual["value"] = json!(200);
    let output = reading(&p, &c, actual.clone());
    let mut next = c.clone();
    next.generated_basis["authored_basis"]["value"] = json!(999);
    assert!(
        compile_native_regeneration_batch(
            m_tree::native_current_m_registry(),
            &p,
            "operation:forged-base",
            "expression:acceptance",
            4,
            membership(),
            vec![next],
            BTreeSet::from(["scene".into()]),
            std::slice::from_ref(&c),
            &[CurrentContribution {
                contribution_ref: c.contribution_ref.clone(),
                material: actual,
                overlays: vec![]
            }],
            vec![output]
        )
        .unwrap_err()
        .contains("authored native basis")
    );
}

#[test]
fn source_anchor_survives_scene_delete_suppresses_generation_and_explicitly_releases() {
    use ql_mef::procedural_conduct::{
        NativeRecipeProgram, NativeSceneSource, SceneRecipe, scene_deletion_tombstone,
    };
    use ql_mef::procedural_intervention::ManualEdit;
    let mut p = procedure();
    let mut context = materialization();
    context.state = "held".into();
    let source = context.scenes[0].current_presentation.clone().unwrap();
    let program = NativeRecipeProgram::SceneMaterial {
        outputs: vec![SceneRecipe {
            output_slot: "passage".into(),
            scene_ref: "expression:acceptance:scene:generated".into(),
            sequence_holds: vec![],
            source: NativeSceneSource {
                native_owner: "oi.expression".into(),
                expression_ref: p.occurrence_ref.clone(),
                scene_ref: context.scenes[0].scene_ref.clone(),
                document_revision: 4,
                source_basis: p.recipe.clone(),
                material_fingerprint: fingerprint(&source).unwrap(),
                presentation: source.clone(),
                principal: subject(),
                contributors: vec![],
                locus_ref: p.locus_ref.clone(),
                locus_revision: "current-native-profile".into(),
            },
        }],
    };
    p.recipe_parameters
        .insert("native_program".into(), json!(program));
    let c = instantiate_scene(
        &p,
        &p.occurrence_ref,
        "passage",
        "expression:acceptance:scene:generated",
        &[subject()],
        &source,
    )
    .unwrap();
    context.scenes.push(NativeRetentionScene {
        scene_ref: c.occurrence_ref.clone(),
        document_revision: 4,
        existing_retention: None,
        current_presentation: None,
        principal: subject(),
        contributors: vec![],
        locus: context.scenes[0].locus.clone(),
    });
    let mut prepared = compile_native_batch(
        m_tree::native_current_m_registry(),
        &p,
        "operation:construct",
        &p.occurrence_ref,
        4,
        membership(),
        vec![c.clone()],
        BTreeSet::from(["scene".into()]),
    )
    .unwrap();
    let mut missing = context.clone();
    missing.scenes.remove(0);
    assert!(materialize_retention(&mut prepared.clone(), &missing).is_err());
    materialize_retention(&mut prepared, &context).unwrap();
    assert_eq!(prepared.membership, membership());
    assert_eq!(
        prepared.metadata_scope,
        vec![source_scene_anchor(&p, &c).unwrap().unwrap()]
    );
    let changes = prepared.native_edit["changes"].as_array().unwrap();
    assert_eq!(changes[0]["change"], "scene_material_set");
    assert_eq!(changes[0]["scene_ref"], context.scenes[0].scene_ref);
    let anchored = &changes[0]["presentation"]["scene"]["procedural"];
    assert_eq!(
        anchored["contributions"][0]["owned_addresses"],
        json!(c.owned_addresses)
    );
    let entity_refs = c.generated_basis["scene"]["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            let id = e["id"].as_str().unwrap().to_owned();
            (id.clone(), id)
        })
        .collect::<BTreeMap<_, _>>();
    let mut authored = c.generated_basis.clone();
    authored["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
    let manual = ManualEdit {
        expression_ref: p.occurrence_ref.clone(),
        scene_ref: c.occurrence_ref.clone(),
        contribution_ref: c.contribution_ref.clone(),
        owned_addresses: c.owned_addresses.clone(),
        entity_refs: entity_refs.clone(),
        before: c.generated_basis.clone(),
        after: authored.clone(),
        actor_ref: "human:earlier".into(),
        operation_ref: "operation:real-manual-force".into(),
        document_revision: 4,
        retained_overlays: vec![],
        retained_native_records: vec![],
    };
    let earlier = ql_mef::procedural_intervention::extract_manual_interventions(&manual)
        .unwrap()
        .native_records;
    let mut anchored = anchored.clone();
    anchored["contributions"][0]["authored_overrides"] = json!(earlier);
    let mut anchor = context.scenes[0].clone();
    anchor.existing_retention = Some(anchored.clone());
    let request = SceneDeletionAnchor {
        procedure: p.clone(),
        contribution: serde_json::from_value(anchored["contributions"][0].clone()).unwrap(),
        anchor: anchor.clone(),
        edit: ManualEdit {
            expression_ref: p.occurrence_ref.clone(),
            scene_ref: c.occurrence_ref.clone(),
            contribution_ref: c.contribution_ref.clone(),
            owned_addresses: c.owned_addresses.clone(),
            entity_refs: entity_refs.clone(),
            before: authored.clone(),
            after: Value::Null,
            actor_ref: "human:owner".into(),
            operation_ref: "operation:remove-actual-scene".into(),
            document_revision: 5,
            retained_overlays: vec![],
            retained_native_records: earlier.clone(),
        },
    };
    let deletion = prepare_scene_deletion_anchor(&request).unwrap();
    assert!(
        matches!(&deletion.changes[0],NativeChange::SceneMaterialSet{scene_ref,..} if scene_ref==&anchor.scene_ref)
    );
    assert!(
        matches!(&deletion.changes[1],NativeChange::SceneRemove{scene_ref} if scene_ref==&c.occurrence_ref)
    );
    let NativeChange::SceneMaterialSet { presentation, .. } = &deletion.changes[0] else {
        unreachable!()
    };
    anchor.existing_retention = Some(presentation["scene"]["procedural"].clone());
    anchor.document_revision = 5;
    let mut current = context.clone();
    current.scenes = vec![anchor.clone()];
    current.document_revision = 5;
    assert!(scene_deletion_tombstone(&c, &current));
    // Wrong anchor cannot suppress the continuing world even with identical row.
    current.scenes[0].scene_ref = "expression:acceptance:scene:foreign".into();
    assert!(!scene_deletion_tombstone(&c, &current));
    let release = SceneDeletionRelease {
        procedure: p.clone(),
        contribution: serde_json::from_value(
            anchor.existing_retention.as_ref().unwrap()["contributions"][0].clone(),
        )
        .unwrap(),
        anchor,
        document_revision: 6,
        operation_ref: "operation:release-explicit".into(),
        actor_ref: "human:owner".into(),
    };
    let released = release_scene_deletion_anchor(&release).unwrap();
    assert_eq!(released.changes.len(), 1);
    let NativeChange::SceneMaterialSet { presentation, .. } = &released.changes[0] else {
        unreachable!()
    };
    current.scenes = vec![release.anchor.clone()];
    current.scenes[0].existing_retention = Some(presentation["scene"]["procedural"].clone());
    assert!(!scene_deletion_tombstone(&c, &current));
    current.document_revision = 6;
    current.scenes[0].document_revision = 6;
    let mut recreated = instantiate_scene(
        &p,
        &p.occurrence_ref,
        "passage",
        &c.occurrence_ref,
        &[subject()],
        &source,
    )
    .unwrap();
    assert_eq!(recreated.contribution_ref, c.contribution_ref);
    assert_eq!(recreated.owned_addresses, c.owned_addresses);
    assert!(
        ql_mef::procedural_conduct::apply_released_scene_interventions(
            &mut recreated,
            &p,
            &current
        )
        .unwrap()
    );
    assert_eq!(recreated.generated_basis, c.generated_basis); // source recipe remains inspectable
    let effective = recreated
        .native_changes
        .iter()
        .find_map(|change| match change {
            NativeChange::SceneMaterialSet { presentation, .. } => Some(presentation),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        effective["scene"]["entities"][0]["force"]["strength"],
        json!(0.875)
    );
    assert_eq!(
        effective["scene"]["entities"][0]["id"],
        c.generated_basis["scene"]["entities"][0]["id"]
    );
    let rows = presentation["scene"]["procedural"]["contributions"][0]["authored_overrides"]
        .as_array()
        .unwrap();
    assert!(
        rows.iter()
            .any(|r| r["actor"] == "human:earlier" && r["value"] == 0.875)
    );
    assert!(
        rows.iter()
            .any(|r| r["path"] == "/scene" && r["persistent"] == false)
    );
    let mut foreign = request;
    foreign.anchor.scene_ref = "expression:acceptance:scene:foreign".into();
    assert!(prepare_scene_deletion_anchor(&foreign).is_err());
}

#[test]
fn sustained_native_output_membership_retirement_keeps_original_empty_selection_and_baseline() {
    let p = procedure();
    let c = force(&p);
    let mut actual = c.generated_basis["authored_basis"].clone();
    actual["value"] = json!(200);
    actual["target_revision"] = json!(3);
    let source = reading(&p, &c, actual.clone());
    let current = CurrentContribution {
        contribution_ref: c.contribution_ref.clone(),
        material: actual,
        overlays: vec![],
    };
    let batch = prepare_native_output_regeneration(
        m_tree::native_current_m_registry(),
        &p,
        "operation:sustained-membership-left",
        &p.occurrence_ref,
        4,
        membership(),
        std::slice::from_ref(&c),
        &[current],
        &[],
        vec![source],
        BTreeSet::from(["scene".into()]),
        &materialization(),
    )
    .unwrap();
    assert_eq!(batch.membership, membership());
    assert_eq!(batch.contributions[0].contribution_ref, c.contribution_ref);
    assert_eq!(
        batch.contributions[0].generated_basis["authored_basis"],
        c.generated_basis["authored_basis"]
    );
    assert_eq!(batch.native_edit["changes"][1]["value"], 120);
    assert_eq!(
        batch.native_edit["changes"][0]["presentation"]["scene"]["procedural"]["contributions"][0]
            ["status"],
        "retired"
    );
}
