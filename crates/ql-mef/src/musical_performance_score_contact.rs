//! Read-only Contact score qualification. A retained handle is evidence of its
//! original queued occurrence; this consumer cannot prepare or replay a slot.
use serde_json::Value;

pub(super) fn exact(left: &Value, right: &Value) -> bool {
    crate::continuous::performance::retained_evidence::exact_value(left, right)
}

fn keys(value: &Value, expected: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or("original Contact object absent")?;
    if object.len() != expected.len() || expected.iter().any(|key| !object.contains_key(*key)) {
        return Err("original Contact field set differs".into());
    }
    Ok(())
}
fn count(value: &Value) -> Result<u64, String> {
    super::counter(value)
}

/// Resolve the literal C59 application to the complete original native source
/// and queued pulse. The outer compiler first qualifies every source asset
/// against its borrowed actual owner; this function grants no source authority.
pub(super) fn original<'a>(
    contact: &Value,
    basis: &Value,
    performance: &'a Value,
) -> Result<&'a Value, String> {
    if performance["schema"] != "oi.expression-performance/v3" {
        return Err("Contact score needs the full native v3 source delivery".into());
    }
    keys(contact, &["handle", "occurrence", "operands"])?;
    keys(&contact["handle"], &["generation", "slot"])?;
    keys(
        &contact["occurrence"],
        &["constructor_lineage", "original_request_id"],
    )?;
    if count(&contact["handle"]["generation"])? == 0
        || count(&contact["handle"]["slot"])? >= 16
        || count(&contact["occurrence"]["original_request_id"])? == 0
        || contact["occurrence"]["constructor_lineage"]
            .as_str()
            .is_none_or(str::is_empty)
    {
        return Err("original Contact handle/occurrence differs".into());
    }
    let mut selected = None;
    for asset in super::array(&performance["native_sources"])? {
        if asset["basis_digest"] != basis["content_digest"] {
            continue;
        }
        let Some(history) = asset.get("native_contact_admission_history") else {
            continue;
        };
        for admission in super::array(history)? {
            let source = &admission["source"];
            if !exact(&source["occurrence"], &contact["occurrence"]) {
                continue;
            }
            keys(
                admission,
                &[
                    "schema",
                    "before_source_assets",
                    "source",
                    "native_admission",
                ],
            )?;
            keys(
                source,
                &[
                    "schema",
                    "original_native_request_id",
                    "scene_constructor",
                    "authored_definition",
                    "native_boundary",
                    "occurrence",
                    "original_gravity_input",
                    "native_operands",
                    "original_body",
                    "force_newtons",
                    "exciter_position_metres",
                ],
            )?;
            let queue = &admission["native_admission"]["payload"]["score_admission"];
            let operands = &source["native_operands"];
            // The exact complete original 36-key native operands, including
            // every finite binary64 bit, come from the same retained source.
            if operands.as_object().is_none_or(|object| object.len() != 36)
                || admission["schema"] != "ql.native-scene-contact-admission/v1"
                || source["schema"] != "ql.native-scene-contact-source/v1"
                || !admission["before_source_assets"].is_object()
                || admission["native_admission"]["schema"] != "ql.performance-worker-reply/v1"
                || admission["native_admission"]["accepted"] != true
                || admission["native_admission"]["payload"]["queue_committed"] != true
                || admission["native_admission"]["payload"]["application_committed"] != false
                || !exact(
                    &admission["native_admission"]["payload"]["contact_source"],
                    source,
                )
                || queue["schema"] != "ql.native-score-admission/v1"
                || queue["queued"] != true
                || queue["event"]["kind"] != 7
                || !exact(&queue["event"]["contact"], &contact["handle"])
                || !exact(operands, &contact["operands"])
                || source["original_native_request_id"]
                    != contact["occurrence"]["original_request_id"]
                || source["scene_constructor"]["instance_ref"]
                    != contact["occurrence"]["constructor_lineage"]
                || operands["preparation_ref"]
                    != basis["prepared_body"]["request"]["preparation_ref"]
                || operands["state_ref"] != basis["prepared_body"]["request"]["state_ref"]
                || operands["body_revision"] != basis["audio_determination"]["body_revision"]
                || operands["source_generation"] != basis["identity"]["m3_generation"]
                || operands["source_coordinate"]
                    != basis["prepared_body"]["source_coordinate"]["source_ref"]
                || operands["source_revision"] != basis["prepared_body"]["source_revision"]
                || operands["pratibimba"] != true
                || super::array(&source["force_newtons"])?.len() != 512
                || queue["event"]["sample"] != operands["impact_sample"]
                || queue["event"]["requested_sample"] != operands["impact_sample"]
            {
                return Err(
                    "Contact score detached from its full original body/programme/admission".into(),
                );
            }
            if selected.is_some_and(|old| !exact(old, admission)) {
                return Err("Contact occurrence has conflicting original source epochs".into());
            }
            selected = Some(admission);
        }
    }
    selected.ok_or_else(|| "Contact score lost its original native occurrence producer".into())
}

