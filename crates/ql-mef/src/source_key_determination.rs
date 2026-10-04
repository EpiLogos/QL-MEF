//! Explicit source-degree reduction over the existing twelve-address field.
//!
//! These operations run on the control owner. They do not allocate targets in
//! an audio callback, infer missing pitches, change a physical eigenbasis, or
//! authorize a retained JSON receipt. A owns the applied native event.
use std::sync::Arc;

use ql_core::{QlCoordinate, QlFace, QlPosition};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::continuous::coupled::CoupledBasis;
use crate::m2_tuning_sources::{
    CollectionStanding, SourcePitchAvailability, TuningSourceCollection, TuningSourceEntry,
};
use crate::music_determination::{
    ExactRatio, Fundamental, MusicalDetermination, NoteTarget, PositionalCollection,
    TuningProvenance,
};
use crate::{MCoordinate, MFace, pitch_at_lens};

pub const SOURCE_KEY_SCHEMA: &str = "ql.source-key-target/v1";
pub const SOURCE_KEY_REDUCTION_SCHEMA: &str = "ql.source-key-reduction/v1";
/// Declared representation limit, independent of the source's interval law.
pub const SOURCE_REGISTER_MIN: i8 = -32;
pub const SOURCE_REGISTER_MAX: i8 = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyDegreeAssignment {
    pub key: u8,
    pub source_degree: u16,
    pub octave: i8,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveKeyReduction {
    pub provenance: TuningProvenance,
    pub assignments: Vec<KeyDegreeAssignment>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentDegreeAssignment {
    pub component: u8,
    pub source_degree: u16,
    pub octave: i8,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceOctetReduction {
    pub provenance: TuningProvenance,
    pub assignments: Vec<ComponentDegreeAssignment>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePitchRequirement {
    /// The collection's actual standing remains in every target receipt.
    DeclaredAvailable,
    SourceAuthoredExact,
    /// Current source has no complete authentic maqam pitch set.
    AuthenticCondition,
}
pub struct SparseMusicalConsumer<'a> {
    pub basis: &'a CoupledBasis,
    pub writer: &'a MCoordinate,
    pub phase: u8,
    /// Required for a condition collection: the actual relation compiler's
    /// original input and current output, rather than a printable role label.
    pub condition: Option<SparseConditionConsumer<'a>>,
}
pub struct SparseConditionConsumer<'a> {
    pub producer_input: &'a crate::m2_engine::M2Request,
    pub plan: &'a crate::m2_relation_plan::M2RelationPlan,
}
pub struct SparseKeyPreparation<'a> {
    pub determination: MusicalDetermination,
    pub collection: TuningSourceCollection,
    pub reduction: ActiveKeyReduction,
    pub octet: Option<SourceOctetReduction>,
    pub requirement: SourcePitchRequirement,
    pub consumer: SparseMusicalConsumer<'a>,
}

/// Only preparation against current joined native owners constructs this set.
/// Its source/policy fields remain immutable after admission.
#[derive(Clone, Debug)]
pub struct SparseKeyTargets {
    determination: MusicalDetermination,
    collection: Arc<TuningSourceCollection>,
    reduction: ActiveKeyReduction,
    assignments: [Option<KeyDegreeAssignment>; 12],
    octet: Option<SourceOctetReduction>,
    requirement: SourcePitchRequirement,
    writer: MCoordinate,
    phase: u8,
    m2_generation: u64,
}

impl SparseKeyTargets {
    pub fn prepare(input: SparseKeyPreparation<'_>) -> Result<Self, String> {
        input.determination.validate_coupled_consumer(
            input.consumer.basis,
            input.consumer.writer,
            input.consumer.phase,
        )?;
        validate_collection(&input.collection, &input.determination, &input.consumer)?;
        validate_requirement(&input.collection, input.requirement)?;
        validate_provenance(&input.reduction.provenance)?;
        if input.reduction.assignments.len() > 12 {
            return Err("active key reduction exceeds twelve explicit assignments".into());
        }
        let mut assignments = [None; 12];
        for assignment in &input.reduction.assignments {
            canonical_coordinate(assignment.key)?;
            validate_register(assignment.octave)?;
            let slot = &mut assignments[usize::from(assignment.key)];
            if slot.is_some() {
                return Err("duplicate canonical key assignment".into());
            }
            let entry = input
                .collection
                .entry(assignment.source_degree)
                .ok_or("assigned source degree is absent from actual collection")?;
            if !matches!(entry.pitch, SourcePitchAvailability::Unavailable(_)) {
                resolve_pitch(&input.collection, entry, assignment.octave, 0)?;
            }
            *slot = Some(*assignment);
        }
        if let Some(octet) = &input.octet {
            validate_octet(octet, &input.collection)?;
        }
        Ok(Self {
            determination: input.determination,
            collection: Arc::new(input.collection),
            reduction: input.reduction,
            assignments,
            octet: input.octet,
            requirement: input.requirement,
            writer: input.consumer.writer.clone(),
            phase: input.consumer.phase,
            m2_generation: input
                .consumer
                .basis
                .m2_input
                .stamp
                .identity
                .profile_generation,
        })
    }
    pub fn determination(&self) -> &MusicalDetermination {
        &self.determination
    }
    pub fn collection(&self) -> &TuningSourceCollection {
        &self.collection
    }
    pub fn reduction(&self) -> &ActiveKeyReduction {
        &self.reduction
    }
    pub const fn m2_generation(&self) -> u64 {
        self.m2_generation
    }
    pub fn nodal_quartet(&self) -> &PositionalCollection<4> {
        self.determination.nodal_quartet()
    }
    /// A normal canonical key lookup can yield no target. Neither a zero-Hz
    /// sentinel nor a substitute architectural pitch is produced in that case.
    pub fn key_target(
        &self,
        key: u8,
        register: i8,
        touch_ref: &str,
    ) -> Result<SourceKeyTarget, String> {
        let coordinate = canonical_coordinate(key)?;
        validate_register(register)?;
        bounded_ref(touch_ref, "touch")?;
        let Some(assignment) = self.assignments[usize::from(key)] else {
            return Ok(SourceKeyTarget::Unavailable(UnavailableSourceKey {
                key,
                source_degree: None,
                reason: "source-key-unassigned".into(),
                reduction_policy: self.reduction.provenance.clone(),
                collection: self.collection.clone(),
            }));
        };
        let entry = self
            .collection
            .entry(assignment.source_degree)
            .ok_or("immutable source assignment lost its degree")?;
        if let SourcePitchAvailability::Unavailable(reason) = &entry.pitch {
            return Ok(SourceKeyTarget::Unavailable(UnavailableSourceKey {
                key,
                source_degree: Some(assignment.source_degree),
                reason: reason.clone(),
                reduction_policy: self.reduction.provenance.clone(),
                collection: self.collection.clone(),
            }));
        }
        let pitch = resolve_pitch(&self.collection, entry, assignment.octave, register)?;
        let fundamental = Fundamental::new(
            self.collection.tonic_provenance().hertz,
            self.collection.tonic_provenance().provenance.clone(),
        )?;
        let note = NoteTarget {
            identity: self.determination.identity().clone(),
            source_coordinate: self.determination.coordinate().clone(),
            coordinate,
            register,
            touch_ref: touch_ref.into(),
            pitch_class: pitch_at_lens(
                self.determination.basis(),
                self.determination.lens(),
                coordinate,
            ),
            exact_ratio: pitch.exact_ratio,
            hertz: pitch.hertz,
            fundamental,
            tuning_provenance: self.reduction.provenance.clone(),
        };
        Ok(SourceKeyTarget::Available(Box::new(SparseNoteTarget {
            key,
            assignment,
            note,
            applied_octave_ratio: pitch.octave_ratio,
            collection: self.collection.clone(),
            m2_generation: self.m2_generation,
            writer: self.writer.clone(),
            phase: self.phase,
        })))
    }
    /// Independent eightfold source-component policy. Seven active keys do not
    /// create this policy, and no key assignment supplies an implicit filler.
    pub fn audio_octet_targets(&self, register: i8) -> Result<[SourceComponentTarget; 8], String> {
        validate_register(register)?;
        let octet = self
            .octet
            .as_ref()
            .ok_or("source-octet-mapping-unavailable")?;
        let mut targets = Vec::with_capacity(8);
        for component in 0..8u8 {
            let assignment = *octet
                .assignments
                .iter()
                .find(|a| a.component == component)
                .ok_or("explicit source octet lost a component")?;
            let entry = self
                .collection
                .entry(assignment.source_degree)
                .ok_or("explicit source octet lost a degree")?;
            let pitch = resolve_pitch(&self.collection, entry, assignment.octave, register)?;
            let architectural =
                self.determination.explicate_octet().positions[usize::from(component)];
            targets.push(SourceComponentTarget {
                component,
                assignment,
                register,
                coordinate: architectural.coordinate,
                architectural_pitch_class: architectural.pitch_class,
                hertz: pitch.hertz,
                exact_ratio: pitch.exact_ratio,
                applied_octave_ratio: pitch.octave_ratio,
                provenance: octet.provenance.clone(),
                collection: self.collection.clone(),
            });
        }
        targets
            .try_into()
            .map_err(|_| "source octet cardinality drift".into())
    }
    /// Native recomposition, full collection equality, exact writer/phase and
    /// independent revision checks precede A's accepted event operation.
    pub fn validate_coupled_consumer(
        &self,
        consumer: SparseMusicalConsumer<'_>,
        current_collection: &TuningSourceCollection,
    ) -> Result<(), String> {
        if self.collection.as_ref() != current_collection
            || consumer.writer != &self.writer
            || consumer.phase != self.phase
            || consumer.basis.m2_input.stamp.identity.profile_generation != self.m2_generation
        {
            return Err("stale or disconnected sparse source/consumer".into());
        }
        self.determination.validate_coupled_consumer(
            consumer.basis,
            consumer.writer,
            consumer.phase,
        )?;
        validate_collection(current_collection, &self.determination, &consumer)?;
        validate_requirement(current_collection, self.requirement)
    }
    /// Saved reduction/source evidence is requalified against this actual
    /// producer. This does not grant authority from the serialized envelope.
    pub fn verify_retained(&self, candidate: &Value) -> Result<(), String> {
        if self.preparation_receipt()? != *candidate {
            return Err("retained sparse preparation differs from full native replay".into());
        }
        Ok(())
    }
    pub fn preparation_receipt(&self) -> Result<Value, String> {
        let key_availability = (0..12u8)
            .map(|key| {
                self.key_target(key, 0, &format!("catalog:key/{key}"))
                    .and_then(|target| target.receipt())
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(json!({"schema": SOURCE_KEY_REDUCTION_SCHEMA,
            "source_collection": self.collection.as_ref(), "reduction": self.reduction,
            "octet_reduction": self.octet, "requirement": self.requirement,
            "native_identity": identity_wire(&self.determination, self.m2_generation),
            "native_source_coordinate": crate::physical_body::source_coordinate_wire(self.determination.coordinate()),
            "writer": crate::physical_body::source_coordinate_wire(&self.writer), "phase": self.phase,
            "basis": format!("{:?}",self.determination.basis()), "lens":self.determination.lens().slot(),
            "context_frame":self.determination.context_frame().code(),
            "key_availability": key_availability}))
    }
}

#[derive(Clone, Debug)]
pub enum SourceKeyTarget {
    Available(Box<SparseNoteTarget>),
    Unavailable(UnavailableSourceKey),
}
impl SourceKeyTarget {
    pub fn note(&self) -> Option<&NoteTarget> {
        match self {
            Self::Available(target) => Some(target.note()),
            Self::Unavailable(_) => None,
        }
    }
    pub fn receipt(&self) -> Result<Value, String> {
        match self {
            Self::Available(target) => target.receipt(),
            Self::Unavailable(target) => Ok(json!({"schema":SOURCE_KEY_SCHEMA,"available":false,
                "key":target.key,"source_degree":target.source_degree,"reason":target.reason,
                "reduction_policy":target.reduction_policy,
                "source_collection":target.collection.as_ref(),
                "source_receipt":target.collection.source_receipt()})),
        }
    }
}
#[derive(Clone, Debug)]
pub struct UnavailableSourceKey {
    key: u8,
    source_degree: Option<u16>,
    reason: String,
    reduction_policy: TuningProvenance,
    collection: Arc<TuningSourceCollection>,
}
impl UnavailableSourceKey {
    pub const fn key(&self) -> u8 {
        self.key
    }
    pub const fn source_degree(&self) -> Option<u16> {
        self.source_degree
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
}
#[derive(Clone, Debug)]
pub struct SparseNoteTarget {
    key: u8,
    assignment: KeyDegreeAssignment,
    note: NoteTarget,
    applied_octave_ratio: ExactRatio,
    collection: Arc<TuningSourceCollection>,
    m2_generation: u64,
    writer: MCoordinate,
    phase: u8,
}
impl SparseNoteTarget {
    pub fn note(&self) -> &NoteTarget {
        &self.note
    }
    pub const fn assignment(&self) -> KeyDegreeAssignment {
        self.assignment
    }
    pub fn source_entry(&self) -> &TuningSourceEntry {
        self.collection
            .entry(self.assignment.source_degree)
            .expect("prepared immutable source degree")
    }
    pub fn collection(&self) -> &TuningSourceCollection {
        &self.collection
    }
    pub const fn applied_octave_ratio(&self) -> ExactRatio {
        self.applied_octave_ratio
    }
    pub fn receipt(&self) -> Result<Value, String> {
        let entry = self.source_entry();
        Ok(
            json!({"schema":SOURCE_KEY_SCHEMA,"available":true,"key":self.key,
            "source_degree":entry.original_degree,"reason":null,
            "reduction_policy":self.note.tuning_provenance,
            "source_collection":self.collection.as_ref(),"source_receipt":entry.source_receipt,
            "original_pitch":entry.pitch,"source_coordinate":entry.source_coordinate,
            "source_prime":entry.source_prime,"octave_return":entry.octave_return,"spelling":entry.spelling,
            "assigned_octave":self.assignment.octave,"applied_octave_ratio":self.applied_octave_ratio,
            "exact_ratio":self.note.exact_ratio,
            "native_target":{"identity":{"event_ref":self.note.identity.event_ref(),
                "m1_revision":self.note.identity.profile_generation().to_string(),"m2_generation":self.m2_generation.to_string(),
                "tick12":self.note.identity.tick12(),"degree720":self.note.identity.degree720()},
                "m1_source_coordinate":crate::physical_body::source_coordinate_wire(&self.note.source_coordinate),
                "m2_writer":crate::physical_body::source_coordinate_wire(&self.writer),"phase":self.phase,
                "position":self.note.coordinate.position.value(),"coordinate_face":self.note.coordinate.face.as_str(),
                "register":self.note.register,"touch_ref":self.note.touch_ref,"pitch_class":self.note.pitch_class,
                "hertz":self.note.hertz,"exact_ratio":self.note.exact_ratio,
                "fundamental":{"hertz":self.note.fundamental.hertz(),"provenance":self.note.fundamental.provenance()},
                "tuning_provenance":self.note.tuning_provenance}}),
        )
    }
}
#[derive(Clone, Debug)]
pub struct SourceComponentTarget {
    component: u8,
    assignment: ComponentDegreeAssignment,
    register: i8,
    coordinate: QlCoordinate,
    architectural_pitch_class: u8,
    hertz: f64,
    exact_ratio: Option<ExactRatio>,
    applied_octave_ratio: ExactRatio,
    provenance: TuningProvenance,
    collection: Arc<TuningSourceCollection>,
}
impl SourceComponentTarget {
    pub const fn component(&self) -> u8 {
        self.component
    }
    pub const fn assignment(&self) -> ComponentDegreeAssignment {
        self.assignment
    }
    pub const fn hertz(&self) -> f64 {
        self.hertz
    }
    pub const fn exact_ratio(&self) -> Option<ExactRatio> {
        self.exact_ratio
    }
    pub fn receipt(&self) -> Value {
        let entry = self
            .collection
            .entry(self.assignment.source_degree)
            .expect("prepared immutable source component");
        json!({"component":self.component,"assignment":self.assignment,"register":self.register,
            "coordinate":{"position":self.coordinate.position.value(),"face":self.coordinate.face.as_str()},
            "architectural_pitch_class":self.architectural_pitch_class,"hertz":self.hertz,
            "exact_ratio":self.exact_ratio,"applied_octave_ratio":self.applied_octave_ratio,
            "reduction_policy":self.provenance,"source_collection":self.collection.as_ref(),"source_entry":entry})
    }
}

struct ResolvedPitch {
    hertz: f64,
    exact_ratio: Option<ExactRatio>,
    octave_ratio: ExactRatio,
}
fn resolve_pitch(
    collection: &TuningSourceCollection,
    entry: &TuningSourceEntry,
    assigned_octave: i8,
    register: i8,
) -> Result<ResolvedPitch, String> {
    validate_register(assigned_octave)?;
    validate_register(register)?;
    let octave_ratio =
        ExactRatio::new(2, 1)?.pow(i32::from(assigned_octave) + i32::from(register))?;
    let (exact_ratio, multiplier) = match entry.pitch {
        SourcePitchAvailability::ExactRatio(ratio) => {
            let applied = ratio.compose(octave_ratio)?;
            (Some(applied), applied.as_f64())
        }
        SourcePitchAvailability::RetainedQuarterToneSteps(steps) => (
            None,
            2f64.powf(f64::from(steps) / 24.0) * octave_ratio.as_f64(),
        ),
        SourcePitchAvailability::Unavailable(_) => {
            return Err("source-component-pitch-unavailable".into());
        }
    };
    let hertz = collection.tonic_provenance().hertz * multiplier;
    if !hertz.is_finite() || hertz <= 0.0 {
        return Err("nonfinite or nonpositive source target Hz".into());
    }
    Ok(ResolvedPitch {
        hertz,
        exact_ratio,
        octave_ratio,
    })
}
fn validate_octet(
    octet: &SourceOctetReduction,
    collection: &TuningSourceCollection,
) -> Result<(), String> {
    validate_provenance(&octet.provenance)?;
    if octet.assignments.len() != 8 {
        return Err("source octet needs exactly eight explicit component assignments".into());
    }
    let mut seen = [false; 8];
    for assignment in &octet.assignments {
        let slot = seen
            .get_mut(usize::from(assignment.component))
            .ok_or("source component must be in 0..8")?;
        if *slot {
            return Err("duplicate source octet component".into());
        }
        *slot = true;
        let entry = collection
            .entry(assignment.source_degree)
            .ok_or("source octet degree absent")?;
        resolve_pitch(collection, entry, assignment.octave, 0)?;
    }
    Ok(())
}
fn validate_requirement(
    collection: &TuningSourceCollection,
    requirement: SourcePitchRequirement,
) -> Result<(), String> {
    match requirement {
        SourcePitchRequirement::DeclaredAvailable => Ok(()),
        SourcePitchRequirement::SourceAuthoredExact => {
            if collection.standing() != &CollectionStanding::SourceAuthoredInterval
                || collection
                    .entries()
                    .iter()
                    .any(|entry| !matches!(entry.pitch, SourcePitchAvailability::ExactRatio(_)))
            {
                return Err("source-authored exact pitches unavailable; approximation/operator standing cannot be promoted".into());
            }
            Ok(())
        }
        // The current native producer has no authentic complete maqam variant.
        // A future source owner must provide a distinct typed authentic variant
        // before this requirement can admit it. Planetary ratios are separate.
        SourcePitchRequirement::AuthenticCondition => {
            Err("authentic-condition-source-pitches-unavailable".into())
        }
    }
}
fn validate_collection(
    collection: &TuningSourceCollection,
    determination: &MusicalDetermination,
    consumer: &SparseMusicalConsumer<'_>,
) -> Result<(), String> {
    let basis = consumer.basis;
    let source = crate::m2_relation_plan::source_field();
    if collection.registry_revision() != source.registry_revision
        || collection.source_revision() != source.source_revision
    {
        return Err("source collection revision is stale".into());
    }
    let receipt = collection.source_receipt();
    if let Some(correspondence) = receipt.get("correspondence") {
        let owner = consumer
            .condition
            .as_ref()
            .ok_or("condition collection needs actual native relation-plan replay")?;
        let prepared = owner.plan.prepare_request(owner.producer_input, source)?;
        let plan_receipt = serde_json::to_value(owner.plan).map_err(|e| e.to_string())?;
        if owner.plan.identity != basis.m2_input.stamp.identity
            || serde_json::to_value(&prepared.condition).map_err(|e| e.to_string())?
                != serde_json::to_value(&basis.m2_input.condition).map_err(|e| e.to_string())?
            || !basis.input.source_receipts.contains(&plan_receipt)
        {
            return Err(
                "condition collection relation plan is disconnected from joined native consumer"
                    .into(),
            );
        }
        let condition = basis
            .m2_input
            .condition
            .as_ref()
            .ok_or("source collection lacks active M2 condition")?;
        let rule = crate::m2_condition::correspondence_field()
            .rule(condition.maqam_index, condition.role)
            .ok_or("active condition source relation unavailable")?;
        if *correspondence != serde_json::to_value(rule).map_err(|e| e.to_string())? {
            return Err("source collection maqam/role differs from current native M2 path".into());
        }
        let frame = basis.m2_input.execute()?;
        let path = frame
            .condition
            .as_ref()
            .and_then(|c| c.source_path.as_ref())
            .ok_or("joined M2 condition source path disconnected")?;
        if *correspondence != serde_json::to_value(path).map_err(|e| e.to_string())? {
            return Err("source collection differs from compiled M2 correspondence receipt".into());
        }
    }
    if matches!(
        collection.standing(),
        CollectionStanding::NativeRationalOperator
            | CollectionStanding::ExplicitArchitecturalTuning
    ) {
        if receipt["source_coordinate"].as_str()
            != Some(determination.coordinate().source_ref.as_str())
            || receipt["source_prime"].as_bool()
                != Some(determination.coordinate().face == MFace::Pratibimba)
            || receipt["event_ref"].as_str() != Some(determination.identity().event_ref())
            || receipt["profile_generation"].as_str()
                != Some(
                    determination
                        .identity()
                        .profile_generation()
                        .to_string()
                        .as_str(),
                )
            || receipt["basis"].as_str() != Some(format!("{:?}", determination.basis()).as_str())
        {
            return Err(
                "source operator collection has a disconnected M1 coordinate/face/event/basis"
                    .into(),
            );
        }
        if collection.standing() == &CollectionStanding::ExplicitArchitecturalTuning
            && (receipt["lens"].as_u64() != Some(u64::from(determination.lens().slot()))
                || receipt["context_frame"].as_str() != Some(determination.context_frame().code()))
        {
            return Err(
                "architectural source collection lens/Context Frame is disconnected".into(),
            );
        }
    }
    Ok(())
}
fn canonical_coordinate(key: u8) -> Result<QlCoordinate, String> {
    if key >= 12 {
        return Err("canonical Jankó key must be in 0..12".into());
    }
    Ok(QlCoordinate::new(
        QlPosition::new(key / 2).map_err(|e| e.to_string())?,
        if key % 2 == 0 {
            QlFace::Direct
        } else {
            QlFace::Conjugate
        },
    ))
}
fn bounded_ref(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 || value.contains('\0') {
        return Err(format!("bounded nonempty {label} reference required"));
    }
    Ok(())
}
fn validate_provenance(value: &TuningProvenance) -> Result<(), String> {
    Fundamental::new(1.0, value.clone()).map(|_| ())
}
fn validate_register(register: i8) -> Result<(), String> {
    if !(SOURCE_REGISTER_MIN..=SOURCE_REGISTER_MAX).contains(&register) {
        return Err("source register exceeds declared bounded octave representation".into());
    }
    Ok(())
}
fn identity_wire(determination: &MusicalDetermination, m2_generation: u64) -> Value {
    json!({"event_ref":determination.identity().event_ref(),
        "m1_revision":determination.identity().profile_generation().to_string(),
        "m2_generation":m2_generation.to_string(),"tick12":determination.identity().tick12(),
        "degree720":determination.identity().degree720()})
}
