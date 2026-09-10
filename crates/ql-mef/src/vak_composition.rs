//! Operative C-prime / Context-Frame composition in the QL Kernel.
//!
//! This is the operational layer over the accepted structural carrier, not a
//! second shape vocabulary, expression parser, source registry or scheduler.
//! Wholes are immutable uses; a contextual transition always creates a new use.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use ql_core::{AnchorReturn, CallerProvenance, GroundKind, KernelRelationId, QlCoordinate,
    QlFace, QlFamily, QlPosition, QlShape, QlShapeAddress, RelationFieldComposition,
    RelationFieldDerivation, ShapeBinding, StructuralConstellation, StructuralParticipation,
    VakFamily, RELATION_FIELD_COMPOSITION_OPERATOR_REF};
use crate::{ContextFrameId, LensId, LensRef, MefRotation, MusicalBasis, SelfOtherForm,
    VakContextField, VakDivineAct, VakExpressionReadingV1, VakExpressionSubject,
    VakGeneralExpressionEvidence, VakRPath, VakRef, VakRegistry, VakRelationOp,
    VakSourceProvenance, VakStanding, cf_diatonic_cut, directed_pitch_delta};

pub const CONTRACT: &str = "ql.vak-composition/v1";
pub const MAX_DEPTH: usize = 64;
pub const MAX_OBJECTS: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionError(pub String);
impl fmt::Display for CompositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}
impl std::error::Error for CompositionError {}
pub type Result<T> = std::result::Result<T, CompositionError>;
fn err(e: impl fmt::Display) -> CompositionError { CompositionError(e.to_string()) }
fn require(test: bool, why: &str) -> Result<()> {
    if test { Ok(()) } else { Err(err(why)) }
}
fn reference(value: &str) -> Result<()> { require(!value.trim().is_empty(), "empty reference") }
fn references(values: &[String]) -> Result<()> {
    require(!values.is_empty(), "attributable operation requires evidence")?;
    for v in values { reference(v)?; }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Basis {
    pub provenance: CallerProvenance,
    pub revision: String,
    pub evidence: Vec<String>,
}
impl Basis {
    pub fn validate(&self) -> Result<()> {
        CallerProvenance::new(&self.provenance.caller_ref, &self.provenance.source_ref,
            &self.provenance.standing_ref).map_err(err)?;
        reference(&self.revision)?;
        references(&self.evidence)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionBasis { Local, Absolute }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveFrame {
    pub id: ContextFrameId,
    pub lens: LensId,
    pub basis: MusicalBasis,
    pub face: QlFace,
    pub positions: PositionBasis,
}
impl ActiveFrame {
    pub fn coordinate(self) -> QlCoordinate {
        let c = self.id.canonical_selection().at_lens(self.lens).coordinate();
        QlCoordinate::new(match self.positions {
            PositionBasis::Local => c.local_position(), PositionBasis::Absolute => c.absolute_position(),
        }, self.face)
    }
    pub fn pitch(self) -> u8 {
        let cut = cf_diatonic_cut(self.basis, self.lens);
        let i = cut.frames.iter().position(|f| *f == self.id).expect("canonical CF cut");
        cut.pitches[i]
    }
}

/// QL interpretation of a native expression NODE, not another AST. The caller
/// supplies the accepted syntax revision; matching declarations are not proof of
/// native execution or of semantic truth. Arbitrary 109-field refs remain valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullVakBinding {
    pub general: VakGeneralExpressionEvidence,
    pub accepted_syntax_revision: String,
    pub native_node_ref: String,
    pub reading: VakExpressionReadingV1,
    pub self_other: SelfOtherForm,
    pub field: VakContextField,
    pub interpreter: String,
    pub expected_ground: Option<String>,
}
impl FullVakBinding {
    pub fn validate(&self, registry: &VakRegistry) -> Result<()> {
        require(self.general.syntax_version == crate::AIKIT_OPERATIVE_SYNTAX_VERSION,
            "unsupported AIKit syntax contract")?;
        reference(&self.accepted_syntax_revision)?;
        require(self.general.owner_revision == self.accepted_syntax_revision,
            "expression and accepted syntax revision differ")?;
        for r in [&self.general.resolve_path_identity, &self.general.rendered,
            &self.general.full_vak_rendering, &self.native_node_ref, &self.interpreter] { reference(r)?; }
        references(&self.general.evidence)?;
        self.reading.validate(registry).map_err(err)?;
        references(&self.reading.evidence)?;
        for s in &self.reading.subjects { if let VakExpressionSubject::Native(r) = s { reference(r)?; } }
        if let Some(r) = &self.expected_ground { reference(r)?; }
        self.sources(registry)?;
        Ok(())
    }
    pub fn sources(&self, registry: &VakRegistry) -> Result<Vec<VakSourceProvenance>> {
        let mut refs = BTreeSet::from([self.self_other.source_ref(), self.field.source_ref()]);
        refs.extend(registry.bind_operator(self.reading.operator).map_err(err)?.source_support);
        refs.extend(registry.bind_horizon(self.reading.horizon).map_err(err)?.source_support);
        refs.extend(self.reading.relation_refs.iter().cloned());
        refs.extend(self.reading.complement_refs.iter().cloned());
        for s in &self.reading.subjects { if let VakExpressionSubject::Vak(r) = s { refs.insert(r.clone()); } }
        refs.iter().map(|r| registry.locate(r).map(|e| e.source.clone())
            .ok_or_else(|| err(format!("unknown Vāk source {r}")))).collect()
    }
    fn addresses(&self, subject: &str) -> bool {
        self.reading.subjects.iter().any(|s| matches!(s, VakExpressionSubject::Native(r) if r == subject))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WholeInput {
    pub use_ref: String,
    pub form: StructuralConstellation,
    pub binding: ShapeBinding,
    pub category: QlFamily,
    pub ground_ref: String,
    pub ground_face: QlFace,
    pub frame: ActiveFrame,
    pub basis: Basis,
    pub language: Option<FullVakBinding>,
}

/// A use of an existing field can be an axis in another field. This recursive
/// carrier retains its TWO actual sources. It does not manufacture a flattened
/// constellation or assign numeric dimensions as semantic shape authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WholeBody {
    Local(StructuralConstellation),
    Relation {
        row: String,
        column: String,
        /// The v1 carrier is present whenever both immediate sources are local.
        /// Deeper composition references the existing child field carriers.
        carrier: Option<RelationFieldComposition>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Whole {
    pub use_ref: String,
    pub binding: ShapeBinding,
    pub category: QlFamily,
    pub ground_ref: String,
    pub ground_face: QlFace,
    pub frame: ActiveFrame,
    pub basis: Vec<Basis>,
    pub language: Option<FullVakBinding>,
    pub body: WholeBody,
    pub producing_refs: Vec<String>,
    pub transitions: Vec<Transition>,
    depth: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub from_ref: String,
    pub operation: String,
    pub from: ActiveFrame,
    pub into: ActiveFrame,
    pub basis: Basis,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeInput {
    pub use_ref: String,
    pub whole_ref: String,
    pub row: String,
    pub column: String,
    pub frame: ActiveFrame,
    pub ground_ref: String,
    pub ground_face: QlFace,
    pub basis: Basis,
    pub language: Option<FullVakBinding>,
}

/// An exact structured address, NOT a new expression language. Pair addresses
/// recursively contain the selected addresses of their source wholes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectedAddress {
    Anchor(String),
    Member { whole_ref: String, member: StructuralParticipation },
    Relation { whole_ref: String, row: Box<SelectedAddress>, column: Box<SelectedAddress> },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedReading {
    pub use_ref: String,
    pub binding: ShapeBinding,
    pub frame: ActiveFrame,
    pub address: SelectedAddress,
    pub harmonic_pitch: u8,
    /// Child pitches relative to this whole's active harmonic frame. This is
    /// consequential contextual computation, not changing the child frames.
    pub child_intervals: Vec<u8>,
    pub geometry: GeometryReading,
    pub children: Vec<FramedReading>,
    pub carrier_derivations: Vec<RelationFieldDerivation>,
    pub basis: Vec<Basis>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeometryReading {
    pub shape_ref: String,
    pub absolute_position: QlPosition,
    pub frame_phase_degrees: u16,
    /// Derived relative circle phases of the SAME harmonic determination.
    pub child_phase_degrees: Vec<u16>,
    pub viewing_lens: LensRef,
    pub viewing_rotation: MefRotation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis { Row, Column }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentContribution {
    pub actor_ref: String,
    pub result_ref: String,
    pub input_refs: Vec<String>,
    pub evidence: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Determination {
    pub reference: String,
    pub whole_use: String,
    pub reading: FramedReading,
    pub language: Option<FullVakBinding>,
    pub sources: Vec<VakSourceProvenance>,
    pub contribution: Option<AgentContribution>,
    pub basis: Basis,
    pub standing: VakStanding,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Returned {
    pub reference: String,
    pub determination: String,
    pub source_use: String,
    pub target_use: String,
    pub route: AnchorReturn,
    pub source_binding: ShapeBinding,
    pub target_binding: ShapeBinding,
    pub producing_reading: FramedReading,
    pub basis: Basis,
    pub standing: VakStanding,
}

#[derive(Debug, Default)]
pub struct VakComposition {
    wholes: BTreeMap<String, Whole>,
    determinations: BTreeMap<String, Determination>,
    returns: BTreeMap<String, Returned>,
}
impl VakComposition {
    fn vacant(&self, r: &str) -> Result<()> {
        reference(r)?;
        require(self.wholes.len() + self.determinations.len() + self.returns.len() < MAX_OBJECTS,
            "composition object bound exceeded")?;
        require(!self.wholes.contains_key(r) && !self.determinations.contains_key(r)
            && !self.returns.contains_key(r), "reference already exists")
    }
    pub fn whole(&self, r: &str) -> Result<&Whole> { self.wholes.get(r).ok_or_else(|| err(format!("unknown whole use {r}"))) }
    pub fn determination(&self, r: &str) -> Result<&Determination> { self.determinations.get(r).ok_or_else(|| err(format!("unknown determination {r}"))) }
    pub fn returned(&self, r: &str) -> Result<&Returned> { self.returns.get(r).ok_or_else(|| err(format!("unknown Return {r}"))) }

    pub fn bind_whole(&mut self, registry: &VakRegistry, input: WholeInput) -> Result<()> {
        self.vacant(&input.use_ref)?;
        input.basis.validate()?;
        reference(&input.ground_ref)?;
        validate_binding(&input.binding)?;
        let form = StructuralConstellation::new(&input.form.anchor_ref, input.form.members.clone(),
            input.form.returns.clone()).map_err(err)?;
        for m in &form.members { StructuralParticipation::new(&m.subject_ref, m.coordinate.position, m.coordinate.face).map_err(err)?; }
        for r in &form.returns {
            AnchorReturn::new(&r.from_ref, &r.through_anchor_ref, &r.target_ground_ref, r.target_face, r.ground_kind).map_err(err)?;
            require(r.target_ground_position.value() == 0, "Return ground position must be #0")?;
        }
        require(input.binding.whole_ref == form.anchor_ref
            && input.binding.shape_ref == QlShape::Constellation(form.grain()).shape_ref(), "binding/form mismatch")?;
        require(input.binding.members.len() == form.members.len()
            && form.members.iter().all(|m| input.binding.members.contains(m)
                && input.binding.basis_refs.contains(&m.subject_ref)), "binding must retain actual members")?;
        require(input.binding.provenance == input.basis.provenance, "binding/source attribution mismatch")?;
        for r in &input.binding.relation_bindings {
            require(form.members.iter().any(|m| m.coordinate == r.address.row)
                && form.members.iter().any(|m| m.coordinate == r.address.column), "relation outside whole")?;
        }
        if let Some(l) = &input.language {
            l.validate(registry)?;
            require(l.addresses(&input.binding.subject_ref), "expression does not address bound subject")?;
        }
        self.wholes.insert(input.use_ref.clone(), Whole { use_ref: input.use_ref, binding: input.binding,
            category: input.category, ground_ref: input.ground_ref, ground_face: input.ground_face,
            frame: input.frame, basis: vec![input.basis], language: input.language, body: WholeBody::Local(form),
            producing_refs: Vec::new(), transitions: Vec::new(), depth: 0 });
        Ok(())
    }

    pub fn compose(&mut self, registry: &VakRegistry, input: ComposeInput) -> Result<()> {
        self.vacant(&input.use_ref)?;
        input.basis.validate()?;
        reference(&input.whole_ref)?;
        reference(&input.ground_ref)?;
        let row = self.whole(&input.row)?;
        let col = self.whole(&input.column)?;
        let depth = row.depth.max(col.depth) + 1;
        require(depth <= MAX_DEPTH, "recursive field depth exceeded")?;
        // References can only point to earlier immutable objects: cycles cannot
        // be installed, including self-reference and later-inserted back edges.
        let carrier = match (&row.body, &col.body) {
            (WholeBody::Local(a), WholeBody::Local(b)) => Some(RelationFieldComposition::compose(a, b).map_err(err)?),
            _ => None,
        };
        let shape_ref = carrier.as_ref().map(RelationFieldComposition::shape_ref).unwrap_or_else(||
            format!("ql:carrier:1.0.0:relation-field:{}:by:{}", row.binding.shape_ref, col.binding.shape_ref));
        if let Some(l) = &input.language {
            l.validate(registry)?;
            require(matches!(l.reading.operator, VakRelationOp::Relate | VakRelationOp::Contextualise)
                && l.addresses(&row.binding.subject_ref) && l.addresses(&col.binding.subject_ref),
                "composition expression must relate its actual participating subjects")?;
        }
        let binding = ShapeBinding::new(&input.use_ref, shape_ref, &input.whole_ref,
            vec![row.binding.subject_ref.clone(), col.binding.subject_ref.clone()], Vec::new(), Vec::new(),
            Some(input.use_ref.clone()), Some(RELATION_FIELD_COMPOSITION_OPERATOR_REF.into()), Vec::new(),
            input.basis.provenance.clone()).map_err(err)?;
        let mut basis = row.basis.clone();
        extend_unique(&mut basis, &col.basis);
        basis.push(input.basis);
        self.wholes.insert(input.use_ref.clone(), Whole { use_ref: input.use_ref,
            binding, category: QlFamily::C, ground_ref: input.ground_ref, ground_face: input.ground_face,
            frame: input.frame, basis, language: input.language,
            producing_refs: vec![input.row.clone(), input.column.clone()],
            body: WholeBody::Relation { row: input.row, column: input.column, carrier },
            transitions: Vec::new(), depth });
        Ok(())
    }

    pub fn reframe(&mut self, from: &str, into: &str, frame: ActiveFrame, basis: Basis) -> Result<()> {
        self.vacant(into)?;
        basis.validate()?;
        let mut whole = self.whole(from)?.clone();
        whole.transitions.push(Transition { from_ref: from.into(), operation: KernelRelationId::ContextFrame.as_str().into(),
            from: whole.frame, into: frame, basis: basis.clone() });
        whole.frame = frame;
        whole.use_ref = into.into();
        whole.producing_refs.push(from.into());
        whole.basis.push(basis);
        self.wholes.insert(into.into(), whole);
        Ok(())
    }

    /// Deterministic address traversal through an actual composed field.
    pub fn position(&self, start: &str, path: &[Axis]) -> Result<&Whole> {
        require(path.len() <= MAX_DEPTH, "position path bound exceeded")?;
        let mut current = self.whole(start)?;
        for axis in path {
            current = match &current.body {
                WholeBody::Relation { row, column, .. } => self.whole(match axis { Axis::Row => row, Axis::Column => column })?,
                WholeBody::Local(_) => return Err(err("cannot traverse into a local whole without a disclosed child relation")),
            };
        }
        Ok(current)
    }

    pub fn read(&self, use_ref: &str, viewing_lens: LensId) -> Result<FramedReading> {
        let whole = self.whole(use_ref)?;
        let (address, children, carrier_derivations) = match &whole.body {
            WholeBody::Local(form) => {
                let a = if form.members.is_empty() { SelectedAddress::Anchor(form.anchor_ref.clone()) } else {
                    let coordinate = whole.frame.coordinate();
                    let member = form.members.iter().find(|m| m.coordinate == coordinate)
                        .ok_or_else(|| err(format!("{use_ref}: active frame selects an undisclosed member {coordinate:?}")))?;
                    SelectedAddress::Member { whole_ref: form.anchor_ref.clone(), member: member.clone() }
                };
                (a, Vec::new(), Vec::new())
            }
            WholeBody::Relation { row, column, carrier } => {
                let a = self.read(row, viewing_lens)?;
                let b = self.read(column, viewing_lens)?;
                let mut derivations = a.carrier_derivations.clone();
                extend_unique(&mut derivations, &b.carrier_derivations);
                if let Some(c) = carrier {
                    let (SelectedAddress::Member { member: am, .. }, SelectedAddress::Member { member: bm, .. }) = (&a.address, &b.address)
                        else { return Err(err("v1 carrier requires actual positional axes")); };
                    require(c.contains_address(QlShapeAddress { row: am.coordinate, column: bm.coordinate }), "address outside carrier")?;
                    derivations.push(c.derivation());
                }
                (SelectedAddress::Relation { whole_ref: whole.binding.whole_ref.clone(), row: Box::new(a.address.clone()),
                    column: Box::new(b.address.clone()) }, vec![a, b], derivations)
            }
        };
        let pitch = whole.frame.pitch();
        let child_intervals: Vec<u8> = children.iter().map(|c| directed_pitch_delta(pitch, c.harmonic_pitch)).collect();
        let absolute = whole.frame.id.canonical_selection().at_lens(whole.frame.lens).coordinate().absolute_position();
        let local = QlPosition::new((absolute.value() + 6 - viewing_lens.index()) % 6).map_err(err)?;
        Ok(FramedReading { use_ref: use_ref.into(), binding: whole.binding.clone(), frame: whole.frame,
            address, harmonic_pitch: pitch,
            geometry: GeometryReading { shape_ref: whole.binding.shape_ref.clone(), absolute_position: absolute,
                frame_phase_degrees: u16::from(pitch) * 30,
                child_phase_degrees: child_intervals.iter().map(|p| u16::from(*p) * 30).collect(),
                viewing_lens: LensRef::canonical(viewing_lens), viewing_rotation: MefRotation::new(viewing_lens, local) },
            child_intervals, children, carrier_derivations, basis: whole.basis.clone() })
    }

    pub fn determine(&mut self, registry: &VakRegistry, request: DetermineInput) -> Result<()> {
        self.vacant(&request.reference)?;
        request.basis.validate()?;
        let whole = self.whole(&request.whole_use)?;
        if let Some(l) = &request.language {
            l.validate(registry)?;
            require(l.reading.operator == VakRelationOp::Express && l.addresses(&whole.binding.subject_ref),
                "generative expression must Express the producing whole")?;
        }
        if let Some(c) = &request.contribution {
            reference(&c.actor_ref)?; reference(&c.result_ref)?; references(&c.evidence)?; references(&c.input_refs)?;
            require(c.input_refs.contains(&request.whole_use), "Agent input does not name producing whole")?;
            require(request.language.is_some(), "Agent interpretation requires its full-profile reading")?;
        }
        let reading = self.read(&request.whole_use, request.viewing_lens)?;
        let mut sources = Vec::new();
        self.collect_sources(registry, &request.whole_use, &mut sources)?;
        if let Some(l) = &request.language { extend_unique(&mut sources, &l.sources(registry)?); }
        self.determinations.insert(request.reference.clone(), Determination { reference: request.reference,
            whole_use: request.whole_use, reading, language: request.language, sources,
            contribution: request.contribution, basis: request.basis, standing: VakStanding::Derived });
        Ok(())
    }
    fn collect_sources(&self, registry: &VakRegistry, r: &str, into: &mut Vec<VakSourceProvenance>) -> Result<()> {
        let w = self.whole(r)?;
        if let Some(l) = &w.language { extend_unique(into, &l.sources(registry)?); }
        if let WholeBody::Relation { row, column, .. } = &w.body {
            self.collect_sources(registry, row, into)?;
            self.collect_sources(registry, column, into)?;
        }
        Ok(())
    }
    fn contains_use(&self, parent: &str, child: &str) -> Result<bool> {
        if parent == child { return Ok(true); }
        match &self.whole(parent)?.body {
            WholeBody::Local(_) => Ok(false),
            WholeBody::Relation { row, column, .. } => Ok(self.contains_use(row, child)? || self.contains_use(column, child)?),
        }
    }
    pub fn return_result(&mut self, input: ReturnInput) -> Result<()> {
        self.vacant(&input.reference)?; input.basis.validate()?;
        let d = self.determination(&input.determination)?;
        let source = self.whole(&d.whole_use)?;
        let target = self.whole(&input.target_use)?;
        match input.kind {
            GroundKind::Own => require(source.use_ref == target.use_ref && input.relation_evidence.is_none(), "own ground must be producing use")?,
            GroundKind::Parent => require(source.use_ref != target.use_ref && self.contains_use(&target.use_ref, &source.use_ref)?, "target is not an explicit parent")?,
            GroundKind::Child => require(source.use_ref != target.use_ref && self.contains_use(&source.use_ref, &target.use_ref)?, "target is not an explicit child")?,
            GroundKind::Other | GroundKind::Conjugate => {
                require(source.use_ref != target.use_ref, "other ground must remain distinct")?;
                reference(input.relation_evidence.as_deref().ok_or_else(|| err("other/conjugate ground needs attributable relation evidence"))?)?;
                if input.kind == GroundKind::Conjugate { require(source.ground_face != target.ground_face, "ground is not conjugate")?; }
            }
        }
        if let Some(l) = &d.language { if let Some(expected) = &l.expected_ground {
            require(expected == &target.ground_ref, "Return contradicts full-profile explicit ground")?;
        } }
        let route = AnchorReturn::new(&d.reference, &source.binding.whole_ref, &target.ground_ref, target.ground_face, input.kind).map_err(err)?;
        self.returns.insert(input.reference.clone(), Returned { reference: input.reference, determination: d.reference.clone(),
            source_use: source.use_ref.clone(), target_use: target.use_ref.clone(), route,
            source_binding: source.binding.clone(), target_binding: target.binding.clone(),
            producing_reading: d.reading.clone(), basis: input.basis, standing: VakStanding::Derived });
        Ok(())
    }
    /// The returned object keeps its actual body, even when it is a recursively
    /// composed field. No caller-fabricated constellation is needed to reuse it.
    pub fn offer_as_whole(&mut self, return_ref: &str, into: &str, whole_ref: &str, basis: Basis) -> Result<()> {
        self.vacant(into)?; reference(whole_ref)?; basis.validate()?;
        let returned = self.returned(return_ref)?;
        require(basis.provenance.source_ref == return_ref
            && basis.provenance.standing_ref == VakStanding::Derived.as_schema_str(), "returned content remains DERIVED from its Return")?;
        let mut w = self.whole(&returned.source_use)?.clone();
        w.use_ref = into.into();
        w.binding.subject_ref = returned.determination.clone();
        w.binding.whole_ref = whole_ref.into();
        w.binding.provenance = basis.provenance.clone();
        w.binding.derivation_ref = Some(returned.determination.clone());
        w.binding.return_refs.push(return_ref.into());
        w.basis.push(basis);
        w.producing_refs.extend([returned.source_use.clone(), returned.determination.clone(), return_ref.into()]);
        // A new whole's source language stays in its producing relations. It is
        // not an authored-source expression describing the old native subject.
        w.language = None;
        if let WholeBody::Local(form) = &mut w.body {
            form.anchor_ref = whole_ref.into();
            // Historical source Returns are in producing_reading, not silently
            // reanchored to the newly offered whole.
            form.returns.clear();
        }
        self.wholes.insert(into.into(), w);
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct DetermineInput {
    pub reference: String, pub whole_use: String, pub viewing_lens: LensId,
    pub language: Option<FullVakBinding>, pub contribution: Option<AgentContribution>, pub basis: Basis,
}
#[derive(Debug, Clone)]
pub struct ReturnInput {
    pub reference: String, pub determination: String, pub target_use: String,
    pub kind: GroundKind, pub relation_evidence: Option<String>, pub basis: Basis,
}
fn extend_unique<T: Clone + PartialEq>(into: &mut Vec<T>, from: &[T]) {
    for v in from { if !into.contains(v) { into.push(v.clone()); } }
}
fn validate_binding(b: &ShapeBinding) -> Result<()> {
    CallerProvenance::new(&b.provenance.caller_ref, &b.provenance.source_ref, &b.provenance.standing_ref).map_err(err)?;
    ShapeBinding::new(&b.subject_ref, &b.shape_ref, &b.whole_ref, b.basis_refs.clone(), b.members.clone(),
        b.relation_bindings.clone(), b.derivation_ref.clone(), b.operator_ref.clone(), b.return_refs.clone(), b.provenance.clone()).map_err(err)?;
    for r in &b.relation_bindings { ql_core::ShapeRelationBinding::new(r.address, &r.relation_ref, r.evidence_refs.clone()).map_err(err)?; }
    Ok(())
}

/// C-prime is the situated reflective relation OF a C-grounded whole, not an
/// alias for the conjugate face of QlFamily::C. Its six operations retain the
/// accepted VakFamily relation IDs; none are indexed through another sixfold.
#[derive(Debug, Clone)]
pub struct CPrimeContext {
    pub ground_use: String,
    pub focus_use: String,
    pub category_ground: QlCoordinate,
    pub allowed_operators: Vec<VakRelationOp>,
    pub content_fields: Vec<VakContextField>,
    pub thread: Vec<String>,
    pub r_path: Option<VakRPath>,
    pub operations: Vec<ReflectiveReceipt>,
}
#[derive(Debug, Clone)]
pub struct ReflectiveReceipt {
    pub family: VakFamily,
    pub relation_id: KernelRelationId,
    pub input_refs: Vec<String>,
    pub output_refs: Vec<String>,
    pub basis: Basis,
}
impl CPrimeContext {
    pub fn enter(graph: &VakComposition, ground: &str, category_ground: QlCoordinate) -> Result<Self> {
        graph.whole(ground)?;
        Ok(Self { ground_use: ground.into(), focus_use: ground.into(), category_ground,
            allowed_operators: VakRelationOp::ALL.to_vec(), content_fields: VakContextField::ALL.to_vec(),
            thread: Vec::new(), r_path: None, operations: Vec::new() })
    }
    fn receipt(&mut self, family: VakFamily, inputs: Vec<String>, outputs: Vec<String>, basis: Basis) {
        self.operations.push(ReflectiveReceipt { family, relation_id: family.relation_id(), input_refs: inputs, output_refs: outputs, basis });
    }
    /// CPF discriminates eligible language operations and explicitly selects the
    /// direct/conjugate leading face. It does not complement the position.
    pub fn cpf(&mut self, graph: &mut VakComposition, into: &str, face: QlFace,
        operators: Vec<VakRelationOp>, basis: Basis) -> Result<()> {
        require(!operators.is_empty(), "CPF needs an explicit eligible operator set")?;
        let from = self.focus_use.clone();
        let mut frame = graph.whole(&from)?.frame;
        frame.face = face;
        graph.reframe(&from, into, frame, basis.clone())?;
        self.allowed_operators = operators;
        self.focus_use = into.into();
        self.receipt(VakFamily::Cpf, vec![from], vec![into.into()], basis);
        Ok(())
    }
    /// CT selects source-qualified content fields. It is not CF selection and
    /// does not cast the seven M0-4 fields into seven ContextFrameIds.
    pub fn ct(&mut self, fields: Vec<VakContextField>, basis: Basis) -> Result<()> {
        basis.validate()?;
        require(!fields.is_empty(), "CT requires eligible content fields")?;
        let outputs = fields.iter().map(|f| f.source_ref().to_string()).collect();
        self.content_fields = fields;
        self.receipt(VakFamily::Ct, vec![self.focus_use.clone()], outputs, basis);
        Ok(())
    }
    /// CP resolves the actual positioned whole inside the current composition.
    pub fn cp(&mut self, graph: &VakComposition, path: &[Axis], basis: Basis) -> Result<()> {
        basis.validate()?;
        let from = self.focus_use.clone();
        let focus = graph.position(&from, path)?;
        self.focus_use = focus.use_ref.clone();
        self.receipt(VakFamily::Cp, vec![from], vec![self.focus_use.clone()], basis);
        Ok(())
    }
    pub fn cf(&mut self, graph: &mut VakComposition, into: &str, frame: ActiveFrame, basis: Basis) -> Result<()> {
        let from = self.focus_use.clone();
        graph.reframe(&from, into, frame, basis.clone())?;
        self.focus_use = into.into();
        self.receipt(VakFamily::Cf, vec![from], vec![into.into()], basis);
        Ok(())
    }
    /// CFP binds an explicit recursive whole path to a source R-factor path.
    /// This is a compositional thread; native scheduling remains Factory-owned.
    pub fn cfp(&mut self, graph: &VakComposition, registry: &VakRegistry,
        paths: &[Vec<Axis>], act: VakDivineAct, basis: Basis) -> Result<Vec<FramedReading>> {
        basis.validate()?;
        require(!paths.is_empty() && paths.len() <= MAX_DEPTH, "CFP requires a bounded nonempty thread")?;
        let source_path = registry.r_path(act).map_err(err)?;
        let mut readings = Vec::new();
        let mut thread = Vec::new();
        for path in paths {
            let w = graph.position(&self.focus_use, path)?;
            if let Some(l) = &w.language { self.admit(l)?; }
            readings.push(graph.read(&w.use_ref, w.frame.lens)?);
            thread.push(w.use_ref.clone());
        }
        self.thread = thread.clone();
        self.r_path = Some(source_path);
        self.receipt(VakFamily::Cfp, vec![self.focus_use.clone()], thread, basis);
        Ok(readings)
    }
    pub fn admit(&self, binding: &FullVakBinding) -> Result<()> {
        require(self.allowed_operators.contains(&binding.reading.operator), "CPF excludes this language operation")?;
        require(self.content_fields.contains(&binding.field), "CT excludes this contextual content")
    }
    pub fn determine(&self, graph: &mut VakComposition, registry: &VakRegistry, request: DetermineInput) -> Result<()> {
        require(request.whole_use == self.focus_use, "determination does not use the current CP/CF focus")?;
        if let Some(l) = &request.language { self.admit(l)?; }
        graph.determine(registry, request)
    }
    /// CS closes this declared thread through attributable determinations and
    /// their explicit grounds. No numeric phase silently chooses a destination.
    pub fn cs(&mut self, graph: &mut VakComposition, input: ReturnInput) -> Result<()> {
        let d = graph.determination(&input.determination)?;
        require(d.whole_use == self.focus_use || self.thread.contains(&d.whole_use), "CS result is outside the declared context thread")?;
        if let Some(l) = &d.language { self.admit(l)?; }
        let outputs = vec![input.reference.clone()];
        let inputs = vec![input.determination.clone(), input.target_use.clone()];
        let basis = input.basis.clone();
        graph.return_result(input)?;
        self.receipt(VakFamily::Cs, inputs, outputs, basis);
        Ok(())
    }
}

/// Whole source neighbourhood traversal is inherited, not reimplemented here.
pub fn source_neighbourhood(registry: &VakRegistry, reference: &VakRef, depth: usize) -> Result<crate::VakNeighbourhood> {
    require(depth <= MAX_DEPTH, "source neighbourhood depth bound exceeded")?;
    registry.neighbourhood(reference, depth).map_err(err)
}
