//! Same actual FieldHost, source/receiving owner and qualified C Scene channel.
use super::super::performance::{NativeSceneContactOperation, NativeSceneContactRefusal};
use super::*;

impl FieldHost {
    pub(crate) fn exchange_performance_scene_contact(
        &mut self,
        command: NativeSceneContactOperation<'_, '_>,
    ) -> Result<Value, NativeSceneContactRefusal> {
        if !self.available() {
            return Err("actual Contact/Field worker unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        command.lease.validate_field_sources(
            &self.instance_ref,
            self.session.session().original_basis(),
            &current,
        )?;
        let receiving = self
            .receiving_source
            .as_ref()
            .ok_or("actual Contact original receiving owner absent")?;
        self.performance
            .as_mut()
            .ok_or("actual Contact performance owner absent")?
            .exchange_native_scene_contact(&current, receiving, self.session.session_mut(), command)
    }

    pub(crate) fn hold_failed_scene_contact_retention(&mut self, reason: &str) -> String {
        self.session.session_mut().performance_invalidate(reason)
    }
}
