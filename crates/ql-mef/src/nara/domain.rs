//! Complete source-qualified M4/Nara domain above the accepted Personal receiver.
//!
//! Each source-defined M4 branch remains a distinct module and office. Historical
//! C identities are preserved where authoritative, while old stubs and placeholder
//! behaviours are not promoted into present implementation truth.

#[cfg(test)]
mod acceptance;
#[path = "activity_aw3.rs"]
mod activity_aw3;
mod agent_world;
mod capability;
mod common;
mod context;
mod embodied;
mod identity;
mod integration;
#[path = "domain_operations.rs"]
pub mod operations;
mod oracle;
mod oracle_field;
mod state;
mod transformation;
mod transformation_field;

// `activity_aw3.rs` is physically beside the Central activity owner. This small
// alias lets it remain a child of this compact domain facade without reopening
// the accepted, much larger Personal producer merely to add one module line.
mod activity {
    pub use crate::nara::activity::NaraActivityLog;
}

pub use activity_aw3::*;
pub use agent_world::*;
pub use capability::*;
pub use common::{
    CENTRE_COUNT, CONTEXT_BRANCH_COUNT, ELEMENT_COUNT, EvidenceStanding, IDENTITY_SLOT_COUNT,
    INTEGRATION_OFFICE_COUNT, M4_DOMAIN_CONTRACT, M4Branch, ProtectedRef,
};
pub use context::*;
pub use embodied::*;
pub use identity::*;
pub use integration::*;
pub use oracle::*;
pub use oracle_field::*;
pub use state::*;
pub use transformation::*;
pub use transformation_field::*;

pub(crate) use common::{check_optional_finite, check_refs, check_source, check_text};