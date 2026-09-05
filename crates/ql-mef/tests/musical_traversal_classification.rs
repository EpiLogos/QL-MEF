use ql_core::{ConjugationDegree, ExpansionSide, QlFace, QlPosition, RelationFamily};
use ql_mef::{LensId, MusicalBasis, TraversalExpansionSide, classify_musical_traversal};

fn p(value: u8) -> QlPosition {
    QlPosition::new(value).expect("test position is canonical")
}

#[test]
fn overlapping_traversal_returns_every_valid_musical_candidate() {
    let candidates = classify_musical_traversal(
        MusicalBasis::Chromatic,
        LensId::L0,
        p(2),
        p(3),
        ConjugationDegree::D1,
        None,
    )
    .unwrap();

    assert_eq!(candidates.len(), 2);
    assert!(candidates.iter().any(|candidate| {
        candidate.relation.family == RelationFamily::A
            && candidate.relation.pair_index == 1
            && candidate.frame.coordinates.len() == 2
    }));
    assert!(candidates.iter().any(|candidate| {
        candidate.relation.family == RelationFamily::C
            && candidate.relation.pair_index == 2
            && candidate.frame.coordinates.len() == 2
    }));
}

#[test]
fn noncanonical_traversal_stays_unclassified() {
    let candidates = classify_musical_traversal(
        MusicalBasis::Fifths,
        LensId::L0,
        p(0),
        p(2),
        ConjugationDegree::D1,
        None,
    )
    .unwrap();

    assert!(candidates.is_empty());
}

#[test]
fn d2_source_expansion_survives_reverse_traversal() {
    let candidates = classify_musical_traversal(
        MusicalBasis::Chromatic,
        LensId::L0,
        p(5),
        p(4),
        ConjugationDegree::D2,
        Some(TraversalExpansionSide::Source),
    )
    .unwrap();

    assert_eq!(candidates.len(), 1);
    let candidate = &candidates[0];
    assert_eq!(candidate.relation.family, RelationFamily::A);
    assert_eq!(candidate.relation.pair_index, 2);
    assert!(candidate.relation.reversed);
    assert_eq!(candidate.frame.expansion_side, Some(ExpansionSide::Right));
    assert_eq!(candidate.frame.coordinates.len(), 3);
    assert!(
        candidate.frame.coordinates.iter().any(|coordinate| {
            coordinate.position == p(5) && coordinate.face == QlFace::Conjugate
        })
    );
}

#[test]
fn d2_target_expansion_survives_reverse_traversal() {
    let candidates = classify_musical_traversal(
        MusicalBasis::Chromatic,
        LensId::L0,
        p(5),
        p(4),
        ConjugationDegree::D2,
        Some(TraversalExpansionSide::Target),
    )
    .unwrap();

    assert_eq!(
        candidates[0].frame.expansion_side,
        Some(ExpansionSide::Left)
    );
    assert!(
        candidates[0].frame.coordinates.iter().any(|coordinate| {
            coordinate.position == p(4) && coordinate.face == QlFace::Conjugate
        })
    );
}

#[test]
fn d3_preserves_direction_sensitive_overlap() {
    let candidates = classify_musical_traversal(
        MusicalBasis::Fifths,
        LensId::L0,
        p(0),
        p(5),
        ConjugationDegree::D3,
        None,
    )
    .unwrap();

    assert_eq!(candidates.len(), 2);
    assert!(candidates.iter().any(|candidate| {
        candidate.relation.family == RelationFamily::B
            && candidate.relation.pair_index == 2
            && candidate.relation.reversed
            && candidate.frame.coordinates.len() == 4
    }));
    assert!(candidates.iter().any(|candidate| {
        candidate.relation.family == RelationFamily::C
            && candidate.relation.pair_index == 0
            && !candidate.relation.reversed
            && candidate.frame.coordinates.len() == 4
    }));
}
