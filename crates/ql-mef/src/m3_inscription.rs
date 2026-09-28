//! The M3-5 clock inscription seed, read from the Bimba map at an ecliptic degree.
//!
//! The clock is a geometric codon–hexagram construction; its inscription is
//! dynamic (M3′-SPEC §8.6, §8.15). What the map fixes is the seed, and this
//! owner reads it through the map's own relations:
//!
//! - **Clock A (θ = λ):** the degree's decan (`#2-3-*`, named `"<Sign> Decan <k>"`,
//!   ten degrees each from λ 0° = Aries, the zodiac being lens 9)
//!   `EXPRESSES_AS_TAROT_PIP` its pip card; the card's `#3-4.0` reflection
//!   `GOVERNS_TAROT_EXPRESSION` it and `REFLECTS_DNA_FORM` its codon.
//! - **Backbone:** the M3-5 wheel's degree 0/360 is the North anchor's winter
//!   solstice (`#3-5-1`, `MANIFESTS_AT_DEGREE`), the ecliptic's λ 270°. The
//!   degree node is `ANCHORED_BY` one of the 24 governors, whose authored
//!   description names its season's perfect palindrome.
//!
//! The live form at that place is not a table lookup: [`active_pose`] composes
//! the seed codon with an environment quaternion (`quat_active_state`), which
//! is where the rotational states and the three matrices enter.

use crate::m_tree::{MRegistry, MTreeId, native_m_registry};
use crate::m3_source::{M3Source, native_m3_source};
use ql_core::{Codon64, Quat, quat_active_state};
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Lens 9's twelve signs in ecliptic order from λ 0°.
pub const ZODIAC: [&str; 12] = [
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
    "Pisces",
];

/// Ecliptic longitude of the winter solstice: the M3-5 wheel's 0/360.
pub const WINTER_SOLSTICE_LONGITUDE: u16 = 270;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InscriptionSeed {
    pub longitude_deg: f64,
    pub decan_ref: String,
    pub decan_name: String,
    pub pip_card_ref: String,
    pub reflection_ref: String,
    pub pip_codon_ref: String,
    pub pip_codon: u8,
    pub pip_codon_sequence: String,
    pub wheel_degree: u16,
    pub degree_ref: String,
    pub governor_ref: String,
    pub governor_season: String,
    pub governor_palindrome: u8,
    /// Every relation read, in chain order, by its map reference.
    pub relations: Vec<String>,
}

/// The seed codon and its active rotational state under an environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ActivePose {
    pub codon: u8,
    pub rotational_state: u8,
}

fn codon_of(sequence: &str) -> Result<Codon64, String> {
    // A = 00, T = 01, C = 10, G = 11 (M3′-SPEC §8.15), as `m3_source` indexes.
    if sequence.len() != 3 {
        return Err(format!("not a codon: {sequence}"));
    }
    sequence
        .bytes()
        .try_fold(0u8, |acc, n| {
            b"ATCG"
                .iter()
                .position(|b| *b == n)
                .map(|i| (acc << 2) | i as u8)
        })
        .map(Codon64::new)
        .ok_or_else(|| format!("not a codon: {sequence}"))
}

fn decan_index(registry: &MRegistry) -> &'static BTreeMap<String, MTreeId> {
    static INDEX: OnceLock<BTreeMap<String, MTreeId>> = OnceLock::new();
    INDEX.get_or_init(|| {
        registry
            .manifest()
            .nodes
            .iter()
            .filter(|n| n.source_ref.starts_with("#2-3-"))
            .flat_map(|n| n.names.iter().map(move |name| (name.clone(), n.id)))
            .collect()
    })
}

fn one<T>(mut found: impl Iterator<Item = T>, what: &str) -> Result<T, String> {
    let first = found.next().ok_or_else(|| format!("map holds no {what}"))?;
    if found.next().is_some() {
        return Err(format!("map holds more than one {what}"));
    }
    Ok(first)
}

