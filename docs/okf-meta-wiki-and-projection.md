# QL-MEF OKF meta-wiki and Meta-Knowledge Graph Projection

**Profile floor:** `okf-wiki/v1`  
**QL-MEF extension:** `ql-mef/wiki/v1`  
**Mapping contract:** `QL-MEF MetaBinding v1`

QL-MEF is a participant in the same open Wiki ecosystem it enriches. Its theoretical/operator/lens knowledge is authored as ordinary OKF Markdown carrying the open `okf-wiki/v1` profile. QL-MEF-specific ontology is additive producer data; it does not replace the shared Wiki identity envelope.

```text
QL-MEF authored OKF Markdown
  └─ okf-wiki/v1 objects
       ├─ QL definitions
       ├─ relation/operator definitions
       ├─ MEF lens/sublens knowledge
       ├─ derivation/evidence
       └─ mapping/provider knowledge
              │
              ▼ rebuild
QL-MEF Meta-Knowledge Graph Projection
       ├─ canonical refs + revisions
       ├─ authored/mechanical Wiki relations
       ├─ MetaBindings to foreign refs
       └─ later derived/refraction relations
```

The projection is disposable operational/meta-relational state. Its `projection_id` values are implementation bindings. They are never cross-repository identity and are not serialised into `MetaBinding`.

## Three different relations

Do not collapse these into one parent hierarchy:

```text
containment
  QL-MEF WikiSpace → QL-MEF SubWikiSpace / Node

federation
  AIKit / ProjectMap horizon → independent knowledge providers

meta-relation
  QL-MEF concept/lens/field → MetaBinding → foreign WikiSpace/Frame
```

A foreign `target_wiki_ref` or `target_frame_ref` is an opaque stable reference. QL-MEF indexes the binding but does not create a local WikiNode standing in for the foreign object.

## MetaBinding

The portable record retains:

```text
binding_ref
ql_mef_ref
target_wiki_ref
target_frame_ref?
relation
scope_refs
origin
provenance
ql_mef_revision?
target_revision?
extensions
```

Origins are distinct:

```text
authored
recognised
derived
proposed
```

Derived/proposed mappings therefore cannot silently masquerade as recognised or authored mappings.

The binding survives foreign provider unavailability because its identity is not a live provider row. Whether the foreign provider can currently resolve the target is operational state, not mapping identity.

## Reserved Bimba boundary

The projection's complete name is **QL-MEF Meta-Knowledge Graph Projection**.

The **Epi-Logos Bimba Graph** is a different pre-existing canonical graph in the wider system. No projection ID, QL-MEF Wiki ref, Neo4j ID, or MetaBinding target is a Bimba identity merely by resemblance.

A future relation requires an explicit versioned bridge/binding contract with independent authority and provenance. This programme does not implement that bridge.

## Rebuild law

Rebuilding or physically moving authored files must preserve:

- Wiki canonical `ref`;
- Wiki `revision`;
- provenance/source refs;
- MetaBinding refs and foreign target refs.

Projection-local IDs may change without semantic consequence. The Rust implementation currently sorts canonical refs for deterministic local rebuilding, but clients must not rely on those local integers.

## Dependency firewall

Ordinary project Wiki correctness does not depend on this QL-MEF meta-wiki or projection. Project wikis own their objects. QL-MEF becomes meta to them only through explicit mapping/refraction relations.
