//! Conformance for `ql.pole.phase-bridge/v1` — the source-qualified phase
//! bridge between the three coexisting native angle registers (the codon
//! encoder, the argument reader, the M3 clock).
//!
//! The relations pinned here are the ones the module header derives: the
//! encoder sits at its physical register (45°/state, exact on argument-zero
//! seeds), the raw classifier shifts by 4 under antipodal negation away from
//! bin boundaries (interior and boundary cases bounded separately), the ring
//! LUT maps onto the 720° double-cover triangle, the 30° and 60° tick
//! registers meet two clock ticks per LUT tick, and pose admission is the
//! dataset profile carved out of the always-8 candidate sweep.
//!
//! `ql_core::pole` is not wired past the item re-exports yet, so — the
//! established pole-test pattern (`tests/pole_rotational.rs`) — the new
//! module compiles unmodified inside this test crate through a pole-shaped
//! shim that re-exports the very same `ql_core` type.

mod quaternion {
    pub use ql_core::Quat;
}
#[path = "../src/pole/phase.rs"]
mod phase;

use phase::{
    PHASE_BRIDGE_REF, quat_argument_bin, quat_clock_steps, quat_signed_argument,
    ring_tick_clock_steps,
};
use ql_core::m3_clock::M3Clock;
use ql_core::{
    Codon64, Nucleotide, Quat, RING_QUATERNION_LUT, ROTATIONAL_TABLE_ENTRIES, RotationalPose,
    generate_rotational_states, quat_codon_state, quat_from_codon, rotational_profile,
};

/// Rotor whose quaternion-plane argument is exactly `radians`:
/// `(cos(radians), sin(radians), 0, 0)` — the half-angle construction with
/// the physical angle at twice the argument.
fn rotor_at_argument(radians: f32) -> Quat {
    Quat {
        w: radians.cos(),
        x: radians.sin(),
        y: 0.0,
        z: 0.0,
    }
}

fn neg(q: &Quat) -> Quat {
    Quat {
        w: -q.w,
        x: -q.x,
        y: -q.y,
        z: -q.z,
    }
}

#[test]
fn phase_bridge_contract_is_versioned() {
    assert_eq!(PHASE_BRIDGE_REF, "ql.pole.phase-bridge/v1");
}

#[test]
fn codon_state_encoder_sits_at_its_physical_register() {
    // AAA is an argument-zero seed (outer value == inner value, seed
    // argument atan2(0, 18) = 0), so the encoder register relation is exact:
    // the encoder steps the PHYSICAL rotation 45° per state.
    let codon = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::A, Nucleotide::A);
    for s in 0u8..8 {
        let q = quat_codon_state(codon, s);
        let expected = (45 * u64::from(s)) % 720;
        let steps_f = quat_signed_argument(&q) * (360.0 / core::f32::consts::PI);
        assert!(
            (steps_f.rem_euclid(720.0) - expected as f32).abs() < 1e-4,
            "s={s}: float register {} vs expected {expected}",
            steps_f.rem_euclid(720.0)
        );
        assert_eq!(quat_clock_steps(&q), expected, "s={s} physical register");
    }
    // Wrong-register negative case: treating the half-angle rotor as a
    // full-angle rotor would predict 90°/state. Must fail at every state
    // where the two readings differ (1..=7 here).
    for s in 1u8..8 {
        let q = quat_codon_state(codon, s);
        assert_ne!(
            quat_clock_steps(&q),
            (90 * u64::from(s)) % 720,
            "s={s}: 90°/state misregister must not read"
        );
    }
    // General codon (ACG): the seed argument rides along — the w-x subplane
    // of the product multiplies as complex numbers, so the register relation
    // is (45·s + 2·seed_argument) mod 720, and the seed offset is exactly
    // what the module header states.
    let acg = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::C, Nucleotide::G);
    let seed_steps = quat_signed_argument(&quat_from_codon(acg)) * (360.0 / core::f32::consts::PI);
    let offset = seed_steps.rem_euclid(720.0).round() as i64;
    for s in 0u8..8 {
        let expected = (45 * i64::from(s) + offset).rem_euclid(720) as u64;
        assert_eq!(
            quat_clock_steps(&quat_codon_state(acg, s)),
            expected,
            "s={s}: seed offset {offset}"
        );
    }
}

