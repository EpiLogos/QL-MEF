#ifndef QL_PERFORMANCE_CONTACT_SLOTS_HPP
#define QL_PERFORMANCE_CONTACT_SLOTS_HPP
#include <atomic>
#include <cstring>
#include <limits>
#include <memory>
#include <ql/performance_contact_program.hpp>

namespace ql::performance {
// Constructor lineage and original Manager/HostRequest ordinal are independent
// of the document CAS, musical generations and the contact's queue sequence.
// This record is numerical custody. Deserializing it confers no admission.
struct NativeContactOccurrence {
  std::array<char, 256> constructor_lineage{};
  std::uint64_t original_request_id = 0;
  bool operator==(const NativeContactOccurrence &b) const noexcept {
    return constructor_lineage == b.constructor_lineage &&
           original_request_id == b.original_request_id;
  }
};
class NativeSceneContactOccurrenceOwner;
// The retained private Scene owner supplies this after replaying the complete
// admitted occurrence/history on the same qualified host channel. There is no
// JSON reader, public constructor, clone, or caller-issued ordinal factory.
class NativeContactOccurrenceWitness {
  friend class NativeSceneContactOccurrenceOwner;
  NativeContactOccurrence occurrence_{};
  std::shared_ptr<const PreparedContactProgram> program_{};
  NativeContactOccurrenceWitness(
      NativeContactOccurrence occurrence,
      std::shared_ptr<const PreparedContactProgram> qualified_program)
      : occurrence_(occurrence), program_(std::move(qualified_program)) {}

public:
  NativeContactOccurrenceWitness(const NativeContactOccurrenceWitness &) =
      delete;
  NativeContactOccurrenceWitness &
  operator=(const NativeContactOccurrenceWitness &) = delete;
  const NativeContactOccurrence &occurrence() const noexcept {
    return occurrence_;
  }
  bool matches(const PreparedContactProgram &p) const noexcept {
    // Exact already-qualified immutable program custody, never only labels.
    return program_.get() == &p;
  }
};
struct NativeContactSourceIdentity {
  std::array<char, 256> instance{}, event{}, subject{};
  std::uint64_t m1_revision = 0, m2_generation = 0;
  bool operator==(const NativeContactSourceIdentity &b) const noexcept {
    return instance == b.instance && event == b.event && subject == b.subject &&
           m1_revision == b.m1_revision && m2_generation == b.m2_generation;
  }
};
struct NativeContactSlotCheckpoint {
  bool present = false;
  NativeContactHandle handle{};
  NativeContactOccurrence occurrence{};
  NativeContactSourceIdentity source{};
  std::uint64_t admission_sequence = 0;
  ql::GravityContactInput original{};
  ql::PhysicalBodyInput original_body{};
  NativeContactOperands operands{};
  std::array<double, ql::physical_max_frames> force_newtons{};
  NativeContactDelivery delivery{};
};
struct NativeContactCheckpoint {
  static constexpr const char *schema = "ql.performance-contact-state/v1";
  bool history_present = false;
  std::array<char, 256> constructor_lineage{};
  std::uint64_t original_request_high_water = 0;
  std::array<std::uint64_t, contact_slot_capacity> slot_generations{};
  std::array<NativeContactSlotCheckpoint, contact_slot_capacity> slots{};
};
// Control-prepared restore custody. Its construction checks every original
// operand against the actual immutable P preparation before any P mutation.
// Its shared ownership never reaches the callback.
struct NativePreparedContactRestore {
  NativeContactCheckpoint checkpoint{};
  std::array<std::shared_ptr<const PreparedContactProgram>,
             contact_slot_capacity>
      programs{};
};
enum class NativeContactReservation : std::uint8_t {
  Ready,
  Invalid,
  Stale,
  Duplicate,
  Exhausted,
  Budget
};
class NativeContactSlots {
  struct Slot {
    std::shared_ptr<const PreparedContactProgram> custody{};
    std::atomic<const PreparedContactProgram *> published{nullptr};
    std::atomic<std::uint64_t> generation{0}, acknowledged{0};
    std::shared_ptr<const PreparedContactProgram> rollback_custody{};
    NativeContactOccurrence rollback_occurrence{};
    NativeContactSourceIdentity rollback_source{};
    std::uint64_t rollback_sequence = 0;
    NativeContactOccurrence occurrence{};
    NativeContactSourceIdentity source{};
    std::uint64_t sequence = 0;
  };
  std::array<Slot, contact_slot_capacity> slots_{};
  // Only callback/stopped custody writes these; control never reads them live.
  std::array<NativeContactDelivery, contact_slot_capacity> committed_{};
  std::array<NativeContactDelivery, contact_slot_capacity> scratch_{};
  std::array<NativeContactHandle, contact_slot_capacity> observed_handles_{};
  std::array<double, contact_slot_capacity> observed_reservations_{};
  // Only the existing serial producer writes the occurrence high-water.
  std::array<char, 256> lineage_{};
  std::uint64_t high_water_ = 0;
  bool history_present_ = false;

