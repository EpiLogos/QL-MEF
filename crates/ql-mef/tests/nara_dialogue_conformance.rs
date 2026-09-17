//! Dialogical-Nara semantic conformance (QL-MEF #201, TA3/TA4 contract slice).
//!
//! These fixtures are the wire shape a desktop client (O:I) and Actuation's
//! Nara Agency bind against. The tests prove the three joined-acceptance
//! laws on the exact JSON: a bounded context admitting only disclosed,
//! selected or structural refs; a deixis round trip resolving exact refs
//! without inventing any; and an Epii delegation that stays basis-bound and
//! refuses stale application. No transport, provider or renderer is mounted.

use ql_mef::nara::dialogue::{
    DeixisOutcome, DeixisRequest, EpiiDelegation, EpiiEnrichment, NARA_DEIXIS_CONTRACT,
    NARA_DIALOGUE_CONTEXT_CONTRACT, NARA_EPII_DELEGATION_CONTRACT, NaraDialogueContext,
};
use ql_mef::nara::voice::{
    ContextRefreshDisposition, NARA_VOICE_BODY_CONTRACT, VoiceBodyBinding, VoiceBodyDeclaration,
    VoiceBodyRequirements, VoiceCapabilityStatus, VoiceDuplexDisposition,
};
use serde_json::{Value, json};

fn context_fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/nara/nara-dialogue-context-v1.json"
    ))
    .unwrap()
}

fn context() -> NaraDialogueContext {
    serde_json::from_value(context_fixture()).unwrap()
}

/// The fixture is exactly the typed contract's wire shape: deserialising,
/// validating and reserialising reproduces the committed bytes field for
/// field. `deny_unknown_fields` refuses any extra key a client invents.
#[test]
fn bounded_context_fixture_is_the_exact_wire_shape() {
    let fixture = context_fixture();
    let value = context();
    value.validate().unwrap();
    assert_eq!(value.schema, NARA_DIALOGUE_CONTEXT_CONTRACT);
    assert_eq!(serde_json::to_value(&value).unwrap(), fixture);
}

/// Admission law: only disclosed, selected, or structural refs enter the
/// turn. The private source in this scenario appears nowhere in the fixture.
#[test]
fn bounded_context_admits_only_disclosed_selected_and_structural_refs() {
    let value = context();
    let admitted = [
        "source:fixture-disclosed-1",
        "bimba:relation:1",
        "bimba:source:M4.1",
        "expression:fixture-1",
        "bimba:#4",
        "pratibimba:#4",
        "scene:fixture-1",
    ];
    for ref_id in admitted {
        assert!(value.is_admitted(ref_id), "{ref_id} should be admitted");
    }
    let private = "source:fixture-private";
    assert!(
        !serde_json::to_string(&context_fixture())
            .unwrap()
            .contains(private),
        "the fixture must not carry the private source"
    );
    assert!(!value.is_admitted(private));
    let error = value.admit_refs([private]).unwrap_err();
    assert!(
        error.contains("neither disclosed, selected, nor structural"),
        "{error}"
    );
}

/// Deixis round trip on the fixture's own requests and expectations.
#[test]
fn deixis_fixture_round_trips_on_exact_refs() {
    let fixture: Value =
        serde_json::from_str(include_str!("../../../fixtures/nara/nara-deixis-v1.json")).unwrap();
    assert_eq!(fixture["schema"], NARA_DEIXIS_CONTRACT);
    let value = context();

    for case in fixture["requests"].as_array().unwrap() {
        let request: DeixisRequest = serde_json::from_value(case["request"].clone()).unwrap();
        let expected = &case["expected"];
        match value.resolve_deixis(&request) {
            Err(error) => {
                assert_eq!(
                    expected["outcome"], "error",
                    "unexpected deixis refusal: {error}"
                );
                assert!(
                    error.contains(expected["error_contains"].as_str().unwrap()),
                    "{error}"
                );
            }
            Ok(resolution) => {
                assert_ne!(
                    expected["outcome"], "error",
                    "expected a refusal, got {resolution:?}"
                );
                resolution.validate().unwrap();
                let encoded = serde_json::to_value(&resolution).unwrap();
                match expected["outcome"].as_str().unwrap() {
                    "focused" => {
                        let DeixisOutcome::Focused {
                            focus,
                            turn_context,
                        } = &resolution.outcome
                        else {
                            panic!("expected focused outcome, got {}", encoded["outcome"]);
                        };
                        let focus_refs: Vec<&str> =
                            focus.iter().map(|item| item.ref_id.as_str()).collect();
                        assert_eq!(
                            focus_refs,
                            expected["focus_refs"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| v.as_str().unwrap())
                                .collect::<Vec<_>>()
                        );
                        assert!(turn_context.turn_refs.iter().any(|turn| turn
                            == expected["turn_refs_contain"].as_str().unwrap()));
                        assert_eq!(
                            turn_context.expression_revision,
                            expected["expression_revision"].as_str().unwrap()
                        );
                    }
                    "unresolved-outside-context" => {
                        assert_eq!(
                            encoded["outcome"],
                            json!({
                                "outcome": "unresolved-outside-context",
                                "ref_id": expected["ref_id"]
                            })
                        );
                    }
                    other => panic!("unknown expected outcome {other}"),
                }
            }
        }
    }
}

