// V/#293: actual native preparation and Return, then source-score admission.
// These are authored composition operands, never fabricated performed receipts.
// Proposed adoption: crates/ql-mef/tests/independent_score_admission.rs.
mod support {
    include!("support/retained_performance.rs");
}
use ql_mef::musical_performance_return::{
    MusicalPerformanceReturn, ReturnContext, ReturnReference, bind_performance_return,
};
use ql_mef::musical_performance_score::{ScoreKeys, ScoreSource, compile_score};
use ql_mef::nara::{EventBasisRefs, SourceRevision, domain::ProtectedRef, replay::NaraOccasion};
use ql_mef::performance_audio::{PreparedPerformanceBinding, prepare_native_performance};
use serde_json::{Value, json};

fn reference(value: &str) -> ReturnReference {
    ReturnReference {
        reference: value.into(),
        revision: "1".into(),
    }
}
fn world() -> ReturnContext {
    ReturnContext {
        context: reference("reference:v293/score/world"),
        receiver: reference("reference:v293/score/world-receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        kind: "world".into(),
        private: false,
        required_assets: vec![],
    }
}
fn episode(prepared: &PreparedPerformanceBinding) -> NaraOccasion {
    let event = EventBasisRefs::from_basis(prepared.native_basis()).unwrap();
    NaraOccasion {
        occasion_ref: "reference:v293/score/original-episode".into(),
        subject_id: event.subject_ref.clone(),
        event,
        personal_reception_generation: 9,
        identity_revision: "reference:v293/score/controlled-identity/1".into(),
        day_ref: "central:day:2026-10-02".into(),
        now_ref: "central:now:v293-controlled-score".into(),
        occurrence_at_unix_ms: 1000,
        receipt_at_unix_ms: 1001,
        protected_state_ref: ProtectedRef {
            ref_id: "reference:v293/score/controlled-protected-state".into(),
            revision: "1".into(),
            owner_ref: prepared.physical_body().subject_ref().into(),
        },
        activity_refs: vec![],
        oracle_packet_refs: vec![],
        transformation_phase_refs: vec![],
        context_reading_refs: vec!["reference:v293/score/personal-context".into()],
        integration_return_refs: vec![],
        expression_refs: vec!["expression:retained/current".into()],
        source_revisions: vec![SourceRevision {
            source_ref: "reference:v293/score/controlled-original-source".into(),
            revision: "1".into(),
            standing_ref: "reference:controlled-native-episode".into(),
        }],
    }
}
fn protected_context(occasion: &NaraOccasion, shared: bool) -> ReturnContext {
    ReturnContext {
        context: reference("reference:v293/score/personal-context"),
        receiver: reference("reference:v293/score/controlled-receiver"),
        source_occasion: Some(reference(&occasion.occasion_ref)),
        protected_state: Some(reference(&occasion.protected_state_ref.ref_id)),
        consent: shared.then(|| reference("reference:v293/score/controlled-consent")),
        kind: if shared { "shared" } else { "personal" }.into(),
        private: true,
        required_assets: vec![],
    }
}
fn source<'a>(
    p: &'a PreparedPerformanceBinding,
    r: &'a MusicalPerformanceReturn,
) -> ScoreSource<'a> {
    ScoreSource {
        prepared: p,
        original_return: r,
        keys: ScoreKeys::Architectural,
    }
}
fn authored(prepared: &PreparedPerformanceBinding, returned: &MusicalPerformanceReturn) -> Value {
    let mut basis = returned.expression_basis().unwrap();
    // A reference to the existing Expression seal, not an invented C execution.
    basis["content_digest"] = json!("reference:v293/test-authored-basis");
    let pitches = returned.expression_pitches(0).unwrap();
    let sine = prepared.notes()[0]["phase_sin"].as_f64().unwrap();
    let cosine = prepared.notes()[0]["phase_cos"].as_f64().unwrap();
    json!({
        "schema":"oi.expression-performance/v1", "performance_ref":"expression:v293/score/current",
        "sample_rate":prepared.physical_body().request().sample_rate,
        "duration_samples":"1024", "ppq":960, "bases":[basis], "pitches":pitches,
        "layers":[{"layer_ref":"layer:v293/main","title":"Main","enabled":true,"solo":false}],
        "pages":[{"events":[
            ["1","37",0,0,{"n":["1","1",0,0.7,sine,cosine]}],
            ["2","101",0,0,{"e":["1",0.5,pitches[0]["hertz"]]}],
            ["3","256",0,0,{"o":"1"}]
        ]}],
        "parameters":[{"native_owner":"ql.audio","action_ref":"parameter","target_ref":"force","unit":"N",
            "scope":"instrument","minimum":0.0,"maximum":10.0,"baseline":1.0,"smoothing_samples":"64"}],
        "routes":[{"route_ref":"route:v293/force","source_ref":"source:v293/automation","enabled":true,
            "scale":1.0,"offset":0.0,"delay_samples":"0"}],
        "tempo":[{"at_sample":"0","at_tick":"0","micros_per_quarter":500000}],
        "loop_range":null,"position_sample":"0",
        "replay":{"mode":"native_checkpoint","max_reconstruction_samples":"48000",
            "model_revision":"ql.performance-audio/v1","event_tolerance_samples":0,
            "physical_tolerance":0.0,"display_policy":"same-native-cursor"},
        "checkpoints":[], "content_digest":"reference:v293/test-authored-performance"
    })
}

