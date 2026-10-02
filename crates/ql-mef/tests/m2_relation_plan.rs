//! Real native producer/coupled paths and qualified-source fault injection.
//! No mocked provider, worker, oscillator or engine replaces a production call.
use ql_mef::continuous::coupled::{ConditionFrequencyBinding, CoupledInput, REQUEST_V2};
use ql_mef::continuous::scene_field;
use ql_mef::m_tree::{MRegistry, NATIVE_M_MANIFEST, native_m_registry};
use ql_mef::m2::{self, Reading72, Register72};
use ql_mef::m2_condition::{CorrespondenceRole, TuningPolicy, correspondence_field};
use ql_mef::m2_engine::{M2Request, MaterialFibre};
use ql_mef::m2_relation_plan::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-relation-plan-input-v1.json"
    ))
    .unwrap()
}
fn request() -> M2Request {
    serde_json::from_value(fixture()["nativeRequest"].clone()).unwrap()
}
fn context() -> M2RelationPlanContext {
    serde_json::from_value(fixture()["relationContext"].clone()).unwrap()
}
fn source_json() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-relation-source-v1.json"
    ))
    .unwrap()
}
fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

#[test]
fn full_native_source_and_exact_rast_properties_survive_the_producer() {
    let source = source_field();
    assert_eq!(source.node_count(), 598);
    assert_eq!(source.relation_count(), 4490);
    assert_eq!(source.prime_source_nodes.len(), 1);
    assert_eq!(source.prime_source_nodes[0].map_coordinate, "M2'");
    assert_eq!(
        source.property("#2-4.3-1-0", "c_2_tonic_note"),
        Some(&json!("C"))
    );
    assert_eq!(
        source.property("#2-4.3-1-0", "c_2_dominant_note"),
        Some(&json!("G"))
    );
    assert_eq!(
        source.property("#2-4.3-1-0", "c_2_ajnas"),
        Some(&json!(
            "Primary: Rast pentachord on C, Secondary: Rast tetrachord on G"
        ))
    );
    assert_eq!(
        source.property("#2-5-5", "m_2_5_interval_from_root"),
        Some(&json!("Perfect Fifth (3:2)"))
    );
    let prepared = M2RelationPlan::compile(&request(), context(), source).unwrap();
    assert_eq!(prepared.plan.schema, M2_RELATION_PLAN_SCHEMA);
    assert_eq!(prepared.plan.owner, M2_RELATION_PLAN_OWNER);
    for (table, index) in [("mantra", 50), ("planet", 7)] {
        let receipt = prepared
            .plan
            .source_receipts
            .retained_descriptors
            .iter()
            .find(|d| d.table == table && d.index == index)
            .unwrap();
        assert!(receipt.exact_coordinate.is_none());
        assert_eq!(
            receipt.structural_scope,
            m2::catalogue().table(table).unwrap().scope()
        );
        assert_eq!(receipt.retained_sources, m2::catalogue().sources());
        assert_eq!(
            receipt.retained_values_sha256,
            digest(
                &serde_json::to_string(m2::catalogue().table(table).unwrap().row(index).unwrap())
                    .unwrap()
            )
        );
        assert_eq!(
            prepared
                .frame
                .selected_descriptors
                .iter()
                .find(|d| d.table == table && d.index == index)
                .unwrap()
                .exact_coordinate,
            None
        );
    }
    assert_eq!(
        prepared
            .frame
            .condition
            .as_ref()
            .unwrap()
            .source_path
            .as_ref()
            .unwrap()
            .maqam_coordinate,
        "#2-4.3-1-0"
    );
    let operations = &prepared.plan.source_receipts.operations;
    for family in [
        M2PrimeFamily::Ground,
        M2PrimeFamily::Vimarsha,
        M2PrimeFamily::Density,
        M2PrimeFamily::Decan,
        M2PrimeFamily::Arena,
        M2PrimeFamily::Situated,
    ] {
        assert!(operations.iter().any(|o| o.family == family));
    }
    for operation in operations {
        assert_eq!(
            operation.native_id,
            native_m_registry()
                .resolve(&operation.source_coordinate)
                .unwrap()
                .id
        );
        assert_eq!(operation.authored_source_ref, M2_DOMAIN_SPEC_REF);
    }
    assert!(
        prepared
            .plan
            .source_receipts
            .properties
            .iter()
            .any(|p| p.coordinate == "#2-4.3-1-0"
                && p.property == "c_2_ajnas"
                && p.literal.as_ref().is_some_and(Value::is_string))
    );
}

