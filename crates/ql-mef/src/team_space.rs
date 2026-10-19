//! The team space: the full agentic side of the Vāk system.
//!
//! Anima (S4′) composes a situated act through the C′ Vāk grammar; Aletheia
//! (S5′) discloses what happened and returns it. Their members work inside the
//! seven context frames. This module is the grid those members stand in, the
//! depth it is situated at, the path an act's Vāk block takes through it, and
//! the typed decision instruments a member may put to a small decision model.
//!
//! It is a read-only projection. Every value that a QL owner already holds is
//! derived from that owner (`ContextFrameId`, `constitutional_voice`, the C′
//! enums, `Specialist`, `CsProfile`, `native_faculties`). A value held only in
//! a skill or document is held here once and checked against that text by the
//! tests. Nothing here calls a model, a provider or any other service, and a
//! decision-instrument answer is evidence that the engine validates, never a
//! receipt.
use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::cprime_oikonomia::{CfpZStage, CsDirection, CsProfile};
use crate::epi_agent::{native_faculties, native_operations};
use crate::vak_profile::{
    ContentPosition, ContentType, ContextSequence, PROFILE_CONTRACT, Participation, ThreadForm,
    constitutional_voice,
};
use crate::vak_workflow_types::Specialist;
use crate::{CONTEXT_FRAME_GRAMMAR_VERSION, ContextFrameId};

/// Stable contract id of the serialised team space.
pub const TEAM_SPACE_CONTRACT: &str = "ql.vak-team-space/v1";
/// The proving Positions of the first two situated roles (L5 lock §15).
pub const ANIMA_PROVING_POSITION: &str = "central:position:project:O-I:anima-4";
pub const ALETHEIA_PROVING_POSITION: &str = "central:position:project:O-I:aletheia-5";

const ANIMA_ORGAN: &str = "S4′ Anima";
const ALETHEIA_ORGAN: &str = "S5′ Aletheia";
const ANIMA_LEAD: &str = "agent/anima";
const ALETHEIA_LEAD: &str = "agent/aletheia";

// ---------------------------------------------------------------------------
// Values held once here because only a skill or document holds them.
// ---------------------------------------------------------------------------

/// Day questions by CP, with the frame whose voice usually asks them
/// (`skills/day-night-pass`, "Day pass"). CP 4.0 is asked by "the frame
/// assigned by orchestration", so it has no fixed frame.
const DAY: [(ContentPosition, &str, Option<ContextFrameId>); 6] = [
    (ContentPosition::Ground, "What do we have?", None),
    (
        ContentPosition::Definition,
        "What must be true?",
        Some(ContextFrameId::Cf2),
    ),
    (
        ContentPosition::Operation,
        "What is being done?",
        Some(ContextFrameId::Cf3),
    ),
    (
        ContentPosition::Pattern,
        "What shape does it take?",
        Some(ContextFrameId::Cf4),
    ),
    (
        ContentPosition::Context,
        "Where and when in the larger frame?",
        Some(ContextFrameId::Cf6),
    ),
    (
        ContentPosition::Integration,
        "What was produced?",
        Some(ContextFrameId::Cf7),
    ),
];

struct Night {
    position: ContentPosition,
    prime: &'static str,
    question: &'static str,
    reading: &'static str,
    frame: ContextFrameId,
    /// Moirai's mode when Moirai, not the frame's voice, answers.
    moirai_mode: Option<&'static str>,
}

/// Night′ questions (`skills/day-night-pass`, "Night′ pass"), in CP order.
const NIGHT: [Night; 6] = [
    Night {
        position: ContentPosition::Ground,
        prime: "P0′",
        question: "What don't we know?",
        reading: "T0",
        frame: ContextFrameId::Cf1,
        moirai_mode: None,
    },
    Night {
        position: ContentPosition::Definition,
        prime: "P1′",
        question: "What evidence exists?",
        reading: "T1",
        frame: ContextFrameId::Cf3,
        moirai_mode: Some("Klotho"),
    },
    Night {
        position: ContentPosition::Operation,
        prime: "P2′",
        question: "What blocks us?",
        reading: "T2",
        frame: ContextFrameId::Cf3,
        moirai_mode: None,
    },
    Night {
        position: ContentPosition::Pattern,
        prime: "P3′",
        question: "What repeats?",
        reading: "T3",
        frame: ContextFrameId::Cf4,
        moirai_mode: None,
    },
    Night {
        position: ContentPosition::Context,
        prime: "P4′",
        question: "What sources inform?",
        reading: "T4",
        frame: ContextFrameId::Cf5,
        moirai_mode: Some("Lachesis"),
    },
    Night {
        position: ContentPosition::Integration,
        prime: "P5′",
        question: "What crystallises?",
        reading: "T5",
        frame: ContextFrameId::Cf7,
        moirai_mode: Some("Atropos"),
    },
];

/// Specialists other than Moirai, with their frame and, for Agora, the frame
/// it fuses into (`ANIMA-ALETHEIA-TEAMS.md` §2). Moirai's seats come from
/// [`NIGHT`]: one actor in three modes, distributed by the thought it answers.
const SEATS: [(Specialist, ContextFrameId, Option<ContextFrameId>); 5] = [
    (Specialist::Anansi, ContextFrameId::Cf1, None),
    (Specialist::Janus, ContextFrameId::Cf2, None),
    (Specialist::Mercurius, ContextFrameId::Cf4, None),
    (
        Specialist::Agora,
        ContextFrameId::Cf6,
        Some(ContextFrameId::Cf5),
    ),
    (Specialist::Zeithoven, ContextFrameId::Cf7, None),
];

/// The six gates, with the frame of the voice that is the gate's primary
/// contact (each gate skill's "Primary contact"). Gate 6 is always the person.
const GATES: [(u8, &str, &str, Option<ContextFrameId>); 6] = [
    (1, "aletheia-ql-gate", "QL", Some(ContextFrameId::Cf1)),
    (2, "aletheia-m-gate", "M", Some(ContextFrameId::Cf7)),
    (3, "aletheia-s-gate", "S", Some(ContextFrameId::Cf3)),
    (4, "aletheia-m-prime-gate", "M′", Some(ContextFrameId::Cf6)),
    (5, "aletheia-rupa-gate", "Rupa", Some(ContextFrameId::Cf4)),
    (6, "aletheia-collab-gate", "Collaboration", None),
];

