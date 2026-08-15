use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{MappingOrigin, MetaBinding, MetaKnowledgeProjection, MetaProvenance};

pub const META_PORTAL_CONTRACT: &str = "ql-mef/meta-portal/v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortalError {
    InvalidRequest(String),
    InvalidScope(String),
}

impl core::fmt::Display for PortalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidRequest(value) => write!(f, "invalid meta-portal request: {value}"),
            Self::InvalidScope(value) => write!(f, "invalid meta-portal scope: {value}"),
        }
    }
}

impl std::error::Error for PortalError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetAvailability {
    Available,
    Unavailable,
    Restricted,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignTargetResolution {
    pub target_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    pub availability: TargetAvailability,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(default)]
    pub notices: Vec<String>,
}

pub trait ForeignKnowledgeResolver {
    fn resolve(
        &self,
        provider_ref: Option<&str>,
        target_ref: &str,
    ) -> ForeignTargetResolution;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortalScope {
    #[serde(default)]
    pub scope_refs: BTreeSet<String>,
    #[serde(default)]
    pub allow_payload: bool,
}

impl PortalScope {
    pub fn new(scope_refs: impl IntoIterator<Item = String>, allow_payload: bool) -> Self {
        Self {
            scope_refs: scope_refs.into_iter().collect(),
            allow_payload,
        }
    }

    pub fn unrestricted_payload() -> Self {
        Self {
            scope_refs: BTreeSet::new(),
            allow_payload: true,
        }
    }

