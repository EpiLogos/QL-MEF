//! The Ta-Onta procedural stage over the live coupled scene (QL-MEF #296).
//!
//! A procedure is a versioned, source-qualified recipe that injects state
//! changes or generated scenes into the running flow. It addresses only this
//! stage's real constituents — the M3 form determinant, the declared material
//! policy and the continuous display clocks — and compiles to the same native
//! operations the coupled owner already admits. Nothing here invents a second
//! event universe: an M3 change rides the event's own command batch, a
//! material/clock change rides the scene instrument's own determinant paths,
//! and a generated passage rides the admitted M1 form advance, one admitted
//! determinant per generated scene.
//!
//! Procedural regeneration owns identified contributions. A contribution key
//! derives from the procedure identity, its revision and the addressed slot;
//! re-evaluation replaces a procedure's own basis and never another
//! procedure's. Evaluation is deterministic in the event basis: no clock, no
//! randomness — the exact operations a plan names are the operations the host
//! performs, and the recorded warrants travel with every receipt.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::LiftInput;
use super::coupled::CoupledInput;
use crate::m3_state::{COMMAND_SCHEMA, M3Command, M3Operation};

pub const STAGE_PROCEDURE: &str = "ql.stage-procedure/v1";
pub const STAGE_CONTRIBUTION: &str = "ql.stage-contribution/v1";
pub const STAGE_RECEIPT: &str = "ql.stage-receipt/v1";
pub const STAGE_STATE: &str = "ql.stage-state/v1";

/// The constituents a procedure may address, by this stage's own refs. A
/// selector names a set; a change addresses it. Voice retuning has no admitted
/// stage change yet and is refused by name rather than silently ignored.
pub const STAGE_SLOTS: [&str; 4] = [
    "form",
    "material.damping",
    "clock.inscription",
    "clock.lensing",
];

fn clock_slot(slot: &str) -> Option<&'static str> {
    match slot {
        "clock.inscription" => Some("clock.inscription"),
        "clock.lensing" => Some("clock.lensing"),
        _ => None,
    }
}

/// What invokes a procedure: an explicit evaluation (human or Agent), or a
/// named determinant operation on the host. A bound determinant procedure
/// fires only when the host itself performs that admitted operation as a
/// request — never on a timer, a background loop or from inside another
/// procedure's own application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "trigger", rename_all = "kebab-case", deny_unknown_fields)]
pub enum StageTrigger {
    Invocation,
    Determinant { operation: String },
}

/// The host determinant operations a binding may follow.
pub const STAGE_DETERMINANTS: [&str; 2] = ["m1-advance", "replace-event"];

impl StageTrigger {
    fn cause_ref(&self) -> String {
        match self {
            Self::Invocation => "ta-onta:invocation".into(),
            Self::Determinant { operation } => format!("ta-onta:determinant:{operation}"),
        }
    }

    /// The admitted determinant operation a binding follows; an invocation
    /// trigger binds to nothing.
    pub fn determinant(&self) -> Result<&'static str, String> {
        match self {
            Self::Invocation => Err(
                "only determinant-triggered procedures bind to the live flow; an invocation procedure evaluates explicitly"
                    .into(),
            ),
            Self::Determinant { operation } => STAGE_DETERMINANTS
                .iter()
                .find(|admitted| admitted == &operation)
                .copied()
                .ok_or_else(|| {
                    format!(
                        "unknown determinant trigger {operation:?}; admitted: {STAGE_DETERMINANTS:?}"
                    )
                }),
        }
    }
}

/// One typed stage change, drawn only from operations the coupled owner
/// already admits. The M3 operations are the source-qualified form law —
/// fold/pose/line/matrix/clock/aperture, including `CastCreases` — applied
/// through the event's own command batch with its native receipt. A played
/// strike is a momentary performance act on the standing voices, not owned
/// state: it claims no slot and survives no re-evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "change", rename_all = "kebab-case", deny_unknown_fields)]
pub enum StageChange {
    Form {
        operations: Vec<M3Operation>,
    },
    Damping {
        per_second: f64,
    },
    Clock {
        slot: String,
        phase: LiftInput,
    },
    Strike {
        mode_ref: String,
        amplitude: [f64; 2],
    },
}

