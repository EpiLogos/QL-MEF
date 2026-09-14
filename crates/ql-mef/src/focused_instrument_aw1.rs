//! AW1 join for the K9 focused instrument.
//!
//! AW1 remains the Bimba/rooted-world owner. This module only converts one
//! already-resolved `RootedMWorld` into K9's selected-subject carrier so the
//! shared material field and Bimba navigation can point at the same object.

use crate::aw1_world::{AW1_ROOTED_WORLD_CONTRACT, RootedFace, RootedMWorld};
use crate::focused_instrument::{BIMBA_SELECTION_CONTRACT, BimbaSelection};

impl BimbaSelection {
    /// Build a focused selection from AW1's accepted rooted view. The caller
    /// supplies only the encounter-local selection/disclosure identities and
    /// optional K8 constituent identity; source/coordinate/revision/relation
    /// facts come from the AW1 owner result rather than being re-derived here.
    pub fn from_rooted_world(
        world: &RootedMWorld,
        selection_ref: impl Into<String>,
        disclosure_ref: impl Into<String>,
        subject_ref: impl Into<String>,
        field_constituent_ref: Option<String>,
    ) -> Result<Self, String> {
        if world.version != AW1_ROOTED_WORLD_CONTRACT {
            return Err("unsupported AW1 rooted-world contract".into());
        }
        if world.direct.face != RootedFace::Bimba
            || world.direct.source_ref != world.selected_source_ref
            || world.direct.coordinate_id != world.selected_id
            || world.conjugate.source_ref != world.selected_source_ref
            || world.conjugate.coordinate_id != world.selected_id
        {
            return Err("AW1 rooted world lost selected coordinate/face identity".into());
        }
        let mut assertion_refs = world
            .source_relation_ids
            .iter()
            .map(|id| format!("aw1:relation:{id:?}"))
            .collect::<Vec<_>>();
        assertion_refs.push(format!("aw1:registry:{}", world.registry_revision));
        assertion_refs.sort();
        assertion_refs.dedup();
        let selection = Self {
            contract: BIMBA_SELECTION_CONTRACT.into(),
            selection_ref: selection_ref.into(),
            coordinate_ref: world.direct.canonical_ref.clone(),
            source_ref: world.selected_source_ref.clone(),
            source_revision: world.registry_source_revision.clone(),
            disclosure_ref: disclosure_ref.into(),
            subject_ref: subject_ref.into(),
            field_constituent_ref,
            assertion_refs,
        };
        selection.validate()?;
        Ok(selection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aw1_world::resolve_rooted_m_world;
    use crate::m_tree::native_m_registry;

    #[test]
    fn focused_selection_uses_aw1_source_and_bimba_face_without_rebuilding_world() {
        let registry = native_m_registry();
        let world = resolve_rooted_m_world(registry, "#0").unwrap();
        let selection = BimbaSelection::from_rooted_world(
            &world,
            "selection:aw1",
            "disclosure:aw1",
            "nara:1",
            None,
        )
        .unwrap();
        assert_eq!(selection.source_ref, world.selected_source_ref);
        assert_eq!(selection.source_revision, world.registry_source_revision);
        assert_eq!(selection.coordinate_ref, world.direct.canonical_ref);
        assert!(selection.assertion_refs.iter().any(|reference| reference.contains("registry")));
    }
}
