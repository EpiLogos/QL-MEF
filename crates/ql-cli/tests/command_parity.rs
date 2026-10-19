//! `ql capabilities` must list every command the dispatch serves, and serve
//! every command it lists (derived-surfaces law: parity, not presence).
//!
//! The served set is read from the CLI's own `ql help` routes, which the
//! front-door test pins; the listed set from `ql capabilities --json`. Each
//! listed command is then run with an unreadable argument: a served command
//! answers with its own usage or input error, never an "unknown" refusal.
use serde_json::Value;
use std::collections::BTreeSet;
use std::process::{Command, Stdio};

fn ql(args: &[&str]) -> (bool, String, String) {
    let run = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(args)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    (
        run.status.success(),
        String::from_utf8_lossy(&run.stdout).into_owned(),
        String::from_utf8_lossy(&run.stderr).into_owned(),
    )
}

fn listed() -> Vec<String> {
    let (ok, out, _) = ql(&["capabilities", "--json"]);
    assert!(ok);
    let view: Value = serde_json::from_str(&out).unwrap();
    view["commands"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c.as_str().unwrap().to_owned())
        .collect()
}

fn plain(word: &str) -> bool {
    !word.is_empty()
        && !word.starts_with('-')
        && word
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Every route the help text documents, as the dotted identity capabilities uses.
fn documented() -> BTreeSet<String> {
    let (ok, help, _) = ql(&["help"]);
    assert!(ok);
    let mut routes = BTreeSet::new();
    for line in help.lines() {
        let Some(rest) = line.trim_start().strip_prefix("ql ") else {
            continue;
        };
        let tokens: Vec<&str> = rest.split_whitespace().collect();
        let Some(group) = tokens.first().filter(|g| plain(g)) else {
            continue;
        };
        let mut path = vec![(*group).to_owned()];
        match tokens.get(1) {
            Some(t) if plain(t) => {
                path.push((*t).to_owned());
                if group == &"music" && *t == "janko" && tokens.get(2).is_some_and(|t| plain(t)) {
                    path.push(tokens[2].to_owned());
                }
                routes.insert(path.join("."));
            }
            Some(t) if t.starts_with('<') && t.ends_with('>') => {
                let alternatives: Vec<&str> = t[1..t.len() - 1].split('|').collect();
                if alternatives.iter().all(|a| plain(a)) {
                    for a in alternatives {
                        routes.insert(format!("{group}.{a}"));
                    }
                }
            }
            _ => {
                routes.insert((*group).to_owned());
            }
        }
    }
    routes
}

#[test]
fn every_route_the_dispatch_documents_is_listed_by_capabilities() {
    let listed: BTreeSet<String> = listed().into_iter().collect();
    let missing: Vec<_> = documented().difference(&listed).cloned().collect();
    assert!(
        missing.is_empty(),
        "served by dispatch but absent from `ql capabilities`: {missing:?}"
    );
}

#[test]
fn every_listed_command_is_served_by_dispatch() {
    let unknown = |stderr: &str| {
        let text = stderr.to_lowercase();
        text.contains("unknown")
            && (text.contains("operation") || text.contains("verb") || text.contains("command"))
    };
    // The detector must see a refusal when one is real.
    assert!(unknown(&ql(&["kernel", "no-such-operation"]).2));
    assert!(unknown(&ql(&["no-such-command"]).2));
    for id in listed() {
        // Stage operations open a host exchange; their routes are checked
        // against the usage text below instead of being run.
        if id.starts_with("stage.") {
            continue;
        }
        let mut args: Vec<&str> = id.split('.').collect();
        if matches!(
            id.as_str(),
            "config-contribution" | "system" | "capabilities" | "version"
        ) {
            args = vec![id.as_str()];
        } else if id == "service.negotiate" {
            args = vec!["service", "negotiate", "capabilities"];
        } else if id.ends_with(".capabilities") || id == "verify" {
            // Takes no input; running it is harmless and proves it is served.
        } else {
            args.push("/nonexistent/ql-parity-input");
        }
        let (_, _, stderr) = ql(&args);
        assert!(
            !unknown(&stderr),
            "`ql capabilities` lists {id} but dispatch refuses it: {stderr}"
        );
    }
    let (_, _, usage) = ql(&["stage"]);
    for id in listed().iter().filter(|i| i.starts_with("stage.")) {
        let verb = id.trim_start_matches("stage.");
        assert!(usage.contains(&format!("ql stage {verb}")), "{id}: {usage}");
    }
}
