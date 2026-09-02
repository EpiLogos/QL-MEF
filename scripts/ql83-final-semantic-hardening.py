from pathlib import Path

vak = Path('crates/ql-mef/src/vak_oi.rs')
source = vak.read_text()

source = source.replace(
    '    VakError, VakRef, VakRegistry, VakRelationOp, VakStanding,\n',
    '    VakError, VakNeighbourhood, VakPraxisAspect, VakRef, VakRegistry, VakRelationOp, VakStanding,\n',
    1,
)
source = source.replace(
    'pub const VAK_EXPRESSION_CONTRACT: &str = "vak-expression-v1";\n',
    'pub const VAK_EXPRESSION_READING_CONTRACT: &str = "vak-expression-reading-v1";\n'
    'pub const AIKIT_OPERATIVE_SYNTAX_VERSION: &str = "aikit.operative-resolve/v1";\n'
    'pub const AIKIT_OPERATIVE_OWNER_REVISION: &str = "4e35f499c50b987551ab124b4432757973e823ae";\n',
    1,
)
source = source.replace('VakExpressionV1', 'VakExpressionReadingV1')
source = source.replace('VAK_EXPRESSION_CONTRACT', 'VAK_EXPRESSION_READING_CONTRACT')

anchor = '''#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VakExpressionSubject {
    Vak(VakRef),
    Native(String),
}

'''
addition = '''#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakGeneralExpressionEvidence {
    pub syntax_version: String,
    pub owner_revision: String,
    pub resolve_path_identity: String,
    pub rendered: String,
    pub full_vak_rendering: String,
    pub evidence: Vec<String>,
}

impl VakGeneralExpressionEvidence {
    pub fn validate(&self) -> Result<(), VakOiError> {
        if self.syntax_version != AIKIT_OPERATIVE_SYNTAX_VERSION {
            return Err(VakOiError::Contract(self.syntax_version.clone()));
        }
        if self.owner_revision != AIKIT_OPERATIVE_OWNER_REVISION {
            return Err(VakOiError::Observation(
                "general expression owner revision does not match the inspected AIKit syntax owner"
                    .into(),
            ));
        }
        if self.resolve_path_identity.trim().is_empty()
            || self.rendered.trim().is_empty()
            || self.full_vak_rendering.trim().is_empty()
            || self.evidence.is_empty()
        {
            return Err(VakOiError::Missing("general ResolveExpression evidence"));
        }
        Ok(())
    }
}

'''
if source.count(anchor) != 1:
    raise SystemExit('expression subject anchor drifted')
source = source.replace(anchor, anchor + addition, 1)

source = source.replace(
    '''pub struct VakExpressionReadingV1 {
    pub contract: &'static str,
''',
    '''/// A full-Vāk reading attached to one step of the AIKit-owned ResolveExpression/ResolvePath.
/// This is deliberately not a parser or competing AST.
pub struct VakExpressionReadingV1 {
    pub contract: &'static str,
''',
    1,
)

old = '''pub struct VakActionProfileV1 {
    pub contract: &'static str,
    pub action_ref: String,
    pub native_owner: String,
    pub primary_vak_ref: VakRef,
'''
new = '''pub struct VakActionProfileV1 {
    pub contract: &'static str,
    pub action_ref: String,
    pub native_owner: String,
    pub native_handler_ref: String,
    pub native_result_lineage: String,
    pub primary_vak_ref: VakRef,
'''
if source.count(old) != 1:
    raise SystemExit('action profile head anchor drifted')
source = source.replace(old, new, 1)
source = source.replace(
    '    pub divine_acts: Vec<VakDivineAct>,\n    pub affordances: Vec<VakActionAffordance>,\n',
    '    pub divine_acts: Vec<VakDivineAct>,\n    pub praxis_aspects: Vec<VakPraxisAspect>,\n    pub affordances: Vec<VakActionAffordance>,\n',
    1,
)
source = source.replace(
    '''        if self.native_owner.trim().is_empty() {
            return Err(VakOiError::Missing("native_owner"));
        }
''',
    '''        if self.native_owner.trim().is_empty() {
            return Err(VakOiError::Missing("native_owner"));
        }
        if self.native_handler_ref.trim().is_empty() || self.native_result_lineage.trim().is_empty() {
            return Err(VakOiError::Missing("native Action handler/result lineage"));
        }
''',
    1,
)
source = source.replace(
    '''        for act in &self.divine_acts {
            registry.r_path(*act).map_err(VakOiError::Vak)?;
        }
''',
    '''        for act in &self.divine_acts {
            registry.r_path(*act).map_err(VakOiError::Vak)?;
        }
        for aspect in &self.praxis_aspects {
            let reading = registry.praxis_reading(*aspect);
            if reading.source_refs.is_empty() {
                return Err(VakOiError::Observation(
                    "Action praxis aspect has no source-backed Vāk reading".into(),
                ));
            }
        }
''',
    1,
)

