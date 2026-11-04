//! The Torus's M1′ session through the field host, over the real installed
//! worker (O-I #850): every setting reaches the owner and reads back; the band
//! stands beside the rate; the comma starts from nothing and grows only as the
//! field runs; a key strikes its moving point and always the same helix place;
//! Sync follows a tempo change; a tuning switch changes the reported voices.
//! Run with `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test
//! torus_host -- --ignored`. The wire contract is exercised without a worker.
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::continuous::host::{FieldHost, HostOperation, HostRequest};
use ql_mef::continuous::scene_field::{BINDING_REQUEST, BindingRequest, SceneConfig};
use ql_mef::janko::{HelixPlace, JankoKey, JankoSurface, point_of};
use serde_json::{Value, json};

fn worker() -> PathBuf {
    PathBuf::from(std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker"))
}

fn open() -> FieldHost {
    let binding = ql_mef::continuous::scene_field::binding(BindingRequest {
        schema: BINDING_REQUEST.into(),
        instance_ref: "test:torus".into(),
        texture: [32, 16],
        units_per_metre: 1.0,
        event: None,
        sky: None,
        field: None,
        geometry: Some(ql_mef::continuous::scene_field::SceneGeometry {
            longitude_samples: 32,
            latitude_samples: 16,
            metres_per_unit: 1.0,
            attachment: 1,
        }),
        material: None,
        reception: None,
        torus: None,
    })
    .unwrap();
    let config: SceneConfig = serde_json::from_value(binding["host"].clone()).unwrap();
    FieldHost::open_scene(&worker(), config, Duration::from_secs(20)).unwrap()
}

struct Driver {
    host: FieldHost,
    current: Value,
}

impl Driver {
    fn open() -> Self {
        let host = open();
        let current = host.ready();
        assert_eq!(current["status"], "ready");
        Self { host, current }
    }
    fn send(&mut self, command: Value) -> Value {
        let c = &self.current;
        let request = HostRequest {
            schema: "ql.field-host-request/v1".into(),
            instance_ref: c["instance_ref"].as_str().unwrap().into(),
            event_ref: c["field"]["event_ref"].as_str().unwrap().into(),
            subject_ref: c["field"]["subject_ref"].as_str().unwrap().into(),
            request_id: (c["last_request_id"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap()
                + 1)
            .to_string(),
            expected_generation: c["field"]["generation"].as_str().unwrap().into(),
            expected_samples_elapsed: c["field"]["samples_elapsed"].as_str().unwrap().into(),
            command: serde_json::from_value(command).unwrap(),
        };
        self.current = self.host.execute(request);
        self.current.clone()
    }
    fn generation(&self) -> u64 {
        self.current["field"]["generation"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap()
    }
}

#[test]
fn the_torus_operations_are_admitted_by_name_and_strict_about_their_fields() {
    for command in [
        json!({"operation":"torus-state"}),
        json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":440.0}}}),
        json!({"operation":"torus-set","edit":{"rate":{"mode":"sync","note":{"numerator":1,"denominator":4}},"tempo_bpm":90.0}}),
        json!({"operation":"torus-set","edit":{"tuning":"maqam","polyphony":4,"carrier":0.5,"shape":"figure-eight","fundamental":{"source":"hz","hz":261.6}}}),
        json!({"operation":"torus-set","edit":{"polyphony":3},"at_samples_elapsed":"4800"}),
        json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.1,0.0]}),
        json!({"operation":"torus-play","keys":[{"row":0,"column":2},{"row":3,"column":1}],"amplitude":[0.1,0.0],"at_samples_elapsed":"4800"}),
        json!({"operation":"torus-release","keys":[{"row":0,"column":2}]}),
        json!({"operation":"torus-checkpoint"}),
        json!({"operation":"torus-restore","checkpoint":{"schema":"ql.torus-checkpoint/v1"}}),
        json!({"operation":"torus-render","checkpoint":{"schema":"ql.torus-checkpoint/v1"},"frames":4800,"acts":[
            {"action":"strike","at_samples_elapsed":"96000","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0]},
            {"action":"release","at_samples_elapsed":"96100","keys":[{"row":0,"column":2}]},
            {"action":"set","at_samples_elapsed":"96200","edit":{"polyphony":3}}]}),
        json!({"operation":"torus-render","continues":"render:1","frames":4800,"acts":[]}),
        json!({"operation":"torus-receiver","receiver":{"at_metres":[4.5,0.0,0.25]}}),
        json!({"operation":"torus-receiver","receiver":null}),
        json!({"operation":"torus-singlet","codon":5,"amplitude":[0.2,0.0]}),
        json!({"operation":"torus-singlet","codon":8,"amplitude":[0.2,0.0],"at_samples_elapsed":"4800"}),
    ] {
        assert!(
            serde_json::from_value::<HostOperation>(command.clone()).is_ok(),
            "{command}"
        );
    }
    for command in [
        json!({"operation":"torus-set","edit":{"drive":1.0}}),
        json!({"operation":"torus-set","edit":{"tuning":"equal"}}),
        json!({"operation":"torus-set","edit":{"polyphony":3},"at":"4800"}),
        json!({"operation":"torus-play","keys":[{"row":0}],"amplitude":[0.1,0.0]}),
        json!({"operation":"torus-state","x":1}),
        json!({"operation":"torus-checkpoint","at":"0"}),
        json!({"operation":"torus-restore"}),
        json!({"operation":"torus-render","frames":4800,"acts":[{"action":"strike","at_samples_elapsed":"0","keys":[{"row":0,"column":2}]}]}),
        json!({"operation":"torus-render","frames":4800,"acts":[{"action":"hold","at_samples_elapsed":"0","keys":[]}]}),
        json!({"operation":"torus-render","frames":4800,"acts":[{"action":"release","at_samples_elapsed":"0","keys":[],"amplitude":[0.1,0.0]}]}),
        json!({"operation":"torus-render","frames":4800}),
        json!({"operation":"torus-receiver","receiver":{"at_metres":[4.5,0.0]}}),
        json!({"operation":"torus-receiver","receiver":{"at":[4.5,0.0,0.25]}}),
        json!({"operation":"torus-receiver","receiver":{"at_metres":[4.5,0.0,0.25],"distance":2.0}}),
        json!({"operation":"torus-receiver","place":[4.5,0.0,0.25]}),
        json!({"operation":"torus-singlet","amplitude":[0.2,0.0]}),
        json!({"operation":"torus-singlet","codon":5}),
        json!({"operation":"torus-singlet","codon":300,"amplitude":[0.2,0.0]}),
        json!({"operation":"torus-singlet","codon":5,"amplitude":[0.2,0.0],"arc":6}),
        json!({"operation":"torus-singlet","codon":5,"amplitude":[0.2,0.0],"at":"4800"}),
    ] {
        assert!(
            serde_json::from_value::<HostOperation>(command.clone()).is_err(),
            "{command}"
        );
    }
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn each_setting_reaches_the_owner_and_reads_back() {
    let mut d = Driver::open();
    let state = d.send(json!({"operation":"torus-state"}));
    assert_eq!(state["status"], "ok", "{state}");
    let torus = &state["torus"];
    assert_eq!(torus["schema"], "ql.m1-prime-session/v1");
    assert_eq!(torus["band"], "rhythm");
    assert_eq!(torus["comma_cents"], json!(0.0));
    // A new Torus opens under Logic, the manual's default (§21.2 Parameters).
    assert_eq!(torus["settings"]["tuning"], "logic");
    // Where each of the twelve classes stands and what it sounds under the tuning in force (O-I #851).
    let classes = &torus["classes"];
    assert_eq!(classes["tuning"], "logic", "{classes}");
    assert_eq!(classes["by_class"].as_array().unwrap().len(), 12);
    assert_eq!(
        classes["by_class"][0]["sounds"].as_array().unwrap().len(),
        8
    );
    // The lens in force gives the surface its material (M2-1′, O-I #854): the opening event stands under L5′ at position 1.
    assert_eq!(torus["material"]["lens"], "L5'", "{}", torus["material"]);
    assert_eq!(torus["material"]["partner"], "L5");
    assert_eq!(torus["material"]["position"], 1);
    let generation = d.generation();
    let set = d.send(json!({"operation":"torus-set","edit":{
        "rate":{"mode":"free","steps_per_second":440.0},"carrier":0.25,"shape":"spiral","polyphony":3}}));
    assert_eq!(set["status"], "ok", "{set}");
    // Session settings move no native state.
    assert_eq!(d.generation(), generation);
    let torus = &set["torus"];
    assert_eq!(torus["band"], "pitch");
    assert_eq!(torus["settings"]["carrier"], json!(0.25));
    assert_eq!(torus["settings"]["polyphony"], 3);
    assert_eq!(torus["figure"], "spiral");
    let refused = d.send(json!({"operation":"torus-set","edit":{"polyphony":40}}));
    assert_eq!(refused["status"], "refused");
    assert!(refused["error"].as_str().unwrap().contains("Polyphony"));
    // The condition's maqām as the owner reads it (O-I #854): QL's default condition is Kirdan as tonic under the Bimba spelling, which
    // the map does not spell, so the reading names it and why it is absent, and Maqām is refused in those words, changing nothing.
    let maqam = &torus["maqam"];
    assert_eq!(maqam["name"], "Kirdan", "{maqam}");
    assert_eq!(maqam["coordinate"], "#2-4.3-0-4");
    assert!(
        maqam["absent"]
            .as_str()
            .unwrap()
            .contains("no Bimba spelling"),
        "{maqam}"
    );
    let generation = d.generation();
    let refused = d.send(json!({"operation":"torus-set","edit":{"tuning":"maqam"}}));
    assert_eq!(refused["status"], "refused", "{refused}");
    let why = refused["error"].as_str().unwrap();
    assert!(
        why.contains("Kirdan") && why.contains("no Bimba spelling"),
        "{why}"
    );
    assert_eq!(d.generation(), generation);
    let state = d.send(json!({"operation":"torus-state"}));
    assert_eq!(state["torus"]["settings"]["tuning"], "logic");
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn strike_is_the_share_of_a_keys_velocity_that_excites_the_body() {
    // Manual §21.2 Parameters: Strike "0 to 100% · 70% · How strongly velocity excites the body". Under Body source Tattva the element
    // sets it (Earth 45%); it is a session setting, so setting it moves no native state, and each key's amplitude excites by its share.
    let mut d = Driver::open();
    let state = d.send(json!({"operation":"torus-state"}));
    assert_eq!(state["torus"]["settings"]["strike"], json!(0.7), "{state}");
    let generation = d.generation();
    let set = d.send(json!({"operation":"torus-set","edit":{"strike":0.45}}));
    assert_eq!(set["status"], "ok", "{set}");
    assert_eq!(set["torus"]["settings"]["strike"], json!(0.45));
    assert_eq!(d.generation(), generation, "Strike moves no native state");
    let played = d.send(
        json!({"operation":"torus-play","keys":[{"row":2,"column":1}],"amplitude":[0.1,0.0]}),
    );
    assert_eq!(played["status"], "ok", "{played}");
    assert_eq!(played["torus_play"]["strike"], json!(0.45));
    // The key's point is struck at its velocity; the body takes its share once, at the strike (strike_at_when).
    assert_eq!(played["strike_at"]["points"][0]["amplitude"][0], json!(0.1));
    assert_eq!(played["strike_at"]["strike"], json!(0.45));
    let refused = d.send(json!({"operation":"torus-set","edit":{"strike":1.5}}));
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"].as_str().unwrap().contains("Strike"),
        "{refused}"
    );
}

/// What each voice of a strike at points was given (the `ql.strike-at/v1` receipt's voices).
fn given(reply: &Value) -> Vec<[f64; 2]> {
    reply["strike_at"]["voices"]
        .as_array()
        .unwrap()
        .iter()
        .map(|voice| serde_json::from_value(voice["amplitude"].clone()).unwrap())
        .collect()
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn strike_is_the_bodys_share_of_every_strike_a_touch_a_collision_and_a_singlet_too() {
    // Manual §21.2: Strike, "How strongly velocity excites the body", stands in the Body group with Damping, and under Body source
    // Tattva the element sets both: it is the body's, so everything that strikes the Torus (a key, a touch, a clip's note, a collision,
    // a singlet; "What strikes the Torus") excites it by that share. A strike at points takes it at its own sample, once.
    let mut d = Driver::open();
    let points = json!([{"theta": 0.8, "phi": 1.9, "amplitude": [0.2, 0.1]}]);
    let share = d.send(json!({"operation":"strike-at","points":points}));
    assert_eq!(share["status"], "ok", "{share}");
    assert_eq!(
        share["strike_at"]["strike"],
        json!(0.7),
        "the default Strike, 70%"
    );
    assert_eq!(
        share["strike_at"]["points"], points,
        "the receipt names the points as they were struck"
    );
    assert_eq!(
        d.send(json!({"operation":"torus-set","edit":{"strike":1.0}}))["status"],
        "ok"
    );
    let whole = d.send(json!({"operation":"strike-at","points":points}));
    assert_eq!(whole["strike_at"]["strike"], json!(1.0));
    let (part, full) = (given(&share), given(&whole));
    assert_eq!(part.len(), full.len());
    assert!(
        full.iter().any(|v| *v != [0.0, 0.0]),
        "the point reaches the voices"
    );
    for (part, full) in part.iter().zip(&full) {
        for i in 0..2 {
            assert!(
                (part[i] - 0.7 * full[i]).abs() < 1e-12,
                "{part:?} against {full:?}"
            );
        }
    }
    // A strike for a later sample takes the Strike in force there.
    let from: u64 = d.current["field"]["samples_elapsed"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let waits = d.send(json!({"operation":"torus-set","edit":{"strike":0.25},"at_samples_elapsed":(from + 700).to_string()}));
    assert_eq!(waits["status"], "ok", "{waits}");
    let later = d.send(json!({"operation":"strike-at","points":points,"at_samples_elapsed":(from + 1000).to_string()}));
    assert_eq!(later["status"], "ok", "{later}");
    assert_eq!(later["strike_at"]["strike"], json!(0.25));
    for (later, full) in given(&later).iter().zip(&full) {
        for i in 0..2 {
            assert!(
                (later[i] - 0.25 * full[i]).abs() < 1e-12,
                "{later:?} against {full:?}"
            );
        }
    }
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn the_comma_grows_only_as_the_field_runs_and_sync_follows_the_tempo() {
    let mut d = Driver::open();
    d.send(
        json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":40.0}}}),
    );
    assert_eq!(
        d.send(json!({"operation":"torus-state"}))["torus"]["comma_cents"],
        json!(0.0)
    );
    // 0.4 s at 40 steps a second: sixteen steps, one return.
    for _ in 0..(19_200 / 512 + 1) {
        let advanced = d.send(json!({"operation":"advance","frames":512,"muted":true}));
        assert_eq!(advanced["status"], "ok");
    }
    let comma = d.send(json!({"operation":"torus-state"}))["torus"]["comma_cents"]
        .as_f64()
        .unwrap();
    assert!((comma - 23.460_010_384_649_4).abs() < 1e-6, "{comma}");
    // Sync: a quarter note at 120 BPM is two steps a second; at 60 BPM, one.
    let synced = d.send(json!({"operation":"torus-set","edit":{
        "rate":{"mode":"sync","note":{"numerator":1,"denominator":4}},"tempo_bpm":120.0}}));
    assert_eq!(synced["torus"]["steps_per_second"], json!(2.0));
    let slower = d.send(json!({"operation":"torus-set","edit":{"tempo_bpm":60.0}}));
    assert_eq!(slower["torus"]["steps_per_second"], json!(1.0));
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_key_strikes_its_moving_point_and_always_the_same_helix_place() {
    let mut d = Driver::open();
    let key = json!([{"row":2,"column":1}]);
    let first = d.send(json!({"operation":"torus-play","keys":key,"amplitude":[0.1,0.0]}));
    assert_eq!(first["status"], "ok", "{first}");
    let note = &first["torus_play"]["notes"][0];
    let place = note["place"].clone();
    let point = note["point"].clone();
    assert_eq!(first["strike_at"]["points"][0]["theta"], point["theta"]);
    for _ in 0..10 {
        d.send(json!({"operation":"advance","frames":512,"muted":true}));
    }
    let again = d.send(json!({"operation":"torus-play","keys":key,"amplitude":[0.1,0.0]}));
    let later = &again["torus_play"]["notes"][0];
    assert_eq!(later["place"], place, "the same place of its helix");
    assert_ne!(
        later["point"], point,
        "the torus has carried it across the surface"
    );
    // The repeated key is a new note: the old one made way for it.
    assert_eq!(again["torus_play"]["made_way"].as_array().unwrap().len(), 1);
    // Each note names its own eight voices over its own four boundaries.
    assert_eq!(later["cell"]["voices_hz"].as_array().unwrap().len(), 8);
    assert_eq!(later["cell"]["boundaries"].as_array().unwrap().len(), 4);
    let released = d.send(json!({"operation":"torus-release","keys":key}));
    assert_eq!(released["status"], "ok");
    assert!(released["torus"]["notes"].as_array().unwrap().is_empty());
}

/// Advances the field `frames` samples, muted, in blocks the host takes.
fn run_on(d: &mut Driver, mut frames: u64) {
    while frames > 0 {
        let block = frames.min(8192);
        let advanced = d.send(json!({"operation":"advance","frames":block,"muted":true}));
        assert_eq!(advanced["status"], "ok", "{advanced}");
        frames -= block;
    }
}

/// The steps the pulse had turned at a struck point.
fn steps_of(point: &Value) -> f64 {
    point["pulse_steps"].as_f64().unwrap()
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_clips_note_strikes_its_keys_point_at_its_own_sample() {
    // O-I #771 (MT1 ruled: a key's point is dynamic, from its helix and the pulse). A Live clip plays
    // its notes as torus-play acts at their own samples; each strikes its key's point where the pulse
    // stands at that sample, not where it stood when the act was sent.
    let free =
        json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":40.0}}});
    let key = JankoKey::new(2, 1);
    let keys = json!([{"row":2,"column":1}]);
    let mut d = Driver::open();
    assert_eq!(d.send(free.clone())["status"], "ok");
    let at = cursor(&d) + 24_000;
    let played = d.send(
        json!({"operation":"torus-play","keys":keys,"amplitude":[0.2,0.0],
        "at_samples_elapsed":at.to_string()}),
    );
    assert_eq!(played["status"], "ok", "{played}");
    assert_eq!(
        played["strike_at"]["at_samples_elapsed"],
        json!(at.to_string())
    );
    let point = played["torus_play"]["notes"][0]["point"].clone();
    let struck = &played["strike_at"]["points"][0];
    assert_eq!(
        (&struck["theta"], &struck["phi"]),
        (&point["theta"], &point["phi"]),
        "the point passed to strikeAt is the note's point"
    );
    assert_eq!(
        point,
        json!(JankoSurface::new().point(key, steps_of(&point))),
        "the key's helix place carried by the pulse (POINT_LAW)"
    );
    assert_eq!(
        steps_of(&point),
        played["torus_play"]["pulse"]["steps"].as_f64().unwrap()
    );
    // The same key struck now stands elsewhere; struck once the field has run to the note's sample,
    // it stands at the note's point.
    let mut e = Driver::open();
    assert_eq!(e.send(free)["status"], "ok");
    assert_eq!(cursor(&e) + 24_000, at);
    let now = e.send(json!({"operation":"torus-play","keys":keys,"amplitude":[0.2,0.0]}));
    let now = now["torus_play"]["notes"][0]["point"].clone();
    assert!(
        (steps_of(&point) - steps_of(&now) - 20.0).abs() < 1e-9,
        "half a second at forty steps a second: {point} against {now}"
    );
    assert_ne!(now["theta"], point["theta"]);
    run_on(&mut e, 24_000);
    assert_eq!(cursor(&e), at);
    let there = e.send(json!({"operation":"torus-play","keys":keys,"amplitude":[0.2,0.0]}));
    assert_eq!(there["torus_play"]["notes"][0]["point"], point);
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_singlet_strikes_its_codons_place_on_the_ring_where_the_turning_torus_has_carried_it() {
    // O-I #771, the owner's ruling of 4 Nov 2026: a singlet's place is the arc the ring reads its codon
    // at (spanda_field::codon_advance, the epogdoon's inverse), carried by the turning torus by the
    // point law a key's place is (janko POINT_LAW); a nonet-collision codon strikes both its arcs.
    let mut d = Driver::open();
    assert_eq!(
        d.send(
            json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":40.0}}})
        )["status"],
        "ok"
    );
    let before = d.generation();
    let one = d.send(json!({"operation":"torus-singlet","codon":5,"amplitude":[0.2,0.0]}));
    assert_eq!(one["status"], "ok", "{one}");
    assert_eq!(d.generation(), before + 1, "a singlet is one strike");
    let singlet = &one["torus_singlet"];
    assert_eq!(singlet["schema"], "ql.torus-singlet/v1");
    assert_eq!(singlet["codon"], 5);
    assert_eq!(singlet["places"], json!([{"lens":0,"arc":6}]));
    let law = singlet["law"].as_str().unwrap();
    for source in ["codon_advance", "epogdoon", "POINT_LAW", "O-I #771"] {
        assert!(law.contains(source), "the law names {source}: {law}");
    }
    let point = &singlet["points"][0];
    let steps = steps_of(point);
    assert_eq!(*point, json!(point_of(HelixPlace::of_arc(6), steps)));
    assert_eq!(steps, singlet["pulse"]["steps"].as_f64().unwrap());
    let struck = one["strike_at"]["points"].as_array().unwrap();
    assert_eq!(struck.len(), 1);
    assert_eq!(
        (&struck[0]["theta"], &struck[0]["phi"]),
        (&point["theta"], &point["phi"]),
        "the point passed to strikeAt is the codon's carried place"
    );
    assert_eq!(struck[0]["amplitude"], json!([0.2, 0.0]));
    assert_eq!(
        one["strike_at"]["strike"],
        json!(0.7),
        "the body's Strike share"
    );
    // Codon 8 takes two of the nonet's nine: arcs 9 and 10 of the first lens, both struck in one act.
    let two = d.send(json!({"operation":"torus-singlet","codon":8,"amplitude":[0.2,0.0]}));
    assert_eq!(two["status"], "ok", "{two}");
    assert_eq!(d.generation(), before + 2);
    assert_eq!(
        two["torus_singlet"]["places"],
        json!([{"lens":0,"arc":9},{"lens":0,"arc":10}])
    );
    let points = two["torus_singlet"]["points"].as_array().unwrap();
    let struck = two["strike_at"]["points"].as_array().unwrap();
    assert_eq!((points.len(), struck.len()), (2, 2));
    for ((point, struck), arc) in points.iter().zip(struck).zip([9, 10]) {
        assert_eq!(*point, json!(point_of(HelixPlace::of_arc(arc), steps)));
        assert_eq!(
            (&struck["theta"], &struck["phi"]),
            (&point["theta"], &point["phi"])
        );
    }
    // A singlet for a later sample strikes where the pulse will have carried its place there.
    let at = cursor(&d) + 24_000;
    let later = d.send(
        json!({"operation":"torus-singlet","codon":63,"amplitude":[0.2,0.0],
        "at_samples_elapsed":at.to_string()}),
    );
    assert_eq!(later["status"], "ok", "{later}");
    assert_eq!(
        later["strike_at"]["at_samples_elapsed"],
        json!(at.to_string())
    );
    assert_eq!(
        later["torus_singlet"]["places"],
        json!([{"lens":5,"arc":11}])
    );
    let point = &later["torus_singlet"]["points"][0];
    assert!((steps_of(point) - steps - 20.0).abs() < 1e-9, "{point}");
    assert_eq!(
        *point,
        json!(point_of(HelixPlace::of_arc(11), steps_of(point)))
    );
    // There are 64 codons: another is refused in words, and nothing moves.
    let standing = d.generation();
    let refused = d.send(json!({"operation":"torus-singlet","codon":64,"amplitude":[0.2,0.0]}));
    assert_eq!(refused["status"], "refused", "{refused}");
    assert!(
        refused["error"].as_str().unwrap().contains("0 to 63"),
        "{refused}"
    );
    assert_eq!(d.generation(), standing);
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn tuning_and_fundamental_change_the_voice_frequencies_the_owner_reports() {
    let mut d = Driver::open();
    let voices = |reply: &Value| -> Vec<f64> {
        reply["influence"]["voices"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["frequency_hz"].as_f64().unwrap())
            .collect()
    };
    let sky = d.send(json!({"operation":"torus-set","edit":{"tuning":"sky"}}));
    assert_eq!(sky["status"], "ok", "{sky}");
    assert_eq!(voices(&sky).len(), 9);
    let logic = d.send(json!({"operation":"torus-set","edit":{"tuning":"logic"}}));
    assert_eq!(logic["status"], "ok", "{logic}");
    assert_eq!(logic["influence"]["tuning"], "logic");
    assert_eq!(voices(&logic).len(), 8);
    assert_ne!(voices(&logic), voices(&sky)[..8].to_vec());
    assert_eq!(
        logic["influence"]["boundaries"].as_array().unwrap().len(),
        4
    );
    let generation = d.generation();
    let map = voices(&logic);
    let sun = logic["torus"]["map_hz"].as_f64().unwrap();
    let tuned = d.send(
        json!({"operation":"torus-set","edit":{"fundamental":{"source":"hz","hz":2.0 * sun}}}),
    );
    assert_eq!(tuned["status"], "ok", "{tuned}");
    assert_eq!(d.generation(), generation + 1, "one modes continuation");
    for (a, b) in map.iter().zip(voices(&tuned)) {
        assert!((b - 2.0 * a).abs() < 1e-3, "{b} vs {a}");
    }
}

/// The PCM of `seconds` of the field, rendered in blocks of 4800 samples.
/// Each voice's state as the doubles it is, in mode order.
fn metres(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|pair| pair.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()))
        .collect()
}

fn render(d: &mut Driver, seconds: f64) -> Vec<f64> {
    let mut pcm = Vec::new();
    for _ in 0..(seconds * 10.0).round() as usize {
        let advanced = d.send(json!({"operation":"advance","frames":4800,"muted":false}));
        assert_eq!(advanced["status"], "ok", "{advanced}");
        pcm.extend(
            advanced["field"]["audio"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap()),
        );
    }
    pcm
}

fn rms(x: &[f64]) -> f64 {
    (x.iter().map(|v| v * v).sum::<f64>() / x.len() as f64).sqrt()
}

/// The normalised correlation of the signal with itself `lag` samples on.
fn correlation(x: &[f64], lag: usize) -> f64 {
    let (a, b) = (&x[..x.len() - lag], &x[lag..]);
    let dot: f64 = a.iter().zip(b).map(|(p, q)| p * q).sum();
    dot / (rms(a) * rms(b) * a.len() as f64)
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn lowering_the_rate_through_the_bands_is_heard_a_note_becoming_a_beat() {
    // Manual §21.2: "One pulse makes every sound of the Torus"; Carrier is how strongly the pulse
    // drives the body between strikes, and at 0 % the body sounds only when something strikes it.
    // At pitch rates the carrier sounds a note at the pulse's rate; at rhythm rates it beats.
    let mut d = Driver::open();
    // Quiet the opening ring (a damping edit re-excites nothing), then ring briefly: 40 a second.
    assert_eq!(
        d.send(json!({"operation":"set-damping","per_second":4000.0}))["status"],
        "ok"
    );
    render(&mut d, 0.1);
    assert_eq!(
        d.send(json!({"operation":"set-damping","per_second":40.0}))["status"],
        "ok"
    );
    let silent = render(&mut d, 1.0);
    assert!(
        silent.iter().all(|v| v.abs() < 1e-12),
        "at 0 % with no strike the body is silent"
    );
    // A note: 240 steps a second (200 samples a step at 48 kHz), Carrier 50 %.
    let note = d.send(json!({"operation":"torus-set","edit":{
        "carrier":0.5,"rate":{"mode":"free","steps_per_second":240.0}}}));
    assert_eq!(note["status"], "ok", "{note}");
    assert_eq!(note["torus"]["carrier"]["drive"], "pulse");
    let tone = render(&mut d, 0.5);
    let steady = &tone[12_000..];
    assert!(
        rms(steady) > 1e-4,
        "the carrier sounds: rms {}",
        rms(steady)
    );
    let at_step = correlation(steady, 200);
    assert!(at_step > 0.95, "periodic at the pulse's step: {at_step}");
    assert!(
        correlation(steady, 150) < at_step - 0.1,
        "and at the step, not any lag"
    );
    // A beat: lower the rate into Rhythm, four steps a second.
    let beat = d.send(
        json!({"operation":"torus-set","edit":{"rate":{"mode":"free","steps_per_second":4.0}}}),
    );
    assert_eq!(beat["torus"]["band"], "rhythm");
    let beats = render(&mut d, 1.6);
    let envelope: Vec<f64> = beats[4_800..].chunks(240).map(rms).collect();
    let peak = envelope.iter().cloned().fold(0.0, f64::max);
    assert!(peak > 1e-4, "each step strikes the body: peak {peak}");
    let onsets = envelope
        .windows(2)
        .filter(|w| w[0] < 0.3 * peak && w[1] >= 0.3 * peak)
        .count();
    assert!(
        (5..=7).contains(&onsets),
        "a beat four times a second: {onsets} onsets in 1.5 s"
    );
    let quietest = envelope.iter().cloned().fold(f64::INFINITY, f64::min);
    assert!(
        quietest < 0.01 * peak,
        "the body falls quiet between beats: {quietest} of {peak}"
    );
    // Back to 0 %: the drive stops; the tail decays, and nothing sounds again unstruck.
    d.send(json!({"operation":"torus-set","edit":{"carrier":0.0}}));
    let after = render(&mut d, 1.0);
    assert!(
        after[24_000..].iter().all(|v| v.abs() < 1e-6 * peak),
        "the tail decays and nothing drives it"
    );
}

/// O-I 855: a Live take begins again where it began. The Torus is kept at a
/// sample (its body as it rings, its M1′ session, the scene's event with its
/// seeds, the body's damping) and set again later: from there it rings on as it
/// rang on, bit for bit, while the cursor runs on. A damping moved since comes
/// back as one modes continuation. A checkpoint that cannot stand is refused
/// in words and changes nothing.
#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_torus_kept_at_a_sample_rings_on_from_its_restore_as_it_rang() {
    let mut d = Driver::open();
    let played = d.send(
        json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0]}),
    );
    assert_eq!(played["status"], "ok", "{played}");
    render(&mut d, 0.3);
    let generation = d.generation();
    let kept = d.send(json!({"operation":"torus-checkpoint"}));
    assert_eq!(kept["status"], "ok", "{kept}");
    assert_eq!(d.generation(), generation, "keeping the Torus is a read");
    let checkpoint = kept["torus_checkpoint"].clone();
    assert_eq!(checkpoint["schema"], "ql.torus-checkpoint/v1");
    assert_eq!(
        checkpoint["taken_at_samples"],
        kept["field"]["samples_elapsed"]
    );
    // The same doubles (the owner's text keeps every digit the worker wrote; the checkpoint's is the shortest).
    assert_eq!(
        metres(&checkpoint["amplitudes"]),
        metres(&kept["field"]["amplitudes_metres"])
    );
    assert_eq!(checkpoint["session"]["notes"].as_array().unwrap().len(), 1);
    assert!(
        checkpoint["seeds"].is_object()
            || checkpoint["seeds"].is_array()
            || checkpoint["seeds"].is_string(),
        "{checkpoint}"
    );
    let then = render(&mut d, 0.3);

    // The Torus moves on: another key, a setting, the body's damping, time.
    for command in [
        json!({"operation":"torus-play","keys":[{"row":4,"column":5}],"amplitude":[0.3,0.0]}),
        json!({"operation":"torus-set","edit":{"polyphony":3}}),
        json!({"operation":"set-damping","per_second":7.5}),
    ] {
        let answer = d.send(command.clone());
        assert_eq!(answer["status"], "ok", "{command}: {answer}");
    }
    render(&mut d, 0.5);

    // Refused in words, changing nothing.
    let standing = d.current.clone();
    for (path, value, words) in [
        ("/schema", json!("ql.torus-checkpoint/v2"), "checkpoint"),
        ("/sample_rate", json!(44100), "sample rate"),
        ("/amplitudes/0/0", json!(2e6), "state"),
        ("/voices/0", json!("mode:another"), "voices"),
        ("/session/steps", json!(-1.0), "steps"),
    ] {
        let mut bad = checkpoint.clone();
        *bad.pointer_mut(path).unwrap() = value;
        let refused = d.send(json!({"operation":"torus-restore","checkpoint":bad}));
        assert_eq!(refused["status"], "refused", "{path}: {refused}");
        assert!(
            refused["error"].as_str().unwrap().contains(words),
            "{path}: {refused}"
        );
        assert_eq!(
            refused["field"]["generation"], standing["field"]["generation"],
            "{path}"
        );
        assert_eq!(
            metres(&refused["field"]["amplitudes_metres"]),
            metres(&standing["field"]["amplitudes_metres"]),
            "{path}"
        );
    }

    let generation = d.generation();
    let restored = d.send(json!({"operation":"torus-restore","checkpoint":checkpoint}));
    assert_eq!(restored["status"], "ok", "{restored}");
    assert_eq!(
        d.generation(),
        generation + 2,
        "the damping, then the body: two acts"
    );
    assert_eq!(
        metres(&restored["field"]["amplitudes_metres"]),
        metres(&checkpoint["amplitudes"])
    );
    assert_eq!(
        restored["torus"]["settings"],
        checkpoint["session"]["settings"]
    );
    assert_eq!(
        restored["torus"]["notes"].as_array().unwrap().len(),
        1,
        "{restored}"
    );
    assert_eq!(
        restored["torus"]["comma_cents"],
        kept["torus"]["comma_cents"]
    );
    let again = render(&mut d, 0.3);
    assert_eq!(again, then, "it rings on as it rang");
}

/// The live owner's PCM from its cursor for `frames`, in blocks of `block`.
fn render_frames(d: &mut Driver, frames: u64, block: u64) -> Vec<f64> {
    let mut pcm = Vec::new();
    let mut done = 0;
    while done < frames {
        let n = block.min(frames - done);
        let advanced = d.send(json!({"operation":"advance","frames":n,"muted":false}));
        assert_eq!(advanced["status"], "ok", "{advanced}");
        pcm.extend(
            advanced["field"]["audio"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap()),
        );
        done += n;
    }
    pcm
}

fn audio(render: &Value) -> Vec<f64> {
    render["audio"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_take_renders_offline_from_its_checkpoint_as_the_owner_played_it() {
    // O-I #855: Export Audio takes a Live take through the M1′ session offline. The take is a checkpoint (the Torus as it rang when
    // the take began) and its acts at the samples they landed at; the render is a second owner of the same scene, set to the
    // checkpoint, each act made at its sample, its sound the owner's own. The live owner neither moves nor waits for it.
    let mut d = Driver::open();
    let played = d.send(
        json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0]}),
    );
    assert_eq!(played["status"], "ok", "{played}");
    render(&mut d, 0.2);
    let kept = d.send(json!({"operation":"torus-checkpoint"}));
    let checkpoint = kept["torus_checkpoint"].clone();
    let from: u64 = checkpoint["taken_at_samples"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let at = |offset: u64| (from + offset).to_string();
    // The take: a key ahead of the cursor, a release, a setting and a chord, as the live owner takes them.
    let acts = json!([
        {"action":"strike","at_samples_elapsed":at(4800),"keys":[{"row":2,"column":3}],"amplitude":[0.3,0.0]},
        {"action":"release","at_samples_elapsed":at(9600),"keys":[{"row":0,"column":2}]},
        {"action":"set","at_samples_elapsed":at(9600),"edit":{"polyphony":2}},
        {"action":"strike","at_samples_elapsed":at(14400),"keys":[{"row":4,"column":5},{"row":1,"column":1}],"amplitude":[0.25,0.0]},
    ]);
    let mut live = Vec::new();
    let strike = d.send(json!({"operation":"torus-play","keys":[{"row":2,"column":3}],"amplitude":[0.3,0.0],"at_samples_elapsed":at(4800)}));
    assert_eq!(strike["status"], "ok", "{strike}");
    live.extend(render_frames(&mut d, 9600, 4800));
    for command in [
        json!({"operation":"torus-release","keys":[{"row":0,"column":2}]}),
        json!({"operation":"torus-set","edit":{"polyphony":2}}),
        json!({"operation":"torus-play","keys":[{"row":4,"column":5},{"row":1,"column":1}],"amplitude":[0.25,0.0],"at_samples_elapsed":at(14400)}),
    ] {
        let answer = d.send(command.clone());
        assert_eq!(answer["status"], "ok", "{command}: {answer}");
    }
    let made_way = d.current["torus_play"]["made_way"].clone();
    live.extend(render_frames(&mut d, 14400, 4800));
    let frames = 24000u64;
    assert_eq!(live.len() as u64, frames);

    let standing = d.current.clone();
    let whole = d.send(
        json!({"operation":"torus-render","checkpoint":checkpoint,"acts":acts,"frames":frames}),
    );
    assert_eq!(whole["status"], "ok", "{whole}");
    assert_eq!(
        whole["field"]["generation"], standing["field"]["generation"],
        "the live owner does not move"
    );
    assert_eq!(
        whole["field"]["samples_elapsed"],
        standing["field"]["samples_elapsed"]
    );
    assert_eq!(whole["field"]["audio"], json!([]), "nor sound");
    let taken = &whole["torus_render"];
    assert_eq!(taken["schema"], "ql.torus-render/v1");
    assert_eq!(taken["from_samples"], json!(from.to_string()));
    assert_eq!(taken["frames"], json!(frames));
    assert_eq!(taken["sample_rate"], d.current["field"]["sample_rate"]);
    assert_eq!(
        taken["made_way"],
        json!([[], made_way]),
        "the notes that made way, strike by strike, as the owner made them"
    );
    assert!(
        !made_way.as_array().unwrap().is_empty(),
        "past Polyphony 2 the oldest note made way"
    );
    let pcm = audio(taken);
    assert_eq!(pcm.len() as u64, frames);
    assert!(pcm.iter().any(|x| *x != 0.0), "the take sounds");
    assert_eq!(
        pcm, live,
        "sample for sample, the sound the live owner made"
    );
    assert_eq!(
        taken["checkpoint"]["taken_at_samples"],
        json!(at(frames)),
        "and where it ends, kept as the owner keeps it"
    );

    // In parts: a continuation runs on the render owner; an end checkpoint opens another the same.
    let first = d.send(json!({"operation":"torus-render","checkpoint":checkpoint,"acts":acts.as_array().unwrap()[..1],"frames":9600}));
    assert_eq!(first["status"], "ok", "{first}");
    let token = first["torus_render"]["continues"]
        .as_str()
        .unwrap()
        .to_owned();
    let second = d.send(json!({"operation":"torus-render","continues":token,"acts":acts.as_array().unwrap()[1..],"frames":frames - 9600}));
    assert_eq!(second["status"], "ok", "{second}");
    let mut parts = audio(&first["torus_render"]);
    parts.extend(audio(&second["torus_render"]));
    assert_eq!(parts, live, "a continued render is the whole render");
    let again = d.send(json!({"operation":"torus-render","checkpoint":first["torus_render"]["checkpoint"],"acts":acts.as_array().unwrap()[1..],"frames":frames - 9600}));
    assert_eq!(again["status"], "ok", "{again}");
    assert_eq!(
        audio(&again["torus_render"]),
        audio(&second["torus_render"]),
        "the end checkpoint keeps the body exactly"
    );

    // Refused in words, the live owner untouched.
    for (command, words) in [
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[],"frames":0}),
            "frames",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[],"frames":(1u64 << 20) + 1}),
            "frames",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[{"action":"release","at_samples_elapsed":(from - 1).to_string(),"keys":[{"row":0,"column":2}]}],"frames":4800}),
            "before",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[{"action":"release","at_samples_elapsed":at(4800),"keys":[{"row":0,"column":2}]}],"frames":4800}),
            "after",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"acts":[
            {"action":"release","at_samples_elapsed":at(200),"keys":[{"row":0,"column":2}]},
            {"action":"release","at_samples_elapsed":at(100),"keys":[{"row":0,"column":2}]}],"frames":4800}),
            "order",
        ),
        (
            json!({"operation":"torus-render","checkpoint":checkpoint,"continues":token,"acts":[],"frames":4800}),
            "either",
        ),
        (
            json!({"operation":"torus-render","acts":[],"frames":4800}),
            "either",
        ),
        (
            json!({"operation":"torus-render","continues":"render:0","acts":[],"frames":4800}),
            "continues",
        ),
        (
            json!({"operation":"torus-render","checkpoint":{"schema":"ql.torus-checkpoint/v2"},"acts":[],"frames":4800}),
            "checkpoint",
        ),
    ] {
        let refused = d.send(command.clone());
        assert_eq!(refused["status"], "refused", "{command}: {refused}");
        assert!(
            refused["error"].as_str().unwrap().contains(words),
            "{words}: {refused}"
        );
        assert_eq!(
            refused["field"]["generation"],
            standing["field"]["generation"]
        );
        assert_eq!(
            refused["field"]["samples_elapsed"],
            standing["field"]["samples_elapsed"]
        );
    }
}

