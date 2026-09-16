use ql_adapters::{
    DisclosureSelection, DisclosureSession, InstrumentDisclosure, TechneActionRoute, TechneAdapter,
    TechneDisclosure, TechneInstrument, TechneReading,
};
use serde_json::Value;

const REPRESENTATIVE: &str =
    include_str!("../../../fixtures/techne/representative-subject-v1.json");
const ABSENT_FACETS: &str = include_str!("../../../fixtures/techne/absent-facets-v1.json");
const DEVELOPMENT_DAY: &str = include_str!("../../../fixtures/techne/development-day-v1.json");
const TB0_CONNECTIVE_BASE: &str =
    include_str!("../../../fixtures/techne/tb0-connective-base-v1.json");
const READING_SCHEMA: &str =
    include_str!("../../../schemas/techne/ql-techne-reading-v1.schema.json");
const SESSION_SCHEMA: &str =
    include_str!("../../../schemas/techne/ql-techne-session-v1.schema.json");

fn validated(reading: TechneReading, message: &str) -> TechneReading {
    if let Err(error) = reading.validate() {
        panic!("{message}: {error}");
    }
    reading
}

fn parse(fixture: &str) -> TechneReading {
    let reading: TechneReading =
        serde_json::from_str(fixture).expect("conformance fixture must parse as ql.techne/v1");
    validated(reading, "conformance fixture must validate")
}

#[test]
fn representative_fixture_parses_and_discloses_every_instrument() {
    let reading = parse(REPRESENTATIVE);

    assert_eq!(reading.contract, "ql.techne/v1");
    assert!(reading.has_warranted_ql());
    assert!(reading.has_spatial());
    assert_eq!(reading.temporal.len(), 4);
    assert_eq!(reading.expressions.len(), 1);
    assert_eq!(reading.actions.len(), 3);

    let warrant = reading.ql.as_ref().unwrap().warrant.clone();
    assert_eq!(
        warrant.result_class,
        ql_adapters::QlResultClass::Deterministic
    );
    assert!(!warrant.evidence_refs.is_empty());

    for instrument in [
        TechneInstrument::Project,
        TechneInstrument::Canvas,
        TechneInstrument::Timeline,
        TechneInstrument::Journey,
        TechneInstrument::Place,
        TechneInstrument::Palace,
        TechneInstrument::Expressions,
    ] {
        let entry = reading
            .disclosure
            .instruments
            .iter()
            .find(|entry| entry.instrument == instrument)
            .unwrap_or_else(|| panic!("disclosure missing {instrument:?}"));
        assert!(entry.available, "{instrument:?} should be available");
    }
}

#[test]
fn absent_facets_fixture_keeps_absence_as_data() {
    let reading = parse(ABSENT_FACETS);

    assert!(!reading.has_warranted_ql(), "no QL without a warrant");
    assert!(
        !reading.has_spatial(),
        "spatial absence is data, not an error"
    );
    assert!(reading.expressions.is_empty());
    assert_eq!(reading.temporal.len(), 1);

    let unavailable: Vec<(TechneInstrument, String)> = reading
        .disclosure
        .instruments
        .iter()
        .filter(|entry| !entry.available)
        .map(|entry| (entry.instrument, entry.reason.clone().unwrap_or_default()))
        .collect();
    assert!(unavailable.iter().any(|(instrument, reason)| {
        *instrument == TechneInstrument::Place && reason == "no disclosed spatial reading"
    }));
    assert!(unavailable.iter().any(|(instrument, reason)| {
        *instrument == TechneInstrument::Expressions
            && reason == "no Expression is bound to this subject"
    }));
    assert!(
        unavailable
            .iter()
            .any(|(instrument, _)| *instrument == TechneInstrument::Journey)
    );
    assert!(
        unavailable
            .iter()
            .any(|(instrument, _)| *instrument == TechneInstrument::Expressions)
    );

    for (instrument, reason) in &unavailable {
        assert!(
            !reason.trim().is_empty(),
            "{instrument:?} unavailable without a reason"
        );
    }

    // The amended geometry: the six deep instruments carry M′ bindings and
    // the 4:2 reading; Expressions carries the conjugate 3:3 reading.
    let project = reading
        .disclosure
        .instruments
        .iter()
        .find(|entry| entry.instrument == TechneInstrument::Project)
        .expect("project (M0′ ground) is disclosed");
    assert_eq!(project.m_prime, Some(0));
    assert!(matches!(
        project.reading,
        Some(ql_adapters::TechneReadingKind::DeepFourTwo)
    ));
    let expressions = reading
        .disclosure
        .instruments
        .iter()
        .find(|entry| entry.instrument == TechneInstrument::Expressions)
        .expect("expressions (conjugate 3:3) is disclosed");
    assert_eq!(
        expressions.m_prime, None,
        "the conjugate reading binds no M′ office"
    );
    assert!(matches!(
        expressions.reading,
        Some(ql_adapters::TechneReadingKind::ConjugateThreeThree)
    ));
}

