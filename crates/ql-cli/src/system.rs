//! Wave 5 System contribution: the `ql system` settings disclosure.
//!
//! Emits the O:I System settings descriptor (`oi.product-settings-disclosure/v2`,
//! `wave-5/system.1`) for `quaternal-logic`. Bounded to the currently accepted
//! native QL Kernel availability, readings, Actions and readiness. QL remains
//! optional for the System surface; nothing here anticipates or encodes the
//! QL-MEF #123 Vāk / C-prime / Context-Frame conclusions.
//!
//! Read-only: this command performs no mutation and writes nothing.

use ql_service::QlService;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SCHEMA: &str = "oi.product-settings-disclosure/v2";
const PRODUCT_ID: &str = "quaternal-logic";
const CONTRACT_REVISION: &str = "wave-5/system.1";
const OWNER_REF: &str = "ql.cli/v1";

const ABOUT: &str = "Deterministic QL kernel: quaternal positions, forms and the three accepted \
deterministic operators (conjugate-address, complement-address, classify-four-plus-two), native \
verification, and provider-backed service availability. Optional product; the System surface must \
remain functional without it.";

pub fn system_command(json_out: bool) -> Result<String, crate::CliError> {
    let descriptor = build_descriptor()?;
    if json_out {
        return serde_json::to_string_pretty(&descriptor).map_err(crate::CliError::from);
    }

    let availability = descriptor["availability"]["state"]
        .as_str()
        .unwrap_or("unknown");
    let kernel = &descriptor["sections"][0]["settings"];
    let kernel_version = kernel[0]["axes"]["active"]["value"].as_str().unwrap_or("?");
    let forms = kernel[2]["axes"]["active"]["value"]
        .as_array()
        .map_or(0, Vec::len);
    let operators = kernel[3]["axes"]["active"]["value"]
        .as_array()
        .map_or(0, Vec::len);
    let readiness = descriptor["sections"][1]["settings"][0]["axes"]["active"]["value"]
        .as_str()
        .unwrap_or("unknown");
    let provider = descriptor["sections"][2]["settings"][0]["axes"]["active"]["value"]
        .as_str()
        .unwrap_or("unknown");

    Ok(format!(
        "quaternal-logic System disclosure ({SCHEMA}):\n\
         kernel {kernel_version} — {forms} forms, {operators} deterministic operators\n\
         readiness: {readiness}\n\
         service provider: {provider}\n\
         availability: {availability}"
    ))
}

