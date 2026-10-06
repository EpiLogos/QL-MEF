//! `ql stage`: the Ta-Onta procedural stage driven through the native field
//! host the Expressions application opens (QL-MEF #296). One invocation is one
//! owned exchange: open the scene host through the app's own binding route,
//! send one stage operation under the exact request envelope, and print the
//! host's receipt — the last acknowledged field travels with every refusal.
use crate::CliError;
use ql_mef::continuous::host::{FieldHost, HostOperation, HostRequest};
use ql_mef::continuous::scene_field::{self, BindingRequest, SceneConfig, SceneGeometry};
use ql_mef::continuous::stage::StageProcedure;
use serde_json::Value;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(20);
/// The standing scene the application's own opening composes; a fresh agent
/// needs no event of its own to drive the stage (acceptance A17).
const DEFAULT_INSTANCE: &str = "ql-cli:stage";

fn usage() -> String {
    "usage: ql stage state [--config <binding-request.json|->] [--worker <path>] [--instance <ref>]\n\
     usage: ql stage evaluate <procedure.json|-> [--config <path|->] [--worker <path>] [--instance <ref>]\n\
     usage: ql stage bind <procedure.json|-> [--max-evaluations N] [--config <path|->] [--worker <path>] [--instance <ref>]\n\
     usage: ql stage unbind <procedure_ref> [--config <path|->] [--worker <path>] [--instance <ref>]\n\
     usage: ql stage retire <procedure_ref> [--config <path|->] [--worker <path>] [--instance <ref>]"
        .into()
}

fn read(path: &str) -> Result<Vec<u8>, CliError> {
    let mut bytes = Vec::new();
    if path == "-" {
        std::io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|e| CliError(e.to_string()))?;
    } else {
        bytes = std::fs::read(path).map_err(|e| CliError(e.to_string()))?;
    }
    Ok(bytes)
}

/// The installed-suite conventions this CLI already uses: an explicit
/// `--worker <path>` (as `ql scene render` requires) or the shared
/// `QL_FIELD_WORKER` environment the native tests standardise on. Nothing is
/// invented here: with neither present the command refuses and names both.
fn resolve_worker(explicit: Option<&str>) -> Result<PathBuf, CliError> {
    let path = match explicit {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(std::env::var_os("QL_FIELD_WORKER").ok_or_else(|| {
            CliError(
                "the native worker must be named: pass --worker <path> or set \
                 QL_FIELD_WORKER (installed suites install it under \
                 target/<suite>/bin/ql-field-worker, e.g. target/ta-onta-cpp/bin)"
                    .into(),
            )
        })?),
    };
    if !path.is_file() {
        return Err(CliError(format!("missing worker {}", path.display())));
    }
    Ok(path)
}