#[test]
fn development_day_fixture_preserves_occurrence_receipt_and_continuity() {
    let reading = parse(DEVELOPMENT_DAY);

    let occurrence = reading
        .temporal
        .iter()
        .find(|f| f.kind == ql_adapters::TemporalKind::Occurrence)
        .expect("occurrence facet");
    let receipt = reading
        .temporal
        .iter()
        .find(|f| f.kind == ql_adapters::TemporalKind::Receipt)
        .expect("receipt facet");
    assert_ne!(
        occurrence.instant, receipt.instant,
        "occurrence and receipt must stay distinct"
    );
    assert!(occurrence.uncertainty.is_some());
    assert!(receipt.uncertainty.is_some());

    assert!(reading.temporal.iter().any(|f| f.day_ref.is_some()));
    assert!(reading.temporal.iter().any(|f| f.now_ref.is_some()));
    assert!(reading.temporal.iter().any(|f| f.run_ref.is_some()));
    assert!(
        reading
            .temporal
            .iter()
            .any(|f| f.timezone_policy_ref.is_some())
    );

    assert!(!reading.has_warranted_ql());
    assert!(!reading.has_spatial());
    assert!(
        reading
            .disclosure
            .degraded
            .iter()
            .any(|d| d.instrument == TechneInstrument::Timeline)
    );
}

#[test]
fn native_refs_round_trip_byte_exact() {
    for fixture in [
        REPRESENTATIVE,
        ABSENT_FACETS,
        DEVELOPMENT_DAY,
        TB0_CONNECTIVE_BASE,
    ] {
        let reading = parse(fixture);
        let round: TechneReading =
            serde_json::from_str(&serde_json::to_string(&reading).unwrap()).unwrap();

        assert_eq!(reading.subject.subject_ref, round.subject.subject_ref);
        assert_eq!(reading.subject.native_owner, round.subject.native_owner);
        for (before, after) in reading.provenance.iter().zip(round.provenance.iter()) {
            assert_eq!(before.source_ref, after.source_ref);
            assert_eq!(before.source_revision, after.source_revision);
        }
        for (before, after) in reading.actions.iter().zip(round.actions.iter()) {
            assert_eq!(before.action_ref, after.action_ref);
            assert_eq!(before.native_owner, after.native_owner);
        }
        assert_eq!(
            reading.disclosure.instruments, round.disclosure.instruments,
            "disclosure must round-trip including reasons"
        );
        assert_eq!(reading, round, "full reading must round-trip unchanged");
    }
}

#[test]
fn ql_facet_without_warrant_is_rejected() {
    let mut value: Value = serde_json::from_str(REPRESENTATIVE).unwrap();
    value["ql"]["warrant"] = serde_json::Value::Null;
    let parsed: Result<TechneReading, _> = serde_json::from_value(value);
    assert!(
        parsed.is_err(),
        "a QL facet without a warrant must be inexpressible"
    );
}

#[test]
fn view_state_cannot_ride_the_reading_or_the_session() {
    let mut value: Value = serde_json::from_str(REPRESENTATIVE).unwrap();
    value["presentation"] = serde_json::json!({"pan": 12, "zoom": 3});
    assert!(
        serde_json::from_value::<TechneReading>(value).is_err(),
        "presentation state must be inexpressible in a reading"
    );

    assert!(
        !SESSION_SCHEMA.contains("\"pan\""),
        "session schema must not carry view-state fields"
    );
    assert!(
        SESSION_SCHEMA.contains("\"additionalProperties\": false"),
        "session objects are closed"
    );
}

