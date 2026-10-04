//! Source compilation for one actual selected native Scene, over the canonical
//! VakComposition evaluator. A source result is neither a live grant nor an ACK.
//! The private existing FieldHost supplies the real basis/read/timing; C28 and
//! the Expression owner independently re-attest the full current selected source.
use crate::m_tree::{MRegistry, native_current_m_registry};
use crate::procedural_composition::TimingBinding;
use crate::procedural_conduct::{NativePosition, NativeSceneSource};
use crate::procedural_manifestation::{
    NativeReading, NativeSubject, ReadingAvailability, SourceBasis, SubjectRole, fingerprint,
    nonempty, validate_material, validate_native_subject_basis,
};
use crate::vak_composition::VakComposition;
use crate::vak_profile::{CPrimeProfile, ThreadPlan};
use crate::vak_scope::{OperativeScopeCorrelation, OperativeScopeObservation};
use crate::vak_scope_wire::{OPERATIVE_CURRENTNESS_CONTRACT, OperativeScopeCurrentnessRequest};
use crate::vak_workflow_types::{AuthoredCPrime, AuthoredFrame, SourceRevision};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub const SOURCE_BOOTSTRAP_REQUEST: &str = "ql.native-procedural-source-bootstrap-request/v1";
pub const SOURCE_BOOTSTRAP: &str = "ql.native-procedural-source-bootstrap/v1";
const MAX_BOOTSTRAP_BYTES: usize = 8 * 1024 * 1024;

