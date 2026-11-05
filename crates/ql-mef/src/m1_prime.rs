//! The M1′ session: the Torus as QL designs it (O-I #850; DLW manual §21.2).
//!
//! The played-session surface of M1′ (seed `M1'/m1-prime-paramasiva-instrument.md`
//! §2.M1-1′, lines 183–244): the rate of the one pulse and its band, the
//! Fundamental, the Carrier, the Shape, the tuning the voices sound, the notes
//! that sound (Polyphony) and the comma gathered since the Torus opened. It
//! holds no oscillator and no field: the scene owner (`continuous::scene_field`)
//! keeps the body, and this session is the clock it reads its pulse from.
//!
//! Laws ported from the seed, each with its place:
//! - the five rate bands, and phase continuity across them: seed lines
//!   347–365 (`M1_Prime_Rate_Band`, invariant 4 "Phase continuity");
//! - one Pythagorean comma per complete cycle, monotonic, from nothing at the
//!   session's start: seed lines 500, 524, 696, 736;
//! - the return as the totalisation moment, one step: seed line 366;
//! - render-mode auto-switch on band-crossing unless set explicitly: seed
//!   lines 472–479, 522.
//!
//! The ring is the spanda ring of twelve places (`spanda_field::RING_SIZE`):
//! the 720° cover read in twelve arcs, six to a sheet (`RING_HALF`, "one 360°
//! cycle of the double cover"). One step of the pulse is one arc; twelve steps
//! are the full return ("A full return takes two turns"). The comma is derived
//! from QL's own whole tone: six epogdoons (9:8, `m1::ratio_basis`) round one
//! whole-tone family overshoot the octave by 531441:524288.
//!
//! Samples, seconds and steps stay apart: samples become seconds by the sample
//! rate, seconds become steps by the rate (Free) or by the set's tempo (Sync:
//! one step per note length, a whole note being four beats, as Live's synced
//! rates count bars). Those are the only crossings.
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::janko::JankoKey;

pub const SESSION: &str = "ql.m1-prime-session/v1";
pub const SEED_REF: &str =
    "Epi-Logos-C-Experiments:Idea/Bimba/Seeds/M/M1'/m1-prime-paramasiva-instrument.md";

/// One step a (Julian) year: the slowest Free rate.
pub const RATE_MIN: f64 = 1.0 / (365.25 * 86_400.0);
/// The fastest Free rate, in steps a second.
pub const RATE_MAX: f64 = 40_000.0;
/// The default Free rate: the spanda carrier anchor (`spanda_field::HkbParams`).
pub const RATE_DEFAULT: f64 = 2.5;
/// The set's tempo, in beats a minute (Live's range).
pub const TEMPO_MIN: f64 = 20.0;
pub const TEMPO_MAX: f64 = 999.0;
pub const FUNDAMENTAL_MIN: f64 = 20.0;
pub const FUNDAMENTAL_MAX: f64 = 2_000.0;
pub const POLYPHONY_MAX: u8 = 32;
pub const POLYPHONY_DEFAULT: u8 = 16;
/// Strike's default share of a key's velocity (manual §21.2 Parameters: "Strike | 0 to 100% | 70%").
pub const STRIKE_DEFAULT: f64 = 0.7;
fn strike_default() -> f64 {
    STRIKE_DEFAULT
}
/// Steps of one full return: the spanda ring of twelve places.
pub const RETURN_STEPS: f64 = crate::spanda_field::RING_SIZE as f64;
/// The comma: six epogdoons (9:8) over the octave, (9/8)^6 / 2.
pub const PYTHAGOREAN_COMMA: [u64; 2] = [531_441, 524_288];

/// The comma's size in cents, from QL's own whole tone.
pub fn pythagorean_comma_cents() -> f64 {
    let ratio = crate::m1::ratio_basis()
        .ok()
        .and_then(|basis| basis.get(6).map(|r| r.ratio))
        .unwrap_or([9, 8]);
    debug_assert_eq!(ratio, [9, 8], "ratio_basis[6] is the epogdoon");
    let whole_tone = f64::from(ratio[0]) / f64::from(ratio[1]);
    1200.0 * (whole_tone.powi(6) / 2.0).log2()
}

