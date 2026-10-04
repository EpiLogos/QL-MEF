//! Real native program generation, retained-output compiler and intervention
//! projection. Configured journals are consistency operands, never live grants.
#[path = "support/procedural_program.rs"]
mod support;
use ql_mef::{m_tree, procedural_composition::*, procedural_conduct::*};
use serde_json::{Value, json};
use support::*;

fn regenerate(f: &Fixture) -> NativeProgramRegenerationResult {
    prepare_program_regeneration(m_tree::native_current_m_registry(), f.input.clone()).unwrap()
}
fn write_changes(prepared: &PreparedProcedure) -> &[Value] {
    prepared.native_edit["changes"].as_array().unwrap()
}

#[test]
fn separated_native_scalar_rows_compile_each_write_once_and_preserve_original_selector() {
    let f = fixture("force");
    assert_eq!(f.input.current_readings.len(), 4);
    assert!(
        f.input
            .current_readings
            .iter()
            .all(|r| r.properties.len() == 1)
    );
    let result = regenerate(&f);
    let prepared = result.prepared.unwrap();
    assert_eq!(prepared.membership, f.first.membership);
    assert_eq!(
        prepared.original_procedure.selector,
        f.input.procedure.selector
    );
    assert_eq!(prepared.contributions.len(), 2);
    let mut qualified = prepared.clone();
    let (_, graph) =
        ql_mef::vak_composition_wire::compile_request(&f.native_context["source_composition"])
            .unwrap();
    qualified
        .qualify_native_cprime(
            m_tree::native_current_m_registry(),
            &f.input.procedure,
            &graph,
            serde_json::from_value(f.native_context["currentness"].clone()).unwrap(),
            serde_json::from_value(f.native_context["thread_plan"].clone()).unwrap(),
        )
        .unwrap();
    assert!(qualified.native_cprime.is_some());
    let mut identity = qualified.clone();
    identity.fingerprint.clear();
    assert_eq!(
        ql_mef::procedural_manifestation::fingerprint(&identity).unwrap(),
        qualified.fingerprint
    );

    assert!(
        prepared
            .contributions
            .iter()
            .all(|c| c.owned_addresses.len() == 2)
    );
    assert_eq!(
        write_changes(&prepared)
            .iter()
            .filter(|c| c["change"] == "parameter_set")
            .count(),
        2
    );
    for c in &prepared.contributions {
        let old = f
            .first
            .contributions
            .iter()
            .find(|old| old.output_slot == c.output_slot)
            .unwrap();
        assert_eq!(c.contribution_ref, old.contribution_ref);
        assert_eq!(
            c.generated_basis["authored_basis"],
            old.generated_basis["authored_basis"]
        );
    }
    let mut reversed = f.input.clone();
    reversed.current_readings.reverse();
    assert_eq!(
        prepared,
        prepare_program_regeneration(m_tree::native_current_m_registry(), reversed)
            .unwrap()
            .prepared
            .unwrap()
    );
    let mut wrong = f.input.clone();
    wrong
        .current_readings
        .retain(|r| r.address.property.as_deref() == Some("radius"));
    wrong.previous_membership = None;
    wrong.procedure.selector = Selector::Occurrences {
        refs: wrong
            .current_readings
            .iter()
            .map(|r| r.occurrence_ref.clone())
            .collect(),
    };
    let before = json!(wrong);
    assert!(
        prepare_program_regeneration(m_tree::native_current_m_registry(), wrong.clone())
            .unwrap_err()
            .contains("no selected scalar")
    );
    assert_eq!(json!(wrong), before);
}

