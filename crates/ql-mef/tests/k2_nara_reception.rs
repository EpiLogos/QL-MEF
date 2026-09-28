//! Two differently constituted Naras receive one K² event (QL-MEF #135/#201).
//! Constitutions and centre inputs are controlled test values: the rule that
//! derives them from identity evidence belongs to the M4 identity owner.
//! Run with `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test
//! k2_nara_reception -- --ignored`.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::k2::{self, BindingRequest, K2Config, K2Instrument};
use ql_mef::nara::{
    BioQuaternion, ConsentState, EarthBodyConstitution, EventBasisRefs, LifecycleState,
    PersonalConstitution, PersonalEventInput, PersonalLayer, ReceiverConstitution,
    ReceiverEventInput, SourceRevision, WorldContribution,
};
use serde_json::json;

fn source(name: &str) -> SourceRevision {
    SourceRevision {
        source_ref: format!("source:{name}"),
        revision: "controlled-revision-1".into(),
        standing_ref: "controlled-k2-reception".into(),
    }
}

fn layer(name: &str, q: [f64; 4]) -> PersonalLayer {
    PersonalLayer {
        source: source(name),
        observed_at_unix_ms: 1_789_329_600_000,
        quaternion: BioQuaternion {
            w: q[0],
            x: q[1],
            y: q[2],
            z: q[3],
        },
    }
}

fn constitution(subject: &str, scale: f64) -> PersonalConstitution {
    PersonalConstitution {
        subject_id: subject.into(),
        constitution_ref: format!("constitution:{subject}:controlled-v1"),
        source_revisions: vec![source("controlled-personal-basis")],
        consent: ConsentState::Granted,
        lifecycle: LifecycleState::Active,
        identity: layer("identity", [1.0, 0.03 * scale, 0.01, 0.0]),
        transit: layer("transit", [1.0, 0.0, 0.05, 0.02 * scale]),
        activity: layer("activity", [1.0, 0.02, 0.0, 0.04 * scale]),
        ephemeral: None,
        earth_body: EarthBodyConstitution {
            source: source("earth-body"),
            frame_ref: "earth-fixed:controlled".into(),
            orientation: BioQuaternion::IDENTITY,
        },
        receivers: (0..7)
            .map(|ordinal| ReceiverConstitution {
                ordinal,
                label: format!("controlled-centre-{ordinal}"),
                source: source(&format!("centre-{ordinal}")),
                world_weights: [scale * (1.0 + f64::from(ordinal) / 10.0), 0.5, 0.25],
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

fn open(subject: &str, scale: f64) -> K2Instrument {
    let request: BindingRequest = serde_json::from_value(json!({
        "schema": k2::BINDING_REQUEST, "instance_ref": format!("test:{subject}"), "texture": [32, 32],
        "units_per_metre": 1.0,
        "geometry": {"longitude_samples": 16, "latitude_samples": 8, "metres_per_unit": 1.0, "attachment": 1}
    }))
    .unwrap();
    let mut config: K2Config =
        serde_json::from_value(k2::binding(request).unwrap()["host"].clone()).unwrap();
    config.basis.m3.subject_ref = subject.into();
    config.field.subject_ref = subject.into();
    config.reception = Some(constitution(subject, scale));
    let worker = PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER"));
    K2Instrument::open(&worker, config, Duration::from_secs(20)).unwrap()
}

/// Identical world contributions for both subjects, citing each owner's basis.
fn input(refs: &EventBasisRefs) -> PersonalEventInput {
    PersonalEventInput {
        event_ref: refs.event_ref.clone(),
        profile_generation: refs.profile_generation,
        observed_at_unix_ms: 1_789_329_600_000,
        receivers: (0..7)
            .map(|ordinal| {
                let c = |basis: &str, value: f64| WorldContribution {
                    basis_ref: basis.into(),
                    source_ref: format!("controlled:{ordinal}"),
                    value,
                };
                ReceiverEventInput {
                    ordinal,
                    m1: c(&refs.m1_revision, 0.7 + f64::from(ordinal) / 10.0),
                    m2: c(&refs.m2_source_ref, 1.2),
                    m3: c(&refs.m3_source_ref, 0.4),
                }
            })
            .collect(),
    }
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn two_constituted_naras_receive_one_event_differently_and_independently() {
    let (mut a, mut b) = (open("subject:nara-a", 1.0), open("subject:nara-b", 1.7));
    let (ra, rb) = (a.event_basis().unwrap(), b.event_basis().unwrap());
    // Same world: every source and generation but the receiving subject.
    assert_eq!(ra.event_ref, rb.event_ref);
    assert_eq!(ra.m1_revision, rb.m1_revision);
    assert_eq!(ra.m2_source_ref, rb.m2_source_ref);
    assert_eq!(a.shape(), b.shape(), "one event, one sounding body");

    let sa = a.receive_personal(input(&ra)).unwrap();
    let sb = b.receive_personal(input(&rb)).unwrap();
    assert_eq!(sa.receivers.len(), 7);
    assert_eq!(sb.receivers.len(), 7);
    for (x, y) in sa.receivers.iter().zip(&sb.receivers) {
        assert!(x.resonance.is_finite() && y.resonance.is_finite());
        assert!(
            (x.resonance - y.resonance).abs() > 1e-6,
            "centre {} did not differ between constitutions",
            x.ordinal
        );
    }
    // Independent centres: resonance is not one scalar copied seven times.
    let spread = |s: &ql_mef::nara::PersonalFieldState| {
        let r: Vec<f64> = s.receivers.iter().map(|c| c.resonance).collect();
        r.iter().cloned().fold(f64::MIN, f64::max) - r.iter().cloned().fold(f64::MAX, f64::min)
    };
    assert!(spread(&sa) > 1e-3 && spread(&sb) > 1e-3);
    assert_eq!(a.personal_reading().unwrap()["current"], true);

    // A determinant event makes the reception stale; the old basis is refused.
    a.m1_advance(1).unwrap();
    assert_eq!(a.personal_reading().unwrap()["current"], false);
    assert!(a.receive_personal(input(&ra)).is_err());
    let fresh = a.event_basis().unwrap();
    a.receive_personal(input(&fresh)).unwrap();
    assert_eq!(a.personal_reading().unwrap()["current"], true);
    // B was untouched by A's event.
    assert_eq!(b.personal_reading().unwrap()["current"], true);
}
