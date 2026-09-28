//! Nara's lived Day/Flow context (QL-MEF #258 EA1; V02 and V07 at the
//! deterministic level). A controlled, openly authored narrative stands in for
//! personal material.

use ql_mef::nara::dialogue::{AdmittedOccasion, DisclosureKind};
use ql_mef::nara::domain::*;
use ql_mef::nara::lived_context::*;
use ql_mef::nara::{ConsentState, SourceRevision};
use serde_json::json;

const T0: u64 = 1_790_500_000_000;
const DAY: u64 = 24 * 60 * 60 * 1000;
const EARLIER_DAY: &str = "central:source:controlled:Control/user/day/2026-09-01/day.md";
const TODAY: &str = "central:day:controlled:2026-09-03";

/// A `central.document-reading/v1` result in exactly the shape Central returns.
fn day_document(
    source_ref: &str,
    revision: &str,
    day_ref: &str,
    contributions: serde_json::Value,
) -> serde_json::Value {
    json!({
        "schema": "central.document-reading/v1",
        "source": {"source_ref": source_ref, "path": "Control/user/day/x/day.md"},
        "revision": {"revision": revision, "byte_len": 100},
        "document_id": "doc:day",
        "document": {
            "schema": "central.contribution-document/v1",
            "document_id": "doc:day",
            "kind": "day",
            "day_ref": day_ref,
            "fields": [], "entries": [], "operations": [],
            "contributions": contributions
        },
        "unreviewed_external_revision": false
    })
}

fn contribution(
    id: &str,
    entry: &str,
    actor: &str,
    html: &str,
    occurred: u64,
    received: u64,
) -> serde_json::Value {
    json!({
        "id": id, "entry_id": entry, "field_id": null, "html": html,
        "author_ref": if actor == "human" { "person:controlled-a" } else { "agent/nara" },
        "actor_kind": actor, "display_role": "H",
        "occurred_at_unix_seconds": occurred / 1000, "received_at_unix_seconds": received / 1000,
        "locked": false, "human_touched": actor == "human", "removed": false, "reviewed_by": null
    })
}

fn occasion() -> AdmittedOccasion {
    AdmittedOccasion {
        day_ref: TODAY.into(),
        day_revision: "day-rev-3".into(),
        now_ref: "central:now:controlled:session".into(),
        now_revision: "now-rev-1".into(),
        admitted_via_ref: "consent:controlled-a/lived-context".into(),
    }
}

