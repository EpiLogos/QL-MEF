//! Native successor of CE's bounded M2RelationPlan.
//!
//! Source recovery/validation and provider resolution occur before rendering.
//! The plan copies M2-1' Vimarśā's eight audible and four nodal values. M1 owns
//! excitation; material writes preserve the supplied body's eigenfrequencies.
//! Source intervals, retained approximations and absent authentic tuning remain
//! distinct. This module neither fetches a graph nor mutates authored source.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::m_tree::{MRegistry, MTreeId, native_m_registry};
use crate::m2::{self, Reading72, Register72};
use crate::m2_condition::{CorrespondenceRole, M2ConditionInput, correspondence_field};
use crate::m2_engine::{EventIdentity, InputStamp, M2Frame, M2Request, MAX_EXACT_JSON_INTEGER};
use crate::m2_vimarsha::NodalConstraint;

pub const M2_RELATION_PLAN_SCHEMA: &str = "epi.m2.relation-plan.v1";
pub const M2_RELATION_PLAN_OWNER: &str = "M2'/M2-5 bounded relation compiler";
pub const M2_NATIVE_EXTENSION: &str = "ql.m2-relation-plan/native-v1";
pub const M2_DOMAIN_SPEC_REF: &str = "Idea/Bimba/Seeds/M/M2'/M2'-SPEC.md";
pub const M2_SITUATED_LOCK_REF: &str =
    "Idea/Bimba/Seeds/M/M2'/M2-5-SITUATED-RETURN-PLAYBACK-LOCK.md";
pub const SOURCE_SCHEMA: &str = "ql.m2-relation-source/v1";
pub const MAX_PLAN_NODES: usize = 32;
pub const MAX_PLAN_RELATIONS: usize = 256;
pub const MAX_PLAN_CLAIMS: usize = 4096;
pub const MAX_ROUTES: usize = 64;

