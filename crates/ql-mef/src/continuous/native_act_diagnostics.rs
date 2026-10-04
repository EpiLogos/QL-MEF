//! Individual original parsed native receipts on the SAME private Act channel.
//! No aggregate serialization, source authority, request clock or public grant.
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::io::{self, Write};

const CHUNK: usize = 256 * 1024;
const RECEIPT: u64 = super::host::MAX_HOST_OUTPUT as u64;
const FRAMES: u64 = 16384;
const ENCODING: &str = "serde-json-value-utf8/v1";
const PART: &str = "ql.native-act-owner-diagnostic-part/v1";
const ACK: &str = "oi.native-act-owner-diagnostic-ack/v1";
const MANIFEST: &str = "ql.native-act-owner-diagnostics/v1";

fn count(value: &Value) -> Result<u64, String> {
    let s = value.as_str().ok_or("diagnostic counter is not text")?;
    let n: u64 = s.parse().map_err(|_| "invalid diagnostic counter")?;
    if n.to_string() != s {
        return Err("noncanonical diagnostic counter".into());
    }
    Ok(n)
}
fn kind(value: &str) -> Result<(), String> {
    let scalar = [
        "recording.cut_observation",
        "recording.cut_checkpoint",
        "recording.cut_failure",
        "original_capture_receipt",
        "saved_native_checkpoint",
        "last_activity_reply",
        "before_restoration_receipt",
        "restoration_reply",
        "after_restoration_receipt",
        "failed_native_receipts",
    ];
    let restoration = [
        "transport_acknowledgement",
        "restored_receipts",
        "original_capture_receipt",
        "saved_native_checkpoint",
        "before_restoration_checkpoint",
        "after_restoration_checkpoint",
        "restored_applications",
        "restored_input_history",
    ];
    let allowed = scalar.contains(&value)
        || ["restoration.", "successful_restoration."]
            .iter()
            .any(|prefix| {
                value
                    .strip_prefix(prefix)
                    .is_some_and(|field| restoration.contains(&field))
            });
    if !allowed {
        return Err("diagnostic kind is not an original export receipt field".into());
    }
    Ok(())
}
struct Measure {
    bytes: u64,
    digest: Sha256,
}
impl Write for Measure {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes = self
            .bytes
            .checked_add(bytes.len() as u64)
            .filter(|n| *n <= RECEIPT)
            .ok_or_else(|| io::Error::other("native receipt exceeds existing 64 MiB bound"))?;
        self.digest.update(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Constructed only by the privately qualified Act operation handler. This
/// type transports existing Values; it cannot grant source/playback custody.
pub(crate) struct NativeDiagnosticSender<'a, S, A> {
    instance: &'a str,
    request: &'a str,
    send: S,
    answer: A,
    manifest: Vec<Value>,
    frames: u64,
}
impl<'a, S: FnMut(&Value) -> Result<(), String>, A: FnMut() -> Result<Value, String>>
    NativeDiagnosticSender<'a, S, A>
{
    pub(crate) fn new(
        instance: &'a str,
        request: &'a str,
        send: S,
        answer: A,
    ) -> Result<Self, String> {
        if instance.is_empty()
            || instance.len() > 4096
            || instance.chars().any(char::is_control)
            || count(&json!(request))? == 0
        {
            return Err("native diagnostic scope absent".into());
        }
        Ok(Self {
            instance,
            request,
            send,
            answer,
            manifest: Vec::new(),
            frames: 0,
        })
    }
    /// Visits exactly one original field/row. Each vector index remains its
    /// ORIGINAL index; ordinals are transport order only.
    pub(crate) fn send_receipt(
        &mut self,
        field: &str,
        index: usize,
        receipt: &Value,
    ) -> Result<(), String> {
        kind(field)?;
        if self
            .manifest
            .iter()
            .any(|m| m["kind"] == field && m["original_index"] == json!(index.to_string()))
        {
            return Err("original diagnostic receipt repeated".into());
        }
        let mut measure = Measure {
            bytes: 0,
            digest: Sha256::new(),
        };
        receipt
            .serialize(&mut serde_json::Serializer::new(&mut measure))
            .map_err(|e| e.to_string())?;
        if measure.bytes == 0 {
            return Err("empty native diagnostic receipt".into());
        }
        let hash = format!("sha256:{:x}", measure.digest.finalize());
        let ordinal = self.manifest.len() as u64 + 1;
        let mut stream = ReceiptStream {
            sender: self,
            field,
            index,
            ordinal,
            total: measure.bytes,
            hash: hash.clone(),
            pending: Vec::with_capacity(CHUNK + 4),
            offset: 0,
            part: 0,
        };
        receipt
            .serialize(&mut serde_json::Serializer::new(&mut stream))
            .map_err(|e| e.to_string())?;
        stream.finish()?;
        let parts = stream.part;
        self.manifest.push(json!({"receipt_ordinal":ordinal.to_string(),"kind":field,
            "original_index":index.to_string(),"bytes":measure.bytes.to_string(),"sha256":hash,"parts":parts.to_string()}));
        Ok(())
    }
    pub(crate) fn manifest(&self) -> Value {
        json!({"schema":MANIFEST,"encoding":ENCODING,"receipts":self.manifest})
    }
}
struct ReceiptStream<'s, 'a, S, A> {
    sender: &'s mut NativeDiagnosticSender<'a, S, A>,
    field: &'s str,
    index: usize,
    ordinal: u64,
    total: u64,
    hash: String,
    pending: Vec<u8>,
    offset: u64,
    part: u64,
}
impl<S: FnMut(&Value) -> Result<(), String>, A: FnMut() -> Result<Value, String>>
    ReceiptStream<'_, '_, S, A>
{
    fn emit(&mut self, n: usize, final_part: bool) -> Result<(), String> {
        if n == 0 || n > CHUNK || self.sender.frames >= FRAMES {
            return Err("native diagnostic finite frame custody exhausted".into());
        }
        let chunk = std::str::from_utf8(&self.pending[..n]).map_err(|e| e.to_string())?;
        let frame = json!({"schema":PART,"instance_ref":self.sender.instance,"request_id":self.sender.request,
            "receipt_ordinal":self.ordinal.to_string(),"kind":self.field,"original_index":self.index.to_string(),
            "encoding":ENCODING,"part_ordinal":self.part.to_string(),"byte_offset":self.offset.to_string(),
            "total_bytes":self.total.to_string(),"sha256":self.hash,"chunk":chunk,"final_part":final_part});
        if serde_json::to_vec(&frame).map_err(|e| e.to_string())?.len() as u64
            > super::host::MAX_HOST_INPUT
        {
            return Err("native diagnostic frame exceeds existing 32 MiB bound".into());
        }
        (self.sender.send)(&frame)?;
        let next = self
            .offset
            .checked_add(n as u64)
            .ok_or("diagnostic byte offset exhausted")?;
        let ack = (self.sender.answer)()?;
        if ack.as_object().map(|o| o.len()) != Some(10)
            || ack["schema"] != ACK
            || ack["instance_ref"] != self.sender.instance
            || ack["request_id"] != self.sender.request
            || ack["kind"] != self.field
            || count(&ack["receipt_ordinal"])? != self.ordinal
            || count(&ack["original_index"])? != self.index as u64
            || count(&ack["part_ordinal"])? != self.part
            || count(&ack["next_byte_offset"])? != next
            || ack["sha256"] != self.hash
            || ack["retained"] != true
        {
            return Err("native original receipt chunk was not retained by the same owner".into());
        }
        self.sender.frames += 1;
        self.part += 1;
        self.offset = next;
        self.pending.drain(..n);
        Ok(())
    }
    fn finish(&mut self) -> Result<(), String> {
        if self.offset + self.pending.len() as u64 != self.total {
            return Err("native receipt serialization changed length".into());
        }
        self.emit(self.pending.len(), true)
    }
}
impl<S: FnMut(&Value) -> Result<(), String>, A: FnMut() -> Result<Value, String>> Write
    for ReceiptStream<'_, '_, S, A>
{
    fn write(&mut self, mut bytes: &[u8]) -> io::Result<usize> {
        let original = bytes.len();
        while !bytes.is_empty() {
            let n = bytes.len().min(CHUNK + 4 - self.pending.len());
            self.pending.extend_from_slice(&bytes[..n]);
            bytes = &bytes[n..];
            if self.pending.len() > CHUNK {
                let cut = match std::str::from_utf8(&self.pending[..CHUNK]) {
                    Ok(_) => CHUNK,
                    Err(error) if error.error_len().is_none() => error.valid_up_to(),
                    Err(error) => return Err(io::Error::other(error)),
                };
                self.emit(cut, false).map_err(io::Error::other)?;
            }
        }
        Ok(original)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
