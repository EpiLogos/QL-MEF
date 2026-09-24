#!/usr/bin/env bash
set -euo pipefail

operator="skills/ql-operation/SKILL.md"
developer="skills/refraction-adapter-authoring/SKILL.md"

for skill in "$operator" "$developer"; do
  test -f "$skill"
  head -n 1 "$skill" | grep -qx -- '---'
  grep -q '^name:' "$skill"
  grep -q '^description:' "$skill"
  grep -q '^## Contract metadata' "$skill"
done

for operation in capabilities locate refract relate synthesise; do
  grep -q "$operation" "$operator"
done
grep -q 'ql-mef:operator' "$operator"
grep -q 'Disabled' "$operator"
grep -q 'Optional' "$operator"
grep -q 'Required' "$operator"
grep -q 'provenance' "$operator"
grep -q 'alignment' "$operator"

grep -q 'ql-mef:refraction-developer' "$developer"
grep -q 'Bounded formal experiment procedure' "$developer"
grep -q 'explicit promotion' "$developer"
grep -q 'no-QL' "$developer"

echo "QL/MEF native Skills: structural contract OK"

# Anima/Aletheia team skills (docs/integrations/epi-logos/ANIMA-ALETHEIA-TEAMS.md): each is a
# catalogue-ready Skill whose name is its folder, carries its contract metadata and source, and is
# classified as a Method exactly when it is a procedure.
team_references="vak-coordinate-frame darshana repl"
team_methods="vak-evaluate anima-orchestration day-night-pass klein-mode ouroboros symbolic-protein-reading
  relational-graph-traverse wikilink-resonance-scan cross-source-dissonance-detect gnosis-retrieve thought-distil
  anansi aletheia-stack-traverse aletheia-module-audit aletheia-improvement-propose aletheia-self-extend
  aletheia-plugin-integrate aletheia-ql-gate aletheia-m-gate aletheia-s-gate aletheia-m-prime-gate
  aletheia-rupa-gate aletheia-collab-gate"
for folder in $team_references $team_methods; do
  skill="skills/$folder/SKILL.md"
  test -f "$skill"
  head -n 1 "$skill" | grep -qx -- '---'
  grep -qx "name: $folder" "$skill"
  grep -q '^## Contract metadata' "$skill"
  grep -q 'Source:' "$skill"
done
for folder in $team_methods; do
  grep -q '^description: "METHOD: ' "skills/$folder/SKILL.md"
done
for folder in $team_references; do
  if grep -q '^description: "METHOD:' "skills/$folder/SKILL.md"; then
    echo "reference Skill $folder is wrongly classified as a Method" >&2
    exit 1
  fi
done
test -x skills/darshana/scripts/darshana.py
for gate in ql m s m-prime rupa collab; do
  grep -q 'aligned' "skills/aletheia-$gate-gate/SKILL.md"
done
grep -q 'never bypassed' skills/aletheia-collab-gate/SKILL.md
echo "Anima/Aletheia team Skills: structural contract OK"

# All domain practices and exact Method descriptions participate in UX coverage.
python3 scripts/check-ux-spine.py
