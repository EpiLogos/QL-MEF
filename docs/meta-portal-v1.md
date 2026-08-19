# QL-MEF Meta Portal v1

**Contract:** `ql-mef/meta-portal/v1`  
**Mapping floor:** `QL-MEF MetaBinding v1`  
**Wiki floor:** `okf-wiki/v1`

The portal makes QL-MEF's meta-wiki useful across independent project wikis without turning QL-MEF into their parent ontology or identity authority.

## Query directions

Three operations are first-class:

```text
manifestations(meta_ref, scope)
  QL/MEF object → qualified foreign manifestations

meta_context(external_ref, scope)
  foreign Wiki/Frame ref → QL/MEF mappings that contextualise it

cross_wiki_traverse(start_ref, relation/operator/lens, hop budget, scope)
  bounded stable-ref path through MetaBindings and QL-MEF meta relations
```

A typical route is:

```text
foreign Frame A
  → MetaBinding A
  → ql-mef:wiki:node:mef-l1
  → MetaBinding B
  → foreign Frame B
```

Each step retains its stable refs, mapping origin, provider ref, revisions, provenance and qualification state.

## Identity and equivalence law

A `MetaBinding` says that a project object is related to a QL-MEF meta object in a particular qualified way. It does **not** say that two project objects mapped to the same QL position/lens/operator are semantically equivalent.

The portable route structures therefore carry:

```text
qualified_relation = true
semantic_equivalence_asserted = false
```

and never manufacture a direct foreign-object → foreign-object equivalence edge.

## Scope and privacy law

Bindings and payloads have separate visibility decisions.

```text
binding.scope_refs empty
  → public/unscoped binding

caller scope intersects binding.scope_refs
  → binding is eligible

caller scope empty
  → only unscoped bindings are visible

allow_all_scopes = true
  → explicit administrative/unrestricted binding visibility

allow_payload = false
  → eligible binding refs remain inspectable, foreign payload traversal is restricted
```

An empty scope is intentionally **not** an implicit privileged scope.

Foreign provider availability is another independent axis. If a provider is absent, the `MetaBinding`, target ref, provider ref, revisions and provenance survive; availability is reported `unavailable` and payload is absent. The portal additionally drops any stale payload accidentally returned alongside an unavailable resolver status.

## MetaBinding v1 additive fields

QW3 adds two optional stable routing fields without changing canonical target identity:

```text
target_provider_ref
operator_ref
```

`target_provider_ref` identifies the foreign KnowledgeProvider/ProjectMap provider needed to resolve material. It is not a row/database ID and is not substituted for `target_wiki_ref` or `target_frame_ref`.

`operator_ref` qualifies the mapping with a stable QL/MEF structural/lens/operator ref. It enables bounded operator/lens portal queries without interpreting project content locally.

## Mapping origins

The existing distinct origins remain load-bearing:

```text
authored
recognised
derived
proposed
```

Portal results and route steps preserve the origin. A proposed mapping cannot become recognised merely because traversal reaches it.

## KnowledgeRoute / ProjectMap compatibility

`MetaRouteStep` is deliberately shaped as a provider-neutral route hop:

```text
from_ref / from_surface
to_ref / to_surface
relation / origin
binding_ref
provider_ref
from_revision / to_revision
provenance
qualification flags
```

The surfaces are `meta`, `binding`, and `external`. A future AIKit ProjectMap adapter can translate these steps into its wider route envelope without changing the QL-MEF mapping semantics.

Projection-local graph IDs are never emitted by the portal.

## Bimba boundary

Cross-wiki traversal is over the **QL-MEF Meta-Knowledge Graph Projection** plus foreign stable refs. Nothing in the portal identifies that projection with the canonical Epi-Logos Bimba Graph, and the portal emits no Bimba bridge edges.
