//! Native CLI view over the same embedded or explicitly supplied M ledger.
use ql_mef::m_ledger::{MLedger, NATIVE_M_LEDGER};
use ql_mef::m_tree::native_m_registry;

use crate::CliError;

pub fn command(args: &[String], json: bool) -> Result<String, CliError> {
    let operation = args.first().map(String::as_str).unwrap_or("");
    let mut positionals = Vec::new();
    let mut options = std::collections::BTreeMap::new();
    let mut at = 1;
    while at < args.len() {
        let arg = &args[at];
        if arg.starts_with("--") {
            if !["--ledger", "--stratum", "--axis", "--require"].contains(&arg.as_str())
                || options.contains_key(arg.as_str())
            {
                return Err(CliError(format!(
                    "unknown or duplicate ledger option: {arg}"
                )));
            }
            let value = args
                .get(at + 1)
                .filter(|v| !v.starts_with("--"))
                .ok_or_else(|| CliError(format!("missing value for {arg}")))?;
            options.insert(arg.as_str(), value.as_str());
            at += 2;
        } else {
            positionals.push(arg.as_str());
            at += 1;
        }
    }
    let source = options
        .get("--ledger")
        .map(|p| std::fs::read_to_string(p).map_err(|e| CliError(e.to_string())))
        .transpose()?;
    let registry = native_m_registry();
    let ledger = MLedger::from_json(source.as_deref().unwrap_or(NATIVE_M_LEDGER), registry)
        .map_err(CliError)?;
    if operation != "coverage"
        && ((operation == "validate-ledger" && !positionals.is_empty())
            || positionals.len() > 1
            || options.keys().any(|k| *k != "--ledger"))
    {
        return Err(CliError(
            "only coverage accepts a coordinate, stratum, axis or readiness requirement".into(),
        ));
    }
    match operation {
        "ledger" => {
            if let Some(reference) = positionals.first() {
                let view = ledger
                    .coordinate_view(registry, reference)
                    .map_err(CliError)?;
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                serde_json::to_string_pretty(&ledger).map_err(CliError::from)
            }
        }
        "validate-ledger" => {
            let findings = ledger.validate(registry);
            if json {
                serde_json::to_string_pretty(&findings).map_err(CliError::from)
            } else {
                Ok(format!(
                    "M ledger valid; {} explicit coverage/discrepancy findings (not completion)",
                    findings.len()
                ))
            }
        }
        "coverage" => {
            if positionals.len() != 1 {
                return Err(CliError(
                    "coverage requires exactly one M coordinate".into(),
                ));
            }
            let report = ledger
                .coverage(
                    registry,
                    positionals[0],
                    options.get("--stratum").copied().unwrap_or("rust"),
                    options.get("--axis").copied().unwrap_or("operational"),
                    options.get("--require").copied().unwrap_or("verified"),
                )
                .map_err(CliError)?;
            if json {
                serde_json::to_string_pretty(&report).map_err(CliError::from)
            } else {
                Ok(format!(
                    "{}: {} structural coordinates; {} capability/index rows\n{} {} / {}: {} blocking rows\n{} coordinates without computational binding; {} source rows without implementation disposition\n{} structural-index bindings do not establish computational readiness.\nUse --json for exact rows, missing coordinates, orphans and disagreements.",
                    report.scope,
                    report.structural_coordinates,
                    report.rows.len(),
                    report.stratum,
                    report.required_readiness,
                    report.axis,
                    report.blocking_rows.len(),
                    report.coordinates_without_computational_binding.len(),
                    report.source_without_implementation_disposition.len(),
                    report.structural_index_bindings.len()
                ))
            }
        }
        _ => Err(CliError("unknown ledger operation".into())),
    }
}
