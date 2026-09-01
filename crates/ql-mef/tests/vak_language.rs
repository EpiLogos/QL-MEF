use std::collections::BTreeSet;

use ql_mef::{
    SelfOtherForm, VAK_ENTRY_COUNT, VAK_SOURCE_GIT_BLOB, VAK_SOURCE_PATH, VAK_SOURCE_REPOSITORY,
    VAK_SOURCE_REVISION, VakAddressHorizon, VakPraxisAspect, VakRef, VakRegistry, VakRelationOp,
    VakStanding,
};

#[test]
fn authoritative_registry_is_exactly_109_unique_source_entries() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    assert_eq!(registry.len(), VAK_ENTRY_COUNT);
    assert_eq!(registry.len(), 109);

    let unique = registry
        .entries()
        .map(|entry| entry.vak_ref.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), 109);

    let receipt = VakRegistry::source_provenance_receipt();
    assert!(receipt.contains(VAK_SOURCE_REPOSITORY));
    assert!(receipt.contains(VAK_SOURCE_REVISION));
    assert!(receipt.contains(VAK_SOURCE_PATH));
    assert!(receipt.contains(VAK_SOURCE_GIT_BLOB));
    assert!(receipt.contains("\"entry_count\": 109"));
    assert!(receipt.contains("\"parity\": \"exact-git-blob-identity\""));
}

#[test]
fn siva_sixfold_is_present_at_exact_source_coordinates() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let expected = [
        (VakRelationOp::Potential, "M0-5-(0/1)-0", "(@#)"),
        (VakRelationOp::Distinguish, "M0-5-(0/1)-1", "(-)"),
        (VakRelationOp::Affirm, "M0-5-(0/1)-2", "(+)"),
        (VakRelationOp::Relate, "M0-5-(0/1)-3", "(x)"),
        (VakRelationOp::Contextualise, "M0-5-(0/1)-4", "(/)"),
        (VakRelationOp::Express, "M0-5-(0/1)-5", "(=)"),
    ];

    for (operator, coordinate, source_glyph) in expected {
        assert_eq!(operator.position() as usize, expected_position(operator));
        let entry = registry.locate_str(coordinate).unwrap();
        assert!(entry.raw_source_row.contains(source_glyph));
        assert!(entry.raw_source_row.contains(operator.name()));
        assert_eq!(entry.source.standing, VakStanding::SourceBacked);
    }
}

#[test]
fn shakti_sixfold_is_present_at_exact_source_coordinates() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let expected = [
        (VakAddressHorizon::H0, "M0-5-(5/0)-0", "@0 = ##"),
        (VakAddressHorizon::H1, "M0-5-(5/0)-1", "@1 = O#"),
        (VakAddressHorizon::H2, "M0-5-(5/0)-2", "@2 = X#"),
        (VakAddressHorizon::H3, "M0-5-(5/0)-3", "@3 = N#"),
        (VakAddressHorizon::H4, "M0-5-(5/0)-4", "@4 = M#"),
        (VakAddressHorizon::H5, "M0-5-(5/0)-5", "@5 = R#"),
    ];

    for (horizon, coordinate, source_relation) in expected {
        let entry = registry.locate_str(coordinate).unwrap();
        assert!(entry.raw_source_row.contains(source_relation));
        assert!(entry.raw_source_row.contains(horizon.source_symbol()));
        assert_eq!(entry.source.standing, VakStanding::SourceBacked);
    }
}

#[test]
fn self_other_language_parses_generates_and_returns_to_exact_source_nodes() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let expected = [
        ("!", "M0-3-6-0"),
        ("?", "M0-3-6-1"),
        ("!-", "M0-3-6-2"),
        ("-?", "M0-3-6-3"),
        ("!?", "M0-3-6-4"),
        ("?-", "M0-3-6-5"),
        ("-!", "M0-3-6-6"),
        ("?!", "M0-3-6-7"),
        ("-!/!-", "M0-3-6-8"),
        ("-?/?-", "M0-3-6-9"),
        ("!?/?!", "M0-3-6-10"),
        ("?!/!?", "M0-3-6-11"),
    ];

    for (glyph, coordinate) in expected {
        let form = SelfOtherForm::parse(glyph).unwrap();
        assert_eq!(form.to_string(), glyph);
        assert_eq!(form.source_ref().as_str(), coordinate);
        let entry = registry.self_other_entry(form).unwrap();
        assert_eq!(entry.vak_ref.as_str(), coordinate);
        assert_eq!(entry.symbol.as_deref(), Some(glyph));
    }
}

#[test]
fn vak_source_neighbourhood_exposes_the_twelve_grammar_teeth() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let centre = VakRef::new("M0-3-6").unwrap();
    let children = registry.children(&centre);
    assert_eq!(children.len(), 12);
    for form in SelfOtherForm::ALL {
        assert!(children.contains(&&form.source_ref()));
    }

    let neighbourhood = registry.neighbourhood(&centre, 1).unwrap();
    assert_eq!(neighbourhood.entries.len(), 13);
    assert_eq!(neighbourhood.relations.len(), 13);
}

#[test]
fn will_knowledge_action_readings_are_source_backed() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    for aspect in [
        VakPraxisAspect::WillAgency,
        VakPraxisAspect::KnowledgeVimarsa,
        VakPraxisAspect::ActionSvatantrya,
    ] {
        let reading = registry.praxis_reading(aspect);
        assert_eq!(reading.standing, VakStanding::SourceBacked);
        assert!(!reading.source_refs.is_empty());
    }
}

#[test]
fn ordinary_native_ref_can_refract_into_a_real_vak_neighbourhood() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let vak_ref = VakRef::new("M0-3-10-(0/1)").unwrap();
    let reading = registry
        .refract(
            "action:aikit/resolve",
            vak_ref.clone(),
            VakStanding::ImplementationMapping,
            vec!["explicit test binding".into()],
        )
        .unwrap();
    assert_eq!(reading.native_ref, "action:aikit/resolve");
    assert_eq!(reading.vak_ref, vak_ref);
    assert_eq!(reading.standing, VakStanding::ImplementationMapping);
    assert!(!registry.neighbourhood(&reading.vak_ref, 1).unwrap().entries.is_empty());
}

fn expected_position(operator: VakRelationOp) -> usize {
    match operator {
        VakRelationOp::Potential => 0,
        VakRelationOp::Distinguish => 1,
        VakRelationOp::Affirm => 2,
        VakRelationOp::Relate => 3,
        VakRelationOp::Contextualise => 4,
        VakRelationOp::Express => 5,
    }
}
