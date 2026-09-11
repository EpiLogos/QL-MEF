//! K5 M1 computation seated in the accepted M registry, not a second M tree.
//!
//! Reconciles the full retained C body with the later accepted Ananda return at
//! `bb47ab9730f0ddadd4891666fb6f3e0a6d457330`. Existing matheme, QL relation and
//! music operators remain the authority. This candidate API does not pronounce
//! the missing K4 census, all M1 capabilities, C++ or experiential parity ready.
use crate::m_tree::{MTreeId, MTreeNode, native_m_registry};
use crate::{
    HarmonicRatio, LensId, MusicalBasis, MusicalTraversalCandidate, TraversalExpansionSide,
    classify_musical_traversal, directed_pitch_delta, pitch_at_lens,
};
use ql_core::{ConjugationDegree, QlCoordinate, Quat, RING_QUATERNION_LUT};
use serde::Serialize;

pub const ENGINE_VERSION: &str = "0.1.0";
pub const RETURN_REVISION: &str = "bb47ab9730f0ddadd4891666fb6f3e0a6d457330";
pub const SOURCE_REF: &str = "Idea/Bimba/Map/datasets/(0_1) Vortex Modulae - (0_1) x 12Fold and 8_9fold (mod12 and mod10) Archetypal Number Identities - Sheet1.csv";
pub const VALID_FOLDS: [u32; 14] = [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 16, 18, 24];
pub const FLOWERING_REFS: [&str; 6] = [
    "#1-3-4.0000",
    "#1-3-4.0/1",
    "#1-3-4.0/1/2",
    "#1-3-4.0/1/2/3",
    "#1-3-4.4.0-4.4/5",
    "#1-3-4.5/0",
];
pub const DR_RING_MAHAMAYA: [u8; 6] = [1, 2, 4, 8, 7, 5];
pub const DR_RING_PARASHAKTI: [u8; 6] = [3, 6, 9, 3, 6, 9];

