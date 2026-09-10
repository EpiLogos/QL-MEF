use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use ql_mef::MFace;
use ql_mef::m_tree::{MRegistry, MTreeId, NATIVE_M_MANIFEST, native_m_registry};
use serde_json::{Value, json};

#[test]
fn actual_c_descriptors_equal_every_rust_manifest_record() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/m-tree");
    std::fs::create_dir_all(&out).unwrap();
    let executable = out.join("rust-native-probe");
    let built = Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
        .current_dir(&root)
        .args([
            "-std=c11",
            "-O1",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-Ic/include",
            "migration/epi-kernel/k2-m-tree-probe.c",
            "c/src/m_tree.c",
            "-o",
        ])
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let output = Command::new(&executable).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::write(out.join("c-rust-parity.jsonl"), &output.stdout).unwrap();
    let registry = native_m_registry();
    let m = registry.manifest();
    let full = serde_json::to_value(m).unwrap();
    let plurals = BTreeMap::from([
        ("node", "nodes"),
        ("file", "files"),
        ("record", "records"),
        ("relation", "relations"),
        ("binding", "bindings"),
    ]);
    let mut offsets: BTreeMap<String, usize> = BTreeMap::new();
    let mut metadata = 0;
    for line in String::from_utf8(output.stdout).unwrap().lines() {
        let mut row: Value = serde_json::from_str(line).unwrap();
        let kind = row
            .as_object_mut()
            .unwrap()
            .remove("kind")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();
        if kind == "registry" {
            assert_eq!(
                row,
                json!({"schema": m.schema, "registry_revision": m.registry_revision,
                "source_repository": m.source_repository, "source_revision": m.source_revision,
                "nodes": m.nodes.len(), "files": m.files.len(), "records": m.records.len(),
                "relations": m.relations.len(), "bindings": m.bindings.len()})
            );
            metadata += 1;
        } else {
            let index = offsets.entry(kind.clone()).or_default();
            let expected = &full[plurals[kind.as_str()]][*index];
            assert_eq!(&row, expected, "{kind}[{index}]");
            if kind == "node" {
                let node = registry
                    .resolve(row["source_ref"].as_str().unwrap())
                    .unwrap();
                assert_eq!(
                    serde_json::to_value(registry.node(node.id).unwrap()).unwrap(),
                    row
                );
                assert_eq!(
                    registry.children(node.id).map(|n| n.id).collect::<Vec<_>>(),
                    node.children
                );
                assert_eq!(registry.parent(node.id).map(|n| n.id), node.parent_id);
            } else if kind == "relation" {
                let id = MTreeId::parse(row["id"].as_str().unwrap()).unwrap();
                assert_eq!(
                    serde_json::to_value(registry.relation(id).unwrap()).unwrap(),
                    row
                );
            }
            *index += 1;
        }
    }
    assert_eq!(metadata, 1);
    for (singular, plural) in plurals {
        assert_eq!(offsets[singular], full[plural].as_array().unwrap().len());
    }
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn actual_source_parentage_asymmetry_and_alternate_spellings_survive() {
    let r = native_m_registry();
    assert_eq!(r.manifest().nodes.len(), 1876);
    assert_eq!(r.manifest().relations.len(), 21083);
    assert_eq!(r.master().children.len(), 6);
    for (i, expected) in [108, 43, 597, 996, 100, 31].into_iter().enumerate() {
        let root = r.root(i).unwrap();
        assert_eq!(root.subtree_count, expected);
        assert_eq!(r.resolve(&format!("M{i}")).unwrap().id, root.id);
    }
    let compound = r.resolve("#0-4.0/1/2").unwrap();
    assert_eq!(r.parent(compound.id).unwrap().source_ref, "#0-4");
    assert_eq!(compound.local_segment, "0/1/2");
    assert_eq!(compound.separator, ".");
    assert_ne!(compound.id, r.resolve("#0-4.0/1-2").unwrap().id);
    assert_ne!(
        r.resolve("#4.5-0").unwrap().id,
        r.resolve("#4.5.0").unwrap().id
    );
    assert_eq!(r.manifest().alternate_notation_groups.len(), 8);
    for unknown in [
        "#", "#-0", "#0-4.0", "#3-5-5", "#6", "M6", "#0-", "", "#0-01",
    ] {
        assert!(r.resolve(unknown).is_none(), "{unknown}");
    }
    assert!(r.root(usize::MAX).is_none());
    let backward = r.resolve("#2-4").unwrap();
    assert_eq!(backward.source_parent_refs, ["#2", "#2-4.5"]);
    assert_eq!(r.parent(backward.id).unwrap().source_ref, "#2");
    assert_eq!(r.manifest().parent_discrepancies.len(), 1);
    assert_eq!(
        r.manifest().relations.iter().filter(|r| r.cross_m).count(),
        2676
    );
    assert_eq!(
        r.manifest()
            .relations
            .iter()
            .filter(|r| r.from_id.is_none() || r.to_id.is_none())
            .count(),
        450
    );
}

