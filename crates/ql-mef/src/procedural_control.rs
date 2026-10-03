//! Pure control preparation over the native Parameter and actual authored
//! automation/property-track material. The receiving owner attests every read
//! and preview at CAS; this module owns no parameter converter or driver clock.
use crate::m_tree::native_current_m_registry;
use crate::procedural_composition::{OwnedAddress, Procedure, Result, native_parameter_address};
use crate::procedural_intervention::{
    ManualEdit, NativeAuthoredIntervention, extract_owned_manual_interventions,
    native_control_material_pointer, restore_controlled_parent_baseline,
};
use crate::procedural_manifestation::{fingerprint, nonempty, validate_material};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeDriverScene {
    pub scene_ref: String,
    pub entity_refs: BTreeMap<String, String>,
    /// Actual current Presentation, including retention with operations: [].
    pub presentation: Value,
    /// The protected native Document's preview of ParameterSet, using its own
    /// existing conversion. Required for takeover/set_base, absent for release.
    pub parameter_candidate: Option<Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeParameterDriverReading {
    pub schema: String,
    pub expression_ref: String,
    pub document_revision: u64,
    pub entity_ref: String,
    pub parameter: String,
    /// Exact native Parameter serialization {value, automation?} at this CAS.
    pub native_parameter: Value,
    /// Every Scene manifestation affected by the ONE global Parameter operation.
    pub addresses: Vec<OwnedAddress>,
    pub scenes: Vec<NativeDriverScene>,
    /// Protected receiving owner supplies ALL current qualified definitions
    /// covering an older control without procedure_ref. Exactly one must match;
    /// this source input is not a capability or a first-match owner inference.
    #[serde(default)]
    pub qualified_legacy_procedures: Vec<Procedure>,
    /// Protected native receiver qualifies every foreign/legacy peer whose
    /// dormant group metadata can be changed. This DTO grants no live standing.
    #[serde(default)]
    pub qualified_control_peers: Vec<NativeQualifiedControlPeer>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeQualifiedControlPeer {
    pub address: OwnedAddress,
    pub parameter: String,
    pub procedure: Procedure,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlLifetime {
    Gesture,
    Persistent,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum NativeControlAction {
    Takeover {
        value: Value,
        lifetime: ControlLifetime,
    },
    SetBase {
        value: Value,
    },
    Release {},
    ReleaseGesture {
        takeover_operation_ref: String,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeControlInput {
    pub procedure: Procedure,
    pub reading: NativeParameterDriverReading,
    pub actor_ref: String,
    pub operation_ref: String,
    pub action: NativeControlAction,
}

fn scalar(value: &Value) -> Result<()> {
    if !value.as_f64().is_some_and(f64::is_finite) {
        return Err("native scalar control requires a finite native value".into());
    }
    Ok(())
}
fn is_native_flow_contribution(contribution: &Value) -> Result<bool> {
    let Some(flow) = contribution["generated_basis"].get("native_flow") else {
        return Ok(false);
    };
    let changes: Vec<crate::procedural_composition::NativeChange> =
        serde_json::from_value(flow.clone()).map_err(|e| e.to_string())?;
    if changes.is_empty()
        || changes.len() > 64
        || contribution["generated_basis"].get("scene").is_some()
    {
        return Err("native flow contribution has mixed/unbounded material basis".into());
    }
    for change in changes {
        change.validate()?;
        if !matches!(
            change,
            crate::procedural_composition::NativeChange::Focus { .. }
                | crate::procedural_composition::NativeChange::RelationFocus { .. }
                | crate::procedural_composition::NativeChange::SceneReorder { .. }
        ) {
            return Err("native flow contribution has a foreign material operation".into());
        }
    }
    Ok(true)
}
fn validate_control_procedure(control: &Value, input: &NativeControlInput) -> Result<()> {
    if control
        .get("parameter")
        .is_some_and(|p| p.as_str() != Some(input.reading.parameter.as_str()))
    {
        return Err("native retained control parameter differs from original native driver".into());
    }
    match control.get("procedure_ref") {
        Some(reference) if reference.as_str() == Some(input.procedure.procedure_ref.as_str()) => {
            Ok(())
        }
        Some(_) => Err("native retained control belongs to another original procedure".into()),
        None if input.reading.qualified_legacy_procedures.len() == 1
            && input.reading.qualified_legacy_procedures[0] == input.procedure =>
        {
            Ok(())
        }
        None => Err(
            "legacy native control requires exactly one current qualified original procedure"
                .into(),
        ),
    }
}
fn validate_control_peer(
    control: &Value,
    scene: &NativeDriverScene,
    input: &NativeControlInput,
) -> Result<()> {
    if !is_native_scalar_control(control)? {
        return Ok(());
    }
    let address: OwnedAddress =
        serde_json::from_value(control["address"].clone()).map_err(|e| e.to_string())?;
    let supplied = input
        .reading
        .qualified_control_peers
        .iter()
        .filter(|peer| peer.address == address)
        .collect::<Vec<_>>();
    if supplied.len() > 1 {
        return Err("native control peer has ambiguous qualified original definitions".into());
    }
    let (parameter, original): (&str, &Procedure) = if let Some(peer) = supplied.first() {
        if control
            .get("procedure_ref")
            .is_some_and(|p| p != &json!(peer.procedure.procedure_ref))
            || control
                .get("parameter")
                .is_some_and(|p| p != &json!(peer.parameter))
        {
            return Err("native control peer original identity differs".into());
        }
        (&peer.parameter, &peer.procedure)
    } else if control["procedure_ref"] == input.procedure.procedure_ref {
        (
            control["parameter"]
                .as_str()
                .ok_or("native control peer parameter qualification absent")?,
            &input.procedure,
        )
    } else if input.reading.addresses.contains(&address) {
        validate_control_procedure(control, input)?;
        (input.reading.parameter.as_str(), &input.procedure)
    } else {
        return Err("foreign or legacy native control peer requires its exact qualified original definition".into());
    };
    original.validate(native_current_m_registry())?;
    let entity_ref = address
        .entity_ref
        .as_ref()
        .ok_or("native control peer Entity absent")?;
    let canonical_target = format!(
        "entity:{}:{}",
        encode_component(entity_ref),
        local_path(parameter)?.1
    );
    if control["target"] != canonical_target {
        return Err(
            "native retained control target differs from its exact Registry coordinate".into(),
        );
    }
    if original.occurrence_ref != input.reading.expression_ref
        || scene
            .entity_refs
            .iter()
            .filter(|(material, native)| *material == *native && *native == entity_ref)
            .count()
            != 1
        || native_parameter_address(
            &input.reading.expression_ref,
            &scene.scene_ref,
            entity_ref,
            parameter,
        )? != address
        || scene.presentation["scene"]["procedural"]["procedures"]
            .as_array()
            .is_some_and(|definitions| {
                definitions.iter().any(|definition| {
                    definition["procedure_ref"] == original.procedure_ref
                        && definition["definition"] != json!(original)
                })
            })
    {
        return Err(
            "native control peer current namespace/parameter/original definition differs".into(),
        );
    }
    Ok(())
}
fn local_path(parameter: &str) -> Result<(&'static str, &'static str)> {
    // These names are the existing Registry/Document names, not a unit map.
    match parameter {
        "force_strength" => Ok(("force.strength", "forces.strength")),
        "force_spin" => Ok(("force.spin", "forces.spin")),
        "force_radius" => Ok(("force.radius", "forces.radius")),
        "x" => Ok(("position.x", "x")),
        "y" => Ok(("position.y", "y")),
        "z" => Ok(("position.z", "z")),
        "scale" => Ok(("scale", "scale")),
        "rotation" => Ok(("rotation", "rotation")),
        _ => Err("native scalar control has no current authored Registry binding".into()),
    }
}
fn encode_component(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}
fn entity<'a>(presentation: &'a Value, id: &str) -> Result<&'a Value> {
    presentation["scene"]["entities"]
        .as_array()
        .and_then(|a| a.iter().find(|e| e["id"] == id))
        .ok_or("actual native driver material entity is absent".into())
}
fn entity_mut<'a>(presentation: &'a mut Value, id: &str) -> Result<&'a mut Value> {
    presentation["scene"]["entities"]
        .as_array_mut()
        .and_then(|a| a.iter_mut().find(|e| e["id"] == id))
        .ok_or("actual native driver material entity is absent".into())
}
fn at_path<'a>(mut root: &'a Value, path: &str) -> Result<&'a Value> {
    for part in path.split('.') {
        root = root
            .get(part)
            .ok_or("actual authored control path is absent")?;
    }
    Ok(root)
}
fn set_path(root: &mut Value, path: &str, value: Value) -> Result<()> {
    let mut parts = path.split('.').peekable();
    let mut current = root;
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            let object = current
                .as_object_mut()
                .ok_or("authored control target is not an object")?;
            if !object.contains_key(part) {
                return Err("authored control target is absent".into());
            }
            object.insert(part.into(), value);
            return Ok(());
        }
        current = current
            .get_mut(part)
            .ok_or("authored control parent is absent")?;
    }
    Err("authored control path is empty".into())
}
fn lanes(presentation: &Value) -> Result<Vec<Value>> {
    let lanes = presentation["scene"]["automation"]
        .as_array()
        .ok_or("actual automation lane array is absent")?
        .clone();
    let mut ids = BTreeSet::new();
    for lane in &lanes {
        nonempty(
            lane["id"]
                .as_str()
                .ok_or("automation lane identity absent")?,
            "lane.id",
        )?;
        if !ids.insert(lane["id"].as_str().unwrap()) {
            return Err("duplicate native automation lane identity".into());
        }
    }
    for lane in &lanes {
        leader(&lanes, lane)?;
    }
    Ok(lanes)
}
fn leader<'a>(lanes: &'a [Value], mut lane: &'a Value) -> Result<&'a Value> {
    let mut seen = BTreeSet::new();
    while let Some(reference) = lane.get("syncWith") {
        let reference = reference
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or("invalid native automation syncWith")?;
        if !seen.insert(reference) {
            return Err("cyclic native automation group".into());
        }
        lane = lanes
            .iter()
            .find(|l| l["id"] == reference)
            .ok_or("native automation leader disappeared")?;
    }
    Ok(lane)
}
fn suspend_target(lanes: &[Value], target: &str) -> Result<(Vec<Value>, Vec<Value>, Vec<Value>)> {
    let roots = lanes
        .iter()
        .filter(|l| l["target"] == target)
        .map(|l| leader(lanes, l).map(|l| l["id"].clone()))
        .collect::<Result<Vec<_>>>()?;
    let dormant = lanes
        .iter()
        .filter(|l| leader(lanes, l).is_ok_and(|r| roots.contains(&r["id"])))
        .cloned()
        .collect::<Vec<_>>();
    let mut remaining = lanes.to_vec();
    let remove = remaining
        .iter()
        .filter(|l| l["target"] == target)
        .map(|l| l["id"].as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    for id in remove {
        let removed = remaining
            .iter()
            .find(|l| l["id"] == id)
            .ok_or("removed lane missing")?
            .clone();
        if removed.get("syncWith").is_none() {
            let children = remaining
                .iter()
                .filter(|l| l["id"] != id && leader(&remaining, l).is_ok_and(|r| r["id"] == id))
                .map(|l| l["id"].as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            if let Some(promoted) = children.first() {
                let next = remaining.iter_mut().find(|l| l["id"] == *promoted).unwrap();
                for key in [
                    "type", "wave", "rate", "phase", "duration", "delay", "loop", "easing",
                    "firedAt",
                ] {
                    if let Some(value) = removed.get(key) {
                        next[key] = value.clone();
                    } else {
                        next.as_object_mut().unwrap().remove(key);
                    }
                }
                next["enabled"] = json!(next["enabled"] == true && removed["enabled"] == true);
                next.as_object_mut().unwrap().remove("syncWith");
                next["clockId"] = removed
                    .get("clockId")
                    .or_else(|| removed.get("nativeId"))
                    .unwrap_or(&removed["id"])
                    .clone();
                for child in children.iter().skip(1) {
                    remaining.iter_mut().find(|l| l["id"] == *child).unwrap()["syncWith"] =
                        json!(promoted);
                }
            }
        }
        remaining.retain(|l| l["id"] != id);
    }
    for lane in &remaining {
        leader(&remaining, lane)?;
    }
    let suspended = remaining
        .iter()
        .filter(|l| dormant.iter().any(|d| d["id"] == l["id"]))
        .cloned()
        .collect();
    Ok((remaining, dormant, suspended))
}

fn lane_ids(lanes: &[Value]) -> Result<BTreeSet<String>> {
    lanes
        .iter()
        .map(|lane| {
            lane["id"]
                .as_str()
                .filter(|id| !id.is_empty())
                .map(str::to_owned)
                .ok_or_else(|| "native dormant driver identity absent".into())
        })
        .collect()
}
/// Existing canonical lane/group snapshots are authored driver state. Select
/// an exact containing snapshot, never synthesize a clock or a changed lane.
fn validate_promoted_subset(full: &[Value], subset: &[Value], controls: &[Value]) -> Result<()> {
    let ids = lane_ids(subset)?;
    let mut removed = BTreeSet::new();
    for lane in full
        .iter()
        .filter(|lane| lane["id"].as_str().is_some_and(|id| !ids.contains(id)))
    {
        removed.insert(
            lane["target"]
                .as_str()
                .ok_or("native dormant lane target absent")?,
        );
    }
    // This is the retained Source constructor order, not the authored lane
    // array order or a new clock. First takeover appends exactly one control;
    // refresh retains its index and release only removes it. The protected
    // receiver supplies this exact current native retained array and preserves
    // unrelated order. Replaying lane order can change disabled-leader
    // promotion and falsely refuse an actual Source-produced snapshot.
    let mut actual = full.to_vec();
    let mut seen = BTreeSet::new();
    for control in controls {
        if !is_native_scalar_control(control)? || control["takeover"].is_null() {
            continue;
        }
        let target = control["target"]
            .as_str()
            .ok_or("retained native takeover target absent")?;
        if !removed.contains(target) {
            continue;
        }
        if !seen.insert(target) {
            return Err("promoted dormant subset has duplicate retained target provenance".into());
        }
        actual = suspend_target(&actual, target)?.0;
    }
    if seen != removed {
        return Err("promoted dormant subset lacks original retained takeover source".into());
    }
    if actual != subset {
        return Err("promoted dormant subset differs from exact original native suspension".into());
    }
    Ok(())
}
fn canonical_dormant_group(controls: &[Value], captured: &[Value]) -> Result<Vec<Value>> {
    let mut chosen = captured.to_vec();
    let mut ids = lane_ids(&chosen)?;
    for control in controls {
        if !is_native_scalar_control(control)? {
            continue;
        }
        let baseline = control["dormant_lanes"]
            .as_array()
            .ok_or("native dormant group absent")?;
        let other = lane_ids(baseline)?;
        if ids.is_disjoint(&other) {
            continue;
        }
        if other.is_superset(&ids) && other != ids {
            validate_promoted_subset(baseline, &chosen, controls)?;
            chosen = baseline.clone();
            ids = other;
        } else if ids == other && chosen != *baseline {
            return Err(
                "overlapping dormant driver group has contradictory source snapshots".into(),
            );
        } else if !ids.is_superset(&other) {
            return Err(
                "overlapping dormant groups have no exact containing original source".into(),
            );
        } else if ids != other {
            validate_promoted_subset(&chosen, baseline, controls)?;
        }
    }
    Ok(chosen)
}
fn validate_current_group(control: &Value, controls: &[Value], actual: &[Value]) -> Result<()> {
    let suspended = control["suspended_lanes"]
        .as_array()
        .ok_or("native suspended group absent")?;
    let dormant = control["dormant_lanes"]
        .as_array()
        .ok_or("native dormant group absent")?;
    let ids = lane_ids(dormant)?;
    for lane in actual
        .iter()
        .filter(|lane| lane["id"].as_str().is_some_and(|id| ids.contains(id)))
    {
        if !suspended.iter().any(|expected| expected == lane) {
            return Err("dormant automation group changed during takeover".into());
        }
    }
    for expected in suspended {
        if actual.iter().any(|lane| lane == expected) {
            continue;
        }
        // An already qualified sibling takeover can remove this exact lane.
        // Its actual retained dormant snapshot is the only accepted witness.
        let covered = controls.iter().any(|other| {
            other != control
                && is_native_scalar_control(other).is_ok_and(|native| native)
                && other["target"] == expected["target"]
                && other["dormant_lanes"]
                    .as_array()
                    .is_some_and(|a| a.iter().any(|l| l == expected))
        });
        if !covered {
            return Err("dormant automation group changed during takeover".into());
        }
    }
    Ok(())
}
fn update_suspended_groups(controls: &mut [Value], actual: &[Value]) -> Result<()> {
    for control in controls {
        if !is_native_scalar_control(control)? {
            continue;
        }
        let ids = lane_ids(
            control["dormant_lanes"]
                .as_array()
                .ok_or("native dormant group absent")?,
        )?;
        control["suspended_lanes"] = json!(
            actual
                .iter()
                .filter(|lane| lane["id"].as_str().is_some_and(|id| ids.contains(id)))
                .collect::<Vec<_>>()
        );
    }
    Ok(())
}
fn records(value: &Value) -> Result<Vec<NativeAuthoredIntervention>> {
    serde_json::from_value(value.clone()).map_err(|e| e.to_string())
}

fn attribute_preview(
    out: &mut Value,
    scene: &NativeDriverScene,
    after: &Value,
    input: &NativeControlInput,
    address: &OwnedAddress,
    persistent: bool,
) -> Result<Vec<(Value, Value)>> {
    let mut changed = Vec::new();
    let contributions = out["scene"]["procedural"]["contributions"]
        .as_array_mut()
        .ok_or("contribution retention absent")?;
    for c in contributions {
        if c["status"] != "active" || is_native_flow_contribution(c)? {
            continue;
        }
        let owned = serde_json::from_value::<Vec<OwnedAddress>>(c["owned_addresses"].clone())
            .map_err(|e| e.to_string())?;
        if !owned.iter().any(|a| a.covers(address)) {
            continue;
        }
        let old_all = records(&c["authored_overrides"])?;
        let old = old_all
            .iter()
            .filter(|r| r.address.scene_ref.as_ref() == Some(&scene.scene_ref))
            .cloned()
            .collect();
        let foreign = old_all
            .into_iter()
            .filter(|r| r.address.scene_ref.as_ref() != Some(&scene.scene_ref))
            .collect::<Vec<_>>();
        let edit = ManualEdit {
            expression_ref: input.reading.expression_ref.clone(),
            scene_ref: scene.scene_ref.clone(),
            contribution_ref: c["contribution_ref"]
                .as_str()
                .ok_or("contribution identity absent")?
                .into(),
            owned_addresses: owned,
            entity_refs: scene.entity_refs.clone(),
            before: scene.presentation.clone(),
            after: after.clone(),
            actor_ref: input.actor_ref.clone(),
            operation_ref: input.operation_ref.clone(),
            document_revision: input
                .reading
                .document_revision
                .checked_add(1)
                .ok_or("document revision exhausted")?,
            retained_overlays: vec![],
            retained_native_records: old,
        };
        let mut projected = extract_owned_manual_interventions(&edit)?.native_records;
        if !persistent {
            for row in &mut projected {
                if row.revision == edit.document_revision
                    && row.operation_ref.as_deref() == Some(input.operation_ref.as_str())
                {
                    row.persistent = false;
                }
            }
        }
        projected.extend(foreign);
        c["authored_overrides"] = json!(projected);
        changed.push((
            c["contribution_ref"].clone(),
            c["authored_overrides"].clone(),
        ));
    }
    Ok(changed)
}

/// The public value is a typed source preparation, never live authority. Root
/// re-attests exact current native Parameter, original Procedure and previews,
/// then seals the complete resulting native_edit before normal native apply.
pub fn prepare_native_control(input: &NativeControlInput) -> Result<Value> {
    input.procedure.validate(native_current_m_registry())?;
    let r = &input.reading;
    nonempty(&input.actor_ref, "actor")?;
    nonempty(&input.operation_ref, "operation")?;
    validate_material(&serde_json::to_value(input).map_err(|e| e.to_string())?, 0)?;
    if r.schema != "ql.native-parameter-driver/v1"
        || r.expression_ref != input.procedure.occurrence_ref
        || r.scenes.is_empty()
        || r.scenes.len() > 2048
        || r.addresses.len() != r.scenes.len()
        || r.qualified_control_peers.len() > 2048
    {
        return Err(
            "native control reading differs from original whole/current complete manifestations"
                .into(),
        );
    }
    let parameter = r
        .native_parameter
        .as_object()
        .ok_or("actual native Parameter is absent")?;
    if !parameter.contains_key("value")
        || parameter
            .keys()
            .any(|k| !matches!(k.as_str(), "value" | "automation"))
    {
        return Err("native Parameter reading has unknown/missing fields".into());
    }
    scalar(&r.native_parameter["value"])?;
    let mut peer_addresses = BTreeSet::new();
    for peer in &r.qualified_control_peers {
        if !peer_addresses.insert(&peer.address)
            || !r.scenes.iter().any(|scene| {
                scene.presentation["scene"]["procedural"]["controls"]
                    .as_array()
                    .is_some_and(|controls| {
                        controls.iter().any(|control| {
                            control["address"] == json!(peer.address)
                                && is_native_scalar_control(control).is_ok_and(|native| native)
                        })
                    })
            })
        {
            return Err(
                "native control peer is duplicated or absent from actual current controls".into(),
            );
        }
    }
    let (path, suffix) = local_path(&r.parameter)?;
    let mut scene_ids = BTreeSet::new();
    let mut actual_addresses = Vec::new();
    let mut managed_control_addresses = Vec::new();
    let mut scene_coverage = Vec::new();
    let mut changes = Vec::new();
    let mut retained_native = None;
    let target_value = match &input.action {
        NativeControlAction::Takeover { value, .. } | NativeControlAction::SetBase { value } => {
            scalar(value)?;
            Some(value)
        }
        NativeControlAction::Release {} | NativeControlAction::ReleaseGesture { .. } => None,
    };
    for scene in &r.scenes {
        if !scene_ids.insert(&scene.scene_ref)
            || scene.presentation["schema"] != "oi.journey-scene/v1"
            || scene.presentation["scene"]["id"] != scene.scene_ref
        {
            return Err("duplicate/foreign native control Scene".into());
        }
        let address = native_parameter_address(
            &r.expression_ref,
            &scene.scene_ref,
            &r.entity_ref,
            &r.parameter,
        )?;
        if !r.addresses.contains(&address) {
            return Err("native control omits an affected scalar manifestation".into());
        }
        actual_addresses.push(address.clone());
        let material_ids = scene
            .entity_refs
            .iter()
            .filter(|(_, native)| *native == &r.entity_ref)
            .map(|(id, _)| id)
            .collect::<Vec<_>>();
        if material_ids.len() != 1 {
            return Err("native control entity material mapping is ambiguous".into());
        }
        let id = material_ids[0];
        if id != &r.entity_ref {
            return Err(
                "native control material ID differs from actual Document entity reference".into(),
            );
        }
        let target = format!("entity:{}:{suffix}", encode_component(id));
        let bind = format!("entity.{path}");
        let before_value = at_path(entity(&scene.presentation, id)?, path)?.clone();
        scalar(&before_value)?;
        let mut out = scene.presentation.clone();
        let retention = match out["scene"].get("procedural") {
            None => crate::procedural_retention::empty_retention(),
            Some(value) => value.clone(),
        };
        if retention["schema"] != "oi.expression-procedural/v1"
            || retention["operations"]
                .as_array()
                .is_none_or(|a| !a.is_empty())
            || [
                "bindings",
                "procedures",
                "contributions",
                "controls",
                "scene_flow",
                "time_mappings",
                "source_basis",
            ]
            .iter()
            .any(|key| retention[*key].as_array().is_none())
            || retention["procedures"].as_array().is_none_or(|a| {
                a.iter().any(|p| {
                    p["procedure_ref"] == input.procedure.procedure_ref
                        && p["definition"] != json!(input.procedure)
                })
            })
        {
            return Err("native control lost current retention/full original procedure or journal projection".into());
        }
        // An actual shared manifestation can have no procedure/retention row.
        // Root qualifies the original installed definition separately; Source
        // appends only this control. No subject, contribution or clock is minted.
        out["scene"]["procedural"] = retention.clone();
        let mut controls = retention["controls"]
            .as_array()
            .ok_or("native control array absent")?
            .clone();
        let position = controls.iter().position(|c| c["address"] == json!(address));
        if let Some(index) = position {
            validate_control_procedure(&controls[index], input)?;
            controls[index]["procedure_ref"] = json!(input.procedure.procedure_ref);
            controls[index]["parameter"] = json!(r.parameter);
        }
        let original_lanes = lanes(&out)?;
        for control in &controls {
            validate_control_peer(control, scene, input)?;
            if is_native_scalar_control(control)? {
                validate_current_group(control, &controls, &original_lanes)?;
            }
        }
        let original_tracks = out["scene"]["propertyTracks"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let same_track = |track: &Value| track["bind"] == bind && track["entityId"] == *id;
        match &input.action {
            NativeControlAction::Takeover { value, lifetime } => {
                let candidate = scene
                    .parameter_candidate
                    .as_ref()
                    .ok_or("native ParameterSet material preview is required")?;
                let authored = at_path(entity(candidate, id)?, path)?.clone();
                scalar(&authored)?;
                let mut expected = scene.presentation.clone();
                set_path(entity_mut(&mut expected, id)?, path, authored.clone())?;
                if &expected != candidate {
                    return Err(
                        "native Parameter preview changes material outside exact authored scalar"
                            .into(),
                    );
                }
                let index = if let Some(index) = position {
                    if controls[index]["takeover"].is_null() {
                        return Err("retained native control is not an active takeover".into());
                    }
                    validate_current_group(&controls[index], &controls, &original_lanes)?;
                    index
                } else {
                    let (remaining, captured, suspended) =
                        suspend_target(&original_lanes, &target)?;
                    let dormant = canonical_dormant_group(&controls, &captured)?;
                    let group_ids = lane_ids(&dormant)?;
                    for control in &controls {
                        if is_native_scalar_control(control)?
                            && !group_ids.is_disjoint(&lane_ids(
                                control["dormant_lanes"]
                                    .as_array()
                                    .ok_or("native dormant group absent")?,
                            )?)
                        {
                            validate_current_group(control, &controls, &original_lanes)?;
                        }
                    }
                    out["scene"]["automation"] = json!(remaining);
                    let dormant_tracks = original_tracks
                        .iter()
                        .filter(|t| same_track(t))
                        .cloned()
                        .collect::<Vec<_>>();
                    out["scene"]["propertyTracks"] = json!(
                        original_tracks
                            .iter()
                            .filter(|t| !same_track(t))
                            .collect::<Vec<_>>()
                    );
                    let mut dormant_overrides = Vec::new();
                    for contribution in retention["contributions"]
                        .as_array()
                        .ok_or("retained contribution array absent")?
                    {
                        if contribution["status"] != "active"
                            || is_native_flow_contribution(contribution)?
                        {
                            continue;
                        }
                        let owned: Vec<OwnedAddress> =
                            serde_json::from_value(contribution["owned_addresses"].clone())
                                .map_err(|e| e.to_string())?;
                        if owned.iter().any(|a| a.covers(&address)) {
                            dormant_overrides.push(json!({"contribution_ref":contribution["contribution_ref"],"overrides":contribution["authored_overrides"],"takeover_overrides":contribution["authored_overrides"]}));
                        }
                    }
                    controls.push(json!({"address":address,"procedure_ref":input.procedure.procedure_ref,"parameter":r.parameter,"target":target,"authored_base":before_value,"native_base":r.native_parameter,"dormant_lanes":dormant,"suspended_lanes":suspended,"dormant_tracks":dormant_tracks,"source_basis":retention["source_basis"],"dormant_overrides":dormant_overrides,"takeover":null}));
                    controls.len() - 1
                };
                let lifetime_operation =
                    if controls[index]["takeover"]["lifetime"] == json!(lifetime) {
                        controls[index]["takeover"]
                            .get("lifetime_operation_ref")
                            .unwrap_or(&controls[index]["takeover"]["operation_ref"])
                            .clone()
                    } else {
                        json!(input.operation_ref)
                    };
                controls[index]["takeover"] = json!({"value":authored,"native_value":value,"lifetime":lifetime,"actor":input.actor_ref,"operation_ref":input.operation_ref,
                    "lifetime_operation_ref":lifetime_operation,"revision":r.document_revision.checked_add(1).ok_or("document revision exhausted")?});
                // Attribute actual native-converted material with the SAME pure
                // stable-pointer extractor, never multiply authored/native units.
                let mut after = scene.presentation.clone();
                set_path(entity_mut(&mut after, id)?, path, authored.clone())?;
                let attributed = attribute_preview(
                    &mut out,
                    scene,
                    &after,
                    input,
                    &address,
                    *lifetime == ControlLifetime::Persistent,
                )?;
                for (reference, records) in attributed {
                    if let Some(prior) = controls[index]["dormant_overrides"]
                        .as_array_mut()
                        .and_then(|a| a.iter_mut().find(|p| p["contribution_ref"] == reference))
                    {
                        prior["takeover_overrides"] = records;
                    }
                }
            }
            NativeControlAction::SetBase { .. } => {
                if position.is_some() {
                    return Err("set_base must reconcile existing manual takeover first".into());
                }
                let candidate = scene
                    .parameter_candidate
                    .as_ref()
                    .ok_or("native ParameterSet material preview is required")?;
                let authored = at_path(entity(candidate, id)?, path)?.clone();
                scalar(&authored)?;
                let mut expected = scene.presentation.clone();
                set_path(entity_mut(&mut expected, id)?, path, authored)?;
                if &expected != candidate {
                    return Err(
                        "native base preview changes material outside exact authored scalar".into(),
                    );
                }
                attribute_preview(&mut out, scene, candidate, input, &address, true)?;
            }
            NativeControlAction::Release {} | NativeControlAction::ReleaseGesture { .. } => {
                if scene.parameter_candidate.is_some() {
                    return Err(
                        "release must use exact retained baseline, not a caller preview".into(),
                    );
                }
                let index = position.ok_or("native target has no manual takeover")?;
                let control = controls[index].clone();
                if control["takeover"].is_null() || control["target"] != target {
                    return Err("native retained takeover identity differs".into());
                }
                if let NativeControlAction::ReleaseGesture {
                    takeover_operation_ref,
                } = &input.action
                {
                    nonempty(takeover_operation_ref, "gesture takeover operation")?;
                    if control["takeover"]["lifetime"] != "gesture"
                        || control["takeover"]
                            .get("lifetime_operation_ref")
                            .unwrap_or(&control["takeover"]["operation_ref"])
                            .as_str()
                            != Some(takeover_operation_ref.as_str())
                    {
                        return Err(
                            "gesture lifetime release differs from actual active takeover".into(),
                        );
                    }
                }
                let native = control["native_base"].clone();
                if retained_native.as_ref().is_some_and(|n| n != &native) {
                    return Err("shared global native baseline differs across Scenes".into());
                }
                scalar(&native["value"])?;
                retained_native = Some(native);
                validate_current_group(&control, &controls, &original_lanes)?;
                if original_lanes.iter().any(|l| l["target"] == target)
                    || original_tracks.iter().any(same_track)
                {
                    return Err("a new driver owns this target; reconcile before release".into());
                }
                let dormant = canonical_dormant_group(
                    &controls,
                    control["dormant_lanes"]
                        .as_array()
                        .ok_or("dormant lanes absent")?,
                )?;
                let mut restored = original_lanes
                    .into_iter()
                    .filter(|l| !dormant.iter().any(|d| d["id"] == l["id"]))
                    .collect::<Vec<_>>();
                restored.extend(dormant.iter().cloned());
                let mut tracks = original_tracks;
                tracks.extend(
                    control["dormant_tracks"]
                        .as_array()
                        .ok_or("dormant tracks absent")?
                        .iter()
                        .cloned(),
                );
                out["scene"]["propertyTracks"] = json!(tracks);
                let contributions = out["scene"]["procedural"]["contributions"]
                    .as_array_mut()
                    .ok_or("contribution retention absent")?;
                for prior in control["dormant_overrides"]
                    .as_array()
                    .ok_or("dormant overrides absent")?
                {
                    let c = contributions
                        .iter_mut()
                        .find(|c| c["contribution_ref"] == prior["contribution_ref"])
                        .ok_or("generated control ownership changed during takeover")?;
                    if c["status"] != "active" || is_native_flow_contribution(c)? {
                        continue;
                    }
                    if c["authored_overrides"] != prior["takeover_overrides"] {
                        return Err("authored intervention changed during takeover; reconcile before release".into());
                    }
                    c["authored_overrides"] = prior["overrides"].clone();
                }
                // Every still-controlled sibling keeps its original exact
                // group snapshot, even when it was captured after promotion.
                // Restore the group, then use the existing lane suspension
                // operation for the remaining targets at the SAME clock.
                let before_controls = controls.clone();
                controls.remove(index);
                for remaining_control in &mut controls {
                    if !is_native_scalar_control(remaining_control)? {
                        continue;
                    }
                    let baseline = canonical_dormant_group(
                        &before_controls,
                        remaining_control["dormant_lanes"]
                            .as_array()
                            .ok_or("native dormant group absent")?,
                    )?;
                    remaining_control["dormant_lanes"] = json!(baseline);
                    let remaining_target = remaining_control["target"]
                        .as_str()
                        .ok_or("native target absent")?;
                    restored = suspend_target(&restored, remaining_target)?.0;
                }
                out["scene"]["automation"] = json!(restored);
                lanes(&out)?;
            }
        }
        update_suspended_groups(&mut controls, &lanes(&out)?)?;
        reconcile_control_records(&mut controls, &out["scene"]["procedural"]["contributions"])?;
        let original_controls = retention["controls"]
            .as_array()
            .ok_or("native control array absent")?;
        let mut scene_managed = vec![address.clone()];
        for control in original_controls.iter().chain(controls.iter()) {
            let before = original_controls
                .iter()
                .find(|c| c["address"] == control["address"]);
            let after = controls.iter().find(|c| c["address"] == control["address"]);
            if before != after {
                let changed: OwnedAddress = serde_json::from_value(control["address"].clone())
                    .map_err(|e| e.to_string())?;
                if !scene_managed.contains(&changed) {
                    scene_managed.push(changed);
                }
            }
        }
        scene_managed.sort();
        managed_control_addresses.extend(scene_managed.iter().cloned());
        scene_coverage
            .push(json!({"scene_ref":scene.scene_ref,"managed_control_addresses":scene_managed}));
        out["scene"]["procedural"]["controls"] = json!(controls);
        // Scalar material stays current here. Actual existing ParameterSet owns
        // the only numerical projection after this metadata/driver change.
        changes.push(
            json!({"change":"scene_material_set","scene_ref":scene.scene_ref,"presentation":out}),
        );
    }
    actual_addresses.sort();
    managed_control_addresses.sort();
    managed_control_addresses.dedup();
    let mut declared = r.addresses.clone();
    declared.sort();
    if actual_addresses != declared {
        return Err("native control address coverage differs".into());
    }
    let native = match &input.action {
        NativeControlAction::Release {} | NativeControlAction::ReleaseGesture { .. } => {
            retained_native.ok_or("retained native baseline absent")?
        }
        _ => r.native_parameter.clone(),
    };
    changes.push(
        json!({"change":"parameter_manual","entity_ref":r.entity_ref,"parameter":r.parameter}),
    );
    changes.push(json!({"change":"parameter_set","entity_ref":r.entity_ref,"parameter":r.parameter,"value":target_value.unwrap_or(&native["value"])}));
    if matches!(
        &input.action,
        NativeControlAction::SetBase { .. }
            | NativeControlAction::Release {}
            | NativeControlAction::ReleaseGesture { .. }
    ) && !native["automation"].is_null()
    {
        changes.push(json!({"change":"parameter_automate","entity_ref":r.entity_ref,"parameter":r.parameter,"automation":native["automation"]}));
    }
    let native_edit = json!({"operation":"edit","expression_ref":r.expression_ref,"expected_revision":r.document_revision,"actor":input.actor_ref,"changes":changes});
    Ok(
        json!({"schema":"ql.procedural-control-prepared/v1","original_procedure":input.procedure,"operation_ref":input.operation_ref,"resolved_scope":actual_addresses,"managed_control_addresses":managed_control_addresses,"scene_coverage":scene_coverage,"driver_reading":r,"native_edit":native_edit,"intent_fingerprint":fingerprint(&native_edit)?,"standing":"pure native owner source preparation; protected actual Parameter/preview/CAS and normal receiving apply required"}),
    )
}

/// Actual accepted normal Edit candidate. Root supplies both native Parameters
/// and every canonical Scene before/candidate through its private source worker;
/// this pure source preflight owns no converter, driver, field clock or live grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeActiveControlRefresh {
    pub procedure: Procedure,
    pub reading: NativeParameterDriverReading,
    pub candidate_native_parameter: Value,
    pub actor_ref: String,
    pub operation_ref: String,
}
impl NativeActiveControlRefresh {
    pub fn accepted_revision(&self) -> u64 {
        self.reading.document_revision.saturating_add(1)
    }
}

/// Preserve original controlled-target baseline while carrying newer unrelated
/// human interventions into release. No path/coordinate interpreter is mirrored
/// in the host: these are the canonical actual source-owned attributed rows.
fn refresh_dormant_records(
    original: &Value,
    current: &Value,
    addresses: &[OwnedAddress],
    controlled_pointer: &str,
    contribution_ref: &str,
) -> Result<Value> {
    let old: Vec<NativeAuthoredIntervention> =
        serde_json::from_value(original.clone()).map_err(|e| e.to_string())?;
    let new: Vec<NativeAuthoredIntervention> =
        serde_json::from_value(current.clone()).map_err(|e| e.to_string())?;
    let overlaps = |row: &NativeAuthoredIntervention| {
        addresses
            .iter()
            .any(|address| row.address.covers(address) || address.covers(&row.address))
    };
    let mut retained = Vec::new();
    for row in old.into_iter().filter(|row| overlaps(row)) {
        let next = new
            .iter()
            .find(|next| next.path == row.path && next.address == row.address);
        if controlled_pointer.starts_with(&format!("{}/", row.path)) {
            let next = next.ok_or("native current persistent parent intervention disappeared")?;
            retained.push(restore_controlled_parent_baseline(
                &row,
                next,
                controlled_pointer,
                contribution_ref,
            )?);
        } else {
            retained.push(row);
        }
    }
    retained.extend(new.into_iter().filter(|r| !overlaps(r)));
    serde_json::to_value(retained).map_err(|e| e.to_string())
}

fn control_path(control: &Value, address: &OwnedAddress) -> Result<&'static str> {
    let scene = address
        .scene_ref
        .as_deref()
        .ok_or("native control Scene absent")?;
    let entity = address
        .entity_ref
        .as_deref()
        .ok_or("native control Entity absent")?;
    let candidates = [
        "force_strength",
        "force_spin",
        "force_radius",
        "x",
        "y",
        "z",
        "scale",
        "rotation",
    ];
    let parameter = candidates
        .into_iter()
        .find(|parameter| {
            control
                .get("parameter")
                .is_none_or(|p| p.as_str() == Some(*parameter))
                && native_parameter_address(&address.expression_ref, scene, entity, parameter)
                    .is_ok_and(|actual| actual == *address)
        })
        .ok_or("retained control has no exact native scalar coordinate")?;
    let (path, suffix) = local_path(parameter)?;
    let canonical_target = format!("entity:{}:{suffix}", encode_component(entity));
    if control["target"] != canonical_target {
        return Err(
            "native retained control target differs from its exact Registry coordinate".into(),
        );
    }
    Ok(path)
}
fn is_native_scalar_control(control: &Value) -> Result<bool> {
    let native_base = control.get("native_base").is_some();
    let native_value = control["takeover"].get("native_value").is_some();
    if native_base != native_value {
        return Err("retained native control lost its paired Parameter baseline/value".into());
    }
    // Existing authored Field/non-Parameter controls have neither native fact.
    // Their retained bytes continue unchanged; no native lifetime/value is
    // inferred from an authored driver or its control address.
    Ok(native_base)
}

/// Every remaining control observes the SAME canonical contribution rows.
/// Preserve only its own dormant leaf; current sibling interventions and parent
/// bodies continue through either release order. No native unit conversion.
fn reconcile_control_records(controls: &mut [Value], contributions: &Value) -> Result<()> {
    let contributions = contributions
        .as_array()
        .ok_or("actual contribution array absent")?;
    for control in controls {
        if !is_native_scalar_control(control)? {
            continue;
        }
        let address: OwnedAddress =
            serde_json::from_value(control["address"].clone()).map_err(|e| e.to_string())?;
        let path = control_path(control, &address)?;
        let pointer =
            native_control_material_pointer(address.entity_ref.as_deref().unwrap(), path)?;
        let dormant = control["dormant_overrides"]
            .as_array_mut()
            .ok_or("native dormant contribution rows absent")?;
        for prior in dormant {
            let reference = prior["contribution_ref"]
                .as_str()
                .ok_or("dormant contribution identity absent")?
                .to_owned();
            let current = contributions
                .iter()
                .find(|c| c["contribution_ref"] == reference)
                .ok_or("native dormant contribution disappeared")?;
            if current["status"] != "active" || is_native_flow_contribution(current)? {
                continue;
            }
            prior["overrides"] = refresh_dormant_records(
                &prior["overrides"],
                &current["authored_overrides"],
                std::slice::from_ref(&address),
                &pointer,
                &reference,
            )?;
            prior["takeover_overrides"] = current["authored_overrides"].clone();
        }
    }
    Ok(())
}

pub fn refresh_native_active_control(input: &NativeActiveControlRefresh) -> Result<Value> {
    refresh_native_active_control_inner(input, false)
}
fn refresh_native_active_control_inner(
    input: &NativeActiveControlRefresh,
    joined: bool,
) -> Result<Value> {
    let r = &input.reading;
    input.procedure.validate(native_current_m_registry())?;
    nonempty(&input.actor_ref, "actual ordinary edit actor")?;
    nonempty(&input.operation_ref, "actual ordinary edit operation")?;
    let candidate_parameter = input
        .candidate_native_parameter
        .as_object()
        .ok_or("actual candidate native Parameter absent")?;
    if candidate_parameter
        .keys()
        .any(|k| !matches!(k.as_str(), "value" | "automation"))
        || !candidate_parameter.contains_key("value")
        || !input.candidate_native_parameter["automation"].is_null()
        || !r.native_parameter["automation"].is_null()
        || r.document_revision == u64::MAX
    {
        return Err("active manual refresh cannot introduce native automation or unknown Parameter fields; reconcile driver explicitly".into());
    }
    scalar(&input.candidate_native_parameter["value"])?;
    let (path, suffix) = local_path(&r.parameter)?;
    let mut lifetime = None;
    let mut scalar_reading = r.clone();
    for scene in &mut scalar_reading.scenes {
        let id = scene
            .entity_refs
            .get(&r.entity_ref)
            .filter(|id| *id == &r.entity_ref)
            .ok_or("active refresh lacks exact canonical native Entity correspondence")?;
        let address = native_parameter_address(
            &r.expression_ref,
            &scene.scene_ref,
            &r.entity_ref,
            &r.parameter,
        )?;
        let control = scene.presentation["scene"]["procedural"]["controls"]
            .as_array()
            .and_then(|controls| controls.iter().find(|c| c["address"] == json!(address)))
            .ok_or("active refresh target has no actual retained manual control")?;
        if control["takeover"].is_null()
            || control["takeover"]["native_value"] != r.native_parameter["value"]
            || control["native_base"].as_object().is_none_or(|p| {
                !p.contains_key("value")
                    || p.keys()
                        .any(|k| !matches!(k.as_str(), "value" | "automation"))
            })
            || !control["native_base"]["value"]
                .as_f64()
                .is_some_and(f64::is_finite)
            || [
                "dormant_lanes",
                "suspended_lanes",
                "dormant_tracks",
                "dormant_overrides",
            ]
            .iter()
            .any(|key| control[*key].as_array().is_none())
        {
            return Err("active refresh lost current native takeover/value".into());
        }
        let this: ControlLifetime = serde_json::from_value(control["takeover"]["lifetime"].clone())
            .map_err(|e| e.to_string())?;
        if lifetime.is_some_and(|previous| previous != this) {
            return Err("shared active takeover lifetime differs across actual Scenes".into());
        }
        lifetime = Some(this);
        let candidate = scene
            .parameter_candidate
            .as_ref()
            .ok_or("actual FULL normal Edit material candidate absent")?;
        if !joined {
            for other in scene.presentation["scene"]["procedural"]["controls"]
                .as_array()
                .ok_or("actual control array absent")?
            {
                if !is_native_scalar_control(other)? {
                    continue;
                }
                let other_address: OwnedAddress =
                    serde_json::from_value(other["address"].clone()).map_err(|e| e.to_string())?;
                if other_address == address {
                    continue;
                }
                let other_path = control_path(other, &other_address)?;
                let other_entity = other_address
                    .entity_ref
                    .as_deref()
                    .ok_or("actual other control Entity absent")?;
                if at_path(entity(&scene.presentation, other_entity)?, other_path)?
                    != at_path(entity(candidate, other_entity)?, other_path)?
                {
                    return Err("single active refresh omits another changed target; joined refresh required".into());
                }
            }
        }
        if candidate["schema"] != "oi.journey-scene/v1"
            || candidate["scene"]["id"] != scene.scene_ref
            || candidate["scene"]["procedural"] != scene.presentation["scene"]["procedural"]
        {
            return Err(
                "ordinary active refresh cannot forge retained source/control/journal metadata"
                    .into(),
            );
        }
        let target = format!("entity:{}:{suffix}", encode_component(id));
        let bind = format!("entity.{path}");
        if lanes(candidate)?
            .iter()
            .any(|lane| lane["target"] == target)
            || candidate["scene"]["propertyTracks"]
                .as_array()
                .is_some_and(|tracks| {
                    tracks
                        .iter()
                        .any(|track| track["bind"] == bind && track["entityId"] == *id)
                })
        {
            return Err("ordinary active refresh introduces an authored driver on the controlled target; reconcile explicitly".into());
        }
        let authored = at_path(entity(candidate, id)?, path)?.clone();
        scalar(&authored)?;
        let mut scalar_candidate = scene.presentation.clone();
        set_path(entity_mut(&mut scalar_candidate, id)?, path, authored)?;
        scene.parameter_candidate = Some(scalar_candidate);
    }
    let lifetime = lifetime.ok_or("active refresh has no current owned manifestation")?;
    // Reuse the actual native control/source validator and dormant driver
    // preservation. Its numerical operations are excluded from this reply;
    // Root applies the ORIGINAL normal Edit exactly once at accepted CAS.
    let control_input = NativeControlInput {
        procedure: input.procedure.clone(),
        reading: scalar_reading,
        actor_ref: input.actor_ref.clone(),
        operation_ref: input.operation_ref.clone(),
        action: NativeControlAction::Takeover {
            value: input.candidate_native_parameter["value"].clone(),
            lifetime,
        },
    };
    let prepared = prepare_native_control(&control_input)?;
    let mut changes = Vec::new();
    for scene in &r.scenes {
        let address = native_parameter_address(
            &r.expression_ref,
            &scene.scene_ref,
            &r.entity_ref,
            &r.parameter,
        )?;
        let projected = prepared["native_edit"]["changes"]
            .as_array()
            .and_then(|a| {
                a.iter().find(|c| {
                    c["change"] == "scene_material_set" && c["scene_ref"] == scene.scene_ref
                })
            })
            .ok_or("native active refresh lost exact prepared Scene metadata")?;
        let mut out = scene
            .parameter_candidate
            .clone()
            .ok_or("actual accepted material candidate disappeared")?;
        out["scene"]["procedural"] = projected["presentation"]["scene"]["procedural"].clone();
        let mut controls = out["scene"]["procedural"]["controls"]
            .as_array()
            .ok_or("prepared active controls absent")?
            .clone();
        let control = controls
            .iter_mut()
            .find(|c| c["address"] == json!(address))
            .ok_or("prepared active target changed")?;
        let contributions = out["scene"]["procedural"]["contributions"]
            .as_array_mut()
            .ok_or("actual retained contributions absent")?;
        for c in contributions {
            if c["status"] != "active" || is_native_flow_contribution(c)? {
                continue;
            }
            let owned: Vec<OwnedAddress> =
                serde_json::from_value(c["owned_addresses"].clone()).map_err(|e| e.to_string())?;
            if !owned.iter().any(|a| a.covers(&address)) {
                continue;
            }
            let prior = scene.presentation["scene"]["procedural"]["contributions"]
                .as_array()
                .and_then(|a| {
                    a.iter()
                        .find(|old| old["contribution_ref"] == c["contribution_ref"])
                })
                .ok_or("active refresh contribution identity changed")?;
            let records: Vec<NativeAuthoredIntervention> =
                serde_json::from_value(prior["authored_overrides"].clone())
                    .map_err(|e| e.to_string())?;
            let local = records
                .iter()
                .filter(|row| row.address.scene_ref.as_ref() == Some(&scene.scene_ref))
                .cloned()
                .collect::<Vec<_>>();
            let foreign = records
                .into_iter()
                .filter(|row| row.address.scene_ref.as_ref() != Some(&scene.scene_ref))
                .collect::<Vec<_>>();
            let edit = ManualEdit {
                expression_ref: r.expression_ref.clone(),
                scene_ref: scene.scene_ref.clone(),
                contribution_ref: c["contribution_ref"]
                    .as_str()
                    .ok_or("active refresh contribution ref absent")?
                    .into(),
                owned_addresses: owned,
                entity_refs: scene.entity_refs.clone(),
                before: scene.presentation.clone(),
                after: scene.parameter_candidate.clone().unwrap(),
                actor_ref: input.actor_ref.clone(),
                operation_ref: input.operation_ref.clone(),
                document_revision: r.document_revision + 1,
                retained_overlays: vec![],
                retained_native_records: local,
            };
            let mut records = extract_owned_manual_interventions(&edit)?.native_records;
            if lifetime == ControlLifetime::Gesture {
                for record in &mut records {
                    if record.revision == edit.document_revision
                        && record.operation_ref.as_deref() == Some(input.operation_ref.as_str())
                        && r.addresses
                            .iter()
                            .any(|a| record.address.covers(a) || a.covers(&record.address))
                    {
                        record.persistent = false;
                    }
                }
            }
            records.extend(foreign);
            let current = json!(records);
            let dormant = control["dormant_overrides"]
                .as_array_mut()
                .and_then(|a| {
                    a.iter_mut()
                        .find(|d| d["contribution_ref"] == c["contribution_ref"])
                })
                .ok_or("active refresh original dormant contribution disappeared")?;
            dormant["overrides"] = refresh_dormant_records(
                &dormant["overrides"],
                &current,
                &r.addresses,
                &native_control_material_pointer(&r.entity_ref, path)?,
                c["contribution_ref"]
                    .as_str()
                    .ok_or("native dormant contribution ref absent")?,
            )?;
            dormant["takeover_overrides"] = current.clone();
            c["authored_overrides"] = current;
        }
        reconcile_control_records(&mut controls, &out["scene"]["procedural"]["contributions"])?;
        out["scene"]["procedural"]["controls"] = json!(controls);
        changes.push(
            json!({"change":"scene_material_set","scene_ref":scene.scene_ref,"presentation":out}),
        );
    }
    let native_edit = json!({"operation":"edit","expression_ref":r.expression_ref,"expected_revision":r.document_revision,
        "actor":input.actor_ref,"changes":changes});
    Ok(
        json!({"schema":"ql.procedural-active-control-refresh/v1","original_procedure":input.procedure,
        "operation_ref":input.operation_ref,"accepted_revision":r.document_revision+1,"resolved_scope":r.addresses,
        "driver_reading":r,"candidate_native_parameter":input.candidate_native_parameter,"native_edit":native_edit,
        "intent_fingerprint":fingerprint(&native_edit)?,"standing":"Source-owned current manual-control/provenance refresh over actual normal Edit candidate; Root re-attests all native reads/candidates and one accepted CAS; no parameter conversion/driver reset/field ACK"}),
    )
}

/// One bounded pure preflight for all affected controls in one normal native
/// Edit. The original Edit is applied once by the protected receiver; this reply
/// contains metadata-only Scene changes and exact authoritative row coverage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeActiveControlsSceneCoverage {
    pub scene_ref: String,
    pub managed_contribution_refs: Vec<String>,
    pub managed_control_addresses: Vec<OwnedAddress>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeActiveControlsRefreshResult {
    pub schema: String,
    pub original_procedures: Vec<Procedure>,
    pub operation_ref: String,
    pub accepted_revision: u64,
    pub resolved_scope: Vec<OwnedAddress>,
    pub managed_contribution_refs: Vec<String>,
    pub managed_control_addresses: Vec<OwnedAddress>,
    pub scene_coverage: Vec<NativeActiveControlsSceneCoverage>,
    /// Exact Source preparation over the ordinary candidate. The protected
    /// receiver merges named metadata into its original ONE native Edit.
    pub native_edit: Value,
    pub intent_fingerprint: String,
    pub standing: String,
}
pub fn refresh_native_active_controls(inputs: &[NativeActiveControlRefresh]) -> Result<Value> {
    if inputs.is_empty()
        || inputs.len() > 64
        || serde_json::to_vec(inputs).map_err(|e| e.to_string())?.len() > 8 * 1024 * 1024
    {
        return Err("joined active-control refresh count/byte bounds exceeded".into());
    }
    let first = &inputs[0];
    let expression = &first.reading.expression_ref;
    let revision = first.reading.document_revision;
    let mut keys = BTreeSet::new();
    let mut definitions = BTreeMap::new();
    let mut scenes: BTreeMap<String, NativeDriverScene> = BTreeMap::new();
    let mut projections = BTreeMap::new();
    let mut resolved = BTreeSet::new();
    for input in inputs {
        if &input.reading.expression_ref != expression
            || input.reading.document_revision != revision
            || input.actor_ref != first.actor_ref
            || input.operation_ref != first.operation_ref
        {
            return Err(
                "joined active controls cross actual Expression/CAS/actor/operation".into(),
            );
        }
        if !keys.insert((
            input.reading.entity_ref.clone(),
            input.reading.parameter.clone(),
        )) {
            return Err("duplicate global native active-control target".into());
        }
        if definitions
            .insert(
                input.procedure.procedure_ref.clone(),
                input.procedure.clone(),
            )
            .is_some_and(|old| old != input.procedure)
        {
            return Err("joined controls relabel original procedure definition".into());
        }
        let prepared = refresh_native_active_control_inner(input, true)?;
        for scene in &input.reading.scenes {
            if let Some(old) = scenes.get(&scene.scene_ref) {
                if old.presentation != scene.presentation
                    || old.parameter_candidate != scene.parameter_candidate
                    || old.entity_refs != scene.entity_refs
                {
                    return Err("joined controls disagree on exact full Scene before/candidate/correspondence".into());
                }
            } else {
                scenes.insert(scene.scene_ref.clone(), scene.clone());
            }
            let address = native_parameter_address(
                expression,
                &scene.scene_ref,
                &input.reading.entity_ref,
                &input.reading.parameter,
            )?;
            let projected = prepared["native_edit"]["changes"]
                .as_array()
                .and_then(|a| a.iter().find(|c| c["scene_ref"] == scene.scene_ref))
                .and_then(|c| c["presentation"]["scene"]["procedural"]["controls"].as_array())
                .and_then(|a| a.iter().find(|c| c["address"] == json!(address)))
                .ok_or("joined Source projection lost its exact native control")?;
            projections.insert(address.clone(), projected.clone());
            resolved.insert(address);
        }
    }
    let mut changes = Vec::new();
    let mut coverage = Vec::new();
    let mut managed_refs = BTreeSet::new();
    let mut managed_controls = BTreeSet::new();
    for (scene_ref, scene) in scenes {
        let candidate = scene
            .parameter_candidate
            .as_ref()
            .ok_or("joined full candidate absent")?;
        let mut out = candidate.clone();
        let mut controls = scene.presentation["scene"]["procedural"]["controls"]
            .as_array()
            .ok_or("joined current native controls absent")?
            .clone();
        let mut gestures = Vec::new();
        let mut local_targets = Vec::new();
        for control in &mut controls {
            if !is_native_scalar_control(control)? {
                continue;
            }
            let address: OwnedAddress =
                serde_json::from_value(control["address"].clone()).map_err(|e| e.to_string())?;
            let path = control_path(control, &address)?;
            let entity_ref = address
                .entity_ref
                .as_deref()
                .ok_or("joined native Entity absent")?;
            if let Some(projected) = projections.get(&address) {
                *control = projected.clone();
                if control["takeover"]["lifetime"] == "gesture" {
                    gestures.push(address.clone());
                }
                local_targets.push(address);
            } else if at_path(entity(&scene.presentation, entity_ref)?, path)?
                != at_path(entity(candidate, entity_ref)?, path)?
            {
                return Err("joined refresh omits another changed active native target".into());
            }
        }
        let mut contributions = scene.presentation["scene"]["procedural"]["contributions"]
            .as_array()
            .ok_or("joined retained contribution array absent")?
            .clone();
        let mut local_refs = BTreeSet::new();
        for c in &mut contributions {
            if c["status"] != "active" || is_native_flow_contribution(c)? {
                continue;
            }
            let owned: Vec<OwnedAddress> =
                serde_json::from_value(c["owned_addresses"].clone()).map_err(|e| e.to_string())?;
            if !owned
                .iter()
                .any(|a| local_targets.iter().any(|t| a.covers(t)))
            {
                continue;
            }
            let reference = c["contribution_ref"]
                .as_str()
                .ok_or("joined contribution identity absent")?
                .to_owned();
            if !local_refs.insert(reference.clone()) {
                return Err("duplicate joined contribution identity in one Scene".into());
            }
            let old = records(&c["authored_overrides"])?;
            let edit = ManualEdit {
                expression_ref: expression.clone(),
                scene_ref: scene_ref.clone(),
                contribution_ref: reference.clone(),
                owned_addresses: owned,
                entity_refs: scene.entity_refs.clone(),
                before: scene.presentation.clone(),
                after: candidate.clone(),
                actor_ref: first.actor_ref.clone(),
                operation_ref: first.operation_ref.clone(),
                document_revision: revision
                    .checked_add(1)
                    .ok_or("native document revision exhausted")?,
                retained_overlays: vec![],
                retained_native_records: old
                    .iter()
                    .filter(|r| r.address.scene_ref.as_ref() == Some(&scene_ref))
                    .cloned()
                    .collect(),
            };
            let mut next = extract_owned_manual_interventions(&edit)?.native_records;
            for row in &mut next {
                if row.revision == edit.document_revision
                    && row.operation_ref.as_deref() == Some(first.operation_ref.as_str())
                    && gestures
                        .iter()
                        .any(|a| row.address.covers(a) || a.covers(&row.address))
                {
                    row.persistent = false;
                }
            }
            next.extend(
                old.into_iter()
                    .filter(|r| r.address.scene_ref.as_ref() != Some(&scene_ref)),
            );
            c["authored_overrides"] = json!(next);
            managed_refs.insert(reference);
        }
        // Every affected retained control, including an unchanged sibling, must
        // keep current rows and its original controlled baseline together.
        let old_controls = scene.presentation["scene"]["procedural"]["controls"]
            .as_array()
            .unwrap();
        reconcile_control_records(&mut controls, &json!(contributions))?;
        let mut local_controls = BTreeSet::new();
        for control in &controls {
            let address: OwnedAddress =
                serde_json::from_value(control["address"].clone()).map_err(|e| e.to_string())?;
            if projections.contains_key(&address)
                || old_controls
                    .iter()
                    .find(|old| old["address"] == control["address"])
                    .is_some_and(|old| old != control)
            {
                local_controls.insert(address.clone());
                managed_controls.insert(address);
            }
        }
        out["scene"]["procedural"]["contributions"] = json!(contributions);
        out["scene"]["procedural"]["controls"] = json!(controls);
        // All actual ordinary candidate material outside metadata is identical.
        coverage.push(NativeActiveControlsSceneCoverage {
            scene_ref: scene_ref.clone(),
            managed_contribution_refs: local_refs.into_iter().collect(),
            managed_control_addresses: local_controls.into_iter().collect(),
        });
        changes
            .push(json!({"change":"scene_material_set","scene_ref":scene_ref,"presentation":out}));
    }
    let native_edit = json!({"operation":"edit","expression_ref":expression,"expected_revision":revision,
        "actor":first.actor_ref,"changes":changes});
    let intent_fingerprint = fingerprint(&native_edit)?;
    serde_json::to_value(NativeActiveControlsRefreshResult {
        schema: "ql.procedural-active-controls-refresh/v1".into(),
        original_procedures: definitions.into_values().collect(), operation_ref: first.operation_ref.clone(),
        accepted_revision: revision+1, resolved_scope: resolved.into_iter().collect(),
        managed_contribution_refs: managed_refs.into_iter().collect(),
        managed_control_addresses: managed_controls.into_iter().collect(), scene_coverage: coverage,
        native_edit, intent_fingerprint,
        standing: "pure joined Source attribution of one actual normal Edit candidate; protected native read/candidate and one CAS required; no driver conversion, reset or field ACK".into(),
    }).map_err(|e| e.to_string())
}