#[test]
fn all_72_address_views_match_existing_native_registers_and_do_not_become_84() {
    for index in 0..72 {
        let address = M2Address72Views::decode(index).unwrap();
        for (register, axes) in [
            (
                Register72::Mef,
                [address.mef_lens12, address.mef_position6, 0, 0],
            ),
            (
                Register72::Tattva,
                [address.tattva36, address.tattva_phase2, 0, 0],
            ),
            (
                Register72::Decan,
                [
                    address.decan_element4,
                    address.decan_sign3,
                    address.decan_index3,
                    address.decan_face2,
                ],
            ),
            (
                Register72::Shem,
                [address.shem_choir8, address.shem_position9, 0, 0],
            ),
        ] {
            assert_eq!(Reading72::new(register, index as u8).unwrap().axes(), axes);
            assert_eq!(
                Reading72::from_axes(register, axes[0], axes[1], axes[2], axes[3])
                    .unwrap()
                    .index(),
                index as u8
            );
        }
        assert_eq!(address.maqam_row72, index as u8);
        assert_eq!(
            m2::scalar_compress(index as u8).unwrap(),
            (index * 8 / 9) as u8
        );
    }
    assert!(M2Address72Views::decode(72).is_err());
    assert!(M2Address72Views::decode(83).is_err());
}

#[test]
fn actual_vimarsha_eight_audible_and_four_nodal_values_are_copied_with_prime_phase() {
    let mut input = request();
    input.condition.as_mut().unwrap().active_mef_condition = 36;
    input.mef_conditions = vec![36];
    input.vimarsha.as_mut().unwrap().lens = 6;
    let prepared = M2RelationPlan::compile(&input, context(), source_field()).unwrap();
    let reading = &prepared.frame.vimarsha.as_ref().unwrap().reading;
    assert_eq!(
        prepared.plan.execution.audio_octet_hz,
        reading.audio_octet_hz
    );
    assert_eq!(prepared.plan.execution.nodal_quartet, reading.nodal_quartet);
    assert!(
        prepared
            .plan
            .source_receipts
            .operations
            .iter()
            .all(|o| o.mef_phase == M2MefPhase::Prime)
    );
    assert_eq!(prepared.plan.address72.tattva_phase2, 0);
    assert_eq!(prepared.plan.det.epogdoon_index64, 32);
    assert_eq!(
        prepared.plan.det.native_m2_to_m3_mask,
        m2::legacy_det(&[36]).unwrap()
    );
    input.vimarsha.as_mut().unwrap().lens = 0;
    assert!(M2RelationPlan::compile(&input, context(), source_field()).is_err());
}

