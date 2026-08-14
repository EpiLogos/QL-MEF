use core::str::FromStr;

use crate::{QlAddress, QlError, QlFace, QlFormRef, QlPosition};

impl FromStr for QlAddress {
    type Err = QlError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value
            .strip_prefix("qladdr:")
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?;
        let mut parts = raw.split('/');
        let frame = parts
            .next()
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?;
        let face = parts
            .next()
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?;
        let position = parts
            .next()
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?;
        let depth = parts
            .next()
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?;
        if parts.next().is_some() {
            return Err(QlError::InvalidAddress(value.to_owned()));
        }

        let frame = format!("qlform:{frame}").parse::<QlFormRef>()?;
        let face = match face {
            "direct" => QlFace::Direct,
            "conjugate" => QlFace::Conjugate,
            _ => return Err(QlError::InvalidAddress(value.to_owned())),
        };
        let position = position
            .strip_prefix('P')
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?
            .parse::<u8>()
            .map_err(|_| QlError::InvalidAddress(value.to_owned()))?;
        let depth = depth
            .strip_prefix('d')
            .ok_or_else(|| QlError::InvalidAddress(value.to_owned()))?
            .parse::<u32>()
            .map_err(|_| QlError::InvalidAddress(value.to_owned()))?;

        Self::new(frame, QlPosition::new(position)?, face, depth)
    }
}
