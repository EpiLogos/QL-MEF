//! AW2 currentness join for the K9 focused instrument.
//!
//! AW2 remains the semantic owner of operative C-prime scope and currentness.
//! K9 only validates the producer envelope and projects its explicit
//! Current/Stale/Missing standing for presentation/host consumers. The wrappers
//! here retain that owner observation through focused snapshots and operation
//! Returns without performing another reobservation in K9.

use serde::{Deserialize, Serialize};

use crate::focused_instrument::{FocusedInstrumentSnapshot, InstrumentOperationObservation};
use crate::vak_scope::{
    C_PRIME_INTERPRETATION_REF, OPERATIVE_OWNER_REF, OPERATIVE_PROVIDER_REF,
    OperativeScopeObservation,
};
use crate::vak_scope_wire::{
    OPERATIVE_CURRENTNESS_CONTRACT, OperativeScopeCurrentnessResponse,
};

pub const FOCUSED_OPERATIVE_CURRENTNESS_CONTRACT: &str =
    "ql.focused-instrument-operative-currentness/v1";
pub const SOURCE_QUALIFIED_SNAPSHOT_CONTRACT: &str =
    "ql.focused-instrument-source-qualified-snapshot/v1";
pub const SOURCE_QUALIFIED_RETURN_CONTRACT: &str =
    "ql.focused-instrument-source-qualified-return/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FocusedOperativeStanding {
    Current,
    Stale,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FocusedOperativeCurrentness {
    pub contract: String,
    pub standing: FocusedOperativeStanding,
    pub requested_binding_ref: String,
    pub requested_binding_revision: String,
    pub current_whole_ref: String,
    #[serde(default)]
    pub differences: Vec<String>,
    pub owner_response: OperativeScopeCurrentnessResponse,
}

impl FocusedOperativeCurrentness {
    pub fn from_aw2(response: OperativeScopeCurrentnessResponse) -> Result<Self, String> {
        validate_owner_response(&response)?;
        let (standing, differences) = match &response.observation {
            OperativeScopeObservation::Current { binding } => {
                if binding.binding_ref != response.requested_binding_ref
                    || binding.binding_revision != response.requested_binding_revision
                {
                    return Err(
                        "AW2 Current observation disagrees with the requested binding identity"
                            .into(),
                    );
                }
                (FocusedOperativeStanding::Current, Vec::new())
            }
            OperativeScopeObservation::Stale { differences, .. } => {
                if differences.is_empty() {
                    return Err("AW2 Stale observation must disclose its differences".into());
                }
                (FocusedOperativeStanding::Stale, differences.clone())
            }
            OperativeScopeObservation::Missing { binding_ref, .. } => {
                if binding_ref != &response.requested_binding_ref {
                    return Err("AW2 Missing observation names another binding".into());
                }
                (FocusedOperativeStanding::Missing, Vec::new())
            }
        };
        Ok(Self {
            contract: FOCUSED_OPERATIVE_CURRENTNESS_CONTRACT.into(),
            standing,
            requested_binding_ref: response.requested_binding_ref.clone(),
            requested_binding_revision: response.requested_binding_revision.clone(),
            current_whole_ref: response.current_whole_ref.clone(),
            differences,
            owner_response: response,
        })
    }

    pub const fn is_current(&self) -> bool {
        matches!(self.standing, FocusedOperativeStanding::Current)
    }

    /// Attach the already-produced AW2 observation to a K9 snapshot. This does
    /// not make the observation fresher; it only keeps the owner relation visible
    /// through the focused presentation aperture.
    pub fn retain_in_snapshot(
        &self,
        snapshot: FocusedInstrumentSnapshot,
    ) -> Result<SourceQualifiedFocusedSnapshot, String> {
        if let Some(subject) = observed_subject(&self.owner_response.observation)
            && subject != snapshot.event.subject_ref
        {
            return Err("AW2 currentness and focused snapshot name different subjects".into());
        }
        Ok(SourceQualifiedFocusedSnapshot {
            contract: SOURCE_QUALIFIED_SNAPSHOT_CONTRACT.into(),
            instrument: snapshot,
            operative_currentness: self.clone(),
        })
    }

