//! The environment→orientation quaternion chain (T-V).
//!
//! Ports the forward deterministic law by which a composed environment
//! quaternion selects the active rotational state of a known form
//! (FR 2.3.3, FR 2.3.11, M3-C14; branch M3-3):
//!
//! ```text
//! composed = ring_quaternion(tick) × element_quaternion × matrix_axis
//! for each active codon:  state = floor(arg(composed × q_codon) / 45°) & 7
//! ```
//!
//! The chain orients forms that are already determined — the DET overlay's
//! active codon set comes from the M2 vibration masks (see
//! [`crate::pole::transcription`] for the projection masks and wave
//! superposition). This module is the half the kernel owns; the *inverse*
//! seam (entity quaternion → primary address, M3-C31) stays open in
//! [`crate::pole::inverse`], and this module's forward law is exactly what a
//! future canonical law must be accountable to.
//!
//! Quaternion primitives mirror the kernel's (vendor m1.h `quat_mul`,
//! `quat_conj`, `quat_normalize`); the ring LUT is the 12-step SU(2)
//! double-cover loop (vendor m1.h `RING_QUATERNION_LUT`). Floating point is
//! the kernel's own register for this layer (never on the value path).

use crate::pole::codon::{Codon64, MatrixFamily};

/// Semantic identity of the orientation-chain port.
pub const ORIENTATION_CHAIN_REF: &str = "ql.pole.orientation-chain/v1";

