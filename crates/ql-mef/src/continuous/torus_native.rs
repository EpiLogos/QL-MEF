//! The Torus for a native audio host (O-I #849; DLW manual §21.2 "The torus is the only generator").
//!
//! A host that sounds the Torus in its own engine takes, from this owner, the prepared field and the law
//! that plays it, and adds nothing of its own: `torus-native` (`ql.torus-native/v1`) hands over the
//! continuation exactly as it stands at the cursor (`ql.continuous-handover/v1`: every mode with its
//! resident state, drive and gain, the samples and their shapes, the clock, the driver and its remainder,
//! the Carrier's drive, the acts waiting for later samples and the receiver's move not yet reached), the
//! played-address law of the tuning in force (`ql.torus-keys/v1`, which `<ql/torus_keys.hpp>` plays
//! allocation-free on the host's audio thread), and what release and silence do. The host continues
//! the very field this owner would have rendered (`ql::ContinuousField(ql::Continuation)`); nothing
//! here moves: the generation, cursor, body and clock stand.
use serde_json::{Value, json};

use super::scene_field::SceneInstrument;
use super::{FieldSession, Result, STRIKE_CAP_METRES};

pub const TORUS_NATIVE: &str = "ql.torus-native/v1";
pub const CONTINUOUS_HANDOVER: &str = "ql.continuous-handover/v1";
pub const TORUS_KEYS: &str = "ql.torus-keys/v1";

/// What a key's release does to the body a native host plays.
pub const TORUS_RELEASE_LAW: &str = "a key's release moves no native state: the body rings on with its own damping, and the owner counts the notes (torus-release); past Polyphony the oldest notes make way in the count, and the body rings on";
/// How the body a native host plays is silenced (`ql::ContinuousField::silence`).
pub const TORUS_SILENCE_LAW: &str = "silence stills what was struck at the cursor: every resident state at rest and the acts waiting for later samples dropped, one body act; the voices with the drive their material supplies, the receiver's gains, the shapes, the clock, the cursor and the Carrier's drive stand, so a driven body is driven again from rest";
/// How long a handed-over law stands.
pub const TORUS_NATIVE_STANDS: &str = "the law stands until the owner's Torus moves (a setting, a restore, the body chosen, the event or its voices): the host takes the Torus again then; a setting sent for a later sample is in force only once it has landed, so the Torus is handed over after it lands";
/// Why a form's body is handed over with no key law (O-I #849 with #376/#852).
pub const TORUS_BODY_KEYS_REFUSED: &str = "a form's body rings, and a key strikes it at the form's sample nearest the key's point (the body's exciter law), which the torus's key law does not play: no key law is handed over; the owner strikes every act on the body and names each body mode's impulse in its receipt (strike_at.modes), which a native host strikes as it stands";
const TORUS_NATIVE_STANDING: &str = "the owner's own continuation and played-address law, handed over at its cursor for a native host to continue; the host adds no voice, pitch, place, weight or envelope";

impl FieldSession {
    /// The continuation as it stands (`handover`, O-I #849): a read at the cursor. The worker's
    /// answer is admitted as a read (nothing moved) and the continuation it carries must agree
    /// with that answer: the same identity, generation, cursor, clock, body and samples.
    pub fn handover(&mut self) -> Result<Value> {
        let request = json!({"schema":"ql.field-control/v1", "operation":"handover",
            "expected_generation":self.receipt["generation"], "expected_samples_elapsed":self.receipt["samples_elapsed"]});
        let mut receipt = self.worker.exchange(&request)?;
        let Some(handed) = receipt
            .as_object_mut()
            .and_then(|object| object.remove("handover"))
        else {
            return Err(self
                .worker
                .invalidate("the worker answered a handover without its continuation"));
        };
        let read = json!({"schema":"ql.field-control/v1", "operation":"read"});
        if let Err(error) = self
            .guard
            .validate(&read, Some(&self.receipt), &receipt)
            .and_then(|()| continuation_agrees(&handed, &receipt))
        {
            return Err(self.worker.invalidate(&error));
        }
        self.guard.adopted(&receipt);
        self.receipt = receipt;
        Ok(handed)
    }
}

