//! Derivations supported by actual imported/self-reported identity evidence.
//! Source mappings do not supply missing questionnaire data or weighting laws.

use serde_json::{Value, json};

use super::{BioQuaternion, identity_encoding::LENSES, intake::IdentityProfile};

const FUNCTIONS: [&str; 4] = ["sensation", "intuition", "feeling", "thinking"];
const SOURCE_BLOB: &str = "c22505931208c407377a151d16007cbd908aee39";

pub(super) fn validate_jungian_scores(data: &Value) -> Result<(), String> {
    if let Some(basis) = data.get("questionnaire_score_basis") {
        super::text(
            basis.as_str().ok_or(
                "questionnaire_score_basis must declare the comparable function-strength scale",
            )?,
            "questionnaire score basis",
        )?;
    }
    if let Some(scores) = data.get("questionnaire_scores") {
        let scores = scores
            .as_object()
            .ok_or("questionnaire_scores must be an object of actual reported scales")?;
        for name in FUNCTIONS
            .into_iter()
            .chain(["introversion", "extroversion"])
        {
            if let Some(value) = scores.get(name) {
                if !value.as_f64().is_some_and(|n| n.is_finite() && n >= 0.) {
                    return Err(format!(
                        "questionnaire function magnitude {name} must be finite and non-negative"
                    ));
                }
            }
        }
    }
    Ok(())
}

fn source(sections: &[&str]) -> Value {
    json!({"repository":"EpiLogos/Epi-Logos-C-Experiments","path":"Idea/Bimba/Seeds/M/M4'/nara-m4-0-identity-branch-integration-map.md", "commit":"daa660cbc1b8c5da83828698665a753852cb0287", "blob":SOURCE_BLOB, "sections":sections, "standing":"integration-draft"})
}

fn jungian(profile: &IdentityProfile) -> Result<Value, String> {
    let Some(report) = &profile.jungian else {
        return Ok(
            json!({"coordinate":"M4-0-2","status":"unavailable","absence_reason":"No Jungian assessment or self-report supplied","original_report":null}),
        );
    };
    validate_jungian_scores(&report.data)?;
    let scores = report
        .data
        .get("questionnaire_scores")
        .and_then(Value::as_object);
    let basis = report
        .data
        .get("questionnaire_score_basis")
        .and_then(Value::as_str);
    let missing: Vec<_> = FUNCTIONS
        .iter()
        .filter(|name| scores.is_none_or(|s| !s.contains_key(**name)))
        .copied()
        .collect();
    let unprojected: Vec<_> = scores
        .into_iter()
        .flat_map(|s| s.keys())
        .filter(|name| {
            !FUNCTIONS.contains(&name.as_str())
                && !["introversion", "extroversion"].contains(&name.as_str())
        })
        .cloned()
        .collect();
    let mut result = json!({"coordinate":"M4-0-2","status":"unavailable","source":source(&["13-17"]),"original_report":report,
        "score_basis":basis,"missing_functions":missing,"unprojected_score_keys":unprojected,
        "basis":["earth","fire","water","air"],"raw_efwa":null,"balance_efwa":null,"quaternion":null,
        "caps":{"aether_gate":null,"mineral_cap":null},
        "mapping":[{"function":"sensation","lens":"L1p","position":1,"element":"earth"},{"function":"intuition","lens":"L1p","position":4,"element":"fire"},{"function":"feeling","lens":"L1p","position":2,"element":"water"},{"function":"thinking","lens":"L1p","position":3,"element":"air"}],
        "absence_reason":if basis.is_none(){"No declared comparable function-strength basis; raw questionnaire scales are preserved without reinterpretation"}else{"All four comparable elemental function magnitudes are required; missing functions are not zero-filled"}});
    if basis.is_none() {
        return Ok(result);
    }
    if let Some(scores) = scores {
        result["caps"]["aether_gate"] = scores.get("introversion").cloned().unwrap_or(Value::Null);
        result["caps"]["mineral_cap"] = scores.get("extroversion").cloned().unwrap_or(Value::Null);
        if missing.is_empty() {
            let raw = FUNCTIONS.map(|name| {
                scores[name]
                    .as_f64()
                    .expect("validated comparable magnitude")
            });
            result["raw_efwa"] = json!(raw);
            let max = raw.iter().copied().fold(0., f64::max);
            if max == 0. {
                result["absence_reason"] =
                    json!("Reported function magnitudes carry no elemental mass");
                return Ok(result);
            }
            // Scale before both norms so finite reported values cannot overflow.
            let scaled = raw.map(|n| n / max);
            let total: f64 = scaled.iter().sum();
            let q = BioQuaternion {
                w: scaled[0],
                x: scaled[1],
                y: scaled[2],
                z: scaled[3],
            }
            .normalized()?;
            result["balance_efwa"] = json!(scaled.map(|n| n / total));
            result["quaternion"] = json!(q);
            result["status"] = json!("available");
            result["absence_reason"] = Value::Null;
        }
    }
    Ok(result)
}

