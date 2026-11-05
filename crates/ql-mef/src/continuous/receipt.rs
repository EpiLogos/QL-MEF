//! Admission of management acknowledgements, not another field/clock engine.
//! A protocol name alone cannot establish which subject or operation replied.
//! Numerical evolution remains the installed native owner's responsibility.
use serde_json::{Value, json};

use super::{FIELD_CONTRACT, FieldInput, Result};
use crate::m_tree::native_current_m_registry;

fn require(valid: bool, message: &str) -> Result<()> {
    if valid { Ok(()) } else { Err(message.into()) }
}
fn keys(value: &Value, names: &[&str]) -> Result<()> {
    let object = value.as_object().ok_or("receipt requires an object")?;
    require(
        object.len() == names.len() && names.iter().all(|key| object.contains_key(*key)),
        "receipt has missing/unknown fields",
    )
}
fn array(value: &Value, size: usize) -> Result<&Vec<Value>> {
    let values = value.as_array().ok_or("receipt requires an array")?;
    require(values.len() == size, "receipt array dimensions changed")?;
    Ok(values)
}
fn unsigned(value: &Value) -> Result<u64> {
    let text = value
        .as_str()
        .ok_or("cursor must be an exact decimal string")?;
    let number: u64 = text.parse().map_err(|_| "invalid unsigned cursor")?;
    require(number.to_string() == text, "noncanonical unsigned cursor")?;
    Ok(number)
}
fn signed(value: &Value) -> Result<i64> {
    let text = value
        .as_str()
        .ok_or("phase must be an exact decimal string")?;
    let number: i64 = text.parse().map_err(|_| "invalid signed phase")?;
    require(number.to_string() == text, "noncanonical signed phase")?;
    Ok(number)
}
/// The native bank as the worker reads it (QL-MEF #287): at most 32 slots in slot order, each held or free, its note's id and
/// order (no later than the bank's), and eight voices and eight fading voices, each a mode, a frequency and a finite state.
fn bank(slots: &Value, order: &Value) -> Result<()> {
    let order = unsigned(order)?;
    let slots = slots.as_array().ok_or("the worker reads no bank")?;
    require(slots.len() <= 32, "the bank holds more than 32 slots")?;
    let mut last = None;
    for slot in slots {
        keys(slot, &["slot", "held", "id", "order", "voices", "fading"])?;
        let at = slot["slot"]
            .as_u64()
            .filter(|s| *s < 32)
            .ok_or("a bank slot outside the 32")?;
        require(
            last.is_none_or(|last| last < at),
            "the bank's slots out of order",
        )?;
        last = Some(at);
        require(
            slot["held"].is_boolean(),
            "a bank slot does not say whether it holds a note",
        )?;
        unsigned(&slot["id"])?;
        require(
            (1..=order).contains(&unsigned(&slot["order"])?),
            "a bank slot voiced after the bank's last note",
        )?;
        for lane in ["voices", "fading"] {
            for voice in array(&slot[lane], 8)? {
                keys(voice, &["mode_ref", "frequency_hz", "amplitude"])?;
                require(voice["mode_ref"].is_string(), "a bank voice names no mode")?;
                finite(&voice["frequency_hz"], f64::MAX)?;
                for part in array(&voice["amplitude"], 2)? {
                    finite(part, 1e6)?;
                }
            }
        }
    }
    Ok(())
}
fn finite(value: &Value, bound: f64) -> Result<()> {
    require(
        value
            .as_f64()
            .is_some_and(|n| n.is_finite() && n.abs() <= bound),
        "receipt contains a nonfinite/out-of-range number",
    )
}
fn successor(value: &Value) -> Result<u64> {
    unsigned(value)?
        .checked_add(1)
        .ok_or_else(|| "receipt cursor overflow".into())
}
fn phase(value: &Value) -> Result<()> {
    keys(
        value,
        &["turns", "half_degrees", "double_cover_half_degrees"],
    )?;
    let turns = signed(&value["turns"])?;
    let half = value["half_degrees"]
        .as_u64()
        .ok_or("invalid phase residue")?;
    require(half < 720, "phase residue outside one turn")?;
    require(
        value["double_cover_half_degrees"].as_u64()
            == Some(turns.rem_euclid(2) as u64 * 720 + half),
        "phase winding and double cover disagree",
    )
}
fn clock(value: &Value) -> Result<()> {
    keys(
        value,
        &[
            "inscription",
            "lensing",
            "grid_origins",
            "rate_numerators",
            "rate_denominator",
            "rate_remainders",
            "generation",
            "field_ref",
            "centre_ref",
        ],
    )?;
    require(
        value["field_ref"] == "#3-0" && value["centre_ref"] == "#3-5-5/0",
        "unrelated native clock field/centre",
    )?;
    phase(&value["inscription"])?;
    phase(&value["lensing"])?;
    unsigned(&value["generation"])?;
    let denominator = value["rate_denominator"]
        .as_u64()
        .ok_or("invalid clock denominator")?;
    require(
        (1..=1_000_000).contains(&denominator),
        "invalid clock rate bound",
    )?;
    for origin in array(&value["grid_origins"], 3)? {
        require(
            origin.as_u64().is_some_and(|n| n < 720),
            "invalid grid origin",
        )?;
    }
    for rate in array(&value["rate_numerators"], 2)? {
        require(
            (-1_000_000..=1_000_000).contains(&signed(rate)?),
            "invalid clock rate",
        )?;
    }
    for remainder in array(&value["rate_remainders"], 2)? {
        require(
            (0..denominator as i64).contains(&signed(remainder)?),
            "invalid clock remainder",
        )?;
    }
    Ok(())
}

