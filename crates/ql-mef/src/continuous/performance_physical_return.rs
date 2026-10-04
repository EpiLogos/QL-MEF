//! Pure AFTER Return from the actual prepared native physical descendant.
//! This is preparation evidence; the retained P owner still has its BEFORE body.
use super::*;

impl PreparedNativePhysicalEdit {
    pub(in crate::continuous) fn prepare_scene_return(
        &self,
        source: &NativePerformanceReceivingSource,
        declared_seed: u64,
    ) -> Result<crate::musical_performance_return::MusicalPerformanceReturn, String> {
        self.current_receiving.validate_current(
            source,
            &self.after_owner,
            &self.after_current,
            self.cursor,
        )?;
        if self.current_receiving.snapshot()?
            != self.after_owner.source_assets()["current_receiving"]
        {
            return Err("prepared physical Return lost its complete actual N9 source".into());
        }
        crate::musical_performance_return::bind_performance_return(
            self.after_owner.binding(),
            source.original_occasion().cloned(),
            source.return_context().clone(),
            declared_seed,
        )
    }
}
