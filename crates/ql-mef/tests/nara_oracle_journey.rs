//! Continuing single-deck Tarot / I-Ching journey (QL-MEF #258 EA2; V03, V04,
//! V05, V07 at the deterministic level). Expected values are derived here from
//! the protocol and the C line law, not read back from the implementation.

use ql_mef::nara::domain::*;
use ql_mef::nara::{ConsentState, SourceRevision};
use sha2::{Digest, Sha256};

const T0: u64 = 1_790_500_000_000;
const DAY: u64 = 24 * 60 * 60 * 1000;

fn entropy(seed: &str, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    let mut block = Sha256::digest(seed.as_bytes()).to_vec();
    while out.len() < len {
        out.extend_from_slice(&block);
        block = Sha256::digest(&block).to_vec();
    }
    out.truncate(len);
    out
}

fn protected(name: &str) -> ProtectedRef {
    ProtectedRef {
        ref_id: format!("protected:{name}"),
        revision: "r1".into(),
        owner_ref: "central".into(),
    }
}

fn receipt(name: &str) -> OracleEntropyReceipt {
    OracleEntropyReceipt {
        entropy_ref: protected(name),
        method_ref: "controlled-seed".into(),
        provider_ref: "provider:test-seed".into(),
        observed_at_unix_ms: T0,
        evidence_refs: vec![format!("evidence:{name}")],
    }
}

fn source(path: &str, revision: &str) -> SourceRevision {
    SourceRevision {
        source_ref: format!("central:source:controlled:{path}"),
        revision: revision.into(),
        standing_ref: "controlled-narrative".into(),
    }
}

fn thoth(extra: &[&str]) -> DeckDefinition {
    DeckDefinition {
        deck_ref: "deck:thoth-controlled".into(),
        system: OracleSystem::TarotThoth,
        register: TarotRegister::Thoth,
        extra_cards: extra.iter().map(|name| name.to_string()).collect(),
        source_refs: vec!["nara-personal:context/quaternal_tarot_protocol.md".into()],
    }
}

fn open(seed: &str) -> OracleJourney {
    OracleJourney::open(
        OpenJourney {
            journey_ref: "journey:controlled-move".into(),
            subject_id: "controlled:person-a".into(),
            concern: JourneyConcern {
                concern_ref: "concern:leaving-the-estate".into(),
                title: "Leaving the estate and what the move asks".into(),
                basis_sources: vec![source("day/2026-09-01/day.md", "rev-a")],
            },
            deck: thoth(&[]),
            day_ref: "central:day:controlled:2026-09-01".into(),
            actor_ref: "agent/nara".into(),
            entropy: receipt("shuffle"),
            consent: ConsentState::Granted,
            opened_at_unix_ms: T0,
        },
        &entropy(seed, 1024),
    )
    .unwrap()
}

fn act(id: &str, at: u64, request: JourneyRequest) -> JourneyAct {
    JourneyAct {
        request_id: id.into(),
        actor_ref: "agent/nara".into(),
        at_unix_ms: at,
        request,
    }
}

fn draw(
    journey: &mut OracleJourney,
    id: &str,
    spread: SpreadKind,
    day: &str,
    at: u64,
) -> JourneyEffect {
    journey
        .apply(
            act(
                id,
                at,
                JourneyRequest::Draw {
                    spread,
                    day_ref: day.into(),
                    basis_sources: vec![source("day/2026-09-01/day.md", "rev-a")],
                },
            ),
            &[],
        )
        .unwrap()
}

fn placement<'a>(journey: &'a OracleJourney, reference: &str) -> &'a Placement {
    journey
        .placements
        .iter()
        .find(|placement| placement.placement_ref == reference)
        .unwrap()
}

