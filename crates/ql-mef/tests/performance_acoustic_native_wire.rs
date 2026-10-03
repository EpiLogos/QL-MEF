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
    let (current, mut config) = source::config(true);
    config.use_native_m1_harmonic_ratio = false;
    config.excitation.root_linear = 1.0;
    config.excitation.octet_linear = 0.0;
    config.controls.material.damping_alpha_per_second = 40.0;
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
    config.use_native_m1_harmonic_ratio = false;
    config.excitation.root_linear = 1.0;
    config.excitation.octet_linear = 0.0;
    config.controls.material.damping_alpha_per_second = 40.0;
    config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    let owner =
        PerformanceOwner::prepare(&current, "expression:current-receiving/world", config).unwrap();
    let receiving =
        NativePerformanceReceivingSource::world_source(request, world_context()).unwrap();
    (current, owner, receiving)
}

use ql_mef::continuous::performance::{AcousticConfiguration, AcousticDirectivity};
fn configuration(owner: &PerformanceOwner, speed: f64) -> AcousticConfiguration {
    let mut translation = [0.; 3];
    let body = owner.binding().physical_body().request();
    for (node, weight) in body.geometry.nodes.iter().zip(&body.pickup.node_weights) {
        for (value, rest) in translation.iter_mut().zip(node.rest_metres) {
            *value -= rest * weight;
        }
    }
    AcousticConfiguration {
        schema: "ql.native-acoustic-receiving-configuration/v1".into(),
        source_ref: "native:acoustic/source-form-pickup".into(),
        source_motion_ref: "native:acoustic/source-metric-velocity".into(),
        receiver_motion_ref: "native:acoustic/receiver-metric-velocity".into(),
        policy_ref: "native:acoustic/point-source-retarded-linear-delay".into(),
        policy_revision: "1".into(),
        standing: "architecture-model".into(),
        revision: 1,
        source_translation_metres: translation,
        receiver_position_metres: [20., 0., 0.],
        receiver_forward: [-1., 0., 0.],
        source_velocity_metres_per_second: [speed, 0., 0.],
        receiver_velocity_metres_per_second: [0., 0., 0.],
        speed_metres_per_second: 340.,
        minimum_distance_metres: 1.,
        directivity: AcousticDirectivity::Omnidirectional,
        propagation_delay: true,
        span_samples: 96000,
    }
}
fn prepared(
    kind: &str,
    speed: f64,
    birth: u64,
) -> (
    CoupledBasis,
    PerformanceOwner,
    NativePerformanceReceivingSource,
    Value,
) {
    let (basis, mut owner, source) = match kind {
        "world" => true_world(),
        "personal" | "shared" => {
            let (basis, owner) = reference_owner();
            let p = Personal::new(owner.binding());
            (basis, owner, p.source(kind == "shared"))
        }
        _ => panic!("closed actual context required"),
    };
    let config = configuration(&owner, speed);
    let source = source.with_acoustic_configuration(config).unwrap();
    let receiving = source.admit_current(&mut owner, &basis, birth).unwrap();
    let acoustic = owner
        .prepare_acoustic_receiving_source(&basis, &source, birth)
        .unwrap();
    receiving
        .validate_current(&source, &owner, &basis, birth)
        .unwrap();
    acoustic
        .validate_current(&owner, &basis, &source, birth)
        .unwrap();
    let mut fixture = json!({"schema":"ql.native-acoustic-owner-fixture/v1","context_kind":kind,
        "native_preparation":owner.native_packet().unwrap(),"native_basis":owner.binding().native_basis(),
        "native_catalog":owner.native_catalog(),"source_assets":owner.source_assets(),
        "current_receiving":receiving.snapshot().unwrap(),"acoustic":acoustic.snapshot()});
    let digest = format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(&fixture).unwrap())
    );
    fixture["probe_scope_digest"] = json!(digest);
    (basis, owner, source, fixture)
}

