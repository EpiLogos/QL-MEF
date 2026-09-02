from pathlib import Path

path = Path('crates/ql-mef/src/vak_oi.rs')
source = path.read_text()

old = '''    let mut changed_fields = BTreeSet::new();
    let mut recognised_vak_refs = BTreeSet::new();'''
new = '''    let mut changed_fields = Vec::new();
    let mut recognised_vak_refs = BTreeSet::new();'''
if source.count(old) != 1:
    raise SystemExit('Recognition changed-fields declaration anchor drifted')
source = source.replace(old, new, 1)

old = '''        changed_fields.insert(field_for_horizon(step.expression.horizon));
        for vak_ref in step'''
new = '''        let changed_field = field_for_horizon(step.expression.horizon);
        if !changed_fields.contains(&changed_field) {
            changed_fields.push(changed_field);
        }
        for vak_ref in step'''
if source.count(old) != 1:
    raise SystemExit('Recognition changed-field insertion anchor drifted')
source = source.replace(old, new, 1)

old = '        changed_fields: changed_fields.into_iter().collect(),'
new = '        changed_fields,'
if source.count(old) != 1:
    raise SystemExit('Recognition changed-fields return anchor drifted')
source = source.replace(old, new, 1)

path.write_text(source)
