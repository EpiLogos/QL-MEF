//! The deterministic entity/event → Tarot score (`ql.tarot-score/v1`).
//!
//! A score derives every token from existing source-defined relations only:
//!
//! - **Anchor resolution** — an admitted basis carries an optional nara
//!   identity reading (`ql.nara-identity-reading/v1`, whose `natal` sky holds
//!   the natal placements the way `nara::intake_composition::natal_composition`
//!   reads them) and an optional occasion sky (`ql.sky-snapshot/v1`, admitted
//!   through `nara::current::transit`, its existing qualified validation).
//! - **Inscription per longitude** — `m3_inscription::seed_at` resolves the
//!   map chain λ → decan → pip → reflection → codon and the backbone governor;
//!   degrees are never flattened straight to codons (M3′-SPEC §8.6/§8.15).
//! - **Card per codon** — the kernel exact cover
//!   (`ql_core::pole::TarotBridge::card_of_codon`, capability M3-C19): 56
//!   Minor Arcana over the 64 codons, 8 dual courts carrying both codons.
//! - **Charge** — `Codon64::four_charge` with the 4X invariant; raw vectors
//!   and the normalised direction are recorded separately, so the four aces
//!   (equal direction, distinct raw identities) never collapse.
//! - **Pose** — the engine's environment composition
//!   (`ql_core::pole::det_overlay`, capability M3-C14): ring quaternion at the
//!   clock's tick12 × element ring quaternion × matrix axis, then
//!   `quat_active_state`, admitted against `rotational_profile` (M3-C13).
//! - **Clock basis** — `ql_core::m3_clock::M3Clock::at_steps` projected on the
//!   score.
//!
//! Computed tokens never touch a journey's draw path; drawn tokens are read
//! from already-drawn journey placements declared with their act references.
//! The Major Arcana are unrepresentable as score tokens by construction: the
//! only card path is the Minor exact cover (see [`MINOR_ONLY_CARD_LOOKUP`]).
//! The codon→card bridge is one-directional by law (`ql_core::pole::inverse`,
//! M3-C31 open); nothing here invents an inverse.

use crate::continuous::{LiftInput, stage};
use crate::coordinate_expression::{SubjectManifestation, validate_subject_ref};
use crate::m_tree::native_m_registry;
use crate::m2_engine::MAX_EXACT_JSON_INTEGER;
use crate::m3_inscription::seed_at;
use crate::m3_state::M3Operation;
use crate::nara::current::transit;
use ql_core::m3_clock::M3Clock;
use ql_core::{
    Codon64, Element, ElementalQuaternionBasis, MatrixFamily, MinorArcanaCard,
    POLE_TAROT_BRIDGE_REF, RotationalPolarity, TarotBridge, TranscendentOperator, det_overlay,
    generate_rotational_states, quat_active_state, quat_codon_state, quat_rotation_degrees,
    rotational_profile,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;

/// Semantic identity of the Tarot score contract.
pub const TAROT_SCORE_CONTRACT: &str = "ql.tarot-score/v1";

/// Semantic identity of the boundary-function record of a declared passage.
pub const TAROT_BOUNDARY_FUNCTIONS_CONTRACT: &str = "ql.tarot-boundary-functions/v1";

/// Compile-level statement of the minors-only invariant: the derivation's only
/// card lookup is the Minor exact cover, so no score token can carry a Major
/// Arcana card — the type admits no other outcome.
const MINOR_ONLY_CARD_LOOKUP: fn(&TarotBridge, Codon64) -> Option<&MinorArcanaCard> =
    TarotBridge::card_of_codon;

/// The ten native planetary bodies, in native id order (the identity the
/// qualified sky validation already enforces: `nara::current::transit` checks
/// `body["native_planet_id"]` and the name against this same order, as does
/// `nara::intake_composition::natal_composition`).
const NATIVE_BODIES: [&str; 10] = [
    "Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto",
];

/// The M2 five-element ring position of a material element — the spine the
/// environment quaternion reads (`ql_core::pole::element_quaternion`; spine
/// Akasha 0, Air 1, Fire 2, Water 3, Earth 4, `vendor/epi-kernel` m2.c
/// `M2_ELEMENTS`, typed at `ql_core::pole::quaternion`).
const fn element_ring_position(element: Element) -> u8 {
    match element {
        Element::Air => 1,
        Element::Fire => 2,
        Element::Water => 3,
        Element::Earth => 4,
    }
}

/// Where a score token's reading comes from. `Computed` is the deterministic
/// anchor derivation of this module; `Drawn` is a card an open journey
/// already dealt, read without dealing again. `AuthoredAssignment` and
/// `Interpretation` name declared origins other producers may record; this
/// constructor never emits them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScoreOrigin {
    Computed,
    Drawn,
    AuthoredAssignment,
    Interpretation,
}

