//! The Torus's M1′ session through the field host, over the real installed
//! worker (O-I #850): every setting reaches the owner and reads back; the band
//! stands beside the rate; the comma starts from nothing and grows only as the
//! field runs; a key strikes its moving point and always the same helix place;
//! Sync follows a tempo change; a tuning switch changes the reported voices.
//! Run with `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test
//! torus_host -- --ignored`. The wire contract is exercised without a worker.
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
        instance_ref: "test:torus".into(),
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
    fn generation(&self) -> u64 {
        self.current["field"]["generation"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap()
    }
}

#[test]
fn the_torus_operations_are_admitted_by_name_and_strict_about_their_fields() {
    for command in [
        json!({"operation":"torus-state"}),
        json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":440.0}}}),
        json!({"operation":"torus-set","edit":{"rate":{"mode":"sync","note":{"numerator":1,"denominator":4}},"tempo_bpm":90.0}}),
        json!({"operation":"torus-set","edit":{"tuning":"maqam","polyphony":4,"carrier":0.5,"shape":"figure-eight","fundamental":{"source":"hz","hz":261.6}}}),
        json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.1,0.0]}),
        json!({"operation":"torus-play","keys":[{"row":0,"column":2},{"row":3,"column":1}],"amplitude":[0.1,0.0],"at_samples_elapsed":"4800"}),
        json!({"operation":"torus-release","keys":[{"row":0,"column":2}]}),
    ] {
        assert!(
            serde_json::from_value::<HostOperation>(command.clone()).is_ok(),
            "{command}"
        );
    }
    for command in [
        json!({"operation":"torus-set","edit":{"drive":1.0}}),
        json!({"operation":"torus-set","edit":{"tuning":"equal"}}),
        json!({"operation":"torus-play","keys":[{"row":0}],"amplitude":[0.1,0.0]}),
        json!({"operation":"torus-state","x":1}),
    ] {
        assert!(
            serde_json::from_value::<HostOperation>(command.clone()).is_err(),
            "{command}"
        );
    }
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn each_setting_reaches_the_owner_and_reads_back() {
    let mut d = Driver::open();
    let state = d.send(json!({"operation":"torus-state"}));
    assert_eq!(state["status"], "ok", "{state}");
    let torus = &state["torus"];
    assert_eq!(torus["schema"], "ql.m1-prime-session/v1");
    assert_eq!(torus["band"], "rhythm");
    assert_eq!(torus["comma_cents"], json!(0.0));
    // A new Torus opens under Logic, the manual's default (§21.2 Parameters).
    assert_eq!(torus["settings"]["tuning"], "logic");
    // Where each of the twelve classes stands and what it sounds under the tuning in force (O-I #851).
    let classes = &torus["classes"];
    assert_eq!(classes["tuning"], "logic", "{classes}");
    assert_eq!(classes["by_class"].as_array().unwrap().len(), 12);
    assert_eq!(
        classes["by_class"][0]["sounds"].as_array().unwrap().len(),
        8
    );
    let generation = d.generation();
    let set = d.send(json!({"operation":"torus-set","edit":{
        "rate":{"mode":"free","steps_per_second":440.0},"carrier":0.25,"shape":"spiral","polyphony":3}}));
    assert_eq!(set["status"], "ok", "{set}");
    // Session settings move no native state.
    assert_eq!(d.generation(), generation);
    let torus = &set["torus"];
    assert_eq!(torus["band"], "pitch");
    assert_eq!(torus["settings"]["carrier"], json!(0.25));
    assert_eq!(torus["settings"]["polyphony"], 3);
    assert_eq!(torus["figure"], "spiral");
    let refused = d.send(json!({"operation":"torus-set","edit":{"polyphony":40}}));
    assert_eq!(refused["status"], "refused");
    assert!(refused["error"].as_str().unwrap().contains("Polyphony"));
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn the_comma_grows_only_as_the_field_runs_and_sync_follows_the_tempo() {
    let mut d = Driver::open();
    d.send(
        json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":40.0}}}),
    );
    assert_eq!(
        d.send(json!({"operation":"torus-state"}))["torus"]["comma_cents"],
        json!(0.0)
    );
    // 0.4 s at 40 steps a second: sixteen steps, one return.
    for _ in 0..(19_200 / 512 + 1) {
        let advanced = d.send(json!({"operation":"advance","frames":512,"muted":true}));
        assert_eq!(advanced["status"], "ok");
    }
    let comma = d.send(json!({"operation":"torus-state"}))["torus"]["comma_cents"]
        .as_f64()
        .unwrap();
    assert!((comma - 23.460_010_384_649_4).abs() < 1e-6, "{comma}");
    // Sync: a quarter note at 120 BPM is two steps a second; at 60 BPM, one.
    let synced = d.send(json!({"operation":"torus-set","edit":{
        "rate":{"mode":"sync","note":{"numerator":1,"denominator":4}},"tempo_bpm":120.0}}));
    assert_eq!(synced["torus"]["steps_per_second"], json!(2.0));
    let slower = d.send(json!({"operation":"torus-set","edit":{"tempo_bpm":60.0}}));
    assert_eq!(slower["torus"]["steps_per_second"], json!(1.0));
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_key_strikes_its_moving_point_and_always_the_same_helix_place() {
    let mut d = Driver::open();
    let key = json!([{"row":2,"column":1}]);
    let first = d.send(json!({"operation":"torus-play","keys":key,"amplitude":[0.1,0.0]}));
    assert_eq!(first["status"], "ok", "{first}");
    let note = &first["torus_play"]["notes"][0];
    let place = note["place"].clone();
    let point = note["point"].clone();
    assert_eq!(first["strike_at"]["points"][0]["theta"], point["theta"]);
    for _ in 0..10 {
        d.send(json!({"operation":"advance","frames":512,"muted":true}));
    }
    let again = d.send(json!({"operation":"torus-play","keys":key,"amplitude":[0.1,0.0]}));
    let later = &again["torus_play"]["notes"][0];
    assert_eq!(later["place"], place, "the same place of its helix");
    assert_ne!(
        later["point"], point,
        "the torus has carried it across the surface"
    );
    // The repeated key is a new note: the old one made way for it.
    assert_eq!(again["torus_play"]["made_way"].as_array().unwrap().len(), 1);
    // Each note names its own eight voices over its own four boundaries.
    assert_eq!(later["cell"]["voices_hz"].as_array().unwrap().len(), 8);
    assert_eq!(later["cell"]["boundaries"].as_array().unwrap().len(), 4);
    let released = d.send(json!({"operation":"torus-release","keys":key}));
    assert_eq!(released["status"], "ok");
    assert!(released["torus"]["notes"].as_array().unwrap().is_empty());
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn tuning_and_fundamental_change_the_voice_frequencies_the_owner_reports() {
    let mut d = Driver::open();
    let voices = |reply: &Value| -> Vec<f64> {
        reply["influence"]["voices"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["frequency_hz"].as_f64().unwrap())
            .collect()
    };
    let sky = d.send(json!({"operation":"torus-set","edit":{"tuning":"sky"}}));
    assert_eq!(sky["status"], "ok", "{sky}");
    assert_eq!(voices(&sky).len(), 9);
    let logic = d.send(json!({"operation":"torus-set","edit":{"tuning":"logic"}}));
    assert_eq!(logic["status"], "ok", "{logic}");
    assert_eq!(logic["influence"]["tuning"], "logic");
    assert_eq!(voices(&logic).len(), 8);
    assert_ne!(voices(&logic), voices(&sky)[..8].to_vec());
    assert_eq!(
        logic["influence"]["boundaries"].as_array().unwrap().len(),
        4
    );
    let generation = d.generation();
    let map = voices(&logic);
    let sun = logic["torus"]["map_hz"].as_f64().unwrap();
    let tuned = d.send(
        json!({"operation":"torus-set","edit":{"fundamental":{"source":"hz","hz":2.0 * sun}}}),
    );
    assert_eq!(tuned["status"], "ok", "{tuned}");
    assert_eq!(d.generation(), generation + 1, "one modes continuation");
    for (a, b) in map.iter().zip(voices(&tuned)) {
        assert!((b - 2.0 * a).abs() < 1e-3, "{b} vs {a}");
    }
}

/// The PCM of `seconds` of the field, rendered in blocks of 4800 samples.
fn render(d: &mut Driver, seconds: f64) -> Vec<f64> {
    let mut pcm = Vec::new();
    for _ in 0..(seconds * 10.0).round() as usize {
        let advanced = d.send(json!({"operation":"advance","frames":4800,"muted":false}));
        assert_eq!(advanced["status"], "ok", "{advanced}");
        pcm.extend(
            advanced["field"]["audio"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap()),
        );
    }
    pcm
}

fn rms(x: &[f64]) -> f64 {
    (x.iter().map(|v| v * v).sum::<f64>() / x.len() as f64).sqrt()
}

/// The normalised correlation of the signal with itself `lag` samples on.
fn correlation(x: &[f64], lag: usize) -> f64 {
    let (a, b) = (&x[..x.len() - lag], &x[lag..]);
    let dot: f64 = a.iter().zip(b).map(|(p, q)| p * q).sum();
    dot / (rms(a) * rms(b) * a.len() as f64)
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn lowering_the_rate_through_the_bands_is_heard_a_note_becoming_a_beat() {
    // Manual §21.2: "One pulse makes every sound of the Torus"; Carrier is how strongly the pulse
    // drives the body between strikes, and at 0 % the body sounds only when something strikes it.
    // At pitch rates the carrier sounds a note at the pulse's rate; at rhythm rates it beats.
    let mut d = Driver::open();
    // Quiet the opening ring (a damping edit re-excites nothing), then ring briefly: 40 a second.
    assert_eq!(
        d.send(json!({"operation":"set-damping","per_second":4000.0}))["status"],
        "ok"
    );
    render(&mut d, 0.1);
    assert_eq!(
        d.send(json!({"operation":"set-damping","per_second":40.0}))["status"],
        "ok"
    );
    let silent = render(&mut d, 1.0);
    assert!(
        silent.iter().all(|v| v.abs() < 1e-12),
        "at 0 % with no strike the body is silent"
    );
    // A note: 240 steps a second (200 samples a step at 48 kHz), Carrier 50 %.
    let note = d.send(json!({"operation":"torus-set","edit":{
        "carrier":0.5,"rate":{"mode":"free","steps_per_second":240.0}}}));
    assert_eq!(note["status"], "ok", "{note}");
    assert_eq!(note["torus"]["carrier"]["drive"], "pulse");
    let tone = render(&mut d, 0.5);
    let steady = &tone[12_000..];
    assert!(
        rms(steady) > 1e-4,
        "the carrier sounds: rms {}",
        rms(steady)
    );
    let at_step = correlation(steady, 200);
    assert!(at_step > 0.95, "periodic at the pulse's step: {at_step}");
    assert!(
        correlation(steady, 150) < at_step - 0.1,
        "and at the step, not any lag"
    );
    // A beat: lower the rate into Rhythm, four steps a second.
    let beat = d.send(
        json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":4.0}}}),
    );
    assert_eq!(beat["torus"]["band"], "rhythm");
    let beats = render(&mut d, 1.6);
    let envelope: Vec<f64> = beats[4_800..].chunks(240).map(rms).collect();
    let peak = envelope.iter().cloned().fold(0.0, f64::max);
    assert!(peak > 1e-4, "each step strikes the body: peak {peak}");
    let onsets = envelope
        .windows(2)
        .filter(|w| w[0] < 0.3 * peak && w[1] >= 0.3 * peak)
        .count();
    assert!(
        (5..=7).contains(&onsets),
        "a beat four times a second: {onsets} onsets in 1.5 s"
    );
    let quietest = envelope.iter().cloned().fold(f64::INFINITY, f64::min);
    assert!(
        quietest < 0.01 * peak,
        "the body falls quiet between beats: {quietest} of {peak}"
    );
    // Back to 0 %: the drive stops; the tail decays, and nothing sounds again unstruck.
    d.send(json!({"operation":"torus-set","edit":{"carrier":0.0}}));
    let after = render(&mut d, 1.0);
    assert!(
        after[24_000..].iter().all(|v| v.abs() < 1e-6 * peak),
        "the tail decays and nothing drives it"
    );
}
