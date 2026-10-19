# QL capability matrix

Standing: **agent-inference · source-recovered draft for review**. Basis: local checkout `781981374eaa1f02952249dcdbca1af671d1ece4`, 6 September 2026.

This matrix addresses concrete product powers alongside the [whole account](ql.html#whole). Its 33 capability families link the need, operation and useful result to native sources and a governing account unit. It is a selected functional inventory; operation families do not establish every possible integration.




## Native CLI catalog and parity

Discovery command: `target/debug/ql capabilities --json` (built with `cargo build -p ql-cli --bin ql`). The emitted command identifiers are mapped below to their maintained capability records.

<!-- cli-catalog:start -->
| CLI identity | Capability |
| --- | --- |
| `agent-event.frame` | [cap.ql.agent-event](#cap-ql-agent-event) · [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `agent-event.harmonic` | [cap.ql.agent-event](#cap-ql-agent-event) · [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `agent-event.project` | [cap.ql.agent-event](#cap-ql-agent-event) · [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `agent-event.validate` | [cap.ql.agent-event](#cap-ql-agent-event) · [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `config.apply` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.configuration](#cap-ql-configuration) |
| `config.contribute` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.configuration](#cap-ql-configuration) |
| `config.plan` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.configuration](#cap-ql-configuration) |
| `config.reset` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.configuration](#cap-ql-configuration) |
| `config.validate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.configuration](#cap-ql-configuration) |
| `context-frame.list` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.context-frames](#cap-ql-context-frames) |
| `context-frame.team-space` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.team-space](#cap-ql-team-space) |
| `epi-agent.constitution` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.epi-agent](#cap-ql-epi-agent) |
| `epi-agent.faculty` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.epi-agent](#cap-ql-epi-agent) |
| `epi-agent.invoke` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.epi-agent](#cap-ql-epi-agent) |
| `kernel.apply` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.kernel-operators](#cap-ql-kernel-operators) |
| `kernel.capabilities` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.kernel-operators](#cap-ql-kernel-operators) |
| `kernel.coverage` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.m-registry-ledger](#cap-ql-m-registry-ledger) |
| `kernel.ledger` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.m-registry-ledger](#cap-ql-m-registry-ledger) |
| `kernel.m3` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.m3-engine](#cap-ql-m3-engine) |
| `kernel.validate-ledger` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.m-registry-ledger](#cap-ql-m-registry-ledger) |
| `matheme.derive` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `matheme.shadow` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `mef.lenses` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.lens-registry](#cap-ql-lens-registry) |
| `nara.calculate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-identity](#cap-ql-nara-identity) |
| `nara.capabilities` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-identity](#cap-ql-nara-identity) |
| `nara.context` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-dialogue](#cap-ql-nara-dialogue) |
| `nara.coordinate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-dialogue](#cap-ql-nara-dialogue) |
| `nara.delegate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-dialogue](#cap-ql-nara-dialogue) |
| `nara.enrichment` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-dialogue](#cap-ql-nara-dialogue) |
| `nara.inspect` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-identity](#cap-ql-nara-identity) |
| `nara.personal-current` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-identity](#cap-ql-nara-identity) |
| `nara.personal-recompose` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-identity](#cap-ql-nara-identity) |
| `nara.presence-consent` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-identity](#cap-ql-nara-identity) |
| `nara.receive` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-dialogue](#cap-ql-nara-dialogue) |
| `nara.transit` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.nara-identity](#cap-ql-nara-identity) |
| `service.capabilities` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.service-negotiation](#cap-ql-service-negotiation) |
| `service.negotiate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.service-negotiation](#cap-ql-service-negotiation) |
| `shape.presentation` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.shape](#cap-ql-shape) |
| `stage.bind` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.stage](#cap-ql-stage) |
| `stage.evaluate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.stage](#cap-ql-stage) |
| `stage.retire` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.stage](#cap-ql-stage) |
| `stage.state` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.stage](#cap-ql-stage) |
| `stage.unbind` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.stage](#cap-ql-stage) |
| `techne.reading` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.techne-reading](#cap-ql-techne-reading) |
| `vak.capabilities` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-source](#cap-ql-vak-source) |
| `vak.compose` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-composition](#cap-ql-vak-composition) |
| `vak.context` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-context](#cap-ql-vak-context) |
| `vak.locate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-source](#cap-ql-vak-source) |
| `vak.workflow-types` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-composition](#cap-ql-vak-composition) |
| `verify` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.cli-verification](#cap-ql-cli-verification) |
<!-- cli-catalog:end -->

Parity: all 12 identifiers emitted by `ql capabilities --json` map to at least one capability. The catalog does not imply that every QL capability is a CLI command: pairing, shapes, external frames, adapters, wiki contracts and kernel parity remain library-owned. Service reading dispatch is recorded as a `gap`: `service.negotiate` exposes its availability, while the observed native provider is absent and the CLI executes no reading.

## Seed × field contribution

[View declarations](capability-matrix.json) · [Editable CSV](capability-matrix.csv). Select a populated cell for its source and capability links. Unassessed cells carry no assertion.

| Seed | O:I whole | Central | Actuation | AIKit | Software Factory | Workcell |
| --- | --- | --- | --- | --- | --- | --- |
| [Why?](ql.html#whole/why) | [1 capabilities](#field-q0-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |
| [What?](ql.html#whole/what) | [8 capabilities](#field-q1-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |
| [How?](ql.html#whole/how) | [4 capabilities](#field-q2-S) | Unassessed | Unassessed | [3 capabilities](#field-q2-S2) | Unassessed | Unassessed |
| [Who / Whereby?](ql.html#whole/whereby) | [3 capabilities](#field-q3-S) | [3 capabilities](#field-q3-S0) | Unassessed | Unassessed | [3 capabilities](#field-q3-S3) | Unassessed |
| [Where / When?](ql.html#whole/context) | [1 capabilities](#field-q4-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |
| [Why-For?](ql.html#whole/purpose) | [1 capabilities](#field-q5-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |

<details>
<summary>Read the field contributions and their capability links</summary>

<a id="field-q0-S"></a>

### Why? → O:I whole

People and Agents use patterns to understand a subject, but the pattern can remain implicit or become a label detached from the work. Quaternal Logic gives relational understanding an explicit form that can be inspected, compared and tested. Its research begins in the Epi-Logos inquiry into mind, relation and interiority; its software makes sufficiently specified parts of that inquiry operational.

[kernel parity](#cap-ql-kernel-parity)

[Source account passage](ql.html#whole/why) · placement: agent-inference.

<a id="field-q1-S"></a>

### What? → O:I whole

Quaternal Logic / MEF is a Rust software product with a native ql CLI, a shared formal kernel, twelve MEF lenses and versioned service and client contracts. It computes defined structural relations, exposes canonical registries, retrieves source-bearing Vāk context and supports attributable readings of existing subjects. The broader research programme develops further operations through explicit contracts and evidence.

[cli discovery](#cap-ql-cli-discovery) · [kernel operators](#cap-ql-kernel-operators) · [pairing](#cap-ql-pairing) · [lens registry](#cap-ql-lens-registry) · [context frames](#cap-ql-context-frames) · [vak source](#cap-ql-vak-source) · [service negotiation](#cap-ql-service-negotiation) · [client modes](#cap-ql-client-modes)

[Source account passage](ql.html#whole/what) · placement: agent-inference.

<a id="field-q2-S"></a>

### How? → O:I whole

A user or client selects a subject, preserves its identity and revision, then chooses a formal operation or a refractive perspective. QL validates the structure and returns the result with its operation, version and source basis. Deterministic kernel operations act directly; richer readings use an explicitly available provider. A client can compare the reading with its original material and retain the difference that proves useful.

[service readings](#cap-ql-service-readings) · [wiki refraction](#cap-ql-wiki-refraction) · [wiki portals](#cap-ql-wiki-portals) · [living wiki](#cap-ql-living-wiki)

[Source account passage](ql.html#whole/how) · placement: agent-inference.

<a id="field-q2-S2"></a>

### How? → AIKit

An AIKit client can supply a bounded WikiFrame: the target wiki, revision, selected nodes and explicit mapping information.

[wiki refraction](#cap-ql-wiki-refraction) · [wiki portals](#cap-ql-wiki-portals) · [living wiki](#cap-ql-living-wiki)

[Source account passage](ql.html#q2/wiki-encounter) · placement: agent-inference.

<a id="field-q3-S"></a>

### Who / Whereby? → O:I whole

QL serves people organising knowledge, developers exposing relational views and researchers testing formal propositions. Stable addresses, a whole anchor, complementary relations and multiple lenses let them inspect different aspects of the same subject. The subject remains owned by its originating product, while QL owns the formal reading and its provenance.

[shape](#cap-ql-shape) · [external frame](#cap-ql-external-frame) · [wiki structure](#cap-ql-wiki-structure)

[Source account passage](ql.html#whole/whereby) · placement: agent-inference.

<a id="field-q3-S0"></a>

### Who / Whereby? → Central

Central holds durable source; AIKit interprets and navigates knowledge; other native products enact their own responsibilities.

[pairing](#cap-ql-pairing) · [external frame](#cap-ql-external-frame) · [client modes](#cap-ql-client-modes)

[Source account passage](ql.html#q3/identity) · placement: agent-inference.

<a id="field-q3-S3"></a>

### Who / Whereby? → Software Factory

A Factory Run, AIKit Resource or Wiki subject carries its native identity into a QL request.

[pairing](#cap-ql-pairing) · [external frame](#cap-ql-external-frame) · [client modes](#cap-ql-client-modes)

[Source account passage](ql.html#q3/identity) · placement: agent-inference.

<a id="field-q4-S"></a>

### Where / When? → O:I whole

QL can be used at a terminal, through Rust libraries or as an optional provider within a larger O:I workflow. It is useful when a person needs to disclose a structure, compare relationships, or test whether a proposed distinction changes an outcome. Ordinary work and ordinary sources remain usable before any QL reading is requested; the chosen scope and available evidence determine the depth of a reading.

[vak context](#cap-ql-vak-context)

[Source account passage](ql.html#whole/context) · placement: agent-inference.

<a id="field-q5-S"></a>

### Why-For? → O:I whole

QL is for developing a more explicit and relationally aware encounter with knowledge and agentic work. It gives O:I and Epi-Logos a common formal resource while leaving each product responsible for its own actions and meaning. Its success lies in useful understanding and demonstrable operations: a reading that clarifies a decision, a structure that preserves relationships, or an experiment that teaches us which distinctions carry real consequences.

[cli verification](#cap-ql-cli-verification)

[Source account passage](ql.html#whole/purpose) · placement: agent-inference.

</details>


## Functional capabilities

| Capability | Need | Native operation | Result |
|---|---|---|---|
| [cap.ql.cli-discovery](#cap-ql-cli-discovery) | Find a usable operation | ql capabilities and version | Versioned command and kernel/service inventory |
| [cap.ql.kernel-operators](#cap-ql-kernel-operators) | Obtain a reproducible structural result | ql kernel apply; apply_operator | Typed result with input/output and operation version |
| [cap.ql.pairing](#cap-ql-pairing) | Distinguish constructions with coincident vertices | A/B/C pairing and D1/D2/D3 completion | Pair family and derivation retained |
| [cap.ql.shape](#cap-ql-shape) | Work with the actual available structure | QlShape and versioned shape contracts; ql shape presentation <exact-shape-ref> | Valid partials; 4x4, 4x4x4, 6x6, 10x10, seven-fold, relational-sixfold and 18-fold fields, one-fold compression of a disclosed two- or three-fold basis, and a read-only numerical presentation of an exact shape address |
| [cap.ql.m3-generation-law](#cap-ql-m3-generation-law) | Find the codons a hexagram yields under a matrix | pole::generation::generate; MatrixFamily apply_to_codon and apply_to_hexagram; ql_m3_generate | The 192 generation cells; base-partner matrix actions with no gaps; seed and pose map as declared laws; the pose bin read the same on every platform |
| [cap.ql.lens-registry](#cap-ql-lens-registry) | Inspect available lenses before reading | ql mef lenses; MEF registry | Twelve lens identities with face and square |
| [cap.ql.context-frames](#cap-ql-context-frames) | Select a defined cut of a relation | ql context-frame list; ContextFrameId | Named CF1-CF7 grammar and selected/unselected partitions |
| [cap.ql.team-space](#cap-ql-team-space) | Choose who answers a question inside Anima or Aletheia | ql context-frame team-space; team_space::TeamSpace | The S4′/S5′ team grid with depth, Vāk path and decision instruments; gate 6 stays the person’s |
| [cap.ql.m-registry-ledger](#cap-ql-m-registry-ledger) | Know which coordinates of the M tree exist, what is bound to them and what is proven | ql kernel coverage|ledger|validate-ledger; MRegistry and MLedger | The native M registry and the shared ledger: per-coordinate views, validation findings that are not completion, and coverage for one coordinate by stratum, axis and required readiness |
| [cap.ql.m1-engine](#cap-ql-m1-engine) | Read the M1 ring, torus and harmonic state for one event and cycle | ql kernel m1 <request.json>; m1_engine::engine_json and M1Engine | A versioned M1 state: the 12-fold clock and cell, source-literal and accepted-arithmetic readings kept apart, torus position, ratio evidence and the selected coordinate reading |
| [cap.ql.m2-engine](#cap-ql-m2-engine) | Read the Paraśakti field: descriptors, music, colour and material for an event | m2 descriptor field and finite operations; m2_engine; m2_condition; m2_vimarsha; m2_sky | Retained descriptor tables and finite operations over one 72-reading carrier, the M2-1 Vimarśā reading from supplied M1 ratio and M3 pose, the sky of planets and chakras with the map values, and a joint music/colour/material condition for one event |
| [cap.ql.m3-engine](#cap-ql-m3-engine) | Replay and read the transactional M3 state: codon form, pose, source and clock | ql kernel m3 <request.json|->; M3State, m3_engine, m3_source, m3_inscription | Receipts and a final snapshot for up to 256 commands against one exact discrete state, with an optional Nara activity reading; the complete source readings of M3 with typed finite projections; and the clock inscription seed read from the map at an ecliptic degree |
| [cap.ql.scene](#cap-ql-scene) | Compose one integrated M1-M2-M3 event from a dated sky and hear it | ql scene compose|binding|world|render; scene::compose; the torus scene field | The dated sky placed on the clock with each planet voiced at its map ratio, the form advanced by M1, the Nara centres the sky reaches, and the same planets played as surface modes of the M1 torus; render writes a WAV with its PCM hash |
| [cap.ql.stage](#cap-ql-stage) | Drive the Ta-Onta procedural stage of the live scene | ql stage state|evaluate|bind|unbind|retire; continuous::stage | One owned exchange with the native field host: a versioned procedure is evaluated or bound over the stage slots (form, material damping, clock inscription, clock lensing), and the host receipt returns with the last acknowledged field on every refusal |
| [cap.ql.nara-identity](#cap-ql-nara-identity) | Read a person's identity profile, natal chart and transit as QL readings | ql nara capabilities|inspect|calculate|transit|personal-current|personal-recompose|presence-consent | The six-office identity reading of an editable profile, optionally with a natal calculation; a dated sky transit; the personal current that joins identity, transit and an M3 activity reading; its recomposition; and a consent-scope reading with no private state exported |
| [cap.ql.nara-dialogue](#cap-ql-nara-dialogue) | Admit a Nara/Epii dialogue turn against the current coordinate and registry | ql nara coordinate|coordinate-content|coordinate-bundle|source-inventory|context|delegate|enrichment|receive | The rooted M world and Bimba binding of a coordinate, its content and a paged source inventory; a dialogue context checked against the actual current registry; a basis-bound delegation from Nara to Epii; and the admission of Epii's enrichment back into the turn |
| [cap.ql.epi-agent](#cap-ql-epi-agent) | Call the Prime-QL faculties #0 to #5 as a bounded domain surface | ql epi-agent constitution|faculty|invoke | The agent constitution, one faculty with its capability field and operation inputs, and an invocation of a named faculty operation (Anuttara read, Vietoris-Rips, Bimba neighbourhood, representation binding, Tarot score, Nara journey and lived context, Logos return) with canonical mutation off |
| [cap.ql.agent-event](#cap-ql-agent-event) | Project an agent event into a QL frame and admit a decision against it | ql agent-event project|frame|harmonic|validate | The QL projection of a supplied event, its decision frame or harmonic readout, and for validate a rebuilt admission of a response that is bypassed, stale or admitted against the current owners |
| [cap.ql.configuration](#cap-ql-configuration) | Disclose QL's configuration contribution to the O:I configuration plane | ql config validate|plan|apply|reset; ql config-contribution | The frozen configuration-contribution document with every setting read-only, validation and plan answers, and a structured unsupported-setting refusal from apply and reset with no receipt minted |
| [cap.ql.vak-composition](#cap-ql-vak-composition) | Compose a C-prime Context-Frame use and publish the Vāk workflow types | ql vak compose; ql vak workflow-types | Immutable whole uses composed and reframed over the Vāk registry with each transition a new use, and the TypeScript types module rendered from the Rust contract that Factory's typed workflows import, with a --check against the checked-in file |
| [cap.ql.techne-reading](#cap-ql-techne-reading) | Read one Wiki subject through the production Technē adapter | ql techne reading <target.json> | The validated Technē reading of a Wiki refraction target and its co-referenced selections, with every reference copied from the target, the provider or the engine, and a bounded whole disclosed only from a validated shape binding |
| [cap.ql.external-frame](#cap-ql-external-frame) | Compare a domain composition with formal context | External Context Frame reading | Exact partial ambiguous or no reading with mapping provenance |
| [cap.ql.vak-source](#cap-ql-vak-source) | Keep research language attributable | ql vak locate | Source-locked entry with coordinate and provenance |
| [cap.ql.vak-context](#cap-ql-vak-context) | Recover relevant formulation relationships | ql vak context with depth zero through two | Bounded source entries and relations |
| [cap.ql.service-negotiation](#cap-ql-service-negotiation) | Know what a provider can actually supply | ql service capabilities/negotiate; QlService | Support deterministic flag and provider state |
| [cap.ql.service-readings](#cap-ql-service-readings) | Locate refract relate or synthesise a subject | QlService ServiceRequest dispatch | Reading preserving subject provider and result provenance |
| [cap.ql.client-modes](#cap-ql-client-modes) | Continue ordinary work when reading is optional | Disabled Optional Required adapter modes | Native subject preserved and availability handled explicitly |
| [cap.ql.wiki-structure](#cap-ql-wiki-structure) | Use ordinary nodes and disclosed partial wholes | Wiki structural contracts and anchor relations | Stable structural representation without completeness gating, including the partial-conjugate-7 constellation grain |
| [cap.ql.wiki-refraction](#cap-ql-wiki-refraction) | Inspect selected wiki relationships formally | WikiFrame refraction | Derived reading with target revision and operator basis |
| [cap.ql.wiki-portals](#cap-ql-wiki-portals) | Traverse explicit cross-wiki relationships | MetaBinding and portal traversal | Cross-wiki traversal preserving identities |
| [cap.ql.living-wiki](#cap-ql-living-wiki) | Orient and expand within a bounded knowledge field | Living-wiki methods and entry aperture | Attributable partial or qualified contextual reading |
| [cap.ql.cli-verification](#cap-ql-cli-verification) | Obtain a bounded local verification result | ql verify | Named deterministic verification checks |
| [cap.ql.kernel-parity](#cap-ql-kernel-parity) | Keep C and Rust relations aligned to one source | Shared holographic contract and parity fixtures | Explicit common relation identities and source provenance |

## cap-ql-cli-discovery

**Need:** Find a usable operation.

**Operation:** ql capabilities and version.

**Outcome:** Versioned command and kernel/service inventory.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/product](ql.html#q1/product). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-cli/src/lib.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-kernel-operators

**Need:** Obtain a reproducible structural result.

**Operation:** ql kernel apply; apply_operator.

**Outcome:** Typed result with input/output and operation version.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-core/src/apply.rs) · [Tests](../../crates/ql-core/tests/operator_invariants.rs).

## cap-ql-pairing

**Need:** Distinguish constructions with coincident vertices.

**Operation:** A/B/C pairing and D1/D2/D3 completion.

**Outcome:** Pair family and derivation retained.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-core/src/pairing.rs) · [Tests](../../crates/ql-core/tests/pairing_grammar.rs).

## cap-ql-shape

**Need:** Work with the actual available structure.

**Operation:** QlShape and versioned shape contracts; ql shape presentation <exact-shape-ref>.

**Outcome:** Valid partials; 4x4, 4x4x4, 6x6, 10x10, seven-fold, relational-sixfold and 18-fold fields, one-fold compression of a disclosed two- or three-fold basis, and a read-only numerical presentation of an exact shape address.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q3/structure](ql.html#q3/structure). [Source](../../fixtures/kernel/ql-shape-contract-v1.json) · [Code](../../crates/ql-core/src/shape.rs) · [Code](../../crates/ql-core/src/shape_presentation.rs) · [Code](../../crates/ql-cli/src/shape_command.rs) · [Tests](../../crates/ql-core/tests/wiki_structural_contract.rs) · [Tests](../../crates/ql-core/tests/geometry_constellation_resolution.rs).

## cap-ql-m3-generation-law

**Need:** Find the codons a hexagram yields under a matrix.

**Operation:** pole::generation::generate; MatrixFamily apply_to_codon and apply_to_hexagram; ql_m3_generate.

**Outcome:** Each of the 192 generation cells with its two pairs, positive and negative codons and non-dual flag; base-partner matrix actions with no gaps; the active-pose seed and pose map as declared laws; the pose bin read the same on every platform.

**Current basis:** implemented; pole_generation_law, pole_seed_and_pose_map and m3_generation_field passed 2026-10-19.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/kernel-rebuild/M3-SOURCE-PARITY.md) · [Code](../../crates/ql-core/src/pole/generation.rs) · [Tests](../../crates/ql-core/tests/pole_generation_law.rs).

## cap-ql-lens-registry

**Need:** Inspect available lenses before reading.

**Operation:** ql mef lenses; MEF registry.

**Outcome:** Twelve lens identities with face and square.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/readings](ql.html#q1/readings). [Source](../../skills/ql-foundations/SKILL.md) · [Code](../../crates/ql-mef/src/registry.rs) · [Tests](../../crates/ql-mef/tests/identity_and_provenance.rs).

## cap-ql-context-frames

**Need:** Select a defined cut of a relation.

**Operation:** ql context-frame list; ContextFrameId.

**Outcome:** Named CF1-CF7 grammar and selected/unselected partitions.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/readings](ql.html#q1/readings). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-mef/src/context_frame.rs) · [Tests](../../crates/ql-mef/tests/context_frame_promotion.rs).

## cap-ql-team-space

**Need:** Choose who answers a question inside Anima or Aletheia.

**Operation:** ql context-frame team-space; team_space::TeamSpace.

**Outcome:** The S4′/S5′ team grid: CF1–CF7 members by Day and Night′ questions, with depth M0–M5, the Vāk path and the decision instruments; gate 6 stays the person’s.

**Current basis:** implemented; team_space tests and frontdoor_help passed 2026-10-19.

Account: [q1/readings](ql.html#q1/readings). [Source](../../docs/integrations/epi-logos/ANIMA-ALETHEIA-TEAMS.md) · [Code](../../crates/ql-mef/src/team_space.rs) · [Tests](../../crates/ql-mef/src/team_space/tests.rs).

## cap-ql-m-registry-ledger

**Need:** Know which coordinates of the M tree exist, what is bound to them and what is proven.

**Operation:** ql kernel coverage|ledger|validate-ledger; MRegistry and MLedger.

**Outcome:** The native M registry and the shared ledger: per-coordinate views, validation findings that are not completion, and coverage for one coordinate by stratum, axis and required readiness.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/KERNEL-RECURSIVE-M-REGISTRY.md) · [Source](../../docs/KERNEL-M-LEDGER-MATRIX-LOCK.md) · [Code](../../crates/ql-mef/src/m_tree.rs) · [Code](../../crates/ql-mef/src/m_ledger.rs) · [Code](../../crates/ql-cli/src/m_ledger.rs) · [Tests](../../crates/ql-mef/tests/native_m_tree.rs) · [Tests](../../crates/ql-mef/tests/m_ledger.rs) · [Tests](../../crates/ql-cli/tests/m_ledger_cli.rs).

## cap-ql-m1-engine

**Need:** Read the M1 ring, torus and harmonic state for one event and cycle.

**Operation:** ql kernel m1 <request.json>; m1_engine::engine_json and M1Engine.

**Outcome:** A versioned M1 state: the 12-fold clock and cell, source-literal and accepted-arithmetic readings kept apart, torus position, ratio evidence and the selected coordinate reading.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/KERNEL-M1-ENGINE-CONTRACT.md) · [Code](../../crates/ql-mef/src/m1.rs) · [Code](../../crates/ql-mef/src/m1_engine.rs) · [Tests](../../crates/ql-mef/tests/m1_engine.rs) · [Tests](../../crates/ql-cli/tests/m1_engine_cli.rs).

## cap-ql-m2-engine

**Need:** Read the Paraśakti field: descriptors, music, colour and material for an event.

**Operation:** m2 descriptor field and finite operations; m2_engine; m2_condition; m2_vimarsha; m2_sky.

**Outcome:** Retained descriptor tables and finite operations over one 72-reading carrier, the M2-1 Vimarśā reading from supplied M1 ratio and M3 pose, the sky of planets and chakras with the map values, and a joint music/colour/material condition for one event.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/kernel-rebuild/m2-engine-v1.md) · [Code](../../crates/ql-mef/src/m2.rs) · [Code](../../crates/ql-mef/src/m2_engine.rs) · [Code](../../crates/ql-mef/src/m2_condition.rs) · [Code](../../crates/ql-mef/src/m2_vimarsha.rs) · [Code](../../crates/ql-mef/src/m2_sky.rs) · [Tests](../../crates/ql-mef/tests/m2_engine.rs) · [Tests](../../crates/ql-mef/tests/m2_condition.rs) · [Tests](../../crates/ql-mef/tests/m2_vimarsha.rs) · [Tests](../../crates/ql-mef/tests/m2_planet_chakra.rs).

## cap-ql-m3-engine

**Need:** Replay and read the transactional M3 state: codon form, pose, source and clock.

**Operation:** ql kernel m3 <request.json|->; M3State, m3_engine, m3_source, m3_inscription.

**Outcome:** Receipts and a final snapshot for up to 256 commands against one exact discrete state, with an optional Nara activity reading; the complete source readings of M3 with typed finite projections; and the clock inscription seed read from the map at an ecliptic degree.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/KERNEL-M3-ENGINE-CONTRACT.md) · [Code](../../crates/ql-cli/src/m3_command.rs) · [Code](../../crates/ql-mef/src/m3_state.rs) · [Code](../../crates/ql-mef/src/m3_engine.rs) · [Code](../../crates/ql-mef/src/m3_source.rs) · [Code](../../crates/ql-mef/src/m3_inscription.rs) · [Tests](../../crates/ql-cli/tests/m3_native.rs) · [Tests](../../crates/ql-mef/tests/m3_state.rs) · [Tests](../../crates/ql-mef/tests/m3_engine.rs) · [Tests](../../crates/ql-mef/tests/m3_inscription.rs).

## cap-ql-scene

**Need:** Compose one integrated M1-M2-M3 event from a dated sky and hear it.

**Operation:** ql scene compose|binding|world|render; scene::compose; the torus scene field.

**Outcome:** The dated sky placed on the clock with each planet voiced at its map ratio, the form advanced by M1, the Nara centres the sky reaches, and the same planets played as surface modes of the M1 torus; render writes a WAV with its PCM hash.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q4/deployment](ql.html#q4/deployment). [Source](../../docs/kernel-rebuild/K8-COUPLED-EVENT-CONTRACT.md) · [Code](../../crates/ql-cli/src/scene_command.rs) · [Code](../../crates/ql-mef/src/scene.rs) · [Code](../../crates/ql-mef/src/scene_sky.rs) · [Code](../../crates/ql-mef/src/continuous/scene_field.rs) · [Tests](../../crates/ql-cli/tests/scene_native.rs) · [Tests](../../crates/ql-cli/tests/scene_render_native.rs) · [Tests](../../crates/ql-mef/tests/scene_instrument.rs) · [Tests](../../crates/ql-mef/tests/scene_sky.rs).

## cap-ql-stage

**Need:** Drive the Ta-Onta procedural stage of the live scene.

**Operation:** ql stage state|evaluate|bind|unbind|retire; continuous::stage.

**Outcome:** One owned exchange with the native field host: a versioned procedure is evaluated or bound over the stage slots (form, material damping, clock inscription, clock lensing), and the host receipt returns with the last acknowledged field on every refusal.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q4/deployment](ql.html#q4/deployment). [Source](../../docs/kernel-rebuild/K8-COUPLED-EVENT-CONTRACT.md) · [Code](../../crates/ql-cli/src/stage_command.rs) · [Code](../../crates/ql-mef/src/continuous/stage.rs) · [Tests](../../crates/ql-cli/tests/stage_native.rs) · [Tests](../../crates/ql-mef/tests/ta_onta_stage.rs).

## cap-ql-nara-identity

**Need:** Read a person's identity profile, natal chart and transit as QL readings.

**Operation:** ql nara capabilities|inspect|calculate|transit|personal-current|personal-recompose|presence-consent.

**Outcome:** The six-office identity reading of an editable profile, optionally with a natal calculation; a dated sky transit; the personal current that joins identity, transit and an M3 activity reading; its recomposition; and a consent-scope reading with no private state exported.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q4/deployment](ql.html#q4/deployment). [Source](../../docs/NARA-IDENTITY-COMPOSITION-DECISIONS.md) · [Source](../../docs/kernel-rebuild/K10-NARA-M4-CONTRACT.md) · [Code](../../crates/ql-cli/src/nara_command.rs) · [Code](../../crates/ql-cli/src/nara_presence_command.rs) · [Code](../../crates/ql-mef/src/nara.rs) · [Code](../../crates/ql-mef/src/nara/intake.rs) · [Code](../../crates/ql-mef/src/nara/current.rs) · [Code](../../crates/ql-mef/src/nara/expression.rs) · [Tests](../../crates/ql-cli/tests/nara_current_native.rs) · [Tests](../../crates/ql-mef/tests/nara_personal_field.rs).

## cap-ql-nara-dialogue

**Need:** Admit a Nara/Epii dialogue turn against the current coordinate and registry.

**Operation:** ql nara coordinate|coordinate-content|coordinate-bundle|source-inventory|context|delegate|enrichment|receive.

**Outcome:** The rooted M world and Bimba binding of a coordinate, its content and a paged source inventory; a dialogue context checked against the actual current registry; a basis-bound delegation from Nara to Epii; and the admission of Epii's enrichment back into the turn.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q4/deployment](ql.html#q4/deployment). [Source](../../docs/kernel-rebuild/K10-NARA-M4-CONTRACT.md) · [Code](../../crates/ql-cli/src/nara_dialogue_command.rs) · [Code](../../crates/ql-mef/src/nara/dialogue.rs) · [Tests](../../crates/ql-cli/tests/nara_dialogue_native.rs) · [Tests](../../crates/ql-mef/tests/nara_dialogue_conformance.rs).

## cap-ql-epi-agent

**Need:** Call the Prime-QL faculties #0 to #5 as a bounded domain surface.

**Operation:** ql epi-agent constitution|faculty|invoke.

**Outcome:** The agent constitution, one faculty with its capability field and operation inputs, and an invocation of a named faculty operation (Anuttara read, Vietoris-Rips, Bimba neighbourhood, representation binding, Tarot score, Nara journey and lived context, Logos return) with canonical mutation off.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/readings](ql.html#q1/readings). [Source](../../docs/integrations/epi-logos/EPI-LOGOS-AGENT-ARCHITECTURE.md) · [Code](../../crates/ql-cli/src/epi_agent_command.rs) · [Code](../../crates/ql-mef/src/epi_agent.rs) · [Tests](../../crates/ql-cli/tests/agent_research_native.rs) · [Tests](../../crates/ql-cli/tests/pi_agent_native.rs).

## cap-ql-agent-event

**Need:** Project an agent event into a QL frame and admit a decision against it.

**Operation:** ql agent-event project|frame|harmonic|validate.

**Outcome:** The QL projection of a supplied event, its decision frame or harmonic readout, and for validate a rebuilt admission of a response that is bypassed, stale or admitted against the current owners.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/readings](ql.html#q1/readings). [Source](../../docs/integrations/epi-logos/QL-AGENT-DECISION-CONTRACTS.md) · [Code](../../crates/ql-cli/src/agent_event_command.rs) · [Code](../../crates/ql-mef/src/agent_event.rs) · [Code](../../crates/ql-mef/src/agent_event/admission.rs) · [Tests](../../crates/ql-cli/tests/agent_event_native.rs) · [Tests](../../crates/ql-mef/tests/agent_event.rs).

## cap-ql-configuration

**Need:** Disclose QL's configuration contribution to the O:I configuration plane.

**Operation:** ql config validate|plan|apply|reset; ql config-contribution.

**Outcome:** The frozen configuration-contribution document with every setting read-only, validation and plan answers, and a structured unsupported-setting refusal from apply and reset with no receipt minted.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q4/deployment](ql.html#q4/deployment). [Source](../../crates/ql-cli/src/configuration.rs) · [Code](../../crates/ql-cli/src/configuration.rs) · [Tests](../../crates/ql-cli/tests/configuration_plane.rs).

## cap-ql-vak-composition

**Need:** Compose a C-prime Context-Frame use and publish the Vāk workflow types.

**Operation:** ql vak compose; ql vak workflow-types.

**Outcome:** Immutable whole uses composed and reframed over the Vāk registry with each transition a new use, and the TypeScript types module rendered from the Rust contract that Factory's typed workflows import, with a --check against the checked-in file.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/readings](ql.html#q1/readings). [Source](../../docs/QL-VAK-KERNEL-RECONCILIATION.md) · [Code](../../crates/ql-cli/src/vak_composition.rs) · [Code](../../crates/ql-mef/src/vak_composition.rs) · [Code](../../crates/ql-mef/src/vak_workflow_types.rs) · [Tests](../../crates/ql-cli/tests/vak_composition.rs) · [Tests](../../crates/ql-mef/tests/vak_composition.rs) · [Tests](../../crates/ql-mef/tests/vak_workflow_types.rs).

## cap-ql-techne-reading

**Need:** Read one Wiki subject through the production Technē adapter.

**Operation:** ql techne reading <target.json>.

**Outcome:** The validated Technē reading of a Wiki refraction target and its co-referenced selections, with every reference copied from the target, the provider or the engine, and a bounded whole disclosed only from a validated shape binding.

**Current basis:** implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19.

Account: [q1/readings](ql.html#q1/readings). [Source](../../docs/L5-TECHNE-INSTRUMENT-WAYFINDER.md) · [Code](../../crates/ql-cli/src/techne_command.rs) · [Code](../../crates/ql-adapters/src/techne_wiki.rs) · [Tests](../../crates/ql-cli/tests/techne_reading.rs) · [Tests](../../crates/ql-adapters/tests/techne_contract.rs).

## cap-ql-external-frame

**Need:** Compare a domain composition with formal context.

**Operation:** External Context Frame reading.

**Outcome:** Exact partial ambiguous or no reading with mapping provenance.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q3/identity](ql.html#q3/identity). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/66.json) · [Code](../../crates/ql-mef/src/context_frame_target.rs) · [Tests](../../crates/ql-mef/tests/context_frame_external_reading.rs).

## cap-ql-vak-source

**Need:** Keep research language attributable.

**Operation:** ql vak locate.

**Outcome:** Source-locked entry with coordinate and provenance.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/readings](ql.html#q1/readings). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-mef/src/vak.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-vak-context

**Need:** Recover relevant formulation relationships.

**Operation:** ql vak context with depth zero through two.

**Outcome:** Bounded source entries and relations.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q4/deployment](ql.html#q4/deployment). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-cli/src/lib.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-service-negotiation

**Need:** Know what a provider can actually supply.

**Operation:** ql service capabilities/negotiate; QlService.

**Outcome:** Support deterministic flag and provider state.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q1/services](ql.html#q1/services). [Source](../../skills/ql-operation/SKILL.md) · [Code](../../crates/ql-service/src/negotiation.rs) · [Tests](../../crates/ql-service/tests/provider_service.rs).

## cap-ql-service-readings

**Need:** Locate refract relate or synthesise a subject.

**Operation:** QlService ServiceRequest dispatch.

**Outcome:** Reading preserving subject provider and result provenance.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/refract](ql.html#q2/refract). [Source](../../skills/ql-operation/SKILL.md) · [Code](../../crates/ql-service/src/service.rs) · [Tests](../../crates/ql-service/tests/provider_service.rs).

## cap-ql-client-modes

**Need:** Continue ordinary work when reading is optional.

**Operation:** Disabled Optional Required adapter modes.

**Outcome:** Native subject preserved and availability handled explicitly.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q1/services](ql.html#q1/services). [Source](../../skills/ql-operation/SKILL.md) · [Code](../../crates/ql-adapters/src/core.rs) · [Tests](../../crates/ql-adapters/tests/noql_matrix.rs).

## cap-ql-wiki-structure

**Need:** Use ordinary nodes and disclosed partial wholes.

**Operation:** Wiki structural contracts and anchor relations.

**Outcome:** Stable structural representation without completeness gating, including the partial-conjugate-7 constellation grain.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q3/structure](ql.html#q3/structure). [Source](../../docs/wiki-structural-contract-v2.md) · [Code](../../crates/ql-core/src/structural.rs) · [Tests](../../crates/ql-core/tests/wiki_structural_contract.rs).

## cap-ql-wiki-refraction

**Need:** Inspect selected wiki relationships formally.

**Operation:** WikiFrame refraction.

**Outcome:** Derived reading with target revision and operator basis.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/wiki-encounter](ql.html#q2/wiki-encounter). [Source](../../docs/wiki-structural-contract-v2.md) · [Code](../../crates/ql-wiki/src/refraction.rs) · [Tests](../../crates/ql-wiki/tests/wiki_refraction.rs).

## cap-ql-wiki-portals

**Need:** Traverse explicit cross-wiki relationships.

**Operation:** MetaBinding and portal traversal.

**Outcome:** Cross-wiki traversal preserving identities.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/wiki-encounter](ql.html#q2/wiki-encounter). [Source](../../docs/wiki-structural-contract-v2.md) · [Code](../../crates/ql-wiki/src/portal.rs) · [Tests](../../crates/ql-wiki/tests/meta_portal.rs).

## cap-ql-living-wiki

**Need:** Orient and expand within a bounded knowledge field.

**Operation:** Living-wiki methods and entry aperture.

**Outcome:** Attributable partial or qualified contextual reading.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/wiki-encounter](ql.html#q2/wiki-encounter). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/79.json) · [Code](../../crates/ql-wiki/src/living_methods.rs) · [Tests](../../crates/ql-wiki/src/living_methods.rs).

## cap-ql-cli-verification

**Need:** Obtain a bounded local verification result.

**Operation:** ql verify.

**Outcome:** Named deterministic verification checks.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q5/verification](ql.html#q5/verification). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-cli/src/lib.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-kernel-parity

**Need:** Keep C and Rust relations aligned to one source.

**Operation:** Shared holographic contract and parity fixtures.

**Outcome:** Explicit common relation identities and source provenance.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q0/provenance](ql.html#q0/provenance). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-core/src/kernel.rs) · [Tests](../../crates/ql-mef/tests/holographic_kernel_contract.rs).

## Suite directed product field

The `suite-relations` view uses the same CSV contract as `product-field`. Its H/A identifiers retain the human-facing and agent-facing orientations of the six products. Source-defined relation readings and annotations remain attached to each determination in `extensions`; coverage is independent of implementation status. The manifest declares the selected scope and axis meaning.

The `suite-relations` view uses the same CSV contract as `product-field`. Its H/A identifiers retain the human-facing and agent-facing orientations of the six products. Source-defined relation readings and annotations remain attached to each determination in `extensions`; coverage is independent of implementation status. The manifest declares the selected scope and axis meaning.

## Verification scope

Executed: `cargo test -p ql-cli -p ql-core -p ql-mef --all-targets`, all selected tests passed on 6 September 2026. Service/adapter/wiki tests are source traces outside that executed selection. This observation covers native structural behaviour and selected CLI contracts, not model performance or philosophical validation. The source HTML and CSV row identities still require the planned AIKit format adapter for native semantic extraction.

## Lossless CSV appendix

```csv
id,record_type,view_id,row_id,column_id,capability_refs,need,operation,outcome,implementation_status,standing,source_refs,code_refs,test_refs,account_ref,relation,coverage,extensions,question
cap.ql.cli-discovery,capability,,,,[],Find a usable operation,ql capabilities and version,Versioned command and kernel/service inventory,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-cli/src/lib.rs,crates/ql-cli/src/lib.rs,ql.html#q1/product,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""kernel.capabilities"", ""kernel.m3"", ""kernel.coverage"", ""kernel.ledger"", ""kernel.validate-ledger"", ""matheme.derive"", ""matheme.shadow"", ""kernel.apply"", ""mef.lenses"", ""context-frame.list"", ""context-frame.team-space"", ""vak.capabilities"", ""vak.locate"", ""vak.context"", ""vak.compose"", ""vak.workflow-types"", ""techne.reading"", ""shape.presentation"", ""epi-agent.constitution"", ""epi-agent.faculty"", ""epi-agent.invoke"", ""agent-event.project"", ""agent-event.frame"", ""agent-event.harmonic"", ""agent-event.validate"", ""nara.capabilities"", ""nara.inspect"", ""nara.calculate"", ""nara.coordinate"", ""nara.context"", ""nara.delegate"", ""nara.enrichment"", ""nara.receive"", ""nara.transit"", ""nara.personal-current"", ""nara.personal-recompose"", ""nara.presence-consent"", ""service.capabilities"", ""service.negotiate"", ""stage.state"", ""stage.evaluate"", ""stage.bind"", ""stage.unbind"", ""stage.retire"", ""config.contribute"", ""config.validate"", ""config.plan"", ""config.apply"", ""config.reset"", ""verify""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The native capabilities response advertises this complete emitted command catalog.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:agent/ql-multi-matrix-profile"", ""EpiLogos/O-I#65/#220 architecture-documentation-reconciliation 2026-09-30; codex:01a0f43c-41e2-7990-9cf0-1cc9de2b368d"", ""O-I#65/#220:architecture-doc-diagrams-20260930:native-reverse-navigation-and-praxis"", ""O:I#65/#220 exact documentation units; architecture-doc-diagrams-20260930"", ""O:I#65/#220 fresh formal navigation: remove unrelated Expression described-by relation"", ""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/lib.rs"": ""a64bd450173bc40178d97662fdd07bed0699eb650f1747aa1d607525b48653af""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00"", ""documentation"": {""architecture_refs"": [""../../docs/ARCHITECTURE-NAVIGATION.md#governing-source-and-successors"", ""../../docs/KERNEL-REBUILD-WAYFINDER.md#ql-mef-kernel-rebuild-wayfinder"", ""../../docs/kernel-rebuild/M123-SCENE-STRUCTURAL-MAP.md#m1m2m3-live-expression-scene-gate-0-structural-map"", ""../../docs/kernel-rebuild/LIVING-INSTRUMENT-ARCHITECTURE.md#epi-living-instrument--planetary-world-nara-and-enacted-vāk""], ""praxis_refs"": [""skill/central/architecture-authoring"", ""skill/central/diagram-authoring"", ""skill/central/documentation-reconciliation""], ""relations"": [{""relation"": ""described-by"", ""target"": ""../../docs/ARCHITECTURE-NAVIGATION.md#governing-source-and-successors""}]}}",
cap.ql.kernel-operators,capability,,,,[],Obtain a reproducible structural result,ql kernel apply; apply_operator,Typed result with input/output and operation version,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-core/src/apply.rs,crates/ql-core/tests/operator_invariants.rs,ql.html#q1/kernel,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""kernel.capabilities"", ""kernel.apply""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI reports kernel operations and applies advertised deterministic address operators.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/apply.rs"": ""20db087667368b6e80541792105408b72f595b6a9611557628b3b2f2a4380bd1""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.pairing,capability,,,,[],Distinguish constructions with coincident vertices,A/B/C pairing and D1/D2/D3 completion,Pair family and derivation retained,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-core/src/pairing.rs,crates/ql-core/tests/pairing_grammar.rs,ql.html#q1/kernel,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Pair-family construction is implemented through the ql-core library; the CLI does not emit a pairing command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/pairing.rs"": ""480e0ec36ed4a5fe6cbe14d3a91af71cf32772a3ef7c6dd30257af47be975df9""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.shape,capability,,,,[],Work with the actual available structure,QlShape and versioned shape contracts; ql shape presentation <exact-shape-ref>,"Valid partials; 4x4, 4x4x4, 6x6, 10x10, seven-fold, relational-sixfold and 18-fold fields, one-fold compression of a disclosed two- or three-fold basis, and a read-only numerical presentation of an exact shape address",implemented; selected native tests passed 2026-09-06,agent-inference,fixtures/kernel/ql-shape-contract-v1.json,crates/ql-core/src/shape.rs;crates/ql-core/src/shape_presentation.rs;crates/ql-cli/src/shape_command.rs,crates/ql-core/tests/wiki_structural_contract.rs;crates/ql-core/tests/geometry_constellation_resolution.rs,ql.html#q3/structure,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""shape.presentation""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI discloses the numerical presentation of an exact native shape reference; the shape contracts themselves are library facilities.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-core/src/shape.rs"": ""6d890682ab7a60a794e4018e5ef59ca8e62c6075bdfc177ddc068cdae8eee29b"", ""crates/ql-core/src/shape_presentation.rs"": ""163ab18606b19fd55e9e1f398463cbc96c8e5b8844e8ec67797aa7b81edcc18a"", ""crates/ql-cli/src/shape_command.rs"": ""d214a0d0bf76d6e454733a29deb4455cf41e2c927d372c127e6f283830f16241""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.m3-generation-law,capability,,,,[],Find the codons a hexagram yields under a matrix,pole::generation::generate; MatrixFamily apply_to_codon and apply_to_hexagram; ql_m3_generate,"Each of the 192 generation cells with its two pairs, positive and negative codons and non-dual flag; base-partner matrix actions with no gaps; the active-pose seed and pose map as declared laws; the pose bin read the same on every platform","implemented; pole_generation_law, pole_seed_and_pose_map and m3_generation_field passed 2026-10-19",agent-inference,docs/kernel-rebuild/M3-SOURCE-PARITY.md,crates/ql-core/src/pole/generation.rs;crates/ql-core/src/pole/fold.rs;crates/ql-core/src/pole/quaternion.rs;crates/ql-core/src/pole/phase.rs;crates/ql-core/src/pole/codon.rs;crates/ql-core/src/pole/iching.rs;crates/ql-core/src/pole/nucleotide.rs;crates/ql-core/src/pole/mod.rs,crates/ql-core/tests/pole_generation_law.rs;crates/ql-core/tests/pole_seed_and_pose_map.rs;crates/ql-mef/tests/m3_generation_field.rs,ql.html#q1/kernel,,,"{""basis"": ""Generation law read from the map M3-3-2 layer (fixtures/kernel/m3-generation-field-v1.json) and implemented in the pole kernel with C parity."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Rust and C library operations read by the M3 engine and scene; no CLI command reads the generation law on its own.""}, ""maintenance"": {""change_refs"": [""QL-MEF:m3-generation-law-20261019"", ""claude-code:ecf095a4-6301-4173-a48e-e6311322b99d"", ""QL-MEF:m3-generation-law-20261019; claude-code:ecf095a4-6301-4173-a48e-e6311322b99d"", ""QL-MEF:m3-generation-law-20261019 exact pose bin; claude-code:ecf095a4-6301-4173-a48e-e6311322b99d""], ""code_basis"": {""crates/ql-core/src/pole/generation.rs"": ""8c6bbd060f5d0cada96b74a02cc3a2cef09383d96bcbe2aa018b75d10474c84e"", ""crates/ql-core/src/pole/fold.rs"": ""085f1d57bbcc6068305cbbe27593b421ec86b94c31534d1338757cad474f22fc"", ""crates/ql-core/src/pole/quaternion.rs"": ""9ded61a800dffc9a5997da8b79b320e719f5661b58425581d43f278b5935db5d"", ""crates/ql-core/src/pole/phase.rs"": ""d431d5c47336684869305713bd6eabf16799115f2de5d9a7d7b31f4ed075f78c"", ""crates/ql-core/src/pole/codon.rs"": ""707cb39b2efd4efd408eb8eb11dd17337eb3ea36efaa4be34cfb7739712dbfe0"", ""crates/ql-core/src/pole/iching.rs"": ""38431418979b2015e608fe9ccaf384946a30e09ece288324fa4e77ad34d4c316"", ""crates/ql-core/src/pole/nucleotide.rs"": ""03c773ab44cc237f7a76ad81ec6c6353bb05de521e163d1134b274d34e3a323f"", ""crates/ql-core/src/pole/mod.rs"": ""5860aa8283cd98f46e37e57651e57cb2ed91a8d46d8853607b5e6100efdf0b94""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T17:03:41.778351+00:00""}",
cap.ql.lens-registry,capability,,,,[],Inspect available lenses before reading,ql mef lenses; MEF registry,Twelve lens identities with face and square,implemented; selected native tests passed 2026-09-06,agent-inference,skills/ql-foundations/SKILL.md,crates/ql-mef/src/registry.rs,crates/ql-mef/tests/identity_and_provenance.rs,ql.html#q1/readings,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""mef.lenses""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits the MEF lens registry.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-mef/src/registry.rs"": ""646385949853bd05f2b9daac49539f07147f1ccf65d591290d4e3533434ceef7""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.context-frames,capability,,,,[],Select a defined cut of a relation,ql context-frame list; ContextFrameId,Named CF1-CF7 grammar and selected/unselected partitions,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-mef/src/context_frame.rs,crates/ql-mef/tests/context_frame_promotion.rs,ql.html#q1/readings,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""context-frame.list""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits the named Context Frame registry.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""EpiLogos/O-I#65/#220 architecture-documentation-reconciliation 2026-09-30; codex:01a0f43c-41e2-7990-9cf0-1cc9de2b368d"", ""O-I#65/#220:architecture-doc-diagrams-20260930:native-reverse-navigation-and-praxis"", ""O:I#65/#220 exact documentation units; architecture-doc-diagrams-20260930"", ""O:I#65/#220 fresh formal navigation: remove unrelated Expression described-by relation""], ""code_basis"": {""crates/ql-mef/src/context_frame.rs"": ""cd4b2fef86a3f8b604b0048608d5946543ac2ce3cb6786c5272a744a6cc1b15c""}, ""updated_at"": ""2026-10-01""}, ""last_reconciled_at"": ""2026-10-01T00:39:33.969858+00:00"", ""documentation"": {""architecture_refs"": [""../../docs/ARCHITECTURE-NAVIGATION.md#governing-source-and-successors"", ""../../docs/KERNEL-REBUILD-WAYFINDER.md#ql-mef-kernel-rebuild-wayfinder"", ""../../docs/kernel-rebuild/M123-SCENE-STRUCTURAL-MAP.md#m1m2m3-live-expression-scene-gate-0-structural-map"", ""../../docs/kernel-rebuild/LIVING-INSTRUMENT-ARCHITECTURE.md#epi-living-instrument--planetary-world-nara-and-enacted-vāk""], ""praxis_refs"": [""skill/central/architecture-authoring"", ""skill/central/diagram-authoring"", ""skill/central/documentation-reconciliation""], ""relations"": [{""relation"": ""described-by"", ""target"": ""../../docs/ARCHITECTURE-NAVIGATION.md#governing-source-and-successors""}]}}",
cap.ql.team-space,capability,,,,[],Choose who answers a question inside Anima or Aletheia,ql context-frame team-space; team_space::TeamSpace,"The S4′/S5′ team grid: CF1–CF7 members by Day and Night′ questions, with depth M0–M5, the Vāk path and the decision instruments; gate 6 stays the person’s",implemented; team_space tests and frontdoor_help passed 2026-10-19,agent-inference,docs/integrations/epi-logos/ANIMA-ALETHEIA-TEAMS.md,crates/ql-mef/src/team_space.rs;crates/ql-mef/src/team_space/tests.rs,crates/ql-mef/src/team_space/tests.rs;crates/ql-cli/tests/frontdoor_help.rs,ql.html#q1/readings,,,"{""basis"": ""Native team space within S4′ Anima and S5′ Aletheia, derived from the owners named in ANIMA-ALETHEIA-TEAMS.md §10."", ""cli_commands"": [""context-frame.team-space""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits the whole team space, as text or JSON.""}, ""maintenance"": {""change_refs"": [""QL-MEF:vak-team-space-20261019"", ""claude-code:ecf095a4-6301-4173-a48e-e6311322b99d"", ""QL-MEF:vak-team-space-20261019; claude-code:ecf095a4-6301-4173-a48e-e6311322b99d"", ""QL-MEF:vak-team-space-20261019 merge of #332; claude-code:ecf095a4-6301-4173-a48e-e6311322b99d""], ""code_basis"": {""crates/ql-mef/src/team_space.rs"": ""069120b37216b12bdb063ec1a5e1d8ef9120da1d732f82dec87e6a164a66ab39"", ""crates/ql-mef/src/team_space/tests.rs"": ""7f61ef209c74e1370cb933d7c1c43a15e4fba067afd1eb2e9bb52abfb4bc42c8""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T17:50:01.520402+00:00""}",
cap.ql.external-frame,capability,,,,[],Compare a domain composition with formal context,External Context Frame reading,Exact partial ambiguous or no reading with mapping provenance,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/66.json,crates/ql-mef/src/context_frame_target.rs,crates/ql-mef/tests/context_frame_external_reading.rs,ql.html#q3/identity,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""External Context Frame readings are library contracts; the CLI does not emit an external-frame command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-mef/src/context_frame_target.rs"": ""0394a6f9604024ce2d92908d6950c289b454d771e5a4b26536d2ce6754fb3b9c""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.vak-source,capability,,,,[],Keep research language attributable,ql vak locate,Source-locked entry with coordinate and provenance,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-mef/src/vak.rs,crates/ql-cli/src/lib.rs,ql.html#q1/readings,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""vak.capabilities"", ""vak.locate""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI reports and locates source-locked Vāk entries.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-mef/src/vak.rs"": ""6345a97753399a325c0a3b7f821ebf67b552888445fb0d13410b6177f0d63de7""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.vak-context,capability,,,,[],Recover relevant formulation relationships,ql vak context with depth zero through two,Bounded source entries and relations,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-cli/src/lib.rs,crates/ql-cli/src/lib.rs,ql.html#q4/deployment,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""vak.context""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits bounded source-locked Vāk context readings.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:agent/ql-multi-matrix-profile"", ""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/lib.rs"": ""a64bd450173bc40178d97662fdd07bed0699eb650f1747aa1d607525b48653af""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.service-negotiation,capability,,,,[],Know what a provider can actually supply,ql service capabilities/negotiate; QlService,Support deterministic flag and provider state,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,skills/ql-operation/SKILL.md,crates/ql-service/src/negotiation.rs,crates/ql-service/tests/provider_service.rs,ql.html#q1/services,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""service.capabilities"", ""service.negotiate""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI reports provider state and negotiates the advertised service operations.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-service/src/negotiation.rs"": ""a06f9a97dbe4bafcca41575ea2909acea9770554f35f4581b2515badf6edf79e""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.service-readings,capability,,,,[],Locate refract relate or synthesise a subject,QlService ServiceRequest dispatch,Reading preserving subject provider and result provenance,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,skills/ql-operation/SKILL.md,crates/ql-service/src/service.rs,crates/ql-service/tests/provider_service.rs,ql.html#q2/refract,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""gap"", ""reason"": ""The service contract exists, but this CLI only negotiates availability and the observed provider is absent; it does not execute readings.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-service/src/service.rs"": ""c51aed38de62fd2152308c69ac02b55c3dfbb8dc8216e0dfeb2bbaf6be695884""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.client-modes,capability,,,,[],Continue ordinary work when reading is optional,Disabled Optional Required adapter modes,Native subject preserved and availability handled explicitly,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,skills/ql-operation/SKILL.md,crates/ql-adapters/src/core.rs,crates/ql-adapters/tests/noql_matrix.rs,ql.html#q1/services,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Disabled, optional, and required adapter modes are ql-adapters library contracts.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-adapters/src/core.rs"": ""95996a65e0aab39f6990c45889b271b912239f123c9078fbda5c407aaa978ad0""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.wiki-structure,capability,,,,[],Use ordinary nodes and disclosed partial wholes,Wiki structural contracts and anchor relations,"Stable structural representation without completeness gating, including the partial-conjugate-7 constellation grain",implemented; selected native tests passed 2026-09-06,agent-inference,docs/wiki-structural-contract-v2.md,crates/ql-core/src/structural.rs,crates/ql-core/tests/wiki_structural_contract.rs,ql.html#q3/structure,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Wiki structural contracts are exposed by ql-core libraries, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-core/src/structural.rs"": ""f3a952e28bf9b640dbf812f982b054c19e2a4f66cb9c0c2fa5ca45547b1bc36b""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.wiki-refraction,capability,,,,[],Inspect selected wiki relationships formally,WikiFrame refraction,Derived reading with target revision and operator basis,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,docs/wiki-structural-contract-v2.md,crates/ql-wiki/src/refraction.rs,crates/ql-wiki/tests/wiki_refraction.rs,ql.html#q2/wiki-encounter,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""WikiFrame refraction is a ql-wiki library contract, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-wiki/src/refraction.rs"": ""52cf52cd79f77bb7a31f36b1a547103ea9361653aefc60c7dbac2a1319d8729a""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.wiki-portals,capability,,,,[],Traverse explicit cross-wiki relationships,MetaBinding and portal traversal,Cross-wiki traversal preserving identities,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,docs/wiki-structural-contract-v2.md,crates/ql-wiki/src/portal.rs,crates/ql-wiki/tests/meta_portal.rs,ql.html#q2/wiki-encounter,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Portal traversal is a ql-wiki library contract, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-wiki/src/portal.rs"": ""919ce316d0f35420202426ffbb76de04b72dfd0668845fcd16bb2be420b07908""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.living-wiki,capability,,,,[],Orient and expand within a bounded knowledge field,Living-wiki methods and entry aperture,Attributable partial or qualified contextual reading,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/79.json,crates/ql-wiki/src/living_methods.rs,crates/ql-wiki/src/living_methods.rs,ql.html#q2/wiki-encounter,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Living-wiki methods are ql-wiki library contracts, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-wiki/src/living_methods.rs"": ""bf594f7d398b5cac883c14f0739c35eb4d838f41db25571326ef6ef33c70a9e3""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.cli-verification,capability,,,,[],Obtain a bounded local verification result,ql verify,Named deterministic verification checks,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-cli/src/lib.rs,crates/ql-cli/src/lib.rs,ql.html#q5/verification,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""verify""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits its bounded native verification result.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:agent/ql-multi-matrix-profile"", ""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/lib.rs"": ""a64bd450173bc40178d97662fdd07bed0699eb650f1747aa1d607525b48653af""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.kernel-parity,capability,,,,[],Keep C and Rust relations aligned to one source,Shared holographic contract and parity fixtures,Explicit common relation identities and source provenance,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-core/src/kernel.rs,crates/ql-mef/tests/holographic_kernel_contract.rs,ql.html#q0/provenance,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Kernel parity is maintained through ql-core and fixture contracts, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/kernel.rs"": ""d615307e8408749847707e60fae2aeb70d216ee710e0d0fb9a9dfe2861081156""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H0->H5,relation,suite-relations,H0,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""B3|C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H0->A5,relation,suite-relations,H0,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""D2-require|D2-complete"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H1->H5,relation,suite-relations,H1,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H1->A5,relation,suite-relations,H1,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H2->H5,relation,suite-relations,H2,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H2->A5,relation,suite-relations,H2,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H3->H5,relation,suite-relations,H3,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H3->A5,relation,suite-relations,H3,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H4->H5,relation,suite-relations,H4,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H4->A5,relation,suite-relations,H4,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""D2-transform"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H0,relation,suite-relations,H5,H0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""B3|C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H1,relation,suite-relations,H5,H1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H2,relation,suite-relations,H5,H2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H3,relation,suite-relations,H5,H3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H4,relation,suite-relations,H5,H4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H5,relation,suite-relations,H5,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,self:QL,I,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""self:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A0,relation,suite-relations,H5,A0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""D2-transform|D2-complete"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A1,relation,suite-relations,H5,A1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A2,relation,suite-relations,H5,A2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A3,relation,suite-relations,H5,A3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A4,relation,suite-relations,H5,A4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""D2-require"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A5,relation,suite-relations,H5,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,conjugation:QL,H,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": ""D1"", ""cf_view"": """", ""seam"": ""conjugation:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A0->H5,relation,suite-relations,A0,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""D2-transform.inverse|D2-complete.inverse"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A0->A5,relation,suite-relations,A0,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""D3:B3|D3:C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A1->H5,relation,suite-relations,A1,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A1->A5,relation,suite-relations,A1,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A2->H5,relation,suite-relations,A2,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A2->A5,relation,suite-relations,A2,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A3->H5,relation,suite-relations,A3,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A3->A5,relation,suite-relations,A3,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A4->H5,relation,suite-relations,A4,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""D2-require.inverse"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A4->A5,relation,suite-relations,A4,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""D3:A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H0,relation,suite-relations,A5,H0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""D2-require.inverse|D2-complete.inverse"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H1,relation,suite-relations,A5,H1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H2,relation,suite-relations,A5,H2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H3,relation,suite-relations,A5,H3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H4,relation,suite-relations,A5,H4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""D2-transform.inverse"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H5,relation,suite-relations,A5,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,conjugation:QL,H,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": ""D1.inverse"", ""cf_view"": """", ""seam"": ""conjugation:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A0,relation,suite-relations,A5,A0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""D3:B3|D3:C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A1,relation,suite-relations,A5,A1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A2,relation,suite-relations,A5,A2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A3,relation,suite-relations,A5,A3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A4,relation,suite-relations,A5,A4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""D3:A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A5,relation,suite-relations,A5,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,self:QL,I,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""self:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
rel.ql.q0.S,relation,product-field,q0,S,"[""cap.ql.kernel-parity""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-why,,,ql.html#whole/why,"People and Agents use patterns to understand a subject, but the pattern can remain implicit or become a label detached from the work. Quaternal Logic gives relational understanding an explicit form that can be inspected, compared and tested. Its research begins in the Epi-Logos inquiry into mind, relation and interiority; its software makes sufficiently specified parts of that inquiry operational.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q0"", ""source_unit"": ""whole-why"", ""seed_sha256"": ""3869e8d7edfd0673735a78ad9ff3a02303c7cf74fcf63c2d9b392dc5afd30316"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Why?
rel.ql.q1.S,relation,product-field,q1,S,"[""cap.ql.cli-discovery"", ""cap.ql.kernel-operators"", ""cap.ql.pairing"", ""cap.ql.lens-registry"", ""cap.ql.context-frames"", ""cap.ql.vak-source"", ""cap.ql.service-negotiation"", ""cap.ql.client-modes""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-what,,,ql.html#whole/what,"Quaternal Logic / MEF is a Rust software product with a native ql CLI, a shared formal kernel, twelve MEF lenses and versioned service and client contracts. It computes defined structural relations, exposes canonical registries, retrieves source-bearing Vāk context and supports attributable readings of existing subjects. The broader research programme develops further operations through explicit contracts and evidence.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q1"", ""source_unit"": ""whole-what"", ""seed_sha256"": ""ed702daab4d4f5a6a5d743844c8cda1a9e687b5302c16b4e48b3731aee8af483"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",What?
rel.ql.q2.S,relation,product-field,q2,S,"[""cap.ql.service-readings"", ""cap.ql.wiki-refraction"", ""cap.ql.wiki-portals"", ""cap.ql.living-wiki""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-how,,,ql.html#whole/how,"A user or client selects a subject, preserves its identity and revision, then chooses a formal operation or a refractive perspective. QL validates the structure and returns the result with its operation, version and source basis. Deterministic kernel operations act directly; richer readings use an explicitly available provider. A client can compare the reading with its original material and retain the difference that proves useful.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q2"", ""source_unit"": ""whole-how"", ""seed_sha256"": ""b64f4d4d716b918dbf2863312ad9e7c3115cce9773a7788d547894567a6e0c83"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",How?
rel.ql.q3.S,relation,product-field,q3,S,"[""cap.ql.shape"", ""cap.ql.external-frame"", ""cap.ql.wiki-structure""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-whereby,,,ql.html#whole/whereby,"QL serves people organising knowledge, developers exposing relational views and researchers testing formal propositions. Stable addresses, a whole anchor, complementary relations and multiple lenses let them inspect different aspects of the same subject. The subject remains owned by its originating product, while QL owns the formal reading and its provenance.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q3"", ""source_unit"": ""whole-whereby"", ""seed_sha256"": ""124c1f33c42b0140a1d750f54501605e96b42cd630f16f17a532028c2a381e4b"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Who / Whereby?
rel.ql.q4.S,relation,product-field,q4,S,"[""cap.ql.vak-context""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-context,,,ql.html#whole/context,"QL can be used at a terminal, through Rust libraries or as an optional provider within a larger O:I workflow. It is useful when a person needs to disclose a structure, compare relationships, or test whether a proposed distinction changes an outcome. Ordinary work and ordinary sources remain usable before any QL reading is requested; the chosen scope and available evidence determine the depth of a reading.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q4"", ""source_unit"": ""whole-context"", ""seed_sha256"": ""3164ed3d0ccf6d573724b7859bb8dc70ab6b89c92e20590bc7360511f885053b"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Where / When?
rel.ql.q5.S,relation,product-field,q5,S,"[""cap.ql.cli-verification""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-purpose,,,ql.html#whole/purpose,"QL is for developing a more explicit and relationally aware encounter with knowledge and agentic work. It gives O:I and Epi-Logos a common formal resource while leaving each product responsible for its own actions and meaning. Its success lies in useful understanding and demonstrable operations: a reading that clarifies a decision, a structure that preserves relationships, or an experiment that teaches us which distinctions carry real consequences.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q5"", ""source_unit"": ""whole-purpose"", ""seed_sha256"": ""0b2bff63c1b59bed272e2b4a2c7863e946064f70525302b179d8c9e2b17ae630"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Why-For?
rel.ql.q3.S0,relation,product-field,q3,S0,"[""cap.ql.pairing"", ""cap.ql.external-frame"", ""cap.ql.client-modes""]",,,,,agent-inference,ProjectCentral/user/ql.html#q3-identity,,,ql.html#q3/identity,Central holds durable source; AIKit interprets and navigates knowledge; other native products enact their own responsibilities.,,"{""basis"": ""Exact account sentence; cell placement is editorial inference."", ""source_unit"": ""q3-identity"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
rel.ql.q2.S2,relation,product-field,q2,S2,"[""cap.ql.wiki-refraction"", ""cap.ql.wiki-portals"", ""cap.ql.living-wiki""]",,,,,agent-inference,ProjectCentral/user/ql.html#q2-wiki-encounter,,,ql.html#q2/wiki-encounter,"An AIKit client can supply a bounded WikiFrame: the target wiki, revision, selected nodes and explicit mapping information.",,"{""basis"": ""Exact account sentence; cell placement is editorial inference."", ""source_unit"": ""q2-wiki-encounter"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
rel.ql.q3.S3,relation,product-field,q3,S3,"[""cap.ql.pairing"", ""cap.ql.external-frame"", ""cap.ql.client-modes""]",,,,,agent-inference,ProjectCentral/user/ql.html#q3-identity,,,ql.html#q3/identity,"A Factory Run, AIKit Resource or Wiki subject carries its native identity into a QL request.",,"{""basis"": ""Exact account sentence; cell placement is editorial inference."", ""source_unit"": ""q3-identity"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.m-registry-ledger,capability,,,,[],"Know which coordinates of the M tree exist, what is bound to them and what is proven",ql kernel coverage|ledger|validate-ledger; MRegistry and MLedger,"The native M registry and the shared ledger: per-coordinate views, validation findings that are not completion, and coverage for one coordinate by stratum, axis and required readiness",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/KERNEL-RECURSIVE-M-REGISTRY.md;docs/KERNEL-M-LEDGER-MATRIX-LOCK.md,crates/ql-mef/src/m_tree.rs;crates/ql-mef/src/m_ledger.rs;crates/ql-cli/src/m_ledger.rs,crates/ql-mef/tests/native_m_tree.rs;crates/ql-mef/tests/m_ledger.rs;crates/ql-cli/tests/m_ledger_cli.rs,ql.html#q1/kernel,,,"{""basis"": ""The registry consumes the generated native-C M tree and owns coordinate existence; the ledger owns bindings, source assertions, discrepancies and readiness, and a missing entry never means complete. The CLI reads the embedded ledger or one supplied with --ledger."", ""cli_commands"": [""kernel.coverage"", ""kernel.ledger"", ""kernel.validate-ledger""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits the ledger, its validation findings and a coverage report for one coordinate from the same embedded or supplied ledger.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-mef/src/m_tree.rs"": ""c1b65a51bf675656f9b38a3fd66fa685b3fd9a20a696b765ac1aeee5960fb09a"", ""crates/ql-mef/src/m_ledger.rs"": ""b022aef662a61de1ab87207bc160e038118c88fb5c2ddcecceae0ce7a4250b95"", ""crates/ql-cli/src/m_ledger.rs"": ""011e42078a010257bb19672655e0da24fdf4ceaec008637a3f7cebe9d01c3603""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.m1-engine,capability,,,,[],"Read the M1 ring, torus and harmonic state for one event and cycle",ql kernel m1 <request.json>; m1_engine::engine_json and M1Engine,"A versioned M1 state: the 12-fold clock and cell, source-literal and accepted-arithmetic readings kept apart, torus position, ratio evidence and the selected coordinate reading",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/KERNEL-M1-ENGINE-CONTRACT.md,crates/ql-mef/src/m1.rs;crates/ql-mef/src/m1_engine.rs,crates/ql-mef/tests/m1_engine.rs;crates/ql-cli/tests/m1_engine_cli.rs,ql.html#q1/kernel,,,"{""basis"": ""M1 composes the existing matheme, relation and music operators over the M registry. Caller pointer evidence is retained, not certified as a graph observation; storage and authorisation belong to the consumers of M1Engine."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""gap"", ""reason"": ""`ql kernel m1 <request.json>` runs and is tested, but `ql capabilities --json` does not list it, so discovery cannot reach it yet.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-mef/src/m1.rs"": ""54330879ea6ecbdd250adee62652e665f82f65ad9b6bc140bb77dd4214af908c"", ""crates/ql-mef/src/m1_engine.rs"": ""4e16b7ef57cc787366b95f9cf293738e104c2cc1a21dec17c0e3f557bc153ac3""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.m2-engine,capability,,,,[],"Read the Paraśakti field: descriptors, music, colour and material for an event",m2 descriptor field and finite operations; m2_engine; m2_condition; m2_vimarsha; m2_sky,"Retained descriptor tables and finite operations over one 72-reading carrier, the M2-1 Vimarśā reading from supplied M1 ratio and M3 pose, the sky of planets and chakras with the map values, and a joint music/colour/material condition for one event",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/kernel-rebuild/m2-engine-v1.md,crates/ql-mef/src/m2.rs;crates/ql-mef/src/m2_engine.rs;crates/ql-mef/src/m2_condition.rs;crates/ql-mef/src/m2_vimarsha.rs;crates/ql-mef/src/m2_sky.rs,crates/ql-mef/tests/m2_engine.rs;crates/ql-mef/tests/m2_condition.rs;crates/ql-mef/tests/m2_vimarsha.rs;crates/ql-mef/tests/m2_planet_chakra.rs,ql.html#q1/kernel,,,"{""basis"": ""The M registry owns coordinates; M2 does not invent missing nodes or promote every retained correspondence to canon. The engine is a producer boundary, not a physical solver: supplied observations are never relabelled as live or verified, and the seven musical modes are not the seven Context Frames."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""The M2 engine is a Rust library used by the scene, the Nara reception and the native engine tests; no ql command exposes it on its own.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-mef/src/m2.rs"": ""21782cd444ec1d56f4c80f08d16a2b464de8987cf948f8ca729a1e542071887f"", ""crates/ql-mef/src/m2_engine.rs"": ""b22c97f69fd2653f83404a39fdd63cdeff0ae1df4340386b1156a01c1a75e529"", ""crates/ql-mef/src/m2_condition.rs"": ""72cc2257ecafdc684fa368715ef591b50a30454902d3363a7a685d5bf1485402"", ""crates/ql-mef/src/m2_vimarsha.rs"": ""a3893d93fc851f4dddc42fd1d4247ad5c282a799a579068440232510004f8719"", ""crates/ql-mef/src/m2_sky.rs"": ""87cdb1f4fe5cd965898b64c61373f128809cc4e193b412aa7996cdc8082265c6""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.m3-engine,capability,,,,[],"Replay and read the transactional M3 state: codon form, pose, source and clock","ql kernel m3 <request.json|->; M3State, m3_engine, m3_source, m3_inscription","Receipts and a final snapshot for up to 256 commands against one exact discrete state, with an optional Nara activity reading; the complete source readings of M3 with typed finite projections; and the clock inscription seed read from the map at an ecliptic degree",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/KERNEL-M3-ENGINE-CONTRACT.md,crates/ql-cli/src/m3_command.rs;crates/ql-mef/src/m3_state.rs;crates/ql-mef/src/m3_engine.rs;crates/ql-mef/src/m3_source.rs;crates/ql-mef/src/m3_inscription.rs,crates/ql-cli/tests/m3_native.rs;crates/ql-mef/tests/m3_state.rs;crates/ql-mef/tests/m3_engine.rs;crates/ql-mef/tests/m3_inscription.rs,ql.html#q1/kernel,,,"{""basis"": ""The M3 state is shared by Cosmic, Personal reception and M3′. A ref or actor label in the library never grants disclosure or mutation; the host checks authority before calling it. The CLI replay neither authenticates a host nor admits its commands as personal activity."", ""cli_commands"": [""kernel.m3""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI replays an explicit request and its commands (stdin or file, 1 MiB at most) and prints the receipts and snapshot.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/m3_command.rs"": ""d86a11391c48527ac5b0198828e20dcef937534477a02b58c6ebbc2e1c5147da"", ""crates/ql-mef/src/m3_state.rs"": ""e70bb150db7ed248a2bda24a65ce3c787d50fe6dde290ef491f025f8650710bb"", ""crates/ql-mef/src/m3_engine.rs"": ""5eb86f06f97143d086eaa3c2448f81b8da78c9e9ab25f4a7ca2c10eb2a617b9f"", ""crates/ql-mef/src/m3_source.rs"": ""fe60137f379718bf8d5feb8adf5e27b1f43397df6e520bac84db56305ee6846b"", ""crates/ql-mef/src/m3_inscription.rs"": ""d21b53007561f3f8d72463c2ac99c1c36d271ac5ad5f076d170eea2c097100bd""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.scene,capability,,,,[],Compose one integrated M1-M2-M3 event from a dated sky and hear it,ql scene compose|binding|world|render; scene::compose; the torus scene field,"The dated sky placed on the clock with each planet voiced at its map ratio, the form advanced by M1, the Nara centres the sky reaches, and the same planets played as surface modes of the M1 torus; render writes a WAV with its PCM hash",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/kernel-rebuild/K8-COUPLED-EVENT-CONTRACT.md,crates/ql-cli/src/scene_command.rs;crates/ql-mef/src/scene.rs;crates/ql-mef/src/scene_sky.rs;crates/ql-mef/src/continuous/scene_field.rs,crates/ql-cli/tests/scene_native.rs;crates/ql-cli/tests/scene_render_native.rs;crates/ql-mef/tests/scene_instrument.rs;crates/ql-mef/tests/scene_sky.rs,ql.html#q4/deployment,,,"{""basis"": ""Every magnitude that is a choice rather than a map fact is a named tunable with that standing. A body the map does not hold stays visible as unmapped. Scale, damping, strike amplitude and gain have no source table."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""gap"", ""reason"": ""The scene commands (`ql scene compose|binding|world|render`) run and are tested, but `ql capabilities --json` does not list them.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/scene_command.rs"": ""ae4d8b4a9b00246fe473c94d7fbc9726f7a4b742d1162c1e36ee15cb7caa7643"", ""crates/ql-mef/src/scene.rs"": ""bda0a8558135ac5fb864f60706f39148df883fcc35547348dc8254fb1b5da7e2"", ""crates/ql-mef/src/scene_sky.rs"": ""d74bbfb99bd9a38ad366af2f9424d992bfac20ecf5abb515e4f53255f834d73d"", ""crates/ql-mef/src/continuous/scene_field.rs"": ""03e622553a063764bf313c84df43744fe3ab99db28a7dffeabf7228160c7f389""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.stage,capability,,,,[],Drive the Ta-Onta procedural stage of the live scene,ql stage state|evaluate|bind|unbind|retire; continuous::stage,"One owned exchange with the native field host: a versioned procedure is evaluated or bound over the stage slots (form, material damping, clock inscription, clock lensing), and the host receipt returns with the last acknowledged field on every refusal",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/kernel-rebuild/K8-COUPLED-EVENT-CONTRACT.md,crates/ql-cli/src/stage_command.rs;crates/ql-mef/src/continuous/stage.rs,crates/ql-cli/tests/stage_native.rs;crates/ql-mef/tests/ta_onta_stage.rs,ql.html#q4/deployment,,,"{""basis"": ""Evaluation is deterministic in the event basis; a contribution key derives from the procedure identity, revision and slot, so re-evaluation replaces only that procedure. Voice retuning has no admitted stage change and is refused by name."", ""cli_commands"": [""stage.state"", ""stage.evaluate"", ""stage.bind"", ""stage.unbind"", ""stage.retire""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""Each stage operation is one ql command that opens the field host through the app binding route and prints its receipt.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/stage_command.rs"": ""5d5117b83450ef52bdf21eb7a50248de097da930ffff08ef8f547b5ada3367ee"", ""crates/ql-mef/src/continuous/stage.rs"": ""66a8a62b36b19a674a0f4b5758766249f33920e3846a461d3f82f9c50feb7cb9""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.nara-identity,capability,,,,[],"Read a person's identity profile, natal chart and transit as QL readings",ql nara capabilities|inspect|calculate|transit|personal-current|personal-recompose|presence-consent,"The six-office identity reading of an editable profile, optionally with a natal calculation; a dated sky transit; the personal current that joins identity, transit and an M3 activity reading; its recomposition; and a consent-scope reading with no private state exported",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/NARA-IDENTITY-COMPOSITION-DECISIONS.md;docs/kernel-rebuild/K10-NARA-M4-CONTRACT.md,crates/ql-cli/src/nara_command.rs;crates/ql-cli/src/nara_presence_command.rs;crates/ql-mef/src/nara.rs;crates/ql-mef/src/nara/intake.rs;crates/ql-mef/src/nara/current.rs;crates/ql-mef/src/nara/expression.rs,crates/ql-cli/tests/nara_current_native.rs;crates/ql-mef/tests/nara_personal_field.rs,ql.html#q4/deployment,,,"{""basis"": ""Central owns persistence and history; QL validates and derives. A report is never a birth calculation, astronomy stays with the qualified sky provider, and absent activity is never fabricated. Numerical outputs are compositional readings, not health or clinical claims."", ""cli_commands"": [""nara.capabilities"", ""nara.inspect"", ""nara.calculate"", ""nara.transit"", ""nara.personal-current"", ""nara.personal-recompose"", ""nara.presence-consent""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""Each operation reads one request on stdin or from a file (2 MiB at most) and prints the reading; the natal runtime needs an installed uv.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/nara_command.rs"": ""3c680ac4abe6f01966b2a1c1623de35c9df1795bcf167eb13adae2d18f9c2b5e"", ""crates/ql-cli/src/nara_presence_command.rs"": ""43f8bdf3f12ab32fed402cf9480f8af26de59938740900116312cf024aa8a37d"", ""crates/ql-mef/src/nara.rs"": ""eba2eb4870380ca8136c25b73ba708a1a2b68ed87129620cdc0775bc4667bf6e"", ""crates/ql-mef/src/nara/intake.rs"": ""15b8a2b437d9cc0650637261cc49076f861d69b523400a0caab536522acf8b0d"", ""crates/ql-mef/src/nara/current.rs"": ""341d2eda142473f5f9f28bc6846bd70fe542c9e2c74271b7bb327e1f20bf2e7e"", ""crates/ql-mef/src/nara/expression.rs"": ""5bbcb24d4e189848936be671a4e7d03e635446376c62c6084927eb0289f97ac8""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.nara-dialogue,capability,,,,[],Admit a Nara/Epii dialogue turn against the current coordinate and registry,ql nara coordinate|coordinate-content|coordinate-bundle|source-inventory|context|delegate|enrichment|receive,"The rooted M world and Bimba binding of a coordinate, its content and a paged source inventory; a dialogue context checked against the actual current registry; a basis-bound delegation from Nara to Epii; and the admission of Epii's enrichment back into the turn",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/kernel-rebuild/K10-NARA-M4-CONTRACT.md,crates/ql-cli/src/nara_dialogue_command.rs;crates/ql-mef/src/nara/dialogue.rs,crates/ql-cli/tests/nara_dialogue_native.rs;crates/ql-mef/tests/nara_dialogue_conformance.rs,ql.html#q4/deployment,,,"{""basis"": ""QL owns Nara meaning, not speech transport. The host owns identity, Expression and profile reads and persistence; a supplied registry is never authority, and a context that differs from the native coordinate is refused."", ""cli_commands"": [""nara.coordinate"", ""nara.context"", ""nara.delegate"", ""nara.enrichment"", ""nara.receive""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""Each operation takes a request file or stdin; coordinate-content, coordinate-bundle and source-inventory are also accepted by `ql nara` but `ql capabilities --json` does not list them.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/nara_dialogue_command.rs"": ""095c7f50910256b6bf56cbfd90df221cd7bf154e2a55bf77dd42adb13265493c"", ""crates/ql-mef/src/nara/dialogue.rs"": ""eac129e28670f6f886149290144bd7b04bcde40d619d9e253536d71dd8e3efa6""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.epi-agent,capability,,,,[],Call the Prime-QL faculties #0 to #5 as a bounded domain surface,ql epi-agent constitution|faculty|invoke,"The agent constitution, one faculty with its capability field and operation inputs, and an invocation of a named faculty operation (Anuttara read, Vietoris-Rips, Bimba neighbourhood, representation binding, Tarot score, Nara journey and lived context, Logos return) with canonical mutation off",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/integrations/epi-logos/EPI-LOGOS-AGENT-ARCHITECTURE.md,crates/ql-cli/src/epi_agent_command.rs;crates/ql-mef/src/epi_agent.rs,crates/ql-cli/tests/agent_research_native.rs;crates/ql-cli/tests/pi_agent_native.rs,ql.html#q1/readings,,,"{""basis"": ""This surface creates no coordinate registry: it composes the current M registry, the source-locked Anuttara table, the Nara contracts and bounded formal operations. #0/1 belongs to the constituted acting body; faculties are #0..#5."", ""cli_commands"": [""epi-agent.constitution"", ""epi-agent.faculty"", ""epi-agent.invoke""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI prints the constitution, a faculty by position, or the result of one invocation request (16 MiB at most).""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/epi_agent_command.rs"": ""f9e3eebdd80af6fbbad890412f869c5ea6a3dd5281c5db65a66850f7705a5d92"", ""crates/ql-mef/src/epi_agent.rs"": ""a2941cb334b1b074218e3436c931f86095f0133c5a4a1dd8650a1c3796e3e1b5""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.agent-event,capability,,,,[],Project an agent event into a QL frame and admit a decision against it,ql agent-event project|frame|harmonic|validate,"The QL projection of a supplied event, its decision frame or harmonic readout, and for validate a rebuilt admission of a response that is bypassed, stale or admitted against the current owners",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/integrations/epi-logos/QL-AGENT-DECISION-CONTRACTS.md,crates/ql-cli/src/agent_event_command.rs;crates/ql-mef/src/agent_event.rs;crates/ql-mef/src/agent_event/admission.rs,crates/ql-cli/tests/agent_event_native.rs;crates/ql-mef/tests/agent_event.rs,ql.html#q1/readings,,,"{""basis"": ""No model runtime runs here and no provider ontology is accepted. Admission rebuilds the projection and trusts no caller frame or purported admission; a response against a different event, source, frame or kernel basis is stale."", ""cli_commands"": [""agent-event.project"", ""agent-event.frame"", ""agent-event.harmonic"", ""agent-event.validate""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""Each operation reads one request (1 MiB at most) from a file or stdin and prints JSON.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/agent_event_command.rs"": ""2f851e9982b76652e0fa7ba2996ae688c500ae3c17f89e7b7683fc8474ed1708"", ""crates/ql-mef/src/agent_event.rs"": ""35211df6d3a266d0378a5c8fa580eb6ef399e9a9dee1ef751da86bf9610285b3"", ""crates/ql-mef/src/agent_event/admission.rs"": ""4018fc4dc2c73db78af44ac2c0615978c996a1b9a51b19f39af635eb596089de""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.configuration,capability,,,,[],Disclose QL's configuration contribution to the O:I configuration plane,ql config validate|plan|apply|reset; ql config-contribution,"The frozen configuration-contribution document with every setting read-only, validation and plan answers, and a structured unsupported-setting refusal from apply and reset with no receipt minted",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,crates/ql-cli/src/configuration.rs,crates/ql-cli/src/configuration.rs,crates/ql-cli/tests/configuration_plane.rs,ql.html#q4/deployment,,,"{""basis"": ""Only what exists is offered: every accepted kernel faculty is a read-only disclosure, so QL accepts no configuration write and O:I never needs QL. A failed --json invocation exits non-zero with an oi.config-error document."", ""cli_commands"": [""config.contribute"", ""config.validate"", ""config.plan"", ""config.apply"", ""config.reset""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The contribution document and the four transport verbs are ql commands; `ql system` and the O:I System disclosure are separate and not listed by capabilities.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/configuration.rs"": ""8f745301cfb7dca14578ba9ee5b0268bbbed2120bfe124809ac17aac2aea1dc8""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.vak-composition,capability,,,,[],Compose a C-prime Context-Frame use and publish the Vāk workflow types,ql vak compose; ql vak workflow-types,"Immutable whole uses composed and reframed over the Vāk registry with each transition a new use, and the TypeScript types module rendered from the Rust contract that Factory's typed workflows import, with a --check against the checked-in file",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/QL-VAK-KERNEL-RECONCILIATION.md,crates/ql-cli/src/vak_composition.rs;crates/ql-mef/src/vak_composition.rs;crates/ql-mef/src/vak_workflow_types.rs,crates/ql-cli/tests/vak_composition.rs;crates/ql-mef/tests/vak_composition.rs;crates/ql-mef/tests/vak_workflow_types.rs,ql.html#q1/readings,,,"{""basis"": ""The layer is operative over the accepted structural carrier and adds no second vocabulary, expression parser or scheduler. An authored C-prime is a requirement for conduct, not an execution receipt or grant; Factory lowers it and owns the Run."", ""cli_commands"": [""vak.compose"", ""vak.workflow-types""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI composes a step list from a request and prints or checks the rendered workflow types module.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/vak_composition.rs"": ""22dece749dd1b4ab81e298f233c457b787fabfb887fbc9ba9245ac7990e30006"", ""crates/ql-mef/src/vak_composition.rs"": ""b9f9a580ee5981f52ccdaeaca4b335a93bcb9dc0c727264a7721a7f6080c4df1"", ""crates/ql-mef/src/vak_workflow_types.rs"": ""76aafc42f22b42d687c873b2ddaef1d6eb91f0f08ae3cedb1b836911061cc04c""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
cap.ql.techne-reading,capability,,,,[],Read one Wiki subject through the production Technē adapter,ql techne reading <target.json>,"The validated Technē reading of a Wiki refraction target and its co-referenced selections, with every reference copied from the target, the provider or the engine, and a bounded whole disclosed only from a validated shape binding",implemented; workspace tests (cargo test --workspace --all-targets --locked) passed 2026-10-19,agent-inference,docs/L5-TECHNE-INSTRUMENT-WAYFINDER.md,crates/ql-cli/src/techne_command.rs;crates/ql-adapters/src/techne_wiki.rs,crates/ql-cli/tests/techne_reading.rs;crates/ql-adapters/tests/techne_contract.rs,ql.html#q1/readings,,,"{""basis"": ""The file is the subject source; the command holds no store, executes no mutation and never rewrites native identity. Layout never manufactures graph relations."", ""cli_commands"": [""techne.reading""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI reads one target file (16 MiB at most) and prints the reading.""}, ""maintenance"": {""change_refs"": [""QL-MEF:capability-matrix-engines-20261019; claude-code:sonnet-followups""], ""code_basis"": {""crates/ql-cli/src/techne_command.rs"": ""7ddfb4f2f8578c85670fc9d55ef0179a0a9405cb096693808ad338090b152380"", ""crates/ql-adapters/src/techne_wiki.rs"": ""0d31b67b4df87dbfc3eb98a021545022dc9724e14dba82341b6d436645d0efa7""}, ""updated_at"": ""2026-10-19""}, ""last_reconciled_at"": ""2026-10-19T20:19:29.677000+00:00""}",
```
