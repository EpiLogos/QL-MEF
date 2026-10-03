//! Authored preparation of the current native instrument. Source context,
//! source-degree addresses and post-command form are reconstructed by their
//! existing owners. A declared mechanical policy supplies magnitudes only.
use super::{
    ExcitationConfig, PerformanceConfig, RelationConfig, SparseConditionConfig, TuningConfig,
    bounded, face,
};
use crate::continuous::coupled::CoupledBasis;
use crate::m1_engine::M1Engine;
use crate::m2_relation_plan::{
    M2MusicalRouteInput, M2RelationPlan, M2RelationPlanContext, source_field,
};
use crate::m3_state::M3State;
use crate::music_determination::{MusicalDetermination, TuningProvenance, TuningStanding};
use crate::physical_body::{
    BodyFamily, PhysicalMaterial, PhysicalProvenance, PhysicalStanding, SpatialProjection,
};
use crate::source_form_body::{SOURCE_GEOMETRY_BASIS, SourceBodyControls, SourceGeometryRecipe};
use crate::source_key_determination::{
    ActiveKeyReduction, KeyDegreeAssignment, SourcePitchRequirement,
};
use ql_core::QlFace;
use serde::{Deserialize, Serialize};

pub const CURRENT_PREPARATION: &str = "ql.current-source-performance-preparation/v1";

/// This is an explicitly selected reduction, not a claim that a tradition's
/// source degrees intrinsically occupy the architectural CF addresses. Native
/// CF order supplies addresses; the current retained condition supplies pitches.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CurrentSourceChoice {
    RetainedCurrentConditionCfDegreeOrder,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoredMechanicalPolicy {
    pub recipe: SourceGeometryRecipe,
    pub material: PhysicalMaterial,
    pub exciter: SpatialProjection,
    pub pickup: SpatialProjection,
    pub pickup_linear_per_metre: f64,
    pub max_force_newtons: f64,
    pub max_impulse_newton_seconds: f64,
    pub max_displacement_metres: f64,
    pub excitation: ExcitationConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "policy", rename_all = "kebab-case", deny_unknown_fields)]
pub enum CurrentMechanicalPolicy {
    DeclaredInstrumentDefault,
    Explicit {
        mechanical: Box<AuthoredMechanicalPolicy>,
    },
}

/// No observed source, generation, native body, clock, queue or provider receipt
/// can enter through this DTO. The native host's held current basis is separate.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoredCurrentPerformancePreparation {
    pub schema: String,
    pub policy_ref: String,
    pub session_ref: String,
    pub receipt_ref: String,
    pub projection_ref: String,
    pub relation: RelationConfig,
    pub source_face: u8,
    pub physical_face: u8,
    pub mechanical_policy: CurrentMechanicalPolicy,
    pub source_choice: CurrentSourceChoice,
    pub columns: u8,
    pub base_register: i8,
    pub transpose: u8,
}

