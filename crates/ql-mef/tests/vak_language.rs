use std::collections::BTreeSet;

use ql_mef::{
    SelfOtherForm, VAK_ENTRY_COUNT, VAK_SOURCE_GIT_BLOB, VAK_SOURCE_PATH, VAK_SOURCE_REPOSITORY,
    VAK_SOURCE_REVISION, VakAddressHorizon, VakContextField, VakDivineAct, VakPraxisAspect, VakRef,
    VakRegistry, VakRelationKind, VakRelationOp, VakSpeechStance, VakStanding,
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
        assert_eq!(entry.source.standing, VakStanding::Source);
        let binding = registry.bind_operator(operator).unwrap();
        assert_eq!(binding.standing, VakStanding::Implementation);
        assert_eq!(
            binding.source_support,
            vec![VakRef::new(coordinate).unwrap()]
        );
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
        assert_eq!(entry.source.standing, VakStanding::Source);
        let binding = registry.bind_horizon(horizon).unwrap();
        assert_eq!(binding.standing, VakStanding::Implementation);
        assert_eq!(
            binding.source_support,
            vec![VakRef::new(coordinate).unwrap()]
        );
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
        assert!(
            children
                .iter()
                .any(|child| child.as_str() == form.source_coordinate())
        );
    }

    let neighbourhood = registry.neighbourhood(&centre, 1).unwrap();
    // Centre + parent + twelve direct grammar children.
    assert_eq!(neighbourhood.entries.len(), 14);
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
        assert_eq!(reading.standing, VakStanding::Source);
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
            VakStanding::Implementation,
            vec!["explicit test binding".into()],
        )
        .unwrap();
    assert_eq!(reading.native_ref, "action:aikit/resolve");
    assert_eq!(reading.vak_ref, vak_ref);
    assert_eq!(reading.standing, VakStanding::Implementation);
    assert!(
        !registry
            .neighbourhood(&reading.vak_ref, 1)
            .unwrap()
            .entries
            .is_empty()
    );
}

#[test]
fn siva_times_sakti_is_a_source_provenanced_canonical_six_by_six_field() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let field = registry.siva_sakti_operative_field().unwrap();
    assert_eq!(field.ql_shape_ref, ql_core::SIX_BY_SIX_SHAPE_REF);
    assert_eq!(field.cells.len(), 36);
    assert_eq!(field.standing, VakStanding::Implementation);

    for operator in VakRelationOp::ALL {
        for horizon in VakAddressHorizon::ALL {
            let cell = field
                .cells
                .iter()
                .find(|cell| cell.operator == operator && cell.horizon == horizon)
                .unwrap();
            assert_eq!(cell.ql_address.row.position.value(), operator.position());
            assert_eq!(cell.ql_address.column.position.value(), horizon.position());
            assert_eq!(
                cell.operator_source_ref.as_str(),
                operator.source_coordinate()
            );
            assert_eq!(
                cell.horizon_source_ref.as_str(),
                horizon.source_coordinate()
            );
            assert!(registry.locate(&cell.operator_source_ref).is_some());
            assert!(registry.locate(&cell.horizon_source_ref).is_some());
        }
    }
}

#[test]
fn slash_binds_the_two_vak_sixfolds_to_kernel_generation_without_inventing_content() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let field = registry.siva_sakti_relational_sixfold().unwrap();
    assert_eq!(field.ql_shape_ref, ql_core::RELATIONAL_SIXFOLD_SHAPE_REF);
    assert_eq!(
        field.ql_operator_ref,
        ql_core::RELATIONAL_SIXFOLD_OPERATOR_REF
    );
    assert_eq!(field.contextualise_source_ref.as_str(), "M0-5-(0/1)-4");
    assert_eq!(field.return_anchor_symbol, "0/1");
    assert_eq!(field.sites.len(), 6);
    assert!(field.semantic_generation_requires_attributable_return);

    for (position, site) in field.sites.iter().enumerate() {
        assert_eq!(site.position, position as u8);
        assert_eq!(site.operator.position(), position as u8);
        assert_eq!(site.horizon.position(), position as u8);
        assert_eq!(
            site.operator_source_ref.as_str(),
            site.operator.source_coordinate()
        );
        assert_eq!(
            site.horizon_source_ref.as_str(),
            site.horizon.source_coordinate()
        );
        assert_eq!(
            site.ql_operator_ref,
            format!(
                "{}:position-{position}",
                ql_core::RELATIONAL_SIXFOLD_OPERATOR_REF
            )
        );
    }
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

#[test]
fn exact_formal_property_coverage_matches_the_authoritative_source_receipt() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let coverage = registry.formal_coverage();
    assert_eq!(coverage.names, 109);
    assert_eq!(coverage.symbols, 107);
    assert_eq!(coverage.primary_designations, 108);
    assert_eq!(coverage.complete_formulations, 67);
    assert_eq!(coverage.formulation_breakdowns, 49);
    assert_eq!(coverage.metaphysical_names, 19);
    assert_eq!(coverage.descriptions, 97);
}

