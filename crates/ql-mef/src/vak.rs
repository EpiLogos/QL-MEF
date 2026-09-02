use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use ql_core::{QlShapeAddress, RELATIONAL_SIXFOLD_OPERATOR_REF, RelationalSixfold, SixBySixField};

pub const VAK_SOURCE_REPOSITORY: &str = "EpiLogos/Epi-Logos-C-Experiments";
pub const VAK_SOURCE_REVISION: &str = "daa660cbc1b8c5da83828698665a753852cb0287";
pub const VAK_SOURCE_PATH: &str = "Idea/Bimba/Map/datasets/anuttara-deep/anuttara-language-map.md";
pub const VAK_SOURCE_GIT_BLOB: &str = "22835042d4d2c4ba821c252bd4fbfe52f39712ef";
pub const VAK_ENTRY_COUNT: usize = 109;

const AUTHORITATIVE_LANGUAGE: &str =
    include_str!("../../../data/epi-bimba-map/anuttara-language-map.md");
const SOURCE_PROVENANCE: &str =
    include_str!("../../../data/epi-bimba-map/anuttara-language-map.provenance.json");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VakError {
    InvalidRef(String),
    DuplicateRef(String),
    UnknownRef(String),
    SourceEntryCount { expected: usize, actual: usize },
}

impl fmt::Display for VakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRef(value) => write!(f, "invalid Vāk ref `{value}`"),
            Self::DuplicateRef(value) => write!(f, "duplicate Vāk ref `{value}`"),
            Self::UnknownRef(value) => write!(f, "unknown Vāk ref `{value}`"),
            Self::SourceEntryCount { expected, actual } => {
                write!(f, "expected {expected} Vāk source entries, found {actual}")
            }
        }
    }
}

impl std::error::Error for VakError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VakRef(String);

