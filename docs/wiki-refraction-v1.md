# QL-MEF Wiki refraction v1

**Wire contract:** `ql-mef/wiki-refraction/v1`  
**Structural floor:** `ql:structural:2.0.0`  
**Open Wiki floor:** `okf-wiki/v1`

QL-MEF accepts caller-owned Wiki fields and returns separate readings. It never translates caller identity, mutates the target, or promotes derived relations into authored project knowledge.

## Target law

A request carries an immutable caller snapshot:

```text
stable target_ref / target_frame_ref
revision
snapshot hash
provenance
subject refs + optional QL coordinates
relations + epistemic origin/provenance
material
optional structural field
```

Supported target shapes are node-local wholes, Frames/subgraphs, A/B/C pair instances, D1/D2/D3 fields, and bounded WikiSpace fields.

For structural targets the wire record carries the complete field identity rather than only vertex IDs. QL-MEF validates family, pair index, degree, orientation/expansion side, coordinates and canonical operator ref against `ql:structural:2.0.0` before provider availability is considered.

This is load-bearing for the A(2,3) / C(2,3) case: identical vertices do not identify the refractive field.

## MEF law

The structural adapter and MEF provider have different jobs:

```text
Wiki adapter
  validates caller field and preserves its topology/provenance

QlProvider
  supplies lens/sublens disclosure

WikiReading wrapper
  combines the preserved target field with provider disclosure as an attachment
```

The current production reference provider is `RegistryDisclosureProvider`. It deterministically exposes the canonical QL-MEF registry meanings for all twelve lenses and 72 sublenses through the existing Q3 `QlProvider` trait. It proves the external provider path and is intentionally not presented as a model-backed implicit-relation discovery system.

A future semantic provider can disclose richer relation/traversal/tension/absence/evidence candidates behind the same contract. The wire format already has distinct channels for those results.

## WikiReading

A reading retains at minimum:

```text
reading_ref
reading_type = MEF-derived
target_ref / target_frame_ref
target revision + snapshot hash
provider/version/health
lens + sublens refs
QL form/operator refs
harmonic field ref
disclosure + status/confidence
derived subgraph
relation candidates
traversal candidates
tensions
absences
evidence demands
explanation
evidence refs
provenance/result class/warnings
```

The `derived_subgraph` initially carries the caller field with its original epistemic origins. It is not a replacement graph. New MEF relation candidates use the separate candidate channel and remain derived/proposed until the target Wiki's governance recognises them.

## Availability law

`mode` is one of:

```text
disabled
  validate the target, then return no reading and leave client material unchanged

optional
  provider absence/degradation/unadvertised capability is inspectable and nonfatal

required
  provider or requested capability absence is a hard error
```

Invalid coordinates, operator refs, family/pair identities, or lens/sublens mismatches are validation errors in every mode. They are never downgraded to provider unavailability.

A degraded provider may still serve an operation it explicitly advertises; response status remains `degraded`.

## External executable

`ql-wiki-refraction` is a JSON stdin/stdout adapter over `RegistryDisclosureProvider`:

```text
cat request.json | ql-wiki-refraction
```

The executable consumes and emits the same language-neutral Rust wire types used by in-process callers. It gives Python/TypeScript/other clients a real provider path without inventing HTTP or changing Q3 transport neutrality.

## Promotion boundary

QL-MEF may propose:

- latent relation candidates;
- traversal candidates;
- ontology/meta-wiki amendments.

It does not author those into the target Wiki automatically. Project policy owns project knowledge promotion; QL-MEF governance owns its own meta-wiki recognition.

## Bimba boundary

Nothing in this contract names or writes the Epi-Logos Bimba Graph. QL-MEF's rebuildable external index remains the **QL-MEF Meta-Knowledge Graph Projection**. Any future Bimba relation requires a separate explicit bridge contract.
