//! Parent-join proof for the K/AW expression-production session (2026-09-17).
//!
//! One real Bimba coordinate — `#1-2` (Ananda) from
//! `fixtures/kernel/m-tree-v1.json` at the locked C-Experiments revision —
//! carries unchanged identity from the source registry through the production
//! M′ Technē source (`WikiTechneAdapter`) and through the Ta-Onta C′/Vāk
//! composition to its operative Return. The coordinate's six real children
//! (Matrix 0–5) are the whole's six positions; the binding is test-scoped
//! (PROPOSED), not canon.

use ql_adapters::techne_wiki::{WikiSubjectSource, WikiTechneAdapter};
use ql_core::{ConstellationGrain, QlShape};
use ql_wiki::{
    WikiProvenanceRef, WikiRefractionTarget, WikiShapeBindingProvenance, WikiShapeBindingView,
    WikiShapeMember, WikiTargetKind, attach_shape_binding,
};
use serde_json::{Value, json};

const SUBJECT: &str = "#1-2";
const DATASET_SOURCE: &str = "Idea/Bimba/Map/datasets/low-detail/nodes_hash.json";
const SOURCE_REVISION: &str = "daa660cbc1b8c5da83828698665a753852cb0287";
const MATRIX_MEMBERS: [(&str, u8); 6] = [
    ("#1-2-0", 0),
    ("#1-2-1", 1),
    ("#1-2-2", 2),
    ("#1-2-3", 3),
    ("#1-2-4", 4),
    ("#1-2-5", 5),
];

fn ananda_target() -> WikiRefractionTarget {
    let mut target = WikiRefractionTarget {
        kind: WikiTargetKind::NodeLocal,
        target_ref: SUBJECT.into(),
        target_frame_ref: None,
        target_revision: None,
        target_snapshot_hash: "sha256:m-tree-v1:#1-2".into(),
        provenance: vec![WikiProvenanceRef {
            source_ref: DATASET_SOURCE.into(),
            source_revision: None,
            extensions: std::collections::BTreeMap::new(),
        }],
        subjects: vec![],
        relations: vec![],
        structural_field: None,
        material: serde_json::Map::new(),
        extensions: serde_json::Map::new(),
    };
    // The sixfold presents directly: no compression claim is made over the
    // six real Matrix children.
    let presented = QlShape::Constellation(ConstellationGrain::SixFold);
    let binding = WikiShapeBindingView {
        subject_ref: SUBJECT.into(),
        shape_ref: presented.shape_ref(),
        whole_ref: format!("anchor:{SUBJECT}"),
        basis_refs: MATRIX_MEMBERS.iter().map(|(r, _)| (*r).into()).collect(),
        members: MATRIX_MEMBERS
            .iter()
            .map(|(reference, position)| WikiShapeMember {
                subject_ref: (*reference).into(),
                position: *position,
                face: "direct".into(),
            })
            .collect(),
        derivation_ref: None,
        operator_ref: None,
        return_refs: vec![format!("return:{SUBJECT}")],
        provenance: WikiShapeBindingProvenance {
            caller_ref: "ql-cli:production-join".into(),
            source_ref: DATASET_SOURCE.into(),
            standing_ref: "PROPOSED".into(),
        },
    };
    attach_shape_binding(&mut target, &binding).unwrap();
    target
}

fn ananda_subjects() -> impl WikiSubjectSource {
    move |subject_ref: &str| (subject_ref == SUBJECT).then(ananda_target)
}

fn basis() -> Value {
    json!({
        "caller": "ql-cli:production-join",
        "source": DATASET_SOURCE,
        "revision": "join-v1",
        "standing": "PROPOSED",
        "evidence": ["fixtures/kernel/m-tree-v1.json"]
    })
}

fn composition_request() -> Value {
    let mut members = Vec::new();
    for (reference, position) in MATRIX_MEMBERS {
        members.push(json!({"subjectRef": reference, "position": position, "face": "direct"}));
        members.push(json!({"subjectRef": format!("{reference}:conjugate"), "position": position, "face": "conjugate"}));
    }
    let b = basis();
    let hops = [(0usize, 5u8), (5, 0)];
    let mut steps = vec![
        json!({
            "op": "whole", "useRef": "ananda", "subjectRef": SUBJECT, "wholeRef": format!("anchor:{SUBJECT}"),
            "category": "M", "groundRef": format!("ground:{SUBJECT}"), "groundFace": "direct",
            "frame": {"id": "CF2", "lens": "L0", "basis": "chromatic", "face": "direct", "positions": "local"},
            "basis": b, "members": members,
            "sourceReturns": [{"fromRef": format!("source-result:{SUBJECT}"), "anchorRef": format!("anchor:{SUBJECT}"),
                                "groundRef": format!("ground:{SUBJECT}"), "face": "direct", "kind": "own"}]
        }),
        json!({"op": "enter", "id": "ctx", "useRef": "ananda",
               "categoryGround": {"position": 4, "face": "direct"}}),
        json!({"op": "cfp-form", "oikonomia": "oik", "context": "ctx", "form": "fusion", "basis": b}),
        json!({"op": "cs-select", "oikonomia": "oik", "context": "ctx",
               "profile": "direct-synthesis", "direction": "forward-synthesis", "basis": b}),
    ];
    for (k, (from, to)) in hops.iter().enumerate() {
        steps.push(json!({"op": "cp", "context": "ctx", "into": format!("tgt{k}"),
                          "coordinate": {"position": to, "face": "direct"}, "positions": "local", "basis": b}));
        steps.push(json!({"op": "cp", "context": "ctx", "into": format!("src{k}"),
                          "coordinate": {"position": from, "face": "direct"}, "positions": "local", "basis": b}));
        steps.push(
            json!({"op": "determine", "id": format!("d{k}"), "useRef": format!("src{k}"),
                          "lens": "L0", "context": "ctx", "basis": b}),
        );
        steps.push(json!({"op": "cs-hop", "oikonomia": "oik", "context": "ctx", "id": format!("r{k}"),
                          "determination": format!("d{k}"), "target": format!("tgt{k}"), "kind": "other",
                          "relationEvidence": format!("{DATASET_SOURCE}#cs-direct-synthesis-{from}-{to}"),
                          "basis": b}));
    }
    steps.push(
        json!({"op": "z-begin", "oikonomia": "oik", "context": "ctx",
                      "undertakingRef": format!("undertaking:{SUBJECT}-return"),
                      "goalEvidence": [DATASET_SOURCE],
                      "authorisationRef": "authorisation:production-join", "basis": b}),
    );
    for stage in ["compose", "perform", "record", "rehear", "recompose"] {
        steps.push(
            json!({"op": "z-advance", "oikonomia": "oik", "context": "ctx", "stage": stage,
                          "resultRef": format!("result:z-{stage}"),
                          "evidence": [format!("evidence:z-{stage}")], "basis": b}),
        );
    }
    steps.push(json!({"op": "operative-return", "oikonomia": "oik", "returnRef": "r0"}));
    json!({"contract": "ql.vak-composition/v1", "steps": steps})
}

