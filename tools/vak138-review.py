"""One-use #138 review repairs; native runner removes this before final checks.

Exact source anchors fail closed. This is not runtime or permanent CI machinery.
"""
from pathlib import Path


def edit(path, old, new):
    p = Path(path)
    text = p.read_text()
    assert text.count(old) == 1, (path, old[:100], text.count(old))
    p.write_text(text.replace(old, new))


p = 'crates/ql-mef/src/vak_composition.rs'
edit(p, 'mod native_path;', 'mod native_path;\nmod production;\npub use production::{MAX_SHAPE_REF_BYTES, ProductionLineage};')
start = '    fn language_bindings(&self, r: &str) -> Result<Vec<&FullVakBinding>> {'
end = '    fn contains_use(&self, parent: &str, child: &str) -> Result<bool> {'
s = Path(p).read_text()
a, b = s.index(start), s.index(end)
s = s[:a] + '''    fn language_bindings(&self, r: &str) -> Result<Vec<&FullVakBinding>> {
        let lineage = self.lineage(r)?;
        let mut result = Vec::new();
        for w in lineage.wholes {
            if let Some(l) = &w.language { result.push(l); }
        }
        for d in lineage.determinations {
            if let Some(l) = &d.language { result.push(l); }
        }
        for returned in lineage.returns {
            if let Some(l) = &returned.producing.language { result.push(l); }
        }
        Ok(result)
    }
    fn collect_sources(
        &self,
        registry: &VakRegistry,
        r: &str,
        into: &mut Vec<VakSourceProvenance>,
    ) -> Result<()> {
        for l in self.language_bindings(r)? {
            extend_unique(into, &l.sources(registry)?);
        }
        let lineage = self.lineage(r)?;
        for d in lineage.determinations { extend_unique(into, &d.sources); }
        for returned in lineage.returns { extend_unique(into, &returned.producing.sources); }
        Ok(())
    }
''' + s[b:]
Path(p).write_text(s)
old = '''        let shape_ref = carrier
            .as_ref()
            .map(RelationFieldComposition::shape_ref)
            .unwrap_or_else(|| {
                format!(
                    "ql:carrier:1.1.0:relation-field:{}:{}:{}:{}",
                    row.binding.shape_ref.len(),
                    row.binding.shape_ref,
                    col.binding.shape_ref.len(),
                    col.binding.shape_ref
                )
            });'''
new = '''        let shape_ref = match &carrier {
            Some(carrier) => carrier.shape_ref(),
            None => production::recursive_shape_ref(&row.binding.shape_ref, &col.binding.shape_ref)?,
        };'''
edit(p, old, new)
edit(p, '''        if let Some(l) = &d.language {
            if let Some(expected) = &l.expected_ground {''', '''        for l in source.language.iter().chain(d.language.iter()) {
            if let Some(expected) = &l.expected_ground {''')
edit(p, '''        w.binding.return_refs.push(return_ref.into());
        w.basis.push(basis);''', '''        w.binding.return_refs.push(return_ref.into());
        extend_unique(&mut w.basis, &returned.producing.reading.basis);
        extend_unique(&mut w.basis, std::slice::from_ref(&returned.producing.basis));
        extend_unique(&mut w.basis, std::slice::from_ref(&returned.basis));
        w.basis.push(basis);''')