#[test]
fn one_shuffle_deals_in_sequence_with_orientation_fixed_at_the_shuffle() {
    let mut journey = open("seed-a");
    let order = journey.shuffle.order.clone();
    let (start, len) = (journey.shuffle.turned_start, journey.shuffle.turned_len);
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_eq!(
        sorted,
        (0..78).collect::<Vec<_>>(),
        "the shuffle is a permutation of 78"
    );

    let day = draw(
        &mut journey,
        "d1",
        SpreadKind::TorusDay,
        "central:day:controlled:2026-09-01",
        T0 + 1,
    );
    let night_spread = day.effect_refs[0].clone();
    draw(
        &mut journey,
        "d2",
        SpreadKind::KleinNight {
            day_spread_ref: night_spread,
        },
        "central:day:controlled:2026-09-01",
        T0 + 2,
    );
    assert_eq!(journey.dealt, 12);
    for (deck_position, placement) in journey.placements.iter().enumerate() {
        let position = deck_position as u16;
        assert_eq!(
            placement.card, order[deck_position],
            "cards come off the one shuffle in sequence"
        );
        assert_eq!(
            placement.reversed,
            position >= start && position < start + len,
            "reversal is the turned portion, not a fresh coin per draw"
        );
    }
    let labels: Vec<String> = journey
        .placements
        .iter()
        .map(|placement| placement.position.unwrap().label())
        .collect();
    assert_eq!(
        labels,
        [
            "P0", "P1", "P2", "P3", "P4", "P5", "P0'", "P1'", "P2'", "P3'", "P4'", "P5'"
        ]
    );
    assert_eq!(journey.placements[6].klein_face, KleinFace::Retrospective);
}

#[test]
fn replay_reconciles_and_a_new_body_continues_the_same_deck_across_a_day() {
    let mut journey = open("seed-b");
    let first = draw(
        &mut journey,
        "d1",
        SpreadKind::Sphere,
        "central:day:controlled:2026-09-01",
        T0 + 1,
    );
    let replay = draw(
        &mut journey,
        "d1",
        SpreadKind::Sphere,
        "central:day:controlled:2026-09-01",
        T0 + 1,
    );
    assert!(replay.replayed);
    assert_eq!(replay.effect_refs, first.effect_refs);
    assert_eq!(journey.dealt, 2, "a replayed request never deals twice");

    let conflict = journey.apply(
        act(
            "d1",
            T0 + 1,
            JourneyRequest::Draw {
                spread: SpreadKind::TorusDay,
                day_ref: "central:day:controlled:2026-09-01".into(),
                basis_sources: Vec::new(),
            },
        ),
        &[],
    );
    assert!(conflict.unwrap_err().contains("different request"));

    // Close the view; a fresh body reads the persisted state on the next Day.
    let persisted = serde_json::to_string(&journey).unwrap();
    let mut fresh: OracleJourney = serde_json::from_str(&persisted).unwrap();
    fresh.validate().unwrap();
    let before = fresh.shuffle.clone();
    draw(
        &mut fresh,
        "d2",
        SpreadKind::TorusDay,
        "central:day:controlled:2026-09-02",
        T0 + DAY + 5,
    );
    assert_eq!(fresh.shuffle, before, "no reshuffle on re-entry");
    assert_eq!(fresh.dealt, 8);
    assert_eq!(fresh.placements[2].card, before.order[2]);
    assert_eq!(fresh.placements[7].card, before.order[7]);
    assert_eq!(
        fresh.day_refs,
        [
            "central:day:controlled:2026-09-01",
            "central:day:controlled:2026-09-02"
        ]
    );
    assert_eq!(
        fresh.placements[0], journey.placements[0],
        "earlier placements are unchanged"
    );
}

#[test]
fn draw_and_symbolic_assignment_are_distinct_acts() {
    let mut journey = open("seed-c");
    draw(
        &mut journey,
        "d1",
        SpreadKind::Sphere,
        "central:day:controlled:2026-09-01",
        T0 + 1,
    );
    let assigned = journey
        .apply(
            act(
                "a1",
                T0 + 2,
                JourneyRequest::AssignSymbol {
                    card: 56 + 9,
                    reversed: false,
                    position: None,
                    reason: "The withdrawal described in the entry reads as the Hermit".into(),
                    day_ref: "central:day:controlled:2026-09-01".into(),
                },
            ),
            &[],
        )
        .unwrap();
    assert_eq!(journey.dealt, 2, "assignment does not touch the deck");
    let placement = placement(&journey, &assigned.effect_refs[0]);
    assert!(matches!(
        placement.basis,
        PlacementBasis::SymbolicAssignment { .. }
    ));
    let identity = card_identity(&journey.deck, placement.card).unwrap();
    assert_eq!(identity.name, "The Hermit");
    assert_eq!(
        identity.kernel_card_ref.as_deref(),
        Some("ql.pole.tarot-bridge/v1#major:9")
    );
    let view = journey.reading(None).unwrap();
    assert_eq!(view.symbolic_assignments.len(), 1);
    assert_eq!(view.spreads[0].placements.len(), 2);
    assert!(
        !serde_json::to_string(&view).unwrap().contains("\"order\""),
        "the undealt order is never disclosed in the reading"
    );
}