fn cursor(d: &Driver) -> u64 {
    d.current["field"]["samples_elapsed"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap()
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_setting_lands_at_its_sample_inside_a_block_as_a_strike_does() {
    // O-I 855: an edit of the M1′ session sent ahead for a later sample is made at that sample inside the block that reaches it,
    // as a strike is, so the frame no longer waits for the cursor. Its sound is sample for sample the edit made between two blocks
    // split at that sample, and the render of the same take; a strike for a later sample reads the settings in force there.
    let edit = json!({"fundamental":{"source":"hz","hz":261.6},"carrier":0.5,"rate":{"mode":"free","steps_per_second":3.0}});
    let strike = |at: u64| json!({"operation":"torus-play","keys":[{"row":2,"column":3}],"amplitude":[0.3,0.0],"at_samples_elapsed":at.to_string()});
    let opened = || {
        let mut d = Driver::open();
        let played = d.send(
            json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0]}),
        );
        assert_eq!(played["status"], "ok", "{played}");
        render_frames(&mut d, 2048, 512);
        d
    };
    let frames = 4096u64;
    let mut timed = opened();
    let from = cursor(&timed);
    let kept = timed.send(json!({"operation":"torus-checkpoint"}))["torus_checkpoint"].clone();
    // Inside the second block of 512, and a strike after it.
    let (at, struck_at) = (from + 700, from + 1000);
    let before = timed.generation();
    let waits = timed
        .send(json!({"operation":"torus-set","edit":edit,"at_samples_elapsed":at.to_string()}));
    assert_eq!(waits["status"], "ok", "{waits}");
    assert_eq!(
        timed.generation(),
        before,
        "a setting for a later sample moves nothing yet"
    );
    assert_eq!(waits["field"]["samples_elapsed"], json!(from.to_string()));
    assert_eq!(
        waits["torus"]["settings"]["carrier"],
        json!(0.0),
        "the settings in force are still the ones before it"
    );
    assert_eq!(
        waits["torus"]["waiting"],
        json!([{"at_samples_elapsed": at.to_string(), "edit": edit}]),
        "the reading names the setting that waits and its sample"
    );
    let played = timed.send(strike(struck_at));
    assert_eq!(played["status"], "ok", "{played}");
    let mut live = Vec::new();
    let mut landings = Vec::new();
    for _ in 0..frames / 512 {
        let advanced = timed.send(json!({"operation":"advance","frames":512,"muted":false}));
        assert_eq!(advanced["status"], "ok", "{advanced}");
        live.extend(
            advanced["field"]["audio"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap()),
        );
        if advanced.get("torus_set_landed").is_some() {
            landings.push(advanced);
        }
    }
    assert_eq!(landings.len(), 1, "one block made the setting");
    let landed = &landings[0];
    assert_eq!(
        landed["field"]["samples_elapsed"],
        json!((from + 1024).to_string()),
        "the block that reached its sample"
    );
    let made = &landed["torus_set_landed"][0];
    assert_eq!(made["at_samples_elapsed"], json!(at.to_string()));
    assert_eq!(made["applied"], json!(true), "{made}");
    for moved in ["fundamental", "carrier", "rate"] {
        assert!(
            made["moved"].as_array().unwrap().contains(&json!(moved)),
            "{moved}: {made}"
        );
    }
    assert_eq!(landed["torus"]["settings"]["carrier"], json!(0.5));
    assert_eq!(landed["torus"]["waiting"], json!([]));
    assert!(
        landed["influence"].is_object(),
        "the voices' new frequencies come with it"
    );
    assert_eq!(
        timed.generation(),
        before + 2,
        "the strike, and the Fundamental's one modes continuation where it landed"
    );

    // The split path: the same edit made between two blocks split at its sample, then the strike.
    let mut split = opened();
    assert_eq!(cursor(&split), from);
    let mut heard = render_frames(&mut split, at - from, 512);
    let set = split.send(json!({"operation":"torus-set","edit":edit}));
    assert_eq!(set["status"], "ok", "{set}");
    let played_after = split.send(strike(struck_at));
    assert_eq!(played_after["status"], "ok", "{played_after}");
    assert_eq!(
        played["torus_play"]["notes"], played_after["torus_play"]["notes"],
        "the strike read the settings in force at its sample: the same points, places and cells"
    );
    assert_eq!(
        played["torus_play"]["pulse"],
        played_after["torus_play"]["pulse"]
    );
    heard.extend(render_frames(&mut split, frames - (at - from), 512));
    assert_eq!(
        live, heard,
        "sample for sample, the edit made at its sample"
    );
    assert_eq!(timed.generation(), split.generation());
    assert_eq!(
        timed.current["torus"]["settings"],
        split.current["torus"]["settings"]
    );
    assert_eq!(
        timed.current["torus"]["notes"],
        split.current["torus"]["notes"]
    );
    assert_eq!(
        timed.current["torus"]["pulse"],
        split.current["torus"]["pulse"]
    );

    // Nothing moves before its sample: without the setting, the same strike sounds the same until it, and differs after.
    let mut control = opened();
    assert_eq!(control.send(strike(struck_at))["status"], "ok");
    let unset = render_frames(&mut control, frames, 512);
    let k = usize::try_from(at - from).unwrap();
    assert_eq!(
        live[..k],
        unset[..k],
        "before its sample the sound is the sound without it"
    );
    assert_ne!(
        live[k..],
        unset[k..],
        "from its sample the setting is heard"
    );

    // A render of the same take from its checkpoint is the same sound.
    let take = timed.send(json!({"operation":"torus-render","checkpoint":kept,"frames":frames,"acts":[
        {"action":"set","at_samples_elapsed":at.to_string(),"edit":edit},
        {"action":"strike","at_samples_elapsed":struck_at.to_string(),"keys":[{"row":2,"column":3}],"amplitude":[0.3,0.0]}]}));
    assert_eq!(take["status"], "ok", "{take}");
    assert_eq!(
        audio(&take["torus_render"]),
        live,
        "the take renders as the owner played it"
    );
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn a_setting_for_a_later_sample_is_refused_in_words_where_it_cannot_land() {
    let mut d = Driver::open();
    render_frames(&mut d, 1024, 512);
    let now = cursor(&d);
    let set = |at: String, edit: Value| json!({"operation":"torus-set","edit":edit,"at_samples_elapsed":at});
    let refused = |d: &mut Driver, command: Value, words: &str| {
        let standing = d.current.clone();
        let answer = d.send(command.clone());
        assert_eq!(answer["status"], "refused", "{command}: {answer}");
        assert!(
            answer["error"].as_str().unwrap().contains(words),
            "{words}: {answer}"
        );
        assert_eq!(
            answer["field"]["generation"],
            standing["field"]["generation"]
        );
        assert_eq!(
            answer["field"]["samples_elapsed"],
            standing["field"]["samples_elapsed"]
        );
        assert_eq!(
            answer["torus"]["waiting"], standing["torus"]["waiting"],
            "{command}"
        );
    };
    // A first read gives the reading the refusals are judged against.
    assert_eq!(d.send(json!({"operation":"torus-state"}))["status"], "ok");
    refused(
        &mut d,
        set((now - 1).to_string(), json!({"polyphony":3})),
        "behind the owner's cursor",
    );
    refused(
        &mut d,
        set(format!("0{now}"), json!({"polyphony":3})),
        "exact sample cursor",
    );
    refused(
        &mut d,
        set((now + 10).to_string(), json!({})),
        "at least one setting",
    );
    refused(
        &mut d,
        set((now + 10).to_string(), json!({"polyphony":0})),
        "Polyphony",
    );

    // At its own sample the setting is made now, as a strike at the cursor is.
    let made = d.send(set(now.to_string(), json!({"polyphony":4})));
    assert_eq!(made["status"], "ok", "{made}");
    assert_eq!(made["torus"]["settings"]["polyphony"], json!(4));
    assert_eq!(made["torus"]["waiting"], json!([]));

    // A setting cannot land before a strike already played for a later sample: that strike read the settings before it.
    let played = d.send(json!({"operation":"torus-play","keys":[{"row":0,"column":2}],"amplitude":[0.2,0.0],"at_samples_elapsed":(now + 2048).to_string()}));
    assert_eq!(played["status"], "ok", "{played}");
    refused(
        &mut d,
        set((now + 1000).to_string(), json!({"polyphony":3})),
        "before the strike already played",
    );
    let after = d.send(set((now + 2048).to_string(), json!({"polyphony":3})));
    assert_eq!(
        after["status"], "ok",
        "at the strike's own sample it lands after it: {after}"
    );

    // Settings wait in the order of their samples, however they were sent, and at most 64 of them.
    for i in (0..63u64).rev() {
        let answer = d.send(set(
            (now + 3000 + i).to_string(),
            json!({"polyphony": 2 + (i % 3) as u8}),
        ));
        assert_eq!(answer["status"], "ok", "{i}: {answer}");
    }
    let waiting = d.current["torus"]["waiting"].as_array().unwrap().clone();
    assert_eq!(waiting.len(), 64);
    let samples: Vec<u64> = waiting
        .iter()
        .map(|w| w["at_samples_elapsed"].as_str().unwrap().parse().unwrap())
        .collect();
    assert!(samples.windows(2).all(|w| w[0] <= w[1]), "{samples:?}");
    refused(
        &mut d,
        set((now + 5000).to_string(), json!({"polyphony":3})),
        "at most 64",
    );

    // A setting at the end of a block lands as the cursor reaches it; the rest land in their blocks, in order.
    let mut landed = Vec::new();
    for block in 1..=8u64 {
        let advanced = d.send(json!({"operation":"advance","frames":512,"muted":false}));
        assert_eq!(advanced["status"], "ok", "{advanced}");
        let made = advanced["torus_set_landed"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        if block == 4 {
            assert_eq!(
                made.len(),
                1,
                "the setting at the block's end lands with it: {advanced}"
            );
            assert_eq!(advanced["torus"]["settings"]["polyphony"], json!(3));
        }
        landed.extend(made);
    }
    assert_eq!(landed.len(), 64);
    let at: Vec<String> = landed
        .iter()
        .map(|l| l["at_samples_elapsed"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(at[0], (now + 2048).to_string());
    assert!(
        landed.iter().all(|l| l["applied"] == json!(true)),
        "{landed:?}"
    );
    let state = d.send(json!({"operation":"torus-state"}));
    assert_eq!(state["torus"]["waiting"], json!([]));
    assert_eq!(
        state["torus"]["settings"]["polyphony"],
        json!(2 + (62 % 3)),
        "the last made is in force"
    );
}

// The Torus is heard where you stand (O-I 856; QL-MEF #289 packet N): manual §21.2 View, "Receiver On makes your camera the place
// where the torus is heard", and Rules, "A camera move does not change the sound, unless Receiver is On". Receiver Off is the world's
// context, heard by the declared gain wherever the camera is; On is the listener's own, heard at the torus where they stand.

/// The chord every test below plays: three keys of the Jankó surface.
fn chord() -> Value {
    json!({"operation":"torus-play","keys":[{"row":0,"column":2},{"row":3,"column":1},{"row":5,"column":4}],"amplitude":[0.2,0.0]})
}

/// The gains a `torus-receiver` answer says the body is heard by, by voice.
fn heard(reply: &Value) -> Vec<(String, f64)> {
    reply["torus_receiver"]["gains"]
        .as_array()
        .unwrap_or_else(|| panic!("{reply}"))
        .iter()
        .map(|g| {
            (
                g["mode_ref"].as_str().unwrap().to_owned(),
                g["gain"].as_f64().unwrap(),
            )
        })
        .collect()
}

/// A place in the scene's metres (the targets' frame, metres_per_unit 1): `beyond` units out from the torus's outer equator at
/// longitude `phi` of the frame (the body turns by its clock under it), at height 0.
fn standing(beyond: f64, phi: f64) -> Value {
    let out = 16.0 / 9.0 + 1.0 + beyond;
    json!({"at_metres":[out * phi.cos(), out * phi.sin(), 0.0]})
}

/// The body as the owner reports it: everything but the sound.
fn body(reply: &Value) -> Value {
    let field = &reply["field"];
    json!({"generation":field["generation"], "samples_elapsed":field["samples_elapsed"], "clock":field["clock"],
        "amplitudes":field["amplitudes_metres"], "targets":field["targets"], "event_ref":field["event_ref"]})
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn with_the_receiver_off_a_camera_move_leaves_the_sound_unchanged() {
    let (mut control, mut moved) = (Driver::open(), Driver::open());
    for d in [&mut control, &mut moved] {
        assert_eq!(d.send(chord())["status"], "ok");
    }
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for _ in 0..12 {
        // Off is the world's context: saying so again, as a camera moves, asks nothing of the body and hears it as declared.
        let off = moved.send(json!({"operation":"torus-receiver","receiver":null}));
        assert_eq!(off["status"], "ok", "{off}");
        assert_eq!(off["torus_receiver"]["context"], "world");
        let gains = heard(&off);
        assert!(!gains.is_empty());
        assert!(
            gains.iter().all(|(_, g)| *g == 1.0),
            "the declared gain, 1 per metre: {off}"
        );
        assert_eq!(
            body(&off),
            body(&control.current),
            "nothing of the body moves"
        );
        a.extend(render_frames(&mut control, 512, 512));
        b.extend(render_frames(&mut moved, 512, 512));
    }
    assert_eq!(a, b, "the renders are the same, sample for sample");
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn with_the_receiver_on_the_sound_follows_where_you_stand() {
    use std::f64::consts::{FRAC_PI_2, PI, TAU};
    let opened = || {
        let mut d = Driver::open();
        assert_eq!(d.send(chord())["status"], "ok");
        render_frames(&mut d, 1024, 512);
        d
    };
    let mut owners: Vec<Driver> = (0..6).map(|_| opened()).collect();
    let [world, near, aside, far, farther, probe] = &mut owners[..] else {
        unreachable!()
    };
    let generation = near.generation();
    let at_near = near.send(json!({"operation":"torus-receiver","receiver":standing(0.25, 0.0)}));
    let at_aside =
        aside.send(json!({"operation":"torus-receiver","receiver":standing(0.25, FRAC_PI_2)}));
    let at_far = far.send(json!({"operation":"torus-receiver","receiver":standing(1000.0, 0.0)}));
    let at_farther =
        farther.send(json!({"operation":"torus-receiver","receiver":standing(2000.0, 0.0)}));
    for reply in [&at_near, &at_aside, &at_far, &at_farther] {
        assert_eq!(reply["status"], "ok", "{reply}");
        assert_eq!(reply["torus_receiver"]["context"], "personal");
        assert_eq!(
            body(reply),
            body(&world.current),
            "the place moves only what is heard"
        );
        // The place is the listener's: the answer says how the body is heard, never where from.
        let mut keys: Vec<&str> = reply["torus_receiver"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(keys, ["context", "gains", "law", "schema"], "{reply}");
        assert_eq!(reply["torus_receiver"]["schema"], "ql.torus-receiver/v1");
        assert!(!reply.to_string().contains("at_metres"), "{reply}");
    }
    assert_eq!(
        near.generation(),
        generation,
        "a receiving setting, not a body act"
    );

    // Far off, every voice is heard as declared (1 per metre), quieter by the tube radius over the distance.
    for (reply, beyond) in [(&at_far, 1000.0), (&at_farther, 2000.0)] {
        for (mode, gain) in heard(reply) {
            assert!(
                (gain * beyond - 1.0).abs() < 1e-5,
                "{mode} at {beyond}: {gain}"
            );
        }
    }
    // Near, the mix is the surface's under you: the voices are heard unequally, and another place hears another mix.
    let (near_gains, aside_gains) = (heard(&at_near), heard(&at_aside));
    let (low, high) = near_gains
        .iter()
        .fold((f64::INFINITY, 0.0f64), |(l, h), (_, g)| {
            (l.min(*g), h.max(*g))
        });
    assert!(
        high - low > 0.1,
        "near, the voices are heard unequally: {near_gains:?}"
    );
    assert!(
        near_gains
            .iter()
            .zip(&aside_gains)
            .any(|((_, n), (_, a))| (n - a).abs() > 1e-3),
        "another place on the torus hears another mix"
    );

    // The law, stated over the owner's own shape terms: χ of every voice of the tuning in force at every sample of the 32 × 16 sheet,
    // as a strike there meets it. Each voice is heard by the root mean square of χ near the listener (a Gaussian in each sample's
    // distance beyond the nearest, as wide as the listener stands off the surface and never narrower than the sampling's spacing) over
    // its root mean square over the whole sheet, and beyond one tube radius by the radius over the distance.
    let sheet: Vec<(f64, f64)> = (0..16)
        .flat_map(|j| (0..32).map(move |i| (TAU * f64::from(i) / 32.0, TAU * f64::from(j) / 16.0)))
        .collect();
    let mut chi: Vec<Vec<f64>> = Vec::new();
    let mut modes: Vec<String> = Vec::new();
    for part in sheet.chunks(64) {
        let points: Vec<Value> = part
            .iter()
            .map(|(phi, theta)| json!({"theta":theta,"phi":phi,"amplitude":[1e-6,0.0]}))
            .collect();
        let met = probe.send(json!({"operation":"strike-at","points":points}));
        assert_eq!(met["status"], "ok", "{met}");
        let voices = met["strike_at"]["voices"].as_array().unwrap();
        modes = voices
            .iter()
            .map(|v| v["mode_ref"].as_str().unwrap().to_owned())
            .collect();
        for p in 0..part.len() {
            chi.push(
                voices
                    .iter()
                    .map(|v| v["coupling"][p].as_f64().unwrap())
                    .collect(),
            );
        }
    }
    let law = |q: [f64; 3]| -> Vec<f64> {
        let major = 16.0 / 9.0;
        let beyond = (q[0].hypot(q[1]) - major).hypot(q[2]) - 1.0;
        let width = beyond.max(TAU * (major + 1.0) / 32.0);
        let d: Vec<f64> = sheet
            .iter()
            .map(|(phi, theta)| {
                let r = major + theta.cos();
                (q[0] - r * phi.cos())
                    .hypot(q[1] - r * phi.sin())
                    .hypot(q[2] - theta.sin())
            })
            .collect();
        let nearest = d.iter().copied().fold(f64::INFINITY, f64::min);
        let w: Vec<f64> = d
            .iter()
            .map(|d| (-0.5 * ((d - nearest) / width).powi(2)).exp())
            .collect();
        let total: f64 = w.iter().sum();
        (0..modes.len())
            .map(|k| {
                let near = (w
                    .iter()
                    .zip(&chi)
                    .map(|(w, c)| w * c[k] * c[k])
                    .sum::<f64>()
                    / total)
                    .sqrt();
                let whole =
                    (chi.iter().map(|c| c[k] * c[k]).sum::<f64>() / chi.len() as f64).sqrt();
                near / whole * if beyond > 1.0 { 1.0 / beyond } else { 1.0 }
            })
            .collect()
    };
    for (beyond, phi, gains) in [(0.25, 0.0, &near_gains), (0.25, FRAC_PI_2, &aside_gains)] {
        let at: [f64; 3] =
            serde_json::from_value(standing(beyond, phi)["at_metres"].clone()).unwrap();
        for (mode, want) in modes.iter().zip(law(at)) {
            let (_, gain) = gains.iter().find(|(m, _)| m == mode).unwrap();
            assert!(
                (gain - want).abs() < 1e-9 * want.max(1.0),
                "{mode} at φ {phi}: {gain} against the law's {want}"
            );
        }
    }

    // The block after a move reaches the new gains; from then on the sound is the place's, and the body is the same in all.
    let pcm = |d: &mut Driver| {
        render_frames(d, 512, 512);
        render_frames(d, 4096, 512)
    };
    let (w, n, a, f, f2) = (pcm(world), pcm(near), pcm(aside), pcm(far), pcm(farther));
    for d in [&*near, &*aside, &*far, &*farther] {
        assert_eq!(body(&d.current), body(&world.current));
    }
    let peak = f.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    assert!(peak > 0.0);
    assert!(
        f.iter()
            .zip(&f2)
            .all(|(f, f2)| (f2 - 0.5 * f).abs() <= 1e-5 * peak),
        "twice as far, half as loud, sample for sample"
    );
    assert_ne!(n, w, "standing there is not the world's sound");
    let k =
        a.iter().zip(&n).map(|(a, n)| a * n).sum::<f64>() / n.iter().map(|n| n * n).sum::<f64>();
    let rest: Vec<f64> = a.iter().zip(&n).map(|(a, n)| a - k * n).collect();
    assert!(
        rms(&rest) > 1e-3 * rms(&a),
        "aside is another mix, not the same one louder"
    );

    // The body turns under the frame with its clock: a place is heard as that place turned back by the clock's angle.
    let (mut turned, mut unturned) = (opened(), opened());
    let set = turned
        .send(json!({"operation":"set-axis","axis":0,"phase":{"turns":"0","half_degrees":90}}));
    assert_eq!(set["status"], "ok", "{set}");
    assert_eq!(
        set["field"]["clock"]["inscription"]["half_degrees"],
        json!(90)
    );
    let place = standing(0.5, 0.3)["at_metres"].clone();
    let [x, y, z]: [f64; 3] = serde_json::from_value(place.clone()).unwrap();
    let (c, s) = ((PI / 4.0).cos(), (PI / 4.0).sin());
    let back = json!({"at_metres":[c * x + s * y, -s * x + c * y, z]});
    let by_turn =
        heard(&turned.send(json!({"operation":"torus-receiver","receiver":{"at_metres":place}})));
    let by_back = heard(&unturned.send(json!({"operation":"torus-receiver","receiver":back})));
    assert_eq!(by_turn.len(), by_back.len());
    for ((mode, a), (_, b)) in by_turn.iter().zip(&by_back) {
        assert!((a - b).abs() < 1e-9, "{mode}: {a} turned against {b}");
    }
}

#[test]
#[ignore = "needs QL_FIELD_WORKER"]
fn one_performance_in_two_contexts_differs_only_as_declared_and_the_saved_sound_is_the_worlds() {
    let (mut world, mut mine) = (Driver::open(), Driver::open());
    let both = |w: &mut Driver, m: &mut Driver, command: Value| {
        let (a, b) = (w.send(command.clone()), m.send(command.clone()));
        assert_eq!(a["status"], "ok", "{command}: {a}");
        assert_eq!(body(&a), body(&b), "{command}");
        assert_eq!(a["torus"], b["torus"], "{command}");
        assert_eq!(a["influence"], b["influence"], "{command}");
    };
    both(&mut world, &mut mine, chord());
    let block = |w: &mut Driver, m: &mut Driver| {
        let (a, b) = (render_frames(w, 512, 512), render_frames(m, 512, 512));
        assert_eq!(body(&w.current), body(&m.current));
        (a, b)
    };
    let (a, b) = block(&mut world, &mut mine);
    assert_eq!(a, b, "one context, one sound");

    // Switching context is the explicit act; the performance goes on the same in both.
    let on = mine.send(json!({"operation":"torus-receiver","receiver":standing(1.5, 0.7)}));
    assert_eq!(on["status"], "ok", "{on}");
    let mut differed = false;
    for step in 0..6 {
        if step == 2 {
            both(
                &mut world,
                &mut mine,
                json!({"operation":"torus-set","edit":{"carrier":0.4}}),
            );
        }
        if step == 3 {
            both(
                &mut world,
                &mut mine,
                json!({"operation":"torus-play","keys":[{"row":2,"column":6}],"amplitude":[0.3,0.0]}),
            );
        }
        let (a, b) = block(&mut world, &mut mine);
        differed |= a != b;
    }
    assert!(differed, "the personal context is heard");

    // The saved sound is the world's: the checkpoint and the session carry nothing of the listener, and a take renders as the world
    // hears it.
    let (kept_w, kept_m) = (
        world.send(json!({"operation":"torus-checkpoint"})),
        mine.send(json!({"operation":"torus-checkpoint"})),
    );
    assert_eq!(kept_m["status"], "ok", "{kept_m}");
    assert_eq!(kept_w["torus_checkpoint"], kept_m["torus_checkpoint"]);
    assert_eq!(kept_w["torus"], kept_m["torus"]);
    let (state_w, state_m) = (
        world.send(json!({"operation":"torus-state"})),
        mine.send(json!({"operation":"torus-state"})),
    );
    assert_eq!(
        state_w["torus"], state_m["torus"],
        "the session is the world's"
    );
    for text in [kept_m.to_string(), state_m.to_string()] {
        assert!(
            !text.contains("torus_receiver") && !text.contains("at_metres"),
            "{text}"
        );
    }
    let render = |d: &mut Driver, checkpoint: &Value| {
        let r = d.send(
            json!({"operation":"torus-render","checkpoint":checkpoint,"frames":2048,"acts":[]}),
        );
        assert_eq!(r["status"], "ok", "{r}");
        audio(&r["torus_render"])
    };
    let checkpoint = kept_w["torus_checkpoint"].clone();
    assert_eq!(
        render(&mut world, &checkpoint),
        render(&mut mine, &checkpoint)
    );

    // Back in the world's context: one block reaches the declared gains, and from then on the sound is the world's, sample for sample.
    let off = mine.send(json!({"operation":"torus-receiver","receiver":null}));
    assert_eq!(off["torus_receiver"]["context"], "world");
    let (a, b) = block(&mut world, &mut mine);
    assert_ne!(a, b, "the block that reaches the declared gains");
    for _ in 0..4 {
        let (a, b) = block(&mut world, &mut mine);
        assert_eq!(a, b, "the world's sound again");
    }

    // Refused in words, changing nothing: a place past the owner's bounds.
    let standing_body = body(&mine.current);
    let refused =
        mine.send(json!({"operation":"torus-receiver","receiver":{"at_metres":[2e6,0.0,0.0]}}));
    assert_eq!(refused["status"], "refused", "{refused}");
    assert!(
        refused["error"].as_str().unwrap().contains("place"),
        "{refused}"
    );
    assert_eq!(body(&refused), standing_body);
    let (a, b) = block(&mut world, &mut mine);
    assert_eq!(a, b, "still the world's");
}
