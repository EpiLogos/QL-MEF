//! Bounded stdio owner for K9's focused consumer over the one K8/Nara session.
//!
//! This process does not start another numerical field, graph store, renderer or
//! agent loop. It owns encounter-local focus/selection/presentation state and
//! delegates all field/personal operations to `PersonalCoupledSession`. Bimba
//! entries are resolved once through the accepted AW1/native M registry.

use ql_mef::aw1_world::{RootedMWorld, resolve_rooted_m_world};
use ql_mef::continuous::coupled::CoupledInput;
use ql_mef::continuous::host::{MAX_HOST_INPUT, MAX_HOST_OUTPUT};
use ql_mef::continuous::personal::PersonalCoupledSession;
use ql_mef::continuous::{FieldInput, LiftInput};
use ql_mef::focused_instrument::{
    BimbaSelection, ClockPresentation, FocusedInstrument, FocusedInstrumentSnapshot,
    InstrumentFocus, InstrumentOwnerView, OperationStanding, SelectionTracking,
    VakExpressionBinding,
};
use ql_mef::m_tree::native_m_registry;
use ql_mef::nara::{PersonalConstitution, PersonalEventInput};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::time::Duration;

const CONFIG_CONTRACT: &str = "ql.focused-host-config/v1";
const REQUEST_CONTRACT: &str = "ql.focused-host-request/v1";
const RECEIPT_CONTRACT: &str = "ql.focused-host-receipt/v1";
const BIMBA_NAVIGATION_CONTRACT: &str = "ql.focused-host-bimba-navigation/v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BimbaHostEntry {
    source_ref: String,
    selection_ref: String,
    disclosure_ref: String,
    label: String,
    field_constituent_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FocusedHostConfig {
    schema: String,
    instance_ref: String,
    basis: CoupledInput,
    field: FieldInput,
    constitution: PersonalConstitution,
    #[serde(default)]
    bimba: Vec<BimbaHostEntry>,
}

#[derive(Debug, Clone)]
struct BimbaResolved {
    config: BimbaHostEntry,
    world: RootedMWorld,
    selection: BimbaSelection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
enum FocusedHostOperation {
    Read {},
    Bimba {},
    Inspect {},
    SetFocus { focus: InstrumentFocus },
    SelectBimba { selection_ref: String },
    ClearSelection {},
    SetTracking { tracking: SelectionTracking },
    Freeze {},
    ResumeLive {},
    AssembleClock {},
    ExplodeClock { pair: Option<u8> },
    SetClockAxis { axis: u8, phase: LiftInput },
    Advance { frames: u32, muted: bool },
    ReceivePersonal { input: Box<PersonalEventInput> },
    Replace { basis: Box<CoupledInput> },
    BindVakExpression { binding: Box<VakExpressionBinding> },
    ClearVakExpression {},
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct FocusedHostRequest {
    schema: String,
    instance_ref: String,
    event_ref: String,
    subject_ref: String,
    profile_generation: u64,
    request_id: String,
    expected_generation: String,
    expected_samples_elapsed: String,
    command: FocusedHostOperation,
}

fn exact_cursor(text: &str) -> Result<u64, String> {
    let value: u64 = text.parse().map_err(|_| "invalid focused-host cursor")?;
    if value.to_string() != text {
        return Err("noncanonical focused-host cursor".into());
    }
    Ok(value)
}

struct FocusedHost {
    instance_ref: String,
    session: PersonalCoupledSession,
    instrument: FocusedInstrument,
    bimba: BTreeMap<String, BimbaResolved>,
    last_request: u64,
}

impl FocusedHost {
    fn open(worker: &Path, config: FocusedHostConfig, timeout: Duration) -> Result<Self, String> {
        if config.schema != CONFIG_CONTRACT {
            return Err("unsupported focused-host configuration".into());
        }
        if config.instance_ref.is_empty()
            || config.instance_ref.len() > 2048
            || config.instance_ref.chars().any(char::is_control)
        {
            return Err("invalid focused-host instance reference".into());
        }
        let subject = config.constitution.subject_id.clone();
        let session = PersonalCoupledSession::open(
            worker,
            config.basis,
            config.field,
            config.constitution,
            timeout,
        )?;
        let registry = native_m_registry();
        let mut bimba = BTreeMap::new();
        for item in config.bimba {
            if item.selection_ref.trim().is_empty()
                || item.disclosure_ref.trim().is_empty()
                || item.label.trim().is_empty()
            {
                return Err("focused-host Bimba entry needs selection/disclosure/label".into());
            }
            if bimba.contains_key(&item.selection_ref) {
                return Err("duplicate focused-host Bimba selection reference".into());
            }
            let world = resolve_rooted_m_world(registry, &item.source_ref)?;
            let selection = BimbaSelection::from_rooted_world(
                &world,
                item.selection_ref.clone(),
                item.disclosure_ref.clone(),
                subject.clone(),
                item.field_constituent_ref.clone(),
            )?;
            bimba.insert(
                item.selection_ref.clone(),
                BimbaResolved {
                    config: item,
                    world,
                    selection,
                },
            );
        }
        Ok(Self {
            instance_ref: config.instance_ref,
            session,
            instrument: FocusedInstrument::new(),
            bimba,
            last_request: 0,
        })
    }

