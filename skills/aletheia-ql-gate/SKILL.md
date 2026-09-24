---
name: aletheia-ql-gate
description: "METHOD: Gate 1 of Aletheia's six — check the Quaternal and Vāk coordinate integrity of a produced artifact: coordinate notation and family placement, context-frame codes, Vāk refs that actually resolve, QL shape laws, and provenance on every coordinate reference. Returns aligned, annotated, hold or redirect. Use before an artifact that names coordinates is returned, promoted or composed further."
---

# Aletheia QL gate (Gate 1)

## Contract metadata

- Semantic ref: `ql:skill:aletheia-ql-gate` (`skill/ql/aletheia-ql-gate`)
- Native checks: `ql context-frame list --json`; `ql vak locate <vak-ref> --json` and `ql vak context <vak-ref> [depth]`; `ql kernel apply <operator> <ql-address>`; `aikit wiki-shape validate <wiki-file>` for QL-shaped constellations; the shape contract `fixtures/kernel/ql-shape-contract-v1.json` (`skill/ql/ql-foundations`).
- Source: `Body/S/S4/ta-onta/S4-5p-aletheia/S5'/skills/aletheia-ql-gate/SKILL.md`, blob `86af36e294c85df2f27559d07f2d67e7a0971637` (pinned and HEAD agree), with the earlier stub `S5'/skills/gates/aletheia-ql-gate.md` (HEAD blob `5ecaef69…`).
- Gate family: Gate 1 QL · 2 M · 3 S · 4 M′ · 5 Rupa · 6 Collaboration. Result vocabulary for all six: `aligned | annotated | hold | redirect`.
- Primary contact: Nous (`agent/anima-nous`). Moirai pattern: Klotho first — assert and validate the traces. Frame: CT3, CP 4.5. Human in loop: no.

## Checks

1. **Notation.** Coordinates use the canonical forms (`#`, P0–P5, P0′–P5′, compound addresses such as `#0-4.0/1/2`); context frames use the `ql context-frame list` codes and expressions; C′ blocks use `skill/ql/vak-coordinate-frame`.
2. **Resolution.** Every Vāk ref resolves (`ql vak locate`); every kernel address is well formed (`ql kernel apply conjugate-address <address>` succeeds).
3. **Shape laws.** An address never asserts a semantic relation by itself; pair-family provenance is kept; partial shapes are valid disclosed wholes; generated semantic content is attributed to its generating client. Constellations pass `aikit wiki-shape validate`.
4. **Provenance.** Every coordinate reference in the body is a resolvable link or source ref, not a bare label.
5. **C′ mapping.** The artifact's Vāk block agrees with what `ql vak compose` accepted for it, if it was composed.

The old vault frontmatter law (`{family}_{n}_{semantic}` keys, `c_0_source_coordinates`, banned `bimbaCoordinate`/`pos_*`) belonged to the retired Obsidian vault; apply it only to material still in that form, and say so.

## Result

- `aligned` — all checks pass.
- `annotated` — passes with named, non-blocking notes.
- `hold` — a check fails that the producer can fix; name it. Repeated holds escalate to `aletheia-collab-gate`.
- `redirect` — the artifact belongs to another gate or owner; name which.

```text
GATE 1 (QL): <artifact ref>  -> aligned|annotated|hold|redirect
Checks: notation … resolution … shape … provenance … c-prime …
Evidence: <commands run and results>
```
