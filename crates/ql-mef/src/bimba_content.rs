//! Complete, immutable material from the same admitted Bimba READ as M.
//! Source values and qualified relations stay source values: this module grants
//! no numerical interpretation, personal state, executable action or authorship.
use crate::MFace;
use crate::m_tree::native_current_m_registry;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::OnceLock;

pub const SOURCE_JSON: &str = include_str!("../../../fixtures/kernel/bimba-content-v1.json");

fn digest(value: &Value) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

#[derive(Debug)]
pub struct BimbaContent {
    value: Value,
    coordinates: Vec<String>,
    source_by_native_ref: BTreeMap<String, Vec<String>>,
}

impl BimbaContent {
    pub fn from_value(value: Value) -> Result<Self, String> {
        if value["schema"] != "ql.bimba-content/v1" {
            return Err("unsupported complete Bimba source schema".into());
        }
        let revision = value["source_revision"]
            .as_str()
            .ok_or("missing Bimba source revision")?;
        if digest(&value["content"])? != revision {
            return Err("Bimba content hash differs from its admitted source READ".into());
        }
        if native_current_m_registry().manifest().source_revision != revision {
            return Err(
                "complete Bimba content and current M registry have different source revisions"
                    .into(),
            );
        }
        let nodes = value["content"]["nodes"]
            .as_object()
            .ok_or("Bimba node material absent")?;
        let relations = value["content"]["relations"]
            .as_array()
            .ok_or("Bimba qualified relations absent")?;
        for (coordinate, node) in nodes {
            if node["properties"]["coordinate"] != *coordinate || !node["properties"].is_object() {
                return Err("Bimba coordinate/property identity mismatch".into());
            }
        }
        for edge in relations {
            let row = edge
                .as_array()
                .filter(|r| r.len() == 4)
                .ok_or("incomplete qualified Bimba edge")?;
            if !row[3].is_object()
                || !row[1].is_string()
                || !row[0].as_str().is_some_and(|a| nodes.contains_key(a))
                || !row[2].as_str().is_some_and(|b| nodes.contains_key(b))
            {
                return Err("Bimba edge endpoints or qualification absent".into());
            }
        }
        let coordinates = nodes.keys().cloned().collect();
        let registry = native_current_m_registry();
        let mut source_by_native_ref: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for coordinate in nodes.keys() {
            // Graph meta #0…#5 are distinct from the native M-root aliases.
            // Only actual M source spellings may acquire an M coordinate.
            if !coordinate.starts_with('M') {
                continue;
            }
            if let Some(node) = registry
                .resolve(coordinate)
                .filter(|node| !node.records.is_empty())
            {
                source_by_native_ref
                    .entry(node.source_ref.clone())
                    .or_default()
                    .push(coordinate.clone());
            }
        }
        Ok(Self {
            value,
            coordinates,
            source_by_native_ref,
        })
    }

