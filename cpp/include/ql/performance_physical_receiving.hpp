#ifndef QL_PERFORMANCE_PHYSICAL_RECEIVING_HPP
#define QL_PERFORMANCE_PHYSICAL_RECEIVING_HPP
#include <ql/performance_receiving_port.hpp>
namespace ql::performance {
class MovingReceivingPortBinding {
  std::shared_ptr<ql::PhysicalBody> body_;
  ql::MovingSpatialReceiving receiving_;
  NativeReceivingManifest manifest_{};
  static ReceivingRef ref(const std::string &source) {
    ql::reference(source);
    ql::require(source.size() < ReceivingRef{}.size(),
                "receiving reference exceeds fixed native bound");
    ReceivingRef out{};
    std::copy(source.begin(), source.end(), out.begin());
    return out;
  }

public:
  // Existing serial owner supplies its immutable qualified preparation and
  // actual admitted cursor. No control-thread live q/v/cursor read occurs.
  MovingReceivingPortBinding(std::shared_ptr<ql::PhysicalBody> body,
                             const ql::PreparedPhysicalBody &immutable,
                             std::uint64_t admitted_cursor,
                             ql::PreparedMovingSpatialReceiving prepared)
      : body_(std::move(body)),
        receiving_(immutable, admitted_cursor, std::move(prepared)) {
    ql::require(bool(body_), "receiving lost resident physical owner");
    const auto &in = immutable.input();
    const auto &motion = receiving_.preparation().motion();
    const auto &spatial = receiving_.preparation().spatial().input();
    manifest_.receiving_identity = ref(receiving_.preparation().identity());
    manifest_.event = ref(in.event_ref);
    manifest_.subject = ref(in.subject_ref);
    manifest_.preparation = ref(in.preparation_ref);
    manifest_.state = ref(in.state_ref);
    manifest_.source_coordinate = ref(in.source_coordinate);
    manifest_.source_revision = ref(in.source_revision);
    manifest_.eigenbasis = ref(immutable.eigenbasis_identity());
    manifest_.receiver = ref(spatial.receiver_ref);
    manifest_.context = ref(spatial.context_ref);
    manifest_.source_motion = ref(motion.source_motion_ref);
    manifest_.receiver_motion = ref(motion.receiver_motion_ref);
    manifest_.policy = ref(motion.policy_ref);
    manifest_.policy_revision = ref(motion.policy_revision);
    manifest_.standing = ref(motion.standing);
    manifest_.source_generation = in.source_generation;
    manifest_.body_revision = in.body_revision;
    manifest_.sample_rate = in.sample_rate;
    manifest_.pratibimba = in.pratibimba;
    manifest_.history_origin_sample = admitted_cursor;
    manifest_.origin_sample = motion.origin_sample;
    manifest_.end_sample = motion.end_sample;
    ql::require(valid_receiving_manifest(manifest_),
                "prepared native receiving manifest refused");
  }
  const NativeReceivingManifest &manifest() const noexcept { return manifest_; }
  ReceivingPort
  port(const std::shared_ptr<MovingReceivingPortBinding> &custody) {
    ql::require(custody.get() == this,
                "receiving requires retained exact native numerical owner");
    ReceivingPort out{};
    out.owner = this;
    out.body_owner = body_.get();
    out.manifest = &manifest_;
    out.custody = custody;
    out.preflight = [](const void *owner, std::size_t frames,
                       std::uint64_t cursor) noexcept {
      const auto &self =
          *static_cast<const MovingReceivingPortBinding *>(owner);
      return self.receiving_.preflight_before_body(*self.body_, frames, cursor);
    };
    out.process = [](void *owner, const float *pickup, float *received,
                     std::size_t frames, std::uint64_t cursor) noexcept {
      auto &self = *static_cast<MovingReceivingPortBinding *>(owner);
      return self.receiving_.process_pickup_block(*self.body_, pickup, received,
                                                  frames, cursor);
    };
    out.cursor = [](const void *owner) noexcept {
      return static_cast<const MovingReceivingPortBinding *>(owner)
          ->receiving_.samples_elapsed();
    };
    out.observe = [](const void *owner, NativeReceivingReadback &output,
                     std::uint64_t cursor) noexcept {
      const auto &self =
          *static_cast<const MovingReceivingPortBinding *>(owner);
      if (self.receiving_.samples_elapsed() != cursor ||
          self.body_->samples_elapsed() != cursor ||
          !self.receiving_.preparation().matches_preparation(
              self.body_->preparation()))
        return false;
      ql::SpatialMotionPoint point{};
      if (!self.receiving_.preparation().point_at(cursor, point))
        return false;
      output.manifest = self.manifest_;
      output.samples_elapsed = cursor;
      output.end_position = point;
      return true;
    };
    out.write_checkpoint = [](const void *owner,
                              NativeReceivingCheckpoint &output,
                              std::uint64_t cursor) noexcept {
      const auto &self =
          *static_cast<const MovingReceivingPortBinding *>(owner);
      if (self.receiving_.elapsed_ != cursor ||
          self.body_->samples_elapsed() != cursor ||
          !self.receiving_.preparation().matches_preparation(
              self.body_->preparation()))
        return false;
      output.version = 1;
      output.manifest = self.manifest_;
      output.samples_elapsed = cursor;
      output.history_start_sample = self.receiving_.history_start_;
      output.history_linear = self.receiving_.history_;
      return valid_receiving_checkpoint(output, self.manifest_);
    };
    out.validate_checkpoint = [](const void *owner,
                                 const NativeReceivingCheckpoint &saved,
                                 std::uint64_t expected_cursor) noexcept {
      const auto &self =
          *static_cast<const MovingReceivingPortBinding *>(owner);
      return self.receiving_.elapsed_ == expected_cursor &&
             valid_receiving_checkpoint(saved, self.manifest_);
    };
    out.restore_checkpoint =
        [](void *owner, const NativeReceivingCheckpoint &saved) noexcept {
          auto &self = *static_cast<MovingReceivingPortBinding *>(owner);
          self.receiving_.history_ = saved.history_linear;
          self.receiving_.history_start_ = saved.history_start_sample;
          self.receiving_.elapsed_ = saved.samples_elapsed;
        };
    return out;
  }
};
} // namespace ql::performance
#endif
