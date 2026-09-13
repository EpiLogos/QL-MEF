use super::*;

fn leg(id: &str) -> PlannedLeg {
    PlannedLeg {
        unit_ref: id.into(),
        subject_ref: "concern".into(),
        scope_ref: format!("scope:{id}"),
        input_refs: vec![format!("source:{id}")],
        result_ref: format!("return:{id}"),
        after: vec![],
        parent: None,
    }
}
fn plan(ids: &[&str]) -> ThreadPlan {
    ThreadPlan {
        legs: ids.iter().map(|id| leg(id)).collect(),
        aggregation_ref: None,
        continuation_ref: None,
        stop_condition_ref: None,
    }
}

#[test]
fn all_paired_passages_retain_direction_and_extent() {
    let expected = [
        vec![(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)],
        vec![(0, 5), (1, 4)],
        vec![(0, 5), (1, 4), (2, 3)],
        vec![(0, 5), (1, 4), (2, 3), (3, 2)],
        vec![(0, 5), (4, 1), (5, 0)],
        vec![(0, 5), (5, 0)],
    ];
    for (sequence, expected) in ContextSequence::ALL.into_iter().zip(expected) {
        let pairs: Vec<_> = expected
            .into_iter()
            .map(|(a, b)| PositionPair {
                forward: ContentPosition::ALL[a],
                returning: ContentPosition::ALL[b],
            })
            .collect();
        assert_eq!(sequence.pairs(), pairs);
        assert_eq!(
            sequence.walk(InquiryDirection::Forward),
            pairs.iter().map(|p| p.forward).collect::<Vec<_>>()
        );
        assert_eq!(
            sequence.walk(InquiryDirection::Returning),
            pairs.iter().map(|p| p.returning).collect::<Vec<_>>()
        );
    }
}

#[test]
fn all_thread_forms_have_distinct_executable_obligations() {
    let single = plan(&["one"]);
    ThreadForm::Single
        .validate_plan("concern", &single)
        .unwrap();
    let parallel = plan(&["one", "two"]);
    ThreadForm::Parallel
        .validate_plan("concern", &parallel)
        .unwrap();
    let mut chain = parallel.clone();
    chain.legs[1].after.push("one".into());
    chain.legs[1].input_refs.push("return:one".into());
    ThreadForm::Chain.validate_plan("concern", &chain).unwrap();
    assert!(
        ThreadForm::Parallel
            .validate_plan("concern", &chain)
            .is_err()
    );
    let mut fusion = parallel.clone();
    fusion.aggregation_ref = Some("aggregate".into());
    ThreadForm::Fusion
        .validate_plan("concern", &fusion)
        .unwrap();
    let mut sustained = single.clone();
    sustained.continuation_ref = Some("checkpoint".into());
    sustained.stop_condition_ref = Some("stop".into());
    ThreadForm::Sustained
        .validate_plan("concern", &sustained)
        .unwrap();
    let mut nested = parallel;
    nested.legs[1].parent = Some("one".into());
    ThreadForm::Nested
        .validate_plan("concern", &nested)
        .unwrap();
    let roles: BTreeSet<_> = ThreadForm::ALL.iter().map(|f| f.musical_role()).collect();
    assert_eq!(roles.len(), 6);
}

#[test]
fn content_and_participation_are_not_time_or_authority_aliases() {
    assert!(ContentType::Contexts.accepts(ContentType::DayNow));
    assert!(!ContentType::DayNow.accepts(ContentType::Contexts));
    assert!(!ContentType::Operations.accepts(ContentType::Patterns));
    assert!(Participation::AuthorisedUndertaking.requires_undertaking());
    assert!(!Participation::Dialogical.requires_undertaking());
    assert_eq!(constitutional_voice(ContextFrameId::Cf5), "Anima");
    assert_eq!(constitutional_voice(ContextFrameId::Cf7), "Sophia");
}

#[test]
fn lost_chain_input_and_fused_subject_changes_are_rejected() {
    let mut chain = plan(&["one", "two"]);
    chain.legs[1].after.push("one".into());
    assert!(ThreadForm::Chain.validate_plan("concern", &chain).is_err());
    let mut fusion = plan(&["one", "two"]);
    fusion.aggregation_ref = Some("return:one".into());
    assert!(
        ThreadForm::Fusion
            .validate_plan("concern", &fusion)
            .is_err()
    );
    fusion.aggregation_ref = Some("aggregate".into());
    fusion.legs[1].subject_ref = "another-concern".into();
    assert!(
        ThreadForm::Fusion
            .validate_plan("concern", &fusion)
            .is_err()
    );
    assert!(
        ThreadForm::Sustained
            .validate_plan("concern", &plan(&["one"]))
            .is_err()
    );
}

#[test]
fn malformed_or_aliased_native_topology_fails_closed() {
    let mut cyclic = plan(&["one", "two"]);
    cyclic.legs[0].after.push("two".into());
    cyclic.legs[1].after.push("one".into());
    assert!(ThreadForm::Chain.validate_plan("concern", &cyclic).is_err());
    let mut parent_cycle = plan(&["root", "one", "two"]);
    parent_cycle.legs[1].parent = Some("two".into());
    parent_cycle.legs[2].parent = Some("one".into());
    assert!(
        ThreadForm::Nested
            .validate_plan("concern", &parent_cycle)
            .is_err()
    );
    let mut alias = plan(&["one", "two"]);
    alias.legs[1].result_ref = alias.legs[0].result_ref.clone();
    assert!(
        ThreadForm::Parallel
            .validate_plan("concern", &alias)
            .is_err()
    );
    assert!(
        ThreadForm::Single
            .validate_plan("concern", &plan(&[]))
            .is_err()
    );
    let mut unknown = plan(&["one"]);
    unknown.legs[0].after.push("absent".into());
    assert!(
        ThreadForm::Single
            .validate_plan("concern", &unknown)
            .is_err()
    );
}

#[test]
fn every_profile_axis_roundtrips_without_inventing_a_frame_count() {
    for content in ContentType::ALL {
        for thread in ThreadForm::ALL {
            for sequence in ContextSequence::ALL {
                for direction in [InquiryDirection::Forward, InquiryDirection::Returning] {
                    let profile = CPrimeProfile {
                        participation: Participation::Dialogical,
                        content,
                        position: ContentPosition::Ground,
                        thread,
                        sequence,
                        direction,
                    };
                    let encoded = serde_json::to_string(&profile).unwrap();
                    let decoded: CPrimeProfile = serde_json::from_str(&encoded).unwrap();
                    assert_eq!(decoded, profile);
                }
            }
        }
    }
    assert!(serde_json::from_str::<ContentPosition>("\"4.6\"").is_err());
    assert!(serde_json::from_str::<ContextSequence>("\"session\"").is_err());
    assert!(serde_json::from_str::<ThreadForm>("\"Z\"").is_err());
}