/// How the pulse is heard (manual §21.2 "How the Torus sounds"; seed line 347).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Band {
    Cosmic,
    Breath,
    Rhythm,
    Pitch,
    Beyond,
}

impl Band {
    pub const ALL: [Self; 5] = [
        Self::Cosmic,
        Self::Breath,
        Self::Rhythm,
        Self::Pitch,
        Self::Beyond,
    ];
    /// The band of a rate in steps a second. Each band opens at its lower edge;
    /// Pitch closes at 20,000 and Beyond is above it.
    pub fn of(steps_per_second: f64) -> Self {
        if steps_per_second < 0.001 {
            Self::Cosmic
        } else if steps_per_second < 0.5 {
            Self::Breath
        } else if steps_per_second < 20.0 {
            Self::Rhythm
        } else if steps_per_second <= 20_000.0 {
            Self::Pitch
        } else {
            Self::Beyond
        }
    }
    /// The band's edges in steps a second; the last is open.
    pub fn edges(self) -> (f64, Option<f64>) {
        match self {
            Self::Cosmic => (0.0, Some(0.001)),
            Self::Breath => (0.001, Some(0.5)),
            Self::Rhythm => (0.5, Some(20.0)),
            Self::Pitch => (20.0, Some(20_000.0)),
            Self::Beyond => (20_000.0, None),
        }
    }
}

/// A note length, in whole notes (four beats; "bars" as Live's synced rates
/// count them): 1/64 to 8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoteLength {
    pub numerator: u16,
    pub denominator: u16,
}

impl NoteLength {
    pub fn whole_notes(self) -> f64 {
        f64::from(self.numerator) / f64::from(self.denominator)
    }
    fn validate(self) -> Result<(), String> {
        if self.numerator == 0 || self.denominator == 0 {
            return Err("a Sync note length is a positive fraction of a bar".into());
        }
        let whole = self.whole_notes();
        if !(1.0 / 64.0..=8.0).contains(&whole) {
            return Err(format!(
                "a Sync note length runs from 1/64 to 8 bars; {}/{} is outside it",
                self.numerator, self.denominator
            ));
        }
        Ok(())
    }
}

/// How fast the pulse steps round the ring.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Rate {
    Free {
        #[serde(deserialize_with = "crate::continuous::number::float")]
        steps_per_second: f64,
    },
    /// One step per note length at the set's tempo.
    Sync { note: NoteLength },
}

/// The note place 0 stands on: the map's 1:1 (the Sun), or a frequency.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Fundamental {
    Map,
    Hz {
        #[serde(deserialize_with = "crate::continuous::number::float")]
        hz: f64,
    },
}

/// The Shape parameter: Auto changes by the pulse, Sequence follows the shapes
/// set in the scene, any other value holds that shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Shape {
    Auto,
    Sequence,
    Torus,
    FigureEight,
    Spiral,
    KleinBottle,
}

/// The shape the torus shows (seed render modes, lines 474–479).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Figure {
    Torus,
    FigureEight,
    Spiral,
    KleinBottle,
}

/// The shape Auto shows in a band between returns and crossings.
fn band_figure(band: Band) -> Figure {
    match band {
        Band::Pitch | Band::Beyond => Figure::Torus,
        Band::Rhythm => Figure::FigureEight,
        Band::Breath | Band::Cosmic => Figure::Spiral,
    }
}

/// What the voices sound (manual §21.2): Vimarśā's octet, the condition's
/// maqām, or the dated sky's planets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Tuning {
    Logic,
    Maqam,
    Sky,
}

