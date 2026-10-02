//! The dated sky of 28 Sep 2026 12:00 UTC placed around the clock.
use ql_mef::scene_sky::*;
use serde_json::Value;

fn sky() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
    ))
    .unwrap()
}

#[test]
fn every_body_sits_on_the_clock_through_the_map() {
    let p = place(&sky()).unwrap();
    assert_eq!(p.observer_ref, "#2-5-0/1-0");
    assert_eq!(p.bodies.len(), 10);
    for b in &p.bodies {
        println!(
            "{:8} {:>8.3}° {:?} decan {} card {} codon {} governor {} ({}) rulers {:?} chakra {:?}",
            b.body,
            b.longitude_deg,
            b.planet_ref,
            b.seed.decan_name,
            b.seed.pip_card_ref,
            b.seed.pip_codon_sequence,
            b.seed.governor_ref,
            b.seed.governor_season,
            b.decan_rulers,
            b.resonant_chakra_ref
        );
        assert_eq!(b.seed.relations.len(), 4);
        assert!(
            !b.decan_rulers.is_empty(),
            "{}: every decan has a ruler in the map",
            b.body
        );
    }
    let by = |name: &str| p.bodies.iter().find(|b| b.body == name).unwrap();
    // M2-5 holds the Sun and the planets; the map has no Uranus node.
    assert_eq!(by("Sun").planet_ref.as_deref(), Some("#2-5-0/1"));
    assert_eq!(by("Mars").planet_ref.as_deref(), Some("#2-5-7"));
    assert_eq!(by("Uranus").planet_ref, None);
    // PLANETARY_RESONANCE: Saturn -> Muladhara, Moon -> Ajna, Sun -> Sahasrara.
    assert_eq!(
        by("Saturn").resonant_chakra_ref.as_deref(),
        Some("#2-5-0/1-1")
    );
    assert_eq!(
        by("Moon").resonant_chakra_ref.as_deref(),
        Some("#2-5-0/1-6")
    );
    assert_eq!(by("Sun").resonant_chakra_ref.as_deref(), Some("#2-5-0/1-7"));
    // Accepted D10 anchors are qualified source routes, not legacy C indices.
    assert_eq!(
        by("Neptune").resonant_chakra_ref.as_deref(),
        Some("#2-5-0/1-6")
    );
    assert_eq!(
        by("Pluto").resonant_chakra_ref.as_deref(),
        Some("#2-5-0/1-7")
    );
    assert_eq!(by("Uranus").resonant_chakra_ref, None);
    // The Sun at 185.4° stands in Libra 1, under an autumn governor.
    assert_eq!(by("Sun").seed.decan_name, "Libra Decan 1");
    assert_eq!(by("Sun").seed.governor_season, "Autumn");
    assert!(by("Saturn").retrograde && !by("Sun").retrograde);
}

#[test]
fn moving_one_planet_moves_only_its_own_reading() {
    let base = place(&sky()).unwrap();
    let mut moved = sky();
    moved["bodies"][4]["longitude_degrees"] = Value::from(5.0); // Mars to Aries 1
    let after = place(&moved).unwrap();
    for (a, b) in base.bodies.iter().zip(&after.bodies) {
        if a.body == "Mars" {
            assert_ne!(a.seed, b.seed);
            assert_eq!(b.seed.decan_name, "Aries Decan 1");
            assert!(b.in_own_decan, "Mars rules Aries 1 in the map");
        } else {
            assert_eq!(a, b);
        }
    }
}
