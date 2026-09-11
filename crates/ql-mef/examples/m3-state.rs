//! Headless owner integration: validated request plus an ordered command log.
//! Native host authority/disclosure is external to this pure producer.
use ql_mef::m3_state::{M3Command, M3Request, M3State};
use serde::Deserialize;
use serde_json::json;
use std::io::{self, Read};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    request: M3Request,
    commands: Vec<M3Command>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut text = String::new();
    io::stdin()
        .take(1024 * 1024 + 1)
        .read_to_string(&mut text)?;
    if text.len() > 1024 * 1024 {
        return Err("M3 request exceeds 1 MiB".into());
    }
    let input: Input = serde_json::from_str(&text)?;
    if input.commands.len() > 256 {
        return Err("M3 replay exceeds 256 commands".into());
    }
    let mut state = M3State::new(input.request)?;
    let mut receipts = Vec::new();
    for command in input.commands {
        receipts.push(state.apply(command)?);
    }
    println!("{}", json!({"state":state.snapshot(),"receipts":receipts}));
    Ok(())
}
