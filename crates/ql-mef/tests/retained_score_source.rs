#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::performance::PerformanceOwner;
#[test]
fn actual_owner_borrows_complete_sparse_source_without_closing_missing_keys() {
    let (current, config) = source::config(true);
    let owner =
        PerformanceOwner::prepare(&current, "controlled:borrowed-score-source", config).unwrap();
    let (targets, consumer) = owner.source_key_consumer(&current).unwrap().unwrap();
    assert!(consumer.condition.is_some());
    targets
        .validate_coupled_consumer(consumer, targets.collection())
        .unwrap();
    let mut available = 0;
    for key in 0..12 {
        let selected = targets
            .key_target(key, 0, "native:borrowed-score-source/touch")
            .unwrap();
        available += usize::from(selected.note().is_some());
        assert_eq!(
            selected.note().is_some(),
            [0, 2, 4, 5, 7, 9, 11].contains(&key)
        );
    }
    assert_eq!(available, 7);
    assert!(targets.audio_octet_targets(0).is_err());
    let retained = targets.preparation_receipt().unwrap();
    assert_eq!(retained, owner.source_assets()["source_key_preparation"]);
    let mut changed_input = current.input.clone();
    changed_input.m1.tick12 = (changed_input.m1.tick12 + 1) % 12;
    let changed = changed_input.compose().unwrap();
    assert!(owner.source_key_consumer(&changed).is_err());
}
#[test]
fn declared_twelve_key_owner_does_not_fabricate_sparse_source() {
    let (current, config) = source::config(false);
    let owner =
        PerformanceOwner::prepare(&current, "controlled:declared-twelve-source", config).unwrap();
    assert!(owner.source_key_consumer(&current).unwrap().is_none());
}