#[test]
fn card_identity_follows_the_kernel_layout_and_the_deck_register() {
    let deck = thoth(&["Joker"]);
    assert_eq!(deck.card_count(), 79);
    assert_eq!(card_identity(&deck, 56).unwrap().name, "The Fool");
    assert_eq!(card_identity(&deck, 77).unwrap().name, "The Universe");
    assert_eq!(card_identity(&deck, 78).unwrap().arcana, Arcana::Extra);
    assert!(card_identity(&deck, 79).is_err());
    let names: Vec<String> = (0..56)
        .map(|index| card_identity(&deck, index).unwrap().name)
        .collect();
    assert!(names.iter().any(|name| name.ends_with("of Disks")));
    assert!(!names.iter().any(|name| name.ends_with("of Pentacles")));
    // The printed Thoth courts: Knight, Queen, Prince, Princess; never King.
    assert!(names.iter().any(|name| name == "Knight of Cups"));
    assert!(!names.iter().any(|name| name.starts_with("King ")));
    assert!(names.iter().any(|name| name == "Princess of Disks"));
    let rws = DeckDefinition {
        register: TarotRegister::Rws,
        ..thoth(&[])
    };
    assert!((0..56).any(|index| {
        card_identity(&rws, index)
            .unwrap()
            .name
            .ends_with("of Pentacles")
    }));
    for index in 0..56 {
        assert!(
            !card_identity(&deck, index).unwrap().codons.is_empty(),
            "every Minor carries its codon cover"
        );
    }
}

#[test]
fn a_correction_supersedes_without_erasing_the_original_reading() {
    let mut journey = open("seed-d");
    let effect = draw(
        &mut journey,
        "d1",
        SpreadKind::Sphere,
        "central:day:controlled:2026-09-01",
        T0 + 1,
    );
    let target = effect.effect_refs[1].clone();
    let reading = |reading_ref: &str, kind, actor_kind, supersedes: Option<&str>, text: &str| {
        JourneyRequest::Read {
            placement_ref: target.clone(),
            reading: NewReading {
                reading_ref: reading_ref.into(),
                kind,
                actor_kind,
                text: text.into(),
                supersedes: supersedes.map(str::to_string),
                source_refs: vec!["central:source:controlled:day/2026-09-01/day.md".into()],
                occurred_at_unix_ms: T0 + 3,
            },
        }
    };
    journey
        .apply(
            act(
                "r1",
                T0 + 3,
                reading(
                    "reading:1",
                    ReadingKind::Original,
                    ActorKind::Agent,
                    None,
                    "Ground as grief for the house.",
                ),
            ),
            &[],
        )
        .unwrap();
    let bad = journey.apply(
        act(
            "r2",
            T0 + 4,
            reading(
                "reading:2",
                ReadingKind::Correction,
                ActorKind::Human,
                Some("reading:missing"),
                "No.",
            ),
        ),
        &[],
    );
    assert!(bad.is_err());
    journey
        .apply(
            act(
                "r3",
                T0 + DAY,
                reading(
                    "reading:3",
                    ReadingKind::Correction,
                    ActorKind::Human,
                    Some("reading:1"),
                    "It is not grief. The ground is relief that the argument is over.",
                ),
            ),
            &[],
        )
        .unwrap();
    let view = journey.reading(None).unwrap();
    let placement = &view.spreads[0].placements[0];
    assert_eq!(
        placement.reading_history.len(),
        2,
        "the original stays in history"
    );
    assert_eq!(placement.current_readings.len(), 1);
    assert_eq!(placement.current_readings[0].reading_ref, "reading:3");
    assert_eq!(placement.current_readings[0].actor_kind, ActorKind::Human);
    assert_eq!(placement.reading_history[0].actor_kind, ActorKind::Agent);
}