    fn available(&self) -> bool {
        self.session.available()
    }

    fn view(&self) -> Result<InstrumentOwnerView, String> {
        InstrumentOwnerView::from_session(&self.session)
    }

    fn snapshot(&mut self) -> Result<FocusedInstrumentSnapshot, String> {
        let view = self.view()?;
        self.instrument.snapshot(&view)
    }

    fn navigation(&mut self) -> Result<Value, String> {
        let snapshot = self.snapshot()?;
        let selected_ref = snapshot.selection.as_ref().map(|item| item.selection_ref.clone());
        let items = self
            .bimba
            .values()
            .map(|entry| {
                let parent_ref = entry
                    .world
                    .ancestry
                    .iter()
                    .rev()
                    .nth(1)
                    .map(|step| step.source_ref.clone());
                json!({
                    "selection": entry.selection,
                    "label": entry.config.label,
                    "face": "bimba",
                    "parent_ref": parent_ref,
                    "depth": entry.world.ancestry.last().map(|step| step.depth),
                    "relation_refs": entry.selection.assertion_refs,
                })
            })
            .collect::<Vec<_>>();
        let source_revision = self
            .bimba
            .values()
            .next()
            .map(|entry| entry.world.registry_source_revision.clone())
            .unwrap_or_else(|| native_m_registry().manifest().source_revision.clone());
        Ok(json!({
            "contract": BIMBA_NAVIGATION_CONTRACT,
            "source_revision": source_revision,
            "selected_ref": selected_ref,
            "items": items,
            "standing": "AW1/native-M rooted Bimba navigation; K9 retains source identity and owns only encounter selection",
        }))
    }

    fn receipt(
        &mut self,
        request_id: Option<&str>,
        status: &str,
        operation: &str,
        error: Option<&str>,
        owner_receipt: Option<Value>,
        include_bimba: bool,
        include_inspection: bool,
    ) -> Value {
        let snapshot = self.snapshot();
        let mut value = json!({
            "schema": RECEIPT_CONTRACT,
            "instance_ref": self.instance_ref,
            "request_id": request_id,
            "last_request_id": self.last_request.to_string(),
            "status": status,
            "operation": operation,
            "available": self.available(),
            "error": error,
            "snapshot": snapshot.as_ref().ok(),
            "owner_receipt": owner_receipt,
            "standing": "single K8/Nara owner with K9 focus/selection consumer state; no automatic retry after unknown transport",
        });
        if let Err(snapshot_error) = snapshot {
            value["status"] = json!(if self.available() { "refused" } else { "unavailable" });
            value["error"] = json!(snapshot_error);
        }
        if include_bimba {
            match self.navigation() {
                Ok(navigation) => value["bimba"] = navigation,
                Err(navigation_error) => {
                    value["status"] = json!("refused");
                    value["error"] = json!(navigation_error);
                }
            }
        }
        if include_inspection {
            value["inspection"] = self.session.inspect().unwrap_or_else(|error| json!({"error":error}));
        }
        value
    }

    fn ready(&mut self) -> Value {
        self.receipt(None, "ready", "ready", None, None, true, false)
    }

    fn reject_input(&mut self, error: &str) -> Value {
        self.receipt(None, "refused", "parse", Some(error), None, false, false)
    }

