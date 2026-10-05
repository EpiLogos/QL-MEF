//! The Ta-Onta procedural stage over the real installed worker, through the
//! exact field host the Expressions application drives: the app's default
//! opening composes the scene binding, the host opens its supervised worker,
//! and a versioned procedure injects a form fold, a material change, a clock
//! change and a generated passage into the live flow. Run with
//! `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test ta_onta_stage -- --ignored`.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::host::{FieldHost, HostOperation, HostRequest};
use ql_mef::continuous::scene_field::{BINDING_REQUEST, BindingRequest};
use ql_mef::continuous::stage::{StageChange, StagePassage, StageProcedure, StageTrigger};
use ql_mef::m3_state::M3Operation;
use serde_json::{Value, json};

fn worker() -> PathBuf {
    PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker"))
}

/// The application's own plain opening: the composed default scene binding,
/// its host configuration, and one supervised native worker behind it.
fn open(instance: &str) -> FieldHost {
    let binding = ql_mef::continuous::scene_field::binding(BindingRequest {
        schema: BINDING_REQUEST.into(),
        instance_ref: instance.into(),
        texture: [32, 16],
        units_per_metre: 1.0,
        event: None,
        sky: None,
        field: None,
        geometry: Some(ql_mef::continuous::scene_field::SceneGeometry {
            longitude_samples: 32,
            latitude_samples: 16,
            metres_per_unit: 1.0,
            attachment: 1,
        }),
        material: None,
        reception: None,
    })
    .unwrap();
    let config: ql_mef::continuous::scene_field::SceneConfig =
        serde_json::from_value(binding["host"].clone()).unwrap();
    FieldHost::open_scene(&worker(), config, Duration::from_secs(20)).unwrap()
}

struct Driver {
    host: FieldHost,
    current: Value,
}

impl Driver {
    fn open(instance: &str) -> Self {
        let host = open(instance);
        let current = host.ready();
        assert_eq!(current["status"], "ready");
        assert_eq!(current["available"], json!(true));
        Self { host, current }
    }

    fn packet(&self, command: Value) -> HostRequest {
        HostRequest {
            schema: "ql.field-host-request/v1".into(),
            instance_ref: self.current["instance_ref"].as_str().unwrap().into(),
            event_ref: self.current["field"]["event_ref"].as_str().unwrap().into(),
            subject_ref: self.current["field"]["subject_ref"]
                .as_str()
                .unwrap()
                .into(),
            request_id: (self.current["last_request_id"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap()
                + 1)
            .to_string(),
            expected_generation: self.current["field"]["generation"].as_str().unwrap().into(),
            expected_samples_elapsed: self.current["field"]["samples_elapsed"]
                .as_str()
                .unwrap()
                .into(),
            command: serde_json::from_value(command).unwrap(),
        }
    }

    fn send(&mut self, command: Value) -> Value {
        let request = self.packet(command);
        self.current = self.host.execute(request);
        self.current.clone()
    }
}

fn procedure(changes: Vec<StageChange>) -> StageProcedure {
    let selector: Vec<String> = changes
        .iter()
        .map(|c| c.slot().unwrap().to_owned())
        .collect();
    StageProcedure {
        schema: ql_mef::continuous::stage::STAGE_PROCEDURE.into(),
        procedure_ref: "ta-onta:stage:live-fold".into(),
        revision: 1,
        subject_ref: "ql:k2/default-subject".into(),
        trigger: StageTrigger::Invocation,
        selector,
        changes,
        passage: None,
    }
}

fn form_change(operations: Vec<M3Operation>) -> Value {
    serde_json::to_value(HostOperation::StageEvaluate {
        procedure: Box::new(procedure(vec![StageChange::Form { operations }])),
    })
    .unwrap()
}

fn targets(field: &Value) -> Vec<Vec<f64>> {
    field["targets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| {
            t["position"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect()
        })
        .collect()
}

fn max_move(a: &[Vec<f64>], b: &[Vec<f64>]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            x.iter()
                .zip(y)
                .map(|(u, v)| (u - v).abs())
                .fold(0.0, f64::max)
        })
        .fold(0.0, f64::max)
}

