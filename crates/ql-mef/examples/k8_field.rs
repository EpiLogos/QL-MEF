//! Installed native producer runner. Host passes the worker path, never a shell
//! expression. Input is a retained sky-M2 event and supplied geometry definition.
use std::path::Path;
use std::time::Duration;
use ql_mef::continuous::{FieldInput, FieldSession, LiftInput};
use ql_mef::m2_engine::M2Request;
use serde_json::Value;

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 { return Err("usage: k8_field WORKER SKY_M2_EVENT INITIAL_FIELD_JSON".into()); }
    let read = |path: &str| -> Result<Value, String> {
        if std::fs::metadata(path).map_err(|e| e.to_string())?.len() > 32 * 1024 * 1024 {
            return Err("input exceeds 32 MiB".into());
        }
        serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
    };
    let event = read(&args[2])?;
    let initial = read(&args[3])?;
    let m2: M2Request = serde_json::from_value(event["m2_input"].clone()).map_err(|e| e.to_string())?;
    let original = serde_json::to_value(&m2).map_err(|e| e.to_string())?;
    let field: FieldInput = serde_json::from_value(initial["field"].clone()).map_err(|e| e.to_string())?;
    let mut session = FieldSession::open(Path::new(&args[1]), m2, field, Duration::from_secs(20))?;
    println!("{}", session.last_receipt());
    let first = session.advance(1024, false)?;
    assert_eq!(first["targets"], session.read()?["targets"]);
    println!("{first}");
    let refusal = session.advance(9000, false);
    assert!(refusal.is_err() && session.available());
    assert_eq!(first["amplitudes_metres"], session.read()?["amplitudes_metres"]);
    let changed = session.set_axis(1, LiftInput { turns: "-1".into(), half_degrees: 37 })?;
    assert_eq!(changed["clock"]["inscription"], first["clock"]["inscription"]);
    assert_eq!(changed["amplitudes_metres"], first["amplitudes_metres"]);
    println!("{changed}");
    let quiet = session.advance(512, true)?;
    assert!(quiet["audio"].as_array().unwrap().iter().all(|x| x.as_f64() == Some(0.0)));
    assert_ne!(quiet["amplitudes_metres"], changed["amplitudes_metres"]);
    println!("{quiet}");
    assert_eq!(serde_json::to_value(session.original_basis()).unwrap(), original);
    Ok(())
}
fn main() {
    if let Err(error) = run() { eprintln!("{error}"); std::process::exit(2); }
}
