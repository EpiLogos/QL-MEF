use ql_core::{ConjugationDegree, ExpansionSide, RelationFamily};
use ql_mef::{LensId, MusicalBasis, musical_completion_frame, musical_square};

#[test]
fn d_completion_cardinality_is_two_three_four() {
    let d1 = musical_completion_frame(
        MusicalBasis::Chromatic,
        LensId::L0,
        RelationFamily::A,
        0,
        ConjugationDegree::D1,
        None,
    )
    .expect("canonical D1 frame");
    assert_eq!(d1.coordinates.len(), 2);
    assert_eq!(d1.pitches.len(), 2);

    let d2 = musical_completion_frame(
        MusicalBasis::Chromatic,
        LensId::L0,
        RelationFamily::A,
        0,
        ConjugationDegree::D2,
        Some(ExpansionSide::Left),
    )
    .expect("canonical D2 frame");
    assert_eq!(d2.coordinates.len(), 3);
    assert_eq!(d2.pitches.len(), 3);

    let d3 = musical_completion_frame(
        MusicalBasis::Chromatic,
        LensId::L0,
        RelationFamily::A,
        0,
        ConjugationDegree::D3,
        None,
    )
    .expect("canonical D3 frame");
    assert_eq!(d3.coordinates.len(), 4);
    assert_eq!(d3.pitches.len(), 4);
}

#[test]
fn d3_completion_is_the_existing_musical_square() {
    let completion = musical_completion_frame(
        MusicalBasis::Fifths,
        LensId::L2Prime,
        RelationFamily::C,
        1,
        ConjugationDegree::D3,
        None,
    )
    .expect("canonical D3 frame");
    let square = musical_square(MusicalBasis::Fifths, LensId::L2Prime, RelationFamily::C, 1)
        .expect("canonical musical square");

    assert_eq!(completion.family, square.family);
    assert_eq!(completion.pair_index, square.pair_index);
    assert_eq!(completion.coordinates, square.coordinates.to_vec());
    assert_eq!(completion.pitches, square.pitches.to_vec());
}

#[test]
fn d2_requires_exactly_one_conjugate_expansion_side() {
    let error = musical_completion_frame(
        MusicalBasis::Chromatic,
        LensId::L0,
        RelationFamily::B,
        2,
        ConjugationDegree::D2,
        None,
    )
    .expect_err("D2 without a side must remain invalid");

    assert!(error.to_string().contains("exactly one projection side"));
}
