#ifndef QL_PHYSICAL_CHECKPOINT_WIRE_HPP
#define QL_PHYSICAL_CHECKPOINT_WIRE_HPP
// Persistence transport for the existing PhysicalBody owner. Exact integer
// identities/cursors use canonical decimal strings; binary64 modal state is
// retained at full JSON double precision. Reading is not host authorization.
#include <ql/physical_body_wire.hpp>
namespace ql::physical_wire {
inline void checkpoint_put(J* object,const char* key,J* value) {
    require(value && json_object_object_add(object,key,value)==0,"physical checkpoint JSON allocation failed");
}
inline void checkpoint_string(J* object,const char* key,const std::string& value) {
    checkpoint_put(object,key,json_object_new_string_len(value.data(),int(value.size())));
}
inline void checkpoint_integer(J* object,const char* key,std::uint64_t value) { checkpoint_string(object,key,std::to_string(value)); }
inline std::uint64_t checkpoint_decimal(J* value) {
    const auto source=text(value);std::uint64_t result=0;const auto parsed=std::from_chars(source.data(),source.data()+source.size(),result);
    require(parsed.ec==std::errc() && parsed.ptr==source.data()+source.size() && std::to_string(result)==source,"invalid canonical checkpoint integer");return result;
}
inline J* checkpoint_doubles(const std::vector<double>& values) {
    require(values.size()<=physical_max_dofs,"checkpoint modal budget exceeded");auto result=own(json_object_new_array());require(bool(result),"checkpoint array allocation failed");
    for(double value:values){require(std::isfinite(value),"nonfinite checkpoint modal value");require(json_object_array_add(result.get(),json_object_new_double(value))==0,"checkpoint array allocation failed");}
    return result.release();
}
inline Json checkpoint_wire(const PhysicalBodyCheckpoint& saved) {
    auto result=own(json_object_new_object());require(bool(result),"checkpoint object allocation failed");
    checkpoint_string(result.get(),"schema",saved.contract);checkpoint_put(result.get(),"version",json_object_new_int64(saved.version));
    auto identity=own(json_object_new_object());require(bool(identity),"checkpoint identity allocation failed");
    for(const auto& entry:{std::pair{"event_ref",&saved.event_ref},{"subject_ref",&saved.subject_ref},{"preparation_ref",&saved.preparation_ref},
        {"state_ref",&saved.state_ref},{"source_coordinate",&saved.source_coordinate},{"source_revision",&saved.source_revision},
        {"geometry_ref",&saved.geometry_ref},{"geometry_revision",&saved.geometry_revision},{"geometry_source_ref",&saved.geometry_source_ref},
        {"geometry_standing",&saved.geometry_standing},{"material_ref",&saved.material_ref},{"material_revision",&saved.material_revision},
        {"material_source_ref",&saved.material_source_ref},{"material_standing",&saved.material_standing}})checkpoint_string(identity.get(),entry.first,*entry.second);
    checkpoint_string(identity.get(),"face",saved.pratibimba?"pratibimba":"bimba");
    checkpoint_integer(identity.get(),"source_generation",saved.source_generation);checkpoint_integer(identity.get(),"body_revision",saved.body_revision);
    checkpoint_put(result.get(),"identity",identity.release());
    auto basis=own(json_object_new_object());require(bool(basis),"checkpoint basis allocation failed");
    checkpoint_string(basis.get(),"eigenbasis_identity",saved.eigenbasis_identity);checkpoint_put(basis.get(),"sample_rate",json_object_new_uint64(saved.sample_rate));
    checkpoint_put(result.get(),"basis",basis.release());
    auto units=own(json_object_new_object());require(bool(units),"checkpoint units allocation failed");
    checkpoint_string(units.get(),"displacement",saved.displacement_unit);checkpoint_string(units.get(),"velocity",saved.velocity_unit);checkpoint_string(units.get(),"pickup",saved.pickup_unit);
    require(std::isfinite(saved.generalized_modal_mass_kg),"nonfinite modal mass");checkpoint_put(units.get(),"generalized_modal_mass_kg",json_object_new_double(saved.generalized_modal_mass_kg));
    checkpoint_put(result.get(),"units",units.release());
    auto state=own(json_object_new_object());require(bool(state),"checkpoint state allocation failed");
    checkpoint_integer(state.get(),"samples_elapsed",saved.samples_elapsed);checkpoint_put(state.get(),"displacement_modal_metres",checkpoint_doubles(saved.displacement_modal_metres));
    checkpoint_put(state.get(),"velocity_modal_metres_per_second",checkpoint_doubles(saved.velocity_modal_metres_per_second));
    require(std::isfinite(saved.last_pickup_linear),"nonfinite checkpoint pickup");checkpoint_put(state.get(),"last_pickup_linear",json_object_new_double(saved.last_pickup_linear));
    checkpoint_put(result.get(),"state",state.release());return result;
}
inline PhysicalBodyCheckpoint read_checkpoint_wire(J* source) {
    keys(source,{"schema","version","identity","basis","units","state"});PhysicalBodyCheckpoint saved;
    saved.contract=text(field(source,"schema"));require(saved.contract==physical_checkpoint_contract,"unsupported physical checkpoint contract");
    const auto version=exact(field(source,"version"));require(version==1,"unsupported physical checkpoint version");saved.version=1;
    auto identity=field(source,"identity");
    keys(identity,{"event_ref","subject_ref","preparation_ref","state_ref","source_coordinate","source_revision","geometry_ref","geometry_revision",
        "geometry_source_ref","geometry_standing","material_ref","material_revision","material_source_ref","material_standing","face","source_generation","body_revision"});
    for(const auto& entry:{std::pair{"event_ref",&saved.event_ref},{"subject_ref",&saved.subject_ref},{"preparation_ref",&saved.preparation_ref},
        {"state_ref",&saved.state_ref},{"source_coordinate",&saved.source_coordinate},{"source_revision",&saved.source_revision},
        {"geometry_ref",&saved.geometry_ref},{"geometry_revision",&saved.geometry_revision},{"geometry_source_ref",&saved.geometry_source_ref},
        {"geometry_standing",&saved.geometry_standing},{"material_ref",&saved.material_ref},{"material_revision",&saved.material_revision},
        {"material_source_ref",&saved.material_source_ref},{"material_standing",&saved.material_standing}})*entry.second=text(field(identity,entry.first));
    const auto face=text(field(identity,"face"));require(face=="pratibimba" || face=="bimba","invalid checkpoint source face");saved.pratibimba=face=="pratibimba";
    saved.source_generation=checkpoint_decimal(field(identity,"source_generation"));saved.body_revision=checkpoint_decimal(field(identity,"body_revision"));
    auto basis=field(source,"basis");keys(basis,{"eigenbasis_identity","sample_rate"});saved.eigenbasis_identity=text(field(basis,"eigenbasis_identity"));
    const auto rate=exact(field(basis,"sample_rate"));require(rate>=8000 && rate<=192000,"checkpoint sample rate outside physical budget");saved.sample_rate=unsigned(rate);
    auto units=field(source,"units");keys(units,{"displacement","velocity","pickup","generalized_modal_mass_kg"});
    saved.displacement_unit=text(field(units,"displacement"));saved.velocity_unit=text(field(units,"velocity"));saved.pickup_unit=text(field(units,"pickup"));saved.generalized_modal_mass_kg=number(field(units,"generalized_modal_mass_kg"));
    require(saved.displacement_unit=="m" && saved.velocity_unit=="m/s" && saved.pickup_unit=="linear" && saved.generalized_modal_mass_kg==1,"physical checkpoint modal units differ");
    auto state=field(source,"state");keys(state,{"samples_elapsed","displacement_modal_metres","velocity_modal_metres_per_second","last_pickup_linear"});
    saved.samples_elapsed=checkpoint_decimal(field(state,"samples_elapsed"));auto q=field(state,"displacement_modal_metres"),v=field(state,"velocity_modal_metres_per_second");
    const auto modes=count(q,physical_max_dofs);require(modes>0 && count(v,physical_max_dofs)==modes,"checkpoint modal dimensions differ");
    saved.displacement_modal_metres.reserve(modes);saved.velocity_modal_metres_per_second.reserve(modes);
    for(std::size_t i=0;i<modes;++i){saved.displacement_modal_metres.push_back(number(at(q,i)));saved.velocity_modal_metres_per_second.push_back(number(at(v,i)));}
    saved.last_pickup_linear=number(field(state,"last_pickup_linear"));return saved;
}
} // namespace ql::physical_wire
#endif
