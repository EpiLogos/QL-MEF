//! The task-oriented frontdoor help (command-encounter classification §6):
//! bare/help print the outcome-grouped reference with no side effects. QL's
//! domain distinctions are preserved — only exposure is simplified; every
//! exact route stays listed.

use std::process::Command;

#[test]
fn bare_and_help_print_the_grouped_reference_without_side_effects() {
    for args in [vec!["--help"], vec!["help"]] {
        let bin = env!("CARGO_BIN_EXE_ql");
        let output = Command::new(bin)
            .args(&args)
            .output()
            .unwrap_or_else(|e| panic!("ql {args:?} should run: {e}"));
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        assert_eq!(output.status.code(), Some(0), "help never fails: {args:?}");
        assert!(stdout.contains("Quaternal Logic"), "{args:?}: {stdout}");
        assert!(
            stdout.contains("the semantic kernel"),
            "the opening line names the product's act: {args:?}: {stdout}"
        );
        for section in [
            "Kernel derivation and coverage (everyday",
            "Registry, lens and frame reads (contextual):",
            "Specialised constitution and invocation (contextual",
            "Service negotiation (operator",
            "System, diagnosis and configuration (operator",
        ] {
            assert!(stdout.contains(section), "{args:?}: missing `{section}`");
        }
        for route in [
            "ql kernel m1",
            "ql kernel coverage",
            "ql kernel ledger",
            "ql kernel apply",
            "ql matheme derive",
            "ql matheme shadow",
            "ql mef lenses",
            "ql context-frame list",
            "ql vak compose",
            "ql vak locate",
            "ql vak context",
            "ql vak workflow-types",
            "ql techne reading",
            "ql epi-agent constitution",
            "ql epi-agent faculty",
            "ql epi-agent invoke",
            "ql service negotiate",
            "ql verify",
            "ql capabilities",
            "ql system",
            "ql config-contribution",
            "ql config apply",
        ] {
            assert!(stdout.contains(route), "{args:?}: missing `{route}`");
        }
    }
}
