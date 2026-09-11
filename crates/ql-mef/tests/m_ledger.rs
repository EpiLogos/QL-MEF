use ql_mef::m_ledger::*;
use ql_mef::m_tree::native_m_registry;

fn ledger() -> MLedger {
    native_m_ledger().expect("accepted seed ledger")
}
fn codes(l: &MLedger) -> Vec<String> {
    l.validate(native_m_registry())
        .into_iter()
        .filter(|f| f.severity == "error")
        .map(|f| f.code)
        .collect()
}
fn rejects(l: &MLedger, code: &str) {
    assert!(
        codes(l).iter().any(|c| c == code),
        "expected {code}; got {:?}",
        codes(l)
    );
}
fn index(l: &MLedger) -> usize {
    l.rows.iter().position(|r| r.id == "ql.m-index:M1").unwrap()
}

#[test]
fn imports_existing_matrix_families_not_a_manual_deep_census() {
    let l = ledger();
    assert!(codes(&l).is_empty());
    assert_eq!(l.matrices.len(), 10);
    // K3 imported 185 source capabilities + 7 aggregate index rows. K4 added
    // one census row per M1/M2/M3 registry coordinate (43 + 597 + 996) through
    // the automated coverage census; the source imports themselves are
    // unchanged and every capability keeps its distinct identity.
    assert_eq!(l.rows.len(), 1828);
    assert_eq!(l.rows.iter().filter(|r| r.source.is_some()).count(), 185);
    assert_eq!(
        l.rows
            .iter()
            .filter(|r| r.id.starts_with("census:"))
            .count(),
        1636
    );
    // K4 census inventory: 90 discovered C bodies (including the explicitly
    // infrastructural arena machinery), 148 Rust bodies, and one live-graph
    // structural-index record per joined live coordinate (1635).
    assert_eq!(l.implementations.len(), 1873);
    // C-stratum coordinates still lacking a discovered computational body,
    // from the committed census receipts (fixtures/kernel/census/): M1 22,
    // M2 43 and M3 18 coordinates carry discovered C structure, including
    // aggregate coverage; M0/M4/M5 await the K10 extension.
    for (scope, count, lacking) in [
        ("M0", 108, 108),
        ("M1", 43, 21),
        ("M2", 597, 554),
        ("M3", 996, 978),
        ("M4", 100, 100),
        ("M5", 31, 31),
        ("M", 1876, 1793),
    ] {
        let c = l
            .coverage(native_m_registry(), scope, "c", "coordinate", "verified")
            .unwrap();
        assert_eq!(c.structural_coordinates, count);
        assert_eq!(c.coordinates_without_computational_binding.len(), lacking);
        assert_eq!(c.blocking_rows.len(), c.rows.len());
    }
}

#[test]
fn exact_aliases_and_compound_separators_use_registry_parentage() {
    let l = ledger();
    let r = native_m_registry();
    for reference in ["#0-4.0/1/2", "M0-4.0/1/2", "#0-4.0/1-2", "#4.5-0", "#4.5.0"] {
        let c = l
            .coverage(r, reference, "rust", "operational", "verified")
            .unwrap();
        assert_eq!(c.scope, r.resolve(reference).unwrap().source_ref);
        assert_eq!(
            c.structural_coordinates,
            r.resolve(reference).unwrap().subtree_count
        );
    }
    for reference in ["#0-4.0", "M6", " M1", "M1 ", "#", "#-0", "M1-999"] {
        assert!(
            l.coverage(r, reference, "c", "coordinate", "verified")
                .is_err()
        );
    }
    assert!(
        l.coverage(r, "M1", "unknown", "coordinate", "verified")
            .is_err()
    );
    assert!(l.coverage(r, "M1", "c", "complete", "verified").is_err());
}

#[test]
fn source_assertions_can_remain_unresolved_but_not_become_bindings() {
    let mut l = ledger();
    l.rows[0].coordinates.push("#0-4.0".into());
    assert!(
        l.validate(native_m_registry())
            .iter()
            .any(|f| f.code == "unresolved-source-coordinate")
    );
    assert!(codes(&l).is_empty());
    l.implementations[0].coordinates = vec!["#0-4.0".into()];
    rejects(&l, "implementation-coordinate");
}

