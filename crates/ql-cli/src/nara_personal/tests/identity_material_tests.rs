// Additional native identity tests in the existing private test module.

fn supplied_identity(root: &std::path::Path) -> PersonalRecord {
    let record = create(root);
    let mut slot = record.domain.identity.slots[0].clone();
    slot.source = Some(source("identity"));
    slot.protected_value_ref = Some(protected("identity"));
    slot.absence_reason = None;
    slot.standing = EvidenceStanding::Reported;
    apply(
        root,
        &record,
        "identity:source",
        Mutation::IdentityReplace {
            slot,
            identity_revision: "accepted:r1".into(),
        },
    )
}
fn identity_read(root: &std::path::Path, r: &PersonalRecord) -> Value {
    execute(
        root,
        Request::IdentityMaterial {
            target: r.target.clone(),
            consent: consent(),
            expected_revision: r.revision,
        },
    )
    .unwrap()
}
#[test]
fn identity_material_preview_is_native_private_and_has_no_mutation() {
    let t = Temp::new();
    let r = supplied_identity(&t.0);
    let before = std::fs::read_dir(&t.0).unwrap().count();
    let a = identity_read(&t.0, &r);
    let b = identity_read(&t.0, &r);
    assert_eq!(a, b);
    assert_eq!(a["material"]["schema"], "ql.nara-identity-material/v1");
    assert_eq!(a["material"]["sealed"], false);
    assert_eq!(a["material"]["private"], true);
    assert_eq!(a["material"]["public_export"], false);
    assert_eq!(a["material"]["value"].as_str().unwrap().len(), 64);
    assert_eq!(a["material"]["supplied_offices"], json!(["birthdate-name"]));
    assert_eq!(a["source_mutated"], false);
    assert_eq!(
        a["material"]["q_composed"],
        serde_json::to_value(r.domain.q_composed).unwrap()
    );
    let reopened: PersonalRecord = serde_json::from_value(
        execute(
            &t.0,
            Request::Read {
                target: r.target.clone(),
                consent: consent(),
            },
        )
        .unwrap()["record"]
            .clone(),
    )
    .unwrap();
    assert_eq!(reopened.revision, r.revision);
    assert_eq!(reopened.receipts.len(), r.receipts.len());
    assert_eq!(std::fs::read_dir(&t.0).unwrap().count(), before);
    assert!(a["material"].get("slots").is_none());
}
#[test]
fn identity_material_requires_consent_exact_target_and_current_revision() {
    let t = Temp::new();
    let r = supplied_identity(&t.0);
    let mut c = consent();
    c.personal_data = false;
    assert!(execute(
        &t.0,
        Request::IdentityMaterial {
            target: r.target.clone(),
            consent: c,
            expected_revision: r.revision
        }
    )
    .is_err());
    let mut other = r.target.clone();
    other.nara_ref = "nara:other".into();
    assert!(execute(
        &t.0,
        Request::IdentityMaterial {
            target: other,
            consent: consent(),
            expected_revision: r.revision
        }
    )
    .is_err());
    assert!(execute(
        &t.0,
        Request::IdentityMaterial {
            target: r.target.clone(),
            consent: consent(),
            expected_revision: r.revision - 1
        }
    )
    .is_err());
    assert!(serde_json::from_value::<Request>(json!({"operation":"identity_material","target":r.target,"consent":consent(),"expected_revision":r.revision,"raw_transcript":"private"})).is_err());
}
#[test]
fn empty_identity_stays_unprovided_instead_of_becoming_a_demo_hash() {
    let t = Temp::new();
    let r = create(&t.0);
    assert!(identity_material::material(&r)
        .unwrap_err()
        .contains("no identity source"));
    assert!(execute(
        &t.0,
        request(
            &r,
            "empty:seal",
            Mutation::IdentitySeal {
                expected_value: "0".repeat(64)
            }
        )
    )
    .is_err());
}
#[test]
fn identity_material_is_stable_across_order_receipts_revision_labels_and_current_orientation() {
    let t = Temp::new();
    let r = supplied_identity(&t.0);
    let value = identity_material::material(&r).unwrap().value;
    let mut changed = r.clone();
    changed.domain.identity.slots.reverse();
    changed.domain.identity.identity_revision = "same-basis-new-review".into();
    changed.domain.q_composed = BioQuaternion {
        w: 0.0,
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    changed.domain.identity.m3_form_address_ref = Some("m3:separate-form".into());
    changed.domain.identity.identity_quaternion_ref = Some(protected("separate-quaternion"));
    assert_eq!(identity_material::material(&changed).unwrap().value, value);
    let newer = apply(
        &t.0,
        &r,
        "new:feedback",
        Mutation::CentreFeedback {
            ordinal: 2,
            feedback_ref: "source:body-feedback".into(),
        },
    );
    assert_eq!(identity_material::material(&newer).unwrap().value, value);
    let mut changed = r.clone();
    changed.domain.identity.slots[0]
        .source
        .as_mut()
        .unwrap()
        .revision = "r2".into();
    assert_ne!(identity_material::material(&changed).unwrap().value, value);
    let mut changed = r;
    changed.domain.identity.slots[0]
        .tensions
        .push("source:tension".into());
    assert_ne!(identity_material::material(&changed).unwrap().value, value);
}
#[test]
fn sealing_records_the_native_reference_and_identity_replacement_invalidates_it() {
    let t = Temp::new();
    let r = supplied_identity(&t.0);
    let m = identity_material::material(&r).unwrap();
    let request = request(
        &r,
        "seal:once",
        Mutation::IdentitySeal {
            expected_value: m.value.clone(),
        },
    );
    let reply = execute(&t.0, request.clone()).unwrap();
    let r: PersonalRecord = serde_json::from_value(reply["record"].clone()).unwrap();
    assert_eq!(
        r.domain.identity.identity_hash_ref,
        Some(m.identity_hash_ref)
    );
    assert!(identity_material::material(&r).unwrap().sealed);
    assert_eq!(execute(&t.0, request).unwrap()["duplicate"], true);
    let mut slot = r.domain.identity.slots[0].clone();
    slot.source.as_mut().unwrap().revision = "r2".into();
    let r = apply(
        &t.0,
        &r,
        "identity:next",
        Mutation::IdentityReplace {
            slot,
            identity_revision: "accepted:r2".into(),
        },
    );
    assert!(r.domain.identity.identity_hash_ref.is_none());
    assert!(r.domain.identity.identity_quaternion_ref.is_none());
    assert!(r.domain.identity.m3_form_address_ref.is_none());
    assert!(!identity_material::material(&r).unwrap().sealed);
    assert!(execute(
        &t.0,
        self::request(
            &r,
            "stale:seal",
            Mutation::IdentitySeal {
                expected_value: m.value
            }
        )
    )
    .is_err());
}
