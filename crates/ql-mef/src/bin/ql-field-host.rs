//! Local supervised stdio body for the existing coupled instrument. No listener,
//! shell dispatch, graph lookup or audio callback is introduced here.
use ql_mef::continuous::host::{
    FieldHost, HostConfig, HostRequest, MAX_HOST_INPUT, MAX_HOST_OUTPUT,
};
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::time::Duration;

fn send(output: &mut impl Write, value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > MAX_HOST_OUTPUT {
        return Err("host output ceiling exceeded; connection closed without retry".into());
    }
    output.write_all(&bytes).map_err(|e| e.to_string())?;
    output.write_all(b"\n").map_err(|e| e.to_string())?;
    output.flush().map_err(|e| e.to_string())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: ql-field-host WORKER HOST_CONFIG_JSON".into());
    }
    let mut bytes = Vec::new();
    File::open(&args[2])
        .map_err(|e| e.to_string())?
        .take(MAX_HOST_INPUT + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_HOST_INPUT {
        return Err("host configuration ceiling exceeded".into());
    }
    let config: HostConfig = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let mut host = FieldHost::open(Path::new(&args[1]), config, Duration::from_secs(5))?;
    let input = io::stdin();
    let mut input = input.lock();
    let output = io::stdout();
    let mut output = output.lock();
    send(&mut output, &host.ready())?;
    loop {
        let mut line = Vec::new();
        let count = input
            .by_ref()
            .take(MAX_HOST_INPUT + 1)
            .read_until(b'\n', &mut line)
            .map_err(|e| e.to_string())?;
        if count == 0 {
            return Ok(());
        }
        if count as u64 > MAX_HOST_INPUT || !line.ends_with(b"\n") {
            // Framing loss cannot be repaired by treating a suffix as a command.
            return Err("host input is oversized or unterminated; closing owner".into());
        }
        let response = match serde_json::from_slice::<HostRequest>(&line) {
            Ok(request) => host.execute(request),
            Err(_) => host.reject_input("malformed or unknown host request fields"),
        };
        send(&mut output, &response)?;
        if !host.available() {
            return Err("native acknowledgement unavailable; host closed without retry".into());
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ql-field-host: {error}");
        std::process::exit(1);
    }
}
