//! Independent actual native source producer regression. All personal inputs
//! here are declared controlled fixtures, with no human identity or consent.
//! A valid World constructor must not turn retained native private source into
//! public ownership merely because that source is nested in a valid sky.
use ql_mef::nara::{current, intake::IdentityProfile};
use ql_mef::performance_source_context::NativePublicSourceOwnership;
use ql_mef::scene::{WORLD_REQUEST, WorldRequest, world};
use serde_json::{Value, json};

fn source_sky() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap()
}

fn request(sky: Value) -> WorldRequest {
    let event = sky["snapshot_ref"].as_str().unwrap().to_owned();
    serde_json::from_value(json!({
        "schema": WORLD_REQUEST,
        "instance_ref": "expression:controlled-independent-source-context",
        "event_ref": event,
        "subject_ref": "person:controlled-independent-source-context",
        "texture": [64, 64],
        "units_per_metre": 1.0,
        "sky": sky,
        "start": {"tick12": 3, "cycle": 7, "aperture": 9}
    }))
    .unwrap()
}

fn genuine_native_private_reading(sky: &Value) -> Value {
    let profile: IdentityProfile = serde_json::from_value(json!({
        "schema": "ql.nara-identity-profile/v1",
        "person_ref": "person:controlled-independent-source-context",
        "nara_ref": "controlled:independent-source-context/nara",
        "name": "Controlled independent source operator",
        "encoding_policy": ql_mef::nara::identity_encoding::EncodingPolicy::default(),
        "composition_policy": "draft-core-birthdate-decanic-40-60-v1",
        "birth": {"date": "1990-06-15", "time": null, "precision": "unknown",
            "uncertainty_minutes": null, "fold": null, "place": null},
        "jungian": null, "gene_keys": null, "human_design": null,
        "quintessence": null
    }))
    .unwrap();
    let natal = json!({
        "schema": "ql.nara-natal/v1",
        "request": profile.natal_request().unwrap(),
        "status": "partial",
        "reason": "controlled native numerical input; no astronomical qualification",
        "chart": null,
        "sky": sky
    });
    let identity = profile.inspect(Some(&natal)).unwrap();
    let transit = current::transit(Some(sky)).unwrap();
    let private = current::personal_current(&identity, &transit).unwrap();
    assert_eq!(private["schema"], "ql.nara-personal-current/v1");
    assert_eq!(private["private"], true);
    assert_eq!(private["public_export"], false);
    // These are the actual native producer's values, not injected flags.
    assert_eq!(private["identity"], identity);
    assert_eq!(private["transit"], transit);
    private
}

#[test]
fn genuine_public_world_still_has_exact_native_source_ownership() {
    let sky = source_sky();
    let produced = world(request(sky.clone())).unwrap();
    assert_eq!(produced["sky"], sky);
    assert_eq!(produced["event"]["source_receipts"], json!([sky.clone()]));
    let ownership = NativePublicSourceOwnership::world_source(request(sky)).unwrap();
    assert_eq!(
        serde_json::to_value(ownership).unwrap()["original_native_input"],
        produced["event"]
    );
}

#[test]
fn full_original_native_private_reading_cannot_become_public_through_world_sky() {
    let mut sky = source_sky();
    let private = genuine_native_private_reading(&sky);
    sky["original_personal_source"] = private.clone();
    // Required sky bodies, provider receipt, epoch and native registry remain
    // valid. Ordinary source derivation retains the complete added original.
    current::transit(Some(&sky)).unwrap();
    let produced = world(request(sky.clone())).unwrap();
    assert_eq!(
        produced["event"]["source_receipts"][0]["original_personal_source"],
        private
    );
    assert_eq!(produced["sky"], sky);
    assert!(
        NativePublicSourceOwnership::world_source(request(sky)).is_err(),
        "native private original cannot receive public ownership from the World constructor"
    );
}