/// The inscription seed at an ecliptic longitude in [0, 360).
pub fn seed_at(longitude_deg: f64) -> Result<InscriptionSeed, String> {
    if !longitude_deg.is_finite() || !(0.0..360.0).contains(&longitude_deg) {
        return Err("longitude outside [0, 360)".into());
    }
    let registry = native_m_registry();
    let source: &M3Source = native_m3_source();
    let mut relations = Vec::new();

    // Clock A: λ -> decan -> pip card -> reflection -> codon.
    let sign = ZODIAC[(longitude_deg / 30.0) as usize];
    let k = ((longitude_deg % 30.0) / 10.0) as usize + 1;
    let decan_name = format!("{sign} Decan {k}");
    let decan_id = *decan_index(registry)
        .get(&decan_name)
        .ok_or_else(|| format!("map holds no decan named {decan_name}"))?;
    let decan = registry.node(decan_id).ok_or("decan id unresolved")?;
    let pip = one(
        registry
            .relations_for(decan_id)
            .filter(|r| r.from_id == Some(decan_id) && r.source_kind == "EXPRESSES_AS_TAROT_PIP"),
        &format!("pip card for {decan_name}"),
    )?;
    relations.push(pip.relation_ref.clone());
    let card_ref = pip.to_ref.clone().ok_or("pip card unresolved")?;
    let governs = one(
        source.relations().iter().filter(|r| {
            r.kind == "GOVERNS_TAROT_EXPRESSION" && r.to_ref.as_deref() == Some(&card_ref)
        }),
        &format!("reflection governing {card_ref}"),
    )?;
    relations.push(governs.reference.clone());
    let reflection_ref = governs.from_ref.clone().ok_or("reflection unresolved")?;
    let reflection = source
        .resolve(&reflection_ref)
        .ok_or("reflection not in M3")?;
    let reflects = one(
        source
            .outgoing(reflection.id)
            .filter(|r| r.kind == "REFLECTS_DNA_FORM"),
        &format!("DNA form of {reflection_ref}"),
    )?;
    relations.push(reflects.reference.clone());
    let codon_ref = reflects.to_ref.clone().ok_or("codon unresolved")?;
    let codon_node = source.resolve(&codon_ref).ok_or("codon not in M3")?;
    let sequence = codon_node.properties["p_3_sequence"]
        .as_str()
        .ok_or("codon node carries no sequence")?
        .to_string();
    let pip_codon = codon_of(&sequence)?;

    // Backbone: λ -> wheel degree -> degree node -> governor -> palindrome.
    let wheel_degree = ((longitude_deg as u16) + 360 - WINTER_SOLSTICE_LONGITUDE) % 360;
    let degree_ref = if wheel_degree == 0 {
        "#3-5-5/0-0/360".to_string()
    } else {
        format!("#3-5-5/0-{wheel_degree}")
    };
    let degree = source.resolve(&degree_ref).ok_or("degree not in M3")?;
    let anchored = one(
        source
            .outgoing(degree.id)
            .filter(|r| r.kind == "ANCHORED_BY"),
        &format!("governor of {degree_ref}"),
    )?;
    relations.push(anchored.reference.clone());
    let governor_ref = anchored.to_ref.clone().ok_or("governor unresolved")?;
    let governor = source.resolve(&governor_ref).ok_or("governor not in M3")?;
    let description = governor.properties["c_1_description"]
        .as_str()
        .ok_or("governor carries no description")?;
    let palindrome = description
        .strip_prefix("Perfect palindromic ")
        .and_then(|rest| rest.get(..3))
        .filter(|p| p.bytes().all(|b| Some(b) == p.bytes().next()))
        .ok_or_else(|| format!("governor {governor_ref} names no perfect palindrome"))?;
    let governor_palindrome = codon_of(palindrome)?;
    let governor_season = governor.properties["t_3_season"]
        .as_str()
        .ok_or("governor carries no season")?
        .to_string();

    Ok(InscriptionSeed {
        longitude_deg,
        decan_ref: decan.source_ref.clone(),
        decan_name,
        pip_card_ref: card_ref,
        reflection_ref,
        pip_codon_ref: codon_ref,
        pip_codon: pip_codon.address(),
        pip_codon_sequence: sequence,
        wheel_degree,
        degree_ref,
        governor_ref,
        governor_season,
        governor_palindrome: governor_palindrome.address(),
        relations,
    })
}

/// The seed codon's live pose: its active rotational state under the
/// environment quaternion (the ring × element × matrix composition).
pub fn active_pose(seed: &InscriptionSeed, environment: Quat) -> ActivePose {
    let codon = Codon64::new(seed.pip_codon);
    ActivePose {
        codon: seed.pip_codon,
        rotational_state: quat_active_state(environment, codon),
    }
}
