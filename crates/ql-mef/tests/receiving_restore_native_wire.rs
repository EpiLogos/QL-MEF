// Real same-source World/Personal/Shared constructors and native source-form
// recipe reused from the existing current-receiving producer test. Nothing in
// this fixture turns JSON into host/Act/consent authority. The C++ consumer
// executes actual stopped A/P/Manager continuation; installed custody is separate.
#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::{
    coupled::{CoupledBasis, CoupledInput},
    performance::PerformanceOwner,
    performance_receiving::NativePerformanceReceivingSource,
};
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference};
use ql_mef::nara::{
    EventBasisRefs, SourceRevision, current, domain::ProtectedRef, intake::IdentityProfile,
    replay::NaraOccasion,
};
use ql_mef::nara_performance_receiving::*;
use ql_mef::performance_audio::PreparedPerformanceBinding;
use ql_mef::physical_body::{PhysicalProvenance, PhysicalStanding, SpatialProjection};
use serde_json::{Value, json};
fn preserve_native_restore_evidence(input: &[u8], result: &std::process::Output) {
    use std::io::Write;
    let Some(path) = std::env::var_os("QL_RECEIVING_RESTORE_EVIDENCE_DIR") else {
        return;
    };
    let root = std::path::Path::new(&path);
    std::fs::create_dir(root).expect("fresh native receiving restore evidence directory");
    for (name, bytes, limit) in [
        ("producer-input.json", input, 16 * 1024 * 1024),
        (
            "native-stdout.json",
            result.stdout.as_slice(),
            32 * 1024 * 1024,
        ),
        (
            "native-stderr.txt",
            result.stderr.as_slice(),
            4 * 1024 * 1024,
        ),
    ] {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(name))
            .unwrap();
        file.write_all(&bytes[..bytes.len().min(limit)]).unwrap();
        file.sync_all().unwrap();
        if bytes.len() > limit {
            let mut marker = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(root.join(format!("{name}.truncated.json")))
                .unwrap();
            write!(
                marker,
                "{}",
                json!({"complete":false,"actual_bytes":bytes.len(),"retained_prefix_bytes":limit})
            )
            .unwrap();
        }
    }
    let mut exit = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join("native-exit.json"))
        .unwrap();
    write!(
        exit,
        "{}",
        json!({"success":result.status.success(),"code":result.status.code()})
    )
    .unwrap();
    exit.sync_all().unwrap();
    assert!(
        input.len() <= 16 * 1024 * 1024
            && result.stdout.len() <= 32 * 1024 * 1024
            && result.stderr.len() <= 4 * 1024 * 1024,
        "native receiving restore evidence exceeded bounded custody; incomplete prefix marked explicitly"
    );
}
fn reference(s: &str) -> Reference {
    Reference {
        reference: s.into(),
        revision: "1".into(),
    }
}
fn returned(c: &ReceivingContext) -> ReturnContext {
    let r = |v: &Reference| ReturnReference {
        reference: v.reference.clone(),
        revision: v.revision.clone(),
    };
    ReturnContext {
        kind: match c.kind {
            ContextKind::World => "world",
            ContextKind::Personal => "personal",
            ContextKind::Shared => "shared",
        }
        .into(),
        context: r(&c.context),
        receiver: r(&c.receiver),
        source_occasion: c.original_occasion.as_ref().map(r),
        protected_state: c.protected_state.as_ref().map(r),
        consent: c.consent.as_ref().map(r),
        private: c.private,
        required_assets: vec![],
    }
}
fn world_context() -> ReturnContext {
    returned(&ReceivingContext {
        kind: ContextKind::World,
        context: reference("controlled:receiving/world"),
        receiver: reference("controlled:receiving/receiver"),
        protected_state: None,
        consent: None,
        original_occasion: None,
        private: false,
    })
}
struct Personal {
    profile: IdentityProfile,
    natal: Value,
    sky: Value,
    occasion: NaraOccasion,
    calibration: ReceivingCalibration,
    context: ReceivingContext,
}
impl Personal {
    fn new(p: &PreparedPerformanceBinding) -> Self {
        let profile:IdentityProfile=serde_json::from_value(json!({"schema":"ql.nara-identity-profile/v1","person_ref":p.physical_body().subject_ref(),"nara_ref":"controlled:source-context/nara","name":"Controlled source operator","encoding_policy":ql_mef::nara::identity_encoding::EncodingPolicy::default(),"composition_policy":"draft-core-birthdate-decanic-40-60-v1","birth":{"date":"1990-06-15","time":null,"precision":"unknown","uncertainty_minutes":null,"fold":null,"place":null},"jungian":null,"gene_keys":null,"human_design":null,"quintessence":null})).unwrap();
        let natal = json!({"schema":"ql.nara-natal/v1","request":profile.natal_request().unwrap(),"status":"partial","reason":"controlled angular source; no astronomical provider verdict","chart":null,"sky":{"schema":"ql.sky-snapshot/v1","snapshot_ref":"reference:source-context/natal","bodies":(["Sun","Moon","Mercury","Venus","Mars","Jupiter","Saturn","Uranus","Neptune","Pluto"].iter().enumerate().map(|(i,name)|json!({"native_planet_id":i,"body":name,"longitude_degrees":30.0*i as f64})).collect::<Vec<_>>())}});
        let identity = profile.inspect(Some(&natal)).unwrap();
        let mut sky = identity["natal"]["sky"].clone();
        sky["snapshot_ref"] = json!("reference:source-context/current-angular-input");
        sky["source_binding"] =
            json!({"registry_revision":ql_mef::m2::catalogue().registry_revision()});
        sky["request"] = json!({"schema":"ql.sky-request/v1","epoch":"reference:controlled-epoch"});
        sky["receipt_unix_ms"] = json!(900);
        sky["provider"] =
            json!({"adapter_sha256":"reference:controlled-input; no astronomy verdict"});
        current::personal_current(&identity, &current::transit(Some(&sky)).unwrap()).unwrap();
        let occasion = NaraOccasion {
            occasion_ref: "controlled:source-context/original-occasion".into(),
            subject_id: p.physical_body().subject_ref().into(),
            event: EventBasisRefs::from_basis(p.native_basis()).unwrap(),
            personal_reception_generation: 1,
            identity_revision: identity["input_revision"].as_str().unwrap().into(),
            day_ref: "central:day:2026-10-02".into(),
            now_ref: "central:now:controlled-source-context".into(),
            occurrence_at_unix_ms: 100,
            receipt_at_unix_ms: 900,
            protected_state_ref: ProtectedRef {
                ref_id: "controlled:source-context/protected".into(),
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
                revision: ql_mef::m_tree::native_current_m_registry()
                    .manifest()
                    .source_revision
                    .clone(),
                standing_ref: "source".into(),
            }],
        };
        let context = ReceivingContext {
            kind: ContextKind::Personal,
            context: reference("controlled:source-context/personal"),
            receiver: reference("controlled:source-context/receiver"),
            protected_state: Some(reference(&occasion.protected_state_ref.ref_id)),
            consent: None,
            original_occasion: Some(reference(&occasion.occasion_ref)),
            private: true,
        };
        let body = p.physical_body().request();
        let n = body.geometry.nodes.len();
        let calibration = ReceivingCalibration {
            provenance: PhysicalProvenance {
                reference: "controlled:source-context/seven-metric-maps".into(),
                revision: "1".into(),
                source_ref: "reference:explicit-instrument-calibration".into(),
                standing: PhysicalStanding::Reference,
            },
            preparation: Reference {
                reference: body.preparation_ref.clone(),
                revision: body.body_revision.to_string(),
            },
            state: Reference {
                reference: body.state_ref.clone(),
                revision: body.body_revision.to_string(),
            },
            source_force_newtons: 0.1,
            centres: std::array::from_fn(|ordinal| {
                let mut weights = vec![0.; n];
                weights[0] = 1. - (ordinal + 1) as f64 / 8.;
                weights[n - 1] = (ordinal + 1) as f64 / 8.;
                CentreCalibration {
                    ordinal: ordinal as u8,
                    projection: SpatialProjection {
                        axis: [0., 0., 1.],
                        node_weights: weights,
                    },
                }
            }),
        };
        Self {
            profile,
            natal,
            sky,
            occasion,
            calibration,
            context,
        }
    }
    fn source(&self, shared: bool) -> NativePerformanceReceivingSource {
        let mut context = returned(&self.context);
        if shared {
            context.kind = "shared".into();
            context.consent = Some(ReturnReference {
                reference: "controlled:receiving/shared-consent".into(),
                revision: "1".into(),
            });
        }
        if shared {
            NativePerformanceReceivingSource::shared(
                &self.profile,
                Some(&self.natal),
                &self.sky,
                self.occasion.clone(),
                self.calibration.clone(),
                context,
            )
            .unwrap()
        } else {
            NativePerformanceReceivingSource::personal(
                &self.profile,
                Some(&self.natal),
                &self.sky,
                self.occasion.clone(),
                self.calibration.clone(),
                context,
            )
            .unwrap()
        }
    }
}

