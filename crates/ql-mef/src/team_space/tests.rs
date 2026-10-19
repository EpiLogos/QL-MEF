use super::*;
use crate::cprime_oikonomia::CfpThreadForm;
use crate::vak_profile::InquiryDirection;

const TEAMS_DOC: &str =
    include_str!("../../../../docs/integrations/epi-logos/ANIMA-ALETHEIA-TEAMS.md");
const L5_LOCK: &str = include_str!("../../../../docs/L5-TECHNE-DUAL-READING-LOCK.md");
const SOURCE: &str = include_str!("../team_space.rs");

fn space() -> TeamSpace {
    team_space()
}

fn skill(name: &str) -> &'static str {
    skill_text(name).expect("skill is compiled in")
}

fn row<'a>(space: &'a TeamSpace, code: &str) -> &'a Row {
    space.rows.iter().find(|r| r.frame == code).unwrap()
}

fn ids(head: &Value) -> Vec<String> {
    head["labels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["id"].as_str().unwrap().to_string())
        .collect()
}

fn head_of<'a>(space: &'a TeamSpace, instrument: &str, head: &str) -> &'a Value {
    space
        .instrument(instrument)
        .unwrap()
        .heads
        .iter()
        .find(|h| h["id"] == head)
        .unwrap()
}

#[test]
fn the_space_has_a_stable_contract_and_validates() {
    let space = space();
    assert_eq!(space.contract, "ql.vak-team-space/v1");
    space.validate().unwrap();
    assert_eq!(space.organs.len(), 2);
}

#[test]
fn every_frame_row_has_its_voice_and_member() {
    let space = space();
    assert_eq!(space.rows.len(), 7);
    let expected: [(&str, &str, &[&str]); 7] = [
        ("CF1", "Nous", &["Anansi"]),
        ("CF2", "Logos", &["Janus"]),
        ("CF3", "Eros", &["Moirai"]),
        ("CF4", "Mythos", &["Mercurius"]),
        ("CF5", "Anima", &["Moirai"]),
        ("CF6", "Psyche", &["Agora"]),
        ("CF7", "Sophia", &["Zeithoven", "Moirai"]),
    ];
    for (row, (code, voice, members)) in space.rows.iter().zip(expected) {
        assert_eq!(row.frame, code);
        assert_eq!(row.anima.voice, voice);
        let names: Vec<_> = row.aletheia.iter().map(|s| s.specialist).collect();
        let mut sorted_names = names.clone();
        sorted_names.sort();
        let mut sorted_members = members.to_vec();
        sorted_members.sort();
        assert_eq!(sorted_names, sorted_members, "{code}");
    }
    let frames: Vec<_> = ContextFrameId::ALL.iter().map(|f| f.code()).collect();
    assert_eq!(
        space.rows.iter().map(|r| r.frame).collect::<Vec<_>>(),
        frames
    );
    for (row, frame) in space.rows.iter().zip(ContextFrameId::ALL) {
        assert_eq!(row.anima.voice, constitutional_voice(frame));
        assert_eq!(row.expression, frame.expression());
        assert_eq!(row.name, frame.name());
    }
    // Moirai's three modes are distributed by the thought each answers; Agora
    // sits at CF6 and fuses into CF5.
    let modes: Vec<_> = ["CF3", "CF5", "CF7"]
        .iter()
        .map(|c| row(&space, c).aletheia.iter().find_map(|s| s.mode).unwrap())
        .collect();
    assert_eq!(modes, ["Klotho", "Lachesis", "Atropos"]);
    assert_eq!(row(&space, "CF6").aletheia[0].joins_into, Some("CF5"));
    let members: BTreeSet<_> = space
        .rows
        .iter()
        .flat_map(|r| r.aletheia.iter().map(|s| s.specialist))
        .collect();
    assert_eq!(
        members,
        Specialist::ALL
            .iter()
            .map(|s| s.name())
            .collect::<BTreeSet<_>>()
    );
    for r in &space.rows {
        for seat in &r.aletheia {
            let specialist = Specialist::ALL
                .into_iter()
                .find(|s| s.name() == seat.specialist)
                .unwrap();
            assert_eq!(seat.office, specialist.office());
        }
    }
}