impl StageChange {
    /// The stage slot this change addresses, when it addresses owned state at
    /// all; a change may only address a slot its own kind admits.
    pub fn slot(&self) -> Result<Option<&'static str>, String> {
        match self {
            Self::Form { .. } => Ok(Some("form")),
            Self::Damping { .. } => Ok(Some("material.damping")),
            Self::Clock { slot, .. } => clock_slot(slot).map(Some).ok_or_else(|| {
                format!("a clock change addresses clock.inscription or clock.lensing, not {slot}")
            }),
            Self::Strike { .. } => Ok(None),
        }
    }

    fn warrant(&self) -> Value {
        match self {
            Self::Form { operations } => json!({
                "determinant": "the event's own M3 form law",
                "through": "M3 command batch applied by the coupled composer, receipt retained",
                "effect": "form/pose/line/matrix/clock/aperture/crease determination on the live body",
                "operations": operations,
                "warrant": "source-defined (M3 state owner; C-kernel parity-tested)"}),
            Self::Damping { per_second } => json!({
                "determinant": "declared material policy (D30)",
                "through": "the scene instrument's damping continuation on the resident voices",
                "effect": format!("decay per second set to {per_second} 1/s"),
                "units": "1/s",
                "warrant": "declared policy — no source table fixes damping (QL-MEF #135)"}),
            Self::Clock { slot, phase } => json!({
                "determinant": "the continuous display clock",
                "through": format!("the scene instrument's set-axis continuation on {slot}"),
                "effect": "inscription/lensing display phase; the selected M3 aperture is action-only and unaffected",
                "slot": slot,
                "phase": phase,
                "warrant": "declared display driver, distinct from the admitted M3 clock action"}),
            Self::Strike {
                mode_ref,
                amplitude,
            } => json!({
                "determinant": "the played strike",
                "through": "the scene instrument's played excitation on the named standing voice",
                "effect": format!("momentary modal excitation of {mode_ref}"),
                "mode_ref": mode_ref,
                "amplitude": amplitude,
                "warrant": "performance act, not owned state (QL-MEF #281 slice 1); momentary lifetime per the control contract"}),
        }
    }
}

/// A generated passage: follow-on scenes derived from the current event by the
/// source's own form advance. Each generated scene is one admitted M1
/// determinant (one tick: the ring selects the codon, the inscription clock
/// follows, the Vimarśā reading re-reads the body), applied by the host in
/// passage order. The count is bounded; the derivation is recorded, so replay
/// is exact without any hidden generator state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagePassage {
    /// 2..=8 scenes: the invoked state plus its generated continuations.
    pub scenes: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageProcedure {
    pub schema: String,
    /// Stable procedure identity; the key a contribution is owned by.
    pub procedure_ref: String,
    pub revision: u64,
    pub subject_ref: String,
    pub trigger: StageTrigger,
    /// The constituents this procedure addresses. Every change's slot must be
    /// selected.
    pub selector: Vec<String>,
    pub changes: Vec<StageChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passage: Option<StagePassage>,
}

impl StageProcedure {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != STAGE_PROCEDURE {
            return Err(format!(
                "unsupported stage procedure contract {}; expected {STAGE_PROCEDURE}",
                self.schema
            ));
        }
        if self.procedure_ref.is_empty()
            || self.procedure_ref.len() > 2048
            || self.procedure_ref.chars().any(|c| c.is_control())
        {
            return Err("invalid stage procedure reference".into());
        }
        // PS-E (QL-MEF #297): the subject is validated through its semantic
        // owner, the one grammar the Atlas manifestation resolver admits, so
        // a procedure's subject is manifestable wherever the stage plays it.
        crate::coordinate_expression::validate_subject_ref(&self.subject_ref)?;
        if self.revision == 0 {
            return Err("stage procedure revision must be at least 1".into());
        }
        if self.selector.len() > STAGE_SLOTS.len() {
            return Err(format!(
                "a procedure selects at most {} distinct stage slots",
                STAGE_SLOTS.len()
            ));
        }
        for slot in &self.selector {
            if slot.starts_with("voice:") || !STAGE_SLOTS.contains(&slot.as_str()) {
                return Err(format!(
                    "unknown stage slot {slot:?}; admitted: {STAGE_SLOTS:?} (voice retuning is not an admitted stage change yet)"
                ));
            }
        }
        if self.changes.is_empty() || self.changes.len() > 16 {
            return Err("a procedure carries 1..16 changes".into());
        }
        let mut slots = Vec::new();
        for change in &self.changes {
            if let Some(slot) = change.slot()? {
                if slots.contains(&slot) {
                    return Err(format!("two changes address {slot} in one procedure"));
                }
                slots.push(slot);
            }
        }
        if !slots.is_empty() && self.selector.is_empty() {
            return Err(
                "a procedure whose changes claim owned state must select the slots it addresses"
                    .into(),
            );
        }
        for slot in &slots {
            if !self.selector.iter().any(|s| s == slot) {
                return Err(format!(
                    "change addresses {slot} outside the procedure's selector"
                ));
            }
        }
        if let Some(passage) = &self.passage
            && !(2..=8).contains(&passage.scenes)
        {
            return Err("a generated passage holds 2..=8 scenes".into());
        }
        Ok(())
    }

    fn key(&self, slot: &str) -> String {
        format!("{}@{}/{}", self.procedure_ref, self.revision, slot)
    }
}

/// One owned slot of the live stage. Procedural updates replace their own
/// generated basis; a different procedure writing an owned slot is a refusal,
/// because arrival order is not a semantic arbitration rule.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Contribution {
    pub key: String,
    pub procedure_ref: String,
    pub revision: u64,
    pub slot: &'static str,
    pub warrant: Value,
}

/// Who owns which live slot. Authored (caller-supplied) state stays outside
/// this map: only procedures claim, replace and release here.
#[derive(Debug, Clone, Default)]
pub struct StageOwnership {
    owned: BTreeMap<&'static str, Contribution>,
}