fn build_descriptor() -> Result<Value, crate::CliError> {
    let observed = now_ms();

    let kernel = crate::kernel_view();
    let kernel_version = kernel.kernel_version;
    let schema_version = kernel.schema_version;
    let forms = kernel.supported_forms;
    let deterministic_operations = kernel.deterministic_operations;
    let stochastic_operations = kernel.stochastic_operations;
    let research_operations = kernel.research_operations;

    let service = QlService::new();
    let service_view = crate::service_view(&service);
    let provider_state = service_view.provider_state;
    let service_operations: Vec<Value> = service_view
        .operations
        .iter()
        .map(|operation| {
            json!({
                "operation": operation.operation,
                "supported": operation.supported,
                "deterministic": operation.deterministic,
            })
        })
        .collect();

    let (readiness_status, check_count, verify_error) = readiness();

    let availability_state = if verify_error.is_some() {
        "degraded"
    } else {
        "available"
    };
    let availability = json!({
        "state": availability_state,
        "reason": verify_error,
    });

    let degradations = verify_error
        .map(|error| {
            json!([{
                "subject_ref": "quaternal-logic.verify",
                "state": "unavailable",
                "reason": "native verification failed",
                "native_error": error,
            }])
        })
        .unwrap_or_else(|| json!([]));

    let sections = vec![
        json!({
            "id": "kernel",
            "title": "QL Kernel",
            "settings": [
                setting("kernel.version", "Kernel version", "scalar", json!(kernel_version), "ql kernel capabilities", observed),
                setting("kernel.schema-version", "Kernel schema version", "scalar", json!(schema_version), "ql kernel capabilities", observed),
                setting("kernel.supported-forms", "Supported forms", "table", json!(forms), "ql kernel capabilities", observed),
                setting("kernel.deterministic-operators", "Deterministic operators", "table", json!(deterministic_operations), "ql kernel capabilities", observed),
                setting("kernel.stochastic-operators", "Stochastic operators", "table", json!(stochastic_operations), "ql kernel capabilities", observed),
                setting("kernel.research-operators", "Research operators", "table", json!(research_operations), "ql kernel capabilities", observed),
            ],
        }),
        json!({
            "id": "readiness",
            "title": "Native verification",
            "settings": [
                setting("readiness.status", "Verification status", "scalar", json!(readiness_status), "ql verify", observed),
                setting("readiness.check-count", "Verification checks", "scalar", json!(check_count), "ql verify", observed),
            ],
        }),
        json!({
            "id": "service",
            "title": "Provider-backed service",
            "settings": [
                setting("service.provider-state", "Provider state", "scalar", json!(provider_state), "ql service capabilities", observed),
                setting("service.operations", "Negotiated operations", "table", json!(service_operations), "ql service capabilities", observed),
            ],
        }),
    ];

    let actions = vec![json!({
        "action_ref": "ql.kernel.apply",
        "title": "Apply a deterministic kernel operator",
        "args": [
            {"name": "operator", "kind": "select"},
            {"name": "address", "kind": "string"},
        ],
        "availability": "disclosed",
        "unavailable_reason": null,
        "subject_kinds": ["quaternal-logic.address"],
        "authority": { "requires": [], "granted_by": "quaternal-logic", "evidence_ref": null },
        "exposure": { "ui": true, "agent": true, "headless": true },
        "explain": { "ref": "ql kernel capabilities", "command": ["ql", "kernel", "capabilities", "--json"] },
        "history": { "ref": null, "command": null },
    })];

    let body = json!({
        "schema": SCHEMA,
        "product_id": PRODUCT_ID,
        "contract_revision": CONTRACT_REVISION,
        "about": ABOUT,
        "sections": sections,
        "actions": actions,
        "availability": availability,
        "degradations": degradations,
        "obligations": [],
    });

    let mut descriptor = body;
    descriptor["disclosed_at_unix_ms"] = json!(observed);
    descriptor["owner"] = json!({
        "owner_id": PRODUCT_ID,
        "owner_ref": OWNER_REF,
        "owner_version": env!("CARGO_PKG_VERSION"),
        "reading_command": ["ql", "system", "--json"],
        "reading_digest": null,
        "reading_digest_covers": "descriptor with every *_unix_ms field zeroed (disclosed_at_unix_ms, owner.observed_at_unix_ms, every axes.*.provenance.observed_at_unix_ms) and owner.reading_digest set to null",
        "observed_at_unix_ms": observed,
    });

    // §4.5 canonical body: the whole descriptor with every `*_unix_ms` zeroed and
    // `owner.reading_digest` null, so two readings of an unchanged world hash the
    // same and a changed digest means a changed reading, never a changed clock.
    let mut canonical = descriptor.clone();
    zero_unix_ms(&mut canonical);
    if let Some(owner) = canonical.get_mut("owner").and_then(Value::as_object_mut) {
        owner.insert("reading_digest".into(), Value::Null);
    }
    let digest = sha256_hex(&serde_json::to_string(&canonical).map_err(crate::CliError::from)?);
    descriptor["owner"]["reading_digest"] = json!(digest);
    Ok(descriptor)
}

fn readiness() -> (String, usize, Option<String>) {
    match crate::verify_command(true) {
        Ok(output) => {
            let parsed: Value = serde_json::from_str(&output).unwrap_or_default();
            let status = parsed["status"].as_str().unwrap_or("unknown").to_string();
            let checks = parsed["checks"].as_array().map_or(0, Vec::len);
            (status, checks, None)
        }
        Err(error) => ("unavailable".to_string(), 0, Some(error.to_string())),
    }
}

fn setting(
    key: &str,
    title: &str,
    kind: &str,
    value: Value,
    native_path: &str,
    observed_at: i64,
) -> Value {
    json!({
        "key": key,
        "title": title,
        "kind": kind,
        "axes": axes(value, native_path, observed_at),
        "mutable": false,
        "native_path": native_path,
        "bootstrap": false,
        "drift": {
            "state": "none",
            "between": ["declared", "effective"],
            "remediation_action_ref": null,
        },
    })
}

