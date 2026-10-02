# Ta-Onta procedural Expression stage — implementation Wayfinder

**Commission:** [QL-MEF #296](https://github.com/EpiLogos/QL-MEF/issues/296), 2 October 2026.  
**Parallel counterpart:** [physical-musical instrument #281](https://github.com/EpiLogos/QL-MEF/issues/281).  
**Programme:** existing #201/#135 and TA0–TA7; O:I #220/#335/#375 integration and #65 experienced acceptance.  
**Standing:** execution specification for the owner's accepted procedural-stage design. The source minute preserves the owner's intention; the contracts make that intention implementable. Publication supplies development instructions, not runtime acceptance.

Read this document with [state/composition contracts](TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md) and [parallel dispatch and acceptance](TA-ONTA-PROCEDURAL-EXPRESSION-DISPATCH.md). They extend [Ta-Onta → Expression SDK/API](TA-ONTA-EXPRESSION-SDK-WAYFINDER.md), its Atlas and existing native faculties. #296 integrates the procedural stage while #281 integrates the musical instrument, through the same native and application owners.

## 0. Authored ground and the encountered product

### 0.1 Owner correction and acceptance

The owner identified the required object in the 2 October conversation:

> “we have expressions with scenes and glyphs which have their own transformations, colour, physics etc, we're wanting to be composed actual scenes, it's potentially a real playable stage”

> “the whole pasu ontology (entity to form), the idea of having all entities having their given place in the system, which might be at the level of a glyph, force, a movement/sequence, a scene or collection of scenes (I.e a full expression)”

> “the different bimba coordinates as different expressions in that they're essentially just changes of state, and indeed just changes of location”

> “the procedural system [can] simply inject state changes or new scenes or whatever in the flow”

> “a centralised and clean way of knowing and changing state in both global and granular ways”

The fold discussion joined two construction ideas: transformations of the glyph rasterisation and a square polygonal template. The accepted design uses square/material rest coordinates carrying the sampled glyph, with source-driven crease deformations performed through the entity's existing sequence. The owner then commissioned faithful documentation, differentiated tasks and detailed disambiguation for development in parallel with #281.

These quotations and that explicit acceptance are the authored basis of this tranche. The implementation details in the companion contract are engineering determinations serving that basis. Preserve this relation when revising either.

### 0.2 The product to deliver

A person inhabits the existing Expression stage. Its formations, glyph layers, forces, motion, colour, physics and sound are available for direct composition. A human or Agent can understand the current whole, select one constituent, change its operative state, generate a new passage, intervene while it plays, and return to a canonical Bimba place with the same continuing work.

Paśu supplies the subject and its relations. Its expressive manifestation supplies the appropriate role and scale: a glyph/body, influence, movement, scene or entire Expression. M3 determines source-qualified form and transformation. Ta-Onta resolves and performs the composition through native capabilities. O:I owns the actual scene application and running stage.

A successful encounter is concrete: open the clock place; select a subject expressed both as formation and force; inspect what drives it; change a global relation and one local constituent; perform a native M3 change and watch its glyph fold; introduce a generated passage; edit it while it develops; revisit the original place; save, reopen and continue. The musical/physical instrument participates through the same state and timing joins.

### 0.3 Scope and fidelity

The source-defined Ta-Onta, M3 and instrument fields remain the scope. The example above is a joined demonstration, not the inventory of all requirements. Carry complete relevant branches, exact primes, source relations, intended operations and their reasons through the existing capability matrices. Use those matrices to discover omissions as well as to report existing rows.

Build inside the current application. Its resting face stays the stage, with Studio, source, Agent and deeper Technè tools summoned over it. General Expression composition remains useful without an Epi profile; Epi adds its source-bearing determinations and native operations.

## 1. Current foundations and source route

### 1.1 Read order

Begin with O:I `docs/positions/FOUNDING-POSITIONS.md`, the applicable `AGENTS.md` chain and the actual assigned issue. Then read this Wayfinder and the contracts. Load the complete relevant source sections for the selected packet from the dispatch document. The integration parent reads all three documents and both programmes' current writer claims.

Current code determines the extension points. Authored product meaning determines the result. Follow explicit later owner rulings where historical specifications differ, retaining both the original source and its correction.

### 1.2 Inspected native foundation

These are concrete source entry points inspected for this publication. Blob pins establish the reading basis; development consumes their actual current successors and records differences.

| Source | Inspected basis / operative contribution |
|---|---|
| QL `docs/integrations/epi-logos/TA-ONTA-EXPRESSION-SDK-WAYFINDER.md` | Blob `71e2b1c965e350a690853b008fe2091d66e37571`. Ta-Onta SDK, world entry, Atlas, inherited profiles, Nara/Epii and continuation. |
| O:I `docs/cradle/EXPRESSION-FIELD.md` | Blob `5db15accdc6fade9ecec0e7e2cde59de273aab2c`. Stage-first product, native subjects, canonical coordinates as loci of the same whole. |
| O:I `docs/contracts/EXPRESSION-APPLICATION-V1.md` | Blob `a06cefea148527b83b09ed5a5462782134995323`. `KernelOp::Expression`, shared human/Agent requests, stable refs, atomic edits, profiles, carriers, triggers and recovery. |
| O:I `desktop/cradle/expressions-app/README.md` | Blob `3dc39b594e56ff77096551809b9c457d8bfce272`. This vendored tree is the application source of record and the native build/entry route. |
| O:I `desktop/cradle/expressions-app/src/engine/fieldModel.ts` | Blob `b10cca471472874c24f3042e0bd6e6a2ddb07127`. Field/entity/composition, pure-force pins, layered formations, per-link state, target versus uniform ownership and resident continuity. |
| O:I `desktop/cradle/expressions-app/src/engine/GlyphSampler.ts` | Blob `b017a59160625cfdd3dbfbecda3eda0b031a803e`. Square glyph rasterisation, source sampling, measured depth, target textures and cache invalidation. |
| QL `crates/ql-core/src/lib.rs` | Blob `77669f38d10b77047d0fef45d0139a3bbed035d8`. Existing fold/site/pose, codon/I-Ching/Tarot/clock and structural shape-binding exports. |
| QL `crates/ql-mef/src/lib.rs` | Blob `1f872383e2912e2a7915db65775f0048b2bf98a7`. Current coordinate/profile, M3 state, scene, C′/Vāk and native owner modules. |

The source paths resolve in [QL-MEF](https://github.com/EpiLogos/QL-MEF) and [O:I](https://github.com/EpiLogos/O-I). Historical source and deep matrices remain in their recorded native repositories; follow their existing provenance routes rather than copying another source hierarchy into this tranche.

### 1.3 Extend the real stage model

The existing engine has a field carrying shared physics/colour/morph/cymatics, entities with layered formations or pure-force pins, entity sequences and composition orchestration. Sequence links can already carry shape, layer/source, placement, scale, colour and force variations. Position changes are uniforms; shape targets belong to partitions; explicit reset and particle-count change are distinct from ordinary scene changes.

Use this vocabulary as the procedural output. Preserve current world-unit conventions and target/local/world transform ordering. The inspected runtime has 64 formation slots and 64 pin slots per pass; these are rendering budgets, not an instruction to cap the authored world at those counts. Consume the current paging and partition owner with identity-preserving admission. Read the current capability contract rather than reintroducing historical ten-formation or thirty-two-entity limits.

The native application already owns stable document/scene/entity refs, subject bindings, versioned edit batches and recovery. Its current parameter manual operation removes automation. This tranche develops the richer inspectable driver/takeover/resume relation through that owner. A planned extension is recorded as such until the actual producer and consumer exist.

### 1.4 Preserve meaningful existing failures

The SDK's recovery amendments retain actual failures in which configuration/native target acknowledgements succeeded while held resident points did not move. Add state-to-observation tests at that precise receiving boundary, both paused and running. A new receipt is useful only when it identifies what actually applied.

The physical-music parent is already active. Its comments report native source, body and audio work and identify current shared writers. Read the latest #281 and #291 claims before adopting files. Preserve uncommitted work and its evidence. The parallel work below contributes to those running lines, rather than resetting their context or repeating completed discovery.

## 2. Settled architectural decisions

### 2.1 Identity, expressive scale and canonical place

Use three related references: native **subject**, source-qualified **locus**, concrete **occurrence**. A subject may have several manifestations, and each manifestation has an addressable role in its containing composition. A scene or full Expression can itself present a bounded subject. A force or sequence can embody a native relation or process.

Preserve the Paśu ontology's general entity-to-form meaning. Recover its exact original definitions and current native projection in the semantic packet; an existing narrow participation enum does not define the extent of the ontology. Extend references/participation through the appropriate owner rather than minting a second entity universe.

M3 resolves an expressive plan at the appropriate scale: participants, form, arrangement, transformations and temporal organisation. It consumes the actual native symbolic and contextual determination. Rule/profile choices retain their source and interpretation. The plan is performed as ordinary native scene/entity/parameter operations.

### 2.2 Unified state surface

Provide one address model and one coherent application interface for inspecting, subscribing to and changing the whole field, an Expression, a scene, selected occurrences, layers, forces, sequence links, parameters and driver contributions.

The interface joins the existing owners. It exposes configuration revision, native generations and effective observations; it does not relocate the audio buffer or particle field into an application database. Human controls and agent calls resolve the same target identities and operations.

Configuration edits remain atomic through the existing native application. A multi-owner live action adds preparation, scheduled application and acknowledgements for its participating consumers. Its observable status reports the real application boundary. The companion contract defines conflict, retry, cancellation and recovery.

### 2.3 Explicit control and authorship

Each controlled property exposes its authored basis, base driver, named modulation, any manual takeover and effective result. Manual takeover can be released back into the retained driver. Persistent authored edits and temporary performance gestures have distinct lifetimes. A global field setting, a scoped batch of local edits and a group multiplier are distinct operations with discoverable effects.

Procedural regeneration owns identified contributions. It revises those contributions while retaining user overrides and unrelated authored material. Removing a rule retires its contribution according to the recorded lifecycle, with edited material retained or deliberately detached as authored work.

### 2.4 Rules generate actual compositions

Tags and typed native relations select concrete targets. A versioned rule combines trigger, selector, conditions, source-bound recipe, timing and continuity to produce a prepared native operation batch. The available results are state changes, new constituents, new scenes, transitions and whole Expressions.

Use native bounded operations and capabilities. Extend the current composition/C′/Vāk and Expression trigger/profile offices where the required operation belongs. Keep the rule and its output inspectable, interruptible and reusable. The compiler/runtime rules in the companion specify stable contribution keys, target membership, ordering, invalidation, budgets and feedback.

### 2.5 Canonical and generated places share continuity

Atlas entry resolves exact coordinate/branch/face and occasion into inherited profile, subjects, relations and composition. Re-entering a place recovers the intended existing occurrence and work. Explicit instantiation creates another occurrence. A canonical locus can be foregrounded by location, arrangement, scene or disclosure changes within the same stage.

Profiles retain their actual inheritance semantics. At the inspected SDK basis, parents resolve in declared order with first-parent precedence per key and whole-key child replacement: `material` is not implicitly deep-merged. Generate a typed material diff or a complete validated replacement where required; do not erase an inherited body through an assumed merge.

### 2.6 Glyph folds use the existing material pipeline

Use a square local material domain carrying the glyph's rest samples, depth and layer identity. A source-driven crease/panel recipe deforms those samples into current local targets; the normal entity transform places them on the stage. Fold angles/progress run through the existing sequence/parameter control.

The profile selects glyph-mask material or a sheet carrier with glyph content. Preserve sample correspondence and the source's shape while folding. Re-sample on relevant source/topology/density changes, not as a substitute for each frame of deformation. M3 supplies its actual fold/state/pose law, and the #281 body owner consumes the prepared geometry and conditions through one paired interface.

### 2.7 The whole performed act is retained

Save the authored composition, recipe/source revisions, resolved contributions, explicit seeds, control bindings, interventions, scene flow and necessary timing/body state through the existing persistence owners. Reopen configuration, replay a recorded act and resume a physical checkpoint are explicitly selected restoration modes. The same distinction appears in the UI and agent read model.

## 3. Native offices and counterpart contracts

| Office | Responsibility in this tranche |
|---|---|
| O:I Expression application | Stable constituent addresses, validated state reads/changes, local authored composition, operation status, recovery and export. |
| Existing Expressions engine | Actual partition/target/force/colour/sequence state, continuous glyph deformation, observed stage and lifecycle. |
| Paśu/native subject owners | Entity identity, participation and relations; presentation occurrences bind to these native subjects. |
| QL Atlas/Ta-Onta | Source-qualified locus/profile/manifestation resolution, procedural plans and native capability composition. |
| QL M3/C kernel | Symbolic form, fold/pose transformation, codon/I-Ching/Tarot/clock determination and source-qualified entity-to-form operations. |
| #281 audio/physical owners | M1 carrier, M2 determination, real-time control, physical bodies, acoustic/visible observation and sample scheduling. |
| Existing C′/Vāk/Chronos | Composition and temporal relation among performances, scene sequences, native occasions and Return. |
| Existing Studio/Nara/Epii | Human/agent interaction with the same stage; precise selected target, source explanation, procedural authorship and interruption. |
| Existing Central/Expression/episode owners | Durable authorship, files, editions, recorded acts, continuation and qualified sharing. |

Ta-Onta's six organs stay operative throughout: Khora establishes entry/ground; Hen resolves form/profile; Pleroma supplies callable powers; Chronos relates time and continuation; Anima composes/acts; Aletheia receives outcomes and supports Return. Procedural performance develops their existing purposes.

### 3.1 Coordination with active instrument work

| Shared boundary | #296 contribution | Retained #281 counterpart |
|---|---|---|
| Parameter/state/command timing | Stable scopes, driver inspection, composed changes and consumer status | #287 owns actual DSP/device scheduling and audio parameter application. |
| Glyph-to-body | Rest samples, crease recipe, actual M3 deformation and body descriptor | #288 owns prepared physical solver/body state and sound/visible physical coupling. |
| Situated continuation | Atlas instance/locus and scoped procedural context | #289 owns current personal/neutral/shared receiving conditions. |
| Application interaction | State/rule/override/scene-flow Studio surfaces | #290 owns musical-mode/Jankó host layout; one agreed host writer integrates both. |
| Retained performance | Recipe/contribution/control/intervention payload and replay semantics | #292 owns joined performance persistence/score return; share its actual migration and event contract. |
| Independent proof | State, rule, manifestation, fold and continuity adversarial cases | #293 retains instrument proof; exchange fixtures and repeat the joined candidate. |
| Symbolic/music determination | Consume native form/context outputs and bind their expressive roles | #285/#286 retain musical/source determination; #291 retains decision-model/event scope. |

One writer owns each shared module or schema at a time. The two parents publish accepted producer/consumer revisions and resolve conflicts directly. A worker may build a separate component with a pinned contract immediately; it integrates against the real producer when available. A pending counterpart limits only its dependent receiving test, not the entire workstream.

## 4. Delivery order and developer context

### 4.1 Start together

The state-contract, semantic manifestation/Atlas, rule compiler, M3 glyph and independent-test packets begin together on disjoint files. Studio can develop from the current real native read path and receive the new scopes incrementally. Persistence design uses the actual #292 event/migration owner from the beginning.

At intake, record current branches, native seat/claim, consumed source/contract revisions, exact writable files, counterpart owner and failing acceptance cases. Reuse the established development seats/worktrees and build pool. Preserve the existing integrator's Git/build/install ownership. Additional subagents receive bounded source-complete packets, not only a summary of this Wayfinder.

Every worker demonstrates one source → exact identity → native operation → stage effect → detecting-test trace, then implements. Source changes refresh the dependent slice of context. Resume from versioned handoffs and unresolved cases rather than repeating a whole-system census.

### 4.2 First joined passage

Deliver a real scope read and granular edit into the existing scene; bind a subject's formation and force; apply one source-driven M3 fold through the actual sampler/target path; have one procedure create and then update a stable scene contribution; preserve a human override through re-evaluation and save/reopen.

This establishes the shared contract by actual operation. Expand across the full required scopes, symbolic families, controls and continuity rather than stopping at that passage.

### 4.3 Complete the stage

Finish complete entity-to-form participation, every required state/control scope, the rule lifecycle, canonical/derived/authored scene transitions, continuous glyph/fold treatments, complete Studio/agent workflows and retained procedural performance. Bind the full solar/clock/codon/Tarot/I-Ching composition through source-native operations and the current M1/M2 instrument.

Extend the existing deep matrices and source-to-experience records. Each obligation retains source, exact coordinate/subject, operation, counterpart, expected consequence and independent evidence. A failed source lookup remains a repair task and cannot be replaced by a parent description.

### 4.4 Deliver the usable candidate

Build and exercise the actual application under its current native route. Keep paused/running tests, context transitions, concurrency, source drift and recovery in the ordinary walkthrough. Leave the saved **Ta-Onta Procedural Stage Acceptance** project directly reachable in the tested candidate. Provide exact installed/running revisions, source basis, operation receipts, measurements and screen capture with actual produced audio.

The parent tracks component completion separately from integrated and human acceptance. Its return is the running editable stage, with the owner's interventions and canonical places intact.

## 5. Closure, evidence and continuation

All cases in the dispatch document and affected #281/#65 regressions are required. Tests must observe actual native consumers: target metadata, rendered displacement and acoustic output each establish their own part of the causal path.

The independent verifier exercises deliberate wrong-subject/wrong-prime, dropped constituent, duplicate generation, stale revision, manual-override loss, disconnected consumer, unbounded rule cycle and lossy-reopen cases. Preserve the original failure beside the repair and repeated affected activity.

The owner receives an immediately repeatable stage encounter. Human acceptance is recorded when given. Implementation changes land at native owners; the completed work and learned composition procedures return through the existing Ta-Onta/Factory/praxis paths. Keep #296 and #281 separately accountable for their full commissions while sharing their real integrated instrument.