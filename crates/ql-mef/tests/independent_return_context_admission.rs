// V/#293 independent actual native Return negatives. These controlled episodes
// contain no person's private source and grant no host consent or H standing.
// Proposed destination: crates/ql-mef/tests/independent_return_context_admission.rs.
// The existing retained fixture invokes real M1/M2/M3/B/K/P preparation.
mod support {
    include!("support/retained_performance.rs");
}

use ql_mef::musical_performance_return::{ReturnContext, ReturnReference, bind_performance_return};
use ql_mef::nara::{EventBasisRefs, SourceRevision, domain::ProtectedRef, replay::NaraOccasion};
use ql_mef::performance_audio::{PreparedPerformanceBinding, prepare_native_performance};

fn reference(value: &str) -> ReturnReference {
    ReturnReference {
        reference: value.into(),
        revision: "1".into(),
    }
}

fn public_world() -> ReturnContext {
    ReturnContext {
        context: reference("reference:v293/ordinary-world"),
        receiver: reference("reference:v293/public-receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        kind: "world".into(),
        private: false,
        required_assets: vec![],
    }
}

fn original_episode(prepared: &PreparedPerformanceBinding) -> NaraOccasion {
    let event = EventBasisRefs::from_basis(prepared.native_basis()).unwrap();
    NaraOccasion {
        occasion_ref: "reference:v293/controlled-original-episode".into(),
        subject_id: event.subject_ref.clone(),
        event,
        personal_reception_generation: 9,
        identity_revision: "reference:v293/controlled-identity/1".into(),
        day_ref: "central:day:2026-10-02".into(),
        now_ref: "central:now:v293-controlled-return".into(),
        occurrence_at_unix_ms: 1000,
        receipt_at_unix_ms: 1001,
        protected_state_ref: ProtectedRef {
            ref_id: "reference:v293/controlled-protected-state".into(),
            revision: "1".into(),
            owner_ref: prepared.physical_body().subject_ref().into(),
        },
        activity_refs: vec![],
        oracle_packet_refs: vec![],
        transformation_phase_refs: vec![],
        context_reading_refs: vec!["reference:v293/personal-context".into()],
        integration_return_refs: vec![],
        expression_refs: vec!["expression:retained/current".into()],
        source_revisions: vec![SourceRevision {
            source_ref: "reference:v293/controlled-original-source".into(),
            revision: "1".into(),
            standing_ref: "reference:controlled-native-episode".into(),
        }],
    }
}

fn protected_context(episode: &NaraOccasion, shared: bool) -> ReturnContext {
    ReturnContext {
        context: reference("reference:v293/personal-context"),
        receiver: reference("reference:v293/controlled-receiver"),
        source_occasion: Some(reference(&episode.occasion_ref)),
        protected_state: Some(reference(&episode.protected_state_ref.ref_id)),
        consent: shared.then(|| reference("reference:v293/controlled-explicit-consent")),
        kind: if shared { "shared" } else { "personal" }.into(),
        private: true,
        required_assets: vec![],
    }
}

#[test]
fn v293_genuine_world_and_original_protected_episode_have_distinct_native_returns() {
    let prepared = prepare_native_performance(support::preparation()).unwrap();
    let world = bind_performance_return(&prepared, None, public_world(), 19).unwrap();
    let world_basis = world.expression_basis().unwrap();
    assert_eq!(world_basis["context"]["kind"], "world");
    assert_eq!(world_basis["context"]["private"], false);
    assert!(world_basis["context"]["protected_state"].is_null());
    assert!(world_basis["m4_episode"].is_null());
    let episode = original_episode(&prepared);
    let personal = bind_performance_return(
        &prepared,
        Some(episode.clone()),
        protected_context(&episode, false),
        19,
    )
    .unwrap();
    assert_eq!(personal.original_occasion(), Some(&episode));
    assert_eq!(
        personal.replay_score().unwrap().snapshot(),
        prepared.native_basis().m3
    );
    assert_eq!(
        personal.expression_basis().unwrap()["context"]["private"],
        true
    );
}

#[test]
fn v293_world_label_cannot_carry_the_original_protected_native_episode() {
    let prepared = prepare_native_performance(support::preparation()).unwrap();
    let episode = original_episode(&prepared);
    // The original personal native Return succeeds before changing only kind.
    let original = protected_context(&episode, false);
    bind_performance_return(&prepared, Some(episode.clone()), original.clone(), 19).unwrap();
    for private in [false, true] {
        let mut wrong = original.clone();
        wrong.kind = "world".into();
        wrong.private = private;
        assert!(
            bind_performance_return(&prepared, Some(episode.clone()), wrong, 19).is_err(),
            "World accepted an original protected episode with private={private}",
        );
    }
}

#[test]
fn v293_world_cannot_admit_hidden_private_or_consent_custody_without_an_episode() {
    let prepared = prepare_native_performance(support::preparation()).unwrap();
    bind_performance_return(&prepared, None, public_world(), 19).unwrap();
    for hidden in 0..3 {
        let mut wrong = public_world();
        match hidden {
            0 => wrong.private = true,
            1 => wrong.consent = Some(reference("reference:v293/controlled-explicit-consent")),
            _ => {
                wrong.protected_state = Some(reference("reference:v293/controlled-protected-state"))
            }
        }
        assert!(
            bind_performance_return(&prepared, None, wrong, 19).is_err(),
            "ordinary World accepted hidden custody case{hidden}"
        );
    }
}

#[test]
fn v293_shared_opt_in_retains_protected_custody_instead_of_public_relabelling() {
    let prepared = prepare_native_performance(support::preparation()).unwrap();
    let episode = original_episode(&prepared);
    let original = protected_context(&episode, true);
    let shared =
        bind_performance_return(&prepared, Some(episode.clone()), original.clone(), 19).unwrap();
    assert_eq!(shared.original_occasion(), Some(&episode));
    for lost in 0..2 {
        let mut wrong = original.clone();
        if lost == 0 {
            wrong.private = false;
        } else {
            wrong.protected_state = None;
        }
        assert!(
            bind_performance_return(&prepared, Some(episode.clone()), wrong, 19).is_err(),
            "shared native Return accepted lost protected custody case{lost}"
        );
    }
}