source = source.replace(
    '''        action_ref: FACTORY_REQUEST_EVIDENCE_ACTION_REF.into(),
        native_owner: "factory".into(),
        primary_vak_ref:''',
    '''        action_ref: FACTORY_REQUEST_EVIDENCE_ACTION_REF.into(),
        native_owner: "factory".into(),
        native_handler_ref: "factory::FactoryActionExecutor::execute".into(),
        native_result_lineage: "FactoryActionReceipt -> FactoryBuildView@next_revision -> HumanRequest".into(),
        primary_vak_ref:''',
    1,
)
source = source.replace(
    '        divine_acts: vec![VakDivineAct::Grace],\n        affordances:',
    '        divine_acts: vec![VakDivineAct::Grace],\n        praxis_aspects: vec![\n            VakPraxisAspect::WillAgency,\n            VakPraxisAspect::KnowledgeVimarsa,\n            VakPraxisAspect::ActionSvatantrya,\n        ],\n        affordances:',
    1,
)
source = source.replace(
    '''        action_ref: CENTRAL_WORK_LIST_ACTION_REF.into(),
        native_owner: "central".into(),
        primary_vak_ref:''',
    '''        action_ref: CENTRAL_WORK_LIST_ACTION_REF.into(),
        native_owner: "central".into(),
        native_handler_ref: "central::ActionRegistry::execute/work.list -> WorkDiscovery::list".into(),
        native_result_lineage: "ActionResult::Success(work.list) -> WorkItem list + selected connector diagnostics".into(),
        primary_vak_ref:''',
    1,
)
source = source.replace(
    '        divine_acts: vec![VakDivineAct::Freedom],\n        affordances:',
    '        divine_acts: vec![VakDivineAct::Freedom],\n        praxis_aspects: vec![VakPraxisAspect::KnowledgeVimarsa, VakPraxisAspect::ActionSvatantrya],\n        affordances:',
    1,
)

source = source.replace(
    '    pub method_ref: String,\n    pub resolve_expression: String,\n    pub world_ref:',
    '    pub method_ref: String,\n    pub general_expression: VakGeneralExpressionEvidence,\n    pub world_ref:',
    2,
)
source = source.replace(
    '        || observation.method_ref.trim().is_empty()\n        || observation.resolve_expression.trim().is_empty()\n',
    '        || observation.method_ref.trim().is_empty()\n',
    1,
)
source = source.replace(
    '    observation.action_profile.validate(registry)?;\n',
    '    observation.action_profile.validate(registry)?;\n    observation.general_expression.validate()?;\n',
    1,
)
source = source.replace(
    '        method_ref: observation.method_ref,\n        resolve_expression: observation.resolve_expression,\n',
    '        method_ref: observation.method_ref,\n        general_expression: observation.general_expression,\n',
    1,
)

anchor = '''#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakRecognitionProposal {
'''
addition = '''#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VakPraxisInstantiationV1 {
    pub aspect: VakPraxisAspect,
    pub source_refs: Vec<VakRef>,
    pub method_ref: String,
    pub action_ref: String,
    pub actor_ref: Option<String>,
    pub agency_ref: Option<String>,
    pub activity_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    /// Source meaning + observed praxis yields a derived situated reading.
    pub standing: VakStanding,
}

'''
if source.count(anchor) != 1:
    raise SystemExit('recognition proposal anchor drifted')
source = source.replace(anchor, addition + anchor, 1)
source = source.replace(
    '    pub evidence_refs: Vec<String>,\n    pub proposals: Vec<VakRecognitionProposal>,\n',
    '    pub evidence_refs: Vec<String>,\n    pub praxis: Vec<VakPraxisInstantiationV1>,\n    pub proposals: Vec<VakRecognitionProposal>,\n',
    1,
)

