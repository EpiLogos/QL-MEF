use core::fmt;

use ql_semantic::{Operation, ProviderError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceError {
    ProviderAbsent,
    ProviderIncompatible(Option<String>),
    UnsupportedOperation(Operation),
    InvalidRequest(&'static str),
    InputLimitExceeded {
        operation: Operation,
        limit: usize,
        actual: usize,
    },
    Provider(ProviderError),
}

impl From<ProviderError> for ServiceError {
    fn from(value: ProviderError) -> Self {
        Self::Provider(value)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProviderAbsent => f.write_str("QL provider is absent"),
            Self::ProviderIncompatible(detail) => write!(
                f,
                "QL provider is incompatible: {}",
                detail.as_deref().unwrap_or("unspecified")
            ),
            Self::UnsupportedOperation(operation) => {
                write!(f, "provider does not advertise {}", operation.as_str())
            }
            Self::InvalidRequest(message) => write!(f, "invalid service request: {message}"),
            Self::InputLimitExceeded {
                operation,
                limit,
                actual,
            } => write!(
                f,
                "{} input limit exceeded: {actual} > {limit}",
                operation.as_str()
            ),
            Self::Provider(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ServiceError {}
