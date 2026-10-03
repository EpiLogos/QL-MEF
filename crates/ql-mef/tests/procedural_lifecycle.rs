//! Actual native source/library/conduct and manual intervention tests.
//! No mock graph, source interpreter, receiver, body or clock.
use ql_mef::m_tree;
use ql_mef::procedural_composition::*;
use ql_mef::procedural_conduct::lifecycle::*;
use ql_mef::procedural_conduct::*;
use ql_mef::procedural_manifestation::*;
use ql_mef::procedural_retention::NativeRetentionScene;
use ql_mef::procedural_source::NativeBootstrapSceneRead;
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

fn lifecycle_reading(definition: &ConductInstall) -> NativeLifecycleInput {
    let NativeRecipeProgram::SceneMaterial { outputs } = &definition.program else {
        panic!("actual Scene recipe expected")
    };
    let source = &outputs[0].source;
    let locus = NativeReading {
        reference: source.locus_ref.clone(),
        revision: source.locus_revision.clone(),
        availability: ReadingAvailability::Available,
    };
    NativeLifecycleInput {
        schema: LIFECYCLE_INTENT.into(),
        expression_ref: source.expression_ref.clone(),
        document_revision: source.document_revision,
        scene_ref: source.scene_ref.clone(),
        operation_ref: "operation:actual-source-lifecycle".into(),
        actor_ref: "human:stage-author".into(),
        procedure_ref: definition.procedure.procedure_ref.clone(),
        expected_procedure_revision: definition.procedure.revision.clone(),
        action: LifecycleAction::Retire,
        reading: NativeLifecycleReading {
            schema: LIFECYCLE_READING.into(),
            scene_read: NativeBootstrapSceneRead {
                native_owner: source.native_owner.clone(),
                expression_ref: source.expression_ref.clone(),
                scene_ref: source.scene_ref.clone(),
                document_revision: source.document_revision,
                source_basis: source.source_basis.clone(),
                material_fingerprint: fingerprint(&source.presentation).unwrap(),
                presentation: source.presentation.clone(),
                locus: locus.clone(),
                // This is explicitly a pure typed source reading, not a native
                // read receipt/lease/clock positive. Runtime refuses without C.
                source_read_receipt_ref: "test:pure-source-reading".into(),
            },
            current_readings: vec![],
            output_readings: vec![],
            current_contributions: vec![],
            intervention_contexts: vec![],
            materialization: NativeLifecycleMaterialization {
                schema: LIFECYCLE_MATERIALIZATION.into(),
                document_revision: source.document_revision,
                scenes: vec![NativeRetentionScene {
                    scene_ref: source.scene_ref.clone(),
                    document_revision: source.document_revision,
                    existing_retention: Some(ql_mef::procedural_retention::empty_retention()),
                    current_presentation: Some(source.presentation.clone()),
                    principal: source.principal.clone(),
                    contributors: source.contributors.clone(),
                    locus,
                }],
            },
            current_flow: None,
        },
    }
}

fn generated(definition: &ConductInstall) -> Vec<GeneratedContribution> {
    let registry = m_tree::native_current_m_registry();
    let membership = resolve_procedure_membership(
        registry,
        &definition.procedure,
        &definition.expression_ref,
        &definition.current_readings,
        None,
    )
    .unwrap();
    program_contributions(
        registry,
        &definition.procedure,
        &definition.program,
        &definition.expression_ref,
        &definition.current_readings,
        &membership,
    )
    .unwrap()
}

fn policy(definition: &ConductInstall, policy: ScenePolicy) -> SettledScenePolicy {
    let NativeRecipeProgram::SceneMaterial { outputs } = &definition.program else {
        panic!("actual Scene recipe expected")
    };
    SettledScenePolicy {
        from_scene_ref: outputs[0].source.scene_ref.clone(),
        to_scene_ref: outputs[0].scene_ref.clone(),
        policy,
        operation_ref: "operation:pure-policy-config".into(),
        rule_cursor: 3,
        native_position: position(),
        original_timing: definition.procedure.timing.clone(),
    }
}