old = '''    Ok(VakRecognitionV1 {
        contract: VAK_RECOGNITION_CONTRACT,
        recognition_ref: recognition_ref.into(),
        path_ref: path.path_ref.clone(),
        changed_fields,
        recognised_vak_refs: recognised_vak_refs.into_iter().collect(),
        returned_refs: returned_refs.into_iter().collect(),
        evidence_refs,
        proposals: Vec::new(),
'''
new = '''    let activity_refs = path
        .steps
        .iter()
        .filter_map(|step| step.activity_ref.clone())
        .collect::<Vec<_>>();
    let praxis = path
        .action_profile
        .praxis_aspects
        .iter()
        .map(|aspect| {
            let reading = registry.praxis_reading(*aspect);
            VakPraxisInstantiationV1 {
                aspect: *aspect,
                source_refs: reading.source_refs,
                method_ref: path.method_ref.clone(),
                action_ref: path.action_profile.action_ref.clone(),
                actor_ref: path.actor_ref.clone(),
                agency_ref: path.agency_ref.clone(),
                activity_refs: activity_refs.clone(),
                evidence_refs: path.evidence_refs.clone(),
                standing: VakStanding::Derived,
            }
        })
        .collect();

    Ok(VakRecognitionV1 {
        contract: VAK_RECOGNITION_CONTRACT,
        recognition_ref: recognition_ref.into(),
        path_ref: path.path_ref.clone(),
        changed_fields,
        recognised_vak_refs: recognised_vak_refs.into_iter().collect(),
        returned_refs: returned_refs.into_iter().collect(),
        evidence_refs,
        praxis,
        proposals: Vec::new(),
'''
if source.count(old) != 1:
    raise SystemExit('recognition construction anchor drifted')
source = source.replace(old, new, 1)

anchor = 'fn field_for_horizon(horizon: VakAddressHorizon) -> VakContextField {'
addition = '''impl VakRecognitionV1 {
    /// Return the exact source neighbourhoods through which this returned actuality can be read.
    pub fn vak_neighbourhoods(
        &self,
        registry: &VakRegistry,
        depth: usize,
    ) -> Result<Vec<VakNeighbourhood>, VakOiError> {
        self.recognised_vak_refs
            .iter()
            .map(|reference| registry.neighbourhood(reference, depth).map_err(VakOiError::Vak))
            .collect()
    }
}

'''
if source.count(anchor) != 1:
    raise SystemExit('field_for_horizon anchor drifted')
source = source.replace(anchor, addition + anchor, 1)

vak.write_text(source)

lib = Path('crates/ql-mef/src/lib.rs')
source = lib.read_text()
source = source.replace('VAK_EXPRESSION_CONTRACT', 'VAK_EXPRESSION_READING_CONTRACT')
source = source.replace('VakExpressionV1', 'VakExpressionReadingV1')
source = source.replace(
    '    CENTRAL_ACTION_OWNER_REVISION, CENTRAL_WORK_LIST_ACTION_REF, FACTORY_ACTION_OWNER_REVISION,\n',
    '    AIKIT_OPERATIVE_OWNER_REVISION, AIKIT_OPERATIVE_SYNTAX_VERSION, CENTRAL_ACTION_OWNER_REVISION,\n    CENTRAL_WORK_LIST_ACTION_REF, FACTORY_ACTION_OWNER_REVISION,\n',
    1,
)
source = source.replace(
    '    VakActionAffordance, VakActionProfileV1, VakExecutionObservationV1, VakExpressionSubject,\n',
    '    VakActionAffordance, VakActionProfileV1, VakExecutionObservationV1, VakExpressionSubject,\n    VakGeneralExpressionEvidence,\n',
    1,
)
source = source.replace(
    '    VakRecognitionProposal, VakRecognitionV1, central_work_list_profile,\n',
    '    VakPraxisInstantiationV1, VakRecognitionProposal, VakRecognitionV1, central_work_list_profile,\n',
    1,
)
lib.write_text(source)

