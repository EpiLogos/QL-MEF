//! Closed numerical replay of the exact retained native implementation.
//! Historical output is evidence only. Every pair independently contains a
//! fresh CURRENT preparation; only that preparation may admit live receiving.
use super::*;
// The archived file is byte-identical to the published703 source. These module
// aliases preserve its original imports without editing that implementation.
use crate::continuous::{coupled, performance};
#[expect(
    dead_code,
    reason = "Exact published historical code is intentionally retained privately without changing its bytes; only closed replay is exposed"
)]
#[path = "performance_receiving_versions/703/performance_receiving.rs"]
mod native703;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::continuous) enum ReceivingImplementation {
    Current,
    Native703,
}
impl ReceivingImplementation {
    fn revision(self) -> String {
        let source = match self {
            Self::Current => include_str!("performance_receiving.rs"),
            Self::Native703 => {
                include_str!("performance_receiving_versions/703/performance_receiving.rs")
            }
        };
        format!("sha256:{:x}", Sha256::digest(source.as_bytes()))
    }
    /// Chooses only an implemented numerical algorithm. A revision never
    /// grants a source, lease, restored body or consent. The caller must compare
    /// the COMPLETE regenerated saved snapshot and retain the closed C lease.
    pub(in crate::continuous) fn of_retained(value: &Value) -> Result<Self, String> {
        if value["schema"] != "ql.current-performance-receiving/v1"
            || value["source_payload_context"]["schema"]
                != "ql.native-receiving-source-payload-context/v1"
            || value["source_payload_context"]["owner"]["ref"]
                != "crates/ql-mef/src/continuous/performance_receiving.rs"
            || value["source_payload_context"]["owner"]["availability"] != "available"
        {
            return Err("retained receiving lost its exact native implementation owner".into());
        }
        let revision = value["source_payload_context"]["owner"]["revision"]
            .as_str()
            .ok_or("retained native receiving implementation revision absent")?;
        for implementation in [Self::Current, Self::Native703] {
            if revision == implementation.revision() {
                return Ok(implementation);
            }
        }
        Err("retained native receiving implementation is not locally implemented".into())
    }
}

/// A historical snapshot and its separately fresh current admission. Neither
/// can be deserialized. Historical bytes are never returned as a current grant.
pub(in crate::continuous) struct ReplayedReceiving {
    fresh: PreparedCurrentReceiving,
    retained: Value,
}
impl ReplayedReceiving {
    pub(in crate::continuous) fn fresh(&self) -> &PreparedCurrentReceiving {
        &self.fresh
    }
    pub(in crate::continuous) fn retained_snapshot(&self) -> &Value {
        &self.retained
    }
    pub(in crate::continuous) fn into_fresh(self) -> PreparedCurrentReceiving {
        self.fresh
    }
}

