//! Private nonsounding procedure boundary of SAME guarded native field owner.
//! Its source/Act/lease is qualified by C28's actual closed native channel;
//! caller timing/clock/source JSON and public FieldHost pipes cannot mint it.
use super::*;
use crate::procedural_composition::TimingBinding;
use crate::procedural_timing::NativeTimingWitness;

pub(crate) const NATIVE_FIELD_TIMING_DOMAIN: &str = "native_field_samples";

/// Not a public codec or alternate clock. The complete read is the guarded
/// receipt of the existing FieldSession::read, with zero elapsed frames.
pub(crate) struct PreparedFieldProceduralTiming {
    position: NativePosition,
    witness: NativeTimingWitness,
    native_receipt: Value,
}
impl PreparedFieldProceduralTiming {
    pub(crate) fn position(&self) -> &NativePosition {
        &self.position
    }
    pub(crate) fn witness(&self) -> &NativeTimingWitness {
        &self.witness
    }
    pub(crate) fn native_receipt(&self) -> &Value {
        &self.native_receipt
    }
    /// S/C31 join only: use SAME already checked native receipt and clock fact.
    /// Adoption requires the genuine Source Scene consumer assembly call; no
    /// field/source/clock JSON can call this private result's method.
    pub(crate) fn procedural_source_consumer_fact(
        &self,
    ) -> Result<crate::procedural_consumers::NativeTimingConsumerFact, String> {
        let native = self.procedural_consumer_fact()?;
        crate::procedural_consumers::NativeTimingConsumerFact::from_registered_field_clock(
            native.binding(),
            native.position(),
            native.instance_ref(),
            native.required_generation(),
            native.generation_domain(),
            native.constructor_fact(),
            self.native_receipt(),
        )
    }
    /// No exchange/Inspect. Only this non-Serde private factory result can
    /// expose the constructor from its already source/lease checked receipt.
    pub(crate) fn procedural_consumer_fact(&self) -> Result<NativeFieldClockConsumerFact, String> {
        let row = self
            .native_receipt
            .get("timing_owner")
            .ok_or("historical FIELD receipt has no native clock constructor")?;
        let ordinal = exact_cursor(
            row["construction_ordinal"]
                .as_str()
                .ok_or("native clock construction ordinal absent")?,
        )?;
        let instance = row["instance_ref"]
            .as_str()
            .ok_or("native clock constructor instance absent")?;
        let binding = self.witness.event_binding();
        if ordinal == 0
            || row["schema"] != "ql.native-field-clock-constructor/v1"
            || row["generation_domain"] != "native-field-clock-construction"
            || row["generation"] != row["construction_ordinal"]
            || row["samples_elapsed"] != self.position.samples_elapsed
            || row["event_ref"] != self.position.event_ref
            || row["subject_ref"] != self.position.subject_ref
            || row["clock_generation"] != self.native_receipt["clock"]["generation"]
            || binding.domain != NATIVE_FIELD_TIMING_DOMAIN
            || instance == self.position.instance_ref
            || instance == binding.owner_ref
        {
            return Err(
                "FIELD clock constructor differs from privately checked timing/source boundary"
                    .into(),
            );
        }
        let fact = json!({"role":"field_clock","owner_ref":binding.owner_ref,
            "instance_ref":instance,"construction_ordinal":row["construction_ordinal"],
            "generation":row["generation"],"generation_domain":"native-field-clock-construction",
            "sample":row["samples_elapsed"],"source_instance_ref":self.position.instance_ref,
            "native_clock_constructor":row});
        Ok(NativeFieldClockConsumerFact {
            binding,
            position: self.position.clone(),
            instance_ref: instance.to_owned(),
            construction_generation: ordinal,
            fact,
        })
    }
}
/// Private same-receipt custody; no Deserialize/Clone/Default/public factory.
/// S consumes these getters under the already held C31 current Scene lease.
pub(crate) struct NativeFieldClockConsumerFact {
    binding: TimingBinding,
    position: NativePosition,
    instance_ref: String,
    construction_generation: u64,
    fact: Value,
}
impl NativeFieldClockConsumerFact {
    pub(crate) fn binding(&self) -> &TimingBinding {
        &self.binding
    }
    pub(crate) fn position(&self) -> &NativePosition {
        &self.position
    }
    pub(crate) fn instance_ref(&self) -> &str {
        &self.instance_ref
    }
    pub(crate) fn required_generation(&self) -> u64 {
        self.construction_generation
    }
    pub(crate) fn generation_domain(&self) -> &'static str {
        "native-field-clock-construction"
    }
    pub(crate) fn constructor_fact(&self) -> &Value {
        &self.fact
    }
}

