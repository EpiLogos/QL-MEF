use ql_mef::MFace;
use ql_mef::m_tree::native_m_registry;
use ql_mef::m3_material_fold::*;
use ql_mef::m3_source::native_m3_source;
use ql_mef::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation, M3Request, M3State};
use ql_mef::physical_body::*;
use ql_mef::source_form_body::*;
use serde_json::{Value, json};
mod actual_performance_source {
    include!("support/retained_performance.rs");
}
// Reused producer also serves the unchanged original route corpus.
#[allow(dead_code)]
mod actual_receiving_source {
    include!("support/performance_route_management_fixture.rs");
}

fn state(address: u8) -> M3State {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut request: M3Request = serde_json::from_value(fixture["request"].clone()).unwrap();
    request.address = address;
    request.pose = 0;
    request.matrix_axis = 0;
    M3State::new(request).unwrap()
}
fn request(state: &M3State) -> MaterialFoldRequest {
    let snapshot = state.snapshot();
    let source = native_m3_source();
    MaterialFoldRequest {
        event_ref: snapshot["identity"]["event_ref"].as_str().unwrap().into(),
        subject_ref: snapshot["subject_ref"].as_str().unwrap().into(),
        expected_generation: state.generation(),
        expected_source_revision: source.source_revision().into(),
        expected_domain_revision: source.revision().into(),
        expected_registry_revision: native_m_registry().manifest().registry_revision.clone(),
        source_coordinate: source_coordinate_wire(
            &source_form_coordinate(state, MFace::Bimba).unwrap(),
        ),
        recipe_ref: "controlled:source-qualified-retained-square".into(),
        recipe_revision: "1".into(),
        material_treatment: "glyph-mask".into(),
        stage_units_per_material_unit: 200.,
        metres_per_material_unit: 0.1,
    }
}
fn apply(state: &mut M3State, operations: Vec<M3Operation>) -> ql_mef::m3_state::M3Receipt {
    let basis = state.snapshot();
    state
        .apply(M3Command {
            schema: COMMAND_SCHEMA.into(),
            event_ref: basis["identity"]["event_ref"].as_str().unwrap().into(),
            subject_ref: basis["subject_ref"].as_str().unwrap().into(),
            expected_generation: state.generation(),
            actor_ref: "controlled:material-fold-test".into(),
            cause_ref: "controlled:actual-native-operation".into(),
            occurrence_unix_ms: basis["occurrence_unix_ms"].as_u64().unwrap(),
            receipt_unix_ms: basis["receipt_unix_ms"].as_u64().unwrap(),
            operations,
        })
        .unwrap()
}
#[test]
fn all_64_native_forms_and_472_poses_supply_their_actual_sites_and_ranked_pose() {
    let mut total = 0;
    for address in 0..64 {
        let mut state = state(address);
        for slot in 0..state.fold().state_count() {
            apply(&mut state, vec![M3Operation::SetPose { pose: slot }]);
            let r = request(&state);
            let p = prepare_material_fold(&state, &r).unwrap();
            admit_material_fold(&state, &r, &p).unwrap();
            assert_eq!(p.native_state, state.snapshot());
            assert_eq!(p.pose_projection["rotation_slot"], slot);
            assert_eq!(
                p.pose_projection["pose_ordinal"],
                state.fold().rotational_pose().ordinal()
            );
            for i in 0..3 {
                assert_eq!(
                    p.crease_angles_rad[i],
                    (f64::from(state.fold().sites()[i].signed_angle) / 10.).to_radians()
                );
            }
            total += 1;
        }
    }
    assert_eq!(total, 472);
}
#[test]
fn all_384_line_changes_keep_symbolic_result_and_material_determinants_together() {
    for address in 0..64 {
        for line in 0..6 {
            let mut state = state(address);
            let old = prepare_material_fold(&state, &request(&state)).unwrap();
            let receipt = apply(&mut state, vec![M3Operation::ChangeLine { line }]);
            assert_eq!(receipt.status, "applied");
            let p = prepare_material_fold(&state, &request(&state)).unwrap();
            assert_eq!(p.native_state["form"]["address"], address ^ (1 << line));
            assert_eq!(p.native_state, receipt.after);
            assert_ne!(p.source_generation, old.source_generation);
            assert!(
                p.crease_angles_rad != old.crease_angles_rad
                    || p.site_velocities_deg10 != old.site_velocities_deg10
            );
        }
    }
}
#[test]
fn matrix_operations_use_native_result_and_preserve_unresolved_resonance() {
    for address in 0..64 {
        for family in 0..3 {
            let mut state = state(address);
            let before = state.snapshot();
            let r = request(&state);
            let receipt = apply(&mut state, vec![M3Operation::ApplyMatrix { family }]);
            let p = prepare_material_fold(&state, &request(&state)).unwrap();
            assert_eq!(p.native_state, receipt.after);
            if receipt.status == "provisional-unchanged" {
                assert_eq!(p.native_state, before);
                assert_eq!(p.source_generation, r.expected_generation);
            } else {
                assert_eq!(p.source_generation, r.expected_generation + 1);
            }
        }
    }
}
#[test]
fn aperture_clock_and_transcription_are_retained_readings_without_replacing_geometry() {
    let mut state = state(7);
    let before = prepare_material_fold(&state, &request(&state)).unwrap();
    apply(
        &mut state,
        vec![
            M3Operation::ReciprocalAperture,
            M3Operation::AdvanceClock { steps: 360 },
            M3Operation::Transcribe { rna: true },
        ],
    );
    let after = prepare_material_fold(&state, &request(&state)).unwrap();
    assert_eq!(before.crease_angles_rad, after.crease_angles_rad);
    assert_eq!(before.pose_axis, after.pose_axis);
    assert_eq!(before.pose_angle_rad, after.pose_angle_rad);
    assert_ne!(before.native_state["clock"], after.native_state["clock"]);
    assert_ne!(
        before.native_state["aperture"],
        after.native_state["aperture"]
    );
    assert_eq!(after.native_state["transcription"]["rna"], true);
    assert_eq!(before.topology_ref, after.topology_ref);
}
#[test]
fn source_subject_generation_units_and_native_coordinate_are_detecting_fences() {
    let state = state(7);
    let r = request(&state);
    let mut bad = r.clone();
    bad.subject_ref = "wrong-subject".into();
    assert!(prepare_material_fold(&state, &bad).is_err());
    bad = r.clone();
    bad.expected_generation += 1;
    assert!(prepare_material_fold(&state, &bad).is_err());
    bad = r.clone();
    bad.expected_source_revision = "wrong-source".into();
    assert!(prepare_material_fold(&state, &bad).is_err());
    bad = r.clone();
    bad.source_coordinate["face"] = json!("pratibimba");
    assert!(prepare_material_fold(&state, &bad).is_err());
    bad = r.clone();
    bad.source_coordinate["source_ref"] = json!("#3");
    assert!(prepare_material_fold(&state, &bad).is_err());
    bad = r.clone();
    bad.metres_per_material_unit = 0.;
    assert!(prepare_material_fold(&state, &bad).is_err());
    bad = r.clone();
    bad.material_treatment = "glyph-label".into();
    assert!(prepare_material_fold(&state, &bad).is_err());
    let mut p = prepare_material_fold(&state, &r).unwrap();
    p.pose_angle_rad += 0.1;
    assert!(admit_material_fold(&state, &r, &p).is_err());
}
fn provenance(reference: &str) -> PhysicalProvenance {
    PhysicalProvenance {
        reference: reference.into(),
        revision: "1".into(),
        source_ref: SOURCE_GEOMETRY_BASIS.into(),
        standing: PhysicalStanding::AgentProposed,
    }
}
fn body_recipe() -> SourceGeometryRecipe {
    SourceGeometryRecipe {
        provenance: provenance("controlled:source-elastic-frame-law"),
        family: BodyFamily::AxialTruss,
        frame_side_metres: 0.1,
        site_separation_metres: 0.3,
        section_by_element_m2: [1e-4; 4],
        intersite_section_m2: 1e-4,
        prestress_newtons: 0.,
    }
}
fn body_controls(state: &M3State) -> SourceBodyControls {
    let mut weights = vec![0.; 12];
    weights[11] = 1.;
    SourceBodyControls {
        expected_m3_generation: state.generation(),
        body_revision: 1,
        preparation_ref: "controlled:source-body-1".into(),
        state_ref: "controlled:retained-source-body".into(),
        material: PhysicalMaterial {
            provenance: provenance("controlled:elastic-frame-material"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 1000.,
            damping_alpha_per_second: 0.4,
            damping_beta_seconds: 0.,
        },
        sample_rate: 48000,
        exciter: SpatialProjection {
            axis: [0., 0., 1.],
            node_weights: weights.clone(),
        },
        pickup: SpatialProjection {
            axis: [0., 0., 1.],
            node_weights: weights,
        },
        pickup_linear_per_metre: 1000.,
        max_force_newtons: 10.,
        max_impulse_newton_seconds: 0.01,
        max_displacement_metres: 0.01,
    }
}
#[test]
fn pair_regenerates_actual_body_geometry_and_fences_source_or_face_disconnect() {
    let mut state = state(7);
    let r = request(&state);
    let plan = prepare_material_fold(&state, &r).unwrap();
    let recipe = body_recipe();
    let prepared = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Bimba).unwrap(),
        recipe.clone(),
        body_controls(&state),
    )
    .unwrap();
    let pair = pair_material_source_body(&state, &r, &plan, prepared.body(), &recipe).unwrap();
    assert_eq!(pair["reception"], "prepared-consumer-observation-required");
    let mut wrong = recipe.clone();
    wrong.frame_side_metres = 0.11;
    assert!(pair_material_source_body(&state, &r, &plan, prepared.body(), &wrong).is_err());
    let prime = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        recipe.clone(),
        body_controls(&state),
    )
    .unwrap();
    assert!(pair_material_source_body(&state, &r, &plan, prime.body(), &recipe).is_err());
    apply(&mut state, vec![M3Operation::ChangeLine { line: 0 }]);
    let next = request(&state);
    let p = prepare_material_fold(&state, &next).unwrap();
    assert!(pair_material_source_body(&state, &next, &p, prepared.body(), &recipe).is_err());
}
#[test]
fn emit_actual_source_to_consumer_fixture_when_artifact_path_is_explicit() {
    let mut input = actual_performance_source::preparation();
    let mut state = M3State::new(input.coupled.m3.clone()).unwrap();
    let mut replay = Vec::new();
    for command in &input.coupled.m3_commands {
        replay.push(state.apply(command.clone()).unwrap());
    }
    let basis = state.snapshot();
    let command = M3Command {
        schema: COMMAND_SCHEMA.into(),
        event_ref: basis["identity"]["event_ref"].as_str().unwrap().into(),
        subject_ref: basis["subject_ref"].as_str().unwrap().into(),
        expected_generation: state.generation(),
        actor_ref: "controlled:material-fold-test".into(),
        cause_ref: "controlled:actual-native-operation".into(),
        occurrence_unix_ms: basis["occurrence_unix_ms"].as_u64().unwrap(),
        receipt_unix_ms: basis["receipt_unix_ms"].as_u64().unwrap(),
        operations: vec![
            M3Operation::ChangeLine { line: 0 },
            M3Operation::SetPose { pose: 2 },
        ],
    };
    let receipt = state.apply(command.clone()).unwrap();
    replay.push(receipt.clone());
    input.coupled.m3_commands.push(command);
    let mut r = request(&state);
    r.source_coordinate =
        source_coordinate_wire(&source_form_coordinate(&state, MFace::Pratibimba).unwrap());
    let plan = prepare_material_fold(&state, &r).unwrap();
    let samples = samples();
    let native_targets = material_fold_targets(&state, &r, &plan, &samples).unwrap();
    let recipe = body_recipe();
    let prepared = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        recipe.clone(),
        body_controls(&state),
    )
    .unwrap();
    input.physical = prepared.body().request().clone();
    let original_coupled_source = input.coupled.clone();
    let performance =
        ql_mef::performance_audio::prepare_source_form_performance(input, recipe.clone()).unwrap();
    performance.validate_source_form_consumer(&state).unwrap();
    assert_eq!(performance.native_basis().m3, state.snapshot());
    let body_material =
        prepare_body_material(&state, &r, &plan, prepared.body(), &recipe, &samples, 0.01).unwrap();
    let expected = json!({"event_ref":r.event_ref,"subject_ref":r.subject_ref,"source_coordinate":r.source_coordinate,"source_generation":r.expected_generation,
        "source_revision":r.expected_source_revision,"domain_revision":r.expected_domain_revision,"registry_revision":r.expected_registry_revision});
    if let Some(path) = std::env::var_os("QL_MATERIAL_FOLD_FIXTURE") {
        std::fs::write(path,serde_json::to_vec_pretty(&json!({"plan":plan,"expected":expected,"receipt":receipt,"replay":replay,"original_coupled_source":original_coupled_source,"samples":samples,"native_targets":native_targets,"body_material":body_material,"performance":performance,"native_basis":performance.native_basis(),"scope":"actual M1/M2/M3 performance producers with controlled retained rest samples and source-frame metric geometry; no installed GPU/audio acceptance claim"})).unwrap()).unwrap();
    }
}

