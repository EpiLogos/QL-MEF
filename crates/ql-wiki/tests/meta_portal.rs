use std::collections::BTreeSet;

use ql_wiki::{
    CrossWikiTraversalRequest, ForeignTargetResolution, MappingOrigin, MetaBinding,
    MetaKnowledgeProjection, MetaPortal, MetaRouteSurface, PortalScope, StaticForeignResolver,
    TargetAvailability, parse_okf_wiki,
};
use serde_json::{Value, json};

const SPACE: &str = include_str!("../../../fixtures/qw1/meta-wiki/ql-structural-space.md");
const FAMILY_A: &str = include_str!("../../../fixtures/qw1/meta-wiki/relation-family-a.md");
const L1: &str = include_str!("../../../fixtures/qw1/meta-wiki/mef-l1.md");
const BINDINGS: &str = include_str!("../../../fixtures/qw3/portal-bindings.json");
const CONFORMANCE: &str = include_str!("../../../fixtures/qw3/portal-conformance.json");

fn documents() -> Vec<ql_wiki::OkfWikiDocument> {
    [SPACE, FAMILY_A, L1]
        .into_iter()
        .map(|source| parse_okf_wiki(source).unwrap())
        .collect()
}

fn bindings() -> Vec<MetaBinding> {
    serde_json::from_str(BINDINGS).unwrap()
}

fn resolver() -> StaticForeignResolver {
    let mut resolver = StaticForeignResolver::default();
    for (provider, target, revision, title) in [
        (
            "knowledge:glade",
            "glade:wiki:frame:causal-review",
            "7",
            "Glade causal review",
        ),
        (
            "knowledge:second",
            "second:wiki:frame:causal-review",
            "3",
            "Second causal review",
        ),
        (
            "knowledge:second",
            "second:wiki:frame:experimental-causal",
            "1",
            "Experimental causal review",
        ),
        (
            "knowledge:public",
            "public:wiki:frame:causal",
            "1",
            "Public causal review",
        ),
    ] {
        resolver.insert(
            Some(provider.into()),
            target,
            ForeignTargetResolution {
                target_ref: target.into(),
                provider_ref: Some(provider.into()),
                revision: Some(revision.into()),
                availability: TargetAvailability::Available,
                payload: Some(json!({"title": title, "canonical_ref": target})),
                notices: vec![],
            },
        );
    }
    resolver.insert(
        Some("knowledge:offline".into()),
        "offline:wiki:frame:causal-review",
        ForeignTargetResolution {
            target_ref: "offline:wiki:frame:causal-review".into(),
            provider_ref: Some("knowledge:offline".into()),
            revision: Some("11".into()),
            availability: TargetAvailability::Unavailable,
            payload: Some(json!({"stale": "must-not-leak"})),
            notices: vec!["provider currently unavailable".into()],
        },
    );
    resolver
}

fn scope(values: &[&str], allow_payload: bool) -> PortalScope {
    PortalScope::new(
        values.iter().map(|value| (*value).to_owned()),
        allow_payload,
    )
}

fn projection() -> MetaKnowledgeProjection {
    MetaKnowledgeProjection::rebuild(&documents(), &bindings(), 9).unwrap()
}