fn true_world() -> (
    CoupledBasis,
    PerformanceOwner,
    NativePerformanceReceivingSource,
) {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let request:ql_mef::scene::WorldRequest=serde_json::from_value(json!({"schema":ql_mef::scene::WORLD_REQUEST,"instance_ref":"expression:current-receiving/world","event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-current-receiving","texture":[64,64],"units_per_metre":1.0,"sky":sky,"start":{"tick12":3,"cycle":7,"aperture":9}})).unwrap();
    let actual = ql_mef::scene::world(request.clone()).unwrap();
    let input: CoupledInput = serde_json::from_value(actual["event"].clone()).unwrap();
    let current = input.compose().unwrap();
    let (_, mut config) = source::config(true);
    config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    let owner =
        PerformanceOwner::prepare(&current, "expression:current-receiving/world", config).unwrap();
    let receiving =
        NativePerformanceReceivingSource::world_source(request, world_context()).unwrap();
    (current, owner, receiving)
}

#[test]
#[ignore = "requires matching normal-floor native receiving restore binary"]
fn actual_saved_receiving_requalifies_same_body_at_saved_cursor() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let binary = std::env::var("QL_NATIVE_WIRE_TEST").expect("actual native paired driver");
    let (current, owner, world_source) = true_world();
    let personal = Personal::new(owner.binding());
    let ps = personal.source(false);
    let ss = personal.source(true);
    let mut changed_shared = ss.return_context().clone();
    changed_shared.consent.as_mut().unwrap().reference =
        "controlled:restore/other-valid-consent".into();
    let other_shared = NativePerformanceReceivingSource::shared(
        &personal.profile,
        Some(&personal.natal),
        &personal.sky,
        personal.occasion.clone(),
        personal.calibration.clone(),
        changed_shared,
    )
    .unwrap();
    let packet = owner.native_packet().unwrap();
    let one = |source: &NativePerformanceReceivingSource,
               wrong: &NativePerformanceReceivingSource| {
        let at_zero = source.prepare_current(&owner, &current, 0).unwrap();
        let at_saved = source.prepare_current(&owner, &current, 640).unwrap();
        let other = wrong.prepare_current(&owner, &current, 640).unwrap();
        json!({"native_preparation":packet,"native_basis":owner.binding().native_basis(),"native_catalog":owner.native_catalog(),"initial":at_zero.snapshot().unwrap(),"saved_current":at_saved.snapshot().unwrap(),"other_valid_context":other.snapshot().unwrap()})
    };
    let bytes=serde_json::to_vec(&json!({"schema":"ql.receiving-restore-native-fixture/v1","world":one(&world_source,&ps),"personal":one(&ps,&ss),"shared":one(&ss,&other_shared)})).unwrap();
    assert!(bytes.len() < 16 * 1024 * 1024);
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    preserve_native_restore_evidence(&bytes, &output);
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["schema"], "ql.receiving-restore-native-receipt/v1");
    for kind in ["world", "personal", "shared"] {
        assert_eq!(result[kind]["saved_cursor"], "640");
        assert_eq!(result[kind]["resumed_cursor"], "1664");
        assert_eq!(
            result[kind]["native_routes"],
            if kind == "world" { 0 } else { 9 }
        );
        for key in [
            "same_body",
            "original_checkpoint_unchanged",
            "exact_pcm_continuation",
            "exact_modal_qv",
            "original_queues_inputs_preserved",
            "nondefault_programmes_preserved",
            "valid_other_context_refused",
            "guard_reacquisition_refused",
            "catalog_replacement_refused",
            "physical_eigenbasis_refused",
        ] {
            assert_eq!(result[kind][key], true, "{kind}/{key}");
        }
    }
}
