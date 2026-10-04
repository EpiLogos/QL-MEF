#ifndef QL_PHYSICAL_FORCE_ROUTES_WIRE_HPP
#define QL_PHYSICAL_FORCE_ROUTES_WIRE_HPP
// Control-thread join of the actual N producer into the retained A/P owner.
// The independently replayed original ReceivingDefinition/current operation
// and native context custody belong to N/A. JSON equality grants no consent.
#include <ql/physical_checkpoint_wire.hpp>
#include <ql/physical_force_routes.hpp>
namespace ql::physical_wire {
struct PhysicalNativeForceProgram {
  double source_hertz = 0, original_denominator_share = 0;
  double peak_force_newtons = 0;
};
struct PreparedNativePhysicalForceRoutes {
  PreparedPhysicalForceRoutes routes;
  std::array<PhysicalNativeForceProgram, physical_max_personal_force_routes> programs{};
  std::uint64_t m3_input_generation = 0;
  std::string native_basis_sha256, m3_state_sha256;
};
inline PreparedNativePhysicalForceRoutes read_prepared_physical_force_routes(
    J *candidate_operation, J *current_native_operation,
    const PhysicalForceSourceBasis &admitted_source,
    const PreparedPhysicalBody &immutable_body,
    const std::vector<std::string> &native_program_refs,
    std::uint64_t admitted_cursor) {
  keys(candidate_operation,
       {"schema", "definition_digest", "context", "native_sample", "event",
        "native_generations", "source_policy", "m2_generation", "m3_generation",
        "source_instance", "body_revision", "preparation", "state", "calibration",
        "sources", "projections"});
  require(current_native_operation &&
              json_object_equal(candidate_operation, current_native_operation),
          "force operation differs from independently replayed native N producer");
  same_text(field(candidate_operation, "schema"), "ql.nara-performance-receiving/v1");
  require(text(field(candidate_operation, "definition_digest")) == admitted_source.definition_ref &&
              text(field(candidate_operation, "source_instance")) == admitted_source.source_instance_ref &&
              checkpoint_decimal(field(candidate_operation, "native_sample")) == admitted_cursor &&
              checkpoint_decimal(field(candidate_operation, "m2_generation")) == admitted_source.m2_generation &&
              checkpoint_decimal(field(candidate_operation, "m3_generation")) == admitted_source.m3_generation,
          "force operation source/generation/cursor differs from native admission");
  auto event = field(candidate_operation, "event");
  keys(event, {"event_ref", "subject_ref", "profile_generation", "registry_revision",
               "m1_revision", "m2_source_ref", "m2_contract_ref", "m3_source_ref", "m3_contract_ref"});
  require(text(field(event, "event_ref")) == admitted_source.event_ref &&
              text(field(event, "subject_ref")) == admitted_source.subject_ref &&
              exact(field(event, "profile_generation")) == admitted_source.m2_generation &&
              checkpoint_decimal(field(event, "m1_revision")) == admitted_source.m1_revision &&
              ql_m_live_accepts_base(text(field(event, "registry_revision")).c_str()),
          "force operation detached from admitted native event");
  auto generations = field(candidate_operation, "native_generations");
  keys(generations, {"m1_revision", "m2_generation", "m3_source_generation", "m3_generation",
                     "m3_state_sha256", "basis_sha256"});
  require(checkpoint_decimal(field(generations, "m1_revision")) == admitted_source.m1_revision &&
              exact(field(generations, "m2_generation")) == admitted_source.m2_generation &&
              exact(field(generations, "m3_generation")) == admitted_source.m3_generation,
          "force operation independent native generations disconnected");
  auto sources = field(candidate_operation, "sources");
  auto projections = field(candidate_operation, "projections");
  const auto source_count = count(sources, physical_max_personal_force_routes);
  require(native_program_refs.size() == source_count,
          "each independent source requires its own native program identity");
  // Neutral World still belongs to the actual prepared material/body. Source
  // generations can remain unchanged across a physical material revision.
  require(checkpoint_decimal(field(candidate_operation, "body_revision")) == immutable_body.input().body_revision,
          "receiving body revision stale");
  for (const auto &binding : {std::pair{"preparation", &immutable_body.input().preparation_ref},
                              {"state", &immutable_body.input().state_ref}}) {
    auto reference = field(candidate_operation, binding.first);
    keys(reference, {"reference", "revision"});
    require(text(field(reference, "reference")) == *binding.second &&
                checkpoint_decimal(field(reference, "revision")) == immutable_body.input().body_revision,
            "receiving preparation/state disconnected");
  }
  auto context = field(candidate_operation, "context");
  const auto kind = text(field(context, "kind"));
  if (kind == "world") {
    require(source_count == 0 && count(projections, 7) == 0,
            "neutral World cannot acquire personal force arrays");
    return {PreparedPhysicalForceRoutes(immutable_body, admitted_source, {}, admitted_cursor), {},
            exact(field(generations, "m3_source_generation")),
            text(field(generations, "basis_sha256")), text(field(generations, "m3_state_sha256"))};
  }
  require((kind == "personal" || kind == "shared") && source_count == 9 &&
              count(projections, 7) == 7,
          "personal receiving requires nine original drivers and seven calibrated maps");
  std::string calibration_ref, calibration_revision, calibration_source, calibration_standing;
  auto calibration = field(candidate_operation, "calibration");
  provenance(calibration, calibration_ref, calibration_revision, calibration_source, calibration_standing);
  std::array<J *, 7> by_centre{};
  for (std::size_t i = 0; i < 7; ++i) {
    auto projection = at(projections, i);
    keys(projection, {"projection_ref", "centre_ordinal", "native_node_ids", "weights",
                      "metric_axis", "source_force_newtons", "calibration"});
    const auto ordinal = exact(field(projection, "centre_ordinal"));
    require(ordinal < 7 && !by_centre[ordinal] &&
                text(field(projection, "projection_ref")) == calibration_ref + "#centre/" + std::to_string(ordinal) &&
                json_object_equal(field(projection, "calibration"), calibration),
            "duplicate/detached calibrated receiving map");
    by_centre[ordinal] = projection;
  }
  std::vector<PhysicalForceRouteInput> inputs;
  inputs.reserve(source_count);
  std::array<PhysicalNativeForceProgram, physical_max_personal_force_routes> programs{};
  for (std::size_t i = 0; i < source_count; ++i) {
    auto source = at(sources, i);
    keys(source, {"driver_ref", "native_planet_id", "centre_ordinal", "hertz", "source_ref", "target_ref",
                  "share_numerator", "share_denominator", "projection_ref", "calibration_ref",
                  "original_denominator_share", "peak_force_newtons", "relation_refs"});
    const auto planet_index = exact(field(source, "native_planet_id"));
    const auto ordinal = exact(field(source, "centre_ordinal"));
    require(planet_index < 10 && ordinal < 7, "invalid native source/receiving identity");
    QL_M2_PlanetChakraRoute native{};
    require(ql_m2_planet_chakra_route(unsigned(planet_index), &native) == QL_M2_OK &&
                native.chakra_index == ordinal + 1,
            "source receiving route differs from actual native M2 operation");
    auto projection = by_centre[ordinal];
    PhysicalForceRouteInput route;
    route.driver_ref = text(field(source, "driver_ref"));
    route.target_ref = text(field(source, "target_ref"));
    route.program_ref = native_program_refs[i];
    route.planet_coordinate = text(field(source, "source_ref"));
    const auto *chakra = ql_m_live_node_by_id(native.chakra_id);
    require(chakra, "native receiving chakra unavailable");
    route.chakra_coordinate = chakra->source_ref;
    route.native_planet_index = unsigned(planet_index);
    route.centre_ordinal = unsigned(ordinal);
    route.planet_node_id = native.planet_id;
    route.chakra_node_id = native.chakra_id;
    route.share_numerator = text(field(source, "share_numerator"));
    route.share_denominator = text(field(source, "share_denominator"));
    const auto numerator = checkpoint_decimal(field(source, "share_numerator"));
    const auto denominator = checkpoint_decimal(field(source, "share_denominator"));
    require(denominator > 0 && numerator <= denominator,
            "invalid exact original denominator share");
    const double share = number(field(source, "original_denominator_share"));
    require(share >= 0 && share <= 1 &&
                std::abs(share - double(numerator) / double(denominator)) <= 1e-14,
            "source share changed or nine-way renormalized");
    route.projection_ref = text(field(source, "projection_ref"));
    route.calibration_ref = text(field(source, "calibration_ref"));
    route.calibration_revision = calibration_revision;
    route.calibration_source_ref = calibration_source;
    route.calibration_standing = calibration_standing;
    require(route.calibration_ref == calibration_ref &&
                route.projection_ref == text(field(projection, "projection_ref")),
            "source detached from its independent calibrated map");
    auto nodes = field(projection, "native_node_ids");
    auto weights = field(projection, "weights");
    const auto n = immutable_body.input().nodes.size();
    require(count(nodes, physical_max_nodes) == n && count(weights, physical_max_nodes) == n,
            "receiving metric projection body basis differs");
    route.projection.axis = vec(field(projection, "metric_axis"));
    for (std::size_t node = 0; node < n; ++node) {
      route.metric_node_ids.push_back(checkpoint_decimal(at(nodes, node)));
      route.projection.node_weights.push_back(number(at(weights, node)));
    }
    const double calibration_force = number(field(projection, "source_force_newtons"));
    route.peak_force_newtons = number(field(source, "peak_force_newtons"));
    require(calibration_force >= 0 && calibration_force <= immutable_body.input().max_force_newtons &&
                route.peak_force_newtons == calibration_force * share,
            "native calibrated Newton force differs from original source share");
    auto relations = field(source, "relation_refs");
    require(count(relations, 1024) == native.relation_count, "native receiving relation count differs");
    for (std::size_t relation = 0; relation < native.relation_count; ++relation)
      route.relation_refs.push_back(text(at(relations, relation)));
    const double hertz = number(field(source, "hertz"));
    require(hertz >= 1 && hertz <= 20000 && hertz < .45 * immutable_body.input().sample_rate,
            "source frequency exceeds admitted forcing band");
    programs[i] = {hertz, share, route.peak_force_newtons};
    inputs.push_back(std::move(route));
  }
  return {PreparedPhysicalForceRoutes(immutable_body, admitted_source, std::move(inputs), admitted_cursor),
          programs, exact(field(generations, "m3_source_generation")),
          text(field(generations, "basis_sha256")), text(field(generations, "m3_state_sha256"))};
}
// This is a compatibility witness accompanying C's retained original native
// operation/basis/program assets and the existing P q/v checkpoint. It creates
// neither a store nor a clock and cannot independently re-admit a personal act.
inline Json force_routes_preparation_witness(const PreparedPhysicalForceRoutes &routes) {
  auto out = own(json_object_new_object());
  require(bool(out), "force preparation witness allocation failed");
  checkpoint_string(out.get(), "schema", "ql.physical-force-routes-preparation-witness/v1");
  checkpoint_integer(out.get(), "source_basis_seal", routes.source_basis_seal());
  checkpoint_integer(out.get(), "body_revision", routes.body_revision());
  checkpoint_integer(out.get(), "admitted_cursor", routes.admitted_cursor());
  checkpoint_string(out.get(), "preparation_ref", routes.preparation_ref());
  checkpoint_string(out.get(), "state_ref", routes.state_ref());
  checkpoint_string(out.get(), "eigenbasis_identity", routes.eigenbasis_identity());
  auto seals = own(json_object_new_array());
  require(bool(seals), "force route witness allocation failed");
  for (std::size_t i = 0; i < routes.route_count(); ++i)
    require(json_object_array_add(seals.get(), json_object_new_string(std::to_string(routes.route_seal(i)).c_str())) == 0,
            "force route seal allocation failed");
  checkpoint_put(out.get(), "route_seals", seals.release());
  return out;
}
inline void qualify_force_routes_preparation_witness(J *saved,
                                                     const PreparedPhysicalForceRoutes &replayed) {
  keys(saved, {"schema", "source_basis_seal", "body_revision", "admitted_cursor",
               "preparation_ref", "state_ref", "eigenbasis_identity", "route_seals"});
  same_text(field(saved, "schema"), "ql.physical-force-routes-preparation-witness/v1");
  require(checkpoint_decimal(field(saved, "source_basis_seal")) == replayed.source_basis_seal() &&
              checkpoint_decimal(field(saved, "body_revision")) == replayed.body_revision() &&
              checkpoint_decimal(field(saved, "admitted_cursor")) == replayed.admitted_cursor() &&
              text(field(saved, "preparation_ref")) == replayed.preparation_ref() &&
              text(field(saved, "state_ref")) == replayed.state_ref() &&
              text(field(saved, "eigenbasis_identity")) == replayed.eigenbasis_identity(),
          "saved force preparation is stale or disconnected");
  auto seals = field(saved, "route_seals");
  require(count(seals, physical_max_personal_force_routes) == replayed.route_count(),
          "saved independent force routes lost");
  for (std::size_t i = 0; i < replayed.route_count(); ++i)
    require(checkpoint_decimal(at(seals, i)) == replayed.route_seal(i),
            "saved source/projection/program seal changed");
}
} // namespace ql::physical_wire
#endif
