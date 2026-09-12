// Bounded newline-JSON management worker over the real C/C++ owner. JSON work
// stays outside render_audio. Native audio hosts call the typed library directly.
#include <ql/continuous_field.hpp>
#include <json-c/json.h>
#include <charconv>
#include <iostream>
#include <memory>
#include <set>

using J = json_object;
using Json = std::unique_ptr<J, decltype(&json_object_put)>;
Json own(J *p) { return Json(p, &json_object_put); }
J *get(J *o, const char *key) { J *v = nullptr; ql::require(json_object_object_get_ex(o, key, &v), "missing field"); return v; }
void keys(J *o, std::initializer_list<const char *> names) {
    ql::require(o && json_object_get_type(o) == json_type_object, "expected object");
    std::set<std::string> allowed(names.begin(), names.end());
    ql::require(json_object_object_length(o) == int(allowed.size()), "missing/unknown fields");
    json_object_object_foreach(o, k, v) { (void)v; ql::require(allowed.count(k), "unknown field"); }
}
std::string text(J *v) { ql::require(v && json_object_get_type(v) == json_type_string, "expected text"); return {json_object_get_string(v), std::size_t(json_object_get_string_len(v))}; }
double number(J *v) { ql::require(v && (json_object_get_type(v) == json_type_int || json_object_get_type(v) == json_type_double), "expected number"); auto n = json_object_get_double(v); ql::require(std::isfinite(n), "nonfinite number"); return n; }
unsigned small(J *v, unsigned max = UINT32_MAX) { auto n = number(v); ql::require(n >= 0 && n <= max && std::floor(n) == n, "invalid bounded integer"); return unsigned(n); }
bool boolean(J *v) { ql::require(v && json_object_get_type(v) == json_type_boolean, "expected boolean"); return json_object_get_boolean(v); }
template <typename T> T decimal(J *v) { auto s = text(v); T value{}; auto r = std::from_chars(s.data(), s.data() + s.size(), value); ql::require(r.ec == std::errc() && r.ptr == s.data() + s.size(), "invalid exact decimal integer"); return value; }
std::uint64_t exact_number(J *v) { auto n = number(v); ql::require(n >= 0 && n <= 9007199254740991.0 && std::floor(n) == n, "inexact numeric identity"); return std::uint64_t(n); }
std::size_t count(J *a, std::size_t max) { ql::require(a && json_object_get_type(a) == json_type_array, "expected array"); auto n = json_object_array_length(a); ql::require(n <= max, "array budget exceeded"); return n; }
J *at(J *a, std::size_t i) { ql::require(i < count(a, 16777216), "array index out of range"); return json_object_array_get_idx(a, i); }
ql::Complex pair(J *a) { ql::require(count(a, 2) == 2, "expected complex pair"); return {number(at(a, 0)), number(at(a, 1))}; }
ql::Vec3 vec(J *a) { ql::require(count(a, 3) == 3, "expected vector3"); return {number(at(a, 0)), number(at(a, 1)), number(at(a, 2))}; }
void put(J *o, const char *k, J *v) { ql::require(json_object_object_add(o, k, v) == 0, "JSON allocation failed"); }
void string(J *o, const char *k, const std::string &s) { put(o, k, json_object_new_string_len(s.data(), int(s.size()))); }
void integer(J *o, const char *k, std::uint64_t n) { string(o, k, std::to_string(n)); }
J *phase(QL_PhaseLift p) { J *o = json_object_new_object(); string(o, "turns", std::to_string(p.turns)); put(o, "half_degrees", json_object_new_int(p.half_degrees)); put(o, "double_cover_half_degrees", json_object_new_int(ql_phase_double_cover_half_degrees(p))); return o; }
QL_PhaseLift read_phase(J *p) { keys(p, {"turns", "half_degrees"}); return {decimal<std::int64_t>(get(p, "turns")), std::uint16_t(small(get(p, "half_degrees"), 719))}; }
QL_CoupledClock clock_input(J *o) {
    keys(o, {"inscription", "lensing", "grid_origins", "rate_numerators", "rate_denominator", "rate_remainders", "generation"});
    QL_CoupledClock c{}; c.inscription = read_phase(get(o, "inscription")); c.lensing = read_phase(get(o, "lensing"));
    auto origins = get(o, "grid_origins"), rates = get(o, "rate_numerators"), remainder = get(o, "rate_remainders");
    ql::require(count(origins, 3) == 3 && count(rates, 2) == 2 && count(remainder, 2) == 2, "clock array dimensions");
    for (unsigned i = 0; i < 3; ++i) c.grid_origin_half_degrees[i] = std::uint16_t(small(at(origins, i), 719));
    for (unsigned i = 0; i < 2; ++i) { c.rate_numerators[i] = decimal<std::int32_t>(at(rates, i)); c.rate_remainders[i] = decimal<std::int64_t>(at(remainder, i)); }
    c.rate_denominator = small(get(o, "rate_denominator")); c.generation = decimal<std::uint64_t>(get(o, "generation"));
    ql::require(ql_clock_validate(&c) == QL_CLOCK_OK, "native clock validation failed"); return c;
}
J *clock_output(const QL_CoupledClock &c) {
    J *o = json_object_new_object(); put(o, "inscription", phase(c.inscription)); put(o, "lensing", phase(c.lensing));
    integer(o, "generation", c.generation); string(o, "field_ref", ql_clock_field()->source_ref); string(o, "centre_ref", ql_clock_centre()->source_ref);
    J *origins = json_object_new_array(), *rates = json_object_new_array(), *rest = json_object_new_array();
    for (auto n : c.grid_origin_half_degrees) json_object_array_add(origins, json_object_new_int(n));
    for (auto n : c.rate_numerators) json_object_array_add(rates, json_object_new_string(std::to_string(n).c_str()));
    for (auto n : c.rate_remainders) json_object_array_add(rest, json_object_new_string(std::to_string(n).c_str()));
    put(o, "grid_origins", origins); put(o, "rate_numerators", rates); put(o, "rate_remainders", rest);
    put(o, "rate_denominator", json_object_new_int64(c.rate_denominator)); return o;
}
std::vector<ql::Mode> modes(J *m2, J *gains) {
    ql::require(text(get(m2, "schema")) == "ql.m2-engine/v1", "unsupported M2 frame");
    ql::require(ql_m_live_accepts_base(text(get(m2, "registry_revision")).c_str()), "unknown M2 registry basis");
    J *res = get(m2, "resonator"); ql::require(res && json_object_get_type(res) == json_type_object, "M2 has no supplied resonator");
    J *items = get(res, "modes"); auto n = count(items, 4096); ql::require(count(gains, 4096) == n, "gain/mode basis mismatch");
    std::vector<ql::Mode> result; result.reserve(n);
    for (std::size_t i = 0; i < n; ++i) {
        auto item = at(items, i);
        result.push_back({text(get(item, "mode_ref")), text(get(item, "source_coordinate")),
            number(get(item, "frequency_hz")), number(get(item, "damping_per_second")),
            pair(get(item, "amplitude")), pair(get(item, "excitation")), number(at(gains, i))});
    }
    return result;
}
std::uint64_t m2_generation(J *m2) { return exact_number(get(get(m2, "identity"), "profile_generation")); }
ql::ContinuationInput initialize(J *m2, J *field) {
    keys(field, {"subject_ref", "sample_rate", "clock", "driver_numerator", "driver_denominator", "units", "audio_gains", "samples"});
    J *units = get(field, "units"); keys(units, {"amplitude", "excitation", "shape", "position", "audio"});
    for (const auto &entry : {std::pair{"amplitude", "m"}, {"excitation", "m/s"}, {"shape", "dimensionless"}, {"position", "m"}, {"audio", "linear"}})
        ql::require(text(get(units, entry.first)) == entry.second, "unsupported or missing modal units");
    auto material_modes = modes(m2, get(field, "audio_gains"));
    auto res = get(m2, "resonator");
    ql::ContinuationInput input{text(get(get(m2, "identity"), "event_ref")), text(get(field, "subject_ref")),
        text(get(res, "geometry_ref")), text(get(res, "material_ref")), text(get(res, "material_model_ref")),
        m2_generation(m2), small(get(field, "sample_rate")), clock_input(get(field, "clock")),
        small(get(field, "driver_numerator")), small(get(field, "driver_denominator")), std::move(material_modes), {}};
    auto samples = get(field, "samples"); auto n = count(samples, 65536);
    ql::require(!input.modes.empty() && n <= 262144 / input.modes.size(), "JSON shape budget exceeded; use typed host for larger buffers");
    input.samples.reserve(n);
    for (std::size_t i = 0; i < n; ++i) {
        auto s = at(samples, i); keys(s, {"identity", "constituent", "attachment", "rest_metres", "mode_shapes"});
        ql::Sample sample{exact_number(get(s, "identity")), text(get(s, "constituent")),
            ql::ClockAttachment(small(get(s, "attachment"), 2)), vec(get(s, "rest_metres")), {}};
        auto shapes = get(s, "mode_shapes"); ql::require(count(shapes, 4096) == input.modes.size(), "sample modal basis mismatch");
        for (std::size_t j = 0; j < input.modes.size(); ++j) sample.mode_shapes.push_back(vec(at(shapes, j)));
        input.samples.push_back(std::move(sample));
    }
    return input;
}
J *response(const ql::ContinuousField &field, const std::vector<float> &audio, bool targets, double scale) {
    auto r = field.receipt(); auto o = json_object_new_object();
    string(o, "schema", ql::continuous_contract); string(o, "event_ref", field.source().event_ref); string(o, "subject_ref", field.source().subject_ref);
    string(o, "registry_revision", ql_m_live_registry_revision()); string(o, "geometry_ref", field.source().geometry_ref);
    string(o, "material_ref", field.source().material_ref); string(o, "model_ref", field.source().model_ref);
    integer(o, "generation", r.generation); integer(o, "samples_elapsed", r.samples_elapsed); put(o, "clock", clock_output(r.clock));
    string(o, "standing", "computed-supplied-modal-model-not-empirical-material-validation");
    put(o, "sample_rate", json_object_new_int(field.source().sample_rate));
    auto amplitudes = json_object_new_array();
    for (std::size_t i = 0; i < field.source().modes.size(); ++i) {
        auto a = json_object_new_array(); json_object_array_add(a, json_object_new_double(field.amplitude(i).real()));
        json_object_array_add(a, json_object_new_double(field.amplitude(i).imag())); json_object_array_add(amplitudes, a);
    }
    put(o, "amplitudes_metres", amplitudes);
    auto waveform = json_object_new_array(); for (auto v : audio) json_object_array_add(waveform, json_object_new_double(v)); put(o, "audio", waveform);
    auto points = json_object_new_array();
    if (targets) {
        std::vector<float> xyz(field.samples().size() * 3); ql::require(field.write_targets(xyz.data(), xyz.size(), scale), "target projection refused");
        for (std::size_t i = 0; i < field.samples().size(); ++i) {
            auto point = json_object_new_object(); put(point, "identity", json_object_new_uint64(field.samples()[i].identity));
            string(point, "constituent", field.samples()[i].constituent); auto position = json_object_new_array();
            for (unsigned axis = 0; axis < 3; ++axis) json_object_array_add(position, json_object_new_double(xyz[i * 3 + axis]));
            put(point, "position", position); json_object_array_add(points, point);
        }
    }
    put(o, "targets", points); put(o, "presentation_units_per_metre", json_object_new_double(scale)); return o;
}
int main() {
    std::unique_ptr<ql::ContinuousField> field;
    Json basis = own(nullptr), gains = own(nullptr);
    std::string line;
    while (true) {
        line.clear(); char c;
        while (std::cin.get(c) && c != '\n') {
            if (line.size() >= 32 * 1024 * 1024) { std::cerr << "input budget exceeded\n"; return 2; }
            line.push_back(c);
        }
        if (line.empty() && !std::cin) break;
        bool committed = false;
        try {
            auto tok = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(json_tokener_new_ex(64), json_tokener_free);
            json_tokener_set_flags(tok.get(), JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
            auto request = own(json_tokener_parse_ex(tok.get(), line.c_str(), int(line.size())));
            ql::require(json_tokener_get_error(tok.get()) == json_tokener_success &&
                json_tokener_get_parse_end(tok.get()) == line.size(), "invalid complete JSON message");
            auto op = text(get(request.get(), "operation"));
            ql::require(text(get(request.get(), "schema")) == "ql.field-control/v1", "unknown field control contract");
            std::vector<float> audio;
            if (op == "initialize") {
                keys(request.get(), {"schema", "operation", "m2", "field"}); ql::require(!field, "field already initialized");
                auto m2 = get(request.get(), "m2"), f = get(request.get(), "field");
                auto next = std::make_unique<ql::ContinuousField>(initialize(m2, f));
                gains = own(json_object_get(get(f, "audio_gains"))); basis = own(json_object_get(m2)); field = std::move(next); committed = true;
            } else {
                ql::require(bool(field), "field is not initialized");
                if (op != "read") {
                    ql::require(decimal<std::uint64_t>(get(request.get(), "expected_generation")) == field->receipt().generation &&
                        decimal<std::uint64_t>(get(request.get(), "expected_samples_elapsed")) == field->receipt().samples_elapsed,
                        "stale field generation or physical continuation cursor");
                }
                if (op == "advance") {
                    keys(request.get(), {"schema", "operation", "expected_generation", "expected_samples_elapsed", "frames", "muted"});
                    const bool muted = boolean(get(request.get(), "muted"));
                    audio.resize(small(get(request.get(), "frames"), 8192));
                    // A zero-frame read does not need a non-null vector buffer.
                    if (!audio.empty()) ql::require(field->render_audio(audio.data(), audio.size(), muted), "native continuation refused");
                    committed = !audio.empty();
                } else if (op == "set-axis") {
                    keys(request.get(), {"schema", "operation", "expected_generation", "expected_samples_elapsed", "axis", "phase"});
                    field->set_axis(field->receipt().generation, small(get(request.get(), "axis"), 1), read_phase(get(request.get(), "phase"))); committed = true;
                } else if (op == "replace-modes") {
                    keys(request.get(), {"schema", "operation", "expected_generation", "expected_samples_elapsed", "m2", "replace_state"});
                    auto m2 = get(request.get(), "m2"); auto r = get(m2, "resonator");
                    ql::require(m2_generation(m2) > m2_generation(basis.get()) &&
                        text(get(get(m2, "identity"), "event_ref")) == field->source().event_ref &&
                        text(get(r, "geometry_ref")) == field->source().geometry_ref &&
                        text(get(r, "material_ref")) == field->source().material_ref &&
                        text(get(r, "material_model_ref")) == field->source().model_ref, "changed identity/material or unrelated M2 generation");
                    field->replace_modes(field->receipt().generation, modes(m2, gains.get()), boolean(get(request.get(), "replace_state")));
                    basis = own(json_object_get(m2)); committed = true;
                } else { keys(request.get(), {"schema", "operation"}); ql::require(op == "read", "unknown field operation"); }
            }
            auto output = own(response(*field, audio, true, 1));
            put(output.get(), "m2_identity", json_object_get(get(basis.get(), "identity")));
            std::cout << json_object_to_json_string_ext(output.get(), JSON_C_TO_STRING_PLAIN) << '\n' << std::flush;
        } catch (const std::exception &error) {
            auto output = own(json_object_new_object()); string(output.get(), "schema", "ql.field-error/v1"); string(output.get(), "error", error.what());
            put(output.get(), "state_committed", json_object_new_boolean(committed));
            std::cout << json_object_to_json_string_ext(output.get(), JSON_C_TO_STRING_PLAIN) << '\n' << std::flush;
        }
    }
}
