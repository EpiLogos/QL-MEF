mod support;

use ql_mef::{LensId, LensRef, ResultClass};
use ql_semantic::{
    LocateRequest, Operation, ProviderClass, ProviderHealth, ProviderState, RefractRequest,
    RelateRequest, SynthesiseRequest,
};
use ql_service::{QlService, ServiceError, ServiceRequest, ServiceResponse};

use support::{FixtureProvider, input};

#[test]
fn absent_degraded_and_incompatible_states_are_explicit() {
    let absent = QlService::new();
    assert_eq!(absent.capabilities().health.state, ProviderState::Absent);
    assert_eq!(
        absent.locate(LocateRequest {
            input: input("client:a", None),
            frame: None,
        }),
        Err(ServiceError::ProviderAbsent)
    );

    let degraded = QlService::with_provider(FixtureProvider::full(
        "degraded",
        ProviderHealth::degraded("semantic source partially unavailable"),
    ));
    let decision = degraded.negotiate(Operation::Refract);
    assert_eq!(decision.health.state, ProviderState::Degraded);
    assert!(decision.supported);
    assert!(!decision.deterministic);

    let incompatible = QlService::with_provider(FixtureProvider::full(
        "incompatible",
        ProviderHealth::incompatible("schema major mismatch"),
    ));
    assert!(!incompatible.negotiate(Operation::Locate).supported);
    assert!(matches!(
        incompatible.locate(LocateRequest {
            input: input("client:a", None),
            frame: None,
        }),
        Err(ServiceError::ProviderIncompatible(_))
    ));
}

#[test]
fn formal_and_semantic_capability_classes_are_separable() {
    let formal = QlService::with_provider(FixtureProvider::locate_only("formal-only"));
    let capabilities = formal.capabilities().provider.expect("provider");
    assert_eq!(capabilities.classes, [ProviderClass::FormalKernel]);
    assert!(capabilities.supported_lenses.is_empty());
    assert!(formal.negotiate(Operation::Locate).supported);
    assert!(!formal.negotiate(Operation::Refract).supported);

    let full = QlService::with_provider(FixtureProvider::full("full", ProviderHealth::available()));
    let capabilities = full.capabilities().provider.expect("provider");
    assert!(capabilities.classes.contains(&ProviderClass::FormalKernel));
    assert!(
        capabilities
            .classes
            .contains(&ProviderClass::SemanticRefraction)
    );
}

#[test]
fn unadvertised_advanced_operation_is_not_inferred() {
    let service = QlService::with_provider(FixtureProvider::locate_only("formal-only"));
    let error = service
        .refract(RefractRequest {
            input: input("client:a", None),
            lens: LensRef::canonical(LensId::L4),
            sublens: None,
            frame: None,
        })
        .expect_err("refract must not be inferred");
    assert_eq!(
        error,
        ServiceError::UnsupportedOperation(Operation::Refract)
    );
}

#[test]
fn provider_is_replaceable_through_the_public_seam() {
    let mut service = QlService::with_provider(FixtureProvider::full(
        "provider-a",
        ProviderHealth::available(),
    ));
    assert_eq!(
        service
            .capabilities()
            .provider
            .expect("provider")
            .provider
            .provider,
        "provider-a"
    );
    service.replace_provider(FixtureProvider::full(
        "provider-b",
        ProviderHealth::available(),
    ));
    assert_eq!(
        service
            .capabilities()
            .provider
            .expect("provider")
            .provider
            .provider,
        "provider-b"
    );
    service.clear_provider();
    assert_eq!(service.capabilities().health.state, ProviderState::Absent);
}

#[test]
fn deterministic_locate_fixture_replays_identically() {
    let service = QlService::with_provider(FixtureProvider::full(
        "fixture",
        ProviderHealth::available(),
    ));
    let request = LocateRequest {
        input: input("client:artifact/a", Some("source-rev-a")),
        frame: None,
    };
    let first = service.locate(request.clone()).expect("first locate");
    let second = service.locate(request).expect("second locate");
    assert_eq!(first, second);
    assert_eq!(first.provenance.result_class, ResultClass::Deterministic);
    assert_eq!(
        first.provenance.input_refs[0].revision.as_deref(),
        Some("source-rev-a")
    );
    assert!(service.negotiate(Operation::Locate).deterministic);
}

#[test]
fn transport_envelope_preserves_deterministic_operation_semantics() {
    let service = QlService::with_provider(FixtureProvider::full(
        "fixture",
        ProviderHealth::available(),
    ));
    let request = LocateRequest {
        input: input("client:artifact/transport", Some("transport-rev")),
        frame: None,
    };
    let direct = service.locate(request.clone()).expect("direct locate");
    let dispatched = service
        .dispatch(ServiceRequest::Locate(request))
        .expect("dispatched locate");
    match dispatched {
        ServiceResponse::Locate(result) => assert_eq!(result, direct),
        _ => panic!("dispatch changed operation response type"),
    }
}