/// The anchor a token stands on, with the resolved longitude and the stable
/// identity of its source (epoch/snapshot/planet — never the longitude value,
/// so a source longitude correction keeps token identity and moves the score
/// revision instead).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ScoreAnchor {
    /// A natal placement of the admitted identity reading.
    Natal {
        planet_id: u8,
        /// The identity reading's own `input_revision`.
        identity_input_revision: String,
        /// The natal sky snapshot reference.
        natal_snapshot_ref: String,
        longitude_degrees: f64,
    },
    /// An occasion-sky body of the admitted dated sky.
    Kairos {
        body_index: u8,
        snapshot_ref: String,
        epoch_utc: String,
        longitude_degrees: f64,
    },
    /// An already-drawn placement of an open journey, declared with its act
    /// reference; the deck is read, never dealt.
    Oracle {
        journey_ref: String,
        placement_ref: String,
        day_ref: String,
    },
}

impl ScoreAnchor {
    /// The stable anchor identity: what a token reference is hashed over.
    /// Longitude values are deliberately absent — a source longitude
    /// correction keeps token identity and changes the score revision.
    fn identity(&self) -> String {
        match self {
            Self::Natal {
                planet_id,
                identity_input_revision,
                natal_snapshot_ref,
                ..
            } => format!("natal:{planet_id}:{identity_input_revision}:{natal_snapshot_ref}"),
            Self::Kairos {
                body_index,
                snapshot_ref,
                epoch_utc,
                ..
            } => format!("kairos:{body_index}:{snapshot_ref}:{epoch_utc}"),
            Self::Oracle {
                journey_ref,
                placement_ref,
                day_ref,
            } => format!("oracle:{journey_ref}:{placement_ref}:{day_ref}"),
        }
    }

    /// The longitude the anchor resolves to, when its source carries one.
    fn longitude_degrees(&self) -> Option<f64> {
        match self {
            Self::Natal {
                longitude_degrees, ..
            }
            | Self::Kairos {
                longitude_degrees, ..
            } => Some(*longitude_degrees),
            Self::Oracle { .. } => None,
        }
    }

    /// Sort rank: source register order (natal, kairos, oracle).
    fn register(&self) -> u8 {
        match self {
            Self::Natal { .. } => 0,
            Self::Kairos { .. } => 1,
            Self::Oracle { .. } => 2,
        }
    }
}

/// The environment-conditioned active pose of one token's codon, composed the
/// way the engine composes it (`ql_core::pole::det_overlay`: ring quaternion
/// at the clock's tick12 × element ring quaternion × matrix axis) and admitted
/// against the dataset-backed rotational profile (7 or 8 lawful states,
/// M3-C13).
///
/// The candidate evidence reports the generation-sweep entry whose 45°
/// register equals the active state — the vendor rotation register
/// (`rotation_degrees = 45 × slot`) and the environment quantisation
/// (`quat_active_state`, 45° states) are the same angle register. No further
/// correspondence between the two laws is claimed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScorePose {
    pub torus_tick12: u8,
    pub element_ring_position: u8,
    /// The matrix family the environment axis is composed through. The engine
    /// declares the axis as caller input (`m3_state::M3Request::matrix_axis`)
    /// and defaults the fold to Complementary (`FoldState::from_codon`);
    /// no source relation derives one per codon, so the engine default is
    /// pinned here rather than invented.
    pub matrix_family: String,
    /// The environment-conditioned active state, 0..8.
    pub active_state: u8,
    /// The profile's lawful state count for this codon: 7 or 8.
    pub state_count: u8,
    /// Whether the active state lies on the profile's lawful surface.
    pub lawfully_admitted: bool,
    /// Whether the candidate at this 45° register is the collapsed bipolar
    /// orientation (both valences emit it identically; the one literal
    /// non-dual state of the sweep).
    pub collapsed_non_dual: bool,
    /// The candidate's rank in the ranked generation sweep.
    pub candidate_slot: u8,
    /// The candidate's valence ("negative"/"positive").
    pub candidate_valence: String,
    /// The candidate's S/D composition value.
    pub candidate_rotational_value: i16,
    /// The candidate's 45° register: 45° × slot.
    pub candidate_rotation_degrees: u16,
    /// The codon's declared form/phase rotor read in the CORRECTED all-axis
    /// register (#312 §5): the physical rotation the rotor carries,
    /// `quat_rotation_degrees(quat_codon_state(codon, active_state))` —
    /// `2·atan2(|v|, w)` in degrees over the composed axis, in [0, 360]. On
    /// argument-zero seeds (outer coin value == inner value) this is exactly
    /// `45° × active_state`, the encoder's own register; the 2π edge folds to
    /// 0. The predecessor published this rotor in two retired i-plane
    /// registers (`phase_clock_steps` over the 720° cover,
    /// `phase_argument_degrees` half-angle); they are retained in
    /// `ql_core::pole::phase` as the regression witness only. Source:
    /// `ql.pole.phase-bridge/v1` (`ql_core::pole::phase`, re-exported from
    /// `ql_core`).
    pub phase_rotation_degrees: u16,
}

