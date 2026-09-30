//! M2-5, the sky: the planets, Sun, Earth and chakras with their map values.
//!
//! Read from `fixtures/kernel/m2-sky-v1.json`, the projection of the live Bimba
//! map read at the registry's identities (`scripts/m2-sky.py`). The planetary
//! just octave is the map's own `m_2_5_interval_from_root`; the ratio is parsed
//! from that text, which is kept beside it.

use serde::Deserialize;
use serde_json::Value;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct SkyNode {
    #[serde(rename = "ref")]
    pub reference: String,
    pub id: String,
    pub properties: Value,
    #[serde(default)]
    pub just_ratio: Option<[u16; 2]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sky {
    pub schema: String,
    pub registry_revision: String,
    pub map_content_sha256: String,
    pub nodes: Vec<SkyNode>,
}

pub fn sky() -> &'static Sky {
    static SKY: OnceLock<Sky> = OnceLock::new();
    SKY.get_or_init(|| {
        serde_json::from_str(include_str!("../../../fixtures/kernel/m2-sky-v1.json"))
            .expect("validated M2 sky projection")
    })
}

pub fn node(reference: &str) -> Option<&'static SkyNode> {
    sky().nodes.iter().find(|n| n.reference == reference)
}

/// The planet's just ratio from its root, as the map states it.
pub fn just_ratio(planet_ref: &str) -> Option<[u16; 2]> {
    node(planet_ref).and_then(|n| n.just_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m_tree::native_m_registry;

    #[test]
    fn the_just_octave_is_the_maps_and_sits_on_registry_identities() {
        let s = sky();
        assert_eq!(
            s.registry_revision,
            native_m_registry().manifest().registry_revision
        );
        for (planet, ratio) in [
            ("#2-5-0/1", [1, 1]),
            ("#2-5-2", [9, 8]),
            ("#2-5-3", [5, 4]),
            ("#2-5-4", [4, 3]),
            ("#2-5-5", [3, 2]),
            ("#2-5-6", [5, 3]),
            ("#2-5-7", [15, 8]),
            ("#2-5-8", [9, 5]),
            ("#2-5-9", [9, 4]),
        ] {
            assert_eq!(just_ratio(planet), Some(ratio), "{planet}");
            let id = native_m_registry().resolve(planet).unwrap().id;
            assert_eq!(node(planet).unwrap().id, format!("{:016x}", id.as_u64()));
        }
        assert_eq!(
            just_ratio("#2-5-0/1-0"),
            None,
            "Earth is the observer, not a voice"
        );
    }
}