/// Three-coin line values from the entropy stream, derived independently:
/// each coin consumes one byte, heads (odd byte) adds one to the base of six.
fn expected_coin_lines(bytes: &[u8]) -> [u8; 6] {
    let mut lines = [0; 6];
    for (line, chunk) in lines.iter_mut().zip(bytes.chunks(3)) {
        *line = 6 + chunk.iter().filter(|byte| *byte & 1 == 1).count() as u8;
    }
    lines
}

fn cast_request(day: &str) -> JourneyRequest {
    JourneyRequest::CastIChing {
        cast: Box::new(IChingCastRequest {
            coins: true,
            query_ref: protected("query"),
            entropy: receipt("iching"),
            original_payload_ref: protected("packet"),
            cast_degree: Some(210),
            vak_ref: None,
            consent: ConsentState::Granted,
            hygiene: OracleHygiene::Clear,
        }),
        day_ref: day.into(),
        basis_sources: vec![source("day/2026-09-01/day.md", "rev-a")],
        related_placement_refs: Vec::new(),
    }
}

#[test]
fn iching_cast_follows_the_c_line_law_and_keeps_the_original_beside_later_readings() {
    let mut journey = open("seed-e");
    let bytes = entropy("iching-cast", 18);
    let lines = expected_coin_lines(&bytes);
    let effect = journey
        .apply(
            act(
                "c1",
                T0 + 10,
                cast_request("central:day:controlled:2026-09-01"),
            ),
            &bytes,
        )
        .unwrap();
    let reading = journey
        .iching
        .iter()
        .find(|r| r.reading_ref == effect.effect_refs[0])
        .unwrap()
        .clone();
    assert_eq!(reading.lines, lines);

    // m4_cast_iching: yang (odd) sets bit i, 6/9 change, result = bits ^ mask.
    let mut bits = 0u8;
    let mut mask = 0u8;
    for (index, value) in lines.iter().enumerate() {
        if value % 2 == 1 {
            bits |= 1 << index;
        }
        if *value == 6 || *value == 9 {
            mask |= 1 << index;
        }
    }
    assert_eq!(reading.primary, bits);
    assert_eq!(reading.changing_mask, mask);
    assert_eq!(reading.resulting, (mask != 0).then_some(bits ^ mask));
    let nuclear = (((bits >> 2) & 7) << 3) | ((bits >> 1) & 7);
    assert_eq!(reading.nuclear, nuclear);
    assert!(matches!(reading.basis, IChingBasis::Cast { .. }));

    // A replay arriving with fresh entropy reconciles; it never casts again.
    let other = entropy("different", 18);
    let replay = journey
        .apply(
            act(
                "c1",
                T0 + 10,
                cast_request("central:day:controlled:2026-09-01"),
            ),
            &other,
        )
        .unwrap();
    assert!(replay.replayed);
    assert_eq!(journey.iching.len(), 1);
    assert_eq!(journey.iching[0].lines, lines);

    // Later material develops the reading retrospectively; the cast is untouched.
    let original = reading;
    journey
        .apply(
            act(
                "c2",
                T0 + 2 * DAY,
                JourneyRequest::ReadIChing {
                    reading_ref: original.reading_ref.clone(),
                    reading: NewReading {
                        reading_ref: "iching-reading:retro".into(),
                        kind: ReadingKind::Retrospective,
                        actor_kind: ActorKind::Agent,
                        text: "Two days on, the changing line reads as the move itself.".into(),
                        supersedes: None,
                        source_refs: vec!["central:source:controlled:day/2026-09-03/day.md".into()],
                        occurred_at_unix_ms: T0 + 2 * DAY,
                    },
                },
            ),
            &[],
        )
        .unwrap();
    let after = &journey.iching[0];
    assert_eq!(after.basis, original.basis);
    assert_eq!(after.lines, original.lines);
    assert_eq!(after.readings.len(), 1);
}

