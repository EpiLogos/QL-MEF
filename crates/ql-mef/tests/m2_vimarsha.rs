use ql_mef::m2_vimarsha::*;
use serde::{Deserialize, Serialize};
use std::{cell::Cell, collections::BTreeSet, path::PathBuf, process::Command};

// Execute the byte-locked original reader with explicit M1/M3 dependencies.
// These shims inject the same external inputs into both readers; they do NOT
// claim to test or replace the historical M1 ratio or M3 84↔472 projection.
thread_local! {static SEED:Cell<Option<VimarshaSeed>>=const{Cell::new(None)};}
mod kernel {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct MathemeNodalConstraint {
        pub ql_position: u8,
        pub helix: String,
        pub m: u8,
        pub n: u8,
    }
    pub struct KernelTick {
        pub sub_tick: u8,
    }
    pub fn harmonic_ratio_fraction_for_sub_tick(t: u8) -> (u16, u16) {
        SEED.with(|v| {
            let s = v.get().unwrap();
            assert_eq!(s.tick12, t);
            (s.harmonic_ratio[0], s.harmonic_ratio[1])
        })
    }
}
mod codon_rotation_projection {
    use super::*;
    #[derive(Clone, Copy)]
    pub struct MathemeLensMode {
        pub lens: u8,
        pub mode: u8,
    }
    pub struct Surface {
        pub codon_id: u8,
        pub rotation: u8,
        pub rotation_degrees: u16,
    }
    pub fn codon_rotation_from_lens_mode(l: u8, m: u8) -> Option<Surface> {
        SEED.with(|v| {
            let s = v.get()?;
            assert_eq!((s.lens, s.musical_mode), (l, m));
            Some(Surface {
                codon_id: s.codon,
                rotation: s.rotation,
                rotation_degrees: s.rotation as u16 * 45,
            })
        })
    }
}
#[rustfmt::skip]
#[path="../../../fixtures/kernel/m2-reference-vimarsha.rs"]
mod retained;
fn close(a: f32, b: f32) {
    assert!(a.is_finite() && b.is_finite() && a > 0.0 && b > 0.0);
    assert!((a - b).abs() <= 2.0e-6 * a.max(b), "{a} != {b}");
}
#[test]
fn original_reader_native_c_and_rust_agree_with_the_same_supplied_dependencies() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let receipt = root.join("target/m2-receipt");
    std::fs::create_dir_all(&receipt).unwrap();
    let binary = receipt.join("vimarsha-native-probe");
    assert!(
        Command::new("cc")
            .current_dir(&root)
            .args([
                "-std=c11",
                "-O2",
                "-Wall",
                "-Wextra",
                "-Werror",
                "-pedantic",
                "-Ic/include",
                "scripts/m2-vimarsha-probe.c",
                "c/src/m2.c",
                "c/src/m_tree.c",
                "-lm",
                "-o"
            ])
            .arg(&binary)
            .status()
            .unwrap()
            .success()
    );
    let out = Command::new(binary).output().unwrap();
    assert!(out.status.success());
    let lines = String::from_utf8(out.stdout).unwrap();
    let mut seen = BTreeSet::new();
    let mut codons = BTreeSet::new();
    for line in lines.lines() {
        let a: Vec<_> = line.split('\t').collect();
        assert_eq!(a.len(), 32);
        assert_eq!(a[0], "vimarsha");
        let u = |i: usize| a[i].parse::<u8>().unwrap();
        let seed = VimarshaSeed {
            tick12: u(1),
            lens: u(2),
            musical_mode: u(3),
            codon: u(4),
            rotation: u(5),
            harmonic_ratio: [a[6].parse().unwrap(), a[7].parse().unwrap()],
        };
        assert!(seen.insert(a[1..8].join(":")));
        codons.insert(seed.codon);
        SEED.with(|s| s.set(Some(seed)));
        let old = retained::vimarsha_read_profile(
            kernel::KernelTick {
                sub_tick: seed.tick12,
            },
            codon_rotation_projection::MathemeLensMode {
                lens: seed.lens,
                mode: seed.musical_mode,
            },
        );
        let new = read_seed(seed).unwrap();
        for i in 0..8 {
            close(new.audio_octet_hz[i], a[8 + i].parse().unwrap());
            close(new.audio_octet_hz[i], old.audio_octet[i]);
        }
        for i in 0..4 {
            let n = new.nodal_quartet[i];
            let h = match n.helix {
                VimarshaHelix::Bimba => 0,
                VimarshaHelix::Pratibimba => 1,
            };
            assert_eq!(
                [n.ql_position, h, n.m, n.n],
                [u(16 + i * 4), u(17 + i * 4), u(18 + i * 4), u(19 + i * 4)]
            );
            let o = &old.nodal_quartet[i];
            assert_eq!((n.ql_position, n.m, n.n), (o.ql_position, o.m, o.n));
            assert_eq!(o.helix, if h == 0 { "bimba" } else { "pratibimba" });
        }
    }
    assert_eq!(seen.len(), 32256);
    assert_eq!(codons.len(), 64);
    std::fs::write(receipt.join("vimarsha-observed.tsv"), lines).unwrap();
    std::fs::write(receipt.join("vimarsha-parity.json"),"{\"schema\":\"ql.m2-vimarsha-parity/v1\",\"result\":\"passed\",\"cases\":32256,\"source_blob\":\"7b0bc34cee3388a8d22a615ad2c9facc28016e30\",\"standing\":\"reader-formula parity with supplied M1/M3 dependencies; not neighbour implementation parity\"}\n").unwrap();
}
#[test]
fn shared_m3_pose_validation_is_not_a_new_m2_pose_table() {
    for pose in ql_core::all_poses() {
        assert!(read_from_pose(11, 11, 6, [9, 8], pose).is_ok());
    }
    let s = VimarshaSeed {
        tick12: 0,
        lens: 0,
        musical_mode: 0,
        harmonic_ratio: [9, 8],
        codon: 0,
        rotation: 0,
    };
    for bad in [
        VimarshaSeed { tick12: 12, ..s },
        VimarshaSeed { lens: 12, ..s },
        VimarshaSeed {
            musical_mode: 7,
            ..s
        },
        VimarshaSeed {
            harmonic_ratio: [0, 1],
            ..s
        },
        VimarshaSeed {
            harmonic_ratio: [1, 0],
            ..s
        },
        VimarshaSeed { codon: 64, ..s },
        VimarshaSeed { rotation: 8, ..s },
    ] {
        assert!(read_seed(bad).is_err());
    }
    let bad = ql_core::RotationalPose::from_trusted(ql_core::Codon64::new(0), 255);
    assert!(read_from_pose(0, 0, 0, [9, 8], bad).is_err());
}
