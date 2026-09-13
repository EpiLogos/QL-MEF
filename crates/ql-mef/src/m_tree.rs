//! Consumer of the generated native-C M registry, not another authored M tree.
//! Source existence, tree containment, source relations and implementation
//! bindings remain separate. The legacy MCoordinate projection retains payload
//! provenance and face reflection, while M supplies the first-class master index.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::{
    ImplementationBinding, MCoordinate, MFace, MMapIndex, MRelation, MRelationClass,
    MRelationEndpoint, RelationOrientation, SourcePayload, SourceRecordRef,
};

pub const M_TREE_SCHEMA: &str = "ql.m-tree/v1";
/// Accepted v1 source view, retained with its exact historical proof.
pub const NATIVE_M_MANIFEST: &str = include_str!("../../../fixtures/kernel/m-tree-v1.json");

pub const CURRENT_M_MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/m-tree-v2.json"));
pub const CURRENT_M_RECEIPT: &str =
    include_str!(concat!(env!("OUT_DIR"), "/k8-structure-receipt-v1.json"));

/// Spelling-derived ID, shared verbatim with uint64_t in the native ABI.
/// JSON deliberately uses sixteen hexadecimal digits (not lossy JS numbers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MTreeId(u64);

impl MTreeId {
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        if value.len() != 16
            || !value
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(format!("invalid M registry ID: {value}"));
        }
        let id = u64::from_str_radix(value, 16).map_err(|e| e.to_string())?;
        if id == 0 {
            return Err("zero is the native absent-ID sentinel".into());
        }
        Ok(Self(id))
    }
}

impl Serialize for MTreeId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{:016x}", self.0))
    }
}

