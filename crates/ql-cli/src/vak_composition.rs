//! JSON command adapter, not a parser for AIKit's expression language.
use crate::CliError;
use ql_core::*;
use ql_mef::cprime_oikonomia as oik;
use ql_mef::vak_composition::*;
use ql_mef::vak_profile::{
    ContentPosition, ContentType, ContextSequence, InquiryDirection, Participation, ThreadForm,
};
use ql_mef::vak_thought_consumption as thought;
use ql_mef::vak_workflow_types::AuthoredFrame;
use ql_mef::*;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Read;

type R<T> = std::result::Result<T, CliError>;
fn error(e: impl std::fmt::Display) -> CliError {
    CliError(e.to_string())
}
fn text<'a>(v: &'a Value, key: &str) -> R<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| error(format!("missing/non-text {key}")))
}
fn optional(v: &Value, key: &str) -> R<Option<String>> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        _ => Ok(Some(text(v, key)?.into())),
    }
}
fn array<'a>(v: &'a Value, key: &str) -> R<&'a [Value]> {
    v.get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| error(format!("missing/non-array {key}")))
}
fn strings(v: &Value, key: &str) -> R<Vec<String>> {
    array(v, key)?
        .iter()
        .map(|x| {
            x.as_str()
                .filter(|s| !s.trim().is_empty())
                .map(str::to_owned)
                .ok_or_else(|| error(format!("invalid {key} reference")))
        })
        .collect()
}
fn pos(v: &Value) -> R<QlPosition> {
    let n = v
        .as_u64()
        .ok_or_else(|| error("position must be an integer"))?;
    QlPosition::new(u8::try_from(n).map_err(error)?).map_err(error)
}
fn face(v: &str) -> R<QlFace> {
    match v {
        "direct" => Ok(QlFace::Direct),
        "conjugate" => Ok(QlFace::Conjugate),
        _ => Err(error("face must be direct or conjugate")),
    }
}
fn coord(v: &Value) -> R<QlCoordinate> {
    Ok(QlCoordinate::new(
        pos(&v["position"])?,
        face(text(v, "face")?)?,
    ))
}
fn parse_frame(v: &Value) -> R<ActiveFrame> {
    Ok(ActiveFrame {
        id: ContextFrameId::ALL
            .into_iter()
            .find(|f| f.code() == v["id"].as_str().unwrap_or(""))
            .ok_or_else(|| error("unknown canonical CF"))?,
        lens: text(v, "lens")?.parse().map_err(error)?,
        basis: match text(v, "basis")? {
            "chromatic" => MusicalBasis::Chromatic,
            "fifths" => MusicalBasis::Fifths,
            _ => return Err(error("unknown harmonic basis")),
        },
        face: face(text(v, "face")?)?,
        positions: match text(v, "positions")? {
            "local" => PositionBasis::Local,
            "absolute" => PositionBasis::Absolute,
            _ => return Err(error("positions must be local or absolute")),
        },
    })
}
fn parse_basis(v: &Value) -> R<Basis> {
    let b = Basis {
        provenance: CallerProvenance::new(
            text(v, "caller")?,
            text(v, "source")?,
            text(v, "standing")?,
        )
        .map_err(error)?,
        revision: text(v, "revision")?.into(),
        evidence: strings(v, "evidence")?,
    };
    b.validate().map_err(error)?;
    Ok(b)
}
fn op(v: &str) -> R<VakRelationOp> {
    VakRelationOp::ALL
        .into_iter()
        .find(|o| o.glyph() == v)
        .ok_or_else(|| error("unknown Vāk relation operator"))
}
fn field(v: &str) -> R<VakContextField> {
    VakContextField::ALL
        .into_iter()
        .find(|f| f.symbol() == v)
        .ok_or_else(|| error("unknown M0-4 field"))
}
fn axes(v: &Value, key: &str) -> R<Vec<Axis>> {
    strings(v, key)?
        .iter()
        .map(|a| match a.as_str() {
            "row" => Ok(Axis::Row),
            "column" => Ok(Axis::Column),
            _ => Err(error("path step must be row or column")),
        })
        .collect()
}
fn ground(v: &str) -> R<GroundKind> {
    match v {
        "own" => Ok(GroundKind::Own),
        "parent" => Ok(GroundKind::Parent),
        "child" => Ok(GroundKind::Child),
        "other" => Ok(GroundKind::Other),
        "conjugate" => Ok(GroundKind::Conjugate),
        _ => Err(error("unknown ground relation")),
    }
}
/// The words a step names a CFP thread form and a CS profile by. The step
/// parsers and a unit's breakdown (`ql vak unit-steps`) read the same table.
const CFP_FORM_WORDS: [(&str, oik::CfpThreadForm); 6] = [
    ("base", oik::CfpThreadForm::Base),
    ("parallel", oik::CfpThreadForm::IndependentParallel),
    ("chain", oik::CfpThreadForm::Chain),
    ("fusion", oik::CfpThreadForm::Fusion),
    ("sustained", oik::CfpThreadForm::Sustained),
    ("nested", oik::CfpThreadForm::Nested),
];
const CS_PROFILE_WORDS: [(&str, oik::CsProfile); 6] = [
    ("full-traverse", oik::CsProfile::FullTraverse),
    (
        "quick-grounding-context",
        oik::CsProfile::QuickGroundingContext,
    ),
    (
        "ground-through-operation",
        oik::CsProfile::GroundThroughOperation,
    ),
    ("through-pattern", oik::CsProfile::ThroughPattern),
    ("context-focused", oik::CsProfile::ContextFocused),
    ("direct-synthesis", oik::CsProfile::DirectSynthesis),
];
fn word<T: PartialEq + Copy>(table: &[(&'static str, T)], value: T) -> &'static str {
    table
        .iter()
        .find(|(_, v)| *v == value)
        .map_or("", |(w, _)| w)
}
fn cfp_form(v: &str) -> R<oik::CfpThreadForm> {
    CFP_FORM_WORDS
        .iter()
        .find(|(w, _)| *w == v)
        .map(|(_, f)| *f)
        .ok_or_else(|| error("unknown CFP thread form"))
}
fn z_stage(v: &str) -> R<oik::CfpZStage> {
    Ok(match v {
        "compose" => oik::CfpZStage::Compose,
        "perform" => oik::CfpZStage::Perform,
        "record" => oik::CfpZStage::Record,
        "rehear" => oik::CfpZStage::Rehear,
        "recompose" => oik::CfpZStage::Recompose,
        _ => return Err(error("unknown Z stage")),
    })
}
fn cs_profile(v: &str) -> R<oik::CsProfile> {
    CS_PROFILE_WORDS
        .iter()
        .find(|(w, _)| *w == v)
        .map(|(_, p)| *p)
        .ok_or_else(|| error("unknown CS profile"))
}
fn cs_direction(v: &str) -> R<oik::CsDirection> {
    Ok(match v {
        "forward-synthesis" => oik::CsDirection::ForwardSynthesis,
        "returning-inquiry" => oik::CsDirection::ReturningInquiry,
        _ => return Err(error("unknown CS direction")),
    })
}
fn parse_language(v: &Value) -> R<Option<FullVakBinding>> {
    if v.is_null() {
        return Ok(None);
    }
    let g = &v["general"];
    let r = &v["reading"];
    if text(r, "contract")? != VAK_EXPRESSION_READING_CONTRACT {
        return Err(error("wrong reading contract"));
    }
    let subjects = array(r, "subjects")?
        .iter()
        .map(|s| match (s.get("native"), s.get("vak")) {
            (Some(_), None) => Ok(VakExpressionSubject::Native(text(s, "native")?.into())),
            (None, Some(_)) => Ok(VakExpressionSubject::Vak(
                VakRef::new(text(s, "vak")?).map_err(error)?,
            )),
            _ => Err(error("subject must carry exactly one native or vak ref")),
        })
        .collect::<R<Vec<_>>>()?;
    let standing = match text(r, "standing")? {
        "SOURCE" => VakStanding::Source,
        "AUTHORED-ARCHITECTURE" => VakStanding::AuthoredArchitecture,
        "IMPLEMENTATION" => VakStanding::Implementation,
        "OBSERVED" => VakStanding::Observed,
        "DERIVED" => VakStanding::Derived,
        "PROPOSED" => VakStanding::Proposed,
        _ => return Err(error("unknown reading standing")),
    };
    Ok(Some(FullVakBinding {
        general: VakGeneralExpressionEvidence {
            syntax_version: text(g, "syntaxVersion")?.into(),
            owner_revision: text(g, "ownerRevision")?.into(),
            resolve_path_identity: text(g, "resolvePathIdentity")?.into(),
            rendered: text(g, "rendered")?.into(),
            full_vak_rendering: text(g, "fullVakRendering")?.into(),
            evidence: strings(g, "evidence")?,
        },
        accepted_syntax_revision: text(v, "acceptedSyntaxRevision")?.into(),
        native_node_ref: text(v, "nativeNodeRef")?.into(),
        self_other: SelfOtherForm::parse(text(v, "selfOther")?)
            .ok_or_else(|| error("unknown self/other source form"))?,
        field: field(text(v, "field")?)?,
        interpreter: text(v, "interpreter")?.into(),
        expected_ground: optional(v, "expectedGround")?,
        reading: VakExpressionReadingV1 {
            contract: VAK_EXPRESSION_READING_CONTRACT,
            operator: op(text(r, "operator")?)?,
            horizon: VakAddressHorizon::ALL
                .into_iter()
                .find(|h| h.address() == r["horizon"].as_str().unwrap_or(""))
                .ok_or_else(|| error("unknown address horizon"))?,
            subjects,
            relation_refs: strings(r, "relationRefs")?
                .into_iter()
                .map(VakRef::new)
                .collect::<std::result::Result<_, _>>()
                .map_err(error)?,
            complement_refs: strings(r, "complementRefs")?
                .into_iter()
                .map(VakRef::new)
                .collect::<std::result::Result<_, _>>()
                .map_err(error)?,
            world_ref: optional(r, "worldRef")?,
            project_ref: optional(r, "projectRef")?,
            focus_ref: optional(r, "focusRef")?,
            expected_return: optional(r, "expectedReturn")?,
            standing,
            evidence: strings(r, "evidence")?,
        },
    }))
}
fn parse_return(v: &Value) -> R<ReturnInput> {
    Ok(ReturnInput {
        reference: text(v, "id")?.into(),
        determination: text(v, "determination")?.into(),
        target_use: text(v, "target")?.into(),
        kind: ground(text(v, "kind")?)?,
        relation_evidence: optional(v, "relationEvidence")?,
        basis: parse_basis(&v["basis"])?,
    })
}
fn parse_determine(v: &Value) -> R<DetermineInput> {
    let c = &v["contribution"];
    Ok(DetermineInput {
        reference: text(v, "id")?.into(),
        whole_use: text(v, "useRef")?.into(),
        viewing_lens: text(v, "lens")?.parse().map_err(error)?,
        language: parse_language(&v["language"])?,
        contribution: if c.is_null() {
            None
        } else {
            Some(AgentContribution {
                actor_ref: text(c, "actorRef")?.into(),
                result_ref: text(c, "resultRef")?.into(),
                input_refs: strings(c, "inputRefs")?,
                evidence: strings(c, "evidence")?,
            })
        },
        basis: parse_basis(&v["basis"])?,
    })
}
fn parse_whole(v: &Value) -> R<WholeInput> {
    let b = parse_basis(&v["basis"])?;
    let members = array(v, "members")?
        .iter()
        .map(|m| {
            let c = coord(m)?;
            StructuralParticipation::new(text(m, "subjectRef")?, c.position, c.face).map_err(error)
        })
        .collect::<R<Vec<_>>>()?;
    let source_returns = match v.get("sourceReturns") {
        None => Vec::new(),
        Some(_) => array(v, "sourceReturns")?
            .iter()
            .map(|r| {
                AnchorReturn::new(
                    text(r, "fromRef")?,
                    text(r, "anchorRef")?,
                    text(r, "groundRef")?,
                    face(text(r, "face")?)?,
                    ground(text(r, "kind")?)?,
                )
                .map_err(error)
            })
            .collect::<R<Vec<_>>>()?,
    };
    let form = StructuralConstellation::new(text(v, "wholeRef")?, members.clone(), source_returns)
        .map_err(error)?;
    let relations = match v.get("relations") {
        None => Vec::new(),
        Some(_) => array(v, "relations")?
            .iter()
            .map(|r| {
                ShapeRelationBinding::new(
                    QlShapeAddress {
                        row: coord(&r["row"])?,
                        column: coord(&r["column"])?,
                    },
                    text(r, "relationRef")?,
                    strings(r, "evidence")?,
                )
                .map_err(error)
            })
            .collect::<R<Vec<_>>>()?,
    };
    let binding = ShapeBinding::new(
        text(v, "subjectRef")?,
        QlShape::Constellation(form.grain()).shape_ref(),
        text(v, "wholeRef")?,
        members.iter().map(|m| m.subject_ref.clone()).collect(),
        members,
        relations,
        None,
        None,
        form.returns
            .iter()
            .map(AnchorReturn::operator_ref)
            .collect(),
        b.provenance.clone(),
    )
    .map_err(error)?;
    Ok(WholeInput {
        use_ref: text(v, "useRef")?.into(),
        form,
        binding,
        category: QlFamily::ALL
            .into_iter()
            .find(|f| f.code() == v["category"].as_str().unwrap_or(""))
            .ok_or_else(|| error("unknown kernel category"))?,
        ground_ref: text(v, "groundRef")?.into(),
        ground_face: face(text(v, "groundFace")?)?,
        frame: parse_frame(&v["frame"])?,
        basis: b,
        language: parse_language(&v["language"])?,
    })
}

/// Evaluate one bounded request in a private in-memory graph. A failed batch
/// returns no successful receipt and has no external side effects.
pub fn execute_request(request: &Value) -> R<Value> {
    if text(request, "contract")? != CONTRACT {
        return Err(error("wrong composition contract"));
    }
    let steps = array(request, "steps")?;
    if steps.len() > MAX_OBJECTS {
        return Err(error("too many composition steps"));
    }
    let registry = VakRegistry::from_authoritative_source().map_err(error)?;
    let mut graph = VakComposition::default();
    let mut contexts: BTreeMap<String, CPrimeContext> = BTreeMap::new();
    let mut oikonomias: BTreeMap<String, oik::CPrimeOikonomia> = BTreeMap::new();
    let mut results = Vec::new();
    for (index, s) in steps.iter().enumerate() {
        let operation = text(s, "op")?;
        let result = (|| -> R<Value> {
            match operation {
                "whole" => {
                    let input = parse_whole(s)?;
                    let id = input.use_ref.clone();
                    graph.bind_whole(&registry, input).map_err(error)?;
                    Ok(whole_view(graph.whole(&id).map_err(error)?))
                }
                "compose" => {
                    let id = text(s, "useRef")?;
                    graph
                        .compose(
                            &registry,
                            ComposeInput {
                                use_ref: id.into(),
                                whole_ref: text(s, "wholeRef")?.into(),
                                row: text(s, "row")?.into(),
                                column: text(s, "column")?.into(),
                                frame: parse_frame(&s["frame"])?,
                                ground_ref: text(s, "groundRef")?.into(),
                                ground_face: face(text(s, "groundFace")?)?,
                                basis: parse_basis(&s["basis"])?,
                                language: parse_language(&s["language"])?,
                            },
                        )
                        .map_err(error)?;
                    Ok(whole_view(graph.whole(id).map_err(error)?))
                }
                "reframe" => {
                    graph
                        .reframe(
                            text(s, "from")?,
                            text(s, "into")?,
                            parse_frame(&s["frame"])?,
                            parse_basis(&s["basis"])?,
                        )
                        .map_err(error)?;
                    Ok(whole_view(graph.whole(text(s, "into")?).map_err(error)?))
                }
                "recognise" | "reopen" => {
                    let (from, into, basis) =
                        (text(s, "from")?, text(s, "into")?, parse_basis(&s["basis"])?);
                    if operation == "recognise" {
                        let route = match optional(s, "route")? {
                            None => None,
                            Some(code) => Some(SixfoldRoute::parse(&code).ok_or_else(|| {
                                error(format!(
                                    "unknown reading of six {code}; use 4+2, 3+3, pairs-A, pairs-B or pairs-C"
                                ))
                            })?),
                        };
                        graph.recognise(from, into, route, basis).map_err(error)?;
                    } else {
                        graph.reopen(from, into, basis).map_err(error)?;
                    }
                    Ok(whole_view(graph.whole(into).map_err(error)?))
                }
                "read" => Ok(read_view(
                    &graph
                        .read(text(s, "useRef")?, text(s, "lens")?.parse().map_err(error)?)
                        .map_err(error)?,
                )),
                "lineage" => {
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
                "position" => Ok(whole_view(
                    graph
                        .position(text(s, "useRef")?, &axes(s, "path")?)
                        .map_err(error)?,
                )),
                "determine" => {
                    let input = parse_determine(s)?;
                    let id = input.reference.clone();
                    if let Some(c) = optional(s, "context")? {
                        contexts
                            .get(&c)
                            .ok_or_else(|| error("unknown context"))?
                            .determine(&mut graph, &registry, input)
                            .map_err(error)?;
                    } else {
                        graph.determine(&registry, input).map_err(error)?;
                    }
                    Ok(determination_view(graph.determination(&id).map_err(error)?))
                }
                "return" => {
                    let input = parse_return(s)?;
                    let id = input.reference.clone();
                    if let Some(c) = optional(s, "context")? {
                        contexts
                            .get_mut(&c)
                            .ok_or_else(|| error("unknown context"))?
                            .cs(&mut graph, input)
                            .map_err(error)?;
                    } else {
                        graph.return_result(input).map_err(error)?;
                    }
                    Ok(return_view(graph.returned(&id).map_err(error)?))
                }
                "offer" => {
                    graph
                        .offer_as_whole(
                            text(s, "return")?,
                            text(s, "into")?,
                            text(s, "wholeRef")?,
                            parse_basis(&s["basis"])?,
                        )
                        .map_err(error)?;
                    Ok(whole_view(graph.whole(text(s, "into")?).map_err(error)?))
                }
                "enter" => {
                    let id = text(s, "id")?;
                    if contexts.contains_key(id) {
                        return Err(error("context already exists"));
                    }
                    let c = CPrimeContext::enter(
                        &graph,
                        text(s, "useRef")?,
                        coord(&s["categoryGround"])?,
                    )
                    .map_err(error)?;
                    let result = context_view(&c);
                    contexts.insert(id.into(), c);
                    Ok(result)
                }
                "cpf" | "ct" | "cp" | "cf" | "cfp" => {
                    let c = contexts
                        .get_mut(text(s, "context")?)
                        .ok_or_else(|| error("unknown context"))?;
                    let b = parse_basis(&s["basis"])?;
                    match operation {
                        "cpf" => c
                            .cpf(
                                &mut graph,
                                text(s, "into")?,
                                face(text(s, "face")?)?,
                                strings(s, "operators")?
                                    .iter()
                                    .map(|o| op(o))
                                    .collect::<R<Vec<_>>>()?,
                                b,
                            )
                            .map_err(error)?,
                        "ct" => c
                            .ct(
                                strings(s, "fields")?
                                    .iter()
                                    .map(|f| field(f))
                                    .collect::<R<Vec<_>>>()?,
                                b,
                            )
                            .map_err(error)?,
                        "cp" => {
                            if s.get("coordinate").is_some() {
                                let positions = match text(s, "positions")? {
                                    "local" => PositionBasis::Local,
                                    "absolute" => PositionBasis::Absolute,
                                    _ => {
                                        return Err(error(
                                            "CP positions must be local or absolute",
                                        ));
                                    }
                                };
                                c.cp_at(
                                    &mut graph,
                                    text(s, "into")?,
                                    MemberFocus {
                                        coordinate: coord(&s["coordinate"])?,
                                        positions,
                                        basis: b,
                                    },
                                )
                                .map_err(error)?;
                            } else {
                                c.cp(&graph, &axes(s, "path")?, b).map_err(error)?;
                            }
                        }
                        "cf" => c
                            .cf(&mut graph, text(s, "into")?, parse_frame(&s["frame"])?, b)
                            .map_err(error)?,
                        "cfp" => {
                            let act = match text(s, "act")? {
                                "Freedom" => VakDivineAct::Freedom,
                                "Creation" => VakDivineAct::Creation,
                                "Sustenance" => VakDivineAct::Sustenance,
                                "Dissolution" => VakDivineAct::Dissolution,
                                "Veiling" => VakDivineAct::Veiling,
                                "Grace" => VakDivineAct::Grace,
                                "Absorption" => VakDivineAct::Absorption,
                                _ => return Err(error("unknown source R-path act")),
                            };
                            let bindings = array(s, "bindings")?
                                .iter()
                                .map(|x| {
                                    Ok(ThreadBinding {
                                        source_step: VakRef::new(text(x, "sourceStep")?)
                                            .map_err(error)?,
                                        path: axes(x, "path")?,
                                    })
                                })
                                .collect::<R<Vec<_>>>()?;
                            c.cfp(&graph, &registry, &bindings, act, b).map_err(error)?;
                        }
                        _ => unreachable!(),
                    }
                    Ok(context_view(c))
                }
                "inspect-context" => Ok(context_view(
                    contexts
                        .get(text(s, "context")?)
                        .ok_or_else(|| error("unknown context"))?,
                )),
                "cfp-form" | "z-begin" | "z-advance" | "z-reopen" | "cs-select" | "cs-hop" => {
                    let id = text(s, "oikonomia")?;
                    let c = contexts
                        .get_mut(text(s, "context")?)
                        .ok_or_else(|| error("unknown context"))?;
                    let state = oikonomias.entry(id.to_owned()).or_default();
                    match operation {
                        "cfp-form" => state
                            .select_cfp(c, cfp_form(text(s, "form")?)?, parse_basis(&s["basis"])?)
                            .map_err(error)?,
                        "z-begin" => state
                            .begin_z(
                                c,
                                oik::CfpZCycle::authorised(
                                    text(s, "undertakingRef")?,
                                    strings(s, "goalEvidence")?,
                                    text(s, "authorisationRef")?,
                                    parse_basis(&s["basis"])?,
                                )
                                .map_err(error)?,
                            )
                            .map_err(error)?,
                        "z-advance" => state
                            .advance_z(
                                c,
                                z_stage(text(s, "stage")?)?,
                                text(s, "resultRef")?,
                                strings(s, "evidence")?,
                                parse_basis(&s["basis"])?,
                            )
                            .map_err(error)?,
                        "z-reopen" => state
                            .reopen_z(
                                c,
                                strings(s, "reevaluationEvidence")?,
                                parse_basis(&s["basis"])?,
                            )
                            .map_err(error)?,
                        "cs-select" => state
                            .select_cs(
                                c,
                                cs_profile(text(s, "profile")?)?
                                    .passage(cs_direction(text(s, "direction")?)?),
                                parse_basis(&s["basis"])?,
                            )
                            .map_err(error)?,
                        "cs-hop" => {
                            let input = parse_return(s)?;
                            state
                                .perform_cs_hop(c, &mut graph, input)
                                .map_err(error)?;
                        }
                        _ => unreachable!(),
                    }
                    Ok(oikonomia_view(state))
                }
                "inspect-oikonomia" => Ok(oikonomia_view(
                    oikonomias
                        .get(text(s, "oikonomia")?)
                        .ok_or_else(|| error("unknown oikonomia"))?,
                )),
                "operative-return" => {
                    let state = oikonomias
                        .get(text(s, "oikonomia")?)
                        .ok_or_else(|| error("unknown oikonomia"))?;
                    Ok(operative_return_view(
                        &state
                            .returned(&graph, text(s, "returnRef")?)
                            .map_err(error)?,
                    ))
                }
                _ => Err(error(format!(
                    "unknown QL composition operation {operation}"
                ))),
            }
        })()
        .map_err(|e| error(format!("step {index} ({operation}): {e}")))?;
        results.push(json!({"op":operation,"result":result}));
    }
    Ok(json!({"contract":CONTRACT,"sourceRevision":VAK_SOURCE_REVISION,"results":results}))
}
pub fn command(args: &[String], _json: bool) -> R<String> {
    if args.len() != 1 {
        return Err(error("usage: ql vak compose <request.json> [--json]"));
    }
    let file = std::fs::File::open(&args[0]).map_err(error)?;
    let mut data = Vec::new();
    file.take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut data)
        .map_err(error)?;
    if data.len() > 16 * 1024 * 1024 {
        return Err(error("composition request exceeds 16 MiB"));
    }
    let request = serde_json::from_slice(&data).map_err(error)?;
    serde_json::to_string_pretty(&execute_request(&request)?).map_err(error)
}
/// One request read from its file, or from stdin when it is `-`, bounded to
/// 16 MiB as a composition is.
fn bounded_input(path: &str, what: &str) -> R<Vec<u8>> {
    let reader: Box<dyn Read> = if path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path).map_err(|e| error(format!("{path}: {e}")))?)
    };
    let mut data = Vec::new();
    reader
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut data)
        .map_err(error)?;
    if data.len() > 16 * 1024 * 1024 {
        return Err(error(format!("the {what} exceeds 16 MiB")));
    }
    Ok(data)
}

