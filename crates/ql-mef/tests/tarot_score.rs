//! The deterministic entity/event → Tarot score (`ql.tarot-score/v1`):
//! anchor resolution through the qualified sky admissions, inscription
//! through the map chain, cards through the kernel exact cover, the
//! four-charge and pose laws, and the drawn-token boundary against the
//! journey's deck. Sources are the real registry, the real M3 map and the
//! real kernel bridge — no fixture stand-ins for the derivation itself.

use ql_core::{Codon64, MAJOR_ARCANA_COUNT, MinorArcanaCard, TarotBridge, TarotSuit};
use ql_mef::MFace;
use ql_mef::coordinate_expression::{
    ExpressiveRole, SubjectKind, resolve_subject_manifestation, validate_subject_ref,
};
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::nara::domain::*;
use ql_mef::nara::{ConsentState, SourceRevision};
use ql_mef::tarot_score::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SUBJECT: &str = "controlled:tarot-score-subject";
const LOCUS: &str = "#2-5-4";
const T0: u64 = 1_790_500_000_000;
const DAY: u64 = 24 * 60 * 60 * 1000;

/// The shared dated-sky fixture (28 Sep 2026 12:00 UTC). Its admission
/// binding predates the current native registry revision, so the test
/// refreshes exactly that binding to the current registry — the qualified
/// `nara::current::transit` admission itself is the source path under test.
fn sky() -> Value {
    let mut sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
    ))
    .unwrap();
    sky["source_binding"]["registry_revision"] = json!(ql_mef::m2::catalogue().registry_revision());
    sky
}

/// A minimal valid identity reading: the score admits its schema, its
/// input revision and its natal sky (the placements carrier), reading them
/// exactly where `nara::current` and `nara::intake_composition` read them.
fn identity_with(sun_longitude: f64) -> Value {
    let mut natal = sky();
    natal["bodies"][0]["longitude_degrees"] = json!(sun_longitude);
    json!({
        "schema": "ql.nara-identity-reading/v1",
        "person_ref": SUBJECT,
        "nara_ref": "controlled:nara:tarot-score",
        "input_revision": "controlled-input-r1",
        "natal": {"schema": "ql.nara-natal/v1", "sky": natal},
    })
}

/// The four aces of the exact cover, as already-drawn journey placements:
/// deck ids 0/14/28/42, codons AAA/TTT/CCC/GGG.
fn drawn_aces() -> Vec<DrawnPlacement> {
    [0u16, 14, 28, 42]
        .into_iter()
        .map(|card| DrawnPlacement {
            journey_ref: "journey:controlled-score".into(),
            placement_ref: format!("journey:controlled-score/single/{card}"),
            day_ref: "central:day:controlled:2026-09-28".into(),
            card,
        })
        .collect()
}

fn basis<'a>(
    identity: Option<&'a Value>,
    sky: Option<&'a Value>,
    clock_steps: u64,
    primary: PrimaryAnchor,
    drawn: &'a [DrawnPlacement],
) -> ScoreBasis<'a> {
    ScoreBasis {
        subject_ref: SUBJECT,
        locus_ref: LOCUS,
        identity,
        occasion_sky: sky,
        clock_steps,
        primary_anchor: primary,
        drawn,
        boundary_passage: None,
        manifestation: None,
    }
}

fn full_basis() -> (Value, Value, Vec<DrawnPlacement>) {
    (identity_with(5.0), sky(), drawn_aces())
}

// ---------------------------------------------------------------------------
// Journey fixtures — a real open journey with one dealt card
// ---------------------------------------------------------------------------

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

