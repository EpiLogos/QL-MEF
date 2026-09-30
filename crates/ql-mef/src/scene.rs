//! One integrated M1–M2–M3 event for the instrument: the dated sky placed on
//! the clock, each planet voiced by the M2 synth, the form advanced by M1, and
//! the Nara centres the sky reaches.
//!
//! - M2-5 is the sky: each body sits at its ecliptic degree (θ = λ) with the
//!   clock's seed there (`scene_sky`, `m3_inscription`).
//! - The M2 synth voices every planet the map tunes at M1's root times its just
//!   ratio (`m2_sky`); retrograde reverses its modulation direction, and its
//!   daily motion sets the modulation rate against the Spanda beat.
//! - M1's ring state at (tick12, cycle) advances the form (`spanda_field`).
//! - Each chakra centre under the Sun (`#2-5-0/1-1..7`) receives the planets
//!   that resonate with it (`PLANETARY_RESONANCE`); Earth `#2-5-0/1-0` observes.
//!
//! Every magnitude that is a choice, not a map fact, is a named tunable (D30).

use crate::continuous::coupled::SKY_ROOT_HZ;
use crate::m2_sky::just_ratio;
use crate::scene_sky::{PlacedBody, place};
use crate::spanda_field::{HkbParams, ring_codon_advance};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const SCENE: &str = "ql.scene/v1";

/// The scene's tunables. Map facts are not here; only choices are (D30).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneTuning {
    /// M1's harmonic ratio scaling the root (the carrier's own ratio).
    pub m1_harmonic_ratio: [u16; 2],
    /// The root the ratios multiply; the octet's C3.
    pub root_hz: f64,
    /// Modulation rate per degree/day of a planet's motion, in Spanda beats.
    /// Default: the Moon's mean daily motion (13.176°/day) is one beat, so the
    /// fastest body pulses at the Spanda beat and the outer planets breathe.
    pub beats_per_degree_per_day: f64,
}

