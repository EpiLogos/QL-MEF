use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::nara::SourceRevision;

use super::{
    EvidenceStanding, IDENTITY_SLOT_COUNT, ProtectedRef, check_refs, check_source, check_text,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IdentitySlotKind {
    BirthdateName,
    NatalChart,
    JungianAssessment,
    GeneKeys,
    HumanDesign,
    ArchetypalQuintessence,
}

impl IdentitySlotKind {
    pub const fn coordinate(self) -> &'static str {
        match self {
            Self::BirthdateName => "M4.0.0",
            Self::NatalChart => "M4.0.1",
            Self::JungianAssessment => "M4.0.2",
            Self::GeneKeys => "M4.0.3",
            Self::HumanDesign => "M4.0.4",
            Self::ArchetypalQuintessence => "M4.0.5",
        }
    }

    pub const fn ordinal(self) -> u8 {
        match self {
            Self::BirthdateName => 0,
            Self::NatalChart => 1,
            Self::JungianAssessment => 2,
            Self::GeneKeys => 3,
            Self::HumanDesign => 4,
            Self::ArchetypalQuintessence => 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityEvidenceSlot {
    pub kind: IdentitySlotKind,
    pub coordinate_ref: String,
    pub source: Option<SourceRevision>,
    pub protected_value_ref: Option<ProtectedRef>,
    pub evidence_refs: Vec<String>,
    pub tensions: Vec<String>,
    pub absence_reason: Option<String>,
    pub standing: EvidenceStanding,
}

impl IdentityEvidenceSlot {
    pub fn validate(&self) -> Result<(), String> {
        if self.coordinate_ref != self.kind.coordinate() {
            return Err("identity slot coordinate does not match its M4.0 office".into());
        }
        if let Some(source) = &self.source {
            check_source(source)?;
        }
        if let Some(value) = &self.protected_value_ref {
            value.validate()?;
        }
        check_refs(&self.evidence_refs, "identity evidence reference", 256)?;
        check_refs(&self.tensions, "identity tension", 64)?;
        if let Some(reason) = &self.absence_reason {
            check_text(reason, "identity absence reason")?;
        }
        if self.protected_value_ref.is_some() && self.absence_reason.is_some() {
            return Err("identity slot cannot be both present and absent".into());
        }
        if self.protected_value_ref.is_none() && self.absence_reason.is_none() {
            return Err("identity slot must retain a value reference or explicit absence".into());
        }
        if self.standing == EvidenceStanding::Unavailable && self.absence_reason.is_none() {
            return Err("unavailable identity standing requires an absence reason".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityField {
    pub identity_revision: String,
    pub slots: Vec<IdentityEvidenceSlot>,
    pub identity_hash_ref: Option<ProtectedRef>,
    pub identity_quaternion_ref: Option<ProtectedRef>,
    pub m3_form_address_ref: Option<String>,
    pub derivation_refs: Vec<String>,
}

impl IdentityField {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.identity_revision, "identity revision")?;
        if self.slots.len() != IDENTITY_SLOT_COUNT {
            return Err(
                "M4.0 requires all six identity offices, with absence retained explicitly".into(),
            );
        }
        let mut kinds = BTreeSet::new();
        for slot in &self.slots {
            slot.validate()?;
            if !kinds.insert(slot.kind) {
                return Err("duplicate M4.0 identity office".into());
            }
        }
        if kinds.len() != IDENTITY_SLOT_COUNT {
            return Err("M4.0 identity office set is incomplete".into());
        }
        if let Some(reference) = &self.identity_hash_ref {
            reference.validate()?;
        }
        if let Some(reference) = &self.identity_quaternion_ref {
            reference.validate()?;
        }
        if let Some(reference) = &self.m3_form_address_ref {
            check_text(reference, "M3 form address reference")?;
        }
        check_refs(&self.derivation_refs, "identity derivation reference", 256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SourceRevision {
        SourceRevision {
            source_ref: "source:identity".into(),
            revision: "r1".into(),
            standing_ref: "source".into(),
        }
    }

    fn protected() -> ProtectedRef {
        ProtectedRef {
            ref_id: "protected:identity".into(),
            revision: "r1".into(),
            owner_ref: "central".into(),
        }
    }

    fn slot(kind: IdentitySlotKind, present: bool) -> IdentityEvidenceSlot {
        IdentityEvidenceSlot {
            kind,
            coordinate_ref: kind.coordinate().into(),
            source: present.then(source),
            protected_value_ref: present.then(protected),
            evidence_refs: Vec::new(),
            tensions: Vec::new(),
            absence_reason: (!present).then(|| "not supplied".into()),
            standing: if present {
                EvidenceStanding::Source
            } else {
                EvidenceStanding::Unavailable
            },
        }
    }

    #[test]
    fn optional_evidence_does_not_remove_an_identity_office() {
        let field = IdentityField {
            identity_revision: "identity:r1".into(),
            slots: vec![
                slot(IdentitySlotKind::BirthdateName, true),
                slot(IdentitySlotKind::NatalChart, true),
                slot(IdentitySlotKind::JungianAssessment, true),
                slot(IdentitySlotKind::GeneKeys, false),
                slot(IdentitySlotKind::HumanDesign, false),
                slot(IdentitySlotKind::ArchetypalQuintessence, true),
            ],
            identity_hash_ref: Some(protected()),
            identity_quaternion_ref: Some(protected()),
            m3_form_address_ref: Some("m3:form:42".into()),
            derivation_refs: vec!["derivation:identity".into()],
        };
        field.validate().unwrap();
        assert_eq!(field.slots.len(), IDENTITY_SLOT_COUNT);
        assert_eq!(
            field.slots[3].absence_reason.as_deref(),
            Some("not supplied")
        );
    }
}