/// One token of the score: one anchor, one Minor Arcana card, one or two
/// codons, one inscription seed, one charge reading, one pose.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScoreToken {
    /// Content-addressed identity: digest over (subject_ref, role, anchor
    /// identity). Stable across recomputes and across longitude corrections.
    pub token_ref: String,
    pub anchor: ScoreAnchor,
    /// e.g. "natal/Sun", "kairos/Moon", "oracle/<placement_ref>".
    pub role: String,
    /// The exact-cover card reference, "ql.pole.tarot-bridge/v1#minor:{id}".
    pub card_kernel_ref: String,
    /// 1, or 2 for a dual court (the court's both codons, kernel order).
    pub codons: Vec<u8>,
    /// The codon address — the same 64-address read as hexagram.
    pub hexagram_address: u8,
    /// The inscription seed's lawful map refs (decan/pip/reflection/governor)
    /// at the anchor longitude, exactly as `m3_inscription::seed_at` read them.
    pub inscription: Value,
    /// Raw four-charge vector [pp, mm, mp, pm] with the 4X invariant.
    pub four_charge_raw: [i32; 4],
    /// The raw vector normalised to unit length — direction only.
    pub four_charge_normalised: [f64; 4],
    pub pose: ScorePose,
    pub origin: ScoreOrigin,
    /// The journey placement reference for a drawn token; a procedure
    /// reference for an authored assignment. None for computed tokens.
    pub origin_ref: Option<String>,
}

/// The assembled score: `ql.tarot-score/v1`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TarotScore {
    pub schema: &'static str,
    pub subject_ref: String,
    pub locus_ref: String,
    /// Content hash over the full basis inputs (subject, locus, registry
    /// revision, bridge reference, clock steps, every anchor longitude
    /// bit-exact, identity/transit snapshot identities). Same basis → same
    /// revision, same tokens, same serialisation.
    pub basis_revision: String,
    /// Content hash over the derived score itself: it changes exactly when the
    /// derivation output changes for the same subject.
    pub score_revision: u64,
    /// The M3 clock projection of the basis clock steps.
    pub clock: Value,
    /// Tokens in canonical order: role, then anchor register, then anchor
    /// identity. A repeated archetype in separate roles stays separate tokens.
    pub tokens: Vec<ScoreToken>,
    /// The Fool=opening / Universe=completion boundary readings, recorded as
    /// functions of existing Major Arcana when a passage declares them; empty
    /// when nothing is declared. Never fabricated cards.
    pub boundary_functions: Value,
    /// The manifestation content revision when a resolved
    /// `SubjectManifestation` was supplied for the subject; None otherwise.
    pub manifestation_ref: Option<String>,
    /// The role of the declared primary anchor's token ("natal/Sun",
    /// "kairos/Moon") — the one token the score exists to read and the one a
    /// stage compilation selects. Some whenever a basis declared a primary
    /// anchor, which `resolve_tarot_score` requires.
    pub primary_token_role: Option<String>,
}

impl TarotScore {
    /// The canonical serialisation — byte-identical for the same basis.
    pub fn canonical_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }
}

/// One already-drawn journey placement, declared with its act provenance.
/// The score reads these; it never shuffles, deals or advances a deck.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawnPlacement {
    pub journey_ref: String,
    /// The journey's placement reference — the act reference recorded as the
    /// token's `origin_ref`.
    pub placement_ref: String,
    pub day_ref: String,
    /// The kernel deck index (minors 0-55, majors 56-77). Only minors carry
    /// codons, so only a minor can be a score token.
    pub card: u16,
}

/// The declared primary anchor: the one token the score exists to read.
/// Wire shape follows the score's own anchor convention
/// (`{"kind":"natal","planet_id":0}` / `{"kind":"kairos","body_index":3}`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum PrimaryAnchor {
    Natal { planet_id: u8 },
    Kairos { body_index: u8 },
}

/// The admitted basis of a score resolution.
pub struct ScoreBasis<'a> {
    pub subject_ref: &'a str,
    pub locus_ref: &'a str,
    /// Optional `ql.nara-identity-reading/v1`; its `natal` sky carries the
    /// natal placements.
    pub identity: Option<&'a Value>,
    /// Optional `ql.sky-snapshot/v1` occasion sky.
    pub occasion_sky: Option<&'a Value>,
    /// The occasion's unwrapped M3 clock position.
    pub clock_steps: u64,
    pub primary_anchor: PrimaryAnchor,
    /// Already-drawn journey placements read as drawn tokens.
    pub drawn: &'a [DrawnPlacement],
    /// Optional (opening, completion) passage references declaring the
    /// boundary functions; recorded as functions of the existing boundary
    /// Major Arcana, never as fabricated cards.
    pub boundary_passage: Option<(&'a str, &'a str)>,
    /// Optional resolved manifestation of the subject; its subject must be
    /// this basis's subject.
    pub manifestation: Option<&'a SubjectManifestation>,
}

