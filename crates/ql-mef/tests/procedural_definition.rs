//! Actual configured native compiler and first-batch configuration tests.
//! No private timing/Scene factory, native install, Source admission or ACK is
//! constructed. Genuine owner installation/continuation uses the capture gate.
#[path = "support/procedural_program.rs"]
mod support;
use ql_mef::{m_tree, procedural_composition::*, procedural_conduct::*};
use serde_json::json;

fn configuration(kind: &str) -> (ConductInstall, PreparedProcedure, NativePosition) {
    let f = support::fixture(kind);
    let mut prepared = f.first;
    let context = f.native_context;
    let procedure = prepared.original_procedure.clone();
    let readings = if kind == "force" {
        support::scalar_readings(&support::source())
    } else {
        vec![]
    };
    let definition = ConductInstall {
        schema: CONDUCT_CONTRACT.into(),
        procedure: procedure.clone(),
        program: f.input.program,
        expression_ref: support::EXPRESSION.into(),
        document_revision: prepared.expected_document_revision,
        current_readings: readings,
        source_composition: context["source_composition"].clone(),
        currentness: serde_json::from_value(context["currentness"].clone()).unwrap(),
        thread_plan: serde_json::from_value(context["thread_plan"].clone()).unwrap(),
        required_consumers: prepared.required_consumers.clone(),
        interval_ref: "configured:first-batch".into(),
        materialization: None,
    };
    let (_, graph) =
        ql_mef::vak_composition_wire::compile_request(&definition.source_composition).unwrap();
    prepared
        .qualify_native_cprime(
            m_tree::native_current_m_registry(),
            &procedure,
            &graph,
            definition.currentness.clone(),
            definition.thread_plan.clone(),
        )
        .unwrap();
    let position = NativePosition {
        instance_ref: "configured:field".into(),
        event_ref: "configured:event".into(),
        subject_ref: procedure.principal_subject_ref,
        generation: "1".into(),
        samples_elapsed: "42".into(),
    };
    (definition, prepared, position)
}

#[test]
fn actual_first_scene_and_force_batches_remain_exact_pending_seeds() {
    for kind in ["scene", "force"] {
        let (definition, prepared, position) = configuration(kind);
        let original = json!((&definition, &prepared, &position));
        let checkpoint = definition::prepare_native_definition_configuration(
            &definition,
            &prepared,
            &position,
            42,
        )
        .unwrap();
        assert_eq!(checkpoint.definition.procedure, prepared.original_procedure);
        assert_eq!(checkpoint.membership, prepared.membership);
        assert_eq!(checkpoint.last_generated, prepared.contributions);
        assert_eq!(
            checkpoint.pending_operation_ref.as_deref(),
            Some(prepared.operation_ref.as_str())
        );
        assert_eq!(checkpoint.pending_preparation.as_ref(), Some(&prepared));
        assert_eq!(checkpoint.rule.cursor, 42);
        assert_eq!(
            checkpoint.rule.operations,
            prepared.native_edit["changes"].as_array().unwrap().len()
        );
        assert!(checkpoint.events.is_empty());
        assert!(checkpoint.original_source.is_none());
        assert!(checkpoint.original_consumer_contract.is_none());
        assert_eq!(json!((&definition, &prepared, &position)), original);
    }
}
#[test]
fn initial_configuration_refuses_changed_definition_cas_and_native_seal() {
    let (definition, prepared, position) = configuration("scene");
    let original = json!((&definition, &prepared));
    for key in [
        "revision",
        "seed",
        "actor",
        "timing",
        "cas",
        "scope",
        "native_fingerprint",
    ] {
        let mut d = definition.clone();
        let mut p = prepared.clone();
        match key {
            "revision" => d.procedure.revision = "other".into(),
            "seed" => d.procedure.seed = "other".into(),
            "actor" => d.procedure.composition.actor = "agent:other".into(),
            "timing" => d.procedure.timing.epoch_ref = "epoch:other".into(),
            "cas" => d.document_revision += 1,
            "scope" => d.procedure.selector = Selector::All,
            "native_fingerprint" => p.fingerprint = "substituted".into(),
            _ => unreachable!(),
        }
        assert!(
            definition::prepare_native_definition_configuration(&d, &p, &position, 42).is_err(),
            "{key}"
        );
    }
    assert_eq!(json!((&definition, &prepared)), original);
}
#[test]
fn serialized_configuration_cannot_mint_private_initial_or_continued_install() {
    let (definition, prepared, position) = configuration("scene");
    let request = json!({"action":"install_prepared","input":{
        "schema":"ql.native-procedural-prepared-install/v1","definition":definition,
        "original_preparation":prepared,"source_bootstrap":null}});
    // Typed closed input cannot deserialize absent native Scene configuration;
    // transported JSON also has no way to construct NativeTimingWitness.
    assert!(serde_json::from_value::<ConductRequest>(request).is_err());
    assert!(serde_json::from_value::<ConductRequest>(json!({"action":"source_continue","input":{
        "schema":"ql.native-procedural-source-continuation/v1","procedure_ref":"unknown",
        "expected_procedure_revision":"1","source_bootstrap":null,"current_readings":[],"materialization":null}})).is_err());
    assert_eq!(position.samples_elapsed, "42");
}

