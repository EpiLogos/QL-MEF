#!/usr/bin/env python3
from pathlib import Path


def replace_once(source: str, old: str, new: str, label: str) -> str:
    if old not in source:
        raise SystemExit(f"missing patch anchor: {label}")
    return source.replace(old, new, 1)

vak_path = Path("crates/ql-mef/src/vak.rs")
vak = vak_path.read_text()

vak = replace_once(
    vak,
    '''#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
''',
    '''#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
''',
    "VakStanding",
)

vak = vak.replace("VakStanding::SourceBacked", "VakStanding::Source")
vak = vak.replace("VakStanding::ImplementationMapping", "VakStanding::Implementation")

vak = replace_once(
    vak,
    '''pub enum VakRelationKind {
    Parent,
    Child,
    SourceMentions,
    Expresses,
    InvokesThrough,
    TransformsThrough,
    ReadsThrough,
    Other,
}
''',
    '''pub enum VakRelationKind {
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
''',
    "VakRelationKind",
)

vak = replace_once(
    vak,
    '''pub struct VakNeighbourhood {
    pub centre: VakRef,
    pub depth: usize,
    pub entries: Vec<VakRef>,
    pub relations: Vec<VakRelation>,
}
''',
    '''pub struct VakNeighbourhood {
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
''',
    "context/R path types",
)

vak = replace_once(
    vak,
    '''impl fmt::Display for SelfOtherForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.glyph())
    }
}
''',
    '''impl fmt::Display for SelfOtherForm {
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
''',
    "speech stance",
)

vak = replace_once(
    vak,
    '''    pub fn relations_from(&self, reference: &VakRef) -> Result<Vec<VakRelation>, VakError> {
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
''',
    '''    pub fn relations_from(&self, reference: &VakRef) -> Result<Vec<VakRelation>, VakError> {
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
''',
    "relations_from",
)

vak = replace_once(
    vak,
    '''    pub fn praxis_reading(&self, aspect: VakPraxisAspect) -> VakPraxisReading {
        VakPraxisReading {
            aspect,
            source_refs: self
                .search_semantic(aspect.source_term())
                .into_iter()
                .map(|entry| entry.vak_ref.clone())
                .collect(),
            standing: VakStanding::Source,
        }
    }

    pub fn refract(
''',
    '''    pub fn formal_coverage(&self) -> VakFormalCoverage {
        VakFormalCoverage {
            names: self.entries().filter(|entry| entry.name.is_some()).count(),
            symbols: self.entries().filter(|entry| entry.symbol.is_some()).count(),
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

    pub fn context_relations_from(
        &self,
        reference: &VakRef,
    ) -> Result<Vec<VakRelation>, VakError> {
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
                    "EPI-VAK-OPERATIVE-SYNTAX-ARCHITECTURE §8: M0-4 contextual field"
                        .to_owned(),
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
            other => return Err(VakError::InvalidRef(format!("unknown R-path token `{other}`"))),
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
        if let (Some(reference), Some(formula)) = (&principle_nine_ref, act.principle_nine_formula()) {
            evidence.push(format!("{} exact Principle-9 appearance: {}", reference, formula));
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
            VakPraxisAspect::ActionSvatantrya => {
                &["M0-3-10", "M0-3-10-(0/1)", "M0-5-(5/0)-5"]
            }
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

    pub fn refract(
''',
    "registry exact relations",
)

vak = replace_once(
    vak,
    '''        standing: VakStanding::Source,
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
''',
    '''        standing: VakStanding::Derived,
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
                current.push('\\');
                current.push(ch);
            }
            escaped = false;
            continue;
        }
        if ch == '\\' {
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
        current.push('\\');
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
''',
    "source parser",
)

vak_path.write_text(vak)