/// The owned, serde wire form of [`ScoreBasis`] — the same admitted basis an
/// `identity`/`occasion_sky` producer puts on the wire (field host
/// `score-resolve`, `mahamaya.tarot-score.resolve`). [`Self::borrow`]
/// reconstructs the borrowed resolution view; the resolution law itself is
/// and stays [`resolve_tarot_score`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoreBasisInput {
    pub subject_ref: String,
    pub locus_ref: String,
    /// Optional `ql.nara-identity-reading/v1`; its `natal` sky carries the
    /// natal placements.
    pub identity: Option<Value>,
    /// Optional `ql.sky-snapshot/v1` occasion sky.
    pub occasion_sky: Option<Value>,
    /// The occasion's unwrapped M3 clock position.
    pub clock_steps: u64,
    pub primary_anchor: PrimaryAnchor,
    /// Already-drawn journey placements read as drawn tokens.
    #[serde(default)]
    pub drawn: Vec<DrawnPlacement>,
    /// Optional (opening, completion) passage references declaring the
    /// boundary functions; recorded as functions of the existing boundary
    /// Major Arcana, never as fabricated cards.
    pub boundary_passage: Option<(String, String)>,
    /// Optional resolved manifestation of the subject; its subject must be
    /// this basis's subject.
    pub manifestation: Option<SubjectManifestation>,
}

impl ScoreBasisInput {
    /// The borrowed view this input resolves through.
    pub fn borrow(&self) -> ScoreBasis<'_> {
        ScoreBasis {
            subject_ref: &self.subject_ref,
            locus_ref: &self.locus_ref,
            identity: self.identity.as_ref(),
            occasion_sky: self.occasion_sky.as_ref(),
            clock_steps: self.clock_steps,
            primary_anchor: self.primary_anchor,
            drawn: &self.drawn,
            boundary_passage: self
                .boundary_passage
                .as_ref()
                .map(|(opening, completion)| (opening.as_str(), completion.as_str())),
            manifestation: self.manifestation.as_ref(),
        }
    }
}

fn digest_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

/// Bit-exact longitude register for the basis revision: the f64's bits, so
/// any value change moves the revision and no formatting choice can hide one.
fn longitude_register(longitude: f64) -> String {
    format!("{:016x}", longitude.to_bits())
}

/// One admitted anchor source: its anchors and the source identity material
/// the basis revision hashes.
struct AdmittedSource {
    anchors: Vec<ScoreAnchor>,
    identity_input_revision: Option<String>,
    natal_snapshot_ref: Option<String>,
    kairos_snapshot_ref: Option<String>,
    kairos_epoch_utc: Option<String>,
}

/// Admit the natal placements of an identity reading through its `natal` sky,
/// validated by the same qualified sky admission the transit reading uses.
fn admit_natal(identity: &Value) -> Result<AdmittedSource, String> {
    let empty = AdmittedSource {
        anchors: Vec::new(),
        identity_input_revision: None,
        natal_snapshot_ref: None,
        kairos_snapshot_ref: None,
        kairos_epoch_utc: None,
    };
    if identity["schema"] != "ql.nara-identity-reading/v1" {
        return Err("identity anchor requires a ql.nara-identity-reading/v1 reading".into());
    }
    let Some(natal_sky) = identity
        .get("natal")
        .filter(|v| !v.is_null())
        .and_then(|natal| natal.get("sky"))
        .filter(|v| !v.is_null())
    else {
        return Ok(empty);
    };
    let contributions = transit(Some(natal_sky))?["planetary_contributions"].clone();
    let input_revision = identity["input_revision"]
        .as_str()
        .ok_or("identity reading carries no input_revision")?
        .to_string();
    let snapshot_ref = natal_sky["snapshot_ref"]
        .as_str()
        .ok_or("natal sky carries no snapshot reference")?
        .to_string();
    let mut anchors = Vec::with_capacity(contributions.as_array().map_or(0, Vec::len));
    for body in contributions.as_array().ok_or("natal sky bodies absent")? {
        anchors.push(ScoreAnchor::Natal {
            planet_id: body["native_planet_id"]
                .as_u64()
                .ok_or("natal body without a native planet id")? as u8,
            identity_input_revision: input_revision.clone(),
            natal_snapshot_ref: snapshot_ref.clone(),
            longitude_degrees: body["longitude_degrees"]
                .as_f64()
                .ok_or("natal body without a longitude")?,
        });
    }
    Ok(AdmittedSource {
        anchors,
        identity_input_revision: Some(input_revision),
        natal_snapshot_ref: Some(snapshot_ref),
        kairos_snapshot_ref: None,
        kairos_epoch_utc: None,
    })
}

