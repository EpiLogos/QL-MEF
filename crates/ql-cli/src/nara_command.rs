//! Native identity operations over stdin. Source persistence remains Central's.
use crate::CliError;
use ql_mef::nara::intake::IdentityProfile;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
#[path = "nara_dialogue_command.rs"]
mod dialogue;
#[path = "nara_presence_command.rs"]
mod presence;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

const MAX_INPUT: usize = 2 * 1024 * 1024;
const MAX_OUTPUT: u64 = 16 * 1024 * 1024;
const RESOURCES: &[(&str, &str)] = &[
    (
        "providers/nara/natal.py",
        include_str!("../../../providers/nara/natal.py"),
    ),
    (
        "providers/sky/kerykeion_snapshot.py",
        include_str!("../../../providers/sky/kerykeion_snapshot.py"),
    ),
    (
        "providers/sky/requirements.txt",
        include_str!("../../../providers/sky/requirements.txt"),
    ),
    (
        "vendor/epi-kernel/reference/include/m2.h",
        include_str!("../../../vendor/epi-kernel/reference/include/m2.h"),
    ),
    (
        "fixtures/kernel/m2-retained-c-v1.json",
        include_str!("../../../fixtures/kernel/m2-retained-c-v1.json"),
    ),
];
fn error(e: impl std::fmt::Display) -> CliError {
    CliError(e.to_string())
}

fn resources() -> Result<PathBuf, CliError> {
    let mut hash = Sha256::new();
    for (path, body) in RESOURCES {
        hash.update(path);
        hash.update([0]);
        hash.update(body);
    }
    let digest = format!("{:x}", hash.finalize());
    let base = if let Some(path) = std::env::var_os("QL_NARA_PROVIDER_CACHE") {
        PathBuf::from(path)
    } else {
        let home = std::env::var_os("HOME")
            .ok_or_else(|| error("HOME or QL_NARA_PROVIDER_CACHE is required"))?;
        #[cfg(target_os = "macos")]
        let path = PathBuf::from(home).join("Library/Caches/QuaternalLogic/nara-provider");
        #[cfg(not(target_os = "macos"))]
        let path = std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(home).join(".cache"))
            .join("quaternal-logic/nara-provider");
        path
    }
    .join(digest);
    for (relative, content) in RESOURCES {
        let path = base.join(relative);
        if path.exists() {
            if fs::read(&path).map_err(error)? != content.as_bytes() {
                return Err(error(
                    "embedded natal provider cache has changed; remove the damaged generation before retrying",
                ));
            }
        } else {
            fs::create_dir_all(
                path.parent()
                    .ok_or_else(|| error("invalid provider resource path"))?,
            )
            .map_err(error)?;
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(error)?
                .as_nanos();
            let staging = path.with_extension(format!("publishing-{}-{nonce}", std::process::id()));
            let publish = (|| -> Result<(), CliError> {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&staging)
                    .map_err(error)?;
                file.write_all(content.as_bytes()).map_err(error)?;
                file.sync_all().map_err(error)?;
                // A competing calculator can see only complete resource bytes.
                match fs::hard_link(&staging, &path) {
                    Ok(()) => Ok(()),
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                        if fs::read(&path).map_err(error)? == content.as_bytes() {
                            Ok(())
                        } else {
                            Err(error("natal provider resource publication conflict"))
                        }
                    }
                    Err(e) => Err(error(e)),
                }
            })();
            let _ = fs::remove_file(&staging);
            publish?;
        }
    }
    Ok(base.join("providers/nara/natal.py"))
}

fn pipe<R: Read + Send + 'static>(reader: R) -> mpsc::Receiver<Result<Vec<u8>, String>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut data = Vec::new();
        let result = reader
            .take(MAX_OUTPUT + 1)
            .read_to_end(&mut data)
            .map_err(|e| e.to_string())
            .and_then(|_| {
                if data.len() as u64 > MAX_OUTPUT {
                    Err("natal provider output exceeds 16 MiB".into())
                } else {
                    Ok(data)
                }
            });
        let _ = tx.send(result);
    });
    rx
}

/// Always reap a provider, including early pipe setup or wait failures.
struct ProviderProcess(Child);
impl Drop for ProviderProcess {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn uv_executable() -> Result<PathBuf, CliError> {
    if let Some(path) = std::env::var_os("QL_NARA_UV") {
        return Ok(PathBuf::from(path));
    }
    let executable = if cfg!(windows) { "uv.exe" } else { "uv" };
    if let Some(path) = std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|directory| directory.join(executable))
            .find(|candidate| candidate.is_file())
    }) {
        return Ok(path);
    }
    if let Some(path) = std::env::var_os("HOME")
        .map(|home| PathBuf::from(home).join(".local/bin").join(executable))
        .filter(|path| path.is_file())
    {
        return Ok(path);
    }
    Err(error(
        "Natal runtime requires installed uv (PATH or ~/.local/bin/uv); QL_NARA_UV can name it, or QL_NARA_PYTHON can select a qualified diagnostic interpreter",
    ))
}

