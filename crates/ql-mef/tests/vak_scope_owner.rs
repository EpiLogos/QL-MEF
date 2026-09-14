//! Public-owner acceptance for #183: the operative binding must arise from an
//! actual VakComposition whole, not from a caller-constructed CompiledProfile.

use ql_core::{
    AnchorReturn, CallerProvenance, GroundKind, QlFace, QlFamily, QlPosition, QlShape,
    ShapeBinding, StructuralConstellation, StructuralParticipation,
};
use ql_mef::vak_composition::{ActiveFrame, Basis, PositionBasis, VakComposition, WholeInput};
use ql_mef::vak_profile::{
    CPrimeProfile, ContentPosition, ContentType, ContextSequence, InquiryDirection,
    Participation, ThreadForm,
};
use ql_mef::vak_scope::{
    CPrimeOperativeBinding, OperativeScopeCorrelation, OperativeScopeObservation,
};
use ql_mef::{ContextFrameId, LensId, MusicalBasis, VakRegistry, VakStanding};

fn basis(source: &str, revision: &str, evidence: &str) -> Basis {
    Basis {
        provenance: CallerProvenance::new(
            "test:ql-owner",
            source,
            VakStanding::AuthoredArchitecture.as_schema_str(),
        )
        .unwrap(),
        revision: revision.into(),
        evidence: vec![evidence.into()],
    }
}

fn frame() -> ActiveFrame {
    ActiveFrame {
        id: ContextFrameId::Cf5,
        lens: LensId::L0,
        basis: MusicalBasis::Chromatic,
        face: QlFace::Direct,
        positions: PositionBasis::Local,
    }
}

fn profile() -> CPrimeProfile {
    CPrimeProfile {
        participation: Participation::AuthorisedUndertaking,
        content: ContentType::Operations,
        position: ContentPosition::Operation,
        thread: ThreadForm::Chain,
        sequence: ContextSequence::ThroughOperation,
        direction: InquiryDirection::Forward,
    }
}

fn correlation(generation: &str) -> OperativeScopeCorrelation {
    OperativeScopeCorrelation {
        world_ref: "world/one".into(),
        world_generation: generation.into(),
        method_skill_ref: Some("skill/recognised/revisit".into()),
    }
}

fn owner_graph() -> VakComposition {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    let members: Vec<_> = [QlFace::Direct, QlFace::Conjugate]
        .into_iter()
        .flat_map(|face| {
            (0..6).map(move |position| {
                StructuralParticipation::new(
                    format!("member:{position}:{face:?}"),
                    QlPosition::new(position).unwrap(),
                    face,
                )
                .unwrap()
            })
        })
        .collect();
    let anchor = "anchor:undertaking";
    let ground = "ground:undertaking";
    let form = StructuralConstellation::new(
        anchor,
        members.clone(),
        vec![AnchorReturn::new(
            "source-result:undertaking",
            anchor,
            ground,
            QlFace::Direct,
            GroundKind::Own,
        )
        .unwrap()],
    )
    .unwrap();
    let source = basis("source:vak", "source-r1", "evidence:vak");
    let binding = ShapeBinding::new(
        "subject:nara",
        QlShape::Constellation(form.grain()).shape_ref(),
        anchor,
        members.iter().map(|member| member.subject_ref.clone()).collect(),
        members,
        vec![],
        None,
        None,
        vec![],
        source.provenance.clone(),
    )
    .unwrap();
    graph
        .bind_whole(
            &registry,
            WholeInput {
                use_ref: "whole:undertaking".into(),
                form,
                binding,
                category: QlFamily::M,
                ground_ref: ground.into(),
                ground_face: QlFace::Direct,
                frame: frame(),
                basis: source,
                language: None,
            },
        )
        .unwrap();
    graph
}

#[test]
fn public_owner_path_binds_and_reobserves_the_current_ql_whole() {
    let graph = owner_graph();
    let binding = graph
        .bind_operative_scope("whole:undertaking", profile(), correlation("generation-1"))
        .unwrap();

    assert_eq!(binding.whole_ref, "whole:undertaking");
    assert_eq!(binding.subject_ref, "subject:nara");
    assert_eq!(binding.sources.len(), 1);
    assert_eq!(binding.sources[0].source_ref, "source:vak");
    assert_eq!(binding.sources[0].revision, "source-r1");

    let wire = serde_json::to_string(&binding).unwrap();
    let decoded: CPrimeOperativeBinding = serde_json::from_str(&wire).unwrap();
    assert_eq!(decoded, binding);

    let current = graph
        .reobserve_operative_scope(
            &binding,
            "whole:undertaking",
            correlation("generation-1"),
        )
        .unwrap();
    let current_wire = serde_json::to_string(&current).unwrap();
    let current_decoded: OperativeScopeObservation =
        serde_json::from_str(&current_wire).unwrap();
    assert_eq!(current_decoded, current);
    assert!(matches!(
        current,
        OperativeScopeObservation::Current { binding: observed } if observed == binding
    ));

    let stale = graph
        .reobserve_operative_scope(
            &binding,
            "whole:undertaking",
            correlation("generation-2"),
        )
        .unwrap();
    assert!(matches!(
        stale,
        OperativeScopeObservation::Stale { differences, .. }
            if differences.contains(&"world-generation".to_string())
    ));
}

#[test]
fn client_replay_cannot_widen_the_ql_owned_source_basis() {
    let graph = owner_graph();
    let binding = graph
        .bind_operative_scope("whole:undertaking", profile(), correlation("generation-1"))
        .unwrap();
    let mut widened = binding.clone();
    let mut foreign = binding.sources[0].clone();
    foreign.source_ref = "source:foreign".into();
    foreign.revision = "foreign-r1".into();
    foreign.evidence_refs.insert("evidence:foreign".into());
    widened.sources.push(foreign);

    let observation = graph
        .reobserve_operative_scope(
            &widened,
            "whole:undertaking",
            correlation("generation-1"),
        )
        .unwrap();
    assert!(matches!(
        observation,
        OperativeScopeObservation::Stale { differences, .. }
            if differences.contains(&"source-basis".to_string())
    ));
}