#[test]
fn every_current_source_path_keeps_missing_authentic_pitches_detectable() {
    let mut count = 0;
    for rule in &correspondence_field().rules {
        let mut input = request();
        input.condition.as_mut().unwrap().maqam_index = rule.maqam_index;
        input.condition.as_mut().unwrap().role = rule.role;
        let mut ctx = context();
        ctx.requested_maqam_index = Some(rule.maqam_index);
        ctx.requested_role = Some(rule.role);
        ctx.musical.maqam_ref = Some(rule.maqam_coordinate.clone());
        let prepared = M2RelationPlan::compile(&input, ctx, source_field()).unwrap();
        assert!(prepared.plan.execution.intended_tuning_hz.is_none());
        assert!(
            prepared
                .frame
                .condition
                .as_ref()
                .unwrap()
                .musical
                .pitches_hz
                .is_empty()
        );
        assert!(
            prepared
                .plan
                .execution
                .tuning_standing
                .starts_with("unavailable")
        );
        assert!(
            prepared
                .frame
                .condition
                .as_ref()
                .unwrap()
                .gaps
                .iter()
                .any(|g| g.contains("unsupported"))
        );
        count += 1;
    }
    assert_eq!(count, 127);
    let mut input = request();
    input.condition.as_mut().unwrap().tuning = TuningPolicy::Retained24Tet;
    let retained = M2RelationPlan::compile(&input, context(), source_field()).unwrap();
    assert_eq!(
        retained
            .plan
            .execution
            .intended_tuning_hz
            .as_ref()
            .unwrap()
            .len(),
        8
    );
    assert!(
        retained
            .plan
            .execution
            .tuning_standing
            .contains("retained-C-24tet")
    );
}

#[test]
fn native_row_identity_cannot_relabel_scene_default_as_rast() {
    let mut ctx = context();
    ctx.requested_maqam_index = Some(3);
    assert!(
        M2RelationPlan::compile(&request(), ctx, source_field())
            .unwrap_err()
            .contains("musical maqam reference disagrees")
    );
}

#[test]
fn exact_shem_mantra_name_routes_reach_the_existing_descriptor_consumer_and_replay() {
    let input = request();
    let mut ctx = context();
    let station = m2::catalogue()
        .table("station")
        .unwrap()
        .binding(1)
        .unwrap()
        .to_owned();
    ctx.musical.mantra_or_name_route_refs.push(station.clone());
    let prepared = M2RelationPlan::compile(&input, ctx, source_field()).unwrap();
    assert!(
        prepared
            .request
            .selections
            .iter()
            .any(|s| s.table == "station" && s.index == 1)
    );
    assert!(prepared.frame.selected_descriptors.iter().any(|d| {
        serde_json::to_value(d)
            .unwrap()
            .to_string()
            .contains(&station)
    }));
    assert!(
        prepared
            .plan
            .source_receipts
            .operations
            .iter()
            .any(|o| o.source_coordinate == station)
    );
    let replay = prepared
        .plan
        .prepare_request(&input, source_field())
        .unwrap();
    assert_eq!(
        serde_json::to_value(replay).unwrap(),
        serde_json::to_value(prepared.request).unwrap()
    );
}