#[test]
fn lifecycle_pure_source_read_matches_actual_native_recipe_and_graph() {
    let definition = definition();
    let input = lifecycle_reading(&definition);
    input.validate_reading(&definition).unwrap();
    let (_, graph) =
        ql_mef::vak_composition_wire::compile_request(&definition.source_composition).unwrap();
    prepare_native_cprime(
        m_tree::native_current_m_registry(),
        &definition.procedure,
        &graph,
        definition.currentness.clone(),
        definition.thread_plan.clone(),
    )
    .unwrap();
    // A source JSON/currentness positive never becomes runtime timing authority.
    let mut host = ConductHost::default();
    assert!(
        host.execute(
            ConductRequest::Lifecycle {
                input: Box::new(input)
            },
            position(),
            None
        )
        .is_err()
    );
}

#[test]
fn lifecycle_refuses_stale_document_original_source_and_wrong_prime() {
    let definition = definition();
    let input = lifecycle_reading(&definition);
    for changed in 0..6 {
        let mut candidate = input.clone();
        match changed {
            0 => candidate.document_revision += 1,
            1 => candidate.expected_procedure_revision = "stale".into(),
            2 => candidate.reading.scene_read.source_basis.revision = "foreign".into(),
            3 => {
                candidate.reading.materialization.scenes[0]
                    .principal
                    .subject_ref = "ql:m-coordinate:nara:M3".into()
            }
            4 => candidate.reading.scene_read.locus.reference = "ql:m-coordinate:bimba:M1".into(),
            _ => {
                candidate.reading.materialization.scenes[0]
                    .principal
                    .sources[0]
                    .revision = "stale-native-snapshot".into()
            }
        }
        let before = serde_json::to_value(&candidate).unwrap();
        assert!(
            candidate.validate_reading(&definition).is_err(),
            "mutation {changed}"
        );
        assert_eq!(serde_json::to_value(candidate).unwrap(), before);
    }
}

#[test]
fn lifecycle_scene_policy_accepts_actual_optional_relation_selection_shape() {
    let definition = definition();
    let row = policy(&definition, ScenePolicy::Hold);
    let mut input = lifecycle_reading(&definition);
    input.action = LifecycleAction::ScenePolicy {
        from_scene_ref: row.from_scene_ref.clone(),
        to_scene_ref: row.to_scene_ref.clone(),
        policy: row.policy,
    };
    // Native Selection intentionally omits absent relation_ref.
    input.reading.current_flow = Some(json!({"schema":"ql.native-atlas-state/v1",
        "expression_ref":input.expression_ref,"focus":{"scene_ref":input.scene_ref,"entity_ref":null},
        "scene_order":[row.from_scene_ref,row.to_scene_ref]}));
    input.validate_reading(&definition).unwrap();
    input.reading.current_flow.as_mut().unwrap()["focus"]["relation_ref"] =
        json!("relation:actual-shape-binding");
    input.validate_reading(&definition).unwrap();
    input.reading.current_flow.as_mut().unwrap()["focus"]["entity_ref"] =
        json!("expression:acceptance:entity:existing");
    assert!(input.validate_reading(&definition).is_err());
}

