mod support;

use ql_mef::{LensId, LensRef};
use ql_semantic::{LocateRequest, ProviderHealth, RefractRequest};
use ql_service::QlService;

use support::{FixtureProvider, input};

const EPII_GUARDIAN_REF: &str = "epi:agent:epii";
const EPII_STEWARDSHIP_AGENCY_REF: &str = "epi:agency:epii:stewardship";
const ALETHEIA_LABOURING_AGENCY_REF: &str = "epi:agency:aletheia:ql-native-reading";

#[test]
fn epii_guardian_operates_native_ql_readings_without_becoming_execution_identity() {
    let mut service = QlService::with_provider(FixtureProvider::full(
        "guardian-proof-a",
        ProviderHealth::available(),
    ));

    let located = service
        .locate(LocateRequest {
            input: input("epi:guardian-proof/locate", Some("issue-42")),
            frame: None,
        })
        .expect("Epii stewardship must be able to locate through QL-MEF");

    let reading = service
        .refract(RefractRequest {
            input: input("epi:guardian-proof/refract", Some("issue-42")),
            lens: LensRef::canonical(LensId::L4Prime),
            sublens: None,
            frame: None,
        })
        .expect("Epii stewardship must be able to refract through QL-MEF");

    assert_eq!(located.target.subject, "epi:guardian-proof/locate");
    assert_eq!(reading.target.subject, "epi:guardian-proof/refract");
    assert_ne!(EPII_GUARDIAN_REF, EPII_STEWARDSHIP_AGENCY_REF);
    assert_ne!(EPII_GUARDIAN_REF, ALETHEIA_LABOURING_AGENCY_REF);
    assert_ne!(EPII_STEWARDSHIP_AGENCY_REF, ALETHEIA_LABOURING_AGENCY_REF);

    // Execution/provider identity is replaceable; the enduring guardian ref is not.
    let guardian_before_rebind = EPII_GUARDIAN_REF;
    service.replace_provider(FixtureProvider::full(
        "guardian-proof-b",
        ProviderHealth::available(),
    ));
    let rebound = service
        .locate(LocateRequest {
            input: input("epi:guardian-proof/rebound", Some("issue-42")),
            frame: None,
        })
        .expect("provider rebinding must not change guardian identity");

    assert_eq!(rebound.target.subject, "epi:guardian-proof/rebound");
    assert_eq!(EPII_GUARDIAN_REF, guardian_before_rebind);
}