#[test]
fn m_prime_binding_above_five_is_rejected() {
    let mut reading = parse(REPRESENTATIVE);
    reading.disclosure.instruments[0].m_prime = Some(6);
    assert!(reading.validate().is_err(), "the field has M′0–M′5 only");
}

#[test]
fn the_ground_ref_is_step_zero_of_the_traversal() {
    let reading = parse(REPRESENTATIVE);
    assert_eq!(
        reading.ground_ref(),
        reading.whole.as_ref().unwrap().whole_ref
    );
}

#[test]
fn unavailable_instrument_without_reason_fails_validation() {
    let mut reading = parse(ABSENT_FACETS);
    reading.disclosure.instruments[4] = InstrumentDisclosure {
        instrument: TechneInstrument::Place,
        available: false,
        reason: None,
        m_prime: Some(4),
        reading: Some(ql_adapters::TechneReadingKind::DeepFourTwo),
    };
    assert!(reading.validate().is_err());

    let mut session = session_fixture(&reading);
    session.subject_ref = "another:subject".to_string();
    assert!(session.validate().is_err());
}

#[test]
fn schemas_pin_the_language_neutral_contract() {
    for definition in [
        "OpaqueRef",
        "Subject",
        "Whole",
        "WarrantedQlReading",
        "QlWarrant",
        "TemporalFacet",
        "PlaceFacet",
        "SourceProvenance",
        "ExpressionBinding",
        "NativeActionRef",
        "Disclosure",
        "AgencyRole",
    ] {
        assert!(
            READING_SCHEMA.contains(&format!("\"{definition}\"")),
            "reading schema missing definition: {definition}"
        );
    }
    assert!(READING_SCHEMA.contains("\"const\": \"ql.techne/v1\""));
    assert!(
        READING_SCHEMA.contains("mef:lens:L[0-5]'?@1$"),
        "lens refs stay canonical MEF refs"
    );
    assert!(
        !READING_SCHEMA.contains("factory:project"),
        "reading schema must not bake any client vocabulary into QL refs"
    );

    for definition in [
        "DisclosureSelection",
        "DisclosureSession",
        "ActionRoute",
        "ActionRouteReceipt",
        "TechneReadingKind",
    ] {
        assert!(
            SESSION_SCHEMA.contains(&format!("\"{definition}\"")),
            "session schema missing definition: {definition}"
        );
    }
    assert!(SESSION_SCHEMA.contains("never a desktop-owned session record"));
}

#[test]
fn fixture_adapter_routes_actions_to_native_owners_without_executing() {
    let adapter = FixtureTechneAdapter::new();

    let reading = adapter
        .reading("central:now:control:root:b417a7c3d37cd47c2762deee3224687d59db1a9015a322e9a8d7584b3947adf6")
        .expect("development-day reading");
    let receipt = adapter
        .route_action(
            &TechneActionRoute {
                action_ref: "central.now.read".to_string(),
                subject_ref: reading.subject.subject_ref.clone(),
                selection_ref: None,
                input: None,
            },
            &reading,
        )
        .expect("known action routes");

    assert!(receipt.routed);
    assert_eq!(receipt.native_owner, "central/ctrl");
    assert_eq!(receipt.authority.as_deref(), Some("registered-read-action"));
    assert!(
        receipt.reason.is_none(),
        "a routed action carries no refusal reason"
    );

    let refused = adapter
        .route_action(
            &TechneActionRoute {
                action_ref: "not-a-native-action".to_string(),
                subject_ref: reading.subject.subject_ref.clone(),
                selection_ref: None,
                input: None,
            },
            &reading,
        )
        .expect("refusal is still a receipt");
    assert!(!refused.routed);
    assert!(
        refused
            .reason
            .as_deref()
            .unwrap()
            .contains("no native action")
    );

    // Capability discovery is the reading's disclosure, unchanged.
    let capabilities: TechneDisclosure = adapter
        .capabilities("central:source:control:root:Control/user/placement.json")
        .unwrap();
    assert!(
        capabilities
            .instruments
            .iter()
            .any(|entry| entry.instrument == TechneInstrument::Canvas && entry.available)
    );
}

