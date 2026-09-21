//! The native material of an accepted identity basis, not an assessment of a
//! person. A renderer may use its digest as a stable private variation seed.
//! It must not derive a chart, M3 address or quaternion from those hash bits.
use super::*;
use ql_mef::nara::{BioQuaternion, SourceRevision};

pub(super) const SCHEMA: &str = "ql.nara-identity-material/v1";
pub(super) const ALGORITHM: &str = "sha256-native-identity-basis/v1";

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct IdentityMaterial {
    pub schema: &'static str,
    pub algorithm: &'static str,
    pub value: String,
    pub identity_hash_ref: ProtectedRef,
    pub target: Target,
    pub record_revision: u64,
    pub identity_revision: String,
    pub sealed: bool,
    pub m3_form_address_ref: Option<String>,
    pub identity_quaternion_ref: Option<ProtectedRef>,
    pub q_composed: BioQuaternion,
    pub event_ref: String,
    pub reception_generation: u64,
    pub source_revisions: Vec<SourceRevision>,
    pub supplied_offices: Vec<IdentitySlotKind>,
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

pub(super) fn material(record: &PersonalRecord) -> Result<IdentityMaterial, String> {
    record.validate()?;
    let identity = &record.domain.identity;
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
        algorithm: ALGORITHM,
        nara_ref: &record.target.nara_ref,
        subject_ref: &record.target.subject_ref,
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
        schema: SCHEMA,
        algorithm: ALGORITHM,
        value,
        sealed: identity.identity_hash_ref.as_ref() == Some(&reference),
        identity_hash_ref: reference,
        target: record.target.clone(),
        record_revision: record.revision,
        identity_revision: identity.identity_revision.clone(),
        m3_form_address_ref: identity.m3_form_address_ref.clone(),
        identity_quaternion_ref: identity.identity_quaternion_ref.clone(),
        q_composed: record.domain.q_composed,
        event_ref: record.domain.event.event_ref.clone(),
        reception_generation: record.domain.embodied.reception_generation,
        source_revisions: sources,
        supplied_offices,
        private: true,
        public_export: false,
        standing: "native exact identity/provenance basis; current reception orientation is separate; not a diagnosis or archetypal-quintessence compression",
    })
}

/// Call only inside the normal consented, revision-checked native Apply path.
/// Acceptance records a protected reference on the existing record, not a new
/// person, second identity store or a generated interpretation of the sources.
pub(super) fn seal(record: &mut PersonalRecord, expected_value: &str) -> Result<(), String> {
    let material = material(record)?;
    if expected_value != material.value {
        return Err(
            "identity basis changed since review; no identity material was accepted".into(),
        );
    }
    record.domain.identity.identity_hash_ref = Some(material.identity_hash_ref);
    Ok(())
}
