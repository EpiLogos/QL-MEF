//! Actual native source/library/conduct and manual intervention tests.
//! No mock graph, source interpreter, receiver, body or clock.
use ql_mef::m_tree;
use ql_mef::procedural_composition::*;
use ql_mef::procedural_conduct::*;
use ql_mef::procedural_intervention::*;
use ql_mef::procedural_manifestation::*;
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

fn presentation() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap()
}
fn position() -> NativePosition {
    NativePosition {
        instance_ref: "native:existing-owner".into(),
        event_ref: "native:actual-event".into(),
        subject_ref: "person:existing-owner".into(),
        generation: "7".into(),
        samples_elapsed: "1024".into(),
    }
}
fn library(choice: LibraryChoice) -> LibraryBuild {
    let mut p = procedure();
    let pos = position();
    p.timing = TimingBinding {
        owner_ref: pos.instance_ref.clone(),
        epoch_ref: pos.instance_ref,
        domain: "native_samples".into(),
        requested_cursor: 1024,
        time_mapping_ref: None,
    };
    let mut source = presentation();
    source["scene"]["id"] = json!("expression:acceptance:scene:canonical");
    LibraryBuild {
        schema: LIBRARY_CONTRACT.into(),
        source: NativeSceneSource {
            native_owner: "oi.expression".into(),
            expression_ref: "expression:acceptance".into(),
            scene_ref: "expression:acceptance:scene:canonical".into(),
            document_revision: 3,
            source_basis: p.profile.clone(),
            material_fingerprint: fingerprint(&source).unwrap(),
            presentation: source,
            principal: subject(),
            contributors: vec![],
            locus_ref: p.locus_ref.clone(),
            locus_revision: "current-native-profile".into(),
        },
        authored: ProcedureAuthorship {
            procedure_ref: p.procedure_ref,
            revision: p.revision,
            composition: p.composition,
            trigger: Trigger {
                kind: TriggerKind::Explicit,
                mode: TriggerMode::Level,
            },
            selector: p.selector,
            conditions: p.conditions,
            recipe_parameters: p.recipe_parameters,
            membership_mode: p.membership_mode,
            membership_change_policy: p.membership_change_policy,
            timing: p.timing,
            budgets: p.budgets,
            seed: p.seed,
            removal_policy: p.removal_policy,
            failure_policy: p.failure_policy,
            continuation_policy: p.continuation_policy,
        },
        choice,
    }
}
fn output() -> Vec<OutputSlot> {
    vec![OutputSlot {
        output_slot: "native-passage".into(),
        scene_ref: "expression:acceptance:scene:passage".into(),
    }]
}
fn definition() -> ConductInstall {
    use ql_mef::vak_scope::OperativeScopeCorrelation;
    use ql_mef::vak_scope_wire::{
        OPERATIVE_CURRENTNESS_CONTRACT, OperativeScopeCurrentnessRequest,
    };
    let built = library_build(library(LibraryChoice::SceneMaterial { outputs: output() })).unwrap();
    let mut p: Procedure = serde_json::from_value(built["procedure"].clone()).unwrap();
    p.composition.thread = ql_mef::vak_profile::ThreadForm::Single;
    p.composition.sources = vec![p.recipe.source_ref.clone(), p.profile.source_ref.clone()];
    let mut whole: Value = serde_json::from_str::<Value>(include_str!(
        "../../../fixtures/kernel/vak-composition-v1.json"
    ))
    .unwrap()["steps"][0]
        .clone();
    whole["useRef"] = json!("whole:procedural-source-ground");
    whole["subjectRef"] = json!(p.principal_subject_ref);
    whole["frame"]["id"] = json!(p.composition.frame.0.code());
    whole["basis"] = json!({"caller":p.composition.actor,"source":p.recipe.source_ref,"revision":p.recipe.revision,"standing":ql_mef::VakStanding::AuthoredArchitecture.as_schema_str(),"evidence":[p.recipe.source_ref]});
    let source = json!({"contract":"ql.vak-composition/v1","steps":[whole,{"op":"reframe","from":"whole:procedural-source-ground","into":p.composition.whole,
      "frame":{"id":p.composition.frame.0.code(),"lens":"L0","basis":"chromatic","face":"direct","positions":"local"},
      "basis":{"caller":p.composition.actor,"source":p.profile.source_ref,"revision":p.profile.revision,"standing":ql_mef::VakStanding::AuthoredArchitecture.as_schema_str(),"evidence":[p.profile.source_ref]}}]});
    let (_, graph) = ql_mef::vak_composition_wire::compile_request(&source).unwrap();
    let correlation = OperativeScopeCorrelation {
        world_ref: "world:acceptance".into(),
        world_generation: "generation:current".into(),
        method_skill_ref: None,
    };
    let binding = graph
        .bind_operative_scope(
            &p.composition.whole,
            p.composition.profile(),
            correlation.clone(),
        )
        .unwrap();
    p.composition.interpretation.reference = binding.binding_ref.clone();
    p.composition.interpretation.revision = binding.binding_revision.clone();
    ConductInstall {schema:CONDUCT_CONTRACT.into(),program:serde_json::from_value(built["program"].clone()).unwrap(),expression_ref:p.occurrence_ref.clone(),document_revision:3,current_readings:vec![],source_composition:source,
      currentness:OperativeScopeCurrentnessRequest{contract:OPERATIVE_CURRENTNESS_CONTRACT.into(),expected:binding,current_whole_ref:p.composition.whole.clone(),correlation},
      thread_plan:serde_json::from_value(json!({"legs":[{"unit_ref":"oi.expression:native-procedure-leg","subject_ref":p.principal_subject_ref,"scope_ref":p.occurrence_ref,"input_refs":[p.recipe.source_ref],"result_ref":"oi.expression:native-procedure-result","after":[],"parent":null}],"aggregation_ref":null,"continuation_ref":null,"stop_condition_ref":null})).unwrap(),procedure:p,required_consumers:BTreeSet::from(["scene".into(),"nativeBody".into(),"audio".into()]),interval_ref:"native:source-interval".into(),materialization:None}
}
fn event(def: &ConductInstall, n: usize) -> ConductEvent {
    ConductEvent {
        procedure_ref: def.procedure.procedure_ref.clone(),
        event: RuleEvent {
            kind: TriggerKind::Explicit,
            active: true,
            cause: Cause {
                event_ref: format!("actual:event:{n}"),
                ancestors: vec![],
                progress_revision: format!("actual:source:{n}"),
                depth: 1,
            },
        },
        position: position(),
        document_revision: 3,
        current_readings: vec![],
        output_readings: vec![],
        current_contributions: vec![],
        intervention_contexts: vec![],
        materialization: None,
        currentness: def.currentness.clone(),
    }
}
fn manual(before: Value, after: Value, c: &GeneratedContribution) -> ManualEdit {
    ManualEdit {
        expression_ref: "expression:acceptance".into(),
        scene_ref: c.occurrence_ref.clone(),
        contribution_ref: c.contribution_ref.clone(),
        owned_addresses: c.owned_addresses.clone(),
        entity_refs: BTreeMap::new(),
        before,
        after,
        actor_ref: "human:owner".into(),
        operation_ref: "oi.expression:actual-human-edit".into(),
        document_revision: 5,
        retained_overlays: vec![],
        retained_native_records: vec![],
    }
}
fn contribution() -> GeneratedContribution {
    let p = procedure();
    instantiate_scene(
        &p,
        "expression:acceptance",
        "native-passage",
        "expression:acceptance:scene:passage",
        &[subject()],
        &presentation(),
    )
    .unwrap()
}

