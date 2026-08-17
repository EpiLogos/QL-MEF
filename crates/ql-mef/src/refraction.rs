use crate::{ClientRef, LensRef, MefError, QlTarget, SublensRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefractionContract {
    pub target: QlTarget,
    pub lens: LensRef,
    pub sublens: Option<SublensRef>,
}

impl RefractionContract {
    pub fn new(
        target: QlTarget,
        lens: LensRef,
        sublens: Option<SublensRef>,
    ) -> Result<Self, MefError> {
        if let Some(sublens) = sublens {
            if sublens.lens() != lens {
                return Err(MefError::SublensLensMismatch);
            }
        }
        Ok(Self {
            target,
            lens,
            sublens,
        })
    }

    pub fn subject_ref(&self) -> &ClientRef {
        &self.target.subject
    }
}
