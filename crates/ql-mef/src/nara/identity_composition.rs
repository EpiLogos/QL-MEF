//! Explicitly selected M4-0-5 draft core composition. This does not assign
//! canonical receiver dynamics or grant any material effect authority.
use super::{BioQuaternion, intake::IdentityProfile};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const SOURCE_BLOB: &str = "c22505931208c407377a151d16007cbd908aee39";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompositionPolicy {
    #[serde(rename = "draft-core-birthdate-decanic-40-60-v1")]
    DraftCoreBirthdateDecanic4060V1,
}

fn quaternion(value: &Value) -> Result<Option<BioQuaternion>, String> {
    if value.is_null() {
        return Ok(None);
    }
    let q: BioQuaternion = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    let norm = q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z;
    if ![q.w, q.x, q.y, q.z]
        .iter()
        .all(|x| x.is_finite() && *x >= 0.)
        || (norm - 1.).abs() > 1e-10
    {
        return Err(
            "identity composition requires native normalized nonnegative EFWA inputs".into(),
        );
    }
    Ok(Some(q))
}

pub(crate) fn derive(
    profile: &IdentityProfile,
    encoding: &Value,
    natal: Option<&Value>,
) -> Result<Value, String> {
    let decanic = natal.map(|n| &n["decanic_channel"]);
    let birth_q = quaternion(&encoding["elemental"]["quaternion"])?;
    let decan_q = quaternion(
        decanic
            .map(|d| &d["q_decan_element"])
            .unwrap_or(&Value::Null),
    )?;
    let mut missing = Vec::new();
    if profile.encoding_policy.is_none() {
        missing.push("selected encoding_policy");
    }
    if birth_q.is_none() {
        missing.push("birthdate_encoding.elemental.quaternion");
    }
    if decan_q.is_none() {
        missing.push("natal_composition.decanic_channel.q_decan_element");
    }
    let selected = profile.composition_policy.is_some();
    let q_core = match (selected && missing.is_empty(), birth_q, decan_q) {
        (true, Some(a), Some(b)) => Some(
            BioQuaternion {
                w: 0.40 * a.w + 0.60 * b.w,
                x: 0.40 * a.x + 0.60 * b.x,
                y: 0.40 * a.y + 0.60 * b.y,
                z: 0.40 * a.z + 0.60 * b.z,
            }
            .normalized()?,
        ),
        _ => None,
    };
    Ok(json!({
        "schema":"ql.nara-identity-composition/v1", "policy":profile.composition_policy,
        "selected":selected, "status":if q_core.is_some(){"available"}else if selected{"unavailable"}else{"unselected"},
        "standing":"explicitly selected suggested draft core formula; not a canonical receiver policy",
        "source":{"repository":"EpiLogos/Epi-Logos-C-Experiments","revision":"daa660cbc1b8c5da83828698665a753852cb0287",
            "path":"Idea/Bimba/Seeds/M/M4'/nara-m4-0-identity-branch-integration-map.md", "blob":SOURCE_BLOB,"sections":["35","36","37"]},
        "input_revision":profile.revision()?, "basis":["earth","fire","water","air"],
        "weights":[0.40,0.60], "normalization":"L2 of weighted native unit EFWA quaternions; no missing-input redistribution",
        "inputs":{"birthdate_encoding":{"quaternion":birth_q,"encoding_policy":profile.encoding_policy,"source":encoding["source"]},
            "decanic_astrology":{"quaternion":decan_q,"snapshot_ref":decanic.map(|d| &d["snapshot_ref"]),
                "method_source":decanic.map(|d| &d["method_source"]),"native_source":decanic.map(|d| &d["native_source"]),
                "ruler_reception_status":decanic.map(|d| &d["status"]),"unresolved_planet_ids":decanic.map(|d| &d["unresolved_planet_ids"]) }},
        "q_core":q_core, "q_extended":null, "caps":{"birthdate_encoding":encoding["elemental"]["caps"]},
        "missing_inputs":missing, "absence_reason":if q_core.is_some(){Value::Null}else if !selected{json!("No draft core composition policy selected")}else{json!("Both native core inputs and an explicit encoding policy are required")},
        "canonical_policy":false,"effect_authority_granted":false,
        "limits":["Extended identity weighting, resonance bonuses and conflict penalties are not selected by this core policy.",
            "Decan elemental identity retains all placement mass; unresolved ruler reception remains separately unadmitted.",
            "Caps remain separate; no chakra gain, local distribution, transit, activity or physical coupling is inferred."]
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn profile() -> IdentityProfile {
        serde_json::from_value(json!({"schema":"ql.nara-identity-profile/v1","person_ref":"central:person:core-source-test","nara_ref":"ql:nara:core-source-test","name":"Core source test", "birth":{"date":"1990-06-15","time":null,"precision":"unknown","uncertainty_minutes":null,"fold":null,"place":null},"jungian":null,"gene_keys":null,"human_design":null,"quintessence":null})).unwrap()
    }
    fn inputs(p: &IdentityProfile) -> (Value, Value) {
        let encoding = super::super::identity_encoding::derive(
            p,
            &p.encoding_policy.clone().unwrap_or_default(),
        )
        .unwrap();
        // Actual native C descriptor, graph route and weighting engine, with
        // controlled placements; this is not a provider or birth-chart receipt.
        let natal = super::super::intake_composition::natal_composition(&json!({"sky":{"schema":"ql.sky-snapshot/v1","snapshot_ref":"controlled:native-core-arithmetic", "bodies":super::super::intake_composition::PLANETS.iter().enumerate().map(|(id,name)| json!({"native_planet_id":id,"body":name,"longitude_degrees":25.+id as f64*31.})).collect::<Vec<_>>()}})).unwrap();
        (encoding, natal)
    }
    #[test]
    fn selected_core_uses_actual_native_channels_and_exact_source_weights() {
        let mut p = profile();
        p.encoding_policy = Some(Default::default());
        p.composition_policy = Some(CompositionPolicy::DraftCoreBirthdateDecanic4060V1);
        let (e, n) = inputs(&p);
        let r = derive(&p, &e, Some(&n)).unwrap();
        let a = quaternion(&e["elemental"]["quaternion"]).unwrap().unwrap();
        let b = quaternion(&n["decanic_channel"]["q_decan_element"])
            .unwrap()
            .unwrap();
        let raw = [
            0.4 * a.w + 0.6 * b.w,
            0.4 * a.x + 0.6 * b.x,
            0.4 * a.y + 0.6 * b.y,
            0.4 * a.z + 0.6 * b.z,
        ];
        let norm = raw.iter().map(|v| v * v).sum::<f64>().sqrt();
        for (i, k) in ["w", "x", "y", "z"].iter().enumerate() {
            assert!((r["q_core"][k].as_f64().unwrap() - raw[i] / norm).abs() < 1e-14);
        }
        assert_eq!(r["status"], "available");
        assert_eq!(r["caps"]["birthdate_encoding"], e["elemental"]["caps"]);
        assert_eq!(r["effect_authority_granted"], false);
    }
    #[test]
    fn absent_policy_or_missing_native_input_never_substitutes_or_reweights() {
        let mut p = profile();
        let (e, n) = inputs(&p);
        assert_eq!(derive(&p, &e, Some(&n)).unwrap()["status"], "unselected");
        p.composition_policy = Some(CompositionPolicy::DraftCoreBirthdateDecanic4060V1);
        assert!(derive(&p, &e, Some(&n)).unwrap()["q_core"].is_null());
        p.encoding_policy = Some(Default::default());
        assert!(derive(&p, &e, None).unwrap()["q_core"].is_null());
        let mut missing = n.clone();
        missing["decanic_channel"]["q_decan_element"] = Value::Null;
        assert!(derive(&p, &e, Some(&missing)).unwrap()["q_core"].is_null());
    }
    #[test]
    fn selected_policy_is_persisted_revisioned_and_missing_chart_stays_unavailable() {
        let mut p = profile();
        let before = p.revision().unwrap();
        p.composition_policy = Some(CompositionPolicy::DraftCoreBirthdateDecanic4060V1);
        p.encoding_policy = Some(Default::default());
        assert_ne!(before, p.revision().unwrap());
        let bytes = serde_json::to_vec(&p).unwrap();
        let restored: IdentityProfile = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(p, restored);
        let r = restored.inspect(None).unwrap();
        assert_eq!(r["identity_composition"]["status"], "unavailable");
        assert!(r["identity"]["identity_quaternion_ref"].is_null());
        assert_eq!(
            r["derived_identity_contributions"]["identity_blend"],
            r["identity_composition"]
        );
    }
}
