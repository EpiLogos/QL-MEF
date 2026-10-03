//! Additive management of the existing retained worker. Only native source
//! producers construct callback packets. The UI supplies gestures/magnitudes,
//! never a NoteTarget, physical authority, queue ordinal or sample clock.
use super::coupled::{CoupledBasis, CoupledFieldSession};
use crate::MFace;
use crate::m2_relation_plan::M2RelationPlanContext;
use crate::m2_tuning_sources::retained_condition_collection;
use crate::m3_state::M3State;
use crate::music_determination::{
    ExactRatio, Fundamental, RelationSelection, TuningPolicy, TuningProvenance,
};
use crate::performance_audio::{
    self, ExcitationPolicy, FundamentalScaling, KeyTouch, OctetScaling,
    PerformancePreparationInput, PreparedPerformanceBinding,
};
use crate::performance_management::{native_janko_catalog, resolve_performance_touch};
use crate::performance_source_keys::{PreparedSourcePerformance, SourcePerformanceTouch};
use crate::source_form_body::{
    SourceBodyControls, SourceGeometryRecipe, prepare_source_form_body, source_form_coordinate,
};
use crate::source_key_determination::{
    ActiveKeyReduction, SourceOctetReduction, SourcePitchRequirement, SparseConditionConsumer,
    SparseKeyPreparation, SparseKeyTargets, SparseMusicalConsumer,
};
use ql_core::{ConjugationDegree, ExpansionSide, RelationFamily};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;

