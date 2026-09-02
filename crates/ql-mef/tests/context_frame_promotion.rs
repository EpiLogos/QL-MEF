use std::collections::HashSet;

use ql_mef::{
    CONTEXT_FRAME_GRAMMAR_VERSION, ContextFrameCut, ContextFrameId, LensId, MefGrain, MefUnitFace,
    canonical_context_frame_progression,
};

#[test]
fn fixture_matches_the_complete_canonical_cf_partition() {
    let fixture = include_str!("../../../fixtures/q6/context-frame-cut-v1.tsv");
    let rows: Vec<Vec<&str>> = fixture
        .lines()
        .skip(1)
        .filter(|row| !row.is_empty())
        .map(|row| row.split('\t').collect())
        .collect();

    assert_eq!(rows.len(), 12);

    let progression = canonical_context_frame_progression();
    for (row, selection) in rows[..7].iter().zip(progression) {
        assert_eq!(row.len(), 5);
        assert_eq!(row[0], "selected");
        assert_eq!(row[1], selection.frame().code());
        assert_eq!(row[2], selection.local_position().value().to_string());
        assert_eq!(row[3], face_name(selection.unit_face()));
        assert_eq!(row[4], grain_name(selection.grain()));
    }

    let cut = ContextFrameCut::canonical(LensId::L0);
    for (row, coordinate) in rows[7..10].iter().zip(cut.complexification_hooks()) {
        assert_eq!(row[0], "hook");
        assert_eq!(row[1], "-");
        assert_eq!(row[2], coordinate.local_position().value().to_string());
        assert_eq!(row[3], face_name(coordinate.unit_face()));
        assert_eq!(row[4], grain_name(coordinate.grain()));
    }
    for (row, coordinate) in rows[10..].iter().zip(cut.unpicked_outer_anchors()) {
        assert_eq!(row[0], "outer-anchor");
        assert_eq!(row[1], "-");
        assert_eq!(row[2], coordinate.local_position().value().to_string());
        assert_eq!(row[3], face_name(coordinate.unit_face()));
        assert_eq!(row[4], grain_name(coordinate.grain()));
    }
}

#[test]
fn canonical_progression_is_three_names_then_conjugate_cross_then_four_powers() {
    let progression = canonical_context_frame_progression();
    let expected = [
        (
            ContextFrameId::Cf1,
            0,
            MefUnitFace::Name,
            MefGrain::OuterTwo,
        ),
        (
            ContextFrameId::Cf2,
            1,
            MefUnitFace::Name,
            MefGrain::InnerFour,
        ),
        (
            ContextFrameId::Cf3,
            2,
            MefUnitFace::Name,
            MefGrain::InnerFour,
        ),
        (
            ContextFrameId::Cf4,
            2,
            MefUnitFace::Power,
            MefGrain::InnerFour,
        ),
        (
            ContextFrameId::Cf5,
            3,
            MefUnitFace::Power,
            MefGrain::InnerFour,
        ),
        (
            ContextFrameId::Cf6,
            4,
            MefUnitFace::Power,
            MefGrain::InnerFour,
        ),
        (
            ContextFrameId::Cf7,
            5,
            MefUnitFace::Power,
            MefGrain::OuterTwo,
        ),
    ];

    for (selection, (frame, position, face, grain)) in progression.into_iter().zip(expected) {
        assert_eq!(selection.frame(), frame);
        assert_eq!(selection.local_position().value(), position);
        assert_eq!(selection.unit_face(), face);
        assert_eq!(selection.grain(), grain);
    }

    let inner = progression
        .iter()
        .filter(|selection| selection.grain() == MefGrain::InnerFour)
        .count();
    let outer = progression.len() - inner;
    assert_eq!((inner, outer), (5, 2));
}

