//! Exact readback of the existing stopped native Manager observer operation.
//! This validates evidence; it cannot grant source custody or schedule work.
use crate::performance_management::qualify_native_note_wire;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

fn count(value: &Value) -> Result<u64, String> {
    let text = value
        .as_str()
        .ok_or("exact native observer counter absent")?;
    let number: u64 = text
        .parse()
        .map_err(|_| "native observer counter invalid")?;
    if number.to_string() != text {
        return Err("noncanonical native observer counter".into());
    }
    Ok(number)
}
fn fields(value: &Value, keys: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or("native observer object absent")?;
    if object.len() != keys.len() || keys.iter().any(|key| !object.contains_key(*key)) {
        return Err("native observer field set differs".into());
    }
    Ok(())
}
fn entries(value: &Value, bound: usize) -> Result<&Vec<Value>, String> {
    let array = value.as_array().ok_or("native observer array absent")?;
    if array.len() > bound {
        return Err("native observer array exceeds actual owner bound".into());
    }
    Ok(array)
}
fn same_note_row(expected: &Value, actual: &Value, key: &str) -> Result<(), String> {
    let left = expected
        .get(key)
        .ok_or("native observer note field absent")?;
    let right = actual.get(key).ok_or("native observer note field absent")?;
    if left.is_null() {
        if !right.is_null() {
            return Err("native observer absent note changed".into());
        }
    } else {
        qualify_native_note_wire(left, right)?;
    }
    // Only the closed 19-field native note at this explicitly owned path uses
    // binary64 equality. All surrounding types, fields and raw values are exact.
    let mut compared = actual.clone();
    compared[key] = left.clone();
    if *expected != compared {
        return Err("native observer row changed outside native note bits".into());
    }
    Ok(())
}
fn same_applications(expected: &[Value], actual: &[Value]) -> Result<(), String> {
    if expected.len() != actual.len() {
        return Err("native application FIFO length differs".into());
    }
    for (left, right) in expected.iter().zip(actual) {
        same_note_row(left, right, "note")?;
    }
    Ok(())
}
fn same_checkpoint(expected: &Value, actual: &Value) -> Result<(), String> {
    let mut compared = actual.clone();
    let left_inputs = entries(&expected["inputs"], 96)?;
    let right_inputs = entries(&actual["inputs"], 96)?;
    if left_inputs.len() != right_inputs.len() {
        return Err("native input slots changed".into());
    }
    for (left, right) in left_inputs.iter().zip(right_inputs) {
        same_note_row(left, right, "target")?;
    }
    same_journal(
        entries(&expected["input_history"]["entries"], 256)?,
        entries(&actual["input_history"]["entries"], 256)?,
    )?;
    same_applications(
        entries(
            &expected["native_pair"]["audio"]["applications"]["entries"],
            256,
        )?,
        entries(
            &actual["native_pair"]["audio"]["applications"]["entries"],
            256,
        )?,
    )?;
    compared["inputs"] = expected["inputs"].clone();
    compared["input_history"]["entries"] = expected["input_history"]["entries"].clone();
    compared["native_pair"]["audio"]["applications"]["entries"] =
        expected["native_pair"]["audio"]["applications"]["entries"].clone();
    if *expected != compared {
        return Err(
            "native body/voices/phases/queues/source/input custody changed on restitution".into(),
        );
    }
    Ok(())
}
fn same_journal(expected: &[Value], actual: &[Value]) -> Result<(), String> {
    if expected.len() != actual.len() {
        return Err("native input journal lost, forged or reordered".into());
    }
    for (left, right) in expected.iter().zip(actual) {
        fields(
            right,
            &[
                "ordinal",
                "native_sequence",
                "change",
                "operation",
                "input_ref",
                "target",
            ],
        )?;
        same_note_row(left, right, "target")?;
    }
    Ok(())
}
fn record(
    journal: &mut Vec<Value>,
    write: &mut u64,
    ordinal: &mut u64,
    input: &Value,
    change: u8,
    operation: u64,
    sequence: u64,
) -> Result<(), String> {
    if sequence == 0 || journal.len() >= 256 {
        return Err("native input feedback journal exhausted".into());
    }
    *write = write
        .checked_add(1)
        .ok_or("native journal write exhausted")?;
    *ordinal = ordinal
        .checked_add(1)
        .ok_or("native journal ordinal exhausted")?;
    journal.push(json!({"ordinal":ordinal.to_string(),"native_sequence":sequence.to_string(),
        "change":change,"operation":operation,"input_ref":input["input_ref"],"target":input["target"]}));
    Ok(())
}

