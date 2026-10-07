//! The PS-G fold sequence (QL-MEF #299 slice 2; P5 §4.2): interpolation,
//! seek and interruption over fold progress, natively.
//!
//! The existing entity sequence supplies hold, transition, easing, progress
//! and interruption; this owner is the fold law that answers it. A declared
//! sequence walks determined forms — the SAME determinations the form-recipe
//! owner compiles (`ql.psg-form-recipe/v1`), through the SAME kernel cast law
//! — along the stage's display-clock axes. A phase holds a form or transitions
//! to one; the evaluator names, at an exact display-clock cursor, the eased
//! crease-angle path the sampler deforms the retained sample body with.
//!
//! The source's own honesty about forms is kept: a codon resolves at the
//! quanta. On a hold, and at a transition's onset, the standing form is named
//! and its cast telemetry carried; strictly inside a transition the crease
//! path is determined but NO form is named — `resolved_form` is absent, not
//! fabricated.
//!
//! Evaluation is a pure function of the sequence and the cursor: seek is
//! re-reading, never replaying hidden state — evaluating any cursor directly
//! equals reaching it stepwise, backwards or forwards. Progress and entity
//! position are not preparation dependencies: the retained sample body
//! (`ql.psg-form-samples/v1`) stands untouched under every cursor. And an
//! interrupted sequence is recorded — the exact standing it was interrupted
//! in — never silently discarded.
use serde::{Deserialize, Serialize};

use crate::continuous::LiftInput;
use crate::continuous::stage::StageChange;
use crate::form_recipe::{FormDetermination, ResolvedForm, codon_telemetry, compile_determination};
use crate::m3_state::M3Operation;
use ql_core::SiteReading;

pub const FORM_SEQUENCE_CONTRACT: &str = "ql.psg-form-sequence/v1";
pub const FORM_PROGRESS_CONTRACT: &str = "ql.psg-fold-progress/v1";
pub const FOLD_STAGE_EFFECT_CONTRACT: &str = "ql.psg-fold-stage-effect/v1";

/// The phases of one sequence: 1..=16, each 1..=360 turns of its axis.
pub const MAX_PHASES: usize = 16;
pub const MAX_PHASE_HALF_DEGREES: u64 = 360 * 720;
/// The takeover record's retained history.
pub const RETAINED_RECORDS: usize = 64;

/// The display-clock axis the sequence's cursor reads — the stage's own two
/// continuous axes (`clock.inscription`, `clock.lensing`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SequenceAxis {
    Inscription,
    Lensing,
}

impl SequenceAxis {
    pub fn slot(self) -> &'static str {
        match self {
            Self::Inscription => "clock.inscription",
            Self::Lensing => "clock.lensing",
        }
    }
}

/// The easing of a transition, exact in rational arithmetic: linear, or the
/// smoothstep `s²(3−2s)` — monotone, C0 at every boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Easing {
    Linear,
    Smoothstep,
}

impl Easing {
    /// The eased progress as the exact rational `num / den` in 0..1.
    fn eased(self, offset: u64, length: u64) -> (u64, u64) {
        match self {
            Self::Linear => (offset, length),
            // s²(3−2s) with s = offset/length:
            // (offset² · (3·length − 2·offset)) / length³.
            Self::Smoothstep => {
                let squared = offset * offset;
                (
                    squared * (3 * length - 2 * offset),
                    length * length * length,
                )
            }
        }
    }
}

/// One phase of a declared sequence. A phase either holds a determined form
/// or transitions from the standing form to one — the first phase holds,
/// because a sequence starts from a standing form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SequencePhase {
    Hold {
        determination: FormDetermination,
        half_degrees: u64,
    },
    Transition {
        to: FormDetermination,
        half_degrees: u64,
        easing: Easing,
    },
}

impl SequencePhase {
    fn half_degrees(&self) -> u64 {
        match self {
            Self::Hold { half_degrees, .. } | Self::Transition { half_degrees, .. } => {
                *half_degrees
            }
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            Self::Hold { .. } => "hold",
            Self::Transition { .. } => "transition",
        }
    }

    /// The phase's endpoint determination: the form it holds or moves to.
    fn endpoint(&self) -> &FormDetermination {
        match self {
            Self::Hold { determination, .. } => determination,
            Self::Transition { to, .. } => to,
        }
    }
}

/// A declared fold sequence over one display-clock axis. The origin is the
/// stage's own exact clock phase (`LiftInput`); the cursor is that axis's
/// exact step count — turns × 720 + half_degrees.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FoldSequence {
    pub schema: String,
    pub sequence_ref: String,
    pub revision: u64,
    pub subject_ref: String,
    pub axis: SequenceAxis,
    pub origin: LiftInput,
    pub phases: Vec<SequencePhase>,
}

/// The exact crease state one sequence stands in at one cursor: the eased
/// angle path per site, and — at the quanta only — the resolved form with the
/// cast law's own velocities. A native receipt: produced here, read by
/// consumers, never fabricated from JSON.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FoldProgress {
    pub schema: &'static str,
    pub sequence_ref: String,
    pub revision: u64,
    pub axis: SequenceAxis,
    /// The cursor in exact axis steps (turns × 720 + half_degrees).
    pub cursor_steps: u64,
    pub origin_steps: u64,
    pub segment_index: usize,
    pub phase: &'static str,
    pub offset_steps: u64,
    pub length_steps: u64,
    /// The eased progress as the exact rational `num / den` in 0..1.
    pub eased_steps_num: u64,
    pub eased_steps_den: u64,
    pub site_angles_deg10: [i32; 3],
    /// The cast law's own velocities, carried at the quanta only — never
    /// fabricated mid-transition.
    pub site_velocities_deg10: Option<[i32; 3]>,
    pub resolved_form: Option<ResolvedForm>,
    pub standing: String,
}