#[test]
fn library_uses_actual_owner_source_and_sequence_consumers_instead_of_fixture_recipe() {
    let input = library(LibraryChoice::SequenceMaterial {
        outputs: output(),
        holds: vec![SequenceHold {
            entity_ref: "source-form".into(),
            step_ref: Some("source-2".into()),
            seconds: 3.5,
        }],
    });
    let original = input.source.clone();
    let built = library_build(input).unwrap();
    let program: NativeRecipeProgram = serde_json::from_value(built["program"].clone()).unwrap();
    let p: Procedure = serde_json::from_value(built["procedure"].clone()).unwrap();
    let membership = resolve_procedure_membership(
        m_tree::native_current_m_registry(),
        &p,
        &p.occurrence_ref,
        &[],
        None,
    )
    .unwrap();
    let c = program_contributions(
        m_tree::native_current_m_registry(),
        &p,
        &program,
        &p.occurrence_ref,
        &[],
        &membership,
    )
    .unwrap();
    assert_eq!(
        c[0].generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["hold"],
        3.5
    );
    assert_eq!(
        c[0].generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"],
        true
    );
    assert_eq!(
        c[0].generated_basis["scene"]["entities"][0]["source"],
        original.presentation["scene"]["entities"][0]["source"]
    );
    assert_eq!(
        c[0].generated_basis["scene"]["entities"][0]["layers"],
        original.presentation["scene"]["entities"][0]["layers"]
    );
    if let NativeRecipeProgram::SceneMaterial { outputs } = program {
        assert_eq!(outputs[0].source, original);
    } else {
        panic!("actual Scene source required")
    }
    let mut stale = library(LibraryChoice::SceneMaterial { outputs: output() });
    stale.source.presentation["scene"]["entities"][0]["text"] = json!("not read");
    assert!(library_build(stale).unwrap_err().contains("exact native"));
    let mut recursive = library(LibraryChoice::SceneMaterial { outputs: output() });
    recursive.source.presentation["scene"]["procedural"] = json!({"operations":[]});
    recursive.source.material_fingerprint = fingerprint(&recursive.source.presentation).unwrap();
    assert!(library_build(recursive).is_err());
}
#[test]
fn transported_position_cannot_install_or_run_without_genuine_native_owner_witness() {
    let def = definition();
    let pos = position();
    let mut host = ConductHost::default();
    let error = host
        .execute(
            ConductRequest::Install {
                definition: Box::new(def.clone()),
            },
            pos.clone(),
            None,
        )
        .unwrap_err();
    assert!(error.contains("native owner timing/source/Act witness unavailable"));
    assert!(
        host.execute(
            ConductRequest::Event {
                input: Box::new(event(&def, 1))
            },
            pos.clone(),
            None
        )
        .is_err()
    );
    // Pure canonical discovery/library needs no transported clock grant.
    let discovery = host
        .execute(ConductRequest::LibraryDiscover {}, pos.clone(), None)
        .unwrap();
    assert!(discovery.is_object());
    let built = host
        .execute(
            ConductRequest::LibraryBuild {
                input: Box::new(library(LibraryChoice::SceneMaterial { outputs: output() })),
            },
            pos,
            None,
        )
        .unwrap();
    assert!(built["procedure"]["recipe_parameters"]["native_program"].is_object());
}