/// Resolve source spellings and registered M aliases without changing separators.
pub fn node(coordinate: &str) -> Option<&'static MTreeNode> {
    native_m_registry()
        .resolve(coordinate)
        .filter(|n| n.root_position == Some(1))
}
fn id(coordinate: &str) -> Result<MTreeId, String> {
    node(coordinate)
        .map(|n| n.id)
        .ok_or_else(|| format!("unregistered M1 coordinate: {coordinate}"))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Clock {
    /// JSON encodes this as a decimal string, preserving every u64 bit in clients.
    #[serde(serialize_with = "serialize_cycle")]
    pub cycle: u64,
    pub tick12: u32,
    pub position6: u32,
    pub phase: u32,
    pub conjugate_tick12: u32,
    pub conjugate_phase: u32,
    pub degree360: u32,
    pub hopf_fiber: u32,
    pub degree720: u32,
    pub spanda_stage: u32,
}
fn serialize_cycle<S: serde::Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}
impl Clock {
    pub fn new(cycle: u64, tick12: u32) -> Result<Self, String> {
        if tick12 >= 12 {
            return Err("tick12 must be 0..11; no implicit wrap".into());
        }
        let phase = tick12 / 6;
        let degree360 = tick12 * 30;
        let hopf_fiber = (cycle & 1) as u32;
        Ok(Self {
            cycle,
            tick12,
            position6: tick12 % 6,
            phase,
            conjugate_tick12: (tick12 + 6) % 12,
            conjugate_phase: 1 - phase,
            degree360,
            hopf_fiber,
            degree720: degree360 + 360 * hopf_fiber,
            spanda_stage: tick12 % 6,
        })
    }
    /// This generated clock orbit is distinct from the historical ring LUT.
    /// It changes sign after one base traversal and returns after two.
    pub fn spinor(&self) -> Quat {
        let half_angle = f64::from(self.degree720).to_radians() / 2.0;
        Quat {
            w: half_angle.cos() as f32,
            x: half_angle.sin() as f32,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Cell {
    pub coordinate: MTreeId,
    pub dr_coordinate: MTreeId,
    pub family: u32,
    pub row12: u32,
    pub col12: u32,
    /// Quintessence has no scalar. The tuple is {-1, raw_terms[0], raw_terms[2]}.
    pub scalar_valid: bool,
    /// Aperture validity applies to the contributor tuple too, not just a scalar.
    pub decimal10_valid: bool,
    pub raw: Option<i32>,
    pub digit_root: Option<u32>,
    pub decimal10: Option<u32>,
    /// Bimba, Pratibimba, Sum, Difference A, Difference B, in source order.
    pub raw_terms: [i32; 5],
    pub dr_terms: [u32; 5],
    pub decimal_terms: Option<[u32; 5]>,
    pub clock: Clock,
}
pub fn cell(family: u32, row12: u32, col12: u32, cycle: u64, tick12: u32) -> Result<Cell, String> {
    if family >= 6 || row12 >= 12 || col12 >= 12 {
        return Err("Ananda requires family 0..5 and a 12x12 cell".into());
    }
    let clock = Clock::new(cycle, tick12)?;
    let coordinate = id(&format!("#1-2-{family}"))?;
    let dr_coordinate = id(&format!("#1-2-{family}-0"))?;
    let b = (row12 * col12) as i32;
    let raw_terms = [b, b + 1, 2 * b + 1, -1, 1];
    let dr_terms = raw_terms.map(|n| {
        if n == -1 {
            9
        } else if n == 0 {
            0
        } else {
            1 + (n as u32 - 1) % 9
        }
    });
    let scalar_valid = family < 5;
    let decimal10_valid = row12 < 10 && col12 < 10;
    let decimal_terms = decimal10_valid.then(|| raw_terms.map(|n| n.rem_euclid(10) as u32));
    let f = family as usize;
    Ok(Cell {
        coordinate,
        dr_coordinate,
        family,
        row12,
        col12,
        scalar_valid,
        decimal10_valid,
        raw: scalar_valid.then(|| raw_terms[f]),
        digit_root: scalar_valid.then(|| dr_terms[f]),
        decimal10: if scalar_valid {
            decimal_terms.map(|terms| terms[f])
        } else {
            None
        },
        raw_terms,
        dr_terms,
        decimal_terms,
        clock,
    })
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Spanda {
    pub coordinate: MTreeId,
    pub substage_coordinate: Option<MTreeId>,
    pub stage: u32,
    pub substage: u32,
    pub fold_count: u32,
    pub dual_track: bool,
    pub weave_state: f32,
    pub inversion_state: u32,
}
pub fn spanda(stage: u32, substage: u32) -> Result<Spanda, String> {
    if stage >= 6 || substage >= 6 || (stage != 4 && substage != 0) {
        return Err("Spanda stage must be 0..5; only Flowering accepts a nonzero substage".into());
    }
    Ok(Spanda {
        coordinate: id(&format!("#1-3-{stage}"))?,
        substage_coordinate: if stage == 4 {
            Some(id(FLOWERING_REFS[substage as usize])?)
        } else {
            None
        },
        stage,
        substage,
        fold_count: if stage == 4 {
            [4, 6, 8, 10, 12, 0][substage as usize]
        } else {
            0
        },
        dual_track: stage == 4 && substage == 3,
        weave_state: [0.0, 1.0, 1.0, 1.5, 4.0, 5.0][stage as usize],
        inversion_state: [0, 0, 1, 0, 0, 1][stage as usize],
    })
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Formal {
    pub coordinate: MTreeId,
    pub stage: u32,
    pub next: u32,
    pub inverse: u32,
    pub numerator_position: u32,
    pub denominator_position: u32,
    pub signature: i32,
}
pub fn formal(stage: u32) -> Result<Formal, String> {
    if stage >= 6 {
        return Err("formal stage must be 0..5".into());
    }
    Ok(Formal {
        coordinate: id(&format!("#1-4.{stage}"))?,
        stage,
        next: (stage + 1) % 6,
        inverse: 5 - stage,
        numerator_position: [0, 0, 6, 5, 6, 5][stage as usize],
        denominator_position: [6, 5, 5, 0, 0, 6][stage as usize],
        signature: if stage == 0 || stage == 5 { -1 } else { 1 },
    })
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Topology {
    pub coordinate: MTreeId,
    pub tick12: u32,
    pub element_count: u32,
    pub legacy_return_stage: u32,
    pub legacy_ring_quaternion: [f32; 4],
}
pub fn topology(tick12: u32) -> Result<Topology, String> {
    if tick12 >= 12 {
        return Err("topology tick12 must be 0..11".into());
    }
    let q = RING_QUATERNION_LUT[tick12 as usize];
    Ok(Topology {
        coordinate: id("#1-5")?,
        tick12,
        element_count: [1, 2, 2, 3, 4, 5, 8, 10, 12, 6, 7, 11][tick12 as usize],
        legacy_return_stage: if tick12 < 6 { tick12 } else { 11 - tick12 },
        legacy_ring_quaternion: [q.w, q.x, q.y, q.z],
    })
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Torus {
    pub coordinate: MTreeId,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub fn torus(theta1: f64, theta2: f64) -> Result<Torus, String> {
    if !theta1.is_finite() || !theta2.is_finite() {
        return Err("torus angles must be finite".into());
    }
    let radius = 16.0 / 9.0 + theta1.cos();
    Ok(Torus {
        coordinate: id("#1-5-1")?,
        x: radius * theta2.cos(),
        y: radius * theta2.sin(),
        z: theta1.sin(),
    })
}

/// Source-row evidence is deliberately distinct from A/B/C traversal selection.
/// Retain BOTH operands of a composition, unlike the older synthetic string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RatioDerivation {
    SourceRow {
        family: u32,
        row12: u32,
        coordinate: MTreeId,
        source_ref: &'static str,
    },
    Reciprocal {
        input: Box<RatioEvidence>,
    },
    Composition {
        left: Box<RatioEvidence>,
        right: Box<RatioEvidence>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RatioEvidence {
    pub ratio: [u32; 2],
    pub derivation: RatioDerivation,
}
impl RatioEvidence {
    pub fn reciprocal(self) -> Self {
        Self {
            ratio: [self.ratio[1], self.ratio[0]],
            derivation: RatioDerivation::Reciprocal {
                input: Box::new(self),
            },
        }
    }
    /// Checked exact arithmetic: no u16/u32 product overflow from source prototypes.
    pub fn compose(self, right: Self) -> Result<Self, String> {
        if self.ratio.contains(&0) || right.ratio.contains(&0) {
            return Err("positive ratios required".into());
        }
        let n = u64::from(self.ratio[0]) * u64::from(right.ratio[0]);
        let d = u64::from(self.ratio[1]) * u64::from(right.ratio[1]);
        let (mut a, mut b) = (n, d);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        let n = u32::try_from(n / a).map_err(|_| "ratio numerator overflow")?;
        let d = u32::try_from(d / a).map_err(|_| "ratio denominator overflow")?;
        let ratio = HarmonicRatio::new(n, d).ok_or("invalid ratio")?;
        Ok(Self {
            ratio: [ratio.numerator(), ratio.denominator()],
            derivation: RatioDerivation::Composition {
                left: Box::new(self),
                right: Box::new(right),
            },
        })
    }
}
pub fn source_ratio(family: u32, row12: u32) -> Result<Option<RatioEvidence>, String> {
    if family >= 6 || row12 >= 12 {
        return Err("invalid source ratio row".into());
    }
    let ratio = match (family, row12) {
        (1, 0) | (0, 9) => [1, 1],
        (1, 3) => [4, 3],
        (0, 6) => [2, 3],
        (1, 7) => [16, 9],
        (0, 8) => [8, 9],
        _ => return Ok(None),
    };
    Ok(Some(RatioEvidence {
        ratio,
        derivation: RatioDerivation::SourceRow {
            family,
            row12,
            coordinate: id(&format!("#1-2-{family}"))?,
            source_ref: SOURCE_REF,
        },
    }))
}
pub fn ratio_basis() -> Result<[RatioEvidence; 8], String> {
    let source =
        |f, r| source_ratio(f, r)?.ok_or_else(|| "missing accepted source ratio".to_string());
    let unity = source(1, 0)?;
    let fourth = source(1, 3)?;
    let two_thirds = source(0, 6)?;
    let totality = source(1, 7)?;
    let epogdoon = source(0, 8)?.reciprocal();
    Ok([
        unity,
        fourth.clone(),
        fourth.reciprocal(),
        two_thirds.clone().reciprocal(),
        two_thirds,
        totality.clone(),
        epogdoon.clone(),
        totality.compose(epogdoon)?,
    ])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConjugateParticipation {
    None,
    SourceOnly,
    TargetOnly,
    Both,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PointerEvidence {
    pub source_ref: String,
    pub target_ref: String,
    pub relation_ref: String,
    pub relation_roles: Vec<String>,
}
/// Caller-supplied pointer evidence is retained, not certified as a live graph observation.
#[derive(Debug, Clone)]
pub struct TraversalRequest {
    pub source: QlCoordinate,
    pub target: QlCoordinate,
    pub pointer: PointerEvidence,
    pub family: u32,
    pub row12: u32,
    pub col12: u32,
    pub cycle: u64,
    pub tick12: u32,
    pub participation: ConjugateParticipation,
    pub basis: MusicalBasis,
    pub lens: LensId,
}
#[derive(Debug, Clone)]
pub struct Traversal {
    pub cell: Cell,
    pub ratio: Option<RatioEvidence>,
    pub pointer: PointerEvidence,
    pub source: QlCoordinate,
    pub target: QlCoordinate,
    pub interval_semitones: u8,
    /// Preserve all valid families, walked face, reverse orientation and canonical D2 side.
    /// Frames are conjugated projections of the referenced canonical operator when
    /// the walk begins on the prime face; the operator reference remains provenance.
    pub candidates: Vec<MusicalTraversalCandidate>,
}
pub fn traverse(request: TraversalRequest) -> Result<Traversal, String> {
    if [
        &request.pointer.source_ref,
        &request.pointer.target_ref,
        &request.pointer.relation_ref,
    ]
    .iter()
    .any(|s| s.trim().is_empty())
    {
        return Err("pointer source, target and relation references are required".into());
    }
    let cell = cell(
        request.family,
        request.row12,
        request.col12,
        request.cycle,
        request.tick12,
    )?;
    let ratio = source_ratio(request.family, request.row12)?;
    let (degree, side) = match request.participation {
        ConjugateParticipation::None => (ConjugationDegree::D1, None),
        ConjugateParticipation::Both => (ConjugationDegree::D3, None),
        ConjugateParticipation::SourceOnly => {
            (ConjugationDegree::D2, Some(TraversalExpansionSide::Source))
        }
        ConjugateParticipation::TargetOnly => {
            (ConjugationDegree::D2, Some(TraversalExpansionSide::Target))
        }
    };
    let mut candidates = if request.source.face != request.target.face
        || request.source.position == request.target.position
    {
        Vec::new()
    } else {
        classify_musical_traversal(
            request.basis,
            request.lens,
            request.source.position,
            request.target.position,
            degree,
            side,
        )
        .map_err(|e| e.to_string())?
    };
    // The shared classifier deliberately constructs canonical direct-base frames.
    // Lift that existing frame onto the actual walk's base face. This is the
    // kernel face involution, not another relation/completion grammar.
    if request.source.face == ql_core::QlFace::Conjugate {
        for candidate in &mut candidates {
            for coordinate in &mut candidate.frame.coordinates {
                coordinate.face = coordinate.face.conjugate();
            }
            candidate.frame.pitches = candidate
                .frame
                .coordinates
                .iter()
                .copied()
                .map(|coordinate| pitch_at_lens(request.basis, request.lens, coordinate))
                .collect();
        }
    }
    let interval_semitones = directed_pitch_delta(
        pitch_at_lens(request.basis, request.lens, request.source),
        pitch_at_lens(request.basis, request.lens, request.target),
    );
    Ok(Traversal {
        cell,
        ratio,
        pointer: request.pointer,
        source: request.source,
        target: request.target,
        interval_semitones,
        candidates,
    })
}

/// Versioned instrument-facing JSON boundary. Cycle counters are decimal strings;
/// registered IDs are fixed hexadecimal strings. Caller pointer evidence is
/// never relabelled as an authenticated graph observation.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRequest {
    schema: String,
    source: WireCoordinate,
    target: WireCoordinate,
    pointer: PointerEvidence,
    family: u32,
    row12: u32,
    col12: u32,
    cycle: String,
    tick12: u32,
    participation: WireParticipation,
    basis: WireBasis,
    lens12: u8,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct WireCoordinate {
    position6: u8,
    phase: u8,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum WireParticipation {
    None,
    SourceOnly,
    TargetOnly,
    Both,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum WireBasis {
    Chromatic,
    Fifths,
}
impl WireCoordinate {
    fn coordinate(self) -> Result<QlCoordinate, String> {
        let position = ql_core::QlPosition::new(self.position6).map_err(|e| e.to_string())?;
        let face = match self.phase {
            0 => ql_core::QlFace::Direct,
            1 => ql_core::QlFace::Conjugate,
            _ => return Err("phase must be 0 (direct) or 1 (prime)".into()),
        };
        Ok(QlCoordinate::new(position, face))
    }
}
pub fn traverse_json(input: &str) -> Result<String, String> {
    let wire: WireRequest = serde_json::from_str(input).map_err(|e| e.to_string())?;
    if wire.schema != "ql.m1.traversal/v1" {
        return Err("unsupported M1 traversal schema".into());
    }
    if wire.cycle.is_empty() || !wire.cycle.bytes().all(|b| b.is_ascii_digit()) {
        return Err("cycle must be an unsigned decimal string".into());
    }
    let cycle = wire.cycle.parse::<u64>().map_err(|e| e.to_string())?;
    let lens = LensId::ALL
        .get(usize::from(wire.lens12))
        .copied()
        .ok_or("lens12 must be 0..11")?;
    let basis = match wire.basis {
        WireBasis::Chromatic => MusicalBasis::Chromatic,
        WireBasis::Fifths => MusicalBasis::Fifths,
    };
    let participation = match wire.participation {
        WireParticipation::None => ConjugateParticipation::None,
        WireParticipation::SourceOnly => ConjugateParticipation::SourceOnly,
        WireParticipation::TargetOnly => ConjugateParticipation::TargetOnly,
        WireParticipation::Both => ConjugateParticipation::Both,
    };
    let result = traverse(TraversalRequest {
        source: wire.source.coordinate()?,
        target: wire.target.coordinate()?,
        pointer: wire.pointer,
        family: wire.family,
        row12: wire.row12,
        col12: wire.col12,
        cycle,
        tick12: wire.tick12,
        participation,
        basis,
        lens,
    })?;
    let coordinate = |q: QlCoordinate| {
        serde_json::json!({"position6":q.position.value(),
        "phase":if q.face==ql_core::QlFace::Direct {0} else {1}})
    };
    let candidates: Vec<_> = result.candidates.iter().map(|c|serde_json::json!({
        "family":c.relation.family.as_str(), "pair_index":c.relation.pair_index, "reversed":c.relation.reversed,
        "degree":c.frame.degree.as_str(), "expansion_side":c.frame.expansion_side.map(|side|match side {
            ql_core::ExpansionSide::Left=>"left",ql_core::ExpansionSide::Right=>"right"}),
        "operator_ref":c.frame.structural_operator_ref, "coordinates":c.frame.coordinates.iter().copied().map(coordinate).collect::<Vec<_>>(),
        "pitches":c.frame.pitches })).collect();
    serde_json::to_string(&serde_json::json!({
        "schema":"ql.m1.traversal/v1", "engine_version":ENGINE_VERSION, "source_return_revision":RETURN_REVISION,
        "registry_revision":native_m_registry().manifest().registry_revision,
        "music_version":crate::MUSICAL_COMPLETION_VERSION, "music_source":crate::MUSICAL_DERIVATION_SOURCE_PATH,
        "source":coordinate(result.source), "target":coordinate(result.target), "pointer":result.pointer,
        "pointer_evidence_status":"caller-supplied", "cell":result.cell,"ratio_evidence":result.ratio,
        "basis":match basis {MusicalBasis::Chromatic=>"chromatic", MusicalBasis::Fifths=>"fifths"},
        "lens":lens.code(),"interval_semitones":result.interval_semitones,"candidates":candidates,
        "completion_base_phase":result.source.face.kernel_value(),
        "acceptance":"candidate-pending-K4-and-K5", "experiential_parity":"unassessed"
    })).map_err(|e|e.to_string())
}