  static bool valid_lineage(const std::array<char, 256> &r) noexcept {
    return r[0] && std::find(r.begin(), r.end(), '\0') != r.end();
  }
  static bool terminal(NativeContactStatus s) noexcept {
    return s == NativeContactStatus::Completed ||
           s == NativeContactStatus::Refused ||
           s == NativeContactStatus::Interrupted;
  }
  const Slot *slot(NativeContactHandle h) const noexcept {
    if (!h.valid())
      return nullptr;
    const auto &s = slots_[h.slot];
    return s.generation.load(std::memory_order_acquire) == h.generation
               ? &s
               : nullptr;
  }
  static bool same_operands(const NativeContactOperands &a,
                            const NativeContactOperands &b) noexcept {
    // Compare values, never padding bytes of a reconstructed wire object.
    return a.contact_ref == b.contact_ref && a.particle_ref == b.particle_ref &&
           a.collider_ref == b.collider_ref && a.route_ref == b.route_ref &&
           a.source_ref == b.source_ref && a.policy_ref == b.policy_ref &&
           a.policy_revision == b.policy_revision && a.standing == b.standing &&
           a.seed_standing == b.seed_standing &&
           a.preparation_ref == b.preparation_ref &&
           a.state_ref == b.state_ref && a.eigenbasis == b.eigenbasis &&
           a.source_coordinate == b.source_coordinate &&
           a.source_revision == b.source_revision &&
           a.plane_position_metres[0] == b.plane_position_metres[0] &&
           a.plane_position_metres[1] == b.plane_position_metres[1] &&
           a.plane_position_metres[2] == b.plane_position_metres[2] &&
           a.normal[0] == b.normal[0] && a.normal[1] == b.normal[1] &&
           a.normal[2] == b.normal[2] &&
           a.impact_velocity_metres_per_second[0] ==
               b.impact_velocity_metres_per_second[0] &&
           a.impact_velocity_metres_per_second[1] ==
               b.impact_velocity_metres_per_second[1] &&
           a.impact_velocity_metres_per_second[2] ==
               b.impact_velocity_metres_per_second[2] &&
           a.height_metres == b.height_metres &&
           a.initial_normal_velocity_metres_per_second ==
               b.initial_normal_velocity_metres_per_second &&
           a.gravity_metres_per_second_squared ==
               b.gravity_metres_per_second_squared &&
           a.mass_kg == b.mass_kg && a.restitution == b.restitution &&
           a.transfer_fraction == b.transfer_fraction &&
           a.minimum_impact_speed_metres_per_second ==
               b.minimum_impact_speed_metres_per_second &&
           a.impact_seconds == b.impact_seconds &&
           a.impact_speed_metres_per_second ==
               b.impact_speed_metres_per_second &&
           a.planned_impulse_newton_seconds ==
               b.planned_impulse_newton_seconds &&
           a.force_newtons == b.force_newtons &&
           a.trigger_sample == b.trigger_sample &&
           a.impact_sample == b.impact_sample &&
           a.body_revision == b.body_revision &&
           a.source_generation == b.source_generation && a.seed == b.seed &&
           a.sample_rate == b.sample_rate &&
           a.duration_samples == b.duration_samples &&
           a.pratibimba == b.pratibimba;
  }

public:
  static_assert(
      std::atomic<const PreparedContactProgram *>::is_always_lock_free);
  static_assert(std::is_trivially_copyable_v<NativeContactOccurrence>);
  static_assert(std::is_trivially_copyable_v<NativeContactSourceIdentity>);
  // Numerical reservation is called by Engine only after the private witness
  // and its full current source/program have qualified. Publishing a numerical
  // slot alone does not create a native operation or application.
  NativeContactReservation
  reserve(const std::shared_ptr<const PreparedContactProgram> &p,
          const NativeContactOccurrence &occurrence,
          const NativeContactSourceIdentity &source, std::uint64_t sequence,
          const ql::PreparedPhysicalBody &body, std::uint64_t admitted_cursor,
          double scalar_budget_newtons, NativeContactHandle &out) noexcept {
    out = {};
    if (!p || !sequence || !valid_lineage(occurrence.constructor_lineage) ||
        !occurrence.original_request_id ||
        !std::isfinite(scalar_budget_newtons) || scalar_budget_newtons <= 0)
      return NativeContactReservation::Invalid;
    if (!p->matches_preparation(body, admitted_cursor))
      return NativeContactReservation::Stale;
    if (history_present_ && occurrence.constructor_lineage != lineage_)
      return NativeContactReservation::Stale;
    if (occurrence.original_request_id <= high_water_)
      return NativeContactReservation::Duplicate;
    const double remaining = scalar_budget_newtons - reserved_force_newtons();
    if (std::abs(p->operands().force_newtons) > remaining)
      return NativeContactReservation::Budget;
    for (std::size_t i = 0; i < slots_.size(); ++i) {
      auto &s = slots_[i];
      const auto generation = s.generation.load(std::memory_order_acquire);
      if (generation &&
          s.acknowledged.load(std::memory_order_acquire) != generation)
        continue;
      if (generation == std::numeric_limits<std::uint64_t>::max())
        continue;
      // Acknowledgement is published after the callback's final raw access.
      // Thus replacing this control-owned shared_ptr cannot free live data.
      s.rollback_custody = s.custody;
      s.rollback_occurrence = s.occurrence;
      s.rollback_source = s.source;
      s.rollback_sequence = s.sequence;
      s.custody = p;
      s.occurrence = occurrence;
      s.source = source;
      s.sequence = sequence;
      s.published.store(p.get(), std::memory_order_relaxed);
      s.generation.store(generation + 1, std::memory_order_release);
      out = {generation + 1, std::uint8_t(i)};
      return NativeContactReservation::Ready;
    }
    return NativeContactReservation::Exhausted;
  }
  // After the ordinary Engine queue accepts. Queue refusal leaves dedup and
  // generation history untouched and can release an unpublished reservation.
  void accept(NativeContactHandle h) noexcept {
    const auto *s = slot(h);
    if (!s)
      std::terminate();
    lineage_ = s->occurrence.constructor_lineage;
    high_water_ = s->occurrence.original_request_id;
    history_present_ = true;
    slots_[h.slot].rollback_custody.reset();
  }
  void rollback_unpublished(NativeContactHandle h) noexcept {
    if (!h.valid())
      return;
    auto &s = slots_[h.slot];
    if (s.generation.load(std::memory_order_acquire) != h.generation)
      std::terminate();
    s.custody = std::move(s.rollback_custody);
    s.occurrence = s.rollback_occurrence;
    s.source = s.rollback_source;
    s.sequence = s.rollback_sequence;
    s.published.store(s.custody.get(), std::memory_order_relaxed);
    s.generation.store(h.generation - 1, std::memory_order_release);
  }
  double reserved_force_newtons() const noexcept {
    double reserve = 0;
    for (const auto &s : slots_) {
      const auto generation = s.generation.load(std::memory_order_acquire);
      if (generation && s.custody &&
          s.acknowledged.load(std::memory_order_acquire) != generation)
        reserve += std::abs(s.custody->operands().force_newtons);
    }
    return reserve;
  }
  bool history_present() const noexcept { return history_present_; }
  static bool same_delivery(const NativeContactDelivery &a,
                            const NativeContactDelivery &b) noexcept {
    return a.handle == b.handle && a.status == b.status &&
           a.refusal == b.refusal &&
           a.admission_sequence == b.admission_sequence &&
           a.start_application_ordinal == b.start_application_ordinal &&
           a.committed_cursor == b.committed_cursor &&
           a.requested_impact_sample == b.requested_impact_sample &&
           a.admitted_impact_sample == b.admitted_impact_sample &&
           a.actual_impact_sample == b.actual_impact_sample &&
           a.delivered_frames == b.delivered_frames &&
           a.planned_frames == b.planned_frames &&
           a.delivered_impulse_newton_seconds ==
               b.delivered_impulse_newton_seconds &&
           a.planned_impulse_newton_seconds == b.planned_impulse_newton_seconds;
  }
  static bool same_original(const ql::GravityContactInput &a,
                            const ql::GravityContactInput &b) noexcept {
    return a.contact_ref == b.contact_ref && a.particle_ref == b.particle_ref &&
           a.collider_ref == b.collider_ref && a.route_ref == b.route_ref &&
           a.source_ref == b.source_ref && a.policy_ref == b.policy_ref &&
           a.policy_revision == b.policy_revision && a.standing == b.standing &&
           a.plane_position_metres == b.plane_position_metres &&
           a.normal == b.normal && a.height_metres == b.height_metres &&
           a.initial_normal_velocity_metres_per_second ==
               b.initial_normal_velocity_metres_per_second &&
           a.gravity_metres_per_second_squared ==
               b.gravity_metres_per_second_squared &&
           a.mass_kg == b.mass_kg && a.restitution == b.restitution &&
           a.transfer_fraction == b.transfer_fraction &&
           a.minimum_impact_speed_metres_per_second ==
               b.minimum_impact_speed_metres_per_second &&
           a.start_sample == b.start_sample &&
           a.duration_samples == b.duration_samples;
  }
  static bool same_body_input(const ql::PhysicalBodyInput &a,
                              const ql::PhysicalBodyInput &b) noexcept {
    if (a.event_ref != b.event_ref || a.subject_ref != b.subject_ref ||
        a.source_coordinate != b.source_coordinate ||
        a.source_revision != b.source_revision ||
        a.geometry_ref != b.geometry_ref ||
        a.geometry_revision != b.geometry_revision ||
        a.geometry_source_ref != b.geometry_source_ref ||
        a.geometry_standing != b.geometry_standing ||
        a.preparation_ref != b.preparation_ref || a.state_ref != b.state_ref ||
        a.source_generation != b.source_generation ||
        a.body_revision != b.body_revision || a.sample_rate != b.sample_rate ||
        a.pratibimba != b.pratibimba || a.family != b.family ||
        a.material.reference != b.material.reference ||
        a.material.revision != b.material.revision ||
        a.material.source_ref != b.material.source_ref ||
        a.material.standing != b.material.standing ||
        a.material.young_modulus_pa != b.material.young_modulus_pa ||
        a.material.density_kg_per_m3 != b.material.density_kg_per_m3 ||
        a.material.damping_alpha_per_second !=
            b.material.damping_alpha_per_second ||
        a.material.damping_beta_seconds != b.material.damping_beta_seconds ||
        a.exciter.axis != b.exciter.axis ||
        a.exciter.node_weights != b.exciter.node_weights ||
        a.pickup.axis != b.pickup.axis ||
        a.pickup.node_weights != b.pickup.node_weights ||
        a.pickup_linear_per_metre != b.pickup_linear_per_metre ||
        a.max_force_newtons != b.max_force_newtons ||
        a.max_impulse_newton_seconds != b.max_impulse_newton_seconds ||
        a.max_displacement_metres != b.max_displacement_metres ||
        a.nodes.size() != b.nodes.size() || a.edges.size() != b.edges.size())
      return false;
    for (std::size_t i = 0; i < a.nodes.size(); ++i) {
      const auto &x = a.nodes[i], &y = b.nodes[i];
      if (x.identity != y.identity || x.constituent != y.constituent ||
          x.rest_metres != y.rest_metres ||
          x.additional_mass_kg != y.additional_mass_kg || x.fixed != y.fixed)
        return false;
    }
    for (std::size_t i = 0; i < a.edges.size(); ++i) {
      const auto &x = a.edges[i], &y = b.edges[i];
      if (x.first != y.first || x.second != y.second ||
          x.section_m2 != y.section_m2 ||
          x.prestress_newtons != y.prestress_newtons)
        return false;
    }
    return true;
  }
  static bool same_checkpoint(const NativeContactCheckpoint &a,
                              const NativeContactCheckpoint &b) noexcept {
    if (a.history_present != b.history_present ||
        a.constructor_lineage != b.constructor_lineage ||
        a.original_request_high_water != b.original_request_high_water ||
        a.slot_generations != b.slot_generations)
      return false;
    for (std::size_t i = 0; i < a.slots.size(); ++i) {
      const auto &x = a.slots[i], &y = b.slots[i];
      if (x.present != y.present || !(x.handle == y.handle) ||
          !(x.occurrence == y.occurrence) || !(x.source == y.source) ||
          x.admission_sequence != y.admission_sequence ||
          !same_original(x.original, y.original) ||
          !same_body_input(x.original_body, y.original_body) ||
          !same_operands(x.operands, y.operands) ||
          x.force_newtons != y.force_newtons ||
          !same_delivery(x.delivery, y.delivery))
        return false;
    }
    return true;
  }
  // Only the callback's actual ordinary queue drain publishes this reserve.
  // A failed queue push cannot change playing note headroom or physical input.
  bool observe_queue(NativeContactHandle h) noexcept {
    const auto *s = slot(h);
    const auto *p = s ? s->published.load(std::memory_order_acquire) : nullptr;
    if (!p)
      return false;
    observed_handles_[h.slot] = h;
    observed_reservations_[h.slot] = std::abs(p->operands().force_newtons);
    return true;
  }
  void begin_block() noexcept { scratch_ = committed_; }
  bool begin(NativeContactHandle h, std::uint64_t sequence,
             const NativeContactSourceIdentity &source,
             const ql::PreparedPhysicalBody &body, std::uint64_t actual_sample,
             std::uint64_t panic_fence, NativeContactOperands &operands,
             NativeContactOccurrence &occurrence) noexcept {
    const auto *s = slot(h);
    if (!s)
      return false;
    const auto *p = s->published.load(std::memory_order_acquire);
    if (!p || s->sequence != sequence)
      return false;
    operands = p->operands();
    occurrence = s->occurrence;
    auto &d = scratch_[h.slot];
    d = {};
    d.handle = h;
    d.admission_sequence = sequence;
    d.requested_impact_sample = d.admitted_impact_sample =
        p->operands().impact_sample;
    d.actual_impact_sample = actual_sample;
    d.planned_frames = p->operands().duration_samples;
    d.planned_impulse_newton_seconds =
        p->operands().planned_impulse_newton_seconds;
    d.status = NativeContactStatus::Refused;
    if (sequence <= panic_fence)
      d.refusal = NativeContactRefusal::EmergencyFenced;
    else if (!(s->source == source))
      d.refusal = NativeContactRefusal::CurrentSourceChanged;
    else if (actual_sample != p->operands().impact_sample ||
             !p->matches_preparation(body, p->operands().trigger_sample))
      d.refusal = NativeContactRefusal::PhysicalPreparationChanged;
    else {
      d.status = NativeContactStatus::Delivering;
      return true;
    }
    return false;
  }
  void requalify(const NativeContactSourceIdentity &source,
                 const ql::PreparedPhysicalBody &body) noexcept {
    for (auto &d : scratch_) {
      if (d.status != NativeContactStatus::Delivering)
        continue;
      const auto *s = slot(d.handle);
      const auto *p =
          s ? s->published.load(std::memory_order_acquire) : nullptr;
      if (!s || !(s->source == source)) {
        d.status = NativeContactStatus::Interrupted;
        d.refusal = NativeContactRefusal::CurrentSourceChanged;
      } else if (!p ||
                 !p->matches_preparation(body, p->operands().trigger_sample)) {
        d.status = NativeContactStatus::Interrupted;
        d.refusal = NativeContactRefusal::PhysicalPreparationChanged;
      }
    }
  }
  double force_at(std::uint64_t sample) noexcept {
    double force = 0;
    for (std::size_t i = 0; i < scratch_.size(); ++i) {
      auto &d = scratch_[i];
      if (d.status != NativeContactStatus::Delivering)
        continue;
      const auto *s = slot(d.handle);
      const auto *p =
          s ? s->published.load(std::memory_order_acquire) : nullptr;
      if (!p || sample != d.admitted_impact_sample + d.delivered_frames ||
          d.delivered_frames >= p->force().frames) {
        d.status = NativeContactStatus::Interrupted;
        d.refusal = NativeContactRefusal::PhysicalPreparationChanged;
        continue;
      }
      const double f = p->force().force_newtons[d.delivered_frames];
      force += f;
      d.delivered_impulse_newton_seconds += f / p->force().sample_rate;
      if (++d.delivered_frames == p->force().frames)
        d.status = NativeContactStatus::Completed;
    }
    return force;
  }
  void interrupt(std::uint64_t fence) noexcept {
    for (auto &d : scratch_)
      if (d.status == NativeContactStatus::Delivering &&
          d.admission_sequence <= fence) {
        d.status = NativeContactStatus::Interrupted;
        d.refusal = NativeContactRefusal::EmergencyFenced;
      }
  }
  // Callback budget includes every accepted future slot. Accessing the control
  // shared_ptr here is forbidden; immutable raw pointers suffice.
  double callback_reserved_force_newtons() const noexcept {
    double reserve = 0;
    for (double force : observed_reservations_)
      reserve += force;
    return reserve;
  }

