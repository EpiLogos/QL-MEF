//! A form made the Torus's body (O-I #852; QL-MEF #288 packet P). A fold, a
//! codon or a hexagram of M3′, or a drawn glyph, is compiled through the form
//! owners' own types into the body K8 rings: its points are the form's
//! samples, held where the three hinges cross, heard at its free tip, and its
//! modes are read from its own creased lattice. The tuning sets where each
//! voice stands; the body sets the partials each voice rings with, their
//! shapes on the body, and how fast each rings out (π·f/Q).
//!
//! The pure tests need no worker. The last one opens the real Torus and the
//! fold body over the installed worker (`--ignored`, QL_FIELD_WORKER).
use std::f64::consts::{PI, TAU};
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::scene_field::{SceneConfig, SceneInstrument};
use ql_mef::continuous::{FieldSession, StrikeAtInput};
use ql_mef::form_instrument::body::{
    BODY_CONTRACT, BODY_MODEL, BODY_PREPARATION, BODY_REQUEST, BodyMaterial, BodyRequest,
    BodyVoice, CREASE_MOMENT, FormBody, FormKind, FormSymbol, MAX_BODY_SAMPLES, crease_coupling,
};
use ql_mef::form_samples::{
    FORM_SAMPLES_CONTRACT, MaskDeclaration, MaterialTreatment, SampleLayer, SamplePreparation,
    SampleUnits,
};
use ql_mef::m2_engine::MaterialFibre;
use serde_json::{Value, json};

fn sheet(resolution: u16, mask: Option<Vec<f64>>) -> SamplePreparation {
    SamplePreparation {
        schema: FORM_SAMPLES_CONTRACT.into(),
        prep_ref: "ta-onta:psg:fold-body".into(),
        treatment: if mask.is_some() {
            MaterialTreatment::GlyphMask
        } else {
            MaterialTreatment::SheetCarrier
        },
        layers: vec![SampleLayer {
            layer_ref: "form:sheet".into(),
            rest_depth_w: 0.5,
        }],
        resolution,
        units: SampleUnits {
            extent_units: 2.0,
            depth_units: 0.1,
            metres_per_unit: 0.25,
        },
        mask: mask.map(|coverages| MaskDeclaration {
            mask_ref: "app:rasteriser:form@1".into(),
            coverages,
        }),
    }
}

fn request(crease_deg10: [i32; 3]) -> BodyRequest {
    BodyRequest {
        schema: BODY_REQUEST.into(),
        event_ref: "event:controlled-form-body".into(),
        subject_ref: "person:controlled-scene-instrument".into(),
        source_generation: 7,
        form: FormSymbol {
            kind: FormKind::Fold,
            form_ref: "m3-prime:fold/valley-mountain-valley".into(),
            source_ref: "ql.psg-fold-progress/v1:controlled".into(),
        },
        samples: sheet(12, None),
        crease_deg10,
        material: BodyMaterial {
            fibre: MaterialFibre::Earth,
            q_factor: 40.0,
            strike_share: 0.7,
            source_ref: "declared".into(),
        },
        modes: 8,
        sample_rate: 48_000,
        pickup_uv: None,
    }
}

fn voices() -> Vec<BodyVoice> {
    [(220.0, 1.0), (330.0, 0.75)]
        .iter()
        .enumerate()
        .map(|(i, &(frequency_hz, weight))| BodyVoice {
            mode_ref: format!("controlled:voice/{i}"),
            source_coordinate: "#2-1".into(),
            frequency_hz,
            weight,
        })
        .collect()
}

fn ratios(body: &FormBody) -> Vec<f64> {
    body.modes().iter().map(|mode| mode.ratio).collect()
}

fn point(u: f64, v: f64, amplitude: f64) -> StrikeAtInput {
    // The same act on either body: (θ, φ) of the torus sheet is (v, u) of a
    // form's sheet, each over its whole turn.
    StrikeAtInput {
        theta: v * TAU,
        phi: u * TAU,
        amplitude: [amplitude, 0.0],
    }
}