#[test]
fn bimba_identity_survives_from_registry_through_m_prime_instrument_to_aletheia_return() {
    // The registry owns the coordinate: #1-2 with its six real Matrix children.
    let registry_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/kernel/m-tree-v1.json");
    let registry: Value = serde_json::from_slice(&std::fs::read(registry_path).unwrap()).unwrap();
    let node = registry["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["source_ref"] == SUBJECT)
        .unwrap();
    let child_refs: Vec<&str> = node["children"]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| {
            registry["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|n| n["id"] == id.as_str().unwrap())
                .unwrap()["source_ref"]
                .as_str()
                .unwrap()
        })
        .collect();
    let matrix_refs: Vec<&str> = MATRIX_MEMBERS.iter().map(|(r, _)| *r).collect();
    assert_eq!(
        child_refs, matrix_refs,
        "Matrix 0-5 are the real children in order"
    );
    assert_eq!(registry["source_revision"], SOURCE_REVISION);

    // M′ instrument: the production Technē source discloses the ground and
    // the bounded whole with byte-exact refs.
    let subjects = ananda_subjects();
    let adapter = WikiTechneAdapter::new(&subjects);
    let target = ananda_target();
    let reading = adapter.reading_for_target(&target).unwrap();
    reading.validate().unwrap();
    assert_eq!(reading.subject.subject_ref, SUBJECT);
    assert_eq!(reading.ground_ref(), format!("anchor:{SUBJECT}"));
    let whole = reading.whole.as_ref().unwrap();
    assert_eq!(whole.member_refs, matrix_refs);
    let ql = reading.ql.as_ref().unwrap();
    assert_eq!(
        ql.shape_ref,
        QlShape::Constellation(ConstellationGrain::SixFold)
            .shape_ref()
            .into()
    );
    assert!(ql.warrant.provenance_ref.contains("registry-disclosure"));
    assert_eq!(reading.provenance[0].source_ref, DATASET_SOURCE);
    let selections = adapter.selections(&target, &reading).unwrap();
    assert!(selections[0].co_referenced(&selections[1]));

    // Ta-Onta: Anima composes the same coordinate through C′; Aletheia
    // returns through the CS walk with form/passage/Z surviving.
    let result = ql_cli::vak_composition::execute_request(&composition_request()).unwrap();
    assert_eq!(result["contract"], "ql.vak-composition/v1");
    assert_eq!(result["sourceRevision"], SOURCE_REVISION);
    let results = result["results"].as_array().unwrap();
    let whole_view = &results[0]["result"];
    assert_eq!(whole_view["binding"]["subjectRef"], SUBJECT);
    let operative = &results[results.len() - 1]["result"];
    assert_eq!(
        operative["cfpForm"],
        "ql.cprime-oikonomia/v1:cfp3:fusion-f-thread"
    );
    assert_eq!(
        operative["csWalk"]["marker"],
        "ql.cprime-oikonomia/v1:cs5:forward-synthesis:extent:2"
    );
    assert_eq!(operative["return"]["sourceBinding"]["subjectRef"], SUBJECT);
    assert_eq!(operative["return"]["basis"]["source"], DATASET_SOURCE);
    assert_eq!(operative["return"]["producing"]["standing"], "DERIVED");
    let z = operative["zCycle"].as_object().unwrap();
    assert_eq!(z["complete"], json!(true));

    // The join: both instruments hold the same subject on the same source.
    assert_eq!(
        reading.subject.subject_ref,
        whole_view["binding"]["subjectRef"]
    );
    assert_eq!(
        reading.provenance[0].source_ref,
        operative["return"]["basis"]["source"]
    );
    assert_eq!(
        reading.snapshot.as_ref().unwrap().basis_ref,
        Some(format!("sha256:m-tree-v1:{SUBJECT}"))
    );
}
