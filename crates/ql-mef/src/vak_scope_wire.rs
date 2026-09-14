//! Transport-neutral currentness envelope for QL-owned operative C′ bindings.
//!
//! The wire carries a historical owner binding to the existing `VakComposition`
//! owner and returns its fresh Current/Stale/Missing observation. It is not an
//! authority token, not a scope registry and not a stateless echo endpoint.

use serde::{Deserialize, Serialize};

use crate::vak_composition::{CompositionError, Result, VakComposition};
use crate::vak_scope::{
    C_PRIME_INTERPRETATION_REF, CPrimeOperativeBinding, OPERATIVE_BINDING_CONTRACT,
    OPERATIVE_OWNER_REF, OPERATIVE_PROVIDER_REF, OperativeScopeCorrelation,
    OperativeScopeObservation,
};

pub const OPERATIVE_CURRENTNESS_CONTRACT: &str = "ql.operative-scope-currentness/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperativeScopeCurrentnessRequest {
    pub contract: String,
    pub expected: CPrimeOperativeBinding,
    pub current_whole_ref: String,
    pub correlation: OperativeScopeCorrelation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperativeScopeCurrentnessResponse {
    pub contract: String,
    pub provider_ref: String,
    pub owner_ref: String,
    pub interpretation_ref: String,
    pub requested_binding_ref: String,
    pub requested_binding_revision: String,
    pub current_whole_ref: String,
    pub observation: OperativeScopeObservation,
}

impl VakComposition {
    /// Reobserve one transported binding through the actual QL semantic owner.
    ///
    /// The request carries the earlier binding as comparison evidence only. QL
    /// recompiles `current_whole_ref` through `VakComposition`; callers cannot
    /// make a binding Current by serialising it back to the producer unchanged.
    pub fn observe_operative_scope_currentness(
        &self,
        request: OperativeScopeCurrentnessRequest,
    ) -> Result<OperativeScopeCurrentnessResponse> {
        validate_request(&request)?;
        let requested_binding_ref = request.expected.binding_ref.clone();
        let requested_binding_revision = request.expected.binding_revision.clone();
        let current_whole_ref = request.current_whole_ref.clone();
        let observation = self.reobserve_operative_scope(
            &request.expected,
            &request.current_whole_ref,
            request.correlation,
        )?;
        Ok(OperativeScopeCurrentnessResponse {
            contract: OPERATIVE_CURRENTNESS_CONTRACT.into(),
            provider_ref: OPERATIVE_PROVIDER_REF.into(),
            owner_ref: OPERATIVE_OWNER_REF.into(),
            interpretation_ref: C_PRIME_INTERPRETATION_REF.into(),
            requested_binding_ref,
            requested_binding_revision,
            current_whole_ref,
            observation,
        })
    }
}

fn validate_request(request: &OperativeScopeCurrentnessRequest) -> Result<()> {
    if request.contract != OPERATIVE_CURRENTNESS_CONTRACT {
        return Err(CompositionError(
            "unsupported operative currentness wire contract".into(),
        ));
    }
    if request.expected.contract != OPERATIVE_BINDING_CONTRACT {
        return Err(CompositionError(
            "currentness request does not carry a QL operative binding".into(),
        ));
    }
    if request.expected.provider_ref != OPERATIVE_PROVIDER_REF
        || request.expected.owner_ref != OPERATIVE_OWNER_REF
        || request.expected.interpretation_ref != C_PRIME_INTERPRETATION_REF
    {
        return Err(CompositionError(
            "currentness request carries a foreign operative owner".into(),
        ));
    }
    if request.current_whole_ref.trim().is_empty() || request.current_whole_ref.contains('\0') {
        return Err(CompositionError(
            "currentness request has no valid current whole".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::vak_profile::{
        CPrimeProfile, ContentPosition, ContentType, ContextSequence, InquiryDirection,
        Participation, ThreadForm, PROFILE_CONTRACT,
    };
    use crate::vak_scope::{CPrimeOperativeBinding, OperativeFrameReading};

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

    fn historical_binding() -> CPrimeOperativeBinding {
        CPrimeOperativeBinding {
            contract: OPERATIVE_BINDING_CONTRACT.into(),
            provider_ref: OPERATIVE_PROVIDER_REF.into(),
            binding_ref: "ql.cprime-operative-binding/v1|id-fnv1a64=historical".into(),
            binding_revision:
                "ql.cprime-operative-binding/v1|revision-fnv1a64=historical".into(),
            owner_ref: OPERATIVE_OWNER_REF.into(),
            interpretation_ref: C_PRIME_INTERPRETATION_REF.into(),
            interpretation_revision: "historical-profile-revision".into(),
            profile_contract: PROFILE_CONTRACT.into(),
            profile_source: "historical-profile-source".into(),
            profile_source_blob: "historical-profile-blob".into(),
            world_ref: "world/one".into(),
            world_generation: "generation-1".into(),
            whole_ref: "whole:missing".into(),
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
            method_skill_ref: Some("skill/recognised/revisit".into()),
            evidence_refs: BTreeSet::from(["evidence:historical".into()]),
            standing: "derived".into(),
        }
    }

    fn request() -> OperativeScopeCurrentnessRequest {
        OperativeScopeCurrentnessRequest {
            contract: OPERATIVE_CURRENTNESS_CONTRACT.into(),
            expected: historical_binding(),
            current_whole_ref: "whole:missing".into(),
            correlation: OperativeScopeCorrelation {
                world_ref: "world/one".into(),
                world_generation: "generation-1".into(),
                method_skill_ref: Some("skill/recognised/revisit".into()),
            },
        }
    }

    #[test]
    fn wire_round_trip_still_requires_owner_reobservation() {
        let encoded = serde_json::to_string(&request()).unwrap();
        let decoded: OperativeScopeCurrentnessRequest = serde_json::from_str(&encoded).unwrap();
        let response = VakComposition::default()
            .observe_operative_scope_currentness(decoded)
            .unwrap();
        assert_eq!(response.contract, OPERATIVE_CURRENTNESS_CONTRACT);
        assert_eq!(response.provider_ref, OPERATIVE_PROVIDER_REF);
        assert_eq!(response.owner_ref, OPERATIVE_OWNER_REF);
        assert!(matches!(
            response.observation,
            OperativeScopeObservation::Missing { .. }
        ));
    }

    #[test]
    fn foreign_owner_binding_is_refused_before_reobservation() {
        let mut request = request();
        request.expected.provider_ref = "provider/client-echo".into();
        assert!(
            VakComposition::default()
                .observe_operative_scope_currentness(request)
                .is_err()
        );
    }

    #[test]
    fn response_is_a_transportable_owner_observation_not_a_boolean_echo() {
        let response = VakComposition::default()
            .observe_operative_scope_currentness(request())
            .unwrap();
        let encoded = serde_json::to_string(&response).unwrap();
        let decoded: OperativeScopeCurrentnessResponse = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.requested_binding_ref, response.requested_binding_ref);
        assert_eq!(
            decoded.requested_binding_revision,
            response.requested_binding_revision
        );
        assert_eq!(decoded.observation, response.observation);
    }
}