#[test]
fn a_fold_rings_by_its_own_shape_and_a_turn_moves_only_its_picture() {
    let flat = FormBody::prepare(request([0, 0, 0])).unwrap();
    let folded = FormBody::prepare(request([900, 0, 0])).unwrap();
    let turned = FormBody::prepare(request([0, 0, 900])).unwrap();
    assert_eq!(flat.modes().len(), 8);
    assert_eq!(
        flat.modes()[0].ratio,
        1.0,
        "a voice stands on the body's first mode"
    );
    assert!(ratios(&flat).windows(2).all(|w| w[0] <= w[1]));
    // A crease changes how the halves move together, so the partials move.
    let moved = ratios(&flat)
        .iter()
        .zip(ratios(&folded))
        .map(|(a, b)| (a - b).abs() / a)
        .fold(0.0, f64::max);
    assert!(moved > 0.01, "a fold must ring by its own shape: {moved}");
    // A turn of the whole body is a pose: its partials stand, its points move.
    for (a, b) in ratios(&flat).iter().zip(ratios(&turned)) {
        assert!((a - b).abs() < 1e-9, "a turn moved a partial: {a} vs {b}");
    }
    assert_ne!(flat.points()[0].rest_metres, turned.points()[0].rest_metres);
    // Each preparation is versioned against its exact source.
    let refs = [&flat, &folded, &turned].map(|b| b.preparation_ref().to_owned());
    assert!(refs[0] != refs[1] && refs[1] != refs[2] && refs[0] != refs[2]);
    assert!(refs.iter().all(|r| r.starts_with("ql:form-body:")));
}

#[test]
fn the_body_is_held_where_the_hinges_cross_and_heard_at_its_free_tip() {
    let body = FormBody::prepare(request([900, -450, 0])).unwrap();
    let held: Vec<_> = body.points().iter().filter(|p| p.held).collect();
    // An even sheet's hinges cross between its four centre samples.
    assert_eq!(held.len(), 4);
    for p in &held {
        assert!((p.uv[0] - 0.5).abs() < 0.1 && (p.uv[1] - 0.5).abs() < 0.1);
    }
    let at = body.points().iter().position(|p| p.held).unwrap();
    for mode in body.modes() {
        assert_eq!(mode.shape[at], 0.0, "a held point does not move");
        let peak = mode.shape.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        assert!((peak - 1.0).abs() < 1e-12, "each mode's shape peaks at 1");
    }
    // Struck where it is held, the body takes nothing.
    let p = &body.points()[at];
    let silent = body
        .strike_at(&[point(p.uv[0], p.uv[1], 0.2)], &voices())
        .unwrap();
    assert!(silent.iter().all(|s| s.amplitude == [0.0, 0.0]));
    // Heard where it is freest: the point farthest from the hold.
    let pickup = body.pickup();
    let far = |q: &[f64; 2]| ((q[0] - 0.5).powi(2) + (q[1] - 0.5).powi(2)).sqrt();
    let best = body.points().iter().map(|p| far(&p.uv)).fold(0.0, f64::max);
    assert!((far(&body.points()[pickup].uv) - best).abs() < 1e-12);
}

/// What the pickup hears of a strike: each body mode's impulse times its shape
/// at the pickup, over every voice and mode, as one amplitude.
fn heard(body: &FormBody, at: StrikeAtInput) -> f64 {
    let n = body.modes().len();
    body.strike_at(&[at], &voices())
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, strike)| (strike.amplitude[0] * body.modes()[i % n].shape[body.pickup()]).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// DLW build ruling 2026-11-05 (O-I #852): a crease is a hinge line that passes
/// bending moment as well as translation, so a right-angle fold rings as one
/// body. A strike on the far side of the crease from the pickup is heard,
/// within an order of magnitude of a flat sheet struck at the same place.
#[test]
fn a_right_angle_crease_joins_its_faces_by_moment_so_a_strike_beyond_it_is_heard() {
    let at = StrikeAtInput {
        theta: 1.0,
        phi: 2.0,
        amplitude: [0.1, 0.0],
    };
    let flat = heard(&FormBody::prepare(request([0, 0, 0])).unwrap(), at);
    let folded = heard(&FormBody::prepare(request([900, -450, 0])).unwrap(), at);
    assert!(
        folded > 0.1 * flat,
        "a right-angle fold is heard across its crease: {folded} against the flat sheet's {flat}"
    );
}

