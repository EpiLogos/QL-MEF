//! Dated planetary field and selected identity/transit baseline. Astronomy
//! remains the qualified sky provider's; absent activity is never fabricated.
use super::{BioQuaternion, intake::IdentityProfile, intake_composition::PLANETS};
use serde_json::{Value, json};

fn lift(longitudes: &[f64]) -> Result<([u32; 4], Option<BioQuaternion>), String> {
    let mut counts = [0u32; 4];
    for longitude in longitudes {
        if !longitude.is_finite() || !(0.0..360.0).contains(longitude) {
            return Err("transit longitude must be finite in [0,360)".into());
        }
        let sign = (*longitude / 30.).floor() as usize;
        counts[[1, 0, 3, 2][sign % 4]] += 1;
    }
    let total = longitudes.len() as f64;
    let q = if total > 0. {
        Some(
            BioQuaternion {
                w: counts[0] as f64 / total,
                x: counts[1] as f64 / total,
                y: counts[2] as f64 / total,
                z: counts[3] as f64 / total,
            }
            .normalized()?,
        )
    } else {
        None
    };
    Ok((counts, q))
}

pub fn transit(sky: Option<&Value>) -> Result<Value, String> {
    let mut longitudes = Vec::new();
    let mut contributions = Vec::new();
    if let Some(sky) = sky {
        if sky["schema"] != "ql.sky-snapshot/v1"
            || sky["source_binding"]["registry_revision"]
                != json!(crate::m2::catalogue().registry_revision())
        {
            return Err(
                "transit requires the actual qualified sky snapshot and current native M2 registry"
                    .into(),
            );
        }
        let snapshot = sky["snapshot_ref"]
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("transit snapshot reference absent")?;
        if sky["request"]["schema"] != "ql.sky-request/v1"
            || sky["request"]["epoch"].as_str().is_none()
            || sky["receipt_unix_ms"].as_u64().is_none()
            || sky["provider"]["adapter_sha256"].as_str().is_none()
        {
            return Err("transit sky lacks dated provider provenance".into());
        }
        let bodies = sky["bodies"]
            .as_array()
            .ok_or("transit sky bodies absent")?;
        if bodies.len() != PLANETS.len() {
            return Err("qualified transit sky must contain all ten native bodies".into());
        }
        for (id, body) in bodies.iter().enumerate() {
            if body["native_planet_id"] != id || body["body"] != PLANETS[id] {
                return Err("transit native body identity/order drift".into());
            }
            let longitude = body["longitude_degrees"]
                .as_f64()
                .ok_or("transit longitude absent")?;
            lift(&[longitude])?;
            longitudes.push(longitude);
            contributions.push(json!({"native_planet_id":id,"body":PLANETS[id],"longitude_degrees":longitude,"sign_index":(longitude/30.).floor() as usize,"efwa_component":([1,0,3,2][((longitude/30.).floor() as usize)%4]),"weight":1,"snapshot_ref":snapshot}));
        }
    }
    let (counts, q) = lift(&longitudes)?;
    Ok(
        json!({"schema":"ql.nara-transit/v1","status":if q.is_some(){"available"}else{"unavailable"},"sky":sky,"snapshot_ref":sky.map(|s| &s["snapshot_ref"]),
        "q_transit":q,"basis":["earth","fire","water","air"],"elemental_counts":counts,"valid_planet_count":longitudes.len(),"planetary_contributions":contributions,
        "source":{"repository":"EpiLogos/Epi-Logos-C-Experiments","revision":"daa660cbc1b8c5da83828698665a753852cb0287","path":"Body/S/S0/portal-core/src/state.rs","blob":"94f491db2b4fa96a436c3ea6c306a3beea3d1d69","symbol":"update_kairos_full","lines":[118,153]},
        "normalization":"equal valid-planet EFWA counts divided by valid count, then L2; native Hamilton component order",
        "standing":"source-defined dated sky lift; not a chakra receiver weighting policy",
        "absence_reason":if q.is_none(){Some("No actual dated sky supplied; no identity or day-fraction fallback")}else{None},"effect_authority_granted":false}),
    )
}

pub fn personal_current(identity: &Value, transit_reading: &Value) -> Result<Value, String> {
    personal_current_with_activity(identity, transit_reading, None)
}

