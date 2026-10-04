//! Preserve retained authored force interventions through actual native reads.
//! The SAME receiving owner re-attests context/readings at Document CAS. This
//! pure compiler projection grants no authority and performs no unit conversion.
use crate::procedural_composition::{
    GeneratedContribution, NativeChange, OwnedAddress, Result, TargetReading,
    native_parameter_address,
};
use crate::procedural_manifestation::{nonempty, validate_material};
use crate::procedural_retention::{MATERIALIZATION_CONTRACT, NativeMaterialization};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeEffectiveParameter {
    pub native_owner: String,
    pub expression_ref: String,
    pub document_revision: u64,
    pub target_revision: u64,
    pub contribution_ref: String,
    pub address: OwnedAddress,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<OwnedAddress>,
    pub parameter: String,
    /// Exact Document Entity.parameters[key].value / native get_parameter.
    pub value: Value,
    pub actor_ref: String,
    /// Legacy retained overrides can lack these fields. Preserve that absence;
    /// never label the current Document as a fabricated accepted operation.
    pub operation_ref: Option<String>,
    pub intervention_revision: Option<u64>,
}

/// Replace only the effective native ParameterSet operand. `generated_basis`
/// stays the source recipe's value so inspection, retirement and replay retain
/// the difference between generation and authored intervention. Removing the
/// retained intervention releases the recipe's operand on the next generation.
pub fn apply_retained_force_interventions(
    contributions: &mut [GeneratedContribution],
    readings: &[TargetReading],
    context: &NativeMaterialization,
) -> Result<Vec<NativeEffectiveParameter>> {
    if context.schema != MATERIALIZATION_CONTRACT || context.document_revision == 0 {
        return Err("effective parameter requires SAME native Document materialization".into());
    }
    if contributions.len() > 2048 || readings.len() > 2048 || context.scenes.len() > 2048 {
        return Err("native effective contribution/reading bound exceeded".into());
    }
    let mut candidate = contributions.to_vec();
    let mut observed = Vec::new();
    let mut unique = BTreeSet::new();
    for contribution in &mut candidate {
        let Some(parameter) = contribution.generated_basis["parameter"]
            .as_str()
            .map(str::to_owned)
        else {
            continue;
        };
        if contribution.native_changes.len() != 1 || contribution.owned_addresses.is_empty() {
            return Err(
                "native parameter contribution lacks one exact global operation/owned locations"
                    .into(),
            );
        }
        let NativeChange::ParameterSet {
            entity_ref,
            parameter: native_parameter,
            value,
        } = &mut contribution.native_changes[0]
        else {
            return Err("native parameter basis lacks actual ParameterSet".into());
        };
        if native_parameter != &parameter {
            return Err("native parameter operation differs from declared property".into());
        }
        let mut addresses = contribution.owned_addresses.clone();
        addresses.sort();
        if addresses.windows(2).any(|w| w[0] == w[1]) {
            return Err("duplicate native parameter location".into());
        }
        let mut current: Option<(&Value, u64)> = None;
        let mut selected: Option<&Value> = None;
        let mut active_control: Option<Value> = None;
        for address in &addresses {
            address.validate()?;
            let scene = address
                .scene_ref
                .as_deref()
                .ok_or("native parameter location lacks Scene")?;
            if native_parameter_address(&address.expression_ref, scene, entity_ref, &parameter)?
                != *address
            {
                return Err(
                    "effective parameter address differs from exact global native target".into(),
                );
            }
            let targets = readings
                .iter()
                .filter(|r| {
                    r.address.expression_ref == address.expression_ref
                        && r.address.scene_ref == address.scene_ref
                        && r.address.entity_ref.as_deref() == Some(entity_ref.as_str())
                        && r.address.covers(address)
                })
                .collect::<Vec<_>>();
            if targets.len() != 1 {
                return Err(
                    "effective parameter lacks one exact native reading per affected Scene".into(),
                );
            }
            let target = targets[0];
            if target.revision == 0 || target.revision > context.document_revision {
                return Err("effective native parameter revision escapes current CAS".into());
            }
            let native_value = target
                .properties
                .get(&parameter)
                .ok_or("actual native parameter key/value unavailable")?;
            validate_material(native_value, 0)?;
            if let Some((old, revision)) = current {
                if old != native_value || revision != target.revision {
                    return Err("shared native parameter values/revisions disagree".into());
                }
            } else {
                current = Some((native_value, target.revision));
            }
            let scenes = context
                .scenes
                .iter()
                .filter(|s| s.scene_ref == scene)
                .collect::<Vec<_>>();
            if scenes.len() != 1 || scenes[0].document_revision != context.document_revision {
                return Err("effective parameter lacks current actual containing Scene".into());
            }
            let Some(retention) = &scenes[0].existing_retention else {
                continue;
            };
            for control in retention["controls"]
                .as_array()
                .ok_or("current native retention lacks controls")?
            {
                let controlled: OwnedAddress = serde_json::from_value(control["address"].clone())
                    .map_err(|e| e.to_string())?;
                if !controlled.covers(address) || control["takeover"].is_null() {
                    continue;
                }
                let takeover = &control["takeover"];
                if !matches!(
                    takeover["lifetime"].as_str(),
                    Some("gesture" | "persistent")
                ) {
                    return Err("retained active control has unknown lifetime".into());
                }
                let revision = takeover["revision"]
                    .as_u64()
                    .ok_or("active control lacks native accepted revision")?;
                if revision == 0 || revision > context.document_revision {
                    return Err("active control revision escapes actual CAS".into());
                }
                let projected = serde_json::json!({"actor":takeover["actor"],"operation_ref":takeover["operation_ref"],"revision":revision});
                if let Some(old) = &active_control {
                    if old["revision"] == revision && old != &projected {
                        return Err("shared global active control provenance conflicts".into());
                    }
                    if old["revision"].as_u64().unwrap() > revision {
                        continue;
                    }
                }
                active_control = Some(projected);
            }
            let rows = retention["contributions"]
                .as_array()
                .ok_or("current native retention lacks contributions")?;
            let matches = rows
                .iter()
                .filter(|c| c["contribution_ref"] == contribution.contribution_ref)
                .collect::<Vec<_>>();
            if matches.len() > 1 {
                return Err("duplicate retained native parameter contribution".into());
            }
            let Some(retained) = matches.first() else {
                continue;
            };
            if retained["procedure_ref"] != contribution.procedure_ref
                || retained["status"] != "active"
            {
                return Err("native effective contribution is foreign or detached".into());
            }
            for overlay in retained["authored_overrides"]
                .as_array()
                .ok_or("retained authored interventions unavailable")?
            {
                let owned: OwnedAddress = serde_json::from_value(overlay["address"].clone())
                    .map_err(|e| e.to_string())?;
                owned.validate()?;
                if overlay["persistent"] == false || !owned.covers(address) {
                    continue;
                }
                let revision = overlay["revision"]
                    .as_u64()
                    .ok_or("native intervention revision missing")?;
                if revision == 0 || revision > context.document_revision {
                    return Err("native authored intervention revision escapes actual CAS".into());
                }
                if let Some(old) = selected {
                    let old_revision = old["revision"]
                        .as_u64()
                        .ok_or("native intervention revision missing")?;
                    if old_revision == revision
                        && (old["actor"] != overlay["actor"]
                            || old["operation_ref"] != overlay["operation_ref"])
                    {
                        return Err("conflicting native parameter intervention provenance".into());
                    }
                    if old_revision > revision {
                        continue;
                    }
                }
                selected = Some(overlay);
            }
        }
        let overlay = match (selected, active_control.as_ref()) {
            (Some(overlay), Some(control))
                if overlay["revision"].as_u64().unwrap()
                    >= control["revision"].as_u64().unwrap() =>
            {
                overlay
            }
            (_, Some(control)) => control,
            (Some(overlay), None) => overlay,
            (None, None) => continue,
        };
        let actor = overlay["actor"]
            .as_str()
            .ok_or("retained intervention lacks native actor")?;
        nonempty(actor, "native intervention actor")?;
        let operation_ref = match overlay.get("operation_ref") {
            None | Some(Value::Null) => None,
            Some(Value::String(s)) => {
                nonempty(s, "native intervention operation")?;
                Some(s.clone())
            }
            _ => return Err("invalid native intervention operation provenance".into()),
        };
        let (current_value, target_revision) =
            current.ok_or("actual current native parameter unavailable")?;
        if !unique.insert((
            contribution.contribution_ref.clone(),
            entity_ref.clone(),
            parameter.clone(),
        )) {
            return Err("duplicate effective global parameter writer".into());
        }
        // Authored Scene operands retain their own units/provenance. The SAME
        // native owner read supplies this global ParameterSet's exact value.
        *value = current_value.clone();
        observed.push(NativeEffectiveParameter {
            native_owner: "oi.expression".into(),
            expression_ref: addresses[0].expression_ref.clone(),
            document_revision: context.document_revision,
            target_revision,
            contribution_ref: contribution.contribution_ref.clone(),
            address: addresses[0].clone(),
            addresses: if addresses.len() > 1 {
                addresses
            } else {
                Vec::new()
            },
            parameter,
            value: current_value.clone(),
            actor_ref: actor.into(),
            operation_ref,
            intervention_revision: overlay["revision"].as_u64(),
        });
    }
    contributions.clone_from_slice(&candidate);
    Ok(observed)
}