/// The record of what happened to a standing sequence when another took the
/// axis: an in-flight sequence is interrupted — with the exact standing it
/// stood in — and a finished one is recorded completed. A native record:
/// produced here, never fabricated from JSON.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SequenceRecord {
    /// "interrupted", "completed", or "replaced" (the owner's own
    /// re-declaration of the same sequence reference).
    pub kind: &'static str,
    pub sequence_ref: String,
    pub revision: u64,
    pub at_cursor_steps: u64,
    pub segment_index: usize,
    pub phase: &'static str,
    pub eased_steps_num: u64,
    pub eased_steps_den: u64,
    pub standing: &'static str,
}

fn bounded_text(name: &str, value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 2048 || value.chars().any(|c| c.is_control()) {
        return Err(format!("invalid {name} reference"));
    }
    Ok(())
}

/// The display-clock cursor law, exactly the stage's own: canonical turns,
/// half_degrees below a turn, steps within the exact native range.
pub fn cursor_from_lift(phase: &LiftInput) -> Result<u64, String> {
    let turns: u64 = phase
        .turns
        .parse()
        .map_err(|_| "clock phase turns must be a canonical unsigned integer")?;
    if turns.to_string() != phase.turns || phase.half_degrees >= 720 {
        return Err("clock phase must carry canonical turns and half_degrees < 720".into());
    }
    let steps = turns
        .checked_mul(720)
        .and_then(|t| t.checked_add(u64::from(phase.half_degrees)))
        .ok_or("clock phase exceeds the exact native range")?;
    if steps > crate::m2_engine::MAX_EXACT_JSON_INTEGER {
        return Err("clock phase exceeds the exact native range".into());
    }
    Ok(steps)
}

/// One endpoint's compiled form law: the determination, its resolved form and
/// the canonical cast telemetry the sequence holds and interpolates between.
#[derive(Clone)]
struct Endpoint {
    form: ResolvedForm,
    telemetry: [SiteReading; 3],
}

impl Endpoint {
    fn compile(determination: &FormDetermination) -> Result<Self, String> {
        let (_, codon) = compile_determination(determination)?;
        Ok(Self {
            form: ResolvedForm::of_codon(codon),
            telemetry: codon_telemetry(codon),
        })
    }
}

