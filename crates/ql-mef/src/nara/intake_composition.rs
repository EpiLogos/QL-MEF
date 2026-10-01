//! Natal-only elemental reading recovered from the original personal identity
//! producer. This does not supply the complete M4 identity integration or invent
//! receiver dynamics. The astronomical input and the symbolic weighting policy
//! retain separate provenance.

use serde_json::{Value, json};

use super::{BioQuaternion, ConsentState, SourceRevision};
use crate::epi_agent::{ElementReading, NaraElementalRequest, nara_elemental_map};
use crate::m2;

const POLICY: &str = "ql.nara-natal-keplerian-dignity-efwa/v1";
const SOURCE_REPOSITORY: &str = "EpiLogos/Epi-Logos-C-Experiments";
const SOURCE_REVISION: &str = "daa660cbc1b8c5da83828698665a753852cb0287";
const SOURCE_PATH: &str = "Body/S/S0/portal-core/src/personal_identity.rs";
const SOURCE_BLOB: &str = "0496c08d8cab700afaca78a493d97a07c774b332";
pub(super) const PLANETS: [&str; 10] = [
    "Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto",
];
const SIGNS: [&str; 12] = [
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
    "Pisces",
];
const ELEMENTS: [&str; 4] = ["Earth", "Fire", "Water", "Air"];
const CENTRES: [&str; 7] = [
    "Muladhara",
    "Svadhisthana",
    "Manipura",
    "Anahata",
    "Vishuddha",
    "Ajna",
    "Sahasrara",
];

/// A conservative partition of existing evidence, not receiver dynamics. Each
/// channel keeps the original denominator, including evidence with no recipient.
pub(super) struct EvidencePartition {
    pub raw: [f64; 4],
    pub centres: [[f64; 4]; 7],
    pub unrouted: [f64; 4],
    pub unresolved: [f64; 4],
    pub input_planet_ids: Vec<usize>,
    pub unrouted_planet_ids: Vec<usize>,
    pub unresolved_planet_ids: Vec<usize>,
}

impl EvidencePartition {
    pub(super) fn reading(self, channel: &str, source: Value) -> Result<Value, String> {
        let vectors = std::iter::once(&self.raw)
            .chain(self.centres.iter())
            .chain([&self.unrouted, &self.unresolved]);
        if vectors.flatten().any(|v| !v.is_finite() || *v < 0.0) {
            return Err("evidence partition requires finite nonnegative magnitudes".into());
        }
        let total: f64 = self.raw.iter().sum();
        if !total.is_finite() {
            return Err("evidence partition denominator overflow".into());
        }
        for element in 0..4 {
            let assigned: f64 = self.centres.iter().map(|v| v[element]).sum::<f64>()
                + self.unrouted[element]
                + self.unresolved[element];
            if !assigned.is_finite() || (assigned - self.raw[element]).abs() > total * 1e-12 {
                return Err("evidence partition does not conserve its original input".into());
            }
        }
        let numeric = |raw: [f64; 4]| {
            let power: f64 = raw.iter().sum();
            json!({
                "raw_efwa": raw, "weighted_power": power,
                "elemental_share_l1": (total > 0.0).then(|| raw.map(|v| v / total)),
                "mass_share_l1": (total > 0.0).then(|| power / total)
            })
        };
        let centres: Vec<Value> = self
            .centres
            .into_iter()
            .enumerate()
            .map(|(ordinal, raw)| {
                let mut reading = numeric(raw);
                reading["ordinal"] = json!(ordinal);
                reading["native_m2_chakra_id"] = json!(ordinal + 1);
                reading["source_coordinate"] = json!(format!("#2-5-0/1-{}", ordinal + 1));
                reading
            })
            .collect();
        let mut unrouted = numeric(self.unrouted);
        unrouted["planet_ids"] = json!(self.unrouted_planet_ids);
        let mut unresolved = numeric(self.unresolved);
        unresolved["planet_ids"] = json!(self.unresolved_planet_ids);
        Ok(json!({
            "schema": "ql.nara-planetary-evidence-partition/v1", "channel": channel,
            "available": total > 0.0, "basis_order": ELEMENTS,
            "denominator": {
                "weighted_total": total, "input_planet_ids": self.input_planet_ids,
                "source_field": "raw_efwa", "normalization": "L1",
                "scope": "all supplied natal placements; production natal input requires all ten planets, including unresolved and unrouted evidence"
            },
            "source": source, "centres": centres,
            "unrouted": unrouted, "unresolved": unresolved,
            "standing": "conservative share of sourced natal evidence for optional presentation; not canonical receiver amplitude, intrinsic chakra orientation, phase, mode or coupling",
            "channel_combination": "separate readings of the same placements; do not add channels as independent evidence",
            "effect_authority_granted": false
        }))
    }
}

