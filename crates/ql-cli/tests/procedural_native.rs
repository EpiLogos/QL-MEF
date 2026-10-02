//! Real executable uptake of the source-qualified procedural producer.
//! Application, body and audio reception remain their native consumers' proof.
use ql_mef::procedural_composition::*;
use ql_mef::procedural_manifestation::*;
use ql_mef::{MFace, coordinate_expression, m_tree};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Output, Stdio};

const REQUEST: &str = "ql.scene-procedural-request/v1";

fn native(operation: &str, input: Option<&Value>) -> Output {
    let mut args = vec!["scene", "procedural", operation];
    if input.is_some() {
        args.push("-");
    }
    args.push("--json");
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(input) = input {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&serde_json::to_vec(input).unwrap())
            .unwrap();
    }
    child.wait_with_output().unwrap()
}
fn result(operation: &str, input: Option<&Value>) -> Value {
    let output = native(operation, input);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["schema"], "ql.scene-procedural-response/v1");
    value["result"].clone()
}
fn refused(operation: &str, input: &Value, reason: &str) {
    let output = native(operation, Some(input));
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains(reason),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
fn subject() -> NativeSubject {
    let m = m_tree::native_current_m_registry().manifest();
    NativeSubject {
        subject_ref: "ql:m-coordinate:bimba:M3".into(),
        native_owner: "ql-mef".into(),
        presentation_role: SubjectRole::Thing,
        sources: vec![NativeReading {
            reference: format!("{}:{}", m.source_repository, m.source_dataset_tree),
            revision: m.source_snapshot_sha256.clone(),
            availability: ReadingAvailability::Available,
        }],
        readings: vec![],
        actions: vec![],
    }
}
fn procedure() -> Procedure {
    Procedure {schema:PROCEDURE_CONTRACT.into(),procedure_ref:"procedure:native-CLI-passage".into(),revision:"1".into(),
        recipe:SourceBasis {source_ref:"docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3".into(),
            revision:fingerprint(&include_str!("../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md")).unwrap()},
        profile:SourceBasis {source_ref:"fixture:procedural-scene-template-v1".into(),revision:fingerprint(&template()).unwrap()},
        registry_revision:m_tree::native_current_m_registry().manifest().registry_revision.clone(),principal_subject_ref:subject().subject_ref,
        locus_ref:"ql:m-coordinate:bimba:M3".into(),occurrence_ref:"expression:acceptance".into(),
        composition:serde_json::from_value(json!({"CPF":"dialogical","CT":"CT2","CP":"4.2","CF":"CF3","CFP":"CFP1","CS":"CS0",
            "direction":"forward","actor":"agent:nativeCLI","interpretation":{"ref":"ql:interpretation:c-prime","revision":"1"},
            "whole":"expression:acceptance","resolvePath":"aikit:resolve:procedural","contextResolution":"aikit:context:procedural",
            "sources":["docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md"]})).unwrap(),
        trigger:Trigger {kind:TriggerKind::Explicit,mode:TriggerMode::Level},selector:Selector::All,conditions:vec![],recipe_parameters:BTreeMap::new(),
        membership_mode:MembershipMode::Frozen,membership_change_policy:MembershipChangePolicy::AdmitAndRecord,
        timing:TimingBinding {owner_ref:"native:expression".into(),domain:"simulation".into(),epoch_ref:"epoch:actual-owner".into(),requested_cursor:42,time_mapping_ref:None},
        budgets:ExecutionBudget {max_evaluations:8,max_operations:64,max_expansion_depth:8,max_active_instances:8,max_queue:8},
        seed:"source-qualified-native-CLI".into(),seed_algorithm:SEED_ALGORITHM.into(),removal_policy:RemovalPolicy::RetireUneditedDetachEdited,
        failure_policy:FailurePolicy::StopAffectedAndCheckpoint,continuation_policy:ProcedureContinuation::Continue,
        admitted_changes:["scene_create","scene_remove","entity_add","entity_remove","subject_bind","scene_compose","scene_material_set","parameter_set","focus"].into_iter().map(str::to_owned).collect()}
}
fn template() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap()
}
fn prepare() -> Value {
    json!({"schema":REQUEST,"procedure":procedure(),"expression_ref":"expression:acceptance","document_revision":1,"operation_ref":"operation:nativeCLI",
        "current_readings":[],"previous_membership":null,"contributions":[],"required_consumers":["scene","nativeBody","audio"],
        "scene_outputs":(0..3).map(|index|json!({"output_slot":format!("passage-{index}"),"scene_ref":format!("expression:acceptance:scene:passage-{index}"),
            "subjects":[subject()],"source_basis":procedure().profile,"material_fingerprint":fingerprint(&template()).unwrap(),
            "source_presentation":template()})).collect::<Vec<_>>()})
}
fn manifest() -> Value {
    let m = m_tree::native_current_m_registry();
    let b = coordinate_expression::resolve_coordinate_expression(m, "#3", MFace::Bimba).unwrap();
    json!({"schema":REQUEST,"request":{"coordinate_ref":b.rooted_world.direct.canonical_ref,"face":"bimba",
        "expected_registry_revision":m.manifest().registry_revision,"expected_profile_revision":b.binding_content_revision,
        "occasion_ref":"occasion:retained-owner","principal":subject(),"contributors":[],"required_relations":[],"intended_act":"inhabit a native source-qualified form",
        "manifestations":[{"output_slot":"form","occurrence":{"expression_ref":"expression:acceptance","scene_ref":"expression:acceptance:scene:source",
            "entity_ref":"expression:acceptance:entity:source","component":"formation","constituent_ref":null},
            "recipe":{"source_ref":"native:M3_state","revision":m.manifest().source_revision},"material":{"treatment":"glyph_mask"},"standing":"source_derived"}]}})
}

