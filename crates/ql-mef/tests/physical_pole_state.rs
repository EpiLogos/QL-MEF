//! Conformance for `ql.mef.physical-pole-form-state/v1` — the Stage-2
//! composition bridge: one eventRef driving M1/M2/M3 (criterion 1),
//! elemental carrier continuity through the composed object (criterion 2),
//! epogdoon continuity (criterion 3), and the shared deterministic object
//! (criterion 12).

use ql_core::{
    Element, ElementalQuaternionBasis, FoldState, Nucleotide, QlPosition, QuaternionComponents,
};
use ql_mef::{
    Amplitude, M1CarrierFacts, PHYSICAL_POLE_FORM_STATE_REF, PHYSICAL_POLE_FORM_STATE_VERSION,
    PhysicalPoleFormState, PoleIdentity, PoleProvenance, Readiness, TemplateureField,
};

const FIXTURE: &str = include_str!("../../../fixtures/pole/physical-pole-form-state-v1.tsv");

fn rows(tag: &str) -> Vec<Vec<String>> {
    FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').map(|f| f.to_string()).collect::<Vec<_>>())
        .filter(|fields| fields[0] == tag)
        .collect()
}

fn fixture_identity() -> PoleIdentity {
    let row = &rows("identity-sample")[0];
    PoleIdentity::new(
        row[1].clone(),
        row[2].parse().unwrap(),
        row[3].parse().unwrap(),
        row[4].parse().unwrap(),
    )
    .expect("fixture identity")
}

fn fixture_m1() -> M1CarrierFacts {
    let ratio = &rows("active-ratio")[0];
    M1CarrierFacts {
        position6: QlPosition::new(2).expect("position"),
        ring_quaternion: QuaternionComponents {
            w: 1,
            x: 0,
            y: 0,
            z: 0,
        },
        active_ratio: (ratio[1].parse().unwrap(), ratio[2].parse().unwrap()),
        authority_handles: [
            "m1:k2-torus".to_string(),
            "m1:hopf".to_string(),
            "m1:ananda".to_string(),
        ],
    }
}

fn fixture_m2() -> TemplateureField {
    let mut fire = [Amplitude::ZERO; 18];
    fire[7] = Amplitude::new(12, 5);
    TemplateureField::from_amplitudes(
        [Amplitude::ZERO; 18],
        fire,
        [Amplitude::ZERO; 18],
        [Amplitude::ZERO; 18],
    )
}

fn fixture_m3() -> FoldState {
    FoldState::from_codon(
        CodonFixture::from_symbols("ATG"),
        ql_core::ApertureIndex::new(4).expect("aperture"),
        17,
    )
}

/// Tiny helper building codons from fixture symbols.
struct CodonFixture;

impl CodonFixture {
    fn from_symbols(symbols: &str) -> ql_core::Codon64 {
        let nucleotides: Vec<ql_core::Nucleotide> = symbols
            .bytes()
            .map(|b| ql_core::Nucleotide::try_from((b - b'A') & 0x03).expect("letter"))
            .collect();
        ql_core::Codon64::from_nucleotides(nucleotides[0], nucleotides[1], nucleotides[2])
    }
}

fn compose_with(m2: Readiness, m3: Readiness, provider: Readiness) -> PhysicalPoleFormState {
    PhysicalPoleFormState::compose(
        fixture_identity(),
        fixture_m1(),
        fixture_m2(),
        m2,
        fixture_m3(),
        m3,
        PoleProvenance {
            source_handles: vec!["m2:resonator".to_string()],
            derivation_handles: vec![
                ql_mef::TEMPLATEURE_FIELD_CONTRACT_REF.to_string(),
                ql_core::POLE_FOLD_STATE_REF.to_string(),
            ],
            provider_status: provider,
        },
    )
}

#[test]
fn contract_identity_is_versioned() {
    assert_eq!(PHYSICAL_POLE_FORM_STATE_VERSION, "1.0.0");
    assert_eq!(
        PHYSICAL_POLE_FORM_STATE_REF,
        "ql.mef.physical-pole-form-state/v1"
    );
}

