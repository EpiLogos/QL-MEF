"""Temporary, exact-input integration for #138; removed by its verification run."""
from pathlib import Path
import hashlib

def edit(path, old, new):
    p=Path(path); text=p.read_text()
    assert text.count(old)==1, (path,old[:100],text.count(old))
    p.write_text(text.replace(old,new))

p='crates/ql-mef/src/vak_composition.rs'
edit(p,'    pub basis: Vec<Basis>,\n}\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct GeometryReading', '    pub basis: Vec<Basis>,\n    pub source_returns: Vec<AnchorReturn>,\n}\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct GeometryReading')
edit(p,'    pub contribution: Option<AgentContribution>,\n    pub basis: Basis,','    pub contribution: Option<AgentContribution>,\n    pub context: Option<ReflectiveDerivation>,\n    pub basis: Basis,')
edit(p,'    pub producing_reading: FramedReading,\n    pub basis: Basis,','    pub producing: Determination,\n    pub target_basis: Vec<Basis>,\n    pub relation_evidence: Option<String>,\n    pub basis: Basis,')
edit(p,'        let depth = row.depth.max(col.depth) + 1;', '''        for w in [row, col] {
            if let WholeBody::Local(form) = &w.body {
                require(!form.members.is_empty() && !matches!(form.grain(), ql_core::ConstellationGrain::Other { .. }),
                    "recursive carrier requires a disclosed nonempty source axis")?;
            }
        }
        let depth = row.depth.max(col.depth) + 1;''')
edit(p,'format!("ql:carrier:1.0.0:relation-field:{}:by:{}", row.binding.shape_ref, col.binding.shape_ref)', 'format!("ql:carrier:1.1.0:relation-field:{}:{}:{}:{}", row.binding.shape_ref.len(), row.binding.shape_ref, col.binding.shape_ref.len(), col.binding.shape_ref)')
edit(p,'    pub fn read(&self, use_ref: &str, viewing_lens: LensId) -> Result<FramedReading> {\n        let whole = self.whole(use_ref)?;', '''    pub fn read(&self, use_ref: &str, viewing_lens: LensId) -> Result<FramedReading> {
        self.read_bounded(use_ref, viewing_lens, &mut MAX_OBJECTS.clone())
    }
    fn read_bounded(&self, use_ref: &str, viewing_lens: LensId, budget: &mut usize) -> Result<FramedReading> {
        require(*budget > 0, "expanded composition reading bound exceeded")?;
        *budget -= 1;
        let whole = self.whole(use_ref)?;''')
edit(p,'let a = self.read(row, viewing_lens)?;\n                let b = self.read(column, viewing_lens)?;', 'let a = self.read_bounded(row, viewing_lens, budget)?;\n                let b = self.read_bounded(column, viewing_lens, budget)?;')
edit(p,'        let pitch = whole.frame.pitch();','''        let mut source_returns = match &whole.body {
            WholeBody::Local(form) => form.returns.clone(), _ => Vec::new(),
        };
        for child in &children { extend_unique(&mut source_returns, &child.source_returns); }
        let pitch = whole.frame.pitch();''')
edit(p,'child_intervals, children, carrier_derivations, basis: whole.basis.clone() })','child_intervals, children, carrier_derivations, basis: whole.basis.clone(), source_returns })')
edit(p,'contribution: request.contribution, basis: request.basis, standing: VakStanding::Derived','contribution: request.contribution, context: None, basis: request.basis, standing: VakStanding::Derived')
start='    fn collect_sources(&self, registry: &VakRegistry, r: &str, into: &mut Vec<VakSourceProvenance>) -> Result<()> {'
end='    pub fn return_result(&mut self, input: ReturnInput) -> Result<()> {'
s=Path(p).read_text(); a=s.index(start); b=s.index(end)
s=s[:a]+'''    fn language_bindings(&self, r: &str) -> Result<Vec<&FullVakBinding>> {
        let mut pending = vec![r.to_owned()];
        let mut seen = BTreeSet::new();
        let mut result = Vec::new();
        while let Some(r) = pending.pop() {
            if !seen.insert(r.clone()) { continue; }
            let w = self.whole(&r)?;
            if let Some(l) = &w.language { result.push(l); }
            if let WholeBody::Relation { row, column, .. } = &w.body {
                pending.extend([row.clone(), column.clone()]);
            }
            pending.extend(w.producing_refs.iter().filter(|r| self.wholes.contains_key(*r)).cloned());
        }
        Ok(result)
    }
    fn collect_sources(&self, registry: &VakRegistry, r: &str, into: &mut Vec<VakSourceProvenance>) -> Result<()> {
        for l in self.language_bindings(r)? { extend_unique(into, &l.sources(registry)?); }
        Ok(())
    }
    fn contains_use(&self, parent: &str, child: &str) -> Result<bool> {
        let mut pending = vec![parent.to_owned()];
        let mut seen = BTreeSet::new();
        while let Some(r) = pending.pop() {
            if r == child { return Ok(true); }
            if !seen.insert(r.clone()) { continue; }
            if let WholeBody::Relation { row, column, .. } = &self.whole(&r)?.body {
                pending.extend([row.clone(), column.clone()]);
            }
        }
        Ok(false)
    }
''' + s[b:];Path(p).write_text(s)
edit(p,'            producing_reading: d.reading.clone(), basis: input.basis, standing: VakStanding::Derived','            producing: d.clone(), target_basis: target.basis.clone(), relation_evidence: input.relation_evidence, basis: input.basis, standing: VakStanding::Derived')
edit(p,'    pub r_path: Option<VakRPath>,\n    pub operations: Vec<ReflectiveReceipt>,','    pub r_path: Option<VakRPath>,\n    pub focus_path: Vec<Axis>,\n    pub thread_bindings: Vec<ThreadBinding>,\n    pub operations: Vec<ReflectiveReceipt>,')
edit(p,'#[derive(Debug, Clone)]\npub struct ReflectiveReceipt', '#[derive(Debug, Clone, PartialEq, Eq)]\npub struct ReflectiveReceipt')
edit(p,'            thread: Vec::new(), r_path: None, operations: Vec::new()', '            thread: Vec::new(), r_path: None, focus_path: Vec::new(), thread_bindings: Vec::new(), operations: Vec::new()')
edit(p,'        self.allowed_operators = operators;','''        if let Some(w) = graph.wholes.get_mut(into) {
            if let Some(t) = w.transitions.last_mut() { t.operation = VakFamily::Cpf.relation_id().as_str().into(); }
        }
        self.category_ground.face = face;
        self.allowed_operators = operators;''')
