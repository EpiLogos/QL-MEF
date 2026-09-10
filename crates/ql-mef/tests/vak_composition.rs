//! QL-native synthetic specimens; these are not native Agent execution receipts.
use ql_core::*;
use ql_mef::*;
use ql_mef::vak_composition::*;

fn basis(source: &str) -> Basis {
    Basis { provenance: CallerProvenance::new("test:ql", source, "PROPOSED").unwrap(),
        revision: "synthetic-v1".into(), evidence: vec!["test:synthetic-input".into()] }
}
fn frame(id: ContextFrameId) -> ActiveFrame {
    ActiveFrame { id, lens: LensId::L0, basis: MusicalBasis::Chromatic,
        face: QlFace::Direct, positions: PositionBasis::Local }
}
fn language(subject: &str, op: VakRelationOp, field: VakContextField) -> FullVakBinding {
    FullVakBinding {
        general: VakGeneralExpressionEvidence { syntax_version: AIKIT_OPERATIVE_SYNTAX_VERSION.into(),
            owner_revision: "synthetic-owner-revision".into(), resolve_path_identity: "test:resolve-path".into(),
            rendered: "test supplied AST rendering".into(), full_vak_rendering: "test supplied full rendering".into(),
            evidence: vec!["test:not-an-AIKit-execution".into()] },
        accepted_syntax_revision: "synthetic-owner-revision".into(), native_node_ref: format!("test:node:{subject}"),
        reading: VakExpressionReadingV1 { contract: VAK_EXPRESSION_READING_CONTRACT,
            operator: op, horizon: VakAddressHorizon::H5,
            subjects: vec![VakExpressionSubject::Native(subject.into())], relation_refs: vec![], complement_refs: vec![],
            world_ref: Some("test:world".into()), project_ref: None, focus_ref: None, expected_return: None,
            standing: VakStanding::Derived, evidence: vec!["test:interpretation".into()] },
        self_other: SelfOtherForm::Statement, field, interpreter: "test:interpreter".into(), expected_ground: None,
    }
}
fn local(graph: &mut VakComposition, registry: &VakRegistry, name: &str, cf: ContextFrameId, lang: bool) {
    let mut members = Vec::new();
    for face in [QlFace::Direct, QlFace::Conjugate] {
        for p in 0..6 { members.push(StructuralParticipation::new(format!("{name}:{p}:{face}"), QlPosition::new(p).unwrap(), face).unwrap()); }
    }
    let anchor = format!("anchor:{name}");
    let ground = format!("ground:{name}");
    let form = StructuralConstellation::new(&anchor, members.clone(), vec![
        AnchorReturn::new(format!("source-result:{name}"), &anchor, &ground, QlFace::Direct, GroundKind::Own).unwrap()
    ]).unwrap();
    let b = basis(&format!("source:{name}"));
    let binding = ShapeBinding::new(name, QlShape::Constellation(form.grain()).shape_ref(), &anchor,
        members.iter().map(|m| m.subject_ref.clone()).collect(), members, vec![], None, None, vec![], b.provenance.clone()).unwrap();
    graph.bind_whole(registry, WholeInput { use_ref: name.into(), form, binding, category: QlFamily::M,
        ground_ref: ground, ground_face: QlFace::Direct, frame: frame(cf), basis: b,
        language: lang.then(|| language(name, VakRelationOp::Affirm, VakContextField::Techne)) }).unwrap();
}
fn compose(graph: &mut VakComposition, registry: &VakRegistry, name: &str, row: &str, column: &str, cf: ContextFrameId) {
    graph.compose(registry, ComposeInput { use_ref: name.into(), whole_ref: format!("anchor:{name}"),
        row: row.into(), column: column.into(), frame: frame(cf), ground_ref: format!("ground:{name}"),
        ground_face: QlFace::Direct, basis: basis(&format!("composition:{name}")), language: None }).unwrap();
}
fn request(name: &str, whole: &str) -> DetermineInput {
    DetermineInput { reference: name.into(), whole_use: whole.into(), viewing_lens: LensId::L5,
        language: None, contribution: None, basis: basis("test:determination") }
}
fn returning(name: &str, d: &str, target: &str, kind: GroundKind) -> ReturnInput {
    ReturnInput { reference: name.into(), determination: d.into(), target_use: target.into(), kind,
        relation_evidence: None, basis: basis("test:Return") }
}

