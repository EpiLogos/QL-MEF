//! C3F owner contribution: the QL configuration plane (O-I #299, Gate A / C0,
//! `docs/cradle/09-CONFIGURATION-PLANE.md`).
//!
//! `ql config-contribution --json` emits the frozen
//! `oi.configuration-contribution/v1` document; `ql config
//! validate|plan|apply|reset --json` implement the frozen four-verb transport
//! (09 §6). Failures of a `--json` transport invocation exit non-zero with an
//! `oi.config-error/v1` document on stdout.
//!
//! The C3F law is *only what exists*: every accepted QL kernel faculty is a
//! read-only disclosure, so the honest surface is a disclosure-only
//! contribution (`writable: false` on every setting), plan/apply/reset refuse
//! with a structured `unsupported_setting` error, and no receipt is ever
//! minted. Section ids and setting keys map one-to-one onto the `ql system
//! --json` v2 disclosure (09 §17). Nothing here is invented for symmetry with
//! owners that do accept mutation, and O:I must never require QL.

use serde_json::{Value, json};

use crate::CliError;

const CONTRIBUTION_SCHEMA: &str = "oi.configuration-contribution/v1";
const CONTRACT_REVISION: &str = "configuration-plane/contribution.1";
const VALIDATION_SCHEMA: &str = "oi.config-validation/v1";
const PLAN_SCHEMA: &str = "oi.config-plan/v1";
const ERROR_SCHEMA: &str = "oi.config-error/v1";
const OWNER_REF: &str = "quaternal-logic";

/// Frozen scope-kind registry (09 §5). The singular kinds take no `scope_ref`.
const SCOPE_KINDS: [&str; 12] = [
    "world",
    "ground",
    "project",
    "machine",
    "workcell",
    "agency",
    "agent",
    "session-space",
    "agent-session",
    "provider",
    "connector-relation",
    "invocation",
];
const SINGULAR_SCOPE_KINDS: [&str; 3] = ["world", "ground", "machine"];

const ABOUT: &str = "Quaternal Logic's configuration surface is disclosure-only: the accepted \
deterministic kernel, native verification and provider-backed service faculties are read-only \
kernel facts. QL contributes no writable setting; the four transport verbs exist per the frozen \
grammar and refuse mutation with a structured unsupported_setting error.";

const EFFECT_SUMMARY: &str = "QL offers no mutation for this kernel faculty through the \
configuration plane; the value is a read-only kernel fact of the accepted build, so applying a \
change would change nothing.";

struct SectionSpec {
    id: &'static str,
    title: &'static str,
    settings: &'static [SettingSpec],
}

struct SettingSpec {
    key: &'static str,
    title: &'static str,
    description: &'static str,
    value_schema_json: &'static str,
    native_ref: &'static str,
}

impl SettingSpec {
    /// The declared value contract (09 §2.2/§2.3), parsed on demand.
    fn value_schema(&self) -> Value {
        serde_json::from_str(self.value_schema_json).expect("declared value schema is valid JSON")
    }
}