pub(super) struct ReceiptGuard {
    fixed: Value,
    m2_identity: Value,
    samples: Vec<(u64, String)>,
    modes: usize,
    mode_identities: Vec<(Value, Value)>,
    initial_clock: Value,
}
impl ReceiptGuard {
    pub(super) fn new(m2: &Value, field: &FieldInput) -> Result<Self> {
        let modes = m2["resonator"]["modes"]
            .as_array()
            .ok_or("field requires supplied M2 modes")?
            .len();
        require(
            (1..=4096).contains(&modes)
                && !field.samples.is_empty()
                && field.samples.len() <= 65536
                && field.samples.len() <= 262144 / modes,
            "field transport sample/mode budget exceeded",
        )?;
        let mut initial_clock = serde_json::to_value(&field.clock).map_err(|e| e.to_string())?;
        initial_clock["field_ref"] = json!("#3-0");
        initial_clock["centre_ref"] = json!("#3-5-5/0");
        for axis in ["inscription", "lensing"] {
            let turns = signed(&initial_clock[axis]["turns"])?;
            let half = initial_clock[axis]["half_degrees"]
                .as_u64()
                .ok_or("invalid initial phase")?;
            initial_clock[axis]["double_cover_half_degrees"] =
                json!(turns.rem_euclid(2) as u64 * 720 + half);
        }
        clock(&initial_clock)?;
        Ok(Self {
            fixed: json!({"schema":FIELD_CONTRACT, "event_ref":m2["identity"]["event_ref"],
                "subject_ref":field.subject_ref,
                "registry_revision":native_current_m_registry().manifest().registry_revision,
                "geometry_ref":m2["resonator"]["geometry_ref"],
                "material_ref":m2["resonator"]["material_ref"],
                "model_ref":m2["resonator"]["material_model_ref"], "sample_rate":field.sample_rate,
                "standing":"computed-supplied-modal-model-not-empirical-material-validation",
                // The only retained identity an explicit reshape may move.
                "shape_ref":field.shape_ref.clone().map_or_else(|| m2["resonator"]["geometry_ref"].clone(), Value::from)}),
            m2_identity: m2["identity"].clone(),
            samples: field
                .samples
                .iter()
                .map(|s| (s.identity, s.constituent.clone()))
                .collect(),
            modes,
            mode_identities: m2["resonator"]["modes"]
                .as_array()
                .ok_or("missing modes")?
                .iter()
                .map(|mode| (mode["mode_ref"].clone(), mode["source_coordinate"].clone()))
                .collect(),
            initial_clock,
        })
    }