#[test]
fn disclosure_selection_co_reference_holds_across_instruments() {
    let reading = parse(REPRESENTATIVE);
    let graph_selection = DisclosureSelection {
        selection_ref: "ql.techne:selection:fixture:1".to_string(),
        subject_ref: reading.subject.subject_ref.clone(),
        coordinate_ref: None,
        source_ref: Some(reading.provenance[0].source_ref.clone()),
        source_revision: reading.provenance[0].source_revision.clone(),
        disclosure_ref: Some(reading.reading_ref.clone()),
        focus_refs: vec![],
        reading_ref: reading.reading_ref.clone(),
        snapshot_revision: None,
        instrument: TechneInstrument::Canvas,
        agent_session_ref: Some("aikit:agent-session:fixture".to_string()),
        selection_standing: Some("current".to_string()),
    };
    let mut expression_selection = graph_selection.clone();
    expression_selection.instrument = TechneInstrument::Expressions;

    assert!(
        graph_selection.co_referenced(&expression_selection),
        "the same subject on the same reading basis stays co-referenced across instruments"
    );

    let session = DisclosureSession {
        contract: ql_adapters::TECHNE_CONTRACT.to_string(),
        session_ref: "ql.techne:session:fixture:1".to_string(),
        subject_ref: reading.subject.subject_ref.clone(),
        selection: expression_selection.clone(),
        instrument: TechneInstrument::Expressions,
        application_cut: Some(ql_adapters::TechneReadingKind::ConjugateThreeThree),
        whole_ref: None,
        project_ref: None,
        world_ref: None,
        context_frame_ref: None,
        occasion_ref: None,
        return_target_ref: None,
        reading_ref: Some(reading.reading_ref.clone()),
        time_window: None,
        spatial_focus_ref: None,
        reference_frame_ref: None,
        expression_focus_ref: Some(reading.expressions[0].expression_ref.clone()),
        scene_focus_ref: None,
        navigation: vec![ql_adapters::DisclosureNavigation {
            from_instrument: TechneInstrument::Canvas,
            to_instrument: TechneInstrument::Expressions,
            selection_ref: expression_selection.selection_ref.clone(),
        }],
    };
    session.validate().expect("session validates");
}

