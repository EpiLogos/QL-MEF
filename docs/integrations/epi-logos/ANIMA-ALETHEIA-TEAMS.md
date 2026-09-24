# Anima and Aletheia as native agent teams

**Standing:** owner-directed construction, 24 September 2026. Carries gap **G2** of `AW-SPRIME-CONTINUITY-LEDGER.md` to its native owners. Nothing here is recognised by the owner yet: the Central profiles and agent sets are proposal inputs (`generated-proposal` / `unrecognised` once proposed), and the skills are QL-owned sources that AIKit catalogues only when the `ql` source is synced to a revision that contains them.

**Owner's instruction (2026-09-24, verbatim excerpt):** "anima and aletheia need to be made properly as agent-sets/teams, all of their actual subagents and the subagent skills/tools are defined and specced (from the current ql-mef repo) ... when you say 'give them position' this doesn't require the QL prime agent loop at all, this is not a blocker, anima and aletheia exist relative to factory workflows (we have a TS workflows setup already which uses the vak execution language) and work across the development of expressions and techne work on constellations AND normal development and knowledge work."

**Consistency with `K-AW-EXPRESSION-PRODUCTION-ALIGNMENT.md` §4.** That section forbids a *new corpus-specific* roster. This is not one: the members are exactly the source-defined agents pinned in `aw0-sources-original-agents.json` (eight under S4′ Anima, seven under S5′ Aletheia). No member was invented, renamed or dropped; Aletheia remains outside the constitutional frames; Techne remains distinct (§6–§7 below).

## 1. Where each part lives

| Part | Owner | Location |
|---|---|---|
| Team expressions (intent, roster) | Central, root register | `Control/agents/expressions/anima/{intent.md,TEAM.md}`, `Control/agents/expressions/aletheia/{intent.md,TEAM.md}` |
| Member operating instructions (Claude Code subagent bodies) | Central, root register | `Control/agents/expressions/<team>/members/<member>.md` |
| Agent sets `anima`, `aletheia` (`central.agent-set/v1`) | Central | `Control/agents/agent-sets/` once proposed through `central.agent-set.propose` |
| Agent profiles `profile/<slug>` (`central.agent-profile/v1`) | Central | `Control/agents/profiles/` once proposed through `agent-profile.propose` |
| Domain skills `skill/ql/<name>` | QL-MEF (this repo), catalogued by AIKit | `skills/<name>/SKILL.md` |
| Generic practice skills `skill/personal/<name>` | the personal AIKit source | not copied here |
| Positions `anima-4`, `aletheia-5` | O:I Project ground | `Work/O-I/ProjectCentral/relations/positions/` |
| Projection of member files into `.claude/agents/` | AIKit | a separate change in AIKit |

## 2. Members

`cf` is the Context Frame the member works in, by the codes of `ql context-frame list`. For the Anima team it is also the member's constitutional voice (`constitutional_voice`, `crates/ql-mef/src/vak_profile.rs:433-443`). Aletheia members work inside frames without becoming their voice; the source numbered Aletheia's own frames from CF0, and those codes are translated here by expression.

### Anima (S4′) — agent set `anima`, orchestrator `agent/anima`

| Agent / profile | CF | Source frame literal | Office |
|---|---|---|---|
| `agent/anima` / `profile/anima` | CF5 `(4.0/1-4.4/5)` | `(4.0/1-4.4/5)` | the dispatch itself: composes the act through C′, holds execution, boundaries and Return |
| `agent/anima-nous` | CF1 `(00/00)` | `(00/00)` | epistemic clearing: opens ground, surfaces assumptions, curates sources; never the executor |
| `agent/anima-logos` | CF2 `(0/1)` | `(0/1)` | scope, specification, plan; nomos in service of the household |
| `agent/anima-eros` | CF3 `(0/1/2)` | `(0/1/2)` | relational scour; operation, tests and verification as its dev-work expressions |
| `agent/anima-mythos` | CF4 `(0/1/2/3)` | `(0/1/2/3)` | pattern recognition, debugging, archetypal reading |
| `agent/anima-psyche` | CF6 `(4.5/0)` | `(4.0/1-4.4/5)`, CT4b′ | continuity, NOW, coordination; calls on Aletheia |
| `agent/anima-sophia` | CF7 `(5/0)` | `(5/0)` | synthesis, Night′ lead, Möbius return, finishing |
| `agent/anima-techne-helper` | none | none | bounded workshop helper for worktrees and terminal workspaces; no routing, no VAK |

