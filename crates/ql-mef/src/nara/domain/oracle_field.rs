use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::{CapabilityRefs, EvidenceStanding, OracleRecord};

/// Complete M4.2 disposition. Individual cast records sit inside the six-office
/// source field rather than standing in for the whole oracle domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleField {
    /// M4.2.0 — common symbolic substrate / symbol ontology.
    pub common_symbolic_substrate: CapabilityRefs,
    /// M4.2.1 — full M1–M3 cosmic oracle invocation.
    pub cosmic_oracle_invocation: CapabilityRefs,
    /// M4.2.2 — Tarot systems/traditions.
    pub tarot_engines: CapabilityRefs,
    /// M4.2.3 — I-Ching change integration.
    pub iching_integration: CapabilityRefs,
    /// M4.2.4 — entropy, casting and interpretation machinery.
    pub casting_interpretation: CapabilityRefs,
    /// M4.2.5 — hygiene, practice and pedagogy.
    pub hygiene_pedagogy: CapabilityRefs,
    pub records: Vec<OracleRecord>,
    pub standing: EvidenceStanding,
}

impl OracleField {
    pub fn validate(&self) -> Result<(), String> {
        self.common_symbolic_substrate.validate()?;
        self.cosmic_oracle_invocation.validate()?;
        self.tarot_engines.validate()?;
        self.iching_integration.validate()?;
        self.casting_interpretation.validate()?;
        self.hygiene_pedagogy.validate()?;
        if self.records.len() > 4096 {
            return Err("M4.2 oracle history exceeds contract bounds".into());
        }
        let mut ids = BTreeSet::new();
        for record in &self.records {
            record.validate()?;
            if !ids.insert(record.original.packet_ref.as_str()) {
                return Err("duplicate oracle packet reference".into());
            }
        }
        Ok(())
    }

    pub fn record_cast(&mut self, record: OracleRecord) -> Result<(), String> {
        record.validate()?;
        if self
            .records
            .iter()
            .any(|existing| existing.original.packet_ref == record.original.packet_ref)
        {
            return Err("oracle packet reference already exists".into());
        }
        self.records.push(record);
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_six_oracle_offices_remain_visible_when_some_are_unavailable() {
        let field = OracleField {
            common_symbolic_substrate: CapabilityRefs::present(vec!["symbols:ontology".into()])
                .unwrap(),
            cosmic_oracle_invocation: CapabilityRefs::present(vec!["m1-m3:oracle".into()])
                .unwrap(),
            tarot_engines: CapabilityRefs::present(vec!["tarot:rws".into()]).unwrap(),
            iching_integration: CapabilityRefs::present(vec!["iching:change".into()]).unwrap(),
            casting_interpretation: CapabilityRefs::present(vec!["casting:entropy".into()])
                .unwrap(),
            hygiene_pedagogy: CapabilityRefs::absent("pedagogical source not loaded").unwrap(),
            records: Vec::new(),
            standing: EvidenceStanding::Implementation,
        };
        field.validate().unwrap();
        assert!(!field.hygiene_pedagogy.is_present());
    }
}
