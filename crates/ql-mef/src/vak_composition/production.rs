//! Traversal of existing production relations, not a separate source store.
use super::*;

/// Prevent exponential allocation of recursively embedded shape refs before
/// allocation. Read-node and depth limits alone cannot bound this carrier text.
pub const MAX_SHAPE_REF_BYTES: usize = 8 * 1024 * 1024;

pub(super) fn recursive_shape_ref(row: &str, column: &str) -> Result<String> {
    let size = row
        .len()
        .checked_add(column.len())
        .and_then(|n| n.checked_add(128));
    require(
        size.is_some_and(|n| n <= MAX_SHAPE_REF_BYTES),
        "recursive shape reference byte bound exceeded",
    )?;
    Ok(format!(
        "ql:carrier:1.1.0:relation-field:{}:{}:{}:{}",
        row.len(),
        row,
        column.len(),
        column
    ))
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
        let mut result = ProductionLineage {
            wholes: Vec::new(),
            determinations: Vec::new(),
            returns: Vec::new(),
        };
        let mut pending = vec![root.to_owned()];
        let mut seen = BTreeSet::new();
        while let Some(r) = pending.pop() {
            if !seen.insert(r.clone()) {
                continue;
            }
            require(
                seen.len() <= MAX_OBJECTS,
                "production lineage bound exceeded",
            )?;
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
        require(
            language.addresses(&w.binding.subject_ref),
            "interpretation does not address the existing subject",
        )?;
        require(
            matches!(
                language.reading.standing,
                VakStanding::Derived | VakStanding::Proposed
            ),
            "new interpretation remains DERIVED or PROPOSED",
        )?;
        w.use_ref = into.into();
        w.language = Some(language);
        w.producing_refs.push(from.into());
        w.basis.push(basis);
        self.wholes.insert(into.into(), w);
        Ok(())
    }
}
