//! Source compiler tests use actual retained M1/B/K/M3/P producers. Authored
//! score input is a real composition operand, not a fabricated played receipt.
//! Actual played receipt/Document/Act tests belong to the native consumer floor.
#[path = "support/retained_performance.rs"]
mod support;
use ql_mef::MFace;
use ql_mef::m2_tuning_sources::{
    planetary_interval_collection, retained_condition_collection,
    source_spelled_condition_collection,
};
use ql_mef::music_determination::{Fundamental, NoteTarget, TuningProvenance, TuningStanding};
use ql_mef::musical_performance_return::*;
use ql_mef::musical_performance_score::*;
use ql_mef::performance_audio::{PreparedPerformanceBinding, prepare_native_performance};
use ql_mef::source_key_determination::*;
use serde_json::{Value, json};

fn reference(name: &str) -> ReturnReference {
    ReturnReference {
        reference: name.into(),
        revision: "1".into(),
    }
}
fn world(p: &PreparedPerformanceBinding) -> MusicalPerformanceReturn {
    bind_performance_return(
        p,
        None,
        ReturnContext {
            context: reference("reference:world"),
            receiver: reference("reference:world-receiver"),
            source_occasion: None,
            protected_state: None,
            consent: None,
            kind: "world".into(),
            private: false,
            required_assets: vec![],
        },
        73,
    )
    .unwrap()
}
fn source<'a>(
    p: &'a PreparedPerformanceBinding,
    r: &'a MusicalPerformanceReturn,
) -> ScoreSource<'a> {
    ScoreSource {
        prepared: p,
        original_return: r,
        keys: ScoreKeys::Architectural,
    }
}
fn pitch(note: &NoteTarget, key: u8) -> Value {
    json!({"basis":0,"source_coordinate":note.source_coordinate.source_ref,"source_prime":note.source_coordinate.face==MFace::Pratibimba,
        "key":key,"pitch_class":note.pitch_class,"register":note.register,"fundamental_hz":note.fundamental.hertz(),"hertz":note.hertz,
        "exact_ratio":note.exact_ratio.map(|r|json!({"numerator":r.numerator().to_string(),"denominator":r.denominator().to_string()})),
        "tuning_ref":note.tuning_provenance.policy_ref})
}
fn composition(r: &MusicalPerformanceReturn, pitches: Vec<Value>, events: Vec<Value>) -> Value {
    let mut basis = r.expression_basis().unwrap();
    // A supplied native Expression seal remains a reference. The compiler
    // recomputes every source operand and the complete score input itself.
    basis["content_digest"] = json!("reference:test-owned-Expression-basis-receipt");
    json!({"schema":"oi.expression-performance/v1","performance_ref":"expression:musical-score/current","sample_rate":48000,
        "duration_samples":"43200000","ppq":960,"bases":[basis],"pitches":pitches,
        "layers":[{"layer_ref":"layer:main","title":"Main","enabled":true,"solo":false}],
        "pages":events.chunks(128).map(|events|json!({"events":events})).collect::<Vec<_>>(),
        "parameters":[{"native_owner":"ql.audio","action_ref":"parameter","target_ref":"force","unit":"N","scope":"instrument",
            "minimum":0.0,"maximum":10.0,"baseline":1.0,"smoothing_samples":"64"}],
        "routes":[{"route_ref":"route:force","source_ref":"source:automation","enabled":true,"scale":1.0,"offset":0.0,"delay_samples":"0"}],
        "tempo":[{"at_sample":"0","at_tick":"0","micros_per_quarter":500000}],"loop_range":null,"position_sample":"0",
        "replay":{"mode":"native_checkpoint","max_reconstruction_samples":"48000","model_revision":"ql.performance-audio/v1",
            "event_tolerance_samples":0,"physical_tolerance":0.0,"display_policy":"same-native-cursor"},"checkpoints":[],
        "content_digest":"reference:test-owned-Expression-performance-receipt"})
}
fn note_events(p: &PreparedPerformanceBinding) -> Vec<Value> {
    let sine = p.notes()[0]["phase_sin"].as_f64().unwrap();
    let cosine = p.notes()[0]["phase_cos"].as_f64().unwrap();
    vec![
        json!(["1","37",0,0,{"n":["1","1",0,0.7,sine,cosine]}]),
        json!(["2","256",0,0,{"o":"1"}]),
    ]
}

