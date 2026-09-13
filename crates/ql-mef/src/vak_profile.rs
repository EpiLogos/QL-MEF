//! Original C-prime interpretation over the existing VakComposition owner.
//!
//! This module compiles content, position, paired passage and native thread
//! obligations. It neither parses a second expression language nor schedules
//! work. A valid plan is not an execution receipt or an authority grant.
//! Native owners must admit, perform and return the corresponding operation.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::ContextFrameId;
use crate::vak_composition::{ActiveFrame, Basis, CompositionError, Result, VakComposition};

pub const PROFILE_CONTRACT: &str = "ql.vak-composition.profile/v1";
pub const PROFILE_SOURCE: &str = "docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md";
pub const PROFILE_SOURCE_BLOB: &str = "09f7d29ad6262f85bc2858f7c468f22d0bd398f3";
pub const PAIRED_PASSAGE_SOURCE_BLOB: &str = "b51a9a3e9e2866a5fcd56de4738c41d95d99d8d8";
const MAX_LEGS: usize = 4096;
const MAX_DEPTH: usize = 64;

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(CompositionError(message.into()))
    }
}
fn valid_ref(value: &str) -> Result<()> {
    ensure(
        !value.trim().is_empty() && value.len() <= 16384,
        "missing or excessive native reference",
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Participation {
    Dialogical,
    AuthorisedUndertaking,
}
impl Participation {
    /// The native owner enforces this requirement. Neither a selected CF nor
    /// a named/replayed method is itself an authorisation to act unattended.
    pub const fn requires_undertaking(self) -> bool {
        matches!(self, Self::AuthorisedUndertaking)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "CT0")]
    Assumptions,
    #[serde(rename = "CT1")]
    Definitions,
    #[serde(rename = "CT2")]
    Operations,
    #[serde(rename = "CT3")]
    Patterns,
    #[serde(rename = "CT4")]
    Contexts,
    #[serde(rename = "CT4b'")]
    DayNow,
    #[serde(rename = "CT5")]
    Integration,
}
impl ContentType {
    pub const ALL: [Self; 7] = [
        Self::Assumptions,
        Self::Definitions,
        Self::Operations,
        Self::Patterns,
        Self::Contexts,
        Self::DayNow,
        Self::Integration,
    ];
    /// CT4b' specialises contextual content; it is not a new civil-time owner.
    pub fn accepts(self, actual: Self) -> bool {
        self == actual || (self == Self::Contexts && actual == Self::DayNow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentPosition {
    #[serde(rename = "4.0")]
    Ground,
    #[serde(rename = "4.1")]
    Definition,
    #[serde(rename = "4.2")]
    Operation,
    #[serde(rename = "4.3")]
    Pattern,
    #[serde(rename = "4.4")]
    Context,
    #[serde(rename = "4.5")]
    Integration,
}
impl ContentPosition {
    pub const ALL: [Self; 6] = [
        Self::Ground,
        Self::Definition,
        Self::Operation,
        Self::Pattern,
        Self::Context,
        Self::Integration,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InquiryDirection {
    Forward,
    Returning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextSequence {
    #[serde(rename = "CS0")]
    Full,
    #[serde(rename = "CS1")]
    GroundContext,
    #[serde(rename = "CS2")]
    ThroughOperation,
    #[serde(rename = "CS3")]
    ThroughPattern,
    #[serde(rename = "CS4")]
    ContextFocused,
    #[serde(rename = "CS5")]
    DirectIntegration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositionPair {
    pub forward: ContentPosition,
    pub returning: ContentPosition,
}
impl ContextSequence {
    pub const ALL: [Self; 6] = [
        Self::Full,
        Self::GroundContext,
        Self::ThroughOperation,
        Self::ThroughPattern,
        Self::ContextFocused,
        Self::DirectIntegration,
    ];
    pub fn pairs(self) -> Vec<PositionPair> {
        let indices: &[(usize, usize)] = match self {
            Self::Full => &[(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)],
            Self::GroundContext => &[(0, 5), (1, 4)],
            Self::ThroughOperation => &[(0, 5), (1, 4), (2, 3)],
            Self::ThroughPattern => &[(0, 5), (1, 4), (2, 3), (3, 2)],
            Self::ContextFocused => &[(0, 5), (4, 1), (5, 0)],
            Self::DirectIntegration => &[(0, 5), (5, 0)],
        };
        indices
            .iter()
            .map(|&(a, b)| PositionPair {
                forward: ContentPosition::ALL[a],
                returning: ContentPosition::ALL[b],
            })
            .collect()
    }
    /// Select a side of each retained pair, never reverse the textual list.
    /// Direction and extent remain independent of wall-clock time/session ID.
    pub fn walk(self, direction: InquiryDirection) -> Vec<ContentPosition> {
        self.pairs()
            .into_iter()
            .map(|pair| match direction {
                InquiryDirection::Forward => pair.forward,
                InquiryDirection::Returning => pair.returning,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadForm {
    #[serde(rename = "CFP0")]
    Single,
    #[serde(rename = "CFP1")]
    Parallel,
    #[serde(rename = "CFP2")]
    Chain,
    #[serde(rename = "CFP3")]
    Fusion,
    #[serde(rename = "CFP4")]
    Sustained,
    #[serde(rename = "CFP5")]
    Nested,
}
impl ThreadForm {
    pub const ALL: [Self; 6] = [
        Self::Single,
        Self::Parallel,
        Self::Chain,
        Self::Fusion,
        Self::Sustained,
        Self::Nested,
    ];
    pub const fn musical_role(self) -> &'static str {
        match self {
            Self::Single => "single-voice",
            Self::Parallel => "chord",
            Self::Chain => "melody",
            Self::Fusion => "fusion",
            Self::Sustained => "drone",
            Self::Nested => "canon",
        }
    }
}

/// Native planned identities, not invented QL execution units. Actual attempts
/// and returned artifacts remain the native owner's records, including failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlannedLeg {
    pub unit_ref: String,
    pub subject_ref: String,
    pub scope_ref: String,
    pub input_refs: Vec<String>,
    pub result_ref: String,
    pub after: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThreadPlan {
    pub legs: Vec<PlannedLeg>,
    pub aggregation_ref: Option<String>,
    pub continuation_ref: Option<String>,
    pub stop_condition_ref: Option<String>,
}
impl ThreadForm {
    pub fn validate_plan(self, concern: &str, plan: &ThreadPlan) -> Result<()> {
        valid_ref(concern)?;
        ensure(
            !plan.legs.is_empty() && plan.legs.len() <= MAX_LEGS,
            "native thread requires a bounded nonempty leg set",
        )?;
        let mut units = BTreeMap::new();
        let mut results = BTreeSet::new();
        for (index, leg) in plan.legs.iter().enumerate() {
            for value in [
                &leg.unit_ref,
                &leg.subject_ref,
                &leg.scope_ref,
                &leg.result_ref,
            ] {
                valid_ref(value)?;
            }
            ensure(
                leg.input_refs.len() <= MAX_LEGS && leg.after.len() <= MAX_LEGS,
                "native dependency/input bound exceeded",
            )?;
            for value in leg.input_refs.iter().chain(leg.after.iter()) {
                valid_ref(value)?;
            }
            ensure(
                units.insert(leg.unit_ref.as_str(), index).is_none()
                    && results.insert(leg.result_ref.as_str()),
                "duplicate unit or aliased sibling result identity",
            )?;
        }
        for leg in &plan.legs {
            let mut predecessors = BTreeSet::new();
            for predecessor in &leg.after {
                ensure(
                    predecessor != &leg.unit_ref
                        && units.contains_key(predecessor.as_str())
                        && predecessors.insert(predecessor),
                    "unknown, duplicate or self predecessor",
                )?;
            }
            if let Some(parent) = &leg.parent {
                ensure(
                    parent != &leg.unit_ref && units.contains_key(parent.as_str()),
                    "unknown or self parent unit",
                )?;
            }
        }
        // Preserve a DAG even when the owner orders completion differently.
        let mut remaining: BTreeSet<&str> = units.keys().copied().collect();
        while !remaining.is_empty() {
            let ready: Vec<&str> = remaining
                .iter()
                .copied()
                .filter(|id| {
                    plan.legs[units[id]]
                        .after
                        .iter()
                        .all(|dependency| !remaining.contains(dependency.as_str()))
                })
                .collect();
            ensure(!ready.is_empty(), "cyclic native dependency plan")?;
            for id in ready {
                remaining.remove(id);
            }
        }
        for leg in &plan.legs {
            let mut cursor = leg;
            let mut seen = BTreeSet::new();
            while let Some(parent) = &cursor.parent {
                ensure(
                    seen.insert(parent) && seen.len() <= MAX_DEPTH,
                    "cyclic or excessive nested thread depth",
                )?;
                cursor = &plan.legs[units[parent.as_str()]];
            }
        }
        let independent = plan.legs.iter().all(|leg| leg.after.is_empty());
        let unnested = plan.legs.iter().all(|leg| leg.parent.is_none());
        match self {
            Self::Single => ensure(
                plan.legs.len() == 1 && independent && unnested,
                "single voice requires one unnested leg",
            )?,
            Self::Parallel => ensure(
                plan.legs.len() >= 2 && independent && unnested,
                "parallel voices must be independently runnable",
            )?,
            Self::Chain => {
                ensure(
                    plan.legs.len() >= 2 && unnested && plan.legs[0].after.is_empty(),
                    "chain requires an unnested source leg and continuation",
                )?;
                for (index, leg) in plan.legs.iter().enumerate().skip(1) {
                    let previous = &plan.legs[index - 1];
                    ensure(
                        leg.after.contains(&previous.unit_ref)
                            && leg.input_refs.contains(&previous.result_ref)
                            && leg.after.iter().all(|id| units[id.as_str()] < index),
                        "chain must consume its predecessor's identified return",
                    )?;
                }
            }
            Self::Fusion => {
                ensure(
                    plan.legs.len() >= 2
                        && independent
                        && unnested
                        && plan.legs.iter().all(|leg| leg.subject_ref == concern),
                    "fusion requires distinct readings of the same concern",
                )?;
                let aggregation = plan.aggregation_ref.as_deref().ok_or_else(|| {
                    CompositionError("fusion requires an explicit aggregation".into())
                })?;
                valid_ref(aggregation)?;
                ensure(
                    !results.contains(aggregation),
                    "aggregation must not overwrite an individual voice",
                )?;
            }
            Self::Sustained => {
                ensure(
                    plan.legs.len() == 1 && independent && unnested,
                    "sustained work retains one native undertaking",
                )?;
                for value in [&plan.continuation_ref, &plan.stop_condition_ref] {
                    valid_ref(value.as_deref().ok_or_else(|| {
                        CompositionError("sustained work needs resume and stop contracts".into())
                    })?)?;
                }
            }
            Self::Nested => ensure(
                plan.legs.len() >= 2
                    && plan.legs.iter().filter(|leg| leg.parent.is_none()).count() == 1,
                "nested work requires one root and explicitly parented children",
            )?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CPrimeProfile {
    pub participation: Participation,
    pub content: ContentType,
    pub position: ContentPosition,
    pub thread: ThreadForm,
    pub sequence: ContextSequence,
    pub direction: InquiryDirection,
}

/// A derived view of an existing immutable whole. It is deliberately not a
/// persisted second Run/World or a replacement for the original FullVakBinding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledProfile {
    pub contract: &'static str,
    pub whole_use: String,
    pub subject_ref: String,
    pub frame: ActiveFrame,
    pub profile: CPrimeProfile,
    pub pairs: Vec<PositionPair>,
    pub walk: Vec<ContentPosition>,
    pub basis: Vec<Basis>,
}
impl CompiledProfile {
    pub fn validate_plan(&self, plan: &ThreadPlan) -> Result<()> {
        self.profile.thread.validate_plan(&self.subject_ref, plan)
    }
    /// Uses the retained M1/M2 musical owner through ActiveFrame, not a second
    /// C/D/E/F/G/A/B lookup table. Native timing, attempts and audio are external.
    pub fn frame_pitch(&self) -> u8 {
        self.frame.pitch()
    }
}
impl VakComposition {
    pub fn compile_profile(
        &self,
        whole_use: &str,
        profile: CPrimeProfile,
    ) -> Result<CompiledProfile> {
        let whole = self.whole(whole_use)?;
        for basis in &whole.basis {
            basis.validate()?;
        }
        Ok(CompiledProfile {
            contract: PROFILE_CONTRACT,
            whole_use: whole.use_ref.clone(),
            subject_ref: whole.binding.subject_ref.clone(),
            frame: whole.frame,
            pairs: profile.sequence.pairs(),
            walk: profile.sequence.walk(profile.direction),
            profile,
            basis: whole.basis.clone(),
        })
    }
}

/// Constitutional roles qualify an Epi act; they are neither six new M agents
/// nor universal identities imposed on native O:I agents.
pub const fn constitutional_voice(frame: ContextFrameId) -> &'static str {
    match frame {
        ContextFrameId::Cf1 => "Nous",
        ContextFrameId::Cf2 => "Logos",
        ContextFrameId::Cf3 => "Eros",
        ContextFrameId::Cf4 => "Mythos",
        ContextFrameId::Cf5 => "Anima",
        ContextFrameId::Cf6 => "Psyche",
        ContextFrameId::Cf7 => "Sophia",
    }
}

#[cfg(test)]
mod tests;