/// `ql vak thought-consumption <request.json|-> [--json]`: QL's reading of one
/// consumed T/T′ working thought. Factory's `factory.run-thought-consumption/v1`
/// receipt and the meaning chosen for each consumed thought come in as one
/// `ql.vak-thought-consumption-projection/v1` request; the
/// `ql.vak-thought-consumption/v1` reading comes back, with any Recognition
/// candidate as a proposal for AIKit. It stores nothing and grants nothing.
pub fn thought_consumption_command(args: &[String], _json: bool) -> R<String> {
    let [path] = args else {
        return Err(error(
            "usage: ql vak thought-consumption <request.json|-> [--json]",
        ));
    };
    let data = bounded_input(path, "thought-consumption request")?;
    let request: thought::ThoughtConsumptionProjectionRequest = serde_json::from_slice(&data)
        .map_err(|e| {
            error(format!(
                "the request is not a `{}`: {e}",
                thought::THOUGHT_CONSUMPTION_PROJECTION_REQUEST
            ))
        })?;
    let reading = thought::project_thought_consumption(request).map_err(error)?;
    serde_json::to_string_pretty(&reading).map_err(error)
}

/// A unit's heads broken down into QL's composition (`ql vak unit-steps`); the
/// request and the reading carry the same contract, as a composition does.
pub const UNIT_STEPS_CONTRACT: &str = "ql.vak-unit-steps/v1";
/// A unit's seven heads in Factory's spelling, each with the head of the team
/// space's Vāk-block instrument it is read as.
const UNIT_HEADS: [(&str, &str); 7] = [
    ("CPF", "vak-cpf"),
    ("CT", "vak-ct"),
    ("CP", "vak-cp"),
    ("CF", "vak-cf"),
    ("CFP", "vak-cfp"),
    ("CS", "vak-cs"),
    ("direction", "vak-direction"),
];
const UNIT_STEPS_STANDING: &str = "QL's breakdown of one unit's heads: each head's meaning as the Vāk-block instrument holds it, and the composition steps the heads fix. The steps run in a `ql.vak-composition/v1` composition once it has bound the unit's whole and entered its context, and names the context, oikonomia and basis each step takes: a unit names its whole, it does not carry it. It stores nothing, composes nothing and grants nothing.";

