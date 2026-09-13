//! K10.1 early-producer acceptance: two differently constituted controlled
//! Naras receive the same dated M1/M2 source occasion through their own stable
//! M3 subject bindings. Reception uses the accepted coupled basis; it does not
//! infer identity or centre mappings from particle/presentation state.
use ql_mef::continuous::coupled::{CoupledInput, HarmonicSource, REQUEST};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_engine::M2Request;
use ql_mef::m3_state::M3Request;
use ql_mef::nara::{
    BioQuaternion, ConsentState, EarthBodyConstitution, EventBasisRefs, LifecycleState,
    PersonalConstitution, PersonalEventInput, PersonalFieldInstance, PersonalLayer,
    ReceiverConstitution, ReceiverEventInput, SourceRevision, WorldContribution,
};
use serde_json::{Value, json};

fn world_input(subject: &str) -> CoupledInput {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(fixture["config"].clone()).unwrap();
    let m2: M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    let m3_fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_fixture["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m3.stamp.identity = m2.stamp.identity.clone();
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    m3.subject_ref = subject.into();
    CoupledInput {
        schema: REQUEST.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: vec![],
        condition_frequency_bindings: vec![],
        source_receipts: vec![json!({
            "standing":"controlled dated source occasion shared by two Nara fixtures",
            "observed_at":"2026-09-13T20:00:00Z"
        })],
    }
}

fn source(name: &str) -> SourceRevision {
    SourceRevision {
        source_ref: format!("source:{name}"),
        revision: "controlled-revision-1".into(),
        standing_ref: "controlled-k10-acceptance".into(),
    }
}

fn layer(name: &str, quaternion: BioQuaternion) -> PersonalLayer {
    PersonalLayer {
        source: source(name),
        observed_at_unix_ms: 1_789_329_600_000,
        quaternion,
    }
}

fn constitution(subject: &str, scale: f64) -> PersonalConstitution {
    PersonalConstitution {
        subject_id: subject.into(),
        constitution_ref: format!("constitution:{subject}:controlled-v1"),
        source_revisions: vec![source("controlled-personal-basis")],
        consent: ConsentState::Granted,
        lifecycle: LifecycleState::Active,
        identity: layer(
            &format!("{subject}:identity"),
            BioQuaternion {
                w: 1.0,
                x: 0.03 * scale,
                y: 0.01,
                z: 0.0,
            },
        ),
        transit: layer(
            &format!("{subject}:transit"),
            BioQuaternion {
                w: 1.0,
                x: 0.0,
                y: 0.05,
                z: 0.02 * scale,
            },
        ),
        activity: layer(
            &format!("{subject}:activity"),
            BioQuaternion {
                w: 1.0,
                x: 0.02,
                y: 0.0,
                z: 0.04 * scale,
            },
        ),
        ephemeral: Some(layer(
            &format!("{subject}:ephemeral"),
            BioQuaternion::IDENTITY,
        )),
        earth_body: EarthBodyConstitution {
            source: source(&format!("{subject}:earth-body")),
            frame_ref: "earth-fixed:controlled-k10".into(),
            orientation: BioQuaternion::IDENTITY,
        },
        receivers: (0..7)
            .map(|ordinal| ReceiverConstitution {
                ordinal,
                label: format!("controlled-centre-{ordinal}"),
                source: source(&format!("{subject}:centre-{ordinal}")),
                world_weights: [
                    scale * (1.0 + f64::from(ordinal) / 10.0),
                    0.5 + f64::from(ordinal) / 20.0,
                    0.25,
                ],
                orientation: BioQuaternion {
                    w: 1.0,
                    x: f64::from(ordinal) / 20.0,
                    y: 0.01 * scale,
                    z: 0.0,
                },
                resonance_gain: 0.8 + f64::from(ordinal) / 20.0,
                reradiation_gain: 0.35 + scale / 10.0,
            })
            .collect(),
    }
}

fn event(refs: &EventBasisRefs) -> PersonalEventInput {
    PersonalEventInput {
        event_ref: refs.event_ref.clone(),
        profile_generation: refs.profile_generation,
        observed_at_unix_ms: 1_789_329_600_000,
        receivers: (0..7)
            .map(|ordinal| ReceiverEventInput {
                ordinal,
                m1: WorldContribution {
                    basis_ref: refs.m1_revision.clone(),
                    source_ref: format!("observed:m1:{ordinal}"),
                    value: 0.7 + f64::from(ordinal) / 10.0,
                },
                m2: WorldContribution {
                    basis_ref: refs.m2_source_ref.clone(),
                    source_ref: format!("observed:m2:{ordinal}"),
                    value: 1.2 + f64::from(ordinal) / 20.0,
                },
                m3: WorldContribution {
                    basis_ref: refs.m3_source_ref.clone(),
                    source_ref: format!("observed:m3:{ordinal}"),
                    value: 0.4 + f64::from(ordinal) / 30.0,
                },
            })
            .collect(),
    }
}

#[test]
fn two_distinct_naras_receive_the_same_dated_sources_without_collapsing_constitution() {
    let world_a = world_input("subject:controlled-nara-a");
    let world_b = world_input("subject:controlled-nara-b");
    assert_eq!(
        serde_json::to_value(&world_a.m1).unwrap(),
        serde_json::to_value(&world_b.m1).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&world_a.m2).unwrap(),
        serde_json::to_value(&world_b.m2).unwrap()
    );
    assert_eq!(world_a.source_receipts, world_b.source_receipts);

    let basis_a = world_a.compose().unwrap();
    let basis_b = world_b.compose().unwrap();
    let refs_a = EventBasisRefs::from_basis(&basis_a).unwrap();
    let refs_b = EventBasisRefs::from_basis(&basis_b).unwrap();
    assert_eq!(refs_a.event_ref, refs_b.event_ref);
    assert_eq!(refs_a.profile_generation, refs_b.profile_generation);
    assert_eq!(refs_a.m1_revision, refs_b.m1_revision);
    assert_eq!(refs_a.m2_source_ref, refs_b.m2_source_ref);
    assert_eq!(refs_a.m3_source_ref, refs_b.m3_source_ref);
    assert_ne!(refs_a.subject_ref, refs_b.subject_ref);

    let input_a = event(&refs_a);
    let input_b = event(&refs_b);
    assert_eq!(
        input_a, input_b,
        "the received dated world contributions differ"
    );

    let mut nara_a = PersonalFieldInstance::new(constitution(&refs_a.subject_ref, 0.8)).unwrap();
    let mut nara_b = PersonalFieldInstance::new(constitution(&refs_b.subject_ref, 1.6)).unwrap();
    let state_a = nara_a.receive(&basis_a, input_a.clone()).unwrap();
    let state_b = nara_b.receive(&basis_b, input_b.clone()).unwrap();

    assert_eq!(state_a.receivers.len(), 7);
    assert_eq!(state_b.receivers.len(), 7);
    assert_eq!(state_a.event.event_ref, state_b.event.event_ref);
    assert_eq!(state_a.observed_at_unix_ms, state_b.observed_at_unix_ms);
    assert_ne!(state_a.subject_id, state_b.subject_id);
    assert_ne!(state_a.constitution_ref, state_b.constitution_ref);
    assert_ne!(state_a.q_composed, state_b.q_composed);
    assert_ne!(state_a.receivers[0].drive, state_b.receivers[0].drive);
    assert_ne!(state_a.aggregate_resonance, state_b.aggregate_resonance);
    assert_ne!(state_a.aggregate_reradiation, state_b.aggregate_reradiation);

    let replay_a = nara_a.receive(&basis_a, input_a.clone()).unwrap();
    assert_eq!(replay_a, state_a);
    assert_eq!(nara_a.current().unwrap().reception_generation, 1);

    let mut conflict = input_a;
    conflict.receivers[0].m1.value += 0.01;
    assert!(
        nara_a
            .receive(&basis_a, conflict)
            .unwrap_err()
            .contains("conflicting replay")
    );
    assert!(
        nara_a
            .receive(&basis_b, input_b)
            .unwrap_err()
            .contains("subject does not match")
    );
}