#[test]
fn fresh_native_discovery_names_real_source_types_owners_limits_and_open_consumers() {
    let d = result("discover", None);
    assert_eq!(d["contracts"]["composition"], PROCEDURE_CONTRACT);
    assert_eq!(d["limits"]["targets"], 2048);
    assert_eq!(d["limits"]["input_bytes"], 1_048_576);
    assert_eq!(
        d["source"]["snapshot_sha256"],
        m_tree::native_current_m_registry()
            .manifest()
            .source_snapshot_sha256
    );
    assert_eq!(
        d["source_obligations"].as_object().unwrap().len(),
        manifestation_inventory(m_tree::native_current_m_registry()).len()
    );
    assert!(
        d["capability_standing"]["consumer_ack"]
            .as_str()
            .unwrap()
            .contains("actual")
    );
    let caps = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["capabilities", "--json"])
        .output()
        .unwrap();
    assert!(caps.status.success());
    let caps: Value = serde_json::from_slice(&caps.stdout).unwrap();
    assert!(
        caps["commands"]
            .as_array()
            .unwrap()
            .contains(&json!("scene.procedural.prepare"))
    );
}

#[test]
fn actual_manifest_transport_preserves_subject_locus_occurrence_and_detects_wrong_inputs() {
    let input = manifest();
    let plan = result("manifest", Some(&input));
    assert_eq!(
        plan["native_bindings"][0]["principal"]["subject_ref"],
        subject().subject_ref
    );
    assert_eq!(
        plan["native_bindings"][0]["address"]["entity_ref"],
        "expression:acceptance:entity:source"
    );
    assert!(plan["native_bindings"][0]["address"]["property"].is_null());
    let mut wrong = input.clone();
    wrong["request"]["principal"]["subject_ref"] = json!("ql:m-coordinate:bimba:M2");
    refused("manifest", &wrong, "locus");
    let mut wrong = input.clone();
    wrong["request"]["face"] = json!("pratibimba");
    refused("manifest", &wrong, "face");
    let mut wrong = input.clone();
    wrong["request"]["coordinate_ref"] = json!("#3-nonexistent");
    refused("manifest", &wrong, "unknown");
    let mut wrong = input.clone();
    wrong["request"]["expected_registry_revision"] = json!("stale");
    refused("manifest", &wrong, "registry");
    for key in ["ref", "revision"] {
        let mut wrong = input.clone();
        wrong["request"]["principal"]["sources"][0][key] = json!("foreign:source");
        refused("manifest", &wrong, "source");
    }
    let mut wrong = input;
    wrong["request"]["principal"]["sources"][0]["availability"] = json!("withheld");
    refused("manifest", &wrong, "withheld");
}

