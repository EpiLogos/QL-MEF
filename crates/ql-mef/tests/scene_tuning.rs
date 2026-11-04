//! The scene field's tuning choice (O-I #850; DLW manual §21.2 "Tuning"):
//! Logic sounds Vimarśā's octet and its four boundaries shape the body as
//! boundaries, never as voices; Maqām sounds the M2 condition's maqām; Sky the
//! dated sky's planets. Every voice is a ratio over the Fundamental. Read on
//! the composed event alone: no worker is needed.
use ql_mef::continuous::StrikeAtInput;
use ql_mef::continuous::scene_field::{
    BINDING_REQUEST, BindingRequest, ShapeBasis, Tuning, binding, complete, default_event,
    default_material,
};
use ql_mef::m2_engine::MaterialFibre;

fn shape(tuning: Tuning, fundamental_hz: Option<f64>) -> (ShapeBasis, serde_json::Value) {
    let mut event = default_event().unwrap();
    event.fundamental_hz = fundamental_hz;
    let (_, basis) = complete(&event, &default_material(), MaterialFibre::Earth, tuning).unwrap();
    let m2 = basis.m2.clone();
    (ShapeBasis::from_basis(&basis).unwrap(), m2)
}

#[test]
fn logic_sounds_vimarshas_octet_and_its_four_boundaries_are_never_voices() {
    let (shape, m2) = shape(Tuning::Logic, None);
    let octet: Vec<f64> = m2["vimarsha"]["reading"]["audio_octet_hz"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect();
    let logic: Vec<_> = shape.sounding(Tuning::Logic).collect();
    assert_eq!(logic.len(), 8, "eight voices, never seven or nine");
    for (voice, hz) in logic.iter().zip(&octet) {
        assert!(
            (voice.frequency_hz - hz).abs() < 1e-3,
            "{} vs {hz}",
            voice.frequency_hz
        );
        assert_eq!(voice.tuning, Tuning::Logic);
    }
    // The four boundaries are the nodal quartet: they reach the body as the
    // voices' nodal numbers and carry no mode of their own.
    assert_eq!(shape.boundaries.len(), 4);
    let quartet = m2["vimarsha"]["reading"]["nodal_quartet"]
        .as_array()
        .unwrap();
    for (boundary, node) in shape.boundaries.iter().zip(quartet) {
        assert_eq!(u64::from(boundary.m), node["m"].as_u64().unwrap());
        assert_eq!(u64::from(boundary.n), node["n"].as_u64().unwrap());
    }
    assert!(
        shape
            .voices
            .iter()
            .all(|voice| !voice.mode_ref.contains("boundary"))
    );
}

#[test]
fn a_strike_sounds_only_the_voices_of_the_tuning_in_force() {
    let (shape, _) = shape(Tuning::Logic, None);
    let point = StrikeAtInput {
        theta: 0.7,
        phi: 1.3,
        amplitude: [0.1, 0.0],
    };
    let impulses = shape.strike_at_tuned(&[point], Tuning::Logic).unwrap();
    assert_eq!(impulses.len(), 8);
    assert!(
        impulses
            .iter()
            .all(|impulse| impulse.mode_ref.starts_with("scene:logic/"))
    );
    let sky = shape.strike_at_tuned(&[point], Tuning::Sky).unwrap();
    assert_eq!(sky.len(), 9);
    assert!(sky.iter().all(|i| i.mode_ref.starts_with("scene:planet/")));
}

#[test]
fn switching_tuning_changes_the_voice_frequencies_the_owner_reports() {
    let (logic, _) = shape(Tuning::Logic, None);
    let report = |tuning| -> Vec<f64> {
        logic
            .sounding(tuning)
            .map(|voice| voice.frequency_hz)
            .collect()
    };
    assert_ne!(report(Tuning::Logic), report(Tuning::Sky));
    if logic.resident(Tuning::Maqam) {
        assert_eq!(report(Tuning::Maqam).len(), 8);
        assert_ne!(report(Tuning::Maqam), report(Tuning::Logic));
    }
}

#[test]
fn every_voice_is_a_ratio_over_the_fundamental() {
    let (map, _) = shape(Tuning::Sky, None);
    // The map's 1:1 is the Sun's voice.
    let sun = map
        .voices
        .iter()
        .find(|voice| voice.planet_ref == "#2-5-0/1")
        .unwrap()
        .frequency_hz;
    let (tuned, _) = shape(Tuning::Sky, Some(2.0 * sun));
    let every = |shape: &ShapeBasis| -> Vec<_> {
        Tuning::ALL
            .iter()
            .flat_map(|tuning| shape.sounding(*tuning).cloned().collect::<Vec<_>>())
            .collect()
    };
    assert!(
        every(&map).len() >= 17,
        "the sky's nine and the octet's eight at least"
    );
    for (a, b) in every(&map).iter().zip(&every(&tuned)) {
        assert_eq!(a.mode_ref, b.mode_ref);
        assert!(
            (b.frequency_hz - 2.0 * a.frequency_hz).abs() < 1e-6,
            "{}: {} vs {}",
            a.mode_ref,
            b.frequency_hz,
            a.frequency_hz
        );
    }
    let mut event = default_event().unwrap();
    event.fundamental_hz = Some(19.0);
    assert!(event.compose().is_err());
}

#[test]
fn an_event_whose_condition_admits_its_maqam_holds_the_maqam_resident() {
    // The default scene is a v1 event, whose composer admits no condition
    // bus: there Maqām is not resident and is refused by name.
    let (v1, _) = shape(Tuning::Sky, None);
    assert!(!v1.resident(Tuning::Maqam));
    let mut event = default_event().unwrap();
    assert!(
        complete(
            &event,
            &default_material(),
            MaterialFibre::Earth,
            Tuning::Maqam
        )
        .unwrap_err()
        .contains("maqam")
    );
    // The same event under v2, with a tuning the map supplies pitches for,
    // admits the condition's eight pitches.
    event.schema = ql_mef::continuous::coupled::REQUEST_V2.into();
    event.m2.condition.as_mut().unwrap().tuning = ql_mef::m2_condition::TuningPolicy::Retained24Tet;
    let (_, basis) = complete(
        &event,
        &default_material(),
        MaterialFibre::Earth,
        Tuning::Maqam,
    )
    .unwrap();
    let pitches: Vec<f64> = basis.m2["condition"]["musical"]["pitches_hz"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect();
    let shape = ShapeBasis::from_basis(&basis).unwrap();
    let maqam: Vec<f64> = shape
        .sounding(Tuning::Maqam)
        .map(|v| v.frequency_hz)
        .collect();
    assert_eq!(maqam.len(), 8);
    for (voice, hz) in maqam.iter().zip(&pitches) {
        assert!((voice - hz).abs() < 1e-9, "{voice} vs {hz}");
    }
}

/// The opening excitation of each resident group in the binding's composed body.
fn opening(request: serde_json::Value) -> (serde_json::Value, Vec<(String, f64)>) {
    let mut asked = serde_json::json!({"schema": BINDING_REQUEST, "instance_ref": "test:open",
        "texture": [8, 8], "units_per_metre": 1.0});
    for (key, value) in request.as_object().unwrap() {
        asked[key] = value.clone();
    }
    let reply = binding(serde_json::from_value::<BindingRequest>(asked).unwrap()).unwrap();
    let modes = reply["native_basis"]["m2"]["resonator"]["modes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|mode| {
            (
                mode["mode_ref"].as_str().unwrap().to_owned(),
                mode["amplitude"][0].as_f64().unwrap(),
            )
        })
        .collect();
    (reply["host"].clone(), modes)
}

#[test]
fn a_new_torus_opens_under_logic_and_a_saved_sound_opens_with_its_own_tuning() {
    // Manual §21.2 Parameters: Tuning "Logic · Maqām · Sky", default Logic; §21.2 Rules: "A saved
    // sound keeps every parameter, its body, and its tuning … It opens with the same sound."
    let (host, modes) = opening(serde_json::json!({}));
    assert!(
        host.get("torus").is_none(),
        "a new sound names no saved settings"
    );
    let excited = |group: &str| {
        modes
            .iter()
            .filter(|(mode, _)| mode.starts_with(group))
            .map(|(_, a)| *a)
            .collect::<Vec<_>>()
    };
    let strike = default_material().strike_metres;
    assert_eq!(
        excited("scene:logic/"),
        vec![strike; 8],
        "the octet rings at open"
    );
    assert!(
        excited("scene:planet/").iter().all(|a| *a == 0.0),
        "the sky rests"
    );
    let (host, modes) = opening(serde_json::json!({"torus": {"tuning": "sky", "carrier": 0.25}}));
    assert_eq!(host["torus"]["tuning"], "sky");
    assert_eq!(host["torus"]["carrier"], 0.25);
    let excited = |group: &str| {
        modes
            .iter()
            .filter(|(mode, _)| mode.starts_with(group))
            .map(|(_, a)| *a)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        excited("scene:planet/"),
        vec![strike; 9],
        "the saved sound's sky rings"
    );
    assert!(excited("scene:logic/").iter().all(|a| *a == 0.0));
    // A saved sound's settings are the session's own, refused in its words.
    let asked = serde_json::json!({"schema": BINDING_REQUEST, "instance_ref": "test:open",
        "texture": [8, 8], "units_per_metre": 1.0, "torus": {"polyphony": 40}});
    let refused = binding(serde_json::from_value::<BindingRequest>(asked).unwrap()).unwrap_err();
    assert!(refused.contains("Polyphony"), "{refused}");
}

#[test]
fn each_class_sounds_the_voices_of_the_tuning_in_force_by_their_shape_at_its_point() {
    // O-I #851: what each of the twelve classes sounds now is what a key of it struck now gives each
    // voice of the tuning in force: the voice's shape at the class's moving point (strike_at_tuned).
    for (tuning, count, prefix) in [
        (Tuning::Logic, 8, "scene:logic/"),
        (Tuning::Sky, 9, "scene:planet/"),
    ] {
        let (shape, _) = shape(tuning, None);
        let steps = 3.25;
        let reading = shape.class_sounds(tuning, steps).unwrap();
        assert_eq!(reading["tuning"], serde_json::json!(tuning));
        assert_eq!(reading["steps"], serde_json::json!(steps));
        let classes = reading["by_class"].as_array().unwrap();
        assert_eq!(classes.len(), 12);
        let surface = ql_mef::JankoSurface::new();
        for (class, expected) in classes.iter().zip(surface.class_points(steps)) {
            assert_eq!(class["pitch_class"], expected.pitch_class);
            let point = &class["point"];
            assert_eq!(point["arc"], expected.point.arc);
            let sounds = class["sounds"].as_array().unwrap();
            assert_eq!(sounds.len(), count);
            let struck = shape
                .strike_at_tuned(
                    &[StrikeAtInput {
                        theta: expected.point.theta,
                        phi: expected.point.phi,
                        amplitude: [1.0, 0.0],
                    }],
                    tuning,
                )
                .unwrap();
            for (sound, (impulse, voice)) in
                sounds.iter().zip(struck.iter().zip(shape.sounding(tuning)))
            {
                assert!(sound["mode_ref"].as_str().unwrap().starts_with(prefix));
                assert_eq!(sound["mode_ref"], impulse.mode_ref.as_str());
                assert_eq!(sound["frequency_hz"], voice.frequency_hz);
                let coupling = sound["coupling"].as_f64().unwrap();
                assert!((coupling - impulse.amplitude[0]).abs() < 1e-12);
            }
        }
        assert!(reading["law"].as_str().unwrap().contains("tuning in force"));
    }
}