/// Own only an unpublished staging environment; published runtimes are reused.
struct ProviderRuntime {
    executable: PathBuf,
    owned_directory: Option<PathBuf>,
}
impl Drop for ProviderRuntime {
    fn drop(&mut self) {
        if let Some(path) = &self.owned_directory {
            let _ = fs::remove_dir_all(path);
        }
    }
}

fn prepare_runtime(command: &mut Command, deadline: Instant) -> Result<(), CliError> {
    if Instant::now() >= deadline {
        return Err(error(
            "Natal runtime preparation exhausted the calculation deadline",
        ));
    }
    let mut process = ProviderProcess(
        command
            .env_remove("VIRTUAL_ENV")
            .env_remove("PYTHONHOME")
            .env_remove("PYTHONPATH")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| error(format!("Natal managed runtime resolver unavailable: {e}")))?,
    );
    let stdout = pipe(
        process
            .0
            .stdout
            .take()
            .ok_or_else(|| error("uv stdout absent"))?,
    );
    let stderr = pipe(
        process
            .0
            .stderr
            .take()
            .ok_or_else(|| error("uv stderr absent"))?,
    );
    let status = loop {
        match process.0.try_wait().map_err(error)? {
            Some(status) => break status,
            None if Instant::now() >= deadline => {
                return Err(error(
                    "Natal managed runtime preparation exceeded the 45-second calculation deadline",
                ));
            }
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    };
    let _output = stdout
        .recv_timeout(Duration::from_secs(2))
        .map_err(error)?
        .map_err(error)?;
    let errors = stderr
        .recv_timeout(Duration::from_secs(2))
        .map_err(error)?
        .map_err(error)?;
    if !status.success() {
        return Err(error(format!(
            "Natal managed runtime preparation failed: {}",
            String::from_utf8_lossy(&errors)
        )));
    }
    Ok(())
}

