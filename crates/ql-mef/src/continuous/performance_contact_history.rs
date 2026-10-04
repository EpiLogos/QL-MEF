//! Full source coherence for original native contact records. Numerical values
//! and imported JSON never construct an occurrence witness or Scene grant.
//! Exact long-double root/ceil and immutable mode qualification stay in the
//! native solver/replay; this boundary independently verifies the genuine
//! PreparedSourceBody, authored geometry, every force value and source epoch.
use super::*;
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;

pub(crate) const CONTACT_REPLAY_REQUEST: &str = "ql.native-scene-contact-replay-request/v1";
const SOURCE: &str = "ql.native-scene-contact-source/v1";
const DEFINITION: &str = "ql.authored-body-local-plane-contact/v1";

fn keys(value: &Value, expected: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or("complete contact object absent")?;
    if object.len() != expected.len() || expected.iter().any(|key| !object.contains_key(*key)) {
        return Err("contact field set differs from original native transport".into());
    }
    Ok(())
}
fn number(value: &Value) -> Result<f64, String> {
    value
        .as_f64()
        .filter(|v| v.is_finite())
        .ok_or_else(|| "finite native contact number absent".into())
}
fn vector(value: &Value) -> Result<[f64; 3], String> {
    let a = value
        .as_array()
        .filter(|v| v.len() == 3)
        .ok_or("contact vector dimensions differ")?;
    Ok([number(&a[0])?, number(&a[1])?, number(&a[2])?])
}
fn exact(a: &Value, b: &Value) -> Result<(), String> {
    let equal = match (a, b) {
        (Value::Number(a), Value::Number(b)) if a.is_f64() || b.is_f64() => a
            .as_f64()
            .zip(b.as_f64())
            .is_some_and(|(a, b)| a.to_bits() == b.to_bits()),
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| exact(a, b).is_ok())
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, a)| b.get(k).is_some_and(|b| exact(a, b).is_ok()))
        }
        _ => a == b,
    };
    if equal {
        Ok(())
    } else {
        Err("full original native contact value differs".into())
    }
}
fn near(a: f64, b: f64, tolerance: f64) -> Result<(), String> {
    if a.is_finite() && b.is_finite() && (a - b).abs() <= tolerance {
        Ok(())
    } else {
        Err("authored contact geometry does not reproduce its native operand".into())
    }
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Reconstruct exactly the native ct::body_input schema from this private
/// immutable producer. No checkpoint/source JSON is selected as a constructor.
fn native_body(owner: &PerformanceOwner) -> Result<Value, String> {
    let body = owner.binding().physical_body();
    let request = body.request();
    let coordinate = serde_json::to_value(body).map_err(|e| e.to_string())?;
    let nodes: Vec<Value> = request.geometry.nodes.iter().map(|n|json!({
        "identity":n.identity.to_string(),"constituent":n.constituent,
        "rest_metres":n.rest_metres,"additional_mass_kg":n.additional_mass_kg,"fixed":n.fixed
    })).collect();
    let edges: Vec<Value> = request
        .geometry
        .edges
        .iter()
        .map(|e| {
            json!({
                "first":e.first.to_string(),"second":e.second.to_string(),
                "section_m2":e.section_m2,"prestress_newtons":e.prestress_newtons
            })
        })
        .collect();
    let geometry = &request.geometry.provenance;
    let material = &request.material;
    Ok(
        json!({"event_ref":body.event_ref(),"subject_ref":body.subject_ref(),
        "source_coordinate":coordinate["source_coordinate"]["source_ref"],"source_revision":body.source_revision(),
        "geometry_ref":geometry.reference,"geometry_revision":geometry.revision,"geometry_source_ref":geometry.source_ref,
        "geometry_standing":geometry.standing,"preparation_ref":request.preparation_ref,"state_ref":request.state_ref,
        "source_generation":body.source_generation().to_string(),"body_revision":request.body_revision.to_string(),
        "sample_rate":request.sample_rate.to_string(),"pickup_linear_per_metre":request.pickup_linear_per_metre,
        "max_force_newtons":request.max_force_newtons,"max_impulse_newton_seconds":request.max_impulse_newton_seconds,
        "max_displacement_metres":request.max_displacement_metres,"pratibimba":owner.config.physical_face==1,
        "family":match request.geometry.family {crate::physical_body::BodyFamily::AxialTruss=>"0",crate::physical_body::BodyFamily::PrestressedTensionNetwork=>"1"},
        "material":{"reference":material.provenance.reference,"revision":material.provenance.revision,
        "source_ref":material.provenance.source_ref,"standing":material.provenance.standing,
        "young_modulus_pa":material.young_modulus_pa,"density_kg_per_m3":material.density_kg_per_m3,
        "damping_alpha_per_second":material.damping_alpha_per_second,"damping_beta_seconds":material.damping_beta_seconds},
        "exciter":request.exciter,"pickup":request.pickup,"nodes":nodes,"edges":edges}),
    )
}

pub(crate) fn validate_current_scene_contact_record(
    owner: &PerformanceOwner,
    before_source_assets: &Value,
    record: &Value,
) -> Result<(), String> {
    exact(owner.source_assets(), before_source_assets)?;
    keys(
        record,
        &[
            "schema",
            "original_native_request_id",
            "scene_constructor",
            "authored_definition",
            "native_boundary",
            "occurrence",
            "original_gravity_input",
            "native_operands",
            "original_body",
            "force_newtons",
            "exciter_position_metres",
        ],
    )?;
    if record["schema"] != SOURCE {
        return Err("native Scene contact schema differs".into());
    }
    let ordinal = decimal(&record["original_native_request_id"])?;
    let occurrence = &record["occurrence"];
    keys(occurrence, &["constructor_lineage", "original_request_id"])?;
    let constructor = &record["scene_constructor"];
    keys(
        constructor,
        &[
            "schema",
            "expression_ref",
            "scene_ref",
            "instance_ref",
            "construction_generation",
            "generation_domain",
            "initial_document_revision",
            "initial_document_sha256",
            "document_revision",
            "document_sha256",
        ],
    )?;
    if ordinal == 0
        || occurrence["original_request_id"] != record["original_native_request_id"]
        || occurrence["constructor_lineage"] != constructor["instance_ref"]
        || constructor["schema"] != "oi.native-document-scene-constructor/v1"
        || constructor["generation_domain"] != "native-document-scene-construction"
        || constructor["construction_generation"]
            .as_u64()
            .is_none_or(|v| v == 0)
        || constructor["initial_document_revision"].as_u64().is_none()
        || constructor["document_revision"].as_u64().is_none()
    {
        return Err("contact original constructor/Manager occurrence differs".into());
    }
    for key in [
        "expression_ref",
        "scene_ref",
        "instance_ref",
        "initial_document_sha256",
        "document_sha256",
    ] {
        bounded(
            constructor[key]
                .as_str()
                .ok_or("contact constructor reference absent")?,
        )?;
    }
    let boundary = &record["native_boundary"];
    keys(
        boundary,
        &[
            "schema",
            "session_ref",
            "transport_epoch",
            "determination",
            "physical_body",
            "physical_eigenbasis",
            "native_trigger_sample",
            "queue_cursor",
            "queue_horizon",
            "accepted_sequence",
        ],
    )?;
    let trigger = decimal(&boundary["native_trigger_sample"])?;
    if boundary["schema"] != "ql.native-scene-contact-boundary/v1"
        || boundary["session_ref"] != owner.config.session_ref
        || decimal(&boundary["transport_epoch"])? == 0
        || decimal(&boundary["queue_cursor"])? > trigger
        || decimal(&boundary["queue_horizon"])? != trigger
    {
        return Err("contact lost original same-owner native boundary".into());
    }
    decimal(&boundary["accepted_sequence"])?;
    exact(&boundary["determination"], owner.binding().determination())?;
    let body = native_body(owner)?;
    exact(&body, &record["original_body"])?;
    exact(&body, &boundary["physical_body"])?;
    let native = owner.binding().physical_body();
    let request = native.request();
    let def = &record["authored_definition"];
    keys(
        def,
        &[
            "schema",
            "contact_ref",
            "particle_ref",
            "collider_ref",
            "policy_ref",
            "policy_revision",
            "standing",
            "particle_position_metres",
            "particle_velocity_metres_per_second",
            "plane_position_metres",
            "outward_normal",
            "gravity_metres_per_second_squared",
            "mass_kg",
            "restitution",
            "transfer_fraction",
            "minimum_impact_speed_metres_per_second",
            "duration_samples",
        ],
    )?;
    if def["schema"] != DEFINITION
        || !matches!(
            def["standing"].as_str(),
            Some("architecture-model" | "reference" | "tunable-model")
        )
    {
        return Err("authored native contact mechanism/standing differs".into());
    }
    for key in [
        "contact_ref",
        "particle_ref",
        "collider_ref",
        "policy_ref",
        "policy_revision",
        "standing",
    ] {
        let text = def[key]
            .as_str()
            .ok_or("contact original reference absent")?;
        bounded(text)?;
        if text.len() >= 192 {
            return Err("contact reference exceeds original native192 bound".into());
        }
    }
    let frames = def["duration_samples"]
        .as_u64()
        .filter(|v| *v > 0 && *v <= 512)
        .ok_or("contact duration differs")?;
    let normal = vector(&def["outward_normal"])?;
    let particle = vector(&def["particle_position_metres"])?;
    let plane = vector(&def["plane_position_metres"])?;
    let velocity = vector(&def["particle_velocity_metres_per_second"])?;
    let gravity = vector(&def["gravity_metres_per_second_squared"])?;
    let mut anchor = [0.; 3];
    for (node, weight) in request
        .geometry
        .nodes
        .iter()
        .zip(&request.exciter.node_weights)
    {
        for axis in 0..3 {
            anchor[axis] += weight * node.rest_metres[axis];
        }
    }
    let actual_anchor = vector(&record["exciter_position_metres"])?;
    let delta = std::array::from_fn(|axis| particle[axis] - anchor[axis]);
    let plane_delta = std::array::from_fn(|axis| anchor[axis] - plane[axis]);
    near(dot(normal, normal), 1., 1e-10)?;
    near(dot(plane_delta, normal), 0., 1e-10)?;
    let height = dot(delta, normal);
    let speed0 = dot(velocity, normal);
    let g = -dot(gravity, normal);
    for axis in 0..3 {
        if particle[axis].abs() > 1e6
            || plane[axis].abs() > 1e6
            || velocity[axis].abs() > 1e4
            || gravity[axis].abs() > 1e4
            || normal[axis].abs() > 1.
        {
            return Err("contact geometry exceeds original native bound".into());
        }
        near(actual_anchor[axis], anchor[axis], 1e-10)?;
        near(delta[axis], height * normal[axis], 1e-10)?;
        near(velocity[axis], speed0 * normal[axis], 1e-10)?;
        near(gravity[axis], -g * normal[axis], 1e-10)?;
    }
    let original = &record["original_gravity_input"];
    keys(
        original,
        &[
            "contact_ref",
            "particle_ref",
            "collider_ref",
            "route_ref",
            "source_ref",
            "policy_ref",
            "policy_revision",
            "standing",
            "plane_position_metres",
            "normal",
            "height_metres",
            "initial_normal_velocity_metres_per_second",
            "gravity_metres_per_second_squared",
            "mass_kg",
            "restitution",
            "transfer_fraction",
            "minimum_impact_speed_metres_per_second",
            "start_sample",
            "duration_samples",
        ],
    )?;
    for key in [
        "contact_ref",
        "particle_ref",
        "collider_ref",
        "policy_ref",
        "policy_revision",
        "standing",
        "mass_kg",
        "restitution",
        "transfer_fraction",
        "minimum_impact_speed_metres_per_second",
    ] {
        exact(&original[key], &def[key])?;
    }
    if original["route_ref"] != request.preparation_ref
        || original["source_ref"] != request.geometry.provenance.source_ref
        || decimal(&original["start_sample"])? != trigger
        || decimal(&original["duration_samples"])? != frames
    {
        return Err("contact force route is detached from actual prepared body/date".into());
    }
    exact(&original["normal"], &def["outward_normal"])?;
    exact(
        &original["plane_position_metres"],
        &record["exciter_position_metres"],
    )?;
    near(number(&original["height_metres"])?, height, 1e-10)?;
    near(
        number(&original["initial_normal_velocity_metres_per_second"])?,
        speed0,
        1e-10,
    )?;
    near(
        number(&original["gravity_metres_per_second_squared"])?,
        g,
        1e-10,
    )?;
    let mass = number(&def["mass_kg"])?;
    let restitution = number(&def["restitution"])?;
    let transfer = number(&def["transfer_fraction"])?;
    let minimum = number(&def["minimum_impact_speed_metres_per_second"])?;
    if !(0. ..=100.).contains(&height)
        || !(1e-6..=1e4).contains(&g)
        || !(1e-12..=1e3).contains(&mass)
        || !(0. ..=1.).contains(&restitution)
        || !(0. ..=1.).contains(&transfer)
        || !(0. ..=1e4).contains(&minimum)
        || dot(normal, request.exciter.axis).abs() <= 1e-10
    {
        return Err("contact physical quantities/actuator differ".into());
    }
    let operands = &record["native_operands"];
    keys(
        operands,
        &[
            "contact_ref",
            "particle_ref",
            "collider_ref",
            "route_ref",
            "source_ref",
            "policy_ref",
            "policy_revision",
            "standing",
            "seed_standing",
            "preparation_ref",
            "state_ref",
            "eigenbasis",
            "source_coordinate",
            "source_revision",
            "plane_position_metres",
            "normal",
            "impact_velocity_metres_per_second",
            "height_metres",
            "initial_normal_velocity_metres_per_second",
            "gravity_metres_per_second_squared",
            "mass_kg",
            "restitution",
            "transfer_fraction",
            "minimum_impact_speed_metres_per_second",
            "impact_seconds",
            "impact_speed_metres_per_second",
            "planned_impulse_newton_seconds",
            "force_newtons",
            "trigger_sample",
            "impact_sample",
            "body_revision",
            "source_generation",
            "seed",
            "sample_rate",
            "duration_samples",
            "pratibimba",
        ],
    )?;
    for key in [
        "contact_ref",
        "particle_ref",
        "collider_ref",
        "route_ref",
        "source_ref",
        "policy_ref",
        "policy_revision",
        "standing",
        "plane_position_metres",
        "normal",
        "height_metres",
        "initial_normal_velocity_metres_per_second",
        "gravity_metres_per_second_squared",
        "mass_kg",
        "restitution",
        "transfer_fraction",
        "minimum_impact_speed_metres_per_second",
    ] {
        exact(&operands[key], &original[key])?;
    }
    for (key, expected) in [
        ("preparation_ref", body["preparation_ref"].clone()),
        ("state_ref", body["state_ref"].clone()),
        ("source_coordinate", body["source_coordinate"].clone()),
        ("source_revision", body["source_revision"].clone()),
        ("body_revision", body["body_revision"].clone()),
        ("source_generation", body["source_generation"].clone()),
        ("sample_rate", body["sample_rate"].clone()),
        ("pratibimba", body["pratibimba"].clone()),
        ("eigenbasis", boundary["physical_eigenbasis"].clone()),
    ] {
        exact(&operands[key], &expected)?;
    }
    if operands["seed_standing"] != "deterministic-no-randomness"
        || decimal(&operands["seed"])? != 0
        || decimal(&operands["trigger_sample"])? != trigger
        || decimal(&operands["duration_samples"])? != frames
    {
        return Err("contact original deterministic seed/date/body operands differ".into());
    }
    bounded(
        operands["eigenbasis"]
            .as_str()
            .ok_or("contact native eigenbasis absent")?,
    )?;
    // C++ long double is the exact root/ceil owner. Compare its rounded binary64
    // descriptors independently, then compute Newton force in the same original
    // binary64 multiplication order and preserve ALL512 native force bits.
    let h = number(&original["height_metres"])?;
    let v = number(&original["initial_normal_velocity_metres_per_second"])?;
    let g = number(&original["gravity_metres_per_second_squared"])?;
    let predicted_speed = (v * v + 2. * g * h).sqrt();
    let actual_speed = number(&operands["impact_speed_metres_per_second"])?;
    near(
        actual_speed,
        predicted_speed,
        8. * f64::EPSILON * predicted_speed.abs().max(1.),
    )?;
    let seconds = if v < 0. {
        if h == 0. {
            0.
        } else {
            2. * h / (predicted_speed - v)
        }
    } else {
        (v + predicted_speed) / g
    };
    near(
        number(&operands["impact_seconds"])?,
        seconds,
        8. * f64::EPSILON * seconds.abs().max(1.),
    )?;
    if seconds > 86400. {
        return Err("contact time exceeds original native horizon".into());
    }
    let impact = decimal(&operands["impact_sample"])?;
    let offset = impact
        .checked_sub(trigger)
        .ok_or("contact impact predates native trigger")?;
    let scaled = number(&operands["impact_seconds"])? * f64::from(request.sample_rate);
    let rounding = 8. * f64::EPSILON * scaled.abs().max(1.);
    if scaled > offset as f64 + rounding
        || (offset > 0 && scaled <= (offset - 1) as f64 - rounding)
        || impact.checked_add(frames).is_none()
    {
        return Err("contact original sample/date range differs".into());
    }
    let mut projection = 0.;
    for axis in 0..3 {
        projection -= normal[axis] * request.exciter.axis[axis];
    }
    let impulse = if actual_speed < minimum {
        0.
    } else {
        transfer * (1. + restitution) * mass * actual_speed * projection
    };
    let force = impulse * f64::from(request.sample_rate) / frames as f64;
    if impulse.abs() > request.max_impulse_newton_seconds || force.abs() > request.max_force_newtons
    {
        return Err("contact force exceeded original physical budget".into());
    }
    if number(&operands["planned_impulse_newton_seconds"])?.to_bits() != impulse.to_bits()
        || number(&operands["force_newtons"])?.to_bits() != force.to_bits()
    {
        return Err("contact force/impulse no longer reproduces".into());
    }
    let impact_velocity = vector(&operands["impact_velocity_metres_per_second"])?;
    for axis in 0..3 {
        if impact_velocity[axis].to_bits() != (-actual_speed * normal[axis]).to_bits() {
            return Err("contact impact velocity differs".into());
        }
    }
    let forces = record["force_newtons"]
        .as_array()
        .filter(|v| v.len() == 512)
        .ok_or("contact lost original512 force samples")?;
    for (index, value) in forces.iter().enumerate() {
        let expected = if index < (frames as usize) { force } else { 0. };
        if number(value)?.to_bits() != expected.to_bits() {
            return Err("contact force suffix/value would be lost on replay".into());
        }
    }
    Ok(())
}

/// Numerical replay cursor; privately selected C corpus qualification happens
/// before and after the outer factory. This type has no Deserialize/Clone and
/// cannot activate P or issue a native occurrence witness.
pub(crate) struct ContactSourceReplay {
    records: Vec<Value>,
    admissions: Vec<Value>,
    next: usize,
    rows: Vec<Value>,
    previous_epoch: u64,
    previous_sequence: u64,
}
impl ContactSourceReplay {
    pub(crate) fn new(expected: &Value, admissions: &[Value]) -> Result<Self, String> {
        let records = match expected.get("contact_occurrence_history") {
            Some(value) => value
                .as_array()
                .ok_or("complete contact occurrence history absent")?
                .clone(),
            None => Vec::new(),
        };
        if records.len() != admissions.len()
            || serde_json::to_vec(&(expected, admissions))
                .map_err(|e| e.to_string())?
                .len()
                > crate::continuous::MAX_MESSAGE
        {
            return Err(
                "contact source lost original full corpus or native transport bound".into(),
            );
        }
        let mut ordinal = 0;
        let mut lineage: Option<Value> = None;
        for (index, (record, row)) in records.iter().zip(admissions).enumerate() {
            keys(
                row,
                &[
                    "schema",
                    "before_source_assets",
                    "source",
                    "native_admission",
                ],
            )?;
            if row["schema"] != "ql.native-scene-contact-admission/v1"
                || row["source"] != *record
                || record["schema"] != SOURCE
                || !row["before_source_assets"].is_object()
            {
                return Err(
                    "contact corpus detached its exact original source/admission epoch".into(),
                );
            }
            let request = decimal(&record["original_native_request_id"])?;
            if request <= ordinal
                || lineage
                    .as_ref()
                    .is_some_and(|v| *v != record["occurrence"]["constructor_lineage"])
            {
                return Err(
                    "contact original occurrence order/constructor lifetime changed".into(),
                );
            }
            let prefix = row["before_source_assets"].get("contact_occurrence_history");
            if index == 0 {
                if prefix.is_some_and(|v| v.as_array().is_none_or(|v| !v.is_empty())) {
                    return Err("first contact omitted an earlier native occurrence".into());
                }
            } else if prefix != Some(&json!(records[..index])) {
                return Err(
                    "contact before epoch lost the complete original occurrence prefix".into(),
                );
            }
            ordinal = request;
            lineage = Some(record["occurrence"]["constructor_lineage"].clone());
        }
        Ok(Self {
            records,
            admissions: admissions.to_vec(),
            next: 0,
            rows: Vec::new(),
            previous_epoch: 0,
            previous_sequence: 0,
        })
    }
    pub(crate) fn origin_bundle<'a>(&'a self, expected: &'a Value) -> &'a Value {
        self.admissions
            .first()
            .map_or(expected, |v| &v["before_source_assets"])
    }
    pub(crate) fn append_before(
        &mut self,
        owner: &mut PerformanceOwner,
        next_request: Option<u64>,
    ) -> Result<(), String> {
        while self.next < self.records.len() {
            let record = &self.records[self.next];
            let ordinal = decimal(&record["original_native_request_id"])?;
            if next_request.is_some_and(|limit| ordinal >= limit) {
                if next_request == Some(ordinal) {
                    return Err(
                        "contact reused an original physical/acoustic Manager operation".into(),
                    );
                }
                break;
            }
            let retained = &self.admissions[self.next];
            validate_current_scene_contact_record(
                owner,
                &retained["before_source_assets"],
                record,
            )?;
            validate_original_admission(owner, record, &retained["native_admission"])?;
            (self.previous_epoch, self.previous_sequence) = native_order(
                &retained["native_admission"],
                self.previous_epoch,
                self.previous_sequence,
                true,
            )?;
            self.rows.push(json!({"native_preparation":owner.packet()?,"native_basis":owner.binding().native_basis(),
                "physical_pratibimba":owner.config.physical_face==1,"source":record,"native_admission":retained["native_admission"]}));
            self.next += 1;
            owner.source_assets["contact_occurrence_history"] = json!(self.records[..self.next]);
        }
        Ok(())
    }
    pub(crate) fn check_source_application(&mut self, pulse: &Value) -> Result<(), String> {
        (self.previous_epoch, self.previous_sequence) =
            native_order(pulse, self.previous_epoch, self.previous_sequence, false)?;
        Ok(())
    }
    pub(crate) fn complete(&self) -> Result<(), String> {
        if self.next == self.records.len() {
            Ok(())
        } else {
            Err("cold source left an original Contact occurrence disconnected".into())
        }
    }
}
fn native_order(
    pulse: &Value,
    previous_epoch: u64,
    previous_sequence: u64,
    queued_contact: bool,
) -> Result<(u64, u64), String> {
    let epoch = decimal(&pulse["reading"]["transport_epoch"])?;
    let sequence = decimal(&pulse["reading"]["accepted_sequence"])?;
    if epoch == 0
        || epoch < previous_epoch
        || (epoch == previous_epoch
            && (sequence < previous_sequence || (queued_contact && sequence == previous_sequence)))
    {
        return Err("contact/source merged native ACK order regressed".into());
    }
    Ok((epoch, sequence))
}
fn validate_original_admission(
    owner: &PerformanceOwner,
    record: &Value,
    pulse: &Value,
) -> Result<(), String> {
    owner.validate_reply(pulse)?;
    let payload = &pulse["payload"];
    let queue = &payload["score_admission"];
    let event = &queue["event"];
    keys(
        queue,
        &[
            "schema",
            "queued",
            "session_ref",
            "transport_epoch",
            "queue_cursor",
            "queue_horizon",
            "input_ref",
            "event",
            "source",
        ],
    )?;
    let boundary = &record["native_boundary"];
    let operands = &record["native_operands"];
    let impact = decimal(&operands["impact_sample"])?;
    let sequence = decimal(&boundary["accepted_sequence"])?
        .checked_add(1)
        .ok_or("original contact sequence exhausted")?;
    if pulse["accepted"] != true
        || !matches!(
            pulse["operation"].as_str(),
            Some("contact-apply" | "contact-trigger")
        )
        || payload["queue_committed"] != true
        || payload["application_committed"] != false
        || payload["native_result"] != "accepted"
        || payload["contact_source"] != *record
        || queue["schema"] != "ql.native-score-admission/v1"
        || queue["queued"] != true
        || queue["session_ref"] != boundary["session_ref"]
        || queue["transport_epoch"] != boundary["transport_epoch"]
        || queue["input_ref"] != operands["contact_ref"]
        || queue["source"] != boundary["determination"]
        || event["kind"] != 7
        || event["identity"] != boundary["determination"]["identity"]
        || decimal(&event["sequence"])? != sequence
        || decimal(&event["sample"])? != impact
        || decimal(&event["requested_sample"])? != impact
        || decimal(&event["touch"])? != 0
        || number(&event["value"])? != 0.
        || number(&event["pitch_hz"])? != 0.
        || event["late_admitted"] != false
        || event["has_note"] != false
        || event["has_determination"] != false
        || decimal(&event["contact"]["generation"])? == 0
        || decimal(&event["contact"]["slot"])? >= 16
        || decimal(&pulse["reading"]["accepted_sequence"])? != sequence
        || pulse["reading"]["transport_epoch"] != queue["transport_epoch"]
        || decimal(&queue["queue_cursor"])? < decimal(&boundary["queue_cursor"])?
        || decimal(&queue["queue_cursor"])? > impact
        || decimal(&queue["queue_horizon"])? > impact
        || decimal(&queue["queue_horizon"])? < decimal(&boundary["queue_horizon"])?
    {
        return Err(
            "Contact original ACK lost exact queue/ordinal/source/clock/handle custody".into(),
        );
    }
    Ok(())
}

/// Keeps all body/acoustic frames and the entire original Contact corpus alive
/// while the SAME native worker regenerates its exact numerical programmes.
/// The result remains a preparation; C/Host still owns activation and CP restore.
pub(crate) struct PreparedColdContactSource {
    physical: PreparedColdPhysicalSource,
    source_assets: Value,
    contact_admissions: Vec<Value>,
    physical_applications: Vec<Value>,
    acoustic_applications: Vec<Value>,
    rows: Vec<Value>,
}
impl PreparedColdContactSource {
    pub(crate) fn physical(&self) -> &PreparedColdPhysicalSource {
        &self.physical
    }
    pub(crate) fn verification_request(
        &self,
        instance: &str,
        checkpoint_ref: &str,
        original_wire: &str,
    ) -> Result<Value, String> {
        bounded(instance)?;
        bounded(checkpoint_ref)?;
        let last = self.physical.final_frame();
        if original_wire.is_empty() || original_wire.len() > crate::continuous::MAX_MESSAGE {
            return Err("contact original checkpoint transport bound differs".into());
        }
        let value = json!({"schema":CONTACT_REPLAY_REQUEST,"operation":"contact-history-verify",
            "instance_ref":instance,"session_ref":last.owner().config.session_ref,"checkpoint_ref":checkpoint_ref,
            "current_native_preparation":last.owner().packet()?,"current_native_basis":last.owner().binding().native_basis(),
            "physical_pratibimba":last.owner().config.physical_face==1,"rows":self.rows,"checkpoint_wire":original_wire});
        if serde_json::to_vec(&value).map_err(|e| e.to_string())?.len()
            > crate::continuous::MAX_MESSAGE
        {
            return Err(
                "complete Contact numerical replay exceeds original32MiB worker bound".into(),
            );
        }
        Ok(value)
    }
    pub(crate) fn into_parts(self) -> (PreparedColdPhysicalSource, Vec<Value>) {
        (self.physical, self.contact_admissions)
    }
    /// This method itself traverses the original authenticated worker pipe.
    /// A caller cannot qualify retained JSON as a positive numeric result.
    pub(crate) fn qualify_original_native_checkpoint(
        &self,
        session: &mut CoupledFieldSession,
        instance: &str,
        checkpoint_ref: &str,
        original_wire: &str,
        lease: &crate::continuous::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<QualifiedContactCheckpoint, NativeStoppedExchangeFailure> {
        lease.validate_source_assets(instance, &self.source_assets)?;
        lease.validate_recorded_contact_admissions(
            instance,
            &self.source_assets,
            &self.contact_admissions,
        )?;
        lease.validate_recorded_source_applications(
            instance,
            &self.source_assets,
            &self.physical_applications,
            &self.acoustic_applications,
        )?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_wire)?;
        let qualified = QualifiedNativeSceneContactReplayWorkerRequest {
            request: self.verification_request(instance, checkpoint_ref, original_wire)?,
        };
        let original_native_request = qualified.request().clone();
        let original_native_reply =
            session
                .scene_contact_replay_retained(&qualified)
                .map_err(|(reason, original)| NativeStoppedExchangeFailure {
                    reason,
                    native_receipts: original.into_iter().collect(),
                })?;
        self.qualify_after_original_native_reply(
            instance,
            checkpoint_ref,
            original_wire,
            original_native_request,
            original_native_reply.clone(),
            lease,
        )
        .map_err(|reason| NativeStoppedExchangeFailure {
            reason,
            native_receipts: vec![original_native_reply],
        })
    }
    /// Only the actual private Worker/FieldHost result is passed here under the
    /// still-held closed C lease. Caller/source JSON cannot construct this type.
    fn qualify_after_original_native_reply(
        &self,
        instance: &str,
        checkpoint_ref: &str,
        original_wire: &str,
        original_native_request: Value,
        original_native_reply: Value,
        lease: &crate::continuous::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<QualifiedContactCheckpoint, String> {
        lease.validate_source_assets(instance, &self.source_assets)?;
        lease.validate_recorded_contact_admissions(
            instance,
            &self.source_assets,
            &self.contact_admissions,
        )?;
        lease.validate_recorded_source_applications(
            instance,
            &self.source_assets,
            &self.physical_applications,
            &self.acoustic_applications,
        )?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_wire)?;
        exact(
            &original_native_request,
            &self.verification_request(instance, checkpoint_ref, original_wire)?,
        )?;
        keys(
            &original_native_reply,
            &[
                "schema",
                "operation",
                "accepted",
                "instance_ref",
                "session_ref",
                "checkpoint_ref",
                "checkpoint_wire",
                "verified_sources",
            ],
        )?;
        let records: Vec<Value> = self.rows.iter().map(|v| v["source"].clone()).collect();
        let last = self.physical.final_frame();
        if original_native_reply["schema"] != "ql.native-scene-contact-replay-reply/v1"
            || original_native_reply["operation"] != "contact-history-verify"
            || original_native_reply["accepted"] != true
            || original_native_reply["instance_ref"] != instance
            || original_native_reply["session_ref"] != last.owner().config.session_ref
            || original_native_reply["checkpoint_ref"] != checkpoint_ref
            || original_native_reply["checkpoint_wire"] != original_wire
        {
            return Err(
                "original Contact numeric worker reply detached its source/CP selection".into(),
            );
        }
        exact(&original_native_reply["verified_sources"], &json!(records))?;
        lease.validate_source_assets(instance, &self.source_assets)?;
        lease.validate_recorded_contact_admissions(
            instance,
            &self.source_assets,
            &self.contact_admissions,
        )?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_wire)?;
        Ok(QualifiedContactCheckpoint {
            source_assets: self.source_assets.clone(),
            contact_admissions: self.contact_admissions.clone(),
            physical_applications: self.physical_applications.clone(),
            acoustic_applications: self.acoustic_applications.clone(),
            instance: instance.into(),
            checkpoint_ref: checkpoint_ref.into(),
            original_wire: original_wire.into(),
            original_native_request,
            original_native_reply,
            closed_selection: lease.evidence(),
        })
    }
}
/// Issued only from privately regenerated complete C source/CP qualification.
/// This is a pure numerical compiler request, never a Contact admission token.
pub(crate) struct QualifiedNativeSceneContactReplayWorkerRequest {
    request: Value,
}
impl QualifiedNativeSceneContactReplayWorkerRequest {
    pub(crate) fn request(&self) -> &Value {
        &self.request
    }
}
/// Non-Clone/non-Deserialize typed continuation operand, not a witness issuer.
pub(crate) struct QualifiedContactCheckpoint {
    source_assets: Value,
    contact_admissions: Vec<Value>,
    physical_applications: Vec<Value>,
    acoustic_applications: Vec<Value>,
    instance: String,
    checkpoint_ref: String,
    original_wire: String,
    original_native_request: Value,
    original_native_reply: Value,
    closed_selection: Value,
}
impl QualifiedContactCheckpoint {
    /// Exact originals issued/received on the same private native pipe. These
    /// borrowed diagnostics cannot construct a replay grant or Contact witness.
    pub(crate) fn original_request(&self) -> &Value {
        &self.original_native_request
    }
    pub(crate) fn original_reply(&self) -> &Value {
        &self.original_native_reply
    }
    pub(crate) fn validate_prepared(
        &self,
        prepared: &PreparedColdContactSource,
        lease: &crate::continuous::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<(), String> {
        if self.source_assets != prepared.source_assets
            || self.contact_admissions != prepared.contact_admissions
            || self.physical_applications != prepared.physical_applications
            || self.acoustic_applications != prepared.acoustic_applications
        {
            return Err(
                "qualified Contact checkpoint detached its whole original replay candidate".into(),
            );
        }
        exact(
            &self.original_native_request,
            &prepared.verification_request(
                &self.instance,
                &self.checkpoint_ref,
                &self.original_wire,
            )?,
        )?;
        let records: Vec<Value> = prepared.rows.iter().map(|v| v["source"].clone()).collect();
        exact(
            &self.original_native_reply["verified_sources"],
            &json!(records),
        )?;
        self.validate_held(
            &self.instance,
            &self.checkpoint_ref,
            &self.original_wire,
            lease,
        )
    }
    pub(crate) fn validate_held(
        &self,
        instance: &str,
        checkpoint_ref: &str,
        original_wire: &str,
        lease: &crate::continuous::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<(), String> {
        if self.instance != instance
            || self.checkpoint_ref != checkpoint_ref
            || self.original_wire != original_wire
            || self.closed_selection != lease.evidence()
            || self.original_native_request["instance_ref"] != instance
            || self.original_native_request["checkpoint_ref"] != checkpoint_ref
            || self.original_native_request["checkpoint_wire"] != original_wire
            || self.original_native_reply["checkpoint_wire"] != original_wire
        {
            return Err(
                "qualified Contact history lost original held source/checkpoint/worker".into(),
            );
        }
        lease.validate_source_assets(instance, &self.source_assets)?;
        lease.validate_recorded_contact_admissions(
            instance,
            &self.source_assets,
            &self.contact_admissions,
        )?;
        lease.validate_recorded_source_applications(
            instance,
            &self.source_assets,
            &self.physical_applications,
            &self.acoustic_applications,
        )?;
        lease.validate_selected_checkpoint(instance, checkpoint_ref, original_wire)
    }
}
/// Borrowed complete actual C source sidecars. This grouping grants no source
/// custody; the privately minted lease qualifies every byte before replay.
pub(crate) struct ColdNativeContactCorpus<'a> {
    pub(crate) physical_applications: &'a [Value],
    pub(crate) acoustic_applications: &'a [Value],
    pub(crate) contact_admissions: &'a [Value],
}
impl PerformanceOwner {
    pub(crate) fn restore_qualified_contact_admission_history(
        &mut self,
        admissions: Vec<Value>,
        lease: &crate::continuous::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<(), String> {
        let instance = self.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or("cold Contact native instance absent")?
            .to_owned();
        lease.validate_source_assets(&instance, &self.source_assets)?;
        lease.validate_recorded_contact_admissions(&instance, &self.source_assets, &admissions)?;
        let records: Vec<Value> = admissions.iter().map(|v| v["source"].clone()).collect();
        match self.source_assets.get("contact_occurrence_history") {
            Some(value) => exact(value, &json!(records))?,
            None if records.is_empty() => {}
            None => return Err("cold Contact owner lost original numerical occurrences".into()),
        }
        self.contact_admission_history = admissions;
        Ok(())
    }
    pub(in crate::continuous) fn prepare_cold_native_contact_source(
        original: &CoupledBasis,
        instance: &str,
        source: &NativePerformanceReceivingSource,
        expected: &Value,
        corpus: ColdNativeContactCorpus<'_>,
        lease: &crate::continuous::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<PreparedColdContactSource, String> {
        let ColdNativeContactCorpus {
            physical_applications,
            acoustic_applications,
            contact_admissions,
        } = corpus;
        lease.validate_source_assets(instance, expected)?;
        lease.validate_recorded_source_applications(
            instance,
            expected,
            physical_applications,
            acoustic_applications,
        )?;
        lease.validate_recorded_contact_admissions(instance, expected, contact_admissions)?;
        let mut contacts = ContactSourceReplay::new(expected, contact_admissions)?;
        let physical = Self::replay_cold_native_acoustic_source_with_contacts(
            original,
            instance,
            source,
            expected,
            physical_applications,
            acoustic_applications,
            &mut contacts,
        )?;
        contacts.complete()?;
        lease.validate_source_assets(instance, physical.final_frame().owner().source_assets())?;
        lease.validate_recorded_contact_admissions(instance, expected, contact_admissions)?;
        Ok(PreparedColdContactSource {
            physical,
            source_assets: expected.clone(),
            contact_admissions: contact_admissions.to_vec(),
            physical_applications: physical_applications.to_vec(),
            acoustic_applications: acoustic_applications.to_vec(),
            rows: contacts.rows,
        })
    }
}
