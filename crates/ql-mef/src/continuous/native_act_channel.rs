//! Dedicated native Act channel. Constructor qualifies the ACTUAL OS peer and
//! loaded supervisor through existing receipt-owned OI native image custody.
//! No request, environment hash/file, Value or callback can mint this type.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::fd::OwnedFd;
use std::os::unix::fs::MetadataExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const LIMIT: u64 = 1024 * 1024;
const TIMEOUT: Duration = Duration::from_secs(20);
const REQUEST: usize = 32 * 1024 * 1024;
const REPLY: usize = 64 * 1024 * 1024;
fn sha(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
fn count(value: &Value) -> Result<u64, String> {
    let text = value
        .as_str()
        .ok_or("native canonical image counter absent")?;
    let n: u64 = text.parse().map_err(|_| "native image counter invalid")?;
    if n.to_string() != text {
        return Err("native image counter noncanonical".into());
    }
    Ok(n)
}
fn collect(pipe: impl Read + Send + 'static) -> std::sync::mpsc::Receiver<Result<Vec<u8>, String>> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let result = pipe
            .take(LIMIT + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())
            .and_then(|_| {
                if bytes.len() as u64 > LIMIT {
                    Err("native qualifier output exceeds bound".into())
                } else {
                    Ok(bytes)
                }
            });
        let _ = tx.send(result);
    });
    rx
}
fn execute(mut command: Command) -> Result<Vec<u8>, String> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let out = collect(
        child
            .stdout
            .take()
            .ok_or("native qualifier stdout absent")?,
    );
    let err = collect(
        child
            .stderr
            .take()
            .ok_or("native qualifier stderr absent")?,
    );
    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            break status;
        }
        if start.elapsed() >= TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            return Err("native actual parent qualifier timed out".into());
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let out = out
        .recv_timeout(TIMEOUT.saturating_sub(start.elapsed()))
        .map_err(|_| "native qualifier stdout remained open")??;
    let err = err
        .recv_timeout(TIMEOUT.saturating_sub(start.elapsed()))
        .map_err(|_| "native qualifier stderr remained open")??;
    if !status.success() {
        return Err(format!(
            "native-act-origin-refused: {}",
            String::from_utf8_lossy(&err)
        ));
    }
    Ok(out)
}
/// OS account home only. HOME/OI_DATA_HOME/XDG paths never choose the anchor.
fn receipt_root() -> Result<(PathBuf, u32), String> {
    #[cfg(target_os = "linux")]
    {
        let uid = fs::metadata("/proc/self").map_err(|e| e.to_string())?.uid();
        let passwd = fs::read_to_string("/etc/passwd").map_err(|e| e.to_string())?;
        let homes: Vec<_> = passwd
            .lines()
            .filter_map(|line| {
                let fields: Vec<_> = line.split(':').collect();
                (fields.len() == 7 && fields[2].parse::<u32>().ok() == Some(uid))
                    .then(|| PathBuf::from(fields[5]))
            })
            .collect();
        if homes.len() != 1 || !homes[0].is_absolute() {
            return Err("native OS account home unavailable".into());
        }
        Ok((homes[0].join(".local/share/oi"), uid))
    }
    #[cfg(target_os = "macos")]
    {
        let mut id = Command::new("/usr/bin/id");
        id.arg("-u").stdin(Stdio::null());
        let uid: u32 = String::from_utf8(execute(id)?)
            .map_err(|e| e.to_string())?
            .trim()
            .parse()
            .map_err(|_| "native UID unavailable")?;
        let mut user = Command::new("/usr/bin/dscacheutil");
        user.args(["-q", "user", "-a", "uid", &uid.to_string()])
            .stdin(Stdio::null());
        let bytes = execute(user)?;
        let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        let homes: Vec<_> = text
            .lines()
            .filter_map(|l| l.strip_prefix("dir: ").map(PathBuf::from))
            .collect();
        if homes.len() != 1 || !homes[0].is_absolute() {
            return Err("native OS account home unavailable".into());
        }
        Ok((homes[0].join("Library/Application Support/OI"), uid))
    }
}
fn owned(path: &Path, uid: u32) -> Result<Vec<u8>, String> {
    let meta = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !meta.is_file()
        || meta.uid() != uid
        || meta.mode() & 0o022 != 0
        || meta.len() == 0
        || meta.len() > LIMIT
    {
        return Err("native receipt unavailable or foreign".into());
    }
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let opened = file.metadata().map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    (&mut file)
        .take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    let after = file.metadata().map_err(|e| e.to_string())?;
    let current = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    let same = |a: &fs::Metadata, b: &fs::Metadata| {
        a.dev() == b.dev()
            && a.ino() == b.ino()
            && a.len() == b.len()
            && a.uid() == b.uid()
            && a.mode() == b.mode()
            && a.mtime() == b.mtime()
            && a.mtime_nsec() == b.mtime_nsec()
            && a.ctime() == b.ctime()
            && a.ctime_nsec() == b.ctime_nsec()
    };
    if bytes.len() as u64 != meta.len()
        || !same(&meta, &opened)
        || !same(&opened, &after)
        || !same(&after, &current)
    {
        return Err("native receipt changed".into());
    }
    Ok(bytes)
}
fn locate_qualifier() -> Result<(PathBuf, Value), String> {
    let (root, uid) = receipt_root()?;
    let developer = root.join("receipts/dev/installed/oi-native-act-parent.json");
    let cut: Value = if developer.exists() {
        let cut: Value =
            serde_json::from_slice(&owned(&developer, uid)?).map_err(|e| e.to_string())?;
        let source = owned(
            &developer.with_file_name("oi-native-act-parent-source.json"),
            uid,
        )?;
        let receipt: Value = serde_json::from_slice(&source).map_err(|e| e.to_string())?;
        if cut["standing"] != "source_built_component"
            || cut["source_receipt_sha256"] != sha(&source)
            || receipt["schema"] != "oi.native-parent-source-build/v1"
            || receipt["status"] != "component_images_ready"
            || receipt["sources"]["oi"]["revision"] != cut["source_revision"]
            || receipt["sources"]["oi"]["tree"] != cut["source_tree"]
        {
            return Err("native source-built image owner unavailable".into());
        }
        cut
    } else {
        let receipt: Value =
            serde_json::from_slice(&owned(&root.join("receipts/installed-desktop.json"), uid)?)
                .map_err(|e| e.to_string())?;
        let cut = receipt["native_parent_cut"].clone();
        if receipt["schema"] != "oi.installed-desktop/v1"
            || cut["standing"] != "installed"
            || cut["source_revision"] != receipt["bundle"]["source_revision"]
            || cut["packaging_sha256"]
                != json!(format!(
                    "sha256:{}",
                    receipt["bundle"]["sha256"]
                        .as_str()
                        .ok_or("native adopted bundle SHA absent")?
                ))
        {
            return Err("native installed image owner unavailable".into());
        }
        cut
    };
    if cut["schema"] != "oi.native-parent-image-cut/v1" {
        return Err("native image cut unsupported".into());
    }
    let rows = cut["components"]
        .as_array()
        .ok_or("native image cohort absent")?;
    if rows.len() > 512 {
        return Err("native image cohort exceeds bound".into());
    }
    let rows: Vec<_> = rows.iter().filter(|r| r["role"] == "qualifier").collect();
    if rows.len() != 1 {
        return Err("native qualifier image missing or ambiguous".into());
    }
    let component = rows[0];
    let path = PathBuf::from(
        component["executable"]
            .as_str()
            .ok_or("native qualifier image path absent")?,
    );
    if !path.is_absolute() || path.canonicalize().map_err(|e| e.to_string())? != path {
        return Err("native qualifier image not canonical".into());
    }
    let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
    if meta.uid() != uid
        || meta.mode() & 0o022 != 0
        || meta.mode() & 0o111 == 0
        || !meta.is_file()
        || meta.len() == 0
        || meta.len() > 512 * 1024 * 1024
        || meta.len() != count(&component["bytes"])?
    {
        return Err("native qualifier image missing/changed".into());
    }
    let mut image = File::open(&path).map_err(|e| e.to_string())?;
    let mut digest = Sha256::new();
    let mut block = [0u8; 65536];
    loop {
        let n = image.read(&mut block).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        digest.update(&block[..n]);
    }
    if component["sha256"] != json!(format!("sha256:{:x}", digest.finalize())) {
        return Err("native qualifier bytes drifted".into());
    }
    Ok((path, cut))
}

