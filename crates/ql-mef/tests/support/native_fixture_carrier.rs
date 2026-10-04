// Test transport only. Every full native value is reconstructed before the
// existing source/receiving/body/audio consumers. No hash admits native work.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const WIRE_LIMIT: usize = 8 * 1024 * 1024;
const EXPANDED_LIMIT: usize = 4 * WIRE_LIMIT;
const PART_LIMIT: usize = 128;
const NODE_LIMIT: usize = 1_000_000;
const REF_LIMIT: usize = 1024;
const DEPTH_LIMIT: usize = 64;
const SCHEMA: &str = "ql.test-native-fixture-carrier/v1";

#[derive(Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum Node {
    Object(Vec<(String, Node)>),
    Array(Vec<Node>),
    Literal(Value),
    Part(usize),
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Part {
    original_json: String,
    bytes: usize,
    fingerprint: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Carrier {
    schema: String,
    tree: Node,
    parts: Vec<Part>,
}
fn eligible(key: Option<&str>) -> bool {
    matches!(
        key,
        Some(
            "native_basis"
                | "native_preparation"
                | "performance_preparation"
                | "current_m3"
                | "preparation"
        )
    )
}
// Non-authenticating byte identity, explicitly separate from original native
// SHA256/current/consent receipts. Complete values and equality remain decisive.
fn fingerprint(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash = (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}
fn encode_node(
    value: &Value,
    key: Option<&str>,
    parts: &mut Vec<Part>,
    indices: &mut BTreeMap<Vec<u8>, usize>,
) -> Node {
    if eligible(key) && value.is_object() {
        let bytes = serde_json::to_vec(value).unwrap();
        if let Some(index) = indices.get(&bytes) {
            return Node::Part(*index);
        }
        let index = parts.len();
        assert!(index < PART_LIMIT && bytes.len() <= WIRE_LIMIT);
        parts.push(Part {
            bytes: bytes.len(),
            fingerprint: fingerprint(&bytes),
            original_json: String::from_utf8(bytes.clone()).unwrap(),
        });
        indices.insert(bytes, index);
        return Node::Part(index);
    }
    match value {
        Value::Object(object) => Node::Object(
            object
                .iter()
                .map(|(key, child)| (key.clone(), encode_node(child, Some(key), parts, indices)))
                .collect(),
        ),
        Value::Array(array) => Node::Array(
            array
                .iter()
                .map(|child| encode_node(child, None, parts, indices))
                .collect(),
        ),
        _ => Node::Literal(value.clone()),
    }
}
struct Decoder<'a> {
    parts: &'a [Part],
    used: Vec<bool>,
    nodes: usize,
    references: usize,
    expanded_parts: usize,
}
impl Decoder<'_> {
    fn node(&mut self, node: &Node, key: Option<&str>, depth: usize) -> Result<Value, String> {
        self.nodes += 1;
        if depth > DEPTH_LIMIT || self.nodes > NODE_LIMIT {
            return Err("native test carrier tree bound".into());
        }
        Ok(match node {
            Node::Literal(value) => value.clone(),
            Node::Array(array) => Value::Array(
                array
                    .iter()
                    .map(|child| self.node(child, None, depth + 1))
                    .collect::<Result<_, _>>()?,
            ),
            Node::Object(object) => {
                let mut out = serde_json::Map::new();
                for (key, child) in object {
                    if out.contains_key(key) {
                        return Err("duplicate native test carrier key".into());
                    }
                    out.insert(key.clone(), self.node(child, Some(key), depth + 1)?);
                }
                Value::Object(out)
            }
            Node::Part(index) => {
                if !eligible(key) {
                    return Err("native test part outside declared composite".into());
                }
                self.references += 1;
                if self.references > REF_LIMIT {
                    return Err("native test reference bound".into());
                }
                let part = self.parts.get(*index).ok_or("missing native test part")?;
                self.expanded_parts = self
                    .expanded_parts
                    .checked_add(part.bytes)
                    .ok_or("native test expanded size overflow")?;
                if self.expanded_parts > EXPANDED_LIMIT {
                    return Err("native test expanded part bound".into());
                }
                self.used[*index] = true;
                // Parts are literal JSON, never another carrier node. Reference
                // cycles cannot be represented; a reference-shaped native
                // object remains its exact original ordinary source object.
                let value: Value =
                    serde_json::from_str(&part.original_json).map_err(|e| e.to_string())?;
                if !value.is_object()
                    || serde_json::to_string(&value).map_err(|e| e.to_string())?
                        != part.original_json
                {
                    return Err("noncanonical native test part".into());
                }
                value
            }
        })
    }
}
pub fn decode(value: &Value) -> Result<Value, String> {
    if serde_json::to_vec(value).map_err(|e| e.to_string())?.len() >= WIRE_LIMIT {
        return Err("native test wire bound".into());
    }
    let carrier: Carrier = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    if carrier.schema != SCHEMA || carrier.parts.is_empty() || carrier.parts.len() > PART_LIMIT {
        return Err("native test carrier schema/parts bound".into());
    }
    let mut total = 0_usize;
    let mut originals = BTreeSet::new();
    for part in &carrier.parts {
        total = total
            .checked_add(part.bytes)
            .ok_or("native test dictionary overflow")?;
        if part.bytes != part.original_json.len()
            || total > WIRE_LIMIT
            || fingerprint(part.original_json.as_bytes()) != part.fingerprint
            || !originals.insert(&part.original_json)
        {
            return Err("native test part bytes/fingerprint/duplicate differs".into());
        }
    }
    let mut decoder = Decoder {
        parts: &carrier.parts,
        used: vec![false; carrier.parts.len()],
        nodes: 0,
        references: 0,
        expanded_parts: 0,
    };
    let out = decoder.node(&carrier.tree, None, 0)?;
    fn qualify(value: &Value, depth: usize, nodes: &mut usize) -> Result<(), String> {
        *nodes += 1;
        if depth > DEPTH_LIMIT || *nodes > NODE_LIMIT {
            return Err("native test literal/full node bound".into());
        }
        match value {
            Value::Object(values) => {
                for child in values.values() {
                    qualify(child, depth + 1, nodes)?;
                }
            }
            Value::Array(values) => {
                for child in values {
                    qualify(child, depth + 1, nodes)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    qualify(&out, 0, &mut 0)?;
    if decoder.used.iter().any(|used| !used)
        || serde_json::to_vec(&out).map_err(|e| e.to_string())?.len() > EXPANDED_LIMIT
    {
        return Err("native test unused/expanded part bound".into());
    }
    Ok(out)
}
pub fn encode(original: &Value) -> Value {
    let original_bytes = serde_json::to_vec(original).unwrap();
    assert!(original_bytes.len() <= EXPANDED_LIMIT);
    let mut parts = Vec::new();
    let tree = encode_node(original, None, &mut parts, &mut BTreeMap::new());
    let carrier = serde_json::to_value(Carrier {
        schema: SCHEMA.into(),
        tree,
        parts,
    })
    .unwrap();
    let wire = serde_json::to_vec(&carrier).unwrap();
    eprintln!(
        "actual_native_fixture_carrier original_bytes={} encoded_bytes={} parts={}",
        original_bytes.len(),
        wire.len(),
        carrier["parts"].as_array().unwrap().len()
    );
    assert!(wire.len() < WIRE_LIMIT);
    assert_eq!(decode(&carrier).unwrap(), *original);
    carrier
}

pub fn detecting_trials(original: &Value, encoded: &Value) {
    let mut missing = encoded.clone();
    missing["parts"].as_array_mut().unwrap().pop();
    assert!(decode(&missing).is_err());
    let mut changed = encoded.clone();
    changed["parts"][0]["original_json"] = Value::String("{}".into());
    assert!(decode(&changed).is_err());
    let mut digest = encoded.clone();
    digest["parts"][0]["fingerprint"] = Value::String("fnv1a64:0000000000000000".into());
    assert!(decode(&digest).is_err());
    let mut surplus = encoded.clone();
    surplus["unexpected"] = Value::Bool(true);
    assert!(decode(&surplus).is_err());
    let mut unused = encoded.clone();
    unused["parts"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({"original_json":"{}","bytes":2,"fingerprint":fingerprint(b"{}")}));
    assert!(decode(&unused).is_err());
    let mut unknown = encoded.clone();
    unknown["tree"] = serde_json::json!({"kind":"part","value":usize::MAX});
    assert!(decode(&unknown).is_err());
    let mut duplicate = encoded.clone();
    duplicate["parts"]
        .as_array_mut()
        .unwrap()
        .push(encoded["parts"][0].clone());
    assert!(decode(&duplicate).is_err());
    // A native value resembling a recursive carrier reference stays opaque;
    // the transport can express no references inside literal dictionary parts.
    let mut reference_shaped = original.clone();
    reference_shaped["performance_preparation"]["original_unknown_field"] =
        serde_json::json!({"kind":"part","value":0});
    assert_eq!(
        decode(&encode(&reference_shaped)).unwrap(),
        reference_shaped
    );
    // Actual independently rebuilt repeated admissions must not become aliases.
    let mut restored = decode(encoded).unwrap();
    let current = restored["current_native_admission"]["native_basis"].clone();
    restored["native_admission"]["native_basis"]["independent_mutation"] = Value::Bool(true);
    assert_eq!(
        restored["current_native_admission"]["native_basis"],
        current
    );
    assert_eq!(decode(encoded).unwrap(), *original);
}
