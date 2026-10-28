//! A strike at a point of the torus (QL-MEF, DLW manual §21.2 "What strikes the
//! Torus": a touch is "the point of the surface you touch"). The operation is
//! lowered at the scene body to the named-voice strike the native owner already
//! admits: each standing voice receives `amplitude × χ(voice, φ, θ)`, the
//! voice's own shape term at the struck point, summed over the points of one
//! act. These tests need no native worker; the worker-backed ones live in
//! `scene_instrument.rs` (`--ignored`).
use std::f64::consts::{PI, TAU};

use ql_mef::continuous::scene_field::{
    PLANETS, STRIKE_AT_RECEIPT, SceneConfig, ShapeBasis, VOICES, Voice, complete,
};
use ql_mef::continuous::{MAX_STRIKE_POINTS, StrikeAtInput, StrikeInput};
use ql_mef::m2_engine::MaterialFibre;
use serde_json::{Value, json};

/// The M2′ Chladni term on the torus, L = 2π (M2-ARCHITECTURE §5.3.1), written
/// out independently of the crate's own `chi` so a drift in either shows here.
fn oracle(voice: &Voice, phi: f64, theta: f64) -> f64 {
    let phi = phi - voice.longitude_radians;
    let (m, n) = (f64::from(voice.m), f64::from(voice.n));
    let (a, b) = (
        voice.weight * voice.phase_radians.cos(),
        voice.weight * voice.phase_radians.sin(),
    );
    a * (m * phi / 2.0).sin() * (n * theta / 2.0).sin()
        + b * (m * phi / 2.0).cos() * (n * theta / 2.0).cos()
}

fn synthetic_voice(i: usize, phase: f64) -> Voice {
    // Distinct (m, n), anchors and weights per voice; voice 0 carries weight 1
    // (the highest frequency: weight = hz / max, decision D21).
    let weight = (VOICES - i) as f64 / VOICES as f64;
    Voice {
        planet_ref: PLANETS[i],
        mode_ref: format!("scene:planet/{}", PLANETS[i]),
        longitude_radians: 0.1 * i as f64,
        frequency_hz: 100.0 * weight,
        m: (i + 1) as u8,
        n: (VOICES - i) as u8,
        helix: "bimba",
        ql_position: 0,
        weight,
        phase_radians: phase,
        sounding_class: 0,
        class_error_cents: 0.0,
    }
}

fn synthetic(phase: f64) -> ShapeBasis {
    ShapeBasis {
        shape_ref: "test:synthetic-body".into(),
        address72: 0,
        voices: std::array::from_fn(|i| synthetic_voice(i, phase)),
    }
}

fn point(theta: f64, phi: f64, amplitude: [f64; 2]) -> StrikeAtInput {
    StrikeAtInput {
        theta,
        phi,
        amplitude,
    }
}

fn amplitudes(strikes: &[StrikeInput]) -> Vec<[f64; 2]> {
    strikes.iter().map(|s| s.amplitude).collect()
}

fn sky() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v5.json"
    ))
    .unwrap()
}

fn config() -> SceneConfig {
    let sky = sky();
    let request: ql_mef::scene::WorldRequest = serde_json::from_value(json!({
        "schema": ql_mef::scene::WORLD_REQUEST, "instance_ref": "test:strike-at",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-scene-instrument",
        "texture": [64, 64], "units_per_metre": 1.0, "sky": sky,
        "geometry": {"longitude_samples": 32, "latitude_samples": 16, "metres_per_unit": 1.0, "attachment": 1}
    }))
    .unwrap();
    let binding = ql_mef::scene::world(request).unwrap()["binding"].clone();
    serde_json::from_value(binding["host"].clone()).unwrap()
}

/// The real scene body the dated sky composes: nine voices with the Vimarśā
/// reading's own mode numbers, phases and the planets' longitudes.
fn scene_shape() -> (SceneConfig, ShapeBasis) {
    let config = config();
    let (_, basis) = complete(&config.basis, &config.material, MaterialFibre::Earth).unwrap();
    let shape = ShapeBasis::from_basis(&basis).unwrap();
    (config, shape)
}

