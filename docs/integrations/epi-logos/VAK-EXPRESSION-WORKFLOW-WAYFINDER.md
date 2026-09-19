# Vāk Expression workflow — author, perform, curate, invoke

**Standing:** owner-directed design and execution clarification, 19 September 2026. The owner commissioned publication after correcting the previous proposal: operative Skills must not be development reports; Vāk should have a lean TypeScript workflow definition which becomes the first source stamp of a Factory undertaking; real subagent roles and dynamic Expression discovery/invocation are essential. This document specifies the work to execute after the current integration churn. It does not claim an installed TypeScript SDK, loaded agents, a live run or human-assessed craft.

**Existing owners:** QL-MEF #201 / #135 and #94; Factory #217 and its current execution/Return successors; AIKit #267 and its current native practice/provider successors; O:I #65, #306/#335/#366 and #375; Point-Cloud-Demo #6. The [K/AW production map](../../../.wayfinder/maps/k-aw-expression-production.md) remains the parent entry. This is its focused implementation and proving line, not another agent runtime, acceptance campaign or kernel wave.

**Source precedence:** current author corrections; [Ta-Onta full field](TA-ONTA-FULL-FIELD-LOCK.md); [Vāk/Oikonomia](../../kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md); [K/AW production alignment](K-AW-EXPRESSION-PRODUCTION-ALIGNMENT.md); [dual reading](../../L5-TECHNE-DUAL-READING-LOCK.md); current native owner contracts and evidence. Preserve the complete M×S′/SP capability fields. Older Atelier-service and replacement-agent-roster proposals are superseded, not alternate production modes.

## 0. The experience

A person asks the agent to make or adapt an Expression. The agent can find the relevant work and current controls, compose a fitting undertaking in Vāk, enlist the right actual participants, perform in the medium, inspect what happened and respond to correction. Useful work becomes a curated, source-bound repertoire. Later a name, action request, mood description or deliberate gesture can find and invoke it with new inputs, without rehearsing the original implementation history.

The first proving commission is carried by **Factory itself**, not a script with a Factory label attached. Its participants operate the existing O:I Expression application. A later ordinary expressive act can use the same faculties directly; it need not mint a Factory Run for each gesture, frame or scene edit.

The separation is operational:

| Layer | Carries | Must not carry |
|---|---|---|
| Lean Vāk language / role instructions | How to determine, compose, perform and return; stable source identity and relevant constraints | Issue chronology, build commands, migration diary, old missing-feature claims, whole-repo reading instructions |
| `*.vak.ts` workflow source | This undertaking's typed C′ composition, role requirements, subjects, effects, passage and return/proof requirements | A second scheduler, self-granted credentials, arbitrary embedded page scripts or copied native stores |
| Native live tools | What work/faculties/parameters are available now; exact reads, permitted acts, currentness and observations | A frozen inventory in a Skill, private debug handles, ambient authority |
| Development/proving records | Implementation gaps, migration, tests, revisions and readiness | Mandatory context for every runtime actor |

A concise Skill may explain when and how to use the Vāk source/tool entry. A Method remains a Skill whose **description** starts `METHOD:`. Neither is the live search engine or the source of today's parameter values.

## 1. The TypeScript pattern and its native office

### External research, not product authority

Anthropic's official [SDK subagent guide](https://code.claude.com/docs/en/agent-sdk/subagents), inspected 19 September 2026, documents programmatic `agents` definitions supplied to `query()`: description for delegation, prompt for role behaviour, tool and model configuration, and isolated child conversations. Its [custom-tool guide](https://code.claude.com/docs/en/agent-sdk/custom-tools) documents typed tools with a name, description, input schema and handler, exposed through `tool()` / `createSdkMcpServer()`. This is a useful authoring and adapter pattern, not an official Vāk workflow format.