pub(crate) struct NativeFieldTimingRefusal {
    reason: String,
    native_receipt: Option<Value>,
}
impl NativeFieldTimingRefusal {
    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }
    pub(crate) fn native_receipt(&self) -> Option<&Value> {
        self.native_receipt.as_ref()
    }
}
impl From<String> for NativeFieldTimingRefusal {
    fn from(reason: String) -> Self {
        Self {
            reason,
            native_receipt: None,
        }
    }
}
impl From<&str> for NativeFieldTimingRefusal {
    fn from(reason: &str) -> Self {
        reason.to_owned().into()
    }
}
/// The normal browser save codec writes an integral binary64 value such as
/// 220.0 as 220. Compare that spelling only for this portable World basis.
/// Exact native integers beyond the browser's safe range never pass through
/// binary64. Browser JSON also spells -0.0 as 0; this metadata-only check
/// admits that zero spelling. Native sealed values and feedback stay exact.
fn same_portable_basis(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if a == b {
                return true;
            }
            if (a.is_i64() || a.is_u64()) && (b.is_i64() || b.is_u64()) {
                return false;
            }
            const SAFE: u64 = 9_007_199_254_740_991;
            if [a, b].iter().any(|n| {
                n.as_u64().is_some_and(|v| v > SAFE)
                    || n.as_i64().is_some_and(|v| v.unsigned_abs() > SAFE)
            }) {
                return false;
            }
            match (a.as_f64(), b.as_f64()) {
                (Some(a), Some(b)) => (a == 0.0 && b == 0.0) || a.to_bits() == b.to_bits(),
                _ => false,
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same_portable_basis(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter().all(|(key, value)| {
                    b.get(key)
                        .is_some_and(|other| same_portable_basis(value, other))
                })
        }
        _ => left == right,
    }
}