pub const CONTROL: &str = "ql.performance-control/v1";
pub const REPLY: &str = "ql.performance-management-reply/v1";
fn bounded(s: &str) -> Result<(), String> {
    if s.is_empty() || s.len() >= 256 || s.chars().any(char::is_control) {
        Err("bounded native performance reference required".into())
    } else {
        Ok(())
    }
}
fn face(v: u8) -> Result<MFace, String> {
    match v {
        0 => Ok(MFace::Bimba),
        1 => Ok(MFace::Pratibimba),
        _ => Err("native face must be Bimba or Pratibimba".into()),
    }
}
fn decimal(v: &Value) -> Result<u64, String> {
    let s = v
        .as_str()
        .ok_or("canonical decimal native counter required")?;
    let n: u64 = s.parse().map_err(|_| "invalid native counter")?;
    if n.to_string() != s {
        return Err("noncanonical native counter".into());
    }
    Ok(n)
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RelationConfig {
    pub family: String,
    pub pair_index: u8,
    pub degree: u8,
    pub expansion_side: Option<String>,
}
impl RelationConfig {
    fn native(&self) -> Result<RelationSelection, String> {
        Ok(RelationSelection {
            family: match self.family.as_str() {
                "A" => RelationFamily::A,
                "B" => RelationFamily::B,
                "C" => RelationFamily::C,
                _ => return Err("unknown native relation family".into()),
            },
            pair_index: self.pair_index,
            degree: match self.degree {
                1 => ConjugationDegree::D1,
                2 => ConjugationDegree::D2,
                3 => ConjugationDegree::D3,
                _ => return Err("unknown native conjugation degree".into()),
            },
            expansion_side: match self.expansion_side.as_deref() {
                None => None,
                Some("left") => Some(ExpansionSide::Left),
                Some("right") => Some(ExpansionSide::Right),
                _ => return Err("unknown native expansion side".into()),
            },
        })
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "policy", rename_all = "kebab-case", deny_unknown_fields)]
pub enum TuningConfig {
    ExactPitchRatios {
        ratios: [ExactRatio; 12],
        provenance: TuningProvenance,
    },
    EqualTemperament12 {
        provenance: TuningProvenance,
    },
}
impl TuningConfig {
    fn native(&self) -> TuningPolicy {
        match self {
            Self::ExactPitchRatios { ratios, provenance } => TuningPolicy::ExactPitchRatios {
                ratios: *ratios,
                provenance: provenance.clone(),
            },
            Self::EqualTemperament12 { provenance } => TuningPolicy::EqualTemperament12 {
                provenance: provenance.clone(),
            },
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExcitationConfig {
    pub policy_ref: String,
    pub standing: String,
    pub scaling: u8,
    pub reference_hertz: f64,
    pub root_linear: f64,
    pub octet_linear: f64,
    pub weights: [f64; 8],
}
impl ExcitationConfig {
    fn native(&self) -> Result<ExcitationPolicy, String> {
        Ok(ExcitationPolicy {
            policy_ref: self.policy_ref.clone(),
            standing: self.standing.clone(),
            scaling: match self.scaling {
                0 => OctetScaling::NoteRelativeToReference,
                1 => OctetScaling::AbsoluteBus,
                _ => return Err("unknown native octet scaling policy".into()),
            },
            reference_hertz: self.reference_hertz,
            root_linear: self.root_linear,
            octet_linear: self.octet_linear,
            weights: self.weights,
        })
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SparseConditionConfig {
    pub reduction: ActiveKeyReduction,
    pub octet: Option<SourceOctetReduction>,
    pub requirement: SourcePitchRequirement,
}
/// All D30 magnitudes and source reduction are explicit retained inputs. This
/// type deliberately has no browser-carried M1/M2/M3/body/source snapshot.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PerformanceConfig {
    pub schema: String,
    pub session_ref: String,
    pub receipt_ref: String,
    pub projection_ref: String,
    pub relation_context: M2RelationPlanContext,
    pub relation: RelationConfig,
    pub source_face: u8,
    pub physical_face: u8,
    pub excitation: ExcitationConfig,
    pub fundamental_hertz: f64,
    pub fundamental_provenance: TuningProvenance,
    pub use_native_m1_harmonic_ratio: bool,
    pub tuning: TuningConfig,
    pub recipe: SourceGeometryRecipe,
    pub controls: SourceBodyControls,
    pub columns: u8,
    pub base_register: i8,
    pub transpose: u8,
    pub sparse_condition: Option<SparseConditionConfig>,
}
/// Prepared once from the retained native owner; no oscillator/body/clock
/// resides here. The resident C++ owner alone advances and captures q/v.
pub struct PerformanceOwner {
    original_input: Value,
    binding: Arc<PreparedPerformanceBinding>,
    sparse: Option<PreparedSourcePerformance>,
    config: PerformanceConfig,
    cells: Vec<Value>,
    return_binding: Option<PreparedPerformanceBinding>,
    source_assets: Value,
    last: Option<Value>,
}
/// Complete parsed stopped-owner replies on refusal; never source authority.
pub(crate) struct NativeStoppedExchangeFailure {
    pub(crate) reason: String,
    pub(crate) native_receipts: Vec<Value>,
}
impl From<String> for NativeStoppedExchangeFailure {
    fn from(reason: String) -> Self {
        Self {
            reason,
            native_receipts: Vec::new(),
        }
    }
}
impl From<&str> for NativeStoppedExchangeFailure {
    fn from(reason: &str) -> Self {
        Self::from(reason.to_owned())
    }
}

impl From<(String, Option<Value>)> for NativeStoppedExchangeFailure {
    fn from((reason, native_receipt): (String, Option<Value>)) -> Self {
        Self {
            reason,
            native_receipts: native_receipt.into_iter().collect(),
        }
    }
}

impl PerformanceOwner {
    pub fn prepare(
        current: &CoupledBasis,
        instance: &str,
        config: PerformanceConfig,
    ) -> Result<Self, String> {
        if config.schema != "ql.retained-source-performance-config/v1" {
            return Err("native performance configuration schema differs".into());
        }
        for s in [
            &config.session_ref,
            &config.receipt_ref,
            &config.projection_ref,
        ] {
            bounded(s)?;
        }
        if config.controls.sample_rate != 48000
            || config.controls.body_revision == 0
            || !(6..=32).contains(&config.columns)
            || config.transpose > 11
        {
            return Err("native performance dimensions/format invalid".into());
        }
        let original_input = serde_json::to_value(&current.input).map_err(|e| e.to_string())?;
        let mut projected = current.input.clone();
        let legacy = json!({"frequency_bindings":projected.frequency_bindings,"condition_frequency_bindings":projected.condition_frequency_bindings,"sky_frequency_bindings":projected.sky_frequency_bindings});
        projected.frequency_bindings.clear();
        projected.condition_frequency_bindings.clear();
        projected.sky_frequency_bindings.clear();
        // Preserve the original engines, native commands and occasion. This receipt
        // declares a different physical consumer role, not a new event or source.
        let projection = json!({"schema":"ql.retained-physical-consumer-projection/v1","projection_ref":config.projection_ref,"original_native_input":original_input,"legacy_mode_frequency_bindings":legacy,"policy":"excitation pitches do not retune physical eigenmodes"});
        projected.source_receipts.push(projection.clone());
        let mut state = M3State::new(projected.m3.clone())?;
        for command in &projected.m3_commands {
            let receipt = state.apply(command.clone())?;
            if receipt.status != "applied" {
                return Err("retained native M3 command refused".into());
            }
        }
        if config.controls.expected_m3_generation != state.generation() {
            return Err("source body controls are stale against actual retained M3".into());
        }
        let physical = prepare_source_form_body(
            &state,
            source_form_coordinate(&state, face(config.physical_face)?)?,
            config.recipe.clone(),
            config.controls.clone(),
        )?;
        physical.validate_source_geometry(&state)?;
        let original_projected = projected.compose()?;
        let binding = Arc::new(performance_audio::prepare_source_form_performance(
            PerformancePreparationInput {
                coupled: projected,
                relation_context: config.relation_context.clone(),
                source_face: face(config.source_face)?,
                physical_face: face(config.physical_face)?,
                excitation: config.excitation.native()?,
                relation: config.relation.native()?,
                fundamental: Fundamental::new(
                    config.fundamental_hertz,
                    config.fundamental_provenance.clone(),
                )?,
                fundamental_scaling: if config.use_native_m1_harmonic_ratio {
                    FundamentalScaling::NativeM1HarmonicRatio
                } else {
                    FundamentalScaling::AbsoluteReference
                },
                tuning: config.tuning.native(),
                require_authentic_condition_tuning: false,
                physical: physical.body().request().clone(),
                instance_ref: instance.into(),
                receipt_ref: config.receipt_ref.clone(),
                touches: (0..12)
                    .map(|key| KeyTouch {
                        key,
                        register: 0,
                        member: u64::from(key) + 1,
                        touch: u64::from(key) + 1,
                        touch_ref: format!("native:performance/prepared-key/{key}"),
                    })
                    .collect(),
            },
            config.recipe.clone(),
        )?);
        binding.validate_source_form_consumer(&state)?;
        let sparse = if let Some(choice) = &config.sparse_condition {
            let condition = &binding.relation_plan().execution.condition_input;
            let collection = retained_condition_collection(
                condition.maqam_index,
                condition.role,
                &binding.targets().fundamental,
            )?;
            let writer =
                crate::m_tree::native_current_m_registry().coordinate("#2-1", MFace::Pratibimba)?;
            let phase = binding.targets().determination.identity().tick12() / 6;
            let consumer = || SparseMusicalConsumer {
                basis: binding.native_basis(),
                writer: &writer,
                phase,
                condition: Some(SparseConditionConsumer {
                    producer_input: &original_projected.m2_input,
                    plan: binding.relation_plan(),
                }),
            };
            let targets = SparseKeyTargets::prepare(SparseKeyPreparation {
                determination: binding.targets().determination.clone(),
                collection: collection.clone(),
                reduction: choice.reduction.clone(),
                octet: choice.octet.clone(),
                requirement: choice.requirement,
                consumer: consumer(),
            })?;
            Some(PreparedSourcePerformance::admit(
                Arc::clone(&binding),
                targets,
                &collection,
                consumer(),
            )?)
        } else {
            None
        };
        let source_assets = json!({"schema":"ql.retained-performance-source-assets/v1","original_native_input":original_input,"physical_consumer_projection":projection,"native_basis":binding.native_basis(),"source_form_recipe":config.recipe,"source_geometry_reading":physical.source_reading(),"source_key_preparation":sparse.as_ref().map(|s|s.source_preparation()).transpose()?,"configuration":config,"source_context":{"available":false,"reason":"existing native receiving owner has not classified every original source receipt for this source bundle; no public disclosure authority inferred from World labels"},"consumer_roles":{"physical":"canonical-source-form-scalar-excitation","legacy_mode_frequency_remapping":"retired-in-this-explicit-physical-projection","personal_nine_force_routes":{"available":false,"reason":"original N determination and P9 callback admission not joined"},"sky_ten_source_forcing":{"available":false,"reason":"retained original sky contributors are source assets; distinct common-body forcing not joined"}}});
        let mut out = Self {
            original_input,
            binding,
            sparse,
            config,
            cells: vec![],
            return_binding: None,
            source_assets,
            last: None,
        };
        out.cells = out.catalog()?;
        if let Some(source) = &out.sparse {
            let touches = out.available_source_preparation_touches()?;
            out.return_binding = Some(source.native_binding_for_touches(&touches)?);
        }
        Ok(out)
    }
    fn validate_current(&self, current: &CoupledBasis) -> Result<(), String> {
        if serde_json::to_value(&current.input).map_err(|e| e.to_string())? != self.original_input {
            return Err(
                "retained native source changed; prepared source/body transaction required".into(),
            );
        }
        self.binding
            .validate_native_consumers(self.binding.native_basis(), self.binding.physical_body())?;
        Ok(())
    }
    fn catalog(&self) -> Result<Vec<Value>, String> {
        let native = if let Some(s) = &self.sparse {
            s.catalog(
                self.config.columns,
                self.config.base_register,
                self.config.transpose,
            )?
            .into_iter()
            .map(|k| serde_json::to_value(k).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?
        } else {
            native_janko_catalog(
                &self.binding,
                self.config.columns,
                self.config.base_register,
                self.config.transpose,
            )?
            .into_iter()
            .map(|k| {
                let mut v = serde_json::to_value(k).map_err(|e| e.to_string())?;
                v["available"] = json!(true);
                v["source_degree"] = Value::Null;
                v["reason"] = Value::Null;
                v["reduction_policy"] =
                    json!(self.binding.targets().tuning_policy.provenance().policy_ref);
                v["source_collection"] = json!("ql:canonical-twelve-key-field");
                v["source_receipt"] = json!(self.config.receipt_ref);
                Ok(v)
            })
            .collect::<Result<Vec<_>, String>>()?
        };
        native
            .into_iter()
            .map(|mut cell| {
                let available = cell["available"]
                    .as_bool()
                    .ok_or("native source availability absent")?;
                let note = if available {
                    Some(
                        self.resolve(KeyTouch {
                            key: cell["key"].as_u64().ok_or("native catalog key absent")? as u8,
                            register: cell["register_octave"]
                                .as_i64()
                                .ok_or("native catalog register absent")?
                                as i8,
                            member: 1,
                            touch: 1,
                            touch_ref: "native:catalog/target".into(),
                        })?,
                    )
                } else {
                    None
                };
                // Worker catalog is a bounded native target admission, while the visible
                // catalog is emitted by the actual worker's copied readback.
                for name in ["hertz", "coordinate", "face", "ratio"] {
                    cell.as_object_mut()
                        .ok_or("native cell invalid")?
                        .remove(name);
                }
                cell["native_target"] = note.unwrap_or(Value::Null);
                Ok(cell)
            })
            .collect()
    }
    fn resolve(&self, touch: KeyTouch) -> Result<Value, String> {
        if let Some(source) = &self.sparse {
            match source.resolve_touch(touch)? {
                SourcePerformanceTouch::Available { native_target, .. } => Ok(native_target),
                SourcePerformanceTouch::Unavailable { receipt } => Err(format!(
                    "native source key unavailable: {}",
                    receipt["reason"]
                )),
            }
        } else {
            resolve_performance_touch(&self.binding, touch)
        }
    }
    fn available_source_preparation_touches(&self) -> Result<Vec<KeyTouch>, String> {
        let mut addresses = std::collections::BTreeSet::new();
        let mut touches = Vec::new();
        for cell in &self.cells {
            if cell["available"] != true {
                continue;
            }
            let key = u8::try_from(cell["key"].as_u64().ok_or("native source key absent")?)
                .map_err(|_| "native source key out of range")?;
            let register = i8::try_from(
                cell["register_octave"]
                    .as_i64()
                    .ok_or("native register absent")?,
            )
            .map_err(|_| "native register out of range")?;
            if addresses.insert((key, register)) {
                let token =
                    u64::try_from(touches.len() + 1).map_err(|_| "native target capacity")?;
                touches.push(KeyTouch {
                    key,
                    register,
                    member: token,
                    touch: token,
                    touch_ref: format!("native:source-performance/prepared-key/{key}/{register}"),
                });
            }
        }
        if touches.is_empty() || touches.len() > performance_audio::MAX_TOUCHES {
            return Err("available source target catalog exceeds native preparation bound".into());
        }
        Ok(touches)
    }
    fn packet(&self) -> Result<Value, String> {
        if let Some(source) = &self.sparse {
            let mut packet = serde_json::to_value(self.binding()).map_err(|e| e.to_string())?;
            packet["source_key_admission"] = source
                .packet_for_touches(&self.available_source_preparation_touches()?)?["source_key_admission"]
                .clone();
            Ok(packet)
        } else {
            serde_json::to_value(self.binding()).map_err(|e| e.to_string())
        }
    }
    fn raw(&self, operation: &str) -> Result<Value, String> {
        let last = self
            .last
            .as_ref()
            .ok_or("native performance has not been prepared")?;
        Ok(
            json!({"schema":CONTROL,"operation":operation,"session_ref":self.config.session_ref,"expected_transport_epoch":last["reading"]["transport_epoch"],"expected_source":self.binding.determination()["identity"],"expected_body_revision":last["reading"]["scope"]["body_revision"]}),
        )
    }
    fn exchange(
        &mut self,
        session: &mut CoupledFieldSession,
        request: Value,
    ) -> Result<Value, String> {
        let receipt = session.performance_exchange(&request)?;
        if let Err(error) = self.validate_reply(&receipt) {
            return Err(session.performance_invalidate(&error));
        }
        self.last = Some(receipt.clone());
        Ok(receipt)
    }
    fn exchange_retaining_parsed_pulse(
        &mut self,
        session: &mut CoupledFieldSession,
        request: Value,
    ) -> Result<Value, (String, Option<Value>)> {
        let receipt = session.performance_exchange_retained(&request)?;
        if let Err(error) = self.validate_reply(&receipt) {
            return Err((session.performance_invalidate(&error), Some(receipt)));
        }
        self.last = Some(receipt.clone());
        Ok(receipt)
    }

    fn stopped_exchange_with_receipts(
        &mut self,
        session: &mut CoupledFieldSession,
        request: Value,
    ) -> Result<Value, NativeStoppedExchangeFailure> {
        let receipt = session.performance_exchange_retained(&request)?;
        if let Err(error) = self.validate_reply(&receipt) {
            return Err(NativeStoppedExchangeFailure {
                reason: session.performance_invalidate(&error),
                native_receipts: vec![receipt],
            });
        }
        self.last = Some(receipt.clone());
        Ok(receipt)
    }
    /// Crate-private stopped consumer used by the native selected Act export
    /// transaction. No public HostOperation accepts a raw scope/packet here.
    pub(crate) fn owner_stopped_exchange(
        &mut self,
        current: &CoupledBasis,
        receiving: &super::performance_receiving::NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        operation: &str,
        operands: &Value,
    ) -> Result<Value, NativeStoppedExchangeFailure> {
        self.validate_current(current)?;
        let reading = self
            .reading()
            .ok_or("actual stopped native reading absent")?;
        if !matches!(
            reading["device"]["state"].as_str(),
            Some("closed" | "prepared")
        ) {
            return Err("native Act export requires the actual attached device stopped".into());
        }
        let cursor = decimal(&reading["samples_elapsed"])?;
        // Replay the actual complete receiving constructor/grants at the copied
        // cursor, then its retained original admission for exact asset equality.
        receiving.prepare_current(self, current, cursor)?;
        let original_cursor = decimal(
            &self.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let original = receiving.prepare_current(self, current, original_cursor)?;
        if original.snapshot()? != self.source_assets["current_receiving"] {
            return Err(
                "native Act export complete original receiving/source/context changed".into(),
            );
        }
        let allowed: &[&str] = match operation {
            "checkpoint" => &[],
            "score" => &["event", "input_ref"],
            "restore" => &[
                "checkpoint",
                "expected_cursor",
                "transaction_ref",
                "checkpoint_ref",
            ],
            "offline-render" => &["scope", "frames"],
            _ => return Err("operation is outside the stopped selected Act export owner".into()),
        };
        let object = operands
            .as_object()
            .ok_or("typed native export operands absent")?;
        if object.len() != allowed.len() || object.keys().any(|k| !allowed.contains(&k.as_str())) {
            return Err(
                "native export operands override source/epoch/session or omit a typed field".into(),
            );
        }
        let mut request = self.raw(operation)?;
        for (key, value) in object {
            request
                .as_object_mut()
                .ok_or("native owner request absent")?
                .insert(key.clone(), value.clone());
        }
        let mut reply = self.stopped_exchange_with_receipts(session, request)?;
        if operation == "restore" && reply["accepted"] == true {
            // Manager stopped_restore deliberately retires its catalog. Put
            // back THIS native owner's unchanged actual K/B catalog before
            // returning continuation to the same original input lifetimes.
            let mut catalog = self.raw("catalog")?;
            catalog["cells"] = json!(self.cells);
            catalog["transpose"] = json!(self.config.transpose);
            let restored = self
                .stopped_exchange_with_receipts(session, catalog)
                .map_err(|mut failure| {
                    failure.native_receipts.insert(0, reply.clone());
                    failure
                })?;
            // Return the actual catalog refusal together with the original
            // restore pulse; the export validator still refuses publication.
            reply["catalog_restoration"] = restored;
        }
        Ok(reply)
    }
    pub(crate) fn owner_export_touch(&self, touch: KeyTouch) -> Result<Value, String> {
        self.resolve(touch)
    }
    /// Private Host/C28 continuation preserves the complete parsed native pulse
    /// on every post-reply refusal. A lost transport has no invented pulse.
    pub(crate) fn exchange_receiving_readmission(
        &mut self,
        session: &mut CoupledFieldSession,
        request: Value,
    ) -> Result<Value, (String, Option<Value>)> {
        if request["operation"] != "restore-current-receiving" {
            return Err((
                "private receiving continuation operation required".into(),
                None,
            ));
        }
        let receipt = session.performance_exchange_retained(&request)?;
        if let Err(error) = self.validate_reply(&receipt) {
            let reason = session.performance_invalidate(&error);
            return Err((reason, Some(receipt)));
        }
        self.last = Some(receipt.clone());
        Ok(receipt)
    }
    fn validate_reply(&self, reply: &Value) -> Result<(), String> {
        if reply["schema"] != "ql.performance-worker-reply/v1"
            || !reply["accepted"].is_boolean()
            || !reply["reason"].is_string()
            || !reply["recording_available"].is_boolean()
        {
            return Err("malformed native performance acknowledgement".into());
        }
        let r = &reply["reading"];
        let d = self.binding().determination();
        let body = self.binding().physical_body();
        let expected = body.request();
        if r["schema"] != "ql.performance-management/v1"
            || r["session_ref"] != self.config.session_ref
            || r["scope"]["instance_ref"] != d["identity"]["instance"]
            || r["scope"]["event_ref"] != d["identity"]["event"]
            || r["scope"]["subject_ref"] != d["identity"]["subject"]
            || r["scope"]["m1_revision"] != d["identity"]["m1_revision"]
            || r["scope"]["m2_generation"] != d["identity"]["m2_generation"]
            || r["scope"]["body_revision"] != d["body_revision"]
            || r["scope"]["preparation_ref"] != d["body_preparation_ref"]
            || r["scope"]["state_ref"] != d["body_state_ref"]
            || decimal(&r["physical"]["body_revision"])? != expected.body_revision
            || r["physical"]["preparation_ref"] != expected.preparation_ref
            || r["physical"]["state_ref"] != expected.state_ref
            || r["physical"]["event_ref"] != body.event_ref()
            || r["physical"]["subject_ref"] != body.subject_ref()
            || r["physical"]["source_coordinate"] != body.source_coordinate().source_ref
            || r["physical"]["source_revision"] != body.source_revision()
            || decimal(&r["physical"]["source_generation"])? != body.source_generation()
            || r["physical"]["sample_rate"] != expected.sample_rate
            || r["physical"]["pratibimba"] != (body.source_coordinate().face == MFace::Pratibimba)
            || r["physical"]["eigenbasis_identity"]
                .as_str()
                .is_none_or(|s| s.is_empty() || s.len() > 2048)
            || r["body_source"]["recipe_ref"] != self.config.recipe.provenance.reference
            || r["physical"]["samples_elapsed"] != r["samples_elapsed"]
            || r["physical"]["body_revision"] != r["scope"]["body_revision"]
            || r["physical"]["preparation_ref"] != r["scope"]["preparation_ref"]
            || r["physical"]["state_ref"] != r["scope"]["state_ref"]
            || r["body_source"]["kind"] != "sourceForm"
            || r["body_source"]["validated_m3_generation"] != r["physical"]["source_generation"]
        {
            return Err("native performance/body/source reply disconnected".into());
        }
        let cursor = decimal(&r["samples_elapsed"])?;
        let sequence = decimal(&r["accepted_sequence"])?;
        let epoch = decimal(&r["transport_epoch"])?;
        if let Some(previous) = &self.last {
            let p = &previous["reading"];
            // No ordinary command may replace the resident body/basis. A
            // future explicit source/body transition must update this binding
            // only after its sole-callback transaction actually commits.
            if r["physical"]["eigenbasis_identity"] != p["physical"]["eigenbasis_identity"] {
                return Err(
                    "resident native eigenbasis changed without prepared transaction".into(),
                );
            }
            let prior_epoch = decimal(&p["transport_epoch"])?;
            if epoch == prior_epoch {
                if cursor < decimal(&p["samples_elapsed"])?
                    || sequence < decimal(&p["accepted_sequence"])?
                {
                    return Err("native performance cursor/sequence regressed".into());
                }
            } else {
                let ack = &reply["payload"]["transport_ack"];
                if epoch <= prior_epoch
                    || ack["previous_epoch"] != p["transport_epoch"]
                    || ack["epoch"] != r["transport_epoch"]
                    || ack["target_sample"] != r["samples_elapsed"]
                    || ack["accepted_sequence"] != r["accepted_sequence"]
                {
                    return Err("native transport changed without exact acknowledgement".into());
                }
            }
        }
        // A valid independently prepared body can share the exact eigenbasis
        // while its original M3 clock differs. EVERY prepare must retain the
        // complete current source descriptor, even after the resident is known.
        if self.last.is_none() || reply["operation"] == "prepare" {
            let descriptor = &reply["payload"]["body_descriptor"];
            if reply["operation"] != "prepare"
                || reply["accepted"] != true
                || descriptor["schema"] != "ql.native-physical-descriptor/v1"
                || descriptor["physical_preparation"]
                    != serde_json::to_value(body).map_err(|e| e.to_string())?
                || descriptor["eigenbasis_identity"] != r["physical"]["eigenbasis_identity"]
                || descriptor["native_cursor"] != r["samples_elapsed"]
            {
                return Err(
                    "initial native body descriptor differs from exact prepared producer".into(),
                );
            }
        }
        let positions = r["physical"]["positions_metres"]
            .as_array()
            .ok_or("native physical positions absent")?;
        let ids = r["physical"]["node_ids"]
            .as_array()
            .ok_or("native physical identities absent")?;
        let expected_nodes = &self.binding.physical_body().request().geometry.nodes;
        if positions.len() != expected_nodes.len()
            || ids.len() != expected_nodes.len()
            || ids
                .iter()
                .zip(expected_nodes)
                .any(|(id, node)| decimal(id) != Ok(node.identity))
            || positions.iter().any(|p| {
                p.as_array().is_none_or(|v| {
                    v.len() != 3 || v.iter().any(|x| x.as_f64().is_none_or(|n| !n.is_finite()))
                })
            })
        {
            return Err("native source-form snapshot is not complete twelve-node body".into());
        }
        if reply["applications"]
            .as_array()
            .is_none_or(|a| a.len() > 256)
            || reply["input_history"]
                .as_array()
                .is_none_or(|a| a.len() > 256)
            || !reply["recording"].is_object()
        {
            return Err("native application/history custody absent".into());
        }
        Ok(())
    }
    pub fn activate(
        &mut self,
        current: &CoupledBasis,
        session: &mut CoupledFieldSession,
    ) -> Result<Value, String> {
        self.activate_prepared(current, session, None)
    }
    /// Called only by the retained FieldHost's original receiving/context
    /// owner under its actual lease. Source and consent bytes do not grant it.
    pub fn activate_with_current_receiving(
        &mut self,
        current: &CoupledBasis,
        session: &mut CoupledFieldSession,
        source: &super::performance_receiving::NativePerformanceReceivingSource,
    ) -> Result<Value, String> {
        let admitted = source.admit_current(self, current, 0)?;
        self.source_assets["receiving_source_inputs"] = admitted.source_inputs().clone();
        self.source_assets["receiving_definition"] = admitted.definition().snapshot()?;
        self.source_assets["current_receiving"] = admitted.snapshot()?;
        self.activate_prepared(current, session, Some(&admitted))
    }
    fn activate_prepared(
        &mut self,
        current: &CoupledBasis,
        session: &mut CoupledFieldSession,
        receiving: Option<&super::performance_receiving::PreparedCurrentReceiving>,
    ) -> Result<Value, String> {
        self.validate_current(current)?;
        let packet = self.packet()?;
        let mut prepare = json!({"schema":CONTROL,"operation":"prepare","session_ref":self.config.session_ref,"packet":packet,"current_source_packet":packet,"actual_native_basis":self.binding.native_basis(),"m1_pratibimba":self.config.source_face==1,"physical_pratibimba":self.config.physical_face==1,"body_source":{"kind":"sourceForm","recipe_ref":self.config.recipe.provenance.reference,"validated_m3_generation":self.config.controls.expected_m3_generation.to_string()}});
        if let Some(receiving) = receiving {
            // Both fields are the private CURRENT native producer's output on
            // this existing Rust->worker pipe. The UI has no raw packet route.
            let current = receiving.admission().snapshot()?;
            prepare["receiving_admission"] = current.clone();
            prepare["current_receiving_admission"] = current;
        }
        self.exchange(session, prepare)?;
        let mut catalog = self.raw("catalog")?;
        catalog["cells"] = json!(self.cells);
        catalog["transpose"] = json!(self.config.transpose);
        let reply = self.exchange(session, catalog)?;
        self.source_assets["consumer_roles"] = reply["reading"]["consumer_roles"].clone();
        Ok(self.public_reply("performance-prepare", reply, None, None))
    }
    pub fn binding(&self) -> &PreparedPerformanceBinding {
        self.return_binding.as_ref().unwrap_or(&self.binding)
    }
    /// Existing K source targets and their complete original B condition
    /// consumer, borrowed from this current owner for native score compilation.
    /// Absence means the separately declared twelve-key policy, never fillers.
    pub fn source_key_consumer<'a>(
        &'a self,
        current: &CoupledBasis,
    ) -> Result<Option<(&'a SparseKeyTargets, SparseMusicalConsumer<'a>)>, String> {
        self.validate_current(current)?;
        self.sparse
            .as_ref()
            .map(|source| Ok((source.targets(), source.consumer(self.binding())?)))
            .transpose()
    }
    /// Resolve through the existing full or sparse native production target
    /// and shared note wire serializer, including registers outside the UI's
    /// prepared address catalog. Current lease/Act authority remains external;
    /// this owner validates its complete native source before numerical lookup.
    pub fn native_note_target(
        &self,
        current: &CoupledBasis,
        touch: KeyTouch,
    ) -> Result<Value, String> {
        self.validate_current(current)?;
        match &self.sparse {
            Some(source) => match source.resolve_touch(touch)? {
                SourcePerformanceTouch::Available { native_target, .. } => Ok(native_target),
                SourcePerformanceTouch::Unavailable { .. } => {
                    Err("recorded note selected an unavailable native source key".into())
                }
            },
            None => resolve_performance_touch(self.binding(), touch),
        }
    }
    /// Source-current qualification of an actual played NoteOn and its exact
    /// original native input journal target. Identifiers select the original
    /// touch; every source/numeric/ratio/phase field is then independently
    /// regenerated by the existing K/B target and native note wire serializer.
    /// This is stopped/control score work, never a callback or imported grant.
    pub fn qualify_recorded_note(
        &self,
        current: &CoupledBasis,
        played_note: &Value,
        original_press_target: &Value,
    ) -> Result<Value, String> {
        self.validate_current(current)?;
        if crate::performance_management::qualify_native_note_wire(
            original_press_target,
            played_note,
        )
        .is_err()
        {
            return Err("played native note lost its exact original input target".into());
        }
        let counter = |name: &str| -> Result<u64, String> {
            let raw = played_note[name]
                .as_str()
                .ok_or("recorded native note counter absent")?;
            let value = raw
                .parse::<u64>()
                .map_err(|_| "invalid native note counter")?;
            if value == 0 || value.to_string() != raw {
                return Err("noncanonical/zero native note counter".into());
            }
            Ok(value)
        };
        let touch = KeyTouch {
            key: played_note["key"]
                .as_u64()
                .filter(|n| *n < 12)
                .ok_or("recorded native key absent")? as u8,
            register: played_note["register_octave"]
                .as_i64()
                .filter(|n| (-16..=16).contains(n))
                .ok_or("recorded native register absent")? as i8,
            member: counter("member")?,
            touch: counter("touch")?,
            touch_ref: played_note["touch_ref"]
                .as_str()
                .ok_or("recorded original touch reference absent")?
                .to_owned(),
        };
        let actual = self.native_note_target(current, touch)?;
        if crate::performance_management::qualify_native_note_wire(&actual, played_note).is_err() {
            return Err(
                "played native note differs from the current K/B/source/phase target".into(),
            );
        }
        Ok(actual)
    }
    pub fn native_packet(&self) -> Result<Value, String> {
        self.packet()
    }
    pub fn native_catalog(&self) -> &[Value] {
        &self.cells
    }
    pub fn source_context_basis<'a>(
        &'a self,
        current: &'a CoupledBasis,
    ) -> Result<crate::performance_source_context::NativeSourceContextBasis<'a>, String> {
        self.validate_current(current)?;
        Ok(
            crate::performance_source_context::NativeSourceContextBasis::from_retained_owner(
                current,
                self.binding(),
            ),
        )
    }
    /// Actual receiving/context owner supplies its private native witness under
    /// the existing lease. JSON evidence never constructs this admission.
    pub fn admit_source_context(
        &mut self,
        current: &CoupledBasis,
        context: &crate::performance_source_context::NativePerformanceSourceContext,
    ) -> Result<(), String> {
        self.validate_current(current)?;
        context.validate_binding(current, self.binding())?;
        self.source_assets["source_context"] = context.snapshot()?;
        Ok(())
    }
    pub fn source_assets(&self) -> &Value {
        &self.source_assets
    }
    pub fn reading(&self) -> Option<&Value> {
        self.last.as_ref().map(|v| &v["reading"])
    }
    fn public_reply(
        &self,
        operation: &str,
        raw: Value,
        input: Option<&str>,
        touch: Option<&str>,
    ) -> Value {
        let accepted = raw["accepted"] == true;
        let admission = if raw["payload"]["admission"].is_object() {
            json!({"sequence":raw["payload"]["admission"]["sequence"],"sample":raw["payload"]["admission"]["sample"],"input_ref":input,"touch_ref":touch,"clock":"native-output","mapping_uncertainty_samples":raw["payload"]["admission"]["mapping_uncertainty_samples"]})
        } else {
            Value::Null
        };
        json!({"schema":REPLY,"operation":operation,"accepted":accepted,"refusal":if accepted{Value::Null}else{json!({"code":"native-refused","reason":raw["reason"]})},"reading":raw["reading"],"transport_transition":Value::Null,"admission":admission,
   "native_pulse":{"applications":raw["applications"],"input_history":raw["input_history"],"recording":raw["recording"],"recording_available":raw["recording_available"],"release_pending":raw["release_pending"],"release_zero_proven":raw["release_zero_proven"],"release_proof_cursor":raw["release_proof_cursor"]},"native_payload":raw["payload"]})
    }
    pub fn execute(
        &mut self,
        current: &CoupledBasis,
        session: &mut CoupledFieldSession,
        command: PerformanceCommand,
    ) -> Result<Value, String> {
        self.validate_current(current)?;
        let public = command.operation();
        let mut input = None;
        let mut touch_ref = None;
        let mut pending_catalog = None;
        let mut request = match command {
            PerformanceCommand::Inspect {} => self.raw("inspect")?,
            PerformanceCommand::Gesture {
                phase: GesturePhase::Press,
                input_ref,
                row,
                column,
                velocity,
                pressure: None,
            } => {
                bounded(&input_ref)?;
                let row = row.ok_or("native press row absent")?;
                let column = column.ok_or("native press column absent")?;
                let cell = self
                    .cells
                    .iter()
                    .find(|c| c["row"] == row && c["column"] == column)
                    .ok_or("native keyboard cell absent")?
                    .clone();
                if cell["available"] != true {
                    let mut raw = self.exchange(session, self.raw("inspect")?)?;
                    raw["accepted"] = json!(false);
                    raw["reason"] = json!(
                        cell["reason"]
                            .as_str()
                            .unwrap_or("native source key unavailable")
                    );
                    return Ok(self.public_reply(public, raw, Some(&input_ref), None));
                }
                let last = self.last.as_ref().ok_or("native performance unavailable")?;
                let token = decimal(&last["last_native_touch"])?
                    .checked_add(1)
                    .ok_or("native touch exhausted")?;
                let member = decimal(&last["last_native_member"])?
                    .checked_add(1)
                    .ok_or("native member exhausted")?;
                let name = format!("native:performance/touch/{token}");
                let target = self.resolve(KeyTouch {
                    key: cell["key"].as_u64().ok_or("native key absent")? as u8,
                    register: cell["register_octave"]
                        .as_i64()
                        .ok_or("native register absent")? as i8,
                    member,
                    touch: token,
                    touch_ref: name.clone(),
                })?;
                let mut r = self.raw("press")?;
                r["input_ref"] = json!(input_ref);
                r["row"] = json!(row);
                r["column"] = json!(column);
                r["native_target"] = target;
                r["velocity"] = json!(velocity.ok_or("native velocity absent")?);
                input = Some(input_ref);
                touch_ref = Some(name);
                r
            }
            PerformanceCommand::Gesture {
                phase: GesturePhase::Release,
                input_ref,
                row: None,
                column: None,
                velocity: None,
                pressure: None,
            } => {
                bounded(&input_ref)?;
                let mut r = self.raw("release")?;
                r["input_ref"] = json!(input_ref);
                input = Some(input_ref);
                r
            }
            PerformanceCommand::Gesture {
                phase: GesturePhase::Expression,
                input_ref,
                row: None,
                column: None,
                velocity: None,
                pressure: Some(pressure),
            } => {
                bounded(&input_ref)?;
                let mut r = self.raw("expression")?;
                r["input_ref"] = json!(input_ref);
                r["pressure"] = json!(pressure);
                input = Some(input_ref);
                r
            }
            PerformanceCommand::Gesture { .. } => {
                return Err("native gesture fields differ from its phase".into());
            }
            PerformanceCommand::Sustain { down } => {
                let mut r = self.raw("sustain")?;
                r["down"] = json!(down);
                r
            }
            PerformanceCommand::Panic { reason } => {
                bounded(&reason)?;
                self.raw("panic")?
            }
            PerformanceCommand::Hold { reason } => {
                bounded(&reason)?;
                self.raw("hold")?
            }
            PerformanceCommand::Transpose { semitones } => {
                if semitones > 11 {
                    return Err("native transpose outside twelve positions".into());
                }
                let before = self.config.transpose;
                self.config.transpose = semitones;
                let prepared = self.catalog();
                self.config.transpose = before;
                let new = prepared?;
                let mut r = self.raw("catalog")?;
                r["cells"] = json!(new);
                r["transpose"] = json!(semitones);
                pending_catalog = Some((semitones, new));
                r
            }
            PerformanceCommand::Parameter {
                target_ref,
                action,
                value,
            } => {
                let id = parameter_id(&target_ref)?;
                let op = match action {
                    ParameterAction::Set => "parameter",
                    ParameterAction::Clear => "parameter-clear",
                    ParameterAction::Undo => "parameter-undo",
                    ParameterAction::Learn => "parameter-learn",
                };
                let mut r = self.raw(op)?;
                r["parameter"] = json!(id);
                if matches!(action, ParameterAction::Set) {
                    r["value"] = json!(value.ok_or("native parameter value absent")?);
                } else if value.is_some() {
                    return Err("native non-set parameter contains a value".into());
                }
                r
            }
            PerformanceCommand::DeviceEnumerate {} => self.raw("device-enumerate")?,
            PerformanceCommand::DeviceOpen {
                device_id,
                sample_rate,
                buffer_frames,
            } => {
                let mut r = self.raw("device-open")?;
                r["device_id"] = json!(device_id);
                r["sample_rate"] = json!(sample_rate);
                r["buffer_frames"] = json!(buffer_frames);
                r
            }
            PerformanceCommand::DeviceStart {} => self.raw("device-start")?,
            PerformanceCommand::DeviceStop {} => self.raw("device-stop")?,
            PerformanceCommand::DeviceRecover {} => self.raw("device-recover")?,
            PerformanceCommand::DeviceClose {} => self.raw("device-close")?,
        };
        // The worker independently checks exact source/body/epoch and dates live
        // gestures from AUHAL; no DOM timestamp or sample cursor enters this packet.
        request["schema"] = json!(CONTROL);
        let raw = self.exchange(session, request)?;
        if raw["accepted"] == true {
            if let Some((transpose, cells)) = pending_catalog {
                self.config.transpose = transpose;
                self.cells = cells;
                self.source_assets["configuration"] =
                    serde_json::to_value(&self.config).map_err(|e| e.to_string())?;
            }
        }
        Ok(self.public_reply(public, raw, input.as_deref(), touch_ref.as_deref()))
    }
}
fn parameter_id(s: &str) -> Result<u8, String> {
    let name = s
        .strip_prefix("ql:performance/parameter/")
        .ok_or("foreign native parameter target")?;
    match name {
        "force-newtons" => Ok(0),
        "attack-seconds" => Ok(1),
        "release-seconds" => Ok(2),
        "cutoff-hertz" => Ok(3),
        "master-linear" => Ok(4),
        "body-linear" => Ok(5),
        "monitor-linear" => Ok(6),
        _ => Err("unknown native parameter target".into()),
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GesturePhase {
    Press,
    Release,
    Expression,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterAction {
    Set,
    Undo,
    Clear,
    Learn,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", deny_unknown_fields)]
pub enum PerformanceCommand {
    #[serde(rename = "performance-inspect")]
    Inspect {},
    #[serde(rename = "performance-gesture")]
    Gesture {
        phase: GesturePhase,
        input_ref: String,
        #[serde(default)]
        row: Option<u8>,
        #[serde(default)]
        column: Option<u8>,
        #[serde(default)]
        velocity: Option<f64>,
        #[serde(default)]
        pressure: Option<f64>,
    },
    #[serde(rename = "performance-sustain")]
    Sustain { down: bool },
    #[serde(rename = "performance-panic")]
    Panic { reason: String },
    #[serde(rename = "performance-hold")]
    Hold { reason: String },
    #[serde(rename = "performance-transpose")]
    Transpose { semitones: u8 },
    #[serde(rename = "performance-parameter")]
    Parameter {
        target_ref: String,
        action: ParameterAction,
        #[serde(default)]
        value: Option<f64>,
    },
    #[serde(rename = "performance-device-enumerate")]
    DeviceEnumerate {},
    #[serde(rename = "performance-device-open")]
    DeviceOpen {
        device_id: u32,
        sample_rate: u32,
        buffer_frames: u32,
    },
    #[serde(rename = "performance-device-start")]
    DeviceStart {},
    #[serde(rename = "performance-device-stop")]
    DeviceStop {},
    #[serde(rename = "performance-device-recover")]
    DeviceRecover {},
    #[serde(rename = "performance-device-close")]
    DeviceClose {},
}
impl PerformanceCommand {
    pub fn operation(&self) -> &'static str {
        match self {
            Self::Inspect {} => "performance-inspect",
            Self::Gesture { .. } => "performance-gesture",
            Self::Sustain { .. } => "performance-sustain",
            Self::Panic { .. } => "performance-panic",
            Self::Hold { .. } => "performance-hold",
            Self::Transpose { .. } => "performance-transpose",
            Self::Parameter { .. } => "performance-parameter",
            Self::DeviceEnumerate {} => "performance-device-enumerate",
            Self::DeviceOpen { .. } => "performance-device-open",
            Self::DeviceStart {} => "performance-device-start",
            Self::DeviceStop {} => "performance-device-stop",
            Self::DeviceRecover {} => "performance-device-recover",
            Self::DeviceClose {} => "performance-device-close",
        }
    }
}

#[cfg(test)]
#[path = "performance_reply_tests.rs"]
mod reply_tests;

pub(crate) fn owner_parameter_id(target: &str) -> Result<u8, String> {
    parameter_id(target)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "performance_timing.rs"]
mod timing;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) use timing::{
    NativeTimingMoment, NativeTimingRefusal, PreparedProceduralTiming,
    PreparedProceduralTimingDescriptor,
};

#[path = "performance_acoustic.rs"]
mod acoustic;
pub use acoustic::{
    AcousticConfiguration, AcousticDirectivity, PreparedAcousticReceiverUpdate,
    PreparedAcousticReceiving,
};
pub(crate) use acoustic::{AcousticRefusal, PreparedAcousticInstallation};
