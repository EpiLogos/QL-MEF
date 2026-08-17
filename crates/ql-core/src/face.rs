use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QlFace {
    Direct,
    Conjugate,
}

impl QlFace {
    pub const fn conjugate(self) -> Self {
        match self {
            Self::Direct => Self::Conjugate,
            Self::Conjugate => Self::Direct,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Conjugate => "conjugate",
        }
    }
}

impl fmt::Display for QlFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
