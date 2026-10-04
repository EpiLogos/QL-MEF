//! Private current Scene/Act join for the same retained FieldHost.
//! Public HostOperation/PerformanceExchange never accept a prepared body,
//! source lease, native cursor or completed transition packet.
use super::super::performance::{AuthoredNativePhysicalEdit, PreparedNativePhysicalEdit};
use super::*;

impl FieldHost {
    pub(crate) fn prepare_performance_physical_edit(
        &self,
        original_native_request_id: u64,
        authored: AuthoredNativePhysicalEdit,
    ) -> Result<PreparedNativePhysicalEdit, String> {
        if !self.available() {
            return Err("actual physical source owner unavailable".into());
        }
        let current = self.session.session().current_basis();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving source not bound")?;
        self.performance
            .as_ref()
            .ok_or("retained native physical performance not active")?
            .prepare_native_physical_edit(current, source, original_native_request_id, authored)
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn apply_performance_physical_edit(
        &mut self,
        candidate: PreparedNativePhysicalEdit,
        lease: &super::super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<Value, super::super::performance::NativeStoppedExchangeFailure> {
        if !self.available() {
            return Err("actual physical source owner unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving source not bound")?;
        let (pulse, after) = self
            .performance
            .as_mut()
            .ok_or("retained native physical performance not active")?
            .apply_native_physical_edit(
                &current,
                source,
                self.session.session_mut(),
                candidate,
                lease,
            )?;
        // Actual native P/Engine/M4 ACK has already passed every source/body
        // check. This is the same owner's source publication, with no second
        // legacy field advance, reseed, replacement or transport operation.
        match &mut self.session {
            Owner::Supplied(session) => session.adopt_physical_source_after_ack(after),
            Owner::Scene(instrument) => instrument.adopt_physical_source_after_ack(after),
        }
        Ok(pulse)
    }
}