/// Admit the occasion sky through the existing qualified transit validation.
fn admit_kairos(sky: &Value) -> Result<AdmittedSource, String> {
    let reading = transit(Some(sky))?;
    let snapshot_ref = reading["snapshot_ref"]
        .as_str()
        .ok_or("occasion sky carries no snapshot reference")?
        .to_string();
    let epoch_utc = sky["epoch_utc"]
        .as_str()
        .ok_or("occasion sky carries no epoch")?
        .to_string();
    let mut anchors = Vec::new();
    for body in reading["planetary_contributions"]
        .as_array()
        .ok_or("occasion sky bodies absent")?
    {
        anchors.push(ScoreAnchor::Kairos {
            body_index: body["native_planet_id"]
                .as_u64()
                .ok_or("occasion body without a native planet id")? as u8,
            snapshot_ref: snapshot_ref.clone(),
            epoch_utc: epoch_utc.clone(),
            longitude_degrees: body["longitude_degrees"]
                .as_f64()
                .ok_or("occasion body without a longitude")?,
        });
    }
    Ok(AdmittedSource {
        anchors,
        identity_input_revision: None,
        natal_snapshot_ref: None,
        kairos_snapshot_ref: Some(snapshot_ref),
        kairos_epoch_utc: Some(epoch_utc),
    })
}

/// Read the declared drawn placements as oracle anchors. Journey provenance
/// is validated by name; the deck itself is not touched.
fn admit_drawn(drawn: &[DrawnPlacement]) -> Result<Vec<ScoreAnchor>, String> {
    let mut anchors = Vec::with_capacity(drawn.len());
    for placement in drawn {
        for (label, value) in [
            ("journey", &placement.journey_ref),
            ("placement", &placement.placement_ref),
            ("day", &placement.day_ref),
        ] {
            if value.trim().is_empty() || value.len() > 2048 || value.chars().any(char::is_control)
            {
                return Err(format!(
                    "drawn placement carries an invalid {label} reference"
                ));
            }
        }
        if placement.card >= ql_core::MINOR_ARCANA_COUNT as u16 {
            return Err(format!(
                "drawn placement {} carries card {}: a score token is a codon-backed Minor Arcana card; majors and extra cards carry no codon",
                placement.placement_ref, placement.card
            ));
        }
        anchors.push(ScoreAnchor::Oracle {
            journey_ref: placement.journey_ref.clone(),
            placement_ref: placement.placement_ref.clone(),
            day_ref: placement.day_ref.clone(),
        });
    }
    Ok(anchors)
}

/// The token role of an anchor: "natal/Sun", "kairos/Moon",
/// "oracle/<placement_ref>".
fn role_of(anchor: &ScoreAnchor) -> String {
    match anchor {
        ScoreAnchor::Natal { planet_id, .. } => {
            format!("natal/{}", NATIVE_BODIES[*planet_id as usize])
        }
        ScoreAnchor::Kairos { body_index, .. } => {
            format!("kairos/{}", NATIVE_BODIES[*body_index as usize])
        }
        ScoreAnchor::Oracle { placement_ref, .. } => format!("oracle/{placement_ref}"),
    }
}

/// The environment-conditioned pose of one codon, composed exactly as the
/// engine composes the overlay and admitted against the rotational profile.
fn codon_pose(codon: Codon64, clock: M3Clock) -> ScorePose {
    let torus_tick12 = clock.tick12();
    // The card's suit nucleotide is the codon's outer site (the exact-cover
    // law), so the element ring position is the outer nucleotide's element.
    let element = ElementalQuaternionBasis::canonical().element_of(codon.outer());
    let element_ring_position = element_ring_position(element);
    let matrix_family = MatrixFamily::Complementary;
    let overlay = det_overlay(
        u32::from(torus_tick12),
        element_ring_position,
        matrix_family,
        1u64 << codon.address(),
    );
    let active_state = overlay.codon_states[codon.address() as usize]
        .unwrap_or_else(|| quat_active_state(overlay.composed_q, codon));
    let profile = rotational_profile(codon);
    let candidate = &generate_rotational_states(codon)[active_state as usize];
    // The declared form/phase readout: the codon's own state rotor in the
    // corrected all-axis register (#312 §5) — the physical rotation
    // 2·atan2(|v|, w) the rotor carries, in [0, 360].
    let rotor = quat_codon_state(codon, active_state);
    let phase_rotation_degrees = quat_rotation_degrees(&rotor);
    ScorePose {
        torus_tick12,
        element_ring_position,
        matrix_family: "complementary".into(),
        active_state,
        state_count: profile.state_count(),
        lawfully_admitted: active_state < profile.state_count(),
        collapsed_non_dual: candidate.is_non_dual,
        candidate_slot: candidate.rotation_slot,
        candidate_valence: match candidate.polarity {
            RotationalPolarity::Negative => "negative".into(),
            RotationalPolarity::Positive => "positive".into(),
        },
        candidate_rotational_value: candidate.rotational_value,
        candidate_rotation_degrees: candidate.rotation_degrees,
        phase_rotation_degrees,
    }
}