fn axes(value: Value, native_path: &str, observed_at: i64) -> Value {
    let provenance = json!({
        "owner_ref": OWNER_REF,
        "path": native_path,
        "observed_at_unix_ms": observed_at,
    });
    json!({
        "declared":  { "value": value.clone(), "provenance": provenance.clone() },
        "effective": { "value": value.clone(), "provenance": provenance.clone() },
        "active":    { "value": value, "provenance": provenance, "materialisation_ref": null },
        "staged":    { "value": {}, "provenance": provenance.clone(), "stage_ref": null, "stage_state": "none" },
        "expected_effect": { "summary": "Nothing is staged: this QL setting is disclosed read-only and is never staged for application, so applying it would change nothing.", "ref": null },
    })
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

/// Zero every `*_unix_ms` field in the descriptor, recursively, so the canonical
/// reading body is independent of when the reading was taken (§4.5).
fn zero_unix_ms(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let keys: Vec<String> = map
                .keys()
                .filter(|key| key.ends_with("_unix_ms"))
                .cloned()
                .collect();
            for key in keys {
                map.insert(key, json!(0i64));
            }
            for (_, child) in map.iter_mut() {
                zero_unix_ms(child);
            }
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                zero_unix_ms(item);
            }
        }
        _ => {}
    }
}

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor() -> Value {
        let output = system_command(true).unwrap();
        serde_json::from_str(&output).unwrap()
    }

    #[test]
    fn disclosure_matches_frozen_v2_contract() {
        let value = descriptor();
        assert_eq!(value["schema"], SCHEMA);
        assert_eq!(value["product_id"], PRODUCT_ID);
        assert_eq!(value["contract_revision"], CONTRACT_REVISION);
        assert_eq!(value["owner"]["owner_id"], "quaternal-logic");
        assert_eq!(value["owner"]["owner_ref"], OWNER_REF);
        assert_eq!(
            value["owner"]["reading_command"],
            json!(["ql", "system", "--json"])
        );
        assert_eq!(value["availability"]["state"], "available");
        assert!(value["disclosed_at_unix_ms"].is_i64());
        assert!(value["owner"]["observed_at_unix_ms"].is_i64());
    }

    #[test]
    fn digest_verifies_against_canonical_body() {
        let value = descriptor();
        let mut body = value.clone();
        zero_unix_ms(&mut body);
        body["owner"]["reading_digest"] = Value::Null;
        let recomputed = sha256_hex(&serde_json::to_string(&body).unwrap());
        assert_eq!(value["owner"]["reading_digest"], recomputed);
        let digest = value["owner"]["reading_digest"].as_str().unwrap();
        assert_eq!(digest.len(), 64);
        assert!(
            digest
                .chars()
                .all(|character| character.is_ascii_hexdigit())
        );
    }

    #[test]
    fn every_setting_carries_all_axes_with_provenance() {
        let value = descriptor();
        let sections = value["sections"].as_array().unwrap();
        assert_eq!(sections.len(), 3);
        for section in sections {
            for setting in section["settings"].as_array().unwrap() {
                for axis in ["declared", "effective", "active"] {
                    assert!(
                        setting["axes"][axis]["provenance"]["owner_ref"].is_string(),
                        "missing provenance on {axis}"
                    );
                    assert!(setting["axes"][axis]["provenance"]["observed_at_unix_ms"].is_i64());
                }
                assert_eq!(setting["axes"]["staged"]["stage_state"], "none");
                assert_eq!(setting["axes"]["staged"]["value"], json!({}));
                let summary = setting["axes"]["expected_effect"]["summary"]
                    .as_str()
                    .unwrap();
                assert!(
                    !summary.is_empty() && summary != "none",
                    "expected_effect.summary must be a real sentence, not \"none\""
                );
                assert_eq!(setting["mutable"], false);
                assert_eq!(setting["drift"]["state"], "none");
            }
        }
    }

    #[test]
    fn kernel_readings_are_truthful() {
        let value = descriptor();
        let kernel = &value["sections"][0]["settings"];
        let operators = kernel[3]["axes"]["active"]["value"].as_array().unwrap();
        assert_eq!(operators.len(), 3);
        let forms = kernel[2]["axes"]["active"]["value"].as_array().unwrap();
        assert_eq!(forms.len(), 3);
        let readiness = value["sections"][1]["settings"][0]["axes"]["active"]["value"]
            .as_str()
            .unwrap();
        assert_eq!(readiness, "ok");
    }

    #[test]
    fn disclosure_does_not_expose_vak_or_context_frame() {
        let output = system_command(true).unwrap();
        assert!(!output.contains("vak"));
        assert!(!output.contains("Vāk"));
        assert!(!output.contains("context-frame"));
        assert!(!output.contains("contextFrame"));
    }
}