impl VakRef {
    pub fn new(value: impl Into<String>) -> Result<Self, VakError> {
        let value = value.into();
        if value == "M0" || value == "M0'" || value.starts_with("M0-") || value.starts_with("M0(") {
            Ok(Self(value))
        } else {
            Err(VakError::InvalidRef(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VakRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakStanding {
    Source,
    AuthoredArchitecture,
    Implementation,
    Observed,
    Derived,
    Proposed,
}

impl VakStanding {
    pub const fn as_schema_str(self) -> &'static str {
        match self {
            Self::Source => "SOURCE",
            Self::AuthoredArchitecture => "AUTHORED-ARCHITECTURE",
            Self::Implementation => "IMPLEMENTATION",
            Self::Observed => "OBSERVED",
            Self::Derived => "DERIVED",
            Self::Proposed => "PROPOSED",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakSourceProvenance {
    pub repository: &'static str,
    pub revision: &'static str,
    pub path: &'static str,
    pub git_blob: &'static str,
    pub coordinate: VakRef,
    pub source_line: usize,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakEntry {
    pub vak_ref: VakRef,
    pub source: VakSourceProvenance,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub primary_designation: Option<String>,
    pub complete_formulation: Option<String>,
    pub formulation_breakdown: Option<String>,
    pub metaphysical_names: Vec<String>,
    pub description: Option<String>,
    pub r_factors: Vec<String>,
    /// The exact source row is retained because the Markdown is semantic/formal
    /// authority and some historical values contain punctuation that must never
    /// be repaired or normalized by the runtime parser.
    pub raw_source_row: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakRelationKind {
    Parent,
    Child,
    SourceMentions,
    Contextualises,
    ContextualisedBy,
    PrincipleNineAppearance,
    RPathStep,
    Expresses,
    InvokesThrough,
    TransformsThrough,
    ReadsThrough,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakRelation {
    pub from_ref: VakRef,
    pub relation: VakRelationKind,
    pub into_ref: VakRef,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakNeighbourhood {
    pub centre: VakRef,
    pub depth: usize,
    pub entries: Vec<VakRef>,
    pub relations: Vec<VakRelation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakContextField {
    PrimordialMatrix,
    Bimba,
    Pratibimba,
    Language,
    World,
    Particular,
    Techne,
}

impl VakContextField {
    pub const ALL: [Self; 7] = [
        Self::PrimordialMatrix,
        Self::Bimba,
        Self::Pratibimba,
        Self::Language,
        Self::World,
        Self::Particular,
        Self::Techne,
    ];

    pub const fn symbol(self) -> &'static str {
        match self {
            Self::PrimordialMatrix => "##",
            Self::Bimba => "O#",
            Self::Pratibimba => "X#",
            Self::Language => "N#",
            Self::World => "M#",
            Self::Particular => "#",
            Self::Techne => "R#",
        }
    }

    pub const fn source_coordinate(self) -> &'static str {
        match self {
            Self::PrimordialMatrix => "M0-(4.5/0)-0",
            Self::Bimba => "M0-(4.0/1)",
            Self::Pratibimba => "M0-(4.0/1/2)",
            Self::Language => "M0-(4.0/1/2/3)",
            Self::World => "M0-4.4.0-(4.4/5)",
            Self::Particular => "M0-(4.5/0)",
            Self::Techne => "M0-3-10-(0/1)",
        }
    }

    pub const fn address_horizon(self) -> Option<VakAddressHorizon> {
        match self {
            Self::PrimordialMatrix => Some(VakAddressHorizon::H0),
            Self::Bimba => Some(VakAddressHorizon::H1),
            Self::Pratibimba => Some(VakAddressHorizon::H2),
            Self::Language => Some(VakAddressHorizon::H3),
            Self::World => Some(VakAddressHorizon::H4),
            Self::Particular => None,
            Self::Techne => Some(VakAddressHorizon::H5),
        }
    }

    pub fn source_ref(self) -> VakRef {
        VakRef(self.source_coordinate().to_owned())
    }

    pub fn from_ref(reference: &VakRef) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|field| field.source_coordinate() == reference.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakDivineAct {
    Freedom,
    Creation,
    Sustenance,
    Dissolution,
    Veiling,
    Grace,
    Absorption,
}

impl VakDivineAct {
    pub const ALL: [Self; 7] = [
        Self::Freedom,
        Self::Creation,
        Self::Sustenance,
        Self::Dissolution,
        Self::Veiling,
        Self::Grace,
        Self::Absorption,
    ];

    pub const fn r_factor(self) -> &'static str {
        match self {
            Self::Freedom => "R#",
            Self::Creation => "R0",
            Self::Sustenance => "R1",
            Self::Dissolution => "R2",
            Self::Veiling => "R3",
            Self::Grace => "R4",
            Self::Absorption => "R5",
        }
    }

    pub const fn source_coordinate(self) -> &'static str {
        match self {
            Self::Freedom => "M0-3-10-(0/1)",
            Self::Creation => "M0-3-10-2",
            Self::Sustenance => "M0-3-10-3",
            Self::Dissolution => "M0-3-10-4",
            Self::Veiling => "M0-3-10-5",
            Self::Grace => "M0-3-10-6",
            Self::Absorption => "M0-3-10-7",
        }
    }

    pub const fn principle_nine_coordinate(self) -> Option<&'static str> {
        match self {
            Self::Freedom => None,
            Self::Creation => Some("M0-2-9-3"),
            Self::Sustenance => Some("M0-2-9-4"),
            Self::Dissolution => Some("M0-2-9-5"),
            Self::Veiling => Some("M0-2-9-6"),
            Self::Grace => Some("M0-2-9-7"),
            Self::Absorption => Some("M0-2-9-8"),
        }
    }

    pub const fn principle_nine_formula(self) -> Option<&'static str> {
        match self {
            Self::Freedom => None,
            Self::Creation => Some("0R = @ = (9-O#-X#-N#)"),
            Self::Sustenance => Some("1R = @ = (O#-X#-N#-M#-#-(#))"),
            Self::Dissolution => Some("2R = @ = (X#-N#-M#-#-(#)-(@#))"),
            Self::Veiling => Some("3R = @ = ((@#)-(#)-#-M#-N#-X#)"),
            Self::Grace => Some("4R = @ = ((#)-#-M#-N#-X#-O#)"),
            Self::Absorption => Some("5R = @ = (##)"),
        }
    }

    pub const fn path_tokens(self) -> &'static [&'static str] {
        match self {
            Self::Freedom => &["R#"],
            Self::Creation => &["9", "O#", "X#", "N#"],
            Self::Sustenance => &["O#", "X#", "N#", "M#", "#", "(#)"],
            Self::Dissolution => &["X#", "N#", "M#", "#", "(#)", "(@#)"],
            Self::Veiling => &["(@#)", "(#)", "#", "M#", "N#", "X#"],
            Self::Grace => &["(#)", "#", "M#", "N#", "X#", "O#"],
            Self::Absorption => &["##"],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakRPathStep {
    pub token: String,
    pub vak_ref: VakRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakRPath {
    pub act: VakDivineAct,
    pub act_ref: VakRef,
    pub principle_nine_ref: Option<VakRef>,
    pub principle_nine_formula: Option<String>,
    pub steps: Vec<VakRPathStep>,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VakFormalCoverage {
    pub names: usize,
    pub symbols: usize,
    pub primary_designations: usize,
    pub complete_formulations: usize,
    pub formulation_breakdowns: usize,
    pub metaphysical_names: usize,
    pub descriptions: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakRelationOp {
    Potential,
    Distinguish,
    Affirm,
    Relate,
    Contextualise,
    Express,
}

impl VakRelationOp {
    pub const ALL: [Self; 6] = [
        Self::Potential,
        Self::Distinguish,
        Self::Affirm,
        Self::Relate,
        Self::Contextualise,
        Self::Express,
    ];

    pub const fn position(self) -> u8 {
        match self {
            Self::Potential => 0,
            Self::Distinguish => 1,
            Self::Affirm => 2,
            Self::Relate => 3,
            Self::Contextualise => 4,
            Self::Express => 5,
        }
    }

    pub const fn glyph(self) -> &'static str {
        match self {
            Self::Potential => "@#",
            Self::Distinguish => "-",
            Self::Affirm => "+",
            Self::Relate => "x",
            Self::Contextualise => "/",
            Self::Express => "=",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Potential => "Potential / In-pression",
            Self::Distinguish => "Distinguish",
            Self::Affirm => "Affirm",
            Self::Relate => "Relate",
            Self::Contextualise => "Contextualise",
            Self::Express => "Express",
        }
    }

    pub const fn source_coordinate(self) -> &'static str {
        match self {
            Self::Potential => "M0-5-(0/1)-0",
            Self::Distinguish => "M0-5-(0/1)-1",
            Self::Affirm => "M0-5-(0/1)-2",
            Self::Relate => "M0-5-(0/1)-3",
            Self::Contextualise => "M0-5-(0/1)-4",
            Self::Express => "M0-5-(0/1)-5",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakAddressHorizon {
    H0,
    H1,
    H2,
    H3,
    H4,
    H5,
}

impl VakAddressHorizon {
    pub const ALL: [Self; 6] = [Self::H0, Self::H1, Self::H2, Self::H3, Self::H4, Self::H5];

    pub const fn position(self) -> u8 {
        match self {
            Self::H0 => 0,
            Self::H1 => 1,
            Self::H2 => 2,
            Self::H3 => 3,
            Self::H4 => 4,
            Self::H5 => 5,
        }
    }

    pub const fn address(self) -> &'static str {
        match self {
            Self::H0 => "@0",
            Self::H1 => "@1",
            Self::H2 => "@2",
            Self::H3 => "@3",
            Self::H4 => "@4",
            Self::H5 => "@5",
        }
    }

    pub const fn source_symbol(self) -> &'static str {
        match self {
            Self::H0 => "##",
            Self::H1 => "O#",
            Self::H2 => "X#",
            Self::H3 => "N#",
            Self::H4 => "M#",
            Self::H5 => "R#",
        }
    }

    pub const fn source_coordinate(self) -> &'static str {
        match self {
            Self::H0 => "M0-5-(5/0)-0",
            Self::H1 => "M0-5-(5/0)-1",
            Self::H2 => "M0-5-(5/0)-2",
            Self::H3 => "M0-5-(5/0)-3",
            Self::H4 => "M0-5-(5/0)-4",
            Self::H5 => "M0-5-(5/0)-5",
        }
    }

    pub const fn general_name(self) -> &'static str {
        match self {
            Self::H0 => "Ground / Knowing / available knowledge",
            Self::H1 => "Original / determining structure",
            Self::H2 => "Reflection / meaning",
            Self::H3 => "Language / form",
            Self::H4 => "World / context / story",
            Self::H5 => "Power / Techne / praxis",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakOperatorBinding {
    pub operator: VakRelationOp,
    pub standing: VakStanding,
    pub source_support: Vec<VakRef>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakHorizonBinding {
    pub horizon: VakAddressHorizon,
    pub standing: VakStanding,
    pub source_support: Vec<VakRef>,
    pub evidence: Vec<String>,
}

/// One source-provenanced address in the six Śiva operations × six Śakti horizons field.
///
/// The inherited QL `6×6` coordinates are used as the canonical accounting carrier. This is an
/// implementation mapping of two Vāk sixfolds onto that shape; it does not claim that the source
/// text itself names Śiva as the kernel Direct face or Śakti as the Conjugate face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakSivaSaktiCell {
    pub operator: VakRelationOp,
    pub horizon: VakAddressHorizon,
    pub ql_address: QlShapeAddress,
    pub operator_source_ref: VakRef,
    pub horizon_source_ref: VakRef,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakSivaSaktiField {
    pub ql_shape_ref: String,
    pub cells: Vec<VakSivaSaktiCell>,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakSivaSaktiGenerationSite {
    pub position: u8,
    pub operator: VakRelationOp,
    pub horizon: VakAddressHorizon,
    pub operator_source_ref: VakRef,
    pub horizon_source_ref: VakRef,
    pub ql_operator_ref: String,
    pub standing: VakStanding,
}

/// Source-grounded Vāk reading of the canonical kernel `6 / 6′ -> 6+6′` operation.
///
/// The kernel supplies six same-position generation sites and Return. Vāk supplies the exact
/// Śiva/Śakti source identities at those positions. Semantic generated content remains an
/// attributable Agent/client return and is deliberately absent from this deterministic reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakSivaSaktiRelationalSixfold {
    pub ql_shape_ref: String,
    pub ql_operator_ref: String,
    pub contextualise_source_ref: VakRef,
    pub return_anchor_symbol: String,
    pub sites: Vec<VakSivaSaktiGenerationSite>,
    pub semantic_generation_requires_attributable_return: bool,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelfOtherForm {
    ActualIdentity,
    PotentialEssence,
    SubjectiveI,
    AssertedAm,
    Statement,
    ObjectiveIs,
    QueryOfOther,
    ReflexiveQuery,
    IntegratedSelf,
    RelationalOther,
    SelfQuestioningWorld,
    WorldQuestioningSelf,
}

impl SelfOtherForm {
    pub const ALL: [Self; 12] = [
        Self::ActualIdentity,
        Self::PotentialEssence,
        Self::SubjectiveI,
        Self::AssertedAm,
        Self::Statement,
        Self::ObjectiveIs,
        Self::QueryOfOther,
        Self::ReflexiveQuery,
        Self::IntegratedSelf,
        Self::RelationalOther,
        Self::SelfQuestioningWorld,
        Self::WorldQuestioningSelf,
    ];

    pub const fn glyph(self) -> &'static str {
        match self {
            Self::ActualIdentity => "!",
            Self::PotentialEssence => "?",
            Self::SubjectiveI => "!-",
            Self::AssertedAm => "-?",
            Self::Statement => "!?",
            Self::ObjectiveIs => "?-",
            Self::QueryOfOther => "-!",
            Self::ReflexiveQuery => "?!",
            Self::IntegratedSelf => "-!/!-",
            Self::RelationalOther => "-?/?-",
            Self::SelfQuestioningWorld => "!?/?!",
            Self::WorldQuestioningSelf => "?!/!?",
        }
    }

    pub const fn source_coordinate(self) -> &'static str {
        match self {
            Self::ActualIdentity => "M0-3-6-0",
            Self::PotentialEssence => "M0-3-6-1",
            Self::SubjectiveI => "M0-3-6-2",
            Self::AssertedAm => "M0-3-6-3",
            Self::Statement => "M0-3-6-4",
            Self::ObjectiveIs => "M0-3-6-5",
            Self::QueryOfOther => "M0-3-6-6",
            Self::ReflexiveQuery => "M0-3-6-7",
            Self::IntegratedSelf => "M0-3-6-8",
            Self::RelationalOther => "M0-3-6-9",
            Self::SelfQuestioningWorld => "M0-3-6-10",
            Self::WorldQuestioningSelf => "M0-3-6-11",
        }
    }

    pub fn source_ref(self) -> VakRef {
        VakRef(self.source_coordinate().to_owned())
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|form| form.glyph() == value)
    }
}

impl fmt::Display for SelfOtherForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.glyph())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakSpeechStance {
    ActualAssertion,
    PotentialQuestion,
    SubjectiveAssertion,
    AssertedBeing,
    Statement,
    ObjectiveBeing,
    QueryOfOther,
    ReflexiveQuery,
    IntegratedSelfReport,
    RelationalOtherReport,
    SelfQuestioningWorld,
    WorldQuestioningSelf,
}

impl SelfOtherForm {
    pub const fn stance(self) -> VakSpeechStance {
        match self {
            Self::ActualIdentity => VakSpeechStance::ActualAssertion,
            Self::PotentialEssence => VakSpeechStance::PotentialQuestion,
            Self::SubjectiveI => VakSpeechStance::SubjectiveAssertion,
            Self::AssertedAm => VakSpeechStance::AssertedBeing,
            Self::Statement => VakSpeechStance::Statement,
            Self::ObjectiveIs => VakSpeechStance::ObjectiveBeing,
            Self::QueryOfOther => VakSpeechStance::QueryOfOther,
            Self::ReflexiveQuery => VakSpeechStance::ReflexiveQuery,
            Self::IntegratedSelf => VakSpeechStance::IntegratedSelfReport,
            Self::RelationalOther => VakSpeechStance::RelationalOtherReport,
            Self::SelfQuestioningWorld => VakSpeechStance::SelfQuestioningWorld,
            Self::WorldQuestioningSelf => VakSpeechStance::WorldQuestioningSelf,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakSpeechAct {
    pub form: SelfOtherForm,
    pub stance: VakSpeechStance,
    pub source_ref: VakRef,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakPraxisAspect {
    WillAgency,
    KnowledgeVimarsa,
    ActionSvatantrya,
}

impl VakPraxisAspect {
    pub const fn source_term(self) -> &'static str {
        match self {
            Self::WillAgency => "Agency",
            Self::KnowledgeVimarsa => "Vimar",
            Self::ActionSvatantrya => "Svatantrya",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakPraxisReading {
    pub aspect: VakPraxisAspect,
    pub source_refs: Vec<VakRef>,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakActionRelationKind {
    Expresses,
    InvokesThrough,
    TransformsThrough,
    ReadsThrough,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakActionProfile {
    pub vak_ref: VakRef,
    pub native_action_ref: String,
    pub native_owner: String,
    pub relation_kind: VakActionRelationKind,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
    pub binding_revision: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakRefraction {
    pub native_ref: String,
    pub vak_ref: VakRef,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakPath {
    pub method_ref: Option<String>,
    pub resolve_expression: String,
    pub steps: Vec<VakRef>,
    pub action: Option<VakActionProfile>,
    pub invocation_ref: Option<String>,
    pub activity_ref: Option<String>,
    pub evidence_refs: Vec<String>,
    pub return_ref: Option<String>,
    pub recognition: Option<VakRef>,
    pub standing: VakStanding,
}

#[derive(Debug, Clone)]
pub struct VakRegistry {
    entries: BTreeMap<VakRef, VakEntry>,
}

impl VakRegistry {
    pub fn from_authoritative_source() -> Result<Self, VakError> {
        let mut entries = BTreeMap::new();
        for (line_index, line) in AUTHORITATIVE_LANGUAGE.lines().enumerate() {
            let Some(entry) = parse_source_entry(line, line_index + 1)? else {
                continue;
            };
            let key = entry.vak_ref.clone();
            if entries.insert(key.clone(), entry).is_some() {
                return Err(VakError::DuplicateRef(key.to_string()));
            }
        }
        if entries.len() != VAK_ENTRY_COUNT {
            return Err(VakError::SourceEntryCount {
                expected: VAK_ENTRY_COUNT,
                actual: entries.len(),
            });
        }
        Ok(Self { entries })
    }

    pub fn source_markdown() -> &'static str {
        AUTHORITATIVE_LANGUAGE
    }

    pub fn source_provenance_receipt() -> &'static str {
        SOURCE_PROVENANCE
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> impl Iterator<Item = &VakEntry> {
        self.entries.values()
    }

    pub fn locate(&self, reference: &VakRef) -> Option<&VakEntry> {
        self.entries.get(reference)
    }

    pub fn locate_str(&self, reference: &str) -> Result<&VakEntry, VakError> {
        let reference = VakRef::new(reference)?;
        self.locate(&reference)
            .ok_or_else(|| VakError::UnknownRef(reference.to_string()))
    }

    pub fn locate_symbol(&self, symbol: &str) -> Vec<&VakEntry> {
        self.entries()
            .filter(|entry| entry.symbol.as_deref() == Some(symbol))
            .collect()
    }

    pub fn search_semantic(&self, query: &str) -> Vec<&VakEntry> {
        let query = query.to_lowercase();
        self.entries()
            .filter(|entry| entry.raw_source_row.to_lowercase().contains(&query))
            .collect()
    }

    pub fn parent(&self, reference: &VakRef) -> Option<&VakRef> {
        self.entries
            .keys()
            .filter(|candidate| *candidate != reference)
            .filter(|candidate| {
                reference
                    .as_str()
                    .strip_prefix(candidate.as_str())
                    .is_some_and(|suffix| suffix.starts_with('-'))
            })
            .max_by_key(|candidate| candidate.as_str().len())
    }

    pub fn children(&self, reference: &VakRef) -> Vec<&VakRef> {
        self.entries
            .keys()
            .filter(|candidate| {
                self.parent(candidate)
                    .is_some_and(|parent| parent == reference)
            })
            .collect()
    }

    pub fn relations_from(&self, reference: &VakRef) -> Result<Vec<VakRelation>, VakError> {
        if self.locate(reference).is_none() {
            return Err(VakError::UnknownRef(reference.to_string()));
        }
        let mut relations = Vec::new();
        if let Some(parent) = self.parent(reference) {
            relations.push(structural_relation(
                reference.clone(),
                VakRelationKind::Parent,
                parent.clone(),
            ));
        }
        for child in self.children(reference) {
            relations.push(structural_relation(
                reference.clone(),
                VakRelationKind::Child,
                child.clone(),
            ));
        }
        relations.extend(self.context_relations_from(reference)?);
        relations.extend(self.r_relations_from(reference)?);
        Ok(relations)
    }

    pub fn relate(&self, from: &VakRef, into: &VakRef) -> Result<Vec<VakRelation>, VakError> {
        let from_entry = self
            .locate(from)
            .ok_or_else(|| VakError::UnknownRef(from.to_string()))?;
        self.locate(into)
            .ok_or_else(|| VakError::UnknownRef(into.to_string()))?;

        let mut relations = self
            .relations_from(from)?
            .into_iter()
            .filter(|relation| &relation.into_ref == into)
            .collect::<Vec<_>>();
        if from_entry.raw_source_row.contains(into.as_str()) {
            relations.push(VakRelation {
                from_ref: from.clone(),
                relation: VakRelationKind::SourceMentions,
                into_ref: into.clone(),
                standing: VakStanding::Source,
                evidence: vec![format!(
                    "{}:{} contains exact coordinate {}",
                    VAK_SOURCE_PATH, from_entry.source.source_line, into
                )],
            });
        }
        Ok(relations)
    }

    pub fn neighbourhood(
        &self,
        centre: &VakRef,
        depth: usize,
    ) -> Result<VakNeighbourhood, VakError> {
        if self.locate(centre).is_none() {
            return Err(VakError::UnknownRef(centre.to_string()));
        }
        let mut seen = BTreeSet::from([centre.clone()]);
        let mut queue = VecDeque::from([(centre.clone(), 0_usize)]);
        let mut relations = Vec::new();

        while let Some((current, current_depth)) = queue.pop_front() {
            if current_depth >= depth {
                continue;
            }
            for relation in self.relations_from(&current)? {
                if seen.insert(relation.into_ref.clone()) {
                    queue.push_back((relation.into_ref.clone(), current_depth + 1));
                }
                relations.push(relation);
            }
        }

        Ok(VakNeighbourhood {
            centre: centre.clone(),
            depth,
            entries: seen.into_iter().collect(),
            relations,
        })
    }

    pub fn bind_operator(&self, operator: VakRelationOp) -> Result<VakOperatorBinding, VakError> {
        let source_ref = VakRef::new(operator.source_coordinate())?;
        let entry = self
            .locate(&source_ref)
            .ok_or_else(|| VakError::UnknownRef(source_ref.to_string()))?;
        if !entry.raw_source_row.contains(operator.glyph()) {
            return Err(VakError::InvalidRef(format!(
                "{} does not carry Śiva operator glyph {}",
                source_ref,
                operator.glyph()
            )));
        }
        Ok(VakOperatorBinding {
            operator,
            standing: VakStanding::Implementation,
            source_support: vec![source_ref.clone()],
            evidence: vec![format!(
                "PR #84 maps general O:I relation position {} to exact source-backed Śiva node {}: `{}` / {}",
                operator.position(),
                source_ref,
                operator.glyph(),
                operator.name()
            )],
        })
    }

    pub fn bind_horizon(&self, horizon: VakAddressHorizon) -> Result<VakHorizonBinding, VakError> {
        let source_ref = VakRef::new(horizon.source_coordinate())?;
        let entry = self
            .locate(&source_ref)
            .ok_or_else(|| VakError::UnknownRef(source_ref.to_string()))?;
        let source_relation = format!("{} = {}", horizon.address(), horizon.source_symbol());
        if !entry.raw_source_row.contains(&source_relation) {
            return Err(VakError::InvalidRef(format!(
                "{} does not carry Śakti horizon relation {}",
                source_ref, source_relation
            )));
        }
        Ok(VakHorizonBinding {
            horizon,
            standing: VakStanding::Implementation,
            source_support: vec![source_ref.clone()],
            evidence: vec![format!(
                "PR #84 maps general O:I horizon {} to exact source-backed Śakti node {}: {}",
                horizon.address(),
                source_ref,
                source_relation
            )],
        })
    }

    pub fn self_other_entry(&self, form: SelfOtherForm) -> Result<&VakEntry, VakError> {
        let reference = form.source_ref();
        let entry = self
            .locate(&reference)
            .ok_or_else(|| VakError::UnknownRef(reference.to_string()))?;
        if entry.symbol.as_deref() != Some(form.glyph()) {
            return Err(VakError::InvalidRef(format!(
                "{} does not carry expected source glyph {}",
                reference,
                form.glyph()
            )));
        }
        Ok(entry)
    }

    pub fn formal_coverage(&self) -> VakFormalCoverage {
        VakFormalCoverage {
            names: self.entries().filter(|entry| entry.name.is_some()).count(),
            symbols: self
                .entries()
                .filter(|entry| entry.symbol.is_some())
                .count(),
            primary_designations: self
                .entries()
                .filter(|entry| entry.primary_designation.is_some())
                .count(),
            complete_formulations: self
                .entries()
                .filter(|entry| entry.complete_formulation.is_some())
                .count(),
            formulation_breakdowns: self
                .entries()
                .filter(|entry| entry.formulation_breakdown.is_some())
                .count(),
            metaphysical_names: self
                .entries()
                .filter(|entry| !entry.metaphysical_names.is_empty())
                .count(),
            descriptions: self
                .entries()
                .filter(|entry| entry.description.is_some())
                .count(),
        }
    }

    pub fn context_field_entry(&self, field: VakContextField) -> Result<&VakEntry, VakError> {
        self.locate_str(field.source_coordinate())
    }

    pub fn context_relations_from(&self, reference: &VakRef) -> Result<Vec<VakRelation>, VakError> {
        let Some(field) = VakContextField::from_ref(reference) else {
            return Ok(Vec::new());
        };
        self.context_field_entry(field)?;
        let position = VakContextField::ALL
            .iter()
            .position(|candidate| *candidate == field)
            .expect("VakContextField::ALL contains every variant");
        let mut relations = Vec::new();
        if let Some(previous) = position
            .checked_sub(1)
            .and_then(|index| VakContextField::ALL.get(index))
        {
            self.context_field_entry(*previous)?;
            relations.push(VakRelation {
                from_ref: reference.clone(),
                relation: VakRelationKind::ContextualisedBy,
                into_ref: previous.source_ref(),
                standing: VakStanding::AuthoredArchitecture,
                evidence: vec![
                    "EPI-VAK-OPERATIVE-SYNTAX-ARCHITECTURE §8: M0-4 contextual field".to_owned(),
                ],
            });
        }
        if let Some(next) = VakContextField::ALL.get(position + 1) {
            self.context_field_entry(*next)?;
            relations.push(VakRelation {
                from_ref: reference.clone(),
                relation: VakRelationKind::Contextualises,
                into_ref: next.source_ref(),
                standing: VakStanding::AuthoredArchitecture,
                evidence: vec![
                    "EPI-VAK-OPERATIVE-SYNTAX-ARCHITECTURE §8: ## → O# → X# → N# → M# → # → R#"
                        .to_owned(),
                ],
            });
        }
        Ok(relations)
    }

    fn r_path_token_ref(&self, token: &str) -> Result<VakRef, VakError> {
        let coordinate = match token {
            "9" => "M0-2-9",
            "##" => VakContextField::PrimordialMatrix.source_coordinate(),
            "O#" => VakContextField::Bimba.source_coordinate(),
            "X#" => VakContextField::Pratibimba.source_coordinate(),
            "N#" => VakContextField::Language.source_coordinate(),
            "M#" => VakContextField::World.source_coordinate(),
            "#" => VakContextField::Particular.source_coordinate(),
            "R#" => VakContextField::Techne.source_coordinate(),
            "(#)" => "M0-5-(0/1)",
            "(@#)" => "M0-5-(5/0)",
            other => {
                return Err(VakError::InvalidRef(format!(
                    "unknown R-path token `{other}`"
                )));
            }
        };
        let reference = VakRef::new(coordinate)?;
        self.locate(&reference)
            .ok_or_else(|| VakError::UnknownRef(reference.to_string()))?;
        Ok(reference)
    }

    pub fn r_path(&self, act: VakDivineAct) -> Result<VakRPath, VakError> {
        let act_ref = VakRef::new(act.source_coordinate())?;
        self.locate(&act_ref)
            .ok_or_else(|| VakError::UnknownRef(act_ref.to_string()))?;
        let principle_nine_ref = act
            .principle_nine_coordinate()
            .map(VakRef::new)
            .transpose()?;
        if let Some(reference) = &principle_nine_ref {
            let entry = self
                .locate(reference)
                .ok_or_else(|| VakError::UnknownRef(reference.to_string()))?;
            let formula = act
                .principle_nine_formula()
                .expect("principle-nine ref and formula are paired");
            if !entry.raw_source_row.contains(formula) {
                return Err(VakError::InvalidRef(format!(
                    "{} does not carry exact Principle-9 formula {}",
                    reference, formula
                )));
            }
        }
        let steps = act
            .path_tokens()
            .iter()
            .map(|token| {
                Ok(VakRPathStep {
                    token: (*token).to_owned(),
                    vak_ref: self.r_path_token_ref(token)?,
                })
            })
            .collect::<Result<Vec<_>, VakError>>()?;
        let mut evidence = vec![format!(
            "{} identifies {} / {}",
            act_ref,
            act.r_factor(),
            self.locate(&act_ref)
                .and_then(|entry| entry.primary_designation.as_deref())
                .unwrap_or("source-backed divine act")
        )];
        if let (Some(reference), Some(formula)) =
            (&principle_nine_ref, act.principle_nine_formula())
        {
            evidence.push(format!(
                "{} exact Principle-9 appearance: {}",
                reference, formula
            ));
        }
        Ok(VakRPath {
            act,
            act_ref,
            principle_nine_ref,
            principle_nine_formula: act.principle_nine_formula().map(ToOwned::to_owned),
            steps,
            standing: VakStanding::Source,
            evidence,
        })
    }

    pub fn r_relations_from(&self, reference: &VakRef) -> Result<Vec<VakRelation>, VakError> {
        let mut relations = Vec::new();
        for act in VakDivineAct::ALL {
            let path = self.r_path(act)?;
            if path.principle_nine_ref.as_ref() == Some(reference) {
                relations.push(VakRelation {
                    from_ref: reference.clone(),
                    relation: VakRelationKind::PrincipleNineAppearance,
                    into_ref: path.act_ref.clone(),
                    standing: VakStanding::Source,
                    evidence: path.evidence.clone(),
                });
            }
            if &path.act_ref == reference {
                if let Some(first) = path.steps.first() {
                    relations.push(VakRelation {
                        from_ref: reference.clone(),
                        relation: VakRelationKind::Expresses,
                        into_ref: first.vak_ref.clone(),
                        standing: VakStanding::Source,
                        evidence: path.evidence.clone(),
                    });
                }
            }
            for pair in path.steps.windows(2) {
                if &pair[0].vak_ref == reference {
                    relations.push(VakRelation {
                        from_ref: reference.clone(),
                        relation: VakRelationKind::RPathStep,
                        into_ref: pair[1].vak_ref.clone(),
                        standing: VakStanding::Source,
                        evidence: path.evidence.clone(),
                    });
                }
            }
        }
        Ok(relations)
    }

    pub fn parse_speech_act(&self, glyph: &str) -> Result<VakSpeechAct, VakError> {
        let form = SelfOtherForm::parse(glyph)
            .ok_or_else(|| VakError::InvalidRef(format!("unknown M0-3 speech form `{glyph}`")))?;
        let entry = self.self_other_entry(form)?;
        Ok(VakSpeechAct {
            form,
            stance: form.stance(),
            source_ref: entry.vak_ref.clone(),
            standing: VakStanding::Source,
        })
    }

    pub fn praxis_reading(&self, aspect: VakPraxisAspect) -> VakPraxisReading {
        let coordinates: &[&str] = match aspect {
            VakPraxisAspect::WillAgency => &["M0-3-3", "M0-3-6-2"],
            VakPraxisAspect::KnowledgeVimarsa => &["M0-3-(0/1)", "M0-(4.0/1/2)"],
            VakPraxisAspect::ActionSvatantrya => &["M0-3-10", "M0-3-10-(0/1)", "M0-5-(5/0)-5"],
        };
        VakPraxisReading {
            aspect,
            source_refs: coordinates
                .iter()
                .filter_map(|coordinate| self.locate_str(coordinate).ok())
                .map(|entry| entry.vak_ref.clone())
                .collect(),
            standing: VakStanding::Source,
        }
    }

    /// Compose the exact M0-5 Śiva operation sixfold with the exact M0-5 Śakti horizon
    /// sixfold through the canonical QL 6×6 accounting shape.
    pub fn siva_sakti_operative_field(&self) -> Result<VakSivaSaktiField, VakError> {
        let ql_field = SixBySixField::canonical();
        let mut cells = Vec::with_capacity(36);
        for operator in VakRelationOp::ALL {
            self.bind_operator(operator)?;
            for horizon in VakAddressHorizon::ALL {
                self.bind_horizon(horizon)?;
                let index = usize::from(operator.position()) * 6 + usize::from(horizon.position());
                let ql_address = *ql_field.addresses.get(index).ok_or_else(|| {
                    VakError::InvalidRef(format!(
                        "canonical QL 6x6 address missing at Śiva {} × Śakti {}",
                        operator.position(),
                        horizon.position()
                    ))
                })?;
                cells.push(VakSivaSaktiCell {
                    operator,
                    horizon,
                    ql_address,
                    operator_source_ref: VakRef::new(operator.source_coordinate())?,
                    horizon_source_ref: VakRef::new(horizon.source_coordinate())?,
                    standing: VakStanding::Implementation,
                });
            }
        }
        Ok(VakSivaSaktiField {
            ql_shape_ref: ql_field.shape_ref().into(),
            cells,
            standing: VakStanding::Implementation,
        })
    }

    /// Bind the source sixfolds to the kernel's six same-position relational-generation sites.
    /// `/` remains the exact Vāk contextual/dialectical operator while the kernel supplies the
    /// deterministic site/operator identity and Return law.
    pub fn siva_sakti_relational_sixfold(&self) -> Result<VakSivaSaktiRelationalSixfold, VakError> {
        let ql_shape = RelationalSixfold::canonical();
        let contextualise = self.bind_operator(VakRelationOp::Contextualise)?;
        let contextualise_source_ref =
            contextualise
                .source_support
                .first()
                .cloned()
                .ok_or_else(|| {
                    VakError::UnknownRef(VakRelationOp::Contextualise.source_coordinate().into())
                })?;
        let mut sites = Vec::with_capacity(6);
        for ql_site in &ql_shape.sites {
            let position = ql_site.position.value();
            let operator = VakRelationOp::ALL[usize::from(position)];
            let horizon = VakAddressHorizon::ALL[usize::from(position)];
            self.bind_operator(operator)?;
            self.bind_horizon(horizon)?;
            sites.push(VakSivaSaktiGenerationSite {
                position,
                operator,
                horizon,
                operator_source_ref: VakRef::new(operator.source_coordinate())?,
                horizon_source_ref: VakRef::new(horizon.source_coordinate())?,
                ql_operator_ref: ql_site.operator_ref(),
                standing: VakStanding::Implementation,
            });
        }
        Ok(VakSivaSaktiRelationalSixfold {
            ql_shape_ref: ql_shape.shape_ref().into(),
            ql_operator_ref: RELATIONAL_SIXFOLD_OPERATOR_REF.into(),
            contextualise_source_ref,
            return_anchor_symbol: ql_shape.return_anchor_symbol.into(),
            sites,
            semantic_generation_requires_attributable_return: true,
            standing: VakStanding::Implementation,
        })
    }

    pub fn refract(
        &self,
        native_ref: impl Into<String>,
        vak_ref: VakRef,
        standing: VakStanding,
        evidence: Vec<String>,
    ) -> Result<VakRefraction, VakError> {
        self.locate(&vak_ref)
            .ok_or_else(|| VakError::UnknownRef(vak_ref.to_string()))?;
        Ok(VakRefraction {
            native_ref: native_ref.into(),
            vak_ref,
            standing,
            evidence,
        })
    }
}

fn structural_relation(from: VakRef, relation: VakRelationKind, into: VakRef) -> VakRelation {
    VakRelation {
        evidence: vec![format!(
            "exact source coordinate nesting: {} {} {}",
            from,
            match relation {
                VakRelationKind::Parent => "parent",
                VakRelationKind::Child => "child",
                _ => "related",
            },
            into
        )],
        from_ref: from,
        relation,
        into_ref: into,
        standing: VakStanding::Derived,
    }
}

fn split_markdown_row(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }
    let body = &trimmed[1..trimmed.len() - 1];
    let mut cells = Vec::new();
    let mut current = String::new();
    let mut in_code = false;
    let mut escaped = false;
    for ch in body.chars() {
        if escaped {
            if ch == '|' {
                current.push('|');
            } else {
                current.push(char::from(92));
                current.push(ch);
            }
            escaped = false;
            continue;
        }
        if ch == char::from(92) {
            escaped = true;
            continue;
        }
        if ch == '`' {
            in_code = !in_code;
            current.push(ch);
            continue;
        }
        if ch == '|' && !in_code {
            cells.push(current.trim().to_owned());
            current.clear();
        } else {
            current.push(ch);
        }
    }
    if escaped {
        current.push(char::from(92));
    }
    cells.push(current.trim().to_owned());
    Some(cells)
}

fn parse_source_entry(line: &str, source_line: usize) -> Result<Option<VakEntry>, VakError> {
    let Some(fields) = split_markdown_row(line) else {
        return Ok(None);
    };
    if fields.len() != 8 {
        return Ok(None);
    }
    let coordinate = fields[0]
        .strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
        .unwrap_or(&fields[0]);
    if !coordinate.starts_with("M0") {
        return Ok(None);
    }
    let vak_ref = VakRef::new(coordinate)?;
    let name = clean_cell(&fields[1]);
    let symbol = clean_cell(&fields[2]);
    let primary_designation = clean_cell(&fields[3]);
    let complete_formulation = clean_cell(&fields[4]);
    let formulation_breakdown = clean_cell(&fields[5]);
    let metaphysical_names = clean_cell(&fields[6])
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let description = clean_cell(&fields[7]);

    Ok(Some(VakEntry {
        source: VakSourceProvenance {
            repository: VAK_SOURCE_REPOSITORY,
            revision: VAK_SOURCE_REVISION,
            path: VAK_SOURCE_PATH,
            git_blob: VAK_SOURCE_GIT_BLOB,
            coordinate: vak_ref.clone(),
            source_line,
            standing: VakStanding::Source,
        },
        vak_ref,
        name,
        symbol,
        primary_designation,
        complete_formulation,
        formulation_breakdown,
        metaphysical_names,
        description,
        r_factors: extract_r_factors(line),
        raw_source_row: line.to_owned(),
    }))
}

fn clean_cell(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    let value = value
        .strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
        .unwrap_or(value);
    Some(value.to_owned())
}

fn extract_r_factors(value: &str) -> Vec<String> {
    let mut factors = BTreeSet::new();
    for token in value.split(|ch: char| ch.is_whitespace() || matches!(ch, ',' | ';' | '—')) {
        let token =
            token.trim_matches(|ch: char| matches!(ch, '`' | '.' | ':' | '(' | ')' | '[' | ']'));
        let bytes = token.as_bytes();
        if bytes.len() >= 2 && bytes[0] == b'R' && bytes[1].is_ascii_digit() {
            factors.insert(token.to_owned());
        }
    }
    factors.into_iter().collect()
}
