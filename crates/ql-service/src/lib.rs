//! Replaceable, transport-independent service boundary for QL/MEF operations.

mod combine_ops;
mod error;
mod guard;
mod host;
mod negotiation;
mod protocol;
mod read_ops;
mod service;

pub use error::ServiceError;
pub use negotiation::{CapabilityDecision, ServiceCapabilities};
pub use protocol::{ServiceRequest, ServiceResponse};
pub use service::QlService;
