//! Binding's engine (manual §22.3 "The Binding skin"): the coupling at its
//! scale, the string's stretch and its energy over the cell a matrix makes of
//! a hexagram, and the kernel event behind the last motion. The expected
//! values are LightBox's QCD law (EpiLogos/LightBox src/qcd/physics.ts)
//! computed here, and the cell is `ql_core::generate`'s, never a typed table.
use ql_core::*;
use ql_mef::m3_binding::*;
use ql_mef::m3_state::*;
use serde_json::{Value, json};

fn request(hexagram: u8, matrix: &str, register: &str) -> Value {
    json!({"hexagram": hexagram, "matrix": matrix, "register": register,
        "tension_gev_per_fm": 0.9, "coupling_scale_gev": 1.0})
}

fn read(request: Value) -> Value {
    binding_reading(&request).unwrap()
}

/// LightBox's running coupling: one loop, Λ = 210 MeV, three flavours.
fn alpha(scale_gev: f64) -> f64 {
    let raw = 12.0 * std::f64::consts::PI / (27.0 * 2.0 * (scale_gev / 0.21).max(1.05).ln());
    raw.clamp(0.1, 1.5)
}

#[test]
fn the_coupling_runs_at_its_scale_by_lightboxs_one_loop_law() {
    let reading = read(request(17, "moving-resting", "pp"));
    assert_eq!(reading["schema"], BINDING_CONTRACT);
    let a = reading["coupling"]["alpha_s"].as_f64().unwrap();
    assert!((a - alpha(1.0)).abs() < 1e-12);
    // The design's sample: α_s = 0.447 at 1 GeV.
    assert!((a - 0.447).abs() < 5e-4, "{a}");
    let mut high = request(17, "moving-resting", "pp");
    high["coupling_scale_gev"] = json!(10.0);
    let a10 = read(high)["coupling"]["alpha_s"].as_f64().unwrap();
    assert!(a10 < a, "the coupling falls as its scale rises");
    assert!((a10 - alpha(10.0)).abs() < 1e-12);
}

#[test]
fn a_meson_is_stretched_by_the_charge_between_its_ends_and_carries_the_cornell_energy() {
    // Moving and resting at hexagram 17: CT under CC, CCC and CTC (the
    // design's sample), one pp charge apart.
    let cell = generate(MatrixFamily::MovingResting, 17);
    assert!(!cell.is_non_dual());
    let reading = read(request(17, "moving-resting", "pp"));
    let (plus, minus) = (
        cell.positive.four_charge().pp,
        cell.negative.four_charge().pp,
    );
    assert_eq!(
        reading["cell"]["positive"]["sequence"],
        cell.positive.to_string()
    );
    assert_eq!(
        reading["cell"]["negative"]["sequence"],
        cell.negative.to_string()
    );
    assert_eq!(reading["cell"]["positive"]["charge"], json!(plus));
    assert_eq!(reading["cell"]["negative"]["charge"], json!(minus));
    let between = (plus - minus).unsigned_abs() as f64;
    assert_eq!(reading["string"]["charge_between"], json!(between as u64));
    let r = reading["string"]["stretch_fm"].as_f64().unwrap();
    assert!((r - REST_LENGTH_FM * between).abs() < 1e-12);
    let e = reading["string"]["energy_gev"].as_f64().unwrap();
    let a = alpha(1.0);
    assert!((e - (-(4.0 / 3.0) * a * HBAR_C_GEV_FM / r + 0.9 * r)).abs() < 1e-12);
    // The design's sample: 0.8 fm and 0.573 GeV.
    assert!(
        (r - 0.8).abs() < 1e-12 && (e - 0.573).abs() < 5e-4,
        "{r} {e}"
    );
    // Each site's coin value, read from the codon.
    let coins: Vec<Value> = cell
        .positive
        .site_values()
        .iter()
        .map(|v| json!(v.value()))
        .collect();
    assert_eq!(
        reading["cell"]["positive"]["coin_values"],
        Value::Array(coins)
    );
}

