//! Actual native M1 -> joined M2/B -> K per-key targets -> P body preparation.
//! Metric material/tuning below have explicit Reference standing; they are not
//! authentic maqam pitches, a mocked engine or measured physical construction.
use ql_core::{ConjugationDegree, RelationFamily};
use ql_mef::MFace;
use ql_mef::continuous::coupled::{CoupledInput, HarmonicSource, REQUEST};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_engine::M2Request;
use ql_mef::m2_relation_plan::M2RelationPlanContext;
use ql_mef::m3_engine::{M3NodeKind, native_m3_engine};
use ql_mef::m3_state::{M3Request, M3State};
use ql_mef::music_determination::{
    ExactRatio, Fundamental, RelationSelection, TuningPolicy, TuningProvenance, TuningStanding,
};
use ql_mef::performance_audio::*;
use ql_mef::physical_body::*;
use serde_json::{Value, json};
fn tuning_provenance() -> TuningProvenance {
    TuningProvenance {
        policy_ref: "reference:explicit-rational-tuning".into(),
        source_ref: "reference:performer-score".into(),
        revision: "1".into(),
        standing: TuningStanding::Reference,
    }
}
fn physical_provenance(name: &str) -> PhysicalProvenance {
    PhysicalProvenance {
        reference: name.into(),
        revision: "1".into(),
        source_ref: "reference:analytic-bar".into(),
        standing: PhysicalStanding::Reference,
    }
}
pub fn preparation() -> PerformancePreparationInput {
    let m1_seed: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(m1_seed["config"].clone()).unwrap();
    let b: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/m2-relation-plan-input-v1.json"
    ))
    .unwrap();
    let m2: M2Request = serde_json::from_value(b["nativeRequest"].clone()).unwrap();
    let context: M2RelationPlanContext =
        serde_json::from_value(b["relationContext"].clone()).unwrap();
    let m3_seed: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_seed["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m1.revision = "11".into();
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    m1.selected_coordinate = "#1-3".into();
    let command = ql_mef::m3_state::M3Command {
        schema: ql_mef::m3_state::COMMAND_SCHEMA.into(),
        event_ref: m3.stamp.identity.event_ref.clone(),
        subject_ref: m3.subject_ref.clone(),
        expected_generation: m3.stamp.identity.profile_generation,
        actor_ref: "reference:retained-performer".into(),
        cause_ref: "reference:source-defined-score-act".into(),
        occurrence_unix_ms: m3.occurrence_unix_ms + 1,
        receipt_unix_ms: m3.receipt_unix_ms + 1,
        operations: vec![
            ql_mef::m3_state::M3Operation::ApplyMatrix { family: 1 },
            ql_mef::m3_state::M3Operation::AdvanceClock { steps: 721 },
        ],
    };
    let mut state = M3State::new(m3.clone()).unwrap();
    let receipt = state.apply(command.clone()).unwrap();
    assert_eq!(receipt.status, "applied");
    let form = native_m3_engine()
        .node(
            M3NodeKind::Codon,
            usize::from(state.fold().codon().address()),
        )
        .unwrap();
    let physical = BodyPreparationRequest {
        expected_m3_generation: state.generation(),
        body_revision: 1,
        preparation_ref: "reference:performance/body-preparation".into(),
        state_ref: "reference:performance/body-state".into(),
        geometry: MetricGeometry {
            provenance: physical_provenance("reference:metric-axial-bar"),
            family: BodyFamily::AxialTruss,
            nodes: vec![
                PhysicalNode {
                    identity: 1,
                    constituent: form.source_ref.clone(),
                    rest_metres: [0.0; 3],
                    additional_mass_kg: 0.0,
                    fixed: [true; 3],
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
            provenance: physical_provenance("reference:elastic-material"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 2.0,
            damping_alpha_per_second: 4.0,
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
    };
    let pairs = [
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
    ];
    PerformancePreparationInput {
        coupled: CoupledInput {
            schema: REQUEST.into(),
            m1,
            m2,
            m3,
            m3_commands: vec![command],
            harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
            frequency_bindings: vec![],
            condition_frequency_bindings: vec![],
            sky_frequency_bindings: vec![],
            source_receipts: vec![],
        },
        relation_context: context,
        source_face: MFace::Pratibimba,
        physical_face: MFace::Pratibimba,
        excitation: ExcitationPolicy {
            policy_ref: "proposal:D30/native-root-and-octet/v1".into(),
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
        fundamental_scaling: FundamentalScaling::NativeM1HarmonicRatio,
        tuning: TuningPolicy::ExactPitchRatios {
            ratios: pairs.map(|(n, d)| ExactRatio::new(n, d).unwrap()),
            provenance: tuning_provenance(),
        },
        require_authentic_condition_tuning: false,
        physical,
        instance_ref: "expression:retained/current".into(),
        receipt_ref: "receipt:native/performance-admission".into(),
        touches: (0..12)
            .map(|key| KeyTouch {
                key,
                register: 0,
                member: u64::from(key) + 1,
                touch: u64::from(key) + 1,
                touch_ref: format!("touch:janko/{key}"),
            })
            .collect(),
    }
}