/// One codon-backed card reading of one anchor: the card reference, its
/// codons and the inscription record. A computed anchor inscribes the map
/// seed at its longitude; a drawn anchor inscribes its card's own exact-cover
/// reading — no longitude is claimed for a dealt card.
fn card_reading(
    anchor: &ScoreAnchor,
    basis: &ScoreBasis,
) -> Result<(String, Vec<u8>, Value, Codon64), String> {
    let bridge = TarotBridge::kernel();
    let (codon, inscription) = match anchor {
        ScoreAnchor::Natal {
            longitude_degrees, ..
        }
        | ScoreAnchor::Kairos {
            longitude_degrees, ..
        } => {
            let seed = seed_at(*longitude_degrees)?;
            let codon = Codon64::new(seed.pip_codon);
            let inscription = serde_json::to_value(&seed).map_err(|e| e.to_string())?;
            (codon, inscription)
        }
        ScoreAnchor::Oracle { placement_ref, .. } => {
            let placement = basis
                .drawn
                .iter()
                .find(|p| &p.placement_ref == placement_ref)
                .ok_or_else(|| format!("drawn anchor {placement_ref} has no declared placement"))?;
            let card = &bridge.minor()[usize::from(placement.card)];
            let codon = card.codon_a();
            let inscription = json!({
                "standing": "drawn-card-exact-cover-reading; no longitude inscription is claimed",
                "card_id": card.card_id(),
                "suit": card.suit().name(),
                "pip": card.pip().rws_name(),
                "codons": card.codons().map(|c| c.address()).collect::<Vec<_>>(),
            });
            (codon, inscription)
        }
    };
    let card = MINOR_ONLY_CARD_LOOKUP(bridge, codon)
        .ok_or_else(|| format!("codon {} is uncovered by the exact cover", codon.address()))?;
    let codons = card.codons().map(|c| c.address()).collect::<Vec<_>>();
    let card_kernel_ref = format!("{POLE_TAROT_BRIDGE_REF}#minor:{}", card.card_id());
    Ok((card_kernel_ref, codons, inscription, codon))
}

/// The boundary functions of a declared passage: roles of the existing
/// boundary Major Arcana (`TranscendentOperator`, M3-C20), never fabricated
/// cards and never codon addresses.
pub fn declared_boundary_functions(
    opening_passage_ref: &str,
    completion_passage_ref: &str,
) -> Result<Value, String> {
    for (label, reference) in [
        ("opening", opening_passage_ref),
        ("completion", completion_passage_ref),
    ] {
        if reference.trim().is_empty() || reference.chars().any(char::is_control) {
            return Err(format!(
                "boundary passage carries an invalid {label} reference"
            ));
        }
    }
    let operator = |op: TranscendentOperator, passage_ref: &str| {
        let major = &TarotBridge::kernel().major()[usize::from(op.major_index())];
        json!({
            "passage_ref": passage_ref,
            "operator": format!("{op:?}").to_lowercase(),
            "role": op.role(),
            "boundary_value": op.boundary_value(),
            "major_card_ref": format!("{POLE_TAROT_BRIDGE_REF}#major:{}", major.card_id()),
            "major_name": major.name(),
            "quaternion_id": op.quaternion_id(),
        })
    };
    Ok(json!({
        "schema": TAROT_BOUNDARY_FUNCTIONS_CONTRACT,
        "opening": operator(TranscendentOperator::Fool, opening_passage_ref),
        "completion": operator(TranscendentOperator::Universe, completion_passage_ref),
        "standing": "declared-passage-functions-of-existing-boundary-major-arcana; no fabricated cards, no codon addresses",
    }))
}

