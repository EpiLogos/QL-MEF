#ifndef QL_PHYSICAL_TRANSITION_HPP
#define QL_PHYSICAL_TRANSITION_HPP
// Off-thread preparation plus callback-owned commit on the existing P body.
// Source physical-pole §§5/13 leave the numerical transition policy open.
// Exact-node mass projection is an explicit provisional mechanical policy;
// changed elastic/constraint energy is reported as external work, never hidden.
#include <ql/physical_body.hpp>
#include <type_traits>
namespace ql {
enum class PhysicalLiveUpdateKind { Material, FormOrBoundary };
struct PhysicalLiveTransitionReceipt {
  std::uint64_t transaction = 0, before_revision = 0, after_revision = 0,
                samples_elapsed = 0;
  PhysicalLiveUpdateKind kind = PhysicalLiveUpdateKind::Material;
  PhysicalFormTransition policy =
      PhysicalFormTransition::ProjectCorrespondingNodes;
  double before_energy_joules = 0, after_energy_joules = 0,
         external_work_joules = 0;
};
class PreparedPhysicalTransition {
  PhysicalBody candidate_;
  PhysicalBodyInput before_, after_;
  std::string before_basis_, after_basis_, cause_ref_;
  std::vector<double> projection_;
  std::uint64_t transaction_ = 0, sample_ = 0;
  PhysicalLiveUpdateKind kind_;
  PhysicalFormTransition policy_;
  std::size_t before_modes_ = 0, after_modes_ = 0;
  bool committed_ = false;
  static bool same_geometry(const PhysicalBodyInput &a,
                            const PhysicalBodyInput &b) {
    if (a.geometry_ref != b.geometry_ref ||
        a.geometry_revision != b.geometry_revision ||
        a.geometry_source_ref != b.geometry_source_ref ||
        a.geometry_standing != b.geometry_standing || a.family != b.family ||
        a.nodes.size() != b.nodes.size() || a.edges.size() != b.edges.size() ||
        a.exciter.axis != b.exciter.axis ||
        a.exciter.node_weights != b.exciter.node_weights ||
        a.pickup.axis != b.pickup.axis ||
        a.pickup.node_weights != b.pickup.node_weights ||
        a.pickup_linear_per_metre != b.pickup_linear_per_metre)
      return false;
    for (std::size_t n = 0; n < a.nodes.size(); ++n)
      if (a.nodes[n].identity != b.nodes[n].identity ||
          a.nodes[n].constituent != b.nodes[n].constituent ||
          a.nodes[n].rest_metres != b.nodes[n].rest_metres ||
          a.nodes[n].additional_mass_kg != b.nodes[n].additional_mass_kg ||
          a.nodes[n].fixed != b.nodes[n].fixed)
        return false;
    for (std::size_t e = 0; e < a.edges.size(); ++e)
      if (a.edges[e].first != b.edges[e].first ||
          a.edges[e].second != b.edges[e].second ||
          a.edges[e].section_m2 != b.edges[e].section_m2 ||
          a.edges[e].prestress_newtons != b.edges[e].prestress_newtons)
        return false;
    return true;
  }
  // Standard allocator-backed string/vector swaps exchange ownership without
  // allocation or destruction. Do not move-assign whole bodies on callback:
  // an implementation's moved-from string capacity is not a custody promise.
  static void exchange(PhysicalBody &a, PhysicalBody &b) noexcept {
    using std::swap;
    auto &x = a.prepared_.input_;
    auto &y = b.prepared_.input_;
    swap(x.event_ref, y.event_ref);
    swap(x.subject_ref, y.subject_ref);
    swap(x.source_coordinate, y.source_coordinate);
    swap(x.source_revision, y.source_revision);
    swap(x.geometry_ref, y.geometry_ref);
    swap(x.geometry_revision, y.geometry_revision);
    swap(x.geometry_source_ref, y.geometry_source_ref);
    swap(x.geometry_standing, y.geometry_standing);
    swap(x.preparation_ref, y.preparation_ref);
    swap(x.state_ref, y.state_ref);
    swap(x.source_generation, y.source_generation);
    swap(x.body_revision, y.body_revision);
    swap(x.sample_rate, y.sample_rate);
    swap(x.pratibimba, y.pratibimba);
    swap(x.family, y.family);
    swap(x.nodes, y.nodes);
    swap(x.edges, y.edges);
    swap(x.pickup_linear_per_metre, y.pickup_linear_per_metre);
    swap(x.max_force_newtons, y.max_force_newtons);
    swap(x.max_impulse_newton_seconds, y.max_impulse_newton_seconds);
    swap(x.max_displacement_metres, y.max_displacement_metres);
    swap(x.material.reference, y.material.reference);
    swap(x.material.revision, y.material.revision);
    swap(x.material.source_ref, y.material.source_ref);
    swap(x.material.standing, y.material.standing);
    swap(x.material.young_modulus_pa, y.material.young_modulus_pa);
    swap(x.material.density_kg_per_m3, y.material.density_kg_per_m3);
    swap(x.material.damping_alpha_per_second,
         y.material.damping_alpha_per_second);
    swap(x.material.damping_beta_seconds, y.material.damping_beta_seconds);
    swap(x.exciter.axis, y.exciter.axis);
    swap(x.exciter.node_weights, y.exciter.node_weights);
    swap(x.pickup.axis, y.pickup.axis);
    swap(x.pickup.node_weights, y.pickup.node_weights);
    swap(a.prepared_.eigenbasis_identity_, b.prepared_.eigenbasis_identity_);
    swap(a.prepared_.node_mass_, b.prepared_.node_mass_);
    swap(a.prepared_.lambda_, b.prepared_.lambda_);
    swap(a.prepared_.gamma_, b.prepared_.gamma_);
    swap(a.prepared_.excitation_, b.prepared_.excitation_);
    swap(a.prepared_.pickup_, b.prepared_.pickup_);
    swap(a.prepared_.shape_peak_, b.prepared_.shape_peak_);
    swap(a.prepared_.shapes_, b.prepared_.shapes_);
    swap(a.prepared_.step_, b.prepared_.step_);
    swap(a.prepared_.dofs_, b.prepared_.dofs_);
    swap(a.q_, b.q_);
    swap(a.v_, b.v_);
    swap(a.scratch_q_, b.scratch_q_);
    swap(a.scratch_v_, b.scratch_v_);
    swap(a.scratch_audio_, b.scratch_audio_);
    swap(a.elapsed_, b.elapsed_);
    swap(a.last_, b.last_);
  }

public:
  // `before` is the host's retained immutable qualified preparation, not a
  // concurrent reference into a body whose callback may replace its basis.
  // Hold this object in a bounded host preparation slot until acknowledgement;
  // after commit it owns all retired allocations for CONTROL reclamation.
  PreparedPhysicalTransition(const PreparedPhysicalBody &before,
                             PreparedPhysicalBody after,
                             std::uint64_t transaction, std::uint64_t sample,
                             PhysicalLiveUpdateKind kind,
                             PhysicalFormTransition policy,
                             std::string cause_ref)
      : candidate_(std::move(after)), before_(before.input_),
        after_(candidate_.prepared_.input_),
        before_basis_(before.eigenbasis_identity_),
        after_basis_(candidate_.prepared_.eigenbasis_identity_),
        cause_ref_(std::move(cause_ref)), transaction_(transaction),
        sample_(sample), kind_(kind), policy_(policy),
        before_modes_(before.mode_count()), after_modes_(candidate_.q_.size()) {
    reference(cause_ref_);
    require(transaction_ > 0,
            "live body update requires native transaction identity");
    require((kind_ == PhysicalLiveUpdateKind::Material ||
             kind_ == PhysicalLiveUpdateKind::FormOrBoundary) &&
                (policy_ == PhysicalFormTransition::ProjectCorrespondingNodes ||
                 policy_ == PhysicalFormTransition::ExplicitReset),
            "unsupported live body policy");
    require(after_.body_revision > before_.body_revision &&
                after_.source_generation >= before_.source_generation &&
                after_.event_ref == before_.event_ref &&
                after_.subject_ref == before_.subject_ref &&
                after_.state_ref == before_.state_ref &&
                after_.sample_rate == before_.sample_rate &&
                after_.pratibimba == before_.pratibimba &&
                after_.source_revision == before_.source_revision,
            "live body preparation has disconnected source/event/state/face");
    if (kind_ == PhysicalLiveUpdateKind::Material)
      require(policy_ == PhysicalFormTransition::ProjectCorrespondingNodes &&
                  after_.source_coordinate == before_.source_coordinate &&
                  same_geometry(before_, after_),
              "material update cannot reset/change geometry or source form");
    if (policy_ == PhysicalFormTransition::ProjectCorrespondingNodes) {
      require(
          before_.nodes.size() == after_.nodes.size(),
          "live projection requires all exact corresponding node identities");
      for (std::size_t n = 0; n < before_.nodes.size(); ++n)
        require(before_.nodes[n].identity == after_.nodes[n].identity,
                "live projection node identity differs");
      projection_.assign(after_modes_ * before_modes_, 0);
      for (std::size_t a = 0; a < after_modes_; ++a)
        for (std::size_t b = 0; b < before_modes_; ++b) {
          double value = 0;
          for (std::size_t n = 0; n < before_.nodes.size(); ++n)
            for (unsigned axis = 0; axis < 3; ++axis)
              value += candidate_.prepared_.node_mass_[n] *
                       candidate_.prepared_.shapes_[a][n][axis] *
                       before.shapes_[b][n][axis];
          require(std::isfinite(value) && std::abs(value) <= 1e12,
                  "live mass projection exceeds bounded numerical range");
          projection_[a * before_modes_ + b] = value;
        }
    }
  }
  PreparedPhysicalTransition(const PreparedPhysicalTransition &) = delete;
  PreparedPhysicalTransition &
  operator=(const PreparedPhysicalTransition &) = delete;
  PreparedPhysicalTransition(PreparedPhysicalTransition &&) noexcept = default;
  PreparedPhysicalTransition &
  operator=(PreparedPhysicalTransition &&) noexcept = default;
  const PhysicalBodyInput &before_input() const noexcept { return before_; }
  const PhysicalBodyInput &after_input() const noexcept { return after_; }
  const std::string &cause_ref() const noexcept { return cause_ref_; }
  const std::string &before_basis_identity() const noexcept {
    return before_basis_;
  }
  const std::string &after_basis_identity() const noexcept {
    return after_basis_;
  }
  PhysicalLiveUpdateKind kind() const noexcept { return kind_; }
  PhysicalFormTransition policy() const noexcept { return policy_; }
  std::uint64_t transaction() const noexcept { return transaction_; }
  std::uint64_t sample() const noexcept { return sample_; }
  bool committed() const noexcept { return committed_; }
  // Audio owner only, at the exact native scheduled boundary. This modifies
  // candidate scratch on refusal, but resident body/output stay atomic.
  bool apply(PhysicalBody &body,
             PhysicalLiveTransitionReceipt &output) noexcept {
    if (committed_ || body.samples_elapsed() != sample_ ||
        body.body_revision() != before_.body_revision ||
        body.prepared_.eigenbasis_identity_ != before_basis_ ||
        body.q_.size() != before_modes_)
      return false;
    for (std::size_t a = 0; a < after_modes_; ++a) {
      double q = 0, v = 0;
      if (policy_ == PhysicalFormTransition::ProjectCorrespondingNodes)
        for (std::size_t b = 0; b < before_modes_; ++b) {
          const double weight = projection_[a * before_modes_ + b];
          q += weight * body.q_[b];
          v += weight * body.v_[b];
        }
      candidate_.q_[a] = q;
      candidate_.v_[a] = v;
    }
    if (!candidate_.admissible(candidate_.q_, candidate_.v_))
      return false;
    const double before_energy = body.mechanical_energy_joules(),
                 after_energy = candidate_.mechanical_energy_joules();
    double pickup = 0;
    for (std::size_t a = 0; a < after_modes_; ++a)
      pickup += candidate_.q_[a] * candidate_.prepared_.pickup_[a];
    if (!std::isfinite(before_energy) || !std::isfinite(after_energy) ||
        !std::isfinite(pickup) ||
        std::abs(pickup) > std::numeric_limits<float>::max() ||
        !std::isfinite(after_energy - before_energy))
      return false;
    candidate_.elapsed_ = sample_;
    candidate_.last_ = pickup;
    const PhysicalLiveTransitionReceipt result{transaction_,
                                               before_.body_revision,
                                               after_.body_revision,
                                               sample_,
                                               kind_,
                                               policy_,
                                               before_energy,
                                               after_energy,
                                               after_energy - before_energy};
    exchange(body, candidate_);
    committed_ = true;
    output = result;
    return true;
  }
};
static_assert(std::is_nothrow_swappable_v<PhysicalBody>,
              "callback commit must only exchange preallocated body ownership");
} // namespace ql
#endif
