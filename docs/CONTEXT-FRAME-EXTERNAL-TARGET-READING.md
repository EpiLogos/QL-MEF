# Context Frame external-target reading

Status: executable projection contract for QL-MEF #66, stacked on the Q6 Context Frame grammar line.

The seven named Context Frames are QL-MEF-owned readings of structural relations. They are **not** a configuration ontology for O:I, AIKit, Actuation, Factory, Workcell, Central, or an arbitrary external application.

`ql.mef.context-frame-reading/1.0.0` therefore begins from an explicit external sixfold mapping supplied by the caller:

```text
external target
+ explicit six-position mapping
+ mapping source/version identity
+ structural probe
        ↓
QL-MEF context-frame operator
        ↓
exact | partial | ambiguous | no reading
```

The external member names do not determine the result. The operator compares only the supplied QL position / Name-or-Power face / grain relation against the canonical Context Frame selections already defined by `ql.mef.context-frame/1.0.0`.

## Provenance

Every result retains:

- `target_ref`;
- `mapping_source_ref`;
- a stable mapping identity digest;
- `provider_ref`;
- reading + grammar operator versions;
- origin (`Derived`, `Proposed`, `Recognised`);
- evidence refs supplied by the caller;
- the exact structural probe;
- exact/partial/ambiguous/none status.

The built-in digest is an identity digest, not a security checksum. Callers can retain stronger source/content digests as provenance refs.

## Abstention is valid

Incomplete structural evidence is not silently rounded to a named frame. A probe may remain partial or ambiguous; incompatible evidence returns `NoReading`. This is important when QL is reading a technological composition that was not authored in QL terms.

## No runtime authority

A reading never activates, configures, owns, mutates or governs the external target. `ExternalContextFrameReading::is_runtime_authority()` is deliberately always false.

This makes removal equally simple: stop requesting or retaining the optional reading and the underlying technological sixfold remains unchanged.

## O:I fixture

The conformance tests include one mapping whose external names are current O:I products. Those names live only in the external test fixture. QL-MEF core does not import O:I product semantics, and ordinary O:I operation requires no QL provider.

The same test proves renaming external members does not change the structural reading when the explicit mapping relation remains equivalent.
