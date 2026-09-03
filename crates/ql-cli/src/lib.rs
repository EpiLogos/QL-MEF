use ql_core::{KERNEL_VERSION, QlAddress, QlOperator, apply_operator, kernel_capabilities};
use ql_mef::{
    CONTEXT_FRAME_GRAMMAR_VERSION, ContextFrameId, LensFace, MEF_REGISTRY_REVISION,
    MEF_REGISTRY_VERSION, MefSquare, VAK_ENTRY_COUNT, VAK_SOURCE_GIT_BLOB, VAK_SOURCE_PATH,
    VAK_SOURCE_REPOSITORY, VAK_SOURCE_REVISION, VakRegistry, all_lens_definitions,
};
use ql_semantic::{Operation, ProviderState};
use ql_service::QlService;
use serde::Serialize;
use std::error::Error;
use std::fmt::{self, Display};
use std::process::ExitCode;
use std::str::FromStr;

pub const QL_CLI_CONTRACT: &str = "ql.cli/v1";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KernelCapabilitiesView {
    kernel_version: &'static str,
    schema_version: &'static str,
    supported_forms: Vec<String>,
    deterministic_operations: Vec<&'static str>,
    stochastic_operations: Vec<&'static str>,
    research_operations: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServiceCapabilitiesView {
    provider_state: &'static str,
    detail: Option<String>,
    operations: Vec<ServiceOperationView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServiceOperationView {
    operation: &'static str,
    supported: bool,
    deterministic: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CliCapabilitiesView {
    contract: &'static str,
    product: &'static str,
    version: &'static str,
    kernel: KernelCapabilitiesView,
    service: ServiceCapabilitiesView,
    mef_registry_version: &'static str,
    mef_registry_revision: u16,
    context_frame_grammar_version: &'static str,
    vak_source_revision: &'static str,
    commands: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ApplyView {
    contract: &'static str,
    operation: &'static str,
    input: String,
    output: String,
    schema_version: &'static str,
    kernel_version: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VerificationView {
    contract: &'static str,
    product: &'static str,
    version: &'static str,
    status: &'static str,
    checks: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MefLensView {
    lens_ref: String,
    code: &'static str,
    name: &'static str,
    face: &'static str,
    square: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MefRegistryView {
    registry_version: &'static str,
    registry_revision: u16,
    lenses: Vec<MefLensView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContextFrameView {
    code: &'static str,
    expression: &'static str,
    name: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContextFrameRegistryView {
    grammar_version: &'static str,
    frames: Vec<ContextFrameView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VakRegistryView {
    source_repository: &'static str,
    source_revision: &'static str,
    source_path: &'static str,
    source_git_blob: &'static str,
    entry_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VakEntryView {
    vak_ref: String,
    name: Option<String>,
    symbol: Option<String>,
    primary_designation: Option<String>,
    standing: &'static str,
    source_line: usize,
}

pub fn cli_main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match execute_cli(&args) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("ql: {error}");
            ExitCode::from(2)
        }
    }
}

pub fn execute_cli(args: &[String]) -> Result<String, CliError> {
    let mut args = args.to_vec();
    let json = remove_flag(&mut args, "--json");
    match args.first().map(String::as_str) {
        None | Some("help") | Some("--help") | Some("-h") => Ok(help()),
        Some("--version") | Some("version") => Ok(format!("ql {}", env!("CARGO_PKG_VERSION"))),
        Some("capabilities") => render_capabilities(json),
        Some("kernel") => kernel_command(&args[1..], json),
        Some("mef") => mef_command(&args[1..], json),
        Some("context-frame") => context_frame_command(&args[1..], json),
        Some("vak") => vak_command(&args[1..], json),
        Some("service") => service_command(&args[1..], json),
        Some("verify") => verify_command(json),
        Some(command) => Err(CliError(format!(
            "unknown command `{command}`; run `ql help`"
        ))),
    }
}

fn help() -> String {
    format!(
        "Quaternal Logic {}\n\n\
Usage:\n  ql --version\n  ql capabilities [--json]\n  ql kernel capabilities [--json]\n  ql kernel apply <operator> <ql-address> [--json]\n  ql mef lenses [--json]\n  ql context-frame list [--json]\n  ql vak capabilities [--json]\n  ql vak locate <vak-ref> [--json]\n  ql service capabilities [--json]\n  ql service negotiate <capabilities|locate|refract|relate|synthesise> [--json]\n  ql verify [--json]\n\n\
The CLI projects accepted QL kernel, MEF registry, Context-Frame, Vāk registry, and service contracts.\nCurrent deterministic kernel operators: conjugate-address, complement-address, classify-four-plus-two.\nProvider-backed service operations disclose their current negotiated availability.",
        env!("CARGO_PKG_VERSION")
    )
}

fn render_capabilities(json: bool) -> Result<String, CliError> {
    let view = CliCapabilitiesView {
        contract: QL_CLI_CONTRACT,
        product: "quaternal-logic",
        version: env!("CARGO_PKG_VERSION"),
        kernel: kernel_view(),
        service: service_view(&QlService::new()),
        mef_registry_version: MEF_REGISTRY_VERSION,
        mef_registry_revision: MEF_REGISTRY_REVISION,
        context_frame_grammar_version: CONTEXT_FRAME_GRAMMAR_VERSION,
        vak_source_revision: VAK_SOURCE_REVISION,
        commands: vec![
            "kernel.capabilities",
            "kernel.apply",
            "mef.lenses",
            "context-frame.list",
            "vak.capabilities",
            "vak.locate",
            "service.capabilities",
            "service.negotiate",
            "verify",
        ],
    };
    if json {
        serde_json::to_string_pretty(&view).map_err(CliError::from)
    } else {
        Ok(format!(
            "Quaternal Logic {}\nkernel: {}\nMEF: {}@{}\nContext Frame: {}\nVāk source: {}\nservice provider: {}",
            view.version,
            view.kernel.kernel_version,
            view.mef_registry_version,
            view.mef_registry_revision,
            view.context_frame_grammar_version,
            view.vak_source_revision,
            view.service.provider_state
        ))
    }
}

fn kernel_command(args: &[String], json: bool) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some("capabilities") => {
            let view = kernel_view();
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(format!(
                    "kernel {} / schema {}\nforms: {}\noperators: {}",
                    view.kernel_version,
                    view.schema_version,
                    view.supported_forms.join(", "),
                    view.deterministic_operations.join(", ")
                ))
            }
        }
        Some("apply") => {
            let operator = args
                .get(1)
                .ok_or_else(|| CliError("missing kernel operator".into()))?;
            let address = args
                .get(2)
                .ok_or_else(|| CliError("missing QL address".into()))?;
            let operator =
                QlOperator::from_str(operator).map_err(|error| CliError(error.to_string()))?;
            let address =
                QlAddress::from_str(address).map_err(|error| CliError(error.to_string()))?;
            let result = apply_operator(operator, address);
            let view = ApplyView {
                contract: QL_CLI_CONTRACT,
                operation: result.provenance.operation,
                input: result.provenance.input,
                output: result.provenance.output,
                schema_version: result.provenance.schema_version,
                kernel_version: result.provenance.kernel_version,
            };
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(format!(
                    "{} {} -> {}",
                    view.operation, view.input, view.output
                ))
            }
        }
        Some(operation) => Err(CliError(format!("unknown kernel operation `{operation}`"))),
        None => Err(CliError("missing kernel operation".into())),
    }
}

fn mef_command(args: &[String], json: bool) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some("lenses") => {
            let view = mef_registry_view();
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(view
                    .lenses
                    .iter()
                    .map(|lens| {
                        format!(
                            "{}\t{}\t{}\t{}",
                            lens.lens_ref, lens.name, lens.face, lens.square
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
        }
        Some(operation) => Err(CliError(format!("unknown MEF operation `{operation}`"))),
        None => Err(CliError("missing MEF operation".into())),
    }
}

fn context_frame_command(args: &[String], json: bool) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some("list") => {
            let view = context_frame_registry_view();
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(view
                    .frames
                    .iter()
                    .map(|frame| format!("{}\t{}\t{}", frame.code, frame.expression, frame.name))
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
        }
        Some(operation) => Err(CliError(format!(
            "unknown Context-Frame operation `{operation}`"
        ))),
        None => Err(CliError("missing Context-Frame operation".into())),
    }
}

fn vak_command(args: &[String], json: bool) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some("capabilities") => {
            let registry = VakRegistry::from_authoritative_source()
                .map_err(|error| CliError(error.to_string()))?;
            let view = VakRegistryView {
                source_repository: VAK_SOURCE_REPOSITORY,
                source_revision: VAK_SOURCE_REVISION,
                source_path: VAK_SOURCE_PATH,
                source_git_blob: VAK_SOURCE_GIT_BLOB,
                entry_count: registry.len(),
            };
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(format!(
                    "Vāk registry: {} entries\nsource: {}@{}\npath: {}\nblob: {}",
                    view.entry_count,
                    view.source_repository,
                    view.source_revision,
                    view.source_path,
                    view.source_git_blob
                ))
            }
        }
        Some("locate") => {
            let reference = args
                .get(1)
                .ok_or_else(|| CliError("missing Vāk ref".into()))?;
            let registry = VakRegistry::from_authoritative_source()
                .map_err(|error| CliError(error.to_string()))?;
            let entry = registry
                .locate_str(reference)
                .map_err(|error| CliError(error.to_string()))?;
            let view = VakEntryView {
                vak_ref: entry.vak_ref.to_string(),
                name: entry.name.clone(),
                symbol: entry.symbol.clone(),
                primary_designation: entry.primary_designation.clone(),
                standing: entry.source.standing.as_schema_str(),
                source_line: entry.source.source_line,
            };
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(format!(
                    "{}\nname: {}\nsymbol: {}\nstanding: {}\nsource line: {}",
                    view.vak_ref,
                    view.name.as_deref().unwrap_or("-"),
                    view.symbol.as_deref().unwrap_or("-"),
                    view.standing,
                    view.source_line
                ))
            }
        }
        Some(operation) => Err(CliError(format!("unknown Vāk operation `{operation}`"))),
        None => Err(CliError("missing Vāk operation".into())),
    }
}

fn service_command(args: &[String], json: bool) -> Result<String, CliError> {
    let service = QlService::new();
    match args.first().map(String::as_str) {
        Some("capabilities") => {
            let view = service_view(&service);
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(format!(
                    "service provider: {}\n{}",
                    view.provider_state,
                    view.operations
                        .iter()
                        .map(|operation| format!(
                            "{}: supported={}, deterministic={}",
                            operation.operation, operation.supported, operation.deterministic
                        ))
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
            }
        }
        Some("negotiate") => {
            let operation = args
                .get(1)
                .ok_or_else(|| CliError("missing service operation".into()))?;
            let operation = parse_operation(operation)?;
            let decision = service.negotiate(operation);
            let view = ServiceOperationView {
                operation: operation.as_str(),
                supported: decision.supported,
                deterministic: decision.deterministic,
            };
            if json {
                serde_json::to_string_pretty(&view).map_err(CliError::from)
            } else {
                Ok(format!(
                    "{}: supported={}, deterministic={}, provider={}",
                    view.operation,
                    view.supported,
                    view.deterministic,
                    provider_state(decision.health.state)
                ))
            }
        }
        Some(operation) => Err(CliError(format!("unknown service operation `{operation}`"))),
        None => Err(CliError("missing service operation".into())),
    }
}

fn kernel_view() -> KernelCapabilitiesView {
    let capabilities = kernel_capabilities();
    KernelCapabilitiesView {
        kernel_version: capabilities.kernel_version,
        schema_version: capabilities.schema_version,
        supported_forms: capabilities
            .supported_forms
            .into_iter()
            .map(|value| value.to_string())
            .collect(),
        deterministic_operations: capabilities
            .deterministic_operations
            .into_iter()
            .map(QlOperator::as_str)
            .collect(),
        stochastic_operations: capabilities.stochastic_operations.to_vec(),
        research_operations: capabilities.research_operations.to_vec(),
    }
}

fn mef_registry_view() -> MefRegistryView {
    let lenses = all_lens_definitions()
        .iter()
        .copied()
        .map(|definition| MefLensView {
            lens_ref: definition.reference().to_string(),
            code: definition.id().code(),
            name: definition.name(),
            face: lens_face(definition.id().face()),
            square: mef_square(definition.square()),
        })
        .collect();
    MefRegistryView {
        registry_version: MEF_REGISTRY_VERSION,
        registry_revision: MEF_REGISTRY_REVISION,
        lenses,
    }
}

fn context_frame_registry_view() -> ContextFrameRegistryView {
    ContextFrameRegistryView {
        grammar_version: CONTEXT_FRAME_GRAMMAR_VERSION,
        frames: ContextFrameId::ALL
            .into_iter()
            .map(|frame| ContextFrameView {
                code: frame.code(),
                expression: frame.expression(),
                name: frame.name(),
            })
            .collect(),
    }
}

fn service_view(service: &QlService) -> ServiceCapabilitiesView {
    let capabilities = service.capabilities();
    let operations = [
        Operation::Capabilities,
        Operation::Locate,
        Operation::Refract,
        Operation::Relate,
        Operation::Synthesise,
    ]
    .into_iter()
    .map(|operation| {
        let decision = service.negotiate(operation);
        ServiceOperationView {
            operation: operation.as_str(),
            supported: decision.supported,
            deterministic: decision.deterministic,
        }
    })
    .collect();
    ServiceCapabilitiesView {
        provider_state: provider_state(capabilities.health.state),
        detail: capabilities.health.detail,
        operations,
    }
}

fn verify_command(json: bool) -> Result<String, CliError> {
    let address = QlAddress::from_str("qladdr:sixfold@1/direct/P5/d0")
        .map_err(|error| CliError(error.to_string()))?;
    let conjugated = apply_operator(QlOperator::ConjugateAddress, address);
    let complemented = apply_operator(QlOperator::ComplementAddress, address);
    let classified = apply_operator(QlOperator::ClassifyFourPlusTwo, address);
    if conjugated.provenance.output != "qladdr:sixfold@1/conjugate/P5/d0"
        || complemented.provenance.output != "qladdr:sixfold@1/direct/P0/d0"
        || classified.provenance.output != "implicate"
    {
        return Err(CliError("deterministic kernel verification failed".into()));
    }
    if all_lens_definitions().len() != 12 || ContextFrameId::ALL.len() != 7 {
        return Err(CliError(
            "MEF/Context-Frame registry verification failed".into(),
        ));
    }
    let vak =
        VakRegistry::from_authoritative_source().map_err(|error| CliError(error.to_string()))?;
    if vak.len() != VAK_ENTRY_COUNT {
        return Err(CliError("Vāk source registry verification failed".into()));
    }
    let service = QlService::new();
    if !service.negotiate(Operation::Capabilities).supported
        || service.negotiate(Operation::Locate).supported
    {
        return Err(CliError(
            "service capability negotiation verification failed".into(),
        ));
    }
    let result = VerificationView {
        contract: QL_CLI_CONTRACT,
        product: "quaternal-logic",
        version: env!("CARGO_PKG_VERSION"),
        status: "ok",
        checks: vec![
            "kernel.conjugate-address",
            "kernel.complement-address",
            "kernel.classify-four-plus-two",
            "mef.registry",
            "context-frame.registry",
            "vak.source-registry",
            "service.capability-negotiation",
        ],
    };
    if json {
        serde_json::to_string_pretty(&result).map_err(CliError::from)
    } else {
        Ok(format!(
            "Quaternal Logic native verification: ok (kernel {}, {} checks)",
            KERNEL_VERSION,
            result.checks.len()
        ))
    }
}

fn parse_operation(value: &str) -> Result<Operation, CliError> {
    match value {
        "capabilities" => Ok(Operation::Capabilities),
        "locate" => Ok(Operation::Locate),
        "refract" => Ok(Operation::Refract),
        "relate" => Ok(Operation::Relate),
        "synthesise" => Ok(Operation::Synthesise),
        other => Err(CliError(format!("unknown service operation `{other}`"))),
    }
}

fn provider_state(state: ProviderState) -> &'static str {
    match state {
        ProviderState::Absent => "absent",
        ProviderState::Available => "available",
        ProviderState::Degraded => "degraded",
        ProviderState::Incompatible => "incompatible",
    }
}

fn lens_face(face: LensFace) -> &'static str {
    match face {
        LensFace::Day => "day",
        LensFace::Night => "night",
    }
}