edit(p,'        self.focus_use = focus.use_ref.clone();','        self.focus_use = focus.use_ref.clone();\n        self.focus_path = path.to_vec();')
edit(p,'        paths: &[Vec<Axis>], act: VakDivineAct, basis: Basis) -> Result<Vec<FramedReading>> {', '        bindings: &[ThreadBinding], act: VakDivineAct, basis: Basis) -> Result<Vec<FramedReading>> {')
edit(p,'require(!paths.is_empty() && paths.len() <= MAX_DEPTH, "CFP requires a bounded nonempty thread")?;', 'require(!bindings.is_empty() && bindings.len() <= MAX_DEPTH, "CFP requires a bounded nonempty thread")?;')
edit(p,'        for path in paths {\n            let w = graph.position(&self.focus_use, path)?;\n            if let Some(l) = &w.language { self.admit(l)?; }','''        require(source_path.steps.len() == bindings.len(), "CFP must bind every actual source R-path step")?;
        for (step, binding) in source_path.steps.iter().zip(bindings) {
            require(step.vak_ref == binding.source_step, "CFP source step order/identity mismatch")?;
            let w = graph.position(&self.focus_use, &binding.path)?;
            for l in graph.language_bindings(&w.use_ref)? { self.admit(l)?; }
            let language = w.language.as_ref().ok_or_else(|| err("CFP source step needs an attributable whole interpretation"))?;
            require(language.sources(registry)?.iter().any(|s| s.coordinate == binding.source_step),
                "CFP source step is not a declared relation of this whole")?;''')
edit(p,'        self.thread = thread.clone();','        self.thread = thread.clone();\n        self.thread_bindings = bindings.to_vec();')
old='''        if let Some(l) = &request.language { self.admit(l)?; }
        graph.determine(registry, request)'''
new='''        require(self.allowed_operators.contains(&VakRelationOp::Express), "CPF excludes determination")?;
        for l in graph.language_bindings(&request.whole_use)? { self.admit(l)?; }
        if let Some(l) = &request.language { self.admit(l)?; }
        let mut readings = Vec::new();
        let mut sources = Vec::new();
        for r in &self.thread {
            for l in graph.language_bindings(r)? { self.admit(l)?; }
            readings.push(graph.read(r, request.viewing_lens)?);
            graph.collect_sources(registry, r, &mut sources)?;
        }
        if let Some(c) = &request.contribution {
            require(self.thread.iter().all(|r| c.input_refs.contains(r)), "Agent contribution omits a CFP input")?;
        }
        if let Some(path) = &self.r_path {
            for r in std::iter::once(&path.act_ref).chain(path.steps.iter().map(|s| &s.vak_ref)) {
                let source = registry.locate(r).ok_or_else(|| err("missing R-path source"))?;
                extend_unique(&mut sources, &[source.source.clone()]);
            }
        }
        let reference = request.reference.clone();
        graph.determine(registry, request)?;
        let d = graph.determinations.get_mut(&reference).expect("just inserted determination");
        extend_unique(&mut d.sources, &sources);
        d.context = Some(ReflectiveDerivation { category_ground: self.category_ground,
            focus_path: self.focus_path.clone(), allowed_operators: self.allowed_operators.clone(),
            content_fields: self.content_fields.clone(), thread_bindings: self.thread_bindings.clone(),
            r_path: self.r_path.clone(), operations: self.operations.clone(), readings });
        Ok(())'''
edit(p,old,new)
edit(p,'/// Whole source neighbourhood traversal is inherited, not reimplemented here.', '''#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadBinding { pub source_step: VakRef, pub path: Vec<Axis> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectiveDerivation {
    pub category_ground: QlCoordinate,
    pub focus_path: Vec<Axis>,
    pub allowed_operators: Vec<VakRelationOp>,
    pub content_fields: Vec<VakContextField>,
    pub thread_bindings: Vec<ThreadBinding>,
    pub r_path: Option<VakRPath>,
    pub operations: Vec<ReflectiveReceipt>,
    pub readings: Vec<FramedReading>,
}

/// Whole source neighbourhood traversal is inherited, not reimplemented here.''')
# Avoid clone-on-copy and temporary-reference lints in the bounded reader.
edit(p,'self.read_bounded(use_ref, viewing_lens, &mut MAX_OBJECTS.clone())','let mut budget = MAX_OBJECTS;\n        self.read_bounded(use_ref, viewing_lens, &mut budget)')
edit('crates/ql-mef/src/lib.rs','mod vak_oi;','mod vak_oi;\npub mod vak_composition;')
