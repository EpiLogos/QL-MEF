use std::collections::{BTreeMap, BTreeSet};

use ql_core::QlFace;

/// Bimba and Pratibimba are the source image and its exact conjugate reflection.
/// This is deliberately a face of one coordinate identity, not two independently
/// authored ontologies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MFace {
    Bimba,
    Pratibimba,
}

impl MFace {
    pub const fn reflected(self) -> Self {
        match self {
            Self::Bimba => Self::Pratibimba,
            Self::Pratibimba => Self::Bimba,
        }
    }

    pub const fn ql_face(self) -> QlFace {
        match self {
            Self::Bimba => QlFace::Direct,
            Self::Pratibimba => QlFace::Conjugate,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bimba => "bimba",
            Self::Pratibimba => "pratibimba",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MPathSeparator {
    Dot,
    Hyphen,
    Slash,
}

impl MPathSeparator {
    pub const fn as_char(self) -> char {
        match self {
            Self::Dot => '.',
            Self::Hyphen => '-',
            Self::Slash => '/',
        }
    }
}

/// Provenance of one source record. Multiple source records may describe the
/// same source coordinate (for example low-detail and deep exports); they remain
/// separately traceable rather than being silently collapsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRecordRef {
    pub repository: String,
    pub revision: String,
    pub source_path: String,
    pub git_blob: String,
    pub file_sha256: String,
    pub record_class: String,
    pub record_index: usize,
    pub payload_sha256: String,
}

/// Source-owned semantic/structural payload is referred to by its exact source
/// record and digest. QL-MEF can carry and index this payload without promoting
/// its contents into universal QL canon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePayload {
    pub record: SourceRecordRef,
    pub property_keys: Vec<String>,
}

/// A source-preserving M coordinate. `source_ref` and separator sequence are
/// retained exactly because the historical Bimba corpus uses dot, hyphen, and
/// slash notation and those spellings must not be silently rewritten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MCoordinate {
    pub source_ref: String,
    pub root: u8,
    pub path: Vec<u16>,
    pub separators: Vec<MPathSeparator>,
    pub face: MFace,
    pub parent_source_ref: Option<String>,
    pub aliases: Vec<String>,
    pub provenance: Vec<SourceRecordRef>,
    pub payloads: Vec<SourcePayload>,
}

impl MCoordinate {
    pub fn parse_source(source_ref: impl Into<String>, face: MFace) -> Result<Self, String> {
        let source_ref = source_ref.into();
        let parsed = ParsedSourceCoordinate::parse(&source_ref)?;
        Ok(Self {
            parent_source_ref: parsed.parent_source_ref(),
            source_ref,
            root: parsed.root,
            path: parsed.path,
            separators: parsed.separators,
            face,
            aliases: Vec::new(),
            provenance: Vec::new(),
            payloads: Vec::new(),
        })
    }

    pub fn reflected(&self) -> Self {
        let mut reflected = self.clone();
        reflected.face = self.face.reflected();
        reflected
    }

    /// A QL-MEF-native ref preserves source notation and changes only face.
    pub fn canonical_ref(&self) -> String {
        format!(
            "ql:m-coordinate:{}:{}",
            self.face.as_str(),
            self.source_ref.replacen('#', "M", 1)
        )
    }

    pub fn depth(&self) -> usize {
        self.path.len()
    }

