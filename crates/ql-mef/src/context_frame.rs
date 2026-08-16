use ql_core::QlPosition;

use crate::{LensId, MefRotation, MefUnitFace};

pub const CONTEXT_FRAME_GRAMMAR_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextFrameId {
    Cf1,
    Cf2,
    Cf3,
    Cf4,
    Cf5,
    Cf6,
    Cf7,
}

impl ContextFrameId {
    pub const ALL: [Self; 7] = [
        Self::Cf1,
        Self::Cf2,
        Self::Cf3,
        Self::Cf4,
        Self::Cf5,
        Self::Cf6,
        Self::Cf7,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::Cf1 => "CF1",
            Self::Cf2 => "CF2",
            Self::Cf3 => "CF3",
            Self::Cf4 => "CF4",
            Self::Cf5 => "CF5",
            Self::Cf6 => "CF6",
            Self::Cf7 => "CF7",
        }
    }

    pub const fn expression(self) -> &'static str {
        match self {
            Self::Cf1 => "(00/00)",
            Self::Cf2 => "(0/1)",
            Self::Cf3 => "(0/1/2)",
            Self::Cf4 => "(0/1/2/3)",
            Self::Cf5 => "(4.0/1-4.4/5)",
            Self::Cf6 => "(4.5/0)",
            Self::Cf7 => "(5/0)",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Cf1 => "Fourfold-Zero / undifferentiated-ground",
            Self::Cf2 => "Non-Dual Anchor / standing-identity",
            Self::Cf3 => "Dual-Non-Dual / triadic-circulation",
            Self::Cf4 => "Trinitarian / tetradic-prehensive-closure",
            Self::Cf5 => "Fractal-Doubling Executive",
            Self::Cf6 => ".5 Bridge",
            Self::Cf7 => "Total Synthesis / Mobius cyclic-closure-and-reopening",
        }
    }

    pub fn canonical_selection(self) -> ContextFrameSelection {
        match self {
            Self::Cf1 => ContextFrameSelection::new(self, 0, MefUnitFace::Name, MefGrain::OuterTwo),
            Self::Cf2 => ContextFrameSelection::new(self, 1, MefUnitFace::Name, MefGrain::InnerFour),
            Self::Cf3 => ContextFrameSelection::new(self, 2, MefUnitFace::Name, MefGrain::InnerFour),
            Self::Cf4 => ContextFrameSelection::new(self, 2, MefUnitFace::Power, MefGrain::InnerFour),
            Self::Cf5 => ContextFrameSelection::new(self, 3, MefUnitFace::Power, MefGrain::InnerFour),
            Self::Cf6 => ContextFrameSelection::new(self, 4, MefUnitFace::Power, MefGrain::InnerFour),
            Self::Cf7 => ContextFrameSelection::new(self, 5, MefUnitFace::Power, MefGrain::OuterTwo),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MefGrain {
    InnerFour,
    OuterTwo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextFrameSelection {
    frame: ContextFrameId,
    local_position: QlPosition,
    unit_face: MefUnitFace,
    grain: MefGrain,
}

impl ContextFrameSelection {
    fn new(frame: ContextFrameId, local_position: u8, unit_face: MefUnitFace, grain: MefGrain) -> Self {
        Self {
            frame,
            local_position: position(local_position),
            unit_face,
            grain,
        }
    }

    pub const fn frame(self) -> ContextFrameId {
        self.frame
    }

    pub const fn local_position(self) -> QlPosition {
        self.local_position
    }

    pub const fn unit_face(self) -> MefUnitFace {
        self.unit_face
    }

    pub const fn grain(self) -> MefGrain {
        self.grain
    }

    pub fn at_lens(self, lens: LensId) -> ContextFrameCoordinate {
        let coordinate = MefFormCoordinate::new(lens, self.local_position, self.unit_face, self.grain);
        ContextFrameCoordinate {
            frame: self.frame,
            coordinate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MefFormCoordinate {
    lens: LensId,
    local_position: QlPosition,
    absolute_position: QlPosition,
    unit_face: MefUnitFace,
    grain: MefGrain,
}

impl MefFormCoordinate {
    fn new(lens: LensId, local_position: QlPosition, unit_face: MefUnitFace, grain: MefGrain) -> Self {
        let rotation = MefRotation::new(lens, local_position);
        Self {
            lens,
            local_position,
            absolute_position: rotation.absolute_position(),
            unit_face,
            grain,
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

    pub const fn unit_face(self) -> MefUnitFace {
        self.unit_face
    }

    pub const fn grain(self) -> MefGrain {
        self.grain
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextFrameCoordinate {
    frame: ContextFrameId,
    coordinate: MefFormCoordinate,
}

impl ContextFrameCoordinate {
    pub const fn frame(self) -> ContextFrameId {
        self.frame
    }

    pub const fn coordinate(self) -> MefFormCoordinate {
        self.coordinate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextFrameCut {
    lens: LensId,
    selected: [ContextFrameCoordinate; 7],
    complexification_hooks: [MefFormCoordinate; 3],
    unpicked_outer_anchors: [MefFormCoordinate; 2],
}

impl ContextFrameCut {
    pub fn canonical(lens: LensId) -> Self {
        let selected = canonical_context_frame_progression().map(|selection| selection.at_lens(lens));
        let complexification_hooks = [
            MefFormCoordinate::new(lens, position(3), MefUnitFace::Name, MefGrain::InnerFour),
            MefFormCoordinate::new(lens, position(4), MefUnitFace::Name, MefGrain::InnerFour),
            MefFormCoordinate::new(lens, position(1), MefUnitFace::Power, MefGrain::InnerFour),
        ];
        let unpicked_outer_anchors = [
            MefFormCoordinate::new(lens, position(5), MefUnitFace::Name, MefGrain::OuterTwo),
            MefFormCoordinate::new(lens, position(0), MefUnitFace::Power, MefGrain::OuterTwo),
        ];

        Self {
            lens,
            selected,
            complexification_hooks,
            unpicked_outer_anchors,
        }
    }

    pub const fn lens(self) -> LensId {
        self.lens
    }

    pub const fn selected(&self) -> &[ContextFrameCoordinate; 7] {
        &self.selected
    }

    pub const fn complexification_hooks(&self) -> &[MefFormCoordinate; 3] {
        &self.complexification_hooks
    }

    pub const fn unpicked_outer_anchors(&self) -> &[MefFormCoordinate; 2] {
        &self.unpicked_outer_anchors
    }
}

pub fn canonical_context_frame_progression() -> [ContextFrameSelection; 7] {
    ContextFrameId::ALL.map(ContextFrameId::canonical_selection)
}

fn position(value: u8) -> QlPosition {
    match QlPosition::new(value) {
        Ok(position) => position,
        Err(_) => unreachable!("canonical context-frame positions are modulo-six positions"),
    }
}