fn component(sign: u8) -> usize {
    match sign % 4 {
        0 => 1,
        1 => 0,
        2 => 3,
        _ => 2,
    }
}

fn domicile(planet: usize, sign: u8) -> bool {
    matches!(
        (planet, sign),
        (0, 4)
            | (1, 3)
            | (2, 2)
            | (2, 5)
            | (3, 1)
            | (3, 6)
            | (4, 0)
            | (4, 7)
            | (5, 8)
            | (5, 11)
            | (6, 9)
            | (6, 10)
            | (7, 10)
            | (8, 11)
            | (9, 7)
    )
}

fn exaltation(planet: usize, sign: u8) -> bool {
    matches!(
        (planet, sign),
        (0, 0) | (1, 1) | (2, 5) | (3, 11) | (4, 9) | (5, 3) | (6, 6)
    )
}

fn dignity(planet: usize, sign: u8) -> (&'static str, f64) {
    // Source order is consequential: Mercury in Virgo is domicile, even though
    // the same sign also occurs in the source exaltation table.
    if domicile(planet, sign) {
        ("domicile", 1.20)
    } else if exaltation(planet, sign) {
        ("exaltation", 1.10)
    } else if domicile(planet, (sign + 6) % 12) {
        ("detriment", 0.90)
    } else if exaltation(planet, (sign + 6) % 12) {
        ("fall", 0.85)
    } else {
        ("unmodified", 1.00)
    }
}

fn column(table: &m2::RetainedTable, name: &str) -> Result<usize, String> {
    table
        .columns()
        .iter()
        .position(|column| column == name)
        .ok_or_else(|| format!("native M2 planet table lacks {name}"))
}

fn centre_natal_orientation(
    raw: Option<[f64; 4]>,
    snapshot_ref: &str,
    registry_revision: &str,
    ordinal: usize,
    planet_ids: &[usize],
) -> Result<Value, String> {
    if raw.is_some_and(|values| values.iter().any(|v| !v.is_finite() || *v < 0.0)) {
        return Err("natal centre evidence must be finite and nonnegative".into());
    }
    let available = raw.is_some_and(|values| values.iter().any(|value| *value > 0.0));
    let quaternion = if available {
        let [earth, fire, water, air] = raw.expect("available evidence");
        let contribution = |contribution_strength| {
            Some(ElementReading {
                contribution_strength,
                confidence: None,
            })
        };
        // This is the existing strict native elemental mapper. No permission
        // to actuate is inferred from astronomical source material.
        nara_elemental_map(NaraElementalRequest {
            earth: contribution(earth),
            fire: contribution(fire),
            water: contribution(water),
            air: contribution(air),
            source: SourceRevision {
                source_ref: snapshot_ref.into(),
                revision: snapshot_ref.into(),
                standing_ref: POLICY.into(),
            },
            consent: ConsentState::Withheld,
        })?["normalized_quaternion"]
            .clone()
    } else {
        Value::Null
    };
    Ok(json!({
        "status": if available { "available" } else { "unavailable" },
        "quaternion": quaternion,
        "reason": if available { None } else { Some("no nonzero natal elemental evidence") },
        "derivation": {
            "method": "ql.nara-elemental-reading/v1", "input": "raw_efwa_evidence",
            "normalization": "L2; zero unavailable",
            "mapping": {"w":"Earth", "x":"Fire", "y":"Water", "z":"Air"},
            "snapshot_ref": snapshot_ref, "policy": POLICY,
            "source_revision": SOURCE_REVISION, "source_blob": SOURCE_BLOB,
            "native_m2_registry_revision": registry_revision,
            "native_m2_chakra_id": ordinal + 1, "planet_ids": planet_ids
        },
        "role": "natal-only elemental direction; not complete identity, amplitude, phase or coupling",
        "effect_authority_granted": false
    }))
}

