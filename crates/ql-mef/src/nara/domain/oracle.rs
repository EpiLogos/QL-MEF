use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::nara::{ConsentState, SourceRevision};

use super::{EvidenceStanding, ProtectedRef, check_refs, check_source, check_text};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OracleSystem {
    TarotRws,
    TarotThoth,
    TarotMarseille,
    TarotQl,
    IChingCoins,
    IChingYarrow,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OracleHygiene {
    Clear,
    Warning,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleEntropyReceipt {
    pub entropy_ref: ProtectedRef,
    pub method_ref: String,
    pub provider_ref: String,
    pub observed_at_unix_ms: u64,
    pub evidence_refs: Vec<String>,
}

impl OracleEntropyReceipt {
    pub fn validate(&self) -> Result<(), String> {
        self.entropy_ref.validate()?;
        check_text(&self.method_ref, "oracle entropy method")?;
        check_text(&self.provider_ref, "oracle entropy provider")?;
        check_refs(
            &self.evidence_refs,
            "oracle entropy evidence reference",
            128,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleToken {
    pub token_ref: String,
    pub position: u16,
    pub reversed: bool,
    pub changing: bool,
    pub source_refs: Vec<String>,
}

impl OracleToken {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.token_ref, "oracle token")?;
        check_refs(&self.source_refs, "oracle token source reference", 64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleOriginalPacket {
    pub packet_ref: String,
    pub subject_id: String,
    pub event_ref: String,
    pub profile_generation: u64,
    pub system: OracleSystem,
    pub tradition_ref: String,
    pub deck_or_method_ref: String,
    pub spread_ref: Option<String>,
    pub query_ref: ProtectedRef,
    pub entropy: OracleEntropyReceipt,
    pub tokens: Vec<OracleToken>,
    pub cast_degree: Option<u16>,
    pub cast_at_unix_ms: u64,
    pub vak_ref: Option<String>,
    pub original_payload_ref: ProtectedRef,
    pub source_revisions: Vec<SourceRevision>,
    pub consent: ConsentState,
    pub hygiene: OracleHygiene,
}

impl OracleOriginalPacket {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.packet_ref, "oracle packet")?;
        check_text(&self.subject_id, "oracle subject")?;
        check_text(&self.event_ref, "oracle event")?;
        check_text(&self.tradition_ref, "oracle tradition")?;
        check_text(&self.deck_or_method_ref, "oracle deck/method")?;
        if let Some(reference) = &self.spread_ref {
            check_text(reference, "oracle spread")?;
        }
        self.query_ref.validate()?;
        self.entropy.validate()?;
        if self.tokens.is_empty() || self.tokens.len() > 78 {
            return Err("oracle packet requires 1..78 retained cast tokens".into());
        }
        let mut positions = BTreeSet::new();
        for token in &self.tokens {
            token.validate()?;
            if !positions.insert(token.position) {
                return Err("oracle packet has duplicate token positions".into());
            }
        }
        if self.cast_degree.is_some_and(|degree| degree >= 720) {
            return Err("oracle cast degree must be on the 0..719 double-cover ring".into());
        }
        if let Some(reference) = &self.vak_ref {
            check_text(reference, "oracle Vāk reference")?;
        }
        self.original_payload_ref.validate()?;
        if self.source_revisions.is_empty() || self.source_revisions.len() > 256 {
            return Err("oracle packet requires 1..256 source revisions".into());
        }
        for source in &self.source_revisions {
            check_source(source)?;
        }
        if self.consent != ConsentState::Granted {
            return Err("oracle packet requires explicit granted consent".into());
        }
        if self.hygiene == OracleHygiene::Blocked {
            return Err("blocked oracle packet cannot be admitted as a cast".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleInterpretation {
    pub interpretation_ref: String,
    pub packet_ref: String,
    pub interpretation_revision: String,
    pub model_ref: Option<String>,
    pub output_ref: ProtectedRef,
    pub source_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub standing: EvidenceStanding,
    pub interpreted_at_unix_ms: u64,
}

impl OracleInterpretation {
    pub fn validate(&self, packet_ref: &str) -> Result<(), String> {
        check_text(&self.interpretation_ref, "oracle interpretation")?;
        if self.packet_ref != packet_ref {
            return Err("oracle interpretation belongs to a different original packet".into());
        }
        check_text(
            &self.interpretation_revision,
            "oracle interpretation revision",
        )?;
        if let Some(reference) = &self.model_ref {
            check_text(reference, "oracle model")?;
        }
        self.output_ref.validate()?;
        check_refs(
            &self.source_refs,
            "oracle interpretation source reference",
            256,
        )?;
        check_refs(
            &self.evidence_refs,
            "oracle interpretation evidence reference",
            256,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleRecord {
    pub original: OracleOriginalPacket,
    pub interpretations: Vec<OracleInterpretation>,
}

impl OracleRecord {
    pub fn validate(&self) -> Result<(), String> {
        self.original.validate()?;
        if self.interpretations.len() > 1024 {
            return Err("too many oracle reinterpretations".into());
        }
        let mut ids = BTreeSet::new();
        for interpretation in &self.interpretations {
            interpretation.validate(&self.original.packet_ref)?;
            if !ids.insert(interpretation.interpretation_ref.as_str()) {
                return Err("duplicate oracle interpretation reference".into());
            }
        }
        Ok(())
    }

    pub fn add_reinterpretation(
        &mut self,
        interpretation: OracleInterpretation,
    ) -> Result<(), String> {
        interpretation.validate(&self.original.packet_ref)?;
        if self
            .interpretations
            .iter()
            .any(|existing| existing.interpretation_ref == interpretation.interpretation_ref)
        {
            return Err("oracle interpretation reference already exists".into());
        }
        self.interpretations.push(interpretation);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn protected(name: &str) -> ProtectedRef {
        ProtectedRef {
            ref_id: format!("protected:{name}"),
            revision: "r1".into(),
            owner_ref: "central".into(),
        }
    }

    fn source() -> SourceRevision {
        SourceRevision {
            source_ref: "source:iching".into(),
            revision: "r1".into(),
            standing_ref: "source".into(),
        }
    }

    fn original() -> OracleOriginalPacket {
        OracleOriginalPacket {
            packet_ref: "oracle:1".into(),
            subject_id: "nara-a".into(),
            event_ref: "event:1".into(),
            profile_generation: 1,
            system: OracleSystem::IChingCoins,
            tradition_ref: "tradition:iching".into(),
            deck_or_method_ref: "coins".into(),
            spread_ref: None,
            query_ref: protected("query"),
            entropy: OracleEntropyReceipt {
                entropy_ref: protected("entropy"),
                method_ref: "explicit".into(),
                provider_ref: "provider:entropy".into(),
                observed_at_unix_ms: 1,
                evidence_refs: vec!["evidence:entropy".into()],
            },
            tokens: (0..6)
                .map(|position| OracleToken {
                    token_ref: format!("line:{position}"),
                    position,
                    reversed: false,
                    changing: position == 2,
                    source_refs: vec!["source:iching".into()],
                })
                .collect(),
            cast_degree: Some(123),
            cast_at_unix_ms: 2,
            vak_ref: Some("vak:M0-5".into()),
            original_payload_ref: protected("packet"),
            source_revisions: vec![source()],
            consent: ConsentState::Granted,
            hygiene: OracleHygiene::Clear,
        }
    }

    #[test]
    fn reinterpretation_does_not_mutate_original_cast() {
        let original = original();
        let mut record = OracleRecord {
            original: original.clone(),
            interpretations: Vec::new(),
        };
        record
            .add_reinterpretation(OracleInterpretation {
                interpretation_ref: "interpretation:later".into(),
                packet_ref: original.packet_ref.clone(),
                interpretation_revision: "r2".into(),
                model_ref: Some("model:2".into()),
                output_ref: protected("interpretation"),
                source_refs: vec!["source:commentary".into()],
                evidence_refs: Vec::new(),
                standing: EvidenceStanding::Derived,
                interpreted_at_unix_ms: 3,
            })
            .unwrap();
        assert_eq!(record.original, original);
    }
}