#[test]
fn actual_new_graph_cas_keeps_original_definition_and_refuses_semantic_substitution() {
    let (definition, prepared, position) = configuration("scene");
    let original = json!((&definition, &prepared, &position));
    let mut source = definition.source_composition.clone();
    source["steps"][0]["useRef"] = json!("whole:fresh-configuration-read");
    source["steps"][1]["from"] = source["steps"][0]["useRef"].clone();
    let next = definition::reobserve_native_definition_configuration(
        &definition,
        &source,
        &definition.currentness,
        &definition.thread_plan,
        &definition.current_readings,
        4,
    )
    .unwrap();
    assert_eq!(next.document_revision, 4);
    assert_eq!(next.procedure, definition.procedure);
    assert_eq!(json!(next.program), json!(definition.program));
    assert_eq!(next.interval_ref, definition.interval_ref);
    assert_eq!(next.required_consumers, definition.required_consumers);
    assert_eq!(json!(next.thread_plan), json!(definition.thread_plan));
    assert_eq!(next.currentness.expected, definition.currentness.expected);
    assert_ne!(next.source_composition, definition.source_composition);
    assert!(next.materialization.is_none());
    let old =
        definition::prepare_native_definition_configuration(&definition, &prepared, &position, 42)
            .unwrap();
    // Configuration reobservation doesn't mutate the original pending seed.
    assert_eq!(old.pending_preparation.as_ref(), Some(&prepared));
    assert_eq!(old.last_generated, prepared.contributions);
    for changed in ["recipe", "profile"] {
        let mut wrong = source.clone();
        let step = if changed == "recipe" { 0 } else { 1 };
        wrong["steps"][step]["basis"]["revision"] = json!("substituted");
        assert!(
            definition::reobserve_native_definition_configuration(
                &definition,
                &wrong,
                &definition.currentness,
                &definition.thread_plan,
                &definition.current_readings,
                4
            )
            .is_err()
        );
    }
    assert!(
        definition::reobserve_native_definition_configuration(
            &definition,
            &source,
            &definition.currentness,
            &definition.thread_plan,
            &definition.current_readings,
            2
        )
        .is_err()
    );
    assert_eq!(json!((&definition, &prepared, &position)), original);
}

