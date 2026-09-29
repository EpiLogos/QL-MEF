//! Supplied-name/date refraction, from the explicitly provisional M4-0-0 draft.
//! The draft's EWFA quaternion is reordered into the current EFWA contract.
//! Neither a display name nor its derived numeric reading asserts a birth name.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{BioQuaternion, intake::IdentityProfile};

pub const SOURCE_BLOB: &str = "73f3c08e76fa66d044072676d8f5da09cd64bd6a";
pub const LENSES: [&str; 12] = [
    "L0", "L1", "L2", "L3", "L4", "L5", "L0p", "L1p", "L2p", "L3p", "L4p", "L5p",
];
pub type Matrix = [[f64; 6]; 12];

const CELL_MEANINGS: [[&str; 6]; 12] = [
    [
        "Why / presupposition",
        "What",
        "How",
        "Whom / Which / When",
        "Where / When / Why-four",
        "Why-so / Why-not / analogia",
    ],
    [
        "Svatantrya",
        "Material cause",
        "Efficient cause",
        "Formal cause",
        "Final cause",
        "Iccha Shakti / Will",
    ],
    [
        "Tetralemmaic ground",
        "IS",
        "IS-NOT",
        "BOTH",
        "NEITHER",
        "SILENCE",
    ],
    [
        "Concrescent desire",
        "Actual occasion",
        "Ingression",
        "Eternal objects",
        "Community integration",
        "Satisfaction / perishing",
    ],
    [
        "Sein / Being",
        "Geworfenheit",
        "Dasein",
        "Zeit",
        "Besorge / care",
        "Gelassenheit",
    ],
    [
        "Anuttara", "Para Vak", "Pasyanti", "Madhyama", "Vaikhari", "Matrka",
    ],
    ["One", "Two", "Three", "Four", "Five", "Six"],
    [
        "Introversion",
        "Sensation",
        "Feeling",
        "Thinking",
        "Intuition",
        "Extroversion",
    ],
    ["Aether", "Earth", "Water", "Air", "Fire", "Mineral"],
    [
        "Spirit / Geist",
        "Spring",
        "Summer",
        "Autumn",
        "Winter",
        "Life / Aufhebung",
    ],
    [
        "Observe / Questions",
        "Think / Traces",
        "Plan / Challenges",
        "Build / Patterns",
        "Execute / Discovery",
        "Verify / Insight",
    ],
    [
        "Arche",
        "Apokalypsis",
        "Dynamis",
        "Sophia",
        "Parousia",
        "Epi-Logos",
    ],
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolePolicy {
    pub weight: f64,
    /// Exact LENSES order; no fallback affinity is inferred at runtime.
    pub lens_affinities: [f64; 12],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum YPolicy {
    Consonant,
    Vowel,
}

/// A selected numerical policy, not a claim of canonical identity weights.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncodingPolicy {
    pub policy_ref: String,
    pub y_policy: YPolicy,
    pub full_pass_factor: f64,
    pub anchor_factor: f64,
    pub inverse_factor: f64,
    pub square_factor: f64,
    pub mobius_factor: f64,
    pub spanda_factor: f64,
    pub tritone_factor: f64,
    pub position_element_factor: f64,
    pub lens_element_factor: f64,
    pub cap_factor: f64,
    /// Chosen interpretation of the draft's "apply directElementBonus":
    /// multiply the L2p position/cap contribution, not the lens-native colour.
    pub direct_element_multiplier: f64,
    pub roles: BTreeMap<String, RolePolicy>,
}

impl Default for EncodingPolicy {
    fn default() -> Self {
        let roles = [
            (
                "full_name",
                5.,
                [
                    1.15, 1., 0.8, 0.8, 0.95, 1.25, 1.25, 0.8, 1., 0.8, 0.8, 1.20,
                ],
            ),
            (
                "vowel_total",
                4.,
                [
                    0.75, 0.75, 0.75, 1.05, 1.15, 1.25, 0.75, 1.20, 1.10, 0.75, 0.75, 0.75,
                ],
            ),
            (
                "consonant_total",
                3.,
                [
                    0.75, 1.20, 0.75, 0.75, 1., 1.05, 0.75, 0.75, 1., 0.75, 1.10, 0.75,
                ],
            ),
            (
                "date_digit_total",
                5.,
                [
                    0.75, 1.05, 0.75, 1.20, 0.75, 0.75, 0.75, 0.75, 1.05, 1.25, 1.10, 0.75,
                ],
            ),
            (
                "day",
                3.,
                [
                    1.20, 0.70, 0.70, 0.70, 0.70, 0.70, 0.70, 0.70, 1.05, 0.95, 0.70, 1.10,
                ],
            ),
            (
                "month",
                2.,
                [
                    0.70, 0.70, 0.70, 0.70, 1.05, 0.70, 0.70, 1.05, 0.70, 1.20, 0.70, 0.70,
                ],
            ),
            (
                "year",
                2.,
                [
                    0.70, 0.70, 0.70, 0.70, 0.70, 0.70, 0.70, 0.70, 0.70, 1.20, 1.15, 1.05,
                ],
            ),
            (
                "name_date_synthesis",
                6.,
                [
                    0.85, 1.05, 0.85, 1.05, 0.85, 1.15, 0.85, 0.85, 1.30, 0.85, 0.85, 1.15,
                ],
            ),
        ]
        .into_iter()
        .map(|(role, weight, lens_affinities)| {
            (
                role.into(),
                RolePolicy {
                    weight,
                    lens_affinities,
                },
            )
        })
        .collect();
        Self {
            policy_ref: "ql.nara.birthdate-draft-initial/v1".into(),
            y_policy: YPolicy::Consonant,
            full_pass_factor: 0.35,
            anchor_factor: 1.,
            inverse_factor: 0.5,
            square_factor: 0.25,
            mobius_factor: 0.40,
            spanda_factor: 0.35,
            tritone_factor: 0.30,
            position_element_factor: 1.,
            lens_element_factor: 0.35,
            cap_factor: 1.,
            direct_element_multiplier: 1.5,
            roles,
        }
    }
}

impl EncodingPolicy {
    pub fn validate(&self) -> Result<(), String> {
        super::text(&self.policy_ref, "identity encoding policy reference")?;
        let coefficients = [
            self.full_pass_factor,
            self.anchor_factor,
            self.inverse_factor,
            self.square_factor,
            self.mobius_factor,
            self.spanda_factor,
            self.tritone_factor,
            self.position_element_factor,
            self.lens_element_factor,
            self.cap_factor,
            self.direct_element_multiplier,
        ];
        if coefficients
            .iter()
            .chain(
                self.roles
                    .values()
                    .flat_map(|r| std::iter::once(&r.weight).chain(r.lens_affinities.iter())),
            )
            .any(|v| !v.is_finite() || *v < 0.)
        {
            return Err("identity encoding coefficients must be finite and non-negative".into());
        }
        for role in self.roles.keys() {
            if ![
                "full_name",
                "vowel_total",
                "consonant_total",
                "date_digit_total",
                "day",
                "month",
                "year",
                "name_date_synthesis",
                "word_component",
                "letter_stream_item",
                "inclusion_count",
            ]
            .contains(&role.as_str())
            {
                return Err(format!("unsupported identity encoding datum role: {role}"));
            }
        }
        Ok(())
    }
}

fn lens_from_chromatic(c: usize) -> usize {
    c / 2 + (c % 2) * 6
}
fn chromatic_from_lens(l: usize) -> usize {
    (l % 6) * 2 + l / 6
}
fn digit_sum(n: u64) -> u64 {
    n.to_string().bytes().map(|b| u64::from(b - b'0')).sum()
}

fn compound(n: u64) -> Value {
    let mut steps = vec![n];
    let mut root = n;
    while root > 9 {
        root = digit_sum(root);
        steps.push(root);
    }
    json!({"raw":n,"steps":steps,"root9":root,
        "master_flags":steps.iter().filter(|n| [11,22,33].contains(n)).collect::<Vec<_>>(),
        "karmic_debt_flags":steps.iter().filter(|n| [13,14,16,19].contains(n)).collect::<Vec<_>>()})
}

/// The source's full-pass, anchor and square equations. Matrices stay separate
/// so another authored numeric system can reuse the operation without a new
/// adapter or an invented role-to-lens mapping.
#[derive(Debug, Clone, Serialize)]
pub struct Refraction {
    pub full_pass: Matrix,
    pub anchor_bias: Matrix,
    pub square_diffuse: Matrix,
    pub total: Matrix,
}

pub fn refract_datum(
    n: u64,
    role: &RolePolicy,
    policy: &EncodingPolicy,
) -> Result<Refraction, String> {
    policy.validate()?;
    if !role.weight.is_finite()
        || role.weight < 0.
        || role
            .lens_affinities
            .iter()
            .any(|a| !a.is_finite() || *a < 0.)
    {
        return Err("invalid refraction role policy".into());
    }
    let p = (n % 6) as usize;
    let inverse = 5 - p;
    let lens = lens_from_chromatic((n % 12) as usize);
    let mut r = Refraction {
        full_pass: [[0.; 6]; 12],
        anchor_bias: [[0.; 6]; 12],
        square_diffuse: [[0.; 6]; 12],
        total: [[0.; 6]; 12],
    };
    for l in 0..12 {
        let score = role.weight * policy.full_pass_factor * role.lens_affinities[l];
        r.full_pass[l][p] += score;
        r.full_pass[l][inverse] += score * policy.inverse_factor;
    }
    r.anchor_bias[lens][p] += role.weight * policy.anchor_factor;
    r.anchor_bias[lens][inverse] += role.weight * policy.anchor_factor * policy.inverse_factor;
    let index = lens % 6;
    for l in [index, 5 - index, index + 6, 11 - index] {
        r.square_diffuse[l][p] += role.weight * policy.square_factor;
        r.square_diffuse[l][inverse] += role.weight * policy.square_factor * policy.inverse_factor;
    }
    let mobius = 5 - index + (1 - lens / 6) * 6;
    let partner = (lens + 6) % 12;
    let tritone = lens_from_chromatic((chromatic_from_lens(lens) + 6) % 12);
    r.square_diffuse[mobius][p] += role.weight * policy.mobius_factor;
    r.square_diffuse[partner][p] += role.weight * policy.spanda_factor;
    r.square_diffuse[tritone][p] += role.weight * policy.tritone_factor;
    for l in 0..12 {
        for p in 0..6 {
            r.total[l][p] = r.full_pass[l][p] + r.anchor_bias[l][p] + r.square_diffuse[l][p];
            if !r.total[l][p].is_finite() {
                return Err("identity refraction overflow".into());
            }
        }
    }
    Ok(r)
}

/// Current EFWA order; caps remain separate from quaternion components.
pub fn elemental_from_matrix(matrix: &Matrix, policy: &EncodingPolicy) -> Result<Value, String> {
    policy.validate()?;
    // Four EFWA components followed by Aether and Mineral caps.
    let position_element = [4, 0, 2, 3, 1, 5];
    let lens_element = [4, 0, 3, 2, 0, 4, 4, 2, 4, 1, 0, 4];
    let mut scores = [0.; 6];
    for l in 0..12 {
        for p in 0..6 {
            let s = matrix[l][p];
            if !s.is_finite() || s < 0. {
                return Err("invalid elemental source cell".into());
            }
            let e = position_element[p];
            let factor = if e < 4 {
                policy.position_element_factor
            } else {
                policy.cap_factor
            };
            scores[e] += s
                * factor
                * if l == 8 {
                    policy.direct_element_multiplier
                } else {
                    1.
                };
            scores[lens_element[l]] += s * policy.lens_element_factor;
        }
    }
    if scores.iter().any(|s| !s.is_finite()) {
        return Err("identity elemental extraction overflow".into());
    }
    // Preserve the raw extraction; scale only for the norms, after all cells.
    // A finite selected policy may produce scores whose squares overflow.
    let max = scores[..4].iter().copied().fold(0., f64::max);
    let scaled = if max > 0. {
        [
            scores[0] / max,
            scores[1] / max,
            scores[2] / max,
            scores[3] / max,
        ]
    } else {
        [0.; 4]
    };
    let total: f64 = scaled.iter().sum();
    let quaternion = if max > 0. {
        Some(
            BioQuaternion {
                w: scaled[0],
                x: scaled[1],
                y: scaled[2],
                z: scaled[3],
            }
            .normalized()?,
        )
    } else {
        None
    };
    let balance = if total > 0. {
        Some([
            scaled[0] / total,
            scaled[1] / total,
            scaled[2] / total,
            scaled[3] / total,
        ])
    } else {
        None
    };
    Ok(
        json!({"basis":["earth","fire","water","air"],"raw_efwa":&scores[..4],"balance_efwa":balance,"quaternion":quaternion,
        "caps":{"aether_gate":scores[4],"mineral_cap":scores[5]},"normalization":"L1 balance; L2 real quaternion after all cells; caps excluded",
        "status":if quaternion.is_some(){"available"}else{"unavailable"},"absence_reason":if quaternion.is_none(){Some("No elemental mass under the selected policy")}else{None}}),
    )
}

fn matrix_json(matrix: &Matrix) -> Value {
    Value::Object(
        LENSES
            .iter()
            .enumerate()
            .map(|(l, name)| ((*name).into(), json!(matrix[l])))
            .collect(),
    )
}

pub fn derive(profile: &IdentityProfile, policy: &EncodingPolicy) -> Result<Value, String> {
    profile.validate()?;
    policy.validate()?;
    let mut data: Vec<(String, String, u64)> = Vec::new();
    let mut absent = Vec::<String>::new();
    let mut letters = Vec::new();
    let mut words = Vec::new();
    let mut counts = [0u64; 9];
    let mut full_name = None;
    if profile
        .name
        .chars()
        .any(|c| !c.is_ascii() && !c.is_whitespace() && !matches!(c, '‐' | '‑' | '’' | '‘'))
    {
        absent.push("Supplied name has unsupported non-ASCII characters; an explicit transliteration or dedicated alphabet policy is required".into());
    } else {
        let mut total = 0;
        let mut vowel = 0;
        for (word_index, word) in profile
            .name
            .split(|c: char| c.is_whitespace() || matches!(c, '-' | '‐' | '‑'))
            .filter(|s| !s.is_empty())
            .enumerate()
        {
            let mut word_total = 0;
            let mut has_letters = false;
            for letter in word
                .chars()
                .filter(char::is_ascii_alphabetic)
                .map(|c| c.to_ascii_uppercase())
            {
                has_letters = true;
                let n = u64::from((letter as u8 - b'A') % 9 + 1);
                let is_vowel = "AEIOU".contains(letter)
                    || (letter == 'Y' && matches!(policy.y_policy, YPolicy::Vowel));
                if is_vowel {
                    vowel += n;
                }
                word_total += n;
                total += n;
                counts[(n - 1) as usize] += 1;
                let id = format!("letter:{}", letters.len());
                data.push((id.clone(), "letter_stream_item".into(), n));
                letters.push(json!({"id":id,"letter":letter.to_string(),"value":n,"mod6":n%6,"mod12":n%12,"word_index":word_index,"vowel":is_vowel}));
            }
            if has_letters {
                words.push(json!({"word":word,"value":word_total,"compound":compound(word_total)}));
                data.push((
                    format!("word:{word_index}"),
                    "word_component".into(),
                    word_total,
                ));
            }
        }
        if !letters.is_empty() {
            full_name = Some(total);
            data.extend([
                ("full_name".into(), "full_name".into(), total),
                ("vowel_total".into(), "vowel_total".into(), vowel),
                (
                    "consonant_total".into(),
                    "consonant_total".into(),
                    total - vowel,
                ),
            ]);
            for (i, n) in counts.iter().enumerate() {
                data.push((format!("inclusion:{}", i + 1), "inclusion_count".into(), *n));
            }
        } else {
            absent.push("Supplied name has no encodable alphabetic letters".into());
        }
    }
    let mut date_total = None;
    let mut raw_year = None;
    if let Some(date) = &profile.birth.date {
        let year = date[..4]
            .parse::<u64>()
            .map_err(|_| "invalid encoding year")?;
        raw_year =
            Some(json!({"year":year,"mod6":year%6,"mod12":year%12,"compound":compound(year)}));
        data.push(("year_digit_total".into(), "year".into(), digit_sum(year)));
        if date.len() >= 7 {
            data.push((
                "month".into(),
                "month".into(),
                date[5..7].parse().map_err(|_| "invalid encoding month")?,
            ));
        }
        if date.len() == 10 {
            data.push((
                "day".into(),
                "day".into(),
                date[8..10].parse().map_err(|_| "invalid encoding day")?,
            ));
            let total = date
                .bytes()
                .filter(u8::is_ascii_digit)
                .map(|b| u64::from(b - b'0'))
                .sum();
            date_total = Some(total);
            data.push(("date_digit_total".into(), "date_digit_total".into(), total));
        } else {
            absent.push(
                "Full birth date unavailable; day/date total and name-date synthesis remain absent"
                    .into(),
            );
        }
    } else {
        absent.push(
            "Birth date not supplied; temporal numeric data and name-date synthesis remain absent"
                .into(),
        );
    }
    if let (Some(name), Some(date)) = (full_name, date_total) {
        data.push((
            "name_date_synthesis".into(),
            "name_date_synthesis".into(),
            name + date,
        ));
    }
    let mut aggregate = Refraction {
        full_pass: [[0.; 6]; 12],
        anchor_bias: [[0.; 6]; 12],
        square_diffuse: [[0.; 6]; 12],
        total: [[0.; 6]; 12],
    };
    let mut evidence = Vec::new();
    for (id, role, n) in data {
        let p = (n % 6) as usize;
        let lens = lens_from_chromatic((n % 12) as usize);
        let mut cells = Vec::new();
        if let Some(role_policy) = policy.roles.get(&role) {
            let contribution = refract_datum(n, role_policy, policy)?;
            // (lens, position) indexes five parallel 12x6 grids at once.
            #[allow(clippy::needless_range_loop)]
            for l in 0..12 {
                for p in 0..6 {
                    aggregate.full_pass[l][p] += contribution.full_pass[l][p];
                    aggregate.anchor_bias[l][p] += contribution.anchor_bias[l][p];
                    aggregate.square_diffuse[l][p] += contribution.square_diffuse[l][p];
                    aggregate.total[l][p] += contribution.total[l][p];
                    if contribution.total[l][p] > 0. {
                        cells.push(json!({"lens":LENSES[l],"position":p,"meaning":CELL_MEANINGS[l][p],"full_pass":contribution.full_pass[l][p],"anchor_bias":contribution.anchor_bias[l][p],"square_diffuse":contribution.square_diffuse[l][p],"total":contribution.total[l][p]}));
                    }
                }
            }
        }
        let index = lens % 6;
        evidence.push(json!({"id":id,"role":role,"raw":n,"compound":compound(n),"mod6":p,"inverse":5-p,"mod12":n%12,"anchor_lens":LENSES[lens],"direct_cell_meaning":CELL_MEANINGS[lens][p],
            "relations":{"square":if index==0||index==5{"A"}else if index==1||index==4{"B"}else{"C"},"spanda":LENSES[(lens+6)%12],"complement":LENSES[5-index+(lens/6)*6],"mobius":LENSES[5-index+(1-lens/6)*6],"tritone":LENSES[lens_from_chromatic(((n%12)as usize+6)%12)]},
            "weighted":policy.roles.contains_key(&role),"unweighted_reason":if policy.roles.contains_key(&role){None}else{Some("Selected policy supplies no affinity row for this role; numeric evidence retained without invented coefficients")},"cells":cells}));
    }
    let elemental = elemental_from_matrix(&aggregate.total, policy)?;
    Ok(
        json!({"schema":"ql.nara-birthdate-encoding/v1","status":if elemental["status"]=="unavailable"{"unavailable"}else if absent.is_empty(){"available"}else{"partial"},
        "standing":"derived-under-selected-provisional-policy","coordinate":"M4-0-0","name_role":"supplied-name","supplied_name":profile.name,"birth_date":profile.birth.date,
        "source":{"repository":"EpiLogos/Epi-Logos-C-Experiments","path":"Idea/Bimba/Seeds/M/M4'/nara-m4-0-0-birthdate-encoding-spec.md","commit":"daa660cbc1b8c5da83828698665a753852cb0287","blob":SOURCE_BLOB,"sections":["4-5","8-12"],"standing":"provisional-draft"},
        "policy":policy,"policy_notes":["Direct L2p bonus is selected as a multiplier of the position/cap contribution; the source leaves the bonus operation unspecified","Master and karmic numbers are flags, with no extra numeric multiplier","Word, letter and inclusion data require explicit affinity rows before contributing","Source EWFA component notation is corrected to the current EFWA contract"],
        "numeric":{"full_name_total":full_name,"date_digit_total":date_total,"letter_stream":letters,"word_components":words,"inclusion_counts_1_to_9":if full_name.is_some(){Some(counts)}else{None},"year_raw_mods":raw_year},
        "lens_order":LENSES,"matrices":{"full_pass":matrix_json(&aggregate.full_pass),"anchor_bias":matrix_json(&aggregate.anchor_bias),"square_diffuse":matrix_json(&aggregate.square_diffuse),"total":matrix_json(&aggregate.total)},
        "elemental":elemental,"evidence":evidence,"absence_reasons":absent}),
    )
}

#[cfg(test)]
mod tests {
    use super::super::intake::{BirthData, PROFILE_SCHEMA, TimePrecision};
    use super::*;

    fn profile(name: &str, date: Option<&str>) -> IdentityProfile {
        IdentityProfile {
            schema: PROFILE_SCHEMA.into(),
            person_ref: "central:person:encoding-test".into(),
            nara_ref: "ql:nara:encoding-test".into(),
            name: name.into(),
            encoding_policy: None,
            composition_policy: None,
            birth: BirthData {
                date: date.map(str::to_owned),
                time: None,
                precision: TimePrecision::Unknown,
                uncertainty_minutes: None,
                fold: None,
                place: None,
            },
            jungian: None,
            gene_keys: None,
            human_design: None,
            quintessence: None,
        }
    }

    #[test]
    fn numeric_sources_retain_raw_compounds_and_full_refraction() {
        let result = derive(
            &profile("A-J", Some("2000-01-01")),
            &EncodingPolicy::default(),
        )
        .unwrap();
        assert_eq!(result["numeric"]["full_name_total"], 2);
        assert_eq!(result["numeric"]["date_digit_total"], 4);
        assert_eq!(
            result["numeric"]["word_components"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        let evidence = result["evidence"].as_array().unwrap();
        let synthesis = evidence
            .iter()
            .find(|d| d["role"] == "name_date_synthesis")
            .unwrap();
        assert_eq!(synthesis["raw"], 6);
        assert_eq!(synthesis["mod6"], 0);
        assert_eq!(synthesis["inverse"], 5);
        assert_eq!(synthesis["anchor_lens"], "L3");
        assert_eq!(synthesis["cells"].as_array().unwrap().len(), 24);
        assert_eq!(compound(38)["steps"], json!([38, 11, 2]));
        assert_eq!(compound(38)["master_flags"], json!([11]));
        assert_eq!(result["name_role"], "supplied-name");
    }

    #[test]
    fn source_equations_distinguish_square_mobius_spanda_and_tritone() {
        let policy = EncodingPolicy::default();
        let r = refract_datum(37, policy.roles.get("full_name").unwrap(), &policy).unwrap();
        // 37 -> P1, inverse P4, chromatic1 -> L0p; Square A has four lenses.
        assert!((r.full_pass[6][1] - 5. * 0.35 * 1.25).abs() < 1e-12);
        assert_eq!(r.anchor_bias[6][1], 5.);
        assert_eq!(r.anchor_bias[6][4], 2.5);
        assert_eq!(r.square_diffuse[5][1], 5. * (0.25 + 0.40)); // L5 Möbius
        assert_eq!(r.square_diffuse[0][1], 5. * (0.25 + 0.35)); // L0 partner
        assert_eq!(r.square_diffuse[9][1], 5. * 0.30); // L3p tritone, outside A
        assert_eq!(r.square_diffuse[11][4], 5. * 0.25 * 0.5);
    }

    #[test]
    fn extraction_uses_efwa_keeps_caps_and_applies_explicit_bonus() {
        let mut matrix = [[0.; 6]; 12];
        matrix[8][4] = 2.;
        matrix[8][0] = 3.;
        let e = elemental_from_matrix(&matrix, &EncodingPolicy::default()).unwrap();
        assert_eq!(e["raw_efwa"], json!([0., 3., 0., 0.]));
        assert_eq!(e["quaternion"], json!({"w":0.,"x":1.,"y":0.,"z":0.}));
        assert!((e["caps"]["aether_gate"].as_f64().unwrap() - 6.25).abs() < 1e-12);
        assert_eq!(e["caps"]["mineral_cap"], 0.);
    }

    #[test]
    fn policy_edits_change_native_output_and_absence_is_never_filled() {
        let input = profile("Ay", None);
        let mut policy = EncodingPolicy::default();
        let before = derive(&input, &policy).unwrap();
        policy.y_policy = YPolicy::Vowel;
        let after = derive(&input, &policy).unwrap();
        assert_ne!(
            before["elemental"]["quaternion"],
            after["elemental"]["quaternion"]
        );
        assert_eq!(after["numeric"]["date_digit_total"], Value::Null);
        assert!(
            !after["evidence"]
                .as_array()
                .unwrap()
                .iter()
                .any(|d| d["role"] == "name_date_synthesis")
        );
        assert_eq!(
            derive(&profile("Zoë", None), &policy).unwrap()["status"],
            "unavailable"
        );
        assert_eq!(
            derive(&profile("Zoe\u{301}", None), &policy).unwrap()["status"],
            "unavailable"
        );
        assert_eq!(
            derive(&profile("Zoë", Some("2000-01-01")), &policy).unwrap()["status"],
            "partial"
        );
        policy.full_pass_factor = f64::NAN;
        assert!(derive(&input, &policy).is_err());
    }

    #[test]
    fn intake_persists_explicit_policy_and_distinguishes_unselected_preview() {
        let mut input = profile("A-J", Some("2000-01-01"));
        let wire = serde_json::to_value(&input).unwrap();
        assert!(wire.get("encoding_policy").is_none());
        assert_eq!(
            serde_json::from_value::<IdentityProfile>(wire).unwrap(),
            input
        );
        let before = input.inspect(None).unwrap();
        assert_eq!(before["birthdate_encoding"]["selected"], false);
        let selected = EncodingPolicy {
            anchor_factor: 2.,
            ..EncodingPolicy::default()
        };
        input.encoding_policy = Some(selected);
        let roundtrip: IdentityProfile =
            serde_json::from_value(serde_json::to_value(&input).unwrap()).unwrap();
        let after = roundtrip.inspect(None).unwrap();
        assert_eq!(after["birthdate_encoding"]["selected"], true);
        assert_eq!(after["birthdate_encoding"]["policy"]["anchor_factor"], 2.);
        assert_eq!(
            after["matrix"][0]["data"]["encoding"],
            after["birthdate_encoding"]
        );
        assert_ne!(before["input_revision"], after["input_revision"]);
        assert_ne!(
            before["birthdate_encoding"]["elemental"]["quaternion"],
            after["birthdate_encoding"]["elemental"]["quaternion"]
        );
    }

    #[test]
    fn extraction_normalization_is_scale_invariant_and_rejects_overflow() {
        let policy = EncodingPolicy::default();
        let mut matrix = [[0.; 6]; 12];
        matrix[8][1] = 3.;
        matrix[8][4] = 4.;
        let ordinary = elemental_from_matrix(&matrix, &policy).unwrap();
        for scale in [1e200, 1e-200] {
            let scaled = matrix.map(|row| row.map(|n| n * scale));
            let result = elemental_from_matrix(&scaled, &policy).unwrap();
            for component in ["w", "x", "y", "z"] {
                assert!(
                    (ordinary["quaternion"][component].as_f64().unwrap()
                        - result["quaternion"][component].as_f64().unwrap())
                    .abs()
                        < 1e-12
                );
            }
        }
        matrix[8][4] = f64::MAX;
        assert!(
            elemental_from_matrix(&matrix, &policy)
                .unwrap_err()
                .contains("overflow")
        );
        matrix[8][4] = f64::NAN;
        assert!(elemental_from_matrix(&matrix, &policy).is_err());
    }
}