#[test]
fn native_preparation_constructs_three_real_scenes_and_exact_repeat_is_the_same_intent() {
    let input = prepare();
    let a = result("prepare", Some(&input));
    let b = result("prepare", Some(&input));
    assert_eq!(a, b);
    let changes = a["native_edit"]["changes"].as_array().unwrap();
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
    let material = changes
        .iter()
        .find(|c| c["change"] == "scene_material_set")
        .unwrap();
    assert_eq!(
        material["presentation"]["scene"]["entities"][0]["force"]["strength"],
        0.25
    );
    assert_eq!(
        material["presentation"]["scene"]["entities"][0]["sequence"]["steps"][0]["source"]["ascii"]
            ["text"],
        "ATG"
    );
    assert_eq!(
        material["presentation"]["scene"]["entities"][0]["layers"],
        template()["scene"]["entities"][0]["layers"]
    );
    let mut stale = input.clone();
    stale["procedure"]["registry_revision"] = json!("stale");
    refused("prepare", &stale, "stale");
    let mut wrong = input.clone();
    wrong["scene_outputs"][0]["source_basis"]["revision"] = json!("wrong-source-revision");
    refused("prepare", &wrong, "source/profile");
    let mut wrong = input.clone();
    wrong["scene_outputs"][0]["source_presentation"]["scene"]["entities"][0]["force"]["strength"] =
        json!(0.6);
    refused("prepare", &wrong, "fingerprint");
    let mut unknown = input;
    unknown["arbitrary_script"] = json!("unsafe body");
    refused("prepare", &unknown, "unknown field");
}

fn reading(id: &str) -> TargetReading {
    TargetReading {
        occurrence_ref: format!("occurrence:{id}"),
        address: OwnedAddress {
            expression_ref: "expression:acceptance".into(),
            scene_ref: Some("expression:acceptance:scene:existing".into()),
            entity_ref: Some(format!("expression:acceptance:entity:{id}")),
            component: "entity".into(),
            constituent_ref: None,
            property: None,
        },
        subject: subject(),
        revision: 7,
        tags: vec![],
        properties: BTreeMap::new(),
    }
}

#[test]
fn retained_retry_cannot_change_original_selector_or_write_another_existing_target() {
    let mut p = procedure();
    p.selector = Selector::Occurrences {
        refs: vec!["occurrence:a".into()],
    };
    let a = reading("a");
    let scope = resolve_procedure_membership(
        m_tree::native_current_m_registry(),
        &p,
        "expression:acceptance",
        &[a.clone()],
        None,
    )
    .unwrap();
    let mut c = instantiate_scene(
        &p,
        "expression:acceptance",
        "force",
        "expression:acceptance:scene:generated",
        &[subject()],
        &template(),
    )
    .unwrap();
    let mut address = a.address.clone();
    address.component = "property".into();
    address.property = Some("force_strength".into());
    c.owned_addresses = vec![address];
    c.native_changes = vec![NativeChange::ParameterSet {
        entity_ref: a.address.entity_ref.clone().unwrap(),
        parameter: "force_strength".into(),
        value: json!(0.8),
    }];
    let input = json!({"schema":REQUEST,"procedure":p,"expression_ref":"expression:acceptance","document_revision":7,"operation_ref":"operation:retained",
        "current_readings":[a],"previous_membership":scope,"contributions":[c],"scene_outputs":[],"required_consumers":["scene"]});
    assert_eq!(
        result("prepare", Some(&input))["native_edit"]["changes"][0]["value"],
        0.8
    );
    let mut wrong = input.clone();
    wrong["procedure"]["selector"] = json!({"selector":"all"});
    refused("prepare", &wrong, "selector definition");
    let mut wrong = input;
    wrong["contributions"][0]["owned_addresses"][0]["entity_ref"] =
        json!("expression:acceptance:entity:b");
    wrong["contributions"][0]["native_changes"][0]["entity_ref"] =
        json!("expression:acceptance:entity:b");
    refused("prepare", &wrong, "membership");
}

fn rule(p: &Procedure, checkpoint: Option<Value>, operation: Value) -> Value {
    result(
        "rule",
        Some(
            &json!({"schema":REQUEST,"procedure":p,"checkpoint":checkpoint,"operation":operation}),
        ),
    )
}

