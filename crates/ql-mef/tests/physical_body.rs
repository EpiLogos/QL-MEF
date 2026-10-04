//! The current native M3 producer feeds real metric-body admission. C++ tests
//! independently verify numerical dynamics; these verify exact source joins.
use ql_mef::m3_engine::{M3NodeKind, native_m3_engine};
use ql_mef::m3_state::{M3Request, M3State};
use ql_mef::physical_body::*;
use ql_mef::{MCoordinate, MFace};
use serde_json::{Value, json};
fn m3_state(address: u8, pose: u8, aperture: u8, clock_steps: u64) -> M3State {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut request: M3Request = serde_json::from_value(fixture["request"].clone()).unwrap();
    request.address = address;
    request.pose = pose;
    request.aperture = aperture;
    request.clock_steps = clock_steps;
    M3State::new(request).unwrap()
}
fn coordinate(state: &M3State, face: MFace) -> MCoordinate {
    let node = native_m3_engine()
        .node(
            M3NodeKind::Codon,
            usize::from(state.fold().codon().address()),
        )
        .unwrap();
    ql_mef::m_tree::native_current_m_registry()
        .coordinate(&node.source_ref, face)
        .unwrap()
}
fn provenance(reference: &str) -> PhysicalProvenance {
    PhysicalProvenance {
        reference: reference.into(),
        revision: "1".into(),
        source_ref: "controlled:analytic-physical-reference".into(),
        standing: PhysicalStanding::Reference,
    }
}
fn request(state: &M3State) -> BodyPreparationRequest {
    BodyPreparationRequest {
        expected_m3_generation: state.generation(),
        body_revision: 1,
        preparation_ref: "controlled:physical/preparation1".into(),
        state_ref: "controlled:physical/state".into(),
        geometry: MetricGeometry {
            provenance: provenance("controlled:metric-axial-bar"),
            family: BodyFamily::AxialTruss,
            nodes: vec![
                PhysicalNode {
                    identity: 1,
                    constituent: coordinate(state, MFace::Pratibimba).source_ref,
                    rest_metres: [0.0, 0.0, 0.0],
                    additional_mass_kg: 0.0,
                    fixed: [true, true, true],
                },
                PhysicalNode {
                    identity: 2,
                    constituent: "#3-0".into(),
                    rest_metres: [1.0, 0.0, 0.0],
                    additional_mass_kg: 0.0,
                    fixed: [false, true, true],
                },
            ],
            edges: vec![PhysicalEdge {
                first: 0,
                second: 1,
                section_m2: 1e-4,
                prestress_newtons: 0.0,
            }],
        },
        material: PhysicalMaterial {
            provenance: provenance("controlled:elastic-material"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 2.0,
            damping_alpha_per_second: 0.4,
            damping_beta_seconds: 0.0,
        },
        sample_rate: 48000,
        exciter: SpatialProjection {
            axis: [1.0, 0.0, 0.0],
            node_weights: vec![0.0, 1.0],
        },
        pickup: SpatialProjection {
            axis: [1.0, 0.0, 0.0],
            node_weights: vec![0.0, 1.0],
        },
        pickup_linear_per_metre: 1000.0,
        max_force_newtons: 10.0,
        max_impulse_newton_seconds: 0.01,
        max_displacement_metres: 0.1,
    }
}
#[test]
fn actual_m3_form_clock_aperture_and_face_survive_metric_preparation() {
    let state = m3_state(0, 0, 15, 721);
    let before = state.snapshot();
    let body = prepare_source_body(
        &state,
        coordinate(&state, MFace::Pratibimba),
        request(&state),
    )
    .unwrap();
    assert_eq!(body.address(), 0);
    assert_eq!(body.source_coordinate().source_ref, "#3-2-1-1-1");
    assert_eq!(source_face(&body), MFace::Pratibimba);
    assert_eq!(body.clock(), &before["clock"]);
    assert_eq!(body.aperture(), &before["aperture"]);
    assert_eq!(body.aperture()["total_lenses"], 18);
    assert_eq!(body.clock()["degree720"], 1);
    assert_eq!(body.form(), &before["form"]);
    assert_eq!(
        body.source_revision(),
        ql_mef::m3_source::native_m3_source().source_revision()
    );
    assert_eq!(state.snapshot(), before);
    let envelope = serde_json::to_value(&body).unwrap();
    assert_eq!(envelope["units"]["excitation"], "N");
    assert_eq!(envelope["units"]["impulse"], "N*s");
    assert_eq!(envelope["units"]["elastic_modulus"], "Pa");
    assert_eq!(envelope["request"]["geometry"]["family"], "axial-truss");
    assert_eq!(
        envelope["request"]["material"]["provenance"]["standing"],
        "reference"
    );
    let wire = &envelope["source_coordinate"];
    assert_eq!(
        readmit_source_coordinate(wire).unwrap(),
        *body.source_coordinate()
    );
    assert_eq!(wire["face"], "pratibimba");
    assert_eq!(
        wire["provenance"].as_array().unwrap().len(),
        body.source_coordinate().provenance.len()
    );
    assert_eq!(
        wire["payloads"].as_array().unwrap().len(),
        body.source_coordinate().payloads.len()
    );
    let text = serde_json::to_string(&envelope).unwrap();
    let reparsed: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
        readmit_source_coordinate(&reparsed["source_coordinate"]).unwrap(),
        *body.source_coordinate()
    );
    for reference in ["#3-4.0", "#3-5-5/0"] {
        let coordinate = ql_mef::m_tree::native_current_m_registry()
            .coordinate(reference, MFace::Pratibimba)
            .unwrap();
        assert_eq!(
            readmit_source_coordinate(&source_coordinate_wire(&coordinate)).unwrap(),
            coordinate
        );
    }
    let mut forged = wire.clone();
    forged["canonical_ref"] = json!("ql:m-coordinate:bimba:M3-2-1-1-1");
    assert!(readmit_source_coordinate(&forged).is_err());
    let mut forged = wire.clone();
    forged["provenance"] = json!([]);
    assert!(readmit_source_coordinate(&forged).is_err());
    let mut forged = wire.clone();
    forged["separators"] = json!([".", ".", "."]);
    assert!(readmit_source_coordinate(&forged).is_err());
    let mut forged = wire.clone();
    forged["face"] = json!("unknown");
    assert!(readmit_source_coordinate(&forged).is_err());
}
#[test]
fn every_lawful_source_form_and_pose_is_an_exact_admission_not_a_parent_fallback() {
    let mut count = 0;
    for address in 0..64 {
        let first = m3_state(address, 0, address % 16, 720 + u64::from(address));
        for pose in 0..first.fold().state_count() {
            let state = m3_state(address, pose, address % 16, 720 + u64::from(address));
            let body = prepare_source_body(
                &state,
                coordinate(&state, MFace::Pratibimba),
                request(&state),
            )
            .unwrap();
            assert_eq!(body.address(), address);
            assert_eq!(
                body.pose_ordinal(),
                state.fold().rotational_pose().ordinal() as u16
            );
            assert_eq!(body.form()["state_count"], state.fold().state_count());
            assert_eq!(
                body.source_coordinate(),
                &coordinate(&state, MFace::Pratibimba)
            );
            count += 1;
        }
    }
    assert_eq!(count, 472);
}
#[test]
fn unknown_wrong_branch_unbacked_geometry_and_stale_source_are_refused() {
    let state = m3_state(0, 0, 0, 0);
    let current = coordinate(&state, MFace::Pratibimba);
    let registry = ql_mef::m_tree::native_current_m_registry();
    for reference in ["#3", "#3-0", "#3-4.0", "#3-5-5/0", "#2-1"] {
        let wrong = registry.coordinate(reference, MFace::Pratibimba).unwrap();
        assert!(prepare_source_body(&state, wrong, request(&state)).is_err());
    }
    assert!(registry.coordinate("#3-2-999", MFace::Pratibimba).is_err());
    let mut forged = current.clone();
    forged.source_ref = "#3-2-1-1-2".into();
    assert!(prepare_source_body(&state, forged, request(&state)).is_err());
    let mut stale = request(&state);
    stale.expected_m3_generation += 1;
    assert!(prepare_source_body(&state, current.clone(), stale).is_err());
    let mut disconnected = request(&state);
    disconnected.geometry.nodes[0].constituent = "#3-0".into();
    assert!(prepare_source_body(&state, current.clone(), disconnected).is_err());
    let mut no_metric = request(&state);
    no_metric.geometry.edges.clear();
    assert!(prepare_source_body(&state, current.clone(), no_metric).is_err());
    let mut bogus = request(&state);
    bogus.geometry.provenance.source_ref = "presentation\0hinge".into();
    assert!(prepare_source_body(&state, current, bogus).is_err());
}
#[test]
fn physical_units_connectivity_limits_and_constitutive_family_are_enforced() {
    let state = m3_state(0, 0, 0, 0);
    let source = coordinate(&state, MFace::Pratibimba);
    let refused = |request| assert!(prepare_source_body(&state, source.clone(), request).is_err());
    let mut bad = request(&state);
    bad.material.density_kg_per_m3 = 0.0;
    refused(bad);
    let mut bad = request(&state);
    bad.material.young_modulus_pa = f64::NAN;
    refused(bad);
    let mut bad = request(&state);
    bad.geometry.nodes[1].rest_metres = [0.0; 3];
    refused(bad);
    let mut bad = request(&state);
    bad.geometry.nodes[1].identity = 1;
    refused(bad);
    let mut bad = request(&state);
    bad.geometry.nodes[1].constituent = "#3-0-unknown".into();
    refused(bad);
    let mut bad = request(&state);
    bad.geometry.nodes[1].fixed = [true; 3];
    refused(bad);
    let mut bad = request(&state);
    bad.geometry.edges.push(bad.geometry.edges[0].clone());
    refused(bad);
    let mut bad = request(&state);
    bad.geometry.edges[0].second = 3;
    refused(bad);
    let mut bad = request(&state);
    bad.exciter.node_weights = vec![1.0, 1.0];
    refused(bad);
    let mut bad = request(&state);
    bad.exciter.axis = [1.0, 1.0, 0.0];
    refused(bad);
    let mut bad = request(&state);
    bad.geometry.edges[0].prestress_newtons = 100.0;
    refused(bad);
    let mut tension = request(&state);
    tension.geometry.family = BodyFamily::PrestressedTensionNetwork;
    tension.geometry.edges[0].prestress_newtons = 100.0;
    assert!(prepare_source_body(&state, source.clone(), tension).is_ok());
    let mut bad = request(&state);
    bad.geometry
        .nodes
        .resize(MAX_NODES + 1, bad.geometry.nodes[0].clone());
    refused(bad);
    let mut bad = request(&state);
    bad.max_force_newtons = f64::INFINITY;
    refused(bad);
    let mut bad = request(&state);
    bad.max_impulse_newton_seconds = 0.0;
    refused(bad);
    assert!(
        serde_json::from_value::<BodyPreparationRequest>(json!({"geometry":"glyph:AAA"})).is_err()
    );
}
#[test]
fn material_preparation_retains_state_event_geometry_and_clock_then_refuses_stale_updates() {
    let state = m3_state(0, 0, 15, 721);
    let current = prepare_source_body(
        &state,
        coordinate(&state, MFace::Pratibimba),
        request(&state),
    )
    .unwrap();
    let mut material = current.request().material.clone();
    material.provenance.revision = "2".into();
    material.young_modulus_pa *= 4.0;
    let next = prepare_material_update(
        &current,
        &state,
        1,
        material.clone(),
        2,
        "controlled:physical/preparation2".into(),
    )
    .unwrap();
    assert_eq!(next.request().geometry, current.request().geometry);
    assert_eq!(next.request().state_ref, current.request().state_ref);
    assert_eq!(next.event_ref(), current.event_ref());
    assert_eq!(next.subject_ref(), current.subject_ref());
    assert_eq!(next.source_coordinate(), current.source_coordinate());
    assert_eq!(next.clock(), current.clock());
    assert_eq!(next.source_generation(), current.source_generation());
    assert_eq!(next.request().body_revision, 2);
    assert!(
        prepare_material_update(&current, &state, 0, material.clone(), 2, "prep".into()).is_err()
    );
    assert!(
        prepare_material_update(&current, &state, 1, material.clone(), 1, "prep".into()).is_err()
    );
    let other = m3_state(1, 0, 15, 721);
    assert!(prepare_material_update(&current, &other, 1, material, 2, "prep".into()).is_err());
}
#[test]
fn audio_and_visible_receipts_must_name_the_same_body_state_revision_and_sample() {
    let state = m3_state(0, 0, 0, 0);
    let body = prepare_source_body(
        &state,
        coordinate(&state, MFace::Pratibimba),
        request(&state),
    )
    .unwrap();
    let receipt = BodyObservationReceipt {
        contract: CONTRACT.into(),
        event_ref: body.event_ref().into(),
        preparation_ref: body.request().preparation_ref.clone(),
        state_ref: body.request().state_ref.clone(),
        body_revision: 1,
        samples_elapsed: 512,
        pickup_linear: 0.125,
        mechanical_energy_joules: 1e-8,
    };
    assert!(body.validate_observation(&receipt, 512).is_ok());
    let mut wrong = receipt.clone();
    wrong.state_ref = "independent:shader-state".into();
    assert!(body.validate_observation(&wrong, 512).is_err());
    let mut wrong = receipt.clone();
    wrong.body_revision += 1;
    assert!(body.validate_observation(&wrong, 512).is_err());
    let mut wrong = receipt.clone();
    wrong.samples_elapsed -= 1;
    assert!(body.validate_observation(&wrong, 512).is_err());
    let mut wrong = receipt.clone();
    wrong.event_ref = "event:unrelated".into();
    assert!(body.validate_observation(&wrong, 512).is_err());
    let mut wrong = receipt;
    wrong.pickup_linear = f64::NAN;
    assert!(body.validate_observation(&wrong, 512).is_err());
}

