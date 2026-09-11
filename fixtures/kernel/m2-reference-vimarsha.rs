use serde::{Deserialize, Serialize};

use crate::codon_rotation_projection::{codon_rotation_from_lens_mode, MathemeLensMode};
use crate::kernel::{harmonic_ratio_fraction_for_sub_tick, KernelTick, MathemeNodalConstraint};

const C3_HZ: f32 = 130.812_79;
const INNER_FOUR_OFFSETS: [u8; 8] = [2, 4, 6, 8, 3, 5, 7, 9];
const MODE_INTERVALS: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VimarshaReading {
    pub audio_octet: [f32; 8],
    pub nodal_quartet: [MathemeNodalConstraint; 4],
}

pub fn vimarsha_read_profile(tick: KernelTick, lens_mode: MathemeLensMode) -> VimarshaReading {
    let tick12 = tick.sub_tick % 12;
    let substrate_pitch = pitch_class_for_tick(tick12);
    let lens_anchor = pitch_class_for_tick(lens_mode.lens);
    let (ratio_num, ratio_den) = harmonic_ratio_fraction_for_sub_tick(tick12);
    let epogdoon_texture = ratio_num as f32 / ratio_den as f32;
    let surface = codon_rotation_from_lens_mode(lens_mode.lens, lens_mode.mode)
        .expect("valid lens-mode maps into the codon-rotation surface");

    VimarshaReading {
        audio_octet: audio_octet(
            tick12,
            substrate_pitch,
            lens_anchor,
            lens_mode,
            epogdoon_texture,
            surface.rotation_degrees,
        ),
        nodal_quartet: nodal_quartet(tick12, lens_mode, surface.codon_id, surface.rotation),
    }
}

fn audio_octet(
    tick12: u8,
    substrate_pitch: u8,
    lens_anchor: u8,
    lens_mode: MathemeLensMode,
    epogdoon_texture: f32,
    rotation_degrees: u16,
) -> [f32; 8] {
    let mode_anchor = MODE_INTERVALS[lens_mode.mode as usize] as f32;
    let substrate_color = substrate_pitch as f32 / 12.0;
    let lens_color_cents = lens_mode.lens as f32 * 3.0;
    let mode_color_cents = lens_mode.mode as f32 * 5.0;
    let rotation_color_cents = rotation_degrees as f32 / 45.0;
    let octave_lift = if tick12 < 6 { 0.0 } else { 12.0 };
    let mut out = [0.0f32; 8];

    for (slot, offset) in INNER_FOUR_OFFSETS.iter().enumerate() {
        let helix_lift = if slot < 4 { 0.0 } else { 1.0 };
        let modal_breath =
            MODE_INTERVALS[(slot + lens_mode.mode as usize) % MODE_INTERVALS.len()] as f32 / 7.0;
        let semitones = lens_anchor as f32
            + *offset as f32
            + mode_anchor
            + substrate_color
            + modal_breath
            + helix_lift
            + octave_lift;
        let cents = lens_color_cents + mode_color_cents + rotation_color_cents;
        out[slot] = hz_from_c3_semitones(semitones) * epogdoon_texture * cents_ratio(cents);
    }

    out
}

fn nodal_quartet(
    tick12: u8,
    lens_mode: MathemeLensMode,
    codon_id: u8,
    rotation: u8,
) -> [MathemeNodalConstraint; 4] {
    let position = tick12 % 6;
    let seed = lens_mode.lens + lens_mode.mode + position + rotation + (codon_id % 12);

    [
        nodal_constraint(0, "bimba", seed, lens_mode.lens + position),
        nodal_constraint(5, "bimba", seed + 5, lens_mode.mode + tick12),
        nodal_constraint(
            0,
            "pratibimba",
            seed + 1,
            lens_mode.lens + lens_mode.mode + 4,
        ),
        nodal_constraint(5, "pratibimba", seed + 6, tick12 + rotation + 6),
    ]
}

fn nodal_constraint(
    ql_position: u8,
    helix: &'static str,
    m_seed: u8,
    n_seed: u8,
) -> MathemeNodalConstraint {
    MathemeNodalConstraint {
        ql_position,
        helix: helix.to_owned(),
        m: 1 + (m_seed % 12),
        n: 1 + (n_seed % 12),
    }
}

fn pitch_class_for_tick(tick12: u8) -> u8 {
    match tick12 % 12 {
        0 => 0,
        1 => 2,
        2 => 4,
        3 => 6,
        4 => 8,
        5 => 10,
        6 => 1,
        7 => 3,
        8 => 5,
        9 => 7,
        10 => 9,
        _ => 11,
    }
}

fn hz_from_c3_semitones(semitones: f32) -> f32 {
    C3_HZ * 2.0f32.powf(semitones / 12.0)
}

fn cents_ratio(cents: f32) -> f32 {
    2.0f32.powf(cents / 1200.0)
}