impl FieldHost {
    /// C28's private reader compares this exact actual native tuple with the
    /// original selected Scene/World source by its existing native producers.
    /// It is a source observation; serialization cannot construct a C lease.
    pub(crate) fn native_field_source_tuple(&self) -> Value {
        let session = self.session.session();
        json!({"schema":"ql.native-held-field-source/v1","instance_ref":self.instance_ref,
            "original_basis":session.original_basis(),"current_basis":session.current_basis(),
            "original_field":session.original_field()})
    }
    /// Complete Coupled source shared by the actual field/performance owners.
    /// Content restitution grants no FIELD timing or independently live lease.
    pub(crate) fn retained_procedural_source_artifact(&self) -> Result<Value, String> {
        if !self.available() {
            return Err("actual native Coupled source owner unavailable".into());
        }
        let session = self.session.session();
        for basis in [session.original_basis(), session.current_basis()] {
            let actual = serde_json::to_value(basis).map_err(|e| e.to_string())?;
            let replay = serde_json::to_value(basis.input.compose()?).map_err(|e| e.to_string())?;
            if actual != replay {
                return Err(
                    "held native field source differs from full native producer replay".into(),
                );
            }
        }
        Ok(self.native_field_source_tuple())
    }
    /// Original authorship descriptor comes from C28's native registration,
    /// never a UI epoch/default. This observational result is not a live grant.
    pub(crate) fn native_field_timing_descriptor(
        &mut self,
        act_lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<TimingBinding, NativeFieldTimingRefusal> {
        self.prepare_native_field_timing_descriptor(act_lease)
            .map(|prepared| prepared.witness().original_binding().clone())
    }
    /// Preserve the witness of the descriptor's ONE original guarded read.
    /// Bootstrap consumes this private result directly; it must never call
    /// field_procedural_timing_witness for a second observation.
    pub(crate) fn prepare_native_field_timing_descriptor(
        &mut self,
        act_lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<PreparedFieldProceduralTiming, NativeFieldTimingRefusal> {
        if !self.available() || self.performance.is_some() {
            return Err("native nonsounding field descriptor unavailable or A/P-owned".into());
        }
        let source = self.native_field_source_tuple();
        act_lease.validate_field_sources(
            &self.instance_ref,
            self.session.session().original_basis(),
            self.session.session().current_basis(),
        )?;
        let mut before = self.session.session().last_field().clone();
        // A preceding Advance's emitted PCM is not part of a zero-frame read.
        before["audio"] = json!([]);
        let receipt = self.session.session_mut().read_field()?;
        let prepared = (|| -> Result<(NativePosition, NativeTimingWitness), String> {
            if receipt != before || self.native_field_source_tuple() != source {
                return Err(
                    "native descriptor read changed full source or its nonadvancing boundary"
                        .into(),
                );
            }
            let position = NativePosition::from_field(&self.instance_ref, &receipt)?;
            let cursor = position.cursor()?;
            let binding = act_lease.field_timing_binding(
                &self.instance_ref,
                self.session.session().original_field(),
                cursor,
            )?;
            if binding.domain != NATIVE_FIELD_TIMING_DOMAIN || binding.requested_cursor != cursor {
                return Err("native field timing registration has another domain/boundary".into());
            }
            act_lease.validate_field_sources(
                &self.instance_ref,
                self.session.session().original_basis(),
                self.session.session().current_basis(),
            )?;
            act_lease.validate_field_timing(
                &self.instance_ref,
                &binding,
                self.session.session().original_field(),
            )?;
            let owner = json!({"schema":"ql.native-field-timing-fact/v1",
                "binding":binding,"native_position":position,
                "requested_cursor":cursor.to_string(),"admitted_cursor":cursor.to_string(),
                "applied_cursor":null,"field_clock":receipt["clock"],"native_field_receipt":receipt,
                "standing":"same guarded native FieldSession descriptor read boundary; no material/body/audio application acknowledgement"});
            let witness = NativeTimingWitness::from_native_owner(
                binding,
                position.clone(),
                cursor,
                cursor,
                None,
                owner,
                json!({"native_act_source_lease":act_lease.evidence(),"actual_field_source":source}),
            )?;
            Ok((position, witness))
        })();
        match prepared {
            Ok((position, witness)) => Ok(PreparedFieldProceduralTiming {
                position,
                witness,
                native_receipt: receipt,
            }),
            Err(reason) => Err(NativeFieldTimingRefusal {
                reason,
                native_receipt: Some(receipt),
            }),
        }
    }
    /// No PerformanceOwner is necessary for an ordinary nonsounding material
    /// procedure. If A/P owns the field, its R boundary is mandatory instead.
    pub(crate) fn field_procedural_timing_witness(
        &mut self,
        original_binding: TimingBinding,
        act_lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<PreparedFieldProceduralTiming, NativeFieldTimingRefusal> {
        if !self.available() || self.performance.is_some() {
            return Err("native field boundary unavailable or retained A/P owns its clock".into());
        }
        if original_binding.domain != NATIVE_FIELD_TIMING_DOMAIN {
            return Err(
                "field procedure requires its privately registered native field sample domain"
                    .into(),
            );
        }
        let source = self.native_field_source_tuple();
        // C28 must compare the exact original binding and full tuple against
        // current closed-reader/source custody, not merely accept this Value.
        act_lease.validate_field_sources(
            &self.instance_ref,
            self.session.session().original_basis(),
            self.session.session().current_basis(),
        )?;
        act_lease.validate_field_timing(
            &self.instance_ref,
            &original_binding,
            self.session.session().original_field(),
        )?;
        let mut before = self.session.session().last_field().clone();
        // Read returns no PCM; a preceding running Advance legitimately retained
        // its emitted block. Only that old PCM is excluded from zero-frame read.
        before["audio"] = json!([]);
        let receipt = self.session.session_mut().read_field()?;
        let prepared = (|| -> Result<(NativePosition, NativeTimingWitness), String> {
            if receipt != before || self.native_field_source_tuple() != source {
                return Err("native field read changed original/current source or the nonadvancing boundary".into());
            }
            let position = NativePosition::from_field(&self.instance_ref, &receipt)?;
            let cursor = position.cursor()?;
            act_lease.validate_field_sources(
                &self.instance_ref,
                self.session.session().original_basis(),
                self.session.session().current_basis(),
            )?;
            act_lease.validate_field_timing(
                &self.instance_ref,
                &original_binding,
                self.session.session().original_field(),
            )?;
            let owner = json!({"schema":"ql.native-field-timing-fact/v1",
                "binding":original_binding,"native_position":position,
                "requested_cursor":cursor.to_string(),"admitted_cursor":cursor.to_string(),
                "applied_cursor":null,"field_clock":receipt["clock"],"native_field_receipt":receipt,
                "standing":"same guarded native FieldSession read boundary; no material/body/audio application acknowledgement"});
            let witness = NativeTimingWitness::from_native_owner(
                original_binding,
                position.clone(),
                cursor,
                cursor,
                None,
                owner,
                json!({"native_act_source_lease":act_lease.evidence(),"actual_field_source":source}),
            )?;
            Ok((position, witness))
        })();
        match prepared {
            Ok((position, witness)) => Ok(PreparedFieldProceduralTiming {
                position,
                witness,
                native_receipt: receipt,
            }),
            Err(reason) => Err(NativeFieldTimingRefusal {
                reason,
                native_receipt: Some(receipt),
            }),
        }
    }
}

/// C28 calls this while borrowing its genuine closed selected Scene reader.
/// This validates source content; only C28's private live lease can grant use.
/// No Scene.performance or audio/music assets are accessed.
impl FieldHost {
    /// C28 validate_field_timing invokes this on its actual closed Scene.
    /// Full native field material/samples/units/initial clock are required;
    /// a basis-only fallback cannot qualify an imported FieldInput.
    #[cfg(test)]
    pub(crate) fn validate_retained_field_input(
        presentation: &Value,
        actual_original_field: &FieldInput,
    ) -> Result<(), String> {
        Self::validate_retained_field_input_artifact(
            &presentation["scene"]["epiWorld"]["native_field_source"],
            actual_original_field,
        )
    }
    pub(crate) fn validate_retained_field_input_artifact(
        retained: &Value,
        actual_original_field: &FieldInput,
    ) -> Result<(), String> {
        if retained["schema"] != "ql.native-held-field-source/v1"
            || retained["original_field"]
                != serde_json::to_value(actual_original_field).map_err(|e| e.to_string())?
        {
            return Err("closed native FIELD original sample/material/clock input differs or has not been retained".into());
        }
        Ok(())
    }
    #[cfg(test)]
    pub(crate) fn validate_retained_field_sources(
        presentation: &Value,
        instance_ref: &str,
        actual_original: &super::super::coupled::CoupledBasis,
        actual_current: &super::super::coupled::CoupledBasis,
    ) -> Result<(), String> {
        Self::validate_retained_field_sources_with_artifact(
            presentation,
            instance_ref,
            actual_original,
            actual_current,
            presentation["scene"]["epiWorld"].get("native_field_source"),
        )
    }
    /// Borrow actual source qualified from all original C parts; no inline
    /// 65k geometry clone or basis/readback reconstruction is performed.
    pub(crate) fn validate_retained_field_sources_with_artifact(
        presentation: &Value,
        instance_ref: &str,
        actual_original: &super::super::coupled::CoupledBasis,
        actual_current: &super::super::coupled::CoupledBasis,
        retained: Option<&Value>,
    ) -> Result<(), String> {
        use sha2::{Digest, Sha256};
        let record = &presentation["scene"]["epiWorld"];
        let world = &record["world"];
        if presentation["schema"] != "oi.journey-scene/v1"
            || record["schema"] != "oi.epi-world-material/v1"
            || world["schema"] != "oi.epi-portable-world/v1"
            || world["instance_ref"] != instance_ref
            || world["event_ref"] != actual_original.input.m1.event_ref
            || world["event_ref"] != actual_current.input.m1.event_ref
            || world["subject_ref"] != actual_original.input.m3.subject_ref
            || world["subject_ref"] != actual_current.input.m3.subject_ref
            || world["event"] != world["basis"]["input"]
        {
            return Err(
                "closed native FIELD Scene has a foreign original instance/event/subject/source"
                    .into(),
            );
        }
        let owners = world["native_owner_sources"]
            .as_array()
            .ok_or("native FIELD owner sources absent")?;
        let expected = [
            (
                "constructor",
                "crates/ql-mef/src/scene.rs",
                include_str!("../scene.rs"),
            ),
            (
                "coupled",
                "crates/ql-mef/src/continuous/coupled.rs",
                include_str!("coupled.rs"),
            ),
            (
                "field",
                "crates/ql-mef/src/continuous/scene_field.rs",
                include_str!("scene_field.rs"),
            ),
        ];
        if owners.len() != expected.len() {
            return Err("closed native FIELD owner-source roles incomplete".into());
        }
        for (role, reference, source) in expected {
            let matches: Vec<_> = owners.iter().filter(|row| row["role"] == role).collect();
            if matches.len() != 1
                || matches[0]["reading"]["ref"] != reference
                || matches[0]["reading"]["revision"]
                    != format!("sha256:{:x}", Sha256::digest(source.as_bytes()))
                || matches[0]["reading"]["availability"] != "available"
            {
                return Err(
                    "closed native FIELD source owner/reference/revision is unavailable or changed"
                        .into(),
                );
            }
        }
        let original = serde_json::to_value(actual_original).map_err(|e| e.to_string())?;
        let current = serde_json::to_value(actual_current).map_err(|e| e.to_string())?;
        if !same_portable_basis(&world["basis"], &original) {
            return Err(
                "canonical original portable world basis differs from actual original FIELD source"
                    .into(),
            );
        }
        if let Some(retained) = retained {
            if retained["schema"] != "ql.native-held-field-source/v1"
                || retained["instance_ref"] != instance_ref
                || retained["original_basis"] != original
                || retained["current_basis"] != current
            {
                return Err("closed native FIELD source differs from actual full original/current held owners".into());
            }
        } else if original != current || !same_portable_basis(&world["basis"], &original) {
            return Err("full native FIELD continuation source has not been retained in its canonical Scene".into());
        }
        for actual in [actual_original, actual_current] {
            let produced = actual.input.compose()?;
            if serde_json::to_value(produced).map_err(|e| e.to_string())?
                != serde_json::to_value(actual).map_err(|e| e.to_string())?
            {
                return Err(
                    "closed native FIELD source differs from complete producer replay".into(),
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    fn source() -> (super::super::super::coupled::CoupledBasis, Value) {
        let input: super::super::super::coupled::CoupledInput = serde_json::from_str(include_str!(
            "../../../../fixtures/kernel/scene-default-event-v2.json"
        ))
        .unwrap();
        let basis = input.compose().unwrap();
        let owners=[
            ("constructor","crates/ql-mef/src/scene.rs",include_str!("../scene.rs")),
            ("coupled","crates/ql-mef/src/continuous/coupled.rs",include_str!("coupled.rs")),
            ("field","crates/ql-mef/src/continuous/scene_field.rs",include_str!("scene_field.rs")),
        ].into_iter().map(|(role,reference,source)|json!({"role":role,"reading":{"ref":reference,"revision":format!("sha256:{:x}",Sha256::digest(source.as_bytes())),"availability":"available"}})).collect::<Vec<_>>();
        let presentation = json!({"schema":"oi.journey-scene/v1","scene":{"epiWorld":{"schema":"oi.epi-world-material/v1","world":{"schema":"oi.epi-portable-world/v1","instance_ref":"native:field-instance","event_ref":basis.input.m1.event_ref,"subject_ref":basis.input.m3.subject_ref,"basis":basis,"event":basis.input,"native_owner_sources":owners}}}});
        (basis, presentation)
    }
    #[test]
    fn exact_native_field_source_accessor_uses_actual_producer_bases_without_performance_assets() {
        let (basis, presentation) = source();
        FieldHost::validate_retained_field_sources(
            &presentation,
            "native:field-instance",
            &basis,
            &basis,
        )
        .unwrap();
        assert!(
            presentation["scene"]["epiWorld"]
                .get("performance")
                .is_none()
        );
        // This pure source-content validator constructs NO native lease/witness.
    }
    /// Only the normal browser JSON codec's safe integral-number spelling.
    /// The source values are the COMPLETE actual CoupledInput composition,
    /// not reconstructed modal summaries or a private native owner fixture.
    fn browser_safe_number_spellings(value: &mut Value) -> usize {
        match value {
            Value::Number(number) if number.is_f64() => {
                let actual = number.as_f64().unwrap();
                if actual.fract() == 0.0 && actual.abs() <= 9_007_199_254_740_991f64 {
                    *value = json!(actual as i64);
                    1
                } else {
                    0
                }
            }
            Value::Array(rows) => rows.iter_mut().map(browser_safe_number_spellings).sum(),
            Value::Object(rows) => rows.values_mut().map(browser_safe_number_spellings).sum(),
            _ => 0,
        }
    }

    #[test]
    fn actual_complete_portable_world_basis_accepts_browser_number_spelling_only() {
        let (basis, mut presentation) = source();
        let original = serde_json::to_value(&basis).unwrap();
        let world = &mut presentation["scene"]["epiWorld"]["world"];
        // Exercise every safe integral binary64 descendant of the actual
        // complete native World basis and its matching full original event.
        let changed_basis = browser_safe_number_spellings(&mut world["basis"]);
        let changed_event = browser_safe_number_spellings(&mut world["event"]);
        assert!(
            changed_basis > 1 && changed_event > 0,
            "complete native numeric rows must be exercised"
        );
        assert_eq!(
            world["basis"]["input"]["m1"]["revision"],
            original["input"]["m1"]["revision"]
        );
        let saved: Value =
            serde_json::from_slice(&serde_json::to_vec(&presentation).unwrap()).unwrap();
        assert_ne!(saved["scene"]["epiWorld"]["world"]["basis"], original);
        FieldHost::validate_retained_field_sources(&saved, "native:field-instance", &basis, &basis)
            .unwrap();
        let mut changed = saved.clone();
        changed["scene"]["epiWorld"]["world"]["basis"]["input"]["m2"]["condition"]["tonic_hz"] =
            json!(221);
        changed["scene"]["epiWorld"]["world"]["event"]["m2"]["condition"]["tonic_hz"] = json!(221);
        assert!(
            FieldHost::validate_retained_field_sources(
                &changed,
                "native:field-instance",
                &basis,
                &basis
            )
            .is_err()
        );
        let mut added = saved.clone();
        added["scene"]["epiWorld"]["world"]["basis"]["extra"] = json!("another source");
        assert!(
            FieldHost::validate_retained_field_sources(
                &added,
                "native:field-instance",
                &basis,
                &basis
            )
            .is_err()
        );
        // This content check never constructs an Act lease, FIELD witness,
        // Source admission, owner receipt or alternate native clock.
    }

    #[test]
    fn portable_basis_number_equality_preserves_exact_large_identity_and_structure() {
        assert!(same_portable_basis(&json!(0), &json!(-0.0)));
        assert!(same_portable_basis(&json!(0.0), &json!(-0.0)));
        assert!(!same_portable_basis(&json!(0), &json!(-f64::MIN_POSITIVE)));
        assert!(same_portable_basis(
            &json!({"tonic":220}),
            &json!({"tonic":220.0})
        ));
        assert!(same_portable_basis(
            &json!(9_007_199_254_740_991u64),
            &json!(9_007_199_254_740_991f64)
        ));
        assert!(!same_portable_basis(
            &json!(9_007_199_254_740_992u64),
            &json!(9_007_199_254_740_992f64)
        ));
        assert!(!same_portable_basis(
            &json!(9_007_199_254_740_993u64),
            &json!(9_007_199_254_740_992u64)
        ));
        assert!(!same_portable_basis(
            &json!(220),
            &json!(220.00000000000003)
        ));
        assert!(!same_portable_basis(&json!([1, 2]), &json!([2.0, 1.0])));
        assert!(!same_portable_basis(
            &json!({"tonic":220}),
            &json!({"tonic":220.0,"actor":"foreign"})
        ));
    }

    #[test]
    fn native_field_source_accessor_refuses_foreign_subject_and_owner_cut() {
        let (basis, presentation) = source();
        let paths = [
            "subject",
            "owner_ref",
            "owner_revision",
            "owner_availability",
            "event",
        ];
        for change in paths {
            let mut bad = presentation.clone();
            let world = &mut bad["scene"]["epiWorld"]["world"];
            match change {
                "subject" => world["subject_ref"] = json!("foreign:subject"),
                "owner_ref" => {
                    world["native_owner_sources"][0]["reading"]["ref"] = json!("foreign:source")
                }
                "owner_revision" => {
                    world["native_owner_sources"][1]["reading"]["revision"] = json!("sha256:stale")
                }
                "owner_availability" => {
                    world["native_owner_sources"][2]["reading"]["availability"] =
                        json!("unavailable")
                }
                "event" => world["event"]["m1"]["revision"] = json!("foreign:revision"),
                _ => unreachable!(),
            }
            assert!(
                FieldHost::validate_retained_field_sources(
                    &bad,
                    "native:field-instance",
                    &basis,
                    &basis
                )
                .is_err(),
                "accepted {change}"
            );
        }
    }
    #[test]
    fn actual_full_field_continuation_requires_retained_current_source_not_readback_summary() {
        let (basis, mut presentation) = source();
        let mut input = basis.input.clone();
        input.m1.revision = "1".into();
        let current = input.compose().unwrap();
        assert!(
            FieldHost::validate_retained_field_sources(
                &presentation,
                "native:field-instance",
                &basis,
                &current
            )
            .is_err()
        );
        presentation["scene"]["epiWorld"]["native_field_source"] = json!({"schema":"ql.native-held-field-source/v1","instance_ref":"native:field-instance","original_basis":basis,"current_basis":current});
        FieldHost::validate_retained_field_sources(
            &presentation,
            "native:field-instance",
            &basis,
            &current,
        )
        .unwrap();
        presentation["scene"]["epiWorld"]["native_field_source"]["current_basis"]["m1"] =
            json!({"summary":"not the full native owner"});
        assert!(
            FieldHost::validate_retained_field_sources(
                &presentation,
                "native:field-instance",
                &basis,
                &current
            )
            .is_err()
        );
    }
    #[test]
    fn contradictory_canonical_world_and_forged_original_field_material_refuse() {
        let (basis, mut presentation) = source();
        presentation["scene"]["epiWorld"]["native_field_source"] = json!({"schema":"ql.native-held-field-source/v1","instance_ref":"native:field-instance","original_basis":basis,"current_basis":basis});
        presentation["scene"]["epiWorld"]["world"]["basis"]["derivation"]["harmonic_ratio"] =
            json!({"contradiction":"different canonical source"});
        assert!(
            FieldHost::validate_retained_field_sources(
                &presentation,
                "native:field-instance",
                &basis,
                &basis
            )
            .is_err()
        );
        let (_, mut presentation) = source();
        let original:FieldInput=serde_json::from_value(json!({"subject_ref":basis.input.m3.subject_ref,"sample_rate":48000,"clock":{"inscription":{"turns":"0","half_degrees":0},"lensing":{"turns":"0","half_degrees":0},"grid_origins":[0,0,0],"rate_numerators":["0","0"],"rate_denominator":1,"rate_remainders":["0","0"],"generation":"0"},"driver_numerator":1,"driver_denominator":1,"units":{"amplitude":"m","excitation":"m/s","shape":"dimensionless","position":"m","audio":"linear"},"audio_gains":[0.0],"samples":[{"identity":7,"constituent":"material:sample","attachment":0,"rest_metres":[0.0,0.0,0.0],"mode_shapes":[[1.0,0.0,0.0]]}]})).unwrap();
        presentation["scene"]["epiWorld"]["native_field_source"] =
            json!({"schema":"ql.native-held-field-source/v1","original_field":original});
        FieldHost::validate_retained_field_input(&presentation, &original).unwrap();
        presentation["scene"]["epiWorld"]["native_field_source"]["original_field"]["samples"][0]
            ["identity"] = json!(8);
        assert!(FieldHost::validate_retained_field_input(&presentation, &original).is_err());
        // DTO comparison exercises real source validation only; it is neither
        // a native sampled-field receipt nor a private timing witness positive.
    }
}
