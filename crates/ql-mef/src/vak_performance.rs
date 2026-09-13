//! QL projection of an actual Factory Vāk performance.
//!
//! Factory owns Run/WorkflowUnit/Attempt/Return actuality. QL owns the C′ and
//! musical meaning of that performed occasion. This module therefore consumes
//! the published Factory snapshot as source evidence, validates it against an
//! already compiled QL profile, and emits one attributable event for K8/AW3.
//! It is not another scheduler, attempt store, clock, parser or Recognition owner.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::cprime_oikonomia::C_PRIME_OIKONOMIA_CONTRACT;
use crate::music::{MUSICAL_DERIVATION_SOURCE_BLOB, MUSICAL_DERIVATION_SOURCE_PATH, MusicalBasis};
use crate::vak_composition::{CompositionError, Result};
use crate::vak_profile::{
    CompiledProfile, ContentPosition, ContentType, ContextSequence, InquiryDirection,
    Participation, ThreadForm, PROFILE_CONTRACT, PROFILE_SOURCE, PROFILE_SOURCE_BLOB,
    constitutional_voice,
};

pub const FACTORY_VAK_PERFORMANCE_CONTRACT: &str = "factory.vak-orchestration/v1";
pub const PERFORMANCE_PROJECTION_REQUEST: &str = "ql.vak-performance-projection/v1";
pub const PERFORMANCE_EVENT_CONTRACT: &str = "ql.vak-performance-event/v1";
pub const PERFORMANCE_SOURCE: &str =
    "docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md#22-compose-perform-record-rehear-recompose";
const MAX_ATTEMPTS: usize = 4096;
const MAX_SOURCE_REFS: usize = 4096;

fn error(message: impl Into<String>) -> CompositionError {
    CompositionError(message.into())
}

fn require(test: bool, message: &str) -> Result<()> {
    if test { Ok(()) } else { Err(error(message)) }
}

fn reference(value: &str, what: &str) -> Result<()> {
    require(
        !value.trim().is_empty() && value.len() <= 16_384 && !value.contains('\0'),
        what,
    )
}

