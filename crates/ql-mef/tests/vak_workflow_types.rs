//! Contract check for the published Vāk workflow types (VW1).
//!
//! The TypeScript module Factory's domain adapter admits is rendered from the
//! QL Rust contract. These tests fail when the checked-in module, its package
//! manifest or the authored C-prime serde surface drift from that contract.
use std::collections::BTreeSet;
use std::path::PathBuf;

use ql_mef::vak_profile::{
    ContentPosition, ContentType, ContextSequence, InquiryDirection, Participation, ThreadForm,
};
use ql_mef::vak_workflow_types::{
    AuthoredCPrime, AuthoredFrame, CPRIME_FIELDS, SourceRevision, Specialist,
    WORKFLOW_TYPES_CONTRACT, WORKFLOW_TYPES_PATH, WORKFLOW_TYPES_SPECIFIER, typescript_module,
};
use ql_mef::{ContextFrameId, vak_profile::PROFILE_CONTRACT};
use serde_json::{Value, json};

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn authored() -> Value {
    json!({
        "CPF": "authorised-undertaking",
        "authority": "central:source:control:root:Control/user/native-action-authority.json",
        "CT": "CT4b'",
        "CP": "4.4",
        "CF": "CF5",
        "CFP": "CFP2",
        "CS": "CS4",
        "direction": "forward",
        "actor": "agent/anima",
        "interpretation": {"ref": "ql/interpretation/c-prime", "revision": "09f7d29ad6262f85bc2858f7c468f22d0bd398f3"},
        "whole": "project:O-I",
        "resolvePath": "aikit:resolve/project:O-I",
        "contextResolution": "aikit:context/project:O-I",
        "sources": ["central:source:project:quaternal-logic:docs/integrations/epi-logos/VAK-EXPRESSION-WORKFLOW-WAYFINDER.md"]
    })
}

#[test]
fn checked_in_module_is_the_rendering_of_the_rust_contract() {
    let path = repository().join(WORKFLOW_TYPES_PATH);
    let checked_in = std::fs::read_to_string(&path).expect("published Vāk workflow types");
    assert_eq!(
        checked_in,
        typescript_module(),
        "{WORKFLOW_TYPES_PATH} drifted; regenerate with `cargo run -p ql-cli -- vak workflow-types > {WORKFLOW_TYPES_PATH}`"
    );
}

#[test]
fn package_manifest_publishes_exactly_the_rendered_types_under_the_registered_specifier() {
    let root = repository().join("adapters/factory-workflow");
    let manifest: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("package.json")).unwrap()).unwrap();
    assert_eq!(manifest["name"], WORKFLOW_TYPES_SPECIFIER);
    assert_eq!(manifest["types"], "./index.d.ts");
    assert_eq!(manifest["exports"]["."]["types"], "./index.d.ts");
    assert_eq!(manifest["qlContract"], WORKFLOW_TYPES_CONTRACT);
    // Types only: no runtime entry that a consumer could mistake for a loader.
    assert!(manifest["exports"]["."].get("import").is_none());
    assert!(manifest.get("main").is_none());
    let files = manifest["files"].as_array().unwrap();
    assert!(files.iter().any(|file| file == "index.d.ts"));
    assert!(!typescript_module().contains("export declare function"));
    assert!(!typescript_module().contains("export const"));
}