/// Thought meanings returned by Night′ answers (`skills/vak-coordinate-frame`).
const T_READINGS: [(&str, &str); 6] = [
    ("T0", "Question"),
    ("T1", "Trace"),
    ("T2", "Challenge"),
    ("T3", "Pattern"),
    ("T4", "Discovery"),
    ("T5", "Insight"),
];
const T_PRIME_READINGS: [(&str, &str); 6] = [
    ("T0-prime", "Assumption"),
    ("T1-prime", "Lacuna"),
    ("T2-prime", "Affordance"),
    ("T3-prime", "Anomaly"),
    ("T4-prime", "Concealment"),
    ("T5-prime", "Integration"),
];

/// The M_i′ deep instruments (`docs/L5-TECHNE-DUAL-READING-LOCK.md` §6–§11).
const M_PRIME_INSTRUMENTS: [&str; 6] = [
    "Project / Wiki / Graph",
    "Canvas / Constellation",
    "Relation Field / Timeline",
    "Journey / Scenes",
    "Places / World",
    "Palace",
];

// ---------------------------------------------------------------------------
// The serialised shape.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct TeamSpace {
    pub contract: &'static str,
    pub organs: Vec<Organ>,
    pub rows: Vec<Row>,
    pub gates: Vec<Gate>,
    pub depth: Vec<DepthDomain>,
    pub path: Vec<PathLayer>,
    pub instruments: Vec<Instrument>,
    pub thread_forms: Vec<ThreadComposition>,
    pub rules: Vec<Rule>,
    pub sources: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Organ {
    pub organ: &'static str,
    pub lead: &'static str,
    pub office: &'static str,
    pub proving_position: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Row {
    pub frame: &'static str,
    pub expression: &'static str,
    pub name: &'static str,
    pub anima: Voice,
    pub aletheia: Vec<Seat>,
    pub day: Option<DayQuestion>,
    pub night: Vec<NightQuestion>,
    pub gate: Option<u8>,
    pub instruments: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Voice {
    pub voice: &'static str,
    pub agent_ref: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Seat {
    pub specialist: &'static str,
    pub agent_ref: String,
    pub mode: Option<&'static str>,
    pub office: &'static str,
    pub joins_into: Option<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DayQuestion {
    pub position: String,
    pub question: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct NightQuestion {
    pub position: String,
    pub prime: &'static str,
    pub question: &'static str,
    pub reading: &'static str,
    pub answered_by: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Gate {
    pub number: u8,
    pub name: &'static str,
    pub skill: String,
    pub contact: Option<Voice>,
    pub contact_frame: Option<&'static str>,
    pub human_only: bool,
    pub results: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DepthDomain {
    pub m: u8,
    pub faculty: &'static str,
    pub m_prime: &'static str,
    pub anima: String,
    pub aletheia: String,
    pub techne: String,
    pub native_operations: Vec<&'static str>,
    pub proving_position: Option<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PathLayer {
    pub layer: &'static str,
    pub head: String,
    pub field: String,
    pub labels: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Audience {
    ModelOrPerson,
    PersonOnly,
}

#[derive(Debug, Clone, Serialize)]
pub struct Instrument {
    pub id: String,
    pub organ: &'static str,
    pub kind: &'static str,
    pub source: String,
    pub answerable_by: Audience,
    pub seats: Vec<String>,
    pub cells: Vec<String>,
    pub heads: Vec<Value>,
    pub constraints: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ThreadComposition {
    pub thread_form: String,
    pub musical_role: &'static str,
    pub pattern: Option<&'static str>,
    pub family: Option<&'static str>,
    pub joined_by: Option<&'static str>,
    pub contract: Option<PatternContract>,
    pub mechanism: &'static str,
    pub note: &'static str,
}

/// Fields of the thread contract (`face-rig/CREATURE-SPEC.md` §21) for the
/// pattern that stands under a thread form.
#[derive(Debug, Clone, Serialize)]
pub struct PatternContract {
    pub start: &'static str,
    pub stay: &'static str,
    pub count: &'static str,
    pub reach: &'static str,
    pub link: &'static str,
    pub join: &'static str,
    pub change: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Rule {
    pub id: &'static str,
    pub text: &'static str,
    pub source: &'static str,
}

// ---------------------------------------------------------------------------
// Derivation helpers.
// ---------------------------------------------------------------------------

fn wire<T: Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(Value::String(text)) => text,
        _ => unreachable!("closed C-prime enums serialise to one wire string"),
    }
}

fn anima_ref(frame: ContextFrameId) -> String {
    match frame {
        ContextFrameId::Cf5 => ANIMA_LEAD.to_string(),
        _ => format!("agent/anima-{}", constitutional_voice(frame).to_lowercase()),
    }
}

fn aletheia_ref(specialist: Specialist) -> String {
    format!("agent/aletheia-{}", specialist.name().to_lowercase())
}

fn voice(frame: ContextFrameId) -> Voice {
    Voice {
        voice: constitutional_voice(frame),
        agent_ref: anima_ref(frame),
    }
}

fn skill_text(name: &str) -> Option<&'static str> {
    Some(match name {
        "aletheia-ql-gate" => include_str!("../../../skills/aletheia-ql-gate/SKILL.md"),
        "aletheia-m-gate" => include_str!("../../../skills/aletheia-m-gate/SKILL.md"),
        "aletheia-s-gate" => include_str!("../../../skills/aletheia-s-gate/SKILL.md"),
        "aletheia-m-prime-gate" => include_str!("../../../skills/aletheia-m-prime-gate/SKILL.md"),
        "aletheia-rupa-gate" => include_str!("../../../skills/aletheia-rupa-gate/SKILL.md"),
        "aletheia-collab-gate" => include_str!("../../../skills/aletheia-collab-gate/SKILL.md"),
        "gnosis-retrieve" => include_str!("../../../skills/gnosis-retrieve/SKILL.md"),
        "aletheia-stack-traverse" => {
            include_str!("../../../skills/aletheia-stack-traverse/SKILL.md")
        }
        "aletheia-plugin-integrate" => {
            include_str!("../../../skills/aletheia-plugin-integrate/SKILL.md")
        }
        "aletheia-improvement-propose" => {
            include_str!("../../../skills/aletheia-improvement-propose/SKILL.md")
        }
        "klein-mode" => include_str!("../../../skills/klein-mode/SKILL.md"),
        "day-night-pass" => include_str!("../../../skills/day-night-pass/SKILL.md"),
        "vak-coordinate-frame" => include_str!("../../../skills/vak-coordinate-frame/SKILL.md"),
        "vak-evaluate" => include_str!("../../../skills/vak-evaluate/SKILL.md"),
        _ => return None,
    })
}

/// The owner reference and revision of a skill: the revision is the digest of
/// the skill text compiled in, so a changed skill is visibly a changed owner.
fn skill_owner(name: &str) -> (String, String) {
    let text = skill_text(name).unwrap_or_default();
    (
        format!("ql:skill:{name}"),
        format!("sha256:{:x}", Sha256::digest(text.as_bytes())),
    )
}

fn label(id: &str, description: &str, owner_ref: &str, owner_revision: &str) -> Value {
    json!({"id":id,"description":description,"owner_ref":owner_ref,"owner_revision":owner_revision})
}

fn head(id: &str, field: &str, labels: Vec<Value>, min: usize, max: usize, policy: &str) -> Value {
    json!({"id":id,"field":field,"labels":labels,
        "cardinality":{"min":min,"max":max},"ambiguity_policy":policy})
}

/// A single-choice head: empty selection is the representable abstention.
fn choice(id: &str, field: &str, labels: Vec<Value>) -> Value {
    head(id, field, labels, 0, 1, "abstain")
}

fn skill_labels(skill: &str, pairs: &[(&str, &str)]) -> Vec<Value> {
    let (owner, revision) = skill_owner(skill);
    pairs
        .iter()
        .map(|(id, description)| label(id, description, &owner, &revision))
        .collect()
}

fn night_answerer(night: &Night) -> String {
    match night.moirai_mode {
        Some(mode) => format!("Moirai as {mode}"),
        None => constitutional_voice(night.frame).to_string(),
    }
}

fn thread_label_id(form: ThreadForm) -> String {
    wire(&form)
}

// ---------------------------------------------------------------------------
// The decision instruments.
// ---------------------------------------------------------------------------

fn vak_block_instrument() -> Instrument {
    let kernel = |layer: &str| format!("ql:mef:vak-profile:{layer}");

    let cpf: Vec<Value> = [
        Participation::Dialogical,
        Participation::AuthorisedUndertaking,
    ]
    .iter()
    .map(|p| {
        let note = if p.requires_undertaking() {
            "an admitted undertaking; it names its authority"
        } else {
            "dialogue with the person; the person completes the remaining layers"
        };
        label(&wire(p), note, &kernel("participation"), PROFILE_CONTRACT)
    })
    .collect();
    let ct: Vec<Value> = ContentType::ALL
        .iter()
        .map(|c| {
            label(
                &wire(c),
                &format!("{c:?} content"),
                &kernel("content-type"),
                PROFILE_CONTRACT,
            )
        })
        .collect();
    let cp: Vec<Value> = ContentPosition::ALL
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let (_, day, _) = DAY[i];
            let night = NIGHT[i].question;
            label(
                &wire(p),
                &format!("{p:?}: Day {day} Night′ {night}"),
                &kernel("position"),
                PROFILE_CONTRACT,
            )
        })
        .collect();
    let cf: Vec<Value> = ContextFrameId::ALL
        .iter()
        .map(|f| {
            label(
                f.code(),
                &format!(
                    "{} {} ({})",
                    f.expression(),
                    f.name(),
                    constitutional_voice(*f)
                ),
                &format!(
                    "ql:mef:context-frame:{CONTEXT_FRAME_GRAMMAR_VERSION}:{}",
                    f.code()
                ),
                CONTEXT_FRAME_GRAMMAR_VERSION,
            )
        })
        .collect();
    let mut cfp: Vec<Value> = ThreadForm::ALL
        .iter()
        .map(|t| {
            label(
                &thread_label_id(*t),
                t.musical_role(),
                &kernel("thread-form"),
                PROFILE_CONTRACT,
            )
        })
        .collect();
    let z_stages = CfpZStage::ORDER
        .iter()
        .map(|s| s.marker().rsplit(':').next().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" → ");
    cfp.push(label(
        "Z",
        &format!("the governed cycle {z_stages}; it needs an authorised undertaking"),
        "ql:mef:cprime-oikonomia:z",
        crate::cprime_oikonomia::C_PRIME_OIKONOMIA_CONTRACT,
    ));
    let cs: Vec<Value> = ContextSequence::ALL
        .iter()
        .map(|s| {
            let pairs = s
                .pairs()
                .iter()
                .map(|p| format!("{}↔{}", wire(&p.forward), wire(&p.returning)))
                .collect::<Vec<_>>()
                .join(" · ");
            label(&wire(s), &pairs, &kernel("sequence"), PROFILE_CONTRACT)
        })
        .collect();
    let direction: Vec<Value> = [CsDirection::ForwardSynthesis, CsDirection::ReturningInquiry]
        .iter()
        .map(|d| {
            let meaning = match d {
                CsDirection::ForwardSynthesis => "Day: the pass forward through the pairs",
                CsDirection::ReturningInquiry => "Night′: the pass back through the pairs",
            };
            label(
                d.marker(),
                meaning,
                "ql:mef:cprime-oikonomia:direction",
                crate::cprime_oikonomia::C_PRIME_OIKONOMIA_CONTRACT,
            )
        })
        .collect();

    let heads = vec![
        choice("vak-cpf", "participation", cpf),
        head(
            "vak-ct",
            "content-type",
            ct.clone(),
            0,
            ct.len(),
            "preserve-candidates",
        ),
        choice("vak-cp", "position", cp),
        choice("vak-cf", "context-frame", cf),
        choice("vak-cfp", "thread-form", cfp),
        choice("vak-cs", "sequence", cs),
        // `vak-evaluate` allows both directions in one act.
        head(
            "vak-direction",
            "direction",
            direction,
            0,
            2,
            "preserve-candidates",
        ),
    ];
    let z_needs_authority = json!({"kind":"implication","id":"vak-z-needs-authorised-undertaking",
        "rule_ref":"skill/ql/vak-evaluate:cfp-z",
        "antecedent":[{"head_id":"vak-cfp","label_ids":["Z"],"match":"all"}],
        "consequent":[{"head_id":"vak-cpf","label_ids":["authorised-undertaking"],"match":"all"}]});
    Instrument {
        id: "vak-block".into(),
        organ: ANIMA_ORGAN,
        kind: "vak-block",
        source: "ql:skill:vak-evaluate".into(),
        answerable_by: Audience::ModelOrPerson,
        seats: vec![ANIMA_LEAD.into()],
        cells: vec!["CF5".into()],
        heads,
        constraints: vec![z_needs_authority],
    }
}

fn gate_instruments() -> Vec<Instrument> {
    let standard = [
        ("aligned", "all checks pass"),
        ("annotated", "passes with named, non-blocking notes"),
        ("hold", "a check fails that the producer can fix"),
        ("redirect", "the artifact belongs to another gate or owner"),
    ];
    let person = [
        (
            "aligned",
            "the person approved and the approval is recorded",
        ),
        ("hold", "waiting for the person, or declined"),
        (
            "redirect",
            "the change belongs to a different owner's decision",
        ),
    ];
    GATES
        .iter()
        .map(|&(number, skill, _, contact)| {
            let human = contact.is_none();
            let labels = skill_labels(skill, if human { &person[..] } else { &standard[..] });
            Instrument {
                id: format!("gate-{number}"),
                organ: ALETHEIA_ORGAN,
                kind: "gate-reading",
                source: format!("ql:skill:{skill}"),
                answerable_by: if human {
                    Audience::PersonOnly
                } else {
                    Audience::ModelOrPerson
                },
                seats: vec![ALETHEIA_LEAD.into()],
                cells: vec![match contact {
                    Some(frame) => frame.code().into(),
                    None => "person".into(),
                }],
                heads: vec![choice(
                    &format!("gate-{number}-reading"),
                    "gate-reading",
                    labels,
                )],
                constraints: vec![],
            }
        })
        .collect()
}

fn thought_reading_instrument() -> Instrument {
    let (owner, revision) = skill_owner("vak-coordinate-frame");
    let labels: Vec<Value> = T_READINGS
        .iter()
        .chain(T_PRIME_READINGS.iter())
        .map(|(id, meaning)| label(id, meaning, &owner, &revision))
        .collect();
    let mut cells: Vec<String> = Vec::new();
    for night in &NIGHT {
        let code = night.frame.code().to_string();
        if !cells.contains(&code) {
            cells.push(code);
        }
    }
    Instrument {
        id: "thought-reading".into(),
        organ: ALETHEIA_ORGAN,
        kind: "thought-reading",
        source: "ql:skill:day-night-pass".into(),
        answerable_by: Audience::ModelOrPerson,
        seats: vec![aletheia_ref(Specialist::Moirai)],
        cells,
        heads: vec![choice("thought-reading", "thought-reading", labels)],
        constraints: vec![],
    }
}

/// A typed outcome of an Aletheia or Anima skill: (instrument id, organ, skill,
/// seats, and its heads as (head id, field, labels)).
type OutcomeHead<'a> = (&'a str, &'a str, &'a [(&'a str, &'a str)]);

fn outcome_instrument(
    id: &str,
    organ: &'static str,
    skill: &str,
    seats: &[&str],
    heads: &[OutcomeHead<'_>],
) -> Instrument {
    Instrument {
        id: id.into(),
        organ,
        kind: "typed-outcome",
        source: format!("ql:skill:{skill}"),
        answerable_by: Audience::ModelOrPerson,
        seats: seats.iter().map(|s| s.to_string()).collect(),
        cells: vec![],
        heads: heads
            .iter()
            .map(|(head_id, field, labels)| choice(head_id, field, skill_labels(skill, labels)))
            .collect(),
        constraints: vec![],
    }
}

fn outcome_instruments() -> Vec<Instrument> {
    let a = |s: Specialist| aletheia_ref(s);
    let nous = anima_ref(ContextFrameId::Cf1);
    let sophia = anima_ref(ContextFrameId::Cf7);
    let mut list = vec![
        outcome_instrument(
            "gnosis-retrieval",
            ALETHEIA_ORGAN,
            "gnosis-retrieve",
            &[
                &nous,
                &a(Specialist::Anansi),
                &a(Specialist::Moirai),
                ALETHEIA_LEAD,
                &a(Specialist::Agora),
                &a(Specialist::Mercurius),
            ],
            &[(
                "gnosis-grounding",
                "grounding",
                &[
                    ("grounded", "every claim is cited"),
                    ("partial", "some claims have no source; name them"),
                    ("empty", "nothing in this scope"),
                ],
            )],
        ),
        outcome_instrument(
            "stack-traverse",
            ALETHEIA_ORGAN,
            "aletheia-stack-traverse",
            &[ALETHEIA_LEAD, &a(Specialist::Janus), &a(Specialist::Anansi)],
            &[(
                "stack-layer-verdict",
                "layer-verdict",
                &[
                    ("healthy", "intended role, current state and seams agree"),
                    ("drift", "the layer has moved from its intended role"),
                    ("gap", "a seam or part is missing"),
                    ("unverified", "only read, no command was run"),
                ],
            )],
        ),
        outcome_instrument(
            "plugin-integration",
            ALETHEIA_ORGAN,
            "aletheia-plugin-integrate",
            &[&a(Specialist::Agora), ALETHEIA_LEAD],
            &[
                (
                    "plugin-fit",
                    "fit",
                    &[
                        ("redundant", "existing coverage already does this"),
                        ("complementary", "it adds to what exists"),
                        ("conflicting", "it contends with what exists"),
                    ],
                ),
                (
                    "plugin-recommendation",
                    "recommendation",
                    &[
                        (
                            "adopt",
                            "adopt through the owner's path, after the collaboration gate",
                        ),
                        ("reference only", "keep as a reference pattern"),
                        ("decline", "do not take it in"),
                    ],
                ),
            ],
        ),
        outcome_instrument(
            "improvement-proposal",
            ALETHEIA_ORGAN,
            "aletheia-improvement-propose",
            &[
                ALETHEIA_LEAD,
                &a(Specialist::Mercurius),
                &a(Specialist::Zeithoven),
            ],
            &[(
                "improvement-form",
                "form",
                &[
                    ("implementation", "an implementation proposal"),
                    ("addendum", "a plan or specification addendum"),
                    ("owner-issue", "an owner-ready issue"),
                    ("no-change", "the gap is not yet justified"),
                ],
            )],
        ),
        outcome_instrument(
            "klein-telemetry",
            ANIMA_ORGAN,
            "klein-mode",
            &[&sophia, ANIMA_LEAD],
            &[(
                "skill-effectiveness",
                "skill-effectiveness",
                &[
                    ("effective", "the skill served the act"),
                    ("suboptimal", "the skill served the act poorly"),
                    ("ineffective", "the skill did not serve the act"),
                ],
            )],
        ),
    ];
    // Janus's live/mute reading is specified in ANIMA-ALETHEIA-TEAMS.md (spec
    // C) and not yet built; it has no skill, so its owner is this contract.
    let janus_labels = vec![
        label(
            "live",
            "the open spread position is still alive",
            "ql:doc:anima-aletheia-teams:spec-c",
            TEAM_SPACE_CONTRACT,
        ),
        label(
            "mute",
            "the open spread position has gone quiet",
            "ql:doc:anima-aletheia-teams:spec-c",
            TEAM_SPACE_CONTRACT,
        ),
    ];
    list.push(Instrument {
        id: "spread-aliveness".into(),
        organ: ALETHEIA_ORGAN,
        kind: "typed-outcome",
        source: "ANIMA-ALETHEIA-TEAMS.md spec C (specified, not built)".into(),
        answerable_by: Audience::ModelOrPerson,
        seats: vec![a(Specialist::Janus)],
        cells: vec![],
        heads: vec![choice(
            "spread-position-aliveness",
            "aliveness",
            janus_labels,
        )],
        constraints: vec![],
    });
    list
}

// ---------------------------------------------------------------------------
// Thread forms compose instruments as they compose voices.
// ---------------------------------------------------------------------------

fn thread_compositions() -> Vec<ThreadComposition> {
    let contract = |start, stay, count, reach, link, join, change| {
        Some(PatternContract {
            start,
            stay,
            count,
            reach,
            link,
            join,
            change,
        })
    };
    ThreadForm::ALL
        .iter()
        .map(|form| {
            let base = |pattern, family, joined_by, contract, mechanism, note| ThreadComposition {
                thread_form: thread_label_id(*form),
                musical_role: form.musical_role(),
                pattern,
                family,
                joined_by,
                contract,
                mechanism,
                note,
            };
            match form {
                ThreadForm::Single => base(
                    None,
                    None,
                    None,
                    None,
                    "none",
                    "one voice puts its head to one instrument; no pattern stands under it",
                ),
                ThreadForm::Parallel => base(
                    Some("Drive"),
                    Some("Coordinate"),
                    None,
                    contract("Called", "Task", "Crew", "Write", "Peer", "All", "Same"),
                    "mechanism-true",
                    "each voice holds a fixed position in one hunt, as each instrument holds its own head; the instruments answer, they do not write, so Reach is Read here",
                ),
                ThreadForm::Chain => base(
                    Some("Relay"),
                    Some("Carry"),
                    None,
                    contract("Spawned", "Task", "Crew", "Write", "Parent", "None", "Same"),
                    "mechanism-true",
                    "each leg ends and hands its state on; the next instrument is asked only what the previous return left unresolved",
                ),
                ThreadForm::Fusion => base(
                    Some("Scout"),
                    Some("Sense"),
                    Some("Agora"),
                    contract("Spawned", "Condition", "Crew", "Read", "Parent", "Weighted", "Same"),
                    "partial",
                    "scouts read and report and the join is weighted, as Agora weighs plural readings; Quorum, which commits at a count, is used only where Agora's aggregation is a count, since fusion by independent examination has no threshold",
                ),
                ThreadForm::Sustained => base(
                    Some("Watch"),
                    Some("Sense"),
                    None,
                    contract("Standing", "Standing", "One", "Read", "Place", "None", "Same"),
                    "partial",
                    "a drone is a held tone; Watch stands and reads and raises an alarm. CFP4 has a stop contract and writes, so Stay is Condition in the engine and the instrument is only the standing reader",
                ),
                ThreadForm::Nested => base(
                    Some("Arm"),
                    Some("Coordinate"),
                    None,
                    contract("Spawned", "Task", "Crew", "Write", "Parent", "All", "Same"),
                    "partial",
                    "an arm takes a goal and decides the detail locally, as a nested composition does; Arm has no canon's imitation of the outer theme",
                ),
            }
        })
        .chain(std::iter::once(ThreadComposition {
            thread_form: "Z".into(),
            musical_role: "cycle",
            pattern: Some("Metamorph"),
            family: Some("Change"),
            joined_by: None,
            contract: contract("Triggered", "Condition", "One", "Read, then Write", "None", "None", "Changes"),
            mechanism: "partial",
            note: "Metamorph changes scope in stages, as Z changes from compose to perform to record to rehear; Z recomposes into new ground, and Metamorph does not return",
        }))
        .collect()
}

fn rules() -> Vec<Rule> {
    const WAYFINDER: &str = "docs/integrations/epi-logos/QL-NATIVE-DECISION-AGENT-WAYFINDER.md";
    const AGENT: &str = "docs/integrations/epi-logos/EPI-LOGOS-AGENT-ARCHITECTURE.md";
    vec![
        Rule {
            id: "zero-heads-zero-calls",
            text: "A cell with no unresolved head makes no model call.",
            source: WAYFINDER,
        },
        Rule {
            id: "no-inference-on-entry-paths",
            text: "No instrument is consulted on an ordinary entry or tool path without a measured, bounded admission; refinement runs in the background and a stale answer is refused.",
            source: WAYFINDER,
        },
        Rule {
            id: "answer-is-evidence",
            text: "A model answer is evidence for the engine to validate. It is never a receipt, never an observed fact, and never overrides one.",
            source: WAYFINDER,
        },
        Rule {
            id: "abstention-representable",
            text: "An empty selection is a representable abstention and is kept as given.",
            source: "docs/integrations/epi-logos/QL-AGENT-DECISION-CONTRACTS.md",
        },
        Rule {
            id: "not-a-closure-referee",
            text: "No instrument decides that a task, a gate or a day is closed.",
            source: AGENT,
        },
        Rule {
            id: "not-a-recognition-engine",
            text: "No instrument stands in for the person's Recognition.",
            source: AGENT,
        },
        Rule {
            id: "provider-not-from-acting-roster",
            text: "The decision provider is never resolved from the roster of agents acting in the cell, and never from the team's own members.",
            source: AGENT,
        },
        Rule {
            id: "gate-6-is-the-person",
            text: "Gate 6 is answered by the person only; a model answer to it is refused.",
            source: "skills/aletheia-collab-gate/SKILL.md",
        },
        Rule {
            id: "engine-validates",
            text: "The engine decides what holds: ql vak compose accepts or refuses the block, the gate rules and kernel operations accept or refuse the readings.",
            source: "skills/vak-evaluate/SKILL.md",
        },
    ]
}

// ---------------------------------------------------------------------------
// Assembly.
// ---------------------------------------------------------------------------

/// Frames an agent reference stands in; the lead agents stand in the `lead`
/// cell.
fn frames_of(agent_ref: &str) -> Vec<String> {
    let mut frames = Vec::new();
    for frame in ContextFrameId::ALL {
        if frame != ContextFrameId::Cf5 && anima_ref(frame) == agent_ref {
            frames.push(frame.code().to_string());
        }
    }
    for (specialist, frame, _) in SEATS {
        if aletheia_ref(specialist) == agent_ref {
            frames.push(frame.code().to_string());
        }
    }
    if aletheia_ref(Specialist::Moirai) == agent_ref {
        for night in NIGHT.iter().filter(|n| n.moirai_mode.is_some()) {
            frames.push(night.frame.code().to_string());
        }
    }
    if agent_ref == ANIMA_LEAD || agent_ref == ALETHEIA_LEAD {
        frames.push("lead".to_string());
    }
    frames.sort();
    frames.dedup();
    frames
}

fn rows(instruments: &[Instrument]) -> Vec<Row> {
    ContextFrameId::ALL
        .iter()
        .map(|&frame| {
            let mut aletheia: Vec<Seat> = Vec::new();
            for night in NIGHT
                .iter()
                .filter(|n| n.frame == frame && n.moirai_mode.is_some())
            {
                aletheia.push(Seat {
                    specialist: Specialist::Moirai.name(),
                    agent_ref: aletheia_ref(Specialist::Moirai),
                    mode: night.moirai_mode,
                    office: Specialist::Moirai.office(),
                    joins_into: None,
                });
            }
            for (specialist, seat_frame, joins) in SEATS {
                if seat_frame == frame {
                    aletheia.push(Seat {
                        specialist: specialist.name(),
                        agent_ref: aletheia_ref(specialist),
                        mode: None,
                        office: specialist.office(),
                        joins_into: joins.map(ContextFrameId::code),
                    });
                }
            }
            let day =
                DAY.iter()
                    .find(|(_, _, f)| *f == Some(frame))
                    .map(|(position, question, _)| DayQuestion {
                        position: wire(position),
                        question,
                    });
            let night = NIGHT
                .iter()
                .filter(|n| n.frame == frame)
                .map(|n| NightQuestion {
                    position: wire(&n.position),
                    prime: n.prime,
                    question: n.question,
                    reading: n.reading,
                    answered_by: night_answerer(n),
                })
                .collect();
            let gate = GATES
                .iter()
                .find(|(_, _, _, contact)| *contact == Some(frame))
                .map(|(number, ..)| *number);
            Row {
                frame: frame.code(),
                expression: frame.expression(),
                name: frame.name(),
                anima: voice(frame),
                aletheia,
                day,
                night,
                gate,
                instruments: instruments
                    .iter()
                    .filter(|i| i.cells.iter().any(|c| c == frame.code()))
                    .map(|i| i.id.clone())
                    .collect(),
            }
        })
        .collect()
}

fn depth() -> Vec<DepthDomain> {
    native_faculties()
        .iter()
        .map(|&(position, faculty)| {
            let operations = native_operations(position).unwrap_or_default().to_vec();
            DepthDomain {
                m: position,
                faculty,
                m_prime: M_PRIME_INSTRUMENTS[usize::from(position)],
                anima: format!("Anima_{position} = M{position} × S4′"),
                aletheia: format!("Aletheia_{position} = M{position} × S5′"),
                techne: format!("Technē_{position} := Aletheia_{position} operating M{position}′"),
                native_operations: operations,
                proving_position: match position {
                    4 => Some(ANIMA_PROVING_POSITION),
                    5 => Some(ALETHEIA_PROVING_POSITION),
                    _ => None,
                },
            }
        })
        .collect()
}

/// The Vāk block as the act's coordinates: one layer for each head of the
/// block instrument.
fn path(block: &Instrument) -> Vec<PathLayer> {
    const LAYERS: [&str; 7] = ["CPF", "CT", "CP", "CF", "CFP", "CS", "direction"];
    block
        .heads
        .iter()
        .zip(LAYERS)
        .map(|(head, layer)| PathLayer {
            layer,
            head: head["id"].as_str().unwrap_or_default().to_string(),
            field: head["field"].as_str().unwrap_or_default().to_string(),
            labels: head["labels"].as_array().map_or(0, Vec::len),
        })
        .collect()
}

fn gates() -> Vec<Gate> {
    GATES
        .iter()
        .map(|&(number, skill, name, contact)| Gate {
            number,
            name,
            skill: format!("skill/ql/{skill}"),
            contact: contact.map(voice),
            contact_frame: contact.map(ContextFrameId::code),
            human_only: contact.is_none(),
            results: if contact.is_none() {
                vec!["aligned", "hold", "redirect"]
            } else {
                vec!["aligned", "annotated", "hold", "redirect"]
            },
        })
        .collect()
}

/// The team space as the kernel holds it.
pub fn team_space() -> TeamSpace {
    let block = vak_block_instrument();
    let mut instruments = vec![block];
    instruments.extend(gate_instruments());
    instruments.push(thought_reading_instrument());
    instruments.extend(outcome_instruments());
    for instrument in &mut instruments {
        if instrument.cells.is_empty() {
            let mut cells: Vec<String> = instrument
                .seats
                .iter()
                .flat_map(|seat| frames_of(seat))
                .collect();
            cells.sort();
            cells.dedup();
            instrument.cells = cells;
        }
    }
    let path = path(&instruments[0]);
    TeamSpace {
        contract: TEAM_SPACE_CONTRACT,
        organs: vec![
            Organ {
                organ: ANIMA_ORGAN,
                lead: ANIMA_LEAD,
                office: "composes the situated act through the C′ Vāk grammar",
                proving_position: ANIMA_PROVING_POSITION,
            },
            Organ {
                organ: ALETHEIA_ORGAN,
                lead: ALETHEIA_LEAD,
                office: "discloses what happened and returns it",
                proving_position: ALETHEIA_PROVING_POSITION,
            },
        ],
        rows: rows(&instruments),
        gates: gates(),
        depth: depth(),
        path,
        instruments,
        thread_forms: thread_compositions(),
        rules: rules(),
        sources: vec![
            "docs/integrations/epi-logos/ANIMA-ALETHEIA-TEAMS.md",
            "docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md",
            "docs/integrations/epi-logos/QL-NATIVE-DECISION-AGENT-WAYFINDER.md",
            "docs/integrations/epi-logos/QL-AGENT-DECISION-CONTRACTS.md",
            "docs/L5-TECHNE-DUAL-READING-LOCK.md",
            "skills/day-night-pass/SKILL.md",
            "skills/vak-coordinate-frame/SKILL.md",
        ],
    }
}

// ---------------------------------------------------------------------------
// Validation, asking and admitting. None of this calls a model.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamSpaceError {
    UnknownInstrument(String),
    UnknownHead(String),
    /// Gate 6 and any person-only instrument refuse a model answer.
    HumanOnly(String),
    /// The provider is a member of the acting roster or of the team itself.
    ProviderIsActing(String),
    IllegalLabel(String),
    Cardinality {
        head: String,
        given: usize,
    },
    Invalid(String),
}

impl std::fmt::Display for TeamSpaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownInstrument(id) => write!(f, "unknown decision instrument `{id}`"),
            Self::UnknownHead(id) => write!(f, "unknown head `{id}`"),
            Self::HumanOnly(id) => {
                write!(
                    f,
                    "`{id}` is answered by the person only; a model answer is refused"
                )
            }
            Self::ProviderIsActing(who) => write!(
                f,
                "decision provider `{who}` is in the acting roster or the team; it cannot also decide"
            ),
            Self::IllegalLabel(label) => write!(f, "label `{label}` is not legal for this head"),
            Self::Cardinality { head, given } => {
                write!(f, "head `{head}` cannot take {given} labels")
            }
            Self::Invalid(why) => write!(f, "invalid team space: {why}"),
        }
    }
}

impl std::error::Error for TeamSpaceError {}

fn invalid(why: impl Into<String>) -> TeamSpaceError {
    TeamSpaceError::Invalid(why.into())
}

/// The head rules of `ql.agent-decision-frame/v1` (`scripts/ql_agent_contracts.py`):
/// non-empty ids and labels, unique labels, a legal ambiguity policy and a
/// possible cardinality.
pub fn validate_head(head: &Value) -> Result<(), TeamSpaceError> {
    let text = |value: &Value, what: &str| -> Result<(), TeamSpaceError> {
        match value.as_str() {
            Some(s) if !s.trim().is_empty() => Ok(()),
            _ => Err(invalid(format!("head {what} must be non-empty text"))),
        }
    };
    text(&head["id"], "id")?;
    text(&head["field"], "field")?;
    let labels = head["labels"]
        .as_array()
        .filter(|l| !l.is_empty())
        .ok_or_else(|| invalid("a head needs at least one label"))?;
    let mut seen = BTreeSet::new();
    for label in labels {
        for key in ["id", "description", "owner_ref", "owner_revision"] {
            text(&label[key], &format!("label {key}"))?;
        }
        let id = label["id"].as_str().unwrap_or_default();
        if !seen.insert(id) {
            return Err(invalid(format!("duplicate label `{id}`")));
        }
    }
    let min = head["cardinality"]["min"].as_u64();
    let max = head["cardinality"]["max"].as_u64();
    match (min, max) {
        (Some(min), Some(max)) if max >= 1 && min <= max && max as usize <= labels.len() => {}
        _ => return Err(invalid("impossible cardinality")),
    }
    match head["ambiguity_policy"].as_str() {
        Some("preserve-candidates" | "abstain" | "single-if-supported") => Ok(()),
        _ => Err(invalid("unknown ambiguity policy")),
    }
}

fn validate_constraint(constraint: &Value, heads: &[&Value]) -> Result<(), TeamSpaceError> {
    let selections: Vec<&Value> = match constraint["kind"].as_str() {
        Some("implication") => constraint["antecedent"]
            .as_array()
            .into_iter()
            .chain(constraint["consequent"].as_array())
            .flatten()
            .collect(),
        Some("exclusion") => constraint["forbidden_together"]
            .as_array()
            .into_iter()
            .flatten()
            .collect(),
        _ => return Err(invalid("unsupported constraint kind in the team space")),
    };
    if selections.is_empty() {
        return Err(invalid("a constraint needs selections"));
    }
    for selection in selections {
        let head_id = selection["head_id"].as_str().unwrap_or_default();
        let head = heads
            .iter()
            .find(|h| h["id"] == head_id)
            .ok_or_else(|| invalid(format!("constraint names absent head `{head_id}`")))?;
        let legal: BTreeSet<&str> = head["labels"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|l| l["id"].as_str())
            .collect();
        for id in selection["label_ids"].as_array().into_iter().flatten() {
            if !id.as_str().is_some_and(|s| legal.contains(s)) {
                return Err(invalid(format!(
                    "constraint names illegal label in `{head_id}`"
                )));
            }
        }
    }
    Ok(())
}

/// A request to put unresolved heads to a decision instrument.
#[derive(Debug, Clone, Serialize)]
pub struct Ask {
    pub instrument: String,
    pub audience: Audience,
    pub heads: Vec<Value>,
    pub constraints: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answerer {
    Person,
    Model { provider_ref: String },
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub instrument: String,
    pub head: String,
    pub labels: Vec<String>,
    pub answerer: Answerer,
}

/// What an admitted answer is: evidence for the engine, with no receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AnswerStanding {
    pub standing: &'static str,
    pub receipt: bool,
    pub abstention: bool,
}

impl Instrument {
    /// Put the named unresolved heads to the instrument. With none unresolved
    /// there is nothing to ask and no call is made (`None`).
    pub fn ask(&self, unresolved: &[&str]) -> Result<Option<Ask>, TeamSpaceError> {
        if unresolved.is_empty() {
            return Ok(None);
        }
        let mut heads = Vec::new();
        for id in unresolved {
            let head = self
                .heads
                .iter()
                .find(|h| h["id"] == *id)
                .ok_or_else(|| TeamSpaceError::UnknownHead((*id).to_string()))?;
            heads.push(head.clone());
        }
        let asked: BTreeSet<&str> = unresolved.iter().copied().collect();
        let constraints = self
            .constraints
            .iter()
            .filter(|c| {
                let text = c.to_string();
                self.heads.iter().all(|h| {
                    let id = h["id"].as_str().unwrap_or_default();
                    asked.contains(id) || !text.contains(&format!("\"head_id\":\"{id}\""))
                })
            })
            .cloned()
            .collect();
        Ok(Some(Ask {
            instrument: self.id.clone(),
            audience: self.answerable_by,
            heads,
            constraints,
        }))
    }
}

impl TeamSpace {
    pub fn instrument(&self, id: &str) -> Option<&Instrument> {
        self.instruments.iter().find(|i| i.id == id)
    }

    /// Every agent reference the grid names.
    pub fn member_refs(&self) -> BTreeSet<String> {
        let mut refs: BTreeSet<String> = self.organs.iter().map(|o| o.lead.to_string()).collect();
        for row in &self.rows {
            refs.insert(row.anima.agent_ref.clone());
            refs.extend(row.aletheia.iter().map(|s| s.agent_ref.clone()));
        }
        refs
    }

    /// Check the whole space: every head and constraint is legal, ids are
    /// unique, and each instrument keeps one head for each field.
    pub fn validate(&self) -> Result<(), TeamSpaceError> {
        let mut ids = BTreeSet::new();
        for instrument in &self.instruments {
            if !ids.insert(instrument.id.as_str()) {
                return Err(invalid(format!("duplicate instrument `{}`", instrument.id)));
            }
            let heads: Vec<&Value> = instrument.heads.iter().collect();
            let mut fields = BTreeSet::new();
            let mut head_ids = BTreeSet::new();
            for head in &heads {
                validate_head(head)?;
                if !fields.insert(head["field"].as_str().unwrap_or_default())
                    || !head_ids.insert(head["id"].as_str().unwrap_or_default())
                {
                    return Err(invalid(format!(
                        "instrument `{}` repeats a head or field",
                        instrument.id
                    )));
                }
            }
            for constraint in &instrument.constraints {
                validate_constraint(constraint, &heads)?;
            }
        }
        Ok(())
    }

    /// Admit an answer as evidence. A model answer to a person-only
    /// instrument is refused; a provider in the acting roster or the team is
    /// refused; labels and cardinality must be legal. No model is called.
    pub fn admit_answer(
        &self,
        answer: &Answer,
        acting_roster: &[String],
    ) -> Result<AnswerStanding, TeamSpaceError> {
        let instrument = self
            .instrument(&answer.instrument)
            .ok_or_else(|| TeamSpaceError::UnknownInstrument(answer.instrument.clone()))?;
        if let Answerer::Model { provider_ref } = &answer.answerer {
            if instrument.answerable_by == Audience::PersonOnly {
                return Err(TeamSpaceError::HumanOnly(instrument.id.clone()));
            }
            if acting_roster.iter().any(|r| r == provider_ref)
                || self.member_refs().contains(provider_ref)
            {
                return Err(TeamSpaceError::ProviderIsActing(provider_ref.clone()));
            }
        }
        let head = instrument
            .heads
            .iter()
            .find(|h| h["id"] == answer.head.as_str())
            .ok_or_else(|| TeamSpaceError::UnknownHead(answer.head.clone()))?;
        let legal: BTreeSet<&str> = head["labels"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|l| l["id"].as_str())
            .collect();
        let mut given = BTreeSet::new();
        for id in &answer.labels {
            if !legal.contains(id.as_str()) || !given.insert(id.as_str()) {
                return Err(TeamSpaceError::IllegalLabel(id.clone()));
            }
        }
        let min = head["cardinality"]["min"].as_u64().unwrap_or(0) as usize;
        let max = head["cardinality"]["max"].as_u64().unwrap_or(0) as usize;
        if answer.labels.len() < min || answer.labels.len() > max {
            return Err(TeamSpaceError::Cardinality {
                head: answer.head.clone(),
                given: answer.labels.len(),
            });
        }
        Ok(AnswerStanding {
            standing: "evidence",
            receipt: false,
            abstention: answer.labels.is_empty(),
        })
    }

    /// A short reading of the grid for the terminal.
    pub fn render_text(&self) -> String {
        let mut lines = vec![format!(
            "{} — {}",
            TEAM_SPACE_CONTRACT, "rows are the context frames"
        )];
        for row in &self.rows {
            let members = row
                .aletheia
                .iter()
                .map(|s| match s.mode {
                    Some(mode) => format!("{} as {mode}", s.specialist),
                    None => s.specialist.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" + ");
            let day = row.day.as_ref().map_or("-".to_string(), |d| {
                format!("{} {}", d.position, d.question)
            });
            let night = if row.night.is_empty() {
                "-".to_string()
            } else {
                row.night
                    .iter()
                    .map(|n| format!("{} {}", n.prime, n.question))
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            let gate = row.gate.map_or("-".to_string(), |g| format!("gate {g}"));
            lines.push(format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                row.frame,
                row.expression,
                row.anima.voice,
                members,
                day,
                night,
                gate,
                row.instruments.join(",")
            ));
        }
        lines.push("gate 6\tperson only".to_string());
        lines.push(format!(
            "depth: {}",
            self.depth
                .iter()
                .map(|d| format!("M{} {}", d.m, d.m_prime))
                .collect::<Vec<_>>()
                .join(" | ")
        ));
        lines.push(format!(
            "path: {}",
            self.path
                .iter()
                .map(|p| format!("{}({})", p.layer, p.labels))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests;
