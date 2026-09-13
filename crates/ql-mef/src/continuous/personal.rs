//! Subject-bound Nara reception inside the existing coupled K8 owner.
//!
//! The personal layer does not advance another clock or start another numerical
//! field. It receives explicitly supplied, source-qualified centre inputs against
//! the `CoupledBasis` currently acknowledged by the one `CoupledFieldSession`.
//! Field mutation and personal reception remain distinct operations because #134
//! owns the receiver inputs; K8 must not infer them from particles or audio.

use std::path::Path;
use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};

use super::coupled::{CoupledBasis, CoupledFieldSession, CoupledInput};
use super::{FieldInput, LiftInput};
use crate::nara::{
    EventBasisRefs, PersonalConstitution, PersonalEventInput, PersonalFieldInstance,
    PersonalFieldState,
};

pub const PERSONAL_SESSION_CONTRACT: &str = "ql.personal-coupled-session/v1";

/// One subject's existing continuous field plus that subject's personal receiver.
/// Additional views must consume returned receipts; they must not open a second
/// instance merely to inspect the same occasion.
pub struct PersonalCoupledSession {
    coupled: CoupledFieldSession,
    personal: PersonalFieldInstance,
}

impl PersonalCoupledSession {
    pub fn open(
        worker: &Path,
        basis: CoupledInput,
        field: FieldInput,
        constitution: PersonalConstitution,
        timeout: Duration,
    ) -> Result<Self, String> {
        if constitution.subject_id != basis.m3.subject_ref
            || constitution.subject_id != field.subject_ref
        {
            return Err("personal constitution, M3 and continuous field must name one subject".into());
        }
        let personal = PersonalFieldInstance::new(constitution)?;
        let coupled = CoupledFieldSession::open(worker, basis, field, timeout)?;
        Ok(Self { coupled, personal })
    }

    pub fn available(&self) -> bool {
        self.coupled.available()
    }

    pub fn original_basis(&self) -> &CoupledBasis {
        self.coupled.original_basis()
    }

    pub fn current_basis(&self) -> &CoupledBasis {
        self.coupled.current_basis()
    }

    pub fn original_field(&self) -> &FieldInput {
        self.coupled.original_field()
    }

    pub fn last_field(&self) -> &Value {
        self.coupled.last_field()
    }

    pub fn personal(&self) -> &PersonalFieldInstance {
        &self.personal
    }

    pub fn current_personal(&self) -> Option<&PersonalFieldState> {
        self.personal.current()
    }

    /// Whether the last personal reception belongs to the exact current world
    /// event/profile generation and subject. A world replacement never silently
    /// relabels an older personal reading as current.
    pub fn personal_is_current(&self) -> Result<bool, String> {
        let Some(personal) = self.personal.current() else {
            return Ok(false);
        };
        let world = EventBasisRefs::from_basis(self.coupled.current_basis())?;
        Ok(personal.event == world && personal.subject_id == world.subject_ref)
    }

    /// Receives the independently supplied seven-centre inputs against the exact
    /// current basis without advancing or replacing the native material field.
    pub fn receive_personal(
        &mut self,
        input: PersonalEventInput,
    ) -> Result<PersonalFieldState, String> {
        self.personal.receive(self.coupled.current_basis(), input)
    }

    pub fn read_field(&mut self) -> Result<Value, String> {
        self.coupled.read_field()
    }

    pub fn advance_field(&mut self, frames: u32, muted: bool) -> Result<Value, String> {
        self.coupled.advance_field(frames, muted)
    }

    pub fn set_axis_field(&mut self, axis: u8, phase: LiftInput) -> Result<Value, String> {
        self.coupled.set_axis_field(axis, phase)
    }

    pub fn replace_field(&mut self, basis: CoupledInput) -> Result<Value, String> {
        self.coupled.replace_field(basis)
    }

    /// Full owner inspection. This is an in-process API, not permission to expose
    /// protected personal state through a companion/browser surface.
    pub fn inspect(&self) -> Result<Value, String> {
        #[derive(Serialize)]
        struct Inspection<'a> {
            schema: &'static str,
            available: bool,
            world: &'a CoupledBasis,
            field: &'a Value,
            personal: Option<&'a PersonalFieldState>,
            personal_current: bool,
            standing: &'static str,
        }
        serde_json::to_value(Inspection {
            schema: PERSONAL_SESSION_CONTRACT,
            available: self.available(),
            world: self.coupled.current_basis(),
            field: self.coupled.last_field(),
            personal: self.personal.current(),
            personal_current: self.personal_is_current()?,
            standing: "one subject-bound K8 runtime; personal receiver input is separately supplied and protected disclosure remains caller policy",
        })
        .map_err(|error| error.to_string())
    }

    /// Compact currentness suitable for control logic without copying the full
    /// personal/source basis into each native field block.
    pub fn currentness(&self) -> Result<Value, String> {
        let world = EventBasisRefs::from_basis(self.coupled.current_basis())?;
        Ok(json!({
            "schema":PERSONAL_SESSION_CONTRACT,
            "available":self.available(),
            "event_ref":world.event_ref,
            "subject_ref":world.subject_ref,
            "profile_generation":world.profile_generation,
            "field_generation":self.coupled.last_field()["generation"],
            "samples_elapsed":self.coupled.last_field()["samples_elapsed"],
            "personal_reception_generation":self.personal.current().map(|state| state.reception_generation),
            "personal_current":self.personal_is_current()?
        }))
    }
}
