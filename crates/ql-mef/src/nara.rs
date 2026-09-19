//! Subject-local Nara reception over an already accepted M1/M2/M3 world event.
//!
//! This module does not restart the world field, infer a person from renderer
//! state, or invent source mappings for seven centres. A constitution supplies
//! its seven independently sourced receiver mappings explicitly. The producer
//! validates those mappings against the exact coupled event and composes the
//! named bioquaternion relation `Q_identity · Q_transit · Q_activity`.
//! Numerical outputs are compositional readings, not health or clinical claims.

pub mod activity;
pub mod dialogue;
pub mod domain;
pub mod expression;
pub mod multi;
pub mod replay;
pub mod voice;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::continuous::coupled::CoupledBasis;

pub const PERSONAL_FIELD_CONTRACT: &str = "ql.nara-personal-field/v1";
pub const RECEIVER_COUNT: usize = 7;

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        Err(format!("invalid {label}"))
    } else {
        Ok(())
    }
}

fn finite(values: impl IntoIterator<Item = f64>) -> Result<(), String> {
    if values.into_iter().all(f64::is_finite) {
        Ok(())
    } else {
        Err("non-finite personal field value".into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BioQuaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BioQuaternion {
    pub const IDENTITY: Self = Self {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn normalized(self) -> Result<Self, String> {
        finite([self.w, self.x, self.y, self.z])?;
        let n2 = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
        if n2 <= f64::EPSILON {
            return Err("bioquaternion must have non-zero norm".into());
        }
        let n = n2.sqrt();
        Ok(Self {
            w: self.w / n,
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        })
    }

    pub fn compose(self, rhs: Self) -> Result<Self, String> {
        let a = self.normalized()?;
        let b = rhs.normalized()?;
        Self {
            w: a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
            x: a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
            y: a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
            z: a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
        }
        .normalized()
    }

    pub fn alignment(self, rhs: Self) -> Result<f64, String> {
        let a = self.normalized()?;
        let b = rhs.normalized()?;
        Ok((a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z).abs())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsentState {
    Granted,
    Withheld,
    Withdrawn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LifecycleState {
    Active,
    Held,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRevision {
    pub source_ref: String,
    pub revision: String,
    pub standing_ref: String,
}

impl SourceRevision {
    fn validate(&self) -> Result<(), String> {
        text(&self.source_ref, "source_ref")?;
        text(&self.revision, "source revision")?;
        text(&self.standing_ref, "source standing")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalLayer {
    pub source: SourceRevision,
    pub observed_at_unix_ms: u64,
    pub quaternion: BioQuaternion,
}

impl PersonalLayer {
    fn validate(&self) -> Result<(), String> {
        self.source.validate()?;
        self.quaternion.normalized().map(|_| ())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverConstitution {
    /// Source-defined receiver ordinal. V1 requires exactly 0..6 once each but
    /// does not assign a canonical chakra name that the recovered source lacks.
    pub ordinal: u8,
    pub label: String,
    pub source: SourceRevision,
    /// Explicit source/provider mapping from the accepted M1/M2/M3 contribution
    /// triple into this receiver. K10 never manufactures fallback weights.
    pub world_weights: [f64; 3],
    pub orientation: BioQuaternion,
    pub resonance_gain: f64,
    pub reradiation_gain: f64,
}

impl ReceiverConstitution {
    fn validate(&self) -> Result<(), String> {
        if usize::from(self.ordinal) >= RECEIVER_COUNT {
            return Err("receiver ordinal outside seven-centre field".into());
        }
        text(&self.label, "receiver label")?;
        self.source.validate()?;
        finite(self.world_weights)?;
        finite([self.resonance_gain, self.reradiation_gain])?;
        if self.resonance_gain < 0.0 || self.reradiation_gain < 0.0 {
            return Err("receiver gains must be non-negative".into());
        }
        self.orientation.normalized().map(|_| ())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EarthBodyConstitution {
    pub source: SourceRevision,
    pub frame_ref: String,
    pub orientation: BioQuaternion,
}

impl EarthBodyConstitution {
    fn validate(&self) -> Result<(), String> {
        self.source.validate()?;
        text(&self.frame_ref, "EarthBody frame")?;
        self.orientation.normalized().map(|_| ())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalConstitution {
    pub subject_id: String,
    pub constitution_ref: String,
    pub source_revisions: Vec<SourceRevision>,
    pub consent: ConsentState,
    pub lifecycle: LifecycleState,
    pub identity: PersonalLayer,
    pub transit: PersonalLayer,
    pub activity: PersonalLayer,
    /// Retained as an attributable layer in v1. It is deliberately not folded
    /// into the named three-term quaternion relation without a source contract.
    pub ephemeral: Option<PersonalLayer>,
    pub earth_body: EarthBodyConstitution,
    pub receivers: Vec<ReceiverConstitution>,
}

impl PersonalConstitution {
    pub fn validate(&self) -> Result<(), String> {
        text(&self.subject_id, "subject identity")?;
        text(&self.constitution_ref, "constitution reference")?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 256 {
            return Err("personal constitution needs 1..256 source revisions".into());
        }
        for source in &self.source_revisions {
            source.validate()?;
        }
        self.identity.validate()?;
        self.transit.validate()?;
        self.activity.validate()?;
        if let Some(layer) = &self.ephemeral {
            layer.validate()?;
        }
        self.earth_body.validate()?;
        if self.receivers.len() != RECEIVER_COUNT {
            return Err("personal constitution requires exactly seven receivers".into());
        }
        let mut ordinals = BTreeSet::new();
        let mut labels = BTreeSet::new();
        for receiver in &self.receivers {
            receiver.validate()?;
            if !ordinals.insert(receiver.ordinal) || !labels.insert(receiver.label.as_str()) {
                return Err("duplicate receiver ordinal or label".into());
            }
        }
        if ordinals != BTreeSet::from([0, 1, 2, 3, 4, 5, 6]) {
            return Err("seven receiver ordinals must be complete".into());
        }
        Ok(())
    }

    pub fn bioquaternion(&self) -> Result<BioQuaternion, String> {
        self.identity
            .quaternion
            .compose(self.transit.quaternion)?
            .compose(self.activity.quaternion)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventBasisRefs {
    pub event_ref: String,
    pub subject_ref: String,
    pub profile_generation: u64,
    pub registry_revision: String,
    pub m1_revision: String,
    pub m2_source_ref: String,
    pub m2_contract_ref: String,
    pub m3_source_ref: String,
    pub m3_contract_ref: String,
}

impl EventBasisRefs {
    pub fn from_basis(basis: &CoupledBasis) -> Result<Self, String> {
        let m1 = &basis.input.m1;
        let m2 = &basis.m2_input;
        let m3 = &basis.input.m3;
        let event = &m2.stamp.identity;
        if m1.event_ref != event.event_ref || m3.stamp.identity != *event {
            return Err("Nara requires one accepted M1/M2/M3 event identity".into());
        }
        if m2.registry_revision != m3.registry_revision {
            return Err("M2/M3 registry revisions differ".into());
        }
        for (value, label) in [
            (&m3.subject_ref, "M3 subject"),
            (&m1.revision, "M1 revision"),
            (&m2.stamp.source_ref, "M2 source"),
            (&m2.stamp.contract_ref, "M2 contract"),
            (&m3.stamp.source_ref, "M3 source"),
            (&m3.stamp.contract_ref, "M3 contract"),
            (&m2.registry_revision, "registry revision"),
        ] {
            text(value, label)?;
        }
        Ok(Self {
            event_ref: event.event_ref.clone(),
            subject_ref: m3.subject_ref.clone(),
            profile_generation: event.profile_generation,
            registry_revision: m2.registry_revision.clone(),
            m1_revision: m1.revision.clone(),
            m2_source_ref: m2.stamp.source_ref.clone(),
            m2_contract_ref: m2.stamp.contract_ref.clone(),
            m3_source_ref: m3.stamp.source_ref.clone(),
            m3_contract_ref: m3.stamp.contract_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldContribution {
    pub basis_ref: String,
    pub source_ref: String,
    pub value: f64,
}

impl WorldContribution {
    fn validate(&self, expected_basis: &str) -> Result<(), String> {
        text(&self.basis_ref, "world contribution basis")?;
        text(&self.source_ref, "world contribution source")?;
        finite([self.value])?;
        if self.basis_ref != expected_basis {
            return Err(
                "world contribution is not attributable to the accepted event basis".into(),
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverEventInput {
    pub ordinal: u8,
    pub m1: WorldContribution,
    pub m2: WorldContribution,
    pub m3: WorldContribution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalEventInput {
    pub event_ref: String,
    pub profile_generation: u64,
    pub observed_at_unix_ms: u64,
    pub receivers: Vec<ReceiverEventInput>,
}

impl PersonalEventInput {
    fn validate(&self, refs: &EventBasisRefs) -> Result<(), String> {
        text(&self.event_ref, "personal event reference")?;
        if self.event_ref != refs.event_ref || self.profile_generation != refs.profile_generation {
            return Err("personal reception is stale or cross-event".into());
        }
        if self.receivers.len() != RECEIVER_COUNT {
            return Err("personal event requires seven receiver inputs".into());
        }
        let mut ordinals = BTreeSet::new();
        for receiver in &self.receivers {
            if usize::from(receiver.ordinal) >= RECEIVER_COUNT || !ordinals.insert(receiver.ordinal)
            {
                return Err("duplicate or invalid receiver event input".into());
            }
            receiver.m1.validate(&refs.m1_revision)?;
            receiver.m2.validate(&refs.m2_source_ref)?;
            receiver.m3.validate(&refs.m3_source_ref)?;
        }
        if ordinals != BTreeSet::from([0, 1, 2, 3, 4, 5, 6]) {
            return Err("seven receiver event inputs must be complete".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiverState {
    pub ordinal: u8,
    pub label: String,
    pub source: SourceRevision,
    pub input_basis: [WorldContribution; 3],
    pub bioquaternion: BioQuaternion,
    pub receiver_orientation: BioQuaternion,
    pub composed_orientation: BioQuaternion,
    pub orientation_alignment: f64,
    pub drive: f64,
    pub resonance: f64,
    pub reradiation: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EarthBodyState {
    pub source: SourceRevision,
    pub frame_ref: String,
    pub orientation: BioQuaternion,
    pub relation_alignment: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalFieldState {
    pub schema: String,
    pub subject_id: String,
    pub constitution_ref: String,
    pub reception_generation: u64,
    pub event: EventBasisRefs,
    pub observed_at_unix_ms: u64,
    pub consent: ConsentState,
    pub lifecycle: LifecycleState,
    pub q_identity: BioQuaternion,
    pub q_transit: BioQuaternion,
    pub q_activity: BioQuaternion,
    pub q_composed: BioQuaternion,
    pub ephemeral_source: Option<SourceRevision>,
    pub receivers: Vec<ReceiverState>,
    pub earth_body: EarthBodyState,
    pub aggregate_resonance: f64,
    pub aggregate_reradiation: f64,
    pub source_revisions: Vec<SourceRevision>,
    pub standing: String,
}

#[derive(Debug, Clone)]
pub struct PersonalFieldInstance {
    constitution: PersonalConstitution,
    generation: u64,
    last_event: Option<PersonalEventInput>,
    last_state: Option<PersonalFieldState>,
}

impl PersonalFieldInstance {
    pub fn new(constitution: PersonalConstitution) -> Result<Self, String> {
        constitution.validate()?;
        Ok(Self {
            constitution,
            generation: 0,
            last_event: None,
            last_state: None,
        })
    }

    pub fn subject_id(&self) -> &str {
        &self.constitution.subject_id
    }

    pub fn constitution(&self) -> &PersonalConstitution {
        &self.constitution
    }

    pub fn current(&self) -> Option<&PersonalFieldState> {
        self.last_state.as_ref()
    }

    pub fn receive(
        &mut self,
        basis: &CoupledBasis,
        input: PersonalEventInput,
    ) -> Result<PersonalFieldState, String> {
        if self.constitution.consent != ConsentState::Granted {
            return Err("personal reception requires granted consent".into());
        }
        if self.constitution.lifecycle != LifecycleState::Active {
            return Err("personal field instance is not active".into());
        }
        let refs = EventBasisRefs::from_basis(basis)?;
        if self.constitution.subject_id != refs.subject_ref {
            return Err("personal constitution subject does not match accepted world event".into());
        }
        input.validate(&refs)?;
        if let Some(previous) = &self.last_event {
            if previous.event_ref == input.event_ref
                && previous.profile_generation == input.profile_generation
            {
                if previous == &input {
                    return self
                        .last_state
                        .clone()
                        .ok_or_else(|| "personal replay state missing".into());
                }
                return Err("conflicting replay for the same world-event generation".into());
            }
            if previous.event_ref == input.event_ref
                && input.profile_generation < previous.profile_generation
            {
                return Err("stale personal world-event generation".into());
            }
        }
        let q_identity = self.constitution.identity.quaternion.normalized()?;
        let q_transit = self.constitution.transit.quaternion.normalized()?;
        let q_activity = self.constitution.activity.quaternion.normalized()?;
        let q_composed = self.constitution.bioquaternion()?;
        let mut receivers = Vec::with_capacity(RECEIVER_COUNT);
        let mut aggregate_resonance = 0.0;
        let mut aggregate_reradiation = 0.0;
        for constitution in &self.constitution.receivers {
            let event = input
                .receivers
                .iter()
                .find(|candidate| candidate.ordinal == constitution.ordinal)
                .ok_or("missing receiver event input")?;
            let values = [event.m1.value, event.m2.value, event.m3.value];
            let drive = constitution
                .world_weights
                .iter()
                .zip(values)
                .map(|(weight, value)| weight * value)
                .sum::<f64>();
            let orientation = constitution.orientation.normalized()?;
            let alignment = q_composed.alignment(orientation)?;
            let resonance = drive * constitution.resonance_gain * alignment;
            let reradiation = resonance * constitution.reradiation_gain;
            finite([drive, alignment, resonance, reradiation])?;
            aggregate_resonance += resonance;
            aggregate_reradiation += reradiation;
            receivers.push(ReceiverState {
                ordinal: constitution.ordinal,
                label: constitution.label.clone(),
                source: constitution.source.clone(),
                input_basis: [event.m1.clone(), event.m2.clone(), event.m3.clone()],
                bioquaternion: q_composed,
                receiver_orientation: orientation,
                composed_orientation: q_composed.compose(orientation)?,
                orientation_alignment: alignment,
                drive,
                resonance,
                reradiation,
            });
        }
        receivers.sort_by_key(|receiver| receiver.ordinal);
        finite([aggregate_resonance, aggregate_reradiation])?;
        let earth_orientation = self.constitution.earth_body.orientation.normalized()?;
        let earth_body = EarthBodyState {
            source: self.constitution.earth_body.source.clone(),
            frame_ref: self.constitution.earth_body.frame_ref.clone(),
            orientation: earth_orientation,
            relation_alignment: q_composed.alignment(earth_orientation)?,
        };
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or("personal reception generation overflow")?;
        let state = PersonalFieldState {
            schema: PERSONAL_FIELD_CONTRACT.into(),
            subject_id: self.constitution.subject_id.clone(),
            constitution_ref: self.constitution.constitution_ref.clone(),
            reception_generation: self.generation,
            event: refs,
            observed_at_unix_ms: input.observed_at_unix_ms,
            consent: self.constitution.consent.clone(),
            lifecycle: self.constitution.lifecycle.clone(),
            q_identity,
            q_transit,
            q_activity,
            q_composed,
            ephemeral_source: self
                .constitution
                .ephemeral
                .as_ref()
                .map(|layer| layer.source.clone()),
            receivers,
            earth_body,
            aggregate_resonance,
            aggregate_reradiation,
            source_revisions: self.constitution.source_revisions.clone(),
            standing: "subject-local composition over an accepted shared world event; supplied receiver mappings are attributable inputs, not inferred physiology".into(),
        };
        self.last_event = Some(input);
        self.last_state = Some(state.clone());
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(name: &str) -> SourceRevision {
        SourceRevision {
            source_ref: format!("source:{name}"),
            revision: "rev-1".into(),
            standing_ref: "controlled-test-source".into(),
        }
    }

    fn layer(name: &str, quaternion: BioQuaternion) -> PersonalLayer {
        PersonalLayer {
            source: source(name),
            observed_at_unix_ms: 1,
            quaternion,
        }
    }

    fn constitution(subject: &str, first_weight: f64) -> PersonalConstitution {
        PersonalConstitution {
            subject_id: subject.into(),
            constitution_ref: format!("constitution:{subject}"),
            source_revisions: vec![source("personal-basis")],
            consent: ConsentState::Granted,
            lifecycle: LifecycleState::Active,
            identity: layer("identity", BioQuaternion::IDENTITY),
            transit: layer(
                "transit",
                BioQuaternion {
                    w: 1.0,
                    x: 0.2,
                    y: 0.0,
                    z: 0.0,
                },
            ),
            activity: layer(
                "activity",
                BioQuaternion {
                    w: 1.0,
                    x: 0.0,
                    y: 0.1,
                    z: 0.0,
                },
            ),
            ephemeral: Some(layer("ephemeral", BioQuaternion::IDENTITY)),
            earth_body: EarthBodyConstitution {
                source: source("earth-body"),
                frame_ref: "earth-fixed-test-frame".into(),
                orientation: BioQuaternion::IDENTITY,
            },
            receivers: (0..RECEIVER_COUNT)
                .map(|ordinal| ReceiverConstitution {
                    ordinal: ordinal as u8,
                    label: format!("source-centre-{ordinal}"),
                    source: source(&format!("centre-{ordinal}")),
                    world_weights: [if ordinal == 0 { first_weight } else { 1.0 }, 0.5, 0.25],
                    orientation: BioQuaternion {
                        w: 1.0,
                        x: ordinal as f64 / 10.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    resonance_gain: 1.0,
                    reradiation_gain: 0.5,
                })
                .collect(),
        }
    }

    #[test]
    fn quaternion_order_is_identity_transit_activity() {
        let c = constitution("nara-a", 1.0);
        let expected = c
            .identity
            .quaternion
            .compose(c.transit.quaternion)
            .unwrap()
            .compose(c.activity.quaternion)
            .unwrap();
        assert_eq!(c.bioquaternion().unwrap(), expected);
    }

    #[test]
    fn receiver_source_mapping_is_complete_and_not_defaulted() {
        let mut c = constitution("nara-a", 1.0);
        c.receivers.pop();
        assert_eq!(
            c.validate().unwrap_err(),
            "personal constitution requires exactly seven receivers"
        );
        let mut c = constitution("nara-a", 1.0);
        c.receivers[6].ordinal = 0;
        assert_eq!(
            c.validate().unwrap_err(),
            "duplicate receiver ordinal or label"
        );
    }

    #[test]
    fn distinct_constitutions_are_materially_distinct_before_world_reception() {
        let a = constitution("nara-a", 1.0);
        let b = constitution("nara-b", 2.0);
        a.validate().unwrap();
        b.validate().unwrap();
        assert_ne!(a.subject_id, b.subject_id);
        assert_ne!(a.receivers[0].world_weights, b.receivers[0].world_weights);
    }

    #[test]
    fn consent_and_lifecycle_are_not_implied_by_identity() {
        let mut c = constitution("nara-a", 1.0);
        c.consent = ConsentState::Withdrawn;
        let instance = PersonalFieldInstance::new(c).unwrap();
        assert_eq!(instance.subject_id(), "nara-a");
        assert_eq!(instance.constitution().consent, ConsentState::Withdrawn);
    }
}
