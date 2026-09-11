//! Complete source-coordinate readings of M3, with typed finite projections.
//! The K2 registry owns identity. These payloads retain source standing and every
//! qualified/null/duplicate relation; they are not a second mutable Bimba graph.
use crate::m_tree::{MTreeId, native_m_registry};
use ql_core::Codon64;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

pub const DOMAIN_SCHEMA: &str = "ql.m3-domain/v1";
pub const DOMAIN_JSON: &str = include_str!("../../../fixtures/kernel/m3-domain-v1.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceNode {
    pub id: MTreeId,
    pub parent_id: Option<MTreeId>,
    #[serde(rename = "ref")]
    pub reference: String,
    pub role: String,
    pub record: usize,
    pub source_record_index: usize,
    pub properties: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRelation {
    pub id: MTreeId,
    pub from_id: Option<MTreeId>,
    pub to_id: Option<MTreeId>,
    pub from_ref: Option<String>,
    pub to_ref: Option<String>,
    pub kind: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub record: usize,
    pub source_record_index: usize,
    pub properties: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackboneProjection {
    pub id: MTreeId,
    #[serde(rename = "ref")]
    pub reference: String,
    pub codon_id: Option<MTreeId>,
    pub hexagram_id: Option<MTreeId>,
    pub codon_address: Option<u8>,
    pub hexagram_address: Option<u8>,
    pub source_record: usize,
    pub standing: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixCell {
    pub id: MTreeId,
    #[serde(rename = "ref")]
    pub reference: String,
    pub family: u8,
    pub address: u8,
    pub hexagram_ref: String,
    pub resolves_relation: MTreeId,
    pub pair_relations: Vec<MTreeId>,
    pub codon_relations: Vec<MTreeId>,
}
#[derive(Debug, Clone, Serialize)]
pub struct GeneticReading<'a> {
    pub dna: &'a SourceNode,
    pub phase: &'a SourceNode,
    pub rna: Option<&'a SourceNode>,
    /// Includes the distinct Start and amino-acid result for ATG, and conditional
    /// translations without treating their conditions as currently satisfied.
    pub translations: Vec<&'a SourceRelation>,
    pub expression: Vec<&'a SourceRelation>,
    pub standing: &'static str,
}

pub struct M3Source {
    data: Value,
    nodes: Vec<SourceNode>,
    relations: Vec<SourceRelation>,
    backbones: Vec<BackboneProjection>,
    cells: Vec<MatrixCell>,
    by_id: BTreeMap<MTreeId, usize>,
    by_ref: BTreeMap<String, usize>,
    edges: BTreeMap<MTreeId, usize>,
    outgoing: BTreeMap<MTreeId, Vec<usize>>,
    dna: [usize; 64],
    phase: [usize; 64],
    rna: [Option<usize>; 64],
}
impl M3Source {
    fn load() -> Result<Self, String> {
        let data: Value = serde_json::from_str(DOMAIN_JSON).map_err(|e| e.to_string())?;
        let registry = native_m_registry();
        if data["schema"] != DOMAIN_SCHEMA
            || data["registry_revision"] != registry.manifest().registry_revision
            || data["standing"] != "source-reading-not-blanket-semantic-parity"
        {
            return Err("invalid M3 source contract".into());
        }
        let nodes: Vec<SourceNode> =
            serde_json::from_value(data["nodes"].clone()).map_err(|e| e.to_string())?;
        let relations: Vec<SourceRelation> =
            serde_json::from_value(data["relations"].clone()).map_err(|e| e.to_string())?;
        let backbones: Vec<BackboneProjection> =
            serde_json::from_value(data["backbones"].clone()).map_err(|e| e.to_string())?;
        let cells: Vec<MatrixCell> =
            serde_json::from_value(data["matrix_cells"].clone()).map_err(|e| e.to_string())?;
        if nodes.len() != 996
            || relations.len() != 4891
            || backbones.len() != 24
            || cells.len() != 184
        {
            return Err("incomplete M3 source contract".into());
        }
        let mut by_id = BTreeMap::new();
        let mut by_ref = BTreeMap::new();
        for (i, n) in nodes.iter().enumerate() {
            let actual = registry.node(n.id).ok_or("unknown source node")?;
            if actual.source_ref != n.reference
                || actual.parent_id != n.parent_id
                || actual.root_position != Some(3)
                || by_id.insert(n.id, i).is_some()
                || by_ref.insert(n.reference.clone(), i).is_some()
            {
                return Err("source node identity drift".into());
            }
        }
        let canonical: BTreeMap<_, _> = registry
            .manifest()
            .relations
            .iter()
            .map(|r| (r.id, r))
            .collect();
        let mut edges = BTreeMap::new();
        let mut outgoing: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for (i, e) in relations.iter().enumerate() {
            let actual = canonical.get(&e.id).ok_or("unknown source relation")?;
            if actual.from_id != e.from_id
                || actual.to_id != e.to_id
                || actual.source_kind != e.kind
                || actual.record != e.record
                || edges.insert(e.id, i).is_some()
            {
                return Err("source relation identity drift".into());
            }
            if let Some(id) = e.from_id {
                outgoing.entry(id).or_default().push(i);
            }
        }
        let sequence_index = |sequence: &str| -> Result<usize, String> {
            if sequence.len() != 3 {
                return Err("invalid source codon sequence".into());
            }
            sequence.bytes().try_fold(0usize, |a, n| {
                b"ATCG"
                    .iter()
                    .position(|b| *b == n)
                    .map(|i| (a << 2) | i)
                    .ok_or_else(|| "invalid source nucleotide".into())
            })
        };
        let mut dna = [usize::MAX; 64];
        let mut phase = [usize::MAX; 64];
        let mut rna = [None; 64];
        for (i, n) in nodes.iter().enumerate() {
            if n.role == "dna-codon" || n.role == "phase-codon" {
                let seq = n.properties["sequence"]
                    .as_str()
                    .ok_or("source codon missing sequence")?;
                let index = sequence_index(seq)?;
                let target = if n.role == "dna-codon" {
                    &mut dna[index]
                } else {
                    &mut phase[index]
                };
                if *target != usize::MAX {
                    return Err("duplicate source codon".into());
                }
                *target = i;
            }
        }
        if dna.contains(&usize::MAX) || phase.contains(&usize::MAX) {
            return Err("missing source codon".into());
        }
        for n in &data["genetics"]["rna"]
            .as_array()
            .ok_or("RNA projection missing")?
            .clone()
        {
            let index = sequence_index(n["dna"].as_str().ok_or("invalid RNA source")?)?;
            let reference = n["rna_ref"].as_str().ok_or("RNA source ref missing")?;
            rna[index] = Some(*by_ref.get(reference).ok_or("unbound RNA source")?);
        }
        if rna.iter().flatten().count() != 37 {
            return Err("incomplete RNA source field".into());
        }
        let mut seen = BTreeSet::new();
        for c in &cells {
            if c.family >= 3
                || c.address >= 64
                || !seen.insert((c.family, c.address))
                || !by_id.contains_key(&c.id)
                || !by_ref.contains_key(&c.hexagram_ref)
                || c.pair_relations
                    .iter()
                    .chain(&c.codon_relations)
                    .chain([&c.resolves_relation])
                    .any(|id| !edges.contains_key(id))
            {
                return Err("invalid source matrix cell".into());
            }
        }
        Ok(Self {
            data,
            nodes,
            relations,
            backbones,
            cells,
            by_id,
            by_ref,
            edges,
            outgoing,
            dna,
            phase,
            rna,
        })
    }
    pub fn revision(&self) -> &str {
        self.data["catalogue_revision"].as_str().unwrap()
    }
    pub fn source_revision(&self) -> &str {
        self.data["source_revision"].as_str().unwrap()
    }
    pub fn nodes(&self) -> &[SourceNode] {
        &self.nodes
    }
    pub fn relations(&self) -> &[SourceRelation] {
        &self.relations
    }
    pub fn matrix_cells(&self) -> &[MatrixCell] {
        &self.cells
    }
    pub fn backbones(&self) -> &[BackboneProjection] {
        &self.backbones
    }
    pub fn node(&self, id: MTreeId) -> Option<&SourceNode> {
        self.by_id.get(&id).map(|i| &self.nodes[*i])
    }
    pub fn resolve(&self, reference: &str) -> Option<&SourceNode> {
        self.by_ref.get(reference).map(|i| &self.nodes[*i])
    }
    pub fn relation(&self, id: MTreeId) -> Option<&SourceRelation> {
        self.edges.get(&id).map(|i| &self.relations[*i])
    }
    pub fn outgoing(&self, id: MTreeId) -> impl Iterator<Item = &SourceRelation> {
        self.outgoing
            .get(&id)
            .into_iter()
            .flatten()
            .map(|i| &self.relations[*i])
    }
    pub fn backbone(&self, id: MTreeId) -> Option<&BackboneProjection> {
        self.backbones.iter().find(|b| b.id == id)
    }
    pub fn genetic(&self, codon: Codon64) -> GeneticReading<'_> {
        let index = codon.address() as usize;
        let phase = &self.nodes[self.phase[index]];
        GeneticReading {
            dna: &self.nodes[self.dna[index]],
            phase,
            rna: self.rna[index].map(|i| &self.nodes[i]),
            translations: self
                .outgoing(phase.id)
                .filter(|e| {
                    matches!(
                        e.kind.as_str(),
                        "TRANSLATES_TO" | "CONDITIONALLY_TRANSLATES_TO"
                    )
                })
                .collect(),
            expression: self
                .outgoing(phase.id)
                .filter(|e| {
                    matches!(
                        e.kind.as_str(),
                        "GOVERNS_TAROT_EXPRESSION" | "PAIRED_AS_COURT" | "POTENTIATES"
                    )
                })
                .collect(),
            standing: "source-recorded-translation-not-retained-amino-array-equivalence",
        }
    }
    pub fn discrepancies(&self) -> &[Value] {
        self.data["discrepancies"].as_array().unwrap()
    }
}
pub fn native_m3_source() -> &'static M3Source {
    static SOURCE: OnceLock<M3Source> = OnceLock::new();
    SOURCE.get_or_init(|| M3Source::load().expect("compiled source-checked M3 catalogue"))
}