/// A handed-over continuation is the field the worker acknowledged with it, or it is refused.
fn continuation_agrees(handed: &Value, receipt: &Value) -> Result<()> {
    let require = |valid: bool, why: &str| {
        if valid {
            Ok(())
        } else {
            Err(format!(
                "the handed-over continuation is not the field: {why}"
            ))
        }
    };
    require(
        handed["schema"] == CONTINUOUS_HANDOVER,
        "unknown handover contract",
    )?;
    for key in [
        "event_ref",
        "subject_ref",
        "geometry_ref",
        "material_ref",
        "model_ref",
        "generation",
        "samples_elapsed",
        "sample_rate",
    ] {
        require(handed[key] == receipt[key], key)?;
    }
    for key in [
        "inscription",
        "lensing",
        "grid_origins",
        "rate_numerators",
        "rate_denominator",
        "rate_remainders",
        "generation",
    ] {
        let (a, b) = (&handed["clock"][key], &receipt["clock"][key]);
        let same = match key {
            // The receipt's phases also carry their double cover; the handover carries what a clock is made from.
            "inscription" | "lensing" => {
                a["turns"] == b["turns"] && a["half_degrees"] == b["half_degrees"]
            }
            _ => a == b,
        };
        require(same && !a.is_null(), "clock")?;
    }
    let modes = handed["modes"]
        .as_array()
        .ok_or("the continuation names no modes")?;
    let body = receipt["amplitudes_metres"]
        .as_array()
        .ok_or("the field names no body")?;
    require(modes.len() == body.len() && !modes.is_empty(), "mode count")?;
    for (mode, state) in modes.iter().zip(body) {
        require(&mode["state_metres"] == state, "a resident state")?;
    }
    let samples = handed["samples"]
        .as_array()
        .ok_or("the continuation names no samples")?;
    let targets = receipt["targets"]
        .as_array()
        .ok_or("the field names no targets")?;
    require(samples.len() == targets.len(), "sample count")?;
    for (sample, target) in samples.iter().zip(targets) {
        require(
            sample["identity"] == target["identity"]
                && sample["constituent"] == target["constituent"]
                && sample["mode_shapes"].as_array().map(Vec::len) == Some(modes.len()),
            "a sample",
        )?;
    }
    require(
        handed["carrier"]["impulses"]
            .as_array()
            .is_some_and(|impulses| impulses.is_empty() || impulses.len() == modes.len()),
        "the Carrier's drive",
    )?;
    require(
        handed["heard_from"]
            .as_array()
            .is_some_and(|gains| gains.is_empty() || gains.len() == modes.len()),
        "the receiver's gains",
    )?;
    require(handed["pending"].is_array(), "the waiting acts")
}

impl SceneInstrument {
    /// The Torus for a native audio host (`torus-native`, `ql.torus-native/v1`): the continuation as it
    /// stands at the cursor and the played-address law of the tuning in force, each voice by the native
    /// mode it sounds through. While a form's body rings, the continuation is the body's modes and the
    /// key law is refused in words (`keys_refused`: a key strikes the form by its exciter law). A read:
    /// nothing moves. Refused in words while a setting waits for a later sample (the law changes there)
    /// or when the tuning in force holds no voices.
    pub fn torus_native(&mut self) -> Result<Value> {
        let reading = self.torus_reading();
        if let Some(first) = reading["waiting"]
            .as_array()
            .and_then(|waiting| waiting.first())
        {
            return Err(format!(
                "a Torus setting waits for sample {}: the law changes there, so the Torus is handed over once it has landed",
                first["at_samples_elapsed"].as_str().unwrap_or("later")
            ));
        }
        if reading["body"]["kind"] == "form" {
            let continuation = self.session_mut().handover_field()?;
            return Ok(self.handed(continuation, Value::Null, "form"));
        }
        let tuning = self.tuning();
        let shape = self.shape();
        let resident: Vec<&str> = shape
            .resident_voices()
            .map(|v| v.mode_ref.as_str())
            .collect();
        let voices = shape
            .group(tuning)
            .iter()
            .map(|voice| {
                let mode = resident
                    .iter()
                    .position(|reference| *reference == voice.mode_ref)
                    .ok_or_else(|| format!("the voice {} is not resident", voice.mode_ref))?;
                Ok(json!({
                    "mode": mode,
                    "mode_ref": voice.mode_ref,
                    "frequency_hz": voice.frequency_hz,
                    "longitude_radians": voice.longitude_radians,
                    "weight": voice.weight,
                    "phase_radians": voice.phase_radians,
                    "m": voice.m,
                    "n": voice.n,
                }))
            })
            .collect::<Result<Vec<_>>>()?;
        if voices.is_empty() {
            return Err(format!(
                "this scene holds no {} voices: the tuning in force has nothing to strike",
                tuning.code()
            ));
        }
        let arcs: Vec<u8> = (0..crate::janko::ROWS)
            .flat_map(|row| {
                (0..crate::janko::COLUMN_PERIOD).map(move |column| {
                    crate::janko::JankoSurface::helix_place(crate::janko::JankoKey::new(
                        row, column,
                    ))
                    .arc
                })
            })
            .collect();
        let torus = self.torus();
        let (anchor_samples, anchor_steps) = torus.anchor();
        let keys = json!({
            "contract": TORUS_KEYS,
            "arcs": arcs,
            "rows": crate::janko::ROWS,
            "column_period": crate::janko::COLUMN_PERIOD,
            "max_keys": crate::m1_prime::POLYPHONY_MAX,
            "voices": voices,
            "tuning": tuning,
            "strike": torus.settings().strike,
            "pulse": {"anchor_samples": anchor_samples.to_string(), "anchor_steps": anchor_steps,
                "steps_per_second": torus.steps_per_second()},
            "sample_rate": self.session().last_field()["sample_rate"],
            "cap_metres": STRIKE_CAP_METRES,
            "law": crate::janko::POINT_LAW,
        });
        let continuation = self.session_mut().handover_field()?;
        for voice in &voices {
            let mode = voice["mode"].as_u64().unwrap_or(u64::MAX) as usize;
            if continuation["modes"].get(mode).map(|m| &m["mode_ref"]) != Some(&voice["mode_ref"]) {
                return Err(format!(
                    "the voice {} does not sound through the native mode the owner holds in its place",
                    voice["mode_ref"]
                ));
            }
        }
        Ok(self.handed(continuation, keys, "torus"))
    }

