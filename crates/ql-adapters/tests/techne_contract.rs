use ql_adapters::{
    DisclosureSelection, DisclosureSession, InstrumentDisclosure, TechneActionRoute, TechneAdapter,
    TechneDisclosure, TechneInstrument, TechneReading,
};
use serde_json::Value;

const REPRESENTATIVE: &str =
    include_str!("../../../fixtures/techne/representative-subject-v1.json");
const ABSENT_FACETS: &str = include_str!("../../../fixtures/techne/absent-facets-v1.json");
const DEVELOPMENT_DAY: &str = include_str!("../../../fixtures/techne/development-day-v1.json");
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
        TechneInstrument::Canvas,
        TechneInstrument::Timeline,
        TechneInstrument::Place,
        TechneInstrument::Story,
        TechneInstrument::Palace,
        TechneInstrument::Expressions,
        TechneInstrument::M1234,
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
        *instrument == TechneInstrument::M1234 && reason == "no warranted lens binding"
    }));
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
    for fixture in [REPRESENTATIVE, ABSENT_FACETS, DEVELOPMENT_DAY] {
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
fn unavailable_instrument_without_reason_fails_validation() {
    let mut reading = parse(ABSENT_FACETS);
    reading.disclosure.instruments[2] = InstrumentDisclosure {
        instrument: TechneInstrument::Place,
        available: false,
        reason: None,
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
        reading_ref: Some(reading.reading_ref.clone()),
        time_window: None,
        spatial_focus_ref: None,
        expression_focus_ref: Some(reading.expressions[0].expression_ref.clone()),
        navigation: vec![ql_adapters::DisclosureNavigation {
            from_instrument: TechneInstrument::Canvas,
            to_instrument: TechneInstrument::Expressions,
            selection_ref: expression_selection.selection_ref.clone(),
        }],
    };
    session.validate().expect("session validates");
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
        reading_ref: Some(reading.reading_ref.clone()),
        time_window: None,
        spatial_focus_ref: None,
        expression_focus_ref: None,
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
