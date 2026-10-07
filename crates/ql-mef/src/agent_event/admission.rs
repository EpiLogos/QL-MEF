//! Native admission against the owner-issued candidate field and exact basis.
use super::*;

fn decision_error(
    response: &DecisionResponse,
    event: &AgentEvent,
) -> Result<Option<String>, String> {
    if response.schema != "ql.agent-decision-response/v1" {
        return Ok(Some("unsupported decision response".into()));
    }
    if let Some(provider) = &response.provider {
        for (value, name) in [
            (&provider.provider_ref, "provider"),
            (&provider.model_ref, "model"),
            (&provider.model_revision, "model revision"),
            (&provider.runtime_revision, "runtime revision"),
        ] {
            if value.trim().is_empty() {
                return Ok(Some(format!("empty {name}")));
            }
        }
    }
    if response.outcome == DecisionOutcome::Unavailable {
        if !response.proposals.is_empty() {
            return Ok(Some("unavailable provider returned proposals".into()));
        }
        if response.reason.as_ref().is_none_or(|r| r.trim().is_empty()) {
            return Ok(Some("unavailable result lacks reason".into()));
        }
    } else if response.provider.is_none() {
        return Ok(Some(
            "learned response lacks provider/model/runtime identity".into(),
        ));
    }
    let mut heads = BTreeSet::new();
    for proposal in &response.proposals {
        if !heads.insert(&proposal.head_id) {
            return Ok(Some("duplicate learned head".into()));
        }
        if let Some(confidence) = proposal.confidence {
            if !confidence.is_finite() {
                return Err("nonfinite learned confidence cannot be retained as JSON".into());
            }
            if !(0.0..=1.0).contains(&confidence) {
                return Ok(Some("confidence outside probability range".into()));
            }
        }
        let mut labels = BTreeSet::new();
        if proposal
            .label_ids
            .iter()
            .any(|label| label.trim().is_empty() || !labels.insert(label))
        {
            return Ok(Some("empty or duplicate learned label".into()));
        }
        for span in &proposal.spans {
            let exact = span.start <= span.end
                && span.end <= event.material.text.chars().count()
                && event
                    .material
                    .text
                    .chars()
                    .skip(span.start)
                    .take(span.end - span.start)
                    .collect::<String>()
                    == span.text;
            if span.material_ref != event.material.r#ref
                || span.revision != event.material.revision
                || !exact
            {
                return Ok(Some(
                    "semantic evidence span does not match exact source/revision/codepoint range"
                        .into(),
                ));
            }
        }
    }
    Ok(None)
}

