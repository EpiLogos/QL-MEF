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
        json!({"operation":"torus-checkpoint"}),
        json!({"operation":"torus-restore","checkpoint":{"schema":"ql.torus-checkpoint/v1"}}),
        json!({"operation":"torus-render","checkpoint":{"schema":"ql.torus-checkpoint/v1"},"frames":4800,"acts":[
            {"action":"strike","at_samples_elapsed":"96000","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0]},
            {"action":"release","at_samples_elapsed":"96100","keys":[{"row":0,"column":2}]},
            {"action":"set","at_samples_elapsed":"96200","edit":{"polyphony":3}}]}),
        json!({"operation":"torus-render","continues":"render:1","frames":4800,"acts":[]}),
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
        json!({"operation":"torus-checkpoint","at":"0"}),
        json!({"operation":"torus-restore"}),
        json!({"operation":"torus-render","frames":4800,"acts":[{"action":"strike","at_samples_elapsed":"0","keys":[{"row":0,"column":2}]}]}),
        json!({"operation":"torus-render","frames":4800,"acts":[{"action":"hold","at_samples_elapsed":"0","keys":[]}]}),
        json!({"operation":"torus-render","frames":4800,"acts":[{"action":"release","at_samples_elapsed":"0","keys":[],"amplitude":[0.1,0.0]}]}),
        json!({"operation":"torus-render","frames":4800}),
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
    // The lens in force gives the surface its material (M2-1′, O-I #854): the opening event stands under L5′ at position 1.
    assert_eq!(torus["material"]["lens"], "L5'", "{}", torus["material"]);
    assert_eq!(torus["material"]["partner"], "L5");
    assert_eq!(torus["material"]["position"], 1);
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
    // The condition's maqām as the owner reads it (O-I #854): QL's default condition is Kirdan as tonic under the Bimba spelling, which
    // the map does not spell, so the reading names it and why it is absent, and Maqām is refused in those words, changing nothing.
    let maqam = &torus["maqam"];
    assert_eq!(maqam["name"], "Kirdan", "{maqam}");
    assert_eq!(maqam["coordinate"], "#2-4.3-0-4");
    assert!(
        maqam["absent"]
            .as_str()
            .unwrap()
            .contains("no Bimba spelling"),
        "{maqam}"
    );
    let generation = d.generation();
    let refused = d.send(json!({"operation":"torus-set","edit":{"tuning":"maqam"}}));
    assert_eq!(refused["status"], "refused", "{refused}");
    let why = refused["error"].as_str().unwrap();
    assert!(
        why.contains("Kirdan") && why.contains("no Bimba spelling"),
        "{why}"
    );
    assert_eq!(d.generation(), generation);
    let state = d.send(json!({"operation":"torus-state"}));
    assert_eq!(state["torus"]["settings"]["tuning"], "logic");
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn strike_is_the_share_of_a_keys_velocity_that_excites_the_body() {
    // Manual §21.2 Parameters: Strike "0 to 100% · 70% · How strongly velocity excites the body". Under Body source Tattva the element
    // sets it (Earth 45%); it is a session setting, so setting it moves no native state, and each key's amplitude excites by its share.
    let mut d = Driver::open();
    let state = d.send(json!({"operation":"torus-state"}));
    assert_eq!(state["torus"]["settings"]["strike"], json!(0.7), "{state}");
    let generation = d.generation();
    let set = d.send(json!({"operation":"torus-set","edit":{"strike":0.45}}));
    assert_eq!(set["status"], "ok", "{set}");
    assert_eq!(set["torus"]["settings"]["strike"], json!(0.45));
    assert_eq!(d.generation(), generation, "Strike moves no native state");
    let played = d.send(
        json!({"operation":"torus-play","keys":[{"row":2,"column":1}],"amplitude":[0.1,0.0]}),
    );
    assert_eq!(played["status"], "ok", "{played}");
    assert_eq!(played["torus_play"]["strike"], json!(0.45));
    let excited = played["strike_at"]["points"][0]["amplitude"][0]
        .as_f64()
        .unwrap();
    assert!((excited - 0.045).abs() < 1e-12, "{excited}");
    let refused = d.send(json!({"operation":"torus-set","edit":{"strike":1.5}}));
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"].as_str().unwrap().contains("Strike"),
        "{refused}"
    );
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
/// Each voice's state as the doubles it is, in mode order.
fn metres(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|pair| pair.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()))
        .collect()
}

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

