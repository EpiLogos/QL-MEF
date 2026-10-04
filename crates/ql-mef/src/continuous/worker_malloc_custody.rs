//! OFF-default original worker diagnostic custody, outside native callbacks.
//! Three selected exchanges only; no second worker, clock, transcript or grant.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const SELECTOR: &str = "QL_NATIVE_WORKER_MALLOC_TRACE_REQUESTS";
const ROOT: &str = "QL_NATIVE_WORKER_MALLOC_EVIDENCE_ROOT";
const STDERR_BYTES: usize = 8192;
const META_BYTES: usize = 8192;
// At most three pairs, eight disjoint stderr parts and one terminal status.
// The limit is on the entire worker lifetime, not repeated queue replenishment.
const WRITER_JOBS: usize = 12;
const STDERR_PART: usize = 2048;

pub(super) struct SelectedPair {
    pub ordinal: u64,
    pub request: Vec<u8>,
    pub reply: Vec<u8>,
    pub written: bool,
    pub parsed: bool,
    pub frontier: Value,
    pub facts: RequestFacts,
}
enum WriteJob {
    Pair(SelectedPair),
    StderrPart(Vec<u8>),
    StderrFinished(Value),
}

#[derive(Clone)]
pub(super) struct DiagnosticConfig {
    selected: [u64; 3],
    count: usize,
    text: String,
    root: PathBuf,
}
impl DiagnosticConfig {
    pub(super) fn from_env() -> Result<Option<Self>, String> {
        let Some(text) = std::env::var_os(SELECTOR) else {
            return Ok(None);
        };
        let Some(text) = text.to_str() else {
            return Ok(None);
        };
        // Same disabled cases as the actual C++ observer, not a new command.
        let Some((selected, count)) = Self::ordinals(text) else {
            return Ok(None);
        };
        let root = std::env::var_os(ROOT)
            .ok_or("native malloc custody requires its private evidence root")?;
        Self::new(text, selected, count, Path::new(&root)).map(Some)
    }
    fn ordinals(text: &str) -> Option<([u64; 3], usize)> {
        if text.is_empty() || text.len() >= 64 {
            return None;
        }
        let mut selected = [0; 3];
        let mut count = 0;
        for item in text.split(',') {
            if count == 3
                || !item
                    .as_bytes()
                    .first()
                    .is_some_and(|c| (b'1'..=b'9').contains(c))
            {
                return None;
            }
            let n: u64 = item.parse().ok()?;
            if n == 0 || n.to_string() != item || (count != 0 && n <= selected[count - 1]) {
                return None;
            }
            selected[count] = n;
            count += 1;
        }
        Some((selected, count))
    }
    fn new(text: &str, selected: [u64; 3], count: usize, root: &Path) -> Result<Self, String> {
        let metadata = fs::symlink_metadata(root).map_err(|e| e.to_string())?;
        let canonical = root.canonicalize().map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o077 != 0 {
                return Err(
                    "native malloc evidence root exposes private inputs to other accounts".into(),
                );
            }
        }
        if !root.is_absolute()
            || root.as_os_str().len() > 4096
            || !metadata.is_dir()
            || metadata.file_type().is_symlink()
            || canonical != root
        {
            return Err(
                "native malloc evidence requires an existing canonical absolute private directory"
                    .into(),
            );
        }
        Ok(Self {
            selected,
            count,
            text: text.into(),
            root: canonical,
        })
    }
    #[cfg(test)]
    pub(super) fn for_test(text: &str, root: &Path) -> Result<Self, String> {
        let (selected, count) = Self::ordinals(text).ok_or("invalid diagnostic test selector")?;
        Self::new(text, selected, count, root)
    }
    pub(super) fn selector(&self) -> &str {
        &self.text
    }
}

