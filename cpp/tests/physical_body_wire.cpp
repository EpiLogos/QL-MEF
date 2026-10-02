// Receives only actual Rust preparation + actual M3 snapshot over stdin.
// No hand-built symbolic frames or replacement numerical dynamics.
#include <ql/physical_checkpoint_wire.hpp>
#include <cassert>
#include <iostream>
#include <string>
using namespace ql::physical_wire;
void put(J* object,const char* key,J* value) { assert(value);assert(json_object_object_add(object,key,value)==0); }
void string(J* object,const char* key,const std::string& value) { put(object,key,json_object_new_string_len(value.data(),int(value.size()))); }
Json copy(J* source) { J* value=nullptr;assert(json_object_deep_copy(source,&value,nullptr)==0);return own(value); }
template<class Mutation> void reject(J* packet,J* current,Mutation mutation) {
    auto changed=copy(packet);mutation(changed.get());bool refused=false;
    try { ql::PreparedPhysicalBody next(read_prepared_physical_body(changed.get(),current,true)); }
    catch(const std::invalid_argument&) { refused=true; }assert(refused);
}
int main() {
    std::string line;std::getline(std::cin,line);assert(line.size()>0 && line.size()<=1024*1024);
    auto tok=std::unique_ptr<json_tokener,decltype(&json_tokener_free)>(json_tokener_new_ex(64),json_tokener_free);
    json_tokener_set_flags(tok.get(),JSON_TOKENER_STRICT|JSON_TOKENER_VALIDATE_UTF8);
    auto input=own(json_tokener_parse_ex(tok.get(),line.data(),int(line.size())));
    assert(json_tokener_get_error(tok.get())==json_tokener_success && json_tokener_get_parse_end(tok.get())==line.size());
    keys(input.get(),{"preparation","current_m3"});auto packet=field(input.get(),"preparation"),current=field(input.get(),"current_m3");
    ql::PreparedPhysicalBody prepared(read_prepared_physical_body(packet,current,true));
    const auto original=prepared.input();
    reject(packet,current,[](J* x){put(field(x,"source_coordinate"),"face",json_object_new_string("bimba"));});
    reject(packet,current,[](J* x){put(field(x,"source_coordinate"),"canonical_ref",json_object_new_string("ql:m-coordinate:bimba:M3"));});
    reject(packet,current,[](J* x){put(field(x,"source_coordinate"),"source_ref",json_object_new_string("#3-0"));});
    reject(packet,current,[](J* x){put(field(x,"source_coordinate"),"provenance",json_object_new_array());});
    reject(packet,current,[](J* x){put(x,"source_generation",json_object_new_int(9000));});
    reject(packet,current,[](J* x){put(x,"event_ref",json_object_new_string("event:unrelated"));});
    reject(packet,current,[](J* x){put(field(x,"units"),"excitation",json_object_new_string("m/s"));});
    reject(packet,current,[](J* x){put(field(x,"request"),"max_force_newtons",json_object_new_double(0));});
    reject(packet,current,[](J* x){put(field(x,"clock"),"degree720",json_object_new_int(999));});
    reject(packet,current,[](J* x){put(x,"unknown",json_object_new_boolean(true));});
    reject(packet,current,[](J* x){put(field(at(field(field(x,"source_coordinate"),"payloads"),0),"record"),"git_blob",json_object_new_string("forged-source-blob"));});
    auto wrong=copy(current);put(field(wrong.get(),"identity"),"profile_generation",json_object_new_int(9000));
    bool disconnected=false;try { read_prepared_physical_body(packet,wrong.get(),true); } catch(const std::invalid_argument&) { disconnected=true; }assert(disconnected);
    std::vector<double> frequencies;for(std::size_t i=0;i<prepared.mode_count();++i)frequencies.push_back(prepared.frequency_hz(i));
    ql::PhysicalBody body(std::move(prepared));std::array<double,128> force{};std::array<float,128> pcm;
    pcm.fill(33);assert(body.advance_force_block(force.data(),pcm.data(),128,original.body_revision,0));
    for(auto sample:pcm)assert(sample==0);assert(body.mechanical_energy_joules()==0);
    force.fill(1e-4);assert(body.advance_force_block(force.data(),pcm.data(),128,original.body_revision,128));
    assert(body.mechanical_energy_joules()>0);
    const auto observation=body.observation();assert(observation.samples_elapsed==256);
    std::vector<ql::Vec3> displacement(original.nodes.size()),visible(original.nodes.size());
    assert(body.write_displacements(displacement.data(),displacement.size(),original.body_revision,256));
    assert(body.write_visible_positions(visible.data(),visible.size(),original.body_revision,256));
    double pickup=0;bool moved=false;
    for(std::size_t i=0;i<displacement.size();++i)for(unsigned a=0;a<3;++a) {
        assert(std::isfinite(displacement[i][a]));moved=moved || displacement[i][a]!=0;
        assert(std::abs(visible[i][a]-original.nodes[i].rest_metres[a]-displacement[i][a])<1e-14);
        pickup+=displacement[i][a]*original.pickup.axis[a]*original.pickup.node_weights[i]*original.pickup_linear_per_metre;
    }
    assert(moved && std::abs(pickup-observation.pickup_linear)<=1e-6*std::max(1.0,std::abs(pickup)));
    auto before=body.mechanical_energy_joules();pcm.fill(77);
    assert(!body.advance_force_block(force.data(),pcm.data(),128,original.body_revision+1,256));
    assert(body.samples_elapsed()==256 && body.mechanical_energy_joules()==before);for(auto sample:pcm)assert(sample==77);
    const auto saved=body.checkpoint();auto persisted=checkpoint_wire(saved);
    const auto persisted_text=std::string(json_object_to_json_string_ext(persisted.get(),JSON_C_TO_STRING_PLAIN));
    auto reparsed=own(json_tokener_parse(persisted_text.c_str()));const auto recovered=read_checkpoint_wire(reparsed.get());
    assert(recovered.displacement_modal_metres==saved.displacement_modal_metres && recovered.velocity_modal_metres_per_second==saved.velocity_modal_metres_per_second);
    std::array<float,128> continuation,replay;
    assert(body.advance_force_block(force.data(),continuation.data(),128,original.body_revision,256));
    assert(body.restore_checkpoint(recovered,original.body_revision,384));
    assert(body.advance_force_block(force.data(),replay.data(),128,original.body_revision,256));assert(replay==continuation);
    assert(body.restore_checkpoint(recovered,original.body_revision,384));assert(body.samples_elapsed()==256);
    auto malformed=copy(persisted.get());put(field(malformed.get(),"state"),"samples_elapsed",json_object_new_string("0256"));
    bool refused_decimal=false;try {read_checkpoint_wire(malformed.get());}catch(const std::invalid_argument&){refused_decimal=true;}assert(refused_decimal);
    malformed=copy(persisted.get());put(field(malformed.get(),"units"),"velocity",json_object_new_string("normalized/tick"));
    bool refused_units=false;try {read_checkpoint_wire(malformed.get());}catch(const std::invalid_argument&){refused_units=true;}assert(refused_units);
    malformed=copy(persisted.get());put(field(malformed.get(),"identity"),"preparation_ref",json_object_new_string("foreign:physical-preparation"));
    assert(!body.restore_checkpoint(read_checkpoint_wire(malformed.get()),original.body_revision,256));
    assert(body.samples_elapsed()==256 && body.checkpoint().displacement_modal_metres==saved.displacement_modal_metres);
    auto output=own(json_object_new_object());auto receipt=json_object_new_object();
    string(receipt,"contract",ql::physical_body_contract);string(receipt,"event_ref",original.event_ref);
    string(receipt,"preparation_ref",body.preparation_ref());string(receipt,"state_ref",body.state_ref());
    put(receipt,"body_revision",json_object_new_uint64(body.body_revision()));put(receipt,"samples_elapsed",json_object_new_uint64(body.samples_elapsed()));
    put(receipt,"pickup_linear",json_object_new_double(observation.pickup_linear));put(receipt,"mechanical_energy_joules",json_object_new_double(body.mechanical_energy_joules()));
    put(output.get(),"checkpoint",json_object_get(persisted.get()));
    put(output.get(),"receipt",receipt);auto modes=json_object_new_array();for(double f:frequencies)json_object_array_add(modes,json_object_new_double(f));put(output.get(),"frequencies_hz",modes);
    auto vertices=json_object_new_array();for(const auto& node:original.nodes) { auto xyz=json_object_new_array();for(double x:node.rest_metres)json_object_array_add(xyz,json_object_new_double(x));json_object_array_add(vertices,xyz); }
    put(output.get(),"rest_metres",vertices);put(output.get(),"negative_cases",json_object_new_int(12));
    std::cout<<json_object_to_json_string_ext(output.get(),JSON_C_TO_STRING_PLAIN)<<'\n';
}
