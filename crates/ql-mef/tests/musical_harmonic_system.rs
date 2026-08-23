use std::collections::HashSet;

use ql_mef::{
    ContextFrameId, EPOGDOON_FOLD_SEMANTICS, HARMONIC_RELATIONS, HarmonicRatio, LensId,
    MusicalEvidenceClass, epogdoon_72_to_64, epogdoon_preimage_width,
    tonic_context_frame_landscape,
};

#[test]
fn harmonic_fixture_is_exactly_the_promoted_m1_ratio_vocabulary() {
    let fixture = include_str!("../../../fixtures/music/harmonic-ratios-v1.tsv");
    let rows: Vec<&str> = fixture.lines().skip(1).collect();
    assert_eq!(rows.len(), HARMONIC_RELATIONS.len());

    for (row, relation) in rows.into_iter().zip(HARMONIC_RELATIONS) {
        let columns: Vec<&str> = row.split('\t').collect();
        assert_eq!(columns.len(), 6);
        assert_eq!(columns[0], relation.id);
        assert_eq!(columns[1].parse::<u32>().ok(), Some(relation.ratio.numerator()));
        assert_eq!(columns[2].parse::<u32>().ok(), Some(relation.ratio.denominator()));
        assert_eq!(columns[3], relation.evidence.as_str());
        assert_eq!(columns[4], relation.provenance_ref);
        assert_eq!(columns[5], relation.musical_consequence);
    }
}

#[test]
fn authored_ratio_products_close_exactly() {
    let four_three = HarmonicRatio::new(4, 3).expect("valid ratio");
    let three_two = HarmonicRatio::new(3, 2).expect("valid ratio");
    let sixteen_nine = HarmonicRatio::new(16, 9).expect("valid ratio");
    let nine_eight = HarmonicRatio::new(9, 8).expect("valid ratio");
    let two_one = HarmonicRatio::new(2, 1).expect("valid ratio");

    assert_eq!(four_three.multiply(three_two), two_one);
    assert_eq!(three_two.divide(four_three), nine_eight);
    assert_eq!(two_one.divide(sixteen_nine), nine_eight);
    assert_eq!(sixteen_nine.multiply(nine_eight), two_one);
    assert_eq!(four_three.multiply(nine_eight).multiply(four_three), two_one);
    assert_eq!(four_three.reciprocal(), HarmonicRatio::new(3, 4).expect("valid ratio"));
    assert_eq!(three_two.reciprocal(), HarmonicRatio::new(2, 3).expect("valid ratio"));
}

#[test]
fn epogdoon_fixture_proves_the_exact_72_to_64_floor_map() {
    let fixture = include_str!("../../../fixtures/music/epogdoon-72-to-64-v1.tsv");
    let rows: Vec<&str> = fixture.lines().skip(1).collect();
    assert_eq!(rows.len(), 72);

    for row in rows {
        let columns: Vec<&str> = row.split('\t').collect();
        assert_eq!(columns.len(), 2);
        let source = columns[0].parse::<u8>().expect("fixture source index");
        let target = columns[1].parse::<u8>().expect("fixture target index");
        assert_eq!(epogdoon_72_to_64(source), Some(target));
    }

    assert_eq!(epogdoon_72_to_64(72), None);
    assert_eq!(epogdoon_preimage_width(64), None);
}

#[test]
fn epogdoon_map_has_64_targets_and_exactly_8_collision_targets() {
    let targets: HashSet<u8> = (0_u8..72)
        .map(|source| epogdoon_72_to_64(source).expect("in-domain source"))
        .collect();
    assert_eq!(targets.len(), 64);

    let widths: Vec<u8> = (0_u8..64)
        .map(|target| epogdoon_preimage_width(target).expect("in-domain target"))
        .collect();
    assert_eq!(widths.iter().map(|width| usize::from(*width)).sum::<usize>(), 72);
    assert_eq!(widths.iter().filter(|width| **width == 2).count(), 8);
    assert!(widths.iter().all(|width| matches!(width, 1 | 2)));
}

#[test]
fn tonic_context_frame_landscape_is_the_existing_12_by_7_field() {
    let landscape: Vec<_> = tonic_context_frame_landscape().collect();
    assert_eq!(landscape.len(), 84);
    assert_eq!(landscape.iter().copied().collect::<HashSet<_>>().len(), 84);

    for lens in LensId::ALL {
        assert_eq!(
            landscape
                .iter()
                .filter(|address| address.tonic_lens == lens)
                .count(),
            ContextFrameId::ALL.len()
        );
    }

    for frame in ContextFrameId::ALL {
        assert_eq!(
            landscape
                .iter()
                .filter(|address| address.context_frame == frame)
                .count(),
            LensId::ALL.len()
        );
    }
}

#[test]
fn implemented_epogdoon_mapping_does_not_promote_open_fold_semantics() {
    assert_eq!(EPOGDOON_FOLD_SEMANTICS, MusicalEvidenceClass::OpenEdge);
    assert!(HARMONIC_RELATIONS
        .iter()
        .all(|relation| relation.evidence == MusicalEvidenceClass::AuthoredAccepted));
}