/// The contributed surface, structurally identical to the `ql system --json`
/// v2 disclosure (same section ids, same setting keys); `native_ref` names the
/// owner-namespace location each value comes from (09 §2.2, 07 §4.6: a
/// location, never a command string).
static SECTIONS: &[SectionSpec] = &[
    SectionSpec {
        id: "kernel",
        title: "QL Kernel",
        settings: &[
            SettingSpec {
                key: "kernel.version",
                title: "Kernel version",
                description: "Version of the accepted deterministic QL kernel in this build.",
                value_schema_json: r#"{ "type": "scalar" }"#,
                native_ref: "quaternal-logic:kernel:capabilities",
            },
            SettingSpec {
                key: "kernel.schema-version",
                title: "Kernel schema version",
                description: "Schema version of the kernel's address and operator contracts.",
                value_schema_json: r#"{ "type": "scalar" }"#,
                native_ref: "quaternal-logic:kernel:capabilities",
            },
            SettingSpec {
                key: "kernel.supported-forms",
                title: "Supported forms",
                description: "The quaternal forms the accepted kernel supports.",
                value_schema_json: r#"{ "type": "list", "items": { "type": "scalar" } }"#,
                native_ref: "quaternal-logic:kernel:capabilities",
            },
            SettingSpec {
                key: "kernel.deterministic-operators",
                title: "Deterministic operators",
                description: "The three accepted deterministic kernel operators.",
                value_schema_json: r#"{ "type": "list", "items": { "type": "scalar" } }"#,
                native_ref: "quaternal-logic:kernel:capabilities",
            },
            SettingSpec {
                key: "kernel.stochastic-operators",
                title: "Stochastic operators",
                description: "Stochastic operator families disclosed by the kernel.",
                value_schema_json: r#"{ "type": "list", "items": { "type": "scalar" } }"#,
                native_ref: "quaternal-logic:kernel:capabilities",
            },
            SettingSpec {
                key: "kernel.research-operators",
                title: "Research operators",
                description: "Research operator families disclosed by the kernel.",
                value_schema_json: r#"{ "type": "list", "items": { "type": "scalar" } }"#,
                native_ref: "quaternal-logic:kernel:capabilities",
            },
        ],
    },
    SectionSpec {
        id: "readiness",
        title: "Native verification",
        settings: &[
            SettingSpec {
                key: "readiness.status",
                title: "Verification status",
                description: "Outcome of the latest native kernel verification.",
                value_schema_json: r#"{ "type": "scalar" }"#,
                native_ref: "quaternal-logic:verify:report",
            },
            SettingSpec {
                key: "readiness.check-count",
                title: "Verification checks",
                description: "Number of native checks the verification runs.",
                value_schema_json: r#"{ "type": "integer", "minimum": 0 }"#,
                native_ref: "quaternal-logic:verify:report",
            },
        ],
    },
    SectionSpec {
        id: "service",
        title: "Provider-backed service",
        settings: &[
            SettingSpec {
                key: "service.provider-state",
                title: "Provider state",
                description: "Observed state of the provider-backed service relation.",
                value_schema_json: r#"{
                    "type": "enum",
                    "options": [
                        { "value": "absent" },
                        { "value": "available" },
                        { "value": "degraded" },
                        { "value": "incompatible" }
                    ]
                }"#,
                native_ref: "quaternal-logic:service:capabilities",
            },
            SettingSpec {
                key: "service.operations",
                title: "Negotiated operations",
                description: "Per-operation negotiated availability of the service boundary.",
                value_schema_json: r#"{
                    "type": "table",
                    "columns": [
                        { "name": "operation", "type": "scalar" },
                        { "name": "supported", "type": "boolean" },
                        { "name": "deterministic", "type": "boolean" }
                    ]
                }"#,
                native_ref: "quaternal-logic:service:capabilities",
            },
        ],
    },
];

fn contributed_settings() -> impl Iterator<Item = (&'static SectionSpec, &'static SettingSpec)> {
    SECTIONS.iter().flat_map(|section| {
        section
            .settings
            .iter()
            .map(move |setting| (section, setting))
    })
}

fn setting_ref(section: &SectionSpec, setting: &SettingSpec) -> String {
    format!("{OWNER_REF}:{}:{}", section.id, setting.key)
}

fn allowed_scopes() -> Value {
    // The kernel is this-machine state; no other scope kind is meaningful.
    json!([{ "scope_kind": "machine", "scope_ref": null }])
}

/// Per-setting capability. `validate` answers whether a proposed value
/// conforms to the declared value contract; plan/apply/reset are refused at
/// the transport because no QL setting is writable (09 §2.2 agreement law:
/// a non-writable setting must not declare them).
fn setting_operations() -> Value {
    json!({ "validate": true, "plan": false, "apply": false, "reset": false })
}

fn setting_effect() -> Value {
    json!({ "kind": "none", "summary": EFFECT_SUMMARY, "ref": null })
}

/// Owner-level transport capabilities (09 §6): the four verbs exist and answer
/// per the frozen grammar. Per-setting truth stays in each SettingSpec.
fn transport_operations() -> Value {
    json!({
        "transport": "cli/v1",
        "validate": { "availability": "disclosed" },
        "plan": { "availability": "disclosed" },
        "apply": { "availability": "disclosed" },
        "reset": { "availability": "disclosed" }
    })
}

