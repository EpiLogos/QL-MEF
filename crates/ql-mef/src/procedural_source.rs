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
fn compile_source(
    registry: &MRegistry,
    input: &NativeSourceBootstrap,
    observed: &NativeBootstrapObservation,
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
    if observed.position.subject_ref
        != observed.field_source["current_basis"]["input"]["m3"]["subject_ref"]
        || observed.field_source["schema"] != "ql.native-held-field-source/v1"
        || observed.field_source["instance_ref"] != observed.position.instance_ref
    {
        return Err(
            "bootstrap observation differs from actual guarded native field/source boundary".into(),
        );
    }
    validate_bootstrap_timing(observed)?;
    let mut sources = vec![NativeReading {
        reference: format!("ql-mef:held-field-basis:{}", observed.position.instance_ref),
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
    let basis = |source: &SourceBasis| {
        json!({"caller":a.actor_ref,"source":source.source_ref,"revision":source.revision,
        "standing":a.standing_ref,"evidence":[s.source_basis.source_ref,s.material_fingerprint]})
    };
    let members=a.members.iter().map(|m|json!({"subjectRef":m.subject_ref,"position":m.coordinate.position,"face":m.coordinate.face})).collect::<Vec<_>>();
    let returns=a.source_returns.iter().map(|r|json!({"fromRef":r.from_ref,"anchorRef":r.anchor_ref,"groundRef":r.ground_ref,"face":r.face,"kind":r.kind})).collect::<Vec<_>>();
    let relations=a.relations.iter().map(|r|json!({"row":r.row,"column":r.column,"relationRef":r.relation_ref,"evidence":r.evidence})).collect::<Vec<_>>();
    let composition = json!({"contract":crate::vak_composition::CONTRACT,"steps":[
        {"op":"whole","useRef":initial,"wholeRef":s.expression_ref,"subjectRef":principal.subject_ref,"members":members,
         "sourceReturns":returns,"relations":relations,"category":a.category,"groundRef":a.ground_ref,"groundFace":a.ground_face,
         "frame":a.frame,"basis":basis(&s.source_basis),"language":a.language},
        {"op":"reframe","from":initial,"into":s.expression_ref,"frame":a.frame,"basis":basis(&recipe)}]});
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
        .bind_operative_scope(&s.expression_ref, a.profile, correlation.clone())
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
        .compile_profile(&s.expression_ref, a.profile)
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
        let replay = compile_native_source_bootstrap(&input, &observed).unwrap();
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
                &compile_native_source_bootstrap(&input, &observed).unwrap(),
                actual
            );
            // Captured exact native receiver output is independently replayed
            // through actual source compilation. This test creates no lease,
            // witness or prepared performance owner from the recorded JSON.
        }
    }
}