/// Perturb the actual qualified projection and its native source records
/// together. This is source-refresh fault injection, not a replacement registry
/// implementation; production constructors validate both unchanged ID topology
/// and the new payload receipts before dependency validation is exercised.
fn refreshed(
    mut projection: Value,
    edit_coordinate: Option<&str>,
    remove_relation: Option<&str>,
) -> M2SourceField {
    let mut native: Value = serde_json::from_str(NATIVE_M_MANIFEST).unwrap();
    if let Some(coordinate) = edit_coordinate {
        let node = projection["nodes"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|n| n["coordinate"] == coordinate)
            .unwrap();
        let mut properties: Value =
            serde_json::from_str(node["canonical_properties"].as_str().unwrap()).unwrap();
        properties["c_1_name"] = json!(format!(
            "{}; exact source-refresh perturbation",
            properties["c_1_name"].as_str().unwrap()
        ));
        let text = serde_json::to_string(&properties).unwrap();
        let payload = digest(&text);
        node["canonical_properties"] = json!(text);
        node["payload_sha256"] = json!(payload);
        let native_node = native["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|n| n["source_ref"] == coordinate)
            .unwrap();
        let record = native_node["records"][0].as_u64().unwrap() as usize;
        native["records"][record]["payload_sha256"] = node["payload_sha256"].clone();
    }
    if let Some(reference) = remove_relation {
        projection["relations"]
            .as_array_mut()
            .unwrap()
            .retain(|r| r["relation_ref"] != reference);
        native["relations"]
            .as_array_mut()
            .unwrap()
            .retain(|r| r["relation_ref"] != reference);
    }
    let revision = digest(&serde_json::to_string(&projection).unwrap());
    let registry_revision = digest(&(revision.clone() + ":native-source-refresh"));
    projection["source_revision"] = json!(revision);
    projection["source_sha256"] = projection["source_revision"].clone();
    projection["registry_revision"] = json!(registry_revision);
    native["source_revision"] = projection["source_revision"].clone();
    native["registry_revision"] = projection["registry_revision"].clone();
    native["files"][0]["sha256"] = projection["source_sha256"].clone();
    let registry = MRegistry::from_json(&native.to_string()).unwrap();
    M2SourceField::from_json_with_registry(&projection.to_string(), &registry).unwrap()
}

#[test]
fn exact_lost_edge_invalidates_consuming_plan_while_unrelated_source_change_does_not() {
    let plan = M2RelationPlan::compile(&request(), context(), source_field())
        .unwrap()
        .plan;
    let consumed = plan
        .source_receipts
        .relation_sets
        .iter()
        .find(|s| s.from_coordinate == "#2-4.3-1-0" && s.kind == "TONIC_PLANETARY_RESONANCE")
        .unwrap();
    let removed = refreshed(
        source_json(),
        None,
        Some(&consumed.relations[0].relation_ref),
    );
    assert!(
        plan.source_receipts
            .validate_against(&removed)
            .unwrap_err()
            .contains("typed edge set changed")
    );
    let changed_leaf = refreshed(source_json(), Some("#2-4.3-1-0"), None);
    assert!(
        plan.source_receipts
            .validate_against(&changed_leaf)
            .unwrap_err()
            .contains("source property changed")
    );
    let unrelated = source_json()["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| {
            !plan
                .source_receipts
                .operations
                .iter()
                .any(|o| o.source_coordinate == n["coordinate"].as_str().unwrap())
        })
        .unwrap()["coordinate"]
        .as_str()
        .unwrap()
        .to_owned();
    let independent = refreshed(source_json(), Some(&unrelated), None);
    plan.source_receipts.validate_against(&independent).unwrap();
    let mut tampered = plan.source_receipts.clone();
    tampered
        .retained_descriptors
        .iter_mut()
        .find(|d| d.table == "planet" && d.index == 7)
        .unwrap()
        .exact_coordinate = Some("#2-5-0/1".into());
    assert!(
        tampered
            .validate_against(source_field())
            .unwrap_err()
            .contains("retained descriptor")
    );
}

#[test]
fn truncated_or_tampered_projection_never_silently_creates_a_source_path() {
    let mut source = source_json();
    source["relations"].as_array_mut().unwrap().pop();
    assert!(
        M2SourceField::from_json(&source.to_string())
            .unwrap_err()
            .contains("lost a typed edge")
    );
    let mut source = source_json();
    source["nodes"][0]["canonical_properties"] = json!("{\"c_1_name\":\"unqualified edit\"}");
    assert!(M2SourceField::from_json(&source.to_string()).is_err());
    let mut ctx = context();
    ctx.source_coordinates.push("#2-4.4".into());
    assert!(
        M2RelationPlan::compile(&request(), ctx, source_field())
            .unwrap_err()
            .contains("unavailable source descendant")
    );
}

