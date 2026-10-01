//! Source content tests above the M-tree's useful keys-and-hashes projection.
use ql_mef::bimba_content::native_bimba_content;
use serde_json::json;

#[test]
fn native_source_preserves_personal_identity_and_directional_qualified_relations() {
    let source = native_bimba_content().unwrap();
    let locus = source.coordinate("ql:m-coordinate:bimba:M4.4.4.4").unwrap();
    assert_eq!(
        locus["identity"]["uuid"],
        "dcb274c1-fbbc-5914-b27d-dea979c78558"
    );
    assert!(locus["identity"]["properties"].as_object().unwrap().len() > 60);
    assert_eq!(locus["identity"]["coordinate"], "M4.4.4.4");
    assert_ne!(
        source.coordinate("M4-4").unwrap()["identity"]["uuid"],
        locus["identity"]["uuid"]
    );
    let relations = locus["relations"].as_array().unwrap();
    assert!(
        relations
            .iter()
            .any(|edge| edge["from_coordinate"] == "M4.4.4.3"
                && edge["kind"] == "FLOWS_TO"
                && edge["to_coordinate"] == "M4.4.4.4")
    );
    assert!(
        relations
            .iter()
            .any(|edge| edge["from_coordinate"] == "M4.4.4.4"
                && edge["kind"] == "REFLECTS_FOUNDATION"
                && edge["to_coordinate"] == "M0")
    );
    assert!(relations.iter().all(|edge| edge["properties"].is_object()));
    let page = source.inventory(0, 256).unwrap();
    assert!(page["total"].as_u64().unwrap() > 1910);
    assert_eq!(page["items"].as_array().unwrap().len(), 256);
    assert!(source.inventory(0, 257).is_err());
    assert!(source.coordinate("M4.4.4.4 unknown").is_err());
    let earth = source.coordinate("#2-5-0/1-0").unwrap();
    assert_eq!(earth["identity"]["coordinate"], "M2-5-(0/1)-0");
    assert_eq!(earth, source.coordinate("M2-5-(0/1)-0").unwrap());
    let meta = source.coordinate("bimba-source:#0").unwrap();
    let root = source.coordinate("#0").unwrap();
    assert_eq!(meta["identity"]["canonical_ref"], "bimba:#0");
    assert!(meta["identity"]["native_coordinate"].is_null());
    assert_eq!(root["identity"]["coordinate"], "M0");
    assert_ne!(meta["identity"]["uuid"], root["identity"]["uuid"]);
    assert_eq!(
        source
            .coordinate("ql:m-coordinate:pratibimba:M4.4.4.4")
            .unwrap(),
        locus
    );
}

#[test]
fn a_changed_payload_cannot_keep_the_original_source_revision() {
    let mut value: serde_json::Value =
        serde_json::from_str(ql_mef::bimba_content::SOURCE_JSON).unwrap();
    value["content"]["nodes"]["M4.4.4.4"]["properties"]["c_2_uuid"] = json!("wrong-locus");
    assert!(
        ql_mef::bimba_content::BimbaContent::from_value(value)
            .unwrap_err()
            .contains("content hash")
    );
}

#[test]
fn every_advertised_inventory_entrance_opens_its_exact_full_source() {
    let source = native_bimba_content().unwrap();
    let mut offset = 0;
    let mut opened = 0;
    loop {
        let page = source.inventory(offset, 256).unwrap();
        for row in page["items"].as_array().unwrap() {
            let content = source
                .coordinate(row["full_source_ref"].as_str().unwrap())
                .unwrap();
            assert_eq!(content["identity"]["coordinate"], row["coordinate"]);
            assert_eq!(content["identity"]["uuid"], row["uuid"]);
            assert_eq!(
                content["identity"]["properties_sha256"],
                row["properties_sha256"]
            );
            assert_eq!(content["source_revision"], page["source_revision"]);
            assert!(content["identity"]["properties"].is_object());
            opened += 1;
        }
        match page["next_offset"].as_u64() {
            Some(next) => offset = next as usize,
            None => {
                assert_eq!(opened, page["total"].as_u64().unwrap());
                break;
            }
        }
    }
    assert!(source.coordinate("#").is_err());
    let meta = source.coordinate("bimba-source:#").unwrap();
    assert_eq!(meta["identity"]["coordinate"], "#");
    assert_eq!(meta, source.coordinate("bimba:#").unwrap());
}
