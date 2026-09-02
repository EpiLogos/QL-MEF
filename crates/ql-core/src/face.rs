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

    /// Stable holographic-kernel face identity shared with the native C core.
    /// Rust keeps its established `Conjugate` spelling; the common contract
    /// names that same face `prime`.
    pub const fn kernel_code(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Conjugate => "prime",
        }
    }

    pub const fn kernel_value(self) -> u8 {
        match self {
            Self::Direct => 0,
            Self::Conjugate => 1,
        }
    }
}

impl fmt::Display for QlFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