Psyche's own file writes `(4.0/1-4.4/5)` (the CT4b′ fractal doubling); the source's routing tables (`vak-coordinate-frame`, `anima-orchestration`) and QL's `constitutional_voice` place Psyche at `(4.5/0)`. QL governs: CF6.

### Aletheia (S5′) — agent set `aletheia`, orchestrator `agent/aletheia`

| Agent / profile | CF | Source frame literal | Office |
|---|---|---|---|
| `agent/aletheia` / `profile/aletheia` | none | `(5/0)` orientation | the unconcealment organ: Night′ pass, knowledge metabolism, the six gates, Return; not a constitutional peer |
| `agent/aletheia-anansi` | CF1 `(00/00)` | Aletheia CF0 `(0000)` | orientation and gap between blueprint and manifestation; placement with strict provenance |
| `agent/aletheia-janus` | CF2 `(0/1)` | Aletheia CF1 + CF`(0/1)` | temporal threshold: the before/after envelope, session and day seams, sense of reading |
| `agent/aletheia-moirai` | CF3 `(0/1/2)` | Aletheia CF2 `(0/1/2)` | distillation, one actor in three modes: Klotho (traces), Lachesis (sources), Atropos (crystallisation) |
| `agent/aletheia-mercurius` | CF4 `(0/1/2/3)` | Aletheia CF3 `(0/1/2/3)` | cross-domain translation and Kairos signal transport, keeping the origin's charge |
| `agent/aletheia-agora` | CF6 `(4.5/0)` → CF5 | Aletheia CF4a `(4.5/0)` / CF4b `(4.0/1-4.4/5)` | plural gathering and fusion without erasing voices; retrieval before coordination |
| `agent/aletheia-zeithoven` | CF7 `(5/0)` | Aletheia CF5 `(5/0)` | creative advance: turns insight into schedulable, writable next form |

## 3. Skill crosswalk

**Counts: 26 carried (ported as `skill/ql/*`), 32 mapped to an existing native skill or command, 10 not carried.**

### Carried — ported into `skills/` and rewritten against native commands

| Source skill (path under `Body/S/S4/ta-onta/`) | Native id | Team |
|---|---|---|
| `S4-4p-anima/S4'/skills/vak-coordinate-frame` | `skill/ql/vak-coordinate-frame` | Anima (reference) |
| `…/vak-evaluate` | `skill/ql/vak-evaluate` | Anima |
| `…/anima-orchestration` | `skill/ql/anima-orchestration` | Anima |
| `…/day-night-pass` | `skill/ql/day-night-pass` | Anima (Psyche, Sophia) |
| `…/klein-mode` | `skill/ql/klein-mode` | Anima (Sophia) |
| `…/ouroboros` | `skill/ql/ouroboros` | Anima |
| `…/darshana` (+ `darshana.py`) | `skill/ql/darshana` | Anima (reference, script carried verbatim) |
| `…/symbolic-protein-reading` | `skill/ql/symbolic-protein-reading` | Anima (Mythos) |
| named in `S4'/agents/eros.md` §5 only | `skill/ql/relational-graph-traverse`, `skill/ql/wikilink-resonance-scan`, `skill/ql/cross-source-dissonance-detect` | Anima (Eros) — no source body existed; bodies bounded to the one-line source definitions |
| `S4-5p-aletheia/S5'/skills/gnosis-retrieve` | `skill/ql/gnosis-retrieve` | Aletheia |
| `…/thought-distil` | `skill/ql/thought-distil` | Aletheia |
| `…/anansi` | `skill/ql/anansi` | Aletheia |
| `…/repl` | `skill/ql/repl` | Aletheia (reference; uses the `darshana` script) |
| `…/aletheia-stack-traverse` | `skill/ql/aletheia-stack-traverse` | Aletheia |
| `…/aletheia-module-audit` | `skill/ql/aletheia-module-audit` | Aletheia |
| `…/aletheia-improvement-propose` | `skill/ql/aletheia-improvement-propose` | Aletheia |
| `…/aletheia-self-extend` | `skill/ql/aletheia-self-extend` | Aletheia |
| `…/aletheia-plugin-integrate` | `skill/ql/aletheia-plugin-integrate` | Aletheia |
| `…/aletheia-{ql,m,s,m-prime,rupa,collab}-gate` (+ `gates/*.md` stubs) | `skill/ql/aletheia-ql-gate`, `…-m-gate`, `…-s-gate`, `…-m-prime-gate`, `…-rupa-gate`, `…-collab-gate` | Aletheia — the source's six-skill gate family kept as six skills with one result vocabulary |