#[test]
fn tb0_connective_fixture_drives_every_lane() {
    let reading = parse(TB0_CONNECTIVE_BASE);

    // M0′ Project/Wiki/Graph identity: real bounded whole over real sources.
    let whole = reading.whole.as_ref().expect("bounded whole");
    assert!(!whole.member_refs.is_empty());
    assert!(
        whole
            .member_refs
            .iter()
            .all(|member| member.starts_with("central:source:control:root:Work/Quaternal-Logic/"))
    );

    // M2′ Relation Field: several distinct non-temporal typed relation
    // families, one trans-temporal (no dated qualification, interpretation
    // standing) and one temporal-qualified through a real facet ref.
    let relations = &whole.relations;
    assert!(relations.len() >= 4, "several distinct relation families");
    let families: std::collections::HashSet<&str> =
        relations.iter().map(|r| r.relation.as_str()).collect();
    assert_eq!(
        families.len(),
        relations.len(),
        "relation families distinct"
    );
    let instantiates = relations
        .iter()
        .find(|r| r.relation == "INSTANTIATES")
        .expect("trans-temporal archetype relation");
    assert_eq!(instantiates.standing.as_deref(), Some("interpretation"));
    assert!(instantiates.temporal_facet_ref.is_none());
    let implemented_in = relations
        .iter()
        .find(|r| r.relation == "implemented-in")
        .expect("temporal-qualified relation");
    let facet_ref = implemented_in.temporal_facet_ref.as_deref().unwrap();
    assert!(
        reading
            .temporal
            .iter()
            .any(|f| f.facet_ref.as_deref() == Some(facet_ref)),
        "a relation's temporal qualification resolves against a real facet"
    );
    assert!(relations.iter().all(|r| r.relation_ref.is_some()));

    // QL reading: warranted, with the TB0 M-coordinate and Return refs.
    let ql = reading.ql.as_ref().expect("warranted QL reading");
    assert_eq!(ql.m_coordinate_ref.as_deref(), Some("ql:structural:5.0.0"));
    assert!(ql.return_ref.is_some());
    assert_eq!(
        ql.warrant.result_class,
        ql_adapters::QlResultClass::Canonical
    );

    // M2′/M4′ temporal distinctions: occurrence, receipt, validity, day,
    // now, session and run — with attempt and Return continuity refs.
    for kind in [
        ql_adapters::TemporalKind::Occurrence,
        ql_adapters::TemporalKind::Receipt,
        ql_adapters::TemporalKind::Valid,
        ql_adapters::TemporalKind::Day,
        ql_adapters::TemporalKind::Now,
        ql_adapters::TemporalKind::Session,
        ql_adapters::TemporalKind::Run,
    ] {
        assert!(
            reading.temporal.iter().any(|f| f.kind == kind),
            "temporal facet missing: {kind:?}"
        );
    }
    let run = reading
        .temporal
        .iter()
        .find(|f| f.kind == ql_adapters::TemporalKind::Run)
        .unwrap();
    assert!(run.attempt_ref.is_some(), "run carries attempt continuity");
    let session = reading
        .temporal
        .iter()
        .find(|f| f.kind == ql_adapters::TemporalKind::Session)
        .unwrap();
    assert!(
        session.return_ref.is_some(),
        "session carries Return continuity"
    );

    // M4′ World/Places: a dated factual place with uncertainty and hierarchy
    // validity, and a mythic place that is truthfully unlocated.
    assert_eq!(reading.spatial.len(), 2);
    let factual = &reading.spatial[0];
    assert_eq!(factual.relation.as_deref(), Some("OCCURRED_AT"));
    assert_eq!(factual.precision, ql_adapters::PlacePrecision::Approximate);
    assert!(factual.uncertainty.is_some());
    assert!(factual.geometry.is_some());
    assert!(
        factual.hierarchy.iter().any(|h| h.valid_to.is_some()),
        "hierarchy carries historical validity"
    );
    let mythic = &reading.spatial[1];
    assert_eq!(mythic.relation.as_deref(), Some("MYTH_LOCATED_AT"));
    assert_eq!(mythic.precision, ql_adapters::PlacePrecision::Unlocated);
    assert!(
        mythic.geometry.is_none(),
        "an unlocated place carries no geometry"
    );

    // Source selectors and standing.
    assert!(reading.provenance.iter().any(|p| p.selector.is_some()));
    assert!(reading.provenance.iter().all(|p| !p.standing.is_none()));

    // M3′ Journey/Scenes + Expression binding.
    let expression = &reading.expressions[0];
    assert!(expression.scene_ref.is_some());
    assert!(expression.composition_ref.is_some());
    assert!(expression.profile_ref.is_some());

    // Native Actions, including the governed-write Return leg.
    assert!(reading.actions.len() >= 4);
    assert!(
        reading
            .actions
            .iter()
            .any(|a| a.authority == "governed-write")
    );

    // The situated-Agency floor: Guardian stewardship, Anima expressive,
    // Aletheia disclosure and Technē deep-instrument roles.
    assert_eq!(reading.agency.len(), 4);
    let anima = reading
        .agency
        .iter()
        .find(|r| r.role == ql_adapters::AgencyRoleKind::Anima)
        .expect("Anima role example");
    assert_eq!(anima.m_index, 4);
    assert!(matches!(
        anima.reading,
        Some(ql_adapters::TechneReadingKind::ConjugateThreeThree)
    ));
    assert_eq!(anima.instrument, Some(TechneInstrument::Expressions));
    assert!(
        anima.privacy.is_some(),
        "Anima carries its disclosure limits"
    );
    let techne = reading
        .agency
        .iter()
        .find(|r| r.role == ql_adapters::AgencyRoleKind::Techne)
        .expect("Technē role example");
    assert_eq!(techne.m_index, 2);
    assert_eq!(techne.instrument, Some(TechneInstrument::Timeline));
    let guardian = reading
        .agency
        .iter()
        .find(|r| r.role == ql_adapters::AgencyRoleKind::Guardian)
        .expect("Guardian stewardship example");
    assert!(
        guardian.reading.is_none(),
        "stewardship spans both readings"
    );
    for role in &reading.agency {
        assert!(
            role.guardian_ref.is_some(),
            "every situated role names its anchoring Guardian"
        );
    }

    // Capability honesty: one deliberately unavailable instrument with a
    // reason, degraded facets recorded, both cuts disclosed.
    let palace = reading
        .disclosure
        .instruments
        .iter()
        .find(|entry| entry.instrument == TechneInstrument::Palace)
        .unwrap();
    assert!(!palace.available);
    assert!(palace.reason.as_deref().unwrap().len() > 10);
    assert!(!reading.disclosure.degraded.is_empty());
    let deep_cut = reading
        .disclosure
        .cut(ql_adapters::TechneReadingKind::DeepFourTwo)
        .expect("4:2 cut disclosed");
    let conjugate_cut = reading
        .disclosure
        .cut(ql_adapters::TechneReadingKind::ConjugateThreeThree)
        .expect("3:3 cut disclosed");
    assert!(deep_cut.available && conjugate_cut.available);
}

