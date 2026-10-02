//! Paired actual Rust producer -> native C source admission -> physical body.
//! Parent compiles this C++ binary against the qualified current C archive and
//! json-c, then invokes this retained integration test with its exact path.
use ql_mef::MFace;
use ql_mef::m3_state::{M3Request, M3State};
use ql_mef::physical_body::*;
use ql_mef::source_form_body::*;
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};
fn actual_body(address: u8, family: BodyFamily) -> (M3State, PreparedSourceFormBody) {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut request: M3Request = serde_json::from_value(fixture["request"].clone()).unwrap();
    request.address = address;
    request.pose = 0;
    request.matrix_axis = 0;
    let state = M3State::new(request).unwrap();
    let provenance = |reference: &str| PhysicalProvenance {
        reference: reference.into(),
        revision: "1".into(),
        source_ref: SOURCE_GEOMETRY_BASIS.into(),
        standing: PhysicalStanding::AgentProposed,
    };
    let recipe = SourceGeometryRecipe {
        provenance: provenance("controlled:source-native-physical-wire"),
        family,
        frame_side_metres: 0.1,
        site_separation_metres: 0.3,
        section_by_element_m2: [1e-4, 2e-4, 3e-4, 4e-4],
        intersite_section_m2: 1e-4,
        prestress_newtons: if family == BodyFamily::AxialTruss {
            0.0
        } else {
            5.0
        },
    };
    let mut weights = vec![0.0; 12];
    weights[11] = 1.0;
    let controls = SourceBodyControls {
        expected_m3_generation: state.generation(),
        body_revision: 1,
        preparation_ref: format!("controlled:physical-native/{address}"),
        state_ref: "controlled:physical-native/state".into(),
        material: PhysicalMaterial {
            provenance: provenance("controlled:elastic-model"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 1000.0,
            damping_alpha_per_second: 0.4,
            damping_beta_seconds: 0.0,
        },
        sample_rate: 48000,
        exciter: SpatialProjection {
            axis: [0.0, 0.0, 1.0],
            node_weights: weights.clone(),
        },
        pickup: SpatialProjection {
            axis: [0.0, 0.0, 1.0],
            node_weights: weights,
        },
        pickup_linear_per_metre: 1000.0,
        max_force_newtons: 10.0,
        max_impulse_newton_seconds: 0.01,
        max_displacement_metres: 0.01,
    };
    let body = prepare_source_form_body(
        &state,
        source_form_coordinate(&state, MFace::Pratibimba).unwrap(),
        recipe,
        controls,
    )
    .unwrap();
    (state, body)
}
fn native(state: &M3State, body: &PreparedSourceFormBody) -> Value {
    let binary = std::env::var_os("QL_PHYSICAL_WIRE_TEST")
        .or_else(|| std::env::var_os("QL_NATIVE_WIRE_TEST"))
        .expect("parent-qualified physical_body_wire binary required");
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let packet = json!({"preparation":body.body(),"current_m3":state.snapshot()});
    let bytes = serde_json::to_vec(&packet).unwrap();
    assert!(bytes.len() < 1024 * 1024);
    let mut input = child.stdin.take().unwrap();
    input.write_all(&bytes).unwrap();
    input.write_all(b"\n").unwrap();
    drop(input);
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    let receipt: BodyObservationReceipt =
        serde_json::from_value(result["receipt"].clone()).unwrap();
    assert!(body.body().validate_observation(&receipt, 256).is_ok());
    assert!(receipt.mechanical_energy_joules > 0.0);
    assert_eq!(result["negative_cases"], 12);
    assert_eq!(
        result["checkpoint"]["schema"],
        "ql.physical-body-checkpoint/v1"
    );
    assert_eq!(result["checkpoint"]["state"]["samples_elapsed"], "256");
    assert_eq!(result["checkpoint"]["units"]["displacement"], "m");
    assert_eq!(result["checkpoint"]["units"]["velocity"], "m/s");
    assert_eq!(
        result["checkpoint"]["identity"]["source_coordinate"],
        body.body().source_coordinate().source_ref
    );
    assert_eq!(result["checkpoint"]["identity"]["face"], "pratibimba");
    let rest: Vec<[f64; 3]> = serde_json::from_value(result["rest_metres"].clone()).unwrap();
    assert_eq!(
        rest,
        body.body()
            .request()
            .geometry
            .nodes
            .iter()
            .map(|n| n.rest_metres)
            .collect::<Vec<_>>()
    );
    result
}
#[test]
#[ignore = "requires parent-qualified current native C++/json-c binary; explicitly execute at the paired integration gate"]
fn actual_source_metric_and_full_provenance_cross_language_admission_drive_one_body() {
    let (source, body) = actual_body(0, BodyFamily::AxialTruss);
    let axial = native(&source, &body);
    let (source, body) = actual_body(4, BodyFamily::AxialTruss);
    let changed = native(&source, &body);
    assert_ne!(axial["rest_metres"], changed["rest_metres"]);
    assert_ne!(
        axial["frequencies_hz"], changed["frequencies_hz"],
        "native geometric/material change had no eigenstructure effect"
    );
    let (source, body) = actual_body(0, BodyFamily::PrestressedTensionNetwork);
    let tension = native(&source, &body);
    assert_ne!(
        axial["frequencies_hz"], tension["frequencies_hz"],
        "distinct constitutive family collapsed into same physical law"
    );
}
