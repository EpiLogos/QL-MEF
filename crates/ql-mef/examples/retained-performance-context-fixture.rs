//! Controlled native occasions exercise custody; no real person is invented.
use ql_mef::continuous::performance::PerformanceOwner;
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference, bind_performance_return};
use ql_mef::nara::{EventBasisRefs, SourceRevision, domain::ProtectedRef, replay::NaraOccasion};
use serde_json::json;
#[path = "../tests/support/retained_source_performance.rs"]
mod retained_source_performance;
fn reference(value: &str) -> ReturnReference {
    ReturnReference {
        reference: value.into(),
        revision: "1".into(),
    }
}
fn main() -> Result<(), String> {
    let (current, config) = retained_source_performance::config(true);
    let owner = PerformanceOwner::prepare(&current, "expression:retained/current", config)?;
    let mut variants = Vec::new();
    for (suffix, shared) in [("one", false), ("two", false), ("shared", true)] {
        let event = EventBasisRefs::from_basis(owner.binding().native_basis())?;
        let occasion = NaraOccasion {
            occasion_ref: format!("controlled:source-context/occasion/{suffix}"),
            subject_id: event.subject_ref.clone(),
            event,
            personal_reception_generation: 7,
            identity_revision: format!("controlled:source-context/identity/{suffix}"),
            day_ref: "central:day/2026-10-02".into(),
            now_ref: "central:now/controlled-source-context".into(),
            occurrence_at_unix_ms: 1000,
            receipt_at_unix_ms: 1001,
            protected_state_ref: ProtectedRef {
                ref_id: format!("controlled:source-context/protected/{suffix}"),
                revision: "1".into(),
                owner_ref: "controlled:source-context/owner".into(),
            },
            activity_refs: vec![],
            oracle_packet_refs: vec![],
            transformation_phase_refs: vec![],
            context_reading_refs: vec![format!("controlled:source-context/context/{suffix}")],
            integration_return_refs: vec![],
            expression_refs: vec!["expression:retained/current".into()],
            source_revisions: vec![SourceRevision {
                source_ref: "controlled:source-context/original-event".into(),
                revision: "1".into(),
                standing_ref: "controlled:native-test-episode".into(),
            }],
        };
        let context = ReturnContext {
            context: reference(&occasion.context_reading_refs[0]),
            receiver: reference("controlled:source-context/receiver"),
            source_occasion: Some(reference(&occasion.occasion_ref)),
            protected_state: Some(reference(&occasion.protected_state_ref.ref_id)),
            consent: shared.then(|| reference("controlled:source-context/explicit-consent")),
            kind: if shared { "shared" } else { "personal" }.into(),
            private: true,
            required_assets: vec![],
        };
        let returned =
            bind_performance_return(owner.binding(), Some(occasion.clone()), context, 73)?;
        variants.push(json!({"name":suffix,"basis":returned.expression_basis()?,"original_occasion":occasion}));
    }
    println!("{}",serde_json::to_string(&json!({"schema":"ql.retained-performance-context-fixture/v1", "standing":"controlled-native-Return-episodes; no-real-person-consent-or-public-source-classification", "source_assets":owner.source_assets(), "variants":variants})).map_err(|e|e.to_string())?);
    Ok(())
}
