//! Child of the actual FieldHost. Every recording command uses the same
//! existing native PerformanceManagement exchange; there is no observer poll.
use super::super::performance::{
    NativeRecordingCommandRefusal, NativeStoppedExchangeFailure, PerformanceCommand,
};
use super::super::performance_act_bridge::NativeActSourceLease;
use super::FieldHost;
use serde_json::Value;

/// Privately produced actual operation receipt and original worker pulse.
/// No Deserialize/Clone/JSON constructor grants source or native custody.
pub(crate) struct NativeCapturedPerformancePulse {
    receipt: Value,
    pulse: Value,
}
impl NativeCapturedPerformancePulse {
    pub(crate) fn receipt(&self) -> &Value {
        &self.receipt
    }
    pub(crate) fn native_pulse(&self) -> &Value {
        &self.pulse
    }
}

impl FieldHost {
    pub(crate) fn capture_native_performance_command(
        &mut self,
        lease: &NativeActSourceLease<'_>,
        command: PerformanceCommand,
    ) -> Result<NativeCapturedPerformancePulse, NativeRecordingCommandRefusal> {
        if matches!(&command, PerformanceCommand::Transpose { .. }) {
            return Err("recorded tuning transition requires its actual typed Scene/source transition owner".into());
        }
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("recording has no actual current receiving source")?;
        let owner = self
            .performance
            .as_mut()
            .ok_or("recording has no actual native performance owner")?;
        owner.validate_current(&current)?;
        lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
        let original_sample = owner.source_assets()["current_receiving"]["native_admission"]
            ["operation"]["native_sample"].as_str()
            .ok_or("original receiving admission cursor absent")?;
        let original_cursor = original_sample.parse::<u64>().map_err(|e| e.to_string())?;
        if original_sample != original_cursor.to_string() {
            return Err("original receiving admission cursor noncanonical".into());
        }
        let original = source.prepare_current(owner, &current, original_cursor)?;
        if original.snapshot()? != owner.source_assets()["current_receiving"] {
            return Err("recording lost complete original source/context/occasion/grants".into());
        }
        let (receipt, pulse) =
            owner.execute_with_original_pulse(&current, self.session.session_mut(), command)?;
        let post = (|| -> Result<(), String> {
            owner.validate_current(&current)?;
            lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
            let after = source.prepare_current(owner, &current, original_cursor)?;
            if after.snapshot()? != owner.source_assets()["current_receiving"] {
                return Err("recording changed actual original receiving source/context".into());
            }
            let current_source = self.session.session().current_basis();
            if serde_json::to_value(current_source).map_err(|error| error.to_string())?
                != serde_json::to_value(&current).map_err(|error| error.to_string())?
            {
                return Err("recording native source changed after actual command".into());
            }
            Ok(())
        })();
        match post {
            Ok(()) => Ok(NativeCapturedPerformancePulse { receipt, pulse }),
            Err(reason) => Err(NativeRecordingCommandRefusal {
                reason,
                native_pulse: Some(pulse),
            }),
        }
    }
}

/// Actual stopped origin, captured before this native owner admits any input.
/// The complete worker pulse is retained; callers cannot construct this token.
pub(crate) struct NativeCapturedRecordingOrigin {
    pulse: Value,
}
impl NativeCapturedRecordingOrigin {
    pub(crate) fn native_pulse(&self) -> &Value {
        &self.pulse
    }
}
fn origin_counter(value: &Value) -> Result<u64, String> {
    let text = value.as_str().ok_or("native origin counter absent")?;
    let value = text.parse::<u64>().map_err(|e| e.to_string())?;
    if text != value.to_string() {
        return Err("native origin counter noncanonical".into());
    }
    Ok(value)
}
impl FieldHost {
    pub(crate) fn capture_native_recording_origin(
        &mut self,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<NativeCapturedRecordingOrigin, NativeStoppedExchangeFailure> {
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("recording origin has no current native receiving source")?;
        let owner = self
            .performance
            .as_mut()
            .ok_or("recording origin has no actual native performance owner")?;
        owner.validate_current(&current)?;
        lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
        let before = owner
            .reading()
            .ok_or("recording origin native reading absent")?;
        if !matches!(
            before["device"]["state"].as_str(),
            Some("closed" | "prepared")
        ) || origin_counter(&before["accepted_sequence"])? != 0
            || origin_counter(&before["last_applied_application_ordinal"])? != 0
            || origin_counter(&before["last_input_ordinal"])? != 0
        {
            return Err("recording origin must be retained before play or any native input admission; an existing performed prefix cannot be backfilled".into());
        }
        let before_cursor = origin_counter(&before["samples_elapsed"])?;
        let pulse = owner.owner_stopped_exchange(
            &current,
            source,
            self.session.session_mut(),
            "checkpoint",
            &serde_json::json!({}),
        )?;
        let validation = (|| -> Result<(), String> {
            owner.validate_current(&current)?;
            lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
            if serde_json::to_value(self.session.session().current_basis())
                .map_err(|error| error.to_string())?
                != serde_json::to_value(&current).map_err(|error| error.to_string())?
            {
                return Err("native source changed while retaining recording origin".into());
            }
            if pulse["accepted"] != true
                || pulse["recording"]["failure"] != 0
                || origin_counter(&pulse["recording"]["dropped_applications"])? != 0
                || !matches!(
                    pulse["reading"]["device"]["state"].as_str(),
                    Some("closed" | "prepared")
                )
                || origin_counter(&pulse["reading"]["samples_elapsed"])? != before_cursor
                || origin_counter(&pulse["reading"]["accepted_sequence"])? != 0
                || origin_counter(&pulse["reading"]["last_applied_application_ordinal"])? != 0
                || origin_counter(&pulse["reading"]["last_input_ordinal"])? != 0
                || origin_counter(&pulse["last_input_ordinal"])? != 0
                || !pulse["applications"].as_array().is_some_and(Vec::is_empty)
                || !pulse["input_history"].as_array().is_some_and(Vec::is_empty)
            {
                return Err(
                    "native stopped origin changed/admitted input or lost original observation"
                        .into(),
                );
            }
            let audio = &pulse["payload"]["checkpoint"]["native_pair"]["audio"];
            if origin_counter(&audio["cursor"])? != before_cursor
                || origin_counter(&audio["accepted_sequence"])? != 0
                || origin_counter(&audio["applied_application_ordinal"])? != 0
            {
                return Err("captured native origin is not the actual pre-input boundary".into());
            }
            Ok(())
        })();
        match validation {
            Ok(()) => Ok(NativeCapturedRecordingOrigin { pulse }),
            Err(reason) => Err(NativeStoppedExchangeFailure {
                reason,
                native_receipts: vec![pulse],
            }),
        }
    }
}

impl FieldHost {
    /// The same outer ordinal and actual LAST acknowledged Field, projected by
    /// its native owner after the dedicated operation. No second Inspect.
    pub(crate) fn recording_controller_receipt(
        &self,
        id: &str,
        performance: Option<&Value>,
    ) -> Value {
        let mut receipt = self.response(Some(id), "ok", None);
        if let Some(performance) = performance {
            receipt["performance"] = performance.clone();
        }
        receipt
    }
}

impl FieldHost {
    pub(crate) fn recording_controller_refusal(&self, id: &str, reason: &str) -> Value {
        self.response(
            Some(id),
            if self.available() {
                "refused"
            } else {
                "unavailable"
            },
            Some(reason),
        )
    }
}