/// The compiled walk: (start, length, kind, from, to) per phase — a hold's
/// from and to are the same endpoint.
struct Walk {
    origin_steps: u64,
    segments: Vec<(u64, u64, &'static str, Endpoint, Endpoint)>,
    end_steps: u64,
}

fn walk(sequence: &FoldSequence) -> Result<Walk, String> {
    if sequence.schema != FORM_SEQUENCE_CONTRACT {
        return Err(format!(
            "unsupported fold sequence contract {}; expected {FORM_SEQUENCE_CONTRACT}",
            sequence.schema
        ));
    }
    bounded_text("sequence", &sequence.sequence_ref)?;
    if sequence.revision == 0 {
        return Err("fold sequence revision must be at least 1".into());
    }
    crate::coordinate_expression::validate_subject_ref(&sequence.subject_ref)?;
    if sequence.phases.is_empty() || sequence.phases.len() > MAX_PHASES {
        return Err(format!("a fold sequence carries 1..={MAX_PHASES} phases"));
    }
    if !matches!(sequence.phases[0], SequencePhase::Hold { .. }) {
        return Err(
            "a fold sequence starts from a standing form: its first phase must be a hold".into(),
        );
    }
    for phase in &sequence.phases {
        let length = phase.half_degrees();
        if length == 0 || length > MAX_PHASE_HALF_DEGREES {
            return Err(format!(
                "a fold phase holds 1..={MAX_PHASE_HALF_DEGREES} half_degrees of its axis"
            ));
        }
    }
    let origin_steps = cursor_from_lift(&sequence.origin)?;
    // Every phase's endpoint compiles through the form law — the refusals are
    // the recipe's own, by name.
    let compiled: Vec<Endpoint> = sequence
        .phases
        .iter()
        .map(|phase| Endpoint::compile(phase.endpoint()))
        .collect::<Result<_, _>>()?;
    let mut segments = Vec::with_capacity(sequence.phases.len());
    let mut start = origin_steps;
    for (index, phase) in sequence.phases.iter().enumerate() {
        let length = phase.half_degrees();
        let to = compiled[index].clone();
        let from = match phase {
            SequencePhase::Hold { .. } => to.clone(),
            SequencePhase::Transition { .. } => compiled[index - 1].clone(),
        };
        segments.push((start, length, phase.kind(), from, to));
        start += length;
    }
    Ok(Walk {
        origin_steps,
        end_steps: start,
        segments,
    })
}

/// `from + (to − from) · num / den`, exact in i128, rounded to the nearest
/// deg10 with ties away from zero — the declared rounding of the crease path.
fn interpolate(from: i32, to: i32, num: u64, den: u64) -> i32 {
    let den = den as i128;
    let numer = i128::from(from) * den + i128::from(to - from) * (num as i128);
    let quotient = numer / den;
    let remainder = numer % den;
    let rounded = if remainder.abs() * 2 >= den {
        quotient + remainder.signum()
    } else {
        quotient
    };
    rounded as i32
}

/// Evaluates the sequence at an exact display-clock cursor. A pure function of
/// the sequence and the cursor: seek is re-reading, not replaying.
pub fn evaluate_fold_sequence(
    sequence: &FoldSequence,
    cursor_steps: u64,
) -> Result<FoldProgress, String> {
    let Walk {
        origin_steps,
        segments,
        end_steps,
    } = walk(sequence)?;
    if cursor_steps < origin_steps {
        return Err(format!(
            "cursor {cursor_steps} stands before the sequence's origin at {origin_steps} on {}",
            sequence.axis.slot()
        ));
    }
    if cursor_steps > end_steps {
        return Err(format!(
            "cursor {cursor_steps} stands past the sequence's end at {end_steps} on {}",
            sequence.axis.slot()
        ));
    }
    // The governing segment: phases advance at their boundary; the completed
    // end stands in the final phase at full length.
    let index = segments
        .iter()
        .position(|(start, length, _, _, _)| cursor_steps < start + length)
        .unwrap_or(segments.len() - 1);
    let (start, length, kind, from, to) = &segments[index];
    let kind = *kind;
    let offset = cursor_steps - start;
    let (eased_num, eased_den) = match kind {
        "hold" => Easing::Linear.eased(offset.min(*length), *length),
        _ => {
            let easing = match &sequence.phases[index] {
                SequencePhase::Transition { easing, .. } => *easing,
                _ => unreachable!("kind names the phase"),
            };
            easing.eased(offset, *length)
        }
    };
    let site_angles_deg10 = std::array::from_fn(|site| {
        interpolate(
            from.telemetry[site].signed_angle,
            to.telemetry[site].signed_angle,
            eased_num,
            eased_den,
        )
    });
    // The quanta: a hold stands on its form; a transition's onset still stands
    // exactly on the from-form; the completed end stands on the final form.
    // Strictly inside a transition no form is named — the source determines
    // forms at the quanta, and the fold law does not fabricate one.
    let standing_endpoint = if kind == "hold" {
        Some(to)
    } else if offset == 0 {
        Some(from)
    } else if offset == *length {
        Some(to)
    } else {
        None
    };
    let completed = cursor_steps == end_steps;
    let (standing, resolved_form, site_velocities_deg10) = match standing_endpoint {
        Some(endpoint) => (
            if completed {
                "sequence completed: the final form stands".to_string()
            } else if kind == "hold" {
                "standing form: the determination's cast telemetry; the codon resolves".to_string()
            } else {
                "transition onset: the from-form still stands exactly at its quanta".to_string()
            },
            Some(endpoint.form),
            Some(endpoint.telemetry.map(|reading| reading.angular_velocity)),
        ),
        None => (
            "in transition: the eased crease path is determined; the codon resolves at the segment quanta — no form is named mid-transition".to_string(),
            None,
            None,
        ),
    };
    Ok(FoldProgress {
        schema: FORM_PROGRESS_CONTRACT,
        sequence_ref: sequence.sequence_ref.clone(),
        revision: sequence.revision,
        axis: sequence.axis,
        cursor_steps,
        origin_steps,
        segment_index: index,
        phase: kind,
        offset_steps: offset,
        length_steps: *length,
        eased_steps_num: eased_num,
        eased_steps_den: eased_den,
        site_angles_deg10,
        site_velocities_deg10,
        resolved_form,
        standing,
    })
}

/// The exact display-clock phase of a cursor: canonical turns and
/// half_degrees below a turn — the inverse of [`cursor_from_lift`]. The host's
/// own axis phase, compiled from the fold law's step count.
pub fn lift_from_cursor(cursor_steps: u64) -> Result<LiftInput, String> {
    if cursor_steps > crate::m2_engine::MAX_EXACT_JSON_INTEGER {
        return Err("clock phase exceeds the exact native range".into());
    }
    Ok(LiftInput {
        turns: (cursor_steps / 720).to_string(),
        half_degrees: (cursor_steps % 720) as u16,
    })
}

/// The determination whose form the progress names at a quanta: a hold's own
/// endpoint, a transition onset's from-form (the previous phase's endpoint —
/// a transition is never the first segment), a transition end's target.
fn governing_determination<'a>(
    sequence: &'a FoldSequence,
    progress: &FoldProgress,
) -> &'a FormDetermination {
    let index = progress.segment_index;
    if progress.phase == "hold" || progress.offset_steps == progress.length_steps {
        sequence.phases[index].endpoint()
    } else {
        sequence.phases[index - 1].endpoint()
    }
}