/// The coupling across a crease edge is c = d + σ·m·(1 − |d|), d the faces'
/// normals' dot, σ the crease's sense (+1 mountain, −1 valley) and m the
/// moment it passes. It is the face's own 1 within a face, the fold's −1 when
/// folded flat back, σ·m at a right angle, and never beyond one, so the
/// lattice's stiffness stays diagonally dominant.
#[test]
fn the_coupling_across_a_crease_passes_its_moment_and_never_exceeds_one() {
    assert_eq!(CREASE_MOMENT, 0.5);
    for sigma in [1.0, -1.0] {
        assert_eq!(crease_coupling(1.0, sigma), 1.0);
        assert_eq!(crease_coupling(-1.0, sigma), -1.0);
        assert_eq!(crease_coupling(0.0, sigma), sigma * CREASE_MOMENT);
        for k in 0..=200 {
            let d = -1.0 + f64::from(k) / 100.0;
            let c = crease_coupling(d, sigma);
            assert!(c.abs() <= 1.0, "d {d}, σ {sigma}: c {c}");
        }
    }
    // Every fold the sites command prepares a body whose stiffness is positive: no mode below zero.
    for x in (-1800..=1800).step_by(900) {
        for y in (-1800..=1800).step_by(900) {
            let body = FormBody::prepare(request([x, y, 0])).unwrap();
            for mode in body.reading()["modes"].as_array().unwrap() {
                assert!(
                    mode["eigenvalue"].as_f64().unwrap() > 0.0,
                    "[{x}, {y}]: {mode}"
                );
            }
        }
    }
    assert_eq!(BODY_MODEL, "ql.form-body-creased-lattice/v2");
}

/// Sweeping one crease from flat to folded back moves the first partials
/// continuously: no jump where the faces stand at a right angle.
#[test]
fn sweeping_one_crease_moves_the_first_partials_continuously_through_a_right_angle() {
    let first = |deg10: i32| -> Vec<f64> {
        FormBody::prepare(request([deg10, 0, 0])).unwrap().reading()["modes"]
            .as_array()
            .unwrap()
            .iter()
            .take(4)
            .map(|mode| mode["eigenvalue"].as_f64().unwrap())
            .collect()
    };
    let sweep: Vec<Vec<f64>> = (0..=1800).step_by(100).map(first).collect();
    let mut moved = 0.0_f64;
    for pair in sweep.windows(2) {
        for (a, b) in pair[0].iter().zip(&pair[1]) {
            moved = moved.max((a - b).abs() / a);
        }
    }
    assert!(moved > 1e-3, "the crease moves the partials: {moved}");
    // Across a right angle, a tenth of a degree either side moves them by a hair.
    for (a, b) in first(899).iter().zip(first(901)) {
        assert!((a - b).abs() / a < 1e-2, "a jump at 90°: {a} to {b}");
    }
}