The [Skill guide](https://code.claude.com/docs/en/agent-sdk/skills) and [tool-search guide](https://code.claude.com/docs/en/agent-sdk/tool-search) separate selected/preloaded guidance from discoverable live tool schemas. Pin actual SDK/harness versions when implementing; delegation, loading and permission behaviour must be verified there rather than copied from a rolling documentation example.

### Native choice

Use **TypeScript as the readable, typed source form of a Vāk undertaking**. The provider-neutral native composition is the authority; a Claude `AgentDefinition` is one target projection. Other admitted harnesses/model bodies must receive equivalent roles, context and native faculties through their actual adapters. No Claude dependency belongs in the QL kernel or generic Factory workflow meaning.

The inspected native bases are:

- Factory `factory/src/workflow.rs`: `factory.agent-workflow-source/v1`, `WorkflowSourceProvenance`, `AgentRequirements`, `WorkflowUnitSource`, dependencies, independence, barriers/nesting and `CompiledWorkflow`.
- Factory `factory/src/vak_orchestration/model.rs`: `CPrimeExecutionBinding` over `ql.vak-composition.profile/v1` plus `aikit.operative-scope/v1`; original actor/whole/subject/source/path/context and participation.
- QL `crates/ql-mef/src/vak_composition.rs`, `vak_profile.rs`, `cprime_oikonomia.rs`: source-owned language/form/composition and performance qualification.
- AIKit `resource/operative.rs`, `operative_scope.rs` and native invocation/practice implementations: the existing AST, scoped resolution, effective context and reusable praxis.

Those entry paths were inspected in September 2026 source. Reinspect current mains and relevant pushed candidates before changing them. A compatible existing TypeScript frontend wins over creating another one.

The deliverable is a small source frontend/SDK projection into these owners, with types generated or contract-checked against the owning schemas. **Do not fork their Rust types into an independently maintained TypeScript ontology.** Missing fields are added at their real owner with compatibility tests. The adjacent [TypeScript specimen](specimens/expression-craft.vak.ts) illustrates the desired authoring separation; it is self-contained specification material, not a shipped SDK or executable Factory entry.

## 2. The first stamp of a Factory undertaking

The creation path is:

```text
human request / permitted NOW ground
  → authored or agent-prepared *.vak.ts
  → typecheck and restricted deterministic lowering
  → source-qualified QL C′ binding + native Factory WorkflowSource
  → inspectable first-source stamp on the Commission/Run basis
  → actual role/body/tool resolution and authority
  → native admitted dispatch / performance
  → observed results / Return
```

Attach the source ref/revision, transitive source/import digest, compiler/schema version, lowered native semantic digest, subject/whole/source basis, participation regime, capability/role requirements, intended effects and return address through existing provenance/Run fields. The source stamp records what is to be performed **before** its effects; it is not an extra database, a new Run identity or permission to execute.

Keep source bytes and semantic digest distinct: harmless source formatting can alter the source revision without changing native semantic identity. Execution-relevant changes in roles, C′ choices, dependencies, effects, inputs or bounds must change the applicable compiled basis. Resolve current bodies/faculties against that basis at dispatch and retain the actual resolution independently. A later legitimate recomposition creates a new attributable revision/continuation, not retroactive editing of the original stamp.

Use a constrained data/builder subset and allowlisted imports with deterministic lowering. Do not `eval`/import arbitrary submitted TypeScript in the desktop, a live user context or a privileged compiler. Any authorised build-time execution needs the existing Workcell isolation, bounded resources and explicitly declared inputs. No network, clock, randomness or environment-dependent values may silently change the resulting workflow. Expression/page files do not gain executable-script authority from this authoring format.

A definition is allowed to be dialogical: it can propose or ask without authorising mutation. The commissioned craft test receives its normal bounded execution grant once. No approval on every harmless parameter change; no source-authorship, publication or privilege escalation hidden in that grant.

The public user entry must be discoverable through the current Factory/AIKit interfaces. Implement the native capability/CLI/provider action and its help rather than inventing an unregistered command in a Skill. The person sees purpose, proposed participants, intended changes and controls; exact bindings and source remain an inspection away.

## 3. Vāk remains precise and lean

The authoring surface keeps **CPF, CT, CP, CF, CFP, CS** explicit with their existing values and QL interpretation:

- CPF: dialogical or authorised-undertaking participation.
- CT: content/material kind, including the retained CT4b′ source specialization where applicable.
- CP: 4.0 ground, 4.1 definition, 4.2 operation, 4.3 pattern, 4.4 context, 4.5 integration.
- CF: the source-defined constitutional/relational frame; retain its active lens/musical basis through the QL binding.
- CFP0–5: single voice, independent parallel/chord, chain/melody, fusion, sustained/drone, nested/canon.
- CS0–5: exact source-defined paired passage, with direction and extent distinct.

The governing Z cycle remains **compose → perform → record → rehear → recompose**. It is not a seventh CFP or an infinite ungranted loop.

The atomic `@# - + x / =` × `@0…@5` operations use the existing `ResolveExpression`; full Vāk remains the 109-entry source field. Do not make the TypeScript example, six compact operators, seven CFs or one material test an exhaustive replacement. Detailed source meanings, valid values and compositional laws should be obtained from the current QL types/registry, with a short clean language rendering for agents. Mutable availability belongs to live tools.

For every selected C′ determination test what changes in real conduct: participation, content selection, addressed position, contextual voice, topology, passage or Return. Musical performance uses source-backed active harmonic/MEF interpretations and actual execution events; an arbitrary note/theme assigned to a status cannot prove this.

## 4. Real roles, not a list of decorative agent names

The declared field and the actually selected cast are different. Pleroma makes the real capabilities available; Anima selects participants according to the undertaking and their current faculties. A small act can remain one agent. Plural work must really delegate when its composition requires it; do not report simulated personas as independent subagents.

### Constitutional field hosted by Anima

| Role | Source frame | Contribution and return expected |
|---|---|---|
| Nous | CF1 | Open the concern and ground without premature determination; return the relevant question and sources. |
| Logos | CF2 | Articulate distinctions, definitions, constraints and source-bearing form; return a usable precise specification. |
| Eros | CF3 | Bring intention/need into operative exchange with available powers; return an actual permitted operation and its consequence. |
| Mythos | CF4 | Find and articulate the organising image/pattern; return a composition which preserves its relation to the concern. |
| Anima | CF5 | Compose the contextual undertaking and its differentiated voices; retain execution, boundaries and intended Return. |
| Psyche | CF6 | Steward continuity/distribution across context, actors, occasions and bridges; return a recoverable situated relation. |
| Sophia | CF7 | Integrate performed differences and unresolved questions into renewed ground; return a source-qualified synthesis. |

These are role contributions, not compulsory sequential steps or product IDs. The actual C′ progression and CFP decide how they participate. Changing CF need not spawn another process; changing the material model does not remint a canonical actor.

### Existing specialists

Preserve the source definitions and guarded domains of **Anansi** (coordinate/blueprint), **Janus** (temporal/threshold), **Moirai** (GraphRAG distillation, including its source-defined modes), **Mercurius** (Kairos/qualitative temporal patterns), **Agora** (aggregation/skill/plugin absorption) and **Zeithoven** (creative advance/skill-agent creation). Select them where useful; do not require a six-specialist swarm for every scene.

**Aletheia is S5′ disclosure/crystallisation/Return, not an eighth constitutional peer. Techne remains Pleroma's atomic-skill substrate.** Nara and Epii retain their canonical participant relations and the current dual-reading bindings. Do not replace these with corpus-specific `composer/critic` Agent identities. Composition/material/temporal/integration and witness/critic/curator/praxis can name task responsibilities inside a run, not a replacement ontology.

Each selected participant definition must convey, tersely: when to use it; native Agent/Profile/role source and revision; bounded subject/context; actual faculties and relevant clean Skills; output/Return contract; allowed effects; collaboration dependencies; stop/escalation conditions; and the selected body/model requirement. Resolve IDs from their source; never fabricate a canonical `agent:` ref from a display name.

The subagent's real context must contain its own role/praxis and the permitted task material. Do not assume parent prompts, loaded Skills or credentials are inherited. Correlate actual child invocation/session/attempt/tool events and returned artifacts with native Factory/Actuation identities. Provider labels such as Claude `Agent`/older `Task` are transport details, not new Workflows. Unsupported nesting/faculties remain explicit; native Factory nesting must not collapse because one provider has a shallower subagent limit.

Independent verification remains independent from the executor. A different role label, rereading the same answer, or an agent asserting that it verified itself does not establish that separation.

## 5. A live Expression tool surface

The agent needs native discovery and action **while making**, not an ever-longer Skill. Reuse and extend the current AIKit Search/Resolve and O:I Expression application/transport. Provide a small discoverable tool family; the labels below describe offices, not new canonical command names:

| Office | Inputs/result and native owner |
|---|---|
| Search repertoire | Current World/Project/subject, name/action/mood/gesture cue, scope, bounds; AIKit returns actual native work/profile/Method refs, revisions, provenance, availability and applicability. |
| Inspect/describe | O:I exposes selected Expression/scene/entity, source and current/evaluated/base state, actual supported operations; load relevant parameter schemas from the native registry. |
| Compose/apply | Validated operation batches on exact refs/revisions, meaningful atomic boundaries and native authority; O:I remains application owner. |
| Perform/invoke | Bind an existing work or named Method to current inputs, eligible placement and control; use its actual owner Actions, not arbitrary script execution. |
| Observe | Application events plus material/renderer/telemetry/capture evidence at explicit revisions/times; neither a returned request nor decorative motion proves its effect. |
| Interrupt/continue | Stop/hold pending choreography and related work as commissioned, preserve manual edits and meaningful checkpoints, and expose current/uncertain effects. |

Search is read-only; opening/preview, editing, performing, saving, sharing and source promotion are different effects. Do not turn every search result into execution, or send a public name plus arbitrary JSON to an unchecked dispatcher. Invocation rechecks current ref/revision, subject scope, schema, permitted overrides and authority. Refusal keeps useful work and explains the exact next supported act.

Keep initial tool descriptions compact and action-oriented; load detailed schemas for selected faculties rather than inserting every parameter/tool into every prompt. Tool-schema search discovers operations; repertoire search discovers work/practice. They can share native resolution infrastructure but return different objects. A Claude MCP adapter can expose the same native handlers; it must not become another Expression application or another store. Tool availability/allowlists and native execution grants remain distinct.

### Full material coverage

Recover scope and units from the actual engine registry, not the earlier limited `glyph/x/y/z/scale/share` aperture. Account for formations and force-only pins; glyph/image/ASCII and layered bodies; field/relational/pointer forces; topology and resonance/material/colour; camera versus working/arrangement plane; entity sequence versus whole-scene sequence; grouped automation/recorded takes/manual takeover; source/subject bindings; capture and placements. Restrict only where the intended public faculty or actual provider requires it, with a precise reason.

The API must preserve stable entity/link IDs, units and hard bounds, requested/base/evaluated values, driver/blend precedence, protected owner-fed state and real supported pause/recovery semantics. High-frequency simulation stays in the existing engine; issue meaningful parameter/automation commands instead of launching an LLM or HTTP request every frame. An automatic field clock and an agent's new act are distinguishable.

No new renderer, body, simulation clock, audio owner, Nara store or private debug protocol. Retain seven independent M4 centres versus physical resonant stations. Configuration/driver recovery is not a bit-exact particle or resonator rewind.

## 6. Curated repertoire and repeated invocation

Retain three related native objects:

1. **Work:** Expression/Profile/Scene/Edition and source-bound collection membership.
2. **Practice:** an optional reusable Method/Skill or declared Action composition, its inputs, range of variation, expansion, applicability and evidence.
3. **Cue:** name/action request/authored mood association/deliberate gesture which helps resolve the work or practice in the current context.

Cues are input/readings, not another identity store. Mood is an authored or current user description; do not diagnose an internal psychological state from motion or assign a universal emotion→frequency table. Gesture requires an actual observed input binding and current context. Test ambiguity, collisions, refusal, near-miss and gesture feedback/debounce without imposing one unrequested gesture vocabulary.

The same work can be invoked with bounded variation: another subject, slower passage, gentler material, fewer objects, a different supported placement. Preserve its recognisable structure and original edition. Invocation creates a new occasion, not a rewrite of the original performance. Editable variants, source-derived profiles and reusable Methods remain distinguishable.

Use the existing AIKit Inbox/Skill/`METHOD:`/proof/catalogue route for recognised `= name` praxis; do not create a Method registry or treat `=` parsing as promotion. Saving a named Expression is ordinary authored work, not necessarily promotion to proven praxis. Candidate learning and technical verification can proceed automatically inside the commission; user taste, constitutional source, public publication and authoritative promotion follow their own native decisions. No premature H gate blocks construction.

T/T′ captures concise useful questions, decisions, anomalies and integrating returns against actual activity and user correction. Consume them into later useful knowledge/practice; do not fill twelve mandatory files or expose hidden reasoning. Fresh-agent reuse is the test of useful retention, not the presence of a Skill file.

## 7. Implementation and proving line

The labels VW0–VW6 below are local work packets under the existing owners, not new capabilities or a new programme. All scope is mapped before specimens are chosen; a passing first specimen never closes the rest by implication.

### VW0 — actual cut and complete faculty/role ledger

Parent owns this bounded entry. Consume the current Mac-cut and Factory/Day/NOW returns, the current O:I integrated/new-UI contracts and real QL/AIKit/Factory source. Inspect relevant pushed work before declaring gaps. Preserve the QL/M kernel freeze from the separate installation/review commission; this line changes no mathematical canon and does not silently update the Mac's held QL install.

Account for the complete S′/C′/role field and intended expressive controls through: source → code → tool/transport → discovery/loaded role → native effect → observation → Return. Distinguish existing implementation, current candidate, missing binding, missing practice and runtime question. Confirm the baseline findings from the prior source pass: rich native material versus limited agent parameters; ExpressionWorld operations versus external socket reachability; current role/Skill delivery. Do not make another monolithic audit a prerequisite for straightforward independent checks.

### VW1 — TypeScript source and first native stamp

QL owns the Vāk source frontend and exact meaning; Factory owns native workflow compilation/admission and first-stamp attachment; AIKit/Actuation own effective participant/body resolution. Reuse/complete the existing TypeScript/native route if present. Deliver generated/contract-checked types, clean role projections, deterministic restricted lowering, source/digest relation, descriptive compile errors and a real public entry.

Test valid and invalid CPF/CT/CP/CF/CFP/CS; source/import changes; illegal effects; missing/refused role bindings; dependency/independence conflicts; same semantics/different formatting; changed semantics/new basis; honest resume/recompose. Compile emits no action. No speculative package import or example-only CLI counts as shipped support.

### VW2 — live Expression search and performance tools

O:I/AIKit own the live capability family in §5 over existing stores and operations. Bind full intended native parameter/control coverage and exact observed results. Test safe bounded queries, currentness, permissions, batch atomicity, simultaneous drafts, unsupported controls, manual takeover, interrupt and reconnect. No DOM scraping or private debug handle is the normal agent route.

Read actual adapters and callers on both sides. Where GUI/engine code is already being refined, return small compatible bindings/regressions to its owner rather than creating a third app. CLI/API success and new-UI consumption retain separate evidence.

### VW3 — clean Vāk instruction and role delivery

Replace development-history-laden runtime bodies with lean language/role/tool-use contracts, preserving their full operative meaning and stable source refs. Move history, build/debug instructions and readiness to development docs. Preserve valid specialist, formal-operation and maintenance Skills in their proper offices; do not delete unrelated useful practice wholesale.

Discover the actual canonical source/defaults rather than inventing another roster. Generate/load the right target-native agent definitions, including actual child Skill/context delivery, and prove trigger/near-miss and native operation use with a fresh actor. Startup needs the small grammar, role and discovery aperture—not every repository's Wayfinder. A missing native tool is VW2 work, not more prompt prose.

### VW4 — real Factory expressive commission

Run a small but complete source-bound creative commission through the actual Factory execution path. Select a fitting real cast; prove every CFP form with meaningful separate cases, exact CS passage and the other C′ conduct. Parallel alternatives use separate drafts; chained contributions consume actual returned material; fusion has independent comparison; sustained work really stops in the material world; nesting retains parent/child scope and actual native dispatch.

Exercise objects/forces/space/time/continuity/change. The operator receives the ordinary request and actual tools, not an answer key or hidden test handles. Observe both semantic state and real engine effects. Inject a disconnected handler or stale/wrong-subject response: the verifier must reject it. Track material quality, task correctness, context/latency/resource use and unresolved human judgement separately—no arbitrary creative score.

### VW5 — curation, invocation and Z over time

Build a varied coherent set through real work; retain useful failures and human edits. Re-enter with a fresh agent and invoke by name/action/mood/gesture using native resolution. Adapt a retained movement to a new subject/placement, inspect its expansion, keep the original and return the new occurrence.

Prove compose→perform→record→rehear→recompose against observed outcomes; any learned Method uses the existing recognition/proof/source pipeline and is tested on changed inputs and near-misses. Count learning overhead and rejected candidates. A saved transcript or auto-generated Skill file is not a completed learning loop.

### VW6 — integrated experience and handoff

Prove the same source-bound work through current Factory chat/full Run and Run/Agents/Context, focused Expression/Nara, relevant Technē/Epii depth, Library/page, subtle/embedded placement and admitted shared presentation where commissioned. Follow current #375 placement, not older permanent sidebars or old desk layouts.

Corpus work consumes the actual source→Wiki/Knowledge→bounded whole→Expression path of #366; a detached export cannot stand in for this relation. Inspect the current image/video fallback separately from live behaviour. Preserve the same subject/edition/occasion and privacy through transitions. User craft judgement is obtained on meaningful joined work, not used as a veto on unfinished infrastructure.

## 8. Parallelism and landing

After the bounded VW0 account, **VW1, VW2 and VW3 may run in parallel** under one integration lead. Publish their shared operation/role/type contracts early and coordinate common files. VW3 can draft the lean grammar immediately but cannot declare missing tool support available. VW4's fixtures can be prepared alongside; real runs consume the returned working joins. VW5 and VW6 widen exercised work, not restart the earlier programmes.

Use Omarchy and existing sandboxes for code/build/provider/engine tests. One reusable code worktree per coherent feature line, not per agent/session/finding. Disjoint artifact production requires no additional worktree. Serialise writes to the same Expression revision, shared source index, Git index, resident endpoint and active GUI. Preserve the Mac's frontend branch, installed candidate and current tasks. A sandbox with software WebGL is scoped material evidence, not Mac/hardware or personal aesthetic acceptance.

Repairs land at native owners through current PR/CI and coordinated consumer joins. Route defects found while crafting into the actual Factory/NOW path when operational; a bounded bootstrap observation does not masquerade as self-hosting. No automatic merge or source-promotion authority follows from a passed test. Do not launch these workers merely by publishing this plan.

## 9. Spine and closure

The O:I counterpart is `docs/experience/EXPRESSION-FIELD.md` §9 with a registered `vak-expression-craft.json` source module in #65. It retains EF01/02/04/07/08/09/10 and QL UX01/04/05/06/08/09/10/11. Existing #65 C0–C5 governs preparation, actual use, stress, native repair and independent return.

Close each bounded packet from its explicit proof, not from a full-system feeling. Final return contains: exact source/installed/provider/UI vector; source-to-faculty/role coverage; real TypeScript source and native stamped workflow; actual loaded cast and tools; expressive artifacts/editions; original failures and native repairs; independent material and negative/recovery evidence; named-practice reuse; remaining UI/hardware/human questions. Preserve implemented, reachable, loaded, exercised and human-assessed as different statements.

The completion object is **an agent that can genuinely make, vary, inspect, curate and invoke expressive work through the Vāk-composed native world**, with Factory carrying the commissioned work and its improvement. It is not a cleaned-up prompt pack standing in for that world.