#[test]
fn coordinate_prefix_structure_is_derived_while_m0_4_context_chain_is_authored_architecture() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let structural = registry
        .relations_from(&VakRef::new("M0-3-6").unwrap())
        .unwrap();
    assert!(structural.iter().any(|relation| {
        relation.relation == VakRelationKind::Parent && relation.standing == VakStanding::Derived
    }));

    for field in VakContextField::ALL {
        let entry = registry.context_field_entry(field).unwrap();
        assert_eq!(entry.vak_ref.as_str(), field.source_coordinate());
        assert!(entry.raw_source_row.contains(field.symbol()));
    }
    let bimba = VakContextField::Bimba.source_ref();
    let relations = registry.context_relations_from(&bimba).unwrap();
    assert!(relations.iter().any(|relation| {
        relation.relation == VakRelationKind::Contextualises
            && relation.into_ref == VakContextField::Pratibimba.source_ref()
            && relation.standing == VakStanding::AuthoredArchitecture
    }));
}

#[test]
fn principle_nine_divine_action_paths_are_exact_source_relations() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let expected = [
        (
            VakDivineAct::Creation,
            "0R = @ = (9-O#-X#-N#)",
            vec!["9", "O#", "X#", "N#"],
        ),
        (
            VakDivineAct::Sustenance,
            "1R = @ = (O#-X#-N#-M#-#-(#))",
            vec!["O#", "X#", "N#", "M#", "#", "(#)"],
        ),
        (
            VakDivineAct::Dissolution,
            "2R = @ = (X#-N#-M#-#-(#)-(@#))",
            vec!["X#", "N#", "M#", "#", "(#)", "(@#)"],
        ),
        (
            VakDivineAct::Veiling,
            "3R = @ = ((@#)-(#)-#-M#-N#-X#)",
            vec!["(@#)", "(#)", "#", "M#", "N#", "X#"],
        ),
        (
            VakDivineAct::Grace,
            "4R = @ = ((#)-#-M#-N#-X#-O#)",
            vec!["(#)", "#", "M#", "N#", "X#", "O#"],
        ),
        (VakDivineAct::Absorption, "5R = @ = (##)", vec!["##"]),
    ];
    for (act, formula, tokens) in expected {
        let path = registry.r_path(act).unwrap();
        assert_eq!(path.standing, VakStanding::Source);
        assert_eq!(path.principle_nine_formula.as_deref(), Some(formula));
        assert_eq!(
            path.steps
                .iter()
                .map(|step| step.token.as_str())
                .collect::<Vec<_>>(),
            tokens
        );
        assert!(path.principle_nine_ref.is_some());
    }
    let freedom = registry.r_path(VakDivineAct::Freedom).unwrap();
    assert_eq!(freedom.steps[0].token, "R#");
    assert!(freedom.principle_nine_ref.is_none());
}

#[test]
fn m0_3_speech_forms_have_typed_source_grounded_stances() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let query = registry.parse_speech_act("-!").unwrap();
    assert_eq!(query.stance, VakSpeechStance::QueryOfOther);
    assert_eq!(query.standing, VakStanding::Source);
    assert_eq!(query.source_ref.as_str(), "M0-3-6-6");

    let reflexive = registry.parse_speech_act("?!").unwrap();
    assert_eq!(reflexive.stance, VakSpeechStance::ReflexiveQuery);
    let return_question = registry.parse_speech_act("?!/!?").unwrap();
    assert_eq!(
        return_question.stance,
        VakSpeechStance::WorldQuestioningSelf
    );
}

#[test]
fn will_knowledge_and_action_readings_use_explicit_source_coordinates() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let will = registry.praxis_reading(VakPraxisAspect::WillAgency);
    assert_eq!(
        will.source_refs,
        vec![
            VakRef::new("M0-3-3").unwrap(),
            VakRef::new("M0-3-6-2").unwrap(),
        ]
    );
    let knowledge = registry.praxis_reading(VakPraxisAspect::KnowledgeVimarsa);
    assert_eq!(
        knowledge.source_refs,
        vec![
            VakRef::new("M0-3-(0/1)").unwrap(),
            VakRef::new("M0-(4.0/1/2)").unwrap(),
        ]
    );
    let action = registry.praxis_reading(VakPraxisAspect::ActionSvatantrya);
    assert_eq!(
        action.source_refs,
        vec![
            VakRef::new("M0-3-10").unwrap(),
            VakRef::new("M0-3-10-(0/1)").unwrap(),
            VakRef::new("M0-5-(5/0)-5").unwrap(),
        ]
    );
}