fn journey_recognising(source_ref: &str, revision: &str, entry: &str) -> OracleJourney {
    let source = |path: &str, rev: &str| SourceRevision {
        source_ref: path.into(),
        revision: rev.into(),
        standing_ref: "controlled-narrative".into(),
    };
    let protected = |name: &str| ProtectedRef {
        ref_id: format!("protected:{name}"),
        revision: "r1".into(),
        owner_ref: "central".into(),
    };
    let mut journey = OracleJourney::open(
        OpenJourney {
            journey_ref: "journey:controlled-move".into(),
            subject_id: "controlled:person-a".into(),
            concern: JourneyConcern {
                concern_ref: "concern:the-move".into(),
                title: "The move".into(),
                basis_sources: vec![source("central:source:controlled:flow/move", "flow-rev-1")],
            },
            deck: DeckDefinition {
                deck_ref: "deck:thoth".into(),
                system: OracleSystem::TarotThoth,
                register: TarotRegister::Thoth,
                extra_cards: Vec::new(),
                source_refs: Vec::new(),
            },
            day_ref: "central:day:controlled:2026-09-01".into(),
            actor_ref: "agent/nara".into(),
            entropy: OracleEntropyReceipt {
                entropy_ref: protected("shuffle"),
                method_ref: "controlled-seed".into(),
                provider_ref: "provider:test".into(),
                observed_at_unix_ms: T0,
                evidence_refs: vec!["evidence:shuffle".into()],
            },
            consent: ConsentState::Granted,
            opened_at_unix_ms: T0,
        },
        &(0..1024)
            .map(|index| (index * 37 % 251) as u8)
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let effect = journey
        .apply(
            JourneyAct {
                day_ref: None,
                request_id: "d1".into(),
                actor_ref: "agent/nara".into(),
                at_unix_ms: T0,
                request: JourneyRequest::Draw {
                    spread: SpreadKind::Sphere,
                    day_ref: "central:day:controlled:2026-09-01".into(),
                    basis_sources: Vec::new(),
                },
            },
            &[],
        )
        .unwrap();
    journey
        .apply(
            JourneyAct {
                day_ref: None,
                request_id: "r1".into(),
                actor_ref: "person:controlled-a".into(),
                at_unix_ms: T0 + DAY,
                request: JourneyRequest::RelateEvent {
                    placement_ref: effect.effect_refs[1].clone(),
                    source: source(source_ref, revision),
                    entry_selector: Some(entry.into()),
                    relation: "the ground of the reading".into(),
                    occurred_at_unix_ms: T0 + DAY,
                },
            },
            &[],
        )
        .unwrap();
    journey
}

fn request(
    documents: Vec<serde_json::Value>,
    entries: Vec<LivedEntry>,
    journey: Option<OracleJourney>,
) -> LivedContextRequest {
    LivedContextRequest {
        subject_ref: "controlled:person-a".into(),
        concern: LivedConcern {
            concern_ref: "concern:the-move".into(),
            title: "The move".into(),
            terms: vec!["move".into(), "estate".into()],
        },
        occasion: Some(occasion()),
        documents,
        entries,
        journey,
        budget: LivedBudget {
            max_passages: 12,
            max_chars: 20_000,
        },
        disclosed_via_ref: "consent:controlled-a/lived-context".into(),
        composed_at_unix_ms: T0 + 2 * DAY,
    }
}

/// The decisive fact lives only in an earlier Day passage the journey relates
/// to its ground card; nothing in it names the concern's terms.
fn earlier_day(revision: &str, decisive: &str) -> serde_json::Value {
    day_document(
        EARLIER_DAY,
        revision,
        "central:day:controlled:2026-09-01",
        json!([
            contribution(
                "c1",
                "e1",
                "human",
                &format!("<p>{decisive}</p>"),
                T0 + DAY,
                T0 + DAY
            ),
            contribution(
                "c2",
                "e2",
                "human",
                "<p>Bought bread; the bus was late.</p>",
                T0 + DAY,
                T0 + DAY
            ),
        ]),
    )
}

#[test]
fn decisive_earlier_passage_reaches_the_context_through_the_journey() {
    let decisive = "My sister will only sign the papers if I stay until spring.";
    let context = compose(request(
        vec![earlier_day("rev-a", decisive)],
        Vec::new(),
        Some(journey_recognising(EARLIER_DAY, "rev-a", "e1")),
    ))
    .unwrap();
    let passage = context
        .passages
        .iter()
        .find(|passage| passage.text == decisive)
        .expect("the decisive passage is delivered");
    assert!(
        passage
            .reasons
            .iter()
            .any(|reason| matches!(reason, SelectionReason::RecognisesPlacement { .. }))
    );
    assert_eq!(passage.key, format!("{EARLIER_DAY}/doc:day/e1/c1@rev-a"));
    assert_eq!(passage.standing, EvidenceStanding::Reported);
    assert!(
        !context
            .passages
            .iter()
            .any(|passage| passage.text.contains("bread")),
        "unrelated material is not delivered"
    );
    assert!(
        context
            .disclosed
            .iter()
            .any(|disclosed| disclosed.ref_id == passage.key
                && disclosed.disclosure == DisclosureKind::PersonalConsent
                && disclosed.revision == "rev-a")
    );
    let journey = context.journey.as_ref().unwrap();
    assert_eq!(journey.dealt, 2);
    assert_eq!(context.occasion.as_ref().unwrap().day_ref, TODAY);

    // Same inputs, same basis: the delivered context can be checked.
    let again = compose(request(
        vec![earlier_day("rev-a", decisive)],
        Vec::new(),
        Some(journey_recognising(EARLIER_DAY, "rev-a", "e1")),
    ))
    .unwrap();
    assert_eq!(again.context_revision, context.context_revision);
}

#[test]
fn changing_or_removing_the_source_changes_the_supported_reading() {
    let decisive = "My sister will only sign the papers if I stay until spring.";
    let original = compose(request(
        vec![earlier_day("rev-a", decisive)],
        Vec::new(),
        Some(journey_recognising(EARLIER_DAY, "rev-a", "e1")),
    ))
    .unwrap();
    // The person rewrote the passage: the relation was to the old revision.
    let edited = compose(request(
        vec![earlier_day("rev-b", "My sister has already signed.")],
        Vec::new(),
        Some(journey_recognising(EARLIER_DAY, "rev-a", "e1")),
    ))
    .unwrap();
    assert!(
        !edited
            .passages
            .iter()
            .any(|passage| passage.text == decisive)
    );
    assert!(!edited.passages.iter().any(|passage| {
        passage
            .reasons
            .iter()
            .any(|reason| matches!(reason, SelectionReason::RecognisesPlacement { .. }))
    }));
    assert_ne!(edited.context_revision, original.context_revision);

    let removed = compose(request(
        Vec::new(),
        Vec::new(),
        Some(journey_recognising(EARLIER_DAY, "rev-a", "e1")),
    ))
    .unwrap();
    assert!(removed.passages.is_empty());
    assert_ne!(removed.context_revision, original.context_revision);
}

#[test]
fn a_human_correction_supersedes_an_agent_interpretation_without_erasing_it() {
    let interpretation = LivedEntry {
        source_ref: "central:source:controlled:flow/move".into(),
        revision: "flow-rev-1".into(),
        document_id: Some("doc:flow".into()),
        entry_id: Some("e7".into()),
        contribution_id: Some("c7".into()),
        selector: None,
        kind: MaterialKind::Flow,
        actor: LivedActor::Agent,
        author_ref: "agent/nara".into(),
        mode: MaterialMode::Current,
        text: "Nara: the move reads as grief for the estate.".into(),
        day_ref: Some("central:day:controlled:2026-09-02".into()),
        occurred_at_unix_ms: T0 + DAY,
        received_at_unix_ms: T0 + DAY,
        corrects: None,
        unreviewed_revision: false,
    };
    let correction = LivedEntry {
        entry_id: Some("e8".into()),
        contribution_id: Some("c8".into()),
        actor: LivedActor::Human,
        author_ref: "person:controlled-a".into(),
        text: "Not grief. I am relieved the argument about the estate is over.".into(),
        day_ref: Some(TODAY.into()),
        occurred_at_unix_ms: T0 + 2 * DAY,
        received_at_unix_ms: T0 + 2 * DAY,
        corrects: Some(interpretation.key()),
        ..interpretation.clone()
    };
    let context = compose(request(
        Vec::new(),
        vec![interpretation.clone(), correction.clone()],
        None,
    ))
    .unwrap();
    assert_eq!(context.passages.len(), 2);
    let first = &context.passages[0];
    assert_eq!(first.key, correction.key(), "the correction leads");
    assert!(first.reasons.contains(&SelectionReason::Correction {
        corrects: interpretation.key()
    }));
    let old = context
        .passages
        .iter()
        .find(|passage| passage.key == interpretation.key())
        .unwrap();
    assert_eq!(
        old.superseded_by.as_deref(),
        Some(correction.key().as_str())
    );
    assert_eq!(
        old.standing,
        EvidenceStanding::Derived,
        "agent interpretation is never autobiography"
    );
    assert_eq!(context.attribution.human, 1);
    assert_eq!(context.attribution.agent, 1);

    let dangling = LivedEntry {
        corrects: Some("central:source:nowhere@r".into()),
        ..correction
    };
    assert!(compose(request(Vec::new(), vec![interpretation, dangling], None)).is_err());
}

#[test]
fn central_document_mapping_is_exact_and_excludes_removed_material() {
    let mut removed = contribution("c3", "e3", "human", "<p>The move is off.</p>", T0, T0);
    removed["removed"] = json!(true);
    let mut document = day_document(
        EARLIER_DAY,
        "rev-a",
        TODAY,
        json!([
            contribution(
                "c1",
                "e1",
                "human",
                "<p>Packing for the <em>move</em> &amp; the estate.</p>",
                T0,
                T0 + 3 * DAY
            ),
            removed,
        ]),
    );
    document["unreviewed_external_revision"] = json!(true);
    let entries = entries_from_central_document(&document).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].text, "Packing for the move & the estate.");
    assert_eq!(entries[0].occurred_at_unix_ms, T0);
    let context = compose(request(vec![document], Vec::new(), None)).unwrap();
    let passage = &context.passages[0];
    assert!(
        passage.late_receipt,
        "received three days after it occurred"
    );
    assert!(passage.unreviewed_revision);
    assert!(passage.reasons.contains(&SelectionReason::CurrentDay));
    assert!(passage.reasons.contains(&SelectionReason::ConcernTerm {
        term: "move".into()
    }));
    assert!(
        !context
            .passages
            .iter()
            .any(|passage| passage.text.contains("is off"))
    );
}