/// Declared deterministic boundary input exercises the production unequal-hour
/// calculation. It is labelled supplied test input, never live ephemeris.
fn solar_window(input: &M2Request) -> M2ObserverSolarWindow {
    M2ObserverSolarWindow {
        stamp: input.stamp.clone(),
        observer_ref: "test-input:declared-Earth-observer".into(),
        observer_coordinate: "#2-5-0/1-0".into(),
        latitude_degrees: 51.5,
        longitude_degrees: 0.0,
        sunrise_unix_ms: 3 * 86_400_000 + 7 * 3_600_000,
        sunset_unix_ms: 3 * 86_400_000 + 17 * 3_600_000 + 1,
        next_sunrise_unix_ms: 4 * 86_400_000 + 7 * 3_600_000,
        civil_utc_offset_minutes: 0,
        boundary_provider_ref: "test-input:declared-solar-boundaries".into(),
        boundary_source_revision: "test-input:no-live-ephemeris-claim".into(),
    }
}
#[test]
fn observer_unequal_hours_use_actual_source_cycle_and_boundary_candidates() {
    let mut input = request();
    let window = solar_window(&input);
    let mut ctx = context();
    ctx.observer_solar_window = Some(window.clone());
    ctx.requested_maqam_index = None;
    ctx.requested_role = None;
    ctx.musical.maqam_ref = None;
    input.at_unix_ms = window.sunrise_unix_ms;
    let day = M2RelationPlan::compile(&input, ctx.clone(), source_field())
        .unwrap()
        .plan;
    let hour = day.situated.planetary_hour.as_ref().unwrap();
    assert_eq!(hour.sunrise_weekday, 0);
    assert_eq!(hour.hour24, 0);
    assert_eq!(hour.planet_coordinate, "#2-5-0/1");
    assert!(hour.daylight);
    assert_eq!(hour.begins_unix_ms, window.sunrise_unix_ms);
    assert!(
        day.situated
            .maqam_candidates
            .iter()
            .all(|c| c.planet_coordinate == hour.planet_coordinate)
    );
    input.at_unix_ms = hour.ends_unix_ms;
    let next = M2RelationPlan::compile(&input, ctx.clone(), source_field())
        .unwrap()
        .plan;
    assert_eq!(next.situated.planetary_hour.as_ref().unwrap().hour24, 1);
    assert_eq!(
        next.situated
            .planetary_hour
            .as_ref()
            .unwrap()
            .planet_coordinate,
        "#2-5-2"
    );
    input.at_unix_ms = window.sunset_unix_ms;
    let night = M2RelationPlan::compile(&input, ctx.clone(), source_field())
        .unwrap()
        .plan;
    assert_eq!(night.situated.planetary_hour.as_ref().unwrap().hour24, 12);
    assert!(!night.situated.planetary_hour.as_ref().unwrap().daylight);
    assert_eq!(
        night
            .situated
            .planetary_hour
            .as_ref()
            .unwrap()
            .planet_coordinate,
        "#2-5-6"
    );
    input.at_unix_ms = window.next_sunrise_unix_ms;
    assert!(M2RelationPlan::compile(&input, ctx.clone(), source_field()).is_err());
    input.at_unix_ms = window.sunrise_unix_ms;
    ctx.observer_solar_window
        .as_mut()
        .unwrap()
        .stamp
        .identity
        .profile_generation += 1;
    assert!(M2RelationPlan::compile(&input, ctx, source_field()).is_err());
}

