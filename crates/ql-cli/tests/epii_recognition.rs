//! Recognition as QL's compression, through the composition envelope: a
//! disclosed whole is presented as its `0/1` under the kernel's law, reopens
//! to the exact binding it came from, and every declared relation between
//! participations carries the kernel's own reading of the pair.
use ql_cli::vak_composition::execute_request;
use serde_json::{Value, json};

fn basis(source: &str) -> Value {
    json!({"caller":"epii:test","source":source,"revision":"test-v1","standing":"PROPOSED","evidence":["test:declared"]})
}

fn frame() -> Value {
    json!({"id":"CF2","lens":"L0","basis":"chromatic","face":"direct","positions":"local"})
}

/// One turn as a whole: each member is a thought at its T coordinate.
fn whole(use_ref: &str, members: &[(u8, &str)], relations: Value) -> Value {
    json!({"op":"whole","useRef":use_ref,"subjectRef":format!("turn:{use_ref}"),"wholeRef":format!("anchor:{use_ref}"),
        "category":"T","groundRef":"ground:now","groundFace":"direct","frame":frame(),"basis":basis("now:test"),
        "members":members.iter().map(|(p,f)|json!({"subjectRef":format!("thought:{use_ref}:{p}{f}"),"position":p,"face":if *f=="'" {"conjugate"} else {"direct"}})).collect::<Vec<_>>(),
        "relations":relations})
}

