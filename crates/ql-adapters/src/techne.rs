//! The shared L5 Technē contract: `ql.techne/v1`.
//!
//! One language binding of the language-neutral contract pinned in
//! `schemas/techne/`. The adapter composes attributable readings over native
//! owners; it is not a store, never rewrites native identity, and never
//! executes a mutation itself — native Actions route to their owner under the
//! owner's authority. Canonical text: `docs/L5-TECHNE-INSTRUMENT-WAYFINDER.md`
//! §1–§2 and §18 (T0). TB0 (2026-09-16, issue #212) extends the contract
//! additively — relation evidence/standing/temporal qualification, place
//! relation type and uncertainty, attempt/return continuity, warranted
//! M-coordinate and Return refs, application-cut disclosure, the
//! situated-Agency role floor, and the session's dual-reading state — see
//! `docs/L5-TECHNE-TB0-CONNECTIVE-BASE.md`; the contract tag is unchanged.

use serde::{Deserialize, Serialize};

pub const TECHNE_CONTRACT: &str = "ql.techne/v1";

/// The instrument set of the one M′ field's two readings (amended
/// 2026-09-16, owner-ratified). The six 4:2 deep instruments bind M0′–M5′:
/// project (ground), canvas, timeline (relation field), journey, place
/// (world), palace. `Expressions` is the conjugate 3:3 Expression reading —
/// not a deep instrument. Instrument identity is disclosure vocabulary, not
/// an application boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TechneInstrument {
    Project,
    Canvas,
    Timeline,
    Journey,
    Place,
    Palace,
    Expressions,
}

/// Which reading of the one M′ field an aperture carries. Crossing changes
/// the mode of disclosure and available operations, never the subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TechneReadingKind {
    #[serde(rename = "4:2-deep")]
    DeepFourTwo,
    #[serde(rename = "3:3-conjugate")]
    ConjugateThreeThree,
}

impl TechneInstrument {
    /// The M′ office this instrument is the Technē face of. `None` on the
    /// conjugate 3:3 Expression reading — Expression is not a deep
    /// instrument and binds no M′ office.
    pub fn m_prime(self) -> Option<u8> {
        match self {
            TechneInstrument::Project => Some(0),
            TechneInstrument::Canvas => Some(1),
            TechneInstrument::Timeline => Some(2),
            TechneInstrument::Journey => Some(3),
            TechneInstrument::Place => Some(4),
            TechneInstrument::Palace => Some(5),
            TechneInstrument::Expressions => None,
        }
    }

    /// The application cut this instrument belongs to.
    pub fn reading(self) -> TechneReadingKind {
        match self {
            TechneInstrument::Expressions => TechneReadingKind::ConjugateThreeThree,
            _ => TechneReadingKind::DeepFourTwo,
        }
    }

    /// The Research Canvas compatibility-transport vocabulary mapped onto the
    /// canonical contract instruments (TB0, 2026-09-16). This is a migration
    /// bridge for the `TechneWorkspaceTransport` seam: legacy surface names
    /// resolve onto canonical instruments while native refs stay verbatim.
    /// Unknown names return `None` — never guess an instrument.
    pub fn from_transport_alias(alias: &str) -> Option<TechneInstrument> {
        match alias.trim().to_ascii_lowercase().as_str() {
            "projects" | "project" | "project-graph" => Some(TechneInstrument::Project),
            "canvas" | "constellation" => Some(TechneInstrument::Canvas),
            "timeline" | "relations" | "relation-field" => Some(TechneInstrument::Timeline),
            "story" | "journey" | "scenes" => Some(TechneInstrument::Journey),
            "place" | "places" | "map" | "street" | "globe" | "world" => {
                Some(TechneInstrument::Place)
            }
            "palace" => Some(TechneInstrument::Palace),
            "expressions" | "expression" => Some(TechneInstrument::Expressions),
            _ => None,
        }
    }
}

/// Snapshot/revision basis shared by every view of one coherent reading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadingSnapshot {
    #[serde(default)]
    pub revision: Option<String>,
    #[serde(default)]
    pub basis_ref: Option<String>,
}

