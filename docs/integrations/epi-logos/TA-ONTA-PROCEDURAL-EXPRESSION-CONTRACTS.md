# Ta-Onta procedural Expression — implementation contracts

**Parent:** [#296](https://github.com/EpiLogos/QL-MEF/issues/296). Read the [Wayfinder](TA-ONTA-PROCEDURAL-EXPRESSION-WAYFINDER.md) and [dispatch/acceptance](TA-ONTA-PROCEDURAL-EXPRESSION-DISPATCH.md).  
**Purpose:** specify the operative details required by the accepted procedural-stage design. Contract labels P1–P6 below identify work agreements, not new Bimba coordinates or declarations of already-exported APIs. Bind each agreement to the current owner types, commands and counterpart fixtures before implementation.

## 0. P1 — identity, manifestation and scope

### 0.1 Three references in one relation

Every meaningful stage occurrence resolves these fields through current native owners:

| Relation | Required information |
|---|---|
| Subject | Stable native subject ref/owner; Paśu participation/type where applicable; source/reading refs and revisions; relevant typed relations and available native operations. |
| Locus | Exact Bimba/other native location, branch/face, Context Frame/occasion where applicable, source revision and inherited profile lineage. |
| Occurrence | Stable Expression/scene/constituent ref, role, containment, material/control bindings, authored/generation origin and local revision. |

A subject can occur in several scenes and roles; an occurrence can contain several contributing source subjects with separately qualified roles. Use an explicit principal subject where one is required by an existing API. Preserve each contributing binding rather than coercing several subjects into a fabricated identity.

Extend the current `SubjectBinding`, native scene/body references and occurrence index. Scene and Expression subjects, entity layers, forces, sequence links and control routes become inspectable through the same binding relation. Where a force is currently a value object within an entity, its address can be the stable entity occurrence plus a validated component role; mint an independent native occurrence only when its independent lifecycle requires one.

A reference remains stable under list reordering, scene movement, renaming, selection and regenerated presentation. Explicit cloning creates a new occurrence with the same subject binding and a new origin relation. Removing an occurrence leaves its native subject intact and updates any local references according to the validated edit.

### 0.2 Paśu-to-manifestation resolution

Input: subject(s), actual properties/relations, intended act, current locus/occasion, requested expressive scale, source and profile revisions.

Output: a typed expressive plan identifying the participating subjects, their occurrence roles, composition structure, native form/behaviour operations, material recipes, timing, source basis and explanation of the selected relation.

Required roles include formation/glyph/body; force/constraint/modulation; movement/sequence; scene; full Expression or composed scene collection. These are compositional roles, not a compulsory one-to-one enumeration of ontological species. A planet can participate as formation, motion and modulation together; a transformation can have a glyph disclosure while its operative manifestation is a sequence.

Resolve determined native relations first; apply inherited source-qualified recipes; then preserve authored variation and current encounter choices. Bind M3's entity-to-form determination at its existing owner. Source-derived, authored and interpretive choices remain individually inspectable. Develop outstanding source-grounded M3 determination within the assigned native packet rather than treating the current glyph-selection API as the whole task.

### 0.3 One address model for all operations

Read, subscribe and change use the same validated scope selectors:

- current stage/field and its active world/occasion;
- Expression or named collection of existing Expressions;
- scene or named scene set;
- group/entity occurrence;
- layer, force, sequence or sequence link;
- property, driver or modulation contribution;
- all permitted occurrences of a subject, coordinate or typed relation;
- bounded tag/facet query over a declared containing scope.

A scope is an addressable target set, not a promise that every property exists at every level. Capability discovery returns the applicable operations, property types, units, limits, owner and read/write standing for the resolved targets. Unknown targets and an intentionally empty set have distinguishable outcomes.

At preparation, a selector resolves to a stable ordered target set and basis revisions. This is the default for one-shot edits. A sustained selector explicitly subscribes to membership changes and records join/leave policy and observed membership events. A tagged entity appearing later does not silently join an already-recorded one-shot edit.

Deep coordinates preserve prime, branch, the position-4 nesting separator, reflective slash and Context Frame meaning through selection and persistence. Use existing canonical aliases and typed parsers. Keep source-only coordinates addressable through their native content owner even when a numerical M engine binding is inapplicable.

## 1. P2 — state read, observation and coordinated mutation

### 1.1 Coherent bounded state reading

A scope read returns:

| Layer | Required disclosure |
|---|---|
| Authored state | Document/profile/scene/constituent revisions; base values; source bindings; local overrides and their origin. |
| Active composition | Applied recipe versions, generated contributions, selected drivers, sequences/automation/modulators, pending changes and transition policy. |
| Effective observation | Current value/pose/sequence position, native generation, sample/simulation cursor as applicable, observation time, availability and causal operation reference. |
| Change affordances | Exact address, declared type/units/range, owner, admitted operations, actual authority route and relevant limits. |

Return a consistent revision/generation basis. A joined read names the participant generations and any unavailable or lagging participant; it does not fabricate a simultaneous physical snapshot by combining unrelated timestamps. Capture of high-rate state is an explicit bounded operation of the native runtime.

Subscriptions use existing native event transport with scoped deltas, cursor ordering and resynchronisation. On a cursor gap, deliver the current bounded snapshot and its new cursor before further deltas. Coalesce superseded high-rate observations; preserve committed authored operations and their ordering. Unsubscribe, scene closure and destroyed views release their resources.

A source-followed value can change without editing the authored document. Expose that as runtime/driver observation. Simulation steps do not manufacture document-edit history or Agent Activity.

### 1.2 Three distinct kinds of global change

A global field edit changes a real shared field property. A multi-target edit changes the listed local properties. A group control introduces a named modifier over a target set while retaining local base values. Capability discovery and Studio distinguish these meanings.

Existing force emitters act over the shared field according to their established falloff. Selecting a set of emitters restricts which emitters are edited, not automatically which particles feel them. Implement a declared influence mask through the physical/engine owner when a recipe calls for restricted recipients, and disclose that mask separately from edit scope.

### 1.3 Command envelope and preparation

Extend the current application request/receipt model with the minimum fields needed for:

```text
operation identity and actor/native authority context
scope selector + resolved target refs
expected document/component/native/source generations
ordered typed changes or native Action refs
timing domain + requested application boundary
continuity and conflict policy
recipe/performance/cause correlation
```

Preparation validates all targets, capabilities, units, ranges, source versions, cardinality limits and authority requirements; computes dependent material/body preparation off the live callback; and returns the proposed delta plus participant versions. No preparation result reports that a live change has applied.

Retain the existing exact document compare-and-swap semantics. Finer-grained concurrency is an explicit versioned extension: disjoint field edits may be rebased only after checking their recorded read/write sets. A stale whole-document replacement is never an implicit granular merge. Human uncommitted editor text survives an incoming revision and offers an explicit conflict resolution.

### 1.4 Application and acknowledgement

For ordinary document edits, validate the complete batch before mutation and emit the existing atomic edit receipt. For coupled scene/native operations, prepare all required participants, publish a common scheduled operation identity and apply at each participant's declared boundary. Retain requested and actual boundaries.

Expose the operation lifecycle: prepared, accepted/scheduled, applying, applied, rejected, cancelled or interrupted/failed. A final applied result requires the required consumer acknowledgements and identifies the native generations actually observed. A document ACK with an unchanged physical consumer remains pending or failed at that consumer, with its precise cause.

Operations within one owner's transaction preserve that owner's atomicity. Cross-owner coordination records the actual application state of every participant and a recovery policy. External Actions with irreversible effects retain their native Action lifecycle and are composed at an explicit boundary, rather than represented as a reversible document edit.

Late input uses a declared policy appropriate to its class: reject/reschedule a planned scene transition, coalesce replaceable controls, and prioritize release/panic through the real audio owner. Use #287's actual scheduling and late-event guarantees; preserve note-release behaviour under future automation.

### 1.5 Retry, cancellation and failure recovery

The native operation ID and payload fingerprint make retries idempotent within the declared durable retention horizon. Repeating an ID with a different payload is a conflict. A retry after uncertain delivery first inspects the operation and current native generations; it does not repeat a cast, scene insertion or external Action blindly.

Cancel a prepared/scheduled operation before its admitted boundary. Interrupt a continuing sequence at its current observed state, record the interruption and apply the declared stop/hold/release policy. Reversal after application is a new attributable operation evaluated against current state. Preserve the original history.

Recovery retains accepted intent, observed participant acknowledgements and unresolved operations. Reconnection inspects owners, reconciles generations and resumes only the admitted continuation. A source revision change invalidates dependent preparations explicitly. The stage stays usable on its current admitted state while replacement material is prepared.

## 2. P3 — drivers, procedural rules and human authorship

### 2.1 Driver contract

Every controllable property has a declared type, units, domain, scope, owner, rate class and value-combination rule. It exposes:

- authored base/default and source/profile lineage;
- selected base driver: manual value, followed native value, sequence or automation;
- named modulation contributions, with source, mapping, amount, bounds, order and lifetime;
- manual takeover and the retained driver to resume;
- effective commanded value and actual observed value/generation.

A property has one selected base driver. Multiple sources combine through explicitly named modifiers or a defined mixing operator. For an admitted scalar chain, an example combination is `mapped base → ordered scale/offset/nonlinear modifiers → validated domain policy → native smoothing`. The chosen order is part of the recipe and record. Quaternion, colour, transform, enum and physical coefficient properties use their own declared operations; they are not blindly added as scalars.

### 2.2 Takeover, release and regeneration

Default manual takeover controls the effective target value and records the dormant driver/modulator configuration. Release resumes that configuration through the property's transition policy, at the current performance position. Studio exposes an explicit alternate operation for editing the base while modulation remains active.

A momentary gesture ends at its declared lifetime; a persistent authored override survives regeneration and save. Automation recording uses the same real parameter operation and stores its target/time/units. Deleting a driver or source leaves an inspectable unresolved binding or an explicitly chosen retained value; it never silently substitutes an unrelated source.

Procedural updates replace their own generated basis. Authored overrides are reapplied to the same stable targets. If a generated target disappears, retain edited material as a detached authored contribution or present a typed conflict requiring deliberate removal. Record the chosen policy in the procedure so replay and regeneration agree.

### 2.3 Rule and recipe contract

A procedure contains its stable identity/version, source/profile lineage, subject/locus binding, trigger, selector, typed conditions, recipe parameters, permitted operations, timing, budgets, owned contribution keys and failure/continuation policy.

Triggers distinguish edge events from sustained conditions. Supported event inputs include native state changes, admitted musical/physical gestures, source changes, scene entry/leave, sequence transition and explicit human/Agent invocation. Follow existing Action authority and scene trigger validation. Compile recipes into current Expression/native operations; implement missing generic operations at the owning SDK rather than carrying arbitrary script bodies in a scene document.

Tags carry origin and scope: native source facet, authored annotation or generated/proposed reading. Tag matches select candidates. A recipe that requires a native relation validates that relation and its source revision before constructing the corresponding arrangement or behaviour.

### 2.4 Stable generated contributions

A contribution key derives from procedure identity, output slot, bound subject(s) and explicit occurrence/instance identity. The rule version is retained as provenance and an update basis rather than automatically creating a new occurrence on every revision. Explicit instance creation supplies a fresh occurrence key.

Each output records its exact owned constituents/properties. Re-evaluation diffs against the previous generated basis and current authored overlays. Creation, update, retirement and detachment are distinct operations. Material edits use validated typed changes; whole-key profile replacement follows the actual profile contract.

Removing one procedural contribution leaves unrelated local work and native subjects intact. Two procedures writing the same property require a declared modifier composition, priority or conflict decision. Arrival order is not a semantic arbitration rule. Independent changes with compatible read/write sets can proceed concurrently.

### 2.5 Live execution and bounded feedback

Resolve source, symbolic determination and expensive material preparation before the corresponding live operation. The running procedure consumes admitted state/events and prepared capability bindings. It can revise a plan incrementally when its dependencies change.

Maintain cause/ancestor IDs through generated events. Configure maximum rule evaluations, generated operations, expansion depth, active instances and queue length per admitted interval. Detect non-progress cycles; stop or coalesce the affected procedure with an inspectable outcome while the rest of the stage continues. Intentional musical feedback uses its native delayed/bounded operation and physical stability policy.

Random choices use explicit seeded generators with retained seed, algorithm/version and occurrence identity. Variable target ordering is canonicalised or recorded. Pause, resume, seek and cancellation retain the current rule/sequence position. Native snapshots replace continuously polling the full Bimba graph; graph access stays out of audio/render callbacks.

### 2.6 Procedures across media

C′/Vāk sequence, parallelism, sustained participation and nested composition express relations through the existing owners. One passage can control glyph changes, spatial movement, force modulation and musical events with named time mappings. A performed duration, celestial condition, M3 inscription transition and display frame remain distinct determinants linked by explicit mappings.

Agents author/select/revise procedures through Ta-Onta; the prepared runtime performs their bounded operations. Expose the same operations through human controls, native CLI/SDK and relevant installed Skills/Methods. Return the actual native operation result and evidence through the existing Action/Activity/praxis system.

## 3. P4 — Atlas places and scene continuity

### 3.1 Entry and composition

Atlas resolves `exact locus + occasion/source revision + inherited profile + authored variant + current overlay` into the subjects and material composition to inhabit. Reuse current `CoordinateExpressionBinding` and profile resolution. Preserve full parents-first/first-parent-per-key semantics and whole-key child replacement at the consumed revision.

A canonical place is a source-qualified composition basis. Its active occurrence can be changed, revisited, forked or developed into a passage. Its subject and coordinate remain stable. The same source can have several purposeful presentations; changing focus or adopting a profile does not relabel unrelated resident entities.

### 3.2 Transition operation

An admitted transition specifies source/destination occurrence, retained subjects, material additions/removals, desired spatial/scene changes, timing, current context and continuity policy.

Default continuity retains the current stage instance, resident particle state for retained partitions, native instrument instance, applicable clocks and subject bindings. Location changes use the existing transform/uniform path. New bodies use the current partition admission and prepared targets. Retired bodies and audio voices follow their explicit release/tail policy.

Distinguish focus/reframing, active scene change, domain-state operation and full reset. The UI and operation record reveal which act occurred. Explicit reset or particle-count changes retain their existing engine meaning. The chosen musical/physical operation can deliberately change the event; mere navigation preserves it according to the source-defined invariant.

### 3.3 Background and re-entry policy

For each continued scene/instance, select continue, pause/hold or checkpoint-and-release. Record the policy and actual cursor. Re-entry resumes from that policy. Preserve the canonical default/source definition separately from the person's active occurrence.

Switching ordinary Expressions, Technè and M0/M5 depth uses the existing host/surface mechanics and selection references. Subscribe views to the existing instance; dispose view resources without disposing the native subject or running instrument. Back/forward and bookmarked locus entry recover the intended instance and current source qualification.

### 3.4 Context

Bind shared/neutral/personal context through #289's current receiving owner. Source, place, occasion and identity retain their actual revision and disclosure. A context overlay changes only its declared occurrence/receiving contribution. Saved or shared portable work includes permitted references/cues; protected native state follows its existing private recovery path.

## 4. P5 — M3 form, glyph material and fold sequence

### 4.1 Material preparation

Extend the current glyph/source sampler. For every point retain a stable sample ID, source/layer identity, square-domain rest coordinates `(u,v)`, rest depth `w`, density/coverage and the relevant surface/depth metadata. Define the conversion between normalised material coordinates, the existing stage units and the physical body's metric scale explicitly.

Prepare a crease/panel recipe with identity/revision, material treatment, panel membership/connectivity, crease axes, fold-angle orientation, rest transform, source M3 bindings and admitted constraints. The profile chooses glyph-mask occupancy or a full sheet carrier with the glyph's markings/density. Preserve image/ASCII sources, volumetric glyph depth and the existing layered body.

Sampling and topology preparation depend on source shape, material treatment, topology, density/allocation and relevant volume settings. Entity position, camera, fold progress and ordinary angle updates do not trigger a source re-rasterisation. Cache keys include the exact dependencies; invalidate only the affected preparation. Retain a deterministic mapping across compatible updates and explicitly map new samples when topology genuinely changes.

### 4.2 Deformation and target ownership

For a point with rest coordinate `r_i`, evaluate the admitted fold deformation `p_i(t) = F(r_i; crease_state(t))`. Panel transforms are composed along the actual crease structure; their order and hinge orientation belong to the recipe. Use the appropriate constrained solver for admitted coupled crease systems. Tests establish the intended constraints for each supported source motif.

The existing entity sequence supplies hold, transition, easing, progress and interruption. The fold evaluator determines the corresponding geometry at that progress. Applying a fold to a current glyph preserves its sampled content and layer correspondence. Cross-morphing to a different glyph is a distinct source-shape transition with its own correspondence/continuity recipe.

Deformation produces local targets/body geometry; existing entity placement and extent transforms are applied once. Preserve thickness and orientation through the local transform. Expose commanded crease state separately from actual physical deformation where the body is simulated, so a lagging consumer cannot be disguised as an applied pose.

### 4.3 M3 determination and the complete symbolic composition

Consume the existing `FoldState`/`FoldMotif`/`FoldGeometry`, site state, M3 command/receipt and current inscription owners. Carry stable process identity and qualified codon/hexagram/Tarot readings through each transformation. A line change, matrix operation, pose transition and clock aperture have their own native effects and invariants.

Recover and implement source-qualified entity-to-form operations at M3's owner, then select the expressive scale: motif, relation/force arrangement, movement or multi-scene composition. Use full deep source matrices and accepted rulings for the clock, fold sites, rotational poses, Tarot/codon/I-Ching relations and M2 solar input. A handpicked glyph demo does not exhaust that field.

An ordinary fold gesture addresses geometry. A declared receptive operation can submit its measured/selected state to M3, with the native result and source basis retained. The recipe states which direction is active; feedback uses a named causal boundary so the same fold does not repeatedly retrigger itself.

### 4.4 Physical-musical join with #288

Publish one paired body description and generation contract: source/recipe revision, sample/topology identity, units, deformed geometry or control state, material/constraints, excitation/observation bindings and timing. #288 owns physical preparation and numerical body evolution; the procedural glyph packet owns source-to-deformation/control production and its stage integration.

Sound and visible physical response read the relevant shared body state. The selected physical solver and GPU stage have one numerical owner for each evolving variable. #287 schedules the admitted excitation/control; #286 supplies actual M2 material/tuning determination. Ordinary non-sounding glyph procedures remain operable through the same material stage while the paired audio path is integrated.

## 5. P6 — recording, restoration and the operating surface

### 5.1 Retained composition

Through the existing Expression/Scene/Edition and #292 performance/episode owners, retain:

- authored scene/entity/layer/force/sequence configuration and bindings;
- procedure/profile/recipe definitions by exact revision and admitted source references;
- stable contribution identities and current generated basis;
- resolved target sets, seeds, rule membership events and operation ordering;
- base drivers/modulators and persistent/manual intervention lifetimes;
- native symbolic operations and their qualified source/results;
- context/occasion references and privacy standing;
- timing mappings, current sequence/transition cursors and admitted physical checkpoints.

The record must support explanation of why this constituent appeared, what changed it and which human intervention altered it. Extend current storage/codec/migration contracts with backwards-compatible reads and explicit version changes. Keep source identity native and generated state derived.

### 5.2 Three restoration operations

**Open configuration:** reconstruct the authored/procedural configuration, explicitly starting its runtime under the selected entry policy.

**Replay recorded act:** consume retained actual operations, timing, sources/seeds and admitted snapshots through the same native instrument. Exact discrete identity and event ordering are required. Continuous output uses the agreed numerical tolerances and runtime/platform basis.

**Resume checkpoint:** restore the acknowledged compatible runtime state and cursors at their native owners. Verify body/source/version compatibility before resuming. If a checkpoint cannot be resumed, offer a clearly identified configuration-open or recorded replay operation; never label that fallback a restored physical state.

Source drift is a deliberate re-evaluation of the saved composition, with a diff and retained original basis. Crash recovery inspects uncertain operations rather than blindly replaying them. Central file CAS conflicts preserve the dirty draft and owner response.

### 5.3 Studio and agent interaction

Within the existing Stage/Studio, expose whole/scene/selection/component scopes, subject/locus/occurrence breadcrumb, base/effective values, active drivers and procedure contributions. A person can inspect a force without making it a visible glyph, inspect a scene as a subject, or select all manifestations of one native entity.

Support rule/recipe selection and editing, previewed target sets and deltas, live apply, pause/interrupt, takeover/release, author override/detach, scene insert/reorder/transition and save/replay. Direct manipulation and numeric controls call the same native operations. Maintain keyboard accessibility, pointer selection, viewport stability and uncommitted editor text during incoming updates.

Ta-Onta/Nara/Epii receive the bounded current scope, actual effect/driver state and available operations through existing context tools. A fresh agent must discover and perform these operations without DOM scraping or a specially hand-curated copy of the entire scene. Updates return to the person's existing scene and selection unless the commissioned operation intentionally navigates.

### 5.4 Paired conformance

For each contract P1–P6, publish current native type/operation references, version, producer and consumer, positive/negative fixtures, ownership, timing/limits and actual test command. The dispatch document names counterpart tickets. Consumers pin the contract they exercised; adapters disclose mismatched versions. Integrators move both sides together and retain the original failed trial beside its repaired repeat.