#[test]
fn lifecycle_scene_policy_refuses_foreign_places_duplicates_and_selected_source_change() {
    let definition = definition();
    let row = policy(&definition, ScenePolicy::Continue);
    let mut input = lifecycle_reading(&definition);
    input.action = LifecycleAction::ScenePolicy {
        from_scene_ref: row.from_scene_ref.clone(),
        to_scene_ref: row.to_scene_ref.clone(),
        policy: row.policy,
    };
    input.reading.current_flow = Some(json!({"schema":"ql.native-atlas-state/v1",
        "expression_ref":input.expression_ref,"focus":{"scene_ref":null,"entity_ref":null},
        "scene_order":[row.from_scene_ref,row.to_scene_ref]}));
    input.validate_reading(&definition).unwrap();
    let mut foreign = input.clone();
    foreign.reading.current_flow.as_mut().unwrap()["scene_order"][1] =
        json!("expression:foreign:scene:passage");
    assert!(foreign.validate_reading(&definition).is_err());
    let mut duplicated = input.clone();
    duplicated.reading.current_flow.as_mut().unwrap()["scene_order"][1] = json!(input.scene_ref);
    assert!(duplicated.validate_reading(&definition).is_err());
    let mut changed = input;
    changed.reading.materialization.scenes[0]
        .current_presentation
        .as_mut()
        .unwrap()["scene"]["name"] = json!("unattested material replacement");
    assert!(changed.validate_reading(&definition).is_err());
}

#[test]
fn lifecycle_hold_filters_actual_native_construction_and_preserves_basis_identity() {
    let definition = definition();
    let generated = generated(&definition);
    let original = serde_json::to_value(&generated).unwrap();
    let mut lifecycle = LifecycleCheckpoint::default();
    let row = policy(&definition, ScenePolicy::Hold);
    lifecycle
        .scene_policies
        .insert(row.from_scene_ref.clone(), row);
    assert!(
        active_generation(&lifecycle, &definition.procedure, generated.clone())
            .unwrap()
            .is_empty()
    );
    assert_eq!(serde_json::to_value(&generated).unwrap(), original);
    lifecycle.scene_policies.clear();
    assert_eq!(
        active_generation(&lifecycle, &definition.procedure, generated.clone()).unwrap(),
        generated
    );
}

#[test]
fn lifecycle_detach_filters_stable_output_even_after_reconstruction() {
    let definition = definition();
    let first = generated(&definition);
    let mut lifecycle = LifecycleCheckpoint::default();
    lifecycle
        .detached_contribution_refs
        .insert(first[0].contribution_ref.clone());
    let repeated = generated(&definition);
    assert_eq!(first[0].contribution_ref, repeated[0].contribution_ref);
    assert!(
        active_generation(&lifecycle, &definition.procedure, repeated)
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        first[0].generated_basis["scene"]["entities"][0]["source"]["ascii"]["text"],
        json!("ATG")
    );
}

#[test]
fn lifecycle_wire_rejects_caller_event_cursor_currentness_and_receipt_grants() {
    let definition = definition();
    let input = lifecycle_reading(&definition);
    let base = serde_json::to_value(input).unwrap();
    for field in [
        "event",
        "position",
        "currentness",
        "timing",
        "native_record",
    ] {
        let mut value = base.clone();
        value[field] = json!({"source":"caller"});
        assert!(
            serde_json::from_value::<NativeLifecycleInput>(value).is_err(),
            "{field}"
        );
    }
    for field in ["rule_cursor", "state", "lifecycles"] {
        let mut value = base.clone();
        value["reading"]["materialization"][field] = json!(0);
        assert!(
            serde_json::from_value::<NativeLifecycleInput>(value).is_err(),
            "{field}"
        );
    }
}