  void application_ordinal(NativeContactHandle h,
                           std::uint64_t ordinal) noexcept {
    if (h.valid() && scratch_[h.slot].handle == h)
      scratch_[h.slot].start_application_ordinal = ordinal;
  }
  std::uint64_t uncommitted_sequence() const noexcept {
    for (std::size_t i = 0; i < scratch_.size(); ++i)
      if (scratch_[i].admission_sequence &&
          !same_delivery(scratch_[i], committed_[i]))
        return scratch_[i].admission_sequence;
    return 0;
  }
  // Only after actual P, receiver and finite output have all committed.
  // No raw program access follows the acknowledgement in this callback.
  void commit(std::uint64_t cursor) noexcept {
    for (auto &d : scratch_)
      if (d.status != NativeContactStatus::Empty)
        d.committed_cursor = cursor;
    committed_ = scratch_;
  }
  void acknowledge_terminals() noexcept {
    for (const auto &d : committed_)
      if (d.handle.valid() && terminal(d.status)) {
        if (observed_handles_[d.handle.slot] == d.handle) {
          observed_handles_[d.handle.slot] = {};
          observed_reservations_[d.handle.slot] = 0;
        }
        slots_[d.handle.slot].acknowledged.store(d.handle.generation,
                                                 std::memory_order_release);
      }
  }