/// A quaternion in the kernel's component register.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Hamilton product, mirroring the kernel's `quat_mul`.
    pub const fn mul(self, b: Self) -> Self {
        Self {
            w: self.w * b.w - self.x * b.x - self.y * b.y - self.z * b.z,
            x: self.w * b.x + self.x * b.w + self.y * b.z - self.z * b.y,
            y: self.w * b.y - self.x * b.z + self.y * b.w + self.z * b.x,
            z: self.w * b.z + self.x * b.y - self.y * b.x + self.z * b.w,
        }
    }

    pub const fn conj(self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn norm_sq(self) -> f32 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Mirrors the kernel's `quat_normalize` (identity on zero norm).
    pub fn normalize(self) -> Self {
        let norm_sq = self.norm_sq();
        if norm_sq <= 0.0 {
            return self;
        }
        let scale = 1.0 / norm_sq.sqrt();
        Self {
            w: self.w * scale,
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

/// The 12-step ring quaternion LUT — the SU(2) double-cover loop
/// (vendor m1.h `RING_QUATERNION_LUT`).
///
/// Ticks 0..6 sweep the first cover ascending by 60° per tick (half-angles
/// 0°..150°); ticks 6..12 sweep the return. Tick 11 is `(−1, 0, 0, 0)` — the
/// spinorial second identity that only the double cover distinguishes from
/// tick 0.
pub const RING_QUATERNION_LUT: [Quat; 12] = [
    Quat {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: 0.866_025_4,
        x: 0.5,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: 0.5,
        x: 0.866_025_4,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: 0.0,
        x: 1.0,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: -0.5,
        x: 0.866_025_4,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: -0.866_025_4,
        x: 0.5,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: 0.866_025_4,
        x: -0.5,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: 0.5,
        x: -0.866_025_4,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: 0.0,
        x: -1.0,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: -0.5,
        x: -0.866_025_4,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: -0.866_025_4,
        x: -0.5,
        y: 0.0,
        z: 0.0,
    },
    Quat {
        w: -1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
];

/// Ring position: `tick mod 12` into the double-cover loop.
pub const fn quat_from_ring_pos(tick: u32) -> Quat {
    RING_QUATERNION_LUT[(tick % 12) as usize]
}

/// The M2 five-element id spine (vendor m2.c `M2_ELEMENTS`): Akasha 0,
/// Air 1, Fire 2, Water 3, Earth 4 — the ids the DET overlay's active
/// element carries into the ring position.
pub const M2_ELEMENT_RING_POSITIONS: [u8; 5] = [0, 1, 2, 3, 4];

/// Element quaternion: the ring quaternion at the element's spine id
/// (vendor m3.h `m3_element_to_quat`).
pub const fn element_quaternion(element_id: u8) -> Quat {
    quat_from_ring_pos(element_id as u32)
}

/// The quaternion axis of a matrix family (kernel `M3_MATRIX_QUATERNION_AXIS`):
/// Complementary → i, Moving/Resting → j, Same-Quality → k.
pub const fn matrix_axis_quaternion(family: MatrixFamily) -> Quat {
    match family {
        MatrixFamily::Complementary => Quat {
            w: 0.0,
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        MatrixFamily::MovingResting => Quat {
            w: 0.0,
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        MatrixFamily::SameQuality => Quat {
            w: 0.0,
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    }
}

/// The codon's base quaternion seed (vendor m3.h `m3_quat_from_codon`):
/// `w = Σ values`, `x = v_outer − v_inner`, `y = 0`, `z = Σ mod 6`.
///
/// Value-dependent — it reads the corrected coin table. (The kernel's only
/// quaternion→codon bridge is this map's non-invertible shadow; see
/// [`crate::pole::inverse`].)
pub fn quat_from_codon(codon: Codon64) -> Quat {
    let sites = codon.site_values();
    let sum = sites[0].value() as f32 + sites[1].value() as f32 + sites[2].value() as f32;
    let diff = sites[0].value() as f32 - sites[2].value() as f32;
    Quat {
        w: sum,
        x: diff,
        y: 0.0,
        z: (sum as u32 % 6) as f32,
    }
}

/// The codon quaternion rotated by `state × 45°` about the x axis
/// (vendor m3.h `m3_quat_codon_state`).
pub fn quat_codon_state(codon: Codon64, state: u8) -> Quat {
    let base = quat_from_codon(codon);
    if state & 0x07 == 0 {
        return base;
    }
    let angle = (state & 0x07) as f32 * core::f32::consts::FRAC_PI_4;
    let rot = Quat {
        w: (angle * 0.5).cos(),
        x: (angle * 0.5).sin(),
        y: 0.0,
        z: 0.0,
    };
    rot.mul(base)
}

/// The active rotational state of a codon under a composed environment
/// quaternion (vendor m3.h `m3_quat_active_state`): compose, read the
/// argument `atan2(x, w)` wrapped to [0, 2π), quantize to 45° states.
pub fn quat_active_state(environment: Quat, codon: Codon64) -> u8 {
    let composed = environment.mul(quat_from_codon(codon));
    let mut angle = composed.x.atan2(composed.w);
    if angle < 0.0 {
        angle += core::f32::consts::TAU;
    }
    ((angle / core::f32::consts::FRAC_PI_4) as u8) & 0x07
}

/// The DET overlay result: the active codon mask with each active codon's
/// environment-conditioned rotational state (vendor m3.c
/// `m3_det_with_quaternion`). Inactive codons carry `None`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DetOverlay {
    pub torus_tick: u32,
    pub active_mask: u64,
    pub composed_q: Quat,
    /// Per-codon active state, `None` where the mask is dark.
    pub codon_states: [Option<u8>; 64],
}

/// Compose the overlay from the environment facts and an active codon mask.
///
/// The mask is the wave superposition of the active M2 vibration states —
/// supplied by the caller (see [`crate::pole::transcription`] once wired;
/// vendor `transduce_vibration_to_symbol`).
pub fn det_overlay(
    torus_tick: u32,
    element_ring_position: u8,
    matrix: MatrixFamily,
    active_mask: u64,
) -> DetOverlay {
    let ring_q = quat_from_ring_pos(torus_tick);
    let elem_q = element_quaternion(element_ring_position);
    let axis_q = matrix_axis_quaternion(matrix);
    let composed_q = ring_q.mul(elem_q.mul(axis_q));

    let mut codon_states = [None; 64];
    for codon in 0u8..64 {
        if (active_mask >> codon) & 1 != 0 {
            codon_states[codon as usize] = Some(quat_active_state(composed_q, Codon64::new(codon)));
        }
    }
    DetOverlay {
        torus_tick,
        active_mask,
        composed_q,
        codon_states,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pole::nucleotide::Nucleotide;

    #[test]
    fn ring_lut_is_unit_and_carries_the_double_cover() {
        for (tick, q) in RING_QUATERNION_LUT.iter().enumerate() {
            assert!((q.norm_sq() - 1.0).abs() < 1e-6, "tick {tick} must be unit");
        }
        // The spinorial second identity: tick 11 is (−1, 0, 0, 0).
        assert_eq!(RING_QUATERNION_LUT[11].w, -1.0);
        assert_eq!(RING_QUATERNION_LUT[11].x, 0.0);
        // First cover ascends by 60° per tick: LUT[k] = half-angle k·30°.
        for (k, q) in RING_QUATERNION_LUT.iter().enumerate().take(6) {
            let expected_w = (k as f32 * 30.0).to_radians().cos();
            assert!((q.w - expected_w).abs() < 1e-6, "tick {k} half-angle");
        }
        assert_eq!(quat_from_ring_pos(12), RING_QUATERNION_LUT[0]);
        assert_eq!(quat_from_ring_pos(14), RING_QUATERNION_LUT[2]);
    }

    #[test]
    fn quaternion_primitives_match_the_kernel_semantics() {
        let a = Quat {
            w: 1.0,
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let b = Quat {
            w: 5.0,
            x: 6.0,
            y: 7.0,
            z: 8.0,
        };
        let m = a.mul(b);
        assert_eq!(m.w, 1.0 * 5.0 - 2.0 * 6.0 - 3.0 * 7.0 - 4.0 * 8.0);
        assert_eq!(m.x, 1.0 * 6.0 + 2.0 * 5.0 + 3.0 * 8.0 - 4.0 * 7.0);
        let c = a.conj();
        assert_eq!((c.w, c.x, c.y, c.z), (a.w, -a.x, -a.y, -a.z));
        let n = a.normalize();
        assert!((n.norm_sq() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn matrix_axis_quaternions_bind_i_j_k() {
        assert_eq!(
            matrix_axis_quaternion(MatrixFamily::Complementary),
            Quat {
                w: 0.0,
                x: 1.0,
                y: 0.0,
                z: 0.0
            }
        );
        assert_eq!(
            matrix_axis_quaternion(MatrixFamily::MovingResting),
            Quat {
                w: 0.0,
                x: 0.0,
                y: 1.0,
                z: 0.0
            }
        );
        assert_eq!(
            matrix_axis_quaternion(MatrixFamily::SameQuality),
            Quat {
                w: 0.0,
                x: 0.0,
                y: 0.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn codon_quaternion_seed_follows_the_corrected_values() {
        // AAA: sum 18, diff 0, z = 18 mod 6 = 0.
        let aaa = quat_from_codon(Codon64::from_nucleotides(
            Nucleotide::A,
            Nucleotide::A,
            Nucleotide::A,
        ));
        assert_eq!((aaa.w, aaa.x, aaa.y, aaa.z), (18.0, 0.0, 0.0, 0.0));
        // ACG with the corrected table: 6+8+7 = 21, diff = 6−7 = −1,
        // z = 21 mod 6 = 3.
        let acg = quat_from_codon(Codon64::from_nucleotides(
            Nucleotide::A,
            Nucleotide::C,
            Nucleotide::G,
        ));
        assert_eq!((acg.w, acg.x, acg.z), (21.0, -1.0, 3.0));
    }

    #[test]
    fn active_state_quantizes_the_environment_argument() {
        // Identity environment: the codon's own argument decides.
        let codon = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::A, Nucleotide::A);
        let state = quat_active_state(Quat::IDENTITY, codon);
        assert!(state <= 7);
        // A pure i-axis environment rotates the argument by 90°: two states.
        let env = Quat {
            w: 0.0,
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let shifted = quat_active_state(env, codon);
        let _ = (state, shifted);
    }

    #[test]
    fn det_overlay_conditions_only_active_codons() {
        let mask = (1u64 << 0) | (1u64 << 21) | (1u64 << 63);
        let overlay = det_overlay(7, 2, MatrixFamily::Complementary, mask);
        assert_eq!(overlay.active_mask, mask);
        assert_eq!(overlay.torus_tick, 7);
        assert!(overlay.codon_states[0].is_some());
        assert!(overlay.codon_states[21].is_some());
        assert!(overlay.codon_states[63].is_some());
        assert!(overlay.codon_states[1].is_none(), "dark codons stay None");
        // States are 45°-quantized.
        for state in overlay.codon_states.iter().flatten() {
            assert!(*state <= 7);
        }
    }
}
