//! Private selected-source request over the existing retained worker. A real
//! closed Act lease and original-owner replay precede this numerical operand.
//! No public control operation, imported packet or checkpoint grants admission.
use super::retained_evidence::{encoded_bound, exact_value};
use super::*;
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use crate::continuous::performance_receiving::{
    NativePerformanceReceivingSource, PreparedCurrentReceiving,
};

pub(crate) const SELECTED_SOURCE_REQUEST: &str = "ql.native-selected-source-readoption-request/v1";

#[derive(Serialize)]
struct BorrowedRequest<'a> {
    schema: &'static str,
    session_ref: &'a str,
    transport_epoch: &'a Value,
    expected_cursor: &'a Value,
    expected_accepted_sequence: &'a Value,
    transaction_ref: &'a str,
    checkpoint_ref: &'a str,
    original_before_checkpoint_wire: &'a str,
    original_checkpoint_wire: &'a str,
    before_source_packet: &'a Value,
    before_native_basis: &'a CoupledBasis,
    packet: &'a Value,
    actual_native_basis: &'a CoupledBasis,
    m1_pratibimba: bool,
    physical_pratibimba: bool,
    body_source: Value,
    current_source_packet: &'a Value,
    receiving_admission: &'a Value,
    current_receiving_admission: &'a Value,
    current_receiving: &'a Value,
    native_catalog: &'a [Value],
    #[serde(skip_serializing_if = "Option::is_none")]
    original_saved_acoustic: Option<&'a Value>,
}