#[test]
fn agency_role_law_rejects_cross_reading_bindings() {
    let mut reading = parse(TB0_CONNECTIVE_BASE);

    let mut anima_as_deep = reading.agency[1].clone();
    anima_as_deep.reading = Some(ql_adapters::TechneReadingKind::DeepFourTwo);
    reading.agency[1] = anima_as_deep;
    assert!(
        reading.validate().is_err(),
        "Anima_i cannot inhabit the 4:2 deep reading"
    );

    let mut reading = parse(TB0_CONNECTIVE_BASE);
    let mut techne_as_conjugate = reading.agency[3].clone();
    techne_as_conjugate.reading = Some(ql_adapters::TechneReadingKind::ConjugateThreeThree);
    reading.agency[3] = techne_as_conjugate;
    assert!(
        reading.validate().is_err(),
        "Technē_i cannot inhabit the 3:3 conjugate reading"
    );

    let mut reading = parse(TB0_CONNECTIVE_BASE);
    let mut techne_wrong_coordinate = reading.agency[3].clone();
    techne_wrong_coordinate.instrument = Some(TechneInstrument::Journey);
    reading.agency[3] = techne_wrong_coordinate;
    assert!(
        reading.validate().is_err(),
        "Technē_2 cannot operate another coordinate's deep instrument"
    );

    let mut reading = parse(TB0_CONNECTIVE_BASE);
    let mut anima_wrong_instrument = reading.agency[1].clone();
    anima_wrong_instrument.instrument = Some(TechneInstrument::Canvas);
    reading.agency[1] = anima_wrong_instrument;
    assert!(
        reading.validate().is_err(),
        "Anima_i operates the Expression reading, not a deep instrument"
    );

    let mut reading = parse(TB0_CONNECTIVE_BASE);
    let mut guardian_everywhere = reading.agency[0].clone();
    guardian_everywhere.reading = Some(ql_adapters::TechneReadingKind::DeepFourTwo);
    reading.agency[0] = guardian_everywhere;
    reading
        .validate()
        .expect("Guardian stewardship may name either reading");
}

#[test]
fn unavailable_application_cut_requires_a_reason() {
    let mut reading = parse(TB0_CONNECTIVE_BASE);
    reading.disclosure.application_cuts[0].available = false;
    assert!(reading.validate().is_err());
    reading.disclosure.application_cuts[0].reason =
        Some("no deep-instrument reading can be composed for this subject".to_string());
    reading.validate().expect("an unavailable cut states why");
}