# Historical language is provenance, not fresh authored Source, and can be
# explicitly read again without fabricating a constellation or a new parser.
production = Path('crates/ql-mef/src/vak_composition/production.rs')
assert not production.exists()
production.write_text('''//! Traversal of existing production relations, not a separate source store.
use super::*;

/// Prevent exponential allocation of recursively embedded shape refs before
/// allocation. Read-node and depth limits alone cannot bound this carrier text.
pub const MAX_SHAPE_REF_BYTES: usize = 8 * 1024 * 1024;

pub(super) fn recursive_shape_ref(row: &str, column: &str) -> Result<String> {
    let size = row.len().checked_add(column.len()).and_then(|n| n.checked_add(128));
    require(size.is_some_and(|n| n <= MAX_SHAPE_REF_BYTES), "recursive shape reference byte bound exceeded")?;
    Ok(format!("ql:carrier:1.1.0:relation-field:{}:{}:{}:{}", row.len(), row, column.len(), column))
}

/// Borrowed, bounded inspection of existing whole/determination/Return records.
/// Returned.producing is the archived determination at Return time, even if a
/// later native observation extends the separately inspectable determination.
#[derive(Debug)]
pub struct ProductionLineage<'a> {
    pub wholes: Vec<&'a Whole>,
    pub determinations: Vec<&'a Determination>,
    pub returns: Vec<&'a Returned>,
}

impl VakComposition {
    pub fn lineage(&self, root: &str) -> Result<ProductionLineage<'_>> {
        let mut result = ProductionLineage { wholes: Vec::new(), determinations: Vec::new(), returns: Vec::new() };
        let mut pending = vec![root.to_owned()];
        let mut seen = BTreeSet::new();
        while let Some(r) = pending.pop() {
            if !seen.insert(r.clone()) { continue; }
            require(seen.len() <= MAX_OBJECTS, "production lineage bound exceeded")?;
            if let Some(w) = self.wholes.get(&r) {
                result.wholes.push(w);
                if let WholeBody::Relation { row, column, .. } = &w.body {
                    pending.extend([row.clone(), column.clone()]);
                }
                pending.extend(w.producing_refs.iter().cloned());
            } else if let Some(d) = self.determinations.get(&r) {
                result.determinations.push(d);
                pending.push(d.whole_use.clone());
                if let Some(c) = &d.context {
                    pending.push(c.ground_use.clone());
                    pending.extend(c.readings.iter().map(|r| r.use_ref.clone()));
                }
            } else if let Some(returned) = self.returns.get(&r) {
                result.returns.push(returned);
                // Target is an explicit receiving ground, not a producing
                // input. Its exact basis is retained on Returned; do not
                // import unrelated target interpretations into the result.
                pending.extend([returned.source_use.clone(), returned.determination.clone()]);
            } else {
                return Err(err(format!("unknown production reference {r}")));
            }
        }
        Ok(result)
    }

    /// Supply an attributable interpretation OF this existing whole as a new
    /// immutable use. This permits a returned field to enter CFP again without
    /// flattening its form or promoting generated content into authored Source.
    pub fn interpret(
        &mut self,
        registry: &VakRegistry,
        from: &str,
        into: &str,
        language: FullVakBinding,
        basis: Basis,
    ) -> Result<()> {
        self.vacant(into)?;
        basis.validate()?;
        language.validate(registry)?;
        let mut w = self.whole(from)?.clone();
        require(language.addresses(&w.binding.subject_ref), "interpretation does not address the existing subject")?;
        require(matches!(language.reading.standing, VakStanding::Derived | VakStanding::Proposed),
            "new interpretation remains DERIVED or PROPOSED")?;
        w.use_ref = into.into();
        w.language = Some(language);
        w.producing_refs.push(from.into());
        w.basis.push(basis);
        self.wholes.insert(into.into(), w);
        Ok(())
    }
}
''')
cli = 'crates/ql-cli/src/vak_composition.rs'
edit(cli, '''                "position" => Ok(whole_view(''', '''                "lineage" => {
                    let lineage = graph.lineage(text(s, "reference")?).map_err(error)?;
                    Ok(json!({
                        "wholes": lineage.wholes.into_iter().map(whole_view).collect::<Vec<_>>(),
                        "determinations": lineage.determinations.into_iter().map(determination_view).collect::<Vec<_>>(),
                        "returns": lineage.returns.into_iter().map(return_view).collect::<Vec<_>>()
                    }))
                }
                "interpret" => {
                    let into = text(s, "into")?;
                    let language = parse_language(&s["language"])?
                        .ok_or_else(|| error("interpret requires a full-profile reading"))?;
                    graph.interpret(&registry, text(s, "from")?, into, language, parse_basis(&s["basis"])?).map_err(error)?;
                    Ok(whole_view(graph.whole(into).map_err(error)?))
                }
                "position" => Ok(whole_view(''')
