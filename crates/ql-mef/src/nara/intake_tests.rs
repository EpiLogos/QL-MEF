//! Domain tests exercise intake and its accepted six-office output. These do
//! not substitute for providers/nara's real Swiss/Kerykeion execution tests.
use serde_json::{Value, json};

use super::SourceRevision;
use super::intake::{
    BirthData, BirthPlace, IdentityProfile, IdentityReport, PROFILE_SCHEMA, ReportRoute,
    TimePrecision,
};

fn profile() -> IdentityProfile {
    IdentityProfile {
        schema: PROFILE_SCHEMA.into(),
        person_ref: "central:person:intake-contract-one".into(),
        nara_ref: "ql:nara:intake-contract-one".into(),
        name: "Intake contract person".into(),
        encoding_policy: None,
        composition_policy: None,
        birth: BirthData {
            date: Some("1990-06-15".into()),
            time: Some("14:30".into()),
            precision: TimePrecision::Exact,
            uncertainty_minutes: None,
            fold: None,
            place: Some(BirthPlace {
                label: "London".into(),
                latitude_degrees: 51.5074,
                longitude_degrees: -0.1278,
                timezone: "Europe/London".into(),
                source_ref: "self-report:birthplace".into(),
            }),
        },
        jungian: None,
        gene_keys: None,
        human_design: None,
        quintessence: None,
    }
}

#[test]
fn intake_text_obeys_native_provider_bounds_before_saving() {
    let mut value = profile();
    value.birth.place.as_mut().unwrap().source_ref = "x".repeat(513);
    assert!(value.validate().unwrap_err().contains("512"));
    value.birth.place.as_mut().unwrap().source_ref = "entered\nsource".into();
    assert!(value.validate().is_err());
    value.birth.place.as_mut().unwrap().source_ref = "地".repeat(512);
    assert!(value.validate().is_ok());
}

fn report(route: ReportRoute, data: Value) -> IdentityReport {
    IdentityReport {
        source: SourceRevision {
            source_ref: "protected:imported-report".into(),
            revision: "report-revision-one".into(),
            standing_ref: "reported".into(),
        },
        method: "declared source report; no inferred birth calculation".into(),
        route,
        data,
    }
}

fn reports() -> IdentityProfile {
    let mut value = profile();
    value.jungian = Some(report(
        ReportRoute::SelfReport,
        json!({"system":"mbti","type":"INTJ"}),
    ));
    value.gene_keys = Some(report(
        ReportRoute::Import,
        json!({"spheres":[{"name":"Life's Work","key":23,"line":4}]}),
    ));
    value.human_design = Some(report(
        ReportRoute::Import,
        json!({
            "type":"Reflector", "strategy":"Wait a lunar cycle", "authority":"Lunar",
            "profile":"2/4", "definition":"No definition", "personality_gates":[23],
            "design_gates":[1], "defined_centres":[], "channels":[]
        }),
    ));
    value.quintessence = Some(report(
        ReportRoute::SelfReport,
        json!({"reflection":"An authored personal account, not a calculated archetype."}),
    ));
    value
}

// This is an explicit unavailable receipt used to test binding and absence,
// never an invented successful natal calculation or a witness of astronomy.
fn unavailable(value: &IdentityProfile) -> Value {
    json!({
        "schema":"ql.nara-natal/v1", "request":value.natal_request().unwrap(),
        "status":"unavailable", "reason":"unknown-birth-time", "sky":null, "chart":null
    })
}

#[test]
fn six_offices_retain_absence_and_private_source_without_inventing_identity() {
    let value = profile().inspect(None).unwrap();
    let matrix = value["matrix"].as_array().unwrap();
    let slots = value["identity"]["slots"].as_array().unwrap();
    assert_eq!(matrix.len(), 6);
    assert_eq!(slots.len(), 6);
    let names = [
        "birthdate-name",
        "natal-chart",
        "jungian-assessment",
        "gene-keys",
        "human-design",
        "archetypal-quintessence",
    ];
    for (i, name) in names.into_iter().enumerate() {
        assert_eq!(matrix[i]["kind"], name);
        assert_eq!(matrix[i]["coordinate"], format!("M4.0.{i}"));
        assert_eq!(matrix[i]["available"], i == 0);
        assert_eq!(
            slots[i]["standing"],
            if i == 0 { "reported" } else { "unavailable" }
        );
        if i > 0 {
            assert!(slots[i]["source"].is_null());
            assert!(slots[i]["protected_value_ref"].is_null());
            assert!(!slots[i]["absence_reason"].as_str().unwrap().is_empty());
        }
    }
    assert_eq!(value["private"], true);
    assert_eq!(value["public_export"], false);
    assert!(value["identity"]["identity_quaternion_ref"].is_null());
    assert!(value["identity"]["m3_form_address_ref"].is_null());
    assert!(value["natal_composition"].is_null());
}

