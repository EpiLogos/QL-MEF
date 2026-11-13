//! QL's Action field, as the O:I Action registry holds it (`oi act`, O-I
//! #1002, `oi.owner-action-field/v1`).
//!
//! `ql actions [--json]` lists the Actions; `ql action run <id> <json|->`
//! runs one. An Action is one of QL's own routes: the table below names its
//! id, title, mutation class and the inputs the route takes. Running one makes
//! the route's words from the input, checked by name, and runs them through
//! `execute_cli` in this process, so the reading or the refusal is QL's own.
//! A document input (a request the route reads from a file) is written to a
//! file of its own for the run and removed when the run ends.

use crate::{CliError, CliFailure, execute_cli};
use serde_json::{Map, Value, json};
use std::io::Read;
use std::path::PathBuf;

const OWNER_ACTION_FIELD: &str = "oi.owner-action-field/v1";

/// The most an Action's input may be, from stdin or the argument.
const MAX_INPUT_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Text,
    Number,
    /// A JSON document the route reads from a file.
    Document,
}

#[derive(Clone, Copy)]
struct Input {
    name: &'static str,
    /// `None`: the route's next positional; `Some(flag)`: that flag.
    flag: Option<&'static str>,
    kind: Kind,
    required: bool,
    description: &'static str,
}

const fn position(
    name: &'static str,
    kind: Kind,
    required: bool,
    description: &'static str,
) -> Input {
    Input {
        name,
        flag: None,
        kind,
        required,
        description,
    }
}

const fn flag(
    name: &'static str,
    flag: &'static str,
    kind: Kind,
    required: bool,
    description: &'static str,
) -> Input {
    Input {
        name,
        flag: Some(flag),
        kind,
        required,
        description,
    }
}

const fn document(name: &'static str, description: &'static str) -> Input {
    position(name, Kind::Document, true, description)
}

struct FieldAction {
    id: &'static str,
    title: &'static str,
    route: &'static [&'static str],
    inputs: &'static [Input],
    mutation_class: &'static str,
}

const fn read(
    id: &'static str,
    title: &'static str,
    route: &'static [&'static str],
    inputs: &'static [Input],
) -> FieldAction {
    FieldAction {
        id,
        title,
        route,
        inputs,
        mutation_class: "read-only",
    }
}

const fn changing(
    id: &'static str,
    title: &'static str,
    route: &'static [&'static str],
    inputs: &'static [Input],
    mutation_class: &'static str,
) -> FieldAction {
    FieldAction {
        id,
        title,
        route,
        inputs,
        mutation_class,
    }
}

const REQUEST: Input = document("request", "the request document the route reads");
const LEDGER: Input = flag(
    "ledger",
    "--ledger",
    Kind::Text,
    false,
    "a ledger file (the vendored M ledger when absent)",
);
const STAGE_CONFIG: Input = flag(
    "config",
    "--config",
    Kind::Document,
    false,
    "the binding request the stage is configured from",
);
const STAGE_WORKER: Input = flag(
    "worker",
    "--worker",
    Kind::Text,
    false,
    "the native field worker's path (QL_FIELD_WORKER when absent)",
);
const STAGE_INSTANCE: Input = flag(
    "instance",
    "--instance",
    Kind::Text,
    false,
    "the stage instance",
);
const PROCEDURE_REF: Input = position(
    "procedure_ref",
    Kind::Text,
    true,
    "the procedure's ref, from its binding",
);
const LENS: Input = flag(
    "lens",
    "--lens",
    Kind::Number,
    false,
    "the lens, 0..=11 (none when absent)",
);

