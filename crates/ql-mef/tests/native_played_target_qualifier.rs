#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::performance::PerformanceOwner;
use ql_mef::performance_audio::KeyTouch;
use ql_mef::performance_management::resolve_performance_touch;
use serde_json::{Value, json};

fn sparse_note(
    owner: &PerformanceOwner,
    current: &ql_mef::continuous::coupled::CoupledBasis,
    register: i8,
    token: u64,
) -> Value {
    owner
        .native_note_target(
            current,
            KeyTouch {
                key: 0,
                register,
                member: token,
                touch: token + 100,
                touch_ref: format!("native:played/source-touch/{token}"),
            },
        )
        .unwrap()
}

#[test]
fn existing_native_sparse_target_qualifies_unseeded_register_and_original_touch() {
    let (current, config) = source::config(true);
    let owner = PerformanceOwner::prepare(&current, "native:played/source-owner", config).unwrap();
    for register in [-2, -1, 0, 1] {
        let note = sparse_note(&owner, &current, register, (register + 20) as u64);
        assert_eq!(
            owner.qualify_recorded_note(&current, &note, &note).unwrap(),
            note
        );
        let mut journal = note.clone();
        journal["touch_ref"] = json!("native:another/original-touch");
        assert!(
            owner
                .qualify_recorded_note(&current, &note, &journal)
                .is_err()
        );
    }
}
#[test]
fn played_source_ratio_hertz_prime_phase_and_valid_other_branch_cannot_be_relabelled() {
    let (current, config) = source::config(true);
    let owner = PerformanceOwner::prepare(&current, "native:played/source-owner", config).unwrap();
    let note = sparse_note(&owner, &current, -1, 77);
    for (field, replacement) in [
        ("hertz", json!(note["hertz"].as_f64().unwrap() + 1.0)),
        ("ratio_numerator", json!("19")),
        ("fundamental_hz", json!(1.0)),
        (
            "source_face",
            json!(1 - note["source_face"].as_u64().unwrap()),
        ),
        ("source_coordinate", json!("#1-0")),
        ("coordinate_face", json!(1)),
        ("phase_sin", json!(0.125)),
        ("phase_cos", json!(0.125)),
        ("tuning_ref", json!("native:other/tuning")),
        ("pitch_class", json!(11)),
        ("key", json!(1)),
    ] {
        let mut detached = note.clone();
        detached[field] = replacement;
        assert!(
            owner
                .qualify_recorded_note(&current, &detached, &detached)
                .is_err(),
            "{field}"
        );
    }
    let mut changed = current.input.clone();
    changed.m1.tick12 = (changed.m1.tick12 + 1) % 12;
    let changed = changed.compose().unwrap();
    assert!(owner.qualify_recorded_note(&changed, &note, &note).is_err());
}
#[test]
fn declared_architectural_target_uses_existing_resolver_for_arbitrary_register() {
    let (current, config) = source::config(false);
    let owner = PerformanceOwner::prepare(&current, "native:played/twelve-owner", config).unwrap();
    let note = resolve_performance_touch(
        owner.binding(),
        KeyTouch {
            key: 3,
            register: -2,
            member: 71,
            touch: 801,
            touch_ref: "native:played/twelve-touch".into(),
        },
    )
    .unwrap();
    assert_eq!(
        owner.qualify_recorded_note(&current, &note, &note).unwrap(),
        note
    );
}

#[test]
fn complete_native_note_accepts_equal_f64_bits_with_different_decimal_spelling() {
    let (current, config) = source::config(true);
    let owner = PerformanceOwner::prepare(&current, "native:played/source-owner", config).unwrap();
    let note = sparse_note(&owner, &current, -1, 77);
    let mut wire = note.clone();
    for field in ["hertz", "fundamental_hz", "phase_cos", "phase_sin"] {
        let expected = note[field].as_f64().unwrap();
        wire[field] = serde_json::from_str(&format!("{expected:.17e}")).unwrap();
        assert_eq!(wire[field].as_f64().unwrap().to_bits(), expected.to_bits());
        assert_ne!(wire[field].to_string(), note[field].to_string());
    }
    assert_ne!(wire, note);
    // These are the exact two native JSON spellings observed in hosted run45.
    let left: Value = serde_json::from_str("0.86602540378443915").unwrap();
    let right: Value = serde_json::from_str("0.8660254037844392").unwrap();
    assert_ne!(left, right);
    assert_eq!(left.as_f64().unwrap().to_bits(), 0x3febb67ae8584caf);
    assert_eq!(
        left.as_f64().unwrap().to_bits(),
        right.as_f64().unwrap().to_bits()
    );
    assert_eq!(
        owner.qualify_recorded_note(&current, &wire, &note).unwrap(),
        note
    );
    assert_eq!(
        owner.qualify_recorded_note(&current, &note, &wire).unwrap(),
        note
    );
}

#[test]
fn native_note_next_ulp_missing_wrong_type_or_field_set_never_qualifies() {
    let (current, config) = source::config(true);
    let owner = PerformanceOwner::prepare(&current, "native:played/source-owner", config).unwrap();
    let note = sparse_note(&owner, &current, -1, 78);
    for field in ["hertz", "fundamental_hz", "phase_cos", "phase_sin"] {
        let original = note[field].as_f64().unwrap();
        let mut wrong = note.clone();
        wrong[field] = json!(f64::from_bits(original.to_bits() + 1));
        assert!(
            owner
                .qualify_recorded_note(&current, &wrong, &wrong)
                .is_err(),
            "{field} next ULP"
        );
        assert!(
            owner
                .qualify_recorded_note(&current, &wrong, &note)
                .is_err(),
            "{field} journal mismatch"
        );
        wrong = note.clone();
        wrong.as_object_mut().unwrap().remove(field);
        assert!(
            owner
                .qualify_recorded_note(&current, &wrong, &wrong)
                .is_err(),
            "{field} missing"
        );
        for value in [Value::Null, json!(original.to_string()), json!(true)] {
            wrong = note.clone();
            wrong[field] = value;
            assert!(
                owner
                    .qualify_recorded_note(&current, &wrong, &wrong)
                    .is_err(),
                "{field} wrong type"
            );
        }
    }
    let mut extra = note.clone();
    extra["new_source_authority"] = json!(true);
    assert!(
        owner
            .qualify_recorded_note(&current, &extra, &extra)
            .is_err()
    );
    let mut replaced = note.clone();
    replaced
        .as_object_mut()
        .unwrap()
        .remove("source_coordinate");
    replaced["renamed_source_coordinate"] = note["source_coordinate"].clone();
    assert!(
        owner
            .qualify_recorded_note(&current, &replaced, &replaced)
            .is_err()
    );
}