#[test]
fn reports_keep_native_routes_sources_and_absence_of_natal_calculation() {
    let value = reports();
    let reading = value.inspect(None).unwrap();
    for (i, supplied) in [true, false, true, true, true, true]
        .into_iter()
        .enumerate()
    {
        assert_eq!(reading["matrix"][i]["available"], supplied);
    }
    for (i, original) in [
        (2, value.jungian.as_ref().unwrap()),
        (3, value.gene_keys.as_ref().unwrap()),
        (4, value.human_design.as_ref().unwrap()),
        (5, value.quintessence.as_ref().unwrap()),
    ] {
        assert_eq!(reading["matrix"][i]["data"], original.data);
        assert_eq!(
            reading["matrix"][i]["source"],
            serde_json::to_value(&original.source).unwrap()
        );
        assert_eq!(reading["identity"]["slots"][i]["standing"], "reported");
    }
    assert_eq!(reading["matrix"][2]["route"], "self-report");
    assert_eq!(reading["matrix"][3]["route"], "import");
    assert_eq!(reading["matrix"][4]["route"], "import");
    assert_eq!(reading["matrix"][5]["route"], "self-report");
    assert!(reading["natal_composition"].is_null());
}

#[test]
fn profile_roundtrip_keeps_revision_and_reading_and_corrections_replace_basis() {
    let original = reports();
    let reopened: IdentityProfile =
        serde_json::from_slice(&serde_json::to_vec(&original).unwrap()).unwrap();
    assert_eq!(original, reopened);
    assert_eq!(original.revision().unwrap(), reopened.revision().unwrap());
    assert_eq!(
        original.inspect(None).unwrap(),
        reopened.inspect(None).unwrap()
    );
    let old = original.revision().unwrap();
    let mut corrected = original.clone();
    corrected.birth.time = Some("14:31".into());
    assert_ne!(old, corrected.revision().unwrap());
    assert_ne!(
        original.inspect(None).unwrap()["material"]["value"],
        corrected.inspect(None).unwrap()["material"]["value"]
    );
    corrected = original.clone();
    corrected.birth.place.as_mut().unwrap().timezone = "Europe/Paris".into();
    assert_ne!(old, corrected.revision().unwrap());
    corrected = original.clone();
    corrected.jungian.as_mut().unwrap().data["type"] = json!("INFJ");
    assert_ne!(old, corrected.revision().unwrap());
    corrected = original.clone();
    corrected.gene_keys.as_mut().unwrap().source.revision = "report-revision-two".into();
    assert_ne!(old, corrected.revision().unwrap());
    assert_eq!(old, original.revision().unwrap());
}

#[test]
fn natal_receipts_are_bound_to_the_person_and_every_source_correction() {
    let mut original = profile();
    original.birth.precision = TimePrecision::Unknown;
    original.birth.time = None;
    let receipt = unavailable(&original);
    let first = original.inspect(Some(&receipt)).unwrap();
    assert_eq!(first["matrix"][1]["absence_reason"], "unknown-birth-time");
    assert_eq!(first["matrix"][1]["available"], false);
    assert_eq!(first["input_revision"], original.revision().unwrap());
    let mut other = original.clone();
    other.person_ref = "central:person:other".into();
    assert!(other.inspect(Some(&receipt)).is_err());
    let mut corrected = original.clone();
    corrected.birth.date = Some("1990-06-16".into());
    assert!(corrected.inspect(Some(&receipt)).is_err());
    assert!(corrected.inspect(Some(&unavailable(&corrected))).is_ok());
    for key in ["person_ref", "source_revision", "backend_policy"] {
        let mut changed = receipt.clone();
        changed["request"][key] = json!("different");
        assert!(
            original.inspect(Some(&changed)).is_err(),
            "accepted mismatched {key}"
        );
    }
    let mut changed = receipt.clone();
    changed["schema"] = json!("ql.nara-natal/v0");
    assert!(original.inspect(Some(&changed)).is_err());
    let mut refreshed = receipt;
    refreshed["receipt_time"] = json!("later receipt metadata");
    assert_eq!(
        first["material"],
        original.inspect(Some(&refreshed)).unwrap()["material"]
    );
}