    pub fn same_structural_path(&self, other: &Self) -> bool {
        self.root == other.root
            && self.path == other.path
            && self.separators == other.separators
            && self.source_ref == other.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSourceCoordinate {
    root: u8,
    path: Vec<u16>,
    separators: Vec<MPathSeparator>,
    source_ref: String,
}

impl ParsedSourceCoordinate {
    fn parse(source_ref: &str) -> Result<Self, String> {
        let bytes = source_ref.as_bytes();
        if bytes.len() < 2 || bytes[0] != b'#' || !bytes[1].is_ascii_digit() {
            return Err(format!("unsupported M source coordinate `{source_ref}`"));
        }
        let root = bytes[1] - b'0';
        if root > 5 {
            return Err(format!("M root must be 0..5 in `{source_ref}`"));
        }

        let mut cursor = 2;
        let mut path = Vec::new();
        let mut separators = Vec::new();
        while cursor < bytes.len() {
            let separator = match bytes[cursor] {
                b'.' => MPathSeparator::Dot,
                b'-' => MPathSeparator::Hyphen,
                b'/' => MPathSeparator::Slash,
                _ => return Err(format!("unsupported M path syntax in `{source_ref}`")),
            };
            cursor += 1;
            let start = cursor;
            while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
                cursor += 1;
            }
            if start == cursor {
                return Err(format!("missing M path segment in `{source_ref}`"));
            }
            let segment = source_ref[start..cursor]
                .parse::<u16>()
                .map_err(|_| format!("invalid M path segment in `{source_ref}`"))?;
            path.push(segment);
            separators.push(separator);
        }

        Ok(Self {
            root,
            path,
            separators,
            source_ref: source_ref.to_owned(),
        })
    }

    fn parent_source_ref(&self) -> Option<String> {
        if self.path.is_empty() {
            return Some("#".to_owned());
        }
        let mut result = format!("#{}", self.root);
        for (index, segment) in self.path[..self.path.len() - 1].iter().enumerate() {
            result.push(self.separators[index].as_char());
            result.push_str(&segment.to_string());
        }
        Some(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MRelationClass {
    /// Directly present in Idea/Bimba/Map.
    BimbaSource,
    /// Derived from accepted QL law rather than asserted by the Map.
    QlDerived,
    /// Exact M -> M' reflection relation.
    Reflection,
    /// Operational dependency/binding in current software.
    Implementation,
    /// Runtime/event relation.
    Runtime,
    /// Research proposition which has not been promoted.
    ResearchCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationOrientation {
    Directed,
    Undirected,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MRelationEndpoint {
    Coordinate(String),
    ExternalSourceRef(String),
    Missing,
}

impl MRelationEndpoint {
    pub fn source_ref(&self) -> Option<&str> {
        match self {
            Self::Coordinate(value) | Self::ExternalSourceRef(value) => Some(value),
            Self::Missing => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MRelation {
    pub relation_ref: String,
    pub class: MRelationClass,
    pub source_kind: String,
    pub from: MRelationEndpoint,
    pub to: MRelationEndpoint,
    pub orientation: RelationOrientation,
    pub cross_m: bool,
    pub provenance: SourceRecordRef,
    pub payload: SourcePayload,
}

/// Structural existence is intentionally separate from every implementation
/// concern. A coordinate can exist with no binding; a binding cannot make a
/// source coordinate exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplementationBinding {
    pub coordinate_ref: String,
    pub implementation_owner: String,
    pub provider: Option<String>,
    pub capability_state: String,
    pub readiness_state: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectionProof {
    pub source_ref: String,
    pub bimba_ref: String,
    pub pratibimba_ref: String,
    pub root: u8,
    pub path: Vec<u16>,
    pub separators: Vec<MPathSeparator>,
    pub parent_source_ref: Option<String>,
}

/// Queryable whole-M structural field. It indexes source identities and source
/// relations while keeping implementation bindings in a separate collection.
#[derive(Debug, Default, Clone)]
pub struct MMapIndex {
    coordinates: BTreeMap<String, MCoordinate>,
    aliases: BTreeMap<String, String>,
    relations: Vec<MRelation>,
    bindings: Vec<ImplementationBinding>,
}

impl MMapIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_coordinate(&mut self, coordinate: MCoordinate) -> Result<(), String> {
        if coordinate.face != MFace::Bimba {
            return Err(
                "MMapIndex stores source/Bimba identities; Pratibimba is reflected on demand"
                    .to_owned(),
            );
        }
        if let Some(existing) = self.coordinates.get_mut(&coordinate.source_ref) {
            if existing.root != coordinate.root
                || existing.path != coordinate.path
                || existing.separators != coordinate.separators
            {
                return Err(format!(
                    "source coordinate `{}` resolves to incompatible structural paths",
                    coordinate.source_ref
                ));
            }
            for alias in coordinate.aliases {
                if !existing.aliases.contains(&alias) {
                    existing.aliases.push(alias.clone());
                    self.aliases.insert(alias, existing.source_ref.clone());
                }
            }
            existing.provenance.extend(coordinate.provenance);
            existing.payloads.extend(coordinate.payloads);
            return Ok(());
        }

        for alias in &coordinate.aliases {
            if let Some(previous) = self
                .aliases
                .insert(alias.clone(), coordinate.source_ref.clone())
            {
                if previous != coordinate.source_ref {
                    return Err(format!(
                        "alias `{alias}` resolves to both `{previous}` and `{}`",
                        coordinate.source_ref
                    ));
                }
            }
        }
        self.coordinates
            .insert(coordinate.source_ref.clone(), coordinate);
        Ok(())
    }

    pub fn insert_relation(&mut self, relation: MRelation) {
        self.relations.push(relation);
    }

    pub fn add_binding(&mut self, binding: ImplementationBinding) {
        self.bindings.push(binding);
    }

    pub fn resolve(&self, source_or_alias: &str, face: MFace) -> Option<MCoordinate> {
        let source_ref = self
            .aliases
            .get(source_or_alias)
            .map(String::as_str)
            .unwrap_or(source_or_alias);
        self.coordinates.get(source_ref).map(|coordinate| {
            if face == MFace::Bimba {
                coordinate.clone()
            } else {
                coordinate.reflected()
            }
        })
    }

    pub fn source_coordinate_count(&self) -> usize {
        self.coordinates.len()
    }

    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    pub fn relations(&self) -> &[MRelation] {
        &self.relations
    }

    pub fn bindings(&self) -> &[ImplementationBinding] {
        &self.bindings
    }

    pub fn roots(&self) -> BTreeSet<u8> {
        self.coordinates
            .values()
            .map(|coordinate| coordinate.root)
            .collect()
    }

    pub fn coordinates_in_root(&self, root: u8) -> Vec<&MCoordinate> {
        self.coordinates
            .values()
            .filter(|coordinate| coordinate.root == root)
            .collect()
    }

    pub fn relations_for(&self, source_ref: &str) -> Vec<&MRelation> {
        self.relations
            .iter()
            .filter(|relation| {
                relation.from.source_ref() == Some(source_ref)
                    || relation.to.source_ref() == Some(source_ref)
            })
            .collect()
    }

    pub fn implementation_for(&self, coordinate_ref: &str) -> Vec<&ImplementationBinding> {
        self.bindings
            .iter()
            .filter(|binding| binding.coordinate_ref == coordinate_ref)
            .collect()
    }

    pub fn prove_exact_reflection(&self, source_ref: &str) -> Result<ReflectionProof, String> {
        let bimba = self
            .resolve(source_ref, MFace::Bimba)
            .ok_or_else(|| format!("unknown M source coordinate `{source_ref}`"))?;
        let pratibimba = bimba.reflected();
        if !bimba.same_structural_path(&pratibimba) {
            return Err(format!(
                "reflection changed structural path for `{source_ref}`"
            ));
        }
        Ok(ReflectionProof {
            source_ref: bimba.source_ref.clone(),
            bimba_ref: bimba.canonical_ref(),
            pratibimba_ref: pratibimba.canonical_ref(),
            root: bimba.root,
            path: bimba.path.clone(),
            separators: bimba.separators.clone(),
            parent_source_ref: bimba.parent_source_ref.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(path: &str, index: usize) -> SourceRecordRef {
        SourceRecordRef {
            repository: "EpiLogos/Epi-Logos-C-Experiments".to_owned(),
            revision: "source-revision".to_owned(),
            source_path: path.to_owned(),
            git_blob: "blob".to_owned(),
            file_sha256: "file-sha".to_owned(),
            record_class: "source-node".to_owned(),
            record_index: index,
            payload_sha256: format!("payload-{index}"),
        }
    }

    #[test]
    fn mixed_source_notation_is_preserved_coordinate_for_coordinate() {
        let coordinate = MCoordinate::parse_source("#0-4.0/1/2-3", MFace::Bimba).unwrap();
        assert_eq!(coordinate.root, 0);
        assert_eq!(coordinate.path, vec![4, 0, 1, 2, 3]);
        assert_eq!(
            coordinate.separators,
            vec![
                MPathSeparator::Hyphen,
                MPathSeparator::Dot,
                MPathSeparator::Slash,
                MPathSeparator::Slash,
                MPathSeparator::Hyphen
            ]
        );
        assert_eq!(
            coordinate.parent_source_ref.as_deref(),
            Some("#0-4.0/1/2")
        );
        assert_eq!(
            coordinate.canonical_ref(),
            "ql:m-coordinate:bimba:M0-4.0/1/2-3"
        );
    }

    #[test]
    fn reflection_changes_only_face() {
        let coordinate = MCoordinate::parse_source("#2-2-2-4-4", MFace::Bimba).unwrap();
        let reflected = coordinate.reflected();
        assert!(coordinate.same_structural_path(&reflected));
        assert_eq!(coordinate.ql_face(), QlFace::Direct);
        assert_eq!(reflected.face.ql_face(), QlFace::Conjugate);
        assert_eq!(
            reflected.canonical_ref(),
            "ql:m-coordinate:pratibimba:M2-2-2-4-4"
        );
    }

    #[test]
    fn structural_existence_does_not_imply_implementation() {
        let mut index = MMapIndex::new();
        let mut coordinate = MCoordinate::parse_source("#1-2-3", MFace::Bimba).unwrap();
        coordinate.provenance.push(record("nodes.json", 0));
        index.insert_coordinate(coordinate).unwrap();
        assert!(index.resolve("#1-2-3", MFace::Pratibimba).is_some());
        assert!(
            index
                .implementation_for("ql:m-coordinate:pratibimba:M1-2-3")
                .is_empty()
        );
    }

    #[test]
    fn source_relation_kind_direction_and_missing_endpoint_survive() {
        let source_record = record("relations.json", 4);
        let payload = SourcePayload {
            record: source_record.clone(),
            property_keys: vec!["description".to_owned()],
        };
        let relation = MRelation {
            relation_ref: "source-relation:4".to_owned(),
            class: MRelationClass::BimbaSource,
            source_kind: "HAS_QL_FRAME".to_owned(),
            from: MRelationEndpoint::Coordinate("#4".to_owned()),
            to: MRelationEndpoint::Missing,
            orientation: RelationOrientation::Directed,
            cross_m: false,
            provenance: source_record,
            payload,
        };
        let mut index = MMapIndex::new();
        index.insert_relation(relation);
        assert_eq!(index.relations()[0].source_kind, "HAS_QL_FRAME");
        assert_eq!(
            index.relations()[0].orientation,
            RelationOrientation::Directed
        );
        assert_eq!(index.relations()[0].to, MRelationEndpoint::Missing);
    }
}
