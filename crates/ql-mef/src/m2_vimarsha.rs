//! Active M2-1 reading, retaining the portal-core Vimarśā formula.
//!
//! M1 supplies the harmonic ratio; M3 supplies the pose. Neither dependency
//! is recalculated as a competing kernel here. The seven musical modes are
//! NOT the seven Context Frames or the six internal MEF positions.
use ql_core::RotationalPose;
use serde::{Deserialize, Serialize};

pub const POLICY: &str = "ql.m2-vimarsha/retained-portal-reading-v1";
pub const SOURCE: &str = "EpiLogos/Epi-Logos-C-Experiments@daa660cbc1b8c5da83828698665a753852cb0287:Body/S/S0/portal-core/src/parashakti/vimarsha_reading.rs";
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VimarshaSeed {
    pub tick12: u8,
    /// Direct L0..L5 followed by L0'..L5', not Rust's interleaved LensId.
    pub lens: u8,
    pub musical_mode: u8,
    pub harmonic_ratio: [u16; 2],
    /// Bounded numerical source seeds, NOT a declaration of legal M3 pose.
    pub codon: u8,
    pub rotation: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VimarshaHelix {
    Bimba,
    Pratibimba,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodalConstraint {
    pub ql_position: u8,
    pub helix: VimarshaHelix,
    pub m: u8,
    pub n: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VimarshaReading {
    pub policy_ref: String,
    pub source_ref: String,
    pub source_coordinate: String,
    pub seed: VimarshaSeed,
    pub audio_octet_hz: [f32; 8],
    pub nodal_quartet: [NodalConstraint; 4],
}
const INTERVALS: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const OFFSETS: [u8; 8] = [2, 4, 6, 8, 3, 5, 7, 9];
fn pitch(tick: u8) -> u8 {
    if tick < 6 {
        tick * 2
    } else {
        (tick - 6) * 2 + 1
    }
}
/// Checked finite formula. A caller needing M3 legality uses `read_from_pose`.
pub fn read_seed(seed: VimarshaSeed) -> Result<VimarshaReading, String> {
    let VimarshaSeed {
        tick12: t,
        lens: l,
        musical_mode: m,
        harmonic_ratio: [num, den],
        codon,
        rotation: r,
    } = seed;
    if t >= 12 || l >= 12 || m >= 7 || num == 0 || den == 0 || codon >= 64 || r >= 8 {
        return Err("invalid Vimarsha seed".into());
    }
    let texture = num as f32 / den as f32;
    let mut audio = [0.0; 8];
    for (slot, offset) in OFFSETS.iter().enumerate() {
        let breath = INTERVALS[(slot + m as usize) % 7] as f32 / 7.0;
        let semitones = pitch(l) as f32
            + *offset as f32
            + INTERVALS[m as usize] as f32
            + pitch(t) as f32 / 12.0
            + breath
            + if slot < 4 { 0.0 } else { 1.0 }
            + if t < 6 { 0.0 } else { 12.0 };
        let cents = l as f32 * 3.0 + m as f32 * 5.0 + r as f32;
        audio[slot] = 130.812_79_f32
            * 2.0_f32.powf(semitones / 12.0)
            * texture
            * 2.0_f32.powf(cents / 1200.0);
    }
    let pos = t % 6;
    let ms = l + m + pos + r + codon % 12;
    let n = |q, h, a, b| NodalConstraint {
        ql_position: q,
        helix: h,
        m: 1 + a % 12,
        n: 1 + b % 12,
    };
    Ok(VimarshaReading {
        policy_ref: POLICY.into(),
        source_ref: SOURCE.into(),
        source_coordinate: "#2-1".into(),
        seed,
        audio_octet_hz: audio,
        nodal_quartet: [
            n(0, VimarshaHelix::Bimba, ms, l + pos),
            n(5, VimarshaHelix::Bimba, ms + 5, m + t),
            n(0, VimarshaHelix::Pratibimba, ms + 1, l + m + 4),
            n(5, VimarshaHelix::Pratibimba, ms + 6, t + r + 6),
        ],
    })
}
/// Use an actual shared M3 pose, never a freshly invented M2 rotation table.
pub fn read_from_pose(
    tick12: u8,
    lens: u8,
    musical_mode: u8,
    harmonic_ratio: [u16; 2],
    pose: RotationalPose,
) -> Result<VimarshaReading, String> {
    RotationalPose::new(pose.codon(), pose.slot()).map_err(|e| e.to_string())?;
    read_seed(VimarshaSeed {
        tick12,
        lens,
        musical_mode,
        harmonic_ratio,
        codon: pose.codon().address(),
        rotation: pose.slot(),
    })
}