### Mapped — an existing native skill or command does the job

| Source skill | Native mapping | Why this one |
|---|---|---|
| brainstorming (VAK fork) | `skill/personal/brainstorming` | CPF dialogical entry; the VAK extension lives in `skill/ql/vak-evaluate` |
| dispatching-parallel-agents (fork) | `skill/personal/dispatching-parallel-agents` | CFP1 and CFP3 |
| subagent-driven-development (fork) | `skill/personal/subagent-driven-development` | CFP2 chain |
| test-driven-development (fork) | `skill/personal/test-driven-development` | Eros's red/green at CP 4.2 |
| using-superpowers (fork) | `skill/personal/using-superpowers` | skill entry; the VAK-first reading is `skill/ql/vak-evaluate` |
| verification-before-completion (fork) | `skill/personal/verification-before-completion` | evidence before claims |
| writing-plans (fork) | `skill/personal/writing-plans` | Logos |
| executing-plans | `skill/personal/executing-plans` | CFP4 sustained |
| systematic-debugging | `skill/personal/systematic-debugging` | Mythos, Logos |
| finishing-a-development-branch | `skill/personal/finishing-a-development-branch` | Sophia |
| pipeline (Pleroma plugin) | `skill/personal/subagent-driven-development` + Factory workflow units with dependencies | CFP2 per-stage frames |
| team | `skill/personal/dispatching-parallel-agents` + Factory units | CFP1/CFP3 |
| ralph | `skill/personal/executing-plans` + a task list that closes only when done; Factory `verificationObligations` | CFP4 till-done |
| ultrawork | `skill/personal/dispatching-parallel-agents` | parallel throughput |
| tdd | `skill/personal/test-driven-development` | duplicate of the fork |
| ultraqa | `skill/personal/verification-before-completion` + `skill/personal/systematic-debugging`; the loop is `skill/ql/day-night-pass` P2′ | Eros tests, Mythos diagnoses, Sophia signs off |
| plan | `skill/personal/writing-plans` | Logos |
| ralplan | `skill/personal/writing-plans` + `skill/personal/requesting-code-review` | Logos → Mythos → Eros review chain |
| deep-interview | `skill/personal/brainstorming` | CPF dialogical clearing |
| analyze | `skill/personal/systematic-debugging` + `aikit knowledge code context\|impact\|trace` | evidence with file:line |
| git-master | `skill/personal/central-git-convergence` + `skill/personal/finishing-a-development-branch` | branch discipline, landing through review |
| worktrunk | `skill/personal/agent-worktree-lifecycle` + `skill/personal/using-git-worktrees` | worktree lifecycle |
| web-research | Claude Code `WebSearch` / `WebFetch` tools | tools, not a skill |
| deepsearch | `Grep`/`Glob` + `aikit knowledge search`, `aikit knowledge code search` | code search |
| tmux | `tmux` CLI, `herdr` | terminal sessions |
| cmux | `herdr workspace\|worktree\|agent …` | the installed terminal workspace manager |
| ralph-tui | `factory workflow inspect` for commissioned runs; `skill/personal/ralph-tui-*` where that tool is still used | bead visibility |
| techne-spawn | `aikit task spawn <name> --agent claude --worktree`; Claude Code `Agent` with worktree isolation | surgeon spawn (`skill/ql/ouroboros`) |
| techne-relay | Claude Code `SendMessage`; `herdr agent prompt`; `aikit gateway send` | channel to the child |
| techne-list | `aikit task list`; `herdr agent list` | child inventory |
| techne-close | `aikit task close` (refuses an unclean worktree without `--force`) | close and clean |
| pleroma-skill-proxy | AIKit profile / skill-set projection (`aikit set …`, `aikit apply`, `aikit compose`) | the child gets its skills natively |

