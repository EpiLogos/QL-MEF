use core::fmt;
use core::str::FromStr;

use crate::QlError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QlForm {
    Sixfold,
    FourPlusTwo,
    DirectConjugate,
}

impl QlForm {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sixfold => "sixfold",
            Self::FourPlusTwo => "four-plus-two",
            Self::DirectConjugate => "direct-conjugate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlFormRef {
    form: QlForm,
    version: u16,
}

impl QlFormRef {
    pub const SIXFOLD_V1: Self = Self {
        form: QlForm::Sixfold,
        version: 1,
    };
    pub const FOUR_PLUS_TWO_V1: Self = Self {
        form: QlForm::FourPlusTwo,
        version: 1,
    };
    pub const DIRECT_CONJUGATE_V1: Self = Self {
        form: QlForm::DirectConjugate,
        version: 1,
    };

    pub fn new(form: QlForm, version: u16) -> Result<Self, QlError> {
        if version != 1 {
            return Err(QlError::UnsupportedFormVersion {
                form: form.as_str(),
                version,
            });
        }
        Ok(Self { form, version })
    }

    pub const fn form(self) -> QlForm {
        self.form
    }
    pub const fn version(self) -> u16 {
        self.version
    }
}

impl fmt::Display for QlFormRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "qlform:{}@{}", self.form.as_str(), self.version)
    }
}

impl FromStr for QlFormRef {
    type Err = QlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value
            .strip_prefix("qlform:")
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?;
        let (form, version) = raw
            .rsplit_once('@')
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?;
        let form = match form {
            "sixfold" => QlForm::Sixfold,
            "four-plus-two" => QlForm::FourPlusTwo,
            "direct-conjugate" => QlForm::DirectConjugate,
            other => return Err(QlError::UnknownForm(other.to_owned())),
        };
        let version = version
            .parse::<u16>()
            .map_err(|_| QlError::InvalidAddress(value.to_owned()))?;
        Self::new(form, version)
    }
}