/// Invocation-local CURRENT native producer plus, only where explicitly
/// selected for old evidence, an actual703 native constructor. No cache or
/// serialized witness enters either constructor.
pub(in crate::continuous) struct NativeReceivingReplayPreparation<'a> {
    fresh: NativeCurrentReceivingPreparation<'a>,
    historical: Option<native703::NativePerformanceReceivingSource>,
    implementation: ReceivingImplementation,
}
impl NativeReceivingReplayPreparation<'_> {
    pub(in crate::continuous) fn is_for(
        &self,
        source: &NativePerformanceReceivingSource,
        owner: &PerformanceOwner,
        current: &CoupledBasis,
    ) -> bool {
        self.fresh.is_for(source, owner, current)
    }
    pub(in crate::continuous) fn prepare_current_at(
        &self,
        cursor: u64,
    ) -> Result<PreparedCurrentReceiving, String> {
        self.fresh.prepare_at(cursor)
    }
    pub(in crate::continuous) fn validate_current_at(
        &self,
        prepared: &PreparedCurrentReceiving,
        cursor: u64,
    ) -> Result<(), String> {
        self.fresh.validate_at(prepared, cursor)
    }
    pub(in crate::continuous) fn prepare_at(
        &self,
        cursor: u64,
    ) -> Result<ReplayedReceiving, String> {
        self.prepare_using(cursor, self.implementation)
    }
    fn prepare_using(
        &self,
        cursor: u64,
        implementation: ReceivingImplementation,
    ) -> Result<ReplayedReceiving, String> {
        let fresh = self.fresh.prepare_at(cursor)?;
        let current_snapshot = fresh.snapshot()?;
        let retained = if implementation == ReceivingImplementation::Native703 {
            let source = self
                .historical
                .as_ref()
                .ok_or("actual historical native producer absent")?;
            // Execute the FULL original implementation, not a copied revision
            // or a rewritten current snapshot. Its own include_str! identifies
            // the exact archived code whose native factories ran here.
            let original = source.prepare_current(self.fresh.owner, self.fresh.current, cursor)?;
            let original_snapshot = original.snapshot()?;
            same_complete_determinants(&original_snapshot, &current_snapshot)?;
            original_snapshot
        } else {
            current_snapshot
        };
        Ok(ReplayedReceiving { fresh, retained })
    }
    /// A different retained implementation date still shares this ONE fresh
    /// current source replay. The original implementation is independently
    /// constructed from the same privately borrowed inputs when needed.
    pub(in crate::continuous) fn prepare_retained_at(
        &mut self,
        cursor: u64,
        expected: &Value,
    ) -> Result<ReplayedReceiving, String> {
        let implementation = ReceivingImplementation::of_retained(expected)?;
        if implementation == ReceivingImplementation::Native703 && self.historical.is_none() {
            self.historical = Some(
                self.fresh
                    .source
                    .actual_native703_source(self.fresh.owner)?,
            );
        }
        let actual = self.prepare_using(cursor, implementation)?;
        let repeated = self.prepare_using(cursor, implementation)?;
        if !exact_value(actual.retained_snapshot(), expected)
            || !exact_value(repeated.retained_snapshot(), expected)
            || !exact_value(&actual.fresh().snapshot()?, &repeated.fresh().snapshot()?)
        {
            return Err("complete retained native receiving differs at its original date".into());
        }
        Ok(actual)
    }
    pub(in crate::continuous) fn validate_at(
        &self,
        prepared: &ReplayedReceiving,
        cursor: u64,
    ) -> Result<(), String> {
        let repeated = self.prepare_at(cursor)?;
        if !exact_value(&repeated.retained, &prepared.retained)
            || !exact_value(&repeated.fresh.snapshot()?, &prepared.fresh.snapshot()?)
        {
            return Err("complete historical/current native receiving replay changed".into());
        }
        Ok(())
    }
}