#[test]
fn real_process_rule_checkpoints_preserve_pause_cursor_and_cannot_reset_or_restart_cancelled_work()
{
    let p = procedure();
    let start = rule(
        &p,
        None,
        json!({"action":"start","interval_ref":"interval:a"}),
    );
    for mutation in 0..5 {
        let mut changed = p.clone();
        match mutation {
            0 => changed.recipe.revision = "2".into(),
            1 => changed.profile.revision = "2".into(),
            2 => changed.locus_ref = "ql:m-coordinate:bimba:M2".into(),
            3 => changed.seed = "changed-seed".into(),
            _ => changed.revision = "2".into(),
        }
        refused(
            "rule",
            &json!({"schema":REQUEST,"procedure":changed,"checkpoint":start["checkpoint"],
            "operation":{"action":"pause","observed_cursor":99}}),
            "original procedure",
        );
    }
    let paused = rule(
        &p,
        Some(start["checkpoint"].clone()),
        json!({"action":"pause","observed_cursor":99}),
    );
    assert_eq!(paused["checkpoint"]["position"], "paused");
    assert_eq!(paused["checkpoint"]["cursor"], 99);
    let held = rule(
        &p,
        Some(paused["checkpoint"].clone()),
        json!({"action":"next","generated_operations":1}),
    );
    assert!(held["ready_event"].is_null());
    let b = rule(
        &p,
        Some(held["checkpoint"].clone()),
        json!({"action":"begin_interval","interval_ref":"interval:b","observed_cursor":120}),
    );
    refused(
        "rule",
        &json!({"schema":REQUEST,"procedure":p,"checkpoint":b["checkpoint"],"operation":{"action":"begin_interval","interval_ref":"interval:a","observed_cursor":121}}),
        "same owner interval",
    );
    let cancelled = rule(
        &p,
        Some(b["checkpoint"].clone()),
        json!({"action":"cancel","observed_cursor":130}),
    );
    refused(
        "rule",
        &json!({"schema":REQUEST,"procedure":p,"checkpoint":cancelled["checkpoint"],"operation":{"action":"resume","epoch_ref":p.timing.epoch_ref}}),
        "cancelled",
    );
    refused(
        "rule",
        &json!({"schema":REQUEST,"procedure":p,"checkpoint":cancelled["checkpoint"],"operation":{"action":"start","interval_ref":"interval:new"}}),
        "discard",
    );
}

#[test]
fn native_atlas_preserves_the_continuing_world_and_refuses_wrong_prime_or_occasion() {
    let registry = m_tree::native_current_m_registry();
    let basis =
        coordinate_expression::resolve_coordinate_expression(registry, "#3", MFace::Bimba).unwrap();
    let source = PlaceOccurrence {
        occurrence_ref: "occurrence:canonical".into(),
        expression_ref: "expression:acceptance".into(),
        scene_ref: "expression:acceptance:scene:canonical".into(),
        canonical_locus: basis.rooted_world.direct.canonical_ref.clone(),
        registry_revision: registry.manifest().registry_revision.clone(),
        profile_revision: basis.binding_content_revision,
        occasion_ref: "occasion:retained-owner".into(),
        principal_subject_ref: subject().subject_ref,
        retained_entity_refs: vec!["expression:acceptance:entity:resident".into()],
        runtime_instance_ref: "runtime:continuing-owner".into(),
        native_cursor: 1234,
        background_policy: ContinuityPolicy::Continue,
    };
    let mut destination = source.clone();
    destination.occurrence_ref = "occurrence:passage".into();
    destination.scene_ref = "expression:acceptance:scene:passage".into();
    destination.native_cursor = 5678;
    let input = json!({"schema":REQUEST,"source":source,"destination":destination,"face":"bimba","expected_destination_locus":"#3"});
    let transition = result("atlas", Some(&input));
    assert_eq!(
        transition["runtime_instance_ref"],
        "runtime:continuing-owner"
    );
    assert_eq!(transition["source_cursor"], 1234);
    assert_eq!(transition["destination_cursor"], 5678);
    assert_eq!(
        transition["native_changes"],
        json!([{"change":"focus","scene_ref":"expression:acceptance:scene:passage","entity_ref":null}])
    );
    let mut wrong = input.clone();
    wrong["face"] = json!("pratibimba");
    refused("atlas", &wrong, "face");
    let mut wrong = input.clone();
    wrong["destination"]["occasion_ref"] = json!("occasion:different");
    refused("atlas", &wrong, "occasion");
    let mut wrong = input;
    wrong["destination"]["registry_revision"] = json!("stale");
    refused("atlas", &wrong, "stale");
}