/// uv run's temporary environment disappears when its parent exits. Instead,
/// publish a completed relocatable environment in the existing resource cache,
/// and launch Python directly under our provider guard. Concurrent calculators
/// prepare separate staging directories; only a complete runtime can win.
fn provider_python(
    script: &std::path::Path,
    deadline: Instant,
) -> Result<ProviderRuntime, CliError> {
    if let Some(path) = std::env::var_os("QL_NARA_PYTHON") {
        return Ok(ProviderRuntime {
            executable: PathBuf::from(path),
            owned_directory: None,
        });
    }
    let providers = script
        .parent()
        .and_then(|path| path.parent())
        .ok_or_else(|| error("invalid embedded natal resource layout"))?;
    let base = providers
        .parent()
        .ok_or_else(|| error("invalid embedded resource root"))?;
    let python_member = if cfg!(windows) {
        "Scripts/python.exe"
    } else {
        "bin/python"
    };
    let published = base.join(format!(
        "python-3.13-{}-{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    ));
    let marker = format!(
        "uv-managed-python-3.13/relocatable-v1\n{}",
        include_str!("../../../providers/sky/requirements.txt")
    );
    let qualified = || -> Result<(), CliError> {
        if !published.join(python_member).is_file()
            || fs::read(published.join(".ql-runtime-ready")).map_err(error)? != marker.as_bytes()
        {
            return Err(error(
                "Natal managed runtime cache is incomplete or changed; remove its damaged generation before retrying",
            ));
        }
        Ok(())
    };
    if published.exists() {
        qualified()?;
        return Ok(ProviderRuntime {
            executable: published.join(python_member),
            owned_directory: None,
        });
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(error)?
        .as_nanos();
    let directory = base.join(format!("runtime-{}-{nonce}", std::process::id()));
    fs::create_dir(&directory).map_err(error)?;
    let runtime = ProviderRuntime {
        executable: directory.join(python_member),
        owned_directory: Some(directory.clone()),
    };
    let uv = uv_executable()?;
    prepare_runtime(
        Command::new(&uv)
            .args([
                "venv",
                "--quiet",
                "--no-project",
                "--no-config",
                "--relocatable",
                "--python-preference",
                "only-managed",
                "--python",
                "3.13",
            ])
            .arg(&directory),
        deadline,
    )?;
    prepare_runtime(
        Command::new(&uv)
            .args(["pip", "install", "--quiet", "--no-config", "--python"])
            .arg(&runtime.executable)
            .arg("--requirement")
            .arg(providers.join("sky/requirements.txt")),
        deadline,
    )?;
    if !runtime.executable.is_file() {
        return Err(error("uv did not prepare the managed Python executable"));
    }
    fs::write(directory.join(".ql-runtime-ready"), marker.as_bytes()).map_err(error)?;
    match fs::rename(&directory, &published) {
        Ok(()) => {}
        // Another complete nonempty directory cannot be replaced by rename.
        Err(_) if published.exists() => qualified()?,
        Err(e) => return Err(error(e)),
    }
    qualified()?;
    // Keep the venv path: canonicalizing its Python symlink loses its packages.
    Ok(ProviderRuntime {
        executable: published.join(python_member),
        owned_directory: None,
    })
}

pub fn calculate(profile: &IdentityProfile) -> Result<Value, CliError> {
    let request = profile.natal_request().map_err(error)?;
    let script = resources()?;
    calculate_provider(&request, script, &[])
}

fn calculate_provider(
    request: &Value,
    script: PathBuf,
    arguments: &[&str],
) -> Result<Value, CliError> {
    let bytes = serde_json::to_vec(request).map_err(error)?;
    let deadline = Instant::now() + Duration::from_secs(45);
    let runtime = provider_python(&script, deadline)?;
    if Instant::now() >= deadline {
        return Err(error(
            "Natal runtime preparation exhausted the calculation deadline",
        ));
    }
    let mut provider = Command::new(&runtime.executable);
    provider.arg(script);
    provider.args(arguments);
    let mut process = ProviderProcess(
        provider
            .env_remove("PYTHONHOME")
            .env_remove("PYTHONPATH")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| error(format!("Natal Python provider unavailable: {e}")))?,
    );
    let child = &mut process.0;
    let stdout = pipe(
        child
            .stdout
            .take()
            .ok_or_else(|| error("provider stdout absent"))?,
    );
    let stderr = pipe(
        child
            .stderr
            .take()
            .ok_or_else(|| error("provider stderr absent"))?,
    );
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| error("provider stdin absent"))?;
    let (input_tx, input_rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = input_tx.send(stdin.write_all(&bytes).map_err(|e| e.to_string()));
    });
    let status = loop {
        match child.try_wait().map_err(error)? {
            Some(status) => break status,
            None if Instant::now() >= deadline => {
                return Err(error("Natal calculation exceeded 45 seconds"));
            }
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    };
    input_rx
        .recv_timeout(Duration::from_secs(2))
        .map_err(error)?
        .map_err(error)?;
    let output = stdout
        .recv_timeout(Duration::from_secs(2))
        .map_err(error)?
        .map_err(error)?;
    let errors = stderr
        .recv_timeout(Duration::from_secs(2))
        .map_err(error)?
        .map_err(error)?;
    if !status.success() {
        let reason = serde_json::from_slice::<Value>(&output)
            .ok()
            .and_then(|v| v["error"].as_str().map(str::to_owned))
            .unwrap_or_else(|| String::from_utf8_lossy(&errors).into_owned());
        return Err(error(format!("Natal provider refused: {reason}")));
    }
    serde_json::from_slice(&output).map_err(error)
}

/// The accepted sky snapshot itself, from the same embedded provider Nara uses.
pub(crate) fn sky_snapshot(request: &Value, existing_snapshot: bool) -> Result<Value, CliError> {
    let natal_script = resources()?;
    let script = natal_script
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| error("invalid sky resource layout"))?
        .join("sky/kerykeion_snapshot.py");
    let args: &[&str] = if existing_snapshot {
        &["-", "--validate-snapshot"]
    } else {
        &["-"]
    };
    calculate_provider(request, script, args)
}

