use ql_mef::m2_condition::*;
use ql_mef::m2_engine::M2Request;
use serde_json::{Value, json};
use std::{fs, path::PathBuf, process::Command};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn request() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap()
}
fn execute(v: Value) -> Value {
    serde_json::to_value(
        M2Request::from_json(&v.to_string())
            .unwrap()
            .execute()
            .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_native_correspondence_and_pitch_parity() {
    let dir = root().join("target/m2-receipt");
    fs::create_dir_all(&dir).unwrap();
    let exe = dir.join("condition-probe");
    let output = Command::new("cc")
        .current_dir(root())
        .args([
            "-std=c11",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-Ic/include",
            "scripts/m2-condition-probe.c",
            "c/src/m2.c",
            "c/src/m_tree.c",
            "-lm",
            "-o",
        ])
        .arg(&exe)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    fs::write(dir.join("condition-native.tsv"), &text).unwrap();
    let field = correspondence_field();
    let mut rules = 0;
    let mut pitches = 0;
    let mut unavailable = 0;
    for line in text.lines() {
        let p: Vec<_> = line.split('\t').collect();
        let index = p[1].parse().unwrap();
        let role = if p[2] == "0" {
            CorrespondenceRole::Tonic
        } else {
            CorrespondenceRole::Dominant
        };
        match p[0] {
            "rule" => {
                rules += 1;
                let r = field.rule(index, role).unwrap();
                let ids = [
                    Some(r.maqam_node_id),
                    Some(r.planet_node_id),
                    Some(r.chakra_node_id),
                    r.tattva_node_id,
                    Some(r.musical_relations[0].id),
                    Some(r.planetary_relations[0].id),
                ];
                for (i, id) in ids.into_iter().enumerate() {
                    assert_eq!(
                        p[i + 3],
                        format!("{:016x}", id.map(|x| x.as_u64()).unwrap_or(0))
                    );
                }
                assert_eq!(p[9], r.planet_index.to_string());
                assert_eq!(p[10], r.chakra_index.to_string());
                assert_eq!(
                    p[11],
                    r.material_fibre
                        .map(|f| f.index())
                        .unwrap_or(255)
                        .to_string()
                );
                assert_eq!(p[12], r.colour_name.as_deref().unwrap_or("-"));
                assert_eq!(p[13], r.element_literal);
                assert_eq!(p[14], r.planetary_mode_literal);
                assert_eq!(p[15], r.interval_literal);
            }
            "step" => {
                let r = field.rule(index, role).unwrap();
                let d: usize = p[3].parse().unwrap();
                assert_eq!(p[4], u8::from(r.spelled_steps24.is_some()).to_string());
                assert_eq!(
                    p[5],
                    r.spelled_steps24
                        .as_ref()
                        .map(|s| s[d])
                        .unwrap_or(0)
                        .to_string()
                );
            }
            "pitch" => {
                pitches += 1;
                let t = if p[3] == "0" {
                    TuningPolicy::Retained24Tet
                } else {
                    TuningPolicy::BimbaSpelled24Tet
                };
                let d: usize = p[4].parse().unwrap();
                let tonic = [55.0, 261.625565, 1234.567][p[5].parse::<usize>().unwrap()];
                let actual = condition_pitches(index, role, t, tonic).unwrap();
                let native: f64 = p[7].parse().unwrap();
                match actual {
                    Some(values) => {
                        assert_eq!(p[6], "0");
                        assert!((values[d] - native).abs() <= 1e-10 * values[d].max(1.0));
                    }
                    None => {
                        unavailable += 1;
                        assert_eq!(p[6], "4");
                        assert_eq!(native, 91.0);
                    }
                }
            }
            _ => panic!("unexpected observer output"),
        }
    }
    assert_eq!(rules, 127);
    assert_eq!(pitches, 72 * 2 * 2 * 8 * 3);
    assert_eq!(field.gaps.len(), 17);
    fs::write(dir.join("condition-parity.json"),serde_json::to_string_pretty(&json!({"schema":"ql.m2-condition-parity/v1","result":"passed","rules":rules,"pitch_cases":pitches,"explicit_unavailable_cases":unavailable,"registry_revision":field.registry_revision})).unwrap()+"\n").unwrap();
}
#[test]
fn one_event_joins_music_colour_element_drive_world_and_m3() {
    let mut v = request();
    let stamp = v["stamp"].clone();
    v["condition"]["palette"] = json!({"stamp":stamp,"policy_ref":"test-linear-palette-v1","entries":[{"name":"yellow","linear_rgba":[0.8,0.6,0.1,1.0]}]});
    let time = v["at_unix_ms"].as_u64().unwrap();
    v["world_observations"] = json!([{"planet_id":6,"longitude_degrees":12.5,"provider_ref":"controlled-sky-fixture","source_revision":"fixture-v1","observed_at_unix_ms":time-5}]);
    v["resonator"] = json!({
        "stamp":v["stamp"], "provider_ref":"fixture:controlled-resonator",
        "geometry_ref":"fixture:geometry", "material_ref":"fixture:material",
        "material_model_ref":"fixture:constitutive-model",
        "material_parameters":{"elastic_modulus":{"value":100.0,"unit":"GPa","source_ref":"fixture:material"}},
        "modes":[{"mode_ref":"fixture:earth-mode", "source_coordinate":"#2-2-2-5-5",
            "material_fibre":"earth", "carrier_weights":[{"carrier":0,"weight":1.0}],
            "frequency_hz":220.0,"amplitude":[1.0,0.5],"excitation":[0.2,0.0],
            "damping_per_second":0.01,"nodal_state_ref":"fixture:nodes","antinodal_state_ref":"fixture:antinodes"}]
    });
    let out = execute(v);
    let c = &out["condition"];
    assert_eq!(c["schema"], CONDITION_CONTRACT);
    assert_eq!(c["identity"], out["identity"]);
    assert_eq!(c["source_path"]["planet_coordinate"], "#2-5-5");
    assert_eq!(c["source_path"]["chakra_coordinate"], "#2-5-0/1-1");
    assert_eq!(c["source_path"]["material_fibre"], "earth");
    assert_eq!(c["source_path"]["tattva_coordinate"], "#2-2-2-5-5");
    assert_eq!(c["colour"]["source_name"], "yellow");
    assert_eq!(c["colour"]["linear_rgba"], json!([0.8, 0.6, 0.1, 1.0]));
    assert_eq!(c["world"]["age_ms"], 5);
    assert_eq!(c["world"]["observation"]["planet_id"], 6);
    assert_eq!(c["drive"], out["vimarsha"]["reading"]);
    assert_eq!(c["physical_material"], out["resonator"]);
    assert_eq!(c["physical_material"]["stamp"]["identity"], c["identity"]);
    assert_eq!(
        c["physical_material"]["modes"][0]["source_coordinate"],
        c["source_path"]["tattva_coordinate"]
    );
    assert_eq!(
        c["physical_material"]["modes"][0]["material_fibre"],
        c["source_path"]["material_fibre"]
    );
    assert_eq!(c["musical"]["pitches_hz"].as_array().unwrap().len(), 8);
    assert_eq!(c["m3_transform_policies"].as_array().unwrap().len(), 3);
    assert_eq!(
        c["source_path"]["musical_relations"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    fs::create_dir_all(root().join("target/m2-receipt")).unwrap();
    fs::write(
        root().join("target/m2-receipt/condition-frame.json"),
        serde_json::to_string_pretty(&out).unwrap() + "\n",
    )
    .unwrap();
}
#[test]
fn conjugate_and_tuning_changes_keep_the_same_source_path_but_update_music() {
    let direct = execute(request());
    let mut v = request();
    v["mef_conditions"] = json!([36]);
    v["condition"]["active_mef_condition"] = json!(36);
    v["vimarsha"]["lens"] = json!(6);
    let prime = execute(v.clone());
    assert_ne!(
        direct["condition"]["sublens_ref"],
        prime["condition"]["sublens_ref"]
    );
    assert_ne!(
        direct["condition"]["drive"]["audio_octet_hz"],
        prime["condition"]["drive"]["audio_octet_hz"]
    );
    assert_eq!(
        direct["condition"]["source_path"],
        prime["condition"]["source_path"]
    );
    v["condition"]["tuning"] = json!("retained24_tet");
    let retained = execute(v);
    assert_eq!(
        prime["condition"]["source_path"],
        retained["condition"]["source_path"]
    );
    assert_ne!(
        prime["condition"]["musical"]["pitches_hz"],
        retained["condition"]["musical"]["pitches_hz"]
    );
}
#[test]
fn unavailable_mappings_colour_palette_and_provider_do_not_get_invented() {
    let plain = execute(request());
    assert!(plain["condition"]["colour"]["linear_rgba"].is_null());
    assert!(plain["condition"]["world"].is_null());
    assert!(plain["condition"]["physical_material"].is_null());
    let mut v = request();
    v["condition"]["maqam_index"] = json!(4);
    let absent = execute(v);
    assert!(absent["condition"]["source_path"].is_null());
    assert!(absent["condition"]["colour"]["source_name"].is_null());
    assert_eq!(absent["condition"]["musical"]["pitches_hz"], json!([]));
    let mut v = request();
    v["condition"]["maqam_index"] = json!(0);
    let sun = execute(v);
    assert_eq!(sun["condition"]["source_path"]["chakra_index"], 7);
    assert!(sun["condition"]["colour"]["source_name"].is_null());
}
#[test]
fn malformed_and_cross_generation_inputs_fail_before_production() {
    for (key, value) in [
        ("maqam_index", json!(72)),
        ("active_mef_condition", json!(1)),
        ("tonic_hz", json!(-1)),
        ("tuning", json!("guess_rgb")),
    ] {
        let mut v = request();
        v["condition"][key] = value;
        assert!(M2Request::from_json(&v.to_string()).is_err(), "{key}");
    }
    let mut v = request();
    let mut stamp = v["stamp"].clone();
    stamp["identity"]["profile_generation"] = json!(99999);
    v["condition"]["palette"] = json!({"stamp":stamp,"policy_ref":"palette","entries":[]});
    assert!(M2Request::from_json(&v.to_string()).is_err());
    let mut v = request();
    v["vimarsha"] = Value::Null;
    assert!(M2Request::from_json(&v.to_string()).is_err());
    let mut field: Value = serde_json::from_str(SOURCE_FIELD).unwrap();
    field["rules"][0]["planet_coordinate"] = json!("#2-5-5");
    assert!(CorrespondenceField::from_json(&field.to_string()).is_err());
}
