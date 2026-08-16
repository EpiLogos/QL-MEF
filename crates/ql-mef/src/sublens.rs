use core::fmt;
use core::str::FromStr;

use ql_core::QlPosition;

use crate::{coordinate::MefRotation, LensId, LensRef, MefError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SublensRef {
    lens: LensRef,
    position: QlPosition,
}

impl SublensRef {
    pub fn canonical(lens: LensId, position: u8) -> Result<Self, MefError> {
        Self::new(LensRef::canonical(lens), position)
    }

    pub fn new(lens: LensRef, position: u8) -> Result<Self, MefError> {
        let position =
            QlPosition::new(position).map_err(|_| MefError::InvalidSublensPosition(position))?;
        Ok(Self { lens, position })
    }

    pub const fn lens(self) -> LensRef {
        self.lens
    }

    pub const fn position(self) -> QlPosition {
        self.position
    }

    pub fn rotation(self) -> MefRotation {
        MefRotation::new(self.lens.lens(), self.position)
    }
}

impl fmt::Display for SublensRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "mef:sublens:{}.{}@{}",
            self.lens.lens(),
            self.position.value(),
            self.lens.registry_revision()
        )
    }
}

impl FromStr for SublensRef {
    type Err = MefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value
            .strip_prefix("mef:sublens:")
            .ok_or_else(|| MefError::InvalidSublensRef(value.to_owned()))?;
        let (coordinate, version) = raw
            .rsplit_once('@')
            .ok_or_else(|| MefError::InvalidSublensRef(value.to_owned()))?;
        let (lens, position) = coordinate
            .split_once('.')
            .ok_or_else(|| MefError::InvalidSublensRef(value.to_owned()))?;
        let lens = lens.parse::<LensId>()?;
        let version = version
            .parse::<u16>()
            .map_err(|_| MefError::InvalidSublensRef(value.to_owned()))?;
        let position = position
            .parse::<u8>()
            .map_err(|_| MefError::InvalidSublensRef(value.to_owned()))?;
        Self::new(LensRef::new(lens, version)?, position)
    }
}
