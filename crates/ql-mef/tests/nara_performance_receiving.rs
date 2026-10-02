//! Actual native source operators and the current A/B/K/P prepared binding.
//! Controlled angular inputs exercise arithmetic, never qualify astronomy.
#[path = "support/retained_performance.rs"]
mod retained_performance;
use ql_mef::m_tree::{MRegistry, native_current_m_registry};
use ql_mef::nara::{
    EventBasisRefs, SourceRevision, current, domain::ProtectedRef, intake::IdentityProfile,
    replay::NaraOccasion,
};
use ql_mef::nara_performance_receiving::*;
use ql_mef::performance_audio::{PreparedPerformanceBinding, prepare_native_performance};
use ql_mef::physical_body::{PhysicalProvenance, PhysicalStanding};
use serde_json::{Value, json};

fn reference(s: &str) -> Reference {
    Reference {
        reference: s.into(),
        revision: "1".into(),
    }
}
fn prepared() -> PreparedPerformanceBinding {
    prepare_native_performance(retained_performance::preparation()).unwrap()
}
fn identity(p: &PreparedPerformanceBinding, degrees: [f64; 10]) -> Value {
    let profile: IdentityProfile = serde_json::from_value(json!({
        "schema":"ql.nara-identity-profile/v1", "person_ref":p.physical_body().subject_ref(),
        "nara_ref":"reference:receiving/nara", "name":"Controlled native receiving arithmetic",
        "encoding_policy":ql_mef::nara::identity_encoding::EncodingPolicy::default(), "composition_policy":"draft-core-birthdate-decanic-40-60-v1",
        "birth":{"date":"1990-06-15","time":null,"precision":"unknown","uncertainty_minutes":null,"fold":null,"place":null},
        "jungian":null,"gene_keys":null,"human_design":null,"quintessence":null
    })).unwrap();
    // Partial native input receipt explicitly has no chart/provider verdict.
    let natal = json!({"schema":"ql.nara-natal/v1", "request":profile.natal_request().unwrap(),
        "status":"partial", "reason":"controlled-angular-input; astronomical-provider-unqualified", "chart":null,
        "sky":{"schema":"ql.sky-snapshot/v1","snapshot_ref":"reference:controlled-natal-input",
            "bodies":["Sun","Moon","Mercury","Venus","Mars","Jupiter","Saturn","Uranus","Neptune","Pluto"].iter().enumerate().map(|(i,name)|
                json!({"native_planet_id":i,"body":name,"longitude_degrees":degrees[i]})).collect::<Vec<_>>()}});
    profile.inspect(Some(&natal)).unwrap()
}
fn occasion(p: &PreparedPerformanceBinding, i: &Value) -> NaraOccasion {
    NaraOccasion {
        occasion_ref: "reference:receiving/original-occasion".into(),
        subject_id: p.physical_body().subject_ref().into(),
        event: EventBasisRefs::from_basis(p.native_basis()).unwrap(),
        personal_reception_generation: 1,
        identity_revision: i["input_revision"].as_str().unwrap().into(),
        day_ref: "central:day:2026-10-02".into(),
        now_ref: "central:now:receiving-native-test".into(),
        occurrence_at_unix_ms: 100,
        receipt_at_unix_ms: 900,
        protected_state_ref: ProtectedRef {
            ref_id: "reference:receiving/protected-state".into(),
            revision: "1".into(),
            owner_ref: p.physical_body().subject_ref().into(),
        },
        activity_refs: vec![],
        oracle_packet_refs: vec![],
        transformation_phase_refs: vec![],
        context_reading_refs: vec![],
        integration_return_refs: vec![],
        expression_refs: vec![],
        source_revisions: vec![SourceRevision {
            source_ref: "ql:m-registry".into(),
            revision: native_current_m_registry()
                .manifest()
                .source_revision
                .clone(),
            standing_ref: "source".into(),
        }],
    }
}
fn context(o: &NaraOccasion) -> ReceivingContext {
    ReceivingContext {
        kind: ContextKind::Personal,
        context: reference("reference:receiving/personal-context"),
        receiver: reference("reference:receiving/receiver"),
        protected_state: Some(reference(&o.protected_state_ref.ref_id)),
        consent: None,
        original_occasion: Some(reference(&o.occasion_ref)),
        private: true,
    }
}
fn input<'a>(
    p: &'a PreparedPerformanceBinding,
    i: &'a Value,
    o: &'a NaraOccasion,
) -> ReceivingPreparation<'a> {
    ReceivingPreparation {
        prepared: p,
        context: context(o),
        identity: Some(i),
        current: None,
        original_occasion: Some(o),
        calibration: None,
    }
}
fn controlled_current(i: &Value) -> Value {
    let mut sky = i["natal"]["sky"].clone();
    sky["snapshot_ref"] = json!("reference:controlled-dated-operator-input");
    sky["source_binding"] =
        json!({"registry_revision":ql_mef::m2::catalogue().registry_revision()});
    sky["request"] =
        json!({"schema":"ql.sky-request/v1","epoch":"reference:explicit-controlled-epoch"});
    sky["receipt_unix_ms"] = json!(900);
    sky["provider"] =
        json!({"adapter_sha256":"reference:unqualified-controlled-input; no astronomy verdict"});
    current::personal_current(i, &current::transit(Some(&sky)).unwrap()).unwrap()
}
#[test]
fn original_ten_denominator_drives_nine_independent_sources_seven_centres_and_distinct_earth() {
    let p = prepared();
    let i = identity(&p, [0., 30., 60., 90., 120., 150., 180., 210., 240., 270.]);
    let o = occasion(&p, &i);
    let d = prepare_native_receiving(input(&p, &i, &o)).unwrap();
    let v = d.snapshot().unwrap();
    assert_eq!(d.drivers().len(), 9);
    assert_eq!(d.centres().len(), 7);
    assert_eq!(
        d.drivers()
            .iter()
            .map(|x| x.native_planet_id)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 4, 5, 6, 8, 9]
    );
    let expected = ql_mef::nara::intake_composition::natal_composition(&i["natal"]).unwrap();
    let all = expected["planetary_contributions"].as_array().unwrap();
    let denominator = all
        .iter()
        .map(|x| x["weighted_contribution"].as_f64().unwrap())
        .sum::<f64>();
    assert_eq!(v["original_total"].as_f64().unwrap(), denominator);
    for source in d.drivers() {
        let original = &all[usize::from(source.native_planet_id)];
        assert_eq!(
            source.hertz,
            original["native_cousto_frequency_hz"].as_f64().unwrap()
        );
        assert_eq!(
            source.original_denominator_share,
            original["weighted_contribution"].as_f64().unwrap() / denominator
        );
        assert!(!source.relation_refs.is_empty());
    }
    let voiced = d
        .drivers()
        .iter()
        .map(|x| x.original_denominator_share)
        .sum::<f64>();
    let uranus = all[7]["weighted_contribution"].as_f64().unwrap() / denominator;
    assert!((voiced + uranus - 1.).abs() < 1e-14);
    assert!(voiced < 1.);
    assert_eq!(d.centres()[5].driver_ids, vec![1, 8]);
    assert_eq!(d.centres()[6].driver_ids, vec![0, 9]);
    assert_ne!(d.drivers()[1].hertz, d.drivers()[7].hertz);
    assert_ne!(d.drivers()[0].hertz, d.drivers()[8].hertz);
    assert_eq!(v["earth"]["coordinate"], "#2-5-0/1-0");
    assert!(d.centres().iter().all(|c| c.projection.is_none()));
    assert!(d.public_snapshot().unwrap_err().contains("protected"));
    d.validate_sources(input(&p, &i, &o)).unwrap();
}
#[test]
fn original_occurrence_provider_loss_and_absent_calibration_remain_explicit() {
    let p = prepared();
    let i = identity(&p, [0.; 10]);
    let o = occasion(&p, &i);
    let d = prepare_native_receiving(input(&p, &i, &o)).unwrap();
    let v = d.snapshot().unwrap();
    assert_eq!(v["original_occasion"]["occurrence_at_unix_ms"], 100);
    assert_eq!(v["original_occasion"]["receipt_at_unix_ms"], 900);
    assert_eq!(v["current_availability"], "provider-current-unavailable");
    assert!(
        d.native_operation(43199999)
            .unwrap_err()
            .contains("provider unavailable")
    );
    let c = current::personal_current(&i, &current::transit(None).unwrap()).unwrap();
    let mut next = input(&p, &i, &o);
    next.current = Some(&c);
    let unavailable = prepare_native_receiving(next).unwrap();
    assert!(unavailable.native_operation(43199999).is_err());
    assert_eq!(
        unavailable.snapshot().unwrap()["current_availability"],
        "provider-or-identity-current-unavailable"
    );
    assert_eq!(d.drivers()[0].hertz, unavailable.drivers()[0].hertz);
}
#[test]
fn lost_planetary_or_personal_edge_is_detected_against_actual_source_registry() {
    let p = prepared();
    let i = identity(&p, [0.; 10]);
    let o = occasion(&p, &i);
    let d = prepare_native_receiving(input(&p, &i, &o)).unwrap();
    let personal = native_current_m_registry()
        .manifest()
        .relations
        .iter()
        .find(|r| {
            r.from_ref.as_deref() == Some("#4.4.4")
                && r.to_ref.as_deref() == Some("#4.4.4.4")
                && r.source_kind == "CONTAINS_REFLECTION_SPACE"
        })
        .unwrap()
        .relation_ref
        .clone();
    for lost in [d.drivers()[0].relation_refs[0].clone(), personal] {
        let mut m = native_current_m_registry().manifest().clone();
        let before = m.relations.len();
        m.relations.retain(|r| r.relation_ref != lost);
        assert_eq!(
            before - m.relations.len(),
            1,
            "the actual canonical source edge must be removed"
        );
        // A structural registry rejection also correctly detects the loss;
        // no weakened validation or manufactured graph is admitted.
        match MRegistry::from_json(&serde_json::to_string(&m).unwrap()) {
            Ok(registry) => assert!(d.validate_source_registry(&registry).is_err()),
            Err(e) => assert!(!e.is_empty()),
        }
    }
    let mut m = native_current_m_registry().manifest().clone();
    let records = m
        .nodes
        .iter()
        .find(|n| n.source_ref == "#2-5-0/1-0")
        .unwrap()
        .records
        .clone();
    assert!(!records.is_empty());
    for record in records {
        m.records[record].payload_sha256 = "0".repeat(64);
    }
    match MRegistry::from_json(&serde_json::to_string(&m).unwrap()) {
        Ok(registry) => assert!(
            d.validate_source_registry(&registry)
                .unwrap_err()
                .contains("Earth")
        ),
        Err(e) => assert!(!e.is_empty()),
    }
    d.validate_source_registry(native_current_m_registry())
        .unwrap();
}
#[test]
fn changed_identity_original_event_or_shared_consent_refuses_before_preparation() {
    let p = prepared();
    let i = identity(&p, [0.; 10]);
    let mut o = occasion(&p, &i);
    o.event.event_ref = "different:event".into();
    assert!(
        prepare_native_receiving(input(&p, &i, &o))
            .unwrap_err()
            .contains("occasion")
    );
    let o = occasion(&p, &i);
    let mut bad = i.clone();
    bad["input_revision"] = json!("invented");
    assert!(
        prepare_native_receiving(input(&p, &bad, &o))
            .unwrap_err()
            .contains("identity")
    );
    let mut shared = input(&p, &i, &o);
    shared.context.kind = ContextKind::Shared;
    assert!(
        prepare_native_receiving(shared)
            .unwrap_err()
            .contains("opt-in")
    );
    let d = prepare_native_receiving(input(&p, &i, &o)).unwrap();
    let changed = identity(&p, [30.; 10]);
    let revised = occasion(&p, &changed);
    assert!(d.validate_sources(input(&p, &changed, &revised)).is_err());
}
#[test]
fn ordinary_public_world_has_no_personal_occasion_or_modulation() {
    let p = prepared();
    let public = ReceivingContext {
        kind: ContextKind::World,
        context: reference("reference:world"),
        receiver: reference("reference:world-receiver"),
        protected_state: None,
        consent: None,
        original_occasion: None,
        private: false,
    };
    let d = prepare_native_receiving(ReceivingPreparation {
        prepared: &p,
        context: public.clone(),
        identity: None,
        current: None,
        original_occasion: None,
        calibration: None,
    })
    .unwrap();
    let v = d.public_snapshot().unwrap();
    assert!(v["original_occasion"].is_null());
    assert!(v["identity_digest"].is_null());
    assert!(d.drivers().is_empty());
    assert!(d.centres().is_empty());
    let op = d.native_operation(43199999).unwrap();
    assert_eq!(op.native_sample, "43199999");
    op.validate_single_projection_port().unwrap();
    let i = identity(&p, [0.; 10]);
    assert!(
        prepare_native_receiving(ReceivingPreparation {
            prepared: &p,
            context: public,
            identity: Some(&i),
            current: None,
            original_occasion: None,
            calibration: None
        })
        .is_err()
    );
}
#[test]
fn explicit_calibration_uses_actual_metric_nodes_and_cannot_pass_current_single_force_port() {
    let p = prepared();
    let i = identity(&p, [0.; 10]);
    let o = occasion(&p, &i);
    let r = p.physical_body().request();
    let mut calibration = ReceivingCalibration {
        provenance: PhysicalProvenance {
            reference: "reference:instrument-calibration".into(),
            revision: "1".into(),
            source_ref: "reference:performer-calibration".into(),
            standing: PhysicalStanding::Reference,
        },
        preparation: Reference {
            reference: r.preparation_ref.clone(),
            revision: r.body_revision.to_string(),
        },
        state: Reference {
            reference: r.state_ref.clone(),
            revision: r.body_revision.to_string(),
        },
        source_force_newtons: 1.,
        centres: std::array::from_fn(|ordinal| CentreCalibration {
            ordinal: ordinal as u8,
            projection: r.exciter.clone(),
        }),
    };
    let c = controlled_current(&i);
    assert_eq!(c["baseline_available"], true);
    let mut next = input(&p, &i, &o);
    next.calibration = Some(&calibration);
    next.current = Some(&c);
    let d = prepare_native_receiving(next).unwrap();
    assert!(d.centres().iter().all(|x| x.projection.is_some()));
    let op = d.native_operation(43199999).unwrap();
    assert_eq!(op.sources.len(), 9);
    assert_eq!(op.projections.len(), 7);
    assert_eq!(op.native_sample, "43199999");
    assert_eq!(
        op.source_instance,
        p.determination()["identity"]["instance"].as_str().unwrap()
    );
    assert!(
        op.validate_single_projection_port()
            .unwrap_err()
            .contains("distinct-native-force-projections-unavailable")
    );
    for driver in &op.sources {
        assert_eq!(driver.share_denominator, d.drivers()[0].share_denominator);
        let n = driver.share_numerator.parse::<u64>().unwrap();
        let total = driver.share_denominator.parse::<u64>().unwrap();
        assert!((n as f64 / total as f64 - driver.original_denominator_share).abs() < 1e-14);
        assert_eq!(driver.peak_force_newtons, driver.original_denominator_share);
        assert_eq!(
            op.projections[usize::from(driver.centre_ordinal)].projection_ref,
            driver.projection_ref
        );
    }
    calibration.centres[0].projection.node_weights[0] = 1.;
    let mut bad = input(&p, &i, &o);
    bad.calibration = Some(&calibration);
    assert!(
        prepare_native_receiving(bad)
            .unwrap_err()
            .contains("normalized")
    );
    calibration.centres[0].projection.node_weights = vec![1., 0.];
    let mut bad = input(&p, &i, &o);
    bad.calibration = Some(&calibration);
    assert!(
        prepare_native_receiving(bad)
            .unwrap_err()
            .contains("disconnected")
    );
}