#[test]
fn one_meta_node_has_multiple_foreign_manifestations_without_ownership_takeover() {
    let projection = projection();
    let resolver = resolver();
    let portal = MetaPortal::new(&projection, Some(&resolver));
    let response = portal.manifestations(
        "ql-mef:wiki:node:mef-l1",
        &PortalScope::unrestricted_payload(),
    );
    let refs = response
        .manifestations
        .iter()
        .map(|value| value.target_ref.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(refs.len(), 5);
    assert!(refs.contains("glade:wiki:frame:causal-review"));
    assert!(refs.contains("second:wiki:frame:causal-review"));
    assert!(refs.contains("offline:wiki:frame:causal-review"));
    assert!(refs.contains("second:wiki:frame:experimental-causal"));
    assert!(refs.contains("public:wiki:frame:causal"));
    assert!(
        response
            .manifestations
            .iter()
            .all(|value| value.qualified_relation && !value.semantic_equivalence_asserted)
    );
    assert!(!projection.contains_foreign_object("glade:wiki:frame:causal-review"));
    assert!(!projection.contains_foreign_object("second:wiki:frame:causal-review"));
}

#[test]
fn one_foreign_frame_can_enter_multiple_ql_mef_fields() {
    let projection = projection();
    let resolver = resolver();
    let portal = MetaPortal::new(&projection, Some(&resolver));
    let response = portal.meta_context(
        "glade:wiki:frame:causal-review",
        &scope(&["scope:glade"], true),
    );
    let meta_refs = response
        .mappings
        .iter()
        .map(|value| value.ql_mef_ref.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        meta_refs,
        BTreeSet::from([
            "ql-mef:wiki:node:mef-l1",
            "ql-mef:wiki:node:relation-family-a"
        ])
    );
    assert!(
        response
            .mappings
            .iter()
            .any(|value| { value.operator_ref.as_deref() == Some("ql:structural:2.0.0:pair:A:1") })
    );
}

#[test]
fn unavailable_provider_preserves_binding_identity_but_has_no_payload() {
    let projection = projection();
    let resolver = resolver();
    let portal = MetaPortal::new(&projection, Some(&resolver));
    let response =
        portal.manifestations("ql-mef:wiki:node:mef-l1", &scope(&["scope:offline"], true));
    let offline = response
        .manifestations
        .iter()
        .find(|value| value.binding_ref == "ql-mef:binding:offline-l1")
        .unwrap();
    assert_eq!(offline.availability, TargetAvailability::Unavailable);
    assert_eq!(offline.target_ref, "offline:wiki:frame:causal-review");
    assert_eq!(
        offline.target_provider_ref.as_deref(),
        Some("knowledge:offline")
    );
    assert_eq!(offline.target_revision.as_deref(), Some("11"));
    assert!(offline.payload.is_none());
    assert_eq!(offline.origin, MappingOrigin::Recognised);
}

#[test]
fn scope_controls_binding_visibility_and_payload_traversal_independently() {
    let projection = projection();
    let resolver = resolver();
    let portal = MetaPortal::new(&projection, Some(&resolver));

    let glade = portal.manifestations("ql-mef:wiki:node:mef-l1", &scope(&["scope:glade"], true));
    let glade_refs = glade
        .manifestations
        .iter()
        .map(|value| value.target_ref.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        glade_refs,
        BTreeSet::from(["glade:wiki:frame:causal-review", "public:wiki:frame:causal"])
    );

    let empty = portal.manifestations("ql-mef:wiki:node:mef-l1", &scope(&[], true));
    assert_eq!(empty.manifestations.len(), 1);
    assert_eq!(
        empty.manifestations[0].target_ref,
        "public:wiki:frame:causal"
    );

    let restricted =
        portal.manifestations("ql-mef:wiki:node:mef-l1", &scope(&["scope:glade"], false));
    let restricted_glade = restricted
        .manifestations
        .iter()
        .find(|value| value.binding_ref == "ql-mef:binding:glade-l1")
        .unwrap();
    assert_eq!(
        restricted_glade.availability,
        TargetAvailability::Restricted
    );
    assert!(restricted_glade.payload.is_none());

    let all = portal.manifestations(
        "ql-mef:wiki:node:mef-l1",
        &PortalScope::unrestricted_payload(),
    );
    assert_eq!(all.manifestations.len(), 5);
}

#[test]
fn bidirectional_external_meta_external_route_preserves_every_ref_and_provider() {
    let projection = projection();
    let resolver = resolver();
    let portal = MetaPortal::new(&projection, Some(&resolver));
    let request = CrossWikiTraversalRequest {
        start_ref: "glade:wiki:frame:causal-review".into(),
        relation: Some("refracts".into()),
        operator_ref: Some("mef:lens:L1@1".into()),
        lens_ref: Some("mef:lens:L1@1".into()),
        max_hops: 6,
    };
    let response = portal
        .cross_wiki_traverse(&request, &scope(&["scope:glade", "scope:second"], true))
        .unwrap();
    let route = response
        .routes
        .iter()
        .find(|route| route.destination_ref == "second:wiki:frame:causal-review")
        .unwrap();
    assert_eq!(route.steps.len(), 4);
    assert_eq!(route.steps[0].from_surface, MetaRouteSurface::External);
    assert_eq!(route.steps[0].to_surface, MetaRouteSurface::Binding);
    assert_eq!(route.steps[1].to_surface, MetaRouteSurface::Meta);
    assert_eq!(route.steps[2].to_surface, MetaRouteSurface::Binding);
    assert_eq!(route.steps[3].to_surface, MetaRouteSurface::External);
    assert_eq!(route.steps[0].from_ref, "glade:wiki:frame:causal-review");
    assert_eq!(route.steps[1].to_ref, "ql-mef:wiki:node:mef-l1");
    assert_eq!(route.steps[3].to_ref, "second:wiki:frame:causal-review");
    assert_eq!(
        route.steps[0].provider_ref.as_deref(),
        Some("knowledge:glade")
    );
    assert_eq!(
        route.steps[3].provider_ref.as_deref(),
        Some("knowledge:second")
    );
    assert!(
        route
            .steps
            .iter()
            .all(|step| step.qualified_relation && !step.semantic_equivalence_asserted)
    );
    assert!(response.notices[0].contains("do not assert semantic equivalence"));
}

#[test]
fn meta_entry_and_reverse_entry_are_both_first_class() {
    let projection = projection();
    let resolver = resolver();
    let portal = MetaPortal::new(&projection, Some(&resolver));
    let all_scope = scope(&["scope:glade", "scope:second"], true);

    let from_meta = portal
        .cross_wiki_traverse(
            &CrossWikiTraversalRequest {
                start_ref: "ql-mef:wiki:node:mef-l1".into(),
                relation: Some("refracts".into()),
                operator_ref: Some("mef:lens:L1@1".into()),
                lens_ref: None,
                max_hops: 4,
            },
            &all_scope,
        )
        .unwrap();
    assert!(
        from_meta
            .routes
            .iter()
            .any(|route| route.destination_ref == "glade:wiki:frame:causal-review")
    );
    assert!(
        from_meta
            .routes
            .iter()
            .any(|route| route.destination_ref == "second:wiki:frame:causal-review")
    );

    let reverse = portal
        .cross_wiki_traverse(
            &CrossWikiTraversalRequest {
                start_ref: "second:wiki:frame:causal-review".into(),
                relation: Some("refracts".into()),
                operator_ref: Some("mef:lens:L1@1".into()),
                lens_ref: None,
                max_hops: 6,
            },
            &all_scope,
        )
        .unwrap();
    assert!(
        reverse
            .routes
            .iter()
            .any(|route| route.destination_ref == "glade:wiki:frame:causal-review")
    );
}

#[test]
fn proposed_binding_never_masquerades_as_recognised_mapping() {
    let projection = projection();
    let resolver = resolver();
    let portal = MetaPortal::new(&projection, Some(&resolver));
    let response =
        portal.manifestations("ql-mef:wiki:node:mef-l1", &scope(&["scope:second"], true));
    let proposed = response
        .manifestations
        .iter()
        .find(|value| value.binding_ref == "ql-mef:binding:second-l1-proposal")
        .unwrap();
    assert_eq!(proposed.origin, MappingOrigin::Proposed);
    assert_ne!(proposed.origin, MappingOrigin::Recognised);
}

#[test]
fn projection_rebuild_preserves_portal_mapping_paths_without_local_id_leakage() {
    let docs = documents();
    let original_bindings = bindings();
    let first = MetaKnowledgeProjection::rebuild(&docs, &original_bindings, 1).unwrap();
    let mut reversed_docs = docs.clone();
    reversed_docs.reverse();
    let mut reversed_bindings = original_bindings.clone();
    reversed_bindings.reverse();
    let second = MetaKnowledgeProjection::rebuild(&reversed_docs, &reversed_bindings, 2).unwrap();
    let scope = PortalScope::unrestricted_payload();
    let first_response =
        MetaPortal::new(&first, None).manifestations("ql-mef:wiki:node:mef-l1", &scope);
    let second_response =
        MetaPortal::new(&second, None).manifestations("ql-mef:wiki:node:mef-l1", &scope);
    let first_refs = first_response
        .manifestations
        .iter()
        .map(|value| (&value.binding_ref, &value.target_ref))
        .collect::<BTreeSet<_>>();
    let second_refs = second_response
        .manifestations
        .iter()
        .map(|value| (&value.binding_ref, &value.target_ref))
        .collect::<BTreeSet<_>>();
    assert_eq!(first_refs, second_refs);
    let serialized = serde_json::to_string(&first_response).unwrap();
    assert!(!serialized.contains("projection_id"));
    assert!(!serialized.contains("Bimba Graph"));
}

#[test]
fn language_neutral_fixture_is_portable_and_does_not_assert_equivalence() {
    let fixture: Value = serde_json::from_str(CONFORMANCE).unwrap();
    let raw = serde_json::to_string(&fixture).unwrap().to_lowercase();
    assert!(raw.contains("ql-mef/meta-portal/v1"));
    assert!(!raw.contains("glade"));
    assert!(!raw.contains("aikit"));
    assert!(!raw.contains("bimba graph"));
    assert_eq!(fixture["semantic_equivalence_asserted"], false);
}
