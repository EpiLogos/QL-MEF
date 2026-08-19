use core::str::FromStr;
use std::collections::BTreeMap;

use ql_core::{
    ConjugateOpposition, ExpansionSide, QlCoordinate, QlFace, QlPosition, RelationFamily,
};
use ql_mef::{ClientRef, LensRef, QlTarget, SublensRef};
use ql_semantic::{
    Operation, ProviderError, ProviderState, QlProvider, RefractRequest, SemanticStatus,
    TargetInput,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const WIKI_REFRACTION_CONTRACT: &str = "ql-mef/wiki-refraction/v1";
pub const WIKI_READING_TYPE: &str = "MEF-derived";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WikiRefractionError {
    InvalidContract(String),
    InvalidTarget(String),
    InvalidStructuralField(String),
    InvalidLens(String),
    ProviderRequired(String),
    Provider(String),
}

impl core::fmt::Display for WikiRefractionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidContract(value) => write!(f, "invalid Wiki refraction contract: {value}"),
            Self::InvalidTarget(value) => write!(f, "invalid Wiki refraction target: {value}"),
            Self::InvalidStructuralField(value) => {
                write!(f, "invalid Wiki structural field: {value}")
            }
            Self::InvalidLens(value) => write!(f, "invalid lens selection: {value}"),
            Self::ProviderRequired(value) => {
                write!(f, "required QL-MEF provider unavailable: {value}")
            }
            Self::Provider(value) => write!(f, "QL-MEF provider failed: {value}"),
        }
    }
}

impl std::error::Error for WikiRefractionError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevisionValue {
    Integer(u64),
    String(String),
}