#[test]
fn local_to_recursive_field_reframes_operatively_and_returns_as_a_whole() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "a", ContextFrameId::Cf2, true);
    local(&mut graph, &registry, "b", ContextFrameId::Cf3, false);
    local(&mut graph, &registry, "c", ContextFrameId::Cf4, false);
    compose(&mut graph, &registry, "ab", "a", "b", ContextFrameId::Cf1);
    compose(&mut graph, &registry, "outer", "ab", "c", ContextFrameId::Cf5);
    let original = graph.read("outer", LensId::L5).unwrap();
    assert_eq!(original.children[0].children[0].binding.subject_ref, "a");
    assert_eq!(original.carrier_derivations.len(), 1);
    assert_eq!(original.source_returns.len(), 3);
    graph.reframe("outer", "outer-reframed", frame(ContextFrameId::Cf2), basis("test:reframe")).unwrap();
    let changed = graph.read("outer-reframed", LensId::L1Prime).unwrap();
    assert_ne!(original.child_intervals, changed.child_intervals);
    assert_ne!(original.geometry.child_phase_degrees, changed.geometry.child_phase_degrees);
    assert_eq!(original.children[0].frame, changed.children[0].frame);
    assert_eq!(graph.read("outer", LensId::L5).unwrap(), original);
    assert_eq!(graph.position("outer", &[Axis::Row, Axis::Row]).unwrap().use_ref, "a");
    graph.determine(&registry, request("d", "outer-reframed")).unwrap();
    graph.return_result(returning("r", "d", "outer-reframed", GroundKind::Own)).unwrap();
    let returned = graph.returned("r").unwrap();
    assert_eq!(returned.route.through_anchor_ref, "anchor:outer");
    assert_eq!(returned.route.target_ground_ref, "ground:outer");
    assert_eq!(returned.producing.sources.len(), graph.determination("d").unwrap().sources.len());
    let mut derived = basis("r"); derived.provenance.standing_ref = "DERIVED".into();
    graph.offer_as_whole("r", "new-whole", "anchor:new-whole", derived).unwrap();
    compose(&mut graph, &registry, "next", "new-whole", "a", ContextFrameId::Cf7);
    assert_eq!(graph.position("next", &[Axis::Row]).unwrap().binding.subject_ref, "d");
    graph.determine(&registry, request("d-next", "next")).unwrap();
    assert!(!graph.determination("d-next").unwrap().sources.is_empty());
    assert_eq!(graph.determination("d-next").unwrap().standing, VakStanding::Derived);
    assert!(matches!(graph.whole("new-whole").unwrap().body, WholeBody::Relation { .. }));
}

