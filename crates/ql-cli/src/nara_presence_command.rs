//! Read the existing native consent law without constructing a personal field.
//! Consent persistence and authenticated participant admission remain host-owned.
use crate::CliError;
use ql_mef::nara::expression::SharedPresenceConsent;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: String,
    consent: SharedPresenceConsent,
    subject_ref: String,
    expression_ref: String,
    target_ref: String,
    at_unix_ms: u64,
}

pub(super) fn command(bytes: &[u8]) -> Result<String, CliError> {
    let request: Request = serde_json::from_slice(bytes).map_err(|e| CliError(e.to_string()))?;
    if request.schema != "ql.nara-presence-consent-request/v1" {
        return Err(CliError("unsupported Nara presence consent request".into()));
    }
    request.consent.validate().map_err(CliError)?;
    let permitted = request.consent.permits(
        &request.subject_ref,
        &request.expression_ref,
        &request.target_ref,
        request.at_unix_ms,
    );
    serde_json::to_string_pretty(&json!({
        "schema":"ql.nara-presence-consent-reading/v1", "permitted":permitted,
        "consent_ref":request.consent.consent_ref, "subject_ref":request.subject_ref,
        "expression_ref":request.expression_ref,"target_ref":request.target_ref,
        "at_unix_ms":request.at_unix_ms,
        "law":"ql_mef::nara::expression::SharedPresenceConsent",
        "standing":"native consent scope validation; authenticated consent admission and persistence belong to the host",
        "private_state_exported":false
    })).map_err(|e| CliError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_consent_scope_and_time_boundaries() {
        let base = json!({"schema":"ql.nara-presence-consent-request/v1",
            "consent":{"consent_ref":{"ref_id":"consent:two-worlds","revision":"1","owner_ref":"participant:one"},
                "participant_subjects":["participant:one","participant:two"],
                "allowed_expression_refs":["expression:one"],"allowed_target_refs":["participant:two"],
                "granted_at_unix_ms":100,"expires_at_unix_ms":Some(200)},
            "subject_ref":"participant:one","expression_ref":"expression:one","target_ref":"participant:two","at_unix_ms":100});
        let read = |v: &serde_json::Value| {
            serde_json::from_str::<serde_json::Value>(
                &command(&serde_json::to_vec(v).unwrap()).unwrap(),
            )
            .unwrap()
        };
        assert_eq!(read(&base)["permitted"], true);
        for (key, value) in [
            ("subject_ref", json!("participant:other")),
            ("expression_ref", json!("expression:other")),
            ("target_ref", json!("participant:other")),
            ("at_unix_ms", json!(99)),
            ("at_unix_ms", json!(200)),
        ] {
            let mut changed = base.clone();
            changed[key] = value;
            assert_eq!(read(&changed)["permitted"], false);
        }
        let mut malformed = base;
        malformed["consent"]["participant_subjects"] = json!(["participant:one"]);
        assert!(command(&serde_json::to_vec(&malformed).unwrap()).is_err());
    }
}
