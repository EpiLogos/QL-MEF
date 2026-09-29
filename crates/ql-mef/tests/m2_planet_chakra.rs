use ql_mef::m2;
use std::{collections::BTreeSet, path::Path, process::Command};

struct ProbeDirectory(std::path::PathBuf);
impl Drop for ProbeDirectory {
    fn drop(&mut self) {
        // The directory is owned exclusively by this native probe invocation;
        // remove its executable even when a real compiler/assertion fails.
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn actual_c_and_rust_source_routes_preserve_graph_assertions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let directory = root.join(format!("target/m2-planet-chakra-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let _cleanup = ProbeDirectory(directory.clone());
    let executable = directory.join("probe");
    let compiled = Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
        .current_dir(&root)
        .args([
            "-std=c11",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-Ic/include",
            "scripts/m2-planet-chakra-probe.c",
            "c/src/m2.c",
            "c/src/m_tree.c",
            "-lm",
            "-o",
        ])
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let observed = Command::new(&executable).output().unwrap();
    assert!(
        observed.status.success(),
        "{}",
        String::from_utf8_lossy(&observed.stderr)
    );
    let mut planets = BTreeSet::new();
    let mut assertions = BTreeSet::new();
    let mut decans = BTreeSet::new();
    let mut decan_assertions = BTreeSet::new();
    for line in String::from_utf8(observed.stdout).unwrap().lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let planet: usize = fields[1].parse().unwrap();
        if fields[0].starts_with("decan") {
            let route = m2::decan_planet_route(planet as f64 * 10.0 + 5.0).unwrap();
            if fields[0] == "decan" {
                assert!(decans.insert(planet));
                assert_eq!(fields[2], format!("{:016x}", route.decan_id.as_u64()));
                assert_eq!(fields[3], route.planet_index.unwrap_or(255).to_string());
                assert_eq!(
                    fields[4],
                    usize::from(!route.source_conflicts.is_empty()).to_string()
                );
                assert_eq!(fields[5], route.graph_candidates.len().to_string());
                assert_eq!(
                    fields[6],
                    route
                        .graph_candidates
                        .iter()
                        .map(|candidate| candidate.relations.len())
                        .sum::<usize>()
                        .to_string()
                );
                if planet == 11 {
                    // RULED_BY Saturn; RESONATES_WITH, SPANDA_TEMPORAL_RHYTHM and
                    // HARMONICALLY_RESONATES_WITH all reach the Moon.
                    assert_eq!(route.graph_candidates[0].planet_index, 6);
                    assert_eq!(route.source_conflicts.len(), 3);
                    assert!(
                        route
                            .source_conflicts
                            .iter()
                            .all(|c| c.planet_index == 1 && c.planet_coordinate == "#2-5-4")
                    );
                }
            } else {
                let assertion = route
                    .graph_candidates
                    .iter()
                    .flat_map(|candidate| &candidate.relations)
                    .find(|assertion| format!("{:016x}", assertion.id.as_u64()) == fields[2])
                    .unwrap();
                assert!(decan_assertions.insert((planet, assertion.id.as_u64())));
                assert_eq!(fields[3], assertion.relation_ref);
                assert_eq!(fields[4], assertion.record.to_string());
            }
            continue;
        }
        let route = m2::planet_chakra_route(planet).unwrap();
        match fields[0] {
            "absent" => {
                assert!(route.is_none());
                assert!(planets.insert(planet));
            }
            "route" => {
                let route = route.unwrap();
                assert!(planets.insert(planet));
                assert_eq!(fields[2], format!("{:016x}", route.planet_id.as_u64()));
                assert_eq!(fields[3], format!("{:016x}", route.chakra_id.as_u64()));
                assert_eq!(fields[4], route.chakra_index.to_string());
                assert_eq!(fields[5], route.relations.len().to_string());
                assert_eq!(route.chakra_coordinate, format!("#2-5-0/1-{}", 7 - planet));
            }
            "assertion" => {
                let route = route.unwrap();
                let index: usize = fields[2].parse().unwrap();
                let assertion = &route.relations[index];
                assert!(assertions.insert((planet, index)));
                assert_eq!(fields[3], format!("{:016x}", assertion.id.as_u64()));
                assert_eq!(fields[4], assertion.relation_ref);
                assert_eq!(fields[5], assertion.record.to_string());
            }
            other => panic!("unknown native observation {other}"),
        }
    }
    assert_eq!(planets, (0..10).collect());
    assert_eq!(
        assertions.len(),
        (0..7)
            .map(|i| m2::planet_chakra_route(i).unwrap().unwrap().relations.len())
            .sum::<usize>()
    );
    assert!(m2::planet_chakra_route(10).is_err());
    assert_eq!(decans, (0..36).collect());
    assert_eq!(
        decan_assertions.len(),
        (0..36)
            .map(|index| {
                m2::decan_planet_route(index as f64 * 10.0 + 5.0)
                    .unwrap()
                    .graph_candidates
                    .iter()
                    .map(|candidate| candidate.relations.len())
                    .sum::<usize>()
            })
            .sum::<usize>()
    );
    for longitude in [f64::NAN, f64::INFINITY, -1.0, 360.0] {
        assert!(m2::decan_planet_route(longitude).is_err());
    }
    std::fs::remove_file(&executable).unwrap();
    std::fs::remove_dir(&directory).unwrap();
}
