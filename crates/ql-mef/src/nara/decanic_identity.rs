//! Natal decan powers through source-resolved graph rulers and their recipients.
//! Retained C rulers are diagnostic comparisons, never routing authority. Source
//! conflicts remain unresolved. No face, identity blend, or gain is inferred.

use serde_json::{Value, json};

use super::BioQuaternion;
use crate::m2;

const ELEMENTS: [&str; 4] = ["Earth", "Fire", "Water", "Air"];
const PLANETS: [&str; 10] = [
    "Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto",
];

/// Magnitude already derived by the natal producer, before normalization.
/// Keeping it here prevents a second, silently different weighting policy.
pub(super) struct NatalPlacement {
    pub planet_id: usize,
    pub longitude_degrees: f64,
    pub weighted_contribution: f64,
}

fn efwa_component(native_element: u64) -> Result<usize, String> {
    // Decan_Face_Desc.element is ELEMENT_ID, not the distinct F/E/A/W order
    // of Elemental_Throughline.decan_element or the 72-carrier element axis.
    match native_element {
        4 => Ok(0),
        2 => Ok(1),
        3 => Ok(2),
        1 => Ok(3),
        _ => Err("natal decan has no four-element EFWA projection".into()),
    }
}

fn orientation(raw: [f64; 4]) -> Result<Option<BioQuaternion>, String> {
    if raw.iter().any(|v| !v.is_finite() || *v < 0.0) {
        return Err("nonfinite or negative decanic evidence".into());
    }
    let scale = raw.into_iter().fold(0.0_f64, f64::max);
    if scale == 0.0 {
        return Ok(None);
    }
    let [w, x, y, z] = raw.map(|v| v / scale);
    BioQuaternion { w, x, y, z }.normalized().map(Some)
}

