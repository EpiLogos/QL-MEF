"""One-time #138 integration; all changes fail closed on mismatched source."""
from pathlib import Path
import re, json

def edit(path,old,new):
 p=Path(path);s=p.read_text();assert s.count(old)==1,(path,old[:100],s.count(old));p.write_text(s.replace(old,new))
def regex(path,pat,new,count=1):
 p=Path(path);s=p.read_text();s,n=re.subn(pat,new,s);assert n==count,(path,pat[:100],n);p.write_text(s)
p='crates/ql-mef/src/vak_composition.rs'
edit(p,'&[source.source.clone()]','std::slice::from_ref(&source.source)')
edit(p,'    pub transitions: Vec<Transition>,\n    depth: usize,','    pub transitions: Vec<Transition>,\n    pub member_focus: Option<MemberFocus>,\n    depth: usize,')
regex(p,r'(transitions: Vec::new\(\),\s*)(depth[,:])',r'\1member_focus: None,\n                \2',2)
edit(p,'    pub source_returns: Vec<AnchorReturn>,\n}','    pub source_returns: Vec<AnchorReturn>,\n    pub transitions: Vec<Transition>,\n    pub frame_pitch: u8,\n    pub focus_interval: u8,\n}')
edit(p,'        let mut whole = self.whole(from)?.clone();','        let mut whole = self.whole(from)?.clone();\n        require(whole.transitions.len() < MAX_DEPTH, "contextual transition bound exceeded")?;')
edit(p,'let coordinate = whole.frame.coordinate();','let coordinate = selected_coordinate(whole);')
edit(p,'        let pitch = whole.frame.pitch();','''        let frame_pitch = whole.frame.pitch();
        let pitch = match &whole.member_focus {
            Some(focus) => {
                let local = focus_local(focus, whole.frame.lens);
                crate::pitch_at_lens(whole.frame.basis, whole.frame.lens, local)
            }
            None => frame_pitch,
        };''')
edit(p,'directed_pitch_delta(pitch, c.harmonic_pitch)','directed_pitch_delta(frame_pitch, c.harmonic_pitch)')
edit(p,'''        let absolute = whole
            .frame
            .id
            .canonical_selection()
            .at_lens(whole.frame.lens)
            .coordinate()
            .absolute_position();''','''        let absolute = if let Some(focus) = &whole.member_focus {
            MefRotation::new(whole.frame.lens, focus_local(focus, whole.frame.lens).position).absolute_position()
        } else {
            whole.frame.id.canonical_selection().at_lens(whole.frame.lens).coordinate().absolute_position()
        };''')
edit(p,'frame_phase_degrees: u16::from(pitch) * 30,','frame_phase_degrees: u16::from(frame_pitch) * 30,')
edit(p,'            source_returns,\n        })','            source_returns,\n            transitions: whole.transitions.clone(),\n            frame_pitch,\n            focus_interval: directed_pitch_delta(frame_pitch, pitch),\n        })')
# Immutable focus transition stays separate from frame selection.
with Path(p).open('a') as f:f.write('''

/// An explicitly positioned use of an actual member. This is a coordinate in
/// the existing carrier, not a replacement shape or a newly inferred member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberFocus {
    pub coordinate: QlCoordinate,
    pub positions: PositionBasis,
    pub basis: Basis,
}
fn focus_local(f: &MemberFocus, lens: LensId) -> QlCoordinate {
    let position = match f.positions {
        PositionBasis::Local => f.coordinate.position,
        PositionBasis::Absolute => QlPosition::new((f.coordinate.position.value() + 6 - lens.index()) % 6)
            .expect("modulo-six coordinate"),
    };
    QlCoordinate::new(position, f.coordinate.face)
}
fn selected_coordinate(w: &Whole) -> QlCoordinate {
    match &w.member_focus {
        None => w.frame.coordinate(),
        Some(f) => {
            let local = focus_local(f, w.frame.lens);
            QlCoordinate::new(match w.frame.positions {
                PositionBasis::Local => local.position,
                PositionBasis::Absolute => MefRotation::new(w.frame.lens, local.position).absolute_position(),
            }, local.face)
        }
    }
}
impl VakComposition {
    pub fn position_member(&mut self, from: &str, into: &str, focus: MemberFocus) -> Result<()> {
        self.vacant(into)?;
        focus.basis.validate()?;
        let mut w = self.whole(from)?.clone();
        require(w.transitions.len() < MAX_DEPTH, "contextual transition bound exceeded")?;
        w.member_focus = Some(focus.clone());
        let coordinate = selected_coordinate(&w);
        match &w.body {
            WholeBody::Local(form) => require(form.members.iter().any(|m| m.coordinate == coordinate),
                "CP selects a member not disclosed by this whole")?,
            WholeBody::Relation { .. } => return Err(err("CP must address an actual local participant before choosing its member")),
        }
        w.use_ref = into.into();
        w.producing_refs.push(from.into());
        w.transitions.push(Transition { from_ref: from.into(), operation: VakFamily::Cp.relation_id().as_str().into(),
            from: w.frame, into: w.frame, basis: focus.basis.clone() });
        w.basis.push(focus.basis);
        self.wholes.insert(into.into(), w);
        Ok(())
    }
}
impl CPrimeContext {
    /// Resolve a local participant first with CP path, then determine its exact
    /// coordinate. CF remains active; it is never replaced by the CP choice.
    pub fn cp_at(&mut self, graph: &mut VakComposition, into: &str, focus: MemberFocus) -> Result<()> {
        let from = self.focus_use.clone();
        let basis = focus.basis.clone();
        graph.position_member(&from, into, focus)?;
        self.focus_use = into.into();
        self.receipt(VakFamily::Cp, vec![from], vec![into.into()], basis);
        Ok(())
    }
}
''')
# Preserve the explicit containing ground in the producing context.
edit(p,'pub struct ReflectiveDerivation {','pub struct ReflectiveDerivation {\n    pub ground_use: String,\n    pub ground_binding: ShapeBinding,\n    pub ground_basis: Vec<Basis>,')
edit(p,'        d.context = Some(ReflectiveDerivation {','        d.context = Some(ReflectiveDerivation {\n            ground_use: self.ground_use.clone(),\n            ground_binding,\n            ground_basis,')
edit(p,'        let reference = request.reference.clone();','        let ground = graph.whole(&self.ground_use)?;\n        let ground_binding = ground.binding.clone();\n        let ground_basis = ground.basis.clone();\n        let reference = request.reference.clone();')
# CLI library and application entry, preserving the existing parser owner.
cli='crates/ql-cli/src/lib.rs'
edit(cli,'pub const QL_CLI_CONTRACT:', 'pub mod vak_composition;\n\npub const QL_CLI_CONTRACT:')
edit(cli,'Some("context") => vak_context_command(&args[1..], json),','Some("context") => vak_context_command(&args[1..], json),\n        Some("compose") => vak_composition::command(&args[1..], json),')
edit(cli,'            "vak.context",','            "vak.context",\n            "vak.compose",')
edit(cli,'ql vak capabilities [--json]','ql vak compose <request.json> [--json]\\n  ql vak capabilities [--json]')
cp='crates/ql-cli/src/vak_composition.rs'
edit(cp,'"cp"=>c.cp(&graph,&axes(s,"path")?,b).map_err(error)?,','''"cp"=>{
                        if s.get("coordinate").is_some() {
                            let positions=match text(s,"positions")?{"local"=>PositionBasis::Local,"absolute"=>PositionBasis::Absolute,_=>return Err(error("CP positions must be local or absolute"))};
                            c.cp_at(&mut graph,text(s,"into")?,MemberFocus{coordinate:coord(&s["coordinate"])?,positions,basis:b}).map_err(error)?;
                        }else{c.cp(&graph,&axes(s,"path")?,b).map_err(error)?;}
                    },''')
