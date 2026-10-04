//! Regression for the actual CI7 Return failure. No producer stamp is rewritten
//! to make M2 and M3 equal; the current preparation owns their independent IDs.
#[path = "support/retained_performance.rs"]
mod retained_performance;
use ql_mef::nara::EventBasisRefs;
use ql_mef::performance_audio::prepare_native_performance;
use serde_json::{Value, json};

#[test]
fn actual_prepared_performance_retains_m2_one_m3_source_seven_and_post_command_eight() {
    let p = prepare_native_performance(retained_performance::preparation()).unwrap();
    let basis = p.native_basis();
    assert_eq!(basis.m2_input.stamp.identity.profile_generation, 1);
    assert_eq!(basis.input.m3.stamp.identity.profile_generation, 7);
    let refs = EventBasisRefs::from_basis(basis).unwrap();
    let versions = EventBasisRefs::native_generations(basis).unwrap();
    assert_eq!(refs.profile_generation, 1);
    assert_eq!(versions.m2_generation, 1);
    assert_eq!(versions.m3_source_generation, 7);
    assert_eq!(
        versions.m3_generation,
        p.physical_body().source_generation()
    );
    assert!(versions.m3_generation > versions.m3_source_generation);
    assert_eq!(versions.m1_revision, basis.input.m1.revision);
    // The legacy event receipt keeps its exact original v1 shape. This new
    // witness is saved beside the occasion, not injected into old source bytes.
    let encoded = serde_json::to_value(&refs).unwrap();
    assert_eq!(encoded.as_object().unwrap().len(), 9);
    assert!(encoded.get("m3_generation").is_none());
    let restored: EventBasisRefs = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(serde_json::to_value(restored).unwrap(), encoded);
}
#[test]
fn missing_join_or_tampered_post_command_state_and_receipts_cannot_pass_equal_event_labels() {
    let p = prepare_native_performance(retained_performance::preparation()).unwrap();
    for slot in 0..5 {
        let mut basis = p.native_basis().clone();
        match slot {
            0 => basis.input.m1.event_ref = "foreign:event".into(),
            1 => basis.input.m3.stamp.identity.event_ref = "foreign:event".into(),
            2 => basis.m3["identity"]["profile_generation"] = json!(7),
            3 => basis.m3["subject_ref"] = json!("foreign:subject"),
            _ => basis.m3_receipts[0]["after_generation"] = json!(999),
        }
        assert!(EventBasisRefs::from_basis(&basis).is_err(), "tamper {slot}");
        assert!(
            EventBasisRefs::native_generations(&basis).is_err(),
            "tamper {slot}"
        );
    }
}
#[test]
fn independently_versioned_source_defined_score_changes_witness_and_preserves_original_basis() {
    let first = retained_performance::preparation();
    let mut second = retained_performance::preparation();
    second.coupled.m3_commands[0]
        .operations
        .push(ql_mef::m3_state::M3Operation::AdvanceClock { steps: 1 });
    // Both preparations independently replay actual native commands and retain
    // the same metric body policy. No nearest-key/codon transcription occurs.
    let a = prepare_native_performance(first).unwrap();
    let b = prepare_native_performance(second).unwrap();
    assert_eq!(
        EventBasisRefs::from_basis(a.native_basis()).unwrap(),
        EventBasisRefs::from_basis(b.native_basis()).unwrap()
    );
    let av = EventBasisRefs::native_generations(a.native_basis()).unwrap();
    let bv = EventBasisRefs::native_generations(b.native_basis()).unwrap();
    assert_ne!(av.m3_state_sha256, bv.m3_state_sha256);
    assert_ne!(av.basis_sha256, bv.basis_sha256);
    let original: Value = serde_json::to_value(a.native_basis()).unwrap();
    assert_eq!(
        serde_json::to_value(a.native_basis().input.compose().unwrap()).unwrap(),
        original
    );
}