/// The ordinary first-play action declares this policy. Its source/body content
/// still comes exclusively from the actual current owners at preparation time.
/// The mechanical values are AgentProposed, not measured or authentic pitches.
fn instrument_default(tonic_hertz: f64) -> AuthoredMechanicalPolicy {
    let provenance = |reference: &str| PhysicalProvenance {
        reference: reference.into(),
        revision: "1".into(),
        source_ref: SOURCE_GEOMETRY_BASIS.into(),
        standing: PhysicalStanding::AgentProposed,
    };
    let mut weights = vec![0.0; 12];
    weights[11] = 1.0;
    AuthoredMechanicalPolicy {
        recipe: SourceGeometryRecipe {
            provenance: provenance("policy:expressions/physical-instrument/frame-law"),
            family: BodyFamily::PrestressedTensionNetwork,
            frame_side_metres: 0.1,
            site_separation_metres: 0.3,
            section_by_element_m2: [1e-4, 2e-4, 3e-4, 4e-4],
            intersite_section_m2: 1e-4,
            prestress_newtons: 5.0,
        },
        material: PhysicalMaterial {
            provenance: provenance("policy:expressions/physical-instrument/material"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 1000.0,
            damping_alpha_per_second: 0.4,
            damping_beta_seconds: 0.0,
        },
        exciter: SpatialProjection {
            axis: [0.0, 0.0, 1.0],
            node_weights: weights.clone(),
        },
        pickup: SpatialProjection {
            axis: [0.0, 0.0, 1.0],
            node_weights: weights,
        },
        // Explicit proposed sensitivity. Existing 1000 and calibration 1e6
        // policies/recordings are preserved; this is not an observed calibration.
        pickup_linear_per_metre: 250_000.0,
        max_force_newtons: 10.0,
        max_impulse_newton_seconds: 0.01,
        max_displacement_metres: 0.01,
        excitation: ExcitationConfig {
            policy_ref: "policy:expressions/physical-instrument/root-and-octet".into(),
            standing: "agent-proposed-implementation-policy".into(),
            scaling: 0,
            reference_hertz: tonic_hertz,
            root_linear: 0.5,
            octet_linear: 0.5,
            weights: [0.125; 8],
        },
    }
}

/// Called by the native host on its own current basis. This compiler prepares
/// source data only: it neither authorizes an imported basis nor starts a device.
pub(crate) fn prepare_current_configuration(
    current: &CoupledBasis,
    instance_ref: &str,
    authored: AuthoredCurrentPerformancePreparation,
) -> Result<PerformanceConfig, String> {
    if authored.schema != CURRENT_PREPARATION {
        return Err("current native preparation intent schema differs".into());
    }
    for reference in [
        &authored.policy_ref,
        &authored.session_ref,
        &authored.receipt_ref,
        &authored.projection_ref,
    ] {
        bounded(reference)?;
    }
    bounded(instance_ref)?;
    if !(6..=32).contains(&authored.columns)
        || authored.transpose > 11
        || !(-32..=32).contains(&authored.base_register)
    {
        return Err("declared native keyboard layout is outside its representation".into());
    }
    match authored.source_choice {
        CurrentSourceChoice::RetainedCurrentConditionCfDegreeOrder => {}
    }
    let source_face = face(authored.source_face)?;
    face(authored.physical_face)?;
    let relation = authored.relation.native()?;
    let m1 = M1Engine::new(current.input.m1.clone())?;
    let determination = MusicalDetermination::from_engine(&m1, source_face, relation)?;
    let condition = current
        .m2_input
        .condition
        .as_ref()
        .ok_or("actual current joined M2 condition unavailable; no default maqam or tonic")?;
    let source = source_field();
    let maqam = crate::m2::catalogue()
        .table("maqam")?
        .binding(usize::from(condition.maqam_index))
        .ok_or("actual current maqam has no native source coordinate")?;
    let context = M2RelationPlanContext {
        source_score_or_world_ref: instance_ref.into(),
        situated_provider: None,
        musical: M2MusicalRouteInput {
            maqam_ref: Some(maqam.into()),
            ..Default::default()
        },
        material_routes: vec![],
        control_routes: vec![],
        audio_routes: vec![],
        provenance_refs: vec![
            authored.policy_ref.clone(),
            source.source_path.clone(),
            "ql:declared-native-reduction/current-condition-cf-degree-order/v1".into(),
        ],
        observer_solar_window: None,
        requested_maqam_index: Some(condition.maqam_index),
        requested_role: Some(condition.role),
        candidate_ordinal: 0,
        source_coordinates: (0..6).map(|family| format!("#2-{family}")).collect(),
        material_writes: vec![],
    };
    // Native replay seals every consumed source node/typed edge. A missing path
    // is a refusal, not permission to select another available maqam.
    let prepared = M2RelationPlan::compile(&current.m2_input, context.clone(), source)?;
    prepared.plan.prepare_request(&current.m2_input, source)?;
    let selected = &prepared.plan.execution.condition_input;
    if selected.maqam_index != condition.maqam_index
        || selected.role != condition.role
        || selected.tonic_hz.to_bits() != condition.tonic_hz.to_bits()
    {
        return Err(
            "native current condition selection or tonic changed in source compilation".into(),
        );
    }
    let reduction = ActiveKeyReduction {
        provenance: TuningProvenance {
            policy_ref: authored.policy_ref.clone(),
            source_ref: determination.requested_coordinate().into(),
            revision: format!(
                "{}:CF:{}:M2:{}",
                current.input.m1.revision,
                determination.context_frame().code(),
                current.m2_input.stamp.identity.profile_generation
            ),
            standing: TuningStanding::AgentProposed,
        },
        assignments: determination
            .diatonic_cut()
            .positions
            .iter()
            .enumerate()
            .map(|(degree, pitch)| KeyDegreeAssignment {
                key: 2 * pitch.coordinate.position.value()
                    + match pitch.coordinate.face {
                        QlFace::Direct => 0,
                        QlFace::Conjugate => 1,
                    },
                source_degree: degree as u16,
                octave: 0,
            })
            .collect(),
    };
    let mut m3 = M3State::new(current.input.m3.clone())?;
    for command in &current.input.m3_commands {
        if m3.apply(command.clone())?.status != "applied" {
            return Err("actual current M3 command did not apply".into());
        }
    }
    let mechanical = match authored.mechanical_policy {
        CurrentMechanicalPolicy::DeclaredInstrumentDefault => instrument_default(selected.tonic_hz),
        CurrentMechanicalPolicy::Explicit { mechanical } => *mechanical,
    };
    let preparation_ref = format!("{}/body-preparation", authored.session_ref);
    let state_ref = format!("{}/body-state", authored.session_ref);
    bounded(&preparation_ref)?;
    bounded(&state_ref)?;
    let controls = SourceBodyControls {
        expected_m3_generation: m3.generation(),
        body_revision: 1,
        preparation_ref,
        state_ref,
        material: mechanical.material,
        // The current native performance owner supports this actual format.
        sample_rate: 48000,
        exciter: mechanical.exciter,
        pickup: mechanical.pickup,
        pickup_linear_per_metre: mechanical.pickup_linear_per_metre,
        max_force_newtons: mechanical.max_force_newtons,
        max_impulse_newton_seconds: mechanical.max_impulse_newton_seconds,
        max_displacement_metres: mechanical.max_displacement_metres,
    };
    Ok(PerformanceConfig {
        schema: "ql.retained-source-performance-config/v1".into(),
        session_ref: authored.session_ref,
        receipt_ref: authored.receipt_ref,
        projection_ref: authored.projection_ref,
        relation_context: context,
        relation: authored.relation,
        source_face: authored.source_face,
        physical_face: authored.physical_face,
        excitation: mechanical.excitation,
        fundamental_hertz: selected.tonic_hz,
        fundamental_provenance: TuningProvenance {
            policy_ref: authored.policy_ref.clone(),
            source_ref: maqam.into(),
            revision: current
                .m2_input
                .stamp
                .identity
                .profile_generation
                .to_string(),
            standing: TuningStanding::Reference,
        },
        use_native_m1_harmonic_ratio: false,
        // Existing complete architectural preparation seed is explicit External
        // standing. The operative catalog and Return are replaced privately by
        // the genuine sparse source-key producer; this seed fills no source key.
        tuning: TuningConfig::EqualTemperament12 {
            provenance: TuningProvenance {
                policy_ref: format!("{}/architectural-preparation-seed", authored.policy_ref),
                source_ref: "ql:declared-external/12-tet-preparation-seed".into(),
                revision: "1".into(),
                standing: TuningStanding::External,
            },
        },
        recipe: mechanical.recipe,
        controls,
        columns: authored.columns,
        base_register: authored.base_register,
        transpose: authored.transpose,
        sparse_condition: Some(SparseConditionConfig {
            reduction,
            octet: None,
            requirement: SourcePitchRequirement::DeclaredAvailable,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // Reuse the actual owner's single native test-input producer. Its fixture
    // policy never supplies production context, addresses or default values.
    use super::super::native_source_support;

    fn intent() -> AuthoredCurrentPerformancePreparation {
        AuthoredCurrentPerformancePreparation {
            schema: CURRENT_PREPARATION.into(),
            policy_ref: "policy:expressions/current-condition-cf-degree-order".into(),
            session_ref: "expression:native/current-instrument".into(),
            receipt_ref: "receipt:current-native/preparation".into(),
            projection_ref: "policy:expressions/current-metric-consumer".into(),
            relation: super::super::RelationConfig {
                family: "C".into(),
                pair_index: 1,
                degree: 3,
                expansion_side: None,
            },
            source_face: 1,
            physical_face: 1,
            mechanical_policy: CurrentMechanicalPolicy::DeclaredInstrumentDefault,
            source_choice: CurrentSourceChoice::RetainedCurrentConditionCfDegreeOrder,
            columns: 6,
            base_register: 0,
            transpose: 0,
        }
    }

    #[test]
    fn fresh_native_configuration_keeps_source_form_and_seven_available_degrees() {
        let current = native_source_support::config(false).0;
        let config =
            prepare_current_configuration(&current, "expression:current/source", intent()).unwrap();
        let condition = current.m2_input.condition.as_ref().unwrap();
        assert_eq!(
            config.relation_context.requested_maqam_index,
            Some(condition.maqam_index)
        );
        assert_eq!(config.relation_context.requested_role, Some(condition.role));
        assert_eq!(
            config.fundamental_hertz.to_bits(),
            condition.tonic_hz.to_bits()
        );
        assert!(config.relation_context.situated_provider.is_none());
        assert!(config.relation_context.observer_solar_window.is_none());
        assert_eq!(config.relation_context.source_coordinates.len(), 6);
        let mut state = M3State::new(current.input.m3.clone()).unwrap();
        for command in &current.input.m3_commands {
            state.apply(command.clone()).unwrap();
        }
        assert_eq!(config.controls.expected_m3_generation, state.generation());
        assert_eq!(config.controls.pickup_linear_per_metre, 250_000.0);
        assert_eq!(
            config.recipe.provenance.standing,
            PhysicalStanding::AgentProposed
        );
        let owner =
            super::super::PerformanceOwner::prepare(&current, "expression:current/source", config)
                .unwrap();
        let catalog = owner.catalog().unwrap();
        let distinct = |available| {
            catalog
                .iter()
                .filter(|cell| cell["available"] == available)
                .map(|cell| cell["key"].as_u64().unwrap())
                .collect::<std::collections::BTreeSet<_>>()
        };
        assert_eq!(distinct(true).len(), 7);
        assert_eq!(distinct(false).len(), 5);
        assert_eq!(
            owner
                .binding()
                .physical_body()
                .request()
                .geometry
                .nodes
                .len(),
            12
        );
        assert_eq!(
            owner
                .binding()
                .physical_body()
                .request()
                .geometry
                .edges
                .len(),
            34
        );
        assert_eq!(
            owner.source_assets()["original_native_input"],
            serde_json::to_value(&current.input).unwrap()
        );
        assert_eq!(
            owner.source_assets()["configuration"]["sparse_condition"]["requirement"],
            "declared_available"
        );
        assert!(
            owner.source_assets()["source_key_preparation"]
                .to_string()
                .contains("retained_approximation")
        );
    }

    #[test]
    fn all_native_context_frames_supply_their_actual_coordinate_order() {
        for frame in 1..=7 {
            let mut input = native_source_support::config(false).0.input;
            input.m1.context_frame = frame;
            let current = input.compose().unwrap();
            let chosen = intent();
            let expected = MusicalDetermination::from_engine(
                &M1Engine::new(current.input.m1.clone()).unwrap(),
                face(chosen.source_face).unwrap(),
                chosen.relation.native().unwrap(),
            )
            .unwrap();
            let config =
                prepare_current_configuration(&current, "expression:current/source", chosen)
                    .unwrap();
            let assignments = &config
                .sparse_condition
                .as_ref()
                .unwrap()
                .reduction
                .assignments;
            let keys: std::collections::BTreeSet<_> = assignments.iter().map(|a| a.key).collect();
            assert_eq!(keys.len(), 7);
            for (degree, (assignment, pitch)) in assignments
                .iter()
                .zip(&expected.diatonic_cut().positions)
                .enumerate()
            {
                assert_eq!(assignment.source_degree, degree as u16);
                assert_eq!(assignment.octave, 0);
                assert_eq!(
                    assignment.key,
                    2 * pitch.coordinate.position.value()
                        + if pitch.coordinate.face == QlFace::Direct {
                            0
                        } else {
                            1
                        }
                );
            }
        }
    }

    #[test]
    fn unsupported_authentic_path_and_imported_native_authority_are_not_defaults() {
        let current = native_source_support::config(false).0;
        let mut config =
            prepare_current_configuration(&current, "expression:current/source", intent()).unwrap();
        config.sparse_condition.as_mut().unwrap().requirement =
            SourcePitchRequirement::AuthenticCondition;
        assert!(
            super::super::PerformanceOwner::prepare(&current, "expression:current/source", config)
                .is_err()
        );
        let value = serde_json::to_value(intent()).unwrap();
        for field in [
            "relation_context",
            "native_basis",
            "expected_m3_generation",
            "native_target",
            "current_receiving",
        ] {
            let mut mutated = value.clone();
            mutated[field] = serde_json::json!({});
            assert!(
                serde_json::from_value::<AuthoredCurrentPerformancePreparation>(mutated).is_err()
            );
        }
        let mut missing = current;
        missing.m2_input.condition = None;
        assert!(
            prepare_current_configuration(&missing, "expression:current/source", intent()).is_err()
        );
    }
}