/// The native Expression source reader supplies the exact projected current
/// Scene and opaque source-read receipt. The private receiver substitutes these
/// from its actual Document; caller JSON cannot attest this reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeBootstrapSceneRead {
    pub native_owner: String,
    pub expression_ref: String,
    pub scene_ref: String,
    pub document_revision: u64,
    pub source_basis: SourceBasis,
    pub material_fingerprint: String,
    pub presentation: Value,
    pub locus: NativeReading,
    pub source_read_receipt_ref: String,
}
impl NativeBootstrapSceneRead {
    pub(crate) fn validate(&self) -> Result<(), String> {
        self.source_basis.validate()?;
        for (value, name) in [
            (&self.expression_ref, "native Expression"),
            (&self.scene_ref, "native Scene"),
            (&self.source_read_receipt_ref, "native source read receipt"),
            (&self.locus.reference, "native locus"),
            (&self.locus.revision, "native locus revision"),
        ] {
            nonempty(value, name)?;
        }
        if self.native_owner != "oi.expression"
            || self.document_revision == 0
            || !self
                .scene_ref
                .starts_with(&format!("{}:scene:", self.expression_ref))
            || self.presentation["schema"] != "oi.journey-scene/v1"
            || self.presentation["scene"]["id"] != self.scene_ref
            || self.presentation["scene"].get("procedural").is_some()
            || self.locus.availability != ReadingAvailability::Available
            || fingerprint(&self.presentation)? != self.material_fingerprint
        {
            return Err(
                "bootstrap does not carry the exact current projected native Scene/locus".into(),
            );
        }
        validate_material(&self.presentation, 0)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeStageFrame {
    pub id: String,
    pub lens: String,
    pub basis: String,
    pub face: String,
    pub positions: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeStageCoordinate {
    pub position: u8,
    pub face: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeStageMember {
    pub subject_ref: String,
    pub coordinate: NativeStageCoordinate,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeStageReturn {
    pub from_ref: String,
    pub anchor_ref: String,
    pub ground_ref: String,
    pub face: String,
    pub kind: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeStageRelation {
    pub row: NativeStageCoordinate,
    pub column: NativeStageCoordinate,
    pub relation_ref: String,
    pub evidence: Vec<String>,
}
/// Explicit native structural authoring. No graph, profile reading, fabricated
/// currentness or inferred position can be supplied in place of these originals.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageSourceAuthorship {
    pub actor_ref: String,
    pub standing_ref: String,
    pub principal_role: SubjectRole,
    pub contributors: Vec<NativeSubject>,
    pub members: Vec<NativeStageMember>,
    pub source_returns: Vec<NativeStageReturn>,
    pub relations: Vec<NativeStageRelation>,
    pub category: String,
    pub ground_ref: String,
    pub ground_face: String,
    pub frame: NativeStageFrame,
    /// Existing native FullVakBinding JSON surface, parsed by the SAME canonical
    /// adapter; absence is explicit and does not synthesize a language reading.
    pub language: Option<Value>,
    pub profile: CPrimeProfile,
    pub resolve_path: String,
    pub context_resolution: String,
    pub authority: Option<String>,
    pub thread_plan: ThreadPlan,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSourceBootstrap {
    pub schema: String,
    pub scene: NativeBootstrapSceneRead,
    pub authorship: StageSourceAuthorship,
}

/// Non-deserializable current owner observation. Only the private SAME native
/// host creates this from its guarded read/source and genuine C lease.
pub(crate) struct NativeBootstrapObservation {
    pub position: NativePosition,
    pub timing: TimingBinding,
    pub field_source: Value,
    pub field_receipt: Value,
    /// SAME native PerformanceOwner timing pulse, absent on plain FIELD path.
    pub native_timing_pulse: Option<Value>,
    pub native_act_source: Value,
}

pub(crate) fn compile_native_source_bootstrap(
    input: &NativeSourceBootstrap,
    observed: &NativeBootstrapObservation,
) -> Result<Value, String> {
    let registry = native_current_m_registry();
    compile_source(registry, input, observed).map(|(result, _)| result)
}
/// Actual private caller pairs C/R constructor facts before the existing
/// graph compiler. Serialized contracts remain configuration on pure replay.
pub(crate) fn compile_native_source_bootstrap_registered(
    input: &NativeSourceBootstrap,
    observed: &NativeBootstrapObservation,
    contract: &crate::procedural_consumers::NativeConsumerContract,
) -> Result<Value, String> {
    contract.validate_source_read(&input.scene, &observed.position, &observed.timing)?;
    let mut result = compile_native_source_bootstrap(input, observed)?;
    result["consumer_contract"] = serde_json::to_value(contract).map_err(|e| e.to_string())?;
    Ok(result)
}
/// Compile structural Source authoring configuration only. A supplied
/// historical configuration is not a native Source capability. The private
/// continuation caller supplies it only through NativeDefinitionSourceOrigin
/// borrowed from the genuine installed owner, never from request JSON.
pub fn prepare_native_source_composition_configuration(
    input: &NativeSourceBootstrap,
    principal: &NativeSubject,
    semantic_origin: Option<&NativeSourceBootstrap>,
) -> Result<Value, String> {
    struct Budget(usize);
    impl std::io::Write for Budget {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 = self
                .0
                .checked_add(bytes.len())
                .filter(|n| *n <= MAX_BOOTSTRAP_BYTES)
                .ok_or_else(|| {
                    std::io::Error::other("source configuration aggregate exceeds its native bound")
                })?;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    serde_json::to_writer(Budget(0), &(input, principal, semantic_origin))
        .map_err(|e| e.to_string())?;
    if input.schema != SOURCE_BOOTSTRAP_REQUEST {
        return Err("source configuration exceeds its native bound or request schema".into());
    }
    input.scene.validate()?;
    validate_native_subject_basis(native_current_m_registry(), principal)?;
    if let Some(original) = semantic_origin {
        crate::procedural_conduct::definition::validate_continuation_scene_configuration(
            &original.scene,
            &input.scene,
        )?;
        if original.schema != input.schema
            || serde_json::to_value(&original.authorship).map_err(|e| e.to_string())?
                != serde_json::to_value(&input.authorship).map_err(|e| e.to_string())?
        {
            return Err(
                "source configuration changed immutable original structural authoring".into(),
            );
        }
    }
    let s = &input.scene;
    let a = &input.authorship;
    let recipe = SourceBasis {
        source_ref: "docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3"
            .into(),
        revision: fingerprint(&include_str!(
            "../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md"
        ))?,
    };
    let initial = format!(
        "{}:source:{}",
        s.expression_ref,
        fingerprint(&json!({"scene":s,"authorship":a}))?
    );
    let evidence_material = semantic_origin
        .map(|old| &old.scene.material_fingerprint)
        .unwrap_or(&s.material_fingerprint);
    let basis = |source: &SourceBasis| {
        json!({"caller":a.actor_ref,"source":source.source_ref,"revision":source.revision,
        "standing":a.standing_ref,"evidence":[s.source_basis.source_ref,evidence_material]})
    };
    let members=a.members.iter().map(|m|json!({"subjectRef":m.subject_ref,"position":m.coordinate.position,"face":m.coordinate.face})).collect::<Vec<_>>();
    let returns=a.source_returns.iter().map(|r|json!({"fromRef":r.from_ref,"anchorRef":r.anchor_ref,"groundRef":r.ground_ref,"face":r.face,"kind":r.kind})).collect::<Vec<_>>();
    let relations=a.relations.iter().map(|r|json!({"row":r.row,"column":r.column,"relationRef":r.relation_ref,"evidence":r.evidence})).collect::<Vec<_>>();
    Ok(json!({"contract":crate::vak_composition::CONTRACT,"steps":[
        {"op":"whole","useRef":initial,"wholeRef":s.expression_ref,"subjectRef":principal.subject_ref,"members":members,
         "sourceReturns":returns,"relations":relations,"category":a.category,"groundRef":a.ground_ref,"groundFace":a.ground_face,
         "frame":a.frame,"basis":basis(&s.source_basis),"language":a.language},
        {"op":"reframe","from":initial,"into":s.expression_ref,"frame":a.frame,"basis":basis(&recipe)}]}))
}
pub(crate) fn compile_native_source_bootstrap_registered_for_definition(
    input: &NativeSourceBootstrap,
    observed: &NativeBootstrapObservation,
    contract: &crate::procedural_consumers::NativeConsumerContract,
    semantic_origin: Option<
        &crate::procedural_conduct::definition::NativeDefinitionSourceOrigin<'_>,
    >,
) -> Result<Value, String> {
    contract.validate_source_read(&input.scene, &observed.position, &observed.timing)?;
    let (mut result, _) = compile_source_with_origin(
        native_current_m_registry(),
        input,
        observed,
        semantic_origin,
    )?;
    if let Some(origin) = semantic_origin {
        let d = origin.definition();
        let currentness: OperativeScopeCurrentnessRequest =
            serde_json::from_value(result["currentness"].clone()).map_err(|e| e.to_string())?;
        if result["authored_cprime"]
            != serde_json::to_value(&d.procedure.composition).map_err(|e| e.to_string())?
            || currentness.expected != d.currentness.expected
            || currentness.correlation != d.currentness.correlation
            || result["thread_plan"]
                != serde_json::to_value(&d.thread_plan).map_err(|e| e.to_string())?
        {
            return Err(
                "fresh owner Source changed the immutable installed semantic definition".into(),
            );
        }
        result["semantic_source_origin"] = json!({"schema":"ql.native-procedural-semantic-source-origin/v1",
            "procedure_ref":d.procedure.procedure_ref,"procedure_revision":d.procedure.revision,
            "original_scene":{"native_owner":origin.original().scene.native_owner,
                "expression_ref":origin.original().scene.expression_ref,"scene_ref":origin.original().scene.scene_ref,
                "document_revision":origin.original().scene.document_revision,"source_basis":origin.original().scene.source_basis,
                "locus":origin.original().scene.locus,"material_fingerprint":origin.original().scene.material_fingerprint,
                "source_read_receipt_ref":origin.original().scene.source_read_receipt_ref},
            "original_authorship_fingerprint":fingerprint(&origin.original().authorship)?,
            "original_authored_cprime":d.procedure.composition,
            "current_scene_material_fingerprint":input.scene.material_fingerprint,
            "current_document_revision":input.scene.document_revision,
            "standing":"original accepted structural authoring; current material separately read by genuine owner"});
    }
    result["consumer_contract"] = serde_json::to_value(contract).map_err(|e| e.to_string())?;
    Ok(result)
}
fn compile_source(
    registry: &MRegistry,
    input: &NativeSourceBootstrap,
    observed: &NativeBootstrapObservation,
) -> Result<(Value, VakComposition), String> {
    compile_source_with_origin(registry, input, observed, None)
}
fn compile_source_with_origin(
    registry: &MRegistry,
    input: &NativeSourceBootstrap,
    observed: &NativeBootstrapObservation,
    semantic_origin: Option<
        &crate::procedural_conduct::definition::NativeDefinitionSourceOrigin<'_>,
    >,
) -> Result<(Value, VakComposition), String> {
    if input.schema != SOURCE_BOOTSTRAP_REQUEST
        || serde_json::to_vec(input).map_err(|e| e.to_string())?.len() > MAX_BOOTSTRAP_BYTES
    {
        return Err("unsupported or oversized native source bootstrap".into());
    }
    input.scene.validate()?;
    let s = &input.scene;
    let a = &input.authorship;
    nonempty(&a.actor_ref, "source actor")?;
    nonempty(&a.standing_ref, "source standing")?;
    validate_bootstrap_timing(observed)?;
    // FIELD uses its genuine field lifetime. Performance v2 dates the separate
    // Management lifetime, so its original field Source remains explicit.
    let source_instance = match observed.timing.domain.as_str() {
        "native_field_samples" => observed.position.instance_ref.as_str(),
        "native_samples" => observed
            .native_timing_pulse
            .as_ref()
            .and_then(|pulse| {
                pulse["payload"]["timing_fact"]["source"]["identity"]["instance"].as_str()
            })
            .ok_or("bootstrap original performance source instance absent")?,
        _ => return Err("bootstrap native source domain is unpaired".into()),
    };
    if observed.position.subject_ref
        != observed.field_source["current_basis"]["input"]["m3"]["subject_ref"]
        || observed.field_source["schema"] != "ql.native-held-field-source/v1"
        || observed.field_source["instance_ref"] != source_instance
    {
        return Err(
            "bootstrap observation differs from actual guarded native field/source boundary".into(),
        );
    }
    let mut sources = vec![NativeReading {
        reference: format!("ql-mef:held-field-basis:{source_instance}"),
        revision: fingerprint(&observed.field_source["original_basis"])?,
        availability: ReadingAvailability::Available,
    }];
    if observed
        .position
        .subject_ref
        .starts_with("ql:m-coordinate:")
    {
        let m = registry.manifest();
        sources = vec![NativeReading {
            reference: format!("{}:{}", m.source_repository, m.source_dataset_tree),
            revision: m.source_snapshot_sha256.clone(),
            availability: ReadingAvailability::Available,
        }];
    }
    let principal = NativeSubject {
        subject_ref: observed.position.subject_ref.clone(),
        native_owner: "ql-mef".into(),
        presentation_role: a.principal_role,
        sources,
        readings: vec![],
        actions: vec![],
    };
    validate_native_subject_basis(registry, &principal)?;
    let mut subjects = BTreeSet::from([principal.subject_ref.as_str()]);
    if a.contributors.len() > 64
        || a.members.len() > 12
        || a.relations.len() > 144
        || a.source_returns.len() > 64
    {
        return Err("source bootstrap structural/participant budget exceeded".into());
    }
    for contributor in &a.contributors {
        validate_native_subject_basis(registry, contributor)?;
        if !subjects.insert(contributor.subject_ref.as_str()) {
            return Err("duplicate native source contributor".into());
        }
    }
    if a.members
        .iter()
        .any(|m| !subjects.contains(m.subject_ref.as_str()))
        || a.ground_ref != s.locus.reference
        || a.source_returns.iter().any(|r| {
            !subjects.contains(r.from_ref.as_str())
                || r.anchor_ref != s.expression_ref
                || r.ground_ref != s.locus.reference
        })
    {
        return Err("native structural authoring exceeds source participants/whole/locus".into());
    }
    // Exact sources consumed by production LibraryBuild; no alternate recipe
    // or profile hash. A reframe creates a new immutable native use and retains
    // the recipe's additional source basis through the existing native owner.
    let semantic_configuration = semantic_origin.map(|origin| origin.original());
    let composition =
        prepare_native_source_composition_configuration(input, &principal, semantic_configuration)?;
    let recipe = SourceBasis {
        source_ref: "docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md#2.3"
            .into(),
        revision: fingerprint(&include_str!(
            "../../../docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md"
        ))?,
    };
    let (_, graph) = crate::vak_composition_wire::compile_request(&composition)?;
    let correlation = OperativeScopeCorrelation {
        world_ref: s.expression_ref.clone(),
        // This is the exact native material source generation. DocumentCAS and
        // guarded field cursors remain separately re-attested; neither becomes
        // the semantic source generation merely because metadata is retained.
        world_generation: s.source_basis.revision.clone(),
        method_skill_ref: None,
    };
    let binding = graph
        .bind_operative_scope(&s.expression_ref, a.profile.clone(), correlation.clone())
        .map_err(|e| e.to_string())?;
    let request = OperativeScopeCurrentnessRequest {
        contract: OPERATIVE_CURRENTNESS_CONTRACT.into(),
        expected: binding.clone(),
        current_whole_ref: s.expression_ref.clone(),
        correlation,
    };
    let currentness = graph
        .observe_operative_scope_currentness(request.clone())
        .map_err(|e| e.to_string())?;
    if !matches!(
        &currentness.observation,
        OperativeScopeObservation::Current { .. }
    ) {
        return Err("fresh native source bootstrap did not reobserve as current".into());
    }
    let compiled = graph
        .compile_profile(&s.expression_ref, a.profile.clone())
        .map_err(|e| e.to_string())?;
    compiled
        .validate_plan(&a.thread_plan)
        .map_err(|e| e.to_string())?;
    let native_whole = graph.whole(&binding.whole_ref).map_err(|e| e.to_string())?;
    let mut thread_subjects = BTreeSet::from([native_whole.binding.subject_ref.as_str()]);
    thread_subjects.extend(
        native_whole
            .binding
            .member_bindings()
            .iter()
            .map(|m| m.subject_ref.as_str()),
    );
    validate_source_plan(
        &thread_subjects,
        &binding,
        &a.thread_plan,
        &recipe.source_ref,
        &s.expression_ref,
    )?;
    let frame = crate::ContextFrameId::ALL
        .into_iter()
        .find(|f| f.code() == a.frame.id)
        .ok_or("unknown native source ContextFrame")?;
    let authored = AuthoredCPrime {
        participation: a.profile.participation,
        content: a.profile.content,
        position: a.profile.position,
        frame: AuthoredFrame(frame),
        thread: a.profile.thread,
        sequence: a.profile.sequence,
        direction: a.profile.direction,
        actor: a.actor_ref.clone(),
        interpretation: SourceRevision {
            reference: binding.binding_ref.clone(),
            revision: binding.binding_revision.clone(),
        },
        whole: s.expression_ref.clone(),
        resolve_path: a.resolve_path.clone(),
        context_resolution: a.context_resolution.clone(),
        sources: binding
            .sources
            .iter()
            .map(|b| b.source_ref.clone())
            .collect(),
        authority: a.authority.clone(),
    };
    authored.validate().map_err(|e| e.to_string())?;
    let native_scene_source = NativeSceneSource {
        native_owner: s.native_owner.clone(),
        expression_ref: s.expression_ref.clone(),
        scene_ref: s.scene_ref.clone(),
        document_revision: s.document_revision,
        source_basis: s.source_basis.clone(),
        material_fingerprint: s.material_fingerprint.clone(),
        presentation: s.presentation.clone(),
        principal: principal.clone(),
        contributors: a.contributors.clone(),
        locus_ref: s.locus.reference.clone(),
        locus_revision: s.locus.revision.clone(),
    };
    native_scene_source.validate(registry)?;
    let projected = json!({"address":{"expression_ref":s.expression_ref,"scene_ref":s.scene_ref,"entity_ref":null,"component":"scene","constituent_ref":null,"property":null},
        "principal":principal,"contributors":a.contributors,"locus":s.locus,"tags":[]});
    let mut result = json!({"schema":SOURCE_BOOTSTRAP,"native_owner":"ql-mef","expression_ref":s.expression_ref,"scene_ref":s.scene_ref,
        "document_revision":s.document_revision,"registry_revision":registry.manifest().registry_revision,
        "source_read_receipt_ref":s.source_read_receipt_ref,"source_material_fingerprint":s.material_fingerprint,
        "native_scene_source":native_scene_source,"source_composition":composition,"authored_cprime":authored,
        "currentness":request,"currentness_observation":currentness,"thread_plan":a.thread_plan,"binding":projected,
        "source_basis":[recipe,s.source_basis],"native_position":observed.position,"timing":observed.timing,
        "native_field_source":observed.field_source,"native_field_receipt":observed.field_receipt,
        "native_act_source":observed.native_act_source,
        "standing":"actual native source compilation at guarded field/current selected Scene; Root must atomically re-attest original DocumentCAS and retain binding; no material/body/audio application ACK"});
    if let Some(pulse) = &observed.native_timing_pulse {
        result["native_timing_pulse"] = pulse.clone();
        result["standing"] = json!(
            "actual native source compilation at SAME prepared/sounding performance boundary/current selected Scene; Root must atomically re-attest original DocumentCAS and retain binding; no material/body/audio application ACK"
        );
    }
    Ok((result, graph))
}
fn validate_bootstrap_timing(observed: &NativeBootstrapObservation) -> Result<(), String> {
    let position = &observed.position;
    match observed.timing.domain.as_str() {
        "native_field_samples" if observed.native_timing_pulse.is_none() => {
            if observed.field_receipt["subject_ref"] != position.subject_ref
                || observed.field_receipt["event_ref"] != position.event_ref
                || observed.field_receipt["generation"] != position.generation
                || observed.field_receipt["samples_elapsed"] != position.samples_elapsed
                || observed.timing.requested_cursor != position.cursor()?
            {
                return Err("bootstrap differs from actual guarded FIELD boundary".into());
            }
        }
        "native_samples" if observed.field_receipt.is_null() => {
            let pulse = observed
                .native_timing_pulse
                .as_ref()
                .ok_or("bootstrap performance timing pulse absent")?;
            let fact = &pulse["payload"]["timing_fact"];
            let binding = &fact["binding"];
            if fact["schema"] == "ql.native-performance-timing-fact/v2" {
                validate_performance_v2_configuration(pulse, position, &observed.timing)?;
                if fact["moment"] != "boundary" {
                    return Err(
                        "Source bootstrap requires the actual native Boundary descriptor".into(),
                    );
                }
                return Ok(());
            }
            let cursor = |value: &Value| -> Result<u64, String> {
                let text = value
                    .as_str()
                    .ok_or("native timing cursor must retain exact decimal serialization")?;
                let parsed = text
                    .parse::<u64>()
                    .map_err(|_| "invalid native timing cursor".to_owned())?;
                if parsed.to_string() != text {
                    return Err("noncanonical native timing cursor".into());
                }
                Ok(parsed)
            };
            if pulse["schema"] != "ql.performance-worker-reply/v1"
                || pulse["accepted"] != true
                || fact["schema"] != "ql.native-performance-timing-fact/v1"
                || fact["moment"] != "boundary"
                || fact["native_position"]
                    != serde_json::to_value(position).map_err(|e| e.to_string())?
                || binding["owner_ref"] != observed.timing.owner_ref
                || binding["domain"] != observed.timing.domain
                || binding["epoch_ref"] != observed.timing.epoch_ref
                || binding["time_mapping_ref"]
                    != serde_json::to_value(&observed.timing.time_mapping_ref)
                        .map_err(|e| e.to_string())?
                || cursor(&binding["requested_cursor"])? != observed.timing.requested_cursor
                || cursor(&fact["admission_horizon"])? != observed.timing.requested_cursor
                || fact["committed_cursor"] != pulse["reading"]["samples_elapsed"]
                || fact["committed_cursor"] != pulse["reading"]["physical"]["samples_elapsed"]
                || fact["committed_cursor"] != position.samples_elapsed
                || pulse["reading"]["scope"]["instance_ref"] != position.instance_ref
                || pulse["reading"]["scope"]["event_ref"] != position.event_ref
                || pulse["reading"]["scope"]["subject_ref"] != position.subject_ref
                || pulse["reading"]["physical"]["source_generation"] != position.generation
                || fact["transport_epoch"] != pulse["reading"]["transport_epoch"]
            {
                return Err(
                    "bootstrap differs from SAME native PerformanceOwner Boundary descriptor/pulse"
                        .into(),
                );
            }
        }
        _ => return Err("bootstrap timing domain/actual owner receipt is unpaired".into()),
    }
    Ok(())
}

/// Pure configuration correspondence over ONE already-returned actual v2
/// pulse. This function does not construct a witness, consumer, C lease, queue
/// admission or mapping capability from transported/captured JSON. The native
/// owner independently replays original source/receiving and closed Scene reads.
pub(crate) fn validate_performance_v2_configuration(
    pulse: &Value,
    position: &NativePosition,
    timing: &TimingBinding,
) -> Result<(), String> {
    let registry = &pulse["resident_consumers"];
    let fact = &pulse["payload"]["timing_fact"];
    let binding = &fact["binding"];
    let source = &registry["source"];
    let physical = &registry["physical_observation"]["snapshot"];
    let current_physical = &pulse["reading"]["physical"];
    let observation = &registry["timing_observation"];
    let decimal = |value: &Value| -> Result<u64, String> {
        let text = value
            .as_str()
            .ok_or("native v2 cursor must retain decimal text")?;
        let number = text
            .parse::<u64>()
            .map_err(|_| "invalid native v2 cursor".to_owned())?;
        if number.to_string() != text {
            return Err("noncanonical native v2 cursor".into());
        }
        Ok(number)
    };
    let epoch = decimal(&registry["transport_epoch"])?;
    let sample = decimal(&registry["sample"])?;
    decimal(&source["m1_revision"])?;
    decimal(&source["m2_generation"])?;
    let m3 = decimal(&registry["m3_source_generation"])?;
    let generation = decimal(&json!(position.generation))?;
    let rate = physical["sample_rate"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("actual native physical sample rate absent")?;
    let domain = binding
        .as_object()
        .ok_or("complete native v2 timing binding absent")?;
    if domain.len() != 5
        || [
            "owner_ref",
            "domain",
            "epoch_ref",
            "requested_cursor",
            "time_mapping_ref",
        ]
        .iter()
        .any(|key| !domain.contains_key(*key))
        || pulse["schema"] != "ql.performance-worker-reply/v1"
        || pulse["accepted"] != true
        || pulse["reading"]["schema"] != "ql.performance-management/v1"
        || registry["audio_observation"]["callback_output_committed"]
            .as_bool()
            .is_none()
        || fact["schema"] != "ql.native-performance-timing-fact/v2"
        || registry["schema"] != "ql.native-resident-consumer-registry/v2"
        || registry["origin"] != "actual-native-constructors-and-same-pulse"
        || epoch == 0
        || generation != epoch
        || fact["native_position"] != json!(position)
        || source != &fact["source"]["identity"]
        || source["instance"] != fact["source_instance_ref"]
        || fact["source_instance_ref"] != pulse["reading"]["scope"]["instance_ref"]
        || source["event"] != position.event_ref
        || source["subject"] != position.subject_ref
        || pulse["reading"]["scope"]["event_ref"] != position.event_ref
        || pulse["reading"]["scope"]["subject_ref"] != position.subject_ref
        || pulse["reading"]["session_ref"] != timing.owner_ref
        || binding["owner_ref"] != timing.owner_ref
        || binding["domain"] != "native_samples"
        || timing.domain != "native_samples"
        || binding["epoch_ref"] != timing.epoch_ref
        || timing.epoch_ref != format!("ql:performance/transport-epoch/{epoch}")
        || binding["time_mapping_ref"] != json!(timing.time_mapping_ref)
        || timing.time_mapping_ref.as_deref()
            != Some("ql:performance/native-samples-scene-local-seconds/v1")
        || decimal(&binding["requested_cursor"])? != timing.requested_cursor
        || decimal(&fact["admission_horizon"])? != timing.requested_cursor
        || fact["transport_epoch"] != registry["transport_epoch"]
        || registry["transport_epoch"] != pulse["reading"]["transport_epoch"]
        || fact["committed_cursor"] != registry["sample"]
        || registry["sample"] != pulse["reading"]["samples_elapsed"]
        || sample != position.cursor()?
        || physical["schema"] != "ql.native-physical-snapshot/v1"
        || physical["version"] != 1
        || physical["source_generation"] != registry["m3_source_generation"]
        || decimal(&fact["m3_source_generation"])? != m3
        || physical["samples_elapsed"] != registry["sample"]
        || physical["event_ref"] != position.event_ref
        || physical["subject_ref"] != position.subject_ref
        || physical["preparation_ref"] != fact["source"]["body_preparation_ref"]
        || physical["state_ref"] != fact["source"]["body_state_ref"]
        || physical["body_revision"] != fact["source"]["body_revision"]
    {
        return Err(
            "v2 native Management position, original source or independent M3 boundary differs"
                .into(),
        );
    }
    // Keep the complete original P snapshot correspondence rather than dating
    // its source generation with the independent Management transport epoch.
    for key in [
        "event_ref",
        "subject_ref",
        "preparation_ref",
        "state_ref",
        "source_coordinate",
        "source_revision",
        "eigenbasis_identity",
        "source_generation",
        "body_revision",
        "samples_elapsed",
        "sample_rate",
        "pratibimba",
        "pickup_linear",
    ] {
        if physical[key].is_null() || physical[key] != current_physical[key] {
            return Err(format!("v2 native physical snapshot differs at {key}"));
        }
    }
    for (full, current) in [
        ("node_identity", "node_ids"),
        ("visible_positions_metres", "positions_metres"),
        ("mechanical_energy_joules", "energy_joules"),
    ] {
        if physical[full].is_null() || physical[full] != current_physical[current] {
            return Err(format!("v2 original native physical snapshot lost {full}"));
        }
    }
    let rows = registry["required_consumers"]
        .as_array()
        .filter(|rows| (3..=4).contains(&rows.len()))
        .ok_or("v2 complete actual native resident roster absent")?;
    let expected = [
        "audio_engine",
        "physical_body",
        "timing_owner",
        "acoustic_receiving",
    ];
    let mut instances = BTreeSet::new();
    for (index, row) in rows.iter().enumerate() {
        let role = row["role"].as_str().ok_or("v2 native role absent")?;
        let instance = row["instance_ref"]
            .as_str()
            .ok_or("v2 native constructor instance absent")?;
        let token = instance
            .strip_prefix("native-resident:v1:")
            .and_then(|s| s.split_once(':'))
            .ok_or("v2 native resident token is not a constructor identity")?;
        let ordinal = decimal(&row["construction_ordinal"])?;
        let token_ordinal = decimal(&json!(token.1))?;
        let observed = &registry[match role {
            "audio_engine" => "audio_observation",
            "physical_body" => "physical_observation",
            "timing_owner" => "timing_observation",
            "acoustic_receiving" => "receiving_observation",
            _ => return Err("unknown v2 native resident role".into()),
        }];
        let is_timing = role == "timing_owner";
        if role != expected[index]
            || token.0.len() != 32
            || !token
                .0
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
            || token.0.bytes().all(|c| c == b'0')
            || ordinal == 0
            || ordinal != token_ordinal
            || !instances.insert(instance)
            || row["sample"] != registry["sample"]
            || observed["sample"] != registry["sample"]
            || observed["instance_ref"] != instance
            || row["callback_output_committed"]
                != registry["audio_observation"]["callback_output_committed"]
            || row["generation_domain"]
                != if is_timing {
                    "native-management-transport-epoch"
                } else {
                    "native-resident-construction"
                }
            || decimal(&row["generation"])? != if is_timing { epoch } else { ordinal }
            || (role == "physical_body" && physical["resident_instance_ref"] != instance)
            || (is_timing
                && (instance != position.instance_ref
                    || row["owner_ref"] != timing.owner_ref
                    || row["construction_ordinal"] != observation["construction_ordinal"]))
        {
            return Err(
                "v2 native role, constructor lifetime or independent generation domain differs"
                    .into(),
            );
        }
    }
    if (rows.len() == 4) != !registry["receiving_observation"].is_null()
        || observation["owner_ref"] != timing.owner_ref
        || observation["instance_ref"] != position.instance_ref
        || observation["generation"] != registry["transport_epoch"]
        || observation["transport_epoch"] != registry["transport_epoch"]
        || observation["generation_domain"] != "native-management-transport-epoch"
        || observation["committed_cursor"] != registry["sample"]
        || observation["available"] != true
        || observation["callback_output_committed"]
            != registry["audio_observation"]["callback_output_committed"]
        || observation["accepted_sequence"] != fact["accepted_sequence"]
        || observation["accepted_sequence"] != pulse["reading"]["accepted_sequence"]
        || observation["last_applied_sequence"]
            != registry["audio_observation"]["last_applied_sequence"]
        || observation["last_applied_application_ordinal"]
            != fact["last_applied_application_ordinal"]
        || observation["last_applied_application_ordinal"]
            != pulse["reading"]["last_applied_application_ordinal"]
    {
        return Err(
            "v2 Management observation is not the SAME returned native timing pulse".into(),
        );
    }
    let mapping = &fact["time_mapping"];
    if mapping.as_object().is_none_or(|m| m.len() != 12)
        || mapping["schema"] != "ql.native-samples-scene-local-seconds/v1"
        || mapping["owner_ref"] != timing.owner_ref
        || mapping["instance_ref"] != position.instance_ref
        || mapping["transport_epoch"] != registry["transport_epoch"]
        || decimal(&mapping["native_origin_sample"])? != 0
        || decimal(&mapping["scene_local_origin_seconds_numerator"])? != 0
        || decimal(&mapping["seconds_per_sample_numerator"])? != 1
        || decimal(&mapping["seconds_per_sample_denominator"])? != rate
        || mapping["committed_sample"] != registry["sample"]
        || mapping["committed_scene_seconds_numerator"] != registry["sample"]
        || decimal(&mapping["committed_scene_seconds_denominator"])? != rate
        || mapping["standing"].as_str().is_none_or(str::is_empty)
    {
        return Err(
            "v2 native named mapping differs from the actual continued sample coordinate/rate"
                .into(),
        );
    }
    // A valid copied mapping is NOT current Scene attachment or a recording
    // admission. The private C Scene owner must separately qualify that policy.
    Ok(())
}

fn validate_source_plan(
    subjects: &BTreeSet<&str>,
    binding: &crate::vak_scope::CPrimeOperativeBinding,
    plan: &ThreadPlan,
    recipe: &str,
    whole: &str,
) -> Result<(), String> {
    let inputs: BTreeSet<&str> = binding
        .sources
        .iter()
        .map(|s| s.source_ref.as_str())
        .chain(binding.evidence_refs.iter().map(String::as_str))
        .collect();
    for leg in &plan.legs {
        let predecessors: BTreeSet<&str> = plan
            .legs
            .iter()
            .filter(|p| leg.after.contains(&p.unit_ref))
            .map(|p| p.result_ref.as_str())
            .collect();
        if !subjects.contains(leg.subject_ref.as_str())
            || leg.scope_ref != whole
            || leg.input_refs.is_empty()
            || leg
                .input_refs
                .iter()
                .any(|r| !inputs.contains(r.as_str()) && !predecessors.contains(r.as_str()))
            || !leg
                .input_refs
                .iter()
                .any(|r| r == recipe || predecessors.contains(r.as_str()))
        {
            return Err(
                "native source Thread leg exceeds exact participants/scope/source dependencies"
                    .into(),
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn replay_captured_source(
        input: &NativeSourceBootstrap,
        observed: &NativeBootstrapObservation,
        result: &Value,
    ) -> Result<Value, String> {
        if let Some(contract) = result.get("consumer_contract") {
            let contract = serde_json::from_value(contract.clone()).map_err(|e| e.to_string())?;
            let mut replay =
                compile_native_source_bootstrap_registered(input, observed, &contract)?;
            // Configuration diagnostics retained from the original private
            // factories are compared only, never converted into a capability.
            for key in [
                "native_scene_constructor_fact",
                "native_timing_consumer_fact",
            ] {
                let value = result
                    .get(key)
                    .ok_or("captured original constructor fact absent")?;
                replay[key] = value.clone();
            }
            Ok(replay)
        } else {
            compile_native_source_bootstrap(input, observed)
        }
    }
    fn scene() -> NativeBootstrapSceneRead {
        let mut presentation: Value = serde_json::from_str(include_str!(
            "../../../fixtures/kernel/procedural-scene-template-v1.json"
        ))
        .unwrap();
        presentation["scene"]["id"] = json!("expression:source-test:scene:canonical");
        NativeBootstrapSceneRead {
            native_owner: "oi.expression".into(),
            expression_ref: "expression:source-test".into(),
            scene_ref: "expression:source-test:scene:canonical".into(),
            document_revision: 3,
            source_basis: SourceBasis {
                source_ref: "oi.expression:source-test/canonical-material".into(),
                revision: fingerprint(&presentation).unwrap(),
            },
            material_fingerprint: fingerprint(&presentation).unwrap(),
            presentation,
            locus: NativeReading {
                reference: "ql:m-coordinate:bimba:M3".into(),
                revision: native_current_m_registry()
                    .manifest()
                    .registry_revision
                    .clone(),
                availability: ReadingAvailability::Available,
            },
            source_read_receipt_ref: "oi.expression:source-test/read/actual-3".into(),
        }
    }
    #[test]
    fn ordinary_native_material_projection_is_exact_and_does_not_invent_a_binding() {
        let source = scene();
        source.validate().unwrap();
        assert!(source.presentation["scene"].get("procedural").is_none());
        assert!(
            serde_json::to_value(&source)
                .unwrap()
                .get("principal")
                .is_none()
        );
        for kind in [
            "foreignScene",
            "staleMaterial",
            "unavailableLocus",
            "unknownOwner",
            "importedRetention",
        ] {
            let mut wrong = source.clone();
            match kind {
                "foreignScene" => wrong.scene_ref = "expression:other:scene:canonical".into(),
                "staleMaterial" => {
                    wrong.presentation["scene"]["caption"] = json!("actual changed source")
                }
                "unavailableLocus" => wrong.locus.availability = ReadingAvailability::Unavailable,
                "unknownOwner" => wrong.native_owner = "foreign.owner".into(),
                "importedRetention" => {
                    wrong.presentation["scene"]["procedural"] = json!({"operations":[]})
                }
                _ => unreachable!(),
            }
            assert!(wrong.validate().is_err(), "accepted {kind}");
        }
    }
    #[test]
    fn transported_graph_currentness_and_interpretation_are_not_native_bootstrap_input() {
        let mut wire = serde_json::to_value(scene()).unwrap();
        for field in [
            "source_composition",
            "currentness",
            "interpretation",
            "principal",
            "native_act_source",
        ] {
            wire[field] = json!({"state":"current"});
            assert!(serde_json::from_value::<NativeBootstrapSceneRead>(wire.clone()).is_err());
            wire.as_object_mut().unwrap().remove(field);
        }
    }
    /// This gate consumes an output captured from the real private paused/running
    /// C28 owner. It does not mint a lease/witness from its retained evidence.
    #[test]
    #[ignore = "requires actual C28 native bootstrap input/output capture"]
    fn actual_private_native_bootstrap_replays_original_profile_thread_and_detects_changed_source()
    {
        let path = std::env::var("QL_PROCEDURAL_BOOTSTRAP_ARTIFACT")
            .expect("actual native source bootstrap artifact");
        let artifact: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            artifact["schema"],
            "ql.native-procedural-source-bootstrap-capture/v1"
        );
        let input: NativeSourceBootstrap =
            serde_json::from_value(artifact["input"].clone()).unwrap();
        let actual = &artifact["result"];
        assert_eq!(actual["schema"], SOURCE_BOOTSTRAP);
        let observed = NativeBootstrapObservation {
            position: serde_json::from_value(actual["native_position"].clone()).unwrap(),
            timing: serde_json::from_value(actual["timing"].clone()).unwrap(),
            field_source: actual["native_field_source"].clone(),
            field_receipt: actual["native_field_receipt"].clone(),
            native_timing_pulse: actual
                .get("native_timing_pulse")
                .filter(|v| !v.is_null())
                .cloned(),
            native_act_source: actual["native_act_source"].clone(),
        };
        let replay = replay_captured_source(&input, &observed, actual).unwrap();
        assert_eq!(
            &replay, actual,
            "original full native source cannot be relabelled on replay"
        );
        let wrong_boundary = NativeBootstrapObservation {
            position: observed.position.clone(),
            timing: crate::procedural_composition::TimingBinding {
                requested_cursor: observed.timing.requested_cursor.checked_add(1).unwrap(),
                ..observed.timing.clone()
            },
            field_source: observed.field_source.clone(),
            field_receipt: observed.field_receipt.clone(),
            native_timing_pulse: observed.native_timing_pulse.clone(),
            native_act_source: observed.native_act_source.clone(),
        };
        assert!(
            compile_native_source_bootstrap(&input, &wrong_boundary).is_err(),
            "original genuine owner timing descriptor cannot be relabelled"
        );
        if observed.timing.domain == "native_samples" {
            assert!(
                observed.field_receipt.is_null(),
                "performance cannot use stale field receipt"
            );
            let mut wrong = NativeBootstrapObservation {
                position: observed.position.clone(),
                timing: observed.timing.clone(),
                field_source: observed.field_source.clone(),
                field_receipt: Value::Null,
                native_timing_pulse: observed.native_timing_pulse.clone(),
                native_act_source: observed.native_act_source.clone(),
            };
            wrong.native_timing_pulse.as_mut().unwrap()["payload"]["timing_fact"]["binding"]["epoch_ref"] =
                json!("foreign:epoch");
            assert!(
                compile_native_source_bootstrap(&input, &wrong).is_err(),
                "copied genuine performance pulse cannot retarget its original native epoch"
            );
            wrong.native_timing_pulse = None;
            assert!(
                compile_native_source_bootstrap(&input, &wrong).is_err(),
                "no prepared/sounding timing receipt may be fabricated from last_field"
            );
        } else {
            assert!(observed.native_timing_pulse.is_none());
        }
        let graph = crate::vak_composition_wire::compile_request(&actual["source_composition"])
            .unwrap()
            .1;
        let request: OperativeScopeCurrentnessRequest =
            serde_json::from_value(actual["currentness"].clone()).unwrap();
        assert!(matches!(
            graph
                .observe_operative_scope_currentness(request)
                .unwrap()
                .observation,
            OperativeScopeObservation::Current { .. }
        ));
        // A source-qualified contributor remains disclosed independently. Only
        // principal/actual native ShapeBinding members can perform a Thread leg.
        let mut nonmember = input.clone();
        let registry = native_current_m_registry();
        let manifest = registry.manifest();
        let original_whole = graph.whole(&input.scene.expression_ref).unwrap();
        let mut members = BTreeSet::from([original_whole.binding.subject_ref.as_str()]);
        members.extend(
            original_whole
                .binding
                .member_bindings()
                .iter()
                .map(|member| member.subject_ref.as_str()),
        );
        let reference = manifest
            .nodes
            .iter()
            .filter_map(|node| {
                registry
                    .coordinate(&node.source_ref, crate::MFace::Bimba)
                    .ok()
            })
            .map(|coordinate| coordinate.canonical_ref())
            .find(|reference| !members.contains(reference.as_str()))
            .expect(
                "actual native registry must supply a coordinate outside this bounded source whole",
            );
        nonmember
            .authorship
            .contributors
            .retain(|s| s.subject_ref != reference);
        nonmember.authorship.contributors.push(NativeSubject {
            subject_ref: reference.clone(),
            native_owner: "ql-mef".into(),
            presentation_role: SubjectRole::Thing,
            sources: vec![NativeReading {
                reference: format!(
                    "{}:{}",
                    manifest.source_repository, manifest.source_dataset_tree
                ),
                revision: manifest.source_snapshot_sha256.clone(),
                availability: ReadingAvailability::Available,
            }],
            readings: vec![],
            actions: vec![],
        });
        nonmember.authorship.profile.thread = crate::vak_profile::ThreadForm::Single;
        nonmember.authorship.thread_plan.legs.truncate(1);
        let leg = &mut nonmember.authorship.thread_plan.legs[0];
        leg.subject_ref = reference;
        leg.after.clear();
        leg.parent = None;
        nonmember.authorship.thread_plan.aggregation_ref = None;
        nonmember.authorship.thread_plan.continuation_ref = None;
        nonmember.authorship.thread_plan.stop_condition_ref = None;
        assert!(
            compile_native_source_bootstrap(&nonmember, &observed)
                .unwrap_err()
                .contains("participants/scope/source"),
            "qualified nonmember contributor acquired native Thread conduct"
        );
        for change in ["subject", "member", "scope", "input", "source", "prime"] {
            let mut changed = input.clone();
            match change {
                "subject" => {
                    changed.authorship.thread_plan.legs[0].subject_ref = "foreign:subject".into()
                }
                "member" => changed.authorship.members.push(NativeStageMember {
                    subject_ref: "foreign:member".into(),
                    coordinate: NativeStageCoordinate {
                        position: 1,
                        face: "direct".into(),
                    },
                }),
                "scope" => {
                    changed.authorship.thread_plan.legs[0].scope_ref = "expression:foreign".into()
                }
                "input" => {
                    changed.authorship.thread_plan.legs[0].input_refs =
                        vec!["foreign:recipe".into()]
                }
                "source" => {
                    changed.scene.presentation["scene"]["caption"] =
                        json!("different actual material")
                }
                "prime" => changed.authorship.frame.face = "unknown-prime".into(),
                _ => unreachable!(),
            }
            assert!(
                compile_native_source_bootstrap(&changed, &observed).is_err(),
                "accepted {change}"
            );
        }
    }
    #[test]
    #[ignore = "requires actual private prepared-held and sounding coherent v2 Source captures; no native witness mocks"]
    fn actual_v2_bootstrap_keeps_management_transport_epoch_and_field_source_m3_distinct() {
        for key in [
            "QL_PROCEDURAL_BOOTSTRAP_PREPARED_HELD_ARTIFACT",
            "QL_PROCEDURAL_BOOTSTRAP_SOUNDING_ARTIFACT",
        ] {
            let path = std::env::var(key)
                .expect("actual native private coherent v2 Source capture required");
            let capture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            assert_eq!(
                capture["schema"],
                "ql.native-procedural-source-bootstrap-capture/v1"
            );
            let actual = &capture["result"];
            let pulse = &actual["native_timing_pulse"];
            let position: NativePosition =
                serde_json::from_value(actual["native_position"].clone()).unwrap();
            let timing: TimingBinding = serde_json::from_value(actual["timing"].clone()).unwrap();
            let before = pulse.clone();
            validate_performance_v2_configuration(pulse, &position, &timing).unwrap();
            let fact = &pulse["payload"]["timing_fact"];
            assert_eq!(fact["schema"], "ql.native-performance-timing-fact/v2");
            assert_ne!(json!(position.instance_ref), fact["source_instance_ref"]);
            assert_eq!(
                actual["native_field_source"]["instance_ref"],
                fact["source_instance_ref"]
            );
            assert_eq!(json!(position.generation), fact["transport_epoch"]);
            assert_eq!(
                fact["m3_source_generation"],
                pulse["reading"]["physical"]["source_generation"]
            );
            assert!(actual["native_field_receipt"].is_null());
            let changed_decimal = |value: &Value| -> Value {
                json!(
                    (value
                        .as_str()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                        .checked_add(1)
                        .unwrap())
                    .to_string()
                )
            };
            for pointer in [
                "/payload/timing_fact/native_position/generation",
                "/payload/timing_fact/transport_epoch",
                "/payload/timing_fact/m3_source_generation",
                "/resident_consumers/m3_source_generation",
                "/resident_consumers/physical_observation/snapshot/source_generation",
                "/resident_consumers/timing_observation/generation",
                "/resident_consumers/required_consumers/2/construction_ordinal",
                "/payload/timing_fact/time_mapping/seconds_per_sample_denominator",
                "/payload/timing_fact/time_mapping/native_origin_sample",
                "/payload/timing_fact/time_mapping/committed_scene_seconds_numerator",
            ] {
                let mut wrong = pulse.clone();
                let changed = changed_decimal(wrong.pointer(pointer).unwrap());
                *wrong.pointer_mut(pointer).unwrap() = changed;
                assert!(
                    validate_performance_v2_configuration(&wrong, &position, &timing).is_err(),
                    "accepted {pointer}"
                );
            }
            for pointer in [
                "/payload/timing_fact/source_instance_ref",
                "/resident_consumers/source/instance",
                "/resident_consumers/timing_observation/instance_ref",
                "/resident_consumers/required_consumers/2/owner_ref",
                "/payload/timing_fact/binding/time_mapping_ref",
            ] {
                let mut wrong = pulse.clone();
                *wrong.pointer_mut(pointer).unwrap() = json!("foreign:native-owner");
                assert!(
                    validate_performance_v2_configuration(&wrong, &position, &timing).is_err(),
                    "accepted {pointer}"
                );
            }
            let mut wrong = pulse.clone();
            wrong["resident_consumers"]["required_consumers"]
                .as_array_mut()
                .unwrap()
                .swap(1, 2);
            assert!(validate_performance_v2_configuration(&wrong, &position, &timing).is_err());
            let mut wrong = pulse.clone();
            wrong["payload"]["timing_fact"]["schema"] =
                json!("ql.native-performance-timing-fact/v1");
            assert!(
                validate_performance_v2_configuration(&wrong, &position, &timing).is_err(),
                "historical v1 cannot be retagged as a v2 observation"
            );
            let mut wrong = pulse.clone();
            wrong["resident_consumers"]["audio_observation"]["callback_output_committed"] =
                Value::Null;
            assert!(validate_performance_v2_configuration(&wrong, &position, &timing).is_err());
            assert_eq!(pulse, &before);
            // This independently rechecks retained configuration, not the C/R
            // private constructor, Scene mapping attachment or live receiving.
        }
    }
    #[test]
    #[ignore = "requires actual private C28 prepared-held and sounding source-bootstrap captures; no lease or receiver mocks"]
    fn actual_prepared_held_and_sounding_bootstrap_preserve_same_performance_owner_descriptor_and_pulse()
     {
        for (key, callbacks_running) in [
            ("QL_PROCEDURAL_BOOTSTRAP_PREPARED_HELD_ARTIFACT", false),
            ("QL_PROCEDURAL_BOOTSTRAP_SOUNDING_ARTIFACT", true),
        ] {
            let path = std::env::var(key)
                .expect("actual native private prepared/sounding Source capture required");
            let capture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            assert_eq!(
                capture["schema"],
                "ql.native-procedural-source-bootstrap-capture/v1"
            );
            let input: NativeSourceBootstrap =
                serde_json::from_value(capture["input"].clone()).unwrap();
            let actual = &capture["result"];
            let observed = NativeBootstrapObservation {
                position: serde_json::from_value(actual["native_position"].clone()).unwrap(),
                timing: serde_json::from_value(actual["timing"].clone()).unwrap(),
                field_source: actual["native_field_source"].clone(),
                field_receipt: actual["native_field_receipt"].clone(),
                native_timing_pulse: actual
                    .get("native_timing_pulse")
                    .filter(|v| !v.is_null())
                    .cloned(),
                native_act_source: actual["native_act_source"].clone(),
            };
            assert_eq!(observed.timing.domain, "native_samples");
            assert!(observed.field_receipt.is_null());
            let pulse = observed.native_timing_pulse.as_ref().unwrap();
            assert_eq!(
                pulse["payload"]["timing_fact"]["device_callbacks_running"],
                callbacks_running
            );
            assert_eq!(
                &replay_captured_source(&input, &observed, actual).unwrap(),
                actual
            );
            // Captured exact native receiver output is independently replayed
            // through actual source compilation. This test creates no lease,
            // witness or prepared performance owner from the recorded JSON.
        }
    }
}
