//! K6's producer boundary for #154: map-inspectable, complete generations and
//! reproducible original inputs. These tests do not pretend to mount a surface,
//! run a physical solver, retrieve a Day/NOW, or execute an Epii Action.
use ql_mef::m_tree::native_m_registry;
use ql_mef::m2_condition::{CorrespondenceRole, correspondence_field};
use ql_mef::m2_engine::M2Request;
use serde_json::{Value, json};

fn request() -> Value {
    let mut v: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    let stamp = v["stamp"].clone();
    v["condition"]["palette"] = json!({
        "stamp": stamp, "policy_ref": "fixture:explicit-linear-palette",
        "entries": [
            {"name":"yellow", "linear_rgba":[0.8,0.6,0.1,1.0]},
            {"name":"silver", "linear_rgba":[0.7,0.7,0.7,1.0]},
            {"name":"red", "linear_rgba":[0.9,0.1,0.1,1.0]}
        ]
    });
    v["resonator"] = json!({
        "stamp": stamp, "provider_ref":"fixture:material-provider",
        "geometry_ref":"fixture:geometry", "material_ref":"fixture:material",
        "material_model_ref":"fixture:constitutive-model",
        "material_parameters": {
            "elastic_modulus":{"value":100.0,"unit":"GPa","source_ref":"fixture:material"}
        },
        "modes":[{
            "mode_ref":"fixture:earth-mode", "source_coordinate":"#2-2-2-5-5",
            "material_fibre":"earth", "carrier_weights":[{"carrier":0,"weight":1.0}],
            "frequency_hz":220.0,"amplitude":[1.0,0.5],"excitation":[0.2,0.0],
            "damping_per_second":0.01,"nodal_state_ref":"fixture:nodes",
            "antinodal_state_ref":"fixture:antinodes"
        }]
    });
    v
}

fn execute(v: &Value) -> Value {
    serde_json::to_value(
        M2Request::from_json(&v.to_string())
            .expect("valid producer request")
            .execute()
            .expect("producer execution"),
    )
    .unwrap()
}

fn restamp(v: &mut Value, generation: u64) {
    let mut identity = v["stamp"]["identity"].clone();
    identity["profile_generation"] = json!(generation);
    for pointer in [
        "/stamp/identity",
        "/vimarsha/stamp/identity",
        "/m1_excitation/stamp/identity",
        "/resonator/stamp/identity",
        "/condition/palette/stamp/identity",
    ] {
        if let Some(slot) = v.pointer_mut(pointer) {
            *slot = identity.clone();
        }
    }
}

#[test]
fn every_source_path_is_map_addressable_in_the_produced_condition() {
    let registry = native_m_registry();
    let field = correspondence_field();
    assert_eq!(field.rules.len(), 127);
    for (ordinal, rule) in field.rules.iter().enumerate() {
        let mut v = request();
        restamp(&mut v, 100 + ordinal as u64);
        v["condition"]["maqam_index"] = json!(rule.maqam_index);
        v["condition"]["role"] = serde_json::to_value(rule.role).unwrap();
        // A full M2 resonator may contain several elements. Here the supplied
        // fixture deliberately follows the selected path; no physical law is
        // inferred from that source correspondence.
        if let (Some(fibre), Some(tattva)) = (rule.material_fibre, &rule.tattva_coordinate) {
            v["resonator"]["modes"][0]["source_coordinate"] = json!(tattva);
            v["resonator"]["modes"][0]["material_fibre"] = serde_json::to_value(fibre).unwrap();
            v["resonator"]["modes"][0]["carrier_weights"][0]["carrier"] = json!(fibre.index() * 18);
        } else {
            v["resonator"] = Value::Null;
        }
        let observed_at = v["at_unix_ms"].as_u64().unwrap() - 7;
        v["world_observations"] = json!([{
            "planet_id":rule.planet_index,"longitude_degrees":12.5,
            "provider_ref":"fixture:sky","source_revision":"fixture:sky/v1",
            "observed_at_unix_ms":observed_at
        }]);
        let out = execute(&v);
        let c = &out["condition"];
        assert_eq!(c["identity"], out["identity"]);
        assert_eq!(c["source_path"], serde_json::to_value(rule).unwrap());
        assert_eq!(c["drive"], out["vimarsha"]["reading"]);
        assert_eq!(c["physical_material"], out["resonator"]);
        assert_eq!(c["world"]["observation"]["planet_id"], rule.planet_index);
        assert_eq!(c["world"]["age_ms"], 7);
        assert_eq!(c["colour"]["source_name"], json!(rule.colour_name));
        assert_eq!(
            c["colour"]["linear_rgba"].is_null(),
            rule.colour_name.is_none()
        );
        assert_eq!(c["colour"]["palette"]["stamp"]["identity"], c["identity"]);
        let pitches = c["musical"]["pitches_hz"].as_array().unwrap();
        assert_eq!(
            pitches.len(),
            if rule.spelled_steps24.is_some() { 8 } else { 0 }
        );
        for (reference, id) in [
            (&rule.maqam_coordinate, rule.maqam_node_id),
            (&rule.planet_coordinate, rule.planet_node_id),
            (&rule.chakra_coordinate, rule.chakra_node_id),
        ] {
            assert_eq!(registry.resolve(reference).unwrap().id, id);
        }
        for relation in rule
            .musical_relations
            .iter()
            .chain(&rule.planetary_relations)
        {
            let registered = registry
                .manifest()
                .relations
                .iter()
                .find(|r| r.id == relation.id)
                .unwrap();
            assert_eq!(
                registered.from_ref.as_deref(),
                Some(relation.from_coordinate.as_str())
            );
            assert_eq!(
                registered.to_ref.as_deref(),
                Some(relation.to_coordinate.as_str())
            );
        }
        // Source-path inspection addresses the distributed output in this same
        // enclosing frame, not an unrelated winning address or renderer LUT.
        let (_, pointer) = c["m3_form_potential_ref"]
            .as_str()
            .unwrap()
            .split_once('#')
            .unwrap();
        assert_eq!(out.pointer(pointer), Some(&out["modal"]["form_potential"]));
        assert_eq!(out["modal"]["form_potential"].as_array().unwrap().len(), 64);
    }
}

