use serde::{Deserialize, Serialize};

use crate::nara::{BioQuaternion, EventBasisRefs, SourceRevision};

use super::{
    ContextField, EmbodiedField, IdentityField, IntegrationField, M4_DOMAIN_CONTRACT, OracleField,
    TransformationField, check_source, check_text,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M4DomainState {
    pub schema: String,
    pub subject_id: String,
    pub event: EventBasisRefs,
    pub identity: IdentityField,
    pub embodied: EmbodiedField,
    pub oracle: OracleField,
    pub transformation: TransformationField,
    pub context: ContextField,
    pub integration: IntegrationField,
    pub q_composed: BioQuaternion,
    pub source_revisions: Vec<SourceRevision>,
    pub standing: String,
}

impl M4DomainState {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != M4_DOMAIN_CONTRACT {
            return Err("unsupported M4 domain contract".into());
        }
        check_text(&self.subject_id, "M4 subject")?;
        if self.subject_id != self.event.subject_ref {
            return Err("M4 subject does not match the accepted M1/M2/M3 event".into());
        }
        self.identity.validate()?;
        self.embodied.validate()?;
        self.oracle.validate()?;
        for record in &self.oracle.records {
            if record.original.subject_id != self.subject_id
                || record.original.event_ref != self.event.event_ref
                || record.original.profile_generation != self.event.profile_generation
            {
                return Err("oracle record does not belong to this M4 occasion".into());
            }
        }
        self.transformation.validate()?;
        if self.transformation.phase_history.subject_id != self.subject_id {
            return Err("transformation history belongs to another Nara".into());
        }
        self.context.validate()?;
        self.integration.validate()?;
        self.q_composed.normalized()?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 4096 {
            return Err("M4 domain requires 1..4096 source revisions".into());
        }
        for source in &self.source_revisions {
            check_source(source)?;
        }
        check_text(&self.standing, "M4 standing")
    }
}
