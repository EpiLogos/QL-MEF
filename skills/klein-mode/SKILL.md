---
name: klein-mode
description: "METHOD: Run a Day pass and then a Night′ pass over that same execution, so the work reviews itself on one surface — which skills and commands fired, which routes were chosen, where coordination worked or wasted effort — and return structured refinement telemetry rather than narrative. Use after a significant orchestration run, when tuning skill or route selection from live results, or before the next cycle needs refinement data."
---

# Klein mode

## Contract metadata

- Semantic ref: `ql:skill:klein-mode` (`skill/ql/klein-mode`)
- Native evidence sources: Claude Code session record; `factory workflow inspect <state> <run-ref>`; `actuation stream usage`; `aikit recent`, `aikit failures`, `aikit history`; `git log`/`gh pr view` for code runs.
- Source: `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/klein-mode/SKILL.md` (pinned `70b54f07…`; HEAD `ee7e2a2b…`).
- Used by: `agent/anima-sophia` (affinity), `agent/anima`.

Day pass is prospective synthesis; Night′ is the retrospective inversion over what just gathered. They are two senses of reading one surface, run one after the other.

## When

- A significant orchestration run has completed and the system should learn from its own execution.
- Skill or route selection needs tuning from observed results rather than configuration.

## Procedure

1. **Day pass.** Do the work. Keep the actual trace: skills loaded, commands and Actions run, members dispatched (and in which thread form), elapsed time, failures and retries.
2. **Night′ pass over that trace.** Do not change state during the review. For each skill, command and route ask: did it fit, what did it cost, was a better native route available, what repeated, what was wasted.
3. **Emit telemetry**, then route it:
   - skill or route refinements → `skill/ql/aletheia-improvement-propose` (Gate 6 before anything permanent);
   - defects in a product → the owning repository as an issue or PR;
   - the telemetry itself → a NOW note with evidence refs:

```bash
ctrl --json action run projectcentral.now.return '{"project":"<Name>","actor":"agent/anima-sophia","kind":"note","subject":"klein telemetry: <run>","result":"<json below, compact>","status":"active","evidence_refs":"<run refs>"}'
```

## Output

```json
{
  "mode": "klein",
  "completed_at": "<ISO-8601>",
  "day_pass": {
    "skills_invoked": ["skill/ql/vak-evaluate"],
    "commands": ["ql vak compose request.json --json"],
    "agent_routing": ["agent/anima-nous -> agent/anima-logos -> agent/anima-sophia"],
    "thread_form": "CFP2",
    "duration_ms": 0
  },
  "night_pass": {
    "skill_effectiveness": {"skill/ql/vak-evaluate": {"rating": "effective|suboptimal|ineffective", "notes": "…"}},
    "route_adjustments": [{"route": "…", "suggestion": "…"}],
    "pattern_observations": ["…"],
    "execution_antipatterns": ["…"],
    "suggested_refinements": [{"target": "skill/…|route|member", "change": "…", "evidence": "…"}]
  }
}
```

## Limits

- One inversion level: the Night′ reviews the Day, not itself.
- The Day pass completes before the Night′ begins.
- Output is structured telemetry; a refinement is a proposal until recognised.