fn require_ref(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 {
        Err(format!("{label}: missing or unbounded reference"))
    } else {
        Ok(())
    }
}
fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn hash_value(value: &Option<Value>) -> String {
    let tagged = match value {
        Some(value) => serde_json::json!({"present": true, "value": value}),
        None => serde_json::json!({"present": false}),
    };
    hash_bytes(
        serde_json::to_string(&tagged)
            .expect("JSON source value")
            .as_bytes(),
    )
}
fn is_m2(reference: &str) -> bool {
    reference == "#2"
        || reference.starts_with("#2-")
        || reference.starts_with("#2.")
        || reference.starts_with("#2/")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M2SourceNode {
    pub coordinate: String,
    pub map_coordinate: String,
    pub id: MTreeId,
    pub record_index: usize,
    pub payload_sha256: String,
    /// Canonical Python JSON bytes are preserved: exponent spelling must not be
    /// changed before comparison with the native source record's payload hash.
    pub canonical_properties: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct M2SourceRelation {
    pub id: MTreeId,
    pub relation_ref: String,
    pub kind: String,
    pub from_coordinate: String,
    pub to_coordinate: String,
    pub record_index: usize,
    pub payload_sha256: String,
    pub canonical_properties: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M2PrimeSourceNode {
    pub map_coordinate: String,
    pub canonical_properties: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M2SourceField {
    pub schema: String,
    pub registry_revision: String,
    pub source_revision: String,
    pub source_repository: String,
    pub source_path: String,
    pub source_sha256: String,
    pub standing: String,
    nodes: Vec<M2SourceNode>,
    relations: Vec<M2SourceRelation>,
    pub prime_source_nodes: Vec<M2PrimeSourceNode>,
    #[serde(skip)]
    values: BTreeMap<String, Value>,
    #[serde(skip)]
    by_coordinate: BTreeMap<String, usize>,
    #[serde(skip)]
    by_relation: BTreeMap<MTreeId, usize>,
}
impl M2SourceField {
    pub fn from_json(text: &str) -> Result<Self, String> {
        Self::from_json_with_registry(text, native_m_registry())
    }
    /// Refresh uses the qualified successor registry, so plan dependency checks
    /// can distinguish an unrelated source edit from a changed consumed edge.
    pub fn from_json_with_registry(text: &str, registry: &MRegistry) -> Result<Self, String> {
        if text.len() > 32 * 1024 * 1024 {
            return Err("M2 source projection exceeds 32 MiB".into());
        }
        let mut field: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let manifest = registry.manifest();
        if field.schema != SOURCE_SCHEMA
            || field.registry_revision != manifest.registry_revision
            || field.source_revision != manifest.source_revision
            || field.source_repository != manifest.source_repository
            || !manifest
                .files
                .iter()
                .any(|f| f.path == field.source_path && f.sha256 == field.source_sha256)
            || field.nodes.len() > 10000
            || field.relations.len() > 100000
            || field.prime_source_nodes.len() > 10000
        {
            return Err("unqualified or unbounded M2 source projection".into());
        }
        require_ref("source standing", &field.standing)?;
        for (index, node) in field.nodes.iter().enumerate() {
            let native = registry
                .resolve(&node.coordinate)
                .ok_or("unregistered source node")?;
            if !is_m2(&node.coordinate)
                || native.id != node.id
                || field
                    .by_coordinate
                    .insert(node.coordinate.clone(), index)
                    .is_some()
                || hash_bytes(node.canonical_properties.as_bytes()) != node.payload_sha256
                || !native.records.iter().any(|r| {
                    let record = &manifest.records[*r];
                    let file = &manifest.files[record.file];
                    record.record_index == node.record_index
                        && record.payload_sha256 == node.payload_sha256
                        && file.path == field.source_path
                        && file.sha256 == field.source_sha256
                })
            {
                return Err(format!(
                    "M2 source node/record mismatch: {}",
                    node.coordinate
                ));
            }
            let value: Value =
                serde_json::from_str(&node.canonical_properties).map_err(|e| e.to_string())?;
            let object = value
                .as_object()
                .ok_or("source properties must be an object")?;
            if object.get("coordinate").and_then(Value::as_str)
                != Some(node.map_coordinate.as_str())
            {
                return Err("map spelling was lost or substituted".into());
            }
            field.values.insert(node.coordinate.clone(), value);
        }
        let expected_nodes: BTreeSet<_> = manifest
            .nodes
            .iter()
            .filter(|n| is_m2(&n.source_ref))
            .map(|n| n.id)
            .collect();
        if expected_nodes != field.nodes.iter().map(|n| n.id).collect() {
            return Err("M2 projection lost a native descendant".into());
        }
        for (index, edge) in field.relations.iter().enumerate() {
            let native = manifest
                .relations
                .iter()
                .find(|r| r.id == edge.id)
                .ok_or("unregistered source edge")?;
            let record = &manifest.records[native.record];
            let file = &manifest.files[record.file];
            if field.by_relation.insert(edge.id, index).is_some()
                || native.relation_ref != edge.relation_ref
                || native.source_kind != edge.kind
                || native.from_ref.as_deref() != Some(edge.from_coordinate.as_str())
                || native.to_ref.as_deref() != Some(edge.to_coordinate.as_str())
                || record.record_index != edge.record_index
                || record.payload_sha256 != edge.payload_sha256
                || file.path != field.source_path
                || file.sha256 != field.source_sha256
                || hash_bytes(edge.canonical_properties.as_bytes()) != edge.payload_sha256
                || !serde_json::from_str::<Value>(&edge.canonical_properties)
                    .map_err(|e| e.to_string())?
                    .is_object()
            {
                return Err(format!(
                    "M2 typed source edge mismatch: {}",
                    edge.relation_ref
                ));
            }
        }
        let expected_edges: BTreeSet<_> = manifest
            .relations
            .iter()
            .filter(|r| {
                r.from_ref.as_deref().is_some_and(is_m2) || r.to_ref.as_deref().is_some_and(is_m2)
            })
            .map(|r| r.id)
            .collect();
        if expected_edges != field.relations.iter().map(|r| r.id).collect() {
            return Err("M2 projection lost a typed edge".into());
        }
        let mut primes = BTreeSet::new();
        for prime in &field.prime_source_nodes {
            if !(prime.map_coordinate == "M2'" || prime.map_coordinate.starts_with("M2'-"))
                || !primes.insert(&prime.map_coordinate)
                || !serde_json::from_str::<Value>(&prime.canonical_properties)
                    .map_err(|e| e.to_string())?
                    .is_object()
            {
                return Err("invalid prime source node (no native ID may be invented)".into());
            }
        }
        Ok(field)
    }
    pub fn node(&self, coordinate: &str) -> Option<&M2SourceNode> {
        self.by_coordinate.get(coordinate).map(|i| &self.nodes[*i])
    }
    pub fn properties(&self, coordinate: &str) -> Option<&Value> {
        self.values.get(coordinate)
    }
    pub fn property(&self, coordinate: &str, key: &str) -> Option<&Value> {
        self.properties(coordinate)?.get(key)
    }
    pub fn relations(&self, from: &str, kind: &str) -> Vec<&M2SourceRelation> {
        self.relations
            .iter()
            .filter(|r| r.from_coordinate == from && r.kind == kind)
            .collect()
    }
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }
}
pub fn source_field() -> &'static M2SourceField {
    static FIELD: OnceLock<M2SourceField> = OnceLock::new();
    FIELD.get_or_init(|| {
        M2SourceField::from_json(include_str!(
            "../../../fixtures/kernel/m2-relation-source-v1.json"
        ))
        .expect("qualified M2 source projection")
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2Address72Views {
    pub index72: u8,
    pub mef_lens12: u8,
    pub mef_position6: u8,
    pub tattva36: u8,
    pub tattva_phase2: u8,
    pub decan_element4: u8,
    pub decan_sign3: u8,
    pub decan_index3: u8,
    pub decan_face2: u8,
    pub shem_choir8: u8,
    pub shem_position9: u8,
    pub maqam_row72: u8,
}
impl M2Address72Views {
    pub fn decode(index: usize) -> Result<Self, String> {
        if index >= 72 {
            return Err("M2 address outside 72; 84 lens/CF is a distinct space".into());
        }
        let i = index as u8;
        Ok(Self {
            index72: i,
            mef_lens12: i / 6,
            mef_position6: i % 6,
            tattva36: i / 2,
            tattva_phase2: i % 2,
            decan_element4: i / 18,
            decan_sign3: i % 18 / 6,
            decan_index3: i % 6 / 2,
            decan_face2: i % 2,
            shem_choir8: i / 9,
            shem_position9: i % 9,
            maqam_row72: i,
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum M2PrimeFamily {
    #[serde(rename = "M2-0'")]
    Ground,
    #[serde(rename = "M2-1'")]
    Vimarsha,
    #[serde(rename = "M2-2'")]
    Density,
    #[serde(rename = "M2-3'")]
    Decan,
    #[serde(rename = "M2-4'")]
    Arena,
    #[serde(rename = "M2-5'")]
    Situated,
}
impl M2PrimeFamily {
    fn from_coordinate(coordinate: &str) -> Result<Self, String> {
        for (prefix, family) in [
            ("#2-0", Self::Ground),
            ("#2-1", Self::Vimarsha),
            ("#2-2", Self::Density),
            ("#2-3", Self::Decan),
            ("#2-4", Self::Arena),
            ("#2-5", Self::Situated),
        ] {
            if coordinate == prefix
                || coordinate
                    .strip_prefix(prefix)
                    .is_some_and(|s| s.starts_with('-') || s.starts_with('.') || s.starts_with('/'))
            {
                return Ok(family);
            }
        }
        Err("source leaf is not a native M2-0..5 family".into())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M2MefPhase {
    Direct,
    Prime,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2PrimeOperation {
    pub family: M2PrimeFamily,
    pub source_coordinate: String,
    pub native_id: MTreeId,
    /// MEF L/L' phase belongs to the actual stamped frame. It is independent of
    /// the descendant's tattva/decan phase and the authored operation family.
    pub mef_phase: M2MefPhase,
    pub authored_source_ref: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2PropertyReceipt {
    pub coordinate: String,
    pub native_id: MTreeId,
    pub property: String,
    /// None is an actual missing property dependency, never a default literal.
    pub literal: Option<Value>,
    pub value_sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2RelationSetReceipt {
    pub from_coordinate: String,
    pub kind: String,
    pub relations: Vec<M2SourceRelation>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2SourceReceipts {
    pub registry_revision: String,
    pub source_revision: String,
    pub source_path: String,
    pub source_sha256: String,
    pub operations: Vec<M2PrimeOperation>,
    pub properties: Vec<M2PropertyReceipt>,
    pub relation_sets: Vec<M2RelationSetReceipt>,
}
impl M2SourceReceipts {
    fn new(field: &M2SourceField) -> Self {
        Self {
            registry_revision: field.registry_revision.clone(),
            source_revision: field.source_revision.clone(),
            source_path: field.source_path.clone(),
            source_sha256: field.source_sha256.clone(),
            ..Self::default()
        }
    }
    fn retain_node(
        &mut self,
        field: &M2SourceField,
        coordinate: &str,
        phase: M2MefPhase,
    ) -> Result<(), String> {
        let node = field
            .node(coordinate)
            .ok_or_else(|| format!("unavailable source descendant: {coordinate}"))?;
        if self.operations.iter().any(|r| r.native_id == node.id) {
            return Ok(());
        }
        if self.operations.len() >= MAX_PLAN_NODES {
            return Err("source node budget exceeded".into());
        }
        self.operations.push(M2PrimeOperation {
            family: M2PrimeFamily::from_coordinate(coordinate)?,
            source_coordinate: coordinate.into(),
            native_id: node.id,
            mef_phase: phase,
            authored_source_ref: M2_DOMAIN_SPEC_REF.into(),
        });
        for key in field
            .properties(coordinate)
            .and_then(Value::as_object)
            .ok_or("missing native properties")?
            .keys()
        {
            self.retain_property(field, coordinate, key)?;
        }
        Ok(())
    }
    fn retain_property(
        &mut self,
        field: &M2SourceField,
        coordinate: &str,
        key: &str,
    ) -> Result<(), String> {
        if self
            .properties
            .iter()
            .any(|r| r.coordinate == coordinate && r.property == key)
        {
            return Ok(());
        }
        if self.properties.len() >= MAX_PLAN_CLAIMS {
            return Err("source property budget exceeded".into());
        }
        let node = field.node(coordinate).ok_or("missing property node")?;
        let literal = field.property(coordinate, key).cloned();
        self.properties.push(M2PropertyReceipt {
            coordinate: coordinate.into(),
            native_id: node.id,
            property: key.into(),
            value_sha256: hash_value(&literal),
            literal,
        });
        Ok(())
    }
    fn retain_edges(
        &mut self,
        field: &M2SourceField,
        from: &str,
        kind: &str,
    ) -> Result<Vec<M2SourceRelation>, String> {
        if let Some(existing) = self
            .relation_sets
            .iter()
            .find(|r| r.from_coordinate == from && r.kind == kind)
        {
            return Ok(existing.relations.clone());
        }
        let relations: Vec<_> = field.relations(from, kind).into_iter().cloned().collect();
        let n = self
            .relation_sets
            .iter()
            .map(|s| s.relations.len())
            .sum::<usize>()
            + relations.len();
        if n > MAX_PLAN_RELATIONS {
            return Err("source edge budget exceeded".into());
        }
        self.relation_sets.push(M2RelationSetReceipt {
            from_coordinate: from.into(),
            kind: kind.into(),
            relations: relations.clone(),
        });
        Ok(relations)
    }
    /// Called at source-refresh/control admission, never in an audio callback.
    /// The global source hash may change; only consumed literals and exact typed
    /// relation sets govern the selected plan's continued validity.
    pub fn validate_against(&self, field: &M2SourceField) -> Result<(), String> {
        if self.operations.len() > MAX_PLAN_NODES
            || self.properties.len() > MAX_PLAN_CLAIMS
            || self
                .relation_sets
                .iter()
                .map(|s| s.relations.len())
                .sum::<usize>()
                > MAX_PLAN_RELATIONS
        {
            return Err("unbounded plan receipts".into());
        }
        for op in &self.operations {
            if field.node(&op.source_coordinate).map(|n| n.id) != Some(op.native_id)
                || M2PrimeFamily::from_coordinate(&op.source_coordinate)? != op.family
                || op.authored_source_ref != M2_DOMAIN_SPEC_REF
            {
                return Err("source descendant/prime operation identity changed".into());
            }
        }
        for claim in &self.properties {
            if field.node(&claim.coordinate).map(|n| n.id) != Some(claim.native_id)
                || hash_value(&field.property(&claim.coordinate, &claim.property).cloned())
                    != claim.value_sha256
                || hash_value(&claim.literal) != claim.value_sha256
            {
                return Err(format!(
                    "consumed source property changed: {}/{}",
                    claim.coordinate, claim.property
                ));
            }
        }
        for set in &self.relation_sets {
            // record positions are transport provenance; endpoint/kind/identity
            // and payload govern semantics when a new READ reorders records.
            let signature = |r: &M2SourceRelation| {
                (
                    r.id,
                    r.relation_ref.clone(),
                    r.kind.clone(),
                    r.from_coordinate.clone(),
                    r.to_coordinate.clone(),
                    r.payload_sha256.clone(),
                )
            };
            let old: BTreeSet<_> = set.relations.iter().map(signature).collect();
            let now: BTreeSet<_> = field
                .relations(&set.from_coordinate, &set.kind)
                .into_iter()
                .map(signature)
                .collect();
            if old != now {
                return Err(format!(
                    "consumed typed edge set changed: {}/{}",
                    set.from_coordinate, set.kind
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2SituatedProviderBinding {
    pub planetary_state_ref: String,
    pub chakra_state_ref: String,
    pub elemental_state_ref: String,
    #[serde(default)]
    pub identity_relation_refs: Vec<String>,
    #[serde(default)]
    pub temporal_provider_refs: Vec<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2MusicalRouteInput {
    pub maqam_ref: Option<String>,
    pub tuning_ref: Option<String>,
    pub phrase_or_modulation_ref: Option<String>,
    #[serde(default)]
    pub mantra_or_name_route_refs: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2BoundedRoute {
    pub route_ref: String,
    pub provenance_ref: String,
}

/// Actual location and local solar boundaries supplied by the native observer
/// provider. Polar no-rise/no-set is unavailable, never approximated by 06/18h.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2ObserverSolarWindow {
    pub stamp: InputStamp,
    pub observer_ref: String,
    pub observer_coordinate: String,
    pub latitude_degrees: f64,
    pub longitude_degrees: f64,
    pub sunrise_unix_ms: u64,
    pub sunset_unix_ms: u64,
    pub next_sunrise_unix_ms: u64,
    pub civil_utc_offset_minutes: i16,
    pub boundary_provider_ref: String,
    pub boundary_source_revision: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2PlanetaryHour {
    pub observer: M2ObserverSolarWindow,
    /// Sunday = 0, derived from local civil date at sunrise.
    pub sunrise_weekday: u8,
    pub hour24: u8,
    pub daylight: bool,
    pub begins_unix_ms: u64,
    pub ends_unix_ms: u64,
    pub planet_coordinate: String,
    pub planet_id: MTreeId,
    pub chaldean_order_literal: String,
    pub standing: String,
}
fn planetary_hour(
    field: &M2SourceField,
    window: &M2ObserverSolarWindow,
    request: &M2Request,
) -> Result<M2PlanetaryHour, String> {
    if window.stamp.identity != request.stamp.identity
        || window.observer_coordinate != "#2-5-0/1-0"
        || !window.latitude_degrees.is_finite()
        || !(-90.0..=90.0).contains(&window.latitude_degrees)
        || !window.longitude_degrees.is_finite()
        || !(-180.0..=180.0).contains(&window.longitude_degrees)
        || !(-840..=840).contains(&window.civil_utc_offset_minutes)
        || window.next_sunrise_unix_ms > MAX_EXACT_JSON_INTEGER
        || !(window.sunrise_unix_ms < window.sunset_unix_ms
            && window.sunset_unix_ms < window.next_sunrise_unix_ms)
        || window.next_sunrise_unix_ms - window.sunrise_unix_ms > 172_800_000
        || request.at_unix_ms < window.sunrise_unix_ms
        || request.at_unix_ms >= window.next_sunrise_unix_ms
    {
        return Err("missing, stale, cross-event or invalid observer solar window".into());
    }
    for (label, value) in [
        ("observer", &window.observer_ref),
        ("solar boundary provider", &window.boundary_provider_ref),
        ("solar boundary revision", &window.boundary_source_revision),
        ("solar source", &window.stamp.source_ref),
        ("solar contract", &window.stamp.contract_ref),
    ] {
        require_ref(label, value)?;
    }
    field
        .node(&window.observer_coordinate)
        .ok_or("Earth observer missing from native source")?;
    let literal = field
        .property("#2-5", "c_2_chaldean_order_verified")
        .and_then(Value::as_str)
        .ok_or("source Chaldean order unavailable")?;
    let order: Vec<_> = literal.split('→').map(str::trim).collect();
    if order.len() != 7 || order.iter().collect::<BTreeSet<_>>().len() != 7 || order[0] != "Sun" {
        return Err("unsupported or conflicting source Chaldean cycle".into());
    }
    let mut planets = Vec::new();
    for name in &order {
        let matches: Vec<_> = (1..8)
            .filter_map(|i| {
                let coordinate = m2::catalogue().table("planet").ok()?.binding(i)?.to_owned();
                (field
                    .property(&coordinate, "c_1_name")
                    .and_then(Value::as_str)
                    == Some(*name))
                .then_some(coordinate)
            })
            .collect();
        if matches.len() != 1 {
            return Err("Chaldean planet source identity unavailable".into());
        }
        planets.push(matches[0].clone());
    }
    let local_ms =
        i128::from(window.sunrise_unix_ms) + i128::from(window.civil_utc_offset_minutes) * 60_000;
    let days = local_ms.div_euclid(86_400_000);
    let weekday = (days + 4).rem_euclid(7) as u8; // 1970-01-01 was Thursday.
    let daylight = request.at_unix_ms < window.sunset_unix_ms;
    let (start, end, offset) = if daylight {
        (window.sunrise_unix_ms, window.sunset_unix_ms, 0)
    } else {
        (window.sunset_unix_ms, window.next_sunrise_unix_ms, 12)
    };
    let duration = end - start;
    let local_hour = ((u128::from(request.at_unix_ms - start) * 12) / u128::from(duration)) as u8;
    let hour24 = local_hour + offset;
    let ruler_index = (usize::from(weekday) * 24 + usize::from(hour24)) % 7;
    let coordinate = planets[ruler_index].clone();
    // Exact integer half-open segmentation matches the inverse division, even
    // when a solar interval is not divisible by twelve milliseconds.
    let ceil12 = |v: u128| v.div_ceil(12) as u64;
    Ok(M2PlanetaryHour {
        observer: window.clone(),
        sunrise_weekday: weekday,
        hour24,
        daylight,
        begins_unix_ms: start + ceil12(u128::from(duration) * u128::from(local_hour)),
        ends_unix_ms: start + ceil12(u128::from(duration) * u128::from(local_hour + 1)),
        planet_id: field
            .node(&coordinate)
            .ok_or("hour planet source missing")?
            .id,
        planet_coordinate: coordinate,
        chaldean_order_literal: literal.into(),
        standing: "provider-supplied-solar-boundaries; source-Chaldean-cycle; unequal-hours".into(),
    })
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2MaqamCandidate {
    pub maqam_index: u8,
    pub role: CorrespondenceRole,
    pub maqam_coordinate: String,
    pub planet_coordinate: String,
    pub chakra_coordinate: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
pub enum M2MaterialTarget {
    Parameter { name: String, unit: String },
    ModeDamping { mode_ref: String },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2MaterialWrite {
    pub source_coordinate: String,
    pub source_property: String,
    /// A source meaning does not itself specify a physical constant. This
    /// named adjustable interpretation remains distinct from the source.
    pub interpretation_policy_ref: String,
    pub target: M2MaterialTarget,
    pub expected_value: f64,
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
}
fn apply_material(request: &mut M2Request, writes: &[M2MaterialWrite]) -> Result<(), String> {
    if writes.len() > MAX_ROUTES {
        return Err("material write budget exceeded".into());
    }
    if writes.is_empty() {
        return Ok(());
    }
    let identity = request.stamp.identity.clone();
    let state = request
        .resonator
        .as_mut()
        .ok_or("material target lacks supplied physical resonator")?;
    if state.stamp.identity != identity {
        return Err("material target belongs to a different event/generation".into());
    }
    let mut seen = BTreeSet::new();
    for write in writes {
        require_ref(
            "material interpretation policy",
            &write.interpretation_policy_ref,
        )?;
        if [
            write.expected_value,
            write.value,
            write.minimum,
            write.maximum,
        ]
        .iter()
        .any(|v| !v.is_finite())
            || write.minimum > write.maximum
            || !(write.minimum..=write.maximum).contains(&write.value)
        {
            return Err("non-finite or out-of-range tunable material write".into());
        }
        let key = serde_json::to_string(&write.target).map_err(|e| e.to_string())?;
        if !seen.insert(key) {
            return Err("conflicting material writes".into());
        }
        match &write.target {
            M2MaterialTarget::Parameter { name, unit } => {
                require_ref("material parameter name", name)?;
                require_ref("material physical unit", unit)?;
                let parameter = state
                    .material_parameters
                    .get_mut(name)
                    .ok_or("unknown supplied material parameter")?;
                if parameter.unit != *unit
                    || parameter.value.to_bits() != write.expected_value.to_bits()
                {
                    return Err("material parameter unit/precondition mismatch".into());
                }
                parameter.value = write.value;
                parameter.source_ref = write.interpretation_policy_ref.clone();
            }
            M2MaterialTarget::ModeDamping { mode_ref } => {
                require_ref("physical mode", mode_ref)?;
                if write.minimum < 0.0 {
                    return Err("damping cannot be negative".into());
                }
                let mode = state
                    .modes
                    .iter_mut()
                    .find(|m| m.mode_ref == *mode_ref)
                    .ok_or("unknown supplied physical mode")?;
                if mode.damping_per_second.to_bits() != write.expected_value.to_bits() {
                    return Err("mode damping precondition mismatch".into());
                }
                mode.damping_per_second = write.value;
                // frequency_hz, carrier weights, geometry/nodal identity and
                // excitation are deliberately never changed by a material write.
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2RelationPlanContext {
    pub source_score_or_world_ref: String,
    pub situated_provider: Option<M2SituatedProviderBinding>,
    #[serde(default)]
    pub musical: M2MusicalRouteInput,
    #[serde(default)]
    pub material_routes: Vec<M2BoundedRoute>,
    #[serde(default)]
    pub control_routes: Vec<M2BoundedRoute>,
    #[serde(default)]
    pub audio_routes: Vec<M2BoundedRoute>,
    #[serde(default)]
    pub provenance_refs: Vec<String>,
    pub observer_solar_window: Option<M2ObserverSolarWindow>,
    /// Explicit user choice when set; otherwise choose this ordinal from the
    /// stable source/native candidate list. No modulo or invented pitch route.
    pub requested_maqam_index: Option<u8>,
    pub requested_role: Option<CorrespondenceRole>,
    #[serde(default)]
    pub candidate_ordinal: usize,
    #[serde(default)]
    pub source_coordinates: Vec<String>,
    #[serde(default)]
    pub material_writes: Vec<M2MaterialWrite>,
}
impl M2RelationPlanContext {
    fn validate(&self) -> Result<(), String> {
        require_ref("score/world source", &self.source_score_or_world_ref)?;
        for routes in [
            &self.material_routes,
            &self.control_routes,
            &self.audio_routes,
        ] {
            if routes.len() > MAX_ROUTES {
                return Err("route budget exceeded".into());
            }
            for route in routes {
                require_ref("route", &route.route_ref)?;
                require_ref("route provenance", &route.provenance_ref)?;
            }
        }
        for refs in [
            &self.provenance_refs,
            &self.source_coordinates,
            &self.musical.mantra_or_name_route_refs,
        ] {
            if refs.len() > MAX_ROUTES {
                return Err("reference budget exceeded".into());
            }
            for reference in refs {
                require_ref("context reference", reference)?;
            }
        }
        for value in [
            &self.musical.maqam_ref,
            &self.musical.tuning_ref,
            &self.musical.phrase_or_modulation_ref,
        ] {
            if let Some(reference) = value {
                require_ref("musical route", reference)?;
            }
        }
        if let Some(provider) = &self.situated_provider {
            for reference in [
                &provider.planetary_state_ref,
                &provider.chakra_state_ref,
                &provider.elemental_state_ref,
            ] {
                require_ref("situated provider", reference)?;
            }
            for refs in [
                &provider.identity_relation_refs,
                &provider.temporal_provider_refs,
            ] {
                if refs.len() > MAX_ROUTES {
                    return Err("provider reference budget exceeded".into());
                }
                for reference in refs {
                    require_ref("situated evidence", reference)?;
                }
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2NativeAlignment {
    pub maqam_coordinate: String,
    pub planet_coordinate: String,
    pub chakra_coordinate: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2SituatedState {
    pub binding_state: String,
    /// Native input is not the CE compatibility profile. Absence is explicit.
    pub profile_alignment: Option<Value>,
    pub provider_binding: Option<M2SituatedProviderBinding>,
    pub native_alignment: M2NativeAlignment,
    pub planetary_hour: Option<M2PlanetaryHour>,
    pub maqam_candidates: Vec<M2MaqamCandidate>,
    pub selection_policy_ref: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2DetEvidence {
    pub source_index72: u8,
    pub epogdoon_index64: u8,
    pub profile_m2_to_m3_symbol: Option<u8>,
    pub profile_mahamaya_address64: Option<u8>,
    pub native_m2_to_m3_mask: u64,
    pub standing: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2ExecutionRoutes {
    pub audio_octet_hz: [f32; 8],
    pub nodal_quartet: [NodalConstraint; 4],
    pub p_position_element: Option<String>,
    pub l2_prime_element: Option<String>,
    pub musical: M2MusicalRouteInput,
    pub material_routes: Vec<M2BoundedRoute>,
    pub control_routes: Vec<M2BoundedRoute>,
    pub audio_routes: Vec<M2BoundedRoute>,
    pub condition_input: M2ConditionInput,
    pub descriptor_selections: Vec<crate::m2_engine::Selection>,
    /// These are intended excitation/tuning pitches; never body eigenmodes.
    pub intended_tuning_hz: Option<Vec<f64>>,
    pub tuning_standing: String,
    pub material_writes: Vec<M2MaterialWrite>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct M2RelationPlan {
    pub schema: String,
    pub owner: String,
    pub source_score_or_world_ref: String,
    pub address72: M2Address72Views,
    pub situated: M2SituatedState,
    pub det: M2DetEvidence,
    pub execution: M2ExecutionRoutes,
    pub provenance: Vec<String>,
    pub native_extension: String,
    pub identity: EventIdentity,
    pub at_unix_ms: u64,
    pub input_basis_sha256: String,
    pub source_receipts: M2SourceReceipts,
}
#[derive(Debug)]
pub struct PreparedM2Relation {
    pub request: M2Request,
    pub frame: M2Frame,
    pub plan: M2RelationPlan,
}
fn role_kind(role: CorrespondenceRole) -> &'static str {
    match role {
        CorrespondenceRole::Tonic => "TONIC_PLANETARY_RESONANCE",
        CorrespondenceRole::Dominant => "DOMINANT_PLANETARY_RESONANCE",
    }
}
fn unique_target(relations: &[M2SourceRelation], label: &str) -> Result<String, String> {
    let targets: BTreeSet<_> = relations.iter().map(|r| r.to_coordinate.as_str()).collect();
    if targets.len() != 1 {
        return Err(format!("{label}: no unique source-held target"));
    }
    Ok(targets
        .into_iter()
        .next()
        .expect("one source target")
        .into())
}
impl M2RelationPlan {
    /// Production preparation uses the same M2Request/M2Frame producer as the
    /// scene/coupled engine. No graph handles, provider calls or audio IO occur.
    pub fn compile(
        request: &M2Request,
        context: M2RelationPlanContext,
        field: &M2SourceField,
    ) -> Result<PreparedM2Relation, String> {
        context.validate()?;
        if request.registry_revision != field.registry_revision
            || request.registry_revision != native_m_registry().manifest().registry_revision
        {
            return Err("request/source/native registry revisions disagree".into());
        }
        let base_condition = request
            .condition
            .clone()
            .ok_or("relation plan needs existing joint condition input")?;
        let hour = context
            .observer_solar_window
            .as_ref()
            .map(|w| planetary_hour(field, w, request))
            .transpose()?;
        let mut receipts = M2SourceReceipts::new(field);
        let phase = if base_condition.active_mef_condition / 6 >= 6 {
            M2MefPhase::Prime
        } else {
            M2MefPhase::Direct
        };
        let mut candidates = Vec::new();
        let maqams = m2::catalogue().table("maqam")?;
        for index in 0..72usize {
            let coordinate = maqams
                .binding(index)
                .ok_or("unbound canonical maqam leaf")?;
            for role in [CorrespondenceRole::Tonic, CorrespondenceRole::Dominant] {
                let links: Vec<_> = field
                    .relations(coordinate, role_kind(role))
                    .into_iter()
                    .cloned()
                    .collect();
                let Ok(planet) = unique_target(&links, "maqam role") else {
                    continue;
                };
                if hour.as_ref().is_some_and(|h| planet != h.planet_coordinate) {
                    continue;
                }
                let chakras: Vec<_> = field
                    .relations(&planet, "PLANETARY_RESONANCE")
                    .into_iter()
                    .cloned()
                    .collect();
                let Ok(chakra) = unique_target(&chakras, "planetary resonance") else {
                    continue;
                };
                // Existing producer must possess this exact native path too.
                let Some(rule) = correspondence_field().rule(index as u8, role) else {
                    continue;
                };
                if rule.maqam_coordinate != coordinate
                    || rule.planet_coordinate != planet
                    || rule.chakra_coordinate != chakra
                {
                    return Err(
                        "native condition producer disconnected from live typed source path".into(),
                    );
                }
                candidates.push(M2MaqamCandidate {
                    maqam_index: index as u8,
                    role,
                    maqam_coordinate: coordinate.into(),
                    planet_coordinate: planet,
                    chakra_coordinate: chakra,
                });
            }
        }
        if candidates.is_empty() {
            return Err("no connected source maqam candidate for observer hour".into());
        }
        let requested_index = context
            .requested_maqam_index
            .or_else(|| hour.is_none().then_some(base_condition.maqam_index));
        let requested_role = context
            .requested_role
            .or_else(|| hour.is_none().then_some(base_condition.role));
        let filtered: Vec<_> = candidates
            .iter()
            .filter(|c| {
                requested_index.is_none_or(|i| i == c.maqam_index)
                    && requested_role.is_none_or(|r| r == c.role)
            })
            .collect();
        let chosen = (**filtered.get(context.candidate_ordinal).ok_or(
            "requested maqam/role is disconnected from observer hour or candidate ordinal",
        )?)
        .clone();
        if let Some(reference) = &context.musical.maqam_ref {
            if reference != &chosen.maqam_coordinate {
                return Err("musical maqam reference disagrees with selected native path".into());
            }
        }
        // Hour candidate lists are dependent on the actual source roster. A
        // changed other candidate changes this choice operation; explicit plans
        // depend only on their selected role path.
        if hour.is_some() {
            for index in 0..72usize {
                let coordinate = maqams.binding(index).ok_or("unbound canonical maqam")?;
                for role in [CorrespondenceRole::Tonic, CorrespondenceRole::Dominant] {
                    receipts.retain_edges(field, coordinate, role_kind(role))?;
                }
            }
            receipts.retain_node(field, "#2-5", phase)?;
            receipts.retain_node(field, "#2-5-0/1-0", phase)?;
            for index in 1..8usize {
                let coordinate = m2::catalogue()
                    .table("planet")?
                    .binding(index)
                    .ok_or("unbound classical planet")?;
                receipts.retain_node(field, coordinate, phase)?;
                receipts.retain_edges(field, coordinate, "PLANETARY_RESONANCE")?;
            }
        }
        for coordinate in [
            "#2-0",
            "#2-1",
            chosen.maqam_coordinate.as_str(),
            chosen.planet_coordinate.as_str(),
            chosen.chakra_coordinate.as_str(),
        ] {
            receipts.retain_node(field, coordinate, phase)?;
        }
        for selection in &request.selections {
            let coordinate = m2::catalogue()
                .table(&selection.table)?
                .binding(selection.index)
                .ok_or("selected descriptor has no native descendant")?;
            receipts.retain_node(field, coordinate, phase)?;
        }
        for coordinate in &context.source_coordinates {
            receipts.retain_node(field, coordinate, phase)?;
        }
        for coordinate in &context.musical.mantra_or_name_route_refs {
            receipts.retain_node(field, coordinate, phase)?;
        }
        let musical =
            receipts.retain_edges(field, &chosen.maqam_coordinate, role_kind(chosen.role))?;
        if unique_target(&musical, "selected musical role")? != chosen.planet_coordinate {
            return Err("selected musical edge disconnected".into());
        }
        let planetary =
            receipts.retain_edges(field, &chosen.planet_coordinate, "PLANETARY_RESONANCE")?;
        if unique_target(&planetary, "selected planetary resonance")? != chosen.chakra_coordinate {
            return Err("selected planetary edge disconnected".into());
        }
        for key in [
            "c_2_tonic_note",
            "c_2_dominant_note",
            "c_2_ajnas",
            "intervalStructure",
        ] {
            receipts.retain_property(field, &chosen.maqam_coordinate, key)?;
        }
        for write in &context.material_writes {
            receipts.retain_node(field, &write.source_coordinate, phase)?;
            if field
                .property(&write.source_coordinate, &write.source_property)
                .is_none()
            {
                return Err("material interpretation lacks its exact source claim".into());
            }
            receipts.retain_property(field, &write.source_coordinate, &write.source_property)?;
        }
        let mut prepared = request.clone();
        let mut condition = base_condition;
        condition.maqam_index = chosen.maqam_index;
        condition.role = chosen.role;
        prepared.condition = Some(condition.clone());
        for coordinate in &context.musical.mantra_or_name_route_refs {
            let mut found = false;
            for table in m2::catalogue()
                .tables()
                .iter()
                .filter(|t| matches!(t.name(), "shem" | "asma" | "mantra" | "station"))
            {
                for index in 0..table.rows().len() {
                    if table.binding(index) == Some(coordinate.as_str()) {
                        found = true;
                        if !prepared
                            .selections
                            .iter()
                            .any(|s| s.table == table.name() && s.index == index)
                        {
                            prepared.selections.push(crate::m2_engine::Selection {
                                table: table.name().into(),
                                index,
                            });
                        }
                    }
                }
            }
            if !found {
                return Err("name/mantra route has no existing native descriptor consumer".into());
            }
        }
        apply_material(&mut prepared, &context.material_writes)?;
        let frame = prepared.execute()?;
        let produced = frame
            .condition
            .as_ref()
            .ok_or("native condition producer disconnected")?;
        let path = produced
            .source_path
            .as_ref()
            .ok_or("native condition did not consume source path")?;
        if path.maqam_coordinate != chosen.maqam_coordinate
            || path.planet_coordinate != chosen.planet_coordinate
            || path.chakra_coordinate != chosen.chakra_coordinate
        {
            return Err("condition consumer changed compiled relation path".into());
        }
        let address = M2Address72Views::decode(usize::from(condition.active_mef_condition))?;
        let expected_sublens = Reading72::new(Register72::Mef, address.index72)?
            .mef_sublens()?
            .to_string();
        if produced.sublens_ref != expected_sublens {
            return Err("native MEF condition identity mismatch".into());
        }
        let vim = frame
            .vimarsha
            .as_ref()
            .ok_or("native Vimarsha drive disconnected")?;
        let mut provenance = vec![
            M2_DOMAIN_SPEC_REF.into(),
            M2_SITUATED_LOCK_REF.into(),
            format!("native-registry:{}", field.registry_revision),
            format!("bimba-read:{}", field.source_revision),
        ];
        provenance.extend(context.provenance_refs);
        let pitches =
            (!produced.musical.pitches_hz.is_empty()).then(|| produced.musical.pitches_hz.clone());
        let retained_candidates = if hour.is_some() {
            candidates
        } else {
            vec![chosen.clone()]
        };
        let plan = Self {
            schema: M2_RELATION_PLAN_SCHEMA.into(),
            owner: M2_RELATION_PLAN_OWNER.into(),
            source_score_or_world_ref: context.source_score_or_world_ref,
            address72: address.clone(),
            situated: M2SituatedState {
                binding_state: if hour.is_some() {
                    "observer-solar-provider-bound"
                } else if context.situated_provider.is_some() {
                    "references-supplied;solar-provider-unavailable"
                } else {
                    "pending-provider-binding"
                }
                .into(),
                profile_alignment: None,
                provider_binding: context.situated_provider,
                native_alignment: M2NativeAlignment {
                    maqam_coordinate: chosen.maqam_coordinate,
                    planet_coordinate: chosen.planet_coordinate,
                    chakra_coordinate: chosen.chakra_coordinate,
                },
                planetary_hour: hour,
                maqam_candidates: retained_candidates,
                selection_policy_ref:
                    "ql.m2-source-candidates/native-row-role-order-explicit-ordinal-v1".into(),
            },
            det: M2DetEvidence {
                source_index72: address.index72,
                epogdoon_index64: m2::scalar_compress(address.index72)?,
                profile_m2_to_m3_symbol: None,
                profile_mahamaya_address64: None,
                native_m2_to_m3_mask: m2::legacy_det(&[address.index72])?,
                standing:
                    "native-scalar-floor-and-retained-OR-mask-distinct;compatibility-profile-absent"
                        .into(),
            },
            execution: M2ExecutionRoutes {
                audio_octet_hz: vim.reading.audio_octet_hz,
                nodal_quartet: vim.reading.nodal_quartet,
                p_position_element: (!path.element_literal.is_empty())
                    .then(|| path.element_literal.clone()),
                l2_prime_element: None,
                musical: context.musical,
                material_routes: context.material_routes,
                control_routes: context.control_routes,
                audio_routes: context.audio_routes,
                condition_input: condition,
                descriptor_selections: prepared.selections.clone(),
                intended_tuning_hz: pitches,
                tuning_standing: if produced.musical.pitches_hz.is_empty() {
                    "unavailable-complete-source-intervals-absent;no-retained-fallback".into()
                } else {
                    produced.musical.standing.clone()
                },
                material_writes: context.material_writes,
            },
            provenance,
            native_extension: M2_NATIVE_EXTENSION.into(),
            identity: frame.identity.clone(),
            at_unix_ms: frame.at_unix_ms,
            input_basis_sha256: hash_bytes(
                serde_json::to_string(request)
                    .map_err(|e| e.to_string())?
                    .as_bytes(),
            ),
            source_receipts: receipts,
        };
        plan.source_receipts.validate_against(field)?;
        Ok(PreparedM2Relation {
            request: prepared,
            frame,
            plan,
        })
    }
    /// Control-thread replay into the existing engine input. Audio consumers
    /// consume the resulting frame's drive, not this source field.
    pub fn prepare_request(
        &self,
        request: &M2Request,
        field: &M2SourceField,
    ) -> Result<M2Request, String> {
        if self.schema != M2_RELATION_PLAN_SCHEMA
            || self.owner != M2_RELATION_PLAN_OWNER
            || self.native_extension != M2_NATIVE_EXTENSION
            || request.stamp.identity != self.identity
            || request.at_unix_ms != self.at_unix_ms
            || hash_bytes(
                serde_json::to_string(request)
                    .map_err(|e| e.to_string())?
                    .as_bytes(),
            ) != self.input_basis_sha256
        {
            return Err("stale, cross-event or unsupported relation plan".into());
        }
        self.source_receipts.validate_against(field)?;
        if let Some(hour) = &self.situated.planetary_hour {
            if !(hour.begins_unix_ms..hour.ends_unix_ms).contains(&request.at_unix_ms) {
                return Err("planetary hour expired".into());
            }
        }
        let mut prepared = request.clone();
        prepared.condition = Some(self.execution.condition_input.clone());
        prepared.selections = self.execution.descriptor_selections.clone();
        apply_material(&mut prepared, &self.execution.material_writes)?;
        prepared.validate()?;
        Ok(prepared)
    }
}
