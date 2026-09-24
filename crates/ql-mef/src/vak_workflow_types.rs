//! QL's Vāk vocabulary for Factory-owned typed workflow authoring (VW1).
//!
//! Factory owns the generic TypeScript entry, restricted parsing, lowering,
//! compiled unit identity and Commission. QL owns only the domain values and
//! their meaning. This module is the Rust contract from which the published
//! TypeScript types (`adapters/factory-workflow/index.d.ts`) are rendered; the
//! rendered file is never hand-edited and a workspace test fails on drift.
//!
//! An [`AuthoredCPrime`] is what a workflow author writes on one unit. It is a
//! requirement for conduct, not an execution receipt, identity or grant. Factory
//! lowers it to its native C′ execution binding (`ql.vak-composition.profile/v1`
//! plus `aikit.operative-scope/v1`); the actual Run/attempt stays Factory's.
use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ContextFrameId;
use crate::vak_composition::{CompositionError, Result};
use crate::vak_profile::{
    CPrimeProfile, ContentPosition, ContentType, ContextSequence, InquiryDirection,
    PROFILE_CONTRACT, Participation, ThreadForm, constitutional_voice,
};

/// Contract of the rendered TypeScript types module.
pub const WORKFLOW_TYPES_CONTRACT: &str = "ql.vak-workflow-types/v1";
/// Module specifier Factory's domain-adapter registry admits for these types.
pub const WORKFLOW_TYPES_SPECIFIER: &str = "@epilogos/ql-vak";
/// Repository path of the rendered module, relative to the QL-MEF root.
pub const WORKFLOW_TYPES_PATH: &str = "adapters/factory-workflow/index.d.ts";
/// Specialist offices are recovered from this source (historical blob pin).
pub const SPECIALIST_SOURCE: &str = "docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md";
pub const SPECIALIST_SOURCE_BLOB: &str = "038a0316d51d60443a8b1abbddd4db85e512dc52";
/// Aletheia is S5′ disclosure/crystallisation/Return, not an eighth CF peer.
pub const DISCLOSURE_ORGAN: &str = "Aletheia";
const MAX_REF: usize = 2048;
const MAX_SOURCES: usize = 256;

fn error(message: impl Into<String>) -> CompositionError {
    CompositionError(message.into())
}

/// Named specialists preserved by the full-field lock. They are selectable
/// contributions, not a compulsory swarm and not constitutional frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Specialist {
    Anansi,
    Janus,
    Moirai,
    Mercurius,
    Agora,
    Zeithoven,
}
impl Specialist {
    pub const ALL: [Self; 6] = [
        Self::Anansi,
        Self::Janus,
        Self::Moirai,
        Self::Mercurius,
        Self::Agora,
        Self::Zeithoven,
    ];
    pub const fn name(self) -> &'static str {
        match self {
            Self::Anansi => "Anansi",
            Self::Janus => "Janus",
            Self::Moirai => "Moirai",
            Self::Mercurius => "Mercurius",
            Self::Agora => "Agora",
            Self::Zeithoven => "Zeithoven",
        }
    }
    /// The office as the full-field lock and the Vāk Expression wayfinder state it.
    pub const fn office(self) -> &'static str {
        match self {
            Self::Anansi => {
                "Investigate gaps and the authored/implemented relation; coordinate and blueprint work."
            }
            Self::Janus => "Relate temporal occasions; temporal and threshold work.",
            Self::Moirai => {
                "Supply Klotho/Lachesis/Atropos evidence, query and reflection modes; GraphRAG distillation."
            }
            Self::Mercurius => {
                "Carry cross-domain translation; Kairos and qualitative temporal patterns."
            }
            Self::Agora => {
                "Bring plural contributions into evaluation; aggregation and skill/plugin absorption."
            }
            Self::Zeithoven => "Develop the next score; creative advance and skill/agent creation.",
        }
    }
}

