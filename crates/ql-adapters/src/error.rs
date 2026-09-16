use core::fmt;

use ql_mef::MefError;
use ql_service::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    ServiceUnavailable,
    InvalidRefraction(MefError),
    QlRequired(ServiceError),
    InvalidTechneReading(String),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceUnavailable => {
                f.write_str("QL is required but no QL service was supplied")
            }
            Self::InvalidRefraction(error) => write!(f, "invalid QL refraction contract: {error}"),
            Self::QlRequired(error) => write!(f, "required QL operation failed: {error}"),
            Self::InvalidTechneReading(message) => {
                write!(f, "invalid ql.techne/v1 contract: {message}")
            }
        }
    }
}

impl std::error::Error for AdapterError {}