#[test]
fn force_regeneration_keeps_human_native_units_and_recipe_basis_distinct() {
    let f = force_human_fixture();
    let prepared = regenerate(&f).prepared.unwrap();
    let radius = prepared
        .contributions
        .iter()
        .find(|c| c.generated_basis["parameter"] == "force_radius")
        .unwrap();
    assert_eq!(radius.generated_basis["value"], 200);
    assert_eq!(radius.generated_basis["authored_basis"]["value"], 120);
    let writes = write_changes(&prepared);
    assert_eq!(
        writes
            .iter()
            .find(|c| c["change"] == "parameter_set" && c["parameter"] == "force_radius")
            .unwrap()["value"],
        320
    );
    assert_eq!(writes[0]["change"], "scene_material_set");
    assert_eq!(writes[1]["change"], "scene_material_set");
    for material in writes
        .iter()
        .filter(|c| c["change"] == "scene_material_set")
    {
        let row = material["presentation"]["scene"]["procedural"]["contributions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["contribution_ref"] == radius.contribution_ref)
            .unwrap();
        assert_eq!(row["authored_overrides"][0]["value"], 0.8);
        assert_eq!(row["authored_overrides"][0]["actor"], "human:owner");
        assert_eq!(
            material["presentation"]["scene"]["procedural"]["operations"],
            json!([])
        );
    }
    let mut wrong = f.input.clone();
    wrong.intervention_contexts.pop();
    assert!(
        prepare_program_regeneration(m_tree::native_current_m_registry(), wrong)
            .unwrap_err()
            .contains("omit")
    );
    let mut wrong = f.input.clone();
    let reference = wrong.current[0].contribution_ref.clone();
    wrong.current[0].overlays.push(AuthoredOverlay {
        contribution_ref: reference,
        pointer: "/value".into(),
        value: json!(999),
        actor_ref: "caller".into(),
        persistent: true,
        operation: OverlayOperation::Set,
    });
    assert!(
        prepare_program_regeneration(m_tree::native_current_m_registry(), wrong)
            .unwrap_err()
            .contains("caller supplied")
    );
}

#[test]
fn scene_program_regeneration_keeps_human_strength_and_native_sequence_override() {
    let f = scene_human_fixture();
    let result = regenerate(&f);
    let prepared = result.prepared.unwrap();
    let effective =
        &result.regeneration.unwrap().effective_basis[&f.input.previous[0].contribution_ref];
    assert_eq!(
        effective["scene"]["entities"][0]["force"]["strength"],
        0.875
    );
    assert_eq!(
        effective["scene"]["entities"][0]["sequence"]["steps"][1]["hold"],
        7.25
    );
    assert_eq!(
        effective["scene"]["entities"][0]["sequence"]["steps"][1]["holdOverride"],
        true
    );
    assert_eq!(prepared.membership, f.first.membership);
    assert!(prepared.membership.addresses.is_empty());
    assert_eq!(
        prepared.contributions[0].contribution_ref,
        f.input.previous[0].contribution_ref
    );
    assert!(
        !write_changes(&prepared)
            .iter()
            .any(|c| c["change"] == "scene_create" || c["change"] == "entity_add")
    );
    let mut wrong = f.input.clone();
    wrong.output_readings[0].document_revision = 5;
    assert!(prepare_program_regeneration(m_tree::native_current_m_registry(), wrong).is_err());
    let mut wrong = f.input.clone();
    wrong.procedure.recipe_parameters.remove("native_program");
    assert!(
        prepare_program_regeneration(m_tree::native_current_m_registry(), wrong)
            .unwrap_err()
            .contains("native program differs")
    );
}

#[test]
fn atlas_program_regeneration_preserves_actual_human_selection_and_order() {
    let f = atlas_human_fixture();
    let prepared = regenerate(&f).prepared.unwrap();
    let writes = write_changes(&prepared);
    assert_eq!(
        writes.iter().find(|c| c["change"] == "focus").unwrap()["scene_ref"],
        SOURCE
    );
    assert_eq!(
        writes
            .iter()
            .find(|c| c["change"] == "scene_reorder")
            .unwrap()["scene_refs"],
        json!([SOURCE, SHARED])
    );
    assert_eq!(
        prepared.contributions[0].generated_basis["native_flow"][0]["scene_ref"],
        SHARED
    );
    assert_eq!(
        prepared.contributions[0].generated_basis["authored_basis"]["focus"]["scene_ref"],
        SOURCE
    );
    assert_eq!(prepared.membership, f.first.membership);
    let mut wrong = f.input.clone();
    wrong.output_readings[0].current_basis["expression_ref"] = json!("foreign:expression");
    assert!(prepare_program_regeneration(m_tree::native_current_m_registry(), wrong).is_err());
}

#[test]
fn cross_row_native_operand_uses_only_the_original_same_entity_scalar_locations() {
    let f = fixture("force");
    let mut procedure = f.input.procedure.clone();
    let mut program = f.input.program.clone();
    let NativeRecipeProgram::ForceParameters { writes } = &mut program else {
        unreachable!()
    };
    writes.retain(|w| w.parameter == "force_strength");
    writes[0].value = RecipeValue::NativeProperty {
        property: "force_radius".into(),
    };
    procedure
        .recipe_parameters
        .insert("native_program".into(), json!(program));
    let mut readings = f.input.current_readings.clone();
    for r in &mut readings {
        if r.address.property.as_deref() == Some("radius") {
            r.properties.insert("force_radius".into(), json!(8));
        }
    }
    assert!(readings.iter().all(|r| r.properties.len() == 1));
    let contributions = program_contributions(
        m_tree::native_current_m_registry(),
        &procedure,
        &program,
        EXPRESSION,
        &readings,
        &f.first.membership,
    )
    .unwrap();
    assert_eq!(contributions.len(), 1);
    assert_eq!(contributions[0].generated_basis["value"], 8);
    assert_eq!(contributions[0].owned_addresses.len(), 2);
    assert!(
        contributions[0]
            .owned_addresses
            .iter()
            .all(|a| a.property.as_deref() == Some("strength"))
    );
    assert_eq!(
        contributions[0].native_changes,
        vec![NativeChange::ParameterSet {
            entity_ref: f.input.current_readings[0]
                .address
                .entity_ref
                .clone()
                .unwrap(),
            parameter: "force_strength".into(),
            value: json!(8)
        }]
    );
    for mutation in 0..7 {
        let mut wrong = readings.clone();
        let radius = wrong
            .iter()
            .position(|r| r.address.property.as_deref() == Some("radius"))
            .unwrap();
        match mutation {
            0 => {
                wrong.remove(radius);
            }
            1 => wrong[radius].revision += 1,
            2 => wrong[radius].subject.subject_ref = "subject:foreign".into(),
            3 => {
                wrong[radius]
                    .properties
                    .insert("force_radius".into(), json!(9));
            }
            4 => wrong[radius].address.entity_ref = Some(format!("{EXPRESSION}:entity:foreign")),
            5 => {
                wrong[radius].address.property = Some("spin".into());
            }
            _ => wrong.push(wrong[radius].clone()),
        }
        let before = json!(wrong);
        assert!(
            program_contributions(
                m_tree::native_current_m_registry(),
                &procedure,
                &program,
                EXPRESSION,
                &wrong,
                &f.first.membership
            )
            .is_err()
        );
        assert_eq!(json!(wrong), before);
    }
    let mut original_scalar_only = f.first.membership.clone();
    original_scalar_only
        .addresses
        .retain(|_, a| a.property.as_deref() == Some("strength"));
    assert!(
        program_contributions(
            m_tree::native_current_m_registry(),
            &procedure,
            &program,
            EXPRESSION,
            &readings,
            &original_scalar_only
        )
        .unwrap_err()
        .contains("original selected scalar scope")
    );
    assert_eq!(
        f.input.current_readings,
        fixture("force").input.current_readings
    );
}

#[test]
fn unchanged_scene_is_no_change_only_after_exact_full_retained_metadata_comparison() {
    let f = unchanged_scene_fixture();
    let before = json!(f.input);
    let result = regenerate(&f);
    assert_eq!(result.outcome, NativeProgramRegenerationOutcome::NoChange);
    assert!(result.prepared.is_none());
    assert_eq!(result.membership, f.first.membership);
    let basis = result.no_change_basis.unwrap();
    assert_eq!(basis.original_procedure, f.input.procedure);
    assert_eq!(basis.document_revision, f.input.document_revision);
    assert_eq!(basis.operation_ref, f.input.operation_ref);
    assert_eq!(
        basis.current_inputs_fingerprint,
        ql_mef::procedural_manifestation::fingerprint(&f.input).unwrap()
    );
    assert_eq!(json!(f.input), before);
    for mutation in 0..10 {
        let mut input = f.input.clone();
        match mutation {
            0 => input.procedure.revision = "2".into(),
            1 => input.procedure.seed = "new-authored-seed".into(),
            2 => input.procedure.removal_policy = RemovalPolicy::ConflictOnEdited,
            3 => {
                input
                    .procedure
                    .recipe_parameters
                    .insert("authored_policy".into(), json!("changed"));
            }
            4 => input.materialization.rule_cursor += 1,
            5 => input.materialization.state = "running".into(),
            6 => {
                input.procedure.selector = Selector::All;
                input.previous_membership = None;
            }
            7 => input.procedure.trigger.kind = TriggerKind::SourceChanged,
            8 => input.procedure.continuation_policy = ProcedureContinuation::Hold,
            _ => input.procedure.conditions.push(Condition::Equals {
                property: "authored_condition".into(),
                value: json!(true),
            }),
        }
        let result =
            prepare_program_regeneration(m_tree::native_current_m_registry(), input.clone())
                .unwrap();
        assert_eq!(result.outcome, NativeProgramRegenerationOutcome::Prepared);
        assert!(result.no_change_basis.is_none());
        let prepared = result.prepared.unwrap();
        assert_eq!(prepared.original_procedure, input.procedure);
        assert!(
            write_changes(&prepared)
                .iter()
                .all(|c| c["change"] == "scene_material_set")
        );
        let row = &write_changes(&prepared)
            .iter()
            .find(|c| c["scene_ref"] == OUTPUT)
            .unwrap()["presentation"]["scene"]["procedural"]["procedures"][0];
        assert_eq!(row["definition"], json!(input.procedure));
        assert_eq!(row["cursor"], input.materialization.rule_cursor);
        assert_eq!(row["state"], input.materialization.state);
    }
}

#[test]
fn unchanged_selected_scene_retains_original_join_event_without_reobserving_it_as_new() {
    let f = selected_unchanged_scene_fixture();
    assert_eq!(f.first.membership.joined, vec![SOURCE.to_owned()]);
    let before = json!(f.input);
    let source_row = &f
        .input
        .materialization
        .scenes
        .iter()
        .find(|s| s.scene_ref == SOURCE)
        .unwrap()
        .existing_retention
        .as_ref()
        .unwrap()["procedures"][0];
    assert_eq!(source_row["membership_events"].as_array().unwrap().len(), 1);
    assert_eq!(source_row["membership_events"][0]["document_revision"], 3);
    let result = regenerate(&f);
    assert_eq!(result.outcome, NativeProgramRegenerationOutcome::NoChange);
    assert!(result.prepared.is_none());
    assert_eq!(result.membership, f.first.membership);
    let mut changed = f.input.clone();
    changed.procedure.revision = "2".into();
    let result =
        prepare_program_regeneration(m_tree::native_current_m_registry(), changed).unwrap();
    let prepared = result.prepared.unwrap();
    for change in write_changes(&prepared) {
        assert_eq!(
            change["presentation"]["scene"]["procedural"]["procedures"][0]["membership_events"],
            source_row["membership_events"]
        );
    }
    assert_eq!(json!(f.input), before);
}

/// Configuration test of the actual Vak/C-prime compiler only. This does not
/// create a Source observation, native timing witness or producer admission.
#[test]
fn fresh_graph_read_identity_reobserves_the_original_semantic_binding_without_restamping() {
    let mut original = procedure();
    let context = context(&mut original);
    let retained_original = original.clone();
    let request: ql_mef::vak_scope_wire::OperativeScopeCurrentnessRequest =
        serde_json::from_value(context["currentness"].clone()).unwrap();
    let plan = serde_json::from_value(context["thread_plan"].clone()).unwrap();
    let (_, initial_graph) =
        ql_mef::vak_composition_wire::compile_request(&context["source_composition"]).unwrap();
    let first = prepare_native_cprime(
        m_tree::native_current_m_registry(),
        &original,
        &initial_graph,
        request.clone(),
        plan,
    )
    .unwrap();

    // Bootstrap's intermediate use identity includes its full Scene read.
    // A new issued read can therefore have a different use, while the final
    // whole/profile/exact material source basis stays semantically unchanged.
    let mut fresh_wire = context["source_composition"].clone();
    let original_use = fresh_wire["steps"][0]["useRef"]
        .as_str()
        .unwrap()
        .to_owned();
    let fresh_use = format!("{original_use}:fresh-issued-read");
    fresh_wire["steps"][0]["useRef"] = json!(fresh_use);
    fresh_wire["steps"][1]["from"] = json!(fresh_use);
    assert_ne!(fresh_wire, context["source_composition"]);
    let (_, fresh_graph) = ql_mef::vak_composition_wire::compile_request(&fresh_wire).unwrap();
    assert!(initial_graph.whole(&fresh_use).is_err());
    assert!(fresh_graph.whole(&original_use).is_err());
    let binding = fresh_graph
        .bind_operative_scope(
            &original.composition.whole,
            original.composition.profile(),
            request.correlation.clone(),
        )
        .unwrap();
    assert_eq!(binding, request.expected);
    let refreshed = prepare_native_cprime(
        m_tree::native_current_m_registry(),
        &original,
        &fresh_graph,
        request.clone(),
        serde_json::from_value(context["thread_plan"].clone()).unwrap(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(refreshed).unwrap(),
        serde_json::to_value(first).unwrap()
    );
    assert_eq!(original, retained_original);

    // Actual semantic source changes require explicit replacement. Changing
    // the current graph never authorizes rewriting the immutable interpretation.
    for index in [0, 1] {
        let mut changed = fresh_wire.clone();
        changed["steps"][index]["basis"]["revision"] = json!("different-native-source-revision");
        let (_, graph) = ql_mef::vak_composition_wire::compile_request(&changed).unwrap();
        assert!(
            prepare_native_cprime(
                m_tree::native_current_m_registry(),
                &original,
                &graph,
                request.clone(),
                serde_json::from_value(context["thread_plan"].clone()).unwrap(),
            )
            .is_err()
        );
        assert_eq!(original, retained_original);
    }
}
