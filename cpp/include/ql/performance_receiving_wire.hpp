#ifndef QL_PERFORMANCE_RECEIVING_WIRE_HPP
#define QL_PERFORMANCE_RECEIVING_WIRE_HPP
// Control serialization of the immutable callback-owned receiving observation.
// This operation neither samples the mutable body nor grants source/context.
#include <ql/performance_checkpoint_wire.hpp>
namespace ql::performance::receiving_transport {
inline checkpoint_transport::Json readback(const NativeReceivingReadback &r) {
  namespace wire = checkpoint_transport;
  const auto &m = r.manifest;
  require(valid_receiving_manifest(m) && r.samples_elapsed >= m.origin_sample &&
              r.samples_elapsed <= m.end_sample,
          "native receiving observation cursor differs");
  const auto &p = r.end_position;
  for (const auto *v :
       {&p.source_emission_position_metres, &p.receiver_position_metres})
    for (double value : *v)
      require(std::isfinite(value) && std::abs(value) <= 1e6,
              "native receiving metric observation differs");
  require(std::isfinite(p.distance_metres) && p.distance_metres >= 0 &&
              std::isfinite(p.propagation_delay_samples) &&
              p.propagation_delay_samples >= 0 &&
              p.propagation_delay_samples <=
                  ql::receiving_history_samples - 2 &&
              std::isfinite(p.gain_linear) && p.gain_linear >= 0 &&
              p.gain_linear <= 1 &&
              std::isfinite(p.emission_samples_per_received_sample) &&
              p.emission_samples_per_received_sample >= .975 / 1.025 &&
              p.emission_samples_per_received_sample <= 1.025 / .975,
          "native receiving numerical observation differs");
  auto out = wire::object();
  wire::text(out.get(), "schema",
             r.explicit_source_history ? "ql.native-receiving-readback/v2"
                                       : "ql.native-receiving-readback/v1");
  wire::put(out.get(), "manifest", wire::receiving_manifest(m).release());
  wire::u64(out.get(), "samples_elapsed", r.samples_elapsed);
  wire::text(out.get(), "distance_unit", "m");
  wire::text(out.get(), "cursor_unit", "native-audio-sample");
  wire::text(out.get(), "signal_unit", "linear-pickup");
  auto source = wire::array(), receiver = wire::array();
  for (double v : p.source_emission_position_metres)
    wire::append(source.get(), json_object_new_double(v));
  for (double v : p.receiver_position_metres)
    wire::append(receiver.get(), json_object_new_double(v));
  wire::put(out.get(), "source_emission_position_metres", source.release());
  wire::put(out.get(), "receiver_position_metres", receiver.release());
  wire::real(out.get(), "distance_metres", p.distance_metres);
  wire::real(out.get(), "propagation_delay_samples",
             p.propagation_delay_samples);
  wire::real(out.get(), "gain_linear", p.gain_linear);
  wire::real(out.get(), "emission_samples_per_received_sample",
             p.emission_samples_per_received_sample);
  if (r.explicit_source_history) {
    require(r.contributing_source_segments <= ql::receiving_source_segments &&
                r.emitting_body_revision &&
                r.emitting_effective_sample <= r.samples_elapsed &&
                r.emitting_preparation[0] && r.emitting_source_revision[0],
            "historical emitting source observation refused");
    wire::put(out.get(), "contributing_source_segments",
              json_object_new_uint64(r.contributing_source_segments));
    wire::u64(out.get(), "emitting_body_revision", r.emitting_body_revision);
    wire::u64(out.get(), "emitting_effective_sample",
              r.emitting_effective_sample);
    wire::text(out.get(), "emitting_preparation",
               r.emitting_preparation.data());
    wire::text(out.get(), "emitting_source_revision",
               r.emitting_source_revision.data());
  }
  return out;
}
} // namespace ql::performance::receiving_transport
#endif
