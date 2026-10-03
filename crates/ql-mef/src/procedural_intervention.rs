//! Attribute an accepted native before/after human edit to exact stable targets.
//! The receiving owner supplies actor/CAS/material. This computes no authority,
//! writes no journal, and keeps native OwnedAddress identical to StageAddress.
use crate::procedural_composition::{AuthoredOverlay, OverlayOperation, OwnedAddress, Result};
use crate::procedural_manifestation::{nonempty, validate_material};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManualEdit {
    pub expression_ref: String,
    pub scene_ref: String,
    pub contribution_ref: String,
    pub owned_addresses: Vec<OwnedAddress>,
    pub entity_refs: BTreeMap<String, String>,
    pub before: Value,
    pub after: Value,
    pub actor_ref: String,
    pub operation_ref: String,
    pub document_revision: u64,
    pub retained_overlays: Vec<AuthoredOverlay>,
    #[serde(default)]
    pub retained_native_records: Vec<NativeAuthoredIntervention>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterventionKind {
    Set,
    Create,
    Delete,
    Reorder,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeIntervention {
    pub contribution_ref: String,
    pub address: OwnedAddress,
    pub pointer: String,
    pub kind: InterventionKind,
    pub before: Option<Value>,
    pub value: Option<Value>,
    pub actor_ref: String,
    pub operation_ref: String,
    pub document_revision: u64,
    pub persistent: bool,
}
/// The actual receiving owner's attributed row stored under a contribution's
/// authored_overrides. Native readback supplies these rows; they are not a
/// caller permission or an observed physical/runtime control receipt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeAuthoredIntervention {
    pub address: OwnedAddress,
    pub actor: String,
    pub persistent: bool,
    pub revision: u64,
    pub path: String,
    pub value: Value,
    pub kind: InterventionKind,
    pub operation_ref: Option<String>,
    pub before: Option<Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManualInterventions {
    pub schema: String,
    pub interventions: Vec<NativeIntervention>,
    pub native_records: Vec<NativeAuthoredIntervention>,
    pub overlays: Vec<AuthoredOverlay>,
}
fn escaped(part: &str) -> String {
    part.replace('~', "~0").replace('/', "~1")
}
fn pointer(parts: &[String]) -> String {
    format!(
        "/{}",
        parts
            .iter()
            .map(|p| escaped(p))
            .collect::<Vec<_>>()
            .join("/")
    )
}
fn overlap(a: &str, b: &str) -> bool {
    a == b || a.starts_with(&format!("{b}/")) || b.starts_with(&format!("{a}/"))
}
fn current_at<'a>(root: &'a Value, parts: &[String]) -> Option<&'a Value> {
    let mut value = root;
    for part in parts {
        value = match value {
            Value::Object(o) => o.get(part)?,
            Value::Array(a) => {
                let id = part.strip_prefix('@')?;
                a.iter().find(|v| v["id"].as_str() == Some(id))?
            }
            _ => return None,
        };
    }
    Some(value)
}
fn decoded_pointer(value: &str) -> Result<Vec<String>> {
    if !value.starts_with('/') || value.len() > 4096 {
        return Err("invalid stable authored pointer".into());
    }
    value[1..]
        .split('/')
        .map(|raw| {
            let mut chars = raw.chars();
            let mut part = String::new();
            while let Some(ch) = chars.next() {
                if ch == '~' {
                    part.push(match chars.next() {
                        Some('0') => '~',
                        Some('1') => '/',
                        _ => return Err("invalid pointer escaping".into()),
                    });
                } else {
                    part.push(ch);
                }
            }
            Ok(part)
        })
        .collect()
}
fn keyed(values: &[Value]) -> Result<Option<BTreeMap<String, &Value>>> {
    if values.iter().all(|v| v["id"].is_string()) {
        let mut map = BTreeMap::new();
        for v in values {
            let id = v["id"].as_str().unwrap().to_owned();
            if map.insert(id, v).is_some() {
                return Err("ambiguous native constituent identity".into());
            }
        }
        Ok(Some(map))
    } else {
        Ok(None)
    }
}
fn deltas(
    before: Option<&Value>,
    after: Option<&Value>,
    path: &mut Vec<String>,
    out: &mut Vec<(Vec<String>, InterventionKind, Option<Value>, Option<Value>)>,
) -> Result<()> {
    if before == after || path.get(1).is_some_and(|p| p == "procedural") {
        return Ok(());
    }
    if out.len() >= 2048 {
        return Err("native intervention bound exceeded".into());
    }
    match (before, after) {
        (Some(Value::Object(a)), Some(Value::Object(b))) => {
            for key in a.keys().chain(b.keys()).collect::<BTreeSet<_>>() {
                path.push(key.clone());
                deltas(a.get(key), b.get(key), path, out)?;
                path.pop();
            }
        }
        (Some(Value::Array(a)), Some(Value::Array(b))) => {
            if let (Some(am), Some(bm)) = (keyed(a)?, keyed(b)?) {
                for id in am.keys().chain(bm.keys()).collect::<BTreeSet<_>>() {
                    path.push(format!("@{id}"));
                    deltas(am.get(id).copied(), bm.get(id).copied(), path, out)?;
                    path.pop();
                }
                let old = a
                    .iter()
                    .filter_map(|v| v["id"].as_str())
                    .filter(|id| bm.contains_key(*id))
                    .collect::<Vec<_>>();
                let new = b
                    .iter()
                    .filter_map(|v| v["id"].as_str())
                    .filter(|id| am.contains_key(*id))
                    .collect::<Vec<_>>();
                if old != new {
                    out.push((
                        path.clone(),
                        InterventionKind::Reorder,
                        Some(Value::Array(
                            old.into_iter().map(|id| Value::String(id.into())).collect(),
                        )),
                        Some(Value::Array(
                            new.into_iter().map(|id| Value::String(id.into())).collect(),
                        )),
                    ));
                }
            } else {
                out.push((
                    path.clone(),
                    InterventionKind::Set,
                    before.cloned(),
                    after.cloned(),
                ));
            }
        }
        _ => out.push((
            path.clone(),
            if before.is_none() {
                InterventionKind::Create
            } else if after.is_none() {
                InterventionKind::Delete
            } else {
                InterventionKind::Set
            },
            before.cloned(),
            after.cloned(),
        )),
    }
    Ok(())
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneInterventionBasis {
    pub expression_ref: String,
    pub scene_ref: String,
    pub contribution_ref: String,
    pub owned_addresses: Vec<OwnedAddress>,
    pub entity_refs: BTreeMap<String, String>,
    pub current_presentation: Value,
    pub document_revision: u64,
    pub retained_native_records: Vec<NativeAuthoredIntervention>,
}
trait SceneCoordinates {
    fn expression_ref(&self) -> &str;
    fn scene_ref(&self) -> &str;
    fn entity_refs(&self) -> &BTreeMap<String, String>;
}
impl SceneCoordinates for ManualEdit {
    fn expression_ref(&self) -> &str {
        &self.expression_ref
    }
    fn scene_ref(&self) -> &str {
        &self.scene_ref
    }
    fn entity_refs(&self) -> &BTreeMap<String, String> {
        &self.entity_refs
    }
}
impl SceneCoordinates for SceneInterventionBasis {
    fn expression_ref(&self) -> &str {
        &self.expression_ref
    }
    fn scene_ref(&self) -> &str {
        &self.scene_ref
    }
    fn entity_refs(&self) -> &BTreeMap<String, String> {
        &self.entity_refs
    }
}
fn address(
    edit: &impl SceneCoordinates,
    path: &[String],
    _kind: &InterventionKind,
) -> Result<OwnedAddress> {
    let mut result = OwnedAddress {
        expression_ref: edit.expression_ref().to_owned(),
        scene_ref: Some(edit.scene_ref().to_owned()),
        entity_ref: None,
        component: "scene".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    };
    let mut index = 1;
    if path.get(index).map(String::as_str) == Some("entities") {
        index += 1;
        if let Some(id) = path.get(index).and_then(|p| p.strip_prefix('@')) {
            result.entity_ref = Some(
                edit.entity_refs()
                    .get(id)
                    .cloned()
                    .or_else(|| {
                        id.starts_with(&format!("{}:entity:", edit.expression_ref()))
                            .then(|| id.into())
                    })
                    .ok_or("native entity occurrence map is missing")?,
            );
            result.component = "entity".into();
            index += 1;
            match path.get(index).map(String::as_str) {
                Some("force") => {
                    result.component = "force".into();
                    index += 1;
                }
                Some("layers") => {
                    index += 1;
                    if let Some(id) = path.get(index).and_then(|p| p.strip_prefix('@')) {
                        result.component = "layer".into();
                        result.parent_ref = Some(None);
                        result.constituent_ref = Some(id.into());
                        index += 1;
                    } else {
                        index -= 1;
                    }
                }
                Some("sequence") => {
                    result.component = "sequence".into();
                    index += 1;
                    if path.get(index).map(String::as_str) == Some("steps")
                        && path
                            .get(index + 1)
                            .and_then(|p| p.strip_prefix('@'))
                            .is_some()
                    {
                        result.component = "sequence_link".into();
                        result.constituent_ref = Some(path[index + 1][1..].into());
                        index += 2;
                        if path.get(index).map(String::as_str) == Some("layers") {
                            if let Some(layer) =
                                path.get(index + 1).and_then(|p| p.strip_prefix('@'))
                            {
                                result.parent_ref = Some(result.constituent_ref.clone());
                                result.constituent_ref = Some(layer.into());
                                result.component = "layer".into();
                                index += 2;
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            index -= 1;
        }
    }
    if result.entity_ref.is_none() && result.component == "scene" && index < path.len() {
        if path[index] == "field" {
            result.component = "field".into();
            index += 1;
        } else {
            result.component = "property".into();
        }
    }
    let property = path[index..].join(".");
    if !property.is_empty() {
        result.property = Some(property);
    }
    result.validate()?;
    Ok(result)
}
/// Source-owned coordinate construction; callers supply the SAME fixed native
/// control material path, never a numerical conversion or extra write scope.
pub(crate) fn native_control_material_pointer(entity_ref: &str, path: &str) -> Result<String> {
    nonempty(entity_ref, "actual native control Entity")?;
    let mut parts = vec![
        "scene".to_owned(),
        "entities".to_owned(),
        format!("@{entity_ref}"),
    ];
    for part in path.split('.') {
        if part.is_empty() || part == "procedural" {
            return Err("invalid native authored scalar control path".into());
        }
        parts.push(part.to_owned());
    }
    Ok(pointer(&parts))
}

/// A persistent parent spans controlled and unrelated authored leaves. Keep
/// its original attribution, but carry newer sibling material from the actual
/// canonical extractor. Restore ONLY the controlled leaf from that parent's
/// original authored baseline through the SAME stable overlay application.
pub(crate) fn restore_controlled_parent_baseline(
    original: &NativeAuthoredIntervention,
    current: &NativeAuthoredIntervention,
    controlled_pointer: &str,
    contribution_ref: &str,
) -> Result<NativeAuthoredIntervention> {
    if original.address != current.address || original.path != current.path {
        return Err("native dormant parent intervention identity changed".into());
    }
    let parent = decoded_pointer(&original.path)?;
    let controlled = decoded_pointer(controlled_pointer)?;
    if controlled.len() <= parent.len() || !controlled.starts_with(&parent) {
        return Err("native control is not a strict leaf of its persistent parent".into());
    }
    if !matches!(
        original.kind,
        InterventionKind::Set | InterventionKind::Create
    ) || !matches!(
        current.kind,
        InterventionKind::Set | InterventionKind::Create
    ) {
        return Err("native dormant parent was removed/reordered; reconcile before release".into());
    }
    let relative = &controlled[parent.len()..];
    let baseline = current_at(&original.value, relative)
        .ok_or("native dormant parent lost original controlled leaf")?
        .clone();
    let mut restored = original.clone();
    restored.value = current.value.clone();
    crate::procedural_composition::apply_retained_material_overlays(
        &mut restored.value,
        contribution_ref,
        &[AuthoredOverlay {
            contribution_ref: contribution_ref.into(),
            pointer: pointer(relative),
            value: baseline,
            actor_ref: original.actor.clone(),
            persistent: true,
            operation: OverlayOperation::Set,
        }],
    )?;
    Ok(restored)
}

/// Native owner calls AFTER validating the actual authored edit, with the
/// exact pre-edit material and accepted post-edit material. Older explicit
/// overlays are updated from the new current basis, never blindly reapplied.
pub fn extract_manual_interventions(edit: &ManualEdit) -> Result<ManualInterventions> {
    extract_manual_interventions_inner(edit, false)
}
/// Protected ordinary Edit may also contain unrelated authored changes. Only
/// exact covered deltas are attributed; scope/material are never expanded.
pub fn extract_owned_manual_interventions(edit: &ManualEdit) -> Result<ManualInterventions> {
    extract_manual_interventions_inner(edit, true)
}
fn extract_manual_interventions_inner(
    edit: &ManualEdit,
    owned_only: bool,
) -> Result<ManualInterventions> {
    let mut projected = edit.clone();
    if !edit.retained_native_records.is_empty() {
        let native = project_native_authored_interventions(edit, &edit.retained_native_records)?;
        if !edit.retained_overlays.is_empty() && edit.retained_overlays != native {
            return Err("caller overlays differ from protected native stored attribution".into());
        }
        projected.retained_overlays = native;
    }
    let edit = &projected;
    for (value, name) in [
        (&edit.actor_ref, "actual edit actor"),
        (&edit.operation_ref, "actual edit operation"),
        (&edit.contribution_ref, "contribution"),
    ] {
        nonempty(value, name)?;
    }
    if edit.document_revision == 0
        || edit.before["schema"] != "oi.journey-scene/v1"
        || (!edit.after.is_null() && edit.after["schema"] != "oi.journey-scene/v1")
        || edit.before["scene"]["id"] != edit.scene_ref
        || (!edit.after.is_null() && edit.after["scene"]["id"] != edit.scene_ref)
        || edit.owned_addresses.is_empty()
    {
        return Err("manual edit lacks its exact accepted native material/CAS/owned scope".into());
    }
    for a in &edit.owned_addresses {
        a.validate()?;
        if a.expression_ref != edit.expression_ref {
            return Err("manual contribution scope is outside its accepted Scene".into());
        }
    }
    if !edit
        .owned_addresses
        .iter()
        .any(|a| a.scene_ref.as_deref() == Some(&edit.scene_ref))
    {
        return Err("manual Scene has no actual owned location".into());
    }
    for location in edit
        .owned_addresses
        .iter()
        .filter(|a| a.scene_ref.as_deref() != Some(&edit.scene_ref))
    {
        if location.entity_ref.is_none()
            || location.property.is_none()
            || location.constituent_ref.is_some()
            || location.parent_ref.is_some()
            || !matches!(location.component.as_str(), "force" | "entity" | "property")
            || !edit
                .entity_refs
                .values()
                .any(|e| Some(e) == location.entity_ref.as_ref())
            || !edit.owned_addresses.iter().any(|a| {
                a.scene_ref.as_deref() == Some(&edit.scene_ref)
                    && a.entity_ref == location.entity_ref
                    && a.component == location.component
                    && a.constituent_ref == location.constituent_ref
                    && a.property == location.property
                    && a.parent_ref == location.parent_ref
            })
        {
            return Err(
                "foreign Scene scope is not the same exact native global Entity parameter".into(),
            );
        }
    }
    validate_material(&edit.before, 0)?;
    validate_material(&edit.after, 0)?;
    if edit.after.is_null() {
        let whole = OwnedAddress {
            expression_ref: edit.expression_ref.clone(),
            scene_ref: Some(edit.scene_ref.clone()),
            entity_ref: None,
            component: "scene".into(),
            constituent_ref: None,
            parent_ref: None,
            property: None,
        };
        if !edit.owned_addresses.iter().any(|a| a.covers(&whole)) {
            return Err("Scene deletion tombstone requires actual whole Scene ownership".into());
        }
        let mut before = edit.before["scene"].clone();
        before
            .as_object_mut()
            .ok_or("deleted Scene basis is unavailable")?
            .remove("procedural");
        return Ok(ManualInterventions {
            schema: "ql.procedural-manual-interventions/v1".into(),
            interventions: vec![NativeIntervention {
                contribution_ref: edit.contribution_ref.clone(),
                address: whole.clone(),
                pointer: "/scene".into(),
                kind: InterventionKind::Delete,
                before: Some(before.clone()),
                value: None,
                actor_ref: edit.actor_ref.clone(),
                operation_ref: edit.operation_ref.clone(),
                document_revision: edit.document_revision,
                persistent: true,
            }],
            native_records: {
                let mut records = edit
                    .retained_native_records
                    .iter()
                    .filter(|r| r.path != "/scene")
                    .cloned()
                    .collect::<Vec<_>>();
                records.push(NativeAuthoredIntervention {
                    address: whole,
                    actor: edit.actor_ref.clone(),
                    persistent: true,
                    revision: edit.document_revision,
                    path: "/scene".into(),
                    value: Value::Null,
                    kind: InterventionKind::Delete,
                    operation_ref: Some(edit.operation_ref.clone()),
                    before: Some(before),
                });
                records
            },
            overlays: vec![AuthoredOverlay {
                contribution_ref: edit.contribution_ref.clone(),
                pointer: "/scene".into(),
                value: Value::Null,
                actor_ref: edit.actor_ref.clone(),
                persistent: true,
                operation: OverlayOperation::Delete,
            }],
        });
    }
    let mut raw = Vec::new();
    deltas(
        Some(&edit.before["scene"]),
        Some(&edit.after["scene"]),
        &mut vec!["scene".into()],
        &mut raw,
    )?;
    let mut interventions = Vec::new();
    let mut overlays = BTreeMap::new();
    for old in &edit.retained_overlays {
        if old.contribution_ref != edit.contribution_ref {
            return Err("retained overlay belongs to another contribution".into());
        }
        let parts = decoded_pointer(&old.pointer)?;
        if parts.first().map(String::as_str) != Some("scene")
            || parts.get(1).map(String::as_str) == Some("procedural")
        {
            return Err("authored overlay cannot address the retained native journal".into());
        }
        let native_address = address(edit, &parts, &InterventionKind::Set)?;
        if !edit
            .owned_addresses
            .iter()
            .any(|a| a.covers(&native_address))
        {
            return Err("retained manual overlay is outside its contribution scope".into());
        }
        let affected = raw.iter().any(|(path, kind, _, _)| {
            !matches!(kind, InterventionKind::Reorder) && overlap(&old.pointer, &pointer(path))
        });
        if affected && old.operation != OverlayOperation::Reorder {
            if let Some(value) = current_at(&edit.after, &parts) {
                let mut next = old.clone();
                next.value = value.clone();
                next.operation = OverlayOperation::Set;
                next.actor_ref = edit.actor_ref.clone();
                overlays.insert(next.pointer.clone(), next);
            } else if old.operation == OverlayOperation::Delete {
                overlays.insert(old.pointer.clone(), old.clone());
            }
        } else {
            overlays.insert(old.pointer.clone(), old.clone());
        }
    }
    for (parts, kind, before, value) in raw {
        let native_address = address(edit, &parts, &kind)?;
        if !edit
            .owned_addresses
            .iter()
            .any(|owned| owned.covers(&native_address))
        {
            if owned_only {
                continue;
            }
            return Err("manual edit delta is outside its contribution scope".into());
        }
        let stable_pointer = pointer(&parts);
        overlays.insert(
            stable_pointer.clone(),
            AuthoredOverlay {
                contribution_ref: edit.contribution_ref.clone(),
                pointer: stable_pointer.clone(),
                value: value.clone().unwrap_or(Value::Null),
                actor_ref: edit.actor_ref.clone(),
                persistent: true,
                operation: match kind {
                    InterventionKind::Delete => OverlayOperation::Delete,
                    InterventionKind::Reorder => OverlayOperation::Reorder,
                    _ => OverlayOperation::Set,
                },
            },
        );
        interventions.push(NativeIntervention {
            contribution_ref: edit.contribution_ref.clone(),
            address: native_address,
            pointer: stable_pointer,
            kind,
            before,
            value,
            actor_ref: edit.actor_ref.clone(),
            operation_ref: edit.operation_ref.clone(),
            document_revision: edit.document_revision,
            persistent: true,
        });
    }
    let mut native_records = interventions
        .iter()
        .map(|record| NativeAuthoredIntervention {
            address: record.address.clone(),
            actor: record.actor_ref.clone(),
            persistent: record.persistent,
            revision: record.document_revision,
            path: record.pointer.clone(),
            value: record.value.clone().unwrap_or(Value::Null),
            kind: record.kind.clone(),
            operation_ref: Some(record.operation_ref.clone()),
            before: record.before.clone(),
        })
        .collect::<Vec<_>>();
    // A newer child edit also refreshes an older explicit parent override from
    // actual current material. Retaining that parent's old value would undo the
    // accepted edit on regeneration. Preserve all other old attributed rows.
    for old in &edit.retained_overlays {
        if old.operation == OverlayOperation::Reorder
            || native_records.iter().any(|r| r.path == old.pointer)
        {
            continue;
        }
        if interventions
            .iter()
            .any(|r| overlap(&r.pointer, &old.pointer))
        {
            if let Some(next) = overlays.get(&old.pointer) {
                if next.actor_ref == edit.actor_ref && next.operation == OverlayOperation::Set {
                    let parts = decoded_pointer(&old.pointer)?;
                    native_records.push(NativeAuthoredIntervention {
                        address: address(edit, &parts, &InterventionKind::Set)?,
                        actor: edit.actor_ref.clone(),
                        persistent: next.persistent,
                        revision: edit.document_revision,
                        path: old.pointer.clone(),
                        value: next.value.clone(),
                        kind: InterventionKind::Set,
                        operation_ref: Some(edit.operation_ref.clone()),
                        before: current_at(&edit.before, &parts).cloned(),
                    });
                }
            }
        }
    }
    for retained in &edit.retained_native_records {
        if !native_records.iter().any(|r| r.path == retained.path) {
            native_records.push(retained.clone());
        }
    }
    Ok(ManualInterventions {
        native_records,
        schema: "ql.procedural-manual-interventions/v1".into(),
        interventions,
        overlays: overlays.into_values().collect(),
    })
}

/// Convert only protected native stored attribution into source overlay wire.
/// `context.before/after`, entity occurrence map, contribution and CAS come from
/// the same actual Scene read. The receiver re-attests this context and rows.
/// Missing stable paths or mismatched coordinates are refused, not guessed from
/// array positions or arbitrary provenance strings. Later generation consumes
/// these overlays together with actual current material; neither is authority.
pub fn project_native_authored_interventions(
    context: &ManualEdit,
    records: &[NativeAuthoredIntervention],
) -> Result<Vec<AuthoredOverlay>> {
    project_scene_intervention_basis(&SceneInterventionBasis {
        expression_ref: context.expression_ref.clone(),
        scene_ref: context.scene_ref.clone(),
        contribution_ref: context.contribution_ref.clone(),
        owned_addresses: context.owned_addresses.clone(),
        entity_refs: context.entity_refs.clone(),
        current_presentation: context.before.clone(),
        document_revision: context.document_revision,
        retained_native_records: records.to_vec(),
    })
}
/// Source projection of actual stored records/current basis without inventing
/// a new actor or ordinary Edit. Receiving host re-attests this basis at CAS.
pub fn project_scene_intervention_basis(
    context: &SceneInterventionBasis,
) -> Result<Vec<AuthoredOverlay>> {
    let records = &context.retained_native_records;
    if records.len() > 2048
        || context.document_revision == 0
        || context.current_presentation["schema"] != "oi.journey-scene/v1"
        || context.current_presentation["scene"]["id"] != context.scene_ref
        || context.owned_addresses.is_empty()
    {
        return Err("stored intervention projection lacks actual native Scene/CAS/scope".into());
    }
    validate_material(&context.current_presentation, 0)?;
    let mut rows = BTreeMap::<String, &NativeAuthoredIntervention>::new();
    for record in records {
        nonempty(&record.actor, "stored intervention actor")?;
        if record.revision == 0 || record.revision > context.document_revision {
            return Err("stored intervention revision escapes actual native CAS".into());
        }
        if let Some(operation) = &record.operation_ref {
            nonempty(operation, "stored accepted edit operation")?;
        }
        record.address.validate()?;
        let parts = decoded_pointer(&record.path)?;
        if parts.first().map(String::as_str) != Some("scene")
            || parts.get(1).map(String::as_str) == Some("procedural")
            || parts
                .iter()
                .any(|p| p.is_empty() || p.bytes().all(|c| c.is_ascii_digit()))
        {
            return Err("stored intervention lacks a stable material pointer".into());
        }
        let exact = address(context, &parts, &record.kind)?;
        if exact != record.address || !context.owned_addresses.iter().any(|a| a.covers(&exact)) {
            return Err(
                "stored intervention path differs from its exact native owned coordinate".into(),
            );
        }
        validate_material(&record.value, 0)?;
        if matches!(record.kind, InterventionKind::Reorder)
            && !record.value.as_array().is_some_and(|a| {
                a.iter().all(Value::is_string)
                    && a.iter()
                        .filter_map(Value::as_str)
                        .collect::<BTreeSet<_>>()
                        .len()
                        == a.len()
            })
        {
            return Err("stored intervention reorder lacks unique stable native ids".into());
        }
        if let Some(old) = rows.get(&record.path) {
            if old.revision == record.revision && *old != record {
                return Err(
                    "same native revision has conflicting attributed intervention rows".into(),
                );
            }
            if old.revision > record.revision {
                continue;
            }
        }
        rows.insert(record.path.clone(), record);
    }
    let admitted = rows.values().copied().collect::<Vec<_>>();
    Ok(rows
        .into_values()
        .filter(|r| {
            r.persistent
                && !admitted.iter().any(|parent| {
                    parent.persistent
                        && parent.path != r.path
                        && r.path.starts_with(&format!("{}/", parent.path))
                        && parent.revision > r.revision
                })
        })
        .map(|record| AuthoredOverlay {
            contribution_ref: context.contribution_ref.clone(),
            pointer: record.path.clone(),
            value: record.value.clone(),
            actor_ref: record.actor.clone(),
            persistent: true,
            operation: match record.kind {
                InterventionKind::Delete => OverlayOperation::Delete,
                InterventionKind::Reorder => OverlayOperation::Reorder,
                InterventionKind::Set | InterventionKind::Create => OverlayOperation::Set,
            },
        })
        .collect())
}

/// Actual Expression Selection/order before and candidate after an ordinary
/// native Focus/RelationFocus/SceneReorder. No Scene material delta is invented.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlowManualEdit {
    pub expression_ref: String,
    pub contribution_ref: String,
    pub owned_addresses: Vec<OwnedAddress>,
    pub before: Value,
    pub after: Value,
    pub actor_ref: String,
    pub operation_ref: String,
    pub document_revision: u64,
    #[serde(default)]
    pub retained_native_records: Vec<NativeAuthoredIntervention>,
}
fn whole_expression(expression_ref: &str) -> OwnedAddress {
    OwnedAddress {
        expression_ref: expression_ref.into(),
        scene_ref: None,
        entity_ref: None,
        component: "expression".into(),
        constituent_ref: None,
        parent_ref: None,
        property: None,
    }
}
pub fn validate_flow_state(state: &Value, expression_ref: &str) -> Result<()> {
    validate_material(state, 0)?;
    if state["schema"] != "ql.native-atlas-state/v1" || state["expression_ref"] != expression_ref {
        return Err("flow intervention lacks actual SAME Expression native state".into());
    }
    let focus = state["focus"]
        .as_object()
        .ok_or("native selection unavailable")?;
    if !focus.contains_key("scene_ref")
        || !focus.contains_key("entity_ref")
        || focus
            .keys()
            .any(|key| !["scene_ref", "entity_ref", "relation_ref"].contains(&key.as_str()))
    {
        return Err("native Selection must retain exact Scene/entity/relation coordinates".into());
    }
    let scenes = state["scene_order"]
        .as_array()
        .ok_or("native Scene order unavailable")?;
    let mut ids = BTreeSet::new();
    for scene in scenes {
        let scene = scene
            .as_str()
            .ok_or("native Scene order is not a ref list")?;
        if !scene.starts_with(&format!("{expression_ref}:scene:")) || !ids.insert(scene) {
            return Err("native Scene order crosses Expression or duplicates occurrences".into());
        }
    }
    let selection_scene = focus["scene_ref"].as_str();
    if !focus["scene_ref"].is_null() && selection_scene.is_none()
        || selection_scene.is_some_and(|s| !ids.contains(s))
    {
        return Err("native Selection is outside actual Expression Scene order".into());
    }
    let relation = focus.get("relation_ref");
    if !focus["entity_ref"].is_null() && relation.is_some_and(|r| !r.is_null()) {
        return Err("native Selection cannot contain both entity and relation".into());
    }
    if let Some(entity) = focus["entity_ref"].as_str() {
        if selection_scene.is_none() || !entity.starts_with(&format!("{expression_ref}:entity:")) {
            return Err("native entity selection crosses Expression".into());
        }
    } else if !focus["entity_ref"].is_null() {
        return Err("native entity selection is malformed".into());
    }
    if let Some(relation) = relation.and_then(Value::as_str) {
        nonempty(relation, "native relation selection")?;
        if selection_scene.is_none() {
            return Err("native relation selection lacks its Scene".into());
        }
    } else if relation.is_some_and(|r| !r.is_null()) {
        return Err("native relation selection is malformed".into());
    }
    Ok(())
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlowInterventionBasis {
    pub expression_ref: String,
    pub contribution_ref: String,
    pub owned_addresses: Vec<OwnedAddress>,
    pub current_state: Value,
    pub document_revision: u64,
    pub retained_native_records: Vec<NativeAuthoredIntervention>,
}
fn validate_flow_basis(edit: &FlowInterventionBasis) -> Result<OwnedAddress> {
    nonempty(&edit.contribution_ref, "retained flow contribution")?;
    let address = whole_expression(&edit.expression_ref);
    address.validate()?;
    if edit.document_revision == 0
        || edit.owned_addresses.iter().any(|a| a.validate().is_err())
        || !edit.owned_addresses.iter().any(|a| a == &address)
    {
        return Err("flow projection requires exact native whole Expression/CAS".into());
    }
    validate_flow_state(&edit.current_state, &edit.expression_ref)?;
    Ok(address)
}
fn flow_basis(edit: &FlowManualEdit) -> FlowInterventionBasis {
    FlowInterventionBasis {
        expression_ref: edit.expression_ref.clone(),
        contribution_ref: edit.contribution_ref.clone(),
        owned_addresses: edit.owned_addresses.clone(),
        current_state: edit.after.clone(),
        document_revision: edit.document_revision,
        retained_native_records: edit.retained_native_records.clone(),
    }
}
fn validate_flow_edit(edit: &FlowManualEdit) -> Result<OwnedAddress> {
    nonempty(&edit.actor_ref, "actual accepted flow actor")?;
    nonempty(&edit.operation_ref, "actual accepted flow operation")?;
    validate_flow_state(&edit.before, &edit.expression_ref)?;
    validate_flow_basis(&flow_basis(edit))
}
/// Project actual protected native rows over output-state coordinates. These
/// paths intentionally belong to native Atlas state, never Presentation.scene.
pub fn project_native_flow_interventions(edit: &FlowManualEdit) -> Result<Vec<AuthoredOverlay>> {
    validate_flow_edit(edit)?;
    project_flow_intervention_basis(&flow_basis(edit))
}
pub fn project_flow_intervention_basis(
    edit: &FlowInterventionBasis,
) -> Result<Vec<AuthoredOverlay>> {
    let address = validate_flow_basis(edit)?;
    if edit.retained_native_records.len() > 2048 {
        return Err("native flow intervention bound exceeded".into());
    }
    let mut latest = BTreeMap::<String, &NativeAuthoredIntervention>::new();
    for row in &edit.retained_native_records {
        if row.address != address
            || row.revision == 0
            || row.revision > edit.document_revision
            || !["/focus", "/scene_order"].contains(&row.path.as_str())
        {
            return Err("native flow record escapes actual Expression coordinate/CAS".into());
        }
        nonempty(&row.actor, "native flow record actor")?;
        if let Some(operation) = &row.operation_ref {
            nonempty(operation, "native flow record operation")?;
        }
        let (kind, _key) = if row.path == "/focus" {
            (InterventionKind::Set, "focus")
        } else {
            (InterventionKind::Reorder, "scene_order")
        };
        if row.kind != kind {
            return Err("native flow record kind differs from actual operation".into());
        }
        if let Some(old) = latest.get(&row.path) {
            if old.revision == row.revision && *old != row {
                return Err("same native CAS conflicts in flow attribution".into());
            }
            if old.revision > row.revision {
                continue;
            }
        }
        latest.insert(row.path.clone(), row);
    }
    // Selection/order change in one real native Edit is one coherent state.
    // Validate the admitted combined rows, never one row against obsolete focus.
    let mut state = edit.current_state.clone();
    for row in latest.values() {
        state[if row.path == "/focus" {
            "focus"
        } else {
            "scene_order"
        }] = row.value.clone();
    }
    validate_flow_state(&state, &edit.expression_ref)?;
    Ok(latest
        .into_values()
        .filter(|r| r.persistent)
        .map(|row| AuthoredOverlay {
            contribution_ref: edit.contribution_ref.clone(),
            pointer: row.path.clone(),
            value: row.value.clone(),
            actor_ref: row.actor.clone(),
            persistent: true,
            operation: if row.kind == InterventionKind::Reorder {
                OverlayOperation::Reorder
            } else {
                OverlayOperation::Set
            },
        })
        .collect())
}
pub fn extract_flow_interventions(edit: &FlowManualEdit) -> Result<ManualInterventions> {
    let address = validate_flow_edit(edit)?;
    project_native_flow_interventions(edit)?;
    let mut records = edit.retained_native_records.clone();
    let mut interventions = Vec::new();
    for (key, kind) in [
        ("focus", InterventionKind::Set),
        ("scene_order", InterventionKind::Reorder),
    ] {
        if edit.before[key] == edit.after[key] {
            continue;
        }
        let path = format!("/{key}");
        records.retain(|r| r.path != path);
        let record = NativeAuthoredIntervention {
            address: address.clone(),
            actor: edit.actor_ref.clone(),
            persistent: true,
            revision: edit.document_revision,
            path: path.clone(),
            value: edit.after[key].clone(),
            kind: kind.clone(),
            operation_ref: Some(edit.operation_ref.clone()),
            before: Some(edit.before[key].clone()),
        };
        interventions.push(NativeIntervention {
            contribution_ref: edit.contribution_ref.clone(),
            address: address.clone(),
            pointer: path,
            kind,
            before: record.before.clone(),
            value: Some(record.value.clone()),
            actor_ref: edit.actor_ref.clone(),
            operation_ref: edit.operation_ref.clone(),
            document_revision: edit.document_revision,
            persistent: true,
        });
        records.push(record);
    }
    let mut projected = edit.clone();
    projected.retained_native_records = records.clone();
    Ok(ManualInterventions {
        schema: "ql.procedural-manual-interventions/v1".into(),
        interventions,
        native_records: records,
        overlays: project_native_flow_interventions(&projected)?,
    })
}

/// Finite witness-free pure source batch on one actual ordinary Edit candidate.
/// The protected receiving host re-attests installed definitions and Document
/// before ingress; no nested rule/control/readback operation is representable.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum InterventionPreflight {
    ActiveControlsRefresh {
        inputs: Vec<crate::procedural_control::NativeActiveControlRefresh>,
    },
    ActiveControlRefresh {
        input: Box<crate::procedural_control::NativeActiveControlRefresh>,
    },
    InterventionsOwned {
        edit: Box<ManualEdit>,
    },
    FlowInterventions {
        edit: Box<FlowManualEdit>,
    },
    #[serde(rename = "scene_delete_anchor")]
    SceneDeleteAnchor {
        input: Box<crate::procedural_retention::SceneDeletionAnchor>,
    },
    #[serde(rename = "scene_delete_release")]
    SceneDeleteRelease {
        input: Box<crate::procedural_retention::SceneDeletionRelease>,
    },
}
impl InterventionPreflight {
    fn basis(&self) -> (&str, u64, &str, &str) {
        match self {
            Self::ActiveControlsRefresh { inputs } => (
                &inputs[0].reading.expression_ref,
                inputs[0].accepted_revision(),
                &inputs[0].actor_ref,
                &inputs[0].operation_ref,
            ),
            Self::ActiveControlRefresh { input } => (
                &input.reading.expression_ref,
                input.accepted_revision(),
                &input.actor_ref,
                &input.operation_ref,
            ),
            Self::InterventionsOwned { edit } => (
                &edit.expression_ref,
                edit.document_revision,
                &edit.actor_ref,
                &edit.operation_ref,
            ),
            Self::FlowInterventions { edit } => (
                &edit.expression_ref,
                edit.document_revision,
                &edit.actor_ref,
                &edit.operation_ref,
            ),
            Self::SceneDeleteAnchor { input } => (
                &input.edit.expression_ref,
                input.edit.document_revision,
                &input.edit.actor_ref,
                &input.edit.operation_ref,
            ),
            Self::SceneDeleteRelease { input } => (
                &input.procedure.occurrence_ref,
                input.document_revision,
                &input.actor_ref,
                &input.operation_ref,
            ),
        }
    }
}
pub fn intervention_batch(entries: &[InterventionPreflight]) -> Result<Value> {
    let joined = entries
        .iter()
        .filter(|entry| matches!(entry, InterventionPreflight::ActiveControlsRefresh { .. }))
        .count();
    if joined > 1
        || (joined == 1
            && entries
                .iter()
                .any(|entry| matches!(entry, InterventionPreflight::ActiveControlRefresh { .. })))
    {
        return Err(
            "one authoritative joined active-control refresh is required per actual candidate"
                .into(),
        );
    }
    if entries.is_empty()
        || entries.len() > 64
        || entries.iter().any(|entry| matches!(entry, InterventionPreflight::ActiveControlsRefresh { inputs } if inputs.is_empty() || inputs.len() > 64))
        || serde_json::to_vec(entries)
            .map_err(|e| e.to_string())?
            .len()
            > 8 * 1024 * 1024
    {
        return Err("native intervention batch count/byte bounds exceeded".into());
    }
    let original = entries[0].basis();
    let mut results = Vec::with_capacity(entries.len());
    for entry in entries {
        if entry.basis() != original {
            return Err("intervention batch crosses actual Expression/CAS/actor/operation".into());
        }
        let result = match entry {
            InterventionPreflight::ActiveControlsRefresh { inputs } => serde_json::to_value(
                crate::procedural_control::refresh_native_active_controls(inputs)?,
            ),
            InterventionPreflight::ActiveControlRefresh { input } => serde_json::to_value(
                crate::procedural_control::refresh_native_active_control(input)?,
            ),
            InterventionPreflight::InterventionsOwned { edit } => {
                serde_json::to_value(extract_owned_manual_interventions(edit)?)
            }
            InterventionPreflight::FlowInterventions { edit } => {
                serde_json::to_value(extract_flow_interventions(edit)?)
            }
            InterventionPreflight::SceneDeleteAnchor { input } => serde_json::to_value(
                crate::procedural_retention::prepare_scene_deletion_anchor(input)?,
            ),
            InterventionPreflight::SceneDeleteRelease { input } => serde_json::to_value(
                crate::procedural_retention::release_scene_deletion_anchor(input)?,
            ),
        }
        .map_err(|e| e.to_string())?;
        results.push(result);
    }
    Ok(serde_json::json!({"schema":"ql.procedural-intervention-batch/v1","results":results}))
}

/// Protected context accompanies the SAME event request. Rows come from actual
/// retained contribution/source anchor; no additional host ordinal or actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "material_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum NativeInterventionBasis {
    Scene { basis: SceneInterventionBasis },
    NativeFlow { basis: FlowInterventionBasis },
}
impl NativeInterventionBasis {
    pub fn contribution_ref(&self) -> &str {
        match self {
            Self::Scene { basis } => &basis.contribution_ref,
            Self::NativeFlow { basis } => &basis.contribution_ref,
        }
    }
    pub fn project(&self) -> Result<Vec<AuthoredOverlay>> {
        match self {
            Self::Scene { basis } => project_scene_intervention_basis(basis),
            Self::NativeFlow { basis } => project_flow_intervention_basis(basis),
        }
    }
}
