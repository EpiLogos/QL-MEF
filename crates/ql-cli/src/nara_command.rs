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

/// How an existing snapshot is received, separately from its immutable request.
/// Requested preserves current/historical policy; retained is a dated replay.
#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum SnapshotPurpose {
    #[default]
    Requested,
    RetainedOccasion,
}

// Exact previous native provider descriptors remain historical provenance.
// This is not a general Sun-key exception and does not authorize fresh sky.
const LEGACY_SUN_ADAPTER: &str = "e6d96d2ab5e4c539004ce84d7c7956404faea9752ce602bebc170441b457ab81";
const LEGACY_SUN_REGISTRY: &str =
    "82cd2a438fe82fdf3cd6a56383cc591b3beef53bd22b593922cd3fd6768f7de2";
const LEGACY_SUN_HEADER: &str = "7dfcd2906afb4415151d059d2259aa2252dfe74fa74a64a6b843ae5b385ab4c8";

fn source_binding_qualification(sky: &Value, retained: bool) -> Result<Value, CliError> {
    let legacy = sky["source_binding"]["sun_role"] == "parent-not-chakra-mapped";
    if legacy {
        if !retained
            || sky["provider"]["adapter_sha256"] != LEGACY_SUN_ADAPTER
            || sky["source_binding"]["registry_revision"] != LEGACY_SUN_REGISTRY
            || sky["source_binding"]["header_sha256"] != LEGACY_SUN_HEADER
        {
            return Err(error("unqualified legacy Sun descriptor"));
        }
    } else if sky["source_binding"]["sun_role"] != "solar-parent" {
        return Err(error("unknown Sun source descriptor"));
    }
    // #254 D10: actual directed PLANETARY_RESONANCE is authority. Do not
    // infer reception from the descriptive provider string or a C bitmask.
    let route = ql_mef::m2::planet_chakra_route(0)
        .map_err(error)?
        .ok_or_else(|| error("current Sun planetary resonance is absent"))?;
    if route.planet_coordinate != "#2-5-0/1"
        || route.chakra_coordinate != "#2-5-0/1-7"
        || route.chakra_index != 7
        || sky["source_binding"]["registry_revision"] != route.registry_revision
        || !route.relations.iter().any(|relation| {
            relation.source_kind == "PLANETARY_RESONANCE"
                && relation.from_id == Some(route.planet_id)
                && relation.to_id == Some(route.chakra_id)
        })
    {
        return Err(error(
            "Sun descriptor is not grounded in the accepted native Bimba route",
        ));
    }
    Ok(json!({"schema":"ql.sky-source-binding-qualification/v1",
        "snapshot_ref":sky["snapshot_ref"],"legacy_descriptor_admitted":legacy,
        "original_sun_role":sky["source_binding"]["sun_role"],"current_sun_role":"solar-parent",
        "provider_adapter_sha256":sky["provider"]["adapter_sha256"],
        "native_sun_route":route,
        "fresh_current_attested":false,
        "standing":if legacy {"known legacy descriptor retained as historical provenance; current native Bimba relation governs reception"}
            else {"current provider descriptor and native Bimba reception kept distinct"}}))
}

fn acknowledged_sky_admission(admission: &Value, sky: &Value) -> Result<Value, CliError> {
    let purpose: SnapshotPurpose =
        serde_json::from_value(admission["purpose"].clone()).map_err(error)?;
    // Activity operates the already-held occasion. Validate its exact immutable
    // source without recalculation or a renewed current freshness attestation.
    let validated = sky_snapshot(sky, true, SnapshotPurpose::RetainedOccasion)?;
    if validated != *sky {
        return Err(error("acknowledged sky changed during retained validation"));
    }
    let qualification = source_binding_qualification(sky, true)?;
    let current = purpose.admission_value(sky, qualification.clone());
    if *admission == current {
        return Ok(qualification);
    }
    if qualification["legacy_descriptor_admitted"] == true {
        let mut original = current;
        original
            .as_object_mut()
            .unwrap()
            .remove("source_binding_qualification");
        original["validator_source"]["revision"] = json!(format!("sha256:{LEGACY_SUN_ADAPTER}"));
        if *admission == original {
            // Keep the original acknowledged receipt, including its original
            // requested mode; the separate qualifier is explicitly not fresh.
            return Ok(qualification);
        }
    }
    Err(error(
        "personal recomposition sky admission differs from its exact source",
    ))
}