#[test]
fn budget_omissions_are_visible_not_silent() {
    let entries: Vec<LivedEntry> = (0..5)
        .map(|index| LivedEntry {
            source_ref: "central:source:controlled:flow/move".into(),
            revision: "flow-rev-1".into(),
            document_id: Some("doc:flow".into()),
            entry_id: Some(format!("e{index}")),
            contribution_id: None,
            selector: None,
            kind: MaterialKind::Journal,
            actor: LivedActor::Human,
            author_ref: "person:controlled-a".into(),
            mode: MaterialMode::Remembered,
            text: format!("Remembered the estate, part {index}."),
            day_ref: None,
            occurred_at_unix_ms: T0 + index,
            received_at_unix_ms: T0 + index,
            corrects: None,
            unreviewed_revision: false,
        })
        .collect();
    let mut limited = request(Vec::new(), entries, None);
    limited.budget.max_passages = 2;
    let context = compose(limited).unwrap();
    assert_eq!(context.passages.len(), 2);
    assert_eq!(context.omitted.len(), 3);
    assert_eq!(context.candidates, 5);
    assert_eq!(
        context.passages[0].mode,
        MaterialMode::Remembered,
        "mode is kept as the author gave it"
    );
    assert!(context.passages[0].occurred_at_unix_ms > context.passages[1].occurred_at_unix_ms);
}