/// The delegation fixture: admitted scope only, basis-bound result, refused
/// stale application, refused scope escape.
#[test]
fn epii_delegation_fixture_stays_basis_bound_and_refuses_stale_application() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/nara/nara-epii-delegation-v1.json"
    ))
    .unwrap();
    assert_eq!(fixture["schema"], NARA_EPII_DELEGATION_CONTRACT);
    let context: NaraDialogueContext = serde_json::from_value(fixture["context"].clone()).unwrap();
    let moved: NaraDialogueContext =
        serde_json::from_value(fixture["moved_context"].clone()).unwrap();
    let enrichment: EpiiEnrichment = serde_json::from_value(fixture["enrichment"].clone()).unwrap();
    let escaping: EpiiEnrichment =
        serde_json::from_value(fixture["enrichment_escaping_scope"].clone()).unwrap();
    let delegation_spec = &fixture["delegation"];
    let expected = &fixture["expected"];

    let delegate = |scope_candidates: Vec<String>| {
        EpiiDelegation::from_context(
            delegation_spec["delegation_ref"]
                .as_str()
                .unwrap()
                .to_string(),
            &context,
            delegation_spec["epii_session_ref"]
                .as_str()
                .unwrap()
                .to_string(),
            delegation_spec["brief"].as_str().unwrap().to_string(),
            scope_candidates,
            delegation_spec["delegated_at_unix_ms"].as_u64().unwrap(),
        )
    };
    let admitted_scope: Vec<String> = delegation_spec["scope_candidates_admitted"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    // An undisclosed candidate poisons the whole scope: no silent narrowing.
    let poisoned: Vec<String> = delegation_spec["scope_candidates_with_undisclosed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    let error = delegate(poisoned).unwrap_err();
    assert!(
        error.contains(
            expected["undisclosed_scope_error_contains"]
                .as_str()
                .unwrap()
        ),
        "{error}"
    );

    // The admitted scope delegates cleanly and round-trips the wire shape.
    let mut delegation = delegate(admitted_scope.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(&delegation).unwrap(),
        json!({
            "schema": NARA_EPII_DELEGATION_CONTRACT,
            "delegation_ref": "delegation:fixture-1",
            "nara_ref": "nara:fixture-a",
            "epii_session_ref": "epii:session:fixture-1",
            "basis": {
                "context_ref": "dialogue-context:fixture-1",
                "expression_ref": "expression:fixture-1",
                "expression_revision": "rev-7",
                "profile_ref": "profile:M4.1",
                "profile_revision": "prof-2",
                "coordinate_ref": "M4.1.1",
                "bimba_registry_revision": "registry:fixture-r7",
                "occasion": fixture["context"]["occasion"]
            },
            "brief": "What stands behind the pointed relation?",
            "scope_refs": ["source:fixture-disclosed-1"],
            "delegated_at_unix_ms": 15,
            "state": {"state": "delegated"}
        })
    );

    // Receiving retains the enrichment as returned material; it applies
    // nothing by itself.
    delegation.receive_enrichment(&enrichment).unwrap();
    assert!(matches!(
        delegation.state,
        ql_mef::nara::dialogue::DelegationState::Returned { .. }
    ));

    // A second answer to the same delegation is refused; a new answer needs a
    // new delegation.
    assert!(delegation.receive_enrichment(&escaping).is_err());

    // Live at the basis revision: the gate admits proposing (never applying).
    delegation.apply_gate(&enrichment, &context).unwrap();

    // The encounter moved on: the late result is retained material only.
    let error = delegation.apply_gate(&enrichment, &moved).unwrap_err();
    assert!(
        error.contains(
            expected["stale_application_error_contains"]
                .as_str()
                .unwrap()
        ),
        "{error}"
    );

    // Proposals may not escape the delegated scope.
    let fresh = delegate(admitted_scope).unwrap();
    let error = fresh.apply_gate(&escaping, &context).unwrap_err();
    assert!(
        error.contains(expected["escaping_scope_error_contains"].as_str().unwrap()),
        "{error}"
    );
}

/// Voice-body law: the body is a distinct identity and unknown capabilities
/// never silently satisfy the dialogical floor.
#[test]
fn voice_body_identity_and_dialogical_floor() {
    let binding = VoiceBodyBinding {
        schema: NARA_VOICE_BODY_CONTRACT.into(),
        body_ref: "voice-body:fixture-1".into(),
        body_provenance_ref: "provenance:resolved-by-aikit".into(),
    };
    binding
        .binds_distinctly("nara:fixture-a", "session:fixture-1")
        .unwrap();
    assert!(
        binding
            .binds_distinctly("voice-body:fixture-1", "session:fixture-1")
            .is_err()
    );

    let capable = VoiceBodyDeclaration {
        binding: binding.clone(),
        duplex: VoiceDuplexDisposition::FullDuplex,
        barge_in: VoiceCapabilityStatus::Supported,
        manual_interrupt: VoiceCapabilityStatus::Supported,
        structured_event_channel: VoiceCapabilityStatus::Supported,
        reconnect_status_reporting: VoiceCapabilityStatus::Supported,
        context_refresh: ContextRefreshDisposition::ToolAccess,
        observation_refs: vec!["observation:fixture-latency".into()],
    };
    capable
        .satisfies(&VoiceBodyRequirements::dialogical_floor())
        .unwrap();

    let mut silent = capable.clone();
    silent.barge_in = VoiceCapabilityStatus::Unknown;
    let error = silent
        .satisfies(&VoiceBodyRequirements::dialogical_floor())
        .unwrap_err();
    assert!(error.contains("barge-in"), "{error}");
}