#[test]
fn computed_form_is_a_distinct_iching_basis_without_chance() {
    let mut journey = open("seed-f");
    journey
        .apply(
            act(
                "k1",
                T0 + 5,
                JourneyRequest::RecordComputedIChing {
                    computation_ref: "ql:m3-form-reading#codon-hexagram".into(),
                    input: source("ql/m3/form-state", "form-rev-1"),
                    hexagram: 0b101_001,
                    changing_mask: 0,
                    day_ref: "central:day:controlled:2026-09-01".into(),
                    related_placement_refs: Vec::new(),
                },
            ),
            &[],
        )
        .unwrap();
    let reading = &journey.iching[0];
    assert_eq!(reading.primary, 0b101_001);
    assert_eq!(reading.lines, [7, 8, 8, 7, 8, 7]);
    assert_eq!(reading.resulting, None);
    assert!(matches!(reading.basis, IChingBasis::Computed { .. }));
}

#[test]
fn janus_aliveness_tracks_day_flow_recognition_and_resolves_spreads() {
    let mut journey = open("seed-g");
    let effect = draw(
        &mut journey,
        "d1",
        SpreadKind::Sphere,
        "central:day:controlled:2026-09-01",
        T0,
    );
    let (spread, first, second) = (
        effect.effect_refs[0].clone(),
        effect.effect_refs[1].clone(),
        effect.effect_refs[2].clone(),
    );
    let first_name = card_identity(&journey.deck, placement(&journey, &first).card)
        .unwrap()
        .name;
    let note = |path: &str, at: u64, body: &str| DayFlowNote {
        source: source(path, &format!("rev-{at}")),
        entry_selector: Some("entry:1".into()),
        noted_at_unix_ms: at,
        body: body.into(),
    };
    let notes = vec![
        note(
            "day/2026-08-31/day.md",
            T0 - DAY,
            &format!("Before the draw: {first_name}."),
        ),
        note(
            "day/2026-09-02/day.md",
            T0 + DAY,
            &format!("Today the {first_name} kept coming back."),
        ),
    ];
    journey
        .apply(
            act(
                "t1",
                T0 + DAY,
                JourneyRequest::TrackRecognitions {
                    notes: notes.clone(),
                },
            ),
            &[],
        )
        .unwrap();
    journey
        .apply(
            act("t2", T0 + DAY, JourneyRequest::TrackRecognitions { notes }),
            &[],
        )
        .unwrap();
    assert_eq!(
        placement(&journey, &first).recognitions.len(),
        1,
        "a note before the draw is ignored and the same passage never counts twice"
    );
    assert_eq!(placement(&journey, &second).recognitions.len(), 0);

    journey
        .apply(
            act("e1", T0 + 15 * DAY, JourneyRequest::EvaluateAliveness),
            &[],
        )
        .unwrap();
    assert_eq!(
        placement(&journey, &first).live_state,
        LiveState::Generating
    );
    assert_eq!(placement(&journey, &second).live_state, LiveState::Muting);
    journey
        .apply(
            act("e2", T0 + 22 * DAY, JourneyRequest::EvaluateAliveness),
            &[],
        )
        .unwrap();
    assert_eq!(placement(&journey, &second).live_state, LiveState::Mute);
    assert!(
        !journey.spread_resolved(&spread),
        "one position is still generating"
    );

    // A target aspect within a day of exactness reopens a mute position.
    journey
        .apply(
            act(
                "a1",
                T0 + 23 * DAY,
                JourneyRequest::SetTargetAspect {
                    placement_ref: second.clone(),
                    aspect: TargetAspect {
                        planet_a: 6,
                        aspect_kind: AspectKind::Square,
                        planet_b_or_natal: 1,
                        exact_at_unix_ms: Some(T0 + 24 * DAY),
                        source: source("sky/transit", "sky-rev"),
                    },
                },
            ),
            &[],
        )
        .unwrap();
    journey
        .apply(
            act(
                "e3",
                T0 + 24 * DAY - 3_600_000,
                JourneyRequest::EvaluateAliveness,
            ),
            &[],
        )
        .unwrap();
    assert_eq!(
        placement(&journey, &second).live_state,
        LiveState::Generating
    );
}

