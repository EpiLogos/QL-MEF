//! Actual native M3 operations must change their generated physical graph;
//! labels, source payloads and green admission alone are insufficient.
use ql_core::{ElementalQuaternionBasis, MatrixAxis, Mobility};
use ql_mef::MFace;
use ql_mef::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation, M3Request, M3State};
use ql_mef::physical_body::*;
use ql_mef::source_form_body::*;
use serde_json::{Value, json};
fn state(address: u8, pose: u8, aperture: u8, clock_steps: u64) -> M3State {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut request: M3Request = serde_json::from_value(fixture["request"].clone()).unwrap();
    request.address = address;
    request.pose = pose;
    request.aperture = aperture;
    request.clock_steps = clock_steps;
    request.matrix_axis = 0;
    M3State::new(request).unwrap()
}
fn operation(state: &mut M3State, operations: Vec<M3Operation>) {
    let before = state.snapshot();
    state
        .apply(M3Command {
            schema: COMMAND_SCHEMA.into(),
            event_ref: before["identity"]["event_ref"].as_str().unwrap().into(),
            subject_ref: before["subject_ref"].as_str().unwrap().into(),
            expected_generation: state.generation(),
            actor_ref: "controlled:native-source-body-test".into(),
            cause_ref: "controlled:actual-m3-operation".into(),
            occurrence_unix_ms: before["occurrence_unix_ms"].as_u64().unwrap(),
            receipt_unix_ms: before["receipt_unix_ms"].as_u64().unwrap(),
            operations,
        })
        .unwrap();
}
fn provenance(reference: &str) -> PhysicalProvenance {
    PhysicalProvenance {
        reference: reference.into(),
        revision: "1".into(),
        source_ref: SOURCE_GEOMETRY_BASIS.into(),
        standing: PhysicalStanding::AgentProposed,
    }
}
fn recipe(family: BodyFamily) -> SourceGeometryRecipe {
    SourceGeometryRecipe {
        provenance: provenance("controlled:source-elastic-frame-law"),
        family,
        frame_side_metres: 0.1,
        site_separation_metres: 0.3,
        section_by_element_m2: [1e-4, 2e-4, 3e-4, 4e-4],
        intersite_section_m2: 1e-4,
        prestress_newtons: if family == BodyFamily::AxialTruss {
            0.0
        } else {
            5.0
        },
    }
}
fn controls(state: &M3State, revision: u64) -> SourceBodyControls {
    let mut weights = vec![0.0; 12];
    weights[11] = 1.0;
    SourceBodyControls {
        expected_m3_generation: state.generation(),
        body_revision: revision,
        preparation_ref: format!("controlled:source-body/{revision}"),
        state_ref: "controlled:resident-source-body".into(),
        material: PhysicalMaterial {
            provenance: provenance("controlled:elastic-frame-material"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 1000.0,
            damping_alpha_per_second: 0.4,
            damping_beta_seconds: 0.0,
        },
        sample_rate: 48000,
        exciter: SpatialProjection {
            axis: [0.0, 0.0, 1.0],
            node_weights: weights.clone(),
        },
        pickup: SpatialProjection {
            axis: [0.0, 0.0, 1.0],
            node_weights: weights,
        },
        pickup_linear_per_metre: 1000.0,
        max_force_newtons: 10.0,
        max_impulse_newton_seconds: 0.01,
        max_displacement_metres: 0.01,
    }
}
fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter()
        .zip(b)
        .map(|(x, y)| (x - y) * (x - y))
        .sum::<f64>()
        .sqrt()
}
fn physical_equal(a: &MetricGeometry, b: &MetricGeometry) -> bool {
    a.family == b.family
        && a.edges == b.edges
        && a.nodes.len() == b.nodes.len()
        && a.nodes.iter().zip(&b.nodes).all(|(x, y)| {
            x.identity == y.identity
                && x.rest_metres == y.rest_metres
                && x.fixed == y.fixed
                && x.additional_mass_kg == y.additional_mass_kg
        })
}
#[test]
fn independent_metric_reference_and_elemental_sections_derive_from_native_sites() {
    let state = state(0, 0, 0, 0);
    let recipe = recipe(BodyFamily::AxialTruss);
    let geometry = source_form_geometry(&state, &recipe).unwrap();
    let half = 0.05;
    let angle = 22.5_f64.to_radians();
    let expected = [-0.3 - half, -half * angle.cos(), -half * angle.sin()];
    for (actual, expected) in geometry.nodes[0].rest_metres.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 1e-14);
    }
    assert_eq!(geometry.nodes.len(), 12);
    assert_eq!(geometry.edges.len(), 34);
    for node in &geometry.nodes[..2] {
        assert_eq!(node.fixed, [true; 3]);
    }
    for node in &geometry.nodes[2..] {
        assert_eq!(node.fixed, [false; 3]);
    }
    let basis = ElementalQuaternionBasis::canonical();
    for site in 0..3 {
        let element = basis.element_of(state.fold().nucleotides()[site]);
        let in_site: Vec<_> = geometry
            .edges
            .iter()
            .filter(|edge| edge.first / 4 == site && edge.second / 4 == site)
            .collect();
        assert_eq!(in_site.len(), 6);
        assert!(
            in_site
                .iter()
                .all(|edge| edge.section_m2
                    == recipe.section_by_element_m2[element.component_index()])
        );
        assert!(
            (distance(
                geometry.nodes[site * 4].rest_metres,
                geometry.nodes[site * 4 + 1].rest_metres
            ) - recipe.frame_side_metres)
                .abs()
                < 1e-14
        );
        assert!(
            (distance(
                geometry.nodes[site * 4].rest_metres,
                geometry.nodes[site * 4 + 2].rest_metres
            ) - 2.0_f64.sqrt() * recipe.frame_side_metres)
                .abs()
                < 1e-14
        );
    }
    let body = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        recipe,
        controls(&state, 1),
    )
    .unwrap();
    assert_eq!(body.body().request().geometry, geometry);
    assert_eq!(body.body().source_coordinate().face, MFace::Pratibimba);
    assert_eq!(body.source_reading()["transcription"]["sequence"], "AAA");
    assert!(body.validate_source_geometry(&state).is_ok());
}
#[test]
fn every_form_and_native_line_operation_changes_physical_determinants_and_refuses_relabelled_mesh()
{
    let recipe = recipe(BodyFamily::AxialTruss);
    let mut edges = 0;
    for address in 0..64 {
        let source = state(address, 0, 0, 0);
        let before = source_form_geometry(&source, &recipe).unwrap();
        for line in 0..6 {
            let mut next = source.clone();
            operation(&mut next, vec![M3Operation::ChangeLine { line }]);
            assert_eq!(
                next.fold().codon(),
                source.fold().codon().line_change(line).unwrap()
            );
            let after = source_form_geometry(&next, &recipe).unwrap();
            assert!(
                !physical_equal(&before, &after),
                "physical one-bit effect missing at {address}/{line}"
            );
            // Forge all expected constituent labels while holding the old
            // coordinates/constraints. Exact native source regeneration fails.
            let mut relabelled = before.clone();
            for (old, new) in relabelled.nodes.iter_mut().zip(&after.nodes) {
                old.constituent = new.constituent.clone();
            }
            assert!(admit_source_form_metric(&next, &recipe, &relabelled).is_err());
            assert!(admit_source_form_metric(&next, &recipe, &after).is_ok());
            edges += 1;
        }
    }
    assert_eq!(edges, 384);
}
#[test]
fn all_472_native_poses_rotate_actual_metric_sites_and_preserve_element_geometry() {
    let recipe = recipe(BodyFamily::AxialTruss);
    let mut poses = 0;
    for address in 0..64 {
        let first = state(address, 0, 0, 0);
        let canonical = source_form_geometry(&first, &recipe).unwrap();
        let mut previous = canonical.clone();
        for pose in 0..first.fold().state_count() {
            let source = state(address, pose, 0, 0);
            let geometry = source_form_geometry(&source, &recipe).unwrap();
            if pose > 0 {
                assert!(
                    !physical_equal(&previous, &geometry),
                    "physical pose effect missing at {address}/{pose}"
                );
            }
            let theta = (f64::from(pose) * 45.0).to_radians();
            let p = canonical.nodes[8].rest_metres;
            let expected = [
                p[0],
                p[1] * theta.cos() - p[2] * theta.sin(),
                p[1] * theta.sin() + p[2] * theta.cos(),
            ];
            for (actual, expected) in geometry.nodes[8].rest_metres.into_iter().zip(expected) {
                assert!((actual - expected).abs() < 1e-14);
            }
            for (old, new) in canonical.edges.iter().zip(&geometry.edges) {
                assert!(
                    (distance(
                        canonical.nodes[old.first].rest_metres,
                        canonical.nodes[old.second].rest_metres
                    ) - distance(
                        geometry.nodes[new.first].rest_metres,
                        geometry.nodes[new.second].rest_metres
                    ))
                    .abs()
                        < 1e-14
                );
            }
            assert_eq!(
                geometry.nodes[11].fixed[2],
                source.fold().nucleotides()[2].mobility() == Mobility::Resting
            );
            previous = geometry;
            poses += 1;
        }
    }
    assert_eq!(poses, 472);
}
#[test]
fn actual_matrix_operation_is_a_geometric_axis_operation_with_source_gap_refusal() {
    let recipe = recipe(BodyFamily::AxialTruss);
    let mut source = state(0, 1, 0, 0);
    let before = source_form_geometry(&source, &recipe).unwrap();
    operation(&mut source, vec![M3Operation::ApplyMatrix { family: 1 }]);
    assert_eq!(source.fold().active_matrix_axis(), MatrixAxis::J);
    let after = source_form_geometry(&source, &recipe).unwrap();
    assert!(!physical_equal(&before, &after));
    let mut gap = state(5, 0, 0, 0);
    let before = gap.snapshot();
    let receipt = gap
        .apply(M3Command {
            schema: COMMAND_SCHEMA.into(),
            event_ref: before["identity"]["event_ref"].as_str().unwrap().into(),
            subject_ref: before["subject_ref"].as_str().unwrap().into(),
            expected_generation: gap.generation(),
            actor_ref: "controlled:test".into(),
            cause_ref: "controlled:source-gap".into(),
            occurrence_unix_ms: before["occurrence_unix_ms"].as_u64().unwrap(),
            receipt_unix_ms: before["receipt_unix_ms"].as_u64().unwrap(),
            operations: vec![M3Operation::ApplyMatrix { family: 2 }],
        })
        .unwrap();
    assert_eq!(receipt.status, "provisional-unchanged");
    assert_eq!(gap.snapshot(), before);
}
#[test]
fn aperture_clock_and_rna_readings_do_not_rewrite_geometry_or_the_resident_body() {
    let recipe = recipe(BodyFamily::AxialTruss);
    let mut source = state(1, 0, 0, 0);
    let body = prepare_source_form_body(
        &source,
        source_form_coordinate(&source, MFace::Pratibimba).unwrap(),
        recipe.clone(),
        controls(&source, 1),
    )
    .unwrap();
    let before = body.body().request().clone();
    for op in [
        M3Operation::SetAperture { aperture: 15 },
        M3Operation::AdvanceClock { steps: 721 },
        M3Operation::Transcribe { rna: true },
    ] {
        operation(&mut source, vec![op]);
        assert_eq!(
            source_form_geometry(&source, &recipe).unwrap(),
            before.geometry
        );
        let update = prepare_source_form_update(
            &body,
            &source,
            SourceFormUpdateRequest {
                coordinate: source_form_coordinate(&source, MFace::Pratibimba).unwrap(),
                recipe: recipe.clone(),
                controls: controls(&source, 2),
                expected_body_revision: 1,
                expected_sample: 512,
                policy: FormTransitionPolicy::ProjectCorrespondingNodes,
            },
        )
        .unwrap();
        match update {
            SourceFormUpdate::ReadingOnly {
                body_revision,
                preparation_ref,
                state_ref,
                source_reading,
            } => {
                assert_eq!(body_revision, 1);
                assert_eq!(preparation_ref, before.preparation_ref);
                assert_eq!(state_ref, before.state_ref);
                assert_eq!(source_reading, source.snapshot());
            }
            _ => panic!("view/clock/transcription read caused a physical replacement"),
        }
        assert_eq!(body.body().request(), &before);
    }
    assert_eq!(source.snapshot()["aperture"]["total_lenses"], 18);
    assert_eq!(source.snapshot()["clock"]["degree720"], 1);
    assert_eq!(source.snapshot()["transcription"]["sequence"], "AAU");
}
#[test]
fn source_form_transition_keeps_exact_material_node_correspondence_and_declares_energy_work() {
    let mut source = state(0, 0, 0, 0);
    let recipe = recipe(BodyFamily::PrestressedTensionNetwork);
    let body = prepare_source_form_body(
        &source,
        source_form_coordinate(&source, MFace::Pratibimba).unwrap(),
        recipe.clone(),
        controls(&source, 1),
    )
    .unwrap();
    operation(&mut source, vec![M3Operation::ChangeLine { line: 2 }]);
    let update = prepare_source_form_update(
        &body,
        &source,
        SourceFormUpdateRequest {
            coordinate: source_form_coordinate(&source, MFace::Pratibimba).unwrap(),
            recipe: recipe.clone(),
            controls: controls(&source, 2),
            expected_body_revision: 1,
            expected_sample: 256,
            policy: FormTransitionPolicy::ProjectCorrespondingNodes,
        },
    )
    .unwrap();
    match update {
        SourceFormUpdate::Physical { transition, .. } => {
            assert_eq!(transition.expected_sample, 256);
            assert_eq!(
                transition.after.request().state_ref,
                body.body().request().state_ref
            );
            assert_eq!(transition.after.request().body_revision, 2);
            let ids: Vec<_> = transition
                .after
                .request()
                .geometry
                .nodes
                .iter()
                .map(|n| n.identity)
                .collect();
            assert_eq!(ids, (1..=12).collect::<Vec<_>>());
            assert!(!physical_equal(
                &transition.after.request().geometry,
                &body.body().request().geometry
            ));
            assert!(
                transition
                    .after
                    .request()
                    .geometry
                    .edges
                    .iter()
                    .all(|e| e.prestress_newtons == 5.0)
            );
        }
        _ => panic!("actual fold operation had no physical effect"),
    }
    assert!(
        prepare_source_form_update(
            &body,
            &source,
            SourceFormUpdateRequest {
                coordinate: source_form_coordinate(&source, MFace::Pratibimba).unwrap(),
                recipe: recipe.clone(),
                controls: controls(&source, 2),
                expected_body_revision: 0,
                expected_sample: 256,
                policy: FormTransitionPolicy::ProjectCorrespondingNodes,
            }
        )
        .is_err()
    );
    let mut bad = recipe.clone();
    bad.provenance.standing = PhysicalStanding::SourceAuthored;
    assert!(source_form_geometry(&source, &bad).is_err());
    let mut bad = recipe.clone();
    bad.frame_side_metres = f64::NAN;
    assert!(source_form_geometry(&source, &bad).is_err());
    let mut bad = recipe.clone();
    bad.site_separation_metres = 0.01;
    assert!(source_form_geometry(&source, &bad).is_err());
    let mut bad = recipe.clone();
    bad.family = BodyFamily::AxialTruss;
    assert!(source_form_geometry(&source, &bad).is_err());
    let mut bad = recipe;
    bad.provenance.reference = "source\0forgery".into();
    assert!(source_form_geometry(&source, &bad).is_err());
    assert!(
        serde_json::from_value::<SourceGeometryRecipe>(json!({"geometry":"glyph:AAA"})).is_err()
    );
}
