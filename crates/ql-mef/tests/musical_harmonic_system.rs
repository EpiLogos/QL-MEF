use std::collections::HashSet;

use ql_core::{KernelRelationId, QlCoordinate, QlFace, QlFamily, QlPosition, RelationFamily};
use ql_mef::{
    ALL_PITCH_CLASSES, AUTHORED_INTERVAL_REFERENCES, CANONICAL_RATIOS, CrossOperator,
    FIRST_SPANDA_HORIZONTAL, IONIAN_OFFSETS, KERNEL_FAMILY_RELATION, MAJOR_MINOR_CHARACTER_DEGREES,
    MUSICAL_DERIVATION_SOURCE_BLOB, MUSICAL_DERIVATION_SOURCE_PATH, MUSICAL_DERIVATION_VENDOR_COMMIT,
    ModeKind, MusicalBasis, NAME_CONTENT, POWER_CONTENT, SECOND_SPANDA_VERTICAL, c_p_l_family_views,
    cf_diatonic_cut, cross_interval_deltas, d3_interval_deltas, d3_relation_id, derive_pre_m_music,
    explicate_coordinates, implicate_coordinates, lens_anchor, lens_anchors, lens_kernel_coordinate,
    mode_tonic_instance, mode_tonic_landscape, musical_square, musical_squares, pair_interval_deltas,
    spanda_cross_reading_ratios, HarmonicRatio, LensId, ContextFrameId, MefUnitFace,
};

fn pos(value: u8) -> QlPosition { QlPosition::new(value).expect("canonical test position") }

#[test]
fn vendored_v3_and_kernel_heads_are_pinned() {
    assert_eq!(MUSICAL_DERIVATION_SOURCE_PATH, "docs/sources/ql-musical-derivation-v3.md");
    assert_eq!(MUSICAL_DERIVATION_SOURCE_BLOB, "6414c56c6241c3da46e1ea6fdcd7a09b6b66c5aa");
    assert_eq!(MUSICAL_DERIVATION_VENDOR_COMMIT, "9429a9fb5173f32138799046e8e2a4d7a2d86968");
    let fixture = include_str!("../../../fixtures/music/source-provenance-v1.tsv");
    assert!(fixture.contains(MUSICAL_DERIVATION_SOURCE_BLOB));
    assert!(fixture.contains("5c781e87fa321b85c5d799d33d3eea7210d4ea58"));
    assert!(fixture.contains("e5f612653c71c8a9ae74199ef2e8b326e3e9deba"));
}

#[test]
fn spanda_and_ratio_chain_are_exact() {
    assert_eq!(FIRST_SPANDA_HORIZONTAL, (3, 3));
    assert_eq!(SECOND_SPANDA_VERTICAL, (4, 2));
    assert_eq!(spanda_cross_reading_ratios(), [
        HarmonicRatio::new(4, 3).unwrap(), HarmonicRatio::new(2, 3).unwrap(),
        HarmonicRatio::new(3, 4).unwrap(), HarmonicRatio::new(3, 2).unwrap(),
    ]);
    assert_eq!(CANONICAL_RATIOS.len(), 8);
    let fourth = HarmonicRatio::new(4, 3).unwrap();
    let fifth = HarmonicRatio::new(3, 2).unwrap();
    let totality = HarmonicRatio::new(16, 9).unwrap();
    let tick = HarmonicRatio::new(9, 8).unwrap();
    let octave = HarmonicRatio::new(2, 1).unwrap();
    assert_eq!(fourth.multiply(fifth), octave);
    assert_eq!(fifth.divide(fourth), tick);
    assert_eq!(octave.divide(totality), tick);
    assert_eq!(totality.multiply(tick), octave);
    assert_eq!(fourth.multiply(tick).multiply(fourth), octave);
    assert_eq!(fourth.reciprocal(), HarmonicRatio::new(3, 4).unwrap());
    assert_eq!(fifth.reciprocal(), HarmonicRatio::new(2, 3).unwrap());
}

#[test]
fn both_bases_generate_the_same_twelve_pitch_substrate() {
    assert_eq!(MusicalBasis::Chromatic.generator_ratio(), HarmonicRatio::new(9, 8).unwrap());
    assert_eq!(MusicalBasis::Fifths.generator_ratio(), HarmonicRatio::new(3, 2).unwrap());
    assert_eq!(MusicalBasis::Chromatic.helix(QlFace::Direct), [0, 2, 4, 6, 8, 10]);
    assert_eq!(MusicalBasis::Chromatic.helix(QlFace::Conjugate), [1, 3, 5, 7, 9, 11]);
    assert_eq!(MusicalBasis::Fifths.helix(QlFace::Direct), [0, 7, 2, 9, 4, 11]);
    assert_eq!(MusicalBasis::Fifths.helix(QlFace::Conjugate), [6, 1, 8, 3, 10, 5]);
    for basis in MusicalBasis::ALL {
        assert_eq!(basis.substrate().into_iter().collect::<HashSet<_>>(), ALL_PITCH_CLASSES.into_iter().collect());
    }
    assert_eq!(MusicalBasis::Chromatic.conjugate_axis_semitones(), 1);
    assert_eq!(MusicalBasis::Fifths.conjugate_axis_semitones(), 6);
}