#[test]
fn a_strike_at_on_a_body_is_its_mode_shapes_at_the_struck_sample_summed_over_the_act() {
    let body = FormBody::prepare(request([900, 0, 0])).unwrap();
    let voices = voices();
    let a = point(0.1, 0.9, 0.2);
    let b = point(0.8, 0.3, -0.1);
    let one = body.strike_at(&[a], &voices).unwrap();
    assert_eq!(one.len(), voices.len() * body.modes().len());
    let at = body.sample_at(&a);
    let samples = body.samples(&voices, 0);
    for (v, voice) in voices.iter().enumerate() {
        for (k, mode) in body.modes().iter().enumerate() {
            let strike = &one[v * body.modes().len() + k];
            assert_eq!(strike.mode_ref, format!("{}/body/{k}", voice.mode_ref));
            let want = 0.2 * voice.weight * mode.shape[at];
            assert!((strike.amplitude[0] - want).abs() < 1e-12);
            // Exactly the term the native owner is handed as the sample's shape.
            let shape = samples[at].mode_shapes[v * body.modes().len() + k];
            let norm = shape.iter().map(|x| x * x).sum::<f64>().sqrt();
            assert!((norm - (voice.weight * mode.shape[at]).abs()).abs() < 2e-6);
        }
    }
    let chord = body.strike_at(&[a, b], &voices).unwrap();
    let two = body.strike_at(&[b], &voices).unwrap();
    for ((c, x), y) in chord.iter().zip(&one).zip(&two) {
        assert!((c.amplitude[0] - (x.amplitude[0] + y.amplitude[0])).abs() < 1e-12);
    }
    let over = point(0.1, 0.9, 1.0);
    let error = body.strike_at(&[over, over], &voices).unwrap_err();
    assert!(error.contains("metre material policy"), "{error}");
}

#[test]
fn the_resonator_rings_each_voice_through_the_bodys_modes_with_damping_by_q() {
    let body = FormBody::prepare(request([900, 0, 0])).unwrap();
    let voices = voices();
    let m2: ql_mef::m2_engine::M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-engine-request-v1.json"
    ))
    .unwrap();
    let resonator = body.resonator(m2.stamp.clone(), &voices).unwrap();
    assert_eq!(resonator.modes.len(), voices.len() * body.modes().len());
    for (v, voice) in voices.iter().enumerate() {
        for (k, mode) in body.modes().iter().enumerate() {
            let ringing = &resonator.modes[v * body.modes().len() + k];
            let hz = voice.frequency_hz * mode.ratio;
            assert!((ringing.frequency_hz - hz).abs() < 1e-9);
            assert!((ringing.damping_per_second - PI * hz / 40.0).abs() < 1e-9);
            assert_eq!(ringing.amplitude, [0.0, 0.0], "a body rests until struck");
        }
    }
    // The points are the form's own samples, at their creased places.
    let samples = body.samples(&voices, 0);
    assert_eq!(samples.len(), body.points().len());
    for (sample, p) in samples.iter().zip(body.points()) {
        assert_eq!(sample.identity, p.identity);
        assert_eq!(
            p.identity,
            p.sample_id & ((1 << 53) - 1),
            "named by its sample ID"
        );
        assert_eq!(sample.rest_metres, p.rest_metres);
    }
    let lifted = body
        .points()
        .iter()
        .filter(|p| p.rest_metres[2].abs() > 1e-9);
    assert!(lifted.count() > 0, "a valley fold lifts half the sheet");
    let gains = body.audio_gains(&voices, 2.0);
    let pickup = body.pickup();
    for (v, _) in voices.iter().enumerate() {
        for (k, mode) in body.modes().iter().enumerate() {
            assert_eq!(gains[v * body.modes().len() + k], 2.0 * mode.shape[pickup]);
        }
    }
}

#[test]
fn a_glyph_rings_by_where_it_is_drawn() {
    let whole = FormBody::prepare(request([0, 0, 0])).unwrap();
    // A glyph covering the sheet's left two thirds.
    let mask = (0..144)
        .map(|i| if i % 12 < 8 { 1.0 } else { 0.0 })
        .collect();
    let mut drawn = request([0, 0, 0]);
    drawn.form.kind = FormKind::Glyph;
    drawn.samples = sheet(12, Some(mask));
    let glyph = FormBody::prepare(drawn).unwrap();
    assert_eq!(
        glyph.points().len(),
        96,
        "only where the glyph is drawn rings"
    );
    assert!(glyph.points().iter().all(|p| p.uv[0] < 2.0 / 3.0));
    assert!(
        ratios(&whole)
            .iter()
            .zip(ratios(&glyph))
            .any(|(a, b)| (a - b).abs() / a > 0.01)
    );
}

