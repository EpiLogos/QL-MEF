//! Played notes on the installed native worker (QL-MEF #281): a played kernel
//! coordinate routes through the declared inner-four scaffold to the octet
//! slot's own bound mode, and the strike lands on exactly that voice. Requires
//! an installed `ql-field-worker` (QL_FIELD_WORKER or target/k8-cpp).
use std::path::PathBuf;
use std::time::Duration;

use ql_core::{QlCoordinate, QlFace, QlPosition};
use ql_mef::continuous::FieldInput;
use ql_mef::continuous::coupled::{
    CoupledFieldSession, CoupledInput, FrequencyBinding, HarmonicSource, PLAYED_ADDRESS_STANDING,
    REQUEST_V2,
};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_engine::M2Request;
use ql_mef::m3_state::M3Request;
use serde_json::{Value, json};

fn worker() -> PathBuf {
    let path = std::env::var_os("QL_FIELD_WORKER").map_or_else(
        || {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/k8-cpp/bin/ql-field-worker")
        },
        PathBuf::from,
    );
    assert!(path.is_file(), "missing worker {}", path.display());
    path
}

fn coordinate(slot: u8) -> QlCoordinate {
    // The declared scaffold read backwards: slot -> its inner-four coordinate.
    let (position, face) = if slot < 4 {
        (slot + 1, QlFace::Direct)
    } else {
        (slot - 3, QlFace::Conjugate)
    };
    QlCoordinate::new(QlPosition::new(position).unwrap(), face)
}

/// One event, eight voices, every octet slot bound: the full played instrument.
fn input() -> CoupledInput {
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
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_fixture["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    let mut request = CoupledInput {
        schema: REQUEST_V2.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: (0..8)
            .map(|slot| FrequencyBinding {
                mode_ref: format!("controlled:played-mode/{slot}"),
                octet_index: slot,
            })
            .collect(),
        condition_frequency_bindings: vec![],
        sky_frequency_bindings: vec![],
        source_receipts: vec![json!({"standing":"controlled fixture, not a live provider"})],
    };
    let modes: Vec<Value> = (0..8)
        .map(|i| {
            json!({
                "mode_ref":format!("controlled:played-mode/{i}"),
                "source_coordinate":"#2-2-2-5-5", "material_fibre":"earth",
                "carrier_weights":[{"carrier":i,"weight":1.0}],
                "frequency_hz":137.0, "amplitude":[0.002,0.001], "excitation":[0.001,0.0],
                "damping_per_second":0.25, "nodal_state_ref":"controlled:nodes",
                "antinodal_state_ref":"controlled:antinodes"
            })
        })
        .collect();
    request.m2.resonator = Some(
        serde_json::from_value(json!({
            "stamp":request.m2.stamp, "provider_ref":"controlled:played-notes",
            "geometry_ref":"controlled:played-basis", "material_ref":"controlled:linear-medium",
            "material_model_ref":"ql.continuous-linear-mode/v1", "material_parameters":{},
            "modes":modes
        }))
        .unwrap(),
    );
    request
}

fn field(subject: &str) -> FieldInput {
    let audio_gains = vec![1.0; 8];
    let anchor_shapes = vec![[0.0, 0.0, 1.0]; 8];
    let routed_shapes: Vec<[f64; 3]> = (0..8)
        .map(|i| [if i == 3 { 1.0 } else { 0.0 }, 0.0, 0.0])
        .collect();
    serde_json::from_value(json!({"subject_ref":subject,
        "sample_rate":48000, "driver_numerator":720, "driver_denominator":1,
        "clock":{"generation":"4", "inscription":{"turns":"-1","half_degrees":719},
            "lensing":{"turns":"2","half_degrees":1}, "grid_origins":[3,9,21],
            "rate_numerators":["9","8"], "rate_denominator":8, "rate_remainders":["0","0"]},
        "units":{"amplitude":"m","excitation":"m/s","shape":"dimensionless","position":"m","audio":"linear"},
        "audio_gains":audio_gains, "samples":[
            {"identity":1,"constituent":"#3-5-5/0","attachment":0,"rest_metres":[0.0,0.0,0.0],
             "mode_shapes":anchor_shapes},
            {"identity":2,"constituent":"#3-0","attachment":1,"rest_metres":[1.0,0.0,0.0],
             "mode_shapes":routed_shapes}]}))
    .unwrap()
}

fn amplitudes(field: &Value) -> Vec<[f64; 2]> {
    field["amplitudes_metres"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| [a[0].as_f64().unwrap(), a[1].as_f64().unwrap()])
        .collect()
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_played_note_strikes_its_routed_voice_and_nodal_addresses_are_refused() {
    let mut session = CoupledFieldSession::open(
        &worker(),
        input(),
        field("fixture:nara-subject"),
        Duration::from_secs(20),
    )
    .unwrap();
    let seed = session.advance_field(256, false).unwrap();
    // The played address of slot 3 routes to its own mode alone; the seven
    // neighbouring voices continue bit-identically.
    let receipt = session.strike_played(&coordinate(3), [0.05, 0.0]).unwrap();
    let before = amplitudes(&seed);
    let after = amplitudes(&receipt);
    assert_eq!(before.len(), 8);
    assert_ne!(before[3], after[3], "the routed voice did not move");
    for (slot, (was, is)) in before.iter().zip(&after).enumerate() {
        if slot != 3 {
            assert_eq!(was, is, "unbound voice {slot} moved under the played note");
        }
    }
    // The derivation discloses the played addresses with their standing, and
    // the routed slot's sounding pitch is the reading's own octet entry.
    let basis = session.current_basis();
    let played = &basis.derivation["played_addresses"];
    assert_eq!(played["standing"], PLAYED_ADDRESS_STANDING);
    assert_eq!(
        played["addresses"][3]["mode_ref"],
        "controlled:played-mode/3"
    );
    // A nodal outer-two address is refused: the anchors carry no voice.
    let nodal = QlCoordinate::new(QlPosition::new(0).unwrap(), QlFace::Direct);
    let retained = session.last_field().clone();
    let error = session.strike_played(&nodal, [0.05, 0.0]).unwrap_err();
    assert!(error.contains("nodal anchors"), "{error}");
    assert_eq!(session.last_field(), &retained);
    // An unbound slot routes nothing even when the coordinate is inner-four.
    let mut sparse = input();
    sparse.frequency_bindings = vec![FrequencyBinding {
        mode_ref: "controlled:played-mode/3".into(),
        octet_index: 3,
    }];
    let mut other = CoupledFieldSession::open(
        &worker(),
        sparse,
        field("fixture:nara-subject"),
        Duration::from_secs(20),
    )
    .unwrap();
    let error = other
        .strike_played(&coordinate(5), [0.05, 0.0])
        .unwrap_err();
    assert!(error.contains("no bound mode"), "{error}");
}
