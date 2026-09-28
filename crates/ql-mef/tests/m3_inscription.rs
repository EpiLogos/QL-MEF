//! The M3-5 inscription seed read from the map, over the whole ecliptic.
use ql_core::Quat;
use ql_mef::m3_inscription::*;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn every_degree_reads_its_seed_through_the_map() {
    let mut decans = BTreeSet::new();
    let mut governors: BTreeMap<String, (String, u8)> = BTreeMap::new();
    for degree in 0..360u16 {
        let s = seed_at(f64::from(degree) + 0.5).unwrap();
        assert_eq!(s.relations.len(), 4, "{s:?}");
        decans.insert((s.decan_ref.clone(), s.pip_card_ref.clone(), s.pip_codon));
        governors.insert(
            s.governor_ref.clone(),
            (s.governor_season.clone(), s.governor_palindrome),
        );
    }
    assert_eq!(
        decans.len(),
        36,
        "36 decans, each with one pip card and codon"
    );
    assert_eq!(
        governors.len(),
        24,
        "every wheel degree reaches one of the 24 governors"
    );
    // Each season's six governors carry its palindrome: G, A, C, T from the solstice.
    let by_season: BTreeMap<&str, BTreeSet<u8>> =
        governors
            .values()
            .fold(BTreeMap::new(), |mut m, (season, p)| {
                m.entry(season.as_str()).or_default().insert(*p);
                m
            });
    for (season, sequence) in [
        ("Winter", 0b111111u8),
        ("Spring", 0),
        ("Summer", 0b101010),
        ("Autumn", 0b010101),
    ] {
        assert_eq!(by_season[season], BTreeSet::from([sequence]), "{season}");
    }
}

#[test]
fn first_decan_of_aries_is_the_two_of_wands_on_tta_in_spring() {
    let s = seed_at(5.0).unwrap();
    assert_eq!(s.decan_name, "Aries Decan 1");
    assert_eq!(s.decan_ref, "#2-3-1-0-0");
    assert_eq!(s.pip_codon_sequence, "TTA");
    // λ 5° is wheel degree 95: past the spring equinox (λ 0° = wheel 90°).
    assert_eq!(s.wheel_degree, 95);
    assert_eq!(s.governor_season, "Spring");
    // The winter solstice is the wheel's 0/360.
    let solstice = seed_at(270.0).unwrap();
    assert_eq!(
        (solstice.wheel_degree, solstice.degree_ref.as_str()),
        (0, "#3-5-5/0-0/360")
    );
    assert_eq!(solstice.governor_season, "Winter");
}

#[test]
fn the_live_pose_moves_with_the_environment_not_the_table() {
    let s = seed_at(123.0).unwrap();
    let states: BTreeSet<u8> = (0..24)
        .map(|k| {
            let h = f32::from(k as u8) * std::f32::consts::PI / 12.0;
            active_pose(
                &s,
                Quat {
                    w: h.cos(),
                    x: h.sin(),
                    y: 0.0,
                    z: 0.0,
                },
            )
            .rotational_state
        })
        .collect();
    assert!(
        states.len() > 1,
        "a disconnected environment would hold one state"
    );
    assert!(seed_at(360.0).is_err() && seed_at(f64::NAN).is_err());
}
