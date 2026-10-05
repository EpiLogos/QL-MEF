//! Bounded local control of the existing coupled owner. Pipe access is supplied
//! by the native host; a subject/reference is not a grant of authority.
use super::coupled::{CoupledFieldSession, CoupledInput};
use super::scene_field::{self, SceneConfig, SceneInstrument};
use super::stage::{self, StageOwnership};
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
    /// scene only: the Ta-Onta procedural stage's scoped state disclosure —
    /// the live constituents, their effective values and their owners.
    StageState {},
    /// scene only: evaluate one versioned stage procedure against the current
    /// event and apply its plan through this host's own determinant paths.
    StageEvaluate {
        procedure: Box<stage::StageProcedure>,
    },
    /// scene only: release one procedure's contributions (its own key family
    /// only); retained material stays as the now-authored state.
    StageRetire {
        procedure_ref: String,
    },
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
    stage: StageOwnership,
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
            session: Owner::Supplied(Box::new(CoupledFieldSession::open(
                worker,
                config.basis,
                config.field,
                timeout,
            )?)),
            stage: StageOwnership::default(),
            last_request: 0,
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
            stage: StageOwnership::default(),
            last_request: 0,
        })
    }

    /// Chooses the owner from the configuration's own contract.
    pub fn open_config(worker: &Path, bytes: &[u8], timeout: Duration) -> Result<Self, String> {
        let value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
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

    /// The procedural stage's scoped disclosure: the live constituents, their
    /// current effective standing and the procedure that owns each slot.
    fn stage_state(instrument: &SceneInstrument, ownership: &StageOwnership) -> Value {
        let field = instrument.session().last_field();
        let basis = instrument.session().current_basis();
        let slots: Value = stage::STAGE_SLOTS
            .iter()
            .map(|slot| {
                (
                    (*slot).to_string(),
                    match ownership.owner_of(slot) {
                        Some(contribution) => json!({
                            "owner": contribution.key,
                            "procedure_ref": contribution.procedure_ref,
                            "revision": contribution.revision,
                        }),
                        None => json!({"owner": Value::Null}),
                    },
                )
            })
            .collect();
        json!({
            "schema": stage::STAGE_STATE,
            "instance_ref": instrument.instance_ref(),
            "event_ref": field["event_ref"],
            "subject_ref": field["subject_ref"],
            "generation": field["generation"],
            "samples_elapsed": field["samples_elapsed"],
            "form": {"address": basis.m3["form"]["address"], "pose": basis.m3["form"]["pose"],
                "aperture": basis.m3["aperture"]["index"], "clock_steps": basis.m3["clock"]["steps"]},
            "shape_ref": instrument.shape().shape_ref,
            "material": instrument.material(),
            "slots": slots,
            "standing": "live scene state and procedural ownership; audio is never disclosed here",
        })
    }

    /// Applies one evaluated plan through this host's own determinant paths,
    /// in plan order, stopping at the first refusal with its true standing.
    fn stage_evaluate(
        instrument: &mut SceneInstrument,
        ownership: &mut StageOwnership,
        procedure: stage::StageProcedure,
    ) -> Result<(Value, Value), String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "host clock before the epoch")?
            .as_millis() as u64;
        let live = instrument.event();
        let plan = stage::evaluate(&procedure, &live, ownership, now, now)?;
        let strike = instrument.material().strike_on_event;
        let mut applied: Option<Value> = None;
        let apply = |step: &str, result: Result<Value, String>| -> Result<Value, String> {
            result.map_err(|error| format!("stage apply stopped at {step}: {error}"))
        };
        if let Some(event) = &plan.event {
            applied = Some(apply("form", instrument.replace(event, strike))?);
        }
        if let Some(per_second) = plan.damping {
            applied = Some(apply(
                "material.damping",
                instrument.set_damping(per_second),
            )?);
        }
        for (slot, phase) in &plan.clock {
            let axis = if slot == "clock.inscription" { 0 } else { 1 };
            applied = Some(apply(slot, instrument.set_axis(axis, phase.clone()))?);
        }
        for scene in 1..=plan.passage_ticks {
            applied = Some(apply(
                &format!("passage-scene:{scene}"),
                instrument.m1_advance(1),
            )?);
        }
        let event_ref = applied
            .as_ref()
            .and_then(|field| field["event_ref"].as_str())
            .unwrap_or(&live.m1.event_ref)
            .to_owned();
        Ok((
            stage::receipt(&plan, &event_ref, true),
            applied.unwrap_or_else(|| instrument.session().last_field().clone()),
        ))
    }

    /// Dispatches the stage operations; every failure names its cause and the
    /// envelope has already consumed its sequence.
    fn stage_operation(&mut self, request: &HostRequest) -> Value {
        let refusal = "the Ta-Onta procedural stage belongs to a provider-composed scene owner";
        let outcome = match (&request.command, &mut self.session) {
            (HostOperation::StageState {}, Owner::Scene(instrument)) => {
                Ok((Self::stage_state(instrument, &self.stage), None))
            }
            (HostOperation::StageState {}, _) => Err(refusal.into()),
            (HostOperation::StageEvaluate { procedure }, Owner::Scene(instrument)) => {
                Self::stage_evaluate(instrument, &mut self.stage, (**procedure).clone())
                    .map(|(stage, field)| (stage, Some(field)))
            }
            (HostOperation::StageEvaluate { .. }, _) => Err(refusal.into()),
            (HostOperation::StageRetire { procedure_ref }, _) => {
                let retired = self.stage.retire(procedure_ref);
                Ok((
                    json!({"schema": stage::STAGE_RECEIPT, "procedure_ref": procedure_ref,
                        "applied": true, "retired": retired,
                        "standing": "ownership released; retained material stays as the now-authored state"}),
                    None,
                ))
            }
            (
                HostOperation::Read {}
                | HostOperation::Inspect {}
                | HostOperation::Advance { .. }
                | HostOperation::SetAxis { .. }
                | HostOperation::Replace { .. }
                | HostOperation::SetDamping { .. }
                | HostOperation::M1Advance { .. }
                | HostOperation::ReplaceEvent { .. }
                | HostOperation::Influence {}
                | HostOperation::ReceivePersonal { .. }
                | HostOperation::Personal {},
                _,
            ) => unreachable!("non-stage commands returned before stage dispatch"),
        };
        match outcome {
            Ok((stage, field)) => {
                let mut response = self.response(Some(&request.request_id), "ok", None);
                response["stage"] = stage;
                if let Some(field) = field {
                    response["field"] = field;
                    // A stage determinant answers with its new influence
                    // reading, as the other determinant events do.
                    if let Owner::Scene(instrument) = &self.session {
                        response["influence"] = instrument.influence();
                    }
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
            let session = self.session.session();
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
        if matches!(
            &request.command,
            HostOperation::StageState {}
                | HostOperation::StageEvaluate { .. }
                | HostOperation::StageRetire { .. }
        ) {
            return self.stage_operation(&request);
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
                HostOperation::Inspect {}
                | HostOperation::Influence {}
                | HostOperation::ReceivePersonal { .. }
                | HostOperation::Personal {}
                | HostOperation::StageState {}
                | HostOperation::StageEvaluate { .. }
                | HostOperation::StageRetire { .. },
                _,
            ) => unreachable!("reads, reception and stage operations returned before dispatch"),
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

    #[test]
    fn stage_operations_are_admitted_by_name_and_refuse_unknown_fields() {
        let state: HostOperation =
            serde_json::from_value(json!({"operation":"stage-state"})).unwrap();
        assert!(matches!(state, HostOperation::StageState {}));
        let procedure = json!({
            "schema": stage::STAGE_PROCEDURE,
            "procedure_ref": "ta-onta:stage:e2e",
            "revision": 1,
            "subject_ref": "s",
            "trigger": {"trigger": "invocation"},
            "selector": ["form"],
            "changes": [{"change": "form", "operations": [{"operation": "set-pose", "pose": 2}]}]
        });
        let evaluate: HostOperation = serde_json::from_value(json!({
            "operation":"stage-evaluate", "procedure": procedure}))
        .unwrap();
        assert!(matches!(evaluate, HostOperation::StageEvaluate { .. }));
        let retire: HostOperation =
            serde_json::from_value(json!({"operation":"stage-retire", "procedure_ref":"p"}))
                .unwrap();
        assert!(matches!(retire, HostOperation::StageRetire { .. }));
        for value in [
            json!({"operation":"stage-evaluate"}),
            json!({"operation":"stage-evaluate", "procedure": {}, "extra": true}),
            json!({"operation":"stage-retire"}),
            json!({"operation":"stage-state", "procedure_ref": "p"}),
        ] {
            assert!(
                serde_json::from_value::<HostOperation>(value.clone()).is_err(),
                "accepted {value}"
            );
        }
    }
}
