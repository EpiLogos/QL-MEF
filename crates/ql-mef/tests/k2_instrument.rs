//! K² instrument causality over the real installed worker: one native M1
//! determinant is varied while subject, event, clock and material are held.
//! Run with `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test
//! k2_instrument -- --ignored`.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::k2::{self, BindingRequest, K2Config, K2Instrument};
use serde_json::{Value, json};

fn worker() -> PathBuf {
    PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker"))
}

fn config() -> K2Config {
    let request: BindingRequest = serde_json::from_value(json!({
        "schema": k2::BINDING_REQUEST, "instance_ref": "test:k2", "texture": [64, 64],
        "units_per_metre": 1.0,
        "geometry": {"longitude_samples": 32, "latitude_samples": 16, "metres_per_unit": 1.0, "attachment": 1}
    }))
    .unwrap();
    let binding = k2::binding(request).unwrap();
    assert_eq!(
        binding["presentation"]["slots_a"].as_array().unwrap().len(),
        4096
    );
    serde_json::from_value(binding["host"].clone()).unwrap()
}

fn open() -> K2Instrument {
    K2Instrument::open(&worker(), config(), Duration::from_secs(20)).unwrap()
}

fn audio(field: &Value) -> Vec<f64> {
    field["audio"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

fn targets(field: &Value) -> Vec<Vec<f64>> {
    field["targets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| {
            t["position"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect()
        })
        .collect()
}

/// Power at `hz` over the block (a single DFT bin).
fn power(samples: &[f64], hz: f64) -> f64 {
    let (mut re, mut im) = (0.0, 0.0);
    for (i, x) in samples.iter().enumerate() {
        let w = 2.0 * std::f64::consts::PI * hz * i as f64 / 48_000.0;
        re += x * w.cos();
        im -= x * w.sin();
    }
    re * re + im * im
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn one_m1_tick_retunes_reshapes_and_moves_the_body() {
    let (mut control, mut varied) = (open(), open());
    let a = control.session_mut().advance_field(4096, false).unwrap();
    let b = varied.session_mut().advance_field(4096, false).unwrap();
    // Held conditions: two owners over one event are indistinguishable.
    assert_eq!(audio(&a), audio(&b));
    assert_eq!(targets(&a), targets(&b));

    let before = varied.shape().clone();
    let revision = varied.influence()["m1_revision"].clone();
    varied.m1_advance(1).unwrap();
    let after = varied.shape().clone();
    let influence = varied.influence();
    assert_ne!(influence["m1_revision"], revision);
    assert_eq!(after.address72, (before.address72 + 1) % 72);
    assert_ne!(after.shape_ref, before.shape_ref);
    for (old, new) in before.voices.iter().zip(after.voices.iter()) {
        assert_ne!(
            old.frequency_hz, new.frequency_hz,
            "every voice retunes on a tick"
        );
    }

    control.session_mut().advance_field(4096, false).unwrap();
    // The struck control continues; re-strike it identically for a fair pitch test.
    let event = control.event();
    control.replace(&event, true).unwrap();
    let c = control.session_mut().advance_field(8192, false).unwrap();
    let v = varied.session_mut().advance_field(8192, false).unwrap();
    let (c_audio, v_audio) = (audio(&c), audio(&v));
    // Each owner's energy sits at its own voices, not the other's.
    let top_old = before.voices[7].frequency_hz;
    let top_new = after.voices[7].frequency_hz;
    assert!(power(&c_audio, top_old) > 10.0 * power(&c_audio, top_new));
    assert!(power(&v_audio, top_new) > 10.0 * power(&v_audio, top_old));
    // The body moved because the shapes and voices changed, not because time did.
    let moved = targets(&c)
        .iter()
        .zip(targets(&v).iter())
        .map(|(x, y)| {
            x.iter()
                .zip(y)
                .map(|(p, q)| (p - q).powi(2))
                .sum::<f64>()
                .sqrt()
        })
        .fold(0.0, f64::max);
    assert!(
        moved > 1e-3,
        "targets did not follow the determinant (max {moved})"
    );
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn replacement_without_strike_continues_and_a_foreign_resonator_is_refused() {
    let mut instrument = open();
    instrument.session_mut().advance_field(2048, false).unwrap();
    let generation = instrument.session().last_field()["generation"].clone();
    let event = instrument.event();
    instrument.replace(&event, false).unwrap();
    // Same event re-issued: a new M2 generation, identical voices and shapes.
    assert_ne!(instrument.session().last_field()["generation"], generation);
    let mut foreign = config();
    foreign.basis.m2.resonator = serde_json::from_value(json!(null)).unwrap();
    foreign.basis.frequency_bindings = vec![ql_mef::continuous::coupled::FrequencyBinding {
        mode_ref: "x".into(),
        octet_index: 0,
    }];
    assert!(foreign.validate().is_err());
}