#[test]
fn every_missing_path_stays_missing_even_with_a_complete_palette() {
    let field = correspondence_field();
    let mut missing = 0;
    for index in 0..72 {
        for role in [CorrespondenceRole::Tonic, CorrespondenceRole::Dominant] {
            if field.rule(index, role).is_some() {
                continue;
            }
            missing += 1;
            let mut v = request();
            v["condition"]["maqam_index"] = json!(index);
            v["condition"]["role"] = serde_json::to_value(role).unwrap();
            let out = execute(&v);
            assert!(out["condition"]["source_path"].is_null());
            assert!(out["condition"]["colour"]["source_name"].is_null());
            assert!(out["condition"]["colour"]["linear_rgba"].is_null());
            assert_eq!(out["condition"]["musical"]["pitches_hz"], json!([]));
            assert!(!out["condition"]["gaps"].as_array().unwrap().is_empty());
        }
    }
    assert_eq!(missing, 17);
}

#[test]
fn new_generations_do_not_destroy_original_input_replay() {
    let mut original = request();
    restamp(&mut original, 40);
    let saved_request = original.to_string();
    let original_frame = execute(&original);
    let mut changed = original.clone();
    restamp(&mut changed, 41);
    changed["mef_conditions"] = json!([36]);
    changed["condition"]["active_mef_condition"] = json!(36);
    changed["vimarsha"]["lens"] = json!(6);
    changed["condition"]["tuning"] = json!("retained24_tet");
    let changed_frame = execute(&changed);
    let old = &original_frame["condition"];
    let new = &changed_frame["condition"];
    assert_eq!(old["identity"]["event_ref"], new["identity"]["event_ref"]);
    assert_ne!(old["identity"], new["identity"]);
    assert_ne!(old["drive"], new["drive"]);
    assert_ne!(old["musical"]["pitches_hz"], new["musical"]["pitches_hz"]);
    assert_eq!(old["source_path"], new["source_path"]);
    assert_eq!(
        new["identity"],
        new["physical_material"]["stamp"]["identity"]
    );
    assert_eq!(
        new["identity"],
        new["colour"]["palette"]["stamp"]["identity"]
    );
    assert_eq!(
        execute(&serde_json::from_str(&saved_request).unwrap()),
        original_frame
    );
    assert_eq!(
        original.to_string(),
        saved_request,
        "production does not mutate supplied inputs"
    );
}

#[test]
fn each_stamped_dependency_rejects_a_different_event_or_generation() {
    let v = request();
    for pointer in [
        "/vimarsha/stamp/identity",
        "/resonator/stamp/identity",
        "/condition/palette/stamp/identity",
    ] {
        for (key, value) in [
            ("event_ref", json!("fixture:another-event")),
            ("profile_generation", json!(99999)),
        ] {
            let mut broken = v.clone();
            broken.pointer_mut(pointer).unwrap()[key] = value;
            assert!(
                M2Request::from_json(&broken.to_string()).is_err(),
                "{pointer}/{key}"
            );
        }
    }
}