#[test]
fn calendar_validation_preserves_partial_knowledge_and_rejects_impossible_dates() {
    for good in [
        "1800-01-01",
        "2000-02-29",
        "2024-02-29",
        "2399-12-31",
        "1990",
        "1990-06",
    ] {
        let mut value = profile();
        value.birth.date = Some(good.into());
        assert!(value.validate().is_ok(), "rejected {good}");
        assert_eq!(value.natal_request().unwrap()["birth"]["date"], good);
    }
    for bad in [
        "1799-12-31",
        "2400-01-01",
        "1900-02-29",
        "2023-02-29",
        "2024-04-31",
        "2024-00-01",
        "2024-13-01",
        "2024-01-00",
        "24-01-01",
        "2024-1-01",
        "2024-01-01extra",
        "2024-99",
        "",
        "é2024",
    ] {
        let mut value = profile();
        value.birth.date = Some(bad.into());
        assert!(value.validate().is_err(), "accepted {bad}");
    }
    let mut absent = profile();
    absent.birth.date = None;
    absent.birth.time = None;
    absent.birth.precision = TimePrecision::Unknown;
    absent.birth.place = None;
    assert!(absent.validate().is_ok());
    assert!(absent.natal_request().unwrap()["birth"]["date"].is_null());
}

#[test]
fn clock_precision_and_fold_never_silently_supply_exact_birth_time() {
    for good in ["00:00", "23:59", "12:30:59"] {
        let mut value = profile();
        value.birth.time = Some(good.into());
        assert!(value.validate().is_ok());
    }
    for bad in [
        "24:00",
        "12:60",
        "12:30:60",
        "9:05",
        "12:5",
        "-1:00",
        "12:00:00Z",
        "",
    ] {
        let mut value = profile();
        value.birth.time = Some(bad.into());
        assert!(value.validate().is_err(), "accepted {bad}");
    }
    let mut exact = profile();
    exact.birth.time = None;
    assert!(exact.validate().is_err());
    exact = profile();
    exact.birth.uncertainty_minutes = Some(1);
    assert!(exact.validate().is_err());
    let mut approximate = profile();
    approximate.birth.precision = TimePrecision::Approximate;
    for uncertainty in [None, Some(0), Some(1441)] {
        approximate.birth.uncertainty_minutes = uncertainty;
        assert!(approximate.validate().is_err());
    }
    for uncertainty in [1, 30, 1440] {
        approximate.birth.uncertainty_minutes = Some(uncertainty);
        assert!(approximate.validate().is_ok());
        assert_eq!(
            approximate.natal_request().unwrap()["birth"]["precision"],
            "approximate"
        );
    }
    approximate.birth.time = None;
    assert!(approximate.validate().is_err());
    let mut unknown = profile();
    unknown.birth.precision = TimePrecision::Unknown;
    assert!(unknown.validate().is_err());
    unknown.birth.time = None;
    assert!(unknown.validate().is_ok());
    unknown.birth.fold = Some(0);
    assert!(unknown.validate().is_err());
    unknown.birth.fold = None;
    unknown.birth.uncertainty_minutes = Some(30);
    assert!(unknown.validate().is_err());
    for fold in [0, 1] {
        let mut value = profile();
        value.birth.fold = Some(fold);
        assert!(value.validate().is_ok());
    }
    let mut invalid_fold = profile();
    invalid_fold.birth.fold = Some(2);
    assert!(invalid_fold.validate().is_err());
}

#[test]
fn birthplace_refuses_nonfinite_out_of_range_and_unattributable_coordinates() {
    for bad in [f64::NAN, f64::INFINITY, -90.001, 90.001] {
        let mut value = profile();
        value.birth.place.as_mut().unwrap().latitude_degrees = bad;
        assert!(value.validate().is_err());
    }
    for bad in [f64::NAN, f64::NEG_INFINITY, -180.001, 180.001] {
        let mut value = profile();
        value.birth.place.as_mut().unwrap().longitude_degrees = bad;
        assert!(value.validate().is_err());
    }
    let mut value = profile();
    value.birth.place.as_mut().unwrap().source_ref.clear();
    assert!(value.validate().is_err());
    let mut value = profile();
    value.birth.place.as_mut().unwrap().timezone.clear();
    assert!(value.validate().is_err());
}

