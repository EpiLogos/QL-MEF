# L5 Technē T9 — Integration Ledger

**Standing:** implementation record of QL-MEF #220 (the joined-convergence lane), 2026-09-17.
**Authority:** `docs/L5-TECHNE-DUAL-READING-LOCK.md`, `docs/L5-TECHNE-TB0-CONNECTIVE-BASE.md`,
`docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md`,
`docs/L5-TECHNE-INTEGRATED-DEVELOPMENT-WAYFINDER.md` (PR #221 branch), issue receipts on #212–#219.
**Verification:** every line below was checked against the actual trees, branches and live
native Actions on 2026-09-17; nothing is copied from a receipt without confirmation.

---

## 1. The ledger

```text
TB0 pinned revision          TB0-1 — contract tag ql.techne/v1 (additive-optional, no version bump)
TB0 head                     2c704dcc6911a4d0a11b20a23a71c4d222228e14 (branch techne/tb0-connective-base, PR #224 — still OPEN, unmerged)
Surfaces port                techne/tb0-surfaces-port @ 957d2bc612e3afcf9ca8ccf4eb76a65116c97b5b
Converged joined head        techne/tb0-surfaces-port @ 07cfe4a8ca55f78a72191b5abcc5ec695b16866a (local, not pushed)
T9 branch (this record)      techne/t9-integrated-field (worktree /private/tmp/ql-mef-t9-integration), cut from 07cfe4a
```

### Lane heads consumed (verified in local git; receipt comments on the issues)

| Lane | Issue | Branch | Final head | Head commit (occurrence) | Receipt comment (receipt) |
|---|---|---|---|---|---|
| M0′ Project / Wiki / Graph | #213 | techne/m0-project-graph | 2d4a9360a65965d392592462c11a913871292a44 | 2026-09-16T23:33:54+01:00 | 2026-09-16T22:35:17Z |
| M1′ Canvas / Constellation | #214 | techne/m1-canvas | aa920e65284aee66b64417254087020d9b087fa4 | 2026-09-16T23:37:18+01:00 | 2026-09-16T22:38:20Z |
| M2′ Relation Field / Timeline | #215 | techne/m2-relation-field | 374bf3197a59e42c958b078b913bcdee79cd9584 | 2026-09-16T23:43:51+01:00 | 2026-09-16T22:45:01Z |
| M3′ Journey / Scenes | #216 | techne/m3-journey-scenes | 9267e0856de78f3e2291ff04ee66dd7686314b31 | 2026-09-16T23:44:31+01:00 | 2026-09-16T22:46:13Z |
| M4′ World / Places | #217 | techne/m4-world-places | 239bc5b94ea87c8ee0fff98f7f57536e0b693018 | 2026-09-16T23:27:10+01:00 | 2026-09-16T22:27:59Z |
| M5′ Palace / Integral Whole | #218 | techne/m5-palace | bd891081324a68a21c542126bdbfbefa706671ca | 2026-09-16T23:38:52+01:00 | 2026-09-16T22:40:50Z |
| X3 3:3 Expression / Agency / desktop | #219 | techne/x3-expression-agency | 2ae7537341b76c6b2914c3e19a051a52815c9230 | 2026-09-16T23:37:55+01:00 | 2026-09-16T22:39:21Z |

### Native owner revisions actually exercised (installed on this machine)

```text
ctrl        0.1.0 (c7018cc3941a) — Central native Actions (central.now.list, central.day.read, central.time.policy, projectcentral.now.return, projectcentral.now.inspect)
aikit       0.1.0 — 197 capabilities catalogued; the session context ctx_01M2PCRX0QEG1YSV1WBNWTPDHT is the native AgentSession ref used throughout the walk
ql          0.1.0 (kernel 0.1.0-q1, MEF 1.0.0-q2@1, Vāk source daa660cbc1b8c5da83828698665a753852cb0287) — live `ql vak compose`
actuation   0.2.0 — actuation.agency/v1 machinery installed (agency.actualise admission law verified strict)
factory     installed (not exercised this walk — no commissioned developmental act was needed; distinguished, not merged)
O:I desktop installed at org.epilogos.oi.cradle; NOT currently running; the techne/convergence branch (worktree .agent-worktrees/techne-integration @ ec3100c1) carries the PRE-lane surfaces wired into the Cradle composition root
```

### Implemented / fixture-only / provider-tested / degraded

```text
implemented (all six + conjugate, at 07cfe4a):
  M0′ bounded-whole spine + LIST/TREE/GRAPH one-view projections + selection + disclosure
  M1′ authored Views (presentation), proposals (semantic, routing receipts), interactions
  M2′ relation field (7 projections), standing grammar, occurrence/receipt/day/now/session/run
  M3′ crossing law, presence, C′ binding (kernel-pinned CS passages), scene composition intake
  M4′ geography relations, movement ordering, carried occasion, depth chain, scale ladder
  M5′ integral regions, inhabitation, recognition/return legs, agent state
  X3 session dual-reading state, cross-cut legs, Expression bridge, surface lifecycle
provider-tested this walk (T9 additions):
  central-ground provider composing a valid TB0-1 reading from live ctrl reads + real ref table
  full canonical walk over that real reading (tests/techne-t9-integrated-walk.test.mjs, 10/10)
  live `ql vak compose` with real session provenance (13 ops, exit 0)
  real accepted Return into the project register (id t9-integrated-field-walked-one-real-2026-09-17)
fixture-only (regression specimen, kept deterministic):
  fixtures/techne/tb0-connective-base-v1.json — 349 cradle tests consume it; it is regression
  cover, never provider proof
degraded / unavailable (declared in the reading's own disclosure):
  place: the real ground asserts a civil-time frame, not surveyed geography (no geometry invented)
  journey: scene persistence in a live Expression substrate (O:I port-back seam)
  expressions: live running substrate (same seam; crossing/cue proven at session level)
  canvas: durable View persistence (O:I host seam); palace: session-scoped composition
  M4′ producer depths (M2 planetary / M3 world-clock / M4 Nara EarthBody): producers exist as
  Rust libraries but expose no surface-tree leg (recorded by lane E; unchanged)
  AIKit SemanticWiki + ProjectMap providers: ABSENT in the current install (aikit knowledge
  falls back to source-pool with honest absences) — recorded reality, not assumed presence
```

## 2. Compatibility findings (the #212 reconciliation)

**No genuine shared-contract insufficiency was found.** The lanes' convergence note stands
verified: no lane forked `TechneReading`, the schemas, or instrument/cut identity, and T9 adds
no shadow model. Two convergence-time items were verified done at 07cfe4a:

1. The TS mirror (`src/techne/contract.ts`) carries the full TB0-1 field set (mechanical,
   additive) and accepts the rich specimen; the NullableRef fidelity fix is in.
2. The session derives `application_cut` from the instrument; crossings keep every identity
   ref byte-exact (the walk's continuity test re-proves this over the real reading).

**Additive candidates recorded by lanes, still open (correctly not applied):** a C′ reading
facet for Journey/Scene bindings (#216); native producers emitting relation standing, place
relation+uncertainty, attempt/return continuity (T1-successor work). These route through #212
when taken; T9 did not take them.

## 3. The real-subject walk (what closed the cycle)

Subject: `github:EpiLogos/QL-MEF/issues/220` — the Technē dual-reading convergence.
Bounded whole: `git:EpiLogos/QL-MEF:techne/tb0-surfaces-port@07cfe4a` — the converged field
itself (lock docs, TB0 pin, Vāk lock, #212–#219 + lane heads, the O:I #65 absorption, the
ProjectCentral surfaces-port record, the Guardian expression ground, the civil-time policy).

Starting identity, providers, Actions and the full path evidence live in
`docs/t9-evidence/EVIDENCE.md`. The TB0 fixture remains alongside as deterministic regression;
it was never used as provider proof.

## 4. Remaining seams (exact, with owners)

```text
1. O:I port-back      port the refined surfaces (07cfe4a generation) into the O:I desktop
                      (techne/convergence hosts the pre-lane generation); then the installed-
                      desktop performance/lifecycle round (TD7) and live Expression scenes.
                      Owner: O-I (#219/#220 joins already record this).
2. O-I TS mirror      adopt TB0-1 in O-I's own desktop/cradle contract mirror (mechanical).
3. Agency actualise   construct one Anima_i and one Technē_i as actuation.agency/v1 records —
                      machinery installed; admission demands full WorldBinding/metagency-grant/
                      determination-ancestry/identity-evidence construction. Owner: Actuation-side.
4. SemanticWiki       install AIKit SemanticWiki/ProjectMap providers so M0′ reads the wiki
                      graph rather than source-pool fallback. Owner: AIKit.
5. C′ producer CLI    no installed surface drives ql.vak-composition/v1 C′ profiles (CS/CFP)
                      from the kernel; the journey consumes the pinned transcription
                      (cprime_oikonomia.rs → CS_PASSAGES). Owner: QL-MEF (ql-cli).
6. PR #224 / #221     TB0 connective base and the wayfinder/dispatch-map docs PRs are still
                      OPEN; merging them moves the pin from local to canonical main.
7. Human legs         H/EX (eight dual-reading acceptance questions) and Recognition of the
                      Return are the owner's; recorded separately from all automated evidence.
```
