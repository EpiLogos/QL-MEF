//! Original stopped acoustic candidate on the retained FieldHost and P owner.
//! The closed native Scene channel supplies its own original request ordinal.
use super::*;
use crate::continuous::performance::{
    AcousticConfiguration, AcousticRefusal, PreparedAcousticInstallation,
    PreparedAcousticReceiverUpdate,
};

pub(crate) enum PreparedNativeAcousticEdit {
    Install(PreparedAcousticInstallation),
    Replace(PreparedAcousticReceiverUpdate),
}
impl PreparedNativeAcousticEdit {
    pub(crate) fn before_source_assets(&self) -> &Value {
        match self {
            Self::Install(c) => c.before_source_assets(),
            Self::Replace(c) => c.before_source_assets(),
        }
    }
    pub(crate) fn source_assets(&self) -> &Value {
        match self {
            Self::Install(c) => c.source_assets(),
            Self::Replace(c) => c.source_assets(),
        }
    }
    pub(crate) fn native_boundary(&self) -> &Value {
        match self {
            Self::Install(c) => c.native_boundary(),
            Self::Replace(c) => c.native_boundary(),
        }
    }
    pub(crate) fn original_request_id(&self) -> u64 {
        match self {
            Self::Install(c) => c.original_source_request_id(),
            Self::Replace(c) => c.original_source_request_id(),
        }
    }
    pub(crate) fn source_record(&self) -> Result<&Value, String> {
        match self {
            Self::Install(c) => c.source_transition_record(),
            Self::Replace(c) => c.source_transition_record(),
        }
        .ok_or_else(|| "actual acoustic edit has no original request-bound producer record".into())
    }
}

impl FieldHost {
    pub(crate) fn prepare_performance_acoustic_edit(
        &self,
        original_request_id: u64,
        configuration: AcousticConfiguration,
    ) -> Result<PreparedNativeAcousticEdit, String> {
        if !self.available() || original_request_id == 0 {
            return Err("actual acoustic source preparation owner/ordinal unavailable".into());
        }
        let current = self.session.session().current_basis();
        let original = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving source absent")?;
        let after = original
            .clone()
            .with_acoustic_configuration(configuration)?;
        let owner = self
            .performance
            .as_ref()
            .ok_or("retained acoustic P owner absent")?;
        owner.validate_current(current)?;
        if owner.source_assets().get("acoustic_receiving").is_some() {
            Ok(PreparedNativeAcousticEdit::Replace(
                owner.prepare_stopped_acoustic_receiver_assets_at_request(
                    current,
                    &after,
                    original_request_id,
                )?,
            ))
        } else {
            Ok(PreparedNativeAcousticEdit::Install(
                owner.prepare_stopped_acoustic_installation_assets_at_request(
                    current,
                    original,
                    &after,
                    original_request_id,
                )?,
            ))
        }
    }

    pub(crate) fn prepared_acoustic_scene_source(
        &self,
        candidate: &PreparedNativeAcousticEdit,
        declared_seed: u64,
    ) -> Result<Value, String> {
        if !self.available() {
            return Err("actual acoustic source preparation owner unavailable".into());
        }
        let owner = self
            .performance
            .as_ref()
            .ok_or("retained acoustic P owner absent")?;
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving source absent")?;
        owner.validate_current(self.session.session().current_basis())?;
        if owner.source_assets() != candidate.before_source_assets() {
            return Err(
                "actual acoustic candidate lost its complete operative BEFORE source".into(),
            );
        }
        // Acoustic epochs retain the exact musical body/basis and original
        // episode. Their complete source and ACK history is retained separately.
        let returned = crate::musical_performance_return::bind_performance_return(
            owner.binding(),
            source.original_occasion().cloned(),
            source.return_context().clone(),
            declared_seed,
        )?;
        let record = candidate.source_record()?;
        Ok(json!({
            "schema":"ql.native-current-scene-acoustic-candidate/v1",
            "prepared_only":true,
            "original_native_request_id":candidate.original_request_id().to_string(),
            "native_sample":record["native_sample"],
            "declared_seed":declared_seed.to_string(),
            "before_source_assets":candidate.before_source_assets(),
            "source_assets":candidate.source_assets(),
            "native_boundary":candidate.native_boundary(),
            "source_record":record,
            "basis":returned.expression_basis()?,
            "pitches":returned.expression_pitches(0)?,
            "native_preparation":owner.native_packet()?,
            "native_basis":owner.binding().native_basis(),
            "standing":"pure request-bound same-owner M4 AFTER source; no install, body/clock change or rendered-output acknowledgement"
        }))
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn apply_performance_acoustic_edit(
        &mut self,
        candidate: &PreparedNativeAcousticEdit,
        lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<Value, AcousticRefusal> {
        match candidate {
            PreparedNativeAcousticEdit::Install(candidate) => {
                self.install_performance_acoustic_candidate(candidate, lease)
            }
            PreparedNativeAcousticEdit::Replace(candidate) => {
                self.replace_performance_acoustic(candidate, lease)
            }
        }
    }
}
