use ql_core::{QlPosition, RelationFamily, classify_relation_pair};

fn p(value: u8) -> QlPosition {
    QlPosition::new(value).expect("test position is canonical")
}

#[test]
fn every_declared_family_pair_is_recovered() {
    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        for (pair_index, (left, right)) in family.pairs().into_iter().enumerate() {
            let matches = classify_relation_pair(p(left), p(right));
            assert!(matches.iter().any(|candidate| {
                candidate.family == family
                    && candidate.pair_index == pair_index as u8
                    && !candidate.reversed
            }));
        }
    }
}

#[test]
fn classifier_preserves_real_family_overlap() {
    let zero_five = classify_relation_pair(p(0), p(5));
    assert_eq!(zero_five.len(), 2);
    assert!(
        zero_five
            .iter()
            .any(|m| m.family == RelationFamily::B && m.pair_index == 2 && m.reversed)
    );
    assert!(
        zero_five
            .iter()
            .any(|m| m.family == RelationFamily::C && m.pair_index == 0 && !m.reversed)
    );

    let two_three = classify_relation_pair(p(2), p(3));
    assert_eq!(two_three.len(), 2);
    assert!(
        two_three
            .iter()
            .any(|m| m.family == RelationFamily::A && m.pair_index == 1)
    );
    assert!(
        two_three
            .iter()
            .any(|m| m.family == RelationFamily::C && m.pair_index == 2)
    );
}

#[test]
fn reverse_traversal_preserves_family_and_marks_direction() {
    let forward = classify_relation_pair(p(4), p(5));
    assert_eq!(forward.len(), 1);
    assert_eq!(forward[0].family, RelationFamily::A);
    assert_eq!(forward[0].pair_index, 2);
    assert!(!forward[0].reversed);

    let reverse = classify_relation_pair(p(5), p(4));
    assert_eq!(reverse.len(), 1);
    assert_eq!(reverse[0].family, RelationFamily::A);
    assert_eq!(reverse[0].pair_index, 2);
    assert!(reverse[0].reversed);
}

#[test]
fn noncanonical_pairs_remain_unclassified() {
    assert!(classify_relation_pair(p(0), p(2)).is_empty());
    assert!(classify_relation_pair(p(1), p(3)).is_empty());
    assert!(classify_relation_pair(p(4), p(4)).is_empty());
}
