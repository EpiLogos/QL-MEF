use core::fmt;
use core::str::FromStr;

use crate::{QlAddress, QlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QlOperator {
    ConjugateAddress,
    ComplementAddress,
    ClassifyFourPlusTwo,
}

impl QlOperator {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConjugateAddress => "conjugate-address",
            Self::ComplementAddress => "complement-address",
            Self::ClassifyFourPlusTwo => "classify-four-plus-two",
        }
    }
}

impl FromStr for QlOperator {
    type Err = QlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "conjugate-address" => Ok(Self::ConjugateAddress),
            "complement-address" => Ok(Self::ComplementAddress),
            "classify-four-plus-two" => Ok(Self::ClassifyFourPlusTwo),
            other => Err(QlError::UnknownOperator(other.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FourPlusTwoClass {
    Implicate,
    Explicate,
}

impl fmt::Display for FourPlusTwoClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Implicate => "implicate",
            Self::Explicate => "explicate",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorValue {
    Address(QlAddress),
    FourPlusTwo(FourPlusTwoClass),
}

impl fmt::Display for OperatorValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address(address) => address.fmt(f),
            Self::FourPlusTwo(class) => class.fmt(f),
        }
    }
}