#[test]
fn native_regeneration_preserves_actual_retained_human_force_and_detects_lost_override() {
    let mut p = procedure();
    let first = result("prepare", Some(&prepare()));
    let previous: GeneratedContribution =
        serde_json::from_value(first["contributions"][0].clone()).unwrap();
    let mut actual = previous.generated_basis.clone();
    actual["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
    p.revision = "2".into();
    p.recipe_parameters
        .insert("sequence_hold".into(), json!(3.5));
    let mut next = previous.clone();
    next.procedure_revision = p.revision.clone();
    next.recipe = p.recipe.clone();
    next.generated_basis["scene"]["entities"][0]["force"]["strength"] = json!(0.5);
    next.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["hold"] = json!(3.5);
    next.generated_basis["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"] =
        json!(true);
    let current = CurrentContribution {
        contribution_ref: previous.contribution_ref.clone(),
        material: actual,
        overlays: vec![],
    };
    let mut readings = vec![TargetReading {
        occurrence_ref: previous.occurrence_ref.clone(),
        address: previous.owned_addresses[0].clone(),
        subject: subject(),
        revision: 3,
        tags: vec![],
        properties: BTreeMap::new(),
    }];
    for entity in previous.generated_basis["scene"]["entities"]
        .as_array()
        .unwrap()
    {
        let reference = entity["id"].as_str().unwrap();
        let mut address = previous.owned_addresses[0].clone();
        address.entity_ref = Some(reference.into());
        address.component = "entity".into();
        readings.push(TargetReading {
            occurrence_ref: reference.into(),
            address,
            subject: subject(),
            revision: 3,
            tags: vec![],
            properties: BTreeMap::new(),
        });
    }
    let input = json!({"schema":REQUEST,"procedure":p,"expression_ref":"expression:acceptance","document_revision":3,"operation_ref":"operation:regen",
        "current_readings":readings,"previous_membership":null,"previous":[previous],"current":[current],"next":[next],"output_readings":[],"required_consumers":["scene","nativeBody","audio"]});
    let result = result("regenerate", Some(&input));
    let effective = &result["regeneration"]["effective_basis"]
        [input["previous"][0]["contribution_ref"].as_str().unwrap()];
    assert_eq!(
        effective["scene"]["entities"][0]["force"]["strength"],
        0.875
    );
    assert_eq!(
        effective["scene"]["entities"][0]["sequence"]["steps"][1]["hold"],
        3.5
    );
    assert_eq!(result["prepared"]["native_edit"]["expected_revision"], 3);
    let mut wrong = input;
    wrong["current"][0]["overlays"] = json!([{"contribution_ref":wrong["previous"][0]["contribution_ref"],
        "pointer":"/scene/entities/0/force/strength","value":0.9,"actor_ref":"human:owner","persistent":true}]);
    refused("regenerate", &wrong, "positional");
}

#[test]
fn actual_native_scene_and_prior_commands_remain_discoverable() {
    let help = Command::new(env!("CARGO_BIN_EXE_ql"))
        .arg("help")
        .output()
        .unwrap();
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    for route in [
        "ql scene compose",
        "ql scene binding",
        "ql scene procedural",
        "ql epi-agent invoke",
        "ql agent-event project",
    ] {
        assert!(help.contains(route), "{route}");
    }
    let bad = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["scene", "procedural", "unknown", "/nonexistent.json"])
        .output()
        .unwrap();
    assert!(!bad.status.success());
}

#[test]
fn retained_native_output_input_does_not_expand_original_empty_selector() {
    let p = procedure();
    let first = result("prepare", Some(&prepare()));
    let old: GeneratedContribution =
        serde_json::from_value(first["contributions"][0].clone()).unwrap();
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
    let envelope = json!({"operation_ref":"operation:original-source-context","expression_ref":"expression:acceptance","expected_revision":1,
        "actor":p.composition.actor,"scope":{"kind":"expression"},"changes":old.native_changes,
        "sources":[{"ref":p.profile.source_ref,"revision":p.profile.revision,"availability":"available"},
            {"ref":old.recipe.source_ref,"revision":old.recipe.revision,"availability":"available"}],"participants":[],"timing":{"kind":"immediate"},"cause_ref":null});
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
        applied_operation: json!({"fingerprint":fingerprint(&envelope).unwrap(),"envelope":envelope,"targets":old.owned_addresses,"status":"applied",
            "accepted_revision":2,"applied_revision":3,"observations":[],"failure":null}),
    };
    // Exercise the real ql executable's parsing/fencing/compilation; native
    // application still verifies this transported context against its journal.
    let input = json!({"schema":REQUEST,"procedure":p,"expression_ref":"expression:acceptance","document_revision":4,"operation_ref":"operation:continue",
        "current_readings":[],"previous_membership":first["prepared"]["membership"],"previous":[old],"current":[current],"next":[next],
        "output_readings":[output],"required_consumers":["scene","nativeBody","audio"]});
    let prepared = result("regenerate", Some(&input));
    assert_eq!(
        prepared["prepared"]["membership"],
        first["prepared"]["membership"]
    );
    assert_eq!(
        prepared["prepared"]["output_readings"],
        input["output_readings"]
    );
    assert_eq!(prepared["prepared"]["native_edit"]["expected_revision"], 4);
    let mut missing = input.clone();
    missing["output_readings"] = json!([]);
    refused("regenerate", &missing, "membership");
    let mut stale = input.clone();
    stale["output_readings"][0]["document_revision"] = json!(3);
    refused("regenerate", &stale, "revision");
    let mut widened = input;
    widened["procedure"]["selector"] =
        json!({"selector":"subject","subject_ref":subject().subject_ref});
    refused("regenerate", &widened, "selector");
}
