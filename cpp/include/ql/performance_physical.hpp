#ifndef QL_PERFORMANCE_PHYSICAL_HPP
#define QL_PERFORMANCE_PHYSICAL_HPP
#include <ql/performance_audio.hpp>
#include <ql/physical_body.hpp>
namespace ql::performance {
// Control-thread adapter to the actual P owner. No substitute dynamics.
inline PhysicalPort physical_port(ql::PhysicalBody &body) {
  PhysicalPort port{};
  port.owner = &body;
  port.contact_preparation = &body.preparation();
  port.advance = [](void *owner, const double *force, float *pickup,
                    std::size_t frames, std::uint64_t revision,
                    std::uint64_t start) noexcept {
    return static_cast<ql::PhysicalBody *>(owner)->advance_force_block(
        force, pickup, frames, revision, start);
  };
  port.revision = [](const void *owner) noexcept {
    return static_cast<const ql::PhysicalBody *>(owner)->body_revision();
  };
  port.cursor = [](const void *owner) noexcept {
    return static_cast<const ql::PhysicalBody *>(owner)->samples_elapsed();
  };
  port.observe = [](const void *owner, ql::PhysicalSnapshot &out,
                    std::uint64_t revision, std::uint64_t end) noexcept {
    return ql::write_physical_snapshot(
        *static_cast<const ql::PhysicalBody *>(owner), out, revision, end);
  };
  const auto &input = body.preparation().input();
  port.max_force_newtons = input.max_force_newtons;
  port.sample_rate = input.sample_rate;
  port.event = reference(input.event_ref.c_str());
  port.subject = reference(input.subject_ref.c_str());
  port.preparation = reference(body.preparation_ref().c_str());
  port.state = reference(body.state_ref().c_str());
  return port;
}
inline PhysicalPort
physical_port(const std::shared_ptr<ql::PhysicalBody> &body) {
  if (!body)
    throw std::invalid_argument("resident physical body required");
  auto port = physical_port(*body);
  port.custody = body;
  return port;
}
} // namespace ql::performance
#endif