#[test]
fn antipodal_negation_shifts_the_raw_classifier_by_four() {
    let codons = [
        Codon64::from_nucleotides(Nucleotide::A, Nucleotide::A, Nucleotide::A),
        Codon64::from_nucleotides(Nucleotide::A, Nucleotide::C, Nucleotide::G),
    ];
    // Interior environment arguments; the composed argument must stay >= 5°
    // away from every 45° bin boundary, verified numerically per case below.
    for codon in codons {
        for deg in [10.0f32, 100.0, 190.0, 280.0] {
            let env = rotor_at_argument(deg.to_radians());
            let q = env.mul(quat_from_codon(codon));
            let remainder = quat_signed_argument(&q).to_degrees().rem_euclid(45.0);
            let margin = remainder.min(45.0 - remainder);
            assert!(
                margin >= 5.0,
                "codon {} at {deg}° sits {margin}° from a bin boundary; \
                 interior cases must hold >= 5° — fix the chosen cases",
                codon.address()
            );
            let b1 = quat_argument_bin(&q);
            let b2 = quat_argument_bin(&neg(&q));
            assert_eq!(
                (b2 + 8 - b1) % 8,
                4,
                "codon {} at {deg}°: bin {b1} vs negated {b2}",
                codon.address()
            );
        }
    }
}

#[test]
fn bin_boundary_cases_are_separately_bounded() {
    for k in 0u32..8 {
        // Rotor built at the exact boundary argument 45·k degrees.
        let q = rotor_at_argument((45.0 * k as f32).to_radians());
        let n = neg(&q);
        // Both readings are boundary-class: within 1e-3° of a 45° multiple.
        for arg_deg in [
            quat_signed_argument(&q).to_degrees(),
            quat_signed_argument(&n).to_degrees(),
        ] {
            let remainder = arg_deg.rem_euclid(45.0);
            let dist = remainder.min(45.0 - remainder);
            assert!(
                dist < 1e-3,
                "k={k}: constructed argument {arg_deg}° is {dist}° off a boundary"
            );
        }
        let t1 = quat_signed_argument(&q) / core::f32::consts::FRAC_PI_4;
        let t2 = {
            let t = quat_signed_argument(&n) / core::f32::consts::FRAC_PI_4;
            if t < 0.0 { t + 8.0 } else { t }
        };
        let b1 = quat_argument_bin(&q);
        let b2 = quat_argument_bin(&n);
        if t1.fract() == 0.0 && t2.fract() == 0.0 {
            // Exactly representable boundary: the 180° shift of an exact
            // boundary is an exact boundary — both bins pin.
            assert_eq!(b1, (k % 8) as u8, "k={k} exact boundary bin");
            assert_eq!(b2, ((k + 4) % 8) as u8, "k={k} exact negated bin");
        } else {
            // f32 rounding put a hair on one side of the boundary; either
            // floor can flip by one bin. The antipodal relation survives as
            // 4 or 4±1 (wrapping). This hair-flip class is exactly why the
            // interior and boundary cases are separate tests.
            let d = (b2 + 8 - b1) % 8;
            assert!(
                d == 3 || d == 4 || d == 5,
                "k={k}: bins {b1}/{b2} differ by {d}, expected 4 or 4±1"
            );
        }
    }
}

#[test]
fn ring_lut_ticks_map_to_the_double_cover_triangle() {
    for t in 0u32..12 {
        let expected = if t <= 5 {
            60 * u64::from(t)
        } else {
            720 - 60 * (u64::from(t) - 5)
        };
        assert_eq!(ring_tick_clock_steps(t), expected, "tick {t} piecewise");
        let read = quat_clock_steps(&RING_QUATERNION_LUT[t as usize]);
        assert!(
            read.abs_diff(expected) <= 1,
            "tick {t}: quat_clock_steps reads {read}, piecewise says {expected}"
        );
    }
    // Tick 0 is the bimba identity; tick 11 is (−1,0,0,0), physical 360° —
    // the pratibimba identity, second sheet.
    assert_eq!(ring_tick_clock_steps(0), 0);
    assert_eq!(ring_tick_clock_steps(11), 360);
    assert_eq!(M3Clock::at_steps(360).layer(), 1, "tick 11 sits on sheet 1");
}