#[test]
fn word_boundaries_keep_art_out_of_part() {
    let mut journey = open("seed-h");
    journey
        .apply(
            act(
                "a1",
                T0,
                JourneyRequest::AssignSymbol {
                    card: 56 + 14,
                    reversed: false,
                    position: None,
                    reason: "Integration of opposites in the entry".into(),
                    day_ref: "central:day:controlled:2026-09-01".into(),
                },
            ),
            &[],
        )
        .unwrap();
    assert_eq!(card_identity(&journey.deck, 70).unwrap().name, "Art");
    let reference = journey.placements[0].placement_ref.clone();
    let tracked = journey
        .apply(
            act(
                "t1",
                T0 + 1,
                JourneyRequest::TrackRecognitions {
                    notes: vec![DayFlowNote {
                        source: source("day/2026-09-01/day.md", "rev-b"),
                        entry_selector: None,
                        noted_at_unix_ms: T0 + 1,
                        body: "I did my part of the packing.".into(),
                    }],
                },
            ),
            &[],
        )
        .unwrap();
    assert!(tracked.effect_refs.is_empty());
    assert!(placement(&journey, &reference).recognitions.is_empty());
}

#[test]
fn protocol_boundaries_are_enforced() {
    let mut journey = open("seed-i");
    let sphere = draw(
        &mut journey,
        "d1",
        SpreadKind::Sphere,
        "central:day:controlled:2026-09-01",
        T0,
    );
    let night = journey.apply(
        act(
            "d2",
            T0,
            JourneyRequest::Draw {
                spread: SpreadKind::KleinNight {
                    day_spread_ref: sphere.effect_refs[0].clone(),
                },
                day_ref: "central:day:controlled:2026-09-01".into(),
                basis_sources: Vec::new(),
            },
        ),
        &[],
    );
    assert!(night.is_err(), "a Night arc completes a Torus Day spread");
    let lemniscate = journey.apply(
        act(
            "d3",
            T0,
            JourneyRequest::Draw {
                spread: SpreadKind::Lemniscate {
                    within_placement_ref: sphere.effect_refs[1].clone(),
                },
                day_ref: "central:day:controlled:2026-09-01".into(),
                basis_sources: Vec::new(),
            },
        ),
        &[],
    );
    assert!(lemniscate.is_err(), "the lemniscate unpacks P4 only");

    let day = draw(
        &mut journey,
        "d4",
        SpreadKind::TorusDay,
        "central:day:controlled:2026-09-01",
        T0 + 1,
    );
    let p4 = day.effect_refs[5].clone();
    draw(
        &mut journey,
        "d5",
        SpreadKind::Lemniscate {
            within_placement_ref: p4,
        },
        "central:day:controlled:2026-09-01",
        T0 + 2,
    );
    assert_eq!(journey.dealt, 14);

    let closing = draw(
        &mut journey,
        "d6",
        SpreadKind::Closing,
        "central:day:controlled:2026-09-09",
        T0 + 3,
    );
    assert_eq!(closing.effect_refs.len(), 1 + 64);
    assert_eq!(journey.status, JourneyStatus::DeckComplete);
    let after = journey.apply(
        act(
            "d7",
            T0 + 4,
            JourneyRequest::Draw {
                spread: SpreadKind::Single { count: 1 },
                day_ref: "central:day:controlled:2026-09-09".into(),
                basis_sources: Vec::new(),
            },
        ),
        &[],
    );
    assert!(
        after.unwrap_err().contains("0 undealt"),
        "an exhausted deck deals nothing more"
    );
}

#[test]
fn a_tampered_persisted_journey_is_refused() {
    let mut journey = open("seed-j");
    draw(
        &mut journey,
        "d1",
        SpreadKind::Sphere,
        "central:day:controlled:2026-09-01",
        T0,
    );
    let mut value = serde_json::to_value(&journey).unwrap();
    let card = value["placements"][0]["card"].as_u64().unwrap();
    value["placements"][0]["card"] = serde_json::json!((card + 1) % 78);
    let tampered: OracleJourney = serde_json::from_value(value).unwrap();
    assert!(tampered.validate().unwrap_err().contains("shuffle"));

    let mut value = serde_json::to_value(&journey).unwrap();
    value["dealt"] = serde_json::json!(5);
    let tampered: OracleJourney = serde_json::from_value(value).unwrap();
    assert!(tampered.validate().is_err());
}
