//! The #129 parent handoff uses two headless callers and one operational owner.
//! Surface names live in the consumer fixture, never in M1's symbolic kernel.
use ql_mef::m1_engine::{CONTRACT, EngineConfig, M1Engine};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    schema: String,
    standing: String,
    consumer_refs: [String; 2],
    config: EngineConfig,
    advance_ticks: u64,
    deep_coordinate: String,
}

struct ConsumerReading<'a> {
    consumer_ref: &'a str,
    frame: &'a Value,
}

fn consume<'a>(consumer_ref: &'a str, frame: &'a Value) -> ConsumerReading<'a> {
    assert_eq!(frame["schema"], CONTRACT);
    for key in [
        "engine_version",
        "registry_revision",
        "coordinate",
        "reflection",
        "clock",
        "carrier",
        "music",
        "source",
        "relations",
    ] {
        assert!(
            !frame[key].is_null(),
            "missing shared producer field: {key}"
        );
    }
    ConsumerReading {
        consumer_ref,
        frame,
    }
}

fn assert_shared(frame: &Value, consumers: &[String; 2], event: &str, subject: &str) {
    // One immutable generation is delivered to both readers; neither constructs
    // another M1Engine, substitutes a surface identity or re-derives the state.
    let cosmic = consume(&consumers[0], frame);
    let personal = consume(&consumers[1], frame);
    assert_ne!(cosmic.consumer_ref, personal.consumer_ref);
    assert!(std::ptr::eq(cosmic.frame, personal.frame));
    assert_eq!(cosmic.frame, personal.frame);
    assert_eq!(frame["config"]["event_ref"], event);
    assert_eq!(frame["config"]["subject_coordinate"], subject);
    assert_eq!(frame["carrier"]["clock"], frame["clock"]);
    assert!(frame["config"]["revision"].is_string());
    assert!(frame["clock"]["cycle"].is_string());
    // This fixture exercises a producer, not a graphical or personal-data host.
    assert_eq!(frame["standing"]["experiential"], "unassessed");
}

#[test]
fn cosmic_and_personal_consume_one_owner_generation_without_forking() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-parent-consumers-v1.json"
    ))
    .unwrap();
    assert_eq!(fixture.schema, "ql.m1.parent-consumer-fixture/v1");
    assert_eq!(
        fixture.consumer_refs,
        ["epi.cosmic.123", "epi.personal.450"]
    );
    assert!(
        fixture
            .standing
            .contains("not installed surface acceptance")
    );
    let config = fixture.config;
    let original_revision = config.revision.parse::<u64>().unwrap();
    let mut owner = M1Engine::new(config.clone()).unwrap();
    let initial = owner.snapshot().unwrap();
    assert_shared(
        &initial,
        &fixture.consumer_refs,
        &config.event_ref,
        &config.subject_coordinate,
    );
    assert_eq!(initial["config"], json!(config));
    assert_eq!(initial["clock"]["phase"], 1);
    assert_eq!(initial["clock"]["hopf_fiber"], 1);

    // A command from either caller returns to the same owner and publishes one
    // new generation. A second caller's old expected revision cannot overwrite it.
    owner
        .advance(original_revision, fixture.advance_ticks)
        .unwrap();
    let advanced = owner.snapshot().unwrap();
    assert_eq!(advanced["config"]["revision"], "8");
    assert_eq!(advanced["clock"]["cycle"], "9007199254740994");
    assert_eq!(advanced["clock"]["tick12"], 0);
    assert!(owner.advance(original_revision, 1).is_err());
    assert_eq!(owner.snapshot().unwrap(), advanced);
    assert_eq!(initial["config"]["revision"], "7");
    assert_eq!(initial["clock"]["cycle"], "9007199254740993");
    assert_eq!(advanced["source"], initial["source"]);
    assert_eq!(advanced["registry_revision"], initial["registry_revision"]);
    assert_eq!(advanced["coordinate"], initial["coordinate"]);
    assert_shared(
        &advanced,
        &fixture.consumer_refs,
        &config.event_ref,
        &config.subject_coordinate,
    );

    // Deep selection and Return preserve the updated event/clock instead of
    // reopening either parent against a new simulation or initial generation.
    owner.select(8, &fixture.deep_coordinate).unwrap();
    let deep = owner.snapshot().unwrap();
    assert_ne!(deep["coordinate"], advanced["coordinate"]);
    assert_eq!(deep["clock"], advanced["clock"]);
    owner.select(9, &config.selected_coordinate).unwrap();
    let returned = owner.snapshot().unwrap();
    assert_eq!(returned["config"]["revision"], "10");
    for field in ["coordinate", "clock", "carrier", "music", "source"] {
        assert_eq!(returned[field], advanced[field]);
    }
    assert_shared(
        &returned,
        &fixture.consumer_refs,
        &config.event_ref,
        &config.subject_coordinate,
    );
}
