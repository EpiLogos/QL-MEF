//! Native identity/provenance digest over the accepted six-office basis.
//!
//! A renderer may use the digest as a stable private variation seed. It must not
//! derive a chart, M3 address or quaternion from those hash bits. Empty identity
//! is refused rather than turned into a demo hash.

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::SourceRevision;
use super::domain::{IdentityEvidenceSlot, IdentityField, IdentitySlotKind, ProtectedRef};

pub const IDENTITY_MATERIAL_CONTRACT: &str = "ql.nara-identity-material/v1";
pub const IDENTITY_MATERIAL_ALGORITHM: &str = "sha256-native-identity-basis/v1";

const STANDING: &str = "native exact identity/provenance basis; current reception orientation is separate; not a diagnosis or archetypal-quintessence compression";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityMaterial {
    pub schema: &'static str,
    pub algorithm: &'static str,
    pub value: String,
    pub identity_hash_ref: ProtectedRef,
    pub nara_ref: String,
    pub subject_ref: String,
    pub identity_revision: String,
    pub sealed: bool,
    pub m3_form_address_ref: Option<String>,
    pub identity_quaternion_ref: Option<ProtectedRef>,
    pub supplied_offices: Vec<IdentitySlotKind>,
    pub source_revisions: Vec<SourceRevision>,
    pub private: bool,
    pub public_export: bool,
    pub standing: &'static str,
}

/// Canonical serialization is part of v1. Field order is this struct's order;
/// office order is native ordinal; reference sets are sorted lexicographically.
/// Receipt IDs, wall-clock, encounter, provider, current quaternion, M3 address
/// and the identity revision label are deliberately NOT hash inputs.
#[derive(Serialize)]
struct Basis<'a> {
    algorithm: &'static str,
    nara_ref: &'a str,
    subject_ref: &'a str,
    slots: Vec<IdentityEvidenceSlot>,
    derivation_refs: Vec<String>,
}

fn digest(bytes: &[u8]) -> String {
    let hash = Sha256::digest(bytes);
    hash.iter().map(|b| format!("{b:02x}")).collect()
}

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        Err(format!("invalid {label}"))
    } else {
        Ok(())
    }
}

/// Compute the protected identity material for one Nara subject basis.
pub fn material(
    nara_ref: &str,
    subject_ref: &str,
    identity: &IdentityField,
) -> Result<IdentityMaterial, String> {
    text(nara_ref, "nara_ref")?;
    text(subject_ref, "subject_ref")?;
    identity.validate()?;
    let mut slots = identity.slots.clone();
    slots.sort_by_key(|slot| slot.kind.ordinal());
    let mut supplied_offices = Vec::new();
    let mut sources = Vec::new();
    for slot in &mut slots {
        if slot.protected_value_ref.is_some() {
            let source = slot
                .source
                .as_ref()
                .ok_or("identity value has no attributable source basis")?;
            supplied_offices.push(slot.kind);
            sources.push(source.clone());
        }
        slot.evidence_refs.sort();
        slot.tensions.sort();
    }
    if supplied_offices.is_empty() {
        return Err(
            "no identity source is supplied; an empty stack is not a completed identity".into(),
        );
    }
    let mut derivation_refs = identity.derivation_refs.clone();
    derivation_refs.sort();
    let basis = Basis {
        algorithm: IDENTITY_MATERIAL_ALGORITHM,
        nara_ref,
        subject_ref,
        slots,
        derivation_refs,
    };
    let value =
        digest(&serde_json::to_vec(&basis).map_err(|_| "identity basis serialization failed")?);
    let reference = ProtectedRef {
        ref_id: format!("ql:nara-identity:sha256:{value}"),
        revision: value.clone(),
        owner_ref: "ql".into(),
    };
    Ok(IdentityMaterial {
        schema: IDENTITY_MATERIAL_CONTRACT,
        algorithm: IDENTITY_MATERIAL_ALGORITHM,
        value,
        sealed: identity.identity_hash_ref.as_ref() == Some(&reference),
        identity_hash_ref: reference,
        nara_ref: nara_ref.into(),
        subject_ref: subject_ref.into(),
        identity_revision: identity.identity_revision.clone(),
        m3_form_address_ref: identity.m3_form_address_ref.clone(),
        identity_quaternion_ref: identity.identity_quaternion_ref.clone(),
        supplied_offices,
        source_revisions: sources,
        private: true,
        public_export: false,
        standing: STANDING,
    })
}

