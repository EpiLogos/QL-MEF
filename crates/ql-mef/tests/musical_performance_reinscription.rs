#[path = "support/retained_performance.rs"]
mod retained_performance;
use ql_mef::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation};
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference, bind_performance_return};
use ql_mef::nara::{EventBasisRefs, SourceRevision, domain::ProtectedRef, replay::NaraOccasion};
use ql_mef::performance_audio::prepare_native_performance;
use serde_json::json;
fn r(s: &str) -> ReturnReference {
    ReturnReference {
        reference: s.into(),
        revision: "1".into(),
    }
}
fn context(kind: &str) -> ReturnContext {
    ReturnContext {
        context: r("reference:composition"),
        receiver: r("reference:receiver"),
        source_occasion: (kind != "world").then(|| r("reference:original-occasion")),
        protected_state: (kind != "world").then(|| r("reference:protected-state")),
        consent: None,
        kind: kind.into(),
        private: kind != "world",
        required_assets: vec![],
    }
}
#[test]
fn new_native_command_returns_current_score_body_and_preserves_original_occasion_and_edition() {
    let input = retained_performance::preparation();
    let first = prepare_native_performance(input).unwrap();
    let event = EventBasisRefs::from_basis(first.native_basis()).unwrap();
    let original_occasion = NaraOccasion {
        occasion_ref: "reference:original-occasion".into(),
        subject_id: event.subject_ref.clone(),
        event,
        personal_reception_generation: 7,
        identity_revision: "identity:original/1".into(),
        day_ref: "central:day:2026-10-02".into(),
        now_ref: "central:now:composition".into(),
        occurrence_at_unix_ms: 100,
        receipt_at_unix_ms: 900,
        protected_state_ref: ProtectedRef {
            ref_id: "reference:protected-state".into(),
            revision: "1".into(),
            owner_ref: first.physical_body().subject_ref().into(),
        },
        activity_refs: vec![],
        oracle_packet_refs: vec![],
        transformation_phase_refs: vec![],
        context_reading_refs: vec![],
        integration_return_refs: vec![],
        expression_refs: vec![],
        source_revisions: vec![SourceRevision {
            source_ref: "reference:original-event".into(),
            revision: "1".into(),
            standing_ref: "reference:original-source".into(),
        }],
    };
    let old = bind_performance_return(
        &first,
        Some(original_occasion.clone()),
        context("personal"),
        73,
    )
    .unwrap();
    let old_bytes = serde_json::to_vec(&old.snapshot().unwrap()).unwrap();
    let current = old.replay_score().unwrap();
    let snapshot = current.snapshot();
    let command = M3Command {
        schema: COMMAND_SCHEMA.into(),
        event_ref: first.physical_body().event_ref().into(),
        subject_ref: first.physical_body().subject_ref().into(),
        expected_generation: current.generation(),
        actor_ref: "reference:performer".into(),
        cause_ref: "reference:actual-native-score-act".into(),
        occurrence_unix_ms: snapshot["occurrence_unix_ms"].as_u64().unwrap() + 1,
        receipt_unix_ms: snapshot["receipt_unix_ms"].as_u64().unwrap() + 1,
        operations: vec![M3Operation::AdvanceClock { steps: 1 }],
    };
    let mut next_input = retained_performance::preparation();
    next_input.coupled.m3_commands.push(command.clone());
    next_input.physical.expected_m3_generation = current.generation() + 1;
    let next = prepare_native_performance(next_input).unwrap();
    let (new, receipt) = old.reinscribe_retained(command.clone(), &next).unwrap();
    assert_eq!(receipt.status, "applied");
    assert_eq!(
        new.replay_score().unwrap().snapshot(),
        next.native_basis().m3
    );
    assert_eq!(new.original_occasion(), Some(&original_occasion));
    assert_eq!(
        serde_json::to_vec(&old.snapshot().unwrap()).unwrap(),
        old_bytes
    );
    assert_ne!(
        old.snapshot().unwrap()["content_digest"],
        new.snapshot().unwrap()["content_digest"]
    );
    let basis = new.expression_basis().unwrap();
    assert_eq!(
        basis["identity"]["m3_generation"],
        (current.generation() + 1).to_string()
    );
    assert_eq!(
        basis["prepared_body"]["source_generation"],
        current.generation() + 1
    );
    assert_eq!(basis["m4_episode"]["occurrence_at_unix_ms"], 100);
    assert_eq!(basis["m3_replay"]["commands"].as_array().unwrap().len(), 2);
    assert!(
        old.reinscribe_retained(command, &first)
            .unwrap_err()
            .contains("preparation")
    );
}
#[test]
fn a_receipt_without_actual_after_preparation_or_cross_event_command_cannot_change_saved_world_score()
 {
    let p = prepare_native_performance(retained_performance::preparation()).unwrap();
    let old = bind_performance_return(&p, None, context("world"), 73).unwrap();
    let original = old.snapshot().unwrap();
    let state = old.replay_score().unwrap();
    let score = state.snapshot();
    let command = M3Command {
        schema: COMMAND_SCHEMA.into(),
        event_ref: "foreign:event".into(),
        subject_ref: p.physical_body().subject_ref().into(),
        expected_generation: state.generation(),
        actor_ref: "reference:performer".into(),
        cause_ref: "reference:score-act".into(),
        occurrence_unix_ms: score["occurrence_unix_ms"].as_u64().unwrap() + 1,
        receipt_unix_ms: score["receipt_unix_ms"].as_u64().unwrap() + 1,
        operations: vec![M3Operation::AdvanceClock { steps: 1 }],
    };
    assert!(old.reinscribe_retained(command, &p).is_err());
    assert_eq!(old.snapshot().unwrap(), original);
    let basis = old.expression_basis().unwrap();
    assert_eq!(basis["context"]["private"], false);
    assert_eq!(basis["m4_episode"], json!(null));
}
