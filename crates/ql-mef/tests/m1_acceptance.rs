use ql_mef::m_ledger::native_m_ledger;
use ql_mef::m_tree::native_m_registry;
use std::collections::BTreeSet;
use std::process::Command;

#[test]
fn native_bindings_and_readiness_remain_qualified_by_their_actual_source() {
    let ledger = native_m_ledger().unwrap();
    let registry = native_m_registry();
    let historical_registry = registry.manifest().registry_revision
        == "2264f5686abd1eb3192ecabd74457ca87086be8cea5f1f29de8b48d02151ef77";
    assert!(
        ledger
            .validate(registry)
            .iter()
            .all(|f| f.severity != "error")
    );
    for (peer, axis) in [("c", "coordinate"), ("rust", "operational")] {
        let coverage = ledger
            .coverage(registry, "M1", peer, axis, "verified")
            .unwrap();
        assert_eq!(coverage.structural_coordinates, 43);
        assert!(
            coverage
                .coordinates_without_computational_binding
                .is_empty()
        );
        // Whole source/instrument ambitions still block blanket experiential
        // completion. Those limits do not erase the retained native bindings.
        assert!(!coverage.blocking_rows.is_empty());
    }
    let expected: BTreeSet<_> = registry
        .manifest()
        .nodes
        .iter()
        .filter(|n| n.root_position == Some(1))
        .map(|n| format!("census:{}", n.source_ref))
        .collect();
    let actual: BTreeSet<_> = ledger
        .rows
        .iter()
        .filter(|r| expected.contains(&r.id))
        .map(|r| {
            let p = &ledger.assessments[&r.assessment];
            for peer in ["c", "rust"] {
                let claim = &p.readiness[peer];
                assert_eq!(
                    claim.status,
                    if historical_registry {
                        "verified"
                    } else {
                        "unassessed"
                    }
                );
                if !historical_registry {
                    assert_eq!(claim.warrant, "unassessed");
                    assert!(claim.evidence.is_empty());
                    assert!(
                        r.bindings
                            .iter()
                            .any(|id| ledger.implementations.iter().any(|implementation| {
                                implementation.id == *id
                                    && implementation.stratum == peer
                                    && implementation.kind == "computational"
                            }))
                    );
                }
            }
            if !historical_registry {
                assert!(p.parity.values().all(Vec::is_empty));
            }
            assert!(p.parity["experiential"].is_empty());
            r.id.clone()
        })
        .collect();
    assert_eq!(actual, expected);
    // The checker preserves historical acceptance separately and checks exact
    // current tree/source/native replay locks. A produced coverage report or
    // retained computational binding does not promote a new source's readiness.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let check = Command::new("python3")
        .arg("scripts/check-m1-acceptance.py")
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
}
