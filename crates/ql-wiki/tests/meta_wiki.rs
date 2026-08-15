use ql_wiki::{
    META_KNOWLEDGE_GRAPH_NAME, MappingOrigin, MetaBinding, MetaKnowledgeProjection,
    OKF_WIKI_PROFILE, QL_MEF_WIKI_PROFILE, RESERVED_BIMBA_GRAPH_NAME, WikiError, parse_okf_wiki,
};

const SPACE: &str = include_str!("../../../fixtures/qw1/meta-wiki/ql-structural-space.md");
const FAMILY_A: &str = include_str!("../../../fixtures/qw1/meta-wiki/relation-family-a.md");
const L1: &str = include_str!("../../../fixtures/qw1/meta-wiki/mef-l1.md");
const BINDING: &str = include_str!("../../../fixtures/qw1/meta-binding.json");

fn documents() -> Vec<ql_wiki::OkfWikiDocument> {
    [SPACE, FAMILY_A, L1]
        .into_iter()
        .map(|source| parse_okf_wiki(source).expect("valid OKF/Wiki fixture"))
        .collect()
}

#[test]
fn ql_mef_meta_wiki_uses_open_okf_wiki_profile_and_preserves_extensions() {
    let document = parse_okf_wiki(FAMILY_A).unwrap();
    assert_eq!(document.wiki.profile, OKF_WIKI_PROFILE);
    assert_eq!(document.ql_mef_profile(), Some(QL_MEF_WIKI_PROFILE));
    assert_eq!(
        document.wiki.canonical_ref,
        "ql-mef:wiki:node:relation-family-a"
    );
    assert_eq!(document.wiki.revision, 2);
    assert_eq!(document.wiki.object_kind, "node");
    assert_eq!(document.wiki.title(), Some("Relation Family A"));
    assert_eq!(
        document.wiki.raw["ql_mef"]["producer_specific_note"],
        "preserved"
    );
    assert!(document.body.contains("canonical natural-dyad"));
}

#[test]
fn unknown_okf_extensions_round_trip_as_parsed_values() {
    let document = parse_okf_wiki(SPACE).unwrap();
    assert_eq!(document.okf["producer_extension"]["preserve"], true);
    assert_eq!(document.wiki.raw["ql_mef"]["ontology"], "structural-canon");
}

#[test]
fn canonical_ref_is_independent_of_file_location_or_projection_binding() {
    let first = parse_okf_wiki(FAMILY_A).unwrap();
    // Parsing from another path would receive the same content. The parser has
    // no path input and therefore cannot derive identity from filename/location.
    let moved = parse_okf_wiki(FAMILY_A).unwrap();
    assert_eq!(first.wiki.canonical_ref, moved.wiki.canonical_ref);
    assert_eq!(first.wiki.revision, moved.wiki.revision);

    let projection = MetaKnowledgeProjection::rebuild(&documents(), &[], 1).unwrap();
    let projected = projection
        .objects
        .iter()
        .find(|object| object.canonical_ref == first.wiki.canonical_ref)
        .unwrap();
    assert_ne!(projected.projection_id.to_string(), projected.canonical_ref);
}

#[test]
fn projection_rebuild_retains_canonical_mapping_while_local_ids_are_implementation_detail() {
    let docs = documents();
    let first = MetaKnowledgeProjection::rebuild(&docs, &[], 1).unwrap();
    let mut reversed = docs.clone();
    reversed.reverse();
    let second = MetaKnowledgeProjection::rebuild(&reversed, &[], 2).unwrap();
    assert_eq!(first.canonical_refs(), second.canonical_refs());
    assert_eq!(first.canonical_refs().len(), 3);
    assert_eq!(first.projection_version, 1);
    assert_eq!(second.projection_version, 2);
    assert!(first.objects.iter().all(|object| object.projection_id > 0));
}