impl Tuning {
    pub const ALL: [Self; 3] = [Self::Logic, Self::Maqam, Self::Sky];
    pub fn code(self) -> &'static str {
        match self {
            Self::Logic => "logic",
            Self::Maqam => "maqam",
            Self::Sky => "sky",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub rate: Rate,
    pub fundamental: Fundamental,
    /// 0..1: how strongly the pulse drives the body between strikes.
    pub carrier: f64,
    pub shape: Shape,
    pub tuning: Tuning,
    /// 1..=32 notes.
    pub polyphony: u8,
    /// The set's tempo, which Sync reads.
    pub tempo_bpm: f64,
    /// 0..1: how strongly velocity excites the body, the share of a key's amplitude a played note strikes with (manual §21.2 Strike).
    /// Under Body source Tattva the element of the M2 condition sets it (`m2_body::TABLE`); a saved sound without it opens at 70 %.
    #[serde(default = "strike_default")]
    pub strike: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rate: Rate::Free {
                steps_per_second: RATE_DEFAULT,
            },
            fundamental: Fundamental::Map,
            carrier: 0.0,
            shape: Shape::Auto,
            tuning: Tuning::Logic,
            polyphony: POLYPHONY_DEFAULT,
            tempo_bpm: 120.0,
            strike: STRIKE_DEFAULT,
        }
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        match self.rate {
            Rate::Free { steps_per_second } => {
                if !steps_per_second.is_finite()
                    || !(RATE_MIN..=RATE_MAX).contains(&steps_per_second)
                {
                    return Err(format!(
                        "a Free rate runs from one step a year to {RATE_MAX:.0} steps a second"
                    ));
                }
            }
            Rate::Sync { note } => note.validate()?,
        }
        if let Fundamental::Hz { hz } = self.fundamental
            && (!hz.is_finite() || !(FUNDAMENTAL_MIN..=FUNDAMENTAL_MAX).contains(&hz))
        {
            return Err(format!(
                "the Fundamental is Map or {FUNDAMENTAL_MIN:.0} to {FUNDAMENTAL_MAX:.0} Hz"
            ));
        }
        if !self.carrier.is_finite() || !(0.0..=1.0).contains(&self.carrier) {
            return Err("the Carrier runs from 0 to 100 %".into());
        }
        if !self.strike.is_finite() || !(0.0..=1.0).contains(&self.strike) {
            return Err("Strike runs from 0 to 100 %".into());
        }
        if !(1..=POLYPHONY_MAX).contains(&self.polyphony) {
            return Err(format!("Polyphony counts 1 to {POLYPHONY_MAX} notes"));
        }
        if !self.tempo_bpm.is_finite() || !(TEMPO_MIN..=TEMPO_MAX).contains(&self.tempo_bpm) {
            return Err(format!(
                "the set's tempo runs from {TEMPO_MIN:.0} to {TEMPO_MAX:.0} BPM"
            ));
        }
        Ok(())
    }

    pub fn steps_per_second(&self) -> f64 {
        match self.rate {
            Rate::Free { steps_per_second } => steps_per_second,
            // One step per note length: a whole note is four beats of 60/tempo s.
            Rate::Sync { note } => self.tempo_bpm / (240.0 * note.whole_notes()),
        }
    }
}

/// One edit of the session; absent fields are unchanged.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettingsEdit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<Rate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fundamental: Option<Fundamental>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::continuous::number::option"
    )]
    pub carrier: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<Shape>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tuning: Option<Tuning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polyphony: Option<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::continuous::number::option"
    )]
    pub tempo_bpm: Option<f64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::continuous::number::option"
    )]
    pub strike: Option<f64>,
}

impl SettingsEdit {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
    /// The settings this edit makes of `settings`, validated whole.
    pub fn apply(&self, settings: &Settings) -> Result<Settings, String> {
        let next = Settings {
            rate: self.rate.unwrap_or(settings.rate),
            fundamental: self.fundamental.unwrap_or(settings.fundamental),
            carrier: self.carrier.unwrap_or(settings.carrier),
            shape: self.shape.unwrap_or(settings.shape),
            tuning: self.tuning.unwrap_or(settings.tuning),
            polyphony: self.polyphony.unwrap_or(settings.polyphony),
            tempo_bpm: self.tempo_bpm.unwrap_or(settings.tempo_bpm),
            strike: self.strike.unwrap_or(settings.strike),
        };
        next.validate()?;
        Ok(next)
    }
}

