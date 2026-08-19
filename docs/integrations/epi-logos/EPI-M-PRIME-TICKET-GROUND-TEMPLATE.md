# Epi M′ implementation ticket — Coordinate / Relation Ground

Use this section in every substantial M0′–M5′ implementation ticket before feature acceptance is specified.

The purpose is not to make tickets mechanically sixfold. It is to stop a partial implementation from becoming the accidental ontology of an authored M/Bimba subsystem.

PRE-D returned two additional laws which the template now makes explicit:

1. source representation is part of provenance: exact spelling, alternate notation, rootless/meta refs and partial relations must be retained rather than repaired silently;
2. structural existence and operational readiness are different dimensions: a coordinate/ref can exist even when no capability/provider/Surface/Agent disclosure is present.

```markdown
## Coordinate / Relation Ground

### Bimba source scope
- canonical/source documents:
- exact source repository + revision:
- source-reader / normalization / conformance ref:
- machine-readable coordinate source(s):
- machine-readable relation source(s):
- source file / record hashes or locked revision:
- source record classes crossed by this tranche:
- known source conflicts/gaps:
- alternate notation / alias groups crossed:
- rootless/meta refs crossed, if any:
- partial source relations crossed, if any:

### Coordinate manifest / index
- QL-MEF coordinate/index ref:
- root coordinate:
- exact source spelling(s):
- in-scope coordinates:
- recursive/deep coordinates required by this tranche:
- parent/source-parent relation:
- Bimba ↔ Pratibimba exact-path reflection proof/ref:
- experimental/research coordinates, if any:

### Relation manifest / index
- QL-MEF relation/index ref:
- Bimba source relation(s) exercised by this tranche:
- QL-derived/formal relation(s) exercised:
- M↔M′ reflection relation(s):
- implementation/runtime relation(s):
- cross-M source relations exercised:
- endpoint direction/orientation:
- partial/missing endpoint standing, if any:
- unresolved relation gaps:

Do not collapse these relation classes into one generic `related-to` relation.

### Existing substrate mapping
For each material capability used by this tranche:

| Capability | M/M′ coordinate | Current implementation owner | Provider | Capability state | Provider/readiness | Surface/disclosure state | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | implemented / partial / unbound / research | ready / degraded / unavailable / not-assessed | rendered / hidden / undisclosed / not-applicable | |

A populated coordinate cell does not imply any of the implementation/readiness cells are green.

### Vertical rooting
- origin/instrument coordinate:
- carrier coordinate(s):
- review/governance coordinate(s):
- return/promotion coordinate(s):
- exact lineage carried by created artifacts/events/Actions:
- which carried relations are source relations vs implementation-flow relations:

### Parity acceptance
- [ ] **Source parity:** accepted source/revision is explicit and the relevant source records were actually read.
- [ ] **Source representation fidelity:** exact notation/aliases/meta refs/partial records crossed by the tranche are retained explicitly.
- [ ] **Coordinate parity:** exact root/path/parent/face/source Refs resolve without depth flattening.
- [ ] **Relation parity:** source kind/endpoints/direction/provenance survive, or the source gap remains explicit.
- [ ] **Relation-class fidelity:** Bimba-source, QL-derived, reflection, implementation/runtime and research relations are not silently conflated.
- [ ] **Operational parity:** behavior enacts the claimed coordinate/relation rather than merely labelling it.
- [ ] **Readiness fidelity:** structural existence is reported separately from capability/provider/Surface/Agent-disclosure readiness.
- [ ] **Experiential parity:** the human/agent Surface expresses the relation appropriately without architecture-as-dashboard.

### Anti-flattening checks
- [ ] no new local coordinate ontology replaces the QL-MEF/source index;
- [ ] missing implementation does not erase a Bimba coordinate;
- [ ] recursive coordinates retain their full source path;
- [ ] alternate notation is resolved explicitly rather than silently canonicalised;
- [ ] missing/meta source relation endpoints are not invented;
- [ ] provider availability does not become semantic authority;
- [ ] implementation packet/module calls are not written back as Bimba-source relations;
- [ ] governed return/promotion is not replaced by direct mutation;
- [ ] UI routes/components are not canonical semantic identity.
```

## Integrated-instrument addition

For Personal 4/5/0, Cosmic 1/2/3, or another cross-M instrument, add:

```markdown
### Composition ground

| Participant | Root/index ref | Participating coordinate(s) | Source relation(s) into composition | Implementation/composition relation | Current readiness | Native owner |
| --- | --- | --- | --- | --- | --- | --- |
| | | | | | | |

The integrated instrument composes already-rooted coordinates. It does not create replacement identities for them, infer source relations from UI composition, or treat participant root existence as proof that the participant application is implemented.
```