/// Preserve human native flow using actual current Document.selection/order;
/// attributed rows choose the controlled dimension, not a stale row operand.
pub fn apply_retained_flow_interventions(
    contributions: &mut [GeneratedContribution],
    readings: &[TargetReading],
    context: &NativeMaterialization,
) -> Result<()> {
    if context.schema != MATERIALIZATION_CONTRACT || context.document_revision == 0 {
        return Err("native flow override lacks exact SAME Document basis".into());
    }
    let mut candidate = contributions.to_vec();
    for contribution in &mut candidate {
        if !contribution.generated_basis["native_flow"].is_array() {
            continue;
        }
        let whole = contribution
            .owned_addresses
            .iter()
            .find(|a| a.component == "expression" && a.property.is_none())
            .ok_or("native flow output lacks whole Expression scope")?;
        let target = readings
            .iter()
            .filter(|r| r.address == *whole)
            .collect::<Vec<_>>();
        if target.len() != 1 {
            return Err("native flow override lacks one exact whole Expression reading".into());
        }
        let target = target[0];
        let state = target
            .properties
            .get("native_atlas_state")
            .ok_or("actual native Selection/order reading unavailable")?;
        let mut found: Option<&Value> = None;
        for scene in &context.scenes {
            let Some(retention) = &scene.existing_retention else {
                continue;
            };
            for row in retention["contributions"]
                .as_array()
                .ok_or("native flow retention rows unavailable")?
            {
                if row["contribution_ref"] != contribution.contribution_ref {
                    continue;
                }
                if let Some(previous) = found {
                    if previous != row {
                        return Err(
                            "conflicting retained whole Expression contribution projections".into(),
                        );
                    }
                } else {
                    found = Some(row);
                }
            }
        }
        let Some(retained) = found else {
            continue;
        };
        if retained["procedure_ref"] != contribution.procedure_ref || retained["status"] != "active"
        {
            return Err("native flow contribution is foreign or detached".into());
        }
        let records: Vec<crate::procedural_intervention::NativeAuthoredIntervention> =
            serde_json::from_value(retained["authored_overrides"].clone())
                .map_err(|e| e.to_string())?;
        let basis = crate::procedural_intervention::FlowInterventionBasis {
            expression_ref: whole.expression_ref.clone(),
            contribution_ref: contribution.contribution_ref.clone(),
            owned_addresses: contribution.owned_addresses.clone(),
            current_state: state.clone(),
            document_revision: context.document_revision,
            retained_native_records: records,
        };
        let overlays = crate::procedural_intervention::project_flow_intervention_basis(&basis)?;
        for overlay in overlays {
            match overlay.pointer.as_str() {
                "/focus" => {
                    let focus = &state["focus"];
                    if focus["scene_ref"].is_null() {
                        // The accepted human removal already owns empty native
                        // Selection. Omitting recipe Focus preserves that state;
                        // Source cannot fabricate a clear-focus native operation.
                        contribution.native_changes.retain(|change| {
                            !matches!(
                                change,
                                NativeChange::Focus { .. } | NativeChange::RelationFocus { .. }
                            )
                        });
                        continue;
                    }
                    let scene_ref = focus["scene_ref"]
                        .as_str()
                        .ok_or("actual native selection is empty; source cannot invent a Focus")?
                        .into();
                    let effective = if let Some(binding) = focus["relation_ref"].as_str() {
                        NativeChange::RelationFocus {
                            scene_ref,
                            binding_ref: binding.into(),
                        }
                    } else {
                        NativeChange::Focus {
                            scene_ref,
                            entity_ref: focus["entity_ref"].as_str().map(str::to_owned),
                        }
                    };
                    for change in &mut contribution.native_changes {
                        if matches!(
                            change,
                            NativeChange::Focus { .. } | NativeChange::RelationFocus { .. }
                        ) {
                            *change = effective.clone();
                        }
                    }
                }
                "/scene_order" => {
                    let scene_refs: Vec<String> =
                        serde_json::from_value(state["scene_order"].clone())
                            .map_err(|e| e.to_string())?;
                    for change in &mut contribution.native_changes {
                        if matches!(change, NativeChange::SceneReorder { .. }) {
                            *change = NativeChange::SceneReorder {
                                scene_refs: scene_refs.clone(),
                            };
                        }
                    }
                }
                _ => return Err("native flow intervention escapes source output state".into()),
            }
        }
    }
    contributions.clone_from_slice(&candidate);
    Ok(())
}