#[test]
fn compatibility_projection_preserves_records_relations_and_reflection() {
    let r = native_m_registry();
    let index = r.to_m_map_index().unwrap();
    assert_eq!(index.source_coordinate_count(), 1875);
    assert_eq!(index.relation_count(), 21083);
    assert_eq!(index.roots(), BTreeSet::from([0, 1, 2, 3, 4, 5]));
    for n in r.manifest().nodes.iter().filter(|n| n.source_ref != "M") {
        let bimba = index.resolve(&n.source_ref, MFace::Bimba).unwrap();
        let prime = index.resolve(&n.source_ref, MFace::Pratibimba).unwrap();
        assert!(bimba.same_structural_path(&prime));
        assert_eq!(bimba.provenance.len(), n.records.len());
        assert_eq!(bimba.payloads.len(), n.records.len());
        assert_eq!(bimba.provenance[0].revision, r.manifest().source_revision);
        assert_eq!(bimba.source_ref, n.source_ref);
        assert_eq!(
            r.resolve(&n.source_ref.replacen('#', "M", 1)).unwrap().id,
            n.id
        );
        index.prove_exact_reflection(&n.source_ref).unwrap();
    }
    let compound = index.resolve("#0-4.0/1/2", MFace::Bimba).unwrap();
    assert_eq!(compound.parent_source_ref.as_deref(), Some("#0-4"));
    assert_eq!(
        index
            .resolve("#0", MFace::Bimba)
            .unwrap()
            .parent_source_ref
            .as_deref(),
        Some("#")
    );
    let leading_zero = ql_mef::MCoordinate::parse_source("#0-01-002", MFace::Bimba).unwrap();
    assert_eq!(leading_zero.parent_source_ref.as_deref(), Some("#0-01"));
}

#[test]
fn missing_implementation_cannot_erase_or_invent_source_existence() {
    let r = native_m_registry();
    let node = r.resolve("#3-2-1").unwrap();
    assert!(
        !r.manifest()
            .bindings
            .iter()
            .any(|b| b.coordinate_id == Some(node.id))
    );
    let mut body: Value = serde_json::from_str(NATIVE_M_MANIFEST).unwrap();
    body["bindings"] = json!([]);
    let unbound = MRegistry::from_json(&body.to_string()).unwrap();
    assert_eq!(unbound.resolve(&node.source_ref).unwrap().id, node.id);
    let mut binding = r.manifest().bindings[0].clone();
    binding.coordinate_id = Some(MTreeId::parse("ffffffffffffffff").unwrap());
    assert!(r.validate_binding(&binding).is_err());
    binding.coordinate_id = Some(node.id);
    binding.readiness = "full-M-computation".into();
    assert!(r.validate_binding(&binding).is_err());
    binding.readiness = "unbound".into();
    assert!(r.validate_binding(&binding).is_ok());
    assert_eq!(r.resolve(&node.source_ref).unwrap().id, node.id);
}

#[test]
fn malformed_registry_and_id_boundaries_fail_closed() {
    for bad in [
        "0",
        "0000000000000000",
        "FFFFFFFFFFFFFFFF",
        "000000000000000g",
        "00000000000000000",
    ] {
        assert!(MTreeId::parse(bad).is_err());
    }
    let pristine: Value = serde_json::from_str(NATIVE_M_MANIFEST).unwrap();
    let mut duplicate = pristine.clone();
    duplicate["nodes"]
        .as_array_mut()
        .unwrap()
        .push(pristine["nodes"][1].clone());
    assert!(MRegistry::from_json(&duplicate.to_string()).is_err());
    let mut orphan = pristine.clone();
    orphan["nodes"][1]["parent_id"] = Value::Null;
    assert!(MRegistry::from_json(&orphan.to_string()).is_err());
    let mut unknown_endpoint = pristine.clone();
    unknown_endpoint["relations"][0]["from_id"] = json!("ffffffffffffffff");
    assert!(MRegistry::from_json(&unknown_endpoint.to_string()).is_err());
    let mut changed_segment = pristine.clone();
    changed_segment["nodes"][3]["local_segment"] = json!("999");
    assert!(MRegistry::from_json(&changed_segment.to_string()).is_err());
    let mut broken_record = pristine.clone();
    broken_record["records"][0]["file"] = json!(usize::MAX);
    assert!(MRegistry::from_json(&broken_record.to_string()).is_err());
}
