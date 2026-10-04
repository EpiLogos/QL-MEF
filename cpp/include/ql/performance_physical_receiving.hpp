#ifndef QL_PERFORMANCE_PHYSICAL_RECEIVING_HPP
#define QL_PERFORMANCE_PHYSICAL_RECEIVING_HPP
#include <ql/performance_receiving_port.hpp>
#include <ql/physical_transition.hpp>
namespace ql::performance {
class MovingReceivingPortBinding {
  ql::NativeResidentLifetime resident_lifetime_{};
  std::shared_ptr<ql::PhysicalBody> body_;
  ql::PreparedPhysicalBody immutable_body_;
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

  struct SavedPreparationTag {};
  // Immutable preparation plus actual admitted saved cursor; no live P query.
  MovingReceivingPortBinding(std::shared_ptr<ql::PhysicalBody> body,
                             const ql::PreparedPhysicalBody &immutable,
                             std::uint64_t admitted_cursor,
                             ql::PreparedMovingSpatialReceiving prepared,
                             std::uint64_t history_birth, SavedPreparationTag)
      : body_(std::move(body)), immutable_body_(immutable),
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
    manifest_.history_origin_sample = history_birth;
    receiving_.history_start_ = history_birth;
    manifest_.origin_sample = motion.origin_sample;
    manifest_.end_sample = motion.end_sample;
    ql::require(valid_receiving_manifest(manifest_),
                "prepared native receiving manifest refused");
  }

public:
  // Existing first-install construction keeps its original exact birth.
  MovingReceivingPortBinding(std::shared_ptr<ql::PhysicalBody> body,
                             const ql::PreparedPhysicalBody &immutable,
                             std::uint64_t admitted_cursor,
                             ql::PreparedMovingSpatialReceiving prepared)
      : MovingReceivingPortBinding(std::move(body), immutable, admitted_cursor,
                                   std::move(prepared), admitted_cursor,
                                   SavedPreparationTag{}) {}
  // Pure numerical candidate for a fresh resident. The SAME private source
  // owner independently rebuilds the ORIGINAL saved segment preparation;
  // segment origin/end and history birth remain exact. No install/publication
  // happens here, and a matching CP never grants Scene/Act/source authority.
  static std::shared_ptr<MovingReceivingPortBinding>
  from_saved_preparation(std::shared_ptr<ql::PhysicalBody> body,
                         const ql::PreparedPhysicalBody &immutable,
                         std::uint64_t saved_cursor,
                         ql::PreparedMovingSpatialReceiving original,
                         std::uint64_t original_history_birth,
                         const NativeReceivingCheckpoint &saved) {
    ql::require(saved.samples_elapsed == saved_cursor &&
                    saved.history_start_sample == original_history_birth &&
                    original_history_birth <= original.motion().origin_sample &&
                    valid_receiving_checkpoint(saved, saved.manifest),
                "saved receiver candidate has no exact valid original history");
    auto candidate = std::shared_ptr<MovingReceivingPortBinding>(
        new MovingReceivingPortBinding(
            std::move(body), immutable, saved_cursor, std::move(original),
            original_history_birth, SavedPreparationTag{}));
    ql::require(same_receiving_manifest(candidate->manifest_, saved.manifest),
                "saved receiving source/trajectory/body/manifest differs");
    candidate->receiving_.history_ = saved.history_linear;
    candidate->receiving_.restore_history_peak();
    if (saved.version == 2) {
      ql::require(
          ql::current_receiving_source_matches(
              saved.source_history, immutable,
              candidate->receiving_.preparation()),
          "saved receiving historical tail differs from actual current source");
      candidate->receiving_.source_history_ = saved.source_history;
    } else if (candidate->receiving_.preparation().motion().origin_sample <=
               original_history_birth) {
      // Exact original single-segment legacy source is known from the admitted
      // current typed preparation, so record its actual original birth.
      candidate->receiving_.source_history_ = {};
      ql::require(ql::append_receiving_source(
                      candidate->receiving_.source_history_, immutable,
                      candidate->receiving_.preparation(),
                      original_history_birth, original_history_birth),
                  "saved receiving lost actual original single emitter");
    }
    ql::SpatialMotionPoint point{};
    unsigned contributions = 0;
    ql::require(candidate->receiving_.historical_point(saved_cursor, point,
                                                       contributions),
                "saved receiving historical source trajectory is invalid");
    return candidate;
  }
  // Pure numerical AFTER candidate for the original P owner. The actual
  // private source/occasion owner must independently qualify the AFTER body
  // and receiving operands. This function transfers the full genuine BEFORE
  // delay ring; it neither grants source authority nor commits a body change.
  // The original motion segment/date and history birth remain intact.
  static std::shared_ptr<MovingReceivingPortBinding>
  from_prepared_body_transition(std::shared_ptr<ql::PhysicalBody> body,
                                ql::PreparedPhysicalTransition &transition,
                                ql::PreparedMovingSpatialReceiving after,
                                const NativeReceivingCheckpoint &retained) {
    ql::require(
        body && !transition.committed() && transition.preflight(*body) &&
            retained.samples_elapsed == transition.sample() &&
            valid_receiving_checkpoint(retained, retained.manifest),
        "body receiving continuation requires exact stopped BEFORE custody");
    const auto &before = body->preparation();
    const auto &in = before.input();
    const auto &m = retained.manifest;
    ql::require(
        m.event == ref(in.event_ref) && m.subject == ref(in.subject_ref) &&
            m.preparation == ref(in.preparation_ref) &&
            m.state == ref(in.state_ref) &&
            m.source_coordinate == ref(in.source_coordinate) &&
            m.source_revision == ref(in.source_revision) &&
            m.eigenbasis == ref(before.eigenbasis_identity()) &&
            m.source_generation == in.source_generation &&
            m.body_revision == in.body_revision &&
            m.sample_rate == in.sample_rate && m.pratibimba == in.pratibimba,
        "body receiving continuation lost the original full P preparation");
    const auto &prepared = transition.pending_after_preparation();
    const auto &spatial = after.spatial().input();
    ql::require(
        after.matches_preparation(prepared) &&
            ref(spatial.context_ref) == m.context &&
            ref(spatial.receiver_ref) == m.receiver &&
            retained.history_start_sample <= after.motion().origin_sample &&
            after.motion().origin_sample <= retained.samples_elapsed &&
            retained.samples_elapsed < after.motion().end_sample,
        "body receiving continuation lost context/receiver/original history");
    auto candidate = std::shared_ptr<MovingReceivingPortBinding>(
        new MovingReceivingPortBinding(
            std::move(body), prepared, retained.samples_elapsed,
            std::move(after), retained.history_start_sample,
            SavedPreparationTag{}));
    candidate->receiving_.history_ = retained.history_linear;
    candidate->receiving_.restore_history_peak();
    if (retained.version == 2 || retained.source_history.count) {
      candidate->receiving_.source_history_ = retained.source_history;
    } else {
      // Legacy single-segment source is reconstructed only from the actual
      // BEFORE preparation plus the preserved full current trajectory.
      candidate->receiving_.source_history_ = {};
      ql::PreparedMovingSpatialReceiving original(
          before, candidate->receiving_.preparation().spatial().input(),
          candidate->receiving_.preparation().motion());
      ql::require(
          ref(original.identity()) == m.receiving_identity &&
              ql::append_receiving_source(
                  candidate->receiving_.source_history_, before, original,
                  retained.history_start_sample, retained.history_start_sample),
          "legacy receiving lacks the original historical source trajectory");
    }
    ql::require(ql::append_receiving_source(
                    candidate->receiving_.source_history_, prepared,
                    candidate->receiving_.preparation(),
                    retained.samples_elapsed, retained.history_start_sample),
                "body receiving dated source history exceeded or lost original "
                "provenance");
    ql::SpatialMotionPoint point{};
    unsigned contributions = 0;
    ql::require(candidate->receiving_.historical_point(retained.samples_elapsed,
                                                       point, contributions),
                "body receiving historical source point refused");
    return candidate;
  }
  // Pure control construction from an actual stopped receiver checkpoint.
  // No equality here grants source/context: Engine/Management recheck this
  // exact ring against their actual retained owner before the port swap.
  MovingReceivingPortBinding(std::shared_ptr<ql::PhysicalBody> body,
                             const ql::PreparedPhysicalBody &immutable,
                             std::uint64_t admitted_cursor,
                             ql::PreparedMovingSpatialReceiving prepared,
                             const NativeReceivingCheckpoint &retained)
      : MovingReceivingPortBinding(std::move(body), immutable, admitted_cursor,
                                   std::move(prepared)) {
    const auto &m = retained.manifest;
    ql::require(
        valid_receiving_checkpoint(retained, m) &&
            retained.samples_elapsed == admitted_cursor &&
            manifest_.origin_sample == admitted_cursor &&
            m.event == manifest_.event && m.subject == manifest_.subject &&
            m.preparation == manifest_.preparation &&
            m.state == manifest_.state &&
            m.source_coordinate == manifest_.source_coordinate &&
            m.source_revision == manifest_.source_revision &&
            m.eigenbasis == manifest_.eigenbasis &&
            m.source_generation == manifest_.source_generation &&
            m.body_revision == manifest_.body_revision &&
            m.sample_rate == manifest_.sample_rate &&
            m.pratibimba == manifest_.pratibimba,
        "receiving continuation lost original same-body native history");
    manifest_.history_origin_sample = retained.history_start_sample;
    receiving_.history_start_ = retained.history_start_sample;
    receiving_.history_ = retained.history_linear;
    receiving_.restore_history_peak();
    ql::require(retained.version == 2 || retained.source_history.count,
                "receiving act lacks actual original emitter history");
    receiving_.source_history_ = retained.source_history;
    ql::require(
        ql::append_receiving_source(receiving_.source_history_, immutable_body_,
                                    receiving_.preparation(), admitted_cursor,
                                    receiving_.history_start_),
        "receiving act lost original dated emitter history");
    ql::require(valid_receiving_manifest(manifest_),
                "receiving continuation manifest refused");
  }
  const NativeReceivingManifest &manifest() const noexcept { return manifest_; }
  ReceivingPort
  port(const std::shared_ptr<MovingReceivingPortBinding> &custody) {
    ql::require(custody.get() == this,
                "receiving requires retained exact native numerical owner");
    ReceivingPort out{};
    out.resident = resident_lifetime_.token();
    out.owner = this;
    out.body_owner = body_.get();
    out.manifest = &manifest_;
    out.immutable_preparation = &receiving_.preparation();
    out.immutable_body_preparation = &immutable_body_;
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
      unsigned contributions = 0;
      std::uint32_t selected = 0;
      if (!self.receiving_.historical_point(cursor, point, contributions,
                                            &selected))
        return false;
      output.resident = self.resident_lifetime_.token();
      output.manifest = self.manifest_;
      output.samples_elapsed = cursor;
      output.end_position = point;
      output.explicit_source_history =
          self.receiving_.source_history_.explicit_history;
      output.contributing_source_segments = contributions;
      const auto &history = self.receiving_.source_history_;
      const auto &segment = history.segments[selected];
      output.emitting_body_revision = segment.body_revision;
      output.emitting_effective_sample = segment.effective_sample;
      const auto copy_ref = [&](ReceivingRef &out,
                                ql::ReceivingSourceReference ref) noexcept {
        out = {};
        const auto *s = ql::receiving_source_reference(history, ref);
        if (s)
          std::memcpy(out.data(), s, ref.length);
      };
      copy_ref(output.emitting_preparation, segment.references[2]);
      copy_ref(output.emitting_source_revision, segment.references[1]);
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
      output.version = self.receiving_.source_history_.explicit_history ? 2 : 1;
      output.source_history = self.receiving_.source_history_;
      output.manifest = self.manifest_;
      output.samples_elapsed = cursor;
      output.history_start_sample = self.receiving_.history_start_;
      output.history_linear = self.receiving_.history_;
      return valid_receiving_checkpoint(output, self.manifest_);
    };
    out.write_transport_checkpoint = [](const void *owner,
                                        NativeReceivingCheckpoint &output,
                                        std::uint64_t cursor) noexcept {
      const auto &self =
          *static_cast<const MovingReceivingPortBinding *>(owner);
      if (self.receiving_.elapsed_ != cursor)
        return false;
      output.version = self.receiving_.source_history_.explicit_history ? 2 : 1;
      output.source_history = self.receiving_.source_history_;
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
             valid_receiving_checkpoint(saved, self.manifest_) &&
             (saved.version != 2 ||
              ql::current_receiving_source_matches(
                  saved.source_history, self.immutable_body_,
                  self.receiving_.preparation()));
    };
    out.restore_checkpoint =
        [](void *owner, const NativeReceivingCheckpoint &saved) noexcept {
          auto &self = *static_cast<MovingReceivingPortBinding *>(owner);
          self.receiving_.history_ = saved.history_linear;
          self.receiving_.restore_history_peak();
          if (saved.version == 2)
            self.receiving_.source_history_ = saved.source_history;
          self.receiving_.history_start_ = saved.history_start_sample;
          self.receiving_.elapsed_ = saved.samples_elapsed;
        };
    return out;
  }
};
} // namespace ql::performance
#endif