    /// A body chosen for this continuation (O-I #852) keeps its event, subject
    /// and sample rate: the chosen body's guard must name them as this one does.
    pub(super) fn chooses(&self, next: &ReceiptGuard) -> Result<()> {
        for key in ["event_ref", "subject_ref", "sample_rate"] {
            require(
                self.fixed[key] == next.fixed[key],
                "a body is chosen for the same event, subject and sample rate",
            )?;
        }
        Ok(())
    }

    pub(super) fn validate(
        &self,
        request: &Value,
        previous: Option<&Value>,
        value: &Value,
    ) -> Result<()> {
        keys(
            value,
            &[
                "schema",
                "event_ref",
                "subject_ref",
                "registry_revision",
                "geometry_ref",
                "material_ref",
                "model_ref",
                "sample_rate",
                "standing",
                "generation",
                "samples_elapsed",
                "clock",
                "amplitudes_metres",
                "audio",
                "targets",
                "presentation_units_per_metre",
                "m2_identity",
                "shape_ref",
                "bank",
                "bank_order",
            ],
        )?;
        bank(&value["bank"], &value["bank_order"])?;
        let reshaping = request["operation"] == "replace-shapes";
        for (key, expected) in self.fixed.as_object().ok_or("invalid retained identity")? {
            let expected = if reshaping && key == "shape_ref" {
                &request["shape_ref"]
            } else {
                expected
            };
            require(
                &value[key] == expected,
                "worker replied for another identity/source/standing",
            )?;
        }
        require(
            value["presentation_units_per_metre"].as_f64() == Some(1.0),
            "presentation scale changed",
        )?;
        let replacing = request["operation"] == "replace-modes";
        let identity = if replacing {
            &request["m2"]["identity"]
        } else {
            &self.m2_identity
        };
        if replacing {
            let old_generation = self.m2_identity["profile_generation"]
                .as_u64()
                .ok_or("invalid retained M2 generation")?;
            require(
                identity["event_ref"] == self.m2_identity["event_ref"]
                    && identity["profile_generation"]
                        .as_u64()
                        .is_some_and(|n| n > old_generation),
                "worker accepted unrelated or stale M2 basis",
            )?;
            let resonator = &request["m2"]["resonator"];
            for (source, retained) in [
                ("geometry_ref", "geometry_ref"),
                ("material_ref", "material_ref"),
                ("material_model_ref", "model_ref"),
            ] {
                require(
                    resonator[source] == self.fixed[retained],
                    "worker accepted changed material identity",
                )?;
            }
            for (mode, (reference, coordinate)) in array(&resonator["modes"], self.modes)?
                .iter()
                .zip(&self.mode_identities)
            {
                require(
                    &mode["mode_ref"] == reference && &mode["source_coordinate"] == coordinate,
                    "worker accepted changed mode identity/order",
                )?;
            }
        }
        require(
            &value["m2_identity"] == identity,
            "worker M2 source generation disagrees",
        )?;
        for (target, (id, constituent)) in array(&value["targets"], self.samples.len())?
            .iter()
            .zip(&self.samples)
        {
            keys(target, &["identity", "constituent", "position"])?;
            require(
                target["identity"].as_u64() == Some(*id)
                    && target["constituent"].as_str() == Some(constituent.as_str()),
                "worker changed sample correspondence",
            )?;
            for coordinate in array(&target["position"], 3)? {
                finite(coordinate, f32::MAX as f64)?;
            }
        }
        for amplitude in array(&value["amplitudes_metres"], self.modes)? {
            for component in array(amplitude, 2)? {
                finite(component, 1e12)?;
            }
        }
        clock(&value["clock"])?;
        let generation = unsigned(&value["generation"])?;
        let elapsed = unsigned(&value["samples_elapsed"])?;
        let advancing = request["operation"] == "advance";
        let frames = if advancing {
            request["frames"].as_u64().ok_or("invalid frame count")?
        } else {
            0
        };
        require(frames <= 8192, "worker accepted excessive advance")?;
        for sample in array(&value["audio"], frames as usize)? {
            finite(sample, f32::MAX as f64)?;
            if request["muted"] == true {
                require(sample.as_f64() == Some(0.0), "muted output is not silent")?;
            }
        }
        let Some(previous) = previous else {
            require(
                request["operation"] == "initialize"
                    && elapsed == 0
                    && Some(generation) == identity["profile_generation"].as_u64()
                    && value["clock"] == self.initial_clock,
                "initial worker receipt disagrees with supplied basis",
            )?;
            return Ok(());
        };
        let operation = request["operation"].as_str().ok_or("missing operation")?;
        let changing = replacing
            || reshaping
            || operation == "replace-body"
            || operation == "set-axis"
            || operation == "strike"
            || operation == "voice-notes"
            || operation == "set-state";
        require(
            generation
                == if changing {
                    successor(&previous["generation"])?
                } else {
                    unsigned(&previous["generation"])?
                },
            "worker control generation disagrees with operation",
        )?;
        require(
            Some(elapsed) == unsigned(&previous["samples_elapsed"])?.checked_add(frames),
            "worker physical cursor disagrees with operation",
        )?;
        for key in ["grid_origins", "rate_numerators", "rate_denominator"] {
            require(
                value["clock"][key] == previous["clock"][key],
                "worker changed retained clock trajectory",
            )?;
        }
        match operation {
            "read" | "replace-modes" => {
                require(
                    value["clock"] == previous["clock"],
                    "non-clock operation changed clock",
                )?;
                if !replacing || request["replace_state"] == false {
                    require(
                        value["amplitudes_metres"] == previous["amplitudes_metres"]
                            && value["targets"] == previous["targets"],
                        "ordinary operation reset resident state",
                    )?;
                }
            }
            "replace-shapes" => {
                // Nodal re-reading of the same samples and voices: only the
                // targets may move. The worker must have received a complete basis.
                for sample in array(&request["shapes"], self.samples.len())? {
                    for shape in array(sample, self.modes)? {
                        for coefficient in array(shape, 3)? {
                            finite(coefficient, 1e6)?;
                        }
                    }
                }
                require(
                    value["clock"] == previous["clock"]
                        && value["amplitudes_metres"] == previous["amplitudes_metres"],
                    "shape replacement changed clock or resident state",
                )?;
            }
            "strike" => {
                // A played excitation: resident state and its targets move on the
                // standing continuation; the clock and cursor do not. Each act
                // names a bounded amplitude; the native owner resolves the mode
                // references, so a reply is admitted only for a well-formed act.
                let acts = request["strikes"]
                    .as_array()
                    .ok_or("a strike act requires an array")?;
                require(
                    !acts.is_empty() && acts.len() <= self.modes,
                    "a strike act must name 1..=mode-count modes",
                )?;
                for act in acts {
                    let reference = act["mode_ref"]
                        .as_str()
                        .filter(|r| !r.is_empty() && r.len() <= 2048)
                        .ok_or("invalid strike mode reference")?;
                    require(
                        !reference.chars().any(|c| c < ' ' || c == '\u{7f}'),
                        "invalid strike mode reference",
                    )?;
                    for component in array(&act["amplitude"], 2)? {
                        finite(component, 1e6)?;
                    }
                }
                require(
                    value["clock"] == previous["clock"],
                    "a strike changed the clock",
                )?;
                // A strike at a sample (O-I #809) is admitted now and lands in
                // the render at that sample: one behind the cursor was already
                // rendered, and one ahead of it moves nothing yet.
                if let Some(at) = request.get("at_samples_elapsed") {
                    let at = unsigned(at)?;
                    let cursor = unsigned(&previous["samples_elapsed"])?;
                    require(
                        at >= cursor,
                        "a strike's sample is behind the cursor: that sound is already rendered",
                    )?;
                    if at > cursor {
                        require(
                            value["amplitudes_metres"] == previous["amplitudes_metres"]
                                && value["targets"] == previous["targets"],
                            "a strike moved the body before its sample",
                        )?;
                    }
                }
            }
            "voice-notes" => {
                // Each note its own voices (QL-MEF #287): the bank moves, and the targets with it (the notes are drawn as they
                // are heard); the resident voices and the clock do not. Voiced now, each note stands in the bank; voiced for a
                // later sample, nothing moves yet.
                let notes = request["notes"]
                    .as_array()
                    .filter(|notes| !notes.is_empty() && notes.len() <= 32)
                    .ok_or("an act voices 1 to 32 notes")?;
                require(
                    value["clock"] == previous["clock"]
                        && value["amplitudes_metres"] == previous["amplitudes_metres"],
                    "voicing notes moved the resident body or the clock",
                )?;
                let at = match request.get("at_samples_elapsed") {
                    Some(at) => unsigned(at)?,
                    None => unsigned(&previous["samples_elapsed"])?,
                };
                let cursor = unsigned(&previous["samples_elapsed"])?;
                require(
                    at >= cursor,
                    "a note's sample is behind the cursor: that sound is already rendered",
                )?;
                if at > cursor {
                    require(
                        value["bank"] == previous["bank"]
                            && value["targets"] == previous["targets"],
                        "a note moved the bank before its sample",
                    )?;
                } else {
                    let bank = value["bank"].as_array().ok_or("the worker reads no bank")?;
                    let gone = request["made_way"]
                        .as_array()
                        .ok_or("an act names the notes making way")?;
                    for note in notes.iter().filter(|note| !gone.contains(&note["id"])) {
                        require(
                            bank.iter()
                                .any(|slot| slot["held"] == true && slot["id"] == note["id"]),
                            "a voiced note is not held in the bank",
                        )?;
                    }
                    for id in gone
                        .iter()
                        .filter(|id| !notes.iter().any(|note| note["id"] == **id))
                    {
                        require(
                            !bank
                                .iter()
                                .any(|slot| slot["held"] == true && slot["id"] == *id),
                            "a note that made way is still held in the bank",
                        )?;
                    }
                }
            }
            "set-axis" => {
                let axis = request["axis"]
                    .as_u64()
                    .filter(|n| *n < 2)
                    .ok_or("invalid axis")? as usize;
                let names = ["inscription", "lensing"];
                let selected = &value["clock"][names[axis]];
                require(
                    selected["turns"] == request["phase"]["turns"]
                        && selected["half_degrees"] == request["phase"]["half_degrees"]
                        && value["clock"][names[1 - axis]] == previous["clock"][names[1 - axis]]
                        && value["clock"]["rate_remainders"][axis] == "0"
                        && value["clock"]["rate_remainders"][1 - axis]
                            == previous["clock"]["rate_remainders"][1 - axis]
                        && unsigned(&value["clock"]["generation"])?
                            == successor(&previous["clock"]["generation"])?
                        && value["amplitudes_metres"] == previous["amplitudes_metres"],
                    "independent axis acknowledgement disagrees",
                )?;
            }
            "set-state" => {
                // The body as it rang at a checkpoint, set again (O-I 855): every
                // resident state exactly as asked, one per mode; the clock stays.
                let asked = array(&request["amplitudes"], self.modes)?;
                for state in asked {
                    for component in array(state, 2)? {
                        finite(component, 1e6)?;
                    }
                }
                require(
                    value["clock"] == previous["clock"],
                    "setting the body changed the clock",
                )?;
                // With its bank (QL-MEF #287), the bank set exactly as asked; without, the bank stands.
                match request.get("bank") {
                    Some(asked) => require(
                        value["bank"] == *asked && value["bank_order"] == request["bank_order"],
                        "the bank set is not the bank asked for",
                    )?,
                    None => require(
                        value["bank"] == previous["bank"]
                            && value["bank_order"] == previous["bank_order"],
                        "setting the body moved its bank",
                    )?,
                }
                for (set, asked) in array(&value["amplitudes_metres"], self.modes)?
                    .iter()
                    .zip(asked)
                {
                    for (a, b) in array(set, 2)?.iter().zip(array(asked, 2)?) {
                        require(
                            a.as_f64() == b.as_f64(),
                            "the body set is not the state asked for",
                        )?;
                    }
                }
            }
            "replace-body" => {
                // The body chosen for the continuation (O-I #852): this guard is the
                // chosen body's, so its identity, voices and samples were judged
                // above. Choosing it moves no clock, and its voices ring from the
                // states its frame supplies.
                require(
                    request["m2"]["identity"] == self.m2_identity,
                    "worker chose another body than the frame sent",
                )?;
                require(
                    value["clock"] == previous["clock"],
                    "choosing the body changed the clock",
                )?;
                for (state, mode) in array(&value["amplitudes_metres"], self.modes)?
                    .iter()
                    .zip(array(&request["m2"]["resonator"]["modes"], self.modes)?)
                {
                    for (a, b) in array(state, 2)?.iter().zip(array(&mode["amplitude"], 2)?) {
                        require(
                            a.as_f64() == b.as_f64(),
                            "the chosen body does not ring from its supplied states",
                        )?;
                    }
                }
            }
            "set-carrier" => {
                // The M1′ pulse's drive of the body (the Torus's Carrier): a
                // performance setting, not a body act. The pulse's later steps
                // move the body in the render; setting the drive moves nothing.
                let rate = request["steps_per_second"]
                    .as_f64()
                    .ok_or("invalid carrier rate")?;
                require(rate.is_finite() && rate >= 0.0, "invalid carrier rate")?;
                let anchor = unsigned(&request["anchor_samples"])?;
                require(
                    anchor <= unsigned(&previous["samples_elapsed"])?,
                    "the carrier's pulse is anchored after the cursor",
                )?;
                let phase = request["anchor_steps"]
                    .as_f64()
                    .ok_or("invalid carrier pulse phase")?;
                require(
                    phase.is_finite() && phase >= 0.0,
                    "invalid carrier pulse phase",
                )?;
                for impulse in array(&request["impulses"], self.modes)? {
                    for component in array(impulse, 2)? {
                        finite(component, 1e6)?;
                    }
                }
                require(
                    value["clock"] == previous["clock"]
                        && value["amplitudes_metres"] == previous["amplitudes_metres"]
                        && value["targets"] == previous["targets"],
                    "setting the carrier moved the body",
                )?;
            }
            "set-gains" => {
                // Where the body is heard from (the Torus's receiver, O-I 856): a
                // receiving setting, not a body act. The gains move only what is heard.
                for gain in array(&request["audio_gains"], self.modes)? {
                    finite(gain, 1e6)?;
                }
                require(
                    value["clock"] == previous["clock"]
                        && value["amplitudes_metres"] == previous["amplitudes_metres"]
                        && value["targets"] == previous["targets"],
                    "setting the receiver moved the body",
                )?;
            }
            "advance" => {
                // Native C owns the exact lifted trajectory. This checks its
                // wire cursor boundary; native numerical parity is tested there.
                let before = unsigned(&previous["clock"]["generation"])?;
                let after = unsigned(&value["clock"]["generation"])?;
                require(
                    after == before || before.checked_add(1) == Some(after),
                    "clock generation skipped/backdated",
                )?;
                if frames == 0 {
                    require(
                        value["clock"] == previous["clock"]
                            && value["targets"] == previous["targets"]
                            && value["amplitudes_metres"] == previous["amplitudes_metres"],
                        "zero advance changed state",
                    )?;
                }
            }
            _ => return Err("unknown acknowledged operation".into()),
        }
        Ok(())
    }

    pub(super) fn adopted(&mut self, receipt: &Value) {
        self.m2_identity = receipt["m2_identity"].clone();
        self.fixed["shape_ref"] = receipt["shape_ref"].clone();
    }
}

#[cfg(test)]
mod tests;