#[test]
fn no_complete_flag_or_missing_axes() {
    let mut l = ledger();
    l.assessments
        .get_mut("unassessed")
        .unwrap()
        .readiness
        .get_mut("rust")
        .unwrap()
        .status = "COMPLETE".into();
    rejects(&l, "unsupported-completion-claim");
    let mut l = ledger();
    l.assessments
        .get_mut("unassessed")
        .unwrap()
        .parity
        .remove("experiential");
    rejects(&l, "assessment-axes");
    let mut value: serde_json::Value = serde_json::from_str(NATIVE_M_LEDGER).unwrap();
    value["complete"] = true.into();
    assert!(MLedger::from_json(&value.to_string(), native_m_registry()).is_err());
}

#[test]
fn readiness_without_evidence_is_rejected() {
    let mut l = ledger();
    let claim = l
        .assessments
        .get_mut("unassessed")
        .unwrap()
        .readiness
        .get_mut("rust")
        .unwrap();
    claim.status = "verified".into();
    claim.warrant = "tested".into();
    rejects(&l, "unsupported-readiness");
    rejects(&l, "readiness-without-binding");
}

#[test]
fn registry_index_evidence_is_not_a_computational_test() {
    let mut l = ledger();
    let claim = l
        .assessments
        .get_mut("k2-index")
        .unwrap()
        .readiness
        .get_mut("rust")
        .unwrap();
    claim.status = "verified".into();
    claim.warrant = "tested".into();
    rejects(&l, "unsupported-readiness");
    rejects(&l, "readiness-without-binding");
}

#[test]
fn coordinate_test_cannot_establish_operational_or_experiential_readiness() {
    let mut l = ledger();
    l.evidence[0].kind = "test-receipt".into();
    l.evidence[0].result = "passed".into();
    for i in &mut l.implementations {
        if i.stratum == "rust" {
            i.kind = "computational".into();
        }
    }
    let claim = l
        .assessments
        .get_mut("k2-index")
        .unwrap()
        .readiness
        .get_mut("rust")
        .unwrap();
    claim.status = "verified".into();
    claim.warrant = "tested".into();
    rejects(&l, "unsupported-readiness");
    l.evidence[0].axes.push("operational".into());
    assert!(codes(&l).is_empty(), "{:?}", codes(&l));
    l.evidence[0].registry_revision = "0".repeat(64);
    rejects(&l, "evidence-contract");
}

#[test]
fn source_declaration_cannot_prove_implementation() {
    let mut l = ledger();
    for i in &mut l.implementations {
        if i.stratum == "rust" {
            i.kind = "computational".into();
        }
    }
    let claim = l
        .assessments
        .get_mut("k2-index")
        .unwrap()
        .readiness
        .get_mut("rust")
        .unwrap();
    claim.status = "implemented".into();
    claim.warrant = "implemented".into();
    rejects(&l, "unsupported-implementation-claim");
}

#[test]
fn orphan_c_and_rust_constructs_are_reported_but_explicit_infrastructure_is_valid() {
    for stratum in ["c", "rust"] {
        let mut l = ledger();
        let mut implementation = l.implementations[0].clone();
        implementation.id = format!("{stratum}:orphan");
        implementation.stratum = stratum.into();
        l.implementations.push(implementation);
        rejects(&l, "orphan-implementation");
        let implementation = l.implementations.last_mut().unwrap();
        implementation.disposition = "infrastructural".into();
        implementation.coordinates.clear();
        assert!(codes(&l).is_empty());
        l.implementations.last_mut().unwrap().rationale.clear();
        rejects(&l, "orphan-implementation");
    }
}

#[test]
fn rust_claim_must_resolve_same_coordinate_as_row() {
    let mut l = ledger();
    let id = l.rows[index(&l)].bindings[1].clone();
    l.implementations
        .iter_mut()
        .find(|i| i.id == id)
        .unwrap()
        .coordinates = vec!["M2".into()];
    rejects(&l, "binding-coordinate-mismatch");
}

#[test]
fn dangling_bindings_and_false_dispositions_are_rejected() {
    let mut l = ledger();
    l.rows[0].bindings.push("missing:c".into());
    rejects(&l, "missing-binding");
    let mut l = ledger();
    l.rows[0].dispositions.insert("cpp".into(), "bound".into());
    rejects(&l, "implementation-disposition");
}