#[test]
fn cross_cut_crossing_preserves_identity_and_co_reference() {
    let reading = parse(TB0_CONNECTIVE_BASE);

    // A 4:2 deep session over the fixture subject, situated with an
    // AgentSession, occasion and Return target.
    let mut session = session_fixture(&reading);
    session.selection.agent_session_ref = Some("aikit:agent-session:fixture:l5-techne".to_string());
    session.selection.source_ref = Some(reading.provenance[0].source_ref.clone());
    session.selection.source_revision = reading.provenance[0].source_revision.clone();
    session.occasion_ref = Some(
        reading
            .temporal
            .iter()
            .find(|f| f.kind == ql_adapters::TemporalKind::Now)
            .unwrap()
            .now_ref
            .clone()
            .unwrap(),
    );
    session.return_target_ref = Some("aikit:wiki:stage:l5-techne-contract-ground".to_string());
    session.world_ref = Some("central:world:control:root".to_string());
    session.context_frame_ref = Some("mef:context-frame:CF4".to_string());
    session
        .validate()
        .expect("deep session over the fixture subject validates");

    let before_selection = session.selection.clone();
    let before_subject = session.subject_ref.clone();
    let before_occasion = session.occasion_ref.clone();
    let before_return = session.return_target_ref.clone();
    let before_session = session.selection.agent_session_ref.clone();

    // Cross 4:2 → 3:3: disclosure changes, identity does not.
    let crossed = session
        .cross_cut(TechneInstrument::Expressions)
        .expect("the crossing onto the conjugate cut");
    assert!(matches!(
        crossed,
        ql_adapters::TechneReadingKind::ConjugateThreeThree
    ));
    session.expression_focus_ref = Some(reading.expressions[0].expression_ref.clone());
    session.scene_focus_ref = reading.expressions[0].scene_ref.clone();
    session.navigation.push(ql_adapters::DisclosureNavigation {
        from_instrument: TechneInstrument::Timeline,
        to_instrument: TechneInstrument::Expressions,
        selection_ref: session.selection.selection_ref.clone(),
    });
    session.validate().expect("the crossed session validates");

    assert_eq!(
        session.subject_ref, before_subject,
        "subject is not reminted"
    );
    assert_eq!(
        session.selection.source_ref, before_selection.source_ref,
        "source basis is not reminted"
    );
    assert_eq!(
        session.selection.source_revision, before_selection.source_revision,
        "revision basis is not reminted"
    );
    assert_eq!(
        session.occasion_ref, before_occasion,
        "occasion survives the cut"
    );
    assert_eq!(
        session.return_target_ref, before_return,
        "Return target survives"
    );
    assert_eq!(
        session.selection.agent_session_ref, before_session,
        "one AgentSession across the cut"
    );
    assert!(
        session.selection.co_referenced(&before_selection),
        "the two readings stay co-referenced over one subject and reading basis"
    );

    // Reading-level identity is byte-stable: the same reading backs both
    // cuts, its actions and provenance untouched.
    let same_reading_actions = reading.actions.len();
    assert_eq!(same_reading_actions, 4);
    assert!(
        reading
            .actions
            .iter()
            .any(|a| a.action_ref == "oi.expression.open"),
        "the crossing Action is the same native ref either side of the cut"
    );

    // Crossing back to the deep field through the M0′ ground instrument.
    session
        .cross_cut(TechneInstrument::Project)
        .expect("the crossing back onto the deep cut");
    assert_eq!(session.instrument, TechneInstrument::Project);
    session.validate().expect("the returned session validates");

    // A crossing onto the cut already occupied is refused.
    let mut settled = session_fixture(&reading);
    assert!(settled.cross_cut(TechneInstrument::Canvas).is_err());
}

#[test]
fn session_cut_and_instrument_must_agree() {
    let reading = parse(TB0_CONNECTIVE_BASE);
    let mut session = session_fixture(&reading);
    session.application_cut = Some(ql_adapters::TechneReadingKind::ConjugateThreeThree);
    assert!(
        session.validate().is_err(),
        "a 3:3 cut cannot sit under a 4:2 deep instrument"
    );
}