/// The application's own plain opening: its composed default scene binding, or
/// the caller's `ql.scene-binding-request/v1` through the same
/// `scene_field::binding` route. Either way one supervised native worker
/// stands behind the returned host configuration.
fn binding_request(instance: &str, config: Option<&str>) -> Result<BindingRequest, CliError> {
    let mut request: BindingRequest = match config {
        Some(path) => serde_json::from_slice(&read(path)?).map_err(|e| CliError(e.to_string()))?,
        // The default mirrors the live stage E2E's opening: the QL default
        // event on the retained 32x16 presentation.
        None => BindingRequest {
            schema: scene_field::BINDING_REQUEST.into(),
            instance_ref: instance.into(),
            texture: [32, 16],
            units_per_metre: 1.0,
            event: None,
            sky: None,
            field: None,
            geometry: Some(SceneGeometry {
                longitude_samples: 32,
                latitude_samples: 16,
                metres_per_unit: 1.0,
                attachment: 1,
            }),
            material: None,
            reception: None,
        },
    };
    if instance != DEFAULT_INSTANCE {
        request.instance_ref = instance.into();
    }
    Ok(request)
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, CliError> {
    value[key]
        .as_str()
        .ok_or_else(|| CliError(format!("the host receipt carried no {key}")))
}

/// One owned exchange with one host: the ready acknowledgement, then one stage
/// operation under the exact sequence/cursor envelope the field host admits.
struct Driver {
    host: FieldHost,
    current: Value,
}

impl Driver {
    fn open(worker: &Path, request: BindingRequest) -> Result<Self, CliError> {
        let binding = scene_field::binding(request).map_err(CliError)?;
        let config: SceneConfig =
            serde_json::from_value(binding["host"].clone()).map_err(|e| CliError(e.to_string()))?;
        let host = FieldHost::open_scene(worker, config, TIMEOUT).map_err(CliError)?;
        let current = host.ready();
        if current["status"] != "ready" {
            return Err(CliError("the opened host never became ready".into()));
        }
        Ok(Self { host, current })
    }

    /// The admitted envelope: the receipt's own cursors, the next sequence,
    /// and the caller's expected generation/samples exactly as the driver in
    /// the live stage E2E composes them.
    fn send(&mut self, command: HostOperation) -> Result<Value, CliError> {
        let request = HostRequest {
            schema: ql_mef::continuous::host::HOST_REQUEST.into(),
            instance_ref: text(&self.current, "instance_ref")?.into(),
            event_ref: text(&self.current["field"], "event_ref")?.into(),
            subject_ref: text(&self.current["field"], "subject_ref")?.into(),
            request_id: (text(&self.current, "last_request_id")?
                .parse::<u64>()
                .map_err(|_| {
                    CliError("the host receipt carried a noncanonical sequence".into())
                })?
                + 1)
            .to_string(),
            expected_generation: text(&self.current["field"], "generation")?.into(),
            expected_samples_elapsed: text(&self.current["field"], "samples_elapsed")?.into(),
            command,
        };
        self.current = self.host.execute(request);
        Ok(self.current.clone())
    }
}

/// Success prints the host's receipt document; a refusal prints the same
/// receipt (the last acknowledged field inside it) on stdout and fails with
/// the cause on stderr, so a scripted agent loses nothing by either outcome.
fn settle(
    operation: &'static str,
    driver: &mut Driver,
    command: HostOperation,
) -> Result<String, CliError> {
    let response = driver.send(command)?;
    if response["status"] != "ok" {
        let document =
            serde_json::to_string_pretty(&response).map_err(|e| CliError(e.to_string()))?;
        println!("{document}");
        return Err(CliError(format!(
            "stage {operation} {}: {}",
            response["status"].as_str().unwrap_or("refused"),
            response["error"].as_str().unwrap_or("unnamed refusal")
        )));
    }
    serde_json::to_string_pretty(&response).map_err(|e| CliError(e.to_string()))
}

/// The stage owner's own loading and validation, shared by evaluate and bind:
/// strict serde on the versioned contract, then `StageProcedure::validate`.
fn load_procedure(bytes: &[u8]) -> Result<StageProcedure, CliError> {
    let procedure: StageProcedure =
        serde_json::from_slice(bytes).map_err(|e| CliError(e.to_string()))?;
    procedure.validate().map_err(CliError)?;
    Ok(procedure)
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    let operation = args.first().ok_or_else(|| CliError(usage()))?;
    if !matches!(
        operation.as_str(),
        "state" | "evaluate" | "bind" | "unbind" | "retire"
    ) {
        return Err(CliError(usage()));
    }
    let mut worker = None;
    let mut config = None;
    let mut instance = DEFAULT_INSTANCE.to_owned();
    let mut instance_given = false;
    let mut max_evaluations = None;
    let mut positional: Option<&String> = None;
    let mut rest = &args[1..];
    if operation.as_str() != "state" {
        positional = Some(rest.first().ok_or_else(|| CliError(usage()))?);
        rest = &rest[1..];
    }
    let mut i = 0;
    while i < rest.len() {
        let flag = rest[i].as_str();
        let value = || {
            rest.get(i + 1)
                .ok_or_else(|| CliError(format!("stage flag `{flag}` needs a value")))
        };
        match flag {
            "--worker" => worker = Some(value()?.clone()),
            "--config" => config = Some(value()?.clone()),
            "--instance" => {
                instance = value()?.clone();
                instance_given = true;
            }
            "--max-evaluations" => {
                let value = value()?;
                max_evaluations = Some(value.parse::<u32>().map_err(|_| {
                    CliError(format!(
                        "--max-evaluations must be a whole number, not `{value}`"
                    ))
                })?);
            }
            other => return Err(CliError(format!("unknown stage flag `{other}`"))),
        }
        i += 2;
    }
    // Cheap refusals come first: the stage owner's own validation runs before
    // any worker is spawned, and an unnameable worker refuses before transport.
    let procedure = match (operation.as_str(), positional) {
        ("evaluate" | "bind", Some(path)) => Some(load_procedure(&read(path)?)?),
        _ => None,
    };
    let worker = resolve_worker(worker.as_deref())?;
    let mut request = binding_request(&instance, config.as_deref())?;
    if instance_given {
        request.instance_ref = instance.clone();
    }
    let mut driver = Driver::open(&worker, request)?;
    match (operation.as_str(), procedure) {
        ("state", _) => settle("state", &mut driver, HostOperation::StageState {}),
        ("evaluate", Some(procedure)) => settle(
            "evaluate",
            &mut driver,
            HostOperation::StageEvaluate {
                procedure: Box::new(procedure),
            },
        ),
        ("bind", Some(procedure)) => settle(
            "bind",
            &mut driver,
            HostOperation::StageBind {
                procedure: Box::new(procedure),
                max_evaluations,
            },
        ),
        ("unbind", _) => settle(
            "unbind",
            &mut driver,
            HostOperation::StageUnbind {
                procedure_ref: positional.expect("checked above").clone(),
            },
        ),
        ("retire", _) => settle(
            "retire",
            &mut driver,
            HostOperation::StageRetire {
                procedure_ref: positional.expect("checked above").clone(),
            },
        ),
        _ => Err(CliError(usage())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ql_mef::continuous::stage::{STAGE_PROCEDURE, StageChange, StagePassage, StageTrigger};
    use ql_mef::m3_state::M3Operation;

    /// The committed fixture is real stage DATA: it loads through the same
    /// serde + validation the CLI performs, and it exercises the A17 shape —
    /// a form fold plus a generated passage on the default scene's subject.
    #[test]
    fn the_committed_fixture_loads_and_validates_through_the_stage_owner() {
        let bytes = include_str!("../../../fixtures/kernel/stage-procedure-example-v1.json");
        let procedure = load_procedure(bytes.as_bytes()).unwrap();
        assert_eq!(procedure.schema, STAGE_PROCEDURE);
        assert_eq!(procedure.procedure_ref, "ta-onta:stage:cli-fold");
        assert_eq!(procedure.subject_ref, "ql:k2/default-subject");
        assert!(matches!(procedure.trigger, StageTrigger::Invocation));
        assert_eq!(procedure.selector, vec!["form".to_owned()]);
        assert_eq!(procedure.changes.len(), 1);
        assert!(
            matches!(&procedure.changes[0], StageChange::Form { operations }
            if operations.len() == 2
                && matches!(operations[0], M3Operation::CastCreases { .. })
                && matches!(operations[1], M3Operation::SetPose { pose: 2 }))
        );
        let passage: StagePassage = procedure.passage.expect("the example carries a passage");
        assert_eq!(
            passage.scenes, 3,
            "the invoked fold plus two generated scenes"
        );
    }

    #[test]
    fn the_fixture_is_accepted_by_the_stage_evaluation_law_itself() {
        let bytes = include_str!("../../../fixtures/kernel/stage-procedure-example-v1.json");
        let procedure: StageProcedure = serde_json::from_str(bytes).unwrap();
        // Evaluation runs the full validate() plus slot-claiming against a
        // fresh ownership; the example must plan cleanly with no host at all.
        let live: ql_mef::continuous::coupled::CoupledInput = serde_json::from_str(include_str!(
            "../../../fixtures/kernel/scene-default-event-v2.json"
        ))
        .unwrap();
        let mut ownership = ql_mef::continuous::stage::StageOwnership::default();
        let plan = ql_mef::continuous::stage::evaluate(
            &procedure,
            &live,
            &mut ownership,
            live.m2.stamp.identity.profile_generation,
            0,
            0,
        )
        .unwrap();
        assert_eq!(plan.procedure_ref, "ta-onta:stage:cli-fold");
        assert_eq!(plan.passage_ticks, 2);
        assert_eq!(
            plan.contributions.len(),
            2,
            "the form claim and the passage claim"
        );
    }

    #[test]
    fn usage_and_unknown_flags_refuse_before_any_worker_is_named() {
        let error = command(&[]).unwrap_err();
        assert!(error.to_string().contains("usage: ql stage"), "{error}");
        let error = command(&["polish".into()]).unwrap_err();
        assert!(error.to_string().contains("usage: ql stage"), "{error}");
        let error = command(&["state".into(), "--surprise".into(), "1".into()]).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("unknown stage flag `--surprise`"),
            "{error}"
        );
        let error = command(&["evaluate".into()]).unwrap_err();
        assert!(error.to_string().contains("usage: ql stage"), "{error}");
        let error =
            command(&["bind".into(), "p.json".into(), "--max-evaluations".into()]).unwrap_err();
        assert!(error.to_string().contains("needs a value"), "{error}");
    }

    #[test]
    fn a_missing_worker_refuses_by_naming_both_admitted_routes() {
        let error = resolve_worker(Some("/nonexistent/ql-field-worker")).unwrap_err();
        assert!(error.to_string().contains("missing worker"), "{error}");
        let error = command(&["state".into()]).unwrap_err();
        assert!(
            error.to_string().contains("--worker") && error.to_string().contains("QL_FIELD_WORKER"),
            "{error}"
        );
    }

    #[test]
    fn a_nonnumeric_binding_budget_refuses_before_transport() {
        let error = command(&[
            "bind".into(),
            "procedure.json".into(),
            "--max-evaluations".into(),
            "many".into(),
        ])
        .unwrap_err();
        assert!(error.to_string().contains("whole number"), "{error}");
    }
}
