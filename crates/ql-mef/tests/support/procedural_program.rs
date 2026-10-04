//! Actual native compiler/configuration operands. The configured Operation
//! below tests native consistency only; it is NOT an O:I journal, Source grant,
//! timing witness, body reception or consumer acknowledgement.
use ql_mef::{
    m_tree, procedural_composition::*, procedural_conduct::*, procedural_intervention::*,
    procedural_manifestation::*, procedural_retention::*,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
pub fn subject() -> NativeSubject {
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

pub fn procedure() -> Procedure {
    let registry = m_tree::native_current_m_registry();
    Procedure { schema: PROCEDURE_CONTRACT.into(), procedure_ref: "procedure:source-native-passage".into(), revision: "1".into(),
        recipe: SourceBasis { source_ref: "docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3".into(),
            revision:fingerprint(&include_str!("../../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md")).unwrap() },
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

pub fn presentation() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/procedural-scene-template-v1.json"
    ))
    .unwrap()
}

pub const EXPRESSION: &str = "expression:acceptance";
pub const SOURCE: &str = "expression:acceptance:scene:canonical";
pub const OUTPUT: &str = "expression:acceptance:scene:passage";
pub const SHARED: &str = "expression:acceptance:scene:shared";

pub fn source() -> NativeSceneSource {
    let p = procedure();
    // Production constructor gives ALL template constituents/references their
    // native namespace; no partial handwritten entity occurrence map.
    let material = instantiate_scene(
        &p,
        EXPRESSION,
        "canonical-material",
        SOURCE,
        &[subject()],
        &presentation(),
    )
    .unwrap()
    .generated_basis;
    NativeSceneSource {
        native_owner: "oi.expression".into(),
        expression_ref: EXPRESSION.into(),
        scene_ref: SOURCE.into(),
        document_revision: 3,
        source_basis: p.profile,
        material_fingerprint: fingerprint(&material).unwrap(),
        presentation: material,
        principal: subject(),
        contributors: vec![],
        locus_ref: p.locus_ref,
        locus_revision: "current-native-profile".into(),
    }
}
pub fn entity_refs(material: &Value) -> BTreeMap<String, String> {
    material["scene"]["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            let id = e["id"].as_str().unwrap().to_owned();
            (id.clone(), id)
        })
        .collect()
}
pub fn authored(p: &Procedure) -> ProcedureAuthorship {
    ProcedureAuthorship {
        procedure_ref: p.procedure_ref.clone(),
        revision: p.revision.clone(),
        composition: p.composition.clone(),
        trigger: p.trigger.clone(),
        selector: p.selector.clone(),
        conditions: p.conditions.clone(),
        recipe_parameters: BTreeMap::new(),
        membership_mode: p.membership_mode,
        membership_change_policy: p.membership_change_policy.clone(),
        timing: p.timing.clone(),
        budgets: p.budgets.clone(),
        seed: p.seed.clone(),
        removal_policy: p.removal_policy,
        failure_policy: p.failure_policy,
        continuation_policy: p.continuation_policy,
    }
}
pub fn whole() -> OwnedAddress {
    OwnedAddress {
        expression_ref: EXPRESSION.into(),
        scene_ref: None,
        entity_ref: None,
        component: "expression".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    }
}
pub fn flow(scene: &str, order: &[&str]) -> Value {
    json!({"schema":"ql.native-atlas-state/v1","expression_ref":EXPRESSION,
        "focus":{"scene_ref":scene,"entity_ref":null},"scene_order":order})
}
pub fn scalar_readings(source: &NativeSceneSource) -> Vec<TargetReading> {
    let entity = source.presentation["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap();
    let mut readings = Vec::new();
    for scene in [SOURCE, SHARED] {
        for (parameter, value) in [
            ("force_radius", json!(120)),
            ("force_strength", json!(0.15)),
        ] {
            let address = native_parameter_address(EXPRESSION, scene, entity, parameter).unwrap();
            readings.push(TargetReading {
                occurrence_ref: format!("configured-issued:{}", fingerprint(&address).unwrap()),
                address,
                subject: subject(),
                revision: 3,
                tags: vec![],
                properties: BTreeMap::from([(parameter.into(), value)]),
            });
        }
    }
    readings
}
fn retained_scene(source: &NativeSceneSource, scene: &str, revision: u64) -> NativeRetentionScene {
    let mut material = source.presentation.clone();
    material["scene"]["id"] = json!(scene);
    NativeRetentionScene {
        scene_ref: scene.into(),
        document_revision: revision,
        current_presentation: Some(material),
        existing_retention: Some(empty_retention()),
        principal: subject(),
        contributors: vec![],
        locus: NativeReading {
            reference: source.locus_ref.clone(),
            revision: source.locus_revision.clone(),
            availability: ReadingAvailability::Available,
        },
    }
}
pub fn context(p: &mut Procedure) -> Value {
    use ql_mef::vak_scope::OperativeScopeCorrelation;
    use ql_mef::vak_scope_wire::{
        OPERATIVE_CURRENTNESS_CONTRACT, OperativeScopeCurrentnessRequest,
    };
    p.composition.thread = ql_mef::vak_profile::ThreadForm::Single;
    p.composition.sources = vec![p.recipe.source_ref.clone(), p.profile.source_ref.clone()];
    let mut whole: Value = serde_json::from_str::<Value>(include_str!(
        "../../../../fixtures/kernel/vak-composition-v1.json"
    ))
    .unwrap()["steps"][0]
        .clone();
    whole["useRef"] = json!("whole:program-ground");
    whole["subjectRef"] = json!(p.principal_subject_ref);
    whole["frame"]["id"] = json!(p.composition.frame.0.code());
    whole["basis"] = json!({"caller":p.composition.actor,"source":p.recipe.source_ref,"revision":p.recipe.revision,
        "standing":ql_mef::VakStanding::AuthoredArchitecture.as_schema_str(),"evidence":[p.recipe.source_ref]});
    let source = json!({"contract":"ql.vak-composition/v1","steps":[whole,{"op":"reframe","from":"whole:program-ground","into":p.composition.whole,
        "frame":{"id":p.composition.frame.0.code(),"lens":"L0","basis":"chromatic","face":"direct","positions":"local"},
        "basis":{"caller":p.composition.actor,"source":p.profile.source_ref,"revision":p.profile.revision,
        "standing":ql_mef::VakStanding::AuthoredArchitecture.as_schema_str(),"evidence":[p.profile.source_ref]}}]});
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
    let currentness = OperativeScopeCurrentnessRequest {
        contract: OPERATIVE_CURRENTNESS_CONTRACT.into(),
        expected: binding,
        current_whole_ref: p.composition.whole.clone(),
        correlation,
    };
    json!({"source_composition":source,"currentness":currentness,"thread_plan":{
        "legs":[{"unit_ref":"oi.expression:program-leg","subject_ref":p.principal_subject_ref,"scope_ref":p.occurrence_ref,
        "input_refs":[p.recipe.source_ref],"result_ref":"oi.expression:program-result","after":[],"parent":null}],
        "aggregation_ref":null,"continuation_ref":null,"stop_condition_ref":null}})
}
pub struct Fixture {
    pub input: NativeProgramRegeneration,
    pub first: PreparedProcedure,
    pub native_context: Value,
}
pub fn fixture(kind: &str) -> Fixture {
    let source = source();
    let mut base = procedure();
    let mut readings = match kind {
        "force" => scalar_readings(&source),
        "atlas" => vec![TargetReading {
            occurrence_ref: EXPRESSION.into(),
            address: whole(),
            subject: subject(),
            revision: 3,
            tags: vec![],
            properties: BTreeMap::from([(
                "native_atlas_state".into(),
                flow(SOURCE, &[SOURCE, SHARED]),
            )]),
        }],
        "scene" => vec![],
        _ => panic!("unknown configured native recipe"),
    };
    base.selector = Selector::Occurrences {
        refs: readings.iter().map(|r| r.occurrence_ref.clone()).collect(),
    };
    let choice = match kind {
        "force" => LibraryChoice::ForceParameters {
            writes: vec![
                ParameterRecipe {
                    output_slot: "radius".into(),
                    parameter: "force_radius".into(),
                    value: RecipeValue::Constant { value: json!(200) },
                },
                ParameterRecipe {
                    output_slot: "strength".into(),
                    parameter: "force_strength".into(),
                    value: RecipeValue::Constant { value: json!(0.8) },
                },
            ],
        },
        "atlas" => LibraryChoice::AtlasPassage {
            changes: vec![
                NativeChange::Focus {
                    scene_ref: SHARED.into(),
                    entity_ref: None,
                },
                NativeChange::SceneReorder {
                    scene_refs: vec![SHARED.into(), SOURCE.into()],
                },
            ],
        },
        _ => LibraryChoice::SequenceMaterial {
            outputs: vec![OutputSlot {
                output_slot: "passage".into(),
                scene_ref: OUTPUT.into(),
            }],
            holds: vec![SequenceHold {
                entity_ref: source.presentation["scene"]["entities"][0]["id"]
                    .as_str()
                    .unwrap()
                    .into(),
                step_ref: Some(
                    source.presentation["scene"]["entities"][0]["sequence"]["steps"][1]["id"]
                        .as_str()
                        .unwrap()
                        .into(),
                ),
                seconds: 3.5,
            }],
        },
    };
    let built = library_build(LibraryBuild {
        schema: LIBRARY_CONTRACT.into(),
        source: source.clone(),
        authored: authored(&base),
        choice,
    })
    .unwrap();
    let mut procedure: Procedure = serde_json::from_value(built["procedure"].clone()).unwrap();
    procedure.admitted_changes.insert("scene_reorder".into());
    let native_context = context(&mut procedure);
    let program: NativeRecipeProgram = serde_json::from_value(built["program"].clone()).unwrap();
    let membership = resolve_procedure_membership(
        m_tree::native_current_m_registry(),
        &procedure,
        EXPRESSION,
        &readings,
        None,
    )
    .unwrap();
    let contributions = program_contributions(
        m_tree::native_current_m_registry(),
        &procedure,
        &program,
        EXPRESSION,
        &readings,
        &membership,
    )
    .unwrap();
    let mut first = compile_native_batch(
        m_tree::native_current_m_registry(),
        &procedure,
        "operation:original-program",
        EXPRESSION,
        3,
        membership,
        contributions,
        BTreeSet::from(["scene".into()]),
    )
    .unwrap();
    let mut materialization = NativeMaterialization {
        schema: MATERIALIZATION_CONTRACT.into(),
        lifecycles: vec![],
        document_revision: 3,
        rule_cursor: 0,
        state: "held".into(),
        scenes: vec![retained_scene(&source, SOURCE, 3)],
    };
    if kind == "scene" {
        let mut new = retained_scene(&source, OUTPUT, 3);
        new.current_presentation = None;
        new.existing_retention = None;
        materialization.scenes.push(new);
    } else {
        materialization
            .scenes
            .push(retained_scene(&source, SHARED, 3));
    }
    materialize_retention(&mut first, &materialization).unwrap();
    let sources = vec![
        NativeReading {
            reference: procedure.recipe.source_ref.clone(),
            revision: procedure.recipe.revision.clone(),
            availability: ReadingAvailability::Available,
        },
        NativeReading {
            reference: procedure.profile.source_ref.clone(),
            revision: procedure.profile.revision.clone(),
            availability: ReadingAvailability::Available,
        },
    ];
    let envelope = json!({"operation_ref":first.operation_ref,"expression_ref":EXPRESSION,"expected_revision":3,"actor":procedure.composition.actor,
        "scope":{"kind":"expression"},"changes":first.native_edit["changes"],"sources":sources,"participants":[],"timing":{"kind":"immediate"},"cause_ref":null});
    let mut output_readings = Vec::new();
    let mut current = Vec::new();
    for c in &first.contributions {
        let actual = if kind == "scene" {
            c.generated_basis.clone()
        } else if kind == "force" {
            let mut value = c.generated_basis["authored_basis"].clone();
            value["value"] = c.generated_basis["value"].clone();
            value["target_revision"] = json!(5);
            value
        } else {
            flow(SHARED, &[SHARED, SOURCE])
        };
        current.push(CurrentContribution {
            contribution_ref: c.contribution_ref.clone(),
            material: actual.clone(),
            overlays: vec![],
        });
        output_readings.push(RetainedOutputReading {schema:RETAINED_OUTPUT_READING.into(),native_owner:"oi.expression".into(),expression_ref:EXPRESSION.into(),
            document_revision:6,procedure_ref:procedure.procedure_ref.clone(),source_basis:vec![procedure.recipe.clone(),procedure.profile.clone()],
            origin_source_basis:Some(sources.clone()),contribution_ref:c.contribution_ref.clone(),output_slot:c.output_slot.clone(),subject_refs:c.subjects.clone(),
            occurrence_ref:c.occurrence_ref.clone(),recipe_revision:c.recipe.revision.clone(),owned_addresses:c.owned_addresses.clone(),generated_basis:c.generated_basis.clone(),
            current_basis:actual,status:"active".into(),applied_operation:json!({"fingerprint":fingerprint(&envelope).unwrap(),"envelope":envelope,
                "targets":first.contributions.iter().flat_map(|c|c.owned_addresses.clone()).collect::<Vec<_>>(),"status":"applied","accepted_revision":4,
                "applied_revision":5,"observations":[],"failure":null})});
    }
    materialization.document_revision = 6;
    materialization.rule_cursor = 1;
    for scene in &mut materialization.scenes {
        scene.document_revision = 6;
        let change = first.native_edit["changes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["change"] == "scene_material_set" && c["scene_ref"] == scene.scene_ref)
            .unwrap();
        let mut material = change["presentation"].clone();
        scene.existing_retention = Some(
            material["scene"]
                .as_object_mut()
                .unwrap()
                .remove("procedural")
                .unwrap(),
        );
        scene.current_presentation = Some(material);
    }
    for r in &mut readings {
        r.revision = 5;
        if let Some(v) = r.properties.get_mut("force_radius") {
            *v = json!(200);
        }
        if let Some(v) = r.properties.get_mut("force_strength") {
            *v = json!(0.8);
        }
        if let Some(v) = r.properties.get_mut("native_atlas_state") {
            *v = flow(SHARED, &[SHARED, SOURCE]);
        }
    }
    Fixture {
        input: NativeProgramRegeneration {
            procedure,
            program,
            expression_ref: EXPRESSION.into(),
            document_revision: 6,
            operation_ref: "operation:regenerate-program".into(),
            current_readings: readings,
            previous_membership: Some(first.membership.clone()),
            previous: first.contributions.clone(),
            current,
            output_readings,
            intervention_contexts: vec![],
            materialization,
            required_consumers: BTreeSet::from(["scene".into()]),
        },
        first,
        native_context,
    }
}

pub fn force_human_fixture() -> Fixture {
    let mut f = fixture("force");
    let index = f
        .input
        .previous
        .iter()
        .position(|c| c.generated_basis["parameter"] == "force_radius")
        .unwrap();
    let old = f.input.previous[index].clone();
    for scene in &mut f.input.materialization.scenes {
        let before = scene.current_presentation.clone().unwrap();
        let mut after = before.clone();
        after["scene"]["entities"][0]["force"]["radius"] = json!(0.8);
        let edit = ManualEdit {
            expression_ref: EXPRESSION.into(),
            scene_ref: scene.scene_ref.clone(),
            contribution_ref: old.contribution_ref.clone(),
            owned_addresses: old.owned_addresses.clone(),
            entity_refs: entity_refs(&before),
            before,
            after: after.clone(),
            actor_ref: "human:owner".into(),
            operation_ref: "operation:ordinary-radius".into(),
            document_revision: 6,
            retained_overlays: vec![],
            retained_native_records: vec![],
        };
        let records = extract_owned_manual_interventions(&edit)
            .unwrap()
            .native_records;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].value, json!(0.8));
        let retention = scene.existing_retention.as_mut().unwrap();
        let row = retention["contributions"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|r| r["contribution_ref"] == old.contribution_ref)
            .unwrap();
        row["authored_overrides"] = json!(records);
        scene.current_presentation = Some(after.clone());
        f.input
            .intervention_contexts
            .push(NativeInterventionBasis::Scene {
                basis: SceneInterventionBasis {
                    expression_ref: EXPRESSION.into(),
                    scene_ref: scene.scene_ref.clone(),
                    contribution_ref: old.contribution_ref.clone(),
                    owned_addresses: old.owned_addresses.clone(),
                    entity_refs: entity_refs(&after),
                    current_presentation: after,
                    document_revision: 6,
                    retained_native_records: records,
                },
            });
    }
    let mut actual = f.input.current[index].material.clone();
    actual["value"] = json!(320);
    actual["target_revision"] = json!(6);
    f.input.current[index].material = actual.clone();
    f.input.output_readings[index].current_basis = actual;
    for reading in &mut f.input.current_readings {
        reading.revision = 6;
        if let Some(value) = reading.properties.get_mut("force_radius") {
            *value = json!(320);
        }
    }
    f
}