#[test]
fn external_meta_binding_maps_foreign_frame_without_taking_ownership() {
    let binding: MetaBinding = serde_json::from_str(BINDING).unwrap();
    assert_eq!(binding.origin, MappingOrigin::Recognised);
    binding.validate().unwrap();
    let projection =
        MetaKnowledgeProjection::rebuild(&documents(), std::slice::from_ref(&binding), 4).unwrap();

    assert_eq!(
        projection.mapped_targets("ql-mef:wiki:node:mef-l1"),
        vec![&binding]
    );
    assert_eq!(
        projection.mappings_for_target("example:wiki:frame:decision-17"),
        vec![&binding]
    );
    assert!(!projection.contains_foreign_object("example:wiki:frame:decision-17"));
    assert!(!projection.contains_foreign_object("example:wiki:project-alpha"));
    let relation = projection
        .relations
        .iter()
        .find(|relation| relation.origin_ref.as_deref() == Some(binding.binding_ref.as_str()))
        .unwrap();
    assert_eq!(relation.from_ref, "ql-mef:wiki:node:mef-l1");
    assert_eq!(relation.to_ref, "example:wiki:frame:decision-17");
    assert_eq!(relation.relation, "refracts");
    assert_eq!(relation.origin, "recognised");
}

#[test]
fn unavailable_foreign_provider_does_not_destroy_mapping_identity() {
    let binding: MetaBinding = serde_json::from_str(BINDING).unwrap();
    assert_eq!(binding.extensions["foreign_provider_state"], "unavailable");
    let projection =
        MetaKnowledgeProjection::rebuild(&documents(), std::slice::from_ref(&binding), 5).unwrap();
    let observed_bindings = projection.mapped_targets("ql-mef:wiki:node:mef-l1");
    let [observed] = observed_bindings.as_slice() else {
        panic!("expected one mapping")
    };
    assert_eq!(observed.binding_ref, binding.binding_ref);
    assert_eq!(observed.target_wiki_ref, binding.target_wiki_ref);
    assert_eq!(observed.target_frame_ref, binding.target_frame_ref);
}

#[test]
fn mapping_origins_remain_distinct_in_projection() {
    let base: MetaBinding = serde_json::from_str(BINDING).unwrap();
    let bindings = [
        MetaBinding {
            binding_ref: "binding:authored".into(),
            origin: MappingOrigin::Authored,
            ..base.clone()
        },
        MetaBinding {
            binding_ref: "binding:derived".into(),
            origin: MappingOrigin::Derived,
            ..base.clone()
        },
        MetaBinding {
            binding_ref: "binding:proposed".into(),
            origin: MappingOrigin::Proposed,
            ..base
        },
    ];
    let projection = MetaKnowledgeProjection::rebuild(&documents(), &bindings, 6).unwrap();
    let origins = projection
        .relations
        .iter()
        .filter(|relation| relation.relation == "refracts")
        .map(|relation| relation.origin.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        origins,
        std::collections::BTreeSet::from(["authored", "derived", "proposed"])
    );
}

#[test]
fn provider_or_database_id_is_rejected_as_canonical_wiki_identity() {
    let bad = r#"---
type: QL Definition
wiki_profile: okf-wiki/v1
wiki:
  profile: okf-wiki/v1
  object: node
  ref: ql-mef:wiki:node:bad
  revision: 1
  provenance: []
  type: QL Definition
  space_refs: []
  source_refs: []
  provider_id: neo4j-row-17
---
# Bad
"#;
    assert!(matches!(
        parse_okf_wiki(bad),
        Err(WikiError::ProviderIdentityLeak(field)) if field == "provider_id"
    ));
}

#[test]
fn meta_knowledge_projection_and_bimba_graph_names_are_explicitly_not_identified() {
    assert_eq!(
        META_KNOWLEDGE_GRAPH_NAME,
        "QL-MEF Meta-Knowledge Graph Projection"
    );
    assert_eq!(RESERVED_BIMBA_GRAPH_NAME, "Epi-Logos Bimba Graph");
    assert_ne!(META_KNOWLEDGE_GRAPH_NAME, RESERVED_BIMBA_GRAPH_NAME);
    let source = format!("{SPACE}\n{FAMILY_A}\n{L1}\n{BINDING}");
    assert!(!source.contains("Bimba Graph"));
}
