from pathlib import Path
import json

def edit(path,old,new):
 p=Path(path);s=p.read_text();assert s.count(old)==1,(path,old[:100],s.count(old));p.write_text(s.replace(old,new))
p='crates/ql-mef/src/vak_oi.rs'
edit(p,'impl VakGeneralExpressionEvidence {\n    pub fn validate(&self) -> Result<(), VakOiError> {','''impl VakGeneralExpressionEvidence {
    pub fn validate(&self) -> Result<(), VakOiError> {
        self.validate_for_revision(AIKIT_OPERATIVE_OWNER_REVISION)
    }
    /// An explicit consumer-selected syntax revision, not a silent replacement
    /// of the frozen #83 acceptance revision or a claim of native execution.
    pub fn validate_for_revision(&self, accepted_revision: &str) -> Result<(), VakOiError> {
        if accepted_revision.trim().is_empty() {
            return Err(VakOiError::Missing("accepted AIKit syntax revision"));
        }''')
edit(p,'if self.owner_revision != AIKIT_OPERATIVE_OWNER_REVISION {','if self.owner_revision != accepted_revision {')
edit(p,'''pub fn reconstruct_observed_vak_path(
    registry: &VakRegistry,
    observation: VakExecutionObservationV1,
) -> Result<VakPathV1, VakOiError> {''','''pub fn reconstruct_observed_vak_path(
    registry: &VakRegistry,
    observation: VakExecutionObservationV1,
) -> Result<VakPathV1, VakOiError> {
    reconstruct_observed_vak_path_for_revision(registry, observation, AIKIT_OPERATIVE_OWNER_REVISION)
}

/// Same observed-path validation under an explicitly accepted syntax revision.
pub fn reconstruct_observed_vak_path_for_revision(
    registry: &VakRegistry,
    observation: VakExecutionObservationV1,
    accepted_syntax_revision: &str,
) -> Result<VakPathV1, VakOiError> {''')
edit(p,'observation.general_expression.validate()?;','observation.general_expression.validate_for_revision(accepted_syntax_revision)?;')
edit('crates/ql-mef/src/lib.rs','reconstruct_observed_vak_path,','reconstruct_observed_vak_path, reconstruct_observed_vak_path_for_revision,')
p='crates/ql-mef/src/vak_composition.rs'
edit(p,'pub const CONTRACT:','mod native_path;\npub use native_path::{NativePathCorrelation, NativePathInput};\n\npub const CONTRACT:')
edit(p,'    pub frame_pitch: u8,','    pub language: Option<FullVakBinding>,\n    pub member_focus: Option<MemberFocus>,\n    pub frame_pitch: u8,')
edit(p,'            frame_pitch,','            language: whole.language.clone(),\n            member_focus: whole.member_focus.clone(),\n            frame_pitch,')
edit(p,'    pub contribution: Option<AgentContribution>,\n    pub context:','    pub contribution: Option<AgentContribution>,\n    pub native_paths: Vec<NativePathCorrelation>,\n    pub context:')
edit(p,'                context: None,','                context: None,\n                native_paths: Vec::new(),')
# CS writes its own receipt into the returned copy, never back-promoting d.
edit(p,'        graph.return_result(input)?;\n        self.receipt(VakFamily::Cs, inputs, outputs, basis);','''        let return_ref = input.reference.clone();
        graph.return_result(input)?;
        self.receipt(VakFamily::Cs, inputs, outputs, basis);
        if let Some(returned) = graph.returns.get_mut(&return_ref) {
            if let Some(context) = &mut returned.producing.context {
                context.operations.push(self.operations.last().expect("CS receipt").clone());
            }
        }''')
# The CLI now emits the complete source reading at every participating use.
cp='crates/ql-cli/src/vak_composition.rs'
edit(cp,'"harmonicPitch":r.harmonic_pitch,','"language":r.language.as_ref().map(language_view),"memberFocus":r.member_focus.as_ref().map(|f|json!({"coordinate":coord_view(f.coordinate),"positions":match f.positions{PositionBasis::Local=>"local",PositionBasis::Absolute=>"absolute"},"basis":basis_view(&f.basis)})),"harmonicPitch":r.harmonic_pitch,')
edit(cp,'"useRef":w.use_ref,"binding":binding_view(&w.binding),','"useRef":w.use_ref,"language":w.language.as_ref().map(language_view),"binding":binding_view(&w.binding),')
edit(cp,'"basis":basis_view(&d.basis),"standing":d.standing.as_schema_str(),','"basis":basis_view(&d.basis),"standing":d.standing.as_schema_str(),"nativePaths":d.native_paths.iter().map(|p|json!({"reference":p.reference,"determination":p.determination,"pathRef":p.path.path_ref,"nativeNodeRef":p.native_node_ref,"nativeStepRef":p.native_step_ref,"actorRef":p.path.actor_ref,"evidence":p.path.evidence_refs,"basis":basis_view(&p.basis),"standing":p.standing.as_schema_str()})).collect::<Vec<_>>(),')
with Path(cp).open('a') as f:f.write('''
fn language_view(l:&FullVakBinding)->Value {
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
''')
# This fixture was generated during initial integration but its then-narrow git
# staging omitted it. Supply it as a tracked public QL-level specimen.
def basis(s,standing='PROPOSED'):return dict(caller='specimen:ql',source=s,revision='synthetic-v1',standing=standing,evidence=['specimen:caller-input'])
def frame(cf):return dict(id=cf,lens='L0',basis='chromatic',face='direct',positions='local')
def whole(n,cf):return dict(op='whole',useRef=n,subjectRef='subject:'+n,wholeRef='anchor:'+n,category='M',groundRef='ground:'+n,groundFace='direct',frame=frame(cf),basis=basis('source:'+n),members=[dict(subjectRef=f'{n}:{p}:{f}',position=p,face=f) for f in ['direct','conjugate'] for p in range(6)],sourceReturns=[dict(fromRef='source-result:'+n,anchorRef='anchor:'+n,groundRef='ground:'+n,face='direct',kind='own')])
def compose(n,a,b,cf):return dict(op='compose',useRef=n,wholeRef='anchor:'+n,row=a,column=b,frame=frame(cf),groundRef='ground:'+n,groundFace='direct',basis=basis('specimen:composition'))
steps=[whole('a','CF2'),whole('b','CF3'),whole('c','CF4'),compose('ab','a','b','CF1'),compose('outer','ab','c','CF5'),dict(op='read',useRef='outer',lens='L5'),dict(op='reframe',**{'from':'outer'},into='outer-cf2',frame=frame('CF2'),basis=basis('specimen:frame-change')),dict(op='read',useRef='outer-cf2',lens="L1'"),dict(op='determine',id='d',useRef='outer-cf2',lens='L5',basis=basis('specimen:determination')),dict(op='return',id='r',determination='d',target='outer-cf2',kind='own',basis=basis('specimen:Return')),dict(op='offer',**{'return':'r'},into='offered',wholeRef='anchor:offered',basis=basis('r','DERIVED')),compose('next','offered','a','CF7'),dict(op='read',useRef='next',lens='L0')]
f=Path('fixtures/kernel/vak-composition-v1.json');assert not f.exists();f.write_text(json.dumps(dict(contract='ql.vak-composition/v1',steps=steps),indent=2)+'\n')
