//! Protected multiple-Nara reception over one shared world occasion.
//!
//! Each Nara retains an independent `PersonalFieldInstance`. Shared presence is
//! composed only from explicitly consented Expression projections, never from a
//! bulk raw-personal snapshot.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::continuous::coupled::CoupledBasis;

use super::expression::{ExpressionDisclosure, NaraExpressionProjection, SharedPresenceConsent};
use super::{
    EventBasisRefs, PersonalConstitution, PersonalEventInput, PersonalFieldInstance,
    PersonalFieldState,
};

pub const NARA_MULTI_FIELD_CONTRACT: &str = "ql.nara-multi-field/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedWorldKey {
    pub event_ref: String,
    pub profile_generation: u64,
    pub registry_revision: String,
    pub m1_revision: String,
    pub m2_source_ref: String,
    pub m2_contract_ref: String,
    pub m3_source_ref: String,
    pub m3_contract_ref: String,
}

impl From<&EventBasisRefs> for SharedWorldKey {
    fn from(value: &EventBasisRefs) -> Self {
        Self {
            event_ref: value.event_ref.clone(),
            profile_generation: value.profile_generation,
            registry_revision: value.registry_revision.clone(),
            m1_revision: value.m1_revision.clone(),
            m2_source_ref: value.m2_source_ref.clone(),
            m2_contract_ref: value.m2_contract_ref.clone(),
            m3_source_ref: value.m3_source_ref.clone(),
            m3_contract_ref: value.m3_contract_ref.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProtectedNaraFieldSet {
    schema: &'static str,
    shared_world: Option<SharedWorldKey>,
    instances: BTreeMap<String, PersonalFieldInstance>,
}

impl Default for ProtectedNaraFieldSet {
    fn default() -> Self {
        Self {
            schema: NARA_MULTI_FIELD_CONTRACT,
            shared_world: None,
            instances: BTreeMap::new(),
        }
    }
}

impl ProtectedNaraFieldSet {
    pub fn schema(&self) -> &'static str {
        self.schema
    }

    pub fn shared_world(&self) -> Option<&SharedWorldKey> {
        self.shared_world.as_ref()
    }

    pub fn add_constitution(&mut self, constitution: PersonalConstitution) -> Result<(), String> {
        let subject_id = constitution.subject_id.clone();
        if self.instances.contains_key(&subject_id) {
            return Err("Nara subject already exists in protected field set".into());
        }
        self.instances
            .insert(subject_id, PersonalFieldInstance::new(constitution)?);
        Ok(())
    }

    pub fn subjects(&self) -> impl Iterator<Item = &str> {
        self.instances.keys().map(String::as_str)
    }

    /// Explicit per-subject read. There is intentionally no bulk raw-state read.
    pub fn current(&self, subject_id: &str) -> Option<&PersonalFieldState> {
        self.instances
            .get(subject_id)
            .and_then(PersonalFieldInstance::current)
    }

    fn check_world(&mut self, refs: &EventBasisRefs) -> Result<(), String> {
        let key = SharedWorldKey::from(refs);
        match &self.shared_world {
            None => {
                self.shared_world = Some(key);
                Ok(())
            }
            Some(existing) if existing == &key => Ok(()),
            Some(_) => Err("multi-Nara field set cannot mix different world occasions".into()),
        }
    }

    pub fn receive(
        &mut self,
        subject_id: &str,
        basis: &CoupledBasis,
        input: PersonalEventInput,
    ) -> Result<PersonalFieldState, String> {
        let refs = EventBasisRefs::from_basis(basis)?;
        if refs.subject_ref != subject_id {
            return Err("requested Nara does not match the subject-specific world basis".into());
        }
        self.check_world(&refs)?;
        self.instances
            .get_mut(subject_id)
            .ok_or("unknown Nara subject")?
            .receive(basis, input)
    }

    pub fn shared_presence(
        &self,
        projections: Vec<NaraExpressionProjection>,
        consent: &SharedPresenceConsent,
        at_unix_ms: u64,
    ) -> Result<Vec<NaraExpressionProjection>, String> {
        consent.validate()?;
        if projections.len() < 2 {
            return Err("shared multi-Nara presence requires at least two projections".into());
        }
        let mut subjects = BTreeSet::new();
        let mut shared = Vec::with_capacity(projections.len());
        for projection in projections {
            if !self.instances.contains_key(&projection.subject_id) {
                return Err("shared projection belongs to an unknown Nara".into());
            }
            subjects.insert(projection.subject_id.clone());
            let projection = projection.share_with(consent, at_unix_ms)?;
            if projection.disclosure != ExpressionDisclosure::SharedPresence {
                return Err("shared projection did not enter shared-presence disclosure".into());
            }
            shared.push(projection);
        }
        if subjects.len() < 2 {
            return Err("shared multi-Nara presence requires distinct subjects".into());
        }
        Ok(shared)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nara::{
        BioQuaternion, ConsentState, EarthBodyConstitution, LifecycleState, PersonalLayer,
        ReceiverConstitution, SourceRevision,
    };

    fn source(name: &str) -> SourceRevision {
        SourceRevision {
            source_ref: format!("source:{name}"),
            revision: "r1".into(),
            standing_ref: "controlled-test".into(),
        }
    }

    fn layer(name: &str) -> PersonalLayer {
        PersonalLayer {
            source: source(name),
            observed_at_unix_ms: 1,
            quaternion: BioQuaternion::IDENTITY,
        }
    }

    fn constitution(subject: &str, offset: f64) -> PersonalConstitution {
        PersonalConstitution {
            subject_id: subject.into(),
            constitution_ref: format!("constitution:{subject}"),
            source_revisions: vec![source("constitution")],
            consent: ConsentState::Granted,
            lifecycle: LifecycleState::Active,
            identity: layer("identity"),
            transit: layer("transit"),
            activity: layer("activity"),
            ephemeral: None,
            earth_body: EarthBodyConstitution {
                source: source("earth"),
                frame_ref: "earth-fixed".into(),
                orientation: BioQuaternion::IDENTITY,
            },
            receivers: (0..7)
                .map(|ordinal| ReceiverConstitution {
                    ordinal,
                    label: format!("centre-{ordinal}"),
                    source: source(&format!("centre-{ordinal}")),
                    world_weights: [1.0 + offset, 0.5, 0.25],
                    orientation: BioQuaternion::IDENTITY,
                    resonance_gain: 1.0,
                    reradiation_gain: 0.5,
                })
                .collect(),
        }
    }

    fn refs(subject: &str, event: &str) -> EventBasisRefs {
        EventBasisRefs {
            event_ref: event.into(),
            subject_ref: subject.into(),
            profile_generation: 1,
            registry_revision: "registry:r1".into(),
            m1_revision: "m1:r1".into(),
            m2_source_ref: "m2:source".into(),
            m2_contract_ref: "m2:contract".into(),
            m3_source_ref: "m3:source".into(),
            m3_contract_ref: "m3:contract".into(),
        }
    }

    #[test]
    fn distinct_naras_can_share_world_key_without_sharing_subject_identity() {
        let a = refs("nara-a", "event:1");
        let b = refs("nara-b", "event:1");
        assert_ne!(a.subject_ref, b.subject_ref);
        assert_eq!(SharedWorldKey::from(&a), SharedWorldKey::from(&b));
    }

    #[test]
    fn registry_keeps_constitutions_independent() {
        let mut set = ProtectedNaraFieldSet::default();
        set.add_constitution(constitution("nara-a", 0.0)).unwrap();
        set.add_constitution(constitution("nara-b", 1.0)).unwrap();
        assert_eq!(set.subjects().collect::<Vec<_>>(), vec!["nara-a", "nara-b"]);
        assert!(set.add_constitution(constitution("nara-a", 2.0)).is_err());
    }

    #[test]
    fn one_registry_refuses_mixed_world_occasions() {
        let mut set = ProtectedNaraFieldSet::default();
        set.check_world(&refs("nara-a", "event:1")).unwrap();
        set.check_world(&refs("nara-b", "event:1")).unwrap();
        assert!(set.check_world(&refs("nara-b", "event:2")).is_err());
    }
}
