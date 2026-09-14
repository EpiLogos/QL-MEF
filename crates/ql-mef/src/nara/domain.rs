//! Complete source-qualified M4/Nara domain above the accepted Personal receiver.
//!
//! Each source-defined M4 branch remains a distinct module and office. Historical
//! C identities are preserved where authoritative, while old stubs and placeholder
//! behaviours are not promoted into present implementation truth.

mod common;
mod context;
mod embodied;
mod identity;
mod integration;
pub mod operations;
mod oracle;
mod state;
mod transformation;

pub use common::{
    CENTRE_COUNT, CONTEXT_BRANCH_COUNT, ELEMENT_COUNT, EvidenceStanding, IDENTITY_SLOT_COUNT,
    INTEGRATION_OFFICE_COUNT, M4_DOMAIN_CONTRACT, M4Branch, ProtectedRef,
};
pub use context::*;
pub use embodied::*;
pub use identity::*;
pub use integration::*;
pub use oracle::*;
pub use state::*;
pub use transformation::*;

pub(crate) use common::{check_optional_finite, check_refs, check_source, check_text};