#[test]
fn stochastic_refraction_carries_model_config_source_subject_and_revision() {
    let service = QlService::with_provider(FixtureProvider::full(
        "fixture",
        ProviderHealth::available(),
    ));
    let subject = input("client:artifact/a", Some("source-rev-a"));
    let reading = service
        .refract(RefractRequest {
            input: subject.clone(),
            lens: LensRef::canonical(LensId::L4Prime),
            sublens: None,
            frame: None,
        })
        .expect("semantic reading");
    assert_eq!(reading.target.subject, subject.target.subject);
    assert_eq!(
        reading.provenance.result_class,
        ResultClass::SemanticStochastic
    );
    assert_eq!(
        reading.provenance.model.as_deref(),
        Some("fixture-semantic-model")
    );
    assert_eq!(
        reading
            .provenance
            .config_ref
            .as_ref()
            .map(|value| value.as_str()),
        Some("fixture:config/q3")
    );
    assert_eq!(
        reading.provenance.input_refs[0].revision.as_deref(),
        Some("source-rev-a")
    );
    assert_eq!(reading.evidence_refs[0].as_str(), "fixture:source/corpus-1");
}

#[test]
fn relate_preserves_each_caller_revision() {
    let service = QlService::with_provider(FixtureProvider::full(
        "fixture",
        ProviderHealth::available(),
    ));
    let relation = service
        .relate(RelateRequest {
            inputs: vec![
                input("client:a", Some("rev-a")),
                input("client:b", Some("rev-b")),
            ],
            frame: None,
            lenses: vec![LensRef::canonical(LensId::L2)],
        })
        .expect("relation");
    assert_eq!(relation.subjects.len(), 2);
    assert_eq!(
        relation.provenance.result_class,
        ResultClass::SemanticStochastic
    );
    assert_eq!(
        relation.provenance.input_refs[0].revision.as_deref(),
        Some("rev-a")
    );
    assert_eq!(
        relation.provenance.input_refs[1].revision.as_deref(),
        Some("rev-b")
    );
}

#[test]
fn relate_and_synthesise_preserve_sources_differences_and_unresolved_material() {
    let service = QlService::with_provider(FixtureProvider::full(
        "fixture",
        ProviderHealth::available(),
    ));
    let relation = service
        .relate(RelateRequest {
            inputs: vec![input("client:a", None), input("client:b", None)],
            frame: None,
            lenses: vec![LensRef::canonical(LensId::L2)],
        })
        .expect("relation");
    assert_eq!(relation.subjects.len(), 2);
    assert_eq!(
        relation.provenance.result_class,
        ResultClass::SemanticStochastic
    );

    let reading_a = service
        .refract(RefractRequest {
            input: input("client:a", None),
            lens: LensRef::canonical(LensId::L1),
            sublens: None,
            frame: None,
        })
        .expect("reading a");
    let reading_b = service
        .refract(RefractRequest {
            input: input("client:b", None),
            lens: LensRef::canonical(LensId::L4),
            sublens: None,
            frame: None,
        })
        .expect("reading b");
    let synthesis = service
        .synthesise(SynthesiseRequest {
            readings: vec![reading_a, reading_b],
            frame: None,
        })
        .expect("synthesis");
    assert_eq!(synthesis.retained_differences, ["difference retained"]);
    assert_eq!(synthesis.unresolved, ["unresolved question"]);
    assert_eq!(synthesis.synthesis.tensions, ["tension retained"]);
}

#[test]
fn service_rejects_invalid_arity_before_provider_execution() {
    let service = QlService::with_provider(FixtureProvider::full(
        "fixture",
        ProviderHealth::available(),
    ));
    assert_eq!(
        service.relate(RelateRequest {
            inputs: vec![input("client:only", None)],
            frame: None,
            lenses: vec![],
        }),
        Err(ServiceError::InvalidRequest(
            "relate requires at least two subjects"
        ))
    );
    assert_eq!(
        service.synthesise(SynthesiseRequest {
            readings: vec![],
            frame: None,
        }),
        Err(ServiceError::InvalidRequest(
            "synthesise requires at least one reading"
        ))
    );
}

#[test]
fn service_enforces_advertised_input_limits() {
    let service = QlService::with_provider(FixtureProvider::full(
        "fixture",
        ProviderHealth::available(),
    ));
    let error = service
        .relate(RelateRequest {
            inputs: (0..5)
                .map(|index| input(&format!("client:{index}"), None))
                .collect(),
            frame: None,
            lenses: vec![],
        })
        .expect_err("limit must be enforced");
    assert!(matches!(
        error,
        ServiceError::InputLimitExceeded {
            operation: Operation::Relate,
            limit: 4,
            actual: 5
        }
    ));
}
