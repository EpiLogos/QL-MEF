//! Pure actual Source control functions over the production authoring template.
//! Native Parameter/CAS read and numerical preview receiving are separate gates.
use ql_mef::{
    m_tree, procedural_composition::*, procedural_control::*, procedural_manifestation::*,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;
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

fn input() -> NativeControlInput {
    let p = procedure();
    let mut material: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap();
    let scene = "expression:acceptance:scene:canonical";
    let native = "expression:acceptance:entity:existing";
    material["scene"]["id"] = json!(scene);
    let id = native.to_owned();
    material["scene"]["entities"][0]["id"] = json!(id);
    material["scene"]["entities"][0]["force"]["radius"] = json!(0.3);
    let encoded_id = id.replace(':', "%3A");
    let target = format!("entity:{encoded_id}:forces.radius");
    let sibling = format!("entity:{encoded_id}:forces.strength");
    let leader = json!({"id":"native-driver:radius","target":target,"entityId":id,"enabled":true,"type":"lfo","wave":"sine","min":0.1,"max":0.5,"rate":0.75,"phase":0.2,"blend":"replace","duration":2,"delay":0,"loop":"loop","firedAt":null,"clockId":"actual-source-clock:radius"});
    let mut child = leader.clone();
    child["id"] = json!("native-driver:strength");
    child["target"] = json!(sibling);
    child["syncWith"] = json!("native-driver:radius");
    child["rate"] = json!(7);
    child["min"] = json!(0.2);
    child["max"] = json!(0.8);
    material["scene"]["automation"] = json!([leader, child]);
    material["scene"]["propertyTracks"] = json!([{"id":"actual-track:radius","bind":"entity.force.radius","entityId":id,"points":[{"time":0,"value":0.3},{"time":2,"value":0.5}]}]);
    material["scene"]["procedural"] = json!({"schema":"oi.expression-procedural/v1","bindings":[],"procedures":[{"procedure_ref":p.procedure_ref,"revision":p.revision,"source_basis":[{"ref":p.recipe.source_ref,"revision":p.recipe.revision,"availability":"available"},{"ref":p.profile.source_ref,"revision":p.profile.revision,"availability":"available"}],"seed":{"algorithm":p.seed_algorithm,"version":"1","value":p.seed},"definition":p,"resolved_targets":[],"cursor":0,"state":"held","membership_events":[]}],"contributions":[],"controls":[],"operations":[],"scene_flow":[],"time_mappings":[],"source_basis":[{"ref":"source:canonical","revision":"1","availability":"available"}]});
    // Whole-Scene interventions walk every real constituent in the production
    // authoring template, including its force pin. Keep the exact identity map
    // for the complete material rather than only the controlled formation.
    let entity_refs = material["scene"]["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entity| {
            let actual = entity["id"].as_str().unwrap().to_owned();
            (actual.clone(), actual)
        })
        .collect();
    let mut candidate = material.clone();
    candidate["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    NativeControlInput {
        procedure: p,
        actor_ref: "human:owner".into(),
        operation_ref: "operation:takeover-native-radius".into(),
        action: NativeControlAction::Takeover {
            value: json!(640),
            lifetime: ControlLifetime::Persistent,
        },
        reading: NativeParameterDriverReading {
            qualified_legacy_procedures: vec![],
            qualified_control_peers: vec![],
            schema: "ql.native-parameter-driver/v1".into(),
            expression_ref: "expression:acceptance".into(),
            document_revision: 8,
            entity_ref: native.into(),
            parameter: "force_radius".into(),
            native_parameter: json!({"value":120,"automation":{"min":80,"max":200,"rate_hz":0.75,"waveform":"sine"}}),
            addresses: vec![
                native_parameter_address("expression:acceptance", scene, native, "force_radius")
                    .unwrap(),
            ],
            scenes: vec![NativeDriverScene {
                scene_ref: scene.into(),
                entity_refs,
                presentation: material,
                parameter_candidate: Some(candidate),
            }],
        },
    }
}
#[test]
fn native_takeover_release_preserves_exact_driver_clock_and_native_baseline_units() {
    let request = input();
    let prepared = prepare_native_control(&request).unwrap();
    let changes = prepared["native_edit"]["changes"].as_array().unwrap();
    assert_eq!(changes[0]["change"], "scene_material_set");
    assert_eq!(changes[1]["change"], "parameter_manual");
    assert_eq!(changes[2]["change"], "parameter_set");
    assert_eq!(changes[2]["value"], 640);
    let material = &changes[0]["presentation"];
    let control = &material["scene"]["procedural"]["controls"][0];
    assert_eq!(control["authored_base"], 0.3);
    assert_eq!(control["native_base"]["value"], 120);
    assert_eq!(control["takeover"]["value"], 1.6);
    assert_eq!(control["takeover"]["native_value"], 640);
    let promoted = &material["scene"]["automation"][0];
    assert_eq!(promoted["id"], "native-driver:strength");
    assert_eq!(promoted["rate"], 0.75);
    assert_eq!(promoted["clockId"], "actual-source-clock:radius");
    assert!(promoted.get("syncWith").is_none());
    assert!(
        material["scene"]["propertyTracks"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    // The actual receiving ParameterSet, not Source, owns material projection.
    assert_eq!(material["scene"]["entities"][0]["force"]["radius"], 0.3);
    let mut release = request.clone();
    release.action = NativeControlAction::Release {};
    release.operation_ref = "operation:release-native-radius".into();
    release.reading.document_revision = 9;
    release.reading.native_parameter = json!({"value":640});
    release.reading.scenes[0].presentation = material.clone();
    release.reading.scenes[0].parameter_candidate = None;
    let released = prepare_native_control(&release).unwrap();
    let ops = released["native_edit"]["changes"].as_array().unwrap();
    assert_eq!(ops[2]["value"], 120);
    assert_eq!(ops[3]["change"], "parameter_automate");
    assert_eq!(
        ops[3]["automation"],
        request.reading.native_parameter["automation"]
    );
    assert_eq!(
        ops[0]["presentation"]["scene"]["automation"],
        request.reading.scenes[0].presentation["scene"]["automation"]
    );
    assert_eq!(
        ops[0]["presentation"]["scene"]["propertyTracks"],
        request.reading.scenes[0].presentation["scene"]["propertyTracks"]
    );
    assert!(
        ops[0]["presentation"]["scene"]["procedural"]["controls"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    release.reading.scenes[0].presentation["scene"]["automation"][0]["rate"] = json!(1.25);
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("group changed")
    );
}
#[test]
fn native_control_fences_foreign_preview_original_definition_and_complete_locations() {
    let request = input();
    let mut foreign = request.clone();
    foreign.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["caption"] = json!("unrelated edit");
    assert!(prepare_native_control(&foreign).is_err());
    let mut foreign = request.clone();
    foreign.procedure.seed = "different original source".into();
    assert!(
        prepare_native_control(&foreign)
            .unwrap_err()
            .contains("full original procedure")
    );
    let mut foreign = request.clone();
    foreign.reading.addresses[0].property = Some("strength".into());
    assert!(prepare_native_control(&foreign).is_err());
    let mut foreign = request.clone();
    foreign.reading.scenes[0].presentation["scene"]["automation"][1]["syncWith"] =
        json!("native-driver:strength");
    assert!(
        prepare_native_control(&foreign)
            .unwrap_err()
            .contains("cyclic")
    );
    let mut shared = request.clone();
    let mut other = request.reading.scenes[0].clone();
    other.scene_ref = "expression:acceptance:scene:second".into();
    other.presentation["scene"]["id"] = json!(other.scene_ref);
    other.presentation["scene"]
        .as_object_mut()
        .unwrap()
        .remove("procedural");
    other.parameter_candidate.as_mut().unwrap()["scene"]
        .as_object_mut()
        .unwrap()
        .remove("procedural");
    other.parameter_candidate.as_mut().unwrap()["scene"]["id"] = json!(other.scene_ref);
    shared.reading.scenes.push(other);
    assert!(prepare_native_control(&shared).is_err());
    shared.reading.addresses.push(
        native_parameter_address(
            "expression:acceptance",
            "expression:acceptance:scene:second",
            "expression:acceptance:entity:existing",
            "force_radius",
        )
        .unwrap(),
    );
    let positive = prepare_native_control(&shared).unwrap();
    assert_eq!(positive["resolved_scope"].as_array().unwrap().len(), 2);
    assert_eq!(
        positive["native_edit"]["changes"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|c| c["change"] == "parameter_set")
            .count(),
        1
    );
}

#[test]
fn native_base_edit_preserves_drivers_and_gesture_release_requires_exact_takeover_operation() {
    let mut request = input();
    request.action = NativeControlAction::SetBase { value: json!(640) };
    let base = prepare_native_control(&request).unwrap();
    assert_eq!(
        base["native_edit"]["changes"][0]["presentation"]["scene"]["automation"],
        request.reading.scenes[0].presentation["scene"]["automation"]
    );
    assert_eq!(
        base["native_edit"]["changes"][0]["presentation"]["scene"]["propertyTracks"],
        request.reading.scenes[0].presentation["scene"]["propertyTracks"]
    );
    assert_eq!(
        base["native_edit"]["changes"][3]["automation"],
        request.reading.native_parameter["automation"]
    );
    request.action = NativeControlAction::Takeover {
        value: json!(640),
        lifetime: ControlLifetime::Gesture,
    };
    let taken = prepare_native_control(&request).unwrap();
    let mut release = request.clone();
    release.reading.document_revision = 9;
    release.reading.native_parameter = json!({"value":640});
    release.reading.scenes[0].presentation =
        taken["native_edit"]["changes"][0]["presentation"].clone();
    release.reading.scenes[0].parameter_candidate = None;
    release.action = NativeControlAction::ReleaseGesture {
        takeover_operation_ref: "foreign:gesture".into(),
    };
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("lifetime release")
    );
    release.action = NativeControlAction::ReleaseGesture {
        takeover_operation_ref: request.operation_ref.clone(),
    };
    assert_eq!(
        prepare_native_control(&release).unwrap()["native_edit"]["changes"][2]["value"],
        120
    );
}

#[test]
fn native_set_base_uses_same_attribution_and_survives_procedural_force_regeneration() {
    use ql_mef::procedural_effective::apply_retained_force_interventions;
    use ql_mef::procedural_retention::{
        MATERIALIZATION_CONTRACT, NativeMaterialization, NativeRetentionScene,
    };
    let mut request = input();
    request.action = NativeControlAction::SetBase { value: json!(640) };
    let address = request.reading.addresses[0].clone();
    let id = request.reading.entity_ref.clone();
    let p = &request.procedure;
    let subjects = vec![p.principal_subject_ref.clone()];
    let identity = contribution_identity(&p.procedure_ref, "radius", &subjects, &id).unwrap();
    let c = GeneratedContribution {
        contribution_ref: identity.clone(),
        procedure_ref: p.procedure_ref.clone(),
        procedure_revision: p.revision.clone(),
        recipe: p.recipe.clone(),
        output_slot: "radius".into(),
        subjects: subjects.clone(),
        occurrence_ref: id.clone(),
        owned_addresses: vec![address.clone()],
        native_changes: vec![NativeChange::ParameterSet {
            entity_ref: id.clone(),
            parameter: "force_radius".into(),
            value: json!(200),
        }],
        generated_basis: json!({"parameter":"force_radius","value":200}),
    };
    let prior = json!({"address":address,"actor":"human:older","persistent":true,"revision":7,"path":format!("/scene/entities/@{id}/force/radius"),"value":0.2,"kind":"set","operation_ref":"operation:older","before":0.1});
    request.reading.scenes[0].presentation["scene"]["procedural"]["contributions"] = json!([{"contribution_ref":identity,"procedure_ref":p.procedure_ref,"output_slot":"radius","subject_refs":subjects,"occurrence_ref":id,"recipe_revision":p.recipe.revision,"owned_addresses":[address],"generated_basis":c.generated_basis,"status":"active","authored_overrides":[prior]}]);
    request.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        request.reading.scenes[0].presentation["scene"]["procedural"].clone();
    let prepared = prepare_native_control(&request).unwrap();
    let material = prepared["native_edit"]["changes"][0]["presentation"].clone();
    let row = &material["scene"]["procedural"]["contributions"][0]["authored_overrides"][0];
    assert_eq!(row["value"], 1.6);
    assert_eq!(row["actor"], request.actor_ref);
    assert_eq!(row["revision"], 9);
    assert_eq!(row["operation_ref"], request.operation_ref);
    assert_eq!(row["path"], format!("/scene/entities/@{id}/force/radius"));
    assert_eq!(
        material["scene"]["automation"],
        request.reading.scenes[0].presentation["scene"]["automation"]
    );
    let reading = TargetReading {
        occurrence_ref: id,
        address: request.reading.addresses[0].clone(),
        subject: subject(),
        revision: 9,
        tags: vec![],
        properties: BTreeMap::from([("force_radius".into(), json!(640))]),
    };
    let context = NativeMaterialization {
        schema: MATERIALIZATION_CONTRACT.into(),
        lifecycles: vec![],
        document_revision: 9,
        rule_cursor: 1,
        state: "running".into(),
        scenes: vec![NativeRetentionScene {
            scene_ref: request.reading.scenes[0].scene_ref.clone(),
            document_revision: 9,
            current_presentation: Some(material.clone()),
            existing_retention: Some(material["scene"]["procedural"].clone()),
            principal: subject(),
            contributors: vec![],
            locus: NativeReading {
                reference: p.locus_ref.clone(),
                revision: "current-native-profile".into(),
                availability: ReadingAvailability::Available,
            },
        }],
    };
    let mut next = vec![c.clone()];
    let proof = apply_retained_force_interventions(&mut next, &[reading], &context).unwrap();
    let NativeChange::ParameterSet { value, .. } = &next[0].native_changes[0] else {
        unreachable!()
    };
    assert_eq!(value, &json!(640));
    assert_eq!(next[0].generated_basis, c.generated_basis);
    assert_eq!(proof[0].actor_ref, request.actor_ref);
    assert_eq!(
        proof[0].operation_ref.as_deref(),
        Some(request.operation_ref.as_str())
    );
}

#[test]
fn emit_actual_source_control_consumer_artifact_when_owner_requests_it() {
    let mut request = input();
    let e = &mut request.reading.scenes[0].presentation["scene"]["entities"][0];
    e["sequence"]["enabled"] = json!(true);
    e["sequence"]["steps"][0]["objectState"] = json!({"normalized":true,"size":e["size"],"rotation":e["rotation"],"scale":e["scale"],"tint":e["tint"],"tintWeight":e["tintWeight"],"force":{"kind":"attract","strength":0.15,"radius":0.6,"spin":0.1}});
    let mut candidate = request.reading.scenes[0].presentation.clone();
    candidate["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    request.reading.scenes[0].parameter_candidate = Some(candidate);
    let takeover = prepare_native_control(&request).unwrap();
    let mut release = request.clone();
    release.action = NativeControlAction::Release {};
    release.operation_ref = "operation:release-native-radius".into();
    release.reading.document_revision = 9;
    release.reading.native_parameter = json!({"value":640});
    release.reading.scenes[0].presentation =
        takeover["native_edit"]["changes"][0]["presentation"].clone();
    release.reading.scenes[0].parameter_candidate = None;
    let released = prepare_native_control(&release).unwrap();
    assert_eq!(
        takeover["native_edit"]["changes"][0]["presentation"]["scene"]["entities"][0]["sequence"],
        request.reading.scenes[0].presentation["scene"]["entities"][0]["sequence"]
    );
    if let Ok(path) = std::env::var("QL_PROCEDURAL_CONTROL_CONSUMER_ARTIFACT") {
        let artifact = json!({"schema":"ql.procedural-control-consumer-source-fixture/v1","standing":"actual pure Source producer fixture; no native Document/clock/receiving/physical/audio authority","input":request,"takeover":takeover,"release":released});
        std::fs::write(path, serde_json::to_vec_pretty(&artifact).unwrap()).unwrap();
    }
}

fn refresh_input() -> (NativeActiveControlRefresh, NativeControlInput) {
    let mut original = input();
    original.action = NativeControlAction::Takeover {
        value: json!(640),
        lifetime: ControlLifetime::Gesture,
    };
    let scene = &original.reading.scenes[0].scene_ref;
    let owned = OwnedAddress {
        expression_ref: original.reading.expression_ref.clone(),
        scene_ref: Some(scene.clone()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        property: None,
        parent_ref: None,
    };
    let id = &original.reading.entity_ref;
    let prior = json!({"address":original.reading.addresses[0],"actor":"human:older","persistent":true,"revision":7,
        "path":format!("/scene/entities/@{id}/force/radius"),"value":0.2,"kind":"set","operation_ref":"operation:older","before":0.1});
    original.reading.scenes[0].presentation["scene"]["procedural"]["contributions"] = json!([{"contribution_ref":"contribution:whole-source-scene", "procedure_ref":original.procedure.procedure_ref,
        "owned_addresses":[owned],"authored_overrides":[prior],"status":"active"}]);
    original.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        original.reading.scenes[0].presentation["scene"]["procedural"].clone();
    let taken = prepare_native_control(&original).unwrap();
    let mut reading = original.reading.clone();
    reading.document_revision = 9;
    reading.native_parameter = json!({"value":640});
    reading.scenes[0].presentation = taken["native_edit"]["changes"][0]["presentation"].clone();
    // Native receiving's actual ParameterSet owns this conversion. These pure
    // source tests supply its declared existing preview; they do not test/mint
    // protected Root source origin or an effective field/body/audio ACK.
    reading.scenes[0].presentation["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    let mut candidate = reading.scenes[0].presentation.clone();
    candidate["scene"]["entities"][0]["force"]["radius"] = json!(0.75);
    candidate["scene"]["caption"] = json!("human caption in same ordinary Edit");
    reading.scenes[0].parameter_candidate = Some(candidate);
    (
        NativeActiveControlRefresh {
            procedure: original.procedure.clone(),
            reading,
            candidate_native_parameter: json!({"value":300}),
            actor_ref: "human:new-edit".into(),
            operation_ref: "operation:ordinary-radius-and-caption".into(),
        },
        original,
    )
}
#[test]
fn ordinary_active_control_refresh_uses_actual_native_value_preserves_lifetime_and_unrelated_human_changes()
 {
    let (request, original) = refresh_input();
    let result = refresh_native_active_control(&request).unwrap();
    let changes = result["native_edit"]["changes"].as_array().unwrap();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0]["change"], "scene_material_set");
    let scene = &changes[0]["presentation"]["scene"];
    assert_eq!(scene["caption"], "human caption in same ordinary Edit");
    assert_eq!(scene["entities"][0]["force"]["radius"], 0.75);
    let control = &scene["procedural"]["controls"][0];
    let old = &request.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0];
    for key in [
        "native_base",
        "authored_base",
        "dormant_lanes",
        "suspended_lanes",
        "dormant_tracks",
        "source_basis",
    ] {
        assert_eq!(control[key], old[key], "changed original {key}");
    }
    assert_eq!(control["takeover"]["native_value"], 300);
    assert_eq!(control["takeover"]["value"], 0.75);
    assert_eq!(control["takeover"]["lifetime"], "gesture");
    assert_eq!(control["takeover"]["operation_ref"], request.operation_ref);
    assert_eq!(
        control["takeover"]["lifetime_operation_ref"],
        original.operation_ref
    );
    let rows = scene["procedural"]["contributions"][0]["authored_overrides"]
        .as_array()
        .unwrap();
    let radius = rows
        .iter()
        .find(|r| r["address"]["component"] == "force")
        .unwrap();
    let caption = rows
        .iter()
        .find(|r| r["address"]["property"] == "caption")
        .unwrap();
    assert_eq!(radius["value"], 0.75);
    assert_eq!(radius["persistent"], false);
    assert_eq!(radius["actor"], request.actor_ref);
    assert_eq!(caption["persistent"], true);
    assert_eq!(caption["operation_ref"], request.operation_ref);
    let baseline = control["dormant_overrides"][0]["overrides"]
        .as_array()
        .unwrap();
    assert!(
        baseline
            .iter()
            .any(|r| r["address"]["component"] == "force" && r["value"] == 0.2)
    );
    assert!(
        baseline
            .iter()
            .any(|r| r["address"]["property"] == "caption"
                && r["value"] == "human caption in same ordinary Edit")
    );
    let mut release = original.clone();
    release.reading = request.reading.clone();
    release.reading.document_revision = 10;
    release.reading.native_parameter = json!({"value":300});
    release.reading.scenes[0].presentation = changes[0]["presentation"].clone();
    release.reading.scenes[0].parameter_candidate = None;
    release.action = NativeControlAction::ReleaseGesture {
        takeover_operation_ref: original.operation_ref.clone(),
    };
    let released = prepare_native_control(&release).unwrap();
    let returned = &released["native_edit"]["changes"][0]["presentation"]["scene"];
    assert_eq!(returned["caption"], scene["caption"]);
    assert_eq!(
        returned["procedural"]["contributions"][0]["authored_overrides"],
        control["dormant_overrides"][0]["overrides"]
    );
    assert_eq!(released["native_edit"]["changes"][2]["value"], 120);
    assert_eq!(
        released["native_edit"]["changes"][3]["automation"],
        original.reading.native_parameter["automation"]
    );
    release.action = NativeControlAction::ReleaseGesture {
        takeover_operation_ref: request.operation_ref.clone(),
    };
    assert!(
        prepare_native_control(&release).is_err(),
        "new normal Edit must not replace original gesture lifetime identity"
    );
    let entry = ql_mef::procedural_intervention::InterventionPreflight::ActiveControlRefresh {
        input: Box::new(request),
    };
    let batch = ql_mef::procedural_intervention::intervention_batch(&[entry]).unwrap();
    assert_eq!(batch["results"][0], result);
}
#[test]
fn ordinary_active_refresh_refuses_stale_control_baseline_metadata_forgery_and_new_automation() {
    let (request, _) = refresh_input();
    for kind in [
        "source",
        "native_value",
        "automation",
        "authored_lane",
        "authored_track",
        "baseline",
        "scope",
        "metadata",
        "no_candidate",
    ] {
        let mut bad = request.clone();
        match kind {
            "source" => bad.procedure.seed = "changed original source".into(),
            "native_value" => bad.reading.native_parameter["value"] = json!(639),
            "automation" => {
                bad.candidate_native_parameter["automation"] =
                    json!({"min":80,"max":200,"rate_hz":1,"waveform":"sine"})
            }
            "authored_lane" => {
                let target = bad.reading.scenes[0].presentation["scene"]["procedural"]["controls"]
                    [0]["target"]
                    .clone();
                bad.reading.scenes[0].parameter_candidate.as_mut().unwrap()["scene"]["automation"].as_array_mut().unwrap().push(json!({"id":"new-direct-target-driver","target":target,"enabled":true,"type":"lfo","rate":1}));
            }
            "authored_track" => {
                let id = bad.reading.entity_ref.clone();
                bad.reading.scenes[0].parameter_candidate.as_mut().unwrap()["scene"]["propertyTracks"] = json!([{"id":"new-direct-track","bind":"entity.force.radius","entityId":id,"points":[]}]);
            }
            "baseline" => {
                bad.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("native_base");
            }
            "scope" => bad.reading.addresses[0].property = Some("strength".into()),
            "metadata" => {
                bad.reading.scenes[0].parameter_candidate.as_mut().unwrap()["scene"]["procedural"]
                    ["controls"][0]["takeover"]["actor"] = json!("foreign:actor")
            }
            "no_candidate" => bad.reading.scenes[0].parameter_candidate = None,
            _ => unreachable!(),
        }
        assert!(
            refresh_native_active_control(&bad).is_err(),
            "accepted {kind}"
        );
    }
    let mut wire = serde_json::to_value(
        ql_mef::procedural_intervention::InterventionPreflight::ActiveControlRefresh {
            input: Box::new(request),
        },
    )
    .unwrap();
    for action in [
        "install",
        "event",
        "control_preflight",
        "material_readback",
        "source_bootstrap",
    ] {
        wire["action"] = json!(action);
        assert!(
            serde_json::from_value::<ql_mef::procedural_intervention::InterventionPreflight>(
                wire.clone()
            )
            .is_err()
        );
    }
}

#[test]
fn ordinary_active_refresh_preserves_all_shared_global_entity_locations_without_sibling_scope() {
    let (mut request, _) = refresh_input();
    let mut second = request.reading.scenes[0].clone();
    second.scene_ref = "expression:acceptance:scene:second".into();
    let second_address = native_parameter_address(
        &request.reading.expression_ref,
        &second.scene_ref,
        &request.reading.entity_ref,
        &request.reading.parameter,
    )
    .unwrap();
    second.presentation["scene"]["id"] = json!(second.scene_ref);
    second.presentation["scene"]["procedural"]["controls"][0]["address"] = json!(second_address);
    second.presentation["scene"]["procedural"]["contributions"] = json!([]);
    second.presentation["scene"]["procedural"]["controls"][0]["dormant_overrides"] = json!([]);
    let mut candidate = second.presentation.clone();
    candidate["scene"]["entities"][0]["force"]["radius"] = json!(0.75);
    candidate["scene"]["caption"] = json!("second actual Scene human caption");
    second.parameter_candidate = Some(candidate);
    request.reading.scenes.push(second);
    // Actual same native Entity requires every Scene location. Reading one
    // manifestation cannot silently enlarge the original scalar write scope.
    assert!(refresh_native_active_control(&request).is_err());
    request.reading.addresses.push(second_address.clone());
    let result = refresh_native_active_control(&request).unwrap();
    assert_eq!(result["resolved_scope"], json!(request.reading.addresses));
    let changes = result["native_edit"]["changes"].as_array().unwrap();
    assert_eq!(changes.len(), 2);
    for change in changes {
        assert_eq!(change["change"], "scene_material_set");
        assert_eq!(
            change["presentation"]["scene"]["procedural"]["controls"][0]["takeover"]["native_value"],
            300
        );
        assert_eq!(
            change["presentation"]["scene"]["entities"][0]["force"]["radius"],
            0.75
        );
        assert_eq!(
            change["presentation"]["scene"]["procedural"]["controls"][0]["native_base"],
            request.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0]["native_base"]
        );
    }
    assert_eq!(
        changes[1]["presentation"]["scene"]["caption"],
        "second actual Scene human caption"
    );
    let mut wrong = request;
    wrong.reading.addresses[1].property = Some("strength".into());
    assert!(refresh_native_active_control(&wrong).is_err());
}

#[test]
fn emit_actual_source_strength_takeover_preserving_authored_sequence_for_original_consumer_detector()
 {
    let mut request = input();
    request.operation_ref = "operation:takeover-native-strength".into();
    request.action = NativeControlAction::Takeover {
        value: json!(0.8),
        lifetime: ControlLifetime::Persistent,
    };
    request.reading.parameter = "force_strength".into();
    request.reading.native_parameter = json!({"value":0.3});
    request.reading.addresses = vec![
        native_parameter_address(
            &request.reading.expression_ref,
            &request.reading.scenes[0].scene_ref,
            &request.reading.entity_ref,
            "force_strength",
        )
        .unwrap(),
    ];
    let e = &mut request.reading.scenes[0].presentation["scene"]["entities"][0];
    e["force"]["strength"] = json!(0.3);
    e["sequence"]["enabled"] = json!(true);
    e["sequence"]["steps"][0]["objectState"] = json!({"normalized":true,"size":e["size"],"rotation":e["rotation"],"scale":e["scale"],"tint":e["tint"],"tintWeight":e["tintWeight"],"force":{"kind":"attract","strength":0.15,"radius":0.6,"spin":0.1}});
    let mut candidate = request.reading.scenes[0].presentation.clone();
    candidate["scene"]["entities"][0]["force"]["strength"] = json!(0.8);
    request.reading.scenes[0].parameter_candidate = Some(candidate);
    let takeover = prepare_native_control(&request).unwrap();
    assert_eq!(
        takeover["native_edit"]["changes"][2]["parameter"],
        "force_strength"
    );
    assert_eq!(takeover["native_edit"]["changes"][2]["value"], 0.8);
    assert_eq!(
        takeover["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["controls"][0]
            ["takeover"]["native_value"],
        0.8
    );
    assert_eq!(
        takeover["native_edit"]["changes"][0]["presentation"]["scene"]["entities"][0]["sequence"],
        request.reading.scenes[0].presentation["scene"]["entities"][0]["sequence"]
    );
    let mut release = request.clone();
    release.action = NativeControlAction::Release {};
    release.operation_ref = "operation:release-native-strength".into();
    release.reading.document_revision = 9;
    release.reading.native_parameter = json!({"value":0.8});
    release.reading.scenes[0].presentation =
        takeover["native_edit"]["changes"][0]["presentation"].clone();
    release.reading.scenes[0].parameter_candidate = None;
    let released = prepare_native_control(&release).unwrap();
    assert_eq!(released["native_edit"]["changes"][2]["value"], 0.3);
    if let Ok(path) = std::env::var("QL_PROCEDURAL_CONTROL_STRENGTH_ARTIFACT") {
        let artifact = json!({"schema":"ql.procedural-control-consumer-source-fixture/v1","standing":"actual pure Source strength producer fixture; no native Document/clock/receiving/physical/audio authority","input":request,"takeover":takeover,"release":released});
        std::fs::write(path, serde_json::to_vec_pretty(&artifact).unwrap()).unwrap();
    }
}

use ql_mef::procedural_intervention::{ManualEdit, extract_owned_manual_interventions};
#[test]
fn gesture_refresh_keeps_latest_sibling_under_prior_whole_entity_intervention() {
    let mut original = input();
    original.action = NativeControlAction::Takeover {
        value: json!(640),
        lifetime: ControlLifetime::Gesture,
    };
    let scene_ref = original.reading.scenes[0].scene_ref.clone();
    let entity_ref = original.reading.entity_ref.clone();
    let owned = OwnedAddress {
        expression_ref: original.reading.expression_ref.clone(),
        scene_ref: Some(scene_ref.clone()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        property: None,
        parent_ref: None,
    };
    let created = original.reading.scenes[0].presentation.clone();
    let mut before_creation = created.clone();
    before_creation["scene"]["entities"] = json!([]);
    // Obtain the broad native Entity intervention from the actual native source
    // extractor, as a real constituent creation. No caller row or runtime grant.
    let human_creation = ManualEdit {
        expression_ref: original.reading.expression_ref.clone(),
        scene_ref: scene_ref.clone(),
        contribution_ref: "contribution:source-whole-scene".into(),
        owned_addresses: vec![owned.clone()],
        entity_refs: original.reading.scenes[0].entity_refs.clone(),
        before: before_creation,
        after: created,
        actor_ref: "human:prior-entity-author".into(),
        operation_ref: "operation:prior-real-entity-add".into(),
        document_revision: 7,
        retained_overlays: vec![],
        retained_native_records: vec![],
    };
    let broad = extract_owned_manual_interventions(&human_creation)
        .unwrap()
        .native_records
        .into_iter()
        .find(|row| row.path == format!("/scene/entities/@{entity_ref}"))
        .unwrap();
    assert_eq!(broad.address.component, "entity");
    assert!(broad.address.property.is_none());
    original.reading.scenes[0].presentation["scene"]["procedural"]["contributions"] = json!([{
        "contribution_ref":"contribution:source-whole-scene", "procedure_ref":original.procedure.procedure_ref,
        "owned_addresses":[owned], "authored_overrides":[broad], "status":"active"
    }]);
    original.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        original.reading.scenes[0].presentation["scene"]["procedural"].clone();
    let taken = prepare_native_control(&original).unwrap();
    let mut reading = original.reading.clone();
    reading.document_revision = 9;
    reading.native_parameter = json!({"value":640});
    reading.scenes[0].presentation = taken["native_edit"]["changes"][0]["presentation"].clone();
    let mut candidate = reading.scenes[0].presentation.clone();
    candidate["scene"]["entities"][0]["force"]["radius"] = json!(0.75);
    candidate["scene"]["entities"][0]["name"] = json!("latest human entity name");
    reading.scenes[0].parameter_candidate = Some(candidate);
    let refreshed = refresh_native_active_control(&NativeActiveControlRefresh {
        procedure: original.procedure.clone(),
        reading: reading.clone(),
        candidate_native_parameter: json!({"value":300}),
        actor_ref: "human:latest-normal-edit".into(),
        operation_ref: "operation:radius-and-name".into(),
    })
    .unwrap();
    let material = &refreshed["native_edit"]["changes"][0]["presentation"];
    assert_eq!(
        material["scene"]["entities"][0]["name"],
        "latest human entity name"
    );
    let mut release = original;
    release.operation_ref = "operation:release-refreshed-gesture".into();
    release.reading = reading;
    release.reading.document_revision = 10;
    release.reading.native_parameter = json!({"value":300});
    release.reading.scenes[0].presentation = material.clone();
    release.reading.scenes[0].parameter_candidate = None;
    release.action = NativeControlAction::ReleaseGesture {
        takeover_operation_ref: "operation:takeover-native-radius".into(),
    };
    let released = prepare_native_control(&release).unwrap();
    let scene = &released["native_edit"]["changes"][0]["presentation"]["scene"];
    assert_eq!(scene["entities"][0]["name"], "latest human entity name");
    // Regeneration must read current persistent parent intervention material,
    // rather than an old whole Entity value that erases the later human name.
    let retained_parent = scene["procedural"]["contributions"][0]["authored_overrides"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["path"] == format!("/scene/entities/@{entity_ref}"))
        .unwrap();
    assert_eq!(retained_parent["value"]["name"], "latest human entity name");
    assert_eq!(
        retained_parent["value"]["force"]["radius"], 0.3,
        "only the controlled radius returns to its original persistent baseline"
    );
}

fn joined_gesture_inputs() -> Vec<NativeActiveControlRefresh> {
    let mut radius = input();
    radius.action = NativeControlAction::Takeover {
        value: json!(640),
        lifetime: ControlLifetime::Gesture,
    };
    // Independent native drivers are absent in this material case; the existing
    // group/clock tests above exercise the real driver suspension separately.
    radius.reading.scenes[0].presentation["scene"]["automation"] = json!([]);
    radius.reading.scenes[0].presentation["scene"]["propertyTracks"] = json!([]);
    radius.reading.scenes[0].presentation["scene"]["entities"][0]["force"]["strength"] = json!(0.3);
    let owned = OwnedAddress {
        expression_ref: radius.reading.expression_ref.clone(),
        scene_ref: Some(radius.reading.scenes[0].scene_ref.clone()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    let created = radius.reading.scenes[0].presentation.clone();
    let mut before = created.clone();
    before["scene"]["entities"] = json!([]);
    let rows = extract_owned_manual_interventions(&ManualEdit {
        expression_ref: radius.reading.expression_ref.clone(),
        scene_ref: radius.reading.scenes[0].scene_ref.clone(),
        contribution_ref: "contribution:joined-whole-scene".into(),
        owned_addresses: vec![owned.clone()],
        entity_refs: radius.reading.scenes[0].entity_refs.clone(),
        before,
        after: created,
        actor_ref: "human:original-author".into(),
        operation_ref: "operation:original-entity-add".into(),
        document_revision: 7,
        retained_overlays: vec![],
        retained_native_records: vec![],
    })
    .unwrap()
    .native_records;
    radius.reading.scenes[0].presentation["scene"]["procedural"]["contributions"] = json!([{
        "contribution_ref":"contribution:joined-whole-scene","procedure_ref":radius.procedure.procedure_ref,
        "owned_addresses":[owned],"authored_overrides":rows,"status":"active"}]);
    let mut candidate = radius.reading.scenes[0].presentation.clone();
    candidate["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    radius.reading.scenes[0].parameter_candidate = Some(candidate.clone());
    let taken = prepare_native_control(&radius).unwrap();
    candidate["scene"]["procedural"] =
        taken["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"].clone();
    let mut strength = radius.clone();
    strength.operation_ref = "operation:takeover-native-strength-gesture".into();
    strength.reading.document_revision = 9;
    strength.reading.parameter = "force_strength".into();
    strength.reading.native_parameter = json!({"value":0.3});
    strength.reading.addresses = vec![
        native_parameter_address(
            &strength.reading.expression_ref,
            &strength.reading.scenes[0].scene_ref,
            &strength.reading.entity_ref,
            "force_strength",
        )
        .unwrap(),
    ];
    strength.reading.scenes[0].presentation = candidate.clone();
    candidate["scene"]["entities"][0]["force"]["strength"] = json!(0.8);
    strength.reading.scenes[0].parameter_candidate = Some(candidate.clone());
    strength.action = NativeControlAction::Takeover {
        value: json!(0.8),
        lifetime: ControlLifetime::Gesture,
    };
    let taken = prepare_native_control(&strength).unwrap();
    candidate["scene"]["procedural"] =
        taken["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"].clone();
    radius.reading.document_revision = 10;
    radius.reading.native_parameter = json!({"value":640});
    radius.reading.scenes[0].presentation = candidate.clone();
    strength.reading.document_revision = 10;
    strength.reading.native_parameter = json!({"value":0.8});
    strength.reading.scenes[0].presentation = candidate.clone();
    candidate["scene"]["entities"][0]["force"]["radius"] = json!(0.75);
    candidate["scene"]["entities"][0]["force"]["strength"] = json!(0.65);
    candidate["scene"]["entities"][0]["name"] = json!("latest joined human name");
    candidate["scene"]["caption"] = json!("latest ordinary caption");
    radius.reading.scenes[0].parameter_candidate = Some(candidate.clone());
    strength.reading.scenes[0].parameter_candidate = Some(candidate);
    [radius, strength]
        .into_iter()
        .zip([json!(300), json!(0.65)])
        .map(|(input, value)| NativeActiveControlRefresh {
            procedure: input.procedure,
            reading: input.reading,
            candidate_native_parameter: json!({"value":value}),
            actor_ref: "human:ordinary-multitarget-edit".into(),
            operation_ref: "operation:normal-radius-strength-name".into(),
        })
        .collect()
}

#[test]
fn joined_active_refresh_preserves_both_controls_parent_siblings_and_either_release_order() {
    let requests = joined_gesture_inputs();
    let joined = refresh_native_active_controls(&requests).unwrap();
    assert_eq!(joined["schema"], "ql.procedural-active-controls-refresh/v1");
    assert_eq!(
        joined["managed_contribution_refs"],
        json!(["contribution:joined-whole-scene"])
    );
    assert_eq!(
        joined["managed_control_addresses"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        joined["scene_coverage"][0]["managed_control_addresses"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    let changes = joined["native_edit"]["changes"].as_array().unwrap();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0]["change"], "scene_material_set");
    let material = &changes[0]["presentation"];
    let mut candidate = requests[0].reading.scenes[0]
        .parameter_candidate
        .clone()
        .unwrap();
    candidate["scene"]
        .as_object_mut()
        .unwrap()
        .remove("procedural");
    let mut actual = material.clone();
    actual["scene"]
        .as_object_mut()
        .unwrap()
        .remove("procedural");
    assert_eq!(
        actual, candidate,
        "full ordinary material candidate is exact outside metadata"
    );
    let controls = material["scene"]["procedural"]["controls"]
        .as_array()
        .unwrap();
    assert_eq!(
        controls
            .iter()
            .find(|c| c["parameter"] == "force_radius")
            .unwrap()["takeover"]["native_value"],
        300
    );
    assert_eq!(
        controls
            .iter()
            .find(|c| c["parameter"] == "force_strength")
            .unwrap()["takeover"]["native_value"],
        0.65
    );
    for control in controls {
        assert_eq!(
            control["procedure_ref"],
            requests[0].procedure.procedure_ref
        );
    }
    for order in [[0, 1], [1, 0]] {
        let mut current = material.clone();
        for (offset, index) in order.into_iter().enumerate() {
            let original = &requests[index];
            let mut reading = original.reading.clone();
            reading.document_revision = 11 + offset as u64;
            reading.native_parameter = original.candidate_native_parameter.clone();
            reading.scenes[0].presentation = current;
            reading.scenes[0].parameter_candidate = None;
            let released = prepare_native_control(&NativeControlInput {
                procedure: original.procedure.clone(),
                reading,
                actor_ref: "human:owner".into(),
                operation_ref: format!("operation:release-joined-{index}"),
                action: NativeControlAction::Release {},
            })
            .unwrap();
            current = released["native_edit"]["changes"][0]["presentation"].clone();
            // Scalar projection is the actual existing native Document owner's
            // operation, separate from Source metadata. Here keep the next
            // supplied native read consistent with its original accepted preview.
            let (key, value) = if index == 0 {
                ("radius", json!(0.3))
            } else {
                ("strength", json!(0.3))
            };
            current["scene"]["entities"][0]["force"][key] = value;
        }
        assert!(
            current["scene"]["procedural"]["controls"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        let rows = current["scene"]["procedural"]["contributions"][0]["authored_overrides"]
            .as_array()
            .unwrap();
        let parent = rows
            .iter()
            .find(|r| r["address"]["component"] == "entity" && r["address"]["property"].is_null())
            .unwrap();
        assert_eq!(parent["value"]["name"], "latest joined human name");
        assert_eq!(parent["value"]["force"]["radius"], 0.3);
        assert_eq!(parent["value"]["force"]["strength"], 0.3);
        assert!(rows.iter().any(|r| r["path"] == "/scene/caption"
            && r["value"] == "latest ordinary caption"
            && r["persistent"] == true));
    }
}

#[test]
fn joined_active_refresh_refuses_missing_conflicting_duplicate_and_wrong_owner_inputs() {
    let requests = joined_gesture_inputs();
    assert!(
        refresh_native_active_controls(&requests[..1])
            .unwrap_err()
            .contains("omits another")
    );
    let mut wrong = requests.clone();
    wrong[1] = wrong[0].clone();
    assert!(
        refresh_native_active_controls(&wrong)
            .unwrap_err()
            .contains("duplicate")
    );
    let mut wrong = requests.clone();
    wrong[1].reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["caption"] = json!("different full candidate");
    assert!(
        refresh_native_active_controls(&wrong)
            .unwrap_err()
            .contains("disagree")
    );
    let mut wrong = requests.clone();
    wrong[1].operation_ref = "operation:foreign".into();
    assert!(refresh_native_active_controls(&wrong).is_err());
    let mut wrong = requests.clone();
    wrong[0].reading.scenes[0].presentation["scene"]["procedural"]["controls"][0]["procedure_ref"] =
        json!("procedure:foreign-owner");
    wrong[0].reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        wrong[0].reading.scenes[0].presentation["scene"]["procedural"].clone();
    assert!(refresh_native_active_controls(&wrong).is_err());
}

#[test]
fn native_control_legacy_owner_is_explicit_unique_and_parameter_identity_is_retained() {
    let original = input();
    let taken = prepare_native_control(&original).unwrap();
    let mut release = original.clone();
    release.action = NativeControlAction::Release {};
    release.reading.document_revision = 9;
    release.reading.native_parameter = json!({"value":640});
    release.reading.scenes[0].presentation =
        taken["native_edit"]["changes"][0]["presentation"].clone();
    release.reading.scenes[0].parameter_candidate = None;
    let control = &mut release.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0];
    assert_eq!(control["parameter"], "force_radius");
    control.as_object_mut().unwrap().remove("procedure_ref");
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("exactly one")
    );
    release.reading.qualified_legacy_procedures = vec![release.procedure.clone()];
    assert!(prepare_native_control(&release).is_ok());
    release
        .reading
        .qualified_legacy_procedures
        .push(release.procedure.clone());
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("exactly one")
    );
    release.reading.qualified_legacy_procedures.truncate(1);
    release.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0]["parameter"] =
        json!("force_strength");
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("parameter differs")
    );
}

#[test]
fn typed_intervention_batch_accepts_joined_action_and_refuses_empty_nested_actions() {
    use ql_mef::procedural_intervention::{InterventionPreflight, intervention_batch};
    let inputs = joined_gesture_inputs();
    let batch = intervention_batch(&[InterventionPreflight::ActiveControlsRefresh {
        inputs: inputs.clone(),
    }])
    .unwrap();
    assert_eq!(
        batch["results"][0]["schema"],
        "ql.procedural-active-controls-refresh/v1"
    );
    assert_eq!(
        batch["results"][0],
        refresh_native_active_controls(&inputs).unwrap()
    );
    assert!(
        intervention_batch(&[InterventionPreflight::ActiveControlsRefresh { inputs: vec![] }])
            .is_err()
    );
    assert!(
        intervention_batch(&[
            InterventionPreflight::ActiveControlsRefresh {
                inputs: inputs.clone()
            },
            InterventionPreflight::ActiveControlsRefresh {
                inputs: inputs.clone()
            },
        ])
        .unwrap_err()
        .contains("one authoritative")
    );
    assert!(
        serde_json::from_value::<InterventionPreflight>(
            json!({"action":"active_controls_refresh","inputs":inputs,"control":{"action":"event"}})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<InterventionPreflight>(json!({"action":"install","inputs":[]}))
            .is_err()
    );
}

// The Field record is emitted by the existing actual TS Registry/takeOver. It
// remains authored configuration, not a private native Source or ACK grant.
// The Source scalar operation must preserve this disjoint current record.
#[test]
fn native_scalar_control_preserves_existing_actual_field_takeover_configuration() {
    let artifact: Value =
        serde_json::from_str(include_str!("existing-field-control-source.json")).unwrap();
    assert_eq!(
        artifact["schema"],
        "ta-onta-existing-field-control-source-fixture/v1"
    );
    let field_control = artifact["control"].clone();
    assert_eq!(field_control["address"]["component"], "field");
    assert!(field_control["address"]["entity_ref"].is_null());
    assert_eq!(field_control["target"], "field.opacity");
    let mut request = input();
    request.reading.scenes[0].presentation["scene"]["field"] = artifact["field_after"].clone();
    request.reading.scenes[0].presentation["scene"]["procedural"]["controls"] =
        json!([field_control.clone()]);
    request.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["field"] = artifact["field_after"].clone();
    request.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        request.reading.scenes[0].presentation["scene"]["procedural"].clone();
    let before = request.reading.scenes[0].presentation.clone();
    let taken = prepare_native_control(&request)
        .expect("a disjoint authored Field control must not prevent the native scalar takeover");
    assert_eq!(
        request.reading.scenes[0].presentation, before,
        "pure Source preflight leaves original native reading intact"
    );
    let material = &taken["native_edit"]["changes"][0]["presentation"];
    assert_eq!(material["scene"]["field"], artifact["field_after"]);
    assert!(
        material["scene"]["procedural"]["controls"]
            .as_array()
            .unwrap()
            .contains(&field_control),
        "existing Field control bytes must survive the unrelated scalar operation"
    );
    let mut release = request.clone();
    release.reading.document_revision += 1;
    release.reading.native_parameter = json!({"value":640});
    release.reading.scenes[0].presentation = material.clone();
    release.reading.scenes[0].parameter_candidate = None;
    release.action = NativeControlAction::Release {};
    let released = prepare_native_control(&release).expect(
        "unrelated native scalar release must remain reachable with the original Field control",
    );
    let after = &released["native_edit"]["changes"][0]["presentation"];
    assert_eq!(after["scene"]["field"], artifact["field_after"]);
    assert_eq!(
        after["scene"]["procedural"]["controls"],
        json!([field_control])
    );
}

#[test]
fn joined_native_controls_preserve_actual_field_record_and_refuse_lost_native_baseline() {
    let artifact: Value =
        serde_json::from_str(include_str!("existing-field-control-source.json")).unwrap();
    let field = artifact["control"].clone();
    let mut inputs = joined_gesture_inputs();
    for input in &mut inputs {
        let scene = &mut input.reading.scenes[0];
        scene.presentation["scene"]["field"] = artifact["field_after"].clone();
        scene.presentation["scene"]["procedural"]["controls"]
            .as_array_mut()
            .unwrap()
            .push(field.clone());
        let candidate = scene.parameter_candidate.as_mut().unwrap();
        candidate["scene"]["field"] = artifact["field_after"].clone();
        candidate["scene"]["procedural"] = scene.presentation["scene"]["procedural"].clone();
    }
    let joined = refresh_native_active_controls(&inputs).unwrap();
    assert!(
        joined["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["controls"]
            .as_array()
            .unwrap()
            .contains(&field)
    );
    assert!(
        joined["managed_control_addresses"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| a["component"] != "field")
    );
    let mut wrong = inputs;
    for input in &mut wrong {
        input.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0]
            .as_object_mut()
            .unwrap()
            .remove("native_base");
        input.reading.scenes[0]
            .parameter_candidate
            .as_mut()
            .unwrap()["scene"]["procedural"] =
            input.reading.scenes[0].presentation["scene"]["procedural"].clone();
    }
    assert!(
        refresh_native_active_controls(&wrong).is_err(),
        "losing actual native baseline must not become authored-control permission"
    );
}

fn actual_flow_contribution(request: &NativeControlInput) -> Value {
    use ql_mef::procedural_intervention::{FlowManualEdit, extract_flow_interventions};
    let expression = request.reading.expression_ref.clone();
    let canonical = request.reading.scenes[0].scene_ref.clone();
    let passage = format!("{expression}:scene:passage");
    let owned = OwnedAddress {
        expression_ref: expression.clone(),
        scene_ref: None,
        entity_ref: None,
        component: "expression".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    let subject = request.procedure.principal_subject_ref.clone();
    let reference = contribution_identity(
        &request.procedure.procedure_ref,
        "native-flow",
        std::slice::from_ref(&subject),
        &expression,
    )
    .unwrap();
    let before = json!({"schema":"ql.native-atlas-state/v1","expression_ref":expression,
        "focus":{"scene_ref":canonical,"entity_ref":null},"scene_order":[canonical,passage]});
    let after = json!({"schema":"ql.native-atlas-state/v1","expression_ref":expression,
        "focus":{"scene_ref":passage,"entity_ref":null},"scene_order":[passage,canonical]});
    let rows = extract_flow_interventions(&FlowManualEdit {
        expression_ref: expression.clone(),
        contribution_ref: reference.clone(),
        owned_addresses: vec![owned.clone()],
        before: before.clone(),
        after,
        actor_ref: "human:native-flow-author".into(),
        operation_ref: "operation:actual-native-focus-order".into(),
        document_revision: 7,
        retained_native_records: vec![],
    })
    .unwrap()
    .native_records;
    let flow = vec![
        NativeChange::Focus {
            scene_ref: passage.clone(),
            entity_ref: None,
        },
        NativeChange::SceneReorder {
            scene_refs: vec![passage, canonical],
        },
    ];
    // Exact native Atlas program basis/retained contribution model. Its human
    // rows come from the actual flow extractor, never the Scene extractor or a
    // caller-created Source/clock/receiving receipt.
    json!({"contribution_ref":reference,"procedure_ref":request.procedure.procedure_ref,"output_slot":"native-flow",
        "subject_refs":[subject],"occurrence_ref":expression,"recipe_revision":request.procedure.recipe.revision,
        "owned_addresses":[owned],"generated_basis":{"native_flow":flow,"authored_basis":before},
        "authored_overrides":rows,"status":"active"})
}

#[test]
fn native_scalar_takeover_release_and_join_preserve_actual_atlas_flow_owner_rows() {
    let mut request = input();
    let flow = actual_flow_contribution(&request);
    request.reading.scenes[0].presentation["scene"]["procedural"]["contributions"] =
        json!([flow.clone()]);
    request.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        request.reading.scenes[0].presentation["scene"]["procedural"].clone();
    let taken = prepare_native_control(&request).unwrap();
    let material = &taken["native_edit"]["changes"][0]["presentation"];
    assert_eq!(
        material["scene"]["procedural"]["contributions"],
        json!([flow.clone()])
    );
    assert!(
        material["scene"]["procedural"]["controls"][0]["dormant_overrides"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mut release = request.clone();
    release.reading.document_revision = 9;
    release.reading.native_parameter = json!({"value":640});
    release.reading.scenes[0].presentation = material.clone();
    release.reading.scenes[0].parameter_candidate = None;
    release.action = NativeControlAction::Release {};
    let released = prepare_native_control(&release).unwrap();
    assert_eq!(
        released["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["contributions"],
        json!([flow.clone()])
    );
    let mut joined = joined_gesture_inputs();
    for input in &mut joined {
        let scene = &mut input.reading.scenes[0];
        scene.presentation["scene"]["procedural"]["contributions"]
            .as_array_mut()
            .unwrap()
            .push(flow.clone());
        scene.parameter_candidate.as_mut().unwrap()["scene"]["procedural"] =
            scene.presentation["scene"]["procedural"].clone();
    }
    let result = refresh_native_active_controls(&joined).unwrap();
    assert!(
        !result["managed_contribution_refs"]
            .as_array()
            .unwrap()
            .contains(&flow["contribution_ref"])
    );
    assert!(
        result["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["contributions"]
            .as_array()
            .unwrap()
            .contains(&flow)
    );
    assert!(
        result["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["controls"]
            .as_array()
            .unwrap()
            .iter()
            .all(|c| c["dormant_overrides"]
                .as_array()
                .unwrap()
                .iter()
                .all(|d| d["contribution_ref"] != flow["contribution_ref"]))
    );
    let mut foreign = request;
    foreign.reading.scenes[0].presentation["scene"]["procedural"]["contributions"][0]["generated_basis"]
        ["native_flow"] = json!([{
        "change":"parameter_set","entity_ref":foreign.reading.entity_ref,"parameter":"force_radius","value":400}]);
    foreign.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        foreign.reading.scenes[0].presentation["scene"]["procedural"].clone();
    assert!(
        prepare_native_control(&foreign)
            .unwrap_err()
            .contains("foreign material operation")
    );
}

#[test]
fn joined_native_control_coverage_preserves_detached_and_retired_material_contributions() {
    let mut inputs = joined_gesture_inputs();
    let original =
        inputs[0].reading.scenes[0].presentation["scene"]["procedural"]["contributions"][0].clone();
    let mut inactive = Vec::new();
    for status in ["detached", "retired"] {
        let mut row = original.clone();
        row["status"] = json!(status);
        row["contribution_ref"] = json!(format!("contribution:existing-{status}-whole-scene"));
        inactive.push(row);
    }
    for input in &mut inputs {
        let scene = &mut input.reading.scenes[0];
        scene.presentation["scene"]["procedural"]["contributions"]
            .as_array_mut()
            .unwrap()
            .extend(inactive.clone());
        scene.parameter_candidate.as_mut().unwrap()["scene"]["procedural"] =
            scene.presentation["scene"]["procedural"].clone();
    }
    let result = refresh_native_active_controls(&inputs).unwrap();
    assert_eq!(
        result["managed_contribution_refs"],
        json!(["contribution:joined-whole-scene"])
    );
    for row in inactive {
        assert!(result["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["contributions"].as_array().unwrap().contains(&row),"inactive retained native material/identity/human records remain unchanged");
        assert!(
            !result["managed_contribution_refs"]
                .as_array()
                .unwrap()
                .contains(&row["contribution_ref"])
        );
    }
}

/// The SAME production Source function constructs both takeovers. This is a
/// pure Source configuration case; supplied native Parameter/preview reads do
/// not stand for an applied Document, native field, body or audio receipt.
fn synchronized_group_takeovers() -> (Vec<NativeControlInput>, Value, Value) {
    synchronized_group_takeovers_with_owner(false)
}
fn synchronized_group_takeovers_with_owner(
    foreign_owner: bool,
) -> (Vec<NativeControlInput>, Value, Value) {
    let mut radius = input();
    radius.action = NativeControlAction::Takeover {
        value: json!(640),
        lifetime: ControlLifetime::Gesture,
    };
    radius.reading.scenes[0].presentation["scene"]["entities"][0]["force"]["strength"] = json!(0.3);
    radius.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["entities"][0]["force"]["strength"] = json!(0.3);
    let taken = prepare_native_control(&radius).unwrap();
    let promoted =
        taken["native_edit"]["changes"][0]["presentation"]["scene"]["automation"].clone();
    let mut current = taken["native_edit"]["changes"][0]["presentation"].clone();
    current["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    let mut strength = radius.clone();
    strength.operation_ref = "operation:takeover-synchronized-strength".into();
    strength.reading.document_revision = 9;
    strength.reading.parameter = "force_strength".into();
    strength.reading.native_parameter =
        json!({"value":0.3,"automation":{"min":0.2,"max":0.8,"rate_hz":0.75,"waveform":"sine"}});
    strength.reading.addresses = vec![
        native_parameter_address(
            &strength.reading.expression_ref,
            &strength.reading.scenes[0].scene_ref,
            &strength.reading.entity_ref,
            "force_strength",
        )
        .unwrap(),
    ];
    strength.reading.scenes[0].presentation = current.clone();
    current["scene"]["entities"][0]["force"]["strength"] = json!(0.8);
    strength.reading.scenes[0].parameter_candidate = Some(current);
    strength.action = NativeControlAction::Takeover {
        value: json!(0.8),
        lifetime: ControlLifetime::Gesture,
    };
    if foreign_owner {
        strength.procedure.procedure_ref = "procedure:synchronized-strength-owner".into();
        strength.reading.qualified_control_peers = vec![NativeQualifiedControlPeer {
            address: radius.reading.addresses[0].clone(),
            parameter: radius.reading.parameter.clone(),
            procedure: radius.procedure.clone(),
        }];
    }
    let taken = prepare_native_control(&strength).unwrap();
    let mut current = taken["native_edit"]["changes"][0]["presentation"].clone();
    current["scene"]["entities"][0]["force"]["strength"] = json!(0.8);
    (vec![radius, strength], current, promoted)
}

fn release_synchronized_order(
    inputs: &[NativeControlInput],
    material: Value,
    order: [usize; 2],
) -> Value {
    let mut current = material;
    for (offset, index) in order.into_iter().enumerate() {
        let mut request = inputs[index].clone();
        request.action = NativeControlAction::ReleaseGesture {
            takeover_operation_ref: inputs[index].operation_ref.clone(),
        };
        request.operation_ref = format!("operation:release-synchronized-{index}");
        request.reading.document_revision = 10 + offset as u64;
        request.reading.native_parameter = if index == 0 {
            json!({"value":640})
        } else {
            json!({"value":0.8})
        };
        request.reading.scenes[0].presentation = current;
        request.reading.qualified_control_peers =
            request.reading.scenes[0].presentation["scene"]["procedural"]["controls"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|control| {
                    control["procedure_ref"] != request.procedure.procedure_ref
                        && control.get("native_base").is_some()
                })
                .map(|control| {
                    let original = inputs
                        .iter()
                        .find(|input| {
                            control["procedure_ref"] == input.procedure.procedure_ref
                                && control["parameter"] == input.reading.parameter
                        })
                        .unwrap();
                    NativeQualifiedControlPeer {
                        address: serde_json::from_value(control["address"].clone()).unwrap(),
                        parameter: original.reading.parameter.clone(),
                        procedure: original.procedure.clone(),
                    }
                })
                .collect();
        request.reading.scenes[0].parameter_candidate = None;
        let released = prepare_native_control(&request).unwrap();
        assert_eq!(released["resolved_scope"], json!(request.reading.addresses));
        let mut expected_metadata_scope = request.reading.addresses.clone();
        if offset == 0 {
            let peer_address = &inputs[1 - index].reading.addresses[0];
            let before_peer =
                request.reading.scenes[0].presentation["scene"]["procedural"]["controls"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|control| control["address"] == json!(peer_address))
                    .unwrap();
            let after_peer = released["native_edit"]["changes"][0]["presentation"]["scene"]
                ["procedural"]["controls"]
                .as_array()
                .unwrap()
                .iter()
                .find(|control| control["address"] == json!(peer_address))
                .unwrap();
            if before_peer != after_peer {
                expected_metadata_scope.push(peer_address.clone());
            } else {
                // A retained promoted Source group can already have the exact
                // remaining driver's full baseline and suspension. Removing
                // its sibling does not claim unchanged peer metadata.
                assert_eq!(
                    after_peer, before_peer,
                    "unchanged peer retains all source/clock bytes"
                );
                assert!(
                    !released["managed_control_addresses"]
                        .as_array()
                        .unwrap()
                        .contains(&json!(peer_address)),
                    "metadata coverage must not widen to an unchanged peer"
                );
            }
        }
        expected_metadata_scope.sort();
        assert_eq!(
            released["managed_control_addresses"],
            json!(expected_metadata_scope)
        );
        assert_eq!(
            released["scene_coverage"][0]["managed_control_addresses"],
            released["managed_control_addresses"]
        );
        assert_eq!(
            released["native_edit"]["changes"][2]["value"],
            inputs[index].reading.native_parameter["value"]
        );
        assert_eq!(
            released["native_edit"]["changes"][3]["automation"],
            inputs[index].reading.native_parameter["automation"]
        );
        current = released["native_edit"]["changes"][0]["presentation"].clone();
        let key = if index == 0 { "radius" } else { "strength" };
        current["scene"]["entities"][0]["force"][key] = json!(0.3);
        if offset == 0 {
            let remaining = &current["scene"]["procedural"]["controls"][0];
            assert!(
                !current["scene"]["automation"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|lane| lane["target"] == remaining["target"])
            );
            assert_eq!(
                remaining["dormant_lanes"],
                inputs[0].reading.scenes[0].presentation["scene"]["automation"]
            );
            assert_eq!(
                remaining["takeover"]["native_value"],
                if index == 0 { json!(0.8) } else { json!(640) }
            );
        }
    }
    current
}

#[test]
fn synchronized_native_takeovers_restore_original_group_clock_in_both_release_orders() {
    let (inputs, current, _) = synchronized_group_takeovers();
    assert!(
        current["scene"]["automation"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    for control in current["scene"]["procedural"]["controls"]
        .as_array()
        .unwrap()
    {
        assert_eq!(
            control["dormant_lanes"],
            inputs[0].reading.scenes[0].presentation["scene"]["automation"]
        );
        assert!(control["suspended_lanes"].as_array().unwrap().is_empty());
    }
    for order in [[0, 1], [1, 0]] {
        let released = release_synchronized_order(&inputs, current.clone(), order);
        assert_eq!(
            released["scene"]["automation"],
            inputs[0].reading.scenes[0].presentation["scene"]["automation"]
        );
        assert_eq!(
            released["scene"]["propertyTracks"],
            inputs[0].reading.scenes[0].presentation["scene"]["propertyTracks"]
        );
        assert!(
            released["scene"]["procedural"]["controls"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
}

#[test]
fn retained_promoted_group_from_prior_source_restores_without_losing_earlier_clock() {
    let (inputs, mut current, promoted) = synchronized_group_takeovers();
    // Exact prior Source12 retained shape: second capture was the promoted
    // child; first capture still expected that child before its later removal.
    current["scene"]["procedural"]["controls"][0]["suspended_lanes"] = promoted.clone();
    current["scene"]["procedural"]["controls"][1]["dormant_lanes"] = promoted;
    for order in [[0, 1], [1, 0]] {
        let released = release_synchronized_order(&inputs, current.clone(), order);
        assert_eq!(
            released["scene"]["automation"],
            inputs[0].reading.scenes[0].presentation["scene"]["automation"]
        );
    }
}

#[test]
fn synchronized_takeover_refuses_changed_or_reintroduced_group_instead_of_refreshing_its_clock() {
    let (inputs, current, _) = synchronized_group_takeovers();
    let mut release = inputs[0].clone();
    release.action = NativeControlAction::Release {};
    release.reading.document_revision = 10;
    release.reading.native_parameter = json!({"value":640});
    release.reading.scenes[0].parameter_candidate = None;
    release.reading.scenes[0].presentation = current.clone();
    let mut changed = inputs[0].reading.scenes[0].presentation["scene"]["automation"][0].clone();
    changed["rate"] = json!(1.25);
    release.reading.scenes[0].presentation["scene"]["automation"] = json!([changed]);
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("group changed")
    );
    release.reading.scenes[0].presentation = current;
    release.reading.scenes[0].presentation["scene"]["procedural"]["controls"][1]["dormant_lanes"]
        [0]["phase"] = json!(0.9);
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("contradictory source")
    );
}

#[test]
fn shared_driver_group_requires_exact_foreign_peer_original_and_preserves_authored_field_controls()
{
    let (inputs, current, _) = synchronized_group_takeovers_with_owner(true);
    for order in [[0, 1], [1, 0]] {
        let released = release_synchronized_order(&inputs, current.clone(), order);
        assert_eq!(
            released["scene"]["automation"],
            inputs[0].reading.scenes[0].presentation["scene"]["automation"]
        );
    }
    let mut request = inputs[0].clone();
    request.action = NativeControlAction::Release {};
    request.reading.document_revision = 10;
    request.reading.native_parameter = json!({"value":640});
    request.reading.scenes[0].presentation = current;
    request.reading.scenes[0].parameter_candidate = None;
    assert!(
        prepare_native_control(&request)
            .unwrap_err()
            .contains("exact qualified original")
    );
    let peer = NativeQualifiedControlPeer {
        address: inputs[1].reading.addresses[0].clone(),
        parameter: inputs[1].reading.parameter.clone(),
        procedure: inputs[1].procedure.clone(),
    };
    request.reading.qualified_control_peers = vec![peer.clone()];
    let fixture: Value =
        serde_json::from_str(include_str!("existing-field-control-source.json")).unwrap();
    let field = fixture["control"].clone();
    request.reading.scenes[0].presentation["scene"]["procedural"]["controls"]
        .as_array_mut()
        .unwrap()
        .push(field.clone());
    let prepared = prepare_native_control(&request).unwrap();
    assert!(
        prepared["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["controls"]
            .as_array()
            .unwrap()
            .contains(&field)
    );
    assert!(
        !prepared["managed_control_addresses"]
            .as_array()
            .unwrap()
            .contains(&field["address"])
    );
    request.reading.qualified_control_peers.push(peer.clone());
    assert!(
        prepare_native_control(&request)
            .unwrap_err()
            .contains("duplicated")
    );
    request.reading.qualified_control_peers = vec![peer];
    request.reading.qualified_control_peers[0].parameter = "force_spin".into();
    assert!(
        prepare_native_control(&request)
            .unwrap_err()
            .contains("identity differs")
    );
}

// Append after Source13's real synchronized_group_takeovers helpers.
// This is an actual pure Source-function test; no native/physical/ACK grant.
#[test]
fn promoted_dormant_subset_changed_after_real_refresh_refuses_original_group_restoration() {
    let (inputs, mut prior, promoted) = synchronized_group_takeovers();
    // Source12 retained this exact promoted child, which was emitted by the
    // real first takeover. Preserve its actual legacy group witness.
    prior["scene"]["procedural"]["controls"][0]["suspended_lanes"] = promoted.clone();
    prior["scene"]["procedural"]["controls"][1]["dormant_lanes"] = promoted.clone();

    // Real Source13 refresh updates current suspended membership. It must
    // retain the original dormant clock through subsequent release.
    let mut refresh = inputs[1].clone();
    refresh.operation_ref = "operation:refresh-retained-promoted-strength".into();
    refresh.reading.document_revision = 10;
    refresh.reading.native_parameter = json!({"value": 0.8});
    refresh.reading.scenes[0].presentation = prior.clone();
    refresh.reading.scenes[0].parameter_candidate = Some(prior);
    let refreshed = prepare_native_control(&refresh).unwrap();
    let current = refreshed["native_edit"]["changes"][0]["presentation"].clone();
    assert_eq!(
        current["scene"]["procedural"]["controls"][1]["dormant_lanes"],
        promoted
    );
    for control in current["scene"]["procedural"]["controls"]
        .as_array()
        .unwrap()
    {
        assert!(control["suspended_lanes"].as_array().unwrap().is_empty());
    }

    let mut release = inputs[0].clone();
    release.operation_ref = "operation:release-promoted-subset-origin".into();
    release.action = NativeControlAction::ReleaseGesture {
        takeover_operation_ref: inputs[0].operation_ref.clone(),
    };
    release.reading.document_revision = 11;
    release.reading.native_parameter = json!({"value": 640});
    release.reading.scenes[0].presentation = current;
    release.reading.scenes[0].parameter_candidate = None;
    assert!(
        prepare_native_control(&release).is_ok(),
        "unchanged genuine promoted subset must remain releasable"
    );

    for (key, altered) in [
        ("rate", json!(1.25)),
        ("phase", json!(0.9)),
        ("clockId", json!("foreign-clock")),
    ] {
        let mut wrong = release.clone();
        let row = &mut wrong.reading.scenes[0].presentation["scene"]["procedural"]["controls"][1]["dormant_lanes"]
            [0];
        assert_ne!(
            row[key], altered,
            "negative must change the real retained clock field"
        );
        row[key] = altered;
        let unchanged = serde_json::to_value(&wrong).unwrap();
        assert!(
            prepare_native_control(&wrong).is_err(),
            "changed promoted subset {key} was discarded while selecting a larger group snapshot"
        );
        assert_eq!(serde_json::to_value(&wrong).unwrap(), unchanged);
    }
}

/// Real Source preparation of an existing takeover must not retarget the
/// dormant native driver while retaining the original address/Parameter.
#[test]
fn native_existing_takeover_refuses_wrong_retained_registry_target() {
    let initial = input();
    let prepared = prepare_native_control(&initial).unwrap();
    let mut current = prepared["native_edit"]["changes"][0]["presentation"].clone();
    current["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    let mut refresh = initial.clone();
    refresh.operation_ref = "operation:refresh-wrong-native-target".into();
    refresh.reading.document_revision = 9;
    refresh.reading.native_parameter = json!({"value":640});
    refresh.reading.scenes[0].presentation = current.clone();
    refresh.reading.scenes[0].parameter_candidate = Some(current);
    assert!(prepare_native_control(&refresh).is_ok());
    refresh.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0]["target"] =
        json!("entity:foreign:forces.radius");
    refresh.reading.scenes[0]
        .parameter_candidate
        .as_mut()
        .unwrap()["scene"]["procedural"] =
        refresh.reading.scenes[0].presentation["scene"]["procedural"].clone();
    let unchanged = serde_json::to_value(&refresh).unwrap();
    assert!(
        prepare_native_control(&refresh)
            .unwrap_err()
            .contains("Registry coordinate")
    );
    assert_eq!(serde_json::to_value(&refresh).unwrap(), unchanged);
}
#[test]
fn native_sibling_control_refuses_wrong_retained_registry_target() {
    let (inputs, current, _) = synchronized_group_takeovers();
    let mut release = inputs[1].clone();
    release.operation_ref = "operation:release-sibling-wrong-native-target".into();
    release.action = NativeControlAction::ReleaseGesture {
        takeover_operation_ref: inputs[1].operation_ref.clone(),
    };
    release.reading.document_revision = 10;
    release.reading.native_parameter = json!({"value":0.8});
    release.reading.scenes[0].presentation = current;
    release.reading.scenes[0].parameter_candidate = None;
    assert!(prepare_native_control(&release).is_ok());
    release.reading.scenes[0].presentation["scene"]["procedural"]["controls"][0]["target"] =
        json!("entity:foreign:forces.radius");
    let unchanged = serde_json::to_value(&release).unwrap();
    assert!(
        prepare_native_control(&release)
            .unwrap_err()
            .contains("Registry coordinate")
    );
    assert_eq!(serde_json::to_value(&release).unwrap(), unchanged);
}

// Append to the actual Source control tests after their existing input helper.
// Pure real prepare_native_control sequence; no native/physical/ACK grant.
#[test]
fn genuine_disabled_child_then_leader_takeovers_keep_actual_promoted_suspension_order() {
    let mut initial = input();
    let base = &mut initial.reading.scenes[0].presentation;
    base["scene"]["entities"][0]["force"]["strength"] = json!(0.3);
    base["scene"]["entities"][0]["force"]["spin"] = json!(0.1);
    let root_id = base["scene"]["automation"][0]["id"].clone();
    base["scene"]["automation"][0]["enabled"] = json!(true);
    base["scene"]["automation"][1]["enabled"] = json!(false);
    let mut spin_lane = base["scene"]["automation"][1].clone();
    spin_lane["id"] = json!("driver:third-spin");
    spin_lane["target"] = json!(format!(
        "entity:{}:forces.spin",
        initial.reading.entity_ref.replace(':', "%3A")
    ));
    spin_lane["enabled"] = json!(true);
    spin_lane["syncWith"] = root_id;
    base["scene"]["automation"]
        .as_array_mut()
        .unwrap()
        .push(spin_lane);
    let original_group = base["scene"]["automation"].clone();

    let mut child = initial.clone();
    child.operation_ref = "operation:disabled-child-first".into();
    child.reading.parameter = "force_strength".into();
    child.reading.native_parameter = json!({"value":0.3});
    child.reading.addresses = vec![
        native_parameter_address(
            &child.reading.expression_ref,
            &child.reading.scenes[0].scene_ref,
            &child.reading.entity_ref,
            "force_strength",
        )
        .unwrap(),
    ];
    let mut preview = child.reading.scenes[0].presentation.clone();
    preview["scene"]["entities"][0]["force"]["strength"] = json!(0.8);
    child.reading.scenes[0].parameter_candidate = Some(preview);
    child.action = NativeControlAction::Takeover {
        value: json!(0.8),
        lifetime: ControlLifetime::Gesture,
    };
    let prepared_child = prepare_native_control(&child).unwrap();
    let mut current = prepared_child["native_edit"]["changes"][0]["presentation"].clone();
    current["scene"]["entities"][0]["force"]["strength"] = json!(0.8);

    let mut root = initial.clone();
    root.operation_ref = "operation:enabled-root-second".into();
    root.reading.document_revision += 1;
    root.reading.scenes[0].presentation = current.clone();
    current["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    root.reading.scenes[0].parameter_candidate = Some(current);
    root.action = NativeControlAction::Takeover {
        value: json!(640),
        lifetime: ControlLifetime::Gesture,
    };
    let prepared_root = prepare_native_control(&root).unwrap();
    let mut current = prepared_root["native_edit"]["changes"][0]["presentation"].clone();
    current["scene"]["entities"][0]["force"]["radius"] = json!(1.6);
    assert_eq!(current["scene"]["automation"].as_array().unwrap().len(), 1);
    assert_eq!(current["scene"]["automation"][0]["id"], "driver:third-spin");
    assert_eq!(current["scene"]["automation"][0]["enabled"], true);

    let mut third = initial;
    third.operation_ref = "operation:promoted-spin-third".into();
    third.reading.document_revision += 2;
    third.reading.parameter = "force_spin".into();
    third.reading.native_parameter = json!({"value":0.1});
    third.reading.addresses = vec![
        native_parameter_address(
            &third.reading.expression_ref,
            &third.reading.scenes[0].scene_ref,
            &third.reading.entity_ref,
            "force_spin",
        )
        .unwrap(),
    ];
    third.reading.scenes[0].presentation = current.clone();
    current["scene"]["entities"][0]["force"]["spin"] = json!(0.2);
    third.reading.scenes[0].parameter_candidate = Some(current);
    third.action = NativeControlAction::Takeover {
        value: json!(0.2),
        lifetime: ControlLifetime::Gesture,
    };
    let before = serde_json::to_value(&third).unwrap();
    let prepared_third = prepare_native_control(&third);
    assert!(
        prepared_third.is_ok(),
        "genuine child→leader native suspension was incorrectly replayed in lane-array order: {:?}",
        prepared_third.err()
    );
    let prepared_third = prepare_native_control(&third).unwrap();
    let controls = prepared_third["native_edit"]["changes"][0]["presentation"]["scene"]["procedural"]["controls"].as_array().unwrap();
    assert_eq!(controls.len(), 3);
    for control in controls {
        assert_eq!(control["dormant_lanes"], original_group);
        assert!(control["suspended_lanes"].as_array().unwrap().is_empty());
    }
    assert_eq!(serde_json::to_value(&third).unwrap(), before);
}
