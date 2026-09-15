//! C3F configuration-plane conformance: the `ql config-contribution` document
//! and the four-verb transport (`ql config validate|plan|apply|reset`) against
//! the frozen C0 contract (O-I #299, docs/cradle/09-CONFIGURATION-PLANE.md).
//!
//! The QL law for this lane is *only what exists*: QL natively mutates no
//! configuration, so the contribution is disclosure-only (every setting
//! `writable: false`), every mutation verb refuses with a structured
//! `unsupported_setting` error, and no receipt is ever minted.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use ql_cli::{CliFailure, execute_cli};
use serde_json::{Value, json};

fn run(args: &[&str]) -> Result<String, CliFailure> {
    execute_cli(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

/// Run a config verb that must fail and return its structured
/// `oi.config-error/v1` document (non-zero exit renders as ConfigDocument).
fn error_document(args: &[&str]) -> Value {
    let failure = run(args).expect_err("config verb must fail");
    let document = failure
        .config_document()
        .expect("a --json config failure carries its error document on stdout");
    serde_json::from_str(document).expect("error document is valid JSON")
}

fn success_document(args: &[&str]) -> Value {
    let output = run(args).expect("config verb must succeed");
    serde_json::from_str(&output).expect("response document is valid JSON")
}

fn frozen_schema(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas")
        .join(name);
    serde_json::from_str(
        &std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("frozen schema {path:?} must be readable: {error}")),
    )
    .expect("frozen schema is valid JSON")
}

fn assert_matches_frozen_schema(instance: &Value, schema_name: &str) {
    let schema = frozen_schema(schema_name);
    let errors: Vec<String> = jsonschema::validator_for(&schema)
        .expect("frozen schema compiles")
        .iter_errors(instance)
        .map(|error| error.to_string())
        .collect();
    assert!(
        errors.is_empty(),
        "document does not satisfy frozen {schema_name}: {errors:?}"
    );
}

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_file(name: &str, contents: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "ql-c3f-{}-{}-{name}",
        std::process::id(),
        TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&path, contents).expect("temp file is writable");
    path
}

#[test]
fn contribution_document_satisfies_the_frozen_json_schema() {
    let output = run(&["config-contribution", "--json"]).unwrap();
    let document: Value = serde_json::from_str(&output).unwrap();
    assert_matches_frozen_schema(&document, "oi.configuration-contribution-v1.schema.json");
    assert_eq!(document["schema"], "oi.configuration-contribution/v1");
    assert_eq!(
        document["contract_revision"],
        "configuration-plane/contribution.1"
    );
    assert_eq!(document["owner"]["owner_ref"], "quaternal-logic");
    assert_eq!(document["owner"]["owner_kind"], "product");
    assert_eq!(
        document["owner"]["contribution_command"],
        json!(["ql", "config-contribution", "--json"])
    );
    assert_eq!(document["operations"]["transport"], "cli/v1");
    assert!(document["owner"]["reading_digest"].as_str().unwrap().len() == 64);
}

#[test]
fn contribution_maps_structurally_onto_the_v2_disclosure() {
    // 09 §17: owner_ref ↔ product_id, section_ref ↔ sections[].id,
    // setting_key ↔ settings[].key — exactly, so reconciliation can always
    // find the native axes for a contributed setting.
    let disclosure: Value = serde_json::from_str(&run(&["system", "--json"]).unwrap()).unwrap();
    let contribution: Value =
        serde_json::from_str(&run(&["config-contribution", "--json"]).unwrap()).unwrap();

    let disclosure_surface = disclosure_map(&disclosure);
    let contribution_surface = disclosure_map(&contribution);
    assert_eq!(disclosure_surface, contribution_surface);

    for section in contribution["sections"].as_array().unwrap() {
        for setting in section["settings"].as_array().unwrap() {
            assert_eq!(
                setting["writable"],
                json!(false),
                "{setting} must be read-only"
            );
            assert_eq!(setting["profileable"], json!(false));
            assert_eq!(setting["operations"]["validate"], json!(true));
            assert_eq!(setting["operations"]["plan"], json!(false));
            assert_eq!(setting["operations"]["apply"], json!(false));
            assert_eq!(setting["operations"]["reset"], json!(false));
            assert_eq!(
                setting["allowed_scopes"],
                json!([{ "scope_kind": "machine", "scope_ref": null }])
            );
            assert_eq!(setting["section_ref"], section["id"]);
        }
    }
}

