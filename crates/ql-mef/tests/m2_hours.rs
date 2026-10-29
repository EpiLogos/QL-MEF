//! The planets of the day and the hour, counted from a day at a place by the map's order.
use ql_mef::m2_body;
use ql_mef::m2_engine::MaterialFibre;
use ql_mef::m2_hours::{self, DAY_CONTRACT, HOURS_CONTRACT};
use serde_json::{Value, json};

/// Greenwich on Monday 28 September 2026: sunrise 05:55, sunset 17:45, next sunrise 05:57 (UTC).
const RISE: i64 = 1_790_574_900_000;
const SET: i64 = 1_790_617_500_000;
const NEXT: i64 = 1_790_661_420_000;
const NOON: i64 = 1_790_596_800_000;

fn day(at: i64, weekday: u8, scope: &str) -> Value {
    json!({"schema": DAY_CONTRACT, "epoch_unix_ms": at, "sunrise_unix_ms": RISE, "sunset_unix_ms": SET,
        "next_sunrise_unix_ms": NEXT, "sunrise_iso_weekday": weekday, "sunrise_local_date": "2026-09-28",
        "scope": scope, "place": {"reference": "controlled:place:greenwich", "label": "Greenwich"},
        "day_ref": "sha256:controlled"})
}

#[test]
fn monday_noon_at_greenwich_is_the_seventh_hour_of_the_day_and_mercurys() {
    let hours = m2_hours::hours(&day(NOON, 1, "world-declared")).unwrap();
    assert_eq!(hours["schema"], HOURS_CONTRACT);
    assert_eq!(
        hours["day_ruler"],
        json!({"body": "Moon", "coordinate": "#2-5-4"})
    );
    assert_eq!(hours["hour"]["of"], 7);
    assert_eq!(hours["hour"]["half"], "day");
    assert_eq!(hours["hour"]["ruler"]["body"], "Mercury");
    // The hour's bounds: the seventh twelfth of sunrise to sunset.
    let span = SET - RISE;
    assert_eq!(hours["hour"]["starts_unix_ms"], RISE + span * 6 / 12);
    assert_eq!(hours["hour"]["ends_unix_ms"], RISE + span * 7 / 12);
    // The maqāmāt the map binds to Mercury: its three dominant rules.
    let bound: Vec<(u64, &str)> = hours["hour"]["bound"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| {
            (
                b["maqam_index"].as_u64().unwrap(),
                b["role"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(bound, [(2, "dominant"), (5, "dominant"), (70, "dominant")]);
    assert_eq!(hours["private"], false);
    assert_eq!(hours["day"], day(NOON, 1, "world-declared"));
    assert_eq!(hours["order"]["source"]["coordinate"], "#2-5");
}

#[test]
fn the_first_hour_is_the_days_own_planet_and_the_night_goes_on_round_the_order() {
    let first = m2_hours::hours(&day(RISE, 1, "observer-private")).unwrap();
    assert_eq!(first["hour"]["of"], 1);
    assert_eq!(first["hour"]["ruler"]["body"], "Moon");
    assert_eq!(first["private"], true);
    // 23:00: the sixth hour of the night, the eighteenth from sunrise; Moon + 17 places is Mars.
    let night = m2_hours::hours(&day(NOON + 11 * 3_600_000, 1, "world-declared")).unwrap();
    assert_eq!(night["hour"]["half"], "night");
    assert_eq!(night["hour"]["of"], 6);
    assert_eq!(night["hour"]["ruler"]["body"], "Mars");
    // The last instant before the next sunrise is the twelfth hour of the night.
    let last = m2_hours::hours(&day(NEXT - 1, 1, "world-declared")).unwrap();
    assert_eq!(last["hour"]["of"], 12);
    // Sunday's first hour is the Sun's; Saturday's is Saturn's.
    for (weekday, planet) in [(7, "Sun"), (6, "Saturn"), (5, "Venus")] {
        let hours = m2_hours::hours(&day(RISE, weekday, "world-declared")).unwrap();
        assert_eq!(hours["day_ruler"]["body"], planet, "weekday {weekday}");
        assert_eq!(hours["hour"]["ruler"]["body"], planet);
    }
}

#[test]
fn a_day_out_of_order_or_of_another_kind_is_refused_in_words() {
    let mut early = day(NOON, 1, "world-declared");
    early["epoch_unix_ms"] = json!(RISE - 1);
    assert!(
        m2_hours::hours(&early)
            .unwrap_err()
            .contains("do not stand round")
    );
    let mut other = day(NOON, 1, "world-declared");
    other["schema"] = json!("ql.sky-snapshot/v1");
    assert!(m2_hours::hours(&other).is_err());
    assert!(m2_hours::hours(&day(NOON, 0, "world-declared")).is_err());
    assert!(m2_hours::hours(&day(NOON, 1, "shared-geocentric")).is_err());
}

#[test]
fn the_body_table_gives_each_fibre_the_manuals_starting_constants() {
    let reading = m2_body::reading().unwrap();
    assert_eq!(reading["schema"], "ql.m2-body-constants/v1");
    assert_eq!(reading["standing"], "proposed");
    let rows: Vec<(&str, f64, f64, f64, f64)> = reading["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["fibre"].as_str().unwrap(),
                r["strike_share"].as_f64().unwrap(),
                r["q_factor"].as_f64().unwrap(),
                r["agitation"].as_f64().unwrap(),
                r["boundary_strength"].as_f64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        rows,
        [
            ("earth", 0.45, 12.0, 0.1, 20.0),
            ("water", 0.55, 6.0, 0.6, 10.0),
            ("fire", 0.85, 3.0, 2.0, 4.0),
            ("air", 0.65, 1.5, 3.5, 2.0),
        ]
    );
    // Each fibre's tattva is the map's; Ākāśa carries none.
    let tattvas: Vec<&Value> = reading["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| &r["tattvas"])
        .collect();
    assert_eq!(
        tattvas,
        [
            &json!(["#2-2-2-5-5"]),
            &json!(["#2-2-2-5-4"]),
            &json!(["#2-2-2-5-3"]),
            &json!(["#2-2-2-5-2"])
        ]
    );
    assert_eq!(reading["unbodied"], json!(["#2-2-2-5-0/1"]));
    assert_eq!(m2_body::constants(MaterialFibre::Fire).strike_share, 0.85);
}