/// A constitutional frame on the wire (`CF1`–`CF7`). The frame's lens and
/// harmonic basis stay with the QL binding named by `interpretation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoredFrame(pub ContextFrameId);
impl Serialize for AuthoredFrame {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.code())
    }
}
impl<'de> Deserialize<'de> for AuthoredFrame {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let code = String::deserialize(deserializer)?;
        ContextFrameId::ALL
            .into_iter()
            .find(|frame| frame.code() == code)
            .map(Self)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown context frame {code}")))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRevision {
    #[serde(rename = "ref")]
    pub reference: String,
    pub revision: String,
}

/// One unit's authored C′. Field spellings are the TypeScript surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoredCPrime {
    #[serde(rename = "CPF")]
    pub participation: Participation,
    #[serde(rename = "CT")]
    pub content: ContentType,
    #[serde(rename = "CP")]
    pub position: ContentPosition,
    #[serde(rename = "CF")]
    pub frame: AuthoredFrame,
    #[serde(rename = "CFP")]
    pub thread: ThreadForm,
    #[serde(rename = "CS")]
    pub sequence: ContextSequence,
    pub direction: InquiryDirection,
    /// The declared participant performing this voice (a native Agent ref).
    pub actor: String,
    /// The QL operative binding carrying the whole's lens and harmonic basis.
    pub interpretation: SourceRevision,
    pub whole: String,
    /// AIKit resolve path and context resolution; AIKit remains their owner.
    #[serde(rename = "resolvePath")]
    pub resolve_path: String,
    #[serde(rename = "contextResolution")]
    pub context_resolution: String,
    pub sources: Vec<String>,
    /// Required exactly when participation is an authorised undertaking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
}

fn qualified(value: &str, field: &str) -> Result<()> {
    let boundary = value.find([':', '/']);
    if value.is_empty()
        || value.len() > MAX_REF
        || value.chars().any(|c| c.is_whitespace() || c.is_control())
        || matches!(boundary, None | Some(0))
        || boundary == Some(value.len() - 1)
    {
        return Err(error(format!(
            "{field} must be one bounded owner-qualified reference"
        )));
    }
    Ok(())
}

impl AuthoredCPrime {
    /// Structural validation owned by QL. Factory repeats its own native
    /// binding validation after lowering; neither check grants execution.
    pub fn validate(&self) -> Result<()> {
        qualified(&self.actor, "actor")?;
        qualified(&self.interpretation.reference, "interpretation.ref")?;
        if self.interpretation.revision.trim().is_empty()
            || self.interpretation.revision.len() > MAX_REF
        {
            return Err(error("interpretation.revision is required"));
        }
        qualified(&self.whole, "whole")?;
        qualified(&self.resolve_path, "resolvePath")?;
        qualified(&self.context_resolution, "contextResolution")?;
        if self.sources.is_empty() || self.sources.len() > MAX_SOURCES {
            return Err(error("sources must name 1..256 source references"));
        }
        let mut unique = BTreeSet::new();
        for source in &self.sources {
            qualified(source, "sources")?;
            if !unique.insert(source) {
                return Err(error(format!("duplicate source {source}")));
            }
        }
        match (self.participation.requires_undertaking(), &self.authority) {
            (true, Some(authority)) => qualified(authority, "authority"),
            (true, None) => Err(error(
                "an authorised undertaking names its undertaking authority",
            )),
            (false, Some(_)) => Err(error(
                "a dialogical composition carries no undertaking authority",
            )),
            (false, None) => Ok(()),
        }
    }

    pub fn profile(&self) -> CPrimeProfile {
        CPrimeProfile {
            participation: self.participation,
            content: self.content,
            position: self.position,
            thread: self.thread,
            sequence: self.sequence,
            direction: self.direction,
        }
    }

    pub const fn constitutional_voice(&self) -> &'static str {
        constitutional_voice(self.frame.0)
    }
}

/// Exhaustive variant lists. A new Rust variant fails to compile here until
/// it is placed, so the rendered union can never silently omit a value.
fn participations() -> [Participation; 2] {
    let all = [
        Participation::Dialogical,
        Participation::AuthorisedUndertaking,
    ];
    for value in all {
        match value {
            Participation::Dialogical | Participation::AuthorisedUndertaking => {}
        }
    }
    all
}
fn directions() -> [InquiryDirection; 2] {
    let all = [InquiryDirection::Forward, InquiryDirection::Returning];
    for value in all {
        match value {
            InquiryDirection::Forward | InquiryDirection::Returning => {}
        }
    }
    all
}