#[test]
fn p_pprime_l_lprime_and_c_cprime_share_the_kernel_address_field() {
    assert_eq!(NAME_CONTENT, ["Truth", "Mind", "Word", "Logos", "Son", "Image"]);
    assert_eq!(POWER_CONTENT, ["Play", "Need", "Sacrifice", "Decision", "Love", "Work"]);
    assert_eq!(KERNEL_FAMILY_RELATION, KernelRelationId::FamilySamePosition);
    let coordinate = QlCoordinate::new(pos(2), QlFace::Conjugate);
    let views = c_p_l_family_views(coordinate);
    assert_eq!(views.map(|view| view.family), [QlFamily::C, QlFamily::P, QlFamily::L]);
    assert!(views.iter().all(|view| view.coordinate == coordinate));
    assert_eq!(lens_kernel_coordinate(LensId::L2Prime), coordinate);
    assert_eq!(lens_kernel_coordinate(LensId::L4), QlCoordinate::new(pos(4), QlFace::Direct));
}

#[test]
fn abc_d1_d2_d3_are_kernel_operators_with_basis_specific_pitch_realisations() {
    assert_eq!(pair_interval_deltas(MusicalBasis::Chromatic, RelationFamily::A, QlFace::Direct), [2,2,2]);
    assert_eq!(pair_interval_deltas(MusicalBasis::Chromatic, RelationFamily::B, QlFace::Direct), [2,2,2]);
    assert_eq!(pair_interval_deltas(MusicalBasis::Chromatic, RelationFamily::C, QlFace::Direct), [10,6,2]);
    assert_eq!(pair_interval_deltas(MusicalBasis::Fifths, RelationFamily::A, QlFace::Direct), [7,7,7]);
    assert_eq!(pair_interval_deltas(MusicalBasis::Fifths, RelationFamily::B, QlFace::Direct), [7,7,1]);
    assert_eq!(pair_interval_deltas(MusicalBasis::Fifths, RelationFamily::C, QlFace::Direct), [11,9,7]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Chromatic, CrossOperator::SamePosition), [1,1,1,1,1,1]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Fifths, CrossOperator::SamePosition), [6,6,6,6,6,6]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Chromatic, CrossOperator::Transform), [3,3,3,3,3,3]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Chromatic, CrossOperator::Require), [11,11,11,11,11,11]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Chromatic, CrossOperator::Complete), [11,7,3,11,7,3]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Fifths, CrossOperator::Transform), [1,1,1,1,1,7]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Fifths, CrossOperator::Require), [5,11,11,11,11,11]);
    assert_eq!(cross_interval_deltas(MusicalBasis::Fifths, CrossOperator::Complete), [5,3,1,11,9,7]);
    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        assert_eq!(d3_interval_deltas(MusicalBasis::Chromatic, family), pair_interval_deltas(MusicalBasis::Chromatic, family, QlFace::Direct));
        assert_eq!(d3_interval_deltas(MusicalBasis::Fifths, family), pair_interval_deltas(MusicalBasis::Fifths, family, QlFace::Direct));
    }
    assert_eq!(d3_relation_id(RelationFamily::A), KernelRelationId::ConjugateInvarianceA);
    assert_eq!(d3_relation_id(RelationFamily::B), KernelRelationId::ConjugateInvarianceB);
    assert_eq!(d3_relation_id(RelationFamily::C), KernelRelationId::ConjugateInvarianceC);
    assert_eq!(AUTHORED_INTERVAL_REFERENCES[6].chromatic, "11, 9, 3, 1, 6, 3 st");
    assert_eq!(AUTHORED_INTERVAL_REFERENCES[6].fifths, "5, 3, 1, 1, 3, 6 st");
}

#[test]
fn all_twelve_lenses_are_derived_as_tonic_anchors() {
    let chromatic = [
        (LensId::L0,0),(LensId::L1,2),(LensId::L2,4),(LensId::L3,6),(LensId::L4,8),(LensId::L5,10),
        (LensId::L0Prime,1),(LensId::L1Prime,3),(LensId::L2Prime,5),(LensId::L3Prime,7),(LensId::L4Prime,9),(LensId::L5Prime,11),
    ];
    let fifths = [
        (LensId::L0,0),(LensId::L1,7),(LensId::L2,2),(LensId::L3,9),(LensId::L4,4),(LensId::L5,11),
        (LensId::L0Prime,6),(LensId::L1Prime,1),(LensId::L2Prime,8),(LensId::L3Prime,3),(LensId::L4Prime,10),(LensId::L5Prime,5),
    ];
    for (lens, pitch) in chromatic { assert_eq!(lens_anchor(MusicalBasis::Chromatic, lens).pitch, pitch); }
    for (lens, pitch) in fifths { assert_eq!(lens_anchor(MusicalBasis::Fifths, lens).pitch, pitch); }
    for basis in MusicalBasis::ALL {
        assert_eq!(lens_anchors(basis).into_iter().map(|a| a.pitch).collect::<HashSet<_>>().len(), 12);
    }
}

