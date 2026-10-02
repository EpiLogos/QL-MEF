#ifndef QL_PHYSICAL_BODY_WIRE_HPP
#define QL_PHYSICAL_BODY_WIRE_HPP
// Control-thread transport admission into the existing physical body owner.
// The caller supplies the actual Rust M3 producer and separately admitted face;
// neither this envelope nor a printable reference authenticates host authority.
// No JSON, registry lookup, eigen solve or allocation belongs in the callback.
#include <ql/physical_body.hpp>
#include <json-c/json.h>
#include <charconv>
#include <memory>
#include <set>

namespace ql::physical_wire {
using J=json_object;
using Json=std::unique_ptr<J,decltype(&json_object_put)>;
inline Json own(J* value) { return Json(value,json_object_put); }
inline J* field(J* object,const char* key) {
    J* value=nullptr;
    require(object && json_object_get_type(object)==json_type_object && json_object_object_get_ex(object,key,&value),
        "missing physical preparation field"); return value;
}
inline void keys(J* object,std::initializer_list<const char*> allowed) {
    require(object && json_object_get_type(object)==json_type_object,"physical object required");
    std::set<std::string> expected(allowed.begin(),allowed.end());
    require(json_object_object_length(object)==int(expected.size()),"physical fields missing/unknown");
    json_object_object_foreach(object,key,value) { (void)value; require(expected.count(key),"unknown physical field"); }
}
inline std::string text(J* value) {
    require(value && json_object_get_type(value)==json_type_string,"physical text required");
    std::string result(json_object_get_string(value),std::size_t(json_object_get_string_len(value))); reference(result); return result;
}
inline double number(J* value) {
    require(value && (json_object_get_type(value)==json_type_double || json_object_get_type(value)==json_type_int),"physical number required");
    const double result=json_object_get_double(value);require(std::isfinite(result),"nonfinite physical transport number");return result;
}
inline std::uint64_t exact(J* value) {
    const double n=number(value);require(n>=0 && n<=9007199254740991.0 && std::floor(n)==n,"inexact physical transport identity");return std::uint64_t(n);
}
inline bool boolean(J* value) {
    require(value && json_object_get_type(value)==json_type_boolean,"physical boolean required");return json_object_get_boolean(value);
}
inline std::size_t count(J* value,std::size_t maximum) {
    require(value && json_object_get_type(value)==json_type_array,"physical array required");
    const auto n=json_object_array_length(value);require(n<=maximum,"physical transport resource budget exceeded");return n;
}
inline J* at(J* value,std::size_t index) {
    require(index<count(value,4096),"physical array index out of range");return json_object_array_get_idx(value,index);
}
inline Vec3 vec(J* value) {
    require(count(value,3)==3,"physical vector must have three components");return {number(at(value,0)),number(at(value,1)),number(at(value,2))};
}
inline Json parse_native(const char* source) {
    require(source,"native source JSON missing");auto result=own(json_tokener_parse(source));require(bool(result),"native source JSON invalid");return result;
}
inline void same_text(J* value,const char* expected) { require(expected && text(value)==expected,"physical source field differs from native owner"); }
inline void record(J* wire,const QL_M_SourceRecord* native) {
    require(native,"physical source record unavailable");
    keys(wire,{"repository","revision","source_path","git_blob","file_sha256","record_class","record_index","payload_sha256"});
    const auto* file=ql_m_live_source_file_at(native->file_index);
    const auto* origin=ql_m_live_source_origin_at(native->file_index);require(file && origin,"native source provenance unavailable");
    same_text(field(wire,"repository"),origin->repository);same_text(field(wire,"revision"),origin->revision);
    same_text(field(wire,"source_path"),file->path);same_text(field(wire,"git_blob"),file->git_blob);
    same_text(field(wire,"file_sha256"),file->sha256);same_text(field(wire,"record_class"),file->record_class);
    require(exact(field(wire,"record_index"))==native->record_index,"native source record index differs");
    same_text(field(wire,"payload_sha256"),native->payload_sha256);
}
/// Native registry comparison includes literal notation and full qualified
/// record evidence. No parent fallback, separator rewriting or implicit face.
inline const QL_M_Node* coordinate(J* wire,bool admitted_pratibimba) {
    keys(wire,{"source_ref","canonical_ref","root","path","separators","face","parent_source_ref","aliases","provenance","payloads"});
    const auto source=text(field(wire,"source_ref"));const auto* node=ql_m_live_resolve(source.c_str());
    require(node && source==node->source_ref,"physical source reference must be exact native spelling");
    const std::string face=admitted_pratibimba?"pratibimba":"bimba";require(text(field(wire,"face"))==face,"physical writer face differs from admission");
    require(source.size()>=2 && source[0]=='#',"physical source coordinate syntax");
    require(text(field(wire,"canonical_ref"))=="ql:m-coordinate:"+face+":M"+source.substr(1),"physical canonical face/reference differs");
    require(exact(field(wire,"root"))==node->root_position,"physical root differs from native source");
    auto path=field(wire,"path"),separators=field(wire,"separators");
    const auto parts=count(path,32);require(count(separators,32)==parts,"physical separator/path dimensions differ");
    std::size_t cursor=2,index=0;
    while (cursor<source.size()) {
        require(index<parts,"physical path missing");const char separator=source[cursor++];
        require(separator=='-' || separator=='.' || separator=='/',"native coordinate separator invalid");
        require(text(at(separators,index))==std::string(1,separator),"physical coordinate separator changed");
        const auto begin=cursor;while(cursor<source.size() && source[cursor]>='0' && source[cursor]<='9')++cursor;
        std::uint16_t component=0;const auto parsed=std::from_chars(source.data()+begin,source.data()+cursor,component);
        require(begin<cursor && parsed.ec==std::errc() && parsed.ptr==source.data()+cursor && exact(at(path,index))==component,
            "physical path component differs from native source");++index;
    }
    require(index==parts,"physical path has extra components");
    const auto* parent=ql_m_live_parent(node->id);auto parent_wire=field(wire,"parent_source_ref");
    if (parent && std::string(parent->source_ref)!="M") same_text(parent_wire,parent->source_ref);
    else require(!parent_wire || json_object_get_type(parent_wire)==json_type_null,"physical source parent differs");
    auto aliases=parse_native(node->aliases_json);require(json_object_equal(field(wire,"aliases"),aliases.get()),"physical source aliases differ");
    auto provenance=field(wire,"provenance"),payloads=field(wire,"payloads");
    require(count(provenance,1024)==node->records_count && count(payloads,1024)==node->records_count,"physical source records omitted/added");
    for(std::size_t i=0;i<node->records_count;++i) {
        const auto* native=ql_m_live_node_record_at(node->id,i);record(at(provenance,i),native);
        auto payload=at(payloads,i);keys(payload,{"record","property_keys"});record(field(payload,"record"),native);
        auto properties=parse_native(native->property_keys_json);
        require(json_object_equal(field(payload,"property_keys"),properties.get()),"physical source property evidence differs");
    }
    return node;
}
inline void provenance(J* source,std::string& ref,std::string& revision,std::string& source_ref,std::string& standing) {
    keys(source,{"reference","revision","source_ref","standing"});ref=text(field(source,"reference"));revision=text(field(source,"revision"));
    source_ref=text(field(source,"source_ref"));standing=text(field(source,"standing"));
    require(standing=="source_authored" || standing=="ratified" || standing=="reference" || standing=="agent_proposed" || standing=="measured",
        "unknown physical model standing");
}
inline PhysicalProjection projection(J* source,std::size_t nodes) {
    keys(source,{"axis","node_weights"});PhysicalProjection result{vec(field(source,"axis")),{}};
    auto weights=field(source,"node_weights");require(count(weights,physical_max_nodes)==nodes,"physical projection node basis differs");
    result.node_weights.reserve(nodes);for(std::size_t i=0;i<nodes;++i)result.node_weights.push_back(number(at(weights,i)));return result;
}
inline PhysicalBodyInput read_prepared_physical_body(J* preparation,J* current_m3,bool admitted_pratibimba) {
    keys(preparation,{"schema","contract","event_ref","subject_ref","source_coordinate","source_revision","domain_revision","source_generation",
        "address","pose_ordinal","clock","aperture","form","request","units"});
    same_text(field(preparation,"schema"),"ql.physical-body-preparation/v1");same_text(field(preparation,"contract"),physical_body_contract);
    same_text(field(current_m3,"schema"),"ql.m3-state/v1");
    require(ql_m_live_accepts_base(text(field(current_m3,"registry_revision")).c_str()),"M3 registry not current admitted basis");
    auto identity=field(current_m3,"identity");auto form=field(current_m3,"form");
    PhysicalBodyInput input{};input.event_ref=text(field(preparation,"event_ref"));input.subject_ref=text(field(preparation,"subject_ref"));
    require(input.event_ref==text(field(identity,"event_ref")) && input.subject_ref==text(field(current_m3,"subject_ref")),"physical body disconnected from actual M3 event/subject");
    input.source_revision=text(field(preparation,"source_revision"));
    require(input.source_revision==text(field(current_m3,"source_revision"))
        && text(field(preparation,"domain_revision"))==text(field(current_m3,"domain_revision")),"physical body source/domain revision stale");
    input.source_generation=exact(field(preparation,"source_generation"));
    require(input.source_generation==exact(field(identity,"profile_generation")),"physical body M3 generation stale");
    const auto* node=coordinate(field(preparation,"source_coordinate"),admitted_pratibimba);
    require(node->root_position==3 && std::string(node->source_ref)==text(field(field(form,"codon"),"ref")),"physical body is not actual M3 form branch");
    input.source_coordinate=node->source_ref;input.pratibimba=admitted_pratibimba;
    require(exact(field(preparation,"address"))==exact(field(form,"address"))
        && exact(field(preparation,"pose_ordinal"))==exact(field(form,"pose_ordinal")),"physical body source form/pose stale");
    for(const auto* name:{"form","clock","aperture"})require(json_object_equal(field(preparation,name),field(current_m3,name)),"physical source owner packet disconnected/stale");
    auto units=field(preparation,"units");
    keys(units,{"position","displacement","mass","density","elastic_modulus","section","prestress","excitation","impulse","pickup","pickup_gain","energy","rayleigh_alpha","rayleigh_beta"});
    for(const auto& unit:{std::pair{"position","m"},{"displacement","m"},{"mass","kg"},{"density","kg/m^3"},
        {"elastic_modulus","Pa"},{"section","m^2"},{"prestress","N"},{"excitation","N"},{"impulse","N*s"},
        {"pickup","linear"},{"pickup_gain","linear/m"},{"energy","J"},{"rayleigh_alpha","s^-1"},{"rayleigh_beta","s"}})
        same_text(field(units,unit.first),unit.second);
    auto request=field(preparation,"request");
    keys(request,{"expected_m3_generation","body_revision","preparation_ref","state_ref","geometry","material","sample_rate","exciter","pickup",
        "pickup_linear_per_metre","max_force_newtons","max_impulse_newton_seconds","max_displacement_metres"});
    require(exact(field(request,"expected_m3_generation"))==input.source_generation,"physical request producer generation differs");
    input.body_revision=exact(field(request,"body_revision"));input.preparation_ref=text(field(request,"preparation_ref"));input.state_ref=text(field(request,"state_ref"));
    const auto rate=exact(field(request,"sample_rate"));require(rate<=192000,"physical sample rate exceeds budget");input.sample_rate=unsigned(rate);
    auto geometry=field(request,"geometry");keys(geometry,{"provenance","family","nodes","edges"});
    provenance(field(geometry,"provenance"),input.geometry_ref,input.geometry_revision,input.geometry_source_ref,input.geometry_standing);
    const auto family=text(field(geometry,"family"));require(family=="axial-truss" || family=="prestressed-tension-network","unsupported physical family");
    input.family=family=="axial-truss"?BodyFamily::AxialTruss:BodyFamily::PrestressedTensionNetwork;
    auto nodes=field(geometry,"nodes");const auto n=count(nodes,physical_max_nodes);input.nodes.reserve(n);
    for(std::size_t i=0;i<n;++i) {
        auto source=at(nodes,i);keys(source,{"identity","constituent","rest_metres","additional_mass_kg","fixed"});
        auto fixed=field(source,"fixed");require(count(fixed,3)==3,"physical fixed axes dimensions");
        input.nodes.push_back({exact(field(source,"identity")),text(field(source,"constituent")),vec(field(source,"rest_metres")),
            number(field(source,"additional_mass_kg")),{boolean(at(fixed,0)),boolean(at(fixed,1)),boolean(at(fixed,2))}});
    }
    auto edges=field(geometry,"edges");const auto edge_count=count(edges,physical_max_edges);input.edges.reserve(edge_count);
    for(std::size_t i=0;i<edge_count;++i) {
        auto source=at(edges,i);keys(source,{"first","second","section_m2","prestress_newtons"});
        const auto first=exact(field(source,"first")),second=exact(field(source,"second"));require(first<n && second<n,"physical edge index out of bounds");
        input.edges.push_back({std::size_t(first),std::size_t(second),number(field(source,"section_m2")),number(field(source,"prestress_newtons"))});
    }
    auto material=field(request,"material");keys(material,{"provenance","young_modulus_pa","density_kg_per_m3","damping_alpha_per_second","damping_beta_seconds"});
    provenance(field(material,"provenance"),input.material.reference,input.material.revision,input.material.source_ref,input.material.standing);
    input.material.young_modulus_pa=number(field(material,"young_modulus_pa"));input.material.density_kg_per_m3=number(field(material,"density_kg_per_m3"));
    input.material.damping_alpha_per_second=number(field(material,"damping_alpha_per_second"));input.material.damping_beta_seconds=number(field(material,"damping_beta_seconds"));
    input.exciter=projection(field(request,"exciter"),n);input.pickup=projection(field(request,"pickup"),n);
    input.pickup_linear_per_metre=number(field(request,"pickup_linear_per_metre"));input.max_force_newtons=number(field(request,"max_force_newtons"));
    input.max_impulse_newton_seconds=number(field(request,"max_impulse_newton_seconds"));input.max_displacement_metres=number(field(request,"max_displacement_metres"));
    return input; // PreparedPhysicalBody then performs all numerical admission.
}
} // namespace ql::physical_wire
#endif
