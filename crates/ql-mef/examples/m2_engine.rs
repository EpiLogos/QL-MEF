//! Checkout-independent JSON adapter once compiled; no Python/runtime source dependency.
use std::{env, fs, process};
fn run() -> Result<(), String> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: m2_engine REQUEST.json | --fixture")?;
    let text = if path == "--fixture" {
        include_str!("../../../fixtures/kernel/m2-engine-request-v1.json").to_owned()
    } else {
        fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?
    };
    let frame = ql_mef::m2_engine::M2Request::from_json(&text)?.execute()?;
    println!(
        "{}",
        serde_json::to_string_pretty(&frame).map_err(|e| e.to_string())?
    );
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("M2: {error}");
        process::exit(1);
    }
}
