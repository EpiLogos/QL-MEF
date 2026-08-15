//! Interoperable OKF/`okf-wiki/v1` meta-wiki support for QL-MEF.
//!
//! QL-MEF owns the ontology authored in its own WikiSpace and the mapping ontology
//! represented by [`MetaBinding`]. External Wiki identities stay foreign and are
//! never copied into, or renamed by, the local projection.

mod portal;
mod recognition;
mod refraction;
mod registry_provider;

pub use portal::{
    CrossWikiTraversalRequest, CrossWikiTraversalResponse, ForeignKnowledgeResolver,
    ForeignTargetResolution, META_PORTAL_CONTRACT, ManifestationsResponse, MetaContextResponse,
    MetaManifestation, MetaPortal, MetaRoute, MetaRouteStep, MetaRouteSurface, PortalError,
    PortalScope, StaticForeignResolver, TargetAvailability,
};
pub use recognition::{
    ExternalObservation, META_RECOGNITION_CONTRACT, MetaAmendmentCandidate,
    RecognisedMetaAmendment, RecognitionDecision, RecognitionError, RecognitionLedger,
    RecognitionState, apply_recognised_amendment,
};
pub use refraction::{
    DerivedRelation, DerivedSubgraph, DerivedVertex, FieldCoordinate, LensSelection, ProviderMode,
    ReadingProvider, RefractionStatus, RelationCandidate, RevisionValue, TraversalCandidate,
    WIKI_READING_TYPE, WIKI_REFRACTION_CONTRACT, WikiProvenanceRef, WikiReading,
    WikiRefractionEngine, WikiRefractionError, WikiRefractionRequest, WikiRefractionResponse,
    WikiRefractionTarget, WikiStructuralField, WikiSubjectSnapshot, WikiTargetKind,
    WikiTargetRelation,
};
pub use registry_provider::RegistryDisclosureProvider;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const OKF_WIKI_PROFILE: &str = "okf-wiki/v1";
pub const QL_MEF_WIKI_PROFILE: &str = "ql-mef/wiki/v1";
pub const META_KNOWLEDGE_GRAPH_NAME: &str = "QL-MEF Meta-Knowledge Graph Projection";
pub const RESERVED_BIMBA_GRAPH_NAME: &str = "Epi-Logos Bimba Graph";

#[derive(Debug)]
pub enum WikiError {
    MissingFrontmatter,
    InvalidFrontmatter(String),
    MissingField(&'static str),
    InvalidField(&'static str),
    WrongWikiProfile(String),
    ProviderIdentityLeak(String),
    DuplicateCanonicalRef(String),
    InvalidMetaBinding(String),
}

impl fmt::Display for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFrontmatter => f.write_str("document must begin with YAML frontmatter"),
            Self::InvalidFrontmatter(detail) => write!(f, "invalid YAML frontmatter: {detail}"),
            Self::MissingField(field) => write!(f, "missing required field {field}"),
            Self::InvalidField(field) => write!(f, "invalid field {field}"),
            Self::WrongWikiProfile(value) => {
                write!(f, "expected {OKF_WIKI_PROFILE}, got {value}")
            }
            Self::ProviderIdentityLeak(field) => {
                write!(
                    f,
                    "provider/index identity cannot be canonical Wiki identity: {field}"
                )
            }
            Self::DuplicateCanonicalRef(value) => write!(f, "duplicate canonical Wiki ref {value}"),
            Self::InvalidMetaBinding(detail) => write!(f, "invalid MetaBinding: {detail}"),
        }
    }
}

impl std::error::Error for WikiError {}

#[derive(Debug, Clone, PartialEq)]
pub struct OkfWikiDocument {
    /// Entire OKF frontmatter, including unknown producer extensions.
    pub okf: Map<String, Value>,
    pub wiki: WikiRecord,
    pub body: String,
}