#[test]
fn actual_source_form_pitch_and_same_event_pages_return_without_a_second_clock_or_store() {
    let p = prepare_native_performance(support::preparation()).unwrap();
    let r = world(&p);
    let note = p
        .targets()
        .key_target(4, 0, "source:authored-score")
        .unwrap();
    let input = composition(&r, vec![pitch(&note, 4)], note_events(&p));
    let score = compile_score("expression:retained/current", 1, &[source(&p, &r)], &input).unwrap();
    score.verify_replay(&[source(&p, &r)], &input).unwrap();
    let reading = score.snapshot().unwrap();
    assert_eq!(
        reading["source_bases"][0]["m3_source_score"],
        p.native_basis().m3
    );
    assert_eq!(reading["original_episode_refs"], json!([null]));
    assert_eq!(reading["controls"]["tempo"], input["tempo"]);
    assert_eq!(reading["controls"]["routes"], input["routes"]);
    assert_eq!(
        score.pitch_sources()[0]["exact_ratio"],
        input["pitches"][0]["exact_ratio"]
    );
    let first = &score.pages()[0];
    assert_eq!(first.events, 2);
    assert_eq!(first.first_sample, "37");
    assert_eq!(first.last_sample, "256");
    assert!(first.part_ref.starts_with("sha256:"));
    assert!(
        reading.get("events").is_none(),
        "same Act event pages are referenced rather than duplicated"
    );
    assert_eq!(
        r.replay_score().unwrap().snapshot(),
        p.native_basis().m3,
        "musical timing did not advance M3 geometry/civil state"
    );
}
#[test]
fn authored_edit_layer_route_and_automation_reinscribe_native_score_with_cas_and_immutable_history()
{
    let p = prepare_native_performance(support::preparation()).unwrap();
    let r = world(&p);
    let first = p.targets().key_target(4, 0, "source:score/first").unwrap();
    let mut input = composition(&r, vec![pitch(&first, 4)], note_events(&p));
    let score = compile_score("expression:retained/current", 1, &[source(&p, &r)], &input).unwrap();
    let before = score.snapshot().unwrap();
    let second = p.targets().key_target(6, 0, "source:score/edit").unwrap();
    input["pitches"][0] = pitch(&second, 6);
    input["layers"]
        .as_array_mut()
        .unwrap()
        .push(json!({"layer_ref":"layer:overdub","title":"Overdub","enabled":true,"solo":false}));
    let sine = p.notes()[0]["phase_sin"].as_f64().unwrap();
    let cosine = p.notes()[0]["phase_cos"].as_f64().unwrap();
    input["pages"][0]["events"].as_array_mut().unwrap().extend([
        json!(["3","300",1,0,{"n":["2","2",0,0.4,sine,cosine]}]),
        json!(["4","320",1,0,{"a":[0,2.0,null]}]),
        json!(["5","512",1,0,{"o":"2"}]),
    ]);
    input["routes"][0]["scale"] = json!(0.5);
    input["position_sample"] = json!("512");
    input["loop_range"] = json!({"from_sample":"37","to_sample":"512"});
    let next = score.reinscribe(1, &[source(&p, &r)], &input).unwrap();
    assert_eq!(next.generation().unwrap(), 2);
    assert_ne!(next.pages()[0].part_ref, score.pages()[0].part_ref);
    assert_eq!(
        next.snapshot().unwrap()["controls"]["routes"],
        input["routes"]
    );
    assert_eq!(
        next.snapshot().unwrap()["controls"]["loop_range"],
        input["loop_range"]
    );
    assert_eq!(score.snapshot().unwrap(), before);
    assert!(score.reinscribe(2, &[source(&p, &r)], &input).is_err());
    let mut foreign = input.clone();
    foreign["performance_ref"] = json!("foreign:performance");
    assert!(score.reinscribe(1, &[source(&p, &r)], &foreign).is_err());
}
#[test]
fn lost_exact_tuning_body_force_context_note_or_page_fails_native_source_replay() {
    let p = prepare_native_performance(support::preparation()).unwrap();
    let r = world(&p);
    let note = p.targets().key_target(4, 0, "source:score").unwrap();
    let input = composition(&r, vec![pitch(&note, 4)], note_events(&p));
    let score = compile_score("expression:retained/current", 1, &[source(&p, &r)], &input).unwrap();
    for pointer in [
        "/bases/0/m2_plan",
        "/bases/0/prepared_body",
        "/bases/0/force_state",
        "/bases/0/context",
        "/bases/0/m3_score",
        "/pitches/0/exact_ratio",
    ] {
        let mut lost = input.clone();
        *lost.pointer_mut(pointer).unwrap() = Value::Null;
        assert!(
            compile_score("expression:retained/current", 1, &[source(&p, &r)], &lost).is_err(),
            "lost {pointer}"
        );
    }
    let mut wrong = input.clone();
    wrong["pitches"][0]["hertz"] = json!(note.hertz + 1.0);
    assert!(compile_score("expression:retained/current", 1, &[source(&p, &r)], &wrong).is_err());
    let mut phase = input.clone();
    let sine = p.notes()[0]["phase_sin"].as_f64().unwrap();
    let cosine = p.notes()[0]["phase_cos"].as_f64().unwrap();
    phase["pages"][0]["events"][0][4]["n"][4] = json!(-sine);
    phase["pages"][0]["events"][0][4]["n"][5] = json!(-cosine);
    assert!(
        compile_score("expression:retained/current", 1, &[source(&p, &r)], &phase).is_err(),
        "a valid quadrature cannot replace the actual source M1 phase"
    );
    let mut dropped = input.clone();
    dropped["pages"][0]["events"]
        .as_array_mut()
        .unwrap()
        .remove(1);
    assert!(score.verify_replay(&[source(&p, &r)], &dropped).is_err());
    let mut raw = input.clone();
    raw["pages"][0]["events"][1] = json!(["2", "64", 0, 0, "c"]);
    assert!(
        compile_score("expression:retained/current", 1, &[source(&p, &r)], &raw).is_err(),
        "held context cannot be transferred by relabeling"
    );
}
fn provenance() -> TuningProvenance {
    TuningProvenance {
        policy_ref: "reference:explicit-score-seven-source-keys".into(),
        source_ref: "reference:score-source-degree-assignments".into(),
        revision: "1".into(),
        standing: TuningStanding::Reference,
    }
}
#[test]
fn sparse_seven_source_keys_are_qualified_and_inactive_or_authentic_unavailable_degrees_refuse() {
    let preparation = support::preparation();
    let original_m2 = preparation.coupled.compose().unwrap().m2_input;
    let p = prepare_native_performance(preparation).unwrap();
    let r = world(&p);
    let writer = ql_mef::m_tree::native_current_m_registry()
        .coordinate("#2-1", MFace::Pratibimba)
        .unwrap();
    let consumer = || SparseMusicalConsumer {
        basis: p.native_basis(),
        writer: &writer,
        phase: p.targets().determination.identity().tick12() / 6,
        condition: Some(SparseConditionConsumer {
            producer_input: &original_m2,
            plan: p.relation_plan(),
        }),
    };
    let fundamental = Fundamental::new(220.0, provenance()).unwrap();
    let condition = &p.relation_plan().execution.condition_input;
    let reduction = ActiveKeyReduction {
        provenance: provenance(),
        assignments: [0, 2, 4, 5, 7, 9, 11]
            .into_iter()
            .enumerate()
            .map(|(i, key)| KeyDegreeAssignment {
                key,
                source_degree: i as u16,
                octave: 0,
            })
            .collect(),
    };
    let prepare = |collection| {
        SparseKeyTargets::prepare(SparseKeyPreparation {
            determination: p.targets().determination.clone(),
            collection,
            reduction: reduction.clone(),
            octet: None,
            requirement: SourcePitchRequirement::DeclaredAvailable,
            consumer: consumer(),
        })
        .unwrap()
    };
    let sparse = prepare(
        retained_condition_collection(condition.maqam_index, condition.role, &fundamental).unwrap(),
    );
    let target = sparse.key_target(4, 0, "source:sparse-score").unwrap();
    let actual = target.note().unwrap();
    let input = composition(&r, vec![pitch(actual, 4)], note_events(&p));
    let sparse_source = || ScoreSource {
        prepared: &p,
        original_return: &r,
        keys: ScoreKeys::Sparse {
            targets: &sparse,
            consumer: consumer(),
        },
    };
    let score =
        compile_score("expression:retained/current", 1, &[sparse_source()], &input).unwrap();
    assert_eq!(score.pitch_sources()[0]["key_source"]["source_degree"], 2);
    assert_eq!(
        score.pitch_sources()[0]["key_source"]["source_collection"]["standing"],
        "retained_approximation"
    );
    assert!(
        compile_score("expression:retained/current", 1, &[source(&p, &r)], &input).is_err(),
        "architectural twelve-key pitches cannot substitute source-selected sparse tuning"
    );
    let mut inactive = input.clone();
    inactive["pitches"][0]["key"] = json!(1);
    assert!(
        compile_score(
            "expression:retained/current",
            1,
            &[sparse_source()],
            &inactive
        )
        .is_err()
    );
    let missing = prepare(
        source_spelled_condition_collection(condition.maqam_index, condition.role, &fundamental)
            .unwrap(),
    );
    let missing_source = ScoreSource {
        prepared: &p,
        original_return: &r,
        keys: ScoreKeys::Sparse {
            targets: &missing,
            consumer: consumer(),
        },
    };
    assert!(compile_score("expression:retained/current", 1, &[missing_source], &input).is_err());
    let individual = SparseKeyTargets::prepare(SparseKeyPreparation {
        determination: p.targets().determination.clone(),
        collection: planetary_interval_collection("#2-5-9", &fundamental).unwrap(),
        reduction: ActiveKeyReduction {
            provenance: provenance(),
            assignments: vec![KeyDegreeAssignment {
                key: 0,
                source_degree: 0,
                octave: 0,
            }],
        },
        octet: None,
        requirement: SourcePitchRequirement::SourceAuthoredExact,
        consumer: consumer(),
    })
    .unwrap();
    let target = individual
        .key_target(0, 0, "source:unfolded-pluto")
        .unwrap();
    let note = target.note().unwrap();
    let pluto = composition(&r, vec![pitch(note, 0)], note_events(&p));
    let score = compile_score(
        "expression:retained/current",
        1,
        &[ScoreSource {
            prepared: &p,
            original_return: &r,
            keys: ScoreKeys::Sparse {
                targets: &individual,
                consumer: consumer(),
            },
        }],
        &pluto,
    )
    .unwrap();
    assert_eq!(
        score.pitch_sources()[0]["exact_ratio"],
        json!({"numerator":"9","denominator":"4"})
    );
}
#[test]
fn full_fifteen_minute_45000_event_score_references_lossless_existing_pages_and_detects_drops() {
    let p = prepare_native_performance(support::preparation()).unwrap();
    let r = world(&p);
    let note = p.targets().key_target(4, 0, "source:full-score").unwrap();
    let sine = p.notes()[0]["phase_sin"].as_f64().unwrap();
    let cosine = p.notes()[0]["phase_cos"].as_f64().unwrap();
    let mut events = Vec::with_capacity(45000);
    for n in 0..22500u64 {
        let touch = n + 1;
        let sample = n * 1920;
        events.push(json!([(2*n+1).to_string(),sample.to_string(),0,0,{"n":[touch.to_string(),(n%24+1).to_string(),0,0.7,sine,cosine]}]));
        events.push(
            json!([(2*n+2).to_string(),(sample+960).to_string(),0,0,{"o":touch.to_string()}]),
        );
    }
    let input = composition(&r, vec![pitch(&note, 4)], events);
    let score = compile_score("expression:retained/current", 1, &[source(&p, &r)], &input).unwrap();
    assert_eq!(score.pages().iter().map(|p| p.events).sum::<usize>(), 45000);
    assert_eq!(score.pages().len(), 352);
    assert_eq!(
        score.snapshot().unwrap()["controls"]["duration_samples"],
        "43200000"
    );
    assert!(
        serde_json::to_vec(&score).unwrap().len() < 1024 * 1024,
        "score return shares existing event pages instead of copying 45k events per edition"
    );
    score.verify_replay(&[source(&p, &r)], &input).unwrap();
    let mut missing = input.clone();
    missing["pages"].as_array_mut().unwrap().remove(12);
    assert!(score.verify_replay(&[source(&p, &r)], &missing).is_err());
    assert_eq!(
        input["pages"].as_array().unwrap().len(),
        352,
        "original edition pages were preserved"
    );
}