pub(super) struct Custody {
    directory: PathBuf,
    selected: [u64; 3],
    count: usize,
    pid: u32,
    failed: AtomicBool,
    lost_custody: AtomicBool,
    writer_completed: AtomicBool,
    writer: Mutex<Option<mpsc::SyncSender<WriteJob>>>,
    progress: Mutex<Progress>,
    changed: Condvar,
    close_wait_exhausted: AtomicBool,
}
#[derive(Default)]
struct Progress {
    submitted: [bool; 3],
    queued: [bool; 3],
    seen: [bool; 3],
    eof: bool,
}
fn file(path: &Path) -> Result<File, String> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path).map_err(|e| e.to_string())
}
fn raw(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut output = file(path)?;
    output
        .write_all(bytes)
        .and_then(|_| output.sync_all())
        .map_err(|e| e.to_string())
}
fn metadata(path: &Path, value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > META_BYTES {
        return Err("native malloc custody metadata exceeds its fixed bound".into());
    }
    raw(path, &bytes)
}
fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
impl Custody {
    pub(super) fn open(
        config: DiagnosticConfig,
        pid: u32,
        executable: &Path,
    ) -> Result<(Arc<Self>, JoinHandle<()>), String> {
        let directory = config.root.join(format!("worker-{pid}"));
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        // Actual child PID, fresh caller root and exclusive directory. Never reuse.
        builder.create(&directory).map_err(|e| e.to_string())?;
        let (sender, receiver) = mpsc::sync_channel(WRITER_JOBS);
        let out = Arc::new(Self {
            directory,
            selected: config.selected,
            count: config.count,
            pid,
            failed: AtomicBool::new(false),
            lost_custody: AtomicBool::new(false),
            writer_completed: AtomicBool::new(false),
            writer: Mutex::new(Some(sender)),
            progress: Mutex::new(Progress::default()),
            changed: Condvar::new(),
            close_wait_exhausted: AtomicBool::new(false),
        });
        metadata(
            &out.directory.join("opened.json"),
            &json!({
                "schema":"ql.native-worker-malloc-custody/v1", "phase":"actual-worker-opened",
                "host_pid":std::process::id(), "worker_pid":pid, "executable":executable,
                "selected":out.selected[..out.count].iter().map(u64::to_string).collect::<Vec<_>>(),
                "counter_platform":if cfg!(target_os="macos") { "Apple-SDK-observation-required" } else { "unavailable-non-Apple" },
                "request_reply_bound_bytes":super::MAX_MESSAGE,"stderr_bound_bytes":STDERR_BYTES,
                "writer_job_bound":WRITER_JOBS,"selected_raw_capacity_bound_bytes":3*(2*super::MAX_MESSAGE+1),
                "stderr_part_capacity_bound_bytes":9*STDERR_PART,
                "delivery_order":"actual-result-channel-before-selected-byte-handoff",
                "standing":"original raw worker I/O; source/output authority remains existing native owner"
            }),
        )?;
        let custody = out.clone();
        let writer = thread::Builder::new()
            .name("ql-native-custody-writer".into())
            .spawn(move || custody.write_jobs(receiver))
            .map_err(|e| e.to_string())?;
        out.notice("opened");
        Ok((out, writer))
    }
    #[cfg(test)]
    pub(super) fn directory(&self) -> &Path {
        &self.directory
    }
    pub(super) fn selects(&self, ordinal: u64) -> bool {
        self.selected[..self.count].contains(&ordinal)
    }
    fn notice(&self, phase: &str) {
        // Locator/status only. Actual private payload never enters parent stderr.
        eprintln!(
            "{}",
            json!({"schema":"ql.native-worker-malloc-custody-locator/v1",
            "phase":phase,"worker_pid":self.pid,"directory":self.directory,"failed":self.failed.load(Ordering::Relaxed)})
        );
    }
    fn failure(&self, result: Result<(), String>) {
        if result.is_err() {
            self.failed.store(true, Ordering::Relaxed);
        }
    }
    fn offer(&self, job: WriteJob) -> bool {
        // No blocking send and no disk I/O on transport or stderr drain.
        let sender = self.writer.lock().unwrap_or_else(|e| e.into_inner());
        let accepted = sender.as_ref().is_some_and(|s| s.try_send(job).is_ok());
        if !accepted {
            self.lost_custody.store(true, Ordering::Relaxed);
        }
        accepted
    }
    pub(super) fn submitted(&self, ordinal: u64) {
        if let Some(index) = self.selected[..self.count]
            .iter()
            .position(|n| *n == ordinal)
        {
            self.progress
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .submitted[index] = true;
        }
    }
    pub(super) fn selected_pair(&self, pair: SelectedPair) {
        let index = self.selected[..self.count]
            .iter()
            .position(|n| *n == pair.ordinal);
        // Bound actual retained Vec capacities as well as raw byte lengths.
        // Overspare buffers remain native results; only diagnostic custody fails.
        if pair.request.len() > super::MAX_MESSAGE
            || pair.request.capacity() > super::MAX_MESSAGE
            || pair.reply.len() > super::MAX_MESSAGE + 1
            || pair.reply.capacity() > super::MAX_MESSAGE + 1
            || index.is_none()
        {
            self.lost_custody.store(true, Ordering::Relaxed);
            return;
        }
        let index = index.expect("selected index");
        let mut progress = self.progress.lock().unwrap_or_else(|e| e.into_inner());
        if progress.queued[index] {
            self.lost_custody.store(true, Ordering::Relaxed);
            return;
        }
        progress.queued[index] = self.offer(WriteJob::Pair(pair));
    }
    pub(super) fn finish_writer(&self) {
        self.writer.lock().unwrap_or_else(|e| e.into_inner()).take();
    }
    fn write_jobs(&self, receiver: mpsc::Receiver<WriteJob>) {
        let mut stderr = None;
        let mut stderr_opened = false;
        for job in receiver {
            match job {
                WriteJob::Pair(pair) => {
                    let mut record = exchange_record(
                        pair.ordinal,
                        self.pid,
                        ExchangeBytes {
                            request: &pair.request,
                            reply: &pair.reply,
                            written: pair.written,
                            parsed: pair.parsed,
                        },
                        pair.frontier,
                        &pair.facts,
                    );
                    record["request_capacity_bytes"] = pair.request.capacity().into();
                    record["reply_capacity_bytes"] = pair.reply.capacity().into();
                    self.failure(raw(
                        &self
                            .directory
                            .join(format!("request-{}.json", pair.ordinal)),
                        &pair.request,
                    ));
                    self.failure(raw(
                        &self.directory.join(format!("reply-{}.jsonl", pair.ordinal)),
                        &pair.reply,
                    ));
                    self.failure(metadata(
                        &self
                            .directory
                            .join(format!("exchange-{}.json", pair.ordinal)),
                        &record,
                    ));
                }
                WriteJob::StderrPart(bytes) => {
                    if !stderr_opened {
                        stderr_opened = true;
                        match file(&self.directory.join("worker-stderr.bin")) {
                            Ok(f) => stderr = Some(f),
                            Err(_) => self.failed.store(true, Ordering::Relaxed),
                        }
                    }
                    if let Some(output) = &mut stderr {
                        self.failure(
                            output
                                .write_all(&bytes)
                                .and_then(|()| output.sync_all())
                                .map_err(|e| e.to_string()),
                        );
                    }
                }
                WriteJob::StderrFinished(mut record) => {
                    // Non-Apple or an empty real pipe still has original empty bytes.
                    if !stderr_opened {
                        stderr_opened = true;
                        match file(&self.directory.join("worker-stderr.bin")) {
                            Ok(f) => stderr = Some(f),
                            Err(_) => self.failed.store(true, Ordering::Relaxed),
                        }
                    }
                    record["file_write_failed"] = self.failed.load(Ordering::Relaxed).into();
                    self.failure(metadata(
                        &self.directory.join("stderr-status.json"),
                        &record,
                    ));
                }
            }
        }
        self.writer_completed.store(true, Ordering::Release);
    }
    fn observed_line(&self, bytes: &[u8]) -> bool {
        if let Ok(v) = serde_json::from_slice::<Value>(bytes)
            && v["schema"] == "ql.native-worker-malloc-observation/v1"
            && v["pid"].as_u64() == Some(u64::from(self.pid))
            && let Some(ordinal) = counter(&v["completed_request_ordinal"])
            && let Some(index) = self.selected[..self.count]
                .iter()
                .position(|n| *n == ordinal)
        {
            let mut progress = self.progress.lock().unwrap_or_else(|e| e.into_inner());
            let newly_seen = !progress.seen[index];
            progress.seen[index] = true;
            self.changed.notify_all();
            return newly_seen;
        }
        false
    }
    pub(super) fn wait_for_submitted_rows(&self) {
        if !cfg!(target_os = "macos") {
            return;
        }
        // Native stdout flush precedes its destruction-boundary stderr row.
        // Wait only at diagnostic close, never change an actual operation result.
        let state = self.progress.lock().unwrap_or_else(|e| e.into_inner());
        let (state, result) = self
            .changed
            .wait_timeout_while(state, Duration::from_secs(1), |p| {
                !p.eof && (0..self.count).any(|i| p.submitted[i] && !p.seen[i])
            })
            .unwrap_or_else(|e| e.into_inner());
        drop(state);
        self.close_wait_exhausted
            .store(result.timed_out(), Ordering::Relaxed);
    }
    pub(super) fn stderr(&self, mut input: impl Read) {
        let (mut observed, mut retained) = (0_u64, 0_usize);
        let mut complete = false;
        let mut overflow = false;
        let mut chunk = [0_u8; 4096];
        let mut line = Vec::with_capacity(STDERR_PART);
        let mut pending = Vec::with_capacity(STDERR_PART);
        let mut oversized_line = false;
        loop {
            match input.read(&mut chunk) {
                Ok(0) => {
                    complete = true;
                    break;
                }
                Ok(n) => {
                    observed = match observed.checked_add(n as u64) {
                        Some(v) => v,
                        None => {
                            overflow = true;
                            u64::MAX
                        }
                    };
                    let keep = n.min(STDERR_BYTES.saturating_sub(retained));
                    for b in &chunk[..keep] {
                        pending.push(*b);
                        let mut selected_row = false;
                        if *b == b'\n' {
                            if !oversized_line {
                                selected_row = self.observed_line(&line);
                            }
                            line.clear();
                            oversized_line = false;
                        } else if line.len() < STDERR_PART {
                            line.push(*b);
                        } else {
                            oversized_line = true;
                        }
                        // At most four full chunks, three newly seen SDK rows,
                        // and one final prefix. Never clone the cumulative prefix.
                        if pending.len() == STDERR_PART || selected_row {
                            let bytes =
                                std::mem::replace(&mut pending, Vec::with_capacity(STDERR_PART));
                            self.offer(WriteJob::StderrPart(bytes));
                        }
                    }
                    retained += keep;
                    // Continue draining even after the finite retained prefix.
                }
                Err(_) => {
                    self.failed.store(true, Ordering::Relaxed);
                    break;
                }
            }
        }
        if !pending.is_empty() {
            self.offer(WriteJob::StderrPart(pending));
        }
        self.progress.lock().unwrap_or_else(|e| e.into_inner()).eof = complete;
        self.changed.notify_all();
        self.offer(WriteJob::StderrFinished(json!({
            "schema":"ql.native-worker-malloc-stderr-custody/v1","worker_pid":self.pid,
            "eof_observed":complete,"observed_bytes":observed.to_string(),"retained_bytes":retained,
            "truncated":observed > STDERR_BYTES as u64,"counter_overflow":overflow,
            "file_write_failed":false
        })));
    }
    pub(super) fn closed(&self, phase: &str, status: Option<ExitStatus>) {
        let progress = self.progress.lock().unwrap_or_else(|e| e.into_inner());
        let signal: Option<i32> = {
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                status.as_ref().and_then(ExitStatusExt::signal)
            }
            #[cfg(not(unix))]
            {
                None
            }
        };
        self.failure(metadata(&self.directory.join("closed.json"), &json!({
            "schema":"ql.native-worker-malloc-custody-close/v1","worker_pid":self.pid,
            "phase":phase,"owned_child_waited":status.is_some(),
            "actual_exit_code":status.as_ref().and_then(ExitStatus::code),"actual_exit_signal":signal,
            "actual_exit_success":status.as_ref().map(ExitStatus::success),
            "submitted":progress.submitted[..self.count],"observed":progress.seen[..self.count],
            "selected_pairs_queued":progress.queued[..self.count],
            "lost_custody":self.lost_custody.load(Ordering::Relaxed),
            "writer_completed":self.writer_completed.load(Ordering::Acquire),
            "close_wait_exhausted":self.close_wait_exhausted.load(Ordering::Relaxed),
            "evidence_write_failed":self.failed.load(Ordering::Relaxed),
            "standing":"lifecycle observation only; missing selected rows remain unexecuted/incomplete"
        })));
        self.notice(phase);
    }
}