impl OkfWikiDocument {
    pub fn ql_mef_profile(&self) -> Option<&str> {
        self.okf.get("ql_mef_profile").and_then(Value::as_str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WikiRecord {
    pub profile: String,
    pub object_kind: String,
    pub canonical_ref: String,
    pub revision: u64,
    pub provenance: Vec<Value>,
    /// Entire Wiki object, including unknown producer/profile extensions.
    pub raw: Map<String, Value>,
}

impl WikiRecord {
    pub fn from_value(value: Value) -> Result<Self, WikiError> {
        let raw = value
            .as_object()
            .cloned()
            .ok_or(WikiError::InvalidField("wiki"))?;
        reject_provider_identity(&raw)?;
        let profile = required_string(&raw, "profile")?;
        if profile != OKF_WIKI_PROFILE {
            return Err(WikiError::WrongWikiProfile(profile));
        }
        let object_kind = required_string(&raw, "object")?;
        let canonical_ref = required_string(&raw, "ref")?;
        let revision = raw
            .get("revision")
            .and_then(Value::as_u64)
            .ok_or(WikiError::InvalidField("wiki.revision"))?;
        let provenance = raw
            .get("provenance")
            .and_then(Value::as_array)
            .cloned()
            .ok_or(WikiError::InvalidField("wiki.provenance"))?;
        Ok(Self {
            profile,
            object_kind,
            canonical_ref,
            revision,
            provenance,
            raw,
        })
    }

    pub fn title(&self) -> Option<&str> {
        self.raw.get("title").and_then(Value::as_str)
    }

    pub fn string(&self, key: &str) -> Option<&str> {
        self.raw.get(key).and_then(Value::as_str)
    }

    pub fn refs(&self, key: &str) -> Vec<String> {
        self.raw
            .get(key)
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    }
}

pub fn parse_okf_wiki(markdown: &str) -> Result<OkfWikiDocument, WikiError> {
    let (yaml, body) = split_frontmatter(markdown)?;
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(yaml)
        .map_err(|error| WikiError::InvalidFrontmatter(error.to_string()))?;
    let json = serde_json::to_value(yaml_value)
        .map_err(|error| WikiError::InvalidFrontmatter(error.to_string()))?;
    let okf = json
        .as_object()
        .cloned()
        .ok_or(WikiError::InvalidFrontmatter(
            "root must be a mapping".to_owned(),
        ))?;
    let profile = okf
        .get("wiki_profile")
        .and_then(Value::as_str)
        .ok_or(WikiError::MissingField("wiki_profile"))?;
    if profile != OKF_WIKI_PROFILE {
        return Err(WikiError::WrongWikiProfile(profile.to_owned()));
    }
    let wiki = okf
        .get("wiki")
        .cloned()
        .ok_or(WikiError::MissingField("wiki"))?;
    Ok(OkfWikiDocument {
        okf,
        wiki: WikiRecord::from_value(wiki)?,
        body: body.to_owned(),
    })
}

fn split_frontmatter(markdown: &str) -> Result<(&str, &str), WikiError> {
    let rest = markdown
        .strip_prefix("---\n")
        .ok_or(WikiError::MissingFrontmatter)?;
    let end = rest.find("\n---\n").ok_or(WikiError::MissingFrontmatter)?;
    let yaml = &rest[..end];
    let body = &rest[end + 5..];
    Ok((yaml, body))
}

fn required_string(map: &Map<String, Value>, key: &'static str) -> Result<String, WikiError> {
    let value = map
        .get(key)
        .and_then(Value::as_str)
        .ok_or(WikiError::MissingField(key))?;
    if value.trim().is_empty() {
        return Err(WikiError::InvalidField(key));
    }
    Ok(value.to_owned())
}

fn reject_provider_identity(map: &Map<String, Value>) -> Result<(), WikiError> {
    for key in [
        "provider_id",
        "providerId",
        "row_id",
        "rowId",
        "db_id",
        "dbId",
    ] {
        if map.contains_key(key) {
            return Err(WikiError::ProviderIdentityLeak(key.to_owned()));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MappingOrigin {
    Authored,
    Recognised,
    Derived,
    Proposed,
}

impl MappingOrigin {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Recognised => "recognised",
            Self::Derived => "derived",
            Self::Proposed => "proposed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaProvenance {
    pub source_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaBinding {
    pub binding_ref: String,
    pub ql_mef_ref: String,
    pub target_wiki_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_frame_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_provider_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_ref: Option<String>,
    pub relation: String,
    #[serde(default)]
    pub scope_refs: Vec<String>,
    pub origin: MappingOrigin,
    #[serde(default)]
    pub provenance: Vec<MetaProvenance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ql_mef_revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_revision: Option<String>,
    #[serde(default)]
    pub extensions: BTreeMap<String, Value>,
}

impl MetaBinding {
    pub fn validate(&self) -> Result<(), WikiError> {
        for (field, value) in [
            ("binding_ref", &self.binding_ref),
            ("ql_mef_ref", &self.ql_mef_ref),
            ("target_wiki_ref", &self.target_wiki_ref),
            ("relation", &self.relation),
        ] {
            if value.trim().is_empty() {
                return Err(WikiError::InvalidMetaBinding(format!("{field} is empty")));
            }
        }
        if self
            .target_frame_ref
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(WikiError::InvalidMetaBinding(
                "target_frame_ref is empty".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn target_ref(&self) -> &str {
        self.target_frame_ref
            .as_deref()
            .unwrap_or(&self.target_wiki_ref)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedObject {
    /// Projection-local implementation binding. Never use this as semantic identity.
    pub projection_id: u64,
    pub canonical_ref: String,
    pub object_kind: String,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedRelation {
    /// Projection-local implementation binding. Never use this as semantic identity.
    pub projection_id: u64,
    pub from_ref: String,
    pub to_ref: String,
    pub relation: String,
    pub origin: String,
    pub origin_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaKnowledgeProjection {
    pub projection_version: u64,
    pub objects: Vec<ProjectedObject>,
    pub relations: Vec<ProjectedRelation>,
    pub meta_bindings: Vec<MetaBinding>,
}

impl MetaKnowledgeProjection {
    pub fn rebuild(
        documents: &[OkfWikiDocument],
        bindings: &[MetaBinding],
        projection_version: u64,
    ) -> Result<Self, WikiError> {
        let mut seen = BTreeSet::new();
        let mut ordered = documents.iter().collect::<Vec<_>>();
        ordered.sort_by(|left, right| left.wiki.canonical_ref.cmp(&right.wiki.canonical_ref));
        let mut objects = Vec::with_capacity(ordered.len());
        let mut relations = Vec::new();
        let mut next_relation_id = 1_u64;

        for (index, document) in ordered.into_iter().enumerate() {
            let wiki = &document.wiki;
            if !seen.insert(wiki.canonical_ref.clone()) {
                return Err(WikiError::DuplicateCanonicalRef(wiki.canonical_ref.clone()));
            }
            objects.push(ProjectedObject {
                projection_id: index as u64 + 1,
                canonical_ref: wiki.canonical_ref.clone(),
                object_kind: wiki.object_kind.clone(),
                revision: wiki.revision,
            });

            match wiki.object_kind.as_str() {
                "edge" => {
                    if let (Some(from_ref), Some(to_ref), Some(relation)) = (
                        wiki.string("from_ref"),
                        wiki.string("to_ref"),
                        wiki.string("relation"),
                    ) {
                        relations.push(ProjectedRelation {
                            projection_id: next_relation_id,
                            from_ref: from_ref.to_owned(),
                            to_ref: to_ref.to_owned(),
                            relation: relation.to_owned(),
                            origin: wiki.string("origin").unwrap_or("authored").to_owned(),
                            origin_ref: Some(wiki.canonical_ref.clone()),
                        });
                        next_relation_id += 1;
                    }
                }
                "space" => {
                    for child in wiki.refs("child_space_refs") {
                        relations.push(ProjectedRelation {
                            projection_id: next_relation_id,
                            from_ref: wiki.canonical_ref.clone(),
                            to_ref: child,
                            relation: "contains-space".to_owned(),
                            origin: "mechanical".to_owned(),
                            origin_ref: Some(wiki.canonical_ref.clone()),
                        });
                        next_relation_id += 1;
                    }
                    for node in wiki.refs("node_refs") {
                        relations.push(ProjectedRelation {
                            projection_id: next_relation_id,
                            from_ref: wiki.canonical_ref.clone(),
                            to_ref: node,
                            relation: "contains-node".to_owned(),
                            origin: "mechanical".to_owned(),
                            origin_ref: Some(wiki.canonical_ref.clone()),
                        });
                        next_relation_id += 1;
                    }
                }
                _ => {}
            }
        }

        let mut meta_bindings = bindings.to_vec();
        meta_bindings.sort_by(|left, right| left.binding_ref.cmp(&right.binding_ref));
        for binding in &meta_bindings {
            binding.validate()?;
            relations.push(ProjectedRelation {
                projection_id: next_relation_id,
                from_ref: binding.ql_mef_ref.clone(),
                to_ref: binding.target_ref().to_owned(),
                relation: binding.relation.clone(),
                origin: binding.origin.as_str().to_owned(),
                origin_ref: Some(binding.binding_ref.clone()),
            });
            next_relation_id += 1;
        }

        Ok(Self {
            projection_version,
            objects,
            relations,
            meta_bindings,
        })
    }

    pub fn canonical_refs(&self) -> BTreeSet<&str> {
        self.objects
            .iter()
            .map(|object| object.canonical_ref.as_str())
            .collect()
    }

    pub fn mapped_targets(&self, ql_mef_ref: &str) -> Vec<&MetaBinding> {
        self.meta_bindings
            .iter()
            .filter(|binding| binding.ql_mef_ref == ql_mef_ref)
            .collect()
    }

    pub fn mappings_for_target(&self, target_ref: &str) -> Vec<&MetaBinding> {
        self.meta_bindings
            .iter()
            .filter(|binding| {
                binding.target_wiki_ref == target_ref
                    || binding.target_frame_ref.as_deref() == Some(target_ref)
            })
            .collect()
    }

    pub fn contains_foreign_object(&self, target_ref: &str) -> bool {
        self.objects
            .iter()
            .any(|object| object.canonical_ref == target_ref)
    }
}