/// The M3 inscription clock of a native readback, wherever the exact integer
/// travels as a number or as its canonical decimal string.
fn source_clock_steps(readback: &Value) -> u64 {
    let steps = &readback["m3_clock"]["steps"];
    steps
        .as_u64()
        .or_else(|| steps.as_str().and_then(|s| s.parse().ok()))
        .expect("native readback carries its M3 inscription clock")
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn the_stage_discloses_scoped_state_over_the_live_scene() {
    let mut driver = Driver::open("test:stage-state");
    let state = driver.send(json!({"operation":"stage-state"}));
    assert_eq!(state["status"], "ok");
    let stage = &state["stage"];
    assert_eq!(stage["schema"], "ql.stage-state/v1");
    assert_eq!(stage["subject_ref"], "ql:k2/default-subject");
    assert_eq!(stage["form"]["address"], json!(7));
    assert_eq!(stage["slots"]["form"]["owner"], Value::Null);
    assert_eq!(stage["slots"]["material.damping"]["owner"], Value::Null);
    assert!(stage["material"]["damping_per_second"].is_number());
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn one_procedure_folds_the_form_and_the_body_moves() {
    let mut driver = Driver::open("test:stage-fold");
    let before = targets(&driver.current["field"]);
    let response = driver.send(form_change(vec![M3Operation::SetPose { pose: 2 }]));
    assert_eq!(response["status"], "ok", "{}", response["error"]);
    let stage = &response["stage"];
    assert_eq!(stage["applied"], json!(true));
    assert_eq!(
        stage["contributions"][0]["key"],
        "ta-onta:stage:live-fold@1/form"
    );
    assert_eq!(stage["contributions"][0]["slot"], "form");
    // The fold is a real determinant: the acknowledged field moved and the
    // influence reading answers with it.
    assert!(max_move(&before, &targets(&response["field"])) > 0.0);
    assert!(response["influence"].is_object());
    assert!(response["influence"]["native_readback"].is_object());

    // The claimed slot is disclosed until its procedure retires it.
    let state = driver.send(json!({"operation":"stage-state"}));
    assert_eq!(
        state["stage"]["slots"]["form"]["owner"],
        json!("ta-onta:stage:live-fold@1/form")
    );
    let retired =
        driver.send(json!({"operation":"stage-retire", "procedure_ref":"ta-onta:stage:live-fold"}));
    assert_eq!(retired["status"], "ok");
    assert_eq!(
        retired["stage"]["retired"],
        json!(["ta-onta:stage:live-fold@1/form"])
    );
    let state = driver.send(json!({"operation":"stage-state"}));
    assert_eq!(state["stage"]["slots"]["form"]["owner"], Value::Null);
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn the_same_procedure_on_the_same_basis_replays_the_same_body() {
    let mut one = Driver::open("test:stage-replay-a");
    let mut two = Driver::open("test:stage-replay-b");
    let a = one.send(form_change(vec![
        M3Operation::CastCreases {
            angles_deg10: [120, -60, 30],
            velocities_deg10: [0, 0, 0],
        },
        M3Operation::SetPose { pose: 3 },
    ]));
    let b = two.send(form_change(vec![
        M3Operation::CastCreases {
            angles_deg10: [120, -60, 30],
            velocities_deg10: [0, 0, 0],
        },
        M3Operation::SetPose { pose: 3 },
    ]));
    assert_eq!(a["status"], "ok", "{}", a["error"]);
    assert_eq!(b["status"], "ok", "{}", b["error"]);
    // Two hosts, one event basis: the fold's body is identical, without any
    // hidden generator state between evaluations.
    assert_eq!(a["influence"]["shape_ref"], b["influence"]["shape_ref"]);
    assert_eq!(targets(&a["field"]), targets(&b["field"]));
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn a_stale_cursor_is_refused_and_the_stage_stays_usable() {
    let mut driver = Driver::open("test:stage-stale");
    let mut request = driver.packet(form_change(vec![M3Operation::SetPose { pose: 1 }]));
    request.expected_generation = "999999".into();
    let refused = driver.host.execute(request);
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("stale native cursor")
    );
    assert_eq!(refused["available"], json!(true));
    // The envelope consumed its sequence even on refusal; the refused
    // response carries the host's true cursor for the successor request.
    driver.current = refused;
    let ok = driver.send(json!({"operation":"stage-state"}));
    assert_eq!(ok["status"], "ok");
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn a_generated_passage_advances_the_flow_scene_by_scene() {
    let mut procedure = procedure(vec![StageChange::Form {
        operations: vec![M3Operation::SetPose { pose: 4 }],
    }]);
    procedure.passage = Some(StagePassage { scenes: 3 });
    let mut driver = Driver::open("test:stage-passage");
    let before = targets(&driver.current["field"]);
    let state_before = driver.send(json!({"operation":"stage-state"}));
    let clock_before = state_before["stage"]["form"]["clock_steps"]
        .as_u64()
        .unwrap();
    let generation_before: u64 = state_before["field"]["generation"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let response = driver.send(
        serde_json::to_value(HostOperation::StageEvaluate {
            procedure: Box::new(procedure),
        })
        .unwrap(),
    );
    assert_eq!(response["status"], "ok", "{}", response["error"]);
    let stage = &response["stage"];
    assert_eq!(stage["generated_scenes"], json!(2));
    // Each generated scene was one admitted M1 form advance: the inscription
    // clock carries two admitted thirty-degree ticks.
    let clock_after = source_clock_steps(&response["influence"]["native_readback"]);
    assert_eq!(clock_after, clock_before + 60);
    // The invoked fold and both generated scenes were real determinants on
    // the live worker: control advanced at least once per determinant.
    let generation_after: u64 = response["field"]["generation"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!(generation_after >= generation_before + 3);
    assert!(max_move(&before, &targets(&response["field"])) > 0.0);
    let state = driver.send(json!({"operation":"stage-state"}));
    assert_eq!(
        state["stage"]["slots"]["form"]["procedure_ref"],
        json!("ta-onta:stage:live-fold")
    );
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn a_second_procedure_on_an_owned_slot_is_refused_by_name() {
    let mut driver = Driver::open("test:stage-conflict");
    let first = StageProcedure {
        procedure_ref: "ta-onta:stage:first".into(),
        ..procedure(vec![StageChange::Damping { per_second: 0.75 }])
    };
    let held = driver.send(
        serde_json::to_value(HostOperation::StageEvaluate {
            procedure: Box::new(first),
        })
        .unwrap(),
    );
    assert_eq!(held["status"], "ok", "{}", held["error"]);
    assert_eq!(
        held["stage"]["contributions"][0]["key"],
        json!("ta-onta:stage:first@1/material.damping")
    );
    let second = StageProcedure {
        procedure_ref: "ta-onta:stage:second".into(),
        ..procedure(vec![StageChange::Damping { per_second: 0.2 }])
    };
    let refused = driver.send(
        serde_json::to_value(HostOperation::StageEvaluate {
            procedure: Box::new(second),
        })
        .unwrap(),
    );
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("owned by ta-onta:stage:first@1/material.damping")
    );
    // Arrival order never arbitrates: the first procedure still owns the slot
    // and its declared damping is still the one in force.
    let state = driver.send(json!({"operation":"stage-state"}));
    assert_eq!(
        state["stage"]["slots"]["material.damping"]["owner"],
        json!("ta-onta:stage:first@1/material.damping")
    );
    assert_eq!(
        state["stage"]["material"]["damping_per_second"],
        json!(0.75)
    );
}
