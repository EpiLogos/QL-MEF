//! Public transport for the existing transactional M3 producer.
//! Selections and owner references remain explicit; this pure replay neither
//! authenticates a host nor admits its commands as personal activity.
use crate::CliError;
use ql_mef::m3_state::{M3Command, M3Request, M3State};
use serde::Deserialize;
use serde_json::json;
use std::io::Read;

const MAX_INPUT: u64 = 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    request: M3Request,
    commands: Vec<M3Command>,
    activity_policy: Option<ql_mef::nara::m3_activity::Policy>,
    activity_start_generation: Option<u64>,
}

pub(crate) fn command(args: &[String]) -> Result<String, CliError> {
    if args.len() != 1 {
        return Err(CliError(
            "usage: ql kernel m3 <request.json|-> --json".into(),
        ));
    }
    let reader: Box<dyn Read> = if args[0] == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(&args[0]).map_err(|e| CliError(e.to_string()))?)
    };
    let mut input = String::new();
    reader
        .take(MAX_INPUT + 1)
        .read_to_string(&mut input)
        .map_err(|e| CliError(e.to_string()))?;
    if input.len() as u64 > MAX_INPUT {
        return Err(CliError("M3 request exceeds 1 MiB".into()));
    }
    replay(&input)
}

pub(crate) fn replay(input: &str) -> Result<String, CliError> {
    let input: Input = serde_json::from_str(&input).map_err(CliError::from)?;
    if input.commands.len() > 256 {
        return Err(CliError("M3 replay exceeds 256 commands".into()));
    }
    let mut state = M3State::new(input.request.clone()).map_err(CliError)?;
    let mut receipts = Vec::with_capacity(input.commands.len());
    for command in input.commands {
        receipts.push(state.apply(command).map_err(CliError)?);
    }
    let final_generation = state.snapshot()["identity"]["profile_generation"]
        .as_u64()
        .ok_or_else(|| CliError("Native M3 generation unavailable".into()))?;
    let start = input.activity_start_generation.unwrap_or(0);
    if start > final_generation
        || (input.activity_policy.is_none() && input.activity_start_generation.is_some())
    {
        return Err(CliError(
            "Activity policy boundary must name an existing native generation and selected policy"
                .into(),
        ));
    }
    let mut result = json!({"state":state.snapshot(),"receipts":receipts});
    if let Some(policy) = input.activity_policy {
        result["activity"] =
            ql_mef::nara::m3_activity::receive(&input.request, &receipts, policy, start)
                .map_err(CliError)?;
    }
    serde_json::to_string(&result).map_err(CliError::from)
}