/// The stage-host effect binding (P5 §4.2, the stage's own sequence
/// semantics): what the Ta-Onta stage performs for the fold sequence at one
/// exact display-clock cursor — the fold law compiled into the host's own
/// typed changes. A native receipt: produced here, read by consumers, never
/// fabricated from JSON.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StageEffect {
    pub schema: &'static str,
    pub sequence_ref: String,
    pub revision: u64,
    pub subject_ref: String,
    pub axis: SequenceAxis,
    pub cursor_steps: u64,
    /// The cursor as the exact display-clock phase the host sets.
    pub cursor_phase: LiftInput,
    /// The commanded fold state at this cursor — the progress law's own
    /// reading, unchanged.
    pub progress: FoldProgress,
    /// The boundary form's exact M3 operations — the same compiled
    /// determination the form-recipe owner names — present only when the codon
    /// resolved onto a form the host's disclosed standing does not hold.
    /// Strictly inside a transition this is absent: the codon resolves at the
    /// quanta only, and the host is issued no form mid-path.
    pub form_operations: Option<Vec<M3Operation>>,
    pub standing: String,
}

impl StageEffect {
    /// The host's own typed changes carrying this effect, in the application
    /// order the stage's evaluate path performs them: the boundary form onto
    /// the event's own command batch when one resolved, then the display axis
    /// moved to the cursor.
    pub fn changes(&self) -> Vec<StageChange> {
        let mut changes = Vec::with_capacity(2);
        if let Some(operations) = &self.form_operations {
            changes.push(StageChange::Form {
                operations: operations.clone(),
            });
        }
        changes.push(StageChange::Clock {
            slot: self.axis.slot().to_owned(),
            phase: self.cursor_phase.clone(),
        });
        changes
    }

    /// The stage slots the effect's changes address — the selector the
    /// carrying procedure must declare.
    pub fn selector(&self) -> Vec<String> {
        let mut slots = Vec::with_capacity(2);
        if self.form_operations.is_some() {
            slots.push("form".to_owned());
        }
        slots.push(self.axis.slot().to_owned());
        slots
    }
}

/// Compiles the stage effect for the sequence at one exact cursor against the
/// host's disclosed standing form address. Pure: the same sequence, cursor and
/// standing name the same effect — the driven stage re-reads, never replays,
/// and a form already standing is never re-issued.
pub fn stage_effect(
    sequence: &FoldSequence,
    cursor_steps: u64,
    standing_address: u8,
) -> Result<StageEffect, String> {
    let progress = evaluate_fold_sequence(sequence, cursor_steps)?;
    let cursor_phase = lift_from_cursor(cursor_steps)?;
    let form_operations = match progress.resolved_form.as_ref() {
        None => None,
        // The codon's form already stands on the host: the binding re-issues
        // nothing — the boundary is carried, not replayed every read.
        Some(form) if form.address == standing_address => None,
        Some(form) => {
            let (operations, codon) =
                compile_determination(governing_determination(sequence, &progress))?;
            if codon.address() != form.address {
                return Err(format!(
                    "the sequence's governing determination resolves form {}, but the progress named {} at cursor {cursor_steps}",
                    codon.address(),
                    form.address
                ));
            }
            Some(operations)
        }
    };
    let standing = match (&form_operations, progress.resolved_form.as_ref()) {
        (Some(_), Some(form)) => format!(
            "boundary resolution: the codon resolved onto form {} at the quanta; the compiled form change rides the event's own command batch",
            form.address
        ),
        (None, Some(form)) => format!(
            "quanta already bound: the codon's form {form_address} stands on the host; no form change is issued",
            form_address = form.address
        ),
        (None, None) => {
            "in transition: no form is named and none is issued; the eased crease path is the commanded state the sampler deforms the retained body with".to_string()
        }
        (Some(_), None) => unreachable!("a form change exists only at a named quanta"),
    };
    Ok(StageEffect {
        schema: FOLD_STAGE_EFFECT_CONTRACT,
        sequence_ref: sequence.sequence_ref.clone(),
        revision: sequence.revision,
        subject_ref: sequence.subject_ref.clone(),
        axis: sequence.axis,
        cursor_steps,
        cursor_phase,
        progress,
        form_operations,
        standing,
    })
}

/// The native owner of the active fold sequence on its axis. A takeover is a
/// named act with a recorded receipt: the displaced sequence's exact standing
/// is retained, never silently discarded.
#[derive(Debug, Clone, Default)]
pub struct FoldSequenceOwner {
    active: Option<StandingSequence>,
    records: Vec<SequenceRecord>,
}

#[derive(Debug, Clone)]
struct StandingSequence {
    sequence: FoldSequence,
    end_steps: u64,
    last_cursor: u64,
    evaluations: u32,
}

impl FoldSequenceOwner {
    /// Declares a sequence on the axis. A standing sequence is recorded
    /// interrupted (if in flight), completed (if finished) or replaced (if
    /// this is the owner's own re-declaration of the same reference), and the
    /// returned record names it. The history retains the last
    /// [`RETAINED_RECORDS`] records.
    pub fn declare(&mut self, sequence: FoldSequence) -> Result<Option<SequenceRecord>, String> {
        let compiled = walk(&sequence)?;
        let record = match self.active.take() {
            None => None,
            Some(standing) => {
                let kind = if standing.sequence.sequence_ref == sequence.sequence_ref {
                    "replaced"
                } else if standing.last_cursor < standing.end_steps {
                    "interrupted"
                } else {
                    "completed"
                };
                let progress = evaluate_fold_sequence(&standing.sequence, standing.last_cursor)
                    .expect("the standing cursor stays within its own sequence");
                let record = SequenceRecord {
                    kind,
                    sequence_ref: standing.sequence.sequence_ref.clone(),
                    revision: standing.sequence.revision,
                    at_cursor_steps: standing.last_cursor,
                    segment_index: progress.segment_index,
                    phase: progress.phase,
                    eased_steps_num: progress.eased_steps_num,
                    eased_steps_den: progress.eased_steps_den,
                    standing: if kind == "interrupted" {
                        "interrupted in flight; the standing it stood in is recorded"
                    } else {
                        "released with its standing recorded"
                    },
                };
                self.retain(record.clone());
                Some(record)
            }
        };
        self.active = Some(StandingSequence {
            sequence,
            end_steps: compiled.end_steps,
            last_cursor: compiled.origin_steps,
            evaluations: 0,
        });
        Ok(record)
    }