fn open_journey(seed: &str) -> OracleJourney {
    OracleJourney::open(
        OpenJourney {
            journey_ref: "journey:controlled-score".into(),
            subject_id: SUBJECT.into(),
            concern: JourneyConcern {
                concern_ref: "concern:score-derivation".into(),
                title: "The score derivation concern".into(),
                basis_sources: vec![source("day/2026-09-28/day.md", "rev-a")],
            },
            deck: DeckDefinition {
                deck_ref: "deck:thoth-controlled".into(),
                system: OracleSystem::TarotThoth,
                register: TarotRegister::Thoth,
                extra_cards: vec![],
                source_refs: vec!["nara-personal:context/quaternal_tarot_protocol.md".into()],
            },
            day_ref: "central:day:controlled:2026-09-28".into(),
            actor_ref: "agent/nara".into(),
            entropy: receipt("shuffle"),
            consent: ConsentState::Granted,
            opened_at_unix_ms: T0,
        },
        &entropy(seed, 1024),
    )
    .unwrap()
}

/// A real open journey whose first dealt card is a Minor Arcana card, so the
/// placement can be read as a score token. The deck is shuffled and dealt
/// here exactly once; the returned snapshot is the journey as the draw left
/// it, proving later that the score consumed nothing.
fn journey_with_minor_single_draw() -> (OracleJourney, DrawnPlacement, OracleJourney) {
    for index in 0..64u32 {
        let mut journey = open_journey(&format!("score-seed-{index}"));
        journey
            .apply(
                JourneyAct {
                    request_id: "draw-1".into(),
                    actor_ref: "agent/nara".into(),
                    at_unix_ms: T0 + DAY,
                    day_ref: Some("central:day:controlled:2026-09-28".into()),
                    request: JourneyRequest::Draw {
                        spread: SpreadKind::Single { count: 1 },
                        day_ref: "central:day:controlled:2026-09-28".into(),
                        basis_sources: vec![source("day/2026-09-28/day.md", "rev-a")],
                    },
                },
                &[],
            )
            .unwrap();
        let placement = &journey.placements[0];
        if usize::from(placement.card) < ql_core::MINOR_ARCANA_COUNT {
            let declared = DrawnPlacement {
                journey_ref: journey.journey_ref.clone(),
                placement_ref: placement.placement_ref.clone(),
                day_ref: placement.day_ref.clone(),
                card: placement.card,
            };
            let after_draw = journey.clone();
            return (journey, declared, after_draw);
        }
    }
    panic!("no controlled shuffle dealt a minor first");
}

// ---------------------------------------------------------------------------
// 1. Determinism
// ---------------------------------------------------------------------------

#[test]
fn score_is_deterministic_across_repeated_resolution() {
    let (identity_a, sky_a, drawn_a) = full_basis();
    let a = resolve_tarot_score(&basis(
        Some(&identity_a),
        Some(&sky_a),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn_a,
    ))
    .unwrap();
    // A second, independently constructed basis over the same values.
    let (identity_b, sky_b, drawn_b) = full_basis();
    let b = resolve_tarot_score(&basis(
        Some(&identity_b),
        Some(&sky_b),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn_b,
    ))
    .unwrap();

    assert_eq!(a.schema, TAROT_SCORE_CONTRACT);
    assert_eq!(a.canonical_json().unwrap(), b.canonical_json().unwrap());
    assert_eq!(a.basis_revision, b.basis_revision);
    assert_eq!(a.score_revision, b.score_revision);
    fn refs(s: &TarotScore) -> Vec<&String> {
        s.tokens.iter().map(|t| &t.token_ref).collect()
    }
    assert_eq!(refs(&a), refs(&b));

    // Ten natal, ten kairos and four drawn tokens, in canonical order.
    assert_eq!(a.tokens.len(), 24);
    let roles = a.tokens.iter().map(|t| t.role.as_str()).collect::<Vec<_>>();
    let mut sorted = roles.clone();
    sorted.sort_unstable();
    assert_eq!(roles, sorted, "tokens are in canonical role order");
    assert_eq!(
        a.tokens
            .iter()
            .filter(|t| t.origin == ScoreOrigin::Computed)
            .count(),
        20
    );
    assert_eq!(
        a.tokens
            .iter()
            .filter(|t| t.origin == ScoreOrigin::Drawn)
            .count(),
        4
    );
    // The declared primary anchor's token is present and computed.
    assert!(
        a.tokens
            .iter()
            .any(|t| t.role == "natal/Sun" && t.origin == ScoreOrigin::Computed)
    );
}

