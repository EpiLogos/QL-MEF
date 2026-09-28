//! Explicit shape replacement over the installed native worker (QL-MEF #135).
//! Nodal lines redistribute over the same samples and modal voices: resident
//! state, clock and PCM continue; only targets and the named shape basis move.
//! Requires an installed `ql-field-worker` (QL_FIELD_WORKER or target/k8-cpp).
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use ql_mef::continuous::FieldInput;
use ql_mef::continuous::FieldSession;
use ql_mef::continuous::coupled::{
    CoupledFieldSession, CoupledInput, FrequencyBinding, HarmonicSource, REQUEST,
};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_engine::M2Request;
use ql_mef::m3_state::M3Request;
use serde_json::{Value, json};

fn worker() -> PathBuf {
    let path = std::env::var_os("QL_FIELD_WORKER").map_or_else(
        || {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/k8-cpp/bin/ql-field-worker")
        },
        PathBuf::from,
    );
    assert!(path.is_file(), "missing worker {}", path.display());
    path
}

fn m2() -> M2Request {
    let mut input: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-engine-request-v1.json"
    ))
    .unwrap();
    let mode = |i: u32, re: f64| {
        json!({"mode_ref":format!("controlled:reshape-mode/{i}"), "source_coordinate":"#2-1",
            "material_fibre":"earth", "carrier_weights":[{"carrier":i,"weight":1.0}],
            "frequency_hz":123.0 + f64::from(i) * 61.0, "amplitude":[re,0.001],
            "excitation":[0.001,0.0], "damping_per_second":0.25,
            "nodal_state_ref":"controlled:node", "antinodal_state_ref":"controlled:antinode"})
    };
    input["resonator"] = json!({"stamp":input["stamp"], "provider_ref":"controlled:reshape-test",
        "geometry_ref":"controlled:three-samples", "material_ref":"controlled:linear",
        "material_model_ref":"controlled:modal-v1", "material_parameters":{},
        "modes":[mode(0, 0.005), mode(1, 0.003)]});
    serde_json::from_value(input).unwrap()
}

fn field(subject: &str) -> FieldInput {
    serde_json::from_value(json!({"subject_ref":subject,
        "sample_rate":48000, "driver_numerator":720, "driver_denominator":1,
        "clock":{"generation":"4", "inscription":{"turns":"-1","half_degrees":719},
            "lensing":{"turns":"2","half_degrees":1}, "grid_origins":[3,9,21],
            "rate_numerators":["9","8"], "rate_denominator":8, "rate_remainders":["0","0"]},
        "units":{"amplitude":"m","excitation":"m/s","shape":"dimensionless","position":"m","audio":"linear"},
        "audio_gains":[1.0, 0.5], "samples":[
            {"identity":1,"constituent":"#3-5-5/0","attachment":0,"rest_metres":[0.0,0.0,0.0],
             "mode_shapes":[[0.0,0.0,1.0],[0.0,0.0,1.0]]},
            {"identity":2,"constituent":"#3-0","attachment":1,"rest_metres":[1.0,0.0,0.0],
             "mode_shapes":[[0.0,0.0,1.0],[0.0,0.0,0.0]]},
            {"identity":3,"constituent":"#2-0-0","attachment":2,"rest_metres":[0.0,1.0,0.0],
             "mode_shapes":[[0.0,0.0,0.0],[0.0,0.0,1.0]]}]}))
    .unwrap()
}