const FIELD: &[FieldAction] = &[
    read(
        "ql.kernel.m1",
        "The M1 engine's reading of one request",
        &["kernel", "m1"],
        &[REQUEST],
    ),
    read(
        "ql.kernel.m3",
        "The M3 engine's reading of one request",
        &["kernel", "m3"],
        &[REQUEST],
    ),
    read(
        "ql.kernel.coverage",
        "The M ledger's coverage of a coordinate",
        &["kernel", "coverage"],
        &[
            position(
                "coordinate",
                Kind::Text,
                true,
                "M, M0..M5 or an exact coordinate",
            ),
            flag(
                "stratum",
                "--stratum",
                Kind::Text,
                false,
                "the stratum (rust when absent)",
            ),
            flag(
                "axis",
                "--axis",
                Kind::Text,
                false,
                "the axis (operational when absent)",
            ),
            flag(
                "require",
                "--require",
                Kind::Text,
                false,
                "the standing required (verified when absent)",
            ),
            LEDGER,
        ],
    ),
    read(
        "ql.kernel.ledger",
        "The M ledger's records",
        &["kernel", "ledger"],
        &[
            position(
                "coordinate",
                Kind::Text,
                false,
                "one coordinate's records (every record when absent)",
            ),
            LEDGER,
        ],
    ),
    read(
        "ql.kernel.validate-ledger",
        "Validate the M ledger",
        &["kernel", "validate-ledger"],
        &[LEDGER],
    ),
    read(
        "ql.kernel.apply",
        "Apply a deterministic kernel operator to a QL address",
        &["kernel", "apply"],
        &[
            position(
                "operator",
                Kind::Text,
                true,
                "conjugate-address, complement-address or classify-four-plus-two",
            ),
            position("address", Kind::Text, true, "the QL address"),
        ],
    ),
    read(
        "ql.kernel.body-constants",
        "M2's proposed body constants by element fibre",
        &["kernel", "body-constants"],
        &[],
    ),
    read(
        "ql.kernel.m3-binding",
        "Binding's engine over a hexagram's cell under a matrix",
        &["kernel", "m3-binding"],
        &[REQUEST],
    ),
    read(
        "ql.kernel.capabilities",
        "The kernel's forms and operators",
        &["kernel", "capabilities"],
        &[],
    ),
    read(
        "ql.matheme.derive",
        "The matheme's definitional 0-layer derivation",
        &["matheme", "derive"],
        &[],
    ),
    read(
        "ql.matheme.shadow",
        "The matheme's shadow",
        &["matheme", "shadow"],
        &[],
    ),
    read(
        "ql.mef.lenses",
        "The MEF lens registry",
        &["mef", "lenses"],
        &[],
    ),
    read(
        "ql.context-frame.list",
        "The Context-Frame registry",
        &["context-frame", "list"],
        &[],
    ),
    read(
        "ql.context-frame.read",
        "The External Context Frame reading of a sixfold mapping",
        &["context-frame", "read"],
        &[REQUEST],
    ),
    read(
        "ql.context-frame.team-space",
        "The Anima and Aletheia team space",
        &["context-frame", "team-space"],
        &[],
    ),
    read(
        "ql.context-frame.team-space.check",
        "The engine's word on cells of one decision instrument",
        &["context-frame", "team-space", "check"],
        &[
            position("instrument", Kind::Text, true, "the decision instrument"),
            document("cells", "the ql.instrument-cells/v1 cells"),
        ],
    ),
    read(
        "ql.vak.compose",
        "Compose one Vāk request",
        &["vak", "compose"],
        &[REQUEST],
    ),
    read(
        "ql.vak.unit-steps",
        "One unit's heads broken into the composition",
        &["vak", "unit-steps"],
        &[document("unit", "the unit document")],
    ),
    read(
        "ql.vak.thought-consumption",
        "The reading of one consumed T/T′ thought from Factory's receipt",
        &["vak", "thought-consumption"],
        &[REQUEST],
    ),
    read(
        "ql.vak.locate",
        "Locate one Vāk entry",
        &["vak", "locate"],
        &[position("vak_ref", Kind::Text, true, "the Vāk ref")],
    ),
    read(
        "ql.vak.context",
        "A Vāk entry's source-locked neighbourhood",
        &["vak", "context"],
        &[
            position("vak_ref", Kind::Text, true, "the Vāk ref"),
            position(
                "depth",
                Kind::Number,
                false,
                "the neighbourhood's depth (1 when absent)",
            ),
        ],
    ),
    read(
        "ql.vak.workflow-types",
        "The Vāk workflow types module",
        &["vak", "workflow-types"],
        &[flag(
            "check",
            "--check",
            Kind::Text,
            false,
            "a checked-in module to hold to the contract",
        )],
    ),
    read(
        "ql.vak.capabilities",
        "The Vāk registry's source",
        &["vak", "capabilities"],
        &[],
    ),
    read(
        "ql.techne.reading",
        "The Technē reading of one target",
        &["techne", "reading"],
        &[document("target", "the target document")],
    ),
    read(
        "ql.shape.presentation",
        "The presentation of one exact shape",
        &["shape", "presentation"],
        &[position(
            "shape_ref",
            Kind::Text,
            true,
            "the exact shape's ref",
        )],
    ),
    read(
        "ql.shape.bind",
        "The shape binding of a drafted constellation",
        &["shape", "bind"],
        &[REQUEST],
    ),
    read(
        "ql.wiki.living-plan",
        "The Living Wiki refraction plan",
        &["wiki", "living-plan"],
        &[REQUEST],
    ),
    read(
        "ql.wiki.refract",
        "One Wiki refraction request",
        &["wiki", "refract"],
        &[REQUEST],
    ),
    read(
        "ql.scene.compose",
        "Compose the dated sky's scene",
        &["scene", "compose"],
        &[REQUEST],
    ),
    read(
        "ql.scene.binding",
        "The continuous-field host binding the Live instrument opens",
        &["scene", "binding"],
        &[REQUEST],
    ),
    read(
        "ql.scene.world",
        "The dated sky placed on the clock as a world reading",
        &["scene", "world"],
        &[REQUEST],
    ),
    changing(
        "ql.scene.render",
        "Render the played scene offline to a WAV file",
        &["scene", "render"],
        &[
            document("request", "the scene binding request"),
            flag(
                "worker",
                "--worker",
                Kind::Text,
                true,
                "the native field worker's path",
            ),
            flag(
                "seconds",
                "--seconds",
                Kind::Number,
                true,
                "the take's length in seconds",
            ),
            flag("out", "--out", Kind::Text, true, "the WAV file to write"),
        ],
        "locally-mutating",
    ),
    read(
        "ql.stage.state",
        "The Ta-Onta stage's live slots, owners and bindings",
        &["stage", "state"],
        &[STAGE_CONFIG, STAGE_WORKER, STAGE_INSTANCE],
    ),
    changing(
        "ql.stage.evaluate",
        "Evaluate one versioned procedure on the stage",
        &["stage", "evaluate"],
        &[
            document("procedure", "the procedure document"),
            STAGE_CONFIG,
            STAGE_WORKER,
            STAGE_INSTANCE,
        ],
        "externally-mutating",
    ),
    changing(
        "ql.stage.bind",
        "Bind a determinant-triggered procedure to the live flow",
        &["stage", "bind"],
        &[
            document("procedure", "the procedure document"),
            flag(
                "max_evaluations",
                "--max-evaluations",
                Kind::Number,
                false,
                "the evaluation budget",
            ),
            STAGE_CONFIG,
            STAGE_WORKER,
            STAGE_INSTANCE,
        ],
        "externally-mutating",
    ),
    changing(
        "ql.stage.unbind",
        "Remove a procedure's binding",
        &["stage", "unbind"],
        &[PROCEDURE_REF, STAGE_CONFIG, STAGE_WORKER, STAGE_INSTANCE],
        "externally-mutating",
    ),
    changing(
        "ql.stage.retire",
        "Release one procedure's contributions",
        &["stage", "retire"],
        &[PROCEDURE_REF, STAGE_CONFIG, STAGE_WORKER, STAGE_INSTANCE],
        "externally-mutating",
    ),
    read(
        "ql.music.janko.window",
        "The Janko surface over one column period",
        &["music", "janko", "window"],
        &[
            flag(
                "columns",
                "--columns",
                Kind::Number,
                false,
                "columns, 1..=24 (one period when absent)",
            ),
            flag(
                "first_column",
                "--first-column",
                Kind::Number,
                false,
                "the first column (0 when absent)",
            ),
            LENS,
        ],
    ),
    read(
        "ql.music.janko.key",
        "One Janko key's sounding class, coordinate and fifths overlay",
        &["music", "janko", "key"],
        &[
            flag("row", "--row", Kind::Number, true, "the row, 0..=5"),
            flag("column", "--column", Kind::Number, true, "the column"),
            LENS,
        ],
    ),
    read(
        "ql.music.janko.touch-points",
        "The three repeated touch-points of one sounding note",
        &["music", "janko", "touch-points"],
        &[flag(
            "pitch",
            "--pitch",
            Kind::Number,
            true,
            "the pitch class, 0..=11",
        )],
    ),
    read(
        "ql.music.modes",
        "The kernel's mode catalogue",
        &["music", "modes"],
        &[],
    ),
    read(
        "ql.nara.capabilities",
        "Nara's identity field: its operations and schemas",
        &["nara", "capabilities"],
        &[],
    ),
    read(
        "ql.nara.inspect",
        "Nara inspect",
        &["nara", "inspect"],
        &[REQUEST],
    ),
    read(
        "ql.nara.calculate",
        "Nara calculate",
        &["nara", "calculate"],
        &[REQUEST],
    ),
    read(
        "ql.nara.transit",
        "Nara transit",
        &["nara", "transit"],
        &[REQUEST],
    ),
    read(
        "ql.nara.hours",
        "Nara hours",
        &["nara", "hours"],
        &[REQUEST],
    ),
    read(
        "ql.nara.personal-current",
        "Nara personal-current",
        &["nara", "personal-current"],
        &[REQUEST],
    ),
    read(
        "ql.nara.personal-recompose",
        "Nara personal-recompose",
        &["nara", "personal-recompose"],
        &[REQUEST],
    ),
    read(
        "ql.nara.presence-consent",
        "Nara presence-consent",
        &["nara", "presence-consent"],
        &[REQUEST],
    ),
    read(
        "ql.nara.coordinate",
        "Nara coordinate",
        &["nara", "coordinate"],
        &[REQUEST],
    ),
    read(
        "ql.nara.coordinate-content",
        "Nara coordinate-content",
        &["nara", "coordinate-content"],
        &[REQUEST],
    ),
    read(
        "ql.nara.coordinate-bundle",
        "Nara coordinate-bundle",
        &["nara", "coordinate-bundle"],
        &[REQUEST],
    ),
    read(
        "ql.nara.source-inventory",
        "Nara source-inventory",
        &["nara", "source-inventory"],
        &[REQUEST],
    ),
    read(
        "ql.nara.context",
        "Nara dialogue context",
        &["nara", "context"],
        &[REQUEST],
    ),
    read(
        "ql.nara.delegate",
        "Nara dialogue delegate",
        &["nara", "delegate"],
        &[REQUEST],
    ),
    read(
        "ql.nara.enrichment",
        "Nara dialogue enrichment",
        &["nara", "enrichment"],
        &[REQUEST],
    ),
    read(
        "ql.nara.receive",
        "Nara dialogue receive",
        &["nara", "receive"],
        &[REQUEST],
    ),
    read(
        "ql.epi-agent.constitution",
        "The Prime-QL body's constitution",
        &["epi-agent", "constitution"],
        &[],
    ),
    read(
        "ql.epi-agent.faculty",
        "One faculty of the Prime-QL body",
        &["epi-agent", "faculty"],
        &[position("faculty", Kind::Text, true, "the faculty, #0..#5")],
    ),
    read(
        "ql.epi-agent.invoke",
        "The Prime-QL body's reading of one invocation request",
        &["epi-agent", "invoke"],
        &[REQUEST],
    ),
    read(
        "ql.agent-event.project",
        "Project supplied agent events",
        &["agent-event", "project"],
        &[REQUEST],
    ),
    read(
        "ql.agent-event.frame",
        "Frame supplied agent events",
        &["agent-event", "frame"],
        &[REQUEST],
    ),
    read(
        "ql.agent-event.harmonic",
        "The harmonic reading of supplied agent events",
        &["agent-event", "harmonic"],
        &[REQUEST],
    ),
    read(
        "ql.agent-event.validate",
        "Validate supplied agent events",
        &["agent-event", "validate"],
        &[REQUEST],
    ),
    read(
        "ql.service.capabilities",
        "The QL service's provider state and operations",
        &["service", "capabilities"],
        &[],
    ),
    read(
        "ql.service.negotiate",
        "One service operation's negotiated availability",
        &["service", "negotiate"],
        &[position(
            "operation",
            Kind::Text,
            true,
            "capabilities, locate, refract, relate or synthesise",
        )],
    ),
    read(
        "ql.verify",
        "Verify the deterministic kernel and the registries",
        &["verify"],
        &[],
    ),
    read(
        "ql.capabilities",
        "QL's CLI contract and capabilities",
        &["capabilities"],
        &[],
    ),
    read("ql.system", "QL's system disclosure", &["system"], &[]),
    read(
        "ql.config-contribution",
        "The settings QL contributes (disclosure only)",
        &["config-contribution"],
        &[],
    ),
];