#[test]
fn a_point_on_a_voices_node_excites_it_by_zero_and_its_antinode_by_the_full_shape() {
    // Phase 0 makes every voice a pure sin(mφ'/2)·sin(nθ/2) term, so its nodes
    // and antinodes are exact: φ' = φ − λ.
    let body = synthetic(0.0);
    let amplitude = [0.6, -0.3];
    for (i, voice) in body.voices.iter().enumerate() {
        let (m, n) = (f64::from(voice.m), f64::from(voice.n));
        // Node of voice i: on its own anchor meridian (φ = λ), off the equator.
        let node = point(PI / n, voice.longitude_radians, amplitude);
        let struck = body.strike_at(&[node]).unwrap();
        assert_eq!(struck.len(), VOICES);
        assert_eq!(
            struck[i].amplitude,
            [0.0, 0.0],
            "voice {i} is struck on its own node and must receive exactly nothing"
        );
        assert!(
            struck
                .iter()
                .enumerate()
                .any(|(j, s)| j != i && s.amplitude != [0.0, 0.0]),
            "a node of voice {i} must not silence the other voices"
        );
        // Antinode of voice i: sin(mφ'/2) = sin(nθ/2) = 1.
        let antinode = point(PI / n, voice.longitude_radians + PI / m, amplitude);
        let struck = body.strike_at(&[antinode]).unwrap();
        for (got, want) in struck[i]
            .amplitude
            .iter()
            .zip(amplitude.map(|a| a * voice.weight))
        {
            assert!(
                (got - want).abs() < 1e-12,
                "voice {i} at its antinode: {got} against the full shape {want}"
            );
        }
    }
    // The strongest voice (weight 1) is excited by exactly the full amplitude.
    let top = &body.voices[0];
    assert_eq!(top.weight, 1.0);
    let at = point(
        PI / f64::from(top.n),
        top.longitude_radians + PI / f64::from(top.m),
        amplitude,
    );
    let struck = body.strike_at(&[at]).unwrap();
    assert!((struck[0].amplitude[0] - amplitude[0]).abs() < 1e-12);
    assert!((struck[0].amplitude[1] - amplitude[1]).abs() < 1e-12);
}

#[test]
fn the_nine_voices_are_named_by_their_own_mode_references_in_scene_order() {
    let body = synthetic(0.7);
    let struck = body.strike_at(&[point(1.0, 2.0, [0.1, 0.0])]).unwrap();
    let names: Vec<&str> = struck.iter().map(|s| s.mode_ref.as_str()).collect();
    let want: Vec<String> = PLANETS
        .iter()
        .map(|planet| format!("scene:planet/{planet}"))
        .collect();
    assert_eq!(names, want.iter().map(String::as_str).collect::<Vec<_>>());
}

#[test]
fn the_coupling_is_the_shape_term_with_its_weight_and_nothing_else() {
    // A general phase and general points, against the independently written law.
    let body = synthetic(0.9);
    for (theta, phi) in [(0.3, 0.4), (1.7, 5.9), (4.4, 0.01), (6.2, 3.3)] {
        for amplitude in [[0.25, 0.0], [0.0, -0.5], [0.4, 0.2]] {
            let struck = body.strike_at(&[point(theta, phi, amplitude)]).unwrap();
            for (voice, strike) in body.voices.iter().zip(&struck) {
                let chi = oracle(voice, phi, theta);
                assert!((strike.amplitude[0] - amplitude[0] * chi).abs() < 1e-15);
                assert!((strike.amplitude[1] - amplitude[1] * chi).abs() < 1e-15);
            }
        }
    }
}

#[test]
fn the_coupling_agrees_with_the_mode_shapes_the_native_owner_carries() {
    // The body's own law, not a second one: striking a sampled point with unit
    // amplitude gives each voice the normal component of the very mode_shape
    // the native owner holds for that sample (shapes travel rounded to 1e-6).
    let (config, shape) = scene_shape();
    let geometry = &config.geometry;
    let samples = shape.samples(geometry).unwrap();
    let (lon, lat) = (
        usize::from(geometry.longitude_samples),
        usize::from(geometry.latitude_samples),
    );
    let mut checked = 0;
    for (i, j) in [(0, 0), (3, 2), (7, 5), (16, 8), (31, 15), (11, 13), (25, 1)] {
        let phi = TAU * i as f64 / lon as f64;
        let theta = TAU * j as f64 / lat as f64;
        let sample = &samples[j * lon + i];
        assert_eq!(sample.identity, (j * lon + i) as u64);
        let normal = [
            theta.cos() * phi.cos(),
            theta.cos() * phi.sin(),
            theta.sin(),
        ];
        let struck = shape.strike_at(&[point(theta, phi, [1.0, 0.0])]).unwrap();
        for (voice, (strike, mode_shape)) in struck.iter().zip(&sample.mode_shapes).enumerate() {
            let carried: f64 = mode_shape.iter().zip(normal).map(|(c, n)| c * n).sum();
            assert!(
                (strike.amplitude[0] - carried).abs() <= 1.0e-6,
                "sample ({i},{j}) voice {voice}: strike {} against carried shape {carried}",
                strike.amplitude[0]
            );
            assert_eq!(strike.amplitude[1], 0.0);
            checked += 1;
        }
    }
    assert_eq!(checked, 7 * VOICES);
}