fn roster_line(reference: &str) -> &'static str {
    TEAMS_DOC
        .lines()
        .find(|l| l.starts_with("| `agent/") && l.contains(&format!("`{reference}`")))
        .unwrap_or_else(|| panic!("no roster row for {reference}"))
}

#[test]
fn the_grid_agrees_with_the_published_rosters() {
    let space = space();
    for row in &space.rows {
        let line = roster_line(&row.anima.agent_ref);
        assert!(line.contains(row.frame), "{line}");
        assert!(line.contains(row.expression), "{line}");
    }
    for row in &space.rows {
        for seat in &row.aletheia {
            let line = roster_line(&seat.agent_ref);
            match (seat.specialist, seat.mode) {
                // The roster seats Moirai once, at CF3; the other two modes are
                // this space's distribution by Night′ reading.
                ("Moirai", Some("Klotho")) => assert!(line.contains("CF3")),
                ("Moirai", _) => {}
                ("Agora", _) => {
                    assert!(line.contains("CF6") && line.contains("CF5"), "{line}");
                }
                _ => assert!(line.contains(row.frame), "{line}"),
            }
        }
    }
    for lead in ["agent/anima", "agent/aletheia"] {
        assert!(space.member_refs().contains(lead));
    }
    for position in [ANIMA_PROVING_POSITION, ALETHEIA_PROVING_POSITION] {
        assert!(TEAMS_DOC.contains(position));
    }
}

#[test]
fn day_and_night_questions_agree_with_the_skills() {
    let pass = skill("day-night-pass");
    let frame_skill = skill("vak-coordinate-frame");
    for (position, question, frame) in DAY {
        let key = format!("| {} ", wire(&position));
        let line = pass.lines().find(|l| l.starts_with(&key)).unwrap();
        assert!(line.contains(question), "{line}");
        if let Some(frame) = frame {
            assert!(line.contains(&format!("({})", frame.code())), "{line}");
            assert!(line.contains(constitutional_voice(frame)), "{line}");
        }
    }
    for night in &NIGHT {
        let line = pass
            .lines()
            .find(|l| l.contains(&format!("| {} ", night.prime)))
            .unwrap();
        assert!(line.contains(night.question), "{line}");
        assert!(
            line.contains(&format!("({})", night.frame.code())),
            "{line}"
        );
        if let Some(mode) = night.moirai_mode {
            assert!(line.contains(&format!("Moirai as {mode}")), "{line}");
        }
        assert!(
            pass.contains(&format!(
                "{}→`{}`",
                night.prime.replace('′', "′"),
                night.reading
            )),
            "{}",
            night.prime
        );
        let table = frame_skill
            .lines()
            .find(|l| l.starts_with(&format!("| {} |", wire(&night.position))))
            .unwrap();
        assert!(table.contains(night.question), "{table}");
        let day = DAY.iter().find(|d| d.0 == night.position).unwrap().1;
        assert!(table.contains(day), "{table}");
        assert!(table.contains(night.reading), "{table}");
    }
    for (code, meaning) in T_READINGS.iter().chain(T_PRIME_READINGS.iter()) {
        assert!(
            frame_skill.contains(&format!("{code} {meaning}"))
                || frame_skill.contains(&format!("{} {meaning}", code.replace("-prime", "-prime"))),
            "{code} {meaning}"
        );
    }
    // The space's rows carry the same questions.
    let space = space();
    let cf3 = row(&space, "CF3");
    assert_eq!(cf3.day.as_ref().unwrap().question, "What is being done?");
    assert_eq!(cf3.night.len(), 2);
    assert_eq!(cf3.night[0].answered_by, "Moirai as Klotho");
    assert_eq!(row(&space, "CF1").night[0].reading, "T0");
    assert!(row(&space, "CF5").day.is_none());
    assert!(row(&space, "CF2").night.is_empty());
}