/// (section id → {setting keys}) for either plane's document.
fn disclosure_map(document: &Value) -> Vec<(String, Vec<String>)> {
    let mut sections: Vec<(String, Vec<String>)> = document["sections"]
        .as_array()
        .unwrap()
        .iter()
        .map(|section| {
            let mut keys: Vec<String> = section["settings"]
                .as_array()
                .unwrap()
                .iter()
                .map(|setting| match setting["key"].as_str() {
                    Some(key) => key.to_string(),
                    None => setting["setting_ref"]
                        .as_str()
                        .unwrap()
                        .split(':')
                        .nth(2)
                        .unwrap()
                        .to_string(),
                })
                .collect();
            keys.sort();
            (section["id"].as_str().unwrap().to_string(), keys)
        })
        .collect();
    sections.sort();
    sections
}

#[test]
fn declared_value_contracts_accept_the_reported_native_values() {
    // The declared value contract must be truthful to the value shape QL
    // actually reports on the v2 plane: every disclosed active value
    // validates against the contribution's own value_schema.
    let disclosure: Value = serde_json::from_str(&run(&["system", "--json"]).unwrap()).unwrap();
    let contribution: Value =
        serde_json::from_str(&run(&["config-contribution", "--json"]).unwrap()).unwrap();

    let mut contracts = std::collections::BTreeMap::new();
    for section in contribution["sections"].as_array().unwrap() {
        for setting in section["settings"].as_array().unwrap() {
            let key = setting["setting_ref"]
                .as_str()
                .unwrap()
                .split(':')
                .nth(2)
                .unwrap();
            contracts.insert(key.to_string(), setting["value_schema"].clone());
        }
    }

    for section in disclosure["sections"].as_array().unwrap() {
        for setting in section["settings"].as_array().unwrap() {
            let key = setting["key"].as_str().unwrap();
            let value = &setting["axes"]["active"]["value"];
            let schema = &contracts[key];
            let mut violations = Vec::new();
            validate_value(schema, value, "$", &mut violations);
            assert!(
                violations.is_empty(),
                "disclosed value of {key} violates its declared contract: {violations:?}"
            );
        }
    }
}