#[test]
fn eight_plus_four_and_three_by_three_squares_are_derived_from_kernel_pairs() {
    assert_eq!(explicate_coordinates().len(), 8);
    assert_eq!(implicate_coordinates().len(), 4);
    assert!(explicate_coordinates().iter().all(|c| matches!(c.position.value(), 1..=4)));
    assert!(implicate_coordinates().iter().all(|c| matches!(c.position.value(), 0 | 5)));
    assert_eq!(musical_squares(MusicalBasis::Chromatic, LensId::L0).len(), 9);
    assert_eq!(musical_square(MusicalBasis::Chromatic, LensId::L0, RelationFamily::A, 0).unwrap().pitches, [0,2,1,3]);
    assert_eq!(musical_square(MusicalBasis::Chromatic, LensId::L0, RelationFamily::C, 1).unwrap().pitches, [2,8,3,9]);
    assert_eq!(musical_square(MusicalBasis::Chromatic, LensId::L0, RelationFamily::A, 1).unwrap().pitches,
               musical_square(MusicalBasis::Chromatic, LensId::L0, RelationFamily::C, 2).unwrap().pitches);
    assert_eq!(musical_square(MusicalBasis::Fifths, LensId::L0, RelationFamily::A, 0).unwrap().pitches, [0,7,6,1]);
    assert_eq!(musical_square(MusicalBasis::Fifths, LensId::L0, RelationFamily::C, 1).unwrap().pitches, [7,4,1,10]);
}

#[test]
fn context_frames_cut_the_reference_diatonic_and_all_seven_modes() {
    let cut = cf_diatonic_cut(MusicalBasis::Chromatic, LensId::L0);
    assert_eq!(cut.lens_tonic, 0);
    assert_eq!(cut.pitches, [0,2,4,5,7,9,11]);
    assert_eq!(cut.frames, ContextFrameId::ALL);
    assert_eq!(cut.forms, [MefUnitFace::Name,MefUnitFace::Name,MefUnitFace::Name,MefUnitFace::Power,MefUnitFace::Power,MefUnitFace::Power,MefUnitFace::Power]);
    assert_eq!(IONIAN_OFFSETS, [0,2,4,5,7,9,11]);
    let dorian = mode_tonic_instance(MusicalBasis::Chromatic, LensId::L0, ModeKind::Dorian);
    assert_eq!(dorian.tonic, 2);
    assert_eq!(dorian.context_frame, ContextFrameId::Cf2);
    assert_eq!(dorian.pitches, [2,4,5,7,9,11,0]);
    assert_eq!(ModeKind::Lydian.relative_offsets(), [0,2,4,6,7,9,11]);
    assert_eq!(ModeKind::Aeolian.relative_offsets(), [0,2,3,5,7,8,10]);
    assert_eq!(ModeKind::Ionian.form_pattern(), [MefUnitFace::Name,MefUnitFace::Name,MefUnitFace::Name,MefUnitFace::Power,MefUnitFace::Power,MefUnitFace::Power,MefUnitFace::Power]);
    assert_eq!(ModeKind::Locrian.form_pattern(), [MefUnitFace::Name,MefUnitFace::Power,MefUnitFace::Power,MefUnitFace::Name,MefUnitFace::Power,MefUnitFace::Name,MefUnitFace::Name]);
}

#[test]
fn major_minor_selection_and_complete_84_field_are_executable() {
    assert_eq!(MAJOR_MINOR_CHARACTER_DEGREES.map(|d| d.degree), [3,6,7]);
    assert_eq!(MAJOR_MINOR_CHARACTER_DEGREES.map(|d| d.major_offset), [4,9,11]);
    assert_eq!(MAJOR_MINOR_CHARACTER_DEGREES.map(|d| d.minor_offset), [3,8,10]);
    for basis in MusicalBasis::ALL {
        let landscape = mode_tonic_landscape(basis);
        assert_eq!(landscape.len(), 84);
        assert_eq!(landscape.iter().copied().collect::<HashSet<_>>().len(), 84);
        for lens in LensId::ALL { assert_eq!(landscape.iter().filter(|entry| entry.lens == lens).count(), 7); }
        for mode in ModeKind::ALL { assert_eq!(landscape.iter().filter(|entry| entry.mode == mode).count(), 12); }
    }
}

#[test]
fn full_pre_m_derivation_is_inspectable_from_each_basis() {
    for basis in MusicalBasis::ALL {
        let d = derive_pre_m_music(basis);
        assert_eq!(d.direct_helix, basis.helix(QlFace::Direct));
        assert_eq!(d.conjugate_helix, basis.helix(QlFace::Conjugate));
        assert_eq!(d.lens_anchors.len(), 12);
        assert_eq!(d.explicate_coordinates.len(), 8);
        assert_eq!(d.implicate_coordinates.len(), 4);
        assert_eq!(d.lens_zero_squares.len(), 9);
        assert_eq!(d.mode_tonic_landscape.len(), 84);
    }
}