/// Derive NativeInputBindings::application/retire_absent and the stopped
/// PerformanceManagement::pulse from its complete original checkpoint.
/// The caller separately qualifies the actual native ACK/source/owner. Every
/// non-observer field, complete original source and future operation is exact.
/// Binary64 targets remain raw native values, including signed zero.
pub fn qualify_stopped_observer_feedback(
    saved: &Value,
    after: &Value,
    acknowledged_epoch: u64,
    applications: &[Value],
    returned_history: &[Value],
) -> Result<(), String> {
    const MANAGEMENT: [&str; 12] = [
        "schema",
        "session_ref",
        "transport_epoch",
        "recording_failed",
        "release_pending",
        "panic_applied",
        "release_request",
        "release_sequence",
        "release_proof_cursor",
        "native_pair",
        "inputs",
        "input_history",
    ];
    fields(saved, &MANAGEMENT)?;
    fields(after, &MANAGEMENT)?;
    if saved["schema"] != "ql.performance-management-checkpoint/v1"
        || acknowledged_epoch <= count(&saved["transport_epoch"])?
        || acknowledged_epoch != count(&after["transport_epoch"])?
        || applications.len() > 256
        || returned_history.len() > 256
    {
        return Err("stopped native observer schema/epoch/bounds differ".into());
    }
    let mut expected = saved.clone();
    expected["transport_epoch"] = json!(acknowledged_epoch.to_string());
    // A genuine historical checkpoint may be restored before any pulse.
    // That cut retains every original unread row, with no generated feedback.
    if applications.is_empty()
        && returned_history.is_empty()
        && same_checkpoint(&expected, after).is_ok()
    {
        return Ok(());
    }
    let audio = &saved["native_pair"]["audio"];
    let queue = &audio["applications"];
    let restored_queue = &after["native_pair"]["audio"]["applications"];
    fields(queue, &["read", "write", "entries"])?;
    fields(restored_queue, &["read", "write", "entries"])?;
    let read = count(&queue["read"])?;
    let app_write = count(&queue["write"])?;
    let original_apps = entries(&queue["entries"], 256)?;
    if app_write.checked_sub(read) != Some(original_apps.len() as u64)
        || read.checked_add(applications.len() as u64) != Some(app_write)
        || count(&restored_queue["read"])? != app_write
        || count(&restored_queue["write"])? != app_write
        || !entries(&restored_queue["entries"], 256)?.is_empty()
    {
        return Err("native original application FIFO changed or was lost".into());
    }
    same_applications(original_apps, applications)?;
    let history = &saved["input_history"];
    fields(
        history,
        &[
            "read",
            "write",
            "last_ordinal",
            "last_touch_token",
            "last_member_token",
            "entries",
        ],
    )?;
    let history_read = count(&history["read"])?;
    let mut write = count(&history["write"])?;
    let mut ordinal = count(&history["last_ordinal"])?;
    let mut journal = entries(&history["entries"], 256)?.clone();
    if write.checked_sub(history_read) != Some(journal.len() as u64) || ordinal != write {
        return Err("original native input journal cut differs".into());
    }
    for (index, entry) in journal.iter().enumerate() {
        if history_read.checked_add(index as u64 + 1) != Some(count(&entry["ordinal"])?) {
            return Err("original native input journal order differs".into());
        }
        qualify_native_note_wire(&entry["target"], &entry["target"])?;
    }
    let mut inputs = BTreeMap::new();
    for input in entries(&saved["inputs"], 96)? {
        fields(
            input,
            &[
                "slot",
                "input_ref",
                "target",
                "release_pending",
                "press_applied",
                "press_sequence",
                "release_sequence",
            ],
        )?;
        let slot = input["slot"].as_u64().ok_or("native input slot absent")?;
        if slot >= 96 || inputs.insert(slot, input.clone()).is_some() {
            return Err("native input fixed slot differs".into());
        }
        qualify_native_note_wire(&input["target"], &input["target"])?;
    }
    for (index, app) in applications.iter().enumerate() {
        if !app["note"].is_null() {
            qualify_native_note_wire(&original_apps[index]["note"], &app["note"])?;
        }
        let kind = app["kind"]
            .as_u64()
            .ok_or("native application kind absent")?;
        let applied = app["applied"]
            .as_bool()
            .ok_or("native application result absent")?;
        let sequence = count(&app["sequence"])?;
        if app["schema"] != "ql.performance-applied-event/v2"
            || kind > 6
            || app["status"] != if applied { "applied" } else { "refused" }
            || read.checked_add(index as u64 + 1)
                != Some(count(&app["applied_application_ordinal"])?)
            || count(&app["applied_application_ordinal"])?
                > count(&audio["applied_application_ordinal"])?
            || sequence == 0
            || sequence > count(&audio["accepted_sequence"])?
            || count(&app["committed_cursor"])? > count(&audio["cursor"])?
        {
            return Err("native original application source/order/cursor differs".into());
        }
        let mut removed = Vec::new();
        for (slot, input) in &mut inputs {
            if kind != 4 && input["target"]["touch"] != app["touch"] {
                continue;
            }
            record(
                &mut journal,
                &mut write,
                &mut ordinal,
                input,
                if applied { 2 } else { 3 },
                kind,
                sequence,
            )?;
            if applied && kind == 0 && sequence == count(&input["press_sequence"])? {
                qualify_native_note_wire(&input["target"], &app["note"])?;
                input["press_applied"] = json!(true);
            }
            if (applied && (kind == 1 || kind == 4)) || (!applied && kind == 0) {
                removed.push(*slot);
            }
        }
        for slot in removed {
            inputs.remove(&slot);
        }
        if kind == 4 && applied && sequence == count(&expected["release_sequence"])? {
            expected["panic_applied"] = json!(true);
        }
    }
    let held = entries(&audio["touches"], 96)?
        .iter()
        .map(|touch| count(&touch["token"]))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let last_sequence = count(&audio["applied_sequence"])?;
    let mut retired = Vec::new();
    for (slot, input) in &inputs {
        if input["press_applied"]
            .as_bool()
            .ok_or("native input press standing absent")?
            && count(&input["press_sequence"])? <= last_sequence
            && !held.contains(&count(&input["target"]["touch"])?)
        {
            record(
                &mut journal,
                &mut write,
                &mut ordinal,
                input,
                4,
                1,
                last_sequence,
            )?;
            retired.push(*slot);
        }
    }
    for slot in retired {
        inputs.remove(&slot);
    }
    same_journal(&journal, returned_history)?;
    expected["inputs"] = json!(inputs.into_values().collect::<Vec<_>>());
    expected["input_history"]["read"] = json!(
        history_read
            .checked_add(returned_history.len() as u64)
            .ok_or("native journal read exhausted")?
            .to_string()
    );
    expected["input_history"]["write"] = json!(write.to_string());
    expected["input_history"]["last_ordinal"] = json!(ordinal.to_string());
    expected["input_history"]["entries"] = json!([]);
    expected["native_pair"]["audio"]["applications"]["read"] = json!(app_write.to_string());
    expected["native_pair"]["audio"]["applications"]["entries"] = json!([]);
    let request = count(&expected["release_request"])?;
    if expected["release_pending"]
        .as_bool()
        .ok_or("native release standing absent")?
        && (request != 0
            || expected["panic_applied"]
                .as_bool()
                .ok_or("native panic standing absent")?)
    {
        let proof = &audio["release_proof"];
        let cursor = count(&audio["cursor"])?;
        let fence = if request != 0 {
            count(&proof["emergency_observed"])? >= request
                && cursor
                    .checked_sub(count(&proof["emergency_applied_sample"])?)
                    .is_some_and(|n| n >= 512)
        } else {
            count(&expected["release_sequence"])? != 0
                && last_sequence >= count(&expected["release_sequence"])?
        };
        if !audio["fault"]
            .as_bool()
            .ok_or("native fault standing absent")?
            && saved["native_pair"]["physical"]["state"]["samples_elapsed"] == audio["cursor"]
            && held.is_empty()
            && entries(&audio["voices"], 24)?.is_empty()
            && entries(&audio["tails"], 16)?.is_empty()
            && !audio["sustain"]
                .as_bool()
                .ok_or("native sustain standing absent")?
            && count(&proof["force_zero_samples"])? >= 512
            && fence
        {
            expected["release_pending"] = json!(false);
            expected["release_proof_cursor"] = json!(cursor.to_string());
        }
    }
    if audio["recording"]["failure"]
        .as_u64()
        .ok_or("native recording standing absent")?
        != 0
    {
        expected["recording_failed"] = json!(true);
    }
    same_checkpoint(&expected, after)
}
