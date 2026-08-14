use ql_core::{QlAddress, QlFormRef};
use ql_mef::LensRef;

#[test]
fn factory_v1_fixture_preserves_client_ref_and_uses_canonical_ql_mef_lens() {
    let fixture = include_str!("../../../fixtures/q4/factory-interop-v1.json");

    assert!(fixture.contains("\"sourceContractVersion\": \"factory.interop/v1\""));
    assert!(fixture.contains("\"sourceRef\": \"factory:claim:c-1\""));
    assert!(fixture.contains("\"sourceRevision\": \"sha256:claim-c-1-r1\""));
    assert!(fixture.contains("\"lensRef\": \"mef:lens:L3@1\""));
    assert!(fixture.contains("\"targetRef\": \"factory:claim:c-1\""));
    assert!(fixture.contains("\"status\": \"factory-113-contract-consumed\""));
}

#[test]
fn legacy_factory_ql_strings_are_rejected_instead_of_translated() {
    assert!(
        "qlform:factory-development/v1"
            .parse::<QlFormRef>()
            .is_err()
    );
    assert!("day:2.3".parse::<QlAddress>().is_err());
    assert!("lens:L3".parse::<LensRef>().is_err());

    let fixture = include_str!("../../../fixtures/q4/factory-interop-v1.json");
    assert!(fixture.contains("\"rejectedLegacyRefs\""));
    assert!(fixture.contains("\"qltarget:claim-whole\""));
}

#[test]
fn adapter_schema_keeps_client_refs_opaque_and_ql_refs_canonical() {
    let schema = include_str!("../../../schemas/q4/adapter.schema.json");

    for definition in [
        "ClientRef",
        "QlMode",
        "LensRef",
        "SublensRef",
        "ClientSubject",
        "QlAttachment",
        "RefractionRequest",
    ] {
        assert!(
            schema.contains(&format!("\"{definition}\"")),
            "missing schema definition: {definition}"
        );
    }
    for mode in ["disabled", "optional", "required"] {
        assert!(schema.contains(&format!("\"{mode}\"")));
    }
    assert!(schema.contains("^mef:lens:L[0-5]'?@1$"));
    assert!(!schema.contains("factory:project"));
    assert!(!schema.contains("aikit:context"));
}
