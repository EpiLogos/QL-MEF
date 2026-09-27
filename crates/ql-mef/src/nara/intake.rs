//! Editable identity source. Central owns persistence and history; QL validates
//! and derives the six-office reading. A report is never a birth calculation.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

use super::{
    SourceRevision,
    domain::{
        EvidenceStanding, IdentityEvidenceSlot, IdentityField, IdentitySlotKind, ProtectedRef,
    },
};

pub const PROFILE_SCHEMA: &str = "ql.nara-identity-profile/v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthPlace {
    pub label: String,
    pub latitude_degrees: f64,
    pub longitude_degrees: f64,
    pub timezone: String,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimePrecision {
    Exact,
    Approximate,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthData {
    pub date: Option<String>,
    pub time: Option<String>,
    pub precision: TimePrecision,
    pub uncertainty_minutes: Option<u32>,
    pub fold: Option<u8>,
    pub place: Option<BirthPlace>,
}

/// Imported material retains its actual system/provider and source revision.
/// `data` is system-specific, validated below, and never silently inferred.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityReport {
    pub source: SourceRevision,
    pub method: String,
    pub route: ReportRoute,
    pub data: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReportRoute {
    Import,
    SelfReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityProfile {
    pub schema: String,
    pub person_ref: String,
    pub nara_ref: String,
    pub name: String,
    pub birth: BirthData,
    pub jungian: Option<IdentityReport>,
    pub gene_keys: Option<IdentityReport>,
    pub human_design: Option<IdentityReport>,
    pub quintessence: Option<IdentityReport>,
}

fn text(value: &str, label: &str) -> Result<(), String> {
    super::text(value, label)?;
    require(
        value.chars().count() <= 512 && !value.chars().any(|c| (c as u32) < 32),
        &format!("{label} must contain at most 512 characters and no control characters"),
    )
}
fn require(test: bool, reason: &str) -> Result<(), String> {
    if test { Ok(()) } else { Err(reason.into()) }
}
fn date(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.len() == 4 && bytes.iter().all(u8::is_ascii_digit) {
        return require(
            (1800..2400).contains(&value.parse::<u32>().map_err(|_| "invalid birth year")?),
            "birth year outside provider range",
        );
    }
    if bytes.len() == 7
        && bytes[4] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(i, c)| i == 4 || c.is_ascii_digit())
    {
        date(&format!("{value}-01"))?;
        return Ok(());
    }
    require(
        bytes.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes
                .iter()
                .enumerate()
                .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit()),
        "birth date must be YYYY-MM-DD",
    )?;
    let year = value[..4]
        .parse::<u32>()
        .map_err(|_| "invalid birth year")?;
    let month = value[5..7]
        .parse::<u32>()
        .map_err(|_| "invalid birth month")?;
    let day = value[8..].parse::<u32>().map_err(|_| "invalid birth day")?;
    require(
        (1800..2400).contains(&year) && (1..=12).contains(&month),
        "birth date outside provider range [1800,2400)",
    )?;
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    require(
        day > 0 && day <= days[(month - 1) as usize],
        "invalid calendar birth date",
    )
}

impl BirthData {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(value) = &self.date {
            date(value)?;
        }
        if let Some(value) = &self.time {
            let parts = value.split(':').collect::<Vec<_>>();
            require(
                (parts.len() == 2 || parts.len() == 3)
                    && parts
                        .iter()
                        .all(|p| p.len() == 2 && p.bytes().all(|b| b.is_ascii_digit())),
                "birth time must be HH:MM or HH:MM:SS",
            )?;
            let nums = parts
                .iter()
                .map(|p| p.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            require(
                nums[0] < 24 && nums[1] < 60 && (nums.len() == 2 || nums[2] < 60),
                "invalid civil birth time",
            )?;
        }
        match self.precision {
            TimePrecision::Unknown => require(
                self.time.is_none() && self.uncertainty_minutes.is_none() && self.fold.is_none(),
                "unknown birth time must not carry an invented clock time, fold or precision",
            )?,
            TimePrecision::Exact => require(
                self.time.is_some() && self.uncertainty_minutes.is_none(),
                "exact birth time requires a time and no uncertainty interval",
            )?,
            TimePrecision::Approximate => require(
                self.time.is_some()
                    && self
                        .uncertainty_minutes
                        .is_some_and(|v| (1..=1440).contains(&v)),
                "approximate birth time requires an uncertainty of 1..1440 minutes",
            )?,
        }
        require(
            self.fold.is_none_or(|v| v <= 1),
            "DST fold must be zero or one",
        )?;
        if let Some(place) = &self.place {
            text(&place.label, "birthplace")?;
            text(&place.source_ref, "birthplace source")?;
            text(&place.timezone, "birthplace IANA timezone")?;
            require(
                place.latitude_degrees.is_finite()
                    && (-90.0..=90.0).contains(&place.latitude_degrees),
                "invalid birthplace latitude",
            )?;
            require(
                place.longitude_degrees.is_finite()
                    && (-180.0..=180.0).contains(&place.longitude_degrees),
                "invalid birthplace longitude",
            )?;
        }
        Ok(())
    }
}