#[test]
fn a_chord_of_two_points_is_the_sum_of_the_two_single_strikes() {
    let (_, shape) = scene_shape();
    let (a, b) = (point(0.8, 1.9, [0.2, 0.1]), point(2.4, 5.1, [0.3, -0.2]));
    let chord = amplitudes(&shape.strike_at(&[a, b]).unwrap());
    let (only_a, only_b) = (
        amplitudes(&shape.strike_at(&[a]).unwrap()),
        amplitudes(&shape.strike_at(&[b]).unwrap()),
    );
    assert_eq!(chord.len(), VOICES);
    for voice in 0..VOICES {
        assert_eq!(
            chord[voice],
            [
                only_a[voice][0] + only_b[voice][0],
                only_a[voice][1] + only_b[voice][1]
            ],
            "voice {voice}: a chord is the sum of its single strikes"
        );
    }
    // The order of the points inside one act does not matter to the body.
    let reversed = amplitudes(&shape.strike_at(&[b, a]).unwrap());
    for voice in 0..VOICES {
        for part in 0..2 {
            assert!((chord[voice][part] - reversed[voice][part]).abs() < 1e-15);
        }
    }
    // A repeated point is a doubled strike: the act is a sum, not a set.
    let twice = amplitudes(&shape.strike_at(&[a, a]).unwrap());
    for voice in 0..VOICES {
        for part in 0..2 {
            assert!((twice[voice][part] - 2.0 * only_a[voice][part]).abs() < 1e-15);
        }
    }
}

#[test]
fn a_full_chord_of_sixty_four_points_is_one_act() {
    let (_, shape) = scene_shape();
    let points: Vec<StrikeAtInput> = (0..MAX_STRIKE_POINTS)
        .map(|k| {
            point(
                TAU * (k as f64 + 0.5) / 97.0,
                TAU * (k as f64 + 0.25) / 89.0,
                [0.01, 0.0],
            )
        })
        .collect();
    let struck = shape.strike_at(&points).unwrap();
    assert_eq!(
        struck.len(),
        VOICES,
        "one strike per voice, however many points"
    );
    let mut by_hand = vec![[0.0f64; 2]; VOICES];
    for p in &points {
        let single = shape.strike_at(&[*p]).unwrap();
        for (sum, one) in by_hand.iter_mut().zip(&single) {
            sum[0] += one.amplitude[0];
            sum[1] += one.amplitude[1];
        }
    }
    assert_eq!(amplitudes(&struck), by_hand);
}

#[test]
fn a_refused_point_is_refused_in_words_and_the_whole_act_with_it() {
    let (_, shape) = scene_shape();
    let good = point(1.0, 2.0, [0.1, 0.0]);
    let refusals: Vec<(StrikeAtInput, &str)> = vec![
        (point(f64::NAN, 2.0, [0.1, 0.0]), "theta"),
        (point(f64::INFINITY, 2.0, [0.1, 0.0]), "theta"),
        (point(-0.1, 2.0, [0.1, 0.0]), "theta"),
        (point(TAU, 2.0, [0.1, 0.0]), "theta"),
        (point(7.0, 2.0, [0.1, 0.0]), "theta"),
        (point(1.0, f64::NAN, [0.1, 0.0]), "phi"),
        (point(1.0, f64::NEG_INFINITY, [0.1, 0.0]), "phi"),
        (point(1.0, -1e-9, [0.1, 0.0]), "phi"),
        (point(1.0, TAU, [0.1, 0.0]), "phi"),
        (point(1.0, 100.0, [0.1, 0.0]), "phi"),
        (point(1.0, 2.0, [f64::NAN, 0.0]), "amplitude"),
        (point(1.0, 2.0, [0.0, f64::INFINITY]), "amplitude"),
        (point(1.0, 2.0, [1.5, 0.0]), "amplitude"),
        (point(1.0, 2.0, [0.0, -1.5]), "amplitude"),
    ];
    for (bad, word) in refusals {
        // Alone, and as one point of an otherwise good chord: either way the
        // act is refused whole and returns no partial impulses.
        for act in [vec![bad], vec![good, bad], vec![bad, good]] {
            let error = shape.strike_at(&act).unwrap_err();
            assert!(
                error.contains(word),
                "refusal of {bad:?} should name {word}: {error}"
            );
        }
    }
    let empty = shape.strike_at(&[]).unwrap_err();
    assert!(
        empty.contains("1..=64") || empty.contains("1..64"),
        "{empty}"
    );
    let too_many = vec![good; MAX_STRIKE_POINTS + 1];
    let error = shape.strike_at(&too_many).unwrap_err();
    assert!(error.contains("64"), "{error}");
    // The domain edge: 0 is on the sheet, 2π is not.
    assert!(shape.strike_at(&[point(0.0, 0.0, [0.1, 0.0])]).is_ok());
    assert!(
        shape
            .strike_at(&[point(TAU - 1e-9, TAU - 1e-9, [0.1, 0.0])])
            .is_ok()
    );
}