// ---------------------------------------------------------------------------
// 2. The four aces under one direction
// ---------------------------------------------------------------------------

#[test]
fn four_aces_keep_four_identities_under_equal_direction() {
    let bridge = TarotBridge::kernel();
    let suits = [
        TarotSuit::Cups,
        TarotSuit::Wands,
        TarotSuit::Pentacles,
        TarotSuit::Swords,
    ];

    // The map's 36 decan pip codons carry no perfect palindrome, so the four
    // charge identities are read from the four-charge law directly and enter
    // the score through the exact cover's aces.
    for degree in 0..360u16 {
        let seed = ql_mef::m3_inscription::seed_at(f64::from(degree) + 0.5).unwrap();
        let class = Codon64::new(seed.pip_codon).classify();
        assert!(
            !matches!(class, ql_core::CodonClass::PerfectPalindromic),
            "decan pip codon unexpectedly a perfect palindrome"
        );
    }

    let mut raws = Vec::new();
    let mut directions = Vec::new();
    let mut codons = Vec::new();
    for suit in suits {
        let card = &bridge.suit_cards(suit)[0];
        assert_eq!(card.pip().value(), 0, "ace of {}", suit.name());
        let codon = card.codon_a();
        assert!(!card.is_dual_court());
        codons.push(codon);
        let charge = codon.four_charge();
        assert!(charge.invariant_holds(), "4X invariant at {codon}");
        // The raw vector is k·(3,−1,1,1) with k the suit nucleotide's value.
        let k = i32::from(codon.outer().coin_value().value());
        let raw = [
            i32::from(charge.pp),
            i32::from(charge.mm),
            i32::from(charge.mp),
            i32::from(charge.pm),
        ];
        assert_eq!(raw, [3 * k, -k, k, k], "{codon} raw charge");
        raws.push(raw);
        let norm = (raw.iter().map(|v| v * v).sum::<i32>() as f64).sqrt();
        directions.push(raw.map(|v| f64::from(v) / norm));
    }
    // Four distinct raw identities.
    assert_eq!(raws.len(), 4);
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(raws[i], raws[j], "raw charges {i}/{j} distinct");
        }
    }
    // One direction: all four normalised vectors agree within 1e-9.
    for (i, a) in directions.iter().enumerate() {
        for b in &directions {
            for (slot, (x, y)) in a.iter().zip(b.iter()).enumerate() {
                assert!((x - y).abs() < 1e-9, "directions {i} agree at slot {slot}");
            }
        }
    }
    // Four different suit cards: A/T/C/G → Cups/Wands/Pentacles/Swords.
    for (codon, suit) in codons.iter().zip(suits) {
        let card = bridge.card_of_codon(*codon).unwrap();
        assert_eq!(card.suit(), suit);
        assert_eq!(card.pip().value(), 0);
    }

    // The score never merges them: four drawn tokens, four cards, one
    // direction each.
    let (identity, sky, drawn) = full_basis();
    let score = resolve_tarot_score(&basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn,
    ))
    .unwrap();
    let drawn_tokens: Vec<_> = score
        .tokens
        .iter()
        .filter(|t| t.origin == ScoreOrigin::Drawn)
        .collect();
    assert_eq!(drawn_tokens.len(), 4);
    let token_codons: Vec<u8> = drawn_tokens.iter().map(|t| t.hexagram_address).collect();
    assert_eq!(token_codons, vec![0, 21, 42, 63]);
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(drawn_tokens[i].token_ref, drawn_tokens[j].token_ref);
            assert_ne!(
                drawn_tokens[i].card_kernel_ref,
                drawn_tokens[j].card_kernel_ref
            );
        }
    }
    for token in &drawn_tokens {
        let norm = (token.four_charge_raw.iter().map(|v| v * v).sum::<i32>() as f64).sqrt();
        let expected = token.four_charge_raw.map(|v| f64::from(v) / norm);
        for slot in 0..4 {
            assert!((token.four_charge_normalised[slot] - expected[slot]).abs() < 1e-12);
            assert!((token.four_charge_normalised[slot] - directions[0][slot]).abs() < 1e-9);
        }
    }
}