fn head_label<T: serde::de::DeserializeOwned>(
    heads: &serde_json::Map<String, Value>,
    head: &str,
) -> R<Option<(String, T)>> {
    match heads.get(head) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(label)) => serde_json::from_value(Value::String(label.clone()))
            .map(|value| Some((label.clone(), value)))
            .map_err(|e| error(format!("{head} `{label}`: {e}"))),
        Some(_) => Err(error(format!(
            "{head} is one label, or absent while it is open"
        ))),
    }
}
fn wire_label<T: serde::Serialize>(value: &T) -> R<String> {
    serde_json::to_value(value)
        .map_err(error)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| error("a C′ label is written as text"))
}
/// The kernel's two C′ owners hold the thread forms and the sequences in the
/// same order (ql-mef team_space tests pin it): CFPn is the nth CFP form and
/// CSn the nth CS profile.
fn same_place<A: PartialEq, B: Copy>(value: &A, from: &[A], into: &[B]) -> R<B> {
    from.iter()
        .position(|x| x == value)
        .and_then(|i| into.get(i).copied())
        .ok_or_else(|| error("QL's two C′ owners disagree on an order"))
}

/// Break one unit's heads (`{contract, unit, heads}`, Factory's `CPF … direction`
/// with an open head absent or null) into QL's composition. Each head is
/// refused in QL's words when its label is not QL's; read against the team
/// space's Vāk-block instrument for its meaning; and, where the heads fix one,
/// given as the step of `ql.vak-composition/v1` that carries it: CFP is
/// `cfp-form`, CS with its direction is `cs-select` with the hops it walks.
pub fn unit_steps(request: &Value) -> R<Value> {
    if text(request, "contract")? != UNIT_STEPS_CONTRACT {
        return Err(error(format!(
            "a unit's breakdown takes `{UNIT_STEPS_CONTRACT}`"
        )));
    }
    let unit = text(request, "unit")?;
    let heads = request
        .get("heads")
        .and_then(Value::as_object)
        .ok_or_else(|| error("missing/non-object heads"))?;
    if let Some(stray) = heads
        .keys()
        .find(|k| !UNIT_HEADS.iter().any(|(head, _)| head == k))
    {
        return Err(error(format!(
            "`{stray}` is no head of a unit; its heads are CPF, CT, CP, CF, CFP, CS and direction"
        )));
    }
    let participation: Option<(String, Participation)> = head_label(heads, "CPF")?;
    let content: Option<(String, ContentType)> = head_label(heads, "CT")?;
    let position: Option<(String, ContentPosition)> = head_label(heads, "CP")?;
    let frame: Option<(String, AuthoredFrame)> = head_label(heads, "CF")?;
    let thread: Option<(String, ThreadForm)> = head_label(heads, "CFP")?;
    let sequence: Option<(String, ContextSequence)> = head_label(heads, "CS")?;
    let direction: Option<(String, InquiryDirection)> = head_label(heads, "direction")?;
    let passage_direction = direction.as_ref().map(|(_, d)| match d {
        InquiryDirection::Forward => oik::CsDirection::ForwardSynthesis,
        InquiryDirection::Returning => oik::CsDirection::ReturningInquiry,
    });
    // Each head as given, and the Vāk-block label it is.
    let chosen: [Option<(String, String)>; 7] = [
        participation
            .as_ref()
            .map(|(l, v)| Ok::<_, CliError>((l.clone(), wire_label(v)?)))
            .transpose()?,
        content
            .as_ref()
            .map(|(l, v)| Ok::<_, CliError>((l.clone(), wire_label(v)?)))
            .transpose()?,
        position
            .as_ref()
            .map(|(l, v)| Ok::<_, CliError>((l.clone(), wire_label(v)?)))
            .transpose()?,
        frame
            .as_ref()
            .map(|(l, v)| (l.clone(), v.0.code().to_owned())),
        thread
            .as_ref()
            .map(|(l, v)| Ok::<_, CliError>((l.clone(), wire_label(v)?)))
            .transpose()?,
        sequence
            .as_ref()
            .map(|(l, v)| Ok::<_, CliError>((l.clone(), wire_label(v)?)))
            .transpose()?,
        direction
            .as_ref()
            .zip(passage_direction)
            .map(|((l, _), d)| (l.clone(), d.marker().to_owned())),
    ];
    let space = ql_mef::team_space::team_space();
    let block = space
        .instrument("vak-block")
        .ok_or_else(|| error("QL's team space holds no Vāk-block instrument"))?;
    let mut views = Vec::new();
    let mut open = Vec::new();
    for ((head, instrument_head), given) in UNIT_HEADS.iter().zip(chosen) {
        let held = block
            .heads
            .iter()
            .find(|h| h["id"] == *instrument_head)
            .ok_or_else(|| error(format!("the Vāk block holds no `{instrument_head}`")))?;
        let labels = held["labels"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default();
        views.push(match given {
            Some((label, id)) => {
                let meant = labels.iter().find(|l| l["id"] == id.as_str()).ok_or_else(|| {
                    error(format!("{head} `{label}` is no label of the Vāk block's `{instrument_head}`"))
                })?;
                json!({"head":head,"field":held["field"],"state":"chosen","label":label,"instrumentHead":instrument_head,
                    "instrumentLabel":id,"meaning":meant["description"],"owner":meant["owner_ref"]})
            }
            None => {
                open.push(*head);
                json!({"head":head,"field":held["field"],"state":"open","label":null,"instrumentHead":instrument_head,
                    "labels":labels.iter().map(|l| l["id"].clone()).collect::<Vec<_>>()})
            }
        });
    }
    let mut compose = Vec::new();
    if let Some((_, form)) = &thread {
        let form = same_place(form, &ThreadForm::ALL, &oik::CfpThreadForm::ALL)?;
        compose.push(json!({"head":"CFP","step":{"op":"cfp-form","form":word(&CFP_FORM_WORDS, form)},"marker":form.marker()}));
    }
    let mut walk = Value::Null;
    if let (Some((_, sequence)), Some((_, inquiry)), Some(cs_direction)) =
        (&sequence, &direction, passage_direction)
    {
        let profile = same_place(sequence, &ContextSequence::ALL, &oik::CsProfile::ALL)?;
        let passage = profile.passage(cs_direction);
        compose.push(json!({"head":"CS","step":{"op":"cs-select","profile":word(&CS_PROFILE_WORDS, profile),"direction":cs_direction.marker()},
            "marker":passage.marker(),
            "hops":passage.hops.iter().map(|h| json!({"index":h.index,"from":h.from.value(),"to":h.to.value(),"marker":h.marker(profile, cs_direction)})).collect::<Vec<_>>()}));
        walk = sequence
            .walk(*inquiry)
            .iter()
            .map(wire_label)
            .collect::<R<Vec<_>>>()?
            .into();
    }
    Ok(
        json!({"contract":UNIT_STEPS_CONTRACT,"sourceRevision":VAK_SOURCE_REVISION,"unit":unit,"heads":views,"open":open,
        "compose":compose,"walk":walk,"needs":["context","oikonomia","basis"],"standing":UNIT_STEPS_STANDING}),
    )
}
/// `ql vak unit-steps <unit.json|-> [--json]`.
pub fn unit_steps_command(args: &[String], _json: bool) -> R<String> {
    let [path] = args else {
        return Err(error("usage: ql vak unit-steps <unit.json|-> [--json]"));
    };
    let data = bounded_input(path, "unit")?;
    let request: Value =
        serde_json::from_slice(&data).map_err(|e| error(format!("the unit is not JSON: {e}")))?;
    serde_json::to_string_pretty(&unit_steps(&request)?).map_err(error)
}
fn basis_view(b: &Basis) -> Value {
    json!({"caller":b.provenance.caller_ref,"source":b.provenance.source_ref,"revision":b.revision,"standing":b.provenance.standing_ref,"evidence":b.evidence})
}
fn coord_view(c: QlCoordinate) -> Value {
    json!({"position":c.position.value(),"face":c.face.as_str()})
}
fn member_view(m: &StructuralParticipation) -> Value {
    json!({"subjectRef":m.subject_ref,"coordinate":coord_view(m.coordinate)})
}
fn route_view(r: &AnchorReturn) -> Value {
    json!({"fromRef":r.from_ref,"anchorRef":r.through_anchor_ref,"groundRef":r.target_ground_ref,
    "groundPosition":r.target_ground_position.value(),"face":r.target_face.as_str(),"kind":r.ground_kind.as_str(),"operatorRef":r.operator_ref()})
}
fn frame_view(f: ActiveFrame) -> Value {
    let c = f.id.canonical_selection().at_lens(f.lens).coordinate();
    json!({"id":f.id.code(),"expression":f.id.expression(),"lens":f.lens.code(),"basis":match f.basis{MusicalBasis::Chromatic=>"chromatic",MusicalBasis::Fifths=>"fifths"},
        "face":f.face.as_str(),"positions":match f.positions{PositionBasis::Local=>"local",PositionBasis::Absolute=>"absolute"},
        "localPosition":c.local_position().value(),"absolutePosition":c.absolute_position().value(),"unitFace":match c.unit_face(){MefUnitFace::Name=>"Name",MefUnitFace::Power=>"Power"},
        "grain":match c.grain(){MefGrain::InnerFour=>"inner-four",MefGrain::OuterTwo=>"outer-two"}})
}
fn binding_view(b: &ShapeBinding) -> Value {
    json!({"subjectRef":b.subject_ref,"shapeRef":b.shape_ref,"wholeRef":b.whole_ref,"basisRefs":b.basis_refs,
    "members":b.members.iter().map(member_view).collect::<Vec<_>>(),"relations":b.relation_bindings.iter().map(|r|json!({"row":coord_view(r.address.row),"column":coord_view(r.address.column),"relationRef":r.relation_ref,"evidence":r.evidence_refs,"kernel":pair_view(r.address.row,r.address.column)})).collect::<Vec<_>>(),
    "derivationRef":b.derivation_ref,"operatorRef":b.operator_ref,"returnRefs":b.return_refs,
    "provenance":{"caller":b.provenance.caller_ref,"source":b.provenance.source_ref,"standing":b.provenance.standing_ref}})
}
/// The kernel's own reading of a declared pair: every contract relation and
/// every A/B/C field its positions belong to.
fn pair_view(row: QlCoordinate, column: QlCoordinate) -> Value {
    let reading = classify_coordinate_pair(row, column);
    json!({"relations":reading.relations.iter().map(|r|r.as_str()).collect::<Vec<_>>(),
    "fields":reading.families.iter().map(|m|json!({"family":m.family.as_str(),"pairIndex":m.pair_index,"reversed":m.reversed})).collect::<Vec<_>>()})
}
fn derivation_view(d: &RelationFieldDerivation) -> Value {
    json!({"sourceWholeRefs":d.source_whole_refs,"sourceShapeRefs":d.source_shape_refs,
    "sourceGrains":d.source_grains.map(|g|g.as_str()),"operatorRef":d.operator_ref,"generatedShapeRef":d.generated_shape_ref,"returnBasis":d.return_basis,
    "sourceReturnRefs":d.source_return_refs,"sourceReturns":d.source_returns.iter().map(route_view).collect::<Vec<_>>()})
}
fn whole_view(w: &Whole) -> Value {
    json!({"useRef":w.use_ref,"language":w.language.as_ref().map(language_view),"binding":binding_view(&w.binding),"category":w.category.code(),"groundRef":w.ground_ref,"groundFace":w.ground_face.as_str(),
    "frame":frame_view(w.frame),"basis":w.basis.iter().map(basis_view).collect::<Vec<_>>(),"producingRefs":w.producing_refs,
    "body":match &w.body{WholeBody::Local(f)=>json!({"kind":"constellation","anchorRef":f.anchor_ref,"grain":f.grain().as_str(),"members":f.members.iter().map(member_view).collect::<Vec<_>>(),"returns":f.returns.iter().map(route_view).collect::<Vec<_>>()}),
        WholeBody::Relation{row,column,carrier}=>json!({"kind":"relation-field","rowUse":row,"columnUse":column,"carrierDerivation":carrier.as_ref().map(|c|derivation_view(&c.derivation()))})},
    "transitions":w.transitions.iter().map(|t|json!({"fromRef":t.from_ref,"operator":t.operation,"from":frame_view(t.from),"into":frame_view(t.into),"basis":basis_view(&t.basis)})).collect::<Vec<_>>()})
}
fn address_view(a: &SelectedAddress) -> Value {
    match a {
        SelectedAddress::Anchor(r) => json!({"kind":"anchor","wholeRef":r}),
        SelectedAddress::Member { whole_ref, member } => {
            json!({"kind":"member","wholeRef":whole_ref,"member":member_view(member)})
        }
        SelectedAddress::Relation {
            whole_ref,
            row,
            column,
        } => {
            json!({"kind":"relation","wholeRef":whole_ref,"row":address_view(row),"column":address_view(column)})
        }
    }
}
fn read_view(r: &FramedReading) -> Value {
    let pair = match &r.address {
        SelectedAddress::Relation { row, column, .. } => match (row.as_ref(), column.as_ref()) {
            (
                SelectedAddress::Member { member: a, .. },
                SelectedAddress::Member { member: b, .. },
            ) => pair_view(a.coordinate, b.coordinate),
            _ => Value::Null,
        },
        _ => Value::Null,
    };
    json!({"useRef":r.use_ref,"pairKernel":pair,"binding":binding_view(&r.binding),"frame":frame_view(r.frame),"address":address_view(&r.address),
    "language":r.language.as_ref().map(language_view),"memberFocus":r.member_focus.as_ref().map(|f|json!({"coordinate":coord_view(f.coordinate),"positions":match f.positions{PositionBasis::Local=>"local",PositionBasis::Absolute=>"absolute"},"basis":basis_view(&f.basis)})),"harmonicPitch":r.harmonic_pitch,"framePitch":r.frame_pitch,"focusInterval":r.focus_interval,"childIntervals":r.child_intervals,"geometry":{"shapeRef":r.geometry.shape_ref,"absolutePosition":r.geometry.absolute_position.value(),
        "framePhaseDegrees":r.geometry.frame_phase_degrees,"childPhaseDegrees":r.geometry.child_phase_degrees,"viewingLens":r.geometry.viewing_lens.to_string(),
        "viewingLocalPosition":r.geometry.viewing_rotation.local_position().value(),"viewingAbsolutePosition":r.geometry.viewing_rotation.absolute_position().value()},
    "children":r.children.iter().map(read_view).collect::<Vec<_>>(),"carrierDerivations":r.carrier_derivations.iter().map(derivation_view).collect::<Vec<_>>(),
    "transitions":r.transitions.iter().map(|t|json!({"fromRef":t.from_ref,"operator":t.operation,"from":frame_view(t.from),"into":frame_view(t.into),"basis":basis_view(&t.basis)})).collect::<Vec<_>>(),
    "sourceReturns":r.source_returns.iter().map(route_view).collect::<Vec<_>>(),"basis":r.basis.iter().map(basis_view).collect::<Vec<_>>()})
}
fn source_view(s: &VakSourceProvenance) -> Value {
    json!({"repository":s.repository,"revision":s.revision,"path":s.path,"gitBlob":s.git_blob,"coordinate":s.coordinate.to_string(),"sourceLine":s.source_line,"standing":s.standing.as_schema_str()})
}
fn receipt_view(r: &ReflectiveReceipt) -> Value {
    json!({"family":r.family.code(),"operatorRef":r.relation_id.as_str(),"inputRefs":r.input_refs,"outputRefs":r.output_refs,"basis":basis_view(&r.basis)})
}
fn paths_view(p: &[Axis]) -> Value {
    json!(
        p.iter()
            .map(|a| match a {
                Axis::Row => "row",
                Axis::Column => "column",
            })
            .collect::<Vec<_>>()
    )
}
fn rpath_view(p: &VakRPath) -> Value {
    json!({"actRef":p.act_ref.to_string(),"principleNineRef":p.principle_nine_ref.as_ref().map(ToString::to_string),"principleNineFormula":p.principle_nine_formula,
    "steps":p.steps.iter().map(|s|json!({"token":s.token,"vakRef":s.vak_ref.to_string()})).collect::<Vec<_>>(),"standing":p.standing.as_schema_str(),"evidence":p.evidence})
}
fn context_view(c: &CPrimeContext) -> Value {
    json!({"groundUse":c.ground_use,"focusUse":c.focus_use,"categoryGround":coord_view(c.category_ground),"focusPath":paths_view(&c.focus_path),
    "allowedOperators":c.allowed_operators.iter().map(|o|o.glyph()).collect::<Vec<_>>(),"contentFields":c.content_fields.iter().map(|f|f.symbol()).collect::<Vec<_>>(),
    "thread":c.thread,"threadBindings":c.thread_bindings.iter().map(|b|json!({"sourceStep":b.source_step.to_string(),"path":paths_view(&b.path)})).collect::<Vec<_>>(),
    "rPath":c.r_path.as_ref().map(rpath_view),"operations":c.operations.iter().map(receipt_view).collect::<Vec<_>>()})
}
fn z_cycle_view(z: &oik::CfpZCycle) -> Value {
    json!({"undertakingRef":z.undertaking_ref,"authorisationRef":z.authorisation_ref,"iteration":z.iteration,
    "expectedStage":z.expected_stage().map(|s|s.marker()),"complete":z.complete(),
    "stages":z.stages.iter().map(|r|json!({"iteration":r.iteration,"stage":r.stage.marker(),"resultRef":r.result_ref,"evidence":r.evidence,"basis":basis_view(&r.basis)})).collect::<Vec<_>>(),
    "reevaluationEvidence":z.reevaluation_evidence})
}
fn cs_walk_view(p: &oik::CsPassage, executed: &[oik::CsExecutedHop]) -> Value {
    json!({"marker":p.marker(),"profile":p.profile.index(),"direction":p.direction.marker(),
    "hops":p.hops.iter().map(|h|json!({"index":h.index,"from":h.from.value(),"to":h.to.value()})).collect::<Vec<_>>(),
    "executed":executed.iter().map(|h|json!({"index":h.index,"returnRef":h.return_ref,"determinationRef":h.determination_ref,"targetUse":h.target_use,"from":h.from.value(),"to":h.to.value()})).collect::<Vec<_>>()})
}
fn oikonomia_view(o: &oik::CPrimeOikonomia) -> Value {
    json!({"cfpForm":o.cfp_form.map(|f|f.marker()),"zCycle":o.z_cycle.as_ref().map(z_cycle_view),
    "csWalk":o.cs_passage.as_ref().map(|p|cs_walk_view(p,&o.cs_executed)),"csComplete":o.cs_complete()})
}
fn operative_return_view(r: &oik::OperativeReturn) -> Value {
    json!({"return":return_view(&r.base),"cfpForm":r.cfp_form.marker(),
    "zCycle":r.z_cycle.as_ref().map(z_cycle_view),
    "csWalk":cs_walk_view(&r.cs_passage,&r.cs_executed)})
}
fn determination_view(d: &Determination) -> Value {
    json!({"reference":d.reference,"wholeUse":d.whole_use,"reading":read_view(&d.reading),"sources":d.sources.iter().map(source_view).collect::<Vec<_>>(),
    "basis":basis_view(&d.basis),"standing":d.standing.as_schema_str(),"nativePaths":d.native_paths.iter().map(|p|json!({"reference":p.reference,"determination":p.determination,"pathRef":p.path.path_ref,"nativeNodeRef":p.native_node_ref,"nativeStepRef":p.native_step_ref,"actorRef":p.path.actor_ref,"evidence":p.path.evidence_refs,"basis":basis_view(&p.basis),"standing":p.standing.as_schema_str()})).collect::<Vec<_>>(),
    "expression":d.language.as_ref().map(|l|json!({"nativeNodeRef":l.native_node_ref,"resolvePathIdentity":l.general.resolve_path_identity,"syntaxVersion":l.general.syntax_version,
        "ownerRevision":l.general.owner_revision,"rendered":l.general.rendered,"fullVakRendering":l.general.full_vak_rendering,"selfOther":l.self_other.glyph(),"field":l.field.symbol(),
        "interpreter":l.interpreter,"expectedGround":l.expected_ground,"evidence":l.reading.evidence})),
    "contribution":d.contribution.as_ref().map(|c|json!({"actorRef":c.actor_ref,"resultRef":c.result_ref,"inputRefs":c.input_refs,"evidence":c.evidence})),
    "context":d.context.as_ref().map(|c|json!({"groundUse":c.ground_use,"groundBinding":binding_view(&c.ground_binding),"groundBasis":c.ground_basis.iter().map(basis_view).collect::<Vec<_>>(),"categoryGround":coord_view(c.category_ground),"focusPath":paths_view(&c.focus_path),
        "allowedOperators":c.allowed_operators.iter().map(|o|o.glyph()).collect::<Vec<_>>(),"contentFields":c.content_fields.iter().map(|f|f.symbol()).collect::<Vec<_>>(),
        "rPath":c.r_path.as_ref().map(rpath_view),"threadBindings":c.thread_bindings.iter().map(|b|json!({"sourceStep":b.source_step.to_string(),"path":paths_view(&b.path)})).collect::<Vec<_>>(),
        "operations":c.operations.iter().map(receipt_view).collect::<Vec<_>>(),"readings":c.readings.iter().map(read_view).collect::<Vec<_>>()}))})
}
fn return_view(r: &Returned) -> Value {
    json!({"reference":r.reference,"determination":r.determination,"sourceUse":r.source_use,"targetUse":r.target_use,"route":route_view(&r.route),
    "sourceBinding":binding_view(&r.source_binding),"targetBinding":binding_view(&r.target_binding),"producing":determination_view(&r.producing),
    "targetBasis":r.target_basis.iter().map(basis_view).collect::<Vec<_>>(),"relationEvidence":r.relation_evidence,"basis":basis_view(&r.basis),"standing":r.standing.as_schema_str()})
}

fn language_view(l: &FullVakBinding) -> Value {
    json!({"acceptedSyntaxRevision":l.accepted_syntax_revision,"nativeNodeRef":l.native_node_ref,
        "selfOther":l.self_other.glyph(),"field":l.field.symbol(),"interpreter":l.interpreter,"expectedGround":l.expected_ground,
        "general":{"syntaxVersion":l.general.syntax_version,"ownerRevision":l.general.owner_revision,"resolvePathIdentity":l.general.resolve_path_identity,
            "rendered":l.general.rendered,"fullVakRendering":l.general.full_vak_rendering,"evidence":l.general.evidence},
        "reading":{"contract":l.reading.contract,"operator":l.reading.operator.glyph(),"horizon":l.reading.horizon.address(),
            "subjects":l.reading.subjects.iter().map(|s|match s{VakExpressionSubject::Native(r)=>json!({"native":r}),VakExpressionSubject::Vak(r)=>json!({"vak":r.to_string()})}).collect::<Vec<_>>(),
            "relationRefs":l.reading.relation_refs.iter().map(ToString::to_string).collect::<Vec<_>>(),"complementRefs":l.reading.complement_refs.iter().map(ToString::to_string).collect::<Vec<_>>(),
            "worldRef":l.reading.world_ref,"projectRef":l.reading.project_ref,"focusRef":l.reading.focus_ref,"expectedReturn":l.reading.expected_return,
            "standing":l.reading.standing.as_schema_str(),"evidence":l.reading.evidence}})
}
