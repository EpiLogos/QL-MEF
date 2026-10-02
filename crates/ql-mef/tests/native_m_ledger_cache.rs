//! Actual compiled native source equality, racing first reads and owned callers.
//! No fake ledger, runtime provider, readiness promotion or numerical shortcut.
use ql_mef::m_ledger::{MLedger, NATIVE_M_LEDGER, native_m_ledger};
use ql_mef::m_tree::native_m_registry;
use sha2::{Digest, Sha256};
use std::sync::{Arc, Barrier};
use std::{hint::black_box, path::Path, time::Instant};

#[test]
fn native_immutable_cache_preserves_full_source_concurrent_reads_and_mutated_callers() {
    // This direct parse does not initialize native_m_ledger's cache. All eight
    // threads are the first callers in this dedicated integration-test process.
    let direct_start = Instant::now();
    let original = MLedger::from_json(NATIVE_M_LEDGER, native_m_registry()).unwrap();
    let first_direct_ms = direct_start.elapsed().as_secs_f64() * 1000.0;
    let expected = serde_json::to_vec(&original).unwrap();
    let barrier = Arc::new(Barrier::new(8));
    let racing_start = Instant::now();
    let results = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                let barrier = Arc::clone(&barrier);
                scope.spawn(move || {
                    barrier.wait();
                    native_m_ledger().expect("The actual compiled native ledger remains admitted")
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    let racing_first_reads_ms = racing_start.elapsed().as_secs_f64() * 1000.0;
    for result in &results {
        assert_eq!(
            serde_json::to_vec(result).unwrap(),
            expected,
            "Cache must preserve every source, relation, readiness, evidence and decision field"
        );
    }
    // Exercise actual nested owned storage rather than just distinct outer
    // addresses. A caller cannot poison the next native operation's source.
    let mut changed = results.into_iter().next().unwrap();
    changed.rows[0]
        .invariants
        .push("Caller-owned change; never a canonical ledger decision".into());
    changed.evidence[0].subjects.clear();
    let claim = changed
        .assessments
        .get_mut("unassessed")
        .unwrap()
        .readiness
        .get_mut("rust")
        .unwrap();
    claim.status = "verified".into();
    claim.warrant = "tested".into();
    claim.evidence.clear();
    let findings = changed.validate(native_m_registry());
    assert!(
        findings
            .iter()
            .any(|finding| finding.severity == "error" && finding.code == "unsupported-readiness")
    );
    assert!(
        findings
            .iter()
            .any(|finding| finding.severity == "error" && finding.code == "evidence-contract")
    );
    let errors: Vec<_> = findings
        .into_iter()
        .filter(|finding| finding.severity == "error")
        .collect();
    let expected_refusal = serde_json::to_string(&errors).unwrap();
    let changed_json = serde_json::to_string(&changed).unwrap();
    assert_eq!(
        MLedger::from_json(&changed_json, native_m_registry()).unwrap_err(),
        expected_refusal,
        "Arbitrary caller input must still perform actual structural/readiness refusal checks"
    );
    assert_eq!(
        serde_json::to_vec(&native_m_ledger().unwrap()).unwrap(),
        expected,
        "Mutating a caller's rows, evidence or nested readiness must not change the cached source"
    );
    // This query's full output includes global validation findings and vertical
    // blockers. Caching admission cannot turn a retained binding into readiness.
    let fresh = original
        .coverage(native_m_registry(), "M1", "rust", "operational", "verified")
        .unwrap();
    let cached = native_m_ledger()
        .unwrap()
        .coverage(native_m_registry(), "M1", "rust", "operational", "verified")
        .unwrap();
    assert_eq!(
        serde_json::to_vec(&fresh).unwrap(),
        serde_json::to_vec(&cached).unwrap()
    );
    assert_eq!(
        MLedger::from_json("{", native_m_registry()).unwrap_err(),
        MLedger::from_json("{", native_m_registry()).unwrap_err()
    );

    // Time actual admitted native sources and the actual warmed owned accessor.
    // Equality checks are outside each timing window. No performance threshold
    // substitutes for correctness or the unchanged native coupling regression.
    let mut timings = Vec::new();
    for _ in 0..3 {
        let start = Instant::now();
        let uncached = MLedger::from_json(black_box(NATIVE_M_LEDGER), native_m_registry()).unwrap();
        black_box(&uncached);
        let parse_ms = start.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(serde_json::to_vec(&uncached).unwrap(), expected);
        drop(uncached);
        let start = Instant::now();
        let owned = native_m_ledger().unwrap();
        black_box(&owned);
        let clone_ms = start.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(serde_json::to_vec(&owned).unwrap(), expected);
        timings.push(serde_json::json!({"uncached_native_parse_validation_ms":parse_ms,"cached_native_owned_clone_ms":clone_ms}));
    }
    let receipt = serde_json::json!({"schema":"ql.native-ledger-cache-test/v1",
        "standing":"Actual compiled native admission, full equality, concurrent first reads and owned-caller isolation; no numerical readiness or installed experience promotion",
        "ledger_revision":original.ledger_revision,"registry_revision":original.registry.revision,
        "compiled_ledger_sha256":format!("{:x}",Sha256::digest(NATIVE_M_LEDGER.as_bytes())),
        "canonical_result_sha256":format!("{:x}",Sha256::digest(&expected)),
        "first_uncached_read_ms":first_direct_ms,"concurrent_first_read_count":8,
        "concurrent_first_reads_ms":racing_first_reads_ms,"timings":timings,
        "invalid_candidate_refused":true,"caller_mutation_isolated":true,
        "not_proved":["GPU/audio performance","Current numerical readiness promotion","Installed whole-experience acceptance"]});
    if let Ok(path) = std::env::var("EPI_LEDGER_CACHE_RECEIPT") {
        assert!(Path::new(&path).is_absolute());
        std::fs::write(path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
    }
    println!("{receipt}");
}
