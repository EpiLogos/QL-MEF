//! Optional client adapters that preserve client-owned identity while attaching QL/MEF readings.
//!
//! Factory and AIKit remain semantic owners of their own refs. QL-MEF refracts those refs; it does
//! not rename them or make QL a hidden prerequisite for ordinary client behaviour.

mod aikit;
mod attachment;
mod client;
mod core;
mod error;
mod factory;

pub use aikit::AiKitAdapter;
pub use attachment::{AdapterResult, QlAttachment, QlMode};
pub use client::{AdapterSubject, AiKitSubject, ClientRecord, ClientSubject, FactorySubject};
pub use error::AdapterError;
pub use factory::FactoryAdapter;

pub(crate) use core::AdapterCore;