// ---------------------------------------------------------------------------
// 3. Dual courts carry both codons
// ---------------------------------------------------------------------------

#[test]
fn dual_court_token_carries_both_codons() {
    let bridge = TarotBridge::kernel();
    // Every suit's dual court: yin suits dual at Prince+King, yang at
    // Princess+Queen; both codons resolve to the same card identity.
    for suit in TarotSuit::ALL {
        for pip in suit.dual_court_pips() {
            let card = &bridge.suit_cards(suit)[pip.value() as usize];
            assert!(
                card.is_dual_court(),
                "{} of {}",
                pip.rws_name(),
                suit.name()
            );
            let codons: Vec<u8> = card.codons().map(|c| c.address()).collect();
            assert_eq!(codons.len(), 2);
            for codon in &codons {
                let found = bridge.card_of_codon(Codon64::new(*codon)).unwrap();
                assert_eq!(found.card_id(), card.card_id());
            }
            assert_ne!(codons[0], codons[1]);
        }
    }

    // The Prince of Cups (deck id 11) as a drawn token carries both codons.
    let prince_of_cups =
        &bridge.suit_cards(TarotSuit::Cups)[TarotSuit::Cups.dual_court_pips()[0].value() as usize];
    assert_eq!(prince_of_cups.card_id(), 11);
    let drawn = vec![DrawnPlacement {
        journey_ref: "journey:controlled-score".into(),
        placement_ref: "journey:controlled-score/single/prince".into(),
        day_ref: "central:day:controlled:2026-09-28".into(),
        card: prince_of_cups.card_id() as u16,
    }];
    let (identity, sky, _aces) = full_basis();
    let score = resolve_tarot_score(&basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn,
    ))
    .unwrap();
    let token = score
        .tokens
        .iter()
        .find(|t| t.origin == ScoreOrigin::Drawn)
        .unwrap();
    let expected: Vec<u8> = prince_of_cups.codons().map(|c| c.address()).collect();
    assert_eq!(token.codons, expected);
    assert_eq!(token.codons.len(), 2);
    assert!(token.codons.contains(&token.hexagram_address));
    // The card identity is identical read from either codon.
    for codon in &token.codons {
        let found = bridge.card_of_codon(Codon64::new(*codon)).unwrap();
        assert_eq!(
            token.card_kernel_ref,
            format!("ql.pole.tarot-bridge/v1#minor:{}", found.card_id())
        );
    }
}

// ---------------------------------------------------------------------------
// 4. Computed scores never draw
// ---------------------------------------------------------------------------

