//! Provider-neutral voice-body semantics for dialogical Nara (TA3).
//!
//! Speech transport and provider choice stay entirely outside QL: no transport
//! types and no provider names appear here. What QL owns is the semantic
//! requirements a voice body must satisfy for foreground dialogue, the
//! explicit status of each capability (supported / unsupported / unknown),
//! and the speech-era identity law:
//!
//! ```text
//! NaraRef != AgentSessionRef != voice body != transport session
//! ```
//!
//! AIKit resolves the actual body; Actuation owns the canonical Agent and
//! authority relation; O:I owns the audio/UI adapter. A body that cannot
//! state a capability is `unknown`, and unknown never satisfies a
//! requirement — unavailable material remains a named condition.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub const NARA_VOICE_BODY_CONTRACT: &str = "ql.nara-voice-body/v1";

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        Err(format!("invalid {label}"))
    } else {
        Ok(())
    }
}

fn refs(values: &[String], label: &str, max: usize) -> Result<(), String> {
    if values.len() > max {
        return Err(format!("too many {label}"));
    }
    let mut unique = BTreeSet::new();
    for value in values {
        text(value, label)?;
        if !unique.insert(value.as_str()) {
            return Err(format!("duplicate {label}"));
        }
    }
    Ok(())
}

/// How the body carries a dialogue turn. A statement about interaction shape,
/// not a transport mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VoiceDuplexDisposition {
    /// Both sides may speak at once.
    FullDuplex,
    /// One side holds the floor per turn.
    StreamedTurnTaking,
    /// The body does not state this.
    Unknown,
}

/// Explicit capability status. Absence of evidence is `unknown`, never a
/// silent yes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VoiceCapabilityStatus {
    Supported,
    Unsupported,
    Unknown,
}

impl VoiceCapabilityStatus {
    pub const fn satisfies(self) -> bool {
        matches!(self, Self::Supported)
    }
}

/// How the bounded `NaraDialogueContext` reaches the body across a turn:
/// pushed on change, pulled by the body through a tool access, or neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextRefreshDisposition {
    PushOnChange,
    ToolAccess,
    None,
}

/// The voice body actually coupled to a dialogue turn. The ref is opaque; its
/// resolution, provider and transport are other owners' concerns. Provenance
/// is a ref: which body spoke stays attributable without naming any provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceBodyBinding {
    pub schema: String,
    pub body_ref: String,
    pub body_provenance_ref: String,
}

impl VoiceBodyBinding {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_VOICE_BODY_CONTRACT {
            return Err("unsupported Nara voice body contract".into());
        }
        text(&self.body_ref, "voice body reference")?;
        text(&self.body_provenance_ref, "voice body provenance reference")
    }

    /// Speech-era identity law: the body is neither the Nara nor the
    /// AgentSession carrying the turn.
    pub fn binds_distinctly(&self, nara_ref: &str, agent_session_ref: &str) -> Result<(), String> {
        self.validate()?;
        if self.body_ref == nara_ref {
            return Err("the voice body is not the Nara".into());
        }
        if self.body_ref == agent_session_ref {
            return Err("the voice body is not the AgentSession".into());
        }
        Ok(())
    }
}

/// What dialogical Nara semantically requires of a voice body. This is the
/// requirement half of the provider-neutral contract: AIKit resolves bodies
/// against it; it does not implement or name any body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceBodyRequirements {
    pub schema: String,
    /// Full-duplex or at least streamed turn-taking.
    pub duplex: VoiceDuplexDisposition,
    /// Interruption/barge-in status must be disclosed.
    pub barge_in: VoiceCapabilityStatus,
    /// Manual stop must exist.
    pub manual_interrupt: VoiceCapabilityStatus,
    /// A structured event/tool request channel so deixis and ExpressiveAct
    /// operations can travel beside the speech.
    pub structured_event_channel: VoiceCapabilityStatus,
    /// Connection/reconnect/degraded status must be observable, so a
    /// reconnect preserves the canonical Nara/Expression instead of
    /// inventing a second dialogue.
    pub reconnect_status_reporting: VoiceCapabilityStatus,
    /// How the bounded context refreshes across turns.
    pub context_refresh: ContextRefreshDisposition,
}

impl VoiceBodyRequirements {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARA_VOICE_BODY_CONTRACT {
            return Err("unsupported Nara voice body contract".into());
        }
        Ok(())
    }

    /// The floor for foreground dialogical Nara: streamed interaction,
    /// disclosed barge-in, manual stop, a structured channel, observable
    /// reconnect status, and a context path that is not absent.
    pub fn dialogical_floor() -> Self {
        Self {
            schema: NARA_VOICE_BODY_CONTRACT.into(),
            duplex: VoiceDuplexDisposition::StreamedTurnTaking,
            barge_in: VoiceCapabilityStatus::Supported,
            manual_interrupt: VoiceCapabilityStatus::Supported,
            structured_event_channel: VoiceCapabilityStatus::Supported,
            reconnect_status_reporting: VoiceCapabilityStatus::Supported,
            context_refresh: ContextRefreshDisposition::ToolAccess,
        }
    }
}

/// What a body actually discloses about itself, plus the observation refs
/// (latency/material) kept for later fitness evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoiceBodyDeclaration {
    pub binding: VoiceBodyBinding,
    pub duplex: VoiceDuplexDisposition,
    pub barge_in: VoiceCapabilityStatus,
    pub manual_interrupt: VoiceCapabilityStatus,
    pub structured_event_channel: VoiceCapabilityStatus,
    pub reconnect_status_reporting: VoiceCapabilityStatus,
    pub context_refresh: ContextRefreshDisposition,
    pub observation_refs: Vec<String>,
}

