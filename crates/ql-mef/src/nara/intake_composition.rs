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
const PLANETS: [&str; 10] = [
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
    let mut contributions = Vec::with_capacity(10);
    for (id, position) in positions.into_iter().enumerate() {
        let longitude = position.ok_or_else(|| format!("missing natal planet: {}", PLANETS[id]))?;
        let row = table.row(id)?;
        if row[id_column] != id as u64 {
            return Err("native M2 planet identity order changed".into());
        }
        let signature = u8::try_from(row[signature_column])
            .map_err(|_| "native M2 elemental signature exceeds one byte")?;
        let [native_element, chakra, native_phase] = m2::unpack_signature(signature)?;
        let base_weight = row[velocity_column] as f64;
        if base_weight <= 0.0 {
            return Err("native M2 planetary weight must be positive".into());
        }
        let sign = (longitude / 30.0).floor() as u8;
        let element = component(sign);
        let (dignity_name, multiplier) = dignity(id, sign);
        let weight = base_weight * multiplier;
        let mut planetary_raw = [0.0; 4];
        planetary_raw[element] = weight;
        raw[element] += weight;

        // Membership comes from the actual retained M2 relation. Its evidence
        // determines a natal direction below, independently of receiver gain,
        // oscillation phase, or coupling.
        let ordinal = if id == 0 {
            if chakra != 0 {
                return Err("native M2 Sun parent mapping changed".into());
            }
            None
        } else {
            if !(1..=7).contains(&chakra) {
                return Err("native M2 non-Sun body has no canonical receiving centre".into());
            }
            let ordinal = usize::from(chakra - 1);
            centre_planets[ordinal].push(id);
            centre_raw[ordinal][element] += weight;
            Some(ordinal)
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
            "native_m2_chakra_id": chakra,
            "native_cousto_frequency_hz": row[frequency_column],
            "receiving_centre_ordinal": ordinal,
            "role": if id == 0 { "solar-parent-not-chakra-mapped" } else { "mapped-planetary-evidence" }
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
                "natal_orientation": natal_orientation,
                "standing": "native M2 evidence membership and normalized natal direction; receiver dynamics not derived"
            }))
        })
        .collect::<Result<_, String>>()?;
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
    fn native_m2_partitions_keep_sun_and_earth_separate() {
        let reading = natal_composition(&controlled_input([15.0; 10])).unwrap();
        let expected = [
            vec![6, 9],
            vec![1],
            vec![4, 5],
            vec![3],
            vec![2],
            vec![7],
            vec![8],
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
        assert!(reading["planetary_contributions"][0]["receiving_centre_ordinal"].is_null());
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
        assert!(root["z"].as_f64().unwrap() > 0.0);
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
}
