//! Safe management bridge to the installed C/C++ continuous owner.
//! Native host chooses/authorises the executable and subject. Neither a source
//! reference nor a geometry payload grants authority. No child/JSON/domain work
//! runs on an audio callback; this serial API transfers bounded control batches.
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
}

struct Exchange {
    request: Vec<u8>,
    reply: SyncSender<Result<Value>>,
}
struct Worker {
    child: Child,
    sender: Option<SyncSender<Exchange>>,
    reader: Option<JoinHandle<()>>,
    timeout: Duration,
    poisoned: bool,
}
impl Worker {
    fn open(executable: &Path, timeout: Duration) -> Result<Self> {
        if timeout.is_zero() || timeout > Duration::from_secs(120) {
            return Err("worker timeout must be within (0,120] seconds".into());
        }
        let executable = executable.canonicalize().map_err(|e| e.to_string())?;
        let mut child = Command::new(executable)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())?;
        let mut input = child.stdin.take().ok_or("missing worker stdin")?;
        let mut output = BufReader::new(child.stdout.take().ok_or("missing worker stdout")?);
        let (sender, receiver): (SyncSender<Exchange>, Receiver<Exchange>) = mpsc::sync_channel(1);
        let reader = thread::spawn(move || {
            while let Ok(job) = receiver.recv() {
                let result = (|| {
                    input.write_all(&job.request).map_err(|e| e.to_string())?;
                    input.write_all(b"\n").map_err(|e| e.to_string())?;
                    input.flush().map_err(|e| e.to_string())?;
                    let mut bytes = Vec::new();
                    output.by_ref().take((MAX_MESSAGE + 1) as u64)
                        .read_until(b'\n', &mut bytes).map_err(|e| e.to_string())?;
                    if bytes.len() > MAX_MESSAGE || bytes.last() != Some(&b'\n') {
                        return Err("missing/excessive worker response".into());
                    }
                    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
                })();
                let failed = result.is_err();
                if job.reply.send(result).is_err() || failed { break; }
            }
        });
        Ok(Self { child, sender: Some(sender), reader: Some(reader), timeout, poisoned: false })
    }
    fn exchange(&mut self, value: &Value) -> Result<Value> {
        if self.poisoned { return Err("worker unavailable; retained basis is unchanged".into()); }
        let request = serde_json::to_vec(value).map_err(|e| e.to_string())?;
        if request.len() > MAX_MESSAGE { return Err("field control exceeds 32 MiB".into()); }
        let (reply, receiver) = mpsc::sync_channel(1);
        let result = self.sender.as_ref().ok_or("worker closed")?
            .try_send(Exchange { request, reply }).map_err(|e| e.to_string())
            .and_then(|()| receiver.recv_timeout(self.timeout).map_err(|e| e.to_string()))
            .and_then(|v| v);
        let value = match result {
            Ok(value) => value,
            Err(error) => {
                self.poisoned = true;
                let _ = self.child.kill();
                return Err(format!("worker transport failed; operation standing unknown: {error}"));
            }
        };
        if value["schema"] == "ql.field-error/v1" {
            // An explicit refusal is recoverable only when native state did not
            // commit. Lost replies/timeouts may have committed; never retry them
            // automatically or pretend the previous receipt is current.
            if value["state_committed"] != false { self.poisoned = true; }
            return Err(value.to_string());
        }
        if value["schema"] != FIELD_CONTRACT {
            self.poisoned = true;
            return Err("unrecognised worker response; continuation standing unknown".into());
        }
        Ok(value)
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.sender.take();
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() { let _ = reader.join(); }
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
}
impl FieldSession {
    pub fn open(executable: &Path, m2: M2Request, field: FieldInput, timeout: Duration) -> Result<Self> {
        let frame = m2.execute()?;
        let mut worker = Worker::open(executable, timeout)?;
        let receipt = worker.exchange(&json!({"schema":"ql.field-control/v1", "operation":"initialize", "m2":frame, "field":field}))?;
        Ok(Self { worker, original: m2.clone(), current: m2, receipt })
    }
    pub fn original_basis(&self) -> &M2Request { &self.original }
    pub fn current_basis(&self) -> &M2Request { &self.current }
    /// Last acknowledged receipt only; inspect availability before labelling it live.
    pub fn last_receipt(&self) -> &Value { &self.receipt }
    pub fn available(&self) -> bool { !self.worker.poisoned }
    fn operation(&mut self, operation: &str, extra: Value) -> Result<Value> {
        let mut request = json!({"schema":"ql.field-control/v1", "operation":operation,
            "expected_generation":self.receipt["generation"], "expected_samples_elapsed":self.receipt["samples_elapsed"]});
        let object = request.as_object_mut().ok_or("invalid internal operation")?;
        object.extend(extra.as_object().ok_or("invalid operation parameters")?.clone());
        let receipt = self.worker.exchange(&request)?;
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
        let receipt = self.operation("replace-modes", json!({"m2":frame, "replace_state":replace_state}))?;
        self.current = m2;
        Ok(receipt)
    }
    pub fn read(&mut self) -> Result<Value> {
        let receipt = self.worker.exchange(&json!({"schema":"ql.field-control/v1", "operation":"read"}))?;
        self.receipt = receipt.clone();
        Ok(receipt)
    }
}
