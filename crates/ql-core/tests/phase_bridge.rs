//! Conformance for `ql.pole.phase-bridge/v1` — the source-qualified phase
//! bridge between the native angle registers (the codon encoder, the
//! CORRECTED all-axis reader, the M3 clock) and the retired i-plane register
//! retained as the predecessor's regression witness (#312 §5).
//!
//! The relations pinned here are the ones the module header derives: the
//! encoder sits at its physical register and the corrected reader agrees
//! with it on the ascending half (45°·s, exact on argument-zero seeds,
//! folding at the 2π edge); the corrected antipodal law is the octant MIRROR
//! (`bin(−q) = (7 − bin(q)) mod 8`), not the retired +4; the retired i-plane
//! witness keeps its +4 diagnostic on the retired functions by name; the
//! ring LUT maps onto the 720° double-cover triangle and, read on the
//! corrected direction-blind register, onto the 60°-magnitude table with the
//! 360° fold at tick 11; and pose admission is the dataset profile carved
//! out of the always-8 candidate sweep.
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
    PHASE_BRIDGE_REF, quat_argument_bin, quat_argument_bin_retired_i_plane,
    quat_clock_steps_retired_i_plane, quat_rotation_degrees, quat_signed_argument,
    ring_tick_clock_steps,
};
use ql_core::m3_clock::M3Clock;
use ql_core::{
    Codon64, Nucleotide, Quat, RING_QUATERNION_LUT, ROTATIONAL_TABLE_ENTRIES, RotationalPose,
    generate_rotational_states, quat_active_state, quat_active_state_retired_i_plane,
    quat_codon_state, quat_from_codon, rotational_profile,
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

/// The corrected rotation angle of `q` in degrees — the register
/// `quat_argument_bin` quantises (`2·atan2(|v|, w)` in [0, 360]), spelled
/// out so tests can verify interior margins against the 45° grid.
fn corrected_angle_degrees(q: &Quat) -> f32 {
    let vmag = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
    let angle = 2.0 * vmag.atan2(q.w);
    (if angle < 0.0 {
        angle + core::f32::consts::TAU
    } else {
        angle
    })
    .to_degrees()
}

/// Distance in degrees from the nearest 45° bin boundary of the corrected
/// register (0 exactly on a boundary).
fn corrected_boundary_margin(q: &Quat) -> f32 {
    let remainder = corrected_angle_degrees(q) % 45.0;
    remainder.min(45.0 - remainder)
}

#[test]
fn phase_bridge_contract_is_versioned() {
    assert_eq!(PHASE_BRIDGE_REF, "ql.pole.phase-bridge/v1");
}

#[test]
fn codon_state_encoder_sits_at_its_physical_register() {
    // AAA is an argument-zero seed (outer value == inner value, seed
    // (18, 0, 0, 0)), so the corrected register relation is exact: the
    // encoder's rotor at state s carries the physical rotation 45·s, and
    // s = 8 folds to 0 — the 2π edge the vendor comment folds back through
    // the &0x07.
    let codon = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::A, Nucleotide::A);
    for s in 0u8..=8 {
        let q = quat_codon_state(codon, s);
        let expected = (45 * u16::from(s)) % 360;
        assert_eq!(
            quat_rotation_degrees(&q),
            expected,
            "s={s}: corrected rotation register"
        );
        let degrees = corrected_angle_degrees(&q);
        assert!(
            (degrees - expected as f32).abs() < 1e-3,
            "s={s}: float register {degrees} vs expected {expected}"
        );
    }
    // Wrong-register negative case: reading the half-angle rotor as if each
    // state were 90° of physical rotation must fail at every state where the
    // two readings differ (1..=7; s=0 and the s=8 fold both sit at 0).
    for s in 1u8..8 {
        let q = quat_codon_state(codon, s);
        assert_ne!(
            quat_rotation_degrees(&q),
            (90 * u16::from(s)) % 360,
            "s={s}: 90°/state misregister must not read"
        );
    }
    // General codon (ACG, seed (21, −1, 0, 3)): the seed's w-x pair rotates
    // as a complex number while the Mod term z rides the magnitude — the
    // derived law is φ(s) = 2·atan2(sqrt((c·X + s·W)² + Z²), c·W − s·X) with
    // c = cos(22.5°·s), s = sin(22.5°·s), (W, X, Z) = the seed. Pinned
    // against that closed form (all cases ≥ 2° clear of a bin boundary).
    let acg = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::C, Nucleotide::G);
    let seed = quat_from_codon(acg);
    assert_eq!((seed.w, seed.x, seed.y, seed.z), (21.0, -1.0, 0.0, 3.0));
    for s in 0u8..=7 {
        let theta = (core::f32::consts::FRAC_PI_4 / 2.0) * f32::from(s);
        let (c, sin) = (theta.cos(), theta.sin());
        let xy = c * seed.x + sin * seed.w;
        let w = c * seed.w - sin * seed.x;
        let expected = 2.0 * (xy * xy + seed.z * seed.z).sqrt().atan2(w);
        let q = quat_codon_state(acg, s);
        let degrees = corrected_angle_degrees(&q);
        assert!(
            (degrees - expected.to_degrees()).abs() < 1e-3,
            "s={s}: derived {}° vs read {degrees}°",
            expected.to_degrees()
        );
        let margin = corrected_boundary_margin(&q);
        assert!(
            margin >= 2.0,
            "s={s}: {margin}° from a boundary; interior cases must hold >= 2°"
        );
    }
}

