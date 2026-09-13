#!/usr/bin/env python3
"""Check the explicitly pending H-review standing; never confer ratification."""
from pathlib import Path
import json

root = Path(__file__).resolve().parents[1]
base = root / 'docs/kernel-rebuild'
standing = json.loads((base / 'ux-publication-standing.json').read_text())
assert standing['publication'] == 'authorised_by_owner'
assert standing['H_ratification'] == 'pending'
assert standing['reviewer'] == 'Satya'
assert standing['publication_is_H_ratification'] is False
assert standing['tests_are_H_ratification'] is False
assert standing['human_experience_validated'] is False
for path in standing['files'] + standing['existing_skill_amendments']:
    assert (root / path).is_file(), path
for path in ['UX-INTENT-SOURCE-MINUTE.md', 'UX-SPINE-RECONCILIATION.md', 'AGENT-PRACTICE-AND-BOOTSTRAP.md']:
    assert 'H ratification pending (Satya)' in (base / path).read_text(), path
for path in ['AGENTS.md', 'skills/README.md']:
    assert 'H ratification pending (Satya)' in (root / path).read_text(), path
trace = json.loads((base / 'ux-spine-trace.json').read_text())
assert all(s['status'] == 'specified' and not s['evidence'] and s['human_assessment'] is None for s in trace['stories'])
print(json.dumps({'publication': 'authorised', 'H_ratification': 'pending', 'reviewer': 'Satya', 'scope': 'source markers only; no runtime or human-validation claim'}))