/// Resolve the deterministic Tarot score of an admitted basis. Same basis
/// values resolve to a byte-identical serialised score.
pub fn resolve_tarot_score(basis: &ScoreBasis) -> Result<TarotScore, String> {
    validate_subject_ref(basis.subject_ref)?;
    validate_subject_ref(basis.locus_ref)?;
    let clock = M3Clock::at_steps(basis.clock_steps);

    // Anchor admission: natal and kairos are each admitted through their
    // existing qualified validations; drawn placements are read by act ref.
    let mut anchors = Vec::new();
    let mut identity_input_revision = None;
    let mut natal_snapshot_ref = None;
    let mut kairos_snapshot_ref = None;
    let mut kairos_epoch_utc = None;
    for admitted in [
        basis.identity.map(admit_natal).transpose()?,
        basis.occasion_sky.map(admit_kairos).transpose()?,
    ]
    .into_iter()
    .flatten()
    {
        anchors.extend(admitted.anchors);
        identity_input_revision = admitted.identity_input_revision.or(identity_input_revision);
        natal_snapshot_ref = admitted.natal_snapshot_ref.or(natal_snapshot_ref);
        kairos_snapshot_ref = admitted.kairos_snapshot_ref.or(kairos_snapshot_ref);
        kairos_epoch_utc = admitted.kairos_epoch_utc.or(kairos_epoch_utc);
    }
    anchors.extend(admit_drawn(basis.drawn)?);
    if anchors.is_empty() {
        return Err(
            "no anchor basis: neither identity natal placements nor an occasion sky is admitted"
                .into(),
        );
    }

    // The declared primary anchor must be present, by name.
    let primary_identity = match basis.primary_anchor {
        PrimaryAnchor::Natal { planet_id } => {
            if usize::from(planet_id) >= NATIVE_BODIES.len() {
                return Err(format!(
                    "declared primary natal anchor names planet id {planet_id}, outside the ten native bodies"
                ));
            }
            anchors
                .iter()
                .filter_map(|a| match a {
                    ScoreAnchor::Natal {
                        planet_id: found, ..
                    } if *found == planet_id => Some(a.identity()),
                    _ => None,
                })
                .next()
                .ok_or_else(|| {
                    format!(
                        "declared primary natal anchor {} is absent from the admitted identity",
                        NATIVE_BODIES[usize::from(planet_id)]
                    )
                })?
        }
        PrimaryAnchor::Kairos { body_index } => {
            if usize::from(body_index) >= NATIVE_BODIES.len() {
                return Err(format!(
                    "declared primary kairos anchor names body index {body_index}, outside the ten native bodies"
                ));
            }
            anchors
                .iter()
                .filter_map(|a| match a {
                    ScoreAnchor::Kairos {
                        body_index: found, ..
                    } if *found == body_index => Some(a.identity()),
                    _ => None,
                })
                .next()
                .ok_or_else(|| {
                    format!(
                        "declared primary kairos anchor {} is absent from the admitted occasion sky",
                        NATIVE_BODIES[usize::from(body_index)]
                    )
                })?
        }
    };

    // The manifestation link must belong to this subject.
    let manifestation_ref = match basis.manifestation {
        Some(manifestation) => {
            if manifestation.subject_ref != basis.subject_ref {
                return Err(format!(
                    "manifestation subject {} does not match the score subject {}",
                    manifestation.subject_ref, basis.subject_ref
                ));
            }
            Some(manifestation.manifestation_content_revision.clone())
        }
        None => None,
    };

    // One token per anchor; repeated archetypes in separate roles stay
    // separate tokens and consume nothing.
    let mut tokens = Vec::with_capacity(anchors.len());
    for anchor in &anchors {
        let role = role_of(anchor);
        let token_ref = format!(
            "sha256:{}",
            digest_hex(
                format!(
                    "{}\u{0}{}\u{0}{}",
                    basis.subject_ref,
                    role,
                    anchor.identity()
                )
                .as_bytes()
            )
        );
        let (card_kernel_ref, codons, inscription, codon) = card_reading(anchor, basis)?;
        let hexagram_address = codon.address();
        let pose = codon_pose(codon, clock);
        let charge = codon.four_charge();
        let raw = [
            i32::from(charge.pp),
            i32::from(charge.mm),
            i32::from(charge.mp),
            i32::from(charge.pm),
        ];
        let norm = (raw.iter().map(|v| v * v).sum::<i32>() as f64).sqrt();
        let normalised = if norm > 0.0 {
            raw.map(|v| f64::from(v) / norm)
        } else {
            raw.map(f64::from)
        };
        tokens.push(ScoreToken {
            token_ref,
            anchor: anchor.clone(),
            role,
            card_kernel_ref,
            codons,
            hexagram_address,
            inscription,
            four_charge_raw: raw,
            four_charge_normalised: normalised,
            pose,
            origin: match anchor {
                ScoreAnchor::Oracle { .. } => ScoreOrigin::Drawn,
                _ => ScoreOrigin::Computed,
            },
            origin_ref: match anchor {
                ScoreAnchor::Oracle { placement_ref, .. } => Some(placement_ref.clone()),
                _ => None,
            },
        });
    }
    tokens.sort_by(|a, b| {
        a.role
            .cmp(&b.role)
            .then_with(|| a.anchor.register().cmp(&b.anchor.register()))
            .then_with(|| a.anchor.identity().cmp(&b.anchor.identity()))
    });

    // Basis revision: content hash over the full basis inputs.
    let mut basis_material = String::new();
    for part in [
        basis.subject_ref,
        basis.locus_ref,
        &native_m_registry().manifest().registry_revision,
        POLE_TAROT_BRIDGE_REF,
        &basis.clock_steps.to_string(),
        identity_input_revision.as_deref().unwrap_or(""),
        natal_snapshot_ref.as_deref().unwrap_or(""),
        kairos_snapshot_ref.as_deref().unwrap_or(""),
        kairos_epoch_utc.as_deref().unwrap_or(""),
        &primary_identity,
    ] {
        basis_material.push_str(part);
        basis_material.push('\u{0}');
    }
    for token in &tokens {
        if let Some(longitude) = token.anchor.longitude_degrees() {
            basis_material.push_str(&token.role);
            basis_material.push('\u{0}');
            basis_material.push_str(&longitude_register(longitude));
            basis_material.push('\u{0}');
        }
    }
    let basis_revision = format!("sha256:{}", digest_hex(basis_material.as_bytes()));
    let boundary_functions = match basis.boundary_passage {
        Some((opening, completion)) => declared_boundary_functions(opening, completion)?,
        None => json!({}),
    };

    // The declared primary's token role — the one token the score exists to
    // read and the one a stage compilation selects. The primary match above
    // already validated the body index against the ten native bodies.
    let primary_token_role = match basis.primary_anchor {
        PrimaryAnchor::Natal { planet_id } => {
            format!("natal/{}", NATIVE_BODIES[usize::from(planet_id)])
        }
        PrimaryAnchor::Kairos { body_index } => {
            format!("kairos/{}", NATIVE_BODIES[usize::from(body_index)])
        }
    };

    let mut score = TarotScore {
        schema: TAROT_SCORE_CONTRACT,
        subject_ref: basis.subject_ref.to_string(),
        locus_ref: basis.locus_ref.to_string(),
        basis_revision,
        score_revision: 0,
        clock: json!({
            "steps": clock.steps(),
            "degree720": clock.degree720(),
            "degree360": clock.degree360(),
            "layer": clock.layer(),
            "tick12": clock.tick12(),
            "completed_double_covers": clock.completed_double_covers(),
        }),
        tokens,
        boundary_functions,
        manifestation_ref,
        primary_token_role: Some(primary_token_role),
    };
    // The score revision is the content hash of the score itself: it moves
    // exactly when the derivation output changes for the same subject.
    let canonical = score.canonical_json()?;
    let digest = digest_hex(canonical.as_bytes());
    let mut high = [0u8; 8];
    high.copy_from_slice(&digest.as_bytes()[..8]);
    score.score_revision = u64::from_be_bytes(high);
    Ok(score)
}