    /// Retain the same owner observation beside an operation Return. This is
    /// explicitly the observation supplied at the operation aperture, not a
    /// post-effect reobservation or an assertion that the Return made it current.
    pub fn retain_in_return(
        &self,
        operation: InstrumentOperationObservation,
    ) -> SourceQualifiedOperationReturn {
        SourceQualifiedOperationReturn {
            contract: SOURCE_QUALIFIED_RETURN_CONTRACT.into(),
            currentness_aperture: "supplied-owner-observation-not-post-effect-reobservation".into(),
            operation,
            operative_currentness: self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceQualifiedFocusedSnapshot {
    pub contract: String,
    pub instrument: FocusedInstrumentSnapshot,
    pub operative_currentness: FocusedOperativeCurrentness,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceQualifiedOperationReturn {
    pub contract: String,
    pub currentness_aperture: String,
    pub operation: InstrumentOperationObservation,
    pub operative_currentness: FocusedOperativeCurrentness,
}

fn observed_subject(observation: &OperativeScopeObservation) -> Option<&str> {
    match observation {
        OperativeScopeObservation::Current { binding } => Some(binding.subject_ref.as_str()),
        OperativeScopeObservation::Stale { observed, .. } => Some(observed.subject_ref.as_str()),
        OperativeScopeObservation::Missing { .. } => None,
    }
}

fn validate_owner_response(response: &OperativeScopeCurrentnessResponse) -> Result<(), String> {
    if response.contract != OPERATIVE_CURRENTNESS_CONTRACT {
        return Err("unsupported AW2 operative-currentness contract".into());
    }
    if response.provider_ref != OPERATIVE_PROVIDER_REF
        || response.owner_ref != OPERATIVE_OWNER_REF
        || response.interpretation_ref != C_PRIME_INTERPRETATION_REF
    {
        return Err("operative currentness is not an observation from the QL AW2 owner".into());
    }
    if response.requested_binding_ref.trim().is_empty()
        || response.requested_binding_revision.trim().is_empty()
        || response.current_whole_ref.trim().is_empty()
    {
        return Err("operative currentness response lacks stable owner identity".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::{Value, json};

    use crate::focused_instrument::{
        ClockDisclosure, ClockPresentation, FieldCursor, FocusDisclosure, FocusedInstrumentSnapshot,
        InstrumentFocus, InstrumentOperationObservation, OperationStanding, SelectionTracking,
        TemporalPresentation, FOCUSED_INSTRUMENT_CONTRACT, OPERATION_OBSERVATION_CONTRACT,
    };
    use crate::nara::EventBasisRefs;
    use crate::vak_profile::{
        CPrimeProfile, ContentPosition, ContentType, ContextSequence, InquiryDirection,
        Participation, ThreadForm, PROFILE_CONTRACT,
    };
    use crate::vak_scope::{
        CPrimeOperativeBinding, OperativeFrameReading, OperativeScopeObservation,
        OPERATIVE_BINDING_CONTRACT,
    };

    use super::*;

    fn profile() -> CPrimeProfile {
        CPrimeProfile {
            participation: Participation::AuthorisedUndertaking,
            content: ContentType::Operations,
            position: ContentPosition::Operation,
            thread: ThreadForm::Chain,
            sequence: ContextSequence::ThroughOperation,
            direction: InquiryDirection::Forward,
        }
    }

    fn binding() -> CPrimeOperativeBinding {
        CPrimeOperativeBinding {
            contract: OPERATIVE_BINDING_CONTRACT.into(),
            provider_ref: OPERATIVE_PROVIDER_REF.into(),
            binding_ref: "binding:one".into(),
            binding_revision: "binding-r1".into(),
            owner_ref: OPERATIVE_OWNER_REF.into(),
            interpretation_ref: C_PRIME_INTERPRETATION_REF.into(),
            interpretation_revision: "profile-r1".into(),
            profile_contract: PROFILE_CONTRACT.into(),
            profile_source: "source:profile".into(),
            profile_source_blob: "blob:profile".into(),
            world_ref: "world:one".into(),
            world_generation: "generation-1".into(),
            whole_ref: "whole:one".into(),
            subject_ref: "subject:nara".into(),
            frame: OperativeFrameReading {
                context_frame: "CF5".into(),
                constitutional_voice: "Anima".into(),
                lens: "L0".into(),
                musical_basis: "chromatic".into(),
                face: "direct".into(),
                position_basis: "local".into(),
                frame_pitch: 0,
            },
            profile: profile(),
            sources: Vec::new(),
            method_skill_ref: None,
            evidence_refs: BTreeSet::from(["evidence:one".into()]),
            standing: "derived".into(),
        }
    }

    fn response(observation: OperativeScopeObservation) -> OperativeScopeCurrentnessResponse {
        OperativeScopeCurrentnessResponse {
            contract: OPERATIVE_CURRENTNESS_CONTRACT.into(),
            provider_ref: OPERATIVE_PROVIDER_REF.into(),
            owner_ref: OPERATIVE_OWNER_REF.into(),
            interpretation_ref: C_PRIME_INTERPRETATION_REF.into(),
            requested_binding_ref: "binding:one".into(),
            requested_binding_revision: "binding-r1".into(),
            current_whole_ref: "whole:one".into(),
            observation,
        }
    }

    fn snapshot(subject: &str) -> FocusedInstrumentSnapshot {
        let cursor = FieldCursor {
            event_ref: "event:one".into(),
            subject_ref: subject.into(),
            profile_generation: 1,
            field_generation: "1".into(),
            samples_elapsed: "0".into(),
        };
        FocusedInstrumentSnapshot {
            schema: FOCUSED_INSTRUMENT_CONTRACT.into(),
            available: true,
            event: EventBasisRefs {
                event_ref: "event:one".into(),
                subject_ref: subject.into(),
                profile_generation: 1,
                registry_revision: "registry:r1".into(),
                m1_revision: "m1:r1".into(),
                m2_source_ref: "m2:source".into(),
                m2_contract_ref: "m2:contract".into(),
                m3_source_ref: "m3:source".into(),
                m3_contract_ref: "m3:contract".into(),
            },
            live_cursor: cursor.clone(),
            presented_cursor: cursor,
            temporal: TemporalPresentation::Live,
            tracking: SelectionTracking::Follow,
            selection: None,
            selection_standing: None,
            selected_target: None,
            focus: FocusDisclosure {
                focus: InstrumentFocus::M5,
                available: true,
                current: true,
                source_refs: vec!["source:one".into()],
                payload: Value::Null,
                standing: "controlled".into(),
            },
            clock: ClockDisclosure {
                presentation: ClockPresentation::Assembled,
                owner_clock: json!({"field_ref":"#3-0","centre_ref":"#3-5-5/0"}),
                field_ref: "#3-0".into(),
                centre_ref: "#3-5-5/0".into(),
                standing: "controlled".into(),
            },
            vak_expression: None,
            vak_performance: None,
            personal_current: false,
            standing: "controlled".into(),
        }
    }

    fn operation() -> InstrumentOperationObservation {
        InstrumentOperationObservation {
            schema: OPERATION_OBSERVATION_CONTRACT.into(),
            operation: "advance".into(),
            standing: OperationStanding::Applied,
            before: None,
            after: None,
            owner_receipt: Some(json!({"result":"applied"})),
            error: None,
        }
    }

    #[test]
    fn current_is_only_accepted_when_aw2_identity_agrees() {
        let current = FocusedOperativeCurrentness::from_aw2(response(
            OperativeScopeObservation::Current { binding: binding() },
        ))
        .unwrap();
        assert_eq!(current.standing, FocusedOperativeStanding::Current);
        assert!(current.is_current());

        let mut other = binding();
        other.binding_revision = "binding-r2".into();
        assert!(
            FocusedOperativeCurrentness::from_aw2(response(
                OperativeScopeObservation::Current { binding: other },
            ))
            .is_err()
        );
    }

    #[test]
    fn stale_and_missing_remain_explicit_owner_standings() {
        let stale = FocusedOperativeCurrentness::from_aw2(response(
            OperativeScopeObservation::Stale {
                observed: binding(),
                differences: vec!["source-basis".into()],
            },
        ))
        .unwrap();
        assert_eq!(stale.standing, FocusedOperativeStanding::Stale);
        assert_eq!(stale.differences, vec!["source-basis"]);

        let missing = FocusedOperativeCurrentness::from_aw2(response(
            OperativeScopeObservation::Missing {
                binding_ref: "binding:one".into(),
                current_whole_ref: "whole:one".into(),
                reason: "not present".into(),
            },
        ))
        .unwrap();
        assert_eq!(missing.standing, FocusedOperativeStanding::Missing);
    }

    #[test]
    fn foreign_or_boolean_like_currentness_is_not_admitted() {
        let mut foreign = response(OperativeScopeObservation::Current { binding: binding() });
        foreign.provider_ref = "provider/client-echo".into();
        assert!(FocusedOperativeCurrentness::from_aw2(foreign).is_err());

        let stale = response(OperativeScopeObservation::Stale {
            observed: binding(),
            differences: Vec::new(),
        });
        assert!(FocusedOperativeCurrentness::from_aw2(stale).is_err());
    }

    #[test]
    fn owner_currentness_is_retained_through_snapshot_and_operation_return() {
        let currentness = FocusedOperativeCurrentness::from_aw2(response(
            OperativeScopeObservation::Current { binding: binding() },
        ))
        .unwrap();
        let focused = currentness
            .retain_in_snapshot(snapshot("subject:nara"))
            .unwrap();
        assert_eq!(
            focused.operative_currentness.standing,
            FocusedOperativeStanding::Current
        );
        assert_eq!(focused.contract, SOURCE_QUALIFIED_SNAPSHOT_CONTRACT);

        let returned = currentness.retain_in_return(operation());
        assert_eq!(returned.contract, SOURCE_QUALIFIED_RETURN_CONTRACT);
        assert_eq!(returned.operation.standing, OperationStanding::Applied);
        assert_eq!(
            returned.currentness_aperture,
            "supplied-owner-observation-not-post-effect-reobservation"
        );
    }

    #[test]
    fn focused_snapshot_refuses_another_aw2_subject() {
        let currentness = FocusedOperativeCurrentness::from_aw2(response(
            OperativeScopeObservation::Current { binding: binding() },
        ))
        .unwrap();
        assert!(
            currentness
                .retain_in_snapshot(snapshot("subject:other"))
                .is_err()
        );
    }
}
