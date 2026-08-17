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