fn transit(request: &Value, existing_snapshot: bool) -> Result<Value, CliError> {
    let natal_script = resources()?;
    let script = natal_script
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| error("invalid sky resource layout"))?
        .join("sky/kerykeion_snapshot.py");
    let args: &[&str] = if existing_snapshot {
        &["-", "--validate-snapshot"]
    } else {
        &["-"]
    };
    let sky = calculate_provider(request, script, args)?;
    ql_mef::nara::current::transit(Some(&sky)).map_err(error)
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersonalCurrentRequest {
    schema: String,
    profile: IdentityProfile,
    sky_request: Option<Value>,
    sky_snapshot: Option<Value>,
    m3_input: Option<Value>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersonalRecomposeRequest {
    schema: String,
    current: Value,
    m3_input: Value,
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    if args.len() == 1 && args[0] == "capabilities" {
        return serde_json::to_string_pretty(&json!({"schema":"ql.nara-identity-capabilities/v1","operations":["inspect","calculate","transit","personal-current","personal-recompose","presence-consent"],"coordinate_operations":["coordinate"],"dialogue_operations":["context","delegate","enrichment","receive"],"dialogue_registry":"native-current-m-registry","dialogue_persistence_owner":"host","profile_schema":"ql.nara-identity-profile/v1","persistence_owner":"central","natal_provider":"Kerykeion","provider_python":"uv-managed Python 3.13 with embedded providers/sky/requirements.txt; QL_NARA_PYTHON diagnostic override","provider_uv":"QL_NARA_UV, PATH, or ~/.local/bin/uv","input":"JSON profile on stdin or file","identity_offices":["birthdate-name","natal-chart","jungian-assessment","gene-keys","human-design","archetypal-quintessence"],"automatic_agent_or_model_invocation":false})).map_err(error);
    }
    let [operation, path] = args else {
        return Err(error(
            "usage: ql nara <inspect|calculate|transit|personal-current|personal-recompose|presence-consent|coordinate|context|delegate|enrichment|receive> <request.json|-> [--json]",
        ));
    };
    if ![
        "inspect",
        "calculate",
        "transit",
        "personal-current",
        "personal-recompose",
        "presence-consent",
        "coordinate",
        "context",
        "delegate",
        "enrichment",
        "receive",
    ]
    .contains(&operation.as_str())
    {
        return Err(error("unknown Nara operation"));
    }
    let mut bytes = Vec::new();
    if path == "-" {
        std::io::stdin()
            .take(MAX_INPUT as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(error)?;
    } else {
        fs::File::open(path)
            .map_err(error)?
            .take(MAX_INPUT as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(error)?;
    }
    if bytes.len() > MAX_INPUT {
        return Err(error("Nara input exceeds 2 MiB"));
    }
    if operation == "presence-consent" {
        return presence::command(&bytes);
    }
    if ["coordinate", "context", "delegate", "enrichment", "receive"].contains(&operation.as_str())
    {
        return dialogue::command(operation, &bytes);
    }
    if operation == "personal-recompose" {
        let request: PersonalRecomposeRequest = serde_json::from_slice(&bytes).map_err(error)?;
        if request.schema != "ql.nara-personal-recompose-request/v1"
            || request.current["schema"] != "ql.nara-personal-current/v1"
        {
            return Err(error("unsupported personal recomposition request"));
        }
        let m3: Value = serde_json::from_str(&crate::m3_command::replay(
            &serde_json::to_string(&request.m3_input).map_err(error)?,
        )?)
        .map_err(error)?;
        let activity = m3
            .get("activity")
            .ok_or_else(|| error("Select an explicit native M3 activity policy"))?;
        let current = ql_mef::nara::current::personal_current_with_activity(
            &request.current["identity"],
            &request.current["transit"],
            Some(activity),
        )
        .map_err(error)?;
        return serde_json::to_string_pretty(&current).map_err(error);
    }
    if operation == "transit" {
        let request: Value = serde_json::from_slice(&bytes).map_err(error)?;
        return serde_json::to_string_pretty(&transit(&request, false)?).map_err(error);
    }
    if operation == "personal-current" {
        let request: PersonalCurrentRequest = serde_json::from_slice(&bytes).map_err(error)?;
        if request.schema != "ql.nara-personal-current-request/v1" {
            return Err(error("unsupported personal current request"));
        }
        request.profile.validate().map_err(error)?;
        let natal = calculate(&request.profile)?;
        let identity = request.profile.inspect(Some(&natal)).map_err(error)?;
        let transit = match (&request.sky_request, &request.sky_snapshot) {
            (Some(sky), None) => transit(sky, false)?,
            (None, Some(sky)) => transit(sky, true)?,
            _ => {
                return Err(error(
                    "personal current requires exactly one sky_request or sky_snapshot",
                ));
            }
        };
        let m3 = request
            .m3_input
            .as_ref()
            .map(|input| {
                let reading =
                    crate::m3_command::replay(&serde_json::to_string(input).map_err(error)?)?;
                serde_json::from_str::<Value>(&reading).map_err(error)
            })
            .transpose()?;
        let activity = m3.as_ref().and_then(|reading| reading.get("activity"));
        let current =
            ql_mef::nara::current::personal_current_with_activity(&identity, &transit, activity)
                .map_err(error)?;
        return serde_json::to_string_pretty(&current).map_err(error);
    }
    let profile: IdentityProfile = serde_json::from_slice(&bytes).map_err(error)?;
    profile.validate().map_err(error)?;
    let natal = if operation == "calculate" {
        Some(calculate(&profile)?)
    } else {
        None
    };
    let reading = profile.inspect(natal.as_ref()).map_err(error)?;
    serde_json::to_string_pretty(&reading).map_err(error)
}
