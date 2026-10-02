# M′ physical music — reference research and acceptance

Parent: [QL-MEF #281](https://github.com/EpiLogos/QL-MEF/issues/281). Read the [Wayfinder](M-PRIME-PHYSICAL-MUSIC-WAYFINDER.md) and [curated work packets](M-PRIME-PHYSICAL-MUSIC-WORK-PACKETS.md). This document supplies concrete research assignments and test definitions. It is not a report that the vendor products or the owner's installed instrument have been exercised.

## 0. Primary reference catalogue

Reference routes checked for this planning publication on **2 October 2026**. Product pages describe published capabilities, not source-code internals or independently measured performance. At execution, pin the exact version/manual/platform and distinguish documentation, demonstration, code inspection and local experiment. Search-index snapshots and cached pages can show different release cuts; record the actual artifact used rather than merging them into a fictional product.

### Sound Particles — spatial sound and production workflow

Primary entry: [official Sound Particles overview](https://soundparticles.com/products/soundparticles/overview/), with [official product catalogue](https://www.soundparticles.com/products). The overview presents a standalone spatial audio workstation: sound instances are placed and moved in a 3D scene, received through virtual microphones and rendered. The product family also contains plugins. Treat the workstation and individual plugins as separate reference artifacts.

**Inspect:** particle/emitter and sound-source identity; clip/track association; movement versus audio modifiers; receiver/microphone controls; coordinate systems and units; multiple views; deterministic randomisation; transport/rendering; multichannel routing; effects/instrument hosting and project restoration. Trace how one user action changes the signal path and scene together.

**Use in our design:** an understandable stage of actual sounding objects, explicit receivers, visual spatial editing and professional routing/arrangement. Do not treat a moving emitter as proof of a vibrating material body or import the vendor's object/store model as our domain ontology.

### Shrapnel — Tech Audio's collision-driven instrument

Primary entry: [Tech Audio official projects](https://techaud.io/projects/) and [official site/release posts](https://techaud.io/). The current developer catalogue identifies Shrapnel as an instrument in which particle collisions produce sound. Follow its official product/manual/download links for the exact current build. Do not confuse it with similarly named audio products or assume a plugin format, solver or granular/resonator architecture from the name.

**Inspect:** emitter/particle lifetime, obstacles and contact, force and velocity controls, collision event extraction, sound/sample assignment, event timing, parameter modulation, random seeds, playback/host transport and saving. Demonstrate whether a physical state changes an excitation event, a continuing waveform, spatial reception, or several of these.

**Use in our design:** a direct, playable connection between physical interaction and sonic articulation. Translate contact into explicit native events/parameters with provenance, timing and controllable scope. A collision-triggered sample is a useful module; it does not replace the required resonant physical body.

### Physical Audio Tetrad — oscillator, modulation and physical resonators

Primary entry: [official Tetrad product page](https://physicalaudio.co.uk/products/tetrad/), following its current documentation/download route. The published design combines oscillators, matrix routing and modulation/granular processing with modelled plate resonators. It exposes expressive controls and source-to-destination modulation.

**Inspect:** excitation versus resonator lifetime, playable polyphony, gate/ADSR versus physical decay, matrix routing, per-note versus global modulation, tuning controls, spatial/pickup control, safety/headroom and preset recall. Verify current implementation behaviour instead of relying on a remembered older signal chain.

**Use in our design:** a concrete comparison for M1 carrier → M2 processing/material response and a professional modulation surface. Retain our native source and tuning authority; adopting a useful interaction does not adopt another musical ontology.

### Anukari — interactive physical bodies and visible modulation

Primary entry: [official Anukari site](https://anukari.com/). The developer describes an interactive 3D mass/spring instrument/effects system with excitation, physical interconnection and visible manipulation; the page also describes modulation, MPE and microtuning facilities.

**Inspect:** construction versus performance, physical connectivity, excitation/pickup placement, parameter and force modulation, CPU/GPU scheduling, feedback stability, polyphony, external audio, automation and patch persistence. Inspect publicly available implementation material only where actually provided.

**Use in our design:** the most direct additional comparison for constructing and playing the same visible physical object. It is a reference, not an instruction to replace our accepted continuous/GPU engine with a mass/spring-only system.

### Reference boundary

These products are research subjects, not mandatory dependencies or purchases. Use public manuals/source and authorised trials/installed copies. Do not redistribute vendor code, proprietary presets, sample libraries, artwork or internal documentation. Record licensing and supported integration boundaries before any reuse. Learn mechanisms and interactions, then implement the source-defined Epi instrument through current native owners.

## 1. Required research return

The R packet returns a source-bearing architecture comparison, not a general review. For each relevant claim record the product/version/platform; primary document or actual observation; the specific operation; accessible implementation evidence; the inferred mechanism explicitly identified as inference; its relevance to our native contract; and a reproduction/negative test.

Cover the following questions with concrete results:

1. What is the source of energy/signal: oscillator, recorded sound, external audio, collision impulse or other excitation? What exactly does the simulated state control?
2. Which timebase schedules notes, collisions, modulation, physics, display and offline rendering? What is merely visually synchronised?
3. How are geometry, material, constraints, exciters and pickups represented? Which changes require preparation rather than realtime mutation?
4. What are per-note, per-body, per-instrument, per-context and presentation-only parameters? How are units, ranges, smoothing and automation exposed?
5. How does a user select, connect, modulate, record and recover the instrument? Which actions are direct and which require another editor?
6. What survives save/reopen and host replay: seeds, audio assets, body state, tuning, routing and automation? What cannot be reproduced from a preset alone?
7. Which implementation techniques are compatible with our current C/Rust/C++/GPU ownership, source fidelity, realtime budgets and maintainability?

Each chosen approach includes one rejected alternative and the measured or architectural reason. Start with the existing implementation and the smallest native extension. A convincing external demo does not establish that the same technique meets our latency, source or state requirements.

### Bounded experiments before large implementation commitments

| Experiment | Required causal trace | Returned result |
|---|---|---|
| Moving source | source/trajectory/receiver change → predicted spatial audio change | Controlled trajectories, receiver configuration, audio capture and operation/readback; distinguish receiver movement from camera-only movement. |
| Collision articulation | reproducible physical contact → timestamped native excitation/gesture → sound | Seed/event trace, collision timing, response law, missing/duplicated-event tests and an edited replay. |
| Oscillator into body | native M1 carrier/M2 determination → actual resonator → acoustic observation | Impulse or stable-tone response, modal/decay measurements, changes to excitation/observation points and source/body provenance. |
| Shared physical cause | one body state → visible displacement and acoustic output | Matched body/state/time IDs and expected numerical relations; severing one binding must fail. |
| Modulation and persistence | gesture/sequence → typed modulation → actual effect → saved and reopened performance | Repeated effective parameter values, audio/physical consequences, undo and replay. |

Prototype code lives under the current owner's test/experiment conventions. A prototype is promoted only after its native integration, source relations and regression tests are explicit. Remove abandoned duplicate runtime paths after preserving useful experiment evidence.

## 2. Source and context fidelity at implementation time

Carry source → exact coordinate → native operation → intended user consequence → test through the existing capability matrices and ledger. Use current matrix IDs rather than creating a parallel registry from the local case labels below. Full source coverage and evidence scope remain independently inspectable.

The latest owner rulings resolve specific historic conflicts: M1 oscillator source, M2 tuning/modulation and Vimarśā shared bus; live map precedence over disagreeing M2 tables; observer-place hour ruler; tunable magnitudes with standing; distinct M3 geometric inscription and aperture readings. Preserve these beside the original deep instrument sources. A failed exact lookup is not a valid parent summary.

For every delegated context packet, retain the exact source/contract head and delivered sections. A fresh implementer must correctly identify its branch and expected effect before coding. A later accepted source change invalidates dependent work explicitly. Resolved, delivered, invoked and tested are different statuses in the existing tools.

**No new whole-system census at each restart.** Resume from the packet's versioned source/contract/defect basis, load changes and proceed. Equally, do not resume from a compressed summary that has lost M′ descendants, tuning distinctions or required physical behaviour.

## 3. Discriminating acceptance cases

These case labels are navigation within #281, not new product capability identifiers. Map them to the existing source-obligation and evidence system. Preserve original G1–G10 and full source requirements.

| Case | Actual operation and pass rule | Wrong implementation the test must detect |
|---|---|---|
| T01 — complete source field | Reconcile complete M1′/M2′ sources and required M3/M4 joins; every required behaviour has exact source/coordinate, native operation, accessible action and appropriate evidence. | All current rows pass but Jankó, deep descendants, a professional operation or physical body was omitted from the inventory. |
| T02 — coordinate round trip | Parse/resolve/disclose/dispatch/save/reopen/replay exact direct/prime addresses and deep paths using QL grammar. Test aliases, the position-4 dot, reflective slash/frame and source revisions. | Prime collision, changed branch, unconditional parent fallback or a generated shallow tree. |
| T03 — formal musical identity | Exercise both twelve-position bases, A/B/C and D cardinalities, twelve lens anchors, seven CF cases, source-defined reductions and explicit tuning realisations. | A second music grammar, octatonic inferred from array size, physical modes equated to MEF cells, or silent equal-temperament coercion. |
| T04 — Jankó performance | Six rows/two whole-tone families/three touch-points, semitone adjacency and all twelve transpositions; play chords with concurrent repeated controls, sustain, release and expression. | Wrong note map, premature release, stuck voice, key events captured while editing text, or an isolated browser piano. |
| T05 — actual ownership | Trace native M1 carrier through M2 MEF/Vimarśā and bounded relation plan to rendered output; verify exact tuning and nodal/voice roles. | M2 independent pitch source, bypassed producer, renderer-local correspondence or nodal quartet played as four extra notes. |
| T06 — physical body | Excite a source-derived geometry with stated material/constraints; observe sound and visible displacement from the same physical state; change one physical determinant. | Unrelated oscillator and shader, unbound mesh, label-only material, duplicated CPU/GPU integrators or hidden energy injection. |
| T07 — differentiated controls | Change a genuine M2 lens/material/tuning/sky determinant and compare the source-predicted target/output; separately change only an M3 display aperture or presentation view under fixed sounding determinants. | Treating every lens as sound-invariant or every display action as musical mutation. |
| T08 — typed modulation | Assign/edit/remove a per-note and a global route; expose baseline/effective values; exercise smoothing, bounds, timing and undo. | Unit mismatch, global overwrite from one note, stale source result, immediate unsafe coefficient jump or a control with no native effect. |
| T09 — context and receiving | Play the same source-defined instrument in neutral/world and personal contexts, then return; exercise a permitted shared projection. | Shared mutable private state, mandatory personal profile for ordinary synthesis, duplicate engine per view or a camera mistaken for a listener. |
| T10 — whole composition | Play→record/overdub→edit→layer→route→automate→loop/seek→save→close/reopen→continue→export through the production app. | Notes persist but body/forces/tuning/context/automation disappear; a second project store; a generated file bypasses real UI/runtime. |
| T11 — score and return | Use retained event through native M3 score/transcription and M4 episode, then re-inscribe/replay through the actual M2/M1 owners. | New guessed codon classifier, independently generated substitute performance, lost timing/source or a second journey clock. |
| T12 — realtime and lifecycle | Execute the numerical protocol below while changing views and controls; lose/reconnect the device, change sample rate and restore the project. | A buffered audition path called low-latency, audio depending on UI frames, unbounded queue, dropped input, stuck tails or false success on a disconnected device. |
| T13 — independent context/proof | Fresh agent retrieves and applies deep M′/Jankó/audio/physical requirements through ordinary tooling; independent verifier reruns the installed candidate and controlled mutations. | A hand-curated prompt is the sole memory of the instruments; a source-string test passes after the producer is disconnected. |
| T14 — owner-visible delivery | Launch exact tested build through current app route; open saved acceptance project, record UI plus actual output and provide direct entry/controls. | Screenshot, dev-only unreachable route, mock gateway, stale installed build or simulated human acceptance. |

Test views, tabs, full-screen and deep/personal transitions for state continuity and duplicate subscriptions/instances. Test failure surfaces explicitly: missing source/provider, stale plan, invalid body, inaccessible audio device, unsupported export and rejected authority must be distinguishable from empty successful output. Preserve all affected current cosmic/personal, privacy and source-read-through regressions.

## 4. Numerical and material proof protocol

The following are default engineering acceptance targets commissioned in #281, not claims of existing measurements. Apply stricter source thresholds. Record the actual workload before comparing implementations; do not silently shrink it to obtain a pass.

### Audio and scheduling

Test the installed Mac native audio path at **48 kHz, 128 or 256 frames per callback**. Record OS, hardware, audio interface/device, backend, build IDs, sample rate, buffer, channels, voice count, body/resonator complexity, modulation/routes, graphics load and session duration. Test the full representative performance, not a lone diagnostic oscillator.

Measure input-to-output latency using a documented trigger/timestamp and output-capture or loopback method. State what the measurement includes and the measurement uncertainty. Report median, p95 and p99; **p95 must be at most 30 ms**. Configured buffer duration is not a measured latency. Keep physical/device evidence separate from an in-memory render.

Run a **15-minute** representative play/record/edit/automation session with zero audio underruns, stuck notes or unintended clipping. Report callback deadlines, queue high-water marks, dropped/late events, CPU and memory; log explicit overload behaviour separately from the passing workload. Exercise view changes and nontrivial physical/material edits during the run.

For controlled stable reference tones, estimate output frequency against the intended native source and require error within **1 cent**; this is not a requirement to force an intentionally inharmonic resonator or modulated pitch onto equal temperament. Offline deterministic event scheduling must agree within **one sample**. Preserve exact-ratio/source metadata even where device/export encodings approximate it.

Test voice identity across repeated touch-points, sustain and release, bounded voice stealing, state restore, sample-rate changes and device loss/reconnect. Ringing bodies and effect tails follow their model rather than being indiscriminately cut on key-up. All-notes-off/panic stops active excitation safely and exposes its tail-reset policy.

### Physical causality and stability

Choose source-required reference bodies and record geometry/material/constraint/exciter/pickup revisions and numerical method. Establish analytic or independently computed expected responses where available. For more complex bodies, retain convergence or higher-accuracy reference comparisons with declared error norms, tolerances and operating ranges.

Measure impulse response, modal frequencies/decays or equivalent physical quantities, zero-input stability, finite state, bounds and transition continuity. Test excitation/observation location changes, material/boundary changes and a source-defined form transformation. A deliberately decoupled visible or audio branch must fail the intended common-state comparison.

Audio and display have different rates. Compare their shared physical state and appropriate observation functions, not raw equality of an audio sample and a screen pixel. Record frame/sample mapping and interpolation; graphics slowdown must not corrupt the audio state. Distinguish rendering quality from acoustic numerical accuracy.

### Determinism, contexts and replay

Freeze source refs, body plan, tuning, events, random seeds and rendering mode. Repeat offline and controlled realtime runs with expected exact event identity and stated numerical tolerance. A realtime GPU display need not be bit-identical to a replay to preserve the native physical/musical result; its actual checkpoint/reconstruction guarantees must be explicit and tested.

Repeat the same performance with independently grounded contexts. Verify immutable source definitions, context-specific state and permitted receiving differences. A historical sky is a dated computation/source, not a fake live observation. Personal interpretive meaning and measured physical/audio output remain separate evidence.

### UI and usability

Automate the full user journey in the production host where possible, then verify native installed boot/device/state. Browser renderer tests are not native carrier/audio proof. Count required interactions and record timing for the ordinary play/edit/save/reopen path so later changes can be compared. Keep real human review for musical feel, clarity and desired experience; the verifier does not manufacture it.

## 5. Evidence, integration and handover

Use existing #65/QL matrices, receipt conventions and native evidence refs. Each result names source/ruling revisions, candidate/build/install/running versions, test input, operation, expected output, observed output, numerical method/tolerance, device/provider/context, actor/verifier identity, logs/capture/artifacts and remaining exact defect. Keep original failed trials and assistance beside successful repeats.

A missing local device or inaccessible provider is an unexecuted material test, never a synthetic pass. Implement the production work and all executable checks, then obtain that exact material test through the authorised local harness. The integrator owns this progression; the owner does not coordinate a relay of subagents.

The final non-private project is **M1′–M2′ Instrument Acceptance**. It contains a playable Jankó/native harmonic performance, at least one source-defined resonating body, meaningful M2 modulation/tuning/material operations, M3 form/sequence, declared M4 receiving context, layered/editable recorded material, automation/routing and source provenance. Its ordinary app reopen continues the same work. Additional source-required branches remain independently covered; this project is not the scope ceiling.

Deliver the exact running candidate, immediate app/menu/deep-link or reachable entry, saved project, rendered outputs, UI-and-audio demonstration and independent evidence. The owner can play and edit it at handover. Keep #281 open until its complete requirements and live delivery gates pass. Publication of these documents, green source checks or a set of plausible subagent reports cannot close the feature.