#[test]
fn computed_score_never_draws_and_retry_creates_no_occurrence() {
    let (journey, declared, after_draw) = journey_with_minor_single_draw();
    let dealt_before = journey.dealt;
    let placements_before = journey.placements.len();
    assert_eq!(journey.remaining(), 77, "one card dealt from 78");

    let (identity, sky, _aces) = full_basis();
    let first = resolve_tarot_score(&basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        std::slice::from_ref(&declared),
    ))
    .unwrap();
    let second = resolve_tarot_score(&basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        std::slice::from_ref(&declared),
    ))
    .unwrap();

    // The drawn token set is unchanged and the act reference round-trips.
    let drawn: Vec<_> = first
        .tokens
        .iter()
        .filter(|t| t.origin == ScoreOrigin::Drawn)
        .collect();
    assert_eq!(drawn.len(), 1);
    assert_eq!(
        drawn[0].origin_ref.as_deref(),
        Some(declared.placement_ref.as_str())
    );
    assert_eq!(drawn[0].role, format!("oracle/{}", declared.placement_ref));
    let drawn_second: Vec<_> = second
        .tokens
        .iter()
        .filter(|t| t.origin == ScoreOrigin::Drawn)
        .collect();
    assert_eq!(drawn[0].token_ref, drawn_second[0].token_ref);
    assert_eq!(drawn[0].card_kernel_ref, drawn_second[0].card_kernel_ref);

    // No deck position advanced: the journey is exactly as the draw left it
    // after both resolutions, and the recompute produced no new token.
    assert_eq!(journey.dealt, dealt_before);
    assert_eq!(journey.placements.len(), placements_before);
    assert_eq!(journey, after_draw);
    assert_eq!(first.tokens.len(), second.tokens.len());
    assert_eq!(
        first.canonical_json().unwrap(),
        second.canonical_json().unwrap()
    );

    // A drawn Major carries no codon and is refused by name.
    let major = DrawnPlacement {
        journey_ref: "journey:controlled-score".into(),
        placement_ref: "journey:controlled-score/single/major".into(),
        day_ref: "central:day:controlled:2026-09-28".into(),
        card: ql_core::MINOR_ARCANA_COUNT as u16,
    };
    let error = resolve_tarot_score(&basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        std::slice::from_ref(&major),
    ))
    .unwrap_err();
    assert!(error.contains("Minor"), "named refusal: {error}");
}

// ---------------------------------------------------------------------------
// 5. Basis change vs source correction
// ---------------------------------------------------------------------------

#[test]
fn basis_change_changes_revision_but_source_correction_keeps_token_identity() {
    // Same subject, same roles, one longitude value corrected (same planet,
    // same decan: 5.0° and 5.5° both stand in Aries Decan 1 on TTA).
    let identity_a = identity_with(5.0);
    let identity_b = identity_with(5.5);
    let sky_a = sky();
    let sky_b = sky();
    let no_drawn: Vec<DrawnPlacement> = Vec::new();
    let a = resolve_tarot_score(&basis(
        Some(&identity_a),
        Some(&sky_a),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &no_drawn,
    ))
    .unwrap();
    let b = resolve_tarot_score(&basis(
        Some(&identity_b),
        Some(&sky_b),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &no_drawn,
    ))
    .unwrap();

    assert_ne!(
        a.basis_revision, b.basis_revision,
        "longitude bits move the basis"
    );
    assert_ne!(
        a.score_revision, b.score_revision,
        "the derivation output changed"
    );
    fn sun(s: &TarotScore) -> &ScoreToken {
        s.tokens.iter().find(|t| t.role == "natal/Sun").unwrap()
    }
    assert_eq!(
        sun(&a).token_ref,
        sun(&b).token_ref,
        "token identity survives the correction"
    );
    assert_eq!(sun(&a).hexagram_address, sun(&b).hexagram_address);
    assert_eq!(sun(&a).card_kernel_ref, sun(&b).card_kernel_ref);
    let sun_longitude = |anchor: &ScoreAnchor| match anchor {
        ScoreAnchor::Natal {
            longitude_degrees, ..
        }
        | ScoreAnchor::Kairos {
            longitude_degrees, ..
        } => *longitude_degrees,
        ScoreAnchor::Oracle { .. } => f64::NAN,
    };
    assert_ne!(
        sun_longitude(&sun(&a).anchor),
        sun_longitude(&sun(&b).anchor)
    );

    // A changed clock basis moves the clock projection and the environment
    // pose, admitted per the profile's state count.
    let c = resolve_tarot_score(&basis(
        Some(&identity_a),
        Some(&sky_a),
        500,
        PrimaryAnchor::Natal { planet_id: 0 },
        &no_drawn,
    ))
    .unwrap();
    assert_ne!(a.clock, c.clock);
    assert_ne!(a.basis_revision, c.basis_revision);
    let sun_c = sun(&c);
    assert_ne!(sun(&a).pose.active_state, sun_c.pose.active_state);
    for score in [&a, &c] {
        let token = sun(score);
        let lawful = token.pose.active_state < token.pose.state_count;
        assert_eq!(token.pose.lawfully_admitted, lawful);
        assert_eq!(token.pose.candidate_slot, token.pose.active_state);
        assert_eq!(
            token.pose.candidate_rotation_degrees,
            u16::from(token.pose.active_state) * 45
        );
    }
    // Token identity is clock-independent: the basis moved, the anchor did not.
    assert_eq!(sun(&a).token_ref, sun_c.token_ref);
}

