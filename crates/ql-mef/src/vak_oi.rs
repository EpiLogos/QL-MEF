use std::collections::BTreeSet;
use std::fmt;

use crate::vak::{
    SelfOtherForm, VakActionRelationKind, VakAddressHorizon, VakContextField, VakDivineAct,
    VakError, VakRef, VakRegistry, VakRelationOp, VakStanding,
};

pub const VAK_OI_PRIMITIVE_MATRIX_CONTRACT: &str = "vak-oi-primitive-relation-matrix-v1";
pub const VAK_ACTION_PROFILE_CONTRACT: &str = "vak-action-profile-v1";
pub const VAK_EXPRESSION_CONTRACT: &str = "vak-expression-v1";
pub const VAK_PATH_CONTRACT: &str = "vak-path-v1";
pub const VAK_RECOGNITION_CONTRACT: &str = "vak-recognition-v1";

/// Exact native owner revisions inspected for the first cross-product Action proving pair.
pub const FACTORY_ACTION_OWNER_REVISION: &str = "71287c179e7686cb37cb25267958f0c390a1ecb7";
pub const CENTRAL_ACTION_OWNER_REVISION: &str = "77625f40081a3e38503d732f44c607a94888d9d6";
pub const FACTORY_REQUEST_EVIDENCE_ACTION_REF: &str = "action:01ARZ3NDEKTSV4RRFFQ69G5FAP";
pub const CENTRAL_WORK_LIST_ACTION_REF: &str = "work.list";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VakOiPrimitiveKind {
    ResourceRef,
    SourceRef,
    Source,
    Ground,
    Canon,
    Reading,
    KnowledgeNode,
    KnowledgeRoute,
    World,
    Project,
    SessionSpace,
    SharedField,
    Journey,
    Run,
    Participant,
    Agent,
    Agency,
    AgentSet,
    AgentSession,
    Capability,
    Skill,
    UsageOverlay,
    SkillSet,
    Method,
    ContextSource,
    ContextResolution,
    Action,
    ActionRef,
    Surface,
    Projection,
    Invocation,
    Activity,
    ActuationStream,
    Result,
    Claim,
    Evidence,
    Return,
    Recognition,
}

impl VakOiPrimitiveKind {
    pub const ALL: [Self; 38] = [
        Self::ResourceRef,
        Self::SourceRef,
        Self::Source,
        Self::Ground,
        Self::Canon,
        Self::Reading,
        Self::KnowledgeNode,
        Self::KnowledgeRoute,
        Self::World,
        Self::Project,
        Self::SessionSpace,
        Self::SharedField,
        Self::Journey,
        Self::Run,
        Self::Participant,
        Self::Agent,
        Self::Agency,
        Self::AgentSet,
        Self::AgentSession,
        Self::Capability,
        Self::Skill,
        Self::UsageOverlay,
        Self::SkillSet,
        Self::Method,
        Self::ContextSource,
        Self::ContextResolution,
        Self::Action,
        Self::ActionRef,
        Self::Surface,
        Self::Projection,
        Self::Invocation,
        Self::Activity,
        Self::ActuationStream,
        Self::Result,
        Self::Claim,
        Self::Evidence,
        Self::Return,
        Self::Recognition,
    ];

