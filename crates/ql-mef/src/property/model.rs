//! Rich operational values over existing source-defined property identities.
//! Original enums mirror the executed graph-schema source, not a new C canon.
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::m_tree::{MRegistry, MTreeId};

pub type Result<T> = std::result::Result<T, String>;
pub const PROPERTY_VERSION: &str = "ql.bimba-property/v1";
pub const MAX_VALUE_BYTES: usize = 262_144;
pub const MAX_REFERENCES: usize = 256;

pub(crate) fn require(ok: bool, message: &str) -> Result<()> {
    if ok { Ok(()) } else { Err(message.into()) }
}

pub(crate) fn reference(value: &str) -> Result<()> {
    require(
        !value.is_empty() && value == value.trim() && value.len() <= 4096 && !value.contains('\0'),
        "invalid or excessive property reference",
    )
}

pub(crate) fn bounded(value: &Value) -> Result<()> {
    let mut pending = vec![(value, 0usize)];
    let mut count = 0usize;
    while let Some((value, depth)) = pending.pop() {
        count += 1;
        require(
            count <= 32_768 && depth <= 32,
            "property value structural budget exceeded",
        )?;
        match value {
            Value::Array(items) => pending.extend(items.iter().map(|v| (v, depth + 1))),
            Value::Object(items) => pending.extend(items.values().map(|v| (v, depth + 1))),
            _ => {}
        }
    }
    require(
        serde_json::to_vec(value).map_err(|e| e.to_string())?.len() <= MAX_VALUE_BYTES,
        "property value byte budget exceeded",
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyOwner {
    Node,
    Relationship,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    String,
    StringList,
    Integer,
    Float,
    Boolean,
    DateTime,
    JsonString,
    Embedding,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cardinality {
    One,
    Many,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Disclosure {
    Public,
    Internal,
    Protected,
}

/// Exact fields exported by the original 194-entry registry. Coordinate home
/// remains declaration provenance; applicability is supplied separately below.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePropertySpec {
    pub key: String,
    pub coordinate_home: String,
    pub owner: PropertyOwner,
    pub value_type: PropertyType,
    pub cardinality: Cardinality,
    pub disclosure: Disclosure,
    pub source_family: String,
    pub indexed: bool,
    pub compatibility: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceUse {
    Current,
    Historical,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePin {
    pub reference: String,
    pub revision: String,
    pub use_mode: SourceUse,
}
impl SourcePin {
    pub fn validate(&self) -> Result<()> {
        reference(&self.reference)?;
        reference(&self.revision)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionRef {
    pub identity: String,
    pub revision: u64,
}
impl VersionRef {
    pub fn validate(&self) -> Result<()> {
        reference(&self.identity)?;
        require(self.revision > 0, "zero is not a property revision")
    }
}

/// Uses the existing node/relation ID verbatim. No lexical prefix matching,
/// alias rewriting, native subject creation or second coordinate registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    pub registry_revision: String,
    pub owner: PropertyOwner,
    pub id: MTreeId,
}
impl Subject {
    pub fn validate(&self, registry: &MRegistry) -> Result<()> {
        require(
            self.registry_revision == registry.manifest().registry_revision,
            "property subject registry revision is stale",
        )?;
        require(
            match self.owner {
                PropertyOwner::Node => registry.node(self.id).is_some(),
                PropertyOwner::Relationship => registry.relation(self.id).is_some(),
            },
            "property subject is absent or has the wrong node/relation kind",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    pub reference: VersionRef,
    pub declaration: SourcePin,
    pub spec: SourcePropertySpec,
    pub aliases: Vec<String>,
    pub applies_to: Vec<Subject>,
    pub applicability_basis: SourcePin,
    pub unit: Option<String>,
    /// When present these are allowed complete values, not inferred conversions.
    pub domain: Vec<Value>,
    pub embedding_dimensions: Option<usize>,
}
impl Definition {
    pub fn validate(&self, registry: &MRegistry) -> Result<()> {
        self.reference.validate()?;
        self.declaration.validate()?;
        self.applicability_basis.validate()?;
        for item in [
            &self.spec.key,
            &self.spec.coordinate_home,
            &self.spec.source_family,
        ] {
            reference(item)?;
        }
        require(
            !self.applies_to.is_empty() && self.applies_to.len() <= MAX_REFERENCES,
            "property applicability must name a bounded nonempty subject set",
        )?;
        let mut subjects = BTreeSet::new();
        for subject in &self.applies_to {
            subject.validate(registry)?;
            require(
                subject.owner == self.spec.owner && subjects.insert(subject.id),
                "duplicate or wrong-kind property applicability",
            )?;
        }
        require(
            self.aliases.len() <= MAX_REFERENCES && self.domain.len() <= MAX_REFERENCES,
            "property alias/domain budget exceeded",
        )?;
        let mut names = BTreeSet::from([self.spec.key.as_str()]);
        for alias in &self.aliases {
            reference(alias)?;
            require(names.insert(alias), "duplicate property key or alias")?;
        }
        if let Some(unit) = &self.unit {
            reference(unit)?;
        }
        match (self.spec.value_type, self.embedding_dimensions) {
            (PropertyType::Embedding, Some(n)) if n > 0 && n <= 32_768 => {}
            (PropertyType::Embedding, _) => {
                return Err("embedding requires source-qualified dimensions".into());
            }
            (_, None) => {}
            _ => return Err("embedding dimensions attached to another value type".into()),
        }
        for value in &self.domain {
            self.spec.validate_value(value, self.embedding_dimensions)?;
        }
        Ok(())
    }
    pub fn validate_value(&self, value: &Value) -> Result<()> {
        self.spec.validate_value(value, self.embedding_dimensions)?;
        require(
            self.domain.is_empty() || self.domain.contains(value),
            "property value outside declared domain",
        )
    }
}

impl SourcePropertySpec {
    pub fn validate_value(&self, value: &Value, dimensions: Option<usize>) -> Result<()> {
        bounded(value)?;
        if self.cardinality == Cardinality::Many
            && !matches!(
                self.value_type,
                PropertyType::StringList | PropertyType::Embedding
            )
        {
            let values = value
                .as_array()
                .ok_or("many-valued property requires an array")?;
            for item in values {
                self.scalar(item, dimensions)?;
            }
            Ok(())
        } else {
            self.scalar(value, dimensions)
        }
    }
    fn scalar(&self, value: &Value, dimensions: Option<usize>) -> Result<()> {
        let valid = match self.value_type {
            PropertyType::String => value.is_string(),
            PropertyType::StringList => value
                .as_array()
                .is_some_and(|v| v.iter().all(Value::is_string)),
            PropertyType::Integer => value.as_i64().is_some(),
            PropertyType::Float => value.as_f64().is_some_and(f64::is_finite),
            PropertyType::Boolean => value.is_boolean(),
            PropertyType::DateTime => value.as_str().is_some_and(rfc3339),
            PropertyType::JsonString => value.as_str().is_some_and(|s| {
                serde_json::from_str::<Value>(s).is_ok_and(|v| bounded(&v).is_ok())
            }),
            PropertyType::Embedding => value.as_array().is_some_and(|v| {
                dimensions.is_some_and(|n| n > 0 && n == v.len())
                    && v.iter().all(|x| x.as_f64().is_some_and(f64::is_finite))
            }),
        };
        require(
            valid,
            "value does not satisfy the source-defined property type",
        )
    }
}

/// Supported civil-time wire form: RFC3339 years 0001..9999, explicit zone,
/// ordinary seconds 00..59. Leap-second notation needs its native time owner;
/// this validator rejects it rather than normalising a different instant.
fn rfc3339(value: &str) -> bool {
    let b = value.as_bytes();
    if b.len() < 20 || !value.is_ascii() {
        return false;
    }
    let number = |a: usize, z: usize| -> Option<u32> {
        b.get(a..z)
            .filter(|v| v.iter().all(u8::is_ascii_digit))
            .and_then(|v| std::str::from_utf8(v).ok())?
            .parse()
            .ok()
    };
    if b[4] != b'-'
        || b[7] != b'-'
        || !matches!(b[10], b'T' | b't')
        || b[13] != b':'
        || b[16] != b':'
    {
        return false;
    }
    let (Some(y), Some(m), Some(d), Some(h), Some(min), Some(s)) = (
        number(0, 4),
        number(5, 7),
        number(8, 10),
        number(11, 13),
        number(14, 16),
        number(17, 19),
    ) else {
        return false;
    };
    if y == 0 || !(1..=12).contains(&m) || h > 23 || min > 59 || s > 59 {
        return false;
    }
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
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
    if d == 0 || d > days[(m - 1) as usize] {
        return false;
    }
    let mut i = 19;
    if b.get(i) == Some(&b'.') {
        i += 1;
        let start = i;
        while b.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        if i == start {
            return false;
        }
    }
    match b.get(i..) {
        Some([b'Z' | b'z']) => true,
        Some([b'+' | b'-', a, c, b':', d, e]) => {
            [a, c, d, e].iter().all(|v| v.is_ascii_digit())
                && (a - b'0') * 10 + (c - b'0') <= 23
                && (d - b'0') * 10 + (e - b'0') <= 59
        }
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValueRole {
    Rich,
    Quintessential,
    GraphDerived,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Standing {
    Proposed,
    SourceDeclared,
    Observed,
    Derived,
    Reviewed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphDerivation {
    pub graph: SourcePin,
    pub algorithm: SourcePin,
    pub parameters: Value,
    pub included_nodes: Vec<MTreeId>,
    pub excluded_nodes: Vec<MTreeId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Draft {
    pub identity: String,
    pub definition: VersionRef,
    pub subject: Subject,
    pub value: Value,
    pub role: ValueRole,
    pub standing: Standing,
    pub sources: Vec<SourcePin>,
    pub producer: SourcePin,
    pub result: SourcePin,
    pub warrant: String,
    pub valid_from_unix_ms: Option<i64>,
    pub valid_until_unix_ms: Option<i64>,
    pub derivation: Option<GraphDerivation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertion {
    pub reference: VersionRef,
    pub content: Draft,
    pub previous: Option<VersionRef>,
    pub restored_from: Option<VersionRef>,
    pub recognition: Option<SourcePin>,
    pub admission: SourcePin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Compact {
    pub version: String,
    pub assertion: VersionRef,
    pub definition: VersionRef,
    pub subject: Subject,
    pub value: Value,
    pub role: ValueRole,
    pub standing: Standing,
    pub support: SourcePin,
}
