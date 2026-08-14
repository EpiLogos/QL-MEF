use ql_mef::{
    LensId, LensRef, SublensRef, all_lens_definitions, all_sublens_definitions, lens_definition,
};

#[test]
fn exact_twelve_lens_fixture_matches_registry() {
    let fixture = include_str!("../../../fixtures/q2/lenses.tsv");
    let lines = fixture.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 12);
    assert_eq!(all_lens_definitions().len(), 12);

    for (line, definition) in lines.iter().zip(all_lens_definitions()) {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 5);
        assert_eq!(definition.id().code(), fields[0]);
        assert_eq!(definition.name(), fields[1]);
        assert_eq!(definition.square().code(), fields[2]);
        assert_eq!(definition.square().name(), fields[3]);
        assert_eq!(definition.sublens_labels().as_slice(), fields[4].split('|').collect::<Vec<_>>());
    }
}

#[test]
fn manifold_contains_exactly_seventy_two_addressable_sublenses() {
    let sublenses = all_sublens_definitions().collect::<Result<Vec<_>, _>>().expect("canonical registry");
    assert_eq!(sublenses.len(), 72);
    for lens in LensId::ALL {
        for position in 0..=5 {
            let reference = SublensRef::canonical(lens, position).expect("valid canonical sublens");
            let encoded = reference.to_string();
            assert_eq!(encoded.parse::<SublensRef>().expect("round trip"), reference);
        }
    }
}

#[test]
fn governing_l4_prime_wording_is_exact() {
    assert_eq!(
        lens_definition(LensId::L4Prime).sublens_labels(),
        ["Prompts", "Traces", "Challenges", "Patterns", "Discovery", "Insight"]
    );
}

#[test]
fn versioned_refs_round_trip_and_invalid_coordinates_fail_visibly() {
    for lens in LensId::ALL {
        let reference = LensRef::canonical(lens);
        assert_eq!(reference.to_string().parse::<LensRef>().expect("round trip"), reference);
    }
    assert!("mef:sublens:L4'.6@1".parse::<SublensRef>().is_err());
    assert!("mef:lens:L4'@2".parse::<LensRef>().is_err());
}
