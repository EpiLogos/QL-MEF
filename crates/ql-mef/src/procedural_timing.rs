//! Exact E/R ABI. This is a source interface proposal, not executed native proof.
use crate::procedural_composition::{Result, TimingBinding};
use crate::procedural_conduct::NativePosition;
use serde_json::Value;
/// A private ephemeral owner witness. Deliberately NO Deserialize/Serialize,
/// Clone, Default or public constructor; caller JSON cannot mint it. R calls
/// the crate-private factory only after actual current Act/lease/source and
/// current native clock/queue admission checks on SAME held native owner.
pub struct NativeTimingWitness {
    original_binding: TimingBinding,
    position: NativePosition,
    requested_cursor: u64,
    admitted_cursor: u64,
    applied_cursor: Option<u64>,
    native_owner_evidence: Value,
    native_source_evidence: Value,
}
impl NativeTimingWitness {
    /// R-only native host factory ingress. `owner_evidence` is the actual
    /// Engine/Manager current clock/queue receipt, `source_evidence` the bounded
    /// actual source/Act/lease projection. Neither transported JSON nor an
    /// imported historical score receipt grants this native factory standing.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_native_owner(
        original_binding: TimingBinding,
        position: NativePosition,
        requested_cursor: u64,
        admitted_cursor: u64,
        applied_cursor: Option<u64>,
        native_owner_evidence: Value,
        native_source_evidence: Value,
    ) -> Result<Self> {
        // R factory has already validated the exact original owner/domain/epoch/
        // time_mapping against its private current owner binding. E validates
        // the unchanged binding and actual boundary again before use.
        if original_binding.owner_ref.is_empty()
            || original_binding.domain.is_empty()
            || original_binding.epoch_ref.is_empty()
            || !native_owner_evidence.is_object()
            || !native_source_evidence.is_object()
        {
            return Err("native timing witness lacks actual bound owner/source evidence".into());
        }
        Ok(Self {
            original_binding,
            position,
            requested_cursor,
            admitted_cursor,
            applied_cursor,
            native_owner_evidence,
            native_source_evidence,
        })
    }
    pub fn original_binding(&self) -> &TimingBinding {
        &self.original_binding
    }
    pub fn native_position(&self) -> &NativePosition {
        &self.position
    }
    pub fn requested_cursor(&self) -> u64 {
        self.requested_cursor
    }
    pub fn admitted_cursor(&self) -> u64 {
        self.admitted_cursor
    }
    pub fn applied_cursor(&self) -> Option<u64> {
        self.applied_cursor
    }
    pub fn event_binding(&self) -> TimingBinding {
        let mut binding = self.original_binding.clone();
        binding.requested_cursor = self.admitted_cursor;
        binding
    }
    pub fn evidence(&self) -> Value {
        serde_json::json!({"schema":"ql.procedural-native-timing-evidence/v1",
            "original_binding":self.original_binding,"native_position":self.position,
            "requested_cursor":self.requested_cursor,"admitted_cursor":self.admitted_cursor,
            "applied_cursor":self.applied_cursor,"native_owner":self.native_owner_evidence,
            "native_source":self.native_source_evidence})
    }
}
