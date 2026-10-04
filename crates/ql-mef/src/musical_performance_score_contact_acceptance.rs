//! Acceptance over the actual C current-Scene → recorded Act artifact.
//! This read-only test checks the original linkage, not gravity reconstruction
//! or imported source authority. The closed C/Manager test owns those checks.
use super::{exact, original, qualify_action};
use serde_json::{Value, json};

fn producer(bytes: &[u8]) -> Value {
    let artifact: Value =
        serde_json::from_slice(bytes).expect("complete original C producer bytes");
    assert_eq!(
        artifact["schema"],
        "oi.actual-native-contact-score-delivery/v1"
    );
    artifact
}
fn contacts(performance: &Value) -> Vec<(Value, usize)> {
    let mut result = Vec::new();
    for page in performance["pages"]
        .as_array()
        .expect("actual recorded pages")
    {
        for event in page["events"].as_array().expect("actual performed events") {
            if let Some(contact) = event[4].get("k") {
                let index = usize::try_from(event[3].as_u64().expect("actual musical basis index"))
                    .unwrap();
                result.push((contact.clone(), index));
            }
        }
    }
    assert!(
        !result.is_empty(),
        "the real producer must have performed Contact"
    );
    result
}
fn original_positive(bytes: &[u8]) {
    // Reparse the retained original bytes and qualify EVERY performed Contact
    // after each negative. A previous mutated Value is never the positive.
    let artifact = producer(bytes);
    let performance = &artifact["performance"];
    for (contact, index) in contacts(performance) {
        let basis = &performance["bases"][index];
        qualify_action(&contact, basis, performance).unwrap();
        let admission = original(&contact, basis, performance).unwrap();
        assert_eq!(
            admission["source"]["force_newtons"]
                .as_array()
                .unwrap()
                .len(),
            512
        );
        assert!(exact(
            &admission["source"],
            &admission["native_admission"]["payload"]["contact_source"]
        ));
    }
}
fn changed(value: &Value) -> Value {
    match value {
        Value::Null => json!(false),
        Value::Bool(value) => json!(!value),
        Value::String(value) => json!(format!("{value}:counterproof")),
        Value::Number(number) => {
            if let Some(number) = number.as_f64().filter(|number| number.is_finite()) {
                let next = if number == 0.0 {
                    f64::from_bits(1)
                } else if number > 0.0 {
                    f64::from_bits(number.to_bits() + 1)
                } else {
                    f64::from_bits(number.to_bits() - 1)
                };
                assert!(next.is_finite() && next.to_bits() != number.to_bits());
                json!(next)
            } else {
                json!(null)
            }
        }
        Value::Array(_) | Value::Object(_) => json!(null),
    }
}
fn changed_numeric_kind(value: &Value) -> Option<Value> {
    let Value::Number(number) = value else {
        return None;
    };
    if number.is_f64() {
        let number = number.as_f64()?;
        // Mutate only actual exactly integral binary64 originals. The value
        // remains numerically equal while the saved representation is lost.
        if number.is_finite() && number.fract() == 0.0 && (-1.0e12..=1.0e12).contains(&number) {
            Some(json!(number as i64))
        } else {
            None
        }
    } else {
        let number = number.as_f64()?;
        if number.is_finite() && (-1.0e12..=1.0e12).contains(&number) {
            Some(json!(number))
        } else {
            None
        }
    }
}
fn leaves(value: &Value, prefix: &str, result: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                let escaped = key.replace('~', "~0").replace('/', "~1");
                leaves(value, &format!("{prefix}/{escaped}"), result);
            }
        }
        Value::Array(array) => {
            for (index, value) in array.iter().enumerate() {
                leaves(value, &format!("{prefix}/{index}"), result);
            }
        }
        _ => result.push(prefix.to_owned()),
    }
}
fn mutate_admissions(performance: &mut Value, contact: &Value, mut edit: impl FnMut(&mut Value)) {
    let mut count = 0;
    for asset in performance["native_sources"].as_array_mut().unwrap() {
        let Some(rows) = asset.get_mut("native_contact_admission_history") else {
            continue;
        };
        for row in rows.as_array_mut().unwrap() {
            if exact(&row["source"]["occurrence"], &contact["occurrence"]) {
                edit(row);
                count += 1;
            }
        }
    }
    assert!(count > 0, "actual same-occurrence source rows absent");
}
fn refused(bytes: &[u8], contact: &Value, basis: &Value, performance: &Value, label: &str) {
    assert!(
        qualify_action(contact, basis, performance).is_err(),
        "{label}"
    );
    original_positive(bytes);
}