pub fn contribution_command(json_out: bool) -> Result<String, CliError> {
    let document = build_contribution()?;
    if json_out {
        return serde_json::to_string_pretty(&document).map_err(CliError::from);
    }
    let settings: usize = SECTIONS.iter().map(|section| section.settings.len()).sum();
    Ok(format!(
        "quaternal-logic configuration contribution ({CONTRIBUTION_SCHEMA}):\n\
         {settings} settings across {} sections, 0 writable (disclosure-only)\n\
         availability: {}",
        SECTIONS.len(),
        document["availability"]["state"]
            .as_str()
            .unwrap_or("unknown"),
    ))
}

fn build_contribution() -> Result<Value, CliError> {
    let observed = crate::system::now_ms();

    let sections = SECTIONS
        .iter()
        .map(|section| {
            json!({
                "id": section.id,
                "title": section.title,
                "settings": section.settings
                    .iter()
                    .map(|setting| setting_spec(section, setting))
                    .collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();

    // Availability is probed, never asserted (07 §4.7): the same native
    // verification that backs the v2 disclosure backs this document.
    let (_, _, verify_error) = crate::system::readiness();
    let availability = if verify_error.is_some() {
        json!({ "state": "degraded", "reason": verify_error })
    } else {
        json!({ "state": "available", "reason": null })
    };
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

    let body = json!({
        "schema": CONTRIBUTION_SCHEMA,
        "contract_revision": CONTRACT_REVISION,
        "owner": {
            "owner_ref": OWNER_REF,
            "owner_kind": "product",
            "owner_version": env!("CARGO_PKG_VERSION"),
            "contribution_command": ["ql", "config-contribution", "--json"],
            "disclosed_at_unix_ms": observed,
            "reading_digest": null,
            "reading_digest_covers": "07 §4.5 convention",
        },
        "about": ABOUT,
        "sections": sections,
        "operations": transport_operations(),
        "availability": availability,
        "degradations": degradations,
        "obligations": [
            "QL is an optional semantic owner: no baseline O:I dependency on QL exists or may be created (#299 C3F).",
            "QL contributes no writable setting. If a native configurable faculty is ever accepted, it becomes writable through a new accepted QL release, not through this plane.",
        ],
    });

    let mut document = body;
    // 07 §4.5 convention: the digest covers the document with every
    // `*_unix_ms` zeroed and `owner.reading_digest` null.
    let mut canonical = document.clone();
    crate::system::zero_unix_ms(&mut canonical);
    if let Some(owner) = canonical.get_mut("owner").and_then(Value::as_object_mut) {
        owner.insert("reading_digest".into(), Value::Null);
    }
    let digest =
        crate::system::sha256_hex(&serde_json::to_string(&canonical).map_err(CliError::from)?);
    document["owner"]["reading_digest"] = json!(digest);
    Ok(document)
}

fn setting_spec(section: &SectionSpec, setting: &SettingSpec) -> Value {
    json!({
        "setting_ref": setting_ref(section, setting),
        "section_ref": section.id,
        "title": setting.title,
        "description": setting.description,
        "value_schema": setting.value_schema(),
        "allowed_scopes": allowed_scopes(),
        "writable": false,
        "profileable": false,
        "sensitive": false,
        "default_semantics": "none",
        "effect": setting_effect(),
        "operations": setting_operations(),
        "native_ref": setting.native_ref,
    })
}

/// Failure of a configuration-plane command. For `--json` transport failures
/// `document` carries the `oi.config-error/v1` stdout document (09 §6);
/// human-mode failures stay plain messages like the rest of the CLI.
#[derive(Debug)]
pub struct ConfigFailure {
    pub document: Option<String>,
    pub message: String,
}

impl ConfigFailure {
    fn fail(json_out: bool, error_code: &str, message: impl Into<String>) -> Self {
        Self::fail_with_subject(json_out, error_code, message, None, None)
    }

    fn fail_with_subject(
        json_out: bool,
        error_code: &str,
        message: impl Into<String>,
        setting_ref: Option<&str>,
        scope_kind: Option<&str>,
    ) -> Self {
        let message = message.into();
        let document = if json_out {
            Some(
                json!({
                    "schema": ERROR_SCHEMA,
                    "error_code": error_code,
                    "message": message,
                    "setting_ref": setting_ref,
                    "scope_kind": scope_kind,
                    "retryable": false,
                })
                .to_string(),
            )
        } else {
            None
        };
        Self { document, message }
    }

    fn plain(message: impl Into<String>) -> Self {
        Self {
            document: None,
            message: message.into(),
        }
    }
}

/// The four-verb transport (09 §6). Success yields the bare response document
/// (JSON mode) or a short human line; a `--json` failure yields the structured
/// error document and a non-zero exit.
pub fn command(args: &[String], json_out: bool) -> Result<String, ConfigFailure> {
    let Some(verb) = args.first().map(String::as_str) else {
        return Err(if json_out {
            ConfigFailure::fail(
                true,
                "invalid_value",
                "missing configuration verb; expected validate, plan, apply or reset",
            )
        } else {
            ConfigFailure::plain(
                "missing configuration verb; expected validate, plan, apply or reset",
            )
        });
    };
    let request = parse_flags(json_out, &args[1..])?;
    match verb {
        "validate" => validate_verb(json_out, &request),
        "plan" => plan_verb(json_out, &request),
        "apply" => apply_verb(json_out, &request),
        "reset" => reset_verb(json_out, &request),
        other => Err(ConfigFailure::fail(
            json_out,
            "invalid_value",
            format!(
                "unknown configuration verb `{other}`; expected validate, plan, apply or reset"
            ),
        )),
    }
}

struct VerbRequest {
    setting: Option<String>,
    scope: Option<String>,
    value: Option<String>,
    value_file: Option<String>,
    plan_file: Option<String>,
    changeset: Option<String>,
}

fn parse_flags(json_out: bool, args: &[String]) -> Result<VerbRequest, ConfigFailure> {
    let mut request = VerbRequest {
        setting: None,
        scope: None,
        value: None,
        value_file: None,
        plan_file: None,
        changeset: None,
    };
    let mut at = 0;
    while at < args.len() {
        let flag = args[at].as_str();
        let target = match flag {
            "--setting" => &mut request.setting,
            "--scope" => &mut request.scope,
            "--value" => &mut request.value,
            "--value-file" => &mut request.value_file,
            "--plan-file" => &mut request.plan_file,
            "--changeset" => &mut request.changeset,
            other => {
                return Err(ConfigFailure::fail(
                    json_out,
                    "invalid_value",
                    format!("unknown configuration option `{other}`"),
                ));
            }
        };
        let Some(value) = args.get(at + 1) else {
            return Err(ConfigFailure::fail(
                json_out,
                "invalid_value",
                format!("missing value for `{flag}`"),
            ));
        };
        if target.is_some() {
            return Err(ConfigFailure::fail(
                json_out,
                "invalid_value",
                format!("duplicate configuration option `{flag}`"),
            ));
        }
        *target = Some(value.clone());
        at += 2;
    }
    Ok(request)
}

/// The frozen setting-reference grammar (09 §3): `owner:section:key`, split on
/// `:` into exactly three components; a ref that does not parse is never
/// coerced.
fn parse_setting_ref(reference: &str) -> Option<(String, String, String)> {
    let parts: Vec<&str> = reference.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let (owner, section, key) = (parts[0], parts[1], parts[2]);
    if !is_ref_word(owner) || !is_ref_word(section) || key.is_empty() {
        return None;
    }
    // Frozen grammar (09 §3): every dotted segment is [a-z0-9_][a-z0-9_-]*.
    for segment in key.split('.') {
        let mut chars = segment.chars();
        match chars.next() {
            Some(first) if first.is_ascii_lowercase() || first.is_ascii_digit() || first == '_' => {
            }
            _ => return None,
        }
        if !chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-') {
            return None;
        }
    }
    Some((owner.into(), section.into(), key.into()))
}

fn is_ref_word(word: &str) -> bool {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() || first.is_ascii_digit() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Compact scope form (09 §5): `<kind>` for singular kinds, `<kind>:<ref>`
/// otherwise. Unknown kinds are `unknown_scope_kind`; a scope a setting does
/// not allow is `unsupported_scope` — never a fallback to another scope.
fn parse_scope(json_out: bool, compact: &str) -> Result<Value, ConfigFailure> {
    let (kind, scope_ref) = match compact.split_once(':') {
        Some((kind, reference)) => (kind, Some(reference)),
        None => (compact, None),
    };
    if !SCOPE_KINDS.contains(&kind) {
        return Err(ConfigFailure::fail_with_subject(
            json_out,
            "unknown_scope_kind",
            format!("`{compact}` uses scope kind `{kind}`, which is outside the frozen registry"),
            None,
            Some(kind),
        ));
    }
    if SINGULAR_SCOPE_KINDS.contains(&kind) {
        if let Some(reference) = scope_ref {
            return Err(ConfigFailure::fail_with_subject(
                json_out,
                "invalid_value",
                format!("singular scope kind `{kind}` takes no scope_ref (got `{reference}`)"),
                None,
                Some(kind),
            ));
        }
        return Ok(json!({ "scope_kind": kind, "scope_ref": null }));
    }
    match scope_ref {
        Some(reference) if !reference.is_empty() => {
            Ok(json!({ "scope_kind": kind, "scope_ref": reference }))
        }
        _ => Err(ConfigFailure::fail_with_subject(
            json_out,
            "unsupported_scope",
            format!("non-singular scope kind `{kind}` requires a scope_ref"),
            None,
            Some(kind),
        )),
    }
}

fn resolve_setting(
    json_out: bool,
    reference: &str,
) -> Result<(&'static SectionSpec, &'static SettingSpec), ConfigFailure> {
    let Some((owner, section_id, key)) = parse_setting_ref(reference) else {
        return Err(ConfigFailure::fail(
            json_out,
            "unsupported_setting",
            format!(
                "`{reference}` is not a valid setting ref; a ref that does not parse is never coerced"
            ),
        ));
    };
    if owner != OWNER_REF {
        return Err(ConfigFailure::fail(
            json_out,
            "unsupported_setting",
            format!("`{reference}` lives under owner `{owner}` but this owner is `{OWNER_REF}`"),
        ));
    }
    contributed_settings()
        .find(|(section, setting)| section.id == section_id && setting.key == key)
        .ok_or_else(|| {
            ConfigFailure::fail_with_subject(
                json_out,
                "unsupported_setting",
                format!("`{reference}` is not contributed by {OWNER_REF}"),
                Some(reference),
                None,
            )
        })
}

fn scope_is_allowed(scope: &Value) -> bool {
    let kind = scope["scope_kind"].as_str().unwrap_or_default();
    allowed_scopes().as_array().is_some_and(|scopes| {
        scopes
            .iter()
            .any(|allowed| allowed["scope_kind"] == json!(kind))
    })
}

/// Every addressable configuration request carries an explicit scope (09 §5);
/// a request without one is never silently re-scoped.
fn requested_scope(
    json_out: bool,
    request: &VerbRequest,
    setting_ref: &str,
) -> Result<Value, ConfigFailure> {
    let Some(compact) = request.scope.as_deref() else {
        return Err(ConfigFailure::fail_with_subject(
            json_out,
            "invalid_value",
            "missing --scope; every configuration request carries an explicit scope",
            Some(setting_ref),
            None,
        ));
    };
    parse_scope(json_out, compact)
}

fn require_scope_allowed(
    json_out: bool,
    scope: &Value,
    setting_ref: &str,
    compact: &str,
) -> Result<(), ConfigFailure> {
    if scope_is_allowed(scope) {
        return Ok(());
    }
    Err(ConfigFailure::fail_with_subject(
        json_out,
        "unsupported_scope",
        format!(
            "`{compact}` is not within the allowed scopes of `{setting_ref}`; allowed: machine"
        ),
        Some(setting_ref),
        scope["scope_kind"].as_str(),
    ))
}

fn requested_value(
    json_out: bool,
    request: &VerbRequest,
    setting_ref: &str,
) -> Result<Value, ConfigFailure> {
    match (request.value.as_deref(), request.value_file.as_deref()) {
        (Some(raw), None) => serde_json::from_str(raw).map_err(|error| {
            ConfigFailure::fail_with_subject(
                json_out,
                "invalid_value",
                format!("--value is not valid JSON: {error}"),
                Some(setting_ref),
                None,
            )
        }),
        (None, Some(path)) => {
            let raw = read_input(path).map_err(|message| {
                ConfigFailure::fail_with_subject(
                    json_out,
                    "invalid_value",
                    message,
                    Some(setting_ref),
                    None,
                )
            })?;
            serde_json::from_str(&raw).map_err(|error| {
                ConfigFailure::fail_with_subject(
                    json_out,
                    "invalid_value",
                    format!("--value-file {path} does not hold valid JSON: {error}"),
                    Some(setting_ref),
                    None,
                )
            })
        }
        (Some(_), Some(_)) => Err(ConfigFailure::fail_with_subject(
            json_out,
            "invalid_value",
            "pass either --value or --value-file, never both",
            Some(setting_ref),
            None,
        )),
        (None, None) => Err(ConfigFailure::fail_with_subject(
            json_out,
            "invalid_value",
            "missing --value <json> or --value-file <path|->",
            Some(setting_ref),
            None,
        )),
    }
}

fn read_input(path: &str) -> Result<String, String> {
    if path == "-" {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|error| format!("could not read {path}: {error}"))?;
        Ok(buffer)
    } else {
        std::fs::read_to_string(path).map_err(|error| format!("could not read {path}: {error}"))
    }
}

fn required_setting(
    json_out: bool,
    request: &VerbRequest,
) -> Result<(&'static SectionSpec, &'static SettingSpec, String), ConfigFailure> {
    let Some(reference) = request.setting.as_deref() else {
        return Err(ConfigFailure::fail(
            json_out,
            "invalid_value",
            "missing --setting <setting_ref>",
        ));
    };
    let (section, setting) = resolve_setting(json_out, reference)?;
    Ok((section, setting, setting_ref(section, setting)))
}

fn validate_verb(json_out: bool, request: &VerbRequest) -> Result<String, ConfigFailure> {
    let (_, setting, full_ref) = required_setting(json_out, request)?;
    let scope = requested_scope(json_out, request, &full_ref)?;
    require_scope_allowed(
        json_out,
        &scope,
        &full_ref,
        request.scope.as_deref().unwrap_or_default(),
    )?;
    let value = requested_value(json_out, request, &full_ref)?;
    let mut violations = Vec::new();
    validate_against_schema(&setting.value_schema(), &value, "$", &mut violations);
    let document = json!({
        "schema": VALIDATION_SCHEMA,
        "setting_ref": full_ref,
        "scope": scope,
        "valid": violations.is_empty(),
        "violations": violations,
        "expected_effect": setting_effect(),
    });
    if json_out {
        serde_json::to_string_pretty(&document)
            .map_err(|error| ConfigFailure::fail(true, "internal", error.to_string()))
    } else {
        Ok(format!(
            "{full_ref}: {}",
            if violations.is_empty() {
                "valid"
            } else {
                "invalid"
            }
        ))
    }
}

/// QL contributes no writable setting, so plan always refuses: the setting
/// exists, is addressed correctly, and is still not offered to mutation. This
/// mirrors the O:I kernel orchestration law for `writable: false` specs.
fn plan_verb(json_out: bool, request: &VerbRequest) -> Result<String, ConfigFailure> {
    let (_, _, full_ref) = required_setting(json_out, request)?;
    let scope = requested_scope(json_out, request, &full_ref)?;
    require_scope_allowed(
        json_out,
        &scope,
        &full_ref,
        request.scope.as_deref().unwrap_or_default(),
    )?;
    // The frozen grammar requires a value for plan even where the owner will
    // refuse; parse it so a malformed request is never silently reframed.
    requested_value(json_out, request, &full_ref)?;
    Err(refusal(json_out, &full_ref))
}

fn reset_verb(json_out: bool, request: &VerbRequest) -> Result<String, ConfigFailure> {
    check_changeset(json_out, request)?;
    let (_, _, full_ref) = required_setting(json_out, request)?;
    let scope = requested_scope(json_out, request, &full_ref)?;
    require_scope_allowed(
        json_out,
        &scope,
        &full_ref,
        request.scope.as_deref().unwrap_or_default(),
    )?;
    Err(refusal(json_out, &full_ref))
}

/// Apply consumes an owner-minted plan. QL never mints one (nothing is
/// writable), so any plan naming a QL setting is refused: a wrong document
/// schema is `unsupported_schema`, an unknown setting `unsupported_setting`,
/// and a contributed-but-read-only setting is refused as not writable.
fn apply_verb(json_out: bool, request: &VerbRequest) -> Result<String, ConfigFailure> {
    let Some(plan_file) = request.plan_file.as_deref() else {
        return Err(ConfigFailure::fail(
            json_out,
            "invalid_value",
            "missing --plan-file <path|->",
        ));
    };
    check_changeset(json_out, request)?;
    let raw = read_input(plan_file).map_err(|message| {
        ConfigFailure::fail_with_subject(
            json_out,
            "invalid_value",
            format!("--plan-file: {message}"),
            None,
            None,
        )
    })?;
    let plan: Value = serde_json::from_str(&raw).map_err(|error| {
        ConfigFailure::fail(
            json_out,
            "invalid_value",
            format!("--plan-file does not hold valid JSON: {error}"),
        )
    })?;
    if plan["schema"] != json!(PLAN_SCHEMA) {
        return Err(ConfigFailure::fail_with_subject(
            json_out,
            "unsupported_schema",
            format!(
                "plan document schema is `{}`, expected `{PLAN_SCHEMA}`",
                plan["schema"].as_str().unwrap_or("(absent)")
            ),
            plan["setting_ref"].as_str(),
            None,
        ));
    }
    let Some(setting_ref) = plan["setting_ref"].as_str().map(str::to_owned) else {
        return Err(ConfigFailure::fail(
            json_out,
            "unsupported_schema",
            "plan document carries no setting_ref",
        ));
    };
    resolve_setting(json_out, &setting_ref)?;
    Err(refusal(json_out, &setting_ref))
}

fn refusal(json_out: bool, setting_ref: &str) -> ConfigFailure {
    ConfigFailure::fail_with_subject(
        json_out,
        "unsupported_setting",
        format!(
            "`{setting_ref}` is not writable; the owner does not offer it to mutation. \
             quaternal-logic contributes a disclosure-only configuration surface — every \
             accepted kernel faculty is read-only (#299 C3F: only what exists)."
        ),
        Some(setting_ref),
        None,
    )
}

fn check_changeset(json_out: bool, request: &VerbRequest) -> Result<(), ConfigFailure> {
    let Some(changeset) = request.changeset.as_deref() else {
        return Ok(());
    };
    let valid = changeset.starts_with("cs-")
        && changeset.len() >= 4
        && changeset
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if valid {
        Ok(())
    } else {
        Err(ConfigFailure::fail(
            json_out,
            "invalid_value",
            format!("changeset id `{changeset}` must be `cs-` + a unique suffix"),
        ))
    }
}

/// Owner-native value validation against the declared value contract. The
/// kinds here are exactly the kinds QL declares; the owner's native validation
/// remains authoritative (09 §2.3).
fn validate_against_schema(schema: &Value, value: &Value, path: &str, violations: &mut Vec<Value>) {
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
                        validate_against_schema(
                            item_schema,
                            item,
                            &format!("{path}[{index}]"),
                            violations,
                        );
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
                    match row.as_object() {
                        None => push(
                            "columns",
                            format!("expected an object row at {row_path}"),
                            row_path.as_str(),
                            violations,
                        ),
                        Some(row) => {
                            for column in &columns {
                                let name = column["name"].as_str().unwrap_or_default();
                                let Some(cell) = row.get(name) else {
                                    push(
                                        "columns",
                                        format!("row {row_path} is missing column `{name}`"),
                                        row_path.as_str(),
                                        violations,
                                    );
                                    continue;
                                };
                                let cell_ok = match column["type"].as_str().unwrap_or_default() {
                                    "scalar" => cell.is_string(),
                                    "boolean" => cell.is_boolean(),
                                    "integer" => cell.is_i64() || cell.is_u64(),
                                    "number" => cell.is_number(),
                                    _ => true,
                                };
                                if !cell_ok {
                                    push(
                                        "columns",
                                        format!(
                                            "column `{name}` at {row_path} does not match its declared type"
                                        ),
                                        row_path.as_str(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_ref_grammar_accepts_contributed_refs_and_rejects_malformed() {
        assert!(parse_setting_ref("quaternal-logic:kernel:kernel.version").is_some());
        assert!(parse_setting_ref("quaternal-logic:readiness:readiness.check-count").is_some());
        // Frozen grammar (09 §3): every dotted segment is [a-z0-9_][a-z0-9_-]*.
        assert!(parse_setting_ref("quaternal-logic:kernel:kernel.2nd-form").is_some());
        assert!(parse_setting_ref("quaternal-logic:kernel:kernel").is_some());
        assert!(parse_setting_ref("quaternal-logic:kernel").is_none());
        assert!(parse_setting_ref("quaternal-logic:kernel:Kernel.Version").is_none());
        assert!(parse_setting_ref("quaternal:logic:kernel:kernel.version").is_none());
        assert!(parse_setting_ref("QL:kernel:kernel.version").is_none());
    }

    #[test]
    fn every_contributed_setting_is_disclosure_only() {
        for (section, setting) in contributed_settings() {
            let spec = setting_spec(section, setting);
            assert_eq!(spec["writable"], json!(false));
            assert_eq!(spec["profileable"], json!(false));
            assert_eq!(spec["operations"]["plan"], json!(false));
            assert_eq!(spec["operations"]["apply"], json!(false));
            assert_eq!(spec["operations"]["reset"], json!(false));
            assert_eq!(spec["section_ref"], json!(section.id));
            assert!(
                spec["setting_ref"]
                    .as_str()
                    .unwrap()
                    .starts_with("quaternal-logic:")
            );
        }
    }

    #[test]
    fn scope_parsing_follows_the_frozen_registry() {
        assert_eq!(
            parse_scope(true, "machine").unwrap(),
            json!({ "scope_kind": "machine", "scope_ref": null })
        );
        assert_eq!(
            parse_scope(true, "project:o-i").unwrap(),
            json!({ "scope_kind": "project", "scope_ref": "o-i" })
        );
        let unknown = parse_scope(true, "cluster:west").unwrap_err();
        assert_eq!(
            unknown
                .document
                .as_deref()
                .map(|d| d.contains("unknown_scope_kind")),
            Some(true)
        );
        let missing_ref = parse_scope(true, "project").unwrap_err();
        assert!(
            missing_ref
                .document
                .as_deref()
                .map(|d| d.contains("unsupported_scope"))
                == Some(true)
        );
    }

    #[test]
    fn value_validation_covers_every_declared_kind() {
        let mut violations = Vec::new();
        validate_against_schema(
            &json!({ "type": "list", "items": { "type": "scalar" } }),
            &json!(["conjugate-address", 3]),
            "$",
            &mut violations,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0]["path"], "$[1]");

        let mut violations = Vec::new();
        validate_against_schema(
            &json!({ "type": "enum", "options": [{ "value": "absent" }] }),
            &json!("quantum"),
            "$",
            &mut violations,
        );
        assert_eq!(violations[0]["code"], "enum");

        let mut violations = Vec::new();
        validate_against_schema(
            &json!({ "type": "table", "columns": [
                { "name": "operation", "type": "scalar" },
                { "name": "supported", "type": "boolean" },
            ] }),
            &json!([{ "operation": "locate" }]),
            "$",
            &mut violations,
        );
        assert_eq!(violations[0]["code"], "columns");
    }
}
