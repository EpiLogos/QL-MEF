// Tests the actual native body preparation/state, not renderer strings or a
// fake field worker. Link with the existing current M registry/C clock library.
#include <ql/physical_body.hpp>
#include <cassert>
#include <iostream>
using namespace ql;
static void near(double actual,double expected,double absolute=1e-10,double relative=1e-9) {
    assert(std::isfinite(actual));
    assert(std::abs(actual-expected)<=absolute+relative*std::abs(expected));
}
static PhysicalBodyInput bar() {
    PhysicalBodyInput in;
    in.event_ref="controlled:physical-body/event"; in.subject_ref="controlled:physical-body/subject";
    in.source_coordinate="#3-2-1-1-1";
    in.source_revision="907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288";
    in.geometry_ref="controlled:metric-axial-bar"; in.geometry_revision="1";
    in.geometry_source_ref="controlled:analytic-bar-geometry"; in.geometry_standing="reference";
    in.preparation_ref="controlled:body/preparation1"; in.state_ref="controlled:body/state";
    in.source_generation=7; in.body_revision=1; in.sample_rate=48000; in.pratibimba=true;
    in.family=BodyFamily::AxialTruss;
    in.material={"controlled:elastic-material","1","controlled:analytic-material","reference",1e6,2,0.4,0};
    in.nodes={{1,"#3-2-1-1-1",{0,0,0},0,{true,true,true}},
              {2,"#3-0",{1,0,0},0,{false,true,true}}};
    in.edges={{0,1,1e-4,0}};
    in.exciter={{1,0,0},{0,1}}; in.pickup=in.exciter;
    in.pickup_linear_per_metre=1000; in.max_force_newtons=10;
    in.max_impulse_newton_seconds=0.01; in.max_displacement_metres=0.1;
    return in;
}
static void advance(PhysicalBody& body,std::size_t count,double force=0) {
    std::array<double,512> forces; forces.fill(force); std::array<float,512> pcm{};
    while (count) {
        const auto frames=std::min<std::size_t>(count,512);
        assert(body.advance_force_block(forces.data(),pcm.data(),frames,body.body_revision(),body.samples_elapsed()));
        count-=frames;
    }
}
static double displacement(const PhysicalBody& body,std::size_t node=1,unsigned axis=0) {
    std::array<Vec3,2> xyz{};
    assert(body.write_displacements(xyz.data(),xyz.size(),body.body_revision(),body.samples_elapsed()));
    return xyz[node][axis];
}
template<class F> static void refused(F test) {
    bool rejected=false; try { test(); } catch(const std::invalid_argument&) { rejected=true; }
    assert(rejected);
}
static void analytic_eigenstructure() {
    auto in=bar(); PreparedPhysicalBody prepared(in);
    const double mass=2*1e-4*1/2, stiffness=1e6*1e-4/1;
    near(prepared.node_mass_kg(1),mass);
    near(prepared.frequency_hz(0),std::sqrt(stiffness/mass)/(2*pi));
    auto change=in; change.material.young_modulus_pa*=4;
    near(PreparedPhysicalBody(change).frequency_hz(0),2*prepared.frequency_hz(0));
    change=in; change.material.density_kg_per_m3*=4;
    near(PreparedPhysicalBody(change).frequency_hz(0),prepared.frequency_hz(0)/2);
    change=in; change.nodes[1].rest_metres[0]*=2;
    near(PreparedPhysicalBody(change).frequency_hz(0),prepared.frequency_hz(0)/2);
    change=in; change.edges[0].section_m2*=4;
    // For a uniform axial bar, area scales K and M equally.
    near(PreparedPhysicalBody(change).frequency_hz(0),prepared.frequency_hz(0));
}
static void impulse_ringdown_shared_observation() {
    auto in=bar(); PhysicalBody body{PreparedPhysicalBody(in)};
    const double mass=1e-4, omega=1000, gamma=0.2, impulse=1e-6;
    assert(body.apply_impulse_newton_seconds(impulse,1,0));
    const double initial=impulse*impulse/(2*mass);
    near(body.mechanical_energy_joules(),initial,1e-18);
    std::array<double,512> force{}; std::array<float,512> pcm{}; std::array<Vec3,2> visible{};
    for (unsigned block=0;block<20;++block) {
        const double previous_energy=body.mechanical_energy_joules();
        assert(body.advance_force_block(force.data(),pcm.data(),pcm.size(),1,body.samples_elapsed()));
        const double t=double(body.samples_elapsed())/48000, wd=std::sqrt(omega*omega-gamma*gamma);
        const double expected=(impulse/mass)*std::exp(-gamma*t)*std::sin(wd*t)/wd;
        near(displacement(body),expected,1e-13);
        near(pcm.back(),expected*1000,1e-8,1e-6);
        assert(body.mechanical_energy_joules()<=previous_energy*(1+1e-10));
        assert(body.write_visible_positions(visible.data(),visible.size(),1,body.samples_elapsed()));
        near(visible[1][0],1+expected);
        const auto receipt=body.observation();
        assert(*receipt.state_ref==in.state_ref && *receipt.preparation_ref==in.preparation_ref);
        assert(receipt.samples_elapsed==body.samples_elapsed() && receipt.body_revision==1);
        near(receipt.pickup_linear,pcm.back(),1e-12);
        // Severing the visible/audio time identity must refuse observation.
        assert(!body.write_displacements(visible.data(),visible.size(),1,receipt.samples_elapsed-1));
        assert(!body.write_displacements(visible.data(),visible.size(),2,receipt.samples_elapsed));
    }
}
static void constant_force_and_zero_input() {
    auto in=bar(); in.material.damping_alpha_per_second=0;
    PhysicalBody body{PreparedPhysicalBody(in)};
    advance(body,4096,0);
    assert(displacement(body)==0 && body.mechanical_energy_joules()==0);
    advance(body,321,0.01);
    const double t=321.0/48000;
    near(displacement(body),0.01/100*(1-std::cos(1000*t)),1e-13);
    // Exact undamped continuation retains energy for a long bounded run.
    const double energy=body.mechanical_energy_joules();
    advance(body,48000,0); near(body.mechanical_energy_joules(),energy,1e-16,1e-8);
}
static void exciter_and_pickup_location_are_physical() {
    auto in=bar(); PhysicalBody audible{PreparedPhysicalBody(in)};
    auto at_boundary=in; at_boundary.exciter.node_weights={1,0};
    PhysicalBody silent{PreparedPhysicalBody(at_boundary)};
    advance(audible,100,0.01); advance(silent,100,0.01);
    assert(std::abs(displacement(audible))>1e-8 && displacement(silent)==0);
    auto wrong_axis=in; wrong_axis.pickup.axis={0,1,0};
    PhysicalBody orthogonal{PreparedPhysicalBody(wrong_axis)}; advance(orthogonal,100,0.01);
    near(displacement(orthogonal),displacement(audible));
    assert(orthogonal.observation().pickup_linear==0);
    // Both bodies' physical resonance remains independent of pickup/camera.
    near(audible.preparation().frequency_hz(0),orthogonal.preparation().frequency_hz(0));
}
static void force_blocks_refuse_atomically() {
    PhysicalBody body{PreparedPhysicalBody(bar())}; advance(body,20,0.01);
    const double q=displacement(body),energy=body.mechanical_energy_joules(); const auto cursor=body.samples_elapsed();
    std::array<double,4> force{0.01,0.01,0.01,0.01}; std::array<float,4> output; output.fill(123);
    const auto check=[&] { assert(body.samples_elapsed()==cursor); assert(displacement(body)==q);
        assert(body.mechanical_energy_joules()==energy); for(float x:output) assert(x==123); };
    assert(!body.advance_force_block(force.data(),output.data(),4,2,cursor)); check();
    assert(!body.advance_force_block(force.data(),output.data(),4,1,cursor-1)); check();
    force[3]=std::numeric_limits<double>::quiet_NaN();
    assert(!body.advance_force_block(force.data(),output.data(),4,1,cursor)); check();
    force[3]=100;
    assert(!body.advance_force_block(force.data(),output.data(),4,1,cursor)); check();
    assert(!body.advance_force_block(force.data(),output.data(),513,1,cursor)); check();
    assert(!body.apply_impulse_newton_seconds(0.02,1,cursor)); check();
    auto tiny=bar(); tiny.max_displacement_metres=1e-10;
    PhysicalBody bounded{PreparedPhysicalBody(tiny)}; force.fill(1);
    assert(!bounded.advance_force_block(force.data(),output.data(),4,1,0));
    assert(bounded.samples_elapsed()==0 && bounded.mechanical_energy_joules()==0);
    for(float x:output) assert(x==123);
}
static void material_update_preserves_resident_geometry_state_and_time() {
    auto in=bar(); PhysicalBody body{PreparedPhysicalBody(in)}; advance(body,321,0.01);
    const double q=displacement(body); const auto cursor=body.samples_elapsed();
    auto next=in; next.material.young_modulus_pa*=4; next.material.revision="2";
    next.body_revision=2; next.preparation_ref="controlled:body/preparation2";
    assert(!body.replace_material(PreparedPhysicalBody(next),0));
    assert(body.body_revision()==1 && displacement(body)==q && body.samples_elapsed()==cursor);
    assert(body.replace_material(PreparedPhysicalBody(next),1));
    assert(body.body_revision()==2 && body.samples_elapsed()==cursor && body.state_ref()==in.state_ref);
    near(displacement(body),q);
    near(body.preparation().frequency_hz(0),2000/(2*pi));
    auto changed_shape=next; changed_shape.body_revision=3; changed_shape.nodes[1].rest_metres[0]=2;
    assert(!body.replace_material(PreparedPhysicalBody(changed_shape),2));
    auto changed_event=next; changed_event.body_revision=3; changed_event.event_ref="controlled:other";
    assert(!body.replace_material(PreparedPhysicalBody(changed_event),2));
    assert(body.samples_elapsed()==cursor && body.body_revision()==2);
}
static void distinct_tension_and_rigid_body_laws() {
    auto tension=bar(); tension.family=BodyFamily::PrestressedTensionNetwork;
    tension.nodes[1].fixed={true,false,true}; tension.exciter.axis={0,1,0}; tension.pickup=tension.exciter;
    tension.edges[0].prestress_newtons=100;
    PreparedPhysicalBody prepared(tension); near(prepared.frequency_hz(0),1000/(2*pi));
    auto changed=tension; changed.edges[0].prestress_newtons*=4;
    near(PreparedPhysicalBody(changed).frequency_hz(0),2*prepared.frequency_hz(0));
    auto axial=bar(); axial.edges[0].prestress_newtons=100;
    refused([&]{PreparedPhysicalBody invalid(axial);});
    auto free=bar(); free.nodes[0].fixed={false,true,true}; free.material.damping_alpha_per_second=0;
    PhysicalBody moving{PreparedPhysicalBody(free)};
    assert(moving.preparation().mode_count()==2); near(moving.preparation().frequency_hz(0),0);
    assert(moving.apply_impulse_newton_seconds(1e-6,1,0)); advance(moving,48,0);
    // Centre-of-mass translation from J/(2m), alongside relative vibration.
    near((displacement(moving,0)+displacement(moving,1))/2,1e-6/(2e-4)*0.001,1e-12);
}
static void numerical_and_resource_rejections() {
    auto in=bar(); in.source_coordinate="#3-2-999"; refused([&]{PreparedPhysicalBody invalid(in);});
    in=bar(); in.nodes[1].rest_metres={0,0,0}; refused([&]{PreparedPhysicalBody invalid(in);});
    in=bar(); in.material.density_kg_per_m3=0; refused([&]{PreparedPhysicalBody invalid(in);});
    in=bar(); in.material.young_modulus_pa=1e13; refused([&]{PreparedPhysicalBody invalid(in);});
    in=bar(); in.edges.push_back(in.edges[0]); refused([&]{PreparedPhysicalBody invalid(in);});
    in=bar(); in.exciter.axis={2,0,0}; refused([&]{PreparedPhysicalBody invalid(in);});
    in=bar(); in.nodes.resize(33,in.nodes[0]); refused([&]{PreparedPhysicalBody invalid(in);});
    for(double gamma:{0.0,1000.0,1e6}) {
        const auto step=physical_step(1e6,gamma,1.0/48000);
        for(double value:{step.a11,step.a12,step.a21,step.a22,step.bq,step.bv}) assert(std::isfinite(value));
        // Equilibrium F/K remains fixed under under/critical/over damping.
        near(step.a11+step.bq*1e6,1,1e-12);
        near(step.a21+step.bv*1e6,0,1e-10);
    }
}
static void explicit_form_transition_retains_identity_cursor_and_reports_work() {
    auto in=bar(); PhysicalBody body{PreparedPhysicalBody(in)}; advance(body,321,0.01);
    const auto cursor=body.samples_elapsed(); const double q=displacement(body),energy=body.mechanical_energy_joules();
    auto next=in; next.nodes[1].rest_metres[0]=2; next.geometry_revision="2"; next.geometry_ref="controlled:longer-bar";
    next.body_revision=2; next.source_generation=8; next.preparation_ref="controlled:new-form";
    PhysicalFormTransitionReceipt receipt{};
    assert(!body.replace_form(PreparedPhysicalBody(next),1,cursor-1,PhysicalFormTransition::ProjectCorrespondingNodes,receipt));
    assert(body.body_revision()==1 && body.samples_elapsed()==cursor && displacement(body)==q);
    assert(body.replace_form(PreparedPhysicalBody(next),1,cursor,PhysicalFormTransition::ProjectCorrespondingNodes,receipt));
    assert(body.body_revision()==2 && body.samples_elapsed()==cursor && body.state_ref()==in.state_ref);
    near(displacement(body),q); near(body.preparation().frequency_hz(0),500/(2*pi));
    near(receipt.before_energy_joules,energy,1e-15);
    near(receipt.external_work_joules,receipt.after_energy_joules-energy,1e-15);
    next.body_revision=3; next.preparation_ref="controlled:explicit-reset";
    assert(body.replace_form(PreparedPhysicalBody(next),2,cursor,PhysicalFormTransition::ExplicitReset,receipt));
    assert(displacement(body)==0 && body.mechanical_energy_joules()==0 && body.samples_elapsed()==cursor);
    assert(receipt.after_energy_joules==0 && receipt.external_work_joules<=0);
}
static void bounded_checkpoint_restore_replays_exact_state_and_refuses_atomically() {
    auto in=bar();PhysicalBody body{PreparedPhysicalBody(in)};advance(body,321,0.01);
    const auto saved=body.checkpoint();assert(saved.version==1 && saved.contract==physical_checkpoint_contract);
    assert(saved.displacement_unit=="m" && saved.velocity_unit=="m/s" && saved.generalized_modal_mass_kg==1);
    assert(saved.eigenbasis_identity==body.preparation().eigenbasis_identity());
    std::array<double,128> forces;for(std::size_t i=0;i<forces.size();++i)forces[i]=0.01*std::sin(2*pi*73*i/48000);
    std::array<float,128> expected,actual;
    assert(body.advance_force_block(forces.data(),expected.data(),128,1,321));const auto final=body.checkpoint();
    assert(body.restore_checkpoint(saved,1,449));assert(body.samples_elapsed()==321);
    assert(body.checkpoint().displacement_modal_metres==saved.displacement_modal_metres);
    assert(body.checkpoint().velocity_modal_metres_per_second==saved.velocity_modal_metres_per_second);
    assert(body.advance_force_block(forces.data(),actual.data(),128,1,321));assert(actual==expected);
    const auto replay=body.checkpoint();assert(replay.displacement_modal_metres==final.displacement_modal_metres);
    assert(replay.velocity_modal_metres_per_second==final.velocity_modal_metres_per_second);
    assert(replay.last_pickup_linear==final.last_pickup_linear && replay.samples_elapsed==final.samples_elapsed);
    // Reopen into the same independently prepared native body, not only a
    // seed replay of a surviving object. Complete state resumes bit exactly.
    PhysicalBody reopened{PreparedPhysicalBody(in)};assert(reopened.restore_checkpoint(saved,1,0));
    assert(reopened.advance_force_block(forces.data(),actual.data(),128,1,321));assert(actual==expected);
    const auto resident=body.checkpoint();
    const auto unchanged=[&](){const auto now=body.checkpoint();assert(now.samples_elapsed==resident.samples_elapsed
        && now.displacement_modal_metres==resident.displacement_modal_metres && now.velocity_modal_metres_per_second==resident.velocity_modal_metres_per_second
        && now.last_pickup_linear==resident.last_pickup_linear && now.eigenbasis_identity==resident.eigenbasis_identity);};
    const auto reject=[&](PhysicalBodyCheckpoint wrong){assert(!body.restore_checkpoint(wrong,1,449));unchanged();};
    auto wrong=saved;wrong.version=2;reject(wrong);
    wrong=saved;wrong.contract="foreign:checkpoint";reject(wrong);
    wrong=saved;wrong.state_ref="independent:state";reject(wrong);
    wrong=saved;wrong.event_ref="foreign:event";reject(wrong);
    wrong=saved;wrong.preparation_ref="foreign:preparation";reject(wrong);
    wrong=saved;wrong.material_revision="stale";reject(wrong);
    wrong=saved;wrong.geometry_revision="stale";reject(wrong);
    wrong=saved;wrong.source_revision="stale";reject(wrong);
    wrong=saved;wrong.pratibimba=!wrong.pratibimba;reject(wrong);
    wrong=saved;wrong.source_generation+=1;reject(wrong);
    wrong=saved;wrong.eigenbasis_identity="same-label-different-physical-basis";reject(wrong);
    wrong=saved;wrong.displacement_unit="normalized";reject(wrong);
    wrong=saved;wrong.generalized_modal_mass_kg=2;reject(wrong);
    wrong=saved;wrong.displacement_modal_metres.push_back(0);reject(wrong);
    wrong=saved;wrong.velocity_modal_metres_per_second[0]=std::numeric_limits<double>::quiet_NaN();reject(wrong);
    wrong=saved;wrong.displacement_modal_metres[0]=1e9;reject(wrong);
    wrong=saved;wrong.last_pickup_linear+=1;reject(wrong);
    assert(!body.restore_checkpoint(saved,2,449));unchanged();assert(!body.restore_checkpoint(saved,1,448));unchanged();
    auto silently_changed=in;silently_changed.material.young_modulus_pa*=4;
    PhysicalBody incompatible{PreparedPhysicalBody(silently_changed)};
    assert(incompatible.preparation().eigenbasis_identity()!=saved.eigenbasis_identity);
    assert(!incompatible.restore_checkpoint(saved,1,0));assert(incompatible.samples_elapsed()==0 && incompatible.mechanical_energy_joules()==0);
}
int main() {
    analytic_eigenstructure(); impulse_ringdown_shared_observation(); constant_force_and_zero_input();
    exciter_and_pickup_location_are_physical(); force_blocks_refuse_atomically();
    material_update_preserves_resident_geometry_state_and_time(); distinct_tension_and_rigid_body_laws();
    numerical_and_resource_rejections();
    explicit_form_transition_retains_identity_cursor_and_reports_work();
    bounded_checkpoint_restore_replays_exact_state_and_refuses_atomically();
    std::cout<<"native physical body analytic and causal tests passed\n";
}
