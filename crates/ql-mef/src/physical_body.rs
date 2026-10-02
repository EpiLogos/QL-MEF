//! Control-thread M3 source/form admission for the existing C++ body owner.
//! A glyph label, pair angle or normalized hinge is not a metric mesh. A
//! provider supplies metric topology with its own standing and revision; the
//! native C++ preparation derives eigenstructure from that physical evidence.
use crate::m3_engine::{M3NodeKind, native_m3_engine};
use crate::m3_source::native_m3_source;
use crate::m3_state::M3State;
use crate::{MCoordinate, MFace};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

pub const CONTRACT: &str = "ql.physical-body/v1";
pub const PREPARATION_SCHEMA: &str = "ql.physical-body-preparation/v1";
pub const MAX_NODES: usize = 32;
pub const MAX_EDGES: usize = 96;
pub const MAX_FORCE_FRAMES: usize = 512;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BodyFamily {
    AxialTruss,
    PrestressedTensionNetwork,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhysicalStanding {
    SourceAuthored,
    Ratified,
    Reference,
    AgentProposed,
    Measured,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalProvenance {
    pub reference: String,
    pub revision: String,
    pub source_ref: String,
    pub standing: PhysicalStanding,
}
impl PhysicalProvenance {
    fn validate(&self) -> Result<(), String> {
        for reference in [&self.reference, &self.revision, &self.source_ref] {
            bounded_ref(reference)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalMaterial {
    pub provenance: PhysicalProvenance,
    pub young_modulus_pa: f64,
    pub density_kg_per_m3: f64,
    pub damping_alpha_per_second: f64,
    pub damping_beta_seconds: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalNode {
    pub identity: u64,
    pub constituent: String,
    pub rest_metres: [f64; 3],
    pub additional_mass_kg: f64,
    pub fixed: [bool; 3],
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalEdge {
    pub first: usize,
    pub second: usize,
    pub section_m2: f64,
    pub prestress_newtons: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetricGeometry {
    pub provenance: PhysicalProvenance,
    pub family: BodyFamily,
    pub nodes: Vec<PhysicalNode>,
    pub edges: Vec<PhysicalEdge>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpatialProjection {
    pub axis: [f64; 3],
    pub node_weights: Vec<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodyPreparationRequest {
    pub expected_m3_generation: u64,
    pub body_revision: u64,
    pub preparation_ref: String,
    pub state_ref: String,
    pub geometry: MetricGeometry,
    pub material: PhysicalMaterial,
    pub sample_rate: u32,
    pub exciter: SpatialProjection,
    pub pickup: SpatialProjection,
    pub pickup_linear_per_metre: f64,
    pub max_force_newtons: f64,
    pub max_impulse_newton_seconds: f64,
    pub max_displacement_metres: f64,
}
/// Private fields make the admitted output immutable. A serialized envelope is
/// evidence to the native host; it is not authentication or mutation authority.
#[derive(Debug, Clone, Serialize)]
pub struct PreparedSourceBody {
    schema: &'static str,
    contract: &'static str,
    event_ref: String,
    subject_ref: String,
    #[serde(serialize_with = "serialize_source_coordinate")]
    source_coordinate: MCoordinate,
    source_revision: String,
    domain_revision: String,
    source_generation: u64,
    address: u8,
    pose_ordinal: u16,
    // Retain the real owner clock and aperture packet; never manufacture a
    // second clock or confuse eighteen M3 apertures with twelve MEF lenses.
    clock: Value,
    aperture: Value,
    form: Value,
    request: BodyPreparationRequest,
    units: Value,
}
impl PreparedSourceBody {
    pub fn request(&self) -> &BodyPreparationRequest {
        &self.request
    }
    pub fn source_coordinate(&self) -> &MCoordinate {
        &self.source_coordinate
    }
    pub fn event_ref(&self) -> &str {
        &self.event_ref
    }
    pub fn subject_ref(&self) -> &str {
        &self.subject_ref
    }
    pub fn source_generation(&self) -> u64 {
        self.source_generation
    }
    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }
    pub fn clock(&self) -> &Value {
        &self.clock
    }
    pub fn aperture(&self) -> &Value {
        &self.aperture
    }
    pub fn form(&self) -> &Value {
        &self.form
    }
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn pose_ordinal(&self) -> u16 {
        self.pose_ordinal
    }
    pub fn validate_observation(
        &self,
        receipt: &BodyObservationReceipt,
        current_sample: u64,
    ) -> Result<(), String> {
        if receipt.contract != CONTRACT
            || receipt.event_ref != self.event_ref
            || receipt.preparation_ref != self.request.preparation_ref
            || receipt.state_ref != self.request.state_ref
            || receipt.body_revision != self.request.body_revision
            || receipt.samples_elapsed != current_sample
        {
            return Err("disconnected or stale physical body observation".into());
        }
        if !receipt.pickup_linear.is_finite()
            || !receipt.mechanical_energy_joules.is_finite()
            || receipt.mechanical_energy_joules < 0.0
        {
            return Err("nonfinite physical body observation".into());
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodyObservationReceipt {
    pub contract: String,
    pub event_ref: String,
    pub preparation_ref: String,
    pub state_ref: String,
    pub body_revision: u64,
    pub samples_elapsed: u64,
    pub pickup_linear: f64,
    pub mechanical_energy_joules: f64,
}
/// Receives the actual M3 producer. The supplied coordinate must be its exact
/// current form constituent with native provenance and declared face. A parent
/// branch, dot/hyphen substitution or unknown descendant is never a fallback.
pub fn prepare_source_body(
    state: &M3State,
    coordinate: MCoordinate,
    request: BodyPreparationRequest,
) -> Result<PreparedSourceBody, String> {
    if request.expected_m3_generation != state.generation() {
        return Err("stale M3 body preparation".into());
    }
    let engine = native_m3_engine();
    let node = engine
        .node(
            M3NodeKind::Codon,
            usize::from(state.fold().codon().address()),
        )
        .ok_or("native M3 form has no source constituent")?;
    let expected =
        crate::m_tree::native_current_m_registry().coordinate(&node.source_ref, coordinate.face)?;
    if coordinate != expected {
        return Err("body coordinate differs from the actual native M3 form".into());
    }
    validate_physical_request(&request)?;
    // A metric provider must keep the exact form target among its constituents.
    if !request
        .geometry
        .nodes
        .iter()
        .any(|node| node.constituent == expected.source_ref)
    {
        return Err("metric body disconnected from the M3 form constituent".into());
    }
    let snapshot = state.snapshot();
    let event_ref = snapshot["identity"]["event_ref"]
        .as_str()
        .ok_or("M3 event identity missing")?
        .to_owned();
    let subject_ref = snapshot["subject_ref"]
        .as_str()
        .ok_or("M3 subject identity missing")?
        .to_owned();
    bounded_ref(&event_ref)?;
    bounded_ref(&subject_ref)?;
    let source = native_m3_source();
    Ok(PreparedSourceBody {
        schema: PREPARATION_SCHEMA,
        contract: CONTRACT,
        event_ref,
        subject_ref,
        source_coordinate: coordinate,
        source_revision: source.source_revision().into(),
        domain_revision: source.revision().into(),
        source_generation: state.generation(),
        address: state.fold().codon().address(),
        pose_ordinal: state.fold().rotational_pose().ordinal() as u16,
        clock: snapshot["clock"].clone(),
        aperture: snapshot["aperture"].clone(),
        form: snapshot["form"].clone(),
        request,
        units: serde_json::json!({"position":"m","displacement":"m","mass":"kg","density":"kg/m^3",
            "elastic_modulus":"Pa","section":"m^2","prestress":"N","excitation":"N","impulse":"N*s",
            "pickup":"linear","pickup_gain":"linear/m","energy":"J","rayleigh_alpha":"s^-1","rayleigh_beta":"s"}),
    })
}

fn bounded_ref(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 2048 || value.chars().any(char::is_control) {
        return Err("bounded printable physical source reference required".into());
    }
    Ok(())
}
fn magnitude(value: f64, low: f64, high: f64, name: &str) -> Result<(), String> {
    if !value.is_finite() || value < low || value > high {
        return Err(format!("invalid physical {name}"));
    }
    Ok(())
}
fn projection(value: &SpatialProjection, nodes: usize) -> Result<(), String> {
    if value.node_weights.len() != nodes {
        return Err("physical projection/node mismatch".into());
    }
    for axis in value.axis {
        magnitude(axis, -1.0, 1.0, "axis")?;
    }
    if (value.axis.iter().map(|x| x * x).sum::<f64>() - 1.0).abs() > 1e-10 {
        return Err("physical axis must be normalized".into());
    }
    for weight in &value.node_weights {
        magnitude(*weight, 0.0, 1.0, "projection weight")?;
    }
    if (value.node_weights.iter().sum::<f64>() - 1.0).abs() > 1e-10 {
        return Err("physical projection weights must sum to one".into());
    }
    Ok(())
}
fn validate_physical_request(request: &BodyPreparationRequest) -> Result<(), String> {
    bounded_ref(&request.preparation_ref)?;
    bounded_ref(&request.state_ref)?;
    request.geometry.provenance.validate()?;
    request.material.provenance.validate()?;
    if request.body_revision > crate::m2_engine::MAX_EXACT_JSON_INTEGER {
        return Err("body revision exceeds exact transport integer".into());
    }
    if !(8000..=192000).contains(&request.sample_rate) {
        return Err("unsupported physical sample rate".into());
    }
    let geometry = &request.geometry;
    if !(2..=MAX_NODES).contains(&geometry.nodes.len())
        || geometry.edges.is_empty()
        || geometry.edges.len() > MAX_EDGES
    {
        return Err("physical preparation resource budget exceeded".into());
    }
    let material = &request.material;
    magnitude(material.young_modulus_pa, 1.0, 1e13, "elastic modulus Pa")?;
    magnitude(material.density_kg_per_m3, 1e-3, 1e6, "density kg/m3")?;
    magnitude(
        material.damping_alpha_per_second,
        0.0,
        1e6,
        "Rayleigh alpha s^-1",
    )?;
    magnitude(material.damping_beta_seconds, 0.0, 1.0, "Rayleigh beta s")?;
    magnitude(
        request.pickup_linear_per_metre,
        -1e9,
        1e9,
        "pickup gain per metre",
    )?;
    magnitude(request.max_force_newtons, 1e-12, 1e9, "Newton force limit")?;
    magnitude(
        request.max_impulse_newton_seconds,
        1e-12,
        1e9,
        "Newton-second impulse limit",
    )?;
    magnitude(
        request.max_displacement_metres,
        1e-12,
        1e3,
        "displacement limit metres",
    )?;
    projection(&request.exciter, geometry.nodes.len())?;
    projection(&request.pickup, geometry.nodes.len())?;
    let registry = crate::m_tree::native_current_m_registry();
    let mut masses = Vec::with_capacity(geometry.nodes.len());
    let mut dofs = 0;
    for (index, node) in geometry.nodes.iter().enumerate() {
        bounded_ref(&node.constituent)?;
        if registry.resolve(&node.constituent).is_none() {
            return Err("unknown exact physical constituent".into());
        }
        if node.identity > crate::m2_engine::MAX_EXACT_JSON_INTEGER
            || (index > 0 && geometry.nodes[index - 1].identity >= node.identity)
        {
            return Err("unordered/duplicate/inexact body node identity".into());
        }
        for position in node.rest_metres {
            magnitude(position, -1e3, 1e3, "geometry metres")?;
        }
        magnitude(node.additional_mass_kg, 0.0, 1e9, "point mass kg")?;
        masses.push(node.additional_mass_kg);
        dofs += node.fixed.iter().filter(|fixed| !**fixed).count();
    }
    if dofs == 0 {
        return Err("body has no unconstrained degrees of freedom".into());
    }
    let mut edges = BTreeSet::new();
    for edge in &geometry.edges {
        if edge.first >= geometry.nodes.len()
            || edge.second >= geometry.nodes.len()
            || edge.first == edge.second
            || !edges.insert((edge.first.min(edge.second), edge.first.max(edge.second)))
        {
            return Err("invalid/duplicate physical edge".into());
        }
        magnitude(edge.section_m2, 1e-12, 1e3, "edge section m2")?;
        magnitude(edge.prestress_newtons, 0.0, 1e9, "prestress Newtons")?;
        if geometry.family == BodyFamily::AxialTruss && edge.prestress_newtons != 0.0 {
            return Err("axial truss cannot use tension-network law".into());
        }
        let length = geometry.nodes[edge.first]
            .rest_metres
            .into_iter()
            .zip(geometry.nodes[edge.second].rest_metres)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        magnitude(length, 1e-6, 1e3, "edge length metres")?;
        let mass = material.density_kg_per_m3 * edge.section_m2 * length / 2.0;
        masses[edge.first] += mass;
        masses[edge.second] += mass;
    }
    for mass in masses {
        magnitude(mass, 1e-12, 1e12, "node mass kg")?;
    }
    Ok(())
}

/// Physical geometry/material updates retain all symbolic source identities;
/// the native runtime decides whether its resident state can be projected.
pub fn prepare_material_update(
    current: &PreparedSourceBody,
    state: &M3State,
    expected_body_revision: u64,
    material: PhysicalMaterial,
    next_body_revision: u64,
    next_preparation_ref: String,
) -> Result<PreparedSourceBody, String> {
    if current.request.body_revision != expected_body_revision
        || next_body_revision <= expected_body_revision
        || current.source_generation != state.generation()
    {
        return Err("stale material preparation".into());
    }
    let snapshot = state.snapshot();
    if snapshot["identity"]["event_ref"].as_str() != Some(current.event_ref.as_str())
        || snapshot["subject_ref"].as_str() != Some(current.subject_ref.as_str())
    {
        return Err("material update disconnected from body event".into());
    }
    let mut request = current.request.clone();
    request.material = material;
    request.body_revision = next_body_revision;
    request.preparation_ref = next_preparation_ref;
    prepare_source_body(state, current.source_coordinate.clone(), request)
}

pub fn source_face(prepared: &PreparedSourceBody) -> MFace {
    prepared.source_coordinate.face
}

fn source_record_wire(record: &crate::m_map::SourceRecordRef) -> Value {
    serde_json::json!({"repository":record.repository,"revision":record.revision,"source_path":record.source_path,
        "git_blob":record.git_blob,"file_sha256":record.file_sha256,"record_class":record.record_class,
        "record_index":record.record_index,"payload_sha256":record.payload_sha256})
}
/// Local canonical projection of the native type; no shared ontology/Serde
/// changes. Every native source field is retained, including literal separator
/// sequence and qualified records. Temporal phase stays in the M3 clock packet.
pub fn source_coordinate_wire(coordinate: &MCoordinate) -> Value {
    serde_json::json!({"source_ref":coordinate.source_ref,"canonical_ref":coordinate.canonical_ref(),
        "root":coordinate.root,"path":coordinate.path,
        "separators":coordinate.separators.iter().map(|s|s.as_char()).collect::<Vec<_>>(),
        "face":coordinate.face.as_str(),"parent_source_ref":coordinate.parent_source_ref,
        "aliases":coordinate.aliases,"provenance":coordinate.provenance.iter().map(source_record_wire).collect::<Vec<_>>(),
        "payloads":coordinate.payloads.iter().map(|p|serde_json::json!({"record":source_record_wire(&p.record),
            "property_keys":p.property_keys})).collect::<Vec<_>>()})
}
fn serialize_source_coordinate<S: serde::Serializer>(
    coordinate: &MCoordinate,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    source_coordinate_wire(coordinate).serialize(serializer)
}
/// Re-admit by resolving the actual current registry and comparing its complete
/// projection. Serialized provenance is retained evidence, never authority.
pub fn readmit_source_coordinate(wire: &Value) -> Result<MCoordinate, String> {
    let reference = wire["source_ref"]
        .as_str()
        .ok_or("source coordinate reference missing")?;
    bounded_ref(reference)?;
    let face = match wire["face"].as_str() {
        Some("bimba") => MFace::Bimba,
        Some("pratibimba") => MFace::Pratibimba,
        _ => return Err("source coordinate face missing/unknown".into()),
    };
    let coordinate = crate::m_tree::native_current_m_registry().coordinate(reference, face)?;
    if source_coordinate_wire(&coordinate) != *wire {
        return Err("serialized coordinate differs from current native source/provenance".into());
    }
    Ok(coordinate)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FormTransitionPolicy {
    ProjectCorrespondingNodes,
    ExplicitReset,
}
#[derive(Debug, Clone, Serialize)]
pub struct PreparedFormTransition {
    pub contract: &'static str,
    pub expected_body_revision: u64,
    pub expected_sample: u64,
    pub policy: FormTransitionPolicy,
    pub before_preparation_ref: String,
    pub after: PreparedSourceBody,
}
/// The native M3 operation must already have occurred. This admits its new
/// source form and supplied metric geometry as an explicit physical transition;
/// it neither selects the symbolic form nor advances another clock.
pub fn prepare_form_transition(
    current: &PreparedSourceBody,
    state: &M3State,
    coordinate: MCoordinate,
    request: BodyPreparationRequest,
    expected_body_revision: u64,
    expected_sample: u64,
    policy: FormTransitionPolicy,
) -> Result<PreparedFormTransition, String> {
    if expected_body_revision != current.request.body_revision
        || request.body_revision <= expected_body_revision
        || state.generation() < current.source_generation
        || request.state_ref != current.request.state_ref
    {
        return Err("stale physical form transition".into());
    }
    let after = prepare_source_body(state, coordinate, request)?;
    if after.event_ref != current.event_ref
        || after.subject_ref != current.subject_ref
        || after.request.sample_rate != current.request.sample_rate
    {
        return Err("physical form transition disconnected from event/state/time".into());
    }
    if policy == FormTransitionPolicy::ProjectCorrespondingNodes {
        let before = &current.request.geometry.nodes;
        let nodes = &after.request.geometry.nodes;
        if before.len() != nodes.len()
            || before
                .iter()
                .zip(nodes)
                .any(|(a, b)| a.identity != b.identity)
        {
            return Err("form projection lacks exact node correspondence".into());
        }
    }
    Ok(PreparedFormTransition {
        contract: CONTRACT,
        expected_body_revision,
        expected_sample,
        policy,
        before_preparation_ref: current.request.preparation_ref.clone(),
        after,
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodalBoundaryMapping {
    /// Index into the actual native quartet, never a new oscillator index.
    pub quartet_index: u8,
    pub node_identities: Vec<u64>,
    /// Complete boundary state at each named node, permitting release as well
    /// as fixation. Multiple roles naming one node must agree exactly.
    pub fixed_axes: [bool; 3],
}
#[derive(Debug, Clone, Serialize)]
pub struct PreparedNodalBoundaryUpdate {
    pub source_reading: crate::m2_vimarsha::VimarshaReading,
    pub mappings: [NodalBoundaryMapping; 4],
    pub transition: PreparedFormTransition,
}
/// Consume the actual joined M2 writer and an explicit metric provider map.
/// Nodal m/n values retain their source standing; they never become Hertz or
/// an invented mesh coordinate. A provider binds each boundary to exact node
/// IDs and physical axes, after which the native body applies constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodalBoundaryUpdateRequest {
    pub mappings: [NodalBoundaryMapping; 4],
    pub expected_body_revision: u64,
    pub expected_sample: u64,
    pub next_body_revision: u64,
    pub next_preparation_ref: String,
}
pub fn prepare_nodal_boundaries(
    current: &PreparedSourceBody,
    state: &M3State,
    basis: &crate::continuous::coupled::CoupledBasis,
    update: NodalBoundaryUpdateRequest,
) -> Result<PreparedNodalBoundaryUpdate, String> {
    let NodalBoundaryUpdateRequest {
        mappings,
        expected_body_revision,
        expected_sample,
        next_body_revision,
        next_preparation_ref,
    } = update;
    let replay = basis.input.compose()?;
    if replay.m1 != basis.m1
        || replay.m2 != basis.m2
        || replay.m3 != basis.m3
        || replay.derivation != basis.derivation
        || replay.m3_receipts != basis.m3_receipts
        || serde_json::to_value(&replay.m2_input).map_err(|e| e.to_string())?
            != serde_json::to_value(&basis.m2_input).map_err(|e| e.to_string())?
        || basis.m3 != state.snapshot()
        || basis.m2_input.stamp.identity.event_ref != current.event_ref
    {
        return Err(
            "nodal constraints disconnected from actual native M1/M2/M3 composition".into(),
        );
    }
    let reading: crate::m2_vimarsha::VimarshaReading =
        serde_json::from_value(basis.m2["vimarsha"]["reading"].clone())
            .map_err(|e| format!("native nodal quartet unavailable: {e}"))?;
    if reading.policy_ref != crate::m2_vimarsha::POLICY
        || reading.source_ref != crate::m2_vimarsha::SOURCE
        || reading.source_coordinate != "#2-1"
    {
        return Err("unbacked Vimarśā boundary producer".into());
    }
    let mut request = current.request.clone();
    request.expected_m3_generation = state.generation();
    request.body_revision = next_body_revision;
    request.preparation_ref = next_preparation_ref;
    request.geometry.provenance.revision = format!(
        "{}:nodal:{next_body_revision}",
        request.geometry.provenance.revision
    );
    let mut roles = BTreeSet::new();
    let mut node_boundaries = std::collections::BTreeMap::new();
    for mapping in &mappings {
        if mapping.quartet_index >= 4
            || !roles.insert(mapping.quartet_index)
            || mapping.node_identities.is_empty()
            || mapping.node_identities.len() > MAX_NODES
        {
            return Err("incomplete/duplicate bounded nodal boundary map".into());
        }
        let mut ids = BTreeSet::new();
        for identity in &mapping.node_identities {
            if !ids.insert(*identity) {
                return Err("duplicate nodal boundary node".into());
            }
            if let Some(previous) = node_boundaries.insert(*identity, mapping.fixed_axes)
                && previous != mapping.fixed_axes
            {
                return Err("conflicting physical boundary roles at one node".into());
            }
            let node = request
                .geometry
                .nodes
                .iter_mut()
                .find(|node| node.identity == *identity)
                .ok_or("nodal boundary does not name an existing metric node")?;
            node.fixed = mapping.fixed_axes;
        }
    }
    let transition = prepare_form_transition(
        current,
        state,
        current.source_coordinate.clone(),
        request,
        expected_body_revision,
        expected_sample,
        FormTransitionPolicy::ProjectCorrespondingNodes,
    )?;
    Ok(PreparedNodalBoundaryUpdate {
        source_reading: reading,
        mappings,
        transition,
    })
}