fn kind_name(kind: Kind) -> &'static str {
    match kind {
        Kind::Text => "string",
        Kind::Number => "number",
        Kind::Document => "object",
    }
}

/// The route as `ql help` writes it: positionals, then flags.
fn usage(action: &FieldAction) -> String {
    let mut words = vec!["ql".to_owned()];
    words.extend(action.route.iter().map(|word| (*word).to_owned()));
    let mut flags = Vec::new();
    for input in action.inputs {
        match (input.flag, input.required) {
            (None, true) => words.push(format!("<{}>", input.name)),
            (None, false) => words.push(format!("[{}]", input.name)),
            (Some(flag), true) => flags.push(format!("{flag} <{}>", input.name)),
            (Some(flag), false) => flags.push(format!("[{flag} <{}>]", input.name)),
        }
    }
    words.extend(flags);
    words.join(" ")
}

fn descriptor(action: &FieldAction) -> Value {
    let mut positional = 0;
    let inputs = action
        .inputs
        .iter()
        .map(|input| {
            let mut row = json!({
                "name": input.name,
                "type": kind_name(input.kind),
                "required": input.required,
                "description": input.description,
            });
            match input.flag {
                Some(flag) => row["flag"] = json!(flag),
                None => {
                    row["position"] = json!(positional);
                    positional += 1;
                }
            }
            if input.kind == Kind::Document {
                row["document"] = json!(true);
            }
            row
        })
        .collect::<Vec<_>>();
    json!({
        "id": action.id,
        "title": action.title,
        "description": usage(action),
        "mutation_class": action.mutation_class,
        "inputs": inputs,
        "output": {"type": "ql-json"},
        "route": action.route,
        "availability": {"available": true, "reason": null},
    })
}