    fn retain(&mut self, record: SequenceRecord) {
        self.records.push(record);
        let excess = self.records.len().saturating_sub(RETAINED_RECORDS);
        self.records.drain(..excess);
    }

    /// Evaluates the active sequence at an exact cursor, recording the cursor
    /// the sequence now stands at (seek moves it; evaluation itself is pure).
    pub fn progress_at(&mut self, cursor_steps: u64) -> Result<FoldProgress, String> {
        let standing = self
            .active
            .as_mut()
            .ok_or("no active fold sequence on the axis; declare one first")?;
        let progress = evaluate_fold_sequence(&standing.sequence, cursor_steps)?;
        standing.last_cursor = cursor_steps;
        standing.evaluations += 1;
        Ok(progress)
    }

    /// The active sequence's reference and revision, when one stands.
    pub fn active(&self) -> Option<(&str, u64)> {
        let standing = self.active.as_ref()?;
        Some((
            standing.sequence.sequence_ref.as_str(),
            standing.sequence.revision,
        ))
    }

    /// The active sequence's standing: reference, revision, the cursor it
    /// stands at, and how many times progress was read through the owner.
    pub fn standing_state(&self) -> Option<(&str, u64, u64, u32)> {
        let standing = self.active.as_ref()?;
        Some((
            standing.sequence.sequence_ref.as_str(),
            standing.sequence.revision,
            standing.last_cursor,
            standing.evaluations,
        ))
    }