### Not carried

| Source skill | Reason |
|---|---|
| chatlog-fetcher | renders AI chat pages with the user's browser cookies — credential-bearing browser automation with no native owner; session evidence now lives in Actuation streams (`actuation stream replay`) and Central NOW |
| youtube-transcript | external media fetch with no native owner in the suite; a harness-local copy exists outside AIKit |
| ask-claude, ask-gemini | a second model is a separate body chosen by AIKit/Actuation model and harness routing (`aikit model-resolve`, `aikit client`, `aikit task spawn --agent …`), not a prompt-helper skill |
| ai-slop-cleaner | no AIKit-catalogued equivalent; in a Claude Code body the built-in `simplify` review covers code cleanup |
| security-review | no AIKit-catalogued equivalent; in a Claude Code body the built-in `security-review` covers it |
| context7 | only a candidate example in `aletheia-plugin-integrate`; adoption would go through that membrane |
| `vama_shakti_template_authority` (Psyche entitlement) | an entitlement of the retired Pi summon factory (`techne_vama_summon`); profiles are now authored through Central `agent-profile.propose`, and the six-section soul shape survives as the member-file structure |
| aletheia-drift-detection, aletheia-elo-rating (`S4-5p-aletheia/skills/custom/`) | depend on the retired S3 SpaceTimeDB runtime (`retrain-loop.sql`, `elo-runtime.sql`); rating and retraining of agents, models and skills stay research until implemented (capability matrix S5′ M3 row) |

The 20 staged Pleroma copies pinned in `aw0-sources-original-skills.json` (`S4-2p-pleroma/staged/…`) no longer exist at HEAD; they duplicate rows above.

## 4. Tool crosswalk

**Counts over 57 original tools: 45 mapped to a native command, 4 carried as the same tool or script, 8 to build.**

Return doors used below: `projectcentral.now.return` (project with `project`, root without), `central.now.thoughts.append` (T stream), `central.now.learnings.distill`, `projectcentral.now.promote` (the only door into a wiki), `projectcentral.source.return` (proposed change to Project source). No member edits `wiki.json`.

### Anima extension and carriers (`S4-4p-anima/extension/*.ts`, `S4-4p-anima/S4/*.ts`)

