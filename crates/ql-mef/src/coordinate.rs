use ql_core::QlPosition;

use crate::{LensFace, LensId};

pub const MEF_ROTATION_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MefUnitFace {
    Name,
    Power,
}

impl MefUnitFace {
    pub const fn opposite(self) -> Self {
        match self {
            Self::Name => Self::Power,
            Self::Power => Self::Name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MefRotation {
    lens: LensId,
    local_position: QlPosition,
    absolute_position: QlPosition,
    leading_unit: MefUnitFace,
}

impl MefRotation {
    pub fn new(lens: LensId, local_position: QlPosition) -> Self {
        let absolute_value = (lens.index() + local_position.value()) % 6;
        let absolute_position = match QlPosition::new(absolute_value) {
            Ok(position) => position,
            Err(_) => unreachable!("modulo-six MEF rotation must stay inside QL positions"),
        };
        let leading_unit = match lens.face() {
            LensFace::Day => MefUnitFace::Name,
            LensFace::Night => MefUnitFace::Power,
        };

        Self {
            lens,
            local_position,
            absolute_position,
            leading_unit,
        }
    }

    pub const fn lens(self) -> LensId {
        self.lens
    }

    pub const fn local_position(self) -> QlPosition {
        self.local_position
    }

    pub const fn absolute_position(self) -> QlPosition {
        self.absolute_position
    }

    pub const fn leading_unit(self) -> MefUnitFace {
        self.leading_unit
    }

    pub const fn companion_unit(self) -> MefUnitFace {
        self.leading_unit.opposite()
    }
}