/// The selected subject in its native identity. Refs stay opaque: adapters
/// carry them verbatim and never re-key or shorten them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechneSubject {
    pub subject_ref: String,
    pub native_owner: String,
    #[serde(default)]
    pub native_revision: Option<String>,
    #[serde(default)]
    pub readings: Vec<SubjectReading>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub standing: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectReading {
    pub r#ref: String,
    #[serde(default)]
    pub revision: Option<String>,
}

/// A typed relation inside the bounded whole. Relation vocabulary is the
/// provider's own; origins are preserved, not relabelled. TB0 (2026-09-16)
/// adds optional stable relation identity, evidence/standing, owner-supplied
/// derivation/confidence and a temporal qualification that resolves against
/// the reading's temporal facets — nothing here is inferred by the adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WholeRelation {
    pub relation: String,
    #[serde(default)]
    pub relation_ref: Option<String>,
    pub from_ref: String,
    pub to_ref: String,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    pub origin_ref: Option<String>,
    /// Evidence standing (fact, interpretation, myth, allegation, disputed,
    /// …) in the owner's vocabulary. Trans-temporal/archetypal relations stay
    /// distinct from dated ones through standing plus the absence of a
    /// temporal qualification.
    #[serde(default)]
    pub standing: Option<String>,
    #[serde(default)]
    pub source_ref: Option<String>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional real temporal qualification: resolves against a `facet_ref`
    /// in the reading's temporal array. Absent means no dated qualification —
    /// never a manufactured one.
    #[serde(default)]
    pub temporal_facet_ref: Option<String>,
    /// Owner-supplied derivation only; the adapter never infers it.
    #[serde(default)]
    pub derivation_ref: Option<String>,
    /// Owner-supplied confidence, in the owner's own terms.
    #[serde(default)]
    pub confidence: Option<String>,
}

/// Bounded local whole: leaf → local whole navigation without a global graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechneWhole {
    pub whole_ref: String,
    #[serde(default)]
    pub member_refs: Vec<String>,
    #[serde(default)]
    pub relations: Vec<WholeRelation>,
    #[serde(default)]
    pub focus_refs: Vec<String>,
}

/// The existing QL warrant discipline: provenance + result class + evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QlWarrant {
    pub result_class: QlResultClass,
    pub evidence_refs: Vec<String>,
    pub provenance_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QlResultClass {
    Canonical,
    Deterministic,
    SemanticStochastic,
    Research,
}

/// QL shape/constellation/lens/refraction reading. Present only when
/// warranted; the schema requires the warrant, so unwarranted QL metadata is
/// inexpressible in a valid reading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarrantedQlReading {
    #[serde(default)]
    pub address: Option<String>,
    /// TB0: canonical M-coordinate ref where warranted (the subject's M0–M5
    /// registry coordinate). M′ instrument bindings stay on the disclosure
    /// entries; this carries the warranted M identity of the reading itself.
    #[serde(default)]
    pub m_coordinate_ref: Option<String>,
    #[serde(default)]
    pub shape_ref: Option<String>,
    #[serde(default)]
    pub constellation_ref: Option<String>,
    #[serde(default)]
    pub lens_ref: Option<String>,
    #[serde(default)]
    pub sublens_ref: Option<String>,
    #[serde(default)]
    pub context_frame_ref: Option<String>,
    #[serde(default)]
    pub refraction_summary: Option<String>,
    #[serde(default)]
    pub harmonic_reading: Option<String>,
    #[serde(default)]
    pub geometric_reading: Option<String>,
    #[serde(default)]
    pub vak_source_ref: Option<String>,
    #[serde(default)]
    pub derivation_refs: Vec<String>,
    /// TB0: where this warranted reading Returns into knowledge ground, when
    /// the owner supplies one.
    #[serde(default)]
    pub return_ref: Option<String>,
    pub warrant: QlWarrant,
}