    /// The `ql.torus-native/v1` document of `continuation` and its key law (`null` for a form's body,
    /// with the refusal in words), at the field as it stands.
    fn handed(&self, continuation: Value, keys: Value, body: &str) -> Value {
        let field = self.session().last_field();
        let mut handed = json!({
            "schema": TORUS_NATIVE,
            "instance_ref": self.instance_ref(),
            "event_ref": field["event_ref"],
            "subject_ref": field["subject_ref"],
            "generation": field["generation"],
            "at_samples_elapsed": field["samples_elapsed"],
            "body": body,
            "continuation": continuation,
            "keys": keys,
            "release": {"law": TORUS_RELEASE_LAW},
            "silence": {"law": TORUS_SILENCE_LAW},
            "stands": TORUS_NATIVE_STANDS,
            "standing": TORUS_NATIVE_STANDING,
        });
        if handed["keys"].is_null() {
            handed["keys_refused"] = json!(TORUS_BODY_KEYS_REFUSED);
        }
        handed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field() -> Value {
        json!({"event_ref":"e","subject_ref":"s","geometry_ref":"g","material_ref":"m","model_ref":"model",
            "generation":"7","samples_elapsed":"1001","sample_rate":48000,
            "clock":{"inscription":{"turns":"0","half_degrees":718,"double_cover_half_degrees":718},
                "lensing":{"turns":"1","half_degrees":17,"double_cover_half_degrees":737},
                "grid_origins":[3,9,21],"rate_numerators":["9","8"],"rate_denominator":8,"rate_remainders":["0","4"],"generation":"4"},
            "amplitudes_metres":[[0.01,0.005],[-0.01,0.0]],
            "targets":[{"identity":1,"constituent":"#3-5-5/0","position":[0.0,0.0,0.0]}]})
    }
    fn handed() -> Value {
        json!({"schema":CONTINUOUS_HANDOVER,"event_ref":"e","subject_ref":"s","geometry_ref":"g","material_ref":"m","model_ref":"model",
            "generation":"7","samples_elapsed":"1001","sample_rate":48000,"driver_remainder":"5",
            "clock":{"inscription":{"turns":"0","half_degrees":718},"lensing":{"turns":"1","half_degrees":17},
                "grid_origins":[3,9,21],"rate_numerators":["9","8"],"rate_denominator":8,"rate_remainders":["0","4"],"generation":"4"},
            "modes":[{"mode_ref":"a","state_metres":[0.01,0.005]},{"mode_ref":"b","state_metres":[-0.01,0.0]}],
            "samples":[{"identity":1,"constituent":"#3-5-5/0","mode_shapes":[[0.0,0.0,1.0],[0.0,0.0,0.0]]}],
            "carrier":{"impulses":[]},"pending":[],"heard_from":[2.0,2.0]})
    }

    /// One way a handover can be broken: what it breaks, and how.
    type Breaking = (&'static str, fn(&mut Value));

    #[test]
    fn a_continuation_is_admitted_only_as_the_field_it_was_handed_over_with() {
        assert_eq!(continuation_agrees(&handed(), &field()), Ok(()));
        let broken: [Breaking; 9] = [
            ("contract", |h| {
                h["schema"] = json!("ql.continuous-handover/v0")
            }),
            ("generation", |h| h["generation"] = json!("8")),
            ("samples_elapsed", |h| h["samples_elapsed"] = json!("1000")),
            ("clock", |h| {
                h["clock"]["lensing"]["half_degrees"] = json!(19)
            }),
            ("a resident state", |h| {
                h["modes"][1]["state_metres"] = json!([-0.01, 1e-300])
            }),
            ("mode count", |h| {
                h["modes"].as_array_mut().unwrap().pop();
            }),
            ("a sample", |h| h["samples"][0]["identity"] = json!(2)),
            ("the Carrier's drive", |h| {
                h["carrier"]["impulses"] = json!([[0.0, 0.0]])
            }),
            ("the receiver's gains", |h| h["heard_from"] = json!([2.0])),
        ];
        for (why, breaking) in broken {
            let mut h = handed();
            breaking(&mut h);
            let refused = continuation_agrees(&h, &field()).unwrap_err();
            assert!(refused.contains(why), "{why}: {refused}");
        }
    }
}
