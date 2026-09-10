use ql_cli::execute_cli;

fn run(args: &[&str]) -> Result<String, ql_cli::CliError> {
    execute_cli(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

#[test]
fn native_coverage_and_joined_ledger_require_no_python_or_working_checkout() {
    let value: serde_json::Value =
        serde_json::from_str(&run(&["kernel", "coverage", "M3", "--json"]).unwrap()).unwrap();
    assert_eq!(value["schema"], "ql.m-coverage/v1");
    assert_eq!(value["structural_coordinates"], 996);
    assert!(!value["blocking_rows"].as_array().unwrap().is_empty());
    let value: serde_json::Value =
        serde_json::from_str(&run(&["kernel", "ledger", "M0-4.0/1/2", "--json"]).unwrap()).unwrap();
    assert_eq!(value["coordinate"]["source_ref"], "#0-4.0/1/2");
    assert!(
        run(&["kernel", "validate-ledger"])
            .unwrap()
            .contains("not completion")
    );
}

#[test]
fn query_errors_do_not_fall_back_to_whole_m_or_unknown_readiness() {
    for args in [
        vec!["kernel", "coverage"],
        vec!["kernel", "coverage", "M6"],
        vec!["kernel", "coverage", "M1", "M2"],
        vec!["kernel", "coverage", "M1", "--stratum"],
        vec!["kernel", "coverage", "M1", "--wat"],
        vec![
            "kernel",
            "coverage",
            "M1",
            "--stratum",
            "c",
            "--stratum",
            "rust",
        ],
        vec!["kernel", "coverage", "M1", "--require", "COMPLETE"],
        vec!["kernel", "validate-ledger", "M1"],
    ] {
        assert!(run(&args).is_err(), "accepted invalid {args:?}");
    }
}

#[test]
fn capabilities_disclose_the_ledger_commands() {
    let capabilities: serde_json::Value =
        serde_json::from_str(&run(&["capabilities", "--json"]).unwrap()).unwrap();
    assert!(
        capabilities["commands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "kernel.coverage")
    );
}
