//! Conformance for `ql.pole.elemental-carrier/v1` — the ratified
//! ElementalQuaternionBasis (T2) with carrier and epogdoon continuity
//! (acceptance criteria 2 and 3).

use ql_core::{
    AngleDeg10, Element, ElementalQuaternionBasis, Nucleotide, Transduction18to16, det_shadow,
};

const FIXTURE: &str = include_str!("../../../fixtures/pole/elemental-carrier-contract-v1.tsv");

fn fixture_rows(tag: &str) -> Vec<Vec<String>> {
    FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').map(|f| f.to_string()).collect::<Vec<_>>())
        .filter(|fields| fields[0] == tag)
        .collect()
}

#[test]
fn fixture_basis_rows_pin_the_ratified_relations() {
    let rows = fixture_rows("basis");
    assert_eq!(rows.len(), 4, "the four material elements");
    for fields in rows {
        let component_slot: usize = fields[4].parse().expect("component slot");
        let element = match fields[1].as_str() {
            "earth" => Element::Earth,
            "fire" => Element::Fire,
            "water" => Element::Water,
            "air" => Element::Air,
            other => panic!("unknown element: {other}"),
        };
        assert_eq!(element.name(), fields[2], "element name");
        // Nucleotide relation: A=Water, T=Fire, C=Earth, G=Air.
        let nucleotide = match fields[3].as_str() {
            "A" => Nucleotide::A,
            "T" => Nucleotide::T,
            "C" => Nucleotide::C,
            "G" => Nucleotide::G,
            other => panic!("unknown nucleotide letter: {other}"),
        };
        let basis = ElementalQuaternionBasis::canonical();
        assert_eq!(
            basis.element_of(nucleotide),
            element,
            "element_of {nucleotide}"
        );
        assert_eq!(
            basis.nucleotide_of(element),
            nucleotide,
            "nucleotide_of {element:?}"
        );
        // Quaternion component: [w,x,y,z] = [Earth,Fire,Water,Air].
        assert_eq!(element.component_index(), component_slot, "component slot");
        let expected_component = match fields[5].as_str() {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            other => panic!("unknown component: {other}"),
        };
        assert_eq!(element.component_index(), expected_component);
    }
}

#[test]
fn fixture_carrier_rows_carry_four_eighteen_to_four_sixteen() {
    let rows: Vec<(String, usize)> = fixture_rows("carrier")
        .into_iter()
        .map(|fields| (fields[1].clone(), fields[2].parse().expect("carrier count")))
        .collect();
    let get = |name: &str| {
        rows.iter()
            .find(|(key, _)| key == name)
            .unwrap_or_else(|| panic!("missing carrier row {name}"))
            .1
    };
    assert_eq!(get("fibres"), 4);
    assert_eq!(get("fibre-states"), 18);
    assert_eq!(get("form-states"), 16);
    assert_eq!(get("fibres") * get("fibre-states"), 72);
    assert_eq!(get("fibres") * get("form-states"), 64);

    // Second Spanda row: 100 = 4(3²+4²) = 4×5²; 100/5 = 20 (the fibre
    // quantum in degrees — M2-C23 pentadic elemental aperture).
    let spanda = fixture_rows("carrier")
        .into_iter()
        .find(|fields| fields[1] == "second-spanda")
        .expect("second-spanda carrier row");
    assert_eq!(spanda[2], "100");
    assert_eq!(spanda[3], "5");
    assert_eq!(spanda[4], "20");
}

#[test]
fn fixture_transduction_rows_match_the_epogdoon_fold() {
    let rows = fixture_rows("transduction");
    assert_eq!(rows.len(), 18);
    for fields in rows {
        let source: u8 = fields[1].parse().expect("source");
        let target: u8 = fields[2].parse().expect("target");
        let transduction = Transduction18to16::new(source).expect("fixture source in fibre range");
        assert_eq!(transduction.target(), target, "T18→16 at {source}");
        let folded_flag = if transduction.is_folded() {
            "folded"
        } else {
            "identity"
        };
        assert_eq!(folded_flag, fields[3], "folded flag at {source}");
    }
}