impl<'de> Deserialize<'de> for MTreeId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MTreeNode {
    pub id: MTreeId,
    pub source_ref: String,
    pub parent_id: Option<MTreeId>,
    pub root_id: MTreeId,
    pub root_position: Option<u8>,
    pub local_segment: String,
    pub separator: String,
    pub depth: usize,
    pub lexical_depth: usize,
    pub lexical_parent_source_ref: Option<String>,
    pub parent_basis: String,
    pub source_parent_refs: Vec<String>,
    pub structural_status: String,
    pub aggregate: bool,
    pub names: Vec<String>,
    pub aliases: Vec<String>,
    pub records: Vec<usize>,
    pub children: Vec<MTreeId>,
    pub subtree_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MTreeSourceFile {
    pub path: String,
    pub git_blob: String,
    pub sha256: String,
    pub record_class: String,
    pub bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MTreeSourceRecord {
    pub file: usize,
    pub record_index: usize,
    pub payload_sha256: String,
    pub property_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MTreeRelation {
    pub id: MTreeId,
    pub relation_ref: String,
    pub class: String,
    pub source_kind: String,
    pub from_ref: Option<String>,
    pub to_ref: Option<String>,
    pub from_id: Option<MTreeId>,
    pub to_id: Option<MTreeId>,
    pub orientation: String,
    pub cross_m: bool,
    pub record: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MTreeBinding {
    pub identity: String,
    pub module: String,
    pub disposition: String,
    pub coordinate_id: Option<MTreeId>,
    pub relation_id: Option<MTreeId>,
    pub readiness: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MTreeManifest {
    pub schema: String,
    pub registry_revision: String,
    pub id_scheme: String,
    pub source_repository: String,
    pub source_revision: String,
    pub source_dataset_tree: String,
    pub source_snapshot_sha256: String,
    pub source_lock_sha256: String,
    pub compiler_sha256: BTreeMap<String, String>,
    pub master_id: MTreeId,
    pub roots: Vec<MTreeId>,
    pub files: Vec<MTreeSourceFile>,
    pub records: Vec<MTreeSourceRecord>,
    pub nodes: Vec<MTreeNode>,
    pub relations: Vec<MTreeRelation>,
    pub bindings: Vec<MTreeBinding>,
    pub alternate_notation_groups: Vec<serde_json::Value>,
    pub meta_source_records: Vec<serde_json::Value>,
    pub parent_discrepancies: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub registry_lineage: Vec<MRegistryLineage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MRegistryLineage {
    pub registry_revision: String,
    pub promotion_id: String,
    pub promotion_sha256: String,
    pub source_repository: String,
    pub source_revision: String,
    pub source_path: String,
    pub source_git_blob: String,
    pub standing: String,
}

/// An immutable index over the common manifest. This constructor validates
/// structure; the content digest is verified by generation/parity acceptance,
/// not advertised as a cryptographic authenticity check for arbitrary input.
#[derive(Debug)]
pub struct MRegistry {
    manifest: MTreeManifest,
    by_ref: BTreeMap<String, usize>,
    by_id: BTreeMap<MTreeId, usize>,
    relations_by_id: BTreeMap<MTreeId, usize>,
}

impl MRegistry {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let manifest: MTreeManifest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if !matches!(manifest.schema.as_str(), M_TREE_SCHEMA | "ql.m-tree/v2")
            || manifest.roots.len() != 6
        {
            return Err("unsupported M registry schema or root count".into());
        }
        if (manifest.schema == M_TREE_SCHEMA && !manifest.registry_lineage.is_empty())
            || (manifest.schema == "ql.m-tree/v2" && manifest.registry_lineage.is_empty())
        {
            return Err("registry schema/lineage mismatch".into());
        }
        let hex = |s: &str, len| {
            s.len() == len
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        };
        for origin in &manifest.registry_lineage {
            if !hex(&origin.registry_revision, 64)
                || !hex(&origin.promotion_sha256, 64)
                || !hex(&origin.source_revision, 40)
                || !hex(&origin.source_git_blob, 40)
                || origin.promotion_id.is_empty()
                || origin.source_repository.is_empty()
                || origin.source_path.is_empty()
                || origin.standing != "owner-ratified-structural-promotion"
            {
                return Err("invalid structural promotion lineage".into());
            }
        }
        for file in &manifest.files {
            match (&file.repository, &file.revision) {
                (None, None) => (),
                (Some(repository), Some(revision))
                    if !repository.is_empty() && hex(revision, 40) => {}
                _ => return Err("incomplete source-file origin".into()),
            }
        }
        let mut registry = Self {
            manifest,
            by_ref: BTreeMap::new(),
            by_id: BTreeMap::new(),
            relations_by_id: BTreeMap::new(),
        };
        for (i, node) in registry.manifest.nodes.iter().enumerate() {
            if registry.by_ref.insert(node.source_ref.clone(), i).is_some()
                || registry.by_id.insert(node.id, i).is_some()
            {
                return Err("duplicate M node spelling or identity".into());
            }
        }
        let master = registry
            .node(registry.manifest.master_id)
            .ok_or("missing M master")?;
        if master.source_ref != "M"
            || master.parent_id.is_some()
            || master.depth != 0
            || !master.aggregate
            || master.root_id != master.id
            || master.root_position.is_some()
            || master.structural_status != "master-index"
            || master.children != registry.manifest.roots
        {
            return Err("invalid M master".into());
        }
        for (position, id) in registry.manifest.roots.iter().enumerate() {
            let root = registry.node(*id).ok_or("missing M root")?;
            if root.source_ref != format!("#{position}")
                || root.root_id != root.id
                || root.root_position != Some(position as u8)
                || !root.aggregate
                || root.parent_id != Some(master.id)
            {
                return Err("invalid M aggregate root".into());
            }
        }
        let mut reached = BTreeSet::new();
        let mut pending = vec![master.id];
        while let Some(id) = pending.pop() {
            if !reached.insert(id) {
                return Err("cycle or duplicate M tree child".into());
            }
            let node = registry.node(id).ok_or("missing tree child")?;
            let mut subtree_count: usize = 1;
            for child in &node.children {
                let c = registry.node(*child).ok_or("unresolved tree child")?;
                if c.parent_id != Some(id) || Some(c.depth) != node.depth.checked_add(1) {
                    return Err("M parent/child/depth disagreement".into());
                }
                subtree_count = subtree_count
                    .checked_add(c.subtree_count)
                    .ok_or("M subtree count overflow")?;
                pending.push(*child);
            }
            if node.subtree_count != subtree_count {
                return Err("incorrect M subtree count".into());
            }
            if node.id != master.id {
                let coordinate = MCoordinate::parse_source(&node.source_ref, MFace::Bimba)?;
                if node.structural_status != "source-declared"
                    || node.records.is_empty()
                    || node.root_position != Some(coordinate.root)
                    || node.root_id != registry.manifest.roots[usize::from(coordinate.root)]
                    || node.lexical_depth != coordinate.depth()
                {
                    return Err("invalid source coordinate standing/root/provenance".into());
                }
                if coordinate.depth() > 0 {
                    let parent = registry
                        .node(node.parent_id.ok_or("missing parent")?)
                        .ok_or("unknown parent")?;
                    if node.source_ref
                        != format!(
                            "{}{}{}",
                            parent.source_ref, node.separator, node.local_segment
                        )
                        || !matches!(node.separator.as_str(), "-" | "." | "/")
                        || !matches!(
                            node.parent_basis.as_str(),
                            "source-parent" | "existing-source-prefix"
                        )
                    {
                        return Err("M local segment does not round-trip".into());
                    }
                }
            }
            if node
                .records
                .iter()
                .any(|i| *i >= registry.manifest.records.len())
            {
                return Err("invalid M source record index".into());
            }
        }
        if reached.len() != registry.manifest.nodes.len() {
            return Err("disconnected M registry nodes".into());
        }
        for record in &registry.manifest.records {
            if record.file >= registry.manifest.files.len() {
                return Err("invalid M source file index".into());
            }
        }
        let mut relation_refs = BTreeSet::new();
        for (i, relation) in registry.manifest.relations.iter().enumerate() {
            if registry.relations_by_id.insert(relation.id, i).is_some()
                || !relation_refs.insert(&relation.relation_ref)
                || relation.record >= registry.manifest.records.len()
                || relation.class != "bimba-source"
                || !matches!(
                    relation.orientation.as_str(),
                    "directed" | "undirected" | "unspecified"
                )
            {
                return Err("invalid or duplicate M source relation".into());
            }
            for (id, reference) in [
                (relation.from_id, &relation.from_ref),
                (relation.to_id, &relation.to_ref),
            ] {
                if let Some(id) = id {
                    let node = registry.node(id).ok_or("unknown M relation endpoint")?;
                    if reference.as_deref() != Some(node.source_ref.as_str()) {
                        return Err("M relation endpoint spelling disagrees with ID".into());
                    }
                } else if reference
                    .as_deref()
                    .and_then(|s| registry.resolve(s))
                    .is_some()
                {
                    return Err("known M endpoint incorrectly marked external".into());
                }
            }
        }
        for binding in &registry.manifest.bindings {
            registry.validate_binding(binding)?;
        }
        Ok(registry)
    }

    pub fn manifest(&self) -> &MTreeManifest {
        &self.manifest
    }
    pub fn node(&self, id: MTreeId) -> Option<&MTreeNode> {
        self.by_id.get(&id).map(|i| &self.manifest.nodes[*i])
    }
    pub fn resolve(&self, reference: &str) -> Option<&MTreeNode> {
        let source = if reference != "M" && reference.starts_with('M') {
            format!("#{}", &reference[1..])
        } else {
            reference.to_owned()
        };
        self.by_ref.get(&source).map(|i| &self.manifest.nodes[*i])
    }
    pub fn master(&self) -> &MTreeNode {
        &self.manifest.nodes[self.by_id[&self.manifest.master_id]]
    }
    pub fn root(&self, position: usize) -> Option<&MTreeNode> {
        self.manifest
            .roots
            .get(position)
            .and_then(|id| self.node(*id))
    }
    pub fn parent(&self, id: MTreeId) -> Option<&MTreeNode> {
        self.node(id)
            .and_then(|n| n.parent_id)
            .and_then(|p| self.node(p))
    }
    pub fn children(&self, id: MTreeId) -> impl Iterator<Item = &MTreeNode> {
        self.node(id)
            .into_iter()
            .flat_map(|n| &n.children)
            .filter_map(|id| self.node(*id))
    }
    pub fn relation(&self, id: MTreeId) -> Option<&MTreeRelation> {
        self.relations_by_id
            .get(&id)
            .map(|i| &self.manifest.relations[*i])
    }
    pub fn relations_for(&self, id: MTreeId) -> impl Iterator<Item = &MTreeRelation> {
        self.manifest
            .relations
            .iter()
            .filter(move |r| r.from_id == Some(id) || r.to_id == Some(id))
    }
    pub fn validate_binding(&self, binding: &MTreeBinding) -> Result<(), String> {
        let target_valid = match binding.disposition.as_str() {
            "COORDINATE-BOUND" => {
                binding.coordinate_id.and_then(|id| self.node(id)).is_some()
                    && binding.relation_id.is_none()
            }
            "CROSS-COORDINATE" => {
                binding
                    .relation_id
                    .and_then(|id| self.relation(id))
                    .is_some()
                    && binding.coordinate_id.is_none()
            }
            "INFRASTRUCTURAL" => binding.coordinate_id.is_none() && binding.relation_id.is_none(),
            _ => false,
        };
        let readiness_valid = match binding.readiness.as_str() {
            "unbound" => true,
            "structural-index-only" => !binding.evidence.is_empty(),
            _ => false,
        };
        if target_valid
            && readiness_valid
            && !binding.identity.is_empty()
            && !binding.module.is_empty()
        {
            Ok(())
        } else {
            Err("unresolved/invalid M binding; structural existence is unchanged".into())
        }
    }

    fn source_record(&self, index: usize) -> SourceRecordRef {
        let r = &self.manifest.records[index];
        let f = &self.manifest.files[r.file];
        SourceRecordRef {
            repository: f
                .repository
                .as_ref()
                .unwrap_or(&self.manifest.source_repository)
                .clone(),
            revision: f
                .revision
                .as_ref()
                .unwrap_or(&self.manifest.source_revision)
                .clone(),
            source_path: f.path.clone(),
            git_blob: f.git_blob.clone(),
            file_sha256: f.sha256.clone(),
            record_class: f.record_class.clone(),
            record_index: r.record_index,
            payload_sha256: r.payload_sha256.clone(),
        }
    }

    /// Legacy face-bearing value, now joined to real source parentage/provenance.
    /// Its root-parent '#' convention remains compatible; the M master is exposed
    /// by this registry, not falsely reinterpreted as the external '#' meta field.
    pub fn coordinate(&self, reference: &str, face: MFace) -> Result<MCoordinate, String> {
        let node = self.resolve(reference).ok_or("unknown M coordinate")?;
        let mut c = MCoordinate::parse_source(&node.source_ref, face)?;
        if let Some(parent) = self.parent(node.id).filter(|p| p.source_ref != "M") {
            c.parent_source_ref = Some(parent.source_ref.clone());
        }
        c.aliases = node.aliases.clone();
        for index in &node.records {
            let record = self.source_record(*index);
            c.provenance.push(record.clone());
            c.payloads.push(SourcePayload {
                record,
                property_keys: self.manifest.records[*index].property_keys.clone(),
            });
        }
        Ok(c)
    }

    pub fn to_m_map_index(&self) -> Result<MMapIndex, String> {
        let mut index = MMapIndex::new();
        for node in self.manifest.nodes.iter().filter(|n| n.source_ref != "M") {
            index.insert_coordinate(self.coordinate(&node.source_ref, MFace::Bimba)?)?;
        }
        let endpoint = |id: Option<MTreeId>, r: &Option<String>| match (id, r) {
            (Some(_), Some(r)) => MRelationEndpoint::Coordinate(r.clone()),
            (None, Some(r)) => MRelationEndpoint::ExternalSourceRef(r.clone()),
            _ => MRelationEndpoint::Missing,
        };
        for r in &self.manifest.relations {
            let provenance = self.source_record(r.record);
            index.insert_relation(MRelation {
                relation_ref: r.relation_ref.clone(),
                class: MRelationClass::BimbaSource,
                source_kind: r.source_kind.clone(),
                from: endpoint(r.from_id, &r.from_ref),
                to: endpoint(r.to_id, &r.to_ref),
                cross_m: r.cross_m,
                orientation: match r.orientation.as_str() {
                    "directed" => RelationOrientation::Directed,
                    "undirected" => RelationOrientation::Undirected,
                    _ => RelationOrientation::Unspecified,
                },
                payload: SourcePayload {
                    record: provenance.clone(),
                    property_keys: self.manifest.records[r.record].property_keys.clone(),
                },
                provenance,
            });
        }
        for b in &self.manifest.bindings {
            if let Some(node) = b
                .coordinate_id
                .and_then(|id| self.node(id))
                .filter(|n| n.source_ref != "M")
            {
                index.add_binding(ImplementationBinding {
                    coordinate_ref: node.source_ref.clone(),
                    implementation_owner: b.module.clone(),
                    provider: None,
                    capability_state: b.identity.clone(),
                    readiness_state: b.readiness.clone(),
                    evidence_refs: vec![b.evidence.clone()],
                });
            }
        }
        Ok(index)
    }
}

pub fn native_m_registry() -> &'static MRegistry {
    static REGISTRY: OnceLock<MRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        MRegistry::from_json(NATIVE_M_MANIFEST).expect("validated native M registry")
    })
}

/// Current source projection for K8 and new map/property consumers. Accepted
/// v1 M1–M3 producers deliberately retain `native_m_registry()` and their exact
/// source/registry receipts; a composed event carries both versioned bases.
pub fn native_current_m_registry() -> &'static MRegistry {
    static REGISTRY: OnceLock<MRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        MRegistry::from_json(CURRENT_M_MANIFEST).expect("validated current native M registry")
    })
}
