//! Faithful L5 offices of the one QL Kernel, projected onto existing MEF identities.
//!
//! These are architectural readings, not six new packages or runtime readiness states.
//! The source-owned Para Vak labels remain in the MEF registry. See
//! `docs/QL-VAK-KERNEL-RECONCILIATION.md` for authority, scope and integration gates.

use crate::{ContextFrameId, LensId, SublensRef, lens_definition};

pub const L5_PROJECTION_VERSION: &str = "ql.kernel.l5-projection/v1";
pub const QL_KERNEL_NAME: &str = "QL Kernel";

/// L5 is the concrescence of this existing articulation square, not a kernel wrapper.
/// Order follows the canonical L0 / L0' / L5' / L5 articulation.
pub const ARTICULATION_SQUARE: [LensId; 4] =
    [LensId::L0, LensId::L0Prime, LensId::L5Prime, LensId::L5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum L5Office {
    Syntax,
    Root,
    Harmonics,
    Geometry,
    MetaEpistemicFramework,
    Techne,
}

/// The architectural relation to Context Frames, not evidence of live execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextFrameOfficeRole {
    /// Syntax articulates the frame language; it does not select a runtime frame.
    Articulates,
    /// The positioned/conjugate seed grounds the possibility of frame composition.
    Grounds,
    /// Harmonic/contextual determination already makes Context Frames active.
    Activates,
    /// Later determinations must retain the same active frame and its provenance.
    Propagates,
}

impl L5Office {
    pub const ALL: [Self; 6] = [
        Self::Syntax,
        Self::Root,
        Self::Harmonics,
        Self::Geometry,
        Self::MetaEpistemicFramework,
        Self::Techne,
    ];

    /// Local L5 position. This is deliberately not an absolute QL position.
    pub const fn local_position(self) -> u8 {
        match self {
            Self::Syntax => 0,
            Self::Root => 1,
            Self::Harmonics => 2,
            Self::Geometry => 3,
            Self::MetaEpistemicFramework => 4,
            Self::Techne => 5,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Syntax => "Syntax",
            Self::Root => "Root",
            Self::Harmonics => "Harmonics",
            Self::Geometry => "Geometry",
            Self::MetaEpistemicFramework => "Meta Epistemic Framework",
            Self::Techne => "Techne",
        }
    }

    /// An existing MEF sublens identity; no new address namespace is minted.
    pub fn reference(self) -> SublensRef {
        SublensRef::canonical(LensId::L5, self.local_position())
            .expect("the six L5 offices have valid local positions 0..5")
    }

    /// Keep the source label distinct from the office name in this projection.
    pub fn source_label(self) -> &'static str {
        lens_definition(LensId::L5).sublens_labels()[self.local_position() as usize]
    }

    pub const fn context_frame_role(self) -> ContextFrameOfficeRole {
        match self {
            Self::Syntax => ContextFrameOfficeRole::Articulates,
            Self::Root => ContextFrameOfficeRole::Grounds,
            Self::Harmonics => ContextFrameOfficeRole::Activates,
            Self::Geometry | Self::MetaEpistemicFramework | Self::Techne => {
                ContextFrameOfficeRole::Propagates
            }
        }
    }
}

/// Exact lookup in the existing seven-frame canon, not another expression parser.
///
/// Historical Ta-Onta also admitted `(4/5/0)`. It has no canonical frame identity
/// here and is intentionally unresolved, not silently aliased to CF5 or CF6.
/// Success identifies a frame; it does not confer execution or runtime authority.
pub fn lookup_context_frame_expression(expression: &str) -> Option<ContextFrameId> {
    ContextFrameId::ALL
        .into_iter()
        .find(|frame| frame.expression() == expression)
}
