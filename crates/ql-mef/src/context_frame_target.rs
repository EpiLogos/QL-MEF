use core::fmt;

use ql_core::QlPosition;

use crate::{
    canonical_context_frame_progression, ContextFrameId, MefGrain, MefUnitFace,
    CONTEXT_FRAME_GRAMMAR_VERSION,
};

pub const CONTEXT_FRAME_TARGET_READING_VERSION: &str = "ql.mef.context-frame-reading/1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalContextFrameError {
    EmptyTargetRef,
    EmptyMappingSourceRef,
    EmptyProviderRef,
    EmptyExternalPositionRef(u8),
    DuplicateExternalPositionRef(String),
    EmptyEvidenceRef,
    InvalidPosition(u8),
}

impl fmt::Display for ExternalContextFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTargetRef => f.write_str("EMPTY_TARGET_REF"),
            Self::EmptyMappingSourceRef => f.write_str("EMPTY_MAPPING_SOURCE_REF"),
            Self::EmptyProviderRef => f.write_str("EMPTY_PROVIDER_REF"),
            Self::EmptyExternalPositionRef(position) => {
                write!(f, "EMPTY_EXTERNAL_POSITION_REF:{position}")
            }
            Self::DuplicateExternalPositionRef(reference) => {
                write!(f, "DUPLICATE_EXTERNAL_POSITION_REF:{reference}")
            }
            Self::EmptyEvidenceRef => f.write_str("EMPTY_EVIDENCE_REF"),
            Self::InvalidPosition(position) => write!(f, "INVALID_POSITION:{position}"),
        }
    }
}

impl std::error::Error for ExternalContextFrameError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalSixfoldMapping {
    target_ref: String,
    mapping_source_ref: String,
    external_position_refs: [String; 6],
    mapping_digest: String,
}

impl ExternalSixfoldMapping {
    pub fn new(
        target_ref: impl Into<String>,
        mapping_source_ref: impl Into<String>,
        external_position_refs: [String; 6],
    ) -> Result<Self, ExternalContextFrameError> {
        let target_ref = target_ref.into();
        let mapping_source_ref = mapping_source_ref.into();
        if target_ref.trim().is_empty() {
            return Err(ExternalContextFrameError::EmptyTargetRef);
        }
        if mapping_source_ref.trim().is_empty() {
            return Err(ExternalContextFrameError::EmptyMappingSourceRef);
        }
        for (index, reference) in external_position_refs.iter().enumerate() {
            if reference.trim().is_empty() {
                return Err(ExternalContextFrameError::EmptyExternalPositionRef(index as u8));
            }
            if external_position_refs[..index].contains(reference) {
                return Err(ExternalContextFrameError::DuplicateExternalPositionRef(
                    reference.clone(),
                ));
            }
        }
        let mapping_digest = digest_mapping(&target_ref, &mapping_source_ref, &external_position_refs);
        Ok(Self {
            target_ref,
            mapping_source_ref,
            external_position_refs,
            mapping_digest,
        })
    }

    pub fn target_ref(&self) -> &str {
        &self.target_ref
    }

    pub fn mapping_source_ref(&self) -> &str {
        &self.mapping_source_ref
    }

    pub fn mapping_digest(&self) -> &str {
        &self.mapping_digest
    }

    pub fn external_ref(&self, position: QlPosition) -> &str {
        &self.external_position_refs[position.value() as usize]
    }