    pub const fn schema_name(self) -> &'static str {
        match self {
            Self::ResourceRef => "ResourceRef",
            Self::SourceRef => "SourceRef",
            Self::Source => "Source",
            Self::Ground => "Ground",
            Self::Canon => "Canon",
            Self::Reading => "Reading",
            Self::KnowledgeNode => "KnowledgeNode",
            Self::KnowledgeRoute => "KnowledgeRoute",
            Self::World => "World",
            Self::Project => "Project",
            Self::SessionSpace => "SessionSpace",
            Self::SharedField => "SharedField",
            Self::Journey => "Journey",
            Self::Run => "Run",
            Self::Participant => "Participant",
            Self::Agent => "Agent",
            Self::Agency => "Agency",
            Self::AgentSet => "AgentSet",
            Self::AgentSession => "AgentSession",
            Self::Capability => "Capability",
            Self::Skill => "Skill",
            Self::UsageOverlay => "UsageOverlay",
            Self::SkillSet => "SkillSet",
            Self::Method => "Method",
            Self::ContextSource => "ContextSource",
            Self::ContextResolution => "ContextResolution",
            Self::Action => "Action",
            Self::ActionRef => "ActionRef",
            Self::Surface => "Surface",
            Self::Projection => "Projection",
            Self::Invocation => "Invocation",
            Self::Activity => "Activity",
            Self::ActuationStream => "ActuationStream",
            Self::Result => "Result",
            Self::Claim => "Claim",
            Self::Evidence => "Evidence",
            Self::Return => "Return",
            Self::Recognition => "Recognition",
        }
    }

    /// Authored-architecture placement into the M0-4/M0-5 field. This is a reading relation,
    /// never a claim that an O:I primitive *is* the Vāk family returned here.
    pub const fn default_field(self) -> VakContextField {
        match self {
            Self::ResourceRef | Self::SourceRef | Self::Source | Self::Ground | Self::Canon => {
                VakContextField::Bimba
            }
            Self::Reading | Self::KnowledgeNode | Self::KnowledgeRoute | Self::Result
            | Self::Claim | Self::Evidence => VakContextField::Pratibimba,
            Self::Surface | Self::Projection => VakContextField::Language,
            Self::World | Self::Project | Self::SessionSpace | Self::SharedField | Self::Journey
            | Self::Run | Self::Participant | Self::Agent | Self::Agency | Self::AgentSet
            | Self::AgentSession => VakContextField::World,
            Self::ContextSource | Self::ContextResolution => VakContextField::Particular,
            Self::Capability | Self::Skill | Self::UsageOverlay | Self::SkillSet | Self::Method
            | Self::Action | Self::ActionRef | Self::Invocation | Self::Activity
            | Self::ActuationStream | Self::Return | Self::Recognition => VakContextField::Techne,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakOiRelationKind {
    Describes,
    Qualifies,
    Contextualises,
    Reflects,
    Addresses,
    Relates,
    Expresses,
    BindsToAction,
    InvokesThrough,
    ManifestsAsActivity,
    ReturnsThrough,
    ParticipatesInMethod,
    ObservedInVakPath,
    LearnedAsFamiliarRoute,
    RecognisedInto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VakOiSemanticAltitude {
    Identity,
    CurrentState,
    Activity,
    SituatedUse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakOiPrimitiveRelation {
    pub native_ref: String,
    pub primitive: VakOiPrimitiveKind,
    pub native_owner: String,
    pub vak_refs: Vec<VakRef>,
    pub relation: VakOiRelationKind,
    pub operator: Option<VakRelationOp>,
    pub horizon: Option<VakAddressHorizon>,
    pub altitude: VakOiSemanticAltitude,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
    pub world_ref: Option<String>,
    pub project_ref: Option<String>,
    pub focus_ref: Option<String>,
}

impl VakOiPrimitiveRelation {
    pub fn validate(&self, registry: &VakRegistry) -> Result<(), VakOiError> {
        if self.native_ref.trim().is_empty() {
            return Err(VakOiError::Missing("native_ref"));
        }
        if self.native_owner.trim().is_empty() {
            return Err(VakOiError::Missing("native_owner"));
        }
        if self.vak_refs.is_empty() {
            return Err(VakOiError::Missing("vak_refs"));
        }
        if self.evidence.is_empty() {
            return Err(VakOiError::Missing("evidence"));
        }
        for vak_ref in &self.vak_refs {
            if registry.locate(vak_ref).is_none() {
                return Err(VakOiError::Vak(VakError::UnknownRef(vak_ref.to_string())));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakOiPrimitiveMatrixV1 {
    pub contract: &'static str,
    pub relations: Vec<VakOiPrimitiveRelation>,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
}

impl VakOiPrimitiveMatrixV1 {
    pub fn validate(&self, registry: &VakRegistry) -> Result<(), VakOiError> {
        if self.contract != VAK_OI_PRIMITIVE_MATRIX_CONTRACT {
            return Err(VakOiError::Contract(self.contract.to_owned()));
        }
        if self.evidence.is_empty() {
            return Err(VakOiError::Missing("matrix evidence"));
        }
        for relation in &self.relations {
            relation.validate(registry)?;
        }
        let covered = self
            .relations
            .iter()
            .map(|relation| relation.primitive)
            .collect::<BTreeSet<_>>();
        let missing = VakOiPrimitiveKind::ALL
            .into_iter()
            .filter(|kind| !covered.contains(kind))
            .map(VakOiPrimitiveKind::schema_name)
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            return Err(VakOiError::PrimitiveCoverage(missing));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VakExpressionSubject {
    Vak(VakRef),
    Native(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakExpressionV1 {
    pub contract: &'static str,
    pub operator: VakRelationOp,
    pub horizon: VakAddressHorizon,
    pub subjects: Vec<VakExpressionSubject>,
    pub relation_refs: Vec<VakRef>,
    pub complement_refs: Vec<VakRef>,
    pub world_ref: Option<String>,
    pub project_ref: Option<String>,
    pub focus_ref: Option<String>,
    pub expected_return: Option<String>,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
}

impl VakExpressionV1 {
    pub fn validate(&self, registry: &VakRegistry) -> Result<(), VakOiError> {
        if self.contract != VAK_EXPRESSION_CONTRACT {
            return Err(VakOiError::Contract(self.contract.to_owned()));
        }
        if self.subjects.is_empty() {
            return Err(VakOiError::Missing("expression subjects"));
        }
        if self.evidence.is_empty() {
            return Err(VakOiError::Missing("expression evidence"));
        }
        registry.bind_operator(self.operator).map_err(VakOiError::Vak)?;
        registry.bind_horizon(self.horizon).map_err(VakOiError::Vak)?;
        for vak_ref in self
            .subjects
            .iter()
            .filter_map(|subject| match subject {
                VakExpressionSubject::Vak(vak_ref) => Some(vak_ref),
                VakExpressionSubject::Native(_) => None,
            })
            .chain(self.relation_refs.iter())
            .chain(self.complement_refs.iter())
        {
            if registry.locate(vak_ref).is_none() {
                return Err(VakOiError::Vak(VakError::UnknownRef(vak_ref.to_string())));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakActionAffordance {
    pub operator: VakRelationOp,
    pub horizon: VakAddressHorizon,
    pub role: String,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakActionProfileV1 {
    pub contract: &'static str,
    pub action_ref: String,
    pub native_owner: String,
    pub primary_vak_ref: VakRef,
    pub related_vak_refs: Vec<VakRef>,
    pub relation_kinds: Vec<VakActionRelationKind>,
    pub divine_acts: Vec<VakDivineAct>,
    pub affordances: Vec<VakActionAffordance>,
    pub expected_return_relations: Vec<VakOiRelationKind>,
    pub standing: VakStanding,
    pub evidence: Vec<String>,
    pub binding_revision: String,
}

impl VakActionProfileV1 {
    pub fn validate(&self, registry: &VakRegistry) -> Result<(), VakOiError> {
        if self.contract != VAK_ACTION_PROFILE_CONTRACT {
            return Err(VakOiError::Contract(self.contract.to_owned()));
        }
        if self.action_ref.trim().is_empty() {
            return Err(VakOiError::Missing("action_ref"));
        }
        if self.native_owner.trim().is_empty() {
            return Err(VakOiError::Missing("native_owner"));
        }
        if self.binding_revision.trim().is_empty() {
            return Err(VakOiError::Missing("binding_revision"));
        }
        if self.evidence.is_empty() {
            return Err(VakOiError::Missing("action evidence"));
        }
        for vak_ref in std::iter::once(&self.primary_vak_ref).chain(self.related_vak_refs.iter()) {
            if registry.locate(vak_ref).is_none() {
                return Err(VakOiError::Vak(VakError::UnknownRef(vak_ref.to_string())));
            }
        }
        for affordance in &self.affordances {
            registry.bind_operator(affordance.operator).map_err(VakOiError::Vak)?;
            registry.bind_horizon(affordance.horizon).map_err(VakOiError::Vak)?;
        }
        for act in &self.divine_acts {
            registry.r_path(*act).map_err(VakOiError::Vak)?;
        }
        Ok(())
    }
}

pub fn factory_request_evidence_profile(
    registry: &VakRegistry,
) -> Result<VakActionProfileV1, VakOiError> {
    let profile = VakActionProfileV1 {
        contract: VAK_ACTION_PROFILE_CONTRACT,
        action_ref: FACTORY_REQUEST_EVIDENCE_ACTION_REF.into(),
        native_owner: "factory".into(),
        primary_vak_ref: SelfOtherForm::QueryOfOther.source_ref(),
        related_vak_refs: vec![
            VakContextField::Pratibimba.source_ref(),
            VakContextField::Techne.source_ref(),
        ],
        relation_kinds: vec![
            VakActionRelationKind::InvokesThrough,
            VakActionRelationKind::ReadsThrough,
        ],
        divine_acts: vec![VakDivineAct::Grace],
        affordances: vec![
            VakActionAffordance {
                operator: VakRelationOp::Potential,
                horizon: VakAddressHorizon::H2,
                role: "ask what additional evidence could bear on the candidate".into(),
                standing: VakStanding::AuthoredArchitecture,
            },
            VakActionAffordance {
                operator: VakRelationOp::Express,
                horizon: VakAddressHorizon::H5,
                role: "issue the authorised native Action and return the resulting request".into(),
                standing: VakStanding::Implementation,
            },
        ],
        expected_return_relations: vec![
            VakOiRelationKind::ManifestsAsActivity,
            VakOiRelationKind::ReturnsThrough,
            VakOiRelationKind::RecognisedInto,
        ],
        standing: VakStanding::Implementation,
        evidence: vec![
            format!(
                "EpiLogos/agent-system-design@{FACTORY_ACTION_OWNER_REVISION}:factory/src/build.rs"
            ),
            "FactoryActionExecutor authorises request-evidence and returns FactoryActionReceipt"
                .into(),
            "factory/tests/build_file_provider.rs executes and persists the returned HumanRequest"
                .into(),
        ],
        binding_revision: FACTORY_ACTION_OWNER_REVISION.into(),
    };
    profile.validate(registry)?;
    Ok(profile)
}

pub fn central_work_list_profile(
    registry: &VakRegistry,
) -> Result<VakActionProfileV1, VakOiError> {
    let profile = VakActionProfileV1 {
        contract: VAK_ACTION_PROFILE_CONTRACT,
        action_ref: CENTRAL_WORK_LIST_ACTION_REF.into(),
        native_owner: "central".into(),
        primary_vak_ref: VakContextField::World.source_ref(),
        related_vak_refs: vec![
            VakContextField::Bimba.source_ref(),
            VakContextField::Techne.source_ref(),
        ],
        relation_kinds: vec![
            VakActionRelationKind::ReadsThrough,
            VakActionRelationKind::Expresses,
        ],
        divine_acts: vec![VakDivineAct::Freedom],
        affordances: vec![
            VakActionAffordance {
                operator: VakRelationOp::Relate,
                horizon: VakAddressHorizon::H4,
                role: "resolve the current Central Work world through its native discovery Port"
                    .into(),
                standing: VakStanding::Implementation,
            },
            VakActionAffordance {
                operator: VakRelationOp::Express,
                horizon: VakAddressHorizon::H3,
                role: "return the discovered Work items and selected-connector diagnostics"
                    .into(),
                standing: VakStanding::Implementation,
            },
        ],
        expected_return_relations: vec![
            VakOiRelationKind::InvokesThrough,
            VakOiRelationKind::ReturnsThrough,
        ],
        standing: VakStanding::Implementation,
        evidence: vec![
            format!("EpiLogos/Central@{CENTRAL_ACTION_OWNER_REVISION}:ctrl/src/action.rs"),
            "Central ActionRegistry binds work.list to WorkDiscovery and structured ActionResult"
                .into(),
            "ctrl/tests/port_connector.rs executes work.list through the selected native Port"
                .into(),
        ],
        binding_revision: CENTRAL_ACTION_OWNER_REVISION.into(),
    };
    profile.validate(registry)?;
    Ok(profile)
}

/// Broad first relation matrix. Generic primitive relations are authored-architecture readings;
/// the two Action entries are pinned implementation facts from their native owners.
pub fn oi_reference_primitive_matrix(
    registry: &VakRegistry,
) -> Result<VakOiPrimitiveMatrixV1, VakOiError> {
    let mut relations = Vec::new();
    for primitive in VakOiPrimitiveKind::ALL {
        let field = primitive.default_field();
        relations.push(VakOiPrimitiveRelation {
            native_ref: format!("oi:primitive/{}", primitive.schema_name()),
            primitive,
            native_owner: primitive_owner(primitive).into(),
            vak_refs: vec![field.source_ref()],
            relation: VakOiRelationKind::Relates,
            operator: None,
            horizon: field.address_horizon(),
            altitude: primitive_altitude(primitive),
            standing: VakStanding::AuthoredArchitecture,
            evidence: vec![
                "QL-MEF#83 EPI-VAK-OI-RELATION-MATRIX-WAYFINDER Pass E".into(),
                "O:I current native-owner composition; Vāk relation does not replace native ownership"
                    .into(),
            ],
            world_ref: None,
            project_ref: None,
            focus_ref: None,
        });
    }

    let factory = factory_request_evidence_profile(registry)?;
    relations.push(VakOiPrimitiveRelation {
        native_ref: factory.action_ref.clone(),
        primitive: VakOiPrimitiveKind::ActionRef,
        native_owner: factory.native_owner.clone(),
        vak_refs: std::iter::once(factory.primary_vak_ref.clone())
            .chain(factory.related_vak_refs.clone())
            .collect(),
        relation: VakOiRelationKind::BindsToAction,
        operator: Some(VakRelationOp::Express),
        horizon: Some(VakAddressHorizon::H5),
        altitude: VakOiSemanticAltitude::SituatedUse,
        standing: VakStanding::Implementation,
        evidence: factory.evidence.clone(),
        world_ref: None,
        project_ref: None,
        focus_ref: None,
    });

    let central = central_work_list_profile(registry)?;
    relations.push(VakOiPrimitiveRelation {
        native_ref: central.action_ref.clone(),
        primitive: VakOiPrimitiveKind::ActionRef,
        native_owner: central.native_owner.clone(),
        vak_refs: std::iter::once(central.primary_vak_ref.clone())
            .chain(central.related_vak_refs.clone())
            .collect(),
        relation: VakOiRelationKind::BindsToAction,
        operator: Some(VakRelationOp::Relate),
        horizon: Some(VakAddressHorizon::H4),
        altitude: VakOiSemanticAltitude::SituatedUse,
        standing: VakStanding::Implementation,
        evidence: central.evidence.clone(),
        world_ref: None,
        project_ref: None,
        focus_ref: None,
    });

    let matrix = VakOiPrimitiveMatrixV1 {
        contract: VAK_OI_PRIMITIVE_MATRIX_CONTRACT,
        relations,
        standing: VakStanding::AuthoredArchitecture,
        evidence: vec![
            "QL-MEF#83 Pass E broad O:I primitive relation matrix".into(),
            format!("Factory Action owner pinned at {FACTORY_ACTION_OWNER_REVISION}"),
            format!("Central Action owner pinned at {CENTRAL_ACTION_OWNER_REVISION}"),
        ],
    };
    matrix.validate(registry)?;
    Ok(matrix)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakPathStepV1 {
    pub step_id: String,
    pub expression: VakExpressionV1,
    pub native_subject_refs: Vec<String>,
    pub method_ref: Option<String>,
    pub action_ref: Option<String>,
    pub invocation_ref: Option<String>,
    pub activity_ref: Option<String>,
    pub result_refs: Vec<String>,
    pub return_ref: Option<String>,
    pub source_surface: Option<String>,
    pub evidence_refs: Vec<String>,
    pub standing: VakStanding,
}

impl VakPathStepV1 {
    fn validate(&self, registry: &VakRegistry) -> Result<(), VakOiError> {
        if self.step_id.trim().is_empty() {
            return Err(VakOiError::Missing("path step id"));
        }
        if self.evidence_refs.is_empty() {
            return Err(VakOiError::Missing("path step evidence"));
        }
        self.expression.validate(registry)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakExecutionObservationV1 {
    pub observation_ref: String,
    pub owner_revision: String,
    pub evidence_run_ref: String,
    pub method_ref: String,
    pub resolve_expression: String,
    pub world_ref: Option<String>,
    pub project_ref: Option<String>,
    pub focus_ref: Option<String>,
    pub actor_ref: Option<String>,
    pub agency_ref: Option<String>,
    pub action_profile: VakActionProfileV1,
    pub steps: Vec<VakPathStepV1>,
    pub evidence_refs: Vec<String>,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakPathV1 {
    pub contract: &'static str,
    pub path_ref: String,
    pub observation_ref: String,
    pub method_ref: String,
    pub resolve_expression: String,
    pub world_ref: Option<String>,
    pub project_ref: Option<String>,
    pub focus_ref: Option<String>,
    pub actor_ref: Option<String>,
    pub agency_ref: Option<String>,
    pub action_profile: VakActionProfileV1,
    pub steps: Vec<VakPathStepV1>,
    pub evidence_refs: Vec<String>,
    pub standing: VakStanding,
}

pub fn reconstruct_observed_vak_path(
    registry: &VakRegistry,
    observation: VakExecutionObservationV1,
) -> Result<VakPathV1, VakOiError> {
    if observation.standing != VakStanding::Observed {
        return Err(VakOiError::Observation(
            "path reconstruction requires OBSERVED execution evidence".into(),
        ));
    }
    if observation.observation_ref.trim().is_empty()
        || observation.owner_revision.trim().is_empty()
        || observation.evidence_run_ref.trim().is_empty()
        || observation.method_ref.trim().is_empty()
        || observation.resolve_expression.trim().is_empty()
    {
        return Err(VakOiError::Observation(
            "observation identity, owner revision, run, Method and Resolve expression are required"
                .into(),
        ));
    }
    if observation.evidence_refs.is_empty() || observation.steps.is_empty() {
        return Err(VakOiError::Observation(
            "observed path requires returned evidence and at least one step".into(),
        ));
    }
    observation.action_profile.validate(registry)?;
    if observation.action_profile.binding_revision != observation.owner_revision {
        return Err(VakOiError::Observation(
            "Action profile revision must equal the observed native owner revision".into(),
        ));
    }

    let mut has_invocation = false;
    let mut has_activity = false;
    let mut has_return = false;
    for step in &observation.steps {
        step.validate(registry)?;
        if step.standing != VakStanding::Observed || step.expression.standing != VakStanding::Observed {
            return Err(VakOiError::Observation(
                "every reconstructed path step and expression must be OBSERVED".into(),
            ));
        }
        has_invocation |= step.invocation_ref.is_some();
        has_activity |= step.activity_ref.is_some();
        has_return |= step.return_ref.is_some() || !step.result_refs.is_empty();
    }
    if !(has_invocation && has_activity && has_return) {
        return Err(VakOiError::Observation(
            "observed path must contain Invocation, Activity and Result/Return evidence".into(),
        ));
    }

    Ok(VakPathV1 {
        contract: VAK_PATH_CONTRACT,
        path_ref: format!("vak-path:{}", observation.observation_ref),
        observation_ref: observation.observation_ref,
        method_ref: observation.method_ref,
        resolve_expression: observation.resolve_expression,
        world_ref: observation.world_ref,
        project_ref: observation.project_ref,
        focus_ref: observation.focus_ref,
        actor_ref: observation.actor_ref,
        agency_ref: observation.agency_ref,
        action_profile: observation.action_profile,
        steps: observation.steps,
        evidence_refs: observation
            .evidence_refs
            .into_iter()
            .chain(std::iter::once(observation.evidence_run_ref))
            .collect(),
        standing: VakStanding::Observed,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakRecognitionProposal {
    pub target_ref: String,
    pub proposal: String,
    pub standing: VakStanding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakRecognitionV1 {
    pub contract: &'static str,
    pub recognition_ref: String,
    pub path_ref: String,
    pub changed_fields: Vec<VakContextField>,
    pub recognised_vak_refs: Vec<VakRef>,
    pub returned_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub proposals: Vec<VakRecognitionProposal>,
    pub standing: VakStanding,
}

pub fn recognise_vak_return(
    registry: &VakRegistry,
    path: &VakPathV1,
    recognition_ref: impl Into<String>,
    evidence_refs: Vec<String>,
) -> Result<VakRecognitionV1, VakOiError> {
    if path.contract != VAK_PATH_CONTRACT || path.standing != VakStanding::Observed {
        return Err(VakOiError::Observation(
            "Recognition consumes an OBSERVED vak-path-v1".into(),
        ));
    }
    if evidence_refs.is_empty() {
        return Err(VakOiError::Missing("Recognition evidence"));
    }

    let mut changed_fields = BTreeSet::new();
    let mut recognised_vak_refs = BTreeSet::new();
    let mut returned_refs = BTreeSet::new();
    for step in &path.steps {
        step.expression.validate(registry)?;
        changed_fields.insert(field_for_horizon(step.expression.horizon));
        for vak_ref in step
            .expression
            .relation_refs
            .iter()
            .chain(step.expression.complement_refs.iter())
            .chain(step.expression.subjects.iter().filter_map(|subject| match subject {
                VakExpressionSubject::Vak(vak_ref) => Some(vak_ref),
                VakExpressionSubject::Native(_) => None,
            }))
        {
            recognised_vak_refs.insert(vak_ref.clone());
        }
        if let Some(return_ref) = &step.return_ref {
            returned_refs.insert(return_ref.clone());
        }
        returned_refs.extend(step.result_refs.iter().cloned());
    }

    Ok(VakRecognitionV1 {
        contract: VAK_RECOGNITION_CONTRACT,
        recognition_ref: recognition_ref.into(),
        path_ref: path.path_ref.clone(),
        changed_fields: changed_fields.into_iter().collect(),
        recognised_vak_refs: recognised_vak_refs.into_iter().collect(),
        returned_refs: returned_refs.into_iter().collect(),
        evidence_refs,
        proposals: Vec::new(),
        // The path occurrence is observed; the semantic integration across it is a derived reading.
        standing: VakStanding::Derived,
    })
}

fn field_for_horizon(horizon: VakAddressHorizon) -> VakContextField {
    match horizon {
        VakAddressHorizon::H0 => VakContextField::PrimordialMatrix,
        VakAddressHorizon::H1 => VakContextField::Bimba,
        VakAddressHorizon::H2 => VakContextField::Pratibimba,
        VakAddressHorizon::H3 => VakContextField::Language,
        VakAddressHorizon::H4 => VakContextField::World,
        VakAddressHorizon::H5 => VakContextField::Techne,
    }
}

fn primitive_owner(primitive: VakOiPrimitiveKind) -> &'static str {
    match primitive {
        VakOiPrimitiveKind::ResourceRef | VakOiPrimitiveKind::SourceRef => "O:I",
        VakOiPrimitiveKind::Source | VakOiPrimitiveKind::Ground | VakOiPrimitiveKind::Canon
        | VakOiPrimitiveKind::World | VakOiPrimitiveKind::Project => "Central",
        VakOiPrimitiveKind::Reading | VakOiPrimitiveKind::KnowledgeNode
        | VakOiPrimitiveKind::KnowledgeRoute | VakOiPrimitiveKind::SessionSpace
        | VakOiPrimitiveKind::AgentSession | VakOiPrimitiveKind::Capability
        | VakOiPrimitiveKind::Skill | VakOiPrimitiveKind::UsageOverlay
        | VakOiPrimitiveKind::SkillSet | VakOiPrimitiveKind::Method
        | VakOiPrimitiveKind::ContextSource | VakOiPrimitiveKind::ContextResolution
        | VakOiPrimitiveKind::Surface | VakOiPrimitiveKind::Projection => "AIKit",
        VakOiPrimitiveKind::SharedField | VakOiPrimitiveKind::Participant => "O:I",
        VakOiPrimitiveKind::Journey | VakOiPrimitiveKind::Run | VakOiPrimitiveKind::Result
        | VakOiPrimitiveKind::Claim | VakOiPrimitiveKind::Evidence
        | VakOiPrimitiveKind::Recognition => "Factory",
        VakOiPrimitiveKind::Agent | VakOiPrimitiveKind::Agency | VakOiPrimitiveKind::AgentSet
        | VakOiPrimitiveKind::Invocation | VakOiPrimitiveKind::Activity
        | VakOiPrimitiveKind::ActuationStream | VakOiPrimitiveKind::Return => "Actuation",
        VakOiPrimitiveKind::Action | VakOiPrimitiveKind::ActionRef => "native product owner",
    }
}

fn primitive_altitude(primitive: VakOiPrimitiveKind) -> VakOiSemanticAltitude {
    match primitive {
        VakOiPrimitiveKind::Invocation | VakOiPrimitiveKind::Activity
        | VakOiPrimitiveKind::ActuationStream => VakOiSemanticAltitude::Activity,
        VakOiPrimitiveKind::ContextResolution | VakOiPrimitiveKind::Method
        | VakOiPrimitiveKind::Action | VakOiPrimitiveKind::ActionRef
        | VakOiPrimitiveKind::Surface | VakOiPrimitiveKind::Projection
        | VakOiPrimitiveKind::Return | VakOiPrimitiveKind::Recognition => {
            VakOiSemanticAltitude::SituatedUse
        }
        VakOiPrimitiveKind::Result | VakOiPrimitiveKind::Claim | VakOiPrimitiveKind::Evidence => {
            VakOiSemanticAltitude::CurrentState
        }
        _ => VakOiSemanticAltitude::Identity,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VakOiError {
    Vak(VakError),
    Missing(&'static str),
    Contract(String),
    PrimitiveCoverage(Vec<&'static str>),
    Observation(String),
}

impl fmt::Display for VakOiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vak(error) => error.fmt(formatter),
            Self::Missing(field) => write!(formatter, "missing required Vāk/O:I field: {field}"),
            Self::Contract(contract) => write!(formatter, "unexpected Vāk/O:I contract: {contract}"),
            Self::PrimitiveCoverage(missing) => {
                write!(formatter, "primitive matrix is missing: {}", missing.join(", "))
            }
            Self::Observation(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for VakOiError {}

impl From<VakError> for VakOiError {
    fn from(error: VakError) -> Self {
        Self::Vak(error)
    }
}
