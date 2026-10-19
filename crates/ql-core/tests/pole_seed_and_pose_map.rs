//! The two declared test variables of the active-pose reader: the quaternion
//! seed (FR 2.Q.9 Sum/Difference/Mod, default; FR 2.Q.17 four-charge) and the
//! pose map (raw bin, default; lawful slot). Defaults reproduce today's
//! behaviour exactly; the measured divergence is pinned.

use std::collections::HashSet;

use ql_core::{
    Codon64, CodonClass, MatrixFamily, PoseMap, Quat, QuatSeed, det_overlay, element_quaternion,
    matrix_axis_quaternion, quat_active_pose, quat_active_state, quat_from_codon,
    quat_from_four_charge, quat_from_ring_pos,
};

/// ring tick (12) x element position (5) x matrix axis (3): the 180 environments
/// the DET overlay composes.
fn environments() -> Vec<Quat> {
    let mut out = Vec::new();
    for tick in 0u32..12 {
        for element in 0u8..5 {
            for family in MatrixFamily::ALL {
                out.push(
                    quat_from_ring_pos(tick)
                        .mul(element_quaternion(element).mul(matrix_axis_quaternion(family))),
                );
            }
        }
    }
    assert_eq!(out.len(), 180);
    out
}

#[test]
fn defaults_reproduce_the_current_reader_exactly() {
    assert_eq!(QuatSeed::default(), QuatSeed::SumDifferenceMod);
    assert_eq!(PoseMap::default(), PoseMap::RawBin);
    for env in environments() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let reading = quat_active_pose(env, codon, QuatSeed::default(), PoseMap::default());
            assert_eq!(reading.slot, quat_active_state(env, codon));
            assert_eq!(reading.slot, reading.bin);
            assert!(!reading.collapsed);
            assert_eq!(reading.seed, QuatSeed::SumDifferenceMod);
            assert_eq!(reading.pose_map, PoseMap::RawBin);
        }
    }
    // The overlay (which uses the default reader) agrees.
    let overlay = det_overlay(3, 2, MatrixFamily::SameQuality, u64::MAX);
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        assert_eq!(
            overlay.codon_states[address as usize],
            Some(quat_active_state(overlay.composed_q, codon))
        );
    }
}

#[test]
fn seeds_are_named_by_their_fr() {
    assert_eq!(QuatSeed::SumDifferenceMod.fr(), "FR 2.Q.9");
    assert_eq!(QuatSeed::FourCharge.fr(), "FR 2.Q.17");
}

#[test]
fn sum_difference_mod_seed_has_no_y_and_46_distinct_values() {
    let mut distinct = HashSet::new();
    for address in 0u8..64 {
        let q = quat_from_codon(Codon64::new(address));
        assert_eq!(q.y, 0.0);
        distinct.insert((q.w as i32, q.x as i32, q.z as i32));
    }
    assert_eq!(distinct.len(), 46);
}

#[test]
fn four_charge_seed_is_one_to_one_over_the_64_codons() {
    let mut distinct = HashSet::new();
    for address in 0u8..64 {
        let q = quat_from_four_charge(Codon64::new(address));
        distinct.insert((q.w as i32, q.x as i32, q.y as i32, q.z as i32));
    }
    assert_eq!(distinct.len(), 64);
}

#[test]
fn the_two_seeds_read_different_bins_as_measured() {
    let envs = environments();
    let mut same = 0usize;
    let mut total = 0usize;
    let mut non_dual_on_bin7 = [0usize; 2];
    for env in &envs {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let a = quat_active_pose(*env, codon, QuatSeed::SumDifferenceMod, PoseMap::RawBin);
            let b = quat_active_pose(*env, codon, QuatSeed::FourCharge, PoseMap::RawBin);
            total += 1;
            if a.bin == b.bin {
                same += 1;
            }
            if codon.classify() != CodonClass::Dual {
                non_dual_on_bin7[0] += usize::from(a.bin == 7);
                non_dual_on_bin7[1] += usize::from(b.bin == 7);
            }
        }
    }
    assert_eq!(total, 11_520);
    // 43.8% of states read the same bin under both seeds (f32 kernel reader,
    // the 12 x 5 x 3 DET-overlay environments; an offline f64 replication
    // with the same environments gives 44.4%, so bin boundaries are sensitive).
    let agreement = same as f64 / total as f64;
    let _ = agreement;
    assert_eq!(same, 5050, "agreeing states of 11,520");
    // Non-dual codons on bin 7 (no lawful 8th pose), of 7,200 states: the
    // Sum/Difference/Mod seed lands there far more often than the four-charge seed.
    // (200 under macOS atan2f, which read 15 folded-edge states as bin 7; the
    // exact reader folds them to 0 on every platform, as glibc did.)
    assert_eq!(non_dual_on_bin7, [185, 15]);
}

#[test]
fn lawful_slot_keeps_every_reading_inside_the_codons_poses() {
    let envs = environments();
    let mut collapsed_total = 0;
    for env in &envs {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            for seed in [QuatSeed::SumDifferenceMod, QuatSeed::FourCharge] {
                let raw = quat_active_pose(*env, codon, seed, PoseMap::RawBin);
                let lawful = quat_active_pose(*env, codon, seed, PoseMap::LawfulSlot);
                assert_eq!(raw.bin, lawful.bin, "same bin, different map");
                assert!(lawful.slot < codon.rotational_state_count());
                assert_eq!(lawful.seed, seed);
                assert_eq!(lawful.pose_map, PoseMap::LawfulSlot);
                if lawful.collapsed {
                    collapsed_total += 1;
                    assert_eq!(lawful.bin, 7);
                    assert_eq!(codon.rotational_state_count(), 7);
                    assert_eq!(lawful.slot, 0, "bin 7 reads as its mirror, bin 0");
                } else {
                    assert_eq!(lawful.slot, lawful.bin);
                }
                // Dual codons never collapse.
                if codon.classify() == CodonClass::Dual {
                    assert!(!lawful.collapsed);
                }
            }
        }
    }
    // The same non-dual bin-7 counts, summed over both seeds.
    assert_eq!(collapsed_total, 185 + 15);
}