/// O-I 855: a Live take begins again where it began. The Torus is kept at a
/// sample (its body as it rings, its M1′ session, the scene's event with its
/// seeds, the body's damping) and set again later: from there it rings on as it
/// rang on, bit for bit, while the cursor runs on. A damping moved since comes
/// back as one modes continuation. A checkpoint that cannot stand is refused
/// in words and changes nothing.
#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_torus_kept_at_a_sample_rings_on_from_its_restore_as_it_rang() {
    let mut d = Driver::open();
    let played = d.send(
        json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0]}),
    );
    assert_eq!(played["status"], "ok", "{played}");
    render(&mut d, 0.3);
    let generation = d.generation();
    let kept = d.send(json!({"operation":"torus-checkpoint"}));
    assert_eq!(kept["status"], "ok", "{kept}");
    assert_eq!(d.generation(), generation, "keeping the Torus is a read");
    let checkpoint = kept["torus_checkpoint"].clone();
    assert_eq!(checkpoint["schema"], "ql.torus-checkpoint/v1");
    assert_eq!(
        checkpoint["taken_at_samples"],
        kept["field"]["samples_elapsed"]
    );
    // The same doubles (the owner's text keeps every digit the worker wrote; the checkpoint's is the shortest).
    assert_eq!(
        metres(&checkpoint["amplitudes"]),
        metres(&kept["field"]["amplitudes_metres"])
    );
    assert_eq!(checkpoint["session"]["notes"].as_array().unwrap().len(), 1);
    assert!(
        checkpoint["seeds"].is_object()
            || checkpoint["seeds"].is_array()
            || checkpoint["seeds"].is_string(),
        "{checkpoint}"
    );
    let then = render(&mut d, 0.3);

    // The Torus moves on: another key, a setting, the body's damping, time.
    for command in [
        json!({"operation":"torus-play","keys":[{"row":4,"column":5}],"amplitude":[0.3,0.0]}),
        json!({"operation":"torus-set","edit":{"polyphony":3}}),
        json!({"operation":"set-damping","per_second":7.5}),
    ] {
        let answer = d.send(command.clone());
        assert_eq!(answer["status"], "ok", "{command}: {answer}");
    }
    render(&mut d, 0.5);

    // Refused in words, changing nothing.
    let standing = d.current.clone();
    for (path, value, words) in [
        ("/schema", json!("ql.torus-checkpoint/v2"), "checkpoint"),
        ("/sample_rate", json!(44100), "sample rate"),
        ("/amplitudes/0/0", json!(2e6), "state"),
        ("/voices/0", json!("mode:another"), "voices"),
        ("/session/steps", json!(-1.0), "steps"),
    ] {
        let mut bad = checkpoint.clone();
        *bad.pointer_mut(path).unwrap() = value;
        let refused = d.send(json!({"operation":"torus-restore","checkpoint":bad}));
        assert_eq!(refused["status"], "refused", "{path}: {refused}");
        assert!(
            refused["error"].as_str().unwrap().contains(words),
            "{path}: {refused}"
        );
        assert_eq!(
            refused["field"]["generation"], standing["field"]["generation"],
            "{path}"
        );
        assert_eq!(
            metres(&refused["field"]["amplitudes_metres"]),
            metres(&standing["field"]["amplitudes_metres"]),
            "{path}"
        );
    }

    let generation = d.generation();
    let restored = d.send(json!({"operation":"torus-restore","checkpoint":checkpoint}));
    assert_eq!(restored["status"], "ok", "{restored}");
    assert_eq!(
        d.generation(),
        generation + 2,
        "the damping, then the body: two acts"
    );
    assert_eq!(
        metres(&restored["field"]["amplitudes_metres"]),
        metres(&checkpoint["amplitudes"])
    );
    assert_eq!(
        restored["torus"]["settings"],
        checkpoint["session"]["settings"]
    );
    assert_eq!(
        restored["torus"]["notes"].as_array().unwrap().len(),
        1,
        "{restored}"
    );
    assert_eq!(
        restored["torus"]["comma_cents"],
        kept["torus"]["comma_cents"]
    );
    let again = render(&mut d, 0.3);
    assert_eq!(again, then, "it rings on as it rang");
}