| Tool | What it did | Native now (owner) |
|---|---|---|
| `vak_evaluate` | assign CPF/CT/CP/CF/CFP/CS | agent judgement per `skill/ql/vak-evaluate`, checked by `ql vak compose <request.json>` and `ql context-frame list` (QL-MEF) |
| `goal_prelude` | NOW-bound dialogical /goal artifact | `projectcentral.now.return` `kind` `question`/`note`, or a `T0` thought via `central.now.thoughts.append` (Central) |
| `nous_disclose` | curate a context package into the session notebook | `aikit knowledge search\|resolve\|read`, `aikit refocus`, then a NOW `note` listing the sources (AIKit, Central) |
| `anima_orchestrate` | CF → agent routing decision | `skill/ql/anima-orchestration` table over `ql context-frame list` (QL-MEF) |
| `dispatch_agent` | run one team agent | Claude Code `Agent` with the member slug; `aikit task spawn`; a Factory unit with `agentRefs` |
| `dispatch_parallel_agents` | CFP1 | several `Agent` calls in one message; independent Factory units |
| `dispatch_fusion_agents` | CFP3 with Agora aggregation | the same brief to N members + `agent/aletheia-agora`; Factory units + barrier |
| `dispatch_moirai_night_pass` | Klotho/Lachesis/Atropos over a Sophia disclosure | `agent/aletheia-moirai` per `skill/ql/day-night-pass`; results as `T1`/`T4`/`T5` thoughts |
| `anima_self_invoke` | queue a task in another Anima session | `aikit gateway send` to a Position (`central:position:project:O-I:anima-4`); `aikit gateway delegate` for custody-bearing work (AIKit, Factory) |
| `run_chain` | sequential pipeline | sequential `Agent` calls passing only the selection; Factory units with `dependencies` |
| `subagent_create` / `_continue` / `_list` / `_remove` | background subagents | `Agent` (background) and `SendMessage`; `aikit task spawn\|list\|close` |
| `tilldone` | block closure until the task list is done | a task list that closes only when every task is done; Factory `verificationObligations` and `factory workflow inspect` |
| `anima_arena_orchestrate` | classifier-aware turn routing in an arena scene | **to build** — spec A |

### Khora, Hen, Chronos (`S4-0p-khora`, `S4-1p-hen`, `S4-3p-chronos` extensions)

| Tool | What it did | Native now (owner) |
|---|---|---|
| `khora_write` | the canonical vault write | the Return doors above (Central); code and docs through the owning repository's branch and PR |
| `hen_hybrid_retrieve` | Obsidian search + Neo4j traversal | `aikit knowledge search`, `aikit knowledge relations\|graph` (AIKit) |
| `hen_frontmatter_validate` | 126-key vault frontmatter schema | `aikit wiki validate`, `aikit wiki-shape validate` for wiki material; the vault schema is historical (`skill/ql/aletheia-ql-gate`) |
| `hen_template_invoke` | render a template into the vault | task specs are Factory workflow unit fields (`developmentalConcern`, `requiredDifference`, `returnContract`); working notes are NOW returns; `central.template.stamp` only stamps Central's own default sources |
| `graph_query` | reported Cypher unavailable | `aikit knowledge graph`; Bimba graph through `skill/ql/bimba-cypher` under its authority rules (QL-MEF) |
| `web_search`, `web_fetch` | DuckDuckGo search, Jina reader fetch | Claude Code `WebSearch`, `WebFetch` |
| `chronos_temporal_status` | Day folder, active NOWs, archive backlog | `central.day.read`, `central.now.list`, `projectcentral.now.inspect`, `central.time.policy` (Central) |
| `chronos_cron_register` | register a cron job | `aikit routine create` from a proven basis (Draft until the owner enables it with a fresh authority receipt); occurrences resolved by `central.time.occurrences` under the civil-time policy (AIKit, Central) |
| `chronos_cron_list` | list cron jobs | `aikit routine list` |
| `chronos_kairos_fetch` | Kerykeion natal chart + planet degrees from birth data | current sky: `providers/sky/kerykeion_snapshot.py` (`ql.sky-snapshot/v1`, `docs/kernel-rebuild/K8-SKY-CONTRACT.md`); natal/personal part **to build** — spec B |
| `chronos_kairos_status` | natal/stub mode, planet validity | **to build** with spec B |

### Janus pure functions (`S4-0p-khora/modules/now-klein-weighting.ts`)

| Tool | What it did | Native now |
|---|---|---|
| `janus_track_spreads`, `janus_evaluate_aliveness`, `janus_spread_resolved` | track OracleSpread positions as live or mute; resolve a spread | **to build** — spec C |
| `janus_weight_session` | per-session Klein weighting (prospective/retrospective) from Kairos | **to build** — spec D |

### Aletheia S5′ tools (`S4-5p-aletheia/S5'/tools/*.ts`)

