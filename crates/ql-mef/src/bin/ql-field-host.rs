//! Existing supervised FieldHost. Generic stdin cannot supply native Act
//! authority; the separate channel is actual OS/native-image qualified.
use ql_mef::continuous::host::{FieldHost, HostRequest, MAX_HOST_INPUT, MAX_HOST_OUTPUT};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use ql_mef::continuous::native_act_channel::{NativeActOperation, QualifiedNativeActChannel};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use ql_mef::continuous::performance_act_bridge::serve_native_act_pipe;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

enum Input {
    Public(Result<Value, String>),
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    NativeAct(
        NativeActOperation,
        mpsc::SyncSender<QualifiedNativeActChannel>,
    ),
    Failed(String),
    Closed,
}
fn frame(reader: &mut impl BufRead) -> Result<Option<Result<Value, String>>, String> {
    let mut line = Vec::new();
    let n = reader
        .by_ref()
        .take(MAX_HOST_INPUT + 1)
        .read_until(b'\n', &mut line)
        .map_err(|e| e.to_string())?;
    if n == 0 {
        return Ok(None);
    }
    if n as u64 > MAX_HOST_INPUT || !line.ends_with(b"\n") {
        return Err("host input oversized or unterminated; owner closed".into());
    }
    Ok(Some(
        serde_json::from_slice(&line).map_err(|e| e.to_string()),
    ))
}
fn send(output: &mut impl Write, value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > MAX_HOST_OUTPUT {
        return Err("host output ceiling exceeded; no retry".into());
    }
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .and_then(|_| output.flush())
        .map_err(|e| e.to_string())
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--native-act-channel-capabilities" {
        println!(
            "{}",
            serde_json::json!({"schema":"ql.native-act-channel-capabilities/v1",
            "qualified_parent_channel":cfg!(any(target_os="linux",target_os="macos")),
            "generic_act_ingress":"refused-before-mint",
            "native_operation_schema":"ql.native-act-owner-request/v1"})
        );
        return Ok(());
    }
    if args.len() != 3 && !(args.len() == 5 && args[3] == "--native-act-channel") {
        return Err(
            "usage: ql-field-host WORKER HOST_CONFIG_JSON [--native-act-channel PRIVATE_SOCKET]"
                .into(),
        );
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
    let mut host = FieldHost::open_config(Path::new(&args[1]), &bytes, Duration::from_secs(20))?;
    let output = io::stdout();
    let mut output = output.lock();
    send(&mut output, &host.ready())?;
    let (tx, rx) = mpsc::sync_channel(2);
    let public = tx.clone();
    std::thread::spawn(move || {
        let input = io::stdin();
        let mut input = input.lock();
        loop {
            let event = match frame(&mut input) {
                Ok(Some(value)) => Input::Public(value),
                Ok(None) => {
                    let _ = public.send(Input::Closed);
                    return;
                }
                Err(error) => {
                    let _ = public.send(Input::Failed(error));
                    return;
                }
            };
            if public.send(event).is_err() {
                return;
            }
        }
    });
    if args.len() == 5 {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let path = args[4].clone();
            let native = tx.clone();
            std::thread::spawn(move || {
                let mut channel = match QualifiedNativeActChannel::establish(Path::new(&path)) {
                    Ok(channel) => channel,
                    Err(error) => {
                        let _ = native.send(Input::Failed(error));
                        return;
                    }
                };
                loop {
                    let operation = match channel.next_operation() {
                        Ok(operation) => operation,
                        Err(error) => {
                            let _ = native.send(Input::Failed(error));
                            return;
                        }
                    };
                    let (return_tx, return_rx) = mpsc::sync_channel(1);
                    if native.send(Input::NativeAct(operation, return_tx)).is_err() {
                        return;
                    }
                    channel = match return_rx.recv() {
                        Ok(channel) => channel,
                        Err(_) => return,
                    };
                }
            });
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        return Err("native Act parent channel unavailable on this platform".into());
    }
    drop(tx);
    for input in rx {
        match input {
            Input::Public(value) => {
                let response=match value {
                    Ok(value) if value["schema"]=="ql.native-act-owner-request/v1"=>
                        host.reject_input("native Act requires actual qualified parent channel; generic stdin has no selection authority"),
                    value=>match value.and_then(|v|serde_json::from_value::<HostRequest>(v).map_err(|e|e.to_string())) {
                        Ok(request)=>host.execute(request),
                        Err(_)=>host.reject_input("malformed or unknown host request fields"),
                    },
                };
                send(&mut output, &response)?;
            }
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            Input::NativeAct(mut operation, return_tx) => {
                serve_native_act_pipe(&mut host, &mut operation)?;
                return_tx
                    .send(operation.into_channel())
                    .map_err(|_| "native qualified Act reader closed")?;
            }
            Input::Failed(error) => return Err(error),
            Input::Closed => return Ok(()),
        }
        if !host.available() {
            return Err("native acknowledgement unavailable; owner closed without retry".into());
        }
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("ql-field-host: {error}");
        std::process::exit(1);
    }
}
