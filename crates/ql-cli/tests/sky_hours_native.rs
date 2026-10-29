//! The planets of the day and the hour through the actual executable and the qualified Swiss provider,
//! and M2's body constants by fibre. No recorded day stands in for the calculation.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn ql(args: &[&str], input: Option<&Value>) -> std::process::Output {
    let mut p = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = p.stdin.take().unwrap();
    if let Some(input) = input {
        stdin
            .write_all(&serde_json::to_vec(input).unwrap())
            .unwrap();
    }
    drop(stdin);
    p.wait_with_output().unwrap()
}

fn ask(epoch: &str, latitude: f64, longitude: f64, timezone: &str) -> Value {
    json!({"schema": "ql.sky-day-request/v1", "epoch": epoch, "scope": "world-declared", "backend_policy": "allow-moshier",
        "place": {"reference": "controlled:place:test", "label": "Test place", "latitude_degrees": latitude,
            "longitude_degrees": longitude, "altitude_metres": 0.0, "timezone": timezone}})
}

#[test]
fn monday_noon_at_greenwich_is_mercurys_seventh_hour_of_the_moons_day() {
    let request = ask("2026-09-28T12:00:00Z", 51.4769, -0.0005, "Europe/London");
    let out = ql(&["nara", "hours", "-", "--json"], Some(&request));
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let hours: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(hours["schema"], "ql.sky-hours/v1");
    assert_eq!(hours["day"]["schema"], "ql.sky-day/v1");
    assert_eq!(hours["day"]["request"], request);
    assert_eq!(hours["day"]["sunrise_iso_weekday"], 1);
    assert_eq!(hours["day_ruler"]["body"], "Moon");
    assert_eq!(hours["hour"]["half"], "day");
    assert_eq!(hours["hour"]["of"], 7);
    assert_eq!(hours["hour"]["ruler"]["body"], "Mercury");
    assert_eq!(hours["hour"]["bound"].as_array().unwrap().len(), 3);
    assert_eq!(hours["private"], false);
}

#[test]
fn a_place_where_the_sun_does_not_rise_counts_no_hours() {
    let request = ask("2026-12-21T12:00:00Z", 78.22, 15.65, "Arctic/Longyearbyen");
    let out = ql(&["nara", "hours", "-", "--json"], Some(&request));
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("circumpolar"), "{stderr}");
    let mut snapshot = request.clone();
    snapshot["schema"] = json!("ql.sky-request/v1");
    let refused = ql(&["nara", "hours", "-", "--json"], Some(&snapshot));
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("ql.sky-day-request/v1"));
}

#[test]
fn the_body_constants_are_m2s_proposed_table_by_fibre() {
    let out = ql(&["kernel", "body-constants", "--json"], None);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let table: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(table["schema"], "ql.m2-body-constants/v1");
    assert_eq!(table["standing"], "proposed");
    assert_eq!(table["rows"][2]["fibre"], "fire");
    assert_eq!(table["rows"][2]["strike_share"], 0.85);
    assert_eq!(table["unbodied"], json!(["#2-2-2-5-0/1"]));
}