#[test]
fn structural_parent_children_and_relation_disagreements_are_rejected() {
    let mut l = ledger();
    let r = native_m_registry();
    let node = r.resolve("M1").unwrap();
    let assertion = StructureAssertion {
        coordinate: "M1".into(),
        parent: Some("M".into()),
        children: r.children(node.id).map(|n| n.source_ref.clone()).collect(),
        relations: r
            .relations_for(node.id)
            .map(|r| r.relation_ref.clone())
            .collect(),
    };
    let at = l
        .implementations
        .iter()
        .position(|i| i.id == "c:ql.m-index:M1")
        .unwrap();
    l.implementations[at].structure.push(assertion);
    assert!(codes(&l).is_empty());
    l.implementations[at].structure[0].parent = Some("M2".into());
    rejects(&l, "structural-disagreement");
    l.implementations[at].structure[0].parent = Some("M".into());
    l.implementations[at].structure[0].children.clear();
    rejects(&l, "structural-disagreement");
}

#[test]
fn duplicate_rows_and_stale_registry_are_invalid() {
    let mut l = ledger();
    l.rows.push(l.rows[0].clone());
    rejects(&l, "duplicate-or-empty-id");
    let mut l = ledger();
    l.registry.revision = "f".repeat(64);
    rejects(&l, "ledger-contract");
}

#[test]
fn evidence_is_subject_stratum_and_axis_scoped() {
    let mut l = ledger();
    l.rows[0].assessment = "k2-index".into();
    rejects(&l, "unsupported-readiness");
    let mut l = ledger();
    l.evidence[0].strata.clear();
    rejects(&l, "unsupported-readiness");
    let mut l = ledger();
    l.evidence[0].subjects = vec!["*".into()];
    rejects(&l, "evidence-contract");
}

#[test]
fn opposite_parity_claims_cannot_contradict_each_other() {
    let mut l = ledger();
    let p = ParityClaim {
        from_peer: "c".into(),
        to_peer: "rust".into(),
        status: "unassessed".into(),
        warrant: "unassessed".into(),
        evidence: vec![],
    };
    l.assessments
        .get_mut("unassessed")
        .unwrap()
        .parity
        .get_mut("coordinate")
        .unwrap()
        .extend([
            p.clone(),
            ParityClaim {
                from_peer: p.to_peer,
                to_peer: p.from_peer,
                ..p
            },
        ]);
    rejects(&l, "parity-contract");
}

#[test]
fn parity_requires_both_peers_not_an_unrelated_receipt() {
    let mut l = ledger();
    l.evidence[0].kind = "test-receipt".into();
    l.evidence[0].result = "passed".into();
    l.assessments
        .get_mut("k2-index")
        .unwrap()
        .parity
        .get_mut("coordinate")
        .unwrap()
        .push(ParityClaim {
            from_peer: "rust".into(),
            to_peer: "cpp".into(),
            status: "equivalent".into(),
            warrant: "tested".into(),
            evidence: vec!["k2-index-declaration".into()],
        });
    rejects(&l, "unsupported-parity");
    l.evidence[0].strata.push("cpp".into());
    assert!(codes(&l).is_empty(), "{:?}", codes(&l));
}

#[test]
fn discrepancies_allow_all_twelve_directed_peer_pairs() {
    for from in ["bimba", "c", "rust", "cpp"] {
        for to in ["bimba", "c", "rust", "cpp"] {
            if from != to {
                let mut l = ledger();
                l.discrepancies[0].from_peer = from.into();
                l.discrepancies[0].to_peer = to.into();
                assert!(codes(&l).is_empty(), "{from} -> {to}: {:?}", codes(&l));
            }
        }
    }
}

#[test]
fn decision_and_history_cannot_be_skipped() {
    let mut l = ledger();
    l.discrepancies[0].state = "accepted".into();
    rejects(&l, "discrepancy-lifecycle");
    rejects(&l, "missing-resolution-decision");
    l.discrepancies[0].history.push(Transition {
        state: "accepted".into(),
        reference: "review".into(),
    });
    rejects(&l, "discrepancy-lifecycle");
}