pub fn scene_human_fixture() -> Fixture {
    let mut f = fixture("scene");
    let old = f.input.previous[0].clone();
    let before = f.input.current[0].material.clone();
    let mut actual = before.clone();
    actual["scene"]["entities"][0]["force"]["strength"] = json!(0.875);
    let edit = ManualEdit {
        expression_ref: EXPRESSION.into(),
        scene_ref: OUTPUT.into(),
        contribution_ref: old.contribution_ref.clone(),
        owned_addresses: old.owned_addresses.clone(),
        entity_refs: entity_refs(&before),
        before,
        after: actual.clone(),
        actor_ref: "human:owner".into(),
        operation_ref: "operation:human-strength".into(),
        document_revision: 6,
        retained_overlays: vec![],
        retained_native_records: vec![],
    };
    let records = extract_owned_manual_interventions(&edit)
        .unwrap()
        .native_records;
    assert_eq!(records.len(), 1);
    f.input
        .intervention_contexts
        .push(NativeInterventionBasis::Scene {
            basis: SceneInterventionBasis {
                expression_ref: EXPRESSION.into(),
                scene_ref: OUTPUT.into(),
                contribution_ref: old.contribution_ref.clone(),
                owned_addresses: old.owned_addresses.clone(),
                entity_refs: entity_refs(&actual),
                current_presentation: actual.clone(),
                document_revision: 6,
                retained_native_records: records.clone(),
            },
        });
    let scene = f
        .input
        .materialization
        .scenes
        .iter_mut()
        .find(|s| s.scene_ref == OUTPUT)
        .unwrap();
    scene.current_presentation = Some(actual.clone());
    scene.existing_retention.as_mut().unwrap()["contributions"][0]["authored_overrides"] =
        json!(records);
    f.input.current[0].material = actual.clone();
    f.input.output_readings[0].current_basis = actual;
    let NativeRecipeProgram::SceneMaterial { outputs } = &mut f.input.program else {
        panic!("Scene program")
    };
    outputs[0].sequence_holds[0].seconds = 7.25;
    outputs[0].source.document_revision = 6;
    f.input.procedure.revision = "2".into();
    f.input
        .procedure
        .recipe_parameters
        .insert("native_program".into(), json!(f.input.program));
    f
}