#[test]
fn actual_receiver_segment_retains_original_source_history_and_context() {
    for kind in ["world", "personal", "shared"] {
        let (basis, owner, source, _) = prepared(kind, 5.0, 128);
        let original = owner
            .prepare_acoustic_receiving_source(&basis, &source, 128)
            .unwrap();
        let mut after = source.acoustic_configuration().unwrap().clone();
        after.revision = 2;
        after.policy_revision = "2".into();
        after.receiver_motion_ref = "native:acoustic/receiver-metric-velocity-2".into();
        after.receiver_position_metres = [21., 0., 0.];
        let after_source = source
            .clone()
            .with_acoustic_configuration(after.clone())
            .unwrap();
        let update = owner
            .prepare_acoustic_receiver_update(&basis, &after_source, &original, 4096)
            .unwrap();
        update
            .validate_current(&owner, &basis, &after_source, 128)
            .unwrap();
        assert_eq!(update.packet()["origin_sample"], "4096");
        assert_eq!(update.packet()["history_origin_sample"], "128");
        assert_eq!(update.packet()["end_sample"], "100096");
        assert_eq!(
            update.packet()["configuration"],
            serde_json::to_value(&after).unwrap()
        );
        assert_eq!(
            update.packet()["source_body"],
            original.packet()["source_body"]
        );
        assert_eq!(update.packet()["context"], original.packet()["context"]);
        assert_eq!(
            update.current_receiving()["native_admission"]["operation"]["native_sample"],
            "4096"
        );
        assert_eq!(
            update.current_receiving()["native_admission"]["operation"]["sources"],
            original.current_receiving()["native_admission"]["operation"]["sources"]
        );
        assert_eq!(
            update.current_receiving()["source_payload_context"]["private"],
            kind != "world"
        );
        assert_ne!(
            update.packet()["source_position_metres"],
            original.packet()["source_position_metres"]
        );
        assert!(
            owner
                .prepare_acoustic_receiver_update(&basis, &after_source, &original, 127)
                .is_err()
        );
        assert!(
            update
                .validate_current(&owner, &basis, &after_source, 129)
                .is_err()
        );
        for variant in 0..5 {
            let mut changed = after.clone();
            match variant {
                0 => changed.revision = 1,
                1 => changed.source_translation_metres[0] += 0.1,
                2 => changed.source_velocity_metres_per_second[0] = 4.0,
                3 => changed.source_motion_ref = "native:acoustic/other-emitter-path".into(),
                _ => changed.source_velocity_metres_per_second[1] = -0.0,
            }
            let valid_other = source.clone().with_acoustic_configuration(changed).unwrap();
            assert!(
                owner
                    .prepare_acoustic_receiver_update(&basis, &valid_other, &original, 4096)
                    .is_err()
            );
        }
        let mut changed_basis = basis.clone();
        changed_basis.m3.clock_steps += 1;
        assert!(
            owner
                .prepare_acoustic_receiver_update(&changed_basis, &after_source, &original, 4096)
                .is_err()
        );
    }
}

#[test]
fn actual_acoustic_source_keeps_native_context_geometry_and_nine_roles() {
    for kind in ["world", "personal", "shared"] {
        let (basis, owner, source, fixture) = prepared(kind, 0., 128);
        let acoustic = owner
            .prepare_acoustic_receiving_source(&basis, &source, 128)
            .unwrap();
        let packet = acoustic.packet();
        assert_eq!(
            packet["source_body"],
            serde_json::to_value(owner.binding().physical_body()).unwrap()
        );
        assert_eq!(
            packet["context"],
            serde_json::to_value(source.return_context()).unwrap()
        );
        assert_eq!(packet["origin_sample"], "128");
        assert_eq!(packet["history_origin_sample"], "128");
        assert_eq!(packet["end_sample"], "96128");
        assert_eq!(packet["source_position_metres"], json!([0., 0., 0.]));
        assert_eq!(
            fixture["current_receiving"]["source_inputs"]["acoustic_receiving"],
            packet["configuration"]
        );
        assert_eq!(
            fixture["current_receiving"]["source_inputs"]["return_context"],
            packet["context"]
        );
        assert_eq!(
            fixture["current_receiving"]["native_admission"]["operation"]["sources"]
                .as_array()
                .unwrap()
                .len(),
            if kind == "world" { 0 } else { 9 }
        );
        assert_eq!(
            fixture["current_receiving"]["source_payload_context"]["private"],
            kind != "world"
        );
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
        assert_eq!(
            owner
                .binding()
                .physical_body()
                .request()
                .geometry
                .edges
                .len(),
            34
        );
        assert_eq!(
            owner.native_packet().unwrap()["notes"]
                .as_array()
                .unwrap()
                .len(),
            7
        );
        let mut changed = configuration(&owner, 0.);
        changed.receiver_position_metres[0] = 21.;
        changed.revision = 2;
        let changed_source = source.clone().with_acoustic_configuration(changed).unwrap();
        assert!(
            acoustic
                .validate_current(&owner, &basis, &changed_source, 128)
                .is_err()
        );
        assert!(
            acoustic
                .validate_current(&owner, &basis, &source, 129)
                .is_err()
        );
        let mut altered = basis.input.clone();
        altered.m3.clock_steps += 1;
        assert!(
            acoustic
                .validate_current(&owner, &altered.compose().unwrap(), &source, 128)
                .is_err()
        );
        assert!(
            owner
                .prepare_acoustic_receiving_source(&basis, &source, u64::MAX - 10)
                .is_err()
        );
    }
}
#[test]
fn numerical_receiver_preparation_does_not_publish_operative_source_assets() {
    for kind in ["world", "personal", "shared"] {
        let (basis, owner, source, fixture) = prepared(kind, 0., 0);
        let original_assets = owner.source_assets().clone();
        // The genuine pure native acoustic producer exists, but its snapshot
        // is not an operative/retained owner asset. No public value grants it.
        assert!(fixture["acoustic"].is_object());
        assert!(owner.source_assets()["acoustic_receiving"].is_null());
        let mut configuration = source.acoustic_configuration().unwrap().clone();
        configuration.revision = 2;
        configuration.policy_revision = "2".into();
        configuration.receiver_position_metres = [21., 0., 0.];
        let after_source = source
            .clone()
            .with_acoustic_configuration(configuration)
            .unwrap();
        assert!(
            owner
                .prepare_acoustic_receiver_assets(&basis, &after_source, 4096)
                .is_err()
        );
        assert_eq!(*owner.source_assets(), original_assets);
    }
}