#[test]
fn current_scene_material_can_change_without_retagging_original_source_configuration() {
    use ql_mef::procedural_intervention::{ManualEdit, extract_owned_manual_interventions};
    use ql_mef::procedural_manifestation::{NativeReading, ReadingAvailability, fingerprint};
    use ql_mef::procedural_source::NativeBootstrapSceneRead;
    let source = support::source(); // Real instantiate_scene over the production template.
    let before = NativeBootstrapSceneRead {
        native_owner: source.native_owner,
        expression_ref: source.expression_ref,
        scene_ref: source.scene_ref,
        document_revision: source.document_revision,
        source_basis: source.source_basis,
        material_fingerprint: source.material_fingerprint,
        presentation: source.presentation,
        locus: NativeReading {
            reference: source.locus_ref,
            revision: source.locus_revision,
            availability: ReadingAvailability::Available,
        },
        source_read_receipt_ref: "configured:original-material-read".into(),
    };
    let original = json!(before);
    let force = support::fixture("force");
    let contribution = force
        .first
        .contributions
        .iter()
        .find(|c| c.generated_basis["parameter"] == "force_radius")
        .unwrap();
    assert!(
        force.first.native_edit["changes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|change| change["change"] == "parameter_set"
                && change["parameter"] == "force_radius"
                && change["value"] == 200)
    );
    let mut after = before.clone();
    let entity = before.presentation["scene"]["entities"][0]["id"]
        .as_str()
        .unwrap();
    let mut material = before.presentation.clone();
    material["scene"]["entities"][0]["force"]["radius"] = json!(0.8);
    assert!(
        contribution
            .owned_addresses
            .iter()
            .any(
                |a| a.scene_ref.as_deref() == Some(before.scene_ref.as_str())
                    && a.entity_ref.as_deref() == Some(entity)
            )
    );
    let attributed = extract_owned_manual_interventions(&ManualEdit {
        expression_ref: before.expression_ref.clone(),
        scene_ref: before.scene_ref.clone(),
        contribution_ref: contribution.contribution_ref.clone(),
        owned_addresses: contribution.owned_addresses.clone(),
        entity_refs: support::entity_refs(&before.presentation),
        before: before.presentation.clone(),
        after: material.clone(),
        actor_ref: "human:owner".into(),
        operation_ref: "configured:ordinary-edit".into(),
        document_revision: 6,
        retained_overlays: vec![],
        retained_native_records: vec![],
    })
    .unwrap();
    assert_eq!(attributed.native_records.len(), 1);
    assert_eq!(attributed.native_records[0].value, json!(0.8));
    after.presentation = material;
    after.document_revision = 6;
    after.source_read_receipt_ref = "configured:fresh-material-read".into();
    after.material_fingerprint = fingerprint(&after.presentation).unwrap();
    definition::validate_continuation_scene_configuration(&before, &after).unwrap();
    assert_ne!(after.presentation, before.presentation);
    assert_ne!(after.material_fingerprint, before.material_fingerprint);
    assert_eq!(after.source_basis, before.source_basis);
    for field in [
        "namespace",
        "scene",
        "profile",
        "locus",
        "cas",
        "stale_material",
    ] {
        let mut wrong = after.clone();
        match field {
            "namespace" => wrong.expression_ref = "expression:other".into(),
            "scene" => wrong.scene_ref = "expression:acceptance:scene:other".into(),
            "profile" => wrong.source_basis.revision = "substituted".into(),
            "locus" => wrong.locus.revision = "substituted".into(),
            "cas" => wrong.document_revision = 2,
            "stale_material" => wrong.material_fingerprint = before.material_fingerprint.clone(),
            _ => unreachable!(),
        }
        assert!(
            definition::validate_continuation_scene_configuration(&before, &wrong).is_err(),
            "{field}"
        );
    }
    assert_eq!(json!(before), original);
    // This is configuration discrimination, never a private Source/installation
    // or material application claim. PCR2 requires the real owner capture.
}

