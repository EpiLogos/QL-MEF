# Gate 0 / Part B: splitting the infrastructure from the K² meaning (QL-MEF #251, O:I #545)

This is read-only research (27 Sep 2026). I made no edits, commits or fetches in either worktree.
Audit read first: `docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md` (QL main `4669d8d`).

## 0. State of each branch

| | QL-MEF #251 | O:I #545 |
|---|---|---|
| worktree | `QL-MEF #251 branch `feat/m123-engine-binding`` | `O:I #545 branch `feat/m123-expression-instrument`` |
| head | `046c23b` (`feat/m123-engine-binding`), draft, MERGEABLE | `af35abb2` (`feat/m123-expression-instrument`), draft, MERGEABLE |
| merge-base | `56921a5` | (origin/main `36c9e411` is 11 ahead) |
| commits ahead of origin/main | 14 | 5 |
| **behind origin/main** (`rev-list --count HEAD..origin/main`) | **1** (`4669d8d`, which adds AGENTS.md, CLAUDE.md and the audit doc) | **11** (`7fcda9bf`…`36c9e411`: gates, campaign-evidence, dev world, cli fmt) |
| `git merge-tree --write-tree origin/main HEAD` | clean, tree `34d2502…`, exit 0, **no conflicting paths** | clean, tree `23c1556…`, exit 0, **no conflicting paths** |
| diff `origin/main...HEAD --stat` | 26 files, +2989 / −57 | 32 files, +3565 / −241 |
| overlap between main's new commits and the branch's files | none (main touched only AGENTS.md, CLAUDE.md and the audit) | none (the reverse diff over the branch's files equals the branch diff exactly) |
| uncommitted changes | none | **`M desktop/cradle/tests/native-expression-k2-browser.mjs`** (lines 84 and 140: the studio click goes through `frame.evaluate(...click())`). Not on the PR, and I left it untouched. |

## 1. Classification of every commit

(a) is infrastructure to keep, (b) is K² meaning to replace, (c) is mixed.

### QL-MEF (oldest first)

| sha | subject | files | class | notes |
|---|---|---|---|---|
| `213c2a2` | Install the native Expression owners at the same cut as ql | `cpp/Makefile`, `scripts/oi-source-install.sh` | **a** | Managed install of ql, both hosts and the worker, plus the json-c include fix for Macs carrying both Homebrew architectures. |
| `982b19a` | K² played torus … | 17 files | **c** | See C1. |
| `2f96bca` | Register the K² provider with the K8 census and cache the compiled M ledger | `c/registry/promotions/k8-bindings-v1.json`, `m2_engine.rs`, `m_ledger.rs`, census receipt | **c** | The binding entry (`k8-bindings-v1.json:5-19`, disposition for `k2.rs`) is **b**. The ledger cache is **a** but breaks a lock (see §4). The census receipt is derived and must be regenerated. |
| `3b0ba28` | CI: run the shape-replacement and K² instrument tests… | `.github/workflows/kernel-k8-continuous.yml` | **c** | Keep the `k8_reshape` line (branch `:82`). Drop the `k2_instrument` line (`:83`) and the `k2_nara_reception` line (`:84`, added by `3db025b`). The fmt list (`:27`) drops the k2 paths. |
| `b5cf15f` | Session adapter: K² determinant events and the influence reading | `adapters/retained-field/instrument-session.{mjs,test.mjs}` | **a**, carried into QL-B | Transport only: a generation may rise by 1–2 with the cursor unmoved, and `influence()` is a read. It is only useful once a provider exists (see §2). |
| `3db025b` | Publish the K² Expression binding and Nara reception on the K² owner | CI, adapter, `host.rs`, `k2.rs`, `tests/k2_nara_reception.rs`, `K2-EXPRESSION-BINDING.md`, census | **c**, mostly **b** | See C2. |
| `9e436cd` | Re-record the M1 acceptance lock for ql-cli and refresh the ledger | `m1-engine-acceptance-v1.json:439`, `m-ledger-v1.json:3,110365`, census | **b**, drop | It exists only because `ql-cli/src/lib.rs` gained `k2-binding`. With that deferred, `lib.rs` stays byte-identical to main and this commit is unnecessary. |
| `e0043a5` | K² events cost one compose and half the transfer | `k2.rs`, `m2_engine.rs`, `m_ledger.rs`, census | **c** | The `k2.rs` hunks are **b** (carry-forward of the element guess, 1e-6 coefficient quantisation). The `m2_engine.rs:533-540` `OnceLock` revision cache is **a** but see §4. The `m_ledger.rs` hunks net to zero against main. |
| `9b91d53` | ql-sky: digest its payload portably and refuse an empty digest | `scripts/oi-source-install.sh` | **a** | The Omarchy-found sha256sum/shasum fix. |
| `6c82b5f` | Binding: record the unspecified sky-to-condition rule… | `K2-EXPRESSION-BINDING.md` | **b** | |
| `fe5e60a` | K² determinant acknowledgements carry their influence reading | adapter, `host.rs`, census | **c** | The adapter's `lastInfluence` (`instrument-session.mjs:66`, the capture at `:152`) is **a**. `host.rs` putting `response["influence"]` into a determinant reply calls `K2Instrument::influence()`, so it is deferred with the host ops. |
| `bbf0e9f` | Worker: buffered streams instead of a stdio lock per character | `cpp/src/field_worker.cpp` (4 lines at `main()`), census | **a** | Worker I/O. |
| `e29e3a3` | K²: 64×64 samples by default | `k2.rs:414-423`, census | **b** | The invented default sample grid. |
| `046c23b` | Mark the K² binding superseded as a semantic account | `K2-EXPRESSION-BINDING.md` | **b** | |

### O:I (oldest first)

| sha | subject | files | class | notes |
|---|---|---|---|---|
| `3647a825` | Reach the native QL Expression owner from the installed app | `cli/src/{development_field_hardening,product_command,suite_v2,update_flow}.rs`, `cli/tests/development_field_s0.rs`, `kernel/src/native_expression.rs` (where-resolution: `143-173, 176-230, 235-279, 331-339` plus tests `1378-1510`), `surfaces.json` | **a** | The catalogue gains `companions`; `oi update` stages them content-addressed with rollback kept in pairs; `oi where` discloses them; the kernel resolves them through `oi where` with the environment variables as override. `surfaces.json:386-409` points the build at `sh scripts/oi-source-install.sh`, **which does not exist at the currently pinned QL cut** (see §2). |
| `35950ae0` | Compose the live instrument natively, and find `oi` in the installed app | `native_expression.rs` (+1270), `tests/native_expression_compose.rs`, `src-tauri/src/main.rs`, `nativeChannel.ts`, `types.ts` | **c** | See C3. |
| `1e400188` | The live M1–M3 instrument in the Expression app | 20 files (UI, vendored adapter, tests) | **c** | See C4. |
| `3158636e` | Live instrument: gapless determinant events and exact cadence accounting | `controller.ts`, vendored adapter and PROVENANCE, tests | **c** | Fill-before-send, the acknowledged-means-applied rule and latency accounting are **a**. Vendoring QL `fe5e60a` is re-pinned in the split. |
| `af35abb2` | Backcheck: pin QL through the vendored provenance; gate the K² trace | `.github/workflows/native-expression-backcheck.yml` | **c** | Pinning through PROVENANCE (`:27-34`) and building `ql` too (`:116`) are **a**. The K² trace step (`:146-161`) waits for a composer to exist. |

### C1. `982b19a`, hunk by hunk (branch paths)

| file:lines | class |
|---|---|
| `cpp/include/ql/continuous_field.hpp:69-73` (shared `reference()`), `:153-170` (`replace_shapes`) | a |
| `cpp/src/field_worker.cpp` `initialize(...shape_ref)`, `shapes()`, the `replace-shapes` op, echoing `shape_ref` | a |
| `cpp/tests/continuous_field.cpp:101-163` (`shape_replacement_keeps_state_and_pcm`) | a |
| `crates/ql-mef/src/continuous.rs:7` (`pub mod k2;`) | b, remove |
| `continuous.rs:75-78` (`FieldInput.shape_ref`), `:285-315` (`FieldSession::replace_shapes`) | a |
| `continuous/coupled.rs:469-497` (`replace_field_state`, `replace_shapes`) | a |
| `continuous/receipt.rs:159-161, 205, 208-214, 312, 346-361, 407` (admitting `shape_ref` and a reshape) | a |
| `continuous/host.rs:4, 41-57, 81-100, 129-147, 213-316, determinant influence` (K² owner, `M1Advance`/`ReplaceEvent`/`Influence`/`ReceivePersonal`/`Personal`, `open_config` K² branch) | b-bound, defer (§2) |
| `bin/ql-field-host.rs:3, 34` (`open_config`) | defer with host.rs; keep `HostConfig`+`open` as on main |
| `continuous/k2.rs` (whole file, 874 lines) | b. Two neutral helpers could be lifted later: `attach_sky` `:462-502` (mirrors K8 `attach_m2`) and `next_generation` `:551-579`. Everything else is invented meaning: torus+Chladni `:178-310`, `default_*` `:414-460`, `influence()` `:730-789`, `m1_advance` `:709-723`, Nara reception `:646-670`. |
| `crates/ql-cli/src/lib.rs:557-567` (`k2-binding`) | b, defer. This is what forced `9e436cd`. |
| `m2_engine.rs`, `m_ledger.rs` (cache) | a, see §4 |
| `tests/k8_reshape.rs` (uses only `FieldSession` and `CoupledFieldSession`, no k2) | a |
| `tests/k2_instrument.rs` | b |
| `fixtures/kernel/k2-default-event-v1.json` | b |
| `docs/kernel-rebuild/K8-CONTINUOUS-CONTRACT.md:46-70, 112-113, 146-150` | a, except **`:48-53`**, which is K² prose ("nodal quartet … on the K² torus is re-read whenever the M1 tick/lens/mode or M3 pose changes") and needs a neutral sentence: "an explicit re-reading of mode shapes over the same samples and voices" |
| `scripts/oi-source-install.sh:51-83` (`ql-sky` single executable) | a |

### C2. `3db025b`

Everything in this commit is **b** except the adapter's `personal()`. That is transport, but its doc (`instrument-session.mjs:257-259`, "seven supplied centre inputs") and its payload type `nara::PersonalEventInput` are what the audit flagged: per-centre `{m1,m2,m3}` inputs that have no source. The `personal`/`receive-personal` wire should therefore wait for the Nara ruling too.

### C3. `35950ae0`: `native_expression.rs` blame ranges

| lines | content | class |
|---|---|---|
| `280-330` `oi_executable`/`resolve_oi`, plus `src-tauri/src/main.rs:116-121` (pin `OI_BIN` at startup) and test `oi_resolves_override_then_path_then_managed_activation` (`:1964`) | OI_BIN pinning for Dock launches | a |
| `340-500` `run_bounded` and `RunError`, test `bounded_runs_kill_the_whole_group_and_cap_output` (`:1930`) | bounded subprocess | a (only compose uses it, see §2) |
| `803-818` `EXCHANGE_OPERATIONS` gains `m1-advance`/`replace-event`/`influence`, tests `:1666`, `:1685` | wire admission | a-shape, but it names K² host ops (§2) |
| `820-1158` compose request, sky provisioning, `PrivateFile`, provenance, outside-lock prepare/finish (`1220-1285`); `main.rs:39-42` | compose mechanics | a |
| **`1159-1210`** schema `"ql.k2-binding-request/v1"` and `ql kernel k2-binding` | endpoint | **b** |
| `36-40` `Request::Compose` doc "compose a K² binding"; `types.ts:92,95` comment | wording | b |
| `tests/native_expression_compose.rs` (ignored, real QL) | calls `k2-binding` | b-bound |

### C4. `1e400188` and `3158636e`: UI and tests

| file:lines | class |
|---|---|
| `native-field/k2.ts` whole: `K2_PROVIDER`, `readK2`, the eight voices, the `i%4` quartet, `k2CausalTrace` | b. Neutral and salvageable: `lensLabel`/`LENSES`/`CONTEXT_FRAMES` `:12-14` and `editK2Event` `:100-126`, which are determinant edits on the coupled M1/M3 event. |
| `native-field/controller.ts:7, 24-30` (`K2_SAMPLE_RATE` from k2 `default_field`; `CADENCES` 1/s and 12/s, which the audit lists as an open owner question), `:69-70, 107-115` (M3 "entering the M2 Vimarśā reading"), `:129-140`, `:143-151`, `:375-381` | b |
| `controller.ts` serial determinant path `:306-342`, skip-not-queue cadence scheduler `:418-431+`, fill-before-send, acknowledged-means-applied, latency split, influence/source staleness `:401-416` | a |
| `nativeField.ts:1, 24, 49` ("Eight Vimarśā voices" table), `:55` cadence, `:142-160` | b |
| `nativeField.ts` panel structure: open, step, cadence, determinant segments, sound/level/scale, influence table with warrant, return/checkpoint/close | a (shell) |
| `native-field/projection.ts:41-48` (test-only target-disconnect flag, `admitted_a`); `app.ts` `probeSteps` | a (causal-trace hooks) |
| `physicalFormActuator.ts` (`not-actuated` status, consumer-gated apply) | a. The one exception is the `NO_POSE_CONSUMER_REASON` text "M3 reaches the field only through the M2 Vimarśā voices", which is b wording. |
| `nativeActuatorStanding.ts:56-68` (M1 "Vimarśā voices + M1 torus rest body", M3 "codon enter the M2 Vimarśā reading") | b |
| `app.ts` world lens and `instrumentOffered`, `shell.ts` label, `hostedApp.ts:470-476` + `PointCloudHost.tsx:182-188` (host posts its world lens) | a |
| `nativeChannel.ts` compose relay, `types.ts:92-95` | a-shape, travels with compose |
| `tests/native-expression-fixture.mjs` `ControlledK2Owner` | c: protocol fixture carrying the `ql.k2-torus-provider/v1` vocabulary |
| `tests/native-expression-instrument.test.mjs` (11 tests) | c: cadence, serial and refusal laws are a; compose/K² reading is b |
| `tests/native-expression-k2-browser.mjs` | c: the causal method (two controls plus one varied, and the `K2_DISCONNECT_TARGETS=1` run that must fail) is a; the "top voice retunes" assertions (`:173, 192-223`) are b |
| `tests/nara-embodiment-*.test.mjs` | a, except the assertion `standing.layers[2].actuator` matches `/Vimarśā/`, which is b |
| vendored `ql/instrument-session.mjs` and `PROVENANCE.json` (QL `fe5e60a`) | re-vendor from the QL-A merge |

## 2. Split plan

### Where O:I pins QL (three places)

1. **`surfaces.json:361` `docs_ref`, `:382` `command_revision`, `:414` `install.ref`, `:415` `install.revision`** are all `4276b94` (an ancestor of QL main, before the install script). The branch already changed **`:386-409`** to build `sh scripts/oi-source-install.sh` with four companions. Pairing that change with pin `4276b94` would make `oi update` fail, because the script is absent at that cut. **The bump to the QL-A merge sha must happen in the same O:I PR.**
2. **`desktop/cradle/expressions-app/field-studies-journeys/src/native-field/ql/PROVENANCE.json` `revision`**: main has `56921a5`, the branch has `fe5e60a` (a branch-only sha). `instrument-session.mjs` must be byte-identical: main's sha256 is `e971c794…`, the branch's is `a350ffef…`.
3. **`.github/workflows/native-expression-backcheck.yml:34`**: main hard-codes `ref: 56921a5e…`; after `af35abb2` it reads `ref: ${{ steps.ql.outputs.revision }}` from PROVENANCE (`:27-30`).

**Landing order:** QL-A merges first, then O:I-A bumps pins 1 and 2 to QL-A's merge sha. The replacement meaning follows as QL-B, then O:I-B bumps the same pins to QL-B.

### QL-A: infrastructure only (new branch off QL main `4669d8d`)

Take from `213c2a2`, `9b91d53` and `bbf0e9f` whole. From `982b19a` take the C1 rows marked **a**. From `2f96bca`/`e0043a5` take only the ledger-revision cache, relocated as §4 describes. From `3b0ba28` take only the `k8_reshape` step.

- **Remove or defer** (no neutral stub: a host op with no provider behind it is dead, and clippy `-D warnings` rejects dead private items):
  - `continuous.rs:7`, `continuous/k2.rs`, `tests/k2_*.rs` and `fixtures/kernel/k2-default-event-v1.json`.
  - `ql-cli/src/lib.rs:557-567`, together with all of `9e436cd`. `m1-engine-acceptance-v1.json` and `m-ledger-v1.json` then stay at main.
  - The `host.rs` K² owner. That means the `Owner` enum, the variants `M1Advance`/`ReplaceEvent`/`Influence`/`ReceivePersonal`/`Personal` (`:41-57`), `open_k2`/`open_config` (`:129-147`), the dispatch arms (`:213-316`) and the determinant-influence hunk from `fe5e60a`. Keep `host.rs` and `bin/ql-field-host.rs` byte-identical to main.
  - The adapter changes from `b5cf15f`, `3db025b` and `fe5e60a`. They carry forward into QL-B unchanged except that `personal()` waits for the Nara ruling. Keeping `instrument-session.mjs` equal to main means O:I-A can vendor QL-A with no client change.
  - The `k8-bindings-v1.json:5-19` K² entry, `K2-EXPRESSION-BINDING.md`, and `K8-CONTINUOUS-CONTRACT.md:48-53` (reword neutrally).
- **Regenerate:** `python3 scripts/k8-census.py --write-receipt`, then `--check`, which updates `fixtures/kernel/k8-census-receipt-v1.json`.
- **Tests carried:**
  - C++: `cpp/tests/continuous_field.cpp`, section `shape_replacement_keeps_state_and_pcm`.
  - Rust with a real worker, via `--ignored` and `QL_FIELD_WORKER`: `tests/k8_reshape.rs` with `reshape_keeps_state_clock_and_pcm_and_moves_only_targets`, `named_initial_basis_is_echoed_and_admitted`, `worker_refuses_stale_and_structural_reshape_without_commit` and `coupled_reshape_and_declared_strike_versus_continuation`.
  - Plus `continuous::receipt::tests`.
- **CI that runs them:**
  - PR gate `ql-mef-rust.yml` runs `scripts/verify`:
    - `core`: fmt, clippy `-D warnings`, `cargo test --workspace --all-targets`. The ignored worker tests only compile here.
    - `native`: includes `check-m1-acceptance.py`.
    - `invariants`: `m-ledger.py check`, `k8-census.py --check`.
  - **The C++ reshape test and the `--ignored` worker tests run only in `kernel-k8-continuous.yml`.** That workflow is `workflow_call`/`workflow_dispatch`, called only by the weekly `cross-product.yml:105-106`. It must be **dispatched manually on the QL-A branch** before merge.
  - `kernel-m2.yml` (weekly/dispatch) must also be dispatched (§4).

### O:I-A: infrastructure only (new branch off O:I main `36c9e411`)

- **Take:**
  - All of `3647a825`, with `surfaces.json:361,382,414,415` moved to the QL-A merge sha.
  - From `35950ae0`: `oi_executable`/`resolve_oi` (`:280-330`), `main.rs:116-121` and test `:1964`.
  - From `1e400188`: `physicalFormActuator.ts` with the reason text neutralised, plus the `nara-embodiment-*` updates without the `/Vimarśā/` assertion.
  - The host world-lens hunks (`hostedApp.ts`, `PointCloudHost.tsx`, `app.ts` world lens and `instrumentOffered`).
  - The test hooks `projection.ts:41-48` and `app.ts` `probeSteps`.
  - The backcheck change `af35abb2:27-34,116` without the K² trace step.
  - PROVENANCE set to the QL-A merge sha. Vendored files stay byte-identical.
- **Defer to O:I-B:**
  - Compose: `native_expression.rs:36-40, 639-643, 803-1285` including `run_bounded` `340-500` (only compose calls it, so it would be dead code under clippy), `main.rs:39-42`, `nativeChannel.ts`, `types.ts:92-95` and `tests/native_expression_compose.rs`.
  - The `EXCHANGE_OPERATIONS` additions `:811-813`, since QL-A has no such host ops.
  - `k2.ts`, `controller.ts`, `nativeField.ts`, `nativeActuatorStanding.ts`, `tests/native-expression-{fixture,instrument.test,k2-browser}.mjs`, backcheck `:146-161`, and `3158636e`.
  - In O:I-B only compose's endpoint (`:1159-1210`) and the K² rows of C4 change. The rest carries over as-is.
- **Tests and CI:**
  - `verify.yml` (the PR gate): cli fmt, clippy, tests, which runs `development_field_s0.rs` `where_discloses_companions_beside_the_resolved_executable`, the `update_flow.rs` tests `a_build_stages_companions_beside_the_primary_and_rollback_keeps_pairs`, `a_build_that_omits_a_companion_links_nothing` and `adoption_takes_companions_from_the_primary_directory_or_refuses`, and the `product_command.rs` tests `quaternal_logic_declares_its_native_expression_companions` and `companion_declarations_are_validated`. It also runs the central-bootstrap job.
  - `desktop.yml` (path-scoped PR): kernel `cargo test --all-targets` covers `installed_ql_resolves_the_executable_and_every_present_companion`, `installed_ql_names_exactly_what_is_missing`, `the_where_query_is_bounded_and_its_failures_are_unavailable` and the `oi_resolves_…` test.
  - `native-expression-backcheck.yml` (path-scoped PR) runs against the QL-A cut.
  - **`native-expression-instrument.test.mjs` and `nara-embodiment-*.test.mjs` are not run by any workflow** (grep over `.github/workflows` finds nothing). O:I-B should wire them into `desktop.yml` or the backcheck.

### Omarchy replay scripts

These were in neither repo; they are preserved from a session scratchpad at `replay/ql-omarchy-replay.sh` (QL, 25 lines) and `replay/oi-omarchy-replay.sh` (O:I, 30 lines). Both target the old branches and run the `k2_*` tests. To keep them, add them to the repos (or the O:I gates) with the `k2_*` lines removed for the A PRs.

## 3. Conflicts with current main

| repo | `merge-tree --write-tree origin/main HEAD` | behind | conflicting paths |
|---|---|---|---|
| QL-MEF | clean (`34d25026…`) | 1 (`4669d8d`) | none |
| O:I | clean (`23c15562…`) | 11 | none |

Because the split rebuilds both branches off current main, textual rebase risk is nil. The conflicts that matter are semantic: the pins in §2 and the lock in §4.

## 4. Defects found in the "sound" infrastructure

1. **The M2 proof lock is broken by the ledger cache.**
   - `fixtures/kernel/m2-finite-proof-v1.json:21` pins `crates/ql-mef/src/m2_engine.rs` at `b22c97f6…`, which is main's digest. The branch's file is `346db167…`.
   - `scripts/k8-preservation.py` raises "changed numerical/source proof input" for any changed proof input other than `c/Makefile`. `kernel-m2.yml` runs it through `m2-ledger.py check`.
   - That workflow is weekly or dispatch only, so the PR gate does not catch this. The branch's own record of "m2_engine 13/13" does not exercise this check.
   - Fix in QL-A: leave `m2_engine.rs` byte-identical to main and put the once-per-process cache inside `m_ledger.rs` (for example, make `native_m_ledger()` memoise). Alternatively, publish a reviewed successor proof.
2. **The surfaces pin and the install script disagree** (§2, pin 1). Merging O:I without the pin bump makes `oi update` of quaternal-logic fail.
3. **The worker-level acceptance is not PR-gated in QL** (`kernel-k8-continuous.yml` is only reached through `cross-product.yml`). The instrument node tests are not CI-gated in O:I.
4. **There is an uncommitted edit in the O:I worktree** (`native-expression-k2-browser.mjs:84,140`) that is not on PR #545.