/// One labelled native time fact. Occurrence, receipt, validity and the
/// continuity refs (day/now/session/run) stay distinct by law.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalFacet {
    pub kind: TemporalKind,
    #[serde(default)]
    pub facet_ref: Option<String>,
    #[serde(default)]
    pub instant: Option<String>,
    #[serde(default)]
    pub interval: Option<TemporalInterval>,
    #[serde(default)]
    pub precision: Option<TemporalPrecision>,
    #[serde(default)]
    pub day_ref: Option<String>,
    #[serde(default)]
    pub now_ref: Option<String>,
    #[serde(default)]
    pub session_ref: Option<String>,
    #[serde(default)]
    pub run_ref: Option<String>,
    /// TB0: Factory attempt continuity when the owner supplies it; rides a
    /// run/session facet, never replaces it.
    #[serde(default)]
    pub attempt_ref: Option<String>,
    /// TB0: Return continuity (e.g. a late Factory Return) when the owner
    /// supplies it; occurrence and receipt stay distinct regardless.
    #[serde(default)]
    pub return_ref: Option<String>,
    #[serde(default)]
    pub timezone_policy_ref: Option<String>,
    #[serde(default)]
    pub uncertainty: Option<String>,
    #[serde(default)]
    pub source_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemporalKind {
    Occurrence,
    Receipt,
    Valid,
    SourceCreated,
    SourceModified,
    Day,
    Now,
    Session,
    Run,
    Generation,
    Presentation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TemporalPrecision {
    Millennium,
    Century,
    Decade,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Subsecond,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalInterval {
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub from_precision: Option<TemporalPrecision>,
    #[serde(default)]
    pub to_precision: Option<TemporalPrecision>,
}

/// Temporally valid Place identity. Coordinates are optional; precision,
/// hierarchy and validity are data, and `unlocated` is a truthful state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceFacet {
    pub place_ref: String,
    /// TB0: the subject's native place-relation type, preserved verbatim
    /// (OCCURRED_AT, LOCATED_IN, OPERATED_IN, TRAVELLED_TO,
    /// MYTH_LOCATED_AT, …). These are not interchangeable.
    #[serde(default)]
    pub relation: Option<String>,
    #[serde(default)]
    pub identity: Option<PlaceIdentity>,
    #[serde(default)]
    pub geometry: Option<PlaceGeometry>,
    pub precision: PlacePrecision,
    /// TB0: owner-supplied spatial uncertainty in the owner's terms; never
    /// inferred to fill this in.
    #[serde(default)]
    pub uncertainty: Option<String>,
    #[serde(default)]
    pub hierarchy: Vec<PlaceHierarchyMember>,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_to: Option<String>,
    #[serde(default)]
    pub observer_frame: Option<String>,
    #[serde(default)]
    pub source_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlacePrecision {
    Exact,
    Approximate,
    Region,
    Unlocated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceIdentity {
    #[serde(default)]
    pub names: Vec<PlaceName>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceName {
    pub name: String,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_to: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceGeometry {
    pub r#type: PlaceGeometryType,
    pub coordinates: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlaceGeometryType {
    Point,
    Polygon,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceHierarchyMember {
    pub place_ref: String,
    pub relation: String,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_to: Option<String>,
}

/// Exact source selector: the unit every instrument can return to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "unit", rename_all = "snake_case")]
pub enum SourceSelector {
    TextSpan {
        start: u64,
        end: u64,
        #[serde(default)]
        anchor_ref: Option<String>,
    },
    TimestampRange {
        from: String,
        to: String,
    },
    ImageRegion {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    Other {
        kind: String,
        value: String,
    },
}

/// Source provenance with standing and optional exact selector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceProvenance {
    pub source_ref: String,
    #[serde(default)]
    pub source_revision: Option<String>,
    pub native_owner: String,
    #[serde(default)]
    pub selector: Option<SourceSelector>,
    #[serde(default)]
    pub standing: Option<String>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Binding to the existing O:I Expression-world substrate — never a second
/// scene ontology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressionBinding {
    pub expression_ref: String,
    #[serde(default)]
    pub revision: Option<String>,
    #[serde(default)]
    pub scene_ref: Option<String>,
    #[serde(default)]
    pub composition_ref: Option<String>,
    #[serde(default)]
    pub profile_ref: Option<String>,
}

/// A native owner's disclosed Action. Routing only; the owner executes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeActionRef {
    pub action_ref: String,
    pub native_owner: String,
    pub authority: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub expected_effects: Vec<String>,
    #[serde(default)]
    pub input_schema_ref: Option<String>,
}

/// Capability honesty for one instrument: availability with an explicit
/// reason whenever it is unavailable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstrumentDisclosure {
    pub instrument: TechneInstrument,
    pub available: bool,
    #[serde(default)]
    pub reason: Option<String>,
    /// The M′ office this instrument is the Technē face of. Absent on the
    /// conjugate 3:3 reading.
    #[serde(default)]
    pub m_prime: Option<u8>,
    #[serde(default)]
    pub reading: Option<TechneReadingKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DegradedDisclosure {
    pub instrument: TechneInstrument,
    pub reason: String,
}

/// Gentle neighbouring-instrument affordance. Suggestion is not execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosureSuggestion {
    pub instrument: TechneInstrument,
    pub reason: String,
}

/// Cut-level availability: whether the 4:2 deep reading and the 3:3
/// conjugate reading can actually be entered for this subject, and why not
/// when they cannot. TB0 (2026-09-16).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationCutDisclosure {
    pub cut: TechneReadingKind,
    pub available: bool,
    #[serde(default)]
    pub reason: Option<String>,
}

/// The situated-Agency role kinds of the dual-reading lock. These are role
/// bindings over existing Actuation/AIKit machinery — never new canonical
/// Agent identities, and never a renaming of the Guardians.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgencyRoleKind {
    /// Guardian_i stewardship context (spans both readings).
    Guardian,
    /// Anima_i = M_i × S4′, the expressive operator of the 3:3 reading.
    Anima,
    /// Aletheia_i = M_i × S5′, the disclosure/knowledge-metabolism/Return
    /// operator of the 4:2 reading.
    Aletheia,
    /// Technē_i := Aletheia_i while situated in and operating the M_i′ deep
    /// instrument.
    Techne,
}

/// One situated-Agency role binding (TB0, 2026-09-16). Structured state from
/// which ordinary Actuation/AIKit machinery constructs `Guardian_i`
/// stewardship, `Anima_i` expressive roles and `Aletheia_i`/`Technē_i`
/// deep-instrument roles. A situated Agency receives exact subject, cut,
/// instrument, selection, sources, Actions, authority and Return target from
/// the rest of the reading/session — this binding names who is situated and
/// under whose stewardship.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgencyRole {
    pub role: AgencyRoleKind,
    /// The coordinate i this situated role inhabits (M_i × S4′/S5′).
    pub m_index: u8,
    /// Anima binds the 3:3 conjugate reading; Aletheia/Techne bind the 4:2
    /// deep reading; Guardian stewardship spans both and may leave this
    /// absent.
    #[serde(default)]
    pub reading: Option<TechneReadingKind>,
    /// The instrument this role operates: the M_i′ deep instrument for
    /// Techne; the Expression reading for Anima.
    #[serde(default)]
    pub instrument: Option<TechneInstrument>,
    /// The canonical Guardian identity anchoring this situated role
    /// (stewardship context). Present does not mean identical:
    /// Guardian_i != Anima_i != Techne_i.
    #[serde(default)]
    pub guardian_ref: Option<String>,
    /// The existing native AgentSession the Agency is situated in; never a
    /// new runtime.
    #[serde(default)]
    pub agent_session_ref: Option<String>,
    /// The AIKit Profile the Agency is constructed through.
    #[serde(default)]
    pub profile_ref: Option<String>,
    /// The authority this role operates under, in disclosure terms;
    /// execution always crosses the native owner seam.
    #[serde(default)]
    pub authority: Option<String>,
    /// Disclosure limits, e.g. Nara-private state excluded.
    #[serde(default)]
    pub privacy: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechneDisclosure {
    pub instruments: Vec<InstrumentDisclosure>,
    #[serde(default)]
    pub degraded: Vec<DegradedDisclosure>,
    /// TB0: cut-level availability over and above per-instrument entries.
    #[serde(default)]
    pub application_cuts: Vec<ApplicationCutDisclosure>,
    #[serde(default)]
    pub suggestions: Vec<DisclosureSuggestion>,
}

impl TechneDisclosure {
    /// The recorded availability of one application cut, when disclosed.
    pub fn cut(&self, cut: TechneReadingKind) -> Option<&ApplicationCutDisclosure> {
        self.application_cuts.iter().find(|entry| entry.cut == cut)
    }
}

/// The portable Technē reading: one subject, its optional facets, its
/// actions, and its capability disclosure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechneReading {
    pub contract: String,
    pub reading_ref: String,
    #[serde(default)]
    pub snapshot: Option<ReadingSnapshot>,
    pub subject: TechneSubject,
    #[serde(default)]
    pub whole: Option<TechneWhole>,
    #[serde(default)]
    pub ql: Option<WarrantedQlReading>,
    #[serde(default)]
    pub temporal: Vec<TemporalFacet>,
    #[serde(default)]
    pub spatial: Vec<PlaceFacet>,
    #[serde(default)]
    pub provenance: Vec<SourceProvenance>,
    #[serde(default)]
    pub expressions: Vec<ExpressionBinding>,
    #[serde(default)]
    pub actions: Vec<NativeActionRef>,
    /// TB0: situated-Agency role bindings constructible through existing
    /// Actuation/AIKit machinery. Absent when no role is situated here.
    #[serde(default)]
    pub agency: Vec<AgencyRole>,
    pub disclosure: TechneDisclosure,
}

impl TechneReading {
    /// Semantic checks the JSON Schema expresses but programmatic
    /// construction can bypass: contract tag, temporal carriers, and the
    /// unavailable-instrument reason law.
    pub fn validate(&self) -> Result<(), crate::AdapterError> {
        if self.contract != TECHNE_CONTRACT {
            return Err(crate::AdapterError::InvalidTechneReading(format!(
                "contract tag {:?} is not {TECHNE_CONTRACT}",
                self.contract
            )));
        }
        for facet in &self.temporal {
            let carried = facet.instant.is_some()
                || facet.interval.is_some()
                || facet.day_ref.is_some()
                || facet.now_ref.is_some()
                || facet.session_ref.is_some()
                || facet.run_ref.is_some();
            if !carried {
                return Err(crate::AdapterError::InvalidTechneReading(format!(
                    "temporal facet {:?} carries no instant, interval or continuity ref",
                    facet.kind
                )));
            }
        }
        for instrument in &self.disclosure.instruments {
            if !instrument.available && instrument.reason.is_none() {
                return Err(crate::AdapterError::InvalidTechneReading(format!(
                    "instrument {:?} is unavailable without a reason",
                    instrument.instrument
                )));
            }
            if let Some(m_prime) = instrument.m_prime {
                if m_prime > 5 {
                    return Err(crate::AdapterError::InvalidTechneReading(format!(
                        "instrument {:?} binds M′{m_prime}; the field has M′0–M′5 only",
                        instrument.instrument
                    )));
                }
            }
        }
        for cut in &self.disclosure.application_cuts {
            if !cut.available && cut.reason.is_none() {
                return Err(crate::AdapterError::InvalidTechneReading(format!(
                    "application cut {:?} is unavailable without a reason",
                    cut.cut
                )));
            }
        }
        // The situated-Agency role law: Anima inhabits the 3:3 reading,
        // Aletheia/Techne the 4:2 reading, Techne operates the deep
        // instrument of its own coordinate, Anima operates the Expression
        // reading — and no binding becomes a Guardian replacement.
        for role in &self.agency {
            if role.m_index > 5 {
                return Err(crate::AdapterError::InvalidTechneReading(format!(
                    "agency role {:?} binds M′{}; the field has M′0–M′5 only",
                    role.role, role.m_index
                )));
            }
            match role.role {
                AgencyRoleKind::Guardian => {}
                AgencyRoleKind::Anima => {
                    if let Some(reading) = role.reading {
                        if reading != TechneReadingKind::ConjugateThreeThree {
                            return Err(crate::AdapterError::InvalidTechneReading(
                                "Anima_i inhabits the 3:3 conjugate reading, not the 4:2 deep reading"
                                    .to_string(),
                            ));
                        }
                    }
                    if let Some(instrument) = role.instrument {
                        if instrument != TechneInstrument::Expressions {
                            return Err(crate::AdapterError::InvalidTechneReading(
                                "Anima_i operates the Expression reading, not a deep instrument"
                                    .to_string(),
                            ));
                        }
                    }
                }
                AgencyRoleKind::Aletheia | AgencyRoleKind::Techne => {
                    if let Some(reading) = role.reading {
                        if reading != TechneReadingKind::DeepFourTwo {
                            return Err(crate::AdapterError::InvalidTechneReading(
                                "Aletheia_i/Technē_i inhabit the 4:2 deep reading, not the 3:3 conjugate reading"
                                    .to_string(),
                            ));
                        }
                    }
                    if role.role == AgencyRoleKind::Techne {
                        if let Some(instrument) = role.instrument {
                            let bound = instrument.m_prime();
                            if bound != Some(role.m_index) {
                                return Err(crate::AdapterError::InvalidTechneReading(format!(
                                    "Technē_{} operates its own coordinate's deep instrument (M′{}); the binding names {:?}",
                                    role.m_index,
                                    bound
                                        .map(|m| m.to_string())
                                        .unwrap_or_else(|| "no M′ office".to_string()),
                                    instrument
                                )));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// `true` when the reading warrants at least one place facet.
    pub fn has_spatial(&self) -> bool {
        !self.spatial.is_empty()
    }

    /// `true` when the reading carries a warranted QL facet.
    pub fn has_warranted_ql(&self) -> bool {
        self.ql.is_some()
    }

    /// The M0′ ground entry: the subject's own bounded whole when the whole
    /// is disclosed, else the subject itself — step 0 of the canonical
    /// traversal.
    pub fn ground_ref(&self) -> &str {
        match &self.whole {
            Some(whole) => &whole.whole_ref,
            None => &self.subject.subject_ref,
        }
    }
}

/// A mutation request entering the adapter. The adapter resolves the native
/// owner; it never executes. Execution crosses the native authority seam.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechneActionRoute {
    pub action_ref: String,
    pub subject_ref: String,
    #[serde(default)]
    pub selection_ref: Option<String>,
    #[serde(default)]
    pub input: Option<serde_json::Value>,
}

/// What routing returned: the owning native authority, or the refusal reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechneActionRouteReceipt {
    pub action_ref: String,
    pub native_owner: String,
    pub routed: bool,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub authority: Option<String>,
    #[serde(default)]
    pub expected_effects: Vec<String>,
}

/// Source-qualified selection co-referenced by every instrument — the
/// constellation-wide generalisation of the K9 `SourceQualifiedSelection`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosureSelection {
    pub selection_ref: String,
    pub subject_ref: String,
    #[serde(default)]
    pub coordinate_ref: Option<String>,
    #[serde(default)]
    pub source_ref: Option<String>,
    #[serde(default)]
    pub source_revision: Option<String>,
    #[serde(default)]
    pub disclosure_ref: Option<String>,
    #[serde(default)]
    pub focus_refs: Vec<String>,
    pub reading_ref: String,
    #[serde(default)]
    pub snapshot_revision: Option<String>,
    pub instrument: TechneInstrument,
    #[serde(default)]
    pub agent_session_ref: Option<String>,
    #[serde(default)]
    pub selection_standing: Option<String>,
}

impl DisclosureSelection {
    /// Two instruments are co-referenced when they hold the same subject on
    /// the same reading basis. Views that cannot represent the exact
    /// selection must still preserve this pair rather than substitute an
    /// object.
    pub fn co_referenced(&self, other: &DisclosureSelection) -> bool {
        self.subject_ref == other.subject_ref && self.reading_ref == other.reading_ref
    }
}

/// One cross-instrument transition of the shared selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosureNavigation {
    pub from_instrument: TechneInstrument,
    pub to_instrument: TechneInstrument,
    pub selection_ref: String,
}

/// Ephemeral shared disclosure state over the adapter. Not a canonical
/// domain object; deliberately carries no view/layout/camera/lane fields —
/// presentation state belongs to the surface owner. TB0 (2026-09-16): the
/// session carries the dual-reading state explicitly — active application
/// cut, selected whole, Project/World/Context-Frame focus, the current
/// occasion, scene and reference-frame focus and the Return target — while
/// every identity ref stays native and byte-exact across cuts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosureSession {
    pub contract: String,
    pub session_ref: String,
    pub subject_ref: String,
    pub selection: DisclosureSelection,
    pub instrument: TechneInstrument,
    /// TB0: the active reading of the one M′ field. Consistent with
    /// `instrument`: expressions ↔ 3:3-conjugate; the six deep instruments
    /// ↔ 4:2-deep.
    #[serde(default)]
    pub application_cut: Option<TechneReadingKind>,
    /// TB0: the selected bounded whole when the selection is whole-scoped.
    #[serde(default)]
    pub whole_ref: Option<String>,
    /// TB0: active Project/ProjectCentral focus.
    #[serde(default)]
    pub project_ref: Option<String>,
    /// TB0: active World focus (Central root World).
    #[serde(default)]
    pub world_ref: Option<String>,
    /// TB0: active QL Context Frame focus.
    #[serde(default)]
    pub context_frame_ref: Option<String>,
    /// TB0: the current shared occasion (event/now continuity) that must
    /// survive cut crossings unchanged.
    #[serde(default)]
    pub occasion_ref: Option<String>,
    /// TB0: where attributable Return from this session is addressed.
    #[serde(default)]
    pub return_target_ref: Option<String>,
    #[serde(default)]
    pub reading_ref: Option<String>,
    #[serde(default)]
    pub time_window: Option<DisclosureTimeWindow>,
    #[serde(default)]
    pub spatial_focus_ref: Option<String>,
    /// TB0: the wider situated reference frame in focus (solar/Earth/
    /// geography scale chain) distinct from any single place.
    #[serde(default)]
    pub reference_frame_ref: Option<String>,
    #[serde(default)]
    pub expression_focus_ref: Option<String>,
    /// TB0: focused Expression Scene when the focus is scene-scoped; the
    /// scene ref stays the Expression owner's.
    #[serde(default)]
    pub scene_focus_ref: Option<String>,
    #[serde(default)]
    pub navigation: Vec<DisclosureNavigation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosureTimeWindow {
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
}

impl DisclosureSession {
    pub fn validate(&self) -> Result<(), crate::AdapterError> {
        if self.contract != TECHNE_CONTRACT {
            return Err(crate::AdapterError::InvalidTechneReading(format!(
                "contract tag {:?} is not {TECHNE_CONTRACT}",
                self.contract
            )));
        }
        if self.selection.subject_ref != self.subject_ref {
            return Err(crate::AdapterError::InvalidTechneReading(
                "session subject_ref differs from the selected subject_ref".to_string(),
            ));
        }
        if let Some(cut) = self.application_cut {
            if cut != self.instrument.reading() {
                return Err(crate::AdapterError::InvalidTechneReading(format!(
                    "session application cut {cut:?} is inconsistent with instrument {:?}",
                    self.instrument
                )));
            }
        }
        Ok(())
    }

    /// TB0: the dual-reading crossing — move this session to
    /// `target_instrument`'s application cut over the same subject. Only
    /// disclosure state changes: subject, selection basis, sources,
    /// occasion, Actions and Return target are carried untouched, which is
    /// the cross-cut identity law. Refuses a crossing onto the cut the
    /// session already occupies.
    pub fn cross_cut(
        &mut self,
        target_instrument: TechneInstrument,
    ) -> Result<TechneReadingKind, crate::AdapterError> {
        let target_cut = target_instrument.reading();
        let current_cut = self
            .application_cut
            .unwrap_or_else(|| self.instrument.reading());
        if target_cut == current_cut {
            return Err(crate::AdapterError::InvalidTechneReading(format!(
                "crossing requires the other application cut; {:?} is already {:?}",
                target_instrument, target_cut
            )));
        }
        self.instrument = target_instrument;
        self.application_cut = Some(target_cut);
        Ok(target_cut)
    }
}

/// The shared business/read-model boundary of the constellation. Instrument
/// UIs consume this; they do not invent their own semantic model, and the
/// adapter never becomes a store or a native owner.
pub trait TechneAdapter {
    /// The composed reading for one subject at the adapter's current basis.
    fn reading(&self, subject_ref: &str) -> Result<TechneReading, crate::AdapterError>;

    /// Capability discovery for one subject. Derived from the reading's
    /// disclosure: which instruments this subject actually supports and why.
    fn capabilities(&self, subject_ref: &str) -> Result<TechneDisclosure, crate::AdapterError> {
        Ok(self.reading(subject_ref)?.disclosure)
    }

    /// Resolve the native owner for a mutation request. Returns a route
    /// receipt; execution stays with the owner under its own authority.
    fn route_action(
        &self,
        route: &TechneActionRoute,
        reading: &TechneReading,
    ) -> Result<TechneActionRouteReceipt, crate::AdapterError>;
}