impl Default for SceneTuning {
    fn default() -> Self {
        Self {
            m1_harmonic_ratio: [1, 1],
            root_hz: SKY_ROOT_HZ,
            beats_per_degree_per_day: 1.0 / 13.176,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Voice {
    pub planet_ref: String,
    pub just_ratio: [u16; 2],
    pub frequency_hz: f64,
    /// +1 direct, −1 retrograde: the direction the modulation turns.
    pub direction: i8,
    pub modulation_hz: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SceneBody {
    #[serde(flatten)]
    pub placed: PlacedBody,
    pub voice: Option<Voice>,
}

/// Compose the scene from an accepted sky snapshot and M1's (tick12, cycle).
pub fn compose(
    snapshot: &Value,
    tick12: u8,
    cycle: u64,
    tuning: SceneTuning,
) -> Result<Value, String> {
    let [num, den] = tuning.m1_harmonic_ratio;
    if tick12 >= 12
        || num == 0
        || den == 0
        || !tuning.root_hz.is_finite()
        || tuning.root_hz <= 0.0
        || !tuning.beats_per_degree_per_day.is_finite()
        || tuning.beats_per_degree_per_day < 0.0
    {
        return Err("invalid scene tick or tuning".into());
    }
    let hkb = HkbParams::default();
    let root = tuning.root_hz * f64::from(num) / f64::from(den);
    let placement = place(snapshot)?;
    let bodies: Vec<SceneBody> = placement
        .bodies
        .into_iter()
        .map(|placed| {
            let voice = placed.planet_ref.as_deref().and_then(|planet| {
                just_ratio(planet).map(|ratio| Voice {
                    planet_ref: planet.to_string(),
                    just_ratio: ratio,
                    frequency_hz: root * f64::from(ratio[0]) / f64::from(ratio[1]),
                    direction: if placed.retrograde { -1 } else { 1 },
                    modulation_hz: hkb.base_freq_hz
                        * tuning.beats_per_degree_per_day
                        * placed.speed_deg_per_day.abs(),
                })
            });
            SceneBody { placed, voice }
        })
        .collect();
    let centres: Vec<Value> = (1..=7)
        .map(|i| {
            let centre = format!("#2-5-0/1-{i}");
            let receiving: Vec<&str> = bodies
                .iter()
                .filter(|b| b.placed.resonant_chakra_ref.as_deref() == Some(centre.as_str()))
                .map(|b| b.placed.body.as_str())
                .collect();
            json!({"centre_ref":centre, "anatomy":crate::m2_sky::node(&centre)
                .and_then(|n| n.properties.get("c_2_anatomical_location").cloned()),
                "receiving":receiving})
        })
        .collect();
    let form = ring_codon_advance(tick12, cycle);
    Ok(json!({
        "schema": SCENE,
        "snapshot_ref": placement.snapshot_ref,
        "epoch_utc": placement.epoch_utc,
        "observer_ref": placement.observer_ref,
        "m1": {"tick12": tick12, "cycle": cycle, "hkb": hkb, "root_hz": root},
        "form": {"codon": form.address(), "advanced_by": "spanda_field::ring_codon_advance"},
        "bodies": bodies,
        "centres": centres,
        "tuning": tuning,
        "sources": {
            "sky_revision": crate::m2_sky::sky().map_content_sha256,
            "registry_revision": crate::m2_sky::sky().registry_revision,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sky() -> Value {
        serde_json::from_str(include_str!(
            "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
        ))
        .unwrap()
    }

    #[test]
    fn the_dated_sky_composes_one_scene() {
        let scene = compose(&sky(), 3, 7, SceneTuning::default()).unwrap();
        let bodies = scene["bodies"].as_array().unwrap();
        assert_eq!(bodies.len(), 10);
        let by = |name: &str| bodies.iter().find(|b| b["body"] == name).unwrap();
        // Sun 1:1 at the root; Saturn 3:2, retrograde.
        assert_eq!(by("Sun")["voice"]["frequency_hz"], json!(SKY_ROOT_HZ));
        assert_eq!(by("Saturn")["voice"]["just_ratio"], json!([3, 2]));
        assert_eq!(by("Saturn")["voice"]["direction"], json!(-1));
        // Uranus has no map node, so no voice; it still sits on the clock.
        assert!(by("Uranus")["voice"].is_null());
        assert_eq!(by("Uranus")["seed"]["decan_name"], "Gemini Decan 1");
        // Seven centres; Saturn reaches Muladhara, the Sun Sahasrara.
        let centres = scene["centres"].as_array().unwrap();
        assert_eq!(centres.len(), 7);
        assert_eq!(centres[0]["receiving"], json!(["Saturn"]));
        assert_eq!(centres[6]["receiving"], json!(["Sun"]));
        assert_eq!(
            scene["form"]["codon"],
            json!(ring_codon_advance(3, 7).address())
        );
    }

    #[test]
    fn m1_moves_the_root_and_form_but_not_the_sky() {
        let a = compose(&sky(), 3, 7, SceneTuning::default()).unwrap();
        let tuned = SceneTuning {
            m1_harmonic_ratio: [3, 2],
            ..SceneTuning::default()
        };
        let b = compose(&sky(), 9, 7, tuned).unwrap();
        for (x, y) in a["bodies"]
            .as_array()
            .unwrap()
            .iter()
            .zip(b["bodies"].as_array().unwrap())
        {
            assert_eq!(x["seed"], y["seed"], "M1 does not move the sky");
            if let (Some(f), Some(g)) = (
                x["voice"]["frequency_hz"].as_f64(),
                y["voice"]["frequency_hz"].as_f64(),
            ) {
                assert!(
                    (g / f - 1.5).abs() < 1e-12,
                    "the whole sky transposes with M1's ratio"
                );
            }
        }
        assert_ne!(a["form"], b["form"], "the ring state advances the form");
    }
}
