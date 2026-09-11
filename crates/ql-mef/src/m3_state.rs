//! M3's transactional producer shared by Cosmic, Personal reception and M3′.
//!
//! Host authority is checked by the native Action owner before calling this
//! library. A ref or actor label here never grants disclosure/mutation rights.
//! Commands operate on one exact discrete state; C++ receives its generation,
//! not a second clock or renderer-owned transcription table.
use crate::m_tree::{MTreeId, native_m_registry};
use crate::m2_engine::{EventIdentity, InputStamp, MAX_EXACT_JSON_INTEGER};
use crate::m3_engine::{M3Engine, M3NodeKind, native_m3_engine};
use crate::m3_source::native_m3_source;
use ql_core::m3_clock::M3Clock;
use ql_core::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const STATE_SCHEMA: &str = "ql.m3-state/v1";
pub const REQUEST_SCHEMA: &str = "ql.m3-state-request/v1";
pub const COMMAND_SCHEMA: &str = "ql.m3-command/v1";
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BasisRef {
    pub role: String,
    pub reference: String,
    pub revision: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M3Request {
    pub schema: String,
    pub registry_revision: String,
    pub stamp: InputStamp,
    pub subject_ref: String,
    pub occurrence_unix_ms: u64,
    pub receipt_unix_ms: u64,
    pub clock_steps: u64,
    pub address: u8,
    pub pose: u8,
    pub aperture: u8,
    pub matrix_axis: u8,
    pub rna: bool,
    /// Handles only: identity hash, identity quaternion, live composition and
    /// bioquaternion are separate owner readings, never inferred from address64.
    pub bases: Vec<BasisRef>,
    pub m2_basis: Option<InputStamp>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
pub enum M3Operation {
    SelectForm {
        address: u8,
    },
    ChangeLine {
        line: u8,
    },
    ApplyMatrix {
        family: u8,
    },
    SetPose {
        pose: u8,
    },
    SetAperture {
        aperture: u8,
    },
    ReciprocalAperture,
    AdvanceClock {
        steps: u64,
    },
    Transcribe {
        rna: bool,
    },
    CastCreases {
        angles_deg10: [i32; 3],
        velocities_deg10: [i32; 3],
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M3Command {
    pub schema: String,
    pub event_ref: String,
    pub subject_ref: String,
    pub expected_generation: u64,
    pub actor_ref: String,
    pub cause_ref: String,
    pub occurrence_unix_ms: u64,
    pub receipt_unix_ms: u64,
    pub operations: Vec<M3Operation>,
}
#[derive(Debug, Clone, Serialize)]
pub struct M3Receipt {
    pub schema: &'static str,
    pub event_ref: String,
    pub subject_ref: String,
    pub before_generation: u64,
    pub after_generation: u64,
    pub status: &'static str,
    pub failed_operation: Option<usize>,
    pub command: M3Command,
    pub before: Value,
    pub after: Value,
}
#[derive(Debug, Clone)]
pub struct M3State {
    request: M3Request,
    generation: u64,
    fold: FoldState,
    clock: M3Clock,
    rna: bool,
    occurrence_unix_ms: u64,
    receipt_unix_ms: u64,
}
fn text(s: &str) -> Result<(), String> {
    if s.trim().is_empty() || s.len() > 2048 || s.chars().any(char::is_control) {
        Err("invalid bounded M3 reference".into())
    } else {
        Ok(())
    }
}
fn exact(n: u64) -> Result<(), String> {
    if n > MAX_EXACT_JSON_INTEGER {
        Err("integer exceeds exact JSON range".into())
    } else {
        Ok(())
    }
}
fn code(n: u8) -> Result<Codon64, String> {
    if n >= 64 {
        Err("form address outside 0..64".into())
    } else {
        Ok(Codon64::new(n))
    }
}
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
impl M3State {
    pub fn new(request: M3Request) -> Result<Self, String> {
        if request.schema != REQUEST_SCHEMA
            || request.registry_revision != native_m_registry().manifest().registry_revision
        {
            return Err("M3 contract or registry mismatch".into());
        }
        for r in [
            &request.stamp.identity.event_ref,
            &request.stamp.source_ref,
            &request.stamp.contract_ref,
            &request.subject_ref,
        ] {
            text(r)?;
        }
        for n in [
            request.stamp.identity.profile_generation,
            request.clock_steps,
            request.occurrence_unix_ms,
            request.receipt_unix_ms,
        ] {
            exact(n)?;
        }
        if request.matrix_axis >= 3 || request.bases.len() > 64 {
            return Err("M3 input outside bounded domain".into());
        }
        let mut roles = std::collections::BTreeSet::new();
        for b in &request.bases {
            for r in [&b.role, &b.reference, &b.revision] {
                text(r)?;
            }
            if !roles.insert(&b.role) {
                return Err("duplicate M3 basis role".into());
            }
        }
        if let Some(b) = &request.m2_basis {
            if b.identity != request.stamp.identity {
                return Err("M2/M3 event or generation drift".into());
            }
            text(&b.source_ref)?;
            text(&b.contract_ref)?;
        }
        let clock = M3Clock::at_steps(request.clock_steps);
        let mut fold = FoldState::from_codon(
            code(request.address)?,
            ApertureIndex::new(request.aperture).map_err(err)?,
            clock.degree360() / 6,
        );
        fold.set_rotational_index(request.pose).map_err(err)?;
        fold.set_matrix_axis(MatrixFamily::ALL[request.matrix_axis as usize].axis());
        Ok(Self {
            generation: request.stamp.identity.profile_generation,
            fold,
            clock,
            rna: request.rna,
            occurrence_unix_ms: request.occurrence_unix_ms,
            receipt_unix_ms: request.receipt_unix_ms,
            request,
        })
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn fold(&self) -> &FoldState {
        &self.fold
    }
    pub fn clock(&self) -> M3Clock {
        self.clock
    }
    fn replace_form(&mut self, codon: Codon64) {
        let mut next =
            FoldState::from_codon(codon, self.fold.aperture16(), self.clock.degree360() / 6);
        next.set_rotational_index(self.fold.rotational_index().min(next.state_count() - 1))
            .expect("bounded inherited pose");
        next.set_matrix_axis(self.fold.active_matrix_axis());
        self.fold = next;
    }
    /// Atomic batch: any invalid operation is an error; any source-native RES
    /// gap returns a non-mutating provisional receipt, including earlier ops.
    pub fn apply(&mut self, command: M3Command) -> Result<M3Receipt, String> {
        if command.schema != COMMAND_SCHEMA
            || command.event_ref != self.request.stamp.identity.event_ref
            || command.subject_ref != self.request.subject_ref
            || command.expected_generation != self.generation
        {
            return Err("M3 command contract/event/subject/generation mismatch".into());
        }
        text(&command.actor_ref)?;
        text(&command.cause_ref)?;
        exact(command.occurrence_unix_ms)?;
        exact(command.receipt_unix_ms)?;
        if command.operations.is_empty() || command.operations.len() > 64 {
            return Err("M3 batch must contain 1..64 operations".into());
        }
        let next_generation = self
            .generation
            .checked_add(1)
            .ok_or("generation overflow")?;
        exact(next_generation)?;
        let before = self.snapshot();
        let mut next = self.clone();
        let mut gap = None;
        for (index, op) in command.operations.iter().enumerate() {
            match *op {
                M3Operation::SelectForm { address } => next.replace_form(code(address)?),
                M3Operation::ChangeLine { line } => {
                    next.replace_form(next.fold.codon().line_change(line).map_err(err)?)
                }
                M3Operation::ApplyMatrix { family } => {
                    let f = *MatrixFamily::ALL
                        .get(family as usize)
                        .ok_or("invalid matrix family")?;
                    match next.fold.apply_matrix(f).map_err(err)? {
                        ApplyOutcome::Applied(fold) => next.fold = fold,
                        ApplyOutcome::Provisional => {
                            gap = Some(index);
                            break;
                        }
                    }
                }
                M3Operation::SetPose { pose } => {
                    next.fold.set_rotational_index(pose).map_err(err)?
                }
                M3Operation::SetAperture { aperture } => {
                    let ap = ApertureIndex::new(aperture).map_err(err)?;
                    let mut f =
                        FoldState::from_codon(next.fold.codon(), ap, next.clock.degree360() / 6);
                    f.set_rotational_index(next.fold.rotational_index())
                        .map_err(err)?;
                    f.set_matrix_axis(next.fold.active_matrix_axis());
                    next.fold = f;
                }
                M3Operation::ReciprocalAperture => next.fold.fold_to_reciprocal_aperture(),
                M3Operation::AdvanceClock { steps } => {
                    let total = next
                        .clock
                        .steps()
                        .checked_add(steps)
                        .ok_or("clock overflow")?;
                    exact(total)?;
                    next.clock = M3Clock::at_steps(total);
                    next.replace_form(next.fold.codon());
                }
                M3Operation::Transcribe { rna } => next.rna = rna,
                M3Operation::CastCreases {
                    angles_deg10,
                    velocities_deg10,
                } => {
                    let sites = std::array::from_fn(|i| SiteReading {
                        signed_angle: angles_deg10[i],
                        angular_velocity: velocities_deg10[i],
                    });
                    let f = FoldState::from_cast(
                        sites,
                        next.fold.aperture16(),
                        next.clock.degree360() / 6,
                    );
                    next.replace_form(f.codon());
                }
            }
        }
        if gap.is_none() {
            next.generation = next_generation;
            next.occurrence_unix_ms = command.occurrence_unix_ms;
            next.receipt_unix_ms = command.receipt_unix_ms;
            *self = next;
        }
        Ok(M3Receipt {
            schema: "ql.m3-receipt/v1",
            event_ref: command.event_ref.clone(),
            subject_ref: command.subject_ref.clone(),
            before_generation: command.expected_generation,
            after_generation: self.generation,
            status: if gap.is_some() {
                "provisional-unchanged"
            } else {
                "applied"
            },
            failed_operation: gap,
            command,
            before,
            after: self.snapshot(),
        })
    }
    /// One source-attributed packet for every consumer. Opening/hiding a surface
    /// does not call an operation, advance a generation, or restart this clock.
    pub fn snapshot(&self) -> Value {
        let engine = native_m3_engine();
        let source = native_m3_source();
        let c = self.fold.codon();
        let clock = engine.clock(self.clock);
        let ap = self.fold.aperture16();
        let charge = c.four_charge();
        let g = self.fold.geometry();
        let q = quat_codon_state(c, self.fold.rotational_index());
        let efwa = c
            .nucleotides()
            .map(|n| [2usize, 1, 0, 3][n.bits() as usize]);
        let mut counts = [0; 4];
        for e in efwa {
            counts[e] += 1;
        }
        let ref_value = |kind, index| {
            let n = engine.node(kind, index).unwrap();
            json!({"id":n.id,"ref":n.source_ref})
        };
        let genetic = source.genetic(c);
        let card = TarotBridge::kernel()
            .card_of_codon(c)
            .expect("complete minor exact cover");
        let linked:Vec<_>=source.matrix_cells().iter().filter(|cell|cell.address==c.address()).map(|cell|json!({"cell":cell,
            "qualified_pair_relations":cell.pair_relations.iter().map(|id|source.relation(*id).unwrap()).collect::<Vec<_>>(),
            "qualified_codon_relations":cell.codon_relations.iter().map(|id|source.relation(*id).unwrap()).collect::<Vec<_>>()
        })).collect();
        json!({"schema":STATE_SCHEMA,"registry_revision":self.request.registry_revision,"domain_revision":source.revision(),"source_revision":source.source_revision(),
            "identity":EventIdentity{event_ref:self.request.stamp.identity.event_ref.clone(),profile_generation:self.generation},"subject_ref":self.request.subject_ref,
            "occurrence_unix_ms":self.occurrence_unix_ms,"receipt_unix_ms":self.receipt_unix_ms,
            "bases":{"initial":self.request.stamp,"m2":self.request.m2_basis,"owner_readings":self.request.bases,
                "standing":"original-input-generations-retained-not-new-observations"},
            "consumers":["epi.cosmic.123","epi.personal.450","epi.deep.m3"],
            "form":{"codon":ref_value(M3NodeKind::Codon,c.address() as usize),"hexagram":ref_value(M3NodeKind::Hexagram,c.address() as usize),
                "address":c.address(),"pose":self.fold.rotational_index(),"pose_ordinal":self.fold.rotational_pose().ordinal(),"state_count":self.fold.state_count(),"lawful_field":472,
                "nucleotides":c.nucleotides().map(|n|n.bits()),"coin_values":c.site_values().map(|v|v.value()),"charges_pp_mm_mp_pm":[charge.pp,charge.mm,charge.mp,charge.pm],
                "pair_xy":ref_value(M3NodeKind::Pair,g.pair_xy.index() as usize),"pair_yz":ref_value(M3NodeKind::Pair,g.pair_yz.index() as usize),
                "pair_angles_deg10":[g.pair_angle_xy().0,g.pair_angle_yz().0],"hinge":g.hinge.bits(),"matrix_axis":self.fold.active_matrix_axis() as u8,
                "angles_deg10":self.fold.sites().map(|s|s.signed_angle),"velocities_deg10":self.fold.sites().map(|s|s.angular_velocity),
                "orientation_seed_wxyz":[q.w,q.x,q.y,q.z],"orientation_standing":"M3-source-codon-rotation-not-identity-quaternion"},
            "elemental":{"component_order":["Earth","Fire","Water","Air"],"site_components":efwa,"counts":counts,
                "standing":"three-site-form-carrier-not-personal-constitution-or-bioquaternion"},
            "aperture":{"static_lenses":16,"total_lenses":18,"index":ap.index(),"division_deg10":ap.division_deg10(),"reciprocal_index":ap.reciprocal().index(),
                "reciprocal_division_deg10":ap.complement_deg10(),"fibonacci_phase60":self.fold.fibonacci_phase60(),"void_ring_orientation_deg10":ap.orientation().0},
            "clock":{"steps":self.clock.steps(),"degree720":self.clock.degree720(),"degree360":self.clock.degree360(),"layer":self.clock.layer(),"tick12":self.clock.tick12(),
                "completed_double_covers":self.clock.completed_double_covers(),"polar720":self.clock.polar720(),"degree":{"id":clock.degree.id,"ref":clock.degree.source_ref},
                "backbone":{"id":clock.backbone.id,"ref":clock.backbone.source_ref},"clockwise_id":clock.clockwise.id,"polar_id":clock.polar.id,
                "source_backbone_prototype":source.backbone(clock.backbone.id),"uniform_hexagram_estimate":self.clock.uniform_hexagram_estimate(),
                "estimate_standing":"legacy-arithmetic-estimate-not-selected-form-or-source-prototype"},
            "transcription":{"rna":self.rna,"sequence":String::from_utf8(M3Engine::transcribe(c,self.rna).to_vec()).unwrap(),"source":genetic,
                "retained_amino_slot":m3_codon_amino_index(c),"retained_slot_standing":"retained-internal-table-not-source-translation-equivalence"},
            "iching":{"complement_address":complement(c.address()),"nuclear_address":nuclear_hexagram(c.address()),
                "line_change_addresses":c.neighbours().map(|v|v.address()),"standing":"native-six-bit-law-not-uncorrected-source-LINE_CHANGE-claims"},
            "tarot":{"card_id":card.card_id(),"suit":card.suit().name(),"rank":card.rank_name(),
                "codons":card.codons().map(|c|c.address()).collect::<Vec<_>>(),"standing":"retained-native-exact-cover-with-source-expression-separately-qualified"},
            "source_matrix_cells":linked,"source_discrepancy_templates":source.discrepancies().len(),
            "discrepancy_authority":"ql.m-ledger/v1: current lifecycle, not frozen source templates",
            "readiness":{"discrete":"operative","source":"retained-qualified-reading","continuous_cpp":"consumer-required","personal_identity":"owner-supplied-handles","experiential":"not-claimed"}})
    }
    pub fn source_node(&self, id: MTreeId) -> Option<&'static crate::m3_source::SourceNode> {
        native_m3_source().node(id)
    }
}