/// Mirror of the owner-native value validator (kept local so this integration
/// test exercises the documented contract, not a private helper).
fn validate_value(schema: &Value, value: &Value, path: &str, violations: &mut Vec<Value>) {
    let kind = schema["type"].as_str().unwrap_or_default();
    let push = |code: &str, message: String, path: &str, violations: &mut Vec<Value>| {
        violations.push(json!({ "code": code, "message": message, "path": path }));
    };
    match kind {
        "scalar" => {
            if !value.is_string() {
                push(
                    "type",
                    format!("expected a string, got {value}"),
                    path,
                    violations,
                );
            }
        }
        "integer" => {
            if !value.is_i64() && !value.is_u64() {
                push(
                    "type",
                    format!("expected an integer, got {value}"),
                    path,
                    violations,
                );
            } else if let Some(minimum) = schema["minimum"].as_i64() {
                if value.as_i64().is_some_and(|v| v < minimum) {
                    push(
                        "minimum",
                        format!("expected >= {minimum}, got {value}"),
                        path,
                        violations,
                    );
                }
            }
        }
        "enum" => {
            let matches = schema["options"]
                .as_array()
                .is_some_and(|options| options.iter().any(|option| option["value"] == *value));
            if !matches {
                push(
                    "enum",
                    format!("`{value}` is not a disclosed option"),
                    path,
                    violations,
                );
            }
        }
        "list" => match value.as_array() {
            Some(items) => {
                if let Some(item_schema) = schema.get("items") {
                    for (index, item) in items.iter().enumerate() {
                        validate_value(item_schema, item, &format!("{path}[{index}]"), violations);
                    }
                }
            }
            None => push(
                "type",
                format!("expected an array, got {value}"),
                path,
                violations,
            ),
        },
        "table" => match value.as_array() {
            Some(rows) => {
                let columns = schema["columns"].as_array().cloned().unwrap_or_default();
                for (index, row) in rows.iter().enumerate() {
                    let row_path = format!("{path}[{index}]");
                    let Some(row) = row.as_object() else {
                        push(
                            "columns",
                            format!("expected an object row at {row_path}"),
                            &row_path,
                            violations,
                        );
                        continue;
                    };
                    for column in &columns {
                        let name = column["name"].as_str().unwrap_or_default();
                        match row.get(name) {
                            None => push(
                                "columns",
                                format!("row {row_path} is missing column `{name}`"),
                                &row_path,
                                violations,
                            ),
                            Some(cell) => {
                                let ok = match column["type"].as_str().unwrap_or_default() {
                                    "scalar" => cell.is_string(),
                                    "boolean" => cell.is_boolean(),
                                    "integer" => cell.is_i64() || cell.is_u64(),
                                    "number" => cell.is_number(),
                                    _ => true,
                                };
                                if !ok {
                                    push(
                                        "columns",
                                        format!(
                                            "column `{name}` at {row_path} does not match its declared type"
                                        ),
                                        &row_path,
                                        violations,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            None => push(
                "type",
                format!("expected an array, got {value}"),
                path,
                violations,
            ),
        },
        other => push(
            "schema",
            format!("value kind `{other}` is not declared by this contribution"),
            path,
            violations,
        ),
    }
}

#[test]
fn contribution_carries_no_native_state_axes() {
    // Plane separation (09 §1): a contribution never carries declared /
    // effective / active values, and never a desired axis.
    let output = run(&["config-contribution", "--json"]).unwrap();
    let document: Value = serde_json::from_str(&output).unwrap();
    for forbidden in ["declared", "effective", "active", "desired", "staged"] {
        assert!(
            !contains_key(&document, forbidden),
            "contribution must not carry the `{forbidden}` axis"
        );
    }
}

fn contains_key(value: &Value, key: &str) -> bool {
    match value {
        Value::Object(map) => {
            map.contains_key(key) || map.values().any(|child| contains_key(child, key))
        }
        Value::Array(items) => items.iter().any(|child| contains_key(child, key)),
        _ => false,
    }
}

#[test]
fn validate_round_trips_conforming_values() {
    let document = success_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "machine",
        "--value",
        "\"9.9.9\"",
    ]);
    assert_matches_frozen_schema(&document, "oi.config-validation-v1.schema.json");
    assert_eq!(document["valid"], json!(true));
    assert_eq!(
        document["setting_ref"],
        "quaternal-logic:kernel:kernel.version"
    );
    assert_eq!(
        document["scope"],
        json!({ "scope_kind": "machine", "scope_ref": null })
    );

    let operators = success_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.deterministic-operators",
        "--scope",
        "machine",
        "--value",
        r#"["conjugate-address", "complement-address", "classify-four-plus-two"]"#,
    ]);
    assert_eq!(operators["valid"], json!(true));

    let provider_state = success_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:service:service.provider-state",
        "--scope",
        "machine",
        "--value",
        "\"absent\"",
    ]);
    assert_eq!(provider_state["valid"], json!(true));

    let operations_path = temp_file(
        "operations.json",
        r#"[{ "operation": "locate", "supported": false, "deterministic": false }]"#,
    );
    let operations_table = success_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:service:service.operations",
        "--scope",
        "machine",
        "--value-file",
        operations_path.to_str().unwrap(),
    ]);
    assert_eq!(operations_table["valid"], json!(true));
}

