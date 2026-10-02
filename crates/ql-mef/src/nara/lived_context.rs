//! Nara's lived context: the person's relevant Day/Flow history, selected for a
//! present concern and delivered with exact source identity (QL-MEF #258 EA1).
//!
//! The host reads the material from its Central owner (`central.document.read`
//! readings, or other carriers mapped to [`LivedEntry`]) and passes it here.
//! This composer selects what bears on the concern and the continuing journey,
//! keeps human writing, agent interpretation and operational records apart,
//! reads corrections as corrections, and returns the exact basis it selected,
//! with a revision the delivered context can be checked against. It never
//! writes to the person's source.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::dialogue::{AdmittedOccasion, DisclosedRef, DisclosureKind};
use super::domain::{
    EvidenceStanding, JourneyReadingEntry, LiveState, OracleJourney, PersonDisposition,
    card_identity,
};

pub const LIVED_CONTEXT_SCHEMA: &str = "ql.nara-lived-context/v1";
const CENTRAL_DOCUMENT_READING: &str = "central.document-reading/v1";
const MAX_CANDIDATES: usize = 4096;
const MAX_PASSAGE_TEXT: usize = 64 * 1024;
const LATE_RECEIPT_MS: u64 = 24 * 60 * 60 * 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MaterialKind {
    Day,
    Flow,
    Dialogue,
    Journal,
    Dream,
    Import,
    Annotation,
    Return,
}

/// Who produced the material. Operational T/T′ belongs to an agent's NOW/Run
/// context and is never presented as an event in the person's life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LivedActor {
    Human,
    Agent,
    Operational,
}

/// The standing of what the passage says, as its author gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MaterialMode {
    Current,
    Quoted,
    Remembered,
    Imagined,
    Negated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LivedEntry {
    pub source_ref: String,
    pub revision: String,
    pub document_id: Option<String>,
    pub entry_id: Option<String>,
    pub contribution_id: Option<String>,
    /// A span or other selector inside the entry, where the carrier has one.
    pub selector: Option<String>,
    pub kind: MaterialKind,
    pub actor: LivedActor,
    pub author_ref: String,
    pub mode: MaterialMode,
    pub text: String,
    pub day_ref: Option<String>,
    pub occurred_at_unix_ms: u64,
    pub received_at_unix_ms: u64,
    /// The passage key this entry corrects, when it is a correction.
    pub corrects: Option<String>,
    /// The carrier reported a revision nobody has reviewed yet.
    #[serde(default)]
    pub unreviewed_revision: bool,
}

impl LivedEntry {
    /// Exact passage identity: source, document-local address and revision.
    pub fn key(&self) -> String {
        let mut address = String::new();
        for part in [&self.document_id, &self.entry_id, &self.contribution_id]
            .into_iter()
            .flatten()
        {
            address.push('/');
            address.push_str(part);
        }
        if let Some(selector) = &self.selector {
            address.push('#');
            address.push_str(selector);
        }
        format!("{}{}@{}", self.source_ref, address, self.revision)
    }

