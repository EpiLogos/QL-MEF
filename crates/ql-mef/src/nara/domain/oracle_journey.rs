//! M4.2 continuing oracle journey: one Tarot deck and one I-Ching change
//! history across a Day/Flow concern.
//!
//! Source basis:
//! - Quaternal Tarot protocol (`nara-personal` `context/quaternal_tarot_protocol.md`
//!   §II spreads, §VII P4 lemniscate, §XI practice): the deck is shuffled once,
//!   a portion is turned around during the shuffle (which is what opens the
//!   field for reversals), and cards are then drawn in sequence. The owner's
//!   recorded journey (`tarot_journey_complete.md`) keeps one shuffle per deck,
//!   draws "when felt right" and closes the deck when it is exhausted.
//! - Janus spread aliveness (`Epi-Logos-C-Experiments`
//!   `Body/S/S4/ta-onta/S4-5p-aletheia/modules/janus-doorway.ts`, blob
//!   `a68352c2`, commit `ba6c1c3c`, `janus_track_spreads` /
//!   `janus_evaluate_aliveness` / `janus_spread_resolved`), ported with its
//!   thresholds. Two deliberate adaptations: a recognition is keyed by its exact
//!   source revision/selector so replaying the same Day/Flow material never
//!   counts twice, and terms match on word boundaries so "Art" is not
//!   recognised inside "part".
//! - I-Ching line/bit law of the C cast (`vendor/epi-kernel/reference/src/m4.c`
//!   `m4_cast_iching`): line i (bottom = 0) sets bit i when yang; 6/9 change;
//!   the resulting hexagram is `bits ^ changing_mask`.
//!
//! The journey is computed here and persisted by its Central owner. Every
//! mutation is an attributed operation keyed by a request id, so a replayed
//! request reconciles to its first effect instead of dealing again.

use std::collections::BTreeSet;

use ql_core::nuclear_hexagram;
use ql_core::{MAJOR_ARCANA_COUNT, MINOR_ARCANA_COUNT, TarotBridge, TarotSuit};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::nara::{ConsentState, SourceRevision};

use super::operations::{EntropyCursor, OracleCastContext};
use super::{
    OracleEntropyReceipt, OracleHygiene, OracleOriginalPacket, OracleRecord, OracleSystem,
    ProtectedRef, check_refs, check_source, check_text,
};

pub const ORACLE_JOURNEY_SCHEMA: &str = "ql.nara-oracle-journey/v1";
pub const ORACLE_JOURNEY_READING_SCHEMA: &str = "ql.nara-oracle-journey-reading/v1";
pub const QUATERNAL_TAROT_PROTOCOL_REF: &str =
    "nara-personal:context/quaternal_tarot_protocol.md#shuffle-once-turn-portion-draw-in-sequence";
pub const JANUS_ALIVENESS_SOURCE_REF: &str =
    "epi-logos-c-experiments:Body/S/S4/ta-onta/S4-5p-aletheia/modules/janus-doorway.ts@ba6c1c3c";

const STANDARD_DECK_CARDS: u16 = (MINOR_ARCANA_COUNT + MAJOR_ARCANA_COUNT) as u16;
const MAX_EXTRA_CARDS: usize = 2;
const MAX_TEXT: usize = 16 * 1024;
const DAY_MS: u64 = 24 * 60 * 60 * 1000;
const HOUR_MS: u64 = 60 * 60 * 1000;

fn check_long_text(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > MAX_TEXT {
        return Err(format!("invalid {label}"));
    }
    Ok(())
}

fn digest_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

// ---------------------------------------------------------------------------
// Deck and card identity
// ---------------------------------------------------------------------------

/// The naming register the deck prints its cards in. One kernel deck, two
/// registers (`ql_core::pole::tarot`); the register never changes identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TarotRegister {
    Thoth,
    Rws,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeckDefinition {
    pub deck_ref: String,
    pub system: OracleSystem,
    pub register: TarotRegister,
    /// Physical cards beyond the 78 that this particular deck carries (for
    /// example a Joker the person reads with). Declared per deck, never
    /// assumed: the kernel's Fool/Universe operators are readings of existing
    /// Majors, not extra cards.
    pub extra_cards: Vec<String>,
    pub source_refs: Vec<String>,
}

impl DeckDefinition {
    pub fn validate(&self) -> Result<(), String> {
        check_text(&self.deck_ref, "deck")?;
        if !matches!(
            self.system,
            OracleSystem::TarotRws
                | OracleSystem::TarotThoth
                | OracleSystem::TarotMarseille
                | OracleSystem::TarotQl
        ) {
            return Err("journey deck requires a Tarot oracle system".into());
        }
        if self.extra_cards.len() > MAX_EXTRA_CARDS {
            return Err("journey deck declares too many extra cards".into());
        }
        check_refs(&self.extra_cards, "deck extra card", MAX_EXTRA_CARDS)?;
        check_refs(&self.source_refs, "deck source reference", 32)
    }

    pub fn card_count(&self) -> u16 {
        STANDARD_DECK_CARDS + self.extra_cards.len() as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arcana {
    Minor,
    Major,
    Extra,
}

/// A card's identity resolved against the kernel Tarot bridge. Deck indices
/// follow the kernel quaternion layout: Minors 0–55 (`suit × 14 + pip`),
/// Majors 56–77; declared extra cards follow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardIdentity {
    pub deck_index: u16,
    pub token_ref: String,
    pub name: String,
    pub arcana: Arcana,
    pub kernel_card_ref: Option<String>,
    pub codons: Vec<u8>,
}

fn suit_name(suit: TarotSuit, register: TarotRegister) -> &'static str {
    match (suit, register) {
        (TarotSuit::Pentacles, TarotRegister::Thoth) => "Disks",
        _ => suit.name(),
    }
}

pub fn card_identity(deck: &DeckDefinition, index: u16) -> Result<CardIdentity, String> {
    let bridge = TarotBridge::kernel();
    let token_ref = format!("tarot:{}:{index}", deck.deck_ref);
    if (index as usize) < MINOR_ARCANA_COUNT {
        let card = &bridge.minor()[index as usize];
        let rank = match deck.register {
            TarotRegister::Thoth => card.pip().thoth_name(),
            TarotRegister::Rws => card.pip().rws_name(),
        };
        return Ok(CardIdentity {
            deck_index: index,
            token_ref,
            name: format!("{rank} of {}", suit_name(card.suit(), deck.register)),
            arcana: Arcana::Minor,
            kernel_card_ref: Some(format!("ql.pole.tarot-bridge/v1#minor:{}", card.card_id())),
            codons: card.codons().map(|codon| codon.address()).collect(),
        });
    }
    if index < STANDARD_DECK_CARDS {
        let card = &bridge.major()[index as usize - MINOR_ARCANA_COUNT];
        return Ok(CardIdentity {
            deck_index: index,
            token_ref,
            name: card.name().to_string(),
            arcana: Arcana::Major,
            kernel_card_ref: Some(format!("ql.pole.tarot-bridge/v1#major:{}", card.card_id())),
            codons: Vec::new(),
        });
    }
    let extra = deck
        .extra_cards
        .get(usize::from(index - STANDARD_DECK_CARDS))
        .ok_or_else(|| format!("card index {index} is outside deck {}", deck.deck_ref))?;
    Ok(CardIdentity {
        deck_index: index,
        token_ref,
        name: extra.clone(),
        arcana: Arcana::Extra,
        kernel_card_ref: None,
        codons: Vec::new(),
    })
}

