# Q4 client adapters and NoQL parity

Q4 proves that Factory and AIKit can consume QL/MEF as optional enrichment without transferring ownership of client identity or making QL a hidden prerequisite.

## Identity ownership

Client references and revisions remain opaque client-owned values. `FactorySubject` and `AiKitSubject` are thin type wrappers over one shared `ClientSubject`; they do not parse or rename client identity. A `ClientRecord` remains structurally separate from its QL attachment, so the original client payload/ref/revision can be returned unchanged in every adapter mode.

QL refraction uses the client reference directly as the `QlTarget.subject`. There is no adapter-generated `qlTargetRef` and no legacy-to-canonical identity translation table.

## Explicit modes

`QlMode` is explicit:

- `Disabled` — do not invoke QL; return the client record unchanged with a disabled attachment.
- `Optional` — attempt enrichment when available; absence, incompatibility, or an unadvertised operation returns a non-fatal unavailable attachment. Other provider failures become a non-fatal failed attachment.
- `Required` — absence or provider failure is a hard adapter error because the caller explicitly requested QL as a prerequisite.

Invalid QL/MEF coordinates are never treated as optional failure. A mismatched sublens is rejected before provider execution rather than coerced.

## Provider lifecycle

A degraded provider may still enrich when it advertises `refract`; the attachment carries degraded health alongside the reading. Provider/version, model, configuration ref, source/input refs and revisions remain inspectable through Q3 provenance.

## Factory and AIKit parity

Both public adapter surfaces delegate to the same adapter core. There is no privileged Factory path and no separate AIKit semantic layer in QL-MEF. Tests exercise the same shared ref through both surfaces and require the ref/revision and resulting QL target subject to remain byte-for-byte unchanged.

## Shared-floor status

Q4 consumes the Factory `factory.interop/v1-alpha` ref/revision shape as an alpha dependency while Factory #113 remains open. The Q4 fixture records the demonstrated shared subject `factory:claim:c-1` / `sha256:claim-c-1-r1` and uses standalone QL-MEF canonical `mef:lens:L3@1` with `targetRef` equal to the client-owned Factory ref.

The currently observed legacy Factory QL composition strings (`qlform:factory-development/v1`, `day:2.3`, `lens:L3`, `qltarget:claim-whole`) are explicitly rejected by QL-MEF tests rather than translated. Their resolution belongs to the shared interop floor in Factory #113 because the Factory schema identifies Standalone QL/MEF as semantic owner of that composition.

Until #113 settles the cross-repo contract, Q4 may be implementation-complete on the QL-MEF side but must remain blocker-visible and must not be closed as the final cross-repo adapter slice.
