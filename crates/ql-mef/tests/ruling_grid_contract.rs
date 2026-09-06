//! Conformance for `ql.mef.ruling-grid/v1` — the T3 ruling-grid address
//! transform over the MEF 72-address space, with the projective-closure
//! check and the preserved absolute-position law.

use ql_core::AngleDeg10;
use ql_mef::{
    LensId, MEF_ROTATION_VERSION, RULING_GRID_CONTRACT_REF, RULING_GRID_VERSION, RulingGridAddress,
    RulingS, RulingT, SublensRef, projective_closure,
};

const FIXTURE: &str = include_str!("../../../fixtures/pole/ruling-grid-v1.tsv");

#[test]
fn contract_identity_is_versioned() {
    assert_eq!(RULING_GRID_VERSION, "1.0.0");
    assert_eq!(RULING_GRID_CONTRACT_REF, "ql.mef.ruling-grid/v1");
}

#[test]
fn fixture_ruling_rows_cover_all_72_addresses() {
    let mut rows = 0;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        if fields[0] != "ruling" {
            continue;
        }
        rows += 1;
        let lens = LensId::ALL[fields[1].parse::<usize>().expect("lens index")];
        let local: u8 = fields[2].parse().expect("local position");
        let sublens = SublensRef::canonical(lens, local).expect("canonical coordinate");
        let grid = RulingGridAddress::from_sublens(sublens);

        assert_eq!(
            grid.s().orientation().reduced().0,
            fields[3].parse::<i32>().unwrap(),
            "{row}"
        );
        assert_eq!(
            grid.t().orientation().reduced().0,
            fields[4].parse::<i32>().unwrap(),
            "{row}"
        );
        assert_eq!(
            grid.absolute_position(),
            fields[5].parse::<u8>().unwrap(),
            "{row}"
        );

        let back = grid.to_sublens().expect("grid address restores");
        assert_eq!(back.lens().lens(), lens);
        assert_eq!(back.position().value(), local);
        assert_eq!(
            back.rotation().absolute_position().value(),
            sublens.rotation().absolute_position().value(),
            "absolute-position law through the round trip at {row}"
        );
    }
    assert_eq!(
        rows, 72,
        "the fixture must carry all 72 ruling intersections"
    );
}

#[test]
fn fixture_closure_rows_match_the_projective_closure_report() {
    let report = projective_closure().expect("the grid must close");
    let mut seen_intersections = false;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        if fields[0] != "closure" {
            continue;
        }
        let value: usize = fields[2].parse().expect("closure count");
        match fields[1] {
            "intersections" => {
                assert_eq!(report.intersections, value);
                seen_intersections = true;
            }
            "p-lines" => assert_eq!(report.p_lines, value),
            "l-lines" => assert_eq!(report.l_lines, value),
            "points-per-p-line" => assert_eq!(report.points_per_p_line, value),
            "points-per-l-line" => assert_eq!(report.points_per_l_line, value),
            other => panic!("unknown closure row: {other}"),
        }
    }
    assert!(seen_intersections);
}

#[test]
fn ruling_lines_close_through_their_full_cycle() {
    for s in 0u8..12 {
        let mut current = RulingS::new(s).expect("s in range").advanced(1);
        let mut steps = 1;
        while current.index() != s {
            current = current.advanced(1);
            steps += 1;
        }
        assert_eq!(steps, 12, "P line {s} must close after 12 steps");
    }
    for t in 0u8..6 {
        let mut current = RulingT::new(t).expect("t in range").advanced(1);
        let mut steps = 1;
        while current.index() != t {
            current = current.advanced(1);
            steps += 1;
        }
        assert_eq!(steps, 6, "L line {t} must close after 6 steps");
    }
}

#[test]
fn ruling_quanta_tile_the_full_turn() {
    assert_eq!(RulingS::QUANTUM_DEG10, 300, "P quantum 30°");
    assert_eq!(RulingT::QUANTUM_DEG10, 600, "L quantum 60°");
    assert_eq!(12 * RulingS::QUANTUM_DEG10, AngleDeg10::FULL_TURN_DEG10);
    assert_eq!(6 * RulingT::QUANTUM_DEG10, AngleDeg10::FULL_TURN_DEG10);
}

#[test]
fn transform_preserves_the_mef_rotation_identity() {
    // The ruling grid introduces no second substrate: the MEF rotation of
    // the restored coordinate matches the source rotation, version included.
    for lens in [LensId::L0, LensId::L3Prime, LensId::L5] {
        for local in [0u8, 3, 5] {
            let sublens = SublensRef::canonical(lens, local).expect("coordinate");
            let restored = RulingGridAddress::from_sublens(sublens)
                .to_sublens()
                .expect("restore");
            assert_eq!(
                restored.rotation().absolute_position(),
                sublens.rotation().absolute_position()
            );
            assert_eq!(MEF_ROTATION_VERSION, "1.0.0");
        }
    }
}
