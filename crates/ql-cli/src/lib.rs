use ql_core::{KERNEL_VERSION, QlAddress, QlOperator, apply_operator, kernel_capabilities};
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
Usage:\n  ql --version\n  ql capabilities [--json]\n  ql kernel capabilities [--json]\n  ql kernel apply <operator> <ql-address> [--json]\n  ql service capabilities [--json]\n  ql service negotiate <capabilities|locate|refract|relate|synthesise> [--json]\n  ql verify [--json]\n\n\
Current deterministic operators: conjugate-address, complement-address, classify-four-plus-two.\nProvider-backed service operations disclose their current negotiated availability.",
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
        commands: vec![
            "kernel.capabilities",
            "kernel.apply",
            "service.capabilities",
            "service.negotiate",
            "verify",
        ],
    };
    if json {
        serde_json::to_string_pretty(&view).map_err(CliError::from)
    } else {
        Ok(format!(
            "Quaternal Logic {}\nkernel: {}\ndeterministic operators: {}\nservice provider: {}",
            view.version,
            view.kernel.kernel_version,
            view.kernel.deterministic_operations.join(", "),
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
    fn native_verify_exercises_kernel_and_service_negotiation() {
        let output = execute_cli(&["verify".into(), "--json".into()]).unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value["status"], "ok");
    }
}