fn samples() -> Vec<MaterialRestSample> {
    [
        [-0.75, -0.5, 0.01],
        [-0.5, 0., 0.],
        [-0.25, 0.5, -0.02],
        [0.25, -0.5, 0.02],
        [0.75, 0.5, -0.02],
        [1., 1., 0.],
    ]
    .into_iter()
    .enumerate()
    .map(|(i, rest_material)| MaterialRestSample {
        id: serde_json::to_string(&json!([
            "controlled:sampler-square",
            if i < 3 { "layer-a" } else { "layer-b" },
            "controlled:sampler-allocation",
            i % 3
        ]))
        .unwrap(),
        source_ref: "controlled:sampler-square".into(),
        source_revision: "1".into(),
        layer_ref: if i < 3 { "layer-a" } else { "layer-b" }.into(),
        rest_material,
        density: 0.8,
    })
    .collect()
}
#[test]
fn native_geometry_retains_exact_sample_source_layer_and_density_without_clock_advance() {
    let state = state(7);
    let r = request(&state);
    let plan = prepare_material_fold(&state, &r).unwrap();
    let input = samples();
    let before = state.snapshot();
    let target = material_fold_targets(&state, &r, &plan, &input).unwrap();
    assert_eq!(state.snapshot(), before);
    assert_eq!(target.len(), input.len());
    for (sample, target) in input.iter().zip(&target) {
        assert_eq!(sample.id, target.id);
        assert_eq!(sample.layer_ref, target.layer_ref);
        assert_eq!(sample.source_ref, target.source_ref);
        assert_eq!(sample.density, target.density);
    }
    let mut invalid = samples();
    invalid[1].id = invalid[0].id.clone();
    assert!(material_fold_targets(&state, &r, &plan, &invalid).is_err());
    invalid = samples();
    invalid[0].rest_material[0] = 1.1;
    assert!(material_fold_targets(&state, &r, &plan, &invalid).is_err());
    invalid = samples();
    invalid[0].rest_material[2] = f64::NAN;
    assert!(material_fold_targets(&state, &r, &plan, &invalid).is_err());
    assert_eq!(
        target,
        material_fold_targets(&state, &r, &plan, &input).unwrap()
    );
}