    /// The retained takeover records, oldest first.
    pub fn records(&self) -> &[SequenceRecord] {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form_recipe::{DeclaredMobility, DeclaredPolarity, DeclaredSite};

    fn origin() -> LiftInput {
        LiftInput {
            turns: "0".into(),
            half_degrees: 0,
        }
    }

    /// The event's standing form: address 7 — X yin/moving (A), hinge Y
    /// yang/moving (T), Z yang/resting (G).
    fn address(address: u8) -> FormDetermination {
        FormDetermination::Address { address }
    }

    /// ATC, the increment-1 specimen motif: angles [225, −225, 225],
    /// velocities [225, 225, 0] deg10.
    fn atc() -> FormDetermination {
        FormDetermination::FoldMotif {
            sites: [
                DeclaredSite {
                    polarity: DeclaredPolarity::Yin,
                    mobility: DeclaredMobility::Moving,
                },
                DeclaredSite {
                    polarity: DeclaredPolarity::Yang,
                    mobility: DeclaredMobility::Moving,
                },
                DeclaredSite {
                    polarity: DeclaredPolarity::Yin,
                    mobility: DeclaredMobility::Resting,
                },
            ],
        }
    }

    /// hold 7 one turn, transition to ATC over one turn (smoothstep), hold ATC
    /// one turn. Only the Z site moves: −225° → +225°.
    fn sequence() -> FoldSequence {
        FoldSequence {
            schema: FORM_SEQUENCE_CONTRACT.into(),
            sequence_ref: "ta-onta:psg:moon-fold".into(),
            revision: 1,
            subject_ref: "ql:k2/default-subject".into(),
            axis: SequenceAxis::Inscription,
            origin: origin(),
            phases: vec![
                SequencePhase::Hold {
                    determination: address(7),
                    half_degrees: 720,
                },
                SequencePhase::Transition {
                    to: atc(),
                    half_degrees: 720,
                    easing: Easing::Smoothstep,
                },
                SequencePhase::Hold {
                    determination: atc(),
                    half_degrees: 720,
                },
            ],
        }
    }

    #[test]
    fn the_display_clock_cursor_is_the_stage_exact_law() {
        assert_eq!(
            cursor_from_lift(&LiftInput {
                turns: "1".into(),
                half_degrees: 36,
            })
            .unwrap(),
            756
        );
        assert!(
            cursor_from_lift(&LiftInput {
                turns: "01".into(),
                half_degrees: 0,
            })
            .unwrap_err()
            .contains("canonical")
        );
        assert!(
            cursor_from_lift(&LiftInput {
                turns: "0".into(),
                half_degrees: 720,
            })
            .unwrap_err()
            .contains("half_degrees")
        );
    }

    #[test]
    fn hold_telemetry_is_the_cast_law_and_the_form_resolves() {
        let seq = sequence();
        let progress = evaluate_fold_sequence(&seq, 0).unwrap();
        assert_eq!(progress.schema, FORM_PROGRESS_CONTRACT);
        assert_eq!(progress.phase, "hold");
        assert_eq!(progress.segment_index, 0);
        // Codon 7: X yin/moving +225, Y yang/moving −225, Z yang/resting −225;
        // velocities 225, 225, 0.
        assert_eq!(progress.site_angles_deg10, [225, -225, -225]);
        assert_eq!(progress.site_velocities_deg10, Some([225, 225, 0]));
        let form = progress.resolved_form.expect("a hold names its form");
        assert_eq!(form.address, 7);
        assert!(progress.standing.contains("standing form"));
        // The hold's cursor passes; the form does not move.
        let later = evaluate_fold_sequence(&seq, 719).unwrap();
        assert_eq!(later.site_angles_deg10, progress.site_angles_deg10);
        assert_eq!(later.resolved_form, progress.resolved_form);
    }

    #[test]
    fn the_transition_is_the_exact_eased_crease_path_with_no_mid_form() {
        let seq = sequence();
        // Transition onset: the from-form still stands exactly.
        let onset = evaluate_fold_sequence(&seq, 720).unwrap();
        assert_eq!(onset.phase, "transition");
        assert_eq!(onset.segment_index, 1);
        assert_eq!(onset.offset_steps, 0);
        assert_eq!(onset.site_angles_deg10, [225, -225, -225]);
        assert_eq!(onset.resolved_form.unwrap().address, 7);
        assert!(onset.standing.contains("onset"));

        // Smoothstep midpoint (offset 360 of 720): eased exactly 1/2 — the Z
        // crease stands at the exact midpoint 0°; X and Y do not move.
        let mid = evaluate_fold_sequence(&seq, 1080).unwrap();
        assert_eq!(mid.eased_steps_num * 2, mid.eased_steps_den, "exact 1/2");
        assert_eq!(mid.site_angles_deg10, [225, -225, 0]);
        assert!(
            mid.resolved_form.is_none(),
            "no form is named mid-transition"
        );
        assert!(mid.site_velocities_deg10.is_none());
        assert!(mid.standing.contains("no form is named"));

        // Analytic smoothstep at s = 1/4: eased exactly 5/32; the Z crease at
        // −225 + 450 · 5/32 = −154.6875 deg10 → −155 deg10 (nearest, ties away).
        let quarter = evaluate_fold_sequence(&seq, 720 + 180).unwrap();
        assert_eq!(quarter.eased_steps_num * 32, quarter.eased_steps_den * 5);
        assert_eq!(quarter.site_angles_deg10[2], -155);
        assert_eq!(quarter.site_angles_deg10[0], 225);
        assert_eq!(quarter.site_angles_deg10[1], -225);

        // Linear over the same span moves faster early but meets the same
        // ends: at s = 1/4 the Z crease is −225 + 450/4 = −112.5 deg10 →
        // −113 deg10 (nearest, ties away from zero).
        let mut linear = sequence();
        linear.phases[1] = SequencePhase::Transition {
            to: atc(),
            half_degrees: 720,
            easing: Easing::Linear,
        };
        let linear_quarter = evaluate_fold_sequence(&linear, 720 + 180).unwrap();
        assert_eq!(
            linear_quarter.eased_steps_num * 4,
            linear_quarter.eased_steps_den
        );
        assert_eq!(linear_quarter.site_angles_deg10[2], -113);
        assert_ne!(
            linear_quarter.site_angles_deg10[2], quarter.site_angles_deg10[2],
            "the easings differ inside the segment"
        );
    }

    #[test]
    fn boundaries_are_continuous_and_the_completed_end_stands() {
        let seq = sequence();
        // C0: one display step before the transition's end, the Z crease is
        // within two of the from-form's angle at the boundary.
        let before = evaluate_fold_sequence(&seq, 1439).unwrap();
        assert_eq!(before.phase, "transition");
        assert!(
            (before.site_angles_deg10[2] - (-225)).abs() <= 450 * 2,
            "one display step from the boundary"
        );
        let boundary = evaluate_fold_sequence(&seq, 1440).unwrap();
        assert_eq!(boundary.segment_index, 2);
        assert_eq!(boundary.phase, "hold");
        assert_eq!(boundary.site_angles_deg10, [225, -225, 225]);
        assert_eq!(boundary.resolved_form.unwrap().address, 0b00_01_10);
        // The completed end: the final form stands at full length.
        let end = evaluate_fold_sequence(&seq, 2160).unwrap();
        assert_eq!(end.site_angles_deg10, [225, -225, 225]);
        assert_eq!(end.resolved_form.unwrap().address, 0b00_01_10);
        assert!(end.standing.contains("sequence completed"));
        // Outside the declared domain, refusal by name.
        let past = evaluate_fold_sequence(&seq, 2161).unwrap_err();
        assert!(past.contains("past the sequence's end"), "{past}");
        let mut late_origin = sequence();
        late_origin.origin = LiftInput {
            turns: "1".into(),
            half_degrees: 0,
        };
        assert!(
            evaluate_fold_sequence(&late_origin, 719)
                .unwrap_err()
                .contains("before the sequence's origin")
        );
    }

    #[test]
    fn seek_is_re_reading_never_replaying() {
        let seq = sequence();
        // Any cursor evaluates identically however it is reached: directly,
        // after a later cursor, or stepping back — no hidden state.
        let direct = evaluate_fold_sequence(&seq, 1080).unwrap();
        evaluate_fold_sequence(&seq, 1500).unwrap();
        let back = evaluate_fold_sequence(&seq, 1080).unwrap();
        assert_eq!(direct, back);
        let stepped_on = evaluate_fold_sequence(&seq, 1080).unwrap();
        assert_eq!(stepped_on, direct);
        // And the one-step neighbour lands within one site step of it.
        let stepped = evaluate_fold_sequence(&seq, 1079).unwrap();
        assert!((stepped.site_angles_deg10[2] - direct.site_angles_deg10[2]).abs() <= 450 * 2);
    }

    #[test]
    fn refusals_name_the_sequence_law() {
        let mut wrong_schema = sequence();
        wrong_schema.schema = "ql.psg-form-sequence/v0".into();
        assert!(
            evaluate_fold_sequence(&wrong_schema, 0)
                .unwrap_err()
                .contains("unsupported fold sequence contract")
        );
        let mut no_hold = sequence();
        no_hold.phases.remove(0);
        assert!(
            evaluate_fold_sequence(&no_hold, 0)
                .unwrap_err()
                .contains("starts from a standing form")
        );
        let mut empty = sequence();
        empty.phases.clear();
        assert!(
            evaluate_fold_sequence(&empty, 0)
                .unwrap_err()
                .contains("1..=16 phases")
        );
        let mut zero = sequence();
        zero.phases[0] = SequencePhase::Hold {
            determination: address(7),
            half_degrees: 0,
        };
        assert!(
            evaluate_fold_sequence(&zero, 0)
                .unwrap_err()
                .contains("half_degrees of its axis")
        );
        let mut unbound = sequence();
        unbound.phases[0] = SequencePhase::Hold {
            determination: address(64),
            half_degrees: 720,
        };
        let error = evaluate_fold_sequence(&unbound, 0).unwrap_err();
        assert!(error.contains("six-bit field 0..64"), "{error}");
        let mut anonymous = sequence();
        anonymous.sequence_ref = String::new();
        assert!(
            evaluate_fold_sequence(&anonymous, 0)
                .unwrap_err()
                .contains("invalid sequence reference")
        );
        let mut foreign = sequence();
        foreign.subject_ref = "subject\0bad".into();
        assert!(evaluate_fold_sequence(&foreign, 0).is_err());
        let mut late = sequence();
        late.origin = LiftInput {
            turns: "01".into(),
            half_degrees: 0,
        };
        assert!(evaluate_fold_sequence(&late, 0).is_err());
    }

    #[test]
    fn interruption_is_recorded_with_the_exact_standing() {
        let mut owner = FoldSequenceOwner::default();
        assert!(owner.active().is_none());
        assert!(
            owner
                .progress_at(0)
                .unwrap_err()
                .contains("no active fold sequence")
        );
        assert!(owner.declare(sequence()).unwrap().is_none());
        assert_eq!(owner.active(), Some(("ta-onta:psg:moon-fold", 1)));
        assert_eq!(
            owner.standing_state(),
            Some(("ta-onta:psg:moon-fold", 1, 0, 0))
        );
        // The sequence stands mid-transition when a successor takes the axis.
        let mid = owner.progress_at(1000).unwrap();
        assert_eq!(mid.segment_index, 1);
        assert_eq!(
            owner.standing_state(),
            Some(("ta-onta:psg:moon-fold", 1, 1000, 1))
        );
        let mut successor = sequence();
        successor.sequence_ref = "ta-onta:psg:successor".into();
        successor.revision = 2;
        let record = owner
            .declare(successor)
            .unwrap()
            .expect("a takeover record");
        assert_eq!(record.kind, "interrupted");
        assert_eq!(record.sequence_ref, "ta-onta:psg:moon-fold");
        assert_eq!(record.at_cursor_steps, 1000);
        assert_eq!(record.segment_index, mid.segment_index);
        assert_eq!(record.phase, "transition");
        assert_eq!(record.eased_steps_num, mid.eased_steps_num);
        assert_eq!(record.eased_steps_den, mid.eased_steps_den);
        assert_eq!(owner.active(), Some(("ta-onta:psg:successor", 2)));
        // The successor runs to completion; a third declaration records the
        // completed standing, and the same reference re-declared is a replace.
        owner.progress_at(2160).unwrap();
        let completed = owner
            .declare(sequence())
            .unwrap()
            .expect("completion record");
        assert_eq!(completed.kind, "completed");
        assert_eq!(completed.at_cursor_steps, 2160);
        let replaced = owner.declare(sequence()).unwrap().expect("replace record");
        assert_eq!(replaced.kind, "replaced");
        // The history retains every takeover, oldest first.
        let records = owner.records();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].kind, "interrupted");
        assert_eq!(records[1].kind, "completed");
        assert_eq!(records[2].kind, "replaced");
    }

    #[test]
    fn the_sequence_declaration_travels_exactly() {
        // The declared sequence is plain data: serialization round trip exact.
        let seq = sequence();
        let json = serde_json::to_value(&seq).unwrap();
        let back: FoldSequence = serde_json::from_value(json).unwrap();
        assert_eq!(
            serde_json::to_value(&back).unwrap(),
            serde_json::to_value(&seq).unwrap()
        );
    }

    #[test]
    fn the_progress_law_never_touches_the_retained_body() {
        // The dependency law, executed: the sample body's cache key and every
        // retained rest coordinate stand identical under every cursor.
        use crate::form_samples::{
            FORM_SAMPLES_CONTRACT, MaterialTreatment, SampleLayer, SamplePreparation, SampleUnits,
            prepare_samples,
        };
        let preparation = SamplePreparation {
            schema: FORM_SAMPLES_CONTRACT.into(),
            prep_ref: "ta-onta:psg:moon-square".into(),
            treatment: MaterialTreatment::SheetCarrier,
            layers: vec![SampleLayer {
                layer_ref: "glyph:front".into(),
                rest_depth_w: 0.5,
            }],
            resolution: 4,
            units: SampleUnits {
                extent_units: 2.0,
                depth_units: 0.5,
                metres_per_unit: 1.0,
            },
            mask: None,
        };
        let body = prepare_samples(preparation.clone()).unwrap();
        let rest_before: Vec<[f64; 3]> = body.samples.iter().map(|s| body.rest_metres(s)).collect();
        let mut owner = FoldSequenceOwner::default();
        owner.declare(sequence()).unwrap();
        for cursor in [0, 720, 1080, 1440, 2160] {
            owner.progress_at(cursor).unwrap();
        }
        let body_after = prepare_samples(preparation).unwrap();
        assert_eq!(body_after.preparation_sha256, body.preparation_sha256);
        let rest_after: Vec<[f64; 3]> = body_after
            .samples
            .iter()
            .map(|s| body_after.rest_metres(s))
            .collect();
        assert_eq!(rest_before, rest_after);
    }

    #[test]
    fn the_cursor_phase_is_the_exact_inverse_of_the_cursor_law() {
        for cursor in [0u64, 1, 719, 720, 756, 1440, 2160, 123_457] {
            let phase = lift_from_cursor(cursor).unwrap();
            assert_eq!(phase.half_degrees as u64, cursor % 720);
            assert_eq!(cursor_from_lift(&phase).unwrap(), cursor);
            assert_eq!(
                cursor_from_lift(&phase).unwrap(),
                cursor / 720 * 720 + cursor % 720
            );
        }
        assert!(lift_from_cursor(u64::MAX).is_err());
    }

    #[test]
    fn the_stage_effect_binds_the_law_to_the_hosts_own_changes() {
        let seq = sequence();
        // Origin, the host standing on form 7 (the sequence's own hold): no
        // form change exists — the law's standing IS the host's standing.
        let at_origin = stage_effect(&seq, 0, 7).unwrap();
        assert_eq!(at_origin.schema, FOLD_STAGE_EFFECT_CONTRACT);
        assert_eq!(at_origin.cursor_steps, 0);
        assert_eq!(at_origin.cursor_phase, origin());
        assert!(at_origin.form_operations.is_none());
        assert_eq!(at_origin.selector(), vec!["clock.inscription".to_owned()]);
        assert_eq!(at_origin.changes().len(), 1);
        assert!(at_origin.standing.contains("quanta already bound"));
        assert_eq!(at_origin.progress.resolved_form.unwrap().address, 7);

        // Strictly inside the transition: no form is named and none is issued.
        let mid = stage_effect(&seq, 1080, 7).unwrap();
        assert!(mid.form_operations.is_none());
        assert_eq!(mid.selector(), vec!["clock.inscription".to_owned()]);
        assert_eq!(mid.progress.site_angles_deg10, [225, -225, 0]);
        assert!(mid.standing.contains("no form is named and none is issued"));

        // The boundary: the codon resolves onto ATC while the host stands on
        // 7 — the effect compiles the SAME operations the recipe of that
        // determination compiles, and the changes carry the clock and the form.
        let boundary = stage_effect(&seq, 1440, 7).unwrap();
        assert_eq!(
            boundary.cursor_phase,
            LiftInput {
                turns: "2".into(),
                half_degrees: 0
            }
        );
        let (expected, _) = compile_determination(&atc()).unwrap();
        assert_eq!(boundary.form_operations.as_ref().unwrap(), &expected);
        assert_eq!(
            boundary.selector(),
            vec!["form".to_owned(), "clock.inscription".to_owned()]
        );
        let changes = boundary.changes();
        assert_eq!(changes.len(), 2);
        assert_eq!(
            serde_json::to_value(&changes[0]).unwrap(),
            serde_json::to_value(&crate::continuous::stage::StageChange::Form {
                operations: expected.clone()
            })
            .unwrap()
        );
        assert!(matches!(
            &changes[1],
            StageChange::Clock { slot, .. } if slot == "clock.inscription"
        ));
        assert!(boundary.standing.contains("boundary resolution"));

        // The same boundary with the form already bound: re-issued nothing.
        let rebound = stage_effect(&seq, 1440, 0b00_01_10).unwrap();
        assert!(rebound.form_operations.is_none());
        assert!(rebound.standing.contains("quanta already bound"));

        // The transition onset names the from-form: a host standing elsewhere
        // is told the law's standing at the one quanta where naming is lawful.
        let onset = stage_effect(&seq, 720, 3).unwrap();
        let (from_ops, from_codon) = compile_determination(&address(7)).unwrap();
        assert_eq!(from_codon.address(), 7);
        assert_eq!(onset.form_operations.as_ref().unwrap(), &from_ops);
        assert!(onset.standing.contains("boundary resolution"));

        // Purity: the same sequence, cursor and standing name the same effect.
        assert_eq!(
            stage_effect(&seq, 1440, 7).unwrap(),
            stage_effect(&seq, 1440, 7).unwrap()
        );
        // And the cursor law's own refusals surface by name.
        assert!(
            stage_effect(&seq, 2161, 7)
                .unwrap_err()
                .contains("past the sequence's end")
        );
    }
}
