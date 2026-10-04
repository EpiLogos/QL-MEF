#ifndef QL_PERFORMANCE_SCENE_CONTACT_OWNER_HPP
#define QL_PERFORMANCE_SCENE_CONTACT_OWNER_HPP
#include <ql/performance_contact_slots.hpp>
#include <ql/physical_scene_contact.hpp>

namespace ql::performance::scene_contact_transport {
class Channel;
}
namespace ql::performance {
// The real worker's dedicated native Scene channel is the sole constructor.
// That pipe is owned by the actual QL Manager/FieldHost; its Rust ingress must
// first hold the C current-Document reader, live S SceneOwner, complete native
// source and original occurrence history. These are not JSON permissions.
// The constructor below checks the actual native physical producer again and
// retains its exact immutable programme until native queue admission returns.
class NativeSceneContactOccurrenceOwner {
  friend class scene_contact_transport::Channel;
  NativeContactOccurrence occurrence_{};
  ql::PreparedPhysicalSceneContact contact_;
  std::shared_ptr<const PreparedContactProgram> programme_;
  NativeSceneContactOccurrenceOwner(
      NativeContactOccurrence occurrence, const ql::PreparedPhysicalBody &body,
      std::uint64_t actual_native_cursor,
      const ql::PhysicalSceneContactDefinition &definition)
      : occurrence_(occurrence), contact_(ql::prepare_physical_scene_contact(
                                     body, actual_native_cursor, definition)),
        programme_(std::make_shared<const PreparedContactProgram>(
            body, actual_native_cursor, contact_.native_input)) {
    ql::require(occurrence_.original_request_id != 0 &&
                    occurrence_.constructor_lineage[0] != 0 &&
                    std::find(occurrence_.constructor_lineage.begin(),
                              occurrence_.constructor_lineage.end(), char(0)) !=
                        occurrence_.constructor_lineage.end() &&
                    programme_->force().force_newtons ==
                        contact_.force.force_newtons,
                "native Scene contact lost its original occurrence/solver");
  }
  NativeContactOccurrenceWitness witness() const {
    return NativeContactOccurrenceWitness(occurrence_, programme_);
  }

public:
  NativeSceneContactOccurrenceOwner(const NativeSceneContactOccurrenceOwner &) =
      delete;
  NativeSceneContactOccurrenceOwner &
  operator=(const NativeSceneContactOccurrenceOwner &) = delete;
};
} // namespace ql::performance
#endif