fn accepted_reverse_correction(l: &mut MLedger) {
    let row_id = l.rows[index(l)].id.clone();
    let d = &mut l.discrepancies[0];
    d.from_peer = "cpp".into();
    d.to_peer = "bimba".into();
    d.subjects = vec![row_id];
    d.state = "accepted".into();
    d.proposal = Some(Resolution {
        target_peer: "bimba".into(),
        change: "Correct a graph structural assertion using returned embodiment evidence".into(),
        evidence: vec![],
    });
    d.history.extend([
        Transition {
            state: "proposed".into(),
            reference: "proposal".into(),
        },
        Transition {
            state: "accepted".into(),
            reference: "decision".into(),
        },
    ]);
    d.decision = Some(Decision {
        authority: Authority {
            peer: "bimba".into(),
            reference: "authorial-review".into(),
            reason: "Returned evidence accepted by the owning authority".into(),
        },
        evidence: vec!["review".into()],
    });
    let mut evidence = l.evidence[0].clone();
    evidence.id = "review".into();
    evidence.kind = "review".into();
    evidence.result = "accepted".into();
    evidence.subjects = vec![d.id.clone()];
    l.evidence.push(evidence);
}

#[test]
fn reverse_correction_has_a_real_proposal_decision_application_lifecycle() {
    let mut l = ledger();
    accepted_reverse_correction(&mut l);
    assert!(codes(&l).is_empty());
    l.discrepancies[0].state = "applied".into();
    l.discrepancies[0].history.push(Transition {
        state: "applied".into(),
        reference: "applied-change".into(),
    });
    rejects(&l, "unproved-resolution-application");
    let mut evidence = l.evidence.last().unwrap().clone();
    evidence.id = "application".into();
    evidence.kind = "observation".into();
    evidence.result = "passed".into();
    evidence.strata = vec!["source".into()];
    l.evidence.push(evidence);
    l.discrepancies[0].proposal.as_mut().unwrap().evidence = vec!["application".into()];
    assert!(codes(&l).is_empty());
}

#[test]
fn structural_promotion_mandates_c_parity_and_binding() {
    let mut l = ledger();
    accepted_reverse_correction(&mut l);
    l.discrepancies[0].promotion = Some("structural-canon".into());
    rejects(&l, "promotion-without-c-parity");
}

#[test]
fn research_relations_do_not_need_canon_promotion() {
    let mut l = ledger();
    l.rows[0].relations.push(RelationClaim {
        relation_ref: "research:unpromoted".into(),
        standing: "research".into(),
        evidence: vec![],
    });
    assert!(codes(&l).is_empty());
    l.rows[0].relations[0].standing = "structural-canon".into();
    rejects(&l, "promotion-without-c-parity");
}

#[test]
fn vertical_dependencies_are_included_and_cycles_rejected() {
    let mut l = ledger();
    let external = l.rows.iter().find(|r| r.scope == "M5").unwrap().id.clone();
    let at = index(&l);
    l.rows[at].dependencies.push(external.clone());
    let report = l
        .coverage(native_m_registry(), "M1", "rust", "operational", "verified")
        .unwrap();
    assert!(report.rows.contains(&external));
    let id = l.rows[at].id.clone();
    l.rows
        .iter_mut()
        .find(|r| r.id == external)
        .unwrap()
        .dependencies
        .push(id);
    rejects(&l, "dependency-cycle-or-missing");
}

#[test]
fn source_coverage_gaps_and_known_backward_parent_discrepancy_remain_visible() {
    let l = ledger();
    let c = l
        .coverage(native_m_registry(), "M2", "rust", "operational", "verified")
        .unwrap();
    assert!(!c.source_without_implementation_disposition.is_empty());
    assert!(!c.coordinates_without_capability_rows.is_empty());
    assert!(
        c.findings
            .iter()
            .any(|f| f.code == "open-discrepancy" && f.detail.contains("#2-4.5"))
    );
    assert!(c.findings.iter().all(|f| f.severity != "error"));
}