/// One sounding note: a key struck at a sample, at the arc its point landed on.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Note {
    pub id: u64,
    pub key: JankoKey,
    /// The ring arc (0..12) the key's moving point struck.
    pub arc: u8,
    pub struck_at_samples: u64,
}

/// What a strike of keys did to the count of notes.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct NoteAdmission {
    pub admitted: Vec<Note>,
    /// The oldest notes that made way, oldest first.
    pub made_way: Vec<Note>,
}

/// The pulse at one sample.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Pulse {
    /// Continuous steps since the Torus opened.
    pub steps: f64,
    /// The arc of the ring of twelve the pulse stands on.
    pub arc: u8,
    /// Its place on the 720° cover, degrees.
    pub cover_degrees: f64,
    /// Full returns since the Torus opened.
    pub returns: u64,
}

/// A sounding note as a checkpoint keeps it: how long before the checkpoint it was struck.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointNote {
    pub id: u64,
    pub key: JankoKey,
    pub arc: u8,
    pub struck_samples_ago: u64,
}

/// The session as it stands at a sample (O-I 855: what a Live take keeps to begin again where
/// it began): the settings, the pulse's continuous steps (so its arc, its place on the cover and
/// the comma gathered), the Klein bottle's steps still to show, the sounding notes, and the next
/// note's number. Times are kept as samples before the checkpoint, so it is set again at any
/// later sample of the scene's clock (`Session::restore`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionCheckpoint {
    pub settings: Settings,
    pub steps: f64,
    pub opened_samples_ago: u64,
    pub klein_steps_left: Option<f64>,
    pub notes: Vec<CheckpointNote>,
    pub next_note: u64,
}

/// The live M1′ session over one scene owner's sample clock.
#[derive(Debug, Clone)]
pub struct Session {
    settings: Settings,
    sample_rate: u32,
    opened_at_samples: u64,
    /// The pulse is `anchor_steps` at `anchor_samples`, then the rate since.
    anchor_samples: u64,
    anchor_steps: f64,
    /// A tritone crossing shows the Klein bottle until this many steps.
    klein_until_steps: Option<f64>,
    notes: VecDeque<Note>,
    next_note: u64,
}

impl Session {
    /// Opens the Torus at `at_samples` of the scene's clock: the pulse at step
    /// zero, no comma, no notes.
    pub fn open(settings: Settings, sample_rate: u32, at_samples: u64) -> Result<Self, String> {
        settings.validate()?;
        if sample_rate == 0 {
            return Err("the M1′ session needs the scene's sample rate".into());
        }
        Ok(Self {
            settings,
            sample_rate,
            opened_at_samples: at_samples,
            anchor_samples: at_samples,
            anchor_steps: 0.0,
            klein_until_steps: None,
            notes: VecDeque::new(),
            next_note: 0,
        })
    }

    /// The session as it stands at `samples` (O-I 855; `SessionCheckpoint`).
    pub fn checkpoint(&self, samples: u64) -> SessionCheckpoint {
        let steps = self.steps_at(samples);
        SessionCheckpoint {
            settings: self.settings,
            steps,
            opened_samples_ago: samples.saturating_sub(self.opened_at_samples),
            klein_steps_left: self
                .klein_until_steps
                .filter(|until| *until > steps)
                .map(|until| until - steps),
            notes: self
                .notes
                .iter()
                .map(|note| CheckpointNote {
                    id: note.id,
                    key: note.key,
                    arc: note.arc,
                    struck_samples_ago: samples.saturating_sub(note.struck_at_samples),
                })
                .collect(),
            next_note: self.next_note,
        }
    }

