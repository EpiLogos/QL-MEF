//! Native whole-event construction over an actual provider snapshot and the
//! compiled source. These assertions do not establish installed visual/audio
//! acceptance; the worker test consumes actual native targets below.
use ql_mef::scene::{self, WorldRequest};
use serde_json::{Value, json};

fn request() -> WorldRequest {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    serde_json::from_value(json!({"schema":scene::WORLD_REQUEST,
        "instance_ref":"expression:controlled-cosmic-world","event_ref":sky["snapshot_ref"],
        "subject_ref":"person:controlled-world-a", "texture":[64,64],"units_per_metre":1.0,
        "sky":sky,
        "start":{"tick12":3,"cycle":7,"aperture":9}}))
    .unwrap()
}

#[test]
fn one_native_constructor_joins_the_actual_event_sky_form_and_field() {
    let world = scene::world(request()).unwrap();
    if let Ok(directory) = std::env::var("QL_SCENE_WORLD_EXPORT_DIR") {
        let output = std::path::Path::new(&directory);
        std::fs::create_dir_all(output).unwrap();
        std::fs::write(
            output.join("controlled-native-world-a.json"),
            serde_json::to_vec_pretty(&world).unwrap(),
        )
        .unwrap();
        let mut second = request();
        second.subject_ref = "person:controlled-world-b".into();
        second.instance_ref = "expression:controlled-cosmic-world-b".into();
        let paired = scene::world(second).unwrap();
        assert_eq!(world["scene"]["bodies"], paired["scene"]["bodies"]);
        assert_eq!(world["scene"]["form"], paired["scene"]["form"]);
        assert_ne!(world["subject_ref"], paired["subject_ref"]);
        assert_ne!(world["instance_ref"], paired["instance_ref"]);
        std::fs::write(
            output.join("controlled-native-world-b.json"),
            serde_json::to_vec_pretty(&paired).unwrap(),
        )
        .unwrap();
    }
    assert_eq!(world["scene"]["form"]["codon"], 13); // independently retained C ring case(3,7)
    assert_eq!(world["scene"]["m1"]["root_hz"], json!(130.81279));
    assert_eq!(world["scene"]["bodies"].as_array().unwrap().len(), 10);
    assert_eq!(world["scene"]["centres"].as_array().unwrap().len(), 7);
    assert_eq!(world["event"]["m1"]["event_ref"], world["event_ref"]);
    assert_eq!(world["event"]["m3"]["subject_ref"], world["subject_ref"]);
    assert_eq!(world["scene"]["snapshot_ref"], world["snapshot_ref"]);
    assert_eq!(
        world["event"]["source_receipts"].as_array().unwrap(),
        &vec![world["sky"].clone()]
    );
    assert_ne!(
        world["sky"]["epoch_unix_ms"],
        world["sky"]["receipt_unix_ms"]
    );
    assert_eq!(
        world["event"]["m3"]["occurrence_unix_ms"],
        world["sky"]["epoch_unix_ms"]
    );
    assert_eq!(
        world["event"]["m3"]["receipt_unix_ms"],
        world["sky"]["receipt_unix_ms"]
    );
    assert_eq!(world["native_readback"]["m1_clock"]["degree360"], 90);
    assert_eq!(world["native_readback"]["m1_clock"]["degree720"], 450);
    assert_eq!(world["native_readback"]["m3_clock"]["degree720"], 450);
    assert_eq!(world["native_readback"]["selected_aperture"]["index"], 9);
    assert_eq!(
        world["native_readback"]["selected_aperture"]["division_deg10"],
        300
    );
    assert_eq!(
        world["native_readback"]["form"]["address"],
        world["scene"]["form"]["codon"]
    );
    assert!(world["current_form"]["triplet"].as_str().unwrap().len() == 3);
    assert!(
        world["current_form"]["hexagram_glyph"]
            .as_str()
            .unwrap()
            .chars()
            .count()
            == 1
    );
    assert_eq!(
        world["current_form"]["native_hinge"]["points"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    for (name, count) in [
        ("degree", 360),
        ("backbone", 24),
        ("decan", 36),
        ("codon", 64),
        ("skin", 72),
        ("aperture", 18),
    ] {
        let rows = world["registers"][name].as_array().unwrap();
        assert_eq!(rows.len(), count);
        let refs: std::collections::BTreeSet<_> = rows
            .iter()
            .map(|r| r["reading"]["ref"].as_str().unwrap())
            .collect();
        assert_eq!(refs.len(), count, "{name} must not duplicate source rows");
        assert!(
            rows.iter()
                .all(|r| !r["source_refs"].as_array().unwrap().is_empty())
        );
    }
    assert_eq!(
        world["native_readback"]["form_process"]["process_subject_ref"],
        "ql:scene-form:expression:controlled-cosmic-world"
    );
    assert_eq!(
        world["native_readback"]["form_process"]["triplet"],
        world["current_form"]["triplet"]
    );
    assert_eq!(
        world["native_readback"]["form_process"]["hexagram_glyph"],
        world["current_form"]["hexagram_glyph"]
    );
    assert_eq!(
        world["native_readback"]["form_process"]["current_reading"]["availability"],
        "available"
    );
    assert_eq!(
        world["native_readback"]["form_process"]["hexagram_reading"]["revision"],
        json!("907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288")
    );
    let mut held = request();
    held.start =
        serde_json::from_value(world["native_readback"]["continuation_start"].clone()).unwrap();
    let continued = scene::world(held).unwrap();
    assert_eq!(
        continued["native_readback"]["m1_clock"],
        world["native_readback"]["m1_clock"]
    );
    assert_eq!(
        continued["native_readback"]["m3_clock"],
        world["native_readback"]["m3_clock"]
    );
    assert_eq!(continued["current_form"], world["current_form"]);
    assert_eq!(
        continued["native_readback"]["continuous_clock"],
        world["native_readback"]["continuous_clock"]
    );
    assert_ne!(world["event"]["m1"]["event_ref"], "ql:k2/default-event");
    let voices = world["basis"]["derivation"]["sky_voices"]
        .as_array()
        .unwrap();
    assert_eq!(voices.len(), 9);
    assert_eq!(world["basis"], world["binding"]["native_basis"]);
    assert_eq!(world["event"], world["basis"]["input"]);
    assert_eq!(
        world["current_form"],
        world["native_readback"]["form_process"]
    );
}

#[test]
fn incomplete_sky_stale_registry_and_unbound_default_subject_fail() {
    let mut absent = request();
    absent.sky["bodies"].as_array_mut().unwrap().pop();
    assert!(scene::world(absent).unwrap_err().contains("ten"));
    let mut stale = request();
    stale.sky["source_binding"]["registry_revision"] = json!("unrelated-source");
    assert!(scene::world(stale).unwrap_err().contains("registry"));
    let mut wrong = request();
    wrong.subject_ref = "ql:k2/default-subject".into();
    assert!(scene::world(wrong).is_err());
    let mut disconnected = request();
    disconnected.event_ref = "event:unrelated-occasion".into();
    assert!(
        scene::world(disconnected)
            .unwrap_err()
            .contains("admitted sky snapshot")
    );
}

#[test]
fn cancer_third_decan_uses_the_accepted_moon_route_and_outer_anchors_are_qualified() {
    for longitude in [110.0, 115.0, 119.999999] {
        let route = ql_mef::m2::decan_planet_route(longitude).unwrap();
        assert_eq!(route.decan_coordinate, "#2-3-4-0-2");
        assert_eq!(route.planet_index, Some(1));
        assert!(route.source_conflicts.is_empty());
        let receiver = ql_mef::m2::planet_chakra_route(1).unwrap().unwrap();
        assert_eq!(receiver.chakra_coordinate, "#2-5-0/1-6");
    }
    assert!(ql_mef::m2::planet_chakra_route(7).unwrap().is_none());
    for (planet, centre) in [(8, 6), (9, 7)] {
        let route = ql_mef::m2::planet_chakra_route(planet).unwrap().unwrap();
        assert_eq!(route.chakra_index, centre);
        assert!(
            route
                .relations
                .iter()
                .all(|r| r.source_kind == "HAS_CHAKRAL_ANCHOR")
        );
    }
}

#[test]
#[ignore = "requires an actual QL_FIELD_WORKER native worker"]
fn native_m1_advance_moves_inscription_and_form_preserving_sky_pitch_and_aperture() {
    use ql_mef::continuous::scene_field::{SceneConfig, SceneInstrument};
    let world = scene::world(request()).unwrap();
    let config: SceneConfig = serde_json::from_value(world["binding"]["host"].clone()).unwrap();
    let worker =
        std::path::PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("actual native worker"));
    let mut owner =
        SceneInstrument::open(&worker, config, std::time::Duration::from_secs(20)).unwrap();
    let before = owner.influence();
    assert!(before["m3_generation"].is_u64());
    assert_eq!(
        before["m3_generation"],
        before["native_readback"]["m3_generation"]
    );
    let voices = owner.shape().voices;
    let before_field = owner.session_mut().read_field().unwrap();
    owner.m1_advance(1).unwrap();
    let after = owner.influence();
    assert_eq!(
        after["m3_generation"],
        after["native_readback"]["m3_generation"]
    );
    let after_field = owner.session_mut().read_field().unwrap();
    assert_eq!(after["native_readback"]["m1_clock"]["degree360"], 120);
    assert_eq!(after["native_readback"]["m3_clock"]["degree360"], 120);
    assert_eq!(
        after["native_readback"]["continuous_clock"]["inscription"]["half_degrees"],
        240
    );
    // Independent original vendor C ring expectation supplied by the source
    // reviewer: (tick3,cycle7)→13AGT; (tick4,cycle7)→14AGC.
    assert_eq!(before["native_readback"]["form"]["address"], 13);
    assert_eq!(after["native_readback"]["form"]["address"], 14);
    assert_eq!(before["native_readback"]["form_process"]["triplet"], "AGT");
    assert_eq!(after["native_readback"]["form_process"]["triplet"], "AGC");
    let first_segment =
        before["native_readback"]["form_process"]["native_hinge"]["segments"][0].clone();
    assert_eq!(
        first_segment,
        after["native_readback"]["form_process"]["native_hinge"]["segments"][0]
    );
    assert_ne!(
        before["native_readback"]["form_process"]["native_hinge"]["segments"][1],
        after["native_readback"]["form_process"]["native_hinge"]["segments"][1]
    );
    assert_eq!(
        before["native_readback"]["selected_aperture"]["index"],
        after["native_readback"]["selected_aperture"]["index"]
    );
    for (a, b) in voices.iter().zip(&owner.shape().voices) {
        assert_eq!(a.frequency_hz, b.frequency_hz);
        assert_eq!(a.longitude_radians, b.longitude_radians);
    }
    assert_eq!(
        before["native_readback"]["form_process"]["process_subject_ref"],
        after["native_readback"]["form_process"]["process_subject_ref"]
    );
    assert_ne!(
        before["native_readback"]["form_process"]["current_reading"],
        after["native_readback"]["form_process"]["current_reading"]
    );
    assert_eq!(
        after["native_readback"]["form_process"]["coordinate_ref"],
        after["native_readback"]["form"]["codon"]["ref"]
    );
    let inscription = after["native_readback"]["m3_clock"].clone();
    let form = after["native_readback"]["form"].clone();
    let mut changed = owner.event();
    changed.m3.aperture = 5;
    owner.replace(&changed, false).unwrap();
    let lens = owner.influence();
    let lens_field = owner.session_mut().read_field().unwrap();
    assert_eq!(lens["native_readback"]["selected_aperture"]["index"], 5);
    assert_eq!(lens["native_readback"]["m3_clock"], inscription);
    assert_eq!(lens["native_readback"]["form"], form);
    assert_eq!(
        lens["native_readback"]["continuous_clock"],
        after["native_readback"]["continuous_clock"]
    );
    for (a, b) in voices.iter().zip(&owner.shape().voices) {
        assert_eq!(a.frequency_hz, b.frequency_hz);
        assert_eq!(a.longitude_radians, b.longitude_radians);
    }
    assert_eq!(
        lens_field["targets"], after_field["targets"],
        "Action-only aperture selection must not change the actual native torus targets"
    );
    let mut reopen_request = request();
    reopen_request.start =
        serde_json::from_value(lens["native_readback"]["continuation_start"].clone()).unwrap();
    let reopened = scene::world(reopen_request).unwrap();
    assert_eq!(reopened["native_readback"]["m1_clock"]["degree360"], 120);
    assert_eq!(reopened["native_readback"]["m3_clock"]["degree360"], 120);
    assert_eq!(reopened["native_readback"]["selected_aperture"]["index"], 5);
    assert_eq!(
        reopened["native_readback"]["form"]["address"],
        lens["native_readback"]["form"]["address"]
    );
    assert_eq!(
        reopened["native_readback"]["continuous_clock"],
        lens["native_readback"]["continuous_clock"]
    );
    let max_move = before_field["targets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(after_field["targets"].as_array().unwrap())
        .map(|(a, b)| {
            a["position"]
                .as_array()
                .unwrap()
                .iter()
                .zip(b["position"].as_array().unwrap())
                .map(|(x, y)| (x.as_f64().unwrap() - y.as_f64().unwrap()).powi(2))
                .sum::<f64>()
                .sqrt()
        })
        .fold(0.0_f64, f64::max);
    assert!(
        max_move > 1e-8,
        "actual receiving native targets must change"
    );
    if let Ok(directory) = std::env::var("QL_SCENE_WORLD_RECEIPT_DIR") {
        let path = std::path::Path::new(&directory);
        std::fs::create_dir_all(path).unwrap();
        for (name, value) in [
            ("world.json", world),
            ("before-influence.json", before),
            ("after-m1-influence.json", after),
            ("after-aperture-influence.json", lens),
            ("before-field.json", before_field),
            ("after-m1-field.json", after_field),
            ("after-aperture-field.json", lens_field),
        ] {
            std::fs::write(path.join(name), serde_json::to_vec_pretty(&value).unwrap()).unwrap();
        }
        std::fs::write(path.join("causal-receipt.json"),serde_json::to_vec_pretty(&json!({"schema":"ql.scene-world-causal-proof/v1",
            "worker_path":worker,"rendering":"native K8 target/audio readback; no GPU visual/hardware audio claim",
            "m1_expected_delta_degrees":30,"selected_aperture_invariant":true,"sky_and_pitch_invariant":true,
            "actual_max_target_displacement_metres":max_move,"set_aperture_expected":"reading changes; clock/form/sky/pitch invariant"})).unwrap()).unwrap();
    }
}

#[test]
fn complete_source_registers_have_distinct_native_readings_and_qualified_sources() {
    let registers = scene::register_catalogue().unwrap();
    for (name, count) in [
        ("degree", 360),
        ("backbone", 24),
        ("decan", 36),
        ("codon", 64),
        ("skin", 72),
        ("aperture", 18),
    ] {
        let rows = registers[name].as_array().unwrap();
        assert_eq!(rows.len(), count);
        let refs: std::collections::BTreeSet<_> = rows
            .iter()
            .map(|r| r["reading"]["ref"].as_str().unwrap())
            .collect();
        assert_eq!(refs.len(), count, "{name}");
        for row in rows {
            for source in row["source_refs"].as_array().unwrap() {
                assert!(source["ref"].is_string());
                assert!(source["revision"].is_string());
                assert_eq!(source["availability"], "available");
            }
        }
    }
    assert_eq!(registers["degree"][90]["coordinate_ref"], "#3-5-5/0-90");
}