#[test]
fn all_six_reflective_families_have_distinct_operative_effects() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "a", ContextFrameId::Cf2, true);
    local(&mut graph, &registry, "b", ContextFrameId::Cf3, false);
    compose(&mut graph, &registry, "ab", "a", "b", ContextFrameId::Cf5);
    let mut c = CPrimeContext::enter(&graph, "ab", QlCoordinate::new(QlPosition::new(4).unwrap(), QlFace::Direct)).unwrap();
    c.cp(&graph, &[Axis::Row], basis("test:cp")).unwrap();
    assert_eq!(c.focus_use, "a");
    c.cpf(&mut graph, "a-prime", QlFace::Conjugate, VakRelationOp::ALL.to_vec(), basis("test:cpf")).unwrap();
    let selected = graph.read("a-prime", LensId::L0).unwrap();
    assert!(matches!(selected.address, SelectedAddress::Member { member, .. } if member.coordinate.face == QlFace::Conjugate));
    c.ct(vec![VakContextField::Techne], basis("test:ct")).unwrap();
    let mut f = frame(ContextFrameId::Cf4); f.face = QlFace::Conjugate;
    c.cf(&mut graph, "a-new-frame", f, basis("test:cf")).unwrap();
    let path = registry.r_path(VakDivineAct::Freedom).unwrap();
    let bindings: Vec<_> = path.steps.iter().map(|s| ThreadBinding { source_step: s.vak_ref.clone(), path: vec![] }).collect();
    let thread = c.cfp(&graph, &registry, &bindings, VakDivineAct::Freedom, basis("test:cfp")).unwrap();
    assert_eq!(thread[0].use_ref, "a-new-frame");
    let mut req = request("derived", "a-new-frame");
    req.language = Some(language("a", VakRelationOp::Express, VakContextField::Techne));
    req.contribution = Some(AgentContribution { actor_ref: "test:Agent".into(), result_ref: "test:proposed-text".into(),
        input_refs: vec!["a-new-frame".into()], evidence: vec!["test:supplied-not-observed".into()] });
    c.determine(&mut graph, &registry, req).unwrap();
    c.cs(&mut graph, returning("returned", "derived", "a-new-frame", GroundKind::Own)).unwrap();
    for family in VakFamily::ALL {
        assert!(c.operations.iter().any(|op| op.family == family && op.relation_id == family.relation_id()));
    }
    let d = graph.determination("derived").unwrap();
    assert_eq!(d.context.as_ref().unwrap().r_path.as_ref().unwrap().act, VakDivineAct::Freedom);
    assert_eq!(d.context.as_ref().unwrap().readings.len(), bindings.len());
    assert_eq!(d.contribution.as_ref().unwrap().actor_ref, "test:Agent");
    assert_eq!(graph.returned("returned").unwrap().producing.standing, VakStanding::Derived);
}

#[test]
fn cpf_ct_and_source_path_refusals_are_atomic_and_consequential() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "a", ContextFrameId::Cf2, true);
    let mut c = CPrimeContext::enter(&graph, "a", frame(ContextFrameId::Cf2).coordinate()).unwrap();
    c.ct(vec![VakContextField::Bimba], basis("test:exclude-techne")).unwrap();
    assert!(c.determine(&mut graph, &registry, request("refused-ct", "a")).is_err());
    assert!(graph.determination("refused-ct").is_err());
    c.ct(vec![VakContextField::Techne], basis("test:admit-techne")).unwrap();
    c.cpf(&mut graph, "n", QlFace::Direct, vec![VakRelationOp::Affirm], basis("test:withhold-expression")).unwrap();
    assert!(c.determine(&mut graph, &registry, request("refused-cpf", "n")).is_err());
    let before = c.operations.len();
    let bad = vec![ThreadBinding { source_step: SelfOtherForm::ActualIdentity.source_ref(), path: vec![] }];
    assert!(c.cfp(&graph, &registry, &bad, VakDivineAct::Freedom, basis("test:bad-r-path")).is_err());
    assert_eq!(c.operations.len(), before);
}

