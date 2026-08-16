use ql_mef::{LensId, MEF_ROTATION_VERSION, MefUnitFace, SublensRef};

#[test]
fn rotation_fixture_covers_all_72_mef_coordinates() {
    let fixture = include_str!("../../../fixtures/q6/mef-rotation-v1.tsv");
    let mut count = 0;

    for row in fixture.lines().skip(1).filter(|row| !row.is_empty()) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 4, "invalid rotation fixture row: {row}");

        let lens = fields[0].parse::<LensId>().expect("fixture lens must parse");
        let local = fields[1]
            .parse::<u8>()
            .expect("fixture local position must parse");
        let expected_absolute = fields[2]
            .parse::<u8>()
            .expect("fixture absolute position must parse");
        let expected_leading = match fields[3] {
            "Name" => MefUnitFace::Name,
            "Power" => MefUnitFace::Power,
            other => panic!("unsupported fixture unit face: {other}"),
        };

        let sublens = SublensRef::canonical(lens, local).expect("fixture coordinate must be valid");
        let rotation = sublens.rotation();

        assert_eq!(rotation.lens(), lens);
        assert_eq!(rotation.local_position().value(), local);
        assert_eq!(rotation.absolute_position().value(), expected_absolute);
        assert_eq!(rotation.leading_unit(), expected_leading);
        assert_eq!(rotation.companion_unit(), expected_leading.opposite());
        count += 1;
    }

    assert_eq!(count, 72);
}

#[test]
fn day_night_twins_share_rotation_but_not_leading_unit() {
    for pair in LensId::ALL.chunks_exact(2) {
        let day = pair[0];
        let night = pair[1];
        assert_eq!(day.index(), night.index());

        for local in 0..6 {
            let day_rotation = SublensRef::canonical(day, local)
                .expect("day coordinate must be valid")
                .rotation();
            let night_rotation = SublensRef::canonical(night, local)
                .expect("night coordinate must be valid")
                .rotation();

            assert_eq!(
                day_rotation.absolute_position(),
                night_rotation.absolute_position()
            );
            assert_eq!(day_rotation.leading_unit(), MefUnitFace::Name);
            assert_eq!(night_rotation.leading_unit(), MefUnitFace::Power);
        }
    }
}

#[test]
fn rotation_wraps_modulo_six_without_creating_extra_positions() {
    assert_eq!(
        SublensRef::canonical(LensId::L5, 1)
            .expect("valid coordinate")
            .rotation()
            .absolute_position()
            .value(),
        0
    );
    assert_eq!(
        SublensRef::canonical(LensId::L4Prime, 5)
            .expect("valid coordinate")
            .rotation()
            .absolute_position()
            .value(),
        3
    );
    assert!(SublensRef::canonical(LensId::L0, 6).is_err());
}

#[test]
fn promotion_manifest_keeps_semantic_roles_and_runtime_policy_outside_rotation() {
    let manifest = include_str!("../../../fixtures/q6/mef-rotation-promotion-v1.json");

    assert_eq!(MEF_ROTATION_VERSION, "1.0.0");
    assert!(manifest.contains("\"capability\": \"ql.mef.rotation\""));
    assert!(manifest.contains("\"status\": \"specified-formal-structure\""));
    assert!(manifest.contains("\"automaticInvocation\": false"));
    assert!(manifest.contains("\"localToAbsolute\": \"(anchor + local) mod 6\""));
    assert!(manifest.contains("\"semanticRoleBinding\": \"not-promoted\""));
    assert!(manifest.contains("\"runtimePolicy\": \"not-promoted\""));

    for research_extension in [
        "ql.state64.runtime-semantics",
        "ql.epogdoon.retained-difference-metric",
        "ql.topology.control-rules",
        "ql.mef.context-operational-roles",
    ] {
        assert!(manifest.contains(research_extension));
    }

    assert!(manifest.contains(
        "\"sha256\": \"1e1ee72c6445eea2e7df057798bd1b0441f6e5eeac289dddb65d680d7ca6bb51\""
    ));
    assert!(manifest.contains(
        "\"sha256\": \"61508658f871f232f554127052d683ecded15c28694cc79a110bcf7841492699\""
    ));
}