// ---------------------------------------------------------------------------
// 6. Named refusals
// ---------------------------------------------------------------------------

#[test]
fn refuses_without_anchor_basis_and_refuses_missing_declared_primary() {
    let empty: Vec<DrawnPlacement> = Vec::new();
    let error = resolve_tarot_score(&basis(
        None,
        None,
        0,
        PrimaryAnchor::Natal { planet_id: 0 },
        &empty,
    ))
    .unwrap_err();
    assert!(error.contains("no anchor basis"), "named refusal: {error}");

    // Declared primary natal anchor absent: only a kairos sky is admitted.
    let sky_only = sky();
    let error = resolve_tarot_score(&basis(
        None,
        Some(&sky_only),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &empty,
    ))
    .unwrap_err();
    assert!(
        error.contains("declared primary natal anchor Sun is absent"),
        "named refusal: {error}"
    );

    // Declared primary kairos anchor absent: only natal placements admitted.
    let identity_only = identity_with(5.0);
    let error = resolve_tarot_score(&basis(
        Some(&identity_only),
        None,
        359,
        PrimaryAnchor::Kairos { body_index: 0 },
        &empty,
    ))
    .unwrap_err();
    assert!(
        error.contains("declared primary kairos anchor Sun is absent"),
        "named refusal: {error}"
    );

    // Out-of-range primary anchors refuse by name too.
    let (identity, sky, drawn) = full_basis();
    let error = resolve_tarot_score(&basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 10 },
        &drawn,
    ))
    .unwrap_err();
    assert!(
        error.contains("outside the ten native bodies"),
        "named refusal: {error}"
    );

    // Unqualified subject and locus references refuse.
    assert!(validate_subject_ref("ql:k2/default-subject").is_ok());
    assert!(validate_subject_ref("person:someone").is_ok());
    assert!(validate_subject_ref("").is_err());
    assert!(validate_subject_ref("bad\0subject").is_err());
}

// ---------------------------------------------------------------------------
// 7. Majors stay outside the score
// ---------------------------------------------------------------------------

