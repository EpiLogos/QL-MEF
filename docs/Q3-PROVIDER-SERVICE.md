# Q3 provider and service boundary

Q3 places replaceable provider contracts behind one transport-independent QL/MEF operation family: `capabilities`, `locate`, `refract`, `relate`, and `synthesise`.

## Provider classes

`FormalKernel` and `SemanticRefraction` are explicit capability classes. A provider may expose one or both. The service does not infer semantic capability from formal capability, and it does not branch on provider brand names. Replaceability is expressed through the public `QlProvider` trait and `QlService::replace_provider`.

`locate` may return a unique locus, ambiguity, insufficient information, or unsupported mapping. It is not required to manufacture a coordinate. Semantic operations return disclosure status and optional confidence rather than disguising model output as deterministic structure.

## Negotiation and lifecycle

Clients may call `capabilities` before advanced operations. Provider health is explicit: `absent`, `available`, `degraded`, or `incompatible`. A degraded provider may serve only the operations it still advertises. An incompatible or absent provider cannot serve advanced operations. Unadvertised operations fail visibly.

Input limits are part of the capability contract and are enforced at the service boundary. `relate` requires at least two subjects and `synthesise` requires at least one prior reading.

## Provenance

Q3 provenance identifies schema and MEF registry versions, provider/version, operation, input refs and revisions, optional model, optional provider configuration ref, result class, and warnings. Reading evidence refs remain explicit source references. Semantic/stochastic fixtures therefore carry model, configuration, source, and source-revision evidence together.

## Transport neutrality

`ServiceRequest` and `ServiceResponse` are transport envelopes over the same Rust operations. Dispatch delegates to the same service methods used by direct in-process callers. Tests require a deterministic direct call and its dispatched equivalent to return the same semantic result.

`schemas/q3/service.schema.json` provides a language-neutral capability and request contract. Q3 does not introduce HTTP, RPC, a daemon, a database, or a provider-specific service framework; those may be adapters later without redefining operation meaning.

## Ownership boundary

QL-MEF continues to preserve client-owned references. It does not create Factory Project/Run/Action identities, AIKit context identities, Workcell materialisation identities, or Loop Runtime recurrence state.