    fn admit(&mut self, request: &FocusedHostRequest) -> Result<(), String> {
        let snapshot = self.snapshot()?;
        if request.schema != REQUEST_CONTRACT
            || request.instance_ref != self.instance_ref
            || request.event_ref != snapshot.event.event_ref
            || request.subject_ref != snapshot.event.subject_ref
            || request.profile_generation != snapshot.event.profile_generation
        {
            return Err("focused-host request has foreign schema/instance/event/subject/world".into());
        }
        let sequence = exact_cursor(&request.request_id)?;
        if self.last_request.checked_add(1) != Some(sequence) {
            return Err("stale, repeated or skipped focused-host request sequence".into());
        }
        self.last_request = sequence;
        exact_cursor(&request.expected_generation)?;
        exact_cursor(&request.expected_samples_elapsed)?;
        if request.expected_generation != snapshot.live_cursor.field_generation
            || request.expected_samples_elapsed != snapshot.live_cursor.samples_elapsed
        {
            return Err("focused-host request is based on a stale native cursor".into());
        }
        if !self.available() {
            return Err("native transport standing unknown; explicit new owner required".into());
        }
        Ok(())
    }

    fn execute(&mut self, request: FocusedHostRequest) -> Value {
        let request_id = request.request_id.clone();
        if let Err(error) = self.admit(&request) {
            let status = if self.available() { "refused" } else { "unavailable" };
            return self.receipt(
                Some(&request_id),
                status,
                "admission",
                Some(&error),
                None,
                false,
                false,
            );
        }
        let operation_name = match &request.command {
            FocusedHostOperation::Read {} => "read",
            FocusedHostOperation::Bimba {} => "bimba",
            FocusedHostOperation::Inspect {} => "inspect",
            FocusedHostOperation::SetFocus { .. } => "set-focus",
            FocusedHostOperation::SelectBimba { .. } => "select-bimba",
            FocusedHostOperation::ClearSelection {} => "clear-selection",
            FocusedHostOperation::SetTracking { .. } => "set-tracking",
            FocusedHostOperation::Freeze {} => "freeze",
            FocusedHostOperation::ResumeLive {} => "resume-live",
            FocusedHostOperation::AssembleClock {} => "assemble-clock",
            FocusedHostOperation::ExplodeClock { .. } => "explode-clock",
            FocusedHostOperation::SetClockAxis { .. } => "set-clock-axis",
            FocusedHostOperation::Advance { .. } => "advance",
            FocusedHostOperation::ReceivePersonal { .. } => "receive-personal",
            FocusedHostOperation::Replace { .. } => "replace",
            FocusedHostOperation::BindVakExpression { .. } => "bind-vak-expression",
            FocusedHostOperation::ClearVakExpression {} => "clear-vak-expression",
        };
        let mut owner_receipt = None;
        let mut status = "ok";
        let mut error = None;
        let result: Result<(), String> = match request.command {
            FocusedHostOperation::Read {} | FocusedHostOperation::Bimba {} | FocusedHostOperation::Inspect {} => Ok(()),
            FocusedHostOperation::SetFocus { focus } => {
                self.instrument.set_focus(focus);
                Ok(())
            }
            FocusedHostOperation::SelectBimba { selection_ref } => {
                let selection = self
                    .bimba
                    .get(&selection_ref)
                    .map(|entry| entry.selection.clone())
                    .ok_or_else(|| "unknown admitted Bimba selection".to_string());
                selection.and_then(|selection| {
                    let view = self.view()?;
                    self.instrument.select_bimba(&view, selection)
                })
            }
            FocusedHostOperation::ClearSelection {} => {
                self.instrument.clear_selection();
                Ok(())
            }
            FocusedHostOperation::SetTracking { tracking } => {
                self.instrument.set_tracking(tracking);
                Ok(())
            }
            FocusedHostOperation::Freeze {} => {
                let view = self.view()?;
                self.instrument.freeze(&view)
            }
            FocusedHostOperation::ResumeLive {} => {
                self.instrument.resume_live();
                Ok(())
            }
            FocusedHostOperation::AssembleClock {} => {
                self.instrument.assemble_clock();
                Ok(())
            }
            FocusedHostOperation::ExplodeClock { pair } => self.instrument.explode_clock(pair),
            FocusedHostOperation::SetClockAxis { axis, phase } => {
                let observation = self.instrument.set_clock_axis(&mut self.session, axis, phase);
                status = match observation.standing {
                    OperationStanding::Applied => "ok",
                    OperationStanding::Refused => "refused",
                    OperationStanding::Unknown => "unavailable",
                };
                error = observation.error.clone();
                owner_receipt = observation.owner_receipt.clone();
                Ok(())
            }
            FocusedHostOperation::Advance { frames, muted } => {
                if frames > 8192 {
                    Err("native block ceiling exceeded".into())
                } else {
                    let observation = self.instrument.advance(&mut self.session, frames, muted);
                    status = match observation.standing {
                        OperationStanding::Applied => "ok",
                        OperationStanding::Refused => "refused",
                        OperationStanding::Unknown => "unavailable",
                    };
                    error = observation.error.clone();
                    owner_receipt = observation.owner_receipt.clone();
                    Ok(())
                }
            }
            FocusedHostOperation::ReceivePersonal { input } => self
                .session
                .receive_personal(*input)
                .map(|state| owner_receipt = serde_json::to_value(state).ok()),
            FocusedHostOperation::Replace { basis } => self
                .session
                .replace_field(*basis)
                .map(|receipt| owner_receipt = Some(receipt)),
            FocusedHostOperation::BindVakExpression { binding } => {
                let view = self.view()?;
                self.instrument.bind_vak_expression(&view, *binding)
            }
            FocusedHostOperation::ClearVakExpression {} => {
                self.instrument.clear_vak_expression();
                Ok(())
            }
        };
        if let Err(command_error) = result {
            status = if self.available() { "refused" } else { "unavailable" };
            error = Some(command_error);
        }
        self.receipt(
            Some(&request_id),
            status,
            operation_name,
            error.as_deref(),
            owner_receipt,
            matches!(operation_name, "bimba" | "inspect"),
            operation_name == "inspect",
        )
    }
}

fn send(output: &mut impl Write, value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    if bytes.len() > MAX_HOST_OUTPUT {
        return Err("focused-host output ceiling exceeded; connection closed without retry".into());
    }
    output.write_all(&bytes).map_err(|error| error.to_string())?;
    output.write_all(b"\n").map_err(|error| error.to_string())?;
    output.flush().map_err(|error| error.to_string())
}

fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: ql-focused-host WORKER FOCUSED_HOST_CONFIG_JSON".into());
    }
    let mut bytes = Vec::new();
    File::open(&args[2])
        .map_err(|error| error.to_string())?
        .take(MAX_HOST_INPUT + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > MAX_HOST_INPUT {
        return Err("focused-host configuration ceiling exceeded".into());
    }
    let config: FocusedHostConfig = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    let mut host = FocusedHost::open(Path::new(&args[1]), config, Duration::from_secs(5))?;
    let input = io::stdin();
    let mut input = input.lock();
    let output = io::stdout();
    let mut output = output.lock();
    send(&mut output, &host.ready())?;
    loop {
        let mut line = Vec::new();
        let count = input
            .by_ref()
            .take(MAX_HOST_INPUT + 1)
            .read_until(b'\n', &mut line)
            .map_err(|error| error.to_string())?;
        if count == 0 {
            return Ok(());
        }
        if count as u64 > MAX_HOST_INPUT || !line.ends_with(b"\n") {
            return Err("focused-host input is oversized or unterminated; closing owner".into());
        }
        let response = match serde_json::from_slice::<FocusedHostRequest>(&line) {
            Ok(request) => host.execute(request),
            Err(_) => host.reject_input("malformed or unknown focused-host request fields"),
        };
        send(&mut output, &response)?;
        if !host.available() {
            return Err("native acknowledgement unavailable; focused host closed without retry".into());
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ql-focused-host: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focused_host_command_contract_is_closed_and_clock_pair_is_not_a_domain_alias() {
        for value in [
            json!({"operation":"set-focus","focus":"m0"}),
            json!({"operation":"advance","frames":1}),
            json!({"operation":"explode-clock","pair":8,"extra":true}),
            json!({"operation":"shutdown"}),
        ] {
            assert!(serde_json::from_value::<FocusedHostOperation>(value).is_err());
        }
        let command: FocusedHostOperation =
            serde_json::from_value(json!({"operation":"explode-clock","pair":3})).unwrap();
        assert!(matches!(command, FocusedHostOperation::ExplodeClock { pair: Some(3) }));
        let presentation = ClockPresentation::Exploded { pair: Some(3) };
        assert!(matches!(presentation, ClockPresentation::Exploded { .. }));
    }

    #[test]
    fn focused_host_cursors_reject_aliases_and_overflow() {
        for value in ["", "01", "+1", "-0", " 1", "1e3", "18446744073709551616"] {
            assert!(exact_cursor(value).is_err());
        }
        assert_eq!(exact_cursor("18446744073709551615").unwrap(), u64::MAX);
    }
}
