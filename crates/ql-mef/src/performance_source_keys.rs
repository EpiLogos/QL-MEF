//! Existing native performance consumes actual B/K sparse source targets.
//! Physical addresses, source degrees and the actual M2 octet stay distinct.
//! The control owner retains complete source receipts; callback targets remain
//! bounded typed notes, with no JSON/theory/body preparation in the callback.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::MCoordinate;
use crate::m2_engine::M2Request;
use crate::m2_tuning_sources::TuningSourceCollection;
use crate::performance_audio::{KeyTouch, PreparedPerformanceBinding};
use crate::performance_management::{encode_performance_note, native_janko_addresses};
use crate::source_key_determination::{
    SourceKeyTarget, SparseConditionConsumer, SparseKeyTargets, SparseMusicalConsumer,
};

pub const SPARSE_PERFORMANCE: &str = "ql.source-performance-admission/v1";

#[derive(Clone, Debug, Serialize)]
pub struct NativeSourceKeyboardCell {
    pub row: u8,
    pub column: u8,
    pub register_octave: i8,
    pub key: u8,
    pub pitch_class: u8,
    pub label: String,
    pub available: bool,
    pub hertz: Option<f64>,
    pub coordinate: Option<String>,
    pub face: Option<u8>,
    pub ratio: Option<Value>,
    pub source_degree: Option<u16>,
    pub reason: Option<String>,
    pub reduction_policy: String,
    pub source_collection: String,
    pub source_receipt: String,
}

