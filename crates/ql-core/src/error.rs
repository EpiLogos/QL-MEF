use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QlError {
    UnknownForm(String),
    UnsupportedFormVersion { form: &'static str, version: u16 },
    UnsupportedAddressFrame { form: &'static str, version: u16 },
    InvalidPosition(u8),
    InvalidAddress(String),
    UnknownOperator(String),
    InvalidPoleValue { field: &'static str, value: u32 },
}

impl QlError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnknownForm(_) => "UNKNOWN_FORM",
            Self::UnsupportedFormVersion { .. } => "UNSUPPORTED_FORM_VERSION",
            Self::UnsupportedAddressFrame { .. } => "UNSUPPORTED_ADDRESS_FRAME",
            Self::InvalidPosition(_) => "INVALID_POSITION",
            Self::InvalidAddress(_) => "INVALID_ADDRESS",
            Self::UnknownOperator(_) => "UNKNOWN_OPERATOR",
            Self::InvalidPoleValue { .. } => "INVALID_POLE_VALUE",
        }
    }
}

impl fmt::Display for QlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownForm(form) => write!(f, "unknown QL form: {form}"),
            Self::UnsupportedFormVersion { form, version } => {
                write!(f, "unsupported QL form version: {form}@{version}")
            }
            Self::UnsupportedAddressFrame { form, version } => {
                write!(f, "unsupported QL address frame: {form}@{version}")
            }
            Self::InvalidPosition(position) => {
                write!(f, "invalid QL position P{position}; expected P0..P5")
            }
            Self::InvalidAddress(address) => write!(f, "invalid canonical QL address: {address}"),
            Self::UnknownOperator(operator) => {
                write!(f, "unsupported deterministic QL operator: {operator}")
            }
            Self::InvalidPoleValue { field, value } => {
                write!(f, "invalid physical-pole value for {field}: {value}")
            }
        }
    }
}

impl std::error::Error for QlError {}