fn numeric_trace(n: u64) -> Value {
    let chromatic = (n % 12) as usize;
    json!({"number":n,"mod6":n%6,"inverse":5-n%6,"mod12":chromatic,"anchor_lens":LENSES[chromatic/2+(chromatic%2)*6]})
}

fn line_projection(line: u64) -> Value {
    let (position, element, role) = match line {
        1 => (1, "earth", "element"),
        2 => (2, "water", "element"),
        3 => (3, "air", "element"),
        4 => (4, "fire", "element"),
        5 => (5, "mineral", "cap"),
        6 => (0, "aether", "cap"),
        _ => unreachable!("validated Gene Keys line"),
    };
    json!({"line":line,"position":position,"element":element,"output_role":role,"standing":"provisional-source-mapping","weighted":false})
}

/// Retain the original attributable report alongside supported source routes.
/// GK/HD do not get quaternions from invented sphere/gate weights or affinities.
pub fn derive(profile: &IdentityProfile) -> Result<Value, String> {
    profile.validate()?;
    let gene_keys = if let Some(report) = &profile.gene_keys {
        let spheres: Vec<_> = report.data["spheres"].as_array().expect("validated Gene Keys spheres").iter().map(|sphere| {
            json!({"name":sphere["name"],"key":numeric_trace(sphere["key"].as_u64().expect("validated key")),"line_projection":line_projection(sphere["line"].as_u64().expect("validated line"))})
        }).collect();
        json!({"coordinate":"M4-0-3","status":"trace-only","source":source(&["23.1-23.4"]),"original_report":report,"spheres":spheres,"quaternion":null,"absence_reason":"No selected key/sphere weighting and lens-affinity policy; source residues and provisional line projections retained"})
    } else {
        json!({"coordinate":"M4-0-3","status":"unavailable","original_report":null,"quaternion":null,"absence_reason":"No attributable Gene Keys import supplied"})
    };
    let human_design = if let Some(report) = &profile.human_design {
        let mut gates = Vec::new();
        for (key, side) in [
            ("personality_gates", "personality"),
            ("design_gates", "design"),
        ] {
            for gate in report.data[key]
                .as_array()
                .expect("validated Human Design gates")
            {
                gates.push(json!({"side":side,"gate":numeric_trace(gate.as_u64().expect("validated gate")),"line_projection":null,"line_absence_reason":"Imported gate identifier has no line; no line inferred"}));
            }
        }
        json!({"coordinate":"M4-0-4","status":"trace-only","source":source(&["32.3-32.5"]),"original_report":report,"gates":gates,"defined_centres":report.data["defined_centres"],"channels":report.data["channels"],"quaternion":null,"absence_reason":"No selected gate/centre weighting and lens-affinity policy; nine BodyGraph centres and Personality/Design remain their own source structures"})
    } else {
        json!({"coordinate":"M4-0-4","status":"unavailable","original_report":null,"quaternion":null,"absence_reason":"No attributable Human Design BodyGraph import supplied"})
    };
    Ok(
        json!({"schema":"ql.nara-derived-identity-contributions/v1","jungian":jungian(profile)?,"gene_keys":gene_keys,"human_design":human_design,"identity_blend":null,"blend_absence_reason":"Constituent derivations do not select identity blend weights"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile() -> IdentityProfile {
        serde_json::from_value(json!({"schema":"ql.nara-identity-profile/v1","person_ref":"central:person:report-derivation-test","nara_ref":"ql:nara:report-derivation-test","name":"Report derivation test",
            "birth":{"date":null,"time":null,"precision":"unknown","uncertainty_minutes":null,"fold":null,"place":null},"jungian":null,"gene_keys":null,"human_design":null,"quintessence":null})).unwrap()
    }

    fn report(data: Value) -> super::super::intake::IdentityReport {
        serde_json::from_value(json!({"source":{"source_ref":"protected:reported-assessment","revision":"assessment-revision-one","standing_ref":"reported"},"method":"Actual supplied function-strength measurements on one declared scale","route":"import","data":data})).unwrap()
    }

    #[test]
    fn comparable_reported_functions_reach_real_efwa_quaternion() {
        let mut p = profile();
        p.jungian = Some(report(
            json!({"system":"jungian","type":"INTJ","questionnaire_score_basis":"Comparable measured function strengths, common ratio scale",
            "questionnaire_scores":{"sensation":0.,"intuition":3.,"feeling":0.,"thinking":4.,"introversion":2.}}),
        ));
        let r = p.inspect(None).unwrap();
        let j = &r["derived_identity_contributions"]["jungian"];
        assert_eq!(j["raw_efwa"], json!([0., 3., 0., 4.]));
        assert_eq!(j["quaternion"], json!({"w":0.,"x":0.6,"y":0.,"z":0.8}));
        assert_eq!(j["caps"]["aether_gate"], 2.);
        assert_eq!(j["caps"]["mineral_cap"], Value::Null);
        assert_eq!(
            j["original_report"],
            serde_json::to_value(p.jungian.as_ref().unwrap()).unwrap()
        );
        p.jungian.as_mut().unwrap().data["questionnaire_scores"]["sensation"] = json!(6.);
        assert_ne!(
            derive(&p).unwrap()["jungian"]["quaternion"],
            j["quaternion"]
        );
    }

    #[test]
    fn no_function_strength_is_inferred_from_type_or_missing_scale() {
        let mut p = profile();
        p.jungian = Some(report(json!({"system":"mbti","type":"INTJ"})));
        assert_eq!(derive(&p).unwrap()["jungian"]["quaternion"], Value::Null);
        p.jungian.as_mut().unwrap().data["questionnaire_scores"] =
            json!({"sensation":1.,"intuition":2.,"feeling":3.,"thinking":4.,"Openness":95});
        let r = derive(&p).unwrap();
        assert_eq!(r["jungian"]["quaternion"], Value::Null);
        assert_eq!(r["jungian"]["unprojected_score_keys"], json!(["Openness"]));
        p.jungian.as_mut().unwrap().data["questionnaire_score_basis"] =
            json!("Comparable strengths, same measurement scale");
        p.jungian.as_mut().unwrap().data["questionnaire_scores"]
            .as_object_mut()
            .unwrap()
            .remove("feeling");
        assert_eq!(
            derive(&p).unwrap()["jungian"]["missing_functions"],
            json!(["feeling"])
        );
        assert_eq!(derive(&p).unwrap()["jungian"]["quaternion"], Value::Null);
        p.jungian.as_mut().unwrap().data["questionnaire_scores"]["feeling"] = json!(-1.);
        assert!(p.validate().is_err());
    }

    #[test]
    fn imported_keys_and_bodygraph_gates_keep_distinct_source_traces() {
        let mut p = profile();
        p.gene_keys = Some(report(
            json!({"spheres":[{"name":"Life's Work","key":23,"line":4}]}),
        ));
        p.human_design = Some(report(
            json!({"type":"Projector","strategy":"Invitation","authority":"Splenic","profile":"1/3","definition":"Single","personality_gates":[62],"design_gates":[62],"defined_centres":["ajna","throat"],"channels":[]}),
        ));
        let r = derive(&p).unwrap();
        assert_eq!(
            r["gene_keys"]["spheres"][0]["key"],
            json!({"number":23,"mod6":5,"inverse":0,"mod12":11,"anchor_lens":"L5p"})
        );
        assert_eq!(
            r["gene_keys"]["spheres"][0]["line_projection"]["element"],
            "fire"
        );
        assert_eq!(r["gene_keys"]["quaternion"], Value::Null);
        let gates = r["human_design"]["gates"].as_array().unwrap();
        assert_eq!(gates.len(), 2);
        assert_eq!(gates[0]["gate"]["anchor_lens"], "L1");
        assert_eq!(gates[0]["side"], "personality");
        assert_eq!(gates[1]["side"], "design");
        assert_eq!(
            r["human_design"]["defined_centres"],
            json!(["ajna", "throat"])
        );
        assert_eq!(r["human_design"]["quaternion"], Value::Null);
    }
}
