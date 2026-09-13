use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use ql_mef::m_tree::{
    CURRENT_M_MANIFEST, MRegistry, MTreeId, native_current_m_registry, native_m_registry,
};
use serde_json::{Value, json};

#[test]
fn actual_current_c_descriptors_equal_every_rust_manifest_record() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/k8-registry");
    std::fs::create_dir_all(&out).unwrap();
    let executable = out.join("rust-native-probe");
    assert!(
        Command::new("make")
            .current_dir(&root)
            .args(["-s", "-C", "c", "all"])
            .status()
            .unwrap()
            .success()
    );
    let source = include_str!("../../../migration/epi-kernel/k2-m-tree-probe.c")
        .replace("ql_m_", "ql_m_live_")
        .replace("ql/m_tree.h", "ql/m_tree_live.h")
        .replace("QL_M_TREE_SCHEMA", "QL_M_LIVE_TREE_SCHEMA");
    let source = source.replace(
        r#"printf(",\"bytes\":%" PRIu64 "}\n", f->bytes);"#,
        r#"printf(",\"bytes\":%" PRIu64, f->bytes);
            const QL_M_SourceOrigin *origin = ql_m_live_source_origin_at(i);
            if (strcmp(origin->repository, ql_m_live_source_repository()) || strcmp(origin->revision, ql_m_live_source_revision())) {
                key("repository", origin->repository); key("revision", origin->revision);
            }
            puts("}");"#);
    std::fs::write(out.join("probe.c"), source).unwrap();
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
            "target/k8-registry/probe.c",
            "c/build/libql-mef-c.a",
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
    let registry = native_current_m_registry();
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
fn current_lineage_and_all_historical_identities_are_preserved() {
    let old = native_m_registry();
    let current = native_current_m_registry();
    assert_eq!(
        current.manifest().nodes.len(),
        old.manifest().nodes.len() + 27
    );
    assert_eq!(
        current.manifest().registry_lineage[0].registry_revision,
        old.manifest().registry_revision
    );
    for original in &old.manifest().nodes {
        let new = current.node(original.id).unwrap();
        let mut a = serde_json::to_value(original).unwrap();
        let mut b = serde_json::to_value(new).unwrap();
        for key in ["children", "subtree_count"] {
            a.as_object_mut().unwrap().remove(key);
            b.as_object_mut().unwrap().remove(key);
        }
        assert_eq!(a, b);
        assert!(original.children.iter().all(|id| new.children.contains(id)));
    }
    for original in &old.manifest().relations {
        assert_eq!(
            serde_json::to_value(original).unwrap(),
            serde_json::to_value(current.relation(original.id).unwrap()).unwrap()
        );
    }
    let origin = current.manifest().files.last().unwrap();
    assert_eq!(origin.repository.as_deref(), Some("EpiLogos/QL-MEF"));
    assert_eq!(
        origin.revision.as_deref(),
        Some("efa599661aab171033769d33a3c2b6233c86eca5")
    );
    assert_eq!(current.resolve("#3-5-5/0").unwrap().children.len(), 360);
    assert_eq!(current.resolve("#2-0-2").unwrap().children.len(), 8);
}

#[test]
fn invalid_lineage_and_partial_origin_do_not_enter_current_registry() {
    let mut value: Value = serde_json::from_str(CURRENT_M_MANIFEST).unwrap();
    value["registry_lineage"] = json!([]);
    assert!(MRegistry::from_json(&value.to_string()).is_err());
    let mut value: Value = serde_json::from_str(CURRENT_M_MANIFEST).unwrap();
    value["registry_lineage"][0]["registry_revision"] = json!("main");
    assert!(MRegistry::from_json(&value.to_string()).is_err());
    let mut value: Value = serde_json::from_str(CURRENT_M_MANIFEST).unwrap();
    value["files"]
        .as_array_mut()
        .unwrap()
        .last_mut()
        .unwrap()
        .as_object_mut()
        .unwrap()
        .remove("revision");
    assert!(MRegistry::from_json(&value.to_string()).is_err());
}
