//! Bounded original-evidence serialization and finite numeric bit equality.
//! These helpers issue no source, constructor, clock, occurrence or lease.
use serde::Serialize;
use serde_json::Value;
use std::io::{self, Write};

struct CountWriter {
    used: usize,
    limit: usize,
}
impl Write for CountWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let next = self
            .used
            .checked_add(bytes.len())
            .filter(|n| *n <= self.limit)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "complete original evidence exceeds native transport bound",
                )
            })?;
        self.used = next;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
/// Charge escaped JSON, including object keys and punctuation, through a
/// nonallocating writer. The serializer stops before accepting excess bytes.
pub(crate) fn encoded_bound<T: Serialize + ?Sized>(
    value: &T,
    limit: usize,
    name: &str,
) -> Result<usize, String> {
    let mut count = CountWriter { used: 0, limit };
    serde_json::to_writer(&mut count, value)
        .map_err(|e| format!("complete original {name}: {e}"))?;
    Ok(count.used)
}
/// Compare original numeric representations and finite f64 bits recursively;
/// signed zero and integer precision cannot disappear behind Value equality.
pub(crate) fn exact_value(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) if a.is_f64() || b.is_f64() => {
            a.is_f64()
                && b.is_f64()
                && a.as_f64().zip(b.as_f64()).is_some_and(|(a, b)| {
                    a.is_finite() && b.is_finite() && a.to_bits() == b.to_bits()
                })
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| exact_value(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(key, a)| b.get(key).is_some_and(|b| exact_value(a, b)))
        }
        _ => a == b,
    }
}
#[derive(Serialize)]
struct OriginalContactEvidence<'a> {
    schema: &'static str,
    original_request: &'a Value,
    original_reply: &'a Value,
}
/// Preserve the exact originals only after charging BOTH full borrowed values
/// together, with JSON escape expansion, against the original 32MiB envelope.
pub(crate) fn contact_checkpoint_evidence(request: &Value, reply: &Value) -> Result<Value, String> {
    let original = OriginalContactEvidence {
        schema: "ql.native-scene-contact-checkpoint-evidence/v1",
        original_request: request,
        original_reply: reply,
    };
    encoded_bound(
        &original,
        crate::continuous::MAX_MESSAGE,
        "Contact checkpoint evidence",
    )?;
    serde_json::to_value(original).map_err(|e| e.to_string())
}