test_path = Path("crates/ql-mef/tests/vak_language.rs")
tests = test_path.read_text()
tests = tests.replace("VakStanding::SourceBacked", "VakStanding::Source")
tests = tests.replace("VakStanding::ImplementationMapping", "VakStanding::Implementation")
tests = replace_once(
    tests,
    '''    VAK_SOURCE_REVISION, VakAddressHorizon, VakPraxisAspect, VakRef, VakRegistry, VakRelationOp,
    VakStanding,
''',
    '''    VAK_SOURCE_REVISION, VakAddressHorizon, VakContextField, VakDivineAct, VakPraxisAspect,
    VakRef, VakRegistry, VakRelationKind, VakRelationOp, VakSpeechStance, VakStanding,
''',
    "test imports",
)
tests += r'''

#[test]
fn exact_formal_property_coverage_matches_the_authoritative_source_receipt() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let coverage = registry.formal_coverage();
    assert_eq!(coverage.names, 109);
    assert_eq!(coverage.symbols, 107);
    assert_eq!(coverage.primary_designations, 108);
    assert_eq!(coverage.complete_formulations, 67);
    assert_eq!(coverage.formulation_breakdowns, 49);
    assert_eq!(coverage.metaphysical_names, 19);
    assert_eq!(coverage.descriptions, 97);
}

#[test]
fn coordinate_prefix_structure_is_derived_while_m0_4_context_chain_is_authored_architecture() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let structural = registry
        .relations_from(&VakRef::new("M0-3-6").unwrap())
        .unwrap();
    assert!(structural.iter().any(|relation| {
        relation.relation == VakRelationKind::Parent && relation.standing == VakStanding::Derived
    }));

    for field in VakContextField::ALL {
        let entry = registry.context_field_entry(field).unwrap();
        assert_eq!(entry.vak_ref.as_str(), field.source_coordinate());
        assert!(entry.raw_source_row.contains(field.symbol()));
    }
    let bimba = VakContextField::Bimba.source_ref();
    let relations = registry.context_relations_from(&bimba).unwrap();
    assert!(relations.iter().any(|relation| {
        relation.relation == VakRelationKind::Contextualises
            && relation.into_ref == VakContextField::Pratibimba.source_ref()
            && relation.standing == VakStanding::AuthoredArchitecture
    }));
}

#[test]
fn principle_nine_divine_action_paths_are_exact_source_relations() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let expected = [
        (VakDivineAct::Creation, "0R = @ = (9-O#-X#-N#)", vec!["9", "O#", "X#", "N#"]),
        (
            VakDivineAct::Sustenance,
            "1R = @ = (O#-X#-N#-M#-#-(#))",
            vec!["O#", "X#", "N#", "M#", "#", "(#)"],
        ),
        (
            VakDivineAct::Dissolution,
            "2R = @ = (X#-N#-M#-#-(#)-(@#))",
            vec!["X#", "N#", "M#", "#", "(#)", "(@#)"],
        ),
        (
            VakDivineAct::Veiling,
            "3R = @ = ((@#)-(#)-#-M#-N#-X#)",
            vec!["(@#)", "(#)", "#", "M#", "N#", "X#"],
        ),
        (
            VakDivineAct::Grace,
            "4R = @ = ((#)-#-M#-N#-X#-O#)",
            vec!["(#)", "#", "M#", "N#", "X#", "O#"],
        ),
        (VakDivineAct::Absorption, "5R = @ = (##)", vec!["##"]),
    ];
    for (act, formula, tokens) in expected {
        let path = registry.r_path(act).unwrap();
        assert_eq!(path.standing, VakStanding::Source);
        assert_eq!(path.principle_nine_formula.as_deref(), Some(formula));
        assert_eq!(
            path.steps.iter().map(|step| step.token.as_str()).collect::<Vec<_>>(),
            tokens
        );
        assert!(path.principle_nine_ref.is_some());
    }
    let freedom = registry.r_path(VakDivineAct::Freedom).unwrap();
    assert_eq!(freedom.steps[0].token, "R#");
    assert!(freedom.principle_nine_ref.is_none());
}

#[test]
fn m0_3_speech_forms_have_typed_source_grounded_stances() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let query = registry.parse_speech_act("-!").unwrap();
    assert_eq!(query.stance, VakSpeechStance::QueryOfOther);
    assert_eq!(query.standing, VakStanding::Source);
    assert_eq!(query.source_ref.as_str(), "M0-3-6-6");

    let reflexive = registry.parse_speech_act("?!").unwrap();
    assert_eq!(reflexive.stance, VakSpeechStance::ReflexiveQuery);
    let return_question = registry.parse_speech_act("?!/!?").unwrap();
    assert_eq!(
        return_question.stance,
        VakSpeechStance::WorldQuestioningSelf
    );
}

#[test]
fn will_knowledge_and_action_readings_use_explicit_source_coordinates() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let will = registry.praxis_reading(VakPraxisAspect::WillAgency);
    assert_eq!(
        will.source_refs,
        vec![
            VakRef::new("M0-3-3").unwrap(),
            VakRef::new("M0-3-6-2").unwrap(),
        ]
    );
    let knowledge = registry.praxis_reading(VakPraxisAspect::KnowledgeVimarsa);
    assert_eq!(
        knowledge.source_refs,
        vec![
            VakRef::new("M0-3-(0/1)").unwrap(),
            VakRef::new("M0-(4.0/1/2)").unwrap(),
        ]
    );
    let action = registry.praxis_reading(VakPraxisAspect::ActionSvatantrya);
    assert_eq!(
        action.source_refs,
        vec![
            VakRef::new("M0-3-10").unwrap(),
            VakRef::new("M0-3-10-(0/1)").unwrap(),
            VakRef::new("M0-5-(5/0)-5").unwrap(),
        ]
    );
}
'''
test_path.write_text(tests)