fn wire<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .expect("QL C-prime enum serialises as a string")
}
fn literal(value: &str) -> String {
    serde_json::to_string(value).expect("string literal")
}
fn union<T: Serialize>(values: impl IntoIterator<Item = T>) -> String {
    values
        .into_iter()
        .map(|value| literal(&wire(&value)))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// The authored C′ fields as rendered, in contract order. The workspace test
/// proves this list equals the serde surface of [`AuthoredCPrime`].
pub const CPRIME_FIELDS: [&str; 14] = [
    "CPF",
    "authority",
    "CT",
    "CP",
    "CF",
    "CFP",
    "CS",
    "direction",
    "actor",
    "interpretation",
    "whole",
    "resolvePath",
    "contextResolution",
    "sources",
];

/// Render the published TypeScript types module from this Rust contract.
pub fn typescript_module() -> String {
    let mut out = String::new();
    out.push_str(
        "// Generated by `ql vak workflow-types` from the QL-MEF Rust contract\n\
         // (crates/ql-mef/src/vak_workflow_types.rs over vak_profile.rs and context_frame.rs).\n\
         // Do not hand-edit: crates/ql-mef/tests/vak_workflow_types.rs fails on drift.\n\
         // Types only. Factory's restricted workflow compiler admits this module through its\n\
         // registered domain adapter and lowers a unit's `composition` to its native C-prime\n\
         // execution binding. Nothing here grants execution, identity or authority.\n\n",
    );
    let line = |out: &mut String, doc: &str, body: String| {
        if !doc.is_empty() {
            out.push_str(&format!("/** {doc} */\n"));
        }
        out.push_str(&body);
        out.push_str("\n\n");
    };
    line(
        &mut out,
        "",
        format!(
            "export type WorkflowTypesContract = {};",
            literal(WORKFLOW_TYPES_CONTRACT)
        ),
    );
    line(
        &mut out,
        "The QL profile contract the lowered binding declares.",
        format!(
            "export type ProfileContract = {};",
            literal(PROFILE_CONTRACT)
        ),
    );
    line(
        &mut out,
        "CPF: dialogical or authorised-undertaking participation.",
        format!("export type Participation = {};", union(participations())),
    );
    line(
        &mut out,
        "CT: content kind; CT4b' specialises contextual content.",
        format!("export type ContentType = {};", union(ContentType::ALL)),
    );
    line(
        &mut out,
        "CP: 4.0 ground, 4.1 definition, 4.2 operation, 4.3 pattern, 4.4 context, 4.5 integration.",
        format!(
            "export type ContentPosition = {};",
            union(ContentPosition::ALL)
        ),
    );
    line(
        &mut out,
        "CF: the source-defined constitutional/relational frame.",
        format!(
            "export type ContextFrame = {};",
            ContextFrameId::ALL
                .iter()
                .map(|frame| literal(frame.code()))
                .collect::<Vec<_>>()
                .join(" | ")
        ),
    );
    line(
        &mut out,
        "CFP0-5: single voice, parallel/chord, chain/melody, fusion, sustained/drone, nested/canon.",
        format!("export type ThreadForm = {};", union(ThreadForm::ALL)),
    );
    line(
        &mut out,
        "CS0-5: exact source-defined paired passage; direction and extent stay distinct.",
        format!(
            "export type ContextSequence = {};",
            union(ContextSequence::ALL)
        ),
    );
    line(
        &mut out,
        "",
        format!("export type InquiryDirection = {};", union(directions())),
    );
    line(
        &mut out,
        "Musical role of each thread form.",
        format!(
            "export type ThreadMusicalRole = {{\n{}\n}};",
            ThreadForm::ALL
                .iter()
                .map(|form| format!(
                    "  readonly {}: {};",
                    wire(form),
                    literal(form.musical_role())
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    );
    line(
        &mut out,
        "Each passage as its retained forward/returning position pairs.",
        format!(
            "export type Passages = {{\n{}\n}};",
            ContextSequence::ALL
                .iter()
                .map(|sequence| format!(
                    "  readonly {}: readonly [{}];",
                    wire(sequence),
                    sequence
                        .pairs()
                        .iter()
                        .map(|pair| format!(
                            "readonly [{}, {}]",
                            literal(&wire(&pair.forward)),
                            literal(&wire(&pair.returning))
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    );
    line(
        &mut out,
        "Constitutional voices hosted by Anima; role contributions, not product identities.",
        format!(
            "export type ConstitutionalVoice = {};",
            ContextFrameId::ALL
                .iter()
                .map(|frame| literal(constitutional_voice(*frame)))
                .collect::<Vec<_>>()
                .join(" | ")
        ),
    );
    line(
        &mut out,
        "Each constitutional voice's source frame.",
        format!(
            "export type VoiceFrame = {{\n{}\n}};",
            ContextFrameId::ALL
                .iter()
                .map(|frame| format!(
                    "  readonly {}: {};",
                    constitutional_voice(*frame),
                    literal(frame.code())
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    );
    line(
        &mut out,
        "Named specialists: selectable contributions, never a compulsory swarm.",
        format!(
            "export type Specialist = {};",
            Specialist::ALL
                .iter()
                .map(|specialist| literal(specialist.name()))
                .collect::<Vec<_>>()
                .join(" | ")
        ),
    );
    line(
        &mut out,
        "Specialist offices as the full-field lock states them.",
        format!(
            "export type SpecialistOffice = {{\n{}\n}};",
            Specialist::ALL
                .iter()
                .map(|specialist| format!(
                    "  readonly {}: {};",
                    specialist.name(),
                    literal(specialist.office())
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    );
    line(
        &mut out,
        "S5-prime disclosure/crystallisation/Return; not an eighth constitutional peer.",
        format!(
            "export type DisclosureOrgan = {};",
            literal(DISCLOSURE_ORGAN)
        ),
    );
    line(
        &mut out,
        "",
        "export type SourceRevision = {\n  readonly ref: string;\n  readonly revision: string;\n};"
            .into(),
    );
    let participation = participations()
        .iter()
        .map(|value| {
            if value.requires_undertaking() {
                format!(
                    "{{ readonly CPF: {}; readonly authority: string }}",
                    literal(&wire(value))
                )
            } else {
                format!(
                    "{{ readonly CPF: {}; readonly authority?: never }}",
                    literal(&wire(value))
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n  | ");
    line(
        &mut out,
        "An authorised undertaking names its authority; a dialogical act carries none.",
        format!("export type CPrimeParticipation =\n  | {participation};"),
    );
    line(
        &mut out,
        "One unit's authored C-prime. Factory lowers it to its native execution binding; \
         the unit's subjectRef is the binding subject and `actor` must be a declared participant.",
        "export type CPrime = CPrimeParticipation & {\n  \
         readonly CT: ContentType;\n  \
         readonly CP: ContentPosition;\n  \
         readonly CF: ContextFrame;\n  \
         readonly CFP: ThreadForm;\n  \
         readonly CS: ContextSequence;\n  \
         readonly direction: InquiryDirection;\n  \
         readonly actor: string;\n  \
         readonly interpretation: SourceRevision;\n  \
         readonly whole: string;\n  \
         readonly resolvePath: string;\n  \
         readonly contextResolution: string;\n  \
         readonly sources: ReadonlyArray<string>;\n\
         };"
        .into(),
    );
    out.truncate(out.trim_end().len());
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_constitutional_voice_and_specialist_is_rendered() {
        let module = typescript_module();
        for frame in ContextFrameId::ALL {
            assert!(module.contains(&format!(
                "readonly {}: \"{}\";",
                constitutional_voice(frame),
                frame.code()
            )));
        }
        for specialist in Specialist::ALL {
            assert!(module.contains(&format!("\"{}\"", specialist.name())));
        }
        assert!(module.contains("\"CT4b'\""));
    }
}