/// The re-read nodal quartet as supplied per-sample shapes (sample, then mode).
fn nodal() -> Vec<Vec<[f64; 3]>> {
    vec![
        vec![[0.0, 0.0, 1.0], [0.0, 0.0, -1.0]],
        vec![[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        vec![[1.0, 0.0, 0.0], [0.0, 0.5, 0.0]],
    ]
}

fn open(subject: &str) -> FieldSession {
    FieldSession::open(&worker(), m2(), field(subject), Duration::from_secs(20)).unwrap()
}

#[test]
#[ignore = "requires installed ql-field-worker (QL_FIELD_WORKER or target/k8-cpp)"]
fn reshape_keeps_state_clock_and_pcm_and_moves_only_targets() {
    let (mut shaped, mut control) = (open("controlled:a"), open("controlled:a"));
    assert_eq!(
        shaped.last_receipt()["shape_ref"],
        "controlled:three-samples"
    );
    let a = shaped.advance(1024, false).unwrap();
    let b = control.advance(1024, false).unwrap();
    assert_eq!(a, b);
    let refs = |r: &Value| {
        r["targets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| (t["identity"].clone(), t["constituent"].clone()))
            .collect::<Vec<_>>()
    };
    let reshaped = shaped
        .replace_shapes("controlled:nodal/2", nodal())
        .unwrap();
    let generation: u64 = a["generation"].as_str().unwrap().parse().unwrap();
    assert_eq!(reshaped["generation"], (generation + 1).to_string());
    assert_eq!(reshaped["samples_elapsed"], a["samples_elapsed"]);
    assert_eq!(reshaped["clock"], a["clock"]);
    assert_eq!(reshaped["amplitudes_metres"], a["amplitudes_metres"]);
    assert_eq!(reshaped["m2_identity"], a["m2_identity"]);
    assert_eq!(reshaped["shape_ref"], "controlled:nodal/2");
    assert_eq!(refs(&reshaped), refs(&a));
    assert_ne!(reshaped["targets"], a["targets"]);
    // Fixed sample at the origin: former z0+z1 now reads z0-z1 exactly.
    let z = |i: usize| a["amplitudes_metres"][i][0].as_f64().unwrap();
    let position = &reshaped["targets"][0]["position"];
    assert_eq!(
        position[2].as_f64().unwrap(),
        f64::from((z(0) - z(1)) as f32)
    );
    assert_eq!(position[0], 0.0);
    // Reads keep the admitted basis; audio after reshaping equals the control.
    assert_eq!(shaped.read().unwrap(), reshaped);
    let after = shaped.advance(2048, false).unwrap();
    let expected = control.advance(2048, false).unwrap();
    assert_eq!(after["audio"], expected["audio"]);
    assert!(
        after["audio"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_f64() != Some(0.0))
    );
    assert_eq!(after["amplitudes_metres"], expected["amplitudes_metres"]);
    assert_eq!(after["clock"], expected["clock"]);
    assert_eq!(after["samples_elapsed"], expected["samples_elapsed"]);
    assert_ne!(after["targets"], expected["targets"]);
    assert_eq!(after["shape_ref"], "controlled:nodal/2");
    // A second re-reading is again one declared generation.
    let again = shaped
        .replace_shapes(
            "controlled:nodal/3",
            field("x")
                .samples
                .iter()
                .map(|s| s.mode_shapes.clone())
                .collect(),
        )
        .unwrap();
    assert_eq!(again["targets"], {
        let mut original = control.read().unwrap();
        original["targets"].take()
    });
    // Structural/nonfinite bases are refused locally; the session stays live.
    let last = shaped.last_receipt().clone();
    let mut short = nodal();
    short.pop();
    assert!(shaped.replace_shapes("controlled:short", short).is_err());
    let mut narrow = nodal();
    narrow[1].pop();
    assert!(shaped.replace_shapes("controlled:narrow", narrow).is_err());
    let mut invalid = nodal();
    invalid[2][0][1] = f64::NAN;
    assert!(shaped.replace_shapes("controlled:nan", invalid).is_err());
    assert!(shaped.replace_shapes("", nodal()).is_err());
    assert!(shaped.replace_shapes("bad\nref", nodal()).is_err());
    assert!(shaped.available());
    assert_eq!(shaped.last_receipt(), &last);
    assert_eq!(shaped.read().unwrap(), last);
}

#[test]
#[ignore = "requires installed ql-field-worker (QL_FIELD_WORKER or target/k8-cpp)"]
fn named_initial_basis_is_echoed_and_admitted() {
    let mut input = field("controlled:named");
    input.shape_ref = Some("controlled:nodal/1".into());
    let mut session = FieldSession::open(&worker(), m2(), input, Duration::from_secs(20)).unwrap();
    assert_eq!(session.last_receipt()["shape_ref"], "controlled:nodal/1");
    assert_eq!(session.read().unwrap()["shape_ref"], "controlled:nodal/1");
}

/// Raw wire: the worker itself refuses stale/structural reshapes uncommitted.
#[test]
#[ignore = "requires installed ql-field-worker (QL_FIELD_WORKER or target/k8-cpp)"]
fn worker_refuses_stale_and_structural_reshape_without_commit() {
    let mut child = Command::new(worker())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let mut send = |value: Value| -> Value {
        writeln!(stdin, "{value}").unwrap();
        let mut line = String::new();
        stdout.read_line(&mut line).unwrap();
        serde_json::from_str(&line).unwrap()
    };
    let frame = serde_json::to_value(m2().execute().unwrap()).unwrap();
    let initial = send(
        json!({"schema":"ql.field-control/v1","operation":"initialize",
        "m2":frame, "field":field("controlled:wire")}),
    );
    assert_eq!(initial["schema"], "ql.continuous-field/v1");
    let reshape = |generation: &Value, elapsed: &Value, shapes: Value| {
        json!({"schema":"ql.field-control/v1","operation":"replace-shapes",
            "expected_generation":generation, "expected_samples_elapsed":elapsed,
            "shape_ref":"controlled:wire/2", "shapes":shapes})
    };
    let (generation, elapsed) = (&initial["generation"], &initial["samples_elapsed"]);
    let stale: u64 = generation.as_str().unwrap().parse::<u64>().unwrap() - 1;
    let refusals = [
        reshape(&json!(stale.to_string()), elapsed, json!(nodal())),
        reshape(generation, &json!("1"), json!(nodal())),
        reshape(generation, elapsed, json!(&nodal()[..2])),
        reshape(
            generation,
            elapsed,
            json!([nodal()[0], nodal()[1], [[1.0, 0.0, 0.0]]]),
        ),
        reshape(
            generation,
            elapsed,
            json!([nodal()[0], nodal()[1], [[1.0, 0.0, 0.0], [0.0, 2e6, 0.0]]]),
        ),
        reshape(
            generation,
            elapsed,
            json!([nodal()[0], nodal()[1], [[1.0, 0.0], [0.0, 0.0, 0.0]]]),
        ),
    ];
    for request in refusals {
        let refused = send(request);
        assert_eq!(refused["schema"], "ql.field-error/v1", "{refused}");
        assert_eq!(refused["state_committed"], false);
    }
    let mut extra = reshape(generation, elapsed, json!(nodal()));
    extra["replace_state"] = json!(true);
    assert_eq!(send(extra)["state_committed"], false);
    let read = send(json!({"schema":"ql.field-control/v1","operation":"read"}));
    assert_eq!(read, initial);
    let accepted = send(reshape(generation, elapsed, json!(nodal())));
    assert_eq!(accepted["shape_ref"], "controlled:wire/2");
    assert_eq!(accepted["amplitudes_metres"], initial["amplitudes_metres"]);
    drop(stdin);
    child.wait().unwrap();
}

fn coupled_input() -> CoupledInput {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(fixture["config"].clone()).unwrap();
    let mut m2: M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    let m3_fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_fixture["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    let modes: Vec<Value> = (0..2)
        .map(|i| {
            json!({"mode_ref":format!("controlled:voice/{i}"), "source_coordinate":"#2-2-2-5-5",
                "material_fibre":"earth", "carrier_weights":[{"carrier":i,"weight":1.0}],
                "frequency_hz":137.0, "amplitude":[0.002 * f64::from(i + 1),0.001],
                "excitation":[0.001,0.0], "damping_per_second":0.25,
                "nodal_state_ref":"controlled:nodes", "antinodal_state_ref":"controlled:antinodes"})
        })
        .collect();
    m2.resonator = Some(
        serde_json::from_value(
            json!({"stamp":m2.stamp, "provider_ref":"controlled:reshape",
            "geometry_ref":"controlled:torus-samples", "material_ref":"controlled:linear-medium",
            "material_model_ref":"ql.continuous-linear-mode/v1", "material_parameters":{},
            "modes":modes}),
        )
        .unwrap(),
    );
    CoupledInput {
        schema: REQUEST.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: vec![FrequencyBinding {
            mode_ref: "controlled:voice/0".into(),
            octet_index: 3,
        }],
        condition_frequency_bindings: vec![],
        source_receipts: vec![json!({"standing":"controlled fixture, not a live provider"})],
    }
}

fn newer(input: &CoupledInput) -> CoupledInput {
    let mut next = input.clone();
    next.m2.stamp.identity.profile_generation += 1;
    let identity = next.m2.stamp.identity.clone();
    for stamp in [
        next.m2.vimarsha.as_mut().map(|v| &mut v.stamp),
        next.m2.m1_excitation.as_mut().map(|v| &mut v.stamp),
        next.m2.resonator.as_mut().map(|v| &mut v.stamp),
        next.m2
            .condition
            .as_mut()
            .and_then(|v| v.palette.as_mut())
            .map(|v| &mut v.stamp),
    ]
    .into_iter()
    .flatten()
    {
        stamp.identity = identity.clone();
    }
    next
}

#[test]
#[ignore = "requires installed ql-field-worker (QL_FIELD_WORKER or target/k8-cpp)"]
fn coupled_reshape_and_declared_strike_versus_continuation() {
    let input = coupled_input();
    let field = field("fixture:nara-subject");
    let mut owner =
        CoupledFieldSession::open(&worker(), input.clone(), field, Duration::from_secs(20))
            .unwrap();
    let advanced = owner.advance_field(1024, false).unwrap();
    let reshaped = owner.replace_shapes("controlled:nodal/2", nodal()).unwrap();
    assert_eq!(reshaped["amplitudes_metres"], advanced["amplitudes_metres"]);
    assert_eq!(reshaped["shape_ref"], "controlled:nodal/2");
    assert_ne!(reshaped["targets"], advanced["targets"]);
    // Continuation keeps resident amplitudes across a newer M2 reading.
    let continued = owner.replace_field_state(newer(&input), false).unwrap();
    assert_eq!(
        continued["amplitudes_metres"],
        reshaped["amplitudes_metres"]
    );
    assert_eq!(continued["shape_ref"], "controlled:nodal/2");
    // A declared strike re-excites every mode from the supplied amplitudes.
    let struck = owner
        .replace_field_state(newer(&newer(&input)), true)
        .unwrap();
    let supplied = &owner.current_basis().m2["resonator"]["modes"];
    for (i, amplitude) in struck["amplitudes_metres"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        for part in 0..2 {
            assert_eq!(
                amplitude[part].as_f64(),
                supplied[i]["amplitude"][part].as_f64()
            );
        }
    }
    assert_ne!(struck["amplitudes_metres"], continued["amplitudes_metres"]);
    assert_eq!(struck["samples_elapsed"], advanced["samples_elapsed"]);
    assert_eq!(struck["clock"], advanced["clock"]);
    // A stale strike is refused with the retained basis unchanged.
    let snapshot = owner.snapshot();
    assert!(owner.replace_field_state(newer(&input), true).is_err());
    assert_eq!(owner.snapshot(), snapshot);
}
