mod articulation;
mod becoming;
mod encounter;

use crate::{LensId, LensRef, MefError, MefSquare, SublensRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LensDefinition {
    id: LensId,
    name: &'static str,
    sublens_labels: [&'static str; 6],
}

impl LensDefinition {
    pub(crate) const fn new(id: LensId, name: &'static str, sublens_labels: [&'static str; 6]) -> Self {
        Self { id, name, sublens_labels }
    }

    pub const fn id(self) -> LensId { self.id }
    pub const fn name(self) -> &'static str { self.name }
    pub const fn square(self) -> MefSquare { self.id.square() }
    pub const fn reference(self) -> LensRef { LensRef::canonical(self.id) }
    pub const fn sublens_labels(self) -> [&'static str; 6] { self.sublens_labels }

    pub fn sublens(self, position: u8) -> Result<SublensDefinition, MefError> {
        let reference = SublensRef::canonical(self.id, position)?;
        Ok(SublensDefinition { reference, label: self.sublens_labels[position as usize] })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SublensDefinition {
    reference: SublensRef,
    label: &'static str,
}

impl SublensDefinition {
    pub const fn reference(self) -> SublensRef { self.reference }
    pub const fn label(self) -> &'static str { self.label }
}

pub const LENS_DEFINITIONS: [LensDefinition; 12] = [
    articulation::L0,
    articulation::L0_PRIME,
    encounter::L1,
    encounter::L1_PRIME,
    becoming::L2,
    becoming::L2_PRIME,
    becoming::L3,
    becoming::L3_PRIME,
    encounter::L4,
    encounter::L4_PRIME,
    articulation::L5,
    articulation::L5_PRIME,
];

pub const fn all_lens_definitions() -> &'static [LensDefinition; 12] { &LENS_DEFINITIONS }

pub fn lens_definition(id: LensId) -> &'static LensDefinition {
    LENS_DEFINITIONS.iter().find(|definition| definition.id == id).unwrap_or(&LENS_DEFINITIONS[0])
}

pub fn all_sublens_definitions() -> impl Iterator<Item = SublensDefinition> {
    LENS_DEFINITIONS.iter().flat_map(|lens| {
        (0_u8..=5).filter_map(move |position| lens.sublens(position).ok())
    })
}