#[test]
fn a_chord_may_not_sum_past_the_declared_material_policy() {
    // Each point alone is within the 1.0 metre policy; together they are not.
    let body = synthetic(0.0);
    let top = &body.voices[0];
    let at = point(
        PI / f64::from(top.n),
        top.longitude_radians + PI / f64::from(top.m),
        [0.9, 0.0],
    );
    assert!(body.strike_at(&[at]).is_ok());
    let error = body.strike_at(&[at, at]).unwrap_err();
    assert!(error.contains(&top.mode_ref), "{error}");
    assert!(error.contains("1.0"), "{error}");
}

#[test]
fn the_point_type_is_strict_about_its_fields() {
    let ok: StrikeAtInput =
        serde_json::from_value(json!({"theta": 1.0, "phi": 2.0, "amplitude": [0.1, 0.0]})).unwrap();
    assert_eq!(ok, point(1.0, 2.0, [0.1, 0.0]));
    for bad in [
        json!({"theta": 1.0, "phi": 2.0, "amplitude": [0.1, 0.0], "mode_ref": "x"}),
        json!({"theta": 1.0, "amplitude": [0.1, 0.0]}),
        json!({"theta": 1.0, "phi": 2.0}),
        json!({"theta": 1.0, "phi": 2.0, "amplitude": [0.1]}),
        json!({"theta": "1", "phi": 2.0, "amplitude": [0.1, 0.0]}),
    ] {
        assert!(
            serde_json::from_value::<StrikeAtInput>(bad.clone()).is_err(),
            "accepted {bad}"
        );
    }
}

#[test]
fn the_receipt_names_the_points_and_each_voices_coupling() {
    let (_, shape) = scene_shape();
    let act = [point(0.8, 1.9, [0.2, 0.1]), point(2.4, 5.1, [0.3, -0.2])];
    let impulses = shape.strike_at(&act).unwrap();
    let receipt = shape.strike_at_receipt(&act, &impulses);
    assert_eq!(receipt["schema"], STRIKE_AT_RECEIPT);
    assert_eq!(STRIKE_AT_RECEIPT, "ql.strike-at/v1");
    assert_eq!(
        receipt["points"],
        json!([
            {"theta": 0.8, "phi": 1.9, "amplitude": [0.2, 0.1]},
            {"theta": 2.4, "phi": 5.1, "amplitude": [0.3, -0.2]}
        ]),
        "the receipt names the struck points exactly as given"
    );
    let voices = receipt["voices"].as_array().unwrap();
    assert_eq!(voices.len(), VOICES);
    for (i, voice) in voices.iter().enumerate() {
        assert_eq!(voice["mode_ref"], impulses[i].mode_ref);
        assert_eq!(voice["planet_ref"], PLANETS[i]);
        assert_eq!(voice["amplitude"], json!(impulses[i].amplitude));
        let coupling = voice["coupling"].as_array().unwrap();
        assert_eq!(coupling.len(), act.len(), "one coupling per struck point");
        let rebuilt: [f64; 2] = {
            let c: Vec<f64> = coupling.iter().map(|v| v.as_f64().unwrap()).collect();
            [
                act[0].amplitude[0] * c[0] + act[1].amplitude[0] * c[1],
                act[0].amplitude[1] * c[0] + act[1].amplitude[1] * c[1],
            ]
        };
        assert_eq!(rebuilt, impulses[i].amplitude);
    }
    assert!(receipt["law"].as_str().unwrap().contains("χ"));
    assert!(
        receipt["standing"]
            .as_str()
            .unwrap()
            .contains("performance act")
    );
}
