---
name: ouroboros
description: "METHOD: Make the lemniscate self-fold operational — the system changes itself by delegating the change to a fresh agent in an isolated worktree (surgeon) while the parent (patient) holds context, constraints and acceptance, observes, and accepts or rejects the returned diff. Use for self-modification, multi-file refactors that need bounded scope, or parallel development across worktrees."
---

# Ouroboros

## Contract metadata

- Semantic ref: `ql:skill:ouroboros` (`skill/ql/ouroboros`)
- Native owners: AIKit `aikit task spawn --worktree|list|close`; Claude Code `Agent` with worktree isolation and `SendMessage`; herdr `herdr worktree create|list|remove`, `herdr agent list|prompt|read|wait`; git and `gh` for review and landing; Factory units for commissioned bead sequences.
- Replaces the Pleroma hooks `techne-spawn`, `techne-relay`, `techne-list`, `techne-close`, `pleroma-skill-proxy`, the `worktrunk` and `ralph-tui` skills, and the `preflight-validate.sh`/`postflight-verify.sh` gates.
- Source: `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/ouroboros/SKILL.md` (pinned `f3318f5b…`; HEAD `12948817…`).
- Used by: `agent/anima` and `agent/anima-psyche` (affinity); worktree mechanics by `agent/anima-techne-helper`.

The fold is the QL P↔P′ return made operational. The surgeon-in-worktree is one instance of it; the fold is the return, not the spawn.

## Roles

| Role | Who | Holds |
|---|---|---|
| Patient | the parent (Anima or Psyche in the lead body) | context, constraints, acceptance criteria; its codebase is what changes |
| Surgeon | a fresh agent in its own worktree | the bounded task; fresh context; produces commits |

The parent never edits the files under operation during the fold. It delegates, observes, and accepts or rejects.

## Procedure

1. **Preflight.** Confirm the task scope, the repository's protected-branch rule, and a clean starting point: `git -C <repo> fetch origin`, `ctrl git census <Project>` (read-only), no conflicting writer on the same files. Choose a worktree only for a real isolation reason (`skill/personal/agent-worktree-lifecycle`).
2. **Spawn the surgeon** with its own worktree and a bounded brief (goal, files in lane, gates to run, how to return):
   - separate process: `aikit task spawn <name> --agent claude --worktree`;
   - in-session: an `Agent` call with worktree isolation;
   - terminal workspace: `herdr worktree create …` then `herdr agent start …`.
   The surgeon receives its skills through AIKit's projection of its profile or skill set, not through a proxy of the parent's registry.
3. **Relay** only through the channel: `SendMessage` to the in-session agent, `herdr agent prompt` to a terminal agent, or `aikit gateway send` to a Position. No shared mutable state.
4. **Beads.** For a deterministic sequence, write each bead as a task-list item (one bounded unit: spawn, execute, verify, advance) — or as Factory workflow units with dependencies when the work is commissioned. No skipping, no reordering.
5. **Postflight.** Run the repository's own gates against the surgeon's worktree; check the diff is inside the lane; the surgeon lands through a branch and PR (`gh pr create`); protected `main` is never pushed directly.
6. **Close.** `aikit task close <task>` (it refuses an unclean worktree unless `--force` names the loss), `herdr worktree remove`, or the Agent tool's own cleanup. Worktree cleanup is mandatory on success and failure; a kept worktree is named with its reason.

## Output

```text
OUROBOROS: <task-name>
Surgeon: <agent / task id>  Worktree: <branch>  Bead: <n>/<total>
Status: spawning | active | relay | verifying | landing | complete | failed
Gates: <commands run and results>  Landing: <PR url or none>
```

## Limits

One surgeon, one worktree. Postflight gates must pass before a landing is proposed; merging is the repository's review process, not the fold's.
