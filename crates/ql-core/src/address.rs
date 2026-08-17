use core::fmt;

use crate::{QlError, QlFace, QlForm, QlFormRef, QlPosition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlAddress {
    frame: QlFormRef,
    position: QlPosition,
    face: QlFace,
    depth: u32,
}

impl QlAddress {
    pub fn new(
        frame: QlFormRef,
        position: QlPosition,
        face: QlFace,
        depth: u32,
    ) -> Result<Self, QlError> {
        if frame.form() != QlForm::Sixfold {
            return Err(QlError::UnsupportedAddressFrame {
                form: frame.form().as_str(),
                version: frame.version(),
            });
        }
        Ok(Self {
            frame,
            position,
            face,
            depth,
        })
    }

    pub fn sixfold(position: u8, face: QlFace, depth: u32) -> Result<Self, QlError> {
        Self::new(
            QlFormRef::SIXFOLD_V1,
            QlPosition::new(position)?,
            face,
            depth,
        )
    }

    pub const fn frame(self) -> QlFormRef {
        self.frame
    }
    pub const fn position(self) -> QlPosition {
        self.position
    }
    pub const fn face(self) -> QlFace {
        self.face
    }
    pub const fn depth(self) -> u32 {
        self.depth
    }

    pub fn with_face(self, face: QlFace) -> Self {
        Self { face, ..self }
    }

    pub fn with_position(self, position: QlPosition) -> Self {
        Self { position, ..self }
    }
}

impl fmt::Display for QlAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "qladdr:{}@{}/{}/P{}/d{}",
            self.frame.form().as_str(),
            self.frame.version(),
            self.face,
            self.position.value(),
            self.depth
        )
    }
}
