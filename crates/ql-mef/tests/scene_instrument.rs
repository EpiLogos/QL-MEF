//! The scene instrument over the real installed worker: the dated sky sounds on
//! the M1 torus. One determinant is varied while subject, clock and material are
//! held. Run with `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test
//! scene_instrument -- --ignored`.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::StrikeInput;
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

/// Canonical 44-byte-header 16-bit mono PCM WAV of produced frames; the plain
/// container a host OS audio player opens without any project context.
fn wav16(samples: &[f64], rate: u32) -> Vec<u8> {
    let mut wav = Vec::with_capacity(44 + samples.len() * 2);
    let data_len = (samples.len() * 2) as u32;
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_len).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&rate.to_le_bytes());
    wav.extend_from_slice(&(rate * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    for x in samples {
        let scaled = (x * 32767.0).round().clamp(-32768.0, 32767.0) as i16;
        wav.extend_from_slice(&scaled.to_le_bytes());
    }
    wav
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_played_strike_sounds_through_the_production_path_and_renders_a_wav() {
    let (mut played, mut control) = (open(), open());
    // Two owners over one event hold one quiet ringing body.
    let seed = played.session_mut().advance_field(1024, false).unwrap();
    assert_eq!(
        seed,
        control.session_mut().advance_field(1024, false).unwrap()
    );
    // The played act: strike the Moon voice once, hard. It is a performance
    // act, not a determinant — identity and clock continue on the same body.
    let moon_ref = played.shape().voices[3].mode_ref.clone();
    assert_eq!(moon_ref, format!("scene:planet/{}", PLANETS[3]));
    let struck = played
        .strike(&[StrikeInput {
            mode_ref: moon_ref,
            amplitude: [0.9, 0.0],
        }])
        .unwrap();
    assert_eq!(struck["event_ref"], seed["event_ref"]);
    assert_eq!(struck["subject_ref"], seed["subject_ref"]);
    assert_eq!(struck["clock"], seed["clock"]);
    assert_eq!(
        struck["generation"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap(),
        seed["generation"].as_str().unwrap().parse::<u64>().unwrap() + 1
    );
    assert_eq!(struck["samples_elapsed"], seed["samples_elapsed"]);
    // Exactly the struck voice's amplitude moved; every other voice, the clock
    // and the sample basis continue bit-identically.
    let (seed_amplitudes, struck_amplitudes) = (
        seed["amplitudes_metres"].as_array().unwrap(),
        struck["amplitudes_metres"].as_array().unwrap(),
    );
    assert_eq!(seed_amplitudes.len(), PLANETS.len());
    for (i, (before, after)) in seed_amplitudes.iter().zip(struck_amplitudes).enumerate() {
        if i == 3 {
            assert_ne!(before, after, "the struck voice did not move");
        } else {
            assert_eq!(before, after, "unstruck voice {i} moved under the strike");
        }
    }
    // An act naming no current voice is refused without touching the owner.
    let retained = played.session().last_field().clone();
    let refused = played.strike(&[StrikeInput {
        mode_ref: "scene:planet/none".into(),
        amplitude: [0.1, 0.0],
    }]);
    assert!(refused.is_err());
    assert_eq!(played.session().last_field(), &retained);
    // The struck body sounds: three produced blocks, then the plain WAV file
    // a player opens. The unstruck twin is the control.
    let (mut struck_pcm, mut control_pcm) = (Vec::new(), Vec::new());
    for _ in 0..3 {
        struck_pcm.extend(audio(
            &played.session_mut().advance_field(8192, false).unwrap(),
        ));
        control_pcm.extend(audio(
            &control.session_mut().advance_field(8192, false).unwrap(),
        ));
    }
    let moon_hz = played.shape().voices[3].frequency_hz;
    let struck_power = power(&struck_pcm, moon_hz);
    assert!(
        struck_power > 50.0 * power(&control_pcm, moon_hz),
        "the played strike did not sound the struck voice: {struck_power} vs {}",
        power(&control_pcm, moon_hz),
    );
    assert_ne!(struck_pcm, control_pcm, "the produced PCM did not change");
    let wav = wav16(&struck_pcm, 48_000);
    let path = std::env::temp_dir().join(format!("ql-played-strike-{}.wav", std::process::id()));
    std::fs::write(&path, &wav).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WAVE");
    assert_eq!(&bytes[12..16], b"fmt ");
    assert_eq!(&bytes[36..40], b"data");
    assert_eq!(
        u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
        48_000
    );
    assert_eq!(
        u16::from_le_bytes([bytes[22], bytes[23]]),
        1,
        "mono channel"
    );
    assert_eq!(
        u16::from_le_bytes([bytes[34], bytes[35]]),
        16,
        "16-bit samples"
    );
    assert_eq!(bytes.len(), 44 + struck_pcm.len() * 2);
    let _ = std::fs::remove_file(&path);
    if let Ok(directory) = std::env::var("QL_SCENE_WORLD_RECEIPT_DIR") {
        let target = std::path::Path::new(&directory);
        std::fs::create_dir_all(target).unwrap();
        std::fs::write(target.join("played-strike.wav"), &wav).unwrap();
        std::fs::write(
            target.join("played-strike.json"),
            serde_json::to_vec_pretty(&json!({
                "schema":"ql.scene-played-strike-proof/v1", "worker_path":worker(),
                "sample_rate":48_000, "struck_voice":{"mode_ref":format!("scene:planet/{}", PLANETS[3]), "frequency_hz":moon_hz},
                "strike_amplitude_metres":[0.9, 0.0], "struck_power":struck_power,
                "control_power":power(&control_pcm, moon_hz),
                "wav":"played-strike.wav", "unstruck_voice_powers":played.shape().voices
                    .iter().filter(|v| v.mode_ref != format!("scene:planet/{}", PLANETS[3]))
                    .map(|v| json!({"planet_ref":v.planet_ref, "frequency_hz":v.frequency_hz}))
                    .collect::<Vec<_>>(),
                "environment":"actual native worker PCM rendered to a playable WAV; hardware audio not asserted"
            }))
            .unwrap(),
        )
        .unwrap();
    }
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_played_strike_through_the_host_is_subject_scoped_and_cursor_exact() {
    use ql_mef::continuous::host::{FieldHost, HOST_REQUEST, HostRequest};
    let config = config();
    let instance = config.instance_ref.clone();
    let mut host = FieldHost::open_scene(&worker(), config, Duration::from_secs(20)).unwrap();
    let initial = host.ready()["field"].clone();
    // The played addresses travel with the owner's own influence reading.
    let inspect = host.execute(
        serde_json::from_value::<HostRequest>(json!({
            "schema":HOST_REQUEST, "instance_ref":instance, "event_ref":initial["event_ref"],
            "subject_ref":initial["subject_ref"], "request_id":"1",
            "expected_generation":initial["generation"],
            "expected_samples_elapsed":initial["samples_elapsed"], "command":{"operation":"inspect"}
        }))
        .unwrap(),
    );
    assert_eq!(inspect["status"], "ok");
    let moon = inspect["influence"]["voices"].as_array().unwrap()[3]["mode_ref"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_eq!(moon, format!("scene:planet/{}", PLANETS[3]));
    let strike = |id: &str, generation: &Value, person: &Value, mode: &str| -> HostRequest {
        serde_json::from_value(json!({"schema":HOST_REQUEST, "instance_ref":instance,
            "event_ref":initial["event_ref"], "subject_ref":person, "request_id":id,
            "expected_generation":generation,
            "expected_samples_elapsed":initial["samples_elapsed"],
            "command":{"operation":"strike","strikes":[{"mode_ref":mode,"amplitude":[0.9,0.0]}]}}))
        .unwrap()
    };
    let foreign = host.execute(strike(
        "2",
        &initial["generation"],
        &json!("person:foreign"),
        &moon,
    ));
    assert_eq!(foreign["status"], "refused");
    assert_eq!(foreign["field"], initial);
    let admitted = host.execute(strike(
        "2",
        &initial["generation"],
        &initial["subject_ref"],
        &moon,
    ));
    assert_eq!(admitted["status"], "ok");
    assert_ne!(
        admitted["field"]["amplitudes_metres"],
        initial["amplitudes_metres"]
    );
    assert_eq!(admitted["field"]["clock"], initial["clock"]);
    assert_eq!(
        admitted["field"]["generation"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap(),
        initial["generation"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap()
            + 1
    );
    // A strike is a performance act, not a determinant: it answers with the
    // field alone, and the owner's influence reading is unchanged after it.
    assert!(
        admitted.get("influence").is_none(),
        "a strike answered as a determinant"
    );
    let reread = host.execute(serde_json::from_value::<HostRequest>(json!({
        "schema":HOST_REQUEST, "instance_ref":instance, "event_ref":initial["event_ref"],
        "subject_ref":initial["subject_ref"], "request_id":"3",
        "expected_generation":admitted["field"]["generation"],
        "expected_samples_elapsed":admitted["field"]["samples_elapsed"], "command":{"operation":"inspect"}
    }))
    .unwrap());
    assert_eq!(reread["status"], "ok");
    // Every source determinant is unchanged; only the control cursor moved.
    for key in [
        "voices",
        "address72",
        "shape_ref",
        "material",
        "m1_revision",
        "m3_generation",
        "event_ref",
        "subject_ref",
        "native_readback",
    ] {
        assert_eq!(
            reread["influence"][key], inspect["influence"][key],
            "a strike changed determinant {key}"
        );
    }
    let stale = host.execute(strike(
        "4",
        &initial["generation"],
        &initial["subject_ref"],
        &moon,
    ));
    assert_eq!(stale["status"], "refused");
    assert_eq!(stale["field"], reread["field"]);
    let unknown = host.execute(strike(
        "5",
        &reread["field"]["generation"],
        &initial["subject_ref"],
        "scene:planet/none",
    ));
    assert_eq!(unknown["status"], "refused");
    assert_eq!(unknown["field"], reread["field"]);
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

/// Real worker regression: D30 continues all nine resident amplitudes. This is
/// deliberately configured/ignored until a real worker is supplied; a normal
/// ignored-suite green supplies no native damping acceptance.
#[test]
#[ignore = "requires the installed ql-field-worker"]
fn scene_damping_is_acknowledged_without_a_strike_shape_or_clock_reset() {
    // WorldRequest creates M2 and M3 stamps from one EventIdentity. Predict
    // its lawful successor before opening either native worker; this metadata
    // continuation does not turn M3's clock or change its symbolic form.
    let constructor = config();
    assert_eq!(
        constructor.basis.m3.stamp.identity,
        constructor.basis.m2.stamp.identity
    );
    assert!(constructor.basis.m3_commands.is_empty());
    let expected_profile_generation = constructor
        .basis
        .m2
        .stamp
        .identity
        .profile_generation
        .checked_add(1)
        .unwrap();
    let mut low =
        SceneInstrument::open(&worker(), constructor.clone(), Duration::from_secs(20)).unwrap();
    let mut high =
        SceneInstrument::open(&worker(), constructor.clone(), Duration::from_secs(20)).unwrap();
    let seed_low = low.session_mut().advance_field(256, false).unwrap();
    let seed_high = high.session_mut().advance_field(256, false).unwrap();
    assert_eq!(seed_low, seed_high);
    let shape = low.shape().clone();
    let before = low.influence();
    let a = low.set_damping(0.0).unwrap();
    let b = high.set_damping(2.0).unwrap();
    assert_eq!(
        a, b,
        "policy admission preserves the exact resident field state"
    );
    assert_eq!(a["amplitudes_metres"], seed_low["amplitudes_metres"]);
    assert_eq!(a["targets"], seed_low["targets"]);
    assert_eq!(a["clock"], seed_low["clock"]);
    assert_eq!(a["samples_elapsed"], seed_low["samples_elapsed"]);
    assert!(audio(&a).is_empty());
    assert_eq!(
        a["generation"].as_str().unwrap().parse::<u64>().unwrap(),
        seed_low["generation"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap()
            + 1
    );
    assert_eq!(low.shape(), &shape);
    assert_eq!(high.shape(), &shape);
    for instrument in [&low, &high] {
        let after = instrument.influence();
        assert_eq!(after["m3_generation"], expected_profile_generation);
        assert_eq!(
            after["native_readback"]["profile_generation"],
            expected_profile_generation
        );
        assert_eq!(
            after["native_readback"]["m3_generation"],
            expected_profile_generation
        );
        assert_eq!(after["m1_revision"], constructor.basis.m1.revision);
        for key in [
            "event_ref",
            "subject_ref",
            "m1_revision",
            "shape_ref",
            "address72",
            "voices",
            "geometry",
            "material_standing",
        ] {
            assert_eq!(
                after[key], before[key],
                "only declared damping changes: {key}"
            );
        }
        for key in [
            "m1_clock",
            "m1_carrier",
            "m3_clock",
            "selected_aperture",
            "form",
            "continuous_clock",
            "continuation_start",
        ] {
            assert_eq!(
                after["native_readback"][key], before["native_readback"][key],
                "clock/form invariant: {key}"
            );
        }
    }
    assert_eq!(low.influence()["material"]["damping_per_second"], 0.0);
    assert_eq!(high.influence()["material"]["damping_per_second"], 2.0);
    let fa = low.session_mut().advance_field(4096, false).unwrap();
    let fb = high.session_mut().advance_field(4096, false).unwrap();
    let decay = (-2.0 * 4096.0 / 48_000.0_f64).exp();
    // One complex multiply per sample, two source-qualified exponential/step
    // evaluations. Bound grows with the actual operation count, not a relaxed
    // semantic expectation. Native exact zero elapsed checks above stay exact.
    let roundoff = 64.0 * 4096.0 * f64::EPSILON;
    assert_eq!(fa["amplitudes_metres"].as_array().unwrap().len(), 9);
    for (x, y) in fa["amplitudes_metres"]
        .as_array()
        .unwrap()
        .iter()
        .zip(fb["amplitudes_metres"].as_array().unwrap())
    {
        let ax = x[0].as_f64().unwrap().hypot(x[1].as_f64().unwrap());
        let ay = y[0].as_f64().unwrap().hypot(y[1].as_f64().unwrap());
        assert!(ax > 0.0 && ay > 0.0);
        assert!(
            (ay / ax - decay).abs() <= roundoff,
            "nine-mode damping prediction: {ax} {ay} {decay}"
        );
    }
    assert_eq!(fa["clock"], fb["clock"]);
    assert_eq!(fa["samples_elapsed"], fb["samples_elapsed"]);
    assert_ne!(
        fa["targets"], fb["targets"],
        "the native receiving body changes"
    );
    assert_ne!(fa["audio"], fb["audio"], "the native PCM changes");
    let current = high.session().last_field().clone();
    let influence = high.influence();
    for invalid in [-1.0, 1_000_001.0, f64::NAN, f64::INFINITY] {
        assert!(high.set_damping(invalid).is_err());
        assert_eq!(high.session().last_field(), &current);
        assert_eq!(high.influence(), influence);
    }
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn scene_damping_host_requires_exact_scope_cursor_and_complete_ack() {
    use ql_mef::continuous::host::{FieldHost, HOST_REQUEST, HostRequest};
    let config = config();
    let instance = config.instance_ref.clone();
    let mut host = FieldHost::open_scene(&worker(), config, Duration::from_secs(20)).unwrap();
    let initial = host.ready()["field"].clone();
    let request = |id: &str, generation: &Value, person: &Value, value: f64| -> HostRequest {
        serde_json::from_value(json!({"schema":HOST_REQUEST,"instance_ref":instance,
            "event_ref":initial["event_ref"],"subject_ref":person,"request_id":id,
            "expected_generation":generation,"expected_samples_elapsed":initial["samples_elapsed"],
            "command":{"operation":"set-damping","per_second":value}}))
        .unwrap()
    };
    let foreign = host.execute(request(
        "1",
        &initial["generation"],
        &json!("person:foreign"),
        2.0,
    ));
    assert_eq!(foreign["status"], "refused");
    assert_eq!(foreign["field"], initial);
    let admitted = host.execute(request(
        "1",
        &initial["generation"],
        &initial["subject_ref"],
        2.0,
    ));
    assert_eq!(admitted["status"], "ok");
    assert_eq!(admitted["influence"]["material"]["damping_per_second"], 2.0);
    assert_eq!(
        admitted["influence"]["generation"],
        admitted["field"]["generation"]
    );
    assert_eq!(
        admitted["field"]["amplitudes_metres"],
        initial["amplitudes_metres"]
    );
    let stale = host.execute(request(
        "2",
        &initial["generation"],
        &initial["subject_ref"],
        0.0,
    ));
    assert_eq!(stale["status"], "refused");
    assert_eq!(stale["field"], admitted["field"]);
    let invalid = host.execute(request(
        "3",
        &admitted["field"]["generation"],
        &initial["subject_ref"],
        -1.0,
    ));
    assert_eq!(invalid["status"], "refused");
    assert_eq!(invalid["field"], admitted["field"]);
    assert!(serde_json::from_value::<HostRequest>(json!({"schema":HOST_REQUEST,"instance_ref":instance,
        "event_ref":initial["event_ref"],"subject_ref":initial["subject_ref"],"request_id":"4",
        "expected_generation":admitted["field"]["generation"],"expected_samples_elapsed":initial["samples_elapsed"],
        "command":{"operation":"set-damping","per_second":2.0,"strike":true}})).is_err());
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_played_class_strikes_the_voices_nearest_it_and_never_retunes_them() {
    let (mut played, mut control) = (open(), open());
    let influence = played.influence();
    let played_addresses = &influence["played_addresses"];
    assert_eq!(
        played_addresses["standing"],
        ql_mef::continuous::scene_field::SCENE_PLAYED_STANDING
    );
    let classes = played_addresses["by_class"].as_array().unwrap();
    assert_eq!(classes.len(), 12);
    // Every standing voice is addressed by exactly one class, the one its own
    // pitch is nearest over C3, within a quarter-tone.
    let root = ql_mef::continuous::coupled::SKY_ROOT_HZ;
    let mut addressed = 0;
    for (class, entry) in classes.iter().enumerate() {
        assert_eq!(entry["pitch_class"], class);
        for voice in entry["voices"].as_array().unwrap() {
            addressed += 1;
            let hz = voice["frequency_hz"].as_f64().unwrap();
            let semitones = 12.0 * (hz / root).log2();
            assert_eq!(
                semitones.round().rem_euclid(12.0) as usize,
                class,
                "{voice}"
            );
            assert!(voice["cents_from_class"].as_f64().unwrap().abs() <= 50.0);
        }
    }
    assert_eq!(addressed, PLANETS.len());
    // A key of the first sounding class: strike exactly the addressed refs.
    let key = classes
        .iter()
        .find(|c| !c["voices"].as_array().unwrap().is_empty())
        .expect("some class sounds");
    let acts: Vec<StrikeInput> = key["voices"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| StrikeInput {
            mode_ref: v["mode_ref"].as_str().unwrap().into(),
            amplitude: [0.9, 0.0],
        })
        .collect();
    let before: Vec<f64> = played
        .shape()
        .voices
        .iter()
        .map(|v| v.frequency_hz)
        .collect();
    played.session_mut().advance_field(1024, false).unwrap();
    control.session_mut().advance_field(1024, false).unwrap();
    played.strike(&acts).unwrap();
    let (mut struck_pcm, mut control_pcm) = (Vec::new(), Vec::new());
    for _ in 0..3 {
        struck_pcm.extend(audio(
            &played.session_mut().advance_field(8192, false).unwrap(),
        ));
        control_pcm.extend(audio(
            &control.session_mut().advance_field(8192, false).unwrap(),
        ));
    }
    for v in key["voices"].as_array().unwrap() {
        let hz = v["frequency_hz"].as_f64().unwrap();
        assert!(
            power(&struck_pcm, hz) > 50.0 * power(&control_pcm, hz),
            "class key did not sound its voice at {hz} Hz"
        );
    }
    // Played, never retuned: every voice keeps its own just-ratio pitch.
    let after: Vec<f64> = played
        .shape()
        .voices
        .iter()
        .map(|v| v.frequency_hz)
        .collect();
    assert_eq!(before, after);
}
