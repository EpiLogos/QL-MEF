//! Coordinate-bound M3 engine view over the existing core and K2 registry.
//! No second tree and no automatic equivalence between unrelated source orders.
//! Bimba's exact relations are retained; graph-dependent symbolic clock columns
//! remain unreconciled source records rather than promoted canon.
use crate::m_tree::{MRegistry, MTreeId, MTreeNode, MTreeRelation, native_m_registry};
use ql_core::m3_clock::M3Clock;
use ql_core::{Codon64, Nucleotide, TarotBridge, Trigram};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

pub const M3_ENGINE_VERSION: &str = "ql.m3-engine/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum M3NodeKind {
    Root,
    Nucleotide,
    Pair,
    Codon,
    Trigram,
    Hexagram,
    Matrix,
    Minor,
    Major,
    Degree,
    Backbone,
    Phase,
}
impl M3NodeKind {
    pub const ALL: [Self; 12] = [Self::Root, Self::Nucleotide, Self::Pair, Self::Codon,
        Self::Trigram, Self::Hexagram, Self::Matrix, Self::Minor, Self::Major,
        Self::Degree, Self::Backbone, Self::Phase];
    pub const fn count(self) -> usize {
        [1, 4, 16, 64, 8, 64, 3, 56, 22, 360, 24, 1][self as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct M3ClockFrame<'a> {
    pub clock: M3Clock,
    pub degree: &'a MTreeNode,
    pub backbone: &'a MTreeNode,
    pub clockwise: &'a MTreeNode,
    pub polar: &'a MTreeNode,
}

pub struct M3Engine<'a> {
    registry: &'a MRegistry,
    groups: Vec<Vec<MTreeId>>,
    anchors: Vec<MTreeId>,
}

impl<'a> M3Engine<'a> {
    pub fn new(registry: &'a MRegistry) -> Result<Self, String> {
        let lookup = |source: String| registry.resolve(&source).map(|n| n.id)
            .ok_or_else(|| format!("missing M3 coordinate: {source}"));
        let mut groups = Vec::new();
        for kind in M3NodeKind::ALL {
            let mut ids = Vec::new();
            for index in 0..kind.count() {
                let reference = match kind {
                    M3NodeKind::Root => "#3".into(),
                    M3NodeKind::Nucleotide => format!("#3-2-{}", index + 1),
                    M3NodeKind::Pair => format!("#3-2-{}-{}", index / 4 + 1, index % 4 + 1),
                    M3NodeKind::Codon => format!("#3-2-{}-{}-{}", index / 16 + 1, (index / 4) % 4 + 1, index % 4 + 1),
                    M3NodeKind::Trigram => format!("#3-1-{index}"),
                    M3NodeKind::Hexagram => {
                        let upper = Trigram::LUT.iter().find(|t| t.binary == (index >> 3) as u8).unwrap().id;
                        let lower = Trigram::LUT.iter().find(|t| t.binary == (index & 7) as u8).unwrap().id;
                        format!("#3-1-{upper}-{lower}")
                    }
                    M3NodeKind::Matrix => format!("#3-3-2-{index}"),
                    M3NodeKind::Minor => {
                        let card = &TarotBridge::kernel().minor()[index];
                        let path = match card.suit().nucleotide() {
                            Nucleotide::A => 2, Nucleotide::T => 1,
                            Nucleotide::C => 4, Nucleotide::G => 3,
                        };
                        format!("#3-4-{path}-{}", card.pip().value())
                    }
                    M3NodeKind::Major => format!("#3-4-5/0-{index}"),
                    M3NodeKind::Degree if index == 0 => "#3-5-5/0-0/360".into(),
                    M3NodeKind::Degree => format!("#3-5-5/0-{index}"),
                    M3NodeKind::Backbone => format!("#3-5-{}-{}", index / 6 + 1, index % 6),
                    M3NodeKind::Phase => "#3-4.0".into(),
                };
                ids.push(lookup(reference)?);
            }
            groups.push(ids);
        }
        let mut edges: BTreeMap<(&str, MTreeId), BTreeSet<MTreeId>> = BTreeMap::new();
        for relation in &registry.manifest().relations {
            if let (Some(from), Some(to)) = (relation.from_id, relation.to_id) {
                edges.entry((&relation.source_kind, from)).or_default().insert(to);
            }
        }
        let degrees = &groups[M3NodeKind::Degree as usize];
        let backbone: BTreeSet<_> = groups[M3NodeKind::Backbone as usize].iter().copied().collect();
        let mut anchors = Vec::new();
        for (degree, id) in degrees.iter().enumerate() {
            let targets = edges.get(&("ANCHORED_BY", *id)).ok_or("unbound clock degree")?;
            if targets.len() != 1 || !targets.is_subset(&backbone) {
                return Err("ambiguous clock backbone".into());
            }
            let anchor = *targets.first().unwrap();
            if !edges.get(&("GOVERNS_DEGREE_ARC", anchor)).is_some_and(|targets| targets.contains(id)) {
                return Err("missing reciprocal clock backbone assertion".into());
            }
            for (kind, offset) in [("FLOWS_CLOCKWISE", 1), ("POLAR_OPPOSITE", 180)] {
                if edges.get(&(kind, *id)) != Some(&BTreeSet::from([degrees[(degree + offset) % 360]])) {
                    return Err(format!("clock source/arithmetic discrepancy: {kind}, {degree}"));
                }
            }
            anchors.push(anchor);
        }
        if backbone.iter().any(|id| anchors.iter().filter(|a| *a == id).count() != 15) {
            return Err("backbone degree partition is not 24 x 15".into());
        }
        Ok(Self { registry, groups, anchors })
    }
    pub fn registry(&self) -> &'a MRegistry { self.registry }
    pub fn node(&self, kind: M3NodeKind, ordinal: usize) -> Option<&'a MTreeNode> {
        self.groups[kind as usize].get(ordinal).and_then(|id| self.registry.node(*id))
    }
    pub fn coordinates(&self) -> impl Iterator<Item = &'a MTreeNode> {
        self.registry.manifest().nodes.iter().filter(|n| n.root_position == Some(3))
    }
    pub fn source_relations(&self, id: MTreeId) -> impl Iterator<Item = &'a MTreeRelation> {
        self.registry.manifest().relations.iter().filter(move |r| r.from_id == Some(id) || r.to_id == Some(id))
    }
    pub fn clock(&self, clock: M3Clock) -> M3ClockFrame<'a> {
        let d = clock.degree360() as usize;
        M3ClockFrame { clock, degree: self.node(M3NodeKind::Degree, d).unwrap(),
            backbone: self.registry.node(self.anchors[d]).unwrap(),
            clockwise: self.node(M3NodeKind::Degree, (d+1)%360).unwrap(),
            polar: self.node(M3NodeKind::Degree, (d+180)%360).unwrap() }
    }
    /// Alphabetic DNA/RNA transcription of the same form, not a polarity flip.
    pub fn transcribe(codon: Codon64, rna: bool) -> [u8; 3] {
        let alphabet = if rna { b"AUCG" } else { b"ATCG" };
        codon.nucleotides().map(|n| alphabet[n.bits() as usize])
    }
}

pub fn native_m3_engine() -> &'static M3Engine<'static> {
    static ENGINE: OnceLock<M3Engine<'static>> = OnceLock::new();
    ENGINE.get_or_init(|| M3Engine::new(native_m_registry()).expect("accepted M3 source relation field"))
}
