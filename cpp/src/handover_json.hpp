// The worker's `handover` answer (O-I #849): a `ql::Continuation` as JSON, `ql.continuous-handover/v1`, for a native audio host to
// continue the field exactly. Included by field_worker.cpp after its JSON helpers (put, string, integer); off the audio path.
// Exact integers travel as canonical decimal strings, doubles at round-trip precision (json-c's %.17g), the clock in the very
// form the worker's `initialize` reads, so the host's continuation is this field's, sample for sample.
#ifndef QL_FIELD_WORKER_HANDOVER_JSON_HPP
#define QL_FIELD_WORKER_HANDOVER_JSON_HPP
inline constexpr const char *handover_contract = "ql.continuous-handover/v1";

J *handover_pair(ql::Complex z) {
    J *a = json_object_new_array();
    json_object_array_add(a, json_object_new_double(z.real()));
    json_object_array_add(a, json_object_new_double(z.imag()));
    return a;
}
J *handover_vec(const ql::Vec3 &v) {
    J *a = json_object_new_array();
    for (double x : v) json_object_array_add(a, json_object_new_double(x));
    return a;
}
J *handover_phase(QL_PhaseLift p) {
    J *o = json_object_new_object();
    string(o, "turns", std::to_string(p.turns));
    put(o, "half_degrees", json_object_new_int(p.half_degrees));
    return o;
}
// The clock as `clock_input` reads it.
J *handover_clock(const QL_CoupledClock &c) {
    J *o = json_object_new_object();
    put(o, "inscription", handover_phase(c.inscription));
    put(o, "lensing", handover_phase(c.lensing));
    J *origins = json_object_new_array(), *rates = json_object_new_array(), *rest = json_object_new_array();
    for (auto n : c.grid_origin_half_degrees) json_object_array_add(origins, json_object_new_int(n));
    for (auto n : c.rate_numerators) json_object_array_add(rates, json_object_new_string(std::to_string(n).c_str()));
    for (auto n : c.rate_remainders) json_object_array_add(rest, json_object_new_string(std::to_string(n).c_str()));
    put(o, "grid_origins", origins);
    put(o, "rate_numerators", rates);
    put(o, "rate_denominator", json_object_new_int64(c.rate_denominator));
    put(o, "rate_remainders", rest);
    integer(o, "generation", c.generation);
    return o;
}
J *handover_output(const ql::Continuation &c) {
    J *o = json_object_new_object();
    const auto &in = c.input;
    string(o, "schema", handover_contract);
    string(o, "event_ref", in.event_ref);
    string(o, "subject_ref", in.subject_ref);
    string(o, "geometry_ref", in.geometry_ref);
    string(o, "material_ref", in.material_ref);
    string(o, "model_ref", in.model_ref);
    integer(o, "generation", in.generation);
    put(o, "sample_rate", json_object_new_int64(in.sample_rate));
    integer(o, "samples_elapsed", c.samples_elapsed);
    integer(o, "driver_remainder", c.driver_remainder);
    put(o, "driver_numerator", json_object_new_int64(in.driver_numerator));
    put(o, "driver_denominator", json_object_new_int64(in.driver_denominator));
    put(o, "last_unmuted_sample", json_object_new_double(c.last_unmuted_sample));
    put(o, "clock", handover_clock(in.clock));
    J *modes = json_object_new_array();
    for (const auto &m : in.modes) {
        J *mode = json_object_new_object();
        string(mode, "mode_ref", m.reference);
        string(mode, "source_coordinate", m.source_coordinate);
        put(mode, "frequency_hz", json_object_new_double(m.frequency_hz));
        put(mode, "damping_per_second", json_object_new_double(m.damping_per_second));
        put(mode, "state_metres", handover_pair(m.amplitude_metres));
        put(mode, "drive_metres_per_second", handover_pair(m.drive_metres_per_second));
        put(mode, "audio_gain_per_metre", json_object_new_double(m.audio_gain_per_metre));
        json_object_array_add(modes, mode);
    }
    put(o, "modes", modes);
    J *samples = json_object_new_array();
    for (const auto &s : in.samples) {
        J *sample = json_object_new_object();
        put(sample, "identity", json_object_new_uint64(s.identity));
        string(sample, "constituent", s.constituent);
        put(sample, "attachment", json_object_new_int(int(s.attachment)));
        put(sample, "rest_metres", handover_vec(s.rest_metres));
        J *shapes = json_object_new_array();
        for (const auto &shape : s.mode_shapes) json_object_array_add(shapes, handover_vec(shape));
        put(sample, "mode_shapes", shapes);
        json_object_array_add(samples, sample);
    }
    put(o, "samples", samples);
    J *carrier = json_object_new_object();
    put(carrier, "steps_per_second", json_object_new_double(c.carrier.steps_per_second));
    integer(carrier, "anchor_samples", c.carrier.anchor_samples);
    put(carrier, "anchor_steps", json_object_new_double(c.carrier.anchor_steps));
    put(carrier, "reached", json_object_new_double(c.carrier.reached));
    J *impulses = json_object_new_array();
    for (auto z : c.carrier.impulses) json_object_array_add(impulses, handover_pair(z));
    put(carrier, "impulses", impulses);
    put(o, "carrier", carrier);
    J *pending = json_object_new_array();
    for (const auto &act : c.pending) {
        J *entry = json_object_new_object();
        integer(entry, "at_samples_elapsed", act.at_samples_elapsed);
        J *struck = json_object_new_array();
        for (const auto &[mode, z] : act.impulses) {
            J *one = json_object_new_object();
            put(one, "mode", json_object_new_uint64(mode));
            put(one, "amplitude", handover_pair(z));
            json_object_array_add(struck, one);
        }
        put(entry, "impulses", struck);
        json_object_array_add(pending, entry);
    }
    put(o, "pending", pending);
    J *heard = json_object_new_array();
    for (double g : c.heard_from) json_object_array_add(heard, json_object_new_double(g));
    put(o, "heard_from", heard);
    return o;
}
#endif
