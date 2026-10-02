// Existing actual retained M1/M2/M3 producer with an explicit, source-derived
// twelve-node elastic-frame realisation. Magnitudes have AgentProposed standing.
#[path = "retained_performance.rs"]
mod retained_performance;
use ql_mef::continuous::coupled::CoupledBasis;
use ql_mef::continuous::performance::*;
use ql_mef::m3_state::M3State;
use ql_mef::music_determination::TuningPolicy;
use ql_mef::performance_audio::{FundamentalScaling, OctetScaling};
use ql_mef::physical_body::*;
use ql_mef::source_form_body::*;
use ql_mef::source_key_determination::{
    ActiveKeyReduction, KeyDegreeAssignment, SourcePitchRequirement,
};
pub fn config(sparse: bool) -> (CoupledBasis, PerformanceConfig) {
    let input = retained_performance::preparation();
    let current = input.coupled.compose().unwrap();
    let mut state = M3State::new(input.coupled.m3.clone()).unwrap();
    for command in &input.coupled.m3_commands {
        assert_eq!(state.apply(command.clone()).unwrap().status, "applied");
    }
    let provenance = |name: &str| PhysicalProvenance {
        reference: name.into(),
        revision: "1".into(),
        source_ref: SOURCE_GEOMETRY_BASIS.into(),
        standing: PhysicalStanding::AgentProposed,
    };
    let mut weights = vec![0.0; 12];
    weights[11] = 1.0;
    let recipe = SourceGeometryRecipe {
        provenance: provenance("controlled:source-frame-law"),
        family: BodyFamily::PrestressedTensionNetwork,
        frame_side_metres: 0.1,
        site_separation_metres: 0.3,
        section_by_element_m2: [1e-4, 2e-4, 3e-4, 4e-4],
        intersite_section_m2: 1e-4,
        prestress_newtons: 5.0,
    };
    let controls = SourceBodyControls {
        expected_m3_generation: state.generation(),
        body_revision: 1,
        preparation_ref: "controlled:source-performance/preparation".into(),
        state_ref: "controlled:source-performance/resident".into(),
        material: PhysicalMaterial {
            provenance: provenance("controlled:source-frame-material"),
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
    };
    let tuning = match input.tuning {
        TuningPolicy::ExactPitchRatios { ratios, provenance } => {
            TuningConfig::ExactPitchRatios { ratios, provenance }
        }
        TuningPolicy::EqualTemperament12 { provenance } => {
            TuningConfig::EqualTemperament12 { provenance }
        }
    };
    let reduction = ActiveKeyReduction {
        provenance: input.fundamental.provenance().clone(),
        assignments: [0, 2, 4, 5, 7, 9, 11]
            .into_iter()
            .enumerate()
            .map(|(degree, key)| KeyDegreeAssignment {
                key,
                source_degree: degree as u16,
                octave: 0,
            })
            .collect(),
    };
    let config = PerformanceConfig {
        schema: "ql.retained-source-performance-config/v1".into(),
        session_ref: "controlled:source-performance/session".into(),
        receipt_ref: "controlled:source-performance/admission".into(),
        projection_ref: "controlled:retained-source/physical-role".into(),
        relation_context: input.relation_context,
        relation: RelationConfig {
            family: input.relation.family.as_str().into(),
            pair_index: input.relation.pair_index,
            degree: match input.relation.degree {
                ql_core::ConjugationDegree::D1 => 1,
                ql_core::ConjugationDegree::D2 => 2,
                ql_core::ConjugationDegree::D3 => 3,
            },
            expansion_side: input.relation.expansion_side.map(|s| s.as_str().into()),
        },
        source_face: 1,
        physical_face: 1,
        excitation: ExcitationConfig {
            policy_ref: input.excitation.policy_ref,
            standing: input.excitation.standing,
            scaling: match input.excitation.scaling {
                OctetScaling::NoteRelativeToReference => 0,
                OctetScaling::AbsoluteBus => 1,
            },
            reference_hertz: input.excitation.reference_hertz,
            root_linear: input.excitation.root_linear,
            octet_linear: input.excitation.octet_linear,
            weights: input.excitation.weights,
        },
        fundamental_hertz: input.fundamental.hertz(),
        fundamental_provenance: input.fundamental.provenance().clone(),
        use_native_m1_harmonic_ratio: input.fundamental_scaling
            == FundamentalScaling::NativeM1HarmonicRatio,
        tuning,
        recipe,
        controls,
        columns: 6,
        base_register: 0,
        transpose: 0,
        sparse_condition: sparse.then_some(SparseConditionConfig {
            reduction,
            octet: None,
            requirement: SourcePitchRequirement::DeclaredAvailable,
        }),
    };
    (current, config)
}