#[test]
fn return_ground_relations_are_checked_not_guessed_from_position_five() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut g = VakComposition::default();
    local(&mut g, &registry, "a", ContextFrameId::Cf2, false);
    local(&mut g, &registry, "b", ContextFrameId::Cf3, false);
    compose(&mut g, &registry, "ab", "a", "b", ContextFrameId::Cf5);
    g.determine(&registry, request("da", "a")).unwrap();
    g.return_result(returning("parent", "da", "ab", GroundKind::Parent)).unwrap();
    assert_eq!(g.returned("parent").unwrap().route.through_anchor_ref, "anchor:a");
    assert!(g.return_result(returning("wrong-parent", "da", "b", GroundKind::Parent)).is_err());
    g.determine(&registry, request("dab", "ab")).unwrap();
    g.return_result(returning("child", "dab", "a", GroundKind::Child)).unwrap();
    assert!(g.return_result(returning("other-no-evidence", "da", "b", GroundKind::Other)).is_err());
    let mut other = returning("other", "da", "b", GroundKind::Other);
    other.relation_evidence = Some("test:explicit-other-relation".into());
    g.return_result(other).unwrap();
    let mut promoted = basis("other"); promoted.provenance.standing_ref = "SOURCE".into();
    assert!(g.offer_as_whole("other", "forged", "anchor:forged", promoted).is_err());
}

#[test]
fn source_registry_and_local_absolute_mef_rotation_survive() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    assert_eq!(registry.len(), 109);
    for id in ContextFrameId::ALL {
        for lens in LensId::ALL {
            let mut graph = VakComposition::default();
            local(&mut graph, &registry, "a", id, false);
            let mut f = frame(id); f.lens = lens; f.positions = PositionBasis::Absolute;
            graph.reframe("a", "at-lens", f, basis("test:rotation")).unwrap();
            let first = graph.read("at-lens", LensId::L0).unwrap();
            let other = graph.read("at-lens", LensId::L5Prime).unwrap();
            assert_eq!(first.frame, other.frame);
            assert_eq!(first.harmonic_pitch, other.harmonic_pitch);
            assert_eq!(first.geometry.absolute_position, other.geometry.viewing_rotation.absolute_position());
            assert_eq!(first.address, other.address);
        }
    }
    assert!(ql_mef::l5_projection::lookup_context_frame_expression("(4/5/0)").is_none());
}

#[test]
fn recursive_shape_grouping_does_not_collapse_and_exponential_reads_are_bounded() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    for n in ["a", "b", "c"] { local(&mut graph, &registry, n, ContextFrameId::Cf2, false); }
    compose(&mut graph, &registry, "ab", "a", "b", ContextFrameId::Cf2);
    compose(&mut graph, &registry, "bc", "b", "c", ContextFrameId::Cf2);
    compose(&mut graph, &registry, "left", "ab", "c", ContextFrameId::Cf2);
    compose(&mut graph, &registry, "right", "a", "bc", ContextFrameId::Cf2);
    assert_ne!(graph.whole("left").unwrap().binding.shape_ref, graph.whole("right").unwrap().binding.shape_ref);
    let mut last = "a".to_owned();
    for i in 0..14 { let next = format!("double:{i}"); compose(&mut graph, &registry, &next, &last, &last, ContextFrameId::Cf2); last = next; }
    assert!(graph.read(&last, LensId::L0).unwrap_err().0.contains("bound"));
    let bad = ComposeInput { use_ref: "self".into(), whole_ref: "self".into(), row: "self".into(), column: "a".into(),
        frame: frame(ContextFrameId::Cf2), ground_ref: "ground:self".into(), ground_face: QlFace::Direct,
        basis: basis("test:cycle"), language: None };
    assert!(graph.compose(&registry, bad).is_err());
}

#[test]
fn arbitrary_full_profile_refs_and_expected_ground_are_preserved() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "a", ContextFrameId::Cf2, true);
    let mut req = request("d", "a");
    let mut l = language("a", VakRelationOp::Express, VakContextField::Techne);
    l.reading.relation_refs.push(VakRef::new("M0").unwrap());
    l.expected_ground = Some("ground:elsewhere".into());
    req.language = Some(l);
    graph.determine(&registry, req).unwrap();
    assert!(graph.determination("d").unwrap().sources.iter().any(|s| s.coordinate.as_str() == "M0"));
    assert!(graph.return_result(returning("wrong", "d", "a", GroundKind::Own)).is_err());
    assert!(graph.returned("wrong").is_err());
}
