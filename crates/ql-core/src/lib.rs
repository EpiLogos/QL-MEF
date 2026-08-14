//! Deterministic executable QL kernel.
//!
//! Q1 intentionally contains no semantic inference and no Loop Runtime dependency.

mod address;
mod address_parse;
mod apply;
mod deterministic;
mod error;
mod face;
mod form;
mod kernel;
mod operator;
mod position;

pub use address::QlAddress;
pub use apply::apply_operator;
pub use deterministic::{DeterministicProvenance, DeterministicResult};
pub use error::QlError;
pub use face::QlFace;
pub use form::{QlForm, QlFormRef};
pub use kernel::{KERNEL_VERSION, KernelCapabilities, SCHEMA_VERSION, kernel_capabilities};
pub use operator::{FourPlusTwoClass, OperatorValue, QlOperator};
pub use position::QlPosition;
