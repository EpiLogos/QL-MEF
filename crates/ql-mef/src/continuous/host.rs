//! Bounded local control of the existing coupled owner. Pipe access is supplied
//! by the native host; a subject/reference is not a grant of authority.
use super::coupled::{CoupledFieldSession, CoupledInput};
use super::{FieldInput, LiftInput};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

pub const HOST_REQUEST: &str = "ql.field-host-request/v1";
pub const HOST_RECEIPT: &str = "ql.field-host-receipt/v1";
pub const MAX_HOST_INPUT: u64 = 32 * 1024 * 1024;
pub const MAX_HOST_OUTPUT: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostConfig {
    pub instance_ref: String,
    pub basis: CoupledInput,
    pub field: FieldInput,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HostOperation {
    Read {},
    Inspect {},
    Advance { frames: u32, muted: bool },
    SetAxis { axis: u8, phase: LiftInput },
    Replace { basis: Box<CoupledInput> },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostRequest {
    pub schema: String,
    pub instance_ref: String,
    pub event_ref: String,
    pub subject_ref: String,
    pub request_id: String,
    pub expected_generation: String,
    pub expected_samples_elapsed: String,
    pub command: HostOperation,
}

fn exact_cursor(text: &str) -> Result<u64, String> {
    let value: u64 = text.parse().map_err(|_| "invalid host cursor")?;
    if value.to_string() != text {
        return Err("noncanonical host cursor".into());
    }
    Ok(value)
}

/// A single supplied instance, containing the full original/current engines.
/// Reads and inspection neither advance the field nor create another worker.
pub struct FieldHost {
    instance_ref: String,
    session: CoupledFieldSession,
    last_request: u64,
}
impl FieldHost {
    pub fn open(worker: &Path, config: HostConfig, timeout: Duration) -> Result<Self, String> {
        if config.instance_ref.is_empty()
            || config.instance_ref.len() > 2048
            || config.instance_ref.chars().any(char::is_control)
        {
            return Err("invalid native host instance reference".into());
        }
        Ok(Self {
            instance_ref: config.instance_ref,
            session: CoupledFieldSession::open(worker, config.basis, config.field, timeout)?,
            last_request: 0,
        })
    }

    pub fn available(&self) -> bool {
        self.session.available()
    }

    fn response(&self, request_id: Option<&str>, status: &str, error: Option<&str>) -> Value {
        // This is the LAST ACKNOWLEDGED field, not a claim of live state after
        // transport loss. Full original/current sources are only sent on Inspect.
        let mut field = self.session.last_field().clone();
        field["audio"] = json!([]);
        json!({"schema":HOST_RECEIPT, "instance_ref":self.instance_ref,
            "request_id":request_id, "last_request_id":self.last_request.to_string(),
            "status":status, "available":self.available(), "error":error,
            "field":field,
            "standing":"local single-owner control; caller-supplied pipe authority; last acknowledged native state"})
    }

    pub fn ready(&self) -> Value {
        self.response(None, "ready", None)
    }

    /// Bad JSON/unknown fields have no admitted sequence and never reach C++.
    pub fn reject_input(&self, error: &str) -> Value {
        self.response(None, "refused", Some(error))
    }

    fn admit(&mut self, request: &HostRequest) -> Result<(), String> {
        let field = self.session.last_field();
        if request.schema != HOST_REQUEST
            || request.instance_ref != self.instance_ref
            || field["event_ref"] != request.event_ref
            || field["subject_ref"] != request.subject_ref
        {
            return Err("host request has a foreign schema/instance/event/subject".into());
        }
        let sequence = exact_cursor(&request.request_id)?;
        if self.last_request.checked_add(1) != Some(sequence) {
            return Err("stale, repeated or skipped host request sequence".into());
        }
        // An admitted scoped envelope consumes its sequence even if its command
        // is refused. A lost acknowledgement must never be retried implicitly.
        self.last_request = sequence;
        exact_cursor(&request.expected_generation)?;
        exact_cursor(&request.expected_samples_elapsed)?;
        if field["generation"] != request.expected_generation
            || field["samples_elapsed"] != request.expected_samples_elapsed
        {
            return Err("host request is based on a stale native cursor".into());
        }
        if !self.available() {
            return Err("native transport standing unknown; explicit new owner required".into());
        }
        Ok(())
    }

    pub fn execute(&mut self, request: HostRequest) -> Value {
        if let Err(error) = self.admit(&request) {
            let status = if self.available() {
                "refused"
            } else {
                "unavailable"
            };
            return self.response(Some(&request.request_id), status, Some(&error));
        }
        if matches!(&request.command, HostOperation::Inspect { .. }) {
            let mut response = self.response(Some(&request.request_id), "ok", None);
            response["sources"] = json!({"original":self.session.original_basis(),
                "current":self.session.current_basis(), "original_field":self.session.original_field()});
            return response;
        }
        let result = match request.command {
            HostOperation::Read {} => self.session.read_field(),
            HostOperation::Advance { frames, muted } => {
                if frames > 8192 {
                    Err("native block ceiling exceeded".into())
                } else {
                    self.session.advance_field(frames, muted)
                }
            }
            HostOperation::SetAxis { axis, phase } => {
                if axis > 1 {
                    Err("unknown independent clock axis".into())
                } else {
                    self.session.set_axis_field(axis, phase)
                }
            }
            HostOperation::Replace { basis } => self.session.replace_field(*basis),
            HostOperation::Inspect {} => unreachable!("inspection returned before dispatch"),
        };
        match result {
            Ok(field) => {
                let mut response = self.response(Some(&request.request_id), "ok", None);
                response["field"] = field;
                response
            }
            Err(error) => {
                let status = if self.available() {
                    "refused"
                } else {
                    "unavailable"
                };
                self.response(Some(&request.request_id), status, Some(&error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_cursors_keep_the_full_u64_range_without_aliases() {
        for text in ["", "01", "+1", "-0", " 1", "1e3", "18446744073709551616"] {
            assert!(exact_cursor(text).is_err());
        }
        assert_eq!(exact_cursor("18446744073709551615").unwrap(), u64::MAX);
    }

    #[test]
    fn command_contract_rejects_unknown_fields_and_implicit_mute() {
        for value in [
            json!({"operation":"read", "advance":true}),
            json!({"operation":"advance", "frames":128}),
            json!({"operation":"advance", "frames":1.5,"muted":false}),
            json!({"operation":"shutdown"}),
        ] {
            assert!(
                serde_json::from_value::<HostOperation>(value.clone()).is_err(),
                "accepted {value}"
            );
        }
        let command: HostOperation = serde_json::from_value(json!({"operation":"read"})).unwrap();
        assert!(matches!(command, HostOperation::Read {}));
    }
}
