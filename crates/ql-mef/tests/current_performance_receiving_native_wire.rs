#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::{
    coupled::{CoupledBasis, CoupledInput},
    performance::PerformanceOwner,
    performance_receiving::{NativePerformanceReceivingSource, PreparedCurrentReceiving},
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
use sha2::{Digest, Sha256};
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

fn reference_owner() -> (CoupledBasis, PerformanceOwner) {
    let (current, config) = source::config(true);
    let owner =
        PerformanceOwner::prepare(&current, "expression:current-receiving", config).unwrap();
    (current, owner)
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
fn actual_world_source_and_reference_source_replay_full_current_owner() {
    let (current, mut owner, source) = true_world();
    let actual = source.admit_current(&mut owner, &current, 0).unwrap();
    actual
        .validate_current(&source, &owner, &current, 0)
        .unwrap();
    assert_eq!(
        actual.context().snapshot().unwrap()["classifications"][0]["private"],
        false
    );
    let snap = actual.snapshot().unwrap();
    assert!(snap["source_inputs"]["identity_profile"].is_null());
    assert!(snap["source_inputs"]["natal"].is_null());
    assert!(snap["source_inputs"]["original_occasion"].is_null());
    assert_eq!(snap["native_admission"]["operation"]["sources"], json!([]));
    assert_eq!(
        owner
            .binding()
            .physical_body()
            .request()
            .geometry
            .nodes
            .len(),
        12
    );
    let packet = owner.native_packet().unwrap();
    let notes = packet["notes"].as_array().unwrap();
    let receipts = packet["source_key_admission"]["touch_receipts"]
        .as_array()
        .unwrap();
    assert_eq!(notes.len(), receipts.len());
    assert!(notes.len() > 1);
    for (note, receipt) in notes.iter().zip(receipts) {
        assert_eq!(note["touch_ref"], receipt["native_target"]["touch_ref"]);
        assert_eq!(note["hertz"], receipt["native_target"]["hertz"]);
    }
    assert_eq!(snap["source_payload_context"]["private"], false);
    assert_eq!(
        snap["source_payload_context"]["source_inputs_sha256"],
        format!(
            "sha256:{:x}",
            Sha256::digest(serde_json::to_vec(&snap["source_inputs"]).unwrap())
        )
    );
    assert_eq!(
        owner
            .native_catalog()
            .iter()
            .filter(|v| v["available"] == true)
            .count(),
        21
    );
    let (reference, mut ref_owner) = reference_owner();
    let source =
        NativePerformanceReceivingSource::reference_world(&reference, world_context()).unwrap();
    let admitted = source.admit_current(&mut ref_owner, &reference, 0).unwrap();
    assert!(
        admitted.context().snapshot().unwrap()["classifications"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mut changed = reference.input.clone();
    changed
        .source_receipts
        .push(json!({"original":"private source retained"}));
    assert!(
        source
            .prepare_current(&ref_owner, &changed.compose().unwrap(), 0)
            .is_err()
    );
}
#[test]
fn actual_personal_shared_producers_preserve_nine_and_refuse_other_valid_occasion_grant() {
    let (current, mut owner) = reference_owner();
    let personal = Personal::new(owner.binding());
    let source = personal.source(false);
    let admitted = source.admit_current(&mut owner, &current, 0).unwrap();
    admitted
        .validate_current(&source, &owner, &current, 0)
        .unwrap();
    let snap = admitted.snapshot().unwrap();
    assert_eq!(
        snap["native_admission"]["operation"]["sources"]
            .as_array()
            .unwrap()
            .len(),
        9
    );
    assert_eq!(
        snap["native_admission"]["operation"]["projections"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert_eq!(snap["source_context"]["context"]["private"], true);
    assert_eq!(
        snap["source_inputs"]["original_occasion"],
        serde_json::to_value(&personal.occasion).unwrap()
    );
    assert_eq!(
        snap["source_inputs"]["identity_profile"],
        serde_json::to_value(&personal.profile).unwrap()
    );
    let mut other = Personal::new(owner.binding());
    other.occasion.occasion_ref = "controlled:receiving/other-valid-occasion".into();
    other.context.original_occasion = Some(reference(&other.occasion.occasion_ref));
    let other_source = other.source(false);
    other_source.prepare_current(&owner, &current, 0).unwrap();
    assert!(
        admitted
            .validate_current(&other_source, &owner, &current, 0)
            .is_err()
    );
    let shared = personal.source(true);
    let actual_shared = shared.prepare_current(&owner, &current, 0).unwrap();
    assert_eq!(
        actual_shared.snapshot().unwrap()["source_context"]["context"]["kind"],
        "shared"
    );
    assert!(
        admitted
            .validate_current(&shared, &owner, &current, 0)
            .is_err()
    );
    let mut context = shared.return_context().clone();
    context.consent.as_mut().unwrap().reference = "controlled:receiving/other-valid-consent".into();
    let changed = NativePerformanceReceivingSource::shared(
        &personal.profile,
        Some(&personal.natal),
        &personal.sky,
        personal.occasion.clone(),
        personal.calibration.clone(),
        context,
    )
    .unwrap();
    changed.prepare_current(&owner, &current, 0).unwrap();
    assert!(
        actual_shared
            .validate_current(&changed, &owner, &current, 0)
            .is_err()
    );
}
fn prepared(
    current: &CoupledBasis,
    owner: &mut PerformanceOwner,
    source: &NativePerformanceReceivingSource,
) -> Value {
    let actual: PreparedCurrentReceiving = source.admit_current(owner, current, 0).unwrap();
    let probe_scope_digest = format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(owner.source_assets()).unwrap())
    );
    json!({"probe_scope_digest":probe_scope_digest,"native_preparation":owner.native_packet().unwrap(),"native_basis":owner.binding().native_basis(),"native_catalog":owner.native_catalog(),"source_assets":owner.source_assets(),"current_receiving":actual.snapshot().unwrap()})
}
#[test]
#[ignore = "requires matching normal-floor native current-receiving worker-control binary"]
fn actual_current_receiving_source_admits_existing_worker_same_body_callback() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let binary = std::env::var("QL_NATIVE_WIRE_TEST").expect("normal native control driver");
    let (world, mut world_owner, world_source) = true_world();
    let world = prepared(&world, &mut world_owner, &world_source);
    let (personal, mut owner) = reference_owner();
    let inputs = Personal::new(owner.binding());
    let personal = prepared(&personal, &mut owner, &inputs.source(false));
    let (shared, mut owner) = reference_owner();
    let inputs = Personal::new(owner.binding());
    let shared = prepared(&shared, &mut owner, &inputs.source(true));
    let bytes=serde_json::to_vec(&json!({"schema":"ql.current-native-receiving-worker-fixture/v1","world":world,"personal":personal,"shared":shared})).unwrap();
    assert!(bytes.len() < 16 * 1024 * 1024);
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

/// Final owner bundle for C: actual same native FieldHost/worker activation.
/// No fake host, imported admission handle or manufactured consumer readback.
#[test]
#[ignore = "requires the exact normal-floor compiled field worker and a new artifact directory"]
fn actual_existing_field_host_emits_final_activated_source_artifacts() {
    use ql_mef::continuous::host::{FieldHost, HOST_REQUEST, HostOperation, HostRequest};
    use ql_mef::continuous::scene_field::SceneConfig;
    use std::{fs::OpenOptions, io::Write, path::PathBuf, time::Duration};
    let worker =
        PathBuf::from(std::env::var("QL_NATIVE_FIELD_WORKER").expect("exact normal-floor worker"));
    let output = PathBuf::from(
        std::env::var("QL_CURRENT_RECEIVING_ARTIFACT_OUTPUT")
            .expect("new native artifact directory"),
    );
    assert!(!output.exists(), "refuse rewriting native artifacts");
    std::fs::create_dir(&output).unwrap();
    for kind in ["world", "personal", "shared"] {
        let (current, prepared, world_source) = true_world();
        let request: ql_mef::scene::WorldRequest =
            serde_json::from_value(world_source.source_inputs().unwrap()["world_request"].clone())
                .unwrap();
        let actual = ql_mef::scene::world(request).unwrap();
        let scene: SceneConfig = serde_json::from_value(actual["binding"]["host"].clone()).unwrap();
        let instance = scene.instance_ref.clone();
        let receiving = if kind == "world" {
            world_source
        } else {
            Personal::new(prepared.binding()).source(kind == "shared")
        };
        let mut host = FieldHost::open_scene(&worker, scene, Duration::from_secs(20)).unwrap();
        host.bind_performance_receiving_source(receiving).unwrap();
        let ready = host.ready();
        let (_, mut config) = source::config(true);
        config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
            .as_u64()
            .unwrap();
        let request = HostRequest {
            schema: HOST_REQUEST.into(),
            instance_ref: instance.clone(),
            event_ref: ready["field"]["event_ref"].as_str().unwrap().into(),
            subject_ref: ready["field"]["subject_ref"].as_str().unwrap().into(),
            request_id: "1".into(),
            expected_generation: ready["field"]["generation"].as_str().unwrap().into(),
            expected_samples_elapsed: ready["field"]["samples_elapsed"].as_str().unwrap().into(),
            command: HostOperation::PerformancePrepare {
                config: Box::new(config),
            },
        };
        let reply = host.execute(request);
        assert_eq!(reply["status"], "ok", "{reply}");
        assert_eq!(reply["performance"]["accepted"], true, "{reply}");
        let artifact = host.retained_performance_source_artifact(0).unwrap();
        let assets = &artifact["source_assets"];
        assert_eq!(
            assets["current_receiving"]["source_inputs"],
            assets["receiving_source_inputs"]
        );
        assert_eq!(
            assets["current_receiving"]["source_context"],
            assets["source_context"]
        );
        assert_eq!(
            assets["current_receiving"]["receiving_definition"],
            assets["receiving_definition"]
        );
        assert_eq!(
            assets["current_receiving"]["native_admission"]["native_basis"],
            artifact["native_basis"]
        );
        assert_eq!(assets["source_context"]["context"]["kind"], kind);
        assert_eq!(
            assets["consumer_roles"]["personal_nine_force_routes"]["available"],
            kind != "world"
        );
        assert_eq!(
            artifact["native_preparation"]["physical_body"]["request"]["geometry"]["nodes"]
                .as_array()
                .unwrap()
                .len(),
            12
        );
        assert_eq!(
            artifact["native_reading"]["keys"].as_array().unwrap().len(),
            36
        );
        let bytes = serde_json::to_vec(&artifact).unwrap();
        assert!(bytes.len() < 16 * 1024 * 1024);
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(output.join(format!("{kind}.source-performance.json")))
            .unwrap();
        file.write_all(&bytes).unwrap();
        file.write_all(b"\n").unwrap();
        file.sync_all().unwrap();
        // The body/session has not been advanced by artifact retention.
        assert_eq!(artifact["native_reading"]["samples_elapsed"], "0");
        assert_eq!(
            artifact["native_reading"]["physical"]["samples_elapsed"],
            "0"
        );
    }
}
