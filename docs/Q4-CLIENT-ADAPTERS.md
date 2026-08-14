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

Q4 now consumes the Factory `factory.interop/v1` contract established by Factory #113. The cross-repo fixture records the demonstrated shared subject `factory:claim:c-1` / `sha256:claim-c-1-r1` and uses standalone QL-MEF canonical `mef:lens:L3@1` with `targetRef` equal to the client-owned Factory ref.

Factory's parent `factory.interop` contract composes the standalone QL-MEF composition schema through an external `$ref`; the former Factory-local inline QL definition is not part of the parent contract. Factory's Python, Node, TypeScript and Rust conformance consumers are expected to resolve/enforce that same schema boundary rather than merely accepting arbitrary QL strings.

The legacy Factory QL composition strings (`qlform:factory-development/v1`, `day:2.3`, `lens:L3`, `qltarget:claim-whole`) remain explicit rejection cases in QL-MEF. They are not translated, aliased or silently normalised.

Factory #113 therefore no longer blocks the QL-MEF-side adapter contract. Q4 remains open until its full cross-repository Closure is evidenced: the QL-MEF Rust gate must pass and both principal clients, including AIKit, must consume the standalone module through explicit optional adapters without semantic ownership transfer.
