use core::fmt;

use crate::Operation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderError {
    UnsupportedOperation(Operation),
    InvalidRequest(&'static str),
    Failed(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOperation(operation) => {
                write!(f, "provider does not support {}", operation.as_str())
            }
            Self::InvalidRequest(message) => write!(f, "invalid provider request: {message}"),
            Self::Failed(message) => write!(f, "provider failed: {message}"),
        }
    }
}

impl std::error::Error for ProviderError {}