# Extend the existing tests, with their native synthetic fixture helpers.
tests = Path('crates/ql-mef/tests/vak_composition.rs')
with tests.open('a') as f:
    f.write('''

#[test]
fn generated_only_sources_and_readings_survive_return_reentry() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "seed", ContextFrameId::Cf2, false);
    let mut l = language("seed", VakRelationOp::Express, VakContextField::Bimba);
    l.reading.relation_refs.push(VakRef::new("M0").unwrap());
    let mut req = request("generated", "seed");
    req.language = Some(l.clone());
    req.contribution = Some(AgentContribution { actor_ref: "test:Agent".into(), result_ref: "test:new-reading".into(),
        input_refs: vec!["seed".into()], evidence: vec!["test:supplied-interpretation".into()] });
    graph.determine(&registry, req).unwrap();
    graph.return_result(returning("first-return", "generated", "seed", GroundKind::Own)).unwrap();
    let mut b = basis("first-return"); b.provenance.standing_ref = "DERIVED".into();
    graph.offer_as_whole("first-return", "offered", "anchor:offered", b).unwrap();
    graph.determine(&registry, request("second", "offered")).unwrap();
    let second = graph.determination("second").unwrap();
    assert!(second.sources.iter().any(|s| s.coordinate.as_str() == "M0"));
    assert!(second.sources.iter().any(|s| s.coordinate == VakContextField::Bimba.source_ref()));
    let lineage = graph.lineage("second").unwrap();
    assert!(lineage.determinations.iter().any(|d| d.reference == "generated" && d.language.as_ref() == Some(&l)));
    assert_eq!(lineage.returns[0].producing.contribution.as_ref().unwrap().result_ref, "test:new-reading");
    assert_eq!(lineage.returns[0].route.through_anchor_ref, "anchor:seed");
    // The generating interpretation remains operative in CT admission, rather
    // than vanishing at the offered-whole boundary.
    let mut c = CPrimeContext::enter(&graph, "offered", frame(ContextFrameId::Cf2).coordinate()).unwrap();
    c.ct(vec![VakContextField::Techne], basis("test:only-techne")).unwrap();
    assert!(c.determine(&mut graph, &registry, request("ct-refused", "offered")).is_err());
    assert!(graph.determination("ct-refused").is_err());
    assert!(graph.lineage("not-a-production-ref").is_err());
}

#[test]
fn returned_field_can_be_interpreted_and_used_by_a_source_path_again() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "a", ContextFrameId::Cf2, false);
    local(&mut graph, &registry, "b", ContextFrameId::Cf3, false);
    compose(&mut graph, &registry, "field", "a", "b", ContextFrameId::Cf5);
    graph.determine(&registry, request("d", "field")).unwrap();
    graph.return_result(returning("r", "d", "field", GroundKind::Own)).unwrap();
    let mut b = basis("r"); b.provenance.standing_ref = "DERIVED".into();
    graph.offer_as_whole("r", "offered", "anchor:offered", b).unwrap();
    let before = graph.whole("offered").unwrap().clone();
    graph.interpret(&registry, "offered", "read-again", language("d", VakRelationOp::Affirm, VakContextField::Techne), basis("test:read-again")).unwrap();
    assert_eq!(graph.whole("read-again").unwrap().body, before.body);
    assert_eq!(graph.whole("read-again").unwrap().binding, before.binding);
    assert_eq!(graph.whole("offered").unwrap(), &before);
    let path = registry.r_path(VakDivineAct::Freedom).unwrap();
    let bindings: Vec<_> = path.steps.iter().map(|s| ThreadBinding { source_step: s.vak_ref.clone(), path: vec![] }).collect();
    let mut c = CPrimeContext::enter(&graph, "read-again", frame(ContextFrameId::Cf5).coordinate()).unwrap();
    c.cfp(&graph, &registry, &bindings, VakDivineAct::Freedom, basis("test:reentered-cfp")).unwrap();
    c.determine(&mut graph, &registry, request("next", "read-again")).unwrap();
    assert_eq!(graph.determination("next").unwrap().context.as_ref().unwrap().readings[0].binding.subject_ref, "d");
    let mut forged = language("d", VakRelationOp::Affirm, VakContextField::Techne);
    forged.reading.standing = VakStanding::Source;
    assert!(graph.interpret(&registry, "offered", "forged", forged, basis("test:forged-source")).is_err());
    assert!(graph.whole("forged").is_err());
}

#[test]
fn active_whole_return_ground_cannot_be_bypassed_by_omitting_result_language() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "a", ContextFrameId::Cf2, false);
    let mut l = language("a", VakRelationOp::Affirm, VakContextField::Techne);
    l.expected_ground = Some("ground:elsewhere".into());
    graph.interpret(&registry, "a", "situated", l, basis("test:explicit-ground")).unwrap();
    graph.determine(&registry, request("d", "situated")).unwrap();
    assert!(graph.return_result(returning("wrong", "d", "situated", GroundKind::Own)).is_err());
    assert!(graph.returned("wrong").is_err());
}

#[test]
fn recursive_shape_text_is_bounded_before_allocation() {
    let registry = VakRegistry::from_authoritative_source().unwrap();
    let mut graph = VakComposition::default();
    local(&mut graph, &registry, "a", ContextFrameId::Cf2, false);
    let mut last = "a".to_owned();
    let mut refused = false;
    for index in 0..64 {
        let next = format!("doubling:{index}");
        let result = graph.compose(&registry, ComposeInput { use_ref: next.clone(), whole_ref: format!("anchor:{next}"),
            row: last.clone(), column: last.clone(), frame: frame(ContextFrameId::Cf2), ground_ref: "ground:a".into(),
            ground_face: QlFace::Direct, basis: basis("test:bounded-shape"), language: None });
        if let Err(e) = result {
            assert!(e.0.contains("shape reference byte bound"), "{e}");
            assert!(graph.whole(&next).is_err());
            refused = true;
            break;
        }
        assert!(graph.whole(&next).unwrap().binding.shape_ref.len() <= MAX_SHAPE_REF_BYTES);
        last = next;
    }
    assert!(refused);
}
''')
ctests = Path('crates/ql-cli/tests/vak_composition.rs')
with ctests.open('a') as f:
    f.write('''

#[test]
fn native_cli_lineage_retains_return_snapshots_and_replays_identically() {
    let mut request: Value = serde_json::from_slice(&std::fs::read(specimen()).unwrap()).unwrap();
    request["steps"].as_array_mut().unwrap().push(json!({"op":"lineage","reference":"offered"}));
    let file = std::env::temp_dir().join(format!("ql-lineage-{}.json", std::process::id()));
    std::fs::write(&file, serde_json::to_vec(&request).unwrap()).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["vak", "compose"]).arg(&file).arg("--json").output().unwrap();
    let _ = std::fs::remove_file(file);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value, ql_cli::vak_composition::execute_request(&request).unwrap());
    let lineage = &value["results"].as_array().unwrap().last().unwrap()["result"];
    assert!(lineage["determinations"].as_array().unwrap().iter().any(|d| d["reference"] == "d"));
    assert_eq!(lineage["returns"][0]["reference"], "r");
    assert_eq!(lineage["returns"][0]["producing"]["standing"], "DERIVED");
    assert_eq!(lineage["returns"][0]["route"]["anchorRef"], "anchor:outer");
}
''')
doc = 'docs/QL-VAK-KERNEL-RECONCILIATION.md'
edit(doc, '`offer_as_whole` makes the returned determination independently addressable', '`lineage` traverses the existing whole-use, determination and Return relations, including interpretations introduced during generation. The archived producing determination on a Return remains separately inspectable from later observations attached to the live determination. These source-bearing readings participate in subsequent CT/CPF admission and source collection; they are not discarded at re-entry. Receiving-ground provenance remains explicit on Return rather than being silently imported as producing input.\n\n`interpret` attaches a supplied DERIVED/PROPOSED full-profile reading to a new immutable use of the same actual whole. A returned relation field can thereby enter CFP again without being flattened or promoted to authored Source. Active whole and result readings both constrain their declared Return ground.\n\n`offer_as_whole` makes the returned determination independently addressable')
edit(doc, 'position_member,read,determine,return_result,offer_as_whole,record_native_observation}', 'position_member,read,interpret,lineage,determine,return_result,offer_as_whole,record_native_observation}')
edit(doc, 'and `inspect-context`. `determine`/`return`', '`interpret`, `lineage`, and `inspect-context`. `determine`/`return`')
edit(doc, 'and a 16 MiB CLI request.', 'an 8 MiB generated shape-ref bound checked before allocation, and a 16 MiB CLI request.')
with Path(doc).open('a') as f:
    f.write('\nReview regression commands: `cargo test -p ql-mef --test vak_composition` and `cargo test -p ql-cli --test vak_composition`. They cover generation-only source retention, source-path re-entry, explicit-ground refusal, pre-allocation recursive-ref bounds, and actual-binary lineage replay in addition to the initial feature cases.\n')