/// The one shuffle of the journey's deck. `order[i]` is the card at deck
/// position i; positions `turned_start .. turned_start + turned_len` were
/// turned around during the shuffle and deal reversed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeckShuffle {
    pub shuffle_ref: String,
    pub protocol_ref: String,
    pub entropy: OracleEntropyReceipt,
    pub order: Vec<u16>,
    pub turned_start: u16,
    pub turned_len: u16,
    pub shuffled_at_unix_ms: u64,
}

impl DeckShuffle {
    fn shuffle(
        deck: &DeckDefinition,
        shuffle_ref: String,
        entropy: OracleEntropyReceipt,
        entropy_bytes: &[u8],
        shuffled_at_unix_ms: u64,
    ) -> Result<Self, String> {
        let count = deck.card_count();
        let mut cursor = EntropyCursor::new(entropy_bytes);
        let mut order = (0..count).collect::<Vec<_>>();
        for index in (1..order.len()).rev() {
            let selected = cursor.unbiased_index((index + 1) as u16)?;
            order.swap(index, selected);
        }
        let turned_start = cursor.unbiased_index(count)? as u16;
        let turned_len = cursor.unbiased_index(count - turned_start)? as u16 + 1;
        Ok(Self {
            shuffle_ref,
            protocol_ref: QUATERNAL_TAROT_PROTOCOL_REF.into(),
            entropy,
            order,
            turned_start,
            turned_len,
            shuffled_at_unix_ms,
        })
    }

    fn validate(&self, deck: &DeckDefinition) -> Result<(), String> {
        check_text(&self.shuffle_ref, "deck shuffle")?;
        check_text(&self.protocol_ref, "deck shuffle protocol")?;
        self.entropy.validate()?;
        let count = deck.card_count();
        if self.order.len() != usize::from(count) {
            return Err("deck shuffle order does not cover the deck".into());
        }
        let unique = self.order.iter().copied().collect::<BTreeSet<_>>();
        if unique.len() != usize::from(count) || unique.iter().any(|card| *card >= count) {
            return Err("deck shuffle order is not a permutation of the deck".into());
        }
        if self.turned_len == 0 || self.turned_start + self.turned_len > count {
            return Err("turned portion lies outside the deck".into());
        }
        Ok(())
    }

    fn reversed_at(&self, position: u16) -> bool {
        position >= self.turned_start && position < self.turned_start + self.turned_len
    }
}

// ---------------------------------------------------------------------------
// Spreads, placements and readings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arc {
    /// Day arc, torus, orientable, questioning outward (`?`).
    Day,
    /// Night arc, Klein extension, synthesising inward (`.`).
    Night,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpreadPosition {
    pub arc: Arc,
    pub index: u8,
}

impl SpreadPosition {
    pub fn label(self) -> String {
        match self.arc {
            Arc::Day => format!("P{}", self.index),
            Arc::Night => format!("P{}'", self.index),
        }
    }

