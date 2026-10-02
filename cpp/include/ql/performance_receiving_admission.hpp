#ifndef QL_PERFORMANCE_RECEIVING_ADMISSION_HPP
#define QL_PERFORMANCE_RECEIVING_ADMISSION_HPP
// Native control only. The current bundle is independently replayed by the
// Rust lease/context owner. Candidate JSON, matching hashes and cause text do
// not provide authority. No parsing or body construction runs in the callback.
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/physical_force_routes_wire.hpp>
namespace ql::performance {
class AdmittedNativeReceivingSource {
  std::shared_ptr<const ql::physical_wire::PreparedNativePhysicalForceRoutes>
      routes_;
  Determination source_{};
  ql::physical_wire::Json native_basis_;
  friend AdmittedNativeReceivingSource read_native_receiving_admission(
      json_object *, json_object *, const NativePerformance &, json_object *,
      const ql::PreparedPhysicalBody &, const std::vector<std::string> &,
      std::uint64_t);
  AdmittedNativeReceivingSource(
      std::shared_ptr<
          const ql::physical_wire::PreparedNativePhysicalForceRoutes>
          routes,
      const Determination &source, ql::physical_wire::Json native_basis)
      : routes_(std::move(routes)), source_(source),
        native_basis_(std::move(native_basis)) {}

public:
  const auto &routes() const noexcept { return routes_; }
  const Determination &determination() const noexcept { return source_; }
  // Control-only full field comparison, including actual branch/clock/face,
  // relation/tuning policy and live M2 buses. No numerical seal substitutes it.
  bool matches_current(const Determination &candidate,
                       json_object *actual_serial_owner_basis) const {
    if (!actual_serial_owner_basis ||
        !json_object_equal(native_basis_.get(), actual_serial_owner_basis))
      return false;
    const auto current = checkpoint_transport::determination(source_);
    const auto other = checkpoint_transport::determination(candidate);
    return json_object_equal(current.get(), other.get());
  }
};
inline AdmittedNativeReceivingSource read_native_receiving_admission(
    json_object *candidate, json_object *independently_current,
    const NativePerformance &resident_native,
    json_object *actual_resident_native_basis,
    const ql::PreparedPhysicalBody &immutable_preparation,
    const std::vector<std::string> &native_program_refs,
    std::uint64_t native_admitted_cursor) {
  using namespace ql::physical_wire;
  keys(candidate, {"schema", "native_basis", "native_preparation",
                   "native_generations", "receiving_definition", "operation"});
  require(independently_current &&
              json_object_equal(candidate, independently_current),
          "receiving bundle differs from complete independently-current native "
          "source replay");
  same_text(field(candidate, "schema"),
            "ql.performance-receiving-admission/v1");
  auto *basis = field(independently_current, "native_basis");
  auto *preparation = field(independently_current, "native_preparation");
  require(actual_resident_native_basis &&
              json_object_equal(basis, actual_resident_native_basis),
          "receiving basis differs from separately retained actual resident "
          "native source");
  require(json_object_equal(field(preparation, "native_basis"), basis),
          "receiving performance and actual native basis disconnected");
  auto serialized =
      checkpoint_transport::determination(resident_native.determination);
  require(resident_native.body && resident_native.engine &&
              json_object_equal(serialized.get(),
                                field(preparation, "determination")),
          "receiving source differs from resident native audio determination");
  const auto &d = resident_native.determination;
  auto *m1 = field(basis, "m1");
  auto *config = field(m1, "config");
  auto *clock = field(m1, "clock");
  const auto *source = ql_m_live_resolve(
      packet::string(field(config, "selected_coordinate")).c_str());
  require(source && source->root_position == 1 &&
              reference(source->source_ref) == d.m1_coordinate &&
              packet::integer(field(config, "revision")) ==
                  d.identity.m1_revision &&
              packet::ref(config, "event_ref") == d.identity.event &&
              packet::integer(field(clock, "tick12")) == d.tick12 &&
              packet::integer(field(clock, "degree720")) == d.degree720 &&
              packet::integer(field(clock, "phase")) == d.tick12 / 6,
          "receiving port actual M1 branch/clock/face differs from resident "
          "producer");
  auto *operation = field(candidate, "operation");
  auto *definition = field(candidate, "receiving_definition");
  require(json_object_equal(field(candidate, "native_generations"),
                            field(operation, "native_generations")) &&
              json_object_equal(field(definition, "native_generations"),
                                field(operation, "native_generations")) &&
              text(field(definition, "content_digest")) ==
                  text(field(operation, "definition_digest")),
          "receiving definition and original native generations disconnected");
  PhysicalForceSourceBasis admitted;
  admitted.event_ref = d.identity.event.data();
  admitted.subject_ref = d.identity.subject.data();
  admitted.registry_revision = d.registry_revision.data();
  admitted.source_revision = d.source_revision.data();
  admitted.definition_ref = text(field(operation, "definition_digest"));
  admitted.source_instance_ref = text(field(operation, "source_instance"));
  admitted.determination_ref = d.native_receipt_ref.data();
  admitted.m2_writer_coordinate = d.m2_writer.data();
  admitted.m2_pratibimba = d.m2_face == 1;
  admitted.m1_revision = d.identity.m1_revision;
  admitted.m2_generation = d.identity.m2_generation;
  admitted.m3_generation = immutable_preparation.input().source_generation;
  auto qualified = read_prepared_physical_force_routes(
      operation, field(independently_current, "operation"), admitted,
      immutable_preparation, native_program_refs, native_admitted_cursor);
  return AdmittedNativeReceivingSource(
      std::make_shared<const PreparedNativePhysicalForceRoutes>(
          std::move(qualified)),
      d,
      parse_native(json_object_to_json_string_ext(actual_resident_native_basis,
                                                  JSON_C_TO_STRING_PLAIN)));
}
} // namespace ql::performance
#endif
