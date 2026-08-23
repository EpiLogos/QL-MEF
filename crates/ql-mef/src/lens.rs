use core::fmt;
use core::str::FromStr;

use ql_core::QlFace;

use crate::MefError;

pub const MEF_REGISTRY_REVISION: u16 = 1;
pub const MEF_REGISTRY_VERSION: &str = "1.0.0-q2";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LensFace {
    Day,
    Night,
}

impl LensFace {
    /// Bind the existing Day/Night lens face to the shared direct/prime kernel
    /// face without creating a second MEF coordinate system.
    pub const fn kernel_face(self) -> QlFace {
        match self {
            Self::Day => QlFace::Direct,
            Self::Night => QlFace::Conjugate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MefSquare {
    Articulation,
    Encounter,
    Becoming,
}

impl MefSquare {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Articulation => "A",
            Self::Encounter => "B",
            Self::Becoming => "C",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Articulation => "Articulation",
            Self::Encounter => "Encounter",
            Self::Becoming => "Becoming",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LensId {
    L0,
    L0Prime,
    L1,
    L1Prime,
    L2,
    L2Prime,
    L3,
    L3Prime,
    L4,
    L4Prime,
    L5,
    L5Prime,
}

impl LensId {
    pub const ALL: [Self; 12] = [
        Self::L0,
        Self::L0Prime,
        Self::L1,
        Self::L1Prime,
        Self::L2,
        Self::L2Prime,
        Self::L3,
        Self::L3Prime,
        Self::L4,
        Self::L4Prime,
        Self::L5,
        Self::L5Prime,
    ];

    pub const fn index(self) -> u8 {
        match self {
            Self::L0 | Self::L0Prime => 0,
            Self::L1 | Self::L1Prime => 1,
            Self::L2 | Self::L2Prime => 2,
            Self::L3 | Self::L3Prime => 3,
            Self::L4 | Self::L4Prime => 4,
            Self::L5 | Self::L5Prime => 5,
        }
    }

    pub const fn face(self) -> LensFace {
        match self {
            Self::L0 | Self::L1 | Self::L2 | Self::L3 | Self::L4 | Self::L5 => LensFace::Day,
            Self::L0Prime
            | Self::L1Prime
            | Self::L2Prime
            | Self::L3Prime
            | Self::L4Prime
            | Self::L5Prime => LensFace::Night,
        }
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::L0 => "L0",
            Self::L0Prime => "L0'",
            Self::L1 => "L1",
            Self::L1Prime => "L1'",
            Self::L2 => "L2",
            Self::L2Prime => "L2'",
            Self::L3 => "L3",
            Self::L3Prime => "L3'",
            Self::L4 => "L4",
            Self::L4Prime => "L4'",
            Self::L5 => "L5",
            Self::L5Prime => "L5'",
        }
    }

    pub const fn square(self) -> MefSquare {
        match self.index() {
            0 | 5 => MefSquare::Articulation,
            1 | 4 => MefSquare::Encounter,
            _ => MefSquare::Becoming,
        }
    }

    pub const fn conjugate_twin(self) -> Self {
        match self {
            Self::L0 => Self::L0Prime,
            Self::L0Prime => Self::L0,
            Self::L1 => Self::L1Prime,
            Self::L1Prime => Self::L1,
            Self::L2 => Self::L2Prime,
            Self::L2Prime => Self::L2,
            Self::L3 => Self::L3Prime,
            Self::L3Prime => Self::L3,
            Self::L4 => Self::L4Prime,
            Self::L4Prime => Self::L4,
            Self::L5 => Self::L5Prime,
            Self::L5Prime => Self::L5,
        }
    }

    pub const fn same_face_complement(self) -> Self {
        match self {
            Self::L0 => Self::L5,
            Self::L5 => Self::L0,
            Self::L1 => Self::L4,
            Self::L4 => Self::L1,
            Self::L2 => Self::L3,
            Self::L3 => Self::L2,
            Self::L0Prime => Self::L5Prime,
            Self::L5Prime => Self::L0Prime,
            Self::L1Prime => Self::L4Prime,
            Self::L4Prime => Self::L1Prime,
            Self::L2Prime => Self::L3Prime,
            Self::L3Prime => Self::L2Prime,
        }
    }

    pub const fn mobius_partner(self) -> Self {
        match self {
            Self::L0 => Self::L5Prime,
            Self::L5Prime => Self::L0,
            Self::L5 => Self::L0Prime,
            Self::L0Prime => Self::L5,
            Self::L1 => Self::L4Prime,
            Self::L4Prime => Self::L1,
            Self::L4 => Self::L1Prime,
            Self::L1Prime => Self::L4,
            Self::L2 => Self::L3Prime,
            Self::L3Prime => Self::L2,
            Self::L3 => Self::L2Prime,
            Self::L2Prime => Self::L3,
        }
    }
}

impl fmt::Display for LensId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl FromStr for LensId {
    type Err = MefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "L0" => Ok(Self::L0),
            "L0'" => Ok(Self::L0Prime),
            "L1" => Ok(Self::L1),
            "L1'" => Ok(Self::L1Prime),
            "L2" => Ok(Self::L2),
            "L2'" => Ok(Self::L2Prime),
            "L3" => Ok(Self::L3),
            "L3'" => Ok(Self::L3Prime),
            "L4" => Ok(Self::L4),
            "L4'" => Ok(Self::L4Prime),
            "L5" => Ok(Self::L5),
            "L5'" => Ok(Self::L5Prime),
            other => Err(MefError::UnknownLens(other.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LensRef {
    lens: LensId,
    registry_revision: u16,
}

impl LensRef {
    pub const fn canonical(lens: LensId) -> Self {
        Self {
            lens,
            registry_revision: MEF_REGISTRY_REVISION,
        }
    }

    pub fn new(lens: LensId, registry_revision: u16) -> Result<Self, MefError> {
        if registry_revision != MEF_REGISTRY_REVISION {
            return Err(MefError::UnsupportedRegistryVersion(registry_revision));
        }
        Ok(Self {
            lens,
            registry_revision,
        })
    }

    pub const fn lens(self) -> LensId {
        self.lens
    }

    pub const fn registry_revision(self) -> u16 {
        self.registry_revision
    }
}

impl fmt::Display for LensRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mef:lens:{}@{}", self.lens, self.registry_revision)
    }
}

impl FromStr for LensRef {
    type Err = MefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value
            .strip_prefix("mef:lens:")
            .ok_or_else(|| MefError::InvalidLensRef(value.to_owned()))?;
        let (lens, version) = raw
            .rsplit_once('@')
            .ok_or_else(|| MefError::InvalidLensRef(value.to_owned()))?;
        let lens = lens.parse::<LensId>()?;
        let version = version
            .parse::<u16>()
            .map_err(|_| MefError::InvalidLensRef(value.to_owned()))?;
        Self::new(lens, version)
    }
}