fn references(values: &BTreeSet<String>, what: &str) -> Result<()> {
    require(!values.is_empty() && values.len() <= MAX_SOURCE_REFS, what)?;
    for value in values {
        reference(value, what)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactoryLegStatus {
    Active,
    Detached,
    CancelRequested,
    CancellationAccepted,
    ProcessTerminated,
    Quiescent,
    Returned,
    Failed,
    LateResult,
}

impl FactoryLegStatus {
    pub const fn settled(self) -> bool {
        matches!(
            self,
            Self::CancellationAccepted
                | Self::ProcessTerminated
                | Self::Quiescent
                | Self::Returned
                | Self::Failed
                | Self::LateResult
        )
    }

    pub const fn failed(self) -> bool {
        matches!(
            self,
            Self::CancellationAccepted
                | Self::ProcessTerminated
                | Self::Quiescent
                | Self::Failed
                | Self::LateResult
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryVakAttempt {
    pub unit_ref: String,
    pub attempt_index: usize,
    pub current: bool,
    pub execution_ref: String,
    pub actor_ref: String,
    pub whole_ref: String,
    pub subject_ref: String,
    pub ql_binding_ref: String,
    pub ql_binding_revision: String,
    pub ai_kit_resolve_path_ref: String,
    pub context_resolution_ref: String,
    pub source_refs: BTreeSet<String>,
    pub model_ref: String,
    pub provider_ref: String,
    pub status: FactoryLegStatus,
    pub status_history: Vec<FactoryLegStatus>,
    pub artifact_refs: BTreeSet<String>,
    pub late_artifact_refs: BTreeSet<String>,
    pub evidence_refs: BTreeSet<String>,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryVakChainMaterial {
    pub predecessor_unit_ref: String,
    pub predecessor_execution_ref: String,
    pub successor_unit_ref: String,
    pub subject_ref: String,
    pub subject_revision: String,
    pub artifact_refs: BTreeSet<String>,
    pub evidence_refs: BTreeSet<String>,
    pub semantic_differences: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryVakSustainedStop {
    pub stop_condition_ref: String,
    pub native_stop_conditions: String,
    pub owner_ref: String,
    pub source_revision: String,
    pub evidence_refs: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryVakPerformanceSnapshot {
    pub contract: String,
    pub performance_ref: String,
    pub run_ref: String,
    pub run_revision: u64,
    pub workflow_source_ref: String,
    pub workflow_source_revision: String,
    pub workflow_source_digest: String,
    pub actor_ref: String,
    pub subject_ref: String,
    pub whole_ref: String,
    pub ql_binding_ref: String,
    pub ql_binding_revision: String,
    pub ai_kit_resolve_path_ref: String,
    pub context_resolution_ref: String,
    pub source_refs: BTreeSet<String>,
    pub frame: String,
    pub thread: String,
    pub sequence: String,
    pub direction: String,
    pub musical_role: String,
    pub attempts: Vec<FactoryVakAttempt>,
    pub chain_inputs: Vec<FactoryVakChainMaterial>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sustained_stop: Option<FactoryVakSustainedStop>,
}

impl FactoryVakPerformanceSnapshot {
    pub fn settled(&self) -> bool {
        !self.attempts.is_empty() && self.attempts.iter().all(|attempt| attempt.status.settled())
    }

    pub fn has_failure(&self) -> bool {
        self.attempts.iter().any(|attempt| attempt.status.failed())
    }

    pub fn has_late_return(&self) -> bool {
        self.attempts
            .iter()
            .any(|attempt| !attempt.late_artifact_refs.is_empty())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PerformanceObservationMode {
    Live,
    Replay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerformanceObservation {
    pub observation_ref: String,
    pub mode: PerformanceObservationMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_of: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerformanceProjectionRequest {
    pub schema: String,
    pub ql_binding_ref: String,
    pub ql_binding_revision: String,
    pub observation: PerformanceObservation,
    pub factory_receipt_refs: BTreeSet<String>,
    pub snapshot: FactoryVakPerformanceSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerformanceSemantics {
    pub profile_contract: String,
    pub participation: Participation,
    pub content: ContentType,
    pub position: ContentPosition,
    pub context_frame: String,
    pub constitutional_voice: String,
    pub thread: ThreadForm,
    pub sequence: ContextSequence,
    pub direction: InquiryDirection,
    pub musical_role: String,
    pub lens: String,
    pub musical_basis: String,
    pub frame_pitch: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VakPerformanceEvent {
    pub contract: String,
    /// Same performed occasion as Factory's `performanceRef`; QL does not mint a
    /// second event identity merely because it supplies another interpretation.
    pub performance_ref: String,
    pub observation: PerformanceObservation,
    pub factory_contract: String,
    pub factory_receipt_refs: BTreeSet<String>,
    pub ql_binding_ref: String,
    pub ql_binding_revision: String,
    pub ql_basis_refs: BTreeSet<String>,
    pub semantics: PerformanceSemantics,
    pub settled: bool,
    pub has_failure: bool,
    pub has_late_return: bool,
    pub factory: FactoryVakPerformanceSnapshot,
    pub standing: String,
}

impl CompiledProfile {
    /// Project an actual Factory performance into QL's musical/relational event.
    /// The Factory snapshot remains intact inside the event; QL validates the
    /// source binding and derives meaning from this compiled profile rather than
    /// treating owner-returned strings as semantic authority.
    pub fn project_factory_performance(
        &self,
        request: PerformanceProjectionRequest,
    ) -> Result<VakPerformanceEvent> {
        validate_request(self, &request)?;
        let snapshot = &request.snapshot;
        let mut ql_basis_refs = BTreeSet::from([
            PROFILE_SOURCE.to_owned(),
            format!("git-blob:{PROFILE_SOURCE_BLOB}"),
            C_PRIME_OIKONOMIA_CONTRACT.to_owned(),
            PERFORMANCE_SOURCE.to_owned(),
            MUSICAL_DERIVATION_SOURCE_PATH.to_owned(),
            format!("git-blob:{MUSICAL_DERIVATION_SOURCE_BLOB}"),
        ]);
        for basis in &self.basis {
            basis.validate()?;
            ql_basis_refs.insert(basis.provenance.source_ref.clone());
            ql_basis_refs.extend(basis.evidence.iter().cloned());
        }
        ql_basis_refs.extend(snapshot.source_refs.iter().cloned());
        ql_basis_refs.extend(request.factory_receipt_refs.iter().cloned());

        Ok(VakPerformanceEvent {
            contract: PERFORMANCE_EVENT_CONTRACT.into(),
            performance_ref: snapshot.performance_ref.clone(),
            observation: request.observation,
            factory_contract: snapshot.contract.clone(),
            factory_receipt_refs: request.factory_receipt_refs,
            ql_binding_ref: request.ql_binding_ref,
            ql_binding_revision: request.ql_binding_revision,
            ql_basis_refs,
            semantics: PerformanceSemantics {
                profile_contract: PROFILE_CONTRACT.into(),
                participation: self.profile.participation,
                content: self.profile.content,
                position: self.profile.position,
                context_frame: self.frame.id.code().into(),
                constitutional_voice: constitutional_voice(self.frame.id).into(),
                thread: self.profile.thread,
                sequence: self.profile.sequence,
                direction: self.profile.direction,
                musical_role: self.profile.thread.musical_role().into(),
                lens: self.frame.lens.code().into(),
                musical_basis: musical_basis(self.frame.basis).into(),
                frame_pitch: self.frame_pitch(),
            },
            settled: snapshot.settled(),
            has_failure: snapshot.has_failure(),
            has_late_return: snapshot.has_late_return(),
            factory: request.snapshot,
            standing: "QL semantic projection of an actual Factory owner snapshot; musical coherence is not task-fitness or Recognition evidence".into(),
        })
    }
}

fn validate_request(profile: &CompiledProfile, request: &PerformanceProjectionRequest) -> Result<()> {
    require(
        request.schema == PERFORMANCE_PROJECTION_REQUEST,
        "unsupported Vāk performance projection request",
    )?;
    reference(&request.ql_binding_ref, "missing QL binding reference")?;
    reference(
        &request.ql_binding_revision,
        "missing QL binding revision",
    )?;
    references(
        &request.factory_receipt_refs,
        "Factory performance projection requires actual owner receipt evidence",
    )?;
    validate_observation(&request.observation, &request.snapshot.performance_ref)?;
    validate_snapshot(profile, request)
}

fn validate_observation(observation: &PerformanceObservation, performance_ref: &str) -> Result<()> {
    reference(&observation.observation_ref, "missing performance observation")?;
    match observation.mode {
        PerformanceObservationMode::Live => require(
            observation.replay_of.is_none(),
            "live performance observation cannot masquerade as replay",
        ),
        PerformanceObservationMode::Replay => {
            let replay = observation
                .replay_of
                .as_deref()
                .ok_or_else(|| error("replay must name the original performance"))?;
            reference(replay, "invalid replay source")?;
            require(
                replay == performance_ref,
                "replay must retain the original Factory performance identity",
            )
        }
    }
}

fn validate_snapshot(profile: &CompiledProfile, request: &PerformanceProjectionRequest) -> Result<()> {
    let snapshot = &request.snapshot;
    require(
        snapshot.contract == FACTORY_VAK_PERFORMANCE_CONTRACT,
        "unsupported Factory Vāk performance contract",
    )?;
    require(snapshot.run_revision > 0, "Factory Run revision must be positive")?;
    for (value, message) in [
        (&snapshot.performance_ref, "missing Factory performance reference"),
        (&snapshot.run_ref, "missing Factory Run reference"),
        (&snapshot.workflow_source_ref, "missing workflow source reference"),
        (
            &snapshot.workflow_source_revision,
            "missing workflow source revision",
        ),
        (&snapshot.workflow_source_digest, "missing workflow source digest"),
        (&snapshot.actor_ref, "missing performance actor"),
        (&snapshot.subject_ref, "missing performance subject"),
        (&snapshot.whole_ref, "missing performance whole"),
        (&snapshot.ql_binding_ref, "missing source QL binding"),
        (&snapshot.ql_binding_revision, "missing source QL binding revision"),
        (
            &snapshot.ai_kit_resolve_path_ref,
            "missing scoped AIKit Resolve path",
        ),
        (
            &snapshot.context_resolution_ref,
            "missing AIKit ContextResolution",
        ),
    ] {
        reference(value, message)?;
    }
    references(&snapshot.source_refs, "missing Factory source scope")?;
    require(
        snapshot.ql_binding_ref == request.ql_binding_ref
            && snapshot.ql_binding_revision == request.ql_binding_revision,
        "Factory snapshot names another QL binding/revision",
    )?;
    require(
        snapshot.subject_ref == profile.subject_ref && snapshot.whole_ref == profile.whole_use,
        "Factory performance subject/whole differs from the compiled QL profile",
    )?;
    require(
        snapshot.frame == profile.frame.id.code()
            && snapshot.thread == thread_code(profile.profile.thread)
            && snapshot.sequence == sequence_code(profile.profile.sequence)
            && snapshot.direction == direction_code(profile.profile.direction)
            && snapshot.musical_role == profile.profile.thread.musical_role(),
        "Factory C′ performance identity differs from the compiled QL semantics",
    )?;
    validate_attempts(snapshot)?;
    validate_chain(snapshot, profile.profile.thread)?;
    validate_sustained(snapshot, profile.profile.thread)
}

fn validate_attempts(snapshot: &FactoryVakPerformanceSnapshot) -> Result<()> {
    require(
        !snapshot.attempts.is_empty() && snapshot.attempts.len() <= MAX_ATTEMPTS,
        "performance requires bounded actual Factory attempts",
    )?;
    let mut execution_refs = BTreeSet::new();
    let mut attempts_by_unit: BTreeMap<&str, Vec<&FactoryVakAttempt>> = BTreeMap::new();
    for attempt in &snapshot.attempts {
        for (value, message) in [
            (&attempt.unit_ref, "missing WorkflowUnit reference"),
            (&attempt.execution_ref, "missing execution reference"),
            (&attempt.actor_ref, "missing attempt actor"),
            (&attempt.whole_ref, "missing attempt whole"),
            (&attempt.subject_ref, "missing attempt subject"),
            (&attempt.ql_binding_ref, "missing attempt QL binding"),
            (&attempt.ql_binding_revision, "missing attempt QL revision"),
            (
                &attempt.ai_kit_resolve_path_ref,
                "missing attempt Resolve path",
            ),
            (
                &attempt.context_resolution_ref,
                "missing attempt ContextResolution",
            ),
            (&attempt.model_ref, "missing attempt model"),
            (&attempt.provider_ref, "missing attempt provider"),
        ] {
            reference(value, message)?;
        }
        require(
            execution_refs.insert(attempt.execution_ref.as_str()),
            "execution identity reused across Factory attempts",
        )?;
        require(
            attempt.actor_ref == snapshot.actor_ref
                && attempt.subject_ref == snapshot.subject_ref
                && attempt.ql_binding_ref == snapshot.ql_binding_ref
                && attempt.ql_binding_revision == snapshot.ql_binding_revision,
            "attempt changed the semantic actor/subject/QL binding",
        )?;
        references(&attempt.source_refs, "attempt lost its source scope")?;
        require(
            attempt.source_refs.is_subset(&snapshot.source_refs),
            "child attempt widened the performance source scope",
        )?;
        require(
            !attempt.status_history.is_empty()
                && attempt.status_history.first() == Some(&FactoryLegStatus::Active)
                && attempt.status_history.last() == Some(&attempt.status),
            "attempt status history does not describe its actual occasion",
        )?;
        require(
            attempt.artifact_refs.is_disjoint(&attempt.late_artifact_refs),
            "current and late Return identities cannot be collapsed",
        )?;
        if let Some(reason) = &attempt.failure_reason {
            reference(reason, "empty Factory failure reason")?;
        }
        attempts_by_unit
            .entry(attempt.unit_ref.as_str())
            .or_default()
            .push(attempt);
    }
    for attempts in attempts_by_unit.values_mut() {
        attempts.sort_by_key(|attempt| attempt.attempt_index);
        for (expected, attempt) in attempts.iter().enumerate() {
            require(
                attempt.attempt_index == expected,
                "Factory retry history has a missing or duplicate attempt index",
            )?;
            require(
                attempt.current == (expected + 1 == attempts.len()),
                "exactly the latest Factory attempt must be current for its WorkflowUnit",
            )?;
        }
    }
    Ok(())
}

fn validate_chain(snapshot: &FactoryVakPerformanceSnapshot, thread: ThreadForm) -> Result<()> {
    if thread != ThreadForm::Chain {
        return require(
            snapshot.chain_inputs.is_empty(),
            "non-chain performance cannot claim predecessor material",
        );
    }
    let units = snapshot
        .attempts
        .iter()
        .map(|attempt| attempt.unit_ref.as_str())
        .collect::<BTreeSet<_>>();
    if units.len() > 1 {
        require(
            snapshot.chain_inputs.len() == units.len() - 1,
            "chain must expose one admitted predecessor material relation per continuation",
        )?;
    }
    let mut successors = BTreeSet::new();
    for material in &snapshot.chain_inputs {
        for (value, message) in [
            (&material.predecessor_unit_ref, "missing predecessor WorkflowUnit"),
            (
                &material.predecessor_execution_ref,
                "missing predecessor execution",
            ),
            (&material.successor_unit_ref, "missing successor WorkflowUnit"),
            (&material.subject_ref, "missing predecessor subject"),
            (&material.subject_revision, "missing predecessor subject revision"),
        ] {
            reference(value, message)?;
        }
        references(
            &material.artifact_refs,
            "chain continuation requires selected current Return material",
        )?;
        references(
            &material.evidence_refs,
            "chain continuation requires selected Return evidence",
        )?;
        require(
            !material.semantic_differences.is_empty()
                && material
                    .semantic_differences
                    .iter()
                    .all(|value| !value.trim().is_empty()),
            "chain continuation requires the returned semantic difference",
        )?;
        require(
            successors.insert(material.successor_unit_ref.as_str())
                && units.contains(material.predecessor_unit_ref.as_str())
                && units.contains(material.successor_unit_ref.as_str()),
            "chain material names duplicate or unknown units",
        )?;
        let predecessor = snapshot
            .attempts
            .iter()
            .find(|attempt| {
                attempt.unit_ref == material.predecessor_unit_ref
                    && attempt.execution_ref == material.predecessor_execution_ref
            })
            .ok_or_else(|| error("chain material names an unknown predecessor occasion"))?;
        require(
            predecessor.subject_ref == material.subject_ref
                && material.artifact_refs.is_subset(&predecessor.artifact_refs)
                && material.artifact_refs.is_disjoint(&predecessor.late_artifact_refs)
                && material.evidence_refs.is_subset(&predecessor.evidence_refs),
            "chain material was not selected from the predecessor's current Return",
        )?;
    }
    Ok(())
}

fn validate_sustained(snapshot: &FactoryVakPerformanceSnapshot, thread: ThreadForm) -> Result<()> {
    let Some(stop) = &snapshot.sustained_stop else {
        return Ok(());
    };
    require(
        thread == ThreadForm::Sustained,
        "only sustained work can carry a stop observation",
    )?;
    for (value, message) in [
        (&stop.stop_condition_ref, "missing sustained stop condition"),
        (&stop.native_stop_conditions, "missing native stop conditions"),
        (&stop.owner_ref, "missing stop observation owner"),
        (&stop.source_revision, "missing stop source revision"),
    ] {
        reference(value, message)?;
    }
    references(
        &stop.evidence_refs,
        "sustained stop requires attributable evidence",
    )
}

const fn musical_basis(basis: MusicalBasis) -> &'static str {
    match basis {
        MusicalBasis::Chromatic => "chromatic",
        MusicalBasis::Fifths => "fifths",
    }
}

const fn thread_code(thread: ThreadForm) -> &'static str {
    match thread {
        ThreadForm::Single => "CFP0",
        ThreadForm::Parallel => "CFP1",
        ThreadForm::Chain => "CFP2",
        ThreadForm::Fusion => "CFP3",
        ThreadForm::Sustained => "CFP4",
        ThreadForm::Nested => "CFP5",
    }
}

const fn sequence_code(sequence: ContextSequence) -> &'static str {
    match sequence {
        ContextSequence::Full => "CS0",
        ContextSequence::GroundContext => "CS1",
        ContextSequence::ThroughOperation => "CS2",
        ContextSequence::ThroughPattern => "CS3",
        ContextSequence::ContextFocused => "CS4",
        ContextSequence::DirectIntegration => "CS5",
    }
}

const fn direction_code(direction: InquiryDirection) -> &'static str {
    match direction {
        InquiryDirection::Forward => "forward",
        InquiryDirection::Returning => "returning",
    }
}

#[cfg(test)]
mod tests {
    use ql_core::{CallerProvenance, QlFace};

    use crate::{ContextFrameId, LensId};
    use crate::vak_composition::{ActiveFrame, Basis, PositionBasis};

    use super::*;

    fn basis() -> Basis {
        Basis {
            provenance: CallerProvenance::new("test/ql", PROFILE_SOURCE, "DERIVED").unwrap(),
            revision: "ql-source-r1".into(),
            evidence: vec!["evidence:ql-profile".into()],
        }
    }

    fn profile(thread: ThreadForm) -> CompiledProfile {
        CompiledProfile {
            contract: PROFILE_CONTRACT,
            whole_use: "whole:undertaking".into(),
            subject_ref: "subject:nara".into(),
            frame: ActiveFrame {
                id: ContextFrameId::Cf5,
                lens: LensId::L0,
                basis: MusicalBasis::Chromatic,
                face: QlFace::Direct,
                positions: PositionBasis::Local,
            },
            profile: crate::vak_profile::CPrimeProfile {
                participation: Participation::AuthorisedUndertaking,
                content: ContentType::Operations,
                position: ContentPosition::Operation,
                thread,
                sequence: ContextSequence::ThroughOperation,
                direction: InquiryDirection::Forward,
            },
            pairs: ContextSequence::ThroughOperation.pairs(),
            walk: ContextSequence::ThroughOperation.walk(InquiryDirection::Forward),
            basis: vec![basis()],
        }
    }

    fn attempt(
        unit: &str,
        index: usize,
        current: bool,
        execution: &str,
        status: FactoryLegStatus,
    ) -> FactoryVakAttempt {
        let artifact = format!("artifact:{execution}");
        FactoryVakAttempt {
            unit_ref: unit.into(),
            attempt_index: index,
            current,
            execution_ref: execution.into(),
            actor_ref: "agent:epii".into(),
            whole_ref: format!("whole:{unit}"),
            subject_ref: "subject:nara".into(),
            ql_binding_ref: "ql-binding:undertaking".into(),
            ql_binding_revision: "ql-r1".into(),
            ai_kit_resolve_path_ref: format!("resolve:{unit}"),
            context_resolution_ref: format!("context:{unit}"),
            source_refs: BTreeSet::from(["source:undertaking".into(), "source:ql".into()]),
            model_ref: format!("model:{execution}"),
            provider_ref: format!("provider:{execution}"),
            status,
            status_history: vec![FactoryLegStatus::Active, status],
            artifact_refs: if status == FactoryLegStatus::Returned {
                BTreeSet::from([artifact])
            } else {
                BTreeSet::new()
            },
            late_artifact_refs: BTreeSet::new(),
            evidence_refs: if status == FactoryLegStatus::Returned {
                BTreeSet::from([format!("evidence:{execution}")])
            } else {
                BTreeSet::new()
            },
            failure_reason: (status == FactoryLegStatus::Failed).then(|| "provider failed".into()),
        }
    }

    fn snapshot(thread: ThreadForm) -> FactoryVakPerformanceSnapshot {
        let first = attempt("inspect", 0, true, "exec-inspect", FactoryLegStatus::Returned);
        let first_artifact = first.artifact_refs.iter().next().unwrap().clone();
        let first_evidence = first.evidence_refs.iter().next().unwrap().clone();
        let second = attempt("implement", 0, true, "exec-implement", FactoryLegStatus::Returned);
        FactoryVakPerformanceSnapshot {
            contract: FACTORY_VAK_PERFORMANCE_CONTRACT.into(),
            performance_ref: "performance:factory-vak".into(),
            run_ref: "run:factory-vak".into(),
            run_revision: 9,
            workflow_source_ref: "workflow-source:factory".into(),
            workflow_source_revision: "factory-r7".into(),
            workflow_source_digest: "digest:factory".into(),
            actor_ref: "agent:epii".into(),
            subject_ref: "subject:nara".into(),
            whole_ref: "whole:undertaking".into(),
            ql_binding_ref: "ql-binding:undertaking".into(),
            ql_binding_revision: "ql-r1".into(),
            ai_kit_resolve_path_ref: "resolve:undertaking".into(),
            context_resolution_ref: "context:undertaking".into(),
            source_refs: BTreeSet::from(["source:undertaking".into(), "source:ql".into()]),
            frame: "CF5".into(),
            thread: thread_code(thread).into(),
            sequence: "CS2".into(),
            direction: "forward".into(),
            musical_role: thread.musical_role().into(),
            attempts: if thread == ThreadForm::Chain {
                vec![first, second]
            } else {
                vec![first]
            },
            chain_inputs: if thread == ThreadForm::Chain {
                vec![FactoryVakChainMaterial {
                    predecessor_unit_ref: "inspect".into(),
                    predecessor_execution_ref: "exec-inspect".into(),
                    successor_unit_ref: "implement".into(),
                    subject_ref: "subject:nara".into(),
                    subject_revision: "subject-r1".into(),
                    artifact_refs: BTreeSet::from([first_artifact]),
                    evidence_refs: BTreeSet::from([first_evidence]),
                    semantic_differences: BTreeSet::from(["source inspected".into()]),
                }]
            } else {
                Vec::new()
            },
            sustained_stop: None,
        }
    }

    fn request(thread: ThreadForm) -> PerformanceProjectionRequest {
        PerformanceProjectionRequest {
            schema: PERFORMANCE_PROJECTION_REQUEST.into(),
            ql_binding_ref: "ql-binding:undertaking".into(),
            ql_binding_revision: "ql-r1".into(),
            observation: PerformanceObservation {
                observation_ref: "observation:live".into(),
                mode: PerformanceObservationMode::Live,
                replay_of: None,
            },
            factory_receipt_refs: BTreeSet::from([
                "factory-receipt:attempt".into(),
                "factory-receipt:return".into(),
            ]),
            snapshot: snapshot(thread),
        }
    }

    #[test]
    fn actual_chain_performance_keeps_factory_evidence_and_derives_ql_music() {
        let event = profile(ThreadForm::Chain)
            .project_factory_performance(request(ThreadForm::Chain))
            .unwrap();
        assert_eq!(event.contract, PERFORMANCE_EVENT_CONTRACT);
        assert_eq!(event.performance_ref, "performance:factory-vak");
        assert_eq!(event.semantics.context_frame, "CF5");
        assert_eq!(event.semantics.constitutional_voice, "Anima");
        assert_eq!(event.semantics.thread, ThreadForm::Chain);
        assert_eq!(event.semantics.musical_role, "melody");
        assert_eq!(event.semantics.lens, "L0");
        assert_eq!(event.semantics.musical_basis, "chromatic");
        assert_eq!(event.semantics.frame_pitch, profile(ThreadForm::Chain).frame_pitch());
        assert!(event.settled);
        assert!(!event.has_failure);
        assert_eq!(event.factory.chain_inputs.len(), 1);
        assert!(event.ql_basis_refs.contains("factory-receipt:return"));
        assert_eq!(event.factory.attempts[0].execution_ref, "exec-inspect");
    }

    #[test]
    fn profile_mismatch_and_child_source_widening_fail_closed() {
        let compiled = profile(ThreadForm::Chain);
        let mut wrong = request(ThreadForm::Chain);
        wrong.snapshot.thread = "CFP1".into();
        assert!(compiled.project_factory_performance(wrong).is_err());

        let mut widened = request(ThreadForm::Chain);
        widened.snapshot.attempts[1]
            .source_refs
            .insert("source:not-in-parent".into());
        assert!(compiled.project_factory_performance(widened).is_err());
    }

    #[test]
    fn retries_keep_old_provider_and_late_return_without_relabelling_current_attempt() {
        let compiled = profile(ThreadForm::Sustained);
        let mut request = request(ThreadForm::Sustained);
        let mut old = attempt("observe", 0, false, "exec-old", FactoryLegStatus::Failed);
        old.late_artifact_refs.insert("artifact:old-late".into());
        old.evidence_refs.insert("evidence:old-late".into());
        let mut current = attempt("observe", 1, true, "exec-new", FactoryLegStatus::LateResult);
        current.late_artifact_refs.insert("artifact:new-late".into());
        current.evidence_refs.insert("evidence:new-late".into());
        request.snapshot.attempts = vec![old, current];
        request.snapshot.sustained_stop = Some(FactoryVakSustainedStop {
            stop_condition_ref: "stop:done".into(),
            native_stop_conditions: "explicit owner stop".into(),
            owner_ref: "central-now".into(),
            source_revision: "now-r4".into(),
            evidence_refs: BTreeSet::from(["evidence:stop".into()]),
        });
        let event = compiled.project_factory_performance(request).unwrap();
        assert!(event.settled);
        assert!(event.has_failure);
        assert!(event.has_late_return);
        assert_ne!(
            event.factory.attempts[0].provider_ref,
            event.factory.attempts[1].provider_ref
        );
        assert!(!event.factory.attempts[0].current);
        assert!(event.factory.attempts[1].current);
    }

    #[test]
    fn replay_is_an_observation_of_the_same_performance_not_a_new_event() {
        let compiled = profile(ThreadForm::Single);
        let mut replay = request(ThreadForm::Single);
        replay.observation = PerformanceObservation {
            observation_ref: "observation:replay".into(),
            mode: PerformanceObservationMode::Replay,
            replay_of: Some("performance:factory-vak".into()),
        };
        let event = compiled.project_factory_performance(replay).unwrap();
        assert_eq!(event.performance_ref, "performance:factory-vak");
        assert_eq!(event.observation.mode, PerformanceObservationMode::Replay);

        let mut forged = request(ThreadForm::Single);
        forged.observation.mode = PerformanceObservationMode::Replay;
        forged.observation.replay_of = Some("performance:another".into());
        assert!(compiled.project_factory_performance(forged).is_err());
    }

    #[test]
    fn chain_cannot_consume_late_or_unattributed_material() {
        let compiled = profile(ThreadForm::Chain);
        let mut late = request(ThreadForm::Chain);
        let selected = late.snapshot.chain_inputs[0].artifact_refs.clone();
        late.snapshot.attempts[0].artifact_refs.clear();
        late.snapshot.attempts[0]
            .late_artifact_refs
            .extend(selected);
        assert!(compiled.project_factory_performance(late).is_err());

        let mut foreign = request(ThreadForm::Chain);
        foreign.snapshot.chain_inputs[0].predecessor_execution_ref = "exec:foreign".into();
        assert!(compiled.project_factory_performance(foreign).is_err());
    }
}
