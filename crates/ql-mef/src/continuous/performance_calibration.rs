//! Existing FieldHost's original current-instrument calibration. The authored
//! preparation is held by that owner; the command carries no target or value.
use super::super::coupled::{CoupledBasis, CoupledFieldSession};
use super::super::performance::{
    AuthoredCurrentPerformancePreparation, NativeRecordingCommandRefusal, PerformanceOwner,
};
use super::FieldHost;
use serde_json::Value;

/// Shared numerical/control operation. C44 calls this only after its complete
/// original source/context/closed-lease guards; ordinary Host exchange uses
/// the same privately retained preparation and current native owner.
pub(in crate::continuous) fn from_retained_preparation(
    preparation: Option<&AuthoredCurrentPerformancePreparation>,
    owner: &mut PerformanceOwner,
    current: &CoupledBasis,
    session: &mut CoupledFieldSession,
) -> Result<(Value, Value), NativeRecordingCommandRefusal> {
    let preparation = preparation.ok_or("current calibration not applicable: no original fresh native preparation is held; saved/legacy owners cannot readmit it")?;
    let declaration = preparation.output_calibration_declaration();
    let (mut receipt, original) = preparation
        .admit_declared_output_calibration(owner, current, session)?
        .ok_or("current calibration not applicable to the actual explicit mechanical policy")?;
    // The original worker pulse is unchanged. This public operation records
    // which original policy produced the genuine kind5 admission, not sound.
    receipt["operation"] = serde_json::json!("calibrate-current");
    receipt["current_output_calibration"] = declaration;
    Ok((receipt, original))
}
impl FieldHost {
    pub(crate) fn calibrate_current_performance(
        &mut self,
    ) -> Result<(Value, Value), NativeRecordingCommandRefusal> {
        if !self.available() {
            return Err("actual native calibration FieldHost unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let owner = self
            .performance
            .as_mut()
            .ok_or("actual current performance owner absent")?;
        from_retained_preparation(
            self.authored_current_performance_preparation.as_ref(),
            owner,
            &current,
            self.session.session_mut(),
        )
    }
    /// Existing Host exchange must preserve the actual complete refused pulse
    /// as well as the same acknowledged final native reading; no second poll.
    pub(super) fn current_calibration_host_response(&mut self, request_id: &str) -> Value {
        match self.calibrate_current_performance() {
            Ok((receipt, original)) => {
                let mut response = self.response(Some(request_id), "ok", None);
                response["performance"] = receipt;
                response["native_calibration_pulse"] = original;
                response
            }
            Err(failure) => {
                let mut response = self.response(
                    Some(request_id),
                    if self.available() {
                        "refused"
                    } else {
                        "unavailable"
                    },
                    Some(&failure.reason),
                );
                response["native_calibration_pulse"] = failure.native_pulse.unwrap_or(Value::Null);
                response
            }
        }
    }
}