#[test]
fn joined_coordinate_view_retains_source_parentage_relations_and_profiles() {
    let l = ledger();
    let view = l
        .coordinate_view(native_m_registry(), "M0-4.0/1/2")
        .unwrap();
    assert_eq!(view["coordinate"]["source_ref"], "#0-4.0/1/2");
    assert_eq!(view["coordinate"]["local_segment"], "0/1/2");
    assert!(!view["source_records"].as_array().unwrap().is_empty());
    let view = l.coordinate_view(native_m_registry(), "M1").unwrap();
    assert!(
        view["rows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["row"]["id"] == "ql.m-index:M1")
    );
}

/// A valid readiness assessment and C/Rust comparison do not imply that the
/// requested stratum participates in that comparison. These are synthetic
/// evidence declarations for validation tests, not new execution receipts.
fn coverage_parity_fixture(stratum: &str) -> (MLedger, String) {
    let mut l = ledger();
    let at = index(&l);
    let row_id = l.rows[at].id.clone();
    let mut assessment = l.assessments["k2-index"].clone();
    let mut evidence = l.evidence[0].clone();
    evidence.id = "coverage-parity-fixture".into();
    evidence.kind = "observation".into();
    evidence.result = "passed".into();
    evidence.subjects = vec![row_id.clone()];
    evidence.strata = vec![stratum.into(), "c".into(), "rust".into()];
    evidence.axes = AXES.iter().map(|axis| (*axis).into()).collect();
    *assessment.readiness.get_mut(stratum).unwrap() = Claim {
        status: "verified".into(),
        warrant: "tested".into(),
        evidence: vec![evidence.id.clone()],
    };
    let comparisons = assessment.parity.get_mut("coordinate").unwrap();
    comparisons.push(ParityClaim {
        from_peer: "c".into(),
        to_peer: "rust".into(),
        status: "equivalent".into(),
        warrant: "tested".into(),
        evidence: vec![evidence.id.clone()],
    });
    // Neo4j readiness also requires a computational binding. This synthetic
    // fixture makes that prerequisite valid so the test isolates peer selection.
    if stratum == "neo4j" {
        let mut binding = l
            .implementations
            .iter()
            .find(|i| i.stratum == "c" && l.rows[at].bindings.contains(&i.id))
            .unwrap()
            .clone();
        binding.id = "coverage-neo4j-fixture".into();
        binding.stratum = "neo4j".into();
        binding.kind = "computational".into();
        l.rows[at].bindings.push(binding.id.clone());
        l.implementations.push(binding);
    }
    l.rows[at].assessment = "coverage-parity-fixture".into();
    l.assessments
        .insert("coverage-parity-fixture".into(), assessment);
    l.evidence.push(evidence);
    assert!(codes(&l).is_empty(), "{:?}", codes(&l));
    (l, row_id)
}

#[test]
fn source_coverage_cannot_borrow_c_rust_parity() {
    let (l, row_id) = coverage_parity_fixture("source");
    let report = l
        .coverage(
            native_m_registry(),
            "M1",
            "source",
            "coordinate",
            "verified",
        )
        .unwrap();
    assert!(report.blocking_rows.iter().any(|f| f.subject == row_id));
}

#[test]
fn source_coverage_accepts_scoped_bimba_parity_in_either_direction() {
    for (from, to) in [("bimba", "c"), ("c", "bimba")] {
        let (mut l, row_id) = coverage_parity_fixture("source");
        let assessment = l.assessments.get_mut("coverage-parity-fixture").unwrap();
        let p = &mut assessment.parity.get_mut("coordinate").unwrap()[0];
        p.from_peer = from.into();
        p.to_peer = to.into();
        assert!(codes(&l).is_empty(), "{:?}", codes(&l));
        let report = l
            .coverage(
                native_m_registry(),
                "M1",
                "source",
                "coordinate",
                "verified",
            )
            .unwrap();
        assert!(!report.blocking_rows.iter().any(|f| f.subject == row_id));
    }
}

#[test]
fn later_strata_do_not_gain_parity_from_an_unrelated_native_comparison() {
    for stratum in ["neo4j", "application", "instrument"] {
        let (l, row_id) = coverage_parity_fixture(stratum);
        let report = l
            .coverage(native_m_registry(), "M1", stratum, "coordinate", "verified")
            .unwrap();
        assert!(report.blocking_rows.iter().any(|f| f.subject == row_id));
    }
}
