from pathlib import Path
import re

def edit(path,old,new):
 p=Path(path);s=p.read_text();assert s.count(old)==1,(path,old[:100],s.count(old));p.write_text(s.replace(old,new))
def replace_section(path,start,end,new):
 p=Path(path);s=p.read_text();a=s.index(start);b=s.index(end,a);p.write_text(s[:a]+new+s[b:])
p='crates/ql-mef/src/vak_composition.rs'
edit(p,'    pub member_focus: Option<MemberFocus>,\n    depth: usize,','    pub member_focus: Option<MemberFocus>,\n    pub selected_member: Option<StructuralParticipation>,\n    depth: usize,')
s=Path(p).read_text();s,n=re.subn(r'(member_focus: None,\s*)(depth[,:])',r'\1selected_member: None,\n                \2',s);assert n==2;n=Path(p).write_text(s)
edit(p,'fn selected_coordinate(w: &Whole) -> QlCoordinate {','fn selected_coordinate(w: &Whole) -> QlCoordinate {\n    if let Some(member) = &w.selected_member { return member.coordinate; }')
edit(p,'        w.member_focus = Some(focus.clone());','        w.member_focus = Some(focus.clone());\n        w.selected_member = None;')
edit(p,'        w.use_ref = into.into();\n        w.producing_refs.push(from.into());','''        if let WholeBody::Local(form) = &w.body {
            w.selected_member = form.members.iter().find(|m| m.coordinate == coordinate).cloned();
        }
        w.use_ref = into.into();
        w.producing_refs.push(from.into());''')
# Normalize the actual selected member for harmonic/MEF operations after a
# frame change; do not reinterpret the old CP input into a different member.
edit(p,'let local = focus_local(focus, whole.frame.lens);','let local = selected_local(whole);')
edit(p,'Some(focus) => {\n                let local = selected_local(whole);','Some(_) => {\n                let local = selected_local(whole);')
edit(p,'let absolute = if let Some(focus) = &whole.member_focus {','let absolute = if whole.member_focus.is_some() {')
edit(p,'focus_local(focus, whole.frame.lens).position','selected_local(whole).position')
with Path(p).open('a') as f:f.write('''
fn selected_local(w: &Whole) -> QlCoordinate {
    let c = selected_coordinate(w);
    let position = match w.frame.positions {
        PositionBasis::Local => c.position,
        PositionBasis::Absolute => QlPosition::new((c.position.value() + 6 - w.frame.lens.index()) % 6)
            .expect("modulo-six coordinate"),
    };
    QlCoordinate::new(position, c.face)
}
''')
# Reframing preserves the participating whole's identity in an existing parent
# relation. A returned NEW whole has a new subject/anchor and is not an alias.
replace_section(p,'    fn contains_use(&self, parent: &str, child: &str) -> Result<bool> {','    pub fn return_result(','''    fn contains_use(&self, parent: &str, child: &str) -> Result<bool> {
        let child_whole = self.whole(child)?;
        let mut aliases = BTreeSet::new();
        let mut pending = vec![child.to_owned()];
        while let Some(r) = pending.pop() {
            if !aliases.insert(r.clone()) { continue; }
            for source in &self.whole(&r)?.producing_refs {
                if let Some(w) = self.wholes.get(source) {
                    if w.binding.whole_ref == child_whole.binding.whole_ref
                        && w.binding.subject_ref == child_whole.binding.subject_ref {
                        pending.push(source.clone());
                    }
                }
            }
        }
        let mut pending = vec![parent.to_owned()];
        let mut seen = BTreeSet::new();
        while let Some(r) = pending.pop() {
            if aliases.contains(&r) { return Ok(true); }
            if !seen.insert(r.clone()) { continue; }
            if let WholeBody::Relation { row, column, .. } = &self.whole(&r)?.body {
                pending.extend([row.clone(), column.clone()]);
            }
        }
        Ok(false)
    }
''')
# A mere reframe of the same whole is not its own parent/child.
s=Path(p).read_text();old='source.use_ref != target.use_ref\n                    && self.contains_use';new='source.binding.whole_ref != target.binding.whole_ref\n                    && self.contains_use';assert s.count(old)==2,s.count(old);Path(p).write_text(s.replace(old,new))
# Fail closed before producing a CPF use when the conjugate member is absent.
edit(p,'        frame.face = face;\n        graph.reframe(&from, into, frame, basis.clone())?;','''        frame.face = face;
        let selected = if let Some(member) = &graph.whole(&from)?.selected_member {
            let target = QlCoordinate::new(member.coordinate.position, face);
            match &graph.whole(&from)?.body {
                WholeBody::Local(form) => Some(form.members.iter().find(|m| m.coordinate == target)
                    .cloned().ok_or_else(|| err("CPF counterpart is not disclosed"))?),
                _ => return Err(err("positioned CPF requires a local whole")),
            }
        } else { None };
        graph.reframe(&from, into, frame, basis.clone())?;
        if let Some(w) = graph.wholes.get_mut(into) {
            w.selected_member = selected;
            if let Some(focus) = &mut w.member_focus { focus.coordinate.face = face; }
        }''')
# Strengthen correspondence with the actual native execution step, in addition
# to the inherited observed-path contract; no external execution is inferred.
native='crates/ql-mef/src/vak_composition/native_path.rs'
edit(native,'        let value = NativePathCorrelation {','''        require(step.action_ref.as_deref() == Some(path.action_profile.action_ref.as_str())
            && step.method_ref.as_deref() == Some(path.method_ref.as_str()),
            "native step Action/Method differs from its execution path")?;
        require(path.action_profile.affordances.iter().any(|a| a.operator == step.expression.operator
            && a.horizon == step.expression.horizon), "native Action profile does not bind this operation")?;
        require(step.expression.world_ref == language.reading.world_ref
            && step.expression.project_ref == language.reading.project_ref
            && step.expression.focus_ref == language.reading.focus_ref, "native step changes contextual scope")?;
        for subject in &language.reading.subjects {
            if let VakExpressionSubject::Native(r) = subject {
                require(step.native_subject_refs.contains(r), "native step omits a producing subject")?;
            }
        }
        let value = NativePathCorrelation {''')