impl SnapshotPurpose {
    pub(crate) fn admission(self, sky: &Value) -> Result<Value, CliError> {
        let qualification =
            source_binding_qualification(sky, matches!(self, Self::RetainedOccasion))?;
        Ok(self.admission_value(sky, qualification))
    }

    fn admission_value(self, sky: &Value, qualification: Value) -> Value {
        let fresh = matches!(self, Self::Requested) && sky["request"]["mode"] == "current";
        json!({"schema":"ql.sky-admission/v1", "purpose":self,
        "snapshot_ref":sky["snapshot_ref"], "original_mode":sky["request"]["mode"],
        "epoch_utc":sky["epoch_utc"], "receipt_utc":sky["receipt_utc"],
        "fresh_current_attested":fresh,
        "source_binding_qualification":qualification,
        "validation":"immutable-snapshot-and-current-native-source",
        "validator_source":{"source_ref":"providers/sky/kerykeion_snapshot.py",
            "revision":format!("sha256:{:x}", Sha256::digest(include_bytes!("../../../providers/sky/kerykeion_snapshot.py")))},
        "standing":if matches!(self, Self::RetainedOccasion) {
            "retained dated occasion; no fresh-current attestation"
        } else if fresh {
            "current requested epoch validated within its freshness budget"
        } else {
            "selected historical epoch; no fresh-current attestation"
        }})
    }
}

/// The accepted sky itself, from the same embedded provider Nara uses. Purpose
/// is an admission instruction, never a change to that sky's signed request.
pub(crate) fn sky_snapshot(
    request: &Value,
    existing_snapshot: bool,
    purpose: SnapshotPurpose,
) -> Result<Value, CliError> {
    if matches!(purpose, SnapshotPurpose::RetainedOccasion) && !existing_snapshot {
        return Err(error(
            "retained-occasion requires an existing exact sky snapshot",
        ));
    }
    let natal_script = resources()?;
    let script = natal_script
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| error("invalid sky resource layout"))?
        .join("sky/kerykeion_snapshot.py");
    let args: &[&str] = if matches!(purpose, SnapshotPurpose::RetainedOccasion) {
        &["-", "--validate-retained-snapshot"]
    } else if existing_snapshot {
        &["-", "--validate-snapshot"]
    } else {
        &["-"]
    };
    let sky = calculate_provider(request, script, args)?;
    source_binding_qualification(&sky, matches!(purpose, SnapshotPurpose::RetainedOccasion))?;
    Ok(sky)
}