impl StageOwnership {
    pub fn owner_of(&self, slot: &str) -> Option<&Contribution> {
        self.owned.get(slot)
    }

    pub fn contributions(&self) -> impl Iterator<Item = &Contribution> {
        self.owned.values()
    }

    fn claim(
        &mut self,
        procedure: &StageProcedure,
        slot: &'static str,
        warrant: Value,
    ) -> Result<Contribution, String> {
        if let Some(held) = self.owned.get(slot)
            && held.procedure_ref != procedure.procedure_ref
        {
            return Err(format!(
                "stage slot {} is owned by {}; two procedures writing one property need an explicit composition, not arrival order",
                slot, held.key
            ));
        }
        let contribution = Contribution {
            key: procedure.key(slot),
            procedure_ref: procedure.procedure_ref.clone(),
            revision: procedure.revision,
            slot,
            warrant,
        };
        self.owned.insert(slot, contribution.clone());
        Ok(contribution)
    }

    /// Releases one procedure's contributions (its own key family only).
    /// Authored material and the native subjects are untouched; the released
    /// slots return to their authored base drivers.
    pub fn retire(&mut self, procedure_ref: &str) -> Vec<String> {
        let retired: Vec<String> = self
            .owned
            .values()
            .filter(|c| c.procedure_ref == procedure_ref)
            .map(|c| c.key.clone())
            .collect();
        self.owned.retain(|_, c| c.procedure_ref != procedure_ref);
        retired
    }
}

/// The default and largest firing budget of one binding: how often a bound
/// procedure may fire before it stops, inspectably, instead of running away.
pub const DEFAULT_BINDING_BUDGET: u32 = 16;
pub const MAX_BINDING_BUDGET: u32 = 1024;

/// One procedure bound to the live flow. It fires only when the host itself
/// performs its admitted determinant operation as a request; a firing that is
/// applying or refused still consumes one evaluation of its budget.
#[derive(Debug, Clone, Serialize)]
pub struct StageBinding {
    pub procedure: StageProcedure,
    pub max_evaluations: u32,
    pub evaluations: u32,
    pub standing: &'static str,
}

/// The host's bound procedures, keyed by procedure identity. A rebind
/// replaces its own binding; different procedures fire in canonical
/// (reference-sorted) order, never in arrival order.
#[derive(Debug, Clone, Default)]
pub struct StageBindings {
    bound: BTreeMap<String, StageBinding>,
}

impl StageBindings {
    pub fn get(&self, procedure_ref: &str) -> Option<&StageBinding> {
        self.bound.get(procedure_ref)
    }

    pub fn state(&self) -> Vec<Value> {
        self.bound
            .values()
            .map(|b| {
                let determinant = match &b.procedure.trigger {
                    StageTrigger::Determinant { operation } => operation.clone(),
                    StageTrigger::Invocation => String::new(),
                };
                json!({
                    "procedure_ref": b.procedure.procedure_ref,
                    "revision": b.procedure.revision,
                    "determinant": determinant,
                    "evaluations": b.evaluations,
                    "max_evaluations": b.max_evaluations,
                    "standing": b.standing,
                })
            })
            .collect()
    }

    /// Installs or replaces a binding. Only determinant-triggered procedures
    /// bind; the budget bounds its lifetime on the flow.
    pub fn bind(
        &mut self,
        procedure: StageProcedure,
        max_evaluations: Option<u32>,
    ) -> Result<&StageBinding, String> {
        // Validates that the trigger names an admitted determinant operation;
        // an invocation procedure binds to nothing.
        procedure.trigger.determinant()?;
        let max = max_evaluations.unwrap_or(DEFAULT_BINDING_BUDGET);
        if !(1..=MAX_BINDING_BUDGET).contains(&max) {
            return Err(format!(
                "a binding's evaluation budget holds 1..={MAX_BINDING_BUDGET}"
            ));
        }
        let procedure_ref = procedure.procedure_ref.clone();
        let binding = StageBinding {
            evaluations: 0,
            standing: "active",
            max_evaluations: max,
            procedure,
        };
        self.bound.insert(procedure_ref.clone(), binding);
        Ok(self.bound.get(&procedure_ref).expect("just inserted"))
    }

    /// Removes a binding, returning it with its final standing.
    pub fn unbind(&mut self, procedure_ref: &str) -> Option<StageBinding> {
        self.bound.remove(procedure_ref)
    }

    /// The canonical firing order for one admitted determinant operation:
    /// active bindings only, reference-sorted, budgets unspent.
    pub fn admissions(&self, determinant: &str) -> Vec<String> {
        self.bound
            .values()
            .filter(|b| {
                b.standing == "active"
                    && b.procedure.trigger.determinant().as_deref() == Ok(determinant)
            })
            .map(|b| b.procedure.procedure_ref.clone())
            .collect()
    }

    /// Records one consumed firing; an exhausted binding stops firing.
    pub fn record_fired(&mut self, procedure_ref: &str) {
        if let Some(binding) = self.bound.get_mut(procedure_ref) {
            binding.evaluations += 1;
            if binding.evaluations >= binding.max_evaluations {
                binding.standing = "exhausted";
            }
        }
    }
}