#[test]
fn majors_never_receive_codon_addresses() {
    // Compile-level statement of the minors-only invariant: the derivation's
    // only card lookup is the Minor exact cover, so a Major cannot enter a
    // token through the type at all.
    const MINOR_ONLY_CARD_LOOKUP: fn(&TarotBridge, Codon64) -> Option<&MinorArcanaCard> =
        TarotBridge::card_of_codon;
    let _ = MINOR_ONLY_CARD_LOOKUP;

    let bridge = TarotBridge::kernel();
    assert_eq!(bridge.major().len(), MAJOR_ARCANA_COUNT);
    // The majors are a separate register: chromosome pair and amino index,
    // no codons — while every token is a minor exact-cover card.
    for major in bridge.major() {
        let _ = (
            major.card_id(),
            major.name(),
            major.chromosome_pair(),
            major.amino_acid_index(),
        );
    }

    let (identity, sky, drawn) = full_basis();
    let score = resolve_tarot_score(&basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn,
    ))
    .unwrap();
    for token in &score.tokens {
        assert!(
            token
                .card_kernel_ref
                .starts_with("ql.pole.tarot-bridge/v1#minor:"),
            "every token is a minor: {}",
            token.card_kernel_ref
        );
    }
    // Without a declared passage the boundary functions are empty.
    assert_eq!(score.boundary_functions, json!({}));

    // Declared, they are functions of the existing boundary Major Arcana —
    // roles and references, never fabricated cards, never codon addresses.
    let mut declared_basis = basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn,
    );
    declared_basis.boundary_passage = Some((
        "central:day:controlled:2026-09-28#opening",
        "central:day:controlled:2026-09-28#completion",
    ));
    let declared = resolve_tarot_score(&declared_basis).unwrap();
    let boundary = &declared.boundary_functions;
    assert_eq!(boundary["schema"], "ql.tarot-boundary-functions/v1");
    assert_eq!(
        boundary["opening"]["major_card_ref"],
        "ql.pole.tarot-bridge/v1#major:0"
    );
    assert_eq!(
        boundary["completion"]["major_card_ref"],
        "ql.pole.tarot-bridge/v1#major:21"
    );
    assert_eq!(boundary["opening"]["boundary_value"], 0);
    assert_eq!(boundary["completion"]["boundary_value"], 1);
    assert_eq!(boundary["opening"]["role"], "start/opening");
    assert_eq!(boundary["completion"]["role"], "stop/completion");
    assert!(boundary["opening"].get("codons").is_none());
    assert!(boundary["completion"].get("codons").is_none());
}

// ---------------------------------------------------------------------------
// 8. Subject validation and the manifestation link
// ---------------------------------------------------------------------------

#[test]
fn subject_ref_validation_and_manifestation_link() {
    // Invalid subject references refuse before any derivation.
    for subject in ["", "bad\0subject"] {
        let identity = identity_with(5.0);
        let sky = sky();
        let empty: Vec<DrawnPlacement> = Vec::new();
        let mut score_basis = basis(
            Some(&identity),
            Some(&sky),
            359,
            PrimaryAnchor::Natal { planet_id: 0 },
            &empty,
        );
        score_basis.subject_ref = subject;
        let error = resolve_tarot_score(&score_basis).unwrap_err();
        assert!(
            error.contains("invalid native subject reference"),
            "named refusal: {error}"
        );
    }

    // A resolved manifestation of the same subject links into the score.
    let manifestation = resolve_subject_manifestation(
        native_current_m_registry(),
        SUBJECT,
        SubjectKind::Native,
        LOCUS,
        MFace::Bimba,
        &[
            ExpressiveRole::Formation,
            ExpressiveRole::Force,
            ExpressiveRole::Sequence,
        ],
        &[],
        &[],
        None,
    )
    .unwrap();
    assert_eq!(manifestation.subject_ref, SUBJECT);
    let (identity, sky, drawn) = full_basis();
    let mut score_basis = basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn,
    );
    score_basis.manifestation = Some(&manifestation);
    let score = resolve_tarot_score(&score_basis).unwrap();
    assert_eq!(
        score.manifestation_ref.as_deref(),
        Some(manifestation.manifestation_content_revision.as_str())
    );

    // A manifestation of another subject refuses by name.
    let other = resolve_subject_manifestation(
        native_current_m_registry(),
        "ql:k2/default-subject",
        SubjectKind::Native,
        LOCUS,
        MFace::Bimba,
        &[ExpressiveRole::Formation],
        &[],
        &[],
        None,
    )
    .unwrap();
    let mut score_basis = basis(
        Some(&identity),
        Some(&sky),
        359,
        PrimaryAnchor::Natal { planet_id: 0 },
        &drawn,
    );
    score_basis.manifestation = Some(&other);
    let error = resolve_tarot_score(&score_basis).unwrap_err();
    assert!(
        error.contains("does not match the score subject"),
        "named refusal: {error}"
    );
}