#[test]
fn form_transition_consumes_actual_m3_operation_and_declares_state_transfer_policy() {
    use ql_mef::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation};
    let mut state = m3_state(0, 0, 0, 0);
    let body = prepare_source_body(
        &state,
        coordinate(&state, MFace::Pratibimba),
        request(&state),
    )
    .unwrap();
    let snapshot = state.snapshot();
    state
        .apply(M3Command {
            schema: COMMAND_SCHEMA.into(),
            event_ref: body.event_ref().into(),
            subject_ref: body.subject_ref().into(),
            expected_generation: state.generation(),
            actor_ref: "controlled:source-form-test".into(),
            cause_ref: "controlled:one-bit-neighbour".into(),
            occurrence_unix_ms: snapshot["occurrence_unix_ms"].as_u64().unwrap(),
            receipt_unix_ms: snapshot["receipt_unix_ms"].as_u64().unwrap(),
            operations: vec![M3Operation::ChangeLine { line: 0 }],
        })
        .unwrap();
    let mut next = request(&state);
    next.body_revision = 2;
    next.geometry.provenance.revision = "2".into();
    next.geometry.nodes[1].rest_metres[0] = 2.0;
    next.preparation_ref = "controlled:changed-form".into();
    let transition = prepare_form_transition(
        &body,
        &state,
        coordinate(&state, MFace::Pratibimba),
        next.clone(),
        1,
        512,
        FormTransitionPolicy::ProjectCorrespondingNodes,
    )
    .unwrap();
    assert_ne!(transition.after.address(), body.address());
    assert_eq!(transition.expected_sample, 512);
    assert_eq!(transition.after.source_generation(), state.generation());
    assert_eq!(
        transition.after.request().state_ref,
        body.request().state_ref
    );
    assert!(
        prepare_form_transition(
            &body,
            &state,
            coordinate(&state, MFace::Pratibimba),
            next.clone(),
            0,
            512,
            FormTransitionPolicy::ProjectCorrespondingNodes
        )
        .is_err()
    );
    next.geometry.nodes[1].identity = 3;
    assert!(
        prepare_form_transition(
            &body,
            &state,
            coordinate(&state, MFace::Pratibimba),
            next.clone(),
            1,
            512,
            FormTransitionPolicy::ProjectCorrespondingNodes
        )
        .is_err()
    );
    assert!(
        prepare_form_transition(
            &body,
            &state,
            coordinate(&state, MFace::Pratibimba),
            next,
            1,
            512,
            FormTransitionPolicy::ExplicitReset
        )
        .is_ok()
    );
}