#[test]
fn encoder_and_reader_agree_on_the_ascending_half() {
    // AAA's seed is pure w (18, 0, 0, 0) — verified here — so the state
    // rotor IS the composed value's direction and the corrected reader must
    // bin the encoder's states at their own register: state s → bin s, with
    // s = 8 folding to 0 (the 2π edge).
    //
    // These rotors sit ON the 45° grid (the measured angle is 45·s within an
    // f32 hair) — the boundary class, not the interior class: the reading
    // relies on the f32 construction landing on or above each boundary, so
    // each case is verified to sit within 1e-3° of its boundary and the bin
    // is then pinned as measured. (Interior-mirror behaviour is a different
    // test; the vendor's own agreement claim is this boundary-class pin.)
    let codon = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::A, Nucleotide::A);
    let seed = quat_from_codon(codon);
    assert_eq!((seed.w, seed.x, seed.y, seed.z), (18.0, 0.0, 0.0, 0.0));
    for s in 0u8..=7 {
        let q = quat_codon_state(codon, s);
        let margin = corrected_boundary_margin(&q);
        assert!(
            margin < 1e-3,
            "s={s}: measured angle sits {margin}° from the 45°·s boundary; \
             the ascending-half pin expects the on-boundary class"
        );
        assert_eq!(quat_argument_bin(&q), s, "s={s}: encoder==reader");
    }
    // s = 8 is the fold: quat_codon_state masks &7 (returning the s = 0
    // rotor) and the quantiser's &0x07 folds the 360° edge back to 0 — both
    // routes land on bin 0, per the vendor comment.
    assert_eq!(quat_argument_bin(&quat_codon_state(codon, 8)), 0);
}

#[test]
fn retired_i_plane_witness_shifts_the_raw_classifier_by_four() {
    // The RETIRED law's own diagnostic (#312 §5 witness): on the i-plane
    // half-angle register, negation shifts the argument by 180° — a full
    // +4 bin move — executed here against the retired functions by name.
    // This relation belongs to the retired register ONLY; the corrected
    // law's antipodal relation is the mirror (`mirror_is_the_corrected_
    // antipodal_law` below).
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
            let b1 = quat_argument_bin_retired_i_plane(&q);
            let b2 = quat_argument_bin_retired_i_plane(&neg(&q));
            assert_eq!(
                (b2 + 8 - b1) % 8,
                4,
                "codon {} at {deg}°: retired bin {b1} vs negated {b2}",
                codon.address()
            );
            // The retired state function carries the same register as its
            // made-total quantiser.
            assert_eq!(
                quat_active_state_retired_i_plane(env, codon),
                b1,
                "codon {} at {deg}°: witness state == retired bin",
                codon.address()
            );
        }
    }
    // Retired boundary class: at an exact i-plane boundary (BOTH measured
    // arguments exactly representable as 45° multiples) the negated reading
    // is the exact (k+4) mod 8; a boundary a hair off in f32 — including
    // the atan2 π itself, which the runtime libm returns one ulp low — can
    // flip each floor by one bin, so the hair class allows 4 ± 1.
    for k in 0u32..8 {
        let q = rotor_at_argument((45.0 * k as f32).to_radians());
        let n = neg(&q);
        let b1 = quat_argument_bin_retired_i_plane(&q);
        let b2 = quat_argument_bin_retired_i_plane(&n);
        let t1 = quat_signed_argument(&q) / core::f32::consts::FRAC_PI_4;
        let t2 = {
            let t = quat_signed_argument(&n) / core::f32::consts::FRAC_PI_4;
            if t < 0.0 { t + 8.0 } else { t }
        };
        if t1.fract() == 0.0 && t2.fract() == 0.0 {
            assert_eq!(b1, (k % 8) as u8, "k={k} exact retired boundary bin");
            assert_eq!(b2, ((k + 4) % 8) as u8, "k={k} exact negated bin");
        } else {
            let d = (b2 + 8 - b1) % 8;
            assert!(
                d == 3 || d == 4 || d == 5,
                "k={k}: retired bins {b1}/{b2} differ by {d}, expected 4 or 4±1"
            );
        }
    }
}

