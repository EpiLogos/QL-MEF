//! Synthetic admission tests. None of the observation records below claims
//! that this test executed Factory or an external Agent.
use ql_core::*;
use ql_mef::vak_composition::*;
use ql_mef::*;

fn basis(source: &str) -> Basis {
    Basis {
        provenance: CallerProvenance::new("test:caller", source, "PROPOSED").unwrap(),
        revision: "test-v1".into(),
        evidence: vec!["test:synthetic".into()],
    }
}
fn frame() -> ActiveFrame {
    ActiveFrame {
        id: ContextFrameId::Cf2,
        lens: LensId::L0,
        basis: MusicalBasis::Chromatic,
        face: QlFace::Direct,
        positions: PositionBasis::Local,
    }
}
fn local(g: &mut VakComposition, r: &VakRegistry, name: &str) {
    let members: Vec<_> = [QlFace::Direct, QlFace::Conjugate]
        .into_iter()
        .flat_map(|face| (0..6).map(move |p| (face, p)))
        .map(|(face, p)| {
            StructuralParticipation::new(
                format!("{name}:{p}:{face}"),
                QlPosition::new(p).unwrap(),
                face,
            )
            .unwrap()
        })
        .collect();
    let form =
        StructuralConstellation::new(format!("anchor:{name}"), members.clone(), vec![]).unwrap();
    let b = basis(name);
    let binding = ShapeBinding::new(
        name,
        QlShape::Constellation(form.grain()).shape_ref(),
        &form.anchor_ref,
        members.iter().map(|m| m.subject_ref.clone()).collect(),
        members,
        vec![],
        None,
        None,
        vec![],
        b.provenance.clone(),
    )
    .unwrap();
    g.bind_whole(
        r,
        WholeInput {
            use_ref: name.into(),
            form,
            binding,
            category: QlFamily::M,
            ground_ref: format!("ground:{name}"),
            ground_face: QlFace::Direct,
            frame: frame(),
            basis: b,
            language: None,
        },
    )
    .unwrap();
}
fn reading() -> VakExpressionReadingV1 {
    VakExpressionReadingV1 {
        contract: VAK_EXPRESSION_READING_CONTRACT,
        operator: VakRelationOp::Express,
        horizon: VakAddressHorizon::H5,
        subjects: vec![VakExpressionSubject::Native("a".into())],
        relation_refs: vec![SelfOtherForm::Statement.source_ref()],
        complement_refs: vec![],
        world_ref: Some("test:world".into()),
        project_ref: None,
        focus_ref: Some("test:focus".into()),
        expected_return: None,
        standing: VakStanding::Derived,
        evidence: vec!["test:reading-evidence".into()],
    }
}
fn language() -> FullVakBinding {
    FullVakBinding {
        general: VakGeneralExpressionEvidence {
            syntax_version: AIKIT_OPERATIVE_SYNTAX_VERSION.into(),
            owner_revision: "test:syntax-revision".into(),
            resolve_path_identity: "test:resolve-path".into(),
            rendered: "test:AST-rendering".into(),
            full_vak_rendering: "test:full-rendering".into(),
            evidence: vec!["test:AST-evidence".into()],
        },
        accepted_syntax_revision: "test:syntax-revision".into(),
        native_node_ref: "test:AST-node".into(),
        reading: reading(),
        self_other: SelfOtherForm::Statement,
        field: VakContextField::Techne,
        interpreter: "test:Agent".into(),
        expected_ground: Some("ground:a".into()),
    }
}
fn observation(r: &VakRegistry) -> VakExecutionObservationV1 {
    let mut expression = reading();
    expression.standing = VakStanding::Observed;
    let profile = factory_request_evidence_profile(r).unwrap();
    let action = profile.action_ref.clone();
    VakExecutionObservationV1 {
        observation_ref: "test:synthetic-observation".into(),
        owner_revision: profile.binding_revision.clone(),
        evidence_run_ref: "test:synthetic-run".into(),
        method_ref: "test:native-method".into(),
        general_expression: language().general,
        world_ref: expression.world_ref.clone(),
        project_ref: None,
        focus_ref: expression.focus_ref.clone(),
        actor_ref: Some("test:Agent".into()),
        agency_ref: Some("test:Agency".into()),
        action_profile: profile,
        steps: vec![VakPathStepV1 {
            step_id: "test:native-step".into(),
            expression,
            native_subject_refs: vec!["a".into()],
            method_ref: Some("test:native-method".into()),
            action_ref: Some(action),
            invocation_ref: Some("test:invocation".into()),
            activity_ref: Some("test:activity".into()),
            result_refs: vec!["test:Agent-result".into()],
            return_ref: Some("test:native-return".into()),
            source_surface: None,
            evidence_refs: vec!["test:step-evidence".into()],
            standing: VakStanding::Observed,
        }],
        evidence_refs: vec!["test:observation-evidence".into()],
        standing: VakStanding::Observed,
    }
}
#[test]
fn native_paths_keep_source_reading_actor_result_and_return_correlation() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut g = VakComposition::default();
    local(&mut g, &registry, "a");
    g.determine(
        &registry,
        DetermineInput {
            reference: "d".into(),
            whole_use: "a".into(),
            viewing_lens: LensId::L0,
            language: Some(language()),
            contribution: Some(AgentContribution {
                actor_ref: "test:Agent".into(),
                result_ref: "test:Agent-result".into(),
                input_refs: vec!["a".into()],
                evidence: vec!["test:Agent-evidence".into()],
            }),
            basis: basis("test:produce"),
        },
    )
    .unwrap();
    let input = NativePathInput {
        reference: "correlation".into(),
        determination: "d".into(),
        native_step_ref: "test:native-step".into(),
        observation: observation(&registry),
        basis: basis("test:correlation"),
    };
    for change in 0..4 {
        let mut bad = input.clone();
        match change {
            0 => bad.observation.actor_ref = Some("test:other-Actor".into()),
            1 => bad.observation.steps[0].result_refs = vec!["test:other-result".into()],
            2 => bad.observation.steps[0].action_ref = Some("test:other-Action".into()),
            _ => bad.observation.steps[0].expression.world_ref = Some("test:other-world".into()),
        }
        assert!(g.record_native_observation(&registry, bad).is_err());
        assert!(g.determination("d").unwrap().native_paths.is_empty());
    }
    let correlated = g.record_native_observation(&registry, input).unwrap();
    assert_eq!(correlated.standing, VakStanding::Derived);
    assert_eq!(correlated.native_node_ref, "test:AST-node");
    assert_eq!(correlated.native_step_ref, "test:native-step");
    assert_eq!(correlated.path.actor_ref.as_deref(), Some("test:Agent"));
    g.return_result(ReturnInput {
        reference: "returned".into(),
        determination: "d".into(),
        target_use: "a".into(),
        kind: GroundKind::Own,
        relation_evidence: None,
        basis: basis("test:Return"),
    })
    .unwrap();
    assert_eq!(
        g.returned("returned").unwrap().producing.native_paths[0],
        correlated
    );
    assert_eq!(
        g.returned("returned").unwrap().producing.standing,
        VakStanding::Derived
    );
}
#[test]
fn frame_and_lens_changes_keep_the_explicit_cp_member_and_parent_relation() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut g = VakComposition::default();
    local(&mut g, &registry, "a");
    local(&mut g, &registry, "b");
    g.compose(
        &registry,
        ComposeInput {
            use_ref: "parent".into(),
            whole_ref: "anchor:parent".into(),
            row: "a".into(),
            column: "b".into(),
            frame: frame(),
            ground_ref: "ground:parent".into(),
            ground_face: QlFace::Direct,
            basis: basis("test:compose"),
            language: None,
        },
    )
    .unwrap();
    g.position_member(
        "a",
        "a-focused",
        MemberFocus {
            coordinate: QlCoordinate::new(QlPosition::new(4).unwrap(), QlFace::Direct),
            positions: PositionBasis::Absolute,
            basis: basis("test:CP"),
        },
    )
    .unwrap();
    let before = g.read("a-focused", LensId::L0).unwrap();
    let mut f = frame();
    f.lens = LensId::L2Prime;
    f.id = ContextFrameId::Cf4;
    g.reframe("a-focused", "a-reframed", f, basis("test:CF"))
        .unwrap();
    let after = g.read("a-reframed", LensId::L5).unwrap();
    assert_eq!(before.address, after.address);
    assert_eq!(g.whole("a-reframed").unwrap().binding.subject_ref, "a");
    g.determine(
        &registry,
        DetermineInput {
            reference: "d".into(),
            whole_use: "a-reframed".into(),
            viewing_lens: LensId::L5,
            language: None,
            contribution: None,
            basis: basis("test:derive"),
        },
    )
    .unwrap();
    g.return_result(ReturnInput {
        reference: "r".into(),
        determination: "d".into(),
        target_use: "parent".into(),
        kind: GroundKind::Parent,
        relation_evidence: None,
        basis: basis("test:return-parent"),
    })
    .unwrap();
    assert_eq!(
        g.returned("r").unwrap().route.through_anchor_ref,
        "anchor:a"
    );
    assert_eq!(
        g.returned("r").unwrap().route.target_ground_ref,
        "ground:parent"
    );
}