    /// The session a checkpoint kept, set again at `samples` of the scene's clock: from there
    /// the pulse runs on from the kept steps by the kept settings, and the kept notes sound. A
    /// checkpoint that cannot stand is refused in words.
    pub fn restore(
        checkpoint: &SessionCheckpoint,
        sample_rate: u32,
        samples: u64,
    ) -> Result<Self, String> {
        checkpoint.settings.validate()?;
        if sample_rate == 0 {
            return Err("the M1′ session needs the scene's sample rate".into());
        }
        if !checkpoint.steps.is_finite() || checkpoint.steps < 0.0 {
            return Err("a checkpoint's pulse steps must be finite and not negative".into());
        }
        if checkpoint
            .klein_steps_left
            .is_some_and(|left| !left.is_finite() || left <= 0.0 || left > 1.0)
        {
            return Err("a checkpoint's Klein bottle shows for at most one step".into());
        }
        if checkpoint.notes.len() > usize::from(checkpoint.settings.polyphony) {
            return Err(format!(
                "a checkpoint keeps more notes ({}) than its Polyphony allows ({})",
                checkpoint.notes.len(),
                checkpoint.settings.polyphony
            ));
        }
        let mut ids = std::collections::BTreeSet::new();
        for note in &checkpoint.notes {
            if note.arc >= 12 || note.id >= checkpoint.next_note || !ids.insert(note.id) {
                return Err(
                    "a checkpoint's notes each stand on an arc of twelve with their own number before the next"
                        .into(),
                );
            }
        }
        Ok(Self {
            settings: checkpoint.settings,
            sample_rate,
            opened_at_samples: samples.saturating_sub(checkpoint.opened_samples_ago),
            anchor_samples: samples,
            anchor_steps: checkpoint.steps,
            klein_until_steps: checkpoint
                .klein_steps_left
                .map(|left| checkpoint.steps + left),
            notes: checkpoint
                .notes
                .iter()
                .map(|note| Note {
                    id: note.id,
                    key: note.key,
                    arc: note.arc,
                    struck_at_samples: samples.saturating_sub(note.struck_samples_ago),
                })
                .collect(),
            next_note: checkpoint.next_note,
        })
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }
    pub fn steps_per_second(&self) -> f64 {
        self.settings.steps_per_second()
    }
    pub fn band(&self) -> Band {
        Band::of(self.steps_per_second())
    }

    /// The pulse's continuous steps at a sample of the scene's clock. A sample
    /// before the session's anchor reads the anchor: the pulse never runs back.
    pub fn steps_at(&self, samples: u64) -> f64 {
        let elapsed = samples.saturating_sub(self.anchor_samples) as f64;
        self.anchor_steps + elapsed / f64::from(self.sample_rate) * self.steps_per_second()
    }

    pub fn pulse(&self, samples: u64) -> Pulse {
        let steps = self.steps_at(samples);
        let arc = (steps.rem_euclid(RETURN_STEPS).floor() as u8) % 12;
        Pulse {
            steps,
            arc,
            cover_degrees: (steps * 60.0).rem_euclid(720.0),
            returns: (steps / RETURN_STEPS).floor() as u64,
        }
    }

    /// The comma gathered since the Torus opened, in cents: one Pythagorean
    /// comma for each full return. It grows only as the pulse runs, and the
    /// pulse runs only as the scene's samples do.
    pub fn comma_cents(&self, samples: u64) -> f64 {
        self.pulse(samples).returns as f64 * pythagorean_comma_cents()
    }

    /// The shape the torus shows at a sample; `None` under Sequence, whose
    /// shapes are the scene's own.
    pub fn figure(&self, samples: u64) -> Option<Figure> {
        match self.settings.shape {
            Shape::Sequence => None,
            Shape::Torus => Some(Figure::Torus),
            Shape::FigureEight => Some(Figure::FigureEight),
            Shape::Spiral => Some(Figure::Spiral),
            Shape::KleinBottle => Some(Figure::KleinBottle),
            Shape::Auto => {
                let steps = self.steps_at(samples);
                let returning = steps >= RETURN_STEPS && steps.rem_euclid(RETURN_STEPS) < 1.0;
                let crossing = self.klein_until_steps.is_some_and(|until| steps < until);
                Some(if returning || crossing {
                    Figure::KleinBottle
                } else {
                    band_figure(self.band())
                })
            }
        }
    }