/// Whole native JSON equality retains finite f64 bits and numeric kind.
/// A saved -0.0 is never admitted as +0.0 (or an integer zero). This read-only
/// comparison creates no source, context, replay witness or current grant.
pub(in crate::continuous) fn exact_value(a: &Value, b: &Value) -> bool {
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
fn same_keys(a: &Value, b: &Value) -> Result<(), String> {
    let a = a.as_object().ok_or("native receiving object absent")?;
    let b = b.as_object().ok_or("native receiving object absent")?;
    if a.len() != b.len() || a.keys().any(|key| !b.contains_key(key)) {
        return Err("historical/current receiving lost complete key correspondence".into());
    }
    Ok(())
}
/// Historical and fresh provenance must each name their OWN real code. All
/// determinants, source/context/consent, definitions, dates, admissions, hashes
/// and other provenance fields must agree in full. This does not compare a
/// saved witness: saved output is separately compared without any exceptions.
fn same_complete_determinants(original: &Value, current: &Value) -> Result<(), String> {
    same_keys(original, current)?;
    for (key, value) in original.as_object().ok_or("historical receiving absent")? {
        if key != "source_payload_context" && !exact_value(value, &current[key]) {
            return Err(
                "historical implementation differs from current native determinants".into(),
            );
        }
    }
    let a = &original["source_payload_context"];
    let b = &current["source_payload_context"];
    same_keys(a, b)?;
    for (key, value) in a.as_object().ok_or("historical source payload absent")? {
        if key != "owner" && !exact_value(value, &b[key]) {
            return Err("historical/current source payload determinants differ".into());
        }
    }
    let a = &a["owner"];
    let b = &b["owner"];
    same_keys(a, b)?;
    for (key, value) in a.as_object().ok_or("historical owner absent")? {
        if key != "revision" && !exact_value(value, &b[key]) {
            return Err("historical/current implementation owner differs".into());
        }
    }
    if a["revision"] != ReceivingImplementation::Native703.revision()
        || b["revision"] != ReceivingImplementation::Current.revision()
    {
        return Err(
            "native receiving code provenance differs from its actual implementation".into(),
        );
    }
    Ok(())
}

impl NativePerformanceReceivingSource {
    fn actual_native703_source(
        &self,
        owner: &PerformanceOwner,
    ) -> Result<native703::NativePerformanceReceivingSource, String> {
        // Every value below is borrowed from this privately constructed native
        // owner, never decoded from the saved witness or an imported version.
        let mut source = if let Some(request) = &self.world_request {
            native703::NativePerformanceReceivingSource::world_source(
                request.clone(),
                self.context.clone(),
            )?
        } else if let (Some(profile), Some(sky), Some(occasion), Some(calibration)) = (
            &self.profile,
            &self.sky,
            &self.original_occasion,
            &self.calibration,
        ) {
            match self.context.kind.as_str() {
                "personal" => native703::NativePerformanceReceivingSource::personal(
                    profile,
                    self.natal.as_ref(),
                    sky,
                    occasion.clone(),
                    calibration.clone(),
                    self.context.clone(),
                )?,
                "shared" => native703::NativePerformanceReceivingSource::shared(
                    profile,
                    self.natal.as_ref(),
                    sky,
                    occasion.clone(),
                    calibration.clone(),
                    self.context.clone(),
                )?,
                _ => return Err("historical protected source has another actual context".into()),
            }
        } else if self.profile.is_none() && self.sky.is_none() {
            native703::NativePerformanceReceivingSource::reference_world(
                owner.immutable_source_origin(),
                self.context.clone(),
            )?
        } else {
            return Err("historical native source constructor is disconnected".into());
        };
        if let Some(configuration) = &self.acoustic {
            source = source.with_acoustic_configuration(configuration.clone())?;
        }
        if !exact_value(&source.source_inputs()?, &self.source_inputs()?) {
            return Err(
                "historical native constructor lost its complete actual source inputs".into(),
            );
        }
        Ok(source)
    }
    pub(in crate::continuous) fn begin_replay_preparation<'a>(
        &'a self,
        owner: &'a PerformanceOwner,
        current: &'a CoupledBasis,
        implementation: ReceivingImplementation,
    ) -> Result<NativeReceivingReplayPreparation<'a>, String> {
        let fresh = self.begin_current_preparation(owner, current)?;
        let historical = match implementation {
            ReceivingImplementation::Current => None,
            ReceivingImplementation::Native703 => Some(self.actual_native703_source(owner)?),
        };
        Ok(NativeReceivingReplayPreparation {
            fresh,
            historical,
            implementation,
        })
    }
    pub(in crate::continuous) fn prepare_retained(
        &self,
        owner: &PerformanceOwner,
        current: &CoupledBasis,
        cursor: u64,
        expected: &Value,
    ) -> Result<ReplayedReceiving, String> {
        let implementation = ReceivingImplementation::of_retained(expected)?;
        let invocation = self.begin_replay_preparation(owner, current, implementation)?;
        let prepared = invocation.prepare_at(cursor)?;
        invocation.validate_at(&prepared, cursor)?;
        if !exact_value(prepared.retained_snapshot(), expected) {
            return Err(
                "complete original native receiving source/context/date no longer replays".into(),
            );
        }
        Ok(prepared)
    }
    /// Read-only score qualification across the crate boundary. The complete
    /// old implementation/date and its independently fresh current producer
    /// are verified by the native owner; neither closed carrier escapes here.
    pub(crate) fn validate_retained_for_score(
        &self,
        owner: &PerformanceOwner,
        current: &CoupledBasis,
        cursor: u64,
        expected: &Value,
    ) -> Result<(), String> {
        self.prepare_retained(owner, current, cursor, expected)?;
        Ok(())
    }
    pub(in crate::continuous) fn admit_retained(
        &self,
        owner: &mut PerformanceOwner,
        current: &CoupledBasis,
        cursor: u64,
        expected: &Value,
    ) -> Result<ReplayedReceiving, String> {
        let prepared = self.prepare_retained(owner, current, cursor, expected)?;
        owner.admit_source_context(current, prepared.fresh().context())?;
        Ok(prepared)
    }
    pub(in crate::continuous) fn admit_replayed(
        &self,
        owner: &mut PerformanceOwner,
        current: &CoupledBasis,
        cursor: u64,
        implementation: ReceivingImplementation,
    ) -> Result<ReplayedReceiving, String> {
        let invocation = self.begin_replay_preparation(owner, current, implementation)?;
        let prepared = invocation.prepare_at(cursor)?;
        invocation.validate_at(&prepared, cursor)?;
        drop(invocation);
        // Only the newly produced CURRENT context can bind the numerical owner.
        // Historical snapshots remain immutable evidence in its original assets.
        owner.admit_source_context(current, prepared.fresh().context())?;
        Ok(prepared)
    }
}