#[test]
fn lifecycle_two_same_source_slots_detach_one_preserves_sibling_and_human_material() {
    let mut definition = definition();
    let NativeRecipeProgram::SceneMaterial { outputs } = &mut definition.program else {
        panic!("actual native Scene constructor expected")
    };
    let mut sibling = outputs[0].clone();
    sibling.output_slot = "native-passage-sibling".into();
    sibling.scene_ref = "expression:acceptance:scene:passage-sibling".into();
    outputs.push(sibling);
    definition.procedure.recipe_parameters.insert(
        "native_program".into(),
        serde_json::to_value(&definition.program).unwrap(),
    );
    let previous = generated(&definition);
    assert_eq!(previous.len(), 2);
    let a = ql_mef::procedural_retention::source_scene_anchor(&definition.procedure, &previous[0])
        .unwrap()
        .unwrap();
    let b = ql_mef::procedural_retention::source_scene_anchor(&definition.procedure, &previous[1])
        .unwrap()
        .unwrap();
    assert_eq!(a, b);
    assert_ne!(previous[0].contribution_ref, previous[1].contribution_ref);
    let mut human_a = previous[0].generated_basis.clone();
    human_a["scene"]["caption"] = json!("human detached passage remains here");
    human_a["scene"]["entities"][0]["force"]["radius"] = json!(0.875);
    let before_a = human_a.clone();
    let mut human_b = previous[1].generated_basis.clone();
    human_b["scene"]["caption"] = json!("human sibling intervention");
    let current_b = CurrentContribution {
        contribution_ref: previous[1].contribution_ref.clone(),
        material: human_b.clone(),
        overlays: vec![],
    };
    let mut lifecycle = LifecycleCheckpoint::default();
    lifecycle
        .detached_contribution_refs
        .insert(previous[0].contribution_ref.clone());
    let active_previous =
        active_generation(&lifecycle, &definition.procedure, previous.clone()).unwrap();
    let active_next =
        active_generation(&lifecycle, &definition.procedure, generated(&definition)).unwrap();
    assert_eq!(active_previous.len(), 1);
    assert_eq!(active_next.len(), 1);
    assert_eq!(
        active_next[0].contribution_ref,
        previous[1].contribution_ref
    );
    let delta = regenerate(
        &active_previous,
        std::slice::from_ref(&current_b),
        &active_next,
        RemovalPolicy::RetireUneditedDetachEdited,
    )
    .unwrap();
    assert!(delta.retired.is_empty());
    assert!(delta.detached.is_empty());
    assert_eq!(
        delta.effective_basis[&previous[1].contribution_ref]["scene"]["caption"],
        human_b["scene"]["caption"]
    );
    let changes = regeneration_native_changes(&delta, &active_previous, &[current_b]).unwrap();
    assert!(changes.iter().all(|c| match c {
        NativeChange::SceneMaterialSet { scene_ref, .. }
        | NativeChange::SceneRemove { scene_ref }
        | NativeChange::SceneCompose { scene_ref, .. } => scene_ref != &previous[0].occurrence_ref,
        _ => true,
    }));
    assert_eq!(human_a, before_a);
    assert_eq!(
        human_a["scene"]["entities"][0]["force"]["radius"],
        json!(0.875)
    );
}

#[test]
fn lifecycle_cancel_is_clock_free_but_public_transport_cannot_settle_material() {
    let definition = definition();
    let source = lifecycle_reading(&definition);
    let input = NativeLifecycleCancel {
        schema: LIFECYCLE_CANCEL.into(),
        expression_ref: source.expression_ref,
        document_revision: source.document_revision,
        scene_ref: source.scene_ref,
        operation_ref: source.operation_ref,
        actor_ref: source.actor_ref,
        procedure_ref: source.procedure_ref,
        expected_procedure_revision: source.expected_procedure_revision,
        scene_read: source.reading.scene_read,
        contributors: vec![],
        native_record: Value::Null,
    };
    // Actual native source/registry reading is pure configuration. No cancelled
    // Runtime journal, private C31 lease, timing or native material ACK exists.
    input.validate_reading(&definition).unwrap();
    let mut host = ConductHost::default();
    let request = ConductRequest::LifecycleCancel {
        input: Box::new(input.clone()),
    };
    assert!(host.request_timing(&request).unwrap().is_none());
    let error = host.execute(request, position(), None).unwrap_err();
    assert!(error.contains("private staged"), "{error}");
    for field in [
        "currentness",
        "native_position",
        "timing_witness",
        "rule_cursor",
        "event",
    ] {
        let mut wire = serde_json::to_value(&input).unwrap();
        wire[field] = json!({"forged":"caller"});
        assert!(serde_json::from_value::<NativeLifecycleCancel>(wire).is_err());
    }
    for mutate in 0..5 {
        let mut wrong = input.clone();
        match mutate {
            0 => wrong.document_revision += 1,
            1 => wrong.expected_procedure_revision = "stale".into(),
            2 => wrong.scene_read.source_basis.revision = "stale".into(),
            3 => wrong.scene_read.locus.reference = "ql:m-coordinate:pratibimba:M3".into(),
            _ => wrong.expression_ref = "expression:foreign".into(),
        }
        let before = serde_json::to_value(&wrong).unwrap();
        assert!(wrong.validate_reading(&definition).is_err());
        assert_eq!(serde_json::to_value(wrong).unwrap(), before);
    }
}

