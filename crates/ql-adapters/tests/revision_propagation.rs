mod support;

use ql_adapters::{ClientRecord, FactoryAdapter, FactorySubject, QlAttachment, QlMode};
use ql_mef::{LensId, LensRef};
use ql_semantic::{ProviderHealth, ProviderState};
use ql_service::QlService;

use support::AdapterFixtureProvider;

#[test]
fn factory_client_revision_reaches_provider_provenance_unchanged() {
    let service = QlService::with_provider(AdapterFixtureProvider::semantic(
        "fixture",
        ProviderHealth::available(),
    ));
    let adapter = FactoryAdapter::new(Some(&service), QlMode::Optional);
    let client = ClientRecord::new(
        FactorySubject::new("factory:claim:c-1", Some("sha256:claim-c-1-r1".into()))
            .expect("factory subject"),
        "payload".to_owned(),
    );

    let result = adapter
        .refract(client, LensRef::canonical(LensId::L3), None, None)
        .expect("optional refraction");

    assert_eq!(
        result.client.subject.inner().reference().as_str(),
        "factory:claim:c-1"
    );
    assert_eq!(
        result.client.subject.inner().revision(),
        Some("sha256:claim-c-1-r1")
    );

    match result.ql {
        QlAttachment::Reading { health, value } => {
            assert_eq!(health.state, ProviderState::Available);
            assert_eq!(value.target.subject.as_str(), "factory:claim:c-1");
            assert_eq!(
                value.provenance.input_refs[0].reference.as_str(),
                "factory:claim:c-1"
            );
            assert_eq!(
                value.provenance.input_refs[0].revision.as_deref(),
                Some("sha256:claim-c-1-r1")
            );
        }
        other => panic!("expected reading attachment, got {other:?}"),
    }
}
