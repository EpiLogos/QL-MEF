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
fn prepare_native_restore_evidence(kind: &str, input: &[u8]) -> Option<std::path::PathBuf> {
    use std::io::Write;
    let Some(path) = std::env::var_os("QL_RECEIVING_RESTORE_EVIDENCE_DIR") else {
        eprintln!(
            "actual_receiving_restore_preexecution context={kind} input_bytes={} input_limit={}",
            input.len(),
            16 * 1024 * 1024
        );
        return None;
    };
    let root = std::path::Path::new(&path);
    std::fs::create_dir_all(root).expect("native receiving restore evidence root");
    let context = root.join(kind);
    std::fs::create_dir(&context).expect("fresh named native context evidence destination");
    let limit = 16 * 1024 * 1024;
    let mut source = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(context.join("producer-input.json"))
        .unwrap();
    source.write_all(&input[..input.len().min(limit)]).unwrap();
    source.sync_all().unwrap();
    let mut metadata = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(context.join("native-preexecution.json"))
        .unwrap();
    write!(
        metadata,
        "{}",
        json!({"schema":"ql.native-receiving-restore-preexecution/v1",
        "context_kind":kind,"actual_input_bytes":input.len(),"stdin_limit_bytes":limit,
        "retained_input_bytes":input.len().min(limit),"complete_input":input.len() <= limit,
        "child_spawned":false})
    )
    .unwrap();
    metadata.sync_all().unwrap();
    if input.len() > limit {
        let mut marker = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(context.join("producer-input.json.truncated.json"))
            .unwrap();
        write!(
            marker,
            "{}",
            json!({"complete":false,"actual_bytes":input.len(),"retained_prefix_bytes":limit})
        )
        .unwrap();
        marker.sync_all().unwrap();
    }
    Some(context)
}
fn preserve_native_restore_evidence(root: Option<&std::path::Path>, result: &std::process::Output) {
    use std::io::Write;
    let Some(root) = root else {
        return;
    };
    for (name, bytes, limit) in [
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
            marker.sync_all().unwrap();
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
        result.stdout.len() <= 32 * 1024 * 1024 && result.stderr.len() <= 4 * 1024 * 1024,
        "native receiving restore output exceeds bounded custody; incomplete prefix marked explicitly"
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
        use sha2::{Digest, Sha256};
        let mut fixture = json!({"native_preparation":packet,"native_basis":owner.binding().native_basis(),"native_catalog":owner.native_catalog(),"initial":at_zero.snapshot().unwrap(),"saved_current":at_saved.snapshot().unwrap(),"other_valid_context":other.snapshot().unwrap()});
        // Actual content digest of this complete native numerical test source.
        // This explicit component scope grants no selected Scene/Act authority.
        fixture["scope_performance_digest"] = json!(format!(
            "sha256:{:x}",
            Sha256::digest(serde_json::to_vec(&fixture).unwrap())
        ));
        fixture
    };
    // All original source fields and every actual trial remain complete. The
    // bounded child request carries ONE named context, never three duplicate
    // native bases at once. The legacy complete-three CPP path remains intact.
    let mut result = json!({"schema":"ql.receiving-restore-native-receipt/v1"});
    for kind in ["world", "personal", "shared"] {
        let context = match kind {
            "world" => one(&world_source, &ps),
            "personal" => one(&ps, &ss),
            "shared" => one(&ss, &other_shared),
            _ => unreachable!(),
        };
        let request = json!({"schema":"ql.receiving-restore-native-fixture/v1",
            "context_kind":kind,"context":context});
        let bytes = serde_json::to_vec(&request).unwrap();
        let evidence = prepare_native_restore_evidence(kind, &bytes);
        // Preexecution byte count and bounded original prefix exist even when
        // this limit refuses. No fake child exit or callback pulse is emitted.
        assert!(bytes.len() < 16 * 1024 * 1024);
        assert_eq!(
            serde_json::from_slice::<Value>(&bytes).unwrap()["context"],
            context
        );
        let mut child = Command::new(&binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        if let Some(root) = evidence.as_deref() {
            let mut started = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(root.join("native-child-started.json"))
                .unwrap();
            write!(
                started,
                "{}",
                json!({"child_spawned":true,"process_id":child.id()})
            )
            .unwrap();
            started.sync_all().unwrap();
        }
        child.stdin.take().unwrap().write_all(&bytes).unwrap();
        let output = child.wait_with_output().unwrap();
        preserve_native_restore_evidence(evidence.as_deref(), &output);
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(actual["schema"], "ql.receiving-restore-native-receipt/v1");
        assert_eq!(actual.as_object().unwrap().len(), 2);
        assert!(actual[kind].is_object());
        let echo =
            &actual[kind]["control"]["readmission_reply"]["payload"]["receiving_readmission"];
        assert_eq!(
            echo["current_source_packet"],
            request["context"]["native_preparation"]
        );
        assert_eq!(
            echo["current_receiving"],
            request["context"]["saved_current"]
        );
        result[kind] = actual[kind].clone();
    }
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
        let control = &result[kind]["control"];
        for key in [
            "actual_control_exact_continuation",
            "actual_control_other_context_refused",
            "actual_control_caller_source_mutation_refused",
            "actual_control_observer_custody_preserved",
            "actual_control_full_checkpoint_text",
            "actual_control_malformed_checkpoint_text_refused",
        ] {
            assert_eq!(control[key], true, "{kind}/{key}");
        }
        let reply = &control["readmission_reply"];
        assert_eq!(reply["schema"], "ql.performance-worker-reply/v1");
        assert_eq!(reply["operation"], "restore-current-receiving");
        assert_eq!(reply["accepted"], true);
        assert_eq!(reply["reading"]["samples_elapsed"], "640");
        assert_eq!(reply["reading"]["physical"]["samples_elapsed"], "640");
        assert_eq!(reply["reading"]["transport_epoch"], "2");
        let applications = reply["applications"].as_array().unwrap();
        let history = reply["input_history"].as_array().unwrap();
        assert_eq!(applications.len(), 1);
        assert_eq!(history.len(), 2);
        for (name, expected) in [
            ("requested_sample", "0"),
            ("admitted_sample", "0"),
            ("applied_sample", "0"),
            ("sequence", "1"),
            ("committed_cursor", "128"),
        ] {
            assert_eq!(applications[0][name], expected, "{kind}/{name}");
        }
        assert_eq!(applications[0]["applied"], true);
        assert_eq!(history[0]["native_sequence"], "1");
        ql_mef::performance_management::qualify_native_note_wire(
            &packet["notes"][0],
            &applications[0]["note"],
        )
        .unwrap();
        ql_mef::performance_management::qualify_native_note_wire(
            &packet["notes"][0],
            &history[0]["target"],
        )
        .unwrap();
        let evidence = &reply["payload"]["receiving_readmission"];
        assert_eq!(evidence["schema"], "ql.native-receiving-readmission/v1");
        assert_eq!(evidence["transport_ack"], reply["payload"]["transport_ack"]);
        assert_eq!(evidence["transport_ack"]["previous_epoch"], "1");
        assert_eq!(evidence["transport_ack"]["epoch"], "2");
        assert_eq!(evidence["transport_ack"]["previous_cursor"], "0");
        assert_eq!(evidence["transport_ack"]["target_sample"], "640");
        assert_eq!(evidence["transport_ack"]["accepted_sequence"], "2");
        let original: Value =
            serde_json::from_str(evidence["original_checkpoint_wire"].as_str().unwrap()).unwrap();
        let operative: Value =
            serde_json::from_str(evidence["operative_checkpoint_wire"].as_str().unwrap()).unwrap();
        let after: Value =
            serde_json::from_str(evidence["after_checkpoint_wire"].as_str().unwrap()).unwrap();
        // Restoring and pulsing preserves the original unread application,
        // then appends its genuine same-input feedback to the admitted journal.
        // Compare every original operand, not just queue lengths or labels.
        assert_eq!(
            reply["applications"], original["native_pair"]["audio"]["applications"]["entries"],
            "{kind}/whole original native application"
        );
        let admitted = original["input_history"]["entries"].as_array().unwrap();
        assert_eq!(admitted.len(), 1);
        assert_eq!(admitted[0]["change"], 0);
        assert_eq!(history[0], admitted[0], "{kind}/whole original admission");
        let saved_ordinal = original["input_history"]["last_ordinal"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let next_ordinal = saved_ordinal.checked_add(1).unwrap().to_string();
        let mut expected_applied = admitted[0].clone();
        expected_applied["ordinal"] = json!(next_ordinal);
        expected_applied["native_sequence"] = applications[0]["sequence"].clone();
        expected_applied["change"] = json!(2);
        expected_applied["operation"] = json!(0);
        assert_eq!(history[1], expected_applied, "{kind}/whole native feedback");
        assert_eq!(after["input_history"]["last_ordinal"], next_ordinal);
        assert_eq!(after["input_history"]["write"], next_ordinal);
        assert_eq!(after["input_history"]["read"], next_ordinal);
        let mut expected_inputs = original["inputs"].clone();
        assert_eq!(expected_inputs.as_array().unwrap().len(), 1);
        assert_eq!(expected_inputs[0]["press_applied"], false);
        expected_inputs[0]["press_applied"] = json!(true);
        assert_eq!(
            after["inputs"], expected_inputs,
            "{kind}/whole input custody"
        );
        assert_eq!(original["transport_epoch"], operative["transport_epoch"]);
        assert_eq!(after["transport_epoch"], "2");
        assert_eq!(
            original["native_pair"]["physical"],
            operative["native_pair"]["physical"]
        );
        assert_eq!(
            operative["native_pair"]["physical"],
            after["native_pair"]["physical"]
        );
        assert_eq!(original["native_pair"]["audio"]["cursor"], "640");
        assert_eq!(
            original["input_history"]["entries"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert!(
            after["input_history"]["entries"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
}