    fn validate(&self) -> Result<(), String> {
        for (value, label) in [
            (&self.source_ref, "lived source"),
            (&self.revision, "lived revision"),
            (&self.author_ref, "lived author"),
        ] {
            if value.trim().is_empty() || value.len() > 4096 {
                return Err(format!("invalid {label}"));
            }
        }
        if self.text.trim().is_empty() || self.text.len() > MAX_PASSAGE_TEXT {
            return Err(format!("passage {} has no bounded text", self.key()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LivedConcern {
    pub concern_ref: String,
    pub title: String,
    pub terms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LivedBudget {
    pub max_passages: usize,
    pub max_chars: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LivedContextRequest {
    pub subject_ref: String,
    pub concern: LivedConcern,
    pub occasion: Option<AdmittedOccasion>,
    /// `central.document-reading/v1` results exactly as Central returned them.
    #[serde(default)]
    pub documents: Vec<Value>,
    /// Material from other carriers, already mapped by its host.
    #[serde(default)]
    pub entries: Vec<LivedEntry>,
    pub journey: Option<OracleJourney>,
    pub budget: LivedBudget,
    /// The consent/authority receipt through which personal material entered.
    pub disclosed_via_ref: String,
    pub composed_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SelectionReason {
    Correction { corrects: String },
    RecognisesPlacement { placement_ref: String },
    JourneyBasis { journey_ref: String },
    NamesCard { card: String },
    ConcernTerm { term: String },
    CurrentDay,
}

impl SelectionReason {
    fn weight(&self) -> u8 {
        match self {
            Self::Correction { .. } => 6,
            Self::RecognisesPlacement { .. } => 5,
            Self::JourneyBasis { .. } => 4,
            Self::NamesCard { .. } => 3,
            Self::ConcernTerm { .. } => 2,
            Self::CurrentDay => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivedPassage {
    pub key: String,
    pub source_ref: String,
    pub revision: String,
    pub document_id: Option<String>,
    pub entry_id: Option<String>,
    pub contribution_id: Option<String>,
    pub selector: Option<String>,
    pub kind: MaterialKind,
    pub actor: LivedActor,
    pub author_ref: String,
    pub mode: MaterialMode,
    pub standing: EvidenceStanding,
    pub day_ref: Option<String>,
    pub occurred_at_unix_ms: u64,
    pub received_at_unix_ms: u64,
    pub late_receipt: bool,
    pub unreviewed_revision: bool,
    pub superseded_by: Option<String>,
    pub reasons: Vec<SelectionReason>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JourneyPlacementSummary {
    pub placement_ref: String,
    pub card: String,
    pub reversed: bool,
    pub position: Option<String>,
    pub live_state: LiveState,
    pub person_disposition: Option<PersonDisposition>,
    pub current_readings: Vec<JourneyReadingEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JourneySummary {
    pub journey_ref: String,
    pub journey_revision: u64,
    pub deck_ref: String,
    pub dealt: u16,
    pub remaining: u16,
    pub day_refs: Vec<String>,
    pub live_placements: Vec<JourneyPlacementSummary>,
    pub latest_iching: Option<IChingSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IChingSummary {
    pub reading_ref: String,
    pub basis: String,
    pub primary: u8,
    pub resulting: Option<u8>,
    pub current_readings: Vec<JourneyReadingEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Omission {
    pub key: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribution {
    pub human: usize,
    pub agent: usize,
    pub operational: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivedContext {
    pub schema: String,
    /// Content revision of exactly what was selected; the delivered context
    /// is checked against it.
    pub context_revision: String,
    pub subject_ref: String,
    pub concern: LivedConcern,
    pub occasion: Option<AdmittedOccasion>,
    pub passages: Vec<LivedPassage>,
    pub journey: Option<JourneySummary>,
    pub disclosed: Vec<DisclosedRef>,
    pub attribution: Attribution,
    pub candidates: usize,
    pub omitted: Vec<Omission>,
    pub composed_at_unix_ms: u64,
}

fn strip_html(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for character in html.chars() {
        match character {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(character),
            _ => {}
        }
    }
    let decoded = out
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Map one `central.document-reading/v1` result to lived entries. Removed
/// contributions are excluded; nothing is inferred beyond what Central holds.
pub fn entries_from_central_document(reading: &Value) -> Result<Vec<LivedEntry>, String> {
    if reading["schema"] != CENTRAL_DOCUMENT_READING {
        return Err("expected a central.document-reading/v1 result".into());
    }
    let source_ref = reading["source"]["source_ref"]
        .as_str()
        .ok_or("document reading lacks source.source_ref")?;
    let revision = reading["revision"]["revision"]
        .as_str()
        .ok_or("document reading lacks revision.revision")?;
    let document = &reading["document"];
    let document_id = document["document_id"]
        .as_str()
        .ok_or("document reading lacks document_id")?;
    let kind = match document["kind"].as_str() {
        Some("day") => MaterialKind::Day,
        Some("flow") => MaterialKind::Flow,
        Some("dialogue") => MaterialKind::Dialogue,
        _ => return Err("document kind is day, flow or dialogue".into()),
    };
    let unreviewed = reading["unreviewed_external_revision"]
        .as_bool()
        .unwrap_or(false);
    let day_ref = document["day_ref"].as_str().map(str::to_string);
    let mut entries = Vec::new();
    for contribution in document["contributions"]
        .as_array()
        .ok_or("document lacks contributions")?
    {
        if contribution["removed"].as_bool().unwrap_or(false) {
            continue;
        }
        let text = strip_html(contribution["html"].as_str().unwrap_or_default());
        if text.is_empty() {
            continue;
        }
        let actor = match contribution["actor_kind"].as_str() {
            Some("human") => LivedActor::Human,
            Some("agent") => LivedActor::Agent,
            _ => LivedActor::Operational,
        };
        let seconds = |key: &str| contribution[key].as_u64().unwrap_or(0).saturating_mul(1000);
        entries.push(LivedEntry {
            source_ref: source_ref.to_string(),
            revision: revision.to_string(),
            document_id: Some(document_id.to_string()),
            entry_id: contribution["entry_id"].as_str().map(str::to_string),
            contribution_id: contribution["id"].as_str().map(str::to_string),
            selector: None,
            kind,
            actor,
            author_ref: contribution["author_ref"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            mode: MaterialMode::Current,
            text,
            day_ref: day_ref.clone(),
            occurred_at_unix_ms: seconds("occurred_at_unix_seconds"),
            received_at_unix_ms: seconds("received_at_unix_seconds"),
            corrects: None,
            unreviewed_revision: unreviewed,
        });
    }
    Ok(entries)
}

fn contains_term(text: &str, term: &str) -> bool {
    let term = term.trim().to_lowercase();
    if term.is_empty() {
        return false;
    }
    let bytes = text.as_bytes();
    let mut start = 0;
    while let Some(found) = text[start..].find(&term) {
        let begin = start + found;
        let end = begin + term.len();
        let before = begin == 0 || !bytes[begin - 1].is_ascii_alphanumeric();
        let after = end == bytes.len() || !bytes[end].is_ascii_alphanumeric();
        if before && after {
            return true;
        }
        start = begin + 1;
        while !text.is_char_boundary(start) {
            start += 1;
        }
    }
    false
}

fn standing_of(actor: LivedActor) -> EvidenceStanding {
    match actor {
        LivedActor::Human => EvidenceStanding::Reported,
        LivedActor::Agent => EvidenceStanding::Derived,
        LivedActor::Operational => EvidenceStanding::Observed,
    }
}

fn digest_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn journey_summary(journey: &OracleJourney) -> Result<JourneySummary, String> {
    let reading = journey.reading(None)?;
    let live = reading
        .spreads
        .iter()
        .flat_map(|spread| spread.placements.iter())
        .chain(reading.symbolic_assignments.iter())
        .filter(|placement| {
            reading
                .live_placement_refs
                .contains(&placement.placement_ref)
        })
        .map(|placement| JourneyPlacementSummary {
            placement_ref: placement.placement_ref.clone(),
            card: placement.card.name.clone(),
            reversed: placement.reversed,
            position: placement.position.clone(),
            live_state: placement.live_state,
            person_disposition: placement.person_disposition,
            current_readings: placement.current_readings.clone(),
        })
        .collect();
    let latest_iching = journey.iching.last().map(|reading| {
        let superseded = reading
            .readings
            .iter()
            .filter_map(|entry| entry.supersedes.clone())
            .collect::<BTreeSet<_>>();
        IChingSummary {
            reading_ref: reading.reading_ref.clone(),
            basis: match reading.basis {
                super::domain::IChingBasis::Cast { .. } => "cast".into(),
                super::domain::IChingBasis::Computed { .. } => "computed".into(),
            },
            primary: reading.primary,
            resulting: reading.resulting,
            current_readings: reading
                .readings
                .iter()
                .filter(|entry| !superseded.contains(&entry.reading_ref))
                .cloned()
                .collect(),
        }
    });
    Ok(JourneySummary {
        journey_ref: journey.journey_ref.clone(),
        journey_revision: journey.revision,
        deck_ref: journey.deck.deck_ref.clone(),
        dealt: journey.dealt,
        remaining: journey.remaining(),
        day_refs: journey.day_refs.clone(),
        live_placements: live,
        latest_iching,
    })
}

/// Compose the lived context for one concern.
pub fn compose(request: LivedContextRequest) -> Result<LivedContext, String> {
    if request.subject_ref.trim().is_empty() || request.disclosed_via_ref.trim().is_empty() {
        return Err("lived context requires a subject and a disclosure receipt".into());
    }
    if request.budget.max_passages == 0 || request.budget.max_chars == 0 {
        return Err("lived context requires a positive budget".into());
    }
    if let Some(occasion) = &request.occasion {
        occasion.validate()?;
    }
    let journey = request.journey.as_ref();
    if let Some(journey) = journey {
        journey.validate()?;
        // A continuing journey can span Days, but it cannot silently change
        // whose experience or which concern this context is being composed for.
        if journey.subject_id != request.subject_ref {
            return Err("oracle journey belongs to a different lived-context subject".into());
        }
        if journey.concern.concern_ref != request.concern.concern_ref {
            return Err("oracle journey belongs to a different lived-context concern".into());
        }
    }
    let mut candidates = Vec::new();
    for document in &request.documents {
        candidates.extend(entries_from_central_document(document)?);
    }
    candidates.extend(request.entries.iter().cloned());
    if candidates.len() > MAX_CANDIDATES {
        return Err("too many lived-context candidates".into());
    }
    let mut seen = BTreeSet::new();
    for entry in &candidates {
        entry.validate()?;
        if !seen.insert(entry.key()) {
            return Err(format!("duplicate lived passage {}", entry.key()));
        }
    }

    // Corrections: a correction supersedes a passage; both stay attributable.
    let mut superseded_by = BTreeMap::new();
    for entry in &candidates {
        if let Some(target) = &entry.corrects {
            if !seen.contains(target) {
                return Err(format!("correction names an unknown passage {target}"));
            }
            superseded_by.insert(target.clone(), entry.key());
        }
    }

    let live_cards: Vec<String> = match journey {
        Some(journey) => journey
            .placements
            .iter()
            .filter(|placement| placement.live_state != LiveState::Mute)
            .map(|placement| card_identity(&journey.deck, placement.card).map(|card| card.name))
            .collect::<Result<BTreeSet<_>, _>>()?
            .into_iter()
            .collect(),
        None => Vec::new(),
    };

    let mut scored = Vec::new();
    for entry in &candidates {
        let key = entry.key();
        let lower = entry.text.to_lowercase();
        let mut reasons = Vec::new();
        if let Some(target) = &entry.corrects {
            reasons.push(SelectionReason::Correction {
                corrects: target.clone(),
            });
        }
        if let Some(journey) = journey {
            let address = entry.entry_id.as_deref().or(entry.selector.as_deref());
            for placement in &journey.placements {
                if placement.recognitions.iter().any(|recognition| {
                    recognition.source.source_ref == entry.source_ref
                        && recognition.source.revision == entry.revision
                        && recognition
                            .entry_selector
                            .as_deref()
                            .is_none_or(|selector| Some(selector) == address)
                }) {
                    reasons.push(SelectionReason::RecognisesPlacement {
                        placement_ref: placement.placement_ref.clone(),
                    });
                }
            }
            let basis = journey.concern.basis_sources.iter().chain(
                journey
                    .spreads
                    .iter()
                    .flat_map(|spread| spread.basis_sources.iter()),
            );
            if basis.into_iter().any(|source| {
                source.source_ref == entry.source_ref && source.revision == entry.revision
            }) {
                reasons.push(SelectionReason::JourneyBasis {
                    journey_ref: journey.journey_ref.clone(),
                });
            }
        }
        for card in &live_cards {
            if contains_term(&lower, card) {
                reasons.push(SelectionReason::NamesCard { card: card.clone() });
            }
        }
        for term in &request.concern.terms {
            if contains_term(&lower, term) {
                reasons.push(SelectionReason::ConcernTerm { term: term.clone() });
            }
        }
        if let (Some(occasion), Some(day)) = (&request.occasion, &entry.day_ref)
            && occasion.day_ref == *day
        {
            reasons.push(SelectionReason::CurrentDay);
        }
        reasons.sort();
        reasons.dedup();
        if reasons.is_empty() {
            continue;
        }
        let score: u32 = reasons
            .iter()
            .map(|reason| u32::from(reason.weight()))
            .sum();
        let top = reasons
            .iter()
            .map(SelectionReason::weight)
            .max()
            .unwrap_or(0);
        scored.push((top, score, entry, key, reasons));
    }
    scored.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then(b.1.cmp(&a.1))
            .then(b.2.occurred_at_unix_ms.cmp(&a.2.occurred_at_unix_ms))
            .then(a.3.cmp(&b.3))
    });

    let mut passages = Vec::new();
    let mut omitted = Vec::new();
    let mut used = 0_usize;
    for (_, _, entry, key, reasons) in scored {
        if passages.len() >= request.budget.max_passages {
            omitted.push(Omission {
                key,
                reason: "passage budget reached".into(),
            });
            continue;
        }
        if used + entry.text.len() > request.budget.max_chars {
            omitted.push(Omission {
                key,
                reason: "character budget reached".into(),
            });
            continue;
        }
        used += entry.text.len();
        passages.push(LivedPassage {
            superseded_by: superseded_by.get(&key).cloned(),
            key,
            source_ref: entry.source_ref.clone(),
            revision: entry.revision.clone(),
            document_id: entry.document_id.clone(),
            entry_id: entry.entry_id.clone(),
            contribution_id: entry.contribution_id.clone(),
            selector: entry.selector.clone(),
            kind: entry.kind,
            actor: entry.actor,
            author_ref: entry.author_ref.clone(),
            mode: entry.mode,
            standing: standing_of(entry.actor),
            day_ref: entry.day_ref.clone(),
            occurred_at_unix_ms: entry.occurred_at_unix_ms,
            received_at_unix_ms: entry.received_at_unix_ms,
            late_receipt: entry
                .received_at_unix_ms
                .saturating_sub(entry.occurred_at_unix_ms)
                > LATE_RECEIPT_MS,
            unreviewed_revision: entry.unreviewed_revision,
            reasons,
            text: entry.text.clone(),
        });
    }

    let journey_summary = journey.map(journey_summary).transpose()?;
    let mut disclosed: Vec<DisclosedRef> = passages
        .iter()
        .map(|passage| DisclosedRef {
            ref_id: passage.key.clone(),
            revision: passage.revision.clone(),
            standing: passage.standing,
            disclosure: DisclosureKind::PersonalConsent,
            disclosed_via_ref: request.disclosed_via_ref.clone(),
        })
        .collect();
    if let Some(summary) = &journey_summary {
        disclosed.push(DisclosedRef {
            ref_id: summary.journey_ref.clone(),
            revision: format!("journey-revision:{}", summary.journey_revision),
            standing: EvidenceStanding::Derived,
            disclosure: DisclosureKind::PersonalConsent,
            disclosed_via_ref: request.disclosed_via_ref.clone(),
        });
    }
    let attribution = Attribution {
        human: passages
            .iter()
            .filter(|passage| passage.actor == LivedActor::Human)
            .count(),
        agent: passages
            .iter()
            .filter(|passage| passage.actor == LivedActor::Agent)
            .count(),
        operational: passages
            .iter()
            .filter(|passage| passage.actor == LivedActor::Operational)
            .count(),
    };

    let basis = serde_json::to_vec(&(
        &request.subject_ref,
        &request.concern,
        &request.occasion,
        &passages,
        &journey_summary,
    ))
    .map_err(|error| error.to_string())?;
    Ok(LivedContext {
        schema: LIVED_CONTEXT_SCHEMA.into(),
        context_revision: format!("sha256:{}", digest_hex(&basis)),
        subject_ref: request.subject_ref,
        concern: request.concern,
        occasion: request.occasion,
        passages,
        journey: journey_summary,
        disclosed,
        attribution,
        candidates: candidates.len(),
        omitted,
        composed_at_unix_ms: request.composed_at_unix_ms,
    })
}