    pub fn external_position_refs(&self) -> &[String; 6] {
        &self.external_position_refs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextFrameReadingOrigin {
    Derived,
    Proposed,
    Recognised,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextFrameStructuralProbe {
    local_position: QlPosition,
    unit_face: Option<MefUnitFace>,
    grain: Option<MefGrain>,
}

impl ContextFrameStructuralProbe {
    pub fn new(
        local_position: u8,
        unit_face: Option<MefUnitFace>,
        grain: Option<MefGrain>,
    ) -> Result<Self, ExternalContextFrameError> {
        let local_position = QlPosition::new(local_position)
            .map_err(|_| ExternalContextFrameError::InvalidPosition(local_position))?;
        Ok(Self {
            local_position,
            unit_face,
            grain,
        })
    }

    pub const fn local_position(self) -> QlPosition {
        self.local_position
    }

    pub const fn unit_face(self) -> Option<MefUnitFace> {
        self.unit_face
    }

    pub const fn grain(self) -> Option<MefGrain> {
        self.grain
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextFrameReadingStatus {
    Exact(ContextFrameId),
    Partial {
        frame: ContextFrameId,
        missing_face: bool,
        missing_grain: bool,
    },
    Ambiguous(Vec<ContextFrameId>),
    NoReading,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalContextFrameReading {
    target_ref: String,
    mapping_source_ref: String,
    mapping_digest: String,
    provider_ref: String,
    operator_version: String,
    origin: ContextFrameReadingOrigin,
    probe: ContextFrameStructuralProbe,
    status: ContextFrameReadingStatus,
    evidence_refs: Vec<String>,
}

impl ExternalContextFrameReading {
    pub fn target_ref(&self) -> &str {
        &self.target_ref
    }

    pub fn mapping_source_ref(&self) -> &str {
        &self.mapping_source_ref
    }

    pub fn mapping_digest(&self) -> &str {
        &self.mapping_digest
    }

    pub fn provider_ref(&self) -> &str {
        &self.provider_ref
    }

    pub fn operator_version(&self) -> &str {
        &self.operator_version
    }

    pub const fn origin(&self) -> ContextFrameReadingOrigin {
        self.origin
    }

    pub const fn probe(&self) -> ContextFrameStructuralProbe {
        self.probe
    }

    pub fn status(&self) -> &ContextFrameReadingStatus {
        &self.status
    }

    pub fn evidence_refs(&self) -> &[String] {
        &self.evidence_refs
    }

    /// A CF reading is descriptive evidence only. It never confers runtime,
    /// activation, authority, ownership, or lifecycle state on the target.
    pub const fn is_runtime_authority(&self) -> bool {
        false
    }
}

pub fn read_external_context_frame(
    mapping: &ExternalSixfoldMapping,
    probe: ContextFrameStructuralProbe,
    provider_ref: impl Into<String>,
    origin: ContextFrameReadingOrigin,
    evidence_refs: Vec<String>,
) -> Result<ExternalContextFrameReading, ExternalContextFrameError> {
    let provider_ref = provider_ref.into();
    if provider_ref.trim().is_empty() {
        return Err(ExternalContextFrameError::EmptyProviderRef);
    }
    if evidence_refs.iter().any(|reference| reference.trim().is_empty()) {
        return Err(ExternalContextFrameError::EmptyEvidenceRef);
    }

    let mut candidates = Vec::new();
    for selection in canonical_context_frame_progression() {
        if selection.local_position() != probe.local_position {
            continue;
        }
        if probe
            .unit_face
            .is_some_and(|face| face != selection.unit_face())
        {
            continue;
        }
        if probe.grain.is_some_and(|grain| grain != selection.grain()) {
            continue;
        }
        candidates.push(selection.frame());
    }

    let status = match candidates.as_slice() {
        [] => ContextFrameReadingStatus::NoReading,
        [frame] if probe.unit_face.is_some() && probe.grain.is_some() => {
            ContextFrameReadingStatus::Exact(*frame)
        }
        [frame] => ContextFrameReadingStatus::Partial {
            frame: *frame,
            missing_face: probe.unit_face.is_none(),
            missing_grain: probe.grain.is_none(),
        },
        _ => ContextFrameReadingStatus::Ambiguous(candidates),
    };

    Ok(ExternalContextFrameReading {
        target_ref: mapping.target_ref.clone(),
        mapping_source_ref: mapping.mapping_source_ref.clone(),
        mapping_digest: mapping.mapping_digest.clone(),
        provider_ref,
        operator_version: format!(
            "{CONTEXT_FRAME_TARGET_READING_VERSION}+cf-grammar/{CONTEXT_FRAME_GRAMMAR_VERSION}"
        ),
        origin,
        probe,
        status,
        evidence_refs,
    })
}

fn digest_mapping(target: &str, source: &str, refs: &[String; 6]) -> String {
    // Stable FNV-1a identity digest. This is provenance identity, not a security
    // checksum; callers can retain their own source/content digest as a ref too.
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in core::iter::once(target)
        .chain(core::iter::once(source))
        .chain(refs.iter().map(String::as_str))
    {
        for byte in part.as_bytes().iter().copied().chain(core::iter::once(0xff)) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("fnv1a64:{hash:016x}")
}