impl IdentityReport {
    fn validate(&self) -> Result<(), String> {
        self.source.validate()?;
        text(&self.method, "identity report method")?;
        require(
            self.data.is_object(),
            "identity report data must be an object",
        )?;
        require(
            serde_json::to_vec(&self.data)
                .map_err(|e| e.to_string())?
                .len()
                <= 256 * 1024,
            "identity report exceeds 256 KiB",
        )
    }
}

fn integers(value: &Value, key: &str, min: u64, max: u64) -> Result<(), String> {
    let values = value[key]
        .as_array()
        .ok_or_else(|| format!("{key} must be an array"))?;
    let mut seen = BTreeSet::new();
    for number in values {
        let n = number
            .as_u64()
            .ok_or_else(|| format!("{key} requires integer identifiers"))?;
        require(
            (min..=max).contains(&n) && seen.insert(n),
            &format!("{key} contains duplicate or out-of-range identifiers"),
        )?;
    }
    Ok(())
}

impl IdentityProfile {
    pub fn validate(&self) -> Result<(), String> {
        require(
            self.schema == PROFILE_SCHEMA,
            "unsupported identity profile schema",
        )?;
        text(&self.person_ref, "person reference")?;
        text(&self.nara_ref, "Nara reference")?;
        text(&self.name, "name")?;
        self.birth.validate()?;
        for report in [
            &self.jungian,
            &self.gene_keys,
            &self.human_design,
            &self.quintessence,
        ]
        .into_iter()
        .flatten()
        {
            report.validate()?;
        }
        if let Some(report) = &self.jungian {
            let system = report.data["system"]
                .as_str()
                .ok_or("Jungian report requires its actual system")?;
            require(
                ["jungian", "mbti", "16-personalities"].contains(&system),
                "unsupported Jungian report system",
            )?;
            let kind = report.data["type"]
                .as_str()
                .ok_or("Jungian report requires its declared type")?;
            let b = kind.as_bytes();
            require(
                b.len() == 4
                    && b"IE".contains(&b[0])
                    && b"NS".contains(&b[1])
                    && b"FT".contains(&b[2])
                    && b"JP".contains(&b[3]),
                "declared personality type must be one of the sixteen four-letter types",
            )?;
            if let Some(identity) = report.data.get("identity") {
                require(
                    system == "16-personalities" && matches!(identity.as_str(), Some("A" | "T")),
                    "A/T identity belongs only to a 16-personalities report",
                )?;
            }
        }
        if let Some(report) = &self.gene_keys {
            require(
                report.route == ReportRoute::Import,
                "Gene Keys requires an attributable import, not a birth-derived or self-report guess",
            )?;
            let spheres = report.data["spheres"]
                .as_array()
                .ok_or("Gene Keys import requires spheres")?;
            require(
                !spheres.is_empty() && spheres.len() <= 64,
                "Gene Keys import requires 1..64 spheres",
            )?;
            let mut names = BTreeSet::new();
            for sphere in spheres {
                let name = sphere["name"]
                    .as_str()
                    .ok_or("Gene Keys sphere requires a name")?;
                text(name, "sphere name")?;
                require(names.insert(name), "duplicate Gene Keys sphere")?;
                require(
                    sphere["key"]
                        .as_u64()
                        .is_some_and(|v| (1..=64).contains(&v))
                        && sphere["line"]
                            .as_u64()
                            .is_some_and(|v| (1..=6).contains(&v)),
                    "Gene Keys require key 1..64 and line 1..6",
                )?;
            }
        }
        if let Some(report) = &self.human_design {
            require(
                report.route == ReportRoute::Import,
                "Human Design requires an attributable BodyGraph import",
            )?;
            for field in ["type", "strategy", "authority", "profile", "definition"] {
                text(
                    report.data[field]
                        .as_str()
                        .ok_or_else(|| format!("Human Design import requires {field}"))?,
                    field,
                )?;
            }
            integers(&report.data, "personality_gates", 1, 64)?;
            integers(&report.data, "design_gates", 1, 64)?;
            let centres = report.data["defined_centres"]
                .as_array()
                .ok_or("Human Design import requires defined_centres")?;
            let known = [
                "head",
                "ajna",
                "throat",
                "g",
                "heart",
                "spleen",
                "solar-plexus",
                "sacral",
                "root",
            ];
            let mut seen = BTreeSet::new();
            for centre in centres {
                let name = centre.as_str().ok_or("Human Design centre must be named")?;
                require(
                    known.contains(&name) && seen.insert(name),
                    "Human Design centre must be one distinct native BodyGraph centre",
                )?;
            }
            let channels = report.data["channels"]
                .as_array()
                .ok_or("Human Design import requires its channels as two-gate arrays")?;
            let mut seen = BTreeSet::new();
            for channel in channels {
                let pair = channel
                    .as_array()
                    .filter(|p| p.len() == 2)
                    .ok_or("Human Design channel must be a two-gate array")?;
                let a = pair[0]
                    .as_u64()
                    .filter(|v| (1..=64).contains(v))
                    .ok_or("invalid Human Design channel gate")?;
                let b = pair[1]
                    .as_u64()
                    .filter(|v| (1..=64).contains(v))
                    .ok_or("invalid Human Design channel gate")?;
                require(
                    a != b && seen.insert((a.min(b), a.max(b))),
                    "duplicate or self-connected Human Design channel",
                )?;
                for gate in [a, b] {
                    require(
                        ["personality_gates", "design_gates"].iter().any(|key| {
                            report.data[key]
                                .as_array()
                                .is_some_and(|g| g.contains(&json!(gate)))
                        }),
                        "channel gate is absent from imported personality and design gates",
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Semantic source revision: provider receipt time, transits, activity and
    /// renderer state cannot change it. Correcting any source does change it.
    pub fn revision(&self) -> Result<String, String> {
        self.validate()?;
        let value = serde_json::to_value(self).map_err(|e| e.to_string())?;
        let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
        Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
    }

    pub fn natal_request(&self) -> Result<Value, String> {
        Ok(
            json!({"schema":"ql.nara-natal-request/v1","person_ref":self.person_ref,"source_revision":self.revision()?,"birth":self.birth,"backend_policy":"allow-moshier"}),
        )
    }

    /// Produce the accepted six-office field from actual source material.
    /// Calculated natal output must belong to this exact person and input.
    pub fn inspect(&self, natal: Option<&Value>) -> Result<Value, String> {
        let revision = self.revision()?;
        if let Some(natal) = natal {
            require(
                natal["schema"] == "ql.nara-natal/v1"
                    && natal["request"] == self.natal_request()?,
                "natal result does not belong to this exact identity input",
            )?;
            require(
                matches!(
                    natal["status"].as_str(),
                    Some("available" | "partial" | "unavailable")
                ),
                "unsupported natal receipt status",
            )?;
            if natal["chart"].is_object() {
                let chart = &natal["chart"];
                let svg = chart["svg"]
                    .as_str()
                    .filter(|s| s.len() <= 4 * 1024 * 1024 && s.contains("<svg"))
                    .ok_or("natal chart has no bounded SVG")?;
                require(
                    natal["status"] == "available"
                        && natal["sky"].is_object()
                        && chart["media_type"] == "image/svg+xml"
                        && chart["sha256"] == format!("{:x}", Sha256::digest(svg.as_bytes()))
                        && chart["bodies"].as_array().is_some_and(|v| v.len() == 10)
                        && chart["houses"].as_array().is_some_and(|v| v.len() == 12)
                        && chart["precision"]
                            == serde_json::to_value(&self.birth.precision)
                                .map_err(|e| e.to_string())?,
                    "natal chart receipt is incomplete or changed",
                )?;
            } else {
                require(
                    natal["chart"].is_null()
                        && natal["status"] != "available"
                        && natal["reason"].as_str().is_some_and(|s| !s.is_empty()),
                    "unavailable natal chart requires a reason",
                )?;
            }
        }
        let kinds = [
            IdentitySlotKind::BirthdateName,
            IdentitySlotKind::NatalChart,
            IdentitySlotKind::JungianAssessment,
            IdentitySlotKind::GeneKeys,
            IdentitySlotKind::HumanDesign,
            IdentitySlotKind::ArchetypalQuintessence,
        ];
        let reports = [
            None,
            None,
            self.jungian.as_ref(),
            self.gene_keys.as_ref(),
            self.human_design.as_ref(),
            self.quintessence.as_ref(),
        ];
        let mut slots = Vec::new();
        let mut matrix = Vec::new();
        for (i, kind) in kinds.into_iter().enumerate() {
            let report = reports[i];
            let present = i == 0
                || (i == 1 && natal.is_some_and(|n| n["chart"].is_object()))
                || report.is_some();
            let value = if i == 0 {
                json!({"name":self.name,"birth":self.birth})
            } else if i == 1 {
                natal.cloned().unwrap_or(Value::Null)
            } else {
                report.map(|r| r.data.clone()).unwrap_or(Value::Null)
            };
            let source = report.map(|r| r.source.clone()).unwrap_or(SourceRevision {
                source_ref: self.person_ref.clone(),
                revision: revision.clone(),
                standing_ref: if i == 1 { "derived" } else { "reported" }.into(),
            });
            let reason = if i == 1 {
                natal
                    .and_then(|n| n["reason"].as_str())
                    .unwrap_or("Natal calculation has not run for this input")
            } else {
                "Not supplied"
            };
            let standing = if !present {
                EvidenceStanding::Unavailable
            } else if i == 1 {
                EvidenceStanding::Derived
            } else {
                EvidenceStanding::Reported
            };
            slots.push(IdentityEvidenceSlot {
                kind,
                coordinate_ref: kind.coordinate().into(),
                source: present.then_some(source.clone()),
                protected_value_ref: present.then(|| ProtectedRef {
                    ref_id: format!("{}#{}", self.person_ref, kind.coordinate()),
                    revision: revision.clone(),
                    owner_ref: "central".into(),
                }),
                evidence_refs: vec![],
                tensions: vec![],
                absence_reason: (!present).then(|| reason.into()),
                standing,
            });
            matrix.push(json!({"kind":kind,"coordinate":kind.coordinate(),"available":present,"route":if i==0 {"entered-source"}else if i==1 {"calculation"}else if report.is_some_and(|r|r.route==ReportRoute::SelfReport){"self-report"}else{"import"},"source":present.then_some(source),"method":report.map(|r|r.method.as_str()),"data":value,"absence_reason":if present {Value::Null}else{json!(reason)}}));
        }
        let identity = IdentityField {
            identity_revision: revision.clone(),
            slots,
            identity_hash_ref: None,
            identity_quaternion_ref: None,
            m3_form_address_ref: None,
            derivation_refs: vec![],
        };
        identity.validate()?;
        let material =
            super::identity_material::material(&self.nara_ref, &self.person_ref, &identity)?;
        let natal_composition = natal
            .filter(|n| n["sky"].is_object())
            .map(super::intake_composition::natal_composition)
            .transpose()?;
        Ok(
            json!({"schema":"ql.nara-identity-reading/v1","person_ref":self.person_ref,"nara_ref":self.nara_ref,"input_revision":revision,"profile":self,"identity":identity,"material":material,"matrix":matrix,"natal":natal,"natal_composition":natal_composition,"private":true,"public_export":false}),
        )
    }
}
