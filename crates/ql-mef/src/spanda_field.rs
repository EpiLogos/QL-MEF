//! The Spanda field: M1's bi-phasal oscillator, the generator beneath the ring.
//!
//! Native owner of the continuous stratum the C kernel carries at
//! `vendor/epi-kernel/reference/src/m1.c` (T2.11, the Spanda dual oscillator;
//! contract in `include/m1.h`). Coordinates: `#1-3` Spanda, with its poles
//! `#1-3-1` / `#1-3-2`. The relative phase φ of the bimba (0/1) and pratibimba
//! (1/0) poles obeys the HKB law
//!
//! ```text
//! φ̇ = Δω − a·sin φ − 2b·sin 2φ,   V(φ) = −a·cos φ − b·cos 2φ
//! ```
//!
//! with derived defaults a = 1, b = 9/16 (the 16/9 generative gap inverted),
//! Δω = 0 (the poles are co-original) and a carrier anchor of 2.5 Hz (the
//! cited delta band). Every magnitude is a tunable parameter, not a constant.
//!
//! Codons advance from the SU(2) rotational state, the clock cycle and the
//! epogdoon (72 → 64), never from the bare `tick12` integer. M3 owns the
//! codon space; this owns only the advancement clock.

use ql_core::{Codon64, Quat, RING_QUATERNION_LUT};
use serde::Serialize;
use std::f64::consts::PI;

/// The twelvefold: the terminal of the flowering (4 → 6 → 8 → 10 → 12).
pub const RING_SIZE: u8 = 12;
/// One 360° cycle of the double cover.
pub const RING_HALF: u8 = 6;
/// The six base lenses of the 72-space.
pub const BASE_LENSES: u64 = 6;
/// The Parashakti 72 (36 tattvas × 2).
pub const PARASHAKTI_TOTAL: u8 = 72;

/// HKB coefficients. Defaults are derived (see module docs); all are tunable.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct HkbParams {
    /// Pole detuning Δω.
    pub delta_omega: f64,
    /// First-harmonic coupling: the fundamental.
    pub a: f64,
    /// Second-harmonic coupling: the octave hook.
    pub b: f64,
    /// Carrier anchor in Hz. A readout only; it does not enter φ̇.
    pub base_freq_hz: f64,
}

impl Default for HkbParams {
    fn default() -> Self {
        Self {
            delta_omega: 0.0,
            a: 1.0,
            b: 9.0 / 16.0,
            base_freq_hz: 2.5,
        }
    }
}

/// The cited delta band, in Hz.
pub const FREQUENCY_BAND_HZ: (f64, f64) = (1.5, 4.0);

impl HkbParams {
    /// φ̇.
    pub fn drift(&self, phi: f64) -> f64 {
        self.delta_omega - self.a * phi.sin() - 2.0 * self.b * (2.0 * phi).sin()
    }
    /// V(φ).
    pub fn potential(&self, phi: f64) -> f64 {
        -self.a * phi.cos() - self.b * (2.0 * phi).cos()
    }
    /// V″(φ). V″(0) = a + 4b (identity is unconditional); V″(π) = 4b − a
    /// (difference stands only while b/a > 1/4).
    pub fn curvature(&self, phi: f64) -> f64 {
        self.a * phi.cos() + 4.0 * self.b * (2.0 * phi).cos()
    }
    /// Integrate φ̇ from `phi0` by forward Euler; the settled phase in (−π, π].
    pub fn settle(&self, phi0: f64, dt: f64, steps: u32) -> f64 {
        let mut phi = phi0;
        for _ in 0..steps {
            phi += dt * self.drift(phi);
        }
        wrap_phase(phi)
    }
}

/// Wrap an angle to (−π, π], exactly as the C kernel does (Rust `%` is C `fmod`).
pub fn wrap_phase(phi: f64) -> f64 {
    let mut wrapped = (phi + PI) % (2.0 * PI);
    if wrapped <= 0.0 {
        wrapped += 2.0 * PI;
    }
    wrapped - PI
}

/// A counter-phase pole wave: pole 0 is the bimba, rightward `cos(x − t)`;
/// pole 1 is the pratibimba, leftward `cos(x + t)`, flipped by π when the
/// poles are swapped (the half-turn, 6 ticks × 30°).
pub fn pole_wave(x: f64, t: f64, pole: u8, pole_swapped: bool) -> f64 {
    if pole == 0 {
        return (x - t).cos();
    }
    let wave = (x + t).cos();
    if pole_swapped { -wave } else { wave }
}