/// Compile the score's primary determination into one stage procedure: the
/// primary token's form address and admitted pose select the form, the
/// score's clock basis is the clock change, and the declared material policy
/// rides as the damping change. One procedure, one change per slot — the
/// stage's own law (`crate::continuous::stage`); the scene flow composes
/// passages. The score itself is never applied here: evaluation and
/// application stay with the stage's host paths.
pub fn score_stage_procedure(
    score: &TarotScore,
    procedure_ref: &str,
    revision: u64,
    subject_ref: &str,
    damping_per_second: f64,
) -> Result<stage::StageProcedure, String> {
    let Some(primary_role) = &score.primary_token_role else {
        return Err(
            "the score carries no primary token; nothing compiles into a stage procedure".into(),
        );
    };
    let Some(primary) = score
        .tokens
        .iter()
        .find(|token| &token.role == primary_role)
    else {
        return Err(format!(
            "the score's primary token {primary_role} is absent from its own tokens"
        ));
    };
    if subject_ref != score.subject_ref {
        return Err(format!(
            "procedure subject {subject_ref} is not the score's subject {}",
            score.subject_ref
        ));
    }
    if !damping_per_second.is_finite() || !(0.0..=1e6).contains(&damping_per_second) {
        return Err("score stage damping must be finite and in 0..1000000 per second".into());
    }
    // The score's clock basis as the clock change: unwrapped steps split into
    // the stage clock phase's canonical turns + half_degrees register.
    let steps = score
        .clock
        .get("steps")
        .and_then(Value::as_u64)
        .ok_or("the score's clock basis carries no steps")?;
    if steps > MAX_EXACT_JSON_INTEGER {
        return Err("the score's clock basis exceeds the exact native range".into());
    }
    let phase = LiftInput {
        turns: (steps / 720).to_string(),
        half_degrees: (steps % 720) as u16,
    };
    // The primary's form: its address always selects; the pose is applied
    // only where the rotational profile admits it.
    let mut operations = vec![M3Operation::SelectForm {
        address: primary.hexagram_address,
    }];
    if primary.pose.lawfully_admitted {
        operations.push(M3Operation::SetPose {
            pose: primary.pose.active_state,
        });
    }
    let procedure = stage::StageProcedure {
        schema: stage::STAGE_PROCEDURE.into(),
        procedure_ref: procedure_ref.to_owned(),
        revision,
        subject_ref: subject_ref.to_owned(),
        trigger: stage::StageTrigger::Invocation,
        selector: vec![
            "form".into(),
            "material.damping".into(),
            "clock.inscription".into(),
        ],
        changes: vec![
            stage::StageChange::Form { operations },
            stage::StageChange::Damping {
                per_second: damping_per_second,
            },
            stage::StageChange::Clock {
                slot: "clock.inscription".into(),
                phase,
            },
        ],
        passage: None,
    };
    procedure.validate()?;
    Ok(procedure)
}
