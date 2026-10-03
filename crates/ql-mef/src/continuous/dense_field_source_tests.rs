//! Genuine bounded source storage input from the existing native FieldHost.
//! This controlled supplied two-mode field is not a World, private lease,
//! performance, provider observation or source authority fixture.
use super::*;
use crate::continuous::performance::native_source_support;
use std::io::Write;

#[test]
#[ignore = "requires the exact normal-floor native worker; emits its actual retained source"]
fn actual_dense_field_source_keeps_all_65000_original_samples_and_later_basis() {
    let worker = std::path::PathBuf::from(
        std::env::var_os("QL_NATIVE_FIELD_WORKER").expect("actual native worker"),
    );
    let output = std::path::PathBuf::from(
        std::env::var_os("QL_NATIVE_DENSE_FIELD_SOURCE_ARTIFACT")
            .expect("actual native source artifact destination"),
    );
    let mut input = native_source_support::config(false).0.input;
    let mode = |index: u32| {
        json!({"mode_ref":format!("controlled:dense-source/mode/{index}"),
            "source_coordinate":"#2-1","material_fibre":"earth",
            "carrier_weights":[{"carrier":index,"weight":1.0}],
            "frequency_hz":123.0+61.0*f64::from(index),"amplitude":[0.005,0.001],
            "excitation":[0.001,0.0],"damping_per_second":0.25,
            "nodal_state_ref":"controlled:dense-source/node",
            "antinodal_state_ref":"controlled:dense-source/antinode"})
    };
    // Authored numerical operands use the existing supplied-field vocabulary.
    // The real native constructor independently checks every modal/sample
    // operand. Original source/output below comes only from that held owner.
    input.m2.resonator = Some(
        serde_json::from_value(json!({
            "stamp":input.m2.stamp,"provider_ref":"controlled:dense-source",
            "geometry_ref":"controlled:dense-source/grid",
            "material_ref":"controlled:dense-source/linear",
            "material_model_ref":"controlled:dense-source/modal-v1",
            "material_parameters":{},"modes":[mode(0),mode(1)]
        }))
        .unwrap(),
    );
    let samples = (0..65_000u32)
        .map(|index| {
            json!({
                "identity":u64::from(index)+1,
                "constituent":"#3-0",
                "attachment":0,
                "rest_metres":[f64::from(index%250)*0.001,f64::from(index/250)*0.001,0.0],
                "mode_shapes":[[0.0,0.0,1.0],[0.0,1.0,0.0]]
            })
        })
        .collect::<Vec<_>>();
    let field: FieldInput = serde_json::from_value(json!({
        "subject_ref":input.m3.subject_ref,"sample_rate":48000,
        "driver_numerator":1,"driver_denominator":1,
        "clock":{"generation":"0","inscription":{"turns":"0","half_degrees":0},
            "lensing":{"turns":"0","half_degrees":0},"grid_origins":[0,0,0],
            "rate_numerators":["0","0"],"rate_denominator":1,"rate_remainders":["0","0"]},
        "units":{"amplitude":"m","excitation":"m/s","shape":"dimensionless","position":"m","audio":"linear"},
        "audio_gains":[1.0,0.5],"samples":samples,
        "shape_ref":"controlled:dense-source/grid-shape"
    })).unwrap();
    assert!(field.samples.len() * 2 <= 262_144);
    let original_input = serde_json::to_value(&input).unwrap();
    let original_field = serde_json::to_value(&field).unwrap();
    let config = HostConfig {
        instance_ref: "controlled:dense-source/native-host".into(),
        basis: input.clone(),
        field,
    };
    assert!(serde_json::to_vec(&config).unwrap().len() < MAX_HOST_INPUT as usize);
    let mut host = FieldHost::open(&worker, config, Duration::from_secs(60)).unwrap();
    let original = host.retained_procedural_source_artifact().unwrap();
    assert_eq!(original["original_field"], original_field);
    assert_eq!(original["original_basis"]["input"], original_input);
    input.m1.revision = "controlled:dense-source/later-current".into();
    let ready = host.ready();
    let request = HostRequest {
        schema: HOST_REQUEST.into(),
        instance_ref: host.instance_ref.clone(),
        event_ref: ready["field"]["event_ref"].as_str().unwrap().into(),
        subject_ref: ready["field"]["subject_ref"].as_str().unwrap().into(),
        request_id: "1".into(),
        expected_generation: ready["field"]["generation"].as_str().unwrap().into(),
        expected_samples_elapsed: ready["field"]["samples_elapsed"].as_str().unwrap().into(),
        command: HostOperation::Replace {
            basis: Box::new(input.clone()),
        },
    };
    let reply = host.execute(request);
    assert_eq!(reply["status"], "ok", "{reply}");
    let artifact = host.retained_procedural_source_artifact().unwrap();
    assert_eq!(artifact["original_field"], original_field);
    assert_eq!(artifact["original_basis"], original["original_basis"]);
    assert_eq!(
        artifact["current_basis"]["input"],
        serde_json::to_value(&input).unwrap()
    );
    assert_ne!(artifact["original_basis"], artifact["current_basis"]);
    assert_eq!(
        artifact["original_field"]["samples"]
            .as_array()
            .unwrap()
            .len(),
        65_000
    );
    assert_eq!(artifact["original_field"]["samples"][0]["identity"], 1);
    assert_eq!(
        artifact["original_field"]["samples"][64_999]["identity"],
        65_000
    );
    let bytes = serde_json::to_vec(&artifact).unwrap();
    assert!(bytes.len() <= MAX_HOST_OUTPUT);
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&output)
        .unwrap();
    file.write_all(&bytes).unwrap();
    file.sync_all().unwrap();
    use sha2::{Digest, Sha256};
    println!(
        "actual-native-dense-field-source samples=65000 modes=2 original-kept=true current-changed=true bytes={} sha256={:x} private-authority=ungranted",
        bytes.len(),
        Sha256::digest(&bytes)
    );
}
