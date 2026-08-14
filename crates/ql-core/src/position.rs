use crate::QlError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QlPosition(u8);

impl QlPosition {
    pub fn new(value: u8) -> Result<Self, QlError> {
        if value > 5 {
            return Err(QlError::InvalidPosition(value));
        }
        Ok(Self(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }

    pub const fn complement(self) -> Self {
        Self(5 - self.0)
    }
}
