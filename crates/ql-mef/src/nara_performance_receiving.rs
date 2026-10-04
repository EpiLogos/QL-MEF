//! Source-qualified Nara receiving preparation for the existing A/P owner.
//! A definition is immutable source evidence, never host consent or a second
//! engine. Nine independent planetary sources are distinct from M2's octet.
//! Instrument calibration is explicit; anatomy cannot supply physical units.
use crate::m_tree::{MRegistry, MTreeId, native_current_m_registry};
use crate::nara::{
    EventBasisRefs, NativeEventGenerations, current, intake::IdentityProfile, intake_composition,
    replay::NaraOccasion,
};
use crate::performance_audio::PreparedPerformanceBinding;
use crate::physical_body::{PhysicalProvenance, SpatialProjection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const CONTRACT: &str = "ql.nara-performance-receiving/v1";
const MAX_INPUT_BYTES: usize = 8 * 1024 * 1024;
const SOURCE_COUNT: usize = 10;
const DRIVER_COUNT: usize = 9;
const CENTRE_COUNT: usize = 7;

fn text(s: &str) -> Result<(), String> {
    if s.is_empty() || s.trim() != s || s.len() > 4096 || s.chars().any(char::is_control) {
        return Err("bounded native receiving reference required".into());
    }
    Ok(())
}
fn digest<T: Serialize>(value: &T) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("receiving source exceeds existing expanded document bound".into());
    }
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}
fn number(v: &Value, name: &str) -> Result<f64, String> {
    v.get(name)
        .and_then(Value::as_f64)
        .filter(|n| n.is_finite())
        .ok_or_else(|| format!("native receiving {name} absent or nonfinite"))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub reference: String,
    pub revision: String,
}
impl Reference {
    fn validate(&self) -> Result<(), String> {
        text(&self.reference)?;
        text(&self.revision)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextKind {
    World,
    Personal,
    Shared,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReceivingContext {
    pub kind: ContextKind,
    pub context: Reference,
    pub receiver: Reference,
    pub protected_state: Option<Reference>,
    pub consent: Option<Reference>,
    pub original_occasion: Option<Reference>,
    pub private: bool,
}
impl ReceivingContext {
    fn validate(&self, occasion: Option<&NaraOccasion>) -> Result<(), String> {
        self.context.validate()?;
        self.receiver.validate()?;
        for r in [
            &self.protected_state,
            &self.consent,
            &self.original_occasion,
        ]
        .into_iter()
        .flatten()
        {
            r.validate()?;
        }
        match self.kind {
            ContextKind::World => {
                if self.private
                    || self.protected_state.is_some()
                    || self.consent.is_some()
                    || self.original_occasion.is_some()
                    || occasion.is_some()
                {
                    return Err(
                        "ordinary public World cannot carry protected personal context".into(),
                    );
                }
            }
            ContextKind::Personal | ContextKind::Shared => {
                let o =
                    occasion.ok_or("personal receiving requires the original native occasion")?;
                o.validate()?;
                let p = self
                    .protected_state
                    .as_ref()
                    .ok_or("personal receiving requires protected custody")?;
                if !self.private
                    || p.reference != o.protected_state_ref.ref_id
                    || p.revision != o.protected_state_ref.revision
                    || self
                        .original_occasion
                        .as_ref()
                        .map(|r| r.reference.as_str())
                        != Some(o.occasion_ref.as_str())
                {
                    return Err(
                        "receiving context detached from protected original occasion".into(),
                    );
                }
                if self.kind == ContextKind::Shared && self.consent.is_none() {
                    return Err(
                        "shared receiving requires exact opt-in consent; labels grant no authority"
                            .into(),
                    );
                }
            }
        }
        Ok(())
    }
}

/// Explicit calibrated mapping into one actually prepared metric body.
/// The calibration's standing is retained, including Reference/AgentProposed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentreCalibration {
    pub ordinal: u8,
    pub projection: SpatialProjection,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReceivingCalibration {
    pub provenance: PhysicalProvenance,
    pub preparation: Reference,
    pub state: Reference,
    /// Maximum source excitation before original-denominator weighting, in N.
    /// This is an explicit instrument policy, never a therapeutic coefficient.
    pub source_force_newtons: f64,
    pub centres: [CentreCalibration; CENTRE_COUNT],
}
impl ReceivingCalibration {
    fn validate(&self, prepared: &PreparedPerformanceBinding) -> Result<(), String> {
        for s in [
            &self.provenance.reference,
            &self.provenance.revision,
            &self.provenance.source_ref,
        ] {
            text(s)?;
        }
        self.preparation.validate()?;
        self.state.validate()?;
        let r = prepared.physical_body().request();
        if self.preparation.reference != r.preparation_ref
            || self.state.reference != r.state_ref
            || self.preparation.revision != r.body_revision.to_string()
            || self.state.revision != r.body_revision.to_string()
        {
            return Err("receiving calibration targets another prepared body/revision".into());
        }
        if !self.source_force_newtons.is_finite()
            || self.source_force_newtons < 0.0
            || self.source_force_newtons > r.max_force_newtons
        {
            return Err("receiving source force exceeds admitted Newton bound".into());
        }
        for (i, c) in self.centres.iter().enumerate() {
            if usize::from(c.ordinal) != i
                || c.projection.node_weights.len() != r.geometry.nodes.len()
            {
                return Err(
                    "receiving needs seven ordered projections over the actual metric nodes".into(),
                );
            }
            let axis = c.projection.axis;
            let norm = axis.iter().map(|n| n * n).sum::<f64>();
            if !axis.iter().all(|n| n.is_finite())
                || (norm - 1.0).abs() > 1e-12
                || !c
                    .projection
                    .node_weights
                    .iter()
                    .all(|n| n.is_finite() && *n >= 0.0)
                || (c.projection.node_weights.iter().sum::<f64>() - 1.0).abs() > 1e-12
            {
                return Err("receiving projection must have a unit axis and normalized nonnegative node weights".into());
            }
            let excites_free_dof =
                c.projection
                    .node_weights
                    .iter()
                    .zip(&r.geometry.nodes)
                    .any(|(w, n)| {
                        *w > 0.0
                            && axis
                                .iter()
                                .enumerate()
                                .any(|(a, x)| x.abs() > 1e-12 && !n.fixed[a])
                    });
            if !excites_free_dof {
                return Err(
                    "receiving projection is disconnected from every free body degree".into(),
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceWitness {
    pub coordinate: String,
    pub node_id: MTreeId,
    pub payload_sha256: Vec<String>,
    pub relations: Vec<String>,
}
fn witness(registry: &MRegistry, coordinate: &str) -> Result<SourceWitness, String> {
    let node = registry
        .resolve(coordinate)
        .ok_or_else(|| format!("receiving source coordinate lost: {coordinate}"))?;
    let mut payloads = node
        .records
        .iter()
        .map(|i| {
            registry
                .manifest()
                .records
                .get(*i)
                .map(|r| r.payload_sha256.clone())
                .ok_or("receiving source record lost".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    payloads.sort();
    payloads.dedup();
    if payloads.is_empty() {
        return Err("receiving source has no original property witness".into());
    }
    let mut relations = registry
        .relations_for(node.id)
        .map(|r| r.relation_ref.clone())
        .collect::<Vec<_>>();
    relations.sort();
    relations.dedup();
    Ok(SourceWitness {
        coordinate: coordinate.into(),
        node_id: node.id,
        payload_sha256: payloads,
        relations,
    })
}
#[derive(Debug, Clone, Serialize)]
pub struct SourceDriver {
    pub driver_ref: String,
    pub native_planet_id: u8,
    pub planet_coordinate: String,
    pub receiving_centre_ordinal: u8,
    pub hertz: f64,
    pub weighted_contribution: f64,
    pub original_denominator_share: f64,
    /// Exact original weighted contributions in hundredths of the retained
    /// model unit (1.20/1.10/0.90/0.85/1.00 dignity factors). Never renormalized
    /// over the nine routed drivers. Wire uses canonical decimal integers.
    pub share_numerator: String,
    pub share_denominator: String,
    pub elemental_component: u8,
    pub relation_refs: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct CentreBinding {
    pub ordinal: u8,
    pub source: SourceWitness,
    pub anatomy: Value,
    pub original_denominator_share: f64,
    pub driver_ids: Vec<u8>,
    pub projection: Option<SpatialProjection>,
}

/// Private native-produced output. Persist through the existing Scene/Edition
/// asset owner, preserving its context custody; serialized bytes are evidence.
#[derive(Debug, Clone, Serialize)]
pub struct ReceivingDefinition {
    schema: &'static str,
    context: ReceivingContext,
    event: EventBasisRefs,
    native_generations: NativeEventGenerations,
    m2_generation: u64,
    m3_generation: u64,
    occurrence_unix_ms: u64,
    receipt_unix_ms: u64,
    registry_revision: String,
    source_revision: String,
    branch_sources: Vec<SourceWitness>,
    original_occasion: Option<NaraOccasion>,
    identity_digest: Option<String>,
    identity_revision: Option<String>,
    current_digest: Option<String>,
    current_snapshot_ref: Option<String>,
    current_availability: String,
    original_total: Option<f64>,
    source_policy: Option<Reference>,
    original_elemental_balance: Option<Value>,
    uranus_residual: Option<Value>,
    earth: Option<SourceWitness>,
    drivers: Vec<SourceDriver>,
    centres: Vec<CentreBinding>,
    calibration: Option<ReceivingCalibration>,
    /// Common body custody applies to neutral World as well as calibrated
    /// personal routes. M3 source generations alone do not identify material.
    body_revision: String,
    preparation: Reference,
    state: Reference,
    prepared_binding_digest: String,
    source_instance: String,
    native_node_ids: Vec<String>,
    content_digest: String,
}
impl ReceivingDefinition {
    pub fn drivers(&self) -> &[SourceDriver] {
        &self.drivers
    }
    pub fn centres(&self) -> &[CentreBinding] {
        &self.centres
    }
    pub fn content_digest(&self) -> &str {
        &self.content_digest
    }
    pub fn snapshot(&self) -> Result<Value, String> {
        serde_json::to_value(self).map_err(|e| e.to_string())
    }
    pub fn public_snapshot(&self) -> Result<Value, String> {
        if self.context.kind != ContextKind::World || self.context.private {
            return Err(
                "protected receiving evidence cannot enter a public receipt/preset/export".into(),
            );
        }
        self.snapshot()
    }
    /// Verify source and every retained typed edge against an actual owner
    /// registry readback. An older source lock or equal ordinal is insufficient.
    pub fn validate_source_registry(&self, registry: &MRegistry) -> Result<(), String> {
        if registry.manifest().registry_revision != self.registry_revision
            || registry.manifest().source_revision != self.source_revision
        {
            return Err("receiving registry/source revision drift".into());
        }
        for s in self
            .branch_sources
            .iter()
            .chain(self.centres.iter().map(|c| &c.source))
        {
            if serde_json::to_value(witness(registry, &s.coordinate)?).map_err(|e| e.to_string())?
                != serde_json::to_value(s).map_err(|e| e.to_string())?
            {
                return Err("receiving source property/typed edge lost or changed".into());
            }
        }
        if let Some(earth) = &self.earth {
            let actual = witness(registry, &earth.coordinate)?;
            if actual.coordinate != earth.coordinate
                || actual.node_id != earth.node_id
                || actual.payload_sha256 != earth.payload_sha256
            {
                return Err(
                    "distinct Earth source property/native identity lost or changed".into(),
                );
            }
        }
        for r in self
            .drivers
            .iter()
            .flat_map(|d| &d.relation_refs)
            .chain(self.earth.iter().flat_map(|e| &e.relations))
        {
            let retained = native_current_m_registry()
                .manifest()
                .relations
                .iter()
                .find(|a| a.relation_ref == *r)
                .ok_or("receiving original edge unavailable")?;
            let actual = registry
                .manifest()
                .relations
                .iter()
                .find(|a| a.relation_ref == *r)
                .ok_or("receiving required native edge lost")?;
            if serde_json::to_value(retained).map_err(|e| e.to_string())?
                != serde_json::to_value(actual).map_err(|e| e.to_string())?
            {
                return Err("receiving required edge type/recipient changed".into());
            }
        }
        Ok(())
    }
    /// Recompile against original native inputs and compare every determinant.
    /// Actual host admission and A/P rendering are additional required joins.
    pub fn validate_sources(&self, input: ReceivingPreparation<'_>) -> Result<(), String> {
        let rebuilt = prepare_native_receiving(input)?;
        if rebuilt.snapshot()? != self.snapshot()? {
            return Err(
                "receiving source/context/body drift; prior immutable definition retained".into(),
            );
        }
        Ok(())
    }
    /// One bounded operation for the existing management owner. A single-force
    /// prepared body cannot truthfully accept seven independent projections.
    /// Native capability negotiation occurs in the actual A/P owner, not here.
    pub fn native_operation(&self, native_cursor: u64) -> Result<NativeReceivingOperation, String> {
        if self.context.kind == ContextKind::World {
            return Ok(NativeReceivingOperation {
                schema: CONTRACT.into(),
                definition_digest: self.content_digest.clone(),
                context: self.context.clone(),
                native_sample: native_cursor.to_string(),
                event: self.event.clone(),
                m2_generation: self.m2_generation.to_string(),
                m3_generation: self.m3_generation.to_string(),
                native_generations: self.native_generations.clone(),
                source_policy: None,
                source_instance: self.source_instance.clone(),
                body_revision: Some(self.body_revision.clone()),
                preparation: Some(self.preparation.clone()),
                state: Some(self.state.clone()),
                calibration: None,
                sources: vec![],
                projections: vec![],
            });
        }
        if self.current_availability != "available" {
            return Err("receiving current provider unavailable; retained evidence grants no live admission".into());
        }
        let cal = self
            .calibration
            .as_ref()
            .ok_or("source-defined-physical-calibration-unavailable")?;
        let sources = self
            .drivers
            .iter()
            .map(|d| NativeExcitationTarget {
                driver_ref: d.driver_ref.clone(),
                native_planet_id: d.native_planet_id,
                centre_ordinal: d.receiving_centre_ordinal,
                hertz: d.hertz,
                source_ref: d.planet_coordinate.clone(),
                target_ref: format!("{}#driver/{}", self.content_digest, d.native_planet_id),
                share_numerator: d.share_numerator.clone(),
                share_denominator: d.share_denominator.clone(),
                projection_ref: format!(
                    "{}#centre/{}",
                    cal.provenance.reference, d.receiving_centre_ordinal
                ),
                calibration_ref: cal.provenance.reference.clone(),
                original_denominator_share: d.original_denominator_share,
                peak_force_newtons: cal.source_force_newtons * d.original_denominator_share,
                relation_refs: d.relation_refs.clone(),
            })
            .collect();
        Ok(NativeReceivingOperation {
            schema: CONTRACT.into(),
            definition_digest: self.content_digest.clone(),
            context: self.context.clone(),
            native_sample: native_cursor.to_string(),
            event: self.event.clone(),
            m2_generation: self.m2_generation.to_string(),
            m3_generation: self.m3_generation.to_string(),
            native_generations: self.native_generations.clone(),
            source_policy: self.source_policy.clone(),
            source_instance: self.source_instance.clone(),
            body_revision: Some(cal.preparation.revision.clone()),
            preparation: Some(cal.preparation.clone()),
            state: Some(cal.state.clone()),
            calibration: Some(cal.provenance.clone()),
            sources,
            projections: cal
                .centres
                .iter()
                .map(|c| NativeCentreProjection {
                    projection_ref: format!("{}#centre/{}", cal.provenance.reference, c.ordinal),
                    centre_ordinal: c.ordinal,
                    native_node_ids: self.native_node_ids.clone(),
                    weights: c.projection.node_weights.clone(),
                    metric_axis: c.projection.axis,
                    source_force_newtons: cal.source_force_newtons,
                    calibration: cal.provenance.clone(),
                })
                .collect(),
        })
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct NativeExcitationTarget {
    pub driver_ref: String,
    pub native_planet_id: u8,
    pub centre_ordinal: u8,
    pub hertz: f64,
    pub source_ref: String,
    pub target_ref: String,
    pub share_numerator: String,
    pub share_denominator: String,
    pub projection_ref: String,
    pub calibration_ref: String,
    pub original_denominator_share: f64,
    pub peak_force_newtons: f64,
    pub relation_refs: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct NativeCentreProjection {
    pub projection_ref: String,
    pub centre_ordinal: u8,
    pub native_node_ids: Vec<String>,
    pub weights: Vec<f64>,
    pub metric_axis: [f64; 3],
    pub source_force_newtons: f64,
    pub calibration: PhysicalProvenance,
}
#[derive(Debug, Clone, Serialize)]
pub struct NativeReceivingOperation {
    pub schema: String,
    pub definition_digest: String,
    pub context: ReceivingContext,
    /// Native A/P cursor, distinct from authored transport/PPQ or host time.
    pub native_sample: String,
    pub event: EventBasisRefs,
    pub native_generations: NativeEventGenerations,
    pub source_policy: Option<Reference>,
    pub m2_generation: String,
    pub m3_generation: String,
    pub source_instance: String,
    pub body_revision: Option<String>,
    pub preparation: Option<Reference>,
    pub state: Option<Reference>,
    pub calibration: Option<PhysicalProvenance>,
    pub sources: Vec<NativeExcitationTarget>,
    pub projections: Vec<NativeCentreProjection>,
}
impl NativeReceivingOperation {
    /// Existing P's single scalar port is explicitly insufficient. This check
    /// prevents a source/storage increment masquerading as the full consumer.
    pub fn validate_single_projection_port(&self) -> Result<(), String> {
        if !self.sources.is_empty() || !self.projections.is_empty() {
            return Err("distinct-native-force-projections-unavailable".into());
        }
        Ok(())
    }
}

pub struct ReceivingPreparation<'a> {
    pub prepared: &'a PreparedPerformanceBinding,
    pub context: ReceivingContext,
    /// Existing native identity reading, with its original natal receipt.
    pub identity: Option<&'a Value>,
    /// Existing native dated current. Absence remains retained/unavailable.
    pub current: Option<&'a Value>,
    pub original_occasion: Option<&'a NaraOccasion>,
    pub calibration: Option<&'a ReceivingCalibration>,
}
pub fn prepare_native_receiving(
    input: ReceivingPreparation<'_>,
) -> Result<ReceivingDefinition, String> {
    prepare_against_registry(input, native_current_m_registry())
}
fn prepare_against_registry(
    input: ReceivingPreparation<'_>,
    registry: &MRegistry,
) -> Result<ReceivingDefinition, String> {
    input.context.validate(input.original_occasion)?;
    let prepared = input.prepared;
    prepared.validate_native_consumers(prepared.native_basis(), prepared.physical_body())?;
    let event = EventBasisRefs::from_basis(prepared.native_basis())?;
    let body = prepared.physical_body();
    if body.event_ref() != event.event_ref || body.subject_ref() != event.subject_ref {
        return Err("receiving event disconnected from actual audio/physical body".into());
    }
    let score = &prepared.native_basis().m3;
    let mut out = ReceivingDefinition {
        schema: CONTRACT,
        context: input.context.clone(),
        event,
        native_generations: EventBasisRefs::native_generations(prepared.native_basis())?,
        m2_generation: prepared
            .native_basis()
            .m2_input
            .stamp
            .identity
            .profile_generation,
        m3_generation: body.source_generation(),
        occurrence_unix_ms: score["occurrence_unix_ms"]
            .as_u64()
            .ok_or("M3 occurrence unavailable")?,
        receipt_unix_ms: score["receipt_unix_ms"]
            .as_u64()
            .ok_or("M3 receipt unavailable")?,
        registry_revision: registry.manifest().registry_revision.clone(),
        source_revision: registry.manifest().source_revision.clone(),
        branch_sources: vec![],
        original_occasion: input.original_occasion.cloned(),
        identity_digest: None,
        identity_revision: None,
        current_digest: None,
        current_snapshot_ref: None,
        current_availability: "world-unmodified".into(),
        original_total: None,
        source_policy: None,
        original_elemental_balance: None,
        uranus_residual: None,
        earth: None,
        drivers: vec![],
        centres: vec![],
        calibration: input.calibration.cloned(),
        body_revision: body.request().body_revision.to_string(),
        preparation: Reference {
            reference: body.request().preparation_ref.clone(),
            revision: body.request().body_revision.to_string(),
        },
        state: Reference {
            reference: body.request().state_ref.clone(),
            revision: body.request().body_revision.to_string(),
        },
        prepared_binding_digest: digest(prepared)?,
        content_digest: String::new(),
        source_instance: prepared.determination()["identity"]["instance"]
            .as_str()
            .ok_or("audio source instance absent")?
            .into(),
        native_node_ids: body
            .request()
            .geometry
            .nodes
            .iter()
            .map(|n| n.identity.to_string())
            .collect(),
    };
    for coordinate in ["#4.0", "#4.1", "#4.2", "#4.3", "#4.4", "#4.5"] {
        out.branch_sources.push(witness(registry, coordinate)?);
    }
    if input.context.kind == ContextKind::World {
        if input.identity.is_some() || input.current.is_some() || input.calibration.is_some() {
            return Err(
                "ordinary World cannot acquire a hidden personal identity/current/calibration"
                    .into(),
            );
        }
    } else {
        let identity = input
            .identity
            .ok_or("receiving identity source unavailable")?;
        digest(identity)?;
        let profile: IdentityProfile =
            serde_json::from_value(identity["profile"].clone()).map_err(|e| e.to_string())?;
        let natal = identity
            .get("natal")
            .filter(|n| !n.is_null())
            .ok_or("receiving natal source unavailable")?;
        if profile.inspect(Some(natal))? != *identity || profile.person_ref != out.event.subject_ref
        {
            return Err("receiving identity differs from original source or native subject".into());
        }
        let occasion = input
            .original_occasion
            .ok_or("receiving original occasion absent")?;
        if occasion.event != out.event
            || occasion.identity_revision
                != identity["input_revision"]
                    .as_str()
                    .ok_or("identity revision absent")?
        {
            return Err("receiving lost original event/generation/identity occasion".into());
        }
        // Dot nesting and the final typed personal reflection-space edge are
        // required independently of the coordinate's parent metadata.
        for (from, to, kind) in [
            ("#4", "#4.4", "HAS_INTERNAL_COMPONENT"),
            ("#4.4", "#4.4.4", "HAS_INTERNAL_COMPONENT"),
            ("#4.4.4", "#4.4.4.4", "CONTAINS_REFLECTION_SPACE"),
        ] {
            let a = registry
                .resolve(from)
                .ok_or("personal source ancestor unavailable")?;
            let b = registry
                .resolve(to)
                .ok_or("personal source descendant unavailable")?;
            if !registry
                .relations_for(a.id)
                .any(|r| r.from_id == Some(a.id) && r.to_id == Some(b.id) && r.source_kind == kind)
            {
                return Err(format!(
                    "required personal receiving edge lost: {from} {kind} {to}"
                ));
            }
        }
        out.branch_sources.push(witness(registry, "#4.4.4.4")?);
        out.identity_digest = Some(digest(identity)?);
        out.identity_revision = Some(
            identity["input_revision"]
                .as_str()
                .ok_or("identity revision absent")?
                .into(),
        );
        if let Some(c) = input.current {
            digest(c)?;
            let transit = &c["transit"];
            let activity = c.get("activity").filter(|a| !a.is_null());
            if current::personal_current_with_activity(identity, transit, activity)? != *c {
                return Err("receiving current differs from actual dated native derivation".into());
            }
            out.current_digest = Some(digest(c)?);
            out.current_snapshot_ref = c["snapshot_ref"].as_str().map(str::to_owned);
            out.current_availability = if c["baseline_available"] == true {
                "available"
            } else {
                "provider-or-identity-current-unavailable"
            }
            .into();
        } else {
            out.current_availability = "provider-current-unavailable".into();
        }
        let reading = intake_composition::natal_composition(natal)?;
        if reading != identity["natal_composition"] {
            return Err("receiving natal reading lost its native source derivation".into());
        }
        out.source_policy = Some(Reference {
            reference: reading["policy"]
                .as_str()
                .ok_or("original receiving policy absent")?
                .into(),
            revision: reading["policy_source"]["revision"]
                .as_str()
                .ok_or("original receiving policy revision absent")?
                .into(),
        });
        let contributions = reading["planetary_contributions"]
            .as_array()
            .ok_or("ten native contributions absent")?;
        if contributions.len() != SOURCE_COUNT {
            return Err("receiving denominator requires all ten original contributions".into());
        }
        let total = contributions
            .iter()
            .map(|p| number(p, "weighted_contribution"))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum::<f64>();
        if !total.is_finite() || total <= 0.0 {
            return Err("receiving all-source denominator invalid".into());
        }
        let exact_weights = contributions
            .iter()
            .map(|p| {
                let base = number(p, "keplerian_weight")?;
                if base <= 0.0 || base.fract() != 0.0 || base > f64::from(u32::MAX) {
                    return Err("original integer Keplerian coefficient unavailable".to_string());
                }
                let base = base as u64;
                let dignity = match p["dignity"].as_str() {
                    Some("domicile") => 120,
                    Some("exaltation") => 110,
                    Some("detriment") => 90,
                    Some("fall") => 85,
                    Some("unmodified") => 100,
                    _ => return Err("original dignity multiplier policy unavailable".to_string()),
                };
                base.checked_mul(dignity)
                    .ok_or("original weighted coefficient overflow".to_string())
            })
            .collect::<Result<Vec<u64>, String>>()?;
        let exact_total = exact_weights.iter().try_fold(0u64, |a, b| {
            a.checked_add(*b)
                .ok_or("original denominator overflow".to_string())
        })?;
        for (p, exact) in contributions.iter().zip(&exact_weights) {
            if (number(p, "weighted_contribution")? - *exact as f64 / 100.0).abs() > total * 1e-12 {
                return Err(
                    "native original weight differs from source exact dignity policy".into(),
                );
            }
        }
        out.original_total = Some(total);
        out.original_elemental_balance = Some(reading["elemental_balance_l1"].clone());
        out.uranus_residual = Some(reading["presentation_partition"]["unrouted"].clone());
        let snapshot = reading["snapshot_ref"]
            .as_str()
            .ok_or("original natal snapshot absent")?;
        let mut ids = BTreeSet::new();
        for p in contributions {
            let id = p["native_planet_id"]
                .as_u64()
                .filter(|i| *i < 10)
                .ok_or("invalid original native planet")? as u8;
            if !ids.insert(id) {
                return Err("duplicate original planet contribution".into());
            }
            let route = crate::m2::planet_chakra_route(usize::from(id))?;
            let Some(route) = route else {
                if id != 7 || !p["receiving_centre_ordinal"].is_null() {
                    return Err("source receiving route unexpectedly unavailable".into());
                }
                continue;
            };
            let ordinal = route.chakra_index - 1;
            if p["receiving_centre_ordinal"] != ordinal {
                return Err("source receiving centre changed".into());
            }
            let hertz = number(p, "native_cousto_frequency_hz")?;
            if !(1.0..=20000.0).contains(&hertz) {
                return Err("source driver frequency outside admitted audio band".into());
            }
            let mut refs = Vec::new();
            for required in &route.relations {
                let actual = registry
                    .relation(required.id)
                    .ok_or("required planetary receiving edge lost")?;
                if serde_json::to_value(actual).map_err(|e| e.to_string())?
                    != serde_json::to_value(required).map_err(|e| e.to_string())?
                {
                    return Err(
                        "planetary receiving edge changed native identity/type/destination".into(),
                    );
                }
                refs.push(actual.relation_ref.clone());
            }
            refs.sort();
            out.drivers.push(SourceDriver {
                driver_ref: serde_json::to_string(&(snapshot, id)).map_err(|e| e.to_string())?,
                native_planet_id: id,
                planet_coordinate: route.planet_coordinate,
                receiving_centre_ordinal: ordinal,
                hertz,
                weighted_contribution: number(p, "weighted_contribution")?,
                original_denominator_share: number(p, "weighted_contribution")? / total,
                share_numerator: exact_weights[usize::from(id)].to_string(),
                share_denominator: exact_total.to_string(),
                elemental_component: p["efwa_component"]
                    .as_u64()
                    .filter(|i| *i < 4)
                    .ok_or("source EFWA component unavailable")?
                    as u8,
                relation_refs: refs,
            });
        }
        if out.drivers.len() != DRIVER_COUNT {
            return Err(
                "receiving requires nine distinct sources; octet/average substitution refused"
                    .into(),
            );
        }
        if let Some(cal) = input.calibration {
            cal.validate(prepared)?;
        }
        let centres = reading["centre_evidence"]
            .as_array()
            .ok_or("seven original centres absent")?;
        if centres.len() != CENTRE_COUNT {
            return Err("receiving requires all seven centres".into());
        }
        for (ordinal, c) in centres.iter().enumerate() {
            let drivers = out
                .drivers
                .iter()
                .filter(|d| usize::from(d.receiving_centre_ordinal) == ordinal)
                .collect::<Vec<_>>();
            out.centres.push(CentreBinding {
                ordinal: ordinal as u8,
                source: witness(registry, &format!("#2-5-0/1-{}", ordinal + 1))?,
                anatomy: c["body"].clone(),
                original_denominator_share: drivers
                    .iter()
                    .map(|d| d.original_denominator_share)
                    .sum(),
                driver_ids: drivers.iter().map(|d| d.native_planet_id).collect(),
                projection: input
                    .calibration
                    .map(|cal| cal.centres[ordinal].projection.clone()),
            });
        }
        let mut earth = witness(registry, "#2-5-0/1-0")?;
        earth.relations = crate::nara::domain::operations::body::earth_grounding_refs()?;
        for r in &earth.relations {
            if !registry
                .manifest()
                .relations
                .iter()
                .any(|actual| actual.relation_ref == *r)
            {
                return Err("distinct Earth grounding edge lost".into());
            }
        }
        out.earth = Some(earth);
    }
    out.content_digest = digest(&out)?;
    Ok(out)
}
