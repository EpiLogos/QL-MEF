//! The dated sky placed around the M3-5 clock: M2-5 is the sky.
//!
//! Each body of an accepted sky snapshot (`providers/sky`) becomes its `#2-5`
//! coordinate, found by the map's own name for it, sitting on the torus at its
//! ecliptic degree (θ = λ). At that degree the clock's seed is read from the
//! map (`m3_inscription`), and the planet's own typed relations are carried:
//! the chakra it resonates with (`PLANETARY_RESONANCE`) and whether it rules
//! the decan it stands in (`RULED_BY`). Earth, `#2-5-0/1-0`, is the observer at
//! the centre. A body the map does not hold stays visible as unmapped; nothing
//! is invented for it.

use crate::m_tree::{MRegistry, MTreeId, native_m_registry};
use crate::m3_inscription::{InscriptionSeed, seed_at};
use serde::Serialize;
use serde_json::Value;

pub const EARTH_REF: &str = "#2-5-0/1-0";

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlacedBody {
    pub body: String,
    /// The `#2-5` coordinate the map names for this body, if it holds one.
    pub planet_ref: Option<String>,
    pub longitude_deg: f64,
    pub speed_deg_per_day: f64,
    pub retrograde: bool,
    pub seed: InscriptionSeed,
    /// `PLANETARY_RESONANCE` target: the chakra centre under the Sun.
    pub resonant_chakra_ref: Option<String>,
    /// The ruler(s) of the decan it stands in (`RULED_BY`).
    pub decan_rulers: Vec<String>,
    /// The body stands in a decan it rules.
    pub in_own_decan: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SkyPlacement {
    pub snapshot_ref: String,
    pub epoch_utc: String,
    pub observer_ref: &'static str,
    pub bodies: Vec<PlacedBody>,
}

fn planet_by_name(registry: &MRegistry, name: &str) -> Option<MTreeId> {
    registry
        .manifest()
        .nodes
        .iter()
        .find(|n| {
            n.source_ref.starts_with("#2-5-")
                && n.source_ref != EARTH_REF
                && !n.source_ref.starts_with("#2-5-0/1-")
                && n.names.iter().any(|x| x == name)
        })
        .map(|n| n.id)
}

fn targets(registry: &MRegistry, from: MTreeId, kind: &str) -> Vec<String> {
    registry
        .relations_for(from)
        .filter(|r| r.from_id == Some(from) && r.source_kind == kind)
        .filter_map(|r| r.to_ref.clone())
        .collect()
}

/// Place every body of an accepted sky snapshot on the clock.
pub fn place(snapshot: &Value) -> Result<SkyPlacement, String> {
    let registry = native_m_registry();
    let bodies = snapshot["bodies"]
        .as_array()
        .ok_or("sky snapshot carries no bodies")?;
    let mut placed = Vec::with_capacity(bodies.len());
    for b in bodies {
        let body = b["body"].as_str().ok_or("body without a name")?.to_string();
        let longitude_deg = b["longitude_degrees"]
            .as_f64()
            .ok_or("body without a longitude")?;
        let speed_deg_per_day = b["longitude_speed_degrees_per_day"]
            .as_f64()
            .ok_or("body without a speed")?;
        let retrograde = b["retrograde"].as_bool().ok_or("body without retrograde")?;
        if retrograde != (speed_deg_per_day < 0.0) {
            return Err(format!("{body}: retrograde disagrees with its speed"));
        }
        let seed = seed_at(longitude_deg)?;
        let planet = planet_by_name(registry, &body);
        let planet_ref = planet
            .and_then(|id| registry.node(id))
            .map(|n| n.source_ref.clone());
        let resonant_chakra_ref = planet.and_then(|id| {
            targets(registry, id, "PLANETARY_RESONANCE")
                .into_iter()
                .next()
        });
        let decan_id = registry
            .resolve(&seed.decan_ref)
            .ok_or("decan not in registry")?
            .id;
        let decan_rulers = targets(registry, decan_id, "RULED_BY");
        let in_own_decan = planet_ref
            .as_ref()
            .is_some_and(|p| decan_rulers.contains(p));
        placed.push(PlacedBody {
            body,
            planet_ref,
            longitude_deg,
            speed_deg_per_day,
            retrograde,
            seed,
            resonant_chakra_ref,
            decan_rulers,
            in_own_decan,
        });
    }
    Ok(SkyPlacement {
        snapshot_ref: snapshot["snapshot_ref"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        epoch_utc: snapshot["epoch_utc"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        observer_ref: EARTH_REF,
        bodies: placed,
    })
}