#[test]
fn original_source_evidence_reobserves_after_current_material_without_retagging_cprime() {
    use ql_mef::procedural_manifestation::{NativeReading, ReadingAvailability, fingerprint};
    use ql_mef::procedural_source::*;
    use ql_mef::vak_scope::OperativeScopeCorrelation;
    let native = support::source();
    let mut procedure = support::procedure();
    let context = support::context(&mut procedure);
    let original = NativeSourceBootstrap {
        schema: SOURCE_BOOTSTRAP_REQUEST.into(),
        scene: NativeBootstrapSceneRead {
            native_owner: native.native_owner,
            expression_ref: native.expression_ref.clone(),
            scene_ref: native.scene_ref,
            document_revision: native.document_revision,
            source_basis: native.source_basis,
            material_fingerprint: native.material_fingerprint,
            presentation: native.presentation,
            locus: NativeReading {
                reference: native.locus_ref.clone(),
                revision: native.locus_revision,
                availability: ReadingAvailability::Available,
            },
            source_read_receipt_ref: "configured:original-semantic-read".into(),
        },
        authorship: StageSourceAuthorship {
            actor_ref: procedure.composition.actor.clone(),
            standing_ref: "AUTHORED-ARCHITECTURE".into(),
            principal_role: support::subject().presentation_role,
            contributors: vec![],
            members: vec![],
            source_returns: vec![],
            relations: vec![],
            category: "M".into(),
            ground_ref: native.locus_ref,
            ground_face: "direct".into(),
            frame: NativeStageFrame {
                id: procedure.composition.frame.0.code().into(),
                lens: "L0".into(),
                basis: "chromatic".into(),
                face: "direct".into(),
                positions: "local".into(),
            },
            language: None,
            profile: procedure.composition.profile(),
            resolve_path: procedure.composition.resolve_path,
            context_resolution: procedure.composition.context_resolution,
            authority: procedure.composition.authority,
            thread_plan: serde_json::from_value(context["thread_plan"].clone()).unwrap(),
        },
    };
    let untouched = json!(original);
    let mut current = original.clone();
    current.scene.document_revision += 3;
    current.scene.presentation["scene"]["entities"][0]["force"]["strength"] = json!(0.8);
    current.scene.material_fingerprint = fingerprint(&current.scene.presentation).unwrap();
    current.scene.source_read_receipt_ref = "configured:current-semantic-read".into();
    let original_composition =
        prepare_native_source_composition_configuration(&original, &support::subject(), None)
            .unwrap();
    let ordinary_current =
        prepare_native_source_composition_configuration(&current, &support::subject(), None)
            .unwrap();
    let anchored_current = prepare_native_source_composition_configuration(
        &current,
        &support::subject(),
        Some(&original),
    )
    .unwrap();
    let binding = |composition: &serde_json::Value| {
        let (_, graph) = ql_mef::vak_composition_wire::compile_request(composition).unwrap();
        graph
            .bind_operative_scope(
                &original.scene.expression_ref,
                original.authorship.profile.clone(),
                OperativeScopeCorrelation {
                    world_ref: original.scene.expression_ref.clone(),
                    world_generation: original.scene.source_basis.revision.clone(),
                    method_skill_ref: None,
                },
            )
            .unwrap()
    };
    let old_binding = binding(&original_composition);
    assert_ne!(
        binding(&ordinary_current).binding_revision,
        old_binding.binding_revision
    );
    assert_eq!(binding(&anchored_current), old_binding);
    assert_eq!(
        anchored_current["steps"][0]["basis"]["evidence"][1],
        original.scene.material_fingerprint
    );
    assert_ne!(
        current.scene.material_fingerprint,
        original.scene.material_fingerprint
    );
    for field in [
        "profile",
        "actor",
        "frame",
        "thread",
        "locus",
        "source",
        "fingerprint",
    ] {
        let mut wrong = current.clone();
        match field {
            "profile" => wrong.scene.source_basis.revision = "other".into(),
            "actor" => wrong.authorship.actor_ref = "other:actor".into(),
            "frame" => wrong.authorship.frame.id = "CF0".into(),
            "thread" => wrong.authorship.thread_plan.legs[0].subject_ref = "other:subject".into(),
            "locus" => wrong.scene.locus.revision = "other".into(),
            "source" => wrong.scene.source_basis.source_ref = "other:profile".into(),
            "fingerprint" => {
                wrong.scene.material_fingerprint = original.scene.material_fingerprint.clone()
            }
            _ => unreachable!(),
        }
        assert!(
            prepare_native_source_composition_configuration(
                &wrong,
                &support::subject(),
                Some(&original)
            )
            .is_err(),
            "{field}"
        );
    }
    assert_eq!(json!(original), untouched);
    // The actual graph compiler and binding owner execute configuration only.
    // No installed origin, lease, timing witness, Scene fact or ACK is minted.
}