impl VoiceBodyDeclaration {
    pub fn validate(&self) -> Result<(), String> {
        self.binding.validate()?;
        refs(&self.observation_refs, "voice observation reference", 256)
    }

    /// Check a declaration against requirements. Every unmet requirement is
    /// named; `unknown` and `unsupported` are refusals, not tolerances.
    pub fn satisfies(&self, requirements: &VoiceBodyRequirements) -> Result<(), String> {
        self.validate()?;
        requirements.validate()?;
        let mut unmet: Vec<String> = Vec::new();

        // Full-duplex satisfies a streamed-turn-taking requirement;
        // unknown satisfies nothing.
        let duplex_meets = match requirements.duplex {
            VoiceDuplexDisposition::Unknown => true,
            VoiceDuplexDisposition::StreamedTurnTaking => matches!(
                self.duplex,
                VoiceDuplexDisposition::StreamedTurnTaking | VoiceDuplexDisposition::FullDuplex
            ),
            VoiceDuplexDisposition::FullDuplex => self.duplex == VoiceDuplexDisposition::FullDuplex,
        };
        if !duplex_meets {
            unmet.push(format!(
                "duplex: requires at least {:?}, body discloses {:?}",
                requirements.duplex, self.duplex
            ));
        }
        for (name, required, disclosed) in [
            ("barge-in", requirements.barge_in, self.barge_in),
            (
                "manual interrupt",
                requirements.manual_interrupt,
                self.manual_interrupt,
            ),
            (
                "structured event channel",
                requirements.structured_event_channel,
                self.structured_event_channel,
            ),
            (
                "reconnect status reporting",
                requirements.reconnect_status_reporting,
                self.reconnect_status_reporting,
            ),
        ] {
            if required.satisfies() && !disclosed.satisfies() {
                unmet.push(format!(
                    "{name}: required supported, body discloses {disclosed:?}"
                ));
            }
        }
        let context_meets = match requirements.context_refresh {
            ContextRefreshDisposition::None => true,
            required => self.context_refresh == required,
        };
        if !context_meets {
            unmet.push(format!(
                "context refresh: requires {:?}, body discloses {:?}",
                requirements.context_refresh, self.context_refresh
            ));
        }
        if unmet.is_empty() {
            Ok(())
        } else {
            Err(unmet.join("; "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding() -> VoiceBodyBinding {
        VoiceBodyBinding {
            schema: NARA_VOICE_BODY_CONTRACT.into(),
            body_ref: "voice-body:1".into(),
            body_provenance_ref: "provenance:resolved-by-aikit".into(),
        }
    }

    fn declaration() -> VoiceBodyDeclaration {
        VoiceBodyDeclaration {
            binding: binding(),
            duplex: VoiceDuplexDisposition::FullDuplex,
            barge_in: VoiceCapabilityStatus::Supported,
            manual_interrupt: VoiceCapabilityStatus::Supported,
            structured_event_channel: VoiceCapabilityStatus::Supported,
            reconnect_status_reporting: VoiceCapabilityStatus::Supported,
            context_refresh: ContextRefreshDisposition::ToolAccess,
            observation_refs: vec!["observation:latency:1".into()],
        }
    }

    #[test]
    fn body_is_neither_nara_nor_session() {
        binding().binds_distinctly("nara:a", "session:1").unwrap();
        assert_eq!(
            binding()
                .binds_distinctly("voice-body:1", "session:1")
                .unwrap_err(),
            "the voice body is not the Nara"
        );
        assert_eq!(
            binding()
                .binds_distinctly("nara:a", "voice-body:1")
                .unwrap_err(),
            "the voice body is not the AgentSession"
        );
    }

    #[test]
    fn dialogical_floor_is_satisfied_by_a_capable_body() {
        declaration()
            .satisfies(&VoiceBodyRequirements::dialogical_floor())
            .unwrap();
    }

    #[test]
    fn unknown_never_satisfies_and_every_gap_is_named() {
        let mut body = declaration();
        body.duplex = VoiceDuplexDisposition::StreamedTurnTaking;
        body.barge_in = VoiceCapabilityStatus::Unknown;
        let error = body
            .satisfies(&VoiceBodyRequirements::dialogical_floor())
            .unwrap_err();
        assert!(error.contains("barge-in"), "{error}");

        let mut degraded = declaration();
        degraded.duplex = VoiceDuplexDisposition::Unknown;
        degraded.structured_event_channel = VoiceCapabilityStatus::Unknown;
        degraded.reconnect_status_reporting = VoiceCapabilityStatus::Unsupported;
        degraded.context_refresh = ContextRefreshDisposition::None;
        let error = degraded
            .satisfies(&VoiceBodyRequirements::dialogical_floor())
            .unwrap_err();
        assert!(error.contains("duplex"), "{error}");
        assert!(error.contains("structured event channel"), "{error}");
        assert!(error.contains("reconnect status reporting"), "{error}");
        assert!(error.contains("context refresh"), "{error}");
    }

    #[test]
    fn full_duplex_satisfies_a_streamed_requirement_but_not_versa() {
        let requirements = VoiceBodyRequirements {
            duplex: VoiceDuplexDisposition::StreamedTurnTaking,
            ..VoiceBodyRequirements::dialogical_floor()
        };
        declaration().satisfies(&requirements).unwrap();

        let mut turn_taking = declaration();
        turn_taking.duplex = VoiceDuplexDisposition::StreamedTurnTaking;
        turn_taking.satisfies(&requirements).unwrap();

        let strict = VoiceBodyRequirements {
            duplex: VoiceDuplexDisposition::FullDuplex,
            ..VoiceBodyRequirements::dialogical_floor()
        };
        assert!(turn_taking.satisfies(&strict).is_err());
    }
}