tests = Path('crates/ql-mef/tests/vak_oi_runtime.rs')
source = tests.read_text()
source = source.replace('VAK_EXPRESSION_CONTRACT', 'VAK_EXPRESSION_READING_CONTRACT')
source = source.replace('VakExpressionV1', 'VakExpressionReadingV1')
source = source.replace(
    '    VakAddressHorizon, VakContextField, VakExecutionObservationV1, VakExpressionSubject,\n',
    '    AIKIT_OPERATIVE_OWNER_REVISION, AIKIT_OPERATIVE_SYNTAX_VERSION, VakAddressHorizon,\n    VakContextField, VakExecutionObservationV1, VakExpressionSubject, VakGeneralExpressionEvidence,\n',
    1,
)

source = source.replace(
    '        resolve_expression: "@2 candidate / @5 request-evidence".into(),\n',
    '''        general_expression: VakGeneralExpressionEvidence {
            syntax_version: AIKIT_OPERATIVE_SYNTAX_VERSION.into(),
            owner_revision: AIKIT_OPERATIVE_OWNER_REVISION.into(),
            resolve_path_identity: "resolve-path:87fc6f42f239f59b".into(),
            rendered: "@2 candidate / @5 request-evidence".into(),
            full_vak_rendering: "@2 X# candidate / @5 R# request-evidence".into(),
            evidence: vec!["fixture:not-observed".into()],
        },
''',
    1,
)
source = source.replace(
    '            resolve_expression: "@2 candidate / @5 request-evidence".into(),\n',
    '''            general_expression: VakGeneralExpressionEvidence {
                syntax_version: AIKIT_OPERATIVE_SYNTAX_VERSION.into(),
                owner_revision: AIKIT_OPERATIVE_OWNER_REVISION.into(),
                resolve_path_identity: "resolve-path:87fc6f42f239f59b".into(),
                rendered: "@2 candidate / @5 request-evidence".into(),
                full_vak_rendering: "@2 X# candidate / @5 R# request-evidence".into(),
                evidence: vec![run_ref.clone()],
            },
''',
    1,
)
source = source.replace(
    '            resolve_expression: "@4 Central/Work x @3 work.list".into(),\n',
    '''            general_expression: VakGeneralExpressionEvidence {
                syntax_version: AIKIT_OPERATIVE_SYNTAX_VERSION.into(),
                owner_revision: AIKIT_OPERATIVE_OWNER_REVISION.into(),
                resolve_path_identity: "resolve-path:a8d70c5d3fef7c90".into(),
                rendered: "@4 Central/Work x @3 work.list".into(),
                full_vak_rendering: "@4 M# Central/Work x @3 N# work.list".into(),
                evidence: vec![run_ref.clone()],
            },
''',
    1,
)

source = source.replace(
    '    assert!(!factory.affordances.is_empty());\n',
    '''    assert!(!factory.affordances.is_empty());
    assert!(!factory.native_handler_ref.is_empty());
    assert!(!factory.native_result_lineage.is_empty());
    assert_eq!(factory.praxis_aspects.len(), 3);
''',
    1,
)
source = source.replace(
    '    assert!(!central.affordances.is_empty());\n',
    '''    assert!(!central.affordances.is_empty());
    assert!(!central.native_handler_ref.is_empty());
    assert!(!central.native_result_lineage.is_empty());
    assert_eq!(central.praxis_aspects.len(), 2);
''',
    1,
)
source = source.replace(
    '    assert!(recognition.proposals.is_empty());\n',
    '''    assert!(recognition.proposals.is_empty());
    assert_eq!(recognition.praxis.len(), 3);
    assert!(recognition.praxis.iter().all(|praxis| {
        praxis.standing == VakStanding::Derived
            && !praxis.source_refs.is_empty()
            && !praxis.activity_refs.is_empty()
            && praxis.method_ref == "method:ql83/factory-request-evidence-conformance"
    }));
    assert!(recognition.vak_neighbourhoods(&registry, 1).unwrap().iter().all(|field| !field.entries.is_empty()));
''',
    1,
)
source = source.replace(
    '    assert_eq!(central_recognition.standing, VakStanding::Derived);\n',
    '''    assert_eq!(central_recognition.standing, VakStanding::Derived);
    assert_eq!(central_recognition.praxis.len(), 2);
    assert!(central_recognition.praxis.iter().all(|praxis| {
        praxis.standing == VakStanding::Derived && !praxis.activity_refs.is_empty()
    }));
''',
    1,
)
tests.write_text(source)