/// The two poles sounding together.
pub fn superposition(x: f64, t: f64, pole_swapped: bool) -> f64 {
    pole_wave(x, t, 0, pole_swapped) + pole_wave(x, t, 1, pole_swapped)
}

/// The standing-wave envelope at x: 2|cos x|, or 2|sin x| when swapped.
pub fn standing_envelope(x: f64, pole_swapped: bool) -> f64 {
    2.0 * if pole_swapped {
        x.sin().abs()
    } else {
        x.cos().abs()
    }
}

/// The half-turn involution on the ring: (n + 6) mod 12.
pub fn half_turn_index(n: u8) -> u8 {
    (n + RING_HALF) % RING_SIZE
}

/// `tick12` as a readout of a continuous cycle phase (radians). Nothing
/// re-grounds on it.
pub fn tick12_readout(cycle_phase: f64) -> u8 {
    let mut norm = cycle_phase % (2.0 * PI);
    if norm < 0.0 {
        norm += 2.0 * PI;
    }
    let tick = (norm / (2.0 * PI) * f64::from(RING_SIZE)) as u8;
    tick % RING_SIZE
}

/// The 72-space address of (lens, helix, position): lens·12 + helix·6 + position.
pub fn resonance_index(lens: u8, helix: u8, position: u8) -> Option<u8> {
    (u64::from(lens) < BASE_LENSES && helix < 2 && position < RING_HALF)
        .then(|| lens * 12 + helix * 6 + position)
}

/// The epogdoon: the canonical 9:8 transduction 72 → 64 (M2 → M3).
pub fn epogdoon(index72: u8) -> u8 {
    ((u16::from(index72) * 8) / 9) as u8
}

/// The active codon from the SU(2) rotational state and the clock cycle.
///
/// `atan2(x, w)` reads the half-angle with its sign, so antipodal states q
/// and −q (one SO(3) face) resolve to arcs six apart. The cycle selects the
/// lens class of the 72-space; the epogdoon compresses to the 64.
pub fn codon_advance(rot: Quat, cycle: u64) -> Codon64 {
    let mut half_angle = f64::from(rot.x).atan2(f64::from(rot.w));
    if half_angle < 0.0 {
        half_angle += 2.0 * PI;
    }
    let arc = ((half_angle / (PI / 6.0) + 0.5) as u8) % RING_SIZE;
    let helix = u8::from(arc >= RING_HALF);
    let position = arc % RING_HALF;
    let lens = (cycle % BASE_LENSES) as u8;
    match resonance_index(lens, helix, position) {
        Some(index) if index < PARASHAKTI_TOTAL => Codon64::new(epogdoon(index)),
        _ => Codon64::new(0),
    }
}

/// The active codon for one M1 generation: the live M1 ring quaternion
/// (`RING_QUATERNION_LUT`, the C `quat_from_ring_pos`) at this tick, advanced
/// through [`codon_advance`]. Ticks 0..6 climb the first cover; 6..12 are its
/// reflected return (arc n ↔ 11 − n), so the tick reaches the codon only
/// through the signed rotational state.
pub fn ring_codon_advance(tick12: u8, cycle: u64) -> Codon64 {
    codon_advance(RING_QUATERNION_LUT[usize::from(tick12 % RING_SIZE)], cycle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_wells_stand_with_the_derived_coupling() {
        let p = HkbParams::default();
        assert!(p.curvature(0.0) > 0.0);
        assert!(p.curvature(PI) > 0.0, "b/a = 9/16 > 1/4 keeps the π well");
        assert!(p.settle(0.3, 0.01, 4000).abs() < 1e-6);
        assert!((p.settle(PI - 0.3, 0.01, 4000).abs() - PI).abs() < 1e-6);
    }

    #[test]
    fn antipodal_states_resolve_six_arcs_apart() {
        let q = Quat {
            w: 0.8,
            x: 0.6,
            y: 0.0,
            z: 0.0,
        };
        let neg = Quat {
            w: -0.8,
            x: -0.6,
            y: 0.0,
            z: 0.0,
        };
        assert_ne!(codon_advance(q, 0), codon_advance(neg, 0));
    }

    #[test]
    fn epogdoon_covers_the_64() {
        let mut seen = [false; 64];
        for i in 0..PARASHAKTI_TOTAL {
            seen[epogdoon(i) as usize] = true;
        }
        assert!(seen.iter().all(|s| *s));
    }
}