#[test]
fn closed_acoustic_inputs_cannot_inject_context_or_nonfinite_policy() {
    let (_, owner, _, _) = prepared("world", 0., 0);
    let config = configuration(&owner, 0.);
    let mut raw = serde_json::to_value(&config).unwrap();
    raw["context"] = json!({"private":false});
    assert!(serde_json::from_value::<AcousticConfiguration>(raw).is_err());
    let mut bad = config.clone();
    bad.source_velocity_metres_per_second[0] = 9.;
    assert!(bad.validate(48000).is_err());
    bad = config.clone();
    bad.receiver_forward = [0., 0., 0.];
    assert!(bad.validate(48000).is_err());
    bad = config.clone();
    bad.speed_metres_per_second = f64::NAN;
    assert!(bad.validate(48000).is_err());
    bad = config;
    bad.span_samples = 48000 * 61;
    assert!(bad.validate(48000).is_err());
}

fn prepare_evidence(kind: &str, input: &[u8]) -> Option<std::path::PathBuf> {
    use std::io::Write;
    let Some(path) = std::env::var_os("QL_ACOUSTIC_RECEIVING_EVIDENCE_DIR") else {
        eprintln!(
            "actual_acoustic_receiving_preexecution context={kind} input_bytes={} input_limit={}",
            input.len(),
            16 * 1024 * 1024
        );
        return None;
    };
    let root = std::path::Path::new(&path);
    std::fs::create_dir_all(root).expect("native acoustic receiving evidence root");
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
        json!({"schema":"ql.native-acoustic-receiving-preexecution/v1",
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
fn preserve_evidence(root: Option<&std::path::Path>, result: &std::process::Output) {
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
        "native acoustic receiving output exceeds bounded custody; incomplete prefix marked explicitly"
    );
}

#[test]
#[ignore = "requires normal-floor actual performance_acoustic_wire binary and native source owners"]
fn actual_acoustic_producer_reaches_same_body_output_and_complete_history_reopen() {
    use ql_mef::performance_management::qualify_native_note_wire;
    use std::io::Write;
    use std::process::{Command, Stdio};
    for kind in ["world", "personal", "shared"] {
        let (basis, owner, source, mut packet) = prepared(kind, 0., 0);
        let original = owner
            .prepare_acoustic_receiving_source(&basis, &source, 0)
            .unwrap();
        let mut configuration = source.acoustic_configuration().unwrap().clone();
        configuration.revision = 2;
        configuration.policy_revision = "2".into();
        configuration.receiver_motion_ref = "native:acoustic/receiver-segment-2".into();
        configuration.receiver_position_metres = [21., 0., 0.];
        let after_source = source
            .clone()
            .with_acoustic_configuration(configuration)
            .unwrap();
        let after = owner
            .prepare_acoustic_receiver_update(&basis, &after_source, &original, 4096)
            .unwrap();
        after
            .validate_current(&owner, &basis, &after_source, 0)
            .unwrap();
        packet["after_acoustic"] = after.snapshot();
        let mut configuration = after_source.acoustic_configuration().unwrap().clone();
        configuration.revision = 3;
        configuration.policy_revision = "3".into();
        configuration.receiver_motion_ref = "native:acoustic/receiver-segment-3".into();
        configuration.receiver_position_metres = [22., 0., 0.];
        let second_source = after_source
            .clone()
            .with_acoustic_configuration(configuration)
            .unwrap();
        let second = owner
            .prepare_acoustic_receiver_update(&basis, &second_source, &after, 8192)
            .unwrap();
        second
            .validate_current(&owner, &basis, &second_source, 0)
            .unwrap();
        packet["second_acoustic"] = second.snapshot();
        packet.as_object_mut().unwrap().remove("probe_scope_digest");
        packet["probe_scope_digest"] = json!(format!(
            "sha256:{:x}",
            Sha256::digest(serde_json::to_vec(&packet).unwrap())
        ));
        let input = serde_json::to_vec(&packet).unwrap();
        let evidence = prepare_evidence(kind, &input);
        assert!(
            input.len() < 16 * 1024 * 1024,
            "complete native acoustic input exceeds bound"
        );
        let mut child = Command::new(std::env::var("QL_NATIVE_WIRE_TEST").unwrap())
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
                json!({"actual_native_child_pid":child.id(),"child_spawned":true})
            )
            .unwrap();
            started.sync_all().unwrap();
        }
        child.stdin.take().unwrap().write_all(&input).unwrap();
        let result = child.wait_with_output().unwrap();
        preserve_evidence(evidence.as_deref(), &result);
        assert!(
            result.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        let output: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert_eq!(
            output["schema"],
            "ql.native-acoustic-control-component-receipt/v1"
        );
        assert_eq!(output["context_kind"], kind);
        let audio = &output["checkpoint"]["native_pair"]["audio"];
        assert_eq!(audio["cursor"], "4096");
        assert_eq!(audio["has_receiving"], true);
        let history = &audio["receiving"];
        assert_eq!(history["samples_elapsed"], "4096");
        assert_eq!(history["history_start_sample"], "0");
        assert_eq!(history["history_linear"].as_array().unwrap().len(), 16384);
        assert!(
            history["history_linear"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_f64().unwrap() != 0.)
        );
        let context = &packet["acoustic"]["packet"]["context"];
        assert_eq!(
            history["manifest"]["context"],
            context["context"]["reference"]
        );
        assert_eq!(
            history["manifest"]["receiver"],
            context["receiver"]["reference"]
        );
        assert_eq!(output["physical"]["state"]["samples_elapsed"], "6144");
        assert_eq!(output["reading"]["samples_elapsed"], "6144");
        assert_eq!(output["reading"]["physical"]["samples_elapsed"], "6144");
        assert_eq!(
            output["reading"]["receiving_transport"]["samples_elapsed"],
            "6144"
        );
        for (name, count) in [
            ("baseline_pcm", 4096),
            ("received_pcm", 4096),
            ("continued_pcm", 2048),
        ] {
            let pcm = output[name].as_array().unwrap();
            assert_eq!(pcm.len(), count);
            assert!(pcm.iter().all(|v| v.as_f64().unwrap().is_finite()));
            assert!(pcm.iter().any(|v| v.as_f64().unwrap() != 0.));
        }
        assert_ne!(output["baseline_pcm"], output["received_pcm"]);
        let segment = &output["receiver_segment"];
        assert_eq!(segment["schema"], "ql.native-acoustic-segment-component/v1");
        assert_eq!(segment["end_cursor"], "13000");
        let before_receiving = &segment["before_receiving"];
        let after_receiving = &segment["after_receiving"];
        assert_eq!(before_receiving["samples_elapsed"], "4096");
        assert_eq!(after_receiving["samples_elapsed"], "4096");
        assert_eq!(before_receiving["history_start_sample"], "0");
        assert_eq!(after_receiving["history_start_sample"], "0");
        assert_eq!(
            before_receiving["history_linear"],
            after_receiving["history_linear"]
        );
        assert_eq!(after_receiving["manifest"]["origin_sample"], "4096");
        assert_eq!(after_receiving["manifest"]["history_origin_sample"], "0");
        assert_eq!(
            after_receiving["manifest"]["context"],
            context["context"]["reference"]
        );
        assert_eq!(
            after_receiving["manifest"]["receiver"],
            context["receiver"]["reference"]
        );
        assert_eq!(segment["final_receiving"]["samples_elapsed"], "13000");
        assert_eq!(
            segment["final_receiving"]["history_linear"]
                .as_array()
                .unwrap()
                .len(),
            16384
        );
        assert_eq!(segment["physical"]["state"]["samples_elapsed"], "13000");
        assert_ne!(segment["continued_pcm"], segment["unchanged_pcm"]);
        for field in ["continued_pcm", "unchanged_pcm"] {
            let values = segment[field].as_array().unwrap();
            assert_eq!(values.len(), 8904);
            assert!(values.iter().all(|v| v.as_f64().unwrap().is_finite()));
            assert!(values.iter().any(|v| v.as_f64().unwrap() != 0.));
        }
        let segment_apps = segment["applications"].as_array().unwrap();
        assert_eq!(segment_apps.len(), 3);
        for (index, date) in ["0", "9000", "10000"].iter().enumerate() {
            assert_eq!(segment_apps[index]["sequence"], (index + 1).to_string());
            assert_eq!(
                segment_apps[index]["applied_application_ordinal"],
                (index + 1).to_string()
            );
            for timing in ["requested_sample", "admitted_sample", "applied_sample"] {
                assert_eq!(segment_apps[index][timing], *date);
            }
        }
        let segment_journal = segment["input_history"].as_array().unwrap();
        assert!(
            segment_journal
                .iter()
                .all(|row| row["input_ref"] == "native:acoustic/segment-original-input")
        );
        let target = &packet["native_preparation"]["notes"][0];
        qualify_native_note_wire(target, &segment_apps[0]["note"]).unwrap();
        qualify_native_note_wire(target, &segment_apps[2]["note"]).unwrap();
        for row in segment_journal {
            qualify_native_note_wire(target, &row["target"]).unwrap();
        }
        let control = &output["control_receiver_segment"];
        assert_eq!(
            control["schema"],
            "ql.native-acoustic-control-replacement/v1"
        );
        assert_eq!(control["end_cursor"], "13000");
        let before = &control["before_checkpoint"]["native_pair"]["audio"]["receiving"];
        let saved = &control["saved_checkpoint"]["native_pair"]["audio"]["receiving"];
        for (name, cursor, origin) in [
            ("first_ack", "4096", "4096"),
            ("second_ack", "8192", "8192"),
        ] {
            let ack = &control[name];
            assert_eq!(ack["schema"], "ql.native-receiving-replacement/v1");
            assert_eq!(ack["sample"], cursor);
            assert_eq!(ack["transport_epoch"], "1");
            assert_eq!(ack["accepted_sequence"], "3");
            assert_eq!(ack["after_manifest"]["origin_sample"], origin);
            assert_eq!(ack["after_manifest"]["history_origin_sample"], "0");
            assert_eq!(
                ack["after_manifest"]["context"],
                context["context"]["reference"]
            );
            assert_eq!(
                ack["after_manifest"]["receiver"],
                context["receiver"]["reference"]
            );
        }
        assert_eq!(control["first_ack"]["before_manifest"], before["manifest"]);
        assert_eq!(
            control["second_ack"]["before_manifest"],
            control["first_ack"]["after_manifest"]
        );
        assert_eq!(control["second_ack"]["after_manifest"], saved["manifest"]);
        assert_eq!(before["samples_elapsed"], "4096");
        assert_eq!(saved["samples_elapsed"], "8192");
        assert_eq!(before["history_start_sample"], "0");
        assert_eq!(saved["history_start_sample"], "0");
        assert_eq!(control["final_receiving"]["samples_elapsed"], "13000");
        assert_eq!(control["final_receiving"]["history_start_sample"], "0");
        assert_eq!(control["final_receiving"]["manifest"], saved["manifest"]);
        assert_eq!(control["physical"]["state"]["samples_elapsed"], "13000");
        for receiving in [before, saved, &control["final_receiving"]] {
            assert_eq!(receiving["history_linear"].as_array().unwrap().len(), 16384);
            assert!(
                receiving["history_linear"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|v| v.as_f64().unwrap().is_finite())
            );
        }
        for name in ["continued_pcm", "unchanged_pcm"] {
            let pcm = control[name].as_array().unwrap();
            assert_eq!(pcm.len(), 8904);
            assert!(pcm.iter().all(|v| v.as_f64().unwrap().is_finite()));
            assert!(pcm.iter().any(|v| v.as_f64().unwrap() != 0.));
        }
        assert_ne!(control["continued_pcm"], control["unchanged_pcm"]);
        let apps = control["applications"].as_array().unwrap();
        assert_eq!(apps.len(), 3);
        for (index, sample) in ["0", "9000", "10000"].iter().enumerate() {
            assert_eq!(apps[index]["sequence"], (index + 1).to_string());
            assert_eq!(
                apps[index]["applied_application_ordinal"],
                (index + 1).to_string()
            );
            for name in ["requested_sample", "admitted_sample", "applied_sample"] {
                assert_eq!(apps[index][name], *sample);
            }
        }
        qualify_native_note_wire(target, &apps[0]["note"]).unwrap();
        qualify_native_note_wire(target, &apps[2]["note"]).unwrap();
        for row in control["input_history"].as_array().unwrap() {
            assert_eq!(row["input_ref"], "native:original-janko/input");
            qualify_native_note_wire(target, &row["target"]).unwrap();
        }
        let fresh = &output["fresh_receiver_segment"];
        assert_eq!(
            fresh["schema"],
            "ql.native-acoustic-fresh-segment-component/v1"
        );
        assert_eq!(fresh["end_cursor"], "13000");
        assert_eq!(fresh["saved_receiving"], fresh["restored_receiving"]);
        assert_eq!(fresh["saved_receiving"]["samples_elapsed"], "4096");
        assert_eq!(
            fresh["saved_receiving"]["manifest"]["origin_sample"],
            "4096"
        );
        assert_eq!(fresh["saved_receiving"]["history_start_sample"], "0");
        assert_eq!(
            fresh["saved_receiving"]["manifest"]["history_origin_sample"],
            "0"
        );
        assert_eq!(
            fresh["saved_receiving"]["manifest"]["context"],
            context["context"]["reference"]
        );
        assert_eq!(
            fresh["final_receiving"]["manifest"],
            fresh["saved_receiving"]["manifest"]
        );
        assert_eq!(fresh["final_receiving"]["samples_elapsed"], "13000");
        assert_eq!(fresh["final_receiving"]["history_start_sample"], "0");
        assert_eq!(fresh["physical"]["state"]["samples_elapsed"], "13000");
        for name in ["saved_receiving", "restored_receiving", "final_receiving"] {
            let history = fresh[name]["history_linear"].as_array().unwrap();
            assert_eq!(history.len(), 16384);
            assert!(
                history
                    .iter()
                    .all(|value| value.as_f64().unwrap().is_finite())
            );
            assert!(history.iter().any(|value| value.as_f64().unwrap() != 0.));
        }
        let ack = &fresh["transport_ack"];
        assert_eq!(ack["previous_cursor"], "0");
        assert_eq!(ack["target_sample"], "4096");
        assert_eq!(ack["previous_epoch"], "1");
        assert_eq!(ack["epoch"], "2");
        assert_eq!(ack["accepted_sequence"], "3");
        let apps = fresh["applications"].as_array().unwrap();
        assert_eq!(apps.len(), 2);
        for (index, sample) in ["9000", "10000"].iter().enumerate() {
            assert_eq!(apps[index]["sequence"], (index + 2).to_string());
            assert_eq!(
                apps[index]["applied_application_ordinal"],
                (index + 2).to_string()
            );
            for name in ["requested_sample", "admitted_sample", "applied_sample"] {
                assert_eq!(apps[index][name], *sample);
            }
        }
        qualify_native_note_wire(target, &apps[1]["note"]).unwrap();
        for row in fresh["input_history"].as_array().unwrap() {
            assert_eq!(row["input_ref"], "native:acoustic/segment-original-input");
            qualify_native_note_wire(target, &row["target"]).unwrap();
        }
        let pcm = fresh["continued_pcm"].as_array().unwrap();
        assert_eq!(pcm.len(), 8904);
        assert!(pcm.iter().all(|value| value.as_f64().unwrap().is_finite()));
        assert!(pcm.iter().any(|value| value.as_f64().unwrap() != 0.));
        qualify_native_note_wire(target, &output["native_note"]).unwrap();
        let apps = output["applications"].as_array().unwrap();
        let journal = output["input_history"].as_array().unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(journal.len(), 2);
        for field in ["requested_sample", "admitted_sample", "applied_sample"] {
            assert_eq!(apps[0][field], "0");
        }
        assert_eq!(apps[0]["sequence"], "1");
        assert_eq!(apps[0]["applied_application_ordinal"], "1");
        qualify_native_note_wire(target, &apps[0]["note"]).unwrap();
        for entry in journal {
            assert_eq!(entry["input_ref"], "native:original-janko/input");
            qualify_native_note_wire(target, &entry["target"]).unwrap();
        }
    }
}

#[test]
#[ignore = "requires normal-floor actual Control binary and native source owners; no imported C31 authority"]
fn actual_control_fresh_saved_segment_retains_original_ring_and_future_work() {
    use ql_mef::performance_management::qualify_native_note_wire;
    use std::io::Write;
    use std::process::{Command, Stdio};
    for kind in ["world", "personal", "shared"] {
        for saved_cursor in [4096_u64, 8192] {
            let (basis, owner, source, mut packet) = prepared(kind, 0., 0);
            let original = owner
                .prepare_acoustic_receiving_source(&basis, &source, 0)
                .unwrap();
            let mut configuration = source.acoustic_configuration().unwrap().clone();
            configuration.revision = 2;
            configuration.policy_revision = "2".into();
            configuration.receiver_motion_ref = "native:acoustic/receiver-segment-2".into();
            configuration.receiver_position_metres = [21., 0., 0.];
            let after_source = source
                .clone()
                .with_acoustic_configuration(configuration)
                .unwrap();
            let after = owner
                .prepare_acoustic_receiver_update(&basis, &after_source, &original, 4096)
                .unwrap();
            after
                .validate_current(&owner, &basis, &after_source, 0)
                .unwrap();
            packet["after_acoustic"] = after.snapshot();
            packet["saved_current_receiving"] = after_source
                .prepare_current(&owner, &basis, saved_cursor)
                .unwrap()
                .snapshot()
                .unwrap();
            packet["control_saved_trial"] = json!({"schema":"ql.native-acoustic-control-fresh-trial/v1","saved_cursor":saved_cursor.to_string()});
            packet.as_object_mut().unwrap().remove("probe_scope_digest");
            packet["probe_scope_digest"] = json!(format!(
                "sha256:{:x}",
                Sha256::digest(serde_json::to_vec(&packet).unwrap())
            ));
            let input = serde_json::to_vec(&packet).unwrap();
            let evidence =
                prepare_evidence(&format!("{kind}-control-saved-{saved_cursor}"), &input);
            assert!(
                input.len() < 16 * 1024 * 1024,
                "complete actual saved-source child input exceeds original bound"
            );
            let mut child = Command::new(std::env::var("QL_NATIVE_WIRE_TEST").unwrap())
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
                    json!({"actual_native_child_pid":child.id(),"child_spawned":true})
                )
                .unwrap();
                started.sync_all().unwrap();
            }
            child.stdin.take().unwrap().write_all(&input).unwrap();
            let result = child.wait_with_output().unwrap();
            preserve_evidence(evidence.as_deref(), &result);
            assert!(
                result.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            );
            let output: Value = serde_json::from_slice(&result.stdout).unwrap();
            assert_eq!(
                output["schema"],
                "ql.native-acoustic-control-fresh-restore/v1"
            );
            assert_eq!(output["context_kind"], kind);
            assert_eq!(output["saved_cursor"], saved_cursor.to_string());
            assert_eq!(output["original_segment_origin"], "4096");
            assert_eq!(output["original_history_birth"], "0");
            assert_eq!(output["refusal_count"], 9);
            let receipt = &output["readmission"];
            assert_eq!(receipt["schema"], "ql.native-receiving-readmission/v1");
            assert_eq!(
                receipt["original_saved_acoustic"],
                packet["after_acoustic"]["packet"]
            );
            assert_eq!(
                receipt["current_source_packet"],
                packet["native_preparation"]
            );
            assert_eq!(receipt["actual_native_basis"], packet["native_basis"]);
            assert_eq!(
                receipt["current_receiving"],
                packet["saved_current_receiving"]
            );
            let original: Value =
                serde_json::from_str(receipt["original_checkpoint_wire"].as_str().unwrap())
                    .unwrap();
            let before: Value =
                serde_json::from_str(receipt["before_checkpoint_wire"].as_str().unwrap()).unwrap();
            let operative: Value =
                serde_json::from_str(receipt["operative_checkpoint_wire"].as_str().unwrap())
                    .unwrap();
            let after: Value =
                serde_json::from_str(receipt["after_checkpoint_wire"].as_str().unwrap()).unwrap();
            assert_eq!(before["native_pair"]["audio"]["cursor"], "0");
            assert_eq!(before["native_pair"]["audio"]["has_receiving"], false);
            let receiving = &original["native_pair"]["audio"]["receiving"];
            assert_eq!(receiving, &output["original_receiving"]);
            for checkpoint in [&operative, &after] {
                assert_eq!(checkpoint["native_pair"]["audio"]["receiving"], *receiving);
                assert_eq!(
                    checkpoint["native_pair"]["physical"],
                    original["native_pair"]["physical"]
                );
            }
            assert_eq!(receiving["samples_elapsed"], saved_cursor.to_string());
            assert_eq!(receiving["manifest"]["origin_sample"], "4096");
            assert_eq!(receiving["manifest"]["history_origin_sample"], "0");
            assert_eq!(receiving["history_start_sample"], "0");
            let context = &packet["after_acoustic"]["packet"]["context"];
            assert_eq!(
                receiving["manifest"]["context"],
                context["context"]["reference"]
            );
            assert_eq!(
                receiving["manifest"]["receiver"],
                context["receiver"]["reference"]
            );
            assert_eq!(receiving["history_linear"].as_array().unwrap().len(), 16384);
            assert!(
                receiving["history_linear"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|v| v.as_f64().unwrap() != 0.)
            );
            // Actual original f32 sign bits are preserved by the native decoder
            // and complete native checkpoint comparison, not declassified by a hash.
            for checkpoint in [&operative, &after] {
                for (left, right) in receiving["history_linear"].as_array().unwrap().iter().zip(
                    checkpoint["native_pair"]["audio"]["receiving"]["history_linear"]
                        .as_array()
                        .unwrap(),
                ) {
                    assert_eq!(
                        (left.as_f64().unwrap() as f32).to_bits(),
                        (right.as_f64().unwrap() as f32).to_bits()
                    );
                }
            }
            let ack = &receipt["transport_ack"];
            for (field, value) in [
                ("previous_cursor", "0"),
                ("previous_epoch", "1"),
                ("epoch", "2"),
                ("accepted_sequence", "3"),
            ] {
                assert_eq!(ack[field], value);
            }
            assert_eq!(ack["target_sample"], saved_cursor.to_string());
            assert_eq!(output["end_cursor"], "13000");
            assert_eq!(output["physical"]["state"]["samples_elapsed"], "13000");
            assert_eq!(output["actual_reading"]["samples_elapsed"], "13000");
            assert_eq!(
                output["actual_reading"]["physical"]["samples_elapsed"],
                "13000"
            );
            assert_eq!(
                output["actual_reading"]["receiving_transport"]["samples_elapsed"],
                "13000"
            );
            assert_eq!(output["final_receiving"]["manifest"], receiving["manifest"]);
            assert_eq!(output["final_receiving"]["samples_elapsed"], "13000");
            let applications = output["applications"].as_array().unwrap();
            assert_eq!(applications.len(), 2);
            for (index, sample) in ["9000", "10000"].iter().enumerate() {
                assert_eq!(applications[index]["sequence"], (index + 2).to_string());
                assert_eq!(
                    applications[index]["applied_application_ordinal"],
                    (index + 2).to_string()
                );
                for field in ["requested_sample", "admitted_sample", "applied_sample"] {
                    assert_eq!(applications[index][field], *sample);
                }
            }
            let target = &packet["native_preparation"]["notes"][0];
            qualify_native_note_wire(target, &applications[1]["note"]).unwrap();
            let journal = output["input_history"].as_array().unwrap();
            assert!(!journal.is_empty());
            for row in journal {
                assert_eq!(row["input_ref"], "native:original-janko/input");
                qualify_native_note_wire(target, &row["target"]).unwrap();
            }
            let pcm = output["continued_pcm"].as_array().unwrap();
            assert_eq!(pcm.len(), 13000 - saved_cursor as usize);
            assert!(pcm.iter().all(|v| v.as_f64().unwrap().is_finite()));
            assert!(pcm.iter().any(|v| v.as_f64().unwrap() != 0.));
        }
    }
}

