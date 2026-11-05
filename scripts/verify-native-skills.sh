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
team_methods="vak-evaluate anima-orchestration prospective-retrospective-pass klein-mode ouroboros symbolic-protein-reading
  relational-graph-traverse wikilink-resonance-scan cross-source-dissonance-detect gnosis-retrieve thought-distil
  anansi aletheia-stack-traverse aletheia-module-audit aletheia-improvement-propose aletheia-self-extend
  aletheia-plugin-integrate aletheia-ql-gate aletheia-m-gate aletheia-s-gate aletheia-m-prime-gate
  aletheia-rupa-gate aletheia-collab-gate anima-expressive-composition aletheia-expressive-return
  chronos-act-continuity"
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

# Ta-Onta's practice (O-I #754; docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md): the
# Methodology and its six Methods are Skills like the team's, and the registry SkillSets beside
# skills/ name each set in QL's own namespace, with members that are skills in this tree.
ta_onta_methods="ta-onta-enter ta-onta-form ta-onta-find ta-onta-time ta-onta-compose ta-onta-return"
for folder in ta-onta-methodology $ta_onta_methods; do
  skill="skills/$folder/SKILL.md"
  test -f "$skill"
  head -n 1 "$skill" | grep -qx -- '---'
  grep -qx "name: $folder" "$skill"
  grep -q '^## Contract metadata' "$skill"
  grep -q "Semantic ref: \`ql:skill:$folder\`" "$skill"
  grep -q 'Source:' "$skill"
done
grep -q '^description: "METHODOLOGY: ' skills/ta-onta-methodology/SKILL.md
for folder in $ta_onta_methods; do
  grep -q '^description: "METHOD: ' "skills/$folder/SKILL.md"
done
# The law the Methodology carries (FF §5, VAK §12): the powers take part at once, no call is
# compulsory, Return prepares Recognition, and repetition promotes nothing.
grep -q 'take part at once' skills/ta-onta-methodology/SKILL.md
grep -q 'no call is compulsory' skills/ta-onta-methodology/SKILL.md
grep -q 'It is not an interface' skills/ta-onta-methodology/SKILL.md
grep -q 'Return prepares' skills/ta-onta-methodology/SKILL.md
grep -q 'prepare Recognition' skills/ta-onta-return/SKILL.md
grep -q 'nor repetition' skills/ta-onta-methodology/SKILL.md
grep -qx 'schema = 1' skillsets/index.toml
for set in ta-onta $ta_onta_methods; do
  grep -qx "semantic_ref = \"ql:$set\"" skillsets/index.toml
  grep -qx "directory = \"$set\"" skillsets/index.toml
  members="skillsets/$set/members"
  test -s "$members"
  while IFS= read -r member; do
    case "$member" in ''|'#'*) continue ;; esac
    case "$member" in skill/ql/*) ;; *) echo "$members: $member is not a QL skill" >&2; exit 1 ;; esac
    test -f "skills/${member#skill/ql/}/SKILL.md" || { echo "$members: $member has no skill" >&2; exit 1; }
  done < "$members"
done
grep -qx 'skill/ql/ta-onta-methodology' skillsets/ta-onta/members
for folder in $ta_onta_methods; do
  grep -qx "skill/ql/$folder" "skillsets/$folder/members"
  grep -q "\"ql:$folder\"," skillsets/index.toml
done
if grep -rq 'day-night-pass' skillsets skills/ta-onta-*; then
  echo "Ta-Onta still names the renamed day-night-pass" >&2
  exit 1
fi
echo "Ta-Onta practice: structural contract OK"

# Nara's oracle Methods (#258): each carries its protocol and routes readings
# through the native continuing journey, never a hand-written artifact.
for pair in "quaternal-tarot:quaternal_tarot_protocol.md" "quaternal-i-ching:quaternal_i_ching_protocol.md"; do
  folder="${pair%%:*}"
  skill="skills/$folder/SKILL.md"
  test -f "$skill"
  test -f "skills/$folder/resources/methods/${pair##*:}"
  grep -qx "name: $folder" "$skill"
  grep -q '^description: "METHOD: ' "$skill"
  grep -q '^## Contract metadata' "$skill"
  grep -q 'Source:' "$skill"
  grep -q '^## Verify' "$skill"
  grep -q 'nara.journey.apply' "$skill"
  grep -q 'Never edit journey state by hand' "$skill"
  if grep -q 'Pratibimba/Nara' "$skill"; then
    echo "$folder still teaches the retired vault artifact path" >&2
    exit 1
  fi
done
echo "Nara oracle Methods: structural contract OK"

# All domain practices and exact Method descriptions participate in UX coverage.
python3 scripts/check-ux-spine.py