impl core::fmt::Display for RevisionValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::String(value) => f.write_str(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderMode {
    Disabled,
    Optional,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WikiTargetKind {
    NodeLocal,
    Frame,
    Pair,
    D1,
    D2,
    D3,
    Space,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiProvenanceRef {
    pub source_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision: Option<RevisionValue>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiSubjectSnapshot {
    pub subject_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiTargetRelation {
    pub from_ref: String,
    pub to_ref: String,
    pub relation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_ref: Option<String>,
    #[serde(default)]
    pub provenance: Vec<WikiProvenanceRef>,
    #[serde(flatten)]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldCoordinate {
    pub position: u8,
    pub face: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiStructuralField {
    pub operator_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pair_index: Option<u8>,
    pub degree: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expansion_side: Option<String>,
    pub coordinates: Vec<FieldCoordinate>,
    #[serde(default)]
    pub provenance: Vec<WikiProvenanceRef>,
}

impl WikiStructuralField {
    pub fn validate(&self) -> Result<(), WikiRefractionError> {
        match self.degree.as_str() {
            "pair" => {
                let (family, pair) = self.pair()?;
                let expected = pair.operator_ref();
                self.expect_operator(&expected)?;
                self.expect_coordinates(&[
                    QlCoordinate::new(pair.left, QlFace::Direct),
                    QlCoordinate::new(pair.right, QlFace::Direct),
                ])?;
                if self.expansion_side.is_some() {
                    return Err(WikiRefractionError::InvalidStructuralField(
                        "pair target cannot carry expansion_side".into(),
                    ));
                }
                let _ = family;
            }
            "D1" => {
                if self.family.is_some()
                    || self.pair_index.is_some()
                    || self.expansion_side.is_some()
                {
                    return Err(WikiRefractionError::InvalidStructuralField(
                        "D1 is a family-independent same-position conjugation axis".into(),
                    ));
                }
                if self.coordinates.len() != 2 {
                    return Err(WikiRefractionError::InvalidStructuralField(
                        "D1 requires exactly two same-position coordinates".into(),
                    ));
                }
                let parsed = self.parsed_coordinates()?;
                let position = parsed[0].position;
                let expected = ConjugateOpposition::new(position);
                self.expect_operator(&expected.operator_ref())?;
                self.expect_coordinates(&expected.coordinates())?;
            }
            "D2" => {
                let (_, pair) = self.pair()?;
                let side = match self.expansion_side.as_deref() {
                    Some("left") => ExpansionSide::Left,
                    Some("right") => ExpansionSide::Right,
                    _ => {
                        return Err(WikiRefractionError::InvalidStructuralField(
                            "D2 requires expansion_side left|right".into(),
                        ));
                    }
                };
                let expected = pair.d2(side);
                self.expect_operator(&expected.operator_ref())?;
                self.expect_coordinates(&expected.coordinates)?;
            }
            "D3" => {
                let (_, pair) = self.pair()?;
                if self.expansion_side.is_some() {
                    return Err(WikiRefractionError::InvalidStructuralField(
                        "D3 expands both endpoints and cannot carry expansion_side".into(),
                    ));
                }
                let expected = pair.d3();
                self.expect_operator(&expected.operator_ref())?;
                self.expect_coordinates(&expected.coordinates)?;
            }
            other => {
                return Err(WikiRefractionError::InvalidStructuralField(format!(
                    "unknown structural degree {other}"
                )));
            }
        }
        Ok(())
    }

    fn pair(&self) -> Result<(RelationFamily, ql_core::PairInstance), WikiRefractionError> {
        let family = match self.family.as_deref() {
            Some("A") => RelationFamily::A,
            Some("B") => RelationFamily::B,
            Some("C") => RelationFamily::C,
            _ => {
                return Err(WikiRefractionError::InvalidStructuralField(
                    "pair/D2/D3 requires family A|B|C".into(),
                ));
            }
        };
        let pair_index = self.pair_index.ok_or_else(|| {
            WikiRefractionError::InvalidStructuralField(
                "pair/D2/D3 requires pair_index 0..2".into(),
            )
        })?;
        let pair = family
            .pair(pair_index)
            .map_err(|error| WikiRefractionError::InvalidStructuralField(error.to_string()))?;
        Ok((family, pair))
    }

    fn expect_operator(&self, expected: &str) -> Result<(), WikiRefractionError> {
        if self.operator_ref == expected {
            Ok(())
        } else {
            Err(WikiRefractionError::InvalidStructuralField(format!(
                "operator_ref {} does not match canonical {expected}",
                self.operator_ref
            )))
        }
    }

    fn parsed_coordinates(&self) -> Result<Vec<QlCoordinate>, WikiRefractionError> {
        self.coordinates
            .iter()
            .map(|coordinate| {
                let position = QlPosition::new(coordinate.position).map_err(|_| {
                    WikiRefractionError::InvalidStructuralField(format!(
                        "position {} is outside 0..5",
                        coordinate.position
                    ))
                })?;
                let face = parse_face(&coordinate.face)?;
                Ok(QlCoordinate::new(position, face))
            })
            .collect()
    }

    fn expect_coordinates(&self, expected: &[QlCoordinate]) -> Result<(), WikiRefractionError> {
        let mut observed = self.parsed_coordinates()?;
        let mut expected = expected.to_vec();
        observed.sort_by_key(|coordinate| (coordinate.position.value(), coordinate.face.as_str()));
        expected.sort_by_key(|coordinate| (coordinate.position.value(), coordinate.face.as_str()));
        if observed == expected {
            Ok(())
        } else {
            Err(WikiRefractionError::InvalidStructuralField(
                "coordinates do not match canonical operator field".into(),
            ))
        }
    }
}

fn parse_face(value: &str) -> Result<QlFace, WikiRefractionError> {
    match value {
        "direct" => Ok(QlFace::Direct),
        "conjugate" => Ok(QlFace::Conjugate),
        other => Err(WikiRefractionError::InvalidStructuralField(format!(
            "unknown face {other}"
        ))),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiRefractionTarget {
    pub kind: WikiTargetKind,
    pub target_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_frame_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_revision: Option<RevisionValue>,
    pub target_snapshot_hash: String,
    #[serde(default)]
    pub provenance: Vec<WikiProvenanceRef>,
    #[serde(default)]
    pub subjects: Vec<WikiSubjectSnapshot>,
    #[serde(default)]
    pub relations: Vec<WikiTargetRelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structural_field: Option<WikiStructuralField>,
    #[serde(default)]
    pub material: Map<String, Value>,
    #[serde(default)]
    pub extensions: Map<String, Value>,
}

impl WikiRefractionTarget {
    pub fn validate(&self) -> Result<(), WikiRefractionError> {
        for (field, value) in [
            ("target_ref", self.target_ref.as_str()),
            ("target_snapshot_hash", self.target_snapshot_hash.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(WikiRefractionError::InvalidTarget(format!(
                    "{field} cannot be empty"
                )));
            }
        }
        if self
            .target_frame_ref
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(WikiRefractionError::InvalidTarget(
                "target_frame_ref cannot be empty".into(),
            ));
        }
        for subject in &self.subjects {
            if subject.subject_ref.trim().is_empty() {
                return Err(WikiRefractionError::InvalidTarget(
                    "subject_ref cannot be empty".into(),
                ));
            }
            if let Some(position) = subject.position {
                QlPosition::new(position).map_err(|_| {
                    WikiRefractionError::InvalidTarget(format!(
                        "subject position {position} outside 0..5"
                    ))
                })?;
            }
            if let Some(face) = subject.face.as_deref() {
                parse_face(face)?;
            }
        }
        for relation in &self.relations {
            if relation.from_ref.trim().is_empty()
                || relation.to_ref.trim().is_empty()
                || relation.relation.trim().is_empty()
            {
                return Err(WikiRefractionError::InvalidTarget(
                    "relation endpoints and relation name must be non-empty".into(),
                ));
            }
        }
        match self.kind {
            WikiTargetKind::Pair | WikiTargetKind::D1 | WikiTargetKind::D2 | WikiTargetKind::D3 => {
                let field = self.structural_field.as_ref().ok_or_else(|| {
                    WikiRefractionError::InvalidTarget(
                        "structural target requires structural_field".into(),
                    )
                })?;
                field.validate()?;
                let expected = match self.kind {
                    WikiTargetKind::Pair => "pair",
                    WikiTargetKind::D1 => "D1",
                    WikiTargetKind::D2 => "D2",
                    WikiTargetKind::D3 => "D3",
                    _ => unreachable!(),
                };
                if field.degree != expected {
                    return Err(WikiRefractionError::InvalidTarget(format!(
                        "target kind requires structural degree {expected}"
                    )));
                }
            }
            WikiTargetKind::NodeLocal | WikiTargetKind::Frame | WikiTargetKind::Space => {
                if let Some(field) = &self.structural_field {
                    field.validate()?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LensSelection {
    pub lens_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sublens_ref: Option<String>,
}

impl LensSelection {
    fn parse(&self) -> Result<(LensRef, Option<SublensRef>), WikiRefractionError> {
        let lens = LensRef::from_str(&self.lens_ref)
            .map_err(|error| WikiRefractionError::InvalidLens(error.to_string()))?;
        let sublens = self
            .sublens_ref
            .as_deref()
            .map(SublensRef::from_str)
            .transpose()
            .map_err(|error| WikiRefractionError::InvalidLens(error.to_string()))?;
        if sublens.is_some_and(|value| value.lens() != lens) {
            return Err(WikiRefractionError::InvalidLens(
                "sublens does not belong to selected lens".into(),
            ));
        }
        Ok((lens, sublens))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiRefractionRequest {
    pub contract: String,
    pub mode: ProviderMode,
    pub target: WikiRefractionTarget,
    pub lenses: Vec<LensSelection>,
    #[serde(default)]
    pub context: Map<String, Value>,
}

impl WikiRefractionRequest {
    pub fn validate(&self) -> Result<(), WikiRefractionError> {
        if self.contract != WIKI_REFRACTION_CONTRACT {
            return Err(WikiRefractionError::InvalidContract(self.contract.clone()));
        }
        self.target.validate()?;
        if self.mode != ProviderMode::Disabled && self.lenses.is_empty() {
            return Err(WikiRefractionError::InvalidLens(
                "enabled refraction requires at least one lens".into(),
            ));
        }
        for lens in &self.lenses {
            lens.parse()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedVertex {
    pub subject_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedRelation {
    pub from_ref: String,
    pub to_ref: String,
    pub relation: String,
    pub origin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_ref: Option<String>,
    #[serde(default)]
    pub provenance: Vec<WikiProvenanceRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedSubgraph {
    pub vertices: Vec<DerivedVertex>,
    pub relations: Vec<DerivedRelation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationCandidate {
    pub candidate_ref: String,
    pub from_ref: String,
    pub to_ref: String,
    pub relation: String,
    pub origin: String,
    pub state: String,
    #[serde(default)]
    pub provenance: Vec<WikiProvenanceRef>,
    #[serde(default)]
    pub extensions: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraversalCandidate {
    pub refs: Vec<String>,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadingProvider {
    pub provider_ref: String,
    pub provider_version: String,
    pub health: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiReading {
    pub contract: String,
    pub reading_ref: String,
    pub reading_type: String,
    pub target_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_frame_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_revision: Option<RevisionValue>,
    pub target_snapshot_hash: String,
    pub provider: ReadingProvider,
    pub lens_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sublens_ref: Option<String>,
    #[serde(default)]
    pub ql_form_refs: Vec<String>,
    #[serde(default)]
    pub operator_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harmonic_field_ref: Option<String>,
    pub disclosure: String,
    pub disclosure_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_per_mille: Option<u16>,
    pub derived_subgraph: DerivedSubgraph,
    #[serde(default)]
    pub relation_candidates: Vec<RelationCandidate>,
    #[serde(default)]
    pub traversal_candidates: Vec<TraversalCandidate>,
    #[serde(default)]
    pub tensions: Vec<String>,
    #[serde(default)]
    pub absences: Vec<String>,
    #[serde(default)]
    pub evidence_demands: Vec<String>,
    pub explanation: String,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub provenance: Vec<WikiProvenanceRef>,
    pub result_class: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub extensions: Map<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RefractionStatus {
    Disabled,
    Unavailable,
    Degraded,
    Complete,
    Partial,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiRefractionResponse {
    pub contract: String,
    pub status: RefractionStatus,
    pub target_ref: String,
    pub target_snapshot_hash: String,
    pub readings: Vec<WikiReading>,
    #[serde(default)]
    pub notices: Vec<String>,
}

pub struct WikiRefractionEngine<'a> {
    provider: Option<&'a dyn QlProvider>,
}

impl<'a> WikiRefractionEngine<'a> {
    pub const fn new(provider: Option<&'a dyn QlProvider>) -> Self {
        Self { provider }
    }

    pub fn refract(
        &self,
        request: &WikiRefractionRequest,
    ) -> Result<WikiRefractionResponse, WikiRefractionError> {
        // Structural and lens validation is deliberately first: invalid client
        // coordinates/operator refs are never translated into provider absence.
        request.validate()?;

        if request.mode == ProviderMode::Disabled {
            return Ok(WikiRefractionResponse {
                contract: WIKI_REFRACTION_CONTRACT.into(),
                status: RefractionStatus::Disabled,
                target_ref: request.target.target_ref.clone(),
                target_snapshot_hash: request.target.target_snapshot_hash.clone(),
                readings: vec![],
                notices: vec!["QL-MEF refraction disabled; client material unchanged".into()],
            });
        }

        let Some(provider) = self.provider else {
            return self.unavailable(request, "no QL-MEF provider configured");
        };
        let capabilities = provider.capabilities();
        match capabilities.health.state {
            ProviderState::Absent | ProviderState::Incompatible => {
                return self.unavailable(
                    request,
                    capabilities
                        .health
                        .detail
                        .as_deref()
                        .unwrap_or("provider unavailable or incompatible"),
                );
            }
            ProviderState::Available | ProviderState::Degraded => {}
        }
        if !capabilities.supports(Operation::Refract) {
            return self.unavailable(request, "provider does not advertise refract");
        }

        let mut readings = Vec::new();
        let mut notices = Vec::new();
        for selection in &request.lenses {
            let (lens, sublens) = selection.parse()?;
            if !capabilities.supported_lenses.contains(&lens) {
                match request.mode {
                    ProviderMode::Required => {
                        return Err(WikiRefractionError::ProviderRequired(format!(
                            "provider does not advertise lens {lens}"
                        )));
                    }
                    ProviderMode::Optional => {
                        notices.push(format!("provider does not advertise lens {lens}"));
                        continue;
                    }
                    ProviderMode::Disabled => unreachable!(),
                }
            }
            let target_ref = ClientRef::new(request.target.target_ref.clone())
                .map_err(|error| WikiRefractionError::InvalidTarget(error.to_string()))?;
            let mut target = QlTarget::new(target_ref);
            target.subject_type = Some(format!("wiki:{:?}", request.target.kind));
            target.frame_ref = request.target.target_frame_ref.clone();
            let revision = request
                .target
                .target_revision
                .as_ref()
                .map(ToString::to_string);
            let semantic = provider.refract(RefractRequest {
                input: TargetInput::new(target, revision),
                lens,
                sublens,
                frame: None,
            });
            match semantic {
                Ok(reading) => {
                    readings.push(wrap_reading(request, selection, &capabilities, reading))
                }
                Err(ProviderError::InvalidRequest(message)) => {
                    return Err(WikiRefractionError::Provider(format!(
                        "provider rejected validated request: {message}"
                    )));
                }
                Err(error) if request.mode == ProviderMode::Optional => {
                    notices.push(error.to_string());
                }
                Err(error) => {
                    return Err(WikiRefractionError::ProviderRequired(error.to_string()));
                }
            }
        }

        let status = if readings.is_empty() {
            RefractionStatus::Unavailable
        } else if capabilities.health.state == ProviderState::Degraded || !notices.is_empty() {
            RefractionStatus::Degraded
        } else if readings
            .iter()
            .any(|reading| reading.disclosure_status != "complete")
        {
            RefractionStatus::Partial
        } else {
            RefractionStatus::Complete
        };
        Ok(WikiRefractionResponse {
            contract: WIKI_REFRACTION_CONTRACT.into(),
            status,
            target_ref: request.target.target_ref.clone(),
            target_snapshot_hash: request.target.target_snapshot_hash.clone(),
            readings,
            notices,
        })
    }

    fn unavailable(
        &self,
        request: &WikiRefractionRequest,
        detail: &str,
    ) -> Result<WikiRefractionResponse, WikiRefractionError> {
        match request.mode {
            ProviderMode::Required => Err(WikiRefractionError::ProviderRequired(detail.into())),
            ProviderMode::Optional => Ok(WikiRefractionResponse {
                contract: WIKI_REFRACTION_CONTRACT.into(),
                status: RefractionStatus::Unavailable,
                target_ref: request.target.target_ref.clone(),
                target_snapshot_hash: request.target.target_snapshot_hash.clone(),
                readings: vec![],
                notices: vec![detail.into()],
            }),
            ProviderMode::Disabled => unreachable!(),
        }
    }
}

fn wrap_reading(
    request: &WikiRefractionRequest,
    selection: &LensSelection,
    capabilities: &ql_semantic::ProviderCapabilities,
    reading: ql_semantic::SemanticReading,
) -> WikiReading {
    let disclosure_status = match reading.reading.status {
        SemanticStatus::Complete => "complete",
        SemanticStatus::Partial => "partial",
        SemanticStatus::Ambiguous => "ambiguous",
        SemanticStatus::InsufficientEvidence => "insufficient-evidence",
    }
    .to_owned();
    let mut provenance = request.target.provenance.clone();
    for input in &reading.provenance.input_refs {
        provenance.push(WikiProvenanceRef {
            source_ref: input.reference.to_string(),
            source_revision: input.revision.clone().map(RevisionValue::String),
            extensions: BTreeMap::new(),
        });
    }
    let mut operator_refs = Vec::new();
    let harmonic_field_ref = request.target.structural_field.as_ref().map(|field| {
        operator_refs.push(field.operator_ref.clone());
        field.operator_ref.clone()
    });
    let derived_subgraph = DerivedSubgraph {
        vertices: request
            .target
            .subjects
            .iter()
            .map(|subject| DerivedVertex {
                subject_ref: subject.subject_ref.clone(),
                revision: subject.revision.clone(),
            })
            .collect(),
        relations: request
            .target
            .relations
            .iter()
            .map(|relation| DerivedRelation {
                from_ref: relation.from_ref.clone(),
                to_ref: relation.to_ref.clone(),
                relation: relation.relation.clone(),
                origin: relation
                    .origin
                    .clone()
                    .unwrap_or_else(|| "client-authored".into()),
                origin_ref: relation.origin_ref.clone(),
                provenance: relation.provenance.clone(),
            })
            .collect(),
    };
    WikiReading {
        contract: WIKI_REFRACTION_CONTRACT.into(),
        reading_ref: reading.id.to_string(),
        reading_type: WIKI_READING_TYPE.into(),
        target_ref: request.target.target_ref.clone(),
        target_frame_ref: request.target.target_frame_ref.clone(),
        target_revision: request.target.target_revision.clone(),
        target_snapshot_hash: request.target.target_snapshot_hash.clone(),
        provider: ReadingProvider {
            provider_ref: reading.provenance.provider.provider.clone(),
            provider_version: reading.provenance.provider.version.clone(),
            health: format!("{:?}", capabilities.health.state).to_lowercase(),
        },
        lens_ref: selection.lens_ref.clone(),
        sublens_ref: selection.sublens_ref.clone(),
        ql_form_refs: reading
            .ql_form
            .map(|form| form.to_string())
            .into_iter()
            .collect(),
        operator_refs,
        harmonic_field_ref,
        disclosure: reading.reading.text,
        disclosure_status,
        confidence_per_mille: reading.reading.confidence_per_mille,
        derived_subgraph,
        relation_candidates: vec![],
        traversal_candidates: vec![],
        tensions: vec![],
        absences: vec![],
        evidence_demands: vec![],
        explanation: format!(
            "QL-MEF refracted stable target {} through {} without mutating client identity",
            request.target.target_ref, selection.lens_ref
        ),
        evidence_refs: reading
            .evidence_refs
            .into_iter()
            .map(|reference| reference.to_string())
            .collect(),
        provenance,
        result_class: reading.provenance.result_class.to_string(),
        warnings: reading.warnings,
        extensions: request.context.clone(),
    }
}
