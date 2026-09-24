//! Nara/Anima ExpressionProfile — presentation mapping over a protected M4 reading.
//!
//! This declares how a current NaraExpressionSession may drive the existing O:I
//! Expression body. It is presentation grammar, not a claim that visual parameters
//! are the ontology of the centres. Centre identity is never a cymatic station.
//! EarthBody remains the grounding frame and must not become an eighth centre peer.

use serde::{Deserialize, Serialize};

use crate::focused_instrument::{
    NaraCentreExpressionReading, NaraEarthBodyExpressionReading, NaraExpressionSession,
};

pub const ANIMA_EXPRESSION_PROFILE_CONTRACT: &str = "ql.nara-anima-expression-profile/v1";
pub const ANIMA_PROFILE_LINEAGE: &str = "nara-anima-expression-profile/v1";

/// Presentation-only mapping family. Replaceable through profile lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimaPresentationMapping {
    pub m1: &'static str,
    pub m2: &'static str,
    pub m3: &'static str,
    pub centre_amplitude: &'static str,
    pub centre_phase: &'static str,
    pub centre_coupling: &'static str,
    pub earth_body: &'static str,
    pub standing: &'static str,
}

pub const DEFAULT_PRESENTATION_MAPPING: AnimaPresentationMapping = AnimaPresentationMapping {
    m1: "topology / carrier orientation / phase / spatial relation",
    m2: "resonant mode / material excitation / colour / audio",
    m3: "glyph / form / aperture / pose / temporal-form movement (only when ql.m3-physical-form-target is admitted)",
    centre_amplitude: "bounded local expressive intensity",
    centre_phase: "bounded local phase/orientation relation",
    centre_coupling: "explicit relational carrier/modulation between named centres",
    earth_body: "whole-field grounding/frame relation",
    standing: "presentation mappings only; not the ontology of the chakra",
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimaCentreBinding {
    pub ordinal: u8,
    pub locus_ref: String,
    pub label: String,
    pub source_ref: String,
    pub source_revision: String,
    pub m1_basis_ref: String,
    pub m1_source_ref: String,
    pub m1_value: f64,
    pub m2_basis_ref: String,
    pub m2_source_ref: String,
    pub m2_value: f64,
    pub m3_basis_ref: String,
    pub m3_source_ref: String,
    pub m3_value: f64,
    pub amplitude: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimaEarthBodyBinding {
    pub locus_ref: String,
    pub source_ref: String,
    pub source_revision: String,
    pub frame_ref: String,
    pub standing: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimaExpressionProfile {
    pub schema: &'static str,
    pub lineage: &'static str,
    pub subject_ref: String,
    pub event_ref: String,
    pub profile_generation: u64,
    pub personal_reception_generation: u64,
    pub current: bool,
    pub centres: Vec<AnimaCentreBinding>,
    pub earth_body: AnimaEarthBodyBinding,
    pub presentation_mapping: AnimaPresentationMapping,
    /// Hard law: centre identity is never equated with a cymatic frequency/station.
    pub centre_identity_neq_cymatic_station: bool,
    pub standing: &'static str,
}

fn centre_binding(centre: &NaraCentreExpressionReading) -> AnimaCentreBinding {
    AnimaCentreBinding {
        ordinal: centre.ordinal,
        locus_ref: centre.locus_ref.clone(),
        label: centre.label.clone(),
        source_ref: centre.source.source_ref.clone(),
        source_revision: centre.source.revision.clone(),
        m1_basis_ref: centre.m1.basis_ref.clone(),
        m1_source_ref: centre.m1.source_ref.clone(),
        m1_value: centre.m1.value,
        m2_basis_ref: centre.m2.basis_ref.clone(),
        m2_source_ref: centre.m2.source_ref.clone(),
        m2_value: centre.m2.value,
        m3_basis_ref: centre.m3.basis_ref.clone(),
        m3_source_ref: centre.m3.source_ref.clone(),
        m3_value: centre.m3.value,
        amplitude: centre.resonance,
    }
}

fn earth_binding(earth: &NaraEarthBodyExpressionReading) -> AnimaEarthBodyBinding {
    AnimaEarthBodyBinding {
        locus_ref: earth.locus_ref.clone(),
        source_ref: earth.source.source_ref.clone(),
        source_revision: earth.source.revision.clone(),
        frame_ref: earth.frame_ref.clone(),
        standing: "EarthBody is the grounding relation; not an eighth centre peer",
    }
}

/// Project a presentation-safe Anima ExpressionProfile from a current Nara session.
pub fn project_anima_profile(
    session: &NaraExpressionSession,
) -> Result<AnimaExpressionProfile, String> {
    if session.centres.len() != 7 {
        return Err("Anima ExpressionProfile requires exactly seven centres".into());
    }
    let mut centres = session
        .centres
        .iter()
        .map(centre_binding)
        .collect::<Vec<_>>();
    centres.sort_by_key(|centre| centre.ordinal);
    if centres
        .iter()
        .enumerate()
        .any(|(index, centre)| usize::from(centre.ordinal) != index)
    {
        return Err("Anima centre ordinals must be 0..6 independently".into());
    }
    if centres.iter().any(|centre| !centre.amplitude.is_finite()) {
        return Err("Anima centre amplitude must be finite".into());
    }
    if centres
        .iter()
        .any(|centre| centre.locus_ref == session.earth_body.locus_ref)
    {
        return Err("EarthBody must not appear as a centre peer".into());
    }
    Ok(AnimaExpressionProfile {
        schema: ANIMA_EXPRESSION_PROFILE_CONTRACT,
        lineage: ANIMA_PROFILE_LINEAGE,
        subject_ref: session.subject_ref.clone(),
        event_ref: session.event_ref.clone(),
        profile_generation: session.profile_generation,
        personal_reception_generation: session.personal_reception_generation,
        current: session.current,
        centres,
        earth_body: earth_binding(&session.earth_body),
        presentation_mapping: DEFAULT_PRESENTATION_MAPPING,
        centre_identity_neq_cymatic_station: true,
        standing: "live projection of this Nara subject's current M4 embodied field through the Nara/Anima ExpressionProfile; private PersonalFieldState is not copied into a generic Expression document",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::focused_instrument::{
        NaraCentreExpressionReading, NaraEarthBodyExpressionReading, NaraExpressionAvailability,
        NaraExpressionPortableCues, NaraExpressionSession, NaraResonanceStationDisclosure,
    };
    use crate::nara::{SourceRevision, WorldContribution};
    use serde_json::json;

    fn contribution(basis: &str, value: f64) -> WorldContribution {
        WorldContribution {
            basis_ref: basis.into(),
            source_ref: format!("source:{basis}"),
            value,
        }
    }

    fn centre(ordinal: u8) -> NaraCentreExpressionReading {
        NaraCentreExpressionReading {
            locus_ref: format!("ql:nara:subject:1:centre:{ordinal}"),
            ordinal,
            label: format!("centre-{ordinal}"),
            source: SourceRevision {
                source_ref: format!("source:centre:{ordinal}"),
                revision: "r1".into(),
                standing_ref: "source".into(),
            },
            m1: contribution("m1", 0.1 * f64::from(ordinal)),
            m2: contribution("m2", 0.2 * f64::from(ordinal)),
            m3: contribution("m3", 0.3 * f64::from(ordinal)),
            resonance: 0.5 + 0.05 * f64::from(ordinal),
        }
    }

    fn session() -> NaraExpressionSession {
        let centres = (0..7).map(centre).collect::<Vec<_>>();
        let earth = NaraEarthBodyExpressionReading {
            locus_ref: "ql:nara:subject:1:earth-body".into(),
            source: SourceRevision {
                source_ref: "source:earth".into(),
                revision: "r1".into(),
                standing_ref: "source".into(),
            },
            frame_ref: "frame:earth".into(),
        };
        NaraExpressionSession {
            schema: "ql.nara-expression-session/v1".into(),
            subject_ref: "subject:1".into(),
            event_ref: "event:1".into(),
            profile_generation: 3,
            personal_reception_generation: 7,
            current: true,
            centres: centres.clone(),
            earth_body: earth.clone(),
            resonance_stations: NaraResonanceStationDisclosure {
                availability: NaraExpressionAvailability::Unavailable,
                station_refs: Vec::new(),
                standing: "cymatic stations are not centre identity".into(),
            },
            m1_reading_refs: vec!["m1:1".into()],
            m2_reading_refs: vec!["m2:1".into()],
            m3_reading_refs: vec!["m3:1".into()],
            action_refs: Vec::new(),
            m1_presentation: json!({}),
            m2_presentation: json!({}),
            m3_presentation: json!({}),
            portable: NaraExpressionPortableCues {
                schema: "ql.nara-expression-portable-cues/v1".into(),
                subject_ref: "subject:1".into(),
                event_ref: "event:1".into(),
                profile_generation: 3,
                personal_reception_generation: 7,
                centre_locus_refs: centres.iter().map(|c| c.locus_ref.clone()).collect(),
                earth_body_locus_ref: earth.locus_ref.clone(),
                source_refs: Vec::new(),
                cue_refs: Vec::new(),
            },
            standing: "test".into(),
        }
    }

    #[test]
    fn projects_seven_independent_centres_with_earth_as_ground() {
        let profile = project_anima_profile(&session()).unwrap();
        assert_eq!(profile.schema, ANIMA_EXPRESSION_PROFILE_CONTRACT);
        assert_eq!(profile.centres.len(), 7);
        assert!(profile.centre_identity_neq_cymatic_station);
        assert!(!profile
            .centres
            .iter()
            .any(|c| c.locus_ref == profile.earth_body.locus_ref));
        assert!(profile.earth_body.standing.contains("not an eighth"));
        assert!(profile.presentation_mapping.standing.contains("presentation"));
    }

    #[test]
    fn refuses_wrong_centre_count() {
        let mut bad = session();
        bad.centres.pop();
        assert!(project_anima_profile(&bad).unwrap_err().contains("exactly seven"));
    }
}