/// Constructed only from actual immutable native producers, never from JSON.
/// The existing source/Act owner grants the current instance and context before
/// invoking this consumer. Retention hashes are identifiers, not authority.
#[derive(Clone)]
pub struct PreparedSourcePerformance {
    base: Arc<PreparedPerformanceBinding>,
    targets: SparseKeyTargets,
    collection: TuningSourceCollection,
    writer: MCoordinate,
    phase: u8,
    original_condition_input: Option<M2Request>,
}
impl PreparedSourcePerformance {
    pub fn admit(
        base: Arc<PreparedPerformanceBinding>,
        targets: SparseKeyTargets,
        collection: &TuningSourceCollection,
        consumer: SparseMusicalConsumer<'_>,
    ) -> Result<Self, String> {
        base.validate_native_consumers(consumer.basis, base.physical_body())?;
        targets.validate_coupled_consumer(
            SparseMusicalConsumer {
                basis: consumer.basis,
                writer: consumer.writer,
                phase: consumer.phase,
                condition: consumer
                    .condition
                    .as_ref()
                    .map(|c| SparseConditionConsumer {
                        producer_input: c.producer_input,
                        plan: c.plan,
                    }),
            },
            collection,
        )?;
        let out = Self {
            base,
            targets,
            collection: collection.clone(),
            writer: consumer.writer.clone(),
            phase: consumer.phase,
            original_condition_input: consumer.condition.map(|c| c.producer_input.clone()),
        };
        out.validate_current(out.base.as_ref(), &out.collection)?;
        Ok(out)
    }
    pub fn base(&self) -> &PreparedPerformanceBinding {
        &self.base
    }
    pub fn targets(&self) -> &SparseKeyTargets {
        &self.targets
    }
    pub fn validate_current(
        &self,
        current: &PreparedPerformanceBinding,
        collection: &TuningSourceCollection,
    ) -> Result<(), String> {
        self.base
            .validate_native_consumers(current.native_basis(), current.physical_body())?;
        self.targets.validate_coupled_consumer(
            SparseMusicalConsumer {
                basis: current.native_basis(),
                writer: &self.writer,
                phase: self.phase,
                condition: self.original_condition_input.as_ref().map(|original| {
                    SparseConditionConsumer {
                        producer_input: original,
                        plan: current.relation_plan(),
                    }
                }),
            },
            collection,
        )
    }
    pub fn source_preparation(&self) -> Result<Value, String> {
        self.targets.preparation_receipt()
    }
    fn receipt_ref(receipt: &Value) -> Result<String, String> {
        let bytes = serde_json::to_vec(receipt).map_err(|e| e.to_string())?;
        Ok(format!(
            "ql:source-key-receipt/sha256/{:x}",
            Sha256::digest(bytes)
        ))
    }
    /// A missing source address produces its actual unavailable receipt and no
    /// numerical target. The caller must not enqueue a NoteOn in that branch.
    pub fn resolve_touch(&self, touch: KeyTouch) -> Result<SourcePerformanceTouch, String> {
        if touch.member == 0 || touch.touch == 0 {
            return Err("nonzero native source member/touch required".into());
        }
        self.validate_current(&self.base, &self.collection)?;
        let selected = self
            .targets
            .key_target(touch.key, touch.register, &touch.touch_ref)?;
        let receipt = selected.receipt()?;
        let Some(source_note) = selected.note() else {
            return Ok(SourcePerformanceTouch::Unavailable { receipt });
        };
        let rate = f64::from(self.base.physical_body().request().sample_rate);
        if source_note.hertz < 0.001 || source_note.hertz >= rate * 0.45 {
            return Err("source note outside admitted native audio band".into());
        }
        // Retain the actual M1 carrier phase/source identity while using the
        // actual B/K source target. No pitch-class-to-source-degree inference.
        let mut native = encode_performance_note(&self.base, source_note, touch)?;
        native["hertz"] = json!(source_note.hertz);
        native["fundamental_hz"] = json!(source_note.fundamental.hertz());
        native["tuning_ref"] = json!(source_note.tuning_provenance.policy_ref);
        let (n, d) = source_note
            .exact_ratio
            .map(|r| (r.numerator().to_string(), r.denominator().to_string()))
            .unwrap_or_else(|| ("0".into(), "0".into()));
        native["ratio_numerator"] = json!(n);
        native["ratio_denominator"] = json!(d);
        native["exact_ratio"] = json!(source_note.exact_ratio.is_some());
        Ok(SourcePerformanceTouch::Available {
            native_target: native,
            receipt,
        })
    }
    pub fn catalog(
        &self,
        columns: u8,
        base_register: i8,
        transpose: u8,
    ) -> Result<Vec<NativeSourceKeyboardCell>, String> {
        self.validate_current(&self.base, &self.collection)?;
        // Existing native K provides the current architectural pitch substrate
        // and Janko geometry; source availability never changes that address.
        let addresses = native_janko_addresses(&self.base, columns, base_register, transpose)?;
        addresses.into_iter().map(|address| {
            let target = self.targets.key_target(address.key, address.register_octave, "native:source-catalog/lookup")?;
            let receipt = target.receipt()?;
            let policy = receipt["reduction_policy"]["policy_ref"].as_str()
                .ok_or("native source reduction policy absent")?.to_owned();
            let receipt_ref = Self::receipt_ref(&receipt)?;
            let (available,hertz,coordinate,face,ratio,degree,reason,label) = match target {
                SourceKeyTarget::Available(source) => {
                    let note = source.note();
                    if note.hertz < 0.001 || note.hertz >= f64::from(self.base.physical_body().request().sample_rate)*0.45 {
                        return Err("source catalog outside admitted native audio band".into());
                    }
                    (true,Some(note.hertz),Some(note.source_coordinate.source_ref.clone()),
                     Some(u8::from(note.source_coordinate.face == crate::MFace::Pratibimba)),
                     note.exact_ratio.map(|r|json!({"numerator":r.numerator().to_string(),"denominator":r.denominator().to_string()})),
                     Some(source.assignment().source_degree),None,
                     format!("Degree {} / {}",u32::from(source.assignment().source_degree)+1,address.register_octave))
                }
                SourceKeyTarget::Unavailable(source) =>
                    (false,None,None,None,None,source.source_degree(),Some(source.reason().to_owned()),"Unassigned".into()),
            };
            Ok(NativeSourceKeyboardCell {row:address.row,column:address.column,
                register_octave:address.register_octave,key:address.key,pitch_class:address.pitch_class,
                label,available,hertz,coordinate,face,ratio,source_degree:degree,reason,
                reduction_policy:policy,source_collection:self.collection.collection_ref().into(),source_receipt:receipt_ref})
        }).collect()
    }
    /// Prepared packet for the existing actual native A/P consumer. The full
    /// sparse preparation/source receipts remain control-owned C source assets.
    /// Actual M2/Vimarsha octet and quartet remain exactly producer outputs.
    pub fn native_binding_for_touches(
        &self,
        touches: &[KeyTouch],
    ) -> Result<PreparedPerformanceBinding, String> {
        self.base.admit_source_key_targets(self, touches)
    }
    pub fn packet_for_touches(&self, touches: &[KeyTouch]) -> Result<Value, String> {
        if touches.is_empty() || touches.len() > crate::performance_audio::MAX_TOUCHES {
            return Err("bounded available native source touches required".into());
        }
        let mut notes = Vec::new();
        let mut receipts = Vec::new();
        let mut tokens = BTreeSet::new();
        let mut members = BTreeMap::new();
        let mut tuning = None;
        for touch in touches {
            if !tokens.insert(touch.touch) {
                return Err("source touch token repeated".into());
            }
            if let Some(old) = members.insert(touch.member, (touch.key, touch.register)) {
                if old != (touch.key, touch.register) {
                    return Err("source member changed address".into());
                }
            }
            match self.resolve_touch(touch.clone())? {
                SourcePerformanceTouch::Unavailable { .. } => {
                    return Err(
                        "unavailable source key cannot create a native NoteOn packet".into(),
                    );
                }
                SourcePerformanceTouch::Available {
                    native_target,
                    receipt,
                } => {
                    let reference = native_target["tuning_ref"]
                        .as_str()
                        .ok_or("source tuning policy missing")?
                        .to_owned();
                    if tuning.as_ref().is_some_and(|old| old != &reference) {
                        return Err("source tuning policies differ".into());
                    }
                    tuning = Some(reference);
                    notes.push(native_target);
                    receipts.push(receipt);
                }
            }
        }
        let mut packet = serde_json::to_value(self.base.as_ref()).map_err(|e| e.to_string())?;
        packet["determination"]["tuning_ref"] =
            json!(tuning.ok_or("available source target absent")?);
        packet["notes"] = json!(notes);
        packet["source_key_admission"] = json!({"schema":SPARSE_PERFORMANCE,
            "preparation":self.source_preparation()?,"touch_receipts":receipts,
            "physical_standing":"unchanged actual prepared physical body; no eigenmode retuning",
            "audio_octet_source":"unchanged actual M2/Vimarsha bus; source-component reduction is separate"});
        Ok(packet)
    }
}
#[derive(Clone, Debug)]
pub enum SourcePerformanceTouch {
    Available {
        native_target: Value,
        receipt: Value,
    },
    Unavailable {
        receipt: Value,
    },
}
