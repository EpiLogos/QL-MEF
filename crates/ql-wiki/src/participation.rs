use std::collections::{BTreeMap, BTreeSet};

use ql_mef::MFace;
use ql_mef::m_tree::{MRegistry, MTreeId};
use serde::{Deserialize, Serialize};

use crate::{MappingOrigin, MetaKnowledgeProjection, MetaProvenance, ProjectedRelation};

pub const WIKI_PARTICIPATION_CONTRACT: &str = "ql-mef/wiki-participation/v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParticipationError {
    Invalid(String),
    UnknownWikiRef(String),
    WikiRevisionMismatch(String),
    UnknownBimbaRef(String),
    BimbaIdentityMismatch(String),
    Conflict(String),
}

impl core::fmt::Display for ParticipationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Invalid(value) => write!(f, "invalid Wiki participation: {value}"),
            Self::UnknownWikiRef(value) => write!(f, "unknown Wiki ref {value}"),
            Self::WikiRevisionMismatch(value) => {
                write!(f, "Wiki revision mismatch for {value}")
            }
            Self::UnknownBimbaRef(value) => write!(f, "unknown Bimba ref {value}"),
            Self::BimbaIdentityMismatch(value) => {
                write!(f, "Bimba identity mismatch for {value}")
            }
            Self::Conflict(value) => write!(f, "Wiki participation conflict {value}"),
        }
    }
}

impl std::error::Error for ParticipationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParticipationForm {
    Constellation,
    Pair,
    Triad,
    Whole,
    DirectConjugate,
}