/// The live owner's PCM from its cursor for `frames`, in blocks of `block`.
fn render_frames(d: &mut Driver, frames: u64, block: u64) -> Vec<f64> {
    let mut pcm = Vec::new();
    let mut done = 0;
    while done < frames {
        let n = block.min(frames - done);
        let advanced = d.send(json!({"operation":"advance","frames":n,"muted":false}));
        assert_eq!(advanced["status"], "ok", "{advanced}");
        pcm.extend(
            advanced["field"]["audio"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap()),
        );
        done += n;
    }
    pcm
}

fn audio(render: &Value) -> Vec<f64> {
    render["audio"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_take_renders_offline_from_its_checkpoint_as_the_owner_played_it() {
    // O-I #855: Export Audio takes a Live take through the M1′ session offline. The take is a checkpoint (the Torus as it rang when
    // the take began) and its acts at the samples they landed at; the render is a second owner of the same scene, set to the
    // checkpoint, each act made at its sample, its sound the owner's own. The live owner neither moves nor waits for it.
    let mut d = Driver::open();
    let played = d.send(
        json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0]}),
    );
    assert_eq!(played["status"], "ok", "{played}");
    render(&mut d, 0.2);
    let kept = d.send(json!({"operation":"torus-checkpoint"}));
    let checkpoint = kept["torus_checkpoint"].clone();
    let from: u64 = checkpoint["taken_at_samples"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let at = |offset: u64| (from + offset).to_string();
    // The take: a key ahead of the cursor, a release, a setting and a chord, as the live owner takes them.
    let acts = json!([
        {"action":"strike","at_samples_elapsed":at(4800),"keys":[{"row":2,"column":3}],"amplitude":[0.3,0.0]},
        {"action":"release","at_samples_elapsed":at(9600),"keys":[{"row":0,"column":2}]},
        {"action":"set","at_samples_elapsed":at(9600),"edit":{"polyphony":2}},
        {"action":"strike","at_samples_elapsed":at(14400),"keys":[{"row":4,"column":5},{"row":1,"column":1}],"amplitude":[0.25,0.0]},
    ]);
    let mut live = Vec::new();
    let strike = d.send(json!({"operation":"torus-play","keys":[{"row":2,"column":3}],"amplitude":[0.3,0.0],"at_samples_elapsed":at(4800)}));
    assert_eq!(strike["status"], "ok", "{strike}");
    live.extend(render_frames(&mut d, 9600, 4800));
    for command in [
        json!({"operation":"torus-release","keys":[{"row":0,"column":2}]}),
        json!({"operation":"torus-set","edit":{"polyphony":2}}),
        json!({"operation":"torus-play","keys":[{"row":4,"column":5},{"row":1,"column":1}],"amplitude":[0.25,0.0],"at_samples_elapsed":at(14400)}),
    ] {
        let answer = d.send(command.clone());
        assert_eq!(answer["status"], "ok", "{command}: {answer}");
    }
    let made_way = d.current["torus_play"]["made_way"].clone();
    live.extend(render_frames(&mut d, 14400, 4800));
    let frames = 24000u64;
    assert_eq!(live.len() as u64, frames);

    let standing = d.current.clone();
    let whole = d.send(
        json!({"operation":"torus-render","checkpoint":checkpoint,"acts":acts,"frames":frames}),
    );
    assert_eq!(whole["status"], "ok", "{whole}");
    assert_eq!(
        whole["field"]["generation"], standing["field"]["generation"],
        "the live owner does not move"
    );
    assert_eq!(
        whole["field"]["samples_elapsed"],
        standing["field"]["samples_elapsed"]
    );
    assert_eq!(whole["field"]["audio"], json!([]), "nor sound");
    let taken = &whole["torus_render"];
    assert_eq!(taken["schema"], "ql.torus-render/v1");
    assert_eq!(taken["from_samples"], json!(from.to_string()));
    assert_eq!(taken["frames"], json!(frames));
    assert_eq!(taken["sample_rate"], d.current["field"]["sample_rate"]);
    assert_eq!(
        taken["made_way"],
        json!([[], made_way]),
        "the notes that made way, strike by strike, as the owner made them"
    );
    assert!(
        !made_way.as_array().unwrap().is_empty(),
        "past Polyphony 2 the oldest note made way"
    );
    let pcm = audio(taken);
    assert_eq!(pcm.len() as u64, frames);
    assert!(pcm.iter().any(|x| *x != 0.0), "the take sounds");
    assert_eq!(
        pcm, live,
        "sample for sample, the sound the live owner made"
    );
    assert_eq!(
        taken["checkpoint"]["taken_at_samples"],
        json!(at(frames)),
        "and where it ends, kept as the owner keeps it"
    );

    // In parts: a continuation runs on the render owner; an end checkpoint opens another the same.
    let first = d.send(json!({"operation":"torus-render","checkpoint":checkpoint,"acts":acts.as_array().unwrap()[..1],"frames":9600}));
    assert_eq!(first["status"], "ok", "{first}");
    let token = first["torus_render"]["continues"]
        .as_str()
        .unwrap()
        .to_owned();
    let second = d.send(json!({"operation":"torus-render","continues":token,"acts":acts.as_array().unwrap()[1..],"frames":frames - 9600}));
    assert_eq!(second["status"], "ok", "{second}");
    let mut parts = audio(&first["torus_render"]);
    parts.extend(audio(&second["torus_render"]));
    assert_eq!(parts, live, "a continued render is the whole render");
    let again = d.send(json!({"operation":"torus-render","checkpoint":first["torus_render"]["checkpoint"],"acts":acts.as_array().unwrap()[1..],"frames":frames - 9600}));
    assert_eq!(again["status"], "ok", "{again}");
    assert_eq!(
        audio(&again["torus_render"]),
        audio(&second["torus_render"]),
        "the end checkpoint keeps the body exactly"
    );

    // Refused in words, the live owner untouched.
    for (command, words) in [
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[],"frames":0}),
            "frames",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[],"frames":(1u64 << 20) + 1}),
            "frames",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[{"action":"release","at_samples_elapsed":(from - 1).to_string(),"keys":[{"row":0,"column":2}]}],"frames":4800}),
            "before",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[{"action":"release","at_samples_elapsed":at(4800),"keys":[{"row":0,"column":2}]}],"frames":4800}),
            "after",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[
            {"action":"release","at_samples_elapsed":at(200),"keys":[{"row":0,"column":2}]},
            {"action":"release","at_samples_elapsed":at(100),"keys":[{"row":0,"column":2}]}],"frames":4800}),
            "order",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"continues":token,"acts":[],"frames":4800}),
            "either",
        ),
        (
            json!({"operation":"torus-render","acts":[],"frames":4800}),
            "either",
        ),
        (
            json!({"operation":"torus-render","continues":"render:0","acts":[],"frames":4800}),
            "continues",
        ),
        (
            json!({"operation":"torus-render","checkpoint":{"schema":"ql.torus-checkpoint/v2"},"acts":[],"frames":4800}),
            "checkpoint",
        ),
    ] {
        let refused = d.send(command.clone());
        assert_eq!(refused["status"], "refused", "{command}: {refused}");
        assert!(
            refused["error"].as_str().unwrap().contains(words),
            "{words}: {refused}"
        );
        assert_eq!(
            refused["field"]["generation"],
            standing["field"]["generation"]
        );
        assert_eq!(
            refused["field"]["samples_elapsed"],
            standing["field"]["samples_elapsed"]
        );
    }
}
