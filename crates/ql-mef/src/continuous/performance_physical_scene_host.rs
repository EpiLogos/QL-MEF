//! Existing closed Scene owner prepares a complete source, without installing P.
use super::super::performance::PreparedNativePhysicalEdit;
use super::*;

impl FieldHost {
    pub(crate) fn prepared_physical_scene_source(
        &self,
        candidate: &PreparedNativePhysicalEdit,
        declared_seed: u64,
    ) -> Result<Value, String> {
        if !self.available() {
            return Err("actual physical source preparation owner unavailable".into());
        }
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving source absent")?;
        let returned = candidate.prepare_scene_return(source, declared_seed)?;
        Ok(json!({
            "schema":"ql.native-current-scene-physical-candidate/v1",
            "prepared_only":true,
            "original_native_request_id":candidate.original_request_id().to_string(),
            "native_sample":candidate.native_sample().to_string(),
            "declared_seed":declared_seed.to_string(),
            "before_source_assets":candidate.before_source_assets(),
            "source_assets":candidate.source_assets(),
            "native_boundary":candidate.native_boundary(),
            "source_record":candidate.source_record(),
            "basis":returned.expression_basis()?,
            "pitches":returned.expression_pitches(0)?,
            "native_preparation":candidate.source_record()["after_native_preparation"],
            "native_basis":candidate.binding().native_basis(),
            "standing":"actual native pure AFTER source/Return; original P/body/audio boundary remains BEFORE until the closed Scene operation commits"
        }))
    }
}