#[test]
fn gates_agree_with_their_skills() {
    let space = space();
    assert_eq!(space.gates.len(), 6);
    for gate in &space.gates[..5] {
        let name = gate.skill.trim_start_matches("skill/ql/");
        let text = skill(name);
        let contact = gate.contact.as_ref().unwrap();
        assert!(
            text.contains(&format!(
                "Primary contact: {} (`{}`)",
                contact.voice, contact.agent_ref
            )),
            "{name}"
        );
        // Gate 1 states the vocabulary as separate results; gates 2–5 as one
        // piped list. Both spell the same four.
        assert!(
            text.contains("aligned | annotated | hold | redirect"),
            "{name} lacks the shared result vocabulary"
        );
        for result in &gate.results {
            assert!(text.contains(result), "{name} {result}");
        }
        assert_eq!(gate.results, ["aligned", "annotated", "hold", "redirect"]);
        assert!(!gate.human_only);
        assert_eq!(
            gate.contact_frame,
            ContextFrameId::ALL
                .iter()
                .find(|f| constitutional_voice(**f) == contact.voice)
                .map(|f| f.code())
        );
    }
    let six = &space.gates[5];
    assert!(six.human_only && six.contact.is_none());
    assert_eq!(six.results, ["aligned", "hold", "redirect"]);
    let text = skill("aletheia-collab-gate");
    assert!(text.contains("Primary contact: the person"));
    assert!(text.contains("Returns aligned, hold or redirect"));
    assert!(!text.contains("annotated"));
    // Each gate's Anima contact is the row it is read at.
    for (code, gate) in [("CF1", 1), ("CF7", 2), ("CF3", 3), ("CF6", 4), ("CF4", 5)] {
        assert_eq!(row(&space, code).gate, Some(gate));
        assert!(
            row(&space, code)
                .instruments
                .contains(&format!("gate-{gate}"))
        );
    }
    assert_eq!(row(&space, "CF2").gate, None);
    assert_eq!(row(&space, "CF5").gate, None);
    assert!(
        row(&space, "CF5")
            .instruments
            .contains(&"vak-block".to_string())
    );
    assert!(
        space
            .rows
            .iter()
            .all(|r| !r.instruments.contains(&"gate-6".to_string()))
    );
    assert_eq!(space.instrument("gate-6").unwrap().cells, ["person"]);
}

#[test]
fn depth_covers_the_six_m_domains() {
    let space = space();
    assert_eq!(space.depth.len(), 6);
    for domain in &space.depth {
        let (position, faculty) = native_faculties()[usize::from(domain.m)];
        assert_eq!(domain.m, position);
        assert_eq!(domain.faculty, faculty);
        assert_eq!(
            domain.native_operations,
            native_operations(position).unwrap()
        );
        assert!(
            L5_LOCK.contains(&format!("M{}′ — {}", domain.m, domain.m_prime)),
            "M{}′ {}",
            domain.m,
            domain.m_prime
        );
        assert!(domain.techne.contains(&format!("M{}′", domain.m)));
    }
    let proving: Vec<_> = space
        .depth
        .iter()
        .filter_map(|d| d.proving_position.map(|p| (d.m, p)))
        .collect();
    assert_eq!(
        proving,
        [(4, ANIMA_PROVING_POSITION), (5, ALETHEIA_PROVING_POSITION)]
    );
}