/// Calculate a source-attributed natal reading from the existing provider's
/// `sky.bodies` boundary. The caller remains responsible for admitting the
/// provider receipt. Missing/unknown birth time must not become a zero chart.
pub fn natal_composition(natal: &Value) -> Result<Value, String> {
    let sky = natal
        .get("sky")
        .ok_or("natal reading has no sky snapshot")?;
    if sky.get("schema").and_then(Value::as_str) != Some("ql.sky-snapshot/v1") {
        return Err("natal composition requires a ql.sky-snapshot/v1 calculation".into());
    }
    let snapshot_ref = sky
        .get("snapshot_ref")
        .and_then(Value::as_str)
        .filter(|reference| !reference.trim().is_empty())
        .ok_or("natal sky calculation has no snapshot reference")?;
    let bodies = sky
        .get("bodies")
        .and_then(Value::as_array)
        .ok_or("natal sky calculation has no body array")?;
    if bodies.len() != PLANETS.len() {
        return Err("natal composition requires all ten planetary bodies".into());
    }
    let mut positions = [None; 10];
    for body in bodies {
        let id = body
            .get("native_planet_id")
            .and_then(Value::as_u64)
            .filter(|id| *id < PLANETS.len() as u64)
            .ok_or("invalid native natal planet identity")? as usize;
        if body.get("body").and_then(Value::as_str) != Some(PLANETS[id]) {
            return Err(format!("natal planet name disagrees with native ID {id}"));
        }
        let degree = body
            .get("longitude_degrees")
            .and_then(Value::as_f64)
            .filter(|degree| degree.is_finite() && (0.0..360.0).contains(degree))
            .ok_or_else(|| format!("invalid absolute natal longitude for {}", PLANETS[id]))?;
        if positions[id].replace(degree).is_some() {
            return Err(format!("duplicate natal planet: {}", PLANETS[id]));
        }
    }

    let catalogue = m2::catalogue();
    let table = catalogue.table("planet")?;
    let id_column = column(table, "id")?;
    let velocity_column = column(table, "keplerian_vel")?;
    let signature_column = column(table, "elem_sig")?;
    let frequency_column = column(table, "cousto_freq")?;
    let mut raw = [0.0; 4];
    let mut centre_raw = [[0.0; 4]; 7];
    let mut centre_planets: [Vec<usize>; 7] = std::array::from_fn(|_| Vec::new());
    let mut unrouted_raw = [0.0; 4];
    let mut unrouted_planet_ids = Vec::new();
    let mut contributions = Vec::with_capacity(10);
    let mut decanic_inputs = Vec::with_capacity(10);
    for (id, position) in positions.into_iter().enumerate() {
        let longitude = position.ok_or_else(|| format!("missing natal planet: {}", PLANETS[id]))?;
        let row = table.row(id)?;
        if row[id_column] != id as u64 {
            return Err("native M2 planet identity order changed".into());
        }
        let signature = u8::try_from(row[signature_column])
            .map_err(|_| "native M2 elemental signature exceeds one byte")?;
        let [native_element, retained_chakra, native_phase] = m2::unpack_signature(signature)?;
        let base_weight = row[velocity_column] as f64;
        if base_weight <= 0.0 {
            return Err("native M2 planetary weight must be positive".into());
        }
        let sign = (longitude / 30.0).floor() as u8;
        let element = component(sign);
        let (dignity_name, multiplier) = dignity(id, sign);
        let weight = base_weight * multiplier;
        decanic_inputs.push(super::decanic_identity::NatalPlacement {
            planet_id: id,
            longitude_degrees: longitude,
            weighted_contribution: weight,
        });
        let mut planetary_raw = [0.0; 4];
        planetary_raw[element] = weight;
        raw[element] += weight;

        // The typed graph is the routing authority. The retained elem_sig is
        // historical evidence, not permission to replace canonical relations.
        let route = m2::planet_chakra_route(id)?;
        let ordinal = if let Some(route) = &route {
            let ordinal = usize::from(route.chakra_index - 1);
            centre_planets[ordinal].push(id);
            centre_raw[ordinal][element] += weight;
            Some(ordinal)
        } else {
            unrouted_raw[element] += weight;
            unrouted_planet_ids.push(id);
            None
        };
        contributions.push(json!({
            "native_planet_id": id, "body": PLANETS[id],
            "longitude_degrees": longitude, "sign_index": sign,
            "sign": SIGNS[usize::from(sign)], "element": ELEMENTS[element],
            "efwa_component": element, "keplerian_weight": base_weight,
            "keplerian_source_units": "retained M2 arcsec/day x 10; model coefficient, not current measured speed",
            "dignity": dignity_name, "dignity_multiplier": multiplier,
            "weighted_contribution": weight, "raw_efwa": planetary_raw,
            "m2_source_coordinate": table.binding(id),
            "native_m2_element_id": native_element,
            "native_m2_phase_code": native_phase,
            "native_m2_chakra_id": route.as_ref().map(|r| r.chakra_index),
            "planetary_chakra_route": route,
            "retained_m2_chakra_id": retained_chakra,
            "native_cousto_frequency_hz": row[frequency_column],
            "receiving_centre_ordinal": ordinal,
            "role": if ordinal.is_some() { "graph-routed-planetary-evidence" } else { "global-natal-evidence; canonical-centre-route-unavailable" }
        }));
    }
    let total = raw.iter().sum::<f64>();
    if !total.is_finite() || total <= f64::EPSILON {
        return Err("natal elemental contribution has no finite positive total".into());
    }
    let balance = raw.map(|value| value / total);
    let q_natal = BioQuaternion {
        w: raw[0],
        x: raw[1],
        y: raw[2],
        z: raw[3],
    }
    .normalized()?;
    let centres: Vec<Value> = (0..7)
        .map(|ordinal| {
            let body = super::domain::operations::body::centre_body(ordinal as u8)?;
            let natal_orientation = centre_natal_orientation(
                Some(centre_raw[ordinal]),
                snapshot_ref,
                catalogue.registry_revision(),
                ordinal,
                &centre_planets[ordinal],
            )?;
            Ok(json!({
                "ordinal": ordinal, "native_m2_chakra_id": ordinal + 1,
                "label": CENTRES[ordinal], "planet_ids": centre_planets[ordinal],
                "raw_efwa_evidence": centre_raw[ordinal],
                "body": body,
                "natal_orientation": natal_orientation,
                "standing": "canonical M2 planetary resonance membership and normalized natal direction; receiver dynamics not derived"
            }))
        })
        .collect::<Result<_, String>>()?;
    let decanic_channel = super::decanic_identity::derive(&decanic_inputs, snapshot_ref, POLICY)?;
    let presentation_partition = EvidencePartition {
        raw, centres: centre_raw, unrouted: unrouted_raw, unresolved: [0.0; 4],
        input_planet_ids: (0..PLANETS.len()).collect(),
        unrouted_planet_ids, unresolved_planet_ids: Vec::new(),
    }.reading("direct-planetary-resonance", json!({
        "snapshot_ref": snapshot_ref, "weighting_policy": POLICY,
        "weighting_source": {"repository": SOURCE_REPOSITORY, "revision": SOURCE_REVISION,
            "path": SOURCE_PATH, "blob": SOURCE_BLOB},
        "registry_revision": catalogue.registry_revision(),
        "route": "planetary_contributions[].planetary_chakra_route",
        "arithmetic": "centre raw EFWA divided by the same all-planet total as elemental_balance_l1; unavailable routes retain their mass"
    }))?;
    Ok(json!({
        "schema": "ql.nara-natal-composition/v1", "policy": POLICY,
        "policy_source": {
            "repository": SOURCE_REPOSITORY, "revision": SOURCE_REVISION,
            "path": SOURCE_PATH, "blob": SOURCE_BLOB,
            "derivation_lines": [230, 302], "normalization_lines": [86, 135],
            "standing": "recovered symbolic natal weighting policy; not astronomical measurement"
        },
        "native_m2_basis": {
            "registry_revision": catalogue.registry_revision(),
            "sources": catalogue.sources(), "table": table.source_symbol()
        },
        "snapshot_ref": snapshot_ref, "time_resolution": natal.get("time_resolution"),
        "provider_basis": sky.get("provider"),
        "basis_order": ELEMENTS, "planetary_contributions": contributions,
        "raw_efwa": raw, "elemental_balance_l1": balance, "q_natal": q_natal,
        "quaternion_role": "natal-only; not the complete integrated Q_identity",
        "normalization": {"elemental_balance": "L1", "q_natal": "L2; zero refused"},
        "centre_evidence": centres,
        "presentation_partition": presentation_partition,
        "decanic_channel": decanic_channel,
        "earth_body": {"role": "distinct grounding anchor; not an eighth chakra or planetary-array entry"},
        "dynamics_standing": "centre natal elemental directions are derived; full identity, receiver gain, oscillator phase, modes and pairwise coupling are not supplied by this reading",
        "private": true, "public_export": false
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn controlled_input(degrees: [f64; 10]) -> Value {
        json!({"sky": {
            "schema": "ql.sky-snapshot/v1", "snapshot_ref": "controlled:natal-calculation-input",
            "bodies": PLANETS.iter().enumerate().map(|(id, name)| json!({
                "native_planet_id": id, "body": name, "longitude_degrees": degrees[id]
            })).collect::<Vec<_>>()
        }})
    }

    fn values(reading: &Value, key: &str) -> Vec<f64> {
        reading[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect()
    }

    #[test]
    fn actual_weighted_arithmetic_and_two_normalizations_remain_distinct() {
        // Sun at neutral Taurus: 35999 Earth; Moon domicile Cancer: 56724 Water;
        // Mercury domicile Gemini: 17686.8 Air. The remaining seven have neutral
        // air-sign placements: 5982 Air. No planet contributes Fire.
        let reading = natal_composition(&controlled_input([
            45.0, 105.0, 75.0, 75.0, 75.0, 195.0, 75.0, 75.0, 75.0, 75.0,
        ]))
        .unwrap();
        let raw = values(&reading, "raw_efwa");
        for (actual, expected) in raw.iter().zip([35999.0, 0.0, 56724.0, 23668.8]) {
            assert!((actual - expected).abs() < 1e-8);
        }
        let balance = values(&reading, "elemental_balance_l1");
        assert!((balance.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        let q: BioQuaternion = serde_json::from_value(reading["q_natal"].clone()).unwrap();
        assert!((q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z - 1.0).abs() < 1e-12);
        assert!((q.w / q.y - 35999.0 / 56724.0).abs() < 1e-12);
        assert_ne!(q.w, balance[0]);
        assert!(reading.get("q_identity").is_none());
    }

    #[test]
    fn source_dignity_precedence_and_opposites_are_preserved() {
        assert_eq!(dignity(2, 5), ("domicile", 1.2));
        assert_eq!(dignity(0, 0), ("exaltation", 1.1));
        assert_eq!(dignity(0, 10), ("detriment", 0.9));
        assert_eq!(dignity(0, 6), ("fall", 0.85));
        assert_eq!(dignity(0, 1), ("unmodified", 1.0));
        assert_eq!(dignity(2, 11), ("detriment", 0.9));
    }

    #[test]
    fn presentation_partition_preserves_outer_mass_and_original_elemental_balance() {
        let reading = natal_composition(&controlled_input([
            45.0, 105.0, 75.0, 75.0, 75.0, 195.0, 75.0, 75.0, 75.0, 75.0,
        ]))
        .unwrap();
        let partition = &reading["presentation_partition"];
        let total = partition["denominator"]["weighted_total"].as_f64().unwrap();
        assert!((total - (35999.0 + 56724.0 + 23668.8)).abs() < 1e-8);
        assert_eq!(partition["unrouted"]["planet_ids"], json!([7]));
        // Uranus has no accepted graph recipient and retains its own mass42.
        // Qualified Neptune21→Ajna and Pluto14→crown still share this denominator.
        assert_eq!(partition["unrouted"]["weighted_power"], 42.0);
        for (id, ordinal, weight) in [(8, 5, 21.0), (9, 6, 14.0)] {
            assert_eq!(
                reading["planetary_contributions"][id]["receiving_centre_ordinal"],
                ordinal
            );
            assert_eq!(
                reading["planetary_contributions"][id]["weighted_contribution"],
                weight
            );
            assert!(
                partition["centres"][ordinal]["raw_efwa"][3]
                    .as_f64()
                    .unwrap()
                    >= weight
            );
        }
        assert_eq!(partition["unresolved"]["weighted_power"], 0.0);
        let mut mass = partition["unrouted"]["mass_share_l1"].as_f64().unwrap();
        let mut reconstructed = [0.0; 4];
        for row in partition["centres"]
            .as_array()
            .unwrap()
            .iter()
            .chain([&partition["unrouted"], &partition["unresolved"]])
        {
            for (i, value) in reconstructed.iter_mut().enumerate() {
                *value += row["elemental_share_l1"][i].as_f64().unwrap();
            }
        }
        for centre in partition["centres"].as_array().unwrap() {
            mass += centre["mass_share_l1"].as_f64().unwrap();
        }
        assert!((mass - 1.0).abs() < 1e-12);
        for (i, value) in reconstructed.iter().enumerate() {
            assert!((value - reading["elemental_balance_l1"][i].as_f64().unwrap()).abs() < 1e-12);
        }
    }

    #[test]
    fn presentation_partition_refuses_nonfinite_or_lost_evidence() {
        let input = |raw, centres| EvidencePartition {
            raw,
            centres,
            unrouted: [0.0; 4],
            unresolved: [0.0; 4],
            input_planet_ids: vec![0],
            unrouted_planet_ids: vec![],
            unresolved_planet_ids: vec![],
        };
        assert!(
            input([1.0, 0.0, 0.0, 0.0], [[0.0; 4]; 7])
                .reading("test", json!({}))
                .is_err()
        );
        assert!(
            input([f64::MAX; 4], [[0.0; 4]; 7])
                .reading("test", json!({}))
                .is_err()
        );
        assert!(
            input([f64::NAN; 4], [[0.0; 4]; 7])
                .reading("test", json!({}))
                .is_err()
        );
    }

    #[test]
    fn reordered_input_and_retrograde_do_not_change_the_source_calculation() {
        let mut input = controlled_input([
            15.0, 45.0, 75.0, 105.0, 135.0, 165.0, 195.0, 225.0, 255.0, 285.0,
        ]);
        let expected = natal_composition(&input).unwrap();
        let bodies = input["sky"]["bodies"].as_array_mut().unwrap();
        bodies.reverse();
        for body in bodies {
            body["retrograde"] = json!(true);
        }
        assert_eq!(natal_composition(&input).unwrap(), expected);
    }

    #[test]
    fn canonical_graph_partitions_include_sun_and_keep_earth_and_outer_planets_distinct() {
        let reading = natal_composition(&controlled_input([15.0; 10])).unwrap();
        let expected = [
            vec![6],
            vec![5],
            vec![4],
            vec![3],
            vec![2],
            vec![1, 8],
            vec![0, 9],
        ];
        for (ordinal, planets) in expected.into_iter().enumerate() {
            let centre = &reading["centre_evidence"][ordinal];
            assert_eq!(centre["planet_ids"], json!(planets));
            assert_eq!(centre["native_m2_chakra_id"], json!(ordinal + 1));
            let direction = &centre["natal_orientation"];
            assert_eq!(direction["status"], "available");
            assert_eq!(
                direction["quaternion"],
                json!({"w":0.0,"x":1.0,"y":0.0,"z":0.0})
            );
            assert_eq!(direction["derivation"]["planet_ids"], centre["planet_ids"]);
            assert_eq!(direction["derivation"]["native_m2_chakra_id"], ordinal + 1);
            assert_eq!(
                direction["derivation"]["snapshot_ref"],
                reading["snapshot_ref"]
            );
            assert_eq!(direction["effect_authority_granted"], false);
            assert!(centre.get("orientation").is_none());
            assert!(centre.get("amplitude").is_none());
        }
        assert_eq!(
            reading["planetary_contributions"][0]["receiving_centre_ordinal"],
            6
        );
        assert!(reading["planetary_contributions"][7]["receiving_centre_ordinal"].is_null());
        assert!(reading["planetary_contributions"][7]["planetary_chakra_route"].is_null());
        for (id, planet_ref, centre_ref, ordinal) in [
            (8, "#2-5-8", "#2-5-0/1-6", 5),
            (9, "#2-5-9", "#2-5-0/1-7", 6),
        ] {
            let contribution = &reading["planetary_contributions"][id];
            assert_eq!(contribution["receiving_centre_ordinal"], ordinal);
            let route = &contribution["planetary_chakra_route"];
            assert_eq!(route["planet_coordinate"], planet_ref);
            assert_eq!(route["chakra_coordinate"], centre_ref);
            let relations = route["relations"].as_array().unwrap();
            assert!(!relations.is_empty());
            // #254 D10 admitted this qualified source edge, not direct resonance.
            assert!(
                relations
                    .iter()
                    .all(|r| r["source_kind"] == "HAS_CHAKRAL_ANCHOR"
                        && r["from_ref"] == planet_ref
                        && r["to_ref"] == centre_ref
                        && r["orientation"] == "directed")
            );
        }
        for id in 7..10 {
            assert!(
                reading["planetary_contributions"][id]["weighted_contribution"]
                    .as_f64()
                    .unwrap()
                    > 0.0
            );
        }
    }

    #[test]
    fn centre_direction_uses_actual_evidence_ratios_without_inventing_strength() {
        let reading = natal_composition(&controlled_input([
            45.0, 105.0, 75.0, 75.0, 75.0, 195.0, 30.0, 75.0, 75.0, 75.0,
        ]))
        .unwrap();
        for centre in reading["centre_evidence"].as_array().unwrap() {
            let raw = values(centre, "raw_efwa_evidence");
            let norm = raw.iter().map(|x| x * x).sum::<f64>().sqrt();
            let q: BioQuaternion =
                serde_json::from_value(centre["natal_orientation"]["quaternion"].clone()).unwrap();
            for (actual, raw) in [q.w, q.x, q.y, q.z].iter().zip(raw) {
                assert!((actual - raw / norm).abs() < 1e-12);
            }
            assert!(centre.get("gain").is_none());
            assert!(centre.get("q_identity").is_none());
        }
        let root = &reading["centre_evidence"][0]["natal_orientation"]["quaternion"];
        assert!(root["w"].as_f64().unwrap() > 0.0);
        assert_eq!(root["z"], 0.0);
    }

    #[test]
    fn all_centre_body_properties_retain_actual_compiler_provenance() {
        let reading = natal_composition(&controlled_input([15.0; 10])).unwrap();
        let registry = crate::m_tree::native_current_m_registry();
        let manifest = registry.manifest();
        let mut properties = std::collections::BTreeSet::new();
        for ordinal in 0..7 {
            let body = &reading["centre_evidence"][ordinal]["body"];
            let zone = &body["body_zone"];
            assert_eq!(body["ordinal"], ordinal);
            assert_eq!(zone["source_ref"], format!("#2-5-0/1-{}", ordinal + 1));
            assert_eq!(zone["registry_revision"], manifest.registry_revision);
            let node = registry
                .resolve(zone["source_ref"].as_str().unwrap())
                .unwrap();
            let record = node
                .records
                .iter()
                .map(|index| &manifest.records[*index])
                .find(|record| {
                    zone["payload_sha256"] == record.payload_sha256
                        && zone["record_index"] == record.record_index
                })
                .unwrap();
            let file = &manifest.files[record.file];
            assert_eq!(zone["git_blob"], file.git_blob);
            assert_eq!(zone["path"], file.path);
            assert!(
                record
                    .property_keys
                    .iter()
                    .any(|key| key == "c_2_anatomical_location")
            );
            let property = zone["property_ref"].as_str().unwrap();
            assert!(property.ends_with(&format!(
                "#/{}/c_2_anatomical_location",
                record.record_index
            )));
            assert!(properties.insert(property));
            assert!(!zone["anatomical_location"].as_str().unwrap().is_empty());
            assert_eq!(body["sense_refs"], json!([]));
            assert_eq!(body["action_refs"], json!([]));
        }
        assert_eq!(properties.len(), 7);
    }

    #[test]
    fn missing_or_zero_centre_evidence_never_becomes_neutral_orientation() {
        for raw in [None, Some([0.0; 4])] {
            let direction = centre_natal_orientation(raw, "source", "revision", 0, &[]).unwrap();
            assert_eq!(direction["status"], "unavailable");
            assert!(direction["quaternion"].is_null());
        }
        for value in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(
                centre_natal_orientation(
                    Some([value, 0.0, 0.0, 0.0]),
                    "source",
                    "revision",
                    0,
                    &[]
                )
                .is_err()
            );
        }
    }

    #[test]
    fn incomplete_duplicated_misidentified_or_invalid_planets_are_refused() {
        let base = controlled_input([15.0; 10]);
        let mut missing = base.clone();
        missing["sky"]["bodies"].as_array_mut().unwrap().pop();
        assert!(natal_composition(&missing).is_err());
        let mut duplicate = base.clone();
        duplicate["sky"]["bodies"][9] = duplicate["sky"]["bodies"][0].clone();
        assert!(
            natal_composition(&duplicate)
                .unwrap_err()
                .contains("duplicate")
        );
        for bad in [
            json!(360.0),
            json!(-0.001),
            json!(null),
            json!("15"),
            json!(1e300),
        ] {
            let mut invalid = base.clone();
            invalid["sky"]["bodies"][0]["longitude_degrees"] = bad;
            assert!(natal_composition(&invalid).is_err());
        }
        let mut wrong_name = base.clone();
        wrong_name["sky"]["bodies"][0]["body"] = json!("Moon");
        assert!(natal_composition(&wrong_name).is_err());
        for bad_id in [json!(-1), json!(10), json!(1.5), json!("0")] {
            let mut invalid = base.clone();
            invalid["sky"]["bodies"][0]["native_planet_id"] = bad_id;
            assert!(natal_composition(&invalid).is_err());
        }
        assert!(natal_composition(&json!({"sky":null})).is_err());
    }

    #[test]
    fn exact_sign_boundaries_change_only_the_affected_evidence() {
        let mut input = controlled_input([29.999; 10]);
        let before = natal_composition(&input).unwrap();
        input["sky"]["bodies"][6]["longitude_degrees"] = json!(30.0);
        let after = natal_composition(&input).unwrap();
        assert_eq!(before["planetary_contributions"][6]["element"], "Fire");
        assert_eq!(after["planetary_contributions"][6]["element"], "Earth");
        assert_ne!(before["centre_evidence"][0], after["centre_evidence"][0]);
        assert_ne!(
            before["centre_evidence"][0]["natal_orientation"]["quaternion"],
            after["centre_evidence"][0]["natal_orientation"]["quaternion"]
        );
        for ordinal in 1..7 {
            assert_eq!(
                before["centre_evidence"][ordinal],
                after["centre_evidence"][ordinal]
            );
        }
    }

    #[test]
    fn every_planet_correction_follows_its_graph_route_without_erasing_global_evidence() {
        let before = natal_composition(&controlled_input([15.0; 10])).unwrap();
        // Exact 907c native routes: the seven direct correspondences plus
        // qualified Neptune→Ajna and Pluto→crown. Uranus stays unrouted.
        let expected = [
            Some(6),
            Some(5),
            Some(4),
            Some(3),
            Some(2),
            Some(1),
            Some(0),
            None,
            Some(5),
            Some(6),
        ];
        for (planet, target) in expected.into_iter().enumerate() {
            let mut input = controlled_input([15.0; 10]);
            input["sky"]["bodies"][planet]["longitude_degrees"] = json!(45.0);
            let after = natal_composition(&input).unwrap();
            assert_ne!(before["q_natal"], after["q_natal"]);
            for ordinal in 0..7 {
                assert_eq!(
                    before["centre_evidence"][ordinal] != after["centre_evidence"][ordinal],
                    target == Some(ordinal)
                );
            }
            if let Some(ordinal) = target {
                let route = &after["planetary_contributions"][planet]["planetary_chakra_route"];
                assert_eq!(
                    route["chakra_coordinate"],
                    format!("#2-5-0/1-{}", ordinal + 1)
                );
                let relations = route["relations"].as_array().unwrap();
                assert!(!relations.is_empty());
                let kind = if planet >= 8 {
                    "HAS_CHAKRAL_ANCHOR"
                } else {
                    "PLANETARY_RESONANCE"
                };
                assert!(relations.iter().all(|r| r["source_kind"] == kind));
            }
        }
    }
}