pub(crate) fn qualify_action(
    contact: &Value,
    basis: &Value,
    performance: &Value,
) -> Result<(), String> {
    original(contact, basis, performance).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires the genuine current C Scene/Act Contact recording producer"]
    fn actual_contact_score_preserves_all_original_operands_and_source_programme() {
        let path = std::env::var_os("QL_NATIVE_CONTACT_SCORE_ARTIFACT")
            .expect("genuine native owner artifact; no fallback");
        let bytes = std::fs::read(path).expect("original current producer output");
        assert!(!bytes.is_empty() && bytes.len() <= 32 * 1024 * 1024);
        let artifact: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            artifact["schema"],
            "oi.actual-native-contact-score-delivery/v1"
        );
        let performance = &artifact["performance"];
        let mut checked = 0;
        for page in super::super::array(&performance["pages"]).unwrap() {
            for event in super::super::array(&page["events"]).unwrap() {
                let Some(contact) = event[4].get("k") else {
                    continue;
                };
                let basis_index = event[3].as_u64().unwrap() as usize;
                let basis = &performance["bases"][basis_index];
                let original = original(contact, basis, performance).unwrap();
                // Mutate only the real producer's complete operands. The
                // original complete source/pulse remains the counterproof.
                for (pointer, wrong) in [
                    ("/operands/pratibimba", Value::Bool(false)),
                    (
                        "/operands/eigenbasis",
                        Value::String("native:wrong-eigenbasis".into()),
                    ),
                    (
                        "/operands/preparation_ref",
                        Value::String("native:stale-body".into()),
                    ),
                    (
                        "/occurrence/constructor_lineage",
                        Value::String("native:detached-producer".into()),
                    ),
                ] {
                    let mut changed = contact.clone();
                    *changed.pointer_mut(pointer).unwrap() = wrong;
                    assert!(
                        qualify_action(&changed, basis, performance).is_err(),
                        "{pointer}"
                    );
                    qualify_action(contact, basis, performance).unwrap();
                }
                for name in contact["operands"].as_object().unwrap().keys() {
                    let mut changed = contact.clone();
                    changed["operands"].as_object_mut().unwrap().remove(name);
                    assert!(
                        qualify_action(&changed, basis, performance).is_err(),
                        "lost {name}"
                    );
                    qualify_action(contact, basis, performance).unwrap();
                }
                let mut disconnected = performance.clone();
                for asset in disconnected["native_sources"].as_array_mut().unwrap() {
                    asset
                        .as_object_mut()
                        .unwrap()
                        .remove("native_contact_admission_history");
                }
                assert!(qualify_action(contact, basis, &disconnected).is_err());
                qualify_action(contact, basis, performance).unwrap();
                assert_eq!(
                    original["source"]["force_newtons"]
                        .as_array()
                        .unwrap()
                        .len(),
                    512
                );
                let mut zero_checks = 0;
                for (name, number) in contact["operands"].as_object().unwrap() {
                    if number.as_f64() != Some(0.0) {
                        continue;
                    }
                    let mut changed = contact.clone();
                    changed["operands"][name] = serde_json::json!(-number.as_f64().unwrap());
                    assert!(
                        qualify_action(&changed, basis, performance).is_err(),
                        "lossy {name}"
                    );
                    qualify_action(contact, basis, performance).unwrap();
                    zero_checks += 1;
                }
                // The original Contact programme supplies real zero-valued
                // scalar operands; no synthetic positive is introduced.
                assert!(zero_checks > 0);
                checked += 1;
            }
        }
        assert!(checked > 0, "genuine performed Contact score absent");
    }
}

#[cfg(test)]
#[path = "musical_performance_score_contact_acceptance.rs"]
mod original_acceptance;
