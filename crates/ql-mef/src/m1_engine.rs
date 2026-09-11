//! K5 coordinate-backed M1 state for K8/C++ and M1′.
//!
//! Source literals, accepted arithmetic, historical role-return and generated
//! phase are separate views. This module composes existing QL operations; it
//! does not make a renderer or a second coordinate, music or Context-Frame core.
use crate::m_tree::{MTreeId, MTreeRelation, native_m_registry};
use crate::m1::{self, Clock};
use crate::{ContextFrameId, LensId, MusicalBasis, PoleIdentity};
use ql_core::{QlCoordinate, QlFace, QlPosition, Quat};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::OnceLock;

pub const CONTRACT: &str = "ql.m1.engine/v1";
pub const K4_REVISION: &str = "5b24b95d17234ab5d23d84e658c0cc06434b41a3";

fn id(reference: &str) -> Result<MTreeId, String> {
    m1::node(reference)
        .map(|n| n.id)
        .ok_or_else(|| format!("unknown M1 coordinate: {reference}"))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceCell {
    pub family: u32,
    pub row12: u32,
    pub col12: u32,
    pub raw_literal: String,
    pub digit_root_literal: String,
    pub csv_raw_row: u32,
    pub csv_dr_row: u32,
    pub csv_column: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceCells {
    schema: String,
    source_sha256: String,
    source_git_blob: String,
    source_revision: String,
    cells: Vec<SourceCell>,
}
fn source_cells() -> &'static SourceCells {
    static CELLS: OnceLock<SourceCells> = OnceLock::new();
    CELLS.get_or_init(|| {
        let value: SourceCells = serde_json::from_str(include_str!(
            "../../../fixtures/kernel/m1-source-literals-v1.json"
        ))
        .expect("compiled M1 source tokens");
        assert_eq!(value.schema, "ql.m1.source-literals/v1");
        assert_eq!(value.cells.len(), 864);
        assert_eq!(value.source_revision, m1::RETURN_REVISION);
        for (i, c) in value.cells.iter().enumerate() {
            assert_eq!(
                (c.family, c.row12, c.col12),
                ((i / 144) as u32, ((i % 144) / 12) as u32, (i % 12) as u32)
            );
        }
        value
    })
}
pub fn source_cell(family: u32, row: u32, col: u32) -> Result<&'static SourceCell, String> {
    if family >= 6 || row >= 12 || col >= 12 {
        return Err("source cell outside 6 x 12 x 12".into());
    }
    Ok(&source_cells().cells[(family * 144 + row * 12 + col) as usize])
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Reflection {
    pub coordinate: MTreeId,
    pub subject: MTreeId,
    pub source_ground: MTreeId,
    pub reflection_ground: MTreeId,
    pub phase: u32,
    pub conjugate_phase: u32,
}
pub fn reflection(subject: MTreeId, phase: u32) -> Result<Reflection, String> {
    if phase > 1
        || native_m_registry()
            .node(subject)
            .is_none_or(|n| n.root_position != Some(1))
    {
        return Err("reflection needs an existing M1 subject and direct/prime face".into());
    }
    Ok(Reflection {
        coordinate: id(if phase == 0 { "#1-0" } else { "#1-1" })?,
        subject,
        source_ground: id("#1-0")?,
        reflection_ground: id("#1-1")?,
        phase,
        conjugate_phase: 1 - phase,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Grammar {
    pub coordinates: [MTreeId; 6],
    pub ratio_num: u32,
    pub ratio_den: u32,
    pub explicate: u32,
    pub processual: u32,
    pub decimal_frame: u32,
    pub inversion: [u32; 6],
    pub ring_positions: u32,
    pub nesting: [u32; 4],
    pub binary_states: u32,
    pub relation_states: u32,
    pub resonance_states: u32,
    pub cosmic_degrees: u32,
    pub retained_one_total: u32,
    pub genus: u32,
    pub euler_characteristic: u32,
}
pub fn grammar() -> Result<Grammar, String> {
    let d = crate::derive_matheme();
    let explicate = u32::from(crate::SECOND_SPANDA_VERTICAL.0);
    let processual = d.position_hexad;
    Ok(Grammar {
        coordinates: [
            id("#1-4.0")?,
            id("#1-4.1")?,
            id("#1-4.2")?,
            id("#1-4.3")?,
            id("#1-4.4")?,
            id("#1-4.5")?,
        ],
        ratio_num: d.totality_ratio.numerator(),
        ratio_den: d.totality_ratio.denominator(),
        explicate,
        processual,
        decimal_frame: explicate + processual,
        inversion: std::array::from_fn(|i| {
            u32::from(
                QlPosition::new(i as u8)
                    .expect("sixfold")
                    .complement()
                    .value(),
            )
        }),
        ring_positions: d.twelve_ring,
        nesting: std::array::from_fn(|i| processual + i as u32 + 1),
        binary_states: d.binary_register,
        relation_states: d.self_register,
        resonance_states: d.field_cardinality,
        cosmic_degrees: processual * (explicate + processual) * processual,
        retained_one_total: d.cardinality_sum,
        genus: 1,
        euler_characteristic: 0,
    })
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Rotor {
    pub coordinate: MTreeId,
    pub quaternion: [f32; 4],
}
pub fn rotor(a: f64, b: f64) -> Result<Rotor, String> {
    if !a.is_finite() || !b.is_finite() {
        return Err("rotor phases must be finite radians".into());
    }
    let q = Quat {
        w: a.cos() as f32,
        x: a.sin() as f32,
        y: 0.0,
        z: 0.0,
    }
    .mul(Quat {
        w: b.cos() as f32,
        x: 0.0,
        y: b.sin() as f32,
        z: 0.0,
    });
    Ok(Rotor {
        coordinate: id("#1-5-0")?,
        quaternion: [q.w, q.x, q.y, q.z],
    })
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Carrier {
    pub coordinates: [MTreeId; 6],
    pub clock: Clock,
    pub spinor: [f32; 4],
    pub quadrature: [f64; 2],
    pub opposite_quadrature: [f64; 2],
    pub m2_carrier_count: u32,
    pub genus: u32,
    pub explicate_edges: u32,
    pub identification_slots: u32,
}
pub fn carrier(cycle: u64, tick: u32) -> Result<Carrier, String> {
    let clock = Clock::new(cycle, tick)?;
    let angle = f64::from(clock.degree720).to_radians();
    let spinor = rotor(angle / 2.0, 0.0)?.quaternion;
    let quadrature = [angle.cos(), angle.sin()];
    Ok(Carrier {
        coordinates: [
            id("#1-5-0")?,
            id("#1-5-1")?,
            id("#1-5-2")?,
            id("#1-5-3")?,
            id("#1-5-4")?,
            id("#1-5-5")?,
        ],
        clock,
        spinor,
        quadrature,
        opposite_quadrature: quadrature.map(|v| -v),
        m2_carrier_count: crate::field_cardinality(),
        genus: 1,
        explicate_edges: 4,
        identification_slots: 2,
    })
}
pub fn advance_clock(cycle: u64, tick: u32, ticks: u64) -> Result<Clock, String> {
    Clock::new(cycle, tick)?;
    let rem = ticks % 12 + u64::from(tick);
    let cycle = cycle
        .checked_add(ticks / 12)
        .and_then(|v| v.checked_add(rem / 12))
        .ok_or("M1 cycle overflow")?;
    Clock::new(cycle, (rem % 12) as u32)
}
pub fn relations(coordinate: MTreeId) -> Result<Vec<&'static MTreeRelation>, String> {
    reflection(coordinate, 0)?;
    Ok(native_m_registry().relations_for(coordinate).collect())
}

/// Canonical identities resolve from K2; this is an operation dispatch over
/// those identities, not an independently authored coordinate tree.
pub fn coordinate_operation(reference: &str) -> Result<&'static str, String> {
    let n = m1::node(reference).ok_or("unknown M1 coordinate")?;
    let r = n.source_ref.as_str();
    Ok(match r {
        "#1" => "engine",
        "#1-0" | "#1-1" => "reflection",
        "#1-2" => "ananda-field",
        "#1-3" => "clock-and-spanda",
        "#1-4" => "grammar",
        "#1-5" => "carrier",
        "#1-5-0" => "rotor",
        "#1-5-1" => "torus",
        "#1-5-2" => "clock-and-spinor",
        "#1-5-3" => "opposite-phase",
        "#1-5-4" => "quadrature-and-72-carrier",
        "#1-5-5" => "topological-invariants",
        _ if (0..6).any(|i| r == format!("#1-2-{i}") || r == format!("#1-2-{i}-0")) => {
            "ananda-cell-and-source"
        }
        _ if (0..6).any(|i| r == format!("#1-3-{i}")) || FLOWERING_SEATS.contains(&r) => "spanda",
        _ if (0..6).any(|i| r == format!("#1-4.{i}")) => "grammar-and-formal",
        _ => {
            return Err(format!(
                "new M1 coordinate needs explicit operational reconciliation: {r}"
            ));
        }
    })
}

const FLOWERING_SEATS: [&str; 6] = [
    "#1-3-4.0000",
    "#1-3-4.0/1",
    "#1-3-4.0/1/2",
    "#1-3-4.0/1/2/3",
    "#1-3-4.4.0-4.4/5",
    "#1-3-4.5/0",
];

/// Selected-coordinate reading over a shared engine state. Selecting a different
/// family or stage reads THAT family's operation; it never relabels the active
/// cell or mutates it. Active configuration and selected projection are explicit.
fn selected_reading(config: &EngineConfig, cycle: u64) -> Result<Value, String> {
    let n = m1::node(&config.selected_coordinate).ok_or("unknown selection")?;
    let r = n.source_ref.as_str();
    let clock = Clock::new(cycle, config.tick12)?;
    let content = match r {
        "#1" => json!({"clock":clock,"grammar":grammar()?,"carrier":carrier(cycle,config.tick12)?}),
        "#1-0" | "#1-1" => json!(reflection(
            id(&config.subject_coordinate)?,
            if r == "#1-0" { 0 } else { 1 }
        )?),
        "#1-2" => json!(
            (0..6)
                .map(|f| m1::cell(f, config.row12, config.col12, cycle, config.tick12))
                .collect::<Result<Vec<_>, _>>()?
        ),
        "#1-3" => {
            json!({"clock":clock,"active_pass":m1::spanda(clock.spanda_stage,if clock.spanda_stage==4 {config.flowering_substage}else{0})?})
        }
        "#1-4" => json!(grammar()?),
        "#1-5" | "#1-5-2" | "#1-5-3" | "#1-5-4" | "#1-5-5" => json!(carrier(cycle, config.tick12)?),
        "#1-5-0" => json!(rotor(f64::from(clock.degree720).to_radians() / 2.0, 0.0)?),
        "#1-5-1" => json!(m1::torus(f64::from(clock.degree360).to_radians(), 0.0)?),
        _ if r.starts_with("#1-2-") => {
            let family = r[5..6].parse::<u32>().map_err(|e| e.to_string())?;
            json!({"projection":m1::cell(family,config.row12,config.col12,cycle,config.tick12)?,
                   "source":source_cell(family,config.row12,config.col12)?,
                   "register":if r.ends_with("-0") && r.len()>6 {"digit-root"}else{"source-arithmetic"}})
        }
        _ if r.starts_with("#1-3-") => {
            let stage = r[5..6].parse::<u32>().map_err(|e| e.to_string())?;
            let substage = FLOWERING_SEATS
                .iter()
                .position(|s| *s == r)
                .map(|n| n as u32)
                .unwrap_or(if stage == 4 {
                    config.flowering_substage
                } else {
                    0
                });
            json!(m1::spanda(stage, substage)?)
        }
        _ if r.starts_with("#1-4.") => {
            let stage = r[5..6].parse::<u32>().map_err(|e| e.to_string())?;
            json!({"grammar":grammar()?,"formal":m1::formal(stage)?})
        }
        _ => return Err("coordinate lacks a reviewed M1 operation".into()),
    };
    Ok(
        json!({"coordinate":n.id,"source_ref":n.source_ref,"parent_id":n.parent_id,"children":n.children,
        "source_records":n.records,"operation":coordinate_operation(r)?,"content":content}),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HarmonicSelection {
    pub family: u32,
    pub row12: u32,
    pub col12: u32,
    pub flowering_substage: u32,
    pub lens12: u8,
    pub context_frame: u8,
    pub basis: Basis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Basis {
    Chromatic,
    Fifths,
}
impl Basis {
    fn native(self) -> MusicalBasis {
        match self {
            Self::Chromatic => MusicalBasis::Chromatic,
            Self::Fifths => MusicalBasis::Fifths,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineConfig {
    pub event_ref: String,
    pub subject_coordinate: String,
    pub selected_coordinate: String,
    pub revision: String,
    pub cycle: String,
    pub tick12: u32,
    pub family: u32,
    pub row12: u32,
    pub col12: u32,
    pub flowering_substage: u32,
    pub lens12: u8,
    pub context_frame: u8,
    pub basis: Basis,
}
fn counter(s: &str) -> Result<u64, String> {
    if s.is_empty() || s.len() > 20 || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err("counter must be an unsigned decimal string".into());
    }
    s.parse().map_err(|_| "counter outside u64".into())
}
/// A bounded in-memory operational owner. Fields are private so mutation must
/// validate identity/revision and re-compute the shared state before committing.
/// Storage, authorization and realtime interpolation belong to its consumers.
#[derive(Debug, Clone)]
pub struct M1Engine {
    config: EngineConfig,
    cycle: u64,
    revision: u64,
}
impl M1Engine {
    pub fn new(mut config: EngineConfig) -> Result<Self, String> {
        if config.event_ref.trim().is_empty() || config.event_ref.len() > 4096 {
            return Err("bounded event_ref required".into());
        }
        let cycle = counter(&config.cycle)?;
        let revision = counter(&config.revision)?;
        config.cycle = cycle.to_string();
        config.revision = revision.to_string();
        id(&config.subject_coordinate)?;
        coordinate_operation(&config.selected_coordinate)?;
        m1::cell(
            config.family,
            config.row12,
            config.col12,
            cycle,
            config.tick12,
        )?;
        if config.flowering_substage >= 6
            || config.lens12 >= 12
            || !(1..=7).contains(&config.context_frame)
        {
            return Err("invalid Flowering, lens or Context Frame".into());
        }
        Ok(Self {
            config,
            cycle,
            revision,
        })
    }
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }
    pub fn pole_identity(&self) -> Result<PoleIdentity, String> {
        let clock = Clock::new(self.cycle, self.config.tick12)?;
        PoleIdentity::new(
            &self.config.event_ref,
            self.revision,
            self.config.tick12 as u8,
            clock.degree720 as u16,
        )
        .map_err(|e| e.to_string())
    }
    fn check_revision(&self, expected: u64) -> Result<(), String> {
        if self.revision != expected {
            return Err("stale M1 revision".into());
        }
        Ok(())
    }
    pub fn advance(&mut self, expected: u64, ticks: u64) -> Result<(), String> {
        self.check_revision(expected)?;
        if ticks == 0 {
            return Ok(());
        }
        let clock = advance_clock(self.cycle, self.config.tick12, ticks)?;
        let revision = self.revision.checked_add(1).ok_or("M1 revision overflow")?;
        self.cycle = clock.cycle;
        self.revision = revision;
        self.config.cycle = self.cycle.to_string();
        self.config.tick12 = clock.tick12;
        self.config.revision = revision.to_string();
        Ok(())
    }
    pub fn select(&mut self, expected: u64, coordinate: &str) -> Result<(), String> {
        self.check_revision(expected)?;
        coordinate_operation(coordinate)?;
        let canonical = m1::node(coordinate)
            .ok_or("unknown M1 selection")?
            .source_ref
            .clone();
        if id(&self.config.selected_coordinate)? == id(&canonical)? {
            return Ok(());
        }
        let revision = self.revision.checked_add(1).ok_or("M1 revision overflow")?;
        self.config.selected_coordinate = canonical;
        self.revision = revision;
        self.config.revision = revision.to_string();
        Ok(())
    }
    pub fn configure_harmonics(
        &mut self,
        expected: u64,
        selection: HarmonicSelection,
    ) -> Result<(), String> {
        self.check_revision(expected)?;
        let mut config = self.config.clone();
        config.family = selection.family;
        config.row12 = selection.row12;
        config.col12 = selection.col12;
        config.flowering_substage = selection.flowering_substage;
        config.lens12 = selection.lens12;
        config.context_frame = selection.context_frame;
        config.basis = selection.basis;
        if config == self.config {
            return Ok(());
        }
        config.revision = self
            .revision
            .checked_add(1)
            .ok_or("M1 revision overflow")?
            .to_string();
        let next = Self::new(config)?; // validate completely before committing
        *self = next;
        Ok(())
    }
    pub fn snapshot(&self) -> Result<Value, String> {
        let cfg = &self.config;
        let clock = Clock::new(self.cycle, cfg.tick12)?;
        let cell = m1::cell(cfg.family, cfg.row12, cfg.col12, self.cycle, cfg.tick12)?;
        let lens = LensId::ALL[cfg.lens12 as usize];
        let cf = ContextFrameId::ALL[cfg.context_frame as usize - 1];
        let mode = crate::ModeKind::ALL
            .into_iter()
            .find(|m| m.context_frame() == cf)
            .expect("canonical seven CF modes");
        let tonal = crate::mode_tonic_instance(cfg.basis.native(), lens, mode);
        let q = QlCoordinate::new(
            QlPosition::new(clock.position6 as u8).map_err(|e| e.to_string())?,
            if clock.phase == 0 {
                QlFace::Direct
            } else {
                QlFace::Conjugate
            },
        );
        let selected = id(&cfg.selected_coordinate)?;
        let related = relations(selected)?;
        Ok(json!({
            "schema":CONTRACT,"engine_version":m1::ENGINE_VERSION,"k4_revision":K4_REVISION,
            "registry_revision":native_m_registry().manifest().registry_revision,
            "config":cfg,"selected_reading":selected_reading(cfg,self.cycle)?,"coordinate":selected,"operation":coordinate_operation(&cfg.selected_coordinate)?,
            "reflection":reflection(id(&cfg.subject_coordinate)?,clock.phase)?,
            "clock":clock,"cell":cell,"source_cell":source_cell(cfg.family,cfg.row12,cfg.col12)?,
            "source": {"revision":source_cells().source_revision,"sha256":source_cells().source_sha256,"git_blob":source_cells().source_git_blob},
            "spanda":m1::spanda(clock.spanda_stage,if clock.spanda_stage==4 {cfg.flowering_substage} else {0})?,
            "ananda_role_stage":cfg.family,"source_traits":source_traits()?,"grammar":grammar()?,"carrier":carrier(self.cycle,cfg.tick12)?,
            "legacy_topology":m1::topology(cfg.tick12)?,"ratio_basis":m1::ratio_basis()?,
            "music":{"version":crate::MUSICAL_HARMONIC_VERSION,"completion_version":crate::MUSICAL_COMPLETION_VERSION,
                "lens":lens.code(),"pitch_class":crate::pitch_at_lens(cfg.basis.native(),lens,q),
                "context_frame":cf.code(),"context_expression":cf.expression(),
                "mode":format!("{mode:?}"),"tonic":tonal.tonic,"scale_beneath_tonic":tonal.scale_beneath_tonic,
                "pitches":tonal.pitches,"forms":tonal.form_pattern.map(|f|match f {crate::MefUnitFace::Name=>"name",crate::MefUnitFace::Power=>"power"})},
            "relations":related,
            "standing":{"numeric_computation":"verified","literal_source":"verbatim",
                "topological_necessity":"source-proposition-not-empirical-proof",
                "cpp_embodiment":"K8","instrument":"K9","experiential":"unassessed",
                "live_relations":"K2-serialized; K4 live capture establishes coordinate presence"}
        }))
    }
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
enum Action {
    Advance {
        expected_revision: String,
        ticks: String,
    },
    Select {
        expected_revision: String,
        coordinate: String,
    },
    Harmonics {
        expected_revision: String,
        selection: HarmonicSelection,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: String,
    config: EngineConfig,
    action: Option<Action>,
}
/// Pure request execution for CLI/agents. This is not remote write authority:
/// consumers keep the M1Engine instance and authorize its actions natively.
pub fn engine_json(input: &str) -> Result<String, String> {
    let request: Request = serde_json::from_str(input).map_err(|e| e.to_string())?;
    if request.schema != CONTRACT {
        return Err("unsupported M1 engine schema".into());
    }
    let mut engine = M1Engine::new(request.config)?;
    if let Some(action) = request.action {
        match action {
            Action::Advance {
                expected_revision,
                ticks,
            } => engine.advance(counter(&expected_revision)?, counter(&ticks)?)?,
            Action::Select {
                expected_revision,
                coordinate,
            } => engine.select(counter(&expected_revision)?, &coordinate)?,
            Action::Harmonics {
                expected_revision,
                selection,
            } => engine.configure_harmonics(counter(&expected_revision)?, selection)?,
        }
    }
    serde_json::to_string_pretty(&engine.snapshot()?).map_err(|e| e.to_string())
}

/// Independent direct/prime 6-bit states over the existing relational sixfold.
/// Storage ordinals are not M-coordinate identities or symbolic M3 codons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FiniteField {
    pub coordinate: MTreeId,
    pub left_position: u32,
    pub right_position: u32,
    pub direct_word: u32,
    pub prime_word: u32,
    pub relation_index: u32,
    pub pair_index: u32,
    pub direct_face_index: u32,
    pub prime_face_index: u32,
    pub bitwise_complement: u32,
    pub binary_states: u32,
    pub two_face_states: u32,
    pub paired_states: u32,
}
pub fn finite_field(
    left: u32,
    right: u32,
    direct_word: u32,
    prime_word: u32,
) -> Result<FiniteField, String> {
    let binary = crate::binary_register();
    if left >= 6 || right >= 6 || direct_word >= binary || prime_word >= binary {
        return Err("invalid M1 finite-field input".into());
    }
    let relation_index = ql_core::SixBySixField::canonical()
        .addresses
        .iter()
        .position(|r| {
            u32::from(r.row.position.value()) == left
                && u32::from(r.column.position.value()) == right
        })
        .ok_or("absent canonical relation index")? as u32;
    let bitwise_complement = (0..6).fold(0, |bits, i| {
        let face = if direct_word & (1 << i) == 0 {
            QlFace::Direct
        } else {
            QlFace::Conjugate
        };
        bits | if face.conjugate() == QlFace::Conjugate {
            1 << i
        } else {
            0
        }
    });
    Ok(FiniteField {
        coordinate: id("#1-4")?,
        left_position: left,
        right_position: right,
        direct_word,
        prime_word,
        relation_index,
        pair_index: direct_word * binary + prime_word,
        direct_face_index: direct_word,
        prime_face_index: binary + prime_word,
        bitwise_complement,
        binary_states: binary,
        two_face_states: binary * 2,
        paired_states: binary * binary,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceTraits {
    pub coordinates: [MTreeId; 6],
    pub branch_categories: [u32; 6],
    pub mahamaya_ring: [u32; 6],
    pub parashakti_ring: [u32; 6],
    pub unary_mask: u32,
    pub binary_mask: u32,
    pub relational_mask: u32,
}
pub fn source_traits() -> Result<SourceTraits, String> {
    Ok(SourceTraits {
        coordinates: [
            id("#1-0")?,
            id("#1-1")?,
            id("#1-2")?,
            id("#1-3")?,
            id("#1-4")?,
            id("#1-5")?,
        ],
        branch_categories: [0, 1, 2, 5, 4, 6],
        mahamaya_ring: [1, 2, 4, 8, 7, 5],
        parashakti_ring: [3, 6, 9, 3, 6, 9],
        unary_mask: 1,
        binary_mask: 2,
        relational_mask: 4,
    })
}
