//! Bounded transport for the existing native scene owner.
//! Preparation returns the same Expression Edit used by the application.
//! This transport does not apply it, manufacture consumer ACKs or retain state.
use crate::CliError;
use ql_mef::MFace;
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::procedural_composition::{
    CurrentContribution, GeneratedContribution, NativeChange, PROCEDURE_CONTRACT, Procedure,
    ResolvedMembership, RetainedOutputReading, RuleEvent, RuleExecution, TargetReading,
    compile_native_batch, compile_native_regeneration_batch, instantiate_scene, regenerate,
    regeneration_native_changes, resolve_procedure_membership,
};
use ql_mef::procedural_manifestation::{
    ATLAS_TRANSITION_CONTRACT, MANIFESTATION_CONTRACT, ManifestationRequest, NativeSubject,
    PlaceOccurrence, SourceBasis, fingerprint, manifestation_inventory, native_procedural_bindings,
    prepare_atlas_transition, resolve_manifestation,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

pub const PROCEDURAL_REQUEST: &str = "ql.scene-procedural-request/v1";
pub const PROCEDURAL_RESPONSE: &str = "ql.scene-procedural-response/v1";
const MAX_INPUT_BYTES: usize = 1_048_576;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestRequest {
    schema: String,
    request: ManifestationRequest,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SceneOutput {
    output_slot: String,
    scene_ref: String,
    subjects: Vec<NativeSubject>,
    source_basis: SourceBasis,
    material_fingerprint: String,
    source_presentation: Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PrepareRequest {
    schema: String,
    procedure: Procedure,
    expression_ref: String,
    document_revision: u64,
    operation_ref: String,
    current_readings: Vec<TargetReading>,
    previous_membership: Option<ResolvedMembership>,
    contributions: Vec<GeneratedContribution>,
    scene_outputs: Vec<SceneOutput>,
    required_consumers: BTreeSet<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RegenerateRequest {
    schema: String,
    procedure: Procedure,
    expression_ref: String,
    document_revision: u64,
    operation_ref: String,
    current_readings: Vec<TargetReading>,
    previous_membership: Option<ResolvedMembership>,
    previous: Vec<GeneratedContribution>,
    current: Vec<CurrentContribution>,
    next: Vec<GeneratedContribution>,
    output_readings: Vec<RetainedOutputReading>,
    required_consumers: BTreeSet<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AtlasRequest {
    schema: String,
    source: PlaceOccurrence,
    destination: PlaceOccurrence,
    face: String,
    expected_destination_locus: String,
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
enum RuleAction {
    Start {
        interval_ref: String,
    },
    Enqueue {
        event: RuleEvent,
    },
    Next {
        generated_operations: usize,
    },
    Pause {
        observed_cursor: u64,
    },
    Resume {
        epoch_ref: String,
    },
    Seek {
        observed_cursor: u64,
        epoch_ref: String,
    },
    Cancel {
        observed_cursor: u64,
    },
    BeginInterval {
        interval_ref: String,
        observed_cursor: u64,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RuleRequest {
    schema: String,
    procedure: Procedure,
    checkpoint: Option<RuleExecution>,
    operation: RuleAction,
}

fn read(path: &str) -> Result<Vec<u8>, CliError> {
    let input: Box<dyn Read> = if path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path).map_err(|error| CliError(error.to_string()))?)
    };
    let mut bytes = Vec::new();
    input
        .take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| CliError(error.to_string()))?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(CliError(
            "procedural request exceeds one MiB admission".into(),
        ));
    }
    Ok(bytes)
}

fn request<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, CliError> {
    serde_json::from_slice(bytes).map_err(|error| CliError(error.to_string()))
}

fn schema(value: &str) -> Result<(), CliError> {
    if value != PROCEDURAL_REQUEST {
        return Err(CliError(
            "unsupported native scene procedure request".into(),
        ));
    }
    Ok(())
}

fn response(operation: &str, result: Value) -> Value {
    let registry = native_current_m_registry();
    json!({"schema":PROCEDURAL_RESPONSE,"operation":operation,"registry_revision":registry.manifest().registry_revision,
        "source_revision":registry.manifest().source_revision,"result":result})
}

fn discover() -> Value {
    let registry = native_current_m_registry();
    response(
        "discover",
        json!({"contracts":{"request":PROCEDURAL_REQUEST,"response":PROCEDURAL_RESPONSE,
            "manifestation":MANIFESTATION_CONTRACT,"composition":PROCEDURE_CONTRACT,"atlas":ATLAS_TRANSITION_CONTRACT},
        "route":"ql scene procedural <discover|manifest|prepare|regenerate|atlas|rule> [request.json|-] --json",
        "native_owners":{"source":"ql_mef::m_tree::MRegistry","manifestation":"ql_mef::procedural_manifestation::resolve_manifestation",
            "composition":"ql_mef::procedural_composition::compile_native_batch","rule":"ql_mef::procedural_composition::RuleExecution",
            "application":"O:I expression::Request::Edit","retention":"oi.journey-scene/v1 scene.procedural",
            "timing":"Procedure.timing.owner_ref; supplied native epoch/cursor","body":"#288 native body owner","audio":"#287 native audio/control owner"},
        "types":{"subject":"NativeSubject (existing flattened SubjectBinding)","locus":"exact native coordinate plus face/profile/source revision",
            "occurrence":"OccurrenceAddress","target_reading":"TargetReading{occurrence_ref,address,subject,revision,tags,properties}",
            "address":"OwnedAddress{expression_ref,scene_ref,entity_ref,component,constituent_ref,property}",
            "membership":"ResolvedMembership{selector,mode,containing_expression_ref,targets,addresses,joined,left}",
            "retained_output_reading":"RetainedOutputReading; current CAS + original native Operation; re-attested by O:I native owner",
            "procedure":"Procedure","contribution":"GeneratedContribution","prepared":"PreparedProcedure","checkpoint":"RuleExecution"},
        "components":["expression","scene","field","entity","force","layer","sequence","sequence_link","property","driver"],
        "native_changes":["scene_create","scene_remove","entity_add","entity_remove","subject_bind","scene_compose","scene_material_set","parameter_set","focus","scene_reorder"],
        "limits":{"input_bytes":MAX_INPUT_BYTES,"targets":2048,"active_instances":2048,"evaluations_per_interval":4096,
            "operations_per_interval":4096,"expansion_depth":64,"queue":4096,"conditions":64,"recipe_parameters":256,"retained_intervals":4096},
        "source":{"repository":registry.manifest().source_repository,"revision":registry.manifest().source_revision,
            "dataset_tree":registry.manifest().source_dataset_tree,"snapshot_sha256":registry.manifest().source_snapshot_sha256},
        "capability_standing":{"manifest":"native source/profile-qualified plan and binding producer","prepare":"prepared same native Edit; application/readback required",
            "regenerate":"stable contribution three-way diff; same native Edit; application/readback required","atlas":"focus transition over supplied continuing native occurrences",
            "rule":"bounded native checkpoint/event admission; state retained at existing owner",
            "arbitrary_inverse":"unavailable until original entity-to-form source determines the operation",
            "consumer_ack":"only actual scene/body/audio owners can attest application"},
        "source_obligations":manifestation_inventory(registry)}),
    )
}

fn regenerate_request(input: RegenerateRequest) -> Result<Value, CliError> {
    schema(&input.schema)?;
    let registry = native_current_m_registry();
    let membership = resolve_procedure_membership(
        registry,
        &input.procedure,
        &input.expression_ref,
        &input.current_readings,
        input.previous_membership.as_ref(),
    )
    .map_err(CliError)?;
    let delta = regenerate(
        &input.previous,
        &input.current,
        &input.next,
        input.procedure.removal_policy,
    )
    .map_err(CliError)?;
    let changes =
        regeneration_native_changes(&delta, &input.previous, &input.current).map_err(CliError)?;
    let mut by_scene: BTreeMap<String, Vec<NativeChange>> = BTreeMap::new();
    let mut entity_scenes = BTreeMap::new();
    for contribution in input.previous.iter().chain(&input.next) {
        if let Some(entities) = contribution.generated_basis["scene"]["entities"].as_array() {
            for entity in entities {
                let id = entity["id"].as_str().ok_or_else(|| {
                    CliError("generated scene lost stable constituent identity".into())
                })?;
                entity_scenes.insert(id.to_owned(), contribution.occurrence_ref.clone());
            }
        }
    }
    for change in changes {
        let scene = match &change {
            NativeChange::SceneCreate { scene_ref, .. }
            | NativeChange::SceneRemove { scene_ref }
            | NativeChange::SceneCompose { scene_ref, .. }
            | NativeChange::SceneMaterialSet { scene_ref, .. }
            | NativeChange::EntityAdd { scene_ref, .. }
            | NativeChange::Focus { scene_ref, .. } => scene_ref.clone(),
            NativeChange::EntityRemove { entity_ref }
            | NativeChange::SubjectBind { entity_ref, .. }
            | NativeChange::ParameterSet { entity_ref, .. } => {
                entity_scenes.get(entity_ref).cloned().ok_or_else(|| {
                    CliError("regenerated entity has no retained native scene".into())
                })?
            }
            NativeChange::SceneReorder { .. } => {
                return Err(CliError(
                    "scene-order revision belongs to an explicit whole-Expression recipe".into(),
                ));
            }
        };
        by_scene.entry(scene).or_default().push(change);
    }
    let mut contributions = Vec::new();
    for contribution in &input.next {
        if let Some(changes) = by_scene.remove(&contribution.occurrence_ref) {
            let mut compiled = contribution.clone();
            compiled.native_changes = changes;
            contributions.push(compiled);
        }
    }
    for reference in &delta.retired {
        let old = input
            .previous
            .iter()
            .find(|contribution| &contribution.contribution_ref == reference)
            .ok_or_else(|| CliError("retired source contribution is absent".into()))?;
        if let Some(changes) = by_scene.remove(&old.occurrence_ref) {
            let mut compiled = old.clone();
            compiled.recipe = input.procedure.recipe.clone();
            compiled.procedure_revision = input.procedure.revision.clone();
            compiled.native_changes = changes;
            contributions.push(compiled);
        }
    }
    if !by_scene.is_empty() {
        return Err(CliError(
            "native regeneration changes have no exact owned contribution".into(),
        ));
    }
    let prepared = if contributions.is_empty() {
        None
    } else {
        Some(
            compile_native_regeneration_batch(
                registry,
                &input.procedure,
                &input.operation_ref,
                &input.expression_ref,
                input.document_revision,
                membership.clone(),
                contributions,
                input.required_consumers,
                &input.previous,
                &input.current,
                input.output_readings,
            )
            .map_err(CliError)?,
        )
    };
    Ok(response(
        "regenerate",
        json!({"regeneration":delta,"membership":membership,"prepared":prepared}),
    ))
}

fn rule_request(input: RuleRequest) -> Result<Value, CliError> {
    schema(&input.schema)?;
    input
        .procedure
        .validate(native_current_m_registry())
        .map_err(CliError)?;
    let mut ready_event = None;
    let checkpoint = match input.operation {
        RuleAction::Start { interval_ref } => {
            if input.checkpoint.is_some() {
                return Err(CliError(
                    "start cannot discard a retained procedure checkpoint".into(),
                ));
            }
            RuleExecution::new(&input.procedure, &interval_ref).map_err(CliError)?
        }
        operation => {
            let mut checkpoint = input.checkpoint.ok_or_else(|| {
                CliError("rule continuation requires its native retained checkpoint".into())
            })?;
            checkpoint
                .verify_procedure(&input.procedure)
                .map_err(CliError)?;
            if checkpoint.procedure_ref != input.procedure.procedure_ref
                || checkpoint.epoch_ref != input.procedure.timing.epoch_ref
                || checkpoint.evaluations > input.procedure.budgets.max_evaluations
                || checkpoint.operations > input.procedure.budgets.max_operations
            {
                return Err(CliError(
                    "retained rule checkpoint source/scope/epoch or budget is incompatible".into(),
                ));
            }
            match operation {
                RuleAction::Enqueue { event } => checkpoint
                    .enqueue(&input.procedure, event)
                    .map_err(CliError)?,
                RuleAction::Next {
                    generated_operations,
                } => {
                    ready_event = checkpoint
                        .next(&input.procedure, generated_operations)
                        .map_err(CliError)?
                }
                RuleAction::Pause { observed_cursor } => checkpoint.pause(observed_cursor),
                RuleAction::Resume { epoch_ref } => {
                    checkpoint.resume(&epoch_ref).map_err(CliError)?
                }
                RuleAction::Seek {
                    observed_cursor,
                    epoch_ref,
                } => checkpoint
                    .seek(observed_cursor, &epoch_ref)
                    .map_err(CliError)?,
                RuleAction::Cancel { observed_cursor } => checkpoint.cancel(observed_cursor),
                RuleAction::BeginInterval {
                    interval_ref,
                    observed_cursor,
                } => checkpoint
                    .begin_interval(&interval_ref, observed_cursor)
                    .map_err(CliError)?,
                RuleAction::Start { .. } => unreachable!(),
            }
            checkpoint
        }
    };
    // Returning a checkpoint is the state owner's persistence input. It is not
    // a second state store or acknowledgement of the event's native effects.
    let result = json!({"checkpoint":checkpoint,"ready_event":ready_event});
    Ok(response("rule", result))
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    let usage = || {
        CliError("usage: ql scene procedural <discover|manifest|prepare|regenerate|atlas|rule> [request.json|-] [--json]".into())
    };
    let Some(operation) = args.first() else {
        return Err(usage());
    };
    if operation == "discover" {
        if args.len() != 1 {
            return Err(usage());
        }
        return serde_json::to_string_pretty(&discover())
            .map_err(|error| CliError(error.to_string()));
    }
    if args.len() != 2 {
        return Err(usage());
    }
    if !["manifest", "prepare", "regenerate", "atlas", "rule"].contains(&operation.as_str()) {
        return Err(usage());
    }
    let bytes = read(&args[1])?;
    let registry = native_current_m_registry();
    let output = match operation.as_str() {
        "manifest" => {
            let input: ManifestRequest = request(&bytes)?;
            schema(&input.schema)?;
            let plan = resolve_manifestation(registry, input.request).map_err(CliError)?;
            let bindings = native_procedural_bindings(&plan).map_err(CliError)?;
            response("manifest", json!({"plan":plan,"native_bindings":bindings}))
        }
        "prepare" => {
            let mut input: PrepareRequest = request(&bytes)?;
            schema(&input.schema)?;
            let membership = resolve_procedure_membership(
                registry,
                &input.procedure,
                &input.expression_ref,
                &input.current_readings,
                input.previous_membership.as_ref(),
            )
            .map_err(CliError)?;
            if input.scene_outputs.len() > input.procedure.budgets.max_active_instances {
                return Err(CliError("scene output budget exceeded".into()));
            }
            for output in input.scene_outputs {
                if output.source_basis != input.procedure.profile
                    || output.material_fingerprint
                        != fingerprint(&output.source_presentation).map_err(CliError)?
                {
                    return Err(CliError("scene template source/profile revision or consumed material fingerprint differs".into()));
                }
                input.contributions.push(
                    instantiate_scene(
                        &input.procedure,
                        &input.expression_ref,
                        &output.output_slot,
                        &output.scene_ref,
                        &output.subjects,
                        &output.source_presentation,
                    )
                    .map_err(CliError)?,
                );
            }
            let prepared = compile_native_batch(
                registry,
                &input.procedure,
                &input.operation_ref,
                &input.expression_ref,
                input.document_revision,
                membership,
                input.contributions,
                input.required_consumers,
            )
            .map_err(CliError)?;
            response(
                "prepare",
                serde_json::to_value(prepared).map_err(|error| CliError(error.to_string()))?,
            )
        }
        "regenerate" => regenerate_request(request(&bytes)?)?,
        "atlas" => {
            let input: AtlasRequest = request(&bytes)?;
            schema(&input.schema)?;
            let face = match input.face.as_str() {
                "bimba" => MFace::Bimba,
                "pratibimba" => MFace::Pratibimba,
                _ => return Err(CliError("unknown native coordinate face".into())),
            };
            let transition = prepare_atlas_transition(
                registry,
                &input.source,
                &input.destination,
                face,
                &input.expected_destination_locus,
            )
            .map_err(CliError)?;
            response(
                "atlas",
                serde_json::to_value(transition).map_err(|error| CliError(error.to_string()))?,
            )
        }
        "rule" => rule_request(request(&bytes)?)?,
        _ => return Err(usage()),
    };
    serde_json::to_string_pretty(&output).map_err(|error| CliError(error.to_string()))
}