#[test]
fn a_body_prepared_for_another_form_or_material_is_refused_as_stale() {
    let body = FormBody::prepare(request([900, 0, 0])).unwrap();
    body.admit(&request([900, 0, 0])).unwrap();
    let error = body.admit(&request([450, 0, 0])).unwrap_err();
    assert!(error.contains("prepared for another"), "{error}");
    let mut softer = request([900, 0, 0]);
    softer.material.q_factor = 4.5;
    assert!(body.admit(&softer).is_err());
}

#[test]
fn a_body_outside_its_bounds_is_refused_in_words() {
    let mut big = request([0, 0, 0]);
    big.samples = sheet(20, None);
    let error = FormBody::prepare(big).unwrap_err();
    assert!(error.contains(&MAX_BODY_SAMPLES.to_string()), "{error}");
    let mut none = request([0, 0, 0]);
    none.modes = 0;
    assert!(FormBody::prepare(none).unwrap_err().contains("modes"));
    let mut q = request([0, 0, 0]);
    q.material.q_factor = 0.0;
    assert!(FormBody::prepare(q).unwrap_err().contains("Q factor"));
    let mut empty = request([0, 0, 0]);
    empty.samples = sheet(12, Some(vec![0.0; 144]));
    assert!(FormBody::prepare(empty).unwrap_err().contains("no sample"));
    let mut schema = request([0, 0, 0]);
    schema.schema = "other".into();
    assert!(FormBody::prepare(schema).is_err());
}

#[test]
fn the_reading_is_the_prepared_body_a_performance_basis_keeps() {
    let body = FormBody::prepare(request([900, 0, 0])).unwrap();
    let reading = body.reading();
    assert_eq!(reading["schema"], BODY_PREPARATION);
    assert_eq!(reading["contract"], BODY_CONTRACT);
    assert_eq!(reading["event_ref"], "event:controlled-form-body");
    assert_eq!(reading["subject_ref"], "person:controlled-scene-instrument");
    assert_eq!(reading["source_generation"], 7);
    // Every path the kernel's PerformanceBasis reads of a prepared body.
    for path in [
        "/request/preparation_ref",
        "/request/state_ref",
        "/request/body_revision",
        "/request/geometry",
        "/request/material",
        "/request/exciter",
        "/request/pickup",
        "/request/sample_rate",
        "/form",
        "/clock",
        "/units",
    ] {
        assert!(reading.pointer(path).is_some(), "the reading lacks {path}");
    }
    assert_eq!(
        reading["request"]["preparation_ref"],
        body.preparation_ref()
    );
    // The form keeps its meaning as a symbol: its symbolic role stands apart.
    assert_eq!(
        reading["form"],
        json!({"kind":"fold","form_ref":"m3-prime:fold/valley-mountain-valley",
            "source_ref":"ql.psg-fold-progress/v1:controlled"})
    );
    assert_eq!(reading["modes"].as_array().unwrap().len(), 8);
}

// --- Over the installed worker -------------------------------------------

fn worker() -> PathBuf {
    PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker"))
}

fn config() -> SceneConfig {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v5.json"
    ))
    .unwrap();
    let request: ql_mef::scene::WorldRequest = serde_json::from_value(json!({
        "schema": ql_mef::scene::WORLD_REQUEST, "instance_ref": "test:form-body",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-scene-instrument",
        "texture": [64, 64], "units_per_metre": 1.0, "sky": sky,
        "geometry": {"longitude_samples": 32, "latitude_samples": 16, "metres_per_unit": 1.0, "attachment": 1},
        "torus": {"tuning": "sky"}
    }))
    .unwrap();
    let binding = ql_mef::scene::world(request).unwrap()["binding"].clone();
    serde_json::from_value(binding["host"].clone()).unwrap()
}

