// Real native N/A/P producer material for the paired C++ transport test.
// Controlled angular inputs have arithmetic standing and no astronomy verdict.
mod support {
    include!("retained_performance.rs");
}
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::nara::{
    EventBasisRefs, SourceRevision, current, domain::ProtectedRef, intake::IdentityProfile,
    replay::NaraOccasion,
};
use ql_mef::nara_performance_receiving::{
    CentreCalibration, ContextKind, ReceivingCalibration, ReceivingContext, ReceivingPreparation,
    Reference, prepare_native_receiving,
};
use ql_mef::performance_audio::{PreparedPerformanceBinding, prepare_native_performance};
use ql_mef::performance_receiving_admission::prepare_native_receiving_admission;
use ql_mef::physical_body::{PhysicalProvenance, PhysicalStanding, SpatialProjection};
use serde_json::{Value, json};
fn reference(value: &str) -> Reference {
    Reference {
        reference: value.into(),
        revision: "1".into(),
    }
}
fn for_prepared(p: &PreparedPerformanceBinding, native_cursor: u64) -> Result<Value, String> {
    for_prepared_retained(p, native_cursor, None)
}
fn for_prepared_retained(
    p: &PreparedPerformanceBinding,
    native_cursor: u64,
    retained: Option<&mut Value>,
) -> Result<Value, String> {
    p.validate_native_consumers(p.native_basis(), p.physical_body())?;
    let profile: IdentityProfile = serde_json::from_value(json!({
        "schema":"ql.nara-identity-profile/v1", "person_ref":p.physical_body().subject_ref(),
        "nara_ref":"reference:force-routes/nara", "name":"Controlled source operator",
        "encoding_policy":ql_mef::nara::identity_encoding::EncodingPolicy::default(),
        "composition_policy":"draft-core-birthdate-decanic-40-60-v1",
        "birth":{"date":"1990-06-15","time":null,"precision":"unknown","uncertainty_minutes":null,"fold":null,"place":null},
        "jungian":null,"gene_keys":null,"human_design":null,"quintessence":null
    })).map_err(|e| e.to_string())?;
    let natal = json!({"schema":"ql.nara-natal/v1", "request":profile.natal_request()?,
        "status":"partial", "reason":"controlled-angular-input; astronomical-provider-unqualified", "chart":null,
        "sky":{"schema":"ql.sky-snapshot/v1","snapshot_ref":"reference:force-routes/natal-input",
            "bodies":(["Sun","Moon","Mercury","Venus","Mars","Jupiter","Saturn","Uranus","Neptune","Pluto"].iter().enumerate().map(|(i,name)|
                json!({"native_planet_id":i,"body":name,"longitude_degrees":30.0*i as f64})).collect::<Vec<_>>())}});
    let identity = profile.inspect(Some(&natal))?;
    let event = EventBasisRefs::from_basis(p.native_basis())?;
    let occasion = NaraOccasion {
        occasion_ref: "reference:force-routes/original-occasion".into(),
        subject_id: p.physical_body().subject_ref().into(),
        event,
        personal_reception_generation: 1,
        identity_revision: identity["input_revision"]
            .as_str()
            .ok_or("identity revision absent")?
            .into(),
        day_ref: "central:day:2026-10-02".into(),
        now_ref: "central:now:force-routes-controlled-native-test".into(),
        occurrence_at_unix_ms: 100,
        receipt_at_unix_ms: 900,
        protected_state_ref: ProtectedRef {
            ref_id: "reference:force-routes/protected-state".into(),
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
    };
    let context = ReceivingContext {
        kind: ContextKind::Personal,
        context: reference("reference:force-routes/personal-context"),
        receiver: reference("reference:force-routes/receiver"),
        protected_state: Some(reference(&occasion.protected_state_ref.ref_id)),
        consent: None,
        original_occasion: Some(reference(&occasion.occasion_ref)),
        private: true,
    };
    let mut sky = identity["natal"]["sky"].clone();
    sky["snapshot_ref"] = json!("reference:force-routes/dated-operator-input");
    sky["source_binding"] =
        json!({"registry_revision":ql_mef::m2::catalogue().registry_revision()});
    sky["request"] = json!({"schema":"ql.sky-request/v1","epoch":"reference:controlled-epoch"});
    sky["receipt_unix_ms"] = json!(900);
    sky["provider"] =
        json!({"adapter_sha256":"reference:unqualified-controlled-input; no astronomy verdict"});
    let current = current::personal_current(&identity, &current::transit(Some(&sky))?)?;
    let request = p.physical_body().request();
    let calibration = ReceivingCalibration {
        provenance: PhysicalProvenance {
            reference: "reference:force-routes/seven-metric-maps".into(),
            revision: "1".into(),
            source_ref: "reference:explicit-instrument-force-calibration".into(),
            standing: PhysicalStanding::Reference,
        },
        preparation: Reference {
            reference: request.preparation_ref.clone(),
            revision: request.body_revision.to_string(),
        },
        state: Reference {
            reference: request.state_ref.clone(),
            revision: request.body_revision.to_string(),
        },
        source_force_newtons: 0.1,
        centres: std::array::from_fn(|ordinal| {
            let free = (ordinal + 1) as f64 / 8.0;
            CentreCalibration {
                ordinal: ordinal as u8,
                projection: SpatialProjection {
                    axis: [1.0, 0.0, 0.0],
                    node_weights: vec![1.0 - free, free],
                },
            }
        }),
    };
    let preparation = || ReceivingPreparation {
        prepared: p,
        context: context.clone(),
        identity: Some(&identity),
        current: Some(&current),
        original_occasion: Some(&occasion),
        calibration: Some(&calibration),
    };
    let definition = prepare_native_receiving(preparation())?;
    definition.validate_source_registry(native_current_m_registry())?;
    definition.validate_sources(preparation())?;
    let operation = definition.native_operation(native_cursor)?;
    let replayed = prepare_native_receiving(preparation())?;
    let current_operation = replayed.native_operation(native_cursor)?;
    let det = p.determination();
    let source_basis = json!({
        "event_ref":p.physical_body().event_ref(), "subject_ref":p.physical_body().subject_ref(),
        "registry_revision":native_current_m_registry().manifest().registry_revision,
        "source_revision":native_current_m_registry().manifest().source_revision,
        "definition_ref":definition.content_digest(), "source_instance_ref":operation.source_instance,
        "determination_ref":det["native_receipt_ref"], "m2_writer_coordinate":det["m2_writer"],
        "m2_pratibimba":det["m2_face"] == 1,
        "m1_revision":det["identity"]["m1_revision"],"m2_generation":det["identity"]["m2_generation"],
        "m3_generation":p.physical_body().source_generation().to_string()
    });
    let program_refs: Vec<_> = operation
        .sources
        .iter()
        .map(|s| {
            format!(
                "reference:force-routes/native-program/{}",
                s.native_planet_id
            )
        })
        .collect();
    let world_preparation = || ReceivingPreparation {
        prepared: p,
        context: ReceivingContext {
            kind: ContextKind::World,
            context: reference("reference:force-routes/world-context"),
            receiver: reference("reference:force-routes/world-receiver"),
            protected_state: None,
            consent: None,
            original_occasion: None,
            private: false,
        },
        identity: None,
        current: None,
        original_occasion: None,
        calibration: None,
    };
    let world_definition = prepare_native_receiving(world_preparation())?;
    world_definition.validate_sources(world_preparation())?;
    let world_operation = world_definition.native_operation(native_cursor)?;
    let world_current_operation =
        prepare_native_receiving(world_preparation())?.native_operation(native_cursor)?;
    let mut world_source_basis = source_basis.clone();
    world_source_basis["definition_ref"] = json!(world_definition.content_digest());
    world_source_basis["source_instance_ref"] = json!(world_operation.source_instance);
    let native_admission = prepare_native_receiving_admission(
        &definition,
        preparation(),
        p.native_basis(),
        native_cursor,
    )?
    .snapshot()?;
    let current_native_admission = prepare_native_receiving_admission(
        &replayed,
        preparation(),
        p.native_basis(),
        native_cursor,
    )?
    .snapshot()?;
    let world_native_admission = prepare_native_receiving_admission(
        &world_definition,
        world_preparation(),
        p.native_basis(),
        native_cursor,
    )?
    .snapshot()?;
    let replayed_world = prepare_native_receiving(world_preparation())?;
    let world_current_native_admission = prepare_native_receiving_admission(
        &replayed_world,
        world_preparation(),
        p.native_basis(),
        native_cursor,
    )?
    .snapshot()?;
    // Complete physical catalog comes from the actual current native K owner.
    // Each repeated address is resolved independently through that SAME basis.
    let mut native_catalog = Vec::new();
    for cell in ql_mef::performance_management::native_janko_catalog(p, 6, 0, 0)? {
        let note = ql_mef::performance_management::resolve_performance_touch(
            p,
            ql_mef::performance_audio::KeyTouch {
                key: cell.key,
                register: cell.register_octave,
                member: u64::from(cell.row) * 6 + u64::from(cell.column) + 1,
                touch: u64::from(cell.row) * 6 + u64::from(cell.column) + 1,
                touch_ref: format!(
                    "native:retained-revision/catalog/{}/{}",
                    cell.row, cell.column
                ),
            },
        )?;
        native_catalog.push(json!({"row":cell.row,"column":cell.column,
            "key":cell.key,"pitch_class":cell.pitch_class,"register_octave":cell.register_octave,
            "label":cell.label,"available":true,"source_degree":null,"reason":null,
            "reduction_policy":p.targets().tuning_policy.provenance().policy_ref,
            "source_collection":"ql:canonical-twelve-key-field",
            "source_receipt":p.determination()["native_receipt_ref"],"native_target":note}));
    }
    let out: Value = json!({"preparation":p.physical_body(),"current_m3":p.native_basis().m3,
        "operation":operation,"current_operation":current_operation,"source_basis":source_basis,
        "program_refs":program_refs,"world_operation":world_operation,
        "world_current_operation":world_current_operation,"world_source_basis":world_source_basis,
        "native_admission":native_admission,"current_native_admission":current_native_admission,
        "world_native_admission":world_native_admission,"world_current_native_admission":world_current_native_admission,
        "performance_preparation":p,"native_catalog":native_catalog});
    if let Some(retained) = retained {
        use ql_mef::musical_performance_return::{
            ReturnContext, ReturnReference, bind_performance_return,
        };
        let reference = |value: &Reference| ReturnReference {
            reference: value.reference.clone(),
            revision: value.revision.clone(),
        };
        let returned = bind_performance_return(
            p,
            Some(occasion.clone()),
            ReturnContext {
                context: reference(&context.context),
                receiver: reference(&context.receiver),
                source_occasion: context.original_occasion.as_ref().map(reference),
                protected_state: context.protected_state.as_ref().map(reference),
                consent: context.consent.as_ref().map(reference),
                kind: "personal".into(),
                private: context.private,
                required_assets: vec![],
            },
            73,
        )?;
        let basis = returned.expression_basis()?;
        let pitches = returned.expression_pitches(0)?;
        if basis["prepared_body"] != out["preparation"]
            || basis["audio_determination"]
                != serde_json::to_value(p.determination()).map_err(|e| e.to_string())?
            || out["native_admission"]["native_basis"]
                != serde_json::to_value(p.native_basis()).map_err(|e| e.to_string())?
        {
            return Err(
                "same native route-management Return/preparation/source basis disconnected".into(),
            );
        }
        *retained = json!({"schema":"ql.retained-route-management-return/v1",
            "standing":"controlled-native-component; no current Scene/Act grant or astronomical verdict",
            "basis":basis,"pitches":pitches,"native_return":returned.snapshot()?,
            "native_preparation":p,"native_basis":p.native_basis(),
            "native_receiving":definition,"native_admission":out["native_admission"],
            "native_catalog":out["native_catalog"],
            "original_source_inputs":{"identity_profile":profile,"natal":natal,"identity":identity,
                "dated_sky":sky,"current":current,"original_occasion":occasion,"context":context,"calibration":calibration}});
    }
    Ok(out)
}

pub fn packet() -> Result<Value, String> {
    packet_retained(None)
}
pub fn packet_with_return() -> Result<(Value, Value), String> {
    let mut retained = Value::Null;
    let packet = packet_retained(Some(&mut retained))?;
    Ok((packet, retained))
}
fn packet_retained(retained: Option<&mut Value>) -> Result<Value, String> {
    let before = prepare_native_performance(support::preparation())?;
    let mut output = for_prepared_retained(&before, 0, retained)?;
    let mut request = support::preparation();
    request.physical.material.young_modulus_pa *= 4.0;
    request.physical.material.provenance.revision = "2".into();
    request.physical.body_revision = 2;
    request.physical.preparation_ref = "reference:performance/body-preparation2".into();
    let after = prepare_native_performance(request)?;
    output["after_material"] = for_prepared(&after, 512)?;
    let mut pose = support::preparation();
    pose.coupled.m3_commands[0]
        .operations
        .push(ql_mef::m3_state::M3Operation::AdvanceClock { steps: 1 });
    let after_pose = prepare_native_performance(pose)?;
    if after_pose.determination() != before.determination() {
        return Err("pose regression must retain exact original audio determination".into());
    }
    output["after_pose"] = for_prepared(&after_pose, 0)?;
    let mut overload = support::preparation();
    overload.touches = (0..48)
        .map(|i| ql_mef::performance_audio::KeyTouch {
            key: 0,
            register: 0,
            member: i + 1,
            touch: i + 1,
            touch_ref: format!("native:overload/independent-touch/{i}"),
        })
        .collect();
    output["overload"] = for_prepared(&prepare_native_performance(overload)?, 0)?;
    Ok(output)
}
