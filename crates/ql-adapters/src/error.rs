use core::fmt;

use ql_mef::MefError;
use ql_service::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    ServiceUnavailable,
    InvalidRefraction(MefError),
    QlRequired(ServiceError),
}

impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceUnavailable => {
                f.write_str("QL is required but no QL service was supplied")
            }
            Self::InvalidRefraction(error) => write!(f, "invalid QL refraction contract: {error}"),
            Self::QlRequired(error) => write!(f, "required QL operation failed: {error}"),
        }
    }
}

impl std::error::Error for AdapterError {}
