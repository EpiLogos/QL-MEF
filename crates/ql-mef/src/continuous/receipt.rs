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
    require(object.len() == names.len() && names.iter().all(|key| object.contains_key(*key)),
        "receipt has missing/unknown fields")
}
fn array(value: &Value, size: usize) -> Result<&Vec<Value>> {
    let values = value.as_array().ok_or("receipt requires an array")?;
    require(values.len() == size, "receipt array dimensions changed")?;
    Ok(values)
}
fn unsigned(value: &Value) -> Result<u64> {
    let text = value.as_str().ok_or("cursor must be an exact decimal string")?;
    let number: u64 = text.parse().map_err(|_| "invalid unsigned cursor")?;
    require(number.to_string() == text, "noncanonical unsigned cursor")?;
    Ok(number)
}
fn signed(value: &Value) -> Result<i64> {
    let text = value.as_str().ok_or("phase must be an exact decimal string")?;
    let number: i64 = text.parse().map_err(|_| "invalid signed phase")?;
    require(number.to_string() == text, "noncanonical signed phase")?;
    Ok(number)
}
fn finite(value: &Value, bound: f64) -> Result<()> {
    require(value.as_f64().is_some_and(|n| n.is_finite() && n.abs() <= bound),
        "receipt contains a nonfinite/out-of-range number")
}
fn successor(value: &Value) -> Result<u64> {
    unsigned(value)?.checked_add(1).ok_or_else(|| "receipt cursor overflow".into())
}
fn phase(value: &Value) -> Result<()> {
    keys(value, &["turns", "half_degrees", "double_cover_half_degrees"])?;
    let turns = signed(&value["turns"])?;
    let half = value["half_degrees"].as_u64().ok_or("invalid phase residue")?;
    require(half < 720, "phase residue outside one turn")?;
    require(value["double_cover_half_degrees"].as_u64() == Some(turns.rem_euclid(2) as u64 * 720 + half),
        "phase winding and double cover disagree")
}
fn clock(value: &Value) -> Result<()> {
    keys(value, &["inscription", "lensing", "grid_origins", "rate_numerators",
        "rate_denominator", "rate_remainders", "generation", "field_ref", "centre_ref"])?;
    require(value["field_ref"] == "#3-0" && value["centre_ref"] == "#3-5-5/0",
        "unrelated native clock field/centre")?;
    phase(&value["inscription"])?;
    phase(&value["lensing"])?;
    unsigned(&value["generation"])?;
    let denominator = value["rate_denominator"].as_u64().ok_or("invalid clock denominator")?;
    require((1..=1_000_000).contains(&denominator), "invalid clock rate bound")?;
    for origin in array(&value["grid_origins"], 3)? {
        require(origin.as_u64().is_some_and(|n| n < 720), "invalid grid origin")?;
    }
    for rate in array(&value["rate_numerators"], 2)? {
        require((-1_000_000..=1_000_000).contains(&signed(rate)?), "invalid clock rate")?;
    }
    for remainder in array(&value["rate_remainders"], 2)? {
        require((0..denominator as i64).contains(&signed(remainder)?), "invalid clock remainder")?;
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
        let modes = m2["resonator"]["modes"].as_array().ok_or("field requires supplied M2 modes")?.len();
        require((1..=4096).contains(&modes) && !field.samples.is_empty()
            && field.samples.len() <= 65536 && field.samples.len() <= 262144 / modes,
            "field transport sample/mode budget exceeded")?;
        let mut initial_clock = serde_json::to_value(&field.clock).map_err(|e| e.to_string())?;
        initial_clock["field_ref"] = json!("#3-0");
        initial_clock["centre_ref"] = json!("#3-5-5/0");
        for axis in ["inscription", "lensing"] {
            let turns = signed(&initial_clock[axis]["turns"])?;
            let half = initial_clock[axis]["half_degrees"].as_u64().ok_or("invalid initial phase")?;
            initial_clock[axis]["double_cover_half_degrees"] = json!(turns.rem_euclid(2) as u64 * 720 + half);
        }
        clock(&initial_clock)?;
        Ok(Self {
            fixed: json!({"schema":FIELD_CONTRACT, "event_ref":m2["identity"]["event_ref"],
                "subject_ref":field.subject_ref,
                "registry_revision":native_current_m_registry().manifest().registry_revision,
                "geometry_ref":m2["resonator"]["geometry_ref"],
                "material_ref":m2["resonator"]["material_ref"],
                "model_ref":m2["resonator"]["material_model_ref"], "sample_rate":field.sample_rate,
                "standing":"computed-supplied-modal-model-not-empirical-material-validation"}),
            m2_identity: m2["identity"].clone(),
            samples: field.samples.iter().map(|s| (s.identity, s.constituent.clone())).collect(),
            modes,
            mode_identities: m2["resonator"]["modes"].as_array().ok_or("missing modes")?.iter()
                .map(|mode| (mode["mode_ref"].clone(), mode["source_coordinate"].clone())).collect(),
            initial_clock,
        })
    }

    pub(super) fn validate(&self, request: &Value, previous: Option<&Value>, value: &Value) -> Result<()> {
        keys(value, &["schema", "event_ref", "subject_ref", "registry_revision", "geometry_ref",
            "material_ref", "model_ref", "sample_rate", "standing", "generation", "samples_elapsed",
            "clock", "amplitudes_metres", "audio", "targets", "presentation_units_per_metre", "m2_identity"])?;
        for (key, expected) in self.fixed.as_object().ok_or("invalid retained identity")? {
            require(&value[key] == expected, "worker replied for another identity/source/standing")?;
        }
        require(value["presentation_units_per_metre"].as_f64() == Some(1.0), "presentation scale changed")?;
        let replacing = request["operation"] == "replace-modes";
        let identity = if replacing { &request["m2"]["identity"] } else { &self.m2_identity };
        if replacing {
            let old_generation = self.m2_identity["profile_generation"].as_u64().ok_or("invalid retained M2 generation")?;
            require(identity["event_ref"] == self.m2_identity["event_ref"]
                && identity["profile_generation"].as_u64().is_some_and(|n| n > old_generation),
                "worker accepted unrelated or stale M2 basis")?;
            let resonator = &request["m2"]["resonator"];
            for (source, retained) in [("geometry_ref", "geometry_ref"), ("material_ref", "material_ref"), ("material_model_ref", "model_ref")] {
                require(resonator[source] == self.fixed[retained], "worker accepted changed material identity")?;
            }
            for (mode, (reference, coordinate)) in array(&resonator["modes"], self.modes)?.iter().zip(&self.mode_identities) {
                require(&mode["mode_ref"] == reference && &mode["source_coordinate"] == coordinate,
                    "worker accepted changed mode identity/order")?;
            }
        }
        require(&value["m2_identity"] == identity, "worker M2 source generation disagrees")?;
        for (target, (id, constituent)) in array(&value["targets"], self.samples.len())?.iter().zip(&self.samples) {
            keys(target, &["identity", "constituent", "position"])?;
            require(target["identity"].as_u64() == Some(*id) && target["constituent"].as_str() == Some(constituent.as_str()),
                "worker changed sample correspondence")?;
            for coordinate in array(&target["position"], 3)? { finite(coordinate, f32::MAX as f64)?; }
        }
        for amplitude in array(&value["amplitudes_metres"], self.modes)? {
            for component in array(amplitude, 2)? { finite(component, 1e12)?; }
        }
        clock(&value["clock"])?;
        let generation = unsigned(&value["generation"])?;
        let elapsed = unsigned(&value["samples_elapsed"])?;
        let advancing = request["operation"] == "advance";
        let frames = if advancing { request["frames"].as_u64().ok_or("invalid frame count")? } else { 0 };
        require(frames <= 8192, "worker accepted excessive advance")?;
        for sample in array(&value["audio"], frames as usize)? {
            finite(sample, f32::MAX as f64)?;
            if request["muted"] == true { require(sample.as_f64() == Some(0.0), "muted output is not silent")?; }
        }
        let Some(previous) = previous else {
            require(request["operation"] == "initialize" && elapsed == 0
                && Some(generation) == identity["profile_generation"].as_u64()
                && value["clock"] == self.initial_clock, "initial worker receipt disagrees with supplied basis")?;
            return Ok(());
        };
        let operation = request["operation"].as_str().ok_or("missing operation")?;
        let changing = replacing || operation == "set-axis";
        require(generation == if changing { successor(&previous["generation"])? } else { unsigned(&previous["generation"])? },
            "worker control generation disagrees with operation")?;
        require(Some(elapsed) == unsigned(&previous["samples_elapsed"])?.checked_add(frames),
            "worker physical cursor disagrees with operation")?;
        for key in ["grid_origins", "rate_numerators", "rate_denominator"] {
            require(value["clock"][key] == previous["clock"][key], "worker changed retained clock trajectory")?;
        }
        match operation {
            "read" | "replace-modes" => {
                require(value["clock"] == previous["clock"], "non-clock operation changed clock")?;
                if !replacing || request["replace_state"] == false {
                    require(value["amplitudes_metres"] == previous["amplitudes_metres"]
                        && value["targets"] == previous["targets"], "ordinary operation reset resident state")?;
                }
            }
            "set-axis" => {
                let axis = request["axis"].as_u64().filter(|n| *n < 2).ok_or("invalid axis")? as usize;
                let names = ["inscription", "lensing"];
                let selected = &value["clock"][names[axis]];
                require(selected["turns"] == request["phase"]["turns"] && selected["half_degrees"] == request["phase"]["half_degrees"]
                    && value["clock"][names[1 - axis]] == previous["clock"][names[1 - axis]]
                    && value["clock"]["rate_remainders"][axis] == "0"
                    && value["clock"]["rate_remainders"][1 - axis] == previous["clock"]["rate_remainders"][1 - axis]
                    && unsigned(&value["clock"]["generation"])? == successor(&previous["clock"]["generation"])?
                    && value["amplitudes_metres"] == previous["amplitudes_metres"], "independent axis acknowledgement disagrees")?;
            }
            "advance" => {
                // Native C owns the exact lifted trajectory. This checks its
                // wire cursor boundary; native numerical parity is tested there.
                let before = unsigned(&previous["clock"]["generation"])?;
                let after = unsigned(&value["clock"]["generation"])?;
                require(after == before || before.checked_add(1) == Some(after), "clock generation skipped/backdated")?;
                if frames == 0 {
                    require(value["clock"] == previous["clock"] && value["targets"] == previous["targets"]
                        && value["amplitudes_metres"] == previous["amplitudes_metres"], "zero advance changed state")?;
                }
            }
            _ => return Err("unknown acknowledged operation".into()),
        }
        Ok(())
    }

    pub(super) fn adopted(&mut self, receipt: &Value) {
        self.m2_identity = receipt["m2_identity"].clone();
    }
}

#[cfg(test)]
mod tests;
