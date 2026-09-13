use ql_mef::m_tree::native_m_registry;
use ql_wiki::{
    BimbaFace, MappingOrigin, MetaKnowledgeProjection, MetaProvenance, ParticipationForm,
    ParticipationMember, ParticipationReferent, ParticipationRole, ProjectedObject,
    WIKI_PARTICIPATION_CONTRACT, WikiParticipation, bimba_participant, compile_participation,
};

fn projection() -> MetaKnowledgeProjection {
    MetaKnowledgeProjection {
        projection_version: 1,
        objects: vec![
            ProjectedObject {
                projection_id: 1,
                canonical_ref: "wiki:a".into(),
                object_kind: "node".into(),
                revision: 1,
            },
            ProjectedObject {
                projection_id: 2,
                canonical_ref: "wiki:b".into(),
                object_kind: "node".into(),
                revision: 1,
            },
            ProjectedObject {
                projection_id: 3,
                canonical_ref: "wiki:c".into(),
                object_kind: "node".into(),
                revision: 1,
            },
        ],
        relations: vec![],
        meta_bindings: vec![],
    }
}

fn wiki(reference: &str) -> ParticipationMember {
    ParticipationMember {
        role: ParticipationRole::Member,
        referent: ParticipationReferent::Wiki {
            canonical_ref: reference.into(),
            revision: 1,
        },
    }
}

fn participation(
    reference: &str,
    form: ParticipationForm,
    members: Vec<ParticipationMember>,
) -> WikiParticipation {
    WikiParticipation {
        contract: WIKI_PARTICIPATION_CONTRACT.into(),
        participation_ref: reference.into(),
        revision: 1,
        form,
        origin: MappingOrigin::Authored,
        members,
        provenance: vec![MetaProvenance {
            source_ref: "docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md".into(),
            source_revision: Some("owner-ratified-2026-09-12".into()),
        }],
    }
}

#[test]
fn all_five_source_defined_wiki_participation_forms_compile() {
    let registry = native_m_registry();
    let projection = projection();
    let direct = bimba_participant(
        registry,
        "#0",
        BimbaFace::Direct,
        ParticipationRole::Direct,
    )
    .unwrap();
    let conjugate = bimba_participant(
        registry,
        "#0",
        BimbaFace::Conjugate,
        ParticipationRole::Conjugate,
    )
    .unwrap();

    let cases = [
        participation(
            "wiki:constellation/abc",
            ParticipationForm::Constellation,
            vec![wiki("wiki:a"), wiki("wiki:b"), wiki("wiki:c")],
        ),
        participation(
            "wiki:pair/ab",
            ParticipationForm::Pair,
            vec![wiki("wiki:a"), wiki("wiki:b")],
        ),
        participation(
            "wiki:triad/abc",
            ParticipationForm::Triad,
            vec![wiki("wiki:a"), wiki("wiki:b"), wiki("wiki:c")],
        ),
        participation(
            "wiki:whole/a",
            ParticipationForm::Whole,
            vec![wiki("wiki:a")],
        ),
        participation(
            "wiki:direct-conjugate/m0",
            ParticipationForm::DirectConjugate,
            vec![direct, conjugate],
        ),
    ];

    let compiled = cases
        .iter()
        .map(|case| compile_participation(&projection, registry, case).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(compiled.len(), 5);
    assert_eq!(compiled[0].members.len(), 3);
    assert_eq!(compiled[1].members.len(), 2);
    assert_eq!(compiled[2].members.len(), 3);
    assert_eq!(compiled[3].members.len(), 1);
    assert_eq!(compiled[4].members.len(), 2);
}