    /// The figure law Auto runs (O-I #851), so that a reader holding one
    /// reading names the shape at any later sample of the same settings
    /// exactly as [`Self::figure`] does: the band's shape, the Klein bottle
    /// for `klein_for_steps` at each return of `return_steps`, and until the
    /// step a tritone crossing named.
    pub fn figure_law(&self) -> Value {
        let mut by_band = serde_json::Map::new();
        for band in Band::ALL {
            if let Value::String(name) = json!(band) {
                by_band.insert(name, json!(band_figure(band)));
            }
        }
        json!({
            "return_steps": RETURN_STEPS,
            "klein_for_steps": 1.0,
            "klein_until_steps": self.klein_until_steps,
            "by_band": by_band,
            "law": "Auto: the band's shape (pitch and beyond the torus, rhythm the figure-eight, breath and cosmic the spiral); the Klein bottle for one step at each return and until klein_until_steps after a tritone crossing (seed render modes, lines 474–479, 506–508)",
        })
    }

    /// The pulse's anchor: `(samples, steps)`, from which [`Self::steps_at`] runs at the rate.
    pub fn anchor(&self) -> (u64, f64) {
        (self.anchor_samples, self.anchor_steps)
    }

    /// How strongly one step of the pulse drives the body, as a share of one event's excitation: Carrier
    /// itself up to the spanda carrier's anchor rate ([`RATE_DEFAULT`]), then shared out so the
    /// drive a second stays the anchor's. At 0 % (or a stopped pulse) it is nothing, and the
    /// body sounds only when struck (manual §21.2 Carrier).
    pub fn carrier_step(&self) -> f64 {
        let steps_per_second = self.steps_per_second();
        if self.settings.carrier == 0.0 || steps_per_second <= 0.0 {
            return 0.0;
        }
        self.settings.carrier * (RATE_DEFAULT / steps_per_second).min(1.0)
    }

    /// Moves the anchor to `samples` so a change of rate keeps the phase.
    fn rebase(&mut self, samples: u64) {
        let samples = samples.max(self.anchor_samples);
        self.anchor_steps = self.steps_at(samples);
        self.anchor_samples = samples;
    }

