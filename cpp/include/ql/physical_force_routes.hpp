#ifndef QL_PHYSICAL_FORCE_ROUTES_HPP
#define QL_PHYSICAL_FORCE_ROUTES_HPP
#include <ql/m2.h>
#include <ql/m_tree_live.h>
#include <ql/physical_body.hpp>

namespace ql {
inline constexpr const char *physical_force_routes_contract =
    "ql.physical-force-routes/v1";
inline constexpr std::size_t physical_max_personal_force_routes = 9;

// These are producer evidence, never consent or host authentication. The
// serial A/N owner revalidates the original definition/current native event
// before preparing. M1 revision and M2/M3 generations remain independent.
struct PhysicalForceSourceBasis {
  std::string event_ref, subject_ref, registry_revision, source_revision;
  std::string definition_ref, source_instance_ref, determination_ref;
  std::string m2_writer_coordinate;
  bool m2_pratibimba = true;
  std::uint64_t m1_revision = 0, m2_generation = 0, m3_generation = 0;
  bool operator==(const PhysicalForceSourceBasis &b) const noexcept {
    return event_ref == b.event_ref && subject_ref == b.subject_ref &&
           registry_revision == b.registry_revision &&
           source_revision == b.source_revision &&
           definition_ref == b.definition_ref &&
           source_instance_ref == b.source_instance_ref &&
           determination_ref == b.determination_ref &&
           m2_writer_coordinate == b.m2_writer_coordinate &&
           m2_pratibimba == b.m2_pratibimba && m1_revision == b.m1_revision &&
           m2_generation == b.m2_generation && m3_generation == b.m3_generation;
  }
};
struct PhysicalForceRouteInput {
  std::string driver_ref, target_ref, program_ref;
  std::string planet_coordinate, chakra_coordinate;
  // Centre ordinal is N's 0..6 storage ordinal; chakra coordinate/ID retains
  // the actual native local segment 1..7. Earth is segment 0, never a route.
  unsigned native_planet_index = 0, centre_ordinal = 0;
  QL_M_NodeId planet_node_id = 0, chakra_node_id = 0;
  std::vector<std::string> relation_refs;
  // N retains the ten-contributor original denominator. P never renormalizes
  // its nine routes or derives a metric calibration from anatomy text.
  std::string share_numerator, share_denominator;
  std::string projection_ref, calibration_ref, calibration_revision;
  std::string calibration_source_ref, calibration_standing;
  std::vector<std::uint64_t> metric_node_ids;
  PhysicalProjection projection;
  double peak_force_newtons = 0;
};
// Waveform/phase state lives in the existing A source program/checkpoint.
// A must explicitly enable the existing M1 instrument bus to avoid secretly
// duplicating a personal route that already realizes that source function.
struct PhysicalScalarForceBlock {
  const double *force_newtons = nullptr;
  double gain = 0;
  bool enabled = false;
};
struct PhysicalRouteForceBlock {
  std::size_t route_index = 0;
  std::uint64_t preparation_seal = 0;
  const double *force_newtons = nullptr;
  double gain = 0;
  bool enabled = false;
};
struct PhysicalForceRouteUse {
  std::size_t route_index = 0;
  std::uint64_t preparation_seal = 0;
  QL_M_NodeId planet_node_id = 0, chakra_node_id = 0;
  unsigned native_planet_index = 0, centre_ordinal = 0;
  bool enabled = false;
  double gain = 0, peak_applied_force_newtons = 0;
};
struct PhysicalForceRouteReceipt {
  std::uint64_t source_basis_seal = 0, body_revision = 0;
  QL_M_NodeId earth_frame_node_id = 0;
  std::uint64_t start_sample = 0, end_sample = 0;
  std::size_t route_count = 0;
  bool scalar_m1_enabled = false;
  double scalar_m1_gain = 0, scalar_peak_applied_force_newtons = 0;
  double peak_total_absolute_force_newtons = 0;
  std::array<PhysicalForceRouteUse, physical_max_personal_force_routes>
      routes{};
};

class PreparedPhysicalForceRoutes {
  PhysicalForceSourceBasis source_;
  std::vector<PhysicalForceRouteInput> routes_;
  std::string eigenbasis_identity_, preparation_ref_, state_ref_;
  std::string body_source_coordinate_, body_source_revision_;
  bool body_pratibimba_ = false;
  std::uint64_t body_revision_ = 0, admitted_cursor_ = 0, source_seal_ = 0;
  QL_M_NodeId earth_frame_node_id_ = 0;
  unsigned sample_rate_ = 0;
  std::size_t modes_ = 0;
  std::array<std::array<double, physical_max_dofs>,
             physical_max_personal_force_routes>
      coefficients_{};
  std::array<std::uint64_t, physical_max_personal_force_routes> seals_{};
  static bool reference(const std::string &s) noexcept {
    if (s.empty() || s.size() > 4096 || s.front() == ' ' || s.back() == ' ')
      return false;
    for (unsigned char c : s)
      if (c < 32 || c == 127)
        return false;
    return true;
  }
  static bool decimal(const std::string &s, bool nonzero) noexcept {
    if (s.empty() || s.size() > 4096 || (s.size() > 1 && s.front() == '0') ||
        (nonzero && s == "0"))
      return false;
    for (unsigned char c : s)
      if (c < '0' || c > '9')
        return false;
    return true;
  }
  struct Fingerprint {
    std::uint64_t value = 14695981039346656037ULL;
    void integer(std::uint64_t x) noexcept {
      for (unsigned i = 0; i < 8; ++i) {
        value =
            (value ^ static_cast<unsigned char>(x & 255)) * 1099511628211ULL;
        x >>= 8;
      }
    }
    void text(const std::string &s) noexcept {
      integer(s.size());
      for (unsigned char c : s)
        value = (value ^ c) * 1099511628211ULL;
    }
    void real(double x) noexcept {
      std::uint64_t bits;
      std::memcpy(&bits, &x, sizeof(bits));
      integer(bits);
    }
  };
  bool matches(const PreparedPhysicalBody &body) const noexcept {
    const auto &in = body.input();
    return body.eigenbasis_identity() == eigenbasis_identity_ &&
           in.preparation_ref == preparation_ref_ &&
           in.state_ref == state_ref_ && in.event_ref == source_.event_ref &&
           in.subject_ref == source_.subject_ref &&
           in.source_coordinate == body_source_coordinate_ &&
           in.source_revision == body_source_revision_ &&
           in.pratibimba == body_pratibimba_ &&
           in.source_generation == source_.m3_generation &&
           in.body_revision == body_revision_ &&
           in.sample_rate == sample_rate_ && body.mode_count() == modes_;
  }

public:
  // Immutable preparation + admitted native cursor only. Never read a live
  // body's changing q/v/cursor from the control thread. Constructor performs
  // bounded native graph lookup and modal projection outside the callback.
  PreparedPhysicalForceRoutes(const PreparedPhysicalBody &body,
                              PhysicalForceSourceBasis source,
                              std::vector<PhysicalForceRouteInput> routes,
                              std::uint64_t admitted_cursor)
      : source_(std::move(source)), routes_(std::move(routes)),
        eigenbasis_identity_(body.eigenbasis_identity()),
        preparation_ref_(body.input().preparation_ref),
        state_ref_(body.input().state_ref),
        body_source_coordinate_(body.input().source_coordinate),
        body_source_revision_(body.input().source_revision),
        body_pratibimba_(body.input().pratibimba),
        body_revision_(body.input().body_revision),
        admitted_cursor_(admitted_cursor),
        sample_rate_(body.input().sample_rate), modes_(body.mode_count()) {
    require(routes_.size() <= physical_max_personal_force_routes,
            "at most nine personal force routes");
    for (const auto *ref :
         {&source_.event_ref, &source_.subject_ref, &source_.registry_revision,
          &source_.source_revision, &source_.definition_ref,
          &source_.source_instance_ref, &source_.determination_ref,
          &source_.m2_writer_coordinate})
      require(reference(*ref),
              "bounded native force source reference required");
    require(source_.registry_revision == ql_m_live_registry_revision() &&
                source_.source_revision == ql_m_live_source_revision(),
            "force source registry/revision is stale");
    const auto *earth = ql_m_live_resolve("#2-5-0/1-0");
    require(earth, "native Earth observer identity unavailable");
    earth_frame_node_id_ = earth->id;
    const auto *writer =
        ql_m_live_resolve(source_.m2_writer_coordinate.c_str());
    const auto *vimarsha = ql_m_live_resolve("#2-1");
    require(writer && vimarsha && writer->id == vimarsha->id &&
                source_.m2_writer_coordinate == vimarsha->source_ref &&
                source_.m2_pratibimba,
            "force source must retain exact M2-1 Vimarsha writer face");
    constexpr std::uint64_t max_generation = 9007199254740991ULL;
    require(source_.m1_revision <= max_generation &&
                source_.m2_generation <= max_generation &&
                source_.m3_generation <= max_generation && matches(body),
            "force source and physical preparation are disconnected");
    Fingerprint basis;
    basis.text(physical_force_routes_contract);
    for (const auto *ref :
         {&source_.event_ref, &source_.subject_ref, &source_.registry_revision,
          &source_.source_revision, &source_.definition_ref,
          &source_.source_instance_ref, &source_.determination_ref,
          &source_.m2_writer_coordinate, &eigenbasis_identity_,
          &preparation_ref_, &state_ref_, &body_source_coordinate_,
          &body_source_revision_})
      basis.text(*ref);
    basis.integer(source_.m2_pratibimba);
    basis.integer(body_pratibimba_);
    basis.integer(source_.m1_revision);
    basis.integer(source_.m2_generation);
    basis.integer(source_.m3_generation);
    basis.integer(body_revision_);
    basis.integer(sample_rate_);
    basis.integer(admitted_cursor_);
    basis.integer(earth_frame_node_id_);
    source_seal_ = basis.value;
    for (std::size_t r = 0; r < routes_.size(); ++r) {
      const auto &route = routes_[r];
      for (const auto *ref :
           {&route.driver_ref, &route.target_ref, &route.program_ref,
            &route.planet_coordinate, &route.chakra_coordinate,
            &route.projection_ref, &route.calibration_ref,
            &route.calibration_revision, &route.calibration_source_ref,
            &route.calibration_standing})
        require(reference(*ref), "bounded force route reference required");
      require(decimal(route.share_numerator, false) &&
                  decimal(route.share_denominator, true),
              "retain exact original source denominator share");
      require(
          route.target_ref == source_.definition_ref + "#driver/" +
                                  std::to_string(route.native_planet_index) &&
              route.projection_ref == route.calibration_ref + "#centre/" +
                                          std::to_string(route.centre_ordinal),
          "force target/projection must retain native producer binding");
      require(route.calibration_standing == "source_authored" ||
                  route.calibration_standing == "ratified" ||
                  route.calibration_standing == "reference" ||
                  route.calibration_standing == "agent_proposed" ||
                  route.calibration_standing == "measured",
              "declared metric force calibration standing required");
      for (std::size_t prior = 0; prior < r; ++prior) {
        require(routes_[prior].driver_ref != route.driver_ref &&
                    routes_[prior].target_ref != route.target_ref &&
                    routes_[prior].program_ref != route.program_ref &&
                    routes_[prior].native_planet_index !=
                        route.native_planet_index,
                "distinct personal driver identities required");
        const auto &previous = routes_[prior];
        if (previous.projection_ref == route.projection_ref)
          require(
              previous.calibration_ref == route.calibration_ref &&
                  previous.calibration_revision == route.calibration_revision &&
                  previous.calibration_source_ref ==
                      route.calibration_source_ref &&
                  previous.calibration_standing == route.calibration_standing &&
                  previous.metric_node_ids == route.metric_node_ids &&
                  previous.projection.axis == route.projection.axis &&
                  previous.projection.node_weights ==
                      route.projection.node_weights,
              "one calibrated map cannot carry conflicting metric projections");
      }
      QL_M2_PlanetChakraRoute native{};
      require(ql_m2_planet_chakra_route(route.native_planet_index, &native) ==
                  QL_M2_OK,
              "personal source has no native receiving route");
      const auto *planet = ql_m_live_resolve(route.planet_coordinate.c_str());
      const auto *chakra = ql_m_live_resolve(route.chakra_coordinate.c_str());
      require(planet && chakra && planet->id == native.planet_id &&
                  chakra->id == native.chakra_id &&
                  route.planet_coordinate == planet->source_ref &&
                  route.chakra_coordinate == chakra->source_ref &&
                  route.planet_node_id == native.planet_id &&
                  route.chakra_node_id == native.chakra_id &&
                  route.centre_ordinal + 1 == native.chakra_index &&
                  route.relation_refs.size() == native.relation_count,
              "wrong native planet/centre route");
      for (std::size_t rel = 0; rel < native.relation_count; ++rel) {
        const auto *actual =
            ql_m2_planet_chakra_relation(route.native_planet_index, rel);
        require(actual && route.relation_refs[rel] == actual->relation_ref,
                "disconnected native receiving relation");
      }
      const auto &nodes = body.input().nodes;
      require(route.metric_node_ids.size() == nodes.size() &&
                  route.projection.node_weights.size() == nodes.size(),
              "metric force projection must use actual body node order");
      double norm = 0, partition = 0;
      for (double x : route.projection.axis) {
        require(std::isfinite(x), "finite metric force axis required");
        norm += x * x;
      }
      require(std::abs(norm - 1) <= 1e-10,
              "normalized metric force axis required");
      for (std::size_t n = 0; n < nodes.size(); ++n) {
        const double weight = route.projection.node_weights[n];
        require(route.metric_node_ids[n] == nodes[n].identity &&
                    std::isfinite(weight) && weight >= 0 && weight <= 1,
                "detached or invalid metric force projection");
        partition += weight;
      }
      require(std::abs(partition - 1) <= 1e-10 &&
                  std::isfinite(route.peak_force_newtons) &&
                  route.peak_force_newtons >= 0 &&
                  route.peak_force_newtons <= body.input().max_force_newtons,
              "bounded Newton route and partition required");
      bool coupled = false;
      for (std::size_t mode = 0; mode < modes_; ++mode) {
        double coefficient = 0;
        const auto &shape = body.mode_shape(mode);
        for (std::size_t node = 0; node < nodes.size(); ++node)
          for (unsigned axis = 0; axis < 3; ++axis)
            coefficient += shape[node][axis] * route.projection.axis[axis] *
                           route.projection.node_weights[node];
        require(std::isfinite(coefficient), "finite modal route required");
        coefficients_[r][mode] = coefficient;
        coupled = coupled || coefficient != 0;
      }
      require(coupled, "force projection is disconnected from every free mode");
      Fingerprint seal = basis;
      seal.integer(r);
      for (const auto *ref :
           {&route.driver_ref, &route.target_ref, &route.program_ref,
            &route.planet_coordinate, &route.chakra_coordinate,
            &route.share_numerator, &route.share_denominator,
            &route.projection_ref, &route.calibration_ref,
            &route.calibration_revision, &route.calibration_source_ref,
            &route.calibration_standing})
        seal.text(*ref);
      seal.integer(route.native_planet_index);
      seal.integer(route.centre_ordinal);
      seal.integer(route.planet_node_id);
      seal.integer(route.chakra_node_id);
      seal.integer(route.relation_refs.size());
      for (const auto &ref : route.relation_refs)
        seal.text(ref);
      for (std::size_t node = 0; node < nodes.size(); ++node) {
        seal.integer(route.metric_node_ids[node]);
        seal.real(route.projection.node_weights[node]);
      }
      for (double axis : route.projection.axis)
        seal.real(axis);
      seal.real(route.peak_force_newtons);
      for (std::size_t mode = 0; mode < modes_; ++mode)
        seal.real(coefficients_[r][mode]);
      seals_[r] = seal.value;
    }
  }
  const PhysicalForceSourceBasis &source_basis() const noexcept {
    return source_;
  }
  const std::string &eigenbasis_identity() const noexcept {
    return eigenbasis_identity_;
  }
  const std::string &preparation_ref() const noexcept {
    return preparation_ref_;
  }
  const std::string &state_ref() const noexcept { return state_ref_; }
  std::uint64_t source_basis_seal() const noexcept { return source_seal_; }
  // Observer/grounding frame, not a fabricated tenth personal oscillator.
  QL_M_NodeId earth_frame_node_id() const noexcept {
    return earth_frame_node_id_;
  }
  std::uint64_t body_revision() const noexcept { return body_revision_; }
  unsigned sample_rate() const noexcept { return sample_rate_; }
  std::uint64_t admitted_cursor() const noexcept { return admitted_cursor_; }
  std::size_t route_count() const noexcept { return routes_.size(); }
  const PhysicalForceRouteInput &route_input(std::size_t index) const {
    return routes_.at(index);
  }
  std::uint64_t route_seal(std::size_t index) const {
    (void)routes_.at(index);
    return seals_[index];
  }
  // Evidence compatibility only: the owner admits this current source basis.
  // No mutable body or source-registry read in this control-thread preflight.
  bool preflight(const PreparedPhysicalBody &preparation,
                 const PhysicalForceSourceBasis &current_source,
                 std::uint64_t admitted_cursor) const noexcept {
    return current_source == source_ && admitted_cursor >= admitted_cursor_ &&
           matches(preparation);
  }
  // Sole callback/stopped body custody. Every source occupies exactly one
  // block, including explicitly disabled sources. Refusal is fully atomic.
  bool advance_force_block(
      PhysicalBody &body, const PhysicalScalarForceBlock &m1,
      const PhysicalRouteForceBlock *routes, std::size_t route_count,
      float *pickup_linear, std::size_t frames,
      std::uint64_t expected_body_revision, std::uint64_t start_sample,
      PhysicalForceRouteReceipt *receipt = nullptr) const noexcept {
    body.advance_refusal_ = nullptr;
    if (!pickup_linear || frames == 0 || frames > physical_max_frames ||
        route_count != routes_.size() || (route_count && !routes) ||
        expected_body_revision != body_revision_ ||
        start_sample != body.samples_elapsed() ||
        start_sample < admitted_cursor_ ||
        start_sample > std::numeric_limits<std::uint64_t>::max() - frames ||
        !matches(body.preparation()) || !std::isfinite(m1.gain) ||
        std::abs(m1.gain) > 1 || (m1.enabled && !m1.force_newtons)) {
      body.advance_refusal_ =
          "native physical route buffer/body/source/cursor admission differs";
      return false;
    }
    std::array<const PhysicalRouteForceBlock *,
               physical_max_personal_force_routes>
        ordered{};
    PhysicalForceRouteReceipt result;
    result.source_basis_seal = source_seal_;
    result.body_revision = body_revision_;
    result.earth_frame_node_id = earth_frame_node_id_;
    result.start_sample = start_sample;
    result.end_sample = start_sample + frames;
    result.route_count = route_count;
    result.scalar_m1_enabled = m1.enabled;
    result.scalar_m1_gain = m1.gain;
    for (std::size_t b = 0; b < route_count; ++b) {
      const auto &block = routes[b];
      if (block.route_index >= route_count || ordered[block.route_index] ||
          block.preparation_seal != seals_[block.route_index] ||
          !std::isfinite(block.gain) || std::abs(block.gain) > 1 ||
          (block.enabled && !block.force_newtons)) {
        body.advance_refusal_ = "native physical route block/seal/gain differs";
        return false;
      }
      ordered[block.route_index] = &block;
      const auto &input = routes_[block.route_index];
      result.routes[block.route_index] = {block.route_index,
                                          block.preparation_seal,
                                          input.planet_node_id,
                                          input.chakra_node_id,
                                          input.native_planet_index,
                                          input.centre_ordinal,
                                          block.enabled,
                                          block.gain,
                                          0};
    }
    const double max_force = body.preparation().input().max_force_newtons;
    for (std::size_t sample = 0; sample < frames; ++sample) {
      double absolute_sum = 0;
      if (m1.enabled) {
        const double force = m1.force_newtons[sample];
        if (!std::isfinite(force) || std::abs(force) > max_force) {
          body.advance_refusal_ =
              "native physical routed scalar exceeds finite Newton bound";
          return false;
        }
        const double applied = std::abs(force * m1.gain);
        result.scalar_peak_applied_force_newtons =
            std::max(result.scalar_peak_applied_force_newtons, applied);
        absolute_sum += applied;
      }
      for (std::size_t r = 0; r < route_count; ++r) {
        const auto &block = *ordered[r];
        if (!block.enabled)
          continue;
        const double force = block.force_newtons[sample];
        if (!std::isfinite(force) ||
            std::abs(force) > routes_[r].peak_force_newtons) {
          body.advance_refusal_ =
              "native physical independent route exceeds finite Newton bound";
          return false;
        }
        const double applied = std::abs(force * block.gain);
        result.routes[r].peak_applied_force_newtons =
            std::max(result.routes[r].peak_applied_force_newtons, applied);
        absolute_sum += applied;
      }
      if (!std::isfinite(absolute_sum) || absolute_sum > max_force) {
        body.advance_refusal_ =
            "native physical summed routes exceed finite Newton bound";
        return false;
      }
      result.peak_total_absolute_force_newtons =
          std::max(result.peak_total_absolute_force_newtons, absolute_sum);
    }
    const bool advanced = body.advance_projected_force_block(
        [&](std::size_t mode, std::size_t sample, double excitation) noexcept {
          double force =
              m1.enabled ? m1.force_newtons[sample] * m1.gain * excitation : 0;
          for (std::size_t r = 0; r < route_count; ++r) {
            const auto &block = *ordered[r];
            if (block.enabled)
              force += block.force_newtons[sample] * block.gain *
                       coefficients_[r][mode];
          }
          return force;
        },
        pickup_linear, frames);
    if (advanced && receipt)
      *receipt = result;
    return advanced;
  }
};
} // namespace ql
#endif