/// The exact native work one evaluation performs, in application order. The
/// host executes a plan through its existing determinant paths — nothing here
/// bypasses the host's admission envelope.
#[derive(Debug, Clone, Serialize)]
pub struct StagePlan {
    pub procedure_ref: String,
    pub revision: u64,
    /// The event with an appended M3 command batch, when the plan changes form.
    pub event: Option<CoupledInput>,
    pub damping: Option<f64>,
    pub clock: Vec<(String, LiftInput)>,
    /// Momentary played excitations of the standing voices. They claim no
    /// slot: a strike is a performance act, not owned state.
    pub strikes: Vec<super::StrikeInput>,
    /// Generated follow-on scenes, each applied by the host as one admitted
    /// M1 form-advance determinant, in passage order.
    pub passage_ticks: u64,
    pub contributions: Vec<Contribution>,
}

/// The claimed record one evaluation returns: what was claimed, on what
/// warrant, against which event basis. This is the honest application
/// boundary — a plan is not an applied state until its host acknowledges it.
pub fn receipt(plan: &StagePlan, event_ref: &str, applied: bool) -> Value {
    json!({
        "schema": STAGE_RECEIPT,
        "procedure_ref": plan.procedure_ref,
        "revision": plan.revision,
        "event_ref": event_ref,
        "applied": applied,
        "contributions": plan.contributions.iter().map(|c| json!({
            "schema": STAGE_CONTRIBUTION,
            "key": c.key, "slot": c.slot, "warrant": c.warrant,
        })).collect::<Vec<_>>(),
        "momentary_acts": plan.strikes.iter().map(|s| json!({
            "schema": STAGE_CONTRIBUTION,
            "key": format!("{}/strike:{}", plan.procedure_ref, s.mode_ref),
            "slot": Value::Null,
            "warrant": json!({"determinant": "the played strike", "mode_ref": s.mode_ref,
                "standing": "momentary performance act; claims no slot and survives no re-evaluation"}),
        })).collect::<Vec<_>>(),
        "generated_scenes": plan.passage_ticks,
        "standing": "claimed ownership and named operations; applied state is the host's own acknowledged field",
    })
}

/// The M3 generation the composer will see when the event is next composed.
/// The scene owner moves every stamp carrying the event identity together
/// (`next_generation`): when the event's own M2 generation is not ahead of the
/// owner's applied generation, a matching M3 identity is bumped with it.
fn composed_generation(event: &CoupledInput, applied_generation: u64) -> u64 {
    let m2 = &event.m2.stamp.identity;
    let m3 = &event.m3.stamp.identity;
    if m2.profile_generation <= applied_generation && m2 == m3 {
        applied_generation + 1
    } else {
        m3.profile_generation
    }
}

fn form_command(
    procedure: &StageProcedure,
    event: &CoupledInput,
    applied_generation: u64,
    cause: String,
    occurrence_unix_ms: u64,
    receipt_unix_ms: u64,
    operations: Vec<M3Operation>,
) -> M3Command {
    M3Command {
        schema: COMMAND_SCHEMA.into(),
        event_ref: event.m1.event_ref.clone(),
        subject_ref: event.m3.subject_ref.clone(),
        expected_generation: composed_generation(event, applied_generation),
        actor_ref: procedure.key("form"),
        cause_ref: cause,
        occurrence_unix_ms,
        receipt_unix_ms,
        operations,
    }
}