pub(super) fn derive(
    placements: &[NatalPlacement],
    snapshot_ref: &str,
    weighting_policy: &str,
) -> Result<Value, String> {
    let catalogue = m2::catalogue();
    let table = catalogue.table("decan")?;
    let column = |name: &str| {
        table
            .columns()
            .iter()
            .position(|c| c == name)
            .ok_or_else(|| format!("native decan descriptor lacks {name}"))
    };
    let element_column = column("element")?;
    let ruler_column = column("ruling_planet")?;
    let mut seen = std::collections::BTreeSet::new();
    let mut raw = [0.0; 4];
    let mut centre_raw = [[0.0; 4]; 7];
    let mut centre_placements: [Vec<usize>; 7] = std::array::from_fn(|_| Vec::new());
    let mut contributions = Vec::with_capacity(placements.len());
    let mut unresolved_planet_ids = Vec::new();
    let mut unrouted_planet_ids = Vec::new();
    let mut unresolved_raw = [0.0; 4];
    let mut unrouted_raw = [0.0; 4];
    for placement in placements {
        let id = placement.planet_id;
        if id >= PLANETS.len() || !seen.insert(id) {
            return Err("invalid or duplicate natal planet in decanic channel".into());
        }
        let weight = placement.weighted_contribution;
        if !weight.is_finite() || weight <= 0.0 {
            return Err("decanic channel requires positive finite natal magnitude".into());
        }
        let light = m2::situated_decan(placement.longitude_degrees, false)?;
        let shadow = m2::situated_decan(placement.longitude_degrees, true)?;
        let light_row = table.row(usize::from(light.index()))?;
        let shadow_row = table.row(usize::from(shadow.index()))?;
        if light_row[element_column] != shadow_row[element_column]
            || light_row[ruler_column] != shadow_row[ruler_column]
        {
            return Err(
                "decan element/ruler differs by face; an explicit face source is required".into(),
            );
        }
        let element = efwa_component(light_row[element_column])?;
        let retained_ruler_id = usize::try_from(light_row[ruler_column])
            .map_err(|_| "decan ruler exceeds native planet identity")?;
        let retained_ruler = PLANETS
            .get(retained_ruler_id)
            .ok_or("decan ruler is not a native planet")?;
        let graph_route = m2::decan_planet_route(placement.longitude_degrees)?;
        let ruler_id = graph_route.planet_index;
        let ruler = ruler_id.map(|id| PLANETS[id]);
        let route = ruler_id.map(m2::planet_chakra_route).transpose()?.flatten();
        if ruler_id.is_none() {
            unresolved_planet_ids.push(id);
        }
        let ordinal = route.as_ref().map(|r| usize::from(r.chakra_index - 1));
        let mut local_raw = [0.0; 4];
        local_raw[element] = weight;
        raw[element] += weight;
        if let Some(ordinal) = ordinal {
            centre_raw[ordinal][element] += weight;
            centre_placements[ordinal].push(id);
        } else if ruler_id.is_none() {
            unresolved_raw[element] += weight;
        } else {
            unrouted_raw[element] += weight;
            unrouted_planet_ids.push(id);
        }
        let faces: Vec<Value> = [light, shadow].into_iter().map(|reading| json!({
            "face": reading.axes()[3], "native_decan_index": reading.index(),
            "native_axes": reading.axes(), "source_coordinate": table.binding(usize::from(reading.index())),
            "descriptor": table.row(usize::from(reading.index())).expect("validated native decan row")
        })).collect();
        contributions.push(json!({
            "native_planet_id": id, "body": PLANETS[id],
            "longitude_degrees": placement.longitude_degrees,
            "decan_global_index": (placement.longitude_degrees / 10.0).floor() as u8,
            "decan_index_within_sign": light.axes()[2],
            "faces": faces, "selected_face": null,
            "native_element_id": light_row[element_column],
            "element": ELEMENTS[element], "raw_efwa": local_raw,
            "weighted_contribution": weight,
            "decan_ruler_planet_id": ruler_id, "decan_ruler": ruler,
            "ruler_resolution": graph_route.status,
            "graph_decan_route": graph_route,
            "retained_c_ruler_planet_id": retained_ruler_id,
            "retained_c_ruler": retained_ruler,
            "retained_c_agrees_with_graph": ruler_id.map(|id| id == retained_ruler_id),
            "decan_ruler_chakra_route": route, "receiving_centre_ordinal": ordinal,
            "role": "natal placement through its source-resolved graph decan ruler; distinct from natal-body direct planetary resonance",
            "ruler_authority": "typed graph RULED_BY with source conflicts retained; C ruler diagnostic only"
        }));
    }
    let presentation_partition = super::intake_composition::EvidencePartition {
        raw, centres: centre_raw, unrouted: unrouted_raw, unresolved: unresolved_raw,
        input_planet_ids: seen.into_iter().collect(), unrouted_planet_ids,
        unresolved_planet_ids: unresolved_planet_ids.clone(),
    }.reading("decan-ruler-reception", json!({
        "snapshot_ref": snapshot_ref, "weighting_policy": weighting_policy,
        "weighting_source": "natal_composition.policy_source; original per-placement magnitude reused",
        "registry_revision": catalogue.registry_revision(),
        "route": "planetary_contributions[].graph_decan_route and decan_ruler_chakra_route",
        "arithmetic": "centre raw EFWA divided by all natal placement magnitudes; conflicts and absent routes retain mass without redistribution"
    }))?;
    let q = orientation(raw)?;
    let centres = centre_raw
        .into_iter()
        .enumerate()
        .map(|(ordinal, raw)| {
            let direction = orientation(raw)?;
            let magnitude: f64 = raw.iter().sum();
            if !magnitude.is_finite() {
                return Err("decanic centre evidence overflow".to_owned());
            }
            Ok(json!({
                "ordinal": ordinal, "native_m2_chakra_id": ordinal + 1,
                "source_coordinate": format!("#2-5-0/1-{}", ordinal + 1),
                "natal_planet_ids": centre_placements[ordinal],
                "raw_efwa_evidence": raw, "weighted_power": magnitude,
                "decan_ruler_orientation": {
                    "status": if direction.is_some() { "available" } else { "unavailable" },
                    "quaternion": direction, "normalization": "L2; zero remains absent",
                "role": "elemental direction from resolved graph decan rulers; not an intrinsic chakra axis or receiver gain"
                }
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(json!({
        "schema": "ql.nara-decan-ruler-reception/v1", "snapshot_ref": snapshot_ref,
        "status": if unresolved_planet_ids.is_empty() { "available" } else { "partial" },
        "unresolved_planet_ids": unresolved_planet_ids,
        "source_conflict": {
            "reference": "https://github.com/EpiLogos/QL-MEF/pull/254",
            "source_revision": "a735f072c9ed88b6b467a8c4e3a66441fe2a4c83",
            "path": "docs/kernel-rebuild/m123-scene-map/m2-sky.md",
            "section": "4 D1 and D5",
            "detail": "The retained C triplicity rulers disagree with pinned graph Chaldean rulers on 27 of 36 decans and are diagnostic only. Resolved graph rulers select recipients. Cancer decan 3 preserves Moon property versus Saturn RULED_BY as unresolved and contributes no automatic centre assignment."
        },
        "basis_order": ELEMENTS, "weighting_policy": weighting_policy,
        "weighting_source": "same per-placement keplerian weight multiplied by dignity as natal_composition; reused before normalization",
        "method_source": {
            "repository": "EpiLogos/Epi-Logos-C-Experiments",
            "revision": "daa660cbc1b8c5da83828698665a753852cb0287",
            "path": "Idea/Bimba/Seeds/M/M4'/nara-m4-0-identity-branch-integration-map.md",
            "blob": "c22505931208c407377a151d16007cbd908aee39",
            "sections": ["7 canonical Parashakti ordering", "10.2 decan power", "10.3 planetary-chakral route", "10.5 native planetary hierarchy"],
            "standing": "draft integration method evaluated with native decan geometry and source-resolved graph rulers; conflicts remain unadmitted"
        },
        "native_source": {"table": table.source_symbol(), "sources": catalogue.sources(), "registry_revision": catalogue.registry_revision()},
        "planetary_contributions": contributions, "centre_evidence": centres,
        "presentation_partition": presentation_partition,
        "raw_efwa": raw, "q_decan_element": q,
        "method_notes": [
            "Both native faces are retained. Their ruler and element must agree before this face-independent reading is available.",
            "The native descriptors currently give the same element as the sign; this is not an independent extra identity contribution.",
            "The recipient comes from the decan ruler, not the natal body's direct chakra route. The channels are not added together.",
            "Only source-resolved graph rulers feed centre evidence. Conflicting rulers retain their source candidates without choosing a recipient. C table rulers are comparison evidence only.",
            "No MEF carrier reinterpretation, unsupported face selection, chart-ruler override, angular/aspect modifier, blending coefficient or physical gain is inferred."
        ],
        "effect_authority_granted": false
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn natal(sun_longitude: f64) -> Value {
        let input = json!({"sky": {
            "schema":"ql.sky-snapshot/v1", "snapshot_ref":"controlled:real-native-decan-calculation",
            "bodies": PLANETS.iter().enumerate().map(|(id, body)| json!({
                "native_planet_id":id, "body":body,
                "longitude_degrees":if id == 0 { sun_longitude } else { 45.0 }
            })).collect::<Vec<_>>()
        }});
        super::super::intake_composition::natal_composition(&input).unwrap()
    }

    #[test]
    fn within_sign_boundary_changes_ruling_power_and_centre_without_changing_sign() {
        let before = natal(9.999);
        let after = natal(10.0);
        assert_eq!(before["q_natal"], after["q_natal"]);
        assert_eq!(before["centre_evidence"], after["centre_evidence"]);
        let a = &before["decanic_channel"]["planetary_contributions"][0];
        let b = &after["decanic_channel"]["planetary_contributions"][0];
        assert_eq!(a["element"], "Fire");
        assert_eq!(b["element"], "Fire");
        assert_eq!(a["decan_ruler"], "Mars");
        assert_eq!(b["decan_ruler"], "Sun");
        assert_eq!(a["receiving_centre_ordinal"], 2);
        assert_eq!(b["receiving_centre_ordinal"], 6);
        assert_eq!(
            a["weighted_contribution"],
            before["planetary_contributions"][0]["weighted_contribution"]
        );
        assert_ne!(
            before["decanic_channel"]["centre_evidence"],
            after["decanic_channel"]["centre_evidence"]
        );
        assert!(a["selected_face"].is_null());
    }

    #[test]
    fn every_native_decan_face_preserves_its_element_and_canonical_ruler_route() {
        let table = m2::catalogue().table("decan").unwrap();
        for decan in 0..36 {
            let longitude = decan as f64 * 10.0 + 5.0;
            let reading = derive(
                &[NatalPlacement {
                    planet_id: 0,
                    longitude_degrees: longitude,
                    weighted_contribution: 2.0,
                }],
                "source",
                "weighting",
            )
            .unwrap();
            let p = &reading["planetary_contributions"][0];
            for face in 0..2 {
                let native = m2::situated_decan(longitude, face == 1).unwrap();
                let row = table.row(usize::from(native.index())).unwrap();
                assert_eq!(p["faces"][face]["native_decan_index"], native.index());
                assert_eq!(p["native_element_id"], row[0]);
                assert_eq!(p["retained_c_ruler_planet_id"], row[4]);
                // The map's HAS_TATTVA_COUNTERPART reaches the same element row.
                let linked = m2::linked_readings("decan", usize::from(native.index())).unwrap();
                let element = linked.iter().find(|l| l.table == "element").unwrap();
                assert_eq!(element.index, row[0] as usize);
            }
            let graph = m2::decan_planet_route(longitude).unwrap();
            if let Some(planet) = graph.planet_index {
                let route = m2::planet_chakra_route(planet).unwrap().unwrap();
                assert_eq!(p["decan_ruler_planet_id"], planet);
                assert_eq!(p["receiving_centre_ordinal"], route.chakra_index - 1);
            } else {
                assert!(p["decan_ruler_planet_id"].is_null());
                assert!(p["receiving_centre_ordinal"].is_null());
            }
            assert_eq!(
                reading["raw_efwa"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .sum::<f64>(),
                2.0
            );
            assert_eq!(
                reading["centre_evidence"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|c| c["decan_ruler_orientation"]["status"] == "available")
                    .count(),
                usize::from(graph.planet_index.is_some())
            );
        }
    }

    #[test]
    fn graph_ruler_wins_over_c_and_internal_graph_conflict_has_no_automatic_recipient() {
        let aries_three = natal(25.0);
        let contribution = &aries_three["decanic_channel"]["planetary_contributions"][0];
        assert_eq!(contribution["decan_ruler"], "Venus");
        assert_eq!(contribution["retained_c_ruler"], "Jupiter");
        assert_eq!(contribution["retained_c_agrees_with_graph"], false);
        assert_eq!(contribution["receiving_centre_ordinal"], 3);

        let cancer_three = natal(115.0);
        let channel = &cancer_three["decanic_channel"];
        let contribution = &channel["planetary_contributions"][0];
        assert_eq!(channel["status"], "partial");
        assert!(contribution["decan_ruler"].is_null());
        assert!(contribution["receiving_centre_ordinal"].is_null());
        assert_eq!(channel["unresolved_planet_ids"], json!([0]));
        assert!(
            !contribution["graph_decan_route"]["source_conflicts"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        for centre in channel["centre_evidence"].as_array().unwrap() {
            assert!(
                !centre["natal_planet_ids"]
                    .as_array()
                    .unwrap()
                    .contains(&json!(0))
            );
        }
        // The original direct Sun→crown evidence remains available and separate.
        assert_eq!(
            cancer_three["planetary_contributions"][0]["receiving_centre_ordinal"],
            6
        );
    }

    #[test]
    fn missing_evidence_stays_absent_and_invalid_magnitudes_are_refused() {
        let empty = derive(&[], "source", "weighting").unwrap();
        assert!(empty["q_decan_element"].is_null());
        assert_eq!(empty["presentation_partition"]["available"], false);
        assert!(empty["presentation_partition"]["centres"][0]["mass_share_l1"].is_null());
        for c in empty["centre_evidence"].as_array().unwrap() {
            assert!(c["decan_ruler_orientation"]["quaternion"].is_null());
            assert!(c.get("gain").is_none());
        }
        for magnitude in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(
                derive(
                    &[NatalPlacement {
                        planet_id: 0,
                        longitude_degrees: 10.0,
                        weighted_contribution: magnitude
                    }],
                    "source",
                    "weighting"
                )
                .is_err()
            );
        }
    }

    #[test]
    fn presentation_partition_routes_without_redistributing_conflicts_or_coupling_channels() {
        let before = natal(9.999);
        let after = natal(10.0);
        assert_eq!(
            before["presentation_partition"],
            after["presentation_partition"]
        );
        let a = &before["decanic_channel"]["presentation_partition"];
        let b = &after["decanic_channel"]["presentation_partition"];
        let denominator = a["denominator"]["weighted_total"].as_f64().unwrap();
        assert_eq!(a["denominator"], b["denominator"]);
        let solar_mass = before["planetary_contributions"][0]["weighted_contribution"]
            .as_f64()
            .unwrap()
            / denominator;
        for ordinal in 0..7 {
            let delta = b["centres"][ordinal]["mass_share_l1"].as_f64().unwrap()
                - a["centres"][ordinal]["mass_share_l1"].as_f64().unwrap();
            let expected = match ordinal {
                2 => -solar_mass,
                6 => solar_mass,
                _ => 0.0,
            };
            assert!((delta - expected).abs() < 1e-12);
        }
        let conflicted = natal(115.0);
        let partition = &conflicted["decanic_channel"]["presentation_partition"];
        assert_eq!(partition["unresolved"]["planet_ids"], json!([0]));
        assert_eq!(
            partition["denominator"],
            conflicted["presentation_partition"]["denominator"]
        );
        let residual = partition["unresolved"]["mass_share_l1"].as_f64().unwrap();
        let expected = conflicted["planetary_contributions"][0]["weighted_contribution"]
            .as_f64()
            .unwrap()
            / partition["denominator"]["weighted_total"].as_f64().unwrap();
        assert!((residual - expected).abs() < 1e-12);
        let sum: f64 = partition["centres"]
            .as_array()
            .unwrap()
            .iter()
            .map(|centre| centre["mass_share_l1"].as_f64().unwrap())
            .sum();
        assert!((sum + residual - 1.0).abs() < 1e-12);
        assert_eq!(partition["unrouted"]["weighted_power"], 0.0);
    }
}