    /// Applies an edit at a sample of the scene's clock. The pulse keeps its
    /// phase across a change of rate, tempo or band; a refused edit changes
    /// nothing. Returns the names of the settings that moved.
    pub fn edit(&mut self, edit: &SettingsEdit, samples: u64) -> Result<Vec<&'static str>, String> {
        let next = edit.apply(&self.settings)?;
        self.rebase(samples);
        let before = self.settings;
        self.settings = next;
        let mut moved = Vec::new();
        for (name, changed) in [
            ("rate", before.rate != next.rate),
            ("fundamental", before.fundamental != next.fundamental),
            ("carrier", before.carrier != next.carrier),
            ("shape", before.shape != next.shape),
            ("tuning", before.tuning != next.tuning),
            ("polyphony", before.polyphony != next.polyphony),
            ("tempo", before.tempo_bpm != next.tempo_bpm),
            ("strike", before.strike != next.strike),
        ] {
            if changed {
                moved.push(name);
            }
        }
        // Fewer notes allowed: the oldest make way now.
        while self.notes.len() > usize::from(self.settings.polyphony) {
            self.notes.pop_front();
        }
        Ok(moved)
    }

    /// A tritone crossing of the mode (seed lines 506–508): the Klein bottle
    /// for the next step.
    pub fn cross_tritone(&mut self, samples: u64) {
        self.klein_until_steps = Some(self.steps_at(samples) + 1.0);
    }

    /// Admits struck keys as notes, newest last. When more notes sound than
    /// Polyphony allows, the oldest make way. A key already sounding is struck
    /// again as a new note (the old one makes way for it).
    pub fn strike(&mut self, keys: &[JankoKey], samples: u64) -> NoteAdmission {
        let (steps, polyphony) = (self.steps_at(samples), self.settings.polyphony);
        self.admit(keys, samples, steps, polyphony)
    }

    /// The same strike counted now for a later sample, under the session as it will stand there
    /// (`landing`: this session with the settings that wait to be made before that sample): each
    /// key's arc at the pulse there, and the Polyphony in force there (O-I 855).
    pub fn strike_under(
        &mut self,
        keys: &[JankoKey],
        samples: u64,
        landing: &Self,
    ) -> NoteAdmission {
        self.admit(
            keys,
            samples,
            landing.steps_at(samples),
            landing.settings.polyphony,
        )
    }

    fn admit(
        &mut self,
        keys: &[JankoKey],
        samples: u64,
        steps: f64,
        polyphony: u8,
    ) -> NoteAdmission {
        let mut admission = NoteAdmission::default();
        let surface = crate::janko::JankoSurface::new();
        for key in keys {
            if let Some(index) = self.notes.iter().position(|note| note.key == *key) {
                admission.made_way.extend(self.notes.remove(index));
            }
            let note = Note {
                id: self.next_note,
                key: *key,
                arc: surface.point(*key, steps).arc,
                struck_at_samples: samples,
            };
            self.next_note += 1;
            self.notes.push_back(note);
            admission.admitted.push(note);
            while self.notes.len() > usize::from(polyphony) {
                admission.made_way.extend(self.notes.pop_front());
            }
        }
        admission
    }

    /// Releases keys from the count. The body rings on: a released note keeps
    /// its own tail (manual §21.2 Rules).
    pub fn release(&mut self, keys: &[JankoKey]) -> Vec<Note> {
        let mut released = Vec::new();
        self.notes.retain(|note| {
            let gone = keys.contains(&note.key);
            if gone {
                released.push(*note);
            }
            !gone
        });
        released
    }

    pub fn sounding(&self) -> &VecDeque<Note> {
        &self.notes
    }

    /// The id the next note admitted takes.
    pub fn next_note(&self) -> u64 {
        self.next_note
    }

    /// The Fundamental in Hz, given the map's 1:1 (the Sun's voice).
    pub fn fundamental_hz(&self, map_hz: f64) -> f64 {
        match self.settings.fundamental {
            Fundamental::Map => map_hz,
            Fundamental::Hz { hz } => hz,
        }
    }

    /// The session's disclosure at a sample: every setting, the band beside the
    /// rate, the pulse, the shape shown, the comma and the sounding notes.
    pub fn reading(&self, samples: u64, map_hz: f64) -> Value {
        let pulse = self.pulse(samples);
        let band = self.band();
        let (low, high) = band.edges();
        json!({
            "schema": SESSION,
            "settings": self.settings,
            "steps_per_second": self.steps_per_second(),
            "band": band,
            "band_edges": {"from": low, "to": high},
            "fundamental_hz": self.fundamental_hz(map_hz),
            "map_hz": map_hz,
            "pulse": pulse,
            "figure": self.figure(samples),
            "figure_law": self.figure_law(),
            "comma_cents": self.comma_cents(samples),
            "comma": {"per_return": PYTHAGOREAN_COMMA, "cents_per_return": pythagorean_comma_cents(),
                "law": "one Pythagorean comma for each full return of the pulse (twelve steps, the 720° cover), from nothing when the Torus opens; it grows only as the scene's samples run (M1′ seed lines 500, 524, 696)"},
            "opened_at_samples": self.opened_at_samples.to_string(),
            "samples_elapsed": samples.to_string(),
            "notes": self.notes,
            "polyphony": self.settings.polyphony,
            "carrier": {"value": self.settings.carrier,
                "drive": "pulse",
                "step": self.carrier_step(),
                "law": "each step of the pulse gives every voice of the tuning in force this share of the excitation an event gives it; one event's share at most, and above the spanda carrier's anchor rate the drive a second stays the anchor's. At 0 % the body sounds only when something strikes it; at rhythm rates the carrier beats; at pitch rates it sounds (manual §21.2 Carrier)"},
            "crossings": "samples → seconds by the sample rate; seconds → steps by the Free rate, or under Sync one step per note length at the set's tempo (a whole note is four beats)",
            "source_ref": SEED_REF,
        })
    }
}
