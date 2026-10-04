#ifndef QL_PERFORMANCE_PHYSICAL_ROUTES_HPP
#define QL_PERFORMANCE_PHYSICAL_ROUTES_HPP
// Control-prepared adapter to ONE resident physical numerical owner.
// A supplies its independent programme Newton arrays and remains their phase,
// scheduling and checkpoint owner. P only projects/sums/advances q/v once.
#include <ql/performance_physical.hpp>
#include <ql/physical_force_route_port.hpp>
#include <ql/physical_force_routes_wire.hpp>
#include <ql/performance_receiving_admission.hpp>
namespace ql::performance {
class PhysicalRoutesPortBinding {
  std::shared_ptr<ql::PhysicalBody> body_;
  std::shared_ptr<const AdmittedNativeReceivingSource> source_admission_;
  std::shared_ptr<const ql::physical_wire::PreparedNativePhysicalForceRoutes>
      qualified_;
  ql::PhysicalForceRoutePortManifest manifest_{};
  ql::PhysicalForceRouteReceipt last_receipt_{};
  bool has_receipt_ = false;

  static ql::PhysicalForcePortRef bounded(const std::string &source) {
    return reference(source.c_str());
  }
  static std::uint64_t decimal(const std::string &source) {
    if (source.empty() || source.size() > 20 ||
        (source.size() > 1 && source.front() == '0'))
      throw std::invalid_argument("canonical original share required");
    std::uint64_t value = 0;
    for (unsigned char c : source) {
      if (c < '0' || c > '9' ||
          value > (std::numeric_limits<std::uint64_t>::max() - (c - '0')) / 10)
        throw std::invalid_argument("original share overflow");
      value = value * 10 + (c - '0');
    }
    return value;
  }
  static std::uint64_t program_seal(
      std::uint64_t route_seal,
      const ql::physical_wire::PhysicalNativeForceProgram &program) noexcept {
    std::uint64_t result = 14695981039346656037ULL;
    const auto bytes = [&](const void *data, std::size_t count) {
      const auto *p = static_cast<const unsigned char *>(data);
      for (std::size_t i = 0; i < count; ++i)
        result = (result ^ p[i]) * 1099511628211ULL;
    };
    // A compatibility witness, never source or consent authority. Native
    // source decoding/qualification precedes this constructor on control.
    bytes(&route_seal, sizeof(route_seal));
    bytes(&program.source_hertz, sizeof(double));
    bytes(&program.original_denominator_share, sizeof(double));
    bytes(&program.peak_force_newtons, sizeof(double));
    return result;
  }

public:
  // All admission arguments are retained IMMUTABLE native outputs. Never read
  // a running body's mutable preparation/cursor from this control operation.
  // R/N independently replay the complete source against the current native
  // owner. The private typed admission must match the separately retained full
  // basis before this control-side port is constructed or swapped. Every full
  // M3 snapshot change requires fresh admission; callback never parses JSON.
  PhysicalRoutesPortBinding(
      std::shared_ptr<ql::PhysicalBody> body,
      std::shared_ptr<const AdmittedNativeReceivingSource> source_admission,
      const ql::PreparedPhysicalBody &immutable_preparation,
      const Determination &current, json_object *actual_serial_owner_basis,
      bool admitted_m1_pratibimba,
      std::uint64_t native_admitted_cursor)
      : body_(std::move(body)),
        source_admission_(std::move(source_admission)),
        qualified_(source_admission_ ? source_admission_->routes() : nullptr) {
    if (!body_ || !qualified_ ||
        !source_admission_->matches_current(current, actual_serial_owner_basis))
      throw std::invalid_argument("same resident body and qualified routes required");
    const auto &routes = qualified_->routes;
    const auto &source = routes.source_basis();
    if (!routes.preflight(immutable_preparation, source, native_admitted_cursor) ||
        native_admitted_cursor != routes.admitted_cursor() ||
        reference(source.event_ref.c_str()) != current.identity.event ||
        reference(source.subject_ref.c_str()) != current.identity.subject ||
        source.m1_revision != current.identity.m1_revision ||
        source.m2_generation != current.identity.m2_generation ||
        current.m1_face != unsigned(admitted_m1_pratibimba) ||
        current.m2_face != 1 || !source.m2_pratibimba ||
        reference(source.m2_writer_coordinate.c_str()) != current.m2_writer ||
        std::strcmp(current.m2_writer.data(), "#2-1") != 0 ||
        reference(source.determination_ref.c_str()) != current.native_receipt_ref ||
        !ql_m_live_accepts_base(current.registry_revision.data()) ||
        reference(source.source_revision.c_str()) != current.source_revision ||
        reference(routes.preparation_ref().c_str()) != current.body_preparation_ref ||
        reference(routes.state_ref().c_str()) != current.body_state_ref ||
        current.body_revision != routes.body_revision() || current.tick12 >= 12 ||
        current.degree720 >= 720 ||
        (routes.route_count() != 0 && routes.route_count() != 9))
      throw std::invalid_argument("route port source/body/determination disconnected");
    const auto *m1 = ql_m_live_resolve(current.m1_coordinate.data());
    if (!m1 || m1->root_position != 1 ||
        std::strcmp(m1->source_ref, current.m1_coordinate.data()) != 0)
      throw std::invalid_argument("route port requires exact native M1 branch");
    auto &out = manifest_;
    out.sample_rate = routes.sample_rate();
    out.max_force_newtons = immutable_preparation.input().max_force_newtons;
    out.route_count = routes.route_count();
    out.source_basis_seal = routes.source_basis_seal();
    out.body_revision = routes.body_revision();
    out.admitted_cursor = routes.admitted_cursor();
    out.m1_revision = source.m1_revision;
    out.m2_generation = source.m2_generation;
    out.m3_generation = source.m3_generation;
    out.m3_input_generation = qualified_->m3_input_generation;
    out.earth_frame_node_id = routes.earth_frame_node_id();
    out.event_ref = bounded(source.event_ref);
    out.subject_ref = bounded(source.subject_ref);
    out.registry_revision = bounded(source.registry_revision);
    out.source_revision = bounded(source.source_revision);
    out.definition_ref = bounded(source.definition_ref);
    out.source_instance_ref = bounded(source.source_instance_ref);
    out.determination_ref = bounded(source.determination_ref);
    out.preparation_ref = bounded(routes.preparation_ref());
    out.state_ref = bounded(routes.state_ref());
    out.eigenbasis_identity = bounded(routes.eigenbasis_identity());
    out.m1_coordinate = current.m1_coordinate;
    out.m1_pratibimba = admitted_m1_pratibimba;
    out.m2_writer_coordinate = current.m2_writer;
    out.m2_pratibimba = true;
    out.tick12 = current.tick12;
    out.degree720 = current.degree720;
    out.temporal_phase = current.tick12 / 6;
    out.native_basis_sha256 = bounded(qualified_->native_basis_sha256);
    out.m3_state_sha256 = bounded(qualified_->m3_state_sha256);
    // Both World and personal receiving retain genuine M1 note excitation.
    // Only a duplicated legacy native-source scalar stays disabled. The nine
    // independent N programmes retain their own source Hz/weighted Newtons.
    out.scalar_note_enabled = true;
    out.scalar_note_gain = 1;
    out.legacy_native_scalar_enabled = false;
    out.legacy_native_scalar_gain = 0;
    for (std::size_t i = 0; i < out.route_count; ++i) {
      const auto &input = routes.route_input(i);
      const auto &program = qualified_->programs[i];
      if (!std::isfinite(program.source_hertz) || program.source_hertz < 1 ||
          program.source_hertz >= .45 * out.sample_rate ||
          !std::isfinite(program.original_denominator_share) ||
          program.original_denominator_share < 0 || program.original_denominator_share > 1 ||
          program.peak_force_newtons != input.peak_force_newtons)
        throw std::invalid_argument("source programme detached from qualified route");
      auto &handle = out.programs[i];
      handle.driver_ref = bounded(input.driver_ref);
      handle.target_ref = bounded(input.target_ref);
      handle.program_ref = bounded(input.program_ref);
      handle.planet_coordinate = bounded(input.planet_coordinate);
      handle.chakra_coordinate = bounded(input.chakra_coordinate);
      handle.projection_ref = bounded(input.projection_ref);
      handle.calibration_ref = bounded(input.calibration_ref);
      handle.calibration_revision = bounded(input.calibration_revision);
      handle.calibration_source_ref = bounded(input.calibration_source_ref);
      handle.calibration_standing = bounded(input.calibration_standing);
      handle.route_index = i;
      handle.preparation_seal = routes.route_seal(i);
      handle.program_seal = program_seal(handle.preparation_seal, program);
      handle.planet_node_id = input.planet_node_id;
      handle.chakra_node_id = input.chakra_node_id;
      handle.native_planet_index = input.native_planet_index;
      handle.centre_ordinal = input.centre_ordinal;
      handle.share_numerator = decimal(input.share_numerator);
      handle.share_denominator = decimal(input.share_denominator);
      if (!handle.share_denominator ||
          std::abs(double(handle.share_numerator) / double(handle.share_denominator) -
                   program.original_denominator_share) > 1e-14)
        throw std::invalid_argument("original denominator share changed");
      handle.source_hertz = program.source_hertz;
      handle.original_denominator_share = program.original_denominator_share;
      handle.peak_force_newtons = program.peak_force_newtons;
    }
  }
  const ql::PhysicalForceRoutePortManifest &manifest() const noexcept {
    return manifest_;
  }
  const std::shared_ptr<ql::PhysicalBody> &body_custody() const noexcept { return body_; }
  const ql::PhysicalBody *numerical_owner() const noexcept { return body_.get(); }
  bool preflight(const ql::PreparedPhysicalBody &immutable_preparation,
                 const PhysicalForceSourceBasis &current_native_source,
                 std::uint64_t native_admitted_cursor) const noexcept {
    return qualified_->routes.preflight(immutable_preparation, current_native_source,
                                        native_admitted_cursor);
  }
  // Uninterrupted sole callback/stopped custody. The existing P9 operation
  // validates exact current body/basis/cursor/seals before any mutation.
  bool advance_routes(const ql::PhysicalScalarForceBlock &scalar,
                      const ql::PhysicalRouteForceBlock *routes,
                      std::size_t count, float *pickup, std::size_t frames,
                      std::uint64_t revision, std::uint64_t start) noexcept {
    ql::PhysicalForceRouteReceipt receipt;
    if (!qualified_->routes.advance_force_block(*body_, scalar, routes, count,
                                                pickup, frames, revision, start, &receipt))
      return false;
    last_receipt_ = receipt;
    has_receipt_ = true;
    return true;
  }
  // Same callback/stopped custody only; UI consumes A's copied readback.
  bool observe_routes(ql::PhysicalForceRouteReceipt &output,
                      std::uint64_t revision, std::uint64_t end) const noexcept {
    if (!has_receipt_ || last_receipt_.body_revision != revision ||
        last_receipt_.end_sample != end ||
        last_receipt_.source_basis_seal != manifest_.source_basis_seal ||
        last_receipt_.route_count != manifest_.route_count ||
        body_->samples_elapsed() != end || body_->body_revision() != revision)
      return false;
    output = last_receipt_;
    return true;
  }
  bool observe_routes(ql::PhysicalForceRouteReceipt &output) const noexcept {
    return observe_routes(output, body_->body_revision(), body_->samples_elapsed());
  }
};
// Serial control owner. Base scalar port was already admitted by A; preserving
// its actual numerical pointer/thunks keeps existing paired checkpoint exact.
inline PhysicalPort physical_routes_port(
    PhysicalPort scalar_port, const std::shared_ptr<PhysicalRoutesPortBinding> &binding) {
  if (!binding || scalar_port.owner != binding->numerical_owner() ||
      scalar_port.custody.get() != binding->numerical_owner() ||
      !scalar_port.advance || !scalar_port.observe || !scalar_port.cursor ||
      !scalar_port.revision)
    throw std::invalid_argument("route port must keep actual scalar numerical owner/custody");
  const auto &manifest = binding->manifest();
  scalar_port.event = manifest.event_ref;
  scalar_port.subject = manifest.subject_ref;
  scalar_port.preparation = manifest.preparation_ref;
  scalar_port.state = manifest.state_ref;
  scalar_port.sample_rate = manifest.sample_rate;
  scalar_port.max_force_newtons = manifest.max_force_newtons;
  scalar_port.routes_owner = binding.get();
  scalar_port.routes_custody = binding;
  scalar_port.route_manifest = &binding->manifest();
  scalar_port.advance_routes = [](
      void *owner, const ql::PhysicalScalarForceBlock &scalar,
      const ql::PhysicalRouteForceBlock *routes, std::size_t count,
      float *pickup, std::size_t frames, std::uint64_t revision,
      std::uint64_t start) noexcept {
    return static_cast<PhysicalRoutesPortBinding *>(owner)->advance_routes(
        scalar, routes, count, pickup, frames, revision, start);
  };
  scalar_port.observe_routes = [](const void *owner,
                                  ql::PhysicalForceRouteReceipt &out) noexcept {
    return static_cast<const PhysicalRoutesPortBinding *>(owner)->observe_routes(out);
  };
  return scalar_port;
}
} // namespace ql::performance
#endif