#[test]
fn the_vak_block_labels_equal_their_enum_sources() {
    let space = space();
    let block = space.instrument("vak-block").unwrap();
    let get = |id: &str| ids(block.heads.iter().find(|h| h["id"] == id).unwrap());
    assert_eq!(get("vak-cpf"), ["dialogical", "authorised-undertaking"]);
    assert_eq!(get("vak-ct").len(), 7);
    assert_eq!(get("vak-cp").len(), 6);
    assert_eq!(get("vak-cf").len(), 7);
    assert_eq!(get("vak-cfp").len(), 7);
    assert_eq!(get("vak-cs").len(), 6);
    assert_eq!(
        get("vak-direction"),
        ["forward-synthesis", "returning-inquiry"]
    );
    assert_eq!(
        get("vak-ct"),
        ContentType::ALL.iter().map(wire).collect::<Vec<_>>()
    );
    assert_eq!(
        get("vak-cf"),
        ContextFrameId::ALL
            .iter()
            .map(|f| f.code().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        get("vak-cs"),
        ContextSequence::ALL.iter().map(wire).collect::<Vec<_>>()
    );
    // The skill spells the same labels.
    let reference = skill("vak-coordinate-frame");
    for id in get("vak-ct") {
        assert!(reference.contains(&id.replace('\'', "′")), "{id}");
    }
    for id in get("vak-cp") {
        assert!(reference.contains(&format!("| {id} |")), "{id}");
    }
    for id in get("vak-cfp") {
        assert!(reference.contains(&id), "{id}");
    }
    for id in get("vak-cs") {
        assert!(reference.contains(&id), "{id}");
    }
    for id in get("vak-direction").into_iter().chain(get("vak-cpf")) {
        assert!(reference.contains(&id), "{id}");
    }
    // CPF, CT, CP, CFP, CS keep their cardinality; CT and direction are multi.
    let card = |id: &str| {
        let h = block.heads.iter().find(|h| h["id"] == id).unwrap();
        (
            h["cardinality"]["min"].as_u64(),
            h["cardinality"]["max"].as_u64(),
        )
    };
    assert_eq!(card("vak-cpf"), (Some(0), Some(1)));
    assert_eq!(card("vak-ct"), (Some(0), Some(7)));
    assert_eq!(card("vak-direction"), (Some(0), Some(2)));
    // The kernel's two C-prime owners agree on the order of the thread forms,
    // the sequences and the directions.
    for (i, form) in ThreadForm::ALL.iter().enumerate() {
        assert_eq!(usize::from(CfpThreadForm::ALL[i].index()), i);
        assert!(CfpThreadForm::ALL[i].marker().contains(&format!("cfp{i}")));
        assert_eq!(wire(form), format!("CFP{i}"));
    }
    for (sequence, profile) in ContextSequence::ALL.iter().zip(CsProfile::ALL) {
        assert_eq!(wire(sequence), format!("CS{}", profile.index()));
        let hops = profile.passage(CsDirection::ForwardSynthesis).hops;
        let pairs = sequence.pairs();
        assert_eq!(hops.len(), pairs.len());
        for (hop, pair) in hops.iter().zip(pairs) {
            assert_eq!(
                wire(&pair.forward),
                format!("4.{}", hop.paired_left.value())
            );
            assert_eq!(
                wire(&pair.returning),
                format!("4.{}", hop.paired_right.value())
            );
        }
    }
    assert_eq!(wire(&InquiryDirection::Forward), "forward");
    assert_eq!(CsDirection::ForwardSynthesis.marker(), "forward-synthesis");
    // The path is the block's seven layers.
    let layers: Vec<_> = space.path.iter().map(|p| (p.layer, p.labels)).collect();
    assert_eq!(
        layers,
        [
            ("CPF", 2),
            ("CT", 7),
            ("CP", 6),
            ("CF", 7),
            ("CFP", 7),
            ("CS", 6),
            ("direction", 2)
        ]
    );
    // Z asks for an authorised undertaking: the constraint is legal and only
    // travels with both of its heads.
    let ask = block.ask(&["vak-cfp"]).unwrap().unwrap();
    assert!(ask.constraints.is_empty());
    let ask = block.ask(&["vak-cfp", "vak-cpf"]).unwrap().unwrap();
    assert_eq!(ask.constraints.len(), 1);
}

#[test]
fn aletheia_outcome_labels_equal_their_skills() {
    let space = space();
    let cases: [(&str, &str, &str, &[&str]); 7] = [
        (
            "gnosis-retrieval",
            "gnosis-grounding",
            "gnosis-retrieve",
            &["grounded", "partial", "empty"],
        ),
        (
            "stack-traverse",
            "stack-layer-verdict",
            "aletheia-stack-traverse",
            &["healthy", "drift", "gap", "unverified"],
        ),
        (
            "plugin-integration",
            "plugin-fit",
            "aletheia-plugin-integrate",
            &["redundant", "complementary", "conflicting"],
        ),
        (
            "plugin-integration",
            "plugin-recommendation",
            "aletheia-plugin-integrate",
            &["adopt", "reference only", "decline"],
        ),
        (
            "improvement-proposal",
            "improvement-form",
            "aletheia-improvement-propose",
            &["implementation", "addendum", "owner-issue", "no-change"],
        ),
        (
            "klein-telemetry",
            "skill-effectiveness",
            "klein-mode",
            &["effective", "suboptimal", "ineffective"],
        ),
        (
            "thought-reading",
            "thought-reading",
            "day-night-pass",
            &["T0", "T5", "T0-prime", "T5-prime"],
        ),
    ];
    for (instrument, head, skill_name, expected) in cases {
        let labels = ids(head_of(&space, instrument, head));
        if instrument == "thought-reading" {
            assert_eq!(labels.len(), 12);
            for e in expected {
                assert!(labels.contains(&e.to_string()));
            }
            assert!(skill("day-night-pass").contains("T0"));
            continue;
        }
        assert_eq!(labels, expected, "{instrument}/{head}");
        let text = skill(skill_name);
        for label in expected {
            assert!(text.contains(label), "{skill_name} lacks {label}");
        }
        // Owner revision is the digest of the skill text compiled in.
        let (_, revision) = skill_owner(skill_name);
        assert_eq!(
            head_of(&space, instrument, head)["labels"][0]["owner_revision"],
            revision
        );
        // Each seat appears in the skill's "Used by" line.
        let used = text.lines().find(|l| l.contains("Used by")).unwrap();
        for seat in &space.instrument(instrument).unwrap().seats {
            assert!(used.contains(&format!("`{seat}`")), "{skill_name}: {seat}");
        }
    }
    // The spread reading is specified, not built, and lives in the teams doc.
    let spread = ids(head_of(
        &space,
        "spread-aliveness",
        "spread-position-aliveness",
    ));
    assert_eq!(spread, ["live", "mute"]);
    assert!(TEAMS_DOC.contains("`live|mute`"));
    assert_eq!(
        space.instrument("klein-telemetry").unwrap().organ,
        "S4′ Anima"
    );
}

#[test]
fn every_head_validates_and_mirrors_the_decision_frame_contract() {
    let space = space();
    for instrument in &space.instruments {
        for head in &instrument.heads {
            validate_head(head).unwrap();
        }
    }
    let mut bad = head_of(&space, "gate-1", "gate-1-reading").clone();
    bad["cardinality"] = json!({"min":0,"max":9});
    assert!(validate_head(&bad).is_err());
    let mut duplicate = head_of(&space, "gate-1", "gate-1-reading").clone();
    duplicate["labels"][1]["id"] = duplicate["labels"][0]["id"].clone();
    assert!(validate_head(&duplicate).is_err());
    let mut policy = head_of(&space, "gate-1", "gate-1-reading").clone();
    policy["ambiguity_policy"] = json!("guess");
    assert!(validate_head(&policy).is_err());
}

/// The reference validator is the contract's checked source; run every
/// instrument's frame through it when Python and jsonschema are present (the
/// repository gate installs both).
#[test]
fn every_instrument_frame_passes_the_reference_validator() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let available = Command::new("python3")
        .args(["-c", "import jsonschema"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !available {
        eprintln!("skipped: python3 with jsonschema is not available");
        return;
    }
    let space = space();
    let payload = serde_json::to_string(
        &space
            .instruments
            .iter()
            .map(|i| json!({"id":i.id,"heads":i.heads,"constraints":i.constraints}))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let script = r#"
import json, sys
sys.path.insert(0, "scripts")
import ql_agent_contracts as c
event = {"schema":"ql.agent-event/v1","event_ref":"ql:event:team-space-probe","generation":0,
  "occasion_refs":[],"source_basis":[{"ref":"ql:probe:team-space","revision":"r1"}],
  "kind":"probe","subject":"team-space","material":{"ref":"ql:material:probe","revision":"r1","text":"probe"},
  "observed":[],"native_state":[],"provenance_refs":["ql:probe"]}
for item in json.load(sys.stdin):
    frame = {"schema":"ql.agent-decision-frame/v1","frame_ref":"ql:frame:" + item["id"],
      "event_ref":event["event_ref"],"event_basis_digest":c.semantic_basis(event),
      "source_basis":event["source_basis"],
      "kernel_basis":{"owner_ref":"ql:owner:probe","revision":"r1","digest":"sha256:" + "0"*64},
      "determined":{"observed":[],"derived":[]},"unresolved":item["heads"],
      "constraints":item["constraints"],"requested_evidence":["semantic-span"]}
    c.validate_frame(event, frame)
print("ok")
"#;
    let mut child = Command::new("python3")
        .current_dir(&root)
        .args(["-c", script])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn model(provider: &str) -> Answerer {
    Answerer::Model {
        provider_ref: provider.into(),
    }
}

fn answer(instrument: &str, head: &str, labels: &[&str], answerer: Answerer) -> Answer {
    Answer {
        instrument: instrument.into(),
        head: head.into(),
        labels: labels.iter().map(|l| l.to_string()).collect(),
        answerer,
    }
}

#[test]
fn gate_six_refuses_every_non_human_answer() {
    let space = space();
    let gate = space.instrument("gate-6").unwrap();
    assert_eq!(gate.answerable_by, Audience::PersonOnly);
    for label in ["aligned", "hold", "redirect"] {
        let refused = space.admit_answer(
            &answer("gate-6", "gate-6-reading", &[label], model("kev:provider")),
            &[],
        );
        assert_eq!(refused, Err(TeamSpaceError::HumanOnly("gate-6".into())));
    }
    // An abstaining model is refused as well: the gate is not a model's to read.
    assert!(matches!(
        space.admit_answer(
            &answer("gate-6", "gate-6-reading", &[], model("kev:provider")),
            &[]
        ),
        Err(TeamSpaceError::HumanOnly(_))
    ));
    let person = space
        .admit_answer(
            &answer("gate-6", "gate-6-reading", &["aligned"], Answerer::Person),
            &[],
        )
        .unwrap();
    assert_eq!(person.standing, "evidence");
    assert!(!person.receipt);
    // Gate 6 has no `annotated` for anyone.
    assert!(matches!(
        space.admit_answer(
            &answer("gate-6", "gate-6-reading", &["annotated"], Answerer::Person),
            &[]
        ),
        Err(TeamSpaceError::IllegalLabel(_))
    ));
}

#[test]
fn a_model_answer_is_evidence_and_never_a_receipt() {
    let space = space();
    let ok = space
        .admit_answer(
            &answer(
                "gate-1",
                "gate-1-reading",
                &["hold"],
                model("kev:decision-provider"),
            ),
            &["agent/anima-eros".into()],
        )
        .unwrap();
    assert_eq!(
        ok,
        AnswerStanding {
            standing: "evidence",
            receipt: false,
            abstention: false
        }
    );
    // Abstention is representable and kept.
    let abstain = space
        .admit_answer(
            &answer(
                "gate-1",
                "gate-1-reading",
                &[],
                model("kev:decision-provider"),
            ),
            &[],
        )
        .unwrap();
    assert!(abstain.abstention && !abstain.receipt);
    // The provider is never one of the acting roster or the team.
    assert_eq!(
        space.admit_answer(
            &answer(
                "gate-1",
                "gate-1-reading",
                &["aligned"],
                model("provider:acting")
            ),
            &["provider:acting".into()],
        ),
        Err(TeamSpaceError::ProviderIsActing("provider:acting".into()))
    );
    assert_eq!(
        space.admit_answer(
            &answer(
                "gate-1",
                "gate-1-reading",
                &["aligned"],
                model("agent/anima-nous")
            ),
            &[],
        ),
        Err(TeamSpaceError::ProviderIsActing("agent/anima-nous".into()))
    );
    // Illegal labels, repeats and too many labels are refused.
    for labels in [&["bogus"][..], &["hold", "hold"][..]] {
        assert!(matches!(
            space.admit_answer(
                &answer("gate-1", "gate-1-reading", labels, Answerer::Person),
                &[]
            ),
            Err(TeamSpaceError::IllegalLabel(_))
        ));
    }
    assert!(matches!(
        space.admit_answer(
            &answer(
                "gate-1",
                "gate-1-reading",
                &["hold", "aligned"],
                Answerer::Person
            ),
            &[]
        ),
        Err(TeamSpaceError::Cardinality { .. })
    ));
    assert!(matches!(
        space.admit_answer(&answer("nope", "x", &[], Answerer::Person), &[]),
        Err(TeamSpaceError::UnknownInstrument(_))
    ));
    // CT is multi-select; direction takes both.
    assert!(
        space
            .admit_answer(
                &answer(
                    "vak-block",
                    "vak-ct",
                    &["CT0", "CT3"],
                    model("kev:decision-provider")
                ),
                &[]
            )
            .is_ok()
    );
}

#[test]
fn zero_heads_ask_for_zero_model_calls() {
    let space = space();
    for instrument in &space.instruments {
        assert!(instrument.ask(&[]).unwrap().is_none(), "{}", instrument.id);
    }
    let block = space.instrument("vak-block").unwrap();
    let ask = block.ask(&["vak-cf", "vak-cs"]).unwrap().unwrap();
    assert_eq!(ask.heads.len(), 2);
    assert_eq!(ask.audience, Audience::ModelOrPerson);
    assert!(matches!(
        block.ask(&["no-such-head"]),
        Err(TeamSpaceError::UnknownHead(_))
    ));
    let gate = space
        .instrument("gate-6")
        .unwrap()
        .ask(&["gate-6-reading"])
        .unwrap()
        .unwrap();
    assert_eq!(gate.audience, Audience::PersonOnly);
}

#[test]
fn thread_forms_compose_instruments_as_they_compose_voices() {
    let space = space();
    assert_eq!(space.thread_forms.len(), 7);
    let expected = [
        ("CFP0", "single-voice", None),
        ("CFP1", "chord", Some("Drive")),
        ("CFP2", "melody", Some("Relay")),
        ("CFP3", "fusion", Some("Scout")),
        ("CFP4", "drone", Some("Watch")),
        ("CFP5", "canon", Some("Arm")),
        ("Z", "cycle", Some("Metamorph")),
    ];
    for (composition, (form, role, pattern)) in space.thread_forms.iter().zip(expected) {
        assert_eq!(composition.thread_form, form);
        assert_eq!(composition.musical_role, role);
        assert_eq!(composition.pattern, pattern);
    }
    let fusion = &space.thread_forms[3];
    assert_eq!(fusion.joined_by, Some("Agora"));
    assert_eq!(fusion.contract.as_ref().unwrap().join, "Weighted");
    let mechanisms: Vec<_> = space.thread_forms.iter().map(|t| t.mechanism).collect();
    assert_eq!(
        mechanisms,
        [
            "none",
            "mechanism-true",
            "mechanism-true",
            "partial",
            "partial",
            "partial",
            "partial"
        ]
    );
    // Each thread form's label is in the block's CFP head.
    let cfp = ids(head_of(&space, "vak-block", "vak-cfp"));
    for composition in &space.thread_forms {
        assert!(cfp.contains(&composition.thread_form));
    }
}

#[test]
fn the_binding_rules_are_all_stated() {
    let space = space();
    let rules: Vec<_> = space.rules.iter().map(|r| r.id).collect();
    for required in [
        "zero-heads-zero-calls",
        "no-inference-on-entry-paths",
        "answer-is-evidence",
        "abstention-representable",
        "not-a-closure-referee",
        "not-a-recognition-engine",
        "provider-not-from-acting-roster",
        "gate-6-is-the-person",
        "engine-validates",
    ] {
        assert!(rules.contains(&required), "{required}");
    }
    // No instrument decides closure or Recognition.
    for instrument in &space.instruments {
        let mut names = vec![instrument.id.clone()];
        for head in &instrument.heads {
            names.push(head["id"].as_str().unwrap().to_string());
            names.push(head["field"].as_str().unwrap().to_string());
            names.extend(ids(head));
        }
        let text = names.join(" ").to_lowercase();
        assert!(
            !text.contains("closure") && !text.contains("recognition"),
            "{}",
            instrument.id
        );
    }
}

#[test]
fn the_space_serialises_and_round_trips() {
    let space = space();
    let value = serde_json::to_value(&space).unwrap();
    let text = serde_json::to_string_pretty(&value).unwrap();
    assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), value);
    assert_eq!(value["contract"], "ql.vak-team-space/v1");
    assert_eq!(value["rows"].as_array().unwrap().len(), 7);
    assert_eq!(value["depth"].as_array().unwrap().len(), 6);
    assert_eq!(value["gates"][5]["human_only"], true);
    assert_eq!(value["instruments"][0]["heads"][0]["id"], "vak-cpf");
    assert_eq!(
        space
            .render_text()
            .lines()
            .filter(|l| l.starts_with("CF"))
            .count(),
        7
    );
}

#[test]
fn no_code_path_calls_a_model_or_any_service() {
    for forbidden in [
        "std::process",
        "std::net",
        "TcpStream",
        "reqwest",
        "ureq",
        "tokio",
        "Command::new",
        "std::fs",
        "std::env",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "team_space.rs mentions {forbidden}"
        );
    }
}
