//! Paraśakti's complete retained descriptor field and finite operations.
//!
//! The common M registry is the coordinate authority. This module neither
//! invents missing nodes nor promotes every retained correspondence to canon.
//! A shared 72-byte carrier supports four distinct readings; its index is not
//! a semantic equivalence. The historical scalar/OR transforms and current
//! elemental modal transform remain separately named operations.
use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::m_tree::{MTreeId, MTreeNode, MTreeRelation, native_m_registry};
use crate::{Amplitude, ContextFrameId, LensId, LensRef, SublensRef, TemplateureField};

pub const ENGINE_CONTRACT: &str = "ql.m2-engine/v1";
pub const CATALOGUE_SCHEMA: &str = "ql.m2-retained-c/v1";
pub const RETAINED_STANDING: &str = "retained-source-reading-not-whole-subsystem-parity";
pub const MAX_COMPONENT: i64 = 1_000_000;
pub const RETAINED_C_JSON: &str = include_str!("../../../fixtures/kernel/m2-retained-c-v1.json");
const TABLE_NAMES: [&str; 16] = [
    "carrier",
    "mef",
    "tattva",
    "decan",
    "planet",
    "chakra",
    "shem",
    "ratio",
    "maqam",
    "station",
    "asma",
    "mantra",
    "element",
    "det",
    "resonance",
    "routing",
];
const TABLE_COUNTS: [usize; 16] = [
    72, 72, 36, 73, 10, 8, 72, 10, 72, 24, 100, 100, 5, 72, 36, 2,
];
const TABLE_WIDTHS: [usize; 16] = [1, 5, 5, 6, 9, 4, 7, 2, 11, 3, 8, 6, 4, 1, 1, 2];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceLock {
    pub path: String,
    pub sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedTable {
    name: String,
    symbol: String,
    scope: String,
    columns: Vec<String>,
    rows: Vec<Vec<u64>>,
    bindings: Vec<Option<String>>,
}
impl RetainedTable {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn source_symbol(&self) -> &str {
        &self.symbol
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn columns(&self) -> &[String] {
        &self.columns
    }
    pub fn rows(&self) -> &[Vec<u64>] {
        &self.rows
    }
    pub fn binding(&self, row: usize) -> Option<&str> {
        self.bindings.get(row).and_then(Option::as_deref)
    }
    pub fn row(&self, index: usize) -> Result<&[u64], String> {
        self.rows
            .get(index)
            .map(Vec::as_slice)
            .ok_or_else(|| format!("{} row {index} out of range", self.name))
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalogue {
    schema: String,
    registry_revision: String,
    standing: String,
    sources: Vec<SourceLock>,
    tables: Vec<RetainedTable>,
}
impl Catalogue {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let registry = native_m_registry();
        if value.schema != CATALOGUE_SCHEMA
            || value.standing != RETAINED_STANDING
            || value.registry_revision != registry.manifest().registry_revision
            || value.tables.len() != TABLE_NAMES.len()
        {
            return Err("unsupported or stale M2 catalogue".into());
        }
        let root = registry.root(2).ok_or("missing common M2 root")?.id;
        for (i, table) in value.tables.iter().enumerate() {
            if table.name != TABLE_NAMES[i]
                || table.rows.len() != TABLE_COUNTS[i]
                || table.bindings.len() != table.rows.len()
                || table.columns.len() != TABLE_WIDTHS[i]
                || table.symbol.trim().is_empty()
            {
                return Err(format!("invalid M2 table {}", table.name));
            }
            let scope = registry
                .resolve(&table.scope)
                .ok_or("unknown M2 table scope")?;
            if scope.root_id != root {
                return Err("M2 table scope outside M2".into());
            }
            for (row, binding) in table.rows.iter().zip(&table.bindings) {
                if row.len() != table.columns.len() {
                    return Err("invalid M2 row width".into());
                }
                if !matches!(i, 13..=15) && row.iter().any(|v| *v > u16::MAX as u64) {
                    return Err("M2 descriptor exceeds retained scalar representation".into());
                }
                if let Some(reference) = binding {
                    let node = registry
                        .resolve(reference)
                        .ok_or("unknown exact M2 binding")?;
                    if node.root_id != root {
                        return Err("M2 binding outside subsystem".into());
                    }
                    let mut current = node;
                    while current.id != scope.id {
                        current = registry
                            .parent(current.id)
                            .ok_or("binding outside table scope")?;
                    }
                }
            }
        }
        if value.sources.len() != 2
            || value.sources.iter().any(|s| {
                s.path.trim().is_empty()
                    || s.sha256.len() != 64
                    || !s.sha256.bytes().all(|b| b.is_ascii_hexdigit())
            })
        {
            return Err("missing retained-source locks".into());
        }
        Ok(value)
    }
    pub fn registry_revision(&self) -> &str {
        &self.registry_revision
    }
    pub fn sources(&self) -> &[SourceLock] {
        &self.sources
    }
    pub fn tables(&self) -> &[RetainedTable] {
        &self.tables
    }
    pub fn table(&self, name: &str) -> Result<&RetainedTable, String> {
        self.tables
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| format!("unknown M2 table {name}"))
    }
    pub fn reading(&self, table: &str, index: usize) -> Result<DescriptorReading, String> {
        let table = self.table(table)?;
        let row = table.row(index)?;
        let reference = table.binding(index);
        let registry = native_m_registry();
        let node = reference.and_then(|s| registry.resolve(s));
        Ok(DescriptorReading {
            table: table.name.clone(),
            index,
            source_symbol: table.symbol.clone(),
            structural_scope: table.scope.clone(),
            standing: RETAINED_STANDING.into(),
            exact_coordinate: node.map(|n| n.source_ref.clone()),
            coordinate_id: node.map(|n| n.id),
            source_names: node.map(|n| n.names.clone()).unwrap_or_default(),
            coordinate_binding_basis: binding_basis(table.name()).into(),
            fields: table
                .columns
                .iter()
                .cloned()
                .zip(row.iter().copied())
                .collect(),
            source_relation_refs: node
                .map(|n| {
                    registry
                        .relations_for(n.id)
                        .map(|r| r.relation_ref.clone())
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
    /// All actual coordinates, not just leaves with a retained numeric record.
    pub fn coordinates(&self) -> Vec<CoordinateHolding> {
        m2_coordinates()
            .map(|node| {
                let records = self
                    .tables
                    .iter()
                    .flat_map(|table| {
                        table
                            .bindings
                            .iter()
                            .enumerate()
                            .filter(|(_, r)| r.as_deref() == Some(node.source_ref.as_str()))
                            .map(move |(i, _)| format!("{}:{i}", table.name))
                    })
                    .collect::<Vec<_>>();
                CoordinateHolding {
                    coordinate: node.source_ref.clone(),
                    id: node.id,
                    exact_records: records,
                    structural_only_is_not_computation: true,
                }
            })
            .collect()
    }
}
pub fn catalogue() -> &'static Catalogue {
    static VALUE: OnceLock<Catalogue> = OnceLock::new();
    VALUE.get_or_init(|| {
        Catalogue::from_json(RETAINED_C_JSON).expect("compiled M2 catalogue is valid")
    })
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptorReading {
    pub table: String,
    pub index: usize,
    pub source_symbol: String,
    pub structural_scope: String,
    pub standing: String,
    pub exact_coordinate: Option<String>,
    pub coordinate_id: Option<MTreeId>,
    pub source_names: Vec<String>,
    pub coordinate_binding_basis: String,
    #[serde(with = "decimal_fields")]
    pub fields: BTreeMap<String, u64>,
    pub source_relation_refs: Vec<String>,
}
/// Decimal-string wire integers avoid JavaScript rounding of bitmask fields.
mod decimal_fields {
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    pub fn serialize<S: serde::Serializer>(
        value: &BTreeMap<String, u64>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value
            .iter()
            .map(|(key, value)| (key, value.to_string()))
            .collect::<BTreeMap<_, _>>()
            .serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<BTreeMap<String, u64>, D::Error> {
        BTreeMap::<String, String>::deserialize(deserializer)?
            .into_iter()
            .map(|(key, text)| {
                let value = text.parse::<u64>().map_err(serde::de::Error::custom)?;
                if value.to_string() != text {
                    return Err(serde::de::Error::custom("noncanonical decimal u64"));
                }
                Ok((key, value))
            })
            .collect()
    }
}
pub fn binding_basis(table: &str) -> &'static str {
    match table {
        "planet" | "asma" => "unique-normalized-source-name; values-not-equated",
        "station" => "source-station-name-then-level; values-not-equated",
        "tattva" | "element" => "explicit-principle-coordinate; missing-principles-unbound",
        "decan" | "shem" | "maqam" | "chakra" | "mantra" => {
            "explicit-source-structural-slot; semantic-equality-unproven"
        }
        _ => "branch-law; not-an-invented-leaf-coordinate",
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CoordinateHolding {
    pub coordinate: String,
    pub id: MTreeId,
    pub exact_records: Vec<String>,
    pub structural_only_is_not_computation: bool,
}
pub fn m2_coordinates() -> impl Iterator<Item = &'static MTreeNode> {
    let r = native_m_registry();
    let root = r.root(2).expect("common M2 root").id;
    r.manifest().nodes.iter().filter(move |n| n.root_id == root)
}
pub fn m2_relations() -> impl Iterator<Item = &'static MTreeRelation> {
    let r = native_m_registry();
    let root = r.root(2).expect("common M2 root").id;
    r.manifest().relations.iter().filter(move |rel| {
        [rel.from_id, rel.to_id]
            .into_iter()
            .flatten()
            .any(|id| r.node(id).is_some_and(|n| n.root_id == root))
    })
}
/// Actual source relation edges; the legacy same-position mask is NOT this field.
pub fn causal_resonances(reference: &str) -> Result<Vec<&'static MTreeRelation>, String> {
    let r = native_m_registry();
    let node = r.resolve(reference).ok_or("unknown coordinate")?;
    if node.root_id != r.root(2).expect("M2 root").id {
        return Err("not an M2 coordinate".into());
    }
    Ok(r.relations_for(node.id)
        .filter(|rel| rel.source_kind == "CAUSAL_RESONANCE")
        .collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Register72 {
    Mef,
    Tattva,
    Decan,
    Shem,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reading72 {
    register: Register72,
    index: u8,
}
impl Reading72 {
    pub fn new(register: Register72, index: u8) -> Result<Self, String> {
        if index >= 72 {
            return Err("M2 carrier index must be below 72".into());
        }
        Ok(Self { register, index })
    }
    pub fn from_axes(register: Register72, a: u8, b: u8, c: u8, d: u8) -> Result<Self, String> {
        let axes = [a, b, c, d];
        let dims = match register {
            Register72::Mef => [12, 6, 1, 1],
            Register72::Tattva => [36, 2, 1, 1],
            Register72::Decan => [4, 3, 3, 2],
            Register72::Shem => [8, 9, 1, 1],
        };
        if axes.iter().zip(dims).any(|(v, dim)| *v >= dim) {
            return Err("invalid reading axes".into());
        }
        let index = axes
            .iter()
            .zip(dims)
            .fold(0u16, |n, (v, dim)| n * u16::from(dim) + u16::from(*v));
        Self::new(register, index as u8)
    }
    pub const fn index(self) -> u8 {
        self.index
    }
    pub const fn register(self) -> Register72 {
        self.register
    }
    pub fn axes(self) -> [u8; 4] {
        let i = self.index;
        match self.register {
            Register72::Mef => [i / 6, i % 6, 0, 0],
            Register72::Tattva => [i / 2, i % 2, 0, 0],
            Register72::Decan => [i / 18, i % 18 / 6, i % 6 / 2, i % 2],
            Register72::Shem => [i / 9, i % 9, 0, 0],
        }
    }
    /// Explicit carrier reinterpretation, not a correspondence assertion.
    pub const fn reinterpret_carrier(self, register: Register72) -> Self {
        Self {
            register,
            index: self.index,
        }
    }
    pub fn mef_sublens(self) -> Result<SublensRef, String> {
        if self.register != Register72::Mef {
            return Err("not a MEF reading".into());
        }
        let row = self.index / 6;
        // C groups six direct then six prime; LensId::ALL INTERLEAVES twins.
        let slot = (row % 6) * 2 + u8::from(row >= 6);
        SublensRef::canonical(LensId::ALL[slot as usize], self.index % 6).map_err(|e| e.to_string())
    }
    pub fn from_sublens(s: SublensRef) -> Self {
        let lens = s.lens().lens();
        let row = lens.index() + if lens.slot() % 2 == 1 { 6 } else { 0 };
        Self {
            register: Register72::Mef,
            index: row * 6 + s.position().value(),
        }
    }
}
pub fn context_condition(frame: ContextFrameId, lens: LensId) -> Reading72 {
    let s = SublensRef::new(
        LensRef::canonical(lens),
        frame.canonical_selection().local_position().value(),
    )
    .expect("canonical Context Frame sublens");
    Reading72::from_sublens(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TattvaPhase {
    Manifestation,
    Reabsorption,
}
pub fn tattva_step(principle: u8, phase: TattvaPhase) -> Result<Option<u8>, String> {
    if principle >= 36 {
        return Err("tattva outside 0..36".into());
    }
    Ok(match phase {
        TattvaPhase::Manifestation => (principle < 35).then_some(principle + 1),
        TattvaPhase::Reabsorption => principle.checked_sub(1),
    })
}
pub fn elemental_signature(element: u8, chakra: u8, phase: u8) -> Result<u8, String> {
    if element >= 5 || chakra >= 8 || phase >= 4 {
        return Err("invalid element/chakra/phase".into());
    }
    Ok(element | (chakra << 3) | (phase << 6))
}
pub fn unpack_signature(signature: u8) -> Result<[u8; 3], String> {
    if signature & 7 >= 5 {
        return Err("signature has reserved element".into());
    }
    Ok([signature & 7, (signature >> 3) & 7, signature >> 6])
}
pub fn decan_to_fibre(index: u8) -> Result<u8, String> {
    if index >= 72 {
        return Err("invalid decan carrier index".into());
    }
    Ok([1, 0, 3, 2][(index / 18) as usize] * 18 + index % 18)
}
pub fn fibre_target(index: u8) -> Result<u8, String> {
    if index >= 72 {
        return Err("invalid elemental fibre index".into());
    }
    Ok((index / 18) * 16
        + ql_core::Transduction18to16::new(index % 18)
            .map_err(|e| e.to_string())?
            .target())
}
pub fn scalar_compress(index: u8) -> Result<u8, String> {
    if index >= 72 {
        return Err("invalid scalar source index".into());
    }
    Ok((u16::from(index) * 8 / 9) as u8)
}
pub fn scalar_expand(index: u8) -> Result<u8, String> {
    if index >= 64 {
        return Err("invalid scalar form index".into());
    }
    Ok((u16::from(index) * 9 / 8) as u8)
}
pub fn legacy_det(indices: &[u8]) -> Result<u64, String> {
    let table = catalogue().table("det")?;
    indices
        .iter()
        .try_fold(0, |mask, i| Ok(mask | table.row(usize::from(*i))?[0]))
}
pub fn asma_is_projective(index: u8) -> Result<bool, String> {
    if index >= 100 {
        return Err("Asma index outside 99+1".into());
    }
    let row = catalogue().table("routing")?.row(1)?;
    Ok((row[usize::from(index / 64)] >> (index % 64)) & 1 != 0)
}
pub fn asma_is_internal(index: u8) -> Result<bool, String> {
    if index >= 100 {
        return Err("Asma index outside 99+1".into());
    }
    let row = catalogue().table("routing")?.row(0)?;
    Ok((row[usize::from(index / 64)] >> (index % 64)) & 1 != 0)
}
pub fn planet_is_preempted(index: u8) -> Result<bool, String> {
    if index >= 10 {
        return Err("planet index outside mod-10".into());
    }
    Ok(index >= 7)
}
pub const fn digital_root(value: u64) -> u8 {
    if value == 0 {
        0
    } else {
        (1 + (value - 1) % 9) as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Aspect {
    pub kind: Option<u8>,
    pub angle: f64,
    pub orb: f64,
}
pub fn aspect(a: f64, b: f64) -> Result<Aspect, String> {
    if !a.is_finite() || !b.is_finite() || !(0.0..360.0).contains(&a) || !(0.0..360.0).contains(&b)
    {
        return Err("planetary longitudes must be finite in [0,360)".into());
    }
    let mut angle = (a - b).abs();
    if angle > 180.0 {
        angle = 360.0 - angle;
    }
    let mut result = Aspect {
        kind: None,
        angle,
        orb: 999.0,
    };
    for (i, (target, limit)) in [
        (0.0, 10.0),
        (60.0, 6.0),
        (90.0, 8.0),
        (120.0, 8.0),
        (180.0, 10.0),
    ]
    .iter()
    .enumerate()
    {
        let orb = (angle - target).abs();
        if orb <= *limit && orb < result.orb {
            result.kind = Some(i as u8);
            result.orb = orb;
        }
    }
    Ok(result)
}
/// A supplied longitude, not an ephemeris or a claim to live sky observation.
pub fn situated_decan(longitude: f64, shadow: bool) -> Result<Reading72, String> {
    if !longitude.is_finite() || !(0.0..360.0).contains(&longitude) {
        return Err("invalid longitude".into());
    }
    let sign = (longitude / 30.0).floor() as u8;
    let decan = ((longitude % 30.0) / 10.0).floor() as u8;
    Reading72::from_axes(
        Register72::Decan,
        sign % 4,
        sign / 4,
        decan,
        u8::from(shadow),
    )
}
/// Equal-quarter-tone playback of the retained declared pattern. Not a claim
/// that this is the only tuning, nor a normalization to some other maqam.
pub fn maqam_pitches(mode: u8, root_hz: f64) -> Result<[f64; 8], String> {
    if mode >= 72 || !root_hz.is_finite() || root_hz <= 0.0 {
        return Err("invalid mode or root frequency".into());
    }
    let row = catalogue().table("maqam")?.row(usize::from(mode))?;
    let mut result = [root_hz; 8];
    let mut quartertones = 0u64;
    for i in 1..8 {
        quartertones += row[1 + i];
        result[i] = root_hz * 2f64.powf(quartertones as f64 / 24.0);
        if !result[i].is_finite() {
            return Err("frequency overflow".into());
        }
    }
    Ok(result)
}
/// Typed cross-table links retain their register; a same integer is never used
/// as an implicit element/planet/decan identity conversion.
pub fn linked_readings(table: &str, index: usize) -> Result<Vec<DescriptorReading>, String> {
    let r = catalogue().table(table)?.row(index)?;
    let links: Vec<(&str, usize)> = match table {
        "shem" => vec![
            ("element", r[3] as usize),
            ("decan", r[4] as usize),
            ("planet", r[5] as usize),
        ],
        "decan" => {
            // Decan F/E/A/W/quintessence is not the five-element ID order.
            let element = catalogue()
                .table("element")?
                .rows()
                .iter()
                .position(|throughline| throughline[1] == r[0])
                .ok_or("decan element has no retained throughline")?;
            let mut links = vec![("element", element)];
            if index < 72 {
                links.push(("planet", r[4] as usize));
            }
            links
        }
        "element" => vec![("tattva", r[0] as usize), ("chakra", r[3] as usize)],
        "maqam" => vec![("ratio", r[0] as usize), ("planet", r[9] as usize)],
        "mantra" => vec![("element", r[2] as usize)],
        "asma" => vec![("element", r[3] as usize)],
        _ => Vec::new(),
    };
    links
        .into_iter()
        .map(|(t, i)| catalogue().reading(t, i))
        .collect()
}

/// A checked boundary around the existing Templateure authority. Keeping a
/// private bounded field prevents its unchecked i64 helpers from seeing MIN,
/// wraparound sums, or impossible accumulated power through this engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalField {
    field: TemplateureField,
    coefficients: [[i64; 2]; 72],
}
impl ModalField {
    pub fn new(coefficients: &[[i64; 2]]) -> Result<Self, String> {
        let coefficients: [[i64; 2]; 72] = coefficients
            .try_into()
            .map_err(|_| "expected exactly 72 distributed coefficients")?;
        if coefficients
            .iter()
            .flatten()
            .any(|v| !(-MAX_COMPONENT..=MAX_COMPONENT).contains(v))
        {
            return Err("modal coefficient exceeds exact engine bound".into());
        }
        let rows = std::array::from_fn::<_, 4, _>(|f| {
            std::array::from_fn::<_, 18, _>(|s| {
                let a = coefficients[f * 18 + s];
                Amplitude::new(a[0], a[1])
            })
        });
        Ok(Self {
            field: TemplateureField::from_amplitudes(rows[0], rows[1], rows[2], rows[3]),
            coefficients,
        })
    }
    pub fn coefficients(&self) -> &[[i64; 2]; 72] {
        &self.coefficients
    }
    pub fn templateure(&self) -> &TemplateureField {
        &self.field
    }
    pub fn quadrature(&self) -> Self {
        let coefficients = self.coefficients.map(|[re, im]| [-im, re]);
        Self {
            field: self.field.quadrature(),
            coefficients,
        }
    }
    pub fn total_power(&self) -> u128 {
        self.field.total_power()
    }
    pub fn form_potential(&self) -> [[i64; 2]; 64] {
        let form = self.field.transduce();
        let elements = [
            ql_core::Element::Earth,
            ql_core::Element::Fire,
            ql_core::Element::Water,
            ql_core::Element::Air,
        ];
        std::array::from_fn(|i| {
            let a = form.amplitude(elements[i / 16], (i % 16) as u8);
            [a.re, a.im]
        })
    }
}

/// Ordered primitive ground factors, matching the native C ground descriptor.
/// This polynomial/factorisation is not a metaphysical or physical proof.
pub const fn ground_factors() -> [i16; 26] {
    [
        -1, -1, 1, 5, 7, 24, 12, 6, 36, 2, 4, 3, 3, 2, 8, 9, 4, 18, 4, 16, 99, 1, 50, 50, 8, 3,
    ]
}
