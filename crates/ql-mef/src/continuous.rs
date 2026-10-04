//! Safe management bridge to the installed C/C++ continuous owner.
//! Native host chooses/authorises the executable and subject. Neither a source
//! reference nor a geometry payload grants authority. No child/JSON/domain work
//! runs on an audio callback; this serial API transfers bounded control batches.
pub mod coupled;
pub mod host;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod native_act_channel;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) mod native_act_diagnostics;
pub mod performance;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod performance_act_bridge;
pub mod performance_export;
pub mod performance_receiving;
pub mod personal;
mod receipt;
pub mod scene_field;
mod worker_malloc_custody;
#[cfg(test)]
mod worker_malloc_custody_tests;

use receipt::ReceiptGuard;

use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::m2_engine::M2Request;

pub const FIELD_CONTRACT: &str = "ql.continuous-field/v1";
const MAX_MESSAGE: usize = 32 * 1024 * 1024;
type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiftInput {
    pub turns: String,
    pub half_degrees: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClockInput {
    pub inscription: LiftInput,
    pub lensing: LiftInput,
    pub grid_origins: [u16; 3],
    pub rate_numerators: [String; 2],
    pub rate_denominator: u32,
    pub rate_remainders: [String; 2],
    pub generation: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUnits {
    pub amplitude: String,
    pub excitation: String,
    pub shape: String,
    pub position: String,
    pub audio: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSample {
    pub identity: u64,
    pub constituent: String,
    pub attachment: u8,
    pub rest_metres: [f64; 3],
    pub mode_shapes: Vec<[f64; 3]>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInput {
    pub subject_ref: String,
    pub sample_rate: u32,
    pub clock: ClockInput,
    pub driver_numerator: u32,
    pub driver_denominator: u32,
    pub units: FieldUnits,
    pub audio_gains: Vec<f64>,
    pub samples: Vec<FieldSample>,
    /// Names the supplied shape basis; absent means the M2 geometry_ref. Omitted
    /// when absent so legacy serialized inputs and their replay stay unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape_ref: Option<String>,
}

struct Exchange {
    request: Vec<u8>,
    reply: SyncSender<Result<Value>>,
    diagnostic_facts: Option<worker_malloc_custody::RequestFacts>,
}
struct Worker {
    child: Child,
    sender: Option<SyncSender<Exchange>>,
    reader: Option<JoinHandle<()>>,
    timeout: Duration,
    poisoned: bool,
    diagnostic: Option<std::sync::Arc<worker_malloc_custody::Custody>>,
    stderr_reader: Option<JoinHandle<()>>,
    diagnostic_writer: Option<JoinHandle<()>>,
}
impl Worker {
    fn open(executable: &Path, timeout: Duration) -> Result<Self> {
        if timeout.is_zero() || timeout > Duration::from_secs(120) {
            return Err("worker timeout must be within (0,120] seconds".into());
        }
        let executable = executable.canonicalize().map_err(|e| e.to_string())?;
        let diagnostic_config = worker_malloc_custody::DiagnosticConfig::from_env()?;
        Self::open_with_diagnostic(&executable, timeout, diagnostic_config)
    }
    fn open_with_diagnostic(
        executable: &Path,
        timeout: Duration,
        diagnostic_config: Option<worker_malloc_custody::DiagnosticConfig>,
    ) -> Result<Self> {
        if timeout.is_zero() || timeout > Duration::from_secs(120) {
            return Err("worker timeout must be within (0,120] seconds".into());
        }
        let executable = executable.canonicalize().map_err(|e| e.to_string())?;
        let mut command = Command::new(&executable);
        command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(
            if diagnostic_config.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            },
        );
        if let Some(config) = &diagnostic_config {
            command.env("QL_NATIVE_WORKER_MALLOC_TRACE_REQUESTS", config.selector());
        } else {
            // Invalid selectors disable the C++ observer too.
            command.env_remove("QL_NATIVE_WORKER_MALLOC_TRACE_REQUESTS");
        }
        let mut child = command.spawn().map_err(|e| e.to_string())?;
        let (diagnostic, diagnostic_writer) = match diagnostic_config {
            Some(config) => {
                match worker_malloc_custody::Custody::open(config, child.id(), &executable) {
                    Ok((value, writer)) => (Some(value), Some(writer)),
                    Err(error) => {
                        let _ = child.kill();
                        let _ = child.wait();
                        return Err(error);
                    }
                }
            }
            None => (None, None),
        };
        let stderr_reader = if let Some(custody) = &diagnostic {
            let pipe = child
                .stderr
                .take()
                .ok_or("missing diagnostic worker stderr")?;
            let custody = custody.clone();
            Some(thread::spawn(move || custody.stderr(pipe)))
        } else {
            None
        };
        let mut input = child.stdin.take().ok_or("missing worker stdin")?;
        let mut output = BufReader::new(child.stdout.take().ok_or("missing worker stdout")?);
        let (sender, receiver): (SyncSender<Exchange>, Receiver<Exchange>) = mpsc::sync_channel(1);
        let reader_custody = diagnostic.clone();
        let reader = thread::spawn(move || {
            let mut frontier = worker_malloc_custody::Frontier::new();
            while let Ok(job) = receiver.recv() {
                let ordinal = reader_custody.as_ref().and_then(|_| frontier.next());
                let selected =
                    ordinal.is_some_and(|n| reader_custody.as_ref().is_some_and(|c| c.selects(n)));
                let mut bytes = Vec::new();
                let mut written = false;
                let result = (|| {
                    input.write_all(&job.request).map_err(|e| e.to_string())?;
                    input.write_all(b"\n").map_err(|e| e.to_string())?;
                    input.flush().map_err(|e| e.to_string())?;
                    written = true;
                    if selected {
                        reader_custody
                            .as_ref()
                            .expect("selected custody")
                            .submitted(ordinal.expect("selected ordinal"));
                    }
                    output
                        .by_ref()
                        .take((MAX_MESSAGE + 1) as u64)
                        .read_until(b'\n', &mut bytes)
                        .map_err(|e| e.to_string())?;
                    if bytes.len() > MAX_MESSAGE || bytes.last() != Some(&b'\n') {
                        return Err("missing/excessive worker response".into());
                    }
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
                })();
                let selected_frontier = job.diagnostic_facts.as_ref().and_then(|facts| {
                    frontier.observe(facts, result.as_ref().ok());
                    selected.then(|| frontier.snapshot(facts, result.as_ref().ok()))
                });
                let failed = result.is_err();
                let parsed = result.is_ok();
                // Deliver the actual parsed/refused transport result before any
                // diagnostic byte handoff, hashing or file operation.
                let sent = job.reply.send(result).is_ok();
                if let Some(snapshot) = selected_frontier {
                    reader_custody
                        .as_ref()
                        .expect("selected custody")
                        .selected_pair(worker_malloc_custody::SelectedPair {
                            ordinal: ordinal.expect("selected ordinal"),
                            request: job.request,
                            reply: bytes,
                            written,
                            parsed,
                            frontier: snapshot,
                            facts: job.diagnostic_facts.expect("selected facts"),
                        });
                }
                if !sent || failed {
                    break;
                }
            }
        });
        Ok(Self {
            child,
            sender: Some(sender),
            reader: Some(reader),
            timeout,
            poisoned: false,
            diagnostic,
            stderr_reader,
            diagnostic_writer,
        })
    }
    fn invalidate(&mut self, reason: &str) -> String {
        self.poisoned = true;
        let _ = self.child.kill();
        format!("worker unavailable; operation standing unknown: {reason}")
    }
    fn exchange(&mut self, value: &Value) -> Result<Value> {
        self.exchange_contract(value, FIELD_CONTRACT)
    }
    fn exchange_contract(&mut self, value: &Value, contract: &str) -> Result<Value> {
        self.exchange_contract_retained(value, contract)
            .map_err(|(reason, _)| reason)
    }
    // The private receiving/export owner keeps the complete actual parsed
    // reply even when its contract or post-commit standing is refused. An
    // absent/malformed/oversized transport has no invented JSON receipt.
    fn exchange_contract_retained(
        &mut self,
        value: &Value,
        contract: &str,
    ) -> std::result::Result<Value, (String, Option<Value>)> {
        if self.poisoned {
            return Err((
                "worker unavailable; retained basis is unchanged".into(),
                None,
            ));
        }
        let request = serde_json::to_vec(value).map_err(|e| (e.to_string(), None))?;
        if request.len() > MAX_MESSAGE {
            return Err(("field control exceeds 32 MiB".into(), None));
        }
        let (reply, receiver) = mpsc::sync_channel(1);
        let result = self
            .sender
            .as_ref()
            .ok_or_else(|| ("worker closed".to_owned(), None))?
            .try_send(Exchange {
                request,
                reply,
                diagnostic_facts: self
                    .diagnostic
                    .as_ref()
                    .map(|_| worker_malloc_custody::RequestFacts::new(value)),
            })
            .map_err(|e| e.to_string())
            .and_then(|()| {
                receiver
                    .recv_timeout(self.timeout)
                    .map_err(|e| e.to_string())
            })
            .and_then(|v| v);
        let value = match result {
            Ok(value) => value,
            Err(error) => {
                return Err((self.invalidate(&format!("transport failed: {error}")), None));
            }
        };
        if value["schema"] == "ql.field-error/v1" {
            // An explicit refusal is recoverable only when native state did not
            // commit. Lost replies/timeouts may have committed; never retry them
            // automatically or pretend the previous receipt is current.
            if value["state_committed"] != false
                || !value["error"].is_string()
                || value.as_object().is_none_or(|object| object.len() != 3)
            {
                return Err((
                    self.invalidate("unqualified or post-commit error acknowledgement"),
                    Some(value),
                ));
            }
            return Err((value.to_string(), Some(value)));
        }
        if value["schema"] != contract {
            return Err((self.invalidate("unrecognised worker response"), Some(value)));
        }
        Ok(value)
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.sender.take();
        if let Some(custody) = &self.diagnostic {
            custody.wait_for_submitted_rows();
        }
        let _ = self.child.kill();
        let child_status = self.child.wait().ok();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        if let Some(reader) = self.stderr_reader.take() {
            let _ = reader.join();
        }
        if let Some(custody) = &self.diagnostic {
            custody.finish_writer();
        }
        if let Some(writer) = self.diagnostic_writer.take() {
            let _ = writer.join();
        }
        if let Some(custody) = &self.diagnostic {
            custody.closed(
                if self.poisoned {
                    "poisoned-owner-drop"
                } else {
                    "normal-owner-drop"
                },
                child_status,
            );
        }
    }
}

/// One persistent native numerical owner and the complete original/current M2
/// bases. Additional views read the returned state, not another worker instance.
/// Full M1/M3/Nara bases remain their native owners' objects; this bridge does
/// not project them into invented aliases or claim their joined acceptance.
pub struct FieldSession {
    worker: Worker,
    original: M2Request,
    current: M2Request,
    receipt: Value,
    guard: ReceiptGuard,
}
impl FieldSession {
    pub fn open(
        executable: &Path,
        m2: M2Request,
        field: FieldInput,
        timeout: Duration,
    ) -> Result<Self> {
        let frame = m2.execute()?;
        let frame = serde_json::to_value(frame).map_err(|e| e.to_string())?;
        let guard = ReceiptGuard::new(&frame, &field)?;
        let mut worker = Worker::open(executable, timeout)?;
        let request = json!({"schema":"ql.field-control/v1", "operation":"initialize", "m2":frame, "field":field});
        let receipt = worker.exchange(&request)?;
        if let Err(error) = guard.validate(&request, None, &receipt) {
            return Err(worker.invalidate(&error));
        }
        Ok(Self {
            worker,
            original: m2.clone(),
            current: m2,
            receipt,
            guard,
        })
    }
    pub fn original_basis(&self) -> &M2Request {
        &self.original
    }
    pub fn current_basis(&self) -> &M2Request {
        &self.current
    }
    /// Last acknowledged receipt only; inspect availability before labelling it live.
    pub fn last_receipt(&self) -> &Value {
        &self.receipt
    }
    pub fn available(&self) -> bool {
        !self.worker.poisoned
    }
    pub(crate) fn performance_exchange(&mut self, request: &Value) -> Result<Value> {
        if request["schema"] != performance::CONTROL {
            return Err("unknown native performance request contract".into());
        }
        self.worker
            .exchange_contract(request, "ql.performance-worker-reply/v1")
    }
    pub(crate) fn performance_exchange_retained(
        &mut self,
        request: &Value,
    ) -> std::result::Result<Value, (String, Option<Value>)> {
        if request["schema"] != performance::CONTROL {
            return Err(("unknown native performance request contract".into(), None));
        }
        self.worker
            .exchange_contract_retained(request, "ql.performance-worker-reply/v1")
    }
    pub(crate) fn performance_invalidate(&mut self, reason: &str) -> String {
        self.worker.invalidate(reason)
    }
    fn operation(&mut self, operation: &str, extra: Value) -> Result<Value> {
        let mut request = json!({"schema":"ql.field-control/v1", "operation":operation,
            "expected_generation":self.receipt["generation"], "expected_samples_elapsed":self.receipt["samples_elapsed"]});
        let object = request
            .as_object_mut()
            .ok_or("invalid internal operation")?;
        object.extend(
            extra
                .as_object()
                .ok_or("invalid operation parameters")?
                .clone(),
        );
        self.exchange_checked(&request)
    }
    fn exchange_checked(&mut self, request: &Value) -> Result<Value> {
        let receipt = self.worker.exchange(request)?;
        if let Err(error) = self.guard.validate(request, Some(&self.receipt), &receipt) {
            return Err(self.worker.invalidate(&error));
        }
        self.guard.adopted(&receipt);
        self.receipt = receipt.clone();
        Ok(receipt)
    }
    pub fn advance(&mut self, frames: u32, muted: bool) -> Result<Value> {
        self.operation("advance", json!({"frames":frames, "muted":muted}))
    }
    pub fn set_axis(&mut self, axis: u8, phase: LiftInput) -> Result<Value> {
        self.operation("set-axis", json!({"axis":axis, "phase":phase}))
    }
    pub fn replace_modes(&mut self, m2: M2Request, replace_state: bool) -> Result<Value> {
        let frame = m2.execute()?;
        let receipt = self.operation(
            "replace-modes",
            json!({"m2":frame, "replace_state":replace_state}),
        )?;
        self.current = m2;
        Ok(receipt)
    }
    /// Explicit nodal re-reading (`shapes[sample][mode]`, in sample order): the
    /// same samples and modal voices receive a new shape basis. Resident state,
    /// clock and cursor continue; the control generation increments. A malformed
    /// basis is refused here without transport; the worker re-validates it.
    pub fn replace_shapes(&mut self, shape_ref: &str, shapes: Vec<Vec<[f64; 3]>>) -> Result<Value> {
        let samples = self.receipt["targets"].as_array().map_or(0, Vec::len);
        let modes = self.receipt["amplitudes_metres"]
            .as_array()
            .map_or(0, Vec::len);
        if shape_ref.is_empty()
            || shape_ref.len() > 2048
            || shape_ref.chars().any(|c| c < ' ' || c == '\u{7f}')
        {
            return Err("invalid shape basis reference".into());
        }
        if shapes.len() != samples
            || shapes.iter().any(|sample| {
                sample.len() != modes
                    || sample
                        .iter()
                        .flatten()
                        .any(|x| !x.is_finite() || x.abs() > 1e6)
            })
        {
            return Err("shape replacement must keep the sample/mode basis with finite bounded coefficients".into());
        }
        self.operation(
            "replace-shapes",
            json!({"shape_ref":shape_ref, "shapes":shapes}),
        )
    }
    pub fn read(&mut self) -> Result<Value> {
        self.exchange_checked(&json!({"schema":"ql.field-control/v1", "operation":"read"}))
    }
}
