use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::nara::{ConsentState, SourceRevision, WorldContribution};

use super::{
    CENTRE_COUNT, ELEMENT_COUNT, EvidenceStanding, check_optional_finite, check_refs, check_source,
    check_text,
};

/// Canonical quaternion/element order is fixed as Earth, Fire, Water, Air.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElementalEfwa {
    pub earth: f64,
    pub fire: f64,
    pub water: f64,
    pub air: f64,
}

impl ElementalEfwa {
    pub fn validate(self) -> Result<(), String> {
        if [self.earth, self.fire, self.water, self.air]
            .into_iter()
            .all(f64::is_finite)
        {
            Ok(())
        } else {
            Err("non-finite EFWA elemental state".into())
        }
    }

    pub const fn as_array(self) -> [f64; ELEMENT_COUNT] {
        [self.earth, self.fire, self.water, self.air]
    }
}

/// Exact M1/M2/M3 contribution triple acknowledged by one centre. The order is
/// explicit in the field names rather than inferred from a three-element array.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreWorldInputs {
    pub m1: WorldContribution,
    pub m2: WorldContribution,
    pub m3: WorldContribution,
}

impl CentreWorldInputs {
    pub fn validate(&self) -> Result<(), String> {
        for (contribution, label) in [
            (&self.m1, "M1 centre contribution"),
            (&self.m2, "M2 centre contribution"),
            (&self.m3, "M3 centre contribution"),
        ] {
            check_text(&contribution.basis_ref, label)?;
            check_text(&contribution.source_ref, label)?;
            if !contribution.value.is_finite() {
                return Err(format!("non-finite {label}"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreMode {
    pub mode_ref: String,
    pub amplitude: f64,
    pub phase_radians: f64,
    pub source: SourceRevision,
}

impl CentreMode {
    fn validate(&self) -> Result<(), String> {
        check_text(&self.mode_ref, "centre mode")?;
        check_source(&self.source)?;
        if self.amplitude.is_finite() && self.phase_radians.is_finite() {
            Ok(())
        } else {
            Err("non-finite centre mode".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreCoupling {
    pub target_ordinal: u8,
    pub coefficient: f64,
    pub phase_offset_radians: f64,
    pub source: SourceRevision,
}

impl CentreCoupling {
    fn validate(&self, own_ordinal: u8) -> Result<(), String> {
        if usize::from(self.target_ordinal) >= CENTRE_COUNT || self.target_ordinal == own_ordinal {
            return Err("invalid centre coupling target".into());
        }
        if !self.coefficient.is_finite() || !self.phase_offset_radians.is_finite() {
            return Err("non-finite centre coupling".into());
        }
        check_source(&self.source)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreEmbodiment {
    pub ordinal: u8,
    pub label: String,
    pub source: SourceRevision,
    pub world_inputs: CentreWorldInputs,
    pub amplitude: Option<f64>,
    pub phase_radians: Option<f64>,
    pub modes: Vec<CentreMode>,
    pub couplings: Vec<CentreCoupling>,
    pub body_zone_refs: Vec<String>,
    pub sense_refs: Vec<String>,
    pub action_refs: Vec<String>,
    pub feedback_refs: Vec<String>,
    pub standing: EvidenceStanding,
}

impl CentreEmbodiment {
    pub fn validate(&self) -> Result<(), String> {
        if usize::from(self.ordinal) >= CENTRE_COUNT {
            return Err("centre ordinal outside the seven-centre field".into());
        }
        check_text(&self.label, "centre label")?;
        check_source(&self.source)?;
        self.world_inputs.validate()?;
        check_optional_finite(self.amplitude, "centre amplitude")?;
        check_optional_finite(self.phase_radians, "centre phase")?;
        if self.modes.len() > 128 || self.couplings.len() > CENTRE_COUNT - 1 {
            return Err("centre mode/coupling field exceeds contract bounds".into());
        }
        for mode in &self.modes {
            mode.validate()?;
        }
        let mut targets = BTreeSet::new();
        for coupling in &self.couplings {
            coupling.validate(self.ordinal)?;
            if !targets.insert(coupling.target_ordinal) {
                return Err("duplicate centre coupling target".into());
            }
        }
        check_refs(&self.body_zone_refs, "body-zone reference", 128)?;
        check_refs(&self.sense_refs, "sense reference", 128)?;
        check_refs(&self.action_refs, "embodied action reference", 128)?;
        check_refs(&self.feedback_refs, "embodied feedback reference", 128)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EarthBodyEmbodiment {
    pub source: SourceRevision,
    pub frame_ref: String,
    pub amplitude: Option<f64>,
    pub phase_radians: Option<f64>,
    pub relation_refs: Vec<String>,
    pub standing: EvidenceStanding,
}

impl EarthBodyEmbodiment {
    fn validate(&self) -> Result<(), String> {
        check_source(&self.source)?;
        check_text(&self.frame_ref, "EarthBody frame")?;
        check_optional_finite(self.amplitude, "EarthBody amplitude")?;
        check_optional_finite(self.phase_radians, "EarthBody phase")?;
        check_refs(&self.relation_refs, "EarthBody relation reference", 128)
    }
}

/// M4.1.5 keeps embodied intensity/consent/response distinct from the
/// transformation branch's own perturbation/safety state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbodiedSafetyState {
    pub consent: ConsentState,
    pub intensity: Option<f64>,
    pub contraindication_refs: Vec<String>,
    pub response_refs: Vec<String>,
    pub adjustment_refs: Vec<String>,
    pub standing: EvidenceStanding,
}

impl EmbodiedSafetyState {
    pub fn validate(&self) -> Result<(), String> {
        if self
            .intensity
            .is_some_and(|value| !value.is_finite() || value < 0.0)
        {
            return Err("embodied intensity must be finite and non-negative".into());
        }
        check_refs(
            &self.contraindication_refs,
            "embodied contraindication reference",
            256,
        )?;
        check_refs(&self.response_refs, "embodied response reference", 256)?;
        check_refs(
            &self.adjustment_refs,
            "embodied adjustment reference",
            256,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbodiedField {
    pub reception_generation: u64,
    pub elemental_efwa: ElementalEfwa,
    pub elemental_source: SourceRevision,
    pub elemental_standing: EvidenceStanding,
    pub centres: Vec<CentreEmbodiment>,
    pub earth_body: EarthBodyEmbodiment,
    pub nadi_refs: Vec<String>,
    pub sushumna_ref: Option<String>,
    pub temporal_astrology_refs: Vec<String>,
    pub materia_refs: Vec<String>,
    pub operation_refs: Vec<String>,
    pub safety: EmbodiedSafetyState,
}

impl EmbodiedField {
    pub fn validate(&self) -> Result<(), String> {
        self.elemental_efwa.validate()?;
        check_source(&self.elemental_source)?;
        if self.centres.len() != CENTRE_COUNT {
            return Err("M4.1 requires exactly seven independently active centres".into());
        }
        let mut ordinals = BTreeSet::new();
        let mut labels = BTreeSet::new();
        for centre in &self.centres {
            centre.validate()?;
            if !ordinals.insert(centre.ordinal) || !labels.insert(centre.label.as_str()) {
                return Err("duplicate centre ordinal or label".into());
            }
        }
        if ordinals != BTreeSet::from([0, 1, 2, 3, 4, 5, 6]) {
            return Err("seven centre ordinals must be complete".into());
        }
        self.earth_body.validate()?;
        check_refs(&self.nadi_refs, "nadi reference", 256)?;
        if let Some(reference) = &self.sushumna_ref {
            check_text(reference, "sushumna reference")?;
        }
        check_refs(
            &self.temporal_astrology_refs,
            "temporal astrology reference",
            256,
        )?;
        check_refs(&self.materia_refs, "materia reference", 256)?;
        check_refs(&self.operation_refs, "embodied operation reference", 256)?;
        self.safety.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(name: &str) -> SourceRevision {
        SourceRevision {
            source_ref: format!("source:{name}"),
            revision: "r1".into(),
            standing_ref: "source".into(),
        }
    }

    fn contribution(layer: &str, ordinal: usize) -> WorldContribution {
        WorldContribution {
            basis_ref: format!("basis:{layer}"),
            source_ref: format!("source:{layer}:centre-{ordinal}"),
            value: ordinal as f64 + 0.5,
        }
    }

    #[test]
    fn seven_centres_remain_independent_and_earth_body_is_not_an_eighth_peer() {
        let centres = (0..CENTRE_COUNT)
            .map(|ordinal| CentreEmbodiment {
                ordinal: ordinal as u8,
                label: format!("centre-{ordinal}"),
                source: source(&format!("centre-{ordinal}")),
                world_inputs: CentreWorldInputs {
                    m1: contribution("m1", ordinal),
                    m2: contribution("m2", ordinal),
                    m3: contribution("m3", ordinal),
                },
                amplitude: Some(ordinal as f64 / 7.0),
                phase_radians: Some(ordinal as f64 / 10.0),
                modes: Vec::new(),
                couplings: Vec::new(),
                body_zone_refs: Vec::new(),
                sense_refs: Vec::new(),
                action_refs: Vec::new(),
                feedback_refs: Vec::new(),
                standing: EvidenceStanding::Observed,
            })
            .collect();
        let field = EmbodiedField {
            reception_generation: 4,
            elemental_efwa: ElementalEfwa {
                earth: 0.1,
                fire: 0.2,
                water: 0.3,
                air: 0.4,
            },
            elemental_source: source("elemental"),
            elemental_standing: EvidenceStanding::Observed,
            centres,
            earth_body: EarthBodyEmbodiment {
                source: source("earth-body"),
                frame_ref: "earth-fixed".into(),
                amplitude: Some(0.25),
                phase_radians: Some(0.5),
                relation_refs: vec!["relation:ground".into()],
                standing: EvidenceStanding::Derived,
            },
            nadi_refs: vec!["nadi:ida".into(), "nadi:pingala".into()],
            sushumna_ref: Some("nadi:sushumna".into()),
            temporal_astrology_refs: Vec::new(),
            materia_refs: Vec::new(),
            operation_refs: Vec::new(),
            safety: EmbodiedSafetyState {
                consent: ConsentState::Granted,
                intensity: Some(0.5),
                contraindication_refs: Vec::new(),
                response_refs: vec!["response:1".into()],
                adjustment_refs: Vec::new(),
                standing: EvidenceStanding::Reported,
            },
        };
        field.validate().unwrap();
        assert_eq!(field.centres.len(), CENTRE_COUNT);
        assert_eq!(field.earth_body.frame_ref, "earth-fixed");
        assert_eq!(field.centres[3].world_inputs.m2.value, 3.5);
    }

    #[test]
    fn canonical_element_array_is_earth_fire_water_air() {
        let value = ElementalEfwa {
            earth: 1.0,
            fire: 2.0,
            water: 3.0,
            air: 4.0,
        };
        assert_eq!(value.as_array(), [1.0, 2.0, 3.0, 4.0]);
    }
}