#[test]
fn transport_aliases_map_onto_canonical_instruments() {
    // Research Canvas transport vocabulary resolves onto the canonical
    // contract instruments; unknown names are never guessed.
    assert_eq!(
        ql_adapters::TechneInstrument::from_transport_alias("Story"),
        Some(TechneInstrument::Journey)
    );
    assert_eq!(
        ql_adapters::TechneInstrument::from_transport_alias("globe"),
        Some(TechneInstrument::Place)
    );
    assert_eq!(
        ql_adapters::TechneInstrument::from_transport_alias("Projects"),
        Some(TechneInstrument::Project)
    );
    assert_eq!(
        ql_adapters::TechneInstrument::from_transport_alias("timeline"),
        Some(TechneInstrument::Timeline)
    );
    assert_eq!(
        ql_adapters::TechneInstrument::from_transport_alias("expressions"),
        Some(TechneInstrument::Expressions)
    );
    assert_eq!(
        ql_adapters::TechneInstrument::from_transport_alias("not-a-surface"),
        None
    );

    // The amended geometry is readable off the instrument itself.
    assert_eq!(TechneInstrument::Expressions.m_prime(), None);
    assert!(matches!(
        TechneInstrument::Expressions.reading(),
        ql_adapters::TechneReadingKind::ConjugateThreeThree
    ));
    assert_eq!(TechneInstrument::Timeline.m_prime(), Some(2));
    assert!(matches!(
        TechneInstrument::Palace.reading(),
        ql_adapters::TechneReadingKind::DeepFourTwo
    ));
}

fn session_fixture(reading: &TechneReading) -> DisclosureSession {
    DisclosureSession {
        contract: ql_adapters::TECHNE_CONTRACT.to_string(),
        session_ref: "ql.techne:session:fixture:probe".to_string(),
        subject_ref: reading.subject.subject_ref.clone(),
        selection: DisclosureSelection {
            selection_ref: "ql.techne:selection:fixture:probe".to_string(),
            subject_ref: reading.subject.subject_ref.clone(),
            coordinate_ref: None,
            source_ref: None,
            source_revision: None,
            disclosure_ref: None,
            focus_refs: vec![],
            reading_ref: reading.reading_ref.clone(),
            snapshot_revision: None,
            instrument: TechneInstrument::Timeline,
            agent_session_ref: None,
            selection_standing: None,
        },
        instrument: TechneInstrument::Timeline,
        application_cut: Some(ql_adapters::TechneReadingKind::DeepFourTwo),
        whole_ref: None,
        project_ref: None,
        world_ref: None,
        context_frame_ref: None,
        occasion_ref: None,
        return_target_ref: None,
        reading_ref: Some(reading.reading_ref.clone()),
        time_window: None,
        spatial_focus_ref: None,
        reference_frame_ref: None,
        expression_focus_ref: None,
        scene_focus_ref: None,
        navigation: vec![],
    }
}

/// Conformance adapter: serves the pinned fixtures and resolves native
/// action ownership from the reading itself. It executes nothing.
#[derive(Default)]
struct FixtureTechneAdapter {
    readings: Vec<TechneReading>,
}

impl FixtureTechneAdapter {
    fn new() -> Self {
        Self {
            readings: vec![
                parse(REPRESENTATIVE),
                parse(ABSENT_FACETS),
                parse(DEVELOPMENT_DAY),
            ],
        }
    }
}

impl TechneAdapter for FixtureTechneAdapter {
    fn reading(&self, subject_ref: &str) -> Result<TechneReading, ql_adapters::AdapterError> {
        self.readings
            .iter()
            .find(|reading| reading.subject.subject_ref == subject_ref)
            .cloned()
            .ok_or_else(|| {
                ql_adapters::AdapterError::InvalidTechneReading(format!(
                    "no fixture reading for subject {subject_ref}"
                ))
            })
    }

    fn route_action(
        &self,
        route: &TechneActionRoute,
        reading: &TechneReading,
    ) -> Result<ql_adapters::TechneActionRouteReceipt, ql_adapters::AdapterError> {
        Ok(
            match reading
                .actions
                .iter()
                .find(|a| a.action_ref == route.action_ref)
            {
                Some(action) => ql_adapters::TechneActionRouteReceipt {
                    action_ref: action.action_ref.clone(),
                    native_owner: action.native_owner.clone(),
                    routed: true,
                    reason: None,
                    authority: Some(action.authority.clone()),
                    expected_effects: action.expected_effects.clone(),
                },
                None => ql_adapters::TechneActionRouteReceipt {
                    action_ref: route.action_ref.clone(),
                    native_owner: "unresolved".to_string(),
                    routed: false,
                    reason: Some(
                        "no native action of that ref is disclosed for this subject".to_string(),
                    ),
                    authority: None,
                    expected_effects: vec![],
                },
            },
        )
    }
}