fn audio(field: &Value) -> Vec<f64> {
    field["audio"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

/// Power at `hz` over the block: one DFT bin under a Hann window, so a
/// neighbouring partial does not leak into the one read.
fn power(samples: &[f64], hz: f64) -> f64 {
    let (mut re, mut im) = (0.0, 0.0);
    let n = samples.len() as f64;
    for (i, x) in samples.iter().enumerate() {
        let hann = 0.5 - 0.5 * (TAU * i as f64 / n).cos();
        let w = 2.0 * PI * hz * i as f64 / 48_000.0;
        re += hann * x * w.cos();
        im -= hann * x * w.sin();
    }
    re * re + im * im
}

const BLOCK: u32 = 4096;

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_fold_body_rings_differently_from_the_torus_under_the_same_strike_and_its_picture_moves_from_the_same_state()
 {
    let config = config();
    let mut torus =
        SceneInstrument::open(&worker(), config.clone(), Duration::from_secs(20)).unwrap();
    // The fold rings the same nine voices the Torus sounds, under the same
    // event, subject, clock and pickup gain.
    let voices: Vec<BodyVoice> = torus
        .shape()
        .voices
        .iter()
        .map(|voice| BodyVoice {
            mode_ref: voice.mode_ref.clone(),
            source_coordinate: voice.planet_ref.into(),
            frequency_hz: voice.frequency_hz,
            weight: voice.weight,
        })
        .collect();
    let mut folded = request([900, -450, 0]);
    folded.event_ref = config.basis.m2.stamp.identity.event_ref.clone();
    let body = FormBody::prepare(folded).unwrap();
    let mut m2 = torus.session_mut().current_basis().m2_input.clone();
    m2.resonator = Some(body.resonator(m2.stamp.clone(), &voices).unwrap());
    let field = body
        .field_input(
            &config.field,
            &voices,
            0,
            config.material.audio_gain_per_metre,
        )
        .unwrap();
    let mut fold = FieldSession::open(&worker(), m2, field, Duration::from_secs(20)).unwrap();

    // The same strike: one act, one point of the sheet, one impulse.
    let act = point(0.15, 0.85, 0.05);
    torus.strike_at(&[act]).unwrap();
    let impulses = body.strike_at(&[act], &voices).unwrap();
    fold.strike(&impulses).unwrap();
    let (mut torus_blocks, mut fold_blocks) = (Vec::new(), Vec::new());
    let mut states = Vec::new();
    let mut last = Value::Null;
    for _ in 0..3 {
        torus_blocks.push(audio(
            &torus.session_mut().advance_field(BLOCK, false).unwrap(),
        ));
        last = fold.advance(BLOCK, false).unwrap();
        fold_blocks.push(audio(&last));
        states.push(last["amplitudes_metres"].clone());
    }
    assert_ne!(
        torus_blocks[0], fold_blocks[0],
        "the two bodies answer the strike alike"
    );

    // Impulse: the fold's own partials (above each voice) are struck, and the
    // Torus has no partial there.
    let resident: Vec<f64> = torus
        .shape()
        .resident_voices()
        .map(|v| v.frequency_hz)
        .collect();
    let n = body.modes().len();
    let ringing: Vec<f64> = (0..impulses.len())
        .map(|i| voices[i / n].frequency_hz * body.modes()[i % n].ratio)
        .collect();
    let clear = |hz: f64, others: Vec<f64>| {
        others
            .iter()
            .all(|r| (hz - r).abs() > (0.06 * hz).max(40.0))
    };
    let partial = impulses
        .iter()
        .enumerate()
        .filter(|(i, s)| i % n > 0 && s.amplitude[0].abs() > 1e-4)
        .map(|(i, _)| (i, ringing[i]))
        .filter(|&(_, hz)| hz < 600.0 && clear(hz, resident.clone()))
        .map(|(_, hz)| hz)
        .fold(f64::INFINITY, f64::min);
    assert!(
        partial.is_finite(),
        "no struck fold partial stands clear of the Torus's voices"
    );
    let fold_power = power(&fold_blocks[0], partial);
    let torus_power = power(&torus_blocks[0], partial);
    // What the Torus has there is only the skirt of its own voices.
    let voice_power = resident
        .iter()
        .map(|hz| power(&torus_blocks[0], *hz))
        .fold(0.0, f64::max);
    assert!(
        torus_power < 0.01 * voice_power && fold_power > 10.0 * torus_power,
        "the fold's partial at {partial:.1} Hz: fold {fold_power:e}, torus {torus_power:e} (its voices {voice_power:e})"
    );
    let dt = f64::from(BLOCK) / 48_000.0;
    // The state it is heard from rings down mode by mode at π·f/Q exactly.
    let magnitude = |state: &Value, i: usize| {
        let a = &state[i];
        a[0].as_f64().unwrap().hypot(a[1].as_f64().unwrap())
    };
    let mut checked = 0;
    for (i, hz) in ringing.iter().enumerate() {
        let (a0, a1) = (magnitude(&states[0], i), magnitude(&states[1], i));
        if a0 < 1e-9 {
            continue;
        }
        let want = -PI * hz / 40.0 * dt;
        assert!(
            ((a1 / a0).ln() - want).abs() < 1e-3 * want.abs().max(1.0),
            "mode {i} at {hz:.1} Hz rang down by {:.5}, its Q gives {want:.5}",
            (a1 / a0).ln()
        );
        checked += 1;
    }
    assert!(
        checked > n,
        "the strike rang more than one voice's modes: {checked}"
    );

    // Ring-down, against the Torus: its voices ring out at one rate, the
    // material's; the fold's partials each at their own, faster as they rise.
    let rates = |states: &[Value], count: usize| -> Vec<f64> {
        (0..count)
            .filter(|&i| magnitude(&states[0], i) > 1e-9)
            .map(|i| (magnitude(&states[1], i) / magnitude(&states[0], i)).ln() / dt)
            .collect()
    };
    let mut torus_states = Vec::new();
    for _ in 0..2 {
        let field = torus.session_mut().advance_field(BLOCK, false).unwrap();
        torus_states.push(field["amplitudes_metres"].clone());
    }
    let torus_rates = rates(&torus_states, voices.len());
    let fold_rates = rates(&states, ringing.len());
    let spread = |r: &[f64]| {
        let (lo, hi) = r
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
                (lo.min(*x), hi.max(*x))
            });
        hi - lo
    };
    assert!(
        torus_rates.len() > 1 && spread(&torus_rates) < 1e-6,
        "the Torus's voices ring out alike: {torus_rates:?}"
    );
    assert!(
        spread(&fold_rates) > 10.0,
        "the fold's partials ring out by their own frequencies: {fold_rates:?}"
    );
    let energy = |block: &[f64]| block.iter().map(|x| x * x).sum::<f64>();
    let fold_fall = (energy(&fold_blocks[1]) / energy(&fold_blocks[0])).ln();
    let torus_fall = (energy(&torus_blocks[1]) / energy(&torus_blocks[0])).ln();
    assert!(
        (fold_fall - torus_fall).abs() > 0.5,
        "heard, the two ring out alike: fold {fold_fall:.3}, torus {torus_fall:.3}"
    );

    // The picture moves from the same state: each target is its rest point
    // displaced by the modal amplitudes the audio was read from.
    let at = body.sample_at(&act);
    let samples = body.samples(&voices, 0);
    let amplitudes: Vec<f64> = last["amplitudes_metres"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a[0].as_f64().unwrap())
        .collect();
    let target = last["targets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| {
            t["identity"] == json!(samples[at].identity)
                || t["identity"] == json!(samples[at].identity.to_string())
        })
        .expect("the struck sample is drawn");
    for axis in 0..3 {
        let want = samples[at].rest_metres[axis]
            + amplitudes
                .iter()
                .zip(&samples[at].mode_shapes)
                .map(|(a, s)| a * s[axis])
                .sum::<f64>();
        let got = target["position"][axis].as_f64().unwrap();
        assert!(
            (got - want).abs() < 1e-5,
            "axis {axis}: drawn {got}, state {want}"
        );
    }
}