/// No Clone/Deserialize. The complete actual receiving/acoustic producers stay
/// alive while the selected owner and every historical source frame are held.
pub(crate) struct PreparedNativeSelectedSourceRequest {
    request: Value,
    receiving: PreparedCurrentReceiving,
    acoustic: Option<PreparedAcousticReceiving>,
    cursor: u64,
}
impl PreparedNativeSelectedSourceRequest {
    pub(crate) fn request(&self) -> &Value {
        &self.request
    }
    pub(crate) fn validate_selected(
        &self,
        owner: &PerformanceOwner,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<(), String> {
        self.receiving
            .validate_current(source, owner, current, self.cursor)?;
        let instance = owner.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or("selected native instance absent")?;
        lease.validate_source_assets(instance, owner.source_assets())?;
        lease.validate_selected_checkpoint(
            instance,
            self.request["checkpoint_ref"]
                .as_str()
                .ok_or("selected original checkpoint ref absent")?,
            self.request["original_checkpoint_wire"]
                .as_str()
                .ok_or("selected original checkpoint wire absent")?,
        )?;
        if !exact_value(&self.request["packet"], &owner.native_packet()?)
            || !exact_value(
                &self.request["actual_native_basis"],
                &serde_json::to_value(owner.binding().native_basis()).map_err(|e| e.to_string())?,
            )
            || !exact_value(
                &self.request["current_receiving"],
                &self.receiving.snapshot()?,
            )
        {
            return Err("selected native producer changed before source re-adoption".into());
        }
        Ok(())
    }
    pub(crate) fn qualify_reply(&self, pulse: &Value) -> Result<(), String> {
        let original = &self.request;
        let actual = &pulse["payload"]["source_readoption"];
        const FIELDS: [&str; 10] = [
            "schema",
            "before_checkpoint_wire",
            "original_checkpoint_wire",
            "pre_pulse_checkpoint_wire",
            "operative_checkpoint_wire",
            "after_checkpoint_wire",
            "current_source_packet",
            "actual_native_basis",
            "current_receiving",
            "transport_ack",
        ];
        let object = actual
            .as_object()
            .ok_or("actual same-worker source re-adoption evidence absent")?;
        if pulse["accepted"] != true
            || pulse["operation"] != "selected-source-readoption"
            || actual["schema"] != "ql.native-selected-source-readoption/v1"
            || object.len() != FIELDS.len() + usize::from(self.acoustic.is_some())
            || FIELDS.iter().any(|key| !object.contains_key(*key))
            || actual["before_checkpoint_wire"] != original["original_before_checkpoint_wire"]
            || actual["original_checkpoint_wire"] != original["original_checkpoint_wire"]
        {
            return Err("actual selected source re-adoption lost its original transaction".into());
        }
        for field in [
            "current_source_packet",
            "actual_native_basis",
            "current_receiving",
        ] {
            if !exact_value(&actual[field], &original[field]) {
                return Err(format!(
                    "actual selected source re-adoption changed {field}"
                ));
            }
        }
        match self.acoustic.as_ref() {
            Some(acoustic)
                if exact_value(&actual["original_saved_acoustic"], acoustic.packet()) => {}
            None if !object.contains_key("original_saved_acoustic") => {}
            _ => {
                return Err(
                    "actual source re-adoption lost the full original acoustic producer".into(),
                );
            }
        }
        let parse = |field: &str| -> Result<Value, String> {
            let wire = actual[field]
                .as_str()
                .ok_or("actual full native checkpoint text absent")?;
            if wire.is_empty() || wire.len() > crate::continuous::MAX_MESSAGE {
                return Err(
                    "actual complete native checkpoint exceeds the original envelope".into(),
                );
            }
            serde_json::from_str(wire).map_err(|e| e.to_string())
        };
        let pre_pulse = parse("pre_pulse_checkpoint_wire")?;
        if pre_pulse["schema"] != "ql.performance-management-checkpoint/v1" {
            return Err("actual native pre-pulse source checkpoint schema differs".into());
        }
        let operative = parse("operative_checkpoint_wire")?;
        let after = parse("after_checkpoint_wire")?;
        if !exact_value(&operative, &after) {
            return Err(
                "actual selected source restore changed the complete operative native state".into(),
            );
        }
        let before: Value = serde_json::from_str(
            original["original_before_checkpoint_wire"]
                .as_str()
                .ok_or("original resident checkpoint text absent")?,
        )
        .map_err(|e| e.to_string())?;
        let saved: Value = serde_json::from_str(
            original["original_checkpoint_wire"]
                .as_str()
                .ok_or("original selected checkpoint text absent")?,
        )
        .map_err(|e| e.to_string())?;
        let epoch = decimal(&original["transport_epoch"])?
            .checked_add(1)
            .ok_or("selected native transport epoch exhausted")?;
        let expected_ack = json!({"previous_epoch":original["transport_epoch"],
            "epoch":epoch.to_string(),"previous_cursor":original["expected_cursor"],
            "previous_sequence":original["expected_accepted_sequence"],
            "target_sample":saved["native_pair"]["audio"]["cursor"],
            "accepted_sequence":saved["native_pair"]["audio"]["accepted_sequence"],
            "transaction_ref":original["transaction_ref"],"checkpoint_ref":original["checkpoint_ref"]});
        if !exact_value(&actual["transport_ack"], &expected_ack)
            || !exact_value(&pulse["payload"]["transport_ack"], &expected_ack)
            || before["transport_epoch"] != original["transport_epoch"]
            || decimal(&after["transport_epoch"])? != epoch
            || decimal(&pulse["reading"]["transport_epoch"])? != epoch
            || pulse["reading"]["samples_elapsed"] != saved["native_pair"]["audio"]["cursor"]
            || pulse["reading"]["accepted_sequence"]
                != saved["native_pair"]["audio"]["accepted_sequence"]
        {
            return Err(
                "actual same-owner selected source transport acknowledgement differs".into(),
            );
        }
        Ok(())
    }
}

impl PerformanceOwner {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn prepare_selected_source_readoption_request(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        before: &PerformanceOwner,
        before_current: &CoupledBasis,
        original_before_wire: &str,
        original_wire: &str,
        lease: &NativeActSourceLease<'_>,
        checkpoint_ref: &str,
        transaction_ref: &str,
        history: Option<&QualifiedAcousticSourceHistory>,
    ) -> Result<PreparedNativeSelectedSourceRequest, String> {
        bounded(checkpoint_ref)?;
        bounded(transaction_ref)?;
        self.validate_current(current)?;
        before.validate_current(before_current)?;
        if self.last.is_some() || self.config.session_ref != before.config.session_ref {
            return Err(
                "selected-source replay must remain pure under the same original session".into(),
            );
        }
        let instance = self.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or("selected native instance absent")?;
        lease.validate_source_assets(instance, self.source_assets())?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_wire)?;
        if original_wire.is_empty()
            || original_before_wire.is_empty()
            || original_wire.len() > crate::continuous::MAX_MESSAGE
            || original_before_wire.len() > crate::continuous::MAX_MESSAGE
        {
            return Err(
                "original selected/resident full checkpoints exceed native envelope".into(),
            );
        }
        let saved: Value = serde_json::from_str(original_wire).map_err(|e| e.to_string())?;
        let resident: Value =
            serde_json::from_str(original_before_wire).map_err(|e| e.to_string())?;
        let reading = before.reading().ok_or("original resident reading absent")?;
        let cursor = decimal(&saved["native_pair"]["audio"]["cursor"])?;
        if saved["schema"] != "ql.performance-management-checkpoint/v1"
            || resident["schema"] != "ql.performance-management-checkpoint/v1"
            || resident["transport_epoch"] != reading["transport_epoch"]
            || resident["native_pair"]["audio"]["cursor"] != reading["samples_elapsed"]
            || resident["native_pair"]["audio"]["accepted_sequence"] != reading["accepted_sequence"]
            || !matches!(
                reading["device"]["state"].as_str(),
                Some("closed" | "prepared")
            )
        {
            return Err(
                "actual selected-source before checkpoint is stale or device is running".into(),
            );
        }
        let receiving = source.prepare_current(self, current, cursor)?;
        receiving.validate_current(source, self, current, cursor)?;
        let acoustic = self.prepare_saved_acoustic_receiving_with_history(
            current,
            source,
            lease,
            original_wire,
            checkpoint_ref,
            history,
        )?;
        encoded_bound(
            before.binding(),
            8 * 1024 * 1024,
            "original resident native binding",
        )?;
        encoded_bound(
            self.binding(),
            8 * 1024 * 1024,
            "selected original native binding",
        )?;
        let before_packet = before.native_packet()?;
        let packet = self.native_packet()?;
        let admission = receiving.admission().snapshot()?;
        let complete = receiving.snapshot()?;
        let borrowed = BorrowedRequest {
            schema: SELECTED_SOURCE_REQUEST,
            session_ref: &before.config.session_ref,
            transport_epoch: &reading["transport_epoch"],
            expected_cursor: &reading["samples_elapsed"],
            expected_accepted_sequence: &reading["accepted_sequence"],
            transaction_ref,
            checkpoint_ref,
            original_before_checkpoint_wire: original_before_wire,
            original_checkpoint_wire: original_wire,
            before_source_packet: &before_packet,
            before_native_basis: before.binding().native_basis(),
            packet: &packet,
            actual_native_basis: self.binding().native_basis(),
            m1_pratibimba: self.config.source_face == 1,
            physical_pratibimba: self.config.physical_face == 1,
            body_source: json!({"kind":"sourceForm","recipe_ref":self.config.recipe.provenance.reference,
                "validated_m3_generation":self.config.controls.expected_m3_generation.to_string()}),
            current_source_packet: &packet,
            receiving_admission: &admission,
            current_receiving_admission: &admission,
            current_receiving: &complete,
            native_catalog: &self.cells,
            original_saved_acoustic: acoustic.as_ref().map(PreparedAcousticReceiving::packet),
        };
        encoded_bound(
            &borrowed,
            crate::continuous::MAX_MESSAGE,
            "selected-source same-worker request",
        )?;
        let request = serde_json::to_value(borrowed).map_err(|e| e.to_string())?;
        let prepared = PreparedNativeSelectedSourceRequest {
            request,
            receiving,
            acoustic,
            cursor,
        };
        prepared.validate_selected(self, current, source, lease)?;
        Ok(prepared)
    }
    pub(crate) fn retain_selected_source_readoption_pulse(
        &mut self,
        pulse: Value,
    ) -> Result<(), (String, Value)> {
        if let Err(reason) = self.validate_reply(&pulse) {
            return Err((reason, pulse));
        }
        self.last = Some(pulse);
        Ok(())
    }
}