#[test]
#[ignore = "requires actual native Control/A/P receiving binary; private S/Document CAS is separate"]
fn actual_native_resident_registration_keeps_lifetimes_and_output_facts_distinct() {
    use ql_mef::performance_management::qualify_native_note_wire;
    use std::io::Write;
    use std::process::{Command, Stdio};
    for kind in ["world", "personal", "shared"] {
        let (basis, owner, source, mut packet) = prepared(kind, 0., 0);
        let original = owner
            .prepare_acoustic_receiving_source(&basis, &source, 0)
            .unwrap();
        let mut configuration = source.acoustic_configuration().unwrap().clone();
        configuration.revision = 2;
        configuration.policy_revision = "2".into();
        configuration.receiver_motion_ref = "native:acoustic/receiver-segment-2".into();
        configuration.receiver_position_metres = [21., 0., 0.];
        let after_source = source
            .clone()
            .with_acoustic_configuration(configuration)
            .unwrap();
        packet["after_acoustic"] = owner
            .prepare_acoustic_receiver_update(&basis, &after_source, &original, 4096)
            .unwrap()
            .snapshot();
        packet["resident_registry_trial"] = json!(true);
        packet.as_object_mut().unwrap().remove("probe_scope_digest");
        packet["probe_scope_digest"] = json!(format!(
            "sha256:{:x}",
            Sha256::digest(serde_json::to_vec(&packet).unwrap())
        ));
        let input = serde_json::to_vec(&packet).unwrap();
        let evidence = prepare_evidence(&format!("{kind}-actual-resident-registry"), &input);
        assert!(input.len() < 16 * 1024 * 1024);
        let mut child = Command::new(std::env::var("QL_NATIVE_WIRE_TEST").unwrap())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        if let Some(root) = evidence.as_deref() {
            let mut start = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(root.join("native-child-started.json"))
                .unwrap();
            write!(
                start,
                "{}",
                json!({"actual_native_child_pid":child.id(),"child_spawned":true})
            )
            .unwrap();
            start.sync_all().unwrap();
        }
        child.stdin.take().unwrap().write_all(&input).unwrap();
        let result = child.wait_with_output().unwrap();
        preserve_evidence(evidence.as_deref(), &result);
        assert!(
            result.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        let output: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert_eq!(output["schema"], "ql.native-resident-registry-component/v1");
        let role = |v: &Value, name: &str| -> Value {
            v["required_consumers"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["role"] == name)
                .unwrap()
                .clone()
        };
        for name in [
            "initial",
            "other_same_source",
            "installed",
            "queued",
            "played",
            "replaced",
            "continued",
            "restored",
            "replayed",
        ] {
            let registry = &output[name];
            assert_eq!(
                registry["schema"],
                "ql.native-resident-consumer-registry/v1"
            );
            assert_eq!(
                registry["source"],
                packet["native_preparation"]["determination"]["identity"]
            );
            assert_eq!(
                registry["preparation_ref"],
                packet["native_preparation"]["determination"]["body_preparation_ref"]
            );
            assert_eq!(
                registry["state_ref"],
                packet["native_preparation"]["determination"]["body_state_ref"]
            );
            assert_eq!(registry["native_nodes"].as_array().unwrap().len(), 12);
            let roles = registry["required_consumers"].as_array().unwrap();
            assert_eq!(
                roles.len(),
                if name == "initial" || name == "other_same_source" {
                    2
                } else {
                    3
                }
            );
            let mut unique = std::collections::BTreeSet::new();
            for consumer in roles {
                let token = consumer["instance_ref"].as_str().unwrap();
                assert!(token.starts_with("native-resident:v1:"));
                assert!(unique.insert(token));
                assert_eq!(
                    consumer["generation_domain"],
                    "native-resident-construction"
                );
                assert_ne!(consumer["generation"], "0");
                assert_eq!(consumer["sample"], registry["sample"]);
            }
            for (actual, source) in registry["native_nodes"].as_array().unwrap().iter().zip(
                packet["native_preparation"]["physical_body"]["request"]["geometry"]["nodes"]
                    .as_array()
                    .unwrap(),
            ) {
                assert_eq!(
                    actual["native_node_id"],
                    source["identity"].as_u64().unwrap().to_string()
                );
                for axis in 0..3 {
                    assert_eq!(
                        actual["rest_metres"][axis].as_f64().unwrap().to_bits(),
                        source["rest_metres"][axis].as_f64().unwrap().to_bits()
                    );
                }
            }
        }
        for consumer in ["audio_engine", "physical_body"] {
            let initial = role(&output["initial"], consumer);
            assert_ne!(
                initial["instance_ref"],
                role(&output["other_same_source"], consumer)["instance_ref"]
            );
            for state in [
                "installed",
                "queued",
                "played",
                "replaced",
                "continued",
                "restored",
                "replayed",
            ] {
                assert_eq!(
                    initial["instance_ref"],
                    role(&output[state], consumer)["instance_ref"]
                );
                assert_eq!(
                    initial["generation"],
                    role(&output[state], consumer)["generation"]
                );
            }
        }
        assert_ne!(
            role(&output["installed"], "acoustic_receiving")["instance_ref"],
            role(&output["replaced"], "acoustic_receiving")["instance_ref"]
        );
        assert_eq!(
            role(&output["replaced"], "acoustic_receiving")["instance_ref"],
            role(&output["continued"], "acoustic_receiving")["instance_ref"]
        );
        for state in ["initial", "installed", "queued", "replaced", "restored"] {
            assert_eq!(
                output[state]["audio_observation"]["callback_output_committed"],
                false
            );
        }
        for (state, sample) in [
            ("played", "4096"),
            ("continued", "4608"),
            ("replayed", "5632"),
        ] {
            assert_eq!(output[state]["sample"], sample);
            assert_eq!(
                output[state]["audio_observation"]["callback_output_committed"],
                true
            );
            for key in [
                "audio_observation",
                "physical_observation",
                "receiving_observation",
            ] {
                assert_eq!(output[state][key]["sample"], sample);
            }
        }
        assert_eq!(
            output["queued"]["audio_observation"]["last_applied_application_ordinal"],
            "0"
        );
        assert_eq!(
            output["played"]["audio_observation"]["last_applied_application_ordinal"],
            "1"
        );
        assert_eq!(output["restored"]["sample"], "4608");
        assert_eq!(output["restored"]["transport_epoch"], "2");
        assert_eq!(output["exact_restore_continuation_frames"], "1024");
        for state in ["restored", "replayed"] {
            assert_eq!(
                role(&output[state], "acoustic_receiving")["instance_ref"],
                role(&output["replaced"], "acoustic_receiving")["instance_ref"]
            );
        }
        let applications = output["applications"].as_array().unwrap();
        assert_eq!(applications.len(), 1);
        assert_eq!(applications[0]["sequence"], "1");
        assert_eq!(applications[0]["applied_application_ordinal"], "1");
        for field in ["requested_sample", "admitted_sample", "applied_sample"] {
            assert_eq!(applications[0][field], "0");
        }
        let target = &packet["native_preparation"]["notes"][0];
        qualify_native_note_wire(target, &applications[0]["note"]).unwrap();
        let journal = output["input_history"].as_array().unwrap();
        assert_eq!(journal.len(), 2);
        for row in journal {
            assert_eq!(row["input_ref"], "native:original-janko/input");
            qualify_native_note_wire(target, &row["target"]).unwrap();
        }
    }
}