    fn validate(self) -> Result<(), String> {
        if self.index > 5 {
            return Err("spread position must be P0..P5 or P0'..P5'".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SpreadKind {
    /// Two cards: ground (P0) and definition (P1).
    Sphere,
    /// Six cards on the Day arc, P0..P5.
    TorusDay,
    /// Six cards on the Night arc, P0'..P5', completing a Day spread.
    KleinNight { day_spread_ref: String },
    /// Six cards nested inside a prior P4/P4' placement.
    Lemniscate { within_placement_ref: String },
    /// An ad hoc draw of 1..=6 cards without protocol positions.
    Single { count: u8 },
    /// The deck's closing movement: every remaining card.
    Closing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spread {
    pub spread_ref: String,
    pub kind: SpreadKind,
    pub day_ref: String,
    pub basis_sources: Vec<SourceRevision>,
    pub actor_ref: String,
    pub drawn_at_unix_ms: u64,
    pub placement_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "basis", rename_all = "kebab-case", deny_unknown_fields)]
pub enum PlacementBasis {
    /// Dealt from the journey deck at this deck position.
    Drawn {
        spread_ref: String,
        deck_position: u16,
    },
    /// Assigned interpretively to an event or relation; the deck is untouched.
    SymbolicAssignment { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LiveState {
    Generating,
    Muting,
    Mute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KleinFace {
    Prospective,
    Retrospective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AspectKind {
    Conjunction,
    Sextile,
    Square,
    Trine,
    Opposition,
}

/// Janus `TargetAspect`: canonical mod-10 planets, Sun = 0 … Pluto = 9.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetAspect {
    pub planet_a: u8,
    pub aspect_kind: AspectKind,
    pub planet_b_or_natal: u8,
    pub exact_at_unix_ms: Option<u64>,
    pub source: SourceRevision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecognitionVia {
    /// Found by Janus tracking in the person's Day/Flow text.
    Tracked,
    /// Deliberately related to the placement by an attributed act.
    Related,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recognition {
    pub source: SourceRevision,
    pub entry_selector: Option<String>,
    pub via: RecognitionVia,
    pub matched: String,
    pub actor_ref: String,
    pub occurred_at_unix_ms: u64,
    pub received_at_unix_ms: u64,
}

impl Recognition {
    fn key(&self) -> (String, String, Option<String>) {
        (
            self.source.source_ref.clone(),
            self.source.revision.clone(),
            self.entry_selector.clone(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActorKind {
    Human,
    Agent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReadingKind {
    /// The reading given when the symbol entered the journey.
    Original,
    /// A later development that keeps earlier readings current.
    Development,
    /// Replaces a named earlier reading; that reading stays in history.
    Correction,
    /// A later view of an earlier consultation in the light of what followed.
    Retrospective,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyReadingEntry {
    pub reading_ref: String,
    pub kind: ReadingKind,
    pub actor_kind: ActorKind,
    pub author_ref: String,
    pub text: String,
    pub supersedes: Option<String>,
    pub source_refs: Vec<String>,
    pub occurred_at_unix_ms: u64,
    pub received_at_unix_ms: u64,
}

impl JourneyReadingEntry {
    fn validate(&self) -> Result<(), String> {
        check_text(&self.reading_ref, "journey reading")?;
        check_text(&self.author_ref, "journey reading author")?;
        check_long_text(&self.text, "journey reading text")?;
        check_refs(&self.source_refs, "journey reading source", 64)?;
        match (self.kind, &self.supersedes) {
            (ReadingKind::Correction, None) => {
                Err("a correction must name the reading it supersedes".into())
            }
            (ReadingKind::Correction, Some(reference)) => check_text(reference, "superseded"),
            (_, Some(_)) => Err("only a correction supersedes an earlier reading".into()),
            (_, None) => Ok(()),
        }
    }
}

fn check_reading_history(readings: &[JourneyReadingEntry]) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for reading in readings {
        reading.validate()?;
        if let Some(superseded) = &reading.supersedes
            && !seen.contains(superseded.as_str())
        {
            return Err("a correction supersedes an unknown or later reading".into());
        }
        if !seen.insert(reading.reading_ref.as_str()) {
            return Err("duplicate journey reading reference".into());
        }
    }
    Ok(())
}

fn current_readings(readings: &[JourneyReadingEntry]) -> Vec<&JourneyReadingEntry> {
    let superseded = readings
        .iter()
        .filter_map(|reading| reading.supersedes.as_deref())
        .collect::<BTreeSet<_>>();
    readings
        .iter()
        .filter(|reading| !superseded.contains(reading.reading_ref.as_str()))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PersonDisposition {
    Resolved,
    Reopened,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DispositionRecord {
    pub disposition: PersonDisposition,
    pub reason: String,
    pub actor_ref: String,
    pub at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    pub placement_ref: String,
    pub card: u16,
    pub reversed: bool,
    pub position: Option<SpreadPosition>,
    pub basis: PlacementBasis,
    pub klein_face: KleinFace,
    pub day_ref: String,
    pub actor_ref: String,
    pub placed_at_unix_ms: u64,
    pub live_state: LiveState,
    pub state_changed_at_unix_ms: Option<u64>,
    pub target_aspect: Option<TargetAspect>,
    pub recognitions: Vec<Recognition>,
    pub dispositions: Vec<DispositionRecord>,
    pub readings: Vec<JourneyReadingEntry>,
}

impl Placement {
    fn last_recognition_at(&self) -> Option<u64> {
        self.recognitions
            .iter()
            .map(|recognition| recognition.occurred_at_unix_ms)
            .max()
    }

    fn person_disposition(&self) -> Option<PersonDisposition> {
        self.dispositions.last().map(|record| record.disposition)
    }
}

// ---------------------------------------------------------------------------
// I-Ching change history
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "basis", rename_all = "kebab-case", deny_unknown_fields)]
pub enum IChingBasis {
    /// An entropy-bearing consultation; the original packet is retained whole.
    Cast { record: Box<OracleRecord> },
    /// A source-defined native computation which yields a form; no chance and
    /// no changing lines unless the computation declares them.
    Computed {
        computation_ref: String,
        input: SourceRevision,
        hexagram: u8,
        changing_mask: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IChingReading {
    pub reading_ref: String,
    pub basis: IChingBasis,
    /// Line values bottom to top (6, 7, 8, 9).
    pub lines: [u8; 6],
    pub primary: u8,
    pub changing_mask: u8,
    pub resulting: Option<u8>,
    pub nuclear: u8,
    pub day_ref: String,
    pub actor_ref: String,
    pub recorded_at_unix_ms: u64,
    pub related_placement_refs: Vec<String>,
    pub readings: Vec<JourneyReadingEntry>,
}

fn hexagram_from_lines(lines: &[u8; 6]) -> (u8, u8) {
    let mut bits = 0_u8;
    let mut changing = 0_u8;
    for (index, value) in lines.iter().enumerate() {
        if value & 1 == 1 {
            bits |= 1 << index;
        }
        if matches!(value, 6 | 9) {
            changing |= 1 << index;
        }
    }
    (bits & 0x3F, changing & 0x3F)
}

fn lines_from_hexagram(hexagram: u8, changing_mask: u8) -> [u8; 6] {
    let mut lines = [0_u8; 6];
    for (index, line) in lines.iter_mut().enumerate() {
        let yang = hexagram >> index & 1 == 1;
        let changing = changing_mask >> index & 1 == 1;
        *line = match (yang, changing) {
            (true, false) => 7,
            (true, true) => 9,
            (false, false) => 8,
            (false, true) => 6,
        };
    }
    lines
}

fn lines_from_record(record: &OracleRecord) -> Result<[u8; 6], String> {
    let mut lines = [0_u8; 6];
    if record.original.tokens.len() != 6 {
        return Err("an I-Ching cast retains exactly six lines".into());
    }
    for token in &record.original.tokens {
        let value = token
            .token_ref
            .strip_prefix("iching-line:")
            .and_then(|value| value.parse::<u8>().ok())
            .filter(|value| (6..=9).contains(value))
            .ok_or("I-Ching cast token is not a 6..9 line")?;
        let slot = lines
            .get_mut(usize::from(token.position))
            .ok_or("I-Ching line position is outside 0..5")?;
        *slot = value;
    }
    Ok(lines)
}

impl IChingReading {
    fn from_basis(
        reading_ref: String,
        basis: IChingBasis,
        day_ref: String,
        actor_ref: String,
        recorded_at_unix_ms: u64,
        related_placement_refs: Vec<String>,
    ) -> Result<Self, String> {
        let lines = match &basis {
            IChingBasis::Cast { record } => lines_from_record(record)?,
            IChingBasis::Computed {
                hexagram,
                changing_mask,
                ..
            } => {
                if *hexagram > 63 || *changing_mask > 63 {
                    return Err("computed hexagram and changing mask are six-bit".into());
                }
                lines_from_hexagram(*hexagram, *changing_mask)
            }
        };
        let (primary, changing_mask) = hexagram_from_lines(&lines);
        Ok(Self {
            reading_ref,
            basis,
            lines,
            primary,
            changing_mask,
            resulting: (changing_mask != 0).then_some(primary ^ changing_mask),
            nuclear: nuclear_hexagram(primary),
            day_ref,
            actor_ref,
            recorded_at_unix_ms,
            related_placement_refs,
            readings: Vec::new(),
        })
    }

    fn validate(&self) -> Result<(), String> {
        check_text(&self.reading_ref, "I-Ching reading")?;
        check_text(&self.day_ref, "I-Ching day")?;
        check_text(&self.actor_ref, "I-Ching actor")?;
        match &self.basis {
            IChingBasis::Cast { record } => {
                record.validate()?;
                if lines_from_record(record)? != self.lines {
                    return Err("I-Ching lines differ from the retained original cast".into());
                }
            }
            IChingBasis::Computed {
                computation_ref,
                input,
                hexagram,
                changing_mask,
            } => {
                check_text(computation_ref, "I-Ching computation")?;
                check_source(input)?;
                if lines_from_hexagram(*hexagram, *changing_mask) != self.lines {
                    return Err("I-Ching lines differ from the computed form".into());
                }
            }
        }
        let (primary, changing) = hexagram_from_lines(&self.lines);
        if primary != self.primary
            || changing != self.changing_mask
            || self.resulting != (changing != 0).then_some(primary ^ changing)
            || self.nuclear != nuclear_hexagram(primary)
        {
            return Err("I-Ching derived hexagrams do not follow from the lines".into());
        }
        check_refs(
            &self.related_placement_refs,
            "I-Ching related placement",
            78,
        )?;
        check_reading_history(&self.readings)
    }
}

// ---------------------------------------------------------------------------
// Journey
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JourneyStatus {
    Open,
    /// Every card has been dealt; a new deck is a new journey.
    DeckComplete,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyConcern {
    pub concern_ref: String,
    pub title: String,
    /// The Day/Flow passages the concern was opened from.
    pub basis_sources: Vec<SourceRevision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyOperation {
    pub request_id: String,
    pub request_digest: String,
    pub op: String,
    pub actor_ref: String,
    pub at_unix_ms: u64,
    pub effect_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleJourney {
    pub schema: String,
    pub journey_ref: String,
    pub subject_id: String,
    pub concern: JourneyConcern,
    pub deck: DeckDefinition,
    pub shuffle: DeckShuffle,
    pub dealt: u16,
    pub status: JourneyStatus,
    pub day_refs: Vec<String>,
    pub spreads: Vec<Spread>,
    pub placements: Vec<Placement>,
    pub iching: Vec<IChingReading>,
    pub operations: Vec<JourneyOperation>,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenJourney {
    pub journey_ref: String,
    pub subject_id: String,
    pub concern: JourneyConcern,
    pub deck: DeckDefinition,
    pub day_ref: String,
    pub actor_ref: String,
    pub entropy: OracleEntropyReceipt,
    pub consent: ConsentState,
    pub opened_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewReading {
    pub reading_ref: String,
    pub kind: ReadingKind,
    pub actor_kind: ActorKind,
    /// Whose words these are. Required for a human reading, which is the
    /// person's own writing even when an agent records it; an agent reading
    /// is authored by the acting agent.
    #[serde(default)]
    pub author_ref: Option<String>,
    pub text: String,
    pub supersedes: Option<String>,
    pub source_refs: Vec<String>,
    pub occurred_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DayFlowNote {
    pub source: SourceRevision,
    pub entry_selector: Option<String>,
    pub noted_at_unix_ms: u64,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IChingCastRequest {
    pub coins: bool,
    pub query_ref: ProtectedRef,
    pub entropy: OracleEntropyReceipt,
    pub original_payload_ref: ProtectedRef,
    pub cast_degree: Option<u16>,
    pub vak_ref: Option<String>,
    pub consent: ConsentState,
    pub hygiene: OracleHygiene,
}

/// Every mutation of a journey. Each is a distinct act with its own effect
/// and provenance; none of them implies another.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case", deny_unknown_fields)]
pub enum JourneyRequest {
    Draw {
        spread: SpreadKind,
        day_ref: String,
        basis_sources: Vec<SourceRevision>,
    },
    AssignSymbol {
        card: u16,
        reversed: bool,
        position: Option<SpreadPosition>,
        reason: String,
        day_ref: String,
    },
    RelateEvent {
        placement_ref: String,
        source: SourceRevision,
        entry_selector: Option<String>,
        relation: String,
        occurred_at_unix_ms: u64,
    },
    Read {
        placement_ref: String,
        reading: NewReading,
    },
    SetTargetAspect {
        placement_ref: String,
        aspect: TargetAspect,
    },
    Dispose {
        placement_ref: String,
        disposition: PersonDisposition,
        reason: String,
    },
    TrackRecognitions {
        notes: Vec<DayFlowNote>,
    },
    EvaluateAliveness,
    #[serde(rename = "cast-iching")]
    CastIChing {
        cast: Box<IChingCastRequest>,
        day_ref: String,
        basis_sources: Vec<SourceRevision>,
        related_placement_refs: Vec<String>,
    },
    #[serde(rename = "record-computed-iching")]
    RecordComputedIChing {
        computation_ref: String,
        input: SourceRevision,
        hexagram: u8,
        changing_mask: u8,
        day_ref: String,
        related_placement_refs: Vec<String>,
    },
    #[serde(rename = "read-iching")]
    ReadIChing {
        reading_ref: String,
        reading: NewReading,
    },
    Close {
        reason: String,
    },
}

impl JourneyRequest {
    fn name(&self) -> &'static str {
        match self {
            Self::Draw { .. } => "draw",
            Self::AssignSymbol { .. } => "assign-symbol",
            Self::RelateEvent { .. } => "relate-event",
            Self::Read { .. } => "read",
            Self::SetTargetAspect { .. } => "set-target-aspect",
            Self::Dispose { .. } => "dispose",
            Self::TrackRecognitions { .. } => "track-recognitions",
            Self::EvaluateAliveness => "evaluate-aliveness",
            Self::CastIChing { .. } => "cast-iching",
            Self::RecordComputedIChing { .. } => "record-computed-iching",
            Self::ReadIChing { .. } => "read-iching",
            Self::Close { .. } => "close",
        }
    }
}

/// One attributed act on the journey.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyAct {
    pub request_id: String,
    pub actor_ref: String,
    pub at_unix_ms: u64,
    /// The civil Day the act happens on. Every act on a later Day joins that
    /// Day to the journey, not only draws and casts.
    #[serde(default)]
    pub day_ref: Option<String>,
    pub request: JourneyRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyEffect {
    pub request_id: String,
    pub op: String,
    pub replayed: bool,
    pub effect_refs: Vec<String>,
    pub journey_revision: u64,
}

/// The replay key is the attributed act alone. Entropy is deliberately
/// excluded: a live cast replayed with fresh entropy reconciles to its first
/// effect rather than casting again, and raw entropy is never retained.
fn request_digest(act: &JourneyAct) -> Result<String, String> {
    let bytes = serde_json::to_vec(act).map_err(|error| error.to_string())?;
    Ok(digest_hex(&bytes))
}

fn position_label_ref(journey_ref: &str, kind: &str, ordinal: usize) -> String {
    format!("{journey_ref}/{kind}/{ordinal}")
}

impl OracleJourney {
    /// Open a journey: select the deck and shuffle it once.
    pub fn open(input: OpenJourney, entropy_bytes: &[u8]) -> Result<Self, String> {
        check_text(&input.journey_ref, "journey")?;
        check_text(&input.subject_id, "journey subject")?;
        check_text(&input.day_ref, "journey day")?;
        check_text(&input.actor_ref, "journey actor")?;
        input.deck.validate()?;
        input.entropy.validate()?;
        if input.consent != ConsentState::Granted {
            return Err("opening a journey deck requires explicit granted consent".into());
        }
        let open_digest =
            digest_hex(&serde_json::to_vec(&input).map_err(|error| error.to_string())?);
        let shuffle = DeckShuffle::shuffle(
            &input.deck,
            format!("{}/shuffle", input.journey_ref),
            input.entropy,
            entropy_bytes,
            input.opened_at_unix_ms,
        )?;
        let journey = Self {
            schema: ORACLE_JOURNEY_SCHEMA.into(),
            journey_ref: input.journey_ref,
            subject_id: input.subject_id,
            concern: input.concern,
            deck: input.deck,
            shuffle,
            dealt: 0,
            status: JourneyStatus::Open,
            day_refs: vec![input.day_ref],
            spreads: Vec::new(),
            placements: Vec::new(),
            iching: Vec::new(),
            operations: vec![JourneyOperation {
                request_id: "open".into(),
                request_digest: open_digest,
                op: "open".into(),
                actor_ref: input.actor_ref,
                at_unix_ms: input.opened_at_unix_ms,
                effect_refs: Vec::new(),
            }],
            revision: 1,
        };
        journey.validate()?;
        Ok(journey)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != ORACLE_JOURNEY_SCHEMA {
            return Err("unsupported oracle journey schema".into());
        }
        check_text(&self.journey_ref, "journey")?;
        check_text(&self.subject_id, "journey subject")?;
        check_text(&self.concern.concern_ref, "journey concern")?;
        check_long_text(&self.concern.title, "journey concern title")?;
        for source in &self.concern.basis_sources {
            check_source(source)?;
        }
        self.deck.validate()?;
        self.shuffle.validate(&self.deck)?;
        if self.dealt > self.deck.card_count() {
            return Err("journey dealt more cards than the deck holds".into());
        }
        check_refs(&self.day_refs, "journey day", 4096)?;
        let mut placement_refs = BTreeSet::new();
        let mut dealt_positions = BTreeSet::new();
        for placement in &self.placements {
            check_text(&placement.placement_ref, "placement")?;
            if !placement_refs.insert(placement.placement_ref.as_str()) {
                return Err("duplicate placement reference".into());
            }
            card_identity(&self.deck, placement.card)?;
            if let Some(position) = placement.position {
                position.validate()?;
            }
            if let PlacementBasis::Drawn { deck_position, .. } = &placement.basis {
                if *deck_position >= self.dealt
                    || self.shuffle.order[usize::from(*deck_position)] != placement.card
                    || self.shuffle.reversed_at(*deck_position) != placement.reversed
                {
                    return Err("drawn placement does not match the journey's shuffle".into());
                }
                if !dealt_positions.insert(*deck_position) {
                    return Err("a deck position was dealt twice".into());
                }
            }
            check_reading_history(&placement.readings)?;
        }
        if dealt_positions.len() != usize::from(self.dealt) {
            return Err("dealt cards and drawn placements disagree".into());
        }
        let mut spread_refs = BTreeSet::new();
        for spread in &self.spreads {
            if !spread_refs.insert(spread.spread_ref.as_str()) {
                return Err("duplicate spread reference".into());
            }
            if spread
                .placement_refs
                .iter()
                .any(|reference| !placement_refs.contains(reference.as_str()))
            {
                return Err("spread names an unknown placement".into());
            }
        }
        let mut iching_refs = BTreeSet::new();
        for reading in &self.iching {
            reading.validate()?;
            if !iching_refs.insert(reading.reading_ref.as_str()) {
                return Err("duplicate I-Ching reading reference".into());
            }
        }
        let mut request_ids = BTreeSet::new();
        for operation in &self.operations {
            if !request_ids.insert(operation.request_id.as_str()) {
                return Err("duplicate journey request id".into());
            }
        }
        Ok(())
    }

    pub fn remaining(&self) -> u16 {
        self.deck.card_count() - self.dealt
    }

    fn placement_mut(&mut self, placement_ref: &str) -> Result<&mut Placement, String> {
        self.placements
            .iter_mut()
            .find(|placement| placement.placement_ref == placement_ref)
            .ok_or_else(|| format!("unknown placement {placement_ref}"))
    }

    fn note_day(&mut self, day_ref: &str) -> Result<(), String> {
        check_text(day_ref, "journey day")?;
        if !self.day_refs.iter().any(|existing| existing == day_ref) {
            self.day_refs.push(day_ref.to_string());
        }
        Ok(())
    }

    fn require_open(&self) -> Result<(), String> {
        match self.status {
            JourneyStatus::Closed => Err("the journey is closed".into()),
            _ => Ok(()),
        }
    }

    /// Apply one attributed act. A request id already applied with the same
    /// request reconciles to its recorded effect; with a different request it
    /// is refused.
    pub fn apply(
        &mut self,
        act: JourneyAct,
        entropy_bytes: &[u8],
    ) -> Result<JourneyEffect, String> {
        check_text(&act.request_id, "journey request id")?;
        check_text(&act.actor_ref, "journey actor")?;
        let digest = request_digest(&act)?;
        if let Some(existing) = self
            .operations
            .iter()
            .find(|operation| operation.request_id == act.request_id)
        {
            if existing.request_digest != digest {
                return Err(format!(
                    "request {} was already applied with a different request",
                    act.request_id
                ));
            }
            return Ok(JourneyEffect {
                request_id: existing.request_id.clone(),
                op: existing.op.clone(),
                replayed: true,
                effect_refs: existing.effect_refs.clone(),
                journey_revision: self.revision,
            });
        }
        let mut next = self.clone();
        if let Some(day_ref) = &act.day_ref {
            next.note_day(day_ref)?;
        }
        let op = act.request.name();
        let effect_refs = next.perform(&act, entropy_bytes)?;
        next.operations.push(JourneyOperation {
            request_id: act.request_id.clone(),
            request_digest: digest,
            op: op.into(),
            actor_ref: act.actor_ref.clone(),
            at_unix_ms: act.at_unix_ms,
            effect_refs: effect_refs.clone(),
        });
        next.revision += 1;
        next.validate()?;
        *self = next;
        Ok(JourneyEffect {
            request_id: act.request_id,
            op: op.into(),
            replayed: false,
            effect_refs,
            journey_revision: self.revision,
        })
    }

    fn perform(&mut self, act: &JourneyAct, entropy_bytes: &[u8]) -> Result<Vec<String>, String> {
        let actor = act.actor_ref.clone();
        let at = act.at_unix_ms;
        match &act.request {
            JourneyRequest::Draw {
                spread,
                day_ref,
                basis_sources,
            } => {
                self.require_open()?;
                self.note_day(day_ref)?;
                for source in basis_sources {
                    check_source(source)?;
                }
                self.draw(spread, day_ref, basis_sources, &actor, at)
            }
            JourneyRequest::AssignSymbol {
                card,
                reversed,
                position,
                reason,
                day_ref,
            } => {
                self.require_open()?;
                self.note_day(day_ref)?;
                card_identity(&self.deck, *card)?;
                check_long_text(reason, "symbolic assignment reason")?;
                let placement_ref =
                    position_label_ref(&self.journey_ref, "placement", self.placements.len());
                self.placements.push(Placement {
                    placement_ref: placement_ref.clone(),
                    card: *card,
                    reversed: *reversed,
                    position: *position,
                    basis: PlacementBasis::SymbolicAssignment {
                        reason: reason.clone(),
                    },
                    klein_face: klein_face_of(*position),
                    day_ref: day_ref.clone(),
                    actor_ref: actor,
                    placed_at_unix_ms: at,
                    live_state: LiveState::Generating,
                    state_changed_at_unix_ms: None,
                    target_aspect: None,
                    recognitions: Vec::new(),
                    dispositions: Vec::new(),
                    readings: Vec::new(),
                });
                Ok(vec![placement_ref])
            }
            JourneyRequest::RelateEvent {
                placement_ref,
                source,
                entry_selector,
                relation,
                occurred_at_unix_ms,
            } => {
                check_source(source)?;
                check_text(relation, "event relation")?;
                if let Some(selector) = entry_selector {
                    check_text(selector, "event entry selector")?;
                }
                let placement = self.placement_mut(placement_ref)?;
                let recognition = Recognition {
                    source: source.clone(),
                    entry_selector: entry_selector.clone(),
                    via: RecognitionVia::Related,
                    matched: relation.clone(),
                    actor_ref: actor,
                    occurred_at_unix_ms: *occurred_at_unix_ms,
                    received_at_unix_ms: at,
                };
                if placement
                    .recognitions
                    .iter()
                    .any(|existing| existing.key() == recognition.key())
                {
                    return Err("that source passage is already related to this placement".into());
                }
                placement.recognitions.push(recognition);
                Ok(vec![placement_ref.clone()])
            }
            JourneyRequest::Read {
                placement_ref,
                reading,
            } => {
                let entry = new_entry(reading, &actor, at)?;
                let reading_ref = entry.reading_ref.clone();
                let placement = self.placement_mut(placement_ref)?;
                placement.readings.push(entry);
                check_reading_history(&placement.readings)?;
                Ok(vec![placement_ref.clone(), reading_ref])
            }
            JourneyRequest::SetTargetAspect {
                placement_ref,
                aspect,
            } => {
                if aspect.planet_a > 9 || aspect.planet_b_or_natal > 9 {
                    return Err("target aspect planets use the mod-10 Sun..Pluto order".into());
                }
                check_source(&aspect.source)?;
                self.placement_mut(placement_ref)?.target_aspect = Some(aspect.clone());
                Ok(vec![placement_ref.clone()])
            }
            JourneyRequest::Dispose {
                placement_ref,
                disposition,
                reason,
            } => {
                check_long_text(reason, "disposition reason")?;
                self.placement_mut(placement_ref)?
                    .dispositions
                    .push(DispositionRecord {
                        disposition: *disposition,
                        reason: reason.clone(),
                        actor_ref: actor,
                        at_unix_ms: at,
                    });
                Ok(vec![placement_ref.clone()])
            }
            JourneyRequest::TrackRecognitions { notes } => {
                let deck = self.deck.clone();
                let mut changed = Vec::new();
                for placement in &mut self.placements {
                    let identity = card_identity(&deck, placement.card)?;
                    for note in notes {
                        check_source(&note.source)?;
                        if note.noted_at_unix_ms < placement.placed_at_unix_ms {
                            continue;
                        }
                        let Some(matched) = note_recognises(&note.body, &identity, placement)
                        else {
                            continue;
                        };
                        let recognition = Recognition {
                            source: note.source.clone(),
                            entry_selector: note.entry_selector.clone(),
                            via: RecognitionVia::Tracked,
                            matched,
                            actor_ref: actor.clone(),
                            occurred_at_unix_ms: note.noted_at_unix_ms,
                            received_at_unix_ms: at,
                        };
                        if placement
                            .recognitions
                            .iter()
                            .any(|existing| existing.key() == recognition.key())
                        {
                            continue;
                        }
                        placement.recognitions.push(recognition);
                        if changed.last() != Some(&placement.placement_ref) {
                            changed.push(placement.placement_ref.clone());
                        }
                    }
                }
                Ok(changed)
            }
            JourneyRequest::EvaluateAliveness => {
                let mut changed = Vec::new();
                for placement in &mut self.placements {
                    if evaluate_aliveness(placement, at) {
                        changed.push(placement.placement_ref.clone());
                    }
                }
                Ok(changed)
            }
            JourneyRequest::CastIChing {
                cast,
                day_ref,
                basis_sources,
                related_placement_refs,
            } => {
                self.require_open()?;
                self.note_day(day_ref)?;
                self.check_placements(related_placement_refs)?;
                let reading_ref =
                    position_label_ref(&self.journey_ref, "iching", self.iching.len());
                let packet = OracleOriginalPacket::cast_iching(
                    OracleCastContext {
                        system: if cast.coins {
                            OracleSystem::IChingCoins
                        } else {
                            OracleSystem::IChingYarrow
                        },
                        packet_ref: format!("{reading_ref}/cast"),
                        subject_id: self.subject_id.clone(),
                        event_ref: self.concern.concern_ref.clone(),
                        profile_generation: 0,
                        tradition_ref: "tradition:i-ching".into(),
                        deck_or_method_ref: if cast.coins {
                            "method:three-coins".into()
                        } else {
                            "method:yarrow".into()
                        },
                        spread_ref: None,
                        query_ref: cast.query_ref.clone(),
                        entropy: cast.entropy.clone(),
                        cast_degree: cast.cast_degree,
                        cast_at_unix_ms: at,
                        vak_ref: cast.vak_ref.clone(),
                        original_payload_ref: cast.original_payload_ref.clone(),
                        source_revisions: if basis_sources.is_empty() {
                            self.concern.basis_sources.clone()
                        } else {
                            basis_sources.clone()
                        },
                        consent: cast.consent.clone(),
                        hygiene: cast.hygiene,
                    },
                    entropy_bytes,
                )?;
                let reading = IChingReading::from_basis(
                    reading_ref.clone(),
                    IChingBasis::Cast {
                        record: Box::new(OracleRecord {
                            original: packet,
                            interpretations: Vec::new(),
                        }),
                    },
                    day_ref.clone(),
                    actor,
                    at,
                    related_placement_refs.clone(),
                )?;
                self.iching.push(reading);
                Ok(vec![reading_ref])
            }
            JourneyRequest::RecordComputedIChing {
                computation_ref,
                input,
                hexagram,
                changing_mask,
                day_ref,
                related_placement_refs,
            } => {
                self.require_open()?;
                self.note_day(day_ref)?;
                self.check_placements(related_placement_refs)?;
                let reading_ref =
                    position_label_ref(&self.journey_ref, "iching", self.iching.len());
                let reading = IChingReading::from_basis(
                    reading_ref.clone(),
                    IChingBasis::Computed {
                        computation_ref: computation_ref.clone(),
                        input: input.clone(),
                        hexagram: *hexagram,
                        changing_mask: *changing_mask,
                    },
                    day_ref.clone(),
                    actor,
                    at,
                    related_placement_refs.clone(),
                )?;
                self.iching.push(reading);
                Ok(vec![reading_ref])
            }
            JourneyRequest::ReadIChing {
                reading_ref,
                reading,
            } => {
                let entry = new_entry(reading, &actor, at)?;
                let entry_ref = entry.reading_ref.clone();
                let target = self
                    .iching
                    .iter_mut()
                    .find(|existing| existing.reading_ref == *reading_ref)
                    .ok_or_else(|| format!("unknown I-Ching reading {reading_ref}"))?;
                target.readings.push(entry);
                check_reading_history(&target.readings)?;
                Ok(vec![reading_ref.clone(), entry_ref])
            }
            JourneyRequest::Close { reason } => {
                check_long_text(reason, "journey close reason")?;
                self.status = JourneyStatus::Closed;
                Ok(vec![self.journey_ref.clone()])
            }
        }
    }

    fn check_placements(&self, references: &[String]) -> Result<(), String> {
        check_refs(references, "related placement", 78)?;
        for reference in references {
            if !self
                .placements
                .iter()
                .any(|placement| placement.placement_ref == *reference)
            {
                return Err(format!("unknown placement {reference}"));
            }
        }
        Ok(())
    }

    fn draw(
        &mut self,
        spread: &SpreadKind,
        day_ref: &str,
        basis_sources: &[SourceRevision],
        actor: &str,
        at: u64,
    ) -> Result<Vec<String>, String> {
        let positions: Vec<Option<SpreadPosition>> = match spread {
            SpreadKind::Sphere => day_positions(2),
            SpreadKind::TorusDay => day_positions(6),
            SpreadKind::KleinNight { day_spread_ref } => {
                let day = self
                    .spreads
                    .iter()
                    .find(|existing| existing.spread_ref == *day_spread_ref)
                    .ok_or("a Night arc completes an existing Day spread")?;
                if day.kind != SpreadKind::TorusDay {
                    return Err("a Night arc completes a Torus Day spread".into());
                }
                if self.spreads.iter().any(|existing| {
                    matches!(&existing.kind, SpreadKind::KleinNight { day_spread_ref: other } if other == day_spread_ref)
                }) {
                    return Err("that Day spread already has its Night arc".into());
                }
                (0..6)
                    .map(|index| {
                        Some(SpreadPosition {
                            arc: Arc::Night,
                            index,
                        })
                    })
                    .collect()
            }
            SpreadKind::Lemniscate {
                within_placement_ref,
            } => {
                let outer = self
                    .placements
                    .iter()
                    .find(|placement| placement.placement_ref == *within_placement_ref)
                    .ok_or("a lemniscate unpacks an existing placement")?;
                let position = outer
                    .position
                    .filter(|position| position.index == 4)
                    .ok_or("the lemniscate unpacks a P4 or P4' placement (protocol §VII)")?;
                (0..6)
                    .map(|index| {
                        Some(SpreadPosition {
                            arc: position.arc,
                            index,
                        })
                    })
                    .collect()
            }
            SpreadKind::Single { count } => {
                if !(1..=6).contains(count) {
                    return Err("an ad hoc draw takes 1..=6 cards".into());
                }
                vec![None; usize::from(*count)]
            }
            SpreadKind::Closing => vec![None; usize::from(self.remaining())],
        };
        if positions.is_empty() || positions.len() > usize::from(self.remaining()) {
            return Err(format!(
                "the deck holds {} undealt cards; this spread needs {}",
                self.remaining(),
                positions.len()
            ));
        }
        let spread_ref = position_label_ref(&self.journey_ref, "spread", self.spreads.len());
        let mut placement_refs = Vec::with_capacity(positions.len());
        for position in positions {
            let deck_position = self.dealt;
            let placement_ref =
                position_label_ref(&self.journey_ref, "placement", self.placements.len());
            self.placements.push(Placement {
                placement_ref: placement_ref.clone(),
                card: self.shuffle.order[usize::from(deck_position)],
                reversed: self.shuffle.reversed_at(deck_position),
                position,
                basis: PlacementBasis::Drawn {
                    spread_ref: spread_ref.clone(),
                    deck_position,
                },
                klein_face: klein_face_of(position),
                day_ref: day_ref.to_string(),
                actor_ref: actor.to_string(),
                placed_at_unix_ms: at,
                live_state: LiveState::Generating,
                state_changed_at_unix_ms: None,
                target_aspect: None,
                recognitions: Vec::new(),
                dispositions: Vec::new(),
                readings: Vec::new(),
            });
            self.dealt += 1;
            placement_refs.push(placement_ref);
        }
        self.spreads.push(Spread {
            spread_ref: spread_ref.clone(),
            kind: spread.clone(),
            day_ref: day_ref.to_string(),
            basis_sources: basis_sources.to_vec(),
            actor_ref: actor.to_string(),
            drawn_at_unix_ms: at,
            placement_refs: placement_refs.clone(),
        });
        if self.remaining() == 0 {
            self.status = JourneyStatus::DeckComplete;
        }
        let mut effects = vec![spread_ref];
        effects.extend(placement_refs);
        Ok(effects)
    }

    /// Janus `janus_spread_resolved`: every position of the spread is mute.
    pub fn spread_resolved(&self, spread_ref: &str) -> bool {
        let Some(spread) = self
            .spreads
            .iter()
            .find(|spread| spread.spread_ref == spread_ref)
        else {
            return false;
        };
        !spread.placement_refs.is_empty()
            && spread.placement_refs.iter().all(|reference| {
                self.placements
                    .iter()
                    .find(|placement| placement.placement_ref == *reference)
                    .is_some_and(|placement| placement.live_state == LiveState::Mute)
            })
    }
}

fn day_positions(count: u8) -> Vec<Option<SpreadPosition>> {
    (0..count)
        .map(|index| {
            Some(SpreadPosition {
                arc: Arc::Day,
                index,
            })
        })
        .collect()
}

fn klein_face_of(position: Option<SpreadPosition>) -> KleinFace {
    match position.map(|position| position.arc) {
        Some(Arc::Night) => KleinFace::Retrospective,
        _ => KleinFace::Prospective,
    }
}

fn new_entry(reading: &NewReading, actor: &str, at: u64) -> Result<JourneyReadingEntry, String> {
    let author_ref = match (reading.actor_kind, &reading.author_ref) {
        (ActorKind::Human, Some(author)) if author != actor => {
            if reading.source_refs.is_empty() {
                return Err(
                    "a human reading cites the person's own source passage it records".into(),
                );
            }
            author.clone()
        }
        (ActorKind::Human, _) => {
            return Err(
                "a human reading names the person as author_ref; the recording agent is not its author"
                    .into(),
            );
        }
        (ActorKind::Agent, Some(author)) if author != actor => {
            return Err("an agent reading is authored by the acting agent".into());
        }
        (ActorKind::Agent, _) => actor.to_string(),
    };
    let entry = JourneyReadingEntry {
        reading_ref: reading.reading_ref.clone(),
        kind: reading.kind,
        actor_kind: reading.actor_kind,
        author_ref,
        text: reading.text.clone(),
        supersedes: reading.supersedes.clone(),
        source_refs: reading.source_refs.clone(),
        occurred_at_unix_ms: reading.occurred_at_unix_ms,
        received_at_unix_ms: at,
    };
    entry.validate()?;
    Ok(entry)
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

/// Janus `noteRecognisesPosition`, over the card's name in the deck's own
/// register or an explicit `live-spread` note naming the position.
fn note_recognises(body: &str, identity: &CardIdentity, placement: &Placement) -> Option<String> {
    let text = body.to_lowercase();
    if contains_term(&text, &identity.name) {
        return Some(identity.name.clone());
    }
    if let Some(position) = placement.position
        && text.contains("live-spread")
        && contains_term(&text, &position.label().to_lowercase())
    {
        return Some(format!("live-spread {}", position.label()));
    }
    None
}

/// Janus `janus_evaluate_aliveness` for one position. Returns whether the
/// state changed.
fn evaluate_aliveness(placement: &mut Placement, now: u64) -> bool {
    let aspect_at = placement
        .target_aspect
        .as_ref()
        .and_then(|aspect| aspect.exact_at_unix_ms);
    let within = |window: u64| aspect_at.is_some_and(|exact| exact.abs_diff(now) <= window);
    let next = match placement.live_state {
        LiveState::Mute if within(24 * HOUR_MS) => Some(LiveState::Generating),
        LiveState::Muting => {
            let changed_at = placement
                .state_changed_at_unix_ms
                .unwrap_or(placement.placed_at_unix_ms);
            let recognised_after = placement
                .last_recognition_at()
                .is_some_and(|recognised| recognised > changed_at);
            if !recognised_after && now.saturating_sub(changed_at) >= 7 * DAY_MS {
                Some(LiveState::Mute)
            } else if recognised_after {
                Some(LiveState::Generating)
            } else {
                None
            }
        }
        LiveState::Generating
            if placement.recognitions.is_empty()
                && now.saturating_sub(placement.placed_at_unix_ms) > 14 * DAY_MS
                && !within(7 * DAY_MS) =>
        {
            Some(LiveState::Muting)
        }
        _ => None,
    };
    match next {
        Some(state) => {
            placement.live_state = state;
            placement.state_changed_at_unix_ms = Some(now);
            true
        }
        None => false,
    }
}

// ---------------------------------------------------------------------------
// Reading projection: what the person and Nara see. The undealt order of the
// deck is never disclosed.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlacementView {
    pub placement_ref: String,
    pub card: CardIdentity,
    pub reversed: bool,
    pub position: Option<String>,
    pub basis: PlacementBasis,
    pub klein_face: KleinFace,
    pub live_state: LiveState,
    pub person_disposition: Option<PersonDisposition>,
    pub day_ref: String,
    pub placed_at_unix_ms: u64,
    pub recognitions: Vec<Recognition>,
    pub current_readings: Vec<JourneyReadingEntry>,
    pub reading_history: Vec<JourneyReadingEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadView {
    pub spread_ref: String,
    pub kind: SpreadKind,
    pub day_ref: String,
    pub basis_sources: Vec<SourceRevision>,
    pub janus_resolved: bool,
    pub placements: Vec<PlacementView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeckView {
    pub deck_ref: String,
    pub register: TarotRegister,
    pub card_count: u16,
    pub dealt: u16,
    pub remaining: u16,
    pub shuffle_ref: String,
    pub shuffled_at_unix_ms: u64,
    pub protocol_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JourneyReadingView {
    pub schema: String,
    pub journey_ref: String,
    pub journey_revision: u64,
    pub subject_id: String,
    pub concern: JourneyConcern,
    pub status: JourneyStatus,
    pub deck: DeckView,
    pub day_refs: Vec<String>,
    pub spreads: Vec<SpreadView>,
    pub symbolic_assignments: Vec<PlacementView>,
    pub iching: Vec<IChingReading>,
    pub live_placement_refs: Vec<String>,
    pub aliveness_source_ref: String,
}

impl OracleJourney {
    fn placement_view(&self, placement: &Placement) -> Result<PlacementView, String> {
        Ok(PlacementView {
            placement_ref: placement.placement_ref.clone(),
            card: card_identity(&self.deck, placement.card)?,
            reversed: placement.reversed,
            position: placement.position.map(SpreadPosition::label),
            basis: placement.basis.clone(),
            klein_face: placement.klein_face,
            live_state: placement.live_state,
            person_disposition: placement.person_disposition(),
            day_ref: placement.day_ref.clone(),
            placed_at_unix_ms: placement.placed_at_unix_ms,
            recognitions: placement.recognitions.clone(),
            current_readings: current_readings(&placement.readings)
                .into_iter()
                .cloned()
                .collect(),
            reading_history: placement.readings.clone(),
        })
    }

    /// The journey as it can be read now. With a window, only the spreads,
    /// assignments and I-Ching readings made within it (a period review).
    pub fn reading(&self, window: Option<(u64, u64)>) -> Result<JourneyReadingView, String> {
        let inside = |at: u64| window.is_none_or(|(from, to)| at >= from && at <= to);
        let mut spreads = Vec::new();
        for spread in self
            .spreads
            .iter()
            .filter(|spread| inside(spread.drawn_at_unix_ms))
        {
            let mut placements = Vec::new();
            for reference in &spread.placement_refs {
                let placement = self
                    .placements
                    .iter()
                    .find(|placement| placement.placement_ref == *reference)
                    .ok_or("spread names an unknown placement")?;
                placements.push(self.placement_view(placement)?);
            }
            spreads.push(SpreadView {
                spread_ref: spread.spread_ref.clone(),
                kind: spread.kind.clone(),
                day_ref: spread.day_ref.clone(),
                basis_sources: spread.basis_sources.clone(),
                janus_resolved: self.spread_resolved(&spread.spread_ref),
                placements,
            });
        }
        let mut symbolic_assignments = Vec::new();
        for placement in self.placements.iter().filter(|placement| {
            matches!(placement.basis, PlacementBasis::SymbolicAssignment { .. })
                && inside(placement.placed_at_unix_ms)
        }) {
            symbolic_assignments.push(self.placement_view(placement)?);
        }
        Ok(JourneyReadingView {
            schema: ORACLE_JOURNEY_READING_SCHEMA.into(),
            journey_ref: self.journey_ref.clone(),
            journey_revision: self.revision,
            subject_id: self.subject_id.clone(),
            concern: self.concern.clone(),
            status: self.status,
            deck: DeckView {
                deck_ref: self.deck.deck_ref.clone(),
                register: self.deck.register,
                card_count: self.deck.card_count(),
                dealt: self.dealt,
                remaining: self.remaining(),
                shuffle_ref: self.shuffle.shuffle_ref.clone(),
                shuffled_at_unix_ms: self.shuffle.shuffled_at_unix_ms,
                protocol_ref: self.shuffle.protocol_ref.clone(),
            },
            day_refs: self.day_refs.clone(),
            spreads,
            symbolic_assignments,
            iching: self
                .iching
                .iter()
                .filter(|reading| inside(reading.recorded_at_unix_ms))
                .cloned()
                .collect(),
            live_placement_refs: self
                .placements
                .iter()
                .filter(|placement| {
                    placement.live_state != LiveState::Mute
                        && placement.person_disposition() != Some(PersonDisposition::Resolved)
                })
                .map(|placement| placement.placement_ref.clone())
                .collect(),
            aliveness_source_ref: JANUS_ALIVENESS_SOURCE_REF.into(),
        })
    }
}