#[test]
fn lower_tension_slackens_the_string_and_its_energy_falls() {
    let mut slack = request(17, "moving-resting", "pp");
    slack["tension_gev_per_fm"] = json!(0.3);
    let taut = read(request(17, "moving-resting", "pp"));
    let slack = read(slack);
    assert_eq!(taut["string"]["stretch_fm"], slack["string"]["stretch_fm"]);
    assert!(slack["string"]["energy_gev"].as_f64() < taut["string"]["energy_gev"].as_f64());
}

#[test]
fn a_non_dual_cell_is_one_codon_with_no_string_and_no_energy() {
    let (family, hexagram) = MatrixFamily::ALL
        .iter()
        .flat_map(|f| (0u8..64).map(move |h| (*f, h)))
        .find(|(f, h)| generate(*f, *h).is_non_dual())
        .unwrap();
    let name = match family {
        MatrixFamily::Complementary => "complementary",
        MatrixFamily::MovingResting => "moving-resting",
        MatrixFamily::SameQuality => "same-quality",
    };
    let reading = read(request(hexagram, name, "mm"));
    assert_eq!(reading["cell"]["non_dual"], true);
    assert_eq!(reading["string"]["charge_between"], 0);
    assert_eq!(reading["string"]["stretch_fm"], 0.0);
    assert_eq!(reading["string"]["energy_gev"], Value::Null);
}

#[test]
fn the_engine_refuses_physics_outside_the_manuals_ranges_and_unknown_words() {
    for (key, value) in [
        ("tension_gev_per_fm", json!(2.5)),
        ("tension_gev_per_fm", json!(0.05)),
        ("coupling_scale_gev", json!(0.2)),
        ("coupling_scale_gev", json!(11.0)),
        ("register", json!("qq")),
        ("matrix", json!("sideways")),
        ("hexagram", json!(64)),
    ] {
        let mut bad = request(17, "moving-resting", "pp");
        bad[key] = value;
        assert!(binding_reading(&bad).is_err(), "{key}");
    }
    let mut extra = request(17, "moving-resting", "pp");
    extra["position"] = json!([0, 0]);
    assert!(
        binding_reading(&extra).is_err(),
        "Binding takes no position: only the kernel's symbols"
    );
}

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap()
}

fn receipt(operations: Vec<M3Operation>) -> Value {
    let mut v = fixture()["request"].clone();
    v["address"] = json!(17);
    v["pose"] = json!(0);
    v["aperture"] = json!(0);
    v["matrix_axis"] = json!(0);
    let mut state = M3State::new(serde_json::from_value(v).unwrap()).unwrap();
    let mut c: M3Command = serde_json::from_value(fixture()["commands"][0].clone()).unwrap();
    c.expected_generation = state.generation();
    c.operations = operations;
    serde_json::to_value(state.apply(c).unwrap()).unwrap()
}

#[test]
fn the_motion_names_the_kernel_event_behind_it() {
    let with = |receipt: Value| {
        let mut r = request(17, "moving-resting", "pp");
        r["receipt"] = receipt;
        read(r)["event"].clone()
    };
    let matrix = with(receipt(vec![M3Operation::SetMatrix { family: 1 }]));
    assert_eq!(matrix["kind"], "matrix");
    assert_eq!(matrix["what"], "Complementary to moving and resting");
    let line = with(receipt(vec![M3Operation::ChangeLine { line: 4 }]));
    assert_eq!(line["kind"], "line");
    assert!(
        line["what"]
            .as_str()
            .unwrap()
            .starts_with("Line 4 of hexagram")
    );
    let inscribed = with(receipt(vec![M3Operation::SelectForm { address: 23 }]));
    assert_eq!(inscribed["kind"], "inscription");
    assert!(inscribed["what"].as_str().unwrap().ends_with("inscribed"));
    // An operation that moves no cell names no motion.
    assert_eq!(
        with(receipt(vec![M3Operation::SetAperture { aperture: 3 }])),
        Value::Null
    );
    // No receipt: no event is read.
    assert_eq!(
        read(request(17, "moving-resting", "pp"))["event"],
        Value::Null
    );
}
