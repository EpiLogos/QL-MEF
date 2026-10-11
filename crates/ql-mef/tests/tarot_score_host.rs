//! The field host's Tarot score operations over the real installed worker:
//! `score-resolve` holds the subject's deterministic `ql.tarot-score/v1`
//! reading, `score-state` answers it with its currentness, foreign subjects
//! refuse by name, and neither op advances the field. Run with
//! `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test tarot_score_host -- --ignored`.
//!
//! The serde wire contract is exercised without any worker below.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::host::{FieldHost, HostOperation, HostRequest};
use ql_mef::continuous::scene_field::{BINDING_REQUEST, BindingRequest};
use ql_mef::tarot_score::{PrimaryAnchor, ScoreBasisInput};
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

/// The dated-sky fixture with its admission binding refreshed to the current
/// native registry — the qualified transit admission's own requirement.
fn sky() -> Value {
    let mut sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
    ))
    .unwrap();
    sky["source_binding"]["registry_revision"] = json!(ql_mef::m2::catalogue().registry_revision());
    sky
}

/// A `score-resolve` command whose basis names `subject_ref` and carries the
/// admitted dated occasion sky (kairos anchors; the scene subject carries no
/// natal identity reading).
fn score_resolve(subject_ref: String) -> Value {
    serde_json::to_value(HostOperation::ScoreResolve {
        basis: Box::new(ScoreBasisInput {
            subject_ref,
            locus_ref: "#2-5-4".into(),
            identity: None,
            occasion_sky: Some(sky()),
            clock_steps: 359,
            primary_anchor: PrimaryAnchor::Kairos { body_index: 0 },
            drawn: Vec::new(),
            boundary_passage: None,
            manifestation: None,
        }),
    })
    .unwrap()
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn score_resolve_and_state_round_trip_on_the_live_host() {
    let mut driver = Driver::open("test:score-resolve");
    let subject_ref = driver.current["field"]["subject_ref"]
        .as_str()
        .unwrap()
        .to_owned();
    let event_ref = driver.current["field"]["event_ref"]
        .as_str()
        .unwrap()
        .to_owned();
    let response = driver.send(score_resolve(subject_ref));
    if response["status"] == "ok" {
        // The held reading is the score itself: an admitted basis resolved to
        // at least one token, against this field's subject and event.
        let score = &response["score"];
        assert_eq!(score["schema"], "ql.tarot-score/v1");
        assert_eq!(score["subject_ref"], driver.current["field"]["subject_ref"]);
        assert!(!score["tokens"].as_array().unwrap().is_empty());
        assert_eq!(response["current"], json!(true));
        assert_eq!(response["resolved_event_ref"], json!(event_ref)); // score-state answers the held reading with its currentness — the
        // same basis revision, still current against the unchanged event.
        let state = driver.send(json!({"operation":"score-state"}));
        assert_eq!(state["status"], "ok", "{}", state["error"]);
        assert_eq!(
            state["score"]["basis_revision"], score["basis_revision"],
            "score-state answers the held reading, not a re-derivation"
        );
        assert_eq!(state["current"], json!(true));
        assert_eq!(
            state["resolved_event_ref"], state["field"]["event_ref"],
            "the event is unchanged, so the held reading is current"
        );
    } else {
        // A refusal here is the score owner's named basis admission — never a
        // silent substitute reading and never an advance of the field.
        let error = response["error"].as_str().unwrap();
        assert!(
            error.contains("no anchor basis")
                || error.contains("transit requires")
                || error.contains("snapshot"),
            "expected the named basis-admission refusal: {error}"
        );
        assert_eq!(
            response["field"]["generation"],
            driver.current["field"]["generation"]
        );
    }
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn score_foreign_subject_is_refused_and_reads_do_not_advance() {
    let mut driver = Driver::open("test:score-foreign");
    let generation_before = driver.current["field"]["generation"].clone();
    let samples_before = driver.current["field"]["samples_elapsed"].clone();
    let refused = driver.send(score_resolve("person:foreign-score-subject".into()));
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("is not this field's subject"),
        "named refusal: {}",
        refused["error"]
    );
    // The refused op consumed its sequence; the driver's cursor carried on.
    // The score operations are reads: the field stands exactly where it did.
    let read = driver.send(json!({"operation":"read"}));
    assert_eq!(read["status"], "ok");
    assert_eq!(read["field"]["generation"], generation_before);
    assert_eq!(read["field"]["samples_elapsed"], samples_before);
}

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn score_state_before_resolve_refuses_by_name() {
    let mut driver = Driver::open("test:score-state-empty");
    let state = driver.send(json!({"operation":"score-state"}));
    assert_eq!(state["status"], "refused");
    assert!(
        state["error"]
            .as_str()
            .unwrap()
            .contains("no Tarot score has been resolved"),
        "named refusal: {}",
        state["error"]
    );
}

// ---------------------------------------------------------------------------
// Wire contract — no worker required
// ---------------------------------------------------------------------------

#[test]
fn score_operations_admit_their_wire_names_and_refuse_unknown_fields() {
    let resolve: HostOperation = serde_json::from_value(json!({
        "operation":"score-resolve",
        "basis":{
            "subject_ref":"ql:k2/default-subject",
            "locus_ref":"#2-5-4",
            "clock_steps":359,
            "primary_anchor":{"kind":"kairos","body_index":0},
            "drawn":[]
        }
    }))
    .unwrap();
    let HostOperation::ScoreResolve { basis } = &resolve else {
        panic!("score-resolve admitted by name");
    };
    assert_eq!(basis.subject_ref, "ql:k2/default-subject");
    assert_eq!(
        basis.primary_anchor,
        PrimaryAnchor::Kairos { body_index: 0 }
    );

    let state: HostOperation = serde_json::from_value(json!({"operation":"score-state"})).unwrap();
    assert!(matches!(state, HostOperation::ScoreState {}));

    // The wire round trip is stable: kebab-case operation names, basis intact.
    let wire = serde_json::to_value(&resolve).unwrap();
    assert_eq!(wire["operation"], "score-resolve");
    let back: HostOperation = serde_json::from_value(wire).unwrap();
    assert!(matches!(back, HostOperation::ScoreResolve { .. }));

    // Unknown fields and wrong names refuse, as every host operation does.
    for value in [
        json!({"operation":"score-state", "basis":{}}),
        json!({"operation":"score-resolve"}),
        json!({"operation":"score-resolve", "basis":{"surprise":1}}),
        json!({"operation":"score-resolve", "basis":{
            "subject_ref":"s","locus_ref":"#2-5-4","clock_steps":0,
            "primary_anchor":{"kind":"natal","planet_id":0},"drawn":[],"surprise":1}}),
        json!({"operation":"score-resolve", "basis":{
            "subject_ref":"s","locus_ref":"#2-5-4","clock_steps":0,
            "primary_anchor":{"kind":"harmonic","pitch":3},"drawn":[]}}),
    ] {
        assert!(
            serde_json::from_value::<HostOperation>(value.clone()).is_err(),
            "accepted {value}"
        );
    }
}