#[test]
fn real_joined_nodal_quartet_becomes_explicit_constraints_without_extra_voices() {
    use ql_mef::continuous::coupled::{CoupledInput, HarmonicSource, REQUEST};
    let m1_seed: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: ql_mef::m1_engine::EngineConfig =
        serde_json::from_value(m1_seed["config"].clone()).unwrap();
    let m2: ql_mef::m2_engine::M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    let m3_seed: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_seed["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    let state = M3State::new(m3.clone()).unwrap();
    let input = CoupledInput {
        schema: REQUEST.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: vec![],
        condition_frequency_bindings: vec![],
        sky_frequency_bindings: vec![],
        source_receipts: vec![],
    };
    let basis = input.compose().unwrap();
    let body = prepare_source_body(
        &state,
        coordinate(&state, MFace::Pratibimba),
        request(&state),
    )
    .unwrap();
    let maps = std::array::from_fn(|i| NodalBoundaryMapping {
        quartet_index: i as u8,
        node_identities: vec![1],
        fixed_axes: [true, true, true],
    });
    let update = prepare_nodal_boundaries(
        &body,
        &state,
        &basis,
        NodalBoundaryUpdateRequest {
            mappings: maps.clone(),
            expected_body_revision: 1,
            expected_sample: 512,
            next_body_revision: 2,
            next_preparation_ref: "controlled:boundary2".into(),
        },
    )
    .unwrap();
    assert_eq!(update.source_reading.nodal_quartet.len(), 4);
    assert_eq!(
        update.transition.after.request().geometry.nodes.len(),
        body.request().geometry.nodes.len()
    );
    assert_eq!(
        update.transition.after.request().geometry.edges.len(),
        body.request().geometry.edges.len()
    );
    assert_eq!(
        update.transition.after.request().state_ref,
        body.request().state_ref
    );
    assert_eq!(update.transition.expected_sample, 512);
    assert_eq!(update.source_reading.audio_octet_hz.len(), 8);
    let mut conflict = maps.clone();
    conflict[3].fixed_axes = [false, true, true];
    assert!(
        prepare_nodal_boundaries(
            &body,
            &state,
            &basis,
            NodalBoundaryUpdateRequest {
                mappings: conflict,
                expected_body_revision: 1,
                expected_sample: 512,
                next_body_revision: 2,
                next_preparation_ref: "prep".into(),
            },
        )
        .is_err()
    );
    let mut unknown = maps.clone();
    unknown[3].node_identities = vec![999];
    assert!(
        prepare_nodal_boundaries(
            &body,
            &state,
            &basis,
            NodalBoundaryUpdateRequest {
                mappings: unknown,
                expected_body_revision: 1,
                expected_sample: 512,
                next_body_revision: 2,
                next_preparation_ref: "prep".into(),
            },
        )
        .is_err()
    );
    let mut duplicate = maps;
    duplicate[3].quartet_index = 0;
    assert!(
        prepare_nodal_boundaries(
            &body,
            &state,
            &basis,
            NodalBoundaryUpdateRequest {
                mappings: duplicate,
                expected_body_revision: 1,
                expected_sample: 512,
                next_body_revision: 2,
                next_preparation_ref: "prep".into(),
            },
        )
        .is_err()
    );
    let mut disconnected = basis.clone();
    disconnected.m2["vimarsha"]["reading"]["nodal_quartet"][0]["m"] = json!(99);
    let maps = std::array::from_fn(|i| NodalBoundaryMapping {
        quartet_index: i as u8,
        node_identities: vec![1],
        fixed_axes: [true, true, true],
    });
    assert!(
        prepare_nodal_boundaries(
            &body,
            &state,
            &disconnected,
            NodalBoundaryUpdateRequest {
                mappings: maps,
                expected_body_revision: 1,
                expected_sample: 512,
                next_body_revision: 2,
                next_preparation_ref: "prep".into(),
            },
        )
        .is_err()
    );
}
