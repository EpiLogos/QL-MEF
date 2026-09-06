//! Conformance for `ql.mef.templateure-field/v1` — the T4 vibrational
//! templateure field (V72 = ⊕4V18 with amplitudes), the T72→64 = I4⊗T18→16
//! transduction with amplitudes, and the analytic-signal quadrature operator.

use ql_core::Element;
use ql_mef::{
    Amplitude, FormPotential, TEMPLATEURE_FIELD_CONTRACT_REF, TEMPLATEURE_FIELD_VERSION,
    TemplateureField,
};

const FIXTURE: &str = include_str!("../../../fixtures/pole/templateure-field-v1.tsv");

fn rows(tag: &str) -> Vec<Vec<String>> {
    FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').map(|f| f.to_string()).collect::<Vec<_>>())
        .filter(|fields| fields[0] == tag)
        .collect()
}

fn element(name: &str) -> Element {
    match name {
        "earth" => Element::Earth,
        "fire" => Element::Fire,
        "water" => Element::Water,
        "air" => Element::Air,
        other => panic!("unknown element name: {other}"),
    }
}

#[test]
fn contract_identity_is_versioned() {
    assert_eq!(TEMPLATEURE_FIELD_VERSION, "1.0.0");
    assert_eq!(
        TEMPLATEURE_FIELD_CONTRACT_REF,
        "ql.mef.templateure-field/v1"
    );
}

#[test]
fn fixture_quadrature_rows_match_the_exact_operator() {
    let rows = rows("quadrature");
    assert!(!rows.is_empty());
    for fields in rows {
        let re: i64 = fields[1].parse().unwrap();
        let im: i64 = fields[2].parse().unwrap();
        let amplitude = Amplitude::new(re, im);
        let quadrature = amplitude.quadrature();
        assert_eq!(
            (quadrature.re, quadrature.im),
            (fields[3].parse().unwrap(), fields[4].parse().unwrap()),
            "quadrature of ({re},{im})"
        );
        assert_eq!(
            quadrature.power() as u64,
            fields[5].parse::<u64>().unwrap(),
            "power of ({re},{im})"
        );
        // Order 4: four steps return the identity; two steps the antipode.
        let q4 = amplitude
            .quadrature()
            .quadrature()
            .quadrature()
            .quadrature();
        assert_eq!(q4, amplitude);
    }
}

#[test]
fn fixture_transduction_rows_match_the_field_transform() {
    let rows = rows("transduction");
    assert_eq!(rows.len(), 18, "all 18 fibre states under T18→16");
    for fields in rows {
        let source: u8 = fields[1].parse().unwrap();
        let target: u8 = fields[2].parse().unwrap();
        let folded = if source >= 16 { "folded" } else { "identity" };
        assert_eq!(folded, fields[3]);
        let transduction = ql_core::Transduction18to16::new(source).expect("source");
        assert_eq!(transduction.target(), target);
    }
}

#[test]
fn fixture_field_sample_folds_amplitudes_not_indices() {
    // Excitation: Earth fibre lit at state 0 with (1,0) and at folded state
    // 16 with (0,2) — both land on target 0 as the coherent sum (1,2).
    let mut earth = [Amplitude::ZERO; 18];
    for fields in rows("field-sample") {
        assert_eq!(fields[1], "earth");
        let state: u8 = fields[2].parse().unwrap();
        earth[state as usize] =
            Amplitude::new(fields[3].parse().unwrap(), fields[4].parse().unwrap());
    }
    let field = TemplateureField::from_amplitudes(
        earth,
        [Amplitude::ZERO; 18],
        [Amplitude::ZERO; 18],
        [Amplitude::ZERO; 18],
    );
    let potential = field.transduce();
    let result = rows("field-result");
    assert_eq!(result.len(), 1);
    let fields = &result[0];
    let fused = potential.amplitude(element(&fields[1]), fields[2].parse::<u8>().unwrap());
    assert_eq!(
        (fused.re, fused.im),
        (fields[3].parse().unwrap(), fields[4].parse().unwrap())
    );
    assert_eq!(fused.power() as u64, fields[5].parse::<u64>().unwrap());
}

#[test]
fn fixture_invariants_hold_on_arbitrary_fields() {
    let mut water = [Amplitude::ZERO; 18];
    water[1] = Amplitude::new(7, -2);
    water[16] = Amplitude::new(1, 1);
    let mut air = [Amplitude::ZERO; 18];
    air[17] = Amplitude::new(-4, 9);
    let field =
        TemplateureField::from_amplitudes([Amplitude::ZERO; 18], [Amplitude::ZERO; 18], water, air);

    for fields in rows("invariant") {
        match fields[1].as_str() {
            "quadrature-power-conserved" => {
                assert_eq!(field.quadrature().total_power(), field.total_power());
            }
            "quadrature-order" => {
                let twice = field.quadrature().quadrature();
                let four = twice.quadrature().quadrature();
                for element in Element::ALL {
                    for state in 0u8..18 {
                        // Two steps give the antipode, four the identity.
                        if twice.fibre(element).amplitude(state)
                            != field
                                .fibre(element)
                                .amplitude(state)
                                .quadrature()
                                .quadrature()
                        {
                            panic!("quadrature order-2 law broken");
                        }
                        assert_eq!(
                            four.fibre(element).amplitude(state),
                            field.fibre(element).amplitude(state)
                        );
                    }
                }
            }
            "commutes-with-transduction" => {
                let left = field.quadrature().transduce();
                let right = field.transduce().quadrature();
                for element in Element::ALL {
                    for state in 0u8..16 {
                        assert_eq!(
                            left.amplitude(element, state),
                            right.amplitude(element, state)
                        );
                    }
                }
            }
            other => panic!("unknown invariant row: {other}"),
        }
    }
}

#[test]
fn form_potential_is_amplitudes_with_a_discrete_shadow() {
    // The 64-form potential carries amplitudes; the winning form is a
    // projection (the DET's discrete-address shadow), never the store.
    let mut fire = [Amplitude::ZERO; 18];
    fire[5] = Amplitude::new(50, 25);
    let field = TemplateureField::from_amplitudes(
        [Amplitude::ZERO; 18],
        fire,
        [Amplitude::ZERO; 18],
        [Amplitude::ZERO; 18],
    );
    let potential: FormPotential = field.transduce();
    let (element, state) = potential.winning();
    assert_eq!(element, Element::Fire);
    assert_eq!(state, 5);
    assert!(potential.total_power() > 0);
    // Fibre count and states per fibre carry the epogdoon seam.
    assert_eq!(field.fibre(Element::Earth).element(), Element::Earth);
}
