use crate::{QlFormRef, QlOperator};

pub const KERNEL_VERSION: &str = "0.1.0-q1";
pub const SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelCapabilities {
    pub kernel_version: &'static str,
    pub schema_version: &'static str,
    pub supported_forms: [QlFormRef; 3],
    pub deterministic_operations: [QlOperator; 3],
    pub stochastic_operations: [&'static str; 0],
    pub research_operations: [&'static str; 0],
}

pub const fn kernel_capabilities() -> KernelCapabilities {
    KernelCapabilities {
        kernel_version: KERNEL_VERSION,
        schema_version: SCHEMA_VERSION,
        supported_forms: [
            QlFormRef::SIXFOLD_V1,
            QlFormRef::FOUR_PLUS_TWO_V1,
            QlFormRef::DIRECT_CONJUGATE_V1,
        ],
        deterministic_operations: [
            QlOperator::ConjugateAddress,
            QlOperator::ComplementAddress,
            QlOperator::ClassifyFourPlusTwo,
        ],
        stochastic_operations: [],
        research_operations: [],
    }
}