    pub fn binding_visible(&self, binding: &MetaBinding) -> bool {
        binding.scope_refs.is_empty()
            || self.scope_refs.is_empty()
            || binding
                .scope_refs
                .iter()
                .any(|scope| self.scope_refs.contains(scope))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaManifestation {
    pub binding_ref: String,
    pub ql_mef_ref: String,
    pub target_wiki_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_frame_ref: Option<String>,
    pub target_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_provider_ref: Option<String>,
    pub relation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_ref: Option<String>,
    pub origin: MappingOrigin,
    #[serde(default)]
    pub scope_refs: Vec<String>,
    #[serde(default)]
    pub provenance: Vec<MetaProvenance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ql_mef_revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_revision: Option<String>,
    pub availability: TargetAvailability,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(default)]
    pub notices: Vec<String>,
    pub qualified_relation: bool,
    pub semantic_equivalence_asserted: bool,
}

impl MetaManifestation {
    fn from_binding(
        binding: &MetaBinding,
        scope: &PortalScope,
        resolver: Option<&dyn ForeignKnowledgeResolver>,
    ) -> Self {
        let target_ref = binding.target_ref().to_owned();
        let resolution = if !scope.allow_payload {
            ForeignTargetResolution {
                target_ref: target_ref.clone(),
                provider_ref: binding.target_provider_ref.clone(),
                revision: binding.target_revision.clone(),
                availability: TargetAvailability::Restricted,
                payload: None,
                notices: vec!["payload traversal disabled by resolved portal scope".into()],
            }
        } else if let Some(resolver) = resolver {
            resolver.resolve(binding.target_provider_ref.as_deref(), &target_ref)
        } else {
            ForeignTargetResolution {
                target_ref: target_ref.clone(),
                provider_ref: binding.target_provider_ref.clone(),
                revision: binding.target_revision.clone(),
                availability: TargetAvailability::Unknown,
                payload: None,
                notices: vec!["no foreign knowledge resolver configured".into()],
            }
        };
        Self {
            binding_ref: binding.binding_ref.clone(),
            ql_mef_ref: binding.ql_mef_ref.clone(),
            target_wiki_ref: binding.target_wiki_ref.clone(),
            target_frame_ref: binding.target_frame_ref.clone(),
            target_ref,
            target_provider_ref: binding
                .target_provider_ref
                .clone()
                .or(resolution.provider_ref.clone()),
            relation: binding.relation.clone(),
            operator_ref: binding.operator_ref.clone(),
            origin: binding.origin,
            scope_refs: binding.scope_refs.clone(),
            provenance: binding.provenance.clone(),
            ql_mef_revision: binding.ql_mef_revision.clone(),
            target_revision: binding
                .target_revision
                .clone()
                .or(resolution.revision.clone()),
            availability: resolution.availability,
            payload: if scope.allow_payload {
                resolution.payload
            } else {
                None
            },
            notices: resolution.notices,
            qualified_relation: true,
            semantic_equivalence_asserted: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestationsResponse {
    pub contract: String,
    pub meta_ref: String,
    pub manifestations: Vec<MetaManifestation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaContextResponse {
    pub contract: String,
    pub external_ref: String,
    pub mappings: Vec<MetaManifestation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetaRouteSurface {
    Meta,
    Binding,
    External,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaRouteStep {
    pub from_ref: String,
    pub from_surface: MetaRouteSurface,
    pub to_ref: String,
    pub to_surface: MetaRouteSurface,
    pub relation: String,
    pub origin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_revision: Option<String>,
    #[serde(default)]
    pub provenance: Vec<MetaProvenance>,
    pub qualified_relation: bool,
    pub semantic_equivalence_asserted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaRoute {
    pub start_ref: String,
    pub destination_ref: String,
    pub steps: Vec<MetaRouteStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<MetaManifestation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWikiTraversalRequest {
    pub start_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lens_ref: Option<String>,
    pub max_hops: usize,
}

impl CrossWikiTraversalRequest {
    fn validate(&self) -> Result<(), PortalError> {
        if self.start_ref.trim().is_empty() {
            return Err(PortalError::InvalidRequest("start_ref cannot be empty".into()));
        }
        if self.max_hops == 0 || self.max_hops > 32 {
            return Err(PortalError::InvalidRequest(
                "max_hops must be in the range 1..=32".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossWikiTraversalResponse {
    pub contract: String,
    pub start_ref: String,
    pub routes: Vec<MetaRoute>,
    #[serde(default)]
    pub notices: Vec<String>,
}

pub struct MetaPortal<'a> {
    projection: &'a MetaKnowledgeProjection,
    resolver: Option<&'a dyn ForeignKnowledgeResolver>,
}

impl<'a> MetaPortal<'a> {
    pub const fn new(
        projection: &'a MetaKnowledgeProjection,
        resolver: Option<&'a dyn ForeignKnowledgeResolver>,
    ) -> Self {
        Self {
            projection,
            resolver,
        }
    }

    pub fn manifestations(
        &self,
        meta_ref: &str,
        scope: &PortalScope,
    ) -> ManifestationsResponse {
        let manifestations = self
            .projection
            .meta_bindings
            .iter()
            .filter(|binding| binding.ql_mef_ref == meta_ref && scope.binding_visible(binding))
            .map(|binding| MetaManifestation::from_binding(binding, scope, self.resolver))
            .collect();
        ManifestationsResponse {
            contract: META_PORTAL_CONTRACT.into(),
            meta_ref: meta_ref.into(),
            manifestations,
        }
    }

    pub fn meta_context(
        &self,
        external_ref: &str,
        scope: &PortalScope,
    ) -> MetaContextResponse {
        let mappings = self
            .projection
            .meta_bindings
            .iter()
            .filter(|binding| {
                scope.binding_visible(binding)
                    && (binding.target_wiki_ref == external_ref
                        || binding.target_frame_ref.as_deref() == Some(external_ref))
            })
            .map(|binding| MetaManifestation::from_binding(binding, scope, self.resolver))
            .collect();
        MetaContextResponse {
            contract: META_PORTAL_CONTRACT.into(),
            external_ref: external_ref.into(),
            mappings,
        }
    }

    pub fn cross_wiki_traverse(
        &self,
        request: &CrossWikiTraversalRequest,
        scope: &PortalScope,
    ) -> Result<CrossWikiTraversalResponse, PortalError> {
        request.validate()?;
        let mut queue: VecDeque<(String, MetaRouteSurface, Vec<MetaRouteStep>)> = VecDeque::new();
        let start_surface = self.surface_of(&request.start_ref);
        queue.push_back((request.start_ref.clone(), start_surface, vec![]));
        let mut seen: BTreeSet<(String, MetaRouteSurface)> = BTreeSet::new();
        seen.insert((request.start_ref.clone(), start_surface));
        let mut routes = Vec::new();

        while let Some((current, current_surface, path)) = queue.pop_front() {
            if path.len() >= request.max_hops {
                continue;
            }
            for edge in self.portal_edges(&current, current_surface, request, scope) {
                let mut next_path = path.clone();
                next_path.push(edge.clone());
                if edge.to_surface == MetaRouteSurface::External
                    && edge.to_ref != request.start_ref
                {
                    let destination = self
                        .projection
                        .mappings_for_target(&edge.to_ref)
                        .into_iter()
                        .find(|binding| {
                            edge.binding_ref.as_deref() == Some(binding.binding_ref.as_str())
                        })
                        .map(|binding| MetaManifestation::from_binding(binding, scope, self.resolver));
                    routes.push(MetaRoute {
                        start_ref: request.start_ref.clone(),
                        destination_ref: edge.to_ref.clone(),
                        steps: next_path.clone(),
                        destination,
                    });
                }
                let key = (edge.to_ref.clone(), edge.to_surface);
                if seen.insert(key) {
                    queue.push_back((edge.to_ref.clone(), edge.to_surface, next_path));
                }
            }
        }

        routes.sort_by(|left, right| {
            left.steps
                .len()
                .cmp(&right.steps.len())
                .then_with(|| left.destination_ref.cmp(&right.destination_ref))
        });
        routes.dedup_by(|left, right| {
            left.destination_ref == right.destination_ref && left.steps == right.steps
        });
        Ok(CrossWikiTraversalResponse {
            contract: META_PORTAL_CONTRACT.into(),
            start_ref: request.start_ref.clone(),
            routes,
            notices: vec![
                "meta routes are qualified mapping paths; they do not assert semantic equivalence"
                    .into(),
            ],
        })
    }

    fn surface_of(&self, reference: &str) -> MetaRouteSurface {
        if self
            .projection
            .meta_bindings
            .iter()
            .any(|binding| binding.binding_ref == reference)
        {
            MetaRouteSurface::Binding
        } else if self
            .projection
            .objects
            .iter()
            .any(|object| object.canonical_ref == reference)
            || self
                .projection
                .meta_bindings
                .iter()
                .any(|binding| binding.ql_mef_ref == reference)
        {
            MetaRouteSurface::Meta
        } else {
            MetaRouteSurface::External
        }
    }

    fn portal_edges(
        &self,
        current: &str,
        surface: MetaRouteSurface,
        request: &CrossWikiTraversalRequest,
        scope: &PortalScope,
    ) -> Vec<MetaRouteStep> {
        let mut edges = Vec::new();
        match surface {
            MetaRouteSurface::Meta => {
                for binding in self.projection.meta_bindings.iter().filter(|binding| {
                    binding.ql_mef_ref == current
                        && scope.binding_visible(binding)
                        && binding_matches(binding, request)
                }) {
                    edges.push(step_meta_to_binding(binding));
                }
                for relation in self.projection.relations.iter().filter(|relation| {
                    relation.from_ref == current
                        && self.surface_of(&relation.to_ref) == MetaRouteSurface::Meta
                }) {
                    edges.push(MetaRouteStep {
                        from_ref: current.into(),
                        from_surface: MetaRouteSurface::Meta,
                        to_ref: relation.to_ref.clone(),
                        to_surface: MetaRouteSurface::Meta,
                        relation: relation.relation.clone(),
                        origin: relation.origin.clone(),
                        binding_ref: None,
                        provider_ref: None,
                        from_revision: self.meta_revision(current),
                        to_revision: self.meta_revision(&relation.to_ref),
                        provenance: vec![],
                        qualified_relation: true,
                        semantic_equivalence_asserted: false,
                    });
                }
            }
            MetaRouteSurface::Binding => {
                if let Some(binding) = self
                    .projection
                    .meta_bindings
                    .iter()
                    .find(|binding| binding.binding_ref == current && scope.binding_visible(binding))
                {
                    edges.push(step_binding_to_meta(binding));
                    edges.push(step_binding_to_external(binding));
                }
            }
            MetaRouteSurface::External => {
                for binding in self.projection.meta_bindings.iter().filter(|binding| {
                    scope.binding_visible(binding)
                        && binding_matches(binding, request)
                        && (binding.target_wiki_ref == current
                            || binding.target_frame_ref.as_deref() == Some(current))
                }) {
                    edges.push(step_external_to_binding(binding));
                }
            }
        }
        edges.sort_by(|left, right| {
            left.relation
                .cmp(&right.relation)
                .then_with(|| left.to_ref.cmp(&right.to_ref))
        });
        edges
    }

    fn meta_revision(&self, reference: &str) -> Option<String> {
        self.projection
            .objects
            .iter()
            .find(|object| object.canonical_ref == reference)
            .map(|object| object.revision.to_string())
    }
}

fn binding_matches(binding: &MetaBinding, request: &CrossWikiTraversalRequest) -> bool {
    request
        .relation
        .as_ref()
        .is_none_or(|relation| binding.relation == *relation)
        && request
            .operator_ref
            .as_ref()
            .is_none_or(|operator| binding.operator_ref.as_deref() == Some(operator.as_str()))
        && request.lens_ref.as_ref().is_none_or(|lens| {
            binding.operator_ref.as_deref() == Some(lens.as_str())
                || binding
                    .extensions
                    .get("lens_ref")
                    .and_then(Value::as_str)
                    == Some(lens.as_str())
        })
}

fn step_meta_to_binding(binding: &MetaBinding) -> MetaRouteStep {
    MetaRouteStep {
        from_ref: binding.ql_mef_ref.clone(),
        from_surface: MetaRouteSurface::Meta,
        to_ref: binding.binding_ref.clone(),
        to_surface: MetaRouteSurface::Binding,
        relation: "meta-binding".into(),
        origin: binding.origin.as_str().into(),
        binding_ref: Some(binding.binding_ref.clone()),
        provider_ref: binding.target_provider_ref.clone(),
        from_revision: binding.ql_mef_revision.clone(),
        to_revision: None,
        provenance: binding.provenance.clone(),
        qualified_relation: true,
        semantic_equivalence_asserted: false,
    }
}

fn step_binding_to_meta(binding: &MetaBinding) -> MetaRouteStep {
    MetaRouteStep {
        from_ref: binding.binding_ref.clone(),
        from_surface: MetaRouteSurface::Binding,
        to_ref: binding.ql_mef_ref.clone(),
        to_surface: MetaRouteSurface::Meta,
        relation: "meta-context".into(),
        origin: binding.origin.as_str().into(),
        binding_ref: Some(binding.binding_ref.clone()),
        provider_ref: binding.target_provider_ref.clone(),
        from_revision: None,
        to_revision: binding.ql_mef_revision.clone(),
        provenance: binding.provenance.clone(),
        qualified_relation: true,
        semantic_equivalence_asserted: false,
    }
}

fn step_binding_to_external(binding: &MetaBinding) -> MetaRouteStep {
    MetaRouteStep {
        from_ref: binding.binding_ref.clone(),
        from_surface: MetaRouteSurface::Binding,
        to_ref: binding.target_ref().into(),
        to_surface: MetaRouteSurface::External,
        relation: binding.relation.clone(),
        origin: binding.origin.as_str().into(),
        binding_ref: Some(binding.binding_ref.clone()),
        provider_ref: binding.target_provider_ref.clone(),
        from_revision: None,
        to_revision: binding.target_revision.clone(),
        provenance: binding.provenance.clone(),
        qualified_relation: true,
        semantic_equivalence_asserted: false,
    }
}

fn step_external_to_binding(binding: &MetaBinding) -> MetaRouteStep {
    MetaRouteStep {
        from_ref: binding.target_ref().into(),
        from_surface: MetaRouteSurface::External,
        to_ref: binding.binding_ref.clone(),
        to_surface: MetaRouteSurface::Binding,
        relation: "mapped-by".into(),
        origin: binding.origin.as_str().into(),
        binding_ref: Some(binding.binding_ref.clone()),
        provider_ref: binding.target_provider_ref.clone(),
        from_revision: binding.target_revision.clone(),
        to_revision: None,
        provenance: binding.provenance.clone(),
        qualified_relation: true,
        semantic_equivalence_asserted: false,
    }
}

#[derive(Debug, Clone, Default)]
pub struct StaticForeignResolver {
    entries: BTreeMap<(Option<String>, String), ForeignTargetResolution>,
}

impl StaticForeignResolver {
    pub fn insert(
        &mut self,
        provider_ref: Option<String>,
        target_ref: impl Into<String>,
        resolution: ForeignTargetResolution,
    ) {
        self.entries
            .insert((provider_ref, target_ref.into()), resolution);
    }
}

impl ForeignKnowledgeResolver for StaticForeignResolver {
    fn resolve(
        &self,
        provider_ref: Option<&str>,
        target_ref: &str,
    ) -> ForeignTargetResolution {
        self.entries
            .get(&(provider_ref.map(ToOwned::to_owned), target_ref.to_owned()))
            .cloned()
            .unwrap_or_else(|| ForeignTargetResolution {
                target_ref: target_ref.into(),
                provider_ref: provider_ref.map(ToOwned::to_owned),
                revision: None,
                availability: TargetAvailability::Unknown,
                payload: None,
                notices: vec!["target is not present in resolver fixture".into()],
            })
    }
}