| Tool | What it did | Native now (owner) |
|---|---|---|
| `aletheia_gnosis_query` | hybrid vector + graph retrieval | `aikit knowledge search\|read\|relations\|route` per `skill/ql/gnosis-retrieve` (AIKit) |
| `aletheia_gnosis_status` | Gnosis tier status | `aikit knowledge status` |
| `aletheia_gnosis_ingest` | parse, chunk, embed into Gnosis | place the document with its owner (repository PR or `projectcentral.source.return`); AIKit knowledge reads owners' ground. The embedding store is not carried |
| `aletheia_gnosis_notebook_create` | session-scoped retrieval pool | a declared address set carried in a NOW `note` and read with `aikit knowledge route <address>… --query` |
| `aletheia_gnosis_enrich` | assign a coordinate or resonance to an entity | Bimba relation through `skill/ql/bimba-cypher` (explicit authority) or a wiki relation return through NOW promotion |
| `aletheia_session_promote` | promote high-signal memory into Gnosis | `central.now.learnings.distill`, then `projectcentral.now.promote` (`target` `agent-wiki`, `acceptance` `agent-return`) |
| `aletheia_thought_route` | classify into T0–T5 buckets | `central.now.thoughts.append` with `reading` `T0`…`T5` or `T0-prime`…`T5-prime` (Central) |
| `aletheia_crystallise` | distil T buckets into canonical form | `central.now.learnings.distill`; formal T/T′ envelope through `ql epi-agent invoke` `logos.return` |
| `aletheia_seed_refresh` | morning SEED.md from evening crystallisation | carried records with next-review conditions in NOW; the day close (`central.day.lifecycle`, owner token); `aikit now-context prepare` builds the next participant view |
| `aletheia_ingest` | Sophia disclosure + Moirai summaries into Epii's inbox | NOW `handoff` with source and evidence refs; `aikit gateway send` to the Aletheia/Epii Position; owner-side `aikit flow contemplate` |
| `epii_invoke_anima` | Epii asks an Anima session for VAK evaluation | `aikit gateway send` to `central:position:project:O-I:anima-4` |
| `moirai_arena_distill` | arena scene close → graph episodes | **to build** with spec A |
| `aletheia_episodic_record` | QL-typed personal episode | `central.now.thoughts.append` (dated, attributed, T reading); astrological stamp waits on spec B |
| `aletheia_episodic_search` | BFS over the episodic graph | `central.now.thoughts.read`, `aikit knowledge search`; decan/tick12 filters wait on spec B |
| `aletheia_episodic_arc_open` / `_arc_close` / `_arc_status` | named arcs (sagas) | NOW clearings: `central.now.list\|read` for status; allocation and lifecycle are owner-token Actions (`central.now.allocate`, `central.now.lifecycle`) requested through a NOW `question` |
| `aletheia_episodic_oracle_arc` | oracle cast as a four-face saga | QL's Nara faculty `ql epi-agent invoke` position `#4` (`nara.activity.validate`) + the cast's thoughts in NOW |
| `aletheia_episodic_logos_stage` | Logos cycle stage episodes | thoughts in NOW carrying the stage; the formal return through `logos.return` |
| `aletheia_episodic_mobius_arc` | Möbius return arc | `T5` + `T0` thoughts and a NOW `question` (`skill/ql/day-night-pass`) |
| `aletheia_episodic_ingest_thoughts` | T buckets → typed episodes | `central.now.learnings.distill` over the named fixtures |

### Carried as the same tool

| Tool | Now |
|---|---|
| `Bash`, `Read`, `Glob` (techne-helper) | the same Claude Code tools |
| Darshana REPL (Anansi) | `skills/darshana/scripts/darshana.py`, carried verbatim |

### To build — specifications

