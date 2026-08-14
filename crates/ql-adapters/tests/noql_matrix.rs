mod support;

use ql_adapters::{
    AdapterError, AiKitAdapter, AiKitSubject, ClientRecord, FactoryAdapter, FactorySubject,
    QlAttachment, QlMode,
};
use ql_mef::{LensId, LensRef, ResultClass, SublensRef};
use ql_semantic::{ProviderHealth, ProviderState};
use ql_service::{QlService, ServiceError};

use support::AdapterFixtureProvider;

fn factory_record(reference: &str, revision: &str, payload: &str) -> ClientRecord<FactorySubject, String> {
    ClientRecord::new(
        FactorySubject::new(reference, Some(revision.into())).expect("factory subject"),
        payload.into(),
    )
}

fn aikit_record(reference: &str, revision: &str, payload: &str) -> ClientRecord<AiKitSubject, String> {
    ClientRecord::new(
        AiKitSubject::new(reference, Some(revision.into())).expect("aikit subject"),
        payload.into(),
    )
}

#[test]
fn factory_disabled_mode_preserves_client_data_exactly() {
    let adapter = FactoryAdapter::new(None, QlMode::Disabled);
    let result = adapter
        .refract(
            factory_record("factory:claim:c-1", "sha256:claim-c-1-r1", "original payload"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("disabled QL is non-fatal");

    assert_eq!(result.client.subject.inner().reference().as_str(), "factory:claim:c-1");
    assert_eq!(result.client.subject.inner().revision(), Some("sha256:claim-c-1-r1"));
    assert_eq!(result.client.payload, "original payload");
    assert_eq!(result.ql, QlAttachment::Disabled);
}

#[test]
fn aikit_disabled_mode_preserves_client_data_exactly() {
    let adapter = AiKitAdapter::new(None, QlMode::Disabled);
    let result = adapter
        .refract(
            aikit_record("aikit:context:ctx-1", "sha256:ctx-r1", "context payload"),
            LensRef::canonical(LensId::L4Prime),
            None,
            None,
        )
        .expect("disabled QL is non-fatal");

    assert_eq!(result.client.subject.inner().reference().as_str(), "aikit:context:ctx-1");
    assert_eq!(result.client.subject.inner().revision(), Some("sha256:ctx-r1"));
    assert_eq!(result.client.payload, "context payload");
    assert_eq!(result.ql, QlAttachment::Disabled);
}

#[test]
fn optional_mode_without_service_is_non_fatal_and_preserves_identity() {
    let adapter = FactoryAdapter::new(None, QlMode::Optional);
    let result = adapter
        .refract(
            factory_record("factory:claim:c-1", "sha256:claim-c-1-r1", "payload"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("optional absence is non-fatal");

    assert_eq!(result.client.subject.inner().reference().as_str(), "factory:claim:c-1");
    assert_eq!(result.client.payload, "payload");
    match result.ql {
        QlAttachment::Unavailable { health, .. } => assert_eq!(health.state, ProviderState::Absent),
        other => panic!("expected unavailable attachment, got {other:?}"),
    }
}

#[test]
fn optional_incompatible_and_unadvertised_provider_states_are_non_fatal() {
    let incompatible_service = QlService::with_provider(AdapterFixtureProvider::semantic(
        "incompatible",
        ProviderHealth::incompatible("schema mismatch"),
    ));
    let incompatible = FactoryAdapter::new(Some(&incompatible_service), QlMode::Optional)
        .refract(
            factory_record("factory:claim:c-1", "r1", "payload"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("optional incompatible provider is non-fatal");
    assert!(matches!(
        incompatible.ql,
        QlAttachment::Unavailable { health, .. } if health.state == ProviderState::Incompatible
    ));
    assert_eq!(incompatible.client.payload, "payload");

    let formal_service = QlService::with_provider(AdapterFixtureProvider::formal_only("formal-only"));
    let unadvertised = FactoryAdapter::new(Some(&formal_service), QlMode::Optional)
        .refract(
            factory_record("factory:claim:c-1", "r1", "payload"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("optional unadvertised operation is non-fatal");
    assert!(matches!(unadvertised.ql, QlAttachment::Unavailable { .. }));
    assert_eq!(unadvertised.client.payload, "payload");
}

#[test]
fn degraded_provider_can_enrich_without_becoming_a_prerequisite() {
    let service = QlService::with_provider(AdapterFixtureProvider::semantic(
        "degraded",
        ProviderHealth::degraded("one semantic source unavailable"),
    ));
    let result = FactoryAdapter::new(Some(&service), QlMode::Optional)
        .refract(
            factory_record("factory:claim:c-1", "sha256:claim-c-1-r1", "payload"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("degraded advertised refraction remains usable");

    assert_eq!(result.client.subject.inner().reference().as_str(), "factory:claim:c-1");
    match result.ql {
        QlAttachment::Reading { health, value } => {
            assert_eq!(health.state, ProviderState::Degraded);
            assert_eq!(value.target.subject.as_str(), "factory:claim:c-1");
            assert_eq!(value.provenance.provider.provider, "degraded");
            assert_eq!(value.provenance.provider.version, "0.1.0");
            assert_eq!(value.provenance.result_class, ResultClass::SemanticStochastic);
            assert_eq!(value.provenance.model.as_deref(), Some("adapter-fixture-model"));
            assert_eq!(value.provenance.config_ref.as_ref().map(|value| value.as_str()), Some("fixture:config/q4"));
        }
        other => panic!("expected reading attachment, got {other:?}"),
    }
}

#[test]
fn required_mode_fails_hard_when_ql_is_absent_or_provider_fails() {
    let no_service = FactoryAdapter::new(None, QlMode::Required)
        .refract(
            factory_record("factory:claim:c-1", "r1", "payload"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect_err("required QL must fail without service");
    assert_eq!(no_service, AdapterError::ServiceUnavailable);

    let failing_service = QlService::with_provider(AdapterFixtureProvider::failing("failing"));
    let failure = FactoryAdapter::new(Some(&failing_service), QlMode::Required)
        .refract(
            factory_record("factory:claim:c-1", "r1", "payload"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect_err("required provider failure must be visible");
    assert!(matches!(failure, AdapterError::QlRequired(ServiceError::Provider(_))));
}

#[test]
fn invalid_sublens_relation_is_never_coerced_even_in_optional_mode() {
    let service = QlService::with_provider(AdapterFixtureProvider::semantic(
        "fixture",
        ProviderHealth::available(),
    ));
    let error = FactoryAdapter::new(Some(&service), QlMode::Optional)
        .refract(
            factory_record("factory:claim:c-1", "r1", "payload"),
            LensRef::canonical(LensId::L1),
            Some(SublensRef::canonical(LensId::L4, 0).expect("sublens")),
            None,
        )
        .expect_err("mismatched coordinate must fail before provider execution");
    assert!(matches!(error, AdapterError::InvalidRefraction(_)));
}

#[test]
fn factory_and_aikit_surfaces_preserve_the_same_shared_ref_without_translation() {
    let service = QlService::with_provider(AdapterFixtureProvider::semantic(
        "fixture",
        ProviderHealth::available(),
    ));
    let shared_ref = "factory:claim:c-1";
    let revision = "sha256:claim-c-1-r1";

    let factory = FactoryAdapter::new(Some(&service), QlMode::Optional)
        .refract(
            factory_record(shared_ref, revision, "factory view"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("factory refraction");
    let aikit = AiKitAdapter::new(Some(&service), QlMode::Optional)
        .refract(
            aikit_record(shared_ref, revision, "aikit view"),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("aikit refraction");

    assert_eq!(factory.client.subject.inner().reference().as_str(), shared_ref);
    assert_eq!(aikit.client.subject.inner().reference().as_str(), shared_ref);
    assert_eq!(factory.client.subject.inner().revision(), Some(revision));
    assert_eq!(aikit.client.subject.inner().revision(), Some(revision));

    let factory_target = match factory.ql {
        QlAttachment::Reading { value, .. } => value.target.subject,
        other => panic!("expected factory reading, got {other:?}"),
    };
    let aikit_target = match aikit.ql {
        QlAttachment::Reading { value, .. } => value.target.subject,
        other => panic!("expected AIKit reading, got {other:?}"),
    };
    assert_eq!(factory_target.as_str(), shared_ref);
    assert_eq!(aikit_target.as_str(), shared_ref);
}