#[test]
fn fixture_det_shadow_rows_match_the_kernel_fold_back() {
    let rows = fixture_rows("det-shadow");
    assert_eq!(rows.len(), 8, "the epogdoon tax at the 72→64 seam");
    for fields in rows {
        let source: u8 = fields[1].parse().expect("det source");
        let target: u8 = fields[2].parse().expect("det target");
        assert_eq!(det_shadow(source).expect("det in range"), target);
    }
}

#[test]
fn fixture_epogdoon_rows_hold_in_both_registers() {
    let rows = fixture_rows("epogdoon");
    assert_eq!(rows.len(), 2, "count register and angle register");
    for fields in rows {
        match fields[1].as_str() {
            "count" => {
                let fibre: u32 = fields[2].parse().unwrap();
                let form: u32 = fields[3].parse().unwrap();
                let numerator: u32 = fields[4].parse().unwrap();
                let denominator: u32 = fields[5].parse().unwrap();
                // 16/18 = 8/9 — the epogdoon in the count register.
                assert_eq!(form * numerator, fibre * denominator);
            }
            "angle" => {
                let fibre_deg10: i32 = fields[2].parse().unwrap();
                let form_deg10: i32 = fields[3].parse().unwrap();
                let numerator: i32 = fields[4].parse().unwrap();
                let denominator: i32 = fields[5].parse().unwrap();
                // 20° × 9/8 = 22.5° — the same epogdoon in the angle register.
                assert_eq!(fibre_deg10 * numerator, form_deg10 * denominator);
                assert_eq!(fibre_deg10, AngleDeg10::FULL_TURN_DEG10 / 18);
                assert_eq!(form_deg10, AngleDeg10::FULL_TURN_DEG10 / 16);
            }
            other => panic!("unknown epogdoon register: {other}"),
        }
    }
}

#[test]
fn fixture_sync_rows_hold_the_shared_closures() {
    for fields in fixture_rows("sync") {
        let degrees: i32 = fields[2].parse().expect("sync degrees");
        match fields[1].as_str() {
            "fibre-form" => {
                // 9 × 20° = 8 × 22.5° = 180°.
                assert_eq!(degrees, 180);
                assert_eq!(9 * 200, degrees * 10);
                assert_eq!(8 * 225, degrees * 10);
            }
            "full-turn" => {
                // 18 × 20° = 16 × 22.5° = 360°.
                assert_eq!(degrees, 360);
                assert_eq!(18 * 200, degrees * 10);
                assert_eq!(16 * 225, degrees * 10);
            }
            other => panic!("unknown sync row: {other}"),
        }
    }
}

#[test]
fn alternates_are_drift_and_cannot_be_constructed() {
    // The basis is sealed: canonical() is the only constructor, so no
    // permuted elemental order can exist as a basis value. The ratified
    // relations are pinned once more, directly.
    let basis = ElementalQuaternionBasis::canonical();
    assert_eq!(basis.element_of(Nucleotide::A), Element::Water);
    assert_eq!(basis.element_of(Nucleotide::T), Element::Fire);
    assert_eq!(basis.element_of(Nucleotide::C), Element::Earth);
    assert_eq!(basis.element_of(Nucleotide::G), Element::Air);
    let order = basis.component_order();
    assert_eq!(
        order,
        [Element::Earth, Element::Fire, Element::Water, Element::Air]
    );
    let components = basis.components(10, 20, 30, 40);
    assert_eq!(
        (components.w, components.x, components.y, components.z),
        (10, 20, 30, 40)
    );
}

#[test]
fn carrier_continuity_is_inspectable_from_m2_through_m3_into_m4() {
    // Acceptance criterion 2: walk one element from M2 fibre through M3 form
    // into the M4 quaternion basis slot.
    let basis = ElementalQuaternionBasis::canonical();
    for element in Element::ALL {
        let fibre = basis.fibre_index_of(element);
        let nucleotide = basis.nucleotide_of(element);
        let back = basis.element_of(nucleotide);
        let slot = element.component_index();
        assert_eq!(
            fibre, slot,
            "fibre order equals component order for {element:?}"
        );
        assert_eq!(back, element);
        // The element's fibre: 18 states; its form: 16 states; both derived
        // from the same quantum family.
        assert!(Transduction18to16::new(0).is_ok());
        assert!(nucleotide.coin_value().value() >= 6);
    }
}
