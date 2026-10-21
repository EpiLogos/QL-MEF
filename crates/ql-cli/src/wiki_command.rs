//! `ql wiki living-plan <request.json|-> [--json]`: the Living Wiki refraction
//! plan (`ql_wiki::plan_living_wiki_refraction`, profile
//! `ql-mef/living-wiki-refraction/v1`) for a mode, an explicit relevance and an
//! optional existing Wiki refraction request. The library admits the entry
//! aperture 5 -> 0 -> 1 first, then the positions asked for in QL order, up to
//! the budget, and reports what the budget left out. The plan is orientation
//! only: the command reads no wiki, infers no relevance from source text, keeps
//! no store and owns no freshness or Agent orchestration.
use crate::CliError;
use ql_wiki::{
    LivingWikiMode, LivingWikiRefractionPlan, LivingWikiRelevance, WikiRefractionRequest,
    plan_living_wiki_refraction,
};
use serde::Deserialize;
use serde_json::Value;
use std::io::Read;

const MAX_INPUT: u64 = 1024 * 1024;
const USAGE: &str = "usage: ql wiki living-plan <request.json|-> [--json]";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    mode: LivingWikiMode,
    /// The relevance the caller states. Absent, the library's default stands:
    /// no positions beyond the entry, a budget of six.
    #[serde(default)]
    relevance: Option<LivingWikiRelevance>,
    /// An existing Wiki refraction request, carried by Explain when enabled and
    /// required by Formal.
    #[serde(default)]
    refraction: Option<WikiRefractionRequest>,
}

pub(crate) fn command(args: &[String], json: bool) -> Result<String, CliError> {
    let [operation, path] = args else {
        return Err(CliError(USAGE.into()));
    };
    if operation != "living-plan" {
        return Err(CliError(USAGE.into()));
    }
    let reader: Box<dyn Read> = if path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path).map_err(|e| CliError(e.to_string()))?)
    };
    let mut input = String::new();
    reader
        .take(MAX_INPUT + 1)
        .read_to_string(&mut input)
        .map_err(|e| CliError(e.to_string()))?;
    if input.len() as u64 > MAX_INPUT {
        return Err(CliError("Living Wiki plan request exceeds 1 MiB".into()));
    }
    let plan = plan(&input)?;
    if json {
        serde_json::to_string_pretty(&plan).map_err(CliError::from)
    } else {
        Ok(render_plain(
            &serde_json::to_value(&plan).map_err(CliError::from)?,
        ))
    }
}

/// The library's plan for one request; its refusals in its own words.
pub(crate) fn plan(input: &str) -> Result<LivingWikiRefractionPlan, CliError> {
    let request: Request = serde_json::from_str(input)
        .map_err(|e| CliError(format!("invalid Living Wiki plan request: {e}")))?;
    plan_living_wiki_refraction(
        request.mode,
        request.relevance.unwrap_or_default(),
        request.refraction,
    )
    .map_err(|e| CliError(e.to_string()))
}

fn render_plain(plan: &Value) -> String {
    let positions = |value: &Value| {
        let list: Vec<String> = value
            .as_array()
            .into_iter()
            .flatten()
            .map(Value::to_string)
            .collect();
        if list.is_empty() {
            "none".to_string()
        } else {
            list.join(" ")
        }
    };
    format!(
        "{} plan, depth {}: admitted {}; omitted {}\nentry aperture {} (orientation, not the Return); refraction request {}",
        plan["mode"].as_str().unwrap_or("?"),
        plan["depth"].as_str().unwrap_or("?"),
        positions(&plan["positions"]),
        positions(&plan["omitted_relevant_positions"]),
        positions(&plan["entry"]["positions"]),
        if plan["refraction"].is_null() {
            "none"
        } else {
            "carried"
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cli_plan_is_the_librarys_plan() {
        let relevance = LivingWikiRelevance {
            relevant_positions: vec![2, 3],
            requires_context: true,
            full_context_frame: false,
            position_budget: 4,
        };
        let expected =
            plan_living_wiki_refraction(LivingWikiMode::Explain, relevance, None).unwrap();
        let got = plan(
            r#"{"mode":"explain","relevance":{"relevant_positions":[2,3],"requires_context":true,"position_budget":4}}"#,
        )
        .unwrap();
        assert_eq!(got, expected);
        assert_eq!(got.positions, vec![5, 0, 1, 2]);
        assert_eq!(got.omitted_relevant_positions, vec![3, 4]);
    }

    #[test]
    fn absent_relevance_is_the_librarys_default() {
        let got = plan(r#"{"mode":"explain"}"#).unwrap();
        let expected = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance::default(),
            None,
        )
        .unwrap();
        assert_eq!(got, expected);
        assert_eq!(got.positions, vec![5, 0, 1]);
    }

    #[test]
    fn refusals_are_the_librarys_words() {
        let formal = plan(r#"{"mode":"formal"}"#).unwrap_err().0;
        assert_eq!(
            formal,
            "formal Living Wiki depth requires an existing WikiRefractionRequest"
        );
        let budget = plan(r#"{"mode":"explain","relevance":{"position_budget":2}}"#)
            .unwrap_err()
            .0;
        assert!(budget.contains("cannot preserve the 5->0->1 entry aperture"));
        assert!(
            plan(r#"{"mode":"explain","surplus":1}"#)
                .unwrap_err()
                .0
                .starts_with("invalid Living Wiki plan request")
        );
    }
}
