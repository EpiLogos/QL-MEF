//! Private selected-Act receiving continuation over the retained native host.
//! The C28 closed-reader lease grants source access. Values retain evidence;
//! neither an imported checkpoint nor this reply can construct a live lease.
use super::{FieldHost, MAX_HOST_INPUT};
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use serde_json::{Value, json};

pub struct NativeReceivingReadmissionReply {
    native_pulse: Value,
}
impl NativeReceivingReadmissionReply {
    pub fn native_pulse(&self) -> &Value {
        &self.native_pulse
    }
    pub fn readmission(&self) -> &Value {
        &self.native_pulse["payload"]["receiving_readmission"]
    }
}
pub struct NativeReceivingReadmissionRefusal {
    reason: String,
    native_pulse: Option<Value>,
}
impl NativeReceivingReadmissionRefusal {
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn native_pulse(&self) -> Option<&Value> {
        self.native_pulse.as_ref()
    }
    fn before(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            native_pulse: None,
        }
    }
    fn after(reason: impl Into<String>, native_pulse: Value) -> Self {
        Self {
            reason: reason.into(),
            native_pulse: Some(native_pulse),
        }
    }
}
fn decimal(value: &Value) -> Result<u64, String> {
    let text = value
        .as_str()
        .ok_or("exact native receiving cursor absent")?;
    let n: u64 = text
        .parse()
        .map_err(|_| "invalid native receiving cursor")?;
    if n.to_string() != text {
        return Err("noncanonical native receiving cursor".into());
    }
    Ok(n)
}
fn reference(text: &str) -> Result<(), String> {
    if text.is_empty() || text.len() >= 256 || text.chars().any(char::is_control) {
        return Err("bounded native receiving transaction/checkpoint reference required".into());
    }
    Ok(())
}
impl FieldHost {
    /// Only the actual selected native C28 operation can lend this lease.
    /// The same native session, source owner and original receiving producer
    /// are borrowed throughout preflight, serial numerical commit and return.
    pub fn readmit_retained_performance_checkpoint(
        &mut self,
        lease: &NativeActSourceLease<'_>,
        original_checkpoint_wire: &str,
        checkpoint_ref: &str,
        transaction_ref: &str,
    ) -> Result<NativeReceivingReadmissionReply, NativeReceivingReadmissionRefusal> {
        let preflight = || -> Result<_, String> {
            reference(checkpoint_ref)?;
            reference(transaction_ref)?;
            if original_checkpoint_wire.is_empty()
                || original_checkpoint_wire.len() as u64 > MAX_HOST_INPUT
            {
                return Err(
                    "original native receiving checkpoint exceeds existing transport bound".into(),
                );
            }
            lease.validate_selected_checkpoint(
                &self.instance_ref,
                checkpoint_ref,
                original_checkpoint_wire,
            )?;
            let saved: Value =
                serde_json::from_str(original_checkpoint_wire).map_err(|e| e.to_string())?;
            if saved["schema"] != "ql.performance-management-checkpoint/v1" {
                return Err("original selected Management checkpoint schema differs".into());
            }
            let cursor = decimal(&saved["native_pair"]["audio"]["cursor"])?;
            let owner = self
                .performance
                .as_ref()
                .ok_or("retained native performance owner absent")?;
            let source = self
                .receiving_source
                .as_ref()
                .ok_or("retained original receiving producer absent")?;
            let current = self.session.session().current_basis();
            lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
            let prepared = source.prepare_current(owner, current, cursor)?;
            prepared.validate_current(source, owner, current, cursor)?;
            let complete = prepared.snapshot()?;
            let admission = prepared.admission().snapshot()?;
            let reading = owner
                .reading()
                .ok_or("actual retained native reading absent")?;
            decimal(&reading["samples_elapsed"])?;
            decimal(&reading["transport_epoch"])?;
            let request = json!({"schema":"ql.performance-control/v1",
                "operation":"restore-current-receiving", "session_ref":reading["session_ref"],
                "expected_transport_epoch":reading["transport_epoch"],
                "expected_source":owner.binding().determination()["identity"],
                "expected_body_revision":reading["scope"]["body_revision"],
                "original_checkpoint_wire":original_checkpoint_wire,
                "expected_cursor":reading["samples_elapsed"], "checkpoint_ref":checkpoint_ref,
                "transaction_ref":transaction_ref, "current_source_packet":owner.native_packet()?,
                "actual_native_basis":owner.binding().native_basis(),
                "receiving_admission":admission, "current_receiving_admission":admission,
                "current_receiving":complete, "native_catalog":owner.native_catalog()});
            if serde_json::to_vec(&request)
                .map_err(|e| e.to_string())?
                .len() as u64
                > MAX_HOST_INPUT
            {
                return Err(
                    "native receiving continuation request exceeds existing transport bound".into(),
                );
            }
            // Repeat original source/occasion/definition qualification directly
            // before the only native exchange. No state is changed by this read.
            prepared.validate_current(source, owner, current, cursor)?;
            lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
            Ok((request, prepared, cursor))
        };
        let (request, prepared, cursor) =
            preflight().map_err(NativeReceivingReadmissionRefusal::before)?;
        let owner = self.performance.as_mut().expect("preflight retained owner");
        let pulse = owner
            .exchange_receiving_readmission(self.session.session_mut(), request)
            .map_err(|(reason, pulse)| match pulse {
                Some(pulse) => NativeReceivingReadmissionRefusal::after(reason, pulse),
                None => NativeReceivingReadmissionRefusal::before(reason),
            })?;
        let qualify = || -> Result<(), String> {
            let actual = &pulse["payload"]["receiving_readmission"];
            const FIELDS: [&str; 9] = [
                "schema",
                "original_checkpoint_wire",
                "operative_checkpoint_wire",
                "before_checkpoint_wire",
                "after_checkpoint_wire",
                "current_receiving",
                "current_source_packet",
                "actual_native_basis",
                "transport_ack",
            ];
            if pulse["accepted"] != true || pulse["operation"] != "restore-current-receiving" {
                return Err(pulse["reason"]
                    .as_str()
                    .unwrap_or("actual native receiving restore refused")
                    .into());
            }
            let object = actual
                .as_object()
                .ok_or("native receiving readmission evidence absent")?;
            if object.len() != FIELDS.len()
                || FIELDS.iter().any(|name| !object.contains_key(*name))
                || actual["schema"] != "ql.native-receiving-readmission/v1"
                || actual["original_checkpoint_wire"] != original_checkpoint_wire
                || actual["current_receiving"] != prepared.snapshot()?
                || actual["current_source_packet"] != owner.native_packet()?
                || actual["actual_native_basis"]
                    != serde_json::to_value(owner.binding().native_basis())
                        .map_err(|e| e.to_string())?
                || actual["transport_ack"] != pulse["payload"]["transport_ack"]
            {
                return Err("complete actual receiving readmission/source evidence differs".into());
            }
            for name in [
                "operative_checkpoint_wire",
                "before_checkpoint_wire",
                "after_checkpoint_wire",
            ] {
                let text = actual[name]
                    .as_str()
                    .ok_or("complete native checkpoint text absent")?;
                if text.is_empty() || text.len() as u64 > MAX_HOST_INPUT {
                    return Err(
                        "complete native readmission checkpoint exceeds transport bound".into(),
                    );
                }
            }
            let source = self
                .receiving_source
                .as_ref()
                .ok_or("receiving producer lost after exchange")?;
            prepared.validate_current(
                source,
                owner,
                self.session.session().current_basis(),
                cursor,
            )?;
            lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
            lease.validate_selected_checkpoint(
                &self.instance_ref,
                checkpoint_ref,
                original_checkpoint_wire,
            )?;
            Ok(())
        };
        if let Err(reason) = qualify() {
            return Err(NativeReceivingReadmissionRefusal::after(reason, pulse));
        }
        Ok(NativeReceivingReadmissionReply {
            native_pulse: pulse,
        })
    }
}
