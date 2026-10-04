use ql_mef::continuous::performance::*;
use serde_json::json;
mod support {
    #[path = "retained_source_performance.rs"]
    pub mod retained_source_performance;
}
#[test]
fn actual_original_config_and_canonical_twelve_node_source_replay_exactly() {
    let (current, config) = support::retained_source_performance::config(false);
    let owner =
        PerformanceOwner::prepare(&current, "expression:retained/current", config.clone()).unwrap();
    let request = owner.binding().physical_body().request();
    assert_eq!(request.geometry.nodes.len(), 12);
    assert_eq!(request.geometry.edges.len(), 34);
    assert!(owner.binding().source_form_recipe().is_some());
    assert_eq!(
        owner.source_assets()["original_native_input"],
        serde_json::to_value(&current.input).unwrap()
    );
    assert_eq!(
        owner.source_assets()["configuration"],
        serde_json::to_value(&config).unwrap()
    );
    assert_eq!(
        owner.source_assets()["consumer_roles"]["personal_nine_force_routes"]["available"],
        false
    );
    let replay = PerformanceOwner::prepare(
        &current,
        "expression:retained/current",
        serde_json::from_value(owner.source_assets()["configuration"].clone()).unwrap(),
    )
    .unwrap();
    assert_eq!(replay.source_assets(), owner.source_assets());
    assert_eq!(
        replay.native_packet().unwrap(),
        owner.native_packet().unwrap()
    );
    let mut stale = config;
    stale.controls.expected_m3_generation += 1;
    assert!(PerformanceOwner::prepare(&current, "expression:retained/current", stale).is_err());
}
#[test]
fn actual_seven_source_keys_are_sparse_targets_without_rewriting_m2_or_geometry() {
    let (current, full_config) = support::retained_source_performance::config(false);
    let (_, sparse_config) = support::retained_source_performance::config(true);
    let full =
        PerformanceOwner::prepare(&current, "expression:retained/current", full_config).unwrap();
    let sparse =
        PerformanceOwner::prepare(&current, "expression:retained/current", sparse_config).unwrap();
    let packet = sparse.native_packet().unwrap();
    assert!(packet["source_key_admission"]["preparation"].is_object());
    assert_eq!(
        packet["determination"]["audio_octet_hz"],
        full.binding().determination()["audio_octet_hz"]
    );
    assert_eq!(
        packet["determination"]["nodal_quartet"],
        full.binding().determination()["nodal_quartet"]
    );
    assert_eq!(
        packet["physical_body"],
        full.native_packet().unwrap()["physical_body"]
    );
    assert_eq!(packet["notes"].as_array().unwrap().len(), 7);
    assert!(packet["notes"][0]["hertz"].as_f64().unwrap() > 0.0);
    let cells = sparse.native_catalog();
    assert_eq!(cells.len(), 36);
    assert_eq!(cells.iter().filter(|c| c["available"] == true).count(), 21);
    for key in 0..12u64 {
        let copies = cells.iter().filter(|c| c["key"] == key).collect::<Vec<_>>();
        assert_eq!(copies.len(), 3);
        let assigned = [0, 2, 4, 5, 7, 9, 11].contains(&key);
        for cell in copies {
            assert_eq!(cell["available"], assigned);
            if assigned {
                let note = &cell["native_target"];
                assert!(note["hertz"].as_f64().unwrap() > 0.0);
                assert!(cell["source_degree"].as_u64().unwrap() < 7);
                assert_eq!(note["key"], cell["key"]);
                assert_eq!(note["register_octave"], cell["register_octave"]);
                assert_eq!(note["identity"], packet["determination"]["identity"]);
                assert!(cell["reason"].is_null());
            } else {
                assert!(cell["native_target"].is_null());
                assert!(cell["source_degree"].is_null());
                assert!(!cell["reason"].as_str().unwrap().is_empty());
                assert!(cell.get("hertz").is_none());
            }
            assert!(!cell["source_collection"].as_str().unwrap().is_empty());
            assert!(!cell["source_receipt"].as_str().unwrap().is_empty());
        }
    }
}
#[test]
fn command_deserialization_cannot_smuggle_targets_body_clock_or_unknown_fields() {
    for command in [
        json!({"operation":"performance-gesture","phase":"press","input_ref":"input:x","row":0,"column":0,"velocity":1.0,"native_target":{}}),
        json!({"operation":"performance-inspect","sample":48000}),
        json!({"operation":"performance-device-open","device_id":0,"sample_rate":48000,"buffer_frames":128,"body":{}}),
    ] {
        assert!(serde_json::from_value::<PerformanceCommand>(command).is_err());
    }
}

#[test]
fn actual_sparse_return_retains_all_available_targets_from_same_source_binding() {
    use ql_mef::musical_performance_return::{
        ReturnContext, ReturnReference, bind_performance_return,
    };
    let (current, config) = support::retained_source_performance::config(true);
    let owner = PerformanceOwner::prepare(&current, "expression:retained/current", config).unwrap();
    let reference = |value: &str| ReturnReference {
        reference: value.into(),
        revision: "1".into(),
    };
    let returned = bind_performance_return(
        owner.binding(),
        None,
        ReturnContext {
            context: reference("controlled:source/world"),
            receiver: reference("controlled:source/receiver"),
            kind: "world".into(),
            private: false,
            source_occasion: None,
            protected_state: None,
            consent: None,
            required_assets: vec![],
        },
        73,
    )
    .unwrap();
    let packet = owner.native_packet().unwrap();
    let pitches = returned.expression_pitches(0).unwrap();
    assert_eq!(pitches.as_array().unwrap().len(), 7);
    assert_eq!(packet["notes"].as_array().unwrap().len(), 7);
    for (pitch, native) in pitches
        .as_array()
        .unwrap()
        .iter()
        .zip(packet["notes"].as_array().unwrap())
    {
        assert_eq!(pitch["hertz"], native["hertz"]);
        assert_eq!(pitch["tuning_ref"], native["tuning_ref"]);
        assert_eq!(pitch["fundamental_hz"], native["fundamental_hz"]);
    }
    assert_eq!(
        serde_json::to_value(owner.binding()).unwrap()["determination"],
        packet["determination"]
    );
    assert_eq!(
        serde_json::to_value(owner.binding()).unwrap()["physical_body"],
        packet["physical_body"]
    );
    assert_eq!(
        returned.expression_basis().unwrap()["identity"]["event_ref"],
        packet["determination"]["identity"]["event"]
    );
}
