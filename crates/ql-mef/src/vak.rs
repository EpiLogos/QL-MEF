use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

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
    SourceBacked,
    CurrentAuthoredPosition,
    DesignCommitment,
    ImplementationMapping,
    ResearchProposition,
    Inference,
}

impl VakStanding {
    pub const fn as_schema_str(self) -> &'static str {
        match self {
            Self::SourceBacked => "SOURCE-BACKED",
            Self::CurrentAuthoredPosition => "CURRENT-AUTHORED-POSITION",
            Self::DesignCommitment => "DESIGN-COMMITMENT",
            Self::ImplementationMapping => "IMPLEMENTATION-MAPPING",
            Self::ResearchProposition => "RESEARCH-PROPOSITION",
            Self::Inference => "INFERENCE",
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
                standing: VakStanding::SourceBacked,
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
            standing: VakStanding::ImplementationMapping,
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
            standing: VakStanding::ImplementationMapping,
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

    pub fn praxis_reading(&self, aspect: VakPraxisAspect) -> VakPraxisReading {
        VakPraxisReading {
            aspect,
            source_refs: self
                .search_semantic(aspect.source_term())
                .into_iter()
                .map(|entry| entry.vak_ref.clone())
                .collect(),
            standing: VakStanding::SourceBacked,
        }
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
        standing: VakStanding::SourceBacked,
    }
}

fn parse_source_entry(line: &str, source_line: usize) -> Result<Option<VakEntry>, VakError> {
    let trimmed = line.trim();
    let Some(after_prefix) = trimmed.strip_prefix("| `") else {
        return Ok(None);
    };
    let Some(coordinate_end) = after_prefix.find('`') else {
        return Ok(None);
    };
    let coordinate = &after_prefix[..coordinate_end];
    if !coordinate.starts_with("M0") {
        return Ok(None);
    }
    let vak_ref = VakRef::new(coordinate)?;
    let rest = &after_prefix[coordinate_end + 1..];
    let Some(rest) = rest.strip_prefix(" | ") else {
        return Ok(None);
    };
    let fields = rest.split(" | ").collect::<Vec<_>>();
    if fields.len() < 2 {
        return Ok(None);
    }

    let name = clean_cell(fields[0]);
    let symbol = clean_cell(fields[1]);
    let safe_full_row = fields.len() == 8 && fields.last().is_some_and(|cell| cell.is_empty());
    let primary_designation = safe_full_row.then(|| clean_cell(fields[2])).flatten();
    let complete_formulation = safe_full_row.then(|| clean_cell(fields[3])).flatten();
    let formulation_breakdown = safe_full_row.then(|| clean_cell(fields[4])).flatten();
    let metaphysical_names = if safe_full_row {
        clean_cell(fields[5])
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let description = safe_full_row.then(|| clean_cell(fields[6])).flatten();

    Ok(Some(VakEntry {
        source: VakSourceProvenance {
            repository: VAK_SOURCE_REPOSITORY,
            revision: VAK_SOURCE_REVISION,
            path: VAK_SOURCE_PATH,
            git_blob: VAK_SOURCE_GIT_BLOB,
            coordinate: vak_ref.clone(),
            source_line,
            standing: VakStanding::SourceBacked,
        },
        vak_ref,
        name,
        symbol,
        primary_designation,
        complete_formulation,
        formulation_breakdown,
        metaphysical_names,
        description,
        r_factors: extract_r_factors(trimmed),
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