  const std::array<NativeContactDelivery, contact_slot_capacity> &
  deliveries() const noexcept {
    return committed_;
  }
  void write_checkpoint(NativeContactCheckpoint &cp) const {
    cp = {};
    cp.history_present = history_present_;
    cp.constructor_lineage = lineage_;
    cp.original_request_high_water = high_water_;
    for (std::size_t i = 0; i < slots_.size(); ++i) {
      const auto &s = slots_[i];
      const auto generation = s.generation.load(std::memory_order_acquire);
      cp.slot_generations[i] = generation;
      if (!generation || !s.custody)
        continue;
      auto &v = cp.slots[i];
      v.present = true;
      v.handle = {generation, std::uint8_t(i)};
      v.occurrence = s.occurrence;
      v.source = s.source;
      v.admission_sequence = s.sequence;
      v.original = s.custody->original();
      v.original_body = s.custody->original_body();
      v.operands = s.custody->operands();
      v.force_newtons = s.custody->force().force_newtons;
      v.delivery = committed_[i];
      if (!(v.delivery.handle == v.handle)) {
        v.delivery = {};
        v.delivery.handle = v.handle;
        v.delivery.status = NativeContactStatus::Queued;
        v.delivery.admission_sequence = s.sequence;
        v.delivery.requested_impact_sample = v.delivery.admitted_impact_sample =
            v.operands.impact_sample;
        v.delivery.planned_frames = v.operands.duration_samples;
        v.delivery.planned_impulse_newton_seconds =
            v.operands.planned_impulse_newton_seconds;
      }
    }
  }
  static std::shared_ptr<const NativePreparedContactRestore>
  prepare_restore(const NativeContactCheckpoint &cp,
                  const ql::PreparedPhysicalBody &body, std::uint64_t cursor,
                  std::uint64_t accepted_sequence) {
    ql::require(cp.history_present == bool(cp.original_request_high_water) &&
                    (cp.history_present
                         ? valid_lineage(cp.constructor_lineage)
                         : cp.constructor_lineage == std::array<char, 256>{}),
                "native contact checkpoint occurrence owner differs");
    auto out = std::make_shared<NativePreparedContactRestore>();
    out->checkpoint = cp;
    for (std::size_t i = 0; i < cp.slots.size(); ++i) {
      const auto &s = cp.slots[i];
      if (!s.present) {
        ql::require(!cp.slot_generations[i] &&
                        s.delivery.status == NativeContactStatus::Empty,
                    "missing original contact slot history");
        continue;
      }
      ql::require(cp.history_present && s.handle.slot == i &&
                      s.handle.generation == cp.slot_generations[i] &&
                      s.handle.valid() && s.admission_sequence &&
                      s.admission_sequence <= accepted_sequence &&
                      s.occurrence.constructor_lineage ==
                          cp.constructor_lineage &&
                      s.occurrence.original_request_id &&
                      s.occurrence.original_request_id <=
                          cp.original_request_high_water &&
                      s.delivery.handle == s.handle &&
                      s.delivery.admission_sequence == s.admission_sequence &&
                      s.delivery.status != NativeContactStatus::Empty &&
                      unsigned(s.delivery.status) <=
                          unsigned(NativeContactStatus::Interrupted) &&
                      unsigned(s.delivery.refusal) <=
                          unsigned(NativeContactRefusal::ForceBudgetChanged),
                  "native contact checkpoint slot custody differs");
      const ql::PreparedPhysicalBody original_body(s.original_body);
      auto p = std::make_shared<const PreparedContactProgram>(
          original_body, s.original.start_sample, s.original);
      ql::require(s.delivery.status != NativeContactStatus::Delivering ||
                      p->matches_preparation(body, s.original.start_sample),
                  "live contact checkpoint has stale physical preparation");
      ql::require(same_operands(p->operands(), s.operands) &&
                      p->force().force_newtons == s.force_newtons,
                  "native contact original program no longer reproduces");
      const auto &d = s.delivery;
      ql::require(d.planned_frames == p->force().frames &&
                      d.delivered_frames <= d.planned_frames &&
                      d.planned_impulse_newton_seconds ==
                          p->force().impulse_newton_seconds &&
                      d.requested_impact_sample == p->force().start_sample &&
                      d.admitted_impact_sample == p->force().start_sample &&
                      d.committed_cursor <= cursor,
                  "native contact progress date/program differs");
      double delivered = 0;
      for (std::size_t frame = 0; frame < d.delivered_frames; ++frame)
        delivered += p->force().force_newtons[frame] / p->force().sample_rate;
      ql::require(
          delivered == d.delivered_impulse_newton_seconds &&
              (d.status != NativeContactStatus::Queued ||
               (!d.delivered_frames && p->force().start_sample >= cursor)) &&
              (d.status != NativeContactStatus::Delivering ||
               (d.delivered_frames && d.delivered_frames < d.planned_frames &&
                d.actual_impact_sample == p->force().start_sample &&
                cursor == p->force().start_sample + d.delivered_frames)) &&
              (d.status != NativeContactStatus::Completed ||
               d.delivered_frames == d.planned_frames) &&
              (d.status != NativeContactStatus::Refused || !d.delivered_frames),
          "native contact checkpoint would lose/repeat physical force");
      for (std::size_t j = 0; j < i; ++j)
        ql::require(!cp.slots[j].present ||
                        !(cp.slots[j].occurrence == s.occurrence),
                    "native contact checkpoint repeats an original occurrence");
      out->programs[i] = std::move(p);
    }
    return out;
  }
  void restore(const NativePreparedContactRestore &prepared) noexcept {
    const auto &cp = prepared.checkpoint;
    history_present_ = cp.history_present;
    lineage_ = cp.constructor_lineage;
    high_water_ = cp.original_request_high_water;
    for (std::size_t i = 0; i < slots_.size(); ++i) {
      auto &s = slots_[i];
      const auto &v = cp.slots[i];
      s.custody = prepared.programs[i];
      s.rollback_custody.reset();
      s.occurrence = v.occurrence;
      s.source = v.source;
      s.sequence = v.admission_sequence;
      s.published.store(s.custody.get(), std::memory_order_relaxed);
      s.generation.store(cp.slot_generations[i], std::memory_order_release);
      s.acknowledged.store(terminal(v.delivery.status) ? v.handle.generation
                                                       : 0,
                           std::memory_order_release);
      committed_[i] = v.delivery;
      observed_handles_[i] = v.present && !terminal(v.delivery.status)
                                 ? v.handle
                                 : NativeContactHandle{};
      observed_reservations_[i] = v.present && !terminal(v.delivery.status)
                                      ? std::abs(v.operands.force_newtons)
                                      : 0;
    }
    scratch_ = committed_;
  }
};
} // namespace ql::performance
#endif