/// Run ONLY against a captured real same-owner lifecycle->S Cancel->C31
/// material-abandon settlement. No typed journal/timing/private source grant is
/// assembled by this test. The independent owner provides the whole artifact.
#[test]
#[ignore = "requires genuine private C31 lifecycle/S Cancel artifact"]
fn genuine_cancelled_journal_keeps_original_lifecycle_and_refuses_changed_receipts() {
    let path = std::env::var("QL_PROCEDURAL_LIFECYCLE_CANCEL_ARTIFACT")
        .expect("capture the real private native lifecycle and cancelled S journal first");
    let artifact: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    assert_eq!(
        artifact["schema"],
        "ql.procedural-lifecycle-cancel-evidence/v1"
    );
    let definition: ConductInstall =
        serde_json::from_value(artifact["definition"].clone()).unwrap();
    let prepared: PreparedProcedure =
        serde_json::from_value(artifact["original_preparation"].clone()).unwrap();
    let input: NativeLifecycleCancel =
        serde_json::from_value(artifact["cancel_input"].clone()).unwrap();
    input
        .validate_cancelled_configuration(&definition, &prepared)
        .unwrap();
    let host = &artifact["host_response"];
    assert_eq!(host["status"], "ok");
    let reply = &host["procedural"];
    assert_eq!(reply["status"], "material_abandoned");
    assert_eq!(
        reply["original_preparation"],
        artifact["original_preparation"]
    );
    assert_eq!(reply["native_cancelled_record"], input.native_record);
    assert!(reply["prepared"].is_null());
    assert_eq!(reply["lifecycle"]["consumer_release"], "unconfirmed");
    assert_eq!(
        artifact["before_checkpoint"]["native_position"],
        artifact["after_checkpoint"]["native_position"]
    );
    assert_eq!(
        artifact["before_checkpoint"]["last_generated"],
        artifact["after_checkpoint"]["last_generated"]
    );
    assert_eq!(
        artifact["before_checkpoint"]["rule"],
        artifact["after_checkpoint"]["rule"]
    );
    assert_eq!(artifact["retry_host_response"]["procedural"], *reply);
    assert!(artifact["after_checkpoint"]["pending_preparation"].is_null());
    assert!(artifact["after_checkpoint"]["pending_operation_ref"].is_null());
    for mutation in 0..9 {
        let mut changed = input.clone();
        match mutation {
            0 => changed.native_record["status"] = json!("scheduled"),
            1 => changed.native_record["status"] = json!("applied"),
            2 => changed.native_record["applied_revision"] = json!(changed.document_revision),
            3 => changed.native_record["envelope"]["actor"] = json!("agent:foreign"),
            4 => changed.native_record["envelope"]["operation_ref"] = json!("operation:foreign"),
            5 => {
                changed.native_record["envelope"]["expected_revision"] =
                    json!(changed.document_revision)
            }
            6 => changed.native_record["envelope"]["changes"] = json!([]),
            7 => changed.native_record["accepted_revision"] = json!(changed.document_revision + 1),
            _ => changed.native_record["fingerprint"] = json!("unqualified"),
        }
        let before = serde_json::to_value(&changed).unwrap();
        assert!(
            changed
                .validate_cancelled_configuration(&definition, &prepared)
                .is_err()
        );
        assert_eq!(serde_json::to_value(changed).unwrap(), before);
    }
}