fn field() -> Value {
    json!({
        "schema": OWNER_ACTION_FIELD,
        "owner": "ql",
        "actions": FIELD.iter().map(descriptor).collect::<Vec<_>>(),
    })
}

/// `ql actions [--json]`.
pub(crate) fn actions(json: bool) -> Result<String, CliError> {
    if json {
        serde_json::to_string_pretty(&field()).map_err(CliError::from)
    } else {
        Ok(FIELD
            .iter()
            .map(|action| {
                format!(
                    "  {:<36} {:<20} {}",
                    action.id, action.mutation_class, action.title
                )
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

/// The documents written for one run, removed when it ends.
#[derive(Default)]
struct Carried(Vec<PathBuf>);

impl Carried {
    fn carry(&mut self, id: &str, name: &str, document: &Value) -> Result<String, String> {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|elapsed| elapsed.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!(
            "ql-action-{}-{stamp}-{}.json",
            std::process::id(),
            self.0.len()
        ));
        let refused = |error: std::io::Error| {
            format!("{id}'s input `{name}` could not reach the route: {error}")
        };
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(refused)?;
        self.0.push(path.clone());
        std::io::Write::write_all(&mut file, document.to_string().as_bytes()).map_err(refused)?;
        Ok(path.to_string_lossy().into_owned())
    }
}

impl Drop for Carried {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = std::fs::remove_file(path);
        }
    }
}

/// The route's words an Action's input makes: the route, its positionals in
/// order, its flags, then `--json`. An input the route does not take, a
/// required one missing, or a value of the wrong shape is refused by name
/// before anything runs.
fn route_words(id: &str, input: &Value, carried: &mut Carried) -> Result<Vec<String>, String> {
    let action = FIELD
        .iter()
        .find(|action| action.id == id)
        .ok_or_else(|| format!("`{id}` is not in QL's Action field; `ql actions` lists it"))?;
    let given = input
        .as_object()
        .ok_or_else(|| format!("{id} takes one JSON object of named inputs"))?;
    if let Some(name) = given.keys().find(|name| {
        !action
            .inputs
            .iter()
            .any(|input| input.name == name.as_str())
    }) {
        return Err(format!("{id} takes no input `{name}`"));
    }
    for input in action.inputs {
        let value = given.get(input.name).filter(|value| !value.is_null());
        let shaped = match (input.kind, value) {
            (_, None) => !input.required,
            (Kind::Text, Some(value)) => value.is_string(),
            (Kind::Number, Some(Value::String(value))) => value.parse::<f64>().is_ok(),
            (Kind::Number, Some(value)) => value.is_number(),
            (Kind::Document, Some(value)) => value.is_object() || value.is_array(),
        };
        if !shaped {
            let name = input.name;
            return Err(match (value, input.kind) {
                (None, _) => format!("{id} needs its input `{name}`"),
                (_, Kind::Text) => format!("{id}'s input `{name}` is a string"),
                (_, Kind::Number) => format!("{id}'s input `{name}` is a number"),
                (_, Kind::Document) => {
                    format!("{id}'s input `{name}` is a JSON document (an object or an array)")
                }
            });
        }
    }
    let mut words = action
        .route
        .iter()
        .map(|word| (*word).to_owned())
        .collect::<Vec<_>>();
    let mut flags = Vec::new();
    let mut absent_positional = None;
    for input in action.inputs {
        let Some(value) = given.get(input.name).filter(|value| !value.is_null()) else {
            if input.flag.is_none() {
                absent_positional.get_or_insert(input.name);
            }
            continue;
        };
        let word = match value {
            Value::String(value) => value.clone(),
            Value::Number(value) => value.to_string(),
            document => carried.carry(id, input.name, document)?,
        };
        match input.flag {
            Some(flag) => flags.extend([flag.to_owned(), word]),
            None => {
                if let Some(earlier) = absent_positional {
                    return Err(format!("{id} needs `{earlier}` before `{}`", input.name));
                }
                words.push(word);
            }
        }
    }
    words.extend(flags);
    words.push("--json".into());
    Ok(words)
}

/// `ql action run <id> <json|->`: the Action's route, run through
/// `execute_cli` in this process.
pub(crate) fn action_command(args: &[String]) -> Result<String, CliFailure> {
    let refused = |words: String| CliFailure::Message(CliError(words));
    let (Some("run"), Some(id), None) =
        (args.first().map(String::as_str), args.get(1), args.get(3))
    else {
        return Err(refused("usage: ql action run <id> <json|->".into()));
    };
    let raw = match args.get(2).map(String::as_str) {
        None => String::new(),
        Some("-") => {
            let mut text = String::new();
            std::io::stdin()
                .take(MAX_INPUT_BYTES + 1)
                .read_to_string(&mut text)
                .map_err(|error| refused(format!("{id}'s input could not be read: {error}")))?;
            text
        }
        Some(raw) => raw.to_owned(),
    };
    if raw.len() as u64 > MAX_INPUT_BYTES {
        return Err(refused(format!("{id}'s input exceeds 8 MiB")));
    }
    let input = if raw.trim().is_empty() {
        Value::Object(Map::new())
    } else {
        serde_json::from_str::<Value>(&raw)
            .map_err(|error| refused(format!("{id} takes one JSON object: {error}")))?
    };
    let mut carried = Carried::default();
    let words = route_words(id, &input, &mut carried).map_err(refused)?;
    execute_cli(&words)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(words: &[&str]) -> Result<String, String> {
        execute_cli(
            &words
                .iter()
                .map(|word| (*word).to_owned())
                .collect::<Vec<_>>(),
        )
        .map_err(|failure| match failure {
            CliFailure::Message(error) => error.to_string(),
            CliFailure::ConfigDocument(document) => document,
        })
    }

    #[test]
    fn the_field_lists_qls_actions_with_their_inputs() {
        let field = field();
        assert_eq!(field["schema"], OWNER_ACTION_FIELD);
        assert_eq!(field["owner"], "ql");
        let actions = field["actions"].as_array().unwrap();
        let mut ids = std::collections::BTreeSet::new();
        for action in actions {
            let id = action["id"].as_str().unwrap();
            assert!(id.starts_with("ql."), "{id}");
            assert!(ids.insert(id), "{id} is listed once");
            assert!(
                ["read-only", "locally-mutating", "externally-mutating"]
                    .contains(&action["mutation_class"].as_str().unwrap()),
                "{id}"
            );
        }
        let coverage = actions
            .iter()
            .find(|action| action["id"] == "ql.kernel.coverage")
            .unwrap();
        assert_eq!(
            coverage["description"],
            "ql kernel coverage <coordinate> [--stratum <stratum>] [--axis <axis>] [--require <require>] [--ledger <ledger>]"
        );
        assert_eq!(
            coverage["inputs"][0],
            json!({"name": "coordinate", "type": "string", "required": true,
                   "description": "M, M0..M5 or an exact coordinate", "position": 0})
        );
        assert_eq!(coverage["inputs"][4]["flag"], "--ledger");
        let m3 = actions
            .iter()
            .find(|action| action["id"] == "ql.kernel.m3")
            .unwrap();
        assert_eq!(m3["inputs"][0]["type"], "object");
        assert_eq!(m3["inputs"][0]["document"], true);
        // The listing is the CLI's own.
        assert_eq!(
            serde_json::from_str::<Value>(&run(&["actions", "--json"]).unwrap()).unwrap(),
            field
        );
    }

    #[test]
    fn an_input_makes_the_routes_words_and_is_refused_by_name() {
        let mut carried = Carried::default();
        assert_eq!(
            route_words(
                "ql.kernel.coverage",
                &json!({"coordinate": "M2", "require": "verified"}),
                &mut carried
            )
            .unwrap(),
            [
                "kernel",
                "coverage",
                "M2",
                "--require",
                "verified",
                "--json"
            ]
        );
        assert_eq!(
            route_words(
                "ql.vak.context",
                &json!({"vak_ref": "vak:x", "depth": 2}),
                &mut carried
            )
            .unwrap(),
            ["vak", "context", "vak:x", "2", "--json"]
        );
        assert_eq!(
            route_words(
                "ql.music.janko.key",
                &json!({"row": 1, "column": "3"}),
                &mut carried
            )
            .unwrap(),
            [
                "music", "janko", "key", "--row", "1", "--column", "3", "--json"
            ]
        );
        for (id, input, words) in [
            (
                "ql.kernel.coverage",
                json!({"coordinate": "M2", "colour": "red"}),
                "takes no input `colour`",
            ),
            (
                "ql.kernel.coverage",
                json!({}),
                "needs its input `coordinate`",
            ),
            (
                "ql.vak.context",
                json!({"vak_ref": "v", "depth": "deep"}),
                "`depth` is a number",
            ),
            (
                "ql.vak.context",
                json!({"depth": 2}),
                "needs its input `vak_ref`",
            ),
            (
                "ql.kernel.apply",
                json!({"operator": 1, "address": "a"}),
                "`operator` is a string",
            ),
            (
                "ql.kernel.m3",
                json!({"request": "request.json"}),
                "`request` is a JSON document",
            ),
            ("ql.mef.lenses", json!([]), "one JSON object"),
            ("ql.teleport", json!({}), "is not in QL's Action field"),
        ] {
            let refused = route_words(id, &input, &mut carried).unwrap_err();
            assert!(refused.contains(words), "{id}: {refused}");
        }
        assert!(carried.0.is_empty(), "no refused input carried a document");
    }

    #[test]
    fn a_document_reaches_the_route_in_a_file_that_goes_when_the_run_ends() {
        let mut carried = Carried::default();
        let words =
            route_words("ql.kernel.m3", &json!({"request": {"a": 1}}), &mut carried).unwrap();
        let path = PathBuf::from(&words[2]);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), r#"{"a":1}"#);
        drop(carried);
        assert!(!path.exists(), "the carried document is removed");
    }

    #[test]
    fn an_action_answers_as_its_route_does() {
        assert_eq!(
            run(&["action", "run", "ql.mef.lenses", "{}"]).unwrap(),
            run(&["mef", "lenses", "--json"]).unwrap()
        );
        let applied = run(&[
            "action",
            "run",
            "ql.kernel.apply",
            r#"{"operator": "conjugate-address", "address": "qladdr:sixfold@1/direct/P5/d0"}"#,
        ])
        .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&applied).unwrap()["output"],
            "qladdr:sixfold@1/conjugate/P5/d0"
        );
        // A document route reads the request the input carries.
        let request_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/kernel/m1-engine-v1.request.json"
        );
        let request: Value =
            serde_json::from_str(&std::fs::read_to_string(request_path).unwrap()).unwrap();
        assert_eq!(
            run(&[
                "action",
                "run",
                "ql.kernel.m1",
                &json!({"request": request}).to_string()
            ])
            .unwrap(),
            run(&["kernel", "m1", request_path, "--json"]).unwrap()
        );
        // A refusal is the field's, in QL's words, before anything runs.
        let refused = run(&["action", "run", "ql.kernel.m1", r#"{"colour": "red"}"#]).unwrap_err();
        assert!(refused.contains("takes no input `colour`"), "{refused}");
        let refused = run(&["action", "walk", "ql.kernel.m1"]).unwrap_err();
        assert!(refused.starts_with("usage: ql action run"), "{refused}");
    }

    #[test]
    fn every_route_in_the_field_is_one_ql_dispatches() {
        // Each route, given placeholder inputs (a document is a file that is
        // not there, so nothing reads or writes past it), is not refused the
        // way a route QL does not have is refused.
        let absent = std::env::temp_dir().join(format!(
            "ql-action-field-absent-{}.json",
            std::process::id()
        ));
        let absent = absent.to_string_lossy().into_owned();
        for action in FIELD {
            let mut words = action
                .route
                .iter()
                .map(|word| (*word).to_owned())
                .collect::<Vec<_>>();
            let mut flags = Vec::new();
            for input in action.inputs {
                if !input.required && input.name != "worker" {
                    continue;
                }
                let placeholder = match input.kind {
                    Kind::Document => absent.clone(),
                    Kind::Number => "0".to_owned(),
                    Kind::Text if input.name == "worker" || input.name == "out" => absent.clone(),
                    Kind::Text => "no-such-thing".to_owned(),
                };
                match input.flag {
                    Some(flag) => flags.extend([flag.to_owned(), placeholder]),
                    None => words.push(placeholder),
                }
            }
            words.extend(flags);
            let mut sibling = action.route[..action.route.len() - 1]
                .iter()
                .map(|word| (*word).to_owned())
                .collect::<Vec<_>>();
            sibling.push("no-such-route".into());
            let unknown = execute_cli(&sibling).err().map(|failure| match failure {
                CliFailure::Message(error) => error.to_string(),
                CliFailure::ConfigDocument(document) => document,
            });
            let answered = execute_cli(&words).err().map(|failure| match failure {
                CliFailure::Message(error) => error.to_string(),
                CliFailure::ConfigDocument(document) => document,
            });
            if let (Some(unknown), Some(answered)) = (unknown, answered) {
                let last = action.route[action.route.len() - 1];
                assert_ne!(
                    answered,
                    unknown.replace("no-such-route", last),
                    "{} names a route QL does not have",
                    action.id
                );
            }
        }
        assert!(!std::path::Path::new(&absent).exists());
    }
}
