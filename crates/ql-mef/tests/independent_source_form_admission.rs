// Independent V/#293 trials of real source-qualified audio preparation.
// Successor trials after root's source-form repair; original eight-activity
// file remains byte-frozen. This file has not been compiled or executed by V.
// The same independently constructed actual native inputs and Reference
// mechanical recipe are retained so the admission repair is discriminated.
use ql_core::{ConjugationDegree, RelationFamily};
use ql_mef::MFace;
use ql_mef::continuous::coupled::{CoupledInput, HarmonicSource, REQUEST};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_engine::M2Request;
use ql_mef::m2_relation_plan::M2RelationPlanContext;
use ql_mef::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation, M3Request, M3State};
use ql_mef::music_determination::{
    ExactRatio, Fundamental, RelationSelection, TuningPolicy, TuningProvenance, TuningStanding,
};
use ql_mef::performance_audio::*;
use ql_mef::physical_body::*;
use ql_mef::source_form_body::{SOURCE_GEOMETRY_BASIS, SourceGeometryRecipe, source_form_geometry};
use serde_json::Value;

fn fixture(path: &str) -> Value {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    serde_json::from_slice(&std::fs::read(root.join("../../fixtures/kernel").join(path)).unwrap())
        .unwrap()
}
fn physical_provenance(reference: &str) -> PhysicalProvenance {
    PhysicalProvenance {
        reference: reference.into(),
        revision: "v293-controlled-reference-1".into(),
        source_ref: SOURCE_GEOMETRY_BASIS.into(),
        standing: PhysicalStanding::Reference,
    }
}
fn tuning_provenance() -> TuningProvenance {
    TuningProvenance {
        policy_ref: "reference:v293/declared-rational-targets".into(),
        source_ref: "controlled:nonprivate-performer-score".into(),
        revision: "1".into(),
        standing: TuningStanding::Reference,
    }
}
fn recipe() -> SourceGeometryRecipe {
    SourceGeometryRecipe {
        provenance: physical_provenance("reference:v293/elastic-fold-frame-law"),
        family: BodyFamily::AxialTruss,
        frame_side_metres: 0.1,
        site_separation_metres: 0.25,
        section_by_element_m2: [1e-6, 2e-6, 3e-6, 4e-6],
        intersite_section_m2: 2e-6,
        prestress_newtons: 0.0,
    }
}
fn preparation() -> PerformancePreparationInput {
    let mut m1: EngineConfig =
        serde_json::from_value(fixture("m1-engine-v1.request.json")["config"].clone()).unwrap();
    let b = fixture("m2-relation-plan-input-v1.json");
    let mut m2: M2Request = serde_json::from_value(b["nativeRequest"].clone()).unwrap();
    let mut relation_context: M2RelationPlanContext =
        serde_json::from_value(b["relationContext"].clone()).unwrap();
    // Freshly read exact Kirdan descendant; catalogue index 3 is not its
    // authored coordinate and must not replace the compound source spelling.
    m2.condition.as_mut().unwrap().maqam_index = 3;
    relation_context.requested_maqam_index = Some(3);
    relation_context.musical.maqam_ref = Some("#2-4.3-0-4".into());
    relation_context.source_coordinates = [
        "#2-0",
        "#2-1",
        "#2-4.3-0-4",
        "#2-5-5",
        "#2-5-0/1-1",
        "#2-2-2-5-5",
    ]
    .map(String::from)
    .to_vec();
    let mut m3: M3Request =
        serde_json::from_value(fixture("m3-parent-consumer-current-v1.json")["request"].clone())
            .unwrap();
    // Join the fixture events explicitly, retaining their native generations.
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m1.revision = "11".into();
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    let state = M3State::new(m3.clone()).unwrap();
    let geometry = source_form_geometry(&state, &recipe()).unwrap();
    let mut weights = vec![0.0; geometry.nodes.len()];
    weights[10] = 1.0;
    let physical = BodyPreparationRequest {
        expected_m3_generation: state.generation(),
        body_revision: 1,
        preparation_ref: "controlled:v293/source-body-preparation".into(),
        state_ref: "controlled:v293/resident-body".into(),
        geometry,
        material: PhysicalMaterial {
            provenance: physical_provenance("reference:v293/elastic-material"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 1000.0,
            damping_alpha_per_second: 4.0,
            damping_beta_seconds: 0.0,
        },
        sample_rate: 48000,
        exciter: SpatialProjection {
            axis: [1.0, 0.0, 0.0],
            node_weights: weights.clone(),
        },
        pickup: SpatialProjection {
            axis: [1.0, 0.0, 0.0],
            node_weights: weights,
        },
        pickup_linear_per_metre: 1000.0,
        max_force_newtons: 1.0,
        max_impulse_newton_seconds: 0.001,
        max_displacement_metres: 0.1,
    };
    let ratios = [
        (1, 1),
        (256, 243),
        (9, 8),
        (32, 27),
        (81, 64),
        (4, 3),
        (729, 512),
        (3, 2),
        (128, 81),
        (27, 16),
        (16, 9),
        (243, 128),
    ]
    .map(|(n, d)| ExactRatio::new(n, d).unwrap());
    PerformancePreparationInput {
        coupled: CoupledInput {
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
        },
        relation_context,
        source_face: MFace::Pratibimba,
        physical_face: MFace::Pratibimba,
        excitation: ExcitationPolicy {
            policy_ref: "proposal:D30/v293/native-root-and-octet-v1".into(),
            standing: "agent-proposed-implementation-policy".into(),
            scaling: OctetScaling::NoteRelativeToReference,
            reference_hertz: 220.0,
            root_linear: 0.5,
            octet_linear: 0.5,
            weights: [0.125; 8],
        },
        relation: RelationSelection {
            family: RelationFamily::C,
            pair_index: 1,
            degree: ConjugationDegree::D3,
            expansion_side: None,
        },
        fundamental: Fundamental::new(220.0, tuning_provenance()).unwrap(),
        fundamental_scaling: FundamentalScaling::AbsoluteReference,
        tuning: TuningPolicy::ExactPitchRatios {
            ratios,
            provenance: tuning_provenance(),
        },
        require_authentic_condition_tuning: false,
        physical,
        instance_ref: "controlled:v293/existing-expression-instance".into(),
        receipt_ref: "controlled:v293/actual-native-admission".into(),
        touches: (0..12)
            .map(|key| KeyTouch {
                key,
                register: 0,
                member: u64::from(key) + 1,
                touch: u64::from(key) + 1,
                touch_ref: format!("controlled:v293/touch/{key}"),
            })
            .collect(),
    }
}
fn baseline() -> PreparedPerformanceBinding {
    let prepared =
        prepare_native_performance(preparation()).expect("connected baseline must admit");
    assert_eq!(prepared.notes().len(), 12);
    assert_eq!(prepared.physical_body().request().geometry.nodes.len(), 12);
    assert_eq!(prepared.physical_body().request().geometry.edges.len(), 34);
    assert_eq!(
        prepared.physical_body().source_coordinate().face,
        MFace::Pratibimba
    );
    assert_eq!(prepared.determination()["m2_writer"], "#2-1");
    assert_eq!(prepared.determination()["m2_face"], 1);
    assert_eq!(
        prepared.determination()["audio_octet_hz"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    assert_eq!(
        prepared.determination()["nodal_quartet"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
    assert_eq!(
        prepared
            .relation_plan()
            .situated
            .native_alignment
            .maqam_coordinate,
        "#2-4.3-0-4"
    );
    assert_eq!(
        prepared
            .relation_plan()
            .situated
            .native_alignment
            .planet_coordinate,
        "#2-5-5"
    );
    assert_eq!(
        prepared
            .relation_plan()
            .situated
            .native_alignment
            .chakra_coordinate,
        "#2-5-0/1-1"
    );
    assert_eq!(
        prepared.native_basis().m2["condition"]["source_path"]["maqam_node_id"],
        "ec7c57f12cefdfd8"
    );
    assert!(
        prepared
            .relation_plan()
            .execution
            .intended_tuning_hz
            .is_none()
    );
    assert_eq!(
        prepared.targets().tuning_policy.provenance().standing,
        TuningStanding::Reference
    );
    let pitches: std::collections::BTreeSet<_> = prepared
        .notes()
        .iter()
        .map(|note| note["pitch_class"].as_u64().unwrap())
        .collect();
    assert_eq!(pitches.len(), 12);
    for note in prepared.notes() {
        assert_eq!(note["identity"], prepared.determination()["identity"]);
        assert_eq!(note["source_face"], 1);
        assert_eq!(note["exact_ratio"], true);
    }
    prepared
        .validate_native_consumers(prepared.native_basis(), prepared.physical_body())
        .unwrap();
    prepared
}

fn command(input: &PerformancePreparationInput, operations: Vec<M3Operation>) -> M3Command {
    M3Command {
        schema: COMMAND_SCHEMA.into(),
        event_ref: input.coupled.m3.stamp.identity.event_ref.clone(),
        subject_ref: input.coupled.m3.subject_ref.clone(),
        expected_generation: input.coupled.m3.stamp.identity.profile_generation,
        actor_ref: "controlled:v293/performer".into(),
        cause_ref: "controlled:v293/source-form-change".into(),
        occurrence_unix_ms: input.coupled.m3.occurrence_unix_ms + 1,
        receipt_unix_ms: input.coupled.m3.receipt_unix_ms + 1,
        operations,
    }
}
fn source_controls(
    request: &BodyPreparationRequest,
) -> ql_mef::source_form_body::SourceBodyControls {
    ql_mef::source_form_body::SourceBodyControls {
        expected_m3_generation: request.expected_m3_generation,
        body_revision: request.body_revision,
        preparation_ref: request.preparation_ref.clone(),
        state_ref: request.state_ref.clone(),
        material: request.material.clone(),
        sample_rate: request.sample_rate,
        exciter: request.exciter.clone(),
        pickup: request.pickup.clone(),
        pickup_linear_per_metre: request.pickup_linear_per_metre,
        max_force_newtons: request.max_force_newtons,
        max_impulse_newton_seconds: request.max_impulse_newton_seconds,
        max_displacement_metres: request.max_displacement_metres,
    }
}

#[test]
fn v293_stronger_source_form_entry_retains_recipe_and_real_native_receiver() {
    let generic = baseline();
    assert!(generic.source_form_recipe().is_none());
    let input = preparation();
    let state = M3State::new(input.coupled.m3.clone()).unwrap();
    assert!(generic.validate_source_form_consumer(&state).is_err());
    let source = prepare_source_form_performance(input, recipe()).unwrap();
    assert_eq!(source.source_form_recipe(), Some(&recipe()));
    source.validate_source_form_consumer(&state).unwrap();
    source
        .validate_native_consumers(source.native_basis(), source.physical_body())
        .unwrap();
    assert_eq!(source.notes(), generic.notes());
    assert_eq!(
        source.physical_body().request(),
        generic.physical_body().request()
    );
}

#[test]
fn v293_provider_metric_label_cannot_claim_native_source_form_geometry() {
    baseline();
    let mut metric = preparation();
    metric.physical.geometry.nodes[8].rest_metres[0] += 0.005;
    let state = M3State::new(metric.coupled.m3.clone()).unwrap();
    let provider = prepare_native_performance(metric)
        .expect("bounded Reference metric is a valid separate provider role");
    assert!(provider.source_form_recipe().is_none());
    assert!(provider.validate_source_form_consumer(&state).is_err());
    let mut source = preparation();
    source.physical.geometry.nodes[8].rest_metres[0] += 0.005;
    let error = prepare_source_form_performance(source, recipe()).unwrap_err();
    assert!(
        error.contains("source form metric is stale, relabelled or disconnected"),
        "{error}"
    );
}

#[test]
fn v293_real_native_line_change_refuses_relabelled_old_metric_and_admits_regenerated_body() {
    let initial = baseline();
    let mut next = preparation();
    let mut state = M3State::new(next.coupled.m3.clone()).unwrap();
    let act = command(&next, vec![M3Operation::ChangeLine { line: 0 }]);
    assert_eq!(state.apply(act.clone()).unwrap().status, "applied");
    let fresh = source_form_geometry(&state, &recipe()).unwrap();
    next.coupled.m3_commands.push(act.clone());
    next.physical.expected_m3_generation = state.generation();
    next.physical.body_revision = 2;
    for (old, new) in next.physical.geometry.nodes.iter_mut().zip(&fresh.nodes) {
        old.constituent.clone_from(&new.constituent);
    }
    let error = prepare_source_form_performance(next, recipe()).unwrap_err();
    assert!(
        error.contains("source form metric is stale, relabelled or disconnected"),
        "{error}"
    );
    let mut next = preparation();
    next.coupled.m3_commands.push(act);
    next.physical.expected_m3_generation = state.generation();
    next.physical.body_revision = 2;
    next.physical.geometry = fresh;
    let current = prepare_source_form_performance(next, recipe()).unwrap();
    current.validate_source_form_consumer(&state).unwrap();
    assert_ne!(
        initial.physical_body().request().geometry,
        current.physical_body().request().geometry
    );
    assert_eq!(current.notes().len(), 12);
    assert_eq!(
        current.physical_body().source_generation(),
        state.generation()
    );
    let old_state = M3State::new(preparation().coupled.m3).unwrap();
    assert!(current.validate_source_form_consumer(&old_state).is_err());
}

#[test]
fn v293_source_form_missing_relation_still_refuses_after_geometry_admission() {
    let connected = prepare_source_form_performance(preparation(), recipe()).unwrap();
    let original = serde_json::to_value(&connected).unwrap();
    let mut missing = preparation();
    missing.coupled.m2.condition.as_mut().unwrap().maqam_index = 4;
    missing.relation_context.requested_maqam_index = Some(4);
    missing.relation_context.musical.maqam_ref = None;
    let error = prepare_source_form_performance(missing, recipe()).unwrap_err();
    assert!(
        error.contains("requested maqam/role is disconnected"),
        "{error}"
    );
    assert_eq!(serde_json::to_value(&connected).unwrap(), original);
}

#[test]
fn v293_aperture_and_clock_readings_use_existing_p_read_only_policy_without_metric_change() {
    use ql_mef::source_form_body::{
        SourceFormUpdate, SourceFormUpdateRequest, prepare_source_form_body,
        prepare_source_form_update, source_form_coordinate,
    };
    baseline();
    let mut input = preparation();
    let mut state = M3State::new(input.coupled.m3.clone()).unwrap();
    let before_geometry = source_form_geometry(&state, &recipe()).unwrap();
    let body = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        recipe(),
        source_controls(&input.physical),
    )
    .unwrap();
    let act = command(
        &input,
        vec![
            M3Operation::SetAperture { aperture: 15 },
            M3Operation::AdvanceClock { steps: 721 },
        ],
    );
    assert_eq!(state.apply(act.clone()).unwrap().status, "applied");
    assert_eq!(
        source_form_geometry(&state, &recipe()).unwrap(),
        before_geometry
    );
    let mut controls = source_controls(&input.physical);
    controls.expected_m3_generation = state.generation();
    controls.body_revision = 2;
    controls.preparation_ref = "controlled:v293/next-reading".into();
    let update = prepare_source_form_update(
        &body,
        &state,
        SourceFormUpdateRequest {
            coordinate: source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
            recipe: recipe(),
            controls,
            expected_body_revision: 1,
            expected_sample: 731,
            policy: FormTransitionPolicy::ProjectCorrespondingNodes,
        },
    )
    .unwrap();
    match update {
        SourceFormUpdate::ReadingOnly {
            preparation_ref,
            state_ref,
            body_revision,
            source_reading,
        } => {
            assert_eq!(preparation_ref, body.body().request().preparation_ref);
            assert_eq!(state_ref, body.body().request().state_ref);
            assert_eq!(body_revision, 1);
            assert_eq!(source_reading, state.snapshot());
        }
        _ => panic!("aperture/clock reading silently became a physical transition"),
    }
    // Re-admit the actual updated native reading through A with the existing
    // metric/state identities; no callback cursor or PCM assertion is made.
    input.coupled.m3_commands.push(act);
    input.physical.expected_m3_generation = state.generation();
    let prepared = prepare_source_form_performance(input, recipe()).unwrap();
    prepared.validate_source_form_consumer(&state).unwrap();
    assert_eq!(prepared.physical_body().request().geometry, before_geometry);
    assert_eq!(prepared.physical_body().request().body_revision, 1);
}