#[test]
fn mirror_is_the_corrected_antipodal_law() {
    // The CORRECTED antipodal law (#312 §5; module derivation): |v| is
    // negation-invariant and atan2(|v|, −w) = π − atan2(|v|, w) for
    // |v| >= 0, so the corrected angle mirrors — φ(−q) = 360° − φ(q) — and
    // an interior point of bin k mirrors into the interior of bin 7−k:
    // quat_argument_bin(−q) = (7 − quat_argument_bin(q)) mod 8, the octant
    // mirror n → 9−n. NOT the retired +4.
    //
    // Rotors: pure w-x rotors at eight physical angles plus four all-axis
    // compositions (j and k components non-zero — the axes the corrected
    // |v| reads), each verified numerically >= 2° clear of a bin boundary
    // before the relation is asserted (margins are negation-invariant, so
    // one check per pair suffices).
    let mut cases: Vec<Quat> = Vec::new();
    for deg in [10.0f32, 50.0, 100.0, 150.0, 200.0, 260.0, 310.0, 350.0] {
        cases.push(rotor_at_argument((deg / 2.0).to_radians()));
    }
    let seeds = [
        Codon64::from_nucleotides(Nucleotide::A, Nucleotide::A, Nucleotide::A),
        Codon64::from_nucleotides(Nucleotide::A, Nucleotide::C, Nucleotide::G),
    ];
    for seed in seeds {
        let s = quat_from_codon(seed);
        cases.push(
            Quat {
                w: 1.0,
                x: 0.0,
                y: 0.5,
                z: 0.0,
            }
            .mul(s),
        );
        cases.push(
            Quat {
                w: 0.5,
                x: 0.5,
                y: 0.5,
                z: 0.5,
            }
            .mul(s),
        );
    }
    assert!(
        cases.len() >= 6,
        "the mirror law must be executed on at least six rotors"
    );
    for q in &cases {
        let margin = corrected_boundary_margin(q);
        assert!(
            margin >= 2.0,
            "rotor sits {margin}° from a bin boundary; interior cases must \
             hold >= 2° — fix the chosen cases"
        );
        let b1 = quat_argument_bin(q);
        let b2 = quat_argument_bin(&neg(q));
        let mirrored = (7 + 8 - b1) % 8;
        assert_eq!(b2, mirrored, "bin(−q) = {b2}, mirror of {b1} is {mirrored}");
        // The angle itself mirrors: φ(q) + φ(−q) = 360°.
        let sum = corrected_angle_degrees(q) + corrected_angle_degrees(&neg(q));
        assert!(
            (sum - 360.0).abs() < 1e-3,
            "mirrored angles sum to {sum}°, expected 360°"
        );
    }
}

#[test]
fn corrected_antipodal_at_bin_boundaries_is_the_floor_convention() {
    // At an EXACT 45° multiple the mirrored angle is also exact (45·(8−k))
    // and the floor convention reads bin (8−k) mod 8 — not 7−k, and not the
    // retired +4. A boundary a hair off in f32 flips each floor by one bin:
    // φ = 45k + δ with |δ| hair-small gives (b1, b2) = (k, 7−k) for δ > 0
    // and (k−1, (8−k) mod 8) for δ < 0 — so b1 + b2 ≡ 0 (mod 8) on the
    // exact class and ≡ 7 (mod 8) on the hair class, never the retired 4.
    for k in 0u32..8 {
        let q = rotor_at_argument((45.0 * k as f32).to_radians());
        let n = neg(&q);
        for angle in [corrected_angle_degrees(&q), corrected_angle_degrees(&n)] {
            let remainder = angle % 45.0;
            let dist = remainder.min(45.0 - remainder);
            assert!(
                dist < 1e-2,
                "k={k}: constructed angle {angle}° is {dist}° off a boundary"
            );
        }
        let b1 = quat_argument_bin(&q);
        let b2 = quat_argument_bin(&n);
        let sum = (u16::from(b1) + u16::from(b2)) % 8;
        assert!(
            sum == 0 || sum == 7,
            "k={k}: boundary bins {b1}+{b2} ≡ {sum} (mod 8), expected 0 or 7"
        );
    }
    // The 2π edge, pinned as measured: the identity (atan2(0, 1) = 0
    // exactly) reads bin 0. Its negation — (−1, 0, 0, 0), |v| = 0 — sits on
    // the atan2 π itself, and the runtime libm returns π one ulp low
    // (0x40490fda), so the doubled angle lands a hair under 2π and the
    // floor reads bin 7: this is precisely the vendor comment's own "w < 0
    // (opposite hemisphere) reaches the composite state 7", with the &0x07
    // 2π-fold as the exact-arithmetic statement of the same edge (an
    // optimized build that constant-folds atan2 to exact π_f32 reads 0
    // there). The hair class, not an exact pin: allow {0, 7}.
    assert_eq!(quat_argument_bin(&Quat::IDENTITY), 0);
    let antipodal_identity = quat_argument_bin(&neg(&Quat::IDENTITY));
    assert!(
        antipodal_identity == 0 || antipodal_identity == 7,
        "the antipodal identity reads {antipodal_identity}, expected the \
         2π-edge hair class {{0, 7}}"
    );
}

