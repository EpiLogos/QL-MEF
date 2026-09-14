use crate::nara::{
    BioQuaternion, ConsentState, EarthBodyState, EventBasisRefs, LifecycleState,
    PersonalFieldState, ReceiverState, SourceRevision, WorldContribution,
};

use super::operations::EmbodiedContinuationInput;
use super::{CENTRE_COUNT, ElementalEfwa, EmbodiedField, EvidenceStanding};

fn source(name: &str) -> SourceRevision {
    SourceRevision {
        source_ref: format!("source:{name}"),
        revision: "r1".into(),
        standing_ref: "controlled-k10-acceptance".into(),
    }
}

fn contribution(layer: &str, ordinal: usize, value: f64) -> WorldContribution {
    WorldContribution {
        basis_ref: format!("basis:{layer}:dated-world"),
        source_ref: format!("source:{layer}:centre-{ordinal}"),
        value,
    }
}

fn personal(subject: &str, constitution: &str, resonance_offset: f64) -> PersonalFieldState {
    let receivers = (0..CENTRE_COUNT)
        .map(|ordinal| ReceiverState {
            ordinal: ordinal as u8,
            label: format!("centre-{ordinal}"),
            source: source(&format!("{constitution}:centre-{ordinal}")),
            input_basis: [
                contribution("m1", ordinal, 1.0),
                contribution("m2", ordinal, 2.0),
                contribution("m3", ordinal, 3.0),
            ],
            bioquaternion: BioQuaternion::IDENTITY,
            receiver_orientation: BioQuaternion::IDENTITY,
            composed_orientation: BioQuaternion::IDENTITY,
            orientation_alignment: 1.0,
            drive: ordinal as f64 + resonance_offset,
            resonance: ordinal as f64 + 0.5 + resonance_offset,
            reradiation: ordinal as f64 / 2.0 + resonance_offset,
        })
        .collect();

    PersonalFieldState {
        schema: "ql.nara-personal-field/v1".into(),
        subject_id: subject.into(),
        constitution_ref: constitution.into(),
        reception_generation: 9,
        event: EventBasisRefs {
            event_ref: "event:2026-09-14T02:00:00+01:00".into(),
            subject_ref: subject.into(),
            profile_generation: 3,
            registry_revision: "registry:dated-world".into(),
            m1_revision: "basis:m1:dated-world".into(),
            m2_source_ref: "basis:m2:dated-world".into(),
            m2_contract_ref: "m2:contract".into(),
            m3_source_ref: "basis:m3:dated-world".into(),
            m3_contract_ref: "m3:contract".into(),
        },
        observed_at_unix_ms: 1_789_351_200_000,
        consent: ConsentState::Granted,
        lifecycle: LifecycleState::Active,
        q_identity: BioQuaternion::IDENTITY,
        q_transit: BioQuaternion::IDENTITY,
        q_activity: BioQuaternion::IDENTITY,
        q_composed: BioQuaternion::IDENTITY,
        ephemeral_source: None,
        receivers,
        earth_body: EarthBodyState {
            source: source("earth-body"),
            frame_ref: "earth-fixed".into(),
            orientation: BioQuaternion::IDENTITY,
            relation_alignment: 1.0,
        },
        aggregate_resonance: 0.0,
        aggregate_reradiation: 0.0,
        source_revisions: vec![source(constitution)],
        standing: "controlled K8 output fixture for K10 continuation".into(),
    }
}

fn continue_embodied(personal: &PersonalFieldState) -> EmbodiedField {
    EmbodiedField::from_personal_state(
        personal,
        EmbodiedContinuationInput {
            elemental_efwa: ElementalEfwa {
                earth: 0.25,
                fire: 0.25,
                water: 0.25,
                air: 0.25,
            },
            elemental_source: source("elemental-observation"),
            elemental_standing: EvidenceStanding::Observed,
            nadi_refs: vec!["nadi:ida".into(), "nadi:pingala".into()],
            sushumna_ref: Some("nadi:sushumna".into()),
            temporal_astrology_refs: vec!["transit:dated-world".into()],
            materia_refs: Vec::new(),
            operation_refs: Vec::new(),
            safety_intensity: None,
            contraindication_refs: Vec::new(),
            response_refs: Vec::new(),
            adjustment_refs: Vec::new(),
        },
    )
    .unwrap()
}

#[test]
fn two_controlled_naras_keep_distinct_centre_states_over_the_same_dated_world() {
    let nara_a = personal("nara-a", "constitution:a", 0.0);
    let nara_b = personal("nara-b", "constitution:b", 4.0);

    assert_eq!(nara_a.event.event_ref, nara_b.event.event_ref);
    assert_eq!(
        nara_a.event.registry_revision,
        nara_b.event.registry_revision
    );
    assert_eq!(nara_a.event.m1_revision, nara_b.event.m1_revision);
    assert_eq!(nara_a.event.m2_source_ref, nara_b.event.m2_source_ref);
    assert_eq!(nara_a.event.m3_source_ref, nara_b.event.m3_source_ref);

    let m4_a = continue_embodied(&nara_a);
    let m4_b = continue_embodied(&nara_b);

    assert_eq!(m4_a.centres.len(), CENTRE_COUNT);
    assert_eq!(m4_b.centres.len(), CENTRE_COUNT);
    assert_ne!(m4_a.centres[0].source, m4_b.centres[0].source);
    assert_ne!(m4_a.centres[0].amplitude, m4_b.centres[0].amplitude);

    for (a, b) in m4_a.centres.iter().zip(&m4_b.centres) {
        assert_eq!(a.world_inputs.m1.basis_ref, b.world_inputs.m1.basis_ref);
        assert_eq!(a.world_inputs.m2.basis_ref, b.world_inputs.m2.basis_ref);
        assert_eq!(a.world_inputs.m3.basis_ref, b.world_inputs.m3.basis_ref);
        assert_ne!(a.amplitude, b.amplitude);
    }

    assert_eq!(m4_a.earth_body.frame_ref, "earth-fixed");
    assert_eq!(m4_b.earth_body.frame_ref, "earth-fixed");
    assert!(m4_a.earth_body.amplitude.is_none());
    assert!(m4_b.earth_body.amplitude.is_none());
}