fn mef_square(square: MefSquare) -> &'static str {
    match square {
        MefSquare::Articulation => "articulation",
        MefSquare::Encounter => "encounter",
        MefSquare::Becoming => "becoming",
    }
}

fn remove_flag(args: &mut Vec<String>, flag: &str) -> bool {
    let before = args.len();
    args.retain(|arg| arg != flag);
    args.len() != before
}

#[derive(Debug)]
pub struct CliError(String);

impl Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for CliError {}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_and_capabilities_are_stable() {
        assert_eq!(
            execute_cli(&["--version".into()]).unwrap(),
            format!("ql {}", env!("CARGO_PKG_VERSION"))
        );
        let output = execute_cli(&["capabilities".into(), "--json".into()]).unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value["contract"], QL_CLI_CONTRACT);
        assert_eq!(value["product"], "quaternal-logic");
        assert_eq!(value["service"]["providerState"], "absent");
        assert_eq!(value["mefRegistryVersion"], MEF_REGISTRY_VERSION);
        assert_eq!(
            value["contextFrameGrammarVersion"],
            CONTEXT_FRAME_GRAMMAR_VERSION
        );
        assert_eq!(value["vakSourceRevision"], VAK_SOURCE_REVISION);
    }

    #[test]
    fn kernel_apply_projects_real_deterministic_operator() {
        let output = execute_cli(&[
            "kernel".into(),
            "apply".into(),
            "conjugate-address".into(),
            "qladdr:sixfold@1/direct/P2/d0".into(),
            "--json".into(),
        ])
        .unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value["output"], "qladdr:sixfold@1/conjugate/P2/d0");
        assert_eq!(value["operation"], "conjugate-address");
    }

    #[test]
    fn mef_and_context_frame_are_native_registry_projections() {
        let mef = execute_cli(&["mef".into(), "lenses".into(), "--json".into()]).unwrap();
        let mef: serde_json::Value = serde_json::from_str(&mef).unwrap();
        assert_eq!(mef["registryRevision"], MEF_REGISTRY_REVISION);
        assert_eq!(mef["lenses"].as_array().unwrap().len(), 12);
        assert_eq!(mef["lenses"][0]["lensRef"], "mef:lens:L0@1");

        let frames =
            execute_cli(&["context-frame".into(), "list".into(), "--json".into()]).unwrap();
        let frames: serde_json::Value = serde_json::from_str(&frames).unwrap();
        assert_eq!(frames["grammarVersion"], CONTEXT_FRAME_GRAMMAR_VERSION);
        assert_eq!(frames["frames"].as_array().unwrap().len(), 7);
        assert_eq!(frames["frames"][4]["code"], "CF5");
    }

    #[test]
    fn vak_commands_consume_source_locked_registry() {
        let capabilities =
            execute_cli(&["vak".into(), "capabilities".into(), "--json".into()]).unwrap();
        let capabilities: serde_json::Value = serde_json::from_str(&capabilities).unwrap();
        assert_eq!(capabilities["sourceRevision"], VAK_SOURCE_REVISION);
        assert_eq!(capabilities["entryCount"], VAK_ENTRY_COUNT);

        let located =
            execute_cli(&["vak".into(), "locate".into(), "M0".into(), "--json".into()]).unwrap();
        let located: serde_json::Value = serde_json::from_str(&located).unwrap();
        assert_eq!(located["vakRef"], "M0");
        assert_eq!(located["standing"], "SOURCE");
    }

    #[test]
    fn absent_provider_is_truthfully_negotiated() {
        let output = execute_cli(&[
            "service".into(),
            "negotiate".into(),
            "refract".into(),
            "--json".into(),
        ])
        .unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value["supported"], false);
        assert_eq!(value["deterministic"], false);
    }

    #[test]
    fn native_verify_exercises_accepted_native_families() {
        let output = execute_cli(&["verify".into(), "--json".into()]).unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value["status"], "ok");
        assert_eq!(value["checks"].as_array().unwrap().len(), 7);
    }
}
