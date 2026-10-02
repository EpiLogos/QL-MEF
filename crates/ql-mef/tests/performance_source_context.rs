#[path = "support/retained_source_performance.rs"]
mod source;
use ql_mef::continuous::performance::PerformanceOwner;
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference};
use ql_mef::nara::{
    EventBasisRefs, SourceRevision, current, domain::ProtectedRef, intake::IdentityProfile,
    replay::NaraOccasion,
};
use ql_mef::nara_performance_receiving::*;
use ql_mef::performance_audio::PreparedPerformanceBinding;
use ql_mef::performance_source_context::*;
use ql_mef::physical_body::{PhysicalProvenance, PhysicalStanding, SpatialProjection};
use serde_json::{Value, json};
fn reference(s: &str) -> Reference {
    Reference {
        reference: s.into(),
        revision: "1".into(),
    }
}
fn return_ref(r: &Reference) -> ReturnReference {
    ReturnReference {
        reference: r.reference.clone(),
        revision: r.revision.clone(),
    }
}
fn returned(c: &ReceivingContext) -> ReturnContext {
    ReturnContext {
        kind: match c.kind {
            ContextKind::World => "world",
            ContextKind::Personal => "personal",
            ContextKind::Shared => "shared",
        }
        .into(),
        context: return_ref(&c.context),
        receiver: return_ref(&c.receiver),
        source_occasion: c.original_occasion.as_ref().map(return_ref),
        protected_state: c.protected_state.as_ref().map(return_ref),
        consent: c.consent.as_ref().map(return_ref),
        private: c.private,
        required_assets: vec![],
    }
}
fn world() -> ReceivingContext {
    ReceivingContext {
        kind: ContextKind::World,
        context: reference("expression:public-world/source-context"),
        receiver: reference("expression:public-world/receiver"),
        protected_state: None,
        consent: None,
        original_occasion: None,
        private: false,
    }
}
fn neutral(p: &PreparedPerformanceBinding, c: ReceivingContext) -> ReceivingPreparation<'_> {
    ReceivingPreparation {
        prepared: p,
        context: c,
        identity: None,
        current: None,
        original_occasion: None,
        calibration: None,
    }
}
#[test]
fn actual_reference_world_and_sparse_source_asset_receive_native_context() {
    let (current, config) = source::config(true);
    let mut owner =
        PerformanceOwner::prepare(&current, "expression:retained/current", config).unwrap();
    let ownership = NativePublicSourceOwnership::reference_source(&current).unwrap();
    let context = world();
    let definition = prepare_native_receiving(neutral(owner.binding(), context.clone())).unwrap();
    let witness = prepare_native_source_context(
        &owner.source_context_basis(&current).unwrap(),
        &definition,
        neutral(owner.binding(), context.clone()),
        returned(&context),
        Some(&ownership),
    )
    .unwrap();
    witness
        .validate_current(
            &owner.source_context_basis(&current).unwrap(),
            &definition,
            neutral(owner.binding(), context.clone()),
            returned(&context),
            Some(&ownership),
        )
        .unwrap();
    owner.admit_source_context(&current, &witness).unwrap();
    let value = &owner.source_assets()["source_context"];
    assert_eq!(value["availability"], "available");
    assert_eq!(value["context"]["kind"], "world");
    assert_eq!(value["context"]["private"], false);
    assert!(value["original_occasion"].is_null());
    assert!(value["classifications"].as_array().unwrap().is_empty());
    // A second valid native receiving context cannot borrow the first witness.
    let mut other = context.clone();
    other.context = reference("expression:other-valid-world/context");
    let other_definition =
        prepare_native_receiving(neutral(owner.binding(), other.clone())).unwrap();
    assert!(
        witness
            .validate_current(
                &owner.source_context_basis(&current).unwrap(),
                &other_definition,
                neutral(owner.binding(), other.clone()),
                returned(&other),
                Some(&ownership)
            )
            .is_err()
    );
    // Full different source, same architectural fields, is refused by owner CAS.
    let mut changed = current.input.clone();
    changed.source_receipts.push(json!({"schema":"opaque:original-native-receipt","private_context":"exact controlled private payload"}));
    let changed = changed.compose().unwrap();
    assert!(owner.source_context_basis(&changed).is_err());
    assert!(NativePublicSourceOwnership::reference_source(&changed).is_err());
}
struct Personal {
    identity: Value,
    current: Value,
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
        let current =
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
            identity,
            current,
            occasion,
            calibration,
            context,
        }
    }
    fn input<'a>(&'a self, p: &'a PreparedPerformanceBinding) -> ReceivingPreparation<'a> {
        ReceivingPreparation {
            prepared: p,
            context: self.context.clone(),
            identity: Some(&self.identity),
            current: Some(&self.current),
            original_occasion: Some(&self.occasion),
            calibration: Some(&self.calibration),
        }
    }
}
#[test]
fn personal_shared_opaque_receipts_remain_private_and_exact_original_occasion() {
    let (native, config) = source::config(true);
    let mut input = native.input.clone();
    input.source_receipts.push(json!({"schema":"controlled:opaque-original","original_personal":"controlled retained private fact"}));
    let current = input.compose().unwrap();
    let mut owner =
        PerformanceOwner::prepare(&current, "expression:retained/current", config).unwrap();
    let mut personal = Personal::new(owner.binding());
    let definition = prepare_native_receiving(personal.input(owner.binding())).unwrap();
    let witness = prepare_native_source_context(
        &owner.source_context_basis(&current).unwrap(),
        &definition,
        personal.input(owner.binding()),
        returned(&personal.context),
        None,
    )
    .unwrap();
    owner.admit_source_context(&current, &witness).unwrap();
    let snap = witness.snapshot().unwrap();
    assert_eq!(snap["context"]["kind"], "personal");
    assert_eq!(snap["classifications"][0]["private"], true);
    assert_eq!(
        snap["original_occasion"],
        serde_json::to_value(&personal.occasion).unwrap()
    );
    // Another independently valid native occasion cannot reuse this witness.
    personal.occasion.occasion_ref = "controlled:source-context/other-valid-occasion".into();
    personal.context.original_occasion = Some(reference(&personal.occasion.occasion_ref));
    let other = prepare_native_receiving(personal.input(owner.binding())).unwrap();
    assert!(
        witness
            .validate_current(
                &owner.source_context_basis(&current).unwrap(),
                &other,
                personal.input(owner.binding()),
                returned(&personal.context),
                None
            )
            .is_err()
    );
    personal.context.kind = ContextKind::Shared;
    personal.context.consent = Some(reference("controlled:source-context/shared-consent"));
    let shared = prepare_native_receiving(personal.input(owner.binding())).unwrap();
    let shared_witness = prepare_native_source_context(
        &owner.source_context_basis(&current).unwrap(),
        &shared,
        personal.input(owner.binding()),
        returned(&personal.context),
        None,
    )
    .unwrap();
    assert_eq!(
        shared_witness.snapshot().unwrap()["context"]["kind"],
        "shared"
    );
    personal.context.consent = Some(reference("controlled:source-context/other-valid-consent"));
    let changed = prepare_native_receiving(personal.input(owner.binding())).unwrap();
    assert!(
        shared_witness
            .validate_current(
                &owner.source_context_basis(&current).unwrap(),
                &changed,
                personal.input(owner.binding()),
                returned(&personal.context),
                None
            )
            .is_err()
    );
    // A lawful World N definition cannot publicly classify the opaque originals.
    let world_context = world();
    let public_definition =
        prepare_native_receiving(neutral(owner.binding(), world_context.clone())).unwrap();
    assert!(
        prepare_native_source_context(
            &owner.source_context_basis(&current).unwrap(),
            &public_definition,
            neutral(owner.binding(), world_context.clone()),
            returned(&world_context),
            None
        )
        .is_err()
    );
}

