//! The Torus for a native audio host, over the real installed worker (O-I #849): the host's
//! `torus-native` hands over the continuation exactly as it stands (states, cursor, clock, driver,
//! samples, the Carrier's drive) and the played-address law that strikes it, and nothing moves; the
//! law's impulses are the owner's own `torus-play`; it names the voices of the tuning in force by
//! their native mode; it is refused while a setting waits for a later sample. Run with
//! `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test torus_native -- --ignored`. The wire
//! contract is exercised without a worker.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::host::{FieldHost, HostOperation, HostRequest};
use ql_mef::continuous::scene_field::{BINDING_REQUEST, BindingRequest, SceneConfig};
use serde_json::{Value, json};

fn worker() -> PathBuf {
    PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker"))
}

fn open() -> FieldHost {
    let binding = ql_mef::continuous::scene_field::binding(BindingRequest {
        schema: BINDING_REQUEST.into(),
        instance_ref: "test:torus-native".into(),
        texture: [32, 16],
        units_per_metre: 1.0,
        event: None,
        sky: None,
        field: None,
        geometry: Some(ql_mef::continuous::scene_field::SceneGeometry {
            longitude_samples: 32,
            latitude_samples: 16,
            metres_per_unit: 1.0,
            attachment: 1,
        }),
        material: None,
        reception: None,
        torus: None,
    })
    .unwrap();
    let config: SceneConfig = serde_json::from_value(binding["host"].clone()).unwrap();
    FieldHost::open_scene(&worker(), config, Duration::from_secs(20)).unwrap()
}

struct Driver {
    host: FieldHost,
    current: Value,
}

