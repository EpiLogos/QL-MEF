//! Public transport for the existing transactional M3 producer.
//! Selections and owner references remain explicit; this pure replay neither
//! authenticates a host nor admits its commands as personal activity.
use crate::CliError;
use ql_mef::m3_state::{M3Command, M3Request, M3State};
use serde::Deserialize;
use serde_json::json;
use std::io::Read;

const MAX_INPUT: u64 = 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    request: M3Request,
    commands: Vec<M3Command>,
    activity_policy: Option<ql_mef::nara::m3_activity::Policy>,
    activity_start_generation: Option<u64>,
    /// Read-only asks for the generation cell at a hexagram that is not the
    /// form's (the Deck's cast hexagram). `family` omitted reads under the
    /// matrix the state chose. Nothing here moves the state.
    generation_at: Option<Vec<GenerationAt>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GenerationAt {
    hexagram: u8,
    family: Option<u8>,
}

pub(crate) fn command(args: &[String]) -> Result<String, CliError> {
    if args.len() != 1 {
        return Err(CliError(
            "usage: ql kernel m3 <request.json|-> --json".into(),
        ));
    }
    let reader: Box<dyn Read> = if args[0] == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(&args[0]).map_err(|e| CliError(e.to_string()))?)
    };
    let mut input = String::new();
    reader
        .take(MAX_INPUT + 1)
        .read_to_string(&mut input)
        .map_err(|e| CliError(e.to_string()))?;
    if input.len() as u64 > MAX_INPUT {
        return Err(CliError("M3 request exceeds 1 MiB".into()));
    }
    replay(&input)
}

pub(crate) fn replay(input: &str) -> Result<String, CliError> {
    let input: Input = serde_json::from_str(input).map_err(CliError::from)?;
    if input.commands.len() > 256 {
        return Err(CliError("M3 replay exceeds 256 commands".into()));
    }
    let mut state = M3State::new(input.request.clone()).map_err(CliError)?;
    let mut receipts = Vec::with_capacity(input.commands.len());
    for command in input.commands {
        receipts.push(state.apply(command).map_err(CliError)?);
    }
    let final_generation = state.snapshot()["identity"]["profile_generation"]
        .as_u64()
        .ok_or_else(|| CliError("Native M3 generation unavailable".into()))?;
    let start = input.activity_start_generation.unwrap_or(0);
    if start > final_generation
        || (input.activity_policy.is_none() && input.activity_start_generation.is_some())
    {
        return Err(CliError(
            "Activity policy boundary must name an existing native generation and selected policy"
                .into(),
        ));
    }
    let mut result = json!({"state":state.snapshot(),"receipts":receipts});
    if let Some(asks) = input.generation_at {
        if asks.len() > 64 {
            return Err(CliError("M3 generation_at exceeds 64 asks".into()));
        }
        let readings = asks
            .iter()
            .map(|ask| state.generation_at(ask.family, ask.hexagram))
            .collect::<Result<Vec<_>, _>>()
            .map_err(CliError)?;
        result["generation_at"] = json!(readings);
    }
    if let Some(policy) = input.activity_policy {
        result["activity"] =
            ql_mef::nara::m3_activity::receive(&input.request, &receipts, policy, start)
                .map_err(CliError)?;
    }
    serde_json::to_string(&result).map_err(CliError::from)
}

#[cfg(test)]
mod tests {
    use super::replay;
    use ql_mef::m3_state::M3State;
    use serde_json::{Value, json};

    fn fixture() -> Value {
        serde_json::from_str(include_str!(
            "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
        ))
        .unwrap()
    }

    fn ask(mut extra: Value) -> Result<Value, String> {
        let mut input = json!({"request":fixture()["request"].clone(),"commands":[]});
        for (key, value) in extra.as_object_mut().unwrap().iter_mut() {
            input[key] = value.take();
        }
        replay(&input.to_string())
            .map(|out| serde_json::from_str(&out).unwrap())
            .map_err(|e| e.0)
    }

    #[test]
    fn generation_at_reads_other_hexagrams_beside_the_state_without_moving_it() {
        let plain = ask(json!({})).unwrap();
        assert!(plain.get("generation_at").is_none());
        let owner =
            M3State::new(serde_json::from_value(fixture()["request"].clone()).unwrap()).unwrap();
        let asks: Vec<Value> = (0..64u8)
            .flat_map(|h| [json!({"hexagram":h}), json!({"hexagram":h,"family":2})])
            .take(64)
            .collect();
        let got = ask(json!({"generation_at":asks.clone()})).unwrap();
        assert_eq!(got["state"], plain["state"]);
        assert_eq!(got["receipts"], plain["receipts"]);
        let readings = got["generation_at"].as_array().unwrap();
        assert_eq!(readings.len(), 64);
        for (a, reading) in asks.iter().zip(readings) {
            let family = a.get("family").map(|f| f.as_u64().unwrap() as u8);
            let hexagram = a["hexagram"].as_u64().unwrap() as u8;
            assert_eq!(*reading, owner.generation_at(family, hexagram).unwrap());
            assert_eq!(reading["hexagram_id"], hexagram);
        }
        // Under the state's own matrix when no family is asked (request axis 0).
        assert_eq!(readings[0]["matrix"]["family"], "complementary");
        assert_eq!(readings[1]["matrix"]["family"], "same-quality");
    }

    #[test]
    fn generation_at_refuses_what_is_not_a_hexagram_or_a_matrix() {
        for bad in [
            json!([{"hexagram":64}]),
            json!([{"hexagram":0,"family":3}]),
            json!([{"hexagram":0,"surplus":1}]),
            json!([{"family":0}]),
            json!((0..65).map(|_| json!({"hexagram":0})).collect::<Vec<_>>()),
        ] {
            assert!(ask(json!({"generation_at":bad.clone()})).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_set_matrix_command_chooses_the_cell_without_turning_the_form() {
        let f = fixture();
        let mut command = f["commands"][0].clone();
        command["operations"] = json!([{"operation":"set-matrix","family":1}]);
        let plain = ask(json!({})).unwrap();
        let got = ask(json!({"commands":[command]})).unwrap();
        assert_eq!(
            got["state"]["form"]["address"],
            plain["state"]["form"]["address"]
        );
        assert_eq!(
            plain["state"]["generation"]["matrix"]["family"],
            "complementary"
        );
        assert_eq!(
            got["state"]["generation"]["matrix"]["family"],
            "moving-resting"
        );
    }
}
