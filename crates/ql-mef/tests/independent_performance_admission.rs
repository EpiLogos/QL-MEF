//! Independent V/#293 trials of real source-qualified audio preparation.
//! Root may copy this unchanged into ql-mef/tests under finite CI admission.
//! Inputs are controlled native fixtures and a declared Reference mechanical
//! realisation. No mocked producer, device, host authority or human verdict.
use ql_core::{ConjugationDegree, RelationFamily};
use ql_mef::MFace;
use ql_mef::continuous::coupled::{CoupledInput, HarmonicSource, REQUEST};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_condition::CorrespondenceRole;
use ql_mef::m2_engine::M2Request;
use ql_mef::m2_relation_plan::M2RelationPlanContext;
use ql_mef::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation, M3Request, M3State};
use ql_mef::music_determination::{
    ExactRatio, Fundamental, RelationSelection, TuningPolicy, TuningProvenance, TuningStanding,
};
use ql_mef::performance_audio::*;
use ql_mef::physical_body::*;
use ql_mef::source_form_body::{
    SOURCE_GEOMETRY_BASIS, SourceGeometryRecipe, admit_source_form_metric, source_form_geometry,
};
use serde_json::{Value, json};

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

#[test]
fn v293_missing_descendant_refuses_actual_audio_admission_after_partial_m2_gap() {
    let resident = baseline();
    let resident_before = serde_json::to_value(&resident).unwrap();
    let mut missing = preparation();
    missing.coupled.m2.condition.as_mut().unwrap().maqam_index = 4;
    missing.relation_context.requested_maqam_index = Some(4);
    missing.relation_context.musical.maqam_ref = None;
    // The actual native producer legitimately returns a partial frame.
    let partial = missing
        .coupled
        .compose()
        .expect("M2 gap must remain a partial producer result");
    assert!(partial.m2["condition"]["source_path"].is_null());
    assert!(
        !partial.m2["condition"]["gaps"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let error = prepare_native_performance(missing).unwrap_err();
    assert!(
        error.contains("requested maqam/role is disconnected"),
        "{error}"
    );
    // This is immutable control admission; host transactional/live replacement
    // remains a separate required installed trial. No host behaviour is claimed.
    assert_eq!(serde_json::to_value(&resident).unwrap(), resident_before);
    resident
        .validate_native_consumers(resident.native_basis(), resident.physical_body())
        .unwrap();
}

#[test]
fn v293_disconnected_writer_cannot_pass_actual_receiver_with_source_labels_intact() {
    let prepared = baseline();
    let mut output = prepared.native_basis().clone();
    output.m2["vimarsha"]["reading"]["audio_octet_hz"][0] = json!(440.0);
    assert!(
        prepared
            .validate_native_consumers(&output, prepared.physical_body())
            .is_err()
    );
    let mut output = prepared.native_basis().clone();
    output.m2_input.vimarsha = None;
    assert!(
        prepared
            .validate_native_consumers(&output, prepared.physical_body())
            .is_err()
    );
    let mut input = preparation();
    input.coupled.m2.vimarsha = None;
    assert!(prepare_native_performance(input).is_err());
}

#[test]
fn v293_lost_metric_descendant_and_stale_generation_refuse_actual_body_admission() {
    baseline();
    let mut input = preparation();
    let exact = input.physical.geometry.nodes[0].constituent.clone();
    assert_ne!(exact, "#3-0");
    for node in &mut input.physical.geometry.nodes {
        if node.constituent == exact {
            node.constituent = "#3-0".into();
        }
    }
    let error = prepare_native_performance(input).unwrap_err();
    assert!(error.contains("metric body disconnected"), "{error}");
    let mut input = preparation();
    input.physical.expected_m3_generation += 1;
    let error = prepare_native_performance(input).unwrap_err();
    assert!(error.contains("stale M3 body preparation"), "{error}");
}

#[test]
fn v293_wrong_physical_face_is_distinct_from_a_valid_separately_admitted_face() {
    let prime = baseline();
    let mut direct = preparation();
    direct.physical_face = MFace::Bimba;
    let direct = prepare_native_performance(direct).expect("declared native face is selectable");
    assert_eq!(
        direct.physical_body().source_coordinate().face,
        MFace::Bimba
    );
    assert_eq!(
        prime.physical_body().request(),
        direct.physical_body().request()
    );
    assert!(
        prime
            .validate_native_consumers(prime.native_basis(), direct.physical_body())
            .is_err()
    );
    direct
        .validate_native_consumers(direct.native_basis(), direct.physical_body())
        .unwrap();
}

#[test]
fn v293_native_source_face_changes_phase_without_creating_a_second_pitch_or_body_source() {
    let prime = baseline();
    let mut direct = preparation();
    direct.source_face = MFace::Bimba;
    let direct = prepare_native_performance(direct).unwrap();
    assert_eq!(direct.determination()["m1_face"], 0);
    assert_eq!(
        prime.determination()["audio_octet_hz"],
        direct.determination()["audio_octet_hz"]
    );
    assert_eq!(
        prime.physical_body().request(),
        direct.physical_body().request()
    );
    for (a, b) in prime.notes().iter().zip(direct.notes()) {
        assert_eq!(a["hertz"], b["hertz"]);
        assert_eq!(b["source_face"], 0);
        assert!(
            (a["phase_cos"].as_f64().unwrap() + b["phase_cos"].as_f64().unwrap()).abs() < 1e-12
        );
        assert!(
            (a["phase_sin"].as_f64().unwrap() + b["phase_sin"].as_f64().unwrap()).abs() < 1e-12
        );
    }
}

#[test]
fn v293_unsupported_authentic_tuning_and_cross_event_cannot_be_admitted_as_reference() {
    let resident = baseline();
    let mut input = preparation();
    input.require_authentic_condition_tuning = true;
    let error = prepare_native_performance(input).unwrap_err();
    assert!(error.contains("source-unavailable"), "{error}");
    let mut input = preparation();
    input.coupled.m1.event_ref = "controlled:v293/other-event".into();
    assert!(prepare_native_performance(input).is_err());
    let mut input = preparation();
    input.physical.state_ref = "controlled:v293/other-resident-body".into();
    let other = prepare_native_performance(input).unwrap();
    assert!(
        resident
            .validate_native_consumers(resident.native_basis(), other.physical_body())
            .is_err()
    );
}

#[test]
fn v293_actual_source_form_regeneration_refuses_changed_metric_and_relabelled_old_body() {
    baseline();
    let input = preparation();
    let mut state = M3State::new(input.coupled.m3.clone()).unwrap();
    let geometry = source_form_geometry(&state, &recipe()).unwrap();
    admit_source_form_metric(&state, &recipe(), &geometry).unwrap();
    let mut wrong = geometry.clone();
    wrong.nodes[8].rest_metres[0] += 0.005;
    assert!(admit_source_form_metric(&state, &recipe(), &wrong).is_err());
    let before = state.snapshot();
    let receipt = state
        .apply(M3Command {
            schema: COMMAND_SCHEMA.into(),
            event_ref: input.coupled.m3.stamp.identity.event_ref.clone(),
            subject_ref: input.coupled.m3.subject_ref.clone(),
            expected_generation: state.generation(),
            actor_ref: "controlled:v293/performer".into(),
            cause_ref: "controlled:v293/line-change".into(),
            occurrence_unix_ms: input.coupled.m3.occurrence_unix_ms + 1,
            receipt_unix_ms: input.coupled.m3.receipt_unix_ms + 1,
            operations: vec![M3Operation::ChangeLine { line: 0 }],
        })
        .unwrap();
    assert_eq!(receipt.status, "applied");
    assert_ne!(state.snapshot()["form"], before["form"]);
    let next = source_form_geometry(&state, &recipe()).unwrap();
    assert_ne!(next, geometry);
    admit_source_form_metric(&state, &recipe(), &next).unwrap();
    // Transfer all source labels from the new real producer onto the old
    // resident metric. This must not stand for a source-derived physical body.
    let mut relabelled = geometry;
    for (old, new) in relabelled.nodes.iter_mut().zip(&next.nodes) {
        old.constituent.clone_from(&new.constituent);
    }
    assert!(admit_source_form_metric(&state, &recipe(), &relabelled).is_err());
}

#[test]
fn v293_valid_dominant_branch_keeps_its_distinct_native_relations_without_fabricated_material() {
    let tonic = baseline();
    let mut input = preparation();
    input.coupled.m2.condition.as_mut().unwrap().role = CorrespondenceRole::Dominant;
    input.relation_context.requested_role = Some(CorrespondenceRole::Dominant);
    // Explicit source context follows the selected branch; no parent fallback.
    input.relation_context.source_coordinates =
        ["#2-0", "#2-1", "#2-4.3-0-4", "#2-5-0/1", "#2-5-0/1-7"]
            .map(String::from)
            .to_vec();
    let dominant = prepare_native_performance(input).unwrap();
    assert_eq!(
        dominant
            .relation_plan()
            .situated
            .native_alignment
            .planet_coordinate,
        "#2-5-0/1"
    );
    assert_eq!(
        dominant
            .relation_plan()
            .situated
            .native_alignment
            .chakra_coordinate,
        "#2-5-0/1-7"
    );
    assert!(dominant.native_basis().m2["condition"]["source_path"]["tattva_coordinate"].is_null());
    assert!(dominant.native_basis().m2["condition"]["colour"]["source_name"].is_null());
    assert_eq!(
        tonic.determination()["audio_octet_hz"],
        dominant.determination()["audio_octet_hz"]
    );
    assert_eq!(
        tonic.physical_body().request(),
        dominant.physical_body().request()
    );
    assert_eq!(tonic.notes(), dominant.notes());
    assert!(
        tonic
            .validate_native_consumers(dominant.native_basis(), tonic.physical_body())
            .is_err()
    );
    dominant
        .validate_native_consumers(dominant.native_basis(), dominant.physical_body())
        .unwrap();
}
