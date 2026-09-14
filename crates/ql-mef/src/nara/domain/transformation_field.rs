use serde::{Deserialize, Serialize};

use super::{CapabilityRefs, EvidenceStanding, TransformationHistory, TransformationPhase};

/// Complete M4.3 disposition. The actual source-defined 12×3/24-stroke history
/// is retained alongside the other five transformation offices instead of being
/// mistaken for the whole transformation domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransformationField {
    /// M4.3.0 — transformation cycle engine.
    pub cycle_engine: CapabilityRefs,
    /// M4.3.1 — operational grammar.
    pub operational_grammar: CapabilityRefs,
    /// M4.3.2 — dialogical containers.
    pub dialogical_containers: CapabilityRefs,
    /// M4.3.3 — bounded perturbation / control and safety.
    pub control_safety: CapabilityRefs,
    /// M4.3.4 — protocol library.
    pub protocol_library: CapabilityRefs,
    /// M4.3.5 — phase/telemetry history.
    pub phase_history: TransformationHistory,
    pub standing: EvidenceStanding,
}

impl TransformationField {
    pub fn validate(&self) -> Result<(), String> {
        self.cycle_engine.validate()?;
        self.operational_grammar.validate()?;
        self.dialogical_containers.validate()?;
        self.control_safety.validate()?;
        self.protocol_library.validate()?;
        self.phase_history.validate()
    }

    pub fn append_phase(&mut self, phase: TransformationPhase) -> Result<(), String> {
        phase.validate()?;
        if self
            .phase_history
            .phases
            .iter()
            .any(|existing| existing.phase_ref == phase.phase_ref)
        {
            return Err("transformation phase reference already exists".into());
        }
        if self
            .phase_history
            .phases
            .last()
            .is_some_and(|previous| phase.opened_at_unix_ms < previous.opened_at_unix_ms)
        {
            return Err("transformation phase occurs before retained history".into());
        }
        self.phase_history.phases.push(phase);
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformation_offices_do_not_disappear_when_source_is_unavailable() {
        let field = TransformationField {
            cycle_engine: CapabilityRefs::present(vec!["cycle:engine".into()]).unwrap(),
            operational_grammar: CapabilityRefs::present(vec!["grammar:seven-ops".into()])
                .unwrap(),
            dialogical_containers: CapabilityRefs::absent("container source not loaded").unwrap(),
            control_safety: CapabilityRefs::present(vec!["control:bounded".into()]).unwrap(),
            protocol_library: CapabilityRefs::present(vec!["protocol:library".into()]).unwrap(),
            phase_history: TransformationHistory {
                history_ref: "history:1".into(),
                subject_id: "nara-a".into(),
                phases: Vec::new(),
                standing: EvidenceStanding::Source,
            },
            standing: EvidenceStanding::Implementation,
        };
        field.validate().unwrap();
        assert!(!field.dialogical_containers.is_present());
    }
}