pub fn atlas_human_fixture() -> Fixture {
    let mut f = fixture("atlas");
    let old = f.input.previous[0].clone();
    let actual = flow(SOURCE, &[SOURCE, SHARED]);
    let records = extract_flow_interventions(&FlowManualEdit {
        expression_ref: EXPRESSION.into(),
        contribution_ref: old.contribution_ref.clone(),
        owned_addresses: old.owned_addresses.clone(),
        before: f.input.current[0].material.clone(),
        after: actual.clone(),
        actor_ref: "human:atlas".into(),
        operation_ref: "operation:human-navigation".into(),
        document_revision: 6,
        retained_native_records: vec![],
    })
    .unwrap()
    .native_records;
    assert_eq!(records.len(), 2);
    for scene in &mut f.input.materialization.scenes {
        scene.existing_retention.as_mut().unwrap()["contributions"][0]["authored_overrides"] =
            json!(records);
    }
    f.input
        .intervention_contexts
        .push(NativeInterventionBasis::NativeFlow {
            basis: FlowInterventionBasis {
                expression_ref: EXPRESSION.into(),
                contribution_ref: old.contribution_ref.clone(),
                owned_addresses: old.owned_addresses.clone(),
                current_state: actual.clone(),
                document_revision: 6,
                retained_native_records: records,
            },
        });
    f.input.current[0].material = actual.clone();
    f.input.output_readings[0].current_basis = actual.clone();
    f.input.current_readings[0].revision = 6;
    f.input.current_readings[0]
        .properties
        .insert("native_atlas_state".into(), actual);
    f
}