#[test]
fn validate_reports_owner_violations_for_non_conforming_values() {
    let document = success_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:readiness:readiness.check-count",
        "--scope",
        "machine",
        "--value",
        "\"many\"",
    ]);
    assert_eq!(document["valid"], json!(false));
    assert!(!document["violations"].as_array().unwrap().is_empty());

    let document = success_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:service:service.provider-state",
        "--scope",
        "machine",
        "--value",
        "\"quantum\"",
    ]);
    assert_eq!(document["valid"], json!(false));
    assert_eq!(document["violations"][0]["code"], "enum");

    // A valid:false answer is still a successful validation (exit 0): the
    // owner answered the question.
    let output = run(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "machine",
        "--value",
        "42",
    ])
    .unwrap();
    assert!(serde_json::from_str::<Value>(&output).unwrap()["valid"] == json!(false));
}

#[test]
fn unknown_settings_and_scopes_are_structured_errors() {
    let document = error_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.depth",
        "--scope",
        "machine",
        "--value",
        "1",
    ]);
    assert_matches_frozen_schema(&document, "oi.config-error-v1.schema.json");
    assert_eq!(document["error_code"], "unsupported_setting");

    // A ref that does not parse is never coerced (09 §3).
    let document = error_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel",
        "--scope",
        "machine",
        "--value",
        "1",
    ]);
    assert_eq!(document["error_code"], "unsupported_setting");

    // Another product's setting is not contributed here.
    let document = error_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "ai-kit:resolution:model.default",
        "--scope",
        "machine",
        "--value",
        "\"sonnet\"",
    ]);
    assert_eq!(document["error_code"], "unsupported_setting");

    let document = error_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "cluster:west",
        "--value",
        "\"1\"",
    ]);
    assert_eq!(document["error_code"], "unknown_scope_kind");

    // machine is the only allowed scope kind; no fallback to another scope.
    let document = error_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "project:epilogos/o-i",
        "--value",
        "\"1\"",
    ]);
    assert_eq!(document["error_code"], "unsupported_scope");

    // Non-singular kinds need a scope_ref (scope-cases fixture law).
    let document = error_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "project",
        "--value",
        "\"1\"",
    ]);
    assert_eq!(document["error_code"], "unsupported_scope");

    // Every request carries an explicit scope (09 §5).
    let document = error_document(&[
        "config",
        "validate",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--value",
        "\"1\"",
    ]);
    assert_eq!(document["error_code"], "invalid_value");
}