fn completed_event() -> (CoupledInput, M2Request) {
    let mut event: CoupledInput = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/scene-default-event-v2.json"
    ))
    .unwrap();
    event.m2.condition.as_mut().unwrap().maqam_index = 9;
    let (input, basis) = scene_field::complete(
        &event,
        &scene_field::default_material(),
        MaterialFibre::Earth,
    )
    .unwrap();
    (input, basis.m2_input)
}
#[test]
fn actual_scene_provider_coupled_consumer_receives_source_damping_and_preserves_mode_identity() {
    let (mut event, input) = completed_event();
    let before = input.resonator.as_ref().unwrap().clone();
    let mode = &before.modes[0];
    let mut ctx = context();
    ctx.material_writes.push(M2MaterialWrite {
        source_coordinate: "#2-2-1".into(),
        source_property: "c_1_name".into(),
        interpretation_policy_ref: "test-input:declared-tattva-damping-tunable-policy".into(),
        target: M2MaterialTarget::ModeDamping {
            mode_ref: mode.mode_ref.clone(),
        },
        expected_value: mode.damping_per_second,
        value: 0.7,
        minimum: 0.0,
        maximum: 1.0,
    });
    let prepared = M2RelationPlan::compile(&input, ctx, source_field()).unwrap();
    event.m2 = prepared.request.clone();
    let consumed = event.compose().unwrap();
    let modes = &consumed.m2_input.resonator.as_ref().unwrap().modes;
    assert_eq!(modes[0].damping_per_second, 0.7);
    for (old, new) in before.modes.iter().zip(modes) {
        assert_eq!(old.mode_ref, new.mode_ref);
        assert_eq!(old.frequency_hz, new.frequency_hz);
        assert_eq!(old.nodal_state_ref, new.nodal_state_ref);
        assert_eq!(old.antinodal_state_ref, new.antinodal_state_ref);
        assert_eq!(old.carrier_weights.len(), new.carrier_weights.len());
    }
    assert_eq!(
        consumed.m2["resonator"]["modes"][0]["damping_per_second"],
        0.7
    );
    let replay = prepared
        .plan
        .prepare_request(&input, source_field())
        .unwrap();
    assert_eq!(
        serde_json::to_value(replay).unwrap(),
        serde_json::to_value(prepared.request).unwrap()
    );
    // A repeat of complete would rebuild provider material and erase this write.
    // Assert the actual receiving input, so serial integration cannot stop at a
    // plan receipt while forgetting to hand it to the native coupled owner.
    assert_ne!(
        consumed.m2_input.resonator.as_ref().unwrap().modes[0].damping_per_second,
        before.modes[0].damping_per_second
    );
}

#[test]
fn actual_condition_pitch_consumer_refuses_source_unavailable_tuning() {
    let (mut event, input) = completed_event();
    let prepared = M2RelationPlan::compile(&input, context(), source_field()).unwrap();
    let mode_ref = prepared.request.resonator.as_ref().unwrap().modes[0]
        .mode_ref
        .clone();
    event.m2 = prepared.request;
    event.schema = REQUEST_V2.into();
    event.condition_frequency_bindings = vec![ConditionFrequencyBinding {
        mode_ref,
        pitch_index: 0,
    }];
    assert!(
        event
            .compose()
            .unwrap_err()
            .contains("condition pitch unavailable")
    );
}

#[test]
fn stale_basis_late_event_missing_target_and_wrong_units_are_refused_atomically() {
    let (mut event, input) = completed_event();
    let prepared = M2RelationPlan::compile(&input, context(), source_field()).unwrap();
    let mut stale = input.clone();
    stale.stamp.identity.profile_generation += 1;
    assert!(
        prepared
            .plan
            .prepare_request(&stale, source_field())
            .is_err()
    );
    let mut changed_pose = input.clone();
    changed_pose.vimarsha.as_mut().unwrap().pose_ordinal += 1;
    assert!(
        prepared
            .plan
            .prepare_request(&changed_pose, source_field())
            .is_err()
    );
    let original = serde_json::to_value(&input).unwrap();
    for target in [
        M2MaterialTarget::ModeDamping {
            mode_ref: "unavailable:mode".into(),
        },
        M2MaterialTarget::Parameter {
            name: "damping_per_second".into(),
            unit: "Hz".into(),
        },
    ] {
        let mut ctx = context();
        ctx.material_writes.push(M2MaterialWrite {
            source_coordinate: "#2-2-1".into(),
            source_property: "c_1_name".into(),
            interpretation_policy_ref: "test-input:declared-material-policy".into(),
            target,
            expected_value: 0.35,
            value: 0.7,
            minimum: 0.0,
            maximum: 1.0,
        });
        assert!(M2RelationPlan::compile(&input, ctx, source_field()).is_err());
        assert_eq!(serde_json::to_value(&input).unwrap(), original);
    }
    event.m2.at_unix_ms += 1;
    assert!(
        prepared
            .plan
            .prepare_request(&event.m2, source_field())
            .is_err()
    );
    assert_eq!(
        prepared.plan.execution.condition_input.role,
        CorrespondenceRole::Tonic
    );
}