#[test]
fn clock_and_lut_ticks_are_the_30_and_60_degree_registers() {
    for t in 0u32..12 {
        let steps = ring_tick_clock_steps(t);
        let clock = M3Clock::at_steps(steps);
        assert_eq!(
            u64::from(clock.degree720()),
            steps,
            "tick {t} within one cover"
        );
        // One LUT tick = two 30° clock ticks (D5; m1.h:512 asserts the
        // vendor's own ratio). Ascending half: dial ticks 2t and 2t+1 — the
        // "2t mod 24" identity of the brief. Return half: the same six 60°
        // slots in reverse, dial ticks (24 − 2(t−5)) and +1; tick12 reads
        // degree360/30 (the dial mod 12), so there the register reads the
        // mirror 2·(11−t) — pinned as such.
        if t <= 5 {
            assert_eq!(steps / 30, 2 * u64::from(t), "tick {t} dial position");
            assert_eq!(
                u64::from(clock.tick12()),
                (2 * u64::from(t)) % 24,
                "tick {t} tick12"
            );
        } else {
            assert_eq!(
                steps / 30,
                u64::from(24 - 2 * (t - 5)) % 24,
                "tick {t} dial position"
            );
            assert_eq!(
                u64::from(clock.tick12()),
                2 * u64::from(11 - t),
                "tick {t} tick12 mirror"
            );
        }
    }
    assert_eq!(M3Clock::at_steps(720).completed_double_covers(), 1);
    // Sheet collapse negative case: steps 0 and 360 share degree360 —
    // reading only degree360 loses the cover; layer() is what separates them.
    let zero = M3Clock::at_steps(0);
    let mid = M3Clock::at_steps(360);
    assert_eq!(zero.degree360(), mid.degree360());
    assert_ne!(zero.layer(), mid.layer());
}

#[test]
fn seven_or_eight_state_admission_is_the_profile_not_the_classifier() {
    let mut seven_state = 0usize;
    let mut eight_state = 0usize;
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        let profile = rotational_profile(codon);
        let count = profile.state_count();
        assert!(count == 7 || count == 8, "codon {address} count {count}");
        // The classifier agrees entry-for-entry; the profile is the gate.
        assert_eq!(count, codon.rotational_state_count(), "codon {address}");
        if count == 7 {
            seven_state += 1;
        } else {
            eight_state += 1;
        }
        // The profile is dataset provenance over the candidate sweep
        // (src/pole/rotational.rs:19-38, the two-registers and port-truth
        // records, carried from the 2026-10-03 day-note refutation): the
        // sweep always generates all 8 ranked slots; the profile count
        // admits the lawful subset of exactly state_count members.
        let ranked = generate_rotational_states(codon);
        assert_eq!(ranked.len(), ROTATIONAL_TABLE_ENTRIES);
        let mut lawful = 0usize;
        for slot in 0..ROTATIONAL_TABLE_ENTRIES as u8 {
            match RotationalPose::new(codon, slot) {
                Ok(pose) => {
                    lawful += 1;
                    assert_eq!(pose.slot(), slot);
                    assert_eq!(
                        ranked[slot as usize].rotation_slot, slot,
                        "codon {address} slot {slot}"
                    );
                }
                Err(_) => assert!(
                    slot >= count,
                    "codon {address}: slot {slot} below count {count} was refused"
                ),
            }
        }
        assert_eq!(lawful, count as usize, "codon {address} lawful subset");
    }
    assert_eq!(seven_state, 40);
    assert_eq!(eight_state, 24);
    assert_eq!(seven_state * 7 + eight_state * 8, 472);
}