#[test]
fn v293_score_sample_clock_must_match_the_actual_prepared_physical_body_rate() {
    let prepared = prepare_native_performance(support::preparation()).unwrap();
    assert_eq!(prepared.physical_body().request().sample_rate, 48000);
    let returned = bind_performance_return(&prepared, None, world(), 29).unwrap();
    let input = authored(&prepared, &returned);
    let score = compile_score(
        "expression:retained/current",
        1,
        &[source(&prepared, &returned)],
        &input,
    )
    .unwrap();
    score
        .verify_replay(&[source(&prepared, &returned)], &input)
        .unwrap();
    for foreign_rate in [44100u64, 96000, 8000, 192000] {
        let mut detached = input.clone();
        detached["sample_rate"] = json!(foreign_rate);
        assert!(
            compile_score(
                "expression:retained/current",
                1,
                &[source(&prepared, &returned)],
                &detached
            )
            .is_err(),
            "source-qualified score admitted {foreign_rate} Hz against the actual 48000 Hz body"
        );
    }
}

#[test]
fn v293_score_keeps_the_original_protected_episode_and_refuses_a_different_valid_native_return() {
    let prepared = prepare_native_performance(support::preparation()).unwrap();
    let original = episode(&prepared);
    let public = bind_performance_return(&prepared, None, world(), 29).unwrap();
    for shared in [false, true] {
        let returned = bind_performance_return(
            &prepared,
            Some(original.clone()),
            protected_context(&original, shared),
            29,
        )
        .unwrap();
        let input = authored(&prepared, &returned);
        let score = compile_score(
            "expression:retained/current",
            1,
            &[source(&prepared, &returned)],
            &input,
        )
        .unwrap();
        score
            .verify_replay(&[source(&prepared, &returned)], &input)
            .unwrap();
        let snapshot = score.snapshot().unwrap();
        assert_eq!(
            snapshot["original_episode_refs"],
            json!([original.occasion_ref])
        );
        assert_eq!(
            snapshot["source_bases"][0]["original_episode"],
            serde_json::to_value(&original).unwrap()
        );
        assert_eq!(
            snapshot["source_bases"][0]["m3_source_score"],
            prepared.native_basis().m3
        );
        assert_eq!(returned.original_occasion(), Some(&original));
        assert!(
            compile_score(
                "expression:retained/current",
                1,
                &[source(&prepared, &public)],
                &input
            )
            .is_err(),
            "a valid public World Return replaced the protected original episode"
        );
        let mut other = original.clone();
        other.occasion_ref = "reference:v293/score/different-valid-episode".into();
        let replacement = bind_performance_return(
            &prepared,
            Some(other.clone()),
            protected_context(&other, shared),
            29,
        )
        .unwrap();
        assert_eq!(replacement.original_occasion(), Some(&other));
        assert!(
            compile_score(
                "expression:retained/current",
                1,
                &[source(&prepared, &replacement)],
                &input
            )
            .is_err(),
            "same event/body but another valid original episode replaced score custody"
        );
        for pointer in [
            "/bases/0/m4_episode",
            "/bases/0/context/source_occasion",
            "/bases/0/context/protected_state",
        ] {
            let mut lost = input.clone();
            *lost.pointer_mut(pointer).unwrap() = Value::Null;
            assert!(
                compile_score(
                    "expression:retained/current",
                    1,
                    &[source(&prepared, &returned)],
                    &lost
                )
                .is_err(),
                "source score lost original custody at {pointer}"
            );
        }
    }
}
