mod support;

use ql_adapters::{
    AdapterError, ClientRecord, QlAttachment, QlMode, RuntimeEnvelope, RuntimeRefractionAdapter,
    RuntimeSelection, RuntimeStatus, RuntimeSubject,
};
use ql_mef::{LensId, LensRef};
use ql_semantic::ProviderHealth;
use ql_service::QlService;

use support::AdapterFixtureProvider;

fn classic_event() -> ClientRecord<RuntimeSubject, RuntimeEnvelope<&'static str>> {
    let subject = RuntimeSubject::event("factory:runtime:event/run-1/0", Some("0".into()))
        .expect("runtime event subject");
    let envelope = RuntimeEnvelope::new(
        RuntimeSelection::new("classic", "foundation-freeze/1"),
        RuntimeStatus::new("completed", "not_applicable"),
        "unchanged-host-event",
    )
    .with_event("host", "model-returned");
    ClientRecord::new(subject, envelope)
}

fn ql_event() -> ClientRecord<RuntimeSubject, RuntimeEnvelope<&'static str>> {
    let subject = RuntimeSubject::event("factory:runtime:event/run-2/4", Some("4".into()))
        .expect("runtime event subject");
    let envelope = RuntimeEnvelope::new(
        RuntimeSelection::new("ql-core", "foundation-freeze/1"),
        RuntimeStatus::new("completed", "closed"),
        "retained-runtime-semantic-event",
    )
    .with_event("runtime-semantic", "closure-attained");
    ClientRecord::new(subject, envelope)
}

#[test]
fn classic_runtime_remains_ql_free_when_refraction_is_disabled() {
    let adapter = RuntimeRefractionAdapter::new(None, QlMode::Disabled);
    let result = adapter
        .refract(
            classic_event(),
            LensRef::canonical(LensId::L3Prime),
            None,
            None,
        )
        .expect("disabled bridge remains valid");

    assert_eq!(result.client.payload.runtime.id, "classic");
    assert_eq!(
        result.client.payload.runtime.revision,
        "foundation-freeze/1"
    );
    assert_eq!(result.client.payload.status.execution, "completed");
    assert_eq!(result.client.payload.status.semantic, "not_applicable");
    assert_eq!(result.client.payload.channel.as_deref(), Some("host"));
    assert_eq!(result.client.payload.payload, "unchanged-host-event");
    assert_eq!(result.ql, QlAttachment::Disabled);
}

#[test]
fn optional_provider_absence_does_not_corrupt_runtime_state_or_event() {
    let adapter = RuntimeRefractionAdapter::new(None, QlMode::Optional);
    let result = adapter
        .refract(
            ql_event(),
            LensRef::canonical(LensId::L3),
            None,
            None,
        )
        .expect("optional absence is nonfatal");

    assert_eq!(result.client.payload.runtime.id, "ql-core");
    assert_eq!(result.client.payload.status.semantic, "closed");
    assert_eq!(
        result.client.payload.event_type.as_deref(),
        Some("closure-attained")
    );
    assert_eq!(
        result.client.payload.payload,
        "retained-runtime-semantic-event"
    );
    assert!(matches!(result.ql, QlAttachment::Unavailable { .. }));
}

#[test]
fn runtime_selection_and_provider_selection_are_independent_variables() {
    let service = QlService::with_provider(AdapterFixtureProvider::semantic(
        "ql-provider:q5",
        ProviderHealth::available(),
    ));
    let adapter = RuntimeRefractionAdapter::new(Some(&service), QlMode::Optional);
    let result = adapter
        .refract(
            classic_event(),
            LensRef::canonical(LensId::L3Prime),
            None,
            None,
        )
        .expect("external refraction can read a classic trace");

    assert_eq!(result.client.payload.runtime.id, "classic");
    assert_eq!(result.client.payload.status.semantic, "not_applicable");
    match result.ql {
        QlAttachment::Reading { value, .. } => {
            assert_eq!(
                value.target.subject.as_str(),
                "factory:runtime:event/run-1/0"
            );
            assert_eq!(value.provenance.provider.provider, "ql-provider:q5");
            assert_eq!(
                value.provenance.input_refs[0].reference.as_str(),
                "factory:runtime:event/run-1/0"
            );
        }
        other => panic!("expected semantic reading, got {other:?}"),
    }
}

#[test]
fn provider_failure_is_an_attachment_failure_not_a_runtime_failure() {
    let service = QlService::with_provider(AdapterFixtureProvider::failing("ql-provider:q5-fail"));
    let adapter = RuntimeRefractionAdapter::new(Some(&service), QlMode::Optional);
    let result = adapter
        .refract(
            ql_event(),
            LensRef::canonical(LensId::L4Prime),
            None,
            None,
        )
        .expect("optional provider failure is contained");

    assert_eq!(result.client.payload.status.execution, "completed");
    assert_eq!(result.client.payload.status.semantic, "closed");
    assert!(matches!(result.ql, QlAttachment::Failed { .. }));
}

#[test]
fn required_provider_absence_is_explicit_without_changing_the_runtime_contract() {
    let adapter = RuntimeRefractionAdapter::new(None, QlMode::Required);
    let error = adapter
        .refract(
            ql_event(),
            LensRef::canonical(LensId::L1),
            None,
            None,
        )
        .expect_err("required mode must fail when no provider is supplied");

    assert_eq!(error, AdapterError::ServiceUnavailable);
}

#[test]
fn run_event_and_closure_subjects_keep_client_owned_refs() {
    let run = RuntimeSubject::run("factory:run:alpha", Some("r1".into())).unwrap();
    let event = RuntimeSubject::event("factory:event:alpha/7", Some("7".into())).unwrap();
    let closure = RuntimeSubject::closure("factory:closure:alpha", Some("c1".into())).unwrap();

    assert_eq!(run.inner().reference().as_str(), "factory:run:alpha");
    assert_eq!(event.inner().reference().as_str(), "factory:event:alpha/7");
    assert_eq!(
        closure.inner().reference().as_str(),
        "factory:closure:alpha"
    );
    assert_eq!(run.inner().revision(), Some("r1"));
    assert_eq!(event.inner().revision(), Some("7"));
    assert_eq!(closure.inner().revision(), Some("c1"));
}