impl ParticipationForm {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Constellation => "constellation",
            Self::Pair => "pair",
            Self::Triad => "triad",
            Self::Whole => "whole",
            Self::DirectConjugate => "direct-conjugate",
        }
    }

    fn validate_cardinality(self, count: usize) -> Result<(), ParticipationError> {
        let valid = match self {
            Self::Constellation => count >= 2,
            Self::Pair => count == 2,
            Self::Triad => count == 3,
            Self::Whole => count >= 1,
            Self::DirectConjugate => count == 2,
        };
        if valid {
            Ok(())
        } else {
            Err(ParticipationError::Invalid(format!(
                "{} has invalid member count {count}",
                self.as_str()
            )))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParticipationRole {
    Member,
    Direct,
    Conjugate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BimbaFace {
    Direct,
    Conjugate,
}

impl BimbaFace {
    pub const fn m_face(self) -> MFace {
        match self {
            Self::Direct => MFace::Bimba,
            Self::Conjugate => MFace::Pratibimba,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ParticipationReferent {
    Wiki {
        canonical_ref: String,
        revision: u64,
    },
    Bimba {
        coordinate_ref: String,
        coordinate_id: MTreeId,
        registry_revision: String,
        face: BimbaFace,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParticipationMember {
    pub role: ParticipationRole,
    pub referent: ParticipationReferent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WikiParticipation {
    pub contract: String,
    pub participation_ref: String,
    pub revision: u64,
    pub form: ParticipationForm,
    pub origin: MappingOrigin,
    pub members: Vec<ParticipationMember>,
    pub provenance: Vec<MetaProvenance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompiledParticipationMember {
    pub role: ParticipationRole,
    pub canonical_ref: String,
    pub source_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompiledWikiParticipation {
    pub contract: String,
    pub participation_ref: String,
    pub revision: u64,
    pub form: ParticipationForm,
    pub origin: MappingOrigin,
    pub members: Vec<CompiledParticipationMember>,
    pub provenance: Vec<MetaProvenance>,
}

fn nonempty(value: &str, what: &str) -> Result<(), ParticipationError> {
    if value.is_empty() || value != value.trim() || value.contains('\0') {
        Err(ParticipationError::Invalid(what.into()))
    } else {
        Ok(())
    }
}

impl WikiParticipation {
    pub fn validate_shape(&self) -> Result<(), ParticipationError> {
        if self.contract != WIKI_PARTICIPATION_CONTRACT {
            return Err(ParticipationError::Invalid(
                "unsupported participation contract".into(),
            ));
        }
        nonempty(&self.participation_ref, "empty participation_ref")?;
        if self.revision == 0 {
            return Err(ParticipationError::Invalid(
                "participation revision must be positive".into(),
            ));
        }
        self.form.validate_cardinality(self.members.len())?;
        if self.provenance.is_empty() {
            return Err(ParticipationError::Invalid(
                "participation requires source provenance".into(),
            ));
        }
        for source in &self.provenance {
            nonempty(&source.source_ref, "empty participation source_ref")?;
            if source
                .source_revision
                .as_ref()
                .is_some_and(|revision| revision.trim().is_empty())
            {
                return Err(ParticipationError::Invalid(
                    "empty participation source revision".into(),
                ));
            }
        }
        match self.form {
            ParticipationForm::DirectConjugate => {
                let direct = self
                    .members
                    .iter()
                    .filter(|member| member.role == ParticipationRole::Direct)
                    .count();
                let conjugate = self
                    .members
                    .iter()
                    .filter(|member| member.role == ParticipationRole::Conjugate)
                    .count();
                if direct != 1 || conjugate != 1 {
                    return Err(ParticipationError::Invalid(
                        "direct-conjugate requires exactly one direct and one conjugate member"
                            .into(),
                    ));
                }
            }
            _ => {
                if self
                    .members
                    .iter()
                    .any(|member| member.role != ParticipationRole::Member)
                {
                    return Err(ParticipationError::Invalid(
                        "direct/conjugate roles belong only to direct-conjugate participation"
                            .into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

pub fn bimba_participant(
    registry: &MRegistry,
    coordinate_ref: &str,
    face: BimbaFace,
    role: ParticipationRole,
) -> Result<ParticipationMember, ParticipationError> {
    let node = registry
        .resolve(coordinate_ref)
        .ok_or_else(|| ParticipationError::UnknownBimbaRef(coordinate_ref.into()))?;
    Ok(ParticipationMember {
        role,
        referent: ParticipationReferent::Bimba {
            coordinate_ref: node.source_ref.clone(),
            coordinate_id: node.id,
            registry_revision: registry.manifest().registry_revision.clone(),
            face,
        },
    })
}

fn compile_member(
    projection: &MetaKnowledgeProjection,
    registry: &MRegistry,
    member: &ParticipationMember,
) -> Result<CompiledParticipationMember, ParticipationError> {
    match &member.referent {
        ParticipationReferent::Wiki {
            canonical_ref,
            revision,
        } => {
            nonempty(canonical_ref, "empty Wiki canonical_ref")?;
            let object = projection
                .objects
                .iter()
                .find(|object| object.canonical_ref == *canonical_ref)
                .ok_or_else(|| ParticipationError::UnknownWikiRef(canonical_ref.clone()))?;
            if object.revision != *revision {
                return Err(ParticipationError::WikiRevisionMismatch(
                    canonical_ref.clone(),
                ));
            }
            Ok(CompiledParticipationMember {
                role: member.role,
                canonical_ref: canonical_ref.clone(),
                source_identity: format!("wiki:{canonical_ref}@{revision}"),
            })
        }
        ParticipationReferent::Bimba {
            coordinate_ref,
            coordinate_id,
            registry_revision,
            face,
        } => {
            if registry_revision != &registry.manifest().registry_revision {
                return Err(ParticipationError::BimbaIdentityMismatch(
                    coordinate_ref.clone(),
                ));
            }
            let node = registry
                .resolve(coordinate_ref)
                .ok_or_else(|| ParticipationError::UnknownBimbaRef(coordinate_ref.clone()))?;
            if node.id != *coordinate_id || node.source_ref != *coordinate_ref {
                return Err(ParticipationError::BimbaIdentityMismatch(
                    coordinate_ref.clone(),
                ));
            }
            let coordinate = registry
                .coordinate(coordinate_ref, face.m_face())
                .map_err(ParticipationError::BimbaIdentityMismatch)?;
            Ok(CompiledParticipationMember {
                role: member.role,
                canonical_ref: coordinate.canonical_ref(),
                source_identity: format!(
                    "bimba:{}:{}@{}",
                    coordinate_ref,
                    face.m_face().as_str(),
                    registry_revision
                ),
            })
        }
    }
}

pub fn compile_participation(
    projection: &MetaKnowledgeProjection,
    registry: &MRegistry,
    participation: &WikiParticipation,
) -> Result<CompiledWikiParticipation, ParticipationError> {
    participation.validate_shape()?;
    let mut members = Vec::with_capacity(participation.members.len());
    let mut identities = BTreeSet::new();
    for member in &participation.members {
        let compiled = compile_member(projection, registry, member)?;
        if !identities.insert(compiled.source_identity.clone()) {
            return Err(ParticipationError::Invalid(
                "duplicate participation member identity".into(),
            ));
        }
        members.push(compiled);
    }

    if participation.form == ParticipationForm::DirectConjugate {
        let mut source_coordinates = BTreeSet::new();
        let mut faces = BTreeSet::new();
        for member in &participation.members {
            let ParticipationReferent::Bimba {
                coordinate_ref,
                coordinate_id,
                face,
                ..
            } = &member.referent
            else {
                return Err(ParticipationError::Invalid(
                    "direct-conjugate members must both be Bimba faces".into(),
                ));
            };
            source_coordinates.insert((coordinate_ref.clone(), *coordinate_id));
            faces.insert(*face as u8);
            match (member.role, face) {
                (ParticipationRole::Direct, BimbaFace::Direct)
                | (ParticipationRole::Conjugate, BimbaFace::Conjugate) => {}
                _ => {
                    return Err(ParticipationError::Invalid(
                        "direct-conjugate role does not match its Bimba face".into(),
                    ));
                }
            }
        }
        if source_coordinates.len() != 1 || faces.len() != 2 {
            return Err(ParticipationError::Invalid(
                "direct-conjugate must be two faces of one source coordinate".into(),
            ));
        }
    }

    members.sort_by(|left, right| {
        left.role
            .as_sort_key()
            .cmp(&right.role.as_sort_key())
            .then(left.canonical_ref.cmp(&right.canonical_ref))
    });
    Ok(CompiledWikiParticipation {
        contract: WIKI_PARTICIPATION_CONTRACT.into(),
        participation_ref: participation.participation_ref.clone(),
        revision: participation.revision,
        form: participation.form,
        origin: participation.origin,
        members,
        provenance: participation.provenance.clone(),
    })
}

impl ParticipationRole {
    const fn as_sort_key(self) -> u8 {
        match self {
            Self::Member => 0,
            Self::Direct => 1,
            Self::Conjugate => 2,
        }
    }
}

impl CompiledWikiParticipation {
    pub fn relation_origin_ref(&self) -> String {
        format!("{}@{}", self.participation_ref, self.revision)
    }

    pub fn relations(&self, first_projection_id: u64) -> Vec<ProjectedRelation> {
        let origin_ref = self.relation_origin_ref();
        self.members
            .iter()
            .enumerate()
            .map(|(index, member)| ProjectedRelation {
                projection_id: first_projection_id + index as u64,
                from_ref: self.participation_ref.clone(),
                to_ref: member.canonical_ref.clone(),
                relation: format!(
                    "participates:{}:{}",
                    self.form.as_str(),
                    match member.role {
                        ParticipationRole::Member => "member",
                        ParticipationRole::Direct => "direct",
                        ParticipationRole::Conjugate => "conjugate",
                    }
                ),
                origin: self.origin.as_str().into(),
                origin_ref: Some(origin_ref.clone()),
            })
            .collect()
    }
}

/// Apply one already source-qualified participation to the native meta projection.
/// Reapplying the exact revision is idempotent. The same semantic revision with
/// different members fails closed instead of silently rewriting Wiki topology.
pub fn apply_participation(
    projection: &mut MetaKnowledgeProjection,
    registry: &MRegistry,
    participation: &WikiParticipation,
) -> Result<CompiledWikiParticipation, ParticipationError> {
    let compiled = compile_participation(projection, registry, participation)?;
    let origin_ref = compiled.relation_origin_ref();
    let existing = projection
        .relations
        .iter()
        .filter(|relation| relation.origin_ref.as_deref() == Some(origin_ref.as_str()))
        .collect::<Vec<_>>();
    if !existing.is_empty() {
        let expected = compiled
            .relations(1)
            .into_iter()
            .map(|relation| {
                (
                    relation.from_ref,
                    relation.to_ref,
                    relation.relation,
                    relation.origin,
                )
            })
            .collect::<BTreeSet<_>>();
        let actual = existing
            .iter()
            .map(|relation| {
                (
                    relation.from_ref.clone(),
                    relation.to_ref.clone(),
                    relation.relation.clone(),
                    relation.origin.clone(),
                )
            })
            .collect::<BTreeSet<_>>();
        if expected == actual {
            return Ok(compiled);
        }
        return Err(ParticipationError::Conflict(origin_ref));
    }

    let first_projection_id = projection
        .relations
        .iter()
        .map(|relation| relation.projection_id)
        .max()
        .unwrap_or(0)
        + 1;
    projection
        .relations
        .extend(compiled.relations(first_projection_id));
    Ok(compiled)
}

/// Navigation/context helper preserving participation identity rather than
/// flattening it into unrelated search hits.
pub fn participation_context<'a>(
    projection: &'a MetaKnowledgeProjection,
    participation_ref: &str,
    revision: u64,
) -> BTreeMap<&'a str, Vec<&'a ProjectedRelation>> {
    let origin_ref = format!("{participation_ref}@{revision}");
    let mut grouped: BTreeMap<&str, Vec<&ProjectedRelation>> = BTreeMap::new();
    for relation in projection
        .relations
        .iter()
        .filter(|relation| relation.origin_ref.as_deref() == Some(origin_ref.as_str()))
    {
        grouped
            .entry(relation.relation.as_str())
            .or_default()
            .push(relation);
    }
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProjectedObject;
    use ql_mef::m_tree::native_m_registry;

    fn projection() -> MetaKnowledgeProjection {
        MetaKnowledgeProjection {
            projection_version: 7,
            objects: vec![
                ProjectedObject {
                    projection_id: 1,
                    canonical_ref: "wiki:a".into(),
                    object_kind: "node".into(),
                    revision: 2,
                },
                ProjectedObject {
                    projection_id: 2,
                    canonical_ref: "wiki:b".into(),
                    object_kind: "node".into(),
                    revision: 4,
                },
                ProjectedObject {
                    projection_id: 3,
                    canonical_ref: "wiki:c".into(),
                    object_kind: "node".into(),
                    revision: 1,
                },
            ],
            relations: vec![],
            meta_bindings: vec![],
        }
    }

    fn source() -> Vec<MetaProvenance> {
        vec![MetaProvenance {
            source_ref: "docs/kernel-rebuild/PRE-K8-AGENT-WORLD-LOCK.md".into(),
            source_revision: Some("owner-ratified-2026-09-12".into()),
        }]
    }

    fn wiki_member(reference: &str, revision: u64) -> ParticipationMember {
        ParticipationMember {
            role: ParticipationRole::Member,
            referent: ParticipationReferent::Wiki {
                canonical_ref: reference.into(),
                revision,
            },
        }
    }

    fn participation(
        reference: &str,
        form: ParticipationForm,
        members: Vec<ParticipationMember>,
    ) -> WikiParticipation {
        WikiParticipation {
            contract: WIKI_PARTICIPATION_CONTRACT.into(),
            participation_ref: reference.into(),
            revision: 1,
            form,
            origin: MappingOrigin::Authored,
            members,
            provenance: source(),
        }
    }

    #[test]
    fn pair_and_triad_use_exact_wiki_revisions() {
        let registry = native_m_registry();
        let pair = participation(
            "wiki:pair/ab",
            ParticipationForm::Pair,
            vec![wiki_member("wiki:a", 2), wiki_member("wiki:b", 4)],
        );
        assert_eq!(
            compile_participation(&projection(), registry, &pair)
                .unwrap()
                .members
                .len(),
            2
        );
        let stale = participation(
            "wiki:pair/stale",
            ParticipationForm::Pair,
            vec![wiki_member("wiki:a", 1), wiki_member("wiki:b", 4)],
        );
        assert!(matches!(
            compile_participation(&projection(), registry, &stale),
            Err(ParticipationError::WikiRevisionMismatch(_))
        ));
        let wrong_triad = participation(
            "wiki:triad/wrong",
            ParticipationForm::Triad,
            vec![wiki_member("wiki:a", 2), wiki_member("wiki:b", 4)],
        );
        assert!(compile_participation(&projection(), registry, &wrong_triad).is_err());
    }

    #[test]
    fn direct_conjugate_is_two_faces_of_the_same_bimba_identity() {
        let registry = native_m_registry();
        let direct =
            bimba_participant(registry, "#0", BimbaFace::Direct, ParticipationRole::Direct)
                .unwrap();
        let conjugate = bimba_participant(
            registry,
            "#0",
            BimbaFace::Conjugate,
            ParticipationRole::Conjugate,
        )
        .unwrap();
        let relation = participation(
            "wiki:direct-conjugate/m0",
            ParticipationForm::DirectConjugate,
            vec![direct, conjugate],
        );
        let compiled = compile_participation(&projection(), registry, &relation).unwrap();
        assert_eq!(compiled.members.len(), 2);
        assert_ne!(
            compiled.members[0].canonical_ref,
            compiled.members[1].canonical_ref
        );
        assert!(
            compiled
                .members
                .iter()
                .all(|member| member.source_identity.contains("bimba:#0"))
        );
    }

    #[test]
    fn bimba_and_wiki_members_share_one_constellation_without_identity_copying() {
        let registry = native_m_registry();
        let bimba = bimba_participant(registry, "#0", BimbaFace::Direct, ParticipationRole::Member)
            .unwrap();
        let constellation = participation(
            "wiki:constellation/ground",
            ParticipationForm::Constellation,
            vec![wiki_member("wiki:a", 2), bimba],
        );
        let mut projection = projection();
        let compiled = apply_participation(&mut projection, registry, &constellation).unwrap();
        assert_eq!(compiled.members.len(), 2);
        assert!(
            projection
                .relations
                .iter()
                .any(|relation| { relation.to_ref.starts_with("ql:m-coordinate:bimba:M0") })
        );
        assert!(
            projection
                .objects
                .iter()
                .all(|object| { !object.canonical_ref.starts_with("ql:m-coordinate:") })
        );
    }

    #[test]
    fn exact_replay_is_idempotent_but_same_revision_cannot_change_members() {
        let registry = native_m_registry();
        let original = participation(
            "wiki:pair/ab",
            ParticipationForm::Pair,
            vec![wiki_member("wiki:a", 2), wiki_member("wiki:b", 4)],
        );
        let mut projection = projection();
        apply_participation(&mut projection, registry, &original).unwrap();
        let count = projection.relations.len();
        apply_participation(&mut projection, registry, &original).unwrap();
        assert_eq!(projection.relations.len(), count);

        let changed = participation(
            "wiki:pair/ab",
            ParticipationForm::Pair,
            vec![wiki_member("wiki:a", 2), wiki_member("wiki:c", 1)],
        );
        assert!(matches!(
            apply_participation(&mut projection, registry, &changed),
            Err(ParticipationError::Conflict(_))
        ));
    }

    #[test]
    fn applied_participation_is_navigable_as_one_context() {
        let registry = native_m_registry();
        let triad = participation(
            "wiki:triad/abc",
            ParticipationForm::Triad,
            vec![
                wiki_member("wiki:a", 2),
                wiki_member("wiki:b", 4),
                wiki_member("wiki:c", 1),
            ],
        );
        let mut projection = projection();
        apply_participation(&mut projection, registry, &triad).unwrap();
        let context = participation_context(&projection, "wiki:triad/abc", 1);
        assert_eq!(context.values().flatten().count(), 3);
    }
}