fn run(steps: Vec<Value>) -> Vec<Value> {
    let out = execute_request(&json!({"contract":"ql.vak-composition/v1","steps":steps})).unwrap();
    out["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["result"].clone())
        .collect()
}

#[test]
fn a_threefold_turn_is_recognised_as_its_0_1_and_reopens_to_the_same_binding() {
    let results = run(vec![
        whole("t", &[(1, ""), (2, ""), (3, "")], json!([])),
        json!({"op":"recognise","from":"t","into":"t.1","basis":basis("epii:recognition")}),
        json!({"op":"read","useRef":"t.1","lens":"L0"}),
        json!({"op":"reopen","from":"t.1","into":"t.2","basis":basis("epii:reopen")}),
        json!({"op":"read","useRef":"t.2","lens":"L0"}),
    ]);
    let (bound, recognised, presented, reopened, disclosed) = (
        &results[0],
        &results[1],
        &results[2],
        &results[3],
        &results[4],
    );
    assert_eq!(
        recognised["binding"]["shapeRef"],
        "ql:shape:1.0.0:constellation:anchor-only"
    );
    assert_eq!(
        recognised["binding"]["operatorRef"],
        "ql:shape:1.1.0:compression:3-to-1-recognition"
    );
    assert_eq!(
        recognised["binding"]["basisRefs"],
        bound["binding"]["basisRefs"]
    );
    assert_eq!(recognised["body"]["members"], bound["body"]["members"]);
    assert_eq!(presented["address"]["kind"], "anchor");
    assert_eq!(reopened["binding"], bound["binding"]);
    assert_eq!(disclosed["address"]["kind"], "member");
    let operators: Vec<&str> = reopened["transitions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["operator"].as_str().unwrap())
        .collect();
    assert_eq!(
        operators,
        [
            "ql:shape:1.1.0:compression:3-to-1-recognition",
            "ql.vak-composition/v1:reopen-recognised-whole"
        ]
    );
}

#[test]
fn the_twelvefold_is_recognised_through_its_three_sixfolds_eighteen_three_one() {
    let all: Vec<(u8, &str)> = (0..6).flat_map(|p| [(p, ""), (p, "'")]).collect();
    let results = run(vec![
        whole("now", &all, json!([])),
        json!({"op":"recognise","from":"now","into":"now.1","basis":basis("epii:recognition")}),
    ]);
    let derivation = results[1]["binding"]["derivationRef"].as_str().unwrap();
    assert!(
        derivation.contains("compression:18-by-6-to-threefold"),
        "{derivation}"
    );
    assert!(
        derivation.contains("direct+conjugate+relational"),
        "{derivation}"
    );
    assert!(derivation.ends_with("each:[1,2,3,4]+[5,0]"), "{derivation}");
    assert_eq!(results[1]["body"]["members"].as_array().unwrap().len(), 12);
}

#[test]
fn the_four_the_sixfold_and_its_readings_are_recognised_and_reopen_exactly() {
    let six: Vec<(u8, &str)> = (0..6).map(|p| (p, "")).collect();
    let results = run(vec![
        whole("four", &[(1, ""), (2, ""), (3, ""), (4, "")], json!([])),
        json!({"op":"recognise","from":"four","into":"four.1","basis":basis("epii:recognition")}),
        whole("six", &six, json!([])),
        json!({"op":"recognise","from":"six","into":"six.1","basis":basis("epii:recognition")}),
        json!({"op":"recognise","from":"six","into":"six.pairs","route":"pairs-C","basis":basis("epii:recognition")}),
        json!({"op":"read","useRef":"six.pairs","lens":"L0"}),
        json!({"op":"reopen","from":"six.pairs","into":"six.open","basis":basis("epii:reopen")}),
    ]);
    assert!(
        results[1]["binding"]["operatorRef"]
            .as_str()
            .unwrap()
            .ends_with(":3+1")
    );
    let backbone = results[3]["binding"]["derivationRef"].as_str().unwrap();
    assert!(
        backbone
            .ends_with(":6:4+2:from:ql:shape:1.0.0:constellation:sixfold:parts:[1,2,3,4]+[5,0]"),
        "{backbone}"
    );
    let pairs = results[4]["binding"]["derivationRef"].as_str().unwrap();
    assert!(pairs.ends_with("parts:[0,5]+[1,4]+[2,3]"), "{pairs}");
    assert_ne!(
        results[3]["binding"]["operatorRef"],
        results[4]["binding"]["operatorRef"]
    );
    assert_eq!(results[5]["address"]["kind"], "anchor");
    assert_eq!(results[6]["binding"], results[2]["binding"]);
}

#[test]
fn a_set_that_is_not_a_whole_is_told_to_recognise_its_parts() {
    for (members, route, expected) in [
        (vec![(1, ""), (2, ""), (5, "")], None, "recognise its parts"),
        (
            vec![(1, ""), (2, ""), (3, "")],
            Some("4+2"),
            "reading of six was given",
        ),
        (
            vec![(0, ""), (1, ""), (2, ""), (3, ""), (4, ""), (5, "")],
            Some("2+2+2"),
            "unknown reading of six",
        ),
    ] {
        let mut step =
            json!({"op":"recognise","from":"w","into":"w.1","basis":basis("epii:recognition")});
        if let Some(route) = route {
            step["route"] = json!(route);
        }
        let request = json!({"contract":"ql.vak-composition/v1","steps":[whole("w", &members, json!([])), step]});
        let error = execute_request(&request).unwrap_err().to_string();
        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn a_recognised_whole_is_not_recognised_twice_and_a_disclosed_one_does_not_reopen() {
    let request = json!({"contract":"ql.vak-composition/v1","steps":[
        whole("t", &[(1, ""), (4, "")], json!([])),
        {"op":"recognise","from":"t","into":"t.1","basis":basis("epii:recognition")},
        {"op":"recognise","from":"t.1","into":"t.2","basis":basis("epii:recognition")}]});
    assert!(
        execute_request(&request)
            .unwrap_err()
            .to_string()
            .contains("already presented")
    );
    let request = json!({"contract":"ql.vak-composition/v1","steps":[
        whole("t", &[(1, ""), (4, "")], json!([])),
        {"op":"reopen","from":"t","into":"t.1","basis":basis("epii:reopen")}]});
    assert!(
        execute_request(&request)
            .unwrap_err()
            .to_string()
            .contains("not a recognised 0/1")
    );
}

#[test]
fn a_declared_relation_carries_the_kernel_reading_of_its_pair() {
    let relation = |row: (u8, &str), column: (u8, &str)| {
        json!({"row":{"position":row.0,"face":row.1},"column":{"position":column.0,"face":column.1},
            "relationRef":"epii:set-against","evidence":["test:declared"]})
    };
    let results = run(vec![whole(
        "t",
        &[(2, ""), (3, ""), (3, "'")],
        json!([
            relation((2, "direct"), (3, "direct")),
            relation((3, "direct"), (3, "conjugate")),
            relation((2, "direct"), (3, "conjugate")),
        ]),
    )]);
    let kernel: Vec<&Value> = results[0]["binding"]["relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| &r["kernel"])
        .collect();
    assert_eq!(
        kernel[0]["relations"],
        json!([
            "ql.kernel.pair.A/v1",
            "ql.kernel.pair.C/v1",
            "ql.kernel.mirror.complement/v1",
            "ql.kernel.position.successor/v1"
        ])
    );
    assert_eq!(
        kernel[0]["fields"],
        json!([{"family":"A","pairIndex":1,"reversed":false},{"family":"C","pairIndex":2,"reversed":false}])
    );
    assert_eq!(
        kernel[1]["relations"],
        json!(["ql.kernel.cross.same-position/v1"])
    );
    assert_eq!(
        kernel[2]["relations"],
        json!([
            "ql.kernel.cross.transform/v1",
            "ql.kernel.cross.complete/v1"
        ])
    );
}