/// Full original retained Scene basis with no event/metadata advancement.
/// This is a configured native compiler operand, not an installed rule grant.
pub fn unchanged_scene_fixture() -> Fixture {
    let mut f = fixture("scene");
    let retained = f.input.materialization.scenes[0]
        .existing_retention
        .as_ref()
        .unwrap();
    f.input.materialization.rule_cursor = retained["procedures"][0]["cursor"].as_u64().unwrap();
    f.input.materialization.state = retained["procedures"][0]["state"].as_str().unwrap().into();
    f
}

/// The operand and write retain separate exact scalar coordinates. This remains
/// a configured native compiler fixture, not an owner-issued live reading.
pub fn cross_row_force_fixture() -> Fixture {
    let mut f = fixture("force");
    let NativeRecipeProgram::ForceParameters { writes } = &mut f.input.program else {
        unreachable!()
    };
    writes.retain(|w| w.parameter == "force_strength");
    writes[0].value = RecipeValue::NativeProperty {
        property: "force_radius".into(),
    };
    f.input
        .procedure
        .recipe_parameters
        .insert("native_program".into(), json!(f.input.program));
    for reading in &mut f.input.current_readings {
        if let Some(value) = reading.properties.get_mut("force_radius") {
            *value = json!(8);
        }
    }
    for current in &mut f.input.current {
        if current.material["parameter"] == "force_radius" {
            current.material["value"] = json!(8);
        }
    }
    for reading in &mut f.input.output_readings {
        if reading.current_basis["parameter"] == "force_radius" {
            reading.current_basis["value"] = json!(8);
        }
    }
    f
}

