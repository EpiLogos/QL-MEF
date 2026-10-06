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
        .filter_map(|c| c.slot().unwrap().map(str::to_owned))
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

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn a_bound_procedure_fires_on_its_admitted_determinant() {
    // Two owners over one event: the bound host strikes the Moon voice on
    // every admitted M1 advance; the control host only advances.
    let mut bound = Driver::open("test:stage-bound");
    let mut control = Driver::open("test:stage-bound-control");
    let mut procedure = procedure(vec![StageChange::Strike {
        mode_ref: "scene:planet/#2-5-4".into(),
        amplitude: [0.9, 0.0],
    }]);
    procedure.selector = Vec::new();
    procedure.trigger = StageTrigger::Determinant {
        operation: "m1-advance".into(),
    };
    let bind = bound.send(
        serde_json::to_value(HostOperation::StageBind {
            procedure: Box::new(procedure),
            max_evaluations: None,
        })
        .unwrap(),
    );
    assert_eq!(bind["status"], "ok", "{}", bind["error"]);
    assert_eq!(bind["stage"]["binding"]["determinant"], json!("m1-advance"));
    assert_eq!(bind["stage"]["binding"]["standing"], json!("active"));
    let seed_bound = bound.send(json!({"operation":"advance", "frames":256, "muted":true}));
    let seed_control = control.send(json!({"operation":"advance", "frames":256, "muted":true}));
    assert_eq!(seed_bound["status"], "ok");
    assert_eq!(seed_control["status"], "ok");
    let before_bound = seed_bound["field"]["amplitudes_metres"].clone();
    let before_control = seed_control["field"]["amplitudes_metres"].clone();
    let fired = bound.send(json!({"operation":"m1-advance", "ticks":1}));
    let plain = control.send(json!({"operation":"m1-advance", "ticks":1}));
    assert_eq!(fired["status"], "ok", "{}", fired["error"]);
    assert_eq!(plain["status"], "ok", "{}", plain["error"]);
    // The binding fired exactly once, named its momentary act, and the struck
    // voice carries it: the Moon's amplitude differs from the unbound twin
    // while every other voice rang identically.
    let records = fired["stage_bound_firings"].as_array().unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["applied"], json!(true));
    assert_eq!(
        records[0]["receipt"]["momentary_acts"][0]["key"],
        json!("ta-onta:stage:live-fold/strike:scene:planet/#2-5-4")
    );
    let struck = &fired["field"]["amplitudes_metres"];
    let reference = &plain["field"]["amplitudes_metres"];
    assert_ne!(
        struck[3], reference[3],
        "the Moon voice did not carry the strike"
    );
    for (index, (a, b)) in struck
        .as_array()
        .unwrap()
        .iter()
        .zip(reference.as_array().unwrap())
        .enumerate()
    {
        if index != 3 {
            assert_eq!(a, b, "voice {index} changed without being struck");
        }
    }
    let _ = (before_bound, before_control);
    // The binding's budget is disclosed and still active.
    let state = bound.send(json!({"operation":"stage-state"}));
    let bindings = state["stage"]["bindings"].as_array().unwrap();
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0]["evaluations"], json!(1));
    assert_eq!(bindings[0]["standing"], json!("active"));
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn the_bound_procedure_never_reenters_and_stops_at_its_budget() {
    let mut procedure = procedure(vec![StageChange::Form {
        operations: vec![M3Operation::SetPose { pose: 5 }],
    }]);
    procedure.passage = Some(StagePassage { scenes: 3 });
    procedure.trigger = StageTrigger::Determinant {
        operation: "m1-advance".into(),
    };
    let mut driver = Driver::open("test:stage-budget");
    let bind = driver.send(
        serde_json::to_value(HostOperation::StageBind {
            procedure: Box::new(procedure),
            max_evaluations: Some(2),
        })
        .unwrap(),
    );
    assert_eq!(bind["status"], "ok", "{}", bind["error"]);
    // Firing one: the invoked fold plus its two generated scenes. The passage
    // itself advances the M1 flow twice — under re-entry those internal
    // determinants would consume the whole budget here.
    let first = driver.send(json!({"operation":"m1-advance", "ticks":1}));
    assert_eq!(first["status"], "ok", "{}", first["error"]);
    assert_eq!(first["stage_bound_firings"].as_array().unwrap().len(), 1);
    // Firing two: still admitted, the budget not yet spent.
    let second = driver.send(json!({"operation":"m1-advance", "ticks":1}));
    assert_eq!(second["status"], "ok", "{}", second["error"]);
    assert_eq!(second["stage_bound_firings"].as_array().unwrap().len(), 1);
    // The budget is spent: the determinant keeps working, the binding stops.
    let third = driver.send(json!({"operation":"m1-advance", "ticks":1}));
    assert_eq!(third["status"], "ok", "{}", third["error"]);
    assert!(third["stage_bound_firings"].is_null());
    let state = driver.send(json!({"operation":"stage-state"}));
    let bindings = state["stage"]["bindings"].as_array().unwrap();
    assert_eq!(bindings[0]["evaluations"], json!(2));
    assert_eq!(bindings[0]["max_evaluations"], json!(2));
    assert_eq!(bindings[0]["standing"], json!("exhausted"));
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn unbinding_stops_the_firing_and_names_the_final_standing() {
    let mut procedure = procedure(vec![StageChange::Strike {
        mode_ref: "scene:planet/#2-5-4".into(),
        amplitude: [0.5, 0.0],
    }]);
    procedure.selector = Vec::new();
    procedure.trigger = StageTrigger::Determinant {
        operation: "replace-event".into(),
    };
    let mut driver = Driver::open("test:stage-unbind");
    let bind = driver.send(
        serde_json::to_value(HostOperation::StageBind {
            procedure: Box::new(procedure),
            max_evaluations: None,
        })
        .unwrap(),
    );
    assert_eq!(bind["status"], "ok", "{}", bind["error"]);
    // A played strike is not an admitted determinant trigger: no firing.
    let strike = driver.send(json!({"operation":"strike", "strikes":[
            {"mode_ref":"scene:planet/#2-5-0/1", "amplitude":[0.3, 0.0]}]}));
    assert_eq!(strike["status"], "ok", "{}", strike["error"]);
    assert!(strike["stage_bound_firings"].is_null());
    let unbound =
        driver.send(json!({"operation":"stage-unbind", "procedure_ref":"ta-onta:stage:live-fold"}));
    assert_eq!(unbound["status"], "ok");
    assert_eq!(unbound["stage"]["binding"]["standing"], json!("active"));
    let again =
        driver.send(json!({"operation":"stage-unbind", "procedure_ref":"ta-onta:stage:live-fold"}));
    assert_eq!(again["status"], "refused");
    assert!(
        again["error"]
            .as_str()
            .unwrap()
            .contains("no binding names")
    );
    let state = driver.send(json!({"operation":"stage-state"}));
    assert!(state["stage"]["bindings"].as_array().unwrap().is_empty());
}

/// The driven journey's specimen over the installed worker: the fold sequence
/// holds the event's standing form 7 for one turn, transitions to the ATC
/// fold motif over one turn (smoothstep), and holds ATC for one turn — the
/// same walk the native specimen proves, here carried by the live host.
fn driven_sequence() -> ql_mef::form_sequence::FoldSequence {
    use ql_mef::form_recipe::{
        DeclaredMobility, DeclaredPolarity, DeclaredSite, FormDetermination,
    };
    let atc = FormDetermination::FoldMotif {
        sites: [
            DeclaredSite {
                polarity: DeclaredPolarity::Yin,
                mobility: DeclaredMobility::Moving,
            },
            DeclaredSite {
                polarity: DeclaredPolarity::Yang,
                mobility: DeclaredMobility::Moving,
            },
            DeclaredSite {
                polarity: DeclaredPolarity::Yin,
                mobility: DeclaredMobility::Resting,
            },
        ],
    };
    ql_mef::form_sequence::FoldSequence {
        schema: ql_mef::form_sequence::FORM_SEQUENCE_CONTRACT.into(),
        sequence_ref: "ta-onta:psg:driven-journey".into(),
        revision: 1,
        subject_ref: "ql:k2/default-subject".into(),
        axis: ql_mef::form_sequence::SequenceAxis::Inscription,
        origin: ql_mef::continuous::LiftInput {
            turns: "0".into(),
            half_degrees: 0,
        },
        phases: vec![
            ql_mef::form_sequence::SequencePhase::Hold {
                determination: FormDetermination::Address { address: 7 },
                half_degrees: 720,
            },
            ql_mef::form_sequence::SequencePhase::Transition {
                to: atc.clone(),
                half_degrees: 720,
                easing: ql_mef::form_sequence::Easing::Smoothstep,
            },
            ql_mef::form_sequence::SequencePhase::Hold {
                determination: atc.clone(),
                half_degrees: 720,
            },
        ],
    }
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn the_driven_sequence_moves_the_real_worker_through_named_quanta() {
    use ql_mef::form_sequence::stage_effect;

    let sequence = driven_sequence();
    let mut driver = Driver::open("test:psg-driven-journey");
    let standing = |state: &Value| state["stage"]["form"]["address"].as_u64().unwrap() as u8;
    // One driven step: read the host's observed standing through its own
    // stage-state disclosure, compile the fold law's effect at the cursor
    // against it, and evaluate the carrying procedure on the live host.
    let drive = |driver: &mut Driver, cursor: u64| {
        let state = driver.send(json!({"operation":"stage-state"}));
        let effect = stage_effect(&sequence, cursor, standing(&state)).unwrap();
        let mut carried = procedure(effect.changes());
        carried.procedure_ref = "ta-onta:stage:psg-driven".into();
        carried.selector = effect.selector();
        let response = driver.send(
            serde_json::to_value(HostOperation::StageEvaluate {
                procedure: Box::new(carried),
            })
            .unwrap(),
        );
        (effect, response)
    };
    let observed = |response: &Value| {
        response["influence"]["native_readback"]["form"]["address"]
            .as_u64()
            .expect("the native readback discloses the form address")
    };

    // (a) Strictly inside the transition: the effect names no form and issues
    // none — the receipt carries the clock slot alone, and the worker's own
    // readback still stands on form 7. The display axis moved to the cursor.
    let (effect, response) = drive(&mut driver, 1080);
    assert_eq!(response["status"], "ok", "{}", response["error"]);
    assert!(effect.form_operations.is_none());
    let slots: Vec<&str> = response["stage"]["contributions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["slot"].as_str().unwrap())
        .collect();
    assert_eq!(slots, vec!["clock.inscription"]);
    // The display axis moved to the exact cursor phase (1080 steps).
    assert_eq!(
        response["field"]["clock"]["inscription"]["turns"],
        json!("1")
    );
    assert_eq!(
        response["field"]["clock"]["inscription"]["half_degrees"],
        json!(360)
    );
    assert_eq!(observed(&response), 7);

    // (b) The boundary: the codon resolves onto ATC while the host stands on
    // 7 — the compiled form change rides the event's own command batch, the
    // real worker applies it, and the readback discloses the exact cast
    // telemetry of the new form.
    let (effect, response) = drive(&mut driver, 1440);
    assert_eq!(response["status"], "ok", "{}", response["error"]);
    assert!(effect.form_operations.is_some());
    let contributions = response["stage"]["contributions"].as_array().unwrap();
    assert_eq!(contributions.len(), 2);
    assert_eq!(contributions[0]["slot"], json!("form"));
    assert_eq!(
        contributions[0]["key"],
        json!("ta-onta:stage:psg-driven@1/form")
    );
    assert_eq!(contributions[1]["slot"], json!("clock.inscription"));
    assert_eq!(
        observed(&response),
        6,
        "the body moved through the named quanta"
    );
    let form = &response["influence"]["native_readback"]["form"];
    assert_eq!(form["angles_deg10"], json!([225, -225, 225]));
    assert_eq!(form["velocities_deg10"], json!([225, 225, 0]));
    assert_eq!(form["nucleotides"], json!([0, 1, 2]));

    // (c) The hold after the boundary: the form is already bound — the effect
    // re-issues nothing, the receipt carries the clock alone, the form stands.
    let (effect, response) = drive(&mut driver, 1500);
    assert_eq!(response["status"], "ok", "{}", response["error"]);
    assert!(effect.form_operations.is_none());
    assert!(effect.standing.contains("quanta already bound"));
    let slots: Vec<&str> = response["stage"]["contributions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["slot"].as_str().unwrap())
        .collect();
    assert_eq!(slots, vec!["clock.inscription"]);
    let state = driver.send(json!({"operation":"stage-state"}));
    assert_eq!(standing(&state), 6, "the boundary's form still stands");
    assert_eq!(
        state["stage"]["slots"]["form"]["owner"],
        json!("ta-onta:stage:psg-driven@1/form")
    );
}