#[test]
#[ignore = "requires the genuine current C Kernel/Act/Manager Contact producer"]
fn actual_contact_score_all_512_force_cells_and_original_owned_branches() {
    let path = std::env::var_os("QL_NATIVE_CONTACT_SCORE_ARTIFACT")
        .expect("normal C producer artifact; no fixture or reconstructed owner fallback");
    let bytes = std::fs::read(path).unwrap();
    assert!(!bytes.is_empty() && bytes.len() <= 32 * 1024 * 1024);
    original_positive(&bytes);
    let artifact = producer(&bytes);
    let performance = &artifact["performance"];
    for (contact, basis_index) in contacts(performance) {
        let basis = &performance["bases"][basis_index];
        let admission = original(&contact, basis, performance).unwrap();
        assert_eq!(contact["operands"].as_object().unwrap().len(), 36);
        let mut force_checks = 0;
        for index in 0..512 {
            let mut mutated = performance.clone();
            mutate_admissions(&mut mutated, &contact, |row| {
                // Leave the full native pulse unchanged: the native original
                // is the counterproof for every actual retained force cell.
                let cell = &mut row["source"]["force_newtons"][index];
                *cell = changed(cell);
            });
            refused(
                &bytes,
                &contact,
                basis,
                &mutated,
                &format!("force cell {index}"),
            );
            if let Some(wrong_kind) =
                changed_numeric_kind(&admission["source"]["force_newtons"][index])
            {
                let mut mutated = performance.clone();
                mutate_admissions(&mut mutated, &contact, |row| {
                    row["source"]["force_newtons"][index] = wrong_kind.clone();
                });
                refused(
                    &bytes,
                    &contact,
                    basis,
                    &mutated,
                    &format!("force cell {index} numeric kind"),
                );
            }
            force_checks += 1;
        }
        assert_eq!(force_checks, 512);
        let mut zero_sign_checks = 0;
        for (name, value) in contact["operands"].as_object().unwrap() {
            let mut absent = contact.clone();
            absent["operands"].as_object_mut().unwrap().remove(name);
            refused(
                &bytes,
                &absent,
                basis,
                performance,
                &format!("lost operand {name}"),
            );
            for (label, wrong) in [("type", json!(null)), ("value", changed(value))] {
                let mut mutated = contact.clone();
                mutated["operands"][name] = wrong;
                refused(
                    &bytes,
                    &mutated,
                    basis,
                    performance,
                    &format!("operand {name} {label}"),
                );
            }
            if let Some(wrong_kind) = changed_numeric_kind(value) {
                let mut mutated = contact.clone();
                mutated["operands"][name] = wrong_kind;
                refused(
                    &bytes,
                    &mutated,
                    basis,
                    performance,
                    &format!("operand {name} numeric kind"),
                );
            }
            if value.as_f64() == Some(0.0) {
                let mut mutated = contact.clone();
                mutated["operands"][name] = json!(-value.as_f64().unwrap());
                refused(
                    &bytes,
                    &mutated,
                    basis,
                    performance,
                    &format!("native zero sign {name}"),
                );
                zero_sign_checks += 1;
            }
        }
        assert!(zero_sign_checks > 0, "actual native zero operands required");
        for pointer in [
            "/handle/generation",
            "/handle/slot",
            "/occurrence/constructor_lineage",
            "/occurrence/original_request_id",
        ] {
            let mut mutated = contact.clone();
            let value = mutated.pointer_mut(pointer).unwrap();
            *value = changed(value);
            refused(&bytes, &mutated, basis, performance, pointer);
        }
        for (pointer, wrong) in [
            ("/handle/generation", json!("0")),
            ("/handle/slot", json!("16")),
            ("/occurrence/original_request_id", json!("0")),
            ("/occurrence/original_request_id", json!("01")),
        ] {
            let mut mutated = contact.clone();
            *mutated.pointer_mut(pointer).unwrap() = wrong;
            refused(&bytes, &mutated, basis, performance, pointer);
        }
        for key in ["handle", "occurrence", "operands"] {
            let mut absent = contact.clone();
            absent.as_object_mut().unwrap().remove(key);
            refused(&bytes, &absent, basis, performance, key);
        }
        let mut extra = contact.clone();
        extra["unowned_contact_permission"] = json!(true);
        refused(&bytes, &extra, basis, performance, "extra Contact field");

        // Cover every original source leaf: authored geometry, native Scene
        // constructor/CAS, occurrence/body/epoch/clock/context and 512-force
        // programme. The original native pulse remains independently intact.
        let mut paths = Vec::new();
        leaves(&admission["source"], "/source", &mut paths);
        for pointer in paths {
            if pointer.starts_with("/source/force_newtons/") {
                continue;
            }
            let mut mutated = performance.clone();
            mutate_admissions(&mut mutated, &contact, |row| {
                let cell = row.pointer_mut(&pointer).unwrap();
                *cell = changed(cell);
            });
            refused(&bytes, &contact, basis, &mutated, &pointer);
        }
        for field in admission["source"].as_object().unwrap().keys() {
            let mut mutated = performance.clone();
            mutate_admissions(&mut mutated, &contact, |row| {
                row["source"].as_object_mut().unwrap().remove(field);
            });
            refused(
                &bytes,
                &contact,
                basis,
                &mutated,
                &format!("lost source {field}"),
            );
        }
        for field in admission.as_object().unwrap().keys() {
            let mut mutated = performance.clone();
            mutate_admissions(&mut mutated, &contact, |row| {
                row.as_object_mut().unwrap().remove(field);
            });
            refused(
                &bytes,
                &contact,
                basis,
                &mutated,
                &format!("lost admission {field}"),
            );
        }
        for pointer in [
            "/schema",
            "/native_admission/schema",
            "/native_admission/accepted",
            "/native_admission/payload/queue_committed",
            "/native_admission/payload/application_committed",
            "/native_admission/payload/score_admission/schema",
            "/native_admission/payload/score_admission/queued",
            "/native_admission/payload/score_admission/event/kind",
            "/native_admission/payload/score_admission/event/sample",
            "/native_admission/payload/score_admission/event/requested_sample",
            "/native_admission/payload/score_admission/event/contact/generation",
            "/native_admission/payload/score_admission/event/contact/slot",
        ] {
            let mut mutated = performance.clone();
            mutate_admissions(&mut mutated, &contact, |row| {
                let cell = row.pointer_mut(pointer).unwrap();
                *cell = changed(cell);
            });
            refused(&bytes, &contact, basis, &mutated, pointer);
        }
        for pointer in [
            "/content_digest",
            "/prepared_body/request/preparation_ref",
            "/prepared_body/request/state_ref",
            "/audio_determination/body_revision",
            "/identity/m3_generation",
            "/prepared_body/source_coordinate/source_ref",
            "/prepared_body/source_revision",
        ] {
            let mut mutated = basis.clone();
            let cell = mutated.pointer_mut(pointer).unwrap();
            *cell = changed(cell);
            refused(&bytes, &contact, &mutated, performance, pointer);
        }
        let mut detached = performance.clone();
        for asset in detached["native_sources"].as_array_mut().unwrap() {
            asset
                .as_object_mut()
                .unwrap()
                .remove("native_contact_admission_history");
        }
        refused(
            &bytes,
            &contact,
            basis,
            &detached,
            "lost whole original sidecar",
        );
    }
    original_positive(&bytes);
}