#[test]
fn epoch_domain_and_instance_remain_distinct_and_no_source_receipt_grants_timing() {
    let mut def = definition();
    def.procedure.timing.epoch_ref = "actual-performance-epoch:distinct-from-instance".into();
    def.procedure.timing.domain = "sample".into();
    let mut host = ConductHost::default();
    let error = host
        .execute(
            ConductRequest::Install {
                definition: Box::new(def),
            },
            position(),
            None,
        )
        .unwrap_err();
    assert!(error.contains("native owner timing/source/Act witness unavailable"));
    assert!(!error.contains("instance"));
}

#[test]
fn new_ordinary_human_force_edit_replaces_older_override_with_actual_actor_and_stable_target() {
    let old = contribution();
    let id = old.generated_basis["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap();
    let path = format!("/scene/entities/@{id}/force/strength");
    let mut before = old.generated_basis.clone();
    before["scene"]["entities"][0]["force"]["strength"] = json!(0.75);
    let mut after = before.clone();
    after["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
    let mut edit = manual(before, after.clone(), &old);
    edit.retained_overlays = vec![AuthoredOverlay {
        contribution_ref: old.contribution_ref.clone(),
        pointer: path,
        value: json!(0.75),
        actor_ref: "human:older-edit".into(),
        persistent: true,
        operation: OverlayOperation::Set,
    }];
    let attributed = extract_manual_interventions(&edit).unwrap();
    assert_eq!(attributed.interventions[0].actor_ref, "human:owner");
    assert_eq!(attributed.interventions[0].address.component, "force");
    assert_eq!(attributed.overlays[0].value, 0.875);
    let mut next = old.clone();
    next.generated_basis["scene"]["entities"][0]["force"]["strength"] = json!(0.25);
    next.generated_basis["scene"]["entities"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let delta = regenerate(
        std::slice::from_ref(&old),
        &[CurrentContribution {
            contribution_ref: old.contribution_ref.clone(),
            material: after,
            overlays: attributed.overlays,
        }],
        &[next],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    let entities = delta.effective_basis[&old.contribution_ref]["scene"]["entities"]
        .as_array()
        .unwrap();
    assert_eq!(
        entities.iter().find(|e| e["id"] == id).unwrap()["force"]["strength"],
        0.875
    );
}
#[test]
fn manual_deletion_tombstone_and_order_persist_through_generator_delete_readd() {
    let old = contribution();
    let before = old.generated_basis.clone();
    let mut after = before.clone();
    after["scene"]["entities"][0]["layers"]
        .as_array_mut()
        .unwrap()
        .clear();
    after["scene"]["entities"].as_array_mut().unwrap().reverse();
    let attributed = extract_manual_interventions(&manual(before, after.clone(), &old)).unwrap();
    assert!(
        attributed
            .interventions
            .iter()
            .any(|i| i.kind == InterventionKind::Delete && i.address.component == "layer")
    );
    assert!(
        attributed
            .overlays
            .iter()
            .any(|i| i.operation == OverlayOperation::Delete)
    );
    let mut next = old.clone();
    next.generated_basis["scene"]["entities"][0]["layers"]
        .as_array_mut()
        .unwrap()
        .clear();
    next.generated_basis["scene"]["entities"][1]["force"]["strength"] = json!(0.3);
    let first = regenerate(
        std::slice::from_ref(&old),
        &[CurrentContribution {
            contribution_ref: old.contribution_ref.clone(),
            material: after,
            overlays: attributed.overlays,
        }],
        &[next.clone()],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    let material = first.effective_basis[&old.contribution_ref].clone();
    assert_eq!(
        material["scene"]["entities"][0]["id"],
        old.generated_basis["scene"]["entities"][1]["id"]
    );
    let mut readd = next.clone();
    readd.generated_basis["scene"]["entities"][0]["layers"] =
        old.generated_basis["scene"]["entities"][0]["layers"].clone();
    let second = regenerate(
        &[next],
        &[CurrentContribution {
            contribution_ref: old.contribution_ref.clone(),
            material,
            overlays: first.retained_overlays,
        }],
        &[readd],
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    let id = old.generated_basis["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap();
    let entities = second.effective_basis[&old.contribution_ref]["scene"]["entities"]
        .as_array()
        .unwrap();
    assert!(
        entities.iter().find(|e| e["id"] == id).unwrap()["layers"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn same_layer_id_in_base_and_sequence_state_has_exact_parent_native_address() {
    let c = contribution();
    let mut before = c.generated_basis.clone();
    let base = before["scene"]["entities"][0]["layers"][0].clone();
    before["scene"]["entities"][0]["sequence"]["steps"][0]["layers"] = json!([base]);
    let mut after = before.clone();
    after["scene"]["entities"][0]["sequence"]["steps"][0]["layers"][0]["z"] = json!(8);
    let result = extract_manual_interventions(&manual(before, after, &c)).unwrap();
    assert_eq!(result.interventions.len(), 1);
    let a = &result.interventions[0].address;
    assert_eq!(a.component, "layer");
    assert_eq!(a.constituent_ref.as_deref(), Some("retained-layer"));
    assert_eq!(a.parent_ref, Some(Some("source-1".into())));
    let mut base = a.clone();
    base.parent_ref = Some(None);
    assert!(!base.covers(a));
    let mut sequence = a.clone();
    sequence.component = "sequence".into();
    sequence.constituent_ref = None;
    sequence.parent_ref = None;
    sequence.property = None;
    assert!(sequence.covers(a));
    assert!(!sequence.covers(&base));
    let mut link = sequence;
    link.component = "sequence_link".into();
    link.constituent_ref = Some("source-1".into());
    assert!(link.covers(a));
    link.constituent_ref = Some("source-2".into());
    assert!(!link.covers(a));
}

#[test]
fn exact_tri_state_layer_wire_preserves_legacy_base_and_state_without_nonlayer_presence() {
    let c = contribution();
    let scene = c.owned_addresses[0].clone();
    let wire = serde_json::to_value(&scene).unwrap();
    assert!(wire.get("parent_ref").is_none());
    let mut layer = scene.clone();
    layer.component = "layer".into();
    layer.entity_ref = Some("expression:acceptance:entity:existing".into());
    layer.constituent_ref = Some("same-layer".into());
    let omitted = serde_json::to_value(&layer).unwrap();
    assert!(omitted.get("parent_ref").is_none());
    let legacy: OwnedAddress = serde_json::from_value(omitted.clone()).unwrap();
    assert_eq!(legacy.parent_ref, None);
    layer.parent_ref = Some(None);
    let base = serde_json::to_value(&layer).unwrap();
    assert_eq!(base["parent_ref"], Value::Null);
    let roundtrip: OwnedAddress = serde_json::from_value(base).unwrap();
    assert_eq!(roundtrip.parent_ref, Some(None));
    assert!(!roundtrip.covers(&legacy));
    layer.parent_ref = Some(Some("state-exact".into()));
    let state = serde_json::to_value(&layer).unwrap();
    assert_eq!(state["parent_ref"], "state-exact");
    assert_eq!(
        serde_json::from_value::<OwnedAddress>(state).unwrap(),
        layer
    );
    let mut invalid = wire;
    invalid["parent_ref"] = Value::Null;
    assert!(
        serde_json::from_value::<OwnedAddress>(invalid)
            .unwrap()
            .validate()
            .is_err()
    );
}

#[test]
fn property_metadata_is_prepended_without_copying_native_numerical_parameter_conversion() {
    use ql_mef::procedural_retention::*;
    let built = library_build(library(LibraryChoice::ForceParameters {
        writes: vec![ParameterRecipe {
            output_slot: "native-force".into(),
            parameter: "force_strength".into(),
            value: RecipeValue::Constant {
                value: json!(0.875),
            },
        }],
    }))
    .unwrap();
    let p: Procedure = serde_json::from_value(built["procedure"].clone()).unwrap();
    let program: NativeRecipeProgram = serde_json::from_value(built["program"].clone()).unwrap();
    let entity = "expression:acceptance:entity:actual-source";
    let address = native_parameter_address(
        &p.occurrence_ref,
        "expression:acceptance:scene:canonical",
        entity,
        "force_strength",
    )
    .unwrap();
    assert_eq!(address.component, "force");
    assert_eq!(address.property.as_deref(), Some("strength"));
    let reading = TargetReading {
        occurrence_ref: entity.into(),
        address,
        subject: subject(),
        revision: 3,
        tags: vec![],
        properties: BTreeMap::from([("force_strength".into(), json!(0.5))]),
    };
    let membership = resolve_procedure_membership(
        m_tree::native_current_m_registry(),
        &p,
        &p.occurrence_ref,
        std::slice::from_ref(&reading),
        None,
    )
    .unwrap();
    let c = program_contributions(
        m_tree::native_current_m_registry(),
        &p,
        &program,
        &p.occurrence_ref,
        &[reading],
        &membership,
    )
    .unwrap();
    let mut prepared = compile_native_batch(
        m_tree::native_current_m_registry(),
        &p,
        "operation:force-material",
        &p.occurrence_ref,
        3,
        membership,
        c,
        BTreeSet::from(["scene".into()]),
    )
    .unwrap();
    let source = library(LibraryChoice::SceneMaterial { outputs: output() }).source;
    let material = source.presentation.clone();
    let existing = json!({"schema":RETENTION_SCHEMA,"bindings":[],"procedures":[],"contributions":[],"controls":[],"operations":[],"scene_flow":[],"time_mappings":[],"source_basis":[]});
    let context = NativeMaterialization {
        schema: MATERIALIZATION_CONTRACT.into(),
        lifecycles: vec![],
        document_revision: 3,
        rule_cursor: 1,
        state: "held".into(),
        scenes: vec![NativeRetentionScene {
            scene_ref: source.scene_ref,
            document_revision: 3,
            existing_retention: Some(existing),
            current_presentation: Some(material.clone()),
            principal: source.principal,
            contributors: source.contributors,
            locus: NativeReading {
                reference: source.locus_ref,
                revision: source.locus_revision,
                availability: ReadingAvailability::Available,
            },
        }],
    };
    materialize_retention(&mut prepared, &context).unwrap();
    let changes = prepared.native_edit["changes"].as_array().unwrap();
    assert_eq!(changes[0]["change"], "scene_material_set");
    assert_eq!(changes[1]["change"], "parameter_set");
    assert_eq!(changes[1]["value"], 0.875);
    let mut projection = changes[0]["presentation"].clone();
    projection["scene"]
        .as_object_mut()
        .unwrap()
        .remove("procedural");
    assert_eq!(projection, material);
    let r = &changes[0]["presentation"]["scene"]["procedural"];
    assert_eq!(
        r["procedures"][0]["definition"],
        serde_json::to_value(&p).unwrap()
    );
    assert_eq!(r["operations"], json!([]));
    assert_eq!(r["source_basis"][0]["ref"], p.recipe.source_ref);
    assert!(r["source_basis"][0].get("source_ref").is_none());
    let mut fingerprint_basis = prepared.clone();
    fingerprint_basis.fingerprint = String::new();
    assert_eq!(
        prepared.fingerprint,
        fingerprint(&fingerprint_basis).unwrap()
    );
}

/// The actual ql-field-host parser/process and actual installed C++ worker.
/// Explicitly executed by the qualified worker CI lane, never a fake pipe.
#[test]
#[ignore = "requires actual QL_FIELD_WORKER, built ql-field-host and declared TA_ONTA_CONDUCT_RUN_DIR"]
fn actual_field_host_process_receives_pure_source_discovery_at_paused_and_running_boundaries() {
    use std::io::{BufRead, Write};
    use std::process::{Command, Stdio};
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let world:ql_mef::scene::WorldRequest=serde_json::from_value(json!({"schema":ql_mef::scene::WORLD_REQUEST,"instance_ref":"controlled:procedural:real-host","event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-procedural","texture":[64,64],"units_per_metre":1.0,"sky":sky,"geometry":{"longitude_samples":32,"latitude_samples":16,"metres_per_unit":1.0,"attachment":1}})).unwrap();
    let config = ql_mef::scene::world(world).unwrap()["binding"]["host"].clone();
    let directory = std::path::PathBuf::from(
        std::env::var("TA_ONTA_CONDUCT_RUN_DIR").expect("declared Central test run directory"),
    );
    std::fs::create_dir_all(&directory).unwrap();
    let path = directory.join(format!("ql-procedural-host-{}.json", std::process::id()));
    std::fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    let worker = std::env::var("QL_FIELD_WORKER").expect("actual installed worker");
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql-field-host"))
        .args([worker.as_str(), path.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let mut output = std::io::BufReader::new(child.stdout.take().unwrap());
    let mut line = String::new();
    output.read_line(&mut line).unwrap();
    let mut current: Value = serde_json::from_str(&line).unwrap();
    assert_eq!(current["status"], "ready");
    fn send(
        input: &mut impl Write,
        output: &mut impl BufRead,
        current: &Value,
        command: Value,
    ) -> Value {
        let packet = json!({"schema":"ql.field-host-request/v1","instance_ref":current["instance_ref"],"event_ref":current["field"]["event_ref"],"subject_ref":current["field"]["subject_ref"],"request_id":(current["last_request_id"].as_str().unwrap().parse::<u64>().unwrap()+1).to_string(),"expected_generation":current["field"]["generation"],"expected_samples_elapsed":current["field"]["samples_elapsed"],"command":command});
        writeln!(input, "{}", serde_json::to_string(&packet).unwrap()).unwrap();
        input.flush().unwrap();
        let mut line = String::new();
        output.read_line(&mut line).unwrap();
        serde_json::from_str(&line).unwrap()
    }
    let paused = current["field"].clone();
    current = send(
        &mut input,
        &mut output,
        &current,
        json!({"operation":"procedure","request":{"action":"library_discover"}}),
    );
    assert_eq!(current["status"], "ok");
    assert_eq!(current["field"], paused);
    current = send(
        &mut input,
        &mut output,
        &current,
        json!({"operation":"advance","frames":512,"muted":false}),
    );
    assert_eq!(current["status"], "ok");
    assert_eq!(current["field"]["audio"].as_array().unwrap().len(), 512);
    let mut running = current["field"].clone();
    running["audio"] = json!([]);
    current = send(
        &mut input,
        &mut output,
        &current,
        json!({"operation":"procedure","request":{"action":"library_discover"}}),
    );
    assert_eq!(current["status"], "ok");
    assert_eq!(current["field"], running);
    assert_eq!(current["field"]["audio"], json!([]));
    // Genuine installation/event/hold/resume still requires R's actual private
    // timing/source/Act factory; discovery is never a fabricated material ACK.
    drop(input);
    assert!(child.wait().unwrap().success());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn shared_native_subject_retains_every_scene_and_one_global_parameter_operation() {
    let registry = m_tree::native_current_m_registry();
    let mut p = procedure();
    p.selector = Selector::Subject {
        subject_ref: p.principal_subject_ref.clone(),
    };
    let entity = "expression:acceptance:entity:shared";
    let readings = [
        "expression:acceptance:scene:a",
        "expression:acceptance:scene:b",
    ]
    .into_iter()
    .map(|scene| TargetReading {
        occurrence_ref: entity.into(),
        address: OwnedAddress {
            expression_ref: p.occurrence_ref.clone(),
            scene_ref: Some(scene.into()),
            entity_ref: Some(entity.into()),
            component: "entity".into(),
            constituent_ref: None,
            parent_ref: None,
            property: None,
        },
        subject: subject(),
        revision: 3,
        tags: vec![],
        properties: BTreeMap::from([("force_radius".into(), json!(120))]),
    })
    .collect::<Vec<_>>();
    let program = NativeRecipeProgram::ForceParameters {
        writes: vec![ParameterRecipe {
            output_slot: "radius".into(),
            parameter: "force_radius".into(),
            value: RecipeValue::Constant { value: json!(200) },
        }],
    };
    p.recipe_parameters
        .insert("native_program".into(), json!(program));
    let membership = resolve_membership(
        &p.selector,
        &p.occurrence_ref,
        &readings,
        p.membership_mode,
        None,
        p.membership_change_policy.clone(),
    )
    .unwrap();
    assert_eq!(membership.targets.len(), 1);
    assert_eq!(membership.addresses.len(), 2);
    assert_eq!(
        membership.addresses[entity].scene_ref.as_deref(),
        Some("expression:acceptance:scene:a")
    );
    let c = program_contributions(
        registry,
        &p,
        &program,
        &p.occurrence_ref,
        &readings,
        &membership,
    )
    .unwrap();
    assert_eq!(c.len(), 1);
    assert_eq!(c[0].owned_addresses.len(), 2);
    assert_eq!(c[0].native_changes.len(), 1);
    assert_eq!(c[0].occurrence_ref, entity);
    let prepared = compile_native_batch(
        registry,
        &p,
        "operation:shared-radius",
        &p.occurrence_ref,
        3,
        membership.clone(),
        c.clone(),
        BTreeSet::from(["scene".into()]),
    )
    .unwrap();
    assert_eq!(prepared.native_edit["changes"].as_array().unwrap().len(), 1);
    assert_eq!(
        prepared.native_edit["changes"][0]["change"],
        "parameter_set"
    );
    assert_eq!(prepared.membership.addresses.len(), 2);
    let mut reverse = readings.clone();
    reverse.reverse();
    let reversed = resolve_membership(
        &p.selector,
        &p.occurrence_ref,
        &reverse,
        p.membership_mode,
        None,
        p.membership_change_policy.clone(),
    )
    .unwrap();
    assert_eq!(membership, reversed);
    assert_eq!(
        c,
        program_contributions(
            registry,
            &p,
            &program,
            &p.occurrence_ref,
            &reverse,
            &reversed
        )
        .unwrap()
    );
    let mut incomplete = c.clone();
    incomplete[0].owned_addresses.pop();
    assert!(
        compile_native_batch(
            registry,
            &p,
            "operation:incomplete",
            &p.occurrence_ref,
            3,
            membership.clone(),
            incomplete,
            BTreeSet::from(["scene".into()])
        )
        .unwrap_err()
        .contains("affected resolved Scene")
    );
    let mut contradictory = readings.clone();
    contradictory[1]
        .properties
        .insert("force_radius".into(), json!(121));
    assert!(
        program_contributions(
            registry,
            &p,
            &program,
            &p.occurrence_ref,
            &contradictory,
            &membership
        )
        .is_err()
    );
    let scalars = readings
        .iter()
        .enumerate()
        .map(|(n, r)| {
            let mut scalar = r.clone();
            scalar.address = native_parameter_address(
                &p.occurrence_ref,
                r.address.scene_ref.as_deref().unwrap(),
                entity,
                "force_radius",
            )
            .unwrap();
            scalar.occurrence_ref = format!("native-issued-scalar-fixture:{n}");
            scalar
        })
        .collect::<Vec<_>>();
    let mut scalar_p = p.clone();
    scalar_p.selector = Selector::Occurrences {
        refs: scalars.iter().map(|r| r.occurrence_ref.clone()).collect(),
    };
    let scalar_membership = resolve_membership(
        &scalar_p.selector,
        &p.occurrence_ref,
        &scalars,
        p.membership_mode,
        None,
        p.membership_change_policy.clone(),
    )
    .unwrap();
    let scalar_c = program_contributions(
        registry,
        &scalar_p,
        &program,
        &p.occurrence_ref,
        &scalars,
        &scalar_membership,
    )
    .unwrap();
    assert_eq!(scalar_c[0].contribution_ref, c[0].contribution_ref);
    compile_native_batch(
        registry,
        &scalar_p,
        "operation:exact-scalar",
        &p.occurrence_ref,
        3,
        scalar_membership.clone(),
        scalar_c.clone(),
        BTreeSet::from(["scene".into()]),
    )
    .unwrap();
    let mut sibling = scalar_c;
    sibling[0].native_changes[0] = NativeChange::ParameterSet {
        entity_ref: entity.into(),
        parameter: "force_strength".into(),
        value: json!(0.8),
    };
    sibling[0].owned_addresses = sibling[0]
        .owned_addresses
        .iter()
        .map(|a| {
            native_parameter_address(
                &p.occurrence_ref,
                a.scene_ref.as_deref().unwrap(),
                entity,
                "force_strength",
            )
            .unwrap()
        })
        .collect();
    assert!(
        compile_native_batch(
            registry,
            &scalar_p,
            "operation:forged-sibling",
            &p.occurrence_ref,
            3,
            scalar_membership,
            sibling,
            BTreeSet::from(["scene".into()])
        )
        .is_err()
    );
    let mut duplicate = readings;
    duplicate.push(duplicate[0].clone());
    assert!(
        resolve_membership(
            &p.selector,
            &p.occurrence_ref,
            &duplicate,
            p.membership_mode,
            None,
            p.membership_change_policy.clone()
        )
        .is_err()
    );
}

#[test]
fn interval_continuation_requires_original_native_binding_and_advancing_cursor() {
    let p = procedure();
    let mut rule = RuleExecution::new(&p, "actual:initial-interval").unwrap();
    rule.evaluations = 5;
    rule.operations = 11;
    let before = serde_json::to_value(&rule).unwrap();
    let same_cursor = rule.cursor;
    assert!(
        continue_native_interval(
            &mut rule,
            &p,
            "actual:initial-interval",
            &p.timing,
            same_cursor
        )
        .is_err()
    );
    assert_eq!(serde_json::to_value(&rule).unwrap(), before);
    let cursor = rule.cursor + 1;
    let mut foreign = p.timing.clone();
    foreign.epoch_ref = "foreign:native-epoch".into();
    assert!(
        continue_native_interval(&mut rule, &p, "actual:initial-interval", &foreign, cursor)
            .is_err()
    );
    assert!(continue_native_interval(&mut rule, &p, "stale:interval", &p.timing, cursor).is_err());
    continue_native_interval(&mut rule, &p, "actual:initial-interval", &p.timing, cursor).unwrap();
    assert_eq!(rule.evaluations, 0);
    assert_eq!(rule.operations, 0);
    assert_eq!(rule.cursor, cursor);
    assert_eq!(rule.original_procedure, p);
    let interval = rule.interval_ref.clone();
    assert!(continue_native_interval(&mut rule, &p, &interval, &p.timing, cursor).is_err());
}

#[test]
fn one_event_projects_all_native_scene_records_in_native_units_without_another_actor_or_ordinal() {
    use ql_mef::procedural_intervention::{
        InterventionKind, NativeAuthoredIntervention, NativeInterventionBasis,
        SceneInterventionBasis,
    };
    let def = definition();
    let p = &def.procedure;
    let entity = "expression:acceptance:entity:shared";
    let scenes = [
        "expression:acceptance:scene:a",
        "expression:acceptance:scene:b",
    ];
    let addresses = scenes
        .iter()
        .map(|s| native_parameter_address(&p.occurrence_ref, s, entity, "force_radius").unwrap())
        .collect::<Vec<_>>();
    let current = json!({"schema":"ql.native-parameter-state/v1","address":addresses[0],"addresses":addresses,"parameter":"force_radius","value":120,"target_revision":9});
    let reference = "contribution:actual-shared-force";
    let mut input = event(&def, 1);
    input.document_revision = 9;
    input.current_contributions = vec![CurrentContribution {
        contribution_ref: reference.into(),
        material: current.clone(),
        overlays: vec![],
    }];
    // DTO intake only. This projector cannot grant Applied/source authority;
    // receiving native owner independently re-attests the protected journal.
    input.output_readings = vec![RetainedOutputReading {
        schema: RETAINED_OUTPUT_READING.into(),
        native_owner: "oi.expression".into(),
        expression_ref: p.occurrence_ref.clone(),
        document_revision: 9,
        procedure_ref: p.procedure_ref.clone(),
        source_basis: vec![p.recipe.clone(), p.profile.clone()],
        origin_source_basis: None,
        contribution_ref: reference.into(),
        output_slot: "radius:shared".into(),
        subject_refs: vec![p.principal_subject_ref.clone()],
        occurrence_ref: entity.into(),
        recipe_revision: p.recipe.revision.clone(),
        owned_addresses: addresses.clone(),
        generated_basis: json!({"parameter":"force_radius","value":200}),
        current_basis: current,
        status: "active".into(),
        applied_operation: Value::Null,
    }];
    input.intervention_contexts = scenes
        .iter()
        .enumerate()
        .map(|(n, scene)| {
            let mut material = presentation();
            material["scene"]["id"] = json!(scene);
            let id = material["scene"]["entities"][0]["id"]
                .as_str()
                .unwrap()
                .to_owned();
            NativeInterventionBasis::Scene {
                basis: SceneInterventionBasis {
                    expression_ref: p.occurrence_ref.clone(),
                    scene_ref: (*scene).into(),
                    contribution_ref: reference.into(),
                    owned_addresses: addresses.clone(),
                    entity_refs: BTreeMap::from([(id.clone(), entity.into())]),
                    current_presentation: material,
                    document_revision: 9,
                    retained_native_records: vec![NativeAuthoredIntervention {
                        address: addresses[n].clone(),
                        actor: if n == 0 {
                            "human:older"
                        } else {
                            "human:latest"
                        }
                        .into(),
                        persistent: true,
                        revision: 8 + n as u64,
                        path: format!("/scene/entities/@{id}/force/radius"),
                        value: json!(0.3),
                        kind: InterventionKind::Set,
                        operation_ref: Some(format!("accepted:manual:{n}")),
                        before: Some(json!(0.5)),
                    }],
                },
            }
        })
        .collect();
    let original_position = input.position.clone();
    let original_cause = input.event.cause.clone();
    let mut projected = input.clone();
    project_event_interventions(&mut projected).unwrap();
    let overlay = &projected.current_contributions[0].overlays[0];
    assert_eq!(overlay.pointer, "/value");
    assert_eq!(overlay.value, 120);
    assert_ne!(overlay.value, json!(0.3));
    assert_eq!(overlay.actor_ref, "human:latest");
    assert_eq!(projected.position, original_position);
    assert_eq!(projected.event.cause, original_cause);
    let mut missing = input.clone();
    missing.intervention_contexts.pop();
    assert!(
        project_event_interventions(&mut missing)
            .unwrap_err()
            .contains("omit")
    );
    let mut foreign = input.clone();
    let NativeInterventionBasis::Scene { basis } = &mut foreign.intervention_contexts[1] else {
        unreachable!()
    };
    basis.retained_native_records[0].address.scene_ref = Some(scenes[0].into());
    assert!(project_event_interventions(&mut foreign).is_err());
    let mut caller = input;
    caller.current_contributions[0].overlays = projected.current_contributions[0].overlays.clone();
    assert!(
        project_event_interventions(&mut caller)
            .unwrap_err()
            .contains("caller supplied")
    );
}