edit(cp,'"harmonicPitch":r.harmonic_pitch,','"harmonicPitch":r.harmonic_pitch,"framePitch":r.frame_pitch,"focusInterval":r.focus_interval,')
edit(cp,'"sourceReturns":r.source_returns.iter().map(route_view).collect::<Vec<_>>(),','''"transitions":r.transitions.iter().map(|t|json!({"fromRef":t.from_ref,"operator":t.operation,"from":frame_view(t.from),"into":frame_view(t.into),"basis":basis_view(&t.basis)})).collect::<Vec<_>>(),
    "sourceReturns":r.source_returns.iter().map(route_view).collect::<Vec<_>>(),''')
edit(cp,'"context":d.context.as_ref().map(|c|json!({"categoryGround":','"context":d.context.as_ref().map(|c|json!({"groundUse":c.ground_use,"groundBinding":binding_view(&c.ground_binding),"groundBasis":c.ground_basis.iter().map(basis_view).collect::<Vec<_>>(),"categoryGround":')
# Shared language-neutral runnable specimen, not fabricated native execution.
def basis(s,standing='PROPOSED'):return dict(caller='specimen:ql',source=s,revision='synthetic-v1',standing=standing,evidence=['specimen:caller-input'])
def frame(cf):return dict(id=cf,lens='L0',basis='chromatic',face='direct',positions='local')
def whole(n,cf):return dict(op='whole',useRef=n,subjectRef='subject:'+n,wholeRef='anchor:'+n,category='M',groundRef='ground:'+n,groundFace='direct',frame=frame(cf),basis=basis('source:'+n),members=[dict(subjectRef=f'{n}:{p}:{f}',position=p,face=f) for f in ['direct','conjugate'] for p in range(6)],sourceReturns=[dict(fromRef='source-result:'+n,anchorRef='anchor:'+n,groundRef='ground:'+n,face='direct',kind='own')])
def compose(n,a,b,cf):return dict(op='compose',useRef=n,wholeRef='anchor:'+n,row=a,column=b,frame=frame(cf),groundRef='ground:'+n,groundFace='direct',basis=basis('specimen:composition'))
steps=[whole('a','CF2'),whole('b','CF3'),whole('c','CF4'),compose('ab','a','b','CF1'),compose('outer','ab','c','CF5'),dict(op='read',useRef='outer',lens='L5'),dict(op='reframe',**{'from':'outer'},into='outer-cf2',frame=frame('CF2'),basis=basis('specimen:frame-change')),dict(op='read',useRef='outer-cf2',lens="L1'"),dict(op='determine',id='d',useRef='outer-cf2',lens='L5',basis=basis('specimen:determination')),dict(op='return',id='r',determination='d',target='outer-cf2',kind='own',basis=basis('specimen:Return')),dict(op='offer',**{'return':'r'},into='offered',wholeRef='anchor:offered',basis=basis('r','DERIVED')),compose('next','offered','a','CF7'),dict(op='read',useRef='next',lens='L0')]
f=Path('fixtures/kernel/vak-composition-v1.json');assert not f.exists();f.write_text(json.dumps(dict(contract='ql.vak-composition/v1',steps=steps),indent=2)+'\n')
