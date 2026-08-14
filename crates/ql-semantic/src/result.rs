use ql_core::QlAddress;
use ql_mef::{QlProvenance, QlReading, QlRelationReading, QlSynthesis, QlTarget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocateStatus {
    Unique,
    Ambiguous,
    InsufficientInformation,
    UnsupportedMapping,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocateResult {
    pub target: QlTarget,
    pub candidates: Vec<QlAddress>,
    pub status: LocateStatus,
    pub provenance: QlProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticStatus {
    Complete,
    Partial,
    Ambiguous,
    InsufficientEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDisclosure {
    pub text: String,
    pub status: SemanticStatus,
    pub confidence_per_mille: Option<u16>,
}

pub type SemanticReading = QlReading<SemanticDisclosure>;
pub type SemanticRelationReading = QlRelationReading<SemanticDisclosure>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisDisclosure {
    pub common_structure: Vec<String>,
    pub complementary_disclosures: Vec<String>,
    pub tensions: Vec<String>,
    pub possible_next_inquiry: Option<String>,
}

pub type SemanticSynthesis = QlSynthesis<SynthesisDisclosure>;
