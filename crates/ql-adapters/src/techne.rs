//! The shared L5 Technē contract: `ql.techne/v1`.
//!
//! One language binding of the language-neutral contract pinned in
//! `schemas/techne/`. The adapter composes attributable readings over native
//! owners; it is not a store, never rewrites native identity, and never
//! executes a mutation itself — native Actions route to their owner under the
//! owner's authority. Canonical text: `docs/L5-TECHNE-INSTRUMENT-WAYFINDER.md`
//! §1–§2 and §18 (T0).

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
/// provider's own; origins are preserved, not relabelled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WholeRelation {
    pub relation: String,
    pub from_ref: String,
    pub to_ref: String,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    pub origin_ref: Option<String>,
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
    #[serde(default)]
    pub identity: Option<PlaceIdentity>,
    #[serde(default)]
    pub geometry: Option<PlaceGeometry>,
    pub precision: PlacePrecision,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechneDisclosure {
    pub instruments: Vec<InstrumentDisclosure>,
    #[serde(default)]
    pub degraded: Vec<DegradedDisclosure>,
    #[serde(default)]
    pub suggestions: Vec<DisclosureSuggestion>,
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
/// presentation state belongs to the surface owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosureSession {
    pub contract: String,
    pub session_ref: String,
    pub subject_ref: String,
    pub selection: DisclosureSelection,
    pub instrument: TechneInstrument,
    #[serde(default)]
    pub reading_ref: Option<String>,
    #[serde(default)]
    pub time_window: Option<DisclosureTimeWindow>,
    #[serde(default)]
    pub spatial_focus_ref: Option<String>,
    #[serde(default)]
    pub expression_focus_ref: Option<String>,
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
        Ok(())
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