impl Driver {
    fn open() -> Self {
        let host = open();
        let current = host.ready();
        assert_eq!(current["status"], "ready");
        Self { host, current }
    }
    fn send(&mut self, command: Value) -> Value {
        let c = &self.current;
        let request = HostRequest {
            schema: "ql.field-host-request/v1".into(),
            instance_ref: c["instance_ref"].as_str().unwrap().into(),
            event_ref: c["field"]["event_ref"].as_str().unwrap().into(),
            subject_ref: c["field"]["subject_ref"].as_str().unwrap().into(),
            request_id: (c["last_request_id"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap()
                + 1)
            .to_string(),
            expected_generation: c["field"]["generation"].as_str().unwrap().into(),
            expected_samples_elapsed: c["field"]["samples_elapsed"].as_str().unwrap().into(),
            command: serde_json::from_value(command).unwrap(),
        };
        self.current = self.host.execute(request);
        self.current.clone()
    }
    fn ok(&mut self, command: Value) -> Value {
        let reply = self.send(command.clone());
        assert_eq!(reply["status"], "ok", "{command}: {reply}");
        reply
    }
}

fn float(value: &Value) -> f64 {
    value.as_f64().expect("a number")
}

/// The owner's shape term (scene_field.rs `chi`), computed here from the handed-over law alone.
fn chi(voice: &Value, phi: f64, theta: f64) -> f64 {
    let phi = phi - float(&voice["longitude_radians"]);
    let (m, n) = (float(&voice["m"]), float(&voice["n"]));
    let a = float(&voice["weight"]) * float(&voice["phase_radians"]).cos();
    let b = float(&voice["weight"]) * float(&voice["phase_radians"]).sin();
    a * (m * phi / 2.0).sin() * (n * theta / 2.0).sin()
        + b * (m * phi / 2.0).cos() * (n * theta / 2.0).cos()
}

/// What the law strikes for `keys` at `samples` (janko.rs POINT_LAW over the law's arcs and pulse).
fn impulses(law: &Value, keys: &[(u64, u64)], amplitude: [f64; 2], samples: u64) -> Vec<[f64; 2]> {
    let pulse = &law["pulse"];
    let anchor: u64 = pulse["anchor_samples"].as_str().unwrap().parse().unwrap();
    let rate = float(&law["sample_rate"]);
    let steps = float(&pulse["anchor_steps"])
        + samples.saturating_sub(anchor) as f64 / rate * float(&pulse["steps_per_second"]);
    let strike = float(&law["strike"]);
    let voices = law["voices"].as_array().unwrap();
    let mut out = vec![[0.0, 0.0]; voices.len()];
    for (row, column) in keys {
        let arc = law["arcs"][(row * 6 + column % 6) as usize]
            .as_u64()
            .unwrap() as f64;
        let mut arcs = (arc + steps).rem_euclid(12.0);
        if arcs >= 12.0 {
            arcs = 0.0;
        }
        let mut sheet = arcs.rem_euclid(6.0);
        if sheet >= 6.0 {
            sheet = 0.0;
        }
        let (theta, phi) = (
            arcs * (std::f64::consts::PI / 6.0),
            sheet * (std::f64::consts::PI / (6.0 / 2.0)),
        );
        for (impulse, voice) in out.iter_mut().zip(voices) {
            let shape = chi(voice, phi, theta);
            impulse[0] += amplitude[0] * strike * shape;
            impulse[1] += amplitude[1] * strike * shape;
        }
    }
    out
}

fn cursor(reply: &Value) -> u64 {
    reply["field"]["samples_elapsed"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap()
}

#[test]
fn torus_native_is_admitted_by_name_and_strict_about_its_fields() {
    assert!(serde_json::from_value::<HostOperation>(json!({"operation":"torus-native"})).is_ok());
    for command in [
        json!({"operation":"torus-native","at":"0"}),
        json!({"operation":"torus-native","keys":[]}),
    ] {
        assert!(
            serde_json::from_value::<HostOperation>(command.clone()).is_err(),
            "{command}"
        );
    }
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn the_torus_is_handed_over_as_it_stands_and_nothing_moves() {
    let mut d = Driver::open();
    // The Carrier drives the body at the pulse's steps: its drive is the owner's, and travels with the body.
    let carried = d.ok(json!({"operation":"torus-set","edit":{"carrier":0.25}}));
    assert_eq!(carried["torus"]["carrier"]["native"], true, "{carried}");
    d.ok(json!({"operation":"advance","frames":4800,"muted":false}));
    d.ok(json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.3,0.0]}));
    // A key for a later sample waits in the native owner: the continuation carries it.
    let later = cursor(&d.current) + 700;
    d.ok(
        json!({"operation":"torus-play","keys":[{"row":2,"column":4}],"amplitude":[0.2,0.0],
        "at_samples_elapsed":later.to_string()}),
    );
    let before = d.ok(json!({"operation":"advance","frames":333,"muted":false}));
    let reply = d.ok(json!({"operation":"torus-native"}));
    let native = &reply["torus_native"];
    assert_eq!(native["schema"], "ql.torus-native/v1", "{native}");
    let handed = &native["continuation"];
    assert_eq!(handed["schema"], "ql.continuous-handover/v1");
    // Nothing moves: the same generation, cursor, body and clock.
    for key in [
        "generation",
        "samples_elapsed",
        "amplitudes_metres",
        "clock",
        "targets",
    ] {
        assert_eq!(reply["field"][key], before["field"][key], "{key}");
    }
    assert_eq!(handed["generation"], before["field"]["generation"]);
    assert_eq!(
        handed["samples_elapsed"],
        before["field"]["samples_elapsed"]
    );
    assert_eq!(
        native["at_samples_elapsed"],
        before["field"]["samples_elapsed"]
    );
    for key in [
        "event_ref",
        "subject_ref",
        "geometry_ref",
        "material_ref",
        "model_ref",
        "sample_rate",
    ] {
        assert_eq!(handed[key], before["field"][key], "{key}");
    }
    // The states are the body's, exactly; every sample is a target, in order.
    let states: Vec<&Value> = handed["modes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| &m["state_metres"])
        .collect();
    let body: Vec<&Value> = before["field"]["amplitudes_metres"]
        .as_array()
        .unwrap()
        .iter()
        .collect();
    assert_eq!(states, body);
    let samples = handed["samples"].as_array().unwrap();
    let targets = before["field"]["targets"].as_array().unwrap();
    assert_eq!(samples.len(), targets.len());
    for (sample, target) in samples.iter().zip(targets) {
        assert_eq!(sample["identity"], target["identity"]);
        assert_eq!(
            sample["mode_shapes"].as_array().unwrap().len(),
            states.len()
        );
    }
    // The act for a later sample, the driver's remainder and the Carrier's drive travel with it.
    let pending = handed["pending"].as_array().unwrap();
    assert_eq!(pending.len(), 1, "{handed}");
    assert_eq!(pending[0]["at_samples_elapsed"], later.to_string());
    // The driver's remainder is what the rendered frames left of the clock's whole steps: the cursor times
    // the driver's numerator, modulo the rate times its denominator (continuous_field.hpp `render_audio`).
    let remainder: u64 = handed["driver_remainder"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let (numerator, denominator, rate) = (
        handed["driver_numerator"].as_u64().unwrap(),
        handed["driver_denominator"].as_u64().unwrap(),
        handed["sample_rate"].as_u64().unwrap(),
    );
    let cursor_now: u64 = handed["samples_elapsed"].as_str().unwrap().parse().unwrap();
    assert_eq!(
        remainder,
        cursor_now * numerator % (rate * denominator),
        "{handed}"
    );
    assert_ne!(
        remainder, 0,
        "the cursor leaves a remainder here, so a lost one is seen"
    );
    assert_eq!(
        handed["carrier"]["impulses"].as_array().unwrap().len(),
        states.len()
    );
    // The field continues from the handover as it would have.
    let after = d.ok(json!({"operation":"advance","frames":512,"muted":false}));
    assert_eq!(cursor(&after), cursor(&before) + 512);
    // Release and silence are stated: the body rings on; silence stills what was struck.
    assert!(
        native["release"]["law"]
            .as_str()
            .unwrap()
            .contains("rings on")
    );
    assert!(
        native["silence"]["law"]
            .as_str()
            .unwrap()
            .contains("stills")
    );
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn the_handed_law_strikes_what_the_owners_torus_play_strikes() {
    let mut d = Driver::open();
    d.ok(json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":3.5},"strike":0.6}}));
    d.ok(json!({"operation":"advance","frames":2401,"muted":false}));
    let reply = d.ok(json!({"operation":"torus-native"}));
    let native = &reply["torus_native"];
    let law = &native["keys"];
    assert_eq!(law["contract"], "ql.torus-keys/v1");
    assert_eq!(law["arcs"].as_array().unwrap().len(), 36);
    assert_eq!(law["strike"], 0.6);
    assert_eq!(law["tuning"], reply["torus"]["settings"]["tuning"]);
    // Every voice of the tuning in force, by the native mode it sounds through.
    let modes = native["continuation"]["modes"].as_array().unwrap();
    let voices = law["voices"].as_array().unwrap();
    assert_eq!(
        voices.len() as u64,
        reply["torus"]["sounding_voices"].as_u64().unwrap()
    );
    for voice in voices {
        let mode = voice["mode"].as_u64().unwrap() as usize;
        assert_eq!(modes[mode]["mode_ref"], voice["mode_ref"]);
    }
    // The owner strikes the same keys now: its impulses are the law's, to the bit.
    let keys = [(0, 2), (3, 7), (5, 11)];
    let amplitude = [0.35, -0.1];
    let expected = impulses(law, &keys, amplitude, cursor(&reply));
    let played = d.ok(json!({"operation":"torus-play","keys":keys.iter().map(|(r, c)| json!({"row":r,"column":c})).collect::<Vec<_>>(),
        "amplitude":amplitude}));
    let struck: Vec<[f64; 2]> = played["strike_at"]["voices"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| [float(&v["amplitude"][0]), float(&v["amplitude"][1])])
        .collect();
    assert_eq!(struck, expected);
    // And at a later sample the pulse has carried the keys on: the law's steps are the owner's.
    d.ok(json!({"operation":"advance","frames":1777,"muted":false}));
    let expected = impulses(law, &keys, amplitude, cursor(&d.current));
    let played = d.ok(json!({"operation":"torus-play","keys":keys.iter().map(|(r, c)| json!({"row":r,"column":c})).collect::<Vec<_>>(),
        "amplitude":amplitude}));
    let struck: Vec<[f64; 2]> = played["strike_at"]["voices"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| [float(&v["amplitude"][0]), float(&v["amplitude"][1])])
        .collect();
    assert_eq!(struck, expected);
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_tuning_change_hands_over_its_own_voices_and_a_waiting_setting_is_refused() {
    let mut d = Driver::open();
    let logic = d.ok(json!({"operation":"torus-native"}))["torus_native"]["keys"]["voices"].clone();
    d.ok(json!({"operation":"torus-set","edit":{"tuning":"sky"}}));
    let sky = d.ok(json!({"operation":"torus-native"}));
    let voices = &sky["torus_native"]["keys"]["voices"];
    assert_ne!(voices, &logic);
    assert_eq!(sky["torus_native"]["keys"]["tuning"], "sky");
    assert_eq!(
        voices.as_array().unwrap().len() as u64,
        sky["torus"]["sounding_voices"].as_u64().unwrap()
    );
    // A setting for a later sample changes the law there: the Torus is handed over once it has landed.
    let at = cursor(&d.current) + 4800;
    d.ok(
        json!({"operation":"torus-set","edit":{"strike":0.25},"at_samples_elapsed":at.to_string()}),
    );
    let refused = d.send(json!({"operation":"torus-native"}));
    assert_eq!(refused["status"], "refused", "{refused}");
    assert!(
        refused["error"].as_str().unwrap().contains("waits"),
        "{refused}"
    );
    d.ok(json!({"operation":"advance","frames":4800,"muted":false}));
    let landed = d.ok(json!({"operation":"torus-native"}));
    assert_eq!(landed["torus_native"]["keys"]["strike"], 0.25);
}