#[test]
fn plan_apply_and_reset_refuse_mutation_structurally() {
    // Plan: the setting exists and is addressed correctly, and is still not
    // offered to mutation.
    let document = error_document(&[
        "config",
        "plan",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "machine",
        "--value",
        "\"9.9.9\"",
    ]);
    assert_matches_frozen_schema(&document, "oi.config-error-v1.schema.json");
    assert_eq!(document["error_code"], "unsupported_setting");
    assert_eq!(
        document["setting_ref"],
        "quaternal-logic:kernel:kernel.version"
    );

    // Reset: the same refusal.
    let document = error_document(&[
        "config",
        "reset",
        "--json",
        "--setting",
        "quaternal-logic:service:service.provider-state",
        "--scope",
        "machine",
        "--changeset",
        "cs-c3f-reset",
    ]);
    assert_eq!(document["error_code"], "unsupported_setting");

    // Apply of a well-formed owner plan: refused, because QL never mints
    // executable plans and offers no setting to mutation.
    let plan = temp_file(
        "plan.json",
        &json!({
            "schema": "oi.config-plan/v1",
            "plan_id": "ql-plan-1",
            "plan_digest": "9ce44a",
            "setting_ref": "quaternal-logic:kernel:kernel.version",
            "scope": { "scope_kind": "machine", "scope_ref": null },
            "changes": [{ "summary": "set kernel version" }],
            "expected_effect": { "kind": "none", "summary": null, "ref": null }
        })
        .to_string(),
    );
    let document = error_document(&[
        "config",
        "apply",
        "--json",
        "--plan-file",
        plan.to_str().unwrap(),
        "--changeset",
        "cs-c3f-apply",
    ]);
    assert_eq!(document["error_code"], "unsupported_setting");

    // Apply of a foreign-schema document is an explicit schema error.
    let foreign = temp_file(
        "foreign.json",
        &json!({ "schema": "oi.product-settings-disclosure/v2" }).to_string(),
    );
    let document = error_document(&[
        "config",
        "apply",
        "--json",
        "--plan-file",
        foreign.to_str().unwrap(),
    ]);
    assert_eq!(document["error_code"], "unsupported_schema");

    // Apply naming an unknown setting is unsupported_setting, not internal.
    let unknown = temp_file(
        "unknown.json",
        &json!({
            "schema": "oi.config-plan/v1",
            "plan_id": "ql-plan-2",
            "plan_digest": "deadbe",
            "setting_ref": "quaternal-logic:kernel:kernel.depth",
            "scope": { "scope_kind": "machine", "scope_ref": null },
            "changes": [{ "summary": "set unknown key" }],
            "expected_effect": { "kind": "none", "summary": null, "ref": null }
        })
        .to_string(),
    );
    let document = error_document(&[
        "config",
        "apply",
        "--json",
        "--plan-file",
        unknown.to_str().unwrap(),
    ]);
    assert_eq!(document["error_code"], "unsupported_setting");
}

#[test]
fn replay_is_deterministic_and_no_receipt_is_ever_minted() {
    // Idempotency (09 §9) is enforced owner-side. QL's writable surface is
    // empty, so no execution can occur and the replay key
    // (owner_ref, changeset_id, setting_ref, scope, plan_digest) has no
    // executed entry to replay: a repeated request is refused again, exactly
    // as before, with no state change anywhere.
    let first = error_document(&[
        "config",
        "plan",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "machine",
        "--value",
        "\"9.9.9\"",
    ]);
    let second = error_document(&[
        "config",
        "plan",
        "--json",
        "--setting",
        "quaternal-logic:kernel:kernel.version",
        "--scope",
        "machine",
        "--value",
        "\"9.9.9\"",
    ]);
    assert_eq!(first, second);

    // The empty writable surface means oi.config-receipt/v1 is unreachable.
    for verb in [
        vec![
            "validate",
            "--setting",
            "quaternal-logic:kernel:kernel.version",
            "--scope",
            "machine",
            "--value",
            "\"1\"",
        ],
        vec![
            "plan",
            "--setting",
            "quaternal-logic:kernel:kernel.version",
            "--scope",
            "machine",
            "--value",
            "\"1\"",
        ],
        vec![
            "reset",
            "--setting",
            "quaternal-logic:kernel:kernel.version",
            "--scope",
            "machine",
        ],
    ] {
        let mut args = vec!["config", "--json"];
        args.extend_from_slice(&verb);
        match run(&args) {
            Ok(output) => assert!(!output.contains("oi.config-receipt/v1")),
            Err(failure) => {
                let document = failure.config_document().unwrap();
                assert!(!document.contains("oi.config-receipt/v1"));
            }
        }
    }
}

#[test]
fn contribution_digest_verifies_against_canonical_body() {
    let output = run(&["config-contribution", "--json"]).unwrap();
    let document: Value = serde_json::from_str(&output).unwrap();
    let mut canonical = document.clone();
    zero_unix_ms(&mut canonical);
    canonical["owner"]["reading_digest"] = Value::Null;
    let digest = crate_sha256(&serde_json::to_string(&canonical).unwrap());
    assert_eq!(document["owner"]["reading_digest"], json!(digest));
}

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
        Value::Array(items) => items.iter_mut().for_each(zero_unix_ms),
        _ => {}
    }
}

fn crate_sha256(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