#[test]
fn all_native_forms_project_retained_material_over_actual_body_nodes_with_explicit_bounds() {
    for address in 0..64 {
        let state = state(address);
        let r = request(&state);
        let plan = prepare_material_fold(&state, &r).unwrap();
        let recipe = body_recipe();
        let body = prepare_source_form_body(
            &state,
            source_form_coordinate(&state, MFace::Bimba).unwrap(),
            recipe.clone(),
            body_controls(&state),
        )
        .unwrap();
        let projection =
            prepare_body_material(&state, &r, &plan, body.body(), &recipe, &samples(), 0.01)
                .unwrap();
        assert_eq!(
            projection["body"]["request"]["geometry"],
            serde_json::to_value(&body.body().request().geometry).unwrap()
        );
        for sample in projection["samples"].as_array().unwrap() {
            let weights = sample["node_weights"].as_array().unwrap();
            assert_eq!(weights.len(), 4);
            let sum = weights
                .iter()
                .map(|w| w["weight"].as_f64().unwrap())
                .sum::<f64>();
            assert!((sum - 1.).abs() < 1e-12);
            assert!(
                weights
                    .iter()
                    .all(|w| (0.0..=1.0).contains(&w["weight"].as_f64().unwrap()))
            );
            assert!(
                sample["rest_metres"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|v| v.as_f64().unwrap().is_finite())
            );
        }
        let mut wrong = recipe.clone();
        wrong.site_separation_metres += 0.01;
        assert!(
            prepare_body_material(&state, &r, &plan, body.body(), &wrong, &samples(), 0.01)
                .is_err()
        );
        assert!(
            prepare_body_material(&state, &r, &plan, body.body(), &recipe, &samples(), 0.0001)
                .is_err()
        );
    }
}