pub fn personal_current_with_activity(
    identity: &Value,
    transit_reading: &Value,
    activity: Option<&Value>,
) -> Result<Value, String> {
    if identity["schema"] != "ql.nara-identity-reading/v1"
        || transit_reading["schema"] != "ql.nara-transit/v1"
    {
        return Err("personal current requires native identity and transit readings".into());
    }
    let profile: IdentityProfile =
        serde_json::from_value(identity["profile"].clone()).map_err(|e| e.to_string())?;
    let natal = identity.get("natal").filter(|v| !v.is_null());
    let rebuilt = profile.inspect(natal)?;
    if &rebuilt != identity {
        return Err(
            "personal current identity reading differs from its exact source derivation".into(),
        );
    }
    let sky = transit_reading.get("sky").filter(|v| !v.is_null());
    if transit(sky)? != *transit_reading {
        return Err("personal current transit reading differs from its exact dated source".into());
    }
    let a: Option<BioQuaternion> =
        serde_json::from_value(identity["identity_composition"]["q_core"].clone())
            .map_err(|e| e.to_string())?;
    let b: Option<BioQuaternion> =
        serde_json::from_value(transit_reading["q_transit"].clone()).map_err(|e| e.to_string())?;
    let baseline = match (a, b) {
        (Some(a), Some(b)) => Some(a.compose(b)?),
        _ => None,
    };
    let resonance = match (a, b) {
        (Some(a), Some(b)) => {
            let a = a.normalized()?;
            let b = b.normalized()?;
            let signed_dot = (a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z).clamp(-1., 1.);
            Some(
                json!({"signed_dot":signed_dot,"score":signed_dot.abs().clamp(0.,1.),
                "source":{"repository":"EpiLogos/Epi-Logos-C-Experiments","revision":"c7872e96a12e8253de6818876c2a039fd8082e46","path":"Body/S/S0/portal-core/src/personal_identity.rs","blob":"3a834d2ea132710c8ccd838b92db66cb9a9c322c","symbol":"PersonalResonance::from_quaternions","lines":[525,536]},
                "standing":"source-defined identity/transit alignment; no inferred Human Design modulation or medium coefficients"}),
            )
        }
        _ => None,
    };
    let mut output = json!({"schema":"ql.nara-personal-current/v1","person_ref":profile.person_ref,"nara_ref":profile.nara_ref,"input_revision":identity["input_revision"],"snapshot_ref":transit_reading["snapshot_ref"],
        "identity":identity,"transit":transit_reading,"q_identity":a,"resonance":resonance,"q_identity_transit":baseline,"baseline_available":baseline.is_some(),"q_activity":null,"q_composed":null,"activity_status":"unavailable",
        "activity_absence_reason":"No source-bound interpreted activity contribution supplied","standing":"selected draft identity multiplied by actual dated transit; two-input baseline, full three-input composition unavailable",
        "composition":{"operator":"normalized Hamilton multiplication","order":["q_identity","q_transit"],"native_owner":"ql_mef::nara::BioQuaternion::compose","identity_mutated":false},
        "private":true,"public_export":false,"effect_authority_granted":false});
    if let Some(activity) = activity {
        if activity["schema"] != "ql.nara-m3-activity/v1"
            || activity["subject_ref"] != profile.person_ref
            || activity["event_ref"] != transit_reading["snapshot_ref"]
        {
            return Err(
                "Activity does not belong to this saved person and actual dated event".into(),
            );
        }
        output["activity"] = activity.clone();
        if activity["status"] == "available" {
            let q: BioQuaternion = serde_json::from_value(activity["q_activity"].clone())
                .map_err(|e| e.to_string())?;
            let composed = baseline.map(|base| base.compose(q)).transpose()?;
            output["q_activity"] = activity["q_activity"].clone();
            output["q_composed"] = json!(composed);
            output["activity_status"] = json!("available");
            output["activity_absence_reason"] = Value::Null;
            output["standing"] = json!(
                "selected draft identity, actual dated transit, and explicitly selected symbolic M3 activity policy; no legacy contemplation closure"
            );
            output["composition"]["order"] = json!(["q_identity", "q_transit", "q_activity"]);
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_transit_lift_preserves_efwa_and_exact_sign_boundaries() {
        for sign in 0..12 {
            let (c, q) = lift(&[sign as f64 * 30.]).unwrap();
            let component = [1, 0, 3, 2][sign % 4];
            assert_eq!(c[component], 1);
            let q = q.unwrap();
            assert_eq!([q.w, q.x, q.y, q.z][component], 1.);
        }
        assert_eq!(lift(&[29.999]).unwrap().0, [0, 1, 0, 0]);
        assert_eq!(lift(&[30.]).unwrap().0, [1, 0, 0, 0]);
        let (c, q) = lift(&[0., 1., 30., 90.]).unwrap();
        assert_eq!(c, [1, 2, 1, 0]);
        let q = q.unwrap();
        assert!((q.x - 2. / 6f64.sqrt()).abs() < 1e-14);
    }
    #[test]
    fn absent_transit_has_no_time_or_identity_fallback_and_invalid_angles_refuse() {
        let r = transit(None).unwrap();
        assert!(r["q_transit"].is_null());
        assert_eq!(r["status"], "unavailable");
        for value in [f64::NAN, f64::INFINITY, -1., 360.] {
            assert!(lift(&[value]).is_err());
        }
        assert!(transit(Some(&json!({"schema":"ql.sky-snapshot/v1","bodies":[]}))).is_err());
    }
}