#[test]
fn every_rust_value_is_rendered_with_its_serde_spelling() {
    let module = typescript_module();
    let spelled = |value: Value| format!("\"{}\"", value.as_str().unwrap());
    for value in ContentType::ALL {
        assert!(module.contains(&spelled(serde_json::to_value(value).unwrap())));
    }
    for value in ContentPosition::ALL {
        assert!(module.contains(&spelled(serde_json::to_value(value).unwrap())));
    }
    for value in ThreadForm::ALL {
        assert!(module.contains(&spelled(serde_json::to_value(value).unwrap())));
        assert!(module.contains(value.musical_role()));
    }
    for value in ContextSequence::ALL {
        assert!(module.contains(&spelled(serde_json::to_value(value).unwrap())));
    }
    for value in [
        Participation::Dialogical,
        Participation::AuthorisedUndertaking,
    ] {
        assert!(module.contains(&spelled(serde_json::to_value(value).unwrap())));
    }
    for value in [InquiryDirection::Forward, InquiryDirection::Returning] {
        assert!(module.contains(&spelled(serde_json::to_value(value).unwrap())));
    }
    for frame in ContextFrameId::ALL {
        assert!(module.contains(&format!("\"{}\"", frame.code())));
    }
    for specialist in Specialist::ALL {
        assert!(module.contains(specialist.office()));
    }
    assert!(module.contains(&format!("\"{PROFILE_CONTRACT}\"")));
    // QL's wire spelling of the DayNow specialisation is ASCII, not U+2032.
    assert!(module.contains("\"CT4b'\""));
    assert!(!module.contains("CT4b\u{2032}"));
}

#[test]
fn authored_c_prime_fields_are_exactly_the_rendered_fields() {
    let value = authored();
    let parsed: AuthoredCPrime = serde_json::from_value(value.clone()).unwrap();
    parsed.validate().unwrap();
    let round = serde_json::to_value(&parsed).unwrap();
    assert_eq!(round, value);
    let serde_fields = round
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let rendered = CPRIME_FIELDS
        .iter()
        .map(|field| field.to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(serde_fields, rendered);
    let module = typescript_module();
    for field in CPRIME_FIELDS {
        assert!(
            module.contains(&format!("readonly {field}:"))
                || module.contains(&format!("readonly {field}?:")),
            "{field} is not rendered"
        );
    }
    assert_eq!(parsed.frame, AuthoredFrame(ContextFrameId::Cf5));
    assert_eq!(parsed.constitutional_voice(), "Anima");
    assert_eq!(parsed.profile().thread, ThreadForm::Chain);
}

#[test]
fn authored_c_prime_refuses_invalid_frames_authority_and_unknown_fields() {
    let mut bad_frame = authored();
    bad_frame["CF"] = json!("CF8");
    assert!(serde_json::from_value::<AuthoredCPrime>(bad_frame).is_err());

    let mut prime = authored();
    prime["CT"] = json!("CT4b\u{2032}");
    assert!(serde_json::from_value::<AuthoredCPrime>(prime).is_err());

    let mut unknown = authored();
    unknown["harmonicBasis"] = json!("fifths");
    assert!(serde_json::from_value::<AuthoredCPrime>(unknown).is_err());

    let mut missing_authority = authored();
    missing_authority
        .as_object_mut()
        .unwrap()
        .remove("authority");
    let parsed: AuthoredCPrime = serde_json::from_value(missing_authority).unwrap();
    assert!(parsed.validate().is_err());

    let mut dialogical_with_authority = authored();
    dialogical_with_authority["CPF"] = json!("dialogical");
    let parsed: AuthoredCPrime = serde_json::from_value(dialogical_with_authority).unwrap();
    assert!(parsed.validate().is_err());

    let mut no_sources = authored();
    no_sources["sources"] = json!([]);
    let parsed: AuthoredCPrime = serde_json::from_value(no_sources).unwrap();
    assert!(parsed.validate().is_err());

    let mut unqualified = authored();
    unqualified["actor"] = json!("Anima");
    let parsed: AuthoredCPrime = serde_json::from_value(unqualified).unwrap();
    assert!(parsed.validate().is_err());

    let dialogical = AuthoredCPrime {
        participation: Participation::Dialogical,
        authority: None,
        interpretation: SourceRevision {
            reference: "ql/interpretation/c-prime".into(),
            revision: "r1".into(),
        },
        ..serde_json::from_value(authored()).unwrap()
    };
    dialogical.validate().unwrap();
}
