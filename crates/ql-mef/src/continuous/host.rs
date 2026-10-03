//! Bounded local control of the existing coupled owner. Pipe access is supplied
//! by the native host; a subject/reference is not a grant of authority.
use super::coupled::{CoupledFieldSession, CoupledInput};
use super::performance::{PerformanceCommand, PerformanceConfig, PerformanceOwner};
use super::performance_receiving::NativePerformanceReceivingSource;
use super::scene_field::{self, SceneConfig, SceneInstrument};
use super::{FieldInput, LiftInput};
use crate::musical_performance_return::ReturnContext;
use crate::scene::{WorldRequest, world};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

pub const WORLD_HOST_CONFIG: &str = "ql.field-host-world-config/v1";
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

/// Native World factory input retained under the existing host lease. The
/// source constructor independently regenerates its complete scene config;
/// a supplied scene/body/witness is deliberately absent from this contract.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldHostConfig {
    pub schema: String,
    pub instance_ref: String,
    pub world_request: WorldRequest,
    pub receiving_context: ReturnContext,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HostOperation {
    PerformancePrepare {
        config: Box<PerformanceConfig>,
    },
    PerformanceExchange {
        command: Box<PerformanceCommand>,
    },
    Read {},
    Inspect {},
    Advance {
        frames: u32,
        muted: bool,
    },
    SetAxis {
        axis: u8,
        phase: LiftInput,
    },
    Replace {
        basis: Box<CoupledInput>,
    },
    /// scene only: D30 material policy; resident amplitudes and clocks continue.
    SetDamping {
        per_second: f64,
    },
    /// scene only: M1's own advance action, then the whole event is re-read.
    M1Advance {
        ticks: u64,
    },
    /// scene only: a changed caller event; `strike` re-excites the voices explicitly.
    ReplaceEvent {
        event: Box<CoupledInput>,
        strike: bool,
    },
    /// scene only: the acting influence reading with its basis and warrant.
    Influence {},
    /// scene with a Nara constitution: receive supplied seven-centre inputs
    /// against the current event. The material field does not change.
    ReceivePersonal {
        input: Box<crate::nara::PersonalEventInput>,
    },
    /// scene with a Nara constitution: the last reception and its currentness.
    Personal {},
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

/// The one owner a host holds: a fully supplied coupled field, or the scene
/// instrument whose geometry and voices the native provider composes.
enum Owner {
    Supplied(Box<CoupledFieldSession>),
    Scene(Box<SceneInstrument>),
}
impl Owner {
    fn session(&self) -> &CoupledFieldSession {
        match self {
            Self::Supplied(session) => session,
            Self::Scene(instrument) => instrument.session(),
        }
    }
    fn session_mut(&mut self) -> &mut CoupledFieldSession {
        match self {
            Self::Supplied(session) => session,
            Self::Scene(instrument) => instrument.session_mut(),
        }
    }
}

/// A single supplied instance, containing the full original/current engines.
/// Reads and inspection neither advance the field nor create another worker.
pub struct FieldHost {
    instance_ref: String,
    session: Owner,
    last_request: u64,
    performance: Option<PerformanceOwner>,
    receiving_source: Option<NativePerformanceReceivingSource>,
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
            session: Owner::Supplied(Box::new(CoupledFieldSession::open(
                worker,
                config.basis,
                config.field,
                timeout,
            )?)),
            last_request: 0,
            performance: None,
            receiving_source: None,
        })
    }

    pub fn open_scene(
        worker: &Path,
        config: SceneConfig,
        timeout: Duration,
    ) -> Result<Self, String> {
        let instrument = SceneInstrument::open(worker, config, timeout)?;
        Ok(Self {
            instance_ref: instrument.instance_ref().to_owned(),
            session: Owner::Scene(Box::new(instrument)),
            last_request: 0,
            performance: None,
            receiving_source: None,
        })
    }

    /// The existing native Factory opens this under its actual World lease.
    /// Reconstruct before starting the existing worker; original sky/private
    /// source classification and native World completion are both checked.
    pub fn open_world(
        worker: &Path,
        config: WorldHostConfig,
        timeout: Duration,
    ) -> Result<Self, String> {
        if config.schema != WORLD_HOST_CONFIG
            || config.instance_ref != config.world_request.instance_ref
        {
            return Err("native World host source/instance differs".into());
        }
        let source = NativePerformanceReceivingSource::world_source(
            config.world_request.clone(),
            config.receiving_context,
        )?;
        let produced = world(config.world_request)?;
        let scene: SceneConfig = serde_json::from_value(produced["binding"]["host"].clone())
            .map_err(|e| e.to_string())?;
        let mut host = Self::open_scene(worker, scene, timeout)?;
        host.bind_performance_receiving_source(source)?;
        Ok(host)
    }
    /// Protected N owner calls this with its privately constructed actual
    /// profile/occasion/consent source while holding the current Act/lease.
    /// There is no HostOperation accepting a browser witness or definition.
    pub fn bind_performance_receiving_source(
        &mut self,
        source: NativePerformanceReceivingSource,
    ) -> Result<(), String> {
        if self.performance.is_some() {
            return Err(
                "receiving replacement requires an atomic retained source/body transaction".into(),
            );
        }
        if !self.available() {
            return Err("native current owner/lease unavailable".into());
        }
        self.receiving_source = Some(source);
        Ok(())
    }

    /// Chooses the owner from the configuration's own contract.
    pub fn open_config(worker: &Path, bytes: &[u8], timeout: Duration) -> Result<Self, String> {
        let value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if value.get("schema").and_then(Value::as_str) == Some(WORLD_HOST_CONFIG) {
            let config: WorldHostConfig =
                serde_json::from_value(value).map_err(|e| e.to_string())?;
            return Self::open_world(worker, config, timeout);
        }
        if value.get("schema").and_then(Value::as_str) == Some(scene_field::CONFIG) {
            let config: SceneConfig = serde_json::from_value(value).map_err(|e| e.to_string())?;
            return Self::open_scene(worker, config, timeout);
        }
        let config: HostConfig = serde_json::from_value(value).map_err(|e| e.to_string())?;
        Self::open(worker, config, timeout)
    }

    pub fn available(&self) -> bool {
        self.session.session().available()
    }

    fn response(&self, request_id: Option<&str>, status: &str, error: Option<&str>) -> Value {
        // This is the LAST ACKNOWLEDGED field, not a claim of live state after
        // transport loss. Full original/current sources are only sent on Inspect.
        let mut field = self.session.session().last_field().clone();
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

    /// Native C/Act retention consumes the actual activated owner, not a
    /// preparation-only source bundle assembled beside it. This is an immutable
    /// source artifact, not a device/output or checkpoint admission.
    pub fn retained_performance_source_artifact(
        &self,
        declared_seed: u64,
    ) -> Result<Value, String> {
        let owner = self
            .performance
            .as_ref()
            .ok_or("actual retained performance is not active")?;
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving source is not bound")?;
        let current = self.session.session().current_basis();
        if !self.available() || owner.reading().is_none() {
            return Err("native activated owner/current readback unavailable".into());
        }
        // Full original/current producer, receiver, occasion and consent replay
        // precedes retention. A stored snapshot or binding-only setter is not it.
        let prepared_receiving = source.prepare_current(owner, current, 0)?;
        if prepared_receiving.snapshot()? != owner.source_assets()["current_receiving"] {
            return Err(
                "activated native source artifact lost exact current receiving owner".into(),
            );
        }
        let returned = crate::musical_performance_return::bind_performance_return(
            owner.binding(),
            source.original_occasion().cloned(),
            source.return_context().clone(),
            declared_seed,
        )?;
        Ok(json!({"schema":"ql.retained-source-performance-fixture/v1",
            "basis":returned.expression_basis()?,"pitches":returned.expression_pitches(0)?,
            "source_assets":owner.source_assets(),"native_preparation":owner.native_packet()?,
            "native_basis":owner.binding().native_basis(),"native_reading":owner.reading(),
            "standing":"actual activated existing FieldHost/PerformanceOwner/worker; source artifact only, no hardware output or file/Act acceptance"}))
    }

    /// Bad JSON/unknown fields have no admitted sequence and never reach C++.
    pub fn reject_input(&self, error: &str) -> Value {
        self.response(None, "refused", Some(error))
    }

    fn admit(&mut self, request: &HostRequest) -> Result<(), String> {
        let field = self.session.session().last_field();
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
        let performance_exchange =
            matches!(request.command, HostOperation::PerformanceExchange { .. });
        if !performance_exchange
            && (field["generation"] != request.expected_generation
                || field["samples_elapsed"] != request.expected_samples_elapsed)
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
        if matches!(
            &request.command,
            HostOperation::PerformancePrepare { .. } | HostOperation::PerformanceExchange { .. }
        ) {
            let current = self.session.session().current_basis().clone();
            let result = match request.command {
                HostOperation::PerformancePrepare { config } => {
                    if self.performance.is_some() {
                        Err("retained native performance already owns this work".into())
                    } else {
                        PerformanceOwner::prepare(&current, &self.instance_ref, *config).and_then(
                            |mut performance| {
                                let reply = match &self.receiving_source {
                                    Some(source) => performance.activate_with_current_receiving(
                                        &current,
                                        self.session.session_mut(),
                                        source,
                                    )?,
                                    None => performance
                                        .activate(&current, self.session.session_mut())?,
                                };
                                self.performance = Some(performance);
                                Ok(reply)
                            },
                        )
                    }
                }
                HostOperation::PerformanceExchange { command } => match self.performance.as_mut() {
                    Some(performance) => {
                        performance.execute(&current, self.session.session_mut(), *command)
                    }
                    None => Err("retained native performance has not been prepared".into()),
                },
                _ => unreachable!(),
            };
            return match result {
                Ok(performance) => {
                    let mut response = self.response(Some(&request.request_id), "ok", None);
                    response["performance"] = performance;
                    response
                }
                Err(error) => self.response(
                    Some(&request.request_id),
                    if self.available() {
                        "refused"
                    } else {
                        "unavailable"
                    },
                    Some(&error),
                ),
            };
        }
        if self.performance.is_some()
            && !matches!(
                request.command,
                HostOperation::Inspect {}
                    | HostOperation::Influence {}
                    | HostOperation::Personal {}
            )
        {
            return self.response(Some(&request.request_id),"refused",Some("retained A/P owns native time/body; prepared source/material transaction required"));
        }
        if matches!(&request.command, HostOperation::Inspect { .. }) {
            let mut response = self.response(Some(&request.request_id), "ok", None);
            let session = self.session.session();
            if let Some(performance) = &self.performance {
                response["performance_reading"] =
                    performance.reading().cloned().unwrap_or(Value::Null);
                response["performance_sources"] = performance.source_assets().clone();
            }
            response["sources"] = json!({"original":session.original_basis(),
                "current":session.current_basis(), "original_field":session.original_field()});
            if let Owner::Scene(instrument) = &self.session {
                response["influence"] = instrument.influence();
                response["event"] = json!(instrument.event());
            }
            return response;
        }
        if matches!(
            &request.command,
            HostOperation::ReceivePersonal { .. } | HostOperation::Personal {}
        ) {
            let Owner::Scene(instrument) = &mut self.session else {
                return self.response(
                    Some(&request.request_id),
                    "refused",
                    Some("personal reception belongs to a scene owner with a Nara constitution"),
                );
            };
            let result = match request.command {
                HostOperation::ReceivePersonal { input } => instrument
                    .receive_personal(*input)
                    .and_then(|_| instrument.personal_reading()),
                _ => instrument.personal_reading(),
            };
            return match result {
                Ok(personal) => {
                    let mut response = self.response(Some(&request.request_id), "ok", None);
                    response["personal"] = personal;
                    response
                }
                Err(error) => self.response(Some(&request.request_id), "refused", Some(&error)),
            };
        }
        if matches!(&request.command, HostOperation::Influence {}) {
            let mut response = self.response(Some(&request.request_id), "ok", None);
            match &self.session {
                Owner::Scene(instrument) => response["influence"] = instrument.influence(),
                Owner::Supplied(_) => {
                    return self.response(
                        Some(&request.request_id),
                        "refused",
                        Some("influence reading belongs to a provider-composed scene owner"),
                    );
                }
            }
            return response;
        }
        // A scene determinant event answers with its new influence reading, so a
        // consumer needs no second exchange while its audio waits.
        let determinant = matches!(
            &request.command,
            HostOperation::M1Advance { .. }
                | HostOperation::ReplaceEvent { .. }
                | HostOperation::SetDamping { .. }
        );
        let result = match (request.command, &mut self.session) {
            (HostOperation::Read {}, owner) => owner.session_mut().read_field(),
            (HostOperation::Advance { frames, muted }, owner) => {
                if frames > 8192 {
                    Err("native block ceiling exceeded".into())
                } else {
                    owner.session_mut().advance_field(frames, muted)
                }
            }
            (HostOperation::SetAxis { axis, phase }, owner) => {
                if axis > 1 {
                    Err("unknown independent clock axis".into())
                } else {
                    owner.session_mut().set_axis_field(axis, phase)
                }
            }
            (HostOperation::Replace { basis }, Owner::Supplied(session)) => {
                session.replace_field(*basis)
            }
            (HostOperation::SetDamping { per_second }, Owner::Scene(instrument)) => {
                instrument.set_damping(per_second)
            }
            (HostOperation::M1Advance { ticks }, Owner::Scene(instrument)) => {
                instrument.m1_advance(ticks)
            }
            (HostOperation::ReplaceEvent { event, strike }, Owner::Scene(instrument)) => {
                instrument.replace(&event, strike)
            }
            (HostOperation::Replace { .. }, Owner::Scene(_)) => {
                Err("a scene owner composes its own voices; use replace-event".into())
            }
            (
                HostOperation::M1Advance { .. }
                | HostOperation::ReplaceEvent { .. }
                | HostOperation::SetDamping { .. },
                _,
            ) => Err("determinant operations belong to a provider-composed scene owner".into()),
            (
                HostOperation::PerformancePrepare { .. }
                | HostOperation::PerformanceExchange { .. }
                | HostOperation::Inspect {}
                | HostOperation::Influence {}
                | HostOperation::ReceivePersonal { .. }
                | HostOperation::Personal {},
                _,
            ) => unreachable!("reads and reception returned before dispatch"),
        };
        match result {
            Ok(field) => {
                let mut response = self.response(Some(&request.request_id), "ok", None);
                response["field"] = field;
                if let (true, Owner::Scene(instrument)) = (determinant, &self.session) {
                    response["influence"] = instrument.influence();
                }
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