    fn revision(&self) -> &str {
        self.value["source_revision"]
            .as_str()
            .expect("validated source revision")
    }
    fn registry_revision(&self) -> &str {
        &native_current_m_registry().manifest().registry_revision
    }
    fn native_refs(&self, coordinate: &str) -> (Option<String>, String) {
        if !coordinate.starts_with('M') {
            return (None, format!("bimba:{coordinate}"));
        }
        let registry = native_current_m_registry();
        if let Some(node) = registry
            .resolve(coordinate)
            .filter(|node| !node.records.is_empty())
        {
            // Identity needs the native coordinate grammar only. Constructing
            // a full MCoordinate here clones every retained property-key and
            // provenance payload for every inventory row and relation endpoint.
            // Source payloads are independently hash-checked below.
            if let Ok(reading) = crate::MCoordinate::parse_source(&node.source_ref, MFace::Bimba) {
                return (Some(node.source_ref.clone()), reading.canonical_ref());
            }
        }
        (None, format!("bimba:{coordinate}"))
    }
    fn identity(&self, coordinate: &str, full: bool) -> Result<Value, String> {
        let node = &self.value["content"]["nodes"][coordinate];
        let properties = &node["properties"];
        let (native, canonical) = self.native_refs(coordinate);
        let hash = digest(properties)?;
        let mut identity = json!({"coordinate":coordinate,"native_coordinate":native,
            "canonical_ref":canonical,"uuid":properties["c_2_uuid"],
            "title":properties["c_1_name"].as_str().unwrap_or(coordinate),
            "aliases":native.into_iter().collect::<Vec<_>>(),
            "source_revision":self.revision(),"registry_revision":self.registry_revision(),
            "full_source_ref":format!("bimba-source:{coordinate}"),
            "full_properties_ref":format!("bimba-source:{coordinate}#properties"),
            "properties_sha256":hash});
        if full {
            identity["properties"] = properties.clone();
            identity["labels"] = node["labels"].clone();
        }
        Ok(identity)
    }
    pub fn inventory(&self, offset: usize, limit: usize) -> Result<Value, String> {
        if !(1..=256).contains(&limit) || offset > self.coordinates.len() {
            return Err(
                "Bimba inventory requires a bounded existing offset and limit 1..256".into(),
            );
        }
        let end = (offset + limit).min(self.coordinates.len());
        let items = self.coordinates[offset..end]
            .iter()
            .map(|c| self.identity(c, false))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(
            json!({"schema":"ql.bimba-inventory/v1","source_revision":self.revision(),
            "registry_revision":self.registry_revision(),"source_ref":self.value["source_ref"],
            "total":self.coordinates.len(),"relations":self.value["content"]["relations"].as_array().expect("validated edges").len(),
            "offset":offset,"next_offset":if end < self.coordinates.len() {Some(end)} else {None},"items":items}),
        )
    }
    pub fn coordinate(&self, reference: &str) -> Result<Value, String> {
        if reference.is_empty() || reference.len() > 4096 {
            return Err("a bounded Bimba source reference is required".into());
        }
        let selector = reference
            .strip_prefix("bimba-source:")
            .or_else(|| reference.strip_prefix("ql:m-coordinate:bimba:"))
            .or_else(|| reference.strip_prefix("ql:m-coordinate:pratibimba:"))
            .or_else(|| reference.strip_prefix("bimba:"))
            .unwrap_or(reference);
        let explicit_source =
            reference.starts_with("bimba-source:") || reference.starts_with("bimba:");
        let coordinate = if (explicit_source || !selector.starts_with('#'))
            && self.value["content"]["nodes"].get(selector).is_some()
        {
            selector.to_owned()
        } else {
            let registry = native_current_m_registry();
            let node = registry
                .resolve(selector)
                .ok_or("Bimba source coordinate is absent")?;
            let sources = self
                .source_by_native_ref
                .get(&node.source_ref)
                .ok_or("native coordinate has no retained source payload")?;
            if sources.len() != 1 {
                return Err("native coordinate has ambiguous source payload aliases".into());
            }
            sources[0].clone()
        };
        if self.value["content"]["nodes"].get(&coordinate).is_none() {
            return Err("exact Bimba source spelling is absent".into());
        }
        let identity = self.identity(&coordinate, true)?;
        if let Some(native) = identity["native_coordinate"].as_str() {
            let registry = native_current_m_registry();
            let node = registry
                .resolve(native)
                .ok_or("native source identity is absent")?;
            if !node.records.iter().any(|&r| {
                registry.manifest().records[r].payload_sha256 == identity["properties_sha256"]
            }) {
                return Err(
                    "Bimba property material differs from the native coordinate source record"
                        .into(),
                );
            }
        }
        let relations = self.value["content"]["relations"].as_array().expect("validated edges").iter().enumerate().filter(|(_, r)| r[0] == coordinate || r[2] == coordinate).map(|(index,r)| {
            let a = r[0].as_str().expect("validated source"); let b = r[2].as_str().expect("validated target");
            let (_, a_ref) = self.native_refs(a); let (_, b_ref) = self.native_refs(b);
            let tuple_hash = digest(&json!([r[0],r[1],r[2]]))?;
            Ok(json!({"source_index":index,"relation_ref":format!("bimba:relation:{}",&tuple_hash[..24]),
                "from_coordinate":a,"to_coordinate":b,"from_ref":a_ref,"to_ref":b_ref,
                "kind":r[1],"orientation":"directed","properties":r[3],"properties_sha256":digest(&r[3])?,"source_revision":self.revision()}))
        }).collect::<Result<Vec<Value>,String>>()?;
        Ok(
            json!({"schema":"ql.bimba-coordinate-content/v1","source_revision":self.revision(),
            "registry_revision":self.registry_revision(),"identity":identity,"relations":relations,
            "standing":"complete shared source properties and qualified directed relations; no private personal state"}),
        )
    }
}

pub fn native_bimba_content() -> Result<&'static BimbaContent, String> {
    static SOURCE: OnceLock<Result<BimbaContent, String>> = OnceLock::new();
    SOURCE
        .get_or_init(|| {
            serde_json::from_str(SOURCE_JSON)
                .map_err(|e| e.to_string())
                .and_then(BimbaContent::from_value)
        })
        .as_ref()
        .map_err(Clone::clone)
}
