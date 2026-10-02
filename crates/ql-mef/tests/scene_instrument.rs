//! The scene instrument over the real installed worker: the dated sky sounds on
//! the M1 torus. One determinant is varied while subject, clock and material are
//! held. Run with `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test
//! scene_instrument -- --ignored`.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::scene_field::{PLANETS, SceneConfig, SceneInstrument};
use serde_json::{Value, json};

fn worker() -> PathBuf {
    PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker"))
}

fn sky() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap()
}

fn config() -> SceneConfig {
    let sky = sky();
    let request: ql_mef::scene::WorldRequest = serde_json::from_value(json!({
        "schema": ql_mef::scene::WORLD_REQUEST, "instance_ref": "test:scene",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-scene-instrument",
        "texture": [64, 64], "units_per_metre": 1.0, "sky": sky,
        "geometry": {"longitude_samples": 32, "latitude_samples": 16, "metres_per_unit": 1.0, "attachment": 1}
    })).unwrap();
    let binding = ql_mef::scene::world(request).unwrap()["binding"].clone();
    assert_eq!(
        binding["presentation"]["slots_a"].as_array().unwrap().len(),
        4096
    );
    serde_json::from_value(binding["host"].clone()).unwrap()
}

fn open() -> SceneInstrument {
    SceneInstrument::open(&worker(), config(), Duration::from_secs(20)).unwrap()
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

fn max_move(a: &Value, b: &Value) -> f64 {
    targets(a)
        .iter()
        .zip(targets(b).iter())
        .map(|(x, y)| {
            x.iter()
                .zip(y)
                .map(|(p, q)| (p - q).powi(2))
                .sum::<f64>()
                .sqrt()
        })
        .fold(0.0, f64::max)
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn the_sky_sounds_at_its_just_octave_on_the_torus() {
    let mut instrument = open();
    let shape = instrument.shape().clone();
    assert_eq!(shape.voices.len(), PLANETS.len());
    // One worker block is only 0.171 s: at a 130.8 Hz root the previous
    // off-scale probe (root × 1.03) lay inside that block's 5.86 Hz spectral
    // resolution. Receive a full second of actual continued PCM so the
    // independently chosen off-scale frequency can discriminate a voice.
    let mut pcm = Vec::new();
    for _ in 0..6 {
        let field = instrument.session_mut().advance_field(8192, false).unwrap();
        pcm.extend(audio(&field));
    }
    // Every planet's voice carries energy; an off-scale pitch between voices does not.
    let root = shape.voices[0].frequency_hz;
    let powers: Vec<_> = shape
        .voices
        .iter()
        .map(|voice| {
            json!({
                "planet_ref": voice.planet_ref, "frequency_hz": voice.frequency_hz,
                "power": power(&pcm, voice.frequency_hz),
                "ratio_to_off_scale": power(&pcm, voice.frequency_hz) / power(&pcm, root * 1.03)
            })
        })
        .collect();
    if let Ok(directory) = std::env::var("QL_SCENE_WORLD_RECEIPT_DIR") {
        let path = std::path::Path::new(&directory);
        std::fs::create_dir_all(path).unwrap();
        std::fs::write(
            path.join("sky-pcm.json"),
            serde_json::to_vec_pretty(&json!({
                "schema":"ql.scene-audio-causal-proof/v1", "worker_path":worker(),
                "sample_rate":48_000,"frames":pcm.len(),"actual_pcm":pcm,
                "off_scale_frequency_hz":root * 1.03,"powers":powers,
                "native_influence":instrument.influence(),
                "environment":"actual native worker PCM; hardware audio not asserted"
            }))
            .unwrap(),
        )
        .unwrap();
    }
    for voice in &shape.voices {
        assert!(
            power(&pcm, voice.frequency_hz) > 10.0 * power(&pcm, root * 1.03),
            "{} is not sounding; powers={powers:?}",
            voice.planet_ref,
        );
    }
    // The Moon sits a just fourth above the Sun.
    assert!((shape.voices[3].frequency_hz / root - 4.0 / 3.0).abs() < 1e-12);
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_disconnected_sun_audio_consumer_is_detected_with_source_and_body_retained() {
    use ql_mef::continuous::coupled::CoupledFieldSession;
    let mut control = open();
    let basis = control.session_mut().current_basis().input.clone();
    let mut field = control.session_mut().original_field().clone();
    // Disconnect one actual receiving contribution, leaving the admitted
    // source, all nine modal voices and the same surface sampling intact.
    // This is an existing native audio gain, not a synthetic waveform.
    field.audio_gains[0] = 0.0;
    let mut disconnected =
        CoupledFieldSession::open(&worker(), basis, field, Duration::from_secs(20)).unwrap();
    assert_eq!(
        control.session_mut().current_basis().m2,
        disconnected.current_basis().m2
    );
    let mut reference_pcm = Vec::new();
    let mut disconnected_pcm = Vec::new();
    for _ in 0..6 {
        let reference = control.session_mut().advance_field(8192, false).unwrap();
        let varied = disconnected.advance_field(8192, false).unwrap();
        assert_eq!(reference["targets"], varied["targets"]);
        reference_pcm.extend(audio(&reference));
        disconnected_pcm.extend(audio(&varied));
    }
    let root = control.shape().voices[0].frequency_hz;
    let sun_ratio = power(&disconnected_pcm, root) / power(&reference_pcm, root);
    assert!(
        sun_ratio < 0.01,
        "lost Sun consumer was not detected: {sun_ratio}"
    );
    let off_scale = power(&disconnected_pcm, root * 1.03);
    assert!(
        power(&disconnected_pcm, root) < 10.0 * off_scale,
        "the positive nine-voice acceptance would falsely pass a disconnected Sun"
    );
    for voice in &control.shape().voices[1..] {
        let ratio = power(&disconnected_pcm, voice.frequency_hz)
            / power(&reference_pcm, voice.frequency_hz);
        assert!(
            (ratio - 1.0).abs() < 0.05,
            "unrelated {} voice changed: {ratio}",
            voice.planet_ref
        );
    }
    if let Ok(directory) = std::env::var("QL_SCENE_WORLD_RECEIPT_DIR") {
        let path = std::path::Path::new(&directory);
        std::fs::create_dir_all(path).unwrap();
        std::fs::write(
            path.join("disconnected-sun-audio-consumer.json"),
            serde_json::to_vec_pretty(&json!({
                "schema":"ql.scene-native-consumer-negative/v1","worker_path":worker(),
                "source_and_nine_modal_voices_retained":true,"actual_body_targets_invariant":true,
                "disconnected_input":"field.audio_gains[0]=0","sun_power_ratio":sun_ratio,
                "reference_pcm":reference_pcm,"disconnected_pcm":disconnected_pcm,
                "positive_nine_voice_acceptance_rejects_disconnected_output":true
            }))
            .unwrap(),
        )
        .unwrap();
    }
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn one_m1_tick_reshapes_the_skin_and_leaves_the_sky_in_tune() {
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
    assert_ne!(varied.influence()["m1_revision"], revision);
    assert_eq!(after.address72, (before.address72 + 1) % 72);
    assert_ne!(
        after.shape_ref, before.shape_ref,
        "the skin re-reads on a tick"
    );
    for (old, new) in before.voices.iter().zip(after.voices.iter()) {
        // Pitch belongs to the sky and M1's ratio, not the tick.
        assert_eq!(old.frequency_hz, new.frequency_hz);
        assert_eq!(old.longitude_radians, new.longitude_radians);
    }
    let c = control.session_mut().advance_field(4096, false).unwrap();
    let v = varied.session_mut().advance_field(4096, false).unwrap();
    assert!(max_move(&c, &v) > 1e-3, "the body did not follow the tick");
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_moving_planet_carries_its_voice_round_the_clock() {
    let mut instrument = open();
    instrument.session_mut().advance_field(2048, false).unwrap();
    let before = instrument.shape().clone();
    let mut event = instrument.event();
    // Mars (native planet 4) moves 40° along the ecliptic.
    let mars = event
        .m2
        .world_observations
        .iter_mut()
        .find(|o| o.planet_id == 4)
        .unwrap();
    mars.longitude_degrees = (mars.longitude_degrees + 40.0) % 360.0;
    instrument.replace(&event, false).unwrap();
    let after = instrument.shape().clone();
    let idx = PLANETS.iter().position(|p| *p == "#2-5-7").unwrap();
    assert!(
        (after.voices[idx].longitude_radians - before.voices[idx].longitude_radians).abs() > 0.5
    );
    assert_eq!(
        after.voices[idx].frequency_hz, before.voices[idx].frequency_hz,
        "a planet keeps its pitch as it moves"
    );
    for (i, (old, new)) in before.voices.iter().zip(after.voices.iter()).enumerate() {
        if i != idx {
            assert_eq!(old.longitude_radians, new.longitude_radians);
        }
    }
    assert_ne!(after.shape_ref, before.shape_ref);
}

#[test]
fn a_supplied_resonator_or_binding_is_refused() {
    let mut foreign = config();
    foreign.basis.frequency_bindings = vec![ql_mef::continuous::coupled::FrequencyBinding {
        mode_ref: "x".into(),
        octet_index: 0,
    }];
    assert!(foreign.validate().is_err());
}