**A. Arena scene routing and closing distillation** (`anima_arena_orchestrate`, `moirai_arena_distill`). Owner: O:I (the scene surface), with Software Factory (turn units) and AIKit (knowledge return). Inputs: scene ref, participant agent refs, the scene's classifier, CPF set at scene setup (dialogical) and per-turn CF; per turn the utterance and classifier result; at close the transcript ref. Outputs: a per-turn routing receipt `{turn, cf, agent_ref, reason}`; at close a Vāk-compressed reading returned as a NOW learning with cited coordinates as promotion candidates (the Graphiti episode store is not carried). Blocked on: no current O:I arena/dialogue scene surface exists to host it — the owner decides whether it is wanted.

**B. Personal Kairos reading** (`chronos_kairos_fetch`, `chronos_kairos_status`). Owner: QL-MEF (a faculty #4 operation behind `ql epi-agent invoke`), reading Central's protected identity source by owner authority. Inputs: an instant and place resolved under Central's civil-time policy; a `ql.sky-snapshot/v1` for that instant; a protected natal-basis ref (never copied into the reading). Outputs: `{instant, bodies:[{name, longitude, speed, station|shadow}], decan_tattva, transits_to_natal, provenance}` plus a status `{mode: sky-only|natal, planet_valid, source_revision}`. Protected personal state is never published to shared fields.

**C. Oracle spread aliveness** (`janus_track_spreads`, `janus_evaluate_aliveness`, `janus_spread_resolved`). Owner: QL-MEF M4 Nara faculty (pure computation, as the source module is), carrier Central NOW. Inputs: open spread records (cast ref, positions, NOW ref), the content delta since the last read, the current Kairos reading (spec B). Outputs: per-position `live|mute` with reason; a resolution `{resolved, remaining}`. Source: `now-klein-weighting.ts:243-316`.

**D. Session Klein weighting** (`janus_weight_session`). Owner: QL-MEF (computation) with Central NOW as the carrier. Inputs: the Kairos reading (spec B) with motion signals, an optional user override (absolute). Outputs: `{prospective, retrospective}` weights with reasons, recorded as a `T4` thought until Central grows a NOW field for it. Source: `now-klein-weighting.ts:356`.

**Module (not a registered tool): symbolic-protein reader** (`S4-4p-anima/modules/symbolic-protein-reader.ts`). Owner: QL-MEF, faculty #4 (`mythos.symbolic-protein.read` or equivalent). Inputs: a governed chain projection `{position, fingerprint}`, weather `{m1_tick, m2_phase, m3_transcription, sky_snapshot_ref}`, four provenance refs. Output: `MythosArchetypeReading` (`skills/symbolic-protein-reading/SKILL.md`). Refuses without a governed projection; never reads the protected protein body.

## 5. Relation to Factory workflows

The teams exist relative to Factory's TypeScript workflows (Factory #195/#197; `factory workflow sdk|check|compile|commission|inspect`). A unit names who may act and which practice governs it:

```json
"agentRequirements": {
  "agentSetRefs": ["central:pasu:agent-set:anima"],
  "agentRefs": ["agent/anima-eros"]
},
"praxisRefs": ["skill/ql/vak-evaluate", "skill/personal/test-driven-development"]
```

- `agentSetRefs` use Central's paśu form of the set ref (`central:pasu:agent-set:anima`, `central:pasu:agent-set:aletheia`) because Factory's `qualifiedRef` needs an owner-qualified value; the Central record's own `ref` stays `anima` / `aletheia`.
- `agentRefs` are the member agent refs (`agent/anima-logos`, `agent/aletheia-moirai`, …).
- `praxisRefs` are skill ids — `skill/ql/*` for domain practice, `skill/personal/*` for generic practice.
- The Vāk block (`skill/ql/vak-evaluate`) is the unit's optional C′ reading; the specimen `specimens/expression-craft.vak.ts` shows the participant shape (role, role source, agent source, skills, faculties, permitted effects, expected return). Generic Factory workflows require none of it.
- The Vāk workflow adapter (VW1) and Factory's authoring surface are another lane's work; this document only fixes the participant, set and practice identities they consume.

## 6. Three kinds of work

1. **Expression development** (3:3 reading, Anima_i). Anima composes the undertaking through C′; Nous grounds, Logos specifies, Eros operates and verifies, Mythos finds the organising image, Psyche keeps continuity, Sophia integrates. Aletheia returns what happened: Moirai rehears, Agora fuses plural readings, Zeithoven hands the next score forward; the M′ and Rupa gates check the artifact.
2. **Technè on constellations** (4:2 reading). Technē_i := Aletheia_i situated in M_i′, operating the deep instrument through its native Actions (`ql techne reading`, the instrument's published Actions). Aletheia leads with stack traverse, the QL and M′ gates, Anansi placement and Moirai distillation; the collaboration gate stands before any canonical change. `agent/anima-techne-helper` is not Technē: it only prepares worktrees and terminal workspaces.
3. **Ordinary development and knowledge work.** No QL prime loop is required. The Anima lead dispatches Logos for plans, Eros for tests and verification, Mythos for debugging, Sophia for finishing and landing, Nous for clearing and retrieval, Psyche for NOW continuity; Aletheia serves retrieval, distillation, audits and improvement proposals. A Vāk block is at most a silent one-line reading.

## 7. Positions and the twelve situated roles

For every M_i (i = 0…5): Anima_i = M_i × S4′ and Aletheia_i = M_i × S5′ (`docs/L5-TECHNE-DUAL-READING-LOCK.md` §15) — twelve situated roles. The O:I Positions `central:position:project:O-I:anima-4` (Anima 4: M4 Nara × S4′) and `central:position:project:O-I:aletheia-5` (Aletheia 5: M5 Epii × S5′) are their first proving instances. A team is not a Position: the lead agent (`agent/anima`, `agent/aletheia`) is the body eligible to occupy the Position (`aikit inhabit --position <ref>`, Actuation occupancy), and its members are the subagents that body dispatches. Guardian_i ≠ Anima_i ≠ Technē_i.

## 8. Source pins

Source repository `EpiLogos/Epi-Logos-C-Experiments`, read at HEAD `b57ddda2a4815c0c3e0ca88fb01394d8b6a10699` (2026-09-17). `git hash-object` against `aw0-sources-original-agents.json`: 5 of 15 agent files match their pins; 10 have drifted since pinning (all by additions: the 2026-06-18 constitutional-agents disposition notes, and small body edits in Eros, Janus, Agora, Moirai). Every pinned blob still exists in history (`git cat-file -t` → blob). The member files cite the HEAD blob they were rewritten from.

| Agent | Pinned blob | HEAD blob |
|---|---|---|
| anima | `c9bc322a` | `bb1986ff` |
| nous | `24363138` | `6e1bd6f2` |
| logos | `1fcc333d` | `5ca8e79d` |
| eros | `fcdc4ccf` | `455b6456` |
| mythos | `d7be7be0` | `d919e8ed` |
| psyche | `d31e471c` | `51d38ca7` |
| sophia | `d2d50972` | `0d327a7f` |
| techne-helper | `13c5dbcf` | same |
| aletheia | `28b5f698` | same |
| anansi | `ce8fa32c` | same |
| janus | `252cfc57` | `07320916` |
| moirai | `83d487a3` | `5421f5e3` |
| mercurius | `358f9f67` | same |
| agora | `848a1376` | `ec8ae3b9` |
| zeithoven | `ff4839d4` | same |

Skills: 38 of 65 pinned skill files match at HEAD; 7 drifted (`anima-orchestration`, `day-night-pass`, `klein-mode`, `ouroboros`, `vak-coordinate-frame`, and the Pleroma `cmux`, `pleroma-skill-proxy`); 20 staged copies are gone from the tree. Each ported skill records the source blob it was rewritten from, and its pin where one exists.

## 9. Open

- The Central profiles and agent sets are proposal inputs only in this change; proposing them (`agent-profile.propose`, `central.agent-set.propose`) and recognising them (`agent-profile.accept`) are separate acts.
- AIKit catalogues these skills when its `ql` source is moved to a QL-MEF revision that contains them.
- Specs A–D and the symbolic-protein reader are unbuilt; A waits on an owner decision about arena scenes.