#[test]
fn j_torque_and_codon_mod_now_contribute() {
    // The DR-ENV correction reason, executed on BOTH laws (#312 §5). Codon
    // AGT (corrected values 6+7+9 = 22, diff 6−9 = −3, Mod 22 mod 6 = 4):
    // two environments identical except the j (y) torque — 0 vs 0.376 —
    // produce DIFFERENT corrected states while the retired i-plane law reads
    // the SAME state for both: the retired reader sees only (x, w), and the
    // j torque moves neither (the x perturbation it does produce, via the
    // codon's Mod, stays inside its bin by >= 2°).
    let codon = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::G, Nucleotide::T);
    let seed = quat_from_codon(codon);
    assert_eq!((seed.w, seed.x, seed.y, seed.z), (22.0, -3.0, 0.0, 4.0));
    let env_rest = Quat {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let env_torque = Quat {
        w: 1.0,
        x: 0.0,
        y: 0.376,
        z: 0.0,
    };
    let corrected_rest = quat_active_state(env_rest, codon);
    let corrected_torque = quat_active_state(env_torque, codon);
    assert_eq!(corrected_rest, 0, "identity environment reads AGT at bin 0");
    assert_eq!(
        corrected_torque, 1,
        "the j torque crosses the codon into bin 1"
    );
    // ...while the retired law cannot see the torque at all.
    let retired_rest = quat_active_state_retired_i_plane(env_rest, codon);
    let retired_torque = quat_active_state_retired_i_plane(env_torque, codon);
    assert_eq!(retired_rest, retired_torque, "retired law: same state");
    // Margins, verified: corrected bins clear by >= 2° on both environments,
    // retired bin clear by >= 2° on both.
    assert!(corrected_boundary_margin(&env_rest.mul(seed)) >= 2.0);
    assert!(corrected_boundary_margin(&env_torque.mul(seed)) >= 2.0);
    for env in [env_rest, env_torque] {
        let composed = env.mul(seed);
        let arg = quat_signed_argument(&composed).to_degrees();
        let remainder = arg.rem_euclid(45.0);
        assert!(remainder.min(45.0 - remainder) >= 2.0, "retired margin");
    }
    // The codon's Mod (k/z = Σ mod 6) is load-bearing: ablate the composed
    // quaternion's z and the corrected bin drops back to 0 — the Mod term
    // is part of what carries the crossing. Ablate the j component instead
    // and it also drops: the torque is load-bearing through |v| too.
    let composed = env_torque.mul(seed);
    let ablated_z = Quat {
        w: composed.w,
        x: composed.x,
        y: composed.y,
        z: 0.0,
    };
    let ablated_y = Quat {
        w: composed.w,
        x: composed.x,
        y: 0.0,
        z: composed.z,
    };
    assert!(corrected_boundary_margin(&ablated_z) >= 2.0);
    assert!(corrected_boundary_margin(&ablated_y) >= 2.0);
    assert_eq!(quat_argument_bin(&composed), 1);
    assert_eq!(quat_argument_bin(&ablated_z), 0, "the Mod term carries it");
    assert_eq!(quat_argument_bin(&ablated_y), 0, "the j torque carries it");
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
        // The retired i-plane conversion reads the same triangle (this is
        // its own register's mapping, kept as the witness's conformance).
        let read = quat_clock_steps_retired_i_plane(&RING_QUATERNION_LUT[t as usize]);
        assert!(
            read.abs_diff(expected) <= 1,
            "tick {t}: retired conversion reads {read}, piecewise says {expected}"
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
    // The CORRECTED rotation reading of the LUT is direction-blind: |v|
    // cannot tell rotation by θ about n̂ from rotation by θ about −n̂, so the
    // return ticks 6..11 read the SAME magnitudes as ticks 1..5 — 60·(t−5)
    // — and tick 11 (−1, 0, 0, 0) reads atan2(0, −1) = π → 360°, the folded
    // 2π edge. (The retired signed-argument register instead read the return
    // sweep at 720 − 60·(t−5); that asymmetry belongs to it.)
    let corrected_table = [0u16, 60, 120, 180, 240, 300, 60, 120, 180, 240, 300, 360];
    for (t, &expected) in corrected_table.iter().enumerate() {
        let read = quat_rotation_degrees(&RING_QUATERNION_LUT[t]);
        assert_eq!(
            read, expected,
            "tick {t}: corrected rotation register reads {read}"
        );
    }
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
