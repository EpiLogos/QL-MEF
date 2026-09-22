//! `ql techne` — the production Technē reading surface over the Wiki seam.
//!
//! One operation: `ql techne reading <target.json>` — the caller supplies the
//! Wiki's own projection of one subject (a `WikiRefractionTarget`, optionally
//! carrying a shape binding) and the production `WikiTechneAdapter` returns
//! the validated reading plus its co-referenced selections. The file is the
//! subject source; the command holds no store and executes no mutations.

use crate::CliError;
use ql_adapters::{TECHNE_CONTRACT, TechneReading, WikiSubjectSource, WikiTechneAdapter};
use ql_wiki::WikiRefractionTarget;
use serde_json::{Value, json};
use std::io::Read;

type R<T> = Result<T, CliError>;
fn error(e: impl std::fmt::Display) -> CliError {
    CliError(e.to_string())
}

struct FileSubjectSource {
    target: WikiRefractionTarget,
}

impl WikiSubjectSource for FileSubjectSource {
    fn wiki_target(&self, subject_ref: &str) -> Option<WikiRefractionTarget> {
        (subject_ref == self.target.target_ref).then(|| self.target.clone())
    }
}

pub fn command(args: &[String], json: bool) -> R<String> {
    match args {
        [op, path] if op == "reading" => reading(path, json),
        _ => Err(error("usage: ql techne reading <target.json> [--json]")),
    }
}

fn reading(path: &str, json: bool) -> R<String> {
    let file = std::fs::File::open(path).map_err(error)?;
    let mut data = Vec::new();
    file.take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut data)
        .map_err(error)?;
    if data.len() > 16 * 1024 * 1024 {
        return Err(error("techne target exceeds 16 MiB"));
    }
    let target: WikiRefractionTarget =
        serde_json::from_slice(&data).map_err(|e| error(format!("invalid target: {e}")))?;
    let source = FileSubjectSource {
        target: target.clone(),
    };
    let adapter = WikiTechneAdapter::new(&source);
    let reading: TechneReading = adapter.reading_for_target(&target).map_err(error)?;
    let selections = adapter.selections(&target, &reading).map_err(error)?;
    let document = json!({
        "contract": TECHNE_CONTRACT,
        "reading": serde_json::to_value(&reading).map_err(error)?,
        "selections": serde_json::to_value(&selections).map_err(error)?,
    });
    if json {
        serde_json::to_string_pretty(&document).map_err(error)
    } else {
        Ok(render_plain(&document))
    }
}

fn render_plain(document: &Value) -> String {
    let reading = &document["reading"];
    let mut lines = Vec::new();
    lines.push(format!(
        "subject {} (owner {})",
        reading["subject"]["subject_ref"], reading["subject"]["native_owner"]
    ));
    if let Some(whole) = reading["whole"].as_object() {
        lines.push(format!(
            "whole {} · {} members",
            whole["whole_ref"],
            whole["member_refs"].as_array().map(Vec::len).unwrap_or(0)
        ));
    }
    if let Some(ql) = reading["ql"].as_object() {
        lines.push(format!(
            "ql {} lens {:?} warrant {:?}",
            ql["shape_ref"].as_str().unwrap_or("-"),
            ql["lens_ref"],
            ql["warrant"]["result_class"]
        ));
    }
    for instrument in reading["disclosure"]["instruments"]
        .as_array()
        .expect("disclosure instruments")
    {
        let mark = if instrument["available"].as_bool().unwrap_or(false) {
            "available"
        } else {
            "unavailable"
        };
        lines.push(format!(
            "  {:<12} {} ({})",
            format!("{:?}", instrument["instrument"]).to_lowercase(),
            mark,
            instrument["reason"].as_str().unwrap_or("ready")
        ));
    }
    for selection in document["selections"].as_array().expect("selections") {
        lines.push(format!(
            "selection {} on {} ({:?})",
            selection["selection_ref"], selection["reading_ref"], selection["instrument"]
        ));
    }
    lines.join("\n")
}