/// The paired native C++ receiving experiment consumes the two actual source
/// owners below. No serialized fixture grants a Scene/Act Source lease.
#[test]
fn emit_source_qualified_form_receiving_fixture_when_artifact_path_is_explicit() {
    let mut input = actual_performance_source::preparation();
    let mut state = M3State::new(input.coupled.m3.clone()).unwrap();
    let mut original_replay = Vec::new();
    for command in &input.coupled.m3_commands {
        original_replay.push(state.apply(command.clone()).unwrap());
    }
    let recipe = body_recipe();
    let before_source = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        recipe.clone(),
        body_controls(&state),
    )
    .unwrap();
    before_source.validate_source_geometry(&state).unwrap();
    input.physical = before_source.body().request().clone();
    // Preparation is consumed by its real producer. Re-enter that same
    // authored source for the later transition instead of inventing a Clone
    // contract on the preparation owner.
    let mut after_input = actual_performance_source::preparation();
    assert_eq!(
        serde_json::to_value(&after_input.coupled).unwrap(),
        serde_json::to_value(&input.coupled).unwrap()
    );
    let before =
        ql_mef::performance_audio::prepare_source_form_performance(input, recipe.clone()).unwrap();
    before.validate_source_form_consumer(&state).unwrap();
    let before_basis = state.snapshot();
    let command = M3Command {
        schema: COMMAND_SCHEMA.into(),
        event_ref: before_basis["identity"]["event_ref"]
            .as_str()
            .unwrap()
            .into(),
        subject_ref: before_basis["subject_ref"].as_str().unwrap().into(),
        expected_generation: state.generation(),
        actor_ref: "controlled:source-form-receiving-test".into(),
        cause_ref: "controlled:actual-native-form-transition".into(),
        occurrence_unix_ms: before_basis["occurrence_unix_ms"].as_u64().unwrap(),
        receipt_unix_ms: before_basis["receipt_unix_ms"].as_u64().unwrap(),
        operations: vec![
            M3Operation::ChangeLine { line: 0 },
            M3Operation::SetPose { pose: 2 },
        ],
    };
    let receipt = state.apply(command.clone()).unwrap();
    assert_eq!(receipt.status, "applied");
    after_input.coupled.m3_commands.push(command.clone());
    let mut controls = body_controls(&state);
    controls.body_revision = before
        .physical_body()
        .request()
        .body_revision
        .checked_add(1)
        .unwrap();
    controls.preparation_ref = "controlled:source-body-form-2".into();
    let after_source = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        recipe.clone(),
        controls,
    )
    .unwrap();
    after_source.validate_source_geometry(&state).unwrap();
    let transition = prepare_form_transition(
        before.physical_body(),
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        after_source.body().request().clone(),
        before.physical_body().request().body_revision,
        512,
        FormTransitionPolicy::ProjectCorrespondingNodes,
    )
    .unwrap();
    after_input.physical = after_source.body().request().clone();
    let after_coupled_source = after_input.coupled.clone();
    let after =
        ql_mef::performance_audio::prepare_source_form_performance(after_input, recipe.clone())
            .unwrap();
    after.validate_source_form_consumer(&state).unwrap();
    assert_eq!(after.native_basis().m3, receipt.after);
    assert_eq!(before.native_basis().m3, before_basis);
    assert_eq!(after.physical_body().request(), transition.after.request());
    assert_eq!(
        after.physical_body().source_generation(),
        before.physical_body().source_generation() + 1
    );
    // Musical M1/M2 identity and all actual targets stay fixed. The native
    // determination must nevertheless name the newly prepared physical body.
    let mut before_musical = before.determination().clone();
    let mut after_musical = after.determination().clone();
    for field in ["body_preparation_ref", "body_state_ref", "body_revision"] {
        before_musical.as_object_mut().unwrap().remove(field);
        after_musical.as_object_mut().unwrap().remove(field);
    }
    assert_eq!(after_musical, before_musical);
    assert_eq!(after.notes(), before.notes());
    assert_eq!(
        after.determination()["body_preparation_ref"],
        after.physical_body().request().preparation_ref
    );
    assert_eq!(
        after.determination()["body_state_ref"],
        after.physical_body().request().state_ref
    );
    assert_eq!(
        after.determination()["body_revision"],
        after.physical_body().request().body_revision.to_string()
    );
    assert_ne!(
        before.physical_body().request().geometry,
        after.physical_body().request().geometry
    );
    assert!(before.validate_source_form_consumer(&state).is_err());
    let before_packet = actual_receiving_source::for_prepared(&before, 0).unwrap();
    let after_packet = actual_receiving_source::for_prepared(&after, 512).unwrap();
    assert_eq!(
        before_packet,
        actual_receiving_source::for_prepared(&before, 0).unwrap()
    );
    assert_eq!(
        after_packet,
        actual_receiving_source::for_prepared(&after, 512).unwrap()
    );
    assert_ne!(
        before_packet["native_admission"],
        after_packet["native_admission"]
    );
    if let Some(path) = std::env::var_os("QL_SOURCE_FORM_RECEIVING_FIXTURE") {
        use sha2::{Digest, Sha256};
        let mut fixture = json!({
            "schema":"ql.native-source-form-receiving-fixture/v1",
            "before":before_packet,"after":after_packet,
            "after_fresh":actual_receiving_source::for_prepared(&after,0).unwrap(),
            "command":command,"receipt":receipt,"original_replay":original_replay,
            "after_coupled_source":after_coupled_source,"recipe":recipe,
            "form_transition":transition,
            "scope":"Actual current M1/M2/M3 -> source-qualified metric Form -> N9 producer -> P/receiving component experiment; explicit Reference calibration; no Scene/Act private Source grant or installed instrument acceptance."
        });
        fixture["probe_scope_digest"] = json!(format!(
            "sha256:{:x}",
            Sha256::digest(serde_json::to_vec(&fixture).unwrap())
        ));
        std::fs::write(path, serde_json::to_vec_pretty(&fixture).unwrap()).unwrap();
    }
}