#[test]
fn every_lens_cut_partitions_all_twelve_form_addresses_once() {
    for lens in LensId::ALL {
        let cut = ContextFrameCut::canonical(lens);
        assert_eq!(cut.lens(), lens);

        let mut addresses = HashSet::new();
        for selected in cut.selected() {
            let coordinate = selected.coordinate();
            assert_eq!(
                selected.frame().canonical_selection().at_lens(lens),
                *selected
            );
            assert_rotation(
                lens,
                coordinate.local_position().value(),
                coordinate.absolute_position().value(),
            );
            assert!(
                addresses.insert((coordinate.local_position().value(), coordinate.unit_face()))
            );
        }
        for coordinate in cut.complexification_hooks() {
            assert_eq!(coordinate.grain(), MefGrain::InnerFour);
            assert_rotation(
                lens,
                coordinate.local_position().value(),
                coordinate.absolute_position().value(),
            );
            assert!(
                addresses.insert((coordinate.local_position().value(), coordinate.unit_face()))
            );
        }
        for coordinate in cut.unpicked_outer_anchors() {
            assert_eq!(coordinate.grain(), MefGrain::OuterTwo);
            assert_rotation(
                lens,
                coordinate.local_position().value(),
                coordinate.absolute_position().value(),
            );
            assert!(
                addresses.insert((coordinate.local_position().value(), coordinate.unit_face()))
            );
        }

        assert_eq!(addresses.len(), 12);
        for position in 0..6 {
            assert!(addresses.contains(&(position, MefUnitFace::Name)));
            assert!(addresses.contains(&(position, MefUnitFace::Power)));
        }
    }
}

#[test]
fn day_night_twins_share_the_same_cf_cut_coordinates() {
    for pair in LensId::ALL.chunks_exact(2) {
        let day = ContextFrameCut::canonical(pair[0]);
        let night = ContextFrameCut::canonical(pair[1]);

        for (day_selection, night_selection) in day.selected().iter().zip(night.selected()) {
            assert_eq!(day_selection.frame(), night_selection.frame());
            assert_eq!(
                day_selection.coordinate().absolute_position(),
                night_selection.coordinate().absolute_position()
            );
            assert_eq!(
                day_selection.coordinate().unit_face(),
                night_selection.coordinate().unit_face()
            );
        }
    }
}

#[test]
fn promotion_manifest_blocks_unsupported_cf_semantics() {
    let manifest = include_str!("../../../fixtures/q6/context-frame-promotion-v1.json");

    assert_eq!(CONTEXT_FRAME_GRAMMAR_VERSION, "1.0.0");
    assert!(manifest.contains("\"capability\": \"ql.mef.context-frame\""));
    assert!(manifest.contains("\"status\": \"specified-formal-structure\""));
    assert!(manifest.contains("\"automaticInvocation\": false"));
    assert!(manifest.contains("\"canonicalSelection\": \"3 Name -> conjugate-cross -> 4 Power\""));
    assert!(manifest.contains("\"alternateConjugateSelection\": \"not-promoted\""));
    assert!(manifest.contains("\"modalReanchoring\": \"not-promoted\""));
    assert!(manifest.contains("\"semanticRoleBinding\": \"not-promoted\""));
    assert!(manifest.contains("\"runtimePolicy\": \"not-promoted\""));
    assert!(manifest.contains("\"completeFormAddresses\": 12"));

    for source_hash in [
        "61508658f871f232f554127052d683ecded15c28694cc79a110bcf7841492699",
        "1e1ee72c6445eea2e7df057798bd1b0441f6e5eeac289dddb65d680d7ca6bb51",
        "a674e3c0489b1e9bd27078a2ec6c2a0cb1158ea8e37628a9ed36a429c1ab454f",
    ] {
        assert!(manifest.contains(source_hash));
    }
}

fn assert_rotation(lens: LensId, local: u8, absolute: u8) {
    assert_eq!(absolute, (lens.index() + local) % 6);
}

fn face_name(face: MefUnitFace) -> &'static str {
    match face {
        MefUnitFace::Name => "Name",
        MefUnitFace::Power => "Power",
    }
}

fn grain_name(grain: MefGrain) -> &'static str {
    match grain {
        MefGrain::InnerFour => "InnerFour",
        MefGrain::OuterTwo => "OuterTwo",
    }
}
