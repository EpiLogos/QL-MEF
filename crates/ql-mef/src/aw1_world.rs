//! Rooted AW1 view over the existing native M registry.
//!
//! `#` is the kernel taproot named by the agent-world lock. `M` is the M-family
//! master in the generated registry. They are deliberately kept distinct: the
//! registry begins at M because it is an M-family source projection, not because
//! M silently becomes the universal kernel root.

use serde::{Deserialize, Serialize};

use crate::m_tree::{MRegistry, MTreeBinding, MTreeId};
use crate::MFace;

pub const AW1_ROOTED_WORLD_CONTRACT: &str = "ql.aw1-rooted-m-world/v1";
pub const KERNEL_TAPROOT_REF: &str = "#";
pub const M_FAMILY_MASTER_REF: &str = "M";

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RootedFace {
    Bimba,
    Pratibimba,
}

impl RootedFace {
    pub const fn from_m_face(face: MFace) -> Self {
        match face {
            MFace::Bimba => Self::Bimba,
            MFace::Pratibimba => Self::Pratibimba,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bimba => "bimba",
            Self::Pratibimba => "pratibimba",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootedStep {
    pub id: MTreeId,
    pub source_ref: String,
    pub depth: usize,
    pub parent_id: Option<MTreeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaceProjection {
    pub coordinate_id: MTreeId,
    pub source_ref: String,
    pub canonical_ref: String,
    pub face: RootedFace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NativeOwnerStanding {
    Bound,
    Unbound,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeOwnerDisposition {
    pub standing: NativeOwnerStanding,
    pub bindings: Vec<MTreeBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootedMWorld {
    pub version: String,
    pub registry_revision: String,
    pub registry_source_repository: String,
    pub registry_source_revision: String,
    pub kernel_taproot_ref: String,
    pub family_master_id: MTreeId,
    pub family_master_ref: String,
    pub root_position: u8,
    pub root_id: MTreeId,
    pub selected_id: MTreeId,
    pub selected_source_ref: String,
    pub ancestry: Vec<RootedStep>,
    pub direct: FaceProjection,
    pub conjugate: FaceProjection,
    pub source_relation_ids: Vec<MTreeId>,
    pub native_owner: NativeOwnerDisposition,
}

fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn face_projection(
    registry: &MRegistry,
    source_ref: &str,
    face: MFace,
) -> Result<FaceProjection> {
    let node = registry.resolve(source_ref).ok_or("unknown M coordinate")?;
    let coordinate = registry.coordinate(source_ref, face)?;
    require(
        coordinate.source_ref == node.source_ref,
        "face projection changed source identity",
    )?;
    Ok(FaceProjection {
        coordinate_id: node.id,
        source_ref: coordinate.source_ref,
        canonical_ref: coordinate.canonical_ref(),
        face: RootedFace::from_m_face(coordinate.face),
    })
}

/// Resolve one source coordinate through the existing registry as:
/// kernel taproot -> M family master -> source root -> recursive descendants,
/// then expose direct and conjugate faces and the current implementation-owner
/// disposition. No missing binding is upgraded into an inferred owner.
pub fn resolve_rooted_m_world(registry: &MRegistry, reference: &str) -> Result<RootedMWorld> {
    let selected = registry
        .resolve(reference)
        .ok_or("unknown rooted M coordinate")?;
    require(
        selected.source_ref != M_FAMILY_MASTER_REF,
        "select a source M coordinate rather than the family master",
    )?;
    let root_position = selected
        .root_position
        .ok_or("source M coordinate has no root position")?;
    let root = registry
        .root(usize::from(root_position))
        .ok_or("source M coordinate has no registered root")?;
    require(
        selected.root_id == root.id,
        "selected coordinate disagrees with its registered root",
    )?;

    let mut reverse = Vec::new();
    let mut cursor = Some(selected.id);
    while let Some(id) = cursor {
        let node = registry.node(id).ok_or("broken M ancestry")?;
        reverse.push(RootedStep {
            id: node.id,
            source_ref: node.source_ref.clone(),
            depth: node.depth,
            parent_id: node.parent_id,
        });
        cursor = node.parent_id;
    }
    reverse.reverse();
    require(
        reverse.first().is_some_and(|step| {
            step.id == registry.manifest().master_id && step.source_ref == M_FAMILY_MASTER_REF
        }),
        "M ancestry is not rooted in the family master",
    )?;
    require(
        reverse
            .get(1)
            .is_some_and(|step| step.id == root.id && step.source_ref == root.source_ref),
        "M ancestry does not pass through its declared source root",
    )?;
    require(
        reverse.last().is_some_and(|step| step.id == selected.id),
        "M ancestry does not terminate at the selected coordinate",
    )?;

    let mut source_relation_ids = registry
        .relations_for(selected.id)
        .map(|relation| relation.id)
        .collect::<Vec<_>>();
    source_relation_ids.sort();
    source_relation_ids.dedup();

    let mut bindings = registry
        .manifest()
        .bindings
        .iter()
        .filter(|binding| binding.coordinate_id == Some(selected.id))
        .cloned()
        .collect::<Vec<_>>();
    bindings.sort_by(|left, right| left.identity.cmp(&right.identity));
    for binding in &bindings {
        registry.validate_binding(binding)?;
    }
    let standing = if bindings.is_empty() {
        NativeOwnerStanding::Unbound
    } else {
        NativeOwnerStanding::Bound
    };

    Ok(RootedMWorld {
        version: AW1_ROOTED_WORLD_CONTRACT.into(),
        registry_revision: registry.manifest().registry_revision.clone(),
        registry_source_repository: registry.manifest().source_repository.clone(),
        registry_source_revision: registry.manifest().source_revision.clone(),
        kernel_taproot_ref: KERNEL_TAPROOT_REF.into(),
        family_master_id: registry.manifest().master_id,
        family_master_ref: M_FAMILY_MASTER_REF.into(),
        root_position,
        root_id: root.id,
        selected_id: selected.id,
        selected_source_ref: selected.source_ref.clone(),
        ancestry: reverse,
        direct: face_projection(registry, &selected.source_ref, MFace::Bimba)?,
        conjugate: face_projection(registry, &selected.source_ref, MFace::Pratibimba)?,
        source_relation_ids,
        native_owner: NativeOwnerDisposition { standing, bindings },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m_tree::native_m_registry;

    #[test]
    fn kernel_taproot_and_m_family_master_are_not_collapsed() {
        let registry = native_m_registry();
        let world = resolve_rooted_m_world(registry, "#0").unwrap();
        assert_eq!(world.kernel_taproot_ref, "#");
        assert_eq!(world.family_master_ref, "M");
        assert_ne!(world.kernel_taproot_ref, world.family_master_ref);
        assert_eq!(world.ancestry.len(), 2);
        assert_eq!(world.ancestry[0].source_ref, "M");
        assert_eq!(world.ancestry[1].source_ref, "#0");
    }

    #[test]
    fn recursive_coordinate_keeps_exact_ancestry_and_both_faces() {
        let registry = native_m_registry();
        let source = registry
            .manifest()
            .nodes
            .iter()
            .find(|node| node.source_ref != "M" && node.depth > 1)
            .expect("fixture contains recursive M coordinates")
            .source_ref
            .clone();
        let world = resolve_rooted_m_world(registry, &source).unwrap();
        assert!(world.ancestry.len() > 2);
        assert_eq!(world.ancestry.last().unwrap().source_ref, source);
        assert_eq!(world.direct.coordinate_id, world.conjugate.coordinate_id);
        assert_eq!(world.direct.source_ref, world.conjugate.source_ref);
        assert_eq!(world.direct.face, RootedFace::Bimba);
        assert_eq!(world.conjugate.face, RootedFace::Pratibimba);
        assert_ne!(world.direct.canonical_ref, world.conjugate.canonical_ref);
    }

    #[test]
    fn native_owner_standing_is_only_what_the_registry_actually_binds() {
        let registry = native_m_registry();
        let bound_id = registry
            .manifest()
            .bindings
            .iter()
            .find_map(|binding| binding.coordinate_id)
            .expect("fixture contains at least one coordinate binding");
        let node = registry.node(bound_id).unwrap();
        let world = resolve_rooted_m_world(registry, &node.source_ref).unwrap();
        assert_eq!(world.native_owner.standing, NativeOwnerStanding::Bound);
        assert!(!world.native_owner.bindings.is_empty());
        assert!(world
            .native_owner
            .bindings
            .iter()
            .all(|binding| binding.coordinate_id == Some(world.selected_id)));
    }

    #[test]
    fn unknown_or_family_master_selection_fails_closed() {
        let registry = native_m_registry();
        assert!(resolve_rooted_m_world(registry, "#99").is_err());
        assert!(resolve_rooted_m_world(registry, "M").is_err());
    }
}