#[test]
fn actual_existing_world_constructor_owns_only_its_exact_native_sky_receipt() {
    let sky: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let request = || {
        serde_json::from_value::<ql_mef::scene::WorldRequest>(json!({"schema":ql_mef::scene::WORLD_REQUEST,"instance_ref":"expression:controlled-source-context-world","event_ref":sky["snapshot_ref"],"subject_ref":"person:controlled-world-a","texture":[64,64],"units_per_metre":1.0,"sky":sky,"start":{"tick12":3,"cycle":7,"aperture":9}})).unwrap()
    };
    let actual = ql_mef::scene::world(request()).unwrap();
    let original: ql_mef::continuous::coupled::CoupledInput =
        serde_json::from_value(actual["event"].clone()).unwrap();
    let current = original.compose().unwrap();
    let (_, mut config) = source::config(false);
    config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    let mut owner = PerformanceOwner::prepare(
        &current,
        "expression:controlled-source-context-world",
        config,
    )
    .unwrap();
    let ownership = NativePublicSourceOwnership::world_source(request()).unwrap();
    let context = world();
    let definition = prepare_native_receiving(neutral(owner.binding(), context.clone())).unwrap();
    let witness = prepare_native_source_context(
        &owner.source_context_basis(&current).unwrap(),
        &definition,
        neutral(owner.binding(), context.clone()),
        returned(&context),
        Some(&ownership),
    )
    .unwrap();
    owner.admit_source_context(&current, &witness).unwrap();
    let value = &owner.source_assets()["source_context"];
    assert_eq!(value["classifications"].as_array().unwrap().len(), 1);
    assert_eq!(value["classifications"][0]["private"], false);
    assert_eq!(
        value["classifications"][0]["source_owner_ref"],
        actual["native_owner_sources"]["constructor"]["ref"]
    );
    let mut other_request = request();
    other_request.subject_ref = "person:controlled-world-b".into();
    other_request.instance_ref = "expression:another-valid-world".into();
    let other = NativePublicSourceOwnership::world_source(other_request).unwrap();
    assert!(
        witness
            .validate_current(
                &owner.source_context_basis(&current).unwrap(),
                &definition,
                neutral(owner.binding(), context.clone()),
                returned(&context),
                Some(&other)
            )
            .is_err()
    );
}