#[test]
fn import_requirements_do_not_promote_self_report_to_bodygraph_or_gene_keys() {
    for slot in ["gene_keys", "human_design"] {
        let mut value = serde_json::to_value(reports()).unwrap();
        value[slot]["route"] = json!("self-report");
        let rejected: IdentityProfile = serde_json::from_value(value).unwrap();
        assert!(
            rejected.validate().is_err(),
            "accepted self-reported {slot}"
        );
    }
    for (system, identity, valid) in [
        ("mbti", "A", false),
        ("jungian", "T", false),
        ("16-personalities", "A", true),
        ("16-personalities", "T", true),
        ("16-personalities", "X", false),
    ] {
        let mut value = reports();
        value.jungian.as_mut().unwrap().data =
            json!({"system":system,"type":"INTJ","identity":identity});
        assert_eq!(value.validate().is_ok(), valid);
    }
    for kind in ["intj", "INTJ-T", "XXXX", "INT", "ENFJJ"] {
        let mut value = reports();
        value.jungian.as_mut().unwrap().data["type"] = json!(kind);
        assert!(value.validate().is_err());
    }
    for spheres in [
        json!([]),
        json!([{"name":"Life's Work","key":0,"line":1}]),
        json!([{"name":"Life's Work","key":65,"line":1}]),
        json!([{"name":"Life's Work","key":1,"line":7}]),
        json!([{"name":"Life's Work","key":1,"line":1},{"name":"Life's Work","key":2,"line":2}]),
    ] {
        let mut value = reports();
        value.gene_keys.as_mut().unwrap().data["spheres"] = spheres;
        assert!(value.validate().is_err());
    }
    for (field, invalid) in [
        ("personality_gates", json!([1, 1])),
        ("design_gates", json!([0])),
        ("design_gates", json!([65])),
        ("design_gates", json!([1.5])),
        ("defined_centres", json!(["crown"])),
        ("defined_centres", json!(["head", "head"])),
        ("channels", Value::Null),
    ] {
        let mut value = reports();
        value.human_design.as_mut().unwrap().data[field] = invalid;
        assert!(value.validate().is_err(), "accepted invalid {field}");
    }
}

#[test]
fn undeclared_profile_fields_and_unattributable_reports_are_refused() {
    let mut serialized = serde_json::to_value(profile()).unwrap();
    serialized["q_identity"] = json!([1, 0, 0, 0]);
    assert!(serde_json::from_value::<IdentityProfile>(serialized).is_err());
    for slot in ["jungian", "gene_keys", "human_design", "quintessence"] {
        let mut value = serde_json::to_value(reports()).unwrap();
        value[slot]["source"]["revision"] = json!("");
        assert!(
            serde_json::from_value::<IdentityProfile>(value)
                .unwrap()
                .validate()
                .is_err()
        );
    }
}

#[test]
fn channels_require_distinct_pairs_present_in_the_imported_gate_sets() {
    for channels in [
        json!([null]),
        json!(["23-43"]),
        json!([[23]]),
        json!([[23, 43, 1]]),
        json!([[23, 23]]),
        json!([[0, 23]]),
        json!([[23, 65]]),
        json!([[23, 1.5]]),
        json!([[23, 43]]),
    ] {
        let mut value = reports();
        value.human_design.as_mut().unwrap().data["channels"] = channels;
        assert!(value.validate().is_err());
    }
    let mut value = reports();
    let data = &mut value.human_design.as_mut().unwrap().data;
    data["type"] = json!("Projector");
    data["strategy"] = json!("Wait for the invitation");
    data["authority"] = json!("Environmental");
    data["definition"] = json!("Single definition");
    data["design_gates"] = json!([43]);
    data["defined_centres"] = json!(["ajna", "throat"]);
    data["channels"] = json!([[23, 43]]);
    assert!(value.validate().is_ok());
    value.human_design.as_mut().unwrap().data["channels"] = json!([[23, 43], [43, 23]]);
    assert!(value.validate().is_err());
}

#[test]
fn malformed_natal_envelopes_cannot_be_marked_available() {
    let mut value = profile();
    value.birth.precision = TimePrecision::Unknown;
    value.birth.time = None;
    let receipt = unavailable(&value);
    for (field, bad) in [
        ("chart", json!({})),
        ("chart", json!("chart.svg")),
        ("status", json!("available")),
        ("status", json!("calculated")),
        ("reason", Value::Null),
        ("reason", json!("")),
    ] {
        let mut malformed = receipt.clone();
        malformed[field] = bad;
        assert!(
            value.inspect(Some(&malformed)).is_err(),
            "accepted malformed {field}"
        );
    }
}