#[test]
fn criterion_1_one_event_ref_drives_all_sections() {
    let state = compose_with(Readiness::Ready, Readiness::Ready, Readiness::Ready);
    let row = &rows("identity-sample")[0];
    assert_eq!(state.identity().event_ref(), row[1]);
    assert_eq!(
        state.identity().profile_generation(),
        row[2].parse::<u64>().unwrap()
    );
    assert_eq!(state.identity().tick12(), row[3].parse::<u8>().unwrap());
    assert_eq!(state.identity().degree720(), row[4].parse::<u16>().unwrap());
    // The double-cover law: degree360 = degree720 mod 360; shadow layer kept.
    assert_eq!(
        state.identity().degree360(),
        state.identity().degree720() % 360
    );
    // Sections carry no event refs of their own: the M1/M2/M3 projections
    // are sections of the one state, not independent events.
    assert!(
        state
            .m1_carrier()
            .authority_handles
            .iter()
            .all(|h| !h.contains("event:"))
    );
}

#[test]
fn criterion_2_carrier_continuity_rows_match_the_composed_object() {
    let state = compose_with(Readiness::Ready, Readiness::Ready, Readiness::Ready);
    let continuity = state.elemental_carrier_continuity();
    let rows = rows("carrier");
    assert_eq!(rows.len(), 4);
    for fields in rows {
        let element = match fields[1].as_str() {
            "earth" => Element::Earth,
            "fire" => Element::Fire,
            "water" => Element::Water,
            _ => Element::Air,
        };
        let fibre_index: usize = fields[2].parse().unwrap();
        let slot: usize = fields[4].parse().unwrap();
        let found = continuity
            .iter()
            .find(|(e, ..)| *e == element)
            .expect("element in continuity");
        assert_eq!(found.1, fibre_index, "fibre index for {element:?}");
        assert_eq!(found.3, slot, "basis component slot for {element:?}");
        // The nucleotide letter matches the basis relation.
        let nucleotide = match fields[3].as_str() {
            "A" => Nucleotide::A,
            "T" => Nucleotide::T,
            "C" => Nucleotide::C,
            _ => Nucleotide::G,
        };
        assert_eq!(
            found.2,
            nucleotide.bits(),
            "nucleotide bits for {element:?}"
        );
        assert_eq!(
            ElementalQuaternionBasis::canonical().element_of(nucleotide),
            element
        );
        // And the composed M2 field carries the same element at that fibre.
        assert_eq!(state.m2_templateure().fibre(element).element(), element);
    }
}

#[test]
fn criterion_3_epogdoon_continuity_from_the_fixture_ratio() {
    let state = compose_with(Readiness::Ready, Readiness::Ready, Readiness::Ready);
    let ratio = &rows("active-ratio")[0];
    assert_eq!(
        state.m1_carrier().active_ratio.0,
        ratio[1].parse::<u32>().unwrap()
    );
    assert_eq!(
        state.m1_carrier().active_ratio.1,
        ratio[2].parse::<u32>().unwrap()
    );
    // 9/8 — both registers, one ratio.
    assert!(state.epogdoon_continuity_holds());
}

#[test]
fn readiness_aggregation_matches_the_fixture() {
    for fields in rows("readiness") {
        let readiness = |name: &str| match name {
            "ready" => Readiness::Ready,
            "provisional" => Readiness::Provisional,
            _ => Readiness::Unavailable,
        };
        let state = compose_with(
            readiness(&fields[1]),
            readiness(&fields[2]),
            readiness(&fields[4]),
        );
        let expected = readiness(&fields[5]);
        assert_eq!(state.readiness(), expected, "aggregation for {fields:?}");
    }
}

#[test]
fn criterion_12_deterministic_composition_shared_by_human_and_agent() {
    let law = &rows("roles")[0];
    assert_eq!(law[1], "m2-potential");
    assert_eq!(law[2], "m3-resolution");
    assert_eq!(law[3], "never-merged");

    let state_a = compose_with(Readiness::Ready, Readiness::Ready, Readiness::Ready);
    let state_b = compose_with(Readiness::Ready, Readiness::Ready, Readiness::Ready);
    assert_eq!(state_a, state_b, "same sections compose the same object");
    // The resolved M3 form and the M2 potential keep their distinct roles.
    let (element, _) = state_a.m2_templateure().transduce().winning();
    assert_eq!(element, Element::Fire);
    assert_eq!(
        state_a.m3_rupa().codon().address(),
        CodonFixture::from_symbols("ATG").address()
    );
}