pub(super) struct Frontier {
    ordinal: u64,
    rendered: u64,
    overflow: bool,
    field: Option<FieldFrontier>,
}
#[derive(Clone, Copy)]
struct FieldFrontier {
    samples: Option<usize>,
    modes: Option<usize>,
    shapes: Option<usize>,
    elapsed: Option<u64>,
    generation: Option<u64>,
}
fn counter(value: &Value) -> Option<u64> {
    let s = value.as_str()?;
    if s != "0"
        && !s
            .as_bytes()
            .first()
            .is_some_and(|c| (b'1'..=b'9').contains(c))
    {
        return None;
    }
    if !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}
impl Frontier {
    pub(super) fn new() -> Self {
        Self {
            ordinal: 0,
            rendered: 0,
            overflow: false,
            field: None,
        }
    }
    pub(super) fn next(&mut self) -> Option<u64> {
        self.ordinal = match self.ordinal.checked_add(1) {
            Some(v) => v,
            None => {
                self.overflow = true;
                return None;
            }
        };
        Some(self.ordinal)
    }
    pub(super) fn observe(&mut self, request: &RequestFacts, reply: Option<&Value>) {
        let success = reply.is_some_and(|v| v["schema"] == super::FIELD_CONTRACT);
        let rendered = if request.domain == "field" && success {
            reply
                .and_then(|v| v["audio"].as_array())
                .map_or(0, Vec::len)
        } else {
            0
        };
        match self.rendered.checked_add(rendered as u64) {
            Some(n) => self.rendered = n,
            None => self.overflow = true,
        }
        if let Some(v) = reply.filter(|_| success) {
            self.field = Some(FieldFrontier {
                samples: v["targets"].as_array().map(Vec::len),
                modes: v["amplitudes_metres"].as_array().map(Vec::len),
                shapes: request.shapes.or(self.field.and_then(|f| f.shapes)),
                elapsed: counter(&v["samples_elapsed"]),
                generation: counter(&v["generation"]),
            });
        }
    }
    pub(super) fn snapshot(&self, request: &RequestFacts, reply: Option<&Value>) -> Value {
        let rendered = if request.domain == "field"
            && reply.is_some_and(|v| v["schema"] == super::FIELD_CONTRACT)
        {
            reply
                .and_then(|v| v["audio"].as_array())
                .map_or(0, Vec::len)
        } else {
            0
        };
        let field=self.field.map(|f| json!({"field_sample_count":f.samples,"field_mode_count":f.modes,
            "field_shape_vector_count":f.shapes,"field_samples_elapsed":f.elapsed.map(|n|n.to_string()),
            "field_generation":f.generation.map(|n|n.to_string())}));
        json!({"rendered_field_frames":rendered,"rendered_field_frames_total":self.rendered.to_string(),
            "frame_counter_overflow":self.overflow,"field":field})
    }
}
pub(super) struct RequestFacts {
    domain: &'static str,
    operation: String,
    requested_frames: Option<u64>,
    shapes: Option<usize>,
}
impl RequestFacts {
    pub(super) fn new(value: &Value) -> Self {
        let domain = if value["schema"] == "ql.field-control/v1" {
            "field"
        } else if value["schema"] == "ql.performance-control/v1" {
            "performance"
        } else {
            "unknown"
        };
        let operation = value["operation"]
            .as_str()
            .filter(|s| s.len() <= 64 && s.bytes().all(|c| c.is_ascii_graphic()))
            .unwrap_or("unparsed")
            .to_owned();
        let shapes = if domain == "field" && operation == "initialize" {
            value["field"]["samples"].as_array().and_then(|samples| {
                samples.iter().try_fold(0_usize, |sum, s| {
                    sum.checked_add(s["mode_shapes"].as_array()?.len())
                })
            })
        } else if domain == "field" && operation == "replace-shapes" {
            value["shapes"].as_array().and_then(|samples| {
                samples
                    .iter()
                    .try_fold(0_usize, |sum, s| sum.checked_add(s.as_array()?.len()))
            })
        } else {
            None
        };
        Self {
            domain,
            operation,
            requested_frames: value["frames"].as_u64(),
            shapes,
        }
    }
    pub(super) fn summary(&self) -> Value {
        json!({"domain":self.domain,"operation":self.operation,"original_requested_frames":self.requested_frames})
    }
}
pub(super) struct ExchangeBytes<'a> {
    pub request: &'a [u8],
    pub reply: &'a [u8],
    pub written: bool,
    pub parsed: bool,
}
pub(super) fn exchange_record(
    ordinal: u64,
    pid: u32,
    io: ExchangeBytes<'_>,
    frontier: Value,
    facts: &RequestFacts,
) -> Value {
    json!({"schema":"ql.native-worker-malloc-selected-exchange/v1","worker_pid":pid,
        "request_ordinal":ordinal.to_string(),"request_bytes":io.request.len(),"request_sha256":digest(io.request),
        "reply_bytes":io.reply.len(),"reply_sha256":digest(io.reply),"request_fully_written":io.written,
        "response_framed_complete":io.reply.last()==Some(&b'\n') && io.reply.len()<=super::MAX_MESSAGE,
        "response_parsed":io.parsed,"facts":facts.summary(),"frontier":frontier,
        "standing":"actual original serial transport bytes before higher owner validation"})
}