/// Private-channel origin, not an imported selection token. Actual native C
/// closed-reader scope remains mandatory for every operation on this channel.
pub struct QualifiedNativeActChannel {
    reader: BufReader<UnixStream>,
    writer: UnixStream,
    qualification: Value,
}
/// Received exclusively from an independently qualified native channel.
/// There is no request/manifest constructor for this operation carrier.
pub struct NativeActOperation {
    channel: QualifiedNativeActChannel,
    request: Value,
}
impl NativeActOperation {
    pub(crate) fn request(&self) -> &Value {
        &self.request
    }
    pub(crate) fn qualification(&self) -> &Value {
        self.channel.qualification()
    }
    pub(crate) fn read_value(&mut self) -> Result<Value, String> {
        self.channel.read_value()
    }
    pub(crate) fn write_value(&mut self, value: &Value) -> Result<(), String> {
        self.channel.write_value(value)
    }
    pub fn into_channel(self) -> QualifiedNativeActChannel {
        self.channel
    }
}
impl QualifiedNativeActChannel {
    pub fn establish(path: &Path) -> Result<Self, String> {
        let (qualifier, cut) = locate_qualifier()?;
        let stream = UnixStream::connect(path).map_err(|e| e.to_string())?;
        stream
            .set_read_timeout(Some(TIMEOUT))
            .map_err(|e| e.to_string())?;
        stream
            .set_write_timeout(Some(TIMEOUT))
            .map_err(|e| e.to_string())?;
        let duplicate = stream.try_clone().map_err(|e| e.to_string())?;
        let mut command = Command::new(qualifier);
        command
            .args(["desktop", "native-parent-qualify", "--json"])
            .stdin(Stdio::from(OwnedFd::from(duplicate)));
        let qualification: Value =
            serde_json::from_slice(&execute(command)?).map_err(|e| e.to_string())?;
        if qualification["schema"] != "oi.native-parent-channel-qualification/v1"
            || qualification["qualified"] != true
            || qualification["ql_host"]["process"]["pid"].as_u64()
                != Some(u64::from(std::process::id()))
            || qualification["peer"]["pid"] != qualification["supervisor"]["process"]["pid"]
            || qualification["cut"]["source_receipt_sha256"] != cut["source_receipt_sha256"]
            || qualification["cut"]["standing"] != cut["standing"]
        {
            return Err("native channel actual qualifier acknowledgement differs".into());
        }
        let writer = stream.try_clone().map_err(|e| e.to_string())?;
        let mut channel = Self {
            reader: BufReader::new(stream),
            writer,
            qualification,
        };
        channel.write_value(&json!({"schema":"ql.native-act-channel-ready/v1","qualification":channel.qualification}))?;
        Ok(channel)
    }
    pub(crate) fn qualification(&self) -> &Value {
        &self.qualification
    }
    pub fn next_operation(mut self) -> Result<NativeActOperation, String> {
        // An idle native owner is allowed to remain open. The finite control
        // timeout starts after the first byte, rather than ending an idle app.
        self.reader
            .get_ref()
            .set_read_timeout(None)
            .map_err(|e| e.to_string())?;
        if self
            .reader
            .fill_buf()
            .map_err(|e| e.to_string())?
            .is_empty()
        {
            return Err("native qualified Act channel closed".into());
        }
        self.reader
            .get_ref()
            .set_read_timeout(Some(TIMEOUT))
            .map_err(|e| e.to_string())?;
        let request = self.read_value()?;
        Ok(NativeActOperation {
            channel: self,
            request,
        })
    }
    pub(crate) fn read_value(&mut self) -> Result<Value, String> {
        let mut bytes = Vec::new();
        let n = self
            .reader
            .by_ref()
            .take(REQUEST as u64 + 1)
            .read_until(b'\n', &mut bytes)
            .map_err(|e| e.to_string())?;
        if n == 0 || bytes.len() > REQUEST || !bytes.ends_with(b"\n") {
            return Err("native qualified Act channel framing unavailable".into());
        }
        serde_json::from_slice(&bytes).map_err(|e| e.to_string())
    }
    pub(crate) fn write_value(&mut self, value: &Value) -> Result<(), String> {
        let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
        if bytes.len() > REPLY {
            return Err("native qualified Act reply exceeds bound".into());
        }
        self.writer
            .write_all(&bytes)
            .and_then(|_| self.writer.write_all(b"\n"))
            .and_then(|_| self.writer.flush())
            .map_err(|e| e.to_string())
    }
}