/// Rebuild the original actual compiler preparation with one selected Source
/// Scene. This proves that retained historical joins are not fresh events.
pub fn selected_unchanged_scene_fixture() -> Fixture {
    let mut f = unchanged_scene_fixture();
    let address = OwnedAddress {
        expression_ref: EXPRESSION.into(),
        scene_ref: Some(SOURCE.into()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    let mut target = TargetReading {
        occurrence_ref: SOURCE.into(),
        address,
        subject: subject(),
        revision: 3,
        tags: vec![],
        properties: BTreeMap::new(),
    };
    f.input.procedure.selector = Selector::Occurrences {
        refs: vec![SOURCE.into()],
    };
    let registry = m_tree::native_current_m_registry();
    let membership = resolve_procedure_membership(
        registry,
        &f.input.procedure,
        EXPRESSION,
        std::slice::from_ref(&target),
        None,
    )
    .unwrap();
    let contributions = program_contributions(
        registry,
        &f.input.procedure,
        &f.input.program,
        EXPRESSION,
        std::slice::from_ref(&target),
        &membership,
    )
    .unwrap();
    let mut first = compile_native_batch(
        registry,
        &f.input.procedure,
        "operation:original-program",
        EXPRESSION,
        3,
        membership,
        contributions,
        f.input.required_consumers.clone(),
    )
    .unwrap();
    let mut creation = f.input.materialization.clone();
    creation.document_revision = 3;
    for scene in &mut creation.scenes {
        scene.document_revision = 3;
        if scene.scene_ref == OUTPUT {
            scene.current_presentation = None;
            scene.existing_retention = None;
        } else {
            scene.existing_retention = Some(empty_retention());
        }
    }
    materialize_retention(&mut first, &creation).unwrap();
    let mut operation = f.input.output_readings[0].applied_operation.clone();
    operation["envelope"]["changes"] = first.native_edit["changes"].clone();
    operation["targets"] = json!(
        first
            .membership
            .addresses
            .values()
            .cloned()
            .chain(
                first
                    .contributions
                    .iter()
                    .flat_map(|c| c.owned_addresses.clone())
            )
            .chain(first.metadata_scope.clone())
            .collect::<BTreeSet<_>>()
    );
    operation["fingerprint"] = json!(fingerprint(&operation["envelope"]).unwrap());
    for scene in &mut f.input.materialization.scenes {
        let change = first.native_edit["changes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["change"] == "scene_material_set" && c["scene_ref"] == scene.scene_ref)
            .unwrap();
        let mut material = change["presentation"].clone();
        scene.existing_retention = Some(
            material["scene"]
                .as_object_mut()
                .unwrap()
                .remove("procedural")
                .unwrap(),
        );
        scene.current_presentation = Some(material);
    }
    for (reading, contribution) in f.input.output_readings.iter_mut().zip(&first.contributions) {
        assert_eq!(reading.contribution_ref, contribution.contribution_ref);
        reading.generated_basis = contribution.generated_basis.clone();
        reading.applied_operation = operation.clone();
    }
    target.revision = 5;
    f.input.current_readings = vec![target];
    f.input.previous = first.contributions.clone();
    f.input.previous_membership = Some(first.membership.clone());
    f.first = first;
    f
}
