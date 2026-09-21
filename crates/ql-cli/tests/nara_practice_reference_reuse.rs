//! Actual native CLI in an isolated private World; no model or human data.
use serde_json::{Value, json};
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};
static ID: AtomicU64 = AtomicU64::new(0);
struct World(PathBuf);
impl World {
    fn new() -> Self {
        // Canonicalize the base temp dir so macOS's /var -> /private/var (and
        // /tmp -> /private/tmp) symlink does not trip the store's non-symlink
        // storage guard; the unique leaf we create is never itself a symlink.
        let base =
            std::fs::canonicalize(std::env::temp_dir()).unwrap_or_else(|_| std::env::temp_dir());
        let root = base.join(format!(
            "ql-nara-reuse-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            ID.fetch_add(1, Ordering::SeqCst)
        ));
        Self(root)
    }
    fn call(&self, request: Value) -> Value {
        let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
            .args(["nara", "--request-file", "-", "--json"])
            .env("QL_NARA_HOME", &self.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start selected native ql");
        child
            .stdin
            .take()
            .unwrap()
            .write_all(serde_json::to_string(&request).unwrap().as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "native CLI failure: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).expect("native JSON reply")
    }
}
impl Drop for World {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn source(name: &str) -> Value {
    json!({"source_ref":format!("source:{name}"),"revision":"r1","standing_ref":"controlled-test"})
}
fn consent() -> Value {
    json!({"actor_ref":"person:controlled","actor_kind":"human","personal_data":true})
}
fn seed() -> Value {
    let q = json!({"w":1.0,"x":0.0,"y":0.0,"z":0.0});
    json!({"nara_ref":"nara:controlled","personal":{
        "schema":"ql.nara-personal-field/v1","subject_id":"subject:controlled","constitution_ref":"constitution:controlled","reception_generation":1,
        "event":{"event_ref":"event:controlled","subject_ref":"subject:controlled","profile_generation":1,"registry_revision":"r1","m1_revision":"m1:r1","m2_source_ref":"m2:r1","m2_contract_ref":"m2:contract","m3_source_ref":"m3:r1","m3_contract_ref":"m3:contract"},
        "observed_at_unix_ms":1,"consent":"granted","lifecycle":"active","q_identity":q,"q_transit":q,"q_activity":q,"q_composed":q,"ephemeral_source":null,
        "receivers":(0..7).map(|n| json!({"ordinal":n,"label":format!("centre:{n}"),"source":source(&format!("centre:{n}")),
            "input_basis":(["m1","m2","m3"].map(|m| json!({"basis_ref":format!("{m}:r1"),"source_ref":format!("{m}:{n}"),"value":n}))),
            "bioquaternion":q,"receiver_orientation":q,"composed_orientation":q,"orientation_alignment":1.0,"drive":n,"resonance":0.0,"reradiation":0.0})).collect::<Vec<_>>(),
        "earth_body":{"source":source("earthbody"),"frame_ref":"earth-fixed","orientation":q,"relation_alignment":1.0},"aggregate_resonance":0.0,"aggregate_reradiation":0.0,"source_revisions":[source("personal")],"standing":"controlled-test"},
        "embodied":{"elemental_efwa":{"earth":0.1,"fire":0.2,"water":0.3,"air":0.4},"elemental_source":source("elemental"),"elemental_standing":"reported","nadi_refs":[],"sushumna_ref":null,"temporal_astrology_refs":[],"materia_refs":[],"operation_refs":[],"safety_intensity":null,"contraindication_refs":[],"response_refs":[],"adjustment_refs":[]}})
}
fn apply_request(record: &Value, id: &str, mutation: Value) -> Value {
    json!({"operation":"apply","request_id":id,"consent":consent(),"target":record["target"],"expected_revision":record["revision"],"mutation":mutation})
}
fn applied(world: &World, record: &Value, id: &str, mutation: Value) -> Value {
    let reply = world.call(apply_request(record, id, mutation));
    assert_eq!(reply["ok"], true, "native practice transition: {reply}");
    assert_eq!(reply["receipt"]["request_id"], id);
    assert_eq!(
        reply["record"]["revision"].as_u64(),
        record["revision"].as_u64().map(|v| v + 1)
    );
    reply["record"].clone()
}
fn open(world: &World) -> Value {
    let reply = world.call(
        json!({"operation":"create","request_id":"create","consent":consent(),"seed":seed()}),
    );
    assert_eq!(reply["ok"], true, "native create: {reply}");
    applied(
        world,
        &reply["record"],
        "start",
        json!({"kind":"practice_start","phase":{
        "phase_ref":"practice:one","storey":0,"decan":0,"stroke":0,"operation_refs":[],"protocol_ref":"source:protocol","container_ref":reply["record"]["target"]["record_ref"],"source_revisions":[source("protocol")],"opened_at_unix_ms":1,"closed_at_unix_ms":null,"safety":"clear","feedback_refs":[]}}),
    )
}
fn phase(record: &Value) -> &Value {
    &record["domain"]["transformation"]["phase_history"]["phases"][0]
}
#[test]
fn one_source_can_support_distinct_hold_resume_close_acts_without_duplicate_links() {
    let world = World::new();
    let mut record = open(&world);
    for n in 0..3 {
        record = applied(
            &world,
            &record,
            &format!("hold:{n}"),
            json!({"kind":"practice_hold","phase_ref":"practice:one","feedback_ref":"central:source:feedback"}),
        );
        assert_eq!(phase(&record)["safety"], "consent-required");
        record = applied(
            &world,
            &record,
            &format!("resume:{n}"),
            json!({"kind":"practice_resume","phase_ref":"practice:one","safety_review_ref":"central:source:feedback"}),
        );
        assert_eq!(phase(&record)["safety"], "clear");
    }
    let request = apply_request(
        &record,
        "close",
        json!({"kind":"practice_close","phase_ref":"practice:one","feedback_refs":["central:source:feedback","central:source:new","central:source:new"]}),
    );
    let reply = world.call(request.clone());
    assert_eq!(reply["ok"], true, "native close: {reply}");
    record = reply["record"].clone();
    assert_eq!(
        phase(&record)["feedback_refs"],
        json!(["central:source:feedback", "central:source:new"])
    );
    assert!(phase(&record)["closed_at_unix_ms"].is_u64());
    assert_eq!(
        record["receipts"].as_array().unwrap().len(),
        9,
        "one receipt for each real transition, not one per source"
    );
    let replay = world.call(request);
    assert_eq!(replay["duplicate"], true);
    assert_eq!(
        replay["record"], record,
        "replay cannot create another occurrence"
    );
    let restored =
        world.call(json!({"operation":"read","target":record["target"],"consent":consent()}));
    assert_eq!(
        restored["record"], record,
        "independent native process reopens exact private history"
    );
    let stale = world.call(json!({"operation":"apply","request_id":"stale","consent":consent(),"target":record["target"],"expected_revision":1,"mutation":{"kind":"practice_hold","phase_ref":"practice:one","feedback_ref":"source:unapplied"}}));
    assert_eq!(stale["ok"], false);
    assert_eq!(
        world.call(json!({"operation":"read","target":record["target"],"consent":consent()}))["record"],
        record
    );
}
#[test]
fn reference_reuse_does_not_relax_resume_safety_or_input_validation() {
    let world = World::new();
    let record = open(&world);
    let invalid = world.call(apply_request(&record,"not-held",json!({"kind":"practice_resume","phase_ref":"practice:one","safety_review_ref":"source:review"})));
    assert_eq!(
        invalid["ok"], false,
        "a clear phase cannot fabricate a resumed hold"
    );
    let invalid = world.call(apply_request(
        &record,
        "blank",
        json!({"kind":"practice_hold","phase_ref":"practice:one","feedback_ref":""}),
    ));
    assert_eq!(invalid["ok"], false);
    assert_eq!(
        world.call(json!({"operation":"read","target":record["target"],"consent":consent()}))["record"],
        record,
        "rejected actions do not mutate native state"
    );
}
