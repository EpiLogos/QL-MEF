use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MefError {
    UnknownLens(String),
    UnsupportedRegistryVersion(u16),
    InvalidLensRef(String),
    InvalidSublensRef(String),
    InvalidSublensPosition(u8),
    SublensLensMismatch,
    EmptyClientRef,
    EmptyProviderId,
    EmptyProviderVersion,
    UnknownResultClass(String),
}

impl MefError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnknownLens(_) => "UNKNOWN_LENS",
            Self::UnsupportedRegistryVersion(_) => "UNSUPPORTED_REGISTRY_VERSION",
            Self::InvalidLensRef(_) => "INVALID_LENS_REF",
            Self::InvalidSublensRef(_) => "INVALID_SUBLENS_REF",
            Self::InvalidSublensPosition(_) => "INVALID_SUBLENS_POSITION",
            Self::SublensLensMismatch => "SUBLENS_LENS_MISMATCH",
            Self::EmptyClientRef => "EMPTY_CLIENT_REF",
            Self::EmptyProviderId => "EMPTY_PROVIDER_ID",
            Self::EmptyProviderVersion => "EMPTY_PROVIDER_VERSION",
            Self::UnknownResultClass(_) => "UNKNOWN_RESULT_CLASS",
        }
    }
}

impl fmt::Display for MefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for MefError {}