fn transit(
    request: &Value,
    existing_snapshot: bool,
    purpose: SnapshotPurpose,
) -> Result<(Value, Value), CliError> {
    let sky = sky_snapshot(request, existing_snapshot, purpose)?;
    let admission = purpose.admission(&sky)?;
    Ok((
        ql_mef::nara::current::transit(Some(&sky)).map_err(error)?,
        admission,
    ))
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersonalCurrentRequest {
    schema: String,
    profile: IdentityProfile,
    sky_request: Option<Value>,
    sky_snapshot: Option<Value>,
    m3_input: Option<Value>,
    #[serde(default)]
    snapshot_purpose: SnapshotPurpose,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersonalRecomposeRequest {
    schema: String,
    current: Value,
    m3_input: Value,
}

/// Set only on the native Source child invocation. This reduction-only resource
/// ceiling cannot supply a coordinate, registry, actor or owner grant. Ordinary
/// Nara operations keep their original input/output policy when it is absent.
fn source_coordinate_cap(operation: &str) -> Result<Option<usize>, CliError> {
    if operation != "coordinate" {
        return Ok(None);
    }
    let Some(raw) = std::env::var_os("QL_NATIVE_SOURCE_COORDINATE_BYTE_CAP") else {
        return Ok(None);
    };
    let cap = raw
        .to_str()
        .and_then(|raw| raw.parse::<usize>().ok())
        .filter(|cap| *cap > 0 && *cap <= MAX_OUTPUT as usize)
        .ok_or_else(|| error("Invalid private native Source coordinate byte cap"))?;
    Ok(Some(cap))
}
fn bounded_source_coordinate_input<R: Read>(input: R, cap: usize) -> Result<Vec<u8>, CliError> {
    let mut bytes = Vec::new();
    input
        .take(cap.min(MAX_INPUT) as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(error)?;
    if bytes.len() > cap.min(MAX_INPUT) {
        return Err(error("Native Source coordinate input exceeds its byte cap"));
    }
    Ok(bytes)
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    if args.len() == 1 && args[0] == "capabilities" {
        return serde_json::to_string_pretty(&json!({"schema":"ql.nara-identity-capabilities/v1","operations":["inspect","calculate","transit","personal-current","personal-recompose","presence-consent"],"coordinate_operations":["coordinate","coordinate-content","coordinate-bundle","source-inventory"],"dialogue_operations":["context","delegate","enrichment","receive"],"dialogue_registry":"native-current-m-registry","dialogue_persistence_owner":"host","profile_schema":"ql.nara-identity-profile/v1","persistence_owner":"central","natal_provider":"Kerykeion","sky_snapshot_purposes":["requested","retained-occasion"],"sky_admission_schema":"ql.sky-admission/v1","provider_python":"uv-managed Python 3.13 with embedded providers/sky/requirements.txt; QL_NARA_PYTHON diagnostic override","provider_uv":"QL_NARA_UV, PATH, or ~/.local/bin/uv","input":"JSON profile on stdin or file","identity_offices":["birthdate-name","natal-chart","jungian-assessment","gene-keys","human-design","archetypal-quintessence"],"automatic_agent_or_model_invocation":false})).map_err(error);
    }
    let [operation, path] = args else {
        return Err(error(
            "usage: ql nara <inspect|calculate|transit|personal-current|personal-recompose|presence-consent|coordinate|coordinate-content|coordinate-bundle|source-inventory|context|delegate|enrichment|receive> <request.json|-> [--json]",
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
        "coordinate-content",
        "coordinate-bundle",
        "source-inventory",
        "context",
        "delegate",
        "enrichment",
        "receive",
    ]
    .contains(&operation.as_str())
    {
        return Err(error("unknown Nara operation"));
    }
    let source_cap = source_coordinate_cap(operation)?;
    if let Some(cap) = source_cap {
        // File and stdin are bounded before allocation, not after reading an
        // ordinary 2 MiB request into the Source capture horizon.
        let bytes = if path == "-" {
            bounded_source_coordinate_input(std::io::stdin(), cap)?
        } else {
            bounded_source_coordinate_input(fs::File::open(path).map_err(error)?, cap)?
        };
        return dialogue::source_coordinate_command(&bytes, cap);
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
    if [
        "coordinate",
        "coordinate-content",
        "coordinate-bundle",
        "source-inventory",
        "context",
        "delegate",
        "enrichment",
        "receive",
    ]
    .contains(&operation.as_str())
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
        let mut current = ql_mef::nara::current::personal_current_with_activity(
            &request.current["identity"],
            &request.current["transit"],
            Some(activity),
        )
        .map_err(error)?;
        if let Some(admission) = request.current.get("sky_admission") {
            let qualification =
                acknowledged_sky_admission(admission, &request.current["transit"]["sky"])?;
            current["sky_source_binding_qualification"] = qualification;
            // Activity changes neither the original dated event nor its
            // acknowledged admission. This does not attest freshness again.
            current["sky_admission"] = admission.clone();
        }
        return serde_json::to_string_pretty(&current).map_err(error);
    }
    if operation == "transit" {
        let request: Value = serde_json::from_slice(&bytes).map_err(error)?;
        return serde_json::to_string_pretty(
            &transit(&request, false, SnapshotPurpose::Requested)?.0,
        )
        .map_err(error);
    }
    if operation == "personal-current" {
        let request: PersonalCurrentRequest = serde_json::from_slice(&bytes).map_err(error)?;
        if request.schema != "ql.nara-personal-current-request/v1" {
            return Err(error("unsupported personal current request"));
        }
        request.profile.validate().map_err(error)?;
        let (transit, admission) = match (&request.sky_request, &request.sky_snapshot) {
            (Some(sky), None) => transit(sky, false, request.snapshot_purpose)?,
            (None, Some(sky)) => transit(sky, true, request.snapshot_purpose)?,
            _ => {
                return Err(error(
                    "personal current requires exactly one sky_request or sky_snapshot",
                ));
            }
        };
        let natal = calculate(&request.profile)?;
        let identity = request.profile.inspect(Some(&natal)).map_err(error)?;
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
        let mut current =
            ql_mef::nara::current::personal_current_with_activity(&identity, &transit, activity)
                .map_err(error)?;
        current["sky_admission"] = admission;
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

#[cfg(test)]
mod sky_source_tests {
    use super::*;

    fn known_legacy_sky() -> Value {
        serde_json::from_str(include_str!(
            "../../../fixtures/kernel/sky-snapshot-known-e6d-2026-09-15-v1.json"
        ))
        .unwrap()
    }

    #[test]
    fn retained_legacy_descriptor_has_a_separate_actual_native_sun_route() {
        let sky = known_legacy_sky();
        let before = sky.clone();
        let qualification = source_binding_qualification(&sky, true).unwrap();
        assert_eq!(sky, before);
        assert_eq!(qualification["legacy_descriptor_admitted"], true);
        assert_eq!(qualification["fresh_current_attested"], false);
        assert_eq!(
            qualification["native_sun_route"]["planet_coordinate"],
            "#2-5-0/1"
        );
        assert_eq!(
            qualification["native_sun_route"]["chakra_coordinate"],
            "#2-5-0/1-7"
        );
        assert_eq!(qualification["native_sun_route"]["chakra_index"], 7);
        assert_eq!(
            qualification["native_sun_route"]["source_revision"],
            "907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288"
        );
        assert!(
            qualification["native_sun_route"]["relations"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["source_kind"] == "PLANETARY_RESONANCE"
                    && r["relation_ref"] == "bimba:relation:710326725339389fb6905570")
        );
        assert!(source_binding_qualification(&sky, false).is_err());
    }

    #[test]
    fn known_provider_qualification_is_not_a_general_legacy_or_wrong_sun_exception() {
        for (group, key, value) in [
            ("provider", "adapter_sha256", json!("0".repeat(64))),
            ("source_binding", "registry_revision", json!("0".repeat(64))),
            ("source_binding", "header_sha256", json!("0".repeat(64))),
            ("source_binding", "sun_role", json!("Sun-to-wrong-centre")),
        ] {
            let mut sky = known_legacy_sky();
            sky[group][key] = value;
            assert!(source_binding_qualification(&sky, true).is_err());
        }
        // This unit tests the native route after provider admission, not a
        // replacement for complete provider digest/field validation.
        let sky: Value = serde_json::from_str(include_str!(
            "../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v2.json"
        ))
        .unwrap();
        let q = source_binding_qualification(&sky, false).unwrap();
        assert_eq!(q["legacy_descriptor_admitted"], false);
        assert_eq!(q["native_sun_route"]["chakra_index"], 7);
    }
}

#[cfg(test)]
mod source_coordinate_input_bounds_tests {
    use super::*;
    #[test]
    fn same_coordinate_input_has_exact_limit_and_limit_plus_one_refusal() {
        let input = br##"{"coordinate_ref":"#3","face":"bimba"}"##;
        assert_eq!(
            bounded_source_coordinate_input(std::io::Cursor::new(input), input.len()).unwrap(),
            input
        );
        assert!(
            bounded_source_coordinate_input(std::io::Cursor::new(input), input.len() - 1).is_err()
        );
        assert!(bounded_source_coordinate_input(std::io::Cursor::new(input), 0).is_err());
    }
}
