#ifndef QL_PERFORMANCE_PACKET_HPP
#define QL_PERFORMANCE_PACKET_HPP
// Control-thread consumer of the actual Rust native admission output. Existing
// json-c is already a field-worker dependency. No parsing runs in the callback.
#include <ql/performance_physical.hpp>
#include <ql/physical_body_wire.hpp>
#include <json-c/json.h>
#include <set>
#include <string>
#include <vector>
namespace ql::performance {
struct NativePerformance {
    std::shared_ptr<ql::PhysicalBody> body;
    std::shared_ptr<Engine> engine;
    Determination determination{};
    std::vector<NoteTarget> notes;
};
namespace packet {
inline json_object* field(json_object* object,const char* name) {
    json_object* value=nullptr;
    if(!object || !json_object_is_type(object,json_type_object) || !json_object_object_get_ex(object,name,&value) || !value)
        throw std::invalid_argument(std::string("missing native performance field: ")+name);
    return value;
}
inline std::string string(json_object* value) {
    if(!value || !json_object_is_type(value,json_type_string)) throw std::invalid_argument("native performance string required");
    const auto length=json_object_get_string_len(value); const char* text=json_object_get_string(value);
    if(length<=0 || length>=256 || std::strlen(text)!=std::size_t(length)) throw std::invalid_argument("bounded native performance string required");
    return text;
}
inline Ref ref(json_object* object,const char* name) {return reference(string(field(object,name)).c_str());}
inline double number(json_object* value) {
    if(!value || (!json_object_is_type(value,json_type_double) && !json_object_is_type(value,json_type_int))) throw std::invalid_argument("native number required");
    const double result=json_object_get_double(value); if(!std::isfinite(result)) throw std::invalid_argument("finite native number required"); return result;
}
inline std::uint64_t integer(json_object* value) {
    if(json_object_is_type(value,json_type_string)) {
        const auto text=string(value); std::uint64_t out=0;
        if(text.size()>20 || (text.size()>1 && text[0]=='0')) throw std::invalid_argument("canonical native integer string required");
        for(char c:text) {
            if(c<'0' || c>'9' || out>(std::numeric_limits<std::uint64_t>::max()-std::uint64_t(c-'0'))/10)
                throw std::invalid_argument("native integer overflow");
            out=out*10+std::uint64_t(c-'0');
        } return out;
    }
    if(!json_object_is_type(value,json_type_int) || json_object_get_int64(value)<0
        || json_object_get_uint64(value)>9007199254740991ULL) throw std::invalid_argument("exact native integer required");
    return json_object_get_uint64(value);
}
inline std::uint8_t byte(json_object* value) {const auto x=integer(value); if(x>255) throw std::invalid_argument("bounded native byte required"); return std::uint8_t(x);}
inline bool boolean(json_object* value) {
    if(!value || !json_object_is_type(value,json_type_boolean)) throw std::invalid_argument("native boolean required"); return json_object_get_boolean(value)!=0;
}
inline void array(json_object* value,std::size_t size) {
    if(!value || !json_object_is_type(value,json_type_array) || json_object_array_length(value)!=size) throw std::invalid_argument("native array cardinality mismatch");
}
inline ql::Vec3 vector(json_object* value) {
    array(value,3); return {number(json_object_array_get_idx(value,0)),number(json_object_array_get_idx(value,1)),number(json_object_array_get_idx(value,2))};
}
inline Identity identity(json_object* value) {
    Identity out{}; out.instance=ref(value,"instance"); out.event=ref(value,"event"); out.subject=ref(value,"subject");
    out.m1_revision=integer(field(value,"m1_revision")); out.m2_generation=integer(field(value,"m2_generation")); return out;
}
inline Determination determination(json_object* value) {
    Determination d{}; d.identity=identity(field(value,"identity"));
    d.m1_coordinate=ref(value,"m1_coordinate"); d.m2_writer=ref(value,"m2_writer"); d.registry_revision=ref(value,"registry_revision");
    d.source_revision=ref(value,"source_revision"); d.relation_plan_ref=ref(value,"relation_plan_ref"); d.tuning_ref=ref(value,"tuning_ref");
    d.native_receipt_ref=ref(value,"native_receipt_ref"); d.body_preparation_ref=ref(value,"body_preparation_ref"); d.body_state_ref=ref(value,"body_state_ref");
    d.m1_face=byte(field(value,"m1_face")); d.m2_face=byte(field(value,"m2_face")); d.tick12=byte(field(value,"tick12"));
    const auto degree=integer(field(value,"degree720")); if(degree>=720) throw std::invalid_argument("native degree720 outside bound"); d.degree720=std::uint16_t(degree);
    d.basis=byte(field(value,"basis")); d.lens12=byte(field(value,"lens12")); d.context_frame=byte(field(value,"context_frame"));
    d.body_revision=integer(field(value,"body_revision")); d.tuning_available=boolean(field(value,"tuning_available"));
    auto* audio=field(value,"audio_octet_hz"); array(audio,8);
    for(std::size_t i=0;i<8;++i) d.audio_octet_hz[i]=number(json_object_array_get_idx(audio,i));
    auto* nodal=field(value,"nodal_quartet"); array(nodal,4);
    for(std::size_t i=0;i<4;++i) {auto* n=json_object_array_get_idx(nodal,i); d.nodal_quartet[i]={byte(field(n,"position")),byte(field(n,"face")),byte(field(n,"m")),byte(field(n,"n"))};}
    return d;
}
inline NoteTarget note(json_object* value) {
    NoteTarget n{}; n.identity=identity(field(value,"identity")); n.source_coordinate=ref(value,"source_coordinate"); n.tuning_ref=ref(value,"tuning_ref");
    n.member=integer(field(value,"member")); n.touch=integer(field(value,"touch"));
    n.ratio_numerator=integer(field(value,"ratio_numerator")); n.ratio_denominator=integer(field(value,"ratio_denominator"));
    n.key=byte(field(value,"key")); n.position=byte(field(value,"position")); n.coordinate_face=byte(field(value,"coordinate_face")); n.source_face=byte(field(value,"source_face")); n.pitch_class=byte(field(value,"pitch_class"));
    auto* reg=field(value,"register_octave"); if(!json_object_is_type(reg,json_type_int)) throw std::invalid_argument("native signed register required");
    const auto octave=json_object_get_int64(reg); if(octave<-128 || octave>127) throw std::invalid_argument("native register outside i8"); n.register_octave=std::int8_t(octave);
    n.fundamental_hz=number(field(value,"fundamental_hz")); n.hertz=number(field(value,"hertz")); n.phase_sin=number(field(value,"phase_sin")); n.phase_cos=number(field(value,"phase_cos")); n.exact_ratio=boolean(field(value,"exact_ratio"));
    return n;
}
} // namespace packet
// Call only on the native serial owner, using its source-qualified Rust output.
// This parse/prepare operation is coherence checking, never graph authentication.
inline NativePerformance prepare_performance_packet(const std::string& text,json_object* actual_native_basis,
    bool admitted_m1_pratibimba,bool admitted_physical_pratibimba,Parameters parameters={}) {
    if(text.empty() || text.size()>4*1024*1024) throw std::invalid_argument("performance preparation message budget exceeded");
    auto* tokener=json_tokener_new_ex(64); if(!tokener) throw std::bad_alloc();
    json_tokener_set_flags(tokener,JSON_TOKENER_STRICT);
    auto* raw=json_tokener_parse_ex(tokener,text.data(),int(text.size()));
    const auto error=json_tokener_get_error(tokener); const auto end=json_tokener_get_parse_end(tokener); json_tokener_free(tokener);
    std::unique_ptr<json_object,decltype(&json_object_put)> root(raw,json_object_put);
    if(error!=json_tokener_success || !raw || text.find_first_not_of(" \r\n\t",end)!=std::string::npos) throw std::invalid_argument("strict complete native preparation JSON required");
    if(packet::string(packet::field(raw,"schema"))!="ql.performance-preparation/v1" || packet::string(packet::field(raw,"callback_contract"))!=contract) throw std::invalid_argument("native performance preparation contract required");
    NativePerformance out{}; out.determination=packet::determination(packet::field(raw,"determination"));
    if(!actual_native_basis || !json_object_equal(packet::field(raw,"native_basis"),actual_native_basis))
        throw std::invalid_argument("performance consumer disconnected from actual native producer");
    auto* m1=packet::field(actual_native_basis,"m1"); auto* config=packet::field(m1,"config");
    if(packet::integer(packet::field(config,"revision"))!=out.determination.identity.m1_revision
        || packet::ref(config,"event_ref")!=out.determination.identity.event
        || out.determination.m1_face!=unsigned(admitted_m1_pratibimba)) throw std::invalid_argument("M1 source/revision/face disconnected");
    const auto* source=ql_m_live_resolve(packet::string(packet::field(config,"selected_coordinate")).c_str());
    if(!source || source->source_ref!=packet::string(packet::field(packet::field(raw,"determination"),"m1_coordinate")))
        throw std::invalid_argument("M1 exact selected source branch disconnected");
    auto* m2=packet::field(actual_native_basis,"m2"); auto* m2identity=packet::field(m2,"identity");
    if(packet::integer(packet::field(m2identity,"profile_generation"))!=out.determination.identity.m2_generation
        || packet::ref(m2identity,"event_ref")!=out.determination.identity.event) throw std::invalid_argument("M2 source generation/event disconnected");
    auto* bus=packet::field(packet::field(m2,"vimarsha"),"reading");
    if(!json_object_equal(packet::field(packet::field(raw,"determination"),"audio_octet_hz"),packet::field(bus,"audio_octet_hz")))
        throw std::invalid_argument("populated octet disconnected from actual M2 writer");
    auto* nodal=packet::field(bus,"nodal_quartet"); packet::array(nodal,4);
    for(std::size_t i=0;i<4;++i) {
        auto* n=json_object_array_get_idx(nodal,i); const auto helix=packet::string(packet::field(n,"helix"));
        if(helix!="bimba" && helix!="pratibimba") throw std::invalid_argument("native nodal helix unknown");
        const auto& target=out.determination.nodal_quartet[i];
        if(target.position!=packet::byte(packet::field(n,"ql_position")) || target.face!=unsigned(helix=="pratibimba")
            || target.m!=packet::byte(packet::field(n,"m")) || target.n!=packet::byte(packet::field(n,"n")))
            throw std::invalid_argument("populated nodal quartet disconnected from actual M2 writer");
    }
    auto physical=ql::physical_wire::read_prepared_physical_body(packet::field(raw,"physical_body"),packet::field(actual_native_basis,"m3"),admitted_physical_pratibimba);
    out.body=std::make_shared<ql::PhysicalBody>(ql::PreparedPhysicalBody(std::move(physical)));
    out.engine=std::make_shared<Engine>(out.determination,out.body->preparation().input().sample_rate,physical_port(out.body),parameters);
    auto* notes=packet::field(raw,"notes"); const auto count=json_object_is_type(notes,json_type_array)?json_object_array_length(notes):0;
    if(!count || count>max_touches) throw std::invalid_argument("native prepared note budget exceeded");
    std::set<std::uint64_t> touches; out.notes.reserve(count);
    for(std::size_t i=0;i<count;++i) {
        auto target=packet::note(json_object_array_get_idx(notes,i));
        if(!out.engine->validate_note_target(target) || !touches.insert(target.touch).second) throw std::invalid_argument("disconnected native prepared note");
        out.notes.push_back(target);
    } return out;
}
} // namespace ql::performance
#endif
