//! M2-1′, the lens's material: how the surface receives and shows the sound.
//!
//! The lens in force gives the surface its material (DLW manual §20.14 Modes;
//! M2′ seed §2.M2-1′ "Lens Resonance Surface / Materializer"). A lens does not
//! pick the pitch: it decides how the surface takes it. The seed's invariants
//! hold here:
//!
//! 1. One material for each lens and position, the same every time.
//! 2. A lens and its partner (Lx and Lx′, the conjugate twins) share their
//!    substrate and invert its orientation: the partner's surface is the
//!    mirror of the lens's, every magnitude m becoming 1 − m and the valence
//!    turning over.
//! 3. Every one of the 72 places has a material.
//! 4. The material is display-only: it never touches a voice.
//!
//! No M2 table holds a lens's material magnitudes: the seed names the fields
//! (colour, roughness, elasticity, particle affinity, tension) and not their
//! values. They are therefore a declared policy, disclosed as such
//! ([`MATERIAL_STANDING`]): the day lens at its place k = 6·index + position on
//! the 36 day places reads that place as an angle α = 2πk/36 round one cosine
//! palette, and the night lens is its twin's mirror.
use std::f64::consts::PI;

use serde::Serialize;

use crate::{LensFace, LensId};

/// The material's standing: a declared policy, not a source reading.
pub const MATERIAL_STANDING: &str = "declared-policy: no M2 table holds a lens's material magnitudes; the M2′ seed (§2.M2-1′) names the fields, not their values";
/// The law the material follows, as the reading discloses it.
pub const MATERIAL_LAW: &str = "M2-1′ (M2′ seed §2.M2-1′): the day lens at place k = 6·index + position reads α = 2πk/36: colour_j = ½ + ½cos(α − 2πj/3), roughness = ¼ + ⅕sin α, elasticity = ½ + ⅖cos α, particle affinity = ½ + ⅖sin 2α, tension = ½ + ⅖cos 2α; its night twin Lx′ is the mirror (1 − m, valence −1). Display-only: no voice reads it.";

/// The surface a lens gives at a position.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LensMaterial {
    /// The lens in force (`L0` … `L5'`).
    pub lens: &'static str,
    /// Its partner, whose surface is this one's mirror.
    pub partner: &'static str,
    /// `day` or `night`.
    pub face: &'static str,
    /// The lens's position, 0..6.
    pub position: u8,
    /// +1 for a day lens, −1 for its night twin: the orientation the pair inverts.
    pub valence: f64,
    /// The surface's colour, each channel 0..1.
    pub colour_rgb: [f64; 3],
    /// 0..1: how irregular the surface's marks are.
    pub roughness: f64,
    /// 0..1: how softly the surface gives under the motion.
    pub elasticity: f64,
    /// 0..1: how readily the particles gather on the surface.
    pub particle_affinity: f64,
    /// 0..1: how sharply the surface holds its lines.
    pub tension: f64,
    pub standing: &'static str,
    pub law: &'static str,
}

/// The material lens `lens` gives at `position` (0..6). Pure and deterministic.
pub fn lens_material(lens: LensId, position: u8) -> Result<LensMaterial, String> {
    if position >= 6 {
        return Err(format!("a lens's position runs 0..6, not {position}"));
    }
    let day = matches!(lens.face(), LensFace::Day);
    let k = 6.0 * f64::from(lens.index()) + f64::from(position);
    let alpha = 2.0 * PI * k / 36.0;
    let side = |m: f64| if day { m } else { 1.0 - m };
    let channel = |j: f64| side(0.5 + 0.5 * (alpha - 2.0 * PI * j / 3.0).cos());
    Ok(LensMaterial {
        lens: lens.code(),
        partner: lens.conjugate_twin().code(),
        face: if day { "day" } else { "night" },
        position,
        valence: if day { 1.0 } else { -1.0 },
        colour_rgb: [channel(0.0), channel(1.0), channel(2.0)],
        roughness: side(0.25 + 0.2 * alpha.sin()),
        elasticity: side(0.5 + 0.4 * alpha.cos()),
        particle_affinity: side(0.5 + 0.4 * (2.0 * alpha).sin()),
        tension: side(0.5 + 0.4 * (2.0 * alpha).cos()),
        standing: MATERIAL_STANDING,
        law: MATERIAL_LAW,
    })
}
