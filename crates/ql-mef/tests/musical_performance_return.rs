mod support {
    include!("support/retained_performance.rs");
}
use ql_mef::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation};
use ql_mef::musical_performance_return::*;
use ql_mef::nara::{EventBasisRefs, SourceRevision, domain::ProtectedRef, replay::NaraOccasion};
use ql_mef::performance_audio::prepare_native_performance;
use serde_json::Value;
fn r(reference: &str) -> ReturnReference {
    ReturnReference {
        reference: reference.into(),
        revision: "1".into(),
    }
}
fn context(kind: &str) -> ReturnContext {
    ReturnContext {
        context: r("reference:receiving-context"),
        receiver: r("reference:receiving-owner"),
        source_occasion: (kind != "world").then(|| r("reference:original-occasion")),
        protected_state: (kind != "world").then(|| r("reference:protected-native-state")),
        consent: None,
        kind: kind.into(),
        private: kind != "world",
        required_assets: vec![],
    }
}
fn occasion(p: &ql_mef::performance_audio::PreparedPerformanceBinding) -> NaraOccasion {
    let event = EventBasisRefs::from_basis(p.native_basis()).unwrap();
    NaraOccasion {
        occasion_ref: "reference:original-occasion".into(),
        subject_id: event.subject_ref.clone(),
        event,
        personal_reception_generation: 7,
        identity_revision: "identity:retained/4".into(),
        day_ref: "central:day/2026-10-02".into(),
        now_ref: "central:now/physical-musical".into(),
        occurrence_at_unix_ms: 1000,
        receipt_at_unix_ms: 1001,
        protected_state_ref: ProtectedRef {
            ref_id: "reference:protected-native-state".into(),
            revision: "1".into(),
            owner_ref: "reference:personal-owner".into(),
        },
        activity_refs: vec![],
        oracle_packet_refs: vec![],
        transformation_phase_refs: vec![],
        context_reading_refs: vec!["reference:receiving-context".into()],
        integration_return_refs: vec![],
        expression_refs: vec!["expression:retained/current".into()],
        source_revisions: vec![SourceRevision {
            source_ref: "reference:original-event".into(),
            revision: "1".into(),
            standing_ref: "reference:controlled-native-reception".into(),
        }],
    }
}
#[test]
fn actual_post_command_score_body_prime_and_original_episode_are_retained() {
    let p = prepare_native_performance(support::preparation()).unwrap();
    let o = occasion(&p);
    let binding = bind_performance_return(&p, Some(o.clone()), context("personal"), 73).unwrap();
    assert_eq!(binding.original_occasion(), Some(&o));
    assert_eq!(
        binding.replay_score().unwrap().snapshot(),
        p.native_basis().m3
    );
    let scene = binding.expression_basis().unwrap();
    assert_eq!(scene["m1_coordinate"], "#1-3");
    assert_eq!(scene["m1_prime"], true);
    assert_eq!(scene["audio_determination"], *p.determination());
    assert_eq!(scene["m3_score"], p.native_basis().m3);
    assert_eq!(scene["m3_generation"], Value::Null);
    assert_eq!(scene["identity"]["m1_revision"], "11");
    assert_ne!(
        scene["identity"]["m2_generation"],
        scene["identity"]["m3_generation"]
    );
    assert_eq!(
        scene["prepared_body"]["source_generation"],
        scene["m3_score"]["identity"]["profile_generation"]
    );
    assert_eq!(scene["form_state"], scene["m3_score"]["form"]);
    assert_eq!(
        scene["force_state"]["exciter"],
        scene["prepared_body"]["request"]["exciter"]
    );
    assert_eq!(
        scene["m4_episode"]["event"]["m1_revision"],
        scene["identity"]["m1_revision"]
    );
    assert!(scene["m2_plan"]["execution"]["intendedTuningHz"].is_null());
    assert_eq!(scene["tuning"]["standing"], "reference");
    assert_eq!(scene["tuning"]["available"], true);
    assert_eq!(
        binding
            .expression_pitches(0)
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        12
    );
}
#[test]
fn source_defined_reinscription_returns_new_native_generation_without_rewriting_history() {
    let p = prepare_native_performance(support::preparation()).unwrap();
    let binding = bind_performance_return(&p, None, context("world"), 73).unwrap();
    let original = binding.snapshot().unwrap();
    assert!(binding.original_occasion().is_none());
    let world = binding.expression_basis().unwrap();
    assert_eq!(world["context"]["kind"], "world");
    assert_eq!(world["context"]["private"], false);
    assert!(world["context"]["source_occasion"].is_null());
    assert!(world["context"]["protected_state"].is_null());
    assert!(world["m4_episode"].is_null());
    let state = binding.replay_score().unwrap();
    let snapshot = state.snapshot();
    let command = M3Command {
        schema: COMMAND_SCHEMA.into(),
        event_ref: snapshot["identity"]["event_ref"].as_str().unwrap().into(),
        subject_ref: snapshot["subject_ref"].as_str().unwrap().into(),
        expected_generation: state.generation(),
        actor_ref: "reference:retained-performer".into(),
        cause_ref: "reference:source-transcription-choice".into(),
        occurrence_unix_ms: snapshot["occurrence_unix_ms"].as_u64().unwrap() + 1,
        receipt_unix_ms: snapshot["receipt_unix_ms"].as_u64().unwrap() + 1,
        operations: vec![M3Operation::Transcribe { rna: false }],
    };
    let receipt = binding.reinscribe(command.clone()).unwrap();
    assert_eq!(receipt.status, "applied");
    assert_eq!(receipt.after_generation, state.generation() + 1);
    assert_eq!(receipt.after["transcription"]["rna"], false);
    assert_eq!(binding.snapshot().unwrap(), original);
    let mut wrong = command;
    wrong.expected_generation += 1;
    assert!(binding.reinscribe(wrong).is_err());
}
#[test]
fn mismatched_episode_protected_source_and_unavailable_authentic_tuning_are_refused() {
    let p = prepare_native_performance(support::preparation()).unwrap();
    let mut o = occasion(&p);
    o.event.m1_revision = "12".into();
    assert!(bind_performance_return(&p, Some(o), context("personal"), 0).is_err());
    let mut c = context("personal");
    c.protected_state.as_mut().unwrap().revision = "2".into();
    assert!(bind_performance_return(&p, Some(occasion(&p)), c, 0).is_err());
    assert!(bind_performance_return(&p, None, context("personal"), 0).is_err());
    assert!(bind_performance_return(&p, Some(occasion(&p)), context("shared"), 0).is_err());
    let mut input = support::preparation();
    input.require_authentic_condition_tuning = true;
    assert!(prepare_native_performance(input).is_err());
}