fn selection_holds(selection: &Value, chosen: &BTreeMap<String, BTreeSet<String>>) -> bool {
    let actual = selection["head_id"].as_str().and_then(|h| chosen.get(h));
    let expected: BTreeSet<_> = selection["label_ids"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let Some(actual) = actual else { return false };
    match selection["match"].as_str() {
        Some("any") => !actual.is_disjoint(&expected),
        Some("all") => expected.is_subset(actual),
        Some("exact") => *actual == expected,
        _ => false,
    }
}

fn constraint_holds(
    constraint: &Value,
    chosen: &BTreeMap<String, BTreeSet<String>>,
) -> Result<bool, String> {
    let conjunction = |key: &str| {
        constraint[key]
            .as_array()
            .is_some_and(|items| items.iter().all(|s| selection_holds(s, chosen)))
    };
    match constraint["kind"].as_str() {
        Some("implication") => Ok(!conjunction("antecedent") || conjunction("consequent")),
        Some("exclusion") => Ok(!conjunction("forbidden_together")),
        Some("legal-combination") => {
            Ok(constraint["allowed_tuples"].as_array().is_some_and(|rows| {
                rows.iter().any(|row| {
                    row.as_array()
                        .is_some_and(|items| items.iter().all(|s| selection_holds(s, chosen)))
                })
            }))
        }
        Some("candidate-restriction") => {
            let selection = &constraint["allowed"];
            let actual = selection["head_id"].as_str().and_then(|id| chosen.get(id));
            let allowed: BTreeSet<_> = selection["label_ids"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            Ok(actual
                .is_some_and(|labels| labels.iter().all(|label| allowed.contains(label.as_str()))))
        }
        _ => Err("unrecognised native constraint; refusal required".into()),
    }
}

fn constraint_heads(constraint: &Value) -> BTreeSet<String> {
    fn collect(value: &Value, found: &mut BTreeSet<String>) {
        if let Some(id) = value.get("head_id").and_then(Value::as_str) {
            found.insert(id.into());
        }
        match value {
            Value::Array(items) => {
                for item in items {
                    collect(item, found)
                }
            }
            Value::Object(items) => {
                for item in items.values() {
                    collect(item, found)
                }
            }
            _ => {}
        }
    }
    let mut found = BTreeSet::new();
    collect(constraint, &mut found);
    found
}

fn finish_admission(
    mut projection: EventProjection,
    response: DecisionResponse,
    admission_status: &'static str,
) -> Result<DecisionAdmission, String> {
    projection.harmonic["determination_digest"] = json!(value_digest(&projection.determination)?);
    Ok(DecisionAdmission {
        schema: "ql.agent-decision-admission/v1",
        response,
        admission_status,
        projection,
    })
}

/// Rebuild and validate against the current QL owners. No caller/provider frame
/// or purported admission is trusted. This function invokes no model or tool.
pub fn admit_decision(
    request: ProjectionRequest,
    response: DecisionResponse,
) -> Result<DecisionAdmission, String> {
    let mut projection = project_event(request.clone())?;
    if projection.decision_head_ids.is_empty() {
        return finish_admission(projection, response, "bypassed");
    }
    let frame_digest = value_digest(&projection.frame)?;
    let stale = response.event_basis_digest
        != projection.frame["event_basis_digest"]
            .as_str()
            .unwrap_or_default()
        || response.frame_digest != frame_digest
        || response.kernel_basis != projection.frame["kernel_basis"];
    if stale {
        projection.determination["status"] = json!("stale");
        for head in projection.determination["unresolved"]
            .as_array_mut()
            .ok_or("missing dispositions")?
        {
            head["reason"] = json!("late response has a different event/source/frame/kernel basis");
        }
        return finish_admission(projection, response, "stale");
    }
    if let Some(reason) = decision_error(&response, &projection.event)? {
        for head in projection.determination["unresolved"]
            .as_array_mut()
            .ok_or("missing dispositions")?
        {
            head["reason"] = json!(reason);
        }
        return finish_admission(projection, response, "refused");
    }
    if response.outcome == DecisionOutcome::Unavailable {
        projection.determination["status"] = json!("unavailable");
        for head in projection.determination["unresolved"]
            .as_array_mut()
            .ok_or("missing dispositions")?
        {
            head["reason"] = json!(response.reason);
        }
        if let Some(provider) = &response.provider {
            projection.determination["provider"] = json!(provider);
        }
        return finish_admission(projection, response, "unavailable");
    }
    let heads: BTreeMap<String, Value> = projection.frame["unresolved"]
        .as_array()
        .ok_or("missing heads")?
        .iter()
        .map(|h| (h["id"].as_str().unwrap_or_default().into(), h.clone()))
        .collect();
    if response
        .proposals
        .iter()
        .any(|p| !projection.decision_head_ids.contains(&p.head_id))
    {
        for head in projection.determination["unresolved"]
            .as_array_mut()
            .ok_or("missing dispositions")?
        {
            head["reason"] = json!("provider attempted an unrequested or input-required head");
        }
        return finish_admission(projection, response, "refused");
    }
    let chosen: BTreeMap<_, _> = response
        .proposals
        .iter()
        .map(|p| (p.head_id.clone(), p.label_ids.iter().cloned().collect()))
        .collect();
    let mut refused = BTreeMap::<String, (String, Vec<String>)>::new();
    for proposal in &response.proposals {
        let head = &heads[&proposal.head_id];
        let legal: BTreeSet<_> = head["labels"]
            .as_array()
            .ok_or("missing labels")?
            .iter()
            .filter_map(|v| v["id"].as_str())
            .collect();
        let count = proposal.label_ids.len() as u64;
        let valid = count >= head["cardinality"]["min"].as_u64().unwrap_or(1)
            && count <= head["cardinality"]["max"].as_u64().unwrap_or(0)
            && proposal
                .label_ids
                .iter()
                .all(|label| legal.contains(label.as_str()));
        if !valid {
            refused.insert(
                proposal.head_id.clone(),
                (
                    "proposal violates native candidate field/cardinality".into(),
                    vec!["ql:agent:decision-frame:v1:legal-candidates".into()],
                ),
            );
        }
    }
    let mut applications = Vec::new();
    for constraint in projection.frame["constraints"]
        .as_array()
        .ok_or("missing constraints")?
    {
        let involved = constraint_heads(constraint);
        // Omitted heads are unresolved, rather than guessed empty selections.
        let applicable = involved
            .iter()
            .all(|id| chosen.contains_key(id) && !refused.contains_key(id));
        let passes = !applicable || constraint_holds(constraint, &chosen)?;
        applications.push(json!({"constraint_id":constraint["id"],
            "result":if !applicable{"not-applicable"}else if passes{"passed"}else{"refused"},
            "reason":if !applicable{"some participating heads remain unresolved or refused"}else if passes{"native constraint satisfied"}else{"native constraint violated; participating proposals refused"}}));
        if applicable && !passes {
            for id in involved {
                refused.insert(
                    id,
                    (
                        "native combination constraint violated".into(),
                        vec![constraint["rule_ref"].as_str().unwrap_or_default().into()],
                    ),
                );
            }
        }
    }
    // Admission is conservative across connected unresolved constraints. One
    // half of an unevaluated combination must not become an operative choice.
    let mut pending = BTreeSet::new();
    for constraint in projection.frame["constraints"]
        .as_array()
        .ok_or("missing constraints")?
    {
        let involved = constraint_heads(constraint);
        if involved
            .iter()
            .any(|id| !chosen.contains_key(id) || refused.contains_key(id))
        {
            pending.extend(involved.into_iter().filter(|id| !refused.contains_key(id)));
        }
    }
    let mut validated = Vec::new();
    let mut unresolved = Vec::new();
    let mut refusals = Vec::new();
    for (id, head) in &heads {
        let proposal = response.proposals.iter().find(|p| p.head_id == *id);
        if let Some((reason, rules)) = refused.get(id) {
            let proposal = proposal.ok_or("refusal lacks original proposal")?;
            refusals.push(json!({"head_id":id,"label_ids":proposal.label_ids,"origin":"learned","reason":reason,"rule_refs":rules}));
        } else if let Some(proposal) =
            proposal.filter(|p| !p.label_ids.is_empty() && !pending.contains(id))
        {
            let mut rules = vec![json!("ql:agent:decision-frame:v1:legal-candidates")];
            for constraint in projection.frame["constraints"]
                .as_array()
                .ok_or("missing constraints")?
            {
                if constraint_heads(constraint).contains(id) {
                    rules.push(constraint["rule_ref"].clone());
                }
            }
            validated.push(
                json!({"field":head["field"],"value":proposal.label_ids,"origin":"learned",
                "proposal_head":id,"kernel_rule_refs":rules,"basis_digest":frame_digest}),
            );
        } else {
            unresolved.push(json!({"head_id":id,"reason":if pending.contains(id){"connected constraint remains unresolved"}
                else if proposal.is_some(){"provider abstained; confidence is not source evidence"}else{"no learned result supplied"}}));
        }
    }
    projection.determination["learned"] = json!(response.proposals);
    projection.determination["provider"] = json!(response.provider);
    projection.determination["validated"] = json!(validated);
    projection.determination["unresolved"] = json!(unresolved);
    projection.determination["refused_candidates"] = json!(refusals);
    projection.determination["constraint_application"] = json!(applications);
    projection.determination["status"] = json!(if validated.is_empty() {
        if refusals.is_empty() {
            "unresolved"
        } else {
            "refused"
        }
    } else if unresolved.is_empty() && refusals.is_empty() {
        "determined"
    } else {
        "partial"
    });
    if let Err(reason) = complete_validated(&mut projection, &request, &validated) {
        // Preserve the exact impossible answer. A failed deterministic
        // completion cannot leave a tentative admission or harmonic state.
        let mut failed = project_event(request)?;
        failed.determination["learned"] = json!(response.proposals);
        failed.determination["provider"] = json!(response.provider);
        let refusals: Vec<_> = response
            .proposals
            .iter()
            .filter(|p| !p.label_ids.is_empty())
            .map(|p| {
                json!({
            "head_id":p.head_id,"label_ids":p.label_ids,"origin":"learned","reason":reason,
            "rule_refs":["ql:agent:decision-frame:v1:validated-completion"]})
            })
            .collect();
        let refused_heads: BTreeSet<_> = refusals
            .iter()
            .filter_map(|r| r["head_id"].as_str())
            .collect();
        failed.determination["unresolved"]
            .as_array_mut()
            .ok_or("missing dispositions")?
            .retain(|h| !refused_heads.contains(h["head_id"].as_str().unwrap_or_default()));
        failed.determination["refused_candidates"] = json!(refusals);
        failed.determination["constraint_application"] = json!(applications);
        failed.determination["status"] = json!("refused");
        return finish_admission(failed, response, "refused");
    }
    let remaining = projection.determination["unresolved"]
        .as_array()
        .ok_or("missing dispositions")?;
    if !validated.is_empty() {
        projection.determination["status"] =
            json!(if remaining.is_empty() && refusals.is_empty() {
                "determined"
            } else {
                "partial"
            });
    }
    let status = if validated.is_empty() {
        if refusals.is_empty() {
            "unresolved"
        } else {
            "refused"
        }
    } else {
        "admitted"
    };
    finish_admission(projection, response, status)
}

fn complete_validated(
    projection: &mut EventProjection,
    request: &ProjectionRequest,
    validated: &[Value],
) -> Result<(), String> {
    if validated.is_empty() {
        return Ok(());
    }
    let mut branches = vec![Vec::<EventFact>::new()];
    for accepted in validated {
        let field = accepted["field"]
            .as_str()
            .ok_or("validated field missing")?;
        let mut expanded = Vec::new();
        for branch in branches {
            for label in accepted["value"]
                .as_array()
                .ok_or("validated selection missing")?
            {
                let mut branch = branch.clone();
                branch.push(EventFact {
                    field: field.into(),
                    value: label.clone(),
                    origin: "validated".into(),
                    basis_refs: vec![format!(
                        "{}#{}",
                        projection.frame["frame_ref"].as_str().unwrap_or_default(),
                        accepted["proposal_head"].as_str().unwrap_or_default()
                    )],
                    rule_ref: None,
                });
                expanded.push(branch);
            }
        }
        if expanded.len() > 128 {
            return Err("native semantic completion exceeds bounded branch count".into());
        }
        branches = expanded;
    }
    let mut readings = Vec::new();
    for branch in &branches {
        let mut request = request.clone();
        request.requested_heads.clear();
        let result = project_fields(request, branch)?;
        readings.push(json!({"selections":branch.iter().map(|f|json!({"field":f.field,"value":f.value})).collect::<Vec<_>>(),
            "derived":result.determination["derived"],"harmonic":result.harmonic["harmonic"],"missing_inputs":result.missing_inputs}));
    }
    if readings.len() == 1 {
        projection.missing_inputs = readings[0]["missing_inputs"]
            .as_array()
            .ok_or("missing input readout")?
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
        let basis: Vec<_> = branches[0]
            .iter()
            .flat_map(|f| f.basis_refs.iter().cloned())
            .collect();
        let fixed: BTreeSet<_> = projection.determination["observed"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(
                projection.determination["derived"]
                    .as_array()
                    .into_iter()
                    .flatten(),
            )
            .filter_map(|f| f["field"].as_str().map(str::to_owned))
            .collect();
        for mut derived in readings[0]["derived"]
            .as_array()
            .ok_or("completion derivations missing")?
            .iter()
            .cloned()
        {
            if !fixed.contains(derived["field"].as_str().unwrap_or_default()) {
                derived["basis_refs"]
                    .as_array_mut()
                    .ok_or("derivation refs missing")?
                    .extend(basis.iter().map(|r| json!(r)));
                projection.determination["derived"]
                    .as_array_mut()
                    .ok_or("determination derivations missing")?
                    .push(derived);
            }
        }
        let completed_fields: BTreeSet<_> = projection.determination["derived"]
            .as_array()
            .ok_or("missing derivations")?
            .iter()
            .filter_map(|f| f["field"].as_str().map(str::to_owned))
            .collect();
        let completed_heads: BTreeSet<_> = projection.frame["unresolved"]
            .as_array()
            .ok_or("missing heads")?
            .iter()
            .filter(|h| completed_fields.contains(h["field"].as_str().unwrap_or_default()))
            .filter_map(|h| h["id"].as_str().map(str::to_owned))
            .collect();
        projection.determination["unresolved"]
            .as_array_mut()
            .ok_or("missing dispositions")?
            .retain(|h| !completed_heads.contains(h["head_id"].as_str().unwrap_or_default()));
        projection.harmonic["harmonic"] = readings[0]["harmonic"].clone();
    } else {
        let mut remaining = BTreeSet::new();
        for reading in &readings {
            remaining.extend(
                reading["missing_inputs"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .map(str::to_owned),
            );
        }
        projection.missing_inputs = remaining.into_iter().collect();
        let reading = fact(
            "candidate-projections",
            json!(readings),
            &projection.event,
            "ql:agent:decision-frame:v1:ambiguity-preserving-completion",
        );
        projection.harmonic["harmonic"]
            .as_array_mut()
            .ok_or("harmonic facts missing")?
            .push(json!(reading));
    }
    projection.harmonic["formal"] = json!(
        projection.determination["observed"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(
                projection.determination["derived"]
                    .as_array()
                    .into_iter()
                    .flatten()
            )
            .chain(validated.iter())
            .cloned()
            .collect::<Vec<_>>()
    );
    Ok(())
}
