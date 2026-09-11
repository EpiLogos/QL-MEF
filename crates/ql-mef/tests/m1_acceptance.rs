use ql_mef::m_ledger::native_m_ledger;
use ql_mef::m_tree::native_m_registry;
use std::collections::BTreeSet;
use std::process::Command;

#[test]
fn reviewed_coordinates_are_executable_and_wider_provider_gaps_remain_explicit() {
    let ledger = native_m_ledger().unwrap();
    let registry = native_m_registry();
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
        // completion. That must not erase the verified coordinate operations.
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
            assert_eq!(p.readiness["c"].status, "verified");
            assert_eq!(p.readiness["rust"].status, "verified");
            assert!(p.parity["experiential"].is_empty());
            r.id.clone()
        })
        .collect();
    assert_eq!(actual, expected);
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