/// Call only inside a consented, revision-checked Apply path. Acceptance records
/// a protected reference on the existing identity field; it does not create a
/// second identity store or invent an interpretation of the sources.
pub fn seal(
    identity: &mut IdentityField,
    nara_ref: &str,
    subject_ref: &str,
    expected_value: &str,
) -> Result<ProtectedRef, String> {
    let produced = material(nara_ref, subject_ref, identity)?;
    if expected_value != produced.value {
        return Err(
            "identity basis changed since review; no identity material was accepted".into(),
        );
    }
    identity.identity_hash_ref = Some(produced.identity_hash_ref.clone());
    Ok(produced.identity_hash_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nara::domain::EvidenceStanding;

    fn source(revision: &str) -> SourceRevision {
        SourceRevision {
            source_ref: "source:identity".into(),
            revision: revision.into(),
            standing_ref: "source".into(),
        }
    }

    fn protected(revision: &str) -> ProtectedRef {
        ProtectedRef {
            ref_id: format!("protected:identity:{revision}"),
            revision: revision.into(),
            owner_ref: "central".into(),
        }
    }

    fn slot(kind: IdentitySlotKind, present: bool, revision: &str) -> IdentityEvidenceSlot {
        IdentityEvidenceSlot {
            kind,
            coordinate_ref: kind.coordinate().into(),
            source: present.then(|| source(revision)),
            protected_value_ref: present.then(|| protected(revision)),
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

    fn field(present: bool, revision: &str) -> IdentityField {
        IdentityField {
            identity_revision: format!("identity:{revision}"),
            slots: vec![
                slot(IdentitySlotKind::BirthdateName, present, revision),
                slot(IdentitySlotKind::NatalChart, present, revision),
                slot(IdentitySlotKind::JungianAssessment, false, revision),
                slot(IdentitySlotKind::GeneKeys, false, revision),
                slot(IdentitySlotKind::HumanDesign, false, revision),
                slot(IdentitySlotKind::ArchetypalQuintessence, present, revision),
            ],
            identity_hash_ref: None,
            identity_quaternion_ref: Some(protected("q")),
            m3_form_address_ref: Some("m3:form:42".into()),
            derivation_refs: vec!["derivation:identity".into()],
        }
    }

    #[test]
    fn empty_identity_stack_is_refused() {
        let err = material("nara:1", "subject:1", &field(false, "r1")).unwrap_err();
        assert!(err.contains("empty stack") || err.contains("no identity source"));
    }

    #[test]
    fn digest_is_stable_for_the_same_basis() {
        let a = material("nara:1", "subject:1", &field(true, "r1")).unwrap();
        let b = material("nara:1", "subject:1", &field(true, "r1")).unwrap();
        assert_eq!(a.value, b.value);
        assert_eq!(a.algorithm, IDENTITY_MATERIAL_ALGORITHM);
        assert_eq!(a.schema, IDENTITY_MATERIAL_CONTRACT);
        assert!(!a.sealed);
        assert!(a.private);
        assert!(!a.public_export);
    }

    #[test]
    fn source_change_changes_digest_without_using_quaternion_or_m3() {
        let first = material("nara:1", "subject:1", &field(true, "r1")).unwrap();
        let second = material("nara:1", "subject:1", &field(true, "r2")).unwrap();
        assert_ne!(first.value, second.value);
        let mut mutated = field(true, "r1");
        mutated.identity_quaternion_ref = Some(protected("other-q"));
        mutated.m3_form_address_ref = Some("m3:form:99".into());
        mutated.identity_revision = "identity:label-only".into();
        let same = material("nara:1", "subject:1", &mutated).unwrap();
        assert_eq!(first.value, same.value);
    }

    #[test]
    fn seal_accepts_matching_value_and_refuses_stale() {
        let mut identity = field(true, "r1");
        let produced = material("nara:1", "subject:1", &identity).unwrap();
        let sealed = seal(&mut identity, "nara:1", "subject:1", &produced.value).unwrap();
        assert_eq!(identity.identity_hash_ref.as_ref(), Some(&sealed));
        assert!(material("nara:1", "subject:1", &identity).unwrap().sealed);
        let err = seal(&mut identity, "nara:1", "subject:1", "stale").unwrap_err();
        assert!(err.contains("changed since review"));
    }

    #[test]
    fn replace_slot_invalidates_derivative_refs() {
        let mut identity = field(true, "r1");
        let produced = material("nara:1", "subject:1", &identity).unwrap();
        seal(&mut identity, "nara:1", "subject:1", &produced.value).unwrap();
        assert!(identity.identity_hash_ref.is_some());
        identity
            .replace_slot(
                slot(IdentitySlotKind::BirthdateName, true, "r3"),
                "identity:r3".into(),
            )
            .unwrap();
        assert!(identity.identity_hash_ref.is_none());
        assert!(identity.identity_quaternion_ref.is_none());
        assert!(identity.m3_form_address_ref.is_none());
    }
}