/// Evaluates a procedure against the current event, claiming its slots. The
/// M3 change is compiled onto the event's own command batch with the caller's
/// actual times and the owner's applied generation basis (the host supplies
/// both; evaluation itself stays clock-free).
pub fn evaluate(
    procedure: &StageProcedure,
    event: &CoupledInput,
    ownership: &mut StageOwnership,
    applied_generation: u64,
    occurrence_unix_ms: u64,
    receipt_unix_ms: u64,
) -> Result<StagePlan, String> {
    procedure.validate()?;
    if procedure.subject_ref != event.m3.subject_ref {
        return Err(format!(
            "stage procedure addresses subject {}, but the live event belongs to {}",
            procedure.subject_ref, event.m3.subject_ref
        ));
    }
    let mut plan = StagePlan {
        procedure_ref: procedure.procedure_ref.clone(),
        revision: procedure.revision,
        event: None,
        damping: None,
        clock: Vec::new(),
        strikes: Vec::new(),
        passage_ticks: 0,
        contributions: Vec::new(),
    };
    for change in &procedure.changes {
        match change {
            StageChange::Strike {
                mode_ref,
                amplitude,
            } => {
                if mode_ref.is_empty()
                    || mode_ref.len() > 2048
                    || mode_ref.chars().any(|c| c.is_control())
                {
                    return Err(
                        "a played strike names a current scene voice by its mode reference".into(),
                    );
                }
                if amplitude.iter().any(|v| !v.is_finite() || v.abs() > 1.0) {
                    return Err(
                        "a played strike's amplitude is modal metres within the declared material policy (finite, |a| <= 1.0)"
                            .into(),
                    );
                }
                plan.strikes.push(super::StrikeInput {
                    mode_ref: mode_ref.clone(),
                    amplitude: *amplitude,
                });
            }
            StageChange::Form { operations } => {
                if operations.is_empty() || operations.len() > 64 {
                    return Err("an M3 form change carries 1..64 operations".into());
                }
                let mut next = event.clone();
                next.m3_commands.push(form_command(
                    procedure,
                    event,
                    applied_generation,
                    procedure.trigger.cause_ref(),
                    occurrence_unix_ms,
                    receipt_unix_ms,
                    operations.clone(),
                ));
                plan.event = Some(next);
                let warrant = change.warrant();
                plan.contributions
                    .push(ownership.claim(procedure, "form", warrant)?);
            }
            StageChange::Damping { per_second } => {
                if !per_second.is_finite() || !(0.0..=1e6).contains(per_second) {
                    return Err("stage damping must be finite and in 0..1000000 per second".into());
                }
                plan.damping = Some(*per_second);
                let warrant = change.warrant();
                plan.contributions
                    .push(ownership.claim(procedure, "material.damping", warrant)?);
            }
            StageChange::Clock { slot, phase } => {
                let named = clock_slot(slot)
                    .ok_or_else(|| {
                        format!(
                            "a clock change addresses clock.inscription or clock.lensing, not {slot}"
                        )
                    })?
                    .to_owned();
                let turns: u64 = phase
                    .turns
                    .parse()
                    .map_err(|_| "clock phase turns must be a canonical unsigned integer")?;
                if turns.to_string() != phase.turns || phase.half_degrees >= 720 {
                    return Err(
                        "clock phase must carry canonical turns and half_degrees < 720".into(),
                    );
                }
                let steps = turns
                    .checked_mul(720)
                    .and_then(|t| t.checked_add(u64::from(phase.half_degrees)))
                    .ok_or("clock phase exceeds the exact native range")?;
                if steps > crate::m2_engine::MAX_EXACT_JSON_INTEGER {
                    return Err("clock phase exceeds the exact native range".into());
                }
                plan.clock.push((named.to_owned(), phase.clone()));
                let warrant = change.warrant();
                let static_slot: &'static str = clock_slot(&named).expect("named clock slot");
                plan.contributions
                    .push(ownership.claim(procedure, static_slot, warrant)?);
            }
        }
    }
    if let Some(passage) = &procedure.passage {
        plan.passage_ticks = u64::from(passage.scenes - 1);
        plan.contributions
            .push(ownership.claim(procedure, "form", json!({
                "determinant": "the generated passage",
                "through": "one admitted M1 form-advance determinant per generated scene, applied in passage order",
                "effect": format!("{} generated follow-on scenes in the flow", passage.scenes - 1),
                "warrant": "source-defined (spanda ring codon advance; M1 tick; Vimarśā re-read)",
            }))?);
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event() -> CoupledInput {
        serde_json::from_str(include_str!(
            "../../../../fixtures/kernel/scene-default-event-v2.json"
        ))
        .unwrap()
    }

    fn procedure(changes: Vec<StageChange>) -> StageProcedure {
        let selector: Vec<String> = changes
            .iter()
            .filter_map(|c| c.slot().unwrap().map(str::to_owned))
            .collect();
        StageProcedure {
            schema: STAGE_PROCEDURE.into(),
            procedure_ref: "ta-onta:stage:clock-fold".into(),
            revision: 3,
            subject_ref: event().m3.subject_ref,
            trigger: StageTrigger::Invocation,
            selector,
            changes,
            passage: None,
        }
    }

    #[test]
    fn a_form_change_compiles_onto_the_events_own_command_batch() {
        let live = event();
        let generation = live.m3.stamp.identity.profile_generation;
        let procedure = procedure(vec![StageChange::Form {
            operations: vec![M3Operation::SetPose { pose: 3 }],
        }]);
        let mut ownership = StageOwnership::default();
        // The owner's applied generation equals the event's own: the composer
        // moves a matching M3 identity with it, so the command addresses the
        // generation the event will carry when it is next composed.
        let plan = evaluate(&procedure, &live, &mut ownership, generation, 11, 22).unwrap();
        let next = plan.event.expect("form change carries the event");
        assert_eq!(next.m3_commands.len(), 1);
        let command = &next.m3_commands[0];
        assert_eq!(command.schema, COMMAND_SCHEMA);
        assert_eq!(command.event_ref, live.m1.event_ref);
        assert_eq!(command.expected_generation, generation + 1);
        assert_eq!(command.actor_ref, "ta-onta:stage:clock-fold@3/form");
        assert_eq!(command.cause_ref, "ta-onta:invocation");
        assert_eq!(command.occurrence_unix_ms, 11);
        // An owner still behind the event's own generation never bumps it.
        let behind = evaluate(&procedure, &live, &mut ownership, generation - 1, 11, 22).unwrap();
        assert_eq!(
            behind.event.unwrap().m3_commands[0].expected_generation,
            generation
        );
        // The live event itself is untouched by evaluation.
        assert_eq!(live.m3_commands.len(), 0);
        assert_eq!(ownership.contributions().count(), 1);
        assert_eq!(
            ownership.owner_of("form").unwrap().key,
            "ta-onta:stage:clock-fold@3/form"
        );
    }

    #[test]
    fn re_evaluation_replaces_the_own_basis_and_keeps_the_key_stable() {
        let live = event();
        let mut ownership = StageOwnership::default();
        let first = procedure(vec![StageChange::Damping { per_second: 0.5 }]);
        evaluate(&first, &live, &mut ownership, 1, 0, 0).unwrap();
        let revised = StageProcedure {
            revision: 4,
            ..first.clone()
        };
        let plan = evaluate(&revised, &live, &mut ownership, 1, 0, 0).unwrap();
        assert_eq!(plan.damping, Some(0.5));
        assert_eq!(ownership.contributions().count(), 1);
        assert_eq!(
            ownership.owner_of("material.damping").unwrap().key,
            "ta-onta:stage:clock-fold@4/material.damping"
        );
    }

    #[test]
    fn two_procedures_on_one_slot_refuse_arrival_order_arbitration() {
        let live = event();
        let mut ownership = StageOwnership::default();
        let first = StageProcedure {
            procedure_ref: "ta-onta:stage:first".into(),
            ..procedure(vec![StageChange::Damping { per_second: 0.5 }])
        };
        evaluate(&first, &live, &mut ownership, 1, 0, 0).unwrap();
        let second = StageProcedure {
            procedure_ref: "ta-onta:stage:second".into(),
            ..procedure(vec![StageChange::Damping { per_second: 0.9 }])
        };
        let error = evaluate(&second, &live, &mut ownership, 1, 0, 0).unwrap_err();
        assert!(error.contains("owned by ta-onta:stage:first@"), "{error}");
    }

    #[test]
    fn retire_releases_only_the_procedures_own_family() {
        let live = event();
        let mut ownership = StageOwnership::default();
        let first = StageProcedure {
            procedure_ref: "ta-onta:stage:first".into(),
            ..procedure(vec![StageChange::Damping { per_second: 0.5 }])
        };
        let second = StageProcedure {
            procedure_ref: "ta-onta:stage:second".into(),
            ..procedure(vec![StageChange::Form {
                operations: vec![M3Operation::SetPose { pose: 1 }],
            }])
        };
        evaluate(&first, &live, &mut ownership, 1, 0, 0).unwrap();
        evaluate(&second, &live, &mut ownership, 1, 0, 0).unwrap();
        let retired = ownership.retire("ta-onta:stage:first");
        assert_eq!(
            retired,
            vec!["ta-onta:stage:first@3/material.damping".to_owned()]
        );
        assert!(ownership.owner_of("material.damping").is_none());
        assert!(ownership.owner_of("form").is_some());
        // After release the slot can be claimed by another procedure.
        evaluate(&second, &live, &mut ownership, 1, 0, 0).unwrap();
        let third = StageProcedure {
            procedure_ref: "ta-onta:stage:third".into(),
            ..procedure(vec![StageChange::Damping { per_second: 0.2 }])
        };
        evaluate(&third, &live, &mut ownership, 1, 0, 0).unwrap();
    }

    #[test]
    fn evaluation_is_deterministic_in_the_event_basis() {
        let live = event();
        let mut one = StageOwnership::default();
        let mut two = StageOwnership::default();
        let build = || {
            procedure(vec![
                StageChange::Form {
                    operations: vec![
                        M3Operation::CastCreases {
                            angles_deg10: [100, -50, 20],
                            velocities_deg10: [0, 0, 0],
                        },
                        M3Operation::SetPose { pose: 2 },
                    ],
                },
                StageChange::Clock {
                    slot: "clock.lensing".into(),
                    phase: LiftInput {
                        turns: "1".into(),
                        half_degrees: 36,
                    },
                },
                StageChange::Damping { per_second: 0.4 },
            ])
        };
        let a = evaluate(&build(), &live, &mut one, 1, 0, 0).unwrap();
        let b = evaluate(&build(), &live, &mut two, 1, 999, 999).unwrap();
        let strip_times = |plan: &StagePlan| {
            let mut value = serde_json::to_value(plan).unwrap();
            if let Some(event) = value["event"].as_object_mut() {
                for command in event["m3_commands"].as_array_mut().unwrap() {
                    command["occurrence_unix_ms"] = json!(0);
                    command["receipt_unix_ms"] = json!(0);
                }
            }
            value
        };
        assert_eq!(strip_times(&a), strip_times(&b));
        assert_eq!(a.passage_ticks, 0);
    }

    #[test]
    fn a_generated_passage_bounds_its_scenes_and_names_the_law() {
        let live = event();
        let mut procedure = procedure(vec![StageChange::Form {
            operations: vec![M3Operation::SetPose { pose: 5 }],
        }]);
        procedure.passage = Some(StagePassage { scenes: 3 });
        let mut ownership = StageOwnership::default();
        let plan = evaluate(&procedure, &live, &mut ownership, 1, 0, 0).unwrap();
        assert_eq!(plan.passage_ticks, 2);
        assert_eq!(plan.contributions.len(), 2);
        let record = receipt(&plan, &live.m1.event_ref, false);
        assert_eq!(record["generated_scenes"], json!(2));
        assert_eq!(record["applied"], json!(false));
        assert_eq!(record["schema"], STAGE_RECEIPT);
        assert_eq!(
            record["contributions"].as_array().unwrap().len(),
            2,
            "the form claim and the passage claim are distinct contributions"
        );
    }

    #[test]
    fn unknown_slots_foreign_subjects_and_empty_changes_are_refused_by_name() {
        let live = event();
        let mut ownership = StageOwnership::default();
        let voice = procedure(vec![]);
        let voice = StageProcedure {
            selector: vec!["voice:#2-5-4".into()],
            changes: vec![StageChange::Damping { per_second: 0.5 }],
            ..voice
        };
        let error = evaluate(&voice, &live, &mut ownership, 1, 0, 0).unwrap_err();
        assert!(
            error.contains("voice retuning is not an admitted stage change"),
            "{error}"
        );

        let foreign = StageProcedure {
            subject_ref: "person:someone-else".into(),
            ..procedure(vec![StageChange::Damping { per_second: 0.5 }])
        };
        let error = evaluate(&foreign, &live, &mut ownership, 1, 0, 0).unwrap_err();
        assert!(error.contains("but the live event belongs to"), "{error}");

        let wrong_schema = StageProcedure {
            schema: "ql.stage-procedure/v0".into(),
            ..procedure(vec![StageChange::Damping { per_second: 0.5 }])
        };
        assert!(
            evaluate(&wrong_schema, &live, &mut ownership, 1, 0, 0)
                .unwrap_err()
                .contains("unsupported stage procedure contract")
        );

        let selector_mismatch = StageProcedure {
            selector: vec!["form".into()],
            ..procedure(vec![StageChange::Damping { per_second: 0.5 }])
        };
        let error = evaluate(&selector_mismatch, &live, &mut ownership, 1, 0, 0).unwrap_err();
        assert!(
            error.contains("outside the procedure's selector"),
            "{error}"
        );
    }

    #[test]
    fn serde_refuses_unknown_fields_and_anonymous_procedures() {
        let value: Value = serde_json::from_str(
            r#"{"schema":"ql.stage-procedure/v1","procedure_ref":"p","revision":1,
                "subject_ref":"s","trigger":{"trigger":"invocation"},
                "selector":["form"],"changes":[],"surprise":1}"#,
        )
        .unwrap();
        assert!(serde_json::from_value::<StageProcedure>(value).is_err());
        let empty: Value = serde_json::from_str(
            r#"{"schema":"ql.stage-procedure/v1","procedure_ref":"p","revision":1,
                "subject_ref":"s","trigger":{"trigger":"invocation"},
                "selector":["form"],"changes":[]}"#,
        )
        .unwrap();
        let procedure: StageProcedure = serde_json::from_value(empty).unwrap();
        assert_eq!(
            procedure.validate().unwrap_err(),
            "a procedure carries 1..16 changes"
        );
    }

    #[test]
    fn the_stage_subject_is_validated_through_the_pse_subject_owner() {
        let live = event();
        let mut ownership = StageOwnership::default();
        // A malformed subject is refused through the semantic owner's grammar.
        let malformed = StageProcedure {
            subject_ref: "subject\0bad".into(),
            ..procedure(vec![StageChange::Damping { per_second: 0.5 }])
        };
        let error = malformed.validate().unwrap_err();
        assert!(error.contains("PS-E subject owner"), "{error}");
        assert!(evaluate(&malformed, &live, &mut ownership, 1, 0, 0).is_err());
        // The live event's own subject passes the same grammar, and the
        // landed behavior is unchanged for valid subjects.
        crate::coordinate_expression::validate_subject_ref(&live.m3.subject_ref).unwrap();
        assert!(
            evaluate(
                &procedure(vec![StageChange::Damping { per_second: 0.5 }]),
                &live,
                &mut ownership,
                1,
                0,
                0
            )
            .is_ok()
        );
    }

    #[test]
    fn determinate_trigger_carries_its_operation_into_the_cause() {
        let live = event();
        let mut procedure = procedure(vec![StageChange::Form {
            operations: vec![M3Operation::SetPose { pose: 1 }],
        }]);
        procedure.trigger = StageTrigger::Determinant {
            operation: "m1-advance".into(),
        };
        let mut ownership = StageOwnership::default();
        let plan = evaluate(&procedure, &live, &mut ownership, 1, 0, 0).unwrap();
        assert_eq!(
            plan.event.as_ref().unwrap().m3_commands[0].cause_ref,
            "ta-onta:determinant:m1-advance"
        );
    }

    #[test]
    fn clock_phase_is_bounded_and_canonical() {
        let live = event();
        let mut ownership = StageOwnership::default();
        let bad = procedure(vec![StageChange::Clock {
            slot: "clock.inscription".into(),
            phase: LiftInput {
                turns: "01".into(),
                half_degrees: 0,
            },
        }]);
        assert!(
            evaluate(&bad, &live, &mut ownership, 1, 0, 0)
                .unwrap_err()
                .contains("canonical")
        );
        let wide = procedure(vec![StageChange::Clock {
            slot: "clock.lensing".into(),
            phase: LiftInput {
                turns: "0".into(),
                half_degrees: 720,
            },
        }]);
        assert!(
            evaluate(&wide, &live, &mut ownership, 1, 0, 0)
                .unwrap_err()
                .contains("half_degrees")
        );
    }

    #[test]
    fn a_played_strike_is_momentary_and_claims_no_slot() {
        let live = event();
        let mut ownership = StageOwnership::default();
        let mut procedure = procedure(vec![StageChange::Strike {
            mode_ref: "scene:planet/#2-5-4".into(),
            amplitude: [0.9, 0.0],
        }]);
        procedure.selector = Vec::new();
        let plan = evaluate(&procedure, &live, &mut ownership, 1, 0, 0).unwrap();
        assert_eq!(plan.strikes.len(), 1);
        assert_eq!(plan.strikes[0].mode_ref, "scene:planet/#2-5-4");
        assert_eq!(plan.contributions.len(), 0, "a strike claims no slot");
        assert_eq!(ownership.contributions().count(), 0);
        let record = receipt(&plan, &live.m1.event_ref, true);
        let acts = record["momentary_acts"].as_array().unwrap();
        assert_eq!(acts.len(), 1);
        assert_eq!(acts[0]["slot"], Value::Null);
        assert!(
            acts[0]["key"]
                .as_str()
                .unwrap()
                .contains("strike:scene:planet/#2-5-4")
        );
        // Out-of-policy strikes are refused by the stage before any transport.
        let mut loud = procedure.clone();
        loud.changes[0] = StageChange::Strike {
            mode_ref: "scene:planet/#2-5-4".into(),
            amplitude: [1.5, 0.0],
        };
        assert!(
            evaluate(&loud, &live, &mut ownership, 1, 0, 0)
                .unwrap_err()
                .contains("modal metres")
        );
        let mut infinite = procedure;
        infinite.changes[0] = StageChange::Strike {
            mode_ref: "scene:planet/#2-5-4".into(),
            amplitude: [f64::NAN, 0.0],
        };
        assert!(
            evaluate(&infinite, &live, &mut ownership, 1, 0, 0)
                .unwrap_err()
                .contains("modal metres")
        );
    }

    #[test]
    fn bindings_bind_determinant_triggers_only_and_exhaust_inspectably() {
        let mut trigger = StageTrigger::Determinant {
            operation: "m1-advance".into(),
        };
        let mut procedure = procedure(vec![StageChange::Strike {
            mode_ref: "scene:planet/#2-5-4".into(),
            amplitude: [0.4, 0.0],
        }]);
        procedure.selector = Vec::new();
        procedure.trigger = trigger.clone();
        let mut bindings = StageBindings::default();
        bindings.bind(procedure.clone(), Some(2)).unwrap();
        assert!(bindings.bind(procedure.clone(), Some(0)).is_err());
        // An invocation procedure binds to nothing.
        let mut invocation = procedure.clone();
        invocation.trigger = StageTrigger::Invocation;
        assert!(
            bindings
                .bind(invocation, None)
                .unwrap_err()
                .contains("invocation procedure evaluates explicitly")
        );
        // An unknown determinant operation is refused by name.
        trigger = StageTrigger::Determinant {
            operation: "shutdown".into(),
        };
        let mut unknown = procedure.clone();
        unknown.trigger = trigger;
        assert!(
            bindings
                .bind(unknown, None)
                .unwrap_err()
                .contains("admitted")
        );
        // Firing order is canonical and only active bindings admit.
        let mut other = procedure.clone();
        other.procedure_ref = "ta-onta:stage:ahead".into();
        bindings.bind(other, None).unwrap();
        assert_eq!(
            bindings.admissions("m1-advance"),
            vec![
                "ta-onta:stage:ahead".to_owned(),
                "ta-onta:stage:clock-fold".to_owned()
            ]
        );
        bindings.record_fired("ta-onta:stage:clock-fold");
        let binding = bindings.get("ta-onta:stage:clock-fold").unwrap();
        assert_eq!((binding.evaluations, binding.standing), (1, "active"));
        bindings.record_fired("ta-onta:stage:clock-fold");
        assert_eq!(
            bindings.get("ta-onta:stage:clock-fold").unwrap().standing,
            "exhausted"
        );
        // The exhausted binding no longer admits; the other still does.
        assert_eq!(
            bindings.admissions("m1-advance"),
            vec!["ta-onta:stage:ahead".to_owned()]
        );
        let removed = bindings.unbind("ta-onta:stage:clock-fold").unwrap();
        assert_eq!(removed.evaluations, 2);
        assert!(bindings.get("ta-onta:stage:clock-fold").is_none());
    }
}